# Heat: Governor Reset Operation

## Context

The `rbgp_create_governor` operation is specified but not implemented. During analysis, we discovered:

1. **Role confusion in documentation** - Some docs incorrectly suggested Governor manages depot lifecycle (Payor does)
2. **Pristine-state assumption** - Current spec fails if Governor exists, but reset semantics are needed for key rotation and recovery
3. **Missing depot argument** - Spec uses config variable, but Payor can own multiple depots
4. **Missing authentication step** - Spec lacks explicit `{rbtoe_payor_authenticate}` step

This heat specifies and implements `rbgp_governor_reset` - a Payor operation that creates or replaces a Governor for a specified depot.

## Terminology

- **depot_name**: Short user-provided name (e.g., `dev`, max 20 chars)
- **depot_project_id**: Full GCP project ID (e.g., `rbw-dev-202512271430`)
- The `depot_project_id` is the argument to `rbgp_governor_reset`

## Design Decisions

- Governor SA: `governor-{timestamp}` (e.g., `governor-202512271430`) - auto-assigned
- Mason SA: `mason-{depot_name}` (e.g., `mason-dev`) - derived from depot name
- Reset operation: finds all `governor-*` SAs in target depot, deletes them, creates fresh one

## Done

- **Standardize SA naming patterns** — Established naming conventions:
   - Governor: `governor-{timestamp}` (auto-assigned at creation)
   - Mason: `mason-{depot_name}` (was `rbw-{depot_name}-mason`)
   - Director/Retriever: unchanged
   Updated specs (RBSDC, RBSDL), constants, and implementation.

- **Revise RBSGS for complete Payor operation coverage** — Updated Getting Started guide:
   - Clarified Payor creates depots AND governors (both via OAuth)
   - Added depot_list and depot_destroy documentation
   - Added governor_reset to Recovery section
   - Added rbtgo_governor_reset mapping to RBAGS

- **Revise governor_create spec to governor_reset semantics** — Updated spec file:
   - Renamed: `rbw-RBSGC-governor_create.adoc` → `rbw-RBSGR-governor_reset.adoc`
   - Idempotent create-or-replace semantics with `«INPUT_DEPOT_PROJECT_ID»` argument
   - Added authentication step and governor-* cleanup before creation
   - Updated all RBAGS mappings and references

## Current

- **Implement rbgp_governor_reset in Payor module** — Add function to `Tools/rbw/rbgp_Payor.sh`:
   - Argument: `depot_project_id` (obtain via `rbgp_depot_list`)
   - Pattern: `rbgp_depot_create` for Payor OAuth operations
   - Pattern: `zrbgg_create_service_account_with_key` for SA+key creation
   - Governor gets `roles/owner` on depot project (unlike Director/Retriever)
   - Reference: BCG for bash style
   - Test with real depot
   mode: manual

## Remaining

- **Update itch rbgp-create-governor** — Mark as closed/superseded by this implementation, or delete.
   mode: manual
