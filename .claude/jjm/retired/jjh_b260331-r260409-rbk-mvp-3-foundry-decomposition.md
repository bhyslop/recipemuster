# Heat Trophy: rbk-mvp-3-foundry-decomposition

**Firemark:** ₣A0
**Created:** 260331
**Retired:** 260409
**Status:** retired

## Paddock

## Gestalt

rbf_Foundry.sh is a ~4,600-line monolith — 19 public and ~25 private functions spanning four unrelated domains that share a single kindle/sentinel gate. This heat explodes it into five independently-sourceable modules with proper credential boundaries.

## Why now

The monolith forces every Foundry consumer to pass director credential checks even for operations that don't need them (retriever-only wrest/summon, zero-credential plumb). This is the director-gate problem surfaced in ₣AU (₢AUAAn). Decomposition resolves it structurally rather than with workarounds.

GitLab integration elimination in ₣Av left dead code throughout Foundry — cleaning that first reduces noise in the decomposition.

## Target architecture

| Module | File | Domain | Credentials |
|--------|------|--------|-------------|
| rbfc | rbfc_FoundryCore.sh | kindle, sentinel, GCB poll/wait, stitch, plumb | none |
| rbfd | rbfd_FoundryDirectorBuild.sh | ordain, conjure, enshrine, graft, kludge, mirror | director |
| rbfv | rbfv_FoundryVerify.sh | about, vouch, batch_vouch | director |
| rbfl | rbfl_FoundryLedger.sh | inscribe, tally, abjure, delete | director |
| rbfr | rbfr_FoundryRetriever.sh | wrest, summon | retriever |

All child modules source rbfc. rbf becomes a non-terminal parent prefix.

## Sequence rationale

Clean (gitlab purge) → Plan (dependency map) → Core first (shared infra) → Retriever (cleanest cut, no director deps) → Verify → Ledger → Director-Build (residual rename).

## References

- rbf_Foundry.sh — the monolith
- ₣Av — GitLab elimination heat (builds.create + pouch replaced trigger dispatch)
- ₢AUAAn in ₣AU — director-gate problem this decomposition resolves

## Paces

### purge-gitlab-and-rubric-inscribe (₢A0AAA) [complete]

**[260331-1833] complete**

## Character
Mechanical dead code removal — scan and delete. No behavioral change.

## Context
GitLab integration was eliminated in ₣Av (builds.create + pouch replaced trigger dispatch). Dead code and comments remain across shell scripts, specs, and consumer docs. rbf_rubric_inscribe is a dead function with GitLab token fetch logic. Clean all of this before decomposing Foundry.

## Inventory

### Dead code to delete
- rbf_Foundry.sh: entire rbf_rubric_inscribe() function
- rbgc_Constants.sh: CBv2 GitLab constants (secret names, connection suffix)
- rbgm_ManualProcedures.sh: rbgm_gitlab_setup() stub function
- rbgm_cli.sh: rbgm_gitlab_setup dispatch entry
- rbz_zipper.sh: RBZ_GITLAB_SETUP zipper enrollment
- tt/rbw-gPL.GitLabSetup.sh: tabtarget (if still exists)

### Spec/doc references to clean
- RBSRI-rubric_inscribe.adoc: entire doc describes dead GitLab flow — assess delete or gut
- RBS0-SpecTop.adoc: GitLab comments + eliminated tabtarget colophon
- README.consumer.md: GitLab account requirement, rubric repo explanation, setup step
- CLAUDE.consumer.md: GitLab rubric description + rbw-gPL reference
- RBSDE-depot_levy.adoc, RBSDK-director_knight.adoc, RBSRR-RegimeRepo.adoc: elimination comments
- rbgg_Governor.sh, rblm_cli.sh: GitLab comments

## Pre-work
Rescan for gitlab (case-insensitive) across all of Tools/rbk/ on mount.

## Acceptance
- Zero occurrences of gitlab in shell scripts and consumer docs
- Spec comments may remain as historical notes if they document WHY — use judgment
- All existing tests pass

**[260331-1419] rough**

## Character
Mechanical dead code removal — scan and delete. No behavioral change.

## Context
GitLab integration was eliminated in ₣Av (builds.create + pouch replaced trigger dispatch). Dead code and comments remain across shell scripts, specs, and consumer docs. rbf_rubric_inscribe is a dead function with GitLab token fetch logic. Clean all of this before decomposing Foundry.

## Inventory

### Dead code to delete
- rbf_Foundry.sh: entire rbf_rubric_inscribe() function
- rbgc_Constants.sh: CBv2 GitLab constants (secret names, connection suffix)
- rbgm_ManualProcedures.sh: rbgm_gitlab_setup() stub function
- rbgm_cli.sh: rbgm_gitlab_setup dispatch entry
- rbz_zipper.sh: RBZ_GITLAB_SETUP zipper enrollment
- tt/rbw-gPL.GitLabSetup.sh: tabtarget (if still exists)

### Spec/doc references to clean
- RBSRI-rubric_inscribe.adoc: entire doc describes dead GitLab flow — assess delete or gut
- RBS0-SpecTop.adoc: GitLab comments + eliminated tabtarget colophon
- README.consumer.md: GitLab account requirement, rubric repo explanation, setup step
- CLAUDE.consumer.md: GitLab rubric description + rbw-gPL reference
- RBSDE-depot_levy.adoc, RBSDK-director_knight.adoc, RBSRR-RegimeRepo.adoc: elimination comments
- rbgg_Governor.sh, rblm_cli.sh: GitLab comments

## Pre-work
Rescan for gitlab (case-insensitive) across all of Tools/rbk/ on mount.

## Acceptance
- Zero occurrences of gitlab in shell scripts and consumer docs
- Spec comments may remain as historical notes if they document WHY — use judgment
- All existing tests pass

### foundry-decomposition-plan (₢A0AAB) [complete]

**[260331-1853] complete**

## Character
Design conversation — read the monolith, map function dependencies, decide exact file boundaries. No code changes.

## Context
rbf_Foundry.sh is ~4,600 lines with ~19 public + ~25 private functions spanning 4 domains. Decomposition plan from grooming session:

| Prefix | Name | Domain |
|--------|------|--------|
| rbf | Foundry (non-terminal parent) | — |
| rbfd | Foundry Director: Build | ordain, conjure, enshrine, graft, kludge, mirror |
| rbfv | Foundry Verify | about, vouch, batch_vouch |
| rbfl | Foundry Ledger | inscribe, tally, abjure, delete |
| rbfr | Foundry Retriever | wrest, summon |
| rbfc | Foundry Core | kindle, sentinel, GCB poll/wait, stitch, fact files, shared helpers, plumb_full, plumb_compact |

## Deliverable
A concrete file-level plan documenting:
1. Which public functions go to which new module
2. Which private (zrbf_*) functions are shared vs module-specific
3. Sourcing chain: how do child modules access core?
4. CLI routing: one rbf_cli.sh dispatcher, or per-module CLIs?
5. Kindle/sentinel changes: per-module kindle, or shared?
6. Director credential check placement per module
7. Exact list of files to create, modify, delete

## Notes
- Ax renames will have completed before this mounts — use post-Ax names
- rbf_rubric_inscribe already deleted in prior pace
- Function names listed above are pre-Ax; rescan on mount
- plumb functions use no credentials — placed in core, not retriever

## Acceptance
- Written plan reviewed with user before any code changes

**[260331-1650] rough**

## Character
Design conversation — read the monolith, map function dependencies, decide exact file boundaries. No code changes.

