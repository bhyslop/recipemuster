# Regime Second-Pass Inventory

Complete regime list with per-regime concerns for BCG compliance, spec-validator alignment, and inventory sync.
Each regime annotated with AXLA cardinality: Singleton (one assignment source) or Manifold (multiple instances).

## BUK Domain

### BURC — Configuration Regime [Singleton]
- Spec: inline in BUSA
- Validator: burc_regime.sh
- Concerns:
  - Unquoted ${#ARRAY[@]} — search for `ZBURC_UNEXPECTED`

### BURS — Station Regime [Singleton]
- Spec: inline in BUSA
- Validator: burs_regime.sh
- Concerns:
  - Unquoted ${#ARRAY[@]} — search for `ZBURS_UNEXPECTED`

### BURD — Dispatch Runtime [Singleton]
- Spec: BUSD-DispatchRuntime.adoc (included from BUSA)
- Validator: none (runtime-only, by design)
- Concerns:
  - Confirm runtime-only status is intentional and documented

### BURE — Environment Regime [Singleton]
- Spec: inline in BUSA
- Validator: bure_regime.sh
- Concerns:
  - Unquoted ${#ARRAY[@]} — search for `ZBURE_UNEXPECTED`
  - Missing from AT paddock inventory — needs addition to any regime census

## RBW Domain

### RBRR — Repository Regime [Singleton]
- Spec: RBSRR-RegimeRepo.adoc (included from RBSA)
- Validator: rbrr_regime.sh
- Concerns:
  - Unquoted ${#ARRAY[@]} — search for `ZRBRR_UNEXPECTED`
  - ~~[[ == ]] without =~ — FIXED in commit `3b745da8`~~

### RBRN — Nameplate Regime [Manifold: nsproto, srjcl, pluml]
- Spec: RBRN-RegimeNameplate.adoc (included from RBSA)
- Validator: rbrn_regime.sh
- Consumers of `rbrn_load_moniker`: rbob_cli.sh, rbtb_testbench.sh
- Concerns:
  - Unquoted ${#ARRAY[@]} — search for `ZRBRN_UNEXPECTED`
  - `[[ == ]]` without =~ at 8+ sites — search for `\[\[ .* == ` in rbrn_regime.sh
  - Unquoted variables in test — search for `test $` or `test ${` without quotes
  - `[[ ]]` for arithmetic comparison — search for numeric comparisons using `[[ ]]`
  - Spec-validator drift: spec documents boolean flags but validator uses tri-state MODE enums

### RBRP — Payor Regime [Singleton]
- Spec: RBSRP-RegimePayor.adoc (included from RBSA)
- Validator: rbrp_regime.sh
- Concerns:
  - `grep -qE` instead of `[[ =~ ]]` at 3 sites — search for `grep -qE` in rbrp_regime.sh

### RBRO — OAuth Regime [Singleton]
- Spec: RBSRO-RegimeOauth.adoc (included from RBSA)
- Validator: NO rbro_regime.sh exists
- Concerns:
  - Fully spec'd (voicings, include file, kindle/validate/render ops) but no validator implementation
  - Missing from AT paddock inventory

### RBRE — ECR Regime [Singleton]
- Spec: RBSRE-RegimeEcr.adoc (included from RBSA)
- Validator: rbre_regime.sh
- Concerns:
  - Audit for BCG compliance (not yet checked in detail)

### RBRG — GitHub Regime [Singleton] — CANDIDATE FOR REMOVAL
- Spec: RBSRG-RegimeGithub.adoc (included from RBSA)
- Validator: NO rbrg_regime.sh exists (never implemented)
- Status: Ghost regime. Spec'd but never got a proper implementation.
- Live consumer: rbv_PodmanVM.sh — search for `RBRG_PAT` and `RBRG_USERNAME`
- Legacy consumers: Tools/ABANDONED-github/ (leave these alone)
- Decision: REMOVE from active codebase. First pace.

### RBRS — Station Regime [Singleton]
- Spec: RBSRS-RegimeStation.adoc (included from RBSA)
- Validator: rbrs_regime.sh
- Concerns:
  - Appears BCG-compliant per initial audit

### RBRV — Vessel Regime [Manifold: 6 vessels in rbev-vessels/]
- Spec: RBSRV-RegimeVessel.adoc (included from RBSA)
- Validator: rbrv_regime.sh
- CLI: rbrv_cli.sh — **structurally non-compliant** (no furnish, no buc_execute, module-level kindle, ad-hoc case dispatch)
- Concerns:
  - Unquoted ${#ARRAY[@]} — search for `ZRBRV_UNEXPECTED`
  - `[[ == ]]` without =~ — search for `\[\[ .* == ` in rbrv_regime.sh
  - No explicit mode enum (bind vs conjure mutual-presence pattern)

### RBRA — Authentication/Credential Regime [Manifold: governor, retriever, director]
- Spec: RBSRA-CredentialFormat.adoc (included from RBSA)
- Validator: rbra_regime.sh
- Concerns:
  - Unquoted ${#ARRAY[@]} — search for `ZRBRA_UNEXPECTED`
  - `grep -qE` instead of `[[ =~ ]]` — search for `grep -qE` in rbra_regime.sh
  - Not a formal CRR regime (credential format convention)

## Workbench Ad-Hoc Imprint Translation

The rbw workbench (`rbw_workbench.sh`) has explicit case arms for bottle operations that manually translate `BURD_TOKEN_3` (the imprint) to `RBOB_MONIKER`. This is the code that the buz channel mechanism (₢AfAAD) and RBRN scrub (₢AfAAE) will replace.

- Location: `rbw_workbench.sh` — search for `BURD_TOKEN_3` and `RBOB_MONIKER`
- Affected colophons: `rbw-s`, `rbw-z`, `rbw-S`, `rbw-C`, `rbw-B`, `rbw-o`
- Comment in rbz_zipper.sh line 119: "imprint-translated by workbench case arm, not zbuz_exec_lookup"

After buz channel lands, these case arms should collapse into the generic `zbuz_exec_lookup` path.

## Cross-Cutting Concerns

- Unquoted ${#ARRAY[@]} is systemic — affects 8 of 10 validators (search for `${#Z.*UNEXPECTED\[@\]}`)
- `[[ == ]]` misuse affects RBRN (8+ sites), RBRV (2 sites) — search for `\[\[ .* == `
- `grep -qE` misuse affects RBRP (3 sites), RBRA (2 sites) — search for `grep -qE`
- RBRR `[[ == ]]` sites already fixed (commit `3b745da8`, replaced with `[[ =~ ]]`)
- No regime has a CRR glossary document
- RBRN spec-validator drift needs reconciliation
- RBRO has spec but no implementation
- BURE missing from regime census documents