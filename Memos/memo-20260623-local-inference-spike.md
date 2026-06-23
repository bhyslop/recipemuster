# Memo — Local Inference Spike: Fable-directed cold-probe of a weak local model

**Date:** 2026-06-23
**Interlocutors:** Brad Hyslop · Claude Opus 4.8 (1M)
**Paired heat:** `jjk-v4-1-local-inference-spike` (stabled)
**Status:** Iteration 1. A musing-stage record of a design conversation.

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
- The clean handoff between them is an **API boundary** (§6): an OpenAI- or
  Anthropic-shaped endpoint. Fable's corpus runs on one side; the rig is built up
  to and including that boundary. The model is a foreign actor at the Palisade;
  the API is the single membrane that contains it, which is what makes the rig
  model-agnostic by construction.

## 3. The ultimate substance — *delegated discernment*

This is the part Brad reached for with "I don't even know how we talk about
this." It is the load-bearing idea; everything else is plumbing in service of it.

### 3.1 It is the cold probe, generalized from words to models

This is the strongest available framing, and it grounds the concept in vocabulary
the project already owns. MCM's `mcm_cold_probe` tests whether a *name* is
fair-faced: present it cold to a fresh reader, the reader's stated expectation is
the verdict, **no advocacy by the minter is admitted**. Brad's idea is the same
epistemic method on a new substrate: present a *task* cold to a fresh weak model,
the model's actual behavior is the verdict, no advocacy (no benchmark marketing)
admitted. Same method, new substrate, a clean rhyme. We are not inventing an
evaluation method — we are generalizing one the project already trusts.

### 3.2 The corpus is compressed, transferable discernment

A cold probe run at scale against the weak model yields not a pass/fail score but
a **map of its competence boundary** in this codebase's specific terrain. The
map's defining property is that it *externalizes judgment so an agent other than
the mapmaker can navigate by it*. Fable spends the discernment labor once; the
corpus compresses that labor into something queryable by a router and reviewable
by Brad.

The answer key already exists: **retired JJK paces** are a ready-made,
codebase-grounded eval set — the docket *is* the prompt, the landed commit *is*
the known-good outcome. The probe need not invent tasks; the git history is the
corpus seed in Brad's own idiom.

### 3.3 Why a strong model can discern what Brad cannot

Not because it is wiser. Because it can be **systematic, patient, fine-grained,
and unflattering** in a way human bandwidth and human nature will not sustain. It
can run 200 probe trials; it can notice that the weak model fails specifically
when a task requires holding 3+ files in context, or that it botches the BCG
zipper idiom while writing clean standalone bash; it can refuse to advocate for
the model it is assessing. That is precisely the labor Brad's scarce attention
*should not* be spent on — and precisely what compresses into a trustworthy map.

### 3.4 Calibration vs. routing — the two acts (and Brad's tangent)

The undertaking decomposes into two distinct acts:

- **Calibration** — designing probes and interpreting surprising results to build
  and update the competence map. This needs intelligence and discernment (Fable).
  It runs *rarely*: when a new model drops, or a new region of the codebase comes
  into scope.
- **Routing / administration** — given the map, decide where a specific task goes,
  and mechanically run the probe protocol. This is *cheap*. Once the map exists,
  it can run on haiku, or even on **deterministic logic in the Rust orchestrator**
  (match task features against mapped regions).

The expensive discernment is front-loaded into the map; consulting the map is
cheap. This is the same shape as RBK's own discipline — *the discernment lives in
the spec; the code cites it by reference* (the rivet pattern). Here, the
discernment lives in the corpus-plus-map, and the router cites it. **The corpus is
the spec of the weak model's competence.**

This directly answers Brad's tangent: *yes*, administering cold probes can be
haiku or even the Rust orchestration — because the hard part (calibration) has
already been compiled into the map. Re-probing a newly-released model is then
cheap: Rust re-runs the corpus, and Fable/haiku only re-examines the deltas. That
keeps the system sustainable rather than a standing Fable expense.

### 3.5 Brad's own judgment moves up a level