## Context
rbf_Foundry.sh is ~4,600 lines with ~19 public + ~25 private functions spanning 4 domains. Decomposition plan from grooming session:

| Prefix | Name | Domain |
|--------|------|--------|
| rbf | Foundry (non-terminal parent) | — |
| rbfd | Foundry Director: Build | ordain, conjure, enshrine, graft, kludge, mirror |
| rbfv | Foundry Verify | about, vouch, batch_vouch |
| rbfl | Foundry Ledger | inscribe, tally, abjure, delete |
| rbfr | Foundry Retriever | wrest, summon |
| rbfc | Foundry Core | kindle, sentinel, GCB poll/wait, stitch, fact files, shared helpers, plumb_full, plumb_compact |

## Deliverable
A concrete file-level plan documenting:
1. Which public functions go to which new module
2. Which private (zrbf_*) functions are shared vs module-specific
3. Sourcing chain: how do child modules access core?
4. CLI routing: one rbf_cli.sh dispatcher, or per-module CLIs?
5. Kindle/sentinel changes: per-module kindle, or shared?
6. Director credential check placement per module
7. Exact list of files to create, modify, delete

## Notes
- Ax renames will have completed before this mounts — use post-Ax names
- rbf_rubric_inscribe already deleted in prior pace
- Function names listed above are pre-Ax; rescan on mount
- plumb functions use no credentials — placed in core, not retriever

## Acceptance
- Written plan reviewed with user before any code changes

**[260331-1419] rough**

## Character
Design conversation — read the monolith, map function dependencies, decide exact file boundaries. No code changes.

## Context
rbf_Foundry.sh is ~4,600 lines with ~19 public + ~25 private functions spanning 4 domains. Decomposition plan from grooming session:

| Prefix | Name | Domain |
|--------|------|--------|
| rbf | Foundry (non-terminal parent) | — |
| rbfd | Foundry Director: Build | ordain, conjure, enshrine, graft, kludge, mirror |
| rbfv | Foundry Verify | about, vouch, batch_vouch |
| rbfl | Foundry Ledger | inscribe, tally, abjure, delete |
| rbfr | Foundry Retriever | retrieve, summon, inspect_full, inspect_compact |
| rbfc | Foundry Core | kindle, sentinel, GCB poll/wait, stitch, fact files, shared helpers |

## Deliverable
A concrete file-level plan documenting:
1. Which public functions go to which new module
2. Which private (zrbf_*) functions are shared vs module-specific
3. Sourcing chain: how do child modules access core?
4. CLI routing: one rbf_cli.sh dispatcher, or per-module CLIs?
5. Kindle/sentinel changes: per-module kindle, or shared?
6. Director credential check placement per module
7. Exact list of files to create, modify, delete

## Notes
- Ax renames will have completed before this mounts — use post-Ax names
- rbf_rubric_inscribe already deleted in prior pace
- Function names listed above are pre-Ax; rescan on mount

## Acceptance
- Written plan reviewed with user before any code changes

### extract-foundry-core (₢A0AAC) [complete]

**[260401-0438] complete**

## Character
Heavy refactoring — extract 18 shared functions, rename all to zrbfc_*/rbfc_*, update 50+ call sites in the monolith. BCG-compliant module creation.

## Context
Create rbfc_FoundryCore.sh as the shared infrastructure module that all other Foundry child modules depend on. This is the heaviest extraction pace because every subsequent module relies on core, and all cross-references in the monolith must be updated to use the new rbfc prefix.

## BCG Compliance
- ZRBFC_SOURCED guard, ZRBFC_KINDLED=1 last in kindle
- zrbfc_kindle(): shared API endpoints, registry host/path, vessel files, git metadata files, scratch/output files. NO credential checks. NO module-specific temp prefixes.
- zrbfc_sentinel() guard on all functions
- All functions prefixed zrbfc_* (private) or rbfc_* (public)

## Functions to extract and rename

### Public (rbf_ → rbfc_)
- rbf_plumb_full → rbfc_plumb_full
- rbf_plumb_compact → rbfc_plumb_compact

### Shared private (zrbf_ → zrbfc_)
- zrbf_resolve_vessel → zrbfc_resolve_vessel
- zrbf_load_vessel → zrbfc_load_vessel
- zrbf_wait_build_completion → zrbfc_wait_build_completion
- zrbf_resolve_tool_images → zrbfc_resolve_tool_images
- zrbf_ensure_git_metadata → zrbfc_ensure_git_metadata
- zrbf_assemble_about_steps → zrbfc_assemble_about_steps
- zrbf_assemble_vouch_steps → zrbfc_assemble_vouch_steps

### Plumb private (zrbf_ → zrbfc_)
- zrbf_plumb_core → zrbfc_plumb_core
- zrbf_plumb_show_bind → zrbfc_plumb_show_bind
- zrbf_plumb_show_sections → zrbfc_plumb_show_sections
- zrbf_plumb_show_compact → zrbfc_plumb_show_compact
- zrbf_plumb_show_full → zrbfc_plumb_show_full

### Dead code to delete
- zrbf_base64_decode_capture (zero callers after gitlab purge)

## Kindle decomposition
Extract from the monolithic zrbf_kindle() into zrbfc_kindle():
- API base URLs (GCB, GAR, cloud console query base)
- GCB project builds URL, GAR package base
- Registry host/path/API base
- Git metadata files (commit, branch, repo)
- Vessel files (sigil, resolved dir)
- Output files, scratch file

Leave in monolith (for later extraction into per-module kindles):
- Director RBRA check
- Step script directories (per-module)
- Module-specific temp prefixes

## Transition wiring
- Monolith sources rbfc_FoundryCore.sh
- Monolith calls zrbfc_kindle() from its own zrbf_kindle()
- All call sites in monolith updated to use zrbfc_* names
- Create rbfc_cli.sh for plumb commands (lighter furnish: no OAuth, no Director RBRA)
- Update rbz_zipper.sh: plumb enrollments → rbfc_cli.sh with rbfc_ function names

## Acceptance
- rbfc_FoundryCore.sh exists as BCG-compliant module
- rbfc_cli.sh dispatches rbfc_plumb_full, rbfc_plumb_compact
- Plumb works without Director RBRA file present
- Monolith sources rbfc; all existing non-plumb behavior unchanged
- All tests pass — pure refactor, no behavioral change

**[260331-1850] rough**

## Character
Heavy refactoring — extract 18 shared functions, rename all to zrbfc_*/rbfc_*, update 50+ call sites in the monolith. BCG-compliant module creation.

## Context
Create rbfc_FoundryCore.sh as the shared infrastructure module that all other Foundry child modules depend on. This is the heaviest extraction pace because every subsequent module relies on core, and all cross-references in the monolith must be updated to use the new rbfc prefix.

## BCG Compliance
- ZRBFC_SOURCED guard, ZRBFC_KINDLED=1 last in kindle
- zrbfc_kindle(): shared API endpoints, registry host/path, vessel files, git metadata files, scratch/output files. NO credential checks. NO module-specific temp prefixes.
- zrbfc_sentinel() guard on all functions
- All functions prefixed zrbfc_* (private) or rbfc_* (public)

## Functions to extract and rename

### Public (rbf_ → rbfc_)
- rbf_plumb_full → rbfc_plumb_full
- rbf_plumb_compact → rbfc_plumb_compact

### Shared private (zrbf_ → zrbfc_)
- zrbf_resolve_vessel → zrbfc_resolve_vessel
- zrbf_load_vessel → zrbfc_load_vessel
- zrbf_wait_build_completion → zrbfc_wait_build_completion
- zrbf_resolve_tool_images → zrbfc_resolve_tool_images
- zrbf_ensure_git_metadata → zrbfc_ensure_git_metadata
- zrbf_assemble_about_steps → zrbfc_assemble_about_steps
- zrbf_assemble_vouch_steps → zrbfc_assemble_vouch_steps

