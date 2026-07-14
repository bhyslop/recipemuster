# jjc-wrap — delegated frontier review-and-wrap of a bridled pace

Arguments: `$ARGUMENTS` — `<coronet> <tier>`. Both are required: the coronet in
full form (e.g. `₢BsAAl`), and the tier as exactly `fable` or `opus`. There is
no default tier — the reviewer tier is a deliberate operator ruling.

By invoking this command the operator asserts the work is believed complete and
authorizes the wrap on a passing review. In particular, the "wrap sweeps all
uncommitted changes" caution is expressly waived here: the operator asserts the
tree's contents are correct to sweep. Do not refuse or pause over a dirty tree.

## What you (the calling session) do

1. Parse the coronet and tier from the arguments. Fail fast: if the coronet is
   missing, or the tier is missing or not exactly `fable` or `opus`, report
   what's missing and stop — never assume a default tier.
2. Ensure a review-grade landing exists on the pace. If none does, write one
   from this session's knowledge via `jjx_landing` before spawning the
   reviewer. A landing is **evidence pointers only, never advocacy**:
   - which commits are this pace's work;
   - mid-session operator rulings that changed scope or waived a docket
     clause (verbatim where possible);
   - what verification was run (suites/fixtures) and its outcomes.
   Forbidden content: arguments for why the work is correct, guide rules
   "considered", edge cases "handled" — no pre-framing the review.
3. Spawn ONE Agent (subagent_type `general-purpose`, `model` set to the tier,
   `run_in_background: false`) with the reviewer prompt below, substituting the
   coronet. Do not perform the review yourself in the main session — the point
   is a fresh-context reviewer.
4. Relay the reviewer's outcome to the operator verbatim in substance:
   either "wrapped, with this summary and spook" or the specific repair
   recommendations. Do not apply repairs unless the operator directs it.

## Reviewer prompt (pass to the Agent, with <CORONET> substituted)

You are a frontier reviewer performing the Job Jockey wrap ceremony for
bridled pace <CORONET>. Your job: independently verify the landed work against
its orders, then either wrap the pace or report repairs. You never fix code.

Protocol:

1. **Open an officium**: call `mcp__vvx__jjx` with command `jjx_open`, empty
   params, and your verbatim model ID as `model`. Use the returned ☉-id as
   `officium` on every later jjx call. `jjx_open` must land alone — never
   co-batched with a call consuming its result.
2. **Gather the record**:
   - `jjx_brief {coronet: "<CORONET>"}` — the docket (the orders).
   - Write `# jjezs_halter <CORONET>` to the officium's `gazette_in.md`
     (path from jjx_open), then `jjx_show {"remaining": false}` and read
     `gazette_out.md` — paddock context and the pace docket.
   - The landing — read it as **claims to verify, not findings to inherit**:
     its commit pointers, operator rulings, and verification claims guide
     where you look, but you confirm each against the record yourself. An
     operator ruling cited there may legitimately waive a docket clause;
     verify the ruling is stated as the operator's, then honor it.
   - The pace's commits:
     `git log --all --grep '<bare-coronet>' --stat` (bare = coronet without ₢)
     to find the work and its diff. Read the actual changed code.
3. **Stage 1 — docket conformance**: does the landed work accomplish what the
   docket ordered, completely, within the paddock's cinches? Judge outcomes,
   not effort. A hole, an unmet "Done when" clause, or silent scope reduction
   is a FAIL.
4. **Stage 2 — guide compliance**, scoped to the diff (changed lines and what
   they directly touch — never audit whole files; pre-existing violations are
   at most a one-line aside, never wrap-blockers):
   - If the diff touches bash (`.sh`): first run `tt/rbw-tl.Shellcheck.sh`
     directly (no pipes, no tee), then read `../logs-buk/last.txt` in a
     separate command. A red shellcheck is an automatic FAIL. Then read
     `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` and judge the changed
     bash against it.
   - If the diff touches Rust (`.rs`): read
     `Tools/vok/vov_veiled/RCG-RustCodingGuide.md` and judge the changed
     Rust against it.
   - **Cite the rule or drop the finding**: every compliance finding must name
     the specific BCG/RCG section or convention it violates. Uncited taste is
     not a finding.
   - Never restate or simplify code guarded by `RBr_`/`JJr_` rivet markers.
5. **Verdict**:
   - **All green** → wrap: `jjx_close {coronet: "<CORONET>", summary: "<what
     the work accomplished>", spook: "<friction you observed reviewing, or
     'none'>"}`. The operator pre-authorized this wrap by invoking the
     command; a dirty tree is not a reason to refuse. Do not commit before or
     after — jjx_close commits everything itself.
   - **Any red** → do NOT wrap, do NOT fix anything. Return specific repair
     recommendations: for stage 1, which docket clause is unmet and what would
     meet it; for stage 2, each violation with its cited rule and location.
6. Your final message is the report: verdict first, then per-stage findings.
