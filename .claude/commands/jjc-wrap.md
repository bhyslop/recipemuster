# jjc-wrap — delegated frontier review-and-wrap of a bridled pace

Arguments: `$ARGUMENTS` — the tier (required) and optionally the coronet, in
either order. The tier is exactly `fable` or `opus`; there is no default — the
reviewer tier is a deliberate operator ruling. The coronet, when given, is the
full form (e.g. `₢BsAAl`); when omitted it is inferred from this session's
context (step 1). The two are told apart by their disjoint shapes — a tier is
exactly `fable`|`opus`, a coronet is `₢` + 5 characters — so order does not
matter and a lone tier is unambiguous.

By invoking this command the operator asserts the work is believed complete and
authorizes the wrap on a passing review. In particular, the "wrap sweeps all
uncommitted changes" caution is expressly waived here: the operator asserts the
tree's contents are correct to sweep. Do not refuse or pause over a dirty tree.

Invoking this command likewise constitutes the operator's direction to refit the
billet — to enfold trunk's advance before the review runs — exactly as it waives
the dirty-tree caution. It is per-invocation direction, not standing permission:
no session gains authority to refit on its own judgment.

That refit is executed by the spawned reviewer, never by the calling session.
`jjx_refit` is frontier-gated, and this command's caller is routinely a
sub-frontier designee reporting the pace it just landed — so a caller-side refit
would meet an interdictum and, since an interdictum bars the objective by every
route, halt the whole ceremony rather than degrade. The reviewer is frontier by
construction, so the operator's direction travels into the reviewer prompt with
the rest of the ceremony and is discharged there.

## What you (the calling session) do

