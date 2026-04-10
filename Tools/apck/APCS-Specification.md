# Ann's PHI Clipbuddy — Product Specification

## Motivation

Clinicians using Epic EHR frequently need to consult external AI tools — particularly Open Evidence, a medical AI assistant for diagnostic evaluation and literature synthesis. Open Evidence is a web chat interface where clinicians paste clinical questions enriched with patient context: lab values, history, symptoms, medications.

The problem: this patient context is Protected Health Information (PHI) under HIPAA. Open Evidence explicitly advises users not to enter PHI. The HIPAA Safe Harbor method defines 18 categories of identifiers that must be removed for de-identification. Today, clinicians either skip the AI consultation entirely (suboptimal care) or manually scrub text before pasting (error-prone, time-consuming, and unreliable under clinical time pressure).

No clipboard-level tool exists to automate this triage.

## Product Vision

Ann's PHI Clipbuddy is a desktop companion application that sits between Epic and Open Evidence, intercepting clipboard content and automating PHI detection, triage, and anonymization.

The clinician's workflow becomes:

1. In Epic, use "Copy All" to copy the patient note
2. Switch focus to PHI Clipbuddy — the app consumes the clipboard automatically
3. Review the triage display — PHI is highlighted, clinical content passes through
4. Adjust any questionable items if needed
5. Click "Copy Anonymized to Clipboard"
6. Paste into Open Evidence — clean, de-identified, clinically useful text

## Target Platforms

- macOS (primary, first implementation)
- Windows (planned, architecture supports it)

## HIPAA Safe Harbor — The 18 Categories

The following identifiers must be removed for Safe Harbor de-identification:

1. Names
2. Geographic data smaller than state (addresses, zip codes with <20k population)
3. Dates more specific than year (birth, admission, discharge, death; age >89)
4. Phone numbers
5. Fax numbers
6. Email addresses
7. Social Security numbers
8. Medical record numbers
9. Health plan beneficiary numbers
10. Account numbers
11. Certificate/license numbers
12. Vehicle identifiers
13. Device identifiers/serial numbers
14. Web URLs
15. IP addresses
16. Biometric identifiers
17. Full-face photographs (not applicable for text)
18. Any other unique identifying number/code

## User Experience

### Clipboard Consumption

When the application receives window focus, it automatically reads the system clipboard. The app expects HTML content from Epic's "Copy All" feature, which preserves structural formatting (labels, sections, headers). If the clipboard contains plain text, the app processes it with reduced structural context.

The clipboard is not cleared after consumption in the current design. This may be revisited — the specification reserves the option to clear the clipboard as a safety measure in future versions.

### Triage Display

The main window presents two coordinated views:

**Document Preview** (upper/primary area): The clinical note rendered as readable text with inline highlighting indicating PHI classification:

- **RED highlight** (solid border): Definite PHI — names in labeled positions, SSNs, MRNs, dates of birth, phone numbers, addresses, facility names. These are elided in the anonymized output.
- **YELLOW highlight** (dashed border): Questionable — words matching name/location dictionaries in narrative context, or collision words appearing on both blacklist and whitelist. Default to elide, but the clinician can override to pass.
- **GREY text** (dimmed): Safe content — medical terminology, lab values, medications, vitals, diagnoses, common English. Passes through unchanged.

**Findings Panel** (lower/secondary area): A scrollable list of every detected item, showing:

- Matched text
- PHI category label (NAME, DATE, MRN, PHONE, ADDRESS, etc.)
- Classification (RED or YELLOW)
- Toggle control: [ELIDE] or [PASS]

RED items default to ELIDE and cannot be toggled to PASS — they are definite PHI. YELLOW items default to ELIDE but can be toggled to PASS by the clinician. Toggling a finding updates the document preview in real time.

### Anonymized Output

The "Copy Anonymized to Clipboard" button generates plain text from the current triage state:

- ELIDE items are replaced with category-specific placeholders: `[NAME]`, `[DATE]`, `[MRN]`, `[PHONE]`, `[ADDRESS]`, `[EMAIL]`, `[SSN]`, `[FACILITY]`, `[PROVIDER]`, `[ACCOUNT]`, `[ENCOUNTER_ID]`
- PASS items retain their original text
- Date of birth receives a special transform: `DOB: 03/15/1952` becomes `Age: 74` — preserving clinically useful age information while removing the identifying date
- Output is plain text with section structure (line breaks, headers) preserved for readability
- HTML formatting is stripped — this is itself a privacy measure, removing embedded metadata and hidden fields

The anonymized text is written to the system clipboard, ready to paste into Open Evidence.

### Self-Update

The application monitors a designated staging directory for new versions. When a new application bundle appears in the staging directory, the app automatically:

1. Copies the new version over itself
2. Launches the new version
3. Exits the current process

This happens without user confirmation — the update is silent and automatic. The staging directory is `/Users/Shared/apcua/` (globally readable, writable by the deployer).

## Deployment Model

The application is built on the developer's machine and delivered to the clinician's machine via `scp` to the staging directory. The clinician launches the app independently. Updates follow the same path — build, scp, auto-update triggers on the clinician's machine.

No app store, no installer, no signing infrastructure in the current model. The clinician's machine is `anns-macbook-air`, accessible via SSH as `bhyslop`.

## Detection Pipeline

*This section is deliberately left blank in the product specification. The detection pipeline is an engineering concern documented in the living prototype specification ([APCPS-PrototypeSpecification.md](APCPS-PrototypeSpecification.md)). The product spec defines WHAT the user sees (red/yellow/grey triage); the prototype spec defines HOW the engine classifies.*

*As the prototype matures, the pipeline will evolve rapidly. Maintaining parallel pipeline documentation in both specs would create drift without carrying weight.*

## Redistribution Considerations

**No concerns for internal use.** All engine dependencies are MIT/Apache licensed pure Rust crates. Tauri is MIT. Dictionary data from US Census and SSA is public domain.

**For future distribution:** Medical terminology databases (SNOMED-CT, RxNorm, LOINC) provide richer whitelist coverage but require NLM licensing for redistribution. The current approach — hand-curated medical whitelist — avoids this constraint. Evaluate NLM terms when the product matures toward external distribution.

## Open Evidence Context

Open Evidence is a medical AI tool developed by a team associated with Bocconi University. It provides a web chat interface at openevidence.com where clinicians type or paste clinical questions and receive synthesized answers with citations to medical literature. It is a question-answering system consuming free-text natural language — not structured clinical data formats. The typical workflow is copy-paste from clinical notes into the chat interface.

Open Evidence cautions users against entering PHI. This tool exists to make that guidance practical rather than aspirational.
