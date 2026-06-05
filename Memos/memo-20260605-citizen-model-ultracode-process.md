# Ultracode in Practice — How the Citizen-Model Revision Was Produced

Date: 2026-06-05

Audience: Brad. Purpose: explain, plainly, the multi-agent ("ultracode") process that produced the
260605 revision of the citizen-model paddock (₣BZ) and its mechanics memo — so you can judge when to
reach for it again, and read its output critically. This memo is *about the process*, not the model;
the model lives in `memo-20260605-citizen-capability-model.md` and the ₣BZ paddock.

## What ran, in one breath

You turned on ultracode and asked for "improvements throughout." That became **two background
workflows plus my own editing**, in sequence:

1. **Review workflow** — fan out 9 independent reviewer "lenses" over the artifacts → adversarially
   verify every candidate finding → synthesize a ranked recommendation set. (45 candidates → 26
   survived.)
2. *(I curated those into recommendations; you made the two naming calls; I applied the sweep
   myself.)*
3. **Verification workflow** — fan out 4 independent checkers over the *edited* files to confirm
   every finding landed, the rename swept clean, and nothing regressed → synthesize residuals.
   (Found 2 real residuals, which I then fixed.)

The editing in step 2 was **not** a workflow — see "Where agents don't help" below.

## The orchestration shape

A "workflow" here is a deterministic JavaScript script *I* author and the runtime executes in the
background. The script decides what fans out, what verifies, what synthesizes; the runtime spawns
subagents to run each piece, caps concurrency (~10–16 at once), and notifies me when done. The
subagents are themselves Claude instances, but headless: each is told its final output **is data,
not a message**, and most are forced to return a **schema-validated JSON object** (a "finding," a
"verdict," a "residual") rather than prose. That structure is what lets the script count, filter,
dedup, and route results mechanically.

Three roles, kept distinct:

- **Subagents** — blind to each other; each does one narrow job (one review lens, one verification
  of one finding) and returns structured data.
- **The script** — pure control flow: fan-out, the adversarial-verify stage, dedup, synthesis. No
  judgment, just plumbing.
- **Me (the main agent)** — author the scripts, read the synthesized output, **apply my own
  judgment** on top (curate, down-rate, reject), do the coherent editing, and hand you the
  genuinely-yours decisions (the naming calls). The workflow is an instrument; it does not replace
  the judgment layer.

## Workflow 1 — the review (the valuable part)

**Pattern: fan-out → adversarial verify → synthesize.**

