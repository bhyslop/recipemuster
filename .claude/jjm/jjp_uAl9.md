## Character

Product design into prototype implementation. Starts with synthesis (specs capturing a rich design conversation), transitions to exploratory engineering (Tauri + Rust + cross-platform clipboard), then steady implementation of a PHI detection engine. Restructured in ₢A9AAU from a three-tier Rust-only design into a ten-discerner two-placement architecture (7 Rust-app + 3 container via bind-mount JSON); container scaffolding shipped in ₢A9AAZ with empirical validation on two Epic fixtures. Requires careful attention to minting discipline — this kit lives alongside RBK/JJK/BUK and must respect the prefix universe.

## Problem

Clinicians using Epic EHR need to consult Open Evidence (an AI diagnostic tool) but must not paste Protected Health Information (PHI) into it. HIPAA Safe Harbor defines 18 PHI categories that must be removed. Today, clinicians either skip the consultation or manually scrub text — both are unacceptable. No clipboard-level tool exists to automate the triage.

## Solution

Ann's PHI Clipbuddy (`apcap`) — a Tauri desktop app (macOS, later Windows) that:

1. Validates clipboard content as clinical via heuristic, clears system clipboard after consumption
2. Parses Epic "Copy All" HTML for structural context (labels, sections)
3. Runs ten discerners across two placements: 7 Rust (regex, label, surname, firstname, city, english, medical) + 3 container (Stanford, spaCy, Stanza) via bind-mount JSON
4. Combining (forthcoming, ₢A9AAV) fuses discerner outputs into `apcs_unified_finding`
5. Displays a triage view: RED (definite PHI), YELLOW (questionable), GREY (pass)
6. Lets clinician toggle findings
7. Writes anonymized plain text to clipboard with category-specific placeholders (DOB→age, >89→"90+")
8. Self-updates from `/Users/Shared/apcua/` (dormant for prototype — manual quit + relaunch)

## Key Design Decisions

- **No JavaScript**: All rendering logic is Rust. The webview is a passive display surface. Decided during ₢A9AAP.
- **Ten discerners, two placements** (₢A9AAU): Rust-app discerners for fast/deterministic work; container for heavier NLP models; combining stays in Rust (strongly typed).
- **Single-tenant wire**: never more than one clipboard transaction in flight; drop-old handling is defensive.
- **Combining deferred to empirical evidence** (₢A9AAV): design conversation builds on ₢A9AAZ's fixture assay, not a priori.
- **Case-insensitive** dictionary lookups; original case preserved in display.
- **Conservative bias**: false positive preferred over false negative.
- **Provider/facility names** treated as PHI (re-identifying in combination with clinical details).
- **Clipboard always cleared** after consumption.
- **Clinical content heuristic** gates processing.
- **Self-update watcher dormant for prototype** (₢A9AAS): manual quit + relaunch via `tt/apcw-D.Deploy.sh` emitting a forward-to-Ann block.

## Authoritative Documents

| Document | Owns | Path |
|----------|------|------|
| Application Spec (APCAS) | UX plan, workflow, deployment, HIPAA context | `Tools/apck/APCAS-Specification.md` |
| Discerner Spec (APCS0) | Ten-discerner vocabulary, combining (forthcoming) | `Tools/apck/APCS0-SpecTop.adoc` |
| Prototype Spec (APCPS) | Detection pipeline, tech stack, container architecture, wire format, project structure | `Tools/apck/APCPS-PrototypeSpecification.md` |
| Claude Context | Build/run/deploy instructions, file mappings, prefix tree, tabtargets | `Tools/apck/apck-claude-context.md` |
| Rust Coding Guide (RCG) | Naming conventions, test patterns, disciplines | `Tools/vok/vov_veiled/RCG-RustCodingGuide.md` |

## Reference data for combining

`Memos/memo-20260420-apck-container-fixture-assay/` (committed under ₢A9AAZ):

- `epic_initial.json` (382 KB), `epic_geriatric.json` (361 KB) — unedited container outputs on the two Epic fixtures.
- `memo.md` — complementarity analysis with concrete examples.

Observations to carry into ₢A9AAV:

- Stanford strong on narrative PHI; weak on structured identifiers (HOSPITAL conflates address+facility; encounter IDs fragment; email/URL surgery).
- Stanza PERSON corroborates Stanford independently; GPE/FAC triangulate facility/locality Stanford conflates.
- spaCy `en_core_sci_md` emits single generic ENTITY — entity findings weight-zero; use spaCy for syntactic features only.
- Regex wins for structured identifiers at the same span (SSN, encounter IDs, email, URL).
- Stanford ≥0.95 is the high-signal confidence band.
- ~370 KB JSON per normalized page; steady-state sub-1s per fixture.