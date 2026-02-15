# Recipe Bottle Configuration Regime Inventory

**Pace:** ATAAA (study-all-recipe-bottle-regimes)
**Heat:** AT (rbw-regime-consolidation)
**Date:** 2026-02-09
**Inputs:** CRR, all `*_regime.sh` validators, assignment files, `bud_dispatch.sh`, `rbgo_OAuth.sh`, `rbgp_Payor.sh`, `rbgg_Governor.sh`

---

## Regime Inventory Table

| # | Prefix | Full Name | Domain | Scope | Lifecycle | Validator | Assignment(s) | Spec Doc |
|---|--------|-----------|--------|-------|-----------|-----------|---------------|----------|
| 1 | BURC_ | Regime Configuration | BUK | Project | Git-tracked, sourced at dispatch | `buk/burc_regime.sh` | `.buk/burc.env` | -- |
| 2 | BURS_ | Regime Station | BUK | Developer/machine | External (NOT in git) | `buk/burs_regime.sh` | Path from `BURC_STATION_FILE` | -- |
| 3 | BURD_ | Dispatch Runtime | BUK | Ephemeral | Set during `bud_dispatch.sh`, dies with process | -- (runtime only) | -- (computed) | -- |
| 4 | RBRR_ | Regime Repo | RBW | Repository | Git-tracked, sourced at workbench kindle | `rbw/rbrr_regime.sh` | `rbrr.env` | `RBRR-RegimeRepo.adoc` |
| 5 | RBRN_ | Regime Nameplate | RBW | Per-service | Git-tracked, one file per nameplate | `rbw/rbrn_regime.sh` | `rbw/rbrn_nsproto.env`, `rbrn_srjcl.env`, `rbrn_pluml.env` | `RBRN-RegimeNameplate.adoc` |
| 6 | RBRP_ | Regime Payor | RBW | GCP payor project | Git-tracked | `rbw/rbrp_regime.sh` | `rbrp.env` | -- |
| 7 | RBRE_ | Regime ECR | RBW | AWS ECR credentials | External (NOT in git) | `rbw/rbre_regime.sh` | External | -- |
| 8 | RBRG_ | Regime GitHub | RBW | GitHub credentials | External (NOT in git) | `rbw/rbrg_regime.sh` | External | -- |
| 9 | RBRS_ | Regime Station | RBW | Developer workstation | External (NOT in git) | `rbw/rbrs_regime.sh` | External | -- |
| 10 | RBRV_ | Regime Vessel | RBW | Per-vessel build/bind | Git-tracked, one per vessel dir | `rbw/rbrv_regime.sh` | `rbev-vessels/*/rbrv.env` (6 vessels) | -- |
| 11 | RBRA_ | Credential Format | RBW | Per-service-account | External (NOT in git) | -- (no validator) | Written by `rbgp_Payor.sh` / `rbgg_Governor.sh` | -- |

---

## Per-Regime Variable Listings

### 1. BURC_ (Regime Configuration)

Sourced from `.buk/burc.env`. All validated by `burc_regime.sh:zburc_kindle()`.

| Variable | Type | Notes |
|----------|------|-------|
| BURC_STATION_FILE | string | Path to BURS station file (cross-regime link) |
| BURC_TABTARGET_DIR | string | Directory for tabtarget scripts (`tt`) |
| BURC_TABTARGET_DELIMITER | string | Exactly 1 char (`.`) |
| BURC_TOOLS_DIR | string | Tools directory path |
| BURC_PROJECT_ROOT | string | Relative project root |
| BURC_MANAGED_KITS | string | Comma-separated kit list |
| BURC_TEMP_ROOT_DIR | string | Temp directory root |
| BURC_OUTPUT_ROOT_DIR | string | Output directory root |
| BURC_LOG_LAST | string | Log filename stem |
| BURC_LOG_EXT | string | Log file extension |

**Note:** `BURC_PROJECT_ROOT` and `BURC_MANAGED_KITS` are present in the assignment file but not validated by the kindle function. They are consumed downstream.

### 2. BURS_ (Regime Station)

Sourced from path in `BURC_STATION_FILE`. Validated by `burs_regime.sh:zburs_kindle()`.

| Variable | Type | Notes |
|----------|------|-------|
| BURS_LOG_DIR | string | Directory for log files |

### 3. BURD_ (Dispatch Runtime)

