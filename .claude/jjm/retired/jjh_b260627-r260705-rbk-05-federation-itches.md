# Heat Trophy: rbk-05-federation-itches

**Firemark:** ₣Bo
**Created:** 260627
**Retired:** 260705
**Status:** retired

## Paddock

# Paddock: rbk-05-federation-itches

## Context

(Describe the initiative's background and goals)

## References

(List relevant files, docs, or prior work)

## Paces

### federation-open-tendrils-catalog (₢BoAAA) [complete]

**[260630-1438] complete**

Remember the remaining open federation tendrils from the 260627 orient/trot — the ones we did NOT reach a solid resolve to fully address. Context-preservation pace: each section carries the issue, where it stands, the lean (if any), and the coupling, so a later session can act without re-deriving. Source orientation: the ₣Bf paddock "Foedus switching — AUTHORED + residual shape" block, and the dockets named below.

## PRIORITY — drift codebase study (gates the descry / drift / rotation cluster)
The affiance-on-a-pre-existing-pool behavior was re-decided inconsistently across 260627's sessions (the stable->ephemeral key reversal, layered "supersedes" notes, config-match present in one docket but not in the spec). Before ratifying anything in the cluster below, study the CODE: what does affiance (rbgp_payor.sh, the workforcePools.get 200/404/DELETED branch) actually do on a pre-existing pool — write-once-leave-in-place, refuse-and-rotate, and what does it compare? Establish ground truth from the code, then ratify tendrils 3/5/6 against it. This study SUBSUMES the deferred tendril-3 decision; nothing in the cluster is settled until it runs.

## Tendril 3 — descry config-match (DECISION, gated on the study)
descry's contract RBSFD specs pool-health only (present / absent / soft-deleted / provider-missing) — no local-config-vs-cloud-provider MATCH. But the pool-state-classifier pace ₢BfAAs's docket promises "descry's config-drift 'valid' check rides on top of" the classifier. Decision owed: amend RBSFD to add config-match as a descry validity dimension, OR keep RBSFD pool-health-only and make config-comparison the reuse fixture's job (₢BlAAE).
Lean (operator-unratified): amend RBSFD, scoped to DURABLE/INTERACTIVE foedera only — drift bites only a foedus left standing across local edits, and only the interactive Entra foedus stands; the programmatic Keycloak foedus is ephemeral (torn down + re-affianced per run, fresh key per charge) so it is rebuilt-from-local, never reused-while-edited, and a programmatic matcher would be degenerate. Hedge: this assumes the programmatic foedus is fully re-established per run (from the orchestrator's jilt-then-quench) — if a programmatic foedus is ever kept standing, the per-flavor matcher question returns.

## Tendril 5 — local<->cloud drift (deferred FIX)
affiance is write-once on the provider (leaves an existing one in place — the 200-branch), so later local rbrf.env edits never propagate and the cloud provider silently drifts. DETECTION rides tendril 3's config-match. The FIX — affiance PATCHing a drifted provider — stays DEFERRED; repair today is jilt + re-affiance, never in-place patch. NOT a Palisade membrane (our code, editable). Home when ready: an itch, or RBSHR (Horizon Roadmap), the more durable spec-resident choice.

## Tendril 6 — rotation-coupling fact needs a spec home (small spec-write)
Durable fact, currently only in the ₣Bf paddock: Google keys IAM bindings to the workforce-pool ID, not the foedus rbef_ name, so refuse-and-rotate (bumping RBRF_WORKFORCE_POOL_ID on soft-delete) cascades into re-admitting every citizen — which is why the durable pool is kept standing. Per CLAUDE.md (a fact that must stay true after a memo retires needs a spec home, not a paddock), write it into RBSRF or RBSMA. Lean: small spec-write, fold into ₢BfAAs (which implements refuse-and-rotate — this fact is the why behind it) or ₢BfAAM. Rides the drift study (same affiance-pre-existing-pool behavior).

## Tendril 7 — lift the ₢BlAAR barrier (READY action, not a design-open)
₢BlAAR (₣Bl) gates the foedus build paces on ₢BfAAT being merged into this clone. ₢BfAAT is wrapped and its four subdocs (RBSFD / RBSFI / RBSRR / RBSRF) are present in this working tree — so the barrier is liftable. To lift: wrap ₢BlAAR (needs operator confirm — never auto-wrap) AFTER confirming this is the clone ₣Bl runs in. Lifting unblocks ₢BlAAE and the canvass build/test paces.

## Tendril 4 — AXLA opinion-registration voicing (CMK pass, itch-grade)
Coin an AXLA voicing for REGISTERING an opinion + an RBS0 header stanza to state RB's opinions, so e.g. the clean-tree gate rationale CITES a registered opinion instead of restating it. First candidate: the clean-commits-for-container-coherence rationale (the clean-tree gate variant ships it as a plain RBCC creed constant; this would re-home it). CMK/MCM design pass, orthogonal to federation, thin now — defer until enough opinions accrue (load-bearing-complexity test). May instead belong in jji_itch as a CMK nudge.

## Tendril 8 — terrier-poison <-> ₢BfAAF retirement sequencing (WATCH at mount)
₢BfAAF retires the interim rbw-dt/rbw-dT terrier scaffold + the terrier-scaffold / terrier-atomicity fixtures; ₢BlAAY (terrier-poison-fixture) REPLACES the retired atomicity proof. Cross-refs already wired in both dockets (₢BlAAY's Cinched names ₢BfAAF). Sequence so the new fixture arrives as the old goes — no duplicate retirement, no coverage gap. Likely no action beyond watching at mount.

## Tendril 9 — canvass -> rbef_ mapping (build detail for ₢BlAAU)
canvass (₢BlAAU, rbw-jc) enumerates foedera via the cloud (workforcePools.list) and must mark the regime-selected one — which requires correlating a cloud pool-id back to an rbef_ library identity. ₢BlAAU's docket emits pool-id + regime-selected-flag but is silent on HOW: does affiance stamp the rbef_ name into the pool's label/displayName (cloud-side link), or does canvass match pool-ids against the rbrf library's RBRF_WORKFORCE_POOL_ID values (local cross-walk)? Quietly touches affiance (whether it labels the pool). Settle when ₢BlAAU is built; if cloud-side labeling is chosen, affiance must be taught to stamp it.

## Tendril 10 — "proof" word over-promises (MCM Lapidary)
"proof" (rbgp_terrier_proof, fdkyclk-proof, MarshalProofs) reads as the math/rigor sense but names bash demonstrations. The terrier instance self-retires (₢BfAAF + ₢BlAAY); the rest persist. Consider an eviction-sweep remint toward an honest demonstration / shakedown / proving-ground valence, if worth a Lapidary pass. Minor.

## Done when
Each tendril above has either graduated to its own slated work on whichever heat owns it, or been explicitly declined — at which point this remember-pace wraps. The drift study is the gating first move for tendrils 3/5/6.

## Character
Context-preservation catalog, not a build pace; the deliverable is that no open federation tendril is lost. The drift study is the one piece of real investigation work embedded here and may graduate to its own pace.

### mantle-admission-denial-band (₢BoAAB) [complete]

**[260701-1349] complete**

Give mantle-admission denial a named precision band, so role isolation is asserted by exit code instead of by "it failed" —
plus apply the ₣Be review follow-ups that touch the same files.
Design fully settled at the Fable review (2026-07-01);
rationale and file anatomy live in `Memos/memo-20260701-fbl-be-negatives-review.md` ("Settled design" section) —
read it first, it makes every step below mechanical.
(The memo's CLAUDE.md suite-table follow-up is already done — landed separately with the theurge-cosmology subdoc; it is not in the list below.)

## Spec of needed change

Mint `BUBC_band_admission=109` in the bubc sole-mint block (bubc_constants.sh),
with the block-convention comment (new gate, distinct code),
and extend the hand-kept band emit list in `rbcc_emit_consts` (rbcc_constants.sh) so `RBTDGC_BAND_ADMISSION` projects;
rebuild via `tt/rbw-tb.Build.sh` to regenerate the consts.
In `rba_don_capture` (rba_auth.sh), change ONLY the Leg-3 HTTP `403` case arm's return from 1 to the admission band;
the other five failure modes (lapsed sitting, jq body, curl transport, other HTTP, empty accessToken) keep return 1;
update the function's header prose ("or returns 1") to name the distinguished admission return.
In `rbgv_check_mantle` (rbgv_cli.sh), branch on the capture's return:
admission band → `buc_reject` the admission band with an operator-facing deficit message carrying the brevet instruction;
any other nonzero → the existing `buc_die` unchanged.
No edits to any other `rba_don_capture` consumer — the `buc_die` band membrane fans the code out for free.
Add a picket-tier fixture (proposed name `mantle-denial`, home rbtdrv_patrol.rs, terrier-atomicity as the enrollment template:
manifest fixture const + colophon map entry, almanac membership in picket and echelon — never reveille, fixture static with `credless: false`):
don retriever (positive) → unseat retriever → poll don until it exits the admission band
(bounded ceiling — IAM revocation propagates eventually; assert the EXACT band at the terminal state)
→ brevet retriever back → poll don until positive (restore proof).
Self-skip on payor-unreachable like the sibling picket fixtures;
keep the don/unseat scaffolding a discrete helper.
Apply the five memo follow-ups:
BCG band-section carve-out line (usage refusals stay imprecise death; their cases compensate with a pointer assertion);
rba_auth.sh keyfile-ghost guard comment;
rbtdrs_poison.rs pre-rename suite names;
rbtdrm_manifest.rs podvm-resolve stale comment;
reword the BBAA9 coronet code comment purpose-based.
Notch before every test; run reveille, then the new fixture singly via `tt/rbw-tf.FixtureRun.sh` against the standing depot.

## Done when

A don of an unheld-or-unseated mantle exits the admission band at the tabtarget boundary (rbw-am),
distinguished from lapsed-sitting / network / other-HTTP failures, which stay imprecise death;
the mantle-denial fixture proves deny-then-restore on the real freehold subject asserting the exact band;
`RBTDGC_BAND_ADMISSION` projects via the emit list;
the five memo follow-ups are applied;
theurge build and reveille green, and the new fixture green against the standing depot.

## Cinched

Honors the single-subject mantle-impersonation design (cinched 260612) — one subject losing a mantle and being denied, never pseudo-people.
Tests assert the exact exit code, never output text, and assertions are never weakened;
on any live-GCP surprise: report mechanism + one proposed repair and stop — no improvised sleeps or retries beyond the cinched bounded poll.
`rba_don_capture` stays a pure capture — the 403 distinction is the in-band return signal; no die, no reject, no stdout-contract change inside it.
Membrane fan-out is the design: polity verbs exiting the admission band when wielded without the governor mantle is intended, not a regression.
Target mantle is retriever, never governor (unseating governor saws off the wielding branch).
The word `admission` is cinched (settled at the Fable review; the memo records the mint reasoning).
Coordinate with the ₣Bl parley authentic-verb fixture (₢BlAAX, not yet landed): the don/unseat helper stays discrete so it can fold onto it later.
The memo is provenance, not authority — read it for rationale; this docket governs.

## Character

Mechanical for sonnet — every judgment point was pre-settled at the Fable review;
the one live-cloud subtlety (IAM revocation propagation) is handled by the cinched bounded poll.

### mantle-token-home-and-isolation-matrix (₢BoAAC) [complete]

**[260701-1442] complete**

Mint the canonical mantle-identity-token home (pallium-sprued), repoint the patrol off the account-fragment conflation, and prove mantle-don isolation by exit code.

Today rbtdrv_patrol.rs:1418 dons each mantle by passing the ACCOUNT fragments (RBTDGC_ACCOUNT_{GOVERNOR,DIRECTOR,RETRIEVER}, emitted bare from RBCC_account_*) to rbw-am — account-id pieces standing in where mantle tokens belong (the conflation).
A subject lacking a mantle is denied only by the live GCP IAM at the don, surfacing as the admission band once the band pace lands.

## Spec of needed change
Mint RBCC_mantle_{governor,director,retriever} in rbcc_constants.sh — the canonical identity-token home; VALUES carry the pallium value-sprue rbpa_ (RBCC_mantle_governor="rbpa_governor", ...). Document the rbpa_ = pallium legend.
Codegen carries the sprued value cleanly: emit RBCC_mantle_* via rbcc_emit_consts to RBTDGC_MANTLE_*, exactly as RBTDGC_MOORINGS_DIR="rbmm_moorings" already does — a single-prefix NAME holding a sprued VALUE, NOT nesting.
The sprued token is THE token everywhere: the rba_don_capture / rbgv_check_mantle case statements resolve "rbpa_governor", and rbw-am's folio takes the sprued token (no clean/canonical two-form, no normalization). It resolves to the mantle SA (rbma-governor) through the case, so the underscore never reaches an SA-id.
Repoint rbtdrv_patrol.rs:1418 (and its imports) off RBTDGC_ACCOUNT_* onto RBTDGC_MANTLE_* — fixing the account-as-mantle conflation.
Extend that don-each loop into the isolation matrix: leave-one-out — brevet the freehold subject onto two mantles, withhold the third, assert rbw-am on the withheld mantle rejects with the admission band while the held two reach AR.

## Done when
RBCC_mantle_* (rbpa_-sprued) is the canonical token home, emitted to RBTDGC_MANTLE_*;
the don/probe case statements and rbw-am resolve the sprued token, which resolves to the mantle SA via the case;
rbtdrv_patrol.rs no longer sources mantle tokens from RBTDGC_ACCOUNT_*;
a lacking/withheld-mantle don rejects with the admission band from the tabtarget exit;
the patrol's leave-one-out matrix proves the IAM denies a withheld mantle (admission band) while the held two reach AR.

## Cinched
rbpa_ (pallium) IS kept — codegen of a sprued value is the established RBTDGC_MOORINGS_DIR="rbmm_moorings" shape (one-prefix name, sprued value), not nesting.
The account fragments RBCC_account_* stay bare because they compose GCP SA-ids (RFC1035 forbids underscores) — that is the marker pace's concern, NOT a constraint on this token class.
Single-valued: one mantle per colophon. The leave-one-out runtime is the drift guard — no static analysis, no derivation.
The existing terrier bare-wire uses (rbgft_mantle value + its GCS paths) are a separate deferred migration.
Payor isolation deferred. Tests assert the named band, never output text.
Depends on the admission band (₢BoAAB); coordinate with the ₣Bl parley fixture (₢BlAAX) — do not duplicate the don/unseat scaffolding.

## Character
Prefix mint + bash home + emit + patrol repoint + leave-one-out theurge matrix; band, token home, and the rbpa_ sprue settled this chat (260630).

### account-unhewn-marker-rename (₢BoAAD) [complete]

**[260701-1637] complete**

Mark the naked account-fragment constants as codegen sources that must stay bare, so the consolidation/spruing temptation cannot recur.

The RBCC_account_* fragments (governor / director / retriever / payor / mason) compose GCP SA-ids and secret-directory names, so they MUST stay bare (RFC1035 forbids underscores in an SA-id), and they are codegen sources (rbcc_emit_consts -> RBTDGC_ACCOUNT_*).
The original consolidation/converge intent is dropped as inappropriate: the sprued mantle token is a separate class (the preceding pace), and these fragments are correctly bare.

## Spec of needed change
Add a codegen infix to the RBCC_account_* family names so the name itself records "bare on purpose — codegen source / SA-id leaf; do not sprue or consolidate."
The infix MUST be stripped on emit so the generated names are unchanged (RBTDGC_ACCOUNT_GOVERNOR stays itself, never ..._CODEGEN_...); teach rbcc_emit_consts the strip.
Repoint the bash consumers of the renamed constants — the def / emit / fact-ext sites, the rbgp_payor.sh unmake pattern, the rbdc_derived / rblm payor secret-dir, the rbgd_depot / rbgp mason email.

## Done when
The RBCC_account_* family carries the codegen infix recording why it stays bare;
rbcc_emit_consts strips it so the RBTDGC_ACCOUNT_* names are unchanged;
all consumers are repointed and the build regenerates clean.

## Cinched
This is a marker rename, NOT a consolidation — the account fragments stay bare and independent of the sprued mantle class.
Scope: the whole RBCC_account_* family (all are bare SA-id-composing codegen sources), unless the operator narrows it to the three role accounts.
Runs after the preceding pace repoints the patrol off RBTDGC_ACCOUNT_*, so the rename's blast radius no longer includes the mantle-token use.

## Character
Mechanical family rename + emit-strip + consumer repoint; the "why" rides the name.

### syft-single-arch-sbom-fix (₢BoAAE) [complete]

**[260701-1924] complete**

The about+vouch SBOM step fails on a single-arch image.
rbgja02-syft-per-platform.sh's single-platform branch scans the bare image tag with no platform constraint,
so syft falls back to its linux/amd64 default and dies ("no child with platform linux/amd64") on an arm64-only image —
the shape a local graft build on Apple Silicon produces, which is why ordain_graft_demo is the first case to hit it.
Pin the actual discovered platform so single-arch images get an SBOM.

## Done when
A single-arch (locally-built arm64) image produces an SBOM through the about pipeline,
and onboarding-sequence's ordain_graft_demo case completes its about+vouch build green.

## Cinched
Always generate the SBOM — never skip syft for graft.
The SBOM is a content inventory for CVE response, valuable independent of build provenance;
graft's low-provenance is the trust axis, not the inventory axis (the skip-for-graft alternative was studied and declined).
Fix by pinning the discovered platform in the single-platform scan — pass --platform, or reuse the @digest the multi-platform branch already pins (platform_digests.txt is always written by discover-platforms).
Fold in SYFT_CHECK_FOR_APP_UPDATE=false to kill the anchore.io version-check egress timeout (a non-fatal warning in the same step).
Spec home: RBSAB (about pipeline).

## Character
Small, well-targeted cloud-step fix; mechanism fully diagnosed.

### rbsab-syft-scan-scheme-reconcile (₢BoAAF) [complete]

**[260702-0318] complete**

RBSAB and the syft SBOM step (rbgja02-syft-per-platform.sh) disagree on how the per-platform scan target is chosen.
The code and the spec do not merely differ in one mode — they branch on different axes entirely, so neither is a subset of the other.
This pace decides which is canonical and aligns the other; no urgency, surfaced during the ₢BoAAE single-arch SBOM fix (commit 56b0465f6).

## Background — what each side does now

The CODE, after the ₢BoAAE fix, digest-pins EVERY syft scan.
Single- and multi-platform, and all three vessel modes (conjure / bind / graft) alike, now scan `registry:<image>@<digest>`, the digest read from `platform_digests.txt` (always written by discover-platforms step 01).
There is no longer any branch — one unconditional digest-pinned loop.

The SPEC (RBSAB, section "Cloud Build Step 2: Syft SBOM Generation" — find it via `grep -n "Syft SBOM Generation" Tools/rbk/vov_veiled/RBSAB-ark_about.adoc`) documents a THREE-mode scheme keyed on vessel mode:
- conjure multi-platform → per-platform TAG scan (`image:<HALLMARK>-<PLATFORM_SUFFIX>`)
- conjure single-platform → bare TAG scan (`image:<HALLMARK>`)
- bind / graft, any count → per-platform DIGEST scan (`image@<PLATFORM_DIGEST>`)
The spec even carries a NOTE explaining WHY graft needs digest pinning: grafted images may be stored as OCI indexes carrying attestation manifests, so a bare-tag scan makes syft auto-select the worker's native platform (amd64) and fail on an image of another arch — i.e. the spec already correctly diagnosed the exact bug ₢BoAAE fixed.

## Background — how they drifted (the axis mismatch)

The OLD code (before 56b0465f6, read its parent) branched on platform COUNT, not vessel MODE: single-platform → bare tag, multi-platform → digest.
Cross-referenced against the spec's mode-keyed scheme, the old code:
- matched spec for conjure-single (bare tag) and for bind/graft-multi (digest);
- ALREADY diverged, pre-existing, for conjure-multi (code used digest; spec says tag+suffix);
- VIOLATED spec for bind/graft-single (code used bare tag; spec says digest) — this was the ₢BoAAE bug, where the spec was right and the code was wrong.
The ₢BoAAE fix corrected the graft-single violation but went further: it made conjure (both counts) digest-pin too, so conjure now NEWLY diverges from the spec's tag/suffix scheme. Net result: code = "always digest," spec = three modes.

## The decision this pace must make

Is "always digest-pin" the correct canonical behavior?
- Leading hypothesis: YES. Digest pinning always resolves the exact manifest and is immune to index/attestation-manifest ambiguity; tag/suffix scanning only works when there is no such ambiguity. The code is strictly safer, so the code should win and RBSAB Step 2 should be rewritten to collapse its three-mode table into one digest-pinned mode (deleting the conjure tag+suffix language and the single-platform bare-tag mode).
- BEFORE deleting spec language, disprove the alternative: is conjure's tag/suffix scanning load-bearing for a reason not captured here? e.g. does the per-platform *suffix tag* carry something the digest does not (does the produced SBOM need to reflect the tag-addressed artifact rather than the digest-addressed one)? Was the suffix-tag scheme chosen for a conjure-only property? If such a reason exists, the code should instead restore mode-awareness and this pace flips direction.
Record the decision and its reasoning wherever it lands (spec prose or a code comment), so this drift cannot silently recur.

## Done when
RBSAB "Cloud Build Step 2: Syft SBOM Generation" and rbgja02-syft-per-platform.sh describe the same scan-target scheme — either the spec is rewritten to the code's "always digest-pin" single mode (expected), or the code is restored to a mode-aware scheme matching the spec — with the canonical-choice reasoning recorded.

## Character
Spec-vs-code reconciliation requiring judgment — decide which side is canonical (leaning: code), then align the other and record why. Not mechanical; small in edits, load-bearing in the decision.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 A federation-open-tendrils-catalog
  2 B mantle-admission-denial-band
  3 C mantle-token-home-and-isolation-matrix
  4 D account-unhewn-marker-rename
  5 E syft-single-arch-sbom-fix
  6 F rbsab-syft-scan-scheme-reconcile

ABCDEF
·xxx·· rbcc_constants.sh, rbgv_cli.sh
··xx·· rbgp_payor.sh
·xx··· rba_auth.sh, rbtdgc_consts.rs, rbtdrv_patrol.rs
·····x RBSAB-ark_about.adoc
····x· rbgja02-syft-per-platform.sh
···x·· rbdc_derived.sh, rbgd_depot.sh, rblm_cli.sh
··x··· claude-rbk-tabtarget-context.md, rbfcb_host.sh, rbfcg_gar.sh, rbfcp_plumb.sh, rbfd_director.sh, rbfld_delete.sh, rbfln_inventory.sh, rbflw_wrest.sh, rbfly_yoke.sh, rbfr_retriever.sh, rbfv_verify.sh, rbga_registry.sh, rbgb_buckets.sh, rbgg_governor.sh, rbldb_bole.sh, rbldl_lifecycle.sh, rbldr_reliquary.sh, rbldv_immure.sh, rbldw_underpin.sh, rbob_bottle.sh, rbtdrk_depot.rs, rbz_zipper.sh
·x···· BCG-BashConsoleGuide.md, bubc_constants.sh, rbtdra_almanac.rs, rbtdrf_fast.rs, rbtdrm_manifest.rs, rbtdrs_poison.rs

Commit swim lanes (x = commit affiliated with pace):

  1 A federation-open-tendrils-catalog
  2 B mantle-admission-denial-band
  3 C mantle-token-home-and-isolation-matrix
  4 D account-unhewn-marker-rename
  5 E syft-single-arch-sbom-fix
  6 F rbsab-syft-scan-scheme-reconcile

123456789abcdefghijklmnop
·········x···············  A  1c
············xx···········  B  2c
··············xxx········  C  3c
··················xx·····  D  2c
····················xx···  E  2c
·······················xx  F  2c
```

## Steeplechase

### 2026-07-02 03:18 - ₢BoAAF - W

Reconcile RBSAB Step 2 (Syft SBOM Generation) with rbgja02-syft-per-platform.sh: decided the code's 'always digest-pin' is canonical and rewrote the spec to match. Investigation resolved the load-bearing question by disproving the alternative — conjure's per-platform suffix-tag scan was NOT load-bearing: the suffix tags (HALLMARK-<suffix>) are pushed on the ATTEST package (rbgjb05/rbgjb06), not the IMAGE package, which carries only the single image:HALLMARK manifest-list tag, so the spec's conjure-multi scan target image:HALLMARK-<PLATFORM_SUFFIX> named a tag that never exists (and the old count-branched code never used it either). Collapsed the spec's three-mode table into one unconditional digest-pinned mode; preserved the index/attestation-ambiguity reasoning, added that the single-arch hazard is not graft-unique (a single-platform buildx image can still be published as an index), and added a 'Why one digest mode' NOTE recording the decision plus the attest-vs-image-package root cause to prevent recurrence. Code needs no change (already correct post-BoAAE); reasoning homed in spec prose. Spec-only .adoc edit, no test tier applies.

### 2026-07-02 03:18 - ₢BoAAF - n

RBSAB: unify about-step syft scanning to digest-only — all modes, all platform counts

### 2026-07-01 19:25 - Heat - S

rbsab-syft-scan-scheme-reconcile

### 2026-07-01 19:24 - ₢BoAAE - W

Fixed the about pipeline's syft SBOM step dying on single-arch (arm64) images. rbgja02-syft-per-platform.sh's single-platform branch scanned the bare image tag with no platform constraint, so syft fell back to its linux/amd64 default and died ('no child with platform linux/amd64') on an arm64-only image — the shape a local graft build on Apple Silicon produces. Pinned all scans to registry:...@<digest> from platform_digests.txt (always written by discover-platforms step 01, single-platform included; verified in rbgja01-discover-platforms.py _discover_single), collapsing the platform-count branch onto the one proven digest-pinned path. Folded in SYFT_CHECK_FOR_APP_UPDATE=false to kill the anchore.io version-check egress-timeout warning in the same step. Cinched invariant honored: SBOM always generated for graft, no skip path. Commit 56b0465f6; shellcheck 222 clean. Verified green via a full onboarding-sequence gauntlet run (real cloud builds): 8 passed, 0 failed, 0 skipped; ordain_graft_demo's about+vouch build completed and the terminal abjure swept 3 about arks incl. the SBOM. Spec home RBSAB. Left a follow-up pace on the RBSAB/rbgja02 scan-scheme divergence surfaced during review.

### 2026-07-01 18:03 - ₢BoAAE - n

Fix single-arch SBOM failure in the about pipeline's syft step: single-platform branch scanned the bare image tag with no platform constraint, so syft fell back to its linux/amd64 default and died ('no child with platform linux/amd64') on an arm64-only image (local graft build on Apple Silicon). Fix by pinning the discovered platform via @digest, unifying single- and multi-platform scans onto the one proven digest-pinned path (platform_digests.txt is always written by discover-platforms step 01, single-platform included; verified in rbgja01-discover-platforms.py _discover_single). Dropped the now-unused PLATFORM_COUNT read (file-presence guard retained). Folded in SYFT_CHECK_FOR_APP_UPDATE=false to kill the anchore.io version-check egress timeout warning in the same step. Cinched invariant honored: SBOM always generated for graft, no skip path. Spec home RBSAB. Shellcheck 222 clean.

### 2026-07-01 16:37 - ₢BoAAD - W

Renamed the RBCC_account_* family to RBCC_account_unhewn_* (all five: governor/retriever/director/payor/mason), recording 'bare on purpose' in the name itself. The unhewn infix marks these as codegen-source SA-id/secret-dir leaves that must stay bare (RFC1035 forbids the sprue underscore in an SA-id) and must never be consolidated into the sprued mantle class. rbcc_emit_consts gained a single family-agnostic strip (${z_stem/unhewn_/}) so the projected RBTDGC_ACCOUNT_* names stay byte-identical -- proven by an EMPTY diff on the generated rbtdgc_consts.rs, so zero Rust consumers changed. Repointed all ~15 consumer sites across 6 bash files (fact-ext roster refs, rbgp mason-name/mason-email/governor-unmake-glob, rbdc+rblm payor secret-dir, rbgd mason email, rbgv mantle->account map); the collision family RBCC_account_mantle_* (rbma- SA-name fragments) deliberately untouched and now sits as a clean sibling. Operator settled account_unhewn_ over RBCC_unhewn_ (the latter would strip to RBTDGC_GOVERNOR, forfeiting the zero-Rust-change payoff). Silks relabeled from the stale account-localpart-mantle-token-converge (the dropped converge intent). Verification: build regenerates clean, shellcheck 222 clean, reveille 119/0/0. Picket 137/1 -- the one failure (rbtdrv_hallmark_lifecycle post-abjure summon) exonerated as a live-cloud network flake: the buried root cause was curl exit 28 (timeout) in the preceding rekon step, the failing path references none of the renamed constants, and band 110 (RBTDGC_BAND_VACANT) is unchanged. Rebased onto 4 upstream commits (none touching the 6 files); rename verified intact post-rebase.

### 2026-07-01 16:16 - ₢BoAAD - n

Rename RBCC_account_* -> RBCC_account_unhewn_* to record 'bare on purpose' in the name itself. The unhewn infix marks these five fragments (governor/retriever/director/payor/mason) as codegen-source SA-id/secret-dir leaves that must stay bare (RFC1035 forbids the sprue underscore in an SA-id) and must never be consolidated into the sprued mantle class. rbcc_emit_consts gains a single family-agnostic strip (z_stem/unhewn_/) so the projected RBTDGC_ACCOUNT_* names are byte-identical -- build confirms an empty diff on the generated rbtdgc_consts.rs, so zero Rust consumers change. Repointed all consumers: fact-ext roster refs, rbgp mason-name/mason-email/governor-unmake-glob, rbdc+rblm payor secret-dir, rbgd mason email, rbgv mantle->account map. The collision family RBCC_account_mantle_* (rbma- SA-name fragments) deliberately untouched -- it now sits as a clean sibling of account_unhewn_*. Emit doc comment updated: two mechanical name transforms, not one. Shellcheck 222 clean.

### 2026-07-01 16:16 - Heat - T

account-unhewn-marker-rename

### 2026-07-01 14:42 - ₢BoAAC - W

Minted the pallium-sprued mantle-identity-token home (RBCC_mantle_{governor,director,retriever}="rbpa_<role>") and killed the account-fragment-as-mantle conflation on the credential-mint path via a full sweep to one form (no two-form): emitted to RBTDGC_MANTLE_*; rba_don_capture resolves the sprued token to its rbma-<role> SA fragment; rba_token_capture + rbgv_check_mantle validate the sprued form; rbgv_check_mantle derives the bare polity name only for human display + the rbw-pB brevet remediation (brevet stays bare — deferred surface). 52 call sites swept (48 rba_token_capture + 4 payor rba_don_capture). Rust: depot brevet_don_impl split into bare-polity + sprued-token params; patrol heal-loop + denial don repointed ACCOUNT->MANTLE, BREVET/UNSEAT_POLITY kept bare. Extended the mantle-denial fixture into the leave-one-out isolation matrix: retriever withheld -> denied at BUBC_band_admission (109), while governor+director still reach AR (per-mantle isolation), governor pinned as the always-held wielder. Operator chose full-sweep over a translation-boundary at the resolver fork. Validated: build + shellcheck (222 clean), reveille 119/119, payor OAuth probe pass, mantle-denial isolation matrix pass live (dons used sprued tokens, hit HTTP 200 at AR; denial remediation correctly showed the bare 'retriever' for rbw-pB; freehold restored as found). Deferred as cinched: terrier/polity bare-mantle migration, payor isolation.

### 2026-07-01 14:26 - ₢BoAAC - n

Fix set -u unbound-variable crash in zrbgv_furnish's BUZ_FOLIO doc line: the doc block runs for every rbgv command (check_payor/check_avowal/check_mantle) BEFORE rbcc is sourced, so the ${RBCC_mantle_governor} interpolation I added was a fatal unbound reference (payor probe exited 1, would have hit check_mantle too). Revert to literal sprued values (rbpa_governor|...) like the zipper description — a pre-source doc mirror must be literal; the RBCC_mantle_* home stays authoritative. Caught by picket: the payor access-probe fixture died in furnish, aborting the suite before mantle-denial ran.

### 2026-07-01 14:20 - ₢BoAAC - n

Mint the pallium-sprued mantle-identity-token home and kill the account-fragment-as-mantle conflation on the credential-mint path. RBCC_mantle_{governor,director,retriever}="rbpa_<role>" is THE canonical token, emitted to RBTDGC_MANTLE_* via rbcc_emit_consts (single-prefix NAME, sprued VALUE, the RBTDGC_MOORINGS_DIR shape). Full sweep to one form, no two-form: rba_don_capture resolves the sprued token to its rbma-<role> SA fragment, rba_token_capture validates it (+zrbcc_sentinel), rbgv_check_mantle validates the sprued folio and derives the bare polity name only for the human display + rbw-pB brevet remediation (still bare, deferred surface). 52 call sites swept to ${RBCC_mantle_*} (48 rba_token_capture + 4 payor rba_don_capture). Rust: depot brevet_don_impl split into bare-polity + sprued-token params; patrol heal-loop and denial don repointed ACCOUNT->MANTLE, BREVET/UNSEAT_POLITY kept bare. Extended mantle-denial into the leave-one-out isolation matrix: with retriever withheld at the admission band, assert governor+director still reach AR (per-mantle isolation), governor pinned as always-held wielder. Build + shellcheck (222 clean); tests next.

### 2026-07-01 13:49 - ₢BoAAB - W

Minted BUBC_band_admission=109 for mantle-admission denial. rba_don_capture's Leg-3 403 arm returns the admission band (fanned to all five consumers via the buc_die band membrane by design); rbgv_check_mantle buc_rejects it with the brevet remediation while lapsed-sitting/network/other-HTTP stay buc_die. Added the picket-tier mantle-denial theurge fixture (picket+echelon) proving deny-then-restore on the real freehold retriever with an exact-band poll — live run converged to exit 109 at ~90s of IAM revocation propagation and restored to 0 immediately, leaving the subject as found. Projected RBTDGC_BAND_ADMISSION via rbcc_emit_consts; applied five 3Be memo follow-ups (BCG usage-refusal carve-out, keyfile-ghost guard comment, rbtdrs pre-rename suite names, rbtdrm podvm comment, BBAA9 comment reword). reveille 119/119 + the new fixture green live against the standing depot. All work in commit 0ce64b0c4 (pushed).

### 2026-07-01 13:41 - ₢BoAAB - n

Mint BUBC_band_admission=109 for mantle-admission denial; rba_don_capture's Leg-3 403 arm returns the admission band (the buc_die band membrane fans the code to all five consumers by design); rbgv_check_mantle buc_rejects it with the brevet remediation, infra failures stay buc_die. Add the picket-tier mantle-denial theurge fixture (deny-then-restore on the real freehold retriever, exact-band poll, self-skip on payor-unreachable) in picket+echelon. Project RBTDGC_BAND_ADMISSION via rbcc_emit_consts; rebuild regenerates rbtdgc_consts.rs. Apply five 3Be review follow-ups (BCG usage-refusal carve-out, keyfile-ghost guard comment, rbtdrs pre-rename suite names, rbtdrm podvm comment, BBAA9 comment reword). Build + shellcheck green; tests deferred to Phase 2.

### 2026-07-01 13:15 - Heat - d

batch: 1 reslate

### 2026-07-01 13:12 - Heat - d

batch: 1 reslate

### 2026-06-30 14:38 - ₢BoAAA - W

Dispositioned the open federation tendrils from the 260627 session. The drift cluster (T3 descry config-match, T5 drift fix, T6 rotation-coupling) plus a new architecture finding (pool granularity) graduated into a ₣Bf model rework: the per-foedus-pool topology was reversed to a one-pool identity substrate (a foedus is a PROVIDER under one manor pool). Landed: ₣Bf paddock rewritten to the new model; a contract-first spec-recast pace slated (RBSRF/RBSMA/RBSFD); the manor-finisher pace expanded (one-time pool founding + terrier provider-dimension + migration); the foedus-pool-state-classifier pace dropped as moot; ₣Bl canvass build/test re-cut workforcePools.list → providers.list; the reversal drained to the heat-memories memo. T7/T8/T9 absorbed (barrier already lifted; terrier-sequencing folded into the finisher; canvass→rbef_ mapping into the canvass re-cut). T4 (AXLA opinion-voicing) and T10 ('proof' word remint) declined as thin — resurrectable from this catalog's git history.

### 2026-06-30 12:46 - Heat - f

racing

### 2026-06-30 12:42 - Heat - S

syft-single-arch-sbom-fix

### 2026-06-30 12:34 - Heat - d

batch: 2 reslate

### 2026-06-30 12:09 - Heat - d

batch: 1 reslate

### 2026-06-30 11:53 - Heat - S

account-localpart-mantle-token-converge

### 2026-06-30 11:53 - Heat - S

mantle-token-home-and-isolation-matrix

### 2026-06-30 11:23 - Heat - S

mantle-admission-denial-band

### 2026-06-27 14:10 - Heat - S

federation-open-tendrils-catalog

### 2026-06-27 14:10 - Heat - N

rbk-05-federation-itches

