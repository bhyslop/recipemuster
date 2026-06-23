# Memo — Local Inference Spike: Fable-directed cold-probe of a weak local model

**Date:** 2026-06-23
**Interlocutors:** Brad Hyslop · Claude Opus 4.8 (1M)
**Paired heat:** `jjk-v4-1-local-inference-spike` (stabled)
**Status:** Iteration 2. A musing-stage record of an ongoing design conversation.

> **Provenance, not authority.** Per the memo doctrine: a memo is a nudge, never
> load-bearing. Nothing here is cinched. Durable facts earn a spec home only once
> they survive iteration. The paired heat's paddock is deliberately thin and
> "knows" only this memo; we will not build the paddock out until this memo has
> been iterated several times. The shape is not yet settled enough to cinch.

---

## 1. The premise — the bottleneck has migrated

Brad's active supposition, which is the real engine of this whole undertaking:

> As the models get smarter, the real limit in the chain is *my* ability to
> process, understand, and decide issues.

This is the classic systems-constraint result: optimize one stage and the binding
constraint moves to the next. Model raw capability is ceasing to be the limit;
the operator's bandwidth to comprehend and adjudicate is becoming it.

Most tooling offloads the *old* bottleneck — the doing of work. The fascination
here is offloading the *new* bottleneck — the **discernment of what work goes
where**. Concretely: can a strong model (Fable, while it is cheap during its
subsidy window) make fitness judgments about a weak local model that Brad himself
could not make — or would not make at the granularity, patience, and lack of
self-flattery required — and then *route work to the weak model accordingly*?

(The subsidy premise is Brad's; Claude has no independent knowledge of Fable's
pricing or the subsidy window.)

## 2. What the undertaking is — two stacked ideas, and the division of labor

1. **The probe.** A strong model drives an evaluation harness that *cold-probes*
   candidate local models for real coding competence on cerebro's GPU. The output
   is **knowledge**, not infrastructure: where does the best local model land, and
   on which task shapes.
2. **The integration.** Once a viable local model exists, wire it in as a
   non-Anthropic *actor* in the operator's workflow (Job Jockey).

These are sequential; the probe is the interesting one.

**Division of labor (cinch candidate, not yet cinched):**
- **Fable** authors the probe corpus and renders judgment (calibration — see §3).
- **This repo / the Rust orchestrator** builds the *rig* the corpus runs against.
- The clean handoff between them is an **API boundary** (§8): an OpenAI- or
  Anthropic-shaped endpoint. Fable's corpus runs on one side; the rig is built up
  to and including that boundary. The model is a foreign actor at the Palisade;
  the API is the single membrane that contains it, which is what makes the rig
  model-agnostic by construction.

## 3. The ultimate substance — *delegated discernment*

This is the load-bearing idea; everything else is plumbing in service of it.

### 3.1 It is the cold probe, generalized from words to models

MCM's `mcm_cold_probe` tests whether a *name* is fair-faced: present it cold to a
fresh reader, the reader's stated expectation is the verdict, **no advocacy by the
minter is admitted**. Brad's idea is the same epistemic method on a new substrate:
present a *task* cold to a fresh weak model, the model's actual behavior is the
verdict, no advocacy (no benchmark marketing) admitted. Same method, new
substrate, a clean rhyme. We are not inventing an evaluation method — we are
generalizing one the project already trusts.

### 3.2 The corpus is compressed, transferable discernment

A cold probe run at scale yields not a pass/fail score but a **map of the weak
model's competence boundary** in this codebase's specific terrain. The map's
defining property is that it *externalizes judgment so an agent other than the
mapmaker can navigate by it*. Fable spends the discernment labor once; the corpus
compresses it into something queryable by a router and reviewable by Brad. The
answer key already exists: **retired JJK paces** are a ready-made, codebase-grounded
eval set — the docket *is* the prompt, the landed commit *is* the known-good
outcome.

### 3.3 Why a strong model can discern what Brad cannot

Not because it is wiser — because it can be **systematic, patient, fine-grained,
and unflattering** in a way human bandwidth and human nature will not sustain. It
can run 200 trials, notice the model fails specifically on 3+-file context or the
BCG zipper idiom while writing clean standalone bash, and refuse to advocate for
the model it assesses. That is precisely the labor Brad's scarce attention should
not be spent on — and precisely what compresses into a trustworthy map.

