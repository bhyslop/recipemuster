# Three-heat plan-integrity audit — directive

**Issue this in a FRESH chat to audit the integrity of the three-heat parallelization plan (₣Bf / ₣Bi / ₣Bl).**

This is a **read-only** audit: it surfaces findings, it does not mutate the plan. Use **workflows and ultracode** — this is a substantive fan-out audit, not a quick check. Be **skeptical and independent**: re-derive correctness from first principles, do not assume the plan is sound, and do not trust any prior agent's conclusions — including the existing barrier paces and the most recent reslates. Re-verify everything cold.

## Why this audit exists
The plan was built by aggressive LLM-driven restructuring: a large cross-heat restring (paces moved between heats, each getting a new coronet), a per-heat paddock restitch under a heat-gestalt shift, and the insertion of cross-clone barrier paces. Each step can introduce integrity defects that are invisible at the point of change and only bite when a heat is mounted in its clone. Hunt them before they do.

## Ground truths (operator-confirmed — take as given)
- **Three heats, three clones.** ₣Bf (federation-build), ₣Bi (MVP cleanup), ₣Bl (theurge-refactor) each run in their own repo clone, synced occasionally through origin. Cross-clone coordination is carried by **barrier paces**: gate-only paces (silks begin `await-`, docket body opens `BARRIER`) whose completion criterion is "cannot complete until ₢X is wrapped, pushed to origin, and origin merged back into this clone."
- **Coronets are allocated monotonically and never recycled.** A coronet freed by a restring is never reassigned. Therefore a coronet reference that does not resolve against the live inventory is a **stale dangle — never a silent collision** with a different pace. This bounds the referent check: non-resolving = stale, full stop.
- **The contention model the plan is built on** (the frame for "cross-clone dependency"): the hot shared spine files are
  - `Tools/rbk/vov_veiled/RBS0-SpecTop.adoc` — owned by ₣Bf;
  - `Tools/rbk/rbtd/src/rbtdrc_crucible.rs` and the zipper trio (`Tools/rbk/rbz_zipper.sh` + the build-regenerated `Tools/rbk/rbtd/src/rbtdgc_consts.rs` + `Tools/rbk/claude-rbk-tabtarget-context.md`) — owned by ₣Bl;
  - `Tools/rbk/claude-rbk-acronyms.md` — soft (append-distinct), owner-of-record ₣Bf.
  A cross-clone dependency exists wherever two paces in different heats both write — or must order against — one of these files. **Do NOT take the installed barriers or the stated choreography as the authority on what those dependencies are; re-derive them from the dockets and grep, then compare.** The hot-file *ownership* itself is also a thing to verify (see Coverage below), not just to assume.

## Scope
- **IN:** the **remaining (rough) paces** of all three heats (their dockets), the **barrier paces**, the **three paddocks**, and the structural relationships among them.
- **OUT:** provenance memos and any proposal file — do not audit their prose.
- Resolve every coronet reference against the **full** live inventory (done + abandoned included), since a barrier or docket can name any pace.

## How to read the plan (never reach into raw gallops JSON)
- `jjx_open` → officium (pass the model id; carry the officium on every later call).
- **Change ledger:** `git log --oneline -- .claude/jjm/jjg_gallops.json` — readable; every jjx op self-describes ("restring N paces from ₣X", "batch: 1 reslate", "rough → abandoned", "silks=…"). Use it to see exactly what was transferred / reslated / dropped, and scrutinize those paces hardest.
- **Inventory + order, per heat:** `jjx_coronets {firemark, remaining:true}` (the mountable order) and `jjx_coronets {firemark}` (the full inventory, for reference resolution).
- **Dockets + paddocks:** write three `# jjezs_halter <firemark>` notices to `gazette_in.md`, call `jjx_show {remaining:true}`, read `gazette_out.md` (paddocks + all remaining dockets); or `jjx_brief {coronet}` per pace.
- **Identify barriers:** paces whose silks begin `await-` or whose docket body opens `BARRIER`. Treat them as gates, not code paces — do not re-footprint them as work; validate them as gates.

