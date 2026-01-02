# HMK - Claude Code Working Notes

## Draw.io as Foundation

HMK diagrams are authored in draw.io (.drawio files). This document captures implementation knowledge for working with these files programmatically.

## Element ID Naming Conventions

| Prefix | Meaning | Example |
|--------|---------|---------|
| `hs_` | Hard state | `hs_receive`, `hs_fault`, `hs_outer` |
| `dp_` | Decision point | `dp_poll_ignore`, `dp_crc_error` |
| `t_` | Transition (edge) | `t_receive_poll`, `t_crc_stall` |
| `init_dot` | Initial state marker | Single black filled circle |

## XML Structure

### Parent-Child Containment

Elements are nested via `parent` attribute:
- Root elements: `parent="1"` (the diagram root)
- Contained elements: `parent="hs_outer"` (or other container ID)

The outer container (`hs_outer`) is parented to root but contains child states and decision points.

### Container Configuration

**Critical setting for containers:**
```
recursiveResize=0
```

Add to container style to prevent child elements from resizing when container is resized:
```xml
style="rounded=1;whiteSpace=wrap;html=1;container=1;recursiveResize=0;..."
```

**UI equivalent:** Style tab → Properties → uncheck "Resize Children"

**Keyboard workaround:** Hold Ctrl while resizing container

### Shape Styles

**Hard states:**
```
style="rounded=1;whiteSpace=wrap;html=1;fontStyle=1;fontSize=14;arcSize=8"
```

**Decision points (ovals):**
```
style="ellipse;whiteSpace=wrap;html=1"
```

**Initial marker (black dot):**
```
style="ellipse;whiteSpace=wrap;html=1;aspect=fixed;fillColor=#000000"
```

**Transitions (edges):**
```
style="curved=1;endArrow=classic;html=1"
```

### Trigger Color Coding

| Trigger Type | Color | Hex Code |
|--------------|-------|----------|
| Message (ELEM_*) | Blue | `#0000CC` |
| Poll (_poll) | Green | `#009900` |
| Reset/Control | Purple | `#990099` |

Format: `<b><font color='#0000CC'><u>TRIGGER_NAME</u>(params)</font></b>`

### Transition Label Format

Labels use HTML with `<br>` for line breaks:
```
condition<br>/<br>action1<br>action2
```

Comments rendered as: `/* comment */` or `// comment`

## Geometry Notes

- Positions are relative to parent container
- `mxGeometry` contains x, y, width, height
- Edge waypoints in `<Array as="points">` with `<mxPoint x="..." y="..."/>`
- Self-loops use entry/exit anchor points (entryX, entryY, exitX, exitY)

### Coordinate System Fundamentals