### 3.4 Calibration vs. routing — the two acts

- **Calibration** — designing probes and interpreting surprising results to build
  and update the map. Needs intelligence (Fable). Runs *rarely* (new model, new
  codebase region).
- **Routing / administration** — given the map, decide where a task goes, and
  mechanically run the probe protocol. *Cheap.* Can run on a middle model, or even
  deterministic Rust.

The expensive discernment is front-loaded into the map; consulting it is cheap.
Same shape as RBK's rivet pattern — discernment lives in the spec, code cites it.
Here, **the corpus is the spec of the weak model's competence.**

### 3.5 Brad's own judgment moves up a level

He stops adjudicating object-level routing and starts adjudicating the meta ("are
these discernments trustworthy?"). His scarce bandwidth is spent auditing the map,
not navigating it. This is *why iterating this memo first is correct* — the
discernment design is the load-bearing part, and everything downstream inherits
its quality. Garbage probes produce confident-but-wrong routing, worse than no
system at all.

### 3.6 The vocabulary gap (mint deferred)

Candidate handles, offered as sketches **not** mints: *delegated discernment*,
*transferred judgment*, *competence cartography / the fitness map*. Mint deferred
until the concept stabilizes, then into an asterism. ("Assay" and "calibrant"
already carry meanings in the repo, so they are likely disqualified by the grep
gate; the eventual mint must clear it.)

### 3.7 The focused first cut — an agentic-proclivity assay

The initiative's first concrete deliverable, scoped tightly: have the best model
draft an assay that probes the local model's **proclivity to take on an agentic
action** — not single-shot code generation (well-covered elsewhere, and not where
the local model's fate is decided), but the multi-step loop: navigate, edit, run,
observe, iterate. This is the axis where weak models crater and the one that
foray-dispatched use actually demands, so it carries the signal. Stimuli are drawn
from real retired paces (docket as goal, landed commit as a reference point).
Scored after administration.

Crucial framing: the reference is **not assumed to be the ceiling.** The local
model may produce something better — so scoring is not diff-to-reference (§3.8).

### 3.8 Assay design

**Four roles, tiered by how mechanical they are** (spend model quality inverse to
mechanical-ness — JJK's haiku/sonnet/opus philosophy applied to the pipeline):

- **Doer** — the weak local model (the subject).
- **Administrator** — a *middle model* (or the Rust harness). Reconstructs the
  environment, dispatches the stimulus, captures the trajectory. *Decided.*
- **Scorer** — the *best* model. Renders the verdict: goal achieved? trajectory
  quality? beat the reference? *Decided (operator):* best model scores, because
  scoring *is* the discernment and a cheap verdict poisons the map.
- **Auditor** — the operator. Meta-discernment: are the scorer's verdicts
  trustworthy? Spot-checks the high-stakes ones.

**Grading: oracle-first, reference-as-comparator.** Because the reference is not
the ceiling, the primary metric is *goal-achievement via oracle* (reference-
independent), not diff-to-reference. The reference demotes to three honest roles:
difficulty calibrant (a human achieved it, so it is achievable), test source (its
tests are the hard oracle), and quality comparator (better/worse/equal — a judge
call). Oracles don't compete with the best-model scorer; they **brief** it —
pre-digested evidence (compiled? tests pass? `Done when` met?) so the scorer spends
cognition on judgment, not on re-deriving facts.

A codebase gift: retired-pace dockets carry a `## Done when` heading — the success
rubric is partly *pre-written* by the JJK docket discipline. `Done when` + the
landed commit's tests + the diff give a strong oracle-plus-rubric without inventing
one.

**Proclivity lives in the trajectory.** A weak model's agentic failure is usually
not a wrong final answer but *how it engages the loop* — writes one file and
declares done, never runs the tests, gives up on the first red, thrashes. So the
assay scores the *action trace*, not just the terminal diff. Discipline that falls
out: the administrator must **capture, never characterize** — a faithful, complete,
un-editorialized trace to the scorer; any summary it injects is an advocacy leak
into the verdict.

**The "beats the reference" verdict** is real and valuable (it flags genuine local
capability and gives a mild review signal on your own history) but the most
dangerous to score — weak models excel at plausible-but-wrong. Those verdicts get
the most adversarial verification: default to "did not beat it," make the model's
solution survive a skeptic, not charm one.

**Pin the rubric.** A model-scorer over a *standing* assay has a subtle failure:
the scorer drifts across resets (it rhymes, it doesn't continue). If it freelances
fresh criteria each cycle, the drift ledger conflates *model-competence drift* with
*scorer-criteria drift* and the longitudinal signal rots. Fix: the scoring rubric
lives in the commission (the artifact), not the model's head. "Best model scores"
means "best model applies a *specified, stable* rubric with full discernment,"
never "invents a verdict."

**Competence coordinates** (the map's axes): domain (rust/adoc/bash) ×
sub-capability (the useful grain) × **task-shape** (single-shot → multi-step
agentic). The task-shape axis is load-bearing: a corpus that is all single-shot
paints a falsely green map; the assay must ladder it on purpose, since that is
where the boundary actually is. Keep a few probes that separate *agentic ability*
from *idiom-fluency*, so you can tell which is the limiter in this idiom-heavy
codebase.

**Provenance + the honesty problem.** Seed from retired paces (ready-made, in your
idiom). But add held-out / freshly-synthesized probes, because public-ish dockets
may be in training data — same reason SWE-bench Pro exists. Synthesizing held-out
probes is part of what the best model does that the oracle layer cannot.

**Standing mechanics.** Oracle/structural tiers re-run cheaply on cadence as the
drift tripwire; the best-model scorer re-fires on what matters — initial
calibration, a new model/quant, or a coordinate the tripwire flags as changed.
Verdicts accrete into a ledger keyed by (probe, model-version, date); the map is a
projection over the latest, drift is a diff across dates.

### 3.9 The governing constraint — a thin crust, not a cathedral

The system must stay *cheap to maintain*, because the genuinely scarce resource is
the operator's tending-attention. An assay that demands ever-growing human
curation recreates the very bottleneck it was built to relieve — the bottleneck
wearing a crown. So:

- **Readiness, not coverage.** A thin crust over the project-shapes you actually
  drop into beats an exhaustive map of terrain you never enter. Coverage follows
  use (the same instinct as discovering tabtargets from friction). Keep a clean
  ring around the camp; don't map the wilderness.
- **The method is the constant, the content evolves.** What stays fixed is the
  cold-probe discipline (present cold, behavior is the verdict, no advocacy) — now
  aimed at words (MCM), models (here), and provisionally specs. The probes,
  references, and rubrics all move, and the tests must get *fiercer and
  wilder-shaped* as the bottled intelligences improve. They stay discriminating
  precisely because they are rooted in this codebase's idiosyncratic terrain —
  public benchmarks saturate and die; a local assay stays sharp.
- **The owner's enduring role is convergence-sensing.** As capability rises the
  operator's judgment distills upward — off doing, off routing, off scoring —
  toward sensing whether ever-larger projects are *converging or fragmenting*, and
  knowing when a thing must be split to stay coherent. That sense is tacit, grown
  by reps, undelegatable, and it is the anti-brittleness mechanism: it turns
  would-be shatter into re-convergence at a coarser altitude. The human stays in
  the loop not for lack of budget to leave, but because that sense is load-bearing
  and no bottled mind can hold it.

## 4. cerebro — the hardware (surveyed live, 2026-06-23)

A genuinely strong home-inference rig:

- **GPU:** NVIDIA RTX 5090, **32 GB VRAM**, 575 W. Driver 580, CUDA 13.0. Plus an
  Intel Arrow Lake iGPU.
- **CPU:** Intel Core Ultra 9 285K, 24 cores, 5.8 GHz boost.
- **RAM:** **188 GiB** (≈182 free). The sleeper spec — it is what makes the
  CPU/hybrid-MoE play viable (§7).
- **Disk:** 5.5 TB RAID0 (triple-striped, fast), 2.4 TB free. No redundancy by
  design; durable outputs are git-committed off it promptly. Fast striping helps
  *model load / swap* wall-clock during a probe sweep — not steady-state
  inference throughput.
- **OS:** Ubuntu 24.04.4, kernel 6.17. `ssh cerebro`. Already a JJK fundus.

The 32 GB / 188 GB split defines the playing field: a **32B-class model at 4-bit
(~16–19 GB)** is the GPU-resident sweet spot; a 70B at 4-bit (~35 GB) spills into
system RAM and runs in a slow regime. The big RAM buys *reach* (large hybrid MoE),
not *speed*.

## 5. The model landscape (mid-2026, via web search)

- **Qwen3-Coder-30B (Qwen3-Coder-Flash), Q4** — community top pick for pure
  coding; fits 32 GB cleanly with ~13 GB for context; ~40–55 tok/s. The control
  specimen.
- **Qwen3.6-27B dense** — strong general reasoning + coding; ~77% SWE-bench
  Verified; very large context (262K native).
- **Gemma 4** (released 2026-04-02; built on Gemini 3 research) — the **co-baseline
  and the more thematically apt one** (Google-native + MCP-native, in a GCP-native
  project). Variants that fit: 31B dense (GPU-resident) and 26B MoE (hybrid
  candidate). The decisive update: Gemma 3 tool-calling was effectively broken
  (~6.6%); **Gemma 4 jumped to ~86.4%** (τ2-bench). Native function calling with
  MCP integration (JJK *is* an MCP server). Reportedly Apache 2.0 (verify before
  relying on it for anything distributable).
- **The CPU/hybrid-MoE play** (what the 188 GB unlocks): MoE coders with few active
  params (A3B class) run well with llama.cpp's `--n-cpu-moe` / `-ncmoe`
  expert-pinning — hot experts in VRAM, cold pool in system RAM. High upside, sharp
  edges (a documented 80B MoE ran ~5× slower than expected on CPU) — measure, don't
  assume.
- **Honest ceiling:** frontier *hosted* agents top out ~57% on SWE-bench Pro; a
  local 30B lands well below on autonomous multi-step work. The probe's real
  question is "does the best local model clear the *mechanical* tier," not "rival
  Fable."
- **Benchmark caution:** public leaderboards contradict each other — which *is* the
  argument for the cold probe.

## 6. Tooling — the serving spine

- **Ollama: right to *start*, wrong to *standardize on*.** Easiest for a five-minute
  prototype, but it abstracts away the `-ncmoe` control the 188 GB-RAM hybrid thesis
  depends on.
- **The durable spine: `llama-swap` in front of llama.cpp's `llama-server`.** Out of
  the box it runs **one model at a time** (matches the single-job lean), per-model
  config in a file (a declarative control surface), headless/start-on-boot, full
  `-ncmoe` control, and fronts OpenAI- *or Anthropic*-shaped endpoints.
- **Pragmatic path:** prototype on Ollama; build the spine on llama-swap +
  llama-server.

## 7. Harness — path (b), chosen

- **(a)** OpenHands headless via foray — non-Anthropic on *both* axes.
- **(b) Claude Code + local model via the local endpoint** — *chosen.* Keep the
  harness Brad lives in and the JJK MCP integration; swap only the brain.
- **(c)** Run both as probe variants.

Cautions: Claude-Code-on-a-non-Claude-model is likely a base-URL override of
**uncertain supportedness** — verify it is permitted. Framing wrinkle: (b) is
non-Anthropic *model* but Anthropic *tool*; (a) is non-Anthropic on both axes.

## 8. Interaction model — the single-slot worker

Brad's lean — *do not multitask local models* — turns cerebro into a **single-slot
inference worker**: a named resource that does one job at a time. A *class*, not a
cerebro fact. JJK already has foray (bind/relay/check/fetch) and a lock primitive;
the one missing piece is a **claim/lease on the worker** (acquire on dispatch,
queue/refuse concurrent dispatches loudly, release on completion). Lovely doubling:
llama-swap enforces one-model at the *serving* layer; the JJK claim enforces one-job
at the *orchestration* layer.

## 9. Orchestration — LIK as a Rust-first kit

Full Rust orchestration body, with **tabtargets retained as CLI entry points** and a
well-described logging/dispatch process. This is the established house pattern:
**theurge** (`rbtd`) proves it (the bash layer is a thin ~200-line shim; all logic is
Rust), and **`Tools/vok/` already exists** as a second Rust kit beside RBK. theurge
demonstrates the keystone — the **reverse edge** (`rbtdri_invocation.rs`, Rust driving
the system through colophon-keyed tabtargets) — and the logging discipline to mirror
(`rbtdrg_log.rs`). LIK's job (stateful service lifecycle, claim/lease state machine,
HTTP to llama-swap, driving Claude Code as a subprocess, **administering the cold
probes**) is exactly where bash fights and Rust glides.

**The convergence:** control-up-front wants a deliberate surface; one-job-at-a-time
wants a claim/lease state machine; the Rust pull wants typed orchestration. All three
name the *same* artifact — a thin Rust orchestrator over a single-slot worker, with
llama-swap as the service spine.

## 10. Sequencing (recommended)

Discipline held against enthusiasm: **do not scaffold the Rust kit until path (b) is
empirically real.**

1. **Prove the loop cheaply** — Ollama + Gemma 4 + Claude Code, by hand on cerebro.
2. **Build LIK's spine in Rust** around llama-swap.
3. **The JJK single-slot dispatch / claim layer.**
4. **Fable's corpus** runs against this whenever it lands.

## 11. Open questions (for memo iteration)

**Decided since iteration 1:** the assay's first focus is *agentic proclivity* (§3.7);
scoring is *oracle-first, reference-as-comparator* with the reference explicitly not
the ceiling (§3.8); the *scorer is the best model*, the *administrator a middle model
or Rust* (§3.8); the governing constraint is a *thin, cheap-to-maintain crust* (§3.9).

Still open:
- **Vocabulary** (§3.6): what do we *call* delegated discernment? Mint deferred.
- **Corpus structure**: how is the agentic corpus shaped; what does "achieved the
  goal" mean operationally per pace? (Fable's domain; the rig must serve it.)
- **Environment reconstruction**: per-probe isolation (a worktree/clone at the pace's
  parent commit) so the doer's actions don't pollute — connects to the git-worktrees
  direction.
- **Oracle/judge line per domain** (§3.8): where it is drawn *is* the Fable-dependency
  budget; harder to keep oracle-side for agentic/trajectory/beats-reference.
- **Drift-tripwire cadence**: how often the cheap tier re-runs; what flips a coordinate
  to re-score.
- **Claim-model timing**: design the claim/lease now (shapes the Rust types) or after
  the loop + spine?
- **CPU/hybrid scope**: tune `-ncmoe` as a first-class probe candidate, or GPU-resident
  30B baseline first?
- **Supportedness**: is pointing Claude Code at a non-Claude local endpoint permitted?

## 12. Sources

Model landscape:
- https://docs.bswen.com/blog/2026-03-17-best-local-llm-rtx-5090-coding/
- https://apxml.com/posts/best-local-llms-for-every-nvidia-rtx-50-series-gpu
- https://unsloth.ai/docs/models/tutorials/qwen3-coder-how-to-run-locally
- https://knightli.com/en/2026/05/26/rtx-3060-llama-cpp-n-cpu-moe-local-35b/
- https://gist.github.com/DocShotgun/a02a4c0c0a57e43ff4f038b46ca66ae0
- https://frontman.sh/blog/best-open-source-ai-coding-tools-2026/
- https://www.morphllm.com/best-ai-coding-agents-2026
- https://insiderllm.com/guides/best-local-coding-models-2026/

Gemma 4:
- https://willitrunai.com/blog/qwen-3-6-vs-gemma-4
- https://www.verdent.ai/guides/gemma-4-coding-agents
- https://www.kdnuggets.com/local-agentic-programming-on-the-cheap-claude-code-ollama-gemma4
- https://medium.com/google-cloud/i-ran-gemma-4-as-a-local-model-in-codex-cli-7fda754dc0d4
- https://ai.google.dev/gemma/docs/core/model_card_4

Serving tooling:
- https://developers.redhat.com/articles/2026/06/15/llamacpp-vs-vllm-choosing-right-local-llm-inference-engine
- https://github.com/mostlygeek/llama-swap
- https://www.nijho.lt/post/llama-nixos/
- https://daily.dev/blog/running-llms-locally-ollama-llama-cpp-self-hosted-ai-developers/
- https://dasroot.net/posts/2026/05/mastering-multi-model-stacks-llama-swap/
