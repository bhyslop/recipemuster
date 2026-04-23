## Context

Three interwoven infrastructure cleanups culminating in a depot regeneration:

1. **Resource prefixing** (`₢A_AAA`/`₢A_AAB`/`₢A_AAC`) — introduce `RBRR_CLOUD_PREFIX` (cloud-visible names: GAR, Cloud Build, GCP) and `RBRR_RUNTIME_PREFIX` (local container and network names). Enables multi-regime isolation on shared infrastructure.

2. **GAR categorical layout migration** (`₢A_AAK`/`₢A_AAL`) — reorganize GAR top-level namespace into three prefixed categories. Dissolves the `<vessel>:<hallmark>-<suffix>` hyphen-suffix scheme and the `vouches/` aggregator. Tabtargets taking `(vessel, hallmark)` simplify to `(hallmark)`.

3. **Payor credential subdirectory** (`₢A_AAD`) — move `rbro.env` into a `payor/` subdirectory to match the role-subdirectory pattern already established for RBRA credentials.

The depot regen at the tail (`₢A_AAE`) is the hard cutover point. All three threads land before it; depot regen validates the new infrastructure in regenerated form.

## Design decisions captured

### GAR categorical layout

Three top-level namespaces under the cloud prefix:

```
<prefix>hallmarks/<hallmark>/{image, vouch, pouch, about, attest, diags}
<prefix>reliquaries/<date>/<tool>
<prefix>enshrines/<base>
```

Each immediate child of `<prefix>hallmarks` IS a hallmark; the six ark types become plain basename siblings. No more vessel-in-path or suffix-on-tag string grammar.

### Vessel identity

No longer lives in the GAR path. Resolved on demand from the `vouch` or `about` metadata of the hallmark in question. `ls <prefix>hallmarks/` no longer surfaces vessel at a glance; tally and plumb already read metadata for health, and extend the pattern where operation prose needs the vessel name.

### Attest vs vouch

Both persist as peer basenames under `<hallmark>/`. Attestations advertise during build for SLSA provenance; vouches check them at consumption. Load-bearing split — retained by design.

### Pouch

A `FROM scratch` OCI image carrying build context (Dockerfile + supporting files) to Cloud Build's first step. Present for conjure/bind, absent for graft (graft is local push — no GCB involved). Now lives at `<prefix>hallmarks/<hallmark>/pouch` in the new layout.

### SUFFIX → BASENAME constant rename

`RBGC_ARK_SUFFIX_*` constants persist but rename to `RBGC_ARK_BASENAME_*`; their values drop the leading hyphen. Role shifts from "suffix to hyphen-concat" to "literal basename of a sibling file." The concat grammar disappears entirely from ark path construction.

### `vouches/` aggregator removal

`RBGC_VOUCHES_PACKAGE` existed to enable hallmark-only vouch lookup without knowing the vessel. The new layout delivers the same property naturally — every hallmark has its own `vouch` basename under `<prefix>hallmarks/<hallmark>/`. The aggregator becomes redundant and is deleted.

### Cutover discipline

Hard cutover — old and new path shapes do not coexist in live code. Depot regen at `₢A_AAE` is the forcing function; all layout work lands before it.

## References

- `₢A_AAJ` — filesystem-side config-directory naming research (orthogonal to this heat). Decisions recorded in that pace: `rbmm_moorings` umbrella, `rbm*_` prefix family, hard-cutover at depot regen. Filesystem rename itself flows to a separate follow-up heat and is out of this heat's scope.
- `Tools/rbk/rbgc_Constants.sh:122-131` — current ark-suffix constants and `RBGC_VOUCHES_PACKAGE`; this heat renames to `_BASENAME_*` and removes the aggregator.
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh:712-733` — pouch definition and current concat-based path construction.
- `Tools/rbk/rbfl_FoundryLedger.sh:440-650` — current per-suffix HEAD/DELETE dance; collapses to subtree operations in `₢A_AAK`.