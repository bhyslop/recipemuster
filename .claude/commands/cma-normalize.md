---
description: Apply whitespace normalization to concept model documents
argument-hint: [file-path | all]
model: haiku
---

You are applying MCM whitespace normalization (ancestry enhancement) to concept model documents.

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

**Model Enforcement:** This task is designed for haiku. If this prompt was expanded into a larger model's context (Opus/Sonnet), use the Task tool with `model: "haiku"` to delegate this work. Do not execute directly - spawn the agent and relay results.

**CRITICAL CONSTRAINT - Whitespace Only:**
This operation adjusts ONLY line breaks and blank lines. You must NOT change any words, punctuation, or sentence structure. The document content must be identical before and after - only the placement of newlines changes.

**DO NOT:**
- Reword or rephrase any text
- Split compound sentences into multiple sentences
- Join sentences together
- Add new text or delete existing text
- Change punctuation
- "Improve" clarity or readability through rewording
- Fix grammar or spelling (report issues but do not fix)

**Whitespace Rules to Apply:**

1. **One sentence per line**: Break at EXISTING sentence boundaries (periods, question marks, exclamation points followed by space and capital letter). Do not create new sentences by restructuring.

2. **Linked terms isolated**: When a `{term_reference}` appears standalone in prose:
   - Line break before the term
   - Line break after the term
   - Example - BEFORE:
     ```
     The system uses {excm_processor} to handle requests.
     ```
   - Example - AFTER:
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

**Edit Tool Warning:** When using the Edit tool, `{term}` references must remain as single braces. Do NOT escape or double them. The Edit tool takes literal strings - write `{mcm_term}` not `{{mcm_term}}`.

**Process:**
1. Read the target file(s)
2. Apply rules systematically, section by section
3. Show diff of changes for approval - diff should show ONLY line break changes, no word changes
4. Write updated file after approval

**Self-check before presenting diff:** Verify that removing all newlines from both versions produces identical text. If not, you have made unauthorized content changes - revert and try again.

**Error handling:** If file not found or not .adoc, report and stop.
