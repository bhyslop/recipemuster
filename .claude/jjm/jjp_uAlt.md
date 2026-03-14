# Paddock: rbk-mvp-3-add-graft

## Context

Recipe Bottle currently supports two vessel modes: conjure (Cloud Build builds from Dockerfile) and bind (mirror pinned upstream image to GAR). This heat adds a third mode, graft, for users who build locally and want their image accepted into the consecration system.

The name graft comes from botanical grafting: foreign material spliced onto the rootstock. It may take or may not. The metaphor fits the trust posture exactly.

The design conversation surfaced a deeper architectural improvement: all attestation should happen in Cloud Build, not on local workstations. This led to three interconnected changes.

## Core invariant

Once -image is in GAR, the same about-then-vouch pipeline runs regardless of how it got there.

### 1. Graft vessel mode
A locally-built image is pushed to GAR via native docker/podman push, then receives the same about + vouch pipeline as conjure and bind. The vouch verdict is GRAFTED, no provenance chain. No crane dependency needed (image is already local). No dirty-tree guard (git state is irrelevant to an already-built container).

### 2. Standalone about pipeline (extracted from conjure)
The about artifact (-about) is currently generated as steps 06-08 inside the conjure Cloud Build job. Extracting it into a standalone Cloud Build pipeline (Tools/rbk/rbgja/) makes it uniform across all three modes: after -image exists in GAR (however it got there), the director submits an about job that runs syft for SBOM and assembles metadata. This means conjure now requires two Cloud Build round-trips (build, then about).

### 3. Vouch always in Cloud Build (mode-aware)
The bind vouch currently runs locally (docker build + docker push of vouch artifact from workstation). Moving it to Cloud Build eliminates local workstation as an attestation authority. The vouch Cloud Build job becomes mode-aware: conjure gets SLSA provenance verification, bind gets digest-pin comparison, graft gets GRAFTED stamp.

## Trust hierarchy

conjure: Cloud Build build, Cloud Build about (syft), SLSA provenance verified vouch. All Cloud Build.
bind: Mirror to GAR, Cloud Build about (syft), digest-pin vouch. All Cloud Build (was local).
graft: Local docker/podman build, Cloud Build about (syft), GRAFTED vouch. All Cloud Build.

## Consecration format redesign

Old: i20260224_153022-b20260224_160530 (35 chars). i = inscribe, b = build.
New: c260224153022-r260224160530 (27 chars).
- Leading character encodes vessel mode: c (conjure), b (bind), g (graft)
- r = realized (image landed in GAR)
- Drop century 20, drop underscore between date/time
- Regex: [cbg]\d{12}-r\d{12}
- No backward compatibility needed. Depot will be destroyed and recreated by user as a manual prerequisite between spec completion and implementation start. This is outside the heat.

Two timestamps remain meaningful across modes:
- T1: when the operation was initiated (inscribe/mirror/graft)
- T2: when the image materialized in GAR

## Key design decisions
- No crane for graft: docker tag + docker push is sufficient (image already local)
- No dirty-tree guard for graft: the container is already built; git state does not affect it
- About JSON constructed at runtime: like vouch, not committed to rubric repo via inscribe
- Vouch Cloud Build steps: rbgjv01 (slsa-verifier download) early-exits for bind/graft; rbgjv02 branches on _RBGV_VESSEL_MODE
- Syft image pin: currently hardcoded, needs proper RBRG_SYFT_IMAGE_REF regime pin
- Step renumbering: conjure rbgjb/ steps 01-05 + 09 renumber to 01-06 after about extraction

## Pace dependency graph

A (spec) then B (format+regime). F (syft pin) parallel to B. B+F then C (about extraction). C then D (vouch+graft). D then E (tests).

## References
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc: master spec (ark definitions, vessel regime, operations)
- Tools/rbk/vov_veiled/RBSAV-ark_vouch.adoc: current vouch spec (full rewrite needed)
- Tools/rbk/vov_veiled/RBSRV-RegimeVessel.adoc: vessel regime spec
- Tools/rbk/vov_veiled/RBSRG-RegimeGcbPins.adoc: GCB image pin regime (syft pin addition)
- Tools/rbk/rbf_Foundry.sh: implementation (zrbf_vouch_bind to eliminate, rbf_mirror pattern to follow)
- Tools/rbk/rbgjb/: conjure Cloud Build steps (06-08 moving to rbgja/)
- Tools/rbk/rbgjv/: vouch Cloud Build steps (becoming mode-aware)
- Tools/rbk/rbq_Qualify.sh: qualification orchestrator (check for consecration format references)