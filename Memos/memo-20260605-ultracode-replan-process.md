# Memo — How ultracode revised the Lode remainder plan (₢BHAAB)

*2026-06-05. Written to educate the operator on the multi-agent process that produced the ₢BHAAB replan recommendations and the 10-pace slate. Plain-language walkthrough, not a spec. This memo was itself revised after an honest review panel caught it overselling — that story is at the end, because it is the best lesson in here.*

## How to read this

You do not need to know the Lode domain to follow this memo. Wherever a domain word appears — `bole`, `reliquary`, `touchmark`, `ensconce`, `rbi_es` — you can read it as "one of the subsystems being investigated." What matters here is the *process* wrapped around those words, not the words themselves. The two or three places where a domain detail actually carries a teaching point are glossed inline.

## What this was applied to

On 2026-06-05 the operator asked for recommendations on four open planning calls for heat ₣BH (the Lode capture remainder), then said "slate now." Rather than answer those four calls from memory or from the design doc (the "paddock") alone, the work ran an **ultracode workflow** — a script that fanned the question out across ten subagents — to ground every recommendation in the *actual landed code* before any pace was written. This memo explains what that process was, why it was shaped that way, what it actually bought, and where it fell short.

## What "ultracode" changed

Ultracode is a session mode. With it on, the default for a substantial task flips from "one agent (me) does it inline" to "author a workflow that spreads the task across many subagents, and accept high token cost for the most thorough answer."

The machinery is the **Workflow tool**, and it is worth picturing concretely, because every later mention of "agent" rests on it. I write a short JavaScript script *per task* — it is not a standing pipeline. The script loops, and on each pass it launches a fresh Claude instance — a *subagent* — handing it a focused prompt: "read these files, answer this one question, return your answer in this exact shape." That subagent has its own clean context, does its reading, returns a structured result, and disappears. The script collects those results and hands them back to me. I do not read the files; I design the investigation and make the final judgment on what comes back.

## The shape of the run

The operator's four calls did not map one-to-one onto agents. I re-cut them into **five investigation areas** — the four calls plus the "scaffold residue" (a kind-registration question that fed two of the calls), with the provenance-and-docs call deliberately given one agent covering both its halves. Each area got its own reader, and each reader's findings were immediately handed to an independent skeptic to re-check:

```
            INVESTIGATE                 VERIFY
  augur      ──reader──►  findings  ──►  skeptic re-reads the cited files
  scaffold   ──reader──►  findings  ──►  skeptic re-reads the cited files
  vertical   ──reader──►  findings  ──►  skeptic re-reads the cited files
  cutover    ──reader──►  findings  ──►  skeptic re-reads the cited files
  provdocs   ──reader──►  findings  ──►  skeptic re-reads the cited files
```

Ten agents: five to find the facts, five to attack them. The workflow itself did **no synthesis** — it returned the ten raw results as a list. Turning that list into recommendations, and then into a 10-pace plan, happened afterward in my head (see "How the findings became a plan").

## The five design choices, and why each one

**1. Fan out by question, not by file.** The areas are disjoint subsystems — read verbs, capture bodies, cutover points, provenance/docs. One agent holding all of that reads shallowly across everything; five agents each read one area deeply. Coverage comes from the breadth of agents, not the depth of any single context.

**2. Force structured answers.** Each reader had to return a fixed shape: a list of *claims*, each with *evidence* (a file-and-line citation) and a *confidence* level, plus one *recommendation input* — the agent's own one-line suggestion for what the final call should be. Two payoffs. First, it makes the agent show its work — no claim without a citation. Second, the fixed shape can be machine-validated, so an agent that returns malformed or incomplete structure is automatically re-run; that validate-and-retry is what makes a ten-agent fan-out reliable instead of fragile.

**3. Verify adversarially.** Every reader's findings went to a second agent whose only job was to re-read the cited files and try to break the claims. The skeptic did not trust the summary — it opened the files itself. Why bother? Because self-reported confidence turned out to be a poor signal: the two claims the skeptics had to flag on this run (a trust-grade claim, a fetch-mechanism guess) were *both* stamped "high confidence" by their authors. You cannot rely on an agent's own confidence, so a second agent has to re-derive it. That is the whole argument for the verify stage.

**4. Pipeline, don't barrier.** The obvious way — a "barrier" — is two phases: run all five investigations, wait for the slowest, *then* start verifying. That wastes time: the area that finished first sits idle waiting for the laggard. Instead each area ran as its own investigate-then-verify chain, side by side, with verification starting the instant that area's investigation landed. Total time is the longest single chain, not (all investigations) + (all verifications).

