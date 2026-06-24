# Memo — Jailer shellcheck coverage: finding classification and risk assessment

Date: 2026-06-24
Heat: ₣Bi (rbk-14-mvp-loose-ends)
Pace: ₢BiAAD (shellcheck-cover-moorings-and-jailer)

This memo records the jailer risk assessment the ₢BiAAD docket requires before
landing edits to the security-boundary scripts, plus the discovery and mechanism
decisions made at mount. Provenance only — the durable facts (what the gate
covers, why) live in the code and its comments; this is the reasoning trail.

## Goal recap

Bring the load-bearing bash outside `Tools/` to shellcheck-clean, then widen the
release-tier gate (`rbq_qualify_shellcheck`) past its `Tools/`-only root so it
stays clean by construction. The release-tier gate is the single chokepoint:
`rbw-tl`, `rbw-tr`, and marshal-zero (`rblm_cli.sh`) all route through
`rbq_qualify_shellcheck`, so widening that one function widens all three.

## Discovery — 40 files, curated to a covered set

`find . -name '*.sh' -not -path './Tools/*' -not -path './tt/*' -not -path
'./.git/*' -not -path '*/target/*'` yields 40 files. The covered set and the
rationale for every exclusion:

Covered (load-bearing, shipped/run by the system):
- `rbmm_moorings/rbmv_vessels/common-sentry-context/rbjs_sentry.sh` — sentry jailer
- `rbmm_moorings/rbmv_vessels/common-sentry-context/rbjp_pentacle.sh` — pentacle jailer
- `rbmm_moorings/*/rbnnh_post_charge.sh` — per-nameplate charge hooks (globbed;
  one today, srjcl). Glob, not literal, because these recur per nameplate by
  construction.

Excluded, with reason:
- `rbmm_moorings/rbml_launchers/launcher.*.sh` — trivial trampoline shims (the
  release-tier gate already excludes this class).
- `Study/**` — scratch (already excluded class).
- `rbmm_moorings/fdkyclk/fdkyclk-{proof,teardown}.sh` — POC drivers tied to an
  active heat; labeled POC, scratch-class.
- `rbmm_moorings/rbmv_vessels/rbev-bottle-ccyolo/{build-context/entrypoint.sh,
  workspace/count_words.sh}` — example-vessel content (the `rbev-*` vessels are
  reference/illustrative, like `Study/`). `entrypoint.sh` is the one arguable
  add (it genuinely runs as a container entrypoint); held out as example-vessel
  code, trivially foldable in later if wanted.
- `MBS.STATION-reference.sh` — DELETED (see below), not covered.

## Mechanism — explicit extra-file list, not directory-root

`buq_shellcheck` was single-root (`find <dir>`). The covered files are
intermixed with files that must stay excluded — the launcher shims live under
the same `rbmm_moorings/` parent as the jailers — so a directory-root widening
would sweep in the excludes. The fix: `buq_shellcheck` gained a trailing
explicit extra-file parameter (args beyond the third are validated load-bearing
`.sh` files lints alongside the tree; each must exist, so a moved/renamed extra
fails loud rather than silently dropping coverage). `rbq_qualify_shellcheck`
enumerates the set. (Landed in commit 3b794bc88.)

## MBS.STATION-reference.sh — deleted (vestigial)

The docket listed this as load-bearing, but it is not: no consumer anywhere in
the repo, and it was explicitly `git rm`'d during release prep
(`rbk-prep-release.md` §9e). A bilingual bash/make config fragment from the
legacy MBC era, never sourced or included by current code. Per operator ruling
(2026-06-24) it was cut and its release-prep reference removed, rather than given
a shebang and covered. (Same commit 3b794bc88.)

## Finding classification — 34 jailer findings

The first widened gate run surfaced 34 findings in the two jailer scripts (MBS's
single SC2148 was resolved by deletion). The jailers are JDG-dialect POSIX `sh`
that runs inside the crucible security envelope (iptables, dnsmasq, enclave
network), so each finding is classified behavior-neutral vs behavior-affecting
before any edit.

