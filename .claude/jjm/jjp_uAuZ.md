# Paddock: cmk-diptych-prototype

## Context

Diptych is a dual-representation document format for concept models, replacing AsciiDoc
with a format natively optimized for two consumers: git (recto: word-per-line) and
LLMs (verso: joined prose). Prefix characters replace paired delimiters. The mapping
spine is preserved; AsciiDoc ceremony is eliminated.

Key concepts: canon (authoritative source), lectionary (selection rules),
lectio (task-focused extract for constrained agents), recto/verso (git/LLM faces),
codec (trivial mechanical transform between them).

## References

- `Memos/memo-20260209-diptych-format-study.md` — Discovery draft v0.1 (2026-02-09)
- `Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc` — Semantic structure Diptych replaces the expression of
- `Tools/cmk/AXMCM-ClaudeMarkConceptMemo.md` — Earlier exploration, superseded by Diptych
- ₣Aj `rbk-axla-term-voicing-scrub` — Adjacent: redesigning voicing semantics (procedure collapse, entity affiliation). Diptych changes the syntax those voicings are expressed in.

## Session Notes (2026-02-25, axla-procedure-repair chat)

Connection identified: ₣Aj designs what voicings MEAN (collapse procedure hierarchy,
entity affiliation as hierarchy marker, drop language dimension). ₣AZ designs how
voicings are WRITTEN (prefix characters, no AsciiDoc ceremony). In Diptych, cheap
annotations liberate the voicing design — multiple single-concern annotation lines
instead of overloaded lineage chains.