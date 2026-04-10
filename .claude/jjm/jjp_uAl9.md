## Character

Product design into prototype implementation. Starts with synthesis (specs capturing a rich design conversation), transitions to exploratory engineering (Tauri + Rust + cross-platform clipboard), then steady implementation of a three-tier PHI detection engine. Requires careful attention to minting discipline — this kit lives alongside RBK/JJK/BUK and must respect the prefix universe.

## Problem

Clinicians using Epic EHR need to consult Open Evidence (an AI diagnostic tool) but must not paste Protected Health Information (PHI) into it. HIPAA Safe Harbor defines 18 PHI categories that must be removed. Today, clinicians either skip the consultation or manually scrub text — both are unacceptable. No clipboard-level tool exists to automate the triage.

## Solution

Ann's PHI Clipbuddy (`apcap`) — a Tauri desktop app (macOS, later Windows) that:
1. Consumes clipboard content when the app receives focus
2. Parses Epic "Copy All" HTML for structural context (labels, sections)
3. Runs a three-tier PHI detection engine (regex → label-anchored → dictionary)
4. Displays a triage view: RED (definite PHI), YELLOW (questionable), GREY (pass)
5. Lets clinician toggle individual findings between ELIDE and PASS
6. Writes anonymized plain text to clipboard with category-specific placeholders
7. Self-updates from a watched directory without user confirmation

## Tech Stack

| Layer | Choice | Notes |
|-------|--------|-------|
| Clipboard | `arboard` | Only crate with cross-platform HTML clipboard read |
| HTML parsing | `scraper` | DOM + CSS selectors, built on html5ever |
| Dictionary matching | `aho-corasick` | Single-pass multi-pattern, O(n) regardless of dictionary size |
| Pattern matching | `regex` | Structural PHI patterns (SSN, phone, date, address) |
| Word boundaries | `unicode-segmentation` | UAX#29 compliant |
| GUI shell | Tauri 2.x | HTML/CSS/JS frontend, Rust backend |
| File watching | `notify` | FSEvents on macOS, ReadDirectoryChanges on Windows |

All engine crates are pure Rust, fully Cargo-lockable on macOS and Windows.

## Detection Architecture

Three tiers, merged by highest severity:

**Tier 1 — Regex (→ RED):** SSN, phone, email, dates (with DOB→age transform), street addresses, zip codes, MRN/account/encounter patterns following labels.

**Tier 2 — Label-anchored (→ RED):** Words following Epic labels (`Patient:`, `Attending:`, `Facility:`, `Electronically signed by:`, etc.) are names/facilities regardless of dictionary membership. The label is authoritative.

**Tier 3 — Dictionary blacklist/whitelist (→ YELLOW or PASS):**
- Blacklist-only hit → YELLOW (questionable, default ELIDE)
- Blacklist + whitelist collision → YELLOW
- Whitelist-only or neither list → PASS

Proof-of-life dictionaries are curated-small:
- Top 1,000 surnames (Census, public domain)
- Top 1,000 first names (SSA, public domain)
- State capitals + top 100 cities (Census, public domain)
- ~500 medical terms (curated, no SNOMED license needed)
- Top 5,000 common English words

~8k entries total, ~80KB. Full datasets (160k surnames, 100k firstnames, 30k cities, SNOMED/RxNorm) deferred to later heat.

## Anonymization Strategy

Category-specific placeholders: `[NAME]`, `[DATE]`, `[MRN]`, `[PHONE]`, `[ADDRESS]`, `[EMAIL]`, `[SSN]`, `[FACILITY]`, `[PROVIDER]`, `[ACCOUNT]`, `[ENCOUNTER_ID]`.

DOB→age transform: `DOB: 03/15/1952` becomes `Age: 74` (clinically useful, not PHI).

Output is plain text — formatting stripped is itself a privacy measure.

## UX Model

On focus: consume clipboard, run detection, display triage view.

Preview pane shows the document with inline highlighting:
- RED boxes: definite PHI (elided in output)
- YELLOW boxes: questionable (default elide, clinician can toggle to pass)
- GREY text: safe content (passes through)

Findings panel lists each detection with category and [ELIDE ▼] / [PASS ▼] toggle.

"Copy Anonymized to Clipboard" button writes clean output.

No persistence. One patient at a time. No configuration. No click-to-highlight-in-preview (future).

## Self-Update

App watches `/Users/Shared/apcua/` using `notify` crate. On new `.app` bundle appearing:
- Copy new version over self
- Spawn `open -n` on new version
- Exit current process

No confirmation dialog. Fully automatic.

## Deployment

`tt/apcw-d.Deploy.sh` builds and scps to `anns-macbook-air:/Users/Shared/apcua/`. Ann launches independently.

## Project Structure

```
Tools/apck/
  apck-claude-context.md          # Build/run/deploy instructions (RCG reference)
  APCS-Specification.md           # Product spec
  APCPS-PrototypeSpecification.md # Prototype spec
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

Rust source file prefixes follow RCG: `apcr{classifier}_{name}.rs`.

## Redistribution Notes

Engine stack: pure Rust, MIT/Apache, no concerns. Tauri: MIT. Census/SSA dictionaries: public domain. Medical whitelist: hand-curated for prototype avoids SNOMED/RxNorm NLM licensing. Full medical terminology (SNOMED, RxNorm, LOINC) requires NLM redistribution terms if shipped — defer to product maturity.

## Testing Strategy

Synthetic Epic HTML fixture in `test_fixtures/`. Fixture loader (`apcal`) puts HTML on clipboard using `arboard`. Iterate detection algorithms locally before deploying to Ann for validation against real Epic output.

## Open Evidence Context

Web chat interface at openevidence.com. Clinicians paste clinical questions with patient context. No API — copy-paste workflow. They advise against entering PHI. This tool fills that gap at the clipboard level.