Here's a research query focused on the core problem:

**Research Query: HTML Structure-Aware Diff Algorithms and Implementations**

"I need to find algorithms and open source implementations for comparing two versions of HTML documents that preserve structural semantics while presenting changes in a human-readable format.

The core challenge is displaying differences between HTML documents where:

1. **Structure preservation is critical**: The diff must understand HTML's hierarchical DOM structure, not treat it as flat text. Changes should be tracked at the element/block level, not just character level.

2. **Semantic equivalence must be recognized**: Formatting changes that don't affect meaning (whitespace normalization, attribute reordering, word wrapping differences) should not appear as changes.

3. **The output format should be a unified view**: Rather than side-by-side comparison, the ideal output is the new document structure annotated with contextual information about deletions, insertions, and modifications at the block level.

4. **Block identity must be maintained**: When content moves between structural elements or is deleted, the algorithm must track which original blocks/elements the changes came from.

I'm particularly interested in:
- Algorithms that perform tree-based or DOM-aware diffing (not linear text diffing)
- Solutions that handle the challenge of content that spans multiple HTML elements
- Implementations that can produce a single annotated HTML output showing the new version with embedded change information
- Research papers or specifications addressing HTML document comparison beyond simple text diff
- How modern document processors (like those in Google Docs or Microsoft Word's web version) handle HTML-based document comparison

Keywords: HTML tree diff, DOM diff algorithm, structural document comparison, semantic HTML diff, XML tree matching, document versioning visualization, hierarchical diff algorithms"

This query avoids implementation-specific details while capturing the essential problem of structure-aware HTML comparison.