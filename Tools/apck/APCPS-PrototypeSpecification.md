# Ann's PHI Clipbuddy — Prototype Specification

## Scope

This document specifies the engineering pipeline for the proof-of-life prototype. It covers detection algorithms, tech stack, data sources, wire-format JSON schemas, the container image, project structure, and scope boundaries. It does NOT cover the user experience — see [APCAS-Specification.md](APCAS-Specification.md) for the UX plan — nor the detection pipeline's conceptual vocabulary, which lives in [APCS0-SpecTop.adoc](APCS0-SpecTop.adoc).

This is a living document. The pipeline will evolve rapidly during prototype development. Changes here do not require parallel updates to the product spec.

## Scope Boundaries

The prototype deliberately excludes:

- **No JavaScript** — All rendering logic is Rust. The Tauri webview is a passive display surface receiving complete HTML from the Rust backend. Inline `onclick` attributes use the Tauri bridge primitive (`window.__TAURI__.core.invoke`) for command dispatch only — no `.js` files, no JS application logic, no JS state management. This eliminates the language-boundary bug class (e.g., byte-offset vs char-offset mismatch at the Rust-JS serialization boundary).
- **No persistence** — no audit trail, no history of anonymized notes beyond the journal directory artifacts
- **One patient at a time** — no batch processing in the interactive path; batch assay (`apcab`) is dev-only
- **No configuration** — no custom blacklists, no user preferences, no settings UI
- **No click-to-highlight** — clicking a finding does not scroll/highlight in the document preview (future feature)
- **No multi-word merge in Rust-side scans** — adjacent name hits from the dictionary scans are individually flagged, not merged; container-side NER discerners (Stanford, Stanza, spaCy) may produce multi-token spans natively

## Tech Stack

### Rust Application

| Layer | Crate | Version | License | Rationale |
|-------|-------|---------|---------|-----------|
| Clipboard | `arboard` | 3.x | MIT/Apache | Only crate with cross-platform HTML clipboard read (`get_html()`) |
| HTML parsing | `scraper` | latest | MIT | DOM + CSS selectors, built on Mozilla's html5ever |
| Dictionary matching | `aho-corasick` | latest | MIT/Unlicense | Single-pass multi-pattern O(n) regardless of dictionary size |
| Pattern matching | `regex` | latest | MIT/Apache | Structural PHI patterns |
| Word boundaries | `unicode-segmentation` | latest | MIT/Apache | UAX#29 compliant word boundary detection |
| GUI shell | `tauri` | 2.x | MIT | Rust-rendered HTML/CSS frontend, system webview as passive display |
| File watching | `notify` | latest | MIT/Apache | FSEvents on macOS, ReadDirectoryChanges on Windows |
| NSPasteboard FFI (macOS) | `objc2` + `objc2-foundation` + `objc2-app-kit` | 0.6.4 / 0.3.2 / 0.3.2 | MIT | Declared-UTI enumeration for multi-flavor clipboard harvest. Versions pinned to match whatever Tauri already pulls transitively (single objc2 tree resolution). Gated `[target.'cfg(target_os = "macos")'.dependencies]`. |
| JSON deserialization | `serde` + `serde_json` | 1.x | MIT/Apache | Read container's consolidated findings file |

All Rust crates are pure Rust, fully Cargo-lockable on macOS and Windows. Tauri requires the system webview (WebKit on macOS, WebView2 on Windows — both ship with the OS). The webview receives pre-rendered HTML from Rust on every state change; no JavaScript application code exists in the project.

ONNX Runtime (`ort` crate), previously used by the Stanford spike binary `apcnsa`, is no longer used by the production pipeline — all neural inference now runs inside the container. The `apcnsa` spike binary retains the `ort` dependency for its own offline assay runs; see "Historical spike" note under Container Discerners.

### Container Image

| Layer | Package | Rationale |
|-------|---------|-----------|
| Base image | `python:3.11-slim` (or pinned equivalent) | Minimal Python 3.11 footprint |
| Neural inference | `transformers`, `torch` (CPU-only wheel) | Native PyTorch runtime for `StanfordAIMI/stanford-deidentifier-base` |
| Tokenization | `tokenizers` | HuggingFace fast tokenizer (bundled transitively) |
| Biomedical NLP | `scispacy`, `spacy`, model `en_core_sci_md` | Clinical-domain spaCy pipeline — POS, dependency parse, biomedical NER |
| General NLP | `stanza` (English UD package) | POS, morphological features, dependency parse, OntoNotes-style NER as a second independent NER signal |

`scispacy` is layered on `spacy` and tracks upstream spaCy releases with occasional lag; the Dockerfile pins both to a compatible pair. Torch is installed as the CPU-only wheel explicitly — GPU wheels are unusable inside a Linux container on macOS (Metal/MPS does not pass through) and would only bloat the image.

Expected image size: ~1.5–2 GB. One-time pull per deploy target.

### Tauri CLI cwd Constraint

The `cargo tauri build` command discovers its project by walking up from the current working directory looking for `Cargo.toml` and `tauri.conf.json`. It has no `--manifest-path` equivalent. Additionally, `tauri.conf.json` uses relative paths (e.g., `"frontendDist": "./ui"`) that resolve from the Tauri project directory.

