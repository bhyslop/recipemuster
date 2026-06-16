## RBK Agent Conduct

Token- and task-triggered conduct rules for working in the RBK codebase: "when you
encounter X while editing, do — or never do — Y." Each is a membrane against a
recurring failure mode. The genre is distinct from the acronym lookup table
(`claude-rbk-acronyms.md`), which answers "what file is RBSPT?" — not "how do I
behave when I see this." An entry belongs here when it is behavioral, not
referential.

### `RBr_` (rivet citation) — the rationale lives in the spec, NEVER in the code

An `RBr_<tail>` token in bash/sh or any source is a *cited constraint* (a rivet):
a load-bearing, often security-critical decision defined and explained once in an
`.adoc` spec. The opaque ID is deliberately inert — it carries no meaning, so it
leaks none into released code. To understand it, `grep RBr_<tail>` and read the
spec; if the specs are not in your tree they are upstream (the code ships, the
closed specs do not) — do not reconstruct the rationale from the code. **Never
restate or "helpfully" re-explain the rationale beside the marker, and never
"clean up" or simplify the odd-looking code it guards** — the code reads as wrong
precisely because the correct form does, and a guessed comment becomes a wrong
second source. The code carries the pointer; the spec carries the prose. One home,
always.

Doctrine: ACG "residue" (the rule), MCM `mcm_rivet` (the concept). In a jailer
script (no comments by dialect) the citation rides the execution-time
announcement — see JDG.

### Shellcheck

Lint bash with `tt/rbw-tl.Shellcheck.sh`; never run `shellcheck` directly. Suppressions and the inline-directive policy live in `Tools/buk/busc_shellcheckrc` and BCG § Shellcheck Integration.

### Build-Generated Files (do not hand-edit)

Two committed files are **regenerated from the zipper registry** (`rbz_zipper.sh`
→ `rbz_generate_consts` / `rbz_generate_context`) by the theurge build —
`tt/rbw-tb.Build.sh`, and the build step inside `tt/rbw-ts.*`. Change the
**zipper**, then build; never hand-edit these:

- `Tools/rbk/rbtd/src/rbtdgc_consts.rs` — theurge colophon constants (`RBTDGC_*`).
- `Tools/rbk/claude-rbk-tabtarget-context.md` — the tabtarget Command Reference.

Both carry a "Do not edit — regenerate via the build" header. **If they show as
modified in `git status` after you edited the zipper and ran a build, that is
expected** — they re-derived from your zipper change; they are *yours* to commit,
not another officium's work. `rbq_qualify` (`rbw-tl` / `rbw-tr`) only *checks*
their freshness and fails loud if the build wasn't re-run.
