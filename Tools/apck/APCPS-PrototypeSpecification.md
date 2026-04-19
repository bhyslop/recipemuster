# Ann's PHI Clipbuddy — Prototype Specification

## Scope

This document specifies the engineering pipeline for the proof-of-life prototype. It covers detection algorithms, tech stack, data sources, project structure, and scope boundaries. It does NOT cover the user experience — see [APCAS-Specification.md](APCAS-Specification.md) for the UX plan.

This is a living document. The pipeline will evolve rapidly during prototype development. Changes here do not require parallel updates to the product spec.

## Scope Boundaries

The prototype deliberately excludes:

- **No JavaScript** — All rendering logic is Rust. The Tauri webview is a passive display surface receiving complete HTML from the Rust backend. Inline `onclick` attributes use the Tauri bridge primitive (`window.__TAURI__.core.invoke`) for command dispatch only — no `.js` files, no JS application logic, no JS state management. This eliminates the language-boundary bug class (e.g., byte-offset vs char-offset mismatch at the Rust-JS serialization boundary).
- **No persistence** — no logs, no audit trail, no history of anonymized notes
- **One patient at a time** — no batch processing, no queue
- **No configuration** — no custom blacklists, no user preferences, no settings UI
- **No click-to-highlight** — clicking a finding does not scroll/highlight in the document preview (future feature)
- **No multi-word merge** — adjacent name hits are individually flagged, not merged into a single token

## Tech Stack

| Layer | Crate | Version | License | Rationale |
|-------|-------|---------|---------|-----------|
| Clipboard | `arboard` | 3.x | MIT/Apache | Only crate with cross-platform HTML clipboard read (`get_html()`) |
| HTML parsing | `scraper` | latest | MIT | DOM + CSS selectors, built on Mozilla's html5ever |
| Dictionary matching | `aho-corasick` | latest | MIT/Unlicense | Single-pass multi-pattern O(n) regardless of dictionary size |
| Pattern matching | `regex` | latest | MIT/Apache | Structural PHI patterns |
| Word boundaries | `unicode-segmentation` | latest | MIT/Apache | UAX#29 compliant word boundary detection |
| GUI shell | `tauri` | 2.x | MIT | Rust-rendered HTML/CSS frontend, system webview as passive display |
| File watching | `notify` | latest | MIT/Apache | FSEvents on macOS, ReadDirectoryChanges on Windows |

All crates are pure Rust, fully Cargo-lockable on macOS and Windows. Tauri requires the system webview (WebKit on macOS, WebView2 on Windows — both ship with the OS). The webview receives pre-rendered HTML from Rust on every state change; no JavaScript application code exists in the project.

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

If clinical content is NOT detected, the app clears any previous triage state and returns to the initial instruction view. The app displays a brief diagnostic line showing: clipboard content length, content type (HTML/plain text), and the first ~100 characters of content. This diagnostic aids debugging when real Epic data fails the heuristic — the developer can see what was on the clipboard and update the label list accordingly.

This heuristic will be refined when tested against real Epic output. The initial label list is calibrated to pass the synthetic test fixture.

### Clipboard Change Detection

On window focus, the engine compares the current clipboard content (byte-for-byte string comparison) against the last successfully consumed content. If identical, the existing triage state is preserved — no reprocessing. If different, the clinical content heuristic runs on the new content. Clinical notes are a few KB; storing the last consumed string for comparison has negligible memory cost at prototype scale.

## Journal Directory

**Purpose.** One place on disk that accumulates runtime artifacts produced by the app: verbatim clipboard harvests on the Clinical branch, and a running log of every `apcrl_*` emission teed from stdout. A single location keeps the mental model honest — one `ls` answers both "what did the app capture?" and "what did the app report?".

**Path.** `$HOME/apcjd/` — outside any repo tree. The directory is created lazily at application startup (for the log tee) or on first capture (for harvests), whichever comes first. A `.gitignore` entry for `apcjd/` is present as belt-and-suspenders against a misdirected `$HOME` resolution that lands in-tree.

**Contents.** Two independent features share this directory:

| Artifact | Producer | Shape |
|----------|----------|-------|
| Harvest captures | `apcrh_harvest` on Clinical branch | `{N}.{ext}` (see Clipboard Harvest) |
| Observability log | `apcrl_log` file-tee sink | `apcap.log` (see Observability Log) |

