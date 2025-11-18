# Dual View Implementation Specification

## Overview
Implement `{gadi_dual_view}` as specified in GADS sections "Dual View Architecture" and "Definitions". Three subagents will work on distinct files with clear contracts between them.

## Data Structures & Contracts

### gadis_change Object (Core Data Structure)
```javascript
{
  changeId: number,                          // 0, 1, 2, ... (unique index)
  changeType: 'reciprocal' | 'monocular',    // From GADS definition
  forwardOps: [operation, ...],              // From gadis_forward_list
  reverseOps: [operation, ...],              // From gadis_reverse_list
  colorId: number,                           // 0-15, maps to CSS class gad-change-bg-{N}
  colorHex: string                           // Computed color hex for inline use
}
```

### Change Correlation Algorithm (Core Logic)
Location: `gadie_engine.js`

**Function**: `gadie_correlate_changes(forwardList, reverseList) -> changeList`

**Algorithm** (per GADS section "Inspector Change Correlation"):
1. Extract text content from each operation
2. For each forward operation (action: addElement, addTextElement, modifyTextElement):
   - Search reverseList for operation with matching text content
   - If found: create reciprocal_change, mark both as paired, increment reciprocal count
   - If not found: continue
3. For each unpaired reverse operation:
   - Create monocular_change with single reverse op
4. For each unpaired forward operation:
   - Create monocular_change with single forward op
5. Assign colorIds: reciprocal changes get 0-7, monocular changes get 8-15

**Output**: Array of gadis_change objects, sorted by first operation index

---

## Subagent 1: GADIE Engine (gadie_engine.js)

### Scope
Implement complete change correlation + dual view rendering pipeline. Replaces placeholder dualHTML (lines 96-107) with actual implementation.

### Functions to Implement

#### 1. gadie_correlate_changes(forwardList, reverseList)
- Input: Two operation arrays from diff-dom
- Output: Array of gadis_change objects
- Implements text matching algorithm above
- Uses existing: `gadie_escape_html`, text extraction helpers

#### 2. gadie_render_dual_left(fromDOM, changeList)
- Input: Original fromDOM, changeList with color assignments
- Output: HTML string (leftRendered)
- Process:
  - Clone fromDOM
  - For each gadis_change with reverseOps (deletions):
    - Find element by operation.route
    - Add class 'gads-dual-deleted' (will add strikethrough in CSS)
    - Add inline style: `background-color: {changeList[i].colorHex}`
  - Return cloned DOM as innerHTML

#### 3. gadie_render_dual_right(toDOM, changeList)
- Input: Original toDOM, changeList with color assignments
- Output: HTML string (rightRendered)
- Process:
  - Clone toDOM
  - For each gadis_change with forwardOps (additions):
    - Find element by operation.route
    - Add class 'gads-dual-added' (will add bold in CSS)
    - Add inline style: `background-color: {changeList[i].colorHex}`
  - Return cloned DOM as innerHTML

#### 4. gadie_render_change_entries(changeList, operations)
- Input: changeList array, operations array for route access
- Output: HTML string (changePaneHTML)
- For each gadis_change, create:
  ```html
  <div class="gad-change-entry" data-change-id="{changeId}" style="background-color: {colorHex}">
    <div class="gad-change-label">Change #{changeId+1} ({changeType})</div>
    <div class="gad-change-buttons">
      {for each op in forwardOps:}
      <button class="gad-operation-button gad-op-left"
              data-change-id="{changeId}"
              data-operation-index="{opIndex}"
              data-pane="left"
              data-route="{JSON.stringify(op.route)}">
        Left Op
      </button>
      {for each op in reverseOps:}
      <button class="gad-operation-button gad-op-right"
              data-change-id="{changeId}"
              data-operation-index="{opIndex}"
              data-pane="right"
              data-route="{JSON.stringify(op.route)}">
        Right Op
      </button>
    </div>
  </div>
  ```
- Entries ordered by reverseOps sequence (per GADS: "always displaying the target/improved specification state")

