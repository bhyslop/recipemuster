# Heat Trophy: rbk-08-credential-repairs

**Firemark:** ₣BY
**Created:** 260604
**Retired:** 260605
**Status:** retired

## Paddock

# Paddock: rbk-08-credential-repairs

## Context

(Describe the initiative's background and goals)

## References

(List relevant files, docs, or prior work)

## Paces

### lifecycle-split-establish-rekey (₢BYAAA) [complete]

**[260605-0252] complete**

## Character
Surgical, security-sensitive credential refactor — NOT mechanical. Live GCP IAM;
verification is expensive and intermittent. Survey before adding, cite BCG before
writing bash, work one small increment at a time with operator review.

**Read first:** BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) — especially the
load-bearing-complexity example that forbids a generic IAM helper. Grounding:
`Memos/memo-20260604-credential-churn-leak-and-propagation-races.md`. No IAM
member-removal exists in the kit today; the grant pattern (GET -> modify ->
setIamPolicy) lives in rbgi/rbga/rbgg — survey it before adding revoke.

Close two holes. **H1:** divest never revokes IAM bindings -> `deleted:` ghost
tombstones accrue unbounded. **H2:** re-credentialing deletes+recreates the whole
SA when it should rotate the key only.

**Surgical, in place. Do NOT split functions or rewrite the invest verbs**
(`rbgg_invest_director` / `rbgg_invest_retriever`) — their grant sequences stay
byte-for-byte and re-run idempotently on re-invest.

### Bash — rbgg_Governor.sh
- CHANGE `zrbgg_create_service_account_with_key` (keep the name) to idempotent:
  SA-exists preflight skips create instead of `buc_die`; guard create+poll on the
  absent branch; any cross-branch local declared at function scope with explicit init.
- ADD automated key blunt-kill: replace the manual-console-abort on existing
  USER_MANAGED keys with delete-all-then-mint-one (new key-delete infix; reuse rbgg's
  existing jq-name-extraction idiom).
- CHANGE `rbgg_divest_director` / `rbgg_divest_retriever` to revoke their own bindings
  before `zrbgg_divest_role` deletes the SA (director: project + Mason-SA + repo;
  retriever: project).

### Bash — rbgi_IAM.sh
- ADD per-scope member-revoke (project, SA, repo): three self-contained functions
  mirroring `rbgi_add_*_iam_role` one-for-one, NOT a generic helper (BCG load-bearing).
  Share only a jq member-removal helper paralleling `rbgi_jq_add_member_to_role_capture`.

### Rust — rbtd/src/rbtdrk_canonical.rs
- DELETE the pre-divest in `rbtdrk_role_invest_impl` (the divest-then-invest dance was
  H2's test-side scar); drop the now-unused divest_colophon param and fix callers.

### Specs — Tools/rbk/vov_veiled/*.adoc
- RBSRK / RBSDK (invest): fail-loud-on-existing -> idempotent establish + key rotation.
- RBSRD / RBSDD (divest): SA-delete-only -> teardown (revoke, then delete).
- RBSCIG: add the parallel revoke contract.
- RBSCIP: create->grant race relocates to first-establish; delete->recreate race eliminated.

### Settle at mount (cinch with operator)
- Revoke in divest: best-effort-with-loud-log vs fatal. Lean best-effort, so a flaky
  revoke cannot regress the working divest path.

### Out of scope (cinched)
- One-time sweep of existing ghost bindings.
- New operator verbs / colophons (one idempotent invest; tabtarget surface unchanged).
- RBRA one-file-per-person (that is pace ₢BYAAB).

### Done
- Standing-depot re-invest rotates the key without SA delete/recreate (no tombstones);
  divest revokes before delete; bash BCG- + shellcheck-clean (`tt/rbw-tl.Shellcheck.sh`);
  theurge builds + unit tests green; live service-tier validation operator-driven.

**[260604-2019] rough**

## Character
Surgical, security-sensitive credential refactor — NOT mechanical. Live GCP IAM;
verification is expensive and intermittent. Survey before adding, cite BCG before
writing bash, work one small increment at a time with operator review.

**Read first:** BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) — especially the
load-bearing-complexity example that forbids a generic IAM helper. Grounding:
`Memos/memo-20260604-credential-churn-leak-and-propagation-races.md`. No IAM
member-removal exists in the kit today; the grant pattern (GET -> modify ->
setIamPolicy) lives in rbgi/rbga/rbgg — survey it before adding revoke.

Close two holes. **H1:** divest never revokes IAM bindings -> `deleted:` ghost
tombstones accrue unbounded. **H2:** re-credentialing deletes+recreates the whole
SA when it should rotate the key only.

**Surgical, in place. Do NOT split functions or rewrite the invest verbs**
(`rbgg_invest_director` / `rbgg_invest_retriever`) — their grant sequences stay
byte-for-byte and re-run idempotently on re-invest.

### Bash — rbgg_Governor.sh
- CHANGE `zrbgg_create_service_account_with_key` (keep the name) to idempotent:
  SA-exists preflight skips create instead of `buc_die`; guard create+poll on the
  absent branch; any cross-branch local declared at function scope with explicit init.
- ADD automated key blunt-kill: replace the manual-console-abort on existing
  USER_MANAGED keys with delete-all-then-mint-one (new key-delete infix; reuse rbgg's
  existing jq-name-extraction idiom).
- CHANGE `rbgg_divest_director` / `rbgg_divest_retriever` to revoke their own bindings
  before `zrbgg_divest_role` deletes the SA (director: project + Mason-SA + repo;
  retriever: project).

### Bash — rbgi_IAM.sh
- ADD per-scope member-revoke (project, SA, repo): three self-contained functions
  mirroring `rbgi_add_*_iam_role` one-for-one, NOT a generic helper (BCG load-bearing).
  Share only a jq member-removal helper paralleling `rbgi_jq_add_member_to_role_capture`.

