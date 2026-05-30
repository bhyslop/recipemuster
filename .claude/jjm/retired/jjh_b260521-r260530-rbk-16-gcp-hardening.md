# Heat Trophy: rbk-16-gcp-hardening

**Firemark:** ₣BR
**Created:** 260521
**Retired:** 260530
**Status:** retired

## Paddock

## Shape

This heat hardens RBK against GCP behaviors that surface only under real-world churn. Current content: one problem — **depot projectId reservation collisions**.

GCP project IDs are globally unique and stay reserved ~30 days after a project is deleted (DELETE_REQUESTED). The depot-moniker allocator picks "next free" by walking a same-identity displayName search (`projects:search`, via depot_list). That search is identity-scoped: it returns the caller's own projects including their DELETE_REQUESTED ones — so **same-identity churn self-heals**, the allocator increments past its own pending-delete depots.

The blindness is **cross-identity**: a projectId reserved by a *different* identity's deleted project is invisible to the caller's search. The allocator then derives a moniker whose global ID is already held elsewhere, and project creation fails with 409. This bit the project when the payor switched from the gmail (no-org) identity to the scaleinvariant identity: gmail's deleted `canest2bhm100000` held the global ID, invisible to the scaleinvariant allocator, which kept re-deriving the family floor.

## Scope and urgency

Low urgency. Same-identity churn already self-heals; the residual collision recurs **only on a payor-identity switch** — rare. The session that found it worked around it with a family-stem bump. This is robustness, not a blocker.

## Undecided (design, not yet settled)

- **Where the fix lives.** The collider was the *test fixture* moniker picker (`rbtdrk_canonical.rs`, Rust); the production levy (`rbgp_depot_levy`) does not auto-pick — it uses the configured `RBRD_DEPOT_MONIKER` and 409s on collision. So "fixture-only Rust fix" vs "production levy change (`rbgp_Payor.sh` + `RBSDE`/`RBSDN`)" is an open choice — different surfaces, different blast radius.
- **What the fix does.** A candidate-ID availability probe (GET → 404 free / 403 owned-elsewhere / 200 own; increment past taken) tests the real global constraint instead of the same-identity proxy — versus a lighter "clearer 409 + operator guidance" approach.

## Constraints

- Any bash work here reads **BCG** (Bash Console Guide) first. `rbgp_Payor.sh` is a complex BCG-compliant module; do not write bash against it without the guide.

## What done looks like

A decision on fix surface (fixture vs production) and fix shape (probe vs guidance), then the implementation plus matching spec update. Until then, paces stay uncut.

## Paces

## Commit Activity

```
File-touch bitmap: (no work file changes)
```

## Steeplechase

### 2026-05-28 09:19 - Heat - n

Commit the operator-credential-models memo as a committed two-tier plan (keyfile free/no-org + Workforce federation paid/org-required), renamed from the federation-feasibility memo. Mode enum homed in RBRD (tamper-evident, depot-wide), RBRI eliminated, RBRA keyfile-only; single role-keyed token-accessor seam with refactor-first sequencing. Repoint RBSHR roadmap and the reorientation memo to the new filename and both-models framing.

### 2026-05-21 16:20 - Heat - d

paddock curried: capture allocator projectId-reservation problem; cross-identity scope; BCG-before-bash constraint

### 2026-05-21 11:16 - Heat - f

stabled

### 2026-05-21 11:16 - Heat - N

rbk-16-gcp-hardening

