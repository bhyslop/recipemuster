# Delete-Architecture Regression — A Spec-Hygiene Anchor

## Status
Forensic record + process anchor (2026-06-09). Authored against heat ₣BH.

## Purpose
Two purposes, one subordinate to the other.

1. **Record the fault.** Image deletion in Recipe Bottle runs *locally from the
   workstation* (the Director mints an OAuth token and fires `DELETE` straight at
   GAR via curl), when the intended architecture is *cloud-dispatched* — the
   workstation submits a job, the cloud executes the deletion, completion returns
   to the workstation. The correct shape existed in the GitHub Actions era and
   was lost at the Google Cloud Build pivot. It has been wrong for ~10 months.

2. **Use the fault as a process anchor.** This is not a coding bug that a test
   would have caught. It is a *specification-hygiene* failure: the RBS0 spec set
   was written (or aligned) to match the already-regressed code, so the spec
   blessed the regression as intended design and could never flag it. That
   failure mode is the durable lesson, and it implicates the whole spec stack —
   MCM, AXLA, RBS0, BUS0, JJS0. The concrete fault below is the worked example;
   the improvement hypotheses are the point.

## The fault, stated once
Mutating GAR operations split into two architectural classes, and nothing in the
spec corpus names the split:

- **Build-class** (conjure, inscribe, ensconce, conclave, …) — dispatched to
  Cloud Build, executed in the cloud, awaited from the workstation.
- **Delete-class** (abjure, jettison) — executed *locally* from the workstation,
  direct REST `DELETE` against GAR.

The operator's intent is that delete-class should be build-class. It is not, and
has not been since 2025-08-14.

## Forensic timeline

| Date | Commit | Event |
|------|--------|-------|
| GitHub Actions era → 2025-08-07 | `353709c7` (def) | **Right way.** `rbim_delete` → `rbgh_delete_workflow` → `rbga_dispatch "delete_image"` (dispatch workflow to cloud) → `rbga_wait_completion` (poll) → pull deletion history → verify. Now sits under `Tools/ABANDONED-github/rbgh_GithubHost.sh`. |
| **2025-08-07** | `e1a9f16f8` | **The pivot.** "ABOUT TO REDO USING GOOGLE CLOUD BUILD… Removed delete since unnecessary anymore." The cloud-dispatched delete deleted outright (141 lines from `rbim_ImageManagement.sh`). Brief gap with no delete at all. |
| **2025-08-14** | `0b6662676` | **The regression lands.** `rbf_delete()` reappears in `rbf_Foundry.sh`, local from line one: `rbgo_get_token_capture` then a bare `curl … DELETE`. |
| 2026-01-28 | `17511ad8b` | RBSAA (abjure) spec created — codifies the local REST DELETE as the spec. |
| 2026-02-06 | `dbdb6803c` | `rbf_abjure()` added — local enumerate + DELETE loop. |
| 2026-02-09 | `bd344f92e`, `525b7fac` | `rbf_delete` rewritten for locator input; **the alignment commit pulls already-implemented behavior into the spec.** (Origin of the unauthorized `--force` freelance docket text traced and later scrubbed in `3eda4229d`.) |
| 2026-03-31 / 04-01 | `c2a9df49f`, `388dd4544` | Renamed (`delete`→`jettison`) and extracted into `rbfl_FoundryLedger.sh`. Mechanism unchanged. |
| 2026-06-04 | `536fe58f` | Decomposed into today's `Tools/rbk/rbfld_Delete.sh`. Still local. |

## The two implementations, side by side

**Right way** — `Tools/ABANDONED-github/rbgh_GithubHost.sh`:
```bash
rbga_dispatch "${RBRR_REGISTRY_OWNER}" "${RBRR_REGISTRY_NAME}" \
              "delete_image" '{"fqin": "'${z_fqin}'"}'   # dispatch to cloud
rbga_wait_completion "${RBRR_REGISTRY_OWNER}" "${RBRR_REGISTRY_NAME}"  # await
```

**Wrong way (current)** — `Tools/rbk/rbfld_Delete.sh`, both `rbfl_jettison` and `rbfl_abjure`:
```bash
z_token=$(rbgo_get_token_capture "${RBDC_DIRECTOR_RBRA_FILE}")   # local token
rbuh_request "DELETE" "${ZRBFC_REGISTRY_API_BASE}/${z_pkg_path}/manifests/${z_tag}" ...  # local curl
```

(The operator initially described this as "local docker engaging GAR." The engine
is curl/REST, not the docker daemon — but the substance, *local instead of
cloud-dispatched*, is exactly right.)

## Root cause: the spec followed the code

The binding observation is not that the code was wrong — code regresses, that is
ordinary. It is that **the specification was produced by transcribing the
implementation.** The abjure/jettison specs were authored months after the local
delete already existed, and the 2026-02-09 alignment commit literally pulled
implemented behavior into the spec text. A spec written that way inherits the
implementation's regression as ground truth: it documents *how* each operation
deletes (step-by-step, faithfully) but never *where* the operation must execute,
because the code it was copying had already discarded that contract.

