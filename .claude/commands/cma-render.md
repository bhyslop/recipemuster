---
description: Transform concept model to ClaudeMark format
argument-hint: [source-file] [full | --terms t1,t2 | --section "Name"]
model: sonnet
---

You are transforming a concept model document to ClaudeMark format for LLM consumption.

**Configuration:**
- Lenses directory: lenses/
- Kit path: Tools/cmk/concept-model-kit.md

**Source:** $1
**Mode:** $2 (default: full)

**ClaudeMark Syntax:**
- Term definitions: `### Term Name «term-id»`
- Term references: `«term-id»`
- Prose flows naturally without AsciiDoc markup
- Code literals: Standard backticks preserved

**Transformation Rules:**

1. **Extract term definitions**: For each `[[anchor]]` with definition, create:
   ```markdown
   ### Display Text «anchor»
   Definition prose with «other-term» references.
   ```

2. **Convert references**: `{category_term}` becomes `«anchor»` (using the anchor from the attribute definition)

3. **Subsetting modes**:
   - `full`: Include all terms
   - `--terms t1,t2`: Include only listed terms (by anchor name)
   - `--section "Name"`: Include only terms from that section

4. **Strip annotations**: `// ⟦...⟧` comments do not appear in output (they inform transformation, not output)

5. **Preserve structure**: Section hierarchy maps to heading levels

**Output:** Write to `lenses/[source-basename]-claudemark.md`

**Validation before presenting:**
- Verify all term references resolve
- Check that semantic meaning is preserved
- Report any transformation issues

**Error handling:** If source not found, report and stop.
