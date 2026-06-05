## Boundary — shape, not mechanism (cinched)

This heat adopts the federation *shape* — the identity model and the converged
cult verbs — without building the federation *mechanism*. The backing stays
keyfile (service-account keys); a no-org operator runs it unchanged. The payoff:
a later switch to federation changes only what is behind the token — every
command and concept the operator has learned stays identical, so the switch is
non-disruptive. Explicitly OUT of scope: Workforce pool/provider setup, STS
token exchange, IdP device flow, and the RBRD mode-enum activation. Those belong
to the federation tier's own future work (RBSHR "Operator federation";
memo-20260527).

## The target model

- **Identity** = one principal per person, named for the person, not the role.
  In keyfile backing a principal is a service account holding one RBRA key; the
  role prefix (`director-…`) leaves the SA name.
- **Capability** = the IAM grants on that identity. A role (director, retriever,
  governor) becomes a named capability-set — the IAM-role list currently
  hardcoded inside the per-role invest verbs — that a grant verb applies. The
  same set applies identically to an SA member (keyfile) or a `principal://`
  member (federation); that sameness is the convergence.
- **Verbs converge** to two axes: add/remove a person (identity lifecycle) and
  grant/revoke a capability for a person (membership) — plus rekey (keyfile-only
  key rotation) and roster (read). These map onto the credential-lifecycle split
  already underway: add = establish, grant/revoke = the binding lifecycle (revoke
  closing the never-wired teardown half, now generic across roles), rekey =
  key-only rotation, remove = teardown.
- **One mode-seam**: only "add person" (mint SA+key vs register a federated
  principal) and the token accessor (signed-JWT vs STS) ever branch on backing.
  Everything above is backing-blind.

## Why this is tractable (the load-bearing finding)

The token layer is already identity-based: a minted token asserts only "I am this
SA," and Google grants whatever that SA's IAM bindings allow — enforcement is
server-side and un-hackable. Role-typing is cosmetic, living only in the SA name
prefix and the per-role selection constants. So this is mostly removing a naming
convention atop an already-identity-based mechanism, not rebuilding enforcement.

## Cinched decisions

- Enforcement is server-side IAM. No file or client-side check is ever the
  boundary; any honor-system gate a malicious operator bypasses by not running
  our client.
- Roster derives from IAM (an `rbk-identity` label marks RBK-managed SAs once the
  role leaves the name), not a stored list — it cannot drift.
- Capability-sets become named code (lift the inline IAM-role lists out of the
  invest verbs); they are not a config file.
- The mode enum (keyfile vs federation) belongs in RBRD (depot-uniform,
  tripwire-protected) and is added only when federation lands — not in this heat.
- Payor is a capability on the grant/revoke axis like the others, but its
  identity lifecycle (install/refresh) branches because its backing is a human
  OAuth refresh token, not an SA key. RBRP already encodes this ("not every
  operator has a Payor identity").
- Migration grandfathers existing role-named SAs: backfill the label, read grants
  from IAM, let legacy `director-…`/`retriever-…` names age out. No flag day.

## Open forks (resolve within the heat)

- **Governor capability breadth.** Governor holds `roles/owner` today, so it can
  already mint peer admins. Keep it broad and catch dangerous unions by roster
  audit, or scope it so the rung is a hard IAM wall — the latter needs IAM
  Deny/Conditions because project `setIamPolicy` is not role-scoped. Same question
  federation will later ask of the governor principal.
- **Intended-roster.** Whether to store a declared roster (names → intended
  capabilities) alongside the IAM-derived actual, for legibility and an
  intended-vs-actual audit.

## What done looks like (the tectonic surface)

Every cult-verb surface migrated to the converged identity/verb vocabulary, not
just the bash that mints tokens:

- workstation bash (the ~30 role-keyed selection sites collapse to one identity);
- the governor/payor admin verbs (invest/divest/roster/mantle become
  add/grant/revoke/remove parameterized by capability);
- cloud-side bash wherever it names roles;
- the .adoc specs that voice the cult verbs;
- the Rust test cases (theurge) that drive them;
- the onboarding handbook tracks that teach them.

## Sources

The per-person design exploration in rbk-08-credential-repairs that birthed this
heat; memo-20260604-credential-churn-leak-and-propagation-races (H1/H2 lifecycle
split — the enabler); memo-20260527-operator-credential-models (the two-tier plan
and cult-verb shape); RBSHR "Operator federation".