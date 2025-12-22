---
description: Validate concept model links and annotations
argument-hint: [file-path | all]
---

You are validating the structural integrity of concept model documents.

**Configuration:**
- Lenses directory: lenses/
- Kit path: Tools/cmk/concept-model-kit.md

**Target:** $ARGUMENTS (use "all" for all .adoc files in lenses directory)

**Validation Checks:**

1. **Reference completeness**:
   - Every `{term}` used in prose has a `:term:` definition in mapping section
   - Report: "Reference to undefined term: {missing_term}"

2. **Definition completeness**:
   - Every `:term:` definition has a `[[anchor]]` target somewhere in document
   - Report: "Definition without anchor: :term:"

3. **Anchor orphans**:
   - Every `[[anchor]]` has at least one reference via `{term}` or `<<anchor,...>>`
   - Report: "Orphaned anchor: [[unused_anchor]]" (warning, not error)

4. **Annotation format** (if AXL context detected):
   - Strachey brackets properly formed: `// ⟦...⟧`
   - Content is space-separated terms
   - First term is relationship (e.g., `axl_voices`)
   - Report: "Malformed annotation at line N"

**Output format:**
```
Validating: filename.adoc
- Errors: N
  - Reference to undefined term: {foo}
  - Definition without anchor: :bar:
- Warnings: M
  - Orphaned anchor: [[baz]]
- Annotations: P checked, Q issues
```

**Summary:** Report total errors/warnings across all files.

**Error handling:** Continue validation even if some files have issues.
