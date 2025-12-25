# Steeplechase: RBAGS Manual Procedure Specification Alignment

---
### 2025-12-25 - fix-cmk-normalizer-and-normalize-rbags - APPROACH
**Mode**: manual
**Proposed approach**:
- Add explicit exception to Phase 1 rule #2 for section headers (lines starting with `=`)
- Clarify that term isolation applies to prose paragraphs and definition entries, not structural elements
- Run `/cma-normalize` on RBAGS and verify section headers remain intact
- Validate the normalized output has no broken headers
---
