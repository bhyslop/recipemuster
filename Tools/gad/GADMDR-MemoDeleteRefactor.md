# GAD Diff Specification Revision Memo

## Phase Renumbering

* Replace fractional phases (2.5, 5.5, etc.) with whole numbers.
* Suggested sequence:

    1. Immutable Input DOMs
    2. Detached Working DOM (anchor-stable)
    3. Deletion Fact Capture (from *From* DOM)
    4. Diff Operations (insertions, moves, modifications only)
    5. Annotated DOM Assembly (no deletions yet)
    6. Deletion Placement via Semantic Anchors
    7. Uniform Classing and Coalescing
    8. Serialize from Detached DOM

## Debug Output Enhancements

* Add **more intermediate debug view files**:

    * Post-Deletion Fact Capture (DFT dump)
    * Post-Annotated DOM Assembly (before deletions)
    * Post-Deletion Placement (before coalescing)
    * Post-Coalescing (final visual structure before serialization)
* Each debug capture should clearly label its phase and timestamp.

## WebSocket Operations Simplification

* Current state: one WebSocket op per file type (rendered\_content, annotated\_dom, etc.).
* Recommendation: **use a single WebSocket operation type** (e.g., `debug_output`).
* Pass filename and metadata as parameters:

    * `filename_pattern` (explicit target name)
    * `phase` (e.g., `phase3-dft`, `phase6-deletions`)
    * `source_files` (from/to HTML inputs)
* This simplifies the protocol and avoids proliferating op types.

## Expected Benefits

1. Stable preservation of `#fragment` links (no absolute URL pollution).
2. Predictable deletion placement (anchored, deduplicated, no DIFF-ERRORs).
3. Cleaner phase structure: easier for engineers to reason about.
4. Richer debug artifacts: more visibility for diagnosing errors.
5. Simpler WebSocket interface: fewer ops, more flexible payloads.