> A specification updated to match the implementation cannot catch an
> implementation regression. The spec must carry invariants that are independent
> of, and prior to, the code — so the code can be checked against them.

The delete-class/build-class split is a cross-cutting architectural invariant.
It lives in nobody's spec. Every operation spec is internally consistent; the
corpus has no place to state the property that would make abjure and jettison
*visibly* nonconforming. That absence is the spec-hygiene defect this memo
anchors.

## Improvement hypotheses, per spec system

Each is a falsifiable proposal, not a mandate; the operator settles which to
pursue.

### RBS0 — mint an execution-locus Key Premise
RBS0 *just* gained the Key Premise mechanism (`rbsk_pinning_boundary`, commit
`6fbd459c5`) — minted reactively after a supply-chain near-miss. Same shape
applies here: mint a Key Premise stating the execution-locus contract for
mutating GAR operations (e.g., "the workstation submits and awaits; it holds no
credential that mutates the registry directly"). Once the premise exists, RBSAA
and RBSIJ are *spec-detectably* in violation. Precedent shows the mechanism works
and shows the failure mode: premises are currently minted *after* the miss, not
before. The improvement is to make locus a premise the way pinning became one.

### MCM — give operations a slot to voice the invariants that bind them
The operation grammar (`axhob_operation`, `axhopt_typed_parameter`, `axhos_step`,
`axhoot_typed_output`, `axhoc_completion`) describes an operation's *interior* —
parameters in, steps, output, completion. It has no facet for *exterior*
contracts: which cross-cutting invariants the operation must satisfy. Proposal: an
MCM annotation slot by which an operation voices the invariant(s) it is bound by
(`//axl_voices <invariant>`), so a class of operations can be checked for uniform
conformance and a missing or contradicted voicing is a detectable inconsistency.
The deeper MCM gap: a concept model should express negative space ("this MUST run
in the cloud"), not only positive step lists.

### AXLA — add the positive dual of `axk_premise`
`axk_premise` captures a *negative* bound: what the system will not handle. We
have no motif for a *positive* cross-cutting contract a class of things must
satisfy. Propose minting one (working name: an *invariant* or *locus* motif). And
"execution locus" itself (workstation / cloud-dispatched-and-awaited /
in-container) is plausibly a reusable AXLA motif — the build-vs-local distinction
recurs beyond Recipe Bottle (APCK's in-container discerners vs in-process ones is
the same axis). AXLA is where it belongs if it is reusable.

### BUS0 — should the dispatch layer surface locus?
The tabtargets `rbw-fA` (abjure) and `rbw-iJ{h,r,e}` (jettison) present deletion
as ordinary local colophons, indistinguishable at the dispatch layer from their
cloud-dispatched build-class siblings. Open question for BUS0: is execution locus
a property worth surfacing at the colophon/channel vocabulary, or is the dispatch
layer the wrong altitude for an architectural invariant? This is the *weakest* of
the five links and is recorded as a question, not a recommendation — forcing an
invariant into the dispatch vocabulary may be exactly the non-load-bearing
complexity the project's own principles warn against. Worth a deliberate
yes/no, not a default.

### JJS0 — removal should leave a scar; docket drift is a known instance
Two JJS0-relevant process failures sit in this history:

1. **The removal left no scar.** "Removed delete since unnecessary anymore"
   (`e1a9f16f8`) was an in-the-moment rationale that was never revisited when the
   need returned a week later. JJS0's scar mechanism exists to carry lessons
   across the reset gap. Had the removal been recorded as a scar — *"delete
   removed during the cloud pivot; the cloud-dispatched `rbgh_delete_workflow`
   shape is the reference; any re-introduction must restore it"* — the
   re-introduction would have had an anchor to check against, and a fresh
   instance writing `rbf_delete` would have met that anchor. Lesson: removing a
   capability with a disposable rationale should leave a scar carrying the
   removed *design*, not merely a commit message.
2. **Docket drift, already documented.** The unauthorized `--force` freelance
   docket text that rode reslating into the spec (traced in `3eda4229d`) is a
   concrete instance of the very drift JJS0's docket-posture rules target. It
   belongs in JJS0's own lessons as a worked case: freelance docket prose,
   never operator-authorized, became spec via an alignment commit.

## The recursion
This is the CMK roe covenant viewed from the spec side. The roe says the
governing documents change *only in conversation, never by silent unilateral
edit* — because a spec edited to quietly match drifted reality stops being a
joint commitment and becomes a transcript of whatever happened. That is precisely
what the 2026-02-09 alignment commit did to the delete specs: it edited the spec
to match the code instead of holding the code to the spec. The five levers above
are all one move at different altitudes — install invariants that lead the
implementation, so the spec can say *no*.

## Scope note
This memo is the anchor, not the repair. The concrete delete-architecture fix
(restore cloud-dispatched delete, using the abandoned `rbgh_delete_workflow` as
the shape reference, then rewrite RBSAA/RBSIJ to match) is already flagged as a
deferred reassess pace in heat ₣BH and is the operator's to scope. The
spec-hygiene improvements are independent of that repair and outlive it.
