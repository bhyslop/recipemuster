# Governor mantle tombstone leak — H1's scoped-out cousin (keyfile tier)

*2026-06-05. Surfaced while live-validating rbk-08-credential-repairs' H1/H2 fix
against the canonical standing depot `cancbhm-d-canest3bhm100001`. Captured as input
to rbk-14-mvp-prepare-federation-idea, whose generic-teardown convergence would
subsume the fix — so this is a concept capture, not a call to build now.*

## Observation

`gcloud projects get-iam-policy cancbhm-d-canest3bhm100001` (payor token, 2026-06-05)
shows **8** `deleted:serviceAccount:governor-<datestamp>@…?uid=…` tombstones, and the
count rose **7→8** across a single `canonical-invest` run — one fresh ghost per
governor re-mantle. The grounding memo recorded ~30 governor-owner ghosts pre-H1-fix;
the leak has run unbounded since.

## Mechanism — H1, one tier up

`rbgp_governor_mantle` (`Tools/rbk/rbgp_Payor.sh`, "Clean up existing governor-*
service accounts") lists every `governor-*` SA and **DELETEs each one before minting a
new datestamped `governor-YYYYMMDDHHMM`** — with **no revoke of its bindings first**.
The outgoing governor holds `roles/owner` (plus its establish grants) on the depot
project; deleting a still-bound member is exactly the case GCP cannot drop — it
retags the binding `deleted:…?uid=` and keeps it. Identical to H1 (director/retriever
divest deleting before revoking), on the governor/mantle path. The caller is the
**Payor** (project owner), so the leak is purely the missing revoke, not a permission
gap.

This was deliberately out of scope for rbk-08: the grounding memo's point 8 cinched
"Governor out of scope (deliberate single-governor, datestamped, low-churn)." Correct
call for that pace — but the hole is real and unbounded, marching toward GCP's
~1500-member allow-policy cap like its siblings.

## Three ways it closes — in increasing order of remodel

1. **Targeted fix (no remodel).** Wire revoke-before-delete into the mantle delete
   loop: for each outgoing `governor-*`, `rbgi_revoke_project_member` its known grants
   (`roles/owner` + the establish set) before the DELETE. All project-scope, caller is
   the Payor who already owns the project → **lean** (no Class-C tolerance needed —
   simpler than the director case, whose repo/SA resource-scope revokes needed it).
   Smallest possible change; reuses the rbk-08 revoke layer verbatim.

2. **Subsumed by rbk-14 (this heat).** rbk-14 converges the admin verbs so teardown is
   "generic across roles" and `mantle` becomes `remove`+`add` parameterized by
   capability. Two sub-outcomes, both fix the leak:
   - if the governor stays delete+recreate, the generic revoke-before-delete covers it
     like any role;
   - better, if the convergence makes the governor **idempotent** (establish-once +
     rekey, as H2 did for director/retriever), the delete disappears entirely — no SA
     churn, no datestamp, no tombstone.
   Either way a separately-built targeted fix (1) becomes **wasted work** once rbk-14
   lands.

3. **Obviated by federation (beyond rbk-14).** The federation tier (RBSHR;
   `memo-20260527-operator-credential-models.md`) has no per-cycle SA at all — stable
   `principal://` subjects with central revocation — so there is no governor SA to
   tombstone.

## Caveat for rbk-14 implementers — equivalent problems

The convergence's generic governor teardown inherits rbk-08's propagation lessons and
should **reuse, not reinvent**:

- revoke-before-delete on the governor needs the `rbgi_revoke_*` layer (built, proven);
- any ordering where a freshly-minted governor revokes the outgoing one's bindings is a
  **Class-C** (caller-recently-empowered) site against whatever scope it touches — the
  exact race rbk-08 mapped (RBSCIP) and tolerated;
- a delete-then-recreate governor would face the SA delete read-path flap
  (`RBGC_GONE_CONFIRM_STREAK`); datestamped names currently sidestep it, idempotent
  rekey eliminates it.

## Recommendation

Do **not** build the targeted fix now — it is most likely subsumed by rbk-14. This
memo is the capture so the concept is not lost; rbk-14's paddock references it as the
fallback for the no-convergence (or governor-descoped) branch, and as the flag that
governor teardown sits inside this heat's blast radius and must reuse the rbk-08
propagation machinery.

## Sources

- Code: `rbgp_governor_mantle` (`Tools/rbk/rbgp_Payor.sh`, governor cleanup loop);
  `rbgi_revoke_project_member` (`Tools/rbk/rbgi_IAM.sh`).
- `Memos/memo-20260604-credential-churn-leak-and-propagation-races.md` (H1/H2 + the
  point-8 governor scope-out; revoke-side Class-C addendum).
- RBSCIP propagation classes (Class C, caller-recently-empowered).
- rbk-14-mvp-prepare-federation-idea paddock: generic-teardown convergence.