*Fan-out.* Nine lenses, each a separate subagent with a distinct mandate, blind to the others:
internal coherence; conflict with the source memos; fidelity to current behavior in the `.adoc`
specs; **implementation reality** (one lens grepped the actual RBK bash); **GCP facts** (one lens
re-verified the platform claims against live Google docs on the web); adversarial security ("try to
break the model"); vocabulary/minting/load-bearing-complexity; conciseness/shape-discipline; and a
completeness critic ("what's missing"). Diversity is the point — each lens sees what the others are
blind to. They produced **45 candidate findings**.

*Adversarial verify.* This is the step that earns the cost. Each of the 45 candidates was handed to
a *fresh* skeptic subagent told to **refute it** — independently re-read the cited file/spec or
re-run the web search, and default to "false" if it couldn't substantiate the claim. This killed
**19** plausible-but-wrong findings, and in at least one case *corrected* a survivor: a finding
proposed a revoke-ordering that was actually backwards, and the verifier flagged and fixed it. **26
survived.**

*Synthesize.* A final subagent deduped the 26, ranked them, and split them into "safe to apply" vs
"your decision," with a concrete fix each.

*Cost:* ~55 subagents, ~4.05M output tokens, ~9 minutes wall-clock. That is the price of
exhaustiveness; ultracode treats token cost as not-a-constraint, which is why this is opt-in, not
the default.

What I did with it: I read all 26, **agreed with most but not as an oracle** — I re-rated a few,
framed two as naming decisions for you, and flagged the three that were genuine soundness gaps
(including one that exposed a hole in a claim *I* had made the prior turn). That curation is the
human-judgment layer; the workflow surfaced candidates, it did not decide.

## Workflow 2 — verifying my own work

After you approved the two naming calls and I applied the ~22-edit sweep, I ran a second workflow to
**check my own application** — the ultracode habit of "adversarially verify your findings" turned on
*my* edits, not the design. Four independent checkers read the edited files on disk (ground truth,
not my summary): finding-coverage (did each of the 22 land?), rename-sweep (any stray "declared
roster"? did RBSHR's artifact-"citizen" become "holding"?), regression-hunt (did the big edit
introduce new contradictions?), and cross-doc consistency.

It found **2 real residuals** — both in the verb-dissolution table, both artifacts of the rename: a
row that mapped the `roster` verb to the wrong read, and a row whose left-to-right order inverted the
"ledger-withdraw-first" invariant. Neither was cosmetic; the second exposed that I'd attached an
invariant to the wrong verb. I fixed both. *This is the lesson:* a self-verification pass catches not
only the reviewers' false positives but **your own implementation errors** — the edit sweep was large
enough that I introduced two defects, and the second workflow caught them where eyeballing might not.

## What broke (and the recovery)

The verification workflow **failed on its first launch** — a plumbing bug: I passed the findings list
through the workflow's `args` channel and the script couldn't read it (`args.findings` was
undefined), so it threw at script-eval before spawning a single agent (0 agents, 3 ms, no cost). The
fix was to stop relying on `args` and **inline the data** into the script, then relaunch. Worth
knowing: workflow scripts are plain JS with a few sharp edges (no `Date.now`/`Math.random`, careful
with how data is passed in), and a script-level error fails fast and cheap rather than half-running.

## Where agents help, and where they don't

- **They help** for *fan-out* — many independent perspectives (review) or many independent checks
  (verify), each narrow, each blind to the others, then mechanically combined. Breadth and
  adversarial pressure are the wins.
- **They don't help** for *coherent single-document editing.* Applying 22 interdependent edits to two
  files — especially a vocabulary rename that must stay consistent across every sentence — is a
  single-writer job. Parallel agents would conflict on the same lines or drift the new term
  ("declared ledger" vs "capability ledger" vs "the ledger"). So I did that solo, by hand, and used a
  workflow only to *check* it. Picking the right tool per phase matters more than maximizing agent
  count.

## How to drive it yourself

- **Invoke** by including "ultracode" in a message, or `/effort`. That is the opt-in; without it I do
  not spin up dozens of agents.
- **Watch** a running workflow live with `/workflows`. Each run also writes a transcript directory
  (subagent logs) and a script file you can inspect or hand me to re-run.
- **Resume**: a workflow can be relaunched from its run id; unchanged steps return cached results, so
  iterating on the script is cheap.
- **Cost is real.** This revision spent ~4.5M agent tokens across ~61 subagents over ~14 minutes of
  background work. Right for a load-bearing design freeze; overkill for a quick edit.

## Honest limits

- **It is not an oracle.** The review surfaced candidates; ~3 of the 45 were wrong-but-plausible
  even *after* I'd seen them, and the adversarial pass (not I) caught most false ones. The judgment
  to accept/reject/reframe stayed with me, and the naming decisions stayed with you.
- **Verification bottoms out somewhere.** I verified the model with a workflow, and verified my
  application with a workflow — but I fixed the 2 residuals by hand and **self-checked** rather than
  spawning a third workflow to verify the fixes. At some point the recursion has to stop on judgment;
  the residual fixes were small and local.
- **Agents read what's on disk.** They could read the memo, the specs, the bash, and the live web —
  but the paddock is stored in JJK's gallops, not a flat file, so the agents could not read it; I
  verified the paddock myself. A tool's reach bounds what a workflow can check.

## The artifacts this process produced

- `memo-20260605-citizen-capability-model.md` — the mechanics (rewritten twice: a single-reviewer
  pass, then this ultracode pass).
- ₣BZ paddock (`rbk-14-citizen-model`) — the shape.
- `RBSHR-HorizonRoadmap.adoc` — two prose "citizen"→"holding" rewordings.
- `memo-20260527-operator-credential-models.md` — a deferral-honoring breadcrumb.
- This memo — the process record.