Constraint shaping the fixes: `busc_shellcheckrc` policy forbids inline
shellcheck directives anywhere ("All structural suppressions live here. Genuine
findings must be fixed."). So no `# shellcheck disable=` escape — a deliberate
construct that trips a check must be *rewritten* to be clean while preserving
behavior, never suppressed in place.

### Behavior-neutral — direct fix (32)

- **SC2162 `read` without `-r` (4):** sentry lines 37, 42, 59; pentacle line 38.
  All read `ip`-command output or an IPv4 string (interface names, octets) —
  data that never contains backslashes, so `-r` (the safer form) changes nothing.
- **SC2086 unquoted interface name (28):** sentry — 26 occurrences of
  `${RBJ_ENCLAVE_IF}` / `${RBJ_UPLINK_IF}` in `iptables`/`/proc` commands;
  pentacle line 44 — `${RBJP_ENCLAVE_IF}` ×2. Each is a single-token interface
  name (`eth0`-class, field 2 of `ip` output), so quoting a value with no spaces
  or globs is neutral. The sentry already quotes `${RBRN_*}` IPs elsewhere; the
  interface vars were just inconsistent.

### Behavior-affecting — rewrite to preserve (2)

- **SC2086 in the RBJp0 quoting probe — sentry lines 14, 15.** The block
  validates that `RBJE_PROBE` arrives as exactly `alpha bravo` (proving compose
  env-file quoting transported a space-containing value). Lines 14–15 do
  `test "$(echo ${RBJE_PROBE} | cut -d' ' -f1)" = "alpha"` etc. The unquoted
  `echo $RBJE_PROBE` is *deliberate whitespace tokenization* — quoting it would
  change the result for multi-space input (the `for`-loop count above already
  accepts any whitespace width). Since inline suppression is barred, the fix is
  a behavior-preserving rewrite: capture the first two tokens in the existing
  `for z_word in ${RBJE_PROBE}` loop (which is already shellcheck-clean — `for
  ... in $var` is a recognized safe split) and compare those, dropping the
  cut-based lines entirely. Validation is identical (count==2, first==alpha,
  second==bravo) and the same failure modes fail identically.

## Gating — siege before and after

The docket gates any sentry/pentacle edit on tadmor-tier crucible re-testing.
The chosen gate is `siege` (`tt/rbw-ts.TestSuite.siege.sh` — `kludge-tadmor` +
`tadmor` security cases, fully local; *not* "barricade", which does not exist —
the airgap sibling is `blockade`).

- **Pre-change baseline (pristine jailer):** GREEN — siege 62 passed, 0 failed,
  0 skipped (kludge-tadmor + tadmor's 61 cases), crucible quenched clean. Run on
  commit 3b794bc88 (jailer pristine), 2026-06-24.
- **Post-change (jailer fixed):** GREEN — siege 62 passed, 0 failed, 0 skipped
  (identical to baseline), commit 1111f5901. rbw-tl green (217 files clean).
  2026-06-24.
- **Cloud-build verification (operator-requested):** CORRECTION (operator,
  2026-06-24): tadmor is NOT cloud-built — it is local-only (kludge), so the
  attempted `rbw-tO.OrdainCycle.tadmor` was the wrong target (it failed fast at
  the quota preflight, before building — see Spook below). The cloud/airgap
  *sentry* path is **moriah** (blockade suite); the cloud-pipeline smoke test is
  **dogfight** (ordain→summon→run on busybox over the freehold depot). The jailer
  edits are already behavior-proven by the local siege above; dogfight confirms
  the cloud build pipeline is intact, and the cloud-built *sentry* is reached only
  via moriah/blockade (not requested now). dogfight ordains via `rbw-fO`
  (rbfd_cli, which sources rbuh) so it is unaffected by the Spook bug. RESULT
  PENDING (dogfight running).

## Spook — latent bug in the rbob_ordain (`rbw-tO` OrdainCycle) path

Surfaced while (mistakenly) running `rbw-tO.OrdainCycle.tadmor`: the ordain
reaches `rbfd_director.sh:200`'s concurrent-build-quota preflight, which calls
`rbuh_json`, but `rbob_cli.sh`'s furnish sources `rbfd_director.sh` (line 134)
WITHOUT sourcing `rbuh_http.sh` — so `rbuh_json` is command-not-found (exit 127).
The sibling `rbw-fO` path (rbfd_cli.sh) sources+kindles rbuh correctly, so only
the `rbob_ordain` (`rbw-tO`) entry is broken. Pre-existing, unrelated to the
jailer work, and not in this pace's scope. Likely fix: source + kindle
rbuh_http.sh in `rbob_cli.sh` furnish beside the `rbfd_director.sh` source.
Candidate itch / separate pace.

## nsproto lineage — what "cloud build and run nsproto" refers to

`nsproto` ("Prototype Namespace") was the original network-namespace
security-isolation prototype, since fully morphed into today's
tadmor/ifrit/theurge stack. Confirmed via git archaeology (2026-06-24):

- **nsproto era (bash):** nameplate `nsproto` (sentry + bottle + censer) with a
  bash security suite `nsproto-security` — 22 cases
  (`tt/rbtb-ns.TestNsproSecurity.nsproto.sh`), plus hand-bash tabtargets
  (Start/Stop, ConnectBottle/Sentry/Censer, ObserveNetworks) and
  `nsproto-{bottle,sentry}-namespace-setup.sh`. The feeding netns experiments
  survive in `Study/study-net-namespace-permutes/`.
- **Heat ₣Ay (rbk-mvp-4-tadmor-ifrit-creation):** sentry slimmed to Debian
  bookworm-slim, socat-proxy → iptables DNAT (₢AyAAA — whose message records
  "22/22 nsproto security tests on Cloud Build image", the literal source of the
  "cloud build and run nsproto" memory); ifrit attack vessel created
  (rbev-bottle-ubuntu-test → rbev-bottle-ifrit, ₢AyAAD); `nsproto` nameplate
  git-mv'd to `tadmor`, `nsproto-security` → `tadmor-security` (₢AyAAE).
- **Heat ₣A1:** bash tadmor-security retired; all 22 cases converted to the
  theurge crucible (₢A1AAF). These are today's `rbtdrc_*` cases (the 61 now run
  by siege — the suite has grown past the original 22).

So "cloud build and run nsproto" = run the tadmor security crucible against
cloud-built (ordained) sentry+ifrit images — the Cloud Build counterpart of the
local siege. siege covers the kludge build; the cloud run proves the same jailer
edits in the release build path.

## Plan

1. siege baseline (pristine jailer) — commit 3b794bc88 is baseline-prep.
2. This memo (operator review checkpoint).
3. Jailer fixes (32 neutral + 2 rewrites) + extend `rbq` extras coverage; commit.
4. siege re-run (post-change) — prove containment intact.
5. `rbw-tl`/`rbw-tr` green.
6. Cloud-build + run `nsproto` (target TBD).
