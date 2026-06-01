# Heat Trophy: rbk-mvp-release-finalize

**Firemark:** ₣AU
**Created:** 260131
**Retired:** 260601
**Status:** retired

## Paddock

## Paddock: rbk-14-mvp-release-finalize

## Context

A small cleanup-and-validation heat at the tail of the Recipe Bottle release
effort. Its original charter — supply-chain trust hardening plus
release-qualification tooling — has largely shipped or migrated. The
release-qualification charter now lives in the rbk-16 heat (₣BB): the
marshal-zero → gauntlet/pristine model, the RELEASE.md runbook, and the
prep-release ceremony all belong there. The prep-release ceremony rewrite and
the foundry/cloud-step trust audit that once lived here were restrung into ₣BB
so the release ceremony and the foundry surface each have a single owner.

What remains in ₣AU is genuinely local: developer-facing hygiene and end-to-end
validation that do not depend on the gauntlet model.

## Remaining Scope

- **Regime-render error hygiene** — every regime render/validate/info function
  should fail with an actionable parameter message, not a terse "module not
  kindled" crash or a set -u unbound-variable death. Includes a sweep test
  fixture that exercises every render/info and asserts clean exit, plus logging
  discipline for batch-sweep context.
- **Onboarding end-to-end walk** — exercise every handbook track in sequence
  from a learner's perspective; confirm windows render with prefixed resource
  names and probe outputs report the correct prefixed identifiers.
- **openssl base64 helper factoring** — the six base64 sites that ₣BB's base64
  sweep unified on openssl still carry a duplicated invocation; factor into
  rbgu_Utility.sh helpers, or close won't-fix with rationale recorded.
- **Small doc/file hygiene** — relocate the compose-probe sentinel out of the
  .rbk/ runtime config directory into Tools/rbk/ (the file move itself already
  appears done — verify references before closing); and a BCG clarification
  distinguishing the `$(<file)` builtin redirect from command substitution
  (verify against the landed substitution-rule refinement first — may already
  be covered).

## Relationship to other heats

- **₣BB (rbk-16-mvp-release-qualification)** — the live release-qualification
  heat; inherited this heat's prep-release ceremony work and the foundry trust
  audit. The audit is sequenced at ₣BB's tail so it reviews the post-reshaping
  foundry surface rather than transitional code.
- **₣Ak (rbk-mvp-1, retired)** — prior art for the directory/spec/test
  structure.

## Deferred technical debt (post-release itch candidates)

Catalogued during this heat's BCG-compliance work; deferred, not lost. Better
homed in jji_itch.md than in the paddock long-term:

- `z_` prefix on locals — naming convention, no runtime impact (pervasive)
- Unbraced `"$var"` expansions — mechanical
- `2>/dev/null` stderr suppression — case-by-case judgment
- `local -r` missing on single-assignment locals — pervasive
- Raw `echo` for user messages in display-heavy modules
- Bare file truncation (`> file` without `:`)

## References

- Tools/rbk/rbq_Qualify.sh — qualification orchestrator
- Tools/buk/buv_validation.sh — core `buv_render()` engine
- Tools/buk/vov_veiled/BCG-BashConsoleGuide.md — command-substitution rules
- Tools/rbk/rbgu_Utility.sh — utility home for the base64 helpers
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — main specification

## Paces

### emplace-depot-prefix-on-gar-images (₢AUAA3) [abandoned]

**[260415-1525] abandoned**

## Character
Design + mechanical — introduce a new regime constant, thread it through all GAR image reference construction.

## Docket

Add `RBRR_DEPOT_PREFIX` to the repo regime. This prefix gets prepended to every GAR image path, giving all images a common namespace root that can be used for global lookups (e.g., reverse-mapping a hallmark to its vessel without iterating all vessel packages).

### Changes required

- **RBRR regime**: Add `RBRR_DEPOT_PREFIX` constant (repo regime enrollment + render)
- **RBS0 / RBSRR**: Spec the new field — what it is, why it exists, how it composes with existing GAR path construction
- **GAR path construction**: Every site that builds `${ZRBFC_REGISTRY_API_BASE}/${RBRV_SIGIL}` or similar must incorporate the prefix. Audit all six vessel+hallmark commands plus tally, rekon, inscribe, etc.
- **Cloud Build substitutions**: `_RBGV_GAR_PATH` may need to incorporate the prefix
- **Derived constants**: `RBDC` or `ZRBFC` path assembly

### Design questions to resolve during execution

- Does the prefix go between the GAR repository and the vessel sigil (e.g., `repo/PREFIX/vessel:tag`), or is it a prefix on the vessel sigil itself (e.g., `repo/PREFIX-vessel:tag`)?
- What is the default value? Empty string (backward compatible) or a meaningful default?
- Does this affect enshrined tool images or only vessel images?

**[260415-0630] rough**

## Character
Design + mechanical — introduce a new regime constant, thread it through all GAR image reference construction.

## Docket

Add `RBRR_DEPOT_PREFIX` to the repo regime. This prefix gets prepended to every GAR image path, giving all images a common namespace root that can be used for global lookups (e.g., reverse-mapping a hallmark to its vessel without iterating all vessel packages).

### Changes required

- **RBRR regime**: Add `RBRR_DEPOT_PREFIX` constant (repo regime enrollment + render)
- **RBS0 / RBSRR**: Spec the new field — what it is, why it exists, how it composes with existing GAR path construction
- **GAR path construction**: Every site that builds `${ZRBFC_REGISTRY_API_BASE}/${RBRV_SIGIL}` or similar must incorporate the prefix. Audit all six vessel+hallmark commands plus tally, rekon, inscribe, etc.
- **Cloud Build substitutions**: `_RBGV_GAR_PATH` may need to incorporate the prefix
- **Derived constants**: `RBDC` or `ZRBFC` path assembly

### Design questions to resolve during execution

- Does the prefix go between the GAR repository and the vessel sigil (e.g., `repo/PREFIX/vessel:tag`), or is it a prefix on the vessel sigil itself (e.g., `repo/PREFIX-vessel:tag`)?
- What is the default value? Empty string (backward compatible) or a meaningful default?
- Does this affect enshrined tool images or only vessel images?

### ifrit-sortie-env-and-cap-net-raw (₢AUAAx) [complete]

**[260401-0651] complete**

## Character

Small scope but security-adjacent — four compose edits that make the ifrit bottle's security posture deliberate rather than inherited from Docker defaults.

## Docket

Effect four compose changes for ifrit sortie portability, CAP_NET_RAW hygiene, and UID alignment.

**Changes:**

1. **`.rbk/tadmor.compose.yml`** — Add `env_file: tadmor.rbrn.env` to bottle service so RBRN_* vars are available to ifrit sorties at runtime. This is a deliberate security boundary crossing: bottles are normally blind to RBRN vars (sentry gets them explicitly for iptables/dnsmasq). Ifrit needs them for penetration testing against nameplate network topology.

2. **`.rbk/rbob_compose.yml`** — Add `cap_drop: [NET_RAW]` to bottle service so production bottles lose the Docker-default CAP_NET_RAW.

3. **`.rbk/tadmor.compose.yml`** — Add `cap_add: [NET_RAW]` to bottle service so ifrit explicitly opts back in (scapy needs raw sockets). The cap_drop/cap_add pair makes this a deliberate, auditable decision.

4. **`.rbk/tadmor.compose.yml`** — Add `user: "1000:1000"` to bottle service so files created inside the bottle (by Claude Code, sorties, etc.) match the host volume owner. The ifrit vessel has no USER directive — it runs as root by default, causing ownership conflicts on volume-mounted workspace files. Non-root + CAP_NET_RAW is the correct posture: capabilities are kernel-level, independent of UID.

## Context

Ifrit sorties were hardcoding nameplate IPs. All 11 sorties now read `os.environ["RBRN_*"]` instead, but the bottle container currently receives zero RBRN vars. The `env_file` line closes that gap.

CAP_NET_RAW was never explicitly authorized — it arrived via Docker defaults. Deny-by-default in base, grant-by-exception in ifrit's nameplate fragment.

UID 0 (root) was never intentional — it's the Docker default. Files created as root conflict with host UID 1000 on the shared volume.

## Test plan

- `rbw-ch.Hail.tadmor` — verify bottle starts with RBRN vars visible
- `rbw-Is` (ifrit sortie) — verify sorties can read RBRN_* from environment
- Confirm cap_drop/cap_add via `docker inspect` on running containers
- Verify files created inside bottle are owned by 1000:1000

**[260401-0633] rough**

## Character

Small scope but security-adjacent — four compose edits that make the ifrit bottle's security posture deliberate rather than inherited from Docker defaults.

## Docket

Effect four compose changes for ifrit sortie portability, CAP_NET_RAW hygiene, and UID alignment.

**Changes:**

1. **`.rbk/tadmor.compose.yml`** — Add `env_file: tadmor.rbrn.env` to bottle service so RBRN_* vars are available to ifrit sorties at runtime. This is a deliberate security boundary crossing: bottles are normally blind to RBRN vars (sentry gets them explicitly for iptables/dnsmasq). Ifrit needs them for penetration testing against nameplate network topology.

2. **`.rbk/rbob_compose.yml`** — Add `cap_drop: [NET_RAW]` to bottle service so production bottles lose the Docker-default CAP_NET_RAW.

3. **`.rbk/tadmor.compose.yml`** — Add `cap_add: [NET_RAW]` to bottle service so ifrit explicitly opts back in (scapy needs raw sockets). The cap_drop/cap_add pair makes this a deliberate, auditable decision.

4. **`.rbk/tadmor.compose.yml`** — Add `user: "1000:1000"` to bottle service so files created inside the bottle (by Claude Code, sorties, etc.) match the host volume owner. The ifrit vessel has no USER directive — it runs as root by default, causing ownership conflicts on volume-mounted workspace files. Non-root + CAP_NET_RAW is the correct posture: capabilities are kernel-level, independent of UID.

## Context

Ifrit sorties were hardcoding nameplate IPs. All 11 sorties now read `os.environ["RBRN_*"]` instead, but the bottle container currently receives zero RBRN vars. The `env_file` line closes that gap.

CAP_NET_RAW was never explicitly authorized — it arrived via Docker defaults. Deny-by-default in base, grant-by-exception in ifrit's nameplate fragment.

UID 0 (root) was never intentional — it's the Docker default. Files created as root conflict with host UID 1000 on the shared volume.

## Test plan

- `rbw-ch.Hail.tadmor` — verify bottle starts with RBRN vars visible
- `rbw-Is` (ifrit sortie) — verify sorties can read RBRN_* from environment
- Confirm cap_drop/cap_add via `docker inspect` on running containers
- Verify files created inside bottle are owned by 1000:1000

**[260401-0619] rough**

## Character

Small scope but security-adjacent — three compose edits that deliberately cross a container environment boundary and make a capability grant auditable.

## Docket

Effect three compose changes for ifrit sortie portability and CAP_NET_RAW hygiene.

**Changes:**

1. **`.rbk/tadmor.compose.yml`** — Add `env_file: tadmor.rbrn.env` to bottle service so RBRN_* vars are available to ifrit sorties at runtime. This is a deliberate security boundary crossing: bottles are normally blind to RBRN vars (sentry gets them explicitly for iptables/dnsmasq). Ifrit needs them for penetration testing against nameplate network topology.

2. **`.rbk/rbob_compose.yml`** — Add `cap_drop: [NET_RAW]` to bottle service so production bottles lose the Docker-default CAP_NET_RAW.

3. **`.rbk/tadmor.compose.yml`** — Add `cap_add: [NET_RAW]` to bottle service so ifrit explicitly opts back in (scapy needs raw sockets). The cap_drop/cap_add pair makes this a deliberate, auditable decision.

## Context

Ifrit sorties were hardcoding nameplate IPs. All 11 sorties now read `os.environ["RBRN_*"]` instead, but the bottle container currently receives zero RBRN vars. The `env_file` line closes that gap.

CAP_NET_RAW was never explicitly authorized — it arrived via Docker defaults. Deny-by-default in base, grant-by-exception in ifrit's nameplate fragment.

## Test plan

- `rbw-ch.Hail.tadmor` — verify bottle starts with RBRN vars visible
- `rbw-Is` (ifrit sortie) — verify sorties can read RBRN_* from environment
- Confirm cap_drop/cap_add via `docker inspect` on running containers

### nameplate-regime-directory-restructure (₢AUAAv) [complete]

**[260401-0658] complete**

## Character

Mechanical but broad — many call sites share a single path-construction pattern, plus compose fragment paths. Requires careful grep-driven sweep and manual testing of bottle tabtargets.

## Docket

Restructure nameplate regime files from flat `{moniker}.rbrn.env` to directory-based `{moniker}/rbrn.env`. Also move per-nameplate compose fragments into the same directory.

**File moves:**
- `.rbk/tadmor.rbrn.env` → `.rbk/tadmor/rbrn.env`
- `.rbk/srjcl.rbrn.env` → `.rbk/srjcl/rbrn.env`
- `.rbk/pluml.rbrn.env` → `.rbk/pluml/rbrn.env`
- `.rbk/tadmor.compose.yml` → `.rbk/tadmor/compose.yml`

**Code changes — path construction (all use `RBCC_rbrn_suffix`):**
- `rbcc_Constants.sh:33` — redefine or remove `RBCC_rbrn_suffix`; may need a new path-building helper
- `rbob_cli.sh:126` — furnish: `${RBBC_dot_dir}/${z_folio}${RBCC_rbrn_suffix}`
- `rbob_bottle.sh:121` — env path: `${RBBC_dot_dir}/${RBRN_MONIKER}${RBCC_rbrn_suffix}`
- `rbob_bottle.sh:115` — compose fragment: `${RBBC_dot_dir}/${RBRN_MONIKER}.compose.yml`
- `rbrn_cli.sh:201` — furnish: `${RBBC_dot_dir}/${BUZ_FOLIO}${RBCC_rbrn_suffix}`
- `rbrn_cli.sh:48` — survey glob
- `rbrn_regime.sh:125,167` — list/preflight glob: `${RBBC_dot_dir}/*${RBCC_rbrn_suffix}`
- `rbtb_testbench.sh:93` — test load
- `rbtcrv_RegimeValidation.sh:329` — test validation
- `rbgm_ManualProcedures.sh:82` — onboarding

**Pre-existing bug:** `rblm_cli.sh:76,146` globs for `rbrn_*.env` (underscore prefix) — doesn't match current `*.rbrn.env`. Fix to new pattern while here.

**Compose invocation update:** `rbob_compose.yml` header comment (line 7-8) references old path pattern.

**Doc updates:**
- `RBRN-RegimeNameplate.adoc:8`
- `README.consumer.md:277`
- `CLAUDE.consumer.md` / `rbtid/CLAUDE.md:18`

**Test plan — exercise these tabtargets after change:**
- `rbw-ch` (Hail), `rbw-cr` (Rack), `rbw-cs` (Scry), `rbw-cC` (Charge), `rbw-cQ` (Quench) — bottle ops
- `rbw-Ic` (Ifrit), `rbw-Is` (Sortie) — bottle ops
- `rbw-rnl` (list), `rbw-ni` (survey), `rbw-nv` (audit) — nameplate ops
- `rbw-tf` (TestFixture) — test load

**[260401-0600] rough**

## Character

Mechanical but broad — many call sites share a single path-construction pattern, plus compose fragment paths. Requires careful grep-driven sweep and manual testing of bottle tabtargets.

## Docket

Restructure nameplate regime files from flat `{moniker}.rbrn.env` to directory-based `{moniker}/rbrn.env`. Also move per-nameplate compose fragments into the same directory.

**File moves:**
- `.rbk/tadmor.rbrn.env` → `.rbk/tadmor/rbrn.env`
- `.rbk/srjcl.rbrn.env` → `.rbk/srjcl/rbrn.env`
- `.rbk/pluml.rbrn.env` → `.rbk/pluml/rbrn.env`
- `.rbk/tadmor.compose.yml` → `.rbk/tadmor/compose.yml`

**Code changes — path construction (all use `RBCC_rbrn_suffix`):**
- `rbcc_Constants.sh:33` — redefine or remove `RBCC_rbrn_suffix`; may need a new path-building helper
- `rbob_cli.sh:126` — furnish: `${RBBC_dot_dir}/${z_folio}${RBCC_rbrn_suffix}`
- `rbob_bottle.sh:121` — env path: `${RBBC_dot_dir}/${RBRN_MONIKER}${RBCC_rbrn_suffix}`
- `rbob_bottle.sh:115` — compose fragment: `${RBBC_dot_dir}/${RBRN_MONIKER}.compose.yml`
- `rbrn_cli.sh:201` — furnish: `${RBBC_dot_dir}/${BUZ_FOLIO}${RBCC_rbrn_suffix}`
- `rbrn_cli.sh:48` — survey glob
- `rbrn_regime.sh:125,167` — list/preflight glob: `${RBBC_dot_dir}/*${RBCC_rbrn_suffix}`
- `rbtb_testbench.sh:93` — test load
- `rbtcrv_RegimeValidation.sh:329` — test validation
- `rbgm_ManualProcedures.sh:82` — onboarding

**Pre-existing bug:** `rblm_cli.sh:76,146` globs for `rbrn_*.env` (underscore prefix) — doesn't match current `*.rbrn.env`. Fix to new pattern while here.

**Compose invocation update:** `rbob_compose.yml` header comment (line 7-8) references old path pattern.

**Doc updates:**
- `RBRN-RegimeNameplate.adoc:8`
- `README.consumer.md:277`
- `CLAUDE.consumer.md` / `rbtid/CLAUDE.md:18`

**Test plan — exercise these tabtargets after change:**
- `rbw-ch` (Hail), `rbw-cr` (Rack), `rbw-cs` (Scry), `rbw-cC` (Charge), `rbw-cQ` (Quench) — bottle ops
- `rbw-Ic` (Ifrit), `rbw-Is` (Sortie) — bottle ops
- `rbw-rnl` (list), `rbw-ni` (survey), `rbw-nv` (audit) — nameplate ops
- `rbw-tf` (TestFixture) — test load

### rbra-regime-directory-restructure (₢AUAAw) [complete]

**[260401-0814] complete**

## Character

Surgical — fewer call sites than nameplates, but touches credential paths and needs a migration step for existing secrets.

## Docket

Restructure RBRA credential files from `rbra-{role}.env` to `{role}/rbra.env` within the secrets directory. Add `RBRA_ROLE` field for swizzle protection.

**File moves (in `${RBRR_SECRETS_DIR}`):**
- `rbra-governor.env` → `governor/rbra.env`
- `rbra-retriever.env` → `retriever/rbra.env`
- `rbra-director.env` → `director/rbra.env`

**Add `RBRA_ROLE` to each file:**
- `governor/rbra.env` gets `RBRA_ROLE=governor`
- `retriever/rbra.env` gets `RBRA_ROLE=retriever`
- `director/rbra.env` gets `RBRA_ROLE=director`

**Validation:** In `rbra_regime.sh` (or at source-time), assert `RBRA_ROLE` matches the expected role derived from the directory path. Die on mismatch — prevents copy-paste credential swizzle.

**Derived constants update:**
- `rbdc_DerivedConstants.sh:39-41` — change path pattern from `${RBRR_SECRETS_DIR}/rbra-{role}.env` to `${RBRR_SECRETS_DIR}/{role}/rbra.env`

**Directory provisioning:**
- Add `mkdir -p` for `governor/`, `retriever/`, `director/` in the access setup flow (wherever secrets dir is created)

**Migration of existing files:**
- Pace must include a one-shot migration: move existing `rbra-{role}.env` to `{role}/rbra.env` and inject `RBRA_ROLE={role}` line, so testing works without credential regeneration

**Code references to update:**
- `rbra_cli.sh:39-41,81` — file path resolution
- `rbf_Foundry.sh` — heavy consumer of `RBDC_DIRECTOR_RBRA_FILE` (paths come from RBDC, no direct changes needed if RBDC is updated)
- `rbap_AccessProbe.sh:79` — uses `RBDC_GOVERNOR_RBRA_FILE`

**Test plan:**
- `rbw-rar` (render auth), `rbw-rav` (validate auth), `rbw-ral` (list auth) — RBRA CLI ops
- Verify credential sourcing still works for foundry operations

**[260401-0601] rough**

## Character

Surgical — fewer call sites than nameplates, but touches credential paths and needs a migration step for existing secrets.

## Docket

Restructure RBRA credential files from `rbra-{role}.env` to `{role}/rbra.env` within the secrets directory. Add `RBRA_ROLE` field for swizzle protection.

**File moves (in `${RBRR_SECRETS_DIR}`):**
- `rbra-governor.env` → `governor/rbra.env`
- `rbra-retriever.env` → `retriever/rbra.env`
- `rbra-director.env` → `director/rbra.env`

**Add `RBRA_ROLE` to each file:**
- `governor/rbra.env` gets `RBRA_ROLE=governor`
- `retriever/rbra.env` gets `RBRA_ROLE=retriever`
- `director/rbra.env` gets `RBRA_ROLE=director`

**Validation:** In `rbra_regime.sh` (or at source-time), assert `RBRA_ROLE` matches the expected role derived from the directory path. Die on mismatch — prevents copy-paste credential swizzle.

**Derived constants update:**
- `rbdc_DerivedConstants.sh:39-41` — change path pattern from `${RBRR_SECRETS_DIR}/rbra-{role}.env` to `${RBRR_SECRETS_DIR}/{role}/rbra.env`

**Directory provisioning:**
- Add `mkdir -p` for `governor/`, `retriever/`, `director/` in the access setup flow (wherever secrets dir is created)

**Migration of existing files:**
- Pace must include a one-shot migration: move existing `rbra-{role}.env` to `{role}/rbra.env` and inject `RBRA_ROLE={role}` line, so testing works without credential regeneration

**Code references to update:**
- `rbra_cli.sh:39-41,81` — file path resolution
- `rbf_Foundry.sh` — heavy consumer of `RBDC_DIRECTOR_RBRA_FILE` (paths come from RBDC, no direct changes needed if RBDC is updated)
- `rbap_AccessProbe.sh:79` — uses `RBDC_GOVERNOR_RBRA_FILE`

**Test plan:**
- `rbw-rar` (render auth), `rbw-rav` (validate auth), `rbw-ral` (list auth) — RBRA CLI ops
- Verify credential sourcing still works for foundry operations

### purge-legacy-rubric-and-gitlab (₢AUAAt) [complete]

**[260401-0830] complete**

## Character
Mechanical bulk deletion — dead code removal, no behavioral change. Grep-driven sweep.

## Context
The "rubric inscribe" workflow (clone GitLab rubric repo, push build contexts, trigger builds via CB v2 connection) was eliminated in ₣Av. The replacement (builds.create + pouch) is fully operational. However, the dead code, constants, spec, and "eliminated" comments were never purged. Additionally, all GitLab references are legacy — RB now uses GitHub exclusively.

**KEEP**: `rbgji01-inscribe-mirror.sh` and its references — this is the current Cloud Build inscribe-mirror step, unrelated to the legacy rubric workflow.

## Inventory

### Dead code removal

| File | What to remove |
|------|---------------|
| `rbf_Foundry.sh` | `rbf_rubric_inscribe()` function (~lines 2168-2468), kindle constants `ZRBF_BUILD_RUBRIC_LS` and `ZRBF_INSCRIBE_CLONE_DIR` |
| `rbgg_Governor.sh` | `zrbgg_rubric_preflight()` function + 2 call sites (lines ~501-502, ~557-558) |
| `rbgm_ManualProcedures.sh` | `rbgm_gitlab_setup()` function |
| `rbgm_cli.sh` | `rbgm_gitlab_setup` dispatch case |
| `rbgc_Constants.sh` | Rubric constants block: `RBGC_RUBRIC_CLONE_DIR`, `RBGC_RUBRIC_TRIGGER_PREFIX`, all 4 `RBGC_CBV2_*` constants, `RBGC_CBV2_REPOSITORY_ID` |
| `rbra_regime.sh` | `RBRA_RUBRIC_REPO_URL` deprecated enrollment line |
| `rbz_zipper.sh` | `RBZ_GITLAB_SETUP` enrollment line |
| `rblm_cli.sh` | `RBRR_RUBRIC_REPO_URL` stripping case |
| `rbrr_regime.sh` | Rubric elimination comment |

### Spec/doc cleanup

| File | Action |
|------|--------|
| `RBSRI-rubric_inscribe.adoc` | Delete entire file |
| `RBS0-SpecTop.adoc` | Remove `:rbtc_gitlab_setup:` attribute, `[[rbtc_gitlab_setup]]` definition, rubric/gitlab comments |
| `RBSRR-RegimeRepo.adoc` | Remove gitlab elimination comment |
| `RBSDE-depot_levy.adoc` | Remove gitlab elimination comment block |
| `RBSDK-director_knight.adoc` | Remove gitlab/inscribe elimination comment |
| `RBSCB-CloudBuildPosture.adoc` | Check for rubric references |
| `RBSTB-trigger_build.adoc` | Check for rubric references |
| `README.consumer.md` | Remove rubric/gitlab references |
| `CLAUDE.consumer.md` | Remove rubric/gitlab references |
| `CLAUDE.md` | Remove RBSRI from file acronym mappings |

### Regime file cleanup
| File | Action |
|------|--------|
| `.gitignore` | Check for rubric-related ignore patterns |

## Acceptance
- Zero hits for `rubric` in .sh files (consumer runtime)
- Zero hits for `gitlab` (case-insensitive) in .sh files
- `RBSRI-rubric_inscribe.adoc` deleted
- `rbgji01-inscribe-mirror.sh` untouched
- No behavioral change — all removed code was already dead

**[260331-1145] rough**

## Character
Mechanical bulk deletion — dead code removal, no behavioral change. Grep-driven sweep.

## Context
The "rubric inscribe" workflow (clone GitLab rubric repo, push build contexts, trigger builds via CB v2 connection) was eliminated in ₣Av. The replacement (builds.create + pouch) is fully operational. However, the dead code, constants, spec, and "eliminated" comments were never purged. Additionally, all GitLab references are legacy — RB now uses GitHub exclusively.

**KEEP**: `rbgji01-inscribe-mirror.sh` and its references — this is the current Cloud Build inscribe-mirror step, unrelated to the legacy rubric workflow.

## Inventory

### Dead code removal

| File | What to remove |
|------|---------------|
| `rbf_Foundry.sh` | `rbf_rubric_inscribe()` function (~lines 2168-2468), kindle constants `ZRBF_BUILD_RUBRIC_LS` and `ZRBF_INSCRIBE_CLONE_DIR` |
| `rbgg_Governor.sh` | `zrbgg_rubric_preflight()` function + 2 call sites (lines ~501-502, ~557-558) |
| `rbgm_ManualProcedures.sh` | `rbgm_gitlab_setup()` function |
| `rbgm_cli.sh` | `rbgm_gitlab_setup` dispatch case |
| `rbgc_Constants.sh` | Rubric constants block: `RBGC_RUBRIC_CLONE_DIR`, `RBGC_RUBRIC_TRIGGER_PREFIX`, all 4 `RBGC_CBV2_*` constants, `RBGC_CBV2_REPOSITORY_ID` |
| `rbra_regime.sh` | `RBRA_RUBRIC_REPO_URL` deprecated enrollment line |
| `rbz_zipper.sh` | `RBZ_GITLAB_SETUP` enrollment line |
| `rblm_cli.sh` | `RBRR_RUBRIC_REPO_URL` stripping case |
| `rbrr_regime.sh` | Rubric elimination comment |

### Spec/doc cleanup

| File | Action |
|------|--------|
| `RBSRI-rubric_inscribe.adoc` | Delete entire file |
| `RBS0-SpecTop.adoc` | Remove `:rbtc_gitlab_setup:` attribute, `[[rbtc_gitlab_setup]]` definition, rubric/gitlab comments |
| `RBSRR-RegimeRepo.adoc` | Remove gitlab elimination comment |
| `RBSDE-depot_levy.adoc` | Remove gitlab elimination comment block |
| `RBSDK-director_knight.adoc` | Remove gitlab/inscribe elimination comment |
| `RBSCB-CloudBuildPosture.adoc` | Check for rubric references |
| `RBSTB-trigger_build.adoc` | Check for rubric references |
| `README.consumer.md` | Remove rubric/gitlab references |
| `CLAUDE.consumer.md` | Remove rubric/gitlab references |
| `CLAUDE.md` | Remove RBSRI from file acronym mappings |

### Regime file cleanup
| File | Action |
|------|--------|
| `.gitignore` | Check for rubric-related ignore patterns |

## Acceptance
- Zero hits for `rubric` in .sh files (consumer runtime)
- Zero hits for `gitlab` (case-insensitive) in .sh files
- `RBSRI-rubric_inscribe.adoc` deleted
- `rbgji01-inscribe-mirror.sh` untouched
- No behavioral change — all removed code was already dead

### unify-hashing-on-openssl (₢AUAAq) [complete]

**[260401-0834] complete**

## Character
Mechanical — four call sites, one pattern. Rescan on mount since files are being renamed.

## Context
Consumer runtime uses both `sha256sum` (Linux coreutils) and `shasum -a 256` (macOS Perl) for hashing, neither universal. `openssl dgst -sha256 -r` produces sha256sum-compatible output and is present on both platforms. Since openssl is already a committed dependency (JWT signing in rbgo_OAuth, checksum fallback in bud_dispatch), standardize all hashing on it.

## Pre-work: rescan
File renames are in progress. On mount, verify each call site still exists at the expected location.

## Conversion pattern

Replace all sha256sum/shasum invocations with `openssl dgst -sha256 -r`. The `-r` flag produces `hash *filename` format (sha256sum-compatible). Extract hash with `read -r z_hash _` (bash builtin, no awk/cut).

## Call sites (4)

| File | Current | Notes |
|------|---------|-------|
| `bud_dispatch.sh` | `sha256sum` with openssl fallback cascade | Simplify to openssl only |
| `jja_arcanum.sh` | `shasum -a 256` (macOS-only) | Fix portability bug |
| `vob_build.sh` | `sha256sum` then `shasum` dual probe | Simplify to openssl only |
| `rbgje01-enshrine-copy.sh` | `sha256sum` (GCB) | Standardize for consistency |

## Acceptance
- Zero invocations of sha256sum or shasum in the codebase
- All sites use `openssl dgst -sha256 -r`
- Hash extraction uses `read -r` not awk/cut
- All existing tests pass

**[260331-1046] rough**

## Character
Mechanical — four call sites, one pattern. Rescan on mount since files are being renamed.

## Context
Consumer runtime uses both `sha256sum` (Linux coreutils) and `shasum -a 256` (macOS Perl) for hashing, neither universal. `openssl dgst -sha256 -r` produces sha256sum-compatible output and is present on both platforms. Since openssl is already a committed dependency (JWT signing in rbgo_OAuth, checksum fallback in bud_dispatch), standardize all hashing on it.

## Pre-work: rescan
File renames are in progress. On mount, verify each call site still exists at the expected location.

## Conversion pattern

Replace all sha256sum/shasum invocations with `openssl dgst -sha256 -r`. The `-r` flag produces `hash *filename` format (sha256sum-compatible). Extract hash with `read -r z_hash _` (bash builtin, no awk/cut).

## Call sites (4)

| File | Current | Notes |
|------|---------|-------|
| `bud_dispatch.sh` | `sha256sum` with openssl fallback cascade | Simplify to openssl only |
| `jja_arcanum.sh` | `shasum -a 256` (macOS-only) | Fix portability bug |
| `vob_build.sh` | `sha256sum` then `shasum` dual probe | Simplify to openssl only |
| `rbgje01-enshrine-copy.sh` | `sha256sum` (GCB) | Standardize for consistency |

## Acceptance
- Zero invocations of sha256sum or shasum in the codebase
- All sites use `openssl dgst -sha256 -r`
- Hash extraction uses `read -r` not awk/cut
- All existing tests pass

### unify-base64-on-openssl (₢AUAAr) [complete]

**[260401-0838] complete**

## Character
Mechanical with careful testing — eight call sites across six files, plus elimination of two macOS compatibility hacks. The JWT path is security-sensitive and must be verified end-to-end.

## Context
Consumer runtime uses `base64` command for encoding and decoding. GNU and macOS implementations diverge: `-d` vs `-D` for decode, `-w0` vs `-b0` for no-wrap. Two files already have explicit fallback hacks for this. `openssl enc -base64` is portable across all platforms with no flag divergence, and openssl is already a committed dependency.

## Pre-work: rescan
File renames are in progress. On mount, verify each call site still exists at the expected location.

## Conversion patterns

Encode (JWT base64url):
- Before: `base64 | tr -d '\n' | tr '+/' '-_' | tr -d '='`
- After: `openssl enc -base64 -A | tr '+/' '-_' | tr -d '='`
- Note: `-A` suppresses line wrapping, eliminating a `tr -d '\n'` step

Decode (files and strings):
- Before: `base64 -d` (GNU) or `base64 -D` (macOS fallback)
- After: `openssl enc -base64 -d`
- Note: `-d` flag is consistent across all openssl builds

Critical rule: `-A` flag must match on encode and decode sides. External sources (GCB build step outputs, GCP key data) include newlines in their base64 — decode those WITHOUT `-A`.

## Call sites (8 executable + 1 command check)

| File | Line ref | Pattern | Notes |
|------|----------|---------|-------|
| rbgo_OAuth.sh | encode | JWT header base64url | Security-sensitive |
| rbgo_OAuth.sh | encode | JWT signature base64url | Security-sensitive |
| rbgo_OAuth.sh | command check | `command -v base64` | Remove this check |
| rbf_Foundry.sh | decode | Secret value decode | _capture function |
| rbf_Foundry.sh | decode | Build step output decode (enshrine) | File-to-file |
| rbf_Foundry.sh | decode | Build step output decode (consecration) | File-to-file |
| rbv_PodmanVM.sh | decode | Manifest entry decode | Here-string input |
| rgbs_ServiceAccounts.sh | decode | Private key decode | Pipe input |
| rbgp_Payor.sh | decode | Key decode with macOS -D hack | **Hack eliminated** |
| rbgg_Governor.sh | decode | Key decode with macOS -D hack | **Hack eliminated** |

## Verification plan

### A. OAuth JWT roundtrip (critical path)
After converting rbgo_OAuth.sh, run the full OAuth flow to verify JWT signing still produces valid tokens:
1. `tt/rbw-gPR.PayorRefresh.sh` — exercises JWT build, sign, and token exchange
2. Success = GCP returns a valid access token (the JWT was accepted)
3. If token exchange fails, the JWT encoding is broken — immediate rollback

### B. Service account key decode
1. `tt/rbw-GD.GovernorCreatesDirector.sh` — exercises key creation and decode (rbgg_Governor.sh)
2. Verify the decoded key JSON is valid: the subsequent curl call that uses it will fail if decode is wrong

### C. Build step output decode
1. `tt/rbw-DE.DirectorEnshrinesVessel.sh` — exercises the enshrine path (rbf_Foundry.sh build step decode)
2. Success = anchor results extracted and vouch artifact created

### D. Payor key decode (macOS hack removal)
1. `tt/rbw-gPI.PayorInstall.sh` — exercises rbgp_Payor.sh key decode path
2. Run on macOS to confirm the openssl path works without the -d/-D fallback

## Acceptance
- Zero invocations of standalone `base64` command in consumer runtime
- macOS `-d`/`-D` fallback hacks removed from rbgp_Payor.sh and rbgg_Governor.sh
- `command -v base64` check removed from rbgo_OAuth.sh
- OAuth token refresh succeeds (test A)
- All existing tests pass

**[260331-1104] rough**

## Character
Mechanical with careful testing — eight call sites across six files, plus elimination of two macOS compatibility hacks. The JWT path is security-sensitive and must be verified end-to-end.

## Context
Consumer runtime uses `base64` command for encoding and decoding. GNU and macOS implementations diverge: `-d` vs `-D` for decode, `-w0` vs `-b0` for no-wrap. Two files already have explicit fallback hacks for this. `openssl enc -base64` is portable across all platforms with no flag divergence, and openssl is already a committed dependency.

## Pre-work: rescan
File renames are in progress. On mount, verify each call site still exists at the expected location.

## Conversion patterns

Encode (JWT base64url):
- Before: `base64 | tr -d '\n' | tr '+/' '-_' | tr -d '='`
- After: `openssl enc -base64 -A | tr '+/' '-_' | tr -d '='`
- Note: `-A` suppresses line wrapping, eliminating a `tr -d '\n'` step

Decode (files and strings):
- Before: `base64 -d` (GNU) or `base64 -D` (macOS fallback)
- After: `openssl enc -base64 -d`
- Note: `-d` flag is consistent across all openssl builds

Critical rule: `-A` flag must match on encode and decode sides. External sources (GCB build step outputs, GCP key data) include newlines in their base64 — decode those WITHOUT `-A`.

## Call sites (8 executable + 1 command check)

| File | Line ref | Pattern | Notes |
|------|----------|---------|-------|
| rbgo_OAuth.sh | encode | JWT header base64url | Security-sensitive |
| rbgo_OAuth.sh | encode | JWT signature base64url | Security-sensitive |
| rbgo_OAuth.sh | command check | `command -v base64` | Remove this check |
| rbf_Foundry.sh | decode | Secret value decode | _capture function |
| rbf_Foundry.sh | decode | Build step output decode (enshrine) | File-to-file |
| rbf_Foundry.sh | decode | Build step output decode (consecration) | File-to-file |
| rbv_PodmanVM.sh | decode | Manifest entry decode | Here-string input |
| rgbs_ServiceAccounts.sh | decode | Private key decode | Pipe input |
| rbgp_Payor.sh | decode | Key decode with macOS -D hack | **Hack eliminated** |
| rbgg_Governor.sh | decode | Key decode with macOS -D hack | **Hack eliminated** |

## Verification plan

### A. OAuth JWT roundtrip (critical path)
After converting rbgo_OAuth.sh, run the full OAuth flow to verify JWT signing still produces valid tokens:
1. `tt/rbw-gPR.PayorRefresh.sh` — exercises JWT build, sign, and token exchange
2. Success = GCP returns a valid access token (the JWT was accepted)
3. If token exchange fails, the JWT encoding is broken — immediate rollback

### B. Service account key decode
1. `tt/rbw-GD.GovernorCreatesDirector.sh` — exercises key creation and decode (rbgg_Governor.sh)
2. Verify the decoded key JSON is valid: the subsequent curl call that uses it will fail if decode is wrong

### C. Build step output decode
1. `tt/rbw-DE.DirectorEnshrinesVessel.sh` — exercises the enshrine path (rbf_Foundry.sh build step decode)
2. Success = anchor results extracted and vouch artifact created

### D. Payor key decode (macOS hack removal)
1. `tt/rbw-gPI.PayorInstall.sh` — exercises rbgp_Payor.sh key decode path
2. Run on macOS to confirm the openssl path works without the -d/-D fallback

## Acceptance
- Zero invocations of standalone `base64` command in consumer runtime
- macOS `-d`/`-D` fallback hacks removed from rbgp_Payor.sh and rbgg_Governor.sh
- `command -v base64` check removed from rbgo_OAuth.sh
- OAuth token refresh succeeds (test A)
- All existing tests pass

### evict-trivial-posix-utilities (₢AUAAp) [complete]

**[260401-0847] complete**

## Character
Mechanical — six independent single-site replacements. Each is a one-liner or near it. Rescan on mount since files are being renamed.

## Context
Audit of consumer runtime scripts found six POSIX utilities with single or near-single call sites that have direct bash builtin replacements. Each eviction is independent — no ordering dependency between them.

## Pre-work: rescan
File renames are in progress. On mount, verify each call site still exists at the expected location before converting.

## Eviction inventory

| Command | Expected file | Replacement | Notes |
|---------|--------------|-------------|-------|
| **head** | rbi_Image.sh | `read -r` (for head -1) | Trivial |
| **tr** | rbgo_OAuth.sh | `${var//old/new}` parameter expansion | Verify which characters are translated |
| **cut** | buc_command.sh | `read` with IFS or parameter expansion | Verify field extraction pattern |
| **wc** | buq_qualify.sh, butd_dispatch.sh | `${#var}` for length, `${#arr[@]}` for counts | Two files |
| **tput** | rboo_observe.sh, bud_dispatch.sh | Hardcoded ANSI escapes | Loses terminfo portability — assess whether acceptable |
| **ls** | buut_tabtarget.sh | Glob expansion `for f in dir/*` | Verify what ls output is consumed |

## Notes
- **tput** is the only one requiring judgment — ANSI hardcoding loses terminfo abstraction. If the existing usage is just color/reset sequences that are already hardcoded elsewhere in BUK, it's fine. If it queries capabilities, reconsider.
- **awk** was already evicted from bul_launcher.sh (₢AUAAg notch: stty size + read).

## Acceptance
- Zero invocations of head, tr, cut, wc, ls in consumer runtime .sh files
- tput either evicted or explicitly kept with rationale
- All existing tests pass

**[260331-1041] rough**

## Character
Mechanical — six independent single-site replacements. Each is a one-liner or near it. Rescan on mount since files are being renamed.

## Context
Audit of consumer runtime scripts found six POSIX utilities with single or near-single call sites that have direct bash builtin replacements. Each eviction is independent — no ordering dependency between them.

## Pre-work: rescan
File renames are in progress. On mount, verify each call site still exists at the expected location before converting.

## Eviction inventory

| Command | Expected file | Replacement | Notes |
|---------|--------------|-------------|-------|
| **head** | rbi_Image.sh | `read -r` (for head -1) | Trivial |
| **tr** | rbgo_OAuth.sh | `${var//old/new}` parameter expansion | Verify which characters are translated |
| **cut** | buc_command.sh | `read` with IFS or parameter expansion | Verify field extraction pattern |
| **wc** | buq_qualify.sh, butd_dispatch.sh | `${#var}` for length, `${#arr[@]}` for counts | Two files |
| **tput** | rboo_observe.sh, bud_dispatch.sh | Hardcoded ANSI escapes | Loses terminfo portability — assess whether acceptable |
| **ls** | buut_tabtarget.sh | Glob expansion `for f in dir/*` | Verify what ls output is consumed |

## Notes
- **tput** is the only one requiring judgment — ANSI hardcoding loses terminfo abstraction. If the existing usage is just color/reset sequences that are already hardcoded elsewhere in BUK, it's fine. If it queries capabilities, reconsider.
- **awk** was already evicted from bul_launcher.sh (₢AUAAg notch: stty size + read).

## Acceptance
- Zero invocations of head, tr, cut, wc, ls in consumer runtime .sh files
- tput either evicted or explicitly kept with rationale
- All existing tests pass

### evict-grep-from-consumer-runtime (₢AUAAo) [complete]

**[260401-0855] complete**

## Character
Mechanical bulk conversion — pattern-match and replace, not design. Rescan required at mount time since files are being renamed.

## Context
Audit of consumer runtime scripts found ~44 grep invocations across ~19 files in Tools/rbk/ and Tools/buk/ consumer runtime scripts. ALL are replaceable with bash builtins — including the two `grep -c` sites in rbtcpl_PlumlDiagram.sh which are existence checks disguised as counts. This pace eliminates grep entirely as a runtime dependency for RB consumer scripts.

BCG already blesses `[[ var =~ pattern ]]` as the one acceptable `[[ ]]` use case. This is the primary replacement mechanism for grep -qE (regex checks). Simple substring checks use `case` or `test`.

## Pre-work: rescan
File renames are in progress. On mount, rescan Tools/rbk/*.sh and Tools/buk/*.sh for all grep invocations before starting conversions. Do NOT rely on stale line numbers or file names from the audit that produced this pace.

## Conversion patterns

| grep variant | Bash replacement |
|-------------|-----------------|
| `grep -q "str"` (substring) | `case $var in *str*) ... esac` or `test "${var#*str}" != "${var}"` |
| `grep -qE "regex"` | `[[ $var =~ regex ]]` (BCG-blessed exception) |
| `grep -qw "word"` | `[[ $var =~ (^|\s)word(\s|$) ]]` or space-padded case |
| `grep -o "pat" \| cut` (extraction) | `[[ $var =~ pat ]]` then `${BASH_REMATCH[]}` |
| `grep` in pipe (file filtering) | `while read` + `case` |
| `grep -c` (existence check) | `[[ $var =~ pat ]] && ...` (not true counting — just asserts absence) |
| `declare -F \| grep` (function check) | `type -t funcname` or `declare -F funcname` |

## Scope boundaries
- Consumer runtime only: Tools/rbk/*.sh (not rbgja/, rbgjb/, rbgjv/) and Tools/buk/*.sh
- Test fixtures in Tools/rbk/rbts/ are in scope (rbtcpl_PlumlDiagram.sh grep -c sites)
- sed stays (2 load-bearing uses, both earned)
- awk eviction already landed (bul_launcher.sh)

## Acceptance
- Zero grep invocations in consumer runtime .sh files including test fixtures
- All existing tests pass
- No behavioral change

**[260331-1109] rough**

## Character
Mechanical bulk conversion — pattern-match and replace, not design. Rescan required at mount time since files are being renamed.

## Context
Audit of consumer runtime scripts found ~44 grep invocations across ~19 files in Tools/rbk/ and Tools/buk/ consumer runtime scripts. ALL are replaceable with bash builtins — including the two `grep -c` sites in rbtcpl_PlumlDiagram.sh which are existence checks disguised as counts. This pace eliminates grep entirely as a runtime dependency for RB consumer scripts.

BCG already blesses `[[ var =~ pattern ]]` as the one acceptable `[[ ]]` use case. This is the primary replacement mechanism for grep -qE (regex checks). Simple substring checks use `case` or `test`.

## Pre-work: rescan
File renames are in progress. On mount, rescan Tools/rbk/*.sh and Tools/buk/*.sh for all grep invocations before starting conversions. Do NOT rely on stale line numbers or file names from the audit that produced this pace.

## Conversion patterns

| grep variant | Bash replacement |
|-------------|-----------------|
| `grep -q "str"` (substring) | `case $var in *str*) ... esac` or `test "${var#*str}" != "${var}"` |
| `grep -qE "regex"` | `[[ $var =~ regex ]]` (BCG-blessed exception) |
| `grep -qw "word"` | `[[ $var =~ (^|\s)word(\s|$) ]]` or space-padded case |
| `grep -o "pat" \| cut` (extraction) | `[[ $var =~ pat ]]` then `${BASH_REMATCH[]}` |
| `grep` in pipe (file filtering) | `while read` + `case` |
| `grep -c` (existence check) | `[[ $var =~ pat ]] && ...` (not true counting — just asserts absence) |
| `declare -F \| grep` (function check) | `type -t funcname` or `declare -F funcname` |

## Scope boundaries
- Consumer runtime only: Tools/rbk/*.sh (not rbgja/, rbgjb/, rbgjv/) and Tools/buk/*.sh
- Test fixtures in Tools/rbk/rbts/ are in scope (rbtcpl_PlumlDiagram.sh grep -c sites)
- sed stays (2 load-bearing uses, both earned)
- awk eviction already landed (bul_launcher.sh)

## Acceptance
- Zero grep invocations in consumer runtime .sh files including test fixtures
- All existing tests pass
- No behavioral change

**[260331-1020] rough**

## Character
Mechanical bulk conversion — pattern-match and replace, not design. Rescan required at mount time since files are being renamed.

## Context
Audit found ~44 grep invocations across ~19 files in Tools/rbk/ and Tools/buk/ consumer runtime scripts. Of these, ~42 are replaceable with bash builtins; only ~2 grep -c (counting) uses are worth keeping. This pace eliminates grep as a runtime dependency for RB consumer scripts.

BCG already blesses `[[ var =~ pattern ]]` as the one acceptable `[[ ]]` use case. This is the primary replacement mechanism for grep -qE (regex checks). Simple substring checks use `case` or `test`.

## Pre-work: rescan
File renames are in progress. On mount, rescan Tools/rbk/*.sh and Tools/buk/*.sh for all grep invocations before starting conversions. Do NOT rely on stale line numbers or file names from the audit that produced this pace.

## Conversion patterns

| grep variant | Bash replacement |
|-------------|-----------------|
| `grep -q "str"` (substring) | `case $var in *str*) ... esac` or `test "${var#*str}" != "${var}"` |
| `grep -qE "regex"` | `[[ $var =~ regex ]]` (BCG-blessed exception) |
| `grep -qw "word"` | `[[ $var =~ (^|\s)word(\s|$) ]]` or space-padded case |
| `grep -o "pat" \| cut` (extraction) | `[[ $var =~ pat ]]` then `${BASH_REMATCH[]}` |
| `grep` in pipe (file filtering) | `while read` + `case` |
| `grep -c` (counting) | **Keep** — builtin version too verbose to justify |
| `declare -F \| grep` (function check) | `type -t funcname` or `declare -F funcname` |

## Scope boundaries
- Consumer runtime only: Tools/rbk/*.sh (not rbgja/, rbgjb/, rbgjv/) and Tools/buk/*.sh
- sed stays (2 load-bearing uses, both earned)
- awk eviction (1 use in bul_launcher.sh) can ride along if trivial, but is not the focus

## Acceptance
- Zero grep invocations in consumer runtime .sh files (except grep -c if kept)
- All existing tests pass
- No behavioral change

### declare-rb-dependency-inventory-in-rbs0 (₢AUAAs) [complete]

**[260401-0859] complete**

## Character
Specification writing — document the concrete dependency choices RB has made. Pairs with BCG's universal rules.

## Context
BCG (₢AUAAg) codifies the universal dependency discipline: POSIX allowlist, eviction table, principles. This pace adds the RB-specific inventory to RBS0 — what Recipe Bottle specifically depends on, categorized by audience.

## Deliverable: RBS0 dependency inventory section

### Consumer dependencies (required for all users)
Documented in index.html, enforced by qualification:

bash, git, curl, jq, docker, openssl, ssh/scp/ssh-keygen

### Developer dependencies (not required for consumers)
Used in development/testing tooling only:

- **shellcheck** — linter, used by buq_qualify release tier
- **timeout** — GNU coreutils, used by rbtb_testbench; absent on stock macOS

### Specialized runtime
Present in specific operational modules, not required for core workflow:

- **podman** — container runtime (future: dual-runtime support alongside docker)
- **tcpdump** — network debugging in observe module
- **sed** — 2 load-bearing uses (log normalization, Podman output parsing)
- **stty** — terminal width detection in launcher

### Alignment with index.html
Verify index.html dependency claim matches this inventory. Update if diverged (openssl was previously undeclared).

## References
- BCG dependency discipline: ₢AUAAg
- index.html dependency claim (current)
- RBS0-SpecTop.adoc

## Acceptance
- RBS0 has a dependency inventory section with consumer/developer/specialized categories
- index.html updated to declare openssl
- Categories match the BCG framework (POSIX allowlist not repeated — referenced)

**[260331-1115] rough**

## Character
Specification writing — document the concrete dependency choices RB has made. Pairs with BCG's universal rules.

## Context
BCG (₢AUAAg) codifies the universal dependency discipline: POSIX allowlist, eviction table, principles. This pace adds the RB-specific inventory to RBS0 — what Recipe Bottle specifically depends on, categorized by audience.

## Deliverable: RBS0 dependency inventory section

### Consumer dependencies (required for all users)
Documented in index.html, enforced by qualification:

bash, git, curl, jq, docker, openssl, ssh/scp/ssh-keygen

### Developer dependencies (not required for consumers)
Used in development/testing tooling only:

- **shellcheck** — linter, used by buq_qualify release tier
- **timeout** — GNU coreutils, used by rbtb_testbench; absent on stock macOS

### Specialized runtime
Present in specific operational modules, not required for core workflow:

- **podman** — container runtime (future: dual-runtime support alongside docker)
- **tcpdump** — network debugging in observe module
- **sed** — 2 load-bearing uses (log normalization, Podman output parsing)
- **stty** — terminal width detection in launcher

### Alignment with index.html
Verify index.html dependency claim matches this inventory. Update if diverged (openssl was previously undeclared).

## References
- BCG dependency discipline: ₢AUAAg
- index.html dependency claim (current)
- RBS0-SpecTop.adoc

## Acceptance
- RBS0 has a dependency inventory section with consumer/developer/specialized categories
- index.html updated to declare openssl
- Categories match the BCG framework (POSIX allowlist not repeated — referenced)

### bcg-command-trust-tiers (₢AUAAg) [complete]

**[260401-0902] complete**

## Character
Design writing — codify tested principles into lasting BCG rules. The concrete audit work has been extracted into separate paces; this pace documents what we learned.

## Context
The dependency audit (₣AU grooming session, ₢AUAAg–₢AUAAo paces) produced a clear picture of the command dependency surface. BCG already pushes toward builtins (test over [[]], read over head, parameter expansion over dirname). This pace extends that discipline to cover external utility selection.

## Deliverable: BCG dependency discipline section

Add a section to BCG codifying:

### POSIX utility allowlist
These external commands are accepted in any BUK-based project. They have no bash 3.2 builtin replacement and are mandated by POSIX on any system that runs bash:

chmod, cp, date, find, mkdir, mktemp, mv, rm, sed, sleep, sort, stty

### Evicted utilities (with replacement patterns)
Document the builtins that replace these, so future contributors don't re-introduce them:

| Evicted | Replacement |
|---------|-------------|
| awk | `read` with IFS, parameter expansion |
| base64 | `openssl enc -base64` |
| cut | `read` with IFS, parameter expansion |
| grep | `case`, `test`, `[[ =~ ]]` (BCG-blessed) |
| head | `read -r` |
| ls | Glob expansion |
| sha256sum/shasum | `openssl dgst -sha256 -r` |
| tr | `${var//old/new}` parameter expansion |
| wc | `${#var}`, `${#arr[@]}` |

### Declared dependency principle
Anything beyond bash builtins and the POSIX allowlist must be explicitly declared in the project's consumer-facing documentation. Each declared dependency is a cost accepted by every consumer.

### Platform-variant command guidance
Commands that exist on both GNU and BSD but with incompatible flags (stat, base64, sha256sum) must be wrapped in a BUK function or replaced with a declared dependency that behaves consistently (e.g., openssl).

### LLM contributor note
LLMs reach for whatever solves the immediate problem without feeling the cumulative dependency cost. The POSIX allowlist and eviction table serve as a checkpoint — if a command isn't on either list, stop and justify it.

## References
- Eviction paces: ₢AUAAq (hashing), ₢AUAAr (base64), ₢AUAAp (trivial), ₢AUAAo (grep)
- awk eviction: ₢AUAAg notch (bul_launcher.sh)
- Existing BCG sections: Test vs Bracket Expressions, Eval Policy

## Acceptance
- BCG has a dependency discipline section with allowlist, eviction table, and principles
- No code changes — documentation only

**[260331-1114] rough**

## Character
Design writing — codify tested principles into lasting BCG rules. The concrete audit work has been extracted into separate paces; this pace documents what we learned.

## Context
The dependency audit (₣AU grooming session, ₢AUAAg–₢AUAAo paces) produced a clear picture of the command dependency surface. BCG already pushes toward builtins (test over [[]], read over head, parameter expansion over dirname). This pace extends that discipline to cover external utility selection.

## Deliverable: BCG dependency discipline section

Add a section to BCG codifying:

### POSIX utility allowlist
These external commands are accepted in any BUK-based project. They have no bash 3.2 builtin replacement and are mandated by POSIX on any system that runs bash:

chmod, cp, date, find, mkdir, mktemp, mv, rm, sed, sleep, sort, stty

### Evicted utilities (with replacement patterns)
Document the builtins that replace these, so future contributors don't re-introduce them:

| Evicted | Replacement |
|---------|-------------|
| awk | `read` with IFS, parameter expansion |
| base64 | `openssl enc -base64` |
| cut | `read` with IFS, parameter expansion |
| grep | `case`, `test`, `[[ =~ ]]` (BCG-blessed) |
| head | `read -r` |
| ls | Glob expansion |
| sha256sum/shasum | `openssl dgst -sha256 -r` |
| tr | `${var//old/new}` parameter expansion |
| wc | `${#var}`, `${#arr[@]}` |

### Declared dependency principle
Anything beyond bash builtins and the POSIX allowlist must be explicitly declared in the project's consumer-facing documentation. Each declared dependency is a cost accepted by every consumer.

### Platform-variant command guidance
Commands that exist on both GNU and BSD but with incompatible flags (stat, base64, sha256sum) must be wrapped in a BUK function or replaced with a declared dependency that behaves consistently (e.g., openssl).

### LLM contributor note
LLMs reach for whatever solves the immediate problem without feeling the cumulative dependency cost. The POSIX allowlist and eviction table serve as a checkpoint — if a command isn't on either list, stop and justify it.

## References
- Eviction paces: ₢AUAAq (hashing), ₢AUAAr (base64), ₢AUAAp (trivial), ₢AUAAo (grep)
- awk eviction: ₢AUAAg notch (bul_launcher.sh)
- Existing BCG sections: Test vs Bracket Expressions, Eval Policy

## Acceptance
- BCG has a dependency discipline section with allowlist, eviction table, and principles
- No code changes — documentation only

**[260314-2111] rough**

Add a command trust model section to BCG (Bash Console Guide) that codifies the dependency boundary.

## Context

During the MVP-3 release ceremony dry run, the Step 2 command audit surfaced commands used on the host workstation that aren't in the declared dependency list (index.html). The conversation that followed developed a trust framework that belongs in BCG as the authoritative reference.

## Trust Tiers

**Tier 1 — Bash builtins**: `test`, `read`, `printf`, `local`, `trap`, `set`, `readonly`, `shift`, `export`, `unset`, etc. These ARE the language. Cannot be hijacked via PATH. Use freely. BCG already pushes toward builtins (test over [[]], read over head -1, parameter expansion over dirname).

**Tier 2 — POSIX-mandated utilities**: `grep`, `sed`, `awk`, `cp`, `mv`, `rm`, `mkdir`, `cat`, `sort`, `cut`, `tr`, `find`, `diff`, `kill`, `sleep`, `date`, `uname`, `tar`, `tee`, `touch`, `ln`, `xargs`, `chmod`, `ls`, `stat`(check), `tput`, `stty`, `env`, `comm`, `paste`, `wc`, `head`, `tail`, `basename`, `dirname`. Trust the standard, not the implementation. But flag usage must be portable — GNU and BSD diverge on flags. Known divergences should be documented.

**Tier 3 — Dangerous middle**: Things that FEEL universal but aren't POSIX-mandated. This is where trust becomes assumption.
- `base64` — GNU coreutils + macOS, but not standardized. Flag divergence (`-w0` vs `-b0`).
- `timeout` — GNU coreutils only, absent on stock macOS.
- `sha256sum` / `shasum` — one or the other per platform, never both standardized.
- `stat` — exists everywhere, flags completely incompatible between GNU and BSD.
Pattern: wrap in a BUK function that detects and adapts, making the platform seam visible and tested.

**Tier 4 — Declared dependencies**: docker, git, curl, openssh, jq, and possibly openssl. Consumer must consciously install these. Documented in index.html. Each one accepted as a cost.

## Audit Findings to Resolve

- `openssl` used in rbgo_OAuth.sh (JWT signing) and bud_dispatch.sh (checksum fallback) — undeclared. Decide: add to index.html declared deps, or rework?
- `base64` used in 4 files — wrap for portability or accept as near-universal?
- `sha256sum` vs `shasum` split across files — needs a BUK portable wrapper.
- `timeout` used in testbench only — development-only, not runtime. Document as such.
- `shellcheck`, `python3` — development-only tools, not consumer deps.

## Ceremony Integration

The release ceremony Step 2 currently inlines its own tier lists. After BCG codifies the tiers, the ceremony should reference BCG rather than maintaining a separate copy.

## Observation

LLMs love adding dependencies — they reach for whatever solves the immediate problem without feeling the cumulative cost. The trust tier framework in BCG serves as a checkpoint for both human and LLM contributors.

### claudemd-include-restructure (₢AUAAl) [complete]

**[260328-0735] complete**

## Character
Mechanical refactoring with one judgment call per section: what stays in main CLAUDE.md vs what moves to a kit-owned include file.

## Goal
Replace the `<!-- MANAGED:KIT:BEGIN/END -->` patching mechanism with `@path` include directives, so each kit owns its CLAUDE.md section as a standalone file.

## Approach

1. **Extract kit sections** — Move each managed section to a kit-owned file:
   - `<!-- MANAGED:JJK -->` → `Tools/jjk/jjk-claude-context.md`
   - `<!-- MANAGED:CMK -->` → `Tools/cmk/cmk-claude-context.md`
   - `<!-- MANAGED:BUK -->` → `Tools/buk/buk-claude-context.md`
   - `<!-- MANAGED:VVK -->` → `Tools/vvk/vvk-claude-context.md`

2. **Migrate BUK vocabulary** — The "BUK Concepts" section in main CLAUDE.md (colophon, frontispiece, imprint, zipper, workbench, testbench definitions) is BUK knowledge that was manually authored outside the managed section. Move it into `buk-claude-context.md` so BUK owns all its context. This shrinks the main CLAUDE.md and ensures the vocabulary travels with the kit.

3. **Replace managed sections with `@` directives** in main CLAUDE.md:
   ```markdown
   @Tools/buk/buk-claude-context.md
   @Tools/cmk/cmk-claude-context.md
   @Tools/jjk/jjk-claude-context.md
   @Tools/vvk/vvk-claude-context.md
   ```

4. **Remove patching machinery** — Strip `zcmw_patch_claudemd()`, `zjjw_patch_claudemd()`, and related Claude-CLI-invocation code from kit install scripts. Install becomes "drop the file"; uninstall becomes "delete it."

5. **Retire dual-source obligation** — JJK's `vocjjmc_core.md` template and the "also update" comment become unnecessary. The include file IS the source of truth.

6. **Verify** — Start a fresh Claude Code session to confirm all `@` imports expand correctly. Check that `/cma-doctor` or equivalent can detect missing include targets.

## Design Decisions (pre-resolved)

- **No subsetting**: Kit contexts are either small enough to not matter (BUK, CMK, VVK) or tightly coupled enough that splitting creates failure modes (JJK). Ship one include file per kit.
- **README stays separate**: READMEs are conceptual references for humans; CLAUDE.md includes are operational protocols for Claude. No README content needs promotion to compulsory context.
- **Silent skip is acceptable**: A missing include file (uninstalled kit) silently contributes nothing — this is graceful degradation, not a bug.

## Constraints
- Total context should not grow — this is reorganization, not addition
- File names follow kit prefix discipline: `{kit}-claude-context.md`
- BUK vocabulary migration must not lose or alter any definitions

## Verification
- Fresh session loads all kit context correctly
- No `<!-- MANAGED:` markers remain in main CLAUDE.md
- Kit install/uninstall scripts no longer invoke Claude CLI for patching
- `git diff` shows content moved, not changed
- BUK vocabulary (colophon, frontispiece, imprint, etc.) appears in buk-claude-context.md, not main CLAUDE.md

**[260328-0716] rough**

## Character
Mechanical refactoring with one judgment call per section: what stays in main CLAUDE.md vs what moves to a kit-owned include file.

## Goal
Replace the `<!-- MANAGED:KIT:BEGIN/END -->` patching mechanism with `@path` include directives, so each kit owns its CLAUDE.md section as a standalone file.

## Approach

1. **Extract kit sections** — Move each managed section to a kit-owned file:
   - `<!-- MANAGED:JJK -->` → `Tools/jjk/jjk-claude-context.md`
   - `<!-- MANAGED:CMK -->` → `Tools/cmk/cmk-claude-context.md`
   - `<!-- MANAGED:BUK -->` → `Tools/buk/buk-claude-context.md`
   - `<!-- MANAGED:VVK -->` → `Tools/vvk/vvk-claude-context.md`

2. **Migrate BUK vocabulary** — The "BUK Concepts" section in main CLAUDE.md (colophon, frontispiece, imprint, zipper, workbench, testbench definitions) is BUK knowledge that was manually authored outside the managed section. Move it into `buk-claude-context.md` so BUK owns all its context. This shrinks the main CLAUDE.md and ensures the vocabulary travels with the kit.

3. **Replace managed sections with `@` directives** in main CLAUDE.md:
   ```markdown
   @Tools/buk/buk-claude-context.md
   @Tools/cmk/cmk-claude-context.md
   @Tools/jjk/jjk-claude-context.md
   @Tools/vvk/vvk-claude-context.md
   ```

4. **Remove patching machinery** — Strip `zcmw_patch_claudemd()`, `zjjw_patch_claudemd()`, and related Claude-CLI-invocation code from kit install scripts. Install becomes "drop the file"; uninstall becomes "delete it."

5. **Retire dual-source obligation** — JJK's `vocjjmc_core.md` template and the "also update" comment become unnecessary. The include file IS the source of truth.

6. **Verify** — Start a fresh Claude Code session to confirm all `@` imports expand correctly. Check that `/cma-doctor` or equivalent can detect missing include targets.

## Design Decisions (pre-resolved)

- **No subsetting**: Kit contexts are either small enough to not matter (BUK, CMK, VVK) or tightly coupled enough that splitting creates failure modes (JJK). Ship one include file per kit.
- **README stays separate**: READMEs are conceptual references for humans; CLAUDE.md includes are operational protocols for Claude. No README content needs promotion to compulsory context.
- **Silent skip is acceptable**: A missing include file (uninstalled kit) silently contributes nothing — this is graceful degradation, not a bug.

## Constraints
- Total context should not grow — this is reorganization, not addition
- File names follow kit prefix discipline: `{kit}-claude-context.md`
- BUK vocabulary migration must not lose or alter any definitions

## Verification
- Fresh session loads all kit context correctly
- No `<!-- MANAGED:` markers remain in main CLAUDE.md
- Kit install/uninstall scripts no longer invoke Claude CLI for patching
- `git diff` shows content moved, not changed
- BUK vocabulary (colophon, frontispiece, imprint, etc.) appears in buk-claude-context.md, not main CLAUDE.md

**[260328-0704] rough**

## Character
Mechanical refactoring with one judgment call per section: what stays in main CLAUDE.md vs what moves to a kit-owned include file.

## Goal
Replace the `<!-- MANAGED:KIT:BEGIN/END -->` patching mechanism with `@path` include directives, so each kit owns its CLAUDE.md section as a standalone file.

## Approach

1. **Extract kit sections** — Move each managed section to a kit-owned file:
   - `<!-- MANAGED:JJK -->` → `Tools/jjk/jjk-claude-context.md`
   - `<!-- MANAGED:CMK -->` → `Tools/cmk/cmk-claude-context.md`
   - `<!-- MANAGED:BUK -->` → `Tools/buk/buk-claude-context.md`
   - `<!-- MANAGED:VVK -->` → `Tools/vvk/vvk-claude-context.md`

2. **Replace managed sections with `@` directives** in main CLAUDE.md:
   ```markdown
   @Tools/buk/buk-claude-context.md
   @Tools/cmk/cmk-claude-context.md
   @Tools/jjk/jjk-claude-context.md
   @Tools/vvk/vvk-claude-context.md
   ```

3. **Remove patching machinery** — Strip `zcmw_patch_claudemd()`, `zjjw_patch_claudemd()`, and related Claude-CLI-invocation code from kit install scripts. Install becomes "drop the file"; uninstall becomes "delete it."

4. **Retire dual-source obligation** — JJK's `vocjjmc_core.md` template and the "also update" comment become unnecessary. The include file IS the source of truth.

5. **Verify** — Start a fresh Claude Code session to confirm all `@` imports expand correctly. Check that `/cma-doctor` or equivalent can detect missing include targets.

## Constraints
- Total context should not grow — this is reorganization, not addition
- Silent-skip behavior is acceptable for optional kits (graceful degradation)
- File names follow kit prefix discipline: `{kit}-claude-context.md`

## Verification
- Fresh session loads all kit context correctly
- No `<!-- MANAGED:` markers remain in main CLAUDE.md
- Kit install/uninstall scripts no longer invoke Claude CLI for patching
- `git diff` shows content moved, not changed

### slim-sentry-vessel (₢AUAAk) [abandoned]

**[260328-0716] abandoned**

## Character
Research-first, iterative. The sentry's job is narrow (iptables, dnsmasq, network namespace) but it currently wears a full ubuntu:24.04 suit. This pace explores whether a much leaner base image can pass all 22 nsproto security tests. Expect false starts — package availability, tool paths, and shell compatibility may differ across base images. Think hard, websearch, then try.

## Goal
Produce a sentry vessel that passes all nsproto security tests with a significantly smaller base image than ubuntu:24.04. Measure the build-time improvement.

## Approach
1. **Research first**: Inventory exactly which tools/packages the sentry startup script (`rbj_sentry.sh`) and nsproto test suite actually require. Websearch for minimal container images that ship iptables + dnsmasq + iproute2.
2. **Evaluate candidates**: Alpine, distroless, minimal debian, stripped ubuntu. Consider: package availability, shell compatibility (bash vs ash), tool path differences, iptables backend (nftables vs legacy).
3. **Prototype**: Create a new vessel definition (or modify existing). Build and run nsproto tests. Iterate on failures.
4. **Measure**: Compare conjure time and image size against the ubuntu:24.04 baseline (~13 min, reported in onboarding).

## Constraints
- All 22 nsproto security tests must pass — no test modifications to accommodate the new image
- The sentry Dockerfile and `rbj_sentry.sh` are the change surface — nsproto test infrastructure stays fixed
- If no candidate works cleanly, document why and keep ubuntu:24.04

## Verification
- `tt/rbw-tf.TestFixture.nsproto-security.sh` passes 22/22
- Conjure time measured and compared to baseline

**[260327-2148] rough**

## Character
Research-first, iterative. The sentry's job is narrow (iptables, dnsmasq, network namespace) but it currently wears a full ubuntu:24.04 suit. This pace explores whether a much leaner base image can pass all 22 nsproto security tests. Expect false starts — package availability, tool paths, and shell compatibility may differ across base images. Think hard, websearch, then try.

## Goal
Produce a sentry vessel that passes all nsproto security tests with a significantly smaller base image than ubuntu:24.04. Measure the build-time improvement.

## Approach
1. **Research first**: Inventory exactly which tools/packages the sentry startup script (`rbj_sentry.sh`) and nsproto test suite actually require. Websearch for minimal container images that ship iptables + dnsmasq + iproute2.
2. **Evaluate candidates**: Alpine, distroless, minimal debian, stripped ubuntu. Consider: package availability, shell compatibility (bash vs ash), tool path differences, iptables backend (nftables vs legacy).
3. **Prototype**: Create a new vessel definition (or modify existing). Build and run nsproto tests. Iterate on failures.
4. **Measure**: Compare conjure time and image size against the ubuntu:24.04 baseline (~13 min, reported in onboarding).

## Constraints
- All 22 nsproto security tests must pass — no test modifications to accommodate the new image
- The sentry Dockerfile and `rbj_sentry.sh` are the change surface — nsproto test infrastructure stays fixed
- If no candidate works cleanly, document why and keep ubuntu:24.04

## Verification
- `tt/rbw-tf.TestFixture.nsproto-security.sh` passes 22/22
- Conjure time measured and compared to baseline

### convert-crucible-tabtargets-imprint-to-param (₢AUAAu) [complete]

**[260401-0938] complete**

## Character

Mechanical — straightforward channel swap and tabtarget file consolidation.

## Docket

Convert three crucible tabtargets from imprint-style (one file per nameplate) to param-style (single file, nameplate as `$1`):

**Tabtargets to convert:**
- `rbw-ch.Hail` — currently `rbw-ch.Hail.{tadmor,pluml,srjcl}.sh`
- `rbw-cr.Rack` — currently `rbw-cr.Rack.{tadmor,srjcl}.sh`
- `rbw-cs.Scry` — currently `rbw-cs.Scry.{tadmor,pluml,srjcl}.sh`

**Changes required:**

1. **Zipper enrollment** (`rbz_zipper.sh`): Change channel from `"imprint"` to `"param1"` for the three colophons (`rbw-ch`, `rbw-cr`, `rbw-cs`)
2. **Create param-style tabtargets**: `rbw-ch.Hail.sh`, `rbw-cr.Rack.sh`, `rbw-cs.Scry.sh` (same launcher body, no imprint segment)
3. **Delete imprint-style tabtargets**: Remove the per-nameplate files (8 files total)
4. **Update docs**: Adjust any references in `CLAUDE.consumer.md` or `README.consumer.md` if they mention these specific tabtarget filenames

**[260401-0454] rough**

## Character

Mechanical — straightforward channel swap and tabtarget file consolidation.

## Docket

Convert three crucible tabtargets from imprint-style (one file per nameplate) to param-style (single file, nameplate as `$1`):

**Tabtargets to convert:**
- `rbw-ch.Hail` — currently `rbw-ch.Hail.{tadmor,pluml,srjcl}.sh`
- `rbw-cr.Rack` — currently `rbw-cr.Rack.{tadmor,srjcl}.sh`
- `rbw-cs.Scry` — currently `rbw-cs.Scry.{tadmor,pluml,srjcl}.sh`

**Changes required:**

1. **Zipper enrollment** (`rbz_zipper.sh`): Change channel from `"imprint"` to `"param1"` for the three colophons (`rbw-ch`, `rbw-cr`, `rbw-cs`)
2. **Create param-style tabtargets**: `rbw-ch.Hail.sh`, `rbw-cr.Rack.sh`, `rbw-cs.Scry.sh` (same launcher body, no imprint segment)
3. **Delete imprint-style tabtargets**: Remove the per-nameplate files (8 files total)
4. **Update docs**: Adjust any references in `CLAUDE.consumer.md` or `README.consumer.md` if they mention these specific tabtarget filenames

### spook-orient-default-heat-selection (₢AUAAj) [complete]

**[260401-0946] complete**

## Spook

When user says "mount" without a firemark, `jjx_orient` without a firemark defaults to the first racing heat — silently switching context away from the heat the chat was just working on. This happened in ₣Aw chat after wrapping ₢AwAAY: the next mount picked ₣Av instead of continuing ₣Aw.

## Fix

Update CLAUDE.md Mount Protocol (or orient guidance) so that:
1. `jjx_orient` without a firemark is never called blindly — the agent must provide the most recently mounted/groomed heat's firemark
2. If the chat has no prior heat context (fresh session), the agent asks the user which heat to mount rather than letting orient pick a default

This may be a CLAUDE.md-only fix (agent behavior guidance), or may warrant a code change to make parameterless orient return an error. Assess both options.

**[260327-1709] rough**

## Spook

When user says "mount" without a firemark, `jjx_orient` without a firemark defaults to the first racing heat — silently switching context away from the heat the chat was just working on. This happened in ₣Aw chat after wrapping ₢AwAAY: the next mount picked ₣Av instead of continuing ₣Aw.

## Fix

Update CLAUDE.md Mount Protocol (or orient guidance) so that:
1. `jjx_orient` without a firemark is never called blindly — the agent must provide the most recently mounted/groomed heat's firemark
2. If the chat has no prior heat context (fresh session), the agent asks the user which heat to mount rather than letting orient pick a default

This may be a CLAUDE.md-only fix (agent behavior guidance), or may warrant a code change to make parameterless orient return an error. Assess both options.

### bcg-builtin-redirect-vs-subshell (₢AUAA4) [complete]

**[260527-1053] complete**

## Character

Quick editorial clarification — mechanical, not architectural.

## Docket

BCG Command Substitution Rules currently say: "NO unguarded `$()` — every `$(cmd)` must have `|| buc_die`". This conflates two syntactically similar but semantically distinct forms:

- **`$(command)`** — spawns a subshell, runs a command, captures stdout. Can fail. Guard required.
- **`$(<file)`** — bash builtin file redirect. No subshell, no command. Reads file content directly.

Clarify in BCG that `$(<file)` is a builtin redirect exempt from the `|| buc_die` guard requirement. It is not a command substitution despite sharing the `$()` syntax. Add a note in the Command Substitution Rules section distinguishing the two forms.

**[260417-0659] rough**

## Character

Quick editorial clarification — mechanical, not architectural.

## Docket

BCG Command Substitution Rules currently say: "NO unguarded `$()` — every `$(cmd)` must have `|| buc_die`". This conflates two syntactically similar but semantically distinct forms:

- **`$(command)`** — spawns a subshell, runs a command, captures stdout. Can fail. Guard required.
- **`$(<file)`** — bash builtin file redirect. No subshell, no command. Reads file content directly.

Clarify in BCG that `$(<file)` is a builtin redirect exempt from the `|| buc_die` guard requirement. It is not a command substitution despite sharing the `$()` syntax. Add a note in the Command Substitution Rules section distinguishing the two forms.

### relocate-compose-probe-env (₢AUAA1) [complete]

**[260527-1753] complete**

## Character
Mechanical with some judgment — find the right home, move the file, update all references.

## Docket
Move `.rbk/rbje_compose_probe.env` out of the `.rbk/` runtime config directory into somewhere inside `Tools/rbk/`. This file is a sentinel/test fixture (compose env-file quoting probe), not a regime assignment — it doesn't belong alongside `rbrp.env` and `rbrr.env`.

Precise destination TBD during execution. Candidates: `Tools/rbk/` root, a test-fixtures subdirectory, or alongside sentry code that reads it. Need to trace all references (compose files, sentry startup, any sourcing) and update paths.

**[260413-0912] rough**

## Character
Mechanical with some judgment — find the right home, move the file, update all references.

## Docket
Move `.rbk/rbje_compose_probe.env` out of the `.rbk/` runtime config directory into somewhere inside `Tools/rbk/`. This file is a sentinel/test fixture (compose env-file quoting probe), not a regime assignment — it doesn't belong alongside `rbrp.env` and `rbrr.env`.

Precise destination TBD during execution. Candidates: `Tools/rbk/` root, a test-fixtures subdirectory, or alongside sentry code that reads it. Need to trace all references (compose files, sentry startup, any sourcing) and update paths.

### regime-probate-homogeneity (₢AUAAh) [complete]

**[260530-1006] complete**

## Character

Uniform mechanical replication across regimes, gated on one test-design rule
and a few flagged per-regime wrinkles.

## Goal

Make every file-based regime uniformly *provable*: each gains a
`«prefix»_probate` test seam and a matching theurge fast-fixture case pair, so a
user's misconfiguration of *any* regime is caught with the same actionable
enforce verdict. Reifies "a regime" as "a typed config with a kindle→enforce
gate you can put on trial."

## Scope

The file-based regimes lacking a probate: `rbrp`, `rbrs`, `rbro`, `rbra`
(`Tools/rbk/*_regime.sh`) and `burc`, `burn`, `burp`, `burs`
(`Tools/buk/*_regime.sh`). `rbrr`/`rbrd`/`rbrn`/`rbrv` are the reference pattern.
Sideways file-less regimes (`bure` ambient, `burd` dispatch) are out — their
failures are not user-config failures. Credential regimes (`rbro`/`rbra`) are in:
a malformed installed credential is a real user-facing failure; synthetic
non-secret baselines suffice.

## The two parts

- *Bash:* add a `«prefix»_probate` to each, mirroring `rbrr_probate` — source the
  handed file, kindle, enforce. `rbrp` must additionally kindle RBGC (its enforce
  depends on the payor-project regex); `rbro`/`rbra`/`burp` take synthetic
  non-secret baselines (`rbra` private key needs PEM `BEGIN` material).
- *Rust (theurge fast fixture):* per regime, a baseline-valid case (all PASS) and
  one seeded-violation case (verdict-only, binary). Aim the violation at the
  regime's custom enforce check where one exists (`rbrd` joint-length, `rbrp`/
  `rbra` format regexes, `burs` tincture), else an unset required field.

## Out of scope

`render` (already structured and actionable via `buv_render`); the lock-step
asymmetry (only BURC locks enrolled vars) and the BUK-vs-RBK manifold-location
asymmetry — flag if encountered, do not churn.

## Done

Every file-based regime has a probate and a baseline+violation case pair in the
fast fixture; `rbw-tt` unit tests and the fast suite green.

## References

- `Tools/rbk/rbrr_regime.sh` — reference probate (`rbrr_probate`) and its
  test-seam doc comment
- `Tools/rbk/rbtd/src/rbtdrf_fast.rs` — theurge fast fixture; `rbtdrf_run_probate`
  harness and the existing `rv_` cases
- `Tools/buk/buv_validation.sh` — `buv_report`/`buv_vet` engine (the
  `FAIL VARNAME` contract)
- BCG "Regime Module Archetype" — the kindle→enforce→lock pattern

**[260529-0857] rough**

## Character

Uniform mechanical replication across regimes, gated on one test-design rule
and a few flagged per-regime wrinkles.

## Goal

Make every file-based regime uniformly *provable*: each gains a
`«prefix»_probate` test seam and a matching theurge fast-fixture case pair, so a
user's misconfiguration of *any* regime is caught with the same actionable
enforce verdict. Reifies "a regime" as "a typed config with a kindle→enforce
gate you can put on trial."

## Scope

The file-based regimes lacking a probate: `rbrp`, `rbrs`, `rbro`, `rbra`
(`Tools/rbk/*_regime.sh`) and `burc`, `burn`, `burp`, `burs`
(`Tools/buk/*_regime.sh`). `rbrr`/`rbrd`/`rbrn`/`rbrv` are the reference pattern.
Sideways file-less regimes (`bure` ambient, `burd` dispatch) are out — their
failures are not user-config failures. Credential regimes (`rbro`/`rbra`) are in:
a malformed installed credential is a real user-facing failure; synthetic
non-secret baselines suffice.

## The two parts

- *Bash:* add a `«prefix»_probate` to each, mirroring `rbrr_probate` — source the
  handed file, kindle, enforce. `rbrp` must additionally kindle RBGC (its enforce
  depends on the payor-project regex); `rbro`/`rbra`/`burp` take synthetic
  non-secret baselines (`rbra` private key needs PEM `BEGIN` material).
- *Rust (theurge fast fixture):* per regime, a baseline-valid case (all PASS) and
  one seeded-violation case (verdict-only, binary). Aim the violation at the
  regime's custom enforce check where one exists (`rbrd` joint-length, `rbrp`/
  `rbra` format regexes, `burs` tincture), else an unset required field.

## Out of scope

`render` (already structured and actionable via `buv_render`); the lock-step
asymmetry (only BURC locks enrolled vars) and the BUK-vs-RBK manifold-location
asymmetry — flag if encountered, do not churn.

## Done

Every file-based regime has a probate and a baseline+violation case pair in the
fast fixture; `rbw-tt` unit tests and the fast suite green.

## References

- `Tools/rbk/rbrr_regime.sh` — reference probate (`rbrr_probate`) and its
  test-seam doc comment
- `Tools/rbk/rbtd/src/rbtdrf_fast.rs` — theurge fast fixture; `rbtdrf_run_probate`
  harness and the existing `rv_` cases
- `Tools/buk/buv_validation.sh` — `buv_report`/`buv_vet` engine (the
  `FAIL VARNAME` contract)
- BCG "Regime Module Archetype" — the kindle→enforce→lock pattern

**[260529-0857] rough**

## Character

Mostly mechanical fixture extension, gated on one design judgment (below).

## Status correction (verified 260527)

Deliverable A — the actionable-failure repair — is **already satisfied** by
prior heat work; the docket's original "confirmed live crash" premise is stale.
Verified: the OAuth render fails actionably when credentials are absent
("RBRO credentials missing (…) - run rbgp_payor_install", exit 1), reproduced
by renaming the payor credential file aside and restoring it. Survey of every
render/validate/info shows each either delegates to the safe `buv_render`/
`buv_report` engine (which uses `${!var:-}` indirect expansion) or carries a
furnish-level `buc_die`/sentinel guard. No raw `set -u` death path remains.
Re-confirm with a spot reproduction before assuming, but expect no repair work.

## The work — fixture coverage (was Deliverable C)

The fast theurge fixture's regime-smoke set omits several renders/infos. Add the
omitted ones so every render/validate/info is exercised by the fast fixture.

Discovery recipe for the gap: list the `rbtdrf_rs_*` cases registered in the
regime-smoke `case!` block against the full set of `*_render`/`*_validate`/
`*_info` definitions under `Tools/buk` + `Tools/rbk`. The difference is the
work — expect the credential-gated (OAuth), folio-gated (auth roles), station,
depot-render, the BUK node/privilege/peer regimes, and the bottle info path.

## Locked design question — resolve before coding the gated cases

Credential-gated renders (OAuth) exit 0 only when the credential is present, so
a clean-exit assertion passes on a configured station but fails in a
credential-free context. The fast suite is the no-dependencies tier. Decide per
gated render: stage the prerequisite, or assert the actionable-failure path
(the fixture already has a negative-exit helper). This is the judgment the pace
turns on — do not silently assume creds are always present.

## Done

Every regime render/validate/info is invoked by the fast fixture; each gated
render has a deliberate, documented assertion stance (clean-exit-when-staged or
actionable-failure-when-absent).

## References

- `Tools/rbk/rbtd/src/rbtdrf_fast.rs` — fixture to extend (regime-smoke `case!` block)
- `Tools/buk/buv_validation.sh` — safe `buv_render` engine
- `Tools/rbk/rbro_cli.sh`, `rbro_regime.sh` — the already-actionable OAuth path (reference pattern)

**[260527-1829] rough**

## Character

Mostly mechanical fixture extension, gated on one design judgment (below).

## Status correction (verified 260527)

Deliverable A — the actionable-failure repair — is **already satisfied** by
prior heat work; the docket's original "confirmed live crash" premise is stale.
Verified: the OAuth render fails actionably when credentials are absent
("RBRO credentials missing (…) - run rbgp_payor_install", exit 1), reproduced
by renaming the payor credential file aside and restoring it. Survey of every
render/validate/info shows each either delegates to the safe `buv_render`/
`buv_report` engine (which uses `${!var:-}` indirect expansion) or carries a
furnish-level `buc_die`/sentinel guard. No raw `set -u` death path remains.
Re-confirm with a spot reproduction before assuming, but expect no repair work.

## The work — fixture coverage (was Deliverable C)

The fast theurge fixture's regime-smoke set omits several renders/infos. Add the
omitted ones so every render/validate/info is exercised by the fast fixture.

Discovery recipe for the gap: list the `rbtdrf_rs_*` cases registered in the
regime-smoke `case!` block against the full set of `*_render`/`*_validate`/
`*_info` definitions under `Tools/buk` + `Tools/rbk`. The difference is the
work — expect the credential-gated (OAuth), folio-gated (auth roles), station,
depot-render, the BUK node/privilege/peer regimes, and the bottle info path.

## Locked design question — resolve before coding the gated cases

Credential-gated renders (OAuth) exit 0 only when the credential is present, so
a clean-exit assertion passes on a configured station but fails in a
credential-free context. The fast suite is the no-dependencies tier. Decide per
gated render: stage the prerequisite, or assert the actionable-failure path
(the fixture already has a negative-exit helper). This is the judgment the pace
turns on — do not silently assume creds are always present.

## Done

Every regime render/validate/info is invoked by the fast fixture; each gated
render has a deliberate, documented assertion stance (clean-exit-when-staged or
actionable-failure-when-absent).

## References

- `Tools/rbk/rbtd/src/rbtdrf_fast.rs` — fixture to extend (regime-smoke `case!` block)
- `Tools/buk/buv_validation.sh` — safe `buv_render` engine
- `Tools/rbk/rbro_cli.sh`, `rbro_regime.sh` — the already-actionable OAuth path (reference pattern)

**[260527-1749] rough**

## Character

Mechanical — defensive guards plus fixture-coverage extension. One locked
judgment (below); the rest is systematic.

## Problem

A prerelease sweep that calls every regime render/validate/info hits crashes,
not actionable errors. Confirmed live: credential-gated renders (the OAuth
render in `rbro_cli.sh`) expand a secret's length under `set -u`; when creds
aren't loaded the variable is unbound and bash dies raw instead of saying "not
loaded — run the install step." The bottle info path (`rbob_info` in
`rbob_cli.sh`) echoes kindle-computed vars with no unkindled guard — same
failure class.

## Deliverable A — repair

Make each render/validate/info that can be called without its prerequisite
state fail with a message naming the missing input and the step that supplies
it. Survey the credential-gated and folio-gated renders for the same hazard
(the auth and depot regimes are likely peers of the OAuth case).

**Locked decision:** credential-gated renders emit an actionable "not loaded,
run «step»" sentinel — NOT a `${VAR:-}` soft-empty that prints "0 chars."
The actionable message is the whole point of the pace.

## Deliverable B — RETIRED

The old "quiet logging in sweep context" deliverable assumed a standalone bash
sweep script. The sweep now runs through the Rust theurge harness, which
captures tabtarget output; no separate logging discipline is needed.

## Deliverable C — extend the existing fixture

The fast theurge fixture (`rbtdrf_fast.rs`) already sweeps render+validate for
several regimes via a render/validate helper (folio-gated ones via per-folio
loops). It omits exactly the renders that currently crash. After repair, add
the omitted renders to that sweep so every render/info is exercised and
asserted clean-exit.

Discovery recipe for the gap: diff the set of `*_render()` / `*_info()`
definitions under `Tools/buk` + `Tools/rbk` against the renders invoked in
`rbtdrf_fast.rs`. The difference is the work — expect the credential- and
folio-gated renders plus the newer node/privilege/depot regimes.

## Done

Every regime render/validate/info is invoked by the fast fixture and exits
clean; the credential- and folio-gated renders print actionable messages when
their prerequisites are absent.

## References

- `Tools/rbk/rbro_cli.sh`, `Tools/rbk/rbob_cli.sh` — confirmed crash sites
- `Tools/rbk/rbtd/src/rbtdrf_fast.rs` — existing render sweep to extend
- `Tools/buk/buv_validation.sh` — core `buv_render()` engine

**[260315-1146] rough**

## Character
Mechanical with judgment on logging — systematic repair and smoke test.

## Problem
Calling every regime render/info function as a prerelease check currently fails. Many functions die with unhelpful errors ("Module X not kindled", unbound variable crashes) instead of actionable messages. Some renders produce verbose logging inappropriate for a sweep context.

## Deliverable A: Repair error messages across all regime renders

Fix every render/validate/info function so that "failure to have parameter" messages are helpful and correct.

### Specific failures to repair

**Folio-gated renders** (`rbrn_render`, `rbra_render`, `rbrv_render`):
- Die with terse messages when `BUZ_FOLIO` is empty
- Should list available options and name the required parameter clearly

**`rbro_render`** (Tools/rbk/rbro_cli.sh):
- Crashes with `set -u` unbound variable (`${#RBRO_CLIENT_SECRET}`) when credentials aren't loaded
- Custom render (doesn't call `buv_render`) — needs defensive guards

**`rbob_info`** (Tools/rbk/rbob_cli.sh):
- Crashes on unbound `ZRBOB_*` variables if module not kindled
- Sentinel check exists but inner vars have no guards

**Sentinel messages generally:**
- "Module X not kindled - call zX_kindle first" assumes internal knowledge
- Consider whether these need user-facing improvement or are only hit in sweep context

## Deliverable B: Logging discipline for sweep context

Several regime renders produce logging output that is not appropriate when running as part of a batch sweep. Identify which renders log excessively and add a mechanism to suppress or quiet logging during sweep execution (e.g., a BURE_VERBOSE=0 guard, redirect, or sweep-mode flag). Not all renders need this — use judgment per function.

## Deliverable C: Sweep test fixture

Add a test fixture that sources every regime file and calls every render/info function, verifying zero exit status on each.

### Functions to exercise (11 + 1)

**BUK regime renders (3):**
- `burc_render()` — Tools/buk/burc_cli.sh
- `burs_render()` — Tools/buk/burs_cli.sh
- `bure_render()` — Tools/buk/bure_cli.sh

**RBK regime renders (8):**
- `rbra_render()` — Tools/rbk/rbra_cli.sh
- `rbrp_render()` — Tools/rbk/rbrp_cli.sh
- `rbrs_render()` — Tools/rbk/rbrs_cli.sh
- `rbro_render()` — Tools/rbk/rbro_cli.sh
- `rbrg_render()` — Tools/rbk/rbrg_cli.sh
- `rbrn_render()` — Tools/rbk/rbrn_cli.sh
- `rbrr_render()` — Tools/rbk/rbrr_cli.sh
- `rbrv_render()` — Tools/rbk/rbrv_cli.sh

**Other:**
- `rbob_info()` — Tools/rbk/rbob_cli.sh

### Implementation
- Standalone test fixture (e.g., `rbtcrs_RegimeSweep.sh`) runnable from a tabtarget
- Each call must exit 0; any failure = test failure
- Consider integrating as a step in `rbq_qualify_release()`

### References
- Tools/buk/buv_validation.sh — core `buv_render()` engine
- Tools/rbk/rbq_Qualify.sh — qualification orchestrator
- Existing test fixture pattern: rbtcrv_RegimeValidation.sh

**[260315-0944] rough**

## Character
Mechanical — source-and-call smoke test.

## Deliverable
Add a test fixture that sources every regime file and calls every `*_render()` function plus `rbob_info()`, verifying zero exit status on each.

### Functions to exercise (11 + 1)

**BUK regime renders (3):**
- `burc_render()` — Tools/buk/burc_cli.sh
- `burs_render()` — Tools/buk/burs_cli.sh
- `bure_render()` — Tools/buk/bure_cli.sh

**RBK regime renders (8):**
- `rbra_render()` — Tools/rbk/rbra_cli.sh
- `rbrp_render()` — Tools/rbk/rbrp_cli.sh
- `rbrs_render()` — Tools/rbk/rbrs_cli.sh
- `rbro_render()` — Tools/rbk/rbro_cli.sh
- `rbrg_render()` — Tools/rbk/rbrg_cli.sh
- `rbrn_render()` — Tools/rbk/rbrn_cli.sh
- `rbrr_render()` — Tools/rbk/rbrr_cli.sh
- `rbrv_render()` — Tools/rbk/rbrv_cli.sh

**Other:**
- `rbob_info()` — Tools/rbk/rbob_cli.sh

### Implementation
- Standalone test fixture (e.g., `rbtcrs_RegimeSweep.sh`) runnable from a tabtarget
- Each call must exit 0; any failure = test failure
- Validates regime enrollment files are syntactically complete and render functions execute against current regime state
- Consider integrating as a step in `rbq_qualify_release()` so the prep-release ceremony invokes it automatically

### References
- Tools/buk/buv_validation.sh — core `buv_render()` engine
- Tools/rbk/rbq_Qualify.sh — qualification orchestrator
- Existing test fixture pattern: rbtcrv_RegimeValidation.sh

### factor-openssl-base64-helpers (₢AUAA6) [complete]

**[260527-1810] complete**

## Character
Implemented and committed; holding open for one gated validation before wrap.

## Status
The factoring is landed — see commit 139838ac5 for the design (four stateless
base64 primitives in `rbgo_OAuth.sh`, six call sites rewired) and the rationale
for homing them in rbgo rather than rbgu. Shellcheck clean.

Four of the six sites are validated live: the skirmish `canonical-invest`
fixture exercises the two string→file decode sites (governor mantle in
`rbgp_Payor.sh`, retriever/director invest in `rbgg_Governor.sh`) and both JWT
base64url encode primitives (every token auth), all passing against live GCP.

## Remaining before wrap
The two cloud-build decode sites in `rbfd_FoundryDirectorBuild.sh` (conjure
hallmark-consistency and enshrine output) are not yet exercised. They run only
behind the workbench fast-qualify gate that fronts `rbw-fO` and `rbw-cC`
(`rbw_workbench.sh`), which currently fails on residual shellcheck findings
owned by heat ₣BT. The conjure aborts at that gate before reaching the base64
line.

Once ₣BT's fast-qualify gate is clean, run
`tt/rbw-tf.FixtureRun.sh onboarding-sequence`: it reaches the conjure case
(rbfd conjure decode) and, continuing, the airgap chain (rbfd enshrine decode).
`onboarding-sequence` is StateProgressing — it self-conjures and halts on first
failure, so it lands exactly on the conjure without a full skirmish.

## Done
All six sites exercised clean — the four already validated, plus the two rbfd
decode sites via a green `onboarding-sequence` conjure + airgap run.

**[260527-1140] rough**

## Character
Implemented and committed; holding open for one gated validation before wrap.

## Status
The factoring is landed — see commit 139838ac5 for the design (four stateless
base64 primitives in `rbgo_OAuth.sh`, six call sites rewired) and the rationale
for homing them in rbgo rather than rbgu. Shellcheck clean.

Four of the six sites are validated live: the skirmish `canonical-invest`
fixture exercises the two string→file decode sites (governor mantle in
`rbgp_Payor.sh`, retriever/director invest in `rbgg_Governor.sh`) and both JWT
base64url encode primitives (every token auth), all passing against live GCP.

## Remaining before wrap
The two cloud-build decode sites in `rbfd_FoundryDirectorBuild.sh` (conjure
hallmark-consistency and enshrine output) are not yet exercised. They run only
behind the workbench fast-qualify gate that fronts `rbw-fO` and `rbw-cC`
(`rbw_workbench.sh`), which currently fails on residual shellcheck findings
owned by heat ₣BT. The conjure aborts at that gate before reaching the base64
line.

Once ₣BT's fast-qualify gate is clean, run
`tt/rbw-tf.FixtureRun.sh onboarding-sequence`: it reaches the conjure case
(rbfd conjure decode) and, continuing, the airgap chain (rbfd enshrine decode).
`onboarding-sequence` is StateProgressing — it self-conjures and halts on first
failure, so it lands exactly on the conjure without a full skirmish.

## Done
All six sites exercised clean — the four already validated, plus the two rbfd
decode sites via a green `onboarding-sequence` conjure + airgap run.

**[260427-1319] rough**

## Character
Deferred decision — slate now, judge later.

## Problem
`openssl enc -base64 ... -A` duplicated across 6 sites with no load-bearing variation. AUAAr cycle 4 showed the cost: missed `-A` on one site during standardization, dormant ~26 days, surfaces only on live API path.

## Sites
- Decode: `rbgg_Governor.sh:242`, `rbgp_Payor.sh:1355`, `rbfd_FoundryDirectorBuild.sh:959`, `rbfd_FoundryDirectorBuild.sh:1187`
- Encode: `rbgo_OAuth.sh:69`, `rbgo_OAuth.sh:131`

## Proposal
Thin helpers in `rbgu_Utility.sh`:
- `zrbgu_base64_decode_file_to_file(input, output)`
- `zrbgu_base64_decode_string_to_file(b64, output)`
- Matching encode pair; `rbgo_OAuth.sh:69` is legitimately a `_capture` (returns string).

Not `_capture` for the decode pair — output is a file, not a stdout value.

## Done
All 6 sites call helpers; `openssl enc -base64` lives only in `rbgu_Utility.sh`. OR: closed won't-fix with rationale recorded.

## Reference
Commit f3cd71c4 (AUAAr cycle 4 fault analysis).

### bind-mirror-syft-via-container (₢AUAAb) [complete]

**[260313-1925] complete**

Run syft SBOM scan in rbf_mirror using the digest-pinned syft container image (RBRG_SYFT_IMAGE_REF) via docker run, matching the conjure pipeline pattern from rbgjb06. Replace the local command -v syft check with containerized syft invocation against the mirrored GAR image.

**[260313-1907] rough**

Run syft SBOM scan in rbf_mirror using the digest-pinned syft container image (RBRG_SYFT_IMAGE_REF) via docker run, matching the conjure pipeline pattern from rbgjb06. Replace the local command -v syft check with containerized syft invocation against the mirrored GAR image.

### enrich-about-with-build-provenance (₢AUAAS) [complete]

**[260313-1441] complete**

Enrich the -about artifact with BuildKit build metadata and Docker cache inventory to enable base image provenance in inspect.

## New files in -about

- `buildkit_metadata.json` — output of `docker buildx build --metadata-file`, containing BuildKit's own record of resolved base image references, digests, and build parameters. Authoritative source for what images the build consumed.
- `cache_before.json` — Docker daemon + buildx builder cache snapshot taken before the build starts. Expected to be empty since rbgjb03 creates a fresh buildx builder each time, but captured for diagnostic confirmation.
- `cache_after.json` — Docker daemon + buildx builder cache snapshot taken after the build completes. Shows everything pulled/built for this consecration.

## Cache hygiene

The buildx builder is already created fresh in rbgjb03 (`docker buildx create` with no reuse guard), so its cache starts empty. The host Docker daemon may have residual images on shared private worker pools. Add `docker system prune -af` (host daemon) and `docker buildx prune -af` (buildx cache) early in the pipeline to ensure both caches are clean. This makes the before/after diff authoritative — everything in cache_after was pulled for THIS build.

## Build pipeline changes

1. New early step (before rbgjb03): prune host daemon (`docker system prune -af`) and snapshot `cache_before.json` (host `docker images` + buildx state)
2. Modify rbgjb03 (buildx build): add `--metadata-file /workspace/buildkit_metadata.json` to the `docker buildx build` command
3. New post-build step (after rbgjb05): snapshot `cache_after.json` (host `docker images` + buildx state)
4. Modify rbgjb08 (about assembly): add COPY lines for the three new files into Dockerfile.meta — these are platform-independent, so COPY without TARGETARCH variables (unlike sbom/build_info which are per-platform)

Note: rbgjb03 already force-creates the builder (no || reuse guard). rbgjb08 has a || reuse guard for the builder but that is for the -about container build, which is a separate concern.

Note: `docker images` queries the host Docker daemon. The buildx builder uses a docker-container driver with its own internal cache, which is separate. Cache snapshots should capture both: host daemon state via `docker images --format json` and buildx builder state via available buildx inspection commands.

## Inspect integration

Update `zrbf_inspect_show_sections` to display base image provenance from buildkit_metadata.json. Update `zrbf_inspect_show_full` to show cache diff if available. The buildkit_metadata.json is the primary source for base image identity; cache snapshots are supporting diagnostics.

## Acceptance

- buildkit_metadata.json present in -about with resolved base image references
- cache_before.json captured (expected empty or near-empty after prune)
- cache_after.json shows images pulled for this build
- Inspect displays base image identity from buildkit_metadata.json without Dockerfile parsing
- New files are platform-independent (same content across all -about platform variants)

**[260313-1347] rough**

Enrich the -about artifact with BuildKit build metadata and Docker cache inventory to enable base image provenance in inspect.

## New files in -about

- `buildkit_metadata.json` — output of `docker buildx build --metadata-file`, containing BuildKit's own record of resolved base image references, digests, and build parameters. Authoritative source for what images the build consumed.
- `cache_before.json` — Docker daemon + buildx builder cache snapshot taken before the build starts. Expected to be empty since rbgjb03 creates a fresh buildx builder each time, but captured for diagnostic confirmation.
- `cache_after.json` — Docker daemon + buildx builder cache snapshot taken after the build completes. Shows everything pulled/built for this consecration.

## Cache hygiene

The buildx builder is already created fresh in rbgjb03 (`docker buildx create` with no reuse guard), so its cache starts empty. The host Docker daemon may have residual images on shared private worker pools. Add `docker system prune -af` (host daemon) and `docker buildx prune -af` (buildx cache) early in the pipeline to ensure both caches are clean. This makes the before/after diff authoritative — everything in cache_after was pulled for THIS build.

## Build pipeline changes

1. New early step (before rbgjb03): prune host daemon (`docker system prune -af`) and snapshot `cache_before.json` (host `docker images` + buildx state)
2. Modify rbgjb03 (buildx build): add `--metadata-file /workspace/buildkit_metadata.json` to the `docker buildx build` command
3. New post-build step (after rbgjb05): snapshot `cache_after.json` (host `docker images` + buildx state)
4. Modify rbgjb08 (about assembly): add COPY lines for the three new files into Dockerfile.meta — these are platform-independent, so COPY without TARGETARCH variables (unlike sbom/build_info which are per-platform)

Note: rbgjb03 already force-creates the builder (no || reuse guard). rbgjb08 has a || reuse guard for the builder but that is for the -about container build, which is a separate concern.

Note: `docker images` queries the host Docker daemon. The buildx builder uses a docker-container driver with its own internal cache, which is separate. Cache snapshots should capture both: host daemon state via `docker images --format json` and buildx builder state via available buildx inspection commands.

## Inspect integration

Update `zrbf_inspect_show_sections` to display base image provenance from buildkit_metadata.json. Update `zrbf_inspect_show_full` to show cache diff if available. The buildkit_metadata.json is the primary source for base image identity; cache snapshots are supporting diagnostics.

## Acceptance

- buildkit_metadata.json present in -about with resolved base image references
- cache_before.json captured (expected empty or near-empty after prune)
- cache_after.json shows images pulled for this build
- Inspect displays base image identity from buildkit_metadata.json without Dockerfile parsing
- New files are platform-independent (same content across all -about platform variants)

**[260313-1344] rough**

Enrich the -about artifact with BuildKit build metadata and Docker cache inventory to enable base image provenance in inspect.

## New files in -about

- `buildkit_metadata.json` — output of `docker buildx build --metadata-file`, containing BuildKit's own record of resolved base image references, digests, and build parameters. Authoritative source for what images the build consumed.
- `cache_before.json` — `docker images --format json` snapshot taken before the build starts.
- `cache_after.json` — `docker images --format json` snapshot taken after the build completes.

## Cache hygiene

Clear Docker daemon cache (`docker system prune -af`) and remove/recreate the buildx builder (`docker buildx rm rb-builder`) at the start of each build. This makes the before/after cache diff authoritative rather than diagnostic — everything new in cache_after was pulled for THIS build. Eliminates cross-build leakage of other vessels' image references on shared private worker pools.

## Build pipeline changes

1. New early step: prune Docker cache and snapshot `cache_before.json`
2. Modify rbgjb05 (buildx build): add `--metadata-file /workspace/buildkit_metadata.json`
3. Modify rbgjb03 or equivalent: force-recreate buildx builder instead of reuse
4. New post-build step: snapshot `cache_after.json`
5. Modify rbgjb08 (about assembly): COPY the three new files into -about Dockerfile

## Inspect integration

Update `zrbf_inspect_show_sections` to display base image provenance from buildkit_metadata.json. Update `zrbf_inspect_show_full` to show cache diff if available.

## Acceptance

- buildkit_metadata.json present in -about with resolved base image references
- cache_before.json empty (or near-empty) after prune
- cache_after.json shows only images pulled for this build
- Inspect displays base image identity without Dockerfile parsing
- No cross-build information leakage between vessels

### move-rbs-specs-to-rbk-veiled (₢AUAAD) [abandoned]

**[260308-1229] abandoned**

Move all 37 RBS* specification files from legacy `lenses/` to `Tools/rbk/vov_veiled/`.

## Prerequisite
₢AUAAC (delete-rbk-coordinator) must complete first to free the `rbk` prefix.

## Files to move (37 total)
- `RBS-Specification.adoc` (main spec)
- `RBS0-SpecTop.adoc`
- 35 subdocuments: RBSAA, RBSAX, RBSBC, RBSBK, RBSBL, RBSBR, RBSBS, RBSCE, RBSCO, RBSDC, RBSDD, RBSDI, RBSDL, RBSDS, RBSGR, RBSGS, RBSID, RBSIL, RBSIP, RBSIR, RBSNC, RBSNX, RBSOB, RBSPE, RBSPI, RBSPR, RBSPT, RBSRC, RBSRV, RBSSC, RBSSD, RBSSL, RBSSS, RBSTB, RBSVC, RBSVM

## Tasks
1. Create `Tools/rbk/vov_veiled/` directory
2. `git mv` all 37 RBS* files from `lenses/` to `Tools/rbk/vov_veiled/`
3. Update CLAUDE.md acronym mappings for all RBS* entries (new paths)
4. Grep for any cross-references from remaining `lenses/` files to RBS* files and update paths
5. Verify `lenses/` retains only non-RBS files (CRR, RBRN, RBRR, RBWMBX, axl-AXMCM)

## Verification
- All 37 files exist under `Tools/rbk/vov_veiled/`
- No RBS* files remain in `lenses/`
- CLAUDE.md mappings point to new paths
- No broken cross-references

Result: RBS concept models follow the veiled pattern. `Tools/rbk/` established as Recipe Bottle Kit directory.

**[260209-0555] rough**

Move all 37 RBS* specification files from legacy `lenses/` to `Tools/rbk/vov_veiled/`.

## Prerequisite
₢AUAAC (delete-rbk-coordinator) must complete first to free the `rbk` prefix.

## Files to move (37 total)
- `RBS-Specification.adoc` (main spec)
- `RBS0-SpecTop.adoc`
- 35 subdocuments: RBSAA, RBSAX, RBSBC, RBSBK, RBSBL, RBSBR, RBSBS, RBSCE, RBSCO, RBSDC, RBSDD, RBSDI, RBSDL, RBSDS, RBSGR, RBSGS, RBSID, RBSIL, RBSIP, RBSIR, RBSNC, RBSNX, RBSOB, RBSPE, RBSPI, RBSPR, RBSPT, RBSRC, RBSRV, RBSSC, RBSSD, RBSSL, RBSSS, RBSTB, RBSVC, RBSVM

## Tasks
1. Create `Tools/rbk/vov_veiled/` directory
2. `git mv` all 37 RBS* files from `lenses/` to `Tools/rbk/vov_veiled/`
3. Update CLAUDE.md acronym mappings for all RBS* entries (new paths)
4. Grep for any cross-references from remaining `lenses/` files to RBS* files and update paths
5. Verify `lenses/` retains only non-RBS files (CRR, RBRN, RBRR, RBWMBX, axl-AXMCM)

## Verification
- All 37 files exist under `Tools/rbk/vov_veiled/`
- No RBS* files remain in `lenses/`
- CLAUDE.md mappings point to new paths
- No broken cross-references

Result: RBS concept models follow the veiled pattern. `Tools/rbk/` established as Recipe Bottle Kit directory.

### derive-build-strategy-from-platforms (₢AUAAA) [complete]

**[260313-1113] complete**

Remove RBRV_CONJURE_BINFMT_POLICY field; derive build strategy from platform list.

## Problem

The binfmt policy field is a manual declaration of something derivable from data. Every vessel currently says "allow". The field adds per-vessel overhead and a code path ("forbid") that has never been exercised.

## Design

The build strategy is determined by comparing RBRV_CONJURE_PLATFORMS against RBGC_BUILD_RUNNER_PLATFORM:

- Platforms exactly match runner → native build (no QEMU, no binfmt)
- Platforms differ from runner → full pipeline with binfmt registration

The predicate is string equality: does RBRV_CONJURE_PLATFORMS equal RBGC_BUILD_RUNNER_PLATFORM? Since the runner platform is a scalar ("linux/amd64"), any multi-platform or non-native platform list will differ, regardless of ordering. This is the same comparison the existing "forbid" gate already uses (line 579).

Conditionally include rbgjb02-qemu-binfmt.sh in the step array based on this platform-vs-runner comparison. The stitch loop itself stays unchanged — it iterates whatever is in the array.

## Build strategy log line

Emit an explicit, non-inferential log of the build strategy decision at two points:

1. **Stitch time** (local): log whether binfmt step was included and why
2. **Cloud Build runtime**: the stitched JSON should produce a log line in the build output recording the strategy (e.g., in rbgjb01 or a preamble), so Cloud Build history is self-documenting without counting steps

## Integrity argument

binfmt registration runs a --privileged container (tonistiigi/binfmt) that modifies kernel state. Skipping this for native-only builds reduces attack surface and strengthens the integrity claim for single-platform images.

## Work

1. Remove RBRV_CONJURE_BINFMT_POLICY from rbrv_regime.sh enrollment
2. Remove from all 6 vessel rbrv.env files
3. Update rbf_Foundry.sh stitch: conditionally include rbgjb02 in z_step_defs based on platform-vs-runner string equality
4. Delete the binfmt policy gate (lines 577-582) in conjure
5. Add build strategy log line at stitch time
6. Add build strategy log line in stitched Cloud Build output (rbgjb01 or preamble step)
7. Update specs: RBSRV-RegimeVessel.adoc (remove binfmt policy field), RBSCB-CloudBuildPosture.adoc (update multi-arch strategy), RBS0-SpecTop.adoc (remove rbrv_conjure_binfmt_policy linked term)
8. Update regime validation test expectations if AUAAE has landed

## Acceptance

- RBRV_CONJURE_BINFMT_POLICY removed from all vessel configs and regime enrollment
- Native-only vessel produces step list without rbgjb02
- Multi-platform vessel produces full pipeline including rbgjb02
- Build strategy explicitly logged at stitch time (not inferential)
- Build strategy explicitly logged in Cloud Build output (not inferential — don't count steps)
- Specs reflect the change

**[260313-1051] rough**

Remove RBRV_CONJURE_BINFMT_POLICY field; derive build strategy from platform list.

## Problem

The binfmt policy field is a manual declaration of something derivable from data. Every vessel currently says "allow". The field adds per-vessel overhead and a code path ("forbid") that has never been exercised.

## Design

The build strategy is determined by comparing RBRV_CONJURE_PLATFORMS against RBGC_BUILD_RUNNER_PLATFORM:

- Platforms exactly match runner → native build (no QEMU, no binfmt)
- Platforms differ from runner → full pipeline with binfmt registration

The predicate is string equality: does RBRV_CONJURE_PLATFORMS equal RBGC_BUILD_RUNNER_PLATFORM? Since the runner platform is a scalar ("linux/amd64"), any multi-platform or non-native platform list will differ, regardless of ordering. This is the same comparison the existing "forbid" gate already uses (line 579).

Conditionally include rbgjb02-qemu-binfmt.sh in the step array based on this platform-vs-runner comparison. The stitch loop itself stays unchanged — it iterates whatever is in the array.

## Build strategy log line

Emit an explicit, non-inferential log of the build strategy decision at two points:

1. **Stitch time** (local): log whether binfmt step was included and why
2. **Cloud Build runtime**: the stitched JSON should produce a log line in the build output recording the strategy (e.g., in rbgjb01 or a preamble), so Cloud Build history is self-documenting without counting steps

## Integrity argument

binfmt registration runs a --privileged container (tonistiigi/binfmt) that modifies kernel state. Skipping this for native-only builds reduces attack surface and strengthens the integrity claim for single-platform images.

## Work

1. Remove RBRV_CONJURE_BINFMT_POLICY from rbrv_regime.sh enrollment
2. Remove from all 6 vessel rbrv.env files
3. Update rbf_Foundry.sh stitch: conditionally include rbgjb02 in z_step_defs based on platform-vs-runner string equality
4. Delete the binfmt policy gate (lines 577-582) in conjure
5. Add build strategy log line at stitch time
6. Add build strategy log line in stitched Cloud Build output (rbgjb01 or preamble step)
7. Update specs: RBSRV-RegimeVessel.adoc (remove binfmt policy field), RBSCB-CloudBuildPosture.adoc (update multi-arch strategy), RBS0-SpecTop.adoc (remove rbrv_conjure_binfmt_policy linked term)
8. Update regime validation test expectations if AUAAE has landed

## Acceptance

- RBRV_CONJURE_BINFMT_POLICY removed from all vessel configs and regime enrollment
- Native-only vessel produces step list without rbgjb02
- Multi-platform vessel produces full pipeline including rbgjb02
- Build strategy explicitly logged at stitch time (not inferential)
- Build strategy explicitly logged in Cloud Build output (not inferential — don't count steps)
- Specs reflect the change

**[260313-1049] rough**

Remove RBRV_CONJURE_BINFMT_POLICY field; derive build strategy from platform list.

## Problem

The binfmt policy field is a manual declaration of something derivable from data. Every vessel currently says "allow". The field adds per-vessel overhead and a code path ("forbid") that has never been exercised.

## Design

The build strategy is determined by comparing RBRV_CONJURE_PLATFORMS against RBGC_BUILD_RUNNER_PLATFORM:

- All platforms are runner-native → simple build (no QEMU, no binfmt)
- Any platform requires emulation → full pipeline with binfmt registration

The predicate is: does the platform list contain anything the runner can't do natively?

Conditionally include rbgjb02-qemu-binfmt.sh in the step array based on platform count. The stitch loop itself stays unchanged — it iterates whatever is in the array.

## Build strategy log line

Emit an explicit, non-inferential log of the build strategy decision at two points:

1. **Stitch time** (local): log whether binfmt step was included and why
2. **Cloud Build runtime**: the stitched JSON should produce a log line in the build output recording the strategy (e.g., in rbgjb01 or a preamble), so Cloud Build history is self-documenting without counting steps

## Integrity argument

binfmt registration runs a --privileged container (tonistiigi/binfmt) that modifies kernel state. Skipping this for native-only builds reduces attack surface and strengthens the integrity claim for single-platform images.

## Work

1. Remove RBRV_CONJURE_BINFMT_POLICY from rbrv_regime.sh enrollment
2. Remove from all 6 vessel rbrv.env files
3. Update rbf_Foundry.sh stitch: conditionally include rbgjb02 in z_step_defs based on platform count vs runner platform
4. Delete the binfmt policy gate (lines 577-582) in conjure
5. Add build strategy log line at stitch time
6. Add build strategy log line in stitched Cloud Build output (rbgjb01 or preamble step)
7. Update specs: RBSRV-RegimeVessel.adoc (remove binfmt policy field), RBSCB-CloudBuildPosture.adoc (update multi-arch strategy), RBS0-SpecTop.adoc (remove rbrv_conjure_binfmt_policy linked term)
8. Update regime validation test expectations if AUAAE has landed

## Acceptance

- RBRV_CONJURE_BINFMT_POLICY removed from all vessel configs and regime enrollment
- Native-only vessel produces step list without rbgjb02
- Multi-platform vessel produces full pipeline including rbgjb02
- Build strategy explicitly logged at stitch time (not inferential)
- Build strategy explicitly logged in Cloud Build output (not inferential — don't count steps)
- Specs reflect the change

**[260310-1951] rough**

Remove RBRV_CONJURE_BINFMT_POLICY field; derive build strategy from platform list.

## Problem

The binfmt policy field is a manual declaration of something derivable from data. Every vessel currently says "allow". The field adds per-vessel overhead and a code path ("forbid") that has never been exercised.

## Design

The build strategy is determined by comparing RBRV_CONJURE_PLATFORMS against RBGC_BUILD_RUNNER_PLATFORM:

- All platforms are runner-native → simple `docker build` + `docker push` (no QEMU, no binfmt, no buildx)
- Any platform requires emulation → full pipeline: binfmt registration (rbgjb02), buildx multi-platform push (rbgjb03), per-platform pullback (rbgjb04)

The predicate is: does the platform list contain anything the runner can't do natively?

This eliminates the awkward "forbid" semantics (which was really "don't build multi-platform" not "build natively") and handles all cases including arm64-only-on-x86 correctly — it just detects QEMU is needed and uses the emulated pipeline.

## Integrity argument

binfmt registration runs a --privileged container (tonistiigi/binfmt) that modifies kernel state. Skipping this for native-only builds reduces attack surface and strengthens the integrity claim for single-platform amd64 images.

## Conjure-time visibility

When conjure selects build strategy, emit a log line: "Build strategy: native single-platform (linux/amd64)" or "Build strategy: emulated multi-platform via QEMU (linux/amd64, linux/arm64)". The Director sees the consequence of their platform choice at build time.

## Work

1. Remove RBRV_CONJURE_BINFMT_POLICY from rbrv_regime.sh enrollment
2. Remove from all 6 vessel rbrv.env files
3. Update rbf_Foundry.sh: replace binfmt policy check with platform-vs-runner comparison; conditionally include rbgjb02 in build step list; select docker build vs buildx based on result
4. Update build step stitching to produce a shorter step list for native-only builds
5. Add build strategy log line to conjure output
6. Update specs: RBSRV-RegimeVessel.adoc (remove binfmt policy field), RBSCB-CloudBuildRoadmap.adoc (update multi-arch strategy caveat), RBS0-SpecTop.adoc (remove rbrv_conjure_binfmt_policy linked term)
7. Update regime validation test expectations if ₢AUAAE has landed

## Acceptance

- RBRV_CONJURE_BINFMT_POLICY removed from all vessel configs and regime enrollment
- Native-only vessel produces a shorter build step list (no rbgjb02)
- Multi-platform vessel produces full pipeline as before
- Conjure emits build strategy in output
- Specs reflect the change

**[260310-1951] rough**

Remove RBRV_CONJURE_BINFMT_POLICY field; derive build strategy from platform list.

## Problem

The binfmt policy field is a manual declaration of something derivable from data. Every vessel currently says "allow". The field adds per-vessel overhead and a code path ("forbid") that has never been exercised.

## Design

The build strategy is determined by comparing RBRV_CONJURE_PLATFORMS against RBGC_BUILD_RUNNER_PLATFORM:

- All platforms are runner-native → simple `docker build` + `docker push` (no QEMU, no binfmt, no buildx)
- Any platform requires emulation → full pipeline: binfmt registration (rbgjb02), buildx multi-platform push (rbgjb03), per-platform pullback (rbgjb04)

The predicate is: does the platform list contain anything the runner can't do natively?

This eliminates the awkward "forbid" semantics (which was really "don't build multi-platform" not "build natively") and handles all cases including arm64-only-on-x86 correctly — it just detects QEMU is needed and uses the emulated pipeline.

## Integrity argument

binfmt registration runs a --privileged container (tonistiigi/binfmt) that modifies kernel state. Skipping this for native-only builds reduces attack surface and strengthens the integrity claim for single-platform amd64 images.

## Conjure-time visibility

When conjure selects build strategy, emit a log line: "Build strategy: native single-platform (linux/amd64)" or "Build strategy: emulated multi-platform via QEMU (linux/amd64, linux/arm64)". The Director sees the consequence of their platform choice at build time.

## Work

1. Remove RBRV_CONJURE_BINFMT_POLICY from rbrv_regime.sh enrollment
2. Remove from all 6 vessel rbrv.env files
3. Update rbf_Foundry.sh: replace binfmt policy check with platform-vs-runner comparison; conditionally include rbgjb02 in build step list; select docker build vs buildx based on result
4. Update build step stitching to produce a shorter step list for native-only builds
5. Add build strategy log line to conjure output
6. Update specs: RBSRV-RegimeVessel.adoc (remove binfmt policy field), RBSCB-CloudBuildRoadmap.adoc (update multi-arch strategy caveat), RBS0-SpecTop.adoc (remove rbrv_conjure_binfmt_policy linked term)
7. Update regime validation test expectations if ₢AUAAE has landed

## Acceptance

- RBRV_CONJURE_BINFMT_POLICY removed from all vessel configs and regime enrollment
- Native-only vessel produces a shorter build step list (no rbgjb02)
- Multi-platform vessel produces full pipeline as before
- Conjure emits build strategy in output
- Specs reflect the change

**[260131-1214] rough**

Evaluate RBRV_CONJURE_BINFMT_POLICY configuration option: determine whether to implement full support, stub it out, or remove it entirely. This is a decision point for MVP release readiness that affects the conjure subsystem's feature completeness.

### vouch-artifact-multiplatform (₢AUAAJ) [complete]

**[260313-1303] complete**

Make the -vouch artifact multi-platform via buildx, matching the -about pattern.

## Problem

rbgjv03 uses plain `docker build` + `docker push`, producing an amd64-only image. On arm64 hosts, `docker pull` warns about platform mismatch. The -about artifact (rbgjb08) already uses buildx correctly.

## Architecture Decisions (from review 2026-03-13)

**Private pool for vouch**: Move vouch from default pool to private pool (`RBRR_GCB_WORKER_POOL`). Eliminates the untested default-pool-with-buildx permutation. Vouch is fast (~60s, 3 steps) — private pool cost is negligible. The pool already exists for conjure. Add `pool: { name: ... }` to vouch build JSON options.

**Keep space-separated RBRV_CONJURE_PLATFORMS**: The regime variable stays space-separated (bash-native list format). Convert to comma-separated at the stitch point using `${RBRV_CONJURE_PLATFORMS// /,}` — same pattern conjure already uses (rbf_Foundry.sh line 208). Don't couple regime format to Docker CLI encoding.

**Bind-mode vessels out of scope**: Vouch requires GCB SLSA provenance, which only exists for conjure-mode vessels. Bind-mode trust comes from digest pinning (different model). No guard needed — vouch is only called from conjure flow.

## Key Files and Patterns

**rbgjv03-assemble-push-vouch.sh** (current):
- Uses `docker build -t TAG /workspace/vouch_ctx` + `docker push TAG`
- Builder image: RBRG_DOCKER_IMAGE_REF (gcr.io/cloud-builders/docker, digest-pinned)
- Substitutions: _RBGV_GAR_HOST, _RBGV_GAR_PATH, _RBGV_VESSEL, _RBGV_CONSECRATION

**rbgjb08-buildx-push-about.sh** (target pattern):
- Uses `docker buildx build --push --platform=PLATFORMS --tag TAG -f Dockerfile.meta .`
- Creates rb-builder with inspect-or-create pattern
- Receives _RBGY_PLATFORMS as comma-separated substitution

**rbf_Foundry.sh vouch JSON** (lines 1953-1980):
- Currently passes 7 _RBGV_* substitutions, no platforms
- No workerPool in options (only `logging: CLOUD_LOGGING_ONLY`)
- Needs: add _RBGV_PLATFORMS substitution, add pool to options

**rbf_Foundry.sh conjure JSON** (reference, lines 393-441):
- Platform conversion: `local -r z_platforms="${RBRV_CONJURE_PLATFORMS// /,}"`
- Pool: `pool: { name: $zjq_pool }` in options
- Substitution: `_RBGY_PLATFORMS: $zjq_platforms`

## Work

1. **rbgjv03-assemble-push-vouch.sh**: Replace `docker build`+`docker push` with buildx pattern from rbgjb08. Add inspect-or-create for rb-builder. Use `--platform="${_RBGV_PLATFORMS}"`. No TARGETARCH needed in Dockerfile — vouch content is identical across platforms (FROM scratch + COPY of platform-independent JSON files).

2. **rbf_Foundry.sh vouch JSON composition** (~line 1953): Add `_RBGV_PLATFORMS` substitution sourced from `${RBRV_CONJURE_PLATFORMS// /,}`. Add `pool: { name: $zjq_pool }` to options (pass RBRR_GCB_WORKER_POOL through).

3. **RBSAV-ark_vouch.adoc**: Replace completion section statement "The vouch artifact is a single-platform container" with multi-platform documentation. Preserve the parenthetical truth: content is architecture-independent, multi-platform is for pull ergonomics. Update pool reference from default to private. Add _RBGV_PLATFORMS to substitution variables table.

## Acceptance

- `docker pull` of -vouch artifact on arm64 host produces no platform warning
- Vouch contents (vouch_summary.json, verify-*.json) identical across platform variants
- Vouch runs on private pool (same as conjure)
- RBSAV spec reflects multi-platform vouch artifact and private pool
- No new host tooling dependencies
- Fast suite still passes

**[260313-1220] rough**

Make the -vouch artifact multi-platform via buildx, matching the -about pattern.

## Problem

rbgjv03 uses plain `docker build` + `docker push`, producing an amd64-only image. On arm64 hosts, `docker pull` warns about platform mismatch. The -about artifact (rbgjb08) already uses buildx correctly.

## Architecture Decisions (from review 2026-03-13)

**Private pool for vouch**: Move vouch from default pool to private pool (`RBRR_GCB_WORKER_POOL`). Eliminates the untested default-pool-with-buildx permutation. Vouch is fast (~60s, 3 steps) — private pool cost is negligible. The pool already exists for conjure. Add `pool: { name: ... }` to vouch build JSON options.

**Keep space-separated RBRV_CONJURE_PLATFORMS**: The regime variable stays space-separated (bash-native list format). Convert to comma-separated at the stitch point using `${RBRV_CONJURE_PLATFORMS// /,}` — same pattern conjure already uses (rbf_Foundry.sh line 208). Don't couple regime format to Docker CLI encoding.

**Bind-mode vessels out of scope**: Vouch requires GCB SLSA provenance, which only exists for conjure-mode vessels. Bind-mode trust comes from digest pinning (different model). No guard needed — vouch is only called from conjure flow.

## Key Files and Patterns

**rbgjv03-assemble-push-vouch.sh** (current):
- Uses `docker build -t TAG /workspace/vouch_ctx` + `docker push TAG`
- Builder image: RBRG_DOCKER_IMAGE_REF (gcr.io/cloud-builders/docker, digest-pinned)
- Substitutions: _RBGV_GAR_HOST, _RBGV_GAR_PATH, _RBGV_VESSEL, _RBGV_CONSECRATION

**rbgjb08-buildx-push-about.sh** (target pattern):
- Uses `docker buildx build --push --platform=PLATFORMS --tag TAG -f Dockerfile.meta .`
- Creates rb-builder with inspect-or-create pattern
- Receives _RBGY_PLATFORMS as comma-separated substitution

**rbf_Foundry.sh vouch JSON** (lines 1953-1980):
- Currently passes 7 _RBGV_* substitutions, no platforms
- No workerPool in options (only `logging: CLOUD_LOGGING_ONLY`)
- Needs: add _RBGV_PLATFORMS substitution, add pool to options

**rbf_Foundry.sh conjure JSON** (reference, lines 393-441):
- Platform conversion: `local -r z_platforms="${RBRV_CONJURE_PLATFORMS// /,}"`
- Pool: `pool: { name: $zjq_pool }` in options
- Substitution: `_RBGY_PLATFORMS: $zjq_platforms`

## Work

1. **rbgjv03-assemble-push-vouch.sh**: Replace `docker build`+`docker push` with buildx pattern from rbgjb08. Add inspect-or-create for rb-builder. Use `--platform="${_RBGV_PLATFORMS}"`. No TARGETARCH needed in Dockerfile — vouch content is identical across platforms (FROM scratch + COPY of platform-independent JSON files).

2. **rbf_Foundry.sh vouch JSON composition** (~line 1953): Add `_RBGV_PLATFORMS` substitution sourced from `${RBRV_CONJURE_PLATFORMS// /,}`. Add `pool: { name: $zjq_pool }` to options (pass RBRR_GCB_WORKER_POOL through).

3. **RBSAV-ark_vouch.adoc**: Replace completion section statement "The vouch artifact is a single-platform container" with multi-platform documentation. Preserve the parenthetical truth: content is architecture-independent, multi-platform is for pull ergonomics. Update pool reference from default to private. Add _RBGV_PLATFORMS to substitution variables table.

## Acceptance

- `docker pull` of -vouch artifact on arm64 host produces no platform warning
- Vouch contents (vouch_summary.json, verify-*.json) identical across platform variants
- Vouch runs on private pool (same as conjure)
- RBSAV spec reflects multi-platform vouch artifact and private pool
- No new host tooling dependencies
- Fast suite still passes

**[260310-1925] rough**

Make the -vouch artifact multi-platform via buildx, matching the -about pattern.

## Problem

rbgjv03 uses plain `docker build` + `docker push`, producing an amd64-only image. On arm64 hosts, `docker pull` warns about platform mismatch. The -about artifact (rbgjb08) already uses buildx correctly.

## Work

1. Update Tools/rbk/rbgjv/rbgjv03-assemble-push-vouch.sh to use `docker buildx build --push --platform=...` instead of plain `docker build` + `docker push`
2. Follow the rbgjb08 pattern: reuse the rb-builder instance, pass platform substitution vars
3. Ensure the vouch Cloud Build step has the necessary substitution variables (_RBGV_PLATFORMS or equivalent)
4. Update rbf_Foundry.sh vouch build JSON composition if new substitutions are needed
5. **Update RBSAV-ark_vouch.adoc**: remove line 124 statement that vouch is single-platform; document multi-platform buildx pattern

## Acceptance

- `docker pull` of -vouch artifact on arm64 host produces no platform warning
- Vouch contents (vouch_summary.json, verify-*.json) identical across platform variants
- RBSAV spec reflects multi-platform vouch artifact
- No new host tooling dependencies

**[260310-1919] rough**

Make the -vouch artifact multi-platform via buildx, matching the -about pattern.

## Problem

rbgjv03 uses plain `docker build` + `docker push`, producing an amd64-only image. On arm64 hosts, `docker pull` warns about platform mismatch. The -about artifact (rbgjb08) already uses buildx correctly.

## Work

1. Update Tools/rbk/rbgjv/rbgjv03-assemble-push-vouch.sh to use `docker buildx build --push --platform=...` instead of plain `docker build` + `docker push`
2. Follow the rbgjb08 pattern: reuse the rb-builder instance, pass platform substitution vars
3. Ensure the vouch Cloud Build step has the necessary substitution variables (_RBGV_PLATFORMS or equivalent)
4. Update rbf_Foundry.sh vouch build JSON composition if new substitutions are needed

## Acceptance

- `docker pull` of -vouch artifact on arm64 host produces no platform warning
- Vouch contents (vouch_summary.json, verify-*.json) identical across platform variants
- No new host tooling dependencies

### local-vouch-preflight-gate (₢AUAAK) [complete]

**[260313-1345] complete**

Require local -vouch artifact on every bottle start, unconditionally.

## Problem

The current vouch gate only fires on the auto-summon path (when -image is missing locally). If -image is already local, no vouch check happens. A previously-pulled-but-unvouched image starts without complaint.

## Design

1. On every bottle start, for each vessel (sentry + bottle), check that the -vouch image exists locally via `docker image inspect` on the vouch tag
2. If -vouch is missing locally: fatal — do not start, do not auto-summon, do not fall back to remote HEAD check
3. This means summon (which now pulls -vouch) becomes the prerequisite for bottle start
4. Remove or demote the remote vouch gate HEAD check — local presence is the authority

## Rationale

Local -vouch presence proves the operator deliberately acquired the attestation. Remote HEAD check only proves the tag exists in GAR — weaker trust. After the summon change (now pulls -vouch), requiring local presence is a natural tightening.

## Spec Update

**Update RBSBS-bottle_start.adoc**: add a preflight step requiring local -vouch artifact presence for each vessel before proceeding. This is a new gate not currently in the spec.

## Files

- Tools/rbk/rbob_bottle.sh: add unconditional local vouch image inspect to preflight, fatal if missing
- Tools/rbk/rbf_Foundry.sh: rbf_vouch_gate may be simplified or removed if local-only check replaces it
- Tools/rbk/vov_veiled/RBSBS-bottle_start.adoc: add vouch preflight gate step

## Acceptance

- Bottle start with -vouch locally present: succeeds
- Bottle start without -vouch locally: fatal error with clear message to run summon first
- RBSBS spec documents the new preflight requirement
- No network calls required for vouch validation at start time

**[260310-1925] rough**

Require local -vouch artifact on every bottle start, unconditionally.

## Problem

The current vouch gate only fires on the auto-summon path (when -image is missing locally). If -image is already local, no vouch check happens. A previously-pulled-but-unvouched image starts without complaint.

## Design

1. On every bottle start, for each vessel (sentry + bottle), check that the -vouch image exists locally via `docker image inspect` on the vouch tag
2. If -vouch is missing locally: fatal — do not start, do not auto-summon, do not fall back to remote HEAD check
3. This means summon (which now pulls -vouch) becomes the prerequisite for bottle start
4. Remove or demote the remote vouch gate HEAD check — local presence is the authority

## Rationale

Local -vouch presence proves the operator deliberately acquired the attestation. Remote HEAD check only proves the tag exists in GAR — weaker trust. After the summon change (now pulls -vouch), requiring local presence is a natural tightening.

## Spec Update

**Update RBSBS-bottle_start.adoc**: add a preflight step requiring local -vouch artifact presence for each vessel before proceeding. This is a new gate not currently in the spec.

## Files

- Tools/rbk/rbob_bottle.sh: add unconditional local vouch image inspect to preflight, fatal if missing
- Tools/rbk/rbf_Foundry.sh: rbf_vouch_gate may be simplified or removed if local-only check replaces it
- Tools/rbk/vov_veiled/RBSBS-bottle_start.adoc: add vouch preflight gate step

## Acceptance

- Bottle start with -vouch locally present: succeeds
- Bottle start without -vouch locally: fatal error with clear message to run summon first
- RBSBS spec documents the new preflight requirement
- No network calls required for vouch validation at start time

**[260310-1922] rough**

Require local -vouch artifact on every bottle start, unconditionally.

## Problem

The current vouch gate only fires on the auto-summon path (when -image is missing locally). If -image is already local, no vouch check happens. A previously-pulled-but-unvouched image starts without complaint.

## Design

1. On every bottle start, for each vessel (sentry + bottle), check that the -vouch image exists locally via `docker image inspect` on the vouch tag
2. If -vouch is missing locally: fatal — do not start, do not auto-summon, do not fall back to remote HEAD check
3. This means summon (which now pulls -vouch) becomes the prerequisite for bottle start
4. Remove or demote the remote vouch gate HEAD check — local presence is the authority

## Rationale

Local -vouch presence proves the operator deliberately acquired the attestation. Remote HEAD check only proves the tag exists in GAR — weaker trust. After the summon change (now pulls -vouch), requiring local presence is a natural tightening.

## Files

- Tools/rbk/rbob_bottle.sh: add unconditional local vouch image inspect to preflight, fatal if missing
- Tools/rbk/rbf_Foundry.sh: rbf_vouch_gate may be simplified or removed if local-only check replaces it

## Acceptance

- Bottle start with -vouch locally present: succeeds
- Bottle start without -vouch locally: fatal error with clear message to run summon first
- No network calls required for vouch validation at start time

### release-qualification-gate (₢AUAAI) [complete]

**[260313-1141] complete**

Create release qualification gate tabtarget (rbw-QR) and remove shellcheck from routine workbench gate.

## Changes

1. Remove buq_shellcheck call from rbq_qualify_all — workbench gate keeps tabtarget, colophon, and nameplate preflight checks but no longer runs shellcheck on every bottle-start

2. Create rbq_qualify_release function that runs:
   - rbq_qualify_all (cheap checks)
   - buq_shellcheck (full shellcheck sweep)
   - Complete test suite (sequential, not parallel)
   - Fail-fast: stop on first failure

3. Register rbw-QR colophon in rbz_zipper.sh, create tabtarget

## Files

- Tools/rbk/rbq_Qualify.sh: remove shellcheck from qualify_all, add qualify_release
- Tools/rbk/rbq_cli.sh: add cli routing for qualify_release
- Tools/rbk/rbz_zipper.sh: enroll rbw-QR colophon
- tt/: new rbw-QR.QualifyRelease.sh tabtarget

## Acceptance

- Bottle-start no longer runs shellcheck
- rbw-QR runs all checks + shellcheck + complete test suite sequentially
- Existing rbw-qa tabtarget still works (now without shellcheck)

**[260313-1134] rough**

Create release qualification gate tabtarget (rbw-QR) and remove shellcheck from routine workbench gate.

## Changes

1. Remove buq_shellcheck call from rbq_qualify_all — workbench gate keeps tabtarget, colophon, and nameplate preflight checks but no longer runs shellcheck on every bottle-start

2. Create rbq_qualify_release function that runs:
   - rbq_qualify_all (cheap checks)
   - buq_shellcheck (full shellcheck sweep)
   - Complete test suite (sequential, not parallel)
   - Fail-fast: stop on first failure

3. Register rbw-QR colophon in rbz_zipper.sh, create tabtarget

## Files

- Tools/rbk/rbq_Qualify.sh: remove shellcheck from qualify_all, add qualify_release
- Tools/rbk/rbq_cli.sh: add cli routing for qualify_release
- Tools/rbk/rbz_zipper.sh: enroll rbw-QR colophon
- tt/: new rbw-QR.QualifyRelease.sh tabtarget

## Acceptance

- Bottle-start no longer runs shellcheck
- rbw-QR runs all checks + shellcheck + complete test suite sequentially
- Existing rbw-qa tabtarget still works (now without shellcheck)

**[260310-1901] rough**

Create a tiered qualification system and release qualification tabtarget.

## Problem

rbq_qualify_all runs shellcheck (133 files) on every bottle-start and conjure-ark invocation. Shellcheck is valuable but too slow for routine operations.

## Design

1. Split rbq_qualify_all into two tiers:
   - rbq_qualify_fast: tabtarget structure, colophon registrations, nameplate preflight (cheap checks)
   - rbq_qualify_all: everything in fast + shellcheck

2. Workbench gate (rbw_workbench.sh line 67) calls rbq_qualify_fast instead of rbq_qualify_all

3. New release qualification tabtarget that runs:
   - rbq_qualify_all (including shellcheck)
   - Complete test suite (all rbtb/rbtg routes)
   - This is the pre-release gate — everything must pass before tagging

4. Cloud build does NOT run qualification — it builds what it's told. Qualification is a local pre-flight concern.

## Files

- Tools/rbk/rbq_Qualify.sh: split qualify_all, add qualify_fast
- Tools/rbk/rbw_workbench.sh: gate calls qualify_fast
- Tools/rbk/rbz_zipper.sh: enroll new release-qualify colophon
- tt/: new release qualification tabtarget

### regime-validation-testbench (₢AUAAE) [complete]

**[260313-1141] complete**

Add regime validation testbench to rbtg_testbench.sh exercising RBRV and RBRN validators with both synthetic bad inputs and real good inputs.

**Note**: RBRV test cases reflect the state AFTER ¢AUAAA (derive-build-strategy-from-platforms) lands. RBRV_CONJURE_BINFMT_POLICY will have been removed by then — do not test for it.

## Negative tests (but_expect_fatal)

RBRV failure cases:
- Missing RBRV_SIGIL (required xname)
- Neither RBRV_BIND_IMAGE nor RBRV_CONJURE_DOCKERFILE set
- Unexpected RBRV_ variable present (e.g. RBRV_BOGUS=foo)
- Partial conjure config (CONJURE_DOCKERFILE set but CONJURE_PLATFORMS missing)

RBRN failure cases:
- Missing RBRN_MONIKER (required xname)
- Invalid RBRN_RUNTIME (not "docker" or "podman")
- Invalid RBRN_ENTRY_MODE (not "disabled" or "enabled")
- Invalid RBRN_UPLINK_DNS_MODE / RBRN_UPLINK_ACCESS_MODE
- ENTRY_PORT_WORKSTATION >= UPLINK_PORT_MIN when entry enabled
- Unexpected RBRN_ variable present
- Bad IP format for RBRN_ENCLAVE_BASE_IP

## Positive tests (but_expect_ok)

- Validate all vessel rbrv.env files pass via rbrv_cli.sh validate
- Validate all rbrn_*.env nameplate files pass via rbrn_cli.sh validate

## Mechanism

Each negative test runs in a subshell, sets up bad env vars, calls kindle + validate_fields, expects death. Positive tests call through CLI validate path. New route `rbtg-rv` in rbtg_testbench.sh. New tabtarget `tt/rbtg-rv.RegimeValidation.sh`.

## Scope control

- Pure local tests — no GCP, no containers, no network
- Only add to rbtg_testbench.sh and create tabtarget
- Do NOT modify regime.sh or cli.sh files

**[260310-1955] rough**

Add regime validation testbench to rbtg_testbench.sh exercising RBRV and RBRN validators with both synthetic bad inputs and real good inputs.

**Note**: RBRV test cases reflect the state AFTER ¢AUAAA (derive-build-strategy-from-platforms) lands. RBRV_CONJURE_BINFMT_POLICY will have been removed by then — do not test for it.

## Negative tests (but_expect_fatal)

RBRV failure cases:
- Missing RBRV_SIGIL (required xname)
- Neither RBRV_BIND_IMAGE nor RBRV_CONJURE_DOCKERFILE set
- Unexpected RBRV_ variable present (e.g. RBRV_BOGUS=foo)
- Partial conjure config (CONJURE_DOCKERFILE set but CONJURE_PLATFORMS missing)

RBRN failure cases:
- Missing RBRN_MONIKER (required xname)
- Invalid RBRN_RUNTIME (not "docker" or "podman")
- Invalid RBRN_ENTRY_MODE (not "disabled" or "enabled")
- Invalid RBRN_UPLINK_DNS_MODE / RBRN_UPLINK_ACCESS_MODE
- ENTRY_PORT_WORKSTATION >= UPLINK_PORT_MIN when entry enabled
- Unexpected RBRN_ variable present
- Bad IP format for RBRN_ENCLAVE_BASE_IP

## Positive tests (but_expect_ok)

- Validate all vessel rbrv.env files pass via rbrv_cli.sh validate
- Validate all rbrn_*.env nameplate files pass via rbrn_cli.sh validate

## Mechanism

Each negative test runs in a subshell, sets up bad env vars, calls kindle + validate_fields, expects death. Positive tests call through CLI validate path. New route `rbtg-rv` in rbtg_testbench.sh. New tabtarget `tt/rbtg-rv.RegimeValidation.sh`.

## Scope control

- Pure local tests — no GCP, no containers, no network
- Only add to rbtg_testbench.sh and create tabtarget
- Do NOT modify regime.sh or cli.sh files

**[260209-1650] rough**

Add regime validation testbench to rbtg_testbench.sh exercising RBRV and RBRN validators with both synthetic bad inputs and real good inputs.

## Negative tests (but_expect_fatal)

RBRV failure cases:
- Missing RBRV_SIGIL (required xname)
- Neither RBRV_BIND_IMAGE nor RBRV_CONJURE_DOCKERFILE set
- Invalid RBRV_CONJURE_BINFMT_POLICY (not "allow" or "forbid")
- Unexpected RBRV_ variable present (e.g. RBRV_BOGUS=foo)
- Partial conjure config (CONJURE_DOCKERFILE set but CONJURE_PLATFORMS missing)

RBRN failure cases:
- Missing RBRN_MONIKER (required xname)
- Invalid RBRN_RUNTIME (not "docker" or "podman")
- Invalid RBRN_ENTRY_MODE (not "disabled" or "enabled")
- Invalid RBRN_UPLINK_DNS_MODE / RBRN_UPLINK_ACCESS_MODE
- ENTRY_PORT_WORKSTATION >= UPLINK_PORT_MIN when entry enabled
- Unexpected RBRN_ variable present
- Bad IP format for RBRN_ENCLAVE_BASE_IP

## Positive tests (but_expect_ok)

- Validate all 7 rbev-vessels/*/rbrv.env files pass via rbrv_cli.sh validate
- Validate all 3 rbrn_*.env nameplate files pass via rbrn_cli.sh validate

## Mechanism

Each negative test runs in a subshell, sets up bad env vars, calls kindle + validate_fields, expects death. Positive tests call through CLI validate path. New route `rbtg-rv` in rbtg_testbench.sh. New tabtarget `tt/rbtg-rv.RegimeValidation.sh`.

## Scope control

- Pure local tests — no GCP, no containers, no network
- Only add to rbtg_testbench.sh and create tabtarget
- Do NOT modify regime.sh or cli.sh files

### consecration-inspect-command (₢AUAAL) [complete]

**[260313-1505] complete**

Consecration inspect command: two colophons (rbw-RiF full, rbw-Ric compact) displaying trust posture from locally-present -about and -vouch artifacts.

## Status

Core implementation complete (notched). Remaining work: integrate base image provenance data from •AUAAS once the -about artifact is enriched.

Consumer docs (CLAUDE.consumer.md, README.consumer.md) already reference inspect with correct colophons and descriptions.

## Current Implementation

Two public functions `rbf_inspect_full` / `rbf_inspect_compact` with shared `zrbf_inspect_show_sections`. Takes vessel + consecration parameters (like rbf_summon). Extracts from -about/-vouch via `docker create` + `docker cp`. Handles bind vs conjure vessels.

**Compact (rbw-Ric)** — Sectioned view: vessel type, base image (FROM line + syft distro), source, builder, SLSA provenance (with attests/does-not-attest), SBOM summary (count + type breakdown), vouch results. Each section has a 1-sentence explanation.

**Full (rbw-RiF)** — Everything in compact, plus: package inventory table (TYPE NAME VERSION, sorted by type), package licensing table (NAME LICENSE PURL, sorted by name), and Dockerfile contents.

## Remaining: Base Image Provenance (after •AUAAS)

Once •AUAAS enriches -about with `buildkit_metadata.json`, `cache_before.json`, and `cache_after.json`:

1. Update `zrbf_inspect_show_sections` with a **Base Image Provenance** section showing resolved base image references, digests, and layers from `buildkit_metadata.json` — replacing the current FROM-line-grep approach
2. Update `zrbf_inspect_show_full` to show cache inventory diff (cache_before vs cache_after) as a diagnostic section
3. Graceful fallback: if buildkit_metadata.json is absent (older consecrations built before •AUAAS), show the current FROM-line + syft distro approach

## Acceptance

- Conjure vessel: full summary with explicit SLSA boundary statements
- Bind vessel: clear statement that trust is digest-pin only, no false provenance claims
- Base image provenance from buildkit_metadata.json when available (fallback to FROM line)
- Cache inventory diff shown in full mode when available
- SBOM section explains what syft inventories and what it doesn't claim
- Two package listings in full mode: type/name/version and name/license/purl
- Fatal with clear message if required artifacts not locally present
- Works for all vessel types in the current fleet

**[260313-1435] rough**

Consecration inspect command: two colophons (rbw-RiF full, rbw-Ric compact) displaying trust posture from locally-present -about and -vouch artifacts.

## Status

Core implementation complete (notched). Remaining work: integrate base image provenance data from •AUAAS once the -about artifact is enriched.

Consumer docs (CLAUDE.consumer.md, README.consumer.md) already reference inspect with correct colophons and descriptions.

## Current Implementation

Two public functions `rbf_inspect_full` / `rbf_inspect_compact` with shared `zrbf_inspect_show_sections`. Takes vessel + consecration parameters (like rbf_summon). Extracts from -about/-vouch via `docker create` + `docker cp`. Handles bind vs conjure vessels.

**Compact (rbw-Ric)** — Sectioned view: vessel type, base image (FROM line + syft distro), source, builder, SLSA provenance (with attests/does-not-attest), SBOM summary (count + type breakdown), vouch results. Each section has a 1-sentence explanation.

**Full (rbw-RiF)** — Everything in compact, plus: package inventory table (TYPE NAME VERSION, sorted by type), package licensing table (NAME LICENSE PURL, sorted by name), and Dockerfile contents.

## Remaining: Base Image Provenance (after •AUAAS)

Once •AUAAS enriches -about with `buildkit_metadata.json`, `cache_before.json`, and `cache_after.json`:

1. Update `zrbf_inspect_show_sections` with a **Base Image Provenance** section showing resolved base image references, digests, and layers from `buildkit_metadata.json` — replacing the current FROM-line-grep approach
2. Update `zrbf_inspect_show_full` to show cache inventory diff (cache_before vs cache_after) as a diagnostic section
3. Graceful fallback: if buildkit_metadata.json is absent (older consecrations built before •AUAAS), show the current FROM-line + syft distro approach

## Acceptance

- Conjure vessel: full summary with explicit SLSA boundary statements
- Bind vessel: clear statement that trust is digest-pin only, no false provenance claims
- Base image provenance from buildkit_metadata.json when available (fallback to FROM line)
- Cache inventory diff shown in full mode when available
- SBOM section explains what syft inventories and what it doesn't claim
- Two package listings in full mode: type/name/version and name/license/purl
- Fatal with clear message if required artifacts not locally present
- Works for all vessel types in the current fleet

**[260313-1348] rough**

Consecration inspect command: two colophons (rbw-RiF full, rbw-Ric compact) displaying trust posture from locally-present -about and -vouch artifacts.

## Status

Core implementation complete (notched). Remaining work: integrate base image provenance data from ₢AUAAS once the -about artifact is enriched.

## Current Implementation

Two public functions `rbf_inspect_full` / `rbf_inspect_compact` with shared `zrbf_inspect_show_sections`. Takes vessel + consecration parameters (like rbf_summon). Extracts from -about/-vouch via `docker create` + `docker cp`. Handles bind vs conjure vessels.

**Compact (rbw-Ric)** — Sectioned view: vessel type, base image (FROM line + syft distro), source, builder, SLSA provenance (with attests/does-not-attest), SBOM summary (count + type breakdown), vouch results. Each section has a 1-sentence explanation.

**Full (rbw-RiF)** — Everything in compact, plus: package inventory table (TYPE NAME VERSION, sorted by type), package licensing table (NAME LICENSE PURL, sorted by name), and Dockerfile contents.

## Remaining: Base Image Provenance (after ₢AUAAS)

Once ₢AUAAS enriches -about with `buildkit_metadata.json`, `cache_before.json`, and `cache_after.json`:

1. Update `zrbf_inspect_show_sections` with a **Base Image Provenance** section showing resolved base image references, digests, and layers from `buildkit_metadata.json` — replacing the current FROM-line-grep approach
2. Update `zrbf_inspect_show_full` to show cache inventory diff (cache_before vs cache_after) as a diagnostic section
3. Graceful fallback: if buildkit_metadata.json is absent (older consecrations built before ₢AUAAS), show the current FROM-line + syft distro approach

## Acceptance

- Conjure vessel: full summary with explicit SLSA boundary statements
- Bind vessel: clear statement that trust is digest-pin only, no false provenance claims
- Base image provenance from buildkit_metadata.json when available (fallback to FROM line)
- Cache inventory diff shown in full mode when available
- SBOM section explains what syft inventories and what it doesn't claim
- Two package listings in full mode: type/name/version and name/license/purl
- Fatal with clear message if required artifacts not locally present
- Works for all vessel types in the current fleet

**[260313-1258] rough**

New Director command to inspect a consecration's trust posture, build provenance, and package inventory.

## Problem

The supply chain metadata for a consecration (-about and -vouch artifacts) is opaque. A Director who conjured and vouched an ark has no easy way to see: what build strategy was used, what packages are inside, what SLSA provenance actually attests to (and what it doesn't), or the verification results. Bind vessels have a fundamentally different trust model than conjure vessels, and the command must be honest about this.

## Trust Model Honesty

**Conjure vessels**: Built by GCB. Have SLSA Build L3 provenance, SBOM from syft, build transcript, Dockerfile snapshot. Provenance attests build authenticity and input/output binding. It does NOT attest base image security, package integrity, or absence of vulnerabilities. The command must clearly state these boundaries.

**Bind vessels**: External images pinned by digest. No SLSA provenance, no SBOM, no build transcript — none of these exist because GCB didn't build it. Trust is based solely on digest pinning of a known-good external image. The command must clearly say: "this is a bind vessel — trust is digest-pin only, no build provenance exists."

The command must never imply a bind vessel has provenance it doesn't have, or that a conjure vessel's SLSA attestation covers more than it does.

## Design

Inspect reads locally-present -about and -vouch images (pulled by summon) and presents a human-readable summary:

- **Vessel type**: Conjure (built by GCB) or bind (digest-pinned external image)
- **Build strategy** (conjure only): native or emulated, which platforms, QEMU involvement
- **Source** (conjure only): git repo + commit that produced this build
- **Builder** (conjure only): Google Cloud Build ID, service account, build timestamps
- **SLSA provenance** (conjure only): level, what it attests, what it does NOT attest
- **SBOM summary** (conjure only): syft package inventory — package count, base image (pinned or unpinned tag), notable packages. Be clear about what syft claims: it inventories installed packages, not security posture.
- **Vouch results**: per-platform pass/fail, verifier identity and version
- **Digest pin** (bind only): the pinned digest and its source registry

Requires -about and -vouch to be locally present (summon must have been run). Fatal if either is missing. For bind vessels, -vouch may not exist — handle gracefully.

## Data Sources (all local, no network)

- `-about` container: build_info.json (build metadata, SLSA level, git info, QEMU usage), sbom.json (syft package inventory), recipe.txt (Dockerfile)
- `-vouch` container: vouch_summary.json (per-platform verdicts, verifier identity), verify-*.json (full SLSA provenance statements)
- For bind vessels: vessel config (RBRV) provides the digest pin; -about/-vouch may be absent

## Implementation

1. New command in rbf_Foundry.sh or separate module
2. Detect vessel type from -about content (or absence thereof)
3. Extract files from -about and -vouch via `docker create` + `docker cp` into temp dir
4. Parse JSON with jq, format human-readable output with clear section headers
5. New colophon + tabtarget: rbw-CI.ConsecrationInspect (needs imprint for vessel selection)
6. Zipper enrollment and spec treatment

## Acceptance

- Conjure vessel: full summary with explicit SLSA boundary statements
- Bind vessel: clear statement that trust is digest-pin only, no false provenance claims
- SBOM section explains what syft inventories and what it doesn't claim
- Build strategy section shows native vs emulated and why
- Fatal with clear message if required artifacts not locally present
- Works for all vessel types in the current fleet

**[260310-1952] rough**

New Director command to inspect a consecration's build consequences, SBOM, and provenance.

## Problem

The supply chain metadata for a consecration (-about and -vouch artifacts) is opaque. A Director who conjured and vouched an ark has no easy way to see: what build strategy was used (native vs emulated), what packages are inside (SBOM), what the SLSA provenance actually attests to (and what it doesn't), or the verification results.

This came up in conversation — SLSA v1.0 Build L3 sounds impressive but its scope is narrow and non-obvious. The Director needs a command that makes consequences visible.

## Design

A consecration inspect command reads locally-present -about and -vouch images (pulled by summon) and presents a human-readable summary:

- **Build strategy**: native or emulated, which platforms, QEMU involvement
- **Source**: git repo + commit that produced this build
- **Builder**: Google Cloud Build ID, service account, build timestamps
- **SLSA provenance**: level, what it attests (build authenticity, input/output binding), what it does NOT attest (base image security, package integrity)
- **SBOM summary**: package count, base image (pinned or unpinned tag), notable packages
- **Vouch results**: per-platform pass/fail, verifier identity and version

Requires -about and -vouch to be locally present (summon must have been run). Fatal if either is missing.

## Data sources (all local, no network)

- `-about` container: build_info.json (build metadata, SLSA level, git info, QEMU usage), sbom.json (syft package inventory), recipe.txt (Dockerfile)
- `-vouch` container: vouch_summary.json (per-platform verdicts, verifier identity), verify-*.json (full SLSA provenance statements)

## Implementation

1. New command in rbf_Foundry.sh or separate module
2. Extract files from -about and -vouch via `docker create` + `docker cp` into temp dir
3. Parse JSON with jq, format human-readable output
4. New colophon + tabtarget (e.g. rbw-CI.ConsecrationInspect.sh)
5. Spec treatment in RBS0 for the new operation

## Acceptance

- Command produces readable summary from local -about and -vouch artifacts
- SLSA section explains what provenance does and does not attest
- SBOM section highlights base image pinning status
- Build strategy section shows native vs emulated and why
- Fatal with clear message if -about or -vouch not locally present

### spec-updates-for-inspect-and-about-enrichment (₢AUAAT) [complete]

**[260313-1516] complete**

Update specification documents to reflect the consecration inspect command (•AUAAL) and -about artifact enrichment (•AUAAS).

## Note

Consumer docs (CLAUDE.consumer.md, README.consumer.md) already reference inspect commands (rbw-RiF, rbw-Ric) with correct descriptions and include inspect in day-to-day operations. Spec work remains valid but consumer-facing documentation is ahead of specs.

## RBS0-SpecTop.adoc (structural)

- Add operation definition for `ark_inspect` (linked term, attribute reference, anchor)
- Add colophon entries for `rbw-RiF` (full) and `rbw-Ric` (compact)
- Add operation section after ark_vouch in the Ark Group, with proper includes
- Register in operation group listing

## RBSCB-CloudBuildPosture.adoc (artifact contents)

- Update -about artifact contents to include `buildkit_metadata.json`, `cache_before.json`, `cache_after.json`
- Document cache hygiene (prune before build) and its provenance implications
- Describe what BuildKit metadata captures vs what cache inventory captures

## RBSCO-CosmologyIntro.adoc (cross-reference)

- Mention inspect as the mechanism for viewing auxiliary provenance data
- Brief reference to the two modes (compact/full)

## RBSGS-GettingStarted.adoc (command listing)

- Add inspect tabtargets to the command reference
- Brief usage example in the workflow section
- Note: README.consumer.md already covers this for the consumer view

## RBSAC, RBSAS, RBSAV (backlinks)

- Add cross-references to inspect where appropriate (conjure produces what inspect reads, summon retrieves it locally, vouch results are displayed by inspect)

## Possible new file

- Evaluate whether a dedicated RBSII-ark_inspect.adoc spec is warranted or whether the RBS0 operation section is sufficient. Follow the pattern of RBSAC/RBSAS/RBSAV if standalone.

## Acceptance

- All new colophons (rbw-RiF, rbw-Ric) appear in RBS0 colophon registry
- -about artifact contents documented accurately in RBSCB
- No broken cross-references between spec documents
- Inspect operation fully defined with linked terms in RBS0

**[260313-1434] rough**

Update specification documents to reflect the consecration inspect command (•AUAAL) and -about artifact enrichment (•AUAAS).

## Note

Consumer docs (CLAUDE.consumer.md, README.consumer.md) already reference inspect commands (rbw-RiF, rbw-Ric) with correct descriptions and include inspect in day-to-day operations. Spec work remains valid but consumer-facing documentation is ahead of specs.

## RBS0-SpecTop.adoc (structural)

- Add operation definition for `ark_inspect` (linked term, attribute reference, anchor)
- Add colophon entries for `rbw-RiF` (full) and `rbw-Ric` (compact)
- Add operation section after ark_vouch in the Ark Group, with proper includes
- Register in operation group listing

## RBSCB-CloudBuildPosture.adoc (artifact contents)

- Update -about artifact contents to include `buildkit_metadata.json`, `cache_before.json`, `cache_after.json`
- Document cache hygiene (prune before build) and its provenance implications
- Describe what BuildKit metadata captures vs what cache inventory captures

## RBSCO-CosmologyIntro.adoc (cross-reference)

- Mention inspect as the mechanism for viewing auxiliary provenance data
- Brief reference to the two modes (compact/full)

## RBSGS-GettingStarted.adoc (command listing)

- Add inspect tabtargets to the command reference
- Brief usage example in the workflow section
- Note: README.consumer.md already covers this for the consumer view

## RBSAC, RBSAS, RBSAV (backlinks)

- Add cross-references to inspect where appropriate (conjure produces what inspect reads, summon retrieves it locally, vouch results are displayed by inspect)

## Possible new file

- Evaluate whether a dedicated RBSII-ark_inspect.adoc spec is warranted or whether the RBS0 operation section is sufficient. Follow the pattern of RBSAC/RBSAS/RBSAV if standalone.

## Acceptance

- All new colophons (rbw-RiF, rbw-Ric) appear in RBS0 colophon registry
- -about artifact contents documented accurately in RBSCB
- No broken cross-references between spec documents
- Inspect operation fully defined with linked terms in RBS0

**[260313-1352] rough**

Update specification documents to reflect the consecration inspect command (₢AUAAL) and -about artifact enrichment (₢AUAAS).

## RBS0-SpecTop.adoc (structural)

- Add operation definition for `ark_inspect` (linked term, attribute reference, anchor)
- Add colophon entries for `rbw-RiF` (full) and `rbw-Ric` (compact)
- Add operation section after ark_vouch in the Ark Group, with proper includes
- Register in operation group listing

## RBSCB-CloudBuildPosture.adoc (artifact contents)

- Update -about artifact contents to include `buildkit_metadata.json`, `cache_before.json`, `cache_after.json`
- Document cache hygiene (prune before build) and its provenance implications
- Describe what BuildKit metadata captures vs what cache inventory captures

## RBSCO-CosmologyIntro.adoc (cross-reference)

- Mention inspect as the mechanism for viewing auxiliary provenance data
- Brief reference to the two modes (compact/full)

## RBSGS-GettingStarted.adoc (command listing)

- Add inspect tabtargets to the command reference
- Brief usage example in the workflow section

## RBSAC, RBSAS, RBSAV (backlinks)

- Add cross-references to inspect where appropriate (conjure produces what inspect reads, summon retrieves it locally, vouch results are displayed by inspect)

## Possible new file

- Evaluate whether a dedicated RBSII-ark_inspect.adoc spec is warranted or whether the RBS0 operation section is sufficient. Follow the pattern of RBSAC/RBSAS/RBSAV if standalone.

## Acceptance

- All new colophons (rbw-RiF, rbw-Ric) appear in RBS0 colophon registry
- -about artifact contents documented accurately in RBSCB
- No broken cross-references between spec documents
- Inspect operation fully defined with linked terms in RBS0

### directory-restructure-prep (₢AUAAB) [abandoned]

**[260310-1934] abandoned**

Remember planned directory shuffles for future MVP release work:

1. Move AsciiDoc files to a veiled directory (following precedent from other kits like `Tools/cmk/vov_veiled/`, `Tools/jjk/vov_veiled/`, etc.)

2. Rename `Tools/rbw/` directory to `Tools/rbk/` (Recipe Bottle Kit)

This pace is a placeholder to capture the intent. Actual execution may require coordination with other heats touching these files.

**[260202-1944] rough**

Remember planned directory shuffles for future MVP release work:

1. Move AsciiDoc files to a veiled directory (following precedent from other kits like `Tools/cmk/vov_veiled/`, `Tools/jjk/vov_veiled/`, etc.)

2. Rename `Tools/rbw/` directory to `Tools/rbk/` (Recipe Bottle Kit)

This pace is a placeholder to capture the intent. Actual execution may require coordination with other heats touching these files.

### design-consumer-claudemd-guidance (₢AUAAG) [abandoned]

**[260313-1008] abandoned**

Design the consumer-facing CLAUDE.md guidance strategy for Recipe Bottle's open-source release.

## Problem

The development CLAUDE.md contains private information (future project hints, personal workflow details, AXLA references) that must not ship. But consumers using Claude Code need orientation to work effectively with BUK tabtargets, regime configuration, and RBK workflows.

## Strategy (settled in conversation)

- The CMK managed-section install pattern does NOT apply — consumers are outside the kit ecosystem
- Instead, README.md files that ship with the kits contain "Consumer CLAUDE.md Recommendations" sections
- These sections document the minimum context a consumer's Claude needs
- A consumer can reference the README from their CLAUDE.md (`Read Tools/buk/README.md for BUK patterns`) or copy the recommended entries directly
- No install scripts, no managed sections, no leakage surface

## Deliverables

1. **BUK README section**: "Consumer CLAUDE.md Recommendations" covering:
   - Tabtarget vocabulary (colophon/frontispiece/imprint)
   - The `cd` prohibition with rationale
   - Regime structure awareness (BURC/BURS/RBRR)
   - Key BUK workflow verbs

2. **RBK README section** (once Tools/rbk/ exists per ₢AkAAK): covering:
   - Regime prefixes and tabtarget inventory
   - Qualification workflow
   - Key RBK operation verbs

3. **prep-pr ceremony update**: Ensure the RBK prep-pr strips development CLAUDE.md while kit READMEs with consumer guidance survive

## Design Principle

README is version-controlled documentation that users read and understand. CLAUDE.md is invisible machinery. The user chooses to adopt patterns by reading the README — nothing is injected silently.

## Dependencies

- BUK README deliverable can proceed independently
- RBK README deliverable depends on ₢AkAAK (rename-rbw-to-rbk-move-lenses)
- prep-pr update depends on having an RBK prep-pr ceremony designed

**[260303-1918] rough**

Design the consumer-facing CLAUDE.md guidance strategy for Recipe Bottle's open-source release.

## Problem

The development CLAUDE.md contains private information (future project hints, personal workflow details, AXLA references) that must not ship. But consumers using Claude Code need orientation to work effectively with BUK tabtargets, regime configuration, and RBK workflows.

## Strategy (settled in conversation)

- The CMK managed-section install pattern does NOT apply — consumers are outside the kit ecosystem
- Instead, README.md files that ship with the kits contain "Consumer CLAUDE.md Recommendations" sections
- These sections document the minimum context a consumer's Claude needs
- A consumer can reference the README from their CLAUDE.md (`Read Tools/buk/README.md for BUK patterns`) or copy the recommended entries directly
- No install scripts, no managed sections, no leakage surface

## Deliverables

1. **BUK README section**: "Consumer CLAUDE.md Recommendations" covering:
   - Tabtarget vocabulary (colophon/frontispiece/imprint)
   - The `cd` prohibition with rationale
   - Regime structure awareness (BURC/BURS/RBRR)
   - Key BUK workflow verbs

2. **RBK README section** (once Tools/rbk/ exists per ₢AkAAK): covering:
   - Regime prefixes and tabtarget inventory
   - Qualification workflow
   - Key RBK operation verbs

3. **prep-pr ceremony update**: Ensure the RBK prep-pr strips development CLAUDE.md while kit READMEs with consumer guidance survive

## Design Principle

README is version-controlled documentation that users read and understand. CLAUDE.md is invisible machinery. The user chooses to adopt patterns by reading the README — nothing is injected silently.

## Dependencies

- BUK README deliverable can proceed independently
- RBK README deliverable depends on ₢AkAAK (rename-rbw-to-rbk-move-lenses)
- prep-pr update depends on having an RBK prep-pr ceremony designed

### bind-vessel-support-and-plantuml-conversion (₢AUAAW) [complete]

**[260313-1846] complete**

Add and debug bind vessel support. Convert rbev-bottle-plantuml from conjure to bind.

## Context

Bind vessels pin an external image by digest — no Dockerfile, no Cloud Build, trust is the digest pin. The concept exists in code (RBRV_VESSEL_MODE=bind, RBRV_BIND_IMAGE) but no bind vessel currently exists in the fleet. The only prior bind vessel (rbev-nginx-ward) was deleted as purposeless.

## Rationale for plantuml conversion

The current Dockerfile is a no-op wrapper:
```
FROM plantuml/plantuml-server:jetty
RUN echo "Build timestamp: $(date -u +'%Y-%m-%d %H:%M:%S UTC')" > /tmp/build-info.txt
EXPOSE 8080
```
Adds nothing meaningful — 10-20 minutes of Cloud Build + multi-arch + SBOM + vouch to stamp a date on an upstream image. Perfect bind candidate.

## Work

1. Convert rbev-bottle-plantuml from conjure to bind:
   - Change RBRV_VESSEL_MODE from conjure to bind
   - Add RBRV_BIND_IMAGE pointing to upstream plantuml/plantuml-server:jetty with digest pin
   - Remove RBRV_CONJURE_* variables
   - Delete Dockerfile (no longer needed)
2. Verify bind vessel path through the pipeline:
   - Inscription: does inscribe handle bind vessels correctly (skip, or register differently)?
   - Retrieve/summon: does the retrieval path work for digest-pinned upstream images?
   - Inspect: compact and full modes for bind vessels (should show "trust is digest-pin only")
   - Bottle start: does bottle-start work with a bind vessel image?
3. Fix any bugs found in the bind vessel codepath
4. Update consumer docs (README.consumer.md, CLAUDE.consumer.md) vessel structure example to show a real bind vessel
5. Update pluml nameplate if needed to reference the bind image correctly

## Acceptance

- rbev-bottle-plantuml is a working bind vessel
- Bind vessel end-to-end path verified: pin → retrieve → inspect → start
- At least one bind vessel exists in the fleet for testing and documentation
- Consumer docs show bind vessel example with real data

**[260313-1450] rough**

Add and debug bind vessel support. Convert rbev-bottle-plantuml from conjure to bind.

## Context

Bind vessels pin an external image by digest — no Dockerfile, no Cloud Build, trust is the digest pin. The concept exists in code (RBRV_VESSEL_MODE=bind, RBRV_BIND_IMAGE) but no bind vessel currently exists in the fleet. The only prior bind vessel (rbev-nginx-ward) was deleted as purposeless.

## Rationale for plantuml conversion

The current Dockerfile is a no-op wrapper:
```
FROM plantuml/plantuml-server:jetty
RUN echo "Build timestamp: $(date -u +'%Y-%m-%d %H:%M:%S UTC')" > /tmp/build-info.txt
EXPOSE 8080
```
Adds nothing meaningful — 10-20 minutes of Cloud Build + multi-arch + SBOM + vouch to stamp a date on an upstream image. Perfect bind candidate.

## Work

1. Convert rbev-bottle-plantuml from conjure to bind:
   - Change RBRV_VESSEL_MODE from conjure to bind
   - Add RBRV_BIND_IMAGE pointing to upstream plantuml/plantuml-server:jetty with digest pin
   - Remove RBRV_CONJURE_* variables
   - Delete Dockerfile (no longer needed)
2. Verify bind vessel path through the pipeline:
   - Inscription: does inscribe handle bind vessels correctly (skip, or register differently)?
   - Retrieve/summon: does the retrieval path work for digest-pinned upstream images?
   - Inspect: compact and full modes for bind vessels (should show "trust is digest-pin only")
   - Bottle start: does bottle-start work with a bind vessel image?
3. Fix any bugs found in the bind vessel codepath
4. Update consumer docs (README.consumer.md, CLAUDE.consumer.md) vessel structure example to show a real bind vessel
5. Update pluml nameplate if needed to reference the bind image correctly

## Acceptance

- rbev-bottle-plantuml is a working bind vessel
- Bind vessel end-to-end path verified: pin → retrieve → inspect → start
- At least one bind vessel exists in the fleet for testing and documentation
- Consumer docs show bind vessel example with real data

**[260313-1448] rough**

Add and debug bind vessel support. Convert rbev-bottle-plantuml from conjure to bind.

## Context

Bind vessels pin an external image by digest — no Dockerfile, no Cloud Build, trust is the digest pin. The concept exists in code (RBRV_VESSEL_MODE=bind, RBRV_BIND_IMAGE) but no bind vessel currently exists in the fleet. The only prior bind vessel (rbev-nginx-ward) was deleted as purposeless.

## Work

1. Convert rbev-bottle-plantuml from conjure to bind:
   - Change RBRV_VESSEL_MODE from conjure to bind
   - Add RBRV_BIND_IMAGE pointing to upstream plantuml/plantuml-server with digest pin
   - Remove RBRV_CONJURE_* variables
   - Delete Dockerfile (no longer needed)
2. Verify bind vessel path through the pipeline:
   - Inscription: does inscribe handle bind vessels correctly (skip, or register differently)?
   - Retrieve/summon: does the retrieval path work for digest-pinned upstream images?
   - Inspect: compact and full modes for bind vessels (should show "trust is digest-pin only")
   - Bottle start: does bottle-start work with a bind vessel image?
3. Fix any bugs found in the bind vessel codepath
4. Update consumer docs (README.consumer.md, CLAUDE.consumer.md) vessel structure example to show a real bind vessel
5. Update pluml nameplate if needed to reference the bind image correctly

## Acceptance

- rbev-bottle-plantuml is a working bind vessel
- Bind vessel end-to-end path verified: pin → retrieve → inspect → start
- At least one bind vessel exists in the fleet for testing and documentation
- Consumer docs show bind vessel example with real data

### mint-rbj-jailer-prefix (₢AUAAa) [complete]

**[260313-1733] complete**

Mint `rbj` (Recipe Bottle Jailer) as the prefix for sentry setup/security-rule scripts. Resolves the terminal exclusivity violation where `rbs` parents both the specification document tree (RBS0, RBSAA, RBSAC, etc.) and sentry code (`rbss_*`).

## Work

1. **Inventory**: Enumerate all `rbss_*` functions, `RBSS_*` variables, and any files using the sentry prefix under `rbs`
2. **Rename**: `rbss_*` functions → `rbj_*`, `RBSS_*` variables → `RBJ_*`, files `rbss_*.sh` → `rbj_*.sh`
3. **Update callers**: grep all references across Tools/rbk/, Tools/buk/, tt/ tabtargets, and spec documents
4. **Spec linked terms**: Update RBS0-SpecTop.adoc — rename any `:rbss_*:` attributes to `:rbj_*:`, update anchors `[[rbss_*]]` → `[[rbj_*]]`
5. **CLAUDE.md**: Add RBJ to the file acronym mappings and prefix registry
6. **Spec documents**: Update RBSSS (sentry_start), RBSSR (sentry_run), RBSSC (security_config) if they reference `rbss_*` linked terms or code identifiers
7. **Verify**: rbw-Qf passes, no broken references

## Minting Record

- Prefix: `rbj`
- Expansion: Recipe Bottle Jailer
- Domain: sentry container security setup — iptables rules, dnsmasq config, enclave network establishment
- Rationale: `rbs` was overloaded (specs + sentry), `rbj` is clean with zero existing references. "Jailer" reflects FreeBSD jail lineage and the sentry's role confining bottle network access.
- Terminal exclusivity: `rbj` is terminal (no children planned)

## Character

Intricate but mechanical — systematic find-and-replace with careful verification. No architectural changes, just prefix relocation.

## Acceptance

- Zero remaining `rbss_*` or `RBSS_*` references in active code
- `rbj_*` functions and `RBJ_*` variables work correctly
- RBS0 linked terms updated
- CLAUDE.md updated
- rbw-Qf passes
- `rbs` prefix tree contains only specification documents

**[260313-1714] rough**

Mint `rbj` (Recipe Bottle Jailer) as the prefix for sentry setup/security-rule scripts. Resolves the terminal exclusivity violation where `rbs` parents both the specification document tree (RBS0, RBSAA, RBSAC, etc.) and sentry code (`rbss_*`).

## Work

1. **Inventory**: Enumerate all `rbss_*` functions, `RBSS_*` variables, and any files using the sentry prefix under `rbs`
2. **Rename**: `rbss_*` functions → `rbj_*`, `RBSS_*` variables → `RBJ_*`, files `rbss_*.sh` → `rbj_*.sh`
3. **Update callers**: grep all references across Tools/rbk/, Tools/buk/, tt/ tabtargets, and spec documents
4. **Spec linked terms**: Update RBS0-SpecTop.adoc — rename any `:rbss_*:` attributes to `:rbj_*:`, update anchors `[[rbss_*]]` → `[[rbj_*]]`
5. **CLAUDE.md**: Add RBJ to the file acronym mappings and prefix registry
6. **Spec documents**: Update RBSSS (sentry_start), RBSSR (sentry_run), RBSSC (security_config) if they reference `rbss_*` linked terms or code identifiers
7. **Verify**: rbw-Qf passes, no broken references

## Minting Record

- Prefix: `rbj`
- Expansion: Recipe Bottle Jailer
- Domain: sentry container security setup — iptables rules, dnsmasq config, enclave network establishment
- Rationale: `rbs` was overloaded (specs + sentry), `rbj` is clean with zero existing references. "Jailer" reflects FreeBSD jail lineage and the sentry's role confining bottle network access.
- Terminal exclusivity: `rbj` is terminal (no children planned)

## Character

Intricate but mechanical — systematic find-and-replace with careful verification. No architectural changes, just prefix relocation.

## Acceptance

- Zero remaining `rbss_*` or `RBSS_*` references in active code
- `rbj_*` functions and `RBJ_*` variables work correctly
- RBS0 linked terms updated
- CLAUDE.md updated
- rbw-Qf passes
- `rbs` prefix tree contains only specification documents

### bcg-command-substitution-rule-refinement (₢AUAAc) [complete]

**[260314-1024] complete**

Design conversation: refine BCG command substitution rule.

## Context

Wave 4 compliance audit (₢AUAAZ) revealed the current BCG rule — "NO command substitution except $(<file) and _capture functions" — is over-constrictive. Several common codebase patterns are technically violations but carry no hidden-failure risk.

## Patterns That Don't Fit Current Rule

**1. _recite functions**: Return values via stdout like _capture, but the rule only exempts _capture. Callers use identical $() pattern. ~10 call sites in test dispatch (butd_dispatch.sh).

**2. Single-command $(cmd) with explicit || buc_die**: e.g. $(date +%s) || buc_die. The guard makes failure visible — the risk BCG prevents (hidden failures) doesn't apply. ~20 sites converted to temp files in waves 3-4 that didn't need conversion.

**3. Source-time constants**: Color init like ZBUC_RED=$(zbuc_color '1;31') runs before kindle — no temp file infrastructure available. ~10 sites in buc_command.sh, buto_operations.sh.

**4. Bash introspection**: $(compgen -v), $(declare -F | grep), $(mktemp) — no file-based alternative exists.

## The Three Real Dangers

The rule should prohibit these specific patterns:
- `local z_var=$(cmd)` — local swallows exit status, always
- `$(cmd1 | cmd2)` — pipeline inside $() hides intermediate failures
- Unguarded `$()` — no || buc_die, failure is silent

## Decision Points

1. Should _recite be added to the $() exception list alongside _capture?
2. Should single-command-with-guard be explicitly blessed?
3. Should source-time constants be explicitly blessed?
4. Do waves 3-4 temp-file conversions need partial revert, or are they "also fine, just verbose"?
5. How to phrase the refined rule — blanket prohibition with exceptions, or specific prohibitions?

## Acceptance

- BCG command substitution rule updated with owner approval
- Checklist section reflects refined rule
- Prose sections consistent with checklist

**[260313-2214] rough**

Design conversation: refine BCG command substitution rule.

## Context

Wave 4 compliance audit (₢AUAAZ) revealed the current BCG rule — "NO command substitution except $(<file) and _capture functions" — is over-constrictive. Several common codebase patterns are technically violations but carry no hidden-failure risk.

## Patterns That Don't Fit Current Rule

**1. _recite functions**: Return values via stdout like _capture, but the rule only exempts _capture. Callers use identical $() pattern. ~10 call sites in test dispatch (butd_dispatch.sh).

**2. Single-command $(cmd) with explicit || buc_die**: e.g. $(date +%s) || buc_die. The guard makes failure visible — the risk BCG prevents (hidden failures) doesn't apply. ~20 sites converted to temp files in waves 3-4 that didn't need conversion.

**3. Source-time constants**: Color init like ZBUC_RED=$(zbuc_color '1;31') runs before kindle — no temp file infrastructure available. ~10 sites in buc_command.sh, buto_operations.sh.

**4. Bash introspection**: $(compgen -v), $(declare -F | grep), $(mktemp) — no file-based alternative exists.

## The Three Real Dangers

The rule should prohibit these specific patterns:
- `local z_var=$(cmd)` — local swallows exit status, always
- `$(cmd1 | cmd2)` — pipeline inside $() hides intermediate failures
- Unguarded `$()` — no || buc_die, failure is silent

## Decision Points

1. Should _recite be added to the $() exception list alongside _capture?
2. Should single-command-with-guard be explicitly blessed?
3. Should source-time constants be explicitly blessed?
4. Do waves 3-4 temp-file conversions need partial revert, or are they "also fine, just verbose"?
5. How to phrase the refined rule — blanket prohibition with exceptions, or specific prohibitions?

## Acceptance

- BCG command substitution rule updated with owner approval
- Checklist section reflects refined rule
- Prose sections consistent with checklist

### bcg-compliance-audit (₢AUAAZ) [complete]

**[260314-0924] complete**

BCG correctness audit and targeted fixes for Tools/rbk/ and Tools/buk/.

## Completed

### Wave 1 (bd6eba19)
- 5 pseudo-ternary A&&B||C → if/then/else
- 11 non-POSIX [[ == ]] → test =
- ~40 [ ] → test
- 2 eval → ${!name}, 2 eval hardened with name validation
- 6 echo -e → printf '%b'

### Wave 2 (627e1c0f)
- Renamed zbuc_do_execute → zbuc_doc_mode_predicate (non-predicate in if condition, 8 sites)
- Converted [[ ]] → test in version checks (2 sites)
- Split combined local+capture into two-line pattern (bud_dispatch.sh)
- Added || buc_die to 10 unguarded mkdir/cp/rm/date commands
- Replaced head -1 with read -r builtin (2 sites in rbf_Foundry.sh)
- Converted heredoc → echo sequence (rbgp_Payor.sh)
- Replaced dirname → parameter expansion (rbgp_Payor.sh)
- Added readonly to kindle array ZRBRR_DOCKER_ENV

## Remaining: command substitution elimination

BCG rule: "NO command substitution except $(<file) and _capture functions."
~20 violations across 8 files. Each needs temp file, _capture function, or builtin replacement.

### buc_command.sh
- Line ~116: $(find ... | head -1) in buc_tabtarget — convert to temp file + read

### bud_dispatch.sh
- Lines ~109, ~137: $(find ... -print -quit) in test -n — convert to temp file + test -s
- Line ~145: $(git describe ... || echo fallback) — convert to temp file + read

### buto_operations.sh
- Line ~51: nested $(printf '%*s' "$(printf|sed|wc)" '') — refactor width calculation to temp file or _capture

### rbf_Foundry.sh
- Line ~582: $(grep|head|sed) for vessel mode — temp file + parameter expansion
- Lines ~649, ~1289: $(printf|base64 -d) for secret decoding — wrap in _capture (secret, never temp file)
- Lines ~970-980, ~1319-1324: $(git rev-parse), $(git config) — temp file + read
- Line ~2028: $(grep|head|sed) for digest extraction — temp file + parameter expansion
- Line ~2770: $(grep|head) for FROM line — temp file + read
- Line ~2849: $(printf|wc|tr) for image count — temp file + read

### rbgp_Payor.sh
- Line ~186: nested $(printf|wc) inside buc_log_args — simplify or temp file

### rbrr_cli.sh
- Line ~92: $(grep -m1) for secrets dir — temp file + read

### rbv_PodmanVM.sh
- Line ~307: $(date) for identity — temp file + read

## Also remaining: traps
- rbo.observe.sh line ~34: trap cleanup INT TERM
- rboo_observe.sh line ~93: trap ... SIGINT SIGTERM
These need architectural alternatives for long-running observation processes.

## Acceptance
- All command substitution eliminated except $(<file) and _capture
- Trap usage resolved or documented as intentional exception
- rbw-Qf passes
- Fast test suite passes (78 cases)

**[260313-2031] rough**

BCG correctness audit and targeted fixes for Tools/rbk/ and Tools/buk/.

## Completed

### Wave 1 (bd6eba19)
- 5 pseudo-ternary A&&B||C → if/then/else
- 11 non-POSIX [[ == ]] → test =
- ~40 [ ] → test
- 2 eval → ${!name}, 2 eval hardened with name validation
- 6 echo -e → printf '%b'

### Wave 2 (627e1c0f)
- Renamed zbuc_do_execute → zbuc_doc_mode_predicate (non-predicate in if condition, 8 sites)
- Converted [[ ]] → test in version checks (2 sites)
- Split combined local+capture into two-line pattern (bud_dispatch.sh)
- Added || buc_die to 10 unguarded mkdir/cp/rm/date commands
- Replaced head -1 with read -r builtin (2 sites in rbf_Foundry.sh)
- Converted heredoc → echo sequence (rbgp_Payor.sh)
- Replaced dirname → parameter expansion (rbgp_Payor.sh)
- Added readonly to kindle array ZRBRR_DOCKER_ENV

## Remaining: command substitution elimination

BCG rule: "NO command substitution except $(<file) and _capture functions."
~20 violations across 8 files. Each needs temp file, _capture function, or builtin replacement.

### buc_command.sh
- Line ~116: $(find ... | head -1) in buc_tabtarget — convert to temp file + read

### bud_dispatch.sh
- Lines ~109, ~137: $(find ... -print -quit) in test -n — convert to temp file + test -s
- Line ~145: $(git describe ... || echo fallback) — convert to temp file + read

### buto_operations.sh
- Line ~51: nested $(printf '%*s' "$(printf|sed|wc)" '') — refactor width calculation to temp file or _capture

### rbf_Foundry.sh
- Line ~582: $(grep|head|sed) for vessel mode — temp file + parameter expansion
- Lines ~649, ~1289: $(printf|base64 -d) for secret decoding — wrap in _capture (secret, never temp file)
- Lines ~970-980, ~1319-1324: $(git rev-parse), $(git config) — temp file + read
- Line ~2028: $(grep|head|sed) for digest extraction — temp file + parameter expansion
- Line ~2770: $(grep|head) for FROM line — temp file + read
- Line ~2849: $(printf|wc|tr) for image count — temp file + read

### rbgp_Payor.sh
- Line ~186: nested $(printf|wc) inside buc_log_args — simplify or temp file

### rbrr_cli.sh
- Line ~92: $(grep -m1) for secrets dir — temp file + read

### rbv_PodmanVM.sh
- Line ~307: $(date) for identity — temp file + read

## Also remaining: traps
- rbo.observe.sh line ~34: trap cleanup INT TERM
- rboo_observe.sh line ~93: trap ... SIGINT SIGTERM
These need architectural alternatives for long-running observation processes.

## Acceptance
- All command substitution eliminated except $(<file) and _capture
- Trap usage resolved or documented as intentional exception
- rbw-Qf passes
- Fast test suite passes (78 cases)

**[260313-1940] rough**

BCG correctness audit and targeted fixes for Tools/rbk/ and Tools/buk/.

## Completed

- Full BCG violation census across 63 files (~1,900 total violations)
- Categorized by risk tier and counted by file

## Scope (this pace)

Fix violations that affect correctness, portability, or security:

- **Pseudo-ternary `A && B || C`** (8 occurrences, 6 files) — wrong branch can execute
- **Non-POSIX `==` in test expressions** (43 occurrences, 20 files) — breaks in Cloud Build/POSIX
- **Replaceable `eval`** (4 occurrences, 3 files) — injection surface
- **`[ ]` instead of `test`** (27 occurrences, 27 files) — BCG style/portability
- **`echo -e`** (9 occurrences, 7 files) — portability

~91 targeted fixes across ~30 files.

## Deferred (itch after release)

- **z_ prefix on locals** (1,519 occurrences, 63 files) — naming convention, no runtime impact
- **Unbraced expansions `"$var"`** (198 occurrences, 49 files) — mechanical
- **`2>/dev/null` stderr suppression** (73 occurrences, 20 files) — needs case-by-case judgment

## Acceptance

- All pseudo-ternary, ==, eval, [ ], echo -e violations fixed
- rbw-Qf passes
- Deferred items documented in paddock with exact counts

**[260313-1651] rough**

End-to-end BCG compliance review and repair of all bash scripts in Tools/rbk/ and Tools/buk/. Audit against BCG-BashConsoleGuide.md patterns: Zeroes Theory (state space minimization), Interface Contamination Discipline (input forms), error handling, function extraction, variable discipline, quoting, etc.

## Scope

- All .sh files in Tools/rbk/ (rbf, rbga, rbgb, rbgc, rbgg, rbgi, rbgm, rbgo, rbgp, rbgu, rbi, rbob, rbq, rbv, rbdc, workbench, zipper, regime files, CLI files)
- All .sh files in Tools/buk/ (buc, bud, bug, but, buv, buw, workbench, regime files, CLI files)
- Test fixtures and testbenches

## Approach

1. Read BCG thoroughly to establish the checklist
2. Audit files systematically — group by kit, start with foundational modules (buc, bud, buv) since other files depend on their patterns
3. Fix issues in-place, notching progress per logical group
4. Run rbw-Qf after each group to ensure nothing breaks

## Out of scope

- Scripts in other kits (jjk, vok, cmk, ccck, gad) — those are stripped at release
- Spec documents (.adoc)
- Adding new BCG patterns — this is compliance with existing guide

## Acceptance

- Every .sh file in Tools/rbk/ and Tools/buk/ passes BCG review
- rbw-Qf passes
- No functional regressions

### readme-maturity-warning (₢AUAAX) [complete]

**[260314-1024] complete**

Add a prominent maturity/security warning to the top of README.consumer.md (and thus the released README.md). The project's security vision is strong but needs broader review before production trust.

## Tone

Not self-deprecating, not alarming. Honest engineering communication:
- The security architecture is intentional and well-designed
- It has not yet had sufficient independent review
- Users should understand they are early adopters contributing to hardening
- Invite security-minded contributors

## Placement

Immediately after the title/badge line, before any other content. Visually distinct (admonition box or similar markdown callout).

## Content Direction

Something like: "This project implements a rigorous container supply chain trust model. The security vision is sound but the implementation has had limited independent review. We welcome security-focused contributors and responsible disclosure."

Get the exact wording right — this is the first thing potential users read. Draft options and iterate with user before committing.

## Acceptance

- Warning is visually prominent at top of README.consumer.md
- Tone is confident but honest about maturity
- Does not undermine the project's credibility
- Invites contribution rather than discouraging adoption

**[260313-1521] rough**

Add a prominent maturity/security warning to the top of README.consumer.md (and thus the released README.md). The project's security vision is strong but needs broader review before production trust.

## Tone

Not self-deprecating, not alarming. Honest engineering communication:
- The security architecture is intentional and well-designed
- It has not yet had sufficient independent review
- Users should understand they are early adopters contributing to hardening
- Invite security-minded contributors

## Placement

Immediately after the title/badge line, before any other content. Visually distinct (admonition box or similar markdown callout).

## Content Direction

Something like: "This project implements a rigorous container supply chain trust model. The security vision is sound but the implementation has had limited independent review. We welcome security-focused contributors and responsible disclosure."

Get the exact wording right — this is the first thing potential users read. Draft options and iterate with user before committing.

## Acceptance

- Warning is visually prominent at top of README.consumer.md
- Tone is confident but honest about maturity
- Does not undermine the project's credibility
- Invites contribution rather than discouraging adoption

### regime-variable-completeness-check (₢AUAAH) [abandoned]

**[260313-1519] abandoned**

Add a release compliance check that verifies every enrolled regime variable has complete spec treatment.

REQUIRES HUMAN CONVERSATION — regime architecture and scope decisions needed.

## Context

Multiple regime files enroll variables via BCG-style buv_string_enroll and similar. Each enrolled variable should have spec treatment in the relevant .adoc document.

## External Dependencies (status unclear)

- AkAAO (Release Compliance section in RBS0) — now lives as AqAAA in stabled heat Aq (rbk-post-release-ideas). Not landed. This pace can proceed with discovery without it, but the final integration target may not exist yet.
- AlAAE (retire-rbrr-validator) — from retired heat. Status: likely landed or obsoleted. Verify before starting.

## Open Questions

1. Should the completeness check trace producers and consumers of each variable, or is 'variable name appears in a spec' sufficient?
2. Regime types serve different purposes (rbrr, rbra, rbrn, rbrp, rbrv, rbro). The checker needs to know which spec document each regime's variables should appear in. That mapping doesn't exist today.

## Discovery Step (FIRST)

Survey existing regime validation infrastructure:
- buv_* enrollment patterns across all *_regime.sh files
- How regime variables are currently documented in specs
- Identify gaps between enrolled variables and spec coverage

## Deliverables

1. Survey of existing regime validation behaviors
2. Design a completeness checker (possibly rbq_Qualify.sh integration)
3. Run the checker and catalogue any gaps found

## Not in Scope

- Fixing all discovered gaps (those become their own paces)
- Changing regime enrollment machinery

**[260313-1218] rough**

Add a release compliance check that verifies every enrolled regime variable has complete spec treatment.

REQUIRES HUMAN CONVERSATION — regime architecture and scope decisions needed.

## Context

Multiple regime files enroll variables via BCG-style buv_string_enroll and similar. Each enrolled variable should have spec treatment in the relevant .adoc document.

## External Dependencies (status unclear)

- AkAAO (Release Compliance section in RBS0) — now lives as AqAAA in stabled heat Aq (rbk-post-release-ideas). Not landed. This pace can proceed with discovery without it, but the final integration target may not exist yet.
- AlAAE (retire-rbrr-validator) — from retired heat. Status: likely landed or obsoleted. Verify before starting.

## Open Questions

1. Should the completeness check trace producers and consumers of each variable, or is 'variable name appears in a spec' sufficient?
2. Regime types serve different purposes (rbrr, rbra, rbrn, rbrp, rbrv, rbro). The checker needs to know which spec document each regime's variables should appear in. That mapping doesn't exist today.

## Discovery Step (FIRST)

Survey existing regime validation infrastructure:
- buv_* enrollment patterns across all *_regime.sh files
- How regime variables are currently documented in specs
- Identify gaps between enrolled variables and spec coverage

## Deliverables

1. Survey of existing regime validation behaviors
2. Design a completeness checker (possibly rbq_Qualify.sh integration)
3. Run the checker and catalogue any gaps found

## Not in Scope

- Fixing all discovered gaps (those become their own paces)
- Changing regime enrollment machinery

**[260310-1853] rough**

Drafted from ₢AsAAE in ₣As.

Drafted from ₢AkAAP in ₣Ak.

Add a release compliance check that verifies every enrolled regime variable has complete spec treatment.

REQUIRES HUMAN CONVERSATION — regime architecture and scope decisions needed.

## Context

Multiple regime files enroll variables via BCG-style buv_string_enroll and similar.
Each enrolled variable should have spec treatment in the relevant .adoc document.

The legacy rbrr.validator.sh (pre-BCG direct test/regex checks) is being retired
by ₢AlAAE — its two regex validations are migrating into buv_* enrollment types.
This confirms buv_* enrollment is the single source of truth for regime validation.

## Open questions for human conversation

1. **"Who sets it and who reads it"**: Should the completeness check trace producers
   and consumers of each variable, or is "variable name appears in a spec" sufficient?
   This is a scope decision — producer/consumer mapping is valuable but potentially
   a separate pace.

2. **Regime types serve different purposes**: rbrr (depot config), rbra (admin/legacy),
   rbrn (nameplate), rbrp, rbrv, rbro, etc. The checker needs to know which spec
   document each regime's variables should appear in. That mapping doesn't exist today
   and is a design decision to make during this pace.

## Discovery step (FIRST)

Survey existing regime validation infrastructure:
- buv_* enrollment patterns across all *_regime.sh files
- How regime variables are currently documented in specs
- Identify gaps between enrolled variables and spec coverage
- Minimum viable check: every enrolled variable name appears in at least one .adoc spec

## Deliverables

1. Survey of existing regime validation behaviors
2. Design a completeness checker (possibly a tabtarget or rbq_Qualify.sh integration)
3. Add the check to RBS0 Release Compliance section (nucleated by ₢AkAAO)
4. Run the checker and catalogue any gaps found

## Sequencing

Depends on ₢AkAAO (Release Compliance section must exist first).
₢AlAAE (retire-rbrr-validator) should land first — confirms buv_* as single source.

## Not in scope
- Fixing all discovered gaps (those become their own paces)
- Changing regime enrollment machinery

**[260309-1947] rough**

Drafted from ₢AkAAP in ₣Ak.

Add a release compliance check that verifies every enrolled regime variable has complete spec treatment.

REQUIRES HUMAN CONVERSATION — regime architecture and scope decisions needed.

## Context

Multiple regime files enroll variables via BCG-style buv_string_enroll and similar.
Each enrolled variable should have spec treatment in the relevant .adoc document.

The legacy rbrr.validator.sh (pre-BCG direct test/regex checks) is being retired
by ₢AlAAE — its two regex validations are migrating into buv_* enrollment types.
This confirms buv_* enrollment is the single source of truth for regime validation.

## Open questions for human conversation

1. **"Who sets it and who reads it"**: Should the completeness check trace producers
   and consumers of each variable, or is "variable name appears in a spec" sufficient?
   This is a scope decision — producer/consumer mapping is valuable but potentially
   a separate pace.

2. **Regime types serve different purposes**: rbrr (depot config), rbra (admin/legacy),
   rbrn (nameplate), rbrp, rbrv, rbro, etc. The checker needs to know which spec
   document each regime's variables should appear in. That mapping doesn't exist today
   and is a design decision to make during this pace.

## Discovery step (FIRST)

Survey existing regime validation infrastructure:
- buv_* enrollment patterns across all *_regime.sh files
- How regime variables are currently documented in specs
- Identify gaps between enrolled variables and spec coverage
- Minimum viable check: every enrolled variable name appears in at least one .adoc spec

## Deliverables

1. Survey of existing regime validation behaviors
2. Design a completeness checker (possibly a tabtarget or rbq_Qualify.sh integration)
3. Add the check to RBS0 Release Compliance section (nucleated by ₢AkAAO)
4. Run the checker and catalogue any gaps found

## Sequencing

Depends on ₢AkAAO (Release Compliance section must exist first).
₢AlAAE (retire-rbrr-validator) should land first — confirms buv_* as single source.

## Not in scope
- Fixing all discovered gaps (those become their own paces)
- Changing regime enrollment machinery

**[260304-1501] rough**

Add a release compliance check that verifies every enrolled regime variable has complete spec treatment.

REQUIRES HUMAN CONVERSATION — regime architecture and scope decisions needed.

## Context

Multiple regime files enroll variables via BCG-style buv_string_enroll and similar.
Each enrolled variable should have spec treatment in the relevant .adoc document.

The legacy rbrr.validator.sh (pre-BCG direct test/regex checks) is being retired
by ₢AlAAE — its two regex validations are migrating into buv_* enrollment types.
This confirms buv_* enrollment is the single source of truth for regime validation.

## Open questions for human conversation

1. **"Who sets it and who reads it"**: Should the completeness check trace producers
   and consumers of each variable, or is "variable name appears in a spec" sufficient?
   This is a scope decision — producer/consumer mapping is valuable but potentially
   a separate pace.

2. **Regime types serve different purposes**: rbrr (depot config), rbra (admin/legacy),
   rbrn (nameplate), rbrp, rbrv, rbro, etc. The checker needs to know which spec
   document each regime's variables should appear in. That mapping doesn't exist today
   and is a design decision to make during this pace.

## Discovery step (FIRST)

Survey existing regime validation infrastructure:
- buv_* enrollment patterns across all *_regime.sh files
- How regime variables are currently documented in specs
- Identify gaps between enrolled variables and spec coverage
- Minimum viable check: every enrolled variable name appears in at least one .adoc spec

## Deliverables

1. Survey of existing regime validation behaviors
2. Design a completeness checker (possibly a tabtarget or rbq_Qualify.sh integration)
3. Add the check to RBS0 Release Compliance section (nucleated by ₢AkAAO)
4. Run the checker and catalogue any gaps found

## Sequencing

Depends on ₢AkAAO (Release Compliance section must exist first).
₢AlAAE (retire-rbrr-validator) should land first — confirms buv_* as single source.

## Not in scope
- Fixing all discovered gaps (those become their own paces)
- Changing regime enrollment machinery

**[260304-1444] rough**

Add a release compliance check that verifies every enrolled regime variable has complete spec treatment.

REQUIRES HUMAN CONVERSATION — existing regime validation behaviors need discovery first.

## Context

Multiple regime files enroll variables via BCG-style buv_string_enroll and similar.
Each enrolled variable should have spec treatment in the relevant .adoc document.

NOTE: rbrr.validator.sh is legacy and may have been evicted. The current pattern is
buv_validation.sh enrollment functions called from *_regime.sh files. Discovery step
must confirm what actually exists.

## Open questions for human conversation

1. **"Who sets it and who reads it"**: Should the completeness check trace producers
   and consumers of each variable, or is "variable name appears in a spec" sufficient?
   This is a scope decision — producer/consumer mapping is valuable but potentially
   a separate pace.

2. **Regime types serve different purposes**: rbrr (depot config), rbra (admin/legacy),
   rbrn (nameplate), rbrp, rbrv, rbro, etc. The checker needs to know which spec
   document each regime's variables should appear in. That mapping doesn't exist today
   and is a design decision to make during this pace.

## Discovery step (FIRST)

Before designing the checker, survey existing regime validation infrastructure:
- buv_* enrollment patterns across all *_regime.sh files
- Confirm whether rbrr.validator.sh still exists or has been evicted
- How regime variables are currently documented in specs
- Identify gaps between enrolled variables and spec coverage

## Deliverables

1. Survey of existing regime validation behaviors
2. Design a completeness checker (possibly a tabtarget or rbq_Qualify.sh integration)
3. Add the check to RBS0 Release Compliance section (nucleated by ₢AkAAO)
4. Run the checker and catalogue any gaps found

## Sequencing

Depends on ₢AkAAO (Release Compliance section must exist first).

## Not in scope
- Fixing all discovered gaps (those become their own paces)
- Changing regime enrollment machinery

**[260304-1428] rough**

Add a release compliance check that verifies every enrolled regime variable has complete spec treatment.

REQUIRES HUMAN CONVERSATION — existing regime validation behaviors need discovery first.

## Context

Multiple regime files (rbrr_regime.sh, rbra_regime.sh, rbro_regime.sh, etc.) enroll
variables via buv_string_enroll and similar. Each enrolled variable should have:
- Spec definition in the relevant .adoc
- Validation rule (type, range, required/optional)
- Documentation of who sets it and who reads it

## Discovery step (FIRST)

Before designing the checker, survey existing regime validation infrastructure:
- buv_* enrollment patterns across all regime files
- Existing validators (rbrr.validator.sh, etc.)
- How regime variables are currently documented in specs
- Identify gaps between enrolled variables and spec coverage

## Deliverables

1. Survey of existing regime validation behaviors
2. Design a completeness checker (possibly a tabtarget or rbq_Qualify.sh integration)
3. Add the check to RBS0 Release Compliance section (nucleated by premises pace)
4. Run the checker and catalogue any gaps found

## Not in scope
- Fixing all discovered gaps (those become their own paces)
- Changing regime enrollment machinery

### consolidate-lenses-into-veiled (₢AUAAO) [complete]

**[260313-1250] complete**

Three .adoc files remain in lenses/ outside the vov_veiled convention: RBRN-RegimeNameplate.adoc, CRR-ConfigRegimeRequirements.adoc, RBWMBX-BuildxMultiPlatformAuth.adoc. Decide for each: move into appropriate Tools/*/vov_veiled/ directory, or delete if obsolete. Goal: eliminate lenses/ as a separate strip target so all proprietary docs live under vov_veiled/.

**[260313-1153] rough**

Three .adoc files remain in lenses/ outside the vov_veiled convention: RBRN-RegimeNameplate.adoc, CRR-ConfigRegimeRequirements.adoc, RBWMBX-BuildxMultiPlatformAuth.adoc. Decide for each: move into appropriate Tools/*/vov_veiled/ directory, or delete if obsolete. Goal: eliminate lenses/ as a separate strip target so all proprietary docs live under vov_veiled/.

### consumer-claudemd-and-test-discipline (₢AUAAN) [complete]

**[260313-1441] complete**

Create the consumer CLAUDE.md template for open-source publication.

## Status

Substantially complete. All items below are drafted and in review.

## Context

The development CLAUDE.md contains internal tooling details (acronym mappings, JJ config, prefix discipline, kit internals) that must not ship. A consumer CLAUDE.md must ship for zero-friction Claude Code orientation. AUAAG (design-consumer-claudemd-guidance) is already abandoned — this pace supersedes it.

## Consumer CLAUDE.md Contents (delivered)

- Project orientation paragraph
- Glossary of all domain terms (vessel, ark, consecration, vouch, conjure, inscribe, abjure, depot, nameplate, regime, sentry, censer, bottle, rubric)
- Role summary (payor, governor, director, retriever) with authentication methods
- cd prohibition with rationale
- Credential safety (file locations, 600 perms, never-commit rule)
- Test execution discipline: run test fixture tabtargets sequentially, never in parallel
- TabTarget system explanation with naming anatomy
- Full command reference table (~45 commands) grouped by role: Setup, Governor, Director, Retriever, Bottles, Qualification, Regimes, BUK
- Configuration regimes overview (user-configured vs managed)
- Architecture map
- Bash conventions
- Troubleshooting section
- Pointers to Tools/buk/README.md

## Work completed

1. Test execution discipline rule added to development CLAUDE.md
2. Consumer template created at Tools/rbk/vov_veiled/CLAUDE.consumer.md
3. No internal details (JJK, CMK, VOK, prefix discipline, acronym mappings)
4. No vov_veiled/ paths in consumer-facing content
5. No Rust/VOK references

## Remaining

- Final review pass for accuracy
- Verify command table completeness against zipper after beseech clarification (AUAAU)

## Acceptance

- Consumer CLAUDE.md template exists and is self-contained
- Development CLAUDE.md includes test execution discipline rule
- Template lives in vov_veiled/ (source for prep-release swap)
- Glossary covers all domain terms used in command tables
- Command table grouped by role with no internal-only commands
- No vov_veiled/ paths, no Rust references, no internal kit references

**[260313-1433] rough**

Create the consumer CLAUDE.md template for open-source publication.

## Status

Substantially complete. All items below are drafted and in review.

## Context

The development CLAUDE.md contains internal tooling details (acronym mappings, JJ config, prefix discipline, kit internals) that must not ship. A consumer CLAUDE.md must ship for zero-friction Claude Code orientation. AUAAG (design-consumer-claudemd-guidance) is already abandoned — this pace supersedes it.

## Consumer CLAUDE.md Contents (delivered)

- Project orientation paragraph
- Glossary of all domain terms (vessel, ark, consecration, vouch, conjure, inscribe, abjure, depot, nameplate, regime, sentry, censer, bottle, rubric)
- Role summary (payor, governor, director, retriever) with authentication methods
- cd prohibition with rationale
- Credential safety (file locations, 600 perms, never-commit rule)
- Test execution discipline: run test fixture tabtargets sequentially, never in parallel
- TabTarget system explanation with naming anatomy
- Full command reference table (~45 commands) grouped by role: Setup, Governor, Director, Retriever, Bottles, Qualification, Regimes, BUK
- Configuration regimes overview (user-configured vs managed)
- Architecture map
- Bash conventions
- Troubleshooting section
- Pointers to Tools/buk/README.md

## Work completed

1. Test execution discipline rule added to development CLAUDE.md
2. Consumer template created at Tools/rbk/vov_veiled/CLAUDE.consumer.md
3. No internal details (JJK, CMK, VOK, prefix discipline, acronym mappings)
4. No vov_veiled/ paths in consumer-facing content
5. No Rust/VOK references

## Remaining

- Final review pass for accuracy
- Verify command table completeness against zipper after beseech clarification (AUAAU)

## Acceptance

- Consumer CLAUDE.md template exists and is self-contained
- Development CLAUDE.md includes test execution discipline rule
- Template lives in vov_veiled/ (source for prep-release swap)
- Glossary covers all domain terms used in command tables
- Command table grouped by role with no internal-only commands
- No vov_veiled/ paths, no Rust references, no internal kit references

**[260313-1218] rough**

Create the consumer CLAUDE.md template for open-source publication.

## Context

The development CLAUDE.md contains internal tooling details (acronym mappings, JJ config, prefix discipline, kit internals) that must not ship. A minimal consumer CLAUDE.md must ship for zero-friction Claude Code orientation. AUAAG (design-consumer-claudemd-guidance) is already abandoned — this pace supersedes it.

## Consumer CLAUDE.md Contents

- cd prohibition with rationale
- Test execution discipline: run test fixture tabtargets sequentially, never in parallel (fixtures share regime state and container/network namespaces)
- Tabtarget vocabulary basics (colophon/frontispiece/imprint)
- Pointers to Tools/buk/README.md and Tools/rbk/README.md for full documentation

## Work

1. Add test execution discipline rule to development CLAUDE.md immediately
2. Create consumer template at a known location (e.g. Tools/rbk/vov_veiled/CLAUDE.consumer.md)
3. Verify content is minimal and contains no internal details

Note: the prep-release ceremony (AUAAR) handles swapping dev CLAUDE.md for the consumer template during publication. This pace only creates the template.

## Acceptance

- Consumer CLAUDE.md template exists and is self-contained
- Development CLAUDE.md includes test execution discipline rule
- Template lives in vov_veiled/ (stripped from consumer view, used only as source for swap)

**[260312-1911] rough**

Finalize the consumer CLAUDE.md publication strategy and implement test execution discipline.

## Two Problems, One Pace

1. **Test execution discipline**: Claude Code must never run test fixtures in parallel. Fixtures share regime state and container/network namespaces. Add rule to CLAUDE.md: "Run test fixture tabtargets sequentially, never in parallel. Fixtures share regime state and may share container/network resources."

2. **Consumer CLAUDE.md strategy**: The development CLAUDE.md contains secrets (acronym mappings, JJ config, prefix discipline, kit internals) that must not ship. But a minimal CLAUDE.md with agent behavioral constraints must ship for zero-friction Claude Code orientation.

## Design Decision to Finalize

Two CLAUDE.md files:
- Development CLAUDE.md (current, full, never published)
- Consumer CLAUDE.md (minimal, installed by prep-pr when pushing to open-source remote)

Consumer CLAUDE.md contains ONLY:
- cd prohibition with rationale
- Test execution discipline (sequential only)
- Tabtarget vocabulary basics
- Pointers to Tools/buk/README.md and Tools/rbk/README.md for full documentation

## Work

1. Add test execution discipline rule to development CLAUDE.md immediately
2. Decide consumer CLAUDE.md location and naming (e.g., CLAUDE.consumer.md template)
3. Implement consumer CLAUDE.md content
4. Update prep-pr ceremony to swap development to consumer CLAUDE.md on open-source push
5. Verify prep-pr strips all sensitive content

## Relationship to Existing Paces

Subsumes AUAAG (design-consumer-claudemd-guidance) -- that pace's README recommendation sections are still valid but the CLAUDE.md publication decision is settled here. Consider dropping or reslating AUAAG to cover only the README sections.

## References

- Current CLAUDE.md (development, full)
- Tools/buk/README.md, Tools/rbk/README.md (consumer documentation targets)
- prep-pr ceremony (cma-prep-pr skill)

### readme-architecture-diagram (₢AUAAY) [complete]

**[260314-1114] complete**

Add the rbm-abstract-drawio.svg architecture diagram to README.consumer.md using centered HTML img tag. The SVG already survives prep-release stripping. Place after the introductory paragraph, before Key Concepts.

**[260313-1650] rough**

Add the rbm-abstract-drawio.svg architecture diagram to README.consumer.md using centered HTML img tag. The SVG already survives prep-release stripping. Place after the introductory paragraph, before Key Concepts.

### freshen-cosmology-and-render (₢AUAAP) [complete]

**[260314-1140] complete**

Update RBSCO-CosmologyIntro.adoc and faux-render to index.html for open source project page.

## Work

1. Pull roadmap highlights from RBSHR-HorizonRoadmap.adoc into RBSCO as a forward-looking section
2. Rewrite 'Current Challenges' to present tense — GCB/GAR migration is complete, Makefiles replaced by bash
3. Fix 'Vision' section line 67: says "currently only `podman`" — must change to `docker`
4. Update Significant Events timeline through current date
5. Faux-render RBSCO to index.html (sonnet synchronization — resolve MCM attributes to plain text, produce styled HTML matching current dark-mode format)

## Stale Content (identified during consumer doc work)

RBSCO still says "will migrate to Google Cloud Build" (future tense) — this is now accomplished fact. The consumer README.consumer.md and CLAUDE.consumer.md already have the corrected framing with docker (not podman). RBSCO is now the stale document that needs to catch up.

Timeline references to podman (Aug 2024 through Mar 2025) are accurate history and should remain. But the Vision section and Current Challenges must reflect current reality.

## MCM Attribute Resolution

RBSCO uses MCM attribute references ({at_rbm_system}, {at_sentry_container}, etc.) defined in the mapping section of RBS0-SpecTop.adoc. The faux-render must resolve each attribute to its display text from that mapping section. No asciidoctor tooling available — this is a manual/sonnet synchronization job.

## Acceptance

- RBSCO accurately describes current state (cloud build as accomplished, not future)
- Vision section says docker, not podman
- index.html renders correctly in browser with current content
- Roadmap section gives consumers visibility into project direction
- All MCM attributes resolved to plain text in index.html

**[260313-1434] rough**

Update RBSCO-CosmologyIntro.adoc and faux-render to index.html for open source project page.

## Work

1. Pull roadmap highlights from RBSHR-HorizonRoadmap.adoc into RBSCO as a forward-looking section
2. Rewrite 'Current Challenges' to present tense — GCB/GAR migration is complete, Makefiles replaced by bash
3. Fix 'Vision' section line 67: says "currently only `podman`" — must change to `docker`
4. Update Significant Events timeline through current date
5. Faux-render RBSCO to index.html (sonnet synchronization — resolve MCM attributes to plain text, produce styled HTML matching current dark-mode format)

## Stale Content (identified during consumer doc work)

RBSCO still says "will migrate to Google Cloud Build" (future tense) — this is now accomplished fact. The consumer README.consumer.md and CLAUDE.consumer.md already have the corrected framing with docker (not podman). RBSCO is now the stale document that needs to catch up.

Timeline references to podman (Aug 2024 through Mar 2025) are accurate history and should remain. But the Vision section and Current Challenges must reflect current reality.

## MCM Attribute Resolution

RBSCO uses MCM attribute references ({at_rbm_system}, {at_sentry_container}, etc.) defined in the mapping section of RBS0-SpecTop.adoc. The faux-render must resolve each attribute to its display text from that mapping section. No asciidoctor tooling available — this is a manual/sonnet synchronization job.

## Acceptance

- RBSCO accurately describes current state (cloud build as accomplished, not future)
- Vision section says docker, not podman
- index.html renders correctly in browser with current content
- Roadmap section gives consumers visibility into project direction
- All MCM attributes resolved to plain text in index.html

**[260313-1218] rough**

Update RBSCO-CosmologyIntro.adoc and faux-render to index.html for open source project page.

## Work

1. Pull roadmap highlights from RBSHR-HorizonRoadmap.adoc into RBSCO as a forward-looking section
2. Rewrite 'Current Challenges' to present tense — GCB/GAR migration is complete, Makefiles replaced by bash
3. Update Significant Events timeline through current date
4. Faux-render RBSCO to index.html (sonnet synchronization — resolve MCM attributes to plain text, produce styled HTML matching current dark-mode format)

## MCM Attribute Resolution

RBSCO uses MCM attribute references ({at_rbm_system}, {at_sentry_container}, etc.) defined in the mapping section of RBS0-SpecTop.adoc. The faux-render must resolve each attribute to its display text from that mapping section. No asciidoctor tooling available — this is a manual/sonnet synchronization job.

## Acceptance

- RBSCO accurately describes current state (cloud build as accomplished, not future)
- index.html renders correctly in browser with current content
- Roadmap section gives consumers visibility into project direction
- All MCM attributes resolved to plain text in index.html

**[260313-1213] rough**

Update RBSCO-CosmologyIntro.adoc and faux-render to index.html for open source project page.

## Work

1. Pull roadmap highlights from RBSHR-HorizonRoadmap.adoc into RBSCO as a forward-looking section
2. Rewrite 'Current Challenges' to present tense — GCB/GAR migration is complete, Makefiles replaced by bash
3. Update Significant Events timeline through current date
4. Faux-render RBSCO to index.html (sonnet synchronization — resolve MCM attributes to plain text, produce styled HTML matching current dark-mode format)

## Acceptance

- RBSCO accurately describes current state (cloud build as accomplished, not future)
- index.html renders correctly in browser with current content
- Roadmap section gives consumers visibility into project direction

### getting-started-readme (₢AUAAQ) [complete]

**[260313-1454] complete**

Consumer-facing README.md for open-source publication.

## Status

Substantially complete. README.consumer.md exists at Tools/rbk/vov_veiled/ with full clone-to-bottle walkthrough.

## Context

Originally framed as faux-render of RBSGS-GettingStarted.adoc. In practice, authored directly from RBSGS source material, onboarding guide code, and spec content rather than mechanically rendered.

## README.md Contents (delivered)

- Project overview (from RBSCO)
- Key Concepts glossary (vessel, ark, consecration, vouch, depot, rubric, sentry, censer, bottle, nameplate)
- How It Works: image management + bottle orchestration
- Prerequisites with GCP expectations (credit card, OAuth consent screen)
- Using the CLI: tabtarget explanation before setup sequence
- Role summary table with authentication methods
- Adaptive onboarding guide reference (with BUK regime prerequisite note)
- 15-step setup sequence across 4 phases with exact commands
- Day-to-day operations (start/connect/stop/inspect) with nsproto as example vessel
- Credential safety section with file location table
- Configuration regime reference (user-configured vs managed vs BUK base)
- Vessel directory structure example
- Testing discipline (sequential, never parallel)
- Recovery section (6 scenarios)
- Architecture tree
- Claude Code reference pointing to CLAUDE.md

## Remaining

- Live walkthrough test: verify setup steps match actual command behavior
- Confirm rbev-vessels/ structure matches what ships after prep-release strip
- Verify nsproto vessel is included in consumer repo
- Update after beseech clarification (AUAAU) if beseech is restored to consumer docs

## Acceptance

- README.md is self-contained markdown, no adoc dependencies
- New user can follow README from clone to working bottle
- Payor and onboarding steps are clear and sequenced
- Tabtarget names are concrete (not MCM attribute references)
- Recovery section covers all credential loss scenarios
- nsproto identified as test vessel with guidance on custom vessels
- Rubric repo security boundary concept explained
- Credential safety documented with file paths and permissions
- Vessel directory structure shown with example

**[260313-1434] rough**

Consumer-facing README.md for open-source publication.

## Status

Substantially complete. README.consumer.md exists at Tools/rbk/vov_veiled/ with full clone-to-bottle walkthrough.

## Context

Originally framed as faux-render of RBSGS-GettingStarted.adoc. In practice, authored directly from RBSGS source material, onboarding guide code, and spec content rather than mechanically rendered.

## README.md Contents (delivered)

- Project overview (from RBSCO)
- Key Concepts glossary (vessel, ark, consecration, vouch, depot, rubric, sentry, censer, bottle, nameplate)
- How It Works: image management + bottle orchestration
- Prerequisites with GCP expectations (credit card, OAuth consent screen)
- Using the CLI: tabtarget explanation before setup sequence
- Role summary table with authentication methods
- Adaptive onboarding guide reference (with BUK regime prerequisite note)
- 15-step setup sequence across 4 phases with exact commands
- Day-to-day operations (start/connect/stop/inspect) with nsproto as example vessel
- Credential safety section with file location table
- Configuration regime reference (user-configured vs managed vs BUK base)
- Vessel directory structure example
- Testing discipline (sequential, never parallel)
- Recovery section (6 scenarios)
- Architecture tree
- Claude Code reference pointing to CLAUDE.md

## Remaining

- Live walkthrough test: verify setup steps match actual command behavior
- Confirm rbev-vessels/ structure matches what ships after prep-release strip
- Verify nsproto vessel is included in consumer repo
- Update after beseech clarification (AUAAU) if beseech is restored to consumer docs

## Acceptance

- README.md is self-contained markdown, no adoc dependencies
- New user can follow README from clone to working bottle
- Payor and onboarding steps are clear and sequenced
- Tabtarget names are concrete (not MCM attribute references)
- Recovery section covers all credential loss scenarios
- nsproto identified as test vessel with guidance on custom vessels
- Rubric repo security boundary concept explained
- Credential safety documented with file paths and permissions
- Vessel directory structure shown with example

**[260313-1218] rough**

Faux-render RBSGS-GettingStarted.adoc into a consumer-facing README.md.

## Work

1. Resolve MCM attribute references to plain text (e.g. {rbtgo_payor_establish} becomes the tabtarget name tt/rbw-gPE.PayorEstablish.sh or its operation description)
2. Suppress adoc-specific patterns (cross-references, attribute definitions)
3. Integrate with onboarding workflow: RBSGS references rbw-gO (onboarding tabtarget, backed by rbgm_onboarding in rbgm_cli.sh) which is a guided manual procedure. The README should make this the clear entry point after clone.
4. Walk a new user from clone through: payor establish (manual console, rbw-gPE) -> payor install (OAuth, rbw-gPI) -> depot create (rbw-PC) -> governor reset (rbw-PG) -> credential creation -> first bottle-start
5. Consider what level of detail belongs in README vs separate docs
6. Place README.md at repo root for open source publication

## MCM Attribute Resolution

Same as AUAAP: attributes defined in RBS0-SpecTop.adoc mapping section. Manual/sonnet synchronization job.

## Acceptance

- README.md is self-contained markdown, no adoc dependencies
- New user can follow README from clone to working bottle
- Payor and onboarding steps are clear and sequenced
- Tabtarget names are concrete (not MCM attribute references)

**[260313-1213] rough**

Faux-render RBSGS-GettingStarted.adoc into a consumer-facing README.md.

## Work

1. Resolve MCM attribute references to plain text (e.g. {rbtgo_payor_establish} becomes the actual tabtarget name or operation description)
2. Suppress adoc-specific patterns (cross-references, attribute definitions)
3. Integrate with onboarding process — the README should walk a new user from clone through payor establish, depot create, and first bottle-start
4. Consider what level of detail belongs in README vs separate docs
5. Place README.md at repo root for open source publication

## Acceptance

- README.md is self-contained markdown, no adoc dependencies
- New user can follow README from clone to working bottle
- Payor and onboarding steps are clear and sequenced

### prep-release-ceremony-refinements (₢AUAAd) [abandoned]

**[260314-1317] abandoned**

Refinements to /rbk-prep-release slash command identified during ₢AUAAR work:

## 1. External command audit step (LLM task)

Add a ceremony step after stripping that scans surviving .sh files for external command invocations beyond declared dependencies. Excludes cloudbuild scripts (Tools/rbk/rbgja/, rbgjb/, rbgjv/) since those run on GCB, not the user's workstation.

**Declared dependencies** (index.html promise): bash, git, curl, ssh/scp/ssh-keygen, jq, docker

**Assumed POSIX/coreutils** (any system with bash): chmod, cp, mv, rm, mkdir, mktemp, date, sleep, cat, grep, sed, awk, sort, head, tail, wc, tee, touch, ln, tr, cut, printf, test, true, false, kill

The audit flags anything not in either tier. Catches gcloud, podman, python, shellcheck, column, openssl, base64, shasum, etc.

## 2. Fix marshal reset invocation after strip

Step 10 references tt/rbw-MR.MarshalReset.sh which gets stripped in step 8c. Fix: invoke Tools/rbk/rblm_cli.sh directly instead of via the tabtarget, or reorder to run marshal reset before stripping the marshal tabtarget.

## 3. RBRG pin refresh coupling note

RBRG pin refresh and validate/render operations live in rbrr_cli.sh despite being a different regime. Not blocking for release but worth noting as future cleanup (itch candidate).

## References
- .claude/commands/rbk-prep-release.md (the slash command)
- Tools/rbk/rblm_cli.sh (lifecycle marshal)
- index.html lines 211-213 (dependency claim)
- BCG (no precise external command list exists today — audit defines one)

**[260314-1302] rough**

Refinements to /rbk-prep-release slash command identified during ₢AUAAR work:

## 1. External command audit step (LLM task)

Add a ceremony step after stripping that scans surviving .sh files for external command invocations beyond declared dependencies. Excludes cloudbuild scripts (Tools/rbk/rbgja/, rbgjb/, rbgjv/) since those run on GCB, not the user's workstation.

**Declared dependencies** (index.html promise): bash, git, curl, ssh/scp/ssh-keygen, jq, docker

**Assumed POSIX/coreutils** (any system with bash): chmod, cp, mv, rm, mkdir, mktemp, date, sleep, cat, grep, sed, awk, sort, head, tail, wc, tee, touch, ln, tr, cut, printf, test, true, false, kill

The audit flags anything not in either tier. Catches gcloud, podman, python, shellcheck, column, openssl, base64, shasum, etc.

## 2. Fix marshal reset invocation after strip

Step 10 references tt/rbw-MR.MarshalReset.sh which gets stripped in step 8c. Fix: invoke Tools/rbk/rblm_cli.sh directly instead of via the tabtarget, or reorder to run marshal reset before stripping the marshal tabtarget.

## 3. RBRG pin refresh coupling note

RBRG pin refresh and validate/render operations live in rbrr_cli.sh despite being a different regime. Not blocking for release but worth noting as future cleanup (itch candidate).

## References
- .claude/commands/rbk-prep-release.md (the slash command)
- Tools/rbk/rblm_cli.sh (lifecycle marshal)
- index.html lines 211-213 (dependency claim)
- BCG (no precise external command list exists today — audit defines one)

### clarify-beseech-purpose-and-docs (₢AUAAU) [complete]

**[260313-1505] complete**

The beseech command (rbw-DB, DirectorBeseechesArk) has an unclear purpose — how does it differ from summon and retrieve? Investigate rbf_beseech implementation, clarify its role, update its frontispiece/description if needed, and fix any references in README.consumer.md, CLAUDE.consumer.md, and relevant specs (RBSAB-ark_beseech.adoc).

## Context

Beseech is currently excluded from both consumer CLAUDE.md and README.consumer.md pending this investigation. Resolution should either:
- Restore beseech to consumer docs with clear differentiation from summon/retrieve, or
- Remove the tabtarget entirely if it serves no distinct consumer purpose

The prep-release ceremony (AUAAR) lists this pace as a prerequisite — beseech status must be resolved before release.

## Work

1. Read rbf_beseech implementation in rbf_Foundry.sh
2. Read RBSAB-ark_beseech.adoc spec
3. Determine: what does beseech do that summon and retrieve do not?
4. If distinct purpose exists: update frontispiece/description for clarity, restore to consumer docs
5. If no distinct purpose: consider removal or merge, update specs accordingly

## Acceptance

- Beseech purpose is clearly documented or command is removed
- Consumer CLAUDE.md and README.consumer.md are updated accordingly
- RBSAB-ark_beseech.adoc reflects current reality

**[260313-1436] rough**

The beseech command (rbw-DB, DirectorBeseechesArk) has an unclear purpose — how does it differ from summon and retrieve? Investigate rbf_beseech implementation, clarify its role, update its frontispiece/description if needed, and fix any references in README.consumer.md, CLAUDE.consumer.md, and relevant specs (RBSAB-ark_beseech.adoc).

## Context

Beseech is currently excluded from both consumer CLAUDE.md and README.consumer.md pending this investigation. Resolution should either:
- Restore beseech to consumer docs with clear differentiation from summon/retrieve, or
- Remove the tabtarget entirely if it serves no distinct consumer purpose

The prep-release ceremony (AUAAR) lists this pace as a prerequisite — beseech status must be resolved before release.

## Work

1. Read rbf_beseech implementation in rbf_Foundry.sh
2. Read RBSAB-ark_beseech.adoc spec
3. Determine: what does beseech do that summon and retrieve do not?
4. If distinct purpose exists: update frontispiece/description for clarity, restore to consumer docs
5. If no distinct purpose: consider removal or merge, update specs accordingly

## Acceptance

- Beseech purpose is clearly documented or command is removed
- Consumer CLAUDE.md and README.consumer.md are updated accordingly
- RBSAB-ark_beseech.adoc reflects current reality

**[260313-1422] rough**

The beseech command (rbw-DB, DirectorBeseechesArk) has an unclear purpose — how does it differ from summon and retrieve? Investigate rbf_beseech implementation, clarify its role, update its frontispiece/description if needed, and fix any references in README.consumer.md, CLAUDE.consumer.md, and relevant specs (RBSAB-ark_beseech.adoc).

### simplify-manifest-digest-extraction (₢AUAAF) [complete]

**[260313-1515] complete**

Simplify GCB pin digest extraction by replacing branching jq with normalize-then-filter.

## Problem

The current jq in rbrr_cli.sh:rbrr_refresh_gcb_pins uses --verbose output which wraps
manifests in a Descriptor object and returns either an array (multi-arch) or object
(single-arch), requiring if/else branching logic. This complexity caused an architecture
mismatch bug (arm64 digest pinned instead of amd64).

## Discovery (from testing)

--verbose CANNOT be removed. Two of the 7 images (gcr.io/cloud-builders/gcloud,
gcr.io/cloud-builders/docker) are single-platform — they have no manifest list.
Without --verbose, there is no way to obtain their digest from docker manifest inspect.
The remaining 5 images are multi-arch with manifest lists.

## Solution

Keep --verbose but replace the branching jq with a normalize-then-filter pipeline:

first([.] | flatten | .[].Descriptor
  | select(.platform.architecture == $arch and .platform.os == $os)
  | .digest)

How it works: [.] | flatten normalizes both array (multi-arch) and object
(single-platform) to a uniform array. Then one filter path walks .Descriptor,
selects by platform, extracts .digest. No branching.

## Validated (2026-03-13)

Tested new jq against all 7 images. Results match old jq for both single-platform
(gcloud, docker) and multi-arch (oras, alpine, syft, binfmt, skopeo) images.
Docker Hub rate limiting prevented full sweep in one pass but each image type
was confirmed individually.

## Work

1. Replace jq block (lines 345-352) with the normalize-then-filter pipeline
2. Update log message to reflect simplified extraction
3. Update error message (mention single-platform vs multi-arch diagnostic)
4. Run full rbrr_refresh_gcb_pins to verify end-to-end (requires Docker Hub quota)

## Acceptance

- All 7 images resolve to correct linux/amd64 digests
- jq extraction is a single clean pipeline, no if/else branching
- --verbose flag retained (required for single-platform images)

**[260313-1033] rough**

Simplify GCB pin digest extraction by replacing branching jq with normalize-then-filter.

## Problem

The current jq in rbrr_cli.sh:rbrr_refresh_gcb_pins uses --verbose output which wraps
manifests in a Descriptor object and returns either an array (multi-arch) or object
(single-arch), requiring if/else branching logic. This complexity caused an architecture
mismatch bug (arm64 digest pinned instead of amd64).

## Discovery (from testing)

--verbose CANNOT be removed. Two of the 7 images (gcr.io/cloud-builders/gcloud,
gcr.io/cloud-builders/docker) are single-platform — they have no manifest list.
Without --verbose, there is no way to obtain their digest from docker manifest inspect.
The remaining 5 images are multi-arch with manifest lists.

## Solution

Keep --verbose but replace the branching jq with a normalize-then-filter pipeline:

first([.] | flatten | .[].Descriptor
  | select(.platform.architecture == $arch and .platform.os == $os)
  | .digest)

How it works: [.] | flatten normalizes both array (multi-arch) and object
(single-platform) to a uniform array. Then one filter path walks .Descriptor,
selects by platform, extracts .digest. No branching.

## Validated (2026-03-13)

Tested new jq against all 7 images. Results match old jq for both single-platform
(gcloud, docker) and multi-arch (oras, alpine, syft, binfmt, skopeo) images.
Docker Hub rate limiting prevented full sweep in one pass but each image type
was confirmed individually.

## Work

1. Replace jq block (lines 345-352) with the normalize-then-filter pipeline
2. Update log message to reflect simplified extraction
3. Update error message (mention single-platform vs multi-arch diagnostic)
4. Run full rbrr_refresh_gcb_pins to verify end-to-end (requires Docker Hub quota)

## Acceptance

- All 7 images resolve to correct linux/amd64 digests
- jq extraction is a single clean pipeline, no if/else branching
- --verbose flag retained (required for single-platform images)

**[260213-0808] rough**

Simplify GCB pin digest extraction by dropping --verbose from docker manifest inspect.

## Problem

The current jq in rbrr_cli.sh:rbrr_refresh_gcb_pins uses --verbose output which wraps
manifests in a Descriptor object and returns either an array (multi-arch) or object
(single-arch), requiring branching logic. This complexity caused an architecture
mismatch bug (arm64 digest pinned instead of amd64).

## Work

1. Remove --verbose from the docker manifest inspect call
2. Replace the jq extraction with the simpler manifest list format:
   `.manifests[] | select(.platform.architecture == $arch and .platform.os == $os) | .digest`
3. Handle the single-manifest case (no .manifests array) if any images lack a manifest list
4. Verify all 8 images produce correct linux/amd64 digests by comparing against
   the forensic temp files from a --verbose run

## Acceptance

- All 8 GCB images resolve to the same linux/amd64 digests as the current platform-filtered approach
- jq extraction is a single clean pipeline, no array-vs-object branching
- --verbose flag removed from docker manifest inspect

### inscribe-dirty-tree-guard (₢AUAAV) [complete]

**[260313-1516] complete**

Add a dirty-working-tree guard to rbf_rubric_inscribe. Fail early with a clear message if `git diff --quiet` or `git diff --cached --quiet` fails, so the rubric repo's recorded source commit always corresponds exactly to the scripts being inscribed. Without this, inscribed scripts can diverge from any committed state, breaking the paper trail from rubric to source.

**[260313-1440] rough**

Add a dirty-working-tree guard to rbf_rubric_inscribe. Fail early with a clear message if `git diff --quiet` or `git diff --cached --quiet` fails, so the rubric repo's recorded source commit always corresponds exactly to the scripts being inscribed. Without this, inscribed scripts can diverge from any committed state, breaking the paper trail from rubric to source.

### extract-rbrg-cli-from-rbrr (₢AUAAe) [complete]

**[260314-1519] complete**

Extract RBRG pin management functions from rbrr_cli.sh into new rbrg_cli.sh module.

## Functions to move

- zrbrr_write_rbrg → zrbrg_write_rbrg
- rbrr_validate_pins → rbrg_validate_pins
- rbrr_render_pins → rbrg_render_pins
- rbrr_refresh_gcb_pins → rbrg_refresh_gcb_pins
- rbrr_refresh_binary_pins → rbrg_refresh_binary_pins

## Shared state to move

- ZRBRR_IMAGE_PINS_REFRESHED_AT → ZRBRG_IMAGE_PINS_REFRESHED_AT
- ZRBRR_BINARY_PINS_REFRESHED_AT → ZRBRG_BINARY_PINS_REFRESHED_AT

## Wiring

- Create Tools/rbk/rbrg_cli.sh with standard module structure (multiple-inclusion guard, kindle, sentinel)
- Update Tools/rbk/rbz_zipper.sh to route RBRG colophons to new module
- Verify no remaining RBRG references in rbrr_cli.sh
- Run fast qualification to verify wiring

## Character

Mechanical extraction — self-contained functions, no design judgment needed.

**[260314-1342] rough**

Extract RBRG pin management functions from rbrr_cli.sh into new rbrg_cli.sh module.

## Functions to move

- zrbrr_write_rbrg → zrbrg_write_rbrg
- rbrr_validate_pins → rbrg_validate_pins
- rbrr_render_pins → rbrg_render_pins
- rbrr_refresh_gcb_pins → rbrg_refresh_gcb_pins
- rbrr_refresh_binary_pins → rbrg_refresh_binary_pins

## Shared state to move

- ZRBRR_IMAGE_PINS_REFRESHED_AT → ZRBRG_IMAGE_PINS_REFRESHED_AT
- ZRBRR_BINARY_PINS_REFRESHED_AT → ZRBRG_BINARY_PINS_REFRESHED_AT

## Wiring

- Create Tools/rbk/rbrg_cli.sh with standard module structure (multiple-inclusion guard, kindle, sentinel)
- Update Tools/rbk/rbz_zipper.sh to route RBRG colophons to new module
- Verify no remaining RBRG references in rbrr_cli.sh
- Run fast qualification to verify wiring

## Character

Mechanical extraction — self-contained functions, no design judgment needed.

### remove-director-gate-from-foundry-kindle (₢AUAAn) [abandoned]

**[260331-1421] abandoned**

## Character
Surgical refactor — move credential file-existence check from module kindle to individual command functions. Testable on a machine with retriever-only credentials.

## Problem
`zrbf_kindle` (rbf_Foundry.sh:40-41) gates on `RBDC_DIRECTOR_RBRA_FILE` existence at module init. This blocks all 18 foundry commands — including `rbf_retrieve` and `rbf_summon` which only need retriever credentials. A retriever-only station (e.g., cerebro) cannot pull images.

## Scope
- Remove director file-existence check from `zrbf_kindle` (lines 40-41)
- Add `test -f "${RBDC_DIRECTOR_RBRA_FILE}"` guard to each of the 16 director-dependent commands at point of use
- Verify `rbf_retrieve` and `rbf_summon` work with retriever credential only (no director on disk)
- Test on cerebro: `tt/rbw-Rs.RetrieverSummonsConsecration.sh` should succeed without `rbra-director.env`

## Acceptance
- Summon/retrieve work on a station with only retriever credentials
- Director commands fail clearly if director credential is missing
- No behavioral change when both credentials are present

**[260331-0754] rough**

## Character
Surgical refactor — move credential file-existence check from module kindle to individual command functions. Testable on a machine with retriever-only credentials.

## Problem
`zrbf_kindle` (rbf_Foundry.sh:40-41) gates on `RBDC_DIRECTOR_RBRA_FILE` existence at module init. This blocks all 18 foundry commands — including `rbf_retrieve` and `rbf_summon` which only need retriever credentials. A retriever-only station (e.g., cerebro) cannot pull images.

## Scope
- Remove director file-existence check from `zrbf_kindle` (lines 40-41)
- Add `test -f "${RBDC_DIRECTOR_RBRA_FILE}"` guard to each of the 16 director-dependent commands at point of use
- Verify `rbf_retrieve` and `rbf_summon` work with retriever credential only (no director on disk)
- Test on cerebro: `tt/rbw-Rs.RetrieverSummonsConsecration.sh` should succeed without `rbra-director.env`

## Acceptance
- Summon/retrieve work on a station with only retriever credentials
- Director commands fail clearly if director credential is missing
- No behavioral change when both credentials are present

### enshrine-spec-realignment (₢AUAAy) [complete]

**[260407-2236] complete**

## Character
Mechanical — spec text corrections plus one small code extraction. No judgment calls.

## Problem
Spot check of `rbfd_enshrine` against RBS0/RBSAE found 5 places where the spec wording doesn't match the existing implementation. Additionally, enshrine submits a GCB job that depends on reliquary tool images but lacks the GAR preflight check that conjure already has via `zrbfd_registry_preflight`.

## Deliverable A: Spec text corrections (RBS0 + RBSAE)

All tightening wording to match existing code — no behavior changes.

1. **RBS0:1133** — "pulls each upstream image by tag, pushes to GAR" → "submits a Cloud Build job that copies each upstream image" (code submits GCB job, skopeo runs on GCB not locally)
2. **RBSAE:50-52** — "Inspects upstream image to obtain the manifest digest" → clarify digest is computed locally from raw manifest bytes via `openssl dgst -sha256` of `skopeo inspect --raw` output, not retrieved from registry
3. **RBSAE:53** — "sanitize origin tag" → "sanitize full origin string" (code runs `tr ':/' '--'` on the entire ORIGIN, not just the tag portion)
4. **RBSAE typed_parameter block** — Add `{rbbc_require}` vessel mode is `conjure` (code gates on `RBRV_VESSEL_MODE=conjure` at line 762)
5. **RBSAE typed_parameter block** — Add `{rbbc_require}` reliquary tool images exist in GAR (preflight HEAD check)

## Deliverable B: Factor reliquary preflight for enshrine

Extract reliquary canary check (lines 196-239 of `zrbfd_registry_preflight`) into `zrbfd_preflight_reliquary(token, vessel_dir)`. Call from:
- `rbfd_enshrine` — after Director auth, before `zrbfd_enshrine_submit`
- `zrbfd_registry_preflight` — replacing inline layer 1 block, then continuing to layer 2 anchor checks

Same HEAD-request-on-docker:latest pattern, same 404 remediation output. No new logic.

## References
- Tools/rbk/rbfd_FoundryDirectorBuild.sh — `rbfd_enshrine()` (750), `zrbfd_registry_preflight()` (188)
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — lines 1119-1135
- Tools/rbk/vov_veiled/RBSAE-ark_enshrine.adoc — full include
- Tools/rbk/rbgje/rbgje01-enshrine-copy.sh — GCB step script (source of truth for digest/anchor logic)

**[260401-1337] rough**

## Character
Mechanical — spec text corrections plus one small code extraction. No judgment calls.

## Problem
Spot check of `rbfd_enshrine` against RBS0/RBSAE found 5 places where the spec wording doesn't match the existing implementation. Additionally, enshrine submits a GCB job that depends on reliquary tool images but lacks the GAR preflight check that conjure already has via `zrbfd_registry_preflight`.

## Deliverable A: Spec text corrections (RBS0 + RBSAE)

All tightening wording to match existing code — no behavior changes.

1. **RBS0:1133** — "pulls each upstream image by tag, pushes to GAR" → "submits a Cloud Build job that copies each upstream image" (code submits GCB job, skopeo runs on GCB not locally)
2. **RBSAE:50-52** — "Inspects upstream image to obtain the manifest digest" → clarify digest is computed locally from raw manifest bytes via `openssl dgst -sha256` of `skopeo inspect --raw` output, not retrieved from registry
3. **RBSAE:53** — "sanitize origin tag" → "sanitize full origin string" (code runs `tr ':/' '--'` on the entire ORIGIN, not just the tag portion)
4. **RBSAE typed_parameter block** — Add `{rbbc_require}` vessel mode is `conjure` (code gates on `RBRV_VESSEL_MODE=conjure` at line 762)
5. **RBSAE typed_parameter block** — Add `{rbbc_require}` reliquary tool images exist in GAR (preflight HEAD check)

## Deliverable B: Factor reliquary preflight for enshrine

Extract reliquary canary check (lines 196-239 of `zrbfd_registry_preflight`) into `zrbfd_preflight_reliquary(token, vessel_dir)`. Call from:
- `rbfd_enshrine` — after Director auth, before `zrbfd_enshrine_submit`
- `zrbfd_registry_preflight` — replacing inline layer 1 block, then continuing to layer 2 anchor checks

Same HEAD-request-on-docker:latest pattern, same 404 remediation output. No new logic.

## References
- Tools/rbk/rbfd_FoundryDirectorBuild.sh — `rbfd_enshrine()` (750), `zrbfd_registry_preflight()` (188)
- Tools/rbk/vov_veiled/RBS0-SpecTop.adoc — lines 1119-1135
- Tools/rbk/vov_veiled/RBSAE-ark_enshrine.adoc — full include
- Tools/rbk/rbgje/rbgje01-enshrine-copy.sh — GCB step script (source of truth for digest/anchor logic)

### cloud-build-wall-clock-timing (₢AUAAz) [complete]

**[260407-2236] complete**

## Character
Mechanical plumbing — extracting data already present in a fetched JSON response. No design judgment.

## Problem
Cloud Build returns `startTime` and `finishTime` in every build status response. The polling loop in `rbfc_FoundryCore.sh` already fetches this JSON but only extracts `.status`, discarding the timing fields. There is no way to see build wall-clock duration after a cloud build completes.

## Deliverable: Surface build wall-clock timing in terminal output

After `zrbfc_wait_build_completion` detects terminal success, extract `.startTime` and `.finishTime` from the already-fetched `ZRBFC_BUILD_STATUS_FILE`, compute duration, and display via `buc_info`. Write timestamps to `ZRBFC_BUILD_START_FILE` / `ZRBFC_BUILD_FINISH_FILE` kindle constants for optional downstream use.

This covers all cloud build paths (conjure, enshrine, mirror, graft about+vouch, standalone about, standalone vouch) automatically — they all funnel through `zrbfc_wait_build_completion`.

### Implementation
- Two new kindle constants in `zrbfc_kindle()`: `ZRBFC_BUILD_START_FILE`, `ZRBFC_BUILD_FINISH_FILE`
- After success check in `zrbfc_wait_build_completion()`: jq extract, portable date conversion (GNU then BSD fallback), duration computation, `buc_info` display
- Graceful degradation: if timestamps missing or date conversion fails, skip silently

### Scope boundary
- Terminal display only — no about artifact changes, no Python changes, no GCB substitution threading
- The original docket proposed threading timing into `build_info.json` via GCB substitutions, but this is architecturally impossible for conjure/mirror (about step runs inside the same build whose finishTime doesn't exist yet). The simpler approach surfaces the same information where the operator actually sees it.

## Files modified
- `Tools/rbk/rbfc_FoundryCore.sh` — kindle constants + timing extraction in wait function

## References
- `Tools/rbk/rbfc_FoundryCore.sh` — `zrbfc_wait_build_completion()`, `zrbfc_kindle()`

**[260407-2236] rough**

## Character
Mechanical plumbing — extracting data already present in a fetched JSON response. No design judgment.

## Problem
Cloud Build returns `startTime` and `finishTime` in every build status response. The polling loop in `rbfc_FoundryCore.sh` already fetches this JSON but only extracts `.status`, discarding the timing fields. There is no way to see build wall-clock duration after a cloud build completes.

## Deliverable: Surface build wall-clock timing in terminal output

After `zrbfc_wait_build_completion` detects terminal success, extract `.startTime` and `.finishTime` from the already-fetched `ZRBFC_BUILD_STATUS_FILE`, compute duration, and display via `buc_info`. Write timestamps to `ZRBFC_BUILD_START_FILE` / `ZRBFC_BUILD_FINISH_FILE` kindle constants for optional downstream use.

This covers all cloud build paths (conjure, enshrine, mirror, graft about+vouch, standalone about, standalone vouch) automatically — they all funnel through `zrbfc_wait_build_completion`.

### Implementation
- Two new kindle constants in `zrbfc_kindle()`: `ZRBFC_BUILD_START_FILE`, `ZRBFC_BUILD_FINISH_FILE`
- After success check in `zrbfc_wait_build_completion()`: jq extract, portable date conversion (GNU then BSD fallback), duration computation, `buc_info` display
- Graceful degradation: if timestamps missing or date conversion fails, skip silently

### Scope boundary
- Terminal display only — no about artifact changes, no Python changes, no GCB substitution threading
- The original docket proposed threading timing into `build_info.json` via GCB substitutions, but this is architecturally impossible for conjure/mirror (about step runs inside the same build whose finishTime doesn't exist yet). The simpler approach surfaces the same information where the operator actually sees it.

## Files modified
- `Tools/rbk/rbfc_FoundryCore.sh` — kindle constants + timing extraction in wait function

## References
- `Tools/rbk/rbfc_FoundryCore.sh` — `zrbfc_wait_build_completion()`, `zrbfc_kindle()`

**[260402-1013] rough**

## Character
Mechanical plumbing — threading existing data through an established pipeline. No design judgment.

## Problem
Cloud Build returns `startTime` and `finishTime` in every build status response. The polling loop in `rbfc_FoundryCore.sh` already fetches this JSON but only extracts `.status`, discarding the timing fields. There is no way to determine build wall-clock duration from the vouch or about artifacts.

## Deliverable: Surface build timing in about artifact for cloud ordain paths

Capture Cloud Build start/finish times from the polling response and thread them into the about artifact's `build_info.json`. This covers all cloud-based ordain paths (conjure, bind, graft) but explicitly excludes kludge (local `podman build`, no Cloud Build API).

### Implementation path

**1. Extract timing from polling response** (`rbfc_FoundryCore.sh`, `zrbfc_wait_build_completion`):
After the terminal status is detected, extract `startTime` and `finishTime` from the already-fetched build status JSON. Write to known file paths (sibling to existing `ZRBFC_BUILD_ID_FILE` pattern).

**2. Pass as substitution variables** (`rbfd_FoundryDirectorBuild.sh`, stitching functions):
Thread the captured timestamps as Cloud Build substitution variables (`_RBGA_BUILD_START_TIME`, `_RBGA_BUILD_FINISH_TIME`) into the about generation step. This follows the existing pattern for passing `_RBGA_INSCRIBE_TIMESTAMP` and other metadata.

**3. Include in build_info.json** (`rbgja03-build-info-per-platform.py`):
Read the new substitution variables from environment, add `build_start_time` and `build_finish_time` fields to the per-platform `build_info.json`. Compute and include `build_duration_seconds` as a convenience.

### Files to modify
- `Tools/rbk/rbfc_FoundryCore.sh` — extract timing after poll completion
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh` — pass timing as substitution vars
- `Tools/rbk/rbgja/rbgja03-build-info-per-platform.py` — emit timing in build_info

### Scope boundary
- Cloud ordain paths only (conjure, bind, graft). Not kludge.
- No spec changes — this is additive metadata, not a contract change.
- No changes to vouch pipeline — timing lives in about, vouch gates on about as before.

## References
- `Tools/rbk/rbfc_FoundryCore.sh` — `zrbfc_wait_build_completion()`, polling loop
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh` — stitching and substitution variable assembly
- `Tools/rbk/rbgja/rbgja03-build-info-per-platform.py` — about metadata assembly
- Cloud Build API: `metadata.build.startTime`, `metadata.build.finishTime` fields

### relocate-rbro-to-secrets-dir (₢AUAA0) [abandoned]

**[260415-1525] abandoned**

## Character

Mechanical refactoring with spec-level implications. Move RBRO storage from user home directory to RBRR_SECRETS_DIR alongside RBRA credentials.

## Docket

RBRO (Payor OAuth credentials — client secret and refresh token) currently lives at `~/.rbw/rbro.env`, separate from all other credentials which live under `RBRR_SECRETS_DIR`. Consolidate RBRO into `RBRR_SECRETS_DIR/payor/rbra.env` (or similar) so all role credentials follow the same pattern: one directory per role under the secrets directory.

### Changes required

- **RBS0-SpecTop.adoc**: Update `[[rbro_regime]]` definition and any references to `~/.rbw/rbro.env` location
- **RBSRO-RegimeOauth.adoc**: Update regime file path specification
- **README.md**: Update Credential Safety table (currently shows `~/.rbw/rbro.env`) and RBRO appendix entry
- **Implementation files**: `rbgo_OAuth.sh`, `rbhp_payor.sh`, `rbho_onboarding.sh` — anywhere that reads/writes the RBRO path
- **Regime sourcing**: Update `zrbro_kindle` / any path constants pointing at `~/.rbw/`

### Rationale

All other credentials (Governor, Director, Retriever) already live under `RBRR_SECRETS_DIR/{role}/rbra.env`. RBRO being at `~/.rbw/` is a historical outlier. Consolidating simplifies the credential safety story: one directory to protect, one backup target, one permission audit scope.

**[260410-1021] rough**

## Character

Mechanical refactoring with spec-level implications. Move RBRO storage from user home directory to RBRR_SECRETS_DIR alongside RBRA credentials.

## Docket

RBRO (Payor OAuth credentials — client secret and refresh token) currently lives at `~/.rbw/rbro.env`, separate from all other credentials which live under `RBRR_SECRETS_DIR`. Consolidate RBRO into `RBRR_SECRETS_DIR/payor/rbra.env` (or similar) so all role credentials follow the same pattern: one directory per role under the secrets directory.

### Changes required

- **RBS0-SpecTop.adoc**: Update `[[rbro_regime]]` definition and any references to `~/.rbw/rbro.env` location
- **RBSRO-RegimeOauth.adoc**: Update regime file path specification
- **README.md**: Update Credential Safety table (currently shows `~/.rbw/rbro.env`) and RBRO appendix entry
- **Implementation files**: `rbgo_OAuth.sh`, `rbhp_payor.sh`, `rbho_onboarding.sh` — anywhere that reads/writes the RBRO path
- **Regime sourcing**: Update `zrbro_kindle` / any path constants pointing at `~/.rbw/`

### Rationale

All other credentials (Governor, Director, Retriever) already live under `RBRR_SECRETS_DIR/{role}/rbra.env`. RBRO being at `~/.rbw/` is a historical outlier. Consolidating simplifies the credential safety story: one directory to protect, one backup target, one permission audit scope.

### run-all-onboarding-tracks (₢AUAA7) [abandoned]

**[260601-0538] abandoned**

Drafted from ₢A_AAG in ₣A_.

## Character
End-to-end validation — user perspective. Every handbook track must complete cleanly.

## Work
Walk every onboarding track in sequence: Crash Course, Credential Retriever, Credential Director, First Crucible, Director First Cloud Build, Payor, Governor. Verify all handbook windows render correctly with prefixed resource names. Confirm probe outputs show the correct prefixed container/image identifiers. Fix any display or functional issues.

**[260514-1128] rough**

Drafted from ₢A_AAG in ₣A_.

## Character
End-to-end validation — user perspective. Every handbook track must complete cleanly.

## Work
Walk every onboarding track in sequence: Crash Course, Credential Retriever, Credential Director, First Crucible, Director First Cloud Build, Payor, Governor. Verify all handbook windows render correctly with prefixed resource names. Confirm probe outputs show the correct prefixed container/image identifiers. Fix any display or functional issues.

**[260415-1505] rough**

## Character
End-to-end validation — user perspective. Every handbook track must complete cleanly.

## Work
Walk every onboarding track in sequence: Crash Course, Credential Retriever, Credential Director, First Crucible, Director First Cloud Build, Payor, Governor. Verify all handbook windows render correctly with prefixed resource names. Confirm probe outputs show the correct prefixed container/image identifiers. Fix any display or functional issues.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 x ifrit-sortie-env-and-cap-net-raw
  2 v nameplate-regime-directory-restructure
  3 w rbra-regime-directory-restructure
  4 t purge-legacy-rubric-and-gitlab
  5 q unify-hashing-on-openssl
  6 r unify-base64-on-openssl
  7 p evict-trivial-posix-utilities
  8 o evict-grep-from-consumer-runtime
  9 s declare-rb-dependency-inventory-in-rbs0
  10 g bcg-command-trust-tiers
  11 l claudemd-include-restructure
  12 u convert-crucible-tabtargets-imprint-to-param
  13 j spook-orient-default-heat-selection
  14 4 bcg-builtin-redirect-vs-subshell
  15 1 relocate-compose-probe-env
  16 h regime-probate-homogeneity
  17 6 factor-openssl-base64-helpers
  18 b bind-mirror-syft-via-container
  19 S enrich-about-with-build-provenance
  20 A derive-build-strategy-from-platforms
  21 J vouch-artifact-multiplatform
  22 K local-vouch-preflight-gate
  23 I release-qualification-gate
  24 E regime-validation-testbench
  25 L consecration-inspect-command
  26 T spec-updates-for-inspect-and-about-enrichment
  27 W bind-vessel-support-and-plantuml-conversion
  28 a mint-rbj-jailer-prefix
  29 c bcg-command-substitution-rule-refinement
  30 Z bcg-compliance-audit
  31 X readme-maturity-warning
  32 O consolidate-lenses-into-veiled
  33 N consumer-claudemd-and-test-discipline
  34 Y readme-architecture-diagram
  35 P freshen-cosmology-and-render
  36 Q getting-started-readme
  37 U clarify-beseech-purpose-and-docs
  38 F simplify-manifest-digest-extraction
  39 V inscribe-dirty-tree-guard
  40 e extract-rbrg-cli-from-rbrr
  41 y enshrine-spec-realignment
  42 z cloud-build-wall-clock-timing

xvwtqrposgluj41h6bSAJKIELTWacZXONYPQUFVeyz
·················xxxxx··x·x··x······x·x··· rbf_Foundry.sh
·xxx····x··········xx····x··········x··xx· RBS0-SpecTop.adoc
·xxx·······x··············x···x·xx·x······ README.consumer.md
·x········x··············x·x···xx···x····· CLAUDE.md
···········x··········x·x·x·········x··x·· rbz_zipper.sh
··xx·x··········x···x········x············ rbgg_Governor.sh
··x··x·x········x············x············ rbgp_Payor.sh
·xx····x··················x··x············ rbgm_ManualProcedures.sh
·········x···x··············xx············ BCG-BashConsoleGuide.md
·······x··················x·xx············ buv_validation.sh
······xx····················xx············ buc_command.sh
·····x·x·····················x·x·········· rbv_PodmanVM.sh
·····xx·········x············x············ rbgo_OAuth.sh
···x···············x·····x··········x····· RBSCB-CloudBuildPosture.adoc
···x·x··········x·······················x· rbfd_FoundryDirectorBuild.sh
·x························x·····x··x······ CLAUDE.consumer.md
·x···················x·····x·x············ rbob_bottle.sh
·····························x·······x·x·· rbrr_cli.sh
····x·x······················x············ bud_dispatch.sh
··xx···········x·························· rbra_regime.sh
·x·····················x·····x············ rbtcrv_RegimeValidation.sh
·xxx······································ rblm_cli.sh
x··················x······x··············· Dockerfile
···························x·x············ rbj_sentry.sh
·························x········x······· RBSCO-CosmologyIntro.adoc
······················x···x··············· rbw_workbench.sh
····················x····x················ RBSAV-ark_vouch.adoc
···················x······x··············· rbrv.env
·················x········x··············· rbrn_pluml.env
··········x·x····························· jjk-claude-context.md
········x·························x······· index.html
·······x·································x rbfc_FoundryCore.sh
······x······················x············ rboo_observe.sh
····x·····x······························· jja_arcanum.sh
···x·························x············ rbrr_regime.sh
··x····x·································· rbdc_DerivedConstants.sh
··xx······································ RBSRR-RegimeRepo.adoc, rbgc_Constants.sh
·x·····························x·········· RBRN-RegimeNameplate.adoc
·x···························x············ rbrn_regime.sh
·x····················x··················· rbtb_testbench.sh
·x·········x······························ rbob_cli.sh
·xx······································· rbcc_Constants.sh
xx········································ rbob_compose.yml, tadmor.compose.yml
········································x· RBSAE-ark_enshrine.adoc
·······································x·· RBSRG-RegimeGcbPins.adoc, RBSRI-rubric_inscribe.adoc, rbrg_cli.sh
····································x····· RBSAB-ark_beseech.adoc, RBSCK-consecration_check.adoc, rbw-DB.DirectorBeseechesArk.sh
·······························x·········· AXMCM-ClaudeMarkConceptMemo.md, CRR-ConfigRegimeRequirements.adoc, RBWMBX-BuildxMultiPlatformAuth.adoc, axl-AXMCM-ClaudeMarkConceptMemo.md, memo-20260111-buildx-multiplatform-auth.adoc, memo-20260111-config-regime-requirements.adoc, rbha_GithubActions.sh, rbhcr_GithubContainerRegistry.sh, rbhh_GithubHost.sh, rbhim_GithubContainerRegistry.sh, rbhr_GithubRemote.sh, rbhr_cli.sh, rbrm.env, rbrm_regime.sh, rbv_cli.sh, trbim_dockerfile.recipe
·····························x············ bug_guide.sh, buto_operations.sh, rbgu_Utility.sh
····························x············· bupr_PresentationRegime.sh, rbro_cli.sh
···························x·············· rbss.sentry.sh
··························x··············· rbtcal_ArkLifecycle.sh, rbw-DC.DirectorConjuresArk.sh, rbw-DC.DirectorCreatesArk.sh, rbw-DM.DirectorMirrorsBind.sh
·························x················ RBSAC-ark_conjure.adoc, RBSAI-ark_inspect.adoc, RBSAS-ark_summon.adoc, RBSGS-GettingStarted.adoc
························x················· rbw-RiF.RetrieverInspectsFull.sh, rbw-Ric.RetrieverInspectsCompact.sh
·······················x·················· rbw-tf.TestFixture.regime-validation.sh
······················x··················· rbq_Qualify.sh, rbtcqa_QualifyAll.sh, rbw-QR.QualifyRelease.sh, rbw-Qf.QualifyFast.sh, rbw-qa.QualifyAll.sh
·····················x···················· RBSBS-bottle_start.adoc
····················x····················· RBSDI-director_create.adoc, rbgjv03-assemble-push-vouch.sh
···················x······················ RBSRV-RegimeVessel.adoc, proof.txt, rbgjb01-derive-tag-base.sh, rbrv_regime.sh
··················x······················· rbgjb03-buildx-push-multi.sh, rbgjb05-push-per-platform.sh, rbgjb08-buildx-push-about.sh
···············x·························· burc_regime.sh, burn_regime.sh, burp_regime.sh, burs_regime.sh, rbro_regime.sh, rbrp_regime.sh, rbrs_regime.sh, rbtdrf_fast.rs, rbtdrm_manifest.rs
············x····························· jjrm_mcp.rs, jjrq_query.rs, jjrsd_saddle.rs
···········x······························ rbw-Ic.IfritClient.tadmor.sh, rbw-ch.Hail.pluml.sh, rbw-ch.Hail.sh, rbw-ch.Hail.srjcl.sh, rbw-ch.Hail.tadmor.sh, rbw-cr.Rack.sh, rbw-cr.Rack.srjcl.sh, rbw-cr.Rack.tadmor.sh, rbw-cs.Scry.pluml.sh, rbw-cs.Scry.sh, rbw-cs.Scry.srjcl.sh, rbw-cs.Scry.tadmor.sh
··········x······························· buk-claude-context.md, cmk-claude-context.md, cmw_workbench.sh, vvk-claude-context.md
·········x································ bul_launcher.sh, rbw-cC.Charge.pluml.sh, rbw-cC.Charge.srjcl.sh, rbw-cC.Charge.tadmor.sh, rbw-cQ.Quench.pluml.sh, rbw-cQ.Quench.srjcl.sh, rbw-cQ.Quench.tadmor.sh, rbw-s.Start.pluml.sh, rbw-s.Start.srjcl.sh, rbw-s.Start.tadmor.sh, rbw-z.Stop.pluml.sh, rbw-z.Stop.srjcl.sh, rbw-z.Stop.tadmor.sh
·······x·································· rbga_ArtifactRegistry.sh, rbgb_Buckets.sh, rbtcns_TadmorSecurity.sh, rbtcpl_PlumlDiagram.sh, rbtcsj_SrjclJupyter.sh
······x··································· buq_qualify.sh, butd_dispatch.sh, buut_tabtarget.sh, rbi_Image.sh
·····x···································· rgbs_ServiceAccounts.sh
····x····································· rbgje01-enshrine-copy.sh, vob_build.sh
···x······································ .gitignore, RBSCTD-CloudBuildTriggerDispatch.adoc, RBSDE-depot_levy.adoc, RBSDK-director_knight.adoc, RBSDN-depot_initialize.adoc, rbk-prep-release.md
··x······································· rbra_cli.sh
·x········································ compose.yml, pluml.rbrn.env, rbrn.env, rbrn_cli.sh, rbtcfm_FourMode.sh, rbtctm_ThreeMode.sh, rbw-tf.TestFixture.four-mode.sh, rbw-tf.TestFixture.three-mode.sh, srjcl.rbrn.env, tadmor.rbrn.env

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 291 commits)

  1 y enshrine-spec-realignment
  2 z cloud-build-wall-clock-timing
  3 1 relocate-compose-probe-env
  4 4 bcg-builtin-redirect-vs-subshell
  5 6 factor-openssl-base64-helpers
  6 h regime-probate-homogeneity

123456789abcdefghijklmnopqrstuvwxyz
·x··x······························  y  2c
··xx·······························  z  2c
························x··········  1  1c
·························xx········  4  2c
···························xx······  6  2c
·······························xx··  h  2c
```

## Steeplechase

### 2026-06-01 06:08 - Heat - f

silks=rbk-mvp-release-finalize

### 2026-06-01 05:38 - Heat - T

run-all-onboarding-tracks

### 2026-05-30 10:06 - ₢AUAAh - W

Gave the 8 remaining file-based regimes (rbrp/rbrs/rbro/rbra + burc/burn/burp/burs) a probate test seam mirroring rbrr_probate (source handed file → kindle → enforce), each with a baseline-valid + seeded-violation theurge fast-fixture case pair. Violations aim at custom enforce checks where present (rbrp payor regex via RBGC, rbra PEM-BEGIN, burn platform enum, burs tincture regex) else an unset required field; credential regimes use synthetic non-secret baselines. Generalized rbtdrf_run_probate into rbtdrf_run_probate_in (tools_subdir for Tools/rbk vs Tools/buk + raw-bash prelude for cross-module prereqs), keeping a thin wrapper so the 4 reference rv_ call sites stay untouched. Two per-regime prereqs handled via prelude rather than contaminating the bash probates: rbrp stages rbgc kindled (docket-flagged; RBGC kindle proved self-contained), and burs stages bubc — an unanticipated wrinkle where BURS_USER's enrollment help-string expands BUBC_* under set -u, caught by manual smoke test before commit. Verified: rbw-tt 100 passed; rbw-ts fast 129 passed 0 failed (regime-validation fixture 32→48 cases). Code committed 57a633a.

### 2026-05-30 10:05 - ₢AUAAh - n

Probate seam for the 8 remaining file-based regimes (rbrp/rbrs/rbro/rbra + burc/burn/burp/burs), mirroring rbrr_probate: source handed file, kindle, enforce. Each gains a baseline-valid + seeded-violation theurge fast-fixture case pair, with violations aimed at custom enforce checks where present (rbrp payor regex via RBGC, rbra PEM-BEGIN, burn platform enum, burs tincture) else an unset required field. Generalized rbtdrf_run_probate into rbtdrf_run_probate_in (tools_subdir for Tools/rbk vs Tools/buk + a raw-bash prelude for cross-module prereqs: rbrp stages rbgc kindled, burs stages bubc for its help-string expansion); thin wrapper preserves the 4 reference rv_ call sites. 16 new manifest module/probate consts. Credential regimes use synthetic non-secret baselines.

### 2026-05-29 08:57 - Heat - T

regime-probate-homogeneity

### 2026-05-29 09:06 - Heat - f

silks=rbk-18-mvp-release-finalize

### 2026-05-27 18:10 - ₢AUAA6 - W

Validation complete — all six openssl base64 sites exercised clean. The factoring itself landed earlier (139838ac5: four stateless primitives in rbgo_OAuth.sh, six call sites rewired). Four sites were already validated live via the canonical-invest skirmish (two string→file decodes + both JWT base64url encodes). The two remaining cloud-build decode sites in rbfd_FoundryDirectorBuild.sh were gated behind the rbw fast-qualify shellcheck findings owned by ₣BT; once that gate went green (181 files clean), they were exercised here: the conjure decode (rbfd_build:1315) via a single dogfight conjure ordain, and the enshrine decode (zrbfd_enshrine_extract_anchors:1082) via a standalone rbw-dE enshrine of the forge vessel. Both Cloud Builds passed; no code changes, tree clean.

### 2026-05-27 11:18 - ₢AUAA6 - n

Factor openssl base64 into four stateless rbgo primitives, retiring six duplicated invocations.

### 2026-05-27 10:53 - ₢AUAA4 - W

BCG: explicitly distinguish $(<file) builtin redirect from $(command) substitution. Added a prose paragraph in the Temp Files / Command Substitution section carving out $(<file) as a builtin file redirect (no subshell, no command, nothing to fail) exempt from the || buc_die guard — it takes content validation (test -n) instead. Tightened the Command Substitution Rules checklist bullet from a bare 'followed by validation' to state $(<file) needs content validation, not a guard. Closes the gap left by the prior substitution-rule refinement, which had the distinction scattered in code comments but no explicit home in the rules prose.

### 2026-05-27 10:53 - ₢AUAA4 - n

Folded `$(<file)` builtin-redirect distinction into BCG command-substitution rules

### 2026-05-27 17:53 - ₢AUAA1 - W

Verified compose-probe relocation already complete: rbje_compose_probe.env lives at Tools/rbk/ (out of .rbk/ runtime config), the single reader rbob_bottle.sh:131 resolves it via RBCC_KIT_DIR (Tools/rbk/), and no stale .rbk/ path references remain. Done-by-verification — no edits needed.

### 2026-05-27 17:41 - Heat - d

paddock curried: rewrite around reduced scope after restringing prep-release ceremony and foundry audit to BB

### 2026-05-27 17:31 - Heat - r

moved AUAA5 after AUAA6

### 2026-05-27 17:31 - Heat - r

moved AUAA6 after AUAAh

### 2026-05-27 17:31 - Heat - r

moved AUAA1 after AUAA4

### 2026-05-27 17:30 - Heat - r

moved AUAA4 to first

### 2026-05-14 11:28 - Heat - D

restring 1 paces from ₣A_

### 2026-05-11 15:40 - Heat - f

silks=rbk-14-mvp-release-finalize

### 2026-04-27 13:34 - Heat - n

Drop the multi-deliverable anti-pattern (legitimate when deliverables co-discover); add plan-step-leakage rule against docket phase/step labels appearing in code comments.

### 2026-04-27 13:26 - Heat - n

Add slate-time vs mount-time framing, stale-by-mount filter, and docket anti-patterns to JJK context. Goal: prevent overprescription that drowns out project patterns and ages badly between slate and mount.

### 2026-04-27 13:19 - Heat - S

factor-openssl-base64-helpers

### 2026-04-23 11:12 - Heat - S

foundry-and-cloud-step-trust-audit

### 2026-04-23 10:15 - Heat - n

Sketch cross-cloud delivery architecture in RBSHR: embassy, envoy, lode, accredit/recall, entrust, chantry. Names tentative; asymmetric sovereignty stance noted (Recipe Bottle governs identities only in GCP; AWS-side via federated envoy and policy shapes).

### 2026-04-17 06:59 - Heat - S

bcg-builtin-redirect-vs-subshell

### 2026-04-15 15:25 - Heat - T

relocate-rbro-to-secrets-dir

### 2026-04-15 15:25 - Heat - T

emplace-depot-prefix-on-gar-images

### 2026-04-15 06:30 - Heat - S

emplace-depot-prefix-on-gar-images

### 2026-04-14 07:11 - Heat - S

research-config-directory-naming

### 2026-04-13 09:12 - Heat - S

relocate-compose-probe-env

### 2026-04-10 10:21 - Heat - S

relocate-rbro-to-secrets-dir

### 2026-04-07 22:36 - ₢AUAAy - W

Realigned enshrine spec (RBS0 + RBSAE) with implementation: 5 text corrections (GCB submission not local pull, digest from raw manifest sha256sum, full origin sanitization, conjure mode guard, reliquary preflight guard). Extracted zrbfd_preflight_reliquary() from zrbfd_registry_preflight and added call in rbfd_enshrine before GCB submit.

### 2026-04-07 22:36 - ₢AUAAz - W

Surface Cloud Build wall-clock timing in terminal output: added ZRBFC_BUILD_START_FILE/ZRBFC_BUILD_FINISH_FILE kindle constants, extract startTime/finishTime from status JSON after build completion, compute and display duration with portable GNU/BSD date fallback. All cloud build paths (conjure, enshrine, mirror, graft about+vouch) get timing automatically. Reslated docket to match simplified scope — terminal display only, no about artifact threading.

### 2026-04-07 22:36 - ₢AUAAz - n

Surface Cloud Build wall-clock timing after every build completion: extract startTime/finishTime from status JSON, compute duration with portable date conversion, display via buc_info

### 2026-04-07 22:31 - ₢AUAAy - n

Realign enshrine spec with implementation: fix GCB submission wording, digest computation method, origin sanitization scope, add conjure mode and reliquary preflight require guards; extract zrbfd_preflight_reliquary from registry_preflight and call from rbfd_enshrine

### 2026-04-02 17:05 - Heat - n

Rename tabtargets: rbw-ak→rbw-LK (LocalKludge), rbw-cK→rbw-Tk (KludgeCycle), rbw-cO→rbw-To (OrdainCycle). Separates privilege tiers: L for unprivileged local builds, T for compound theurge iteration workflows. Adds git describe to kludge consecration format.

### 2026-04-02 15:42 - Heat - n

Fix all 9 warnings: remove unused vvco_out imports (4), delete unreachable catch-all arm, delete dead Retire structs and cascade Serialize import, delete dead jjrm_dispatch_heat

### 2026-04-02 15:41 - Heat - n

Add deny(warnings) to all three kit crates — build now fails on the 9 existing warnings as expected

### 2026-04-02 15:34 - Heat - n

Extend exsanguination threshold from 4h to 7 days to prevent officium ID collisions; move disk space guard from dispatch layer into handlers (open/orient/show) using vvco_Output

### 2026-04-02 10:13 - Heat - S

cloud-build-wall-clock-timing

### 2026-04-01 13:46 - Heat - n

Realign tally spec with implementation: remove phantom Mode column, fix incomplete recommendation to abjure-only, drop overstated platform coverage claim, note diags parsed but unused for health

### 2026-04-01 13:42 - Heat - n

Realign vouch spec with implementation: remove phantom _RBGV_VERIFY_METHOD substitution, specify airgap pool, clarify local polling vs GCB steps, fix stale jq comment

### 2026-04-01 13:37 - Heat - S

enshrine-spec-realignment

### 2026-04-01 09:46 - ₢AUAAj - W

Made jjx_orient firemark parameter required — parameterless orient now returns a deserialization error instead of silently defaulting to first racing heat. Removed jjrq_resolve_default_heat dead code. Updated Mount Protocol in jjk-claude-context.md to instruct agent to always provide firemark or ask user.

### 2026-04-01 09:45 - ₢AUAAj - n

Make jjx_orient firemark parameter required — parameterless orient is now a deserialization error instead of silently defaulting to first racing heat

### 2026-04-01 09:38 - ₢AUAAu - W

Converted crucible tabtargets (Hail, Rack, Scry) from imprint-style to param-style with nameplate as $1. Added BURD_INTERACTIVE=1 for TTY-preserving dispatch on all interactive commands (Hail, Rack, Scry, IfritClient). Added nameplate listing on missing argument in rbob_cli furnish path.

### 2026-04-01 09:35 - ₢AUAAu - n

Convert crucible tabtargets (Hail, Rack, Scry) from imprint-style to param-style, add BURD_INTERACTIVE for TTY-preserving dispatch, list available nameplates on missing argument, fix same TTY bug on IfritClient

### 2026-04-01 09:19 - Heat - r

moved AUAAh after AUAAj

### 2026-04-01 09:07 - Heat - r

moved AUAAh to first

### 2026-04-01 09:07 - Heat - r

moved AUAAu to first

### 2026-04-01 09:07 - Heat - r

moved AUAAj to first

### 2026-04-01 09:02 - ₢AUAAg - W

Added Command Dependency Discipline section to BCG with POSIX utility allowlist, evicted utilities table, declared dependency principle, platform-variant command guidance, and LLM contributor checkpoint

### 2026-04-01 09:02 - ₢AUAAg - n

Add Command Dependency Discipline section to BCG: POSIX allowlist, eviction table, declared-dependency principle, and platform-variant guidance

### 2026-04-01 08:59 - ₢AUAAs - W

Added dependency inventory section to RBS0 with consumer/developer/specialized categories and updated index.html to declare openssl

### 2026-04-01 08:59 - ₢AUAAs - n

Add dependency inventory to RBS0 and include openssl in base utilities

### 2026-04-01 08:55 - ₢AUAAo - W

Evicted grep from all consumer runtime shell scripts (~43 sites across 12 files): buv_validation.sh 12 regex checks to [[ =~ ]], buc_command.sh 2 function filtering sites, rbgp_Payor.sh 5 regex checks, rbgm_ManualProcedures.sh 3 file searches to while-read+case, rbga/rbgb/rbfc/rbdc 4 sites to capture+test or while-read, rbv_PodmanVM.sh 3 mixed sites, test fixtures rbtcpl/rbtcns/rbtcsj 14 sites to case substring and [[ =~ ]]. Zero grep in Tools/buk/*.sh and Tools/rbk/*.sh consumer runtime. GCB build scripts (rbgje/rbgjm) intentionally excluded.

### 2026-04-01 08:55 - ₢AUAAo - n

Replaced grep pipelines with bash builtins ([[ =~ ]], case/esac, while-read loops) across 12 files: eliminated echo-pipe-grep anti-pattern in BUC command/help dispatch, BUV validation checks (14 sites), RBDC migration probe, RBFC recipe parsing, RBGA/RBGB HTTP status checks, RBGM onboarding probes, RBGP depot validation (6 sites), RBV podman init checks, and test fixtures (TadmorSecurity traceroute, PlumlDiagram curl assertions, SrjclJupyter process check). Remaining grep usage is in GCB cloud scripts and foundry JSON parsing beyond docket scope.

### 2026-04-01 08:47 - ₢AUAAp - W

Evicted six POSIX utilities from listed consumer runtime sites: head→while-read loops (rbi_Image 2 sites), tr→parameter expansion (rbgo_OAuth 2 sites, JWT base64url verified via 15 GCP token exchanges), cut→read with field splitting (buc_command), wc→while-read counters (buq_qualify, butd_dispatch 2 sites), tput→ANSI literals matching BUC constants (rboo_observe) and simplified terminal detection (bud_dispatch), ls→glob loop (buut_tabtarget). Fixed initial mapfile usage that violated BCG bash 3.2 compatibility requirement. Remaining tr/cut/wc hits in GCB scripts and foundry modules are beyond docket scope.

### 2026-04-01 08:47 - ₢AUAAp - n

Replaced external commands (tput, wc, tr, head, ls, mktemp) with bash builtins and ANSI literals across 8 files: while-read loops replace for-in-subshell and wc -l counting, parameter substitution replaces tr pipelines, glob iteration replaces ls, and hardcoded ANSI escapes replace tput queries

### 2026-04-01 08:38 - ₢AUAAr - W

Unified all base64 encoding/decoding on openssl enc -base64 across 6 files (8 call sites): replaced platform-divergent base64 command with portable openssl, eliminated macOS -d/-D fallback hacks in rbgp_Payor and rbgg_Governor, removed command -v base64 check from rbgo_OAuth. JWT encoding verified end-to-end via 15 successful GCP token exchanges across all service account roles.

### 2026-04-01 08:38 - ₢AUAAr - n

Standardize base64 encoding/decoding to openssl enc -base64 across 6 files: replaced base64/base64 -D fallback cascades in rbgg_Governor, rbgp_Payor, removed standalone base64 dependency check in rbgo_OAuth, unified all encode/decode sites in rbfd_FoundryDirectorBuild, rbv_PodmanVM, and rgbs_ServiceAccounts

### 2026-04-01 08:34 - ₢AUAAq - W

Unified all hashing on openssl dgst -sha256 -r across 4 files: removed sha256sum/shasum fallback cascades in bud_dispatch and vob_build, fixed macOS-only portability bug in jja_arcanum, standardized rbgje01-enshrine-copy. All sites now use read -r for hash extraction instead of awk/cut.

### 2026-04-01 08:34 - ₢AUAAq - n

Standardize SHA-256 hashing to openssl dgst -sha256 -r across all shell scripts, replacing platform-dependent fallback chains (sha256sum, shasum) with a single portable command

### 2026-04-01 08:30 - ₢AUAAt - W

Purged legacy rubric and gitlab dead code across 17 files: removed zrbgg_rubric_preflight function and 2 call sites, RBGC_RUBRIC constants, RBRA/RBRR rubric enrollment lines, rblm_cli CBv2 stripping cases, elimination comments from 8 .sh files; cleaned rubric/gitlab references from 7 .adoc specs, README.consumer.md, CLAUDE.md mapping, .gitignore, and rbk-prep-release.md. Zero rubric/gitlab hits in .sh and .adoc. inscribe-mirror untouched.

### 2026-04-01 08:29 - ₢AUAAt - n

Purge legacy rubric and gitlab dead code: remove zrbgg_rubric_preflight function and call sites, RBGC_RUBRIC constants, RBRA/RBRR rubric enrollment lines, rblm_cli stripping case, elimination comments across 8 .sh files; clean rubric/gitlab references from 7 .adoc specs and README; remove RBSRI mapping from CLAUDE.md and rubric gitignore entry

### 2026-04-01 08:14 - ₢AUAAw - W

Restructured RBRA credential files from flat rbra-{role}.env to directory-based {role}/rbra.env layout with RBRA_ROLE swizzle protection, one-shot migration, directory provisioning, and role constants migrated from RBGC to RBCC

### 2026-04-01 07:33 - ₢AUAAv - n

Update tadmor bottle consecration to c260401071151-r260401141508 (has ifrit user)

### 2026-04-01 07:32 - ₢AUAAv - n

Fix compose fragment paths: env_file and volumes resolve relative to first -f file directory (.rbk/), not the fragment file directory

### 2026-04-01 07:10 - ₢AUAAw - n

Restructure RBRA credential files to directory-based {role}/rbra.env layout with RBRA_ROLE swizzle protection, migrate role name constants from RBGC kindle to RBCC tinder, add one-shot migration in RBDC kindle

### 2026-04-01 06:58 - ₢AUAAv - W

Restructured nameplate regime files from flat {moniker}.rbrn.env to directory-based {moniker}/rbrn.env. Moved compose fragment into moniker directory. Renamed RBCC_rbrn_suffix to RBCC_rbrn_file, updated all 10 code call sites, 7 doc files, and fixed pre-existing rblm_cli.sh glob bug that used wrong pattern.

### 2026-04-01 06:56 - ₢AUAAv - n

Remove old flat nameplate and compose files superseded by directory-based layout

### 2026-04-01 06:56 - ₢AUAAv - n

Restructure nameplate regime files from flat {moniker}.rbrn.env to directory-based {moniker}/rbrn.env, move compose fragment into moniker directory, rename RBCC_rbrn_suffix to RBCC_rbrn_file, update all call sites and docs, fix rblm_cli.sh pre-existing glob bug

### 2026-04-01 06:51 - ₢AUAAx - W

Added cap_drop NET_RAW to base bottle compose, added cap_add NET_RAW + env_file for RBRN vars + named ifrit user to tadmor fragment. Dockerfile updated by parallel officium to create unprivileged ifrit user with home directory.

### 2026-04-01 13:49 - ₢AUAAx - n

Fix ifrit bottle user setup: create named ifrit user in Dockerfile, reference by name in compose, gitignore root-owned .claude/ dir, remove HOME hack

### 2026-04-01 06:36 - ₢AUAAx - n

Add cap_drop NET_RAW to base bottle, add cap_add NET_RAW + env_file + user 1000:1000 to tadmor ifrit fragment

### 2026-04-01 06:19 - Heat - S

ifrit-sortie-env-and-cap-net-raw

### 2026-04-01 06:01 - Heat - S

rbra-regime-directory-restructure

### 2026-04-01 06:00 - Heat - S

nameplate-regime-directory-restructure

### 2026-04-01 04:54 - Heat - S

convert-crucible-tabtargets-imprint-to-param

### 2026-03-31 14:21 - Heat - T

remove-director-gate-from-foundry-kindle

### 2026-03-31 11:45 - Heat - S

purge-legacy-rubric-and-gitlab

### 2026-03-31 11:29 - Heat - r

moved AUAAg after AUAAs

### 2026-03-31 11:15 - Heat - S

declare-rb-dependency-inventory-in-rbs0

### 2026-03-31 11:04 - Heat - S

unify-base64-on-openssl

### 2026-03-31 10:46 - Heat - S

unify-hashing-on-openssl

### 2026-03-31 10:41 - Heat - S

evict-trivial-posix-utilities

### 2026-03-31 10:38 - ₢AUAAg - n

Evict awk from consumer runtime: replace stty size | awk field extraction with read builtin in terminal width detection

### 2026-03-31 10:20 - Heat - S

evict-grep-from-consumer-runtime

### 2026-03-31 07:54 - Heat - S

remove-director-gate-from-foundry-kindle

### 2026-03-28 19:18 - Heat - r

moved AUAAm to first

### 2026-03-28 10:17 - ₢AUAAm - n

Always display regime variable names with their prefix (RBRR_, BURC_) for user contextualization

### 2026-03-28 10:16 - ₢AUAAm - n

Trim Getting Started section: remove review_defaults dump, show only next-step guidance

### 2026-03-28 10:14 - ₢AUAAm - n

Add buv_export_and_lock to export all enrolled BURC variables across exec boundary, replacing selective export list

### 2026-03-28 10:13 - Heat - n

Veil forge-only context files (JJK, VVK, CMK), consolidate shared discipline sections (Forbidden Shell Ops, Test Execution, TabTarget System) into buk-claude-context.md as single source of truth, update consumer CLAUDE.md to use @-include for BUK context

### 2026-03-28 10:12 - ₢AUAAm - n

Export BURC_STATION_FILE so it survives the exec boundary into coordinator child processes

### 2026-03-28 10:06 - ₢AUAAm - n

Fix payor role detection: credential file (rbro-payor.env) presence is the role declaration, not committed rbrp.env config

### 2026-03-28 09:59 - ₢AUAAm - n

Replace linear 14-level onboarding staircase with role-aware dashboard: independent credential probes, role inventory display, credential guidance for absent roles, and per-role status tracks (payor/director/retriever)

### 2026-03-28 09:43 - Heat - S

role-aware-onboarding-dashboard

### 2026-03-28 07:35 - ₢AUAAl - W

Replace MANAGED section patching with @-directive includes: extract kit context to standalone files (buk, cmk, jjk, vvk), migrate BUK vocabulary from main CLAUDE.md, remove Claude-CLI patching machinery from install scripts. Verified all four includes expand correctly in fresh session.

### 2026-03-28 07:30 - ₢AUAAl - n

Replace MANAGED section patching with @-directive includes: extract kit context to standalone files, remove Claude-CLI patching machinery from install scripts

### 2026-03-28 07:16 - Heat - T

slim-sentry-vessel

### 2026-03-28 07:04 - Heat - S

claudemd-include-restructure

### 2026-03-28 06:44 - Heat - n

Remove old RBSIP-iptables_init.adoc (renamed to RBSII in prior commit)

### 2026-03-28 06:44 - Heat - n

Introduce Ifrit Pentester (RBSIP) — adversarial AI escape testing framework. New system concept document with base image trade study (Debian bookworm-slim), slim vessel strategy, volume mount design, escape test infrastructure (rbtid/), and consolidated research sources. Rename RBSIP-iptables_init to RBSII to free the acronym.

### 2026-03-27 21:48 - Heat - S

slim-sentry-vessel

### 2026-03-27 17:09 - Heat - S

spook-orient-default-heat-selection

### 2026-03-17 18:49 - Heat - r

moved AUAAi before AUAAg

### 2026-03-17 18:49 - Heat - D

restring 1 paces from ₣Ar

### 2026-03-15 11:46 - Heat - T

prerelease-regime-render-sweep

### 2026-03-15 09:44 - Heat - S

prerelease-regime-render-sweep

### 2026-03-14 21:11 - Heat - S

bcg-command-trust-tiers

### 2026-03-14 20:40 - ₢AUAAR - n

Fixed branch model in release ceremony: replaced develop references with main, candidate now branches from OPEN_SOURCE_UPSTREAM/main with squash-merge of main

### 2026-03-14 20:19 - Heat - d

paddock curried

### 2026-03-14 20:15 - Heat - f

racing

### 2026-03-14 18:11 - Heat - n

Added hairpin peer access item to Horizon Roadmap under Container Runtime, cross-referencing heat AU paddock for design context

### 2026-03-14 18:09 - Heat - d

paddock curried

### 2026-03-14 18:09 - Heat - f

stabled

### 2026-03-14 17:56 - Heat - S

queued-build-advisory-and-quota-guide-repair

### 2026-03-14 15:19 - ₢AUAAe - W

Extracted RBRG pin management (5 functions, 4 state vars) from rbrr_cli.sh into new rbrg_cli.sh module, rerouted zipper enrollments, updated spec references in RBS0, RBSRG, RBSRI

### 2026-03-14 14:30 - ₢AUAAe - n

Extracted RBRG pin management (5 functions, 4 state vars) from rbrr_cli.sh into new rbrg_cli.sh module, rerouted zipper enrollments, updated spec references

### 2026-03-14 13:42 - Heat - S

extract-rbrg-cli-from-rbrr

### 2026-03-14 13:30 - ₢AUAAR - n

Restructured release ceremony ordering: LLM checks (command audit, regime variable completeness) before expensive test suite, marshal reset moved before strip so tabtarget still exists, post-strip changed from full rbw-QR to fast-only rbw-Qf, absorbed AUAAd refinements, dropped redundant final cleanup step

### 2026-03-14 13:17 - Heat - T

prep-release-ceremony-refinements

### 2026-03-14 13:17 - Heat - T

prep-release-slash-command

### 2026-03-14 13:04 - ₢AUAAR - n

Added RBLM acronym mapping to CLAUDE.md file mappings section

### 2026-03-14 13:02 - Heat - S

prep-release-ceremony-refinements

### 2026-03-14 12:54 - ₢AUAAR - n

Added rbw-MR and rbw-MD marshal tabtargets to strip list in prep-release ceremony — internal developer tools, not consumer-facing

### 2026-03-14 12:53 - ₢AUAAR - n

Removed non-load-bearing buc_require confirmation from rblm_duplicate — command is purely additive with buc_die existence guard, no destructive action to confirm

### 2026-03-14 12:48 - Heat - n

Normalize rbj_sentry.sh indentation from 4-space to 2-space to match project bash conventions

### 2026-03-14 12:46 - ₢AUAAR - n

Minted rblm (Lifecycle Marshal) prefix: extracted rbrr_reset to rblm_cli.sh as rblm_reset, added rblm_duplicate for isolated repo cloning with station-files mirroring, eliminated differential furnish from rbrr_cli.sh, enrolled rbw-MD colophon in zipper

### 2026-03-14 12:22 - ₢AUAAR - n

Created /rbk-prep-release slash command — interactive release ceremony with pre/post-strip qualification, LLM regime variable completeness check, comprehensive strip list, marshal reset, and candidate branch mechanics modeled on cma-prep-pr

### 2026-03-14 11:40 - ₢AUAAP - W

Freshened RBSCO-CosmologyIntro.adoc: rewrote Current Challenges as Accomplished Infrastructure (present tense), fixed podman to docker in Vision, added 9 timeline entries (Aug 2025 through Mar 2026 including docker switch and SLSA L3), added Project Direction section from RBSHR roadmap themes, added Links section. Faux-rendered to index.html with all 32 MCM attributes resolved to plain text, stale GHCR/podman/eBPF content removed, dark-mode styling preserved.

### 2026-03-14 11:38 - ₢AUAAP - n

Faux-rendered RBSCO to index.html with all MCM attributes resolved to plain text. Removed stale Current Problem/Definitions/GHCR content, replaced with current Accomplished Infrastructure, Project Direction, and Links sections. Timeline now covers Dec 2023 through Mar 2026. Cross-links between project page and repo established.

### 2026-03-14 11:34 - ₢AUAAP - n

Updated RBSCO: rewrote Current Challenges to present tense (Accomplished Infrastructure), fixed podman to docker in Vision section, added 9 timeline entries (Aug 2025 through Mar 2026 including docker switch), added Project Direction section with roadmap themes, added Links section with repo and project page URLs

### 2026-03-14 11:14 - ₢AUAAY - W

Added architecture diagram to README.consumer.md, rewrote opening to lead with build provenance then runtime isolation, updated Vessel (conjure/bind/graft) and Ark (user-controlled GAR) Key Concepts, deleted legacy .github/workflows and RBM* directories

### 2026-03-14 11:14 - ₢AUAAY - n

Update Vessel and Ark Key Concepts: Vessel now covers all three modes (conjure/bind/graft), Ark emphasizes user-controlled Artifact Registry

### 2026-03-14 10:54 - ₢AUAAY - n

Rewrite README.consumer.md opening to lead with build provenance (GCB orchestration, SLSA, SBOM) then runtime isolation (sentry/bottle), replacing single untrusted-containers paragraph

### 2026-03-14 10:32 - Heat - n

Delete four legacy RBM* root directories: old Jupyter environments and Makefile-based test stubs, no longer referenced by rbk

### 2026-03-14 10:32 - Heat - n

Delete abandoned GitHub Actions workflows (rbgr-build.yml, rbgr-delete.yml) and add architecture diagram to README.consumer.md

### 2026-03-14 10:24 - ₢AUAAc - W

Refined BCG command substitution rule: replaced blanket prohibition with three specific prohibited patterns (local+capture, pipelines inside $(), unguarded $()) and documented legitimate forms (_capture, _recite, bash introspection). Renamed 'literal constant' to 'tinder constant' across all 7 BCG references with kindle metaphor explanation. Converted 10 ZBUC_* color constants from computed source-time $() to BUC_* tinder constants using ANSI-C quoted literals, removed zbuc_color() function, renamed across 4 consumer files.

### 2026-03-14 10:24 - ₢AUAAX - W

Added prominent maturity/security warning to top of README.consumer.md. Warning leads with the three-container runtime containment architecture (censer/sentry/bottle) as the area most needing independent review, follows with supply chain hardening (SLSA, least-privilege, no secrets in VCS). Tone: confident but honest, invites security contributors.

### 2026-03-14 10:24 - ₢AUAAX - n

Add security-review-welcome banner to consumer README with architecture summary and call for independent review

### 2026-03-14 10:08 - ₢AUAAc - n

Convert color constants from computed ZBUC_* (source-time $() command substitution) to BUC_* tinder constants (ANSI-C quoted literals). Remove zbuc_color() function. Rename across 4 consumer files. ABANDONED files left untouched.

### 2026-03-14 10:02 - ₢AUAAc - n

Refine BCG command substitution rule and rename literal constant to tinder constant. Command substitution: replace blanket prohibition with three specific prohibited patterns (local+capture, pipelines inside $(), unguarded $()) and document legitimate forms (_capture, _recite, bash introspection). Tinder constant: rename across all 6 BCG references, add kindle metaphor explanation, add $() violation example, update checklist and shellcheck table.

### 2026-03-14 09:24 - ₢AUAAZ - W

BCG compliance audit across 19 files in BUK/RBK: waves 1-2 (correctness/portability — pseudo-ternaries, [[ ]]→test, eval→indirect, unguarded commands, echo -e→printf), waves 3-4 (command substitution elimination — temp files, builtins, _capture functions). Post-audit cleanup: reverted RBGP function extraction that changed error contract, reverted 3 secret-to-disk conversions (private key, GitLab token, webhook secret), fixed digest whitespace stripping, fixed 5 test&&assignment patterns, replaced process substitution with temp file. 19/102 files covered — focused on highest-risk modules. Rule refinement deferred to ₢AUAAc.

### 2026-03-14 09:03 - ₢AUAAZ - n

Revert secret-to-scratch-file conversions in RBGP (base64-encoded GitLab token, webhook secret, and private key were being written to persistent temp file — secrets must never touch disk per BCG). Restore original in-memory command substitution for all three. Fix remaining test&&assignment in RBF inspect (z_qemu check).

### 2026-03-14 08:54 - ₢AUAAZ - n

BCG compliance repairs in rbf_Foundry.sh: fix digest whitespace stripping to loop-peel all trailing chars (not just one), replace 4 test&&assignment patterns with if/then/fi, replace process-substitution while-read-break with temp-file+read for FROM line extraction.

### 2026-03-14 08:30 - ₢AUAAZ - n

Revert zrbgp_authorization_exchange_capture extraction: restore inline OAuth token exchange with original buc_die error messages at each failure site, restore bug_prompt command substitution. Function extraction was scope creep from compliance audit — changed error contract (buc_die→return 1) without design discussion.

### 2026-03-13 22:14 - Heat - S

bcg-command-substitution-rule-refinement

### 2026-03-13 22:09 - ₢AUAAZ - n

Revert BCG command substitution rule change — spec modification requires deliberate review, not mid-audit edit. Findings about over-constrictive rule preserved in conversation for future design discussion.

### 2026-03-13 22:07 - ₢AUAAZ - n

BCG command substitution rule broadening: decompose blanket prohibition into three specific prohibited patterns (local+capture, pipelines inside $(), unguarded $()) and three legitimate forms (single-command with || buc_die, _capture/_recite contracts, source-time constants). Finding from wave 4 compliance audit — original rule was over-constrictive, prohibiting safe patterns like $(date +%s) || buc_die while the real risks are hidden pipeline failures and local swallowing exit status.

### 2026-03-13 21:29 - ₢AUAAZ - n

BCG wave 4: eliminate command substitution in rbf_Foundry.sh and rbgp_Payor.sh. RBF: jq extractions to batch temp-file-read patterns in inspect display (bind 8 fields, conjure 13 fields, vouch 7+2 fields, buildkit 4 fields), jq/docker/git/curl operational code to scratch-file patterns, $(cat||echo) error display to $(<file) builtins, $(printf|base64) secret decode to _capture function zrbf_base64_decode_capture, grep|head to while-read, printf|wc|tr line count to while-read counter. RBGP: OAuth token exchange extracted to zrbgp_authorization_exchange_capture (BCG-correct: return 1 not buc_die), OAuth JSON parsing to batch jq extraction, bug_prompt to temp file, date/base64/jq operational code to scratch-file patterns. Added ZRBF_SCRATCH_FILE and ZRBGP_SCRATCH_FILE kindle constants.

### 2026-03-13 20:52 - ₢AUAAZ - n

BCG wave 3: eliminate command substitution in operational code. BUK: date to temp file in dispatch bootstrap, find-emptiness checks removed (redundant after wave 2 error handling), git describe to temp file, find|head to temp file+read in buc_tabtarget, nested printf|sed|wc replaced with ${#var}+printf-v builtins. RBK: grep|head|sed pipelines replaced with while-read+case builtins for vessel mode extraction and HTTP digest extraction, git rev-parse/config to temp files in mirror and inscribe functions, date to temp files for build timestamps and vouch timestamps, grep in rbrr_cli replaced with while-read+case builtin

### 2026-03-13 20:32 - Heat - d

paddock curried

### 2026-03-13 20:31 - Heat - T

bcg-compliance-audit

### 2026-03-13 20:27 - ₢AUAAZ - n

Deep BCG compliance audit wave 2: rename zbuc_do_execute to zbuc_doc_mode_predicate (non-predicate in if condition), convert [[ ]] to test in version checks, split combined local+capture into two-line pattern, add || buc_die to 10 unguarded mkdir/cp/rm/date commands, replace head -1 with read -r builtin, convert heredoc to echo sequence, replace dirname with parameter expansion, add readonly to kindle array ZRBRR_DOCKER_ENV

### 2026-03-13 19:48 - Heat - d

paddock curried

### 2026-03-13 19:48 - ₢AUAAZ - n

BCG correctness fixes across 15 files: eliminate 5 pseudo-ternary A&&B||C patterns (buc_command, buto_operations, rbgu_Utility, rbtcrv_RegimeValidation), replace 11 non-POSIX [[ == ]] with test = (rbgg, rbob, rboo, rbrn_regime), convert all [ ] bracket tests to test command (buc_command, bud_dispatch, bug_guide, rbgp_Payor, rbgu, rbgo, rbgm, rbj_sentry), replace eval with ${!name} indirect expansion (buc_command, bud_dispatch), add name validation before remaining eval (buv_validation), replace echo -e with printf %b (buc_command, bug_guide, buto_operations, rbgm)

### 2026-03-13 19:40 - Heat - T

bcg-compliance-audit

### 2026-03-13 19:25 - ₢AUAAb - W

Containerized syft SBOM for bind mirror using RBRG_SYFT_IMAGE_REF via docker run (matching conjure Cloud Build pattern), plus load-bearing digest fidelity verification in bind vouch comparing GAR Docker-Content-Digest against RBRV_BIND_IMAGE pin. Updated inspect display to show pin/GAR digest comparison and match verdict. Full pipeline verified end-to-end with 305-package SBOM.

### 2026-03-13 19:18 - ₢AUAAb - n

Update pluml nameplate for digest-verified mirror test

### 2026-03-13 19:16 - ₢AUAAb - n

Add digest fidelity verification to bind vouch: compares GAR Docker-Content-Digest against RBRV_BIND_IMAGE pin digest, records both in vouch_summary.json with match verdict, updated inspect display to show digest comparison

### 2026-03-13 19:10 - ₢AUAAb - n

Update pluml nameplate to new consecration with SBOM

### 2026-03-13 19:08 - ₢AUAAb - n

Replace local syft binary check with containerized syft via docker run using RBRG_SYFT_IMAGE_REF against mirrored GAR image, matching conjure Cloud Build pattern

### 2026-03-13 19:07 - Heat - S

bind-mirror-syft-via-container

### 2026-03-13 18:58 - Heat - n

Add collaboration style preferences and paddock-first mount protocol guidance — earned through conversation, not designed at a desk

### 2026-03-13 18:46 - ₢AUAAW - W

Bind vessel support end-to-end: converted plantuml to bind mode, added rbf_mirror for upstream-to-GAR mirroring, refactored vouch into bind/conjure branches, fixed abjure for bind vessels, updated inspect display to branch on vessel mode, unified DirectorCreatesArk tabtarget that dispatches conjure vs mirror by vessel mode, fixed fqin validator for digest-pinned refs. Full pipeline verified: mirror → vouch → summon → inspect → bottle start all working for bind vessels.

### 2026-03-13 17:33 - ₢AUAAa - W

Renamed sentry script prefix from rbss to rbj (Recipe Bottle Jailer): file rbss.sentry.sh → rbj_sentry.sh, RBSS_VERBOSE → RBJ_VERBOSE, RBSp → RBJp phase markers, updated rbob_bottle.sh references, added RBJ to CLAUDE.md. Resolves terminal exclusivity violation where rbs parented both spec documents and sentry code. rbw-Qf passed, nsproto-security 22/22 passed.

### 2026-03-13 17:30 - ₢AUAAW - n

Update pluml nameplate bottle consecration to bind mirror result

### 2026-03-13 17:29 - ₢AUAAW - n

Fix rbf_create double-source (peek at vessel mode via grep instead of sourcing), add @ to fqin validator for digest-pinned image refs

### 2026-03-13 17:27 - ₢AUAAW - n

Unified ark creation: renamed DirectorConjuresArk to DirectorCreatesArk with rbf_create dispatcher that routes conjure/bind by vessel mode, removed separate DirectorMirrorsBind tabtarget, updated all RBZ_CONJURE_ARK references to RBZ_CREATE_ARK

### 2026-03-13 17:25 - ₢AUAAa - n

Rename sentry script prefix from rbss to rbj (Recipe Bottle Jailer) — resolves terminal exclusivity violation where rbs parented both spec documents and sentry code

### 2026-03-13 17:19 - ₢AUAAW - n

Bind vessel support: converted plantuml vessel to bind mode, added rbf_mirror for upstream-to-GAR mirroring, refactored vouch into bind/conjure branches, fixed abjure for bind vessels, updated inspect display to branch on vessel mode, added DirectorMirrorsBind tabtarget and zipper entry

### 2026-03-13 17:14 - Heat - S

mint-rbj-jailer-prefix

### 2026-03-13 16:53 - Heat - r

moved AUAAZ after AUAAW

### 2026-03-13 16:51 - Heat - S

bcg-compliance-audit

### 2026-03-13 16:50 - Heat - S

readme-architecture-diagram

### 2026-03-13 15:21 - Heat - S

readme-maturity-warning

### 2026-03-13 15:19 - Heat - T

regime-variable-completeness-check

### 2026-03-13 15:19 - Heat - T

prep-release-slash-command

### 2026-03-13 15:17 - Heat - r

moved AUAAW to first

### 2026-03-13 15:16 - ₢AUAAT - W

Spec updates for inspect and about enrichment: added ark_inspect operation with rbtgo_ark_inspect linked term and rbtc_inspect_full/rbtc_inspect_compact colophons in RBS0. Created RBSAI-ark_inspect.adoc spec. Documented all 6 about-artifact files in RBSCB. Added inspect references in RBSCO, RBSGS, and backlinks in RBSAC/RBSAS/RBSAV. Added RBSAI to CLAUDE.md acronym mappings.

### 2026-03-13 15:16 - ₢AUAAV - W

Added dirty-working-tree guard to rbf_rubric_inscribe. Both guards verified live: unstaged changes and staged changes each produce clear fail-fast error before any credential loading or network calls.

### 2026-03-13 15:15 - ₢AUAAF - W

Replaced branching if/else jq digest extraction with uniform normalize-then-filter pipeline. Validated against 5 of 7 live GCB images (both single-platform and multi-arch); remaining 2 blocked by Docker Hub rate limit, not jq.

### 2026-03-13 15:12 - ₢AUAAT - n

Spec updates for inspect and about enrichment: ark_inspect operation in RBS0 with colophons and linked terms, new RBSAI spec, about artifact contents table in RBSCB, inspect references in RBSCO/RBSGS, backlinks in RBSAC/RBSAS/RBSAV

### 2026-03-13 15:10 - ₢AUAAV - n

Add dirty-working-tree guard to rbf_rubric_inscribe: fail early on unstaged or staged changes so inscribed scripts always match a committed state

### 2026-03-13 15:09 - ₢AUAAF - n

Replace branching if/else jq digest extraction with uniform normalize-then-filter pipeline: first([.] | flatten | .[].Descriptor | select(.platform) | .digest). Handles both single-platform and multi-arch manifests without branching.

### 2026-03-13 15:05 - ₢AUAAU - W

Removed redundant beseech command (rbw-DB) — rbw-Dc (DirectorChecksConsecrations) supersedes it with platform awareness, tri-state health model, and regime-grounded vessel enumeration. Promoted rbw-Dc from bare rbtc_ colophon to full rbtgo_consecration_check operation in RBS0 with new RBSCK spec file. Deleted beseech tabtarget, spec (RBSAB), zipper enrollment, and 130-line implementation from Foundry.

### 2026-03-13 15:05 - ₢AUAAL - W

Enhanced inspect with Build Output section (digest, media type, build ref, image name from buildkit_metadata.json) and Build Cache Delta section (deduped new images from cache_before/cache_after diff) in shared display for both compact and full modes. Moved cache diff from full-only to shared. BCG-compliant variable declarations.

### 2026-03-13 15:03 - ₢AUAAU - n

Remove beseech command (rbw-DB) superseded by rbw-Dc consecration check. Promote rbw-Dc from bare colophon to full rbtgo_consecration_check operation in RBS0 with new RBSCK spec. Delete beseech tabtarget, spec, zipper enrollment, and implementation.

### 2026-03-13 15:00 - ₢AUAAL - n

Enhanced inspect with Build Output section (digest, media type, build ref, image name) and Build Cache Delta section (deduped new images) in shared display. Moved cache diff from full-only to shared. BCG-compliant variable declarations.

### 2026-03-13 14:54 - ₢AUAAQ - W

Verified consumer README.md: all 29 referenced tabtargets exist, rbev-vessels/ survives prep-release, nsproto vessels confirmed. Fixed vessel vs nameplate distinction in both consumer docs (nameplate moniker is the tabtarget imprint, not vessel directory name). Updated vessel structure example with real fleet. Deleted purposeless rbev-nginx-ward bind vessel. Slated AUAAW for bind vessel support with plantuml conversion.

### 2026-03-13 14:54 - ₢AUAAQ - n

Clarify vessel vs nameplate distinction in consumer docs: vessels are build definitions, nameplates tie sentry + bottle vessels into runnable bottles with moniker as tabtarget imprint

### 2026-03-13 14:50 - Heat - T

bind-vessel-support-and-plantuml-conversion

### 2026-03-13 14:48 - Heat - S

bind-vessel-support-and-plantuml-conversion

### 2026-03-13 14:42 - Heat - r

moved AUAAF before AUAAV

### 2026-03-13 14:42 - Heat - r

moved AUAAF to last

### 2026-03-13 14:41 - ₢AUAAN - W

Created consumer CLAUDE.md template with glossary, role summary, full command reference table (~45 commands grouped by role), credential safety, configuration regimes, troubleshooting. Created consumer README.md with complete setup walkthrough (15 steps across 4 phases), vessel directory structure, credential safety, recovery scenarios, Claude Code reference. Added test execution discipline to dev CLAUDE.md. Slated AUAAU for beseech clarification. Reslated 6 downstream pace dockets with insights from consumer doc work.

### 2026-03-13 14:41 - ₢AUAAS - W

Enriched -about artifact with three new platform-independent files: buildkit_metadata.json (--metadata-file on buildx build in step 03), cache_before.json (host daemon snapshot before build in step 03), cache_after.json (host daemon snapshot after pushes in step 05). Added COPY lines in step 08 Dockerfile.meta. Updated inspect: extracts new files, displays BuildKit output digest in Base Image section (shared), displays cache diff with new-image detection in full mode. Discovered Cloud Build workers have pre-populated image caches that must not be pruned. Verified end-to-end with successful busybox consecration.

### 2026-03-13 14:41 - ₢AUAAS - n

Fix empty BuildKit digest display: filter platform keys by containing '/', use top-level containerimage.digest for flat metadata, correct section header text

### 2026-03-13 14:40 - Heat - S

inscribe-dirty-tree-guard

### 2026-03-13 14:37 - ₢AUAAS - n

Remove aggressive cache prune (broke Cloud Build infrastructure images), keep snapshot-only capture. Verified end-to-end: buildkit_metadata.json, cache_before.json, cache_after.json all present in -about, cache diff displays correctly in inspect.

### 2026-03-13 14:36 - ₢AUAAN - n

Draft consumer CLAUDE.md template with glossary, role summary, command reference, credential safety, and troubleshooting; draft consumer README.md with full setup walkthrough, vessel structure, and recovery; add test execution discipline to dev CLAUDE.md

### 2026-03-13 14:36 - Heat - T

clarify-beseech-purpose-and-docs

### 2026-03-13 14:35 - Heat - T

consecration-inspect-command

### 2026-03-13 14:35 - Heat - T

prep-release-slash-command

### 2026-03-13 14:34 - Heat - T

spec-updates-for-inspect-and-about-enrichment

### 2026-03-13 14:34 - Heat - T

freshen-cosmology-and-render

### 2026-03-13 14:34 - Heat - T

getting-started-readme

### 2026-03-13 14:33 - Heat - T

consumer-claudemd-and-test-discipline

### 2026-03-13 14:22 - Heat - S

clarify-beseech-purpose-and-docs

### 2026-03-13 14:01 - ₢AUAAS - n

Enrich -about artifact with BuildKit metadata and Docker cache inventory: prune+snapshot cache_before in step 03, --metadata-file on buildx build, snapshot cache_after in step 05, COPY three new platform-independent files in step 08, extract and display in inspect

### 2026-03-13 13:52 - Heat - S

spec-updates-for-inspect-and-about-enrichment

### 2026-03-13 13:48 - Heat - T

consecration-inspect-command

### 2026-03-13 13:48 - Heat - n

fix stale single-platform vouch description in RBS0 ark vouch definition block

### 2026-03-13 13:47 - Heat - T

enrich-about-with-build-provenance

### 2026-03-13 13:45 - ₢AUAAK - W

Local vouch preflight gate: unconditional docker image inspect for sentry and bottle vouch artifacts on every bottle start, fatal with copy-pasteable summon tabtarget command if missing. Removed remote rbf_vouch_gate call from bottle start path (function preserved for test code). Updated RBSBS spec. Also fixed last two hardcoded buc_tabtarget colophons in rbf_Foundry.sh to use zipper constants.

### 2026-03-13 13:44 - Heat - S

enrich-about-with-build-provenance

### 2026-03-13 13:35 - ₢AUAAL - n

Implement consecration inspect command: rbf_inspect_full (rbw-RiF) and rbf_inspect_compact (rbw-Ric) with shared sections, bind/conjure vessel handling, SLSA boundary callouts, base image identity, SBOM summary, two-listing package inventory (type/name/version + name/license/purl), and Dockerfile display

### 2026-03-13 13:25 - ₢AUAAK - n

replace hardcoded rbw-DV and rbw-DA colophons with zipper constants RBZ_VOUCH_ARK and RBZ_ABJURE_ARK

### 2026-03-13 13:22 - ₢AUAAK - n

local-vouch-preflight-gate: unconditional docker image inspect for vouch artifacts on bottle start, fatal with copy-pasteable summon tabtarget if missing, removed remote vouch gate call

### 2026-03-13 13:03 - ₢AUAAJ - W

Vouch artifact now multi-platform via buildx: rbgjv03 uses docker buildx build --push with _RBGV_PLATFORMS, foundry JSON passes platforms and private pool, governor grants workerPoolUser to director. Renamed poll terminology, bumped vouch timeout. Specs updated (RBSAV, RBSDI, RBS0). Verified end-to-end: vouch builds on private pool, manifest has amd64/arm64/armv7, summon pulls cleanly on arm64.

### 2026-03-13 12:58 - Heat - T

consecration-inspect-command

### 2026-03-13 12:50 - ₢AUAAO - W

Consolidated all lenses/ files into veiled and memos. RBRN moved to Tools/rbk/vov_veiled/ (RBS0 include target). CRR and RBWMBX moved to Memos/ with datestamp naming convention. AXMCM moved to Tools/cmk/vov_veiled/. Moved 3 dormant podman scripts (rbv_PodmanVM.sh, rbv_cli.sh, rbrm_regime.sh), rbrm.env assignment file, and ABANDONED-github/ into vov_veiled. Deleted orphan Tools/test/. Updated CLAUDE.md: removed lenses section, added Memos/ permission, updated file paths. Updated AUAAR strip list with full Tools/ subdirectory inventory. lenses/ now empty.

### 2026-03-13 12:50 - ₢AUAAJ - n

fix vouch buildx Dockerfile path, rename attempt→poll, increase vouch timeout to 50

### 2026-03-13 12:45 - ₢AUAAJ - n

vouch-artifact-multiplatform: buildx in rbgjv03, platforms+pool in foundry JSON, workerPoolUser grant in governor, spec updates

### 2026-03-13 12:38 - ₢AUAAO - n

Move dormant podman scripts (rbv_PodmanVM.sh, rbv_cli.sh, rbrm_regime.sh), rbrm.env assignment file, and ABANDONED-github/ into vov_veiled for publication stripping. Delete orphan Tools/test/ directory.

### 2026-03-13 12:33 - Heat - T

prep-release-slash-command

### 2026-03-13 12:30 - ₢AUAAO - n

Consolidate lenses/ into veiled and memos. RBRN moved to Tools/rbk/vov_veiled/ (RBS0 include target). CRR and RBWMBX moved to Memos/ with datestamp naming. AXMCM moved to Tools/cmk/vov_veiled/. lenses/ now empty. CLAUDE.md updated: removed lenses section, added Memos/ permission, updated RBRN and AXMCM paths.

### 2026-03-13 12:20 - Heat - T

vouch-artifact-multiplatform

### 2026-03-13 12:19 - Heat - d

paddock curried

### 2026-03-13 12:19 - Heat - T

prep-release-slash-command

### 2026-03-13 12:18 - Heat - T

getting-started-readme

### 2026-03-13 12:18 - Heat - T

freshen-cosmology-and-render

### 2026-03-13 12:18 - Heat - T

regime-variable-completeness-check

### 2026-03-13 12:18 - Heat - T

consumer-claudemd-and-test-discipline

### 2026-03-13 12:15 - Heat - r

moved AUAAN after AUAAO

### 2026-03-13 12:14 - Heat - S

prep-release-slash-command

### 2026-03-13 12:13 - Heat - S

getting-started-readme

### 2026-03-13 12:13 - Heat - S

freshen-cosmology-and-render

### 2026-03-13 11:53 - Heat - S

consolidate-lenses-into-veiled

### 2026-03-13 11:41 - ₢AUAAI - W

Split qualification into fast (rbw-Qf) and release (rbw-QR) tiers. Renamed rbw-qa to rbw-Qf: runs tabtargets, colophons, nameplate preflight only — no shellcheck on bottle-start. New rbw-QR release gate runs fast qualify, then shellcheck, then complete test suite sequentially. Updated workbench gate, zipper enrollment, and test case to match.

### 2026-03-13 11:41 - ₢AUAAE - W

Added regime-validation fixture to rbtb_testbench.sh with 21 test cases across RBRR, RBRV, and RBRN regimes. 18 negative tests exercise missing required fields, invalid enum/format values, scope sentinels for unexpected variables, cross-field port conflicts, non-existent directories, and bad IP formats. 3 positive tests validate all real config files (1 rbrr.env, 8 vessel rbrv.env, 3 nameplate rbrn_*.env) via direct kindle+enforce. Baste sources rbrr.env without kindling to keep RBRR_ vars mutable for negative test overrides. Fast suite passes 78 cases across 5 fixtures.

### 2026-03-13 11:41 - ₢AUAAE - n

Add regime validation testbench with 21 test cases covering RBRR, RBRV, and RBRN validators. Negative tests exercise missing fields, invalid formats, scope sentinels, cross-field conflicts, and non-existent directories. Positive tests validate all real rbrr.env, vessel rbrv.env, and nameplate rbrn_*.env files via kindle+enforce. Fast suite passes 78 cases.

### 2026-03-13 11:38 - ₢AUAAI - n

Split qualification into fast (rbw-Qf) and release (rbw-QR) tiers. Bottle-start no longer runs shellcheck — fast qualify does tabtargets, colophons, and nameplate preflight only. New rbw-QR release gate runs fast qualify then shellcheck then complete test suite sequentially.

### 2026-03-13 11:34 - Heat - T

release-qualification-gate

### 2026-03-13 11:13 - ₢AUAAA - W

Removed RBRV_CONJURE_BINFMT_POLICY field from regime enrollment, all 7 vessel configs, and specs. Build strategy now derived from platform-vs-runner string equality: rbgjb02 (privileged binfmt container) conditionally included in stitched Cloud Build JSON. Non-inferential build strategy logged at both stitch time and Cloud Build runtime (step 01). Verified A/B: multi-platform busybox shows 9 steps with qemu-binfmt, native-verify busybox shows 8 steps without. Improved conjure error message to use buc_tabtarget for inscribe hint. Created rbev-busybox-native-verify vessel for ongoing native-path regression.

### 2026-03-13 11:13 - ₢AUAAA - n

Improve missing-trigger error in rbf_build to show tabtarget hint for rubric inscribe

### 2026-03-13 11:00 - ₢AUAAA - n

Remove RBRV_CONJURE_BINFMT_POLICY field, derive build strategy from platform-vs-runner comparison, conditionally include rbgjb02, add non-inferential build strategy logging at stitch and Cloud Build time

### 2026-03-13 10:55 - ₢AUAAA - n

Create native-only busybox vessel for verifying binfmt conditional exclusion

### 2026-03-13 10:51 - Heat - T

derive-build-strategy-from-platforms

### 2026-03-13 10:49 - Heat - T

derive-build-strategy-from-platforms

### 2026-03-13 10:33 - Heat - T

simplify-manifest-digest-extraction

### 2026-03-13 10:08 - Heat - r

moved AUAAE after AUAAI

### 2026-03-13 10:08 - Heat - r

moved AUAAK after AUAAJ

### 2026-03-13 10:08 - Heat - r

moved AUAAJ after AUAAA

### 2026-03-13 10:08 - Heat - r

moved AUAAA after AUAAF

### 2026-03-13 10:08 - Heat - r

moved AUAAF to first

### 2026-03-13 10:08 - Heat - T

design-consumer-claudemd-guidance

### 2026-03-12 19:11 - Heat - S

consumer-claudemd-and-test-discipline

### 2026-03-11 15:16 - Heat - S

release-rbrr-reset

### 2026-03-10 19:55 - Heat - T

regime-validation-testbench

### 2026-03-10 19:52 - Heat - S

consecration-inspect-command

### 2026-03-10 19:51 - Heat - T

derive-build-strategy-from-platforms

### 2026-03-10 19:51 - Heat - T

decide-or-remove-rbrv-conjure-binfmt-policy

### 2026-03-10 19:35 - Heat - d

paddock curried

### 2026-03-10 19:34 - Heat - T

directory-restructure-prep

### 2026-03-10 19:30 - Heat - n

Rename DirectorVouchesArk to DirectorVouchesConsecrations to reflect batch scope

### 2026-03-10 19:28 - Heat - r

moved AUAAE after AUAAA

### 2026-03-10 19:28 - Heat - r

moved AUAAF after AUAAI

### 2026-03-10 19:25 - Heat - T

local-vouch-preflight-gate

### 2026-03-10 19:25 - Heat - T

vouch-artifact-multiplatform

### 2026-03-10 19:22 - Heat - S

local-vouch-preflight-gate

### 2026-03-10 19:20 - Heat - n

Summon now pulls -vouch artifact alongside -image and -about

### 2026-03-10 19:19 - Heat - S

vouch-artifact-multiplatform

### 2026-03-10 19:01 - Heat - r

moved AUAAI to first

### 2026-03-10 19:01 - Heat - S

release-qualification-gate

### 2026-03-10 18:53 - Heat - D

restring 1 paces from ₣As

### 2026-03-10 18:53 - Heat - f

racing, silks=rbk-mvp-3-release-finalize

### 2026-03-08 12:29 - Heat - T

move-rbs-specs-to-rbk-veiled

### 2026-03-03 19:18 - Heat - S

design-consumer-claudemd-guidance

### 2026-02-13 08:08 - Heat - S

simplify-manifest-digest-extraction

### 2026-02-09 16:50 - Heat - S

regime-validation-testbench

### 2026-02-09 05:55 - Heat - S

move-rbs-specs-to-rbk-veiled

### 2026-02-09 05:52 - Heat - S

delete-rbk-coordinator

### 2026-02-09 05:52 - Heat - d

paddock curried (refine)

### 2026-02-07 12:58 - Heat - n

Introduce Operator architectural term to distinguish the person at the keyboard from the Consumer Role they may assume; fix role conflation in RBRA file definitions across RBS and RBAGS specs

### 2026-02-02 19:44 - Heat - S

directory-restructure-prep

### 2026-01-31 12:20 - Heat - f

stabled

### 2026-01-31 12:15 - Heat - f

racing

### 2026-01-31 12:14 - Heat - S

decide-or-remove-rbrv-conjure-binfmt-policy

### 2026-01-31 12:13 - Heat - N

rbw-mvp-release-finalize

