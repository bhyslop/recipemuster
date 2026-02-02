# Paddock: rbw-implement-gar-mirroring

## Context

Implement container image mirroring from upstream registries (Docker Hub, Quay, etc.) to Google Artifact Registry. This enables Recipe Bottle builds to use pinned, locally-controlled copies of base images rather than depending on external registry availability.

## Ideas for Future Paces

### Mirroring Infrastructure

- **rbgjm_mirror.json** is the existing mirror configuration template
- Mirror operations run in Cloud Build using gcrane
- Need to define which images to mirror and version pinning strategy

### Constant Plumbing

The mirror JSON currently hardcodes `-docker.pkg.dev`:
```json
"${_RBGY_GAR_LOCATION}-docker.pkg.dev"
```

Should plumb RBGC constants to Cloud Build similar to ₢APAAR approach:
- `_RBGY_GAR_HOST_SUFFIX` from `RBGC_GAR_HOST_SUFFIX`

This aligns mirroring with the ark build infrastructure.

### Image Pin Management

- Where do we track pinned image digests?
- How do we update pins when upstream releases security patches?
- Integration with RBRR regime for mirror configuration?

### Triggering

- Manual mirror operation vs automated refresh
- How to know when upstream has updates worth mirroring

## References

- `Tools/rbw/rbgjm_mirror.json` — Mirror Cloud Build template
- `Tools/rbw/rbf_Foundry.sh` — Build orchestration (substitution pattern)
- `Tools/rbw/rbgc_Constants.sh` — RBGC constants to plumb
- ₢APAAR in ₣AP — Related work plumbing ark constants to GCB

## Heat Nature

**Planning/design heat** — no paces yet. Accumulate ideas here, then slate paces when ready to implement.
