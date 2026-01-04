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
