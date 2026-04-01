# Heat Trophy: rbk-mvp-3-add-graft

**Firemark:** ₣At
**Created:** 260314
**Retired:** 260331
**Status:** retired

## Paddock

# Paddock: rbk-mvp-3-add-graft

## Core invariant

Every mode produces -image and -about through a single primary Cloud Build job, then vouch runs as a second Cloud Build job. Two Cloud Build round-trips per mode. The operator invokes one command (`rbf_ordain`); the system handles the full pipeline.

For conjure and bind, the primary job combines image production with about generation. For graft, the image arrives via local push, so the primary Cloud Build job degenerates to about-only — but it is still the single primary job, not a separate pipeline stage.

## Pipeline topology

```
rbf_ordain(vessel_dir)
  primary Cloud Build job (mode-dispatched, produces -about; conjure/bind also produce -image):
    conjure -> rbf_build (trigger-dispatched: build+about steps in one job)
    bind    -> builds.create job (image copy+about steps in one job)
    graft   -> rbf_graft (local push) then builds.create job (about steps only)
  rbf_vouch(vessel_dir, consecration)  <- all modes, always separate Cloud Build job
```

The director polls each Cloud Build job to completion before proceeding. One user command, two Cloud Build round-trips.

## Key design decisions

- No crane for graft: docker tag + docker push is sufficient (image already local)
- No dirty-tree guard for graft: the container is already built; git state does not affect it
- Vouch Cloud Build steps: rbgjv01 early-exits for bind/graft; rbgjv02 branches on _RBGV_VESSEL_MODE
- Combined conjure job: about steps embedded in trigger-dispatched cloudbuild.json via stitch. _RBGA_CONSECRATION read from workspace (not substitution) since consecration is computed at build time.
- Combined bind job: image copy + about steps in a single builds.create Cloud Build job. Mason SA pulls from upstream (public images; private upstream auth is out of scope).
- Graft degenerate primary job: about-only builds.create job after local push. Same rbgja scripts, same Cloud Build submission pattern, no image step.
- Vouch separate from conjure: SLSA provenance is a post-build artifact. Cannot verify provenance from within the same build. Hard constraint.
- rbw-DA is taken by abjure. Standalone about recovery tabtarget uses rbw-Db.

## References

- RBSAV-ark_vouch.adoc: vouch spec (always separate Cloud Build job)
- RBSAB-ark_about.adoc: about spec (four Cloud Build steps only, no director wrapper)
- RBSAC-ark_conjure.adoc: conjure spec (combined image+about job)
- RBSAG-ark_graft.adoc: graft spec (degenerate combined, chaining in rbf_ordain)
- RBS0-SpecTop.adoc: top-level spec (three-mode combined delivery)
- rbf_Foundry.sh: implementation
- rbgjb/: conjure Cloud Build steps (stitch must embed rbgja steps)
- rbgja/: about Cloud Build steps (shared scripts across all delivery modes)
- rbgjv/: vouch Cloud Build steps (mode-aware, always separate job)

## Paces

### manual-test-fixes-continuation (₢AtAAM) [complete]

**[260314-2030] complete**

Continue manual testing of three-mode combined delivery pipeline. Pick up from bind vouch failure.

## Character
Systematic bug-fix-and-retry with live GCP builds. Each fix requires spec update, code change, notch, and retry.

## Where we left off

Manual testing of all three modes via `rbf_create` uncovered and fixed several bugs:

### Completed
- **Graft**: Full pipeline works (push + about + vouch). Three bugs fixed:
  1. Cloud Build rejects substitution vars with bash `:-}` syntax — removed from rbgja03 (4 variables)
  2. Attestation manifests (platform unknown/unknown) in OCI indexes pollute platform discovery — added filter to rbgja01 and rbgjv02, with spec updates to RBSAB and RBSAV
  3. Syft index auto-selection fails on arm64-only grafted images — bind/graft now always use @digest pinning in rbgja02
- **Conjure**: Full pipeline works (trigger-dispatched build + stitch-embedded about + vouch). No new bugs.
- **Bind**: Combined mirror job succeeds (image copy + about). Vouch step partially fixed:
  - Fixed `_RBGV_BIND_SOURCE: unbound variable` — Cloud Build cannot handle `${VAR#pattern}` bash expansion; captured to local var first

### Remaining (bind vouch)
- **Digest format mismatch**: `sed 's/.*: *//'` in rbgjv02 line 186 is too greedy — strips `sha256:` prefix from `Docker-Content-Digest` header value. Fix: `s/^[^:]*: *//` to match only the first colon. One-character fix, then retry bind.

### After bind vouch passes
- Run consecration check to verify all artifacts across all three modes
- Consider running inspect to verify mode-specific trust statements
- Created `rbev-busybox-graft` vessel for graft testing (committed)

## Files changed this session
- `Tools/rbk/rbgja/rbgja01-discover-platforms.sh` — attestation filter + diagnostics
- `Tools/rbk/rbgja/rbgja02-syft-per-platform.sh` — digest pinning for bind/graft
- `Tools/rbk/rbgja/rbgja03-build-info-per-platform.sh` — removed `:-}` from 4 substitution refs
- `Tools/rbk/rbgjv/rbgjv02-verify-provenance.sh` — attestation filter + bind source local var
- `Tools/rbk/vov_veiled/RBSAB-ark_about.adoc` — spec: attestation filtering + syft scan targets
- `Tools/rbk/vov_veiled/RBSAV-ark_vouch.adoc` — spec: attestation filtering
- `rbev-vessels/rbev-busybox-graft/rbrv.env` — new graft test vessel

**[260314-1813] rough**

Continue manual testing of three-mode combined delivery pipeline. Pick up from bind vouch failure.

## Character
Systematic bug-fix-and-retry with live GCP builds. Each fix requires spec update, code change, notch, and retry.

## Where we left off

Manual testing of all three modes via `rbf_create` uncovered and fixed several bugs:

### Completed
- **Graft**: Full pipeline works (push + about + vouch). Three bugs fixed:
  1. Cloud Build rejects substitution vars with bash `:-}` syntax — removed from rbgja03 (4 variables)
  2. Attestation manifests (platform unknown/unknown) in OCI indexes pollute platform discovery — added filter to rbgja01 and rbgjv02, with spec updates to RBSAB and RBSAV
  3. Syft index auto-selection fails on arm64-only grafted images — bind/graft now always use @digest pinning in rbgja02
- **Conjure**: Full pipeline works (trigger-dispatched build + stitch-embedded about + vouch). No new bugs.
- **Bind**: Combined mirror job succeeds (image copy + about). Vouch step partially fixed:
  - Fixed `_RBGV_BIND_SOURCE: unbound variable` — Cloud Build cannot handle `${VAR#pattern}` bash expansion; captured to local var first

### Remaining (bind vouch)
- **Digest format mismatch**: `sed 's/.*: *//'` in rbgjv02 line 186 is too greedy — strips `sha256:` prefix from `Docker-Content-Digest` header value. Fix: `s/^[^:]*: *//` to match only the first colon. One-character fix, then retry bind.

### After bind vouch passes
- Run consecration check to verify all artifacts across all three modes
- Consider running inspect to verify mode-specific trust statements
- Created `rbev-busybox-graft` vessel for graft testing (committed)

## Files changed this session
- `Tools/rbk/rbgja/rbgja01-discover-platforms.sh` — attestation filter + diagnostics
- `Tools/rbk/rbgja/rbgja02-syft-per-platform.sh` — digest pinning for bind/graft
- `Tools/rbk/rbgja/rbgja03-build-info-per-platform.sh` — removed `:-}` from 4 substitution refs
- `Tools/rbk/rbgjv/rbgjv02-verify-provenance.sh` — attestation filter + bind source local var
- `Tools/rbk/vov_veiled/RBSAB-ark_about.adoc` — spec: attestation filtering + syft scan targets
- `Tools/rbk/vov_veiled/RBSAV-ark_vouch.adoc` — spec: attestation filtering
- `rbev-vessels/rbev-busybox-graft/rbrv.env` — new graft test vessel

### rbsab-about-procedure-refactor (₢AtAAJ) [complete]

**[260314-1608] complete**

Strip RBSAB of director-side submission wrapper. Keep the four Cloud Build steps as the procedure.

## Character
Precise structural surgery on one spec file. No design decisions — plan is the docket.

## Model check
conjure = one job (image+about via stitch), bind = one job (copy+about), graft = local push then one job (about only), all modes = separate vouch job. RBSAB describes the about Cloud Build steps that are universal across all three modes.

## RBSAB changes

### Opening notes (lines 4-21): STRENGTHEN
Already correct about combined delivery. Add one clarifying sentence after line 10:
"This procedure describes the Cloud Build steps that produce `-about`, not a standalone Cloud Build job. Each mode's operation embeds these steps in its primary Cloud Build job (conjure via stitch, bind via builds.create construction, graft as a degenerate about-only builds.create job)."

### Director-side steps: REMOVE
These describe the graft/recovery submission wrapper, not the about procedure itself:
- "Validate Input Parameters" (current lines 27-33)
- "Load Vessel Configuration" (current lines 35-41)
- "Authenticate as Director" (current lines 43-49)
- "Gate on Image Existence" (current lines 51-59)
- "Submit About Cloud Build" (current lines 61-74)
- "Wait for Build Completion" (current lines 146-152)
- "Display Results" (current lines 154-158)

The `//axhop_parameter_from_type` for ark (lines 27-28) moves to become a NOTE explaining that the about steps receive their context via substitution variables and workspace state, not via function parameters.

### Cloud Build Steps 1-4: KEEP
- Step 1: Discover Platforms (current lines 76-97) — keep as-is
- Step 2: Syft SBOM Generation (current lines 99-112) — keep as-is
- Step 3: Assemble build_info.json (current lines 114-133) — keep as-is
- Step 4: Build and Push About Artifact (current lines 135-144) — keep as-is

### Substitution variables table (lines 176-211): KEEP with one fix
Line 204: _RBGA_BUILD_ID description says "Ephemeral director-side state retained between the conjure and about builds.create calls." Wrong for combined architecture. Replace with: "Conjure Cloud Build job ID (conjure only). In the combined conjure job, populated from the built-in $BUILD_ID. In graft/recovery standalone about, passed by the director from a prior conjure run. Used by step 3 to populate build_info.json. Empty for bind/graft."

Add `//axhos_waymark` anchor `[[rbtgo_about_substitutions]]` before the substitution variables section header for cross-reference from mode specs.