#### 5. gadie_build_dual_view_html(leftRendered, rightRendered, changePaneHTML)
- Input: Three HTML strings from above
- Output: Complete dualHTML string with structure:
  ```html
  <div class="gad-dual-view">
    <div class="gad-dual-panes">
      <div class="gad-left-pane" id="dualLeftPane">
        {leftRendered}
      </div>
      <div class="gad-right-pane" id="dualRightPane">
        {rightRendered}
      </div>
    </div>
    <div class="gad-changes-pane" id="dualChangesPane">
      {changePaneHTML}
    </div>
  </div>
  ```

### Integration Point in gadie_diff()
Replace lines 96-107 with:
```javascript
// Dual view computation
const changeList = gadie_correlate_changes(operations, reverseOps);
const leftRendered = gadie_render_dual_left(fromDOM, changeList);
const rightRendered = gadie_render_dual_right(toDOM, changeList);
const changePaneHTML = gadie_render_change_entries(changeList, operations);
const dualHTML = gadie_build_dual_view_html(leftRendered, rightRendered, changePaneHTML);
```

### Contracts
- **Input**: forwardList, reverseList (from diff-dom)
- **Output**: changeList (array of gadis_change objects)
- **Color IDs**: 0-15, maps directly to CSS classes (see GADIC)
- **Routes**: Pass through unchanged from operations
- **HTML Safety**: Use `gadie_escape_html()` for any user-generated content

---

## Subagent 2: GADIC Styles (gadic_cascade.css)

### Scope
Add layout CSS for three-pane dual view + color palette + styling rules

### New CSS Classes to Add

#### Layout & Structure
```css
.gad-dual-view {
  /* Container for entire dual view */
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
}

.gad-dual-panes {
  /* Side-by-side panes container */
  display: grid;
  grid-template-columns: 1fr 1fr;  /* 50-50 split */
  gap: 1px;
  flex: 1;
  overflow: hidden;
  border-bottom: 1px solid #ccc;
}

.gad-left-pane, .gad-right-pane {
  /* Individual panes with independent scroll */
  overflow-y: auto;
  overflow-x: auto;
  padding: 12px;
  font-family: 'Georgia', serif;
  line-height: 1.6;
  background-color: #fafafa;
}

.gad-right-pane {
  border-left: 1px solid #e0e0e0;
  background-color: #ffffff;
}

.gad-changes-pane {
  /* Bottom pane with operation buttons */
  flex: 0 0 200px;  /* Fixed height, scrollable */
  overflow-y: auto;
  border-top: 1px solid #ccc;
  padding: 12px;
  background-color: #f5f5f5;
}
```

#### Change Entry Styling
```css
.gad-change-entry {
  margin-bottom: 12px;
  padding: 8px 12px;
  border-radius: 4px;
  border-left: 4px solid rgba(0,0,0,0.2);
}

.gad-change-label {
  font-weight: bold;
  font-size: 0.9em;
  margin-bottom: 6px;
  color: #333;
}

.gad-change-buttons {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}
```

#### Operation Button Styling
```css
.gad-operation-button {
  padding: 4px 8px;
  font-size: 0.85em;
  border: 1px solid rgba(0,0,0,0.2);
  background-color: rgba(255,255,255,0.8);
  border-radius: 3px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.gad-operation-button:hover {
  background-color: #fff;
  border-color: #000;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.gad-op-left::before {
  content: "← ";
}

.gad-op-right::before {
  content: "→ ";
}
```

#### Diff Styling (Dual View Specific)
```css
.gads-dual-deleted {
  text-decoration: line-through;
  opacity: 0.7;
}

.gads-dual-added {
  font-weight: bold;
}
```

