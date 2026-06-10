# ₣BH Fable Recommendation — Self-actAs Propagation Flap at Delete-Build Submit

Date: 2026-06-10

Status: Recommendation from the Fable review of the cloud-dispatch delete architecture
(commit 4f8a5c703). Not observable on a standing depot; will surface on a pristine gauntlet.

## The correctable behavior

`rbgg_invest_director` (`Tools/rbk/rbgg_Governor.sh`) now grants the Director SA
`roles/iam.serviceAccountUser` **on itself** — the actAs binding that `builds.create` checks
when the Director-authenticated workstation submits a build whose run-as identity is the
Director SA (`zrbld_spine_dispatch`, `Tools/rbk/rblds_Spine.sh`).

This binding joins the known IAM-propagation race class (Class-C;
memo-20260513-iam-propagation-race-director-invest-gar,
memo-20260604-credential-churn-leak-and-propagation-races): freshly granted, it can lag.
The first delete build submitted shortly after a pristine invest may fail `builds.create`
with PERMISSION_DENIED on the actAs check — a flap, not a defect, but the submit path has no
tolerance for it: `zrbld_spine_dispatch` does `rbuh_json POST` + `rbuh_require_ok` and dies
on the first refusal.

The live smoke (depot canest3bhm100001) ran against a standing depot with long-settled
grants, so this path is **unexercised**. The service-tier verification pace also runs on the
standing depot and will not catch it; only a pristine-depot gauntlet (invest → promptly
banish/abjure) will.

## Recommended repair

Pick one (the first is cheaper and matches existing patterns):

1. **Invest-side read-back gate** — extend the rbk-08 read-back-verification pattern (RBSHR
   "Read-back verification universalization") to the self-actAs binding: after the grant in
   `rbgg_invest_director`, poll the SA IAM policy until the binding is visible before
   declaring invest complete. Confines the wait to invest, keeps the submit path simple.
2. **Submit-side Class-C tolerance** — in `zrbld_spine_dispatch` (or a delete-specific
   wrapper in `zrbld_cloud_delete_dispatch`), bounded retry on the specific
   PERMISSION_DENIED-actAs signature, mirroring the ~81s Class-C tolerance shape. Broader
   blast radius (touches all spine riders), so prefer (1).

Either way, characterize the live signature on the first pristine gauntlet rather than
guessing its exact error body — the lesson of the code-5/code-9 correction this same pace
just taught.
