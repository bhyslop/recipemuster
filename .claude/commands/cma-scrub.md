---
description: Clean up mapping section formatting
argument-hint: [file-path | all]
model: haiku
---

You are cleaning up the mapping section of concept model documents.

**Configuration:**
- Lenses directory: lenses/
- Kit directory: Tools/cmk/
- Kit path: Tools/cmk/concept-model-kit.md

**Target:** $ARGUMENTS (use "all" for all .adoc files in lenses directory)

**File Resolution:**
When a filename is provided (not "all"):
1. If it's a full path that exists → use it
2. If it matches a file in lenses directory → use that
3. If it matches a file in kit directory → use that
4. Common aliases: "MCM" → mcm-MCM-MetaConceptModel.adoc, "AXL" → axl-AXLA-Lexicon.adoc

**Mapping Section Rules:**

1. **Column alignment**: The `<<` of each definition aligns to columns that are multiples of 10
   - Minimum column: 30 (for short attribute names)
   - Adjust all entries together when one requires more space

2. **Ordering**: Entries alphabetized by the replacement text (what appears in `<<anchor,This Text>>`)

3. **Category declarations**: Comment block at top declares all category prefixes used

4. **Variant consistency**: Related variants grouped together:
   ```
   :excm_term:                   <<excm_term,Term>>
   :excm_term_s:                 <<excm_term,Terms>>
   :excm_term_p:                 <<excm_term,Term's>>
   ```

5. **Section markers**: Preserve `// tag::mapping-section[]` and `// end::mapping-section[]`

**Process:**
1. Find the mapping section (between tag markers or at document start)
2. Parse all attribute definitions
3. Recalculate alignment column
4. Sort by replacement text
5. Show diff for approval
6. Write updated file after approval

**Error handling:** If mapping section not found, report and stop.