#### Color Palette (16 colors: 8 reciprocal + 8 monocular)
```css
/* Reciprocal changes (colors 0-7) - warmer palette */
.gad-change-bg-0 { background-color: rgba(255, 107, 107, 0.2) !important; }
.gad-change-bg-1 { background-color: rgba(255, 193, 7, 0.2) !important; }
.gad-change-bg-2 { background-color: rgba(76, 175, 80, 0.2) !important; }
.gad-change-bg-3 { background-color: rgba(33, 150, 243, 0.2) !important; }
.gad-change-bg-4 { background-color: rgba(156, 39, 176, 0.2) !important; }
.gad-change-bg-5 { background-color: rgba(255, 152, 0, 0.2) !important; }
.gad-change-bg-6 { background-color: rgba(0, 150, 136, 0.2) !important; }
.gad-change-bg-7 { background-color: rgba(233, 30, 99, 0.2) !important; }

/* Monocular changes (colors 8-15) - greyed/muted palette */
.gad-change-bg-8 { background-color: rgba(117, 117, 117, 0.15) !important; }
.gad-change-bg-9 { background-color: rgba(158, 158, 158, 0.15) !important; }
.gad-change-bg-10 { background-color: rgba(189, 189, 189, 0.15) !important; }
.gad-change-bg-11 { background-color: rgba(224, 224, 224, 0.15) !important; }
.gad-change-bg-12 { background-color: rgba(207, 216, 220, 0.15) !important; }
.gad-change-bg-13 { background-color: rgba(244, 208, 63, 0.15) !important; }
.gad-change-bg-14 { background-color: rgba(129, 199, 132, 0.15) !important; }
.gad-change-bg-15 { background-color: rgba(144, 202, 249, 0.15) !important; }
```

### Contracts
- **Class Names**: Match exactly what GADIE generates
- **Color IDs**: Maps changeId (0-15) to `.gad-change-bg-{N}` class
- **Pane IDs**: `#dualLeftPane`, `#dualRightPane`, `#dualChangesPane` (match GADIE HTML)
- **Styling**: Does NOT override structure, only styling

---

## Subagent 3: GADIU Navigation (gadiu_user.js)

### Scope
Wire up operation button clicks to scroll and highlight behavior. Add minimal new code.

### New Methods to Add

#### 1. setupDualViewNavigation()
- Called after dual view HTML is injected into #tabContent
- Attach click listeners to all `.gad-operation-button` elements
- Store references to pane elements for reuse:
  ```javascript
  this.dualLeftPane = document.getElementById('dualLeftPane');
  this.dualRightPane = document.getElementById('dualRightPane');
  ```

#### 2. handleOperationButtonClick(e)
- Event handler for operation button clicks
- Extract from button.dataset:
  - `data-change-id`
  - `data-operation-index`
  - `data-pane` ('left' or 'right')
  - `data-route` (JSON array string)
- Call: `scrollPaneToRoute(pane, route)`
- Optional: highlight element briefly

#### 3. scrollPaneToRoute(paneElement, routeArray)
- Input: paneElement (DOM element), routeArray (e.g., [0, 2, 5])
- Find element within paneElement using route
- Scroll element into view: `element.scrollIntoView({ behavior: 'smooth', block: 'center' })`
- Optional: add temporary highlight class

### Integration Point in gadiu_user.js
In `performDiff()` method, after injecting dualHTML into `#tabContent`:
```javascript
if (this.currentTab === 'dual') {
  this.setupDualViewNavigation();
}
```

And when switching to dual tab:
```javascript
switchTab(tabName) {
  // ... existing code ...
  if (tabName === 'dual') {
    this.setupDualViewNavigation();
  }
}
```

### Contracts
- **Route Format**: Array of integers (from diff-dom operations)
- **Pane IDs**: Match GADIE HTML structure
- **Button Attributes**: Match GADIE HTML data attributes exactly
- **No State**: Handlers are stateless, rely on DOM attributes

---

## Testing Strategy

### Manual Testing
1. Load inspector, select two commits with changes
2. Switch to Dual View tab
3. Verify:
   - Left pane shows from document with strikethrough
   - Right pane shows to document with bold
   - Change entries list appears below
   - Each entry has correct background color
   - Colors match across all three panes
4. Click operation buttons:
   - Should scroll to correct element
   - Element should briefly highlight
   - Should be able to see operation location in context

### Debug Artifacts
GADIE will ship debug artifacts for:
- `dual-left-rendered` (HTML string)
- `dual-right-rendered` (HTML string)
- `dual-changes` (JSON with changeList)

These can be inspected in `output/debug-*` files to verify correlation worked correctly.

---

## File-by-File Summary

| File | Responsibility | Lines |
|------|----------------|-------|
| `gadie_engine.js` | Change correlation + rendering | ~400-500 |
| `gadic_cascade.css` | Layout + colors + styling | ~200-250 |
| `gadiu_user.js` | Button wiring + navigation | ~50-100 |

**Total**: ~650-850 lines, three independent files with clear contracts.
