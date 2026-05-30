# Scars

Closed ideas with lessons learned.

## rbgp-create-governor (2025-12-27)
**Status**: Superseded by heat `b251227-governor-reset-operation`

Original itch called for `rbgp_create_governor()` following RBAGS spec. During heat planning, we discovered:
- The spec assumed pristine state (no existing Governor)
- Real-world needs reset semantics for credential rotation
- Naming convention needed standardization (governor-{timestamp})

**Outcome**: Implemented `rbgp_governor_reset()` instead - idempotent create-or-replace operation that deletes existing governor-* SAs before creating a new one.

**Lesson**: Implementation itches should note whether pristine-state or idempotent semantics are intended.

## depot-projectid-collision-global-probe (2026-05-30)
**Status**: Investigated and closed without code change (heat `rbk-16-gcp-hardening` archived).

Cross-identity depot projectId reservation collisions: GCP project IDs are globally unique and stay reserved ~30 days after deletion (DELETE_REQUESTED). The depot-moniker autoincrement picks "next free" off an *identity-scoped* view (`projects:search` / local fact files), so it is blind to IDs reserved by a *different* identity's deleted project. This bit the project on a payor switch (gmail → scaleinvariant): gmail's deleted `cancbhm-d-canest2bhm100000` held the global ID, invisible to scaleinvariant's allocator, which kept re-deriving the family floor and 409'ing.

**The tempting fix is impossible.** "Move the autoincrement onto what's globally visible" cannot work — there is no global read oracle. Verified live (CRM v3 *and* v1 `GET /projects/{id}`): a free ID and a stranger-owned ID return **byte-identical `403 PERMISSION_DENIED`**. GCP does this deliberately (anti-enumeration — it won't confirm existence to a caller who can't access the resource; the error text is literally "the caller does not have permission, or the resource may not exist"). Confirmed by Google docs. So reads cannot distinguish free from taken-elsewhere.

**Outcome**: No change. The live case was already mitigated by a family-stem bump (`canest2` → `canest3`, see `RBTDRK_FAMILY_STEM_BASE` in `rbtdrk_canonical.rs`), which moved into a clean namespace. Urgency is low — recurs only on payor-identity switch.

**Lesson**: The *only* oracle for global projectId availability is the **write** — attempt create, catch `409 alreadyExists`. If this ever recurs and is worth fixing, the sole robust path is create-and-retry-on-409 (increment the moniker on 409 until create succeeds; failed creates reserve nothing), at the cost of coupling moniker-pick to project-create (today they are separate: pick in `rbtdrk_canonical.rs`, create in the `rbgp_depot_levy` bash). Do **not** re-attempt a read-side availability probe — it is provably impossible.