### Plumb private (zrbf_ → zrbfc_)
- zrbf_plumb_core → zrbfc_plumb_core
- zrbf_plumb_show_bind → zrbfc_plumb_show_bind
- zrbf_plumb_show_sections → zrbfc_plumb_show_sections
- zrbf_plumb_show_compact → zrbfc_plumb_show_compact
- zrbf_plumb_show_full → zrbfc_plumb_show_full

### Dead code to delete
- zrbf_base64_decode_capture (zero callers after gitlab purge)

## Kindle decomposition
Extract from the monolithic zrbf_kindle() into zrbfc_kindle():
- API base URLs (GCB, GAR, cloud console query base)
- GCB project builds URL, GAR package base
- Registry host/path/API base
- Git metadata files (commit, branch, repo)
- Vessel files (sigil, resolved dir)
- Output files, scratch file

Leave in monolith (for later extraction into per-module kindles):
- Director RBRA check
- Step script directories (per-module)
- Module-specific temp prefixes

## Transition wiring
- Monolith sources rbfc_FoundryCore.sh
- Monolith calls zrbfc_kindle() from its own zrbf_kindle()
- All call sites in monolith updated to use zrbfc_* names
- Create rbfc_cli.sh for plumb commands (lighter furnish: no OAuth, no Director RBRA)
- Update rbz_zipper.sh: plumb enrollments → rbfc_cli.sh with rbfc_ function names

## Acceptance
- rbfc_FoundryCore.sh exists as BCG-compliant module
- rbfc_cli.sh dispatches rbfc_plumb_full, rbfc_plumb_compact
- Plumb works without Director RBRA file present
- Monolith sources rbfc; all existing non-plumb behavior unchanged
- All tests pass — pure refactor, no behavioral change

**[260331-1650] rough**

## Character
Careful refactoring — extract shared internals into rbfc without changing any behavior.

## Context
Create rbfc (Foundry Core) containing the shared infrastructure that all child modules need: kindle/sentinel, GCB build submission + polling, stitch (build JSON assembly), base64 decode capture, fact file handling. Also includes plumb_full and plumb_compact — these inspect locally-pulled images and require no credentials, so they belong in core rather than gating on retriever or director creds.

## Pre-work
Depends on decomposition plan pace for exact function list and sourcing design.

## Acceptance
- rbfc module exists with extracted shared functions
- plumb_full and plumb_compact work without any credential files present
- rbf_Foundry.sh sources rbfc and all existing behavior is unchanged
- All tests pass — pure refactor, no behavioral change

**[260331-1420] rough**

## Character
Careful refactoring — extract shared internals into rbfc without changing any behavior.

## Context
Create rbfc (Foundry Core) containing the shared infrastructure that all child modules need: kindle/sentinel, GCB build submission + polling, stitch (build JSON assembly), base64 decode capture, fact file handling. This must land first — the other extractions depend on it.

## Pre-work
Depends on decomposition plan pace for exact function list and sourcing design.

## Acceptance
- rbfc module exists with extracted shared functions
- rbf_Foundry.sh sources rbfc and all existing behavior is unchanged
- All tests pass — pure refactor, no behavioral change

### extract-foundry-retriever (₢A0AAD) [complete]

**[260401-0444] complete**

## Character
Surgical extraction — simplest module, two public functions, zero private helpers. Proves the extraction pattern established by core.

## Context
Create rbfr_FoundryRetriever.sh as BCG-compliant module. Wrest and summon are the cleanest cut: they use only retriever credentials, have no module-specific private helpers, and depend only on shared rbfc infrastructure.

This extraction naturally resolves the director-gate problem (₢AUAAn in ₣AU) — retriever operations no longer share a kindle with director operations and no longer require Director RBRA to exist.

## BCG Compliance
- ZRBFR_SOURCED guard, ZRBFR_KINDLED=1 last
- zrbfr_kindle(): calls zrbfc_kindle(), no Director RBRA check, minimal module state
- zrbfr_sentinel() guard

## Functions to extract and rename
- rbf_wrest → rbfr_wrest
- rbf_summon → rbfr_summon

## Transition wiring
- rbfr_FoundryRetriever.sh sources rbfc_FoundryCore.sh
- Create rbfr_cli.sh (lighter furnish: no Director RBRA, sources rbfc + rbfr)
- Update rbz_zipper.sh: wrest/summon enrollments → rbfr_cli.sh with rbfr_ names
- Remove wrest/summon from monolith
- Update RBS0 spec references for renamed functions

## Acceptance
- rbfr module exists with wrest and summon
- Retriever operations work on a station with only retriever credentials (no Director RBRA needed)
- All tests pass

**[260331-1850] rough**

## Character
Surgical extraction — simplest module, two public functions, zero private helpers. Proves the extraction pattern established by core.

## Context
Create rbfr_FoundryRetriever.sh as BCG-compliant module. Wrest and summon are the cleanest cut: they use only retriever credentials, have no module-specific private helpers, and depend only on shared rbfc infrastructure.

This extraction naturally resolves the director-gate problem (₢AUAAn in ₣AU) — retriever operations no longer share a kindle with director operations and no longer require Director RBRA to exist.

## BCG Compliance
- ZRBFR_SOURCED guard, ZRBFR_KINDLED=1 last
- zrbfr_kindle(): calls zrbfc_kindle(), no Director RBRA check, minimal module state
- zrbfr_sentinel() guard

## Functions to extract and rename
- rbf_wrest → rbfr_wrest
- rbf_summon → rbfr_summon

## Transition wiring
- rbfr_FoundryRetriever.sh sources rbfc_FoundryCore.sh
- Create rbfr_cli.sh (lighter furnish: no Director RBRA, sources rbfc + rbfr)
- Update rbz_zipper.sh: wrest/summon enrollments → rbfr_cli.sh with rbfr_ names
- Remove wrest/summon from monolith
- Update RBS0 spec references for renamed functions

## Acceptance
- rbfr module exists with wrest and summon
- Retriever operations work on a station with only retriever credentials (no Director RBRA needed)
- All tests pass

**[260331-1650] rough**

## Character
Surgical extraction — retriever functions have no director credential dependency, making this the cleanest cut.

## Context
Extract rbfr (Foundry Retriever) from Foundry: the wrest and summon operations. These functions only need retriever credentials. After extraction, rbfr sources rbfc for shared infrastructure but does NOT check for director credentials at kindle time.

This extraction naturally resolves the director-gate problem (₢AUAAn in ₣AU) — retriever operations no longer share a kindle with director operations.

Note: plumb (inspect) operations were considered for rbfr but moved to rbfc — they use zero credentials and shouldn't gate on retriever creds either.

## Acceptance
- rbfr module exists with wrest and summon
- Retriever operations work on a station with only retriever credentials
- All tests pass

**[260331-1420] rough**

## Character
Surgical extraction — retriever functions have no director credential dependency, making this the cleanest cut.

## Context
Extract rbfr (Foundry Retriever) from Foundry: the retrieve, summon, and inspect operations. These functions only need retriever credentials. After extraction, rbfr sources rbfc for shared infrastructure but does NOT check for director credentials at kindle time.

This extraction naturally resolves the director-gate problem (₢AUAAn in ₣AU) — retriever operations no longer share a kindle with director operations.

## Acceptance
- rbfr module exists with retriever functions
- Retriever operations work on a station with only retriever credentials
- All tests pass