## Clipboard Harvest

**Purpose.** Preserve verbatim real-world clipboard contents on Clinical-branch detection so the fixture library can grow from real traffic and so Epic's actual flavor structure can be studied. The prototype currently ships with synthetic fixtures only; harvest is the bridge from lab to field without relying on the clinician to remember to save.

**Trigger.** The Clinical branch of clipboard analysis — executed before the system-clipboard zero-out. Non-clinical content is never harvested.

**Storage.** The journal directory (`$HOME/apcjd/`, see above).

**Naming.** `{N}.{ext}` where `N` seeds at 10000 for an empty directory, otherwise `max_existing_numeric_stem + 1`. Files with the same `N` group all flavors of one capture (e.g., `10000.txt` and `10000.html` are the two flavors of capture 10000). Gaps in the numeric sequence are not filled — the scan advances past the current maximum. Non-numeric filenames are ignored in the max calculation, so `apcap.log` and any user-placed `README` or `notes` co-exist without perturbing indexing.

**Flavors.** Every flavor the `arboard` abstraction exposes at the time of capture:

| Flavor | Extension | Present when |
|--------|-----------|--------------|
| Plain text (`get_text`) | `.txt` | Always — required for Clinical branch entry |
| HTML (`get().html()`) | `.html` | Epic "Copy All" places HTML; absence is possible and non-fatal |

RTF and image flavors are documented gaps: `arboard` 3.x does not surface RTF, and image capture requires the `image-data` feature which this project does not enable. Dropping to platform-specific pasteboard APIs to close these gaps is explicitly out of scope for the prototype.

**Failure mode.** A capture failure logs via `apcrl_error_now!` (stdout + tee) and does not abort triage. User-visible behavior is unchanged whether harvest succeeds or fails — the triage pipeline is authoritative.

**Privacy posture.** PHI-at-rest stays outside the repo. Captures are never auto-committed, auto-uploaded, or auto-anonymized. Anonymization and promotion to `test_fixtures/` are manual, out-of-band operations. The clinician-developer coordinates capture review separately.

## Observability Log

**Purpose.** Give the clinician-developer and the engineer a shared, persistent view of what the app did across runs. The `.app` bundle on macOS captures stdout where a non-technical user won't look for it; a file in the journal directory is where both humans already know to peek.

**Mechanism.** `apcrl_log` exposes an optional file-tee sink: `apcrl_tee_init(path)` installs a once-only append-open handle. Every subsequent `apcrl_*_now!`, `_if!`, and comparison-variant emission is written twice — once to stdout (verbatim) and once to the tee file (same format, same line). One format, two sinks.

**Format.** Exactly what stdout receives: `[LEVEL] [file:line] message`. Single emission path means no format drift between modalities — an engineer debugging a remote bundle and an engineer running `cargo run` read the same lines.

**Location.** `$HOME/apcjd/apcap.log`. Filename matches the binary name (`apcap`) as an engineer mnemonic. Append-only — no rotation at prototype scale; a clinical session produces tens of lines, a year of use produces thousands.

**Init.** At `main()` startup, before any other log emission. Failure to install the tee (missing HOME, disk full, permissions) logs to stdout and proceeds — the app starts and stdout remains authoritative.

**Failure mode.** Tee write failures are silently swallowed inside the emitter to avoid recursive-logging hazards (a failing log should not generate more log traffic). Stdout is the authoritative sink; tee is best-effort.

**Privacy posture.** The log lives alongside harvests in the journal directory and inherits the same posture — the whole directory is PHI-permissive by design. The log may carry clipboard content, PHI-adjacent details, or anything else useful to diagnose behavior on the clinician's machine. The discipline that matters is not "keep the log PHI-free" but "keep the journal directory on the doctor's computer" — nothing in `$HOME/apcjd/` is auto-uploaded, auto-committed, or transmitted off-device. Review and promotion of any artifact (capture or log line) to an out-of-site context is a manual act.

## Detection Pipeline

### Overview

Three tiers execute in sequence. Results merge by highest severity — if a token is flagged by multiple tiers, the most severe classification wins.

