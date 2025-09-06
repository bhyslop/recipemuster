# GAD-GADMWM: Memo - Wicked Misunderstand

## Summary

This memo documents a fundamental misunderstanding about how WikEdDiff works that caused significant thrashing in the GADS implementation. The mental model of "render new HTML with old content memories" does not align with WikEdDiff's actual operation as a linear text differ.

## The Misunderstanding

### What Was Expected (Mental Model)
- Take old HTML structure and new HTML structure
- Render the **new HTML** as the base document structure  
- Insert "memories" of **old content** as contextual red deletion blocks
- Highlight **new content** as green insertion blocks
- Result: New document with structured "ghosts" of removed content

### What WikEdDiff Actually Does
WikEdDiff is a **linear text diffing algorithm** designed for wiki markup, not HTML structure:

1. **Input**: Two linearized text strings (HTML structure is flattened)
2. **Processing**: Text-level diff analysis, not structure-aware
3. **Output**: Single linear text with inline `<span>` markup for additions/deletions
4. **Design**: Optimized for wiki text editing, not HTML block preservation

## The Technical Disconnect

When GADS linearizes HTML blocks for WikEdDiff:
```
Block 1: <div>Old content here</div>
Block 2: <p>More old content</p>
```
Becomes: `Old content hereMore old content`

WikEdDiff then produces deletions like:
```
<span class="wikEdDiffDelete">enabling controlled external access</span>
```

This deletion text **spans across the original HTML blocks** but has **no block identity**. When GADS tries to map these back to `deleteOp.blockId`, there is none - the deletion exists in the linearized text space, not the HTML block space.

## Current Symptoms

The GADI inspector logs show the issue clearly:
```
[INSPECTOR-TRACE] [DEBUG] Placing 24 deletion blocks
[INSPECTOR-TRACE] [DEBUG] Skipping deletion 0: no block ID
[INSPECTOR-TRACE] [DEBUG] Skipping deletion 1: no block ID
...all 24 deletions skipped
```

Result: Everything renders as green insertions because deletions can't be placed back into the HTML structure.

## Potential Solutions

### Option 1: Accept WikEdDiff's Linear Nature
- Render WikEdDiff output directly as HTML
- Accept inline deletions/insertions rather than block-level
- Simpler but loses structured "memory" vision

### Option 2: Build HTML-Structure-Aware Differ  
- Create custom algorithm that diffs HTML DOM trees
- Preserve block structure and relationships
- Map changes at the block/element level
- More complex but achieves the original vision

### Option 3: Hybrid Mapping Approach
- Use WikEdDiff for text analysis quality
- Post-process to map linear changes back to source blocks
- Approximate block assignment for deletion spans
- Moderate complexity, partial vision achievement

## HTML-Aware Differ Concept

A true HTML structure-aware differ would:

1. **Parse**: Build DOM trees for old and new HTML
2. **Align**: Match corresponding blocks/elements across versions
3. **Classify**: Identify unchanged, modified, added, and removed blocks
4. **Render**: Generate new structure with old content "memories"

Example output concept:
```html
<!-- New base structure -->
<div class="current-block">New content here</div>

<!-- Contextual memory of removed block -->
<div class="deleted-memory" data-was-after="block-id-123">
  <div class="deletion-content">Old content that was removed</div>
</div>

<!-- Modified block showing changes -->
<div class="modified-block">
  <span class="insertion">New text</span> and 
  <span class="deletion">old text</span> mixed
</div>
```

This would achieve the "new document with old memories" vision but requires significant custom development.

## Decision Point

The current WikEdDiff integration cannot deliver the intended user experience. A decision is needed on whether to:
- Pivot to WikEdDiff's linear model
- Invest in custom HTML-aware diffing
- Explore hybrid approaches

The "vibe coding" phase has revealed this fundamental architectural choice that was obscured by WikEdDiff's initial promise.