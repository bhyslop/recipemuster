# Workforce Identity Pool Constraints — foedus quota / soft-delete (heat ₣BZ, 260617)

## Why this memo

The affiance/jilt proof (affiance-proof pace) and the plan to nucleate a foedus
test fixture raised a quota/debris concern.
The project has been bitten before by the 30-day soft-delete of *Depot projects* —
which is why the team prefers `skirmish` (reuse a canonical depot) over `gauntlet`
(levy fresh projects) to avoid project-quota exhaustion.
Workforce identity pools — what affiance creates — are a *different* resource, so
their constraints were verified directly before committing a fixture that
creates/destroys them.

## Findings (with confidence)

| Fact | Value | Confidence |
|---|---|---|
| Workforce pools per organization | **100** | Documented (GCP IAM quotas) |
| Delete model | Soft-delete; 30-day recovery via `undelete`; then permanent purge | Documented |
| Same-id re-create while soft-deleted | **Collides — cannot re-create; must `undelete`** | Documented for workload pools; empirically consistent for workforce (our jilt proof: GET on a soft-deleted pool returns `200` / `state: DELETED` — the id-namespace is held); asserted by RBSMA's own soft-delete NOTE (409 on same-id create) |
| Soft-deleted pools count against the cap | **Yes (treat as yes)** | Documented for workload pools ("deleted pools continue to count against your quota for that 30-day period"); not explicitly documented for workforce, but the soft-delete model is shared — conservative assumption |

## Implication for fixture design

A fixture that mints a **fresh** pool id every run and soft-deletes it accumulates
toward the 100-cap for 30 days per run — the same class of exhaustion the team
already hit with Depots. **Fresh-per-run create is quota-threatening.**

Two consequences fall out:

1. **A fixture that genuinely tests _create_ cannot reuse an id** (the soft-deleted
   id collides on re-create), so it must mint fresh → it is inherently
   quota-touching → it belongs operator-invoked / skirmish-class, run rarely
   (the exact posture gauntlet's fresh-project churn already takes).
2. **A day-to-day fixture should reuse one _durable_ pool**, restored via `undelete`
   rather than re-created: ensure-or-undelete the single id → exercise → leave it.
   At most one soft-deleted pool ever; never accumulates → quota-flat.

This is the gauntlet(rare, churny) / skirmish(frequent, reuse-canonical) split,
applied to foedus.

## Latent affiance bug surfaced by the proof

Because GET on a soft-deleted pool returns `200` with `state: DELETED` (proven by the
jilt verify loop), affiance's current `200 → "already present", skip create` logic
would treat a *dead* soft-deleted pool as alive: it would skip create and report
success on an unusable (deleted) trust.
To support the reuse-one-durable-pool strategy — and to be correct in general —
affiance must, on `200 + state DELETED`, **undelete** rather than skip.
RBSMA's soft-delete NOTE already anticipates this; the impl does not yet honor it.

## Recommendation

- The full create→destroy foedus fixture is inherently quota-touching: run it
  operator-invoked / skirmish-class, **not** in a frequently-run auto-suite.
- The day-to-day foedus fixture should reuse one **durable** pool via `undelete`
  (quota-flat), which requires the affiance undelete-on-`DELETED` fix above.
- Capture the affiance undelete gap as an RBSHR horizon item if it is not fixed
  in-heat.

## Sources

- [GCP IAM Quotas](https://docs.cloud.google.com/iam/quotas) — workforce pools, 100 per organization
- [locations.workforcePools.delete](https://docs.cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools/delete) / [undelete](https://docs.cloud.google.com/iam/docs/reference/rest/v1/locations.workforcePools/undelete) — soft-delete, 30-day recovery
- [Manage workforce identity pools and providers](https://docs.cloud.google.com/iam/docs/manage-workforce-identity-pools-providers) — undelete within 30 days
- [Manage workload identity pools and providers](https://cloud.google.com/iam/docs/manage-workload-identity-pools-providers) — soft-delete analogue: deleted pools count against quota, id not reusable until purged
- hashicorp/terraform-provider-google issues [#12941](https://github.com/hashicorp/terraform-provider-google/issues/12941), [#14805](https://github.com/hashicorp/terraform-provider-google/issues/14805) — soft-delete state/collision behavior
