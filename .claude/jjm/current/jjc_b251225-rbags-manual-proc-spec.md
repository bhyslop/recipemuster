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

---
### 2025-12-25 21:15 - fix-cmk-normalizer-and-normalize-rbags - WRAP
**Mode**: manual
**Outcome**: Fixed normalizer backtick rules, normalized RBAGS; created itch for Rust replacement due to LLM unreliability
---

---
### 2025-12-26 05:37 - extract-prototype-sequences-dc-and-tb - APPROACH
**Mode**: delegated
**Proposed approach**:
- Extract rbtgo_depot_create body (lines ~355-483) to `lenses/rbw-RBSDC-depot_create.adoc`
- Extract rbtgo_trigger_build body (lines ~814-968) to `lenses/rbw-RBSTB-trigger_build.adoc`
- In RBAGS, replace full sequence with: anchor + terse 1-2 line definition + AsciiDoc include directive
- Test that cross-references still resolve
- Verify full document renders correctly with includes
---
