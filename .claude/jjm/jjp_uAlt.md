# Paddock: rbk-mvp-3-add-graft

## Context

Recipe Bottle currently supports two vessel modes: conjure (Cloud Build builds from Dockerfile) and bind (mirror pinned upstream image to GAR). This heat adds a third mode, graft, for users who build locally and want their image accepted into the consecration system.

The name graft comes from botanical grafting: foreign material spliced onto the rootstock. It may take or may not. The metaphor fits the trust posture exactly.

The design conversation surfaced a deeper architectural improvement: all attestation should happen in Cloud Build, not on local workstations. This led to three interconnected changes.

## Core invariant

Every mode produces -image and -about through a single primary Cloud Build job, then vouch runs as a second Cloud Build job. Two Cloud Build round-trips per mode. The operator invokes one command (`rbf_create`); the system handles the full pipeline.

For conjure and bind, the primary job combines image production with about generation. For graft, the image arrives via local push, so the primary Cloud Build job degenerates to about-only — but it is still the single primary job, not a separate pipeline stage.

### 1. Graft vessel mode
A locally-built image is pushed to GAR via native docker/podman push, then the primary Cloud Build job runs about generation (degenerate — no image step since the image arrived locally). A second Cloud Build job runs vouch. The vouch verdict is GRAFTED, no provenance chain. No crane dependency needed (image is already local). No dirty-tree guard (git state is irrelevant to an already-built container).

### 2. About pipeline (always combined with image operation)
The about artifact (-about) uses the same scripts (rbgja01-04) across all modes. About generation is always part of the primary Cloud Build job — never a separate pipeline stage:
- **Conjure**: about steps are embedded in the trigger-dispatched Cloud Build job alongside build steps. One job produces both -image and -about. The stitch function embeds rbgja scripts during inscribe.
- **Bind**: about steps are included in a builds.create Cloud Build job alongside the image copy step. One job produces both -image and -about.
- **Graft**: about steps run in a builds.create Cloud Build job after local push. Degenerate combined — no image step, just about. Functionally the same scripts, same Cloud Build pattern.

The rbgja scripts work identically in all three contexts. For conjure, the consecration is read from workspace (computed by build step 01); for bind/graft, it comes from the _RBGA_CONSECRATION substitution variable.

### 3. Vouch always in Cloud Build (mode-aware, always separate job)
The bind vouch currently runs locally (docker build + docker push of vouch artifact from workstation). Moving it to Cloud Build eliminates local workstation as an attestation authority. The vouch Cloud Build job becomes mode-aware: conjure gets SLSA provenance verification, bind gets digest-pin comparison, graft gets GRAFTED stamp.

Vouch MUST be a separate Cloud Build job from conjure because SLSA provenance is generated after the build job completes. The slsa-verifier cannot verify provenance from within the same build.

## Pipeline topology

```
rbf_create(vessel_dir)
  primary Cloud Build job (mode-dispatched, produces -about; conjure/bind also produce -image):
    conjure -> rbf_build (trigger-dispatched: build+about steps in one job)
    bind    -> builds.create job (image copy+about steps in one job)
    graft   -> rbf_graft (local push) then builds.create job (about steps only)
  rbf_vouch(vessel_dir, consecration)  <- all modes, always separate Cloud Build job
```

The director polls each Cloud Build job to completion before proceeding. One user command, two Cloud Build round-trips.

## Trust hierarchy

conjure: Cloud Build build+about (one job), Cloud Build vouch (SLSA verified). All Cloud Build.
bind: Cloud Build copy+about (one job), Cloud Build vouch (digest-pin). All Cloud Build.
graft: Local push, Cloud Build about (degenerate primary job), Cloud Build vouch (GRAFTED). Attestation in Cloud Build.

## Consecration format redesign

Old: i20260224_153022-b20260224_160530 (35 chars). i = inscribe, b = build.
New: c260224153022-r260224160530 (27 chars).
- Leading character encodes vessel mode: c (conjure), b (bind), g (graft)
- r = realized (image landed in GAR)
- Drop century 20, drop underscore between date/time
- Regex: [cbg]\d{12}-r\d{12}
- No backward compatibility needed.

