## Character

Junk-drawer of small pre-MVP loose ends —
the residual engineering that accreted under the release heats
once the release-qualification machinery shipped.
As the project approaches MVP the remaining issues trend small and self-contained;
this heat is their shared home,
with no pretense of a single goal beyond settling them before release.

Two loose bands, ordered robustness ahead of hygiene:

- Robustness that smooths a clean release run —
  a credential-flap tolerance,
  an OAuth terminal-failure fail-fast,
  and a build-poll queue/execution clock split.
- Hygiene and cleanup —
  a dormant station regime relocated off the MVP surface,
  a shellcheck-coverage widening past the `Tools/` root,
  two stale Cloud Build spec references,
  a network-diagnostic tool repair,
  and a superseded build-bucket scrub.

Paces stand alone;
order is loose priority, not a dependency chain.
The two lowest-value tail paces (the network-diagnostic repair and the stale spec references)
are itch-candidates if the heat needs thinning rather than finishing.

## Coupling to watch

The credential-flap tolerance and the OAuth fail-fast both touch `rbgo_get_token_capture`;
the fail-fast pace's own docket asks whether it is the same landing as the flap-tolerance touchpoint,
so mount those two adjacent or fold them together.

## Provenance

Restrung from ₣BB and ₣BU.
₣BB built and shipped the release-qualification machinery — its goal is complete;
these paces were ordinary engineering that had collected under it, never release ceremony.
₣BU holds the actual release runs and the RELEASE.md revision,
parked stabled as paddock-memory until release time —
that work is not here and is not reopened by this heat.
Both source heats keep the git history of any dropped dockets.

## References

- ₣BB — shipped the release-qualification machinery; holds the design record of what was built
- ₣BU — the deferred release runs and doc revision (stabled until release time)