This means `cargo tauri build` must run with cwd set to `Tools/apck/apcd/`. This is an external tool constraint, not a design choice. All other cargo commands (`cargo run`, `cargo test`) use `--manifest-path` and run from the project root per BCG convention. The two `cargo tauri` call sites (`apcc_build`, `apcc_deploy`) use BCG isolation subshells to contain the `cd`.

## Input: Epic "Copy All" HTML

Epic's "Copy All" places HTML on the clipboard with structural formatting:

- `<b>Label:</b> value` patterns for demographics and metadata
- Section headers for clinical categories (Chief Complaint, HPI, PMH, Medications, Vitals, Labs, Assessment/Plan)
- Narrative freetext in the History of Present Illness and other sections
- Signature blocks with provider names and timestamps

The HTML structure is semi-structured — not arbitrary prose and not fully machine-readable. This structure is the primary advantage over plain text processing: labels anchor PHI detection with high precision.

## Clipboard Handling

### Clinical Content Heuristic

Before processing, the engine checks whether clipboard content appears to be clinical. The check scans for known Epic/clinical label patterns:

**Positive indicators** (presence of 2+ triggers acceptance):
- `Patient:`, `DOB:`, `MRN:`, `Attending:`, `Facility:`
- `Chief Complaint:`, `History of Present Illness:`, `Assessment/Plan:`
- `Vitals:`, `Labs:`, `Medications:`, `Allergies:`
- HTML `<b>Label:</b>` structure characteristic of Epic formatting

If clinical content is detected, the engine proceeds to the detection pipeline and clears the system clipboard (writes empty string via `arboard::set_text("")`).

If clinical content is NOT detected, the app clears any previous triage state and returns to the initial instruction view. The app displays a brief diagnostic line showing: clipboard content length, content type (HTML/plain text), and the first ~100 characters of content.

This heuristic will be refined when tested against real Epic output. The initial label list is calibrated to pass the synthetic test fixture.

### Clipboard Change Detection

On window focus, the engine compares the current clipboard content (byte-for-byte string comparison) against the last successfully consumed content. If identical, the existing triage state is preserved — no reprocessing. If different, the clinical content heuristic runs on the new content. Clinical notes are a few KB; storing the last consumed string for comparison has negligible memory cost at prototype scale.

## Journal Directory

**Purpose.** One place on disk that accumulates runtime artifacts produced by the app AND the container: verbatim clipboard harvests from every focus with changed content, the normalized-text input that the container consumes, the container's consolidated findings output, the anonymized output, the container's own log, and a running log of every `apcrl_*` emission teed from stdout. A single flat location keeps the mental model honest — one `ls` answers everything.

**Path.** `$HOME/apcjd/` — outside any repo tree. The directory is created lazily at application startup (for the log tee) or on first capture (for harvests), whichever comes first. It is simultaneously the bind-mount target for the container. A `.gitignore` entry for `apcjd/` is present as belt-and-suspenders against a misdirected `$HOME` resolution that lands in-tree.

**Contents.** Five independent producers share this flat directory:

| Artifact | Producer | Shape |
|----------|----------|-------|
| Raw harvest captures | `apcrh_harvest` on every focus with changed content | `{N}-in.{tag}.{ext}` where `{tag}` is `clinical` or `nonclinical` |
| Normalized text | `apcrh_harvest` (Clinical branch only) | `{N}-in.txt` — plain UTF-8 after HTML strip / whitespace collapse; the container's input |
| Container findings | `apcscc_container` on completion | `{N}.json` — consolidated output of all three container discerners (published atomically via `{N}.json.tmp` → rename) |
| Anonymized outputs | `apcap_main` focus handler on Clinical branch | `{N}-out.txt` |
| Observability log (app) | `apcrl_log` file-tee sink | `apcap.log` |
| Observability log (container) | container runtime | `container-log.txt` (truncated on each container restart by the start-container tabtarget) |

All files sharing the same leading digit run `{N}` belong to the same capture.

## Clipboard Harvest

**Purpose.** Preserve verbatim real-world clipboard contents from every focus with changed content so the fixture library can grow from real traffic, so Epic's actual flavor structure can be studied, and so a classifier misfire still leaves an artifact on disk to post-mortem.

**Trigger.** Every focus with changed clipboard content — executed after the classifier runs so the verdict can be tagged into the filename, but before any Clinical-branch clipboard zero-out. The same-as-last short-circuit at the top of the focus handler still applies: re-focusing with unchanged content does not re-harvest.

**Storage.** The journal directory (`$HOME/apcjd/`).

**Naming — raw harvest.** Raw captures are written as `{N}-in.{tag}.{ext}` where `{tag}` is `clinical` or `nonclinical` — the classifier's verdict at the moment of capture. `N` seeds at 10000 for an empty directory, otherwise `max_leading_digit_run + 1` across every filename whose name begins with digits. Files sharing the same `N` group all artifacts of one capture — e.g., `10000-in.clinical.rtf`, `10000-in.clinical.utf8.txt`, `10000-in.clinical.utf16.txt`, `10000-in.txt`, `10000-out.txt`, and `10000.json` are six artifacts of clinical capture 10000. Gaps in the numeric sequence are not filled — the scan advances past the current maximum. Filenames that do not begin with a digit are ignored in the max calculation, so `apcap.log`, `container-log.txt`, and any user-placed `README` or `notes` co-exist without perturbing indexing.