Set by `bud_dispatch.sh` during execution. **Not a declared regime** -- no validator, no assignment file, no spec doc. Runtime-only.

| Variable | Lifecycle | Notes |
|----------|-----------|-------|
| BURD_VERBOSE | Input | Set before dispatch (0/1/2) |
| BURD_REGIME_FILE | Input | Path to BURC env file |
| BURD_NOW_STAMP | Computed | Timestamp+PID+random |
| BURD_TEMP_DIR | Computed | Ephemeral temp directory |
| BURD_OUTPUT_DIR | Computed | Output directory for current run |
| BURD_TRANSCRIPT | Computed | Path to transcript file |
| BURD_GIT_CONTEXT | Computed | `git describe` output |
| BURD_COMMAND | Parsed | Primary command token |
| BURD_TARGET | Parsed | Target argument |
| BURD_CLI_ARGS | Parsed | Remaining CLI arguments (array) |
| BURD_TOKEN_1..5 | Parsed | Positional tokens from tabtarget |
| BURD_LOG_LAST | Computed | Path to "last" log |
| BURD_LOG_SAME | Computed | Path to "same" log |
| BURD_LOG_HIST | Computed | Path to "hist" log |
| BURD_COLOR | Computed | Color policy (0/1) |
| BURD_NO_LOG | Input | Suppress logging |
| BURD_INTERACTIVE | Input | Interactive mode flag |

### 4. RBRR_ (Regime Repo)

Sourced from `rbrr.env` at repo root. Validated by `rbrr_regime.sh:zrbrr_kindle()`.

| Variable | Validator Type | Notes |
|----------|---------------|-------|
| RBRR_REGISTRY_OWNER | xname[2,64] | Container registry owner |
| RBRR_REGISTRY_NAME | xname[2,64] | Container registry name |
| RBRR_HISTORY_DIR | string[1,255] | History directory (must exist) |
| RBRR_NAMEPLATE_PATH | string[1,255] | Nameplate path (must exist) |
| RBRR_VESSEL_DIR | string[1,255] | Vessel directory |
| RBRR_DNS_SERVER | ipv4 | DNS server for containers |
| RBRR_IGNITE_MACHINE_NAME | xname[1,64] | Podman machine for ignition |
| RBRR_DEPLOY_MACHINE_NAME | xname[1,64] | Podman machine for deployment |
| RBRR_CRANE_TAR_GZ | string[1,512] | Path to crane tarball |
| RBRR_MANIFEST_PLATFORMS | string[1,512] | Space-separated platforms |
| RBRR_CHOSEN_PODMAN_VERSION | string[1,16] | Semantic version (X.Y or X.Y.Z) |
| RBRR_CHOSEN_VMIMAGE_ORIGIN | fqin[1,256] | VM image origin FQIN |
| RBRR_CHOSEN_IDENTITY | string[1,128] | Identity reference |
| RBRR_DEPOT_PROJECT_ID | gname[6,63] | GCP depot project ID |
| RBRR_GCP_REGION | gname[1,32] | GCP region |
| RBRR_GAR_REPOSITORY | gname[1,63] | Artifact Registry repository |
| RBRR_GCB_MACHINE_TYPE | string[3,64] | Cloud Build machine type |
| RBRR_GCB_TIMEOUT | string[2,10] | Cloud Build timeout (Ns format) |
| RBRR_GCB_MIN_CONCURRENT_BUILDS | integer[1,3] | Min concurrent builds required |
| RBRR_GOVERNOR_RBRA_FILE | string[1,512] | Path to governor RBRA cred file |
| RBRR_RETRIEVER_RBRA_FILE | string[1,512] | Path to retriever RBRA cred file |
| RBRR_DIRECTOR_RBRA_FILE | string[1,512] | Path to director RBRA cred file |
| RBRR_GCB_GCRANE_IMAGE_REF | odref | Digest-pinned gcrane image |
| RBRR_GCB_ORAS_IMAGE_REF | odref | Digest-pinned oras image |

**Total: 23 variables.**

### 5. RBRN_ (Regime Nameplate)

Sourced from per-nameplate `.env` files. Validated by `rbrn_regime.sh:zrbrn_kindle()`.

