# Regime Voicing Audit — 2026-02-16

Cross-regime audit of all variable voicings across RBSA and BUSA.
Quality gate before wiring tabtargets.

## Scope

Ten regimes audited: BURC, BURS, BURD, RBRR, RBRN, RBRV, RBRP, RBRS, RBRA, RBRO.
Excluded: RBRE and RBRG (being deleted elsewhere).

## 1. Variable Count Summary

| Regime | Validator | Spec (RBSA/BUSA) | Subdocument | Status |
|--------|-----------|-------------------|-------------|--------|
| BURC   | 10        | 10                | 10          | MATCH  |
| BURS   | 1         | 1                 | 1           | MATCH  |
| BURD   | 25        | 25                | 25 (tokens grouped) | MATCH |
| RBRR   | 26        | 27                | 27          | DISCREPANCY (see below) |
| RBRN   | 20        | 20                | 20          | MATCH  |
| RBRV   | 7         | 8                 | 8           | DISCREPANCY (see below) |
| RBRP   | 3         | 4                 | 3           | DISCREPANCY (see below) |
| RBRS   | 3         | 3                 | 3           | MATCH  |
| RBRA   | 4         | 4                 | 4           | MATCH  |
| RBRO   | 2         | 2                 | 2           | MATCH  |

### RBRR discrepancy: RBRR_PAYOR_RBRA_FILE

`RBRR_PAYOR_RBRA_FILE` is defined in RBSA (mapping + anchor) and in RBSRR subdocument but
does NOT exist in `rbrr_regime.sh`. The payor role uses OAuth (RBRO), not a service account
(RBRA). This appears to be a spec-only artifact — the payor has never used an RBRA file.

**Recommendation**: Remove `rbrr_payor_rbra_file` from RBSA mapping, RBSA anchors, and RBSRR
subdocument. This would align all three sources at 26. Not removed during this audit because
the decision affects architectural intent — flagged for owner review.

### RBRV discrepancy: RBRV_VESSEL_MODE

`RBRV_VESSEL_MODE` is defined in RBSA (mapping + anchor + 2 enum values) and in RBSRV
subdocument but does NOT exist in `rbrv_regime.sh`. The validator infers vessel mode
implicitly from presence of `RBRV_BIND_IMAGE` vs `RBRV_CONJURE_DOCKERFILE`. The spec
describes an explicit enum variable that the implementation doesn't have.

**Recommendation**: Either add `RBRV_VESSEL_MODE` to the validator or remove it from the spec.
Not resolved during this audit — flagged for owner review.

### RBRP discrepancy: RBRP_DEPOT_PROJECT_IDS

`RBRP_DEPOT_PROJECT_IDS` is defined in RBSA (mapping + anchor) but does NOT exist in
`rbrp_regime.sh` validator. The depot project list is managed outside the regime validator.

**Recommendation**: Either add to the validator or remove from RBSA. Not resolved during this
audit — flagged for owner review.

## 2. Dimension Consistency

### Fixes applied

| Regime | Variable | Issue | Fix |
|--------|----------|-------|-----|
| RBRR | RBRR_DNS_SERVER | Spec said `axd_optional`, validator requires it | Changed to `axd_required` |
| RBRN | RBRN_ENTRY_PORT_WORKSTATION | Spec said `axd_required`, validator checks conditionally | Changed to `axd_conditional` |
| RBRN | RBRN_ENTRY_PORT_ENCLAVE | Spec said `axd_required`, validator checks conditionally | Changed to `axd_conditional` |
| RBRN | RBRN_UPLINK_ALLOWED_CIDRS | Spec said `axd_required`, validator checks conditionally | Changed to `axd_conditional` |
| RBRN | RBRN_UPLINK_ALLOWED_DOMAINS | Spec said `axd_required`, validator checks conditionally | Changed to `axd_conditional` |
| RBRV | RBRV_BIND_IMAGE | Spec said `axd_required`, in conditional group | Changed to `axd_conditional` |
| RBRV | RBRV_CONJURE_DOCKERFILE | Spec said `axd_required`, in conditional group | Changed to `axd_conditional` |
| RBRV | RBRV_CONJURE_BLDCONTEXT | Spec said `axd_required`, in conditional group | Changed to `axd_conditional` |
| RBRV | RBRV_CONJURE_PLATFORMS | Spec said `axd_optional`, in conditional group | Changed to `axd_conditional` |
| RBRV | RBRV_CONJURE_BINFMT_POLICY | Spec said `axd_optional`, in conditional group | Changed to `axd_conditional` |
| RBRR | RBRR_DNS_SERVER | Type was `rbst_ip_address` (nonstandard) | Changed to `rbst_ipv4` |

### RBRR_GCB_MIN_CONCURRENT_BUILDS — no buv_env_* call

This variable is in kindle (line 48) and in z_known (line 62) but has no `buv_env_*`
validation call in `zrbrr_validate_fields()`. All other kindled variables are validated.

**Recommendation**: Add `buv_env_decimal RBRR_GCB_MIN_CONCURRENT_BUILDS 1 32` or similar.
Not fixed during this audit — flagged for owner review.

## 3. Operation Completeness

| Regime | Cardinality | kindle | validate | render | list | survey | audit | Status |
|--------|-------------|--------|----------|--------|------|--------|-------|--------|
| BURC   | singleton   | yes    | yes      | yes    | -    | -      | -     | OK     |
| BURS   | singleton   | yes    | yes      | yes    | -    | -      | -     | OK     |
| BURD   | singleton   | yes    | yes      | -      | -    | -      | -     | OK (ephemeral) |
| RBRR   | singleton   | yes    | yes      | yes    | -    | -      | -     | OK     |
| RBRN   | manifold    | yes    | yes      | yes    | yes  | yes    | yes   | OK     |
| RBRV   | manifold    | yes    | yes      | yes    | yes  | -      | -     | OK     |
| RBRP   | singleton   | yes    | inline   | yes    | -    | -      | -     | OK     |
| RBRS   | singleton   | yes    | inline   | yes    | -    | -      | -     | OK     |
| RBRA   | manifold    | yes    | yes      | yes    | yes  | -      | -     | OK     |
| RBRO   | singleton   | yes    | yes      | yes    | -    | -      | -     | OK     |