```
Input: Epic HTML clipboard content
         |
    Parse HTML → typed spans with labels and positions
         |
    Tier 1: Regex scan → RED
         |
    Tier 2: Label-anchored extraction → RED
         |
    Tier 3: Dictionary blacklist/whitelist scan → YELLOW or PASS
         |
    Merge by highest severity
         |
    Output: classified token stream → frontend
```

### Tier 1 — Regex Patterns (→ RED)

Structural patterns with low false positive rate. No database dependency.

**Known ambiguities:** The zip code regex (`\d{5}`) can match 5-digit lab values, identifiers, or dosage amounts. The date regex can match version numbers or other formatted numerics. The street address regex can match non-address text with number-word-suffix patterns. These are accepted risks for the prototype — the conservative bias (false positive > false negative) means these appear as RED items the clinician can review. Context-aware refinement is deferred.

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

### Tier 2 — Label-Anchored Extraction (→ RED)

Words immediately following known Epic labels are PHI regardless of dictionary membership. The label is authoritative — if Epic says `Patient: Xylophone McZephyr`, that's a name even if no database contains it.

| Label Pattern | Category | Extraction Rule |
|--------------|----------|-----------------|
| `Patient:` | `[NAME]` | All text until next label or line break |
| `Attending:` | `[PROVIDER]` | All text until next label or line break |
| `Provider:` | `[PROVIDER]` | All text until next label or line break |
| `Referring:` | `[PROVIDER]` | All text until next label or line break |
| `Facility:` | `[FACILITY]` | All text until next label or line break |
| `Electronically signed by:` | `[PROVIDER]` | All text until next label or line break |
| `Emergency contact` | `[NAME]` | Name portion, phone extracted by Tier 1 |
| `primary care physician` | `[PROVIDER]` | Following name (e.g., "Dr. Susan Chen") |

Label matching is case-insensitive on the label text. Value extraction extends to the next structural boundary (label, line break, or HTML element boundary).

**Per-word anchoring:** Each word in the extracted value is individually flagged as RED. "Margaret J. Thornton" after `Patient:` produces three separate **anchored** items, each classified `[NAME]`. In anonymized output: `[NAME] [NAME] [NAME]`. No multi-word merge is attempted.

Items caught by Tier 2 are **anchored** — their RED classification is final. They are not subsequently dictionary-checked. If a word is both anchored (Tier 2) and matched (Tier 3), the anchored classification takes precedence.

### Vocabulary: Anchored vs. Matched

Two independent mechanisms flag words as potential PHI. To avoid confusion, the spec distinguishes:

- **Anchored**: Flagged by Tier 2 because of *position* — the word follows a known label. Always RED. Independent of any dictionary.
- **Matched**: Flagged by Tier 3 because of *identity* — the word appears in a name/location dictionary. YELLOW (unless also anchored, in which case RED wins).

A word can be both anchored and matched (e.g., "Thornton" after `Patient:` AND in the Census surname list). Anchored always takes precedence. The term "blacklist" in this spec refers specifically to the Tier 3 name/location dictionaries; Tier 2 label-anchored extraction is a separate mechanism.

### Tier 3 — Dictionary Blacklist/Whitelist (→ YELLOW or PASS)

Catches names and locations in narrative text that lack label anchors (e.g., "her husband Robert Thornton called 911").

**Case handling:** All dictionary lookups are case-insensitive. The aho-corasick automaton is built from lowercase dictionary entries. Input tokens are lowercased before matching. Original case is preserved in display and anonymized output.

