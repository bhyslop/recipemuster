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

The HIPAA Privacy Rule provides two de-identification methods. This tool implements the **Safe Harbor method** (45 CFR 164.514(b)(2)), which requires removal of 18 specified identifier categories. Reference: [HHS De-Identification Guidance](https://www.hhs.gov/hipaa/for-professionals/privacy/special-topics/de-identification/index.html).

The following identifiers must be removed for Safe Harbor de-identification:

1. Names
2. Geographic data smaller than state (addresses, zip codes with <20k population)
3. Dates more specific than year (birth, admission, discharge, death); ages over 89 aggregated to single category (90+)
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

### Initial State

On launch, before any clipboard consumption, the app displays its name ("Ann's PHI Clipbuddy") and a brief instruction: "Copy a patient note from Epic, then switch to this window." The window title is "Ann's PHI Clipbuddy". The dock icon uses the same name.

### Clipboard Consumption

When the application receives window focus, it reads the system clipboard and evaluates whether the content is clinical. The app performs a lightweight heuristic check for Epic or medical content — presence of known clinical labels (`Patient:`, `DOB:`, `MRN:`, `Attending:`), medical terminology density, or Epic-characteristic HTML structure.

- **Clinical content detected**: The app consumes the clipboard, clears the system clipboard (writes empty string via clipboard API), and proceeds to triage display.
- **Non-clinical content detected**: The app clears any previous triage context and returns to the initial instruction state. No error dialog — the clinician simply copies the right content and refocuses.

**Change detection**: If the clinician switches away and refocuses, the app compares the current clipboard content against the previously consumed content. If unchanged, the existing triage state is preserved — the clinician can continue reviewing without disruption. If the clipboard contains new content, the app re-evaluates and consumes if clinical.

The app always clears the system clipboard after consumption. Clipboard content containing PHI should not linger on the system clipboard where other applications could access it.

The app expects HTML content from Epic's "Copy All" feature. Epic places rich content on the clipboard — both HTML and potentially RTF representations — preserving structural formatting (bold labels, section headers, tabular data). When pasted into applications like Microsoft Word, this formatting is fully preserved. This structural richness is foundational to the detection strategy: bold labels like `Patient:` and `Attending:` serve as high-precision anchors for PHI identification. If the clipboard contains only plain text, the app processes it with reduced structural context (Tier 2 label-anchored detection is weakened).

### Triage Display

The main window presents two coordinated views:

**Document Preview** (upper/primary area): The clinical note rendered as readable text with inline highlighting indicating PHI classification:

- **RED highlight** (solid border): Definite PHI — names in labeled positions, SSNs, MRNs, dates of birth, phone numbers, addresses, facility names. These are elided in the anonymized output.
- **YELLOW highlight** (dashed border): Questionable — words matching name/location dictionaries in narrative context, or collision words appearing on both blacklist and whitelist. Default to elide, but the clinician can override to pass.
- **GREY text** (dimmed): Safe content — medical terminology, lab values, medications, vitals, diagnoses, common English. Passes through unchanged.

**Findings Panel** (lower/secondary area): Two visually separated sections listing detected items. Each entry shows matched text, PHI category label (NAME, DATE, MRN, PHONE, ADDRESS, etc.), and a toggle control: [ELIDE] or [PASS].

- **Questionable Findings** (upper section, YELLOW items): Dictionary matches and ambiguous detections. Default ELIDE, easily toggled to PASS. This is the clinician's primary workspace — most review time is spent here.
- **Definite PHI** (lower section, RED items): Label-anchored names, regex-matched identifiers. Default ELIDE. Toggleable to PASS for expert override, but separated from the questionable section to create deliberate friction — overriding a RED item requires scrolling past all YELLOW items.

Toggling any finding updates the document preview in real time. Provider names and facility names are treated as PHI and appear in the Definite PHI section when label-anchored (e.g., following `Attending:` or `Facility:` labels). While HIPAA Safe Harbor technically covers patient identifiers, provider and facility names can be re-identifying in combination with clinical details — conservative treatment is appropriate.

### Anonymized Output

The "Copy Anonymized to Clipboard" button generates plain text from the current triage state:

- ELIDE items are replaced with category-specific placeholders: `[NAME]`, `[DATE]`, `[MRN]`, `[PHONE]`, `[ADDRESS]`, `[EMAIL]`, `[SSN]`, `[FACILITY]`, `[PROVIDER]`, `[ACCOUNT]`, `[ENCOUNTER_ID]`
- PASS items retain their original text
- Date of birth receives a special transform: `DOB: 03/15/1952` becomes `Age: 74` — preserving clinically useful age information while removing the identifying date. Per HIPAA Safe Harbor (45 CFR 164.514(b)(2)(i)(C)), ages over 89 are aggregated: a 92-year-old becomes `Age: 90+`
- Output is plain text with section structure (line breaks, headers) preserved for readability
- HTML formatting is stripped — this is itself a privacy measure, removing embedded metadata and hidden fields

The anonymized text is written to the system clipboard, ready to paste into Open Evidence.

### Clinical Content Preservation

The tool must be conservative: a false positive (flagging a safe word as PHI) is preferable to a false negative (letting PHI pass through). The following clinical content categories must survive anonymization intact:

- Laboratory values and reference ranges (Troponin, BMP, CBC, etc.)
- Vital signs (BP, HR, RR, Temp, SpO2)
- Medications and dosages (Metformin 1000mg BID, Lisinopril 20mg daily, etc.)
- Diagnoses and conditions (Hypertension, Type 2 Diabetes Mellitus, NSTEMI, etc.)
- Procedures (cholecystectomy, echo, etc.)
- Medical abbreviations (BID, ACS, S/P, WNL, etc.)
- Allergies and reactions (the allergen and reaction type, not the patient who has them)
- Clinical assessments and plan items

This preservation requirement drives the whitelist strategy in the detection engine — medical terminology, drug names, and clinical abbreviations are explicitly whitelisted to prevent false positives on terms that could also appear as proper nouns.

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