From [mxGraph documentation](https://jgraph.github.io/mxgraph/docs/manual.html):
- All geometry coordinates are **relative to the parent cell's origin**
- This applies to both vertices AND edge waypoints
- When a parent cell moves, children stay in same relative position

### Self-Loop Parenting Strategy

**Problem:** By default, self-loops are parented to the container (`hs_outer`). When you drag a hard state, the self-loop's waypoints stay at their absolute position in the container's coordinate space.

**Solution:** Parent self-loops to their own vertex instead of the container.

```xml
<!-- BEFORE: waypoints in hs_outer coordinates -->
<mxCell id="t_ignore_write" parent="hs_outer" source="hs_ignore" target="hs_ignore" edge="1">
  <mxGeometry>
    <Array as="points">
      <mxPoint x="140" y="20" />  <!-- absolute in hs_outer -->
    </Array>
  </mxGeometry>
</mxCell>

<!-- AFTER: waypoints in hs_ignore coordinates -->
<mxCell id="t_ignore_write" parent="hs_ignore" source="hs_ignore" target="hs_ignore" edge="1">
  <mxGeometry>
    <Array as="points">
      <mxPoint x="20" y="-50" />  <!-- relative to hs_ignore origin -->
    </Array>
  </mxGeometry>
</mxCell>
```

**Coordinate conversion:** If the vertex is at (vx, vy) in the container, convert waypoint (wx, wy) to vertex-relative coordinates: `(wx - vx, wy - vy)`

**Benefits:**
- Self-loops move with their vertex when dragging
- No manual waypoint adjustment needed after repositioning
- Semantically correct: self-loop "belongs to" its state

**UI Alternative:** Select vertex + its self-loops → Ctrl+G (Group). Moves as unit but adds structural complexity.

## Working with Draw.io Files

### Reading Structure
1. Parse XML, find all `mxCell` elements
2. Separate vertices (`vertex="1"`) from edges (`edge="1"`)
3. Group by parent to understand containment
4. Edges have `source` and `target` attributes linking to vertex IDs

### Validating Diagrams
- Every `dp_*` should have 2+ outgoing edges (decision = multiple paths)
- Every edge should terminate at `hs_*` or `dp_*`
- `init_dot` should have exactly one outgoing edge
- Container states need entry path through nested `init_dot` or explicit target

## Future Directions

### Self-Loop Reparenting Arcanum

Install an arcanum providing a slash command (e.g., `/hmk-reparent-selfloops`) to automate the self-loop reparenting process:
- Scan diagram for edges where `source == target`
- Reparent each to its vertex instead of the container
- Recalculate waypoint coordinates from container-relative to vertex-relative
- Report changes made

This would allow a simple workflow: edit diagram in draw.io, then run the command to make self-loops draggable with their states.

### Back-and-Forth Transformer

A bidirectional transformation between draw.io visual layout and a semantic "essence" format.

**Concept:**
```
[draw.io with OCD layout] --extract--> [before.hmke]
                                            |
                                         (edit semantics)
                                            v
                                       [after.hmke]
                                            |
[draw.io with OCD layout] --merge(diff)--> [updated draw.io]
```

**Separation of concerns:**
- **Visual/Layout** (draw.io): coordinates, waypoints, styling, human-readable presentation
- **Semantic/Engineering** (.hmke): states, decisions, transitions, conditions, actions

**Implementation considerations:**
- Rust-based XML transformer
- Minimal dependencies: `roxmltree` (~2 transitive deps, pure Rust) for parsing
- Manual XML string building for output (zero additional deps)
- Trust is primary concern for external dependencies

### HMK Essence Format (.hmke)

Line-oriented edge list optimized for LLM comprehension, not human editing:

```
# === NODES ===
STATE   hs_ignore   "BLR_IGNORE"
STATE   hs_receive  "BLR_RECEIVE"
STATE   hs_fault    "BLR_FAULT"
STATE   hs_stall    "BLR_STALL"
DECIDE  dp_poll_ignore
DECIDE  dp_crc_error
CONTAINER hs_outer CONTAINS hs_ignore hs_receive hs_fault hs_stall dp_*

# === EDGES ===
# Format: FROM -> TO | TRIGGER | GUARD | ACTIONS
hs_ignore -> dp_poll_ignore | _poll | .goPending |
dp_poll_ignore -> hs_ignore | | blrd_receiverProcess()==_DONE | Send CDI_GO_REQ; goPending=false
dp_poll_ignore -> hs_receive | | else | blrd_reset; blkOffset=0; buffer.zero
hs_ignore -> hs_ignore | ELEM_WRITE_REQ | | /* ignored */
hs_outer -> dp_init | BLR_RESET_REQ | |
```

**Design rationale:**
- One fact per line (diff-friendly, greppable)
- Explicit relationships (no implicit references)
- Self-loops obvious: `X -> X`
- `|` delimiters unambiguous for typical content
- Trigger vs guard vs action is positional and consistent
- No coordinates or styling (layout file separate concern)

## Reference Links

- [Disable resize children](https://www.drawio.com/blog/disable-resize-children)
- [Resize shapes documentation](https://www.drawio.com/doc/faq/shape-resize)
- [Constrain proportions](https://www.drawio.com/doc/faq/shape-constrain-proportions)
- [mxGraph User Manual](https://jgraph.github.io/mxgraph/docs/manual.html) - coordinate systems, geometry
- [mxGeometry API](https://jgraph.github.io/mxgraph/docs/js-api/files/model/mxGeometry-js.html) - relative property, waypoints
- [draw.io Waypoints](https://www.drawio.com/blog/waypoints-connectors) - working with connector paths
