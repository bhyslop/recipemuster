# Regime Second-Pass Inventory

Complete regime list with per-regime outcomes from the second-pass transformation.
Each regime annotated with AXLA cardinality: Singleton (one assignment source) or Manifold (multiple instances).

All regimes now use buv enrollment infrastructure: `buv_*_enroll` for field contracts, `buv_scope_sentinel` for stray detection, `z*_enforce` (buv_vet + custom checks) as ironclad gate in furnish, and `buv_report`/`buv_render` for CLI validate/render commands.

## BUK Domain

### BURC — Configuration Regime [Singleton]
- Spec: inline in BUS0
- Module: burc_regime.sh / burc_cli.sh
- Scrubbed: ₢AfAAi — enrollment, enforce, scope_sentinel

### BURS — Station Regime [Singleton]
- Spec: inline in BUS0
- Module: burs_regime.sh / burs_cli.sh
- Scrubbed: ₢AfAAf — enrollment, enforce, scope_sentinel

### BURD — Dispatch Runtime [Singleton]
- Spec: BUSD-DispatchRuntime.adoc (included from BUS0)
- Module: burd_regime.sh (runtime-only, no CLI — by design)
- Scrubbed: ₢AfAAc — enrollment applied to runtime variables

### BURE — Environment Regime [Singleton]
- Spec: inline in BUS0
- Module: bure_regime.sh / bure_cli.sh
- Scrubbed: ₢AfAAj — enrollment for VERBOSE, COLOR, COUNTDOWN; ambient vars relocated from BURD (₢AfAAB)

## RBW Domain

### RBRR — Repository Regime [Singleton]
- Spec: RBSRR-RegimeRepo.adoc (included from RBS0)
- Module: rbrr_regime.sh / rbrr_cli.sh
- Scrubbed: ₢AfAAC — exemplar singleton; enrollment, enforce, scope_sentinel, validate, render

### RBRN — Nameplate Regime [Manifold: nsproto, srjcl, pluml]
- Spec: RBRN-RegimeNameplate.adoc (included from RBS0)
- Module: rbrn_regime.sh / rbrn_cli.sh
- Scrubbed: ₢AfAAE — exemplar manifold; enrollment, enforce, differential furnish with folio-conditional kindle

### RBRP — Payor Regime [Singleton]
- Spec: RBSRP-RegimePayor.adoc (included from RBS0)
- Module: rbrp_regime.sh / rbrp_cli.sh
- Scrubbed: ₢AfAAh — enrollment, enforce, grep -qE replaced

### RBRO — OAuth Regime [Singleton]
- Spec: RBSRO-RegimeOauth.adoc (included from RBS0)
- Module: rbro_regime.sh / rbro_cli.sh
- Scrubbed: ₢AfAAO — new module created (previously spec-only), enrollment, enforce, consumers validated

### RBRE — ECR Regime [Singleton]
- Spec: RBSRE-RegimeEcr.adoc (included from RBS0)
- Module: rbre_regime.sh
- Scrubbed: ₢AfAAg — enrollment, enforce

### ~~RBRG — GitHub Regime [Singleton]~~ REMOVED
- Removed: ₢AfAAA — ghost regime (spec'd, never implemented), consumers migrated

### RBRS — Station Regime [Singleton]
- Spec: RBSRS-RegimeStation.adoc (included from RBS0)
- Module: rbrs_regime.sh / rbrs_cli.sh
- Scrubbed: ₢AfAAe — enrollment, enforce

### RBRV — Vessel Regime [Manifold: 6 vessels in rbev-vessels/]
- Spec: RBSRV-RegimeVessel.adoc (included from RBS0)
- Module: rbrv_regime.sh / rbrv_cli.sh
- Scrubbed: ₢AfAAI — structural rebuild; enrollment, enforce, buc_execute, differential furnish

### RBRA — Authentication/Credential Regime [Manifold: governor, retriever, director]
- Spec: RBSRA-CredentialFormat.adoc (included from RBS0)
- Module: rbra_regime.sh / rbra_cli.sh
- Scrubbed: ₢AfAAk — enrollment, enforce, grep -qE replaced

## Workbench Ad-Hoc Imprint Translation — RESOLVED

The rbw workbench ad-hoc case arms for `BURD_TOKEN_3` → `RBOB_MONIKER` translation have been eliminated by the buz channel mechanism (₢AfAAD) and RBRN scrub (₢AfAAE). All bottle colophons now route through `zbuz_exec_lookup`.

## Cross-Cutting Concerns — RESOLVED

All cross-cutting concerns from the initial audit have been resolved by the enrollment transformation:

- ~~Unquoted ${#ARRAY[@]}~~ — eliminated; buv_scope_sentinel handles array safety internally
- ~~`[[ == ]]` misuse~~ — replaced with `[[ =~ ]]` or `test =` in all regime scrubs
- ~~`grep -qE` misuse~~ — replaced with `[[ =~ ]]` in RBRP and RBRA scrubs
- ~~RBRO has spec but no implementation~~ — rbro_regime.sh created (₢AfAAO)
- ~~BURE missing from regime census~~ — added and scrubbed (₢AfAAj)
- ~~`rbrn_load_moniker` consumers~~ — eliminated; RBRN uses standard enrollment pattern
- ~~Spec-validator drift~~ — enrollment declarations are the single source of truth
- CRR glossary documents — not created (out of scope for this heat)

## Infrastructure Paces

Key non-regime paces that enabled the transformation:

- ₢AfAAN — buv enrollment infrastructure (buv_*_enroll, buv_vet, buv_report, buv_scope_sentinel)
- ₢AfAAD — buz channel infrastructure (imprint translation)
- ₢AfAAQ — buv enrollment tests
- ₢AfAAO — audit legacy buv callers
- ₢AfAAP — delete legacy buv_val_*/buv_env_*/buv_opt_* functions
- ₢AfAAJ — BCG updates (regime archetype section, enforce boilerplate, stale buv references removed)
