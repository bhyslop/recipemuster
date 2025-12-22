---
description: Apply whitespace normalization to concept model documents
argument-hint: [file-path | all]
model: haiku
---

You are applying MCM whitespace normalization (ancestry enhancement) to concept model documents.

**Configuration:**
- Lenses directory: lenses/
- Kit path: ../cnmp_CellNodeMessagePrototype/tools/cmk/concept-model-kit.md

**Target:** $ARGUMENTS (use "all" for all .adoc files in lenses directory)

**Whitespace Rules to Apply:**

1. **One sentence per line**: Each sentence ends at a line break. Do not join sentences.

2. **Linked terms isolated**: When a `{term_reference}` appears standalone in prose:
   - Line break before the term
   - Line break after the term
   - Example:
     ```
     The system uses
     {excm_processor}
     to handle requests.
     ```

3. **Term sequences together**: Multiple terms in sequence stay on one line:
   - Line break before the first term
   - Line break after the last term
   - Terms separated by commas/and stay together
   - Example:
     ```
     The interface supports
     {excm_drag}, {excm_drop}, and {excm_scroll}
     operations.
     ```

4. **Blank lines between paragraphs only**:
   - One blank line between paragraphs
   - No blank lines within paragraphs
   - No blank lines around list items (except before bulleted lists in Task Lens sections)
   - No blank lines around code blocks

5. **Preserve code blocks**: Content inside `----` fences is opaque. Do not modify.

6. **Punctuation stays attached**: Periods, commas stay with their text, not on separate lines.

**Process:**
1. Read the target file(s)
2. Apply rules systematically, section by section
3. Show diff of changes for approval
4. Write updated file after approval

**Error handling:** If file not found or not .adoc, report and stop.