**5. Keep the judgment — but be honest about how much.** This is the choice I most want to state accurately, because it is easy to flatter myself here. The agents did **not** just produce facts. The structured shape *required* each one to propose a recommendation ("group these two," "split that one," "do this one early"). So the shape of the plan — group tool+reliquary, split podvm, slate the bole cutover early — was *proposed by the subagents*. My contribution was real but narrower than "I decided": I selected among their proposals, the skeptics stress-tested them, and I reconciled the result against context the agents did not have (see the next section). The honest summary is "machines gather and propose; the human adjudicates and reconciles against private context" — not "machines gather, human decides."

## What verification actually bought — the honest scorecard

It would be easy to write that the skeptics "rescued the plan." They did not. Across all five areas they reversed **zero** recommendations — every one stood. Of roughly forty verdicts, exactly **one** came back false (an over-confident trust-grade claim). The rest were citation slips — a line number pointing at the right content in the wrong file — and prose nuances explicitly marked "does not change the recommendation."

So the trade was: a large token spend bought **confidence**, plus one corrected over-claim and a handful of fixed citations. On a throwaway question that would be waste. On the decision that sets the next ten paces of a heat, paying for confidence that the plan rests on true facts is defensible. But the reader should see it plainly: verification *validated* this plan; it did not save it from error.

The one real catch was worth having. A reader claimed the new capture path emits a "byte-equivalent" address to the old one. The skeptic re-read both and found only the fingerprint *token* is identical — the full addresses differ in shape (`rbi_es/<fp>:<fp>` vs `rbi_ld/<stamp>:<fp>`; same `<fp>`, different package root and package-vs-tag structure). In plain terms: the old path and the new path store the image at differently-shaped addresses, so the new address cannot be reconstructed from the inputs the way the old one could. That correction changed how I wrote the cutover docket.

## The most instructive failure — a context gap that recursed twice

Choice 5 ("keep the judgment, reconcile against private context") earned its keep on exactly one finding, and the story is the most useful thing in this memo because it shows the *limit* of the whole approach.

A skeptic flagged the per-kind verb names (`fetter`, `conclave`, `underpin`, `immure`) as fabricated — not present in the code. **The skeptic was right about the code:** those verbs are not implemented anywhere yet. But "not in the code" is not "fabricated." They are the **paddock's settled vocabulary** — the ₣BH paddock's Vocabulary section names them as the planned verbs for each kind. I had even seeded them into the investigators' shared prompt myself, *because* they are real planned names.

So what actually went wrong was a bug in *my* workflow authoring: the investigators' prompt carried the verb vocabulary, but the verifier's prompt did not. The skeptic, re-reading only code with no access to the design doc, correctly reported "absent from code" and reasonably mislabeled it "fabricated." I caught the mislabel only because I held the paddock context that neither the verifier's prompt nor the code contained.

Here is the part that makes it land. **The review panel for this very memo — three more agents, also not given the paddock — independently flagged the same verbs as "asserted without evidence."** The exact context gap repeated one level up. The lesson is not a one-off: *a verifier is only as good as the context it is given, and code-against-code checking cannot validate a decision that depends on a design doc the agents never received.* The human in the loop is not optional decoration; on this run, that seat was load-bearing, and the obvious mitigation the run did **not** take is plainly: feed the paddock into the agents' context next time.

## How the findings became a plan

Worth stating because the workflow does not do it: synthesis was a manual step. I read the ten verified results, mapped each area's recommendation back onto the operator's four original calls, made the calls (with the one paddock override above), and only then wrote the ten dockets. The workflow is purely investigate-plus-verify; the join from "four questions" to "ten paces" lives in the slate itself (₢BHAAG–₢BHAAP), not in the script.

## What it cost

Per the harness usage telemetry (which is reported to me at completion, not stored in the results file): ten agents, ~937,000 subagent tokens, ~200 tool calls, ~8 minutes of wall-clock. That is a lot of tokens for four planning questions. The trade is deliberate — token cost bought coverage (five subsystems read in depth at once) and the confidence accounting above. Worth it here; wasteful for a quick lookup.

## When this is the right tool, and when it isn't

It fit because the task was **wide** (five disjoint areas), **fact-heavy** (the right answer depended on the real code, not on taste), and **consequential** (it set a multi-pace plan). Those three together justify the fan-out-and-verify shape.

It is the wrong tool for a narrow lookup, for a pure judgment call with no facts to gather, or — the limit this run actually hit — when the decision depends on private design context the agents were not given. This shape works cleanly when the answer lives in the code the agents can read; it degrades exactly to the degree the answer also lives in a design doc they cannot. Half-met here, which is why the human override was load-bearing rather than optional. The fix is cheap and worth remembering: when the judgment needs the paddock, put the paddock in the prompt — for the verifiers too, not just the readers.
