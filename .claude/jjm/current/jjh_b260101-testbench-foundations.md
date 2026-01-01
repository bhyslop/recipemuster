# Heat: Testbench Foundations

## Context

Establish vocabulary and architecture foundations for cloud operation testing. Migrated audit paces from cloud-first-light heat, which validated individual operations but left systematic testing infrastructure undefined.

### Prerequisites

- Merged dockerize-bashize-proto-bottle branch (local testbench: rbt_testbench.sh)
- Keeper depot operational (rbwg-d-proto-251230080456)
- All 14 RBSGS operations validated in cloud-first-light

### Problem Statement

**Vocabulary gap**: No canonical terminology for operation identity. Tests, guides, and routing use inconsistent vocabulary (function names vs command tokens vs "stems").

**Architecture gap**: Local testbench (rbt_testbench.sh) exists for container security tests, but cloud operation testing has no defined architecture. Prior GitHub Action tested image lifecycle only; full depot lifecycle has logistical constraints (billing quota, 30-day delete delay).

### Goals

1. Resolve operation identity vocabulary and update BUK README
2. Define cloud testbench architecture (two-tier: heavy/light)
3. Revive/create image lifecycle test suite (light tier)
4. Complete audits deferred from cloud-first-light

## Remaining

- **Define operation identity vocabulary** — Decide canonical identity for operations: command token (e.g., `rbw-PC`) vs bash function (e.g., `rbgp_depot_create`). Define "stem" terminology or choose better term. Establish patterns for: test naming, error messages, guide cross-references. Update `Tools/buk/README.md` TabTargets section with formal vocabulary.
  mode: manual

- **Define cloud testbench architecture** — Document relationship to local testbench (rbt_testbench.sh). Define two-tier model: Heavy (full depot lifecycle, run rarely due to quota/deletion constraints) vs Light (image lifecycle against keeper depot, can run frequently/CI). Clarify which operations belong to each tier. Document keeper depot role.
  mode: manual

- **Revive image lifecycle test (light tier)** — Port/adapt prior GitHub Action. Runs against keeper depot. Operations: trigger_build, image_list, image_retrieve, image_delete. Create bash test functions following rbt_testbench.sh patterns. Can run frequently without quota concerns.
  mode: manual

- **Design depot lifecycle test (heavy tier)** — Document when/why to run full lifecycle. May remain manual or very-infrequent. Full flow: depot_create → governor_reset → director_create → trigger_build → image_list → image_delete → depot_destroy. Awareness of quota/deletion constraints.
  mode: manual

- **Audit dead code in rbga/rbgb/rbgp modules** — Determine if rbga_*, rbgb_*, and zrbgp_billing_* functions are dead code. Remove or document why retained. (Moved from cloud-first-light)
  mode: manual

- **Add GAR repository name validation** — Build failed silently because RBRR_GAR_REPOSITORY didn't match actual depot repository. Options: (1) runtime validation in rbf_build, (2) depot_create writes RBRR_GAR_REPOSITORY, (3) derive from depot project ID. (Moved from cloud-first-light)
  mode: manual

- **Audit tabtargets for log/no-log correctness** — Review all tt/*.sh for BUD_NO_LOG usage. Secret-handling tabtargets disable logging. Update old-form tabtargets to launcher pattern. (Moved from cloud-first-light)
  mode: manual

- **Audit director/repository configuration process** — Verify Director SA gets repoAdmin on GAR repo during director_create. Verify Mason SA gets writer (not admin) during depot_create. Ensure no manual IAM grants needed between operations. (Moved from cloud-first-light)
  mode: manual

## Done

(none yet)

## Steeplechase

---
### 2026-01-01 - Heat Created

**Origin**: Paces 1-4 (vocabulary, architecture, tests) are new. Paces 5-8 moved from cloud-first-light heat after merge of dockerize-bashize-proto-bottle brought test infrastructure.

**Rationale**: Cloud-first-light validated individual operations; this heat establishes systematic testing foundations.

---