### extract-foundry-verify (₢A0AAE) [complete]

**[260401-0512] complete**

## Character
Moderate extraction — 4 public + 3 private functions. Cross-module caller from ordain requires care.

## Context
Create rbfv_FoundryVerify.sh as BCG-compliant module. The verify domain: vouch_gate, about, vouch, batch_vouch plus their submit helpers (graft_metadata_submit, about_submit, vouch_submit). Shared helpers (ensure_git_metadata, assemble_about/vouch_steps) are already in rbfc by this point.

Cross-module dependency: rbfd_ordain (still in monolith at this point) calls rbfv_vouch and zrbfv_graft_metadata_submit. During transition, the monolith sources rbfv so these calls work.

## BCG Compliance
- ZRBFV_SOURCED guard, ZRBFV_KINDLED=1 last
- zrbfv_kindle(): calls zrbfc_kindle(), checks Director RBRA, sets up vouch/about temp prefixes, step dirs (rbgjv, rbgja)
- zrbfv_sentinel() guard

## Functions to extract and rename

### Public
- rbf_vouch_gate → rbfv_vouch_gate
- rbf_about → rbfv_about
- rbf_vouch → rbfv_vouch
- rbf_batch_vouch → rbfv_batch_vouch

### Private
- zrbf_graft_metadata_submit → zrbfv_graft_metadata_submit
- zrbf_about_submit → zrbfv_about_submit
- zrbf_vouch_submit → zrbfv_vouch_submit