**Naming — normalized text.** On every Clinical-branch focus, after the raw captures are written, `apcrh_harvest` also writes `{N}-in.txt`: the `apcsgt_normalized_text` (plain UTF-8 after HTML strip, whitespace collapse, encoding normalization as described in APCS0). This file is the container's input — the container reads the highest-indexed `{N}-in.txt` it finds in the bind-mounted directory, runs its three discerners, and writes `{N}.json`. The bare `{N}-in.txt` name (no `{tag}` infix, no UTI-derived suffix) distinguishes the pipeline-input file from the raw-flavor captures. `{N}-in.txt` is written only for clinical captures — non-clinical captures produce no normalized text because no detection pipeline runs.

**Naming — UTI-derived extensions.** The `{ext}` portion of raw captures is derived from the declared UTI via a data-driven table in `apcrb_pasteboard::apcrb_extension_for_uti`. Known public UTIs receive human-readable extensions (see Flavors below); any UTI not in the table — including legacy NSPasteboard names like `Unicode text` or a third-party `com.example.thing` — falls through `zapcrb_sanitize_uti` to `{sanitized}.bin`.

**Flavors.** Every flavor the producer **declared** on the pasteboard — enumerated via `NSPasteboardItem::types` on the general pasteboard's first item and fetched via `dataForType:`. Declared flavors are the set the producer actually wrote; macOS may synthesize additional derived types on read. Synthesized types are deliberately skipped — the parity target is `osascript -e 'clipboard info'`, which reports declared types only.

Known UTI → extension mappings:

| UTI | Filename | Typical producer |
|-----|----------|------------------|
| `public.rtf` | `{N}-in.{tag}.rtf` | Epic Copy All, Word |
| `public.utf8-plain-text` | `{N}-in.{tag}.utf8.txt` | Almost everything |
| `public.utf16-plain-text` | `{N}-in.{tag}.utf16.txt` | Epic Copy All, many Cocoa apps |
| `public.plain-text` | `{N}-in.{tag}.txt` | Legacy / ambiguous encoding |
| `public.html` | `{N}-in.{tag}.html` | Browsers; absent from Epic Copy All |
| `public.tiff` / `public.png` / `public.jpeg` | `{N}-in.{tag}.{ext}` | Image copy |
| `public.url` / `public.file-url` | `{N}-in.{tag}.url` / `.fileurl` | URL bar / Finder copy |
| anything else | `{N}-in.{tag}.{sanitized-uti}.bin` | Legacy NSPasteboard names, third-party UTIs |

arboard remains on the classifier's `get_text()` path. The separation is load-bearing: arboard is a simple, stable, cross-platform read of the UTF-8 flavor used to feed the clinical-gate heuristic; `apcrb_pasteboard` is lower-level, Mac-only, riskier FFI code and owns enumeration + multi-flavor capture for disk. A break in `apcrb` (FFI error, empty pasteboard, non-macOS build) surfaces as an absent `{N}-in.{tag}.*` artifact and an `apcrl_error_now!` line in the log — it does not cascade into the classifier.

**Anonymized output.** On every Clinical-branch focus, after the input harvest succeeds, the focus handler also writes `{N}-out.txt`: the anonymizer's output with default-elide toggle states (every finding elided). Because combining is currently deferred, the prototype's anonymized output is derived from Rust-only findings — the combining-aware version will land when combining is spec'd and implemented. Write failure is non-fatal.

**Failure mode.** A capture failure logs via `apcrl_error_now!` (stdout + tee) and does not abort triage. User-visible behavior is unchanged whether harvest succeeds or fails.

**Privacy posture.** PHI-at-rest stays outside the repo. The journal directory is PHI-permissive by design — every artifact in `$HOME/apcjd/` (raw harvests, normalized inputs, container outputs, anonymized pairings, app log, container log) shares the same on-device-only posture: nothing is auto-committed, auto-uploaded, or auto-anonymized. Review and promotion of any artifact to an out-of-site context is a manual act. The container's zero-network posture strengthens this: inference cannot leak PHI over the network because there is no network.

## Observability Log

**Purpose.** Give the clinician-developer and the engineer a shared, persistent view of what the app did across runs. The `.app` bundle on macOS captures stdout where a non-technical user won't look for it; a file in the journal directory is where both humans already know to peek.

**Mechanism.** `apcrl_log` exposes an optional file-tee sink: `apcrl_tee_init(path)` installs a once-only append-open handle. Every subsequent `apcrl_*_now!`, `_if!`, and comparison-variant emission is written twice — once to stdout (verbatim) and once to the tee file (same format, same line).

**Format.** Exactly what stdout receives: `[LEVEL] [YYYY-MM-DD HH:MM:SS.mmm] [file:line] message`. The timestamp is local wall-clock with millisecond precision, captured at emit time via `chrono::Local::now()`.

**Location.** `$HOME/apcjd/apcap.log`. Filename matches the binary name (`apcap`) as an engineer mnemonic. Append-only — no rotation at prototype scale.

