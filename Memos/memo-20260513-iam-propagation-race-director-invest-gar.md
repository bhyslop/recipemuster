# IAM Propagation Race — Director Invest, GAR Repo IAM Policy

**Date:** 2026-05-13
**Observed in:** ₣A_ ₢A_AAF, post-depot-swap SA chain reconstruction on canest2bhm100003
**Disposition:** Disappeared on retry after divest+re-invest (so ~30s additional propagation time)

## Summary

`tt/rbw-adI.GovernorInvestsDirector.sh t3d` failed partway through with HTTP 403 on `artifactregistry.repositories.getIamPolicy` against the freshly-levied depot's GAR repository. The governor service account that owned the OAuth token had been minted ~30 seconds prior with `roles/owner` on the depot project. Cloud Build Editor and serviceAccountUser grants in the same script invocation succeeded immediately before the GAR call failed. Divesting the partial director SA and re-invoking invest succeeded, with the AR roles applied cleanly.

## Trigger conditions

Sequential invocation of, on a brand-new depot project:

1. `tt/rbw-aM.PayorMantlesGovernor.sh` — mints governor SA, grants `roles/owner` on depot project
2. `tt/rbw-arI.GovernorInvestsRetriever.sh <id>` — succeeded
3. `tt/rbw-adI.GovernorInvestsDirector.sh <id>` — failed at AR step

Total wall clock from governor mantle completion to director-invest GAR call: roughly 90 seconds (one retriever invest in between, including its own propagation waits).

## Failure signature

```
[rbgg_invest_director] Adding Cloud Build Editor role (project scope)
[rbgg_invest_director] Grant serviceAccountUser on Mason
[rbgg_invest_director] Grant Artifact Registry roles (complete expected policy)
[rbgg_invest_director] ERROR: Get GAR repo IAM policy (HTTP 403):
  Permission 'artifactregistry.repositories.getIamPolicy' denied on resource
  '//artifactregistry.googleapis.com/projects/cancbhl-d-canest2bhm100003/locations/us-central1/repositories/cancbhl-canest2bhm100003-gar'
  (or it may not exist).
```

The "or it may not exist" clause is GCP's standard tell-nothing 403 wording. `gcloud artifacts repositories list --project=cancbhl-d-canest2bhm100003` confirmed the repo existed (CREATE_TIME ~30 minutes prior). The 403 was strictly an IAM-side propagation lag, not a missing resource.

## Recovery

```
tt/rbw-adD.GovernorDivestsDirector.sh <id>
tt/rbw-adI.GovernorInvestsDirector.sh <id>
```

Divest succeeded immediately. Re-invest succeeded end-to-end with no further intervention. The cumulative additional wait was on the order of 30 seconds.

## Asymmetry observed

Within the same script invocation that failed, two earlier IAM operations against different scopes succeeded against the same governor token:

- `roles/cloudbuild.builds.editor` at **project scope** — succeeded
- `roles/iam.serviceAccountUser` on the Cloud Build mason SA at **service-account scope** — succeeded
- `getIamPolicy` on GAR repo at **artifact-registry-resource scope** — failed 403

This suggests the IAM grant of `roles/owner` propagated to project-level and SA-level IAM caches faster than to artifact-registry-resource-level IAM caches. The script's other propagation retry helpers (notably the SA-key-generation propagation retry) did not cover this call site.

## Possible repair vectors (out of scope for current pace)

- Add a propagation-retry wrapper around the GAR `getIamPolicy` / `setIamPolicy` calls in `rbgg_invest_director` parallel to the SA-key-generation retry already present.
- Insert a deliberate post-mantle wait that covers the AR-resource-IAM propagation window before any subsequent invest call's first AR touch.
- Detect 403 on AR IAM specifically and treat as transient rather than fatal, with bounded retry.

The recovery posture (divest+re-invest) is operationally workable but mints a fresh SA UID and regenerates the rbra.env, both of which are heavier than a transparent retry would be.

## Why this is filed

The failure surfaces only on a fresh-depot SA chain reconstruction (rare path — happens at depot levy time, not during ongoing operation). The "disappeared on retry" character means a single test run can show green or red depending on cumulative propagation time across the chain. Capture so the next encounter surfaces a known-quantity rather than a fresh diagnostic.
