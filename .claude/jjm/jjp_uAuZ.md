## Context

Diptych is a dual-representation document system for concept models,
replacing AsciiDoc with a format natively optimized for two consumers:
git (recto: word-per-line) and LLMs (verso: joined prose).
Prefix characters replace paired delimiters.
The mapping spine is preserved; AsciiDoc ceremony is eliminated.

Key concepts:
canon (authoritative source), lectionary (selection rules),
lectio (task-focused extract for constrained agents),
recto/verso (git/LLM faces),
codec (trivial mechanical transform between them),
recension (candidate name: tool-executed global re-minting pass over a canon).

## Scope and heat shape

This heat is the format/codec prototype.
The vision memo names a one-grammar/three-consumers spine — codec, validator, recension —
and this heat owns the codec; the other two are homed elsewhere:

- The validator intent flowers in VOS (₣AD territory).
- Recension is unassigned to any heat yet.

The memo declares the recension scope boundary —
canon-side renames ride the Diptych lexer; cross-universe renames ride the zipper registries —
and holds a removal-conditions ledger of tolerated inconsistencies awaiting this machinery.

One specified-but-unhomed feature now spans validator and recension: link coverage / auto-link.
Detection is a validator rule — a plain-prose mention of an already-quoined display-text is a candidate unlinked reference;
application is a recension mode — rewrite those candidates across a canon without touching meaning.
Its worklist and magnitude are sized in the memo's ledger; it has no heat of its own.

## Relationship to ₣Aj

₣Aj designs what voicings MEAN — collapse procedure hierarchy, entity affiliation as a hierarchy marker, drop the language dimension.
This heat designs how voicings are WRITTEN — prefix characters, no AsciiDoc ceremony.
In Diptych, cheap annotations liberate the voicing design:
many single-concern annotation lines instead of overloaded lineage chains.

## References

- `Memos/memo-20260209-diptych-vision.md` — the vision memo; umbrella for the spine, the recension scope boundary, and the removal-conditions ledger.
- `Memos/memo-20260610-quoin-minting-introspection.md` — LLM-side minting introspection; validator-rule feedstock.
- `Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc` — the semantic structure whose AsciiDoc expression Diptych replaces.
- `Tools/cmk/AXMCM-ClaudeMarkConceptMemo.md` — earlier exploration, superseded by Diptych.