1. Parse the tier and coronet from the arguments by their disjoint shapes (a
   tier is exactly `fable`|`opus`; a coronet is `₢` + 5 characters), in either
   order. The **tier is required**: if it is missing or not exactly `fable` or
   `opus`, report that and stop — never assume a default. The **coronet is
   optional**:
   - If supplied, use it.
   - If omitted, infer it from this session's context — the pace this session
     most recently mounted or landed. State the inferred coronet explicitly
     (e.g. "No coronet supplied; inferring ₢B4·CAAAX, the pace mounted this
     session") and proceed. That statement lands before the wrap fires (step 5),
     so it is the operator's veto window — state and proceed, never assume
     silently.
   - Drop to a question only on genuine ambiguity: no pace was mounted or landed
     this session, or two equally-recent candidates exist.
2. Ensure a **review-grade** landing exists — judged by its *content*, not by
   mere existence. Read the pace's most recent landing (if any) and hold it
   against the landing-content contract (JJSCLD `content`): evidence pointers for
   the reviewer, never advocacy. Then:
   - If no landing exists, OR the most recent one is advocacy-laden, write a
     fresh review-grade landing from this session's knowledge via `jjx_landing`
     before spawning the reviewer — it supersedes the advocacy one (the reviewer
     reads the latest). Never wave a landing through as "review-grade" just
     because one is present: a designee's completion report is advocacy-laden by
     construction, so expect to rewrite.
   - If the most recent landing already meets the contract, use it as-is.
3. **Do not run the refit yourself.** The refit preflight belongs to the
   reviewer (its step 2), which is frontier by construction. `jjx_refit` is
   frontier-gated: a sub-frontier caller — the common case here — would be
   refused with an interdictum, and an interdictum bars the objective by every
   route, so the ceremony would halt rather than proceed on a stale tree. Do not
   attempt it, and do not reason about your own tier to decide whether you may;
   your only refit duty is relay (step 5).
4. Spawn ONE Agent (subagent_type `general-purpose`, `model` set to the tier,
   `run_in_background: false`) with the reviewer prompt below, substituting the
   coronet. Do not perform the review yourself in the main session — the point
   is a fresh-context reviewer.
5. Relay the reviewer's outcome to the operator verbatim in substance: either
   "wrapped, with this summary and spook", the specific repair recommendations,
   or — when the reviewer aborted on its refit preflight — that `jjx_refit`
   message verbatim, with no conflict judgment of your own added. Do not apply
   repairs, and do not retry a refit abort, unless the operator directs it.

## Reviewer prompt (pass to the Agent, with <CORONET> substituted)

You are a frontier reviewer performing the Job Jockey wrap ceremony for
bridled pace <CORONET>. Your job: independently verify the landed work against
its orders, then either wrap the pace or report repairs. You never fix code.

Protocol:

1. **Open an officium**: call `mcp__vvx__jjx` with command `jjx_open`, empty
   params, and your verbatim model ID as `model`. Use the returned ☉-id as
   `officium` on every later jjx call. `jjx_open` must land alone — never
   co-batched with a call consuming its result.
2. **Refit preflight — enfold trunk before you read anything.** Call
   `mcp__vvx__jjx` with command `jjx_refit`, empty params. Your working
   directory is the billet, so this runs where it must. It comes first because a
   refit can move the tree, and the point is to judge the tree that will
   actually land rather than a pre-refit one. The operator's invocation of this
   command is itself the direction to run it: do not pause to ask, and do not
   gate it on any foreknowledge of staleness — refit reports up-to-date for free
   when trunk has not moved. Route on the outcome:
   - **up-to-date** or **refitted** → proceed to gather the record (step 3).
   - **any other outcome** — offline, or a refusal (a content conflict, a dirty
     billet, or a resolution failure) — stop the ceremony. Do not review, do not
     wrap, do not retry, and do not interpret, judge, or repair a conflict;
     richer conflict handling is a deliberately separate concern, not this
     command's. Return the `jjx_refit` message verbatim as your entire report.
3. **Gather the record**:
   - `jjx_brief {coronet: "<CORONET>"}` — the docket (the orders).
   - Write `# jjezs_halter <CORONET>` to the officium's `gazette_in.md`
     (path from jjx_open), then `jjx_show {"remaining": false}` and read
     `gazette_out.md` — paddock context and the pace docket.
   - The pace's most recent landing — read it as **claims to verify, not
     findings to inherit**: its commit pointers, operator rulings, and
     verification claims guide where you look, but you confirm each against the
     record yourself. An operator ruling cited there may legitimately waive a
     docket clause; verify the ruling is stated as the operator's, then honor it.
   - The pace's commits:
     `git log --all --grep '<bare-coronet>' --stat` (bare = coronet without ₢)
     to find the work and its diff. Read the actual changed code.
4. **Stage 1 — docket conformance**: does the landed work accomplish what the
   docket ordered, completely, within the paddock's cinches? Judge outcomes,
   not effort. A hole, an unmet "Done when" clause, or silent scope reduction
   is a FAIL.
5. **Stage 2 — guide compliance**, scoped to the diff (changed lines and what
   they directly touch — never audit whole files; pre-existing violations are
   at most a one-line aside, never wrap-blockers):
   - If the diff touches bash (`.sh`): first run `tt/rbw-tl.Shellcheck.sh`
     directly (no pipes, no tee), then read `../logs-buk/last.txt` in a
     separate command. A red shellcheck is an automatic FAIL. Then read
     `jjqs_studbook/guides/buk/BCG-BashConsoleGuide.md` and judge the changed
     bash against it.
   - If the diff touches Rust (`.rs`): read
     `jjqs_studbook/guides/vok/RCG-RustCodingGuide.md` and judge the changed
     Rust against it.
   - **Cite the rule or drop the finding**: every compliance finding must name
     the specific BCG/RCG section or convention it violates. Uncited taste is
     not a finding.
   - Never restate or simplify code guarded by `RBr_`/`JJr_` rivet markers.
6. **Verdict**:
   - **All green** → wrap: `jjx_close {coronet: "<CORONET>", summary: "<what
     the work accomplished>", spook: "<friction you observed reviewing, or
     'none'>"}`. The operator pre-authorized this wrap by invoking the
     command; a dirty tree is not a reason to refuse. Do not commit before or
     after — jjx_close commits everything itself.
   - **Any red** → do NOT wrap, do NOT fix anything. Return specific repair
     recommendations: for stage 1, which docket clause is unmet and what would
     meet it; for stage 2, each violation with its cited rule and location.
7. Your final message is the report: verdict first, then per-stage findings.