| Variable | Validator Type | Conditionality |
|----------|---------------|----------------|
| RBRN_MONIKER | xname[2,12] | Required |
| RBRN_DESCRIPTION | string[0,120] | Optional |
| RBRN_RUNTIME | string[1,16] | Required; enum: `docker`/`podman` |
| RBRN_SENTRY_VESSEL | fqin[1,128] | Required |
| RBRN_BOTTLE_VESSEL | fqin[1,128] | Required |
| RBRN_SENTRY_CONSECRATION | fqin[1,128] | Required |
| RBRN_BOTTLE_CONSECRATION | fqin[1,128] | Required |
| RBRN_ENTRY_MODE | enum | Required; `disabled`/`enabled` |
| RBRN_ENTRY_PORT_WORKSTATION | port | When ENTRY_MODE=enabled |
| RBRN_ENTRY_PORT_ENCLAVE | port | When ENTRY_MODE=enabled |
| RBRN_ENCLAVE_BASE_IP | ipv4 | Required |
| RBRN_ENCLAVE_NETMASK | decimal[8,30] | Required |
| RBRN_ENCLAVE_SENTRY_IP | ipv4 | Required |
| RBRN_ENCLAVE_BOTTLE_IP | ipv4 | Required |
| RBRN_UPLINK_PORT_MIN | port | Required |
| RBRN_UPLINK_DNS_MODE | enum | Required; `disabled`/`global`/`allowlist` |
| RBRN_UPLINK_ACCESS_MODE | enum | Required; `disabled`/`global`/`allowlist` |
| RBRN_UPLINK_ALLOWED_CIDRS | list_cidr | When ACCESS_MODE=allowlist |
| RBRN_UPLINK_ALLOWED_DOMAINS | list_domain | When DNS_MODE=allowlist |
| RBRN_VOLUME_MOUNTS | string[0,240] | Optional |

**Total: 20 variables.** 3 nameplate instances: `nsproto`, `srjcl`, `pluml`.

**Note on evolution:** The validator now uses tri-state `_MODE` enums (`disabled`/`global`/`allowlist`) instead of the boolean `_ENABLED`/`_GLOBAL` pattern documented in the original RBRN spec. This is a more recent refactoring that collapsed 5 boolean flags into 2 mode enums plus the existing ENTRY_MODE.

### 6. RBRP_ (Regime Payor)

Sourced from `rbrp.env` at repo root. Validated by `rbrp_regime.sh:zrbrp_kindle()`.

| Variable | Validation | Notes |
|----------|-----------|-------|
| RBRP_PAYOR_PROJECT_ID | Regex (RBGC global pattern) | Must match `rb-payor-YYMMDDHHMMSS` |
| RBRP_BILLING_ACCOUNT_ID | Regex `XXXXXX-XXXXXX-XXXXXX` | Optional during initial setup |
| RBRP_OAUTH_CLIENT_ID | Regex `*.apps.googleusercontent.com` | Optional during initial setup |

**Dependencies:** Requires RBGC (Constants) to be kindled first (`zrbgc_sentinel` call).

**Total: 3 variables.**

### 7. RBRE_ (Regime ECR)

AWS ECR credentials. Validated by `rbre_regime.sh:zrbre_kindle()`.

| Variable | Validator Type | Notes |
|----------|---------------|-------|
| RBRE_AWS_CREDENTIALS_ENV | string[1,255] | AWS credentials environment label |
| RBRE_AWS_ACCESS_KEY_ID | string[20,20] | Fixed 20-char key |
| RBRE_AWS_SECRET_ACCESS_KEY | string[40,40] | Fixed 40-char secret |
| RBRE_AWS_ACCOUNT_ID | string[12,12] | Fixed 12-char account ID |
| RBRE_AWS_REGION | string[1,32] | AWS region |
| RBRE_REPOSITORY_NAME | xname[2,64] | ECR repository name |

**Total: 6 variables.**

### 8. RBRG_ (Regime GitHub)

GitHub credentials. Validated by `rbrg_regime.sh:zrbrg_kindle()`.

| Variable | Validator Type | Notes |
|----------|---------------|-------|
| RBRG_PAT | string[40,100] | Personal access token |
| RBRG_USERNAME | xname[1,39] | GitHub username |

**Total: 2 variables.**

### 9. RBRS_ (Regime Station)

Developer workstation config. Validated by `rbrs_regime.sh:zrbrs_kindle()`.