**Container log.** Separate from `apcap.log`. The container writes all of its own diagnostic output to `$HOME/apcjd/container-log.txt`. This log is cleared (truncated to zero) by the start-container tabtarget on each container start — so a log inspection after a session reflects only that session's container activity. The app never reads or writes this file.

**Init.** At `main()` startup, before any other log emission. Failure to install the tee (missing HOME, disk full, permissions) logs to stdout and proceeds — the app starts and stdout remains authoritative.

**Failure mode.** Tee write failures are silently swallowed inside the emitter to avoid recursive-logging hazards.

**Privacy posture.** All logs in the journal directory inherit the PHI-permissive on-device posture.

## Detection Pipeline

The conceptual vocabulary lives in [APCS0-SpecTop.adoc](APCS0-SpecTop.adoc). This section documents each discerner's concrete parameterization — regex patterns, label registry, dictionary sources, container model pinning.

### Overview

Ten discerners, seven running inside the Rust application and three inside the container, each a pure function of `apcsgt_normalized_text`. Each emits an intimately-typed finding into the evidence pool. Combining rules are deferred; the prototype currently uses a stand-in that treats Rust-side findings only, and the anonymized output `{N}-out.txt` reflects that stand-in. Full combining is a later pace once empirical evidence from real clinical text has flowed through all ten discerners.

```
                                 apcsuc_clipboard_content
                                          |
                                      Clinical gate
                                          |
                                   apcsgn_normalization  (Rust)
                                          |
                                 apcsgt_normalized_text  (= {N}-in.txt)
                                          |
            +-----------------------------+-----------------------------+
            |                             |                             |
    Rust-application discerners      container discerners         (container reads {N}-in.txt,
    (7; in-process emits)            (3; via {N}.json)             writes {N}.json atomically)
            |                             |
            +--------------+--------------+
                           |
                     apcsde_evidence
                           |
                     apcsnc_combining  (DEFERRED)
                           |
                 apcsnu_unified_finding_s
                           |
                    apcsna_anonymization  (Rust)
                           |
                    apcsnt_anonymized_text  (= {N}-out.txt)
```

### Regex Scan — parameterization

Structural patterns with low false-positive rate. No model dependency. Runs in the Rust application.

**Known ambiguities.** The zip code regex (`\d{5}`) can match 5-digit lab values or dosage amounts. The date regex can match version numbers. The street address regex can match non-address text. These are accepted risks for the prototype — the conservative bias (false positive > false negative) means these appear as findings the clinician can toggle. Context-aware refinement is a combining concern.

| Pattern | Category | Regex (representative) | Notes |
|---------|----------|----------------------|-------|
| SSN | `[SSN]` | `\d{3}-\d{2}-\d{4}` | |
| Phone | `[PHONE]` | `\d{3}[-.)]\s?\d{3}[-.)]\s?\d{4}`, `\(\d{3}\)\s?\d{3}-\d{4}` | Multiple formats |
| Email | `[EMAIL]` | `[\w.+-]+@[\w-]+\.[\w.-]+` | |
| Dates | `[DATE]` | `\d{1,2}/\d{1,2}/\d{2,4}`, `\d{1,2}-\d{1,2}-\d{2,4}`, month name patterns | Multiple formats |
| Street address | `[ADDRESS]` | `\d+\s+[A-Z]\w+\s+(St\|Dr\|Ave\|Blvd\|Rd\|Ln\|Way\|Ct\|Pl\|Cir\|Drive\|Street\|Avenue\|Boulevard\|Road\|Lane\|Court\|Place\|Circle)` | Followed by optional city/state/zip |
| Zip code | `[ADDRESS]` | `\b\d{5}(-\d{4})?\b` | Within address context |
| MRN (labeled) | `[MRN]` | Value following `MRN:` label | |
| Account (labeled) | `[ACCOUNT]` | Value following `Account` or `Acct` label | |
| Encounter ID (labeled) | `[ENCOUNTER_ID]` | Value following `Encounter ID:` label | |

Dates receive special handling: when a date follows a `DOB:` label, the engine computes age from the current date and emits `Age: NN` instead of `[DATE]` in the anonymized output. Per HIPAA Safe Harbor (45 CFR 164.514(b)(2)(i)(C)), ages over 89 are aggregated: computed age >89 emits `Age: 90+`.

### Label Scan — parameterization

Words immediately following known Epic labels. Label matching is case-insensitive. Value extraction extends to the next structural boundary. Per-word anchoring — each word is individually flagged. Runs in the Rust application.

| Label Pattern | Category | Extraction Rule |
|--------------|----------|-----------------|
| `Patient:` | `[NAME]` | All text until next label or line break |
| `Attending:` | `[PROVIDER]` | All text until next label or line break |
| `Provider:` | `[PROVIDER]` | All text until next label or line break |
| `Referring:` | `[PROVIDER]` | All text until next label or line break |
| `Facility:` | `[FACILITY]` | All text until next label or line break |
| `Electronically signed by:` | `[PROVIDER]` | All text until next label or line break |
| `Emergency contact` | `[NAME]` | Name portion, phone extracted by regex scan |
| `primary care physician` | `[PROVIDER]` | Following name (e.g., "Dr. Susan Chen") |

### Dictionary Scans — parameterization

