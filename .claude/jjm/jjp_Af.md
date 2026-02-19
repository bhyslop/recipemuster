# Regime Second-Pass Inventory

Complete regime list with per-regime concerns for BCG compliance, spec-validator alignment, and inventory sync.
Each regime annotated with AXLA cardinality: Singleton (one assignment source) or Manifold (multiple instances).

## BUK Domain

### BURC — Configuration Regime [Singleton]
- Spec: inline in BUSA
- Validator: burc_regime.sh
- Concerns:
  - Unquoted ${#ARRAY[@]} (line 75)

### BURS — Station Regime [Singleton]
- Spec: inline in BUSA
- Validator: burs_regime.sh
- Concerns:
  - Unquoted ${#ARRAY[@]} (line 60)

### BURD — Dispatch Runtime [Singleton]
- Spec: BUSD-DispatchRuntime.adoc (included from BUSA)
- Validator: none (runtime-only, by design)
- Concerns:
  - Confirm runtime-only status is intentional and documented

### BURE — Environment Regime [Singleton]
- Spec: inline in BUSA
- Validator: bure_regime.sh
- Concerns:
  - Unquoted ${#ARRAY[@]} (line 63)
  - Missing from AT paddock inventory — needs addition to any regime census

## RBW Domain

### RBRR — Repository Regime [Singleton]
- Spec: RBSRR-RegimeRepo.adoc (included from RBSA)
- Validator: rbrr_regime.sh
- Concerns:
  - Unquoted ${#ARRAY[@]} (line 115)
  - [[ == ]] without =~ (lines 161, 169, 176) — should use test

### RBRN — Nameplate Regime [Manifold: nsproto, srjcl, pluml]
- Spec: RBRN-RegimeNameplate.adoc (included from RBSA)
- Validator: rbrn_regime.sh
- Concerns:
  - Unquoted ${#ARRAY[@]} (line 126)
  - [[ == ]] without =~ at 8+ sites (175, 186, 189, 305, 308, 316, 327, 335)
  - Unquoted variables in test (lines 179–182)
  - [[ ]] for arithmetic comparison (line 267)
  - Spec-validator drift: spec documents boolean flags but validator uses tri-state MODE enums

### RBRP — Payor Regime [Singleton]
- Spec: RBSRP-RegimePayor.adoc (included from RBSA)
- Validator: rbrp_regime.sh
- Concerns:
  - grep -qE instead of [[ =~ ]] at 3 sites (lines 48, 58, 67)

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
- Live consumer: rbv_PodmanVM.sh (lines 134–135, 204) does ad-hoc RBRG_PAT/RBRG_USERNAME checks
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
- Concerns:
  - Unquoted ${#ARRAY[@]} (line 78)
  - [[ == ]] without =~ (lines 93, 98)
  - No explicit mode enum (bind vs conjure mutual-presence pattern)

### RBRA — Authentication/Credential Regime [Manifold: governor, retriever, director]
- Spec: RBSRA-CredentialFormat.adoc (included from RBSA)
- Validator: rbra_regime.sh
- Concerns:
  - Unquoted ${#ARRAY[@]} (line 71)
  - grep -qE instead of [[ =~ ]] (lines 82, 87)
  - Not a formal CRR regime (credential format convention)

## Cross-Cutting Concerns

- Unquoted ${#ARRAY[@]} is systemic — affects 8 of 10 validators
- [[ == ]] misuse affects RBRN (8+ sites), RBRR (3 sites), RBRV (2 sites)
- grep -qE misuse affects RBRP (3 sites), RBRA (2 sites)
- No regime has a CRR glossary document
- RBRN spec-validator drift needs reconciliation
- RBRO has spec but no implementation
- BURE missing from regime census documents