## Transition wiring
- rbfv_FoundryVerify.sh sources rbfc_FoundryCore.sh
- Monolith sources rbfv (for ordain's cross-module calls)
- Create rbfv_cli.sh (sources rbfc + rbfv)
- Update rbz_zipper.sh: batch_vouch enrollment → rbfv_cli.sh with rbfv_ name
- Remove verify functions from monolith
- Update RBS0 spec references

## Acceptance
- rbfv module exists with verification functions
- All tests pass

**[260331-1850] rough**

## Character
Moderate extraction — 4 public + 3 private functions. Cross-module caller from ordain requires care.

## Context
Create rbfv_FoundryVerify.sh as BCG-compliant module. The verify domain: vouch_gate, about, vouch, batch_vouch plus their submit helpers (graft_metadata_submit, about_submit, vouch_submit). Shared helpers (ensure_git_metadata, assemble_about/vouch_steps) are already in rbfc by this point.

Cross-module dependency: rbfd_ordain (still in monolith at this point) calls rbfv_vouch and zrbfv_graft_metadata_submit. During transition, the monolith sources rbfv so these calls work.

## BCG Compliance
- ZRBFV_SOURCED guard, ZRBFV_KINDLED=1 last
- zrbfv_kindle(): calls zrbfc_kindle(), checks Director RBRA, sets up vouch/about temp prefixes, step dirs (rbgjv, rbgja)
- zrbfv_sentinel() guard

## Functions to extract and rename

### Public
- rbf_vouch_gate → rbfv_vouch_gate
- rbf_about → rbfv_about
- rbf_vouch → rbfv_vouch
- rbf_batch_vouch → rbfv_batch_vouch

### Private
- zrbf_graft_metadata_submit → zrbfv_graft_metadata_submit
- zrbf_about_submit → zrbfv_about_submit
- zrbf_vouch_submit → zrbfv_vouch_submit

## Transition wiring
- rbfv_FoundryVerify.sh sources rbfc_FoundryCore.sh
- Monolith sources rbfv (for ordain's cross-module calls)
- Create rbfv_cli.sh (sources rbfc + rbfv)
- Update rbz_zipper.sh: batch_vouch enrollment → rbfv_cli.sh with rbfv_ name
- Remove verify functions from monolith
- Update RBS0 spec references

## Acceptance
- rbfv module exists with verification functions
- All tests pass

**[260331-1420] rough**

## Character
Surgical extraction — verification functions form a coherent attestation group.

## Context
Extract rbfv (Foundry Verify) from Foundry: the about, vouch, and batch_vouch operations. These are attestation/metadata operations that run after build operations create images. They use GCB submission (from rbfc) and require director credentials.

## Acceptance
- rbfv module exists with verification functions
- All tests pass

### extract-foundry-ledger (₢A0AAF) [complete]

**[260401-0540] complete**

## Character
Moderate extraction — 4 public + 1 private function. Registry management domain. Mechanical application of the pattern established by ₢A0AAD/₢A0AAE.

## Context
Create rbfl_FoundryLedger.sh as BCG-compliant module. The ledger domain: inscribe (reliquary), jettison, abjure, tally plus inscribe_submit helper. These manage the registry inventory.

## BCG Compliance
- ZRBFL_SOURCED guard, ZRBFL_KINDLED=1 last (readonly)
- zrbfl_kindle(): first line `test -z "${ZRBFL_KINDLED:-}"`, calls `zrbfc_sentinel` (NOT `zrbfc_kindle` — child modules validate core is kindled, they don't re-kindle it), checks Director RBRA, sets up delete/inscribe/reliquary temp prefixes with `ZRBFL_*` names, step dir (rbgji). All kindle constants `readonly`.
- zrbfl_sentinel(): chains `zrbfc_sentinel` then checks `ZRBFL_KINDLED`
- Every public and private function: first line is `zrbfl_sentinel`
- No cross-namespace variable references (`ZRBF_*` variables must not appear — use `ZRBFC_*` for shared core state, `ZRBFL_*` for module-owned state)
- Temp file prefixes use `rbfl_` discriminator to avoid collisions with sibling modules

## Functions to extract and rename

### Public
- rbf_inscribe → rbfl_inscribe
- rbf_jettison → rbfl_jettison
- rbf_abjure → rbfl_abjure
- rbf_tally → rbfl_tally

### Private
- zrbf_inscribe_submit → zrbfl_inscribe_submit

## Kindle dependency audit
Identify all `ZRBF_*` kindle constants referenced by the extracted functions. Each must be either:
- Already in rbfc (`ZRBFC_*`) — use directly
- Module-specific — define as `ZRBFL_*` in zrbfl_kindle
- Dead after extraction — remove from monolith kindle

## Transition wiring
- rbfl_FoundryLedger.sh sources rbfc_FoundryCore.sh
- Create rbfl_cli.sh: furnish kindles `zrbfc_kindle` then `zrbfl_kindle` (CLI owns the kindle graph)
- Update rbz_zipper.sh: inscribe/jettison/abjure/tally enrollments → rbfl_cli.sh with rbfl_ names
- Remove ledger functions from monolith
- Update RBS0 spec references
- Clean dead kindle constants from monolith (any `ZRBF_*` no longer referenced)

## Acceptance
- rbfl module exists with ledger functions
- BCG compliance: SOURCED guard, KINDLED last+readonly, sentinel on every function, no `ZRBF_*` references in new module, all kindle constants readonly
- rbfl_cli.sh works standalone (no monolith dependency)
- All tests pass

**[260401-0514] rough**

## Character
Moderate extraction — 4 public + 1 private function. Registry management domain. Mechanical application of the pattern established by ₢A0AAD/₢A0AAE.

## Context
Create rbfl_FoundryLedger.sh as BCG-compliant module. The ledger domain: inscribe (reliquary), jettison, abjure, tally plus inscribe_submit helper. These manage the registry inventory.

## BCG Compliance
- ZRBFL_SOURCED guard, ZRBFL_KINDLED=1 last (readonly)
- zrbfl_kindle(): first line `test -z "${ZRBFL_KINDLED:-}"`, calls `zrbfc_sentinel` (NOT `zrbfc_kindle` — child modules validate core is kindled, they don't re-kindle it), checks Director RBRA, sets up delete/inscribe/reliquary temp prefixes with `ZRBFL_*` names, step dir (rbgji). All kindle constants `readonly`.
- zrbfl_sentinel(): chains `zrbfc_sentinel` then checks `ZRBFL_KINDLED`
- Every public and private function: first line is `zrbfl_sentinel`
- No cross-namespace variable references (`ZRBF_*` variables must not appear — use `ZRBFC_*` for shared core state, `ZRBFL_*` for module-owned state)
- Temp file prefixes use `rbfl_` discriminator to avoid collisions with sibling modules

## Functions to extract and rename

### Public
- rbf_inscribe → rbfl_inscribe
- rbf_jettison → rbfl_jettison
- rbf_abjure → rbfl_abjure
- rbf_tally → rbfl_tally

### Private
- zrbf_inscribe_submit → zrbfl_inscribe_submit

## Kindle dependency audit
Identify all `ZRBF_*` kindle constants referenced by the extracted functions. Each must be either:
- Already in rbfc (`ZRBFC_*`) — use directly
- Module-specific — define as `ZRBFL_*` in zrbfl_kindle
- Dead after extraction — remove from monolith kindle

## Transition wiring
- rbfl_FoundryLedger.sh sources rbfc_FoundryCore.sh
- Create rbfl_cli.sh: furnish kindles `zrbfc_kindle` then `zrbfl_kindle` (CLI owns the kindle graph)
- Update rbz_zipper.sh: inscribe/jettison/abjure/tally enrollments → rbfl_cli.sh with rbfl_ names
- Remove ledger functions from monolith
- Update RBS0 spec references
- Clean dead kindle constants from monolith (any `ZRBF_*` no longer referenced)

## Acceptance
- rbfl module exists with ledger functions
- BCG compliance: SOURCED guard, KINDLED last+readonly, sentinel on every function, no `ZRBF_*` references in new module, all kindle constants readonly
- rbfl_cli.sh works standalone (no monolith dependency)
- All tests pass

**[260331-1850] rough**

## Character
Moderate extraction — 4 public + 1 private function. Registry management domain.

## Context
Create rbfl_FoundryLedger.sh as BCG-compliant module. The ledger domain: inscribe (reliquary), jettison, abjure, tally plus inscribe_submit helper. These manage the registry inventory.

## BCG Compliance
- ZRBFL_SOURCED guard, ZRBFL_KINDLED=1 last
- zrbfl_kindle(): calls zrbfc_kindle(), checks Director RBRA, sets up delete/inscribe/reliquary temp prefixes, step dir (rbgji), accept manifest media types
- zrbfl_sentinel() guard

## Functions to extract and rename

### Public
- rbf_inscribe → rbfl_inscribe
- rbf_jettison → rbfl_jettison
- rbf_abjure → rbfl_abjure
- rbf_tally → rbfl_tally

### Private
- zrbf_inscribe_submit → zrbfl_inscribe_submit

## Transition wiring
- rbfl_FoundryLedger.sh sources rbfc_FoundryCore.sh
- Create rbfl_cli.sh (sources rbfc + rbfl)
- Update rbz_zipper.sh: inscribe/jettison/abjure/tally enrollments → rbfl_cli.sh with rbfl_ names
- Remove ledger functions from monolith
- Update RBS0 spec references

## Acceptance
- rbfl module exists with ledger functions
- All tests pass

**[260331-1420] rough**

## Character
Surgical extraction — registry management functions.

## Context
Extract rbfl (Foundry Ledger) from Foundry: the inscribe, tally, abjure, and delete operations. These manage the registry inventory — listing, creating, and destroying consecration artifacts. They use GCB submission (some) and GAR API calls (others), and require director credentials.

## Acceptance
- rbfl module exists with ledger functions
- All tests pass

### extract-foundry-director-build (₢A0AAG) [complete]

**[260407-2222] complete**

## Character
Final extraction — residual monolith becomes rbfd. The monolith dies. Bulk rename with careful cross-module wiring.

## Context
After core, retriever, verify, and ledger are extracted, what remains in the monolith is the build/creation domain: ordain (master dispatch), build (conjure), enshrine, kludge, mirror, graft plus their module-specific helpers (quota_preflight, registry_preflight, stitch_build_json, push_build_context, enshrine_submit, enshrine_extract_anchors, mirror_submit).

Rename the residual file to rbfd_FoundryDirectorBuild.sh. Rename all remaining functions from rbf_/zrbf_ to rbfd_/zrbfd_. Create rbfd_cli.sh.

## BCG Compliance
- ZRBFD_SOURCED guard, ZRBFD_KINDLED=1 last (readonly)
- zrbfd_kindle(): first line `test -z "${ZRBFD_KINDLED:-}"`, calls `zrbfc_sentinel` (NOT `zrbfc_kindle`), checks Director RBRA, sets up build/stitch/enshrine/mirror/graft/context/preflight temp prefixes with `ZRBFD_*` names, step dirs (rbgjb, rbgjm, rbgje). All kindle constants `readonly`.
- zrbfd_sentinel(): chains `zrbfc_sentinel` then checks `ZRBFD_KINDLED`
- Every public and private function: first line is `zrbfd_sentinel`
- No cross-namespace variable references — all `ZRBF_*` must be renamed to `ZRBFD_*`
- Temp file prefixes use `rbfd_` discriminator
- rbfd sources rbfv at module level (ordain cross-module calls). `zrbfd_kindle` calls `zrbfv_kindle` (rbfv uses `zrbfc_sentinel` internally, so no double-kindle).

## Functions to rename

### Public (rbf_ → rbfd_)
- rbf_ordain → rbfd_ordain
- rbf_build → rbfd_build
- rbf_enshrine → rbfd_enshrine
- rbf_kludge → rbfd_kludge
- rbf_mirror → rbfd_mirror
- rbf_graft → rbfd_graft

### Private (zrbf_ → zrbfd_)
- zrbf_kindle → zrbfd_kindle
- zrbf_sentinel → zrbfd_sentinel
- zrbf_quota_preflight → zrbfd_quota_preflight
- zrbf_registry_preflight → zrbfd_registry_preflight
- zrbf_stitch_build_json → zrbfd_stitch_build_json
- zrbf_push_build_context → zrbfd_push_build_context
- zrbf_enshrine_submit → zrbfd_enshrine_submit
- zrbf_enshrine_extract_anchors → zrbfd_enshrine_extract_anchors
- zrbf_mirror_submit → zrbfd_mirror_submit

### Kindle constants (ZRBF_* → ZRBFD_*)
All remaining `ZRBF_*` kindle constants become `ZRBFD_*`. All internal references updated.

## Cross-module calls
rbfd_ordain calls `rbfv_vouch` and `zrbfv_graft_metadata_submit`. rbfd sources rbfv at module level; `zrbfd_kindle` kindles rbfv after core sentinel. rbfd_cli.sh furnish: `zrbfc_kindle` → `zrbfv_kindle` (via zrbfd_kindle internally) → done.

## Transition wiring
- Rename rbf_Foundry.sh → rbfd_FoundryDirectorBuild.sh
- Rename ZRBF_SOURCED → ZRBFD_SOURCED, ZRBF_KINDLED → ZRBFD_KINDLED
- Remove sourcing of extracted modules (rbfc source stays — rbfd sources rbfc like all children; rbfv source stays for cross-module calls)
- Create rbfd_cli.sh (furnish sources rbfc+rbfv via rbfd, kindles zrbfc then zrbfd which kindles zrbfv)
- Update rbz_zipper.sh: remaining enrollments → rbfd_cli.sh with rbfd_ names
- Delete rbf_cli.sh (replaced by 5 per-module CLIs)
- Update RBS0 spec references
- Final verification: `grep -r 'rbf_\|zrbf_' Tools/rbk/*.sh` returns zero hits (excluding comments noting delegation)

## Acceptance
- rbfd module exists (formerly rbf_Foundry.sh)
- rbf_Foundry.sh and rbf_cli.sh no longer exist
- BCG compliance: SOURCED guard, KINDLED last+readonly, sentinel on every function, zero `ZRBF_*` variable definitions or references, all kindle constants readonly
- All child modules (rbfc, rbfd, rbfr, rbfv, rbfl) are independently sourceable via their CLIs
- All 5 per-module CLIs dispatch correctly
- All tests pass
- No monolith remains

**[260401-0514] rough**

## Character
Final extraction — residual monolith becomes rbfd. The monolith dies. Bulk rename with careful cross-module wiring.

## Context
After core, retriever, verify, and ledger are extracted, what remains in the monolith is the build/creation domain: ordain (master dispatch), build (conjure), enshrine, kludge, mirror, graft plus their module-specific helpers (quota_preflight, registry_preflight, stitch_build_json, push_build_context, enshrine_submit, enshrine_extract_anchors, mirror_submit).

Rename the residual file to rbfd_FoundryDirectorBuild.sh. Rename all remaining functions from rbf_/zrbf_ to rbfd_/zrbfd_. Create rbfd_cli.sh.

## BCG Compliance
- ZRBFD_SOURCED guard, ZRBFD_KINDLED=1 last (readonly)
- zrbfd_kindle(): first line `test -z "${ZRBFD_KINDLED:-}"`, calls `zrbfc_sentinel` (NOT `zrbfc_kindle`), checks Director RBRA, sets up build/stitch/enshrine/mirror/graft/context/preflight temp prefixes with `ZRBFD_*` names, step dirs (rbgjb, rbgjm, rbgje). All kindle constants `readonly`.
- zrbfd_sentinel(): chains `zrbfc_sentinel` then checks `ZRBFD_KINDLED`
- Every public and private function: first line is `zrbfd_sentinel`
- No cross-namespace variable references — all `ZRBF_*` must be renamed to `ZRBFD_*`
- Temp file prefixes use `rbfd_` discriminator
- rbfd sources rbfv at module level (ordain cross-module calls). `zrbfd_kindle` calls `zrbfv_kindle` (rbfv uses `zrbfc_sentinel` internally, so no double-kindle).

## Functions to rename

### Public (rbf_ → rbfd_)
- rbf_ordain → rbfd_ordain
- rbf_build → rbfd_build
- rbf_enshrine → rbfd_enshrine
- rbf_kludge → rbfd_kludge
- rbf_mirror → rbfd_mirror
- rbf_graft → rbfd_graft

### Private (zrbf_ → zrbfd_)
- zrbf_kindle → zrbfd_kindle
- zrbf_sentinel → zrbfd_sentinel
- zrbf_quota_preflight → zrbfd_quota_preflight
- zrbf_registry_preflight → zrbfd_registry_preflight
- zrbf_stitch_build_json → zrbfd_stitch_build_json
- zrbf_push_build_context → zrbfd_push_build_context
- zrbf_enshrine_submit → zrbfd_enshrine_submit
- zrbf_enshrine_extract_anchors → zrbfd_enshrine_extract_anchors
- zrbf_mirror_submit → zrbfd_mirror_submit

### Kindle constants (ZRBF_* → ZRBFD_*)
All remaining `ZRBF_*` kindle constants become `ZRBFD_*`. All internal references updated.

## Cross-module calls
rbfd_ordain calls `rbfv_vouch` and `zrbfv_graft_metadata_submit`. rbfd sources rbfv at module level; `zrbfd_kindle` kindles rbfv after core sentinel. rbfd_cli.sh furnish: `zrbfc_kindle` → `zrbfv_kindle` (via zrbfd_kindle internally) → done.

## Transition wiring
- Rename rbf_Foundry.sh → rbfd_FoundryDirectorBuild.sh
- Rename ZRBF_SOURCED → ZRBFD_SOURCED, ZRBF_KINDLED → ZRBFD_KINDLED
- Remove sourcing of extracted modules (rbfc source stays — rbfd sources rbfc like all children; rbfv source stays for cross-module calls)
- Create rbfd_cli.sh (furnish sources rbfc+rbfv via rbfd, kindles zrbfc then zrbfd which kindles zrbfv)
- Update rbz_zipper.sh: remaining enrollments → rbfd_cli.sh with rbfd_ names
- Delete rbf_cli.sh (replaced by 5 per-module CLIs)
- Update RBS0 spec references
- Final verification: `grep -r 'rbf_\|zrbf_' Tools/rbk/*.sh` returns zero hits (excluding comments noting delegation)

## Acceptance
- rbfd module exists (formerly rbf_Foundry.sh)
- rbf_Foundry.sh and rbf_cli.sh no longer exist
- BCG compliance: SOURCED guard, KINDLED last+readonly, sentinel on every function, zero `ZRBF_*` variable definitions or references, all kindle constants readonly
- All child modules (rbfc, rbfd, rbfr, rbfv, rbfl) are independently sourceable via their CLIs
- All 5 per-module CLIs dispatch correctly
- All tests pass
- No monolith remains

**[260331-1850] rough**

## Character
Final extraction — residual monolith becomes rbfd. The monolith dies.

## Context
After core, retriever, verify, and ledger are extracted, what remains in the monolith is the build/creation domain: ordain (master dispatch), build (conjure), enshrine, kludge, mirror, graft plus their module-specific helpers (quota_preflight, registry_preflight, stitch_build_json, push_build_context, enshrine_submit, enshrine_extract_anchors, mirror_submit).

Rename the residual file to rbfd_FoundryDirectorBuild.sh. Rename all remaining functions from rbf_/zrbf_ to rbfd_/zrbfd_. Create rbfd_cli.sh (sources rbfc + rbfd + rbfv — rbfv needed because rbfd_ordain chains build → vouch).

## BCG Compliance
- ZRBFD_SOURCED guard, ZRBFD_KINDLED=1 last
- zrbfd_kindle(): calls zrbfc_kindle(), checks Director RBRA, sets up build/stitch/enshrine/mirror/graft/context/preflight temp prefixes, step dirs (rbgjb, rbgjm, rbgje)
- zrbfd_sentinel() guard

## Functions to rename

### Public (rbf_ → rbfd_)
- rbf_ordain → rbfd_ordain
- rbf_build → rbfd_build
- rbf_enshrine → rbfd_enshrine
- rbf_kludge → rbfd_kludge
- rbf_mirror → rbfd_mirror
- rbf_graft → rbfd_graft

### Private (zrbf_ → zrbfd_)
- zrbf_quota_preflight → zrbfd_quota_preflight
- zrbf_registry_preflight → zrbfd_registry_preflight
- zrbf_stitch_build_json → zrbfd_stitch_build_json
- zrbf_push_build_context → zrbfd_push_build_context
- zrbf_enshrine_submit → zrbfd_enshrine_submit
- zrbf_enshrine_extract_anchors → zrbfd_enshrine_extract_anchors
- zrbf_mirror_submit → zrbfd_mirror_submit

## Cross-module call
rbfd_ordain calls rbfv_vouch and zrbfv_graft_metadata_submit. This works because rbfd_cli.sh furnish sources and kindles both rbfd and rbfv.

## Transition wiring
- Rename rbf_Foundry.sh → rbfd_FoundryDirectorBuild.sh
- Remove all sourcing of extracted modules (rbfc, rbfr, rbfv, rbfl) from the implementation file — those are now sourced by CLIs
- Create rbfd_cli.sh (sources rbfc + rbfd + rbfv)
- Update rbz_zipper.sh: remaining enrollments → rbfd_cli.sh with rbfd_ names
- Delete rbf_cli.sh (replaced by 5 per-module CLIs)
- Update RBS0 spec references
- Final verification: no rbf_ or zrbf_ function definitions remain anywhere

## Acceptance
- rbfd module exists (formerly the residual of rbf_Foundry.sh)
- rbf_Foundry.sh and rbf_cli.sh no longer exist
- All child modules (rbfc, rbfd, rbfr, rbfv, rbfl) are independently sourceable
- All 5 per-module CLIs dispatch correctly
- All tests pass
- No monolith remains

**[260331-1421] rough**

## Character
Final extraction — what remains in rbf_Foundry.sh becomes rbfd.

## Context
After core, retriever, verify, and ledger are extracted, what remains is the build/creation domain: ordain (master dispatch), conjure (GCB cloud build), enshrine (upstream copy), graft (local push + GCB metadata), kludge (local dev build), mirror (bind image copy). Rename the residual file to rbfd.

## Acceptance
- rbfd module exists (formerly the residual of rbf_Foundry.sh)
- rbf_Foundry.sh no longer exists
- All child modules (rbfc, rbfd, rbfr, rbfv, rbfl) are independently sourceable
- All tests pass
- No monolith remains

### post-decomposition-test-sweep (₢A0AAH) [complete]

**[260401-0830] complete**

## Character
Mechanical but careful — run each test fixture, fix any breakage from the rbf→rbfd/rbfl/rbfv/rbfr/rbfc decomposition. The test fixtures exercise the zipper dispatch path so they validate the full wiring.

## Scope
- Fix stale references in test fixtures (rbf_ → rbfd_, zrbf_ → zrbfd_, rbf_Foundry.sh → rbfd_FoundryDirectorBuild.sh)
- Fix stale comments in rbtctm_ThreeMode.sh (references rbf_ordain in header comment)
- Run test fixtures sequentially: three-mode first, then regime-validation, enrollment-validation, etc.
- Any failures are decomposition regressions — fix and re-run

## Acceptance
- All test fixtures pass (or known-unrelated failures documented)
- Zero stale rbf_/zrbf_ references in test fixture files

**[260401-0603] rough**

## Character
Mechanical but careful — run each test fixture, fix any breakage from the rbf→rbfd/rbfl/rbfv/rbfr/rbfc decomposition. The test fixtures exercise the zipper dispatch path so they validate the full wiring.

## Scope
- Fix stale references in test fixtures (rbf_ → rbfd_, zrbf_ → zrbfd_, rbf_Foundry.sh → rbfd_FoundryDirectorBuild.sh)
- Fix stale comments in rbtctm_ThreeMode.sh (references rbf_ordain in header comment)
- Run test fixtures sequentially: three-mode first, then regime-validation, enrollment-validation, etc.
- Any failures are decomposition regressions — fix and re-run

## Acceptance
- All test fixtures pass (or known-unrelated failures documented)
- Zero stale rbf_/zrbf_ references in test fixture files

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 A purge-gitlab-and-rubric-inscribe
  2 B foundry-decomposition-plan
  3 C extract-foundry-core
  4 D extract-foundry-retriever
  5 E extract-foundry-verify
  6 F extract-foundry-ledger
  7 G extract-foundry-director-build
  8 H post-decomposition-test-sweep

ABCDEFGH
x·xxxxx· rbf_Foundry.sh, rbz_zipper.sh
x··xxxx· RBS0-SpecTop.adoc
···xx··x rbfr_cli.sh
··x·x··x rbfc_FoundryCore.sh
······xx rbfd_FoundryDirectorBuild.sh, rbfd_cli.sh, rbtb_testbench.sh
·····x·x rbfl_FoundryLedger.sh, rbfl_cli.sh
·····xx· RBSCB-CloudBuildPosture.adoc
····x··x rbfv_cli.sh, rbtctm_ThreeMode.sh
···xx··· rbfr_FoundryRetriever.sh
··x····x rbfc_cli.sh
·······x CLAUDE.md, rbtcap_AccessProbe.sh, rbtcfm_FourMode.sh
······x· rbf_cli.sh, rbob_cli.sh
·····x·· rbcc_Constants.sh
····x··· rbfv_FoundryVerify.sh
x······· RBSRI-rubric_inscribe.adoc, README.consumer.md, rbgc_Constants.sh, rbgg_Governor.sh, rbgm_ManualProcedures.sh, rbgm_cli.sh, rblm_cli.sh, rbw-gPL.GitLabSetup.sh

Commit swim lanes (x = commit affiliated with pace):

  1 A purge-gitlab-and-rubric-inscribe
  2 B foundry-decomposition-plan
  3 C extract-foundry-core
  4 D extract-foundry-retriever
  5 E extract-foundry-verify
  6 F extract-foundry-ledger
  7 G extract-foundry-director-build
  8 H post-decomposition-test-sweep

123456789abcdefghijklmnopqrstuvwxyz
··········xx·······················  A  2c
············x······················  B  1c
·············xx····················  C  2c
···············xx··················  D  2c
·················xxxx··············  E  4c
·····················xx············  F  2c
·······················x·········xx  G  3c
·························xxxxxxxx··  H  8c
```

## Steeplechase

### 2026-04-07 22:22 - ₢A0AAG - W

Completed foundry monolith extraction: rbf_Foundry.sh renamed to rbfd_FoundryDirectorBuild.sh, all rbf_/zrbf_/ZRBF_ prefixes renamed to rbfd_/zrbfd_/ZRBFD_, rbfd_cli.sh created, rbf_cli.sh deleted, zipper rerouted. Fixed latent BCG sentinel nit in rbfd_kludge. All 75 fast-suite tests pass, zero stale monolith references remain.

### 2026-04-07 22:21 - ₢A0AAG - n

Fix BCG sentinel compliance: rbfd_kludge now calls zrbfd_sentinel instead of zrbfc_sentinel

### 2026-04-01 08:30 - ₢A0AAH - W

Post-decomposition test sweep: fixed rbfc multiple-inclusion guard (buc_die→return 0), added +x to 7 new CLI/module files, fixed missing zipper sourcing in rbfl_cli.sh, fixed graft vessel crash in rbfl_abjure (pre-existing bug). Renamed three-mode test to four-mode with kludge step (15-step sequence). Added Test Execution section to CLAUDE.md documenting suite tiers. All 93 test cases pass across 9 fixtures.

### 2026-04-01 08:29 - ₢A0AAH - n

Add Test Execution section to CLAUDE.md documenting test suite tiers (fast/service/crucible/complete), when to use each, and sequential-only discipline

### 2026-04-01 07:46 - ₢A0AAH - n

Fix rbfl_abjure graft vessel crash: graft mode fell through to conjure branch which reads RBRV_CONJURE_PLATFORMS (unset for graft). Graft uses single image tag like bind.

### 2026-04-01 07:41 - ₢A0AAH - n

Add zipper sourcing and kindling to rbfl_cli.sh — tally uses RBZ_VOUCH_CONSECRATIONS and RBZ_ABJURE_CONSECRATION for tabtarget recommendations

### 2026-04-01 06:57 - ₢A0AAH - n

Rename three-mode test to four-mode, add kludge step (local docker build + vouch tag verify + run + cleanup). 15-step sequence now exercises all four delivery modes: conjure, bind, graft, kludge.

### 2026-04-01 06:23 - ₢A0AAH - n

Add executable permissions to rbfc_cli.sh, rbfr_cli.sh, rbfv_cli.sh — retroactive fix for earlier extraction paces that created CLIs without +x

### 2026-04-01 06:07 - ₢A0AAH - n

Add executable permissions to new CLI and module files (rbfd_cli.sh, rbfl_cli.sh, rbfl_FoundryLedger.sh, rbfd_FoundryDirectorBuild.sh)

### 2026-04-01 06:04 - ₢A0AAH - n

Fix rbfc multiple-inclusion guard (buc_die → return 0) for composition safety when multiple child modules source rbfc. Fix stale rbf_ordain comment in ThreeMode test.

### 2026-04-01 06:03 - Heat - S

post-decomposition-test-sweep

### 2026-04-01 05:58 - ₢A0AAG - n

Rename rbf_Foundry.sh to rbfd_FoundryDirectorBuild.sh, rename all rbf_/zrbf_/ZRBF_ prefixes to rbfd_/zrbfd_/ZRBFD_, create rbfd_cli.sh, delete rbf_cli.sh, reroute zipper enrollments, update source references in rbob_cli.sh and rbtb_testbench.sh, update spec references. Zero bare rbf_/zrbf_/ZRBF_ references remain in production .sh files.

### 2026-04-01 05:40 - ₢A0AAF - W

Extracted rbf_inscribe, rbf_jettison, rbf_abjure, rbf_tally and zrbf_inscribe_submit into BCG-compliant rbfl_FoundryLedger.sh module with rbfl_cli.sh. Rerouted zipper enrollments, removed 5 functions and 7 dead kindle constants from monolith, updated spec references. All kindle constants use ZRBFL_ prefix with rbfl_ temp file discriminators. No ZRBF_ namespace contamination.

### 2026-04-01 05:37 - ₢A0AAF - n

Extract rbf_inscribe, rbf_jettison, rbf_abjure, rbf_tally and zrbf_inscribe_submit into BCG-compliant rbfl_FoundryLedger.sh module with rbfl_cli.sh. Reroute zipper enrollments, remove functions and dead kindle constants from monolith, update spec references.

### 2026-04-01 05:12 - ₢A0AAE - W

Extracted 4 public + 3 private verify functions into BCG-compliant rbfv_FoundryVerify.sh with rbfv_cli.sh. Fixed step dir ownership (ZRBGJA/RBGJV moved from monolith to rbfc as ZRBFC_ kindle constants), eliminated conditional kindle shim, added missing sentinels to rbfv_vouch_gate and 5 rbfc plumb helpers. Wired monolith cross-module calls for ordain transition, updated zipper, test call sites, and spec references.

### 2026-04-01 05:11 - ₢A0AAE - n

Add missing zrbfc_sentinel to 5 plumb private helpers (plumb_core, plumb_show_bind, plumb_show_sections, plumb_show_compact, plumb_show_full) per BCG internal helper first-line rule — retroactive fix for ₢A0AAC

### 2026-04-01 05:09 - ₢A0AAE - n

Fix step dir ownership: move RBGJA/RBGJV step dir variables from monolith to rbfc kindle (ZRBFC_ prefix), update assembly function references, remove dead ZRBF_VOUCH/ABOUT_PREFIX from monolith, eliminate conditional kindle shim from rbfv. Add sentinel to rbfv_vouch_gate per BCG public function rules.

### 2026-04-01 04:58 - ₢A0AAE - n

Extract rbf_vouch_gate, rbf_about, rbf_vouch, rbf_batch_vouch and private helpers into BCG-compliant rbfv_FoundryVerify.sh module with rbfv_cli.sh. Wire monolith cross-module calls (ordain → rbfv_vouch/zrbfv_graft_metadata_submit), update zipper, test call sites, spec references. Fix child module kindle pattern (sentinel instead of re-kindle) across rbfr and rbfv for composition safety.

### 2026-04-01 04:44 - ₢A0AAD - W

Extracted rbf_wrest and rbf_summon into BCG-compliant rbfr_FoundryRetriever.sh module (ZRBFR_SOURCED guard, zrbfr_kindle/sentinel, ZRBFR_TEMP_PREFIX). Created rbfr_cli.sh with retriever-only credential boundary (no Director RBRA). Rerouted zipper enrollments, removed 222 lines from monolith, updated RBS0 spec references.

### 2026-04-01 04:43 - ₢A0AAD - n

Extract rbf_wrest and rbf_summon into BCG-compliant rbfr_FoundryRetriever.sh module with rbfr_cli.sh (retriever-only credentials, no Director RBRA), reroute zipper enrollments, remove functions from monolith, update spec references

### 2026-04-01 04:38 - ₢A0AAC - W

Extracted 15 shared functions into BCG-compliant rbfc_FoundryCore.sh module (ZRBFC_SOURCED guard, zrbfc_kindle/sentinel, all zrbfc_* prefixed). Created rbfc_cli.sh for credential-free plumb dispatch. Updated monolith to source rbfc and delegate all core calls. Deleted dead zrbf_base64_decode_capture. Rerouted zipper plumb enrollments to rbfc_cli.sh.

### 2026-03-31 19:31 - ₢A0AAC - n

Extract 15 shared functions into BCG-compliant rbfc_FoundryCore.sh module, create rbfc_cli.sh for credential-free plumb dispatch, update monolith kindle/sentinel/call-sites/variable-refs, delete dead zrbf_base64_decode_capture, reroute zipper plumb enrollments to rbfc_cli.sh

### 2026-03-31 18:53 - ₢A0AAB - W

Mapped all 44 Foundry functions to 5 BCG-compliant modules (rbfc/rbfd/rbfv/rbfl/rbfr) with full cross-reference analysis, kindle decomposition, CLI routing, and credential boundary design. Reslated extraction paces C-G with BCG-aware dockets including prefix renames and transition wiring.

### 2026-03-31 18:33 - ₢A0AAA - W

Purged all GitLab dead code: deleted rbf_rubric_inscribe (304 lines), CBv2 constants, rbgm_gitlab_setup stub, zipper enrollment, dispatch entry, tabtarget, and RBSRI spec. Cleaned consumer docs (README, CLAUDE) of GitLab prereqs and rubric repo references. Rewrote shell comments. Zero gitlab hits remain in shell scripts and consumer docs; spec historical comments preserved.

### 2026-03-31 18:33 - ₢A0AAA - n

Remove GitLab rubric infrastructure: delete inscribe function, RBSRI spec, CBv2 constants, gitlab_setup command, and rubric repo references from consumer docs

### 2026-03-31 18:04 - Heat - d

paddock curried: capture foundry decomposition gestalt

### 2026-03-31 14:27 - Heat - f

racing

### 2026-03-31 14:21 - Heat - S

extract-foundry-director-build

### 2026-03-31 14:20 - Heat - S

extract-foundry-ledger

### 2026-03-31 14:20 - Heat - S

extract-foundry-verify

### 2026-03-31 14:20 - Heat - S

extract-foundry-retriever

### 2026-03-31 14:20 - Heat - S

extract-foundry-core

### 2026-03-31 14:19 - Heat - S

foundry-decomposition-plan

### 2026-03-31 14:19 - Heat - S

purge-gitlab-and-rubric-inscribe

### 2026-03-31 14:17 - Heat - N

rbk-mvp-3-foundry-decomposition