| Variable | Validator Type | Notes |
|----------|---------------|-------|
| RBRS_PODMAN_ROOT_DIR | string[1,64] | Podman root directory |
| RBRS_VMIMAGE_CACHE_DIR | string[1,64] | VM image cache directory |
| RBRS_VM_PLATFORM | string[1,64] | VM platform identifier |

**Total: 3 variables.**

### 10. RBRV_ (Regime Vessel)

Per-vessel build/bind config. Validated by `rbrv_regime.sh:zrbrv_kindle()`.

| Variable | Validator Type | Conditionality |
|----------|---------------|----------------|
| RBRV_SIGIL | xname[1,64] | Required (must match directory name) |
| RBRV_DESCRIPTION | string[0,512] | Optional |
| RBRV_BIND_IMAGE | fqin[1,512] | When binding (copying from registry) |
| RBRV_CONJURE_DOCKERFILE | string[1,512] | When conjuring (building from source) |
| RBRV_CONJURE_BLDCONTEXT | string[1,512] | When conjuring |
| RBRV_CONJURE_PLATFORMS | string[1,512] | When conjuring |
| RBRV_CONJURE_BINFMT_POLICY | string[1,16] | When conjuring; enum: `allow`/`forbid` |

**Invariant:** At least one of BIND_IMAGE or CONJURE_DOCKERFILE must be set.

**Total: 7 variables.** 6 vessel instances in `rbev-vessels/`.

### 11. RBRA_ (Credential Format)

Service account credential files sourced at runtime. **Not a regime** in the CRR sense -- no validator, no spec doc. A file format convention.

| Variable | Notes |
|----------|-------|
| RBRA_CLIENT_EMAIL | Service account email |
| RBRA_PRIVATE_KEY | RSA private key (PEM) |
| RBRA_PROJECT_ID | GCP project ID |
| RBRA_TOKEN_LIFETIME_SEC | Token lifetime (hardcoded 1800) |

**Written by:** `rbgp_Payor.sh` (governor SA), `rbgg_Governor.sh` (retriever/director SAs)
**Consumed by:** `rbgo_OAuth.sh` (token generation)
**Referenced via:** `RBRR_GOVERNOR_RBRA_FILE`, `RBRR_RETRIEVER_RBRA_FILE`, `RBRR_DIRECTOR_RBRA_FILE`

---

## Cross-Regime Dependency Map

```
BURC ─── sources ──► BURS  (via BURC_STATION_FILE path)
  │
  └─── feeds ──► BURD  (dispatch reads BURC, creates BURD_ runtime vars)

RBRR ─── references ──► RBRA  (3 file paths: GOVERNOR, RETRIEVER, DIRECTOR)
  │
  └─── depends on ──► RBGC  (Constants, for naming patterns)

RBRP ─── depends on ──► RBGC  (zrbgc_sentinel required before kindle)

RBRN ─── independent  (no cross-regime validator dependencies)
RBRE ─── independent
RBRG ─── independent
RBRS ─── independent
RBRV ─── independent

RBRA ─── written by ──► RBGP (Payor), RBGG (Governor)
     ─── consumed by ──► RBGO (OAuth)
     ─── paths in ──► RBRR (3 path variables)
```

### Kindle Order (enforced by sentinel calls)

1. **RBGC** (Constants) -- must be first, depended on by RBRP
2. **BURC** -- infrastructure, enables BURS
3. **BURS** -- station, enables BUD logging
4. **RBRR** -- repo config, enables RBRA file access
5. **RBRP** -- payor config (needs RBGC)
6. All others are independent and can kindle in any order

---

## Naming Convention Analysis (per CRR Framework)

### CRR Compliance Assessment

| Regime | Has Prefix | Has Validator | Has Assignment | Has Spec Doc | Has Glossary | CRR Compliant |
|--------|-----------|---------------|----------------|-------------|-------------|---------------|
| BURC_ | Yes | Yes | Yes | No | No | Partial |
| BURS_ | Yes | Yes | Yes (external) | No | No | Partial |
| BURD_ | Yes | No (runtime) | No | No | No | N/A (not a regime) |
| RBRR_ | Yes | Yes | Yes | Yes | No | Mostly |
| RBRN_ | Yes | Yes | Yes (3 files) | Yes | No | Mostly |
| RBRP_ | Yes | Yes | Yes | No | No | Partial |
| RBRE_ | Yes | Yes | No (external) | No | No | Partial |
| RBRG_ | Yes | Yes | No (external) | No | No | Partial |
| RBRS_ | Yes | Yes | No (external) | No | No | Partial |
| RBRV_ | Yes | Yes | Yes (6 files) | No | No | Partial |
| RBRA_ | Yes | No | N/A | No | No | N/A (not a regime) |

