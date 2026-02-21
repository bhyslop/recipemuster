# Paddock: jjk-commission-haiku-pilot

## Context

Architect JJK for tiered model execution: Haiku as the fast, cheap "jockey" for routine operations, with Opus as the "steward" brought in for judgment calls.

**Origin:** Trial run with Haiku piloting JJK went surprisingly well, but exposed gaps. The path forward: tune JJX output to be Haiku-legible, and explicitly elect Opus for operations requiring deep judgment.

**Vision:**
- Haiku handles 80% of operations — mechanical, well-structured, fast
- Opus handles 20% — bridleability assessment, complex groom decisions, ambiguous specs
- The system knows when small-think suffices and when big-think is needed

## Key Questions

1. **Escalation triggers** — Hard-coded (bridle → always opus) vs. dynamic (haiku recognizes "I need help")?
2. **Model switching mechanics** — How does Claude Code switch models mid-flow? Task tool with `model: "opus"`?
3. **Which operations need Opus?** — Bridle yes. Also: groom? reslate? quarter?
4. **Output redesign** — What makes JJX output "Haiku-friendly"? More explicit state, decision trees, less inference required.

## Architectural Considerations

- JJX commands should emit structured, unambiguous output that smaller models can parse reliably
- Slash command prompts may need model-specific variants or clearer decision criteria
- Bridling is fundamentally a judgment call about scope, ambiguity, and risk — Opus territory
- Cost/speed tradeoff is significant: Haiku for iteration, Opus for commitment

## Spook Log

A "spook" is a failure of slash command guidance or CLAUDE.md context to produce a correct `jjx_*` invocation, causing the agent to stumble and recover via `--help` or trial-and-error. Each spook indicates a gap in the upper API layer. The fix for a spook is always a deft adjustment to CLAUDE.md, a slash command, or jjx output — whichever layer failed to guide the agent correctly. The goal is prevention: make the same mistake impossible next time.

### jjx_alter --status (260209)

CLAUDE.md CLI reference shows `jjx_alter FIREMARK [--status racing|stabled]` but the actual CLI uses `--racing` / `--stabled` flags (no `--status` option). Agent used `--status racing`, got rejected, had to `--help` to discover the correct flag.

**Root cause:** CLAUDE.md CLI reference diverged from implemented CLI.

## References

- Tools/jjk/vov_veiled/JJS0-GallopsData.adoc — JJK data model
- .claude/commands/ — Slash command definitions
- Task tool `model` parameter — mechanism for model selection in subagents