The point of the whole thing is not to remove Brad from judgment — it is to move
his judgment *up a level*. He stops adjudicating object-level routing ("should
this task go to the weak model?") and starts adjudicating the meta ("are these
cold-probe discernments trustworthy?"). His scarce bandwidth is spent auditing the
map, not navigating it — the highest-leverage layer. This is the gradient-delivery
/ trot philosophy applied to delegation itself: spend the human on review of the
discernment, not on the mechanics.

It is also *why iterating this memo first is correct*. The discernment design is
the load-bearing part, and everything downstream inherits its quality. Garbage
probes produce confident-but-wrong routing — worse than no system at all. So the
corpus design is where care concentrates, and it cannot be rushed.

### 3.6 The vocabulary gap (mint deferred)

This wants a name and does not yet have one. It is not "delegation" (too coarse —
that is offloading *doing*). Candidate handles, offered as sketches to think with,
**not** mints: *delegated discernment*, *transferred judgment*, *competence
cartography / the fitness map*, *calibration vs. routing*. Per the project's
minting discipline, the right move is **not** to mint now — the concept is still
forming. Let the memo iterations stabilize it, then mint into an asterism with an
audible register. (Note: "assay" and "calibrant" already carry meanings in the
repo — APCK assay, theurge `rbtdrl_calibrant` — so they are likely disqualified by
the grep gate; the eventual mint must clear it.)

## 4. cerebro — the hardware (surveyed live, 2026-06-23)

A genuinely strong home-inference rig:

- **GPU:** NVIDIA RTX 5090, **32 GB VRAM**, 575 W. Driver 580, CUDA 13.0. Plus an
  Intel Arrow Lake iGPU.
- **CPU:** Intel Core Ultra 9 285K, 24 cores, 5.8 GHz boost.
- **RAM:** **188 GiB** (≈182 free). The sleeper spec — it is what makes the
  CPU/hybrid-MoE play viable (§5).
- **Disk:** 5.5 TB RAID0 (triple-striped, fast), 2.4 TB free. No redundancy by
  design; durable outputs are git-committed off it promptly, so redundancy is a
  non-concern. Fast striping helps *model load / swap* wall-clock during a probe
  sweep — not steady-state inference throughput.
- **OS:** Ubuntu 24.04.4, kernel 6.17. Direct access: `ssh cerebro`. Already a JJK
  fundus (foray target).

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
  (~6.6%); **Gemma 4 jumped to ~86.4%** (τ2-bench agentic tool use) — the
  "broken → works" leap that makes local agentic coding viable. Native function
  calling with MCP integration (relevant: JJK *is* an MCP server). Reportedly
  Apache 2.0 (verify before relying on it for anything distributable).
- **The CPU/hybrid-MoE play** (what the 188 GB unlocks): MoE coders with few active
  params (the A3B class: ~30–35B total, ~3B active/token) run well with llama.cpp's
  `--n-cpu-moe` / `-ncmoe` expert-pinning — hot experts in VRAM, cold expert pool in
  system RAM. A 12 GB GPU reportedly hits 58–62 tok/s on a 35B-A3B this way; cerebro
  could pin a far larger MoE. High upside, sharp edges (a documented 80B MoE ran
  ~5× slower than expected on CPU) — exactly what the probe should *measure*, not
  assume.
- **Honest ceiling:** frontier *hosted* agents top out ~57% on SWE-bench Pro; a
  local 30B lands well below on autonomous multi-step work. The probe's real
  question is "does the best local model clear the bar for the *mechanical* tier,"
  not "can it rival Fable."
- **Benchmark caution:** public leaderboards contradict each other (one source has
  Gemma 4 edging Qwen on SWE-bench; another has Qwen ~2× Gemma on agentic
  multi-file work). That disagreement *is* the argument for the cold probe — public
  numbers are advocacy; the retired-pace corpus is the verdict.

## 6. Tooling — the serving spine

- **Ollama: right to *start*, wrong to *standardize on*.** It is a llama.cpp wrapper
  tuned for ease (perfect for a five-minute prototype), but it abstracts away the
  `-ncmoe` granular offload control — the exact lever the 188 GB-RAM hybrid thesis
  depends on. Standardizing on it would quietly forfeit the capability that
  justifies *this* machine.
- **The durable spine: `llama-swap` in front of llama.cpp's `llama-server`.**
  llama-swap is almost eerily matched to the constraints here: out of the box it
  runs **one model at a time** (request another → it stops the current upstream,
  starts the right one); **per-model config in a file** (different flags/quant per
  model — a declarative control surface, written up front); headless, starts on
  boot; full `-ncmoe` control because it sits over llama-server; and it fronts
  OpenAI- *or Anthropic*-shaped endpoints. Brad's "one job at a time" lean is the
  *default* of the right tool, not a bolt-on.
- **Pragmatic path:** prototype on Ollama to prove the loop fast; build LIK's spine
  on llama-swap + llama-server as the permanent, control-up-front service.

## 7. Harness — path (b), chosen

Three-way harness fork, with (b) selected:
- **(a)** OpenHands headless via foray — non-Anthropic on *both* axes (model and
  tool); the "clean room."
- **(b) Claude Code + local model via the local endpoint** — *chosen.* Keep the
  harness Brad already lives in and the JJK MCP integration; swap only the brain.
  Cheapest path; people already run Gemma 4 inside Claude Code / Codex CLI via a
  local endpoint.
- **(c)** Run both as probe variants — the harness becomes part of what Fable
  measures.

Two cautions on (b): Claude-Code-on-a-non-Claude-model is likely a base-URL
override of **uncertain supportedness** — verify it is permitted before building on
it. And a framing wrinkle for Brad's stated intent ("a non-Anthropic coding
*tool*"): (b) is non-Anthropic *model* but Anthropic *tool*; (a) is non-Anthropic
on both axes. Which he means shifts the harness choice.

## 8. Interaction model — the single-slot worker

Brad's lean — *do not multitask local models; they are weak enough as is* — turns
cerebro into a **single-slot inference worker**: a named resource that does one job
at a time. This is a *class*, not a cerebro fact: any machine Brad fully controls
becomes the same abstraction.

JJK already has most of the parts: **foray** (`jjx_bind`/`relay`/`check`/`fetch`)
dispatches to a fundus, cerebro already is one, and JJK has a lock primitive (the
git-ref commit lock). The one missing primitive is a **claim/lease on the worker**:
a dispatch acquires it, concurrent dispatches (e.g. the Mac session plus another
officium) queue or are refused *loudly*, release on completion.

Lovely doubling: llama-swap enforces one-model at the *serving* layer; the JJK
claim enforces one-job at the *orchestration* layer — belt and suspenders that
reinforce rather than duplicate.

## 9. Orchestration — LIK as a Rust-first kit

Brad is encouraged by a project with a **full Rust orchestration body**, with
**tabtargets retained as CLI entry points** and a well-described logging / dispatch
process.

This is the established house pattern, not a leap:
- **theurge** (the `rbtd` Rust crate) proves it. The bash layer under it is a thin
  ~200-line shim — run codegen, `cargo build`, `exec` the binary, pass the folio
  through — and *all* logic is Rust; bash explicitly disclaims orchestration
  ownership.
- **`Tools/vok/` already exists** as a second Rust kit beside RBK using the same
  launcher pattern, so a third (LIK) is well-trodden.
- The dispatch chain is uniform indirection: thin tabtarget (`export
  BURD_LAUNCHER=…; exec z-launcher.sh`) → `z-launcher.sh` → `bul_launcher.sh` /
  `bud_dispatch.sh` → workbench → `buz_exec_lookup` (zipper colophon dispatch) →
  the Rust binary.
- theurge demonstrates the keystone LIK needs: the **reverse edge**
  (`rbtdri_invocation.rs`) — Rust driving the system through colophon-keyed
  tabtargets. And the logging discipline LIK should mirror is `rbtdrg_log.rs` (RCG
  output discipline: all emission via `rbtdrg_*!` macros, stdout reserved, format
  `[LEVEL] [file:line] message`).

Why this job *wants* Rust (beyond aesthetics): LIK's actual work — stateful service
lifecycle, a claim/lease state machine, HTTP to llama-swap, driving Claude Code as
a subprocess and capturing structured results — is precisely where bash fights back
and Rust glides. And the Rust orchestrator can itself **administer the cold probes**
(§3.4), giving the body real substance beyond babysitting llama-swap.

**Minimum pieces to stand up a Rust kit** (from theurge/vok): tabtarget stubs;
one launcher registration in `rbmm_moorings/rbml_launchers/`; a workbench (copy
`rbw_workbench.sh`); a zipper (`liz_*`, copy the `rbz_zipper.sh` const-projection);
a constants anchor; a crate skeleton (`Cargo.toml`, `lib.rs`, `main.rs`, generated
`*gc_consts.rs`, a log module, an invocation module, a platform module); and a
codegen-before-build step.

**The convergence (the strongest synthesis of the conversation):** control-up-front
wants a deliberate surface; one-job-at-a-time wants a claim/lease state machine; the
Rust pull wants typed orchestration. All three independently name the *same*
artifact — **a thin Rust orchestrator over a single-slot worker, with llama-swap as
the service spine.** Brad did not choose three features; he triangulated one design.

## 10. Sequencing (recommended)

One discipline held against enthusiasm: **do not scaffold the Rust kit until path
(b) is empirically real.** The whole edifice rests on "Claude-Code-on-local-Gemma-4
is good enough to be worth orchestrating." If that loop is janky, the scaffold is
sand.

1. **Prove the loop cheaply** — Ollama + Gemma 4 + Claude Code, by hand on cerebro.
   De-risks (b). The single unconfirmed assumption everything rests on.
2. If it holds — **build LIK's spine in Rust** around llama-swap (the permanent
   service that earns up-front control).
3. Then the **JJK single-slot dispatch / claim layer** (the interaction model).
4. **Fable's corpus** runs against this whenever it lands.

## 11. Open questions (for memo iteration)

- **Vocabulary** (§3.6): what do we *call* delegated discernment? Mint deferred
  until the concept stabilizes.
- **Corpus structure**: how is the cold-probe corpus shaped, and what does "would
  this have passed" mean operationally? (Fable's domain, but the rig must serve it.)
- **Routing mechanics**: once the competence map exists, how does a router consult
  it — haiku, deterministic Rust, or Fable? What is the map's actual form?
- **Claim-model timing**: design the claim/lease model now (it shapes the Rust
  types), or strictly after the loop + spine?
- **CPU/hybrid scope**: tune the `-ncmoe` hybrid path as a first-class probe
  candidate, or stand up only the clean GPU-resident 30B baseline first?
- **Harness axis** (§7): does "non-Anthropic coding tool" mean non-Anthropic *model*
  (b) or non-Anthropic on *both* axes (a)?
- **Supportedness**: is pointing Claude Code at a non-Claude local endpoint a
  permitted configuration?

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