### Prefix Naming Patterns

All regimes follow the mint discipline:
- **BUK domain:** `BU` + one-char suffix: `BURC`, `BURS`, `BUD`
- **RBW domain:** `RBR` + one-char suffix: `RBRR`, `RBRN`, `RBRP`, `RBRE`, `RBRG`, `RBRS`, `RBRV`, `RBRA`

Terminal exclusivity is maintained -- no prefix is both a leaf and a parent.

### Validator Pattern Consistency

All validators follow a uniform structure:
1. Copyright header
2. `set -euo pipefail`
3. Multiple inclusion guard (`Z{PREFIX}_SOURCED`)
4. `z{prefix}_kindle()` function with `Z{PREFIX}_KINDLED` guard
5. Uses `buv_env_*` validation functions (from BUK validation utilities)
6. `z{prefix}_sentinel()` function for downstream callers
7. Rollup construction (RBRN, RBRV only)

---

## Gap Analysis

### Missing Spec Documents (5 regimes)

| Regime | Impact | Recommendation |
|--------|--------|----------------|
| BURC_ | Medium | Create `BURC-RegimeConfiguration.adoc` -- well-defined scope, 10 variables |
| RBRP_ | Low | Create `RBRP-RegimePayor.adoc` -- small (3 vars), but payor setup is critical path |
| RBRE_ | Low | External credentials; spec may not add value |
| RBRG_ | Low | External credentials; spec may not add value |
| RBRS_ | Low | Create `RBRS-RegimeStation.adoc` -- small but documents workstation requirements |

### Missing Glossary Documents (all regimes)

No regime has a CRR-compliant `{crg_glossary_adoc}`. RBRN and RBRR have spec docs that serve some of this purpose, but formal glossary files with `tag::mapping-section[]` and linked term definitions don't exist.

### Missing Service Makefiles (all regimes)

CRR defines `{crg_service_mk}` with validation and render targets. No regime has one -- all validation is done via bash validators called at kindle time. This is a design divergence: the project uses bash-first validation rather than make-based validation.

### Validators Without Full Type Coverage

- RBRP: Uses custom regex validation instead of `buv_env_*` for all 3 variables
- RBRR: Uses `buv_env_gname` and `buv_env_odref` which aren't in CRR's type system (project extensions)
- BUD: No validation at all (runtime-only)

### RBRV Mode Semantics

RBRV uses a mutual-presence pattern (bind vs conjure) rather than an explicit mode enum. Both can be set simultaneously (the validator allows it). This differs from RBRN's explicit `_MODE` enum pattern. Consider whether RBRV should have an explicit `RBRV_MODE` variable (`bind`/`conjure`/`both`).

### RBRN Spec-Validator Drift

The RBRN spec doc still documents boolean flags (`ENTRY_ENABLED`, `DNS_ENABLED`, `ACCESS_ENABLED`, `DNS_GLOBAL`, `ACCESS_GLOBAL`) but the validator now uses tri-state `_MODE` enums. The spec needs updating to match the current validator.

### Credential Regime Gap

RBRA is used consistently (4 variables, written by 2 producers, consumed by 1 consumer) but has no validator. Since credential files are high-value targets, a validator would catch format errors early. Consider creating `rbra_regime.sh`.

---

## Summary Statistics

| Metric | Count |
|--------|-------|
| Total regime prefixes | 11 (9 declared + BUD runtime + RBRA format) |
| Declared regimes with validators | 9 |
| Regimes with spec documents | 2 (RBRR, RBRN) |
| Regimes with glossary documents | 0 |
| Regimes with service makefiles | 0 |
| Total declared variables | ~83 (across all regimes) |
| Git-tracked assignment files | ~12 (1 BURC + 1 RBRR + 3 RBRN + 1 RBRP + 6 RBRV) |
| External (not-in-git) assignment files | ~4 (1 BURS + RBRE + RBRG + RBRS) |
| RBRA credential files | 3 (governor, retriever, director) |
