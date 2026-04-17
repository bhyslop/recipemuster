## Character

Product design into prototype implementation. Starts with synthesis (specs capturing a rich design conversation), transitions to exploratory engineering (Tauri + Rust + cross-platform clipboard), then steady implementation of a three-tier PHI detection engine. Requires careful attention to minting discipline — this kit lives alongside RBK/JJK/BUK and must respect the prefix universe.

## Problem

Clinicians using Epic EHR need to consult Open Evidence (an AI diagnostic tool) but must not paste Protected Health Information (PHI) into it. HIPAA Safe Harbor defines 18 PHI categories that must be removed. Today, clinicians either skip the consultation or manually scrub text — both are unacceptable. No clipboard-level tool exists to automate the triage.

## Solution

Ann's PHI Clipbuddy (`apcap`) — a Tauri desktop app (macOS, later Windows) that:
1. Validates clipboard content as clinical via heuristic, clears system clipboard after consumption
2. Parses Epic "Copy All" HTML for structural context (labels, sections)
3. Runs a three-tier PHI detection engine (regex → label-anchored → dictionary)
4. Displays a triage view: RED (definite PHI), YELLOW (questionable), GREY (pass)
5. Lets clinician toggle findings — YELLOW easily, RED with deliberate friction (separate panel section)
6. Writes anonymized plain text to clipboard with category-specific placeholders (DOB→age, >89→"90+")
7. Self-updates from `/Users/Shared/apcua/` without confirmation

## Key Design Decisions

- **No JavaScript**: All rendering logic is Rust. The webview is a passive display surface. Decided during ₢A9AAP after the byte-vs-char offset mismatch bug (₢A9AAK) demonstrated the cost of a two-language boundary in a HIPAA-adjacent tool. Policy documented in APCPS scope boundaries, APCAS product vision, and apck-claude-context.md.
- **Anchored vs Matched**: Tier 2 (label position) and Tier 3 (dictionary identity) are independent mechanisms with distinct vocabulary — see APCPS
- **Case-insensitive**: All dictionary lookups normalize to lowercase; original case preserved in display
- **Per-word anchoring**: No multi-word merge — each word individually classified
- **Conservative bias**: False positive preferred over false negative
- **Provider/facility names**: Treated as PHI (re-identifying in combination with clinical details)
- **Clipboard always cleared** after consumption
- **Clinical content heuristic** gates processing; non-clinical clipboard shows diagnostic and clears triage

## Authoritative Documents

| Document | Owns | Path |
|----------|------|------|
| Application Spec (APCAS) | UX plan, workflow, deployment, HIPAA context | `Tools/apck/APCAS-Specification.md` |
| Prototype Spec (APCPS) | Detection pipeline, tech stack, dictionaries, project structure, scope boundaries | `Tools/apck/APCPS-PrototypeSpecification.md` |
| Claude Context | Build/run/deploy instructions, file mappings, prefix tree, tabtargets | `Tools/apck/apck-claude-context.md` |
| Rust Coding Guide (RCG) | Naming conventions, test patterns, disciplines | `Tools/vok/vov_veiled/RCG-RustCodingGuide.md` |