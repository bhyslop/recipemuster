# Ann's PHI Clipbuddy — Prototype Specification

## Scope

This document specifies the engineering pipeline for the proof-of-life prototype. It covers detection algorithms, tech stack, data sources, project structure, and scope boundaries. It does NOT cover the user experience — see [APCS-Specification.md](APCS-Specification.md) for the UX plan.

This is a living document. The pipeline will evolve rapidly during prototype development. Changes here do not require parallel updates to the product spec.

## Scope Boundaries

The prototype deliberately excludes:

- **No persistence** — no logs, no audit trail, no history of anonymized notes
- **One patient at a time** — no batch processing, no queue
- **No configuration** — no custom blacklists, no user preferences, no settings UI
- **No click-to-highlight** — clicking a finding does not scroll/highlight in the document preview (future feature)
- **No clipboard source detection** — the app does not verify that clipboard content came from Epic
- **No clipboard clearing** — clipboard is not cleared after consumption (reserved for future)

## Tech Stack

| Layer | Crate | Version | License | Rationale |
|-------|-------|---------|---------|-----------|
| Clipboard | `arboard` | 3.x | MIT/Apache | Only crate with cross-platform HTML clipboard read (`get_html()`) |
| HTML parsing | `scraper` | latest | MIT | DOM + CSS selectors, built on Mozilla's html5ever |
| Dictionary matching | `aho-corasick` | latest | MIT/Unlicense | Single-pass multi-pattern O(n) regardless of dictionary size |
| Pattern matching | `regex` | latest | MIT/Apache | Structural PHI patterns |
| Word boundaries | `unicode-segmentation` | latest | MIT/Apache | UAX#29 compliant word boundary detection |
| GUI shell | `tauri` | 2.x | MIT | HTML/CSS/JS frontend, Rust backend, system webview |
| File watching | `notify` | latest | MIT/Apache | FSEvents on macOS, ReadDirectoryChanges on Windows |

All engine crates are pure Rust, fully Cargo-lockable on macOS and Windows. Tauri requires the system webview (WebKit on macOS, WebView2 on Windows — both ship with the OS).

## Input: Epic "Copy All" HTML

Epic's "Copy All" places HTML on the clipboard with structural formatting:

- `<b>Label:</b> value` patterns for demographics and metadata
- Section headers for clinical categories (Chief Complaint, HPI, PMH, Medications, Vitals, Labs, Assessment/Plan)
- Narrative freetext in the History of Present Illness and other sections
- Signature blocks with provider names and timestamps

The HTML structure is semi-structured — not arbitrary prose and not fully machine-readable. This structure is the primary advantage over plain text processing: labels anchor PHI detection with high precision.

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

Unambiguous structural patterns. No false positives. No database dependency.

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

Items caught by Tier 2 are not subsequently dictionary-checked — their RED classification is final.

### Tier 3 — Dictionary Blacklist/Whitelist (→ YELLOW or PASS)

Catches names and locations in narrative text that lack label anchors (e.g., "her husband Robert Thornton called 911").

**Process:**
1. Segment text into words using `unicode-segmentation` (UAX#29 boundaries)
2. Run aho-corasick automaton against all words (single pass, O(n))
3. For each hit, check whitelist membership
4. Classify:
   - Blacklist hit, no whitelist hit → **YELLOW** (questionable, default ELIDE)
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
  APCS-Specification.md           # Product spec (UX vision)
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
      lib.rs                      # Engine library
      apcre_engine.rs             # PHI detection orchestrator
      apcrp_parse.rs              # HTML clipboard parsing
      apcrm_match.rs              # Dictionary/regex matching
      apcrd_dictionaries.rs       # Dictionary loading
      apcru_update.rs             # Directory watcher + self-update
    ui/
      index.html
      style.css
      app.js
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
│   ├── apcal  — App Loader binary (fixture clipboard tool)
│   └── apcap  — App Prototype binary (Tauri main)
├── apcd   — Rust/Tauri source directory
├── apck   — kit directory
├── apcps  — prototype specification document
├── apcs   — product specification document
├── apcu   (non-terminal)
│   └── apcua — update staging directory (/Users/Shared/apcua/)
├── apcw   — workbench
└── apcz   — zipper
```

Rust source file prefixes follow RCG: `{cipher}r{classifier}_{name}.rs` where cipher is `apc`. See RCG (Rust Coding Guide) for full naming conventions, including `z`-prefix internal declarations, string boundary discipline, and constant discipline.

## Tabtargets

| Tabtarget | Colophon | Purpose |
|-----------|----------|---------|
| `tt/apcw-b.Build.sh` | `apcw-b` | `cargo tauri build` (release) |
| `tt/apcw-r.Run.sh` | `apcw-r` | `cargo tauri dev` (local development) |
| `tt/apcw-d.Deploy.sh` | `apcw-d` | Build + scp to `anns-macbook-air:/Users/Shared/apcua/` |
| `tt/apcw-fl.FixtureLoad.sh` | `apcw-fl` | Run `apcal` to load fixture HTML onto clipboard |