Five separate scans, each parameterized by one dictionary, each emitting its own intimate finding type. All case-insensitive; all use aho-corasick automata built from lowercase entries; all scan the lowercased `apcsgt_normalized_text` while preserving original case for display. All unconditional — every match emits a finding regardless of whitelist membership, and combining is responsible for interpreting overlaps. All run in the Rust application.

| Scan | Dictionary | Implied category | Role |
|------|-----------|------------------|------|
| `apcsds_surname` | surnames.txt | NAME | PHI candidate — emit on match |
| `apcsds_firstname` | firstnames.txt | NAME | PHI candidate — emit on match |
| `apcsds_city` | cities.txt | ADDRESS | PHI candidate (may remap to FACILITY under combining) — emit on match |
| `apcsds_english` | english_whitelist.txt | (none) | Suppression evidence — emit on match |
| `apcsds_medical` | medical_whitelist.txt | (none) | Suppression evidence — emit on match |

The five scans, together, replace the single Tier-3 "dictionary scan" of earlier drafts. The whitelist scans (`english`, `medical`) previously existed only as flags on dictionary findings; promoting them to first-class discerners makes their evidence addressable by combining rules on equal footing with the name/location scans.

### Container Discerners — parameterization

All three run inside `apcscc_container` (see Container Architecture below). Each consumes `{N}-in.txt` (the normalized text) and contributes a key to the consolidated `{N}.json`.

| Discerner | Model | Placement | Primary output signal |
|-----------|-------|-----------|----------------------|
| `apcscs_stanford` | `StanfordAIMI/stanford-deidentifier-base` (HuggingFace transformers, PyTorch CPU) | container | PHI named entities with native 8-label taxonomy |
| `apcscs_spacy` | scispaCy `en_core_sci_md` | container | POS tags, morphological features, dependency parse, biomedical NER |
| `apcscs_stanza` | Stanza English UD pipeline (default English package) | container | POS (UD + language-specific), morphological features, lemma, dependency parse, OntoNotes-style NER |

**Stanford taxonomy.** The `StanfordAIMI/stanford-deidentifier-base` model uses a **flat 8-label** taxonomy (no BIO prefix):

```
O, VENDOR, DATE, HCW, HOSPITAL, ID, PATIENT, PHONE
```

Runs of identical labels form a single entity. This taxonomy is read from `config.json:id2label` at container image build time — no hardcoded mapping on the Rust side. Mapping these native labels to `apcsgc_phi_category` is a combining concern (deferred).

**Observed Stanford strengths** (recorded from spike evaluation on APCK synthetic fixtures):
- Multi-token person names merged as single spans (`Margaret J. Thornton` → one `PATIENT`)
- Facility names merged (`Maine Medical Center` → one `HOSPITAL`)
- Narrative-scope PHI catches where positional anchors are absent

**Observed Stanford weaknesses:** structured identifiers (SSN, URL, address, device serial) fragment across multiple labels — less reliable than regex for the same categories. The regex scan and Stanford scan are complementary; combining is where the complementarity is cashed in.

**Historical spike.** An earlier spike (`apcnsa_main.rs` + formerly APCNS0-NeuralStanford.md) validated Stanford de-identifier utility by running it via ONNX inside a standalone Rust binary. That binary continues to exist as a reference / offline assay tool; the container-based path supersedes it for the production pipeline. The ONNX export ceremony is no longer required — the container uses HuggingFace transformers + PyTorch natively.

**spaCy choice.** `en_core_sci_md` is the clinical-domain model from scispaCy. Provides POS tagging, dependency parsing, and biomedical NER. Chosen for its syntactic signals (POS + dependency parse), which are what enable disambiguation of homographs like "may" (modal verb, AUX) vs. "May" (proper noun, PROPN). Its NER is biomedical-focused (diseases, chemicals) and provides supplementary signal; PHI-focused NER is the Stanford scan's job.

**Stanza choice.** English Universal Dependencies pipeline. Provides a second, architecturally independent syntactic analyzer plus OntoNotes-style NER (PERSON, ORG, GPE, DATE). The two-parser redundancy is deliberate — when spaCy and Stanza agree that "may" is AUX, that's independent corroboration; when they disagree, combining can weight accordingly. Whether both ship in MVP-to-Ann or whether evaluation narrows to one is a decision for a later pace.

## Container Architecture

### Purpose

Isolate Python-native NLP algorithms from the Rust application process while preserving a strong privacy and security posture. The container's zero-network, capability-dropped, non-root posture means the sophisticated algorithms can be iterated on and scaled up without widening the trust boundary of Clipbuddy itself.

### Lifecycle

Container lifecycle is **managed externally from Clipbuddy** — via tabtargets, not by the Tauri app. Clipbuddy presumes the container is already running; if it isn't, the bind-mount directory simply fails to receive `{N}.json` files and combining runs with an empty container-side evidence contribution (graceful degradation — Rust-side evidence still drives the anonymized output).

Tabtarget set (names in the `apcw-X` family; concrete names pending mint):

| Tabtarget | Purpose |
|-----------|---------|
| Image build | `docker build` the container image from the Dockerfile in the project tree |
| Container start | Truncate `container-log.txt`, `docker run` the image with the bind mount and security flags |
| Container stop | `docker stop` + `docker rm` |
| Container status | Report whether the container is running, which image it's bound to, whether it can see the bind-mount directory |

