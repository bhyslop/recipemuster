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

---
### 2025-12-26 05:49 - extract-prototype-sequences-dc-and-tb - DELEGATE
**Spec**:
- Objective: Extract DC and TB sequence bodies to includable files
- Scope: 2 files created, RBAGS modified
- Success: Files contain complete sequences, includes render correctly
- On failure: Stop and report

**Execution trace**:
- Read RBAGS lines 355-520 (depot_create body)
- Read RBAGS lines 912-1113 (trigger_build body)
- Created `lenses/rbw-RBSDC-depot_create.adoc` (162 lines, 15 steps)
- Created `lenses/rbw-RBSTB-trigger_build.adoc` (198 lines, 12 steps)
- Modified RBAGS: replaced depot_create body with terse def + include
- Modified RBAGS: replaced trigger_build body with terse def + include

**Result**: success
Haiku extracted both sequences correctly. Opus reviewed and confirmed nothing lost.

**Modified files**:
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSDC-depot_create.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBSTB-trigger_build.adoc
- /Users/bhyslop/projects/brm_recipebottle/lenses/rbw-RBAGS-AdminGoogleSpec.adoc
---

---
### 2025-12-26 05:49 - review-prototype-extraction - WRAP
**Mode**: manual
**Outcome**: Human reviewed extracted files, verified AsciiDoc rendering, confirmed approach for remaining 11 sequences.
---