All operation voicings use `axhro_kindle` (not `axhro_broach`). Zero `axhro_broach` references
found in any .adoc file.

## 4. Cardinality Annotation Check

All regime definitions in RBSA include `axrd_singleton` or `axrd_manifold`:

| Regime | RBSA annotation | Correct? |
|--------|----------------|----------|
| RBRR   | `axrd_singleton` | Yes |
| RBRA   | `axrd_manifold`  | Yes |
| RBRP   | `axrd_singleton` | Yes |
| RBRO   | `axrd_singleton` | Yes |
| RBRS   | `axrd_singleton` | Yes |
| RBRV   | `axrd_manifold`  | Yes |
| RBRN   | `axrd_manifold`  | Yes |

BUSA regimes:

| Regime | BUSA annotation | Correct? |
|--------|----------------|----------|
| BURC   | `axrd_singleton` | Yes |
| BURS   | `axrd_singleton` | Yes |
| BURD   | `axrd_singleton` | Yes |

All cardinality annotations present and correct.

## 5. Cross-Document Consistency

### Missing attribute references — FIXED

Six RBRR GCB image ref variables existed in `rbrr_regime.sh` validator but had no attribute
references, anchors, or subdocument entries:

- `RBRR_GCB_GCLOUD_IMAGE_REF`
- `RBRR_GCB_DOCKER_IMAGE_REF`
- `RBRR_GCB_SKOPEO_IMAGE_REF`
- `RBRR_GCB_ALPINE_IMAGE_REF`
- `RBRR_GCB_SYFT_IMAGE_REF`
- `RBRR_GCB_BINFMT_IMAGE_REF`

All six have been added to:
- RBSA mapping section (attribute references)
- RBSA RBRR regime section (anchors + definitions)
- RBSRR subdocument (axhrgv_variable markers)

### Orphaned attribute references — FLAGGED

Three attribute references exist in RBSA mapping section with anchors but no corresponding
validator variable:

1. `rbrr_payor_rbra_file` — no RBRR_PAYOR_RBRA_FILE in validator
2. `rbrp_depot_project_ids` — no RBRP_DEPOT_PROJECT_IDS in validator
3. `rbrv_vessel_mode` — no RBRV_VESSEL_MODE in validator

These are design-level decisions, not mechanical fixes. Flagged for owner review.

## 6. RBEV Placeholder Removed

The RBEV section (Environment Variables Regime) was a TBD placeholder at RBSA line ~2503 with:
- No variables defined
- No validator
- No CLI
- A NOTE stating "formal definition within this specification is TBD"

Removed:
- `rbev_regime` attribute reference from mapping section
- `rbev_prefix` attribute reference from mapping section
- `[[rbev_prefix]]` anchor and definition
- Full `=== Environment Variables Regime (RBEV)` section (anchor, definition, NOTE)

Verified: No other documents reference `rbev_regime`, `rbev_prefix`, or `RBEV_` variables.

**Rationale**: Per-vessel build parameters are Dockerfile-local, not a regime. RBEV was a
placeholder concept that was never implemented and does not fit the regime pattern.

## All Fixes Applied

1. Added 6 missing GCB image ref attribute references to RBSA mapping section
2. Added 6 missing GCB image ref anchors/definitions to RBSA RBRR regime section
3. Added 6 missing GCB image ref variable entries to RBSRR subdocument
4. Fixed RBRR_DNS_SERVER dimension: `axd_optional` -> `axd_required`, type `rbst_ip_address` -> `rbst_ipv4`
5. Fixed RBRN_ENTRY_PORT_WORKSTATION dimension: `axd_required` -> `axd_conditional`
6. Fixed RBRN_ENTRY_PORT_ENCLAVE dimension: `axd_required` -> `axd_conditional`
7. Fixed RBRN_UPLINK_ALLOWED_CIDRS dimension: `axd_required` -> `axd_conditional`
8. Fixed RBRN_UPLINK_ALLOWED_DOMAINS dimension: `axd_required` -> `axd_conditional`
9. Fixed RBRV_BIND_IMAGE dimension: `axd_required` -> `axd_conditional`
10. Fixed RBRV_CONJURE_DOCKERFILE dimension: `axd_required` -> `axd_conditional`
11. Fixed RBRV_CONJURE_BLDCONTEXT dimension: `axd_required` -> `axd_conditional`
12. Fixed RBRV_CONJURE_PLATFORMS dimension: `axd_optional` -> `axd_conditional`
13. Fixed RBRV_CONJURE_BINFMT_POLICY dimension: `axd_optional` -> `axd_conditional`
14. Removed RBEV placeholder section from RBSA (mapping refs, prefix anchor, regime section)

## Open Items for Owner Review

1. **RBRR_PAYOR_RBRA_FILE**: Spec-only variable. Remove from spec/subdoc or add to validator?
2. **RBRP_DEPOT_PROJECT_IDS**: Spec-only variable. Remove from spec or add to validator?
3. **RBRV_VESSEL_MODE**: Spec-only variable. Remove from spec/subdoc or add to validator?
4. **RBRR_GCB_MIN_CONCURRENT_BUILDS**: Missing buv_env_* validation call in validator.
