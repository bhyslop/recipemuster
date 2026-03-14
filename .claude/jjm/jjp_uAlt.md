# Paddock: rbk-mvp-3-add-graft

## Core invariant

Every mode produces -image and -about through a single primary Cloud Build job, then vouch runs as a second Cloud Build job. Two Cloud Build round-trips per mode. The operator invokes one command (`rbf_create`); the system handles the full pipeline.

For conjure and bind, the primary job combines image production with about generation. For graft, the image arrives via local push, so the primary Cloud Build job degenerates to about-only — but it is still the single primary job, not a separate pipeline stage.

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

## Key design decisions

- No crane for graft: docker tag + docker push is sufficient (image already local)
- No dirty-tree guard for graft: the container is already built; git state does not affect it
- Vouch Cloud Build steps: rbgjv01 early-exits for bind/graft; rbgjv02 branches on _RBGV_VESSEL_MODE
- Combined conjure job: about steps embedded in trigger-dispatched cloudbuild.json via stitch. _RBGA_CONSECRATION read from workspace (not substitution) since consecration is computed at build time.
- Combined bind job: image copy + about steps in a single builds.create Cloud Build job. Mason SA pulls from upstream (public images; private upstream auth is out of scope).
- Graft degenerate primary job: about-only builds.create job after local push. Same rbgja scripts, same Cloud Build submission pattern, no image step.
- Vouch separate from conjure: SLSA provenance is a post-build artifact. Cannot verify provenance from within the same build. Hard constraint.
- rbw-DA is taken by abjure. Standalone about recovery tabtarget uses rbw-Db.