**Process:**
1. Segment text into words using `unicode-segmentation` (UAX#29 boundaries)
2. Lowercase each token for lookup; preserve original for display
3. Run aho-corasick automaton against lowercased tokens (single pass, O(n))
4. For each hit, check whitelist membership (also lowercase)
5. Classify:
   - Blacklist hit, no whitelist hit → **YELLOW** (matched, default ELIDE)
   - Blacklist hit AND whitelist hit → **YELLOW** (collision, default ELIDE)
   - Whitelist hit only → **PASS**
   - Neither list → **PASS**

**Collision resolution context signals** (applied within YELLOW to aid clinician review):
- Capitalization mid-sentence suggests a proper noun (name) rather than common word
- Adjacent blacklist hits suggest a full name ("Robert Thornton" — two hits together)
- Section context: names in demographics header vs. narrative HPI carry different weight

These signals inform the findings panel display but do not change the YELLOW classification in the prototype. The clinician makes the final call.

## Dictionary Data

### Proof-of-Life Sizing

Curated-small dictionaries that prove the architecture without requiring full datasets:

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

The application watches `/Users/Shared/apcua/` for new `.app` bundles using the `notify` crate (FSEvents backend on macOS). On detection:

1. Copy new bundle over current application path
2. Spawn `open -n <app-path>` to launch new version
3. `std::process::exit(0)` to terminate current process

No confirmation dialog. The watcher runs on a background thread independent of the main Tauri event loop. If the watch directory does not exist at startup, the app creates it silently.

## Project Structure

```
Tools/apck/
  apck-claude-context.md          # Build/run/deploy instructions (RCG reference)
  APCAS-Specification.md          # Application spec (UX vision)
  APCPS-PrototypeSpecification.md # This document (pipeline engineering)
  apcw_workbench.sh               # Bash workbench
  apcz_zipper.sh                  # Zipper enrollment
  test_fixtures/
    epic_progress_note.html       # Synthetic Epic clipboard data
  apcd/                           # Rust/Tauri source directory
    Cargo.toml                    # Two [[bin]]: apcap (app), apcal (fixture loader)
    src/
      apcap_main.rs               # Tauri app entry point
      apcal_main.rs               # Fixture loader entry point
      lib.rs                      # Engine library + test module declarations
      apcre_engine.rs             # PHI detection orchestrator
      apcte_engine.rs             # Tests for engine
      apcrp_parse.rs              # HTML clipboard parsing
      apctp_parse.rs              # Tests for parser
      apcrm_match.rs              # Dictionary/regex matching
      apctm_match.rs              # Tests for matching
      apcrd_dictionaries.rs       # Dictionary loading
      apctd_dictionaries.rs       # Tests for dictionaries
      apcru_update.rs             # Directory watcher + self-update (no unit tests)
    ui/
      index.html
      style.css
    dictionaries/
      surnames.txt
      firstnames.txt
      cities.txt
      medical_whitelist.txt
      english_whitelist.txt
```

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
│   ├── apcrh  — Clipboard harvest module (writes to journal directory)
│   ├── apcrj  — Journal directory path resolver
│   └── apcrl  — Logging macros (info, error, fatal with file/line) + file-tee sink
├── apcj   (non-terminal — journal)
│   └── apcjd  — journal directory ($HOME/apcjd/) holding harvests + apcap.log
├── apck   — kit directory
├── apcps  — prototype specification document
├── apcs   (non-terminal)
│   └── apcs0  — detection pipeline specification (MCM concept model)
├── apcu   (non-terminal)
│   └── apcua — update staging directory (/Users/Shared/apcua/)
├── apcw   — workbench
└── apcz   — zipper
```

Rust source file prefixes follow RCG: `{cipher}r{classifier}_{name}.rs` where cipher is `apc`. Test files: `{cipher}t{classifier}_{name}.rs` — classifier matches between source and test. See RCG (Rust Coding Guide) for full naming conventions, including `z`-prefix internal declarations, string boundary discipline, and constant discipline.

`lib.rs` wiring:
```rust
pub mod apcre_engine;
pub mod apcrp_parse;
pub mod apcrm_match;
pub mod apcrd_dictionaries;
pub mod apcru_update;
pub mod apcrh_harvest;
pub mod apcrj_journal;

#[cfg(test)] mod apcte_engine;
#[cfg(test)] mod apctp_parse;
#[cfg(test)] mod apctm_match;
#[cfg(test)] mod apctd_dictionaries;
#[cfg(test)] mod apcth_harvest;
```

## Tabtargets

| Tabtarget | Colophon | Purpose |
|-----------|----------|---------|
| `tt/apcw-b.Build.sh` | `apcw-b` | `cargo tauri build` (release) |
| `tt/apcw-r.Run.sh` | `apcw-r` | `cargo tauri dev` (local development) |
| `tt/apcw-D.Deploy.sh` | `apcw-D` | Build + scp to `anns-macbook-air:/Users/Shared/apcua/` |
| `tt/apcw-fl.FixtureLoad.sh` | `apcw-fl` | Run `apcal` to load fixture HTML onto clipboard |
| `tt/apcw-t.Test.sh` | `apcw-t` | `cargo test` in `apcd/` |