### Bind Mount

**Host directory.** `$HOME/apcjd/` — the journal directory, bind-mounted into the container at a container-local path (e.g., `/work/apcjd/`). Single directory, read+write for both sides.

**Container files it reads:**
- `{N}-in.txt` — normalized text for the input with index `N`

**Container files it writes:**
- `{N}.json.tmp` — transient, during assembly
- `{N}.json` — consolidated findings, published atomically via `rename(2)`
- `container-log.txt` — running diagnostic log

**Indexing discipline.** The container scans `$HOME/apcjd/` for `{N}-in.txt` files. It processes **only the highest-indexed** input and ignores all others. This implements a drop-old backlog policy: if Ann copies three things quickly, the container processes only the newest; older inputs are skipped. Cleanup of stale inputs and outputs is manual.

### Wire Protocol

1. Clipbuddy writes `{N}-in.txt` containing `apcsgt_normalized_text` to `$HOME/apcjd/`.
2. The container (in a polling or inotify-driven loop; implementation detail) observes the new file, confirms `N` is the highest index present, and runs all three discerners on its contents.
3. The container writes `{N}.json.tmp` with the consolidated findings from all three discerners.
4. When all algorithms have completed and the file is closed, the container `rename(2)`s `{N}.json.tmp` to `{N}.json`. POSIX guarantees the rename is atomic within the same filesystem — readers either see the old (absent) state or the final file, never a partially written intermediate.
5. Clipbuddy (watching via the `notify` crate) observes `{N}.json` existing, reads and deserializes it, and folds its findings into the evidence pool alongside the Rust-side findings.

No sockets, no HTTP, no pipes — POSIX file I/O is the entire wire format.

### Container Output JSON Schema

The `{N}.json` file has this top-level shape:

```json
{
  "index": 10000,
  "stanford": {
    "findings": [ /* see Stanford finding schema */ ]
  },
  "spacy": {
    "findings": [ /* see spaCy finding schema */ ]
  },
  "stanza": {
    "findings": [ /* see Stanza finding schema */ ]
  }
}
```

No schema version field for the prototype — the shape is pinned to the container image version. If the schema evolves, the image tag changes, and any Clipbuddy built against an older schema is simply incompatible with the newer image.

Per-discerner finding schemas:

**Stanford finding:**
```json
{
  "text": "Margaret J. Thornton",
  "start": 45,
  "end": 65,
  "label": "PATIENT",
  "confidence": 0.987
}
```
- `start`/`end` are character offsets into `apcsgt_normalized_text`.
- `label` is one of the 8 native labels (non-`O` values only; `O` tokens are not emitted as findings).
- `confidence` is the model's softmax confidence for the dominant label over the span.

**spaCy finding.** Two variants share a single discerner namespace; each finding is either token-level or entity-level, distinguished by a `kind` field:

```json
{
  "kind": "token",
  "text": "may",
  "start": 120,
  "end": 123,
  "pos": "AUX",
  "tag": "MD",
  "morph": "VerbType=Mod",
  "lemma": "may",
  "head": 5,
  "dep": "aux"
}
```

```json
{
  "kind": "entity",
  "text": "aspirin",
  "start": 84,
  "end": 91,
  "label": "CHEMICAL"
}
```

**Stanza finding.** Same two-variant pattern:

```json
{
  "kind": "token",
  "text": "Margaret",
  "start": 45,
  "end": 53,
  "upos": "PROPN",
  "xpos": "NNP",
  "feats": "Number=Sing",
  "lemma": "Margaret",
  "head": 2,
  "deprel": "nsubj",
  "ner": "B-PERSON"
}
```

```json
{
  "kind": "entity",
  "text": "Margaret J. Thornton",
  "start": 45,
  "end": 65,
  "label": "PERSON"
}
```

These schemas are illustrative for the prototype — concrete field ordering, optionality, and type precision will be fixed by the Rust deserializer types in `apcd/src/` when the consumer is written.

### Security Posture

| Control | Setting | Rationale |
|---------|---------|-----------|
| Network | `--network=none` | The container cannot initiate or accept any network traffic — eliminates PHI exfiltration risk |
| Capabilities | `--cap-drop=all` | No Linux capabilities beyond the bare minimum needed to run Python and write to the bind mount |
| Filesystem | Read-only root filesystem (`--read-only`), writable bind-mount only | The container can mutate only the bind-mounted directory |
| User | Non-root (explicit `USER` directive in Dockerfile) | Standard container hygiene; bind-mount file ownership maps to host UID/GID |
| Resource limits | None (prototype scale) | Memory and CPU caps deferred until empirical usage is observed |

### Rationale Summary

- **Python ecosystem unlocked**: scispaCy, Stanza, HuggingFace transformers run natively without ONNX export ceremony or Rust-Python FFI.
- **Combining stays in Rust**: strongly typed deserialization, full test coverage on the combining logic, unchanged language boundary.
- **Zero-network container**: PHI cannot leave the container over the network because there is no network. Stronger posture than a host-side Python venv.
- **File-I/O wire protocol**: the simplest possible interface — debuggable by eyeballing a directory, no protocol parser to write or maintain.
- **External lifecycle**: container up/down is a developer-directed action, decoupled from Clipbuddy startup. Simplifies the prototype's failure modes.

