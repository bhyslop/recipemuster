## Context

Two threads deepening tadmor's adversarial coverage of sentry containment, paired with the tooling lift required to produce trustworthy evidence:

1. **Scry revival and improvement** — `rbw-cs.Scry` is the network-traffic observation tool used during tadmor runs to verify what an attacker's traffic actually reaches. The tool exists but has gaps that force operators into ad-hoc workarounds. Six paces extend it: sentry-interface coverage, charge-aware lifecycle, finite-duration exit (composability), filter argument (signal-to-noise), docker-bridge capture (coverage extension), and plain-output mode (scriptable consumption).

2. **New tadmor cases** — three adversarial cases extending the existing tadmor-security fixture: an in-bottle empirical regression backstop for POSTROUTING MASQUERADE source-IP behavior, a deferred outside-as-enclave-IP spoof (needs host-side raw-socket sending, not in-bottle ifrit posture), and a consolidation of source-IP spoof parameterization across the existing and new cases.

## Scope and boundaries

- **Test-side and tool-side only.** Sentry hardening itself — the entry-port DNAT-forward rescope, related spec deltas, sentry-config presence assertions — stays in `₣A_`. That hardening work is mid-flight there with code already landed; this heat consumes the new sentry behavior in tests, it doesn't drive it.
- **Scry tool changes are non-breaking.** Plain-output and filter-argument are additive ergonomics; charge-aware lifecycle and finite-duration exit make the tool usable from automation. The existing interactive invocation path keeps working.
- **One deferral is locked.** The outside-as-enclave-IP spoof case is intentionally not implementable in-bottle — it needs a host-side raw-socket sender. The pace records the deferral and the reason; mounting it produces a host-side stub or a stronger deferral, not an in-bottle attempt.

## Done

Scry is comfortable to drive from automation (filter, plain output, finite-duration exit, charge-aware lifecycle all wired). The new tadmor cases land green in the tadmor-security fixture without flake. Source-IP spoof parameterization is consolidated across the cases (or the consolidation pace is explicitly declined on shape grounds at mount).

## Cross-heat reference

The source-IP-spoof consolidation pace references prior ifrit-sortie work that landed in `₣A_` (enclave-spoof-external). That work is complete and stable; the reference is for context, not dependency.