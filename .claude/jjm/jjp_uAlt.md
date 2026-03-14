# Paddock: rbk-mvp-3-add-graft

## Context

Recipe Bottle currently supports two vessel modes: conjure (Cloud Build builds from Dockerfile) and bind (mirror pinned upstream image to GAR). This heat adds a third mode, graft, for users who build locally and want their image accepted into the consecration system.

The name graft comes from botanical grafting: foreign material spliced onto the rootstock. It may take or may not. The metaphor fits the trust posture exactly.

The design conversation surfaced a deeper architectural improvement: all attestation should happen in Cloud Build, not on local workstations. This led to three interconnected changes.

## Core invariant

Each mode produces -image and -about in a single operation, then vouch runs as a separate Cloud Build job. The operator invokes one command (`rbf_create`); the system handles the full pipeline.

### 1. Graft vessel mode
A locally-built image is pushed to GAR via native docker/podman push, then receives about (standalone Cloud Build) and vouch (separate Cloud Build). The vouch verdict is GRAFTED, no provenance chain. No crane dependency needed (image is already local). No dirty-tree guard (git state is irrelevant to an already-built container).

### 2. About pipeline (mode-dependent delivery)
The about artifact (-about) uses the same scripts (rbgja01-04) across all modes, but delivery varies by mode:
- **Conjure**: about steps are embedded in the trigger-dispatched Cloud Build job alongside build steps. One job produces both -image and -about. The stitch function embeds rbgja scripts during inscribe.
- **Bind**: about steps are included in a builds.create Cloud Build job alongside the image copy step. One job produces both -image and -about.
- **Graft**: about runs as a standalone builds.create Cloud Build job after the local push.

The rbgja scripts work identically in all three contexts. For conjure, the consecration is read from workspace (computed by build step 01); for bind/graft, it comes from the _RBGA_CONSECRATION substitution variable.

### 3. Vouch always in Cloud Build (mode-aware, always separate job)
The bind vouch currently runs locally (docker build + docker push of vouch artifact from workstation). Moving it to Cloud Build eliminates local workstation as an attestation authority. The vouch Cloud Build job becomes mode-aware: conjure gets SLSA provenance verification, bind gets digest-pin comparison, graft gets GRAFTED stamp.

Vouch MUST be a separate Cloud Build job from conjure because SLSA provenance is generated after the build job completes. The slsa-verifier cannot verify provenance from within the same build.

## Pipeline topology

```
rbf_create(vessel_dir)
  mode dispatch -> each produces -image AND -about:
    conjure -> rbf_build (trigger-dispatched: build+about steps in one job)
    bind    -> new Cloud Build job (builds.create: image copy+about steps)
    graft   -> rbf_graft (local push) + rbf_about (standalone Cloud Build)
  rbf_vouch(vessel_dir, consecration)  <- all modes, always separate Cloud Build job
```

The director polls each Cloud Build job to completion before proceeding. One user command, multiple Cloud Build round-trips behind the scenes.

## Trust hierarchy

conjure: Cloud Build build+about (one job), Cloud Build vouch (SLSA verified). All Cloud Build.
bind: Cloud Build copy+about (one job), Cloud Build vouch (digest-pin). All Cloud Build.
graft: Local push, Cloud Build about, Cloud Build vouch (GRAFTED). Attestation in Cloud Build.

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
- Combined conjure job: about steps embedded in trigger-dispatched cloudbuild.json via stitch. _RBGA_CONSECRATION read from workspace (not substitution) since consecration is computed at build time.
- Bind moves to Cloud Build: image copy via docker buildx imagetools create in Cloud Build. Mason SA pulls from upstream (public images; private upstream auth is out of scope).
- Vouch separate from conjure: SLSA provenance is a post-build artifact. Cannot verify provenance from within the same build. Hard constraint.
- rbw-DA is taken by abjure. Standalone about recovery tabtarget uses rbw-Db.

## Pace dependency graph

A (spec) then B (format+regime). F (syft pin) parallel to B. B+F then C (about extraction). C then D (vouch+graft+pipeline). D then E (tests).

## Draft code status

A draft implementation exists in the working tree (uncommitted) written against the pre-correction docket. The vouch unification (rbgjv scripts, zrbf_vouch_submit, rbf_vouch) is correct and reusable. The graft push logic is correct but needs its about/vouch tail relocated to rbf_create. The draft does NOT address: rbf_create chaining, rbf_mirror simplification, stitch changes for combined conjure+about, or bind Cloud Build job.

## References
- RBSAV-ark_vouch.adoc: vouch spec (correct as-is -- always separate Cloud Build job)
- RBSAB-ark_about.adoc: about spec (NEEDS UPDATE -- must describe embedded delivery for conjure/bind)
- RBSAC-ark_conjure.adoc: conjure spec (NEEDS UPDATE -- combined build+about job)
- RBSAG-ark_graft.adoc: graft spec (minor update -- chaining in rbf_create, not rbf_graft)
- rbf_Foundry.sh: implementation
- rbgjb/: conjure Cloud Build steps (stitch must embed rbgja steps too)
- rbgja/: about Cloud Build steps (shared scripts across all delivery modes)
- rbgjv/: vouch Cloud Build steps (mode-aware, always separate job)