## Dictionary Data

### Proof-of-Life Sizing

Curated-small dictionaries that prove the architecture without requiring full datasets. Each paired with its own scan (see Dictionary Scans above):

| Dictionary | Source | License | Size | File |
|------------|--------|---------|------|------|
| Surnames | US Census 2010, top 1,000 by frequency | Public domain | ~1,000 entries | `dictionaries/surnames.txt` |
| First names | SSA baby names, top 1,000 | Public domain | ~1,000 entries | `dictionaries/firstnames.txt` |
| Cities | US state capitals + top 100 cities by population | Public domain | ~150 entries | `dictionaries/cities.txt` |
| Medical whitelist | Hand-curated: common medications, lab tests, diagnoses, anatomy, abbreviations | Original curation | ~500 entries | `dictionaries/medical_whitelist.txt` |
| English whitelist | Top 5,000 common English words | Public domain | ~5,000 entries | `dictionaries/english_whitelist.txt` |

Total: ~8,000 entries, ~80KB on disk.

### Full Dataset Sizing (Future)

| Dictionary | Source | License | Size |
|------------|--------|---------|------|
| Surnames | US Census 2010, complete | Public domain | ~160,000 |
| First names | SSA baby names, complete | Public domain | ~100,000 |
| Cities | US Census incorporated places | Public domain | ~30,000 |
| Medical whitelist | SNOMED-CT + RxNorm + LOINC subsets | NLM license (redistribution constraints) | ~100,000+ |
| English whitelist | Full dictionary | Public domain | ~170,000 |

## Anonymization

Combining is deferred. The prototype's current anonymization derives from Rust-only findings (regex, label, and dictionary-style scans) via a stand-in that approximates the post-combining shape. The full combining-aware anonymization will land in a later pace once combining is spec'd.

### Placeholder Strategy

Category-specific placeholders preserve semantic context for Open Evidence:

| PHI Category | Placeholder | Example |
|-------------|-------------|---------|
| Patient/person names | `[NAME]` | "Ms. [NAME] is a 74-year-old female" |
| Dates (non-DOB) | `[DATE]` | "Visit Date: [DATE]" |
| Date of birth | `Age: NN` or `Age: 90+` | "DOB: 03/15/1952" → "Age: 74"; ages >89 → "Age: 90+" |
| Medical record number | `[MRN]` | "MRN: [MRN]" |
| Phone numbers | `[PHONE]` | "phone: [PHONE]" |
| Addresses | `[ADDRESS]` | "at [ADDRESS]" |
| Email addresses | `[EMAIL]` | |
| Social Security numbers | `[SSN]` | |
| Facility names | `[FACILITY]` | "Seen at [FACILITY]" |
| Provider names | `[PROVIDER]` | "Dr. [PROVIDER]" |
| Account numbers | `[ACCOUNT]` | |
| Encounter IDs | `[ENCOUNTER_ID]` | |

### Output Format

Plain text with preserved section structure (line breaks, section headers). HTML formatting is stripped — this is itself a privacy measure, removing embedded metadata and hidden fields. The output should be readable when pasted into Open Evidence's chat input.

## Self-Update Mechanism

Currently **dormant** for the prototype. `apcru_start_watcher()` is commented out in `apcap_main.rs::main()`; `apcru_update.rs` source is retained for trivial revert. Ann's deploy ceremony is a manual quit + relaunch of `Apcap.app` from `/Users/Shared/apcua/`. The seamless hot-swap-during-active-session problem is not one Ann has today.

When re-enabled, the watcher architecture is: `/Users/Shared/apcua/` watched via `notify` (FSEvents on macOS). On detection of a new `.app` bundle:

1. Copy new bundle over current application path
2. Spawn `open -n <app-path>` to launch new version
3. `std::process::exit(0)` to terminate current process

## Project Structure