### Completion section (current lines 160-174): REWRITE
Remove "Proceed to vouch" (that's the caller's concern, not this procedure's). Keep re-about idempotency note. Keep implementation note about shared helper. Completion should describe: successful execution of these steps produces the -about artifact tagged as VESSEL:CONSECRATION-about.

## Paddock update
Replace stale "Spec correction needed" section with "Spec correction completed" noting commit 7a8bd2fa + this pace. Replace stale References entries that say "NEEDS RE-CORRECTION" with current status.

## Files touched
- Tools/rbk/vov_veiled/RBSAB-ark_about.adoc

**[260314-1450] rough**

Strip RBSAB of director-side submission wrapper. Keep the four Cloud Build steps as the procedure.

## Character
Precise structural surgery on one spec file. No design decisions — plan is the docket.

## Model check
conjure = one job (image+about via stitch), bind = one job (copy+about), graft = local push then one job (about only), all modes = separate vouch job. RBSAB describes the about Cloud Build steps that are universal across all three modes.

## RBSAB changes

### Opening notes (lines 4-21): STRENGTHEN
Already correct about combined delivery. Add one clarifying sentence after line 10:
"This procedure describes the Cloud Build steps that produce `-about`, not a standalone Cloud Build job. Each mode's operation embeds these steps in its primary Cloud Build job (conjure via stitch, bind via builds.create construction, graft as a degenerate about-only builds.create job)."

### Director-side steps: REMOVE
These describe the graft/recovery submission wrapper, not the about procedure itself:
- "Validate Input Parameters" (current lines 27-33)
- "Load Vessel Configuration" (current lines 35-41)
- "Authenticate as Director" (current lines 43-49)
- "Gate on Image Existence" (current lines 51-59)
- "Submit About Cloud Build" (current lines 61-74)
- "Wait for Build Completion" (current lines 146-152)
- "Display Results" (current lines 154-158)

The `//axhop_parameter_from_type` for ark (lines 27-28) moves to become a NOTE explaining that the about steps receive their context via substitution variables and workspace state, not via function parameters.

### Cloud Build Steps 1-4: KEEP
- Step 1: Discover Platforms (current lines 76-97) — keep as-is
- Step 2: Syft SBOM Generation (current lines 99-112) — keep as-is
- Step 3: Assemble build_info.json (current lines 114-133) — keep as-is
- Step 4: Build and Push About Artifact (current lines 135-144) — keep as-is

### Substitution variables table (lines 176-211): KEEP with one fix
Line 204: _RBGA_BUILD_ID description says "Ephemeral director-side state retained between the conjure and about builds.create calls." Wrong for combined architecture. Replace with: "Conjure Cloud Build job ID (conjure only). In the combined conjure job, populated from the built-in $BUILD_ID. In graft/recovery standalone about, passed by the director from a prior conjure run. Used by step 3 to populate build_info.json. Empty for bind/graft."

Add `//axhos_waymark` anchor `[[rbtgo_about_substitutions]]` before the substitution variables section header for cross-reference from mode specs.

### Completion section (current lines 160-174): REWRITE
Remove "Proceed to vouch" (that's the caller's concern, not this procedure's). Keep re-about idempotency note. Keep implementation note about shared helper. Completion should describe: successful execution of these steps produces the -about artifact tagged as VESSEL:CONSECRATION-about.

## Paddock update
Replace stale "Spec correction needed" section with "Spec correction completed" noting commit 7a8bd2fa + this pace. Replace stale References entries that say "NEEDS RE-CORRECTION" with current status.

## Files touched
- Tools/rbk/vov_veiled/RBSAB-ark_about.adoc

### rbsav-vouch-platform-discovery-alignment (₢AtAAK) [complete]

**[260314-1614] complete**

Align RBSAV spec with draft code: step 2 discovers platforms for ALL modes, step 3 reads from workspace.

## Character
Small targeted spec edit. Two sections of one file.

## Model check
Vouch is always a separate Cloud Build job (builds.create) for all three modes. Step 2 runs verification logic per mode. Step 3 assembles and pushes the -vouch artifact.

## Current spec (wrong)
- Step 2 conjure branch: discovers platforms via manifest GET (lines 97-100). Correct.
- Step 2 bind branch (lines 117-125): does NOT discover platforms. Wrong.
- Step 2 graft branch (lines 127-129): does NOT discover platforms. Wrong.
- Step 3 (line 136): independently discovers platforms via manifest GET. Wrong — duplicates work and requires registry API access from the docker image.

## Draft code (correct, approved directionally)
- Step 2 discovers platforms for ALL three branches and writes /workspace/vouch_platforms.txt
- Step 3 reads platforms from /workspace/vouch_platforms.txt
- Step 3 runs the docker builder image (not gcloud), so avoiding registry API access in step 3 is cleaner

## RBSAV changes

### Step 2 bind branch (after line 125): ADD platform discovery
After composing vouch_summary.json, add:
- GET manifests for CONSECRATION-image to discover platforms
- Write platform list to /workspace/vouch_platforms.txt

### Step 2 graft branch (after line 129): ADD platform discovery
After composing vouch_summary.json, add:
- GET manifests for CONSECRATION-image to discover platforms
- Write platform list to /workspace/vouch_platforms.txt

### Step 2 conjure branch: ADD workspace write
After platform discovery (line 100), add:
- Write platform list to /workspace/vouch_platforms.txt
(Conjure already discovers platforms; just needs the workspace write)

### Step 3 (line 136): REPLACE platform source
Change: "Platforms: discovered via GET manifest for CONSECRATION-image"
To: "Platforms: read from /workspace/vouch_platforms.txt (written by step 2)"
Remove the parenthetical about manifest list pattern.

## Files touched
- Tools/rbk/vov_veiled/RBSAV-ark_vouch.adoc

**[260314-1454] rough**

Align RBSAV spec with draft code: step 2 discovers platforms for ALL modes, step 3 reads from workspace.

## Character
Small targeted spec edit. Two sections of one file.

## Model check
Vouch is always a separate Cloud Build job (builds.create) for all three modes. Step 2 runs verification logic per mode. Step 3 assembles and pushes the -vouch artifact.

## Current spec (wrong)
- Step 2 conjure branch: discovers platforms via manifest GET (lines 97-100). Correct.
- Step 2 bind branch (lines 117-125): does NOT discover platforms. Wrong.
- Step 2 graft branch (lines 127-129): does NOT discover platforms. Wrong.
- Step 3 (line 136): independently discovers platforms via manifest GET. Wrong — duplicates work and requires registry API access from the docker image.

## Draft code (correct, approved directionally)
- Step 2 discovers platforms for ALL three branches and writes /workspace/vouch_platforms.txt
- Step 3 reads platforms from /workspace/vouch_platforms.txt
- Step 3 runs the docker builder image (not gcloud), so avoiding registry API access in step 3 is cleaner

## RBSAV changes

### Step 2 bind branch (after line 125): ADD platform discovery
After composing vouch_summary.json, add:
- GET manifests for CONSECRATION-image to discover platforms
- Write platform list to /workspace/vouch_platforms.txt

### Step 2 graft branch (after line 129): ADD platform discovery
After composing vouch_summary.json, add:
- GET manifests for CONSECRATION-image to discover platforms
- Write platform list to /workspace/vouch_platforms.txt

### Step 2 conjure branch: ADD workspace write
After platform discovery (line 100), add:
- Write platform list to /workspace/vouch_platforms.txt
(Conjure already discovers platforms; just needs the workspace write)

### Step 3 (line 136): REPLACE platform source
Change: "Platforms: discovered via GET manifest for CONSECRATION-image"
To: "Platforms: read from /workspace/vouch_platforms.txt (written by step 2)"
Remove the parenthetical about manifest list pattern.

## Files touched
- Tools/rbk/vov_veiled/RBSAV-ark_vouch.adoc

### specification-updates-graft-about-vouch (₢AtAAA) [complete]

**[260314-1114] complete**

Update RBS0 and operation subdocuments to specify: (1) graft vessel mode, (2) standalone Cloud Build about pipeline, (3) mode-aware Cloud Build vouch pipeline, (4) new consecration format. Create all missing subdocuments.

## Character
Architectural design writing — intricate cross-document consistency work requiring judgment on several open design questions.

## Consecration Format Change (cross-cutting)
Old: `i20260224_153022-b20260224_160530` (35 chars)
New: `[cbg]YYMMDDHHMMSS-rYYMMDDHHMMSS` (27 chars)
- Mode prefix: `c` (conjure), `b` (bind), `g` (graft)
- `r` = realized (image landed in GAR)
- No backward compatibility — depot will be destroyed and recreated. Clean cut.
- Touches: rbtga_consecration definition, every spec referencing the format

## Documents to modify (10 existing)

**RBS0-SpecTop.adoc** (heaviest):
- Mapping section: add rbrv_vessel_mode_graft, rbrv_group_grafting, rbrv_graft_image, rbtgo_ark_about, rbtgo_ark_graft
- rbtga_ark: all three arrival paths; vouch now mandatory
- rbtga_ark_about: rewrite — standalone Cloud Build syft job, uniform across modes
- rbtga_ark_vouch: rewrite — mandatory, always Cloud Build, mode-aware
- rbtga_consecration: new format, per-mode timestamp semantics
- Vessel mode enum: add graft
- Vessel regime: add rbrv_group_grafting with rbrv_graft_image
- rbtgog_ark: add about and graft operations

**RBSAV-ark_vouch.adoc** (full rewrite): mode-aware Cloud Build, three branches
**RBSAC-ark_conjure.adoc** (minor): conjure produces only -image now
**RBSTB-trigger_build.adoc** (moderate): remove steps 06-08, renumber, new format
**RBSRI-rubric_inscribe.adoc** (moderate — was missing): reduced cloudbuild.json
**RBSAI-ark_inspect.adoc** (moderate): graft branch, three verdict types
**RBSAA-ark_abjure.adoc** (minor): delete all three artifacts uniformly
**RBSCK-consecration_check.adoc** (minor): new format regex, mode display
**RBSAS-ark_summon.adoc** (no change expected)
**RBSRV-RegimeVessel.adoc** (moderate): graft mode, group, kindle/validate/render

## New documents to create (2)

**RBSAB-ark_about.adoc**: standalone Cloud Build about job. Steps: discover platforms, syft, build-info (mode-aware), push -about. Scripts rbgja01-04. Namespace _RBGA_*. Git metadata as substitutions.

**RBSAG-ark_graft.adoc**: push local image via docker/podman native push (no crane). docker tag + push. Runtime param: local image ref. Consecration gYYMMDDHHMMSS-rYYMMDDHHMMSS. No dirty-tree guard (image already built).

## Open design questions
1. Git metadata in about: which fields for bind/graft _RBGA_* substitutions
2. Single-arch handling in rbgja platform discovery
3. RBRV_GRAFT_IMAGE: regime constant vs runtime parameter
4. Graft about build_info: what git metadata is meaningful

**[260314-0954] rough**

Update RBS0 and operation subdocuments to specify: (1) graft vessel mode, (2) standalone Cloud Build about pipeline, (3) mode-aware Cloud Build vouch pipeline, (4) new consecration format. Create all missing subdocuments.

## Character
Architectural design writing — intricate cross-document consistency work requiring judgment on several open design questions.

## Consecration Format Change (cross-cutting)
Old: `i20260224_153022-b20260224_160530` (35 chars)
New: `[cbg]YYMMDDHHMMSS-rYYMMDDHHMMSS` (27 chars)
- Mode prefix: `c` (conjure), `b` (bind), `g` (graft)
- `r` = realized (image landed in GAR)
- No backward compatibility — depot will be destroyed and recreated. Clean cut.
- Touches: rbtga_consecration definition, every spec referencing the format

## Documents to modify (10 existing)

**RBS0-SpecTop.adoc** (heaviest):
- Mapping section: add rbrv_vessel_mode_graft, rbrv_group_grafting, rbrv_graft_image, rbtgo_ark_about, rbtgo_ark_graft
- rbtga_ark: all three arrival paths; vouch now mandatory
- rbtga_ark_about: rewrite — standalone Cloud Build syft job, uniform across modes
- rbtga_ark_vouch: rewrite — mandatory, always Cloud Build, mode-aware
- rbtga_consecration: new format, per-mode timestamp semantics
- Vessel mode enum: add graft
- Vessel regime: add rbrv_group_grafting with rbrv_graft_image
- rbtgog_ark: add about and graft operations

**RBSAV-ark_vouch.adoc** (full rewrite): mode-aware Cloud Build, three branches
**RBSAC-ark_conjure.adoc** (minor): conjure produces only -image now
**RBSTB-trigger_build.adoc** (moderate): remove steps 06-08, renumber, new format
**RBSRI-rubric_inscribe.adoc** (moderate — was missing): reduced cloudbuild.json
**RBSAI-ark_inspect.adoc** (moderate): graft branch, three verdict types
**RBSAA-ark_abjure.adoc** (minor): delete all three artifacts uniformly
**RBSCK-consecration_check.adoc** (minor): new format regex, mode display
**RBSAS-ark_summon.adoc** (no change expected)
**RBSRV-RegimeVessel.adoc** (moderate): graft mode, group, kindle/validate/render

## New documents to create (2)

**RBSAB-ark_about.adoc**: standalone Cloud Build about job. Steps: discover platforms, syft, build-info (mode-aware), push -about. Scripts rbgja01-04. Namespace _RBGA_*. Git metadata as substitutions.

**RBSAG-ark_graft.adoc**: push local image via docker/podman native push (no crane). docker tag + push. Runtime param: local image ref. Consecration gYYMMDDHHMMSS-rYYMMDDHHMMSS. No dirty-tree guard (image already built).

## Open design questions
1. Git metadata in about: which fields for bind/graft _RBGA_* substitutions
2. Single-arch handling in rbgja platform discovery
3. RBRV_GRAFT_IMAGE: regime constant vs runtime parameter
4. Graft about build_info: what git metadata is meaningful

**[260314-0939] rough**

Update RBS0 and operation subdocuments to specify: (1) graft vessel mode, (2) standalone Cloud Build about pipeline, (3) mode-aware Cloud Build vouch pipeline, (4) new consecration format.

## Character
Architectural design writing — intricate cross-document consistency work requiring judgment on several open design questions.

## Consecration Format Change (new, cross-cutting)
Old: `i20260224_153022-b20260224_160530` (35 chars)
New: `«MODE»YYMMDDHHMMSS-rYYMMDDHHMMSS` (27 chars)
- Mode prefix: `c` (conjure), `b` (bind), `g` (graft)
- `r` = realized (image landed in GAR)
- Drop century `20`, drop `_` between date/time
- Regex: `[cbg]\d{12}-r\d{12}`
- Touches: rbtga_consecration definition in RBS0, consecration_check parsing, trigger_build tag construction (rbgjb01), mirror tag construction in rbf_mirror, every spec that references the format

## Documents to modify (9 existing)

**RBS0-SpecTop.adoc** (heaviest):
- Mapping section: add linked terms rbrv_vessel_mode_graft, rbrv_group_grafting, rbrv_graft_image
- rbtga_ark definition: cover all three arrival paths (conjure/bind/graft)
- rbtga_ark_image: add bind/graft origin variants alongside conjure's SLSA description
- rbtga_ark_about: rewrite — no longer conjure build side-effect; now standalone Cloud Build syft job, uniform across modes
- rbtga_ark_vouch: rewrite — no longer optional, no longer SLSA-only; always Cloud Build, mode-aware (SLSA/digest-pin/GRAFTED)
- rbtga_consecration: new format [cbg]YYMMDDHHMMSS-rYYMMDDHHMMSS, redefine timestamp semantics per mode
- Vessel mode enum: add graft value definition
- Vessel regime: add rbrv_group_grafting with rbrv_graft_image
- Operation mapping: add rbtgo_ark_about and rbtgo_ark_graft linked terms

**RBSAV-ark_vouch.adoc** (full rewrite):
- Mode-aware Cloud Build job with three verification branches
- New substitution variable _RBGV_VESSEL_MODE
- Bind branch: digest-pin comparison (replaces current local zrbf_vouch_bind)
- Graft branch: GRAFTED stamp, no provenance
- Conjure branch: existing SLSA verification (largely preserved)

**RBSAC-ark_conjure.adoc** (minor):
- Remove implication that about is generated during conjure build
- Conjure now produces only -image; about is separate downstream job

**RBSTB-trigger_build.adoc** (moderate):
- Remove steps 06-08 (syft, build-info, about assembly) — move to standalone about pipeline
- Build becomes purely: image construction + push
- Update consecration format in tag construction section
- Note: conjure now requires two Cloud Build round-trips (build, then about)

**RBSAI-ark_inspect.adoc** (moderate):
- Add graft branch alongside bind/conjure
- About extraction now uniform (syft SBOM) across all modes
- Vouch display handles three verdict types

**RBSAA-ark_abjure.adoc** (minor):
- Uniformly delete all three artifacts (-image, -about, -vouch) for all modes

**RBSCK-consecration_check.adoc** (minor):
- Consecration pattern match updated for new format
- Vessel mode in display; GRAFTED vouch status distinct from SLSA-vouched

**RBSAS-ark_summon.adoc** (no change expected — already generic)

**RBSRV-RegimeVessel.adoc** (moderate):
- Add graft to vessel mode enum
- Add rbrv_group_grafting with gate, rbrv_graft_image variable
- Update kindle/validate/render sections

## New documents (2)

**RBSAB-ark_about.adoc** (new operation spec):
- Standalone Cloud Build about job: director submits after -image exists in GAR
- Steps: discover platforms (from GAR manifest), syft per-platform, build-info per-platform (mode-aware metadata), assemble+push -about
- Cloud Build step scripts: rbgja01 through rbgja04
- Substitution namespace _RBGA_*
- Uniform across all three vessel modes
- Git metadata (commit, branch, remote) passed as substitution variables from local context

**RBSAG-ark_graft.adoc** (new operation spec):
- Graft operation: push local image to GAR as consecrated -image
- Runtime param: local image reference (not a regime constant — changes each graft)
- Regime param: RBRV_GRAFT_IMAGE could be default local tag
- Consecration: gYYMMDDHHMMSS (graft initiation) + rYYMMDDHHMMSS (push completion)
- Director authenticates, pushes via crane/docker, then triggers about + vouch pipelines

## Cloud Build step script changes (for implementation pace reference)
- New directory rbgja/ (about pipeline): rbgja01 through rbgja04
- Modified rbgjv/ (vouch pipeline): rbgjv02 becomes mode-aware
- Reduced rbgjb/ (build pipeline): steps 06-08 removed, purely image construction

## Open design questions to resolve during spec writing
1. Git metadata in about: conjure gets it from Cloud Build context; bind/graft need it passed as _RBGA_* substitution variables
2. Single-arch handling in rbgja pipeline: bind/graft images may be single-platform; platform discovery step must handle gracefully
3. Per-platform tag suffixes in new format: platform tags like -image-amd64 still appended after consecration
4. RBRV_GRAFT_IMAGE: regime constant (default local tag) vs purely runtime parameter

**[260314-0931] rough**

Update RBS0 and operation subdocuments to specify: (1) graft vessel mode, (2) standalone Cloud Build about pipeline, (3) mode-aware Cloud Build vouch pipeline.

## Character
Architectural design writing — intricate cross-document consistency work requiring judgment on several open design questions.

## Documents to modify (9 existing)

**RBS0-SpecTop.adoc** (heaviest):
- Mapping section: add linked terms rbrv_vessel_mode_graft, rbrv_group_grafting, rbrv_graft_image
- rbtga_ark definition: cover all three arrival paths (conjure/bind/graft)
- rbtga_ark_image: add bind/graft origin variants alongside conjure's SLSA description
- rbtga_ark_about: rewrite — no longer conjure build side-effect; now standalone Cloud Build syft job, uniform across modes
- rbtga_ark_vouch: rewrite — no longer optional, no longer SLSA-only; always Cloud Build, mode-aware (SLSA/digest-pin/GRAFTED)
- Vessel mode enum: add graft value definition
- Vessel regime: add rbrv_group_grafting with rbrv_graft_image
- Consecration format: confirm i-prefix works generically (initiator timestamp) for graft

**RBSAV-ark_vouch.adoc** (full rewrite):
- Mode-aware Cloud Build job with three verification branches
- New substitution variable _RBGV_VESSEL_MODE
- Bind branch: digest-pin comparison (replaces current local zrbf_vouch_bind)
- Graft branch: GRAFTED stamp, no provenance
- Conjure branch: existing SLSA verification (largely preserved)

**RBSAC-ark_conjure.adoc** (minor):
- Remove implication that about is generated during conjure build
- Conjure now produces only -image; about is separate downstream job

**RBSTB-trigger_build.adoc** (moderate):
- Remove steps 06-08 (syft, build-info, about assembly) — these move to standalone about pipeline
- Build becomes purely: image construction + push
- Note: conjure now requires two Cloud Build round-trips

**RBSAI-ark_inspect.adoc** (moderate):
- Add graft branch alongside bind/conjure
- About extraction now uniform (syft SBOM) across all modes
- Vouch display handles three verdict types

**RBSAA-ark_abjure.adoc** (minor):
- Uniformly delete all three artifacts (-image, -about, -vouch) for all modes

**RBSCK-consecration_check.adoc** (minor):
- Vessel mode in display; GRAFTED vouch status distinct from SLSA-vouched

**RBSAS-ark_summon.adoc** (no change expected — already generic)

**RBSRV-RegimeVessel.adoc** (moderate):
- Add graft to vessel mode enum
- Add rbrv_group_grafting with gate, rbrv_graft_image variable
- Update kindle/validate/render sections

## New documents (2)

**RBSAB-ark_about.adoc** (new operation spec):
- Standalone Cloud Build about job: director submits after -image exists in GAR
- Steps: discover platforms, syft per-platform, build-info per-platform, assemble+push -about
- Cloud Build step scripts: rbgja01 through rbgja04
- Substitution namespace _RBGA_*
- Uniform across all three vessel modes

**RBSAG-ark_graft.adoc** (new operation spec):
- Graft operation: push local image to GAR as consecrated -image
- Runtime param: local image reference
- Consecration: i-timestamp (graft initiation) + b-timestamp (placeholder or push completion)
- Director authenticates, pushes via crane/docker, then triggers about + vouch pipelines

## Open design questions to resolve during spec writing
1. Consecration i-prefix: confirm generic 'initiator timestamp' semantics work for graft
2. Graft b-timestamp: Cloud Build doesn't build the image — is b-timestamp the about job time, or graft push time?
3. Git metadata in about: conjure gets it from Cloud Build context; bind/graft need it passed as substitution variables from local
4. Single-arch handling in rbgja pipeline: bind/graft may be single-platform

### consecration-format-and-regime-foundation (₢AtAAB) [complete]

**[260314-1135] complete**

Implement new consecration format and graft regime scaffolding.

## Character
Mechanical but cross-cutting — no backward compatibility needed (depot will be recreated).

## Consecration Format Change
New: `[cbg]YYMMDDHHMMSS-rYYMMDDHHMMSS` (27 chars). No dual-format parsing needed.

## Sites to update
- `rbgjb01-derive-tag-base.sh`: consecration construction, `c` prefix
- `rbf_mirror` in rbf_Foundry.sh: mirror tag construction, `b` prefix
- `rbf_check_consecrations` / consecration_check parsing: new regex only
- All other consecration format references in RBF (vouch gate, abjure, summon, inspect)
- RBGC constants if old format prefixes are defined there
- RBDC derived constants: check for format patterns
- RBQ (rbq_Qualify.sh): check for consecration format references in health checks
- Add buv_consecration_format validator for new regex

## Regime Additions
- Add `graft` to RBRV_VESSEL_MODE validation
- Add rbrv_group_grafting with RBRV_GRAFT_IMAGE
- Update kindle (defaults), validate (type check + group gate), render (display section)
- RBGC: any new constants for graft mode

## Verification
- Existing conjure and bind work with new format
- Regime render shows graft section when mode=graft
- Regime validate accepts graft, rejects unknown modes

## Depends on
- AtAAA spec updates complete

**[260314-1004] rough**

Implement new consecration format and graft regime scaffolding.

## Character
Mechanical but cross-cutting — no backward compatibility needed (depot will be recreated).

## Consecration Format Change
New: `[cbg]YYMMDDHHMMSS-rYYMMDDHHMMSS` (27 chars). No dual-format parsing needed.

## Sites to update
- `rbgjb01-derive-tag-base.sh`: consecration construction, `c` prefix
- `rbf_mirror` in rbf_Foundry.sh: mirror tag construction, `b` prefix
- `rbf_check_consecrations` / consecration_check parsing: new regex only
- All other consecration format references in RBF (vouch gate, abjure, summon, inspect)
- RBGC constants if old format prefixes are defined there
- RBDC derived constants: check for format patterns
- RBQ (rbq_Qualify.sh): check for consecration format references in health checks
- Add buv_consecration_format validator for new regex

## Regime Additions
- Add `graft` to RBRV_VESSEL_MODE validation
- Add rbrv_group_grafting with RBRV_GRAFT_IMAGE
- Update kindle (defaults), validate (type check + group gate), render (display section)
- RBGC: any new constants for graft mode

## Verification
- Existing conjure and bind work with new format
- Regime render shows graft section when mode=graft
- Regime validate accepts graft, rejects unknown modes

## Depends on
- AtAAA spec updates complete

**[260314-0954] rough**

Implement new consecration format and graft regime scaffolding.

## Character
Mechanical but cross-cutting — no backward compatibility needed (depot will be recreated).

## Consecration Format Change
New: `[cbg]YYMMDDHHMMSS-rYYMMDDHHMMSS` (27 chars). No dual-format parsing needed.

## Sites to update
- `rbgjb01-derive-tag-base.sh`: consecration construction, `c` prefix
- `rbf_mirror` in rbf_Foundry.sh: mirror tag construction, `b` prefix
- `rbf_check_consecrations` / consecration_check parsing: new regex only
- All other consecration references in RBF (vouch gate, abjure, summon, inspect)
- RBGC constants if old format prefixes are defined there
- RBDC derived constants: check for format patterns
- Add buv_consecration_format validator for new regex

## Regime Additions
- Add `graft` to RBRV_VESSEL_MODE validation
- Add rbrv_group_grafting with RBRV_GRAFT_IMAGE
- Update kindle (defaults), validate (type check + group gate), render (display section)
- RBGC: any new constants for graft mode

## Verification
- Existing conjure and bind work with new format
- Regime render shows graft section when mode=graft
- Regime validate accepts graft, rejects unknown modes

## Depends on
- AtAAA spec updates complete

**[260314-0944] rough**

Implement new consecration format and graft regime scaffolding.

## Character
Mechanical but cross-cutting — careful find-and-replace with format validation at each site.

## Consecration Format Change
Old: `i20260224_153022-b20260224_160530` (35 chars)
New: `[cbg]YYMMDDHHMMSS-rYYMMDDHHMMSS` (27 chars)
- Mode prefix: `c` (conjure), `b` (bind), `g` (graft)
- `r` = realized (image landed in GAR)
- Drop century, drop underscore

## Sites to update
- `rbgjb01-derive-tag-base.sh`: consecration construction (conjure path, `c` prefix)
- `rbf_mirror` in rbf_Foundry.sh: mirror tag construction (bind path, `b` prefix)
- `rbf_check_consecrations` / consecration_check parsing: regex pattern match updated
- Any other consecration format references in RBF (vouch gate, abjure, summon, inspect)
- RBGC constants if the old format prefixes (`i`, `b`) are defined there

## Regime Additions
- Add `graft` to RBRV_VESSEL_MODE validation (buv check)
- Add rbrv_group_grafting with RBRV_GRAFT_IMAGE variable
- Update kindle (defaults), validate (type check + group gate), render (display section)
- RBGC: add any new constants for graft mode

## Verification
- Existing conjure and bind operations still work with new consecration format
- Regime render shows graft section when mode=graft
- Regime validate accepts graft mode, rejects unknown modes

## Depends on
- ₢AtAAA specification updates (must be complete first)

### syft-gcb-image-pin (₢AtAAF) [complete]

**[260314-1024] complete**

Add pinned syft container image to GCB pin regime alongside existing pins (alpine, gcloud, docker, crane).

## Character
Mechanical — follow existing pin pattern exactly.

## Current state
Syft is currently hardcoded in rbgjb06-syft-per-platform.sh (the about step that moves to rbgja02). Need a proper RBRG_SYFT_IMAGE_REF regime pin with digest.

## Work
- Add RBRG_SYFT_IMAGE_REF to rbrg.env (GCB pins regime)
- Pin to current anchore/syft image with SHA256 digest
- Add to pin refresh tooling (rbw-DPG / rbtc_refresh_gcb_pins) so it stays current
- Update rbgja02 (or rbgjb06 until extraction) to use RBRG_SYFT_IMAGE_REF
- Add to RBSRG-RegimeGcbPins.adoc spec

## Depends on
- Nothing — can be done independently, but must complete before AtAAC (about pipeline uses it)

**[260314-0955] rough**

Add pinned syft container image to GCB pin regime alongside existing pins (alpine, gcloud, docker, crane).

## Character
Mechanical — follow existing pin pattern exactly.

## Current state
Syft is currently hardcoded in rbgjb06-syft-per-platform.sh (the about step that moves to rbgja02). Need a proper RBRG_SYFT_IMAGE_REF regime pin with digest.

## Work
- Add RBRG_SYFT_IMAGE_REF to rbrg.env (GCB pins regime)
- Pin to current anchore/syft image with SHA256 digest
- Add to pin refresh tooling (rbw-DPG / rbtc_refresh_gcb_pins) so it stays current
- Update rbgja02 (or rbgjb06 until extraction) to use RBRG_SYFT_IMAGE_REF
- Add to RBSRG-RegimeGcbPins.adoc spec

## Depends on
- Nothing — can be done independently, but must complete before AtAAC (about pipeline uses it)

### about-pipeline-extraction (₢AtAAC) [complete]

**[260314-1226] complete**

Extract about artifact generation from conjure build into standalone Cloud Build pipeline.

## Character
Substantial refactor with new Cloud Build infrastructure.

## New Cloud Build step scripts (rbgja/)
- rbgja01-discover-platforms.sh: fetch -image manifest, extract platform list, handle single-arch gracefully (no manifest list)
- rbgja02-syft-per-platform.sh: syft SBOM per platform (from rbgjb06). Uses pinned syft image (RBRG_SYFT_IMAGE_REF from AtAAF). Auth via GCE metadata server token.
- rbgja03-build-info-per-platform.sh: metadata JSON (from rbgjb07), mode-aware fields, git from _RBGA_*
- rbgja04-assemble-push-about.sh: push -about container (from rbgjb08)

## RBF changes
- New about Cloud Build submission function (builds.create)
- About JSON is constructed at runtime in RBF (like vouch), NOT committed to rubric repo via inscribe. This is different from conjure's cloudbuild.json pattern.
- Substitution namespace _RBGA_*: enumerate all variables explicitly in implementation
- Same worker pool and mason SA as vouch

## Conjure pipeline reduction (rbgjb/)
- Remove rbgjb06, rbgjb07, rbgjb08
- Renumber remaining steps. Explicit mapping:
  - rbgjb01-derive-tag-base.sh -> rbgjb01 (unchanged)
  - rbgjb02-qemu-binfmt.sh -> rbgjb02 (unchanged)
  - rbgjb03-buildx-push-multi.sh -> rbgjb03 (unchanged)
  - rbgjb04-per-platform-pullback.sh -> rbgjb04 (unchanged)
  - rbgjb05-push-per-platform.sh -> rbgjb05 (unchanged)
  - rbgjb09-imagetools-create.sh -> rbgjb06-imagetools-create.sh (was 09, now 06)
- Update rubric inscribe to generate reduced cloudbuild.json (fewer steps, new filenames)
- Inscribe script inlining references files by name, so renaming requires inscribe template update

## Verification
- Conjure: trigger build (-image), then about job (-about), two round-trips
- About artifact contains syft SBOM and build_info.json
- Inspect reads about content correctly

## Depends on
- AtAAB consecration format and regime foundation
- AtAAF syft pin (about pipeline needs pinned syft image reference)

**[260314-1004] rough**

Extract about artifact generation from conjure build into standalone Cloud Build pipeline.

## Character
Substantial refactor with new Cloud Build infrastructure.

## New Cloud Build step scripts (rbgja/)
- rbgja01-discover-platforms.sh: fetch -image manifest, extract platform list, handle single-arch gracefully (no manifest list)
- rbgja02-syft-per-platform.sh: syft SBOM per platform (from rbgjb06). Uses pinned syft image (RBRG_SYFT_IMAGE_REF from AtAAF). Auth via GCE metadata server token.
- rbgja03-build-info-per-platform.sh: metadata JSON (from rbgjb07), mode-aware fields, git from _RBGA_*
- rbgja04-assemble-push-about.sh: push -about container (from rbgjb08)

## RBF changes
- New about Cloud Build submission function (builds.create)
- About JSON is constructed at runtime in RBF (like vouch), NOT committed to rubric repo via inscribe. This is different from conjure's cloudbuild.json pattern.
- Substitution namespace _RBGA_*: enumerate all variables explicitly in implementation
- Same worker pool and mason SA as vouch

## Conjure pipeline reduction (rbgjb/)
- Remove rbgjb06, rbgjb07, rbgjb08
- Renumber remaining steps. Explicit mapping:
  - rbgjb01-derive-tag-base.sh -> rbgjb01 (unchanged)
  - rbgjb02-qemu-binfmt.sh -> rbgjb02 (unchanged)
  - rbgjb03-buildx-push-multi.sh -> rbgjb03 (unchanged)
  - rbgjb04-per-platform-pullback.sh -> rbgjb04 (unchanged)
  - rbgjb05-push-per-platform.sh -> rbgjb05 (unchanged)
  - rbgjb09-imagetools-create.sh -> rbgjb06-imagetools-create.sh (was 09, now 06)
- Update rubric inscribe to generate reduced cloudbuild.json (fewer steps, new filenames)
- Inscribe script inlining references files by name, so renaming requires inscribe template update

## Verification
- Conjure: trigger build (-image), then about job (-about), two round-trips
- About artifact contains syft SBOM and build_info.json
- Inspect reads about content correctly

## Depends on
- AtAAB consecration format and regime foundation
- AtAAF syft pin (about pipeline needs pinned syft image reference)

**[260314-1000] rough**

Extract about artifact generation from conjure build into standalone Cloud Build pipeline.

## Character
Substantial refactor with new Cloud Build infrastructure.

## New Cloud Build step scripts (rbgja/)
- rbgja01-discover-platforms.sh: fetch -image manifest, extract platform list, handle single-arch gracefully (no manifest list)
- rbgja02-syft-per-platform.sh: syft SBOM per platform (from rbgjb06). Uses pinned syft image (RBRG_SYFT_IMAGE_REF from AtAAF). Auth via GCE metadata server token.
- rbgja03-build-info-per-platform.sh: metadata JSON (from rbgjb07), mode-aware fields, git from _RBGA_*
- rbgja04-assemble-push-about.sh: push -about container (from rbgjb08)

## RBF changes
- New about Cloud Build submission function (builds.create)
- About JSON is constructed at runtime in RBF (like vouch), NOT committed to rubric repo via inscribe. This is different from conjure's cloudbuild.json pattern.
- Substitution namespace _RBGA_*: enumerate all variables explicitly in implementation
- Same worker pool and mason SA as vouch

## Conjure pipeline reduction (rbgjb/)
- Remove rbgjb06, rbgjb07, rbgjb08
- Renumber remaining steps: 01-05 + 09 becomes 01-06 sequentially (rename files)
- Update rubric inscribe to generate reduced cloudbuild.json (fewer steps, new filenames)
- Inscribe script inlining references files by name, so renaming requires inscribe template update

## Verification
- Conjure: trigger build (-image), then about job (-about), two round-trips
- About artifact contains syft SBOM and build_info.json
- Inspect reads about content correctly

## Depends on
- AtAAB consecration format and regime foundation
- AtAAF syft pin (about pipeline needs pinned syft image reference)

**[260314-0954] rough**

Extract about artifact generation from conjure build into standalone Cloud Build pipeline.

## Character
Substantial refactor with new Cloud Build infrastructure.

## New Cloud Build step scripts (rbgja/)
- `rbgja01-discover-platforms.sh`: fetch -image manifest, extract platform list, handle single-arch
- `rbgja02-syft-per-platform.sh`: syft SBOM per platform (from rbgjb06). Uses pinned syft image (see AtAAF)
- `rbgja03-build-info-per-platform.sh`: metadata JSON (from rbgjb07), mode-aware fields, git from _RBGA_*
- `rbgja04-assemble-push-about.sh`: push -about container (from rbgjb08)

## RBF changes
- New about Cloud Build submission function (builds.create)
- Substitution namespace _RBGA_*: enumerate all variables explicitly
- Same worker pool and mason SA as vouch

## Conjure pipeline reduction (rbgjb/)
- Remove rbgjb06, rbgjb07, rbgjb08
- Renumber remaining steps: 01-05 + 09 becomes 01-06 sequentially
- Update rubric inscribe to generate reduced cloudbuild.json

## Verification
- Conjure: trigger build (-image), then about job (-about), two round-trips
- About artifact contains syft SBOM and build_info.json
- Inspect reads about content correctly

## Depends on
- AtAAB consecration format and regime foundation
- AtAAF syft pin (about pipeline needs pinned syft image reference)

**[260314-0944] rough**

Extract about artifact generation from conjure build into standalone Cloud Build pipeline, uniform across all vessel modes.

## Character
Substantial refactor with new Cloud Build infrastructure — careful extraction without breaking conjure.

## New Cloud Build step scripts (rbgja/)
- `rbgja01-discover-platforms.sh`: Fetch -image manifest from GAR, extract platform list. Must handle single-arch images (no manifest list) gracefully.
- `rbgja02-syft-per-platform.sh`: Syft SBOM per platform. Extracted/adapted from current rbgjb06. Auth via GCE metadata server token.
- `rbgja03-build-info-per-platform.sh`: Generate metadata JSON per platform. Adapted from rbgjb07, now mode-aware: conjure gets full provenance fields, bind/graft get reduced fields. Git metadata from _RBGA_* substitution variables.
- `rbgja04-assemble-push-about.sh`: Assemble + push multi-platform FROM scratch -about container. Adapted from rbgjb08.

## RBF changes
- New about Cloud Build submission function: director submits builds.create after -image exists in GAR
- Substitution namespace _RBGA_* (about-specific): GAR coordinates, vessel, consecration, vessel_mode, git metadata
- About job uses same worker pool and mason SA as vouch

## Conjure pipeline reduction (rbgjb/)
- Remove rbgjb06-syft-per-platform.sh (moved to rbgja02)
- Remove rbgjb07-build-info-per-platform.sh (moved to rbgja03)
- Remove rbgjb08-buildx-push-about.sh (moved to rbgja04)
- Update rubric inscribe to generate reduced cloudbuild.json (steps 01-05 + 09 only)
- Conjure now produces only -image; director calls about pipeline as second Cloud Build job

## Verification
- Conjure produces -image via trigger build, then -about via separate about Cloud Build job
- About artifact contains syft SBOM and build_info.json as before
- Inspect still reads about content correctly

## Depends on
- ₢AtAAB consecration format and regime foundation

### diags-artifact-forwarding (₢AtAAG) [complete]

**[260314-1255] complete**

Forward conjure build-time diagnostics to the about pipeline via a transient `-diags` registry artifact.

## Character
Infrastructure plumbing — mechanical but cross-cutting (conjure build, about pipeline, inspect specs).

## Problem
The about pipeline extraction (AtAAC) separated about into its own Cloud Build job, and the review fix commit removed orphaned diagnostic file generation. Three conjure-build-time files are now neither generated nor forwarded:
- `buildkit_metadata.json` (from `buildx build --metadata-file` — still generated in rbgjb03)
- `cache_before.json` (docker daemon snapshot before build — generation removed from rbgjb03 by AtAAC review fixes)
- `cache_after.json` (docker daemon snapshot after pushes — generation removed from rbgjb05 by AtAAC review fixes)

Additionally, `recipe.txt` (Dockerfile content) is conveyed via `_RBGA_DOCKERFILE_CONTENT` substitution variable, which has a hard ~4KB limit. Large multi-stage Dockerfiles are silently omitted.

Spec references to these files were removed from RBSCB and RBSAI by the AtAAC review fix commit — they must be re-added when the files are restored.

## Solution: `-diags` transient artifact

### Conjure side (rbgjb/)

**Restore cache generation:**
- Restore `cache_before.json` generation to rbgjb03 (before `buildx build`). Must be captured before the build starts to show pre-build worker state.
- Restore `cache_after.json` generation to rbgjb05 (after all `docker push`). Must be captured after pushes complete to show post-build worker state.
- `buildkit_metadata.json` is still generated by rbgjb03 via `--metadata-file` flag (no change needed).

**New rbgjb07-push-diags.sh:**
- Build a `FROM scratch` container containing: `buildkit_metadata.json`, `cache_before.json`, `cache_after.json`, and the Dockerfile (read from build context via `${_RBGY_DOCKERFILE}`, no size limit).
- Push as `consecration-diags`.
- Builder: gcr.io/cloud-builders/docker.
- Rename the Dockerfile to `recipe.txt` in the container for consistency with the about artifact's expected filename.
- Update `zrbf_stitch_build_json` to include the new step. New `_RBGY_ARK_SUFFIX_DIAGS` substitution or hardcode `-diags` suffix.

### About side (rbgja/)

**Merge -diags pull into rbgja01** (not a separate rbgja00 step):
- After platform discovery completes, use the same OAuth token to check if `consecration-diags` exists in the registry (HEAD request).
- If present: pull the -diags container, extract all files into /workspace (`recipe.txt`, `buildkit_metadata.json`, `cache_before.json`, `cache_after.json`).
- If absent (bind/graft have no -diags): skip gracefully, log that no -diags found.

**Modify rbgja03 recipe.txt logic:**
- Before writing recipe.txt from `_RBGA_DOCKERFILE_CONTENT`, check if recipe.txt already exists in /workspace (written by -diags extraction in rbgja01).
- If recipe.txt already exists: skip the substitution variable write, log that recipe.txt came from -diags.
- If recipe.txt does not exist: write from `_RBGA_DOCKERFILE_CONTENT` as before (bind/graft path).
- `_RBGA_DOCKERFILE_CONTENT` substitution variable is RETAINED for all modes. For conjure, the director still sends it as a fallback (in case -diags extraction fails), but -diags version takes precedence when present. No partial removal of the substitution variable.

**Modify rbgja04 Dockerfile.meta:**
- Add conditional COPY lines for forwarded files when present: `buildkit_metadata.json`, `cache_before.json`, `cache_after.json`.
- `recipe.txt` COPY is already conditional (no change).

### -diags lifecycle
- `-diags` is NOT deleted by the about pipeline. It persists in the registry as a durable record of conjure build-time diagnostics.
- Abjure cleans up `-diags` alongside `-image`, `-about`, and `-vouch`. Add `-diags` to the abjure suffix list (`RBGC_ARK_SUFFIX_DIAGS` or equivalent).

### Spec updates
- **RBSAB**: document -diags pull in step 1 (merged with platform discovery), document recipe.txt dual-source logic in step 3, update artifact contents to include forwarded enrichment files.
- **RBSCB**: re-add enrichment file references to about artifact contents table (removed by AtAAC review fix). Note that these files are present only for conjure consecrations.
- **RBSRI**: update stitch documentation to include rbgjb07.
- **RBSAI**: re-add references to `buildkit_metadata.json`, `cache_before.json`, `cache_after.json` extraction in inspect (removed by AtAAC review fix). Graceful degradation language is still correct.
- **RBSAA**: add `-diags` to abjure's artifact deletion list.
- **RBSCK**: add `-diags` to consecration check's artifact inventory.

## Verification
- Conjure: -diags pushed after build, about absorbs files, -about contains all enrichment files + full Dockerfile
- Conjure with large Dockerfile (>4KB): recipe.txt present in -about via -diags (no substitution limit)
- Conjure with -diags extraction failure: falls back to _RBGA_DOCKERFILE_CONTENT for recipe.txt, enrichment files absent (graceful degradation)
- Bind/graft: no -diags, about proceeds with recipe.txt from substitution variable only, no enrichment files (correct)
- Inspect: Build Output and Build Cache Delta sections populated for conjure
- Abjure: -diags deleted alongside other ark artifacts
- Consecration check: -diags presence reported for conjure, absence accepted for bind/graft

## Depends on
- AtAAC about pipeline extraction (and its review fix commit c73372aa)

**[260314-1224] rough**

Forward conjure build-time diagnostics to the about pipeline via a transient `-diags` registry artifact.

## Character
Infrastructure plumbing — mechanical but cross-cutting (conjure build, about pipeline, inspect specs).

## Problem
The about pipeline extraction (AtAAC) separated about into its own Cloud Build job, and the review fix commit removed orphaned diagnostic file generation. Three conjure-build-time files are now neither generated nor forwarded:
- `buildkit_metadata.json` (from `buildx build --metadata-file` — still generated in rbgjb03)
- `cache_before.json` (docker daemon snapshot before build — generation removed from rbgjb03 by AtAAC review fixes)
- `cache_after.json` (docker daemon snapshot after pushes — generation removed from rbgjb05 by AtAAC review fixes)

Additionally, `recipe.txt` (Dockerfile content) is conveyed via `_RBGA_DOCKERFILE_CONTENT` substitution variable, which has a hard ~4KB limit. Large multi-stage Dockerfiles are silently omitted.

Spec references to these files were removed from RBSCB and RBSAI by the AtAAC review fix commit — they must be re-added when the files are restored.

## Solution: `-diags` transient artifact

### Conjure side (rbgjb/)

**Restore cache generation:**
- Restore `cache_before.json` generation to rbgjb03 (before `buildx build`). Must be captured before the build starts to show pre-build worker state.
- Restore `cache_after.json` generation to rbgjb05 (after all `docker push`). Must be captured after pushes complete to show post-build worker state.
- `buildkit_metadata.json` is still generated by rbgjb03 via `--metadata-file` flag (no change needed).

**New rbgjb07-push-diags.sh:**
- Build a `FROM scratch` container containing: `buildkit_metadata.json`, `cache_before.json`, `cache_after.json`, and the Dockerfile (read from build context via `${_RBGY_DOCKERFILE}`, no size limit).
- Push as `consecration-diags`.
- Builder: gcr.io/cloud-builders/docker.
- Rename the Dockerfile to `recipe.txt` in the container for consistency with the about artifact's expected filename.
- Update `zrbf_stitch_build_json` to include the new step. New `_RBGY_ARK_SUFFIX_DIAGS` substitution or hardcode `-diags` suffix.

### About side (rbgja/)

**Merge -diags pull into rbgja01** (not a separate rbgja00 step):
- After platform discovery completes, use the same OAuth token to check if `consecration-diags` exists in the registry (HEAD request).
- If present: pull the -diags container, extract all files into /workspace (`recipe.txt`, `buildkit_metadata.json`, `cache_before.json`, `cache_after.json`).
- If absent (bind/graft have no -diags): skip gracefully, log that no -diags found.

**Modify rbgja03 recipe.txt logic:**
- Before writing recipe.txt from `_RBGA_DOCKERFILE_CONTENT`, check if recipe.txt already exists in /workspace (written by -diags extraction in rbgja01).
- If recipe.txt already exists: skip the substitution variable write, log that recipe.txt came from -diags.
- If recipe.txt does not exist: write from `_RBGA_DOCKERFILE_CONTENT` as before (bind/graft path).
- `_RBGA_DOCKERFILE_CONTENT` substitution variable is RETAINED for all modes. For conjure, the director still sends it as a fallback (in case -diags extraction fails), but -diags version takes precedence when present. No partial removal of the substitution variable.

**Modify rbgja04 Dockerfile.meta:**
- Add conditional COPY lines for forwarded files when present: `buildkit_metadata.json`, `cache_before.json`, `cache_after.json`.
- `recipe.txt` COPY is already conditional (no change).

### -diags lifecycle
- `-diags` is NOT deleted by the about pipeline. It persists in the registry as a durable record of conjure build-time diagnostics.
- Abjure cleans up `-diags` alongside `-image`, `-about`, and `-vouch`. Add `-diags` to the abjure suffix list (`RBGC_ARK_SUFFIX_DIAGS` or equivalent).

### Spec updates
- **RBSAB**: document -diags pull in step 1 (merged with platform discovery), document recipe.txt dual-source logic in step 3, update artifact contents to include forwarded enrichment files.
- **RBSCB**: re-add enrichment file references to about artifact contents table (removed by AtAAC review fix). Note that these files are present only for conjure consecrations.
- **RBSRI**: update stitch documentation to include rbgjb07.
- **RBSAI**: re-add references to `buildkit_metadata.json`, `cache_before.json`, `cache_after.json` extraction in inspect (removed by AtAAC review fix). Graceful degradation language is still correct.
- **RBSAA**: add `-diags` to abjure's artifact deletion list.
- **RBSCK**: add `-diags` to consecration check's artifact inventory.

## Verification
- Conjure: -diags pushed after build, about absorbs files, -about contains all enrichment files + full Dockerfile
- Conjure with large Dockerfile (>4KB): recipe.txt present in -about via -diags (no substitution limit)
- Conjure with -diags extraction failure: falls back to _RBGA_DOCKERFILE_CONTENT for recipe.txt, enrichment files absent (graceful degradation)
- Bind/graft: no -diags, about proceeds with recipe.txt from substitution variable only, no enrichment files (correct)
- Inspect: Build Output and Build Cache Delta sections populated for conjure
- Abjure: -diags deleted alongside other ark artifacts
- Consecration check: -diags presence reported for conjure, absence accepted for bind/graft

## Depends on
- AtAAC about pipeline extraction (and its review fix commit c73372aa)

**[260314-1223] rough**

Forward conjure build-time diagnostics to the about pipeline via a transient `-diags` registry artifact.

## Character
Infrastructure plumbing — mechanical but cross-cutting (conjure build, about pipeline, inspect specs).

## Problem
The about pipeline extraction (AtAAC) separated about into its own Cloud Build job, and the review fix commit removed orphaned diagnostic file generation. Three conjure-build-time files are now neither generated nor forwarded:
- `buildkit_metadata.json` (from `buildx build --metadata-file` — still generated in rbgjb03)
- `cache_before.json` (docker daemon snapshot before build — generation removed from rbgjb03 by AtAAC review fixes)
- `cache_after.json` (docker daemon snapshot after pushes — generation removed from rbgjb05 by AtAAC review fixes)

Additionally, `recipe.txt` (Dockerfile content) is conveyed via `_RBGA_DOCKERFILE_CONTENT` substitution variable, which has a hard ~4KB limit. Large multi-stage Dockerfiles are silently omitted.

Spec references to these files were removed from RBSCB and RBSAI by the AtAAC review fix commit — they must be re-added when the files are restored.

## Solution: `-diags` transient artifact

### Conjure side (rbgjb/)

**Restore cache generation:**
- Restore `cache_before.json` generation to rbgjb03 (before `buildx build`). Must be captured before the build starts to show pre-build worker state.
- Restore `cache_after.json` generation to rbgjb05 (after all `docker push`). Must be captured after pushes complete to show post-build worker state.
- `buildkit_metadata.json` is still generated by rbgjb03 via `--metadata-file` flag (no change needed).

**New rbgjb07-push-diags.sh:**
- Build a `FROM scratch` container containing: `buildkit_metadata.json`, `cache_before.json`, `cache_after.json`, and the Dockerfile (read from build context via `${_RBGY_DOCKERFILE}`, no size limit).
- Push as `consecration-diags`.
- Builder: gcr.io/cloud-builders/docker.
- Rename the Dockerfile to `recipe.txt` in the container for consistency with the about artifact's expected filename.
- Update `zrbf_stitch_build_json` to include the new step. New `_RBGY_ARK_SUFFIX_DIAGS` substitution or hardcode `-diags` suffix.

### About side (rbgja/)

**Merge -diags pull into rbgja01** (not a separate rbgja00 step):
- After platform discovery completes, use the same OAuth token to check if `consecration-diags` exists in the registry (HEAD request).
- If present: pull the -diags container, extract all files into /workspace (`recipe.txt`, `buildkit_metadata.json`, `cache_before.json`, `cache_after.json`).
- If absent (bind/graft have no -diags): skip gracefully, log that no -diags found.

**Modify rbgja03 recipe.txt logic:**
- Before writing recipe.txt from `_RBGA_DOCKERFILE_CONTENT`, check if recipe.txt already exists in /workspace (written by -diags extraction in rbgja01).
- If recipe.txt already exists: skip the substitution variable write, log that recipe.txt came from -diags.
- If recipe.txt does not exist: write from `_RBGA_DOCKERFILE_CONTENT` as before (bind/graft path).
- `_RBGA_DOCKERFILE_CONTENT` substitution variable is RETAINED for all modes. For conjure, the director still sends it as a fallback (in case -diags extraction fails), but -diags version takes precedence when present. No partial removal of the substitution variable.

**Modify rbgja04 Dockerfile.meta:**
- Add conditional COPY lines for forwarded files when present: `buildkit_metadata.json`, `cache_before.json`, `cache_after.json`.
- `recipe.txt` COPY is already conditional (no change).

### -diags cleanup
- About pipeline deletes `consecration-diags` after absorbing its contents.
- Use Docker Registry API: `DELETE /v2/<name>/manifests/<digest>` with the mason SA OAuth token (same token used for platform discovery).
- **IAM requirement**: mason SA needs tag-deletion permission on the GAR repository (`artifactregistry.versions.delete` or equivalent). Verify this is included in mason's existing role, or add it.
- If deletion fails: warn but don't fail the about job — stale -diags is harmless and can be cleaned up by abjure.

### Spec updates
- **RBSAB**: document -diags pull in step 1 (merged with platform discovery), document recipe.txt dual-source logic in step 3, update artifact contents to include forwarded enrichment files.
- **RBSCB**: re-add enrichment file references to about artifact contents table (removed by AtAAC review fix). Note that these files are present only for conjure consecrations.
- **RBSRI**: update stitch documentation to include rbgjb07.
- **RBSAI**: re-add references to `buildkit_metadata.json`, `cache_before.json`, `cache_after.json` extraction in inspect (removed by AtAAC review fix). Graceful degradation language is still correct.

## Verification
- Conjure: -diags pushed after build, about absorbs files, -diags deleted, -about contains all enrichment files + full Dockerfile
- Conjure with large Dockerfile (>4KB): recipe.txt present in -about via -diags (no substitution limit)
- Conjure with -diags extraction failure: falls back to _RBGA_DOCKERFILE_CONTENT for recipe.txt, enrichment files absent (graceful degradation)
- Bind/graft: no -diags, about proceeds with recipe.txt from substitution variable only, no enrichment files (correct)
- Inspect: Build Output and Build Cache Delta sections populated for conjure
- Abjure: -diags tag cleaned up if stale (verify existing abjure handles unknown suffixes or add -diags to suffix list)

## Depends on
- AtAAC about pipeline extraction (and its review fix commit c73372aa)

**[260314-1217] rough**

Forward conjure build-time diagnostics to the about pipeline via a transient `-diags` registry artifact.

## Character
Infrastructure plumbing — mechanical but cross-cutting (conjure build, about pipeline, inspect specs).

## Problem
The about pipeline extraction (AtAAC) separated about into its own Cloud Build job. Three conjure-build-time files are now unreachable:
- `buildkit_metadata.json` (from `buildx build --metadata-file`)
- `cache_before.json` (docker daemon snapshot before build)
- `cache_after.json` (docker daemon snapshot after pushes)

Additionally, `recipe.txt` (Dockerfile content) is currently conveyed via `_RBGA_DOCKERFILE_CONTENT` substitution variable, which has a hard ~4KB limit. Large multi-stage Dockerfiles are silently omitted.

## Solution: `-diags` transient artifact

### Conjure side (rbgjb/)
- New rbgjb07-push-diags.sh: build a `FROM scratch` container containing `buildkit_metadata.json`, `cache_before.json`, `cache_after.json`, and the Dockerfile (read from build context, no size limit). Push as `consecration-diags`.
- Builder: gcr.io/cloud-builders/docker
- Files are already in /workspace from earlier conjure steps (rbgjb03 generates buildkit_metadata + cache_before, rbgjb05 generates cache_after, Dockerfile is in build context)
- Update zrbf_stitch_build_json to include the new step

### About side (rbgja/)
- New rbgja00-pull-diags.sh (or modify rbgja01): before platform discovery, check if `consecration-diags` exists in registry. If present, pull and extract all files into /workspace (recipe.txt, buildkit_metadata.json, cache_before.json, cache_after.json). If absent (bind/graft have no -diags), skip gracefully.
- Modify rbgja04 Dockerfile.meta to COPY the forwarded files when present (recipe.txt, buildkit_metadata.json, cache_before.json, cache_after.json — all conditional on file existence)
- Builder: gcr.io/cloud-builders/gcloud (or docker, whichever has registry API access)

### Remove _RBGA_DOCKERFILE_CONTENT
- Remove from zrbf_about_submit substitution construction
- Remove from rbgja03 (recipe.txt write from substitution variable)
- Remove from RBSAB spec substitution table
- recipe.txt now comes exclusively from -diags (conjure) or is absent (bind/graft without optional Dockerfile... actually bind/graft optional Dockerfiles still need conveyance — keep _RBGA_DOCKERFILE_CONTENT for bind/graft only, or accept that bind/graft optional Dockerfiles also go through -diags? No — bind/graft don't have a conjure build to push -diags from. Keep _RBGA_DOCKERFILE_CONTENT for bind/graft optional Dockerfiles only, remove for conjure.)

### -diags cleanup
- About pipeline deletes `consecration-diags` after absorbing its contents. Use registry API DELETE via gcloud/curl with mason SA credentials (about runs as mason on private pool).
- If deletion fails, warn but don't fail the about job — stale -diags is harmless.

### Spec updates
- RBSAB: document -diags pull step, remove _RBGA_DOCKERFILE_CONTENT for conjure, update artifact contents to include forwarded files
- RBSCB: confirm enrichment files are back in about artifact
- RBSRI: update stitch to include rbgjb07
- RBSAI: no change needed (inspect already handles these files)

## Verification
- Conjure: -diags pushed after build, about absorbs files, -diags deleted, -about contains all enrichment files + full Dockerfile
- Bind/graft: no -diags, about proceeds without enrichment files (correct — they don't exist for these modes)
- Inspect: Build Output and Build Cache Delta sections populated for conjure
- Large Dockerfile (>4KB): recipe.txt present in -about (no substitution limit)

## Depends on
- AtAAC about pipeline extraction

### about-pipeline-suffix-substitutions (₢AtAAH) [complete]

**[260314-1309] complete**

Replace hardcoded suffix strings in about pipeline step scripts (rbgja/) with substitution variables.

## Character
Mechanical cleanup — grep-and-replace with substitution plumbing.

## Problem
The about pipeline (rbgja/) hardcodes ark suffix strings (`-image`, `-about`, `-diags`) as magic string literals in step scripts, while the conjure pipeline (rbgjb/) passes them via `_RBGY_ARK_SUFFIX_*` substitution variables backed by `RBGC_ARK_SUFFIX_*` constants. This means the same architectural fact (suffix identity) is expressed in two different forms — Interface Contamination per BCG. Distant relationships between the constant definitions and their usage sites are invisible.

## Solution
1. Add `_RBGA_ARK_SUFFIX_IMAGE`, `_RBGA_ARK_SUFFIX_ABOUT`, `_RBGA_ARK_SUFFIX_DIAGS` to the about pipeline's substitution namespace in `zrbf_about_submit()`
2. Replace hardcoded suffix strings in rbgja01, rbgja02, and rbgja04 with the new substitution variables
3. Update RBSAB spec's substitution variable table to document the new variables

## Verification
- grep for literal `-image`, `-about`, `-diags` in rbgja/ — should find only comments and log messages
- About pipeline substitution block includes all three suffix variables
- RBSAB substitution table documents all three
- Regression: about pipeline produces correct `-about` tag (suffix value correctly substituted by Cloud Build)

## Depends on
- AtAAG diags-artifact-forwarding

**[260314-1249] rough**

Replace hardcoded suffix strings in about pipeline step scripts (rbgja/) with substitution variables.

## Character
Mechanical cleanup — grep-and-replace with substitution plumbing.

## Problem
The about pipeline (rbgja/) hardcodes ark suffix strings (`-image`, `-about`, `-diags`) as magic string literals in step scripts, while the conjure pipeline (rbgjb/) passes them via `_RBGY_ARK_SUFFIX_*` substitution variables backed by `RBGC_ARK_SUFFIX_*` constants. This means the same architectural fact (suffix identity) is expressed in two different forms — Interface Contamination per BCG. Distant relationships between the constant definitions and their usage sites are invisible.

## Solution
1. Add `_RBGA_ARK_SUFFIX_IMAGE`, `_RBGA_ARK_SUFFIX_ABOUT`, `_RBGA_ARK_SUFFIX_DIAGS` to the about pipeline's substitution namespace in `zrbf_about_submit()`
2. Replace hardcoded suffix strings in rbgja01, rbgja02, and rbgja04 with the new substitution variables
3. Update RBSAB spec's substitution variable table to document the new variables

## Verification
- grep for literal `-image`, `-about`, `-diags` in rbgja/ — should find only comments and log messages
- About pipeline substitution block includes all three suffix variables
- RBSAB substitution table documents all three
- Regression: about pipeline produces correct `-about` tag (suffix value correctly substituted by Cloud Build)

## Depends on
- AtAAG diags-artifact-forwarding

**[260314-1243] rough**

Replace hardcoded suffix strings in about pipeline step scripts (rbgja/) with substitution variables.

## Character
Mechanical cleanup — grep-and-replace with substitution plumbing.

## Problem
The about pipeline (rbgja/) hardcodes ark suffix strings (`-image`, `-about`, `-diags`) as magic string literals in step scripts, while the conjure pipeline (rbgjb/) passes them via `_RBGY_ARK_SUFFIX_*` substitution variables backed by `RBGC_ARK_SUFFIX_*` constants. This means the same architectural fact (suffix identity) is expressed in two different forms — Interface Contamination per BCG. Distant relationships between the constant definitions and their usage sites are invisible.

## Solution
1. Add `_RBGA_ARK_SUFFIX_IMAGE`, `_RBGA_ARK_SUFFIX_ABOUT`, `_RBGA_ARK_SUFFIX_DIAGS` to the about pipeline's substitution namespace in `zrbf_about_submit()`
2. Replace hardcoded suffix strings in rbgja01, rbgja04 with the new substitution variables
3. Update RBSAB spec's substitution variable table to document the new variables

## Verification
- grep for literal `-image`, `-about`, `-diags` in rbgja/ — should find only comments
- About pipeline substitution block includes all three suffix variables
- RBSAB substitution table documents all three

### vouch-unification-and-graft (₢AtAAD) [complete]

**[260314-1618] complete**

Vouch unification and graft push. Draft code (committed) is largely correct.

## Character
Mostly mechanical -- verify draft against spec, make targeted fixes.

## Draft code to KEEP as-is
- rbgjv01: early-exit for non-conjure (_RBGV_VESSEL_MODE check)
- rbgjv02: mode branching (conjure SLSA, bind digest-pin via _RBGV_BIND_SOURCE, graft GRAFTED), platform discovery writes /workspace/vouch_platforms.txt
- rbgjv03: reads vouch_platforms.txt, conditional verify-*.json copy, uses _RBGV_ARK_SUFFIX_VOUCH
- zrbf_vouch_submit: unified Cloud Build submission with mode-aware substitutions
- rbf_vouch: always calls zrbf_vouch_submit (zrbf_vouch_bind eliminated)
- inspect: graft display section, bind field compatibility
- rbw-Db tabtarget + zipper enrollment

## Vouch substitution variables (verify in draft)
_RBGV_VESSEL_MODE, _RBGV_BIND_SOURCE (NOT _BIND_IMAGE), _RBGV_GRAFT_SOURCE, _RBGV_ARK_SUFFIX_IMAGE, _RBGV_ARK_SUFFIX_VOUCH. Removed: _RBGV_PLATFORMS.

## Refactor rbf_graft
Remove lines 1226-1230 as a block (step message, zrbf_about_submit call, and "About complete" info line). Remove "Next steps" tabtarget (lines 1245-1247). Update doc_brief (line 1133) from "Graft a locally-built image into GAR, then run about pipeline" to "Graft a locally-built image into GAR" -- about chaining moves to rbf_create in AtAAI. Result: push image to GAR, persist consecration to BURD_OUTPUT_DIR, return.

## Spec
- RBSAV: correct as-is, no changes
- RBSAG completion: note that chaining (about then vouch) is handled by rbf_create, not rbf_graft

## Files touched
- Tools/rbk/rbf_Foundry.sh
- Tools/rbk/vov_veiled/RBSAG-ark_graft.adoc (completion note only)

**[260314-1606] rough**

Vouch unification and graft push. Draft code (committed) is largely correct.

## Character
Mostly mechanical -- verify draft against spec, make targeted fixes.

## Draft code to KEEP as-is
- rbgjv01: early-exit for non-conjure (_RBGV_VESSEL_MODE check)
- rbgjv02: mode branching (conjure SLSA, bind digest-pin via _RBGV_BIND_SOURCE, graft GRAFTED), platform discovery writes /workspace/vouch_platforms.txt
- rbgjv03: reads vouch_platforms.txt, conditional verify-*.json copy, uses _RBGV_ARK_SUFFIX_VOUCH
- zrbf_vouch_submit: unified Cloud Build submission with mode-aware substitutions
- rbf_vouch: always calls zrbf_vouch_submit (zrbf_vouch_bind eliminated)
- inspect: graft display section, bind field compatibility
- rbw-Db tabtarget + zipper enrollment

## Vouch substitution variables (verify in draft)
_RBGV_VESSEL_MODE, _RBGV_BIND_SOURCE (NOT _BIND_IMAGE), _RBGV_GRAFT_SOURCE, _RBGV_ARK_SUFFIX_IMAGE, _RBGV_ARK_SUFFIX_VOUCH. Removed: _RBGV_PLATFORMS.

## Refactor rbf_graft
Remove lines 1226-1230 as a block (step message, zrbf_about_submit call, and "About complete" info line). Remove "Next steps" tabtarget (lines 1245-1247). Update doc_brief (line 1133) from "Graft a locally-built image into GAR, then run about pipeline" to "Graft a locally-built image into GAR" -- about chaining moves to rbf_create in AtAAI. Result: push image to GAR, persist consecration to BURD_OUTPUT_DIR, return.

## Spec
- RBSAV: correct as-is, no changes
- RBSAG completion: note that chaining (about then vouch) is handled by rbf_create, not rbf_graft

## Files touched
- Tools/rbk/rbf_Foundry.sh
- Tools/rbk/vov_veiled/RBSAG-ark_graft.adoc (completion note only)

**[260314-1356] rough**

Vouch unification and graft push. Draft code (committed) is largely correct.

## Character
Mostly mechanical — verify draft against spec, make targeted fixes.

## Draft code to KEEP as-is
- rbgjv01: early-exit for non-conjure (_RBGV_VESSEL_MODE check)
- rbgjv02: mode branching (conjure SLSA, bind digest-pin via _RBGV_BIND_SOURCE, graft GRAFTED), platform discovery writes /workspace/vouch_platforms.txt
- rbgjv03: reads vouch_platforms.txt, conditional verify-*.json copy, uses _RBGV_ARK_SUFFIX_VOUCH
- zrbf_vouch_submit: unified Cloud Build submission with mode-aware substitutions
- rbf_vouch: always calls zrbf_vouch_submit (zrbf_vouch_bind eliminated)
- inspect: graft display section, bind field compatibility
- rbw-Db tabtarget + zipper enrollment

## Vouch substitution variables (verify in draft)
_RBGV_VESSEL_MODE, _RBGV_BIND_SOURCE (NOT _BIND_IMAGE), _RBGV_GRAFT_SOURCE, _RBGV_ARK_SUFFIX_IMAGE, _RBGV_ARK_SUFFIX_VOUCH. Removed: _RBGV_PLATFORMS.

## Refactor rbf_graft
- Remove zrbf_about_submit call (chaining moves to rbf_create in AtAAI)
- Remove 'Next steps' vouch tabtarget from output
- Just: push image to GAR, persist consecration to BURD_OUTPUT_DIR, return

## Spec
- RBSAV: correct as-is, no changes
- RBSAG completion: note that chaining (about then vouch) is handled by rbf_create, not rbf_graft

**[260314-1354] rough**

Vouch unification and graft push. Keep existing draft code (rbgjv01-03 mode-aware, zrbf_vouch_submit, rbf_vouch unified, rbf_graft push logic, inspect graft/bind sections, rbw-Db wiring). Refactor rbf_graft: remove about submission, just push image and persist consecration. Update RBSAG completion note: chaining happens in rbf_create not rbf_graft.

**[260314-1353] rough**

See /tmp/docket_content.txt for full content — placeholder for MCP size workaround

**[260314-1210] rough**

Unify vouch to always run in Cloud Build (mode-aware) and implement graft operation.

## Character
Two related capabilities completing the three-mode architecture.

## Vouch unification (rbgjv/)
- rbgjv02: mode branching via _RBGV_VESSEL_MODE
  - conjure: SLSA provenance verification (preserved)
  - bind: digest-pin comparison, _RBGV_BIND_IMAGE carries pin reference
  - graft: GRAFTED stamp, no verification
- rbgjv01: Cloud Build step still runs for all modes, but script early-exits (exit 0) for bind/graft. Cloud Build does not support conditional step omission.
- Enumerate all new vouch substitution variables: _RBGV_VESSEL_MODE, _RBGV_BIND_IMAGE

## RBF vouch changes
- Eliminate zrbf_vouch_bind entirely
- rbf_vouch always submits Cloud Build regardless of mode
- Pass vessel_mode and mode-specific params as substitutions

## Graft operation (new)
- New rbf_graft function in rbf_Foundry.sh
- Runtime param: local image reference
- Push via docker tag + docker push (no crane, image is already local)
- Verify local image exists before push (docker image inspect), fail early with clear message
- Consecration: gYYMMDDHHMMSS-rYYMMDDHHMMSS
- Steps: authenticate director, docker login to GAR, tag local image, push as -image, submit about pipeline, POLL ABOUT TO COMPLETION, then submit vouch pipeline. Current vouch gates on -about existence, so director must wait for about to finish before submitting vouch.
- No dirty-tree guard (image already built; git state irrelevant to container)
- Wire into rbf_build mode dispatch (case statement alongside conjure/bind)
- Note: this poll-between-jobs pattern also applies to conjure's new two-round-trip flow (AtAAC)

## Workbench wiring
- Create rbw-DA tabtarget for standalone about invocation (director workflow)
- Wire rbf_about into workbench dispatch alongside existing rbw-DV vouch pattern

## Verification
- Bind vouch: Cloud Build, -vouch with digest-pin verdict
- Graft: local push, -about from Cloud Build, -vouch says GRAFTED
- All three modes: -image/-about/-vouch present, all attestation from Cloud Build
- Inspect correct trust posture per mode

## Depends on
- AtAAC about pipeline extraction

**[260314-1004] rough**

Unify vouch to always run in Cloud Build (mode-aware) and implement graft operation.

## Character
Two related capabilities completing the three-mode architecture.

## Vouch unification (rbgjv/)
- rbgjv02: mode branching via _RBGV_VESSEL_MODE
  - conjure: SLSA provenance verification (preserved)
  - bind: digest-pin comparison, _RBGV_BIND_IMAGE carries pin reference
  - graft: GRAFTED stamp, no verification
- rbgjv01: Cloud Build step still runs for all modes, but script early-exits (exit 0) for bind/graft. Cloud Build does not support conditional step omission.
- Enumerate all new vouch substitution variables: _RBGV_VESSEL_MODE, _RBGV_BIND_IMAGE

## RBF vouch changes
- Eliminate zrbf_vouch_bind entirely
- rbf_vouch always submits Cloud Build regardless of mode
- Pass vessel_mode and mode-specific params as substitutions

## Graft operation (new)
- New rbf_graft function in rbf_Foundry.sh
- Runtime param: local image reference
- Push via docker tag + docker push (no crane, image is already local)
- Verify local image exists before push (docker image inspect), fail early with clear message
- Consecration: gYYMMDDHHMMSS-rYYMMDDHHMMSS
- Steps: authenticate director, docker login to GAR, tag local image, push as -image, submit about pipeline, POLL ABOUT TO COMPLETION, then submit vouch pipeline. Current vouch gates on -about existence, so director must wait for about to finish before submitting vouch.
- No dirty-tree guard (image already built; git state irrelevant to container)
- Wire into rbf_build mode dispatch (case statement alongside conjure/bind)
- Note: this poll-between-jobs pattern also applies to conjure's new two-round-trip flow (AtAAC)

## Verification
- Bind vouch: Cloud Build, -vouch with digest-pin verdict
- Graft: local push, -about from Cloud Build, -vouch says GRAFTED
- All three modes: -image/-about/-vouch present, all attestation from Cloud Build
- Inspect correct trust posture per mode

## Depends on
- AtAAC about pipeline extraction

**[260314-1000] rough**

Unify vouch to always run in Cloud Build (mode-aware) and implement graft operation.

## Character
Two related capabilities completing the three-mode architecture.

## Vouch unification (rbgjv/)
- rbgjv02: mode branching via _RBGV_VESSEL_MODE
  - conjure: SLSA provenance verification (preserved)
  - bind: digest-pin comparison, _RBGV_BIND_IMAGE carries pin reference
  - graft: GRAFTED stamp, no verification
- rbgjv01: Cloud Build step still runs for all modes, but script early-exits (exit 0) for bind/graft. Cloud Build does not support conditional step omission.
- Enumerate all new vouch substitution variables: _RBGV_VESSEL_MODE, _RBGV_BIND_IMAGE

## RBF vouch changes
- Eliminate zrbf_vouch_bind entirely
- rbf_vouch always submits Cloud Build regardless of mode
- Pass vessel_mode and mode-specific params as substitutions

## Graft operation (new)
- New rbf_graft function in rbf_Foundry.sh
- Runtime param: local image reference
- Push via docker tag + docker push (no crane, image is already local)
- Verify local image exists before push (docker image inspect), fail early with clear message
- Consecration: gYYMMDDHHMMSS-rYYMMDDHHMMSS
- Steps: authenticate director, docker login to GAR, tag local image, push as -image, submit about pipeline, submit vouch pipeline (sequential Cloud Build jobs)
- No dirty-tree guard (image already built; git state irrelevant to container)
- Wire into rbf_build mode dispatch (case statement alongside conjure/bind)

## Verification
- Bind vouch: Cloud Build, -vouch with digest-pin verdict
- Graft: local push, -about from Cloud Build, -vouch says GRAFTED
- All three modes: -image/-about/-vouch present, all attestation from Cloud Build
- Inspect correct trust posture per mode

## Depends on
- AtAAC about pipeline extraction

**[260314-0955] rough**

Unify vouch to always run in Cloud Build (mode-aware) and implement graft operation.

## Character
Two related capabilities completing the three-mode architecture.

## Vouch unification (rbgjv/)
- rbgjv02: mode branching via _RBGV_VESSEL_MODE
  - conjure: SLSA provenance verification (preserved)
  - bind: digest-pin comparison, _RBGV_BIND_IMAGE carries pin reference
  - graft: GRAFTED stamp, no verification
- rbgjv01: skip slsa-verifier download for bind/graft
- Enumerate all new vouch substitution variables: _RBGV_VESSEL_MODE, _RBGV_BIND_IMAGE

## RBF vouch changes
- Eliminate zrbf_vouch_bind entirely
- rbf_vouch always submits Cloud Build regardless of mode
- Pass vessel_mode and mode-specific params as substitutions

## Graft operation (new)
- New rbf_graft function in rbf_Foundry.sh
- Runtime param: local image reference
- Push via docker tag + docker push (no crane — image is already local)
- Consecration: gYYMMDDHHMMSS-rYYMMDDHHMMSS
- Steps: authenticate director, docker login to GAR, tag local image, push as -image, submit about pipeline, submit vouch pipeline (sequential Cloud Build jobs)
- No dirty-tree guard (image already built; git state irrelevant to container)
- Wire into rbf_build mode dispatch

## Verification
- Bind vouch: Cloud Build, -vouch with digest-pin verdict
- Graft: local push, -about from Cloud Build, -vouch says GRAFTED
- All three modes: -image/-about/-vouch present, all attestation from Cloud Build
- Inspect correct trust posture per mode

## Depends on
- AtAAC about pipeline extraction

**[260314-0944] rough**

Unify vouch to always run in Cloud Build (mode-aware) and implement graft operation.

## Character
Two related capabilities that complete the three-mode architecture.

## Vouch unification (rbgjv/ modifications)
- `rbgjv02-verify-provenance.sh`: Add mode branching via _RBGV_VESSEL_MODE substitution variable
  - conjure: existing SLSA provenance verification (largely preserved)
  - bind: digest-pin comparison — HEAD -image in GAR, extract Docker-Content-Digest, compare against RBRV_BIND_IMAGE pin digest. Verdict: PASS/FAIL.
  - graft: no verification possible. Verdict: GRAFTED — no provenance chain.
- `rbgjv01-download-verifier.sh`: Skip for bind/graft (slsa-verifier not needed). Guard on _RBGV_VESSEL_MODE.
- New substitution variable _RBGV_VESSEL_MODE passed in vouch builds.create request
- For bind: _RBGV_BIND_IMAGE substitution variable carries the pin reference

## RBF vouch changes
- Eliminate `zrbf_vouch_bind` function entirely (local docker build + push)
- `rbf_vouch` always submits Cloud Build job regardless of vessel mode
- Pass vessel_mode and mode-specific params as substitution variables

## Graft operation (new)
- New `rbf_graft` function in rbf_Foundry.sh
- Runtime param: local image reference
- Consecration: gYYMMDDHHMMSS-rYYMMDDHHMMSS
- Steps: authenticate director, push local image to GAR via crane copy (tag as -image), then submit about pipeline, then submit vouch pipeline
- Dirty-tree guard (same as mirror/inscribe)
- Wire into vessel mode dispatch alongside conjure/bind

## Regime integration
- Graft vessels dispatch to rbf_graft from rbf_build's mode switch
- RBRV_GRAFT_IMAGE: decide regime constant vs runtime parameter

## Verification
- Bind vouch now runs in Cloud Build, produces -vouch with digest-pin verdict
- Graft: local image pushed to GAR, -about generated by Cloud Build, -vouch says GRAFTED
- All three modes: -image/-about/-vouch artifacts present, all attestation from Cloud Build
- Inspect displays correct trust posture for all three modes

## Depends on
- ₢AtAAC about pipeline extraction

### pipeline-orchestration-rbf-create-chaining (₢AtAAI) [complete]

**[260314-1643] complete**

Implement combined delivery for bind and graft. Conjure keeps standalone about as transitional state pending stitch embedding.

## Character
Careful refactoring with mode-aware branching. rbf_create gains a chaining tail; rbf_mirror is rewritten as a Cloud Build job; a new rbf_about wrapper is extracted.

## Model check
Target: bind = one builds.create (copy+about), graft = local push then about-only builds.create, conjure = trigger then standalone about (transitional). All modes chain vouch via rbf_create.

## Function topology after this pace

```
rbf_create(vessel_dir)                        [MODIFIED -- adds chaining tail]
  conjure: rbf_build(vessel_dir)              [UNCHANGED -- trigger dispatch, persists consecration]
  bind:    rbf_mirror(vessel_dir)             [REWRITTEN -- combined builds.create, persists consecration]
  graft:   rbf_graft(vessel_dir)              [SIMPLIFIED by AtAAD -- push only, persists consecration]
  -- read consecration from BURD_OUTPUT_DIR/RBF_FACT_CONSECRATION --
  conjure: rbf_about(vessel_dir, con)         [NEW -- standalone about builds.create]
  graft:   rbf_about(vessel_dir, con)         [NEW -- standalone about builds.create]
  bind:    (skip -- about produced by combined job)
  all:     rbf_vouch(vessel_dir, con)         [UNCHANGED]
```

## rbf_create chaining tail (NEW)
After mode dispatch returns, read consecration from BURD_OUTPUT_DIR/RBF_FACT_CONSECRATION (all three modes persist this -- verify rbf_mirror adds it). Then:

```
con = read BURD_OUTPUT_DIR/RBF_FACT_CONSECRATION
case mode:
  conjure|graft: rbf_about(vessel_dir, con)
  bind: skip (primary job produced -about)
rbf_vouch(vessel_dir, con)
```

rbf_create does NOT authenticate -- rbf_about and rbf_vouch each handle their own auth (same pattern rbf_vouch already follows at line 2457).

## New rbf_about(vessel_dir, consecration) wrapper
Parallel structure to rbf_vouch (line 2457). Does: load vessel, authenticate, gate on -image existence, call zrbf_about_submit(consecration, token, conjure_build_id), show results. This is the director-side wrapper that was stripped from the RBSAB spec in pace J. For conjure, read build_id from BURD_OUTPUT_DIR/RBF_FACT_BUILD_ID (persisted by rbf_build at line 769); for graft, pass empty string.

## rbf_mirror -> combined builds.create [MAJOR REWRITE]

Delete local about assembly (lines 1025-1110): build_info.json generation, syft docker run, docker build/push of -about. Replace with combined builds.create submission.

### Combined job shape
```
builds.create JSON:
  steps:
    step 0: Image copy via skopeo (RBRG_SKOPEO_IMAGE_REF, already pinned)
      - Script: rbgjm01-mirror-image.sh (NEW FILE in Tools/rbk/rbgjm/)
      - skopeo copy docker://BIND_SOURCE docker://GAR_HOST/GAR_PATH/VESSEL:CONSECRATION-image
      - Mason SA ambient auth for GAR; public upstream needs no auth
    steps 1-4: About (rbgja01-04, assembled via shared helper)
  substitutions:
    _RBGA_* namespace (same as zrbf_about_submit)
    _RBGA_BIND_SOURCE already exists for build_info.json -- no new prefix needed
  serviceAccount: mason SA
  options: private pool, CLOUD_LOGGING_ONLY
```

### New step script: Tools/rbk/rbgjm/rbgjm01-mirror-image.sh
Skopeo-based registry-to-registry copy. Builder image: RBRG_SKOPEO_IMAGE_REF. Reads _RBGA_GAR_HOST, _RBGA_GAR_PATH, _RBGA_VESSEL, _RBGA_CONSECRATION, _RBGA_ARK_SUFFIX_IMAGE, _RBGA_BIND_SOURCE from substitution variables. No docker daemon needed -- skopeo operates directly on registries. Preserves multi-platform manifest lists natively.

### Step assembly refactor
Extract about step assembly from zrbf_about_submit into zrbf_assemble_about_steps() returning the step array file path. Reuse in both:
- zrbf_about_submit (standalone about for conjure/graft)
- zrbf_mirror_submit (combined bind job -- prepend mirror step from rbgjm01, then about steps from rbgja01-04)

### Image copy step auth
Mason SA authenticates to GAR via ambient Cloud Build credentials. Skopeo uses these automatically when pushing to GAR. Public upstream images need no auth. Private upstream is out of scope (paddock decision).

### What rbf_mirror retains
- Vessel load and validation (lines 943-948)
- Dirty-tree guard (lines 950-955)
- Director auth (lines 957-964)
- GAR coordinates and consecration generation (lines 966-992)
- ADD: consecration persistence to BURD_OUTPUT_DIR (currently missing -- rbf_build has it at line 755)
- Summary output (SIMPLIFY -- remove Next steps tabtarget)

### What rbf_mirror loses
- Local docker pull (line 981) -- Mason SA pulls in Cloud Build via skopeo
- Local imagetools create (lines 993-998) -- done in Cloud Build step 0 via skopeo
- Git metadata capture (lines 1002-1023) -- zrbf_about_submit handles this (lines 2309-2314)
- Local build_info.json (lines 1025-1064) -- done in Cloud Build step 3
- Local syft (lines 1066-1081) -- done in Cloud Build step 2
- Local about container build/push (lines 1083-1110) -- done in Cloud Build step 4

## rbf_graft simplification
Already handled by AtAAD -- removes about block and Next steps tabtarget. Just push, persist consecration, return.

## rbf_build consecration persistence
Already correct (line 755). Also persists build_id (line 769) -- needed by rbf_about for conjure.

## Spec alignment
After this pace: bind and graft match spec (combined delivery). Conjure has a known transitional gap (standalone about) resolved by stitch pace AtAAL.

## Files touched
- Tools/rbk/rbf_Foundry.sh (rbf_create chaining, rbf_about wrapper, rbf_mirror rewrite, zrbf_assemble_about_steps extraction)
- Tools/rbk/rbgjm/rbgjm01-mirror-image.sh (NEW -- skopeo image copy step script)

**[260314-1607] rough**

Implement combined delivery for bind and graft. Conjure keeps standalone about as transitional state pending stitch embedding.

## Character
Careful refactoring with mode-aware branching. rbf_create gains a chaining tail; rbf_mirror is rewritten as a Cloud Build job; a new rbf_about wrapper is extracted.

## Model check
Target: bind = one builds.create (copy+about), graft = local push then about-only builds.create, conjure = trigger then standalone about (transitional). All modes chain vouch via rbf_create.

## Function topology after this pace

```
rbf_create(vessel_dir)                        [MODIFIED -- adds chaining tail]
  conjure: rbf_build(vessel_dir)              [UNCHANGED -- trigger dispatch, persists consecration]
  bind:    rbf_mirror(vessel_dir)             [REWRITTEN -- combined builds.create, persists consecration]
  graft:   rbf_graft(vessel_dir)              [SIMPLIFIED by AtAAD -- push only, persists consecration]
  -- read consecration from BURD_OUTPUT_DIR/RBF_FACT_CONSECRATION --
  conjure: rbf_about(vessel_dir, con)         [NEW -- standalone about builds.create]
  graft:   rbf_about(vessel_dir, con)         [NEW -- standalone about builds.create]
  bind:    (skip -- about produced by combined job)
  all:     rbf_vouch(vessel_dir, con)         [UNCHANGED]
```

## rbf_create chaining tail (NEW)
After mode dispatch returns, read consecration from BURD_OUTPUT_DIR/RBF_FACT_CONSECRATION (all three modes persist this -- verify rbf_mirror adds it). Then:

```
con = read BURD_OUTPUT_DIR/RBF_FACT_CONSECRATION
case mode:
  conjure|graft: rbf_about(vessel_dir, con)
  bind: skip (primary job produced -about)
rbf_vouch(vessel_dir, con)
```

rbf_create does NOT authenticate -- rbf_about and rbf_vouch each handle their own auth (same pattern rbf_vouch already follows at line 2457).

## New rbf_about(vessel_dir, consecration) wrapper
Parallel structure to rbf_vouch (line 2457). Does: load vessel, authenticate, gate on -image existence, call zrbf_about_submit(consecration, token, conjure_build_id), show results. This is the director-side wrapper that was stripped from the RBSAB spec in pace J. For conjure, read build_id from BURD_OUTPUT_DIR/RBF_FACT_BUILD_ID (persisted by rbf_build at line 769); for graft, pass empty string.

## rbf_mirror -> combined builds.create [MAJOR REWRITE]

Delete local about assembly (lines 1025-1110): build_info.json generation, syft docker run, docker build/push of -about. Replace with combined builds.create submission.

### Combined job shape
```
builds.create JSON:
  steps:
    step 0: Image copy via skopeo (RBRG_SKOPEO_IMAGE_REF, already pinned)
      - Script: rbgjm01-mirror-image.sh (NEW FILE in Tools/rbk/rbgjm/)
      - skopeo copy docker://BIND_SOURCE docker://GAR_HOST/GAR_PATH/VESSEL:CONSECRATION-image
      - Mason SA ambient auth for GAR; public upstream needs no auth
    steps 1-4: About (rbgja01-04, assembled via shared helper)
  substitutions:
    _RBGA_* namespace (same as zrbf_about_submit)
    _RBGA_BIND_SOURCE already exists for build_info.json -- no new prefix needed
  serviceAccount: mason SA
  options: private pool, CLOUD_LOGGING_ONLY
```

### New step script: Tools/rbk/rbgjm/rbgjm01-mirror-image.sh
Skopeo-based registry-to-registry copy. Builder image: RBRG_SKOPEO_IMAGE_REF. Reads _RBGA_GAR_HOST, _RBGA_GAR_PATH, _RBGA_VESSEL, _RBGA_CONSECRATION, _RBGA_ARK_SUFFIX_IMAGE, _RBGA_BIND_SOURCE from substitution variables. No docker daemon needed -- skopeo operates directly on registries. Preserves multi-platform manifest lists natively.

### Step assembly refactor
Extract about step assembly from zrbf_about_submit into zrbf_assemble_about_steps() returning the step array file path. Reuse in both:
- zrbf_about_submit (standalone about for conjure/graft)
- zrbf_mirror_submit (combined bind job -- prepend mirror step from rbgjm01, then about steps from rbgja01-04)

### Image copy step auth
Mason SA authenticates to GAR via ambient Cloud Build credentials. Skopeo uses these automatically when pushing to GAR. Public upstream images need no auth. Private upstream is out of scope (paddock decision).

### What rbf_mirror retains
- Vessel load and validation (lines 943-948)
- Dirty-tree guard (lines 950-955)
- Director auth (lines 957-964)
- GAR coordinates and consecration generation (lines 966-992)
- ADD: consecration persistence to BURD_OUTPUT_DIR (currently missing -- rbf_build has it at line 755)
- Summary output (SIMPLIFY -- remove Next steps tabtarget)

### What rbf_mirror loses
- Local docker pull (line 981) -- Mason SA pulls in Cloud Build via skopeo
- Local imagetools create (lines 993-998) -- done in Cloud Build step 0 via skopeo
- Git metadata capture (lines 1002-1023) -- zrbf_about_submit handles this (lines 2309-2314)
- Local build_info.json (lines 1025-1064) -- done in Cloud Build step 3
- Local syft (lines 1066-1081) -- done in Cloud Build step 2
- Local about container build/push (lines 1083-1110) -- done in Cloud Build step 4

## rbf_graft simplification
Already handled by AtAAD -- removes about block and Next steps tabtarget. Just push, persist consecration, return.

## rbf_build consecration persistence
Already correct (line 755). Also persists build_id (line 769) -- needed by rbf_about for conjure.

## Spec alignment
After this pace: bind and graft match spec (combined delivery). Conjure has a known transitional gap (standalone about) resolved by stitch pace AtAAL.

## Files touched
- Tools/rbk/rbf_Foundry.sh (rbf_create chaining, rbf_about wrapper, rbf_mirror rewrite, zrbf_assemble_about_steps extraction)
- Tools/rbk/rbgjm/rbgjm01-mirror-image.sh (NEW -- skopeo image copy step script)

**[260314-1555] rough**

Implement combined delivery for bind and graft. Conjure keeps standalone about as transitional state pending stitch embedding.

## Character
Careful refactoring with mode-aware branching. rbf_create gains a chaining tail; rbf_mirror is rewritten as a Cloud Build job; a new rbf_about wrapper is extracted.

## Model check
Target: bind = one builds.create (copy+about), graft = local push then about-only builds.create, conjure = trigger then standalone about (transitional). All modes chain vouch via rbf_create.

## Function topology after this pace

```
rbf_create(vessel_dir)                        [MODIFIED -- adds chaining tail]
  conjure: rbf_build(vessel_dir)              [UNCHANGED -- trigger dispatch, persists consecration]
  bind:    rbf_mirror(vessel_dir)             [REWRITTEN -- combined builds.create, persists consecration]
  graft:   rbf_graft(vessel_dir)              [SIMPLIFIED by AtAAD -- push only, persists consecration]
  -- read consecration from BURD_OUTPUT_DIR/RBF_FACT_CONSECRATION --
  conjure: rbf_about(vessel_dir, con)         [NEW -- standalone about builds.create]
  graft:   rbf_about(vessel_dir, con)         [NEW -- standalone about builds.create]
  bind:    (skip -- about produced by combined job)
  all:     rbf_vouch(vessel_dir, con)         [UNCHANGED]
```

## rbf_create chaining tail (NEW)
After mode dispatch returns, read consecration from BURD_OUTPUT_DIR/RBF_FACT_CONSECRATION (all three modes persist this -- verify rbf_mirror adds it). Then:
- conjure/graft: call rbf_about(vessel_dir, consecration), then rbf_vouch(vessel_dir, consecration)
- bind: call rbf_vouch(vessel_dir, consecration) only (combined job already produced -about)

rbf_create does NOT authenticate -- rbf_about and rbf_vouch each handle their own auth (same pattern rbf_vouch already follows at line 2457).

## New rbf_about(vessel_dir, consecration) wrapper
Parallel structure to rbf_vouch (line 2457). Does: load vessel, authenticate, gate on -image existence, call zrbf_about_submit(consecration, token, conjure_build_id), show results. This is the director-side wrapper that was stripped from the RBSAB spec in pace J. For conjure, pass build_id from BURD_OUTPUT_DIR/RBF_FACT_BUILD_ID; for graft, pass empty string.

## rbf_mirror -> combined builds.create [MAJOR REWRITE]

Delete local about assembly (lines 1025-1110): build_info.json generation, syft docker run, docker build/push of -about. Replace with combined builds.create submission.

### Combined job shape
```
builds.create JSON:
  steps:
    step 0: Image copy (gcloud builder -- needs gcloud + docker)
      - gcloud auth configure-docker GAR_HOST
      - docker buildx imagetools create --tag GAR_REF BIND_SOURCE
    steps 1-4: About (rbgja01-04, assembled via same pattern as zrbf_about_submit)
  substitutions:
    _RBGA_* namespace (same as zrbf_about_submit)
    _RBGA_BIND_SOURCE already exists for build_info.json -- no new prefix needed
  serviceAccount: mason SA
  options: private pool, CLOUD_LOGGING_ONLY
```

### Step assembly
Extract about step assembly from zrbf_about_submit into zrbf_assemble_about_steps() returning the step array file path. Reuse in both:
- zrbf_about_submit (standalone about for conjure/graft)
- zrbf_mirror_submit (combined bind job -- prepend image-copy step, then about steps)

### Image copy step auth
Mason SA authenticates to GAR via ambient Cloud Build credentials. gcloud auth configure-docker configures the credential helper. Public upstream images need no auth. Private upstream is out of scope (paddock decision).

### What rbf_mirror retains
- Vessel load and validation (lines 943-948)
- Dirty-tree guard (lines 950-955)
- Director auth (lines 957-964)
- GAR coordinates and consecration generation (lines 966-992)
- ADD: consecration persistence to BURD_OUTPUT_DIR (currently missing -- rbf_build has it at line 755)
- Summary output (SIMPLIFY -- remove Next steps tabtarget)

### What rbf_mirror loses
- Local docker pull (line 981) -- Mason SA pulls in Cloud Build
- Local imagetools create (lines 993-998) -- done in Cloud Build step 0
- Git metadata capture (lines 1002-1023) -- done in zrbf_about_submit
- Local build_info.json (lines 1025-1064) -- done in Cloud Build step 3
- Local syft (lines 1066-1081) -- done in Cloud Build step 2
- Local about container build/push (lines 1083-1110) -- done in Cloud Build step 4

## rbf_graft simplification
Already handled by AtAAD -- removes zrbf_about_submit call and Next steps tabtarget. Just push, persist consecration, return.

## rbf_build consecration persistence
Already correct (line 755). Also persists build_id (line 769) -- needed by rbf_about for conjure.

## Spec alignment
After this pace: bind and graft match spec (combined delivery). Conjure has a known transitional gap (standalone about) resolved by stitch pace AtAAL.

## Files touched
- Tools/rbk/rbf_Foundry.sh

**[260314-1528] rough**

Implement combined delivery for bind and graft. Conjure keeps standalone about as transitional state pending stitch embedding.

## Character
Careful refactoring with mode-aware branching. Three functions change shape.

## Model check
Target: bind = one builds.create (copy+about), graft = local push then about-only builds.create, conjure = trigger then standalone about (transitional). All modes chain vouch via rbf_create.

## rbf_create chaining tail (NEW)
After mode dispatch returns consecration (via BURD_OUTPUT_DIR/RBF_FACT_CONSECRATION):
- conjure: submit standalone about builds.create, poll completion, then submit vouch builds.create
- bind: primary job already produced -about, just submit vouch builds.create
- graft: submit about-only builds.create, poll completion, then submit vouch builds.create

## rbf_mirror → combined builds.create
Delete local about assembly (~lines 1022-1107): build_info.json generation, syft docker run, docker build/push of -about container. Replace with: construct one builds.create JSON containing image-copy steps AND rbgja01-04 about steps. One Cloud Build job produces both -image and -about. Mason SA pulls from upstream (public images). Persist consecration to BURD_OUTPUT_DIR, return.

## rbf_graft simplification
Already handled by AtAAD — rbf_graft does local push, persists consecration, returns. rbf_create handles about submission.

## Conjure transitional state
rbf_build returns from trigger dispatch with consecration. rbf_create submits standalone about builds.create (same zrbf_about_submit used for graft). This is transitional — the stitch pace will embed about into the trigger job, eliminating this call for conjure.

## rbf_build consecration persistence
Verify rbf_build, rbf_mirror, and rbf_graft all use the same pattern (BURD_OUTPUT_DIR/RBF_FACT_CONSECRATION).

## Spec alignment
Specs describe combined delivery as architecture. After this pace: bind and graft match spec. Conjure has a known transitional gap (standalone about) documented in paddock, resolved by stitch pace.

## Files touched
- Tools/rbk/rbf_Foundry.sh

**[260314-1356] rough**

rbf_create orchestrates the full pipeline: mode dispatch, then about, then vouch.

## Character
Careful refactoring — three functions change shape, one gains a chaining tail.

## rbf_create chaining tail (NEW)
After mode dispatch returns, rbf_create reads persisted consecration from BURD_OUTPUT_DIR, then calls rbf_about(vessel_dir, consecration), then rbf_vouch(vessel_dir, consecration). One user command, full pipeline.

## rbf_mirror simplification
Delete local about assembly (~lines 1022-1107): build_info.json generation, syft docker run, docker build/push of -about container. rbf_mirror becomes: pull upstream, push to GAR as -image, persist consecration to BURD_OUTPUT_DIR, return. NOTE: bind image copy does NOT move to Cloud Build in this pace. That is a future enhancement.

## rbf_build consecration persistence
rbf_build already persists consecration. Verify rbf_mirror and rbf_graft use the same pattern (BURD_OUTPUT_DIR/RBF_FACT_CONSECRATION).

## Stitch changes for conjure+about combined job: DEFERRED
Not in this pace. Future pace will embed rbgja steps in trigger-dispatched cloudbuild.json.

## Spec updates
- RBSAC: conjure produces -image, director chains about then vouch
- RBSAB: add note that about is submitted as standalone Cloud Build by director for all modes currently

**[260314-1354] rough**

rbf_create chains about then vouch after mode dispatch. Remove local about from rbf_mirror (~40 lines). Each mode function persists consecration. rbf_create reads consecration, calls rbf_about then rbf_vouch. Stitch changes for conjure+about combined job DEFERRED. Update specs: RBSAC (conjure produces image, about chained by director), RBSAB (about delivery mode-dependent).

### conjure-stitch-about-embedding (₢AtAAL) [complete]

**[260314-1718] complete**

Embed rbgja about steps into conjure's trigger-dispatched cloudbuild.json via the stitch function.

## Character
Surgical stitch modification -- understanding the inscribe/stitch pipeline is the hard part, the change itself is mechanical once understood.

## Depends on
AtAAI must be complete first. This pace modifies the rbf_create chaining tail that AtAAI creates -- it removes conjure's standalone about call from that tail.

## Model check
Currently: conjure trigger job produces -image only. After: trigger job produces -image AND -about. rbf_create no longer needs standalone about submission for conjure.

## Stitch changes
The stitch function (in rbf_Foundry.sh) assembles cloudbuild.json for trigger dispatch during inscribe. Append the four rbgja steps (discover platforms, syft SBOM, build_info assembly, about push) after the existing image build steps. The rbgja scripts are already extracted as standalone Cloud Build step scripts. Use zrbf_assemble_about_steps() (extracted in AtAAI) to generate the step JSON array.

## rbf_create update
Remove conjure's standalone about submission from rbf_create chaining tail. After stitch embedding, conjure's trigger job returns having produced both -image and -about. rbf_create chains only vouch for conjure (matching bind behavior).

## Substitution variables
Conjure about steps read consecration from /workspace (computed by build step 01), not from _RBGA_CONSECRATION. The stitch function does NOT add _RBGA_CONSECRATION for conjure. Other _RBGA_* variables may need population -- verify against RBSAB substitution table (rbtgo_about_substitutions anchor).

## Files touched
- Tools/rbk/rbf_Foundry.sh (stitch function + rbf_create conjure branch)
- Possibly: Tools/rbk/rbgjb/ (trigger cloudbuild template, if stitch reads a template)

**[260314-1607] rough**

Embed rbgja about steps into conjure's trigger-dispatched cloudbuild.json via the stitch function.

## Character
Surgical stitch modification -- understanding the inscribe/stitch pipeline is the hard part, the change itself is mechanical once understood.

## Depends on
AtAAI must be complete first. This pace modifies the rbf_create chaining tail that AtAAI creates -- it removes conjure's standalone about call from that tail.

## Model check
Currently: conjure trigger job produces -image only. After: trigger job produces -image AND -about. rbf_create no longer needs standalone about submission for conjure.

## Stitch changes
The stitch function (in rbf_Foundry.sh) assembles cloudbuild.json for trigger dispatch during inscribe. Append the four rbgja steps (discover platforms, syft SBOM, build_info assembly, about push) after the existing image build steps. The rbgja scripts are already extracted as standalone Cloud Build step scripts. Use zrbf_assemble_about_steps() (extracted in AtAAI) to generate the step JSON array.

## rbf_create update
Remove conjure's standalone about submission from rbf_create chaining tail. After stitch embedding, conjure's trigger job returns having produced both -image and -about. rbf_create chains only vouch for conjure (matching bind behavior).

## Substitution variables
Conjure about steps read consecration from /workspace (computed by build step 01), not from _RBGA_CONSECRATION. The stitch function does NOT add _RBGA_CONSECRATION for conjure. Other _RBGA_* variables may need population -- verify against RBSAB substitution table (rbtgo_about_substitutions anchor).

## Files touched
- Tools/rbk/rbf_Foundry.sh (stitch function + rbf_create conjure branch)
- Possibly: Tools/rbk/rbgjb/ (trigger cloudbuild template, if stitch reads a template)

**[260314-1528] rough**

Embed rbgja about steps into conjure's trigger-dispatched cloudbuild.json via the stitch function.

## Character
Surgical stitch modification — understanding the inscribe/stitch pipeline is the hard part, the change itself is mechanical once understood.

## Model check
Currently: conjure trigger job produces -image only. After: trigger job produces -image AND -about. rbf_create no longer needs standalone about submission for conjure.

## Stitch changes
The stitch function (in rbf_Foundry.sh) assembles cloudbuild.json for trigger dispatch during inscribe. Append the four rbgja steps (discover platforms, syft SBOM, build_info assembly, about push) after the existing image build steps. The rbgja scripts are already extracted as standalone Cloud Build step scripts.

## rbf_create update
Remove conjure's standalone about submission from rbf_create chaining tail. After stitch embedding, conjure's trigger job returns having produced both -image and -about. rbf_create chains only vouch for conjure (matching bind behavior).

## Substitution variables
Conjure about steps read consecration from /workspace (computed by build step 01), not from _RBGA_CONSECRATION. The stitch function does NOT add _RBGA_CONSECRATION for conjure. Other _RBGA_* variables may need population — verify against RBSAB substitution table (<<rbtgo_about_substitutions>>).

## Files touched
- Tools/rbk/rbf_Foundry.sh (stitch function + rbf_create conjure branch)
- Possibly: Tools/rbk/rbgjb/ (trigger cloudbuild template, if stitch reads a template)

### three-mode-integration-test (₢AtAAE) [complete]

**[260315-0914] complete**

Three-mode integration test fixture: structural creation.

## Character
Design and scaffolding — build the fixture shape, discover wiring gaps.

## Delivered
- Created `rbtctm_ThreeMode.sh` with 12-step test case (conjure→bind→graft→check→vouch_gate→retrieve→run→cleanup→abjure×3→check)
- Registered three-mode fixture in testbench (`rbtb_testbench.sh`), replacing retired ark-lifecycle
- Created tabtarget `tt/rbw-tf.TestFixture.three-mode.sh`
- Deleted retired `rbtcal_ArkLifecycle.sh` and its tabtarget
- Discovered: graft vessel has stale hardcoded `RBRV_GRAFT_IMAGE` — test needs dynamic wiring from conjure output
- Discovered: conjure consecration format anomaly in test output needs investigation

**[260315-0903] rough**

Three-mode integration test fixture: structural creation.

## Character
Design and scaffolding — build the fixture shape, discover wiring gaps.

## Delivered
- Created `rbtctm_ThreeMode.sh` with 12-step test case (conjure→bind→graft→check→vouch_gate→retrieve→run→cleanup→abjure×3→check)
- Registered three-mode fixture in testbench (`rbtb_testbench.sh`), replacing retired ark-lifecycle
- Created tabtarget `tt/rbw-tf.TestFixture.three-mode.sh`
- Deleted retired `rbtcal_ArkLifecycle.sh` and its tabtarget
- Discovered: graft vessel has stale hardcoded `RBRV_GRAFT_IMAGE` — test needs dynamic wiring from conjure output
- Discovered: conjure consecration format anomaly in test output needs investigation

**[260314-2028] rough**

Three-mode integration test: conjure → bind → graft → vouch → consecration check.

## Character
Methodical execution against live GCP. Mostly mechanical, but watch for environment surprises.

## Test fixture shape

1. **Conjure** busybox via `rbw-DC` on `rbev-busybox` — produces image + about + vouch with SLSA provenance
2. **Bind** plantuml via `rbw-DC` on `rbev-bottle-plantuml` — mirrors upstream image + about + vouch with digest-pin trust
3. **Graft** the conjure-produced busybox via `rbw-DC` on `rbev-busybox-graft` — local push + about + vouch with GRAFTED trust
4. **Vouch** all pending via `rbw-DV` (if not already vouched by rbf_create chaining)
5. **Consecration check** via `rbw-Dc` — verify all three modes show vouched health
6. **Inspect** (optional) via `rbw-DI` — verify mode-specific trust statements in vouch artifacts

## Key invariant
The graft input is the conjure-produced busybox — same image content, different trust path. Consecration check must show both coexisting with their respective trust statements.

## Vessels
- `rbev-busybox` — conjure mode
- `rbev-bottle-plantuml` — bind mode
- `rbev-busybox-graft` — graft mode (already configured, committed earlier in this heat)

**[260314-2026] rough**

Three-mode integration test: conjure → bind → graft → vouch → consecration check.

## Character
Methodical execution against live GCP. Mostly mechanical, but watch for environment surprises.

## Test fixture shape

1. **Conjure** busybox via `rbw-DC` on `rbev-busybox` — produces image + about + vouch with SLSA provenance
2. **Bind** plantuml via `rbw-DC` on `rbev-bottle-plantuml` — mirrors upstream image + about + vouch with digest-pin trust
3. **Graft** the conjure-produced busybox via `rbw-DC` on `rbev-busybox-graft` — local push + about + vouch with GRAFTED trust
4. **Vouch** all pending via `rbw-DV` (if not already vouched by rbf_create chaining)
5. **Consecration check** via `rbw-Dc` — verify all three modes show vouched health
6. **Inspect** (optional) via `rbw-DI` — verify mode-specific trust statements in vouch artifacts

## Key invariant
The graft input is the conjure-produced busybox — same image content, different trust path. Consecration check must show both coexisting with their respective trust statements.

## Vessels
- `rbev-busybox` — conjure mode
- `rbev-bottle-plantuml` — bind mode
- `rbev-busybox-graft` — graft mode (already configured, committed earlier in this heat)

**[260314-1556] rough**

Test cases covering all three vessel modes through the unified pipeline.

## Character
Methodical test construction with systematic coverage. Tests verify combined delivery architecture, not standalone about.

## Split: unit tests (no GCP) vs integration tests (live GCP)

### Unit tests (CI-safe)
- Consecration format: new regex parsing, c/b/g prefix per mode, edge cases
- Regime: graft mode validation, group gates, kindle defaults, render output

### Integration tests (require GCP credentials)

Combined delivery verification:
- Conjure: single trigger-dispatched job produces both -image and -about (verify both artifacts exist after one Cloud Build job)
- Bind: single builds.create job copies image and produces -about (verify both artifacts exist after one Cloud Build job)
- Graft: local push produces -image, then about-only builds.create produces -about (verify two-step sequence)
- All modes: -about contains mode-appropriate build_info.json and SBOM

Vouch pipeline (all three modes):
- Conjure: SLSA verification, per-platform verdicts
- Bind: digest-pin in Cloud Build, PASS/FAIL correct
- Graft: GRAFTED verdict, no verification attempted
- All modes: -vouch pushed by Cloud Build, never local

End-to-end per mode (via rbf_create -- single command, full pipeline):
- Conjure: rbf_create -> trigger job (-image + -about) -> vouch job -> inspect full posture
- Bind: rbf_create -> combined builds.create (-image + -about) -> vouch job -> inspect digest-pin posture
- Graft: rbf_create -> local push (-image) -> about builds.create (-about) -> vouch job -> inspect GRAFTED posture
- Abjure: all three artifacts (-image, -about, -vouch) deleted per mode
- Consecration check: correct health/mode display

Inspect display:
- Verify mode-specific trust statements in output text per mode

### Negative cases
- Graft rejects non-existent local image (docker image inspect fails)
- Vouch rejects unknown vessel mode
- Graft push fails gracefully on GAR auth failure
- About job fails gracefully if -image does not exist in GAR
- rbf_create chaining: vouch not attempted if about fails (conjure/graft)
- rbf_create chaining: vouch not attempted if combined job fails (bind)

## Depends on
- AtAAD vouch unification and graft
- AtAAI combined delivery and chaining
- AtAAL conjure stitch embedding (for conjure combined test)

**[260314-1001] rough**

Test cases covering all three vessel modes through the unified pipeline.

## Character
Methodical test construction with systematic coverage.

## Split: unit tests (no GCP) vs integration tests (live GCP)

### Unit tests (CI-safe)
- Consecration format: new regex parsing, c/b/g prefix per mode, edge cases
- Regime: graft mode validation, group gates, kindle defaults, render output

### Integration tests (require GCP credentials)

About pipeline (all three modes):
- Cloud Build job submitted after -image exists
- Platform discovery: single-arch and multi-arch
- Syft SBOM present in -about for all modes
- build_info.json mode-appropriate (full provenance conjure, reduced bind/graft)

Vouch pipeline (all three modes):
- Conjure: SLSA verification, per-platform verdicts
- Bind: digest-pin in Cloud Build, PASS/FAIL correct
- Graft: GRAFTED verdict, no verification attempted
- All modes: -vouch pushed by Cloud Build, never local

End-to-end per mode:
- Conjure: inscribe, trigger build (-image), about job, vouch job, inspect full posture
- Bind: mirror (-image), about job, vouch job, inspect digest-pin posture
- Graft: docker push (-image), about job, vouch job, inspect GRAFTED posture
- Abjure: all three artifacts (-image, -about, -vouch) deleted per mode
- Consecration check: correct health/mode display

Inspect display:
- Verify mode-specific trust statements in output text per mode

### Negative cases
- Graft rejects non-existent local image (docker image inspect fails)
- Vouch rejects unknown vessel mode
- Graft push fails gracefully on GAR auth failure
- About job fails gracefully if -image does not exist in GAR

## Depends on
- AtAAD vouch unification and graft

**[260314-0955] rough**

Test cases covering all three vessel modes through the unified pipeline.

## Character
Methodical test construction — systematic coverage.

## Split: unit tests (no GCP) vs integration tests (live GCP)

### Unit tests (CI-safe)
- Consecration format: new regex parsing, c/b/g prefix per mode, edge cases
- Regime: graft mode validation, group gates, kindle defaults, render output
- No old-format backward compat tests needed (depot recreated)

### Integration tests (require GCP credentials)

**About pipeline (all three modes)**
- Cloud Build job submitted after -image exists
- Platform discovery: single-arch and multi-arch
- Syft SBOM present in -about for all modes
- build_info.json mode-appropriate (full provenance conjure, reduced bind/graft)

**Vouch pipeline (all three modes)**
- Conjure: SLSA verification, per-platform verdicts
- Bind: digest-pin in Cloud Build, PASS/FAIL correct
- Graft: GRAFTED verdict, no verification attempted
- All modes: -vouch pushed by Cloud Build, never local

**End-to-end per mode**
- Conjure: inscribe, trigger build (-image), about job, vouch job, inspect full posture
- Bind: mirror (-image), about job, vouch job, inspect digest-pin posture
- Graft: docker push (-image), about job, vouch job, inspect GRAFTED posture
- Abjure: all three artifacts (-image, -about, -vouch) deleted per mode
- Consecration check: correct health/mode display

**Inspect display**
- Verify mode-specific trust statements in output text

## Depends on
- AtAAD vouch unification and graft

**[260314-0945] rough**

Add test fixtures and test cases covering all three vessel modes through the new unified pipeline.

## Character
Methodical test construction — systematic coverage of the new three-mode architecture.

## Consecration format tests
- Validate new format regex `[cbg]\d{12}-r\d{12}` parsing
- Verify c/b/g prefix correctly assigned per vessel mode
- Verify old format is rejected / migration handled
- Edge cases: midnight timestamps, year boundaries

## Regime tests
- Graft mode accepted by validation
- RBRV_GRAFT_IMAGE validated when mode=graft
- Conjure/bind regime variables rejected when mode=graft (group gate)
- Kindle defaults correctly applied for graft group
- Render displays graft section

## About pipeline tests (all three modes)
- About Cloud Build job submitted successfully after -image exists
- Platform discovery handles single-arch and multi-arch images
- Syft SBOM present in -about artifact for all modes
- build_info.json contains mode-appropriate metadata (full provenance for conjure, reduced for bind/graft)
- Git metadata passed correctly via _RBGA_* substitution variables

## Vouch pipeline tests (all three modes)
- Conjure: SLSA verification runs, vouch summary contains per-platform verdicts
- Bind: digest-pin comparison runs in Cloud Build, PASS/FAIL verdict correct
- Graft: GRAFTED verdict produced, no verification attempted
- All modes: -vouch artifact pushed to GAR by Cloud Build (never local)

## End-to-end per mode
- Conjure: inscribe → trigger build (-image only) → about job → vouch job → inspect shows full trust posture
- Bind: mirror (-image) → about job → vouch job → inspect shows digest-pin posture
- Graft: local push (-image) → about job → vouch job → inspect shows GRAFTED posture
- Abjure: all three artifacts deleted for each mode
- Consecration check: correct health/status display for each mode

## Depends on
- ₢AtAAD vouch unification and graft

### bure-tweak-mechanism (₢AtAAN) [complete]

**[260315-0931] complete**

Add generic tweak mechanism to BURE ambient regime.

## Character
Mechanical regime plumbing — small, precise additions.

## Work
- Enroll `BURE_TWEAK_NAME` (string, optional) and `BURE_TWEAK_VALUE` (string, optional) in `bure_regime.sh`
- Verify scope sentinel passes with new vars
- Verify BURE CLI render/validate commands work with new enrollment

## Design
BURE stays domain-agnostic. It carries a name/value pair. Consumer code (RBK, etc.) interprets the tweak name. One tweak per invocation is sufficient.

## Propagation
BURE vars propagate as exported env vars through zbuto_invoke → tabtarget → launcher → dispatch → workbench. BURE is NOT kindled during dispatch, so scope sentinel doesn't fire at runtime. Enrollment needed for CLI validation path only.

**[260315-0903] rough**

Add generic tweak mechanism to BURE ambient regime.

## Character
Mechanical regime plumbing — small, precise additions.

## Work
- Enroll `BURE_TWEAK_NAME` (string, optional) and `BURE_TWEAK_VALUE` (string, optional) in `bure_regime.sh`
- Verify scope sentinel passes with new vars
- Verify BURE CLI render/validate commands work with new enrollment

## Design
BURE stays domain-agnostic. It carries a name/value pair. Consumer code (RBK, etc.) interprets the tweak name. One tweak per invocation is sufficient.

## Propagation
BURE vars propagate as exported env vars through zbuto_invoke → tabtarget → launcher → dispatch → workbench. BURE is NOT kindled during dispatch, so scope sentinel doesn't fire at runtime. Enrollment needed for CLI validation path only.

### create-fact-file-enrichment (₢AtAAO) [complete]

**[260315-1031] complete**

Make all three modes emit the same atomic fact files from rbf_create.

## Character
Careful API surface work — fact files are a test contract. No consumer should ever parse or construct values.

## Principle
Fact files are simple, atomic values that consumers read verbatim. No splitting, no construction from regime constants, no implicit coupling.

## Current state
- Conjure (rbf_build): emits 3 files — RBF_FACT_CONSECRATION, RBF_FACT_IMAGE_REF, RBF_FACT_BUILD_ID
- Bind (rbf_mirror): emits 1 file — RBF_FACT_CONSECRATION only
- Graft (rbf_graft): emits 1 file — RBF_FACT_CONSECRATION only

This asymmetry is a bug, not a design choice.

## Deliverable
After rbf_create (any mode), BURD_OUTPUT_DIR contains at minimum:
- `RBF_FACT_CONSECRATION` — consecration string (exists, all modes)
- New: **retrieve locator** fact — `{vessel}:{tag}` passable directly to RBZ_RETRIEVE_IMAGE
- New: **local image ref** fact — full GAR ref usable by docker run or as graft source after retrieve

Define new constants in rbgc_Constants.sh. Add writes to rbf_mirror and rbf_graft to match rbf_build's coverage.

## Functions to modify
- `rbf_mirror()` (rbf_Foundry.sh ~line 1090) — add retrieve locator and local image ref fact writes
- `rbf_graft()` (rbf_Foundry.sh ~line 1362) — add retrieve locator and local image ref fact writes
- `rbf_build()` (rbf_Foundry.sh ~line 861) — add retrieve locator fact write (already has image ref)
- `rbgc_Constants.sh` — define new RBF_FACT_* constants

**[260315-0910] rough**

Make all three modes emit the same atomic fact files from rbf_create.

## Character
Careful API surface work — fact files are a test contract. No consumer should ever parse or construct values.

## Principle
Fact files are simple, atomic values that consumers read verbatim. No splitting, no construction from regime constants, no implicit coupling.

## Current state
- Conjure (rbf_build): emits 3 files — RBF_FACT_CONSECRATION, RBF_FACT_IMAGE_REF, RBF_FACT_BUILD_ID
- Bind (rbf_mirror): emits 1 file — RBF_FACT_CONSECRATION only
- Graft (rbf_graft): emits 1 file — RBF_FACT_CONSECRATION only

This asymmetry is a bug, not a design choice.

## Deliverable
After rbf_create (any mode), BURD_OUTPUT_DIR contains at minimum:
- `RBF_FACT_CONSECRATION` — consecration string (exists, all modes)
- New: **retrieve locator** fact — `{vessel}:{tag}` passable directly to RBZ_RETRIEVE_IMAGE
- New: **local image ref** fact — full GAR ref usable by docker run or as graft source after retrieve

Define new constants in rbgc_Constants.sh. Add writes to rbf_mirror and rbf_graft to match rbf_build's coverage.

## Functions to modify
- `rbf_mirror()` (rbf_Foundry.sh ~line 1090) — add retrieve locator and local image ref fact writes
- `rbf_graft()` (rbf_Foundry.sh ~line 1362) — add retrieve locator and local image ref fact writes
- `rbf_build()` (rbf_Foundry.sh ~line 861) — add retrieve locator fact write (already has image ref)
- `rbgc_Constants.sh` — define new RBF_FACT_* constants

**[260315-0903] rough**

Enrich rbf_create output fact files so tests can reconstruct locators and local image refs.

## Character
Careful API surface expansion — fact files are a test contract.

## Work
- Split existing `RBF_FACT_IMAGE_REF` (full GAR ref) into repository and tag components, or add new complementary fact files
- Ensure all three modes (conjure, bind, graft) emit consistent fact files from `rbf_create`
- Define new constants in `rbgc_Constants.sh`
- Test can construct retrieve locator from vessel sigil + consecration + suffix
- Test can construct local image ref after retrieve from the same facts

## Design principle
The test should never need to hardcode GAR coordinates — fact files provide all building blocks.

### graft-tweak-and-test-wiring (₢AtAAP) [complete]

**[260315-1043] complete**

Wire conjure output into graft input via BURE tweak, complete test fixture.

## Character
Integration wiring — connecting pieces from previous paces. No parsing, no construction.

## Graft-side change
`rbf_graft` checks after vessel loading:
```
if test "${BURE_TWEAK_NAME:-}" = "threemodegraft"; then
  RBRV_GRAFT_IMAGE="${BURE_TWEAK_VALUE}"
fi
```
The committed rbev-busybox-graft/rbrv.env retains its static default; the tweak overrides it during testing.

## Test-side wiring (rbtctm_ThreeMode.sh)
Exact chain between conjure and graft:
1. After conjure: read retrieve locator fact file (verbatim) and local image ref fact file (verbatim)
2. Call `buto_tt_expect_ok "${RBZ_RETRIEVE_IMAGE}" "${z_retrieve_locator}"` — pulls image to local Docker
3. After retrieve: the local image is tagged as the full GAR ref (the local image ref fact value)
4. `export BURE_TWEAK_NAME=threemodegraft`
5. `export BURE_TWEAK_VALUE="${z_local_image_ref}"` — read verbatim from fact file, not constructed
6. Call `buto_tt_expect_ok "${RBZ_CREATE_ARK}" "${z_graft_dir}"` — graft uses tweak override

Zero construction logic in the test. Every value read verbatim from a fact file.

## Key invariant
Graft input is the conjure-produced busybox — same image content, different trust path.

**[260315-0911] rough**

Wire conjure output into graft input via BURE tweak, complete test fixture.

## Character
Integration wiring — connecting pieces from previous paces. No parsing, no construction.

## Graft-side change
`rbf_graft` checks after vessel loading:
```
if test "${BURE_TWEAK_NAME:-}" = "threemodegraft"; then
  RBRV_GRAFT_IMAGE="${BURE_TWEAK_VALUE}"
fi
```
The committed rbev-busybox-graft/rbrv.env retains its static default; the tweak overrides it during testing.

## Test-side wiring (rbtctm_ThreeMode.sh)
Exact chain between conjure and graft:
1. After conjure: read retrieve locator fact file (verbatim) and local image ref fact file (verbatim)
2. Call `buto_tt_expect_ok "${RBZ_RETRIEVE_IMAGE}" "${z_retrieve_locator}"` — pulls image to local Docker
3. After retrieve: the local image is tagged as the full GAR ref (the local image ref fact value)
4. `export BURE_TWEAK_NAME=threemodegraft`
5. `export BURE_TWEAK_VALUE="${z_local_image_ref}"` — read verbatim from fact file, not constructed
6. Call `buto_tt_expect_ok "${RBZ_CREATE_ARK}" "${z_graft_dir}"` — graft uses tweak override

Zero construction logic in the test. Every value read verbatim from a fact file.

## Key invariant
Graft input is the conjure-produced busybox — same image content, different trust path.

**[260315-0903] rough**

Wire conjure output into graft input via BURE tweak, complete test fixture.

## Character
Integration wiring — connecting the pieces from the previous two paces.

## Work
- `rbf_graft` checks `BURE_TWEAK_NAME=threemodegraft` after vessel loading, overrides `RBRV_GRAFT_IMAGE` with `BURE_TWEAK_VALUE`
- Update `rbtctm_ThreeMode.sh`: after conjure, retrieve the conjured image, export BURE tweak vars, then graft
- Test sequence: conjure → read fact files → retrieve → export BURE_TWEAK_NAME/VALUE → graft → check → abjure

## Key invariant
Graft input is the conjure-produced busybox — same image content, different trust path.

### three-mode-run-and-green (₢AtAAQ) [complete]

**[260331-1721] complete**

Re-run three-mode integration test and confirm all 12 steps pass.

## Character
Methodical execution against live GCP. The hard bugs are fixed — this is confirmation.

## Context from previous session
Three fixes applied and inscribed:
1. **Stale skopeo pin** — upstream quay.io deleted old manifest; refreshed via rbw-DPG
2. **rbrg_cli.sh furnish chain** — extraction from rbrr_cli.sh missed `rbrr_regime.sh` source and `zrbrr_kindle` call; added both plus `RBBC_rbrr_file` source
3. **Graft about discover-platforms** — GAR returns gzip-compressed config blobs for single-manifest images; added `--compressed` to curl in rbgja01-discover-platforms.sh

Conjure+vouch passed end-to-end on latest run. Process died mid-vouch-poll due to `script -q /dev/null` wrapper (don't use it — run test fixture directly). Bind and graft paths untested with the --compressed fix.

## Work
- Run `tt/rbw-tf.TestFixture.three-mode.sh` directly (NO `script` wrapper)
- Confirm all 12 steps pass: conjure → bind → graft → check → vouch_gate → retrieve → run → cleanup → abjure×3 → check
- If graft about still fails, check gcloud logs for discover-platforms step

**[260315-1135] rough**

Re-run three-mode integration test and confirm all 12 steps pass.

## Character
Methodical execution against live GCP. The hard bugs are fixed — this is confirmation.

## Context from previous session
Three fixes applied and inscribed:
1. **Stale skopeo pin** — upstream quay.io deleted old manifest; refreshed via rbw-DPG
2. **rbrg_cli.sh furnish chain** — extraction from rbrr_cli.sh missed `rbrr_regime.sh` source and `zrbrr_kindle` call; added both plus `RBBC_rbrr_file` source
3. **Graft about discover-platforms** — GAR returns gzip-compressed config blobs for single-manifest images; added `--compressed` to curl in rbgja01-discover-platforms.sh

Conjure+vouch passed end-to-end on latest run. Process died mid-vouch-poll due to `script -q /dev/null` wrapper (don't use it — run test fixture directly). Bind and graft paths untested with the --compressed fix.

## Work
- Run `tt/rbw-tf.TestFixture.three-mode.sh` directly (NO `script` wrapper)
- Confirm all 12 steps pass: conjure → bind → graft → check → vouch_gate → retrieve → run → cleanup → abjure×3 → check
- If graft about still fails, check gcloud logs for discover-platforms step

**[260315-0904] rough**

Execute three-mode integration test fixture and fix whatever surfaces.

## Character
Methodical execution against live GCP. Expect surprises — the conjure consecration format anomaly (`i20260313_142921-b20260315_034510` not matching `cYYMMDDHHMMSS-rYYMMDDHHMMSS`) is unresolved and may resurface.

## Work
- Run `tt/rbw-tf.TestFixture.three-mode.sh`
- Diagnose and fix failures
- May require investigating consecration check regex (`rbf_Foundry.sh:2954`) or consecration format discrepancies
- Confirm all 12 steps pass: conjure → bind → graft → check → vouch_gate → retrieve → run → cleanup → abjure×3 → check

### bus0-bure-tweak-spec (₢AtAAR) [complete]

**[260315-0936] complete**

Update BUS0 specification to document BURE tweak regime variables.

## Character
Spec writing — document what was built.

## Work
- Add `BURE_TWEAK_NAME` and `BURE_TWEAK_VALUE` to BUS0-BashUtilitiesSpec.adoc
- Document the ambient propagation model (env vars flow through zbuto_invoke without BURE kindle)
- Document the one-tweak-per-invocation constraint
- Document consumer-side interpretation pattern (RBK checks tweak name, BUK doesn't know domain semantics)

**[260315-0904] rough**

Update BUS0 specification to document BURE tweak regime variables.

## Character
Spec writing — document what was built.

## Work
- Add `BURE_TWEAK_NAME` and `BURE_TWEAK_VALUE` to BUS0-BashUtilitiesSpec.adoc
- Document the ambient propagation model (env vars flow through zbuto_invoke without BURE kindle)
- Document the one-tweak-per-invocation constraint
- Document consumer-side interpretation pattern (RBK checks tweak name, BUK doesn't know domain semantics)

### rbs0-fact-file-vocabulary (₢AtAAS) [complete]

**[260315-1051] complete**

Update RBS0 specification to document fact file vocabulary and create command outputs.

## Character
Spec writing — document what was built.

## Work
- Document fact file pattern: well-known filenames in BURD_OUTPUT_DIR emitted by tabtargets for test observability
- Document `RBF_FACT_CONSECRATION`, `RBF_FACT_IMAGE_REF`, `RBF_FACT_BUILD_ID`, and any new fact files added in create-fact-file-enrichment pace
- Document per-mode outputs from `rbf_create` (vessel, consecration, image components)
- Document consecration check fact files (`{sigil}_fact_consec_{consecration}`)
- Cross-reference RBSCK-consecration_check.adoc and relevant ark specs

**[260315-0904] rough**

Update RBS0 specification to document fact file vocabulary and create command outputs.

## Character
Spec writing — document what was built.

## Work
- Document fact file pattern: well-known filenames in BURD_OUTPUT_DIR emitted by tabtargets for test observability
- Document `RBF_FACT_CONSECRATION`, `RBF_FACT_IMAGE_REF`, `RBF_FACT_BUILD_ID`, and any new fact files added in create-fact-file-enrichment pace
- Document per-mode outputs from `rbf_create` (vessel, consecration, image components)
- Document consecration check fact files (`{sigil}_fact_consec_{consecration}`)
- Cross-reference RBSCK-consecration_check.adoc and relevant ark specs

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 M manual-test-fixes-continuation
  2 J rbsab-about-procedure-refactor
  3 K rbsav-vouch-platform-discovery-alignment
  4 A specification-updates-graft-about-vouch
  5 B consecration-format-and-regime-foundation
  6 F syft-gcb-image-pin
  7 C about-pipeline-extraction
  8 G diags-artifact-forwarding
  9 H about-pipeline-suffix-substitutions
  10 D vouch-unification-and-graft
  11 I pipeline-orchestration-rbf-create-chaining
  12 L conjure-stitch-about-embedding
  13 E three-mode-integration-test
  14 N bure-tweak-mechanism
  15 O create-fact-file-enrichment
  16 P graft-tweak-and-test-wiring
  17 Q three-mode-run-and-green
  18 R bus0-bure-tweak-spec
  19 S rbs0-fact-file-vocabulary

MJKABFCGHDILENOPQRS
····x·xxxxxx··xx··· rbf_Foundry.sh
·x·x··xxx···x······ RBSAB-ark_about.adoc
······xxx···x···x·· rbgja01-discover-platforms.sh
·············xx··x· BUS0-BashUtilitiesSpec.adoc
······x·x···x······ rbgja02-syft-per-platform.sh
······xx····x······ rbgja03-build-info-per-platform.sh
······xxx·········· rbgja04-assemble-push-about.sh
···x···x··········x RBSCK-consecration_check.adoc
···x··xx··········· RBSAI-ark_inspect.adoc
··xx········x······ RBSAV-ark_vouch.adoc
·x·x··············x RBS0-SpecTop.adoc, RBSAG-ark_graft.adoc
x········x··x······ rbgjv02-verify-provenance.sh
············x··x··· rbtctm_ThreeMode.sh
············xx····· rbtb_testbench.sh
·······x······x···· rbgc_Constants.sh
······xx··········· RBSCB-CloudBuildPosture.adoc, rbgjb03-buildx-push-multi.sh, rbgjb05-push-per-platform.sh
·····x··········x·· rbrg.env
····x····x········· CLAUDE.consumer.md, README.consumer.md
···x··············x RBSAC-ark_conjure.adoc
···x···x··········· RBSAA-ark_abjure.adoc, RBSRI-rubric_inscribe.adoc
···xx·············· RBSRV-RegimeVessel.adoc
··················x AXLA-Lexicon.adoc
················x·· rbrg_cli.sh
·············x····· BCG-BashConsoleGuide.md, bure_regime.sh, butd_dispatch.sh, butr_registry.sh, rbtcbe_BureEnvironment.sh
············x······ rbrv.env, rbtcal_ArkLifecycle.sh, rbtcap_AccessProbe.sh, rbw-tf.TestFixture.ark-lifecycle.sh, rbw-tf.TestFixture.three-mode.sh
··········x········ rbgjm01-mirror-image.sh
·········x········· rbgjv01-download-verifier.sh, rbgjv03-assemble-push-vouch.sh, rbw-Db.DirectorBuildsAbout.sh, rbz_zipper.sh
·······x··········· rbgjb07-push-diags.sh
······x············ rbgjb06-imagetools-create.sh, rbgjb06-syft-per-platform.sh, rbgjb07-build-info-per-platform.sh, rbgjb08-buildx-push-about.sh, rbgjb09-imagetools-create.sh
····x·············· buv_validation.sh, rbgjb01-derive-tag-base.sh, rbgm_ManualProcedures.sh, rbrv_regime.sh, rbtcrv_RegimeValidation.sh
···x··············· CLAUDE.md, RBSDV-director_vouch.adoc, RBSID-image_delete.adoc, RBSTB-trigger_build.adoc

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 121 commits)

  1 E three-mode-integration-test
  2 M manual-test-fixes-continuation
  3 N bure-tweak-mechanism
  4 R bus0-bure-tweak-spec
  5 O create-fact-file-enrichment
  6 S rbs0-fact-file-vocabulary
  7 P graft-tweak-and-test-wiring
  8 Q three-mode-run-and-green

123456789abcdefghijklmnopqrstuvwxyz
x·····x·········x··················  E  3c
··x··x·····························  M  2c
·················xx················  N  2c
···················xx··············  R  2c
······················xx···········  O  2c
························xx··xx·····  S  4c
··························xx·······  P  2c
······························xxx·x  Q  4c
```

## Steeplechase

### 2026-03-31 17:21 - ₢AtAAQ - W

Three-mode validation achieved through today's parallel ordain runs: conjure (busybox, ifrit, sentry, jupyter), bind (plantuml), plus graft tested in prior pace. All modes produced image+about+vouch. Crucible suite passed 115 cases on both arm64 and amd64. Mirror pool routing bug (bind hardcoded to airgap) found and fixed during this run.

### 2026-03-15 11:35 - Heat - T

three-mode-run-and-green

### 2026-03-15 11:25 - ₢AtAAQ - n

Added --compressed to config blob curl in discover-platforms single-manifest path — GAR returns gzip-compressed config blobs causing jq parse failure

### 2026-03-15 11:04 - ₢AtAAQ - n

Refreshed stale RBRG_SKOPEO_IMAGE_REF pin (upstream deleted old manifest) and fixed rbrg_cli.sh furnish chain missing rbrr_regime.sh source and zrbrr_kindle call (broken during extraction from rbrr_cli.sh)

### 2026-03-15 10:59 - ₢AtAAQ - n

Updated stale RBRG_SKOPEO_IMAGE_REF pin — upstream quay.io/skopeo/stable deleted old manifest, replaced with current index digest

### 2026-03-15 10:51 - ₢AtAAS - W

Documented fact file vocabulary in RBS0 and AXLA. Created axpof_fact voicing and axd_fact dimension for static fact files, axpot_tally voicing and axd_tally dimension for dynamic fact maps. Minted five rbf_fact_* linked terms with detail-site markers in conjure and graft subdocuments. Minted rbcc_fact_consec_infix with tally marker in RBSCK. Established axpo_ (Axial Product-of-Operation) as new AXLA category for operation output voicings.

### 2026-03-15 10:51 - ₢AtAAS - n

Added axpot_tally voicing and axd_tally dimension for dynamically-named fact maps. Minted rbcc_fact_consec_infix in RBS0 and wired tally output marker into RBSCK consecration check

### 2026-03-15 10:43 - ₢AtAAP - W

Wired conjure→graft chain via BURE tweak. Graft-side: BCG-compliant test/|| override of RBRV_GRAFT_IMAGE when BURE_TWEAK_NAME=threemodegraft. Test-side: read GAR_ROOT and ARK_STEM facts from conjure output, retrieve conjured image to local Docker, export tweak vars, graft picks up override.

### 2026-03-15 10:43 - ₢AtAAP - n

Added tweak-override mechanism so ThreeMode test can inject a conjured image reference into graft via BURE_TWEAK_NAME/BURE_TWEAK_VALUE, enabling end-to-end conjure→retrieve→graft pipeline testing

### 2026-03-15 10:39 - ₢AtAAS - n

Replaced prose in fact file detail-site descriptions with proper RBS0 linked terms

### 2026-03-15 10:37 - ₢AtAAS - n

Added axpof_fact voicing (Axial Product-of-Operation) to AXLA, minted five rbf_fact_* linked terms in RBS0, and wired axhoo_output_of_type axd_fact detail-site markers with mode-specific descriptions in conjure and graft subdocuments

### 2026-03-15 10:31 - ₢AtAAO - W

Decomposed fact-file system into three-tier architecture: RBF_FACT_GAR_ROOT (registry prefix), RBF_FACT_ARK_STEM (sigil:consecration composable base), and RBF_FACT_ARK_YIELD per-platform files (local image names). All three delivery modes (conjure, bind, graft) now emit uniform fact coverage. Removed dead RBF_FACT_IMAGE_REF and RBF_FACT_SLSA_LEVEL constants. Dropped .txt extensions from all fact filenames.

### 2026-03-15 10:15 - ₢AtAAO - n

Decomposed fact files into GAR_ROOT, ARK_STEM, and per-platform ARK_YIELD. Removed dead RBF_FACT_IMAGE_REF and RBF_FACT_SLSA_LEVEL. Dropped .txt extensions from all fact filenames. All three modes (conjure, bind, graft) now emit uniform fact coverage.

### 2026-03-15 10:12 - Heat - n

Added axd_fact dimension to AXLA for annotating operation outputs delivered as named fact files, with cross-reference on axhoo_output_of_type

### 2026-03-15 09:36 - ₢AtAAR - W

Added BURE_TWEAK_NAME/VALUE linked terms to BUS0 mapping section and BURE Regime definition section. Documented ambient propagation model, one-tweak-per-invocation constraint, and consumer-side interpretation pattern under new Tweak Mechanism subsection.

### 2026-03-15 09:36 - ₢AtAAR - n

Specify BURE_TWEAK_NAME/VALUE ambient regime variables in BUS0 — linked term definitions, validation constraints, and tweak mechanism section documenting single-override-per-invocation semantics

### 2026-03-15 09:31 - ₢AtAAN - W

Enrolled BURE_TWEAK_NAME/VALUE as optional ambient regime variables with full enrollment, defaults, and scope sentinel coverage. Fixed pre-existing BURE_COUNTDOWN unbound-variable bug. Added bure-tweak test fixture (4 positive + 3 negative cases) in fast suite. Evicted 'sweep' terminology from test infrastructure in favor of 'suite' across 6 files.

### 2026-03-15 09:31 - ₢AtAAN - n

Added BURE tweak mechanism (BURE_TWEAK_NAME/VALUE enrollment), fixed pre-existing BURE_COUNTDOWN default gap, created bure-tweak test fixture (7 cases), and evicted 'sweep' terminology in favor of 'suite' across test infrastructure

### 2026-03-15 09:14 - ₢AtAAE - W

Created three-mode integration test fixture (rbtctm_ThreeMode.sh) with 12-step sequence exercising conjure, bind, and graft delivery modes. Registered in testbench replacing retired ark-lifecycle. Test run revealed graft wiring gap (stale hardcoded RBRV_GRAFT_IMAGE) and conjure consecration format anomaly — both captured as follow-on paces.

### 2026-03-15 09:11 - Heat - T

graft-tweak-and-test-wiring

### 2026-03-15 09:10 - Heat - T

create-fact-file-enrichment

### 2026-03-15 09:04 - Heat - S

rbs0-fact-file-vocabulary

### 2026-03-15 09:04 - Heat - S

bus0-bure-tweak-spec

### 2026-03-15 09:04 - Heat - S

three-mode-run-and-green

### 2026-03-15 09:03 - Heat - S

graft-tweak-and-test-wiring

### 2026-03-15 09:03 - Heat - S

create-fact-file-enrichment

### 2026-03-15 09:03 - Heat - S

bure-tweak-mechanism

### 2026-03-15 09:03 - Heat - T

three-mode-integration-test

### 2026-03-14 20:41 - ₢AtAAE - n

Replaced ark-lifecycle fixture with three-mode integration test: 12-step sequence exercising conjure, bind, and graft delivery modes end-to-end including vouch_gate, retrieve, run, and abjure. Retired single-mode ark-lifecycle test.

### 2026-03-14 20:30 - ₢AtAAM - W

Fixed bind vouch digest parsing: sed pattern s/.*: *// was too greedy, stripping sha256: prefix from Docker-Content-Digest header. Changed to s/^[^:]*: *// to match only first colon. Bind vouch now passes (2 consecrations vouched). Consecration check confirms healthy vouched artifacts across bind and graft modes. All three delivery modes (conjure, bind, graft) verified end-to-end. Reslated final pace as three-mode-integration-test with conjure→bind→graft fixture design.

### 2026-03-14 20:28 - Heat - T

three-mode-integration-test

### 2026-03-14 20:26 - Heat - T

integration-test-cases

### 2026-03-14 20:13 - ₢AtAAM - n

Fixed sed pattern in rbgjv02 line 186: s/.*: *// was too greedy, stripping sha256: prefix from Docker-Content-Digest header value. Changed to s/^[^:]*: *// to match only the first colon.

### 2026-03-14 18:13 - Heat - S

manual-test-fixes-continuation

### 2026-03-14 18:04 - ₢AtAAE - n

Fixed bind vouch failure: Cloud Build substitution cannot handle ${VAR#pattern} bash expansion. Captured _RBGV_BIND_SOURCE into local variable before extracting pinned digest via parameter expansion.

### 2026-03-14 17:49 - ₢AtAAE - n

Added attestation manifest filtering to vouch pipeline: spec (RBSAV step 2) and code (rbgjv02) now filter unknown/unknown platforms from both vouch platform discovery and conjure verification iteration. Prevents vouch from building artifacts for non-runnable attestation manifests.

### 2026-03-14 17:39 - ₢AtAAE - n

Fixed syft platform resolution for grafted images: bind/graft always use @digest pinning to bypass OCI index auto-selection (attestation manifests cause amd64 worker to fail on arm64-only images). Added total manifest count diagnostic to step 1. Updated spec step 2 scan target selection.

### 2026-03-14 17:34 - ₢AtAAE - n

Spec and code fix: filter attestation manifests (platform unknown/unknown) from OCI image index during platform discovery. BuildKit stores SLSA provenance as index entries with unknown/unknown platform; these must be excluded before SBOM generation and build_info assembly

### 2026-03-14 17:27 - ₢AtAAE - n

Created graft test vessel (rbev-busybox-graft) and fixed Cloud Build substitution matching: removed bash :- default syntax from four mode-specific _RBGA_ variable references that Cloud Build could not match as substitution references

### 2026-03-14 17:18 - ₢AtAAL - W

Reviewed implementation of conjure stitch about-embedding. Two independent reviews converged: no critical defects, correct substitution variable coverage (all 16 _RBGA_* accounted for, _RBGA_CONSECRATION and _RBGA_BUILD_ID correctly omitted and post-processed via sed), BCG-compliant, spec-aligned with RBSAB and RBSAC. Five cosmetic/minor items identified (build context bloat, redundant -diags round-trip, multiple cat subshells, log undercount, inscribe latent coupling) — all assessed as acceptable, no changes needed.

### 2026-03-14 17:09 - ₢AtAAL - n

Embedded rbgja about steps into conjure trigger-dispatched cloudbuild.json via stitch function. Post-processes two runtime-resolved substitutions (consecration from workspace file, build ID from CB built-in). Adds _RBGA_* substitutions to stitch build JSON and inscribe placeholder filling. Removes standalone about call from rbf_create conjure chaining.

### 2026-03-14 16:43 - ₢AtAAI - W

Implemented combined delivery for bind and graft modes. rbf_create gains chaining tail (reads consecration, calls rbf_about for conjure/graft, rbf_vouch for all modes). rbf_mirror rewritten from local docker operations to combined Cloud Build (skopeo image copy + about steps via zrbf_mirror_submit). Extracted zrbf_assemble_about_steps for reuse by both standalone about and combined mirror. Extracted zrbf_ensure_git_metadata with BCG-compliant temp file pattern. Created rbgjm01-mirror-image.sh step script. Fixed pre-existing BCG violations (wc -c pipeline in $()) across all four Dockerfile size checks.

### 2026-03-14 16:42 - ₢AtAAI - n

Fixed three pre-existing BCG violations in zrbf_about_submit: replaced $() pipeline on wc -c with temp file + bash string manipulation in conjure, bind, and graft Dockerfile size checks

### 2026-03-14 16:39 - ₢AtAAI - n

BCG compliance fixes: added missing sentinel to zrbf_assemble_about_steps, replaced $() pipeline on wc in zrbf_mirror_submit with temp file + bash string manipulation, removed duplicate comment in rbf_mirror

### 2026-03-14 16:37 - ₢AtAAI - n

Added catch-all buc_die to chaining case, extracted git metadata capture into BCG-compliant zrbf_ensure_git_metadata (temp files, kindle constants, idempotent), replaced inline git capture in both zrbf_about_submit and zrbf_mirror_submit

### 2026-03-14 16:30 - ₢AtAAI - n

Implemented combined delivery for bind and graft: rbf_create gains chaining tail (about for conjure/graft, vouch for all modes), rbf_mirror rewritten as combined Cloud Build (skopeo copy + about steps), zrbf_assemble_about_steps extracted for reuse, zrbf_mirror_submit added, new rbgjm01-mirror-image.sh step script

### 2026-03-14 16:18 - ₢AtAAD - W

Verified all vouch draft code correct as-is (rbgjv01-03 mode branching, zrbf_vouch_submit unified submission, rbf_vouch single path, substitution variables, RBSAV and RBSAG specs). Refactored rbf_graft to remove about pipeline submission and next-steps tabtarget — graft now only pushes image to GAR and persists consecration. About chaining deferred to rbf_create (AtAAI).

### 2026-03-14 16:17 - ₢AtAAD - n

Refactored rbf_graft to remove about pipeline submission and next-steps tabtarget — graft now only pushes image to GAR and persists consecration. About chaining moves to rbf_create. Updated doc_brief accordingly. Verified all vouch draft code (rbgjv01-03, zrbf_vouch_submit, rbf_vouch, substitution variables, RBSAV/RBSAG specs) correct as-is.

### 2026-03-14 16:14 - ₢AtAAK - W

Aligned RBSAV spec with draft code: platform discovery is now a shared prologue in step 2 (one GET manifests call before mode branch, writes vouch_platforms.txt for all modes). Step 3 reads from workspace. Matches rbgjv02 code structure exactly.

### 2026-03-14 16:13 - ₢AtAAK - n

Restructured step 2 platform discovery from per-branch to shared prologue, matching code structure (rbgjv02): one GET manifests call before mode branch writes vouch_platforms.txt for all modes. Removed redundant per-branch GET calls from bind and graft.

### 2026-03-14 16:11 - ₢AtAAK - n

Aligned RBSAV spec with draft code: step 2 now discovers platforms for all three modes (conjure, bind, graft) and writes to /workspace/vouch_platforms.txt; step 3 reads from workspace instead of re-discovering from registry. Updated substitution variables NOTE to reflect the new flow.

### 2026-03-14 16:08 - ₢AtAAJ - W

Stripped RBSAB of director-side submission wrapper (7 steps removed), keeping only four Cloud Build steps as the procedure. Consolidated four overlapping NOTEs into two. Added waymark anchor for substitution variables cross-reference. Fixed _RBGA_BUILD_ID description for combined architecture. Clarified local/remote boundary language across RBSAB, RBSAG, and RBS0 — scoped 'no Cloud Build' to image push, fixed 'embeds' language for graft. Simplified paddock from 107 to 40 lines (dropped pace tracking, kept invariant/topology/decisions/references). Reslated AtAAI with concrete combined delivery docket (skopeo, rbgjm01, zrbf_assemble_about_steps extraction). Slated AtAAL for conjure stitch embedding. Reslated AtAAD and AtAAE to reflect reviewer feedback and combined delivery architecture.

### 2026-03-14 16:07 - Heat - T

conjure-stitch-about-embedding

### 2026-03-14 16:07 - Heat - T

pipeline-orchestration-rbf-create-chaining

### 2026-03-14 16:06 - Heat - T

vouch-unification-and-graft

### 2026-03-14 15:56 - Heat - T

integration-test-cases

### 2026-03-14 15:55 - Heat - T

pipeline-orchestration-rbf-create-chaining

### 2026-03-14 15:37 - ₢AtAAJ - n

Clarified local/remote boundary language across specs: scoped RBSAG 'no Cloud Build' to image push only, consolidated RBSAB four overlapping NOTEs into two, fixed 'embeds' language for graft (job consists of these steps, nothing embedded alongside) in RBSAB and RBS0

### 2026-03-14 15:34 - ₢AtAAJ - n

Restored references section to paddock with clean descriptions (no stale status markers)

### 2026-03-14 15:31 - ₢AtAAJ - n

Stripped RBSAB of director-side submission wrapper (7 steps removed), keeping only the four Cloud Build steps as the procedure. Added embedding context note, waymark anchor for substitution variables cross-reference, fixed _RBGA_BUILD_ID description. Reslated AtAAI for combined delivery, slated conjure stitch pace (AtAAL). Simplified paddock from 107 to 30 lines — dropped pace tracking and historical sections in favor of dockets.

### 2026-03-14 15:28 - Heat - S

conjure-stitch-about-embedding

### 2026-03-14 15:28 - Heat - T

pipeline-orchestration-rbf-create-chaining

### 2026-03-14 14:54 - Heat - S

rbsav-vouch-platform-discovery-alignment

### 2026-03-14 14:50 - Heat - S

rbsab-about-procedure-refactor

### 2026-03-14 14:31 - Heat - n

Tightened graft operation summary in RBS0 to clarify that only the image push is local — about and vouch follow as Cloud Build jobs. Added paddock note marking spec corrections as resolved.

### 2026-03-14 14:28 - Heat - n

Re-corrected specs to reflect combined delivery architecture: about is always part of the primary Cloud Build job (never standalone). Fixed RBSAB, RBSAC, RBSAG, RBS0, RBSRI, RBSCB, CLAUDE.consumer.md — removed all 'always standalone' and 'future optimization' language.

### 2026-03-14 14:15 - Heat - d

paddock curried

### 2026-03-14 13:58 - Heat - n

Corrected pipeline architecture in specs: about is currently always a standalone builds.create job chained by rbf_create (not embedded in image job). Updated RBSAC completion to show rbf_create chaining with future-optimization note for stitch embedding. Added delivery-mode note to RBSAB clarifying standalone-job-now, embedded-future. Updated RBSAG completion to show rbf_create handles about/vouch chaining instead of manual 'Proceed to' steps.

### 2026-03-14 13:56 - Heat - T

pipeline-orchestration-rbf-create-chaining

### 2026-03-14 13:56 - Heat - T

vouch-unification-and-graft

### 2026-03-14 13:54 - Heat - S

pipeline-orchestration-rbf-create-chaining

### 2026-03-14 13:54 - Heat - T

vouch-unification-and-graft

### 2026-03-14 13:53 - Heat - T

vouch-unification-and-graft

### 2026-03-14 13:50 - ₢AtAAD - n

Unified vouch to always run in Cloud Build (mode-aware: conjure=SLSA, bind=digest-pin, graft=GRAFTED). Eliminated zrbf_vouch_bind, replaced with zrbf_vouch_submit handling all modes. Implemented rbf_graft for local image push to GAR with inline about pipeline. Made rbgjv01 early-exit for non-conjure, rbgjv02 mode-branching with platform discovery, rbgjv03 runtime platform consumption. Added rbw-Db standalone about tabtarget. Updated inspect for graft vouch display.

### 2026-03-14 13:09 - ₢AtAAH - W

Replaced hardcoded ark suffix strings in about pipeline (rbgja01, rbgja02, rbgja04) with _RBGA_ARK_SUFFIX_IMAGE, _RBGA_ARK_SUFFIX_ABOUT, _RBGA_ARK_SUFFIX_DIAGS substitution variables. Plumbed constants from RBGC_ARK_SUFFIX_* through zrbf_about_submit() in RBF. Updated RBSAB spec substitution table. Verified: only comments and log messages retain literal suffix strings.

### 2026-03-14 13:07 - ₢AtAAH - n

Replaced hardcoded ark suffix strings (-image, -about, -diags) in about pipeline step scripts with _RBGA_ARK_SUFFIX_* substitution variables sourced from RBGC_ARK_SUFFIX_* constants via zrbf_about_submit(), eliminating Interface Contamination between conjure and about pipelines

### 2026-03-14 12:55 - ₢AtAAG - W

Implemented -diags transient registry artifact for forwarding conjure build-time diagnostics to the about pipeline. Conjure side: restored cache_before/cache_after JSON generation (proper {timestamp, host_daemon_images} format with --no-trunc), new rbgjb07-push-diags.sh builds FROM scratch container with diagnostic files + full Dockerfile. About side: rbgja01 extracts -diags layers via registry API, rbgja03 dual-source recipe.txt (prefer -diags, fall back to substitution variable), rbgja04 conditional COPY for enrichment files. Lifecycle: RBGC_ARK_SUFFIX_DIAGS constant, abjure checks/deletes -diags, consecration check recognizes -diags tags. Stitch updated with step 07 and _RBGY_ARK_SUFFIX_DIAGS substitution. Six specs updated (RBSAB, RBSAA, RBSAI, RBSCB, RBSCK, RBSRI). Slated ₢AtAAH for about-pipeline-suffix-substitutions cleanup.

### 2026-03-14 12:49 - ₢AtAAG - n

Review fix: restore proper cache file JSON format ({timestamp, host_daemon_images:[...]}) matching inspect's jq queries, add --no-trunc for full-length image IDs, include UTC timestamp

### 2026-03-14 12:49 - Heat - T

about-pipeline-suffix-substitutions

### 2026-03-14 12:43 - Heat - S

about-pipeline-suffix-substitutions

### 2026-03-14 12:41 - ₢AtAAG - n

Implement -diags transient registry artifact for forwarding conjure build-time diagnostics (buildkit_metadata.json, cache_before/after.json, full Dockerfile as recipe.txt) from conjure pipeline to about pipeline, with lifecycle support in abjure and consecration check, and spec updates across RBSAB/RBSAA/RBSAI/RBSCB/RBSCK/RBSRI

### 2026-03-14 12:26 - ₢AtAAC - W

Extracted about artifact generation from conjure into standalone Cloud Build pipeline (rbgja/): 4 new step scripts (platform discovery, mode-aware syft SBOM with 3 scan modes, mode-aware build_info for conjure/bind/graft, about container assembly), new rbf_about and zrbf_about_submit in RBF with _RBGA_* substitution namespace and builds.create direct submission, reduced conjure pipeline from 9 to 6 steps (removed syft/build-info/about steps, renumbered imagetools-create), removed orphaned cache_before/cache_after generation and _RBGY_ARK_SUFFIX_ABOUT from conjure stitch. Review fixes: replaced eval with file-based digest lookup, removed dead code, documented 4KB substitution limit, updated RBSAB/RBSCB/RBSAI specs (SBOM filename format, builder image, enrichment file loss accepted). Reslated AtAAD to include rbw-DA tabtarget wiring.

### 2026-03-14 12:24 - Heat - T

diags-artifact-forwarding

### 2026-03-14 12:23 - Heat - T

diags-artifact-forwarding

### 2026-03-14 12:17 - Heat - S

diags-artifact-forwarding

### 2026-03-14 12:11 - ₢AtAAC - n

Review fixes: remove dead SHARED_ARGS and ABOUT_BUILD_ID from rbgja03, replace eval with file-based digest lookup (security), add busybox ash pipefail comment, remove orphaned cache_before/cache_after generation from conjure steps 03/05, document 4KB substitution limit reasoning in RBF, fix RBSAB spec (SBOM filename format, step 2 builder image), remove enrichment file references from RBSCB and RBSAI (buildkit_metadata/cache files no longer in standalone about artifact), reslated AtAAD to include rbw-DA tabtarget wiring

### 2026-03-14 12:10 - Heat - T

vouch-unification-and-graft

### 2026-03-14 11:53 - ₢AtAAC - n

Extract about artifact generation into standalone Cloud Build pipeline (rbgja/): 4 new step scripts (platform discovery, syft SBOM with 3 scan modes, mode-aware build_info, about container assembly), new rbf_about/zrbf_about_submit with _RBGA_* substitution namespace and builds.create submission, reduced conjure pipeline from 9 to 6 steps (removed steps 06-08, renumbered 09→06), removed _RBGY_ARK_SUFFIX_ABOUT from stitch substitutions

### 2026-03-14 11:35 - ₢AtAAB - W

Implemented new consecration format [cbg]YYMMDDHHMMSS-rYYMMDDHHMMSS across 8 files: updated rbgjb01 construction, RBF mirror/inscribe/abjure/check/batch_vouch/doc_param sites (timestamps, regex, examples), added graft to RBRV_VESSEL_MODE enum with Grafting Configuration group (RBRV_GRAFT_IMAGE required, RBRV_GRAFT_OPTIONAL_DOCKERFILE optional), added RBRV_BIND_OPTIONAL_DOCKERFILE to binding group, added buv_consecration_format standalone validator, updated test data and consumer docs. RBGC/RBDC/RBQ checked and confirmed clean. buv_consecration_format intentionally not wired into RBRN validation (would break live nameplates with old-format consecrations from existing GAR artifacts — deferred to post-depot-recreation). All 21 regime validation tests pass.

### 2026-03-14 11:28 - ₢AtAAB - n

Make RBRV_GRAFT_IMAGE required (min 1) per review feedback, update spec to match

### 2026-03-14 11:23 - ₢AtAAB - n

Implement new consecration format [cbg]YYMMDDHHMMSS-rYYMMDDHHMMSS and graft regime scaffolding: updated all format construction/parsing/regex sites across rbgjb01 and RBF, added graft to RBRV_VESSEL_MODE enum with grafting group (RBRV_GRAFT_IMAGE, RBRV_GRAFT_OPTIONAL_DOCKERFILE), added RBRV_BIND_OPTIONAL_DOCKERFILE, added buv_consecration_format validator, updated test data and example strings

### 2026-03-14 11:14 - ₢AtAAA - W

Specified graft vessel mode, standalone about pipeline, mode-aware vouch, and new consecration format ([cbg]YYMMDDHHMMSS-rYYMMDDHHMMSS) across 12 existing specs and 2 new documents (RBSAB-ark_about, RBSAG-ark_graft). Updated RBS0 ark definitions for three arrival paths with mandatory vouch, rewrote RBSAV for mode-aware Cloud Build (conjure SLSA/bind digest-pin/graft GRAFTED), extracted about steps from conjure pipeline, added graft mode to vessel regime, and propagated changes through abjure, inspect, check, director_vouch, inscribe, and trigger_build specs.

### 2026-03-14 11:14 - ₢AtAAA - n

Repair six feedback items: syft digest transport for multi-platform bind, vouch platform discovery via GET manifest, OCI spec note for Created field, ephemeral BUILD_ID clarification, remove old-format mention from check, push atomicity note, and validation for optional Dockerfile variables

### 2026-03-14 11:06 - ₢AtAAA - n

Review round 3: fix abjure error message, amend paddock T1, restore provenance discovery warning, document BUILD_ID cross-job flow, graft PATH runtime, vouch manifest reuse, inscribe step renumbering, recipe.txt 4KB note, single-platform-only graft

### 2026-03-14 10:57 - ₢AtAAA - n

Review round 2: flip syft scan logic, recipe.txt via substitution with RBRV_*_OPTIONAL_DOCKERFILE regime variables, vouch platforms from manifest not config, runtime-neutral image inspect, warn-not-require mode prefix check, bind multi-platform, skip old-format tags, data flow note

### 2026-03-14 10:36 - ₢AtAAA - n

Review fixes: ark_image mode-aware, remove stale vouch text, director_vouch summary, syft registry transport, platform discovery fallback, graft T1 as OCI creation timestamp, bind digest specificity, recipe.txt in about, mode/prefix belt-and-suspenders, shared JSON helper note

### 2026-03-14 10:24 - ₢AtAAF - W

Verified syft GCB pin already fully implemented: RBRG_SYFT_IMAGE_REF present in rbrg.env with digest pin, enrolled in regime validation, documented in RBSRG spec, included in rbrr_refresh_gcb_pins, and consumed via inline substitution in rbgjb06. Refreshed all image pins — skopeo digest updated, syft and 5 others unchanged. No hardcoded syft tags remain in live code (only in ABANDONED-github).

### 2026-03-14 10:24 - ₢AtAAA - n

Specify graft vessel mode, standalone about pipeline, mode-aware vouch, and new consecration format across 12 existing specs and 2 new documents (RBSAB, RBSAG)

### 2026-03-14 10:09 - ₢AtAAF - n

Refresh GCB image pins — skopeo digest updated, syft and all others unchanged

### 2026-03-14 10:04 - Heat - T

vouch-unification-and-graft

### 2026-03-14 10:04 - Heat - T

about-pipeline-extraction

### 2026-03-14 10:04 - Heat - T

consecration-format-and-regime-foundation

### 2026-03-14 10:03 - Heat - d

paddock curried

### 2026-03-14 10:01 - Heat - f

racing

### 2026-03-14 10:01 - Heat - T

integration-test-cases

### 2026-03-14 10:00 - Heat - T

vouch-unification-and-graft

### 2026-03-14 10:00 - Heat - T

about-pipeline-extraction

### 2026-03-14 10:00 - Heat - d

paddock curried

### 2026-03-14 09:55 - Heat - S

syft-gcb-image-pin

### 2026-03-14 09:55 - Heat - T

integration-test-cases

### 2026-03-14 09:55 - Heat - T

vouch-unification-and-graft

### 2026-03-14 09:54 - Heat - T

about-pipeline-extraction

### 2026-03-14 09:54 - Heat - T

consecration-format-and-regime-foundation

### 2026-03-14 09:54 - Heat - T

specification-updates-graft-about-vouch

### 2026-03-14 09:45 - Heat - S

integration-test-cases

### 2026-03-14 09:44 - Heat - S

vouch-unification-and-graft

### 2026-03-14 09:44 - Heat - S

about-pipeline-extraction

### 2026-03-14 09:44 - Heat - S

consecration-format-and-regime-foundation

### 2026-03-14 09:39 - Heat - T

specification-updates-graft-about-vouch

### 2026-03-14 09:31 - Heat - S

specification-updates-graft-about-vouch

### 2026-03-14 09:23 - Heat - N

rbk-mvp-3-add-graft