### Rust — rbtd/src/rbtdrk_canonical.rs
- DELETE the pre-divest in `rbtdrk_role_invest_impl` (the divest-then-invest dance was
  H2's test-side scar); drop the now-unused divest_colophon param and fix callers.

### Specs — Tools/rbk/vov_veiled/*.adoc
- RBSRK / RBSDK (invest): fail-loud-on-existing -> idempotent establish + key rotation.
- RBSRD / RBSDD (divest): SA-delete-only -> teardown (revoke, then delete).
- RBSCIG: add the parallel revoke contract.
- RBSCIP: create->grant race relocates to first-establish; delete->recreate race eliminated.

### Settle at mount (cinch with operator)
- Revoke in divest: best-effort-with-loud-log vs fatal. Lean best-effort, so a flaky
  revoke cannot regress the working divest path.

### Out of scope (cinched)
- One-time sweep of existing ghost bindings.
- New operator verbs / colophons (one idempotent invest; tabtarget surface unchanged).
- RBRA one-file-per-person (that is pace ₢BYAAB).

### Done
- Standing-depot re-invest rotates the key without SA delete/recreate (no tombstones);
  divest revokes before delete; bash BCG- + shellcheck-clean (`tt/rbw-tl.Shellcheck.sh`);
  theurge builds + unit tests green; live service-tier validation operator-driven.

**[260604-1302] complete**

## Character
Intricate but mechanical refactor; the one judgment is the establish/re-key boundary.

Close the credential-lifecycle hole on director + retriever. The fused invest does
SA-create + Binding-grant + Key-create, and its only inverse (divest) does SA-delete
alone — so Bindings are never revoked (H1) and re-key churns the whole SA (H2).

Split the fused invest into three operations:
- establish (idempotent — ensure SA exists + grant its Bindings once)
- re-key (Key-only — delete all Keys, add one; the blunt-kill guarantee preserved)
- teardown (revoke Bindings, then delete SA — the never-wired inverse; only at genuine retirement)

Source: `Memos/memo-20260604-credential-churn-leak-and-propagation-races.md` — objects,
the three lifecycles, H1/H2, and the 8 scope notes (idempotent establish; re-key presumes
established; keep the DRS-400 in rbgi's retry tolerance at establish; governor out of scope).

Done: re-key touches only Keys; Bindings granted once and revoked only on teardown;
standing-depot reruns stop accruing `deleted:` tombstones.

**[260604-1027] rough**

## Character
Intricate but mechanical refactor; the one judgment is the establish/re-key boundary.

Close the credential-lifecycle hole on director + retriever. The fused invest does
SA-create + Binding-grant + Key-create, and its only inverse (divest) does SA-delete
alone — so Bindings are never revoked (H1) and re-key churns the whole SA (H2).

Split the fused invest into three operations:
- establish (idempotent — ensure SA exists + grant its Bindings once)
- re-key (Key-only — delete all Keys, add one; the blunt-kill guarantee preserved)
- teardown (revoke Bindings, then delete SA — the never-wired inverse; only at genuine retirement)

Source: `Memos/memo-20260604-credential-churn-leak-and-propagation-races.md` — objects,
the three lifecycles, H1/H2, and the 8 scope notes (idempotent establish; re-key presumes
established; keep the DRS-400 in rbgi's retry tolerance at establish; governor out of scope).

Done: re-key touches only Keys; Bindings granted once and revoked only on teardown;
standing-depot reruns stop accruing `deleted:` tombstones.

### consider-per-person-multirole-rbra (₢BYAAB) [complete]

**[260604-1940] complete**

## Character
Design exploration — judgment, not mechanical. Output is a recommendation, not an implementation.

Evaluate refactoring RBRA from one-file-per-role-type (director, retriever — each with its
own cult verbs) to one-file-per-person carrying any combination of roles. Collapses the
role-typed verb surface and aligns the keyfile tier with the federation model's
principal -> capability-set shape (smoother eventual migration).

The lifecycle split is its enabler: once identity (establish) and credential (re-key) are
separated, "a principal with a role-set" is the natural next shape.

Key question to resolve: does per-person multi-role preserve least-privilege? Today's
role-typed SAs enforce it by construction — a retriever cannot act as director. A per-person
SA is only as least-privilege as the grants chosen; show the security guarantee survives,
not just the convenience.

Source: the federation footnote in `Memos/memo-20260604-credential-churn-leak-and-propagation-races.md`;
`Memos/memo-20260527-operator-credential-models.md`.

Done: a decision/recommendation (do / don't / how), not implementation.

**[260604-1028] rough**

## Character
Design exploration — judgment, not mechanical. Output is a recommendation, not an implementation.

Evaluate refactoring RBRA from one-file-per-role-type (director, retriever — each with its
own cult verbs) to one-file-per-person carrying any combination of roles. Collapses the
role-typed verb surface and aligns the keyfile tier with the federation model's
principal -> capability-set shape (smoother eventual migration).

The lifecycle split is its enabler: once identity (establish) and credential (re-key) are
separated, "a principal with a role-set" is the natural next shape.

Key question to resolve: does per-person multi-role preserve least-privilege? Today's
role-typed SAs enforce it by construction — a retriever cannot act as director. A per-person
SA is only as least-privilege as the grants chosen; show the security guarantee survives,
not just the convenience.

Source: the federation footnote in `Memos/memo-20260604-credential-churn-leak-and-propagation-races.md`;
`Memos/memo-20260527-operator-credential-models.md`.

Done: a decision/recommendation (do / don't / how), not implementation.

### divest-readflap-poll-debounce (₢BYAAC) [complete]

**[260604-1250] complete**

## Character
Done work, slated for immediate wrap (heat-affiliated commits already landed).

Harden rbuh_poll_until_gone against GCP IAM's post-delete read-flap (404→200)
so canonical-invest's divest-before-invest is durably idempotent, and repair the
RBSRD/RBSDD/RBSCIP spec drift the change implies. The dogfight intermittent
ordain failure observed during verification is documented as a notched memo
under this pace.

**[260604-1214] rough**

## Character
Done work, slated for immediate wrap (heat-affiliated commits already landed).

Harden rbuh_poll_until_gone against GCP IAM's post-delete read-flap (404→200)
so canonical-invest's divest-before-invest is durably idempotent, and repair the
RBSRD/RBSDD/RBSCIP spec drift the change implies. The dogfight intermittent
ordain failure observed during verification is documented as a notched memo
under this pace.

### tools-magic-string-sweep (₢BYAAD) [complete]

**[260605-1429] complete**

## Character
Mechanical sweep with per-locale judgment. The grep is the worklist; the rule
table is the decision procedure. Several locales resolve to a recorded "no change."

## Goal
Eliminate `Tools/` magic-string path literals across executable code, replacing
each with the established mechanism for its locale (BCG dispatch vars / tinder
consts for bash; RCG path consts for Rust). Resolve every grep hit — each earns
a verdict, including no-change for sanctioned forms.

## Discovery signature
```
rg -n 'Tools/' -g '!target' -g '!*.d'
```
Key property: every compliant form expands a variable (`${BURD_TOOLS_DIR}/...`,
`${BURD_BUK_DIR}/...`, `${BURC_TOOLS_DIR}/...`) and contains NO literal `Tools/`
substring (case-sensitive grep; `TOOLS_DIR` is uppercase-underscore). So the
hit-set IS the worklist: each hit is a candidate violation, a sanctioned
bootstrap, or prose. Triage by locale.

## Scope (subdirectories)
`Tools/` (all kits), `rbmm_moorings/` (launchers, compose, Dockerfile), `Study/`,
`tt/`. Docs are out — see bottom.

## Per-locale rule

| Locale | Rule |
|---|---|
| Dispatched bash CLI / workbench / module | Replace literal with dispatch var: `${BURD_BUK_DIR}/...` (BUK files) or `${BURD_TOOLS_DIR}/<kit>/...`. Config path driving a sourcing chain the module owns → tinder const `«PREFIX»_lower_name` instead. Never reconstruct from BASH_SOURCE. (BCG §Dispatch-Provided Directory Variables) |
| Pre-dispatch launcher body (after BURC sourced) | Use `${BURC_TOOLS_DIR}/...` — the `bul_launch` lines already model this. |
| Bootstrap membrane — tabtarget `exec` line; launcher's `source "Tools/buk/bul_launcher.sh"` | No change. Runs before any var exists; cwd=repo-root is trampoline-guaranteed; the single sanctioned literal at the membrane. Launchers are generated by `buut_tabtarget` — fix the generator, never the outputs. |
| Standalone bash not run through dispatch (e.g. uninstaller) | BURD/BURC absent. Resolve own root once via `${BASH_SOURCE[0]%/*}` into a local and derive the rest, or document a cwd contract. Per-script judgment. |
| Bash installer/arcanum acting on a target repo (`$target/Tools/...`) | No change where `Tools/...` is the destination contract in the consumer repo — not a self-reference. Verify each: own-location → candidate; target destination → intrinsic. |
| Rust running-code path | Hoist relative to a `pub const {PREFIX}_SCREAMING: &str` (RCG Identity Rule; exemplar already correct: `VVCC_REGISTRY_PATH`), derive every `root.join(...)` from it. No env-var path in Rust — `root` arrives as a parameter. Const placement/name is mount-time; reuse an existing const home if one fits. |
| Rust kit-root literals (`Tools/rbk`, `Tools/buk`) | Same const treatment for completeness; lower risk. |
| Rust comment / docstring | Prose — fix only if the named path is stale. |
| Rust test fixture / expected-output assertion | The spec of emitted output. Reference the shared const the producer uses, or leave as the expected literal. Lowest priority. |
| Config without dispatch context — `*compose.yml` bind mounts, `Cargo.toml` path deps, `Dockerfile` | No variable mechanism. Cargo path deps stay static (no change). compose bind-mounts are relative to the compose file = deployment contract (leave unless an orchestrator var is already in play). Dockerfile `Tools/...` is a comment (fix-if-stale). |

## Out of scope — docs
`*.md`, `*.adoc`, `Memos/`, root `CLAUDE.md`. The acronym maps and spec cross-refs
intentionally register `ACRONYM → Tools/...`; keeping them current is the rename
workflow's job, not this sweep. Flag separately if a rename already left a map stale.

## Done
Every `Tools/` grep hit in scope carries a verdict; candidate violations are
converted to the locale-appropriate mechanism; sanctioned/intrinsic hits are left
with their rationale; lint and the relevant test tier pass.

**[260604-1414] rough**

## Character
Mechanical sweep with per-locale judgment. The grep is the worklist; the rule
table is the decision procedure. Several locales resolve to a recorded "no change."

## Goal
Eliminate `Tools/` magic-string path literals across executable code, replacing
each with the established mechanism for its locale (BCG dispatch vars / tinder
consts for bash; RCG path consts for Rust). Resolve every grep hit — each earns
a verdict, including no-change for sanctioned forms.

## Discovery signature
```
rg -n 'Tools/' -g '!target' -g '!*.d'
```
Key property: every compliant form expands a variable (`${BURD_TOOLS_DIR}/...`,
`${BURD_BUK_DIR}/...`, `${BURC_TOOLS_DIR}/...`) and contains NO literal `Tools/`
substring (case-sensitive grep; `TOOLS_DIR` is uppercase-underscore). So the
hit-set IS the worklist: each hit is a candidate violation, a sanctioned
bootstrap, or prose. Triage by locale.

## Scope (subdirectories)
`Tools/` (all kits), `rbmm_moorings/` (launchers, compose, Dockerfile), `Study/`,
`tt/`. Docs are out — see bottom.

## Per-locale rule

| Locale | Rule |
|---|---|
| Dispatched bash CLI / workbench / module | Replace literal with dispatch var: `${BURD_BUK_DIR}/...` (BUK files) or `${BURD_TOOLS_DIR}/<kit>/...`. Config path driving a sourcing chain the module owns → tinder const `«PREFIX»_lower_name` instead. Never reconstruct from BASH_SOURCE. (BCG §Dispatch-Provided Directory Variables) |
| Pre-dispatch launcher body (after BURC sourced) | Use `${BURC_TOOLS_DIR}/...` — the `bul_launch` lines already model this. |
| Bootstrap membrane — tabtarget `exec` line; launcher's `source "Tools/buk/bul_launcher.sh"` | No change. Runs before any var exists; cwd=repo-root is trampoline-guaranteed; the single sanctioned literal at the membrane. Launchers are generated by `buut_tabtarget` — fix the generator, never the outputs. |
| Standalone bash not run through dispatch (e.g. uninstaller) | BURD/BURC absent. Resolve own root once via `${BASH_SOURCE[0]%/*}` into a local and derive the rest, or document a cwd contract. Per-script judgment. |
| Bash installer/arcanum acting on a target repo (`$target/Tools/...`) | No change where `Tools/...` is the destination contract in the consumer repo — not a self-reference. Verify each: own-location → candidate; target destination → intrinsic. |
| Rust running-code path | Hoist relative to a `pub const {PREFIX}_SCREAMING: &str` (RCG Identity Rule; exemplar already correct: `VVCC_REGISTRY_PATH`), derive every `root.join(...)` from it. No env-var path in Rust — `root` arrives as a parameter. Const placement/name is mount-time; reuse an existing const home if one fits. |
| Rust kit-root literals (`Tools/rbk`, `Tools/buk`) | Same const treatment for completeness; lower risk. |
| Rust comment / docstring | Prose — fix only if the named path is stale. |
| Rust test fixture / expected-output assertion | The spec of emitted output. Reference the shared const the producer uses, or leave as the expected literal. Lowest priority. |
| Config without dispatch context — `*compose.yml` bind mounts, `Cargo.toml` path deps, `Dockerfile` | No variable mechanism. Cargo path deps stay static (no change). compose bind-mounts are relative to the compose file = deployment contract (leave unless an orchestrator var is already in play). Dockerfile `Tools/...` is a comment (fix-if-stale). |

## Out of scope — docs
`*.md`, `*.adoc`, `Memos/`, root `CLAUDE.md`. The acronym maps and spec cross-refs
intentionally register `ACRONYM → Tools/...`; keeping them current is the rename
workflow's job, not this sweep. Flag separately if a rename already left a map stale.

## Done
Every `Tools/` grep hit in scope carries a verdict; candidate violations are
converted to the locale-appropriate mechanism; sanctioned/intrinsic hits are left
with their rationale; lint and the relevant test tier pass.

### kindle-entry-0trick-renames (₢BYAAE) [complete]

**[260605-0824] complete**

## Character
Mechanical rename + source-line fixups per module. Low judgment, except confirming each rbh0 module's gestalt ModuleName (guessed below).

## Goal
Bring all 6 decomposed modules' kindle "main submodule" to the BCG 0-trick entry form `«prefix»0_«ModuleName».sh` (the single inclusion-guard + whole kindle). Every CLI already conforms (`«prefix»0_cli.sh`); only the kindle entry file is non-conforming. Authority: BCG "Module Decomposition" + Fading Memory FM-002.

## Renames
Group 1 — k-form (FM-002 direct):
- rbldk_Kindle.sh → rbld0_Lode.sh
- rbfck_Kindle.sh → rbfc0_FoundryCore.sh
- rbflk_Kindle.sh → rbfl0_FoundryLedger.sh

Group 2 — rbh0 family, b-base → 0_ModuleName (gestalt names GUESSED — confirm at mount):
- rbhob_base.sh → rbho0_Onboarding.sh
- rbhpb_base.sh → rbhp0_Payor.sh
- rbhwb_base.sh → rbhw0_Windows.sh

## Per rename
Rename the file, then `grep` the OLD basename across the tree to find every sourcer and update each `source` line. Sourcers cross modules: `rbfck_Kindle.sh` is sourced by `rbldk_Kindle.sh` as well as its own `rbfc0_cli.sh`, so the grep-for-sourcers step is load-bearing, not a single-line edit.

## Cinched
- CLIs already 0-form — touch them only for the source-line edit.
- rbh0 `0` slot already holds content landings (`rbho0_start_here.sh`, `rbhw0_top.sh`); the renamed kindle entry is an ADDITIONAL 0-file, not a replacement for those.
- No behavior change — pure rename + sourcing-path edits.

## Done
All 6 kindle entries are `«prefix»0_«ModuleName».sh`; every sourcer updated (verified by grepping the old basenames → zero hits); `rbk-claude-acronyms.md` RBLDK/RBFCK/RBFLK + rbh*b entries updated to the new names; the relevant suite is green (dogfight + lode-lifecycle exercise rbld/rbfc/rbfl; handbook display smoke if one exists for rbh0).

**[260604-1439] rough**

## Character
Mechanical rename + source-line fixups per module. Low judgment, except confirming each rbh0 module's gestalt ModuleName (guessed below).

## Goal
Bring all 6 decomposed modules' kindle "main submodule" to the BCG 0-trick entry form `«prefix»0_«ModuleName».sh` (the single inclusion-guard + whole kindle). Every CLI already conforms (`«prefix»0_cli.sh`); only the kindle entry file is non-conforming. Authority: BCG "Module Decomposition" + Fading Memory FM-002.

## Renames
Group 1 — k-form (FM-002 direct):
- rbldk_Kindle.sh → rbld0_Lode.sh
- rbfck_Kindle.sh → rbfc0_FoundryCore.sh
- rbflk_Kindle.sh → rbfl0_FoundryLedger.sh

Group 2 — rbh0 family, b-base → 0_ModuleName (gestalt names GUESSED — confirm at mount):
- rbhob_base.sh → rbho0_Onboarding.sh
- rbhpb_base.sh → rbhp0_Payor.sh
- rbhwb_base.sh → rbhw0_Windows.sh

## Per rename
Rename the file, then `grep` the OLD basename across the tree to find every sourcer and update each `source` line. Sourcers cross modules: `rbfck_Kindle.sh` is sourced by `rbldk_Kindle.sh` as well as its own `rbfc0_cli.sh`, so the grep-for-sourcers step is load-bearing, not a single-line edit.

## Cinched
- CLIs already 0-form — touch them only for the source-line edit.
- rbh0 `0` slot already holds content landings (`rbho0_start_here.sh`, `rbhw0_top.sh`); the renamed kindle entry is an ADDITIONAL 0-file, not a replacement for those.
- No behavior change — pure rename + sourcing-path edits.

## Done
All 6 kindle entries are `«prefix»0_«ModuleName».sh`; every sourcer updated (verified by grepping the old basenames → zero hits); `rbk-claude-acronyms.md` RBLDK/RBFCK/RBFLK + rbh*b entries updated to the new names; the relevant suite is green (dogfight + lode-lifecycle exercise rbld/rbfc/rbfl; handbook display smoke if one exists for rbh0).

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 A lifecycle-split-establish-rekey
  2 B consider-per-person-multirole-rbra
  3 C divest-readflap-poll-debounce
  4 D tools-magic-string-sweep
  5 E kindle-entry-0trick-renames

ABCDE
x···x rbfd_FoundryDirectorBuild.sh
····x claude-rbk-acronyms.md, rbfc0_FoundryCore.sh, rbfc0_cli.sh, rbfck_Kindle.sh, rbfh_cli.sh, rbfk_kludge.sh, rbfl0_FoundryLedger.sh, rbfl0_cli.sh, rbflk_Kindle.sh, rbfr_FoundryRetriever.sh, rbfv_FoundryVerify.sh, rbho0_Onboarding.sh, rbho0_cli.sh, rbhob_base.sh, rbhp0_Payor.sh, rbhp0_cli.sh, rbhpb_base.sh, rbhw0_Windows.sh, rbhw0_cli.sh, rbhwb_base.sh, rbld0_Lode.sh, rbld0_cli.sh, rbldk_Kindle.sh
···x· rbtdrf_fast.rs, study_workbench.sh, vvu_uninstall.sh
··x·· memo-20260604-dogfight-ordain-poll-connect-timeout.md
x···· BCG-BashConsoleGuide.md, RBSCIG-IamGrantContracts.adoc, RBSCIP-IamPropagation.adoc, RBSDD-director_divest.adoc, RBSDK-director_invest.adoc, RBSRD-retriever_divest.adoc, RBSRK-retriever_invest.adoc, bud_dispatch.sh, bul_launcher.sh, but_test.sh, memo-20260604-credential-churn-leak-and-propagation-races.md, rba_Auth.sh, rbgb_Buckets.sh, rbgd_DepotConstants.sh, rbgg_Governor.sh, rbgi_IAM.sh, rbgjv03-assemble-push-vouch.sh, rbgp_Payor.sh, rbk-claude-acronyms.md, rbk-claude-tabtarget-context.md, rbndb_base.sh, rbq_Qualify.sh, rbtdgc_consts.rs, rbtdrk_canonical.rs, rbte_engine.sh, rbw-tl.Shellcheck.sh, rbz_zipper.sh

Commit swim lanes (x = commit affiliated with pace):

  1 C divest-readflap-poll-debounce
  2 A lifecycle-split-establish-rekey
  3 B consider-per-person-multirole-rbra
  4 D tools-magic-string-sweep
  5 E kindle-entry-0trick-renames

123456789abcdefghijklmnopqrstuv
········x···x··················  C  2c
·········xxx·xxx··x·xxx·xx·····  A  12c
·······················x·······  B  1c
··························xx···  D  2c
·····························xx  E  2c
```

## Steeplechase

### 2026-06-05 08:24 - ₢BYAAE - W

Brought all 6 decomposed-module kindle entries to the BCG 0-trick form «prefix»0_«ModuleName».sh (rbldk_Kindle->rbld0_Lode, rbfck_Kindle->rbfc0_FoundryCore, rbflk_Kindle->rbfl0_FoundryLedger, rbhob_base->rbho0_Onboarding, rbhpb_base->rbhp0_Payor, rbhwb_base->rbhw0_Windows). Retargeted every sourcer + the 4 stale rbfd ownership comments; old basenames grep to zero. Folded the retired k/b acronym keys into their «prefix»0 entries in claude-rbk-acronyms.md (preserving terminal-exclusivity). Hoisted z_rbh0_dir into the 3 rbh0 CLIs. Verified: shellcheck 200 clean, all 13 source refs resolve, all 6 modules confirmed to furnish at runtime. Landed in c73767d4.

### 2026-06-05 08:22 - ₢BYAAE - n

0-trick kindle-entry renames: bring all 6 decomposed-module kindle entries to the BCG «prefix»0_«ModuleName».sh form — rbldk_Kindle->rbld0_Lode, rbfck_Kindle->rbfc0_FoundryCore, rbflk_Kindle->rbfl0_FoundryLedger, rbhob_base->rbho0_Onboarding, rbhpb_base->rbhp0_Payor, rbhwb_base->rbhw0_Windows (Group-2 gestalt names confirmed from each module header). Retarget every sourcer (rbfc0_FoundryCore has 8) plus the 4 stale rbfd ownership comments and the rbh0 CLIs' source-error basenames; old basenames now grep to zero. Fold the retired k/b acronym keys (RBLDK/RBFCK/RBFLK, RBHOB/RBHPB/RBHWB) into their «prefix»0 entries in claude-rbk-acronyms.md — the 0-prefix now homes both CLI and gestalt entry, mirroring the RBFD main+CLI-partner idiom; this keeps terminal-exclusivity (the k/b prefixes are retired, not re-pointed). Also hoist a z_rbh0_dir local into the 3 rbh0 CLIs to DRY the repeated rbh0/ path, re-aligned deterministically. Verified: shellcheck 200 files clean; all 13 renamed source refs resolve; all 6 modules confirmed to furnish at runtime (rbho0/rbhw0/rbhp0/rbfc0 RC=0; rbld0/rbfl0 reached their command bodies, only blocked downstream by a GCP SA-propagation credential timeout unrelated to the rename).

### 2026-06-05 07:46 - Heat - n

Retarget all references to the renamed claude-{kit}-{role}.md context files (the rename itself landed in ba404db6). Update CLAUDE.md @-includes + acronym-map list, fix the long-broken consumer @-include (buk-claude-context.md -> claude-buk-core.md) and its tabtarget-context ref, move the live RBCC_tabtarget_context_file constant, fix the cmk roe/roe-detail mutual cross-refs, the APCPS project-structure listing, and 5 stale cmk-claude-context.md references in cmw_workbench.sh (a latent bug: cmw_install's verify-include grep never matched the actual claude-cmk-core.md include). Shellcheck clean (200 files).

### 2026-06-05 14:29 - ₢BYAAD - W

Swept Tools/ magic-string path literals from executable code. Converted 3 files: rbtdrf_fast.rs (5 hoisted path consts, 26 running-code root.join/probate-arg sites, RCG Identity Rule), study_workbench.sh (sources via dispatch-exported ${BURD_BUK_DIR} instead of ${STUDYW_SCRIPT_DIR}/../Tools/buk), vvu_uninstall.sh (inline Tools/vvk/bin hoisted to readonly ZVVU_BIN_RELDIR + documented standalone cwd=repo-root contract). ~70 remaining hits verdict'd no-change by locale (bootstrap-membrane launchers + their buut_tabtarget generator, $target/Tools/vvk installer destination contracts, compose/Cargo/Dockerfile config, comments/docstrings/test-fixtures, and the cmw ZCMW_KIT_PATH emitted-doc display value) and left byte-identical with rationale recorded in report. Adversarial audit of 73 hits: 0 false no-changes. Verification all green: shellcheck 200/200, theurge unit 137/137, dogfight 6/6 (after operator reauth cleared a Cloud Build UNAUTHENTICATED unrelated to sweep), skirmish 254/254 (1 skip), crucible 212/212 (1 skip). Flagged separately (rename-staleness, out of sweep scope): cmw_workbench.sh:119/137/138/139/156 reference the renamed @Tools/cmk/cmk-claude-context.md (now claude-cmk-core.md), leaving the line-137 install-verification grep permanently unmatched.

### 2026-06-05 07:22 - ₢BYAAD - n

Sweep Tools/ magic-string path literals from executable code (tools-magic-string-sweep). Resolved every in-scope grep hit by locale. rbtdrf_fast.rs: hoist 5 repo-root-relative path consts (RBTDRF_BUK_ROOT/RBK_ROOT/BUV_VALIDATION/RBFCB_BUILDHOST/RBLDS_SPINE) and route all 26 running-code root.join/probate-arg sites through them per the RCG Identity Rule (cf. VVCC_REGISTRY_PATH exemplar); the 3 emitted-bash prelude + docstring literals stay as cwd-root-contract spec. study_workbench.sh: source via the dispatch-exported ${BURD_BUK_DIR} instead of ${STUDYW_SCRIPT_DIR}/../Tools/buk (matching cmw_workbench; STUDYW_SCRIPT_DIR retained, still used for the study sub-dir). vvu_uninstall.sh: hoist the inline Tools/vvk/bin literal to readonly ZVVU_BIN_RELDIR beside ZVVU_BURC_RELPATH, with the standalone cwd=repo-root contract documented. Remaining ~70 hits are sanctioned no-change: bootstrap-membrane launcher source lines (+ the buut_tabtarget generator that emits them), $target/Tools/vvk installer destination contracts in voa_arcanum, compose/Cargo/Dockerfile config, comments/docstrings/test-fixtures, and the cmw ZCMW_KIT_PATH emitted-doc display value (converting it would bake an absolute path into a generated artifact). Adversarially audited: 0 false no-changes. shellcheck clean (200 files); theurge unit tests 137/137. Flagged separately (rename-staleness, out of sweep scope): cmw_workbench.sh:119/137/138/139/156 reference the renamed @Tools/cmk/cmk-claude-context.md (now claude-cmk-core.md), leaving the line-137 install-verification grep permanently unmatched.

### 2026-06-05 02:52 - ₢BYAAA - W

Closed H1 (divest revokes IAM bindings before deleting the SA — no deleted:...?uid= tombstones) and H2 (re-invest rotates the key on the standing SA without delete/recreate) in the keyfile credential tier; bash + Rust, specs aligned. Live service-tier validation completed and proven: canonical-invest green twice; the Class-C (403 caller-recently-empowered) propagation tolerance on the resource-scope revokes fired and was load-bearing (repo getIamPolicy 403 for 81s then 200 during divest); tombstone delta confirmed zero new ghosts across a full divest->reinvest cycle (independent payor-token gcloud: divested live uids never ghosted, distinct canonical-ghost count stable 3+3); H2 rotation verified by standing-SA re-invest (uid unchanged, single USER_MANAGED key rotated). Final commit completed the spec-alignment 1a66443e left half-done: RBSCIP Class-C declaration generalized to resource-scope revoke sites (Class C attaches to caller empowerment, not edit direction) plus a 2026-06-05 evidence row, and a revoke-side addendum to the credential-churn memo. Surfaced separately (not in scope): governor_mantle still leaks a governor-owner tombstone per re-mantle (observed 7->8) — same H1 mechanism, governor tier, deliberately scoped out by the grounding memo.

### 2026-06-05 02:50 - ₢BYAAA - n

Complete the pace's spec-alignment from the live validation: generalize RBSCIP's Class-C tolerance declaration to resource-scope revoke sites (not just grant) — Class C attaches to the caller's empowerment freshness, not the direction of the policy edit — and add the 2026-06-05 divest-revoke 403 catch as an evidence row. Add a revoke-side addendum to the credential-churn memo: wiring H1's revoke half exposed the symmetric caller-recently-empowered race (governor_mantle re-empowers the caller immediately before director_divest's resource-scope getIamPolicy), the partner of the grant-side point 5; records the live no-tombstone (H1) and key-rotation-without-recreate (H2) verification.

### 2026-06-04 19:40 - ₢BYAAB - W

Recommendation: do it. Refactor RBRA to identity-based (principal + capability-set), converging the keyfile and federation cult-verb surfaces so a later keyfile->federation switch is non-disruptive for operators. Enforcement stays server-side IAM (un-hackable); the federation shape is adopted, the mechanism deferred. Target model, cinched decisions, and open forks now live in the rbk-15-identity-credential-convergence paddock.

### 2026-06-05 02:15 - ₢BYAAA - n

Restore Class-C (caller-recently-empowered, 403) propagation tolerance to the resource-scope revokes — a real bug the new canonical-invest divest coverage caught on first live run.

### 2026-06-05 02:01 - ₢BYAAA - n

Add a divest -> reinvest recycle to the shared canonical-invest case list so skirmish, dogfight, and blockade exercise the credential teardown path (H1) and the IAM delete->recreate eventual-consistency edge on every run.

### 2026-06-04 21:46 - ₢BYAAA - n

Align the credential-lifecycle specs with the H1/H2 code landed in 9f0beef9.

### 2026-06-04 14:39 - Heat - S

kindle-entry-0trick-renames

### 2026-06-04 21:34 - ₢BYAAA - n

Close H1 (binding-leak) and H2 (re-key churns the SA) in the keyfile credential tier — bash + Rust; specs follow in the next increment.

### 2026-06-04 14:14 - Heat - S

tools-magic-string-sweep

### 2026-06-04 13:26 - Heat - n

Document the GCP eventual-consistency-without-completion-contract problem in deliverable form and surface it at the failure site that bit us. (1) README: new top-view appendix 'Eventual Consistency and the Missing Completion Contract' (anchor EventualConsistency) — frames the general problem of building atop eventually-consistent cloud APIs that expose no pollable terminal-state signal, conceding the consistency model is defensible while indicting the missing completion contract (AIP-151/ARM exist and are withheld on the racing operations); SA delete demoted to the worked example (empty success, no operation handle, soft-delete tombstone recoverable 30 days); plus a StaleDeleteRead Diagnostics entry cross-linking to it. (2) rbgg_Governor.sh: the invest preflight 'already exists' die now appends a buyy_link_yawp hypothesis ('Maybe this is GCP's post-delete read flap...') linking the README anchor, so the error self-explains; RBRR_PUBLIC_DOCS_URL is enforced on the governor path. (3) RBSCIP delete-side flap entry: wove in the soft-delete-tombstone mechanism and the no-LRO/AIP-151-unused fact as the root cause behind the 404->200 read flap. Shellcheck clean (186 files).

### 2026-06-04 20:10 - ₢BYAAA - n

Back out ALL work attempted during this pace, restoring rbgi_IAM.sh and rbgg_Governor.sh to their pre-pace baseline (46d15a1ec^). Removes the IAM revoke layer (three revoke functions + jq purge helper), the ZRBGG_INFIX_KEY_DELETE infix, and the uncommitted governor H2 idempotency edits. The prior attempt drifted from a too-mechanical docket framing into over-reach (a generic helper written before surveying the codebase, a planned rewrite of battle-tested invest verbs, a unilateral contract deletion). Resetting to a clean slate; the pace will be reslated with precise, surgical, BCG-grounded language before any re-attempt.

### 2026-06-04 13:02 - ₢BYAAA - W

Added the rbw-tl standalone shellcheck tabtarget (BCG-configured lint, no test suite) and steered future sessions to it via the rbk acronyms note. Resolved the BCG SC2153-inline-mandate vs no-inline-directives contradiction (Option A): dropped the per-file line-2 coda from Template 1 and 9 files, relying on busc_shellcheckrc's global suppression on the canonical rbw-tl path. Removed the remaining stray inline directives (rbfd SC2016 redundant, rbgjv03 SC2086 vestigial) so zero inline # shellcheck directives remain tree-wide. Fixed rbte_engine.sh lint red (prose comment colliding with shellcheck's directive parser). Made BCG kit-agnostic (removed all rbw/Recipe-Bottle/RBS0 references). Verified: rbw-tl 186 files clean.

### 2026-06-04 13:01 - ₢BYAAA - n

Complete the no-inline-shellcheck-directives prohibition tree-wide and make BCG kit-agnostic. (1) Remove the two remaining stray inline directives: rbfd's # shellcheck disable=SC2016 (redundant — SC2016 is globally suppressed in busc_shellcheckrc) and rbgjv03's # shellcheck disable=SC2086 (vestigial — guarded a glob, nothing fired). Zero inline # shellcheck directives now remain under Tools/. (2) Genericize BCG's remaining Recipe-Bottle-specific references so the Bash Console Guide reads as the kit-agnostic guide it is: launcher.«workbench»_workbench.sh in the tabtarget example, two "«colophon»" yawp samples, and the Declared Dependency Principle prose de-named from Recipe Bottle/RBS0 to the generic project/spec. Verified: rbw-tl 186 files clean; grep confirms no rbw/rbk/RBS0 refs in BCG.

### 2026-06-04 12:50 - ₢BYAAC - W

Hardened rbuh_poll_until_gone against GCP IAM's post-delete read-flap (404->200) so canonical-invest's divest-before-invest is durably idempotent. Root cause (dogfight retriever_invest): divest deleted the SA and poll_until_gone declared gone on the first 404, but the invest's fail-loud existence preflight saw a stale 200 two seconds later. Fix: require RBGC_GONE_CONFIRM_STREAK (3) consecutive 404s, intervening 200 resets; honors e0d8fb281's cinch (idempotency in fixture divest, invest stays fail-loud). New RBGC_GONE_CONFIRM_STREAK constant + rbuh kindle-sentinel. Spec drift repaired: RBSRD/RBSDD Confirm-Deletion-Propagated step now states consecutive-404; RBSCIP gains the delete-side flap mirroring its create-side entry; memo-20260604-canonical-divest-delete-flap.md captures the transcript evidence. BCG-verified (shellcheck rcfile clean, cupel pass). Verified live: dogfight canonical-invest leg green across two runs, run 2 divesting+reinvesting run 1's standing SAs. Notched memo-20260604-dogfight-ordain-poll-connect-timeout.md documenting an unrelated Pale-class transient (build-status poll connect-timeout aborting a healthy ordain) with a recommended repair (split poll retry tolerance by failure class), not applied. Landed heat-affiliated: 2202ece57 fix, 6c38fed34 specs+memo; pace-affiliated ec558c24b ordain memo.

### 2026-06-04 12:49 - ₢BYAAA - n

Resolve the BCG inline-directive conflict (Option A) and green the shellcheck gate. (1) Fix rbte_engine.sh lint red: reflow the Cygwin path-boundary comment so no line begins with the token 'shellcheck' — it was tripping shellcheck's directive parser (SC1073/SC1072, cascading SC1009). (2) Resolve the line-2-SC2153-mandate vs no-inline-directives-prohibition contradiction in BCG by dropping the mandate: remove the line-2 coda from Template 1 and the 9 files carrying it, rely on busc_shellcheckrc's global SC2153/SC2154 suppression (applied on the canonical rbw-tl path that always passes --rcfile), preserve the set -u runtime-backstop note. The prohibition is now literally true. (3) Simplify the rbk acronyms Shellcheck note to a terse directive pointing at busc_shellcheckrc and BCG. Verified: rbw-tl reports 186 files clean.

### 2026-06-04 19:24 - ₢BYAAA - n

Add the IAM revoke layer for credential teardown (H1 fix groundwork): three self-contained per-scope functions rbgi_revoke_project_member/_sa_member/_repo_member plus the shared rbgi_jq_remove_member_from_policy_capture jq helper, mirroring the battle-tested rbgi_add_*_iam_role grant trio (GET -> modify -> setIamPolicy). Survey confirmed no IAM member-removal existed anywhere in the kit. Each revoke runs before SA delete so the live member is purged and no deleted:...?uid= tombstone is created; 409 fatal, transient 5xx retried, idempotent on absent member. BCG-compliant (no generic helper — per the load-bearing rbgi_add_* example), shellcheck-clean. Also adds ZRBGG_INFIX_KEY_DELETE to the governor kindle for the forthcoming automated key blunt-kill.

### 2026-06-04 12:21 - ₢BYAAA - n

Add rbw-tl standalone shellcheck tabtarget surfacing the BCG-configured lint engine (rbq_qualify_shellcheck) without the test suite. Document the no-raw-shellcheck discipline in rbk acronyms (why bare shellcheck floods BCG-structural false positives, use rbw-tl). De-rbw the BCG shellcheck section to honor the BUK-layer/consumer boundary (drop the rbk-specific consumer row, state the wiring pattern generically). Regenerate tabtarget-context doc and Rust colophon consts via theurge build.

### 2026-06-04 12:15 - ₢BYAAC - n

Document the intermittent dogfight ordain failure observed during verification: rbfd_ordain's build-status poll (zrbfc_wait_build_completion) aborted a healthy in-flight Cloud Build after 3 consecutive curl rc=28 connect-timeouts (~45s station-side network blip), conflating transport-unreachability with build failure. Self-healed on rerun (run 2 same path passed). Recommends splitting ZRBFC_BUILD_POLL_RETRY_TOLERANCE into a small body-error tolerance (keep 3, fail fast) and a larger transport-failure tolerance (~8, ride out blips), since status-poll reachability is not build health. Repair recommended, not applied.

### 2026-06-04 12:14 - Heat - S

divest-readflap-poll-debounce

### 2026-06-04 11:51 - Heat - n

Repair spec drift from the poll_until_gone hardening and document the delete-side IAM read-flap. RBSRD/RBSDD 'Confirm Deletion Propagated' step previously said poll 'until HTTP 404' / 'HTTP 404 within deadline' — the single-404 behavior the dogfight failure proved insufficient; now states RBGC_GONE_CONFIRM_STREAK consecutive 404s with intervening non-404 reset, cross-referencing rbscip_trade_study. RBSCIP (IAM Propagation knowledge base) documented only the create-side flap (200->200->404 after create); added the inverse delete-side flap (404->200 after delete, single 404 not durable proof) as a body bullet + evidence row, mirroring the create-side pattern. New memo-20260604-canonical-divest-delete-flap.md captures the reaped-at-risk transcript evidence (depot cancbhm-d-canest3bhm100001: divest GET 200,200,404 'gone at 6s', invest preflight GET 200 2s later). Records that the task's leftover-SA hypothesis was contradicted by the trace — divest ran and confirmed-gone; the defect is the read-flap.

### 2026-06-04 11:43 - Heat - n

Harden rbuh_poll_until_gone against GCP IAM read-flap so canonical-invest's divest-before-invest is durably idempotent. Root cause (dogfight suite, retriever_invest): the divest ran and deleted the SA (DELETE 200), poll_until_gone saw GET 200,200,404 and declared gone at 6s, but the recreate's preflight GET flapped back to 200 two seconds later — GCP IAM's SA read path is multi-replica eventually-consistent, so a single 404 is not durable proof of deletion. poll_until_gone now requires RBGC_GONE_CONFIRM_STREAK (3) consecutive 404s before returning, resetting the streak on any intervening 200; total wait still bounded by RBGC_MAX_CONSISTENCY_SEC. New RBGC_GONE_CONFIRM_STREAK constant + rbuh kindle-sentinel validation. Honors e0d8fb281's cinch (idempotency in fixture-level divest, invest stays fail-loud) — fix is in the divest/poll membrane, preflight untouched. Sole caller is rbgg divest.

### 2026-06-04 10:29 - Heat - n

Capture the keyfile-tier credential-lifecycle hole grounding heat rbk-08: objects (SA/Key/RBRA/Binding/Resource), the three lifecycles, H1 (Binding revoke never wired) + H2 (re-key runs the SA lifecycle, not the Key lifecycle), the three observed faces (binding leak, create->grant DRS race, delete->recreate race) as consequences, and the establish/re-key/teardown split as the fix. Federation noted as the eventual successor tier, not the MVP fix.

### 2026-06-04 10:28 - Heat - S

consider-per-person-multirole-rbra

### 2026-06-04 10:27 - Heat - S

lifecycle-split-establish-rekey

### 2026-06-04 10:27 - Heat - f

racing

### 2026-06-04 10:25 - Heat - N

rbk-08-credential-repairs