```
Tools/apck/
  apck-claude-context.md          # Build/run/deploy instructions
  APCAS-Specification.md          # Application spec (UX vision)
  APCPS-PrototypeSpecification.md # This document (pipeline engineering)
  APCS0-SpecTop.adoc              # Detection pipeline vocabulary
  apcw_workbench.sh               # Bash workbench
  apcz_zipper.sh                  # Zipper enrollment
  test_fixtures/
    epic_progress_note.html       # Synthetic Epic clipboard data
  apcd/                           # Rust/Tauri source directory
    Cargo.toml                    # Multiple [[bin]]: apcap, apcal, apcad, apcab, apcnsa
    src/
      apcap_main.rs               # Tauri app entry point
      apcal_main.rs               # Fixture loader entry point
      apcab_main.rs               # Batch assay binary
      apcad_main.rs               # Dictionary refresh binary
      apcnsa_main.rs              # Historical Stanford ONNX spike binary (reference only)
      lib.rs                      # Engine library + test module declarations
      apcre_engine.rs             # PHI detection orchestrator
      apcte_engine.rs             # Tests for engine
      apcrp_parse.rs              # HTML clipboard parsing
      apctp_parse.rs              # Tests for parser
      apcrm_match.rs              # Dictionary/regex matching
      apctm_match.rs              # Tests for matching
      apcrd_dictionaries.rs       # Dictionary loading
      apctd_dictionaries.rs       # Tests for dictionaries
      apcru_update.rs             # Directory watcher + self-update (no unit tests; currently dormant)
      apcrh_harvest.rs            # Clipboard harvest orchestrator
      apcth_harvest.rs            # Tests for harvest
      apcrj_journal.rs            # Journal directory path resolver
      apcrl_log.rs                # Logging macros + file-tee sink
      apcrb_pasteboard.rs         # macOS NSPasteboard FFI — declared-UTI enumeration
      apctb_pasteboard.rs         # Tests for pasteboard
    ui/
      index.html
      style.css
    dictionaries/
      surnames.txt
      firstnames.txt
      cities.txt
      medical_whitelist.txt
      english_whitelist.txt
    container/                    # Container build inputs (new)
      Dockerfile                  # Python 3.11-slim + NLP stack
      requirements.txt            # transformers, torch-cpu, scispacy, spacy, en_core_sci_md, stanza
      entrypoint.py               # Long-running loop: watch bind mount, run 3 discerners, atomic publish
      discerners/                 # Per-algorithm Python modules
        stanford.py
        spacy_scan.py
        stanza_scan.py
```

The `container/` subdirectory under `apcd/` holds the Dockerfile and Python sources for the container image. Keeping it adjacent to the Rust application source (rather than in a separate top-level directory) reflects that the two halves — Rust app + container — are one system, built and deployed together.

## Prefix Tree

```
apc  (non-terminal)
├── apca   (non-terminal)
│   ├── apcab  — App Batch binary (assay — detection pipeline on HTML files)
│   ├── apcad  — App Dictionary binary (refresh from public sources)
│   ├── apcal  — App Loader binary (fixture clipboard tool)
│   ├── apcap  — App Prototype binary (Tauri main)
│   └── apcas  — application specification document (UX, workflow)
├── apcc   — CLI command implementations
├── apcd   — Rust/Tauri source directory
│   ├── apcrb  — macOS NSPasteboard FFI — declared-UTI enumeration for harvest
│   ├── apcrh  — Clipboard harvest orchestrator (delegates enumeration to apcrb)
│   ├── apcrj  — Journal directory path resolver
│   └── apcrl  — Logging macros (info, error, fatal with file/line) + file-tee sink
├── apcj   (non-terminal — journal)
│   └── apcjd  — journal directory ($HOME/apcjd/) — shared by app and container
├── apck   — kit directory
├── apcn   (non-terminal — neural)
│   └── apcns  (non-terminal — neural stanford)
│       └── apcnsa  — App Neural Stanford Assay binary (historical ONNX spike, reference only)
├── apcps  — prototype specification document
├── apcs   (non-terminal)
│   └── apcs0  — detection pipeline specification (MCM concept model)
├── apcu   (non-terminal)
│   └── apcua — update staging directory (/Users/Shared/apcua/)
├── apcw   — workbench
└── apcz   — zipper
```

Rust source file prefixes follow RCG: `{cipher}r{classifier}_{name}.rs` where cipher is `apc`. Test files: `{cipher}t{classifier}_{name}.rs` — classifier matches between source and test.

## Tabtargets

| Tabtarget | Colophon | Purpose |
|-----------|----------|---------|
| `tt/apcw-b.Build.sh` | `apcw-b` | `cargo tauri build` (release) |
| `tt/apcw-r.Run.sh` | `apcw-r` | `cargo tauri dev` (local development) |
| `tt/apcw-D.Deploy.sh` | `apcw-D` | Build + scp to `anns-macbook-air:/Users/Shared/apcua/` |
| `tt/apcw-fl.FixtureLoad.sh` | `apcw-fl` | Run `apcal` to load fixture HTML onto clipboard |
| `tt/apcw-t.Test.sh` | `apcw-t` | `cargo test` in `apcd/` |
| `tt/apcw-dr.DictionaryRefresh.sh` | `apcw-dr` | `cargo run --bin apcad` (refresh dictionaries) |
| `tt/apcw-ba.BatchAssay.sh` | `apcw-ba` | `cargo run --bin apcab` (batch assay on HTML directory) |
| `tt/apcw-nsa.NeuralStanfordAssay.sh` | `apcw-nsa` | Historical Stanford ONNX spike binary (reference only) |
| `tt/apcw-nsi.NeuralStanfordInstall.sh` | `apcw-nsi` | Historical Stanford ONNX install (reference only) |

Container lifecycle tabtargets (names pending mint; placement under `apcw-c*` colophon family to distinguish from file-system lifecycle):

| Tabtarget (proposed) | Purpose |
|---------------------|---------|
| `apcw-cb.ContainerBuild.sh` | `docker build` the container image |
| `apcw-cs.ContainerStart.sh` | Truncate `container-log.txt`, `docker run` the container with bind mount and security flags |
| `apcw-cx.ContainerStop.sh` | `docker stop` + `docker rm` |
| `apcw-ci.ContainerStatus.sh` | Report running state, bound image tag, bind-mount reachability |

Final names will be minted when the container lifecycle is implemented.