## Method — full re-derive, then reconcile
1. **Re-footprint every remaining non-barrier pace, cold.** From its docket plus grep of the repo, derive (a) the files it writes, (b) its hard dependencies — which other paces (any heat) must land before it, and why. Derive independently; do **not** read the installed barriers or choreography while footprinting. Adversarially verify each footprint (a second pass that tries to refute it).
2. **Build the dependency graph.** Classify each dependency as in-clone (same heat → handled by pace order) or cross-clone (different heat, mediated by a hot file → must be a barrier).
3. **Reconcile against the installed plan** across the taxonomy below.
4. **Adversarially verify every finding** (is it real, or an artifact of your re-derivation?), severity-rank, and report.

## Risk taxonomy — what to hunt (with the failure patterns)
- **Referent integrity.** Resolve every coronet named in every docket and every barrier against the full inventory. Any non-resolving coronet is a stale dangle — flag with location. Idiomatic provenance breadcrumbs (`Drafted from ₢X in ₣Y`, where ₢X is a long-dead source) are *expected* dangles — note them as low-severity, distinct from a load-bearing dangle in dependency or instruction prose.
- **Topology staleness.** A pace that moved heats whose docket prose still assumes its OLD heat membership. Patterns to match: "do not mount until ₣X wraps" inside a pace now itself in ₣X (a heat told to wait for itself); "supersede the pace in a separate heat / retire its empty heat" when that pace is now in this same heat; "this heat" / "this heat's X" pointing at the wrong heat after the move; a dependency described as cross-heat that is now in-clone, or vice versa.
- **Barrier gaps & spurious barriers — the highest-value check, the reason this audit re-derives.** For every cross-clone dependency in your re-derived graph, confirm a barrier exists *in the waiting heat*, positioned *before* the dependent pace, naming the *correct* upstream coronet, with the merge-origin-and-back semantics. **A real cross-clone dependency with no barrier is a silent gap** — nothing flags it until two clones collide mid-build; this is the defect class most worth the workflow. Conversely flag any barrier that gates a dependency your re-derivation does not support (spurious), whose named upstream coronet is wrong or dangling, or which sits before the wrong pace.
- **Ordering & cycles.** Within each heat, in-clone dependencies must respect pace order (a consumer must not precede its producer). Across heats, hunt for a cross-clone dependency **cycle** (heat A's pace waits on heat B's, which waits back on heat A's) — a deadlock that would strand all three clones.
- **Coverage & assignment.** Every remaining pace lives in exactly one heat (none lost, none duplicated across the restring). Every writer of a hot spine file sits in that file's owning clone (RBS0 → ₣Bf; crucible + zipper trio → ₣Bl). A hot-file writer stranded in the wrong clone is a mis-assignment that the barriers cannot rescue.
- **Paddock ↔ reality.** Significant paddock revisions accompanied a heat-gestalt shift, so check each paddock hard: does its prose match its heat's ACTUAL remaining-pace inventory (no described work that has since left; no silent omission of work that arrived)? Is it coronet-clean and silks-clean (paddock prose must name neither)? Do the cross-heat drains/moves the three paddocks describe reconcile with one another?

## Output
Surface findings **in chat** — the operator will act on them immediately to fix paddocks and dockets. A **severity-ranked report**, severity = blast radius and confabulation risk (a silent barrier gap or a wrong dependency reference outranks an idiomatic provenance dangle). Each finding must be immediately actionable:
- exact location (heat firemark, coronet, section/line),
- what is wrong and why it matters,
- a ready-to-apply proposed fix (the reslate body, the missing barrier's docket + heat + before-which-coronet, the drop).
The audit itself **never mutates** — it proposes; the operator applies in follow-on fixer chats.

## Suggested workflow shape (adapt freely)
1. Scout inline: officium; change ledger; per-heat inventories (remaining + full); dump all dockets, paddocks, and barriers to scratch files.
2. Workflow A (fan out): re-footprint + re-derive deps per remaining non-barrier pace (footprint → adversarial verify), barrier-collected into a dependency graph.
3. Inline: assemble the cross-clone dependency graph; index the installed barriers and the pace order per heat.
4. Workflow B (fan out by taxonomy dimension): reconcile each dimension, each candidate finding adversarially verified.
5. Synthesize: the severity-ranked, immediately-actionable report.
