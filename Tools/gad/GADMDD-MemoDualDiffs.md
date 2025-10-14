# GAD Memo: Dual Diffs (GADMDD)

## Summary

This memo documents insights from 2025-09-05 exploration of diff presentation approaches in GAD (Google AsciiDoc Differ). The core insight: side-by-side dual-pane diff viewing may be more robust and effective than single-pane markup insertion for human-AI collaboration workflows.

## Context

GAD system successfully generates HTML renderings from AsciiDoc across git commits and detects changes using wikEdDiff. However, presenting those changes back to humans for review proved problematic. Initial attempts focused on inserting wikEdDiff markup back into rendered HTML (mono-view approach), but this created complex parsing challenges when HTML elements were involved in the diff.

## Key Insights

### The Fundamental Mismatch
WikiEdDiff operates on text/markup level, treating HTML elements as part of text content. This creates complex nested markup like:
```html
<span class="wikEdDiffDelete">&lt;/a&gt;<span class="wikEdDiffSpace">...
```

When we need to insert these markers back into clean HTML DOM structure, we encounter a semantic impedance mismatch between wikEdDiff's text-centric view and the DOM's hierarchical structure.

### The Human-AI Collaboration Use Case
The real requirement is enabling biological intelligence to quickly verify that digital intelligence's rapid AsciiDoc changes produced intended semantic effects in the rendered output. Not just "what changed" but "what did the change mean in the rendered reality?"

This is pioneering work in human-AI technical writing collaboration interfaces.

### Dual-Pane Advantages
Side-by-side presentation offers several benefits over mono-view:
- **Robustness**: Works regardless of HTML complexity or element boundaries
- **Simplicity**: Avoids complex wikEdDiff markup parsing and DOM surgery  
- **Clarity**: Both "before" and "after" states visible simultaneously
- **Extensibility**: Can add synchronized scrolling, section highlighting, etc.

## Technical Approach

### Current State
GADF successfully generates distinct HTML files for different commits:
- Files stored in `/workspace/gad-working-dir/output/`
- Naming pattern: `main-{sha256hash}.html`
- GADI can fetch and compare these files

### Recommended Implementation
1. **Dual-pane viewer** in GADI with synchronized scrolling
2. **Basic change highlighting** at paragraph/section level
3. **Avoid complex markup surgery** that attempts to place wikEdDiff markers in rendered HTML
4. **Focus on semantic changes** rather than character-level diffs

### Alternative Approaches Considered
1. **Mono-view with markup insertion**: Parse wikEdDiff output and insert deletion/insertion markers into clean HTML DOM - complex and fragile
2. **Plain text mapping**: Extract plain text, diff it, map changes back to DOM - loses semantic HTML context
3. **Custom HTML-aware differ**: Build new diff algorithm that understands element boundaries - significant development effort

## Implications for GADS

This dual-diff approach should inform future GADS specifications:
- Consider dual-pane presentation as primary interface mode
- Design for human cognitive review patterns rather than technical diff completeness  
- Focus on semantic change detection over character-level precision
- Build for sustainable human-AI collaboration workflows

## Future Considerations

### Cognitive Interface Design
Different LLMs may need different structural cues for productive engagement. AsciiDoc patterns serve as cognitive architecture for human-AI collaboration, not just documentation.

### Trust Handles
The system provides "trust handles" - ways for human collaborators to maintain oversight and confidence when AI makes rapid bulk changes to complex technical documents.

### Iteration Requirements
Start with basic dual-pane implementation to establish the human-AI feedback loop, then iterate based on actual usage patterns rather than theoretical requirements.

## Status

This memo captures conceptual insights from 2025-09-05. Implementation work remains to build the dual-pane viewer and validate the approach through practical use.