Two timestamps remain meaningful across modes:
- T1: when the operation was initiated (inscribe/mirror/graft)
- T2: when the image materialized in GAR

**Spec amendment**: Graft T1 is the OCI image `created` metadata. Reproducible builds may produce T1=`700101000000` (epoch zero); this is a true statement, not an error.

## Key design decisions
- No crane for graft: docker tag + docker push is sufficient (image already local)
- No dirty-tree guard for graft: the container is already built; git state does not affect it
- Vouch Cloud Build steps: rbgjv01 early-exits for bind/graft; rbgjv02 branches on _RBGV_VESSEL_MODE
- Combined conjure job: about steps embedded in trigger-dispatched cloudbuild.json via stitch. This is the architecture, not a future optimization. _RBGA_CONSECRATION read from workspace (not substitution) since consecration is computed at build time.
- Combined bind job: image copy + about steps in a single builds.create Cloud Build job. Mason SA pulls from upstream (public images; private upstream auth is out of scope).
- Graft degenerate primary job: about-only builds.create job after local push. Same rbgja scripts, same Cloud Build submission pattern, no image step.
- Vouch separate from conjure: SLSA provenance is a post-build artifact. Cannot verify provenance from within the same build. Hard constraint.
- rbw-DA is taken by abjure. Standalone about recovery tabtarget uses rbw-Db.

## Spec correction needed

The specs (RBSAB, RBSAC, RBS0) were updated during pace A to say about is "always a standalone builds.create job" with combined delivery as a "future optimization." This is wrong — combined delivery is the architecture. The specs drifted from the paddock and need re-correction:
- **RBSAB**: Remove "always standalone" and "Future: may be embedded" language. About is always part of the primary Cloud Build job. Describe combined delivery for conjure/bind and degenerate combined for graft.
- **RBSAC**: Remove "produces only -image" and "Future optimization" language. Conjure's trigger-dispatched job produces -image AND -about.
- **RBS0**: Update the three-mode pipeline description (currently shows image then about then vouch as three separate stages).
- **RBSAG**: Minor — clarify graft's about is the degenerate primary job, not a separate stage.

## Completed paces

A (spec), B (format+regime), F (syft pin), C (about extraction), G (diags forwarding), H (about suffix substitutions). D (vouch unification) has 1 commit but is not wrapped.

The rbgja scripts extracted in pace C are correct and reusable — they are Cloud Build step scripts that work in any Cloud Build job context (trigger-dispatched, builds.create, or degenerate about-only).

## Remaining pace status

- **AtAAD** (vouch-unification-and-graft): Docket is correct. Vouch is always separate; graft refactor removes about/vouch calls from rbf_graft (chaining moves to rbf_create).
- **AtAAI** (pipeline-orchestration-rbf-create-chaining): Docket needs major rewrite. Currently defers stitch changes and bind Cloud Build as "future" — both are core work. rbf_create chaining: conjure/bind primary jobs already produce -about, so rbf_create chains only vouch for those modes. Only graft chains about then vouch.
- **AtAAE** (integration-test-cases): Test cases should verify combined jobs (conjure/bind produce -image and -about in one job), not standalone about.

## References
- RBSAV-ark_vouch.adoc: vouch spec (correct as-is — always separate Cloud Build job)
- RBSAB-ark_about.adoc: about spec (NEEDS RE-CORRECTION — currently says "always standalone," must describe combined delivery)
- RBSAC-ark_conjure.adoc: conjure spec (NEEDS RE-CORRECTION — currently says "only -image," must describe combined image+about job)
- RBSAG-ark_graft.adoc: graft spec (minor — degenerate combined, chaining in rbf_create)
- RBS0-SpecTop.adoc: top-level spec (NEEDS CORRECTION — three-mode description shows separate about stage)
- rbf_Foundry.sh: implementation
- rbgjb/: conjure Cloud Build steps (stitch must embed rbgja steps)
- rbgja/: about Cloud Build steps (shared scripts across all delivery modes)
- rbgjv/: vouch Cloud Build steps (mode-aware, always separate job)