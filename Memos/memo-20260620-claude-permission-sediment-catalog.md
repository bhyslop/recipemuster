# Permission-Rule Sediment Catalog — Claude Code allow-lists across recipemuster clones

**Date:** 2026-06-20
**Author:** Claude (Opus 4.8), with Brad
**Status:** provenance capture — mine, then discard the source piles

## Why this memo exists

Claude Code appends a `permissions.allow` rule every time you click "always allow."
Across the recipemuster clones these grew into large, uncurated piles. This memo
captures them before we delete them, for two uses:

1. **Mining for MCP / Job-Jockey absorption.** Many entries are repetitive shell
   operations (git, project workflows) that would be better expressed as MCP verbs.
   Once an operation lives behind `mcp__vvx__jjx` (or another MCP tool), a *single*
   MCP approval covers it — collapsing dozens of per-command Bash grants to one.
   Read the catalog with that lens: any command-family you run constantly is a
   candidate to become, or route through, an MCP tool.
2. **Backup before reset.** The live `settings.local.json` piles are git-ignored
   (no git history). This memo is their durable record; after it lands we reset the
   piles and replace them with a small curated, version-controlled shared list.

This is a **catalog, not authority** (per the memo discipline): nothing here is
load-bearing. Resurrect a command into a real home (an MCP verb, or the curated
allow-list) deliberately — never treat the pile as a spec.

## Assessment (the "trash pile" finding)

The piles are *sediment, not policy* — snapshots of exact invocations rather than
reusable permissions. Signatures:

- **100% distinct** within each clone — nothing is reused; every entry is a frozen
  one-time command.
- **Trivial commands frozen by the hundred** — alpha alone has ~192 distinct `echo`,
  ~156 `sed`, ~56 `find` rules. Each should be one broad rule, or none.
- **Self-defeating grants** — alpha's `bash:*` and `sh -c *` already permit arbitrary
  code, making the other ~1,838 specific rules theater and silently disabling the
  "ask before unusual command" gate.
- **Risky over-grants buried inside** — a `sudo` cluster (route/arp/ipconfig/tee/pkill),
  `rm -rf <abs path>`, bare interpreter grants (`python3:*`, `perl:*`, `chmod:*`).
  Explicitly **not** carried forward (full list in the catalog).
- **Redundancy** — `./x` vs `x` duplicate pairs (57 in alpha, 11 in beta); `/tmp/`
  one-offs (45 / 23).
- **Divergent per clone** — each machine grew its own pile (no shared baseline), which
  is exactly why the replacement is a single committed `settings.json`.

Precise per-clone counts are in the catalog's first table.

## How to mine this (the MCP / JJ lens)

Sort catalog entries into three buckets:

- **MCP-absorption** — recurring *operations* (git status/diff/log, `gh`, repetitive
  project queries) better expressed as MCP verbs → one approval instead of N Bash
  grants. The "MCP / Job-Jockey absorption candidates" section is the starting point.
- **Curated broad allow** — genuinely-routine safe shell (echo/ls/cat/grep/find/jq,
  read-only git, the `tt/` tabtargets) → a few broad rules in the shared `settings.json`.
- **Discard** — `/tmp` one-offs, exact-argument snapshots, risky grants, heredoc junk.
  Let these re-prompt; that is the gate working.

## What happens next (recorded for provenance)

1. This memo captures the piles (catalog below).
2. The three `settings.local.json` allow-lists are reset to empty (backed up to
   `/tmp/rb-sediment-backup-*.json` at reset time).
3. A small curated, broad-but-safe allow-list is emplaced in the **tracked**
   `.claude/settings.json` (beta), to propagate fleet-wide on pull.
4. The alpha multi-line heredoc entries are dropped without preserving their report
   bodies (one-off `/tmp` writes — listed by target filename only in the catalog).

---

## Sediment catalog (generated)

<!-- catalog body appended by tooling on 2026-06-20 -->
### Per-clone counts

| clone | allow total | Bash (well-formed) | non-Bash | malformed |
|---|---|---|---|---|
| alpha | 1892 | 1840 | 44 | 8 |
| beta | 553 | 514 | 39 | 0 |
| cerebro-alpha | 93 | 84 | 9 | 0 |

### Union — Bash commands by clone coverage (count = how many clones have it; 3 = core)
```
   3 sort -t: -k2 -rn
   3 shellcheck --version
   3 GIT_EDITOR=true git rebase --continue
   3 echo "=== EXIT: $? ==="
   3 echo "=== exit: $? ==="
   3 command -v shellcheck
   3 ./tt/rbw-ts.TestSuite.dogfight.sh *
   3 ./tt/rbw-tl.Shellcheck.sh *
   2 tt/vow-t.Test.sh *
   2 tt/rbw-ts.TestSuite.fast.sh *
   2 tt/rbw-tP.QualifyPristine.sh *
   2 tt/rbw-tl.Shellcheck.sh
   2 tt/rbw-tb.Build.sh *
   2 tt/rbw-rrv.ValidateRepoRegime.sh
   2 tt/rbw-rfv.ValidateFederationRegime.sh
   2 tt/rbw-MG.MarshalGenerate.sh
   2 tt/rbw-ld.DirectorDivinesLodes.sh
   2 tt/rbw-acg.CheckGovernorCredential.sh
   2 tt/rbw-acf.CheckFederatedAccess.sh
   2 tt/rbtd-t.Test.sh
   2 tt/rbtd-s.TestSuite.fast.sh
   2 tt/rbtd-b.Build.sh
   2 tt/buw-rsv.ValidateStationRegime.sh
   2 ssh *
   2 sort -t: -k2 -n -r
   2 rustc --version
   2 mv ../station-files/secrets/assay/rbra.env ../station-files/secrets/director/rbra.env
   2 git *
   2 gcloud --version
   2 echo "TEST_EXIT=$?"
   2 echo "SHELLCHECK_EXIT=$?"
   2 echo "FIXTURE_EXIT=$?"
   2 echo "FAST_EXIT=$?"
   2 echo "EXIT=$?"
   2 echo "exit=$?"
   2 echo "exit:$?"
   2 echo "EXIT: $?"
   2 echo "EXIT_CODE=$?"
   2 echo "CHARGE_EXIT=$?"
   2 echo "BUILD_EXIT=$?"
   2 echo "=== rebase exit: $? ==="
   2 echo "=== BUILD EXIT: $? ==="
   2 echo "---exit:$?---"
   2 echo "---EXIT $?---"
   2 echo "--- exit: $? ---"
   2 echo "--- exit $? ---"
   2 docker info *
   2 docker context *
   2 command -v ruby
   2 command -v docker
   2 command -v asciidoctor
   2 cd *
   2 cat
   2 BURE_CONFIRM=skip tt/rbw-MZ.MarshalZeroes.sh *
   2 asciidoctor --version
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gPR.PayorRefresh.sh
   2 ./tt/vow-t.Test.sh *
   2 ./tt/vow-b.Build.sh *
   2 ./tt/rbw-tt.Test.sh
   2 ./tt/rbw-tq.QualifyFast.sh *
   2 ./tt/rbw-tP.QualifyPristine.sh *
   2 ./tt/rbw-tf.FixtureRun.sh wsl-lifecycle *
   2 ./tt/rbw-tf.FixtureRun.sh reliquary-lifecycle *
   2 ./tt/rbw-tf.FixtureRun.sh lode-lifecycle *
   2 ./tt/rbw-tf.FixtureRun.sh foundry-path *
   2 ./tt/rbw-tf.FixtureRun.sh cupel *
   2 ./tt/rbw-tf.FixtureRun.sh canonical-invest *
   2 ./tt/rbw-tb.Build.sh *
   2 ./tt/rbw-rdr.RenderDepotRegime.sh
   2 ./tt/rbw-MZ.MarshalZeroes.sh *
   2 ./tt/rbw-dl.PayorListsDepots.sh *
   2 ./tt/rbw-cKS.KludgeSentry.sh tadmor *
   2 ./tt/rbw-aM.PayorMantlesGovernor.sh *
   2 ./tt/rbtd-b.Build.sh
   2 ./tt/buw-rpv.ValidatePrivilegeRegime.sh bujn-winpc *
   1 xxd /Users/bhyslop/projects/temp-buk/temp-20260506-135454-80214-889/bujb_wsl_preflight_stdout.txt
   1 xxd /tmp/utf8.bin
   1 xxd /tmp/psutf8.bin
   1 xxd /tmp/direct.bin
   1 xxd ../logs-buk/last.txt
   1 xxd
   1 xargs shellcheck --rcfile=Tools/buk/busc_shellcheckrc -S style -f gcc
   1 xargs sed -n '1,30p'
   1 xargs ls -lt
   1 xargs ls -la
   1 xargs cat:*
   1 xargs cat
   1 xargs basename *
   1 xargs '-I{}' sh -c 'size=$\(git -C /Users/bhyslop/projects/rbm_alpha_recipemuster show "{}:.claude/jjm/jjg_gallops.json" 2>/dev/null | wc -c\); echo "$size {}"'
   1 xargs -n1 basename
   1 xargs -I{} sh -c 'test -f "{}rbrn.env" && echo "{}"'
   1 xargs -I{} git log -1 --format="%H %s" {}
   1 xargs -I {} sh -c 'echo "=== {} ==="; grep -E "source.*buh_handbook|source.*buym_yelp" "{}"'
   1 xargs -I {} grep -l '^export BURD_INTERACTIVE=1$' {}
   1 xargs -I {} basename {}
   1 xargs -0 shellcheck --rcfile=Tools/buk/busc_shellcheckrc -S style -f gcc
   1 whois 93.184.216.34
   1 while read:*
   1 which syft:*
   1 which shellcheck:*
   1 wc:*
   1 wc -l Tools/jjk/vov_veiled/src/jjrg_gallops.rs Tools/jjk/vov_veiled/src/jjrt_types.rs Tools/jjk/vov_veiled/src/jjri_io.rs
   1 wc -l echo echo 'Total matches found:' grep -r "lemma\\|lemmata\\|graven\\|intaglio\\|quoin\\|sprue\\|inlay" /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/cmk --include=*.adoc
   1 wc -l /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk/vov_veiled/src/*.rs
   1 wc -l /Users/bhyslop/projects/rbm_alpha_recipemuster/GREP*
   1 wc -l .claude/jjm/jjg_gallops.json
   1 wc -l __NEW_LINE_7157110d9076c725__ echo echo '6. AT_ SERVICE TERM REFERENCES' echo 'Total at_bottle_service/at_censer_container/at_agile_service/at_sessile_service:' grep -r "at_bottle_service\\|at_censer_container\\|at_agile_service\\|at_sessile_service" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.json
   1 wc -l __NEW_LINE_7157110d9076c725__ echo echo '5. OP*_ TERM REFERENCES' echo 'Total opbs_/opbr_/opss_ references:' grep -r "opbs_\\|opbr_\\|opss_" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.json
   1 wc -l __NEW_LINE_7157110d9076c725__ echo echo '3. FRONTISPIECE REFERENCES' echo 'Total ConnectBottle/ConnectCenser/ConnectSentry/ObserveNetworks:' grep -r "ConnectBottle\\|ConnectCenser\\|ConnectSentry\\|ObserveNetworks" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.yml --include=*.yaml --include=*.html
   1 wc -l __NEW_LINE_7157110d9076c725__ echo echo '2. BOTTLE_START / BOTTLE_RUN / BOTTLE_SERVICE REFERENCES' echo 'Total bottl_start/run/service:' grep -r "bottle_start\\|bottle_run\\|bottle_service" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.yml --include=*.yaml --include=*.html --include=*.json
   1 wait
   1 vm_stat
   1 unzip -p /tmp/gaz2020.zip
   1 unzip -p /tmp/gaz_test.zip
   1 unzip -o names.zip Names_2010Census.csv
   1 unzip -l names.zip
   1 unzip -l /tmp/gaz_test.zip
   1 umask 077
   1 tt/vvw-r.RunVVX.sh jjx:*
   1 tt/vvw-r.RunVVX.sh jjx_show '{}' 2>&1
   1 tt/vvw-r.RunVVX.sh jjx_open 2>&1
   1 tt/vvw-r.RunVVX.sh --help 2>&1
   1 tt/vow-t.Test.sh 2>&1
   1 tt/vow-t.Test.sh
   1 tt/vow-b.Build.sh:*
   1 tt/vow-b.Build.sh *
   1 tt/vow-b.Build.sh
   1 tt/rbw-z.Stop.srjcl.sh:*
   1 tt/rbw-z.Stop.pluml.sh:*
   1 tt/rbw-z.Stop.nsproto.sh:*
   1 tt/rbw-tt.Test.sh *
   1 tt/rbw-tt.Test.sh
   1 tt/rbw-ts.TestSuite.dogfight.sh
   1 tt/rbw-tq.QualifyFast.sh *
   1 tt/rbw-tn.TestNameplate.srjcl.sh:*
   1 tt/rbw-tn.TestNameplate.pluml.sh:*
   1 tt/rbw-tn.TestNameplate.nsproto.sh:*
   1 tt/rbw-Tk.KludgeCycle.tadmor.sh 2>&1
   1 tt/rbw-tK.KludgeCycle.tadmor.sh *
   1 tt/rbw-tf.TestFixture.regime-validation.sh 2>&1
   1 tt/rbw-tf.TestFixture.regime-smoke.sh 2>&1
   1 tt/rbw-tf.TestFixture.regime-credentials.sh 2>&1
   1 tt/rbw-tf.TestFixture.qualify-all.sh 2>&1
   1 tt/rbw-tf.TestFixture.nsproto-security.sh 2>&1
   1 tt/rbw-tf.TestFixture.kick-tires.sh 2>&1
   1 tt/rbw-tf.TestFixture.enrollment-validation.sh 2>&1
   1 tt/rbw-tf.TestFixture.ark-lifecycle.sh 2>&1
   1 tt/rbw-tf.TestFixture.access-probe.sh 2>&1
   1 tt/rbw-tf.QualifyFast.sh
   1 tt/rbw-tf.FixtureRun.sh regime-poison *
   1 tt/rbw-tf.FixtureRun.sh podvm-lifecycle *
   1 tt/rbw-tf.FixtureRun.sh lode-lifecycle *
   1 tt/rbw-tf.FixtureRun.sh foedus-lifecycle *
   1 tt/rbw-tf.FixtureRun.sh foedus-freehold *
   1 tt/rbw-tc.FixtureCase.sh
   1 tt/rbw-ta.TestAll.sh:*
   1 tt/rbw-s.Start.pluml.sh
   1 tt/rbw-s.Start.nsproto.sh:*
   1 tt/rbw-rvv.ValidateVesselRegime.sh rbev-sentry-ubuntu-large:*
   1 tt/rbw-rvv.ValidateVesselRegime.sh rbev-graft-demo *
   1 tt/rbw-rvv.ValidateVessel.sh rbev-sentry-ubuntu-large:*
   1 tt/rbw-rvl.ListVesselRegime.sh
   1 tt/rbw-rva.DirectorAnointsGraftVessel.sh
   1 tt/rbw-rsr.RenderStationRegime.sh
   1 tt/rbw-Rs.RetrieverSummonsArk.sh rbev-bottle-plantuml:*
   1 tt/rbw-rrr.RenderRepoRegime.sh
   1 tt/rbw-rov.ValidateOauthRegime.sh
   1 tt/rbw-rnr.RenderNameplateRegime.sh tadmor *
   1 tt/rbw-rnr.RenderNameplateRegime.sh nsproto:*
   1 tt/rbw-Rl.RetrieverListsImages.sh:*
   1 tt/rbw-RiF.RetrieverInspectsFull.sh rbev-bottle-plantuml:*
   1 tt/rbw-Ric.RetrieverInspectsCompact.sh rbev-sentry-ubuntu-large:*
   1 tt/rbw-Ric.RetrieverInspectsCompact.sh rbev-bottle-plantuml:*
   1 tt/rbw-rfr.RenderFederationRegime.sh
   1 tt/rbw-rdr.RenderDepotRegime.sh
   1 tt/rbw-rav.ValidateAuthRegime.sh rbnae_governor
   1 tt/rbw-rav.ValidateAuthRegime.sh governor *
   1 tt/rbw-pF.FreeholdProof.sh
   1 tt/rbw-PC.PayorCreatesDepot.sh depot10041:*
   1 tt/rbw-Op.OnboardingPayor.sh
   1 tt/rbw-Ofc.OnboardingFirstCrucible.sh:*
   1 tt/rbw-Odg.OnboardingDirectorGraft.sh
   1 tt/rbw-Odf.OnboardingDirectorFirstBuild.sh
   1 tt/rbw-Odb.OnboardingDirectorBind.sh
   1 tt/rbw-Oda.OnboardingDirectorAirgap.sh
   1 tt/rbw-Ocd.OnboardingCredentialDirector.sh
   1 tt/rbw-Occ.OnboardingCrashCourse.sh
   1 tt/rbw-Occ.OnboardingConfigureEnvironment.sh
   1 tt/rbw-o.OnboardingStartHere.sh
   1 tt/rbw-o.ONBOARDING.sh
   1 tt/rbw-ni.NameplateInfo.sh
   1 tt/rbw-MZ.MarshalZeroes.sh *
   1 tt/rbw-MZ.MarshalZeroes.sh
   1 tt/rbw-MR.MarshalReset.sh
   1 tt/rbw-MD.MarshalDuplicate.sh /Users/bhyslop/test-rbk-002
   1 tt/rbw-mA.PayorAffiancesManor.sh
   1 tt/rbw-LK.LocalKludge.sh 2>&1
   1 tt/rbw-lI.DirectorImmuresPodvm.sh podvm-wsl *
   1 tt/rbw-la.DirectorAugursLode.sh vw260610095327 *
   1 tt/rbw-irr.DirectorRekonsReliquary.sh r260425082412 *
   1 tt/rbw-il.DirectorListsRegistry.sh
   1 tt/rbw-iJr.DirectorJettisonsReliquaryImage.sh "reliquaries/r260425082412/skopeo:r260425082412" --force
   1 tt/rbw-iJe.DirectorJettisonsEnshrinement.sh "enshrines/busybox-latest-1487d0af5f:busybox-latest-1487d0af5f" --force
   1 tt/rbw-iJe.DirectorJettisonsEnshrinement.sh
   1 tt/rbw-iar.DirectorAuditsReliquaries.sh
   1 tt/rbw-iah.DirectorAuditsHallmarks.sh
   1 tt/rbw-iae.DirectorAuditsEnshrinements.sh
   1 tt/rbw-HWdd.DockerDesktop.sh
   1 tt/rbw-HWdc.DockerContextDiscipline.sh
   1 tt/rbw-hw.HandbookWindows.sh
   1 tt/rbw-hw
   1 tt/rbw-h0.HandbookTOP.sh
   1 tt/rbw-gq.QuotaBuild.sh
   1 tt/rbw-gPR.PayorRefresh.sh *
   1 tt/rbw-gPR.PayorRefresh.sh
   1 tt/rbw-gPo.PayorOnboarding.sh 2>&1
   1 tt/rbw-gPE.PayorEstablish.sh
   1 tt/rbw-gOR.OnboardRetriever.sh
   1 tt/rbw-go.OnboardMAIN.sh
   1 tt/rbw-gO.Onboarding.sh 2>&1
   1 tt/rbw-gO.Onboarding.sh
   1 tt/rbw-fs.RetrieverSummonsHallmark.sh rbev-sentry-deb-tether:*
   1 tt/rbw-fpf.RetrieverPlumbsFull.sh c260425094751-r260425164754 *
   1 tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-busybox
   1 tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-bottle-plantuml
   1 tt/rbw-fO.DirectorOrdainsHallmark.sh *
   1 tt/rbw-fk.LocalKludge.sh rbev-sentry-deb-tether:*
   1 tt/rbw-fk.LocalKludge.sh
   1 tt/rbw-dY.DirectorYokesReliquaryInVessel.sh somestamp *
   1 tt/rbw-dY.DirectorYokesReliquaryInVessel.sh rbev-sentry-deb-tether *
   1 tt/rbw-dY.DirectorYokesReliquaryInVessel.sh r260426100632 *
   1 tt/rbw-dY.DirectorYokesReliquaryInVessel.sh r260425082412 *
   1 tt/rbw-dY.DirectorYokesReliquaryInVessel.sh bogus-vessel *
   1 tt/rbw-dY.DirectorYokesReliquaryInVessel.sh --help
   1 tt/rbw-dY.DirectorYokesReliquaryInVessel.sh
   1 tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh r260503091950 *
   1 tt/rbw-DV.DirectorVouchesConsecrations.sh rbev-vessels/rbev-busybox c260317210019-r260318040308 2>&1
   1 tt/rbw-DV.DirectorVouchesConsecrations.sh rbev-vessels/rbev-busybox 2>&1
   1 tt/rbw-DV.DirectorVouchesConsecrations.sh
   1 tt/rbw-dU.PayorUnmakesDepot.sh depot10041 *
   1 tt/rbw-dU.PayorUnmakesDepot.sh
   1 tt/rbw-dt.TerrierScaffold.sh
   1 tt/rbw-DS.DirectorSummonsArk.sh rbev-sentry-ubuntu-large:*
   1 tt/rbw-DS.DirectorSummonsArk.sh rbev-bottle-ubuntu-test:*
   1 tt/rbw-DS.DirectorSummonsArk.sh rbev-bottle-plantuml:*
   1 tt/rbw-DS.DirectorSummonsArk.sh rbev-bottle-anthropic-jupyter:*
   1 tt/rbw-dr.DepotRecognosce.sh *
   1 tt/rbw-DO.DirectorOrdainsConsecration.sh rbev-bottle-ifrit:*
   1 tt/rbw-dl.PayorListsDepots.sh *
   1 tt/rbw-dl.PayorListsDepots.sh
   1 tt/rbw-DI.DirectorInscribesRubric.sh 2>&1
   1 tt/rbw-DI.DirectorInscribesReliquary.sh
   1 tt/rbw-dI.DirectorInscribesReliquary.sh
   1 tt/rbw-dE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-busybox
   1 tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-sentry-ubuntu-large:*
   1 tt/rbw-dE.DirectorEnshrinesVessel.sh *
   1 tt/rbw-DE.DirectorEnshrinesBaseImages.sh rbev-vessels/rbev-busybox 2>&1
   1 tt/rbw-DE.DirectorEnshrinesBaseImages.sh rbev-sentry-ubuntu-large:*
   1 tt/rbw-DC.DirectorCreatesConsecration.sh rbev-sentry-ubuntu-large:*
   1 tt/rbw-DC.DirectorCreatesConsecration.sh rbev-busybox:*
   1 tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-busybox 2>&1 | head -80
   1 tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-busybox 2>&1
   1 tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-bottle-plantuml
   1 tt/rbw-DC.DirectorConjuresArk.sh:*
   1 tt/rbw-Dc.DirectorChecksConsecrations.sh 2>&1
   1 tt/rbw-cw.Writ.tadmor.sh iptables *
   1 tt/rbw-cQ.Quench.tadmor.sh 2>&1
   1 tt/rbw-cQ.Quench.tadmor.sh
   1 tt/rbw-cQ.Quench.srjcl.sh 2>&1
   1 tt/rbw-cKS.KludgeSentry.sh tadmor *
   1 tt/rbw-cKB.KludgeBottle.tadmor.sh
   1 tt/rbw-cKB.KludgeBottle.sh tadmor:*
   1 tt/rbw-cic.CrucibleIsCharged.sh tadmor *
   1 tt/rbw-cC.Charge.tadmor.sh 2>&1
   1 tt/rbw-cC.Charge.tadmor.sh *
   1 tt/rbw-cC.Charge.srjcl.sh 2>&1
   1 tt/rbw-cC.Charge.srjcl.sh *
   1 tt/rbw-cb.Bark.tadmor.sh sh *
   1 tt/rbw-cb.Bark.tadmor.sh cat:*
   1 tt/rbw-arI.GovernorInvestsRetriever.sh ret *
   1 tt/rbw-aM.PayorMantlesGovernor.sh
   1 tt/rbw-adr.GovernorRostersDirectors.sh
   1 tt/rbw-adI.GovernorInvestsDirector.sh canest-dir *
   1 tt/rbw-acr.CheckRetrieverCredential.sh
   1 tt/rbw-acp.CheckPayorCredential.sh
   1 tt/rbw-acd.CheckDirectorCredential.sh *
   1 tt/rbw-acd.CheckDirectorCredential.sh
   1 tt/rbtd-s.TestSuite.service.sh
   1 tt/rbtd-s.TestSuite.dogfight.sh *
   1 tt/rbtd-s.TestSuite.crucible.sh 2>&1
   1 tt/rbtd-s.TestSuite.crucible.sh
   1 tt/rbtd-s.SingleCase.tadmor.sh sortie-proto-smuggle-rawsock:*
   1 tt/rbtd-s.SingleCase.tadmor.sh sortie-ns-capability-escape:*
   1 tt/rbtd-s.SingleCase.tadmor.sh sortie-net-srcip-spoof:*
   1 tt/rbtd-s.SingleCase.tadmor.sh sortie-icmp-exfil-payload:*
   1 tt/rbtd-s.SingleCase.srjcl.sh rbtdrc_srjcl_websocket_kernel
   1 tt/rbtd-s.SingleCase.regime-validation.sh rbtdrf_rv_rbrr_repo
   1 tt/rbtd-s.SingleCase.regime-validation.sh rbtdrf_rv_rbrn_all_nameplates
   1 tt/rbtd-s.SingleCase.pristine-lifecycle.sh rbtdrp_marshal_zero_attestation
   1 tt/rbtd-s.SingleCase.pristine-lifecycle.sh rbtdrp_governor_lifecycle
   1 tt/rbtd-s.SingleCase.pristine-lifecycle.sh rbtdrp_depot_lifecycle
   1 tt/rbtd-s.SingleCase.pristine-lifecycle.sh
   1 tt/rbtd-s.SingleCase.handbook-render.sh rbtdrf_hb_onboard_first_crucible
   1 tt/rbtd-s.SingleCase.four-mode.sh rbtdrc_fourmode_conjure_lifecycle *
   1 tt/rbtd-s.SingleCase.four-mode.sh rbtdrc_fourmode_bind_lifecycle
   1 tt/rbtd-s.SingleCase.four-mode.sh
   1 tt/rbtd-s.SingleCase.canonical-establish.sh
   1 tt/rbtd-s.FixtureCase.sh tadmor *
   1 tt/rbtd-s.FixtureCase.sh enrollment-validation *
   1 tt/rbtd-s.FixtureCase.sh bogus-fixture *
   1 tt/rbtd-s.FixtureCase.sh
   1 tt/rbtd-s.FixtureCase.regime-validation.sh rbtdrf_rv_rbrv_all_vessels
   1 tt/rbtd-s.FixtureCase.regime-smoke.sh rbtdrf_rs_unmake_empty_arg_refusal
   1 tt/rbtd-s.FixtureCase.regime-smoke.sh
   1 tt/rbtd-s.FixtureCase.onboarding-sequence.sh rbtdro_onboarding_ordain_airgap *
   1 tt/rbtd-s.FixtureCase.onboarding-sequence.sh
   1 tt/rbtd-s.FixtureCase.moriah.sh
   1 tt/rbtd-r.Run.tadmor.sh 2>&1
   1 tt/rbtd-r.Run.regime-validation.sh
   1 tt/rbtd-r.Run.regime-smoke.sh
   1 tt/rbtd-r.Run.pristine-lifecycle.sh
   1 tt/rbtd-r.Run.handbook-render.sh
   1 tt/rbtd-r.Run.four-mode.sh
   1 tt/rbtd-r.FixtureRun.srjcl.sh *
   1 tt/rbtd-r.FixtureRun.regime-validation.sh
   1 tt/rbtd-r.FixtureRun.regime-smoke.sh
   1 tt/rbtd-r.FixtureRun.onboarding-sequence.sh *
   1 tt/rbtd-r.FixtureRun.handbook-render.sh
   1 tt/rbtd-r.FixtureRun.hallmark-lifecycle.sh
   1 tt/rbtd-r.FixtureRun.enrollment-validation.sh
   1 tt/rbtd-b.Build.sh 2>&1
   1 tt/rbtd-ap.AccessProbe.payor.sh
   1 tt/jjw-tfS.TestFundusSingle.localhost.sh full::bind_send
   1 tt/jjw-tfs.TestFundusScenario.localhost.sh --help
   1 tt/jjw-tfs.TestFundusScenario.localhost.sh
   1 tt/jjw-tfs.TestFundusScenario.cerebro.sh
   1 tt/jjw-tfP2.ProvisionPhase2.localhost.sh 2>&1
   1 tt/jjw-tfP2.ProvisionPhase2.cerebro.sh 2>&1
   1 tt/jjw-tfP2.ProvisionPhase2.cerebro.sh ~/.ssh/id_ed25519.pub 2>&1
   1 tt/jjw-tfP.ProvisionFundusAccounts.localhost.sh
   1 tt/buw-tt-ll.ListLaunchers.sh 2>&1 | head -30
   1 tt/buw-tt-ll.ListLaunchers.sh
   1 tt/buw-tt-cbl.CreateTabTargetBatchLogging.sh *
   1 tt/buw-st.BukSelfTest.sh *
   1 tt/buw-st.BukSelfTest.sh
   1 tt/buw-rsv.ValidateStationRegime.sh foo *
   1 tt/buw-rsv.ValidateStationRegime.sh 2>&1 | head -30
   1 tt/buw-rpv.ValidatePrivilegeRegime.sh smoke-invest *
   1 tt/buw-rpv.ValidatePrivilegeRegime.sh
   1 tt/buw-rpr.RenderPrivilegeRegime.sh smoke-invest *
   1 tt/buw-rpr.RenderPrivilegeRegime.sh
   1 tt/buw-rpl.ListPrivilegeRegime.sh foo *
   1 tt/buw-rpl.ListPrivilegeRegime.sh
   1 tt/buw-rnv.ValidateNodeRegime.sh testbox *
   1 tt/buw-rnv.ValidateNodeRegime.sh smoke *
   1 tt/buw-rnv.ValidateNodeRegime.sh bujn-winpc *
   1 tt/buw-rnv.ValidateNodeRegime.sh
   1 tt/buw-rnr.RenderNodeRegime.sh smoke *
   1 tt/buw-rnr.RenderNodeRegime.sh bujn-winpc *
   1 tt/buw-rnr.RenderNodeRegime.sh
   1 tt/buw-rnl.ListNodeRegime.sh foo *
   1 tt/buw-rnl.ListNodeRegime.sh bujn-winpc *
   1 tt/buw-rnl.ListNodeRegime.sh buj *
   1 tt/buw-rnl.ListNodeRegime.sh
   1 tt/buw-rcv.ValidateConfigRegime.sh foo *
   1 tt/buw-rcv.ValidateConfigRegime.sh 2>&1 | head -30
   1 tt/buw-rcv.ValidateConfigRegime.sh
   1 tt/buw-rcv.ValidateBuc.sh foo *
   1 tt/buw-rcv.ValidateBuc.sh
   1 tt/buw-rcv.sh
   1 tt/buw-rcr.RenderConfigRegime.sh:*
   1 tt/buw-qsc.QualifyShellCheck.sh 2>&1 | tail -20
   1 tt/buw-qsc.QualifyShellCheck.sh 2>&1 | head -80
   1 tt/buw-jwk.WorkloadKnock.sh bujn-winpc *
   1 tt/buw-jwk.WorkloadKnock.sh
   1 tt/buw-jwk.Knock.sh bujn-winpc *
   1 tt/buw-jpS.PrivilegedSsh.sh pwd *
   1 tt/buw-jpS.PrivilegedSsh.sh bujn-winpc *
   1 tt/buw-jpS.PrivilegedSsh.sh
   1 tt/buw-jpGw.GarrisonWsl.sh bujn-winpc *
   1 tt/buw-jpGb.GarrisonBash.sh
   1 tt/buw-jpF.Fenestrate.sh
   1 tt/buw-jpCM.CaparisonMacos.sh --help
   1 tt/buw-jpCL.CaparisonLinux.sh --help
   1 tt/buw-hw.HandbookWindows.sh
   1 tt/buw-hjw.HandbookJurisdictionWindows.sh
   1 tt/buw-hj0.HandbookJurisdictionTop.sh
   1 tt/buw-h0.HandbookTOP.sh
   1 tt/apcw-t.Test.sh
   1 tt/apcw-nsx.NeuralStanfordExport.sh
   1 tt/apcw-nsi.NeuralStanfordInstall.sh
   1 tt/apcw-nsa.NeuralStanfordAssay.sh Tools/apck/test_fixtures
   1 tt/apcw-cx.ContainerStop.sh
   1 tt/apcw-cs.ContainerStart.sh
   1 tt/apcw-ci.ContainerStatus.sh
   1 tt/apcw-cb.ContainerBuild.sh *
   1 tt/apcw-ba.BatchAssay.sh Tools/apck/test_fixtures
   1 tt/apcw-b.Build.sh
   1 traceroute -n -w 1 -m 4 192.168.1.247
   1 traceroute -n -w 1 -m 4 192.168.1.246
   1 traceroute -n -m 3 -w 1 192.168.1.247
   1 tput:*
   1 top -l 1 -o cpu -n 20
   1 Tools/vvk/bin/vvx jjx:*
   1 Tools/vvk/bin/vvx --help
   1 Tools/rbk/rbtd/target/debug/rbtd "rbw-cC rbw-cQ" calibrant-verdicts
   1 Tools/rbk/rbtd/target/debug/rbtd "" calibrant-sentinel
   1 Tools/rbk/rbtd/target/debug/rbtd "" calibrant-progressing
   1 Tools/rbk/rbtd/target/debug/rbtd
   1 timeout 60 ./tt/rbw-gq.QuotaBuild.sh
   1 timeout 60 ./tt/rbw-fhv.HygieneCheckVessel.sh rbev-busybox
   1 timeout 30 ./tt/rbw-ld.DirectorDivinesLodes.sh
   1 time zstd *
   1 time gzip *
   1 then echo:*
   1 test -d .git/rebase-merge -o -d .git/rebase-apply
   1 TERM=xterm-256color bash /tmp/buc_smoke.sh
   1 TERM=xterm-256color ./tt/rbw-gPE.PayorEstablish.sh
   1 TERM=dumb bash /tmp/buc_smoke.sh
   1 tee /tmp/rbw-tP-attempt2.log
   1 tee /tmp/rbtds-service-260425-1004.log
   1 tee /tmp/inscribe2-260425-1004.log
   1 tee /tmp/inscribe-260425-1004.log
   1 tee /tmp/four-mode-run-260425.log
   1 tee /tmp/apcnsa_export4.log
   1 tee /tmp/apcnsa_export3.log
   1 tee /tmp/apcnsa_export2.log
   1 tee /tmp/apcnsa_export.log
   1 tee /tmp/apcnsa_assay.log
   1 tar -xzf /Users/bhyslop/projects/rbm_alpha_recipemuster/.jjk/parcels/vvk-parcel-1014.tar.gz -C /tmp/p1014
   1 tar -xzf /Users/bhyslop/projects/rbm_alpha_recipemuster/.jjk/parcels/vvk-parcel-1013.tar.gz -C /tmp/vvk1013_install
   1 tar -tzf /Users/bhyslop/projects/rbm_alpha_recipemuster/.jjk/parcels/vvk-parcel-1013.tar.gz
   1 tar -tzf /Users/bhyslop/projects/pb_paneboard02/vvk-parcels/vvk-parcel-1013.tar.gz
   1 tar -tzf /Users/bhyslop/projects/pb_paneboard02/vvk-parcels/vvk-parcel-1011.tar.gz
   1 tailscale status:*
   1 tailscale ping *
   1 tail -6 ../logs-buk/same-rbw-fhv-sh.txt
   1 tail -45 ../logs-buk/same-rbw-lB-sh.txt
   1 tail -20 /Users/bhyslop/projects/rbm_alpha_recipemuster/../_logs_buk/hist-rbw-Rw*.txt
   1 tail -10 /Users/bhyslop/projects/rbm_alpha_recipemuster/../_logs_buk/hist-rbw-Dt*.txt
   1 system_profiler SPHardwareDataType
   1 sysctl *
   1 sudo tee *
   1 sudo tcpdump *
   1 sudo pkill *
   1 sudo -n true
   1 sudo -n route delete 169.254.105.91
   1 sudo -n route delete -net 169.254 -interface en1
   1 sudo -n lsof -iUDP:67
   1 sudo -n ipconfig set en0 DHCP
   1 sudo -n arp-scan --interface=en0 169.254.0.0/16 --retry=2 --timeout=500
   1 sudo -n arp -d 169.254.105.91
   1 sshpass -V
   1 ssh:*
   1 ssh-keygen -lf -
   1 ssh-add:*
   1 ssh wsl@rocket *
   1 ssh cygwin@rocket *
   1 ssh cerebro:*
   1 ssh cerebro *
   1 ssh brad@rocket *
   1 ssh -v cygwin@rocket exit
   1 source Tools/rbk/rbrn_regime.sh
   1 source /Users/bhyslop/projects/rbm_alpha_recipemuster/.rbk/rbgd.env
   1 source .rbk/rbrn_pluml.env
   1 sort -u medical_whitelist.txt -o medical_whitelist.txt
   1 sort -u grep -oE '\\{apcs_[a-z_]+\\}' Tools/apck/APCS0-SpecTop.adoc
   1 sort -u cities.txt -o cities.txt
   1 sort -t'\(' -k2 -rn
   1 sort -t' ' -k2 -n -u
   1 sort -t: -k1
   1 sort -t, -k15 -rn
   1 sort -k6,9
   1 sort -k2 -rn
   1 sort -k2
   1 skopeo inspect:*
   1 shellcheck Tools/rbk/rbgu_Utility.sh Tools/rbk/rbfc_FoundryCore.sh
   1 shellcheck Tools/rbk/rbgp_Payor.sh Tools/rbk/rbgp_cli.sh Tools/rbk/rbgc_Constants.sh
   1 shellcheck Tools/rbk/rbgp_Payor.sh Tools/buk/buh_handbook.sh
   1 shellcheck Tools/rbk/rbgp_Payor.sh
   1 shellcheck Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbgp_Payor.sh Tools/rbk/rbgg_Governor.sh
   1 shellcheck Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbgp_Payor.sh Tools/buk/buh_handbook.sh Tools/rbk/rbgu_Utility.sh Tools/rbk/rbgg_Governor.sh Tools/rbk/rbgc_Constants.sh Tools/rbk/rbrp_regime.sh
   1 shellcheck Tools/rbk/rbgjr/rbgjr01-reliquary-preflight.sh
   1 shellcheck Tools/rbk/rbgc_Constants.sh Tools/rbk/rbgp_Payor.sh
   1 shellcheck Tools/rbk/rbfv_FoundryVerify.sh
   1 shellcheck Tools/rbk/rbfd_FoundryDirectorBuild.sh
   1 shellcheck Tools/rbk/rbfc_FoundryCore.sh
   1 shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbz_zipper.sh
   1 shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/rbhodg_director_graft.sh
   1 shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/rbhodb_director_bind.sh
   1 shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
   1 shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_cli.sh
   1 shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbfl_FoundryLedger.sh
   1 shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbfd_FoundryDirectorBuild.sh
   1 shellcheck -x Tools/rbk/rbrr_regime.sh Tools/rbk/rbrd_regime.sh Tools/rbk/rbrv_regime.sh Tools/rbk/rbrn_regime.sh
   1 shellcheck -x Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbndb_base.sh Tools/rbk/rbgc_Constants.sh
   1 shellcheck -x Tools/rbk/rbfd_FoundryDirectorBuild.sh
   1 shellcheck -x Tools/rbk/rbf_Foundry.sh 2>&1 | head -30
   1 shellcheck -x -s bash Tools/rbk/rbfc_FoundryCore.sh
   1 shellcheck -S warning Tools/rbw/rbob_cli.sh 2>&1 || true
   1 shellcheck -S warning Tools/rbw/rbgu_Utility.sh 2>&1 || true
   1 shellcheck -S warning Tools/buk/bujb_jurisdiction.sh
   1 shellcheck -S warning /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgi_IAM.sh
   1 shellcheck -S style Tools/rbw/rbob_cli.sh 2>&1 || true
   1 shellcheck -S style Tools/rbw/rbgu_Utility.sh 2>&1 || true
   1 shellcheck -S style -f gcc Tools/rbw/rbf_Foundry.sh 2>&1 | grep 'SC2188' -A0 | head -6
   1 shellcheck -S style -f gcc Tools/rbw/*.sh Tools/buk/*.sh 2>&1 | grep 'SC2188'
   1 shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2295' | head -5
   1 shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2188' | head -5
   1 shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2153' | head -5
   1 shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2086' | head -10
   1 shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2086' | grep -v 'rbo.observe.sh' | head -5
   1 shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2059' | head -5
   1 shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2034' | head -10
   1 shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep -oP 'SC[0-9]+' | sort | uniq -c | sort -rn
   1 shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep -oE 'SC[0-9]+' | sort | uniq -c | sort -rn
   1 shellcheck -S style -f gcc Tools/buk/*.sh 2>&1 | grep 'SC2153' | head -5
   1 shellcheck -S style -f gcc Tools/buk/*.sh 2>&1 | grep -oP 'SC[0-9]+' | sort | uniq -c | sort -rn
   1 shellcheck -S style -f gcc Tools/buk/*.sh 2>&1 | grep -oE 'SC[0-9]+' | sort | uniq -c | sort -rn
   1 shellcheck -S style -f gcc Tools/buk/*.sh 2>&1 | grep -E 'SC2016|SC2329|SC2004|SC2254|SC2012' | head -10
   1 shellcheck -S error Tools/rbk/rbob_bottle.sh
   1 shellcheck -S error Tools/rbk/rbgp_Payor.sh
   1 shellcheck -S error Tools/rbk/rbgg_Governor.sh
   1 shellcheck -S error Tools/rbk/rbfl_FoundryLedger.sh Tools/rbk/rbfl_cli.sh Tools/rbk/rbz_zipper.sh Tools/rbk/rbfd_FoundryDirectorBuild.sh Tools/rbk/rbh0/rbhodb_director_bind.sh Tools/rbk/rbh0/rbhodf_director_first_build.sh Tools/rbk/rbh0/rbhodg_director_graft.sh
   1 shellcheck -S error Tools/buk/burs_regime.sh Tools/rbk/rbrr_regime.sh
   1 shellcheck -S error /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbgp_Payor.sh
   1 shellcheck -S error /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbfl_FoundryLedger.sh /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbcc_Constants.sh
   1 shellcheck -S error /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbfd_FoundryDirectorBuild.sh
   1 shellcheck -S error /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/bujb_jurisdiction.sh
   1 shellcheck -S error /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbob_bottle.sh
   1 shellcheck -S error /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgl_GarLayout.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgc_Constants.sh
   1 shellcheck -s bash Tools/buk/buc_command.sh
   1 shellcheck -s bash /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/buc_command.sh
   1 shellcheck -s bash /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/bujb_jurisdiction.sh
   1 shellcheck -s bash -e SC1090,SC1091,SC2034 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
   1 shellcheck -s bash __TRACKED_VAR__/Tools/buk/buh_handbook.sh
   1 shellcheck -f gcc /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgg_Governor.sh
   1 shellcheck -f gcc /tmp/rbgg_top.sh
   1 shellcheck -f gcc /tmp/rbgg_no_disable.sh
   1 shellcheck -e SC2155,SC1090,SC1091 Tools/rbk/rbgp_Payor.sh
   1 shellcheck --severity=error Tools/rbk/*.sh 2>&1 | head -40
   1 shellcheck --rcfile=Tools/buk/busc_shellcheckrc Tools/rbk/rbob_bottle.sh
   1 shellcheck --rcfile=Tools/buk/busc_shellcheckrc Tools/rbk/rbgg_Governor.sh Tools/rbk/rbgu_Utility.sh
   1 shellcheck --rcfile=Tools/buk/busc_shellcheckrc Tools/rbk/rbgc_Constants.sh Tools/rbk/rbgi_IAM.sh Tools/rbk/rbgg_Governor.sh
   1 shellcheck --rcfile=Tools/buk/busc_shellcheckrc -x Tools/rbk/rbrr_regime.sh Tools/rbk/rbrd_regime.sh Tools/rbk/rbrv_regime.sh Tools/rbk/rbrn_regime.sh
   1 shellcheck --rcfile=Tools/buk/busc_shellcheckrc -S style -f gcc Tools/rbk/rbq_Qualify.sh Tools/rbk/rblm_cli.sh
   1 shellcheck --rcfile Tools/buk/busc_shellcheckrc --shell=bash Tools/rbk/rbuh_Http.sh Tools/rbk/rbgc_Constants.sh
   1 shellcheck --external-sources --shell=bash Tools/rbk/rbgp_Payor.sh
   1 shellcheck --external-sources --shell=bash Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbro_regime.sh Tools/rbk/rbro_cli.sh Tools/rbk/rbgv_AccessProbe.sh
   1 shasum Tools/vvk/bin/vvx
   1 shasum -a 256 rbmm_moorings/rbmv_vessels/common-sentry-context/rbjs_sentry.sh rbmm_moorings/rbmv_vessels/common-ifrit-context/src/rbida_sorties.rs
   1 shasum -a 256 /Users/bhyslop/projects/station-files/secrets/payor/rbro.env
   1 shasum -a 256 /Users/bhyslop/projects/station-files/secrets/payor/bhyslop-gmail-com.20260513.rbro.env
   1 shasum -a 256
   1 sh -n /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels/common-sentry-context/rbjs_sentry.sh
   1 sh -c *
   1 sftp wsl@rocket
   1 set -e
   1 sed 's/^/:/;s/$/:/'
   1 sed 's/\\x1b\\[[0-9;]*m//g'
   1 sed 's/\\.adoc$//'
   1 sed 's/-.*$//'
   1 sed -n 90,100p Tools/rbk/vov_veiled/RBSLC-lode_conclave.adoc
   1 sed -n 75,90p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
   1 sed -n 620,640p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjro_ops.rs
   1 sed -n 620,635p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjro_ops.rs
   1 sed -n 60,70p Tools/rbk/vov_veiled/RBSLA-lode_augur.adoc
   1 sed -n 5p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjtq_query.rs
   1 sed -n 55,75p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjri_io.rs
   1 sed -n 470,500p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbfv_FoundryVerify.sh
   1 sed -n 388,394p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
   1 sed -n 35,46p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc
   1 sed -n 330,336p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
   1 sed -n 300,320p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
   1 sed -n 260,275p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/vov_veiled/RBSCJ-CloudBuildJson.adoc
   1 sed -n 220,260p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbfv_FoundryVerify.sh
   1 sed -n 202,225p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbldv_Immure.sh
   1 sed -n 17,33p Tools/rbk/__TRACKED_VAR__.sh
   1 sed -n 155,175p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbldw_Underpin.sh
   1 sed -n 141,160p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbldr_Reliquary.sh
   1 sed -n 122,132p Tools/rbk/vov_veiled/RBSLU-lode_underpin.adoc
   1 sed -n 120,135p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjrsc_scout.rs
   1 sed -n 118,128p Tools/rbk/vov_veiled/RBSLE-lode_ensconce.adoc
   1 sed -n 1070,1095p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjrm_mcp.rs
   1 sed -n 105,133p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjrsc_scout.rs
   1 sed -n 1,35p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjtsc_scout.rs
   1 sed -n 1,2p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/buh_handbook.sh
   1 sed -n 1,2p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/buc_command.sh
   1 sed -n '9946,9975p' .claude/jjm/jjg_gallops.json
   1 sed -n '9763,9772p' .claude/jjm/jjg_gallops.json
   1 sed -n '96p' Tools/rbk/rbfly_Yoke.sh
   1 sed -n '95,140p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/buc_command.sh
   1 sed -n '95,135p' rbldw_Underpin.sh
   1 sed -n '95,125p' Memos/memo-20260609-bedrock-quire-shaping.md
   1 sed -n '95,115p' Tools/rbk/rbfd_director.sh
   1 sed -n '932,942p;972,976p' Tools/rbk/rbtd/src/rbtdrf_fast.rs
   1 sed -n '90,130p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
   1 sed -n '882,895p' Tools/rbk/rbfd_FoundryDirectorBuild.sh
   1 sed -n '860,870p' Tools/rbk/rbfd_FoundryDirectorBuild.sh
   1 sed -n '86,92p' Tools/rbk/rbtd/src/rbtdrk_canonical.rs
   1 sed -n '836,843p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '81,85p' Tools/rbk/vov_veiled/RBSAK-ark_kludge.adoc
   1 sed -n '8,11p' RBSRT-RegimeDepot.adoc
   1 sed -n '795,802p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '777,783p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '762,768p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '75,85p' Tools/rbk/rbldv_immure.sh
   1 sed -n '75,110p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbcc_constants.sh
   1 sed -n '743,755p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '741p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '730,742p' Tools/buk/buv_validation.sh
   1 sed -n '72,318p' Tools/rbk/rbld_Lode.sh
   1 sed -n '708,716p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '70,130p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
   1 sed -n '68,74p' Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
   1 sed -n '68,72p' Tools/rbk/rbrn_cli.sh
   1 sed -n '668,702p' Tools/rbk/rbfl_FoundryLedger.sh
   1 sed -n '63,69p' Tools/xxx_rbn.info.sh
   1 sed -n '62,70p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '600,636p' Tools/rbk/rbfl_FoundryLedger.sh
   1 sed -n '60,95p' Tools/rbk/rbyc_common.sh
   1 sed -n '60,70p' Tools/rbk/vov_veiled/rbv_cli.sh
   1 sed -n '6,9p' RBSRV-RegimeVessel.adoc
   1 sed -n '6,9p' RBSRR-RegimeRepo.adoc
   1 sed -n '6,9p' RBSRM-RegimeMachine.adoc
   1 sed -n '6,9p' RBRN-RegimeNameplate.adoc
   1 sed -n '57,60p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbfl_FoundryLedger.sh
   1 sed -n '55,70p' Tools/rbk/rbfd_director.sh
   1 sed -n '55,58p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbfv_FoundryVerify.sh
   1 sed -n '54,56p' Tools/rbk/vov_veiled/RBSDY-director_yoke.adoc
   1 sed -n '525,610p' Tools/rbk/rbgg_Governor.sh
   1 sed -n '524,546p' README.md
   1 sed -n '52,62p' Tools/rbk/vov_veiled/RBSAC-ark_conjure.adoc
   1 sed -n '50,80p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbcc_Constants.sh
   1 sed -n '472,478p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '46,49p' Tools/rbk/rboo_observe.sh
   1 sed -n '459,462p' Tools/rbk/rbfc_FoundryCore.sh
   1 sed -n '41,46p' Tools/buk/bux_cli.sh
   1 sed -n '40,80p' Tools/rbk/rbgje/rbgje01-enshrine-copy.sh
   1 sed -n '40,60p;100,110p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/src/rbtdrk_canonical.rs
   1 sed -n '40,57p' Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
   1 sed -n '40,50p' Tools/rbk/rbh0/rbhw0_cli.sh
   1 sed -n '4,8p' Tools/rbk/vov_veiled/RBSAK-ark_kludge.adoc
   1 sed -n '39,70p' Tools/rbk/rbld_Lode.sh
   1 sed -n '3855,3865p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
   1 sed -n '382,386p' Tools/buk/bud_dispatch.sh
   1 sed -n '380,387p' Tools/buk/bud_dispatch.sh
   1 sed -n '38,46p' Tools/rbk/rbldv_immure.sh
   1 sed -n '375,400p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
   1 sed -n '371,384p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '37,52p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtd/src/rbtdrk_canonical.rs
   1 sed -n '37,40p' Tools/jjk/vov_veiled/src/jjrg_gallops.rs
   1 sed -n '364,370p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '363,369p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '360,380p' Tools/rbk/rbgc_constants.sh
   1 sed -n '36,50p;140,148p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/src/rbtdrp_pristine.rs
   1 sed -n '358,367p' Tools/rbk/rbfln_Inventory.sh
   1 sed -n '352,358p' Tools/rbk/rbgb_Buckets.sh
   1 sed -n '350,360p' Tools/rbk/rbgo_OAuth.sh
   1 sed -n '35,60p' Tools/rbk/rbldw_underpin.sh
   1 sed -n '340,375p' Tools/buk/buv_validation.sh
   1 sed -n '336,345p' Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
   1 sed -n '3314,3410p' Tools/rbk/rbtd/src/rbtdrc_crucible.rs
   1 sed -n '330,345p' Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
   1 sed -n '33,92p' Tools/buk/burd_regime.sh
   1 sed -n '33,46p' Tools/buk/buut_tabtarget.sh
   1 sed -n '320,489p' Tools/rbk/rbld_Lode.sh
   1 sed -n '319,323p' Tools/buk/bujb_jurisdiction.sh
   1 sed -n '309,314p' Tools/apck/apcc_cli.sh
   1 sed -n '30,45p' Tools/rbk/rbgjm/rbgjm01-mirror-image.sh
   1 sed -n '2970,2990p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
   1 sed -n '285,298p' Tools/rbk/rbh0/rbhoda_director_airgap.sh
   1 sed -n '280,330p' Tools/rbk/rbtd/src/rbtdri_invocation.rs
   1 sed -n '28,32p' Tools/rbk/vov_veiled/RBSAC-ark_conjure.adoc
   1 sed -n '270,300p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
   1 sed -n '27,35p' Tools/vvk/vvb_cli.sh
   1 sed -n '265,280p' Tools/rbk/vov_veiled/RBSCJ-CloudBuildJson.adoc
   1 sed -n '260,270p' Tools/rbk/rbfd_director.sh
   1 sed -n '260,270p' Memos/memo-20260609-bedrock-quire-shaping.md
   1 sed -n '2553,2578p' Tools/rbk/rbtd/src/rbtdrc_crucible.rs
   1 sed -n '255,275p' Tools/rbk/rbgo_OAuth.sh
   1 sed -n '255,259p' Tools/buk/bud_dispatch.sh
   1 sed -n '252,262p' Tools/buk/bud_dispatch.sh
   1 sed -n '25,50p' Tools/rbk/rbldb_bole.sh
   1 sed -n '25,33p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '25,33p' Tools/rbk/rbfly_Yoke.sh
   1 sed -n '249,473p' Tools/rbk/rbfc_FoundryCore.sh
   1 sed -n '246,251p' Memos/memo-20260609-federation-canon.md
   1 sed -n '243,247p' Tools/rbk/rbgo_OAuth.sh
   1 sed -n '240,248p' Tools/rbk/rbgo_OAuth.sh
   1 sed -n '2358,2386p' Tools/rbk/rbtd/src/rbtdrc_crucible.rs
   1 sed -n '2306,2313p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
   1 sed -n '230,240p' Tools/buk/buut_tabtarget.sh
   1 sed -n '23,27p' Tools/rbk/vov_veiled/RBSAC-ark_conjure.adoc
   1 sed -n '2274,2293p' Tools/buk/bujb_jurisdiction.sh
   1 sed -n '22,45p' Tools/rbk/rbh0/rbhpb_base.sh
   1 sed -n '218,224p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '213,248p' Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
   1 sed -n '21,25p' Tools/rbk/vov_veiled/RBSAB-ark_about.adoc
   1 sed -n '205,212p' Tools/rbk/vov_veiled/RBSAV-ark_vouch.adoc
   1 sed -n '200,300p'
   1 sed -n '20,60p' Tools/rbk/rbldr_reliquary.sh
   1 sed -n '20,40p' Tools/buk/buq_qualify.sh
   1 sed -n '19p' Tools/rbk/rbtd/src/rbtdrp_pristine.rs
   1 sed -n '195,230p' Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
   1 sed -n '188,205p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtd/Tools
   1 sed -n '180,205p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbz_zipper.sh
   1 sed -n '180,200p;470,485p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
   1 sed -n '18,45p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/src/rbtdrm_manifest.rs
   1 sed -n '178,186p' Tools/rbk/rbfd_director.sh
   1 sed -n '1730,1762p' Tools/rbk/rbtd/src/rbtdrf_fast.rs
   1 sed -n '1725,1742p' Tools/rbk/rbfd_FoundryDirectorBuild.sh
   1 sed -n '168,171p' Tools/rbk/rbrn_regime.sh
   1 sed -n '165,170p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '1612,1640p' Tools/rbk/rbgp_Payor.sh
   1 sed -n '160,190p' Tools/rbk/rbfc_FoundryCore.sh
   1 sed -n '160,182p' Tools/rbk/rbh0/rbhots_tadmor_security.sh
   1 sed -n '159p' Tools/rbk/rbtd/src/rbtdrm_manifest.rs
   1 sed -n '1578,1584p' Tools/rbk/rbgp_Payor.sh
   1 sed -n '155,190p' Tools/rbk/vov_veiled/CLAUDE.consumer.md
   1 sed -n '150,210p' Tools/rbk/rbrn_cli.sh
   1 sed -n '150,153p' Tools/rbk/rbtd/src/rbtdtk_canonical.rs
   1 sed -n '1462,1466p' Tools/rbk/rbtd/src/rbtdrf_fast.rs
   1 sed -n '145,180p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbgp_Payor.sh
   1 sed -n '1438,1480p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
   1 sed -n '140,160p' Tools/rbk/rbz_zipper.sh
   1 sed -n '140,160p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 sed -n '1373,1378p' Tools/rbk/rbgp_Payor.sh
   1 sed -n '128,150p' Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
   1 sed -n '128,145p' Tools/buk/buut_tabtarget.sh
   1 sed -n '128,132p' README.md
   1 sed -n '1270,1320p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
   1 sed -n '126,142p' Tools/rbk/rbtd/src/rbtdrp_pristine.rs
   1 sed -n '125,150p' Tools/rbk/rbfc0_core.sh
   1 sed -n '1239,1243p' Tools/rbk/rbtd/src/rbtdrf_fast.rs
   1 sed -n '119,125p' Tools/rbk/rbtd/src/rbtdtp_pristine.rs
   1 sed -n '118p;176p;178p;190p;192p;218p;251p' rbmm_moorings/rbmv_vessels/common-sentry-context/rbjs_sentry.sh
   1 sed -n '116,124p' Memos/memo-20260609-federation-canon.md
   1 sed -n '110,140p' Tools/rbk/rbrn_cli.sh
   1 sed -n '106,109p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
   1 sed -n '1033,1037p' Tools/rbk/rbtd/src/rbtdrf_fast.rs
   1 sed -n '100,112p' Tools/rbk/rbz_zipper.sh
   1 sed -n '100,108p' Tools/rbk/vov_veiled/RBSCJ-CloudBuildJson.adoc
   1 sed -n '100,108p' Tools/rbk/rbtd/src/rbtdrm_manifest.rs
   1 sed -n '10,16p' RBSRV-RegimeVessel.adoc
   1 sed -n '10,14p' RBRN-RegimeNameplate.adoc
   1 sed -n '1,7p' Tools/jjk/vov_veiled/src/jjtq_query.rs
   1 sed -n '1,60p' Tools/rbk/rbrs_cli.sh
   1 sed -n '1,40p' Tools/rbk/rbtd/src/main.rs
   1 sed -n '1,40p' Tools/rbk/rbtd/src/lib.rs
   1 sed -n '1,40p' Tools/buk/bubc_constants.sh
   1 sed -n '1,30p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/but_test.sh
   1 sed -n '1,2p' Tools/buk/buh_handbook.sh
   1 sed -n '1,2p' Tools/buk/buc_command.sh
   1 sed -n '1,2p' .claude/jjm/officia/260604-1004/gazette_in.md
   1 sed -n '1,25p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/bul_nolog_launcher.sh
   1 sed -n '1,25p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/bul_launcher.sh
   1 sed -n '1,20p' Tools/rbk/rbgjs/rbgjs-gpg-verify-sums.sh
   1 sed -n '1,20p' Tools/rbk/rbgjs/rbgjs-gcrane-append.sh
   1 sed -n '1,20p' rbgjl/rbgjl04-underpin-capture.sh
   1 sed -n '1,18p' rbgjm/rbgjm01-mirror-image.sh
   1 sed -n '1,16p' Tools/rbk/rbgjs/rbgjs-gpg-verify-sums.sh
   1 sed -n '1,14p' Tools/rbk/rbgjs/rbgjs-skopeo-fingerprint.sh
   1 sed -n '/rbrr_probate\(\)/,/^}/p' Tools/rbk/rbrr_regime.sh
   1 sed -n '/rbrn_probate\(\)/,/^}/p' Tools/rbk/rbrn_regime.sh
   1 sed -n '/rbcc_emit_consts\(\)/,/^}/p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbcc_constants.sh
   1 sed -n '/Obtain OAuth2 token/,/SLOT_1_ORIGIN/p' /tmp/new_body_rbgjl01-ensconce-capture.sh
   1 sed -n '/furnish\(\) {/,/^}/p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/burs_cli.sh
   1 sed -n '/furnish\(\) {/,/^}/p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/burn_cli.sh
   1 sed -n '/furnish\(\) {/,/^}/p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/burc_cli.sh
   1 sed -n '/^buc_reject\(\)/,/^}/p' Tools/buk/buc_command.sh
   1 sed -n '/^buc_die\(\)/,/^}/p' Tools/buk/buc_command.sh
   1 sed -i.bak -e s/RBRR_DEPOT_PROJECT_ID/RBDC_DEPOT_PROJECT_ID/g -e s/RBRR_GCB_POOL_STEM/RBDC_GCB_POOL_STEM/g Tools/rbk/rbh0/rbhpq_quota_build.sh
   1 sed -i.bak -e 's/RBRR_DEPOT_PROJECT_ID/RBRR_DEPOT_MONIKER/g' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/rbhodf_director_first_build.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/rbhoda_director_airgap.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/rbhodb_director_bind.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/rbhodg_director_graft.sh
   1 sed -i '' 's/^RBRV_RELIQUARY=.*/RBRV_RELIQUARY=r260327172456/' "$v"
   1 sed -i '' '886,1136d' Tools/rbk/rbfd_FoundryDirectorBuild.sh
   1 sed -i '' '365,403d' Tools/rbk/rbfln_Inventory.sh
   1 sed -i '' '/shellcheck disable=SC2153/,+3d' /tmp/rbgg_no_disable.sh
   1 sed -i '' -e s/rbgp_governor_mantle/rbgp_enrobe_governor/g -e s/RBZ_MANTLE_GOVERNOR/RBZ_ENROBE_GOVERNOR/g -e s/z_mantle_sa/z_govsa/g -e s/z_mantle_delete/z_govsa_delete/g -e 's/Next: mantle Governor/Next: enrobe Governor/g' -e s/demantle/defrock/g Tools/rbk/rbgp_payor.sh
   1 sed -i '' -e s/rbgg_invest_retriever/rbgg_enrobe_retriever/g -e s/rbgg_invest_director/rbgg_enrobe_director/g -e s/rbgg_divest_retriever/rbgg_defrock_retriever/g -e s/rbgg_divest_director/rbgg_defrock_director/g -e s/zrbgg_divest_role/zrbgg_defrock_role/g -e 's/the invested/the enrobed/g' -e s/invester/enrober/g -e s/Investing/Enrobing/g -e 's/Invest a /Enrobe a /g' -e 's/the invest body/the enrobe body/g' -e 's/declaring invest complete/declaring enrobe complete/g' -e s/post-invest/post-enrobe/g -e s/invest-side/enrobe-side/g -e 's/same-name invest/same-name enrobe/g' -e 's/Divest a /Defrock a /g' -e s/Divesting/Defrocking/g -e s/divested/defrocked/g -e 's/Divest operation/Defrock operation/g' Tools/rbk/rbgg_governor.sh
   1 sed -i '' -e 's|Memos/memo-20260610-quoin-minting-introspection\\.md|Memos/retired/memo-20260610-quoin-minting-introspection.md|g' Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
   1 sed -i '' -e 's|Memos/memo-20260610-heat-BH-image-tabtarget-cleanup\\.md|Memos/retired/memo-20260610-heat-BH-image-tabtarget-cleanup.md|g' Tools/rbk/rbz_zipper.sh Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc
   1 sed -i '' -e 's|Memos/memo-20260610-heat-BH-fable-recommendation-convergence-deadline-shape\\.md|Memos/retired/memo-20260610-heat-BH-fable-recommendation-convergence-deadline-shape.md|g' Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc
   1 sed -i '' -e 's/RBZ_DEFROCK_RETRIEVER /RBZ_DEFROCK_RETRIEVER/' -e 's/RBZ_DEFROCK_DIRECTOR /RBZ_DEFROCK_DIRECTOR/' -e 's/"rbgg_defrock_retriever" /"rbgg_defrock_retriever"/' -e 's/"rbgg_defrock_director" /"rbgg_defrock_director"/' Tools/rbk/rbz_zipper.sh
   1 sed -i '' -e 's/one investiture addition/one capability-set addition/' Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
   1 sed -i '' -e 's/mantles, charters/enrobes, charters/' Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc
   1 sed -i '' -e 's/invest existence preflight/enrobe existence preflight/' -e 's/standing-depot re-invest/standing-depot re-enrobe/' -e s/deletes-then-reinvests/deletes-then-re-enrobes/ -e 's/a fresh invest/a fresh enrobe/' -e 's/governor re-mantle/governor re-enrobe/' -e 's/divest repo/defrock repo/' Tools/rbk/vov_veiled/RBSCIP-IamPropagation.adoc
   1 sed -i '' -e 's/capital `I` \(invest\) and `D` \(divest\)/capital `E` \(enrobe\) and `F` \(defrock\)/' -e s/invester/enrober/g -e s/divestiture/defrocking/g -e s/reinvests/re-enrobes/g -e s/reinvest/re-enrobe/g -e s/investing/enrobing/g -e s/invested/enrobed/g -e s/invests/enrobes/g -e s/invest/enrobe/g -e s/Investing/Enrobing/g -e s/Invested/Enrobed/g -e s/Invests/Enrobes/g -e s/Invest/Enrobe/g -e s/INVEST/ENROBE/g -e s/divesting/defrocking/g -e s/divested/defrocked/g -e s/divests/defrocks/g -e s/divest/defrock/g -e s/Divesting/Defrocking/g -e s/Divested/Defrocked/g -e s/Divests/Defrocks/g -e s/Divest/Defrock/g -e s/DIVEST/DEFROCK/g -e s/demantle/defrock/g -e s/governor_mantle/governor_enrobe/g -e s/re-mantles/re-enrobes/g -e s/re-mantle/re-enrobe/g -e s/mantle/enrobe/g -e s/Mantles/Enrobes/g -e s/Mantle/Enrobe/g -e s/MANTLE/ENROBE/g Tools/rbk/vov_veiled/RBS0-SpecTop.adoc Tools/rbk/vov_veiled/RBSDK-director_enrobe.adoc Tools/rbk/vov_veiled/RBSRK-retriever_enrobe.adoc Tools/rbk/vov_veiled/RBSDD-director_defrock.adoc Tools/rbk/vov_veiled/RBSRD-retriever_defrock.adoc
   1 sed -i '' -e 's/A re-divest, or a/A re-defrock, or a/' -e 's/divest of an identity/defrock of an identity/' -e 's/governor freshly re-mantled/governor freshly re-enrobed/' Tools/rbk/vov_veiled/RBSCIG-IamGrantContracts.adoc
   1 sed -f /tmp/apcs0-rename.sed Tools/apck/APCS0-SpecTop.adoc
   1 sed -E 's/\(=.{6}\).*/\\1…/' ../station-files/secrets/payor/rbro.env
   1 sed -E 's/.*\\.\([a-zA-Z0-9]+\)$/\\1/'
   1 script -q /dev/null ./tt/rbw-tf.TestFixture.three-mode.sh 2>&1
   1 script -q /dev/null ./tt/rbw-DPG.DirectorRefreshesGcbPins.sh 2>&1
   1 script -q /dev/null ./tt/rbw-DI.DirectorInscribesRubric.sh 2>&1
   1 scp:*
   1 scp cerebro:projects/rbm_alpha_recipemuster/../logs-buk/hist-rbw-tP-sh-20260514-203134-1368806-985.txt /tmp/bench-bo/cerebro-20260514.txt
   1 scp /Users/bhyslop/projects/station-files/secrets/retriever/rbra.env cerebro:/home/bhyslop/projects/station-files/secrets/retriever/rbra.env
   1 scp /Users/bhyslop/projects/station-files/secrets/director/rbra.env cerebro:/home/bhyslop/projects/station-files/secrets/director/rbra.env
   1 scp 'cerebro:projects/rbm_alpha_recipemuster/../logs-buk/hist-rbw-fO-sh-20260514-2*.txt' /tmp/bench-bo/
   1 scp 'cerebro:.config/gcloud/legacy_credentials/director-bhl@cancbhl-d-canest2bhl100011.iam.gserviceaccount.com/adc.json' /tmp/bench-bo/adc-director.json
   1 scp -o ConnectTimeout=15 /Users/bhyslop/projects/rbm_beta_recipemuster/../logs-buk/hist-rbw-tP-sh-20260520-172702-4781-219.txt cerebro:~/
   1 rustup target *
   1 rustup run *
   1 rustfmt --version
   1 rustfmt --emit stdout t.rs
   1 rustfmt --emit stdout --config imports_granularity=Module t.rs
   1 rustfmt --config imports_layout=Vertical --emit stdout /tmp/fmttest/t.rs
   1 ruby -e 'require "asciidoctor"; puts "ASCIIDOCTOR_GEM_OK"'
   1 ruby --version
   1 rmdir /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid
   1 rmdir /Users/bhyslop/projects/rbm_alpha_recipemuster/.claude/jjm/officia/☉260512-1014
   1 rmdir .buk .rbk
   1 rm tt/rbw-Db.DirectorBuildsAbout.sh
   1 rm Tools/rbk/rbh0/rbhpq_quota_build.sh.bak
   1 rm /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/*.bak
   1 rm -rf vvk1013_install
   1 rm -rf p1014
   1 rm -rf /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid/target
   1 rm -f /tmp/tier6_check.sh
   1 rm -f /tmp/tier5_check.sh /tmp/tier5_check2.sh
   1 rm -f /tmp/scp_probe.txt
   1 rg *
   1 rg -rno '"rb[a-z]{2,3}_[a-z_]+"' Tools/rbk --include='*.sh'
   1 rg -rn -i 'lode.?lifecycle|FIXTURE_LODE|ensconce|divine|banish' /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtd/src/
   1 rg -rn -i 'context_file|claude-rbk|claude-vok|claude-cmk|claude-buk|claude-apck' /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbcc_Constants.sh /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgc_Constants.sh
   1 rg -rin 'muniment|terrier' Tools/rbk --include='*.sh' --include='*.adoc' --include='*.rs'
   1 rg -n 'Tools/' -g '!target' -g '!*.d' -g '!*.md' -g '!*.adoc' -g '!Memos/**' --count-matches
   1 rg -n 'rbi_vouch|rblv_|vouch.*json|jq -n' Tools/rbk/rbfv_verify.sh Tools/rbk/rbldb_*.sh
   1 rg -n -i 'skirmish' /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtd/src/*.rs
   1 rg -n --no-heading '"[a-z]{2,6}_[a-z_]+"\\s*:' Tools/rbk/*.sh
   1 rg -n --glob !.git --glob !Memos/** 'rbk-claude-acronyms|rbk-claude-tabtarget-context|rbk-claude-theurge-ifrit-context|vok-claude-context|apck-claude-context|cmk-claude-context|cmk-rules-of-engagement-detail|claude-cmk-rules-of-engagement|buk-claude-context' /home/bhyslop/projects/rbm_alpha_recipemuster
   1 rg -ln 'invest|divest|roster|mantle' Tools/rbk/vov_veiled/RBSDK*.adoc Tools/rbk/vov_veiled/RBSRK*.adoc Tools/rbk/vov_veiled/RBSDD*.adoc Tools/rbk/vov_veiled/RBSRD*.adoc Tools/rbk/vov_veiled/RBSDR*.adoc Tools/rbk/vov_veiled/RBSRL*.adoc Tools/rbk/vov_veiled/RBSGM*.adoc
   1 Read Tools/rbk/rbtd/src/main.rs
   1 read f *
   1 read a *
   1 read -r p
   1 python3:*
   1 python3 *
   1 python3 -c ":*
   1 python3 -c 'import json,sys; d=json.load\(sys.stdin\); print\("client_email:", d.get\("client_email"\)\); print\("project_id:", d.get\("project_id"\)\)'
   1 pstree -p 13501
   1 ps -p 8488 -o pid,etime,command
   1 printf 'use foo::{aaa, bbb, ccc};\\nfn main\(\) {}\\n'
   1 printf 'SGVsbG8gV29ybGQ='
   1 printf 'scp-protocol-probe\\n'
   1 printf 'Hello World'
   1 printf 'GET / HTTP/1.1\\r\\nHost: www.internic.net\\r\\nUser-Agent: rbid/1.0\\r\\nConnection: close\\r\\n\\r\\n'
   1 printf 'GET / HTTP/1.1\\r\\nHost: www.internic.net\\r\\nConnection: close\\r\\n\\r\\n'
   1 printf '%s\\n' "YzI2MDQwMjA5MTIzMi1yMjYwNDAyMTYxNTIx"
   1 printf '%s' "YzI2MDQwMjA5MTIzMi1yMjYwNDAyMTYxNTIx"
   1 printf '\\n# eof\\n'
   1 printf '{""""alg"""":""""RS256"""",""""typ"""":""""JWT""""}'
   1 podman system:*
   1 podman machine:*
   1 podman images:*
   1 plutil -p "/Applications/SlickEditPro2025.app/Contents/Info.plist" 2>&1 | head -30
   1 pkill -f "target/release/apcap"
   1 pkill -f "target/debug/apcap"
   1 pkill -f "rbw-tf.TestFixture.three-mode"
   1 pkill -f "rbw-tf.TestFixture.four-mode"
   1 pkill -f "rbw-cKS"
   1 pkill -f "rbw_workbench.sh"
   1 pkill -f "docker pull"
   1 pkill -9 -f "docker pull"
   1 pip show *
   1 ping *
   1 ping -c 3 -W 1000 169.254.105.91
   1 ping -c 3 -W 1000 -b en0 169.254.105.91
   1 ping -c 3 -i 0.3 -b 169.254.255.255
   1 ping -c 2 -W 1000 169.254.105.91
   1 ping -c 2 -W 1 DS223j.local
   1 ping -c 2 -W 1 bhyslop-nas2.local
   1 ping -c 2 -W 1 192.168.1.1
   1 ping -c 1 -W 1 DS223j.local
   1 ping -c 1 -W 1 DS223j
   1 ping -c 1 -W 1 bhyslop-nas2.local
   1 ping -c 1 -W 1 bhyslop-nas2
   1 ping -c 1 -W 1 192.168.86.245
   1 ping -c 1 -W 1 192.168.1.247
   1 ping -c 1 -W 1 192.168.1.246
   1 perl:*
   1 perl -i -pe 's/rbtdrp_canonical_rbra\\\(&root, "assay"\\\)/rbtdrp_canonical_rbra\(\\&root, RBTDGC_ROLE_ASSAY\)/g' rbtdrp_pristine.rs
   1 perl -i -pe 's/RBTDRM_ROLE_/RBTDGC_ROLE_/g; s/rbtdrk_canonical_rbra\\\(&root, "assay"\\\)/rbtdrk_canonical_rbra\(\\&root, RBTDGC_ROLE_ASSAY\)/g' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/src/rbtdrk_canonical.rs
   1 perl -i -pe 's/\\bRBTDRP_DOT_DIR\\b/RBTDGC_MOORINGS_DIR/g; *
   1 perl -i -pe 's/\\bRBTDRM_ROLE_/RBTDGC_ROLE_/g; s/crate::RBTD_MOORINGS_DIR\\b/crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR/g' __TRACKED_VAR__.rs
   1 perl -i -pe 's/\\bRBTDRK_RBRR_FILE\\b/RBTDGC_RBRR_FILE/g; *
   1 perl -i -pe 's/\(make_valid_tack\\\([^\)]*?\), None\\\)/$1\)/g' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjtg_gallops.rs
   1 perl -i -pe 's/\(jjtsc_make_tack\\\([^\)]*?\), None\\\)/$1\)/g' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjtsc_scout.rs
   1 perl -0pi -e "s/printf '%s' \\"\\\\\\$\\{z_buym_format\\}\\" >&2/printf '%b' \\"\\\\\\${z_buym_format}\\" >&2/g" Tools/buk/buts/butcym_YelpModule.sh
   1 pandoc --version
   1 osascript -e 'tell application "System Events" to tell \(first application process whose frontmost is true\) to get name of front window'
   1 osascript -e 'tell application "System Events" to get name of first application process whose frontmost is true'
   1 osascript -e 'tell application "iTerm2" to set name of current session of current window to ""'
   1 osascript -e 'tell application "iTerm2" to get tty of current session of current window'
   1 osascript -e 'tell application "iTerm2" to get name of current window'
   1 osascript -e 'tell application "iTerm2" to get name of current session of current window'
   1 osascript -e 'quit app "Docker"'
   1 osascript -e 'quit app "Docker Desktop"'
   1 openssl version *
   1 openssl pkey *
   1 openssl enc:*
   1 openssl dgst:*
   1 open:*
   1 npm:*
   1 npm view:*
   1 NO_COLOR=1 TERM=xterm-256color bash /tmp/buc_smoke.sh
   1 networksetup -listallhardwareports
   1 networksetup -getairportnetwork en1
   1 nc -zv -G 2 169.254.105.91 5001
   1 nc -zv -G 2 169.254.105.91 5000
   1 nc -zv -G 2 169.254.105.91 22
   1 nc -z -G 3 DS223j 548
   1 nc -z -G 3 DS223j 5001
   1 nc -z -G 3 DS223j 5000
   1 nc -z -G 3 DS223j 445
   1 nc -z -G 3 bhyslop-nas2 445
   1 nc -w 5 www.internic.net 80
   1 nc -w 5 192.0.46.9 80
   1 nc -u -w 2 -b 255.255.255.255 9999
   1 nc -G 5 -w 5 -vz ssh.github.com 443
   1 nc -G 5 -w 5 -vz github.com 443
   1 nc -G 5 -w 5 -vz github.com 22
   1 mv:*
   1 mv tt/rbw-DE.DirectorEnshrinesBaseImages.sh tt/rbw-DE.DirectorEnshrinesVessel.sh
   1 mv tt/rbw-DC.DirectorCreatesArk.sh tt/rbw-DC.DirectorCreatesConsecration.sh
   1 mv tt/rbw-DA.DirectorAbjuresArk.sh tt/rbw-DA.DirectorAbjuresConsecration.sh
   1 mv Tools/rbk/rbldk_Kindle.sh        Tools/rbk/rbld0_Lode.sh
   1 mv Tools/rbk/rbh0/rbhob_base.sh     Tools/rbk/rbh0/rbho0_Onboarding.sh
   1 mv Tools/rbk/rbflk_Kindle.sh        Tools/rbk/rbfl0_FoundryLedger.sh
   1 mv Tools/rbk/rbfck_Kindle.sh        Tools/rbk/rbfc0_FoundryCore.sh
   1 mv /Users/bhyslop/projects/station-files/secrets/assay/rbra.env /Users/bhyslop/projects/station-files/secrets/director/rbra.env
   1 mv /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk/jjf_fundus.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk/jjfp_fundus.sh
   1 mv /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk/jjf_cli.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk/jjfp_cli.sh
   1 mv /Users/bhyslop/projects/rbm_alpha_recipemuster/.claude/jjm/officia/☉260512-1014/gazette_in.md /Users/bhyslop/projects/rbm_alpha_recipemuster/.claude/jjm/officia/260512-1014/gazette_in.md
   1 mv /tmp/rbtdrc_crucible.rs.new /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/src/rbtdrc_crucible.rs
   1 mv /tmp/APCS0-new.adoc Tools/apck/APCS0-SpecTop.adoc
   1 mv .rbk/tadmor.rbrn.env .rbk/tadmor/rbrn.env
   1 mv .rbk/tadmor.compose.yml .rbk/tadmor/compose.yml
   1 mv .rbk/srjcl.rbrn.env .rbk/srjcl/rbrn.env
   1 mv .rbk/pluml.rbrn.env .rbk/pluml/rbrn.env
   1 mv .claude/jjm/officia/260604-1004/gazette_out.md .claude/jjm/officia/260604-1004/gazette_in.md
   1 mv ../station-files/secrets/assay/rbra.env ../station-files/secrets/governor/rbra.env
   1 mkdir vvk1013_install
   1 mkdir p1014 *
   1 mkdir -p rbmm_moorings/rbml_launchers
   1 mkdir -p /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels/rbev-bottle-ccyolo/workspace
   1 mkdir -p /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels/rbev-bottle-ccyolo/build-context
   1 mkdir -p /tmp/rbrd-capture
   1 mkdir -p /tmp/fmttest
   1 mkdir -p /tmp/bench-bo
   1 mkdir -p /tmp/apcnsa_compare
   1 mkdir -p /tmp/a6aar_baseline
   1 mkdir -p /tmp/a6aar_after
   1 mkdir -p .rbk/tadmor .rbk/srjcl .rbk/pluml
   1 mkdir -p .claude/jjm/officia/260419-1001
   1 mdutil -s / 2>&1
   1 mdls -name kMDItemDisplayName -name kMDItemContentType -name kMDItemKind "/Applications/SlickEditPro2025.app" 2>&1
   1 mdfind:*
   1 md5:*
   1 md5 -r /Users/bhyslop/projects/station-files/secrets/director/rbra.env /Users/bhyslop/projects/station-files/secrets/retriever/rbra.env
   1 lsof -nP -iTCP:7999 -sTCP:LISTEN
   1 ls:*
   1 ls Tools/rbk/vov_veiled/RBSL*
   1 ls Tools/rbk/vov_veiled/RBSAE*
   1 ls /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-*.sh | xargs -I{} basename {} | sort
   1 ls "../temp-buk/temp-20260609-104733-83286-60/rbtd/rbtdrc_reliquary_lifecycle/"
   1 ls .claude/jjm/
   1 ls -laR .claude/jjm/officia/260603-1004/
   1 ls -1 RBSR*.adoc
   1 ls -1 RBSP*.adoc
   1 ls -1 RBSM*.adoc
   1 ls -1 RBSD*.adoc
   1 ls -1 /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rb?c_Constants.sh
   1 ls -1 __TRACKED_VAR__/burv-output/invoke-00000/previous/
   1 LOG_DIR="/Users/bhyslop/projects/rbm_alpha_recipemuster/../_logs_buk" for f in hist-rbw-DI-sh-20260327-171824-18370-204.txt hist-rbw-DI-sh-20260327-172456-18929-881.txt hist-rbw-DE-sh-20260327-182829-22515-574.txt hist-rbw-DC-sh-20260327-183013-23423-580.txt hist-rbw-DC-sh-20260327-201528-53812-528.txt hist-rbw-DC-sh-20260327-202101-55053-710.txt hist-rbw-DC-sh-20260327-202655-57216-703.txt hist-rbw-DE-sh-20260327-203624-60526-870.txt hist-rbw-DC-sh-20260327-203832-61483-888.txt hist-rbw-DE-sh-20260327-205221-65590-120.txt hist-rbw-DC-sh-20260327-205505-66475-551.txt hist-rbw-Rs-sh-20260327-210847-70631-97.txt hist-rbw-Rs-sh-20260327-210923-71464-289.txt
   1 LC_ALL=C uniq -c
   1 kill 58792
   1 kill %1
   1 kill -9 29018 29104
   1 jq:*
   1 jq -r keys[] /Users/bhyslop/projects/rbm_beta_recipemuster/.claude/settings.local.json
   1 jq -r '.privateKeyData' /var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-20366/burv/invoke-00002/temp/temp-20260427-130309-22827-763/rbgu_gov_key-attempt1_u_resp.json
   1 jq -r .permissions.allow[]? /Users/bhyslop/projects/rbm_beta_recipemuster/.claude/settings.local.json
   1 jq -er '.access_token' /tmp/rbspike/sts_resp.json
   1 JJTEST_HOST=localhost cargo test --manifest-path Tools/jjk/vov_veiled/Cargo.toml --test fundus_scenario relay_concurrent_overlap -- --ignored --nocapture 2>&1
   1 ifconfig -l
   1 id "$u"
   1 iconv -t UTF-16LE
   1 host rocket *
   1 host 93.184.216.34
   1 host 192.168.1.247
   1 host 192.168.1.246
   1 head:*
   1 grep:*
   1 grep *
   1 grep -rnE "RBGD_API_SERVICE_ACCOUNTS=|RBGC_PATH_KEYS=|RBGC_API_ROOT_IAM=|RBGC_IAM_V1=|ZRBGG_INFIX_KEY=|ZRBGG_INFIX_LIST_KEYS=|ZRBGG_INFIX_DELETE=|ZRBGG_PREFIX=" Tools/rbk/
   1 grep -rn "case.*director\\|case.*retriever\\|case.*governor\\|case.*\\\\\\$.*role" /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk --include="*.sh"
   1 grep -rn -i "CI\\b\\|CI/CD\\|automated\\|unattended\\|headless\\|test run\\|gauntlet\\|nightly\\|pipeline\\|cron" Memos/memo-20260427-google-native-human-auth.md Memos/memo-20260522-org-affiliated-credential-reorientation.md
   1 grep -rln "RBRV_RELIQUARY\\|rbrv\\.env" Tools/rbk/rbfli_*.sh Tools/rbk/rbfly_*.sh Tools/rbk/rbfca_*.sh
   1 grep -rln "Forthcoming" .
   1 grep -rln 'python' Tools/rbk/rbfca_StepAssembly.sh Tools/rbk/rblds_Spine.sh Tools/rbk/rbgj*
   1 grep -rln -e 'python' -e '#!/usr/bin/env python' Tools/rbk/rbgj* Tools/rbk/rbgjs
   1 grep -rliE 'worksite' Memos/ Tools/ .claude/jjm/jji_itch.md
   1 grep -nF *
   1 grep -nF '>>>>>>>' .claude/jjm/jjg_gallops.json
   1 grep -nF '=======' .claude/jjm/jjg_gallops.json
   1 grep -n "^=\\{1,3\\} " Tools/jjk/vov_veiled/JJS0_JobJockeySpec.adoc
   1 grep -n '\\$\(' Tools/rbk/rbgp_Payor.sh | grep -v '\\$\(<' | grep -v '\\$\(\(' | grep -v '_capture' | head -30
   1 grep -n '\\$\(' Tools/rbk/rbgp_Payor.sh | grep -v '\\$\(<' | grep -v '\\$\(\(' | grep -v '_capture'
   1 grep -n '\\$\(' Tools/rbk/rbf_Foundry.sh | grep -v '\\$\(<' | grep -v '\\$\(\(' | grep -v '_capture' | head -80
   1 grep -n '\\$\(' Tools/rbk/rbf_Foundry.sh | grep -v '\\$\(<' | grep -v '\\$\(\(' | grep -v '_capture' | head -30
   1 grep -h "^[a-z][a-z0-9]*_[a-z0-9]*\\\(\\\)" /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbra_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbrp_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbrs_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbro_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbrg_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbrn_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbrr_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbrv_regime.sh 2>/dev/null | sort | uniq
   1 grep -h "^[a-z][a-z0-9]*_[a-z0-9]*\\\(\\\)" /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/burc_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/burs_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/bure_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/burd_regime.sh 2>/dev/null | sort | uniq
   1 git:*
   1 GIT_EDITOR=true GIT_SEQUENCE_EDITOR=true git rebase --continue
   1 GIT_EDITOR=true git -C /Users/bhyslop/projects/rbm_alpha_recipemuster rebase --continue
   1 git status *
   1 git stash:*
   1 git show *
   1 git push *
   1 git log *
   1 gh repo:*
   1 gh issue *
   1 gh auth:*
   1 gh api:*
   1 getent hosts *
   1 gem list *
   1 gem install *
   1 gcloud version *
   1 gcloud services:*
   1 gcloud projects:*
   1 gcloud projects *
   1 gcloud org-policies *
   1 gcloud iam:*
   1 gcloud config:*
   1 gcloud config *
   1 gcloud builds:*
   1 gcloud builds *
   1 gcloud billing *
   1 gcloud beta:*
   1 gcloud auth:*
   1 gcloud auth *
   1 gcloud artifacts:*
   1 gcloud artifacts *
   1 gcloud --quiet components install beta 2>&1
   1 gawk --version
   1 for v:*
   1 for u:*
   1 for file:*
   1 for f:*
   1 for d:*
   1 for crate:*
   1 for anchor:*
   1 find:*
   1 find Tools/vok/vov_veiled -name '*.adoc' -exec grep -l '//axhoo_output$' {}
   1 find Tools/rbk/vov_veiled Tools/vok/vov_veiled -name '*.adoc' -exec grep -l 'axhop_parameter_from_type\\|axhop_parameter_from_arg\\|axhoo_output_of_type' {}
   1 find Tools/rbk Tools/buk -name *.sh -type f -exec grep -Hn "^\\s*sed\\||\\s*sed" {}
   1 find Tools/rbk Tools/buk -name *.sh -type f -exec grep -Hn 'sed ' {}
   1 find Tools/rbk -name '*.py'
   1 find Tools/buk Tools/rbk -name "*.sh" -type f -exec grep -l "^[a-z]*_kindle\(\)" {} \\; | head -20
   1 find /var/folders -maxdepth 5 -name rbtd-* -type d
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/tt -name jja-* -o -name jjc-*
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvc -name *.rs -type f
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtid -type f -name *.py -o -name *.txt
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtd/src -type f -name *.rs
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtd/src -name *.rs -type f
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgjv -type f -name "*.sh" -exec wc -lc {} +
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgjm -type f -name "*.sh" -exec wc -lc {} +
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgje -type f -name "*.sh" -exec wc -lc {} +
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgjb -type f -name "*.sh" -exec wc -lc {} +
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgja -type f -name "*.sh" -exec wc -lc {} +
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk -name rbtd* -type f
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk/vov_veiled -name *Gallops* -o -name *Data*
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk -name *.rs -type f
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk -name "*.sh" -type f ! -path "*ABANDONED*" -exec grep -l 'if [a-z_][a-z_0-9]*;' {} \\; 2>/dev/null | head -20
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk -type f -name *.sh
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk -name "*resentation*" -o -name "*bupr*" 2>/dev/null | head -5
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools -name *guard* -o -name *size*
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools -name *enrollment* -o -name *Enrollment*
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels -type f -name *kludge* -o -name *compose*
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels -name "rbrv.env" -type f -exec head -30 {} +
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/Memos -type f -name "*.md" 2>/dev/null | head -20
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/.rb -type f -name "rbrn*.env" | head -1 | xargs cat
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/.rb -name "*.env" -type f 2>/dev/null | head -5
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/.claude/jjm -type f -name *.md
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/.claude/jjm -name *.md -o -name *.json
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/.claude/commands -type f -name "*.md" 2>/dev/null | sort
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster/.buk -name "rbrn*.env" -type f | head -1 | xargs cat 2>/dev/null
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f \\\(-name *registry* -o -name *ledger* -o -name *manifest* \\\)
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f \\\(-name *paddock* -o -name *current* -o -name *heat* \\\)
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f -path "*pluml*" -o -path "*plantuml*" 2>/dev/null | sort
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f -name *tR* -o -name *test*
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f -name *paddock* -o -type f -name *AvAAA*
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f -name *enshrine* -o -name *Enshrine*
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f -name *compose*.yml
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f -name "*jjx*" -o -name "*bridle*" -o -name "*chalk*" -o -name "*arm*" 2>/dev/null | grep -v node_modules | head -30
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -type d -name "rb*" 2>/dev/null | head -50
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -path *station-files* -name *.env
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -name rbrn.env* -type f
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -name launcher* -type f
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -name direct_verify.py -o -name *direct*.py
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -name *test*.sh -o -name *tcase*.sh -o -name *fixture*.sh
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -name *RBSIP* -o -name *ifrit*
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -name *.manifest -o -name *manifest*
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -name "*.md" -exec grep -l "volume\\|compose\\|rbob_charge" {} \\;
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -name "*.log" 2>/dev/null | head -5
   1 find /Users/bhyslop/projects/rbm_alpha_recipemuster -maxdepth 3 -name *gallop* -o -name *A4*
   1 find "/Users/bhyslop/projects/rbm_alpha_recipemuster" -type f \\\( -name "*.sh" -o -name "*.adoc" -o -name "*.md" -o -name "*.rs" \\\) -exec grep -l "BURN_HOST\\|BURN_USER\\|BURN_ALIAS\\|BURN_SSH_PUBKEY\\|BURN_COMMAND" {} \\;
   1 find .claude/jjm/officia/260603-1004/ -type f -size -2k
   1 find .claude/jjm/jje_* -type f
   1 find . -name 'jji_itch.md' -not -path '*/node_modules/*'
   1 find . -name 'CLAUDE*.md' -not -path '*/node_modules/*'
   1 fi
   1 external command\\|exit code\\|stderr suppres" Tools/buk/vov_veiled/BCG-BashConsoleGuide.md | head -60
   1 export SOC='echo hi | tr a-z A-Z'
   1 env
   1 echo "WSL_LIFECYCLE_EXIT=$?"
   1 echo "vvw-r dispatch exit: $?"
   1 echo "vow-b exit: $?"
   1 echo "Updated: $v"
   1 echo "unstamped check exit: $?"
   1 echo "UNIT_EXIT=$?"
   1 echo "TT_EXIT=$?"
   1 echo "THEURGE_EXIT=$?"
   1 echo "theurge tests: $?"
   1 echo "theurge tests exit: $?"
   1 echo "theurge build: $?"
   1 echo "test EXIT=$?"
   1 echo "test exit: $?"
   1 echo "TABTARGET_EXIT=$?"
   1 echo "storage.googleapis.com exit=$?"
   1 echo "status exit: $?"
   1 echo "StartHere exit: $?"
   1 echo "stale-path check exit: $?"
   1 echo "ssh exit: $? \(connection drop on reboot is expected\)"
   1 echo "SMOKE_EXIT=$?"
   1 echo "shellcheck-exit=$?"
   1 echo "SHELLCHECK_EXIT: $?"
   1 echo "shellcheck exit=$?"
   1 echo "shellcheck exit=$? \(0 = clean → gate would pass\)"
   1 echo "SHELLCHECK exit: $?"
   1 echo "SERVICE_SUITE_EXIT=$?"
   1 echo "SELFTEST_EXIT=$?"
   1 echo "RRV_EXIT=$?"
   1 echo "rrv exit: $?"
   1 echo "rrr exit: $?"
   1 echo "ROSTER_EXIT=$?"
   1 echo "RNV_EXIT=$?"
   1 echo "RNV_CCYOLO_EXIT=$?"
   1 echo "RFV_EXIT=$?"
   1 echo "retriever divest rc=$?"
   1 echo "restore build exit=$?"
   1 echo "RELIQUARY_LIFECYCLE_EXIT=$?"
   1 echo "refresh exit=$?"
   1 echo "REBASE_EXIT=$?"
   1 echo "rdv exit: $?"
   1 echo "RC=$?"
   1 echo "rc=$? \(empty above = clean\)"
   1 echo "RC_EXIT=$?"
   1 echo "rbw-tq exit: $?"
   1 echo "rbw-Ofc exit: $?"
   1 echo "rbw-Odf exit: $?"
   1 echo "rbw-o exit: $?"
   1 echo "rbw-hw RC=$? \(rbhw0 furnish — sources rbhw0_Windows.sh\)"
   1 echo "rbw-hw exit: $?"
   1 echo "rbw-gq RC=$? \(rbhp0 furnish — sources rbhp0_Payor.sh\)"
   1 echo "rbw-fhv rbev-busybox RC=$?  \(rbfh furnish — sources rbfc0_FoundryCore.sh\)"
   1 echo "RBFL_SMOKE_EXIT=$?"
   1 echo "RBFC_SMOKE_EXIT=$?"
   1 echo "RAV_EXIT=$?"
   1 echo "quota exit=$?"
   1 echo "QUALIFY_FAST_EXIT=$?"
   1 echo "QF_EXIT=$?"
   1 echo "push-exit=$?"
   1 echo "PROBE_EXIT=$?"
   1 echo "POISON_EXIT=$?"
   1 echo "PLUML_FIXTURE_EXIT=$?"
   1 echo "Occ exit=$?"
   1 echo "o exit=$?"
   1 echo "merge-exit=$?"
   1 echo "merge exit: $?"
   1 echo "MANTLE_EXIT=$?"
   1 echo "MACOS_DOGFIGHT2_EXIT=$?"
   1 echo "ls-grep exit: $?"
   1 echo "LODE_LIFECYCLE_EXIT=$?"
   1 echo "local: $\(shellcheck --version
   1 echo "JILT_RERUN_EXIT: $?"
   1 echo "JILT_EXIT: $?"
   1 echo "INVEST_EXIT=$?"
   1 echo "IMMURE_EXIT=$?"
   1 echo "HEADLESS_EXIT=$?"
   1 echo "HEAD: $\(git -C /home/bhyslop/projects/rbm_alpha_recipemuster rev-parse --short HEAD\)"
   1 echo "harness exit: $?"
   1 echo "harness exit \(1 = diffs present, expected\): $?"
   1 echo "HANDBOOK_EXIT=$?"
   1 echo "grep exit: $?"
   1 echo "GOV_PROBE_EXIT=$?"
   1 echo "FRESH_EXIT=$?"
   1 echo "FIXTURE_EXIT:$?"
   1 echo "FIXTURE_EXIT: $?"
   1 echo "FAST_SUITE_EXIT=$?"
   1 echo "fast suite exit: $?"
   1 echo "FAST exit: $?"
   1 echo "exit=$? \(nonzero/empty = clean\)"
   1 echo "exit=$? \(grep: 1 = zero hits = free\)"
   1 echo "EXIT=$? \(expect nonzero fail-loud, no hang\)"
   1 echo "EXIT=$? \(expect 104 = BUBC_band_credless\)"
   1 echo "exit=$? \(empty above = gallops unchanged on disk\)"
   1 echo "exit=$? \(1 = no hits\)"
   1 echo "exit=$? — empty above = clean tree"
   1 echo "EXIT:$?"
   1 echo "exit: $?" echo '=== Done ==='
   1 echo "exit: $?" echo '--- 7. Stale {vessel}:{hallmark}-X ---' grep -rnE '\\{vessel\\}:\\{hallmark\\}-\(image|vouch|pouch|about|diags\)' Tools/rbk/vov_veiled/
   1 echo "exit: $?" echo '--- 6. Stale «HALLMARK»-image / «HALLMARK»-about etc. ---' grep -rnE '«HALLMARK»-\(image|vouch|pouch|about|diags\)' Tools/rbk/vov_veiled/
   1 echo "exit: $?" echo '--- 5. Vouches aggregator phrasing ---' grep -rnE 'vouches \(superdirectory|package|aggregator\)' Tools/rbk/vov_veiled/
   1 echo "exit: $?" echo '--- 4. Old enshrine tag grammar ---' grep -rnE 'enshrine:\\{?«?[Aa]nchor' Tools/
   1 echo "exit: $?" echo '--- 3. Old hyphen-suffix tag concat ---' grep -rnE ':\\$?\\{[^}]*HALLMARK[^}]*\\}-\(image|vouch|pouch|about|diags\)' Tools/ .rbk/
   1 echo "exit: $?" echo '--- 2. RBGC_VOUCHES_PACKAGE ---' grep -rn 'RBGC_VOUCHES_PACKAGE' Tools/
   1 echo "exit: $?"
   1 echo "exit: $? \(1 = zero hits, good\)"
   1 echo "exit: $? \(1 = truly none\)"
   1 echo "exit: $? \(1 = none, as desired\)"
   1 echo "exit: $? \(1 = no matches = clean\)"
   1 echo "exit: $?  \(no hits = clean gate\)"
   1 echo "exit: $?  \(1 = no matches, good\)"
   1 echo "EXIT_WRITE=$?"
   1 echo "EXIT_RUN=$?"
   1 echo "EXIT_RM=$?"
   1 echo "EXIT_acr=$?"
   1 echo "EXIT_acg=$?"
   1 echo "EXIT_acd=$?"
   1 echo "Exit codes: $?"
   1 echo "Exit code: $?"
   1 echo "exit code: $?"
   1 echo "exit $?"
   1 echo "example.com exit=$?"
   1 echo "EV_EXIT=$?"
   1 echo "establish exit=$?"
   1 echo "empty-arg exit=$?"
   1 echo "DRIFT_EXIT=$?"
   1 echo "DRIFT_EXIT=$? \(expect 1\)"
   1 echo "DOGFIGHT_EXIT=$?"
   1 echo "DIVINE_EXIT=$?"
   1 echo "DIRECTOR_PROBE_EXIT=$?"
   1 echo "director divest rc=$?"
   1 echo "DIR_PROBE_EXIT=$?"
   1 echo "diff-refresh-exit=$?"
   1 echo "diff-establish-exit=$?"
   1 echo "DH_EXIT=$?"
   1 echo "deleted rbw-iae tabtarget: $?"
   1 echo "curia-tip=$\(git -C /Users/bhyslop/projects/rbm_beta_recipemuster rev-parse HEAD\)"
   1 echo "CREDLESS_EXIT=$?"
   1 echo "CrashCourse exit: $?"
   1 echo "clean: $?"
   1 echo "census-clean: $?"
   1 echo "census exit: $?"
   1 echo "CANONICAL_INVEST_EXIT=$?"
   1 echo "buw-st exit: $?"
   1 echo "buw-rpv bujn-winpc exit=$?"
   1 echo "buw-rnv bujn-winpc exit=$?"
   1 echo "BUILD_EXIT:$?"
   1 echo "build EXIT=$?"
   1 echo "BUILD exit: $?"
   1 echo "beta HEAD:  $\(git -C /Users/bhyslop/projects/rbm_beta_recipemuster rev-parse HEAD\)"
   1 echo "BARK_EXIT=$?"
   1 echo "BANISH_EXIT=$?"
   1 echo "asciidoctor: $\(asciidoctor --version
   1 echo "alpha HEAD: $\(git -C /Users/bhyslop/projects/rbm_alpha_recipemuster rev-parse HEAD\)"
   1 echo "ACG_EXIT=$?"
   1 echo "ACF_EXIT=$?"
   1 echo "==LODE_LIFECYCLE_EXIT=$?=="
   1 echo "==DOGFIGHT_EXIT=$?=="
   1 echo "===SHUTDOWN EXIT=$?==="
   1 echo "===RESTART EXIT=$?==="
   1 echo "===NTUSER UNLOAD EXIT=$?==="
   1 echo "===KNOCK2 EXIT=$?==="
   1 echo "===KNOCK1 EXIT=$?==="
   1 echo "===KNOCK EXIT=$?==="
   1 echo "===EXIT $?==="
   1 echo "===EXIT $?"
   1 echo "===CLASSES UNLOAD EXIT=$?==="
   1 echo "===BACKUP EXIT=$?==="
   1 echo "===APPEND EXIT=$?==="
   1 echo "===== merge exit: $? ====="
   1 echo "=== TEST EXIT: $? ==="
   1 echo "=== TABTARGET EXIT $? ==="
   1 echo "=== SUITE EXIT: $? ==="
   1 echo "=== ssh exit: $? ==="
   1 echo "=== SHELLCHECK EXIT: $? ==="
   1 echo "=== shellcheck exit: $? ==="
   1 echo "=== REBASE EXIT: $? ==="
   1 echo "=== rbtd-t exit $? ==="
   1 echo "=== qualify exit: $? ==="
   1 echo "=== PUSH EXIT: $? ==="
   1 echo "=== podvm-lifecycle fixture exit: $? ==="
   1 echo "=== native immure exit: $? ==="
   1 echo "=== MERGE EXIT: $? ==="
   1 echo "=== merge exit: $? ==="
   1 echo "=== KludgeSentry exit: $? ==="
   1 echo "=== KludgeBottle exit: $? ==="
   1 echo "=== KLUDGE-SENTRY EXIT: $? ==="
   1 echo "=== grep exit \(1 = no matches, clean\): $? ==="
   1 echo "=== generate exit: $? ==="
   1 echo "=== FAST SUITE EXIT: $? ==="
   1 echo "=== fast suite exit: $? ==="
   1 echo "=== fast exit $? ==="
   1 echo "=== exit: $? \(empty diff above = both files now match pre-pace\) ==="
   1 echo "=== exit: $? \(1 = no matches = clean\) ==="
   1 echo "=== exit code: $? ==="
   1 echo "=== EXIT $? ==="
   1 echo "=== exit $? \(1=no matches, good\) ==="
   1 echo "=== emplace exit: $? ==="
   1 echo "=== CHARGE EXIT: $? ==="
   1 echo "=== Charge exit: $? ==="
   1 echo "=== build exit: $? ==="
   1 echo "=== banish exit: $? ==="
   1 echo "=== augur exit: $? ==="
   1 echo "\(reboot command exit: $?\)"
   1 echo "\(grep exit $?\)"
   1 echo "\(exit $?\)"
   1 echo "\(exit $? — grep exit 1 = no matches = good\)"
   1 echo "\(exit $? — 1 means clean\)"
   1 echo "\(context: $\(docker context show \)\)"
   1 echo "---TT EXIT: $?---"
   1 echo "---TS EXIT: $?---"
   1 echo "---TQ EXIT: $?---"
   1 echo "---rebase exit: $?---"
   1 echo "---push exit: $?---"
   1 echo "---grep exit: $?"
   1 echo "---exit=$?"
   1 echo "---EXIT=$?---"
   1 echo "---exit=$?---"
   1 echo "---EXIT:$?"
   1 echo "---EXIT:$?---"
   1 echo "---exit: $?"
   1 echo "---EXIT: $?---"
   1 echo "---exit: $?---"
   1 echo "---BUILD EXIT: $?---"
   1 echo "----- EXIT $? -----"
   1 echo "--- exit: $? \(1 = no matches = clean\) ---"
   1 echo "--- exit: $? \(1 = no hits\) ---"
   1 echo "--- exit $?"
   1 echo " [exit $?]"
   1 echo "  renamed in $f"
   1 echo "  renamed bare output in $f"
   1 echo "  exit=$?  \(expect empty / nonzero\)"
   1 echo "  exit=$?  \(expect abc123 / 0\)"
   1 echo "  exit=$?  \(expect 'tok 3600' / 0\)"
   1 echo '=== OP*_ TERM REFERENCES \(opbs_, opbr_, opss_\) BY FILE ===' grep -r "opbs_\\|opbr_\\|opss_" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.json
   1 echo '=== MCM TIER TERMS \(lemma/lemmata/graven/intaglio/quoin/sprue/inlay\) ===' echo 'Searching for MCM vocabulary...' grep -r "lemma\\|lemmata\\|graven\\|intaglio\\|quoin\\|sprue\\|inlay" /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools --include=*.adoc --include=*.md
   1 echo '=== FRONTISPIECE REFERENCES BY FILE ===' grep -r "ConnectBottle\\|ConnectCenser\\|ConnectSentry\\|ObserveNetworks" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.yml --include=*.json
   1 echo '=== CENSER REFERENCES BY FILE ===' grep -r censer /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.yml --include=*.json
   1 echo '=== BOTTLE_START/RUN REFERENCES BY FILE ===' grep -r "bottle_start\\|bottle_run\\|bottle_service" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.json
   1 echo '=== AT_ SERVICE TERMS \(at_bottle_service, at_censer_container, at_agile_service, at_sessile_service\) BY FILE ===' grep -r "at_bottle_service\\|at_censer_container\\|at_agile_service\\|at_sessile_service" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.json
   1 DV
   1 dseditgroup:*
   1 dscacheutil -q host -a name rocket
   1 done
   1 docker version:*
   1 docker version *
   1 docker tag:*
   1 docker system:*
   1 docker start:*
   1 docker run:*
   1 docker run *
   1 docker rmi:*
   1 docker rmi *
   1 docker rm:*
   1 docker pull:*
   1 docker ps:*
   1 docker network:*
   1 docker network *
   1 docker manifest:*
   1 docker info:*
   1 docker images:*
   1 docker image:*
   1 docker exec:*
   1 docker create:*
   1 docker container *
   1 docker compose:*
   1 docker buildx *
   1 docker build:*
   1 docker --version
   1 do sed:*
   1 do printf:*
   1 do if:*
   1 do grep:*
   1 do echo:*
   1 dns-sd -t 5 -B _services._dns-sd._udp local.
   1 dns-sd -t 3 -B _ssh._tcp
   1 dns-sd -t 3 -B _smb._tcp
   1 diskutil list:*
   1 dig +short:*
   1 dig +short A claude.ai
   1 dig +short A api.claude.ai
   1 dig +short A api.anthropic.com
   1 dig +short A anthropic.com
   1 DE
   1 curl:*
   1 curl -sSL -A 'Mozilla/5.0' 'https://cloud.google.com/iam/docs/federated-identity-supported-services' -o /tmp/rbspike/matrix.html
   1 curl -sS 'https://cloud.google.com/iam/docs/federated-identity-supported-services' -o /tmp/rbspike/matrix.html
   1 curl -sS -X POST https://sts.googleapis.com/v1/token -H 'Content-Type: application/x-www-form-urlencoded' --data-urlencode grant_type=urn:ietf:params:oauth:grant-type:token-exchange --data-urlencode audience=//iam.googleapis.com/locations/global/workforcePools/spike-office-test/providers/spike-entra --data-urlencode requested_token_type=urn:ietf:params:oauth:token-type:access_token --data-urlencode subject_token_type=urn:ietf:params:oauth:token-type:id_token --data-urlencode subject_token=__CMDSUB_OUTPUT__ --data-urlencode scope=https://www.googleapis.com/auth/cloud-platform
   1 curl -sS -X POST -H 'Authorization: Bearer __TRACKED_VAR__' -H 'Content-Type: application/json' https://logging.googleapis.com/v2/entries:list -d '{ *
   1 curl -sS -X POST -H 'Authorization: Bearer __TRACKED_VAR__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/p7_new.json
   1 curl -sS -X POST -H 'Authorization: Bearer __TRACKED_VAR__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:getIamPolicy -d '{}'
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://logging.googleapis.com/v2/entries:list -d '{ *
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://iam.googleapis.com/v1/projects/cancbhm-d-canest3bhm100001/serviceAccounts -d '{"accountId": "spike-office-test", "serviceAccount": {"displayName": "spike-office-test", "description": "Federation spike test office SA \(heat BZ pace BZAAA\)"}}'
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://iam.googleapis.com/v1/locations/global/workforcePools/spike-office-test/providers?workforcePoolProviderId=spike-entra -d '{ *
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://iam.googleapis.com/v1/locations/global/workforcePools?workforcePoolId=spike-office-test -d '{ *
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/p8_new.json
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/p6_new.json
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/depot_policy5_new.json
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/depot_policy4_new.json
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:getIamPolicy -d '{}'
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:getIamPolicy -d '{"options":{"requestedPolicyVersion":3}}'
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v1/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/depot_policy3_new.json
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v1/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/depot_policy_new.json
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v1/projects/cancbhm-d-canest3bhm100001:getIamPolicy -d '{}'
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v1/organizations/247899326218:setIamPolicy -d @/tmp/rbspike/org_policy_new.json
   1 curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v1/organizations/247899326218:getIamPolicy -d '{}'
   1 curl -sS -H 'Authorization: Bearer __TRACKED_VAR__' https://artifactregistry.googleapis.com/v1/projects/cancbhm-d-canest3bhm100001/locations/us-central1/repositories
   1 curl -sS -H 'Authorization: Bearer __CMDSUB_OUTPUT__' https://iam.googleapis.com/v1/locations/global/workforcePools?parent=organizations/247899326218
   1 curl -sS -H 'Authorization: Bearer __CMDSUB_OUTPUT__' https://cloudresourcemanager.googleapis.com/v1/organizations:search -X POST -H 'Content-Type: application/json' -d '{}'
   1 curl -sS -H 'Authorization: Bearer __CMDSUB_OUTPUT__' https://artifactregistry.googleapis.com/v1/projects/cancbhm-d-canest3bhm100001/locations/us-central1/repositories
   1 curl -o /dev/null -w "http_code=%{http_code} time_total=%{time_total}\\n" --max-time 15 https://cloudbuild.googleapis.com/
   1 curl -o /dev/null -w "http_code=%{http_code} time_total=%{time_total} time_connect=%{time_connect} time_appconnect=%{time_appconnect}\\n" --max-time 15 https://us-central1-docker.pkg.dev/v2/
   1 cp /Users/bhyslop/projects/station-files/secrets/director/rbra.env /Users/bhyslop/projects/station-files/secrets/director/rbra.env.bak-20260516-stale100009
   1 cp /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rboc_censer.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels/rbev-sentry-debian-slim/rboc_censer.sh
   1 cp /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbj_sentry.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels/rbev-sentry-debian-slim/rbj_sentry.sh
   1 cp ../output-buk/current/epic_progress_note.stanford.assay.txt ../output-buk/current/epic_geriatric_consult.stanford.assay.txt /tmp/apcnsa_compare/
   1 cp ../output-buk/current/epic_geriatric_consult.assay.txt ../output-buk/current/epic_progress_note.assay.txt /tmp/apcnsa_compare/
   1 conjure
   1 command -v syft
   1 command -v ruby gem asciidoctor
   1 command -v gem
   1 command -v gcloud
   1 command -v bundle
   1 command -v asciidoc
   1 claude:*
   1 claude mcp *
   1 chmod:*
   1 chmod +x:*
   1 chmod +x tt/rbw-rfr.RenderFederationRegime.sh tt/rbw-rfv.ValidateFederationRegime.sh
   1 chmod +x tt/rbw-lU.DirectorUnderpinsWsl.sh
   1 chmod +x tt/rbw-lE.DirectorEnsconcesBase.sh tt/rbw-ld.DirectorDivinesLodes.sh tt/rbw-lB.DirectorBanishesLode.sh
   1 chmod +x tt/rbw-lC.DirectorConclavesReliquary.sh
   1 chmod +x tt/rbw-il.DirectorListsRegistry.sh tt/rbw-iw.DirectorWrestsImage.sh tt/rbw-iJ.DirectorJettisonsImage.sh
   1 chmod +x tt/rbw-acf.CheckFederatedAccess.sh
   1 chmod +x Tools/rbk/rbrf_regime.sh
   1 chmod +x Tools/rbk/rbrf_cli.sh
   1 chmod +x Tools/rbk/rbld_cli.sh
   1 chmod +x /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-mA.PayorAffiancesManor.sh
   1 cd /Users/bhyslop/projects/rbm_beta_recipemuster
   1 cd /Users/bhyslop/projects/rbm_alpha_recipemuster
   1 cd /home/bhyslop/projects/rbm_alpha_recipemuster
   1 cd /dev/null
   1 cat:*
   1 cat rbmm_moorings/tadmor/rbnnh_compose.yml
   1 cat /var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-4095/burv/invoke-00000/output/*.txt
   1 cat "../temp-buk/temp-20260609-104733-83286-60/rbtd/rbtdrc_reliquary_lifecycle/05-banish.txt"
   1 cat __TRACKED_VAR__/burv-output/invoke-00000/previous/rbf_fact_lode_brand
   1 cargo test:*
   1 cargo tauri:*
   1 cargo run:*
   1 cargo install:*
   1 cargo check:*
   1 cargo build:*
   1 cargo build *
   1 cargo +nightly fmt --version
   1 cargo --version
   1 BURE_VERBOSE=1 tt/apcw-b.Build.sh
   1 BURE_TWEAK_NAME=buost_regime_poison BURE_TWEAK_VALUE="RBRF_WORKFORCE_POOL_ID=rbproof-aff-0617070756" BURE_CONFIRM=skip tt/rbw-mJ.PayorJiltsManor.sh
   1 BURE_TWEAK_NAME=buorb_credless_guard tt/rbw-acf.CheckFederatedAccess.sh
   1 BURE_TWEAK_NAME=buorb_credless_guard ./tt/rbw-acf.CheckFederatedAccess.sh
   1 BURE_COUNTDOWN=skip tt/rbw-DI.DirectorInscribesReliquary.sh 2>&1
   1 BURE_COUNTDOWN=skip tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-busybox 2>&1
   1 BURE_COUNTDOWN=skip tt/rbw-DC.DirectorCreatesConsecration.sh rbev-vessels/rbev-busybox 2>&1
   1 BURE_COUNTDOWN=skip BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rv.RetrieverVouchesArk.sh:*
   1 BURE_COUNTDOWN=skip BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DP.DirectorRefreshesPins.sh:*
   1 BURE_COUNTDOWN=skip BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DI.DirectorInscribesRubric.sh:*
   1 BURE_COUNTDOWN=skip BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorConjuresArk.sh:*
   1 BURE_COUNTDOWN=skip BURE_CONFIRM=skip ./tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-sentry-debian-slim
   1 BURE_COUNTDOWN=skip BURE_CONFIRM=skip ./tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-bottle-ifrit
   1 BURE_COUNTDOWN=skip BURE_CONFIRM=skip ./tt/rbw-DC.DirectorCreatesConsecration.sh rbev-vessels/rbev-sentry-debian-slim
   1 BURE_COUNTDOWN=skip BURE_CONFIRM=skip ./tt/rbw-DC.DirectorCreatesConsecration.sh rbev-vessels/rbev-bottle-ifrit
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.regime-validation.sh
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.ark-lifecycle.sh
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvv.ValidateVesselRegime.sh rbev-vessels/rbev-sentry-ubuntu-large
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvv.ValidateVesselRegime.sh rbev-vessels/rbev-busybox-airgap-negative-canary
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvv.ValidateVesselRegime.sh rbev-vessels/rbev-busybox
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvv.ValidateVesselRegime.sh rbev-busybox-airgap-negative-canary
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvv.ValidateVesselRegime.sh rbev-busybox
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvv.ValidateVesselRegime.sh rbev-bottle-plantuml
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rrv.ValidateRepoRegime.sh
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rrr.RenderRepoRegime.sh
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rgv.ValidatePinsRegime.sh
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DPG.DirectorRefreshesGcbPins.sh
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DI.DirectorInscribesRubric.sh
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Dc.DirectorChecksConsecrations.sh
   1 BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rev.ValidateEnvironmentRegime.sh
   1 BURE_COUNTDOWN=skip ./tt/rbw-DPG.DirectorRefreshesGcbPins.sh:*
   1 BURE_COUNTDOWN=skip ./tt/rbw-DPB.DirectorRefreshesBinaryPins.sh:*
   1 BURE_COUNTDOWN=skip ./tt/rbw-DP.DirectorRefreshesPins.sh:*
   1 BURE_CONFIRM=skip tt/rbw-ts.TestSuite.siege.sh *
   1 BURE_CONFIRM=skip tt/rbw-ts.TestSuite.dogfight.sh *
   1 BURE_CONFIRM=skip tt/rbw-tP.QualifyPristine.sh *
   1 BURE_CONFIRM=skip tt/rbw-tf.FixtureRun.sh lode-lifecycle *
   1 BURE_CONFIRM=skip tt/rbw-PG.PayorResetsGovernor.sh 2>&1
   1 BURE_CONFIRM=skip tt/rbw-MR.MarshalReset.sh
   1 BURE_CONFIRM=skip tt/rbw-lB.DirectorBanishesLode.sh vw260608213906
   1 BURE_CONFIRM=skip tt/rbw-lB.DirectorBanishesLode.sh vn260608213343
   1 BURE_CONFIRM=skip tt/rbw-lB.DirectorBanishesLode.sh vn
   1 BURE_CONFIRM=skip tt/rbw-GR.GovernorCreatesRetriever.sh mac1 2>&1
   1 BURE_CONFIRM=skip tt/rbw-GD.GovernorCreatesDirector.sh mac1 2>&1
   1 BURE_CONFIRM=skip tt/rbw-dU.PayorUnmakesDepot.sh *
   1 BURE_CONFIRM=skip tt/rbw-aM.PayorMantlesGovernor.sh
   1 BURE_CONFIRM=skip BURE_COUNTDOWN=skip tt/rbw-dU.PayorUnmakesDepot.sh rbwg-d-depot10041-260327170532 *
   1 BURE_CONFIRM=skip BURE_COUNTDOWN=skip tt/rbw-dU.PayorUnmakesDepot.sh depot10041 *
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-MZ.MarshalZeroes.sh *
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-lB.DirectorBanishesLode.sh r260609140912 *
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-fA.DirectorAbjuresHallmark.sh c260603134154-r260603134156 *
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh prlc-d-pristl100000
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh canc-d-canest2100006
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh canc-d-canest2100005
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh canc-d-canest2100004
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh canc-d-canest2100003
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh canc-d-canest2100002
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh canc-d-canest2100001
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.dogfight.sh
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PD.PayorDestroysDepot.sh rbwg-d-depot10040-260323094315
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PD.PayorDestroysDepot.sh rbwg-d-depot10030-260312153057 2>&1
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-lB.DirectorBanishesLode.sh vn260610115913
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fA.DirectorAbjuresHallmark.sh rbev-busybox:*
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-arr.GovernorRostersRetrievers.sh
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-arD.GovernorDivestsRetriever.sh canest-ret
   1 BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-adD.GovernorDivestsDirector.sh canest-dir
   1 BURE_CONFIRM=skip ./tt/rbw-ts.TestSuite.dogfight.sh *
   1 BURE_CONFIRM=skip ./tt/rbw-tP.QualifyPristine.sh
   1 BURE_CONFIRM=skip ./tt/rbw-tf.FixtureRun.sh pluml
   1 BURE_CONFIRM=skip ./tt/rbw-tf.FixtureRun.sh lode-lifecycle *
   1 BURE_CONFIRM=skip ./tt/rbw-tf.FixtureRun.sh canonical-invest
   1 BURE_CONFIRM=skip ./tt/rbw-MZ.MarshalZeroes.sh *
   1 BURE_CONFIRM=skip ./tt/rbw-MZ.MarshalZeroes.sh
   1 BURE_CONFIRM=skip ./tt/rbw-lB.DirectorBanishesLode.sh r260609140912
   1 BURE_CONFIRM=skip ./tt/rbw-lB.DirectorBanishesLode.sh r260609104734 *
   1 BURE_CONFIRM=skip ./tt/rbw-lB.DirectorBanishesLode.sh b260608094131 *
   1 BURE_CONFIRM=skip ./tt/rbw-iJr.DirectorJettisonsReliquaryImage.sh *
   1 BURE_CONFIRM=skip ./tt/rbw-fA.DirectorAbjuresHallmark.sh rbev-busybox:*
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh prlcbhm-d-pristlbhm100000 *
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh prlc-d-pristl100005 *
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh prlc-d-pristl100005
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhm-d-canest3bhm100000
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhm-d-canest2bhm100002 *
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhm-d-canest2bhm100001 *
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhm-d-canest2bhm100000 *
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhm100002 *
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100007 *
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100006 *
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100005 *
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100003 *
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100002 *
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100001 *
   1 BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100000 *
   1 BURE_CONFIRM=skip ./tt/rbw-cQ.Quench.tadmor.sh
   1 BURE_CONFIRM=skip ./tt/rbw-cKS.KludgeSentry.sh tadmor *
   1 BURE_CONFIRM=skip ./tt/rbw-cKB.KludgeBottle.sh tadmor *
   1 BURE_CONFIRM=skip ./tt/rbw-cC.Charge.tadmor.sh *
   1 BURE_CONFIRM=skip ./tt/rbw-cC.Charge.pluml.sh
   1 BURE_CONFIRM=skip ./tt/rbtd-r.FixtureRun.moriah.sh *
   1 BURD_NO_HYPERLINKS=1 tt/rbw-Occ.OnboardingCrashCourse.sh:*
   1 BURD_LAUNCHER=".buk/launcher.rbw_workbench.sh" .buk/launcher.rbw_workbench.sh rbw-dY.DirectorYokesReliquaryInVessel.sh --help
   1 BURD_LAUNCHER=".buk/launcher.buw_workbench.sh" BURD_NO_LOG=1 ./.buk/launcher.buw_workbench.sh "buw-jwk"
   1 BURD_LAUNCHER=".buk/launcher.buw_workbench.sh" BURD_NO_LOG=1 ./.buk/launcher.buw_workbench.sh "buw-jwc"
   1 build
   1 BUC_VERBOSE=0 ./tt/buw-jpF.Fenestrate.sh
   1 brew list:*
   1 brew list *
   1 brew install *
   1 break
   1 bash:*
   1 bash /tmp/validation.sh
   1 bash /tmp/rbk_dirlog.sh 73a38a23-8901-471f-a20c-6e25fa067908
   1 bash /tmp/detailed_analysis.sh
   1 bash /tmp/count_roles.sh
   1 bash /tmp/clusters.sh
   1 bash /tmp/check_drifts.sh
   1 bash -n Tools/rbk/rbz_zipper.sh
   1 bash -n Tools/rbk/rbrr_regime.sh
   1 bash -n Tools/rbk/rbob_bottle.sh
   1 bash -n Tools/rbk/rbgp_payor.sh
   1 bash -n Tools/rbk/rbgg_governor.sh
   1 bash -n Tools/rbk/rbcc_Constants.sh
   1 bash -n Tools/buk/buym_yelp.sh
   1 bash -n Tools/buk/burs_regime.sh
   1 bash -n Tools/buk/buc_command.sh
   1 bash -n /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbgp_Payor.sh
   1 bash -n /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/bujb_jurisdiction.sh
   1 bash -n /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/buc_command.sh
   1 bash -n /Users/bhyslop/projects/rbm_beta_recipemuster/__TRACKED_VAR__
   1 bash -n /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgi_IAM.sh
   1 bash -c 'source Tools/rbk/rbz_zipper.sh && zrbz_kindle && echo "${ZRBZ_COLOPHON_MANIFEST} rbtd-ap"'
   1 bash -c 'source ../buk/buc_command.sh; source rbld0_Lode.sh && declare -F zrbld_cloud_delete_dispatch zrbld_spine_dispatch rbld_banish >/dev/null && echo "RBLD OK: ${ZRBLDS_SOURCED:-unset}/${ZRBLDD_SOURCED:-unset} guards set, delete+spine+banish defined"'
   1 bash -c 'source ../buk/buc_command.sh; source rbfl0_FoundryLedger.sh && declare -F zrbld_cloud_delete_dispatch zrbld_spine_dispatch rbfl_abjure >/dev/null && echo "RBFL OK: ${ZRBLDS_SOURCED:-unset}/${ZRBLDD_SOURCED:-unset} guards set, delete+spine+abjure defined"'
   1 awk NR>=750 && NR<=950 *
   1 awk NR>=6 && NR<=22 {printf "%d|%s\\n", NR, $0} *
   1 awk NR>=560 && NR<=750 *
   1 awk NR>=155 && NR<=275 {printf "%d\\t%s\\n", NR, $0} *
   1 awk NR>=120 && NR<=200 *
   1 awk *
   1 awk 'NR>581' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
   1 awk 'NR>=980 && NR<=1145 && /^===? /{print NR": "$0}' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc
   1 awk 'NR>=9 && NR<=581' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
   1 awk 'NR>=9 && NR<=581' RBS0-SpecTop.adoc
   1 awk 'NR>=600 && NR<=900 && /recipe|hygiene|dockerfile|RECIPE|HYGIENE|DOCKERFILE|expect_code|BAND/' Tools/rbk/rbtd/src/rbtdrf_fast.rs
   1 awk 'NR>=540 && NR<=575 {print NR": "$0}' README.md
   1 awk 'NR>=200' /tmp/rbtds-service-260425-1004.log
   1 awk 'NR>=158' /tmp/rbtds-service-260425-1004.log
   1 awk 'NR>=1540 && NR<=1600' Tools/rbk/rbtd/src/rbtdrf_fast.rs
   1 awk 'NR>=1 && NR<=260 && /^[a-zA-Z_]+\\\(\\\)|^z?rbgc_[a-zA-Z_]*\\\(\\\)|_kindle.*\\\(\\\)|function /{print NR": "$0}' Tools/rbk/rbgc_Constants.sh
   1 awk 'NR<=571 && /^#/ {h=NR": "$0} END{print h}' README.md
   1 awk 'NR<=571 && /^#/ {h=$0} END{print "heading text: "h}' README.md
   1 awk 'NR<=219 && /\\\(\\\)[[:space:]]*\\{?[[:space:]]*$|\\\(\\\)[[:space:]]*\\{/{last=NR": "$0} END{print last}' Tools/rbk/rbgc_Constants.sh
   1 awk 'NR<=200 && /^[a-z_]+\\\(\\\) \\{/ {f=$1} NR==196 {print "Line 196 is in function:", f}' Tools/rbk/rbgg_Governor.sh
   1 awk 'NR<=1500 && /^[a-zA-Z_]+\\\(\\\)|^rbfd_|^zrbfd_/ {print NR":"$0}' Tools/rbk/rbfd_FoundryDirectorBuild.sh
   1 awk 'NR<=1012 && /^== /{h=$0; n=NR} END{print n": "h}' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc
   1 awk 'NR < 2607 || NR > 3033 { print }' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/src/rbtdrc_crucible.rs
   1 awk '$1+0 >= 1250'
   1 awk '/tag::mapping-section/,/end::mapping-section/' Tools/apck/APCS0-SpecTop.adoc
   1 awk '/rbld_banish\\\(\\\)/,/^}/' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbld_Lode.sh
   1 awk '/name: "siege"/,/\\]/' Tools/rbk/rbtd/src/rbtdrc_crucible.rs
   1 awk '/Kludged/,/They diverge/'
   1 awk '/fn rbtdro_onboarding_ordain_bind_impl/,/^}$/' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
   1 awk '/fn rbtdrc_pluml/{f=NR} END{print "first pluml fn at line "f}' Tools/rbk/rbtd/src/rbtdrc_crucible.rs
   1 awk '/Director Subtracks/,/^$/'
   1 awk '/^zrbgp_depot_state_emit\\\(\\\)/,/^# Post-lifecycle hook/' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
   1 awk '/^rbgp_depot_unmake\\\(\\\)/,/^rbgp_depot_list\\\(\\\)/' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbgp_Payor.sh
   1 awk '/^rbgp_depot_unmake\\\(\\\)/,/^rbgp_depot_list\\\(\\\)/' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
   1 awk '/^rbgp_depot_list\\\(\\\)/,/^rbgp_payor_oauth_refresh\\\(\\\)/' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
   1 awk '/^rbgp_depot_list\\\(\\\)/,/^}/' Tools/rbk/rbgp_Payor.sh
   1 awk '/^rbgp_depot_levy\\\(\\\)/,/^rbgp_depot_unmake\\\(\\\)/' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
   1 awk '/^rbgo_get_token_capture/{f=1} f{print NR": "$0} f&&/^}/{exit}' Tools/rbk/rbgo_OAuth.sh
   1 awk '/^rbgg_invest_retriever\\\(\\\)/,/^}/' Tools/rbk/rbgg_governor.sh
   1 awk '/^rbgg_invest_director\\\(\\\)/,/^}/' Tools/rbk/rbgg_governor.sh
   1 awk '/^In .*line [0-9]+:/{ split\($0,a,"line "\); ln=a[2]+0; if \(\(ln>=159 && ln<=273\) || \(ln>=1058 && ln<=1120\)\) {p=1; print "----"; print} else p=0; next} p{print}'
   1 awk '/^buc_require\\\(\\\)/,/^}/' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/buc_command.sh
   1 awk '/^buc_reject\\\(\\\)/,/^}/' Tools/buk/buc_command.sh
   1 awk '/^buc_die\\\(\\\)/{f=1} f{print} f&&/^}/{exit}' Tools/buk/buc_command.sh
   1 awk '/^buc_die\\\(\\\)/,/^}/' Tools/buk/buc_command.sh
   1 awk '/^# Probe per-installation depot states/,/^rbgp_payor_oauth_refresh\\\(\\\)/' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
   1 awk '/^:apcs_/ {print}' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/apck/APCS0-SpecTop.adoc
   1 awk '/#\\[cfg\\\(test\\\)\\]/{p=NR} END{print "cfg\(test\) near line "p}' Tools/rbk/rbtd/src/rbtdte_engine.rs
   1 awk '/\\[\\[apcs_[a-z_]+\\]\\]/' Tools/apck/APCS0-SpecTop.adoc
   1 awk '/"racing_order": \\[/{f=1} f{print} /\\]/{if\(f\)exit}'
   1 awk '{printf "%3d %s\\n", length\($0\), $0}'
   1 awk '{print}'
   1 awk '{print $NF}'
   1 awk '{print $9, $5}'
   1 awk '{print $5, $9}'
   1 awk '{print $1, substr\($0, index\($0, " "\)+1, 200\)}'
   1 awk '{prefix=substr\($0,1,5\); if\(prefix!="apcsc" && prefix!="apcsd" && prefix!="apcsg" && prefix!="apcsn" && prefix!="apcsu"\) print "UNEXPECTED: "$0}'
   1 awk '{for\(i=1;i<=length\($0\);i++\)print substr\($0,1,i\)}'
   1 awk '{ *
   1 awk -F'|' '{ split\($1,a,"_"\); print a[1]"_"a[2]"\\t"$2 }'
   1 awk -F'_' '{print substr\($1, 1, 4\), substr\($1, 4\)}'
   1 awk -F: '$1>=2940 && $1<=3010'
   1 awk -F: '$1>=1653 && $1<=2260'
   1 awk -F: '$1>=1060 && $1<=1210'
   1 awk -F: '{print $1":"$2}'
   1 asciidoctor -o /dev/null /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc
   1 asciidoctor --safe-mode=safe --failure-level=WARN -o /dev/null Tools/jjk/vov_veiled/JJS0_JobJockeySpec.adoc
   1 arp *
   1 ark
   1 2
   1 \\\\
   1 /usr/bin/gcloud --version
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/vvw-r.RunVVX.sh jjx_scout ark
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/vvw-r.RunVVX.sh --help
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/vow-t.Test.sh *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/vow-b.Build.sh *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tt.Test.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-ts.TestSuite.fast.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tq.QualifyFast.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tP.QualifyPristine.sh *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tl.Shellcheck.sh *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tf.FixtureRun.sh lode-lifecycle *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tc.FixtureCase.sh *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tb.Build.sh *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-lE.DirectorEnsconcesBole.sh rbev-bottle-ifrit-forge *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-ld.DirectorDivinesLodes.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-irh.DirectorRekonsHallmark.sh c260603134154-r260603134156 *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-iar.DirectorAuditsReliquaries.sh *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-bottle-ifrit-forge *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dl.PayorListsDepots.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbtd-r.Run.four-mode.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbtd-r.FixtureRun.srjcl.sh *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbtd-r.FixtureRun.pluml.sh *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbtd-b.Build.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/buw-st.BukSelfTest.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/vvk/bin/vvx vvx_unlock --help
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/vvk/bin/vvx --help
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/target/debug/theurge --help
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/target/debug/rbtd regime-smoke *
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/target/debug/rbtd --keep-going regime-smoke
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/target/debug/rbtd --help
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/vvw-r.RunVVX.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/vvw-r.RunVVX.sh jjx:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/vvw-r.RunVVX.sh jjx_record:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/vow-t.Test.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/vow-b.Build.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-z.Stop.tadmor.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-z.Stop.tadmor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-z.Stop.srjcl.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-z.Stop.nsproto.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-z.Stop.nsproto.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tw.TestSweep.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tw.TestSweep.sh fast:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tt.Test.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tT.QualifyTadmor.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSweep.fast.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.skirmish.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.sh ark-lifecycle:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.service.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.fast.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.fast.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.dogfight.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.crucible.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.complete.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tS.QualifySkirmish.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tq.QualifyFast.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-to.TestOne.sh rbtcsl_provenance_tcase:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tl.Shellcheck.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.three-mode.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.tadmor-security.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.srjcl-jupyter.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.regime-validation.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.regime-validation.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.regime-smoke.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.regime-credentials.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.qualify-all.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.pluml-diagram.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.nsproto-security.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.nsproto-security.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.kick-tires.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.four-mode.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.enrollment-validation.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.ark-lifecycle.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.access-probe.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.QualifyFast.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.QualifyFast.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh wsl-lifecycle *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh terrier-scaffold *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh regime-validation *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh regime-poison *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh podvm-lifecycle *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh lode-lifecycle *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh enrollment-validation *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh conformance *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tc.FixtureCase.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tb.Build.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-s.Start.tadmor.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-s.Start.tadmor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-s.Start.srjcl.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-s.Start.nsproto.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rw.RetrieverWrestsImage.sh "rbev-busybox:c260401072219-r260401142450-image" 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvy.DirectorYokesReliquaryAllVessels.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvl.ListVesselRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rv.RetrieverVouchesArk.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-sentry-ubuntu-large:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-sentry-debian-slim:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-bottle-ubuntu-test:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-bottle-plantuml:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-bottle-ifrit:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-bottle-anthropic-jupyter:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh nsproto:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsArk.sh rbev-vessels/rbev-busybox i20260313_142921-b20260313_213400 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsArk.sh rbev-vessels/rbev-busybox i20260313_122446-b20260313_210508 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsArk.sh rbev-vessels/rbev-busybox i20260312_173753-b20260313_180332 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsArk.sh rbev-sentry-ubuntu-large:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rrv.ValidateRepoRegime.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rrv.ValidateRepoRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rpv.ValidatePayorRegime.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rnv.ValidateNameplateRegime.sh tadmor *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rnv.ValidateNameplateRegime.sh ccyolo *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rnr.RenderNameplateRegime.sh tadmor:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rnl.ListNameplateRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-RiF.RetrieverInspectsFull.sh rbev-busybox:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Ric.RetrieverInspectsCompact.sh rbev-busybox:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rav.ValidateAuthRegime.sh retriever:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rar.RenderAuthRegime.sh retriever:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Qf.QualifyFast.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Qf.QualifyFast.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-qa.QualifyAll.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PO.PayorOnboarding.sh 2>&1 || true
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Pl.PayorListsDepots.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PG.PayorResetsGovernor.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PD.PayorDestroysDepot.sh rbwg-d-depot10030-260312153057:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PD.PayorDestroysDepot.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PD.PayorDestroysDepot.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PC.PayorCreatesDepot.sh depot10040:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Op.OnboardingPayor.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Og.OnboardingGovernor.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Ofc.OnboardingFirstCrucible.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Odf.OnboardingDirectorFirstBuild.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Odb.OnboardingDirectorBind.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Oda.OnboardingDirectorAirgap.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Ocr.OnboardingCredentialRetriever.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Ocd.OnboardingCredentialDirector.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Occ.OnboardingCrashCourse.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Occ.OnboardingConfigureEnvironment.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-o.OnboardingStartHere.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-o.ONBOARDING.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-nv.ValidateNameplates.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-MR.MarshalReset.sh 2>&1 || true
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-MG.MarshalGenerate.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-MG.MarshalGenerate.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-MD.MarshalDuplicate.sh /Users/bhyslop/test-marshal 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-lp.DirectorPresagesImmure.sh podvm-native *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-lI.DirectorImmuresPodvm.sh podvm-native *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-lC.DirectorConclavesReliquary.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-la.DirectorAugursLode.sh vn260610115913 *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-irh.DirectorRekonsHallmark.sh c260515152058-r260515152101 *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ir.DirectorRekonsImages.sh rbev-busybox:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-iar.DirectorAuditsReliquaries.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-iah.DirectorAuditsHallmarks.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-HWha.HostAvailability.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-HWdw.DockerWSLNative.sh rbtww-main:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-HWdd.DockerDesktop.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-HWdc.DockerContextDiscipline.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hw.HandbookWindows.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hs.RetrieverSummonsHallmark.sh rbev-sentry-debian-slim:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hs.RetrieverSummonsHallmark.sh rbev-bottle-ifrit:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hO.DirectorOrdainsHallmark.sh rbev-sentry-debian-slim:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hO.DirectorOrdainsHallmark.sh rbev-bottle-ifrit:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hO.DirectorOrdainsHallmark.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-h0.HandbookTOP.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-GS.GovernorDeletesServiceAccount.sh director-director1@rbwg-d-depot10040-260323094315.iam.gserviceaccount.com 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-GR.GovernorCreatesRetriever.sh retriever1:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gq.QuotaBuild.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gPo.PayorOnboarding.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gPE.PayorEstablish.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOR.OnboardRetriever.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOr.OnboardReference.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOP.OnboardPayor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOG.OnboardGovernor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOD.OnboardDirector.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-go.OnboardMAIN.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gO.Onboarding.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-GD.GovernorCreatesDirector.sh director1:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-GD.GovernorCreatesDirector.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ft.RetrieverTalliesHallmarks.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fs.RetrieverSummonsHallmark.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fpc.RetrieverPlumbsCompact.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-busybox:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fO.DirectorOrdainsHallmark.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fhv.HygieneCheckVessel.sh rbev-graft-demo *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fhv.HygieneCheckVessel.sh rbev-bottle-ccyolo *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fhc.HygieneCheck.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fA.DirectorAbjuresHallmark.sh rbev-busybox:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fA.DirectorAbjuresHallmark.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh r260610202716 *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh r260610145233 *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh r260515151530 *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DV.DirectorVouchesConsecrations.sh rbev-vessels/rbev-busybox 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DV.DirectorVouchesConsecrations.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Dt.DirectorTalliesConsecrations.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DPG.DirectorRefreshesGcbPins.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DP.DirectorRefreshesPins.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DO.DirectorOrdainsConsecration.sh rbev-vessels/rbev-sentry-debian-slim 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DO.DirectorOrdainsConsecration.sh rbev-vessels/rbev-busybox 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DO.DirectorOrdainsConsecration.sh rbev-vessels/rbev-bottle-plantuml 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DO.DirectorOrdainsConsecration.sh rbev-vessels/rbev-bottle-ifrit 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DO.DirectorOrdainsConsecration.sh rbev-vessels/rbev-bottle-anthropic-jupyter 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dl.PayorListsDepots.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dl.PayorListsDepots.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DI.DirectorInscribesRubric.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-di.DepotInfo.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-sentry-ubuntu-large 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-busybox 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-bottle-ubuntu-test 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-bottle-ifrit
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-sentry-debian-slim:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh rbev-vessels/rbev-sentry-ubuntu-large 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh rbev-vessels/rbev-busybox 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh rbev-vessels/rbev-bottle-ubuntu-test 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh rbev-sentry-ubuntu-large:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh rbev-sentry-debian-slim:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh rbev-bottle-ifrit:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-busybox
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-bottle-plantuml 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesArk.sh rbev-busybox:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesArk.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesArk.rbev-busybox.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorConjuresArk.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Dc.DirectorChecksConsecrations.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Dc.CheckConsecrations.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DB.DirectorBeseechesArk.sh rbev-sentry-ubuntu-large:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-da.DepotAttribution.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cQ.Quench.tadmor.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cQ.Quench.ccyolo.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cKS.KludgeSentry.sh tadmor *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cKB.KludgeBottle.sh tadmor:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cKB.KludgeBottle.sh ccyolo:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cKB.KludgeBottle.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cic.CrucibleIsCharged.sh tadmor:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cC.Charge.tadmor.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cC.Charge.tadmor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cC.Charge.ccyolo.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-arD.GovernorDivestsRetriever.sh bhl *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-aM.PayorMantlesGovernor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-adI.GovernorInvestsDirector.sh bhl *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-adD.GovernorDivestsDirector.sh bhl *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-t.Test.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-t.Test.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.TestSuite.service.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.TestSuite.fast.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.TestSuite.crucible.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.TestSuite.crucible.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_sortie_http_end_to_end:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.SingleCase.pristine-lifecycle.sh rbtdrp_marshal_zero_attestation
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.SingleCase.pristine-lifecycle.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.FixtureCase.sh regime-validation *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.FixtureCase.canonical-onboarding-sequence.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.tadmor.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.tadmor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.tadmor-security.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.regime-validation.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.pristine-lifecycle.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.four-mode.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.access-probe.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.regime-validation.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.regime-smoke.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.handbook-render.sh *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.enrollment-validation.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.dockerfile-hygiene.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-b.Build.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-b.Build.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbi-iK.IfritKludge.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbi-iB.IfritBuild.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-st.BukSelfTest.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-st.BukSelfTest.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rsv.ValidateStationRegime.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rpl.PrivilegeRegimeList.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rpl.ListPrivilegeRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnva.ValidateAllNodeRegimes.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnv.ValidateNodeRegime.sh winhost-wsl:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnv.ValidateNodeRegime.sh winhost-ps:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnv.ValidateNodeRegime.sh winhost-cyg:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnv.ValidateNodeRegime.sh devbox-linux:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnv.ValidateNodeRegime.sh bujn-winpc *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnr.RenderNodeRegime.sh winhost-cyg:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnl.NodeRegimeList.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnl.ListNodeRegime.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcx.ConstructLocalhost.sh bhyslop:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcw.ConstructWSL.sh 192.168.86.27 bhyslop winhost
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcp.ConstructPowerShell.sh 192.168.86.27 bhyslop winhost
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcl.ConstructLinux.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcc.ConstructCygwin.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rev.ValidateEnvironmentRegime.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rer.RenderEnvironmentRegime.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-qsc.QualifyShellCheck.sh 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-qsc.QualifyShellCheck.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jws.WorkloadInteractiveSession.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jwk.WorkloadKnock.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpS.PrivilegedSsh.sh bujn-winpc *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpS.PrivilegedSsh.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpS bujn-winpc *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpGw.GarrisonWsl.sh bujn-winpc *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpGb.GarrisonBash.sh nonesuch-investiture *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpGb.GarrisonBash.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpF.Fenestrate.sh nonesuch-investiture *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpCW.CaparisonWindows.sh bujn-winpc *
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWew.EnvironmentWSL.sh rbtww-main:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWec.EnvironmentCygwin.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWax.AccessEntrypoints.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWar.AccessRemote.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWab.AccessBase.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-hw.HandbookWindows.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-hjm.HandbookJurisdictionMacos.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-hjl.HandbookJurisdictionLinux.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-hj0.HandbookJurisdictionTop.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/apcw-t.Test.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/apcw-r.Run.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/apcw-ba.BatchAssay.sh:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvk/bin/vvx-darwin-arm64 --help 2>&1 | head -40
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvk/bin/vvx --help 2>&1 | head -30
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvk/bin/vvx --help 2>&1 | head -20
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvk/bin/vvx --help
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvk/bin/vvr-darwin-arm64 --help 2>&1 | head -30
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid/target/debug/rbid dns-allowed-anthropic:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid/target/debug/rbid bogus-attack:*
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid/target/debug/rbid --list 2>&1
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid/target/debug/rbid
   1 /Users/bhyslop/projects/pb_paneboard02/tt/pbw-t.ProofOfConceptTimed.10.sh
   1 /Users/bhyslop/models/stanford-deidentifier/.venv/bin/python -m pip install optimum-onnx
   1 /Users/bhyslop/models/stanford-deidentifier/.venv/bin/python -m pip install onnxruntime
   1 /Users/bhyslop/models/stanford-deidentifier/.venv/bin/python -c "import json; d=json.load\(open\('/Users/bhyslop/models/stanford-deidentifier/config.json'\)\); print\(json.dumps\(d.get\('id2label'\), indent=2\)\)"
   1 /Users/bhyslop/models/stanford-deidentifier/.venv/bin/optimum-cli export *
   1 /Users/bhyslop/models/stanford-deidentifier/.venv/bin/optimum-cli --help
   1 /tmp/vvk1013_install/vvi_install.sh /Users/bhyslop/projects/pb_paneboard02/.buk/burc.env
   1 /tmp/sibling_analysis.sh
   1 /tmp/sc2153_backstop_test.sh
   1 /tmp/rbspike/mint_payor.sh
   1 /tmp/rbrm_verification.txt:*
   1 /tmp/p1015full/vvi_install.sh /Users/bhyslop/projects/djo-DanielsJupyterObsidian/.buk/burc.env
   1 /tmp/p1014/vvi_install.sh /Users/bhyslop/projects/pb_paneboard02/.buk/burc.env
   1 /tmp/count_tests.sh
   1 /tmp/analyze_rbs0.sh
   1 /tmp/analyze_hardcoding.sh
   1 /System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -dump | grep -i slick 2>&1
   1 /System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -dump | grep -B2 "/Applications/SlickEditPro2025" 2>&1
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/vow-b.Build.sh *
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tt.Test.sh
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.skirmish.sh *
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.fast.sh *
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.dogfight.sh *
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.crucible.sh
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tl.Shellcheck.sh *
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh lode-lifecycle *
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh canonical-invest *
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tb.Build.sh
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-adI.GovernorInvestsDirector.sh canest-dir *
   1 /context | head -100
   1 /bin/bash -c *
   1 /bin/bash --version
   1 [[ -f "Tools/buk/vov_veiled/$f" ]]
   1 [ -f "/home/bhyslop/projects/rbm_alpha_recipemuster/$p" ]
   1 [ -d "$PB" ]
   1 " /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/vov_veiled/BCG-BashConsoleGuide.md | head -40
   1 ./tt/vvw-r.RunVVX.sh version:*
   1 ./tt/vvw-r.RunVVX.sh schema:*
   1 ./tt/vvw-r.RunVVX.sh jjx:*
   1 ./tt/vvw-r.RunVVX.sh eval:*
   1 ./tt/vvw-r.RunVVX.sh --version 2>&1
   1 ./tt/vvw-r.RunVVX.sh --help
   1 ./tt/vow-t.Test.sh:*
   1 ./tt/vow-R.ParcelRelease.sh
   1 ./tt/vow-F.Freshen.sh
   1 ./tt/vow-b.Build.sh:*
   1 ./tt/rbw-z.Stop.tadmor.sh 2>&1
   1 ./tt/rbw-tw.TestSweep.sh service:*
   1 ./tt/rbw-tw.TestSweep.sh fast:*
   1 ./tt/rbw-tw.TestSweep.sh complete:*
   1 ./tt/rbw-tw.TestSweep.sh 2>&1
   1 ./tt/rbw-tt.Test.sh *
   1 ./tt/rbw-ts.TestSuite.tadmor.sh *
   1 ./tt/rbw-ts.TestSuite.skirmish.sh *
   1 ./tt/rbw-ts.TestSuite.siege.sh *
   1 ./tt/rbw-ts.TestSuite.sh srjcl-jupyter:*
   1 ./tt/rbw-ts.TestSuite.sh pluml-diagram:*
   1 ./tt/rbw-ts.TestSuite.sh nsproto-security:*
   1 ./tt/rbw-ts.TestSuite.sh ark-lifecycle:*
   1 ./tt/rbw-ts.TestSuite.service.sh 2>&1
   1 ./tt/rbw-ts.TestSuite.service.sh *
   1 ./tt/rbw-ts.TestSuite.service.sh
   1 ./tt/rbw-ts.TestSuite.fast.sh 2>&1 | tail -5
   1 ./tt/rbw-ts.TestSuite.fast.sh 2>&1 | tail -40
   1 ./tt/rbw-ts.TestSuite.fast.sh 2>&1 | tail -25
   1 ./tt/rbw-ts.TestSuite.fast.sh 2>&1 | tail -20
   1 ./tt/rbw-ts.TestSuite.fast.sh 2>&1 | tail -15
   1 ./tt/rbw-ts.TestSuite.fast.sh 2>&1 | tail -10
   1 ./tt/rbw-ts.TestSuite.fast.sh 2>&1
   1 ./tt/rbw-ts.TestSuite.fast.sh *
   1 ./tt/rbw-ts.TestSuite.fast.sh
   1 ./tt/rbw-ts.TestSuite.crucible.sh *
   1 ./tt/rbw-ts.TestSuite.complete.sh *
   1 ./tt/rbw-ts.TestSuite.complete.sh
   1 ./tt/rbw-ts.TestSuite.blockade.sh *
   1 ./tt/rbw-tS.QualifySkirmish.sh *
   1 ./tt/rbw-to.TestOne.sh 2>&1 | tail -15
   1 ./tt/rbw-tO.OrdainCycle.tadmor.sh
   1 ./tt/rbw-tK.KludgeCycle.tadmor.sh 2>&1
   1 ./tt/rbw-tf.TestFixture.three-mode.sh 2>&1
   1 ./tt/rbw-tf.TestFixture.tadmor-security.sh 2>&1
   1 ./tt/rbw-tf.TestFixture.srjcl-jupyter.sh 2>&1
   1 ./tt/rbw-tf.TestFixture.slsa-provenance.sh
   1 ./tt/rbw-tf.TestFixture.regime-validation.sh
   1 ./tt/rbw-tf.TestFixture.regime-smoke.sh
   1 ./tt/rbw-tf.TestFixture.qualify-all.sh
   1 ./tt/rbw-tf.TestFixture.pluml-diagram.sh 2>&1
   1 ./tt/rbw-tf.TestFixture.kick-tires.sh 2>&1 | tail -5
   1 ./tt/rbw-tf.TestFixture.kick-tires.sh
   1 ./tt/rbw-tf.TestFixture.enrollment-validation.sh
   1 ./tt/rbw-tf.TestFixture.ark-lifecycle.sh
   1 ./tt/rbw-tf.QualifyFast.sh 2>&1
   1 ./tt/rbw-tf.QualifyFast.sh
   1 ./tt/rbw-tf.FixtureRun.sh terrier-atomicity *
   1 ./tt/rbw-tf.FixtureRun.sh tadmor *
   1 ./tt/rbw-tf.FixtureRun.sh regime-validation *
   1 ./tt/rbw-tf.FixtureRun.sh regime-poison *
   1 ./tt/rbw-tf.FixtureRun.sh recipe-validation *
   1 ./tt/rbw-tf.FixtureRun.sh pristine-lifecycle *
   1 ./tt/rbw-tf.FixtureRun.sh pluml *
   1 ./tt/rbw-tf.FixtureRun.sh onboarding-sequence *
   1 ./tt/rbw-tf.FixtureRun.sh dogfight *
   1 ./tt/rbw-tf.FixtureRun.sh admission-proof *
   1 ./tt/rbw-tc.FixtureCase.sh tadmor *
   1 ./tt/rbw-tc.FixtureCase.sh handbook-render *
   1 ./tt/rbw-tc.FixtureCase.sh enrollment-validation *
   1 ./tt/rbw-tc.FixtureCase.sh *
   1 ./tt/rbw-tb.Build.sh
   1 ./tt/rbw-s.Start.tadmor.sh 2>&1
   1 ./tt/rbw-rvv.ValidateVesselRegime.sh rbev-bottle-ifrit:*
   1 ./tt/rbw-rvv.ValidateVesselRegime.sh rbev-bottle-ccyolo:*
   1 ./tt/rbw-rvv.ValidateVesselRegime.sh
   1 ./tt/rbw-rvr.RenderVesselRegime.sh rbev-sentry-debian-slim:*
   1 ./tt/rbw-rvr.RenderVesselRegime.sh rbev-bottle-ifrit:*
   1 ./tt/rbw-rvr.RenderVesselRegime.sh rbev-bottle-ccyolo:*
   1 ./tt/rbw-rvr.RenderVesselRegime.sh
   1 ./tt/rbw-rvl.ListVesselRegime.sh:*
   1 ./tt/rbw-rv.RegimeValidate.sh
   1 ./tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-sentry-debian-slim:*
   1 ./tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-bottle-ifrit:*
   1 ./tt/rbw-rrv.ValidateRepoRegime.sh
   1 ./tt/rbw-rrr.RenderRepoRegime.sh:*
   1 ./tt/rbw-rpv.ValidatePayorRegime.sh *
   1 ./tt/rbw-rpr.RenderPayorRegime.sh
   1 ./tt/rbw-rov.ValidateOauthRegime.sh *
   1 ./tt/rbw-rnv.ValidateNameplateRegime.sh tadmor *
   1 ./tt/rbw-rnv.ValidateNameplateRegime.sh srjcl *
   1 ./tt/rbw-rnv.ValidateNameplateRegime.sh pluml:*
   1 ./tt/rbw-rnv.ValidateNameplateRegime.sh moriah *
   1 ./tt/rbw-rnv.ValidateNameplateRegime.sh ccyolo:*
   1 ./tt/rbw-rnr.RenderNameplateRegime.tadmor.sh
   1 ./tt/rbw-rnr.RenderNameplateRegime.sh tadmor:*
   1 ./tt/rbw-rnr.RenderNameplateRegime.sh
   1 ./tt/rbw-rnl.ListNameplateRegime.sh
   1 ./tt/rbw-rgr.RenderPinsRegime.sh:*
   1 ./tt/rbw-rfv.ValidateFederationRegime.sh
   1 ./tt/rbw-rdv.ValidateDepotRegime.sh
   1 ./tt/rbw-ral.ListAuthRegimes.sh
   1 ./tt/rbw-Qf.QualifyFast.sh 2>&1 | tail -30
   1 ./tt/rbw-Qf.QualifyFast.sh 2>&1
   1 ./tt/rbw-qa.QualifyAll.sh 2>&1 | tail -40
   1 ./tt/rbw-Ots.OnboardingTadmorSecurity.sh
   1 ./tt/rbw-Op.OnboardingPayor.sh
   1 ./tt/rbw-Og.OnboardingGovernor.sh:*
   1 ./tt/rbw-Ofc.OnboardingFirstCrucible.sh:*
   1 ./tt/rbw-Odf.OnboardingDirectorFirstBuild.sh --help
   1 ./tt/rbw-Odf.OnboardingDirectorFirstBuild.sh
   1 ./tt/rbw-Ocr.OnboardingCredentialRetriever.sh:*
   1 ./tt/rbw-Ocd.OnboardingCredentialDirector.sh:*
   1 ./tt/rbw-Occ.OnboardingCrashCourse.sh:*
   1 ./tt/rbw-Occ.OnboardingConfigureEnvironment.sh
   1 ./tt/rbw-o.OnboardingStartHere.sh --help
   1 ./tt/rbw-o.OnboardingStartHere.sh
   1 ./tt/rbw-o.ONBOARDING.sh --help
   1 ./tt/rbw-o.ONBOARDING.sh
   1 ./tt/rbw-nv.ValidateNameplates.sh
   1 ./tt/rbw-ni.NameplateInfo.sh
   1 ./tt/rbw-MG.MarshalGenerate.sh 2>&1
   1 ./tt/rbw-MG.MarshalGenerate.sh *
   1 ./tt/rbw-MG.MarshalGenerate.sh
   1 ./tt/rbw-LK.LocalKludge.sh
   1 ./tt/rbw-ld.DirectorDivinesLodes.sh r260609104734 *
   1 ./tt/rbw-ld.DirectorDivinesLodes.sh *
   1 ./tt/rbw-ld.DirectorDivinesLodes.sh
   1 ./tt/rbw-Is.IfritSortie.tadmor.sh 2>&1
   1 ./tt/rbw-irr.DirectorRekonsReliquary.sh r260609093011 *
   1 ./tt/rbw-irr.DirectorRekonsReliquary.sh r260605074843 *
   1 ./tt/rbw-irr.DirectorRekonsReliquary.sh r260513125123 *
   1 ./tt/rbw-ir.DirectorRekonsImages.sh rbev-busybox:*
   1 ./tt/rbw-ir.DirectorRekonsImages.sh rbev-bottle-ifrit:*
   1 ./tt/rbw-iJr.DirectorJettisonsReliquaryImage.sh
   1 ./tt/rbw-iar.DirectorAuditsReliquaries.sh *
   1 ./tt/rbw-iar.DirectorAuditsReliquaries.sh
   1 ./tt/rbw-iah.DirectorAuditsHallmarks.sh *
   1 ./tt/rbw-iah.DirectorAuditsHallmarks.sh
   1 ./tt/rbw-iae.DirectorAuditsEnshrinements.sh *
   1 ./tt/rbw-HWdw.DockerWSLNative.sh rbtww-main:*
   1 ./tt/rbw-HWdd.DockerDesktop.sh
   1 ./tt/rbw-HWdc.DockerContextDiscipline.sh
   1 ./tt/rbw-hw.HandbookWindows.sh
   1 ./tt/rbw-hs.RetrieverSummonsHallmark.sh rbev-bottle-ifrit:*
   1 ./tt/rbw-hO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-bottle-ifrit
   1 ./tt/rbw-hO.DirectorOrdainsHallmark.sh
   1 ./tt/rbw-h0.HandbookTOP.sh:*
   1 ./tt/rbw-gPR.PayorRefresh.sh *
   1 ./tt/rbw-gPE.PayorEstablish.sh
   1 ./tt/rbw-gOR.OnboardRetriever.sh 2>&1
   1 ./tt/rbw-gOr.OnboardReference.sh 2>&1
   1 ./tt/rbw-gOP.OnboardPayor.sh
   1 ./tt/rbw-gOG.OnboardGovernor.sh
   1 ./tt/rbw-gOD.OnboardDirector.sh
   1 ./tt/rbw-go.OnboardMAIN.sh 2>&1
   1 ./tt/rbw-go.OnboardMAIN.sh
   1 ./tt/rbw-gO.Onboarding.sh tadmor:*
   1 ./tt/rbw-gO.Onboarding.sh
   1 ./tt/rbw-ft.RetrieverTalliesHallmarks.sh:*
   1 ./tt/rbw-ft.RetrieverTalliesHallmarks.sh
   1 ./tt/rbw-fs.RetrieverSummonsHallmark.sh:*
   1 ./tt/rbw-fpc.RetrieverPlumbsCompact.sh
   1 ./tt/rbw-fO.DirectorOrdainsHallmark.sh:*
   1 ./tt/rbw-fk.LocalKludge.sh rbev-sentry-debian-slim:*
   1 ./tt/rbw-fk.LocalKludge.sh rbev-sentry-deb-tether *
   1 ./tt/rbw-fA.DirectorAbjuresHallmark.sh rbev-busybox:*
   1 ./tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh r260609093011 *
   1 ./tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh r260513125123 *
   1 ./tt/rbw-DV.DirectorVouchesArk.sh:*
   1 ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhm100002 *
   1 ./tt/rbw-DS.DirectorSummonsArk.sh rbev-sentry-ubuntu-large:*
   1 ./tt/rbw-DS.DirectorSummonsArk.sh rbev-busybox:*
   1 ./tt/rbw-DS.DirectorSummonsArk.sh rbev-bottle-ubuntu-test:*
   1 ./tt/rbw-DS.DirectorSummonsArk.sh rbev-bottle-plantuml:*
   1 ./tt/rbw-DS.DirectorSummonsArk.sh rbev-bottle-anthropic-jupyter:*
   1 ./tt/rbw-DPG.DirectorRefreshesGcbPins.sh 2>&1
   1 ./tt/rbw-DP.DirectorRefreshesPins.sh:*
   1 ./tt/rbw-dL.PayorLeviesDepot.sh *
   1 ./tt/rbw-dI.DirectorInscribesReliquary.sh *
   1 ./tt/rbw-dE.DirectorEnshrinesVessel.sh rbev-bottle-ifrit-forge *
   1 ./tt/rbw-DC.DirectorCreatesConsecration.sh rbev-sentry-debian-slim:*
   1 ./tt/rbw-DC.DirectorCreatesConsecration.sh rbev-bottle-ifrit:*
   1 ./tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-busybox-graft 2>&1
   1 ./tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-busybox 2>&1
   1 ./tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-bottle-plantuml 2>&1
   1 ./tt/rbw-DC.DirectorCreatesArk.sh 2>&1 | head -30
   1 ./tt/rbw-DC.DirectorConjuresArk.sh rbev-vessels/rbev-bottle-plantuml 2>&1
   1 ./tt/rbw-DC.DirectorConjuresArk.sh rbev-vessels/rbev-bottle-anthropic-jupyter 2>&1
   1 ./tt/rbw-Dc.DirectorChecksConsecrations.sh:*
   1 ./tt/rbw-DA.DirectorAbjuresArk.sh:*
   1 ./tt/rbw-cw.Writ.tadmor.sh sh:*
   1 ./tt/rbw-cw.Writ.tadmor.sh sh *
   1 ./tt/rbw-cw.Writ.tadmor.sh ps:*
   1 ./tt/rbw-cw.Writ.tadmor.sh ls:*
   1 ./tt/rbw-cw.Writ.tadmor.sh iptables *
   1 ./tt/rbw-cw.Writ.tadmor.sh cat:*
   1 ./tt/rbw-cQ.Quench.tadmor.sh 2>&1
   1 ./tt/rbw-cQ.Quench.tadmor.sh *
   1 ./tt/rbw-cQ.Quench.tadmor.sh
   1 ./tt/rbw-cQ.Quench.srjcl.sh
   1 ./tt/rbw-cQ.Quench.ccyolo.sh
   1 ./tt/rbw-cKS.KludgeSentry.tadmor.sh *
   1 ./tt/rbw-cKS.KludgeSentry.sh srjcl *
   1 ./tt/rbw-cKS.KludgeSentry.sh pluml *
   1 ./tt/rbw-cKB.KludgeBottle.tadmor.sh 2>&1
   1 ./tt/rbw-cKB.KludgeBottle.sh tadmor:*
   1 ./tt/rbw-cKB.KludgeBottle.sh ccyolo:*
   1 ./tt/rbw-cK.Kludge.tadmor.sh 2>&1
   1 ./tt/rbw-cic.CrucibleIsCharged.tadmor.sh
   1 ./tt/rbw-cic.CrucibleIsCharged.sh tadmor:*
   1 ./tt/rbw-cic.CrucibleIsCharged.sh tadmor *
   1 ./tt/rbw-cic.CrucibleIsCharged.sh srjcl *
   1 ./tt/rbw-cic.CrucibleIsCharged.sh
   1 ./tt/rbw-ch.Hail.sh
   1 ./tt/rbw-cC.Charge.tadmor.sh 2>&1
   1 ./tt/rbw-cC.Charge.tadmor.sh *
   1 ./tt/rbw-cC.Charge.tadmor.sh
   1 ./tt/rbw-cC.Charge.srjcl.sh *
   1 ./tt/rbw-cC.Charge.ccyolo.sh
   1 ./tt/rbw-cb.Bark.tadmor.sh sh *
   1 ./tt/rbw-cb.Bark.tadmor.sh rbid:*
   1 ./tt/rbw-cb.Bark.tadmor.sh rbid *
   1 ./tt/rbw-cb.Bark.tadmor.sh getent *
   1 ./tt/rbw-ca.CrucibleActive.sh tadmor:*
   1 ./tt/rbw-arr.GovernorRostersRetrievers.sh *
   1 ./tt/rbw-arI.GovernorInvestsRetriever.sh t3r *
   1 ./tt/rbw-ak.ArkKludge.sh rbev-sentry-debian-slim:*
   1 ./tt/rbw-ak.ArkKludge.sh rbev-busybox:*
   1 ./tt/rbw-ak.ArkKludge.sh
   1 ./tt/rbw-adr.GovernorRostersDirectors.sh *
   1 ./tt/rbw-adr.GovernorRostersDirectors.sh
   1 ./tt/rbw-adI.GovernorInvestsDirector.sh t3d *
   1 ./tt/rbw-adI.GovernorInvestsDirector.sh deltest *
   1 ./tt/rbw-adI.GovernorInvestsDirector.sh canest-dir *
   1 ./tt/rbw-adD.GovernorDivestsDirector.sh t3d *
   1 ./tt/rbw-acr.CheckRetrieverCredential.sh
   1 ./tt/rbw-acp.CheckPayorCredential.sh
   1 ./tt/rbw-acg.CheckGovernorCredential.sh
   1 ./tt/rbw-acf.CheckFederatedAccess.sh
   1 ./tt/rbw-acd.CheckDirectorCredential.sh
   1 ./tt/rbtd-t.Test.sh 2>&1
   1 ./tt/rbtd-t.Test.sh
   1 ./tt/rbtd-s.TestSuite.service.sh
   1 ./tt/rbtd-s.TestSuite.fast.sh
   1 ./tt/rbtd-s.TestSuite.crucible.sh 2>&1
   1 ./tt/rbtd-s.TestSuite.crucible.sh
   1 ./tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_sortie_sentry_udp_non_dns
   1 ./tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_coordinated_sentry_integrity 2>&1
   1 ./tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_coordinated_sentry_egress_lockdown
   1 ./tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_coordinated_mac_flood_resilience
   1 ./tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_coordinated_dnsmasq_query_audit
   1 ./tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_coordinated_dns_cache_integrity
   1 ./tt/rbtd-s.SingleCase.tadmor.sh pentacle-dnsmasq-responds:*
   1 ./tt/rbtd-s.SingleCase.tadmor.sh coordinated-arp-gratuitous:*
   1 ./tt/rbtd-s.SingleCase.tadmor.sh 2>&1
   1 ./tt/rbtd-s.SingleCase.regime-smoke.sh rs-burc:*
   1 ./tt/rbtd-s.SingleCase.four-mode.sh
   1 ./tt/rbtd-s.SingleCase.enrollment-validation.sh 2>&1
   1 ./tt/rbtd-s.FixtureCase.sh tadmor *
   1 ./tt/rbtd-s.FixtureCase.sh
   1 ./tt/rbtd-r.Run.tadmor.sh
   1 ./tt/rbtd-r.Run.srjcl.sh 2>&1
   1 ./tt/rbtd-r.Run.regime-validation.sh 2>&1
   1 ./tt/rbtd-r.Run.regime-validation.sh
   1 ./tt/rbtd-r.Run.regime-smoke.sh 2>&1
   1 ./tt/rbtd-r.Run.regime-smoke.sh
   1 ./tt/rbtd-r.Run.pluml.sh 2>&1
   1 ./tt/rbtd-r.Run.four-mode.sh 2>&1
   1 ./tt/rbtd-r.Run.four-mode.sh
   1 ./tt/rbtd-r.Run.enrollment-validation.sh 2>&1
   1 ./tt/rbtd-r.Run.access-probe.sh 2>&1
   1 ./tt/rbtd-r.FixtureRun.tadmor.sh *
   1 ./tt/rbtd-r.FixtureRun.tadmor-security.sh *
   1 ./tt/rbtd-r.FixtureRun.srjcl.sh *
   1 ./tt/rbtd-r.FixtureRun.pluml.sh *
   1 ./tt/rbtd-r.FixtureRun.moriah.sh *
   1 ./tt/rbtd-r.FixtureRun.handbook-render.sh *
   1 ./tt/rbtd-r.FixtureRun.canonical-establish.sh
   1 ./tt/rbtd-r.FixtureRun.access-probe.sh *
   1 ./tt/rbtd-b.Build.sh 2>&1
   1 ./tt/rbi-iB.IfritBuild.sh 2>&1
   1 ./tt/jjw-tfS.TestFundusSingle.localhost.sh relay_concurrent_overlap 2>&1
   1 ./tt/jjw-tfS.TestFundusSingle.localhost.sh relay_check_instant 2>&1
   1 ./tt/jjw-tfs.TestFundusScenario.localhost.sh 2>&1
   1 ./tt/jjw-tfs.TestFundusScenario.cerebro.sh 2>&1
   1 ./tt/jjw-tfP2.ProvisionPhase2.localhost.sh 2>&1
   1 ./tt/buw-xd.Delay.sh
   1 ./tt/buw-tt-ll.ListLaunchers.sh *
   1 ./tt/buw-ts.TestSweep.sh 2>&1 | tail -40
   1 ./tt/buw-ta.TestAll.sh
   1 ./tt/buw-st.BukSelfTest.sh 2>&1
   1 ./tt/buw-st.BukSelfTest.sh *
   1 ./tt/buw-st.BukSelfTest.sh
   1 ./tt/buw-SI.StationInit.sh 2>&1
   1 ./tt/buw-rsr.RenderStationRegime.sh *
   1 ./tt/buw-rpv.ValidatePrivilegeRegime.sh
   1 ./tt/buw-rpr.RenderPrivilegeRegime.sh *
   1 ./tt/buw-rnv.ValidateNodeRegime.sh jjfu-full:*
   1 ./tt/buw-rnv.ValidateNodeRegime.sh bujn-winpc *
   1 ./tt/buw-rnr.RenderNodeRegime.sh *
   1 ./tt/buw-rer.RenderEnvironmentRegime.sh *
   1 ./tt/buw-rcr.RenderConfigRegime.sh *
   1 ./tt/buw-qsc.QualifyShellCheck.sh 2>&1
   1 ./tt/buw-qsc.QualifyShellCheck.sh *
   1 ./tt/buw-jwk.WorkloadKnock.sh bujn-winpc *
   1 ./tt/buw-jpW.WslInstall.sh *
   1 ./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc *
   1 ./tt/buw-jpIW.InvigilateWindows.sh --help
   1 ./tt/buw-jpGw.GarrisonWsl.sh bujn-winpc *
   1 ./tt/buw-jpF.Fenestrate.sh *
   1 ./tt/buw-jpCW.CaparisonWindows.sh bujn-winpc *
   1 ./tt/buw-HWew.EnvironmentWSL.sh rbtww-main:*
   1 ./tt/buw-HWec.EnvironmentCygwin.sh:*
   1 ./tt/buw-HWax.AccessEntrypoints.sh
   1 ./tt/buw-HWar.AccessRemote.sh
   1 ./tt/buw-HWab.AccessBase.sh:*
   1 ./tt/buw-hw.HandbookWindows.sh:*
   1 ./tt/buw-hj0.HandbookJurisdictionTop.sh *
   1 ./tt/buw-h0.HandbookTOP.sh:*
   1 ./tt/buw-dly.Delay.sh
   1 ./tt/buw-d.Delay.sh
   1 ./tt/apcw-t.Test.sh:*
   1 ./Tools/vvk/bin/vvx --version
```

### Command-head frequency (combined)
```
 278 echo
 234 sed
  76 shellcheck
  68 BURE_CONFIRM=skip
  65 awk
  59 find
  29 docker
  29 BURE_COUNTDOWN=skip
  28 curl
  24 bash
  22 mv
  22 ./tt/rbw-tf.FixtureRun.sh
  20 grep
  19 gcloud
  15 xargs
  15 sort
  15 ping
  15 command
  14 nc
  13 mkdir
  13 ls
  12 chmod
  11 rg
  10 wc
  10 tee
  10 sudo
  10 printf
   9 ssh
   9 rm
   9 perl
   9 cargo
   9 ./tt/rbw-ts.TestSuite.fast.sh
   9 ./tt/rbtd-s.SingleCase.tadmor.sh
   8 pkill
   8 osascript
   8 git
   8 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh
   7 tt/rbw-dY.DirectorYokesReliquaryInVessel.sh
   7 scp
   7 for
   7 cat
   7 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh
   7 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh
   6 xxd
   6 cd
   6 ./tt/vvw-r.RunVVX.sh
   6 ./tt/rbw-cw.Writ.tadmor.sh
   5 unzip
   5 tt/rbw-tf.FixtureRun.sh
   5 tar
   5 shasum
   5 jq
   5 GIT_EDITOR=true
   5 do
   5 dig
   5 cp
   5 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DO.DirectorOrdainsConsecration.sh
   5 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnv.ValidateNodeRegime.sh
   5 ./tt/rbw-rnv.ValidateNameplateRegime.sh
   5 ./tt/rbw-DS.DirectorSummonsArk.sh
   4 tt/vvw-r.RunVVX.sh
   4 tt/vow-t.Test.sh
   4 tt/rbw-DS.DirectorSummonsArk.sh
   4 tt/rbtd-s.SingleCase.tadmor.sh
   4 tt/rbtd-s.SingleCase.pristine-lifecycle.sh
   4 tt/rbtd-s.FixtureCase.sh
   4 tt/buw-rsv.ValidateStationRegime.sh
   4 tt/buw-rnv.ValidateNodeRegime.sh
   4 tt/buw-rnl.ListNodeRegime.sh
   4 Tools/rbk/rbtd/target/debug/rbtd
   4 tail
   4 rustfmt
   4 python3
   4 openssl
   4 host
   4 gh
   4 asciidoctor
   4 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsArk.sh
   4 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DE.DirectorEnshrinesVessel.sh
   4 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesArk.sh
   4 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid/target/debug/rbid
   4 ./tt/rbw-tw.TestSweep.sh
   4 ./tt/rbw-ts.TestSuite.sh
   4 ./tt/rbw-tc.FixtureCase.sh
   4 ./tt/rbw-rvr.RenderVesselRegime.sh
   4 ./tt/rbw-DC.DirectorCreatesArk.sh
   4 ./tt/rbw-cKS.KludgeSentry.sh
   4 ./tt/rbw-cic.CrucibleIsCharged.sh
   4 ./tt/rbw-cb.Bark.tadmor.sh
   3 tt/vow-b.Build.sh
   3 tt/rbw-fO.DirectorOrdainsHallmark.sh
   3 tt/rbw-DV.DirectorVouchesConsecrations.sh
   3 tt/rbw-DC.DirectorCreatesArk.sh
   3 tt/rbtd-s.SingleCase.four-mode.sh
   3 tt/rbtd-b.Build.sh
   3 tt/buw-rnr.RenderNodeRegime.sh
   3 tt/buw-rcv.ValidateConfigRegime.sh
   3 tt/buw-jpS.PrivilegedSsh.sh
   3 traceroute
   3 timeout
   3 source
   3 script
   3 rmdir
   3 read
   3 podman
   3 kill
   3 dns-sd
   3 brew
   3 /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/target/debug/rbtd
   3 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/vvw-r.RunVVX.sh
   3 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PD.PayorDestroysDepot.sh
   3 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hO.DirectorOrdainsHallmark.sh
   3 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh
   3 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cKB.KludgeBottle.sh
   3 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvk/bin/vvx
   3 /Users/bhyslop/models/stanford-deidentifier/.venv/bin/python
   3 ./tt/vow-t.Test.sh
   3 ./tt/vow-b.Build.sh
   3 ./tt/rbw-tt.Test.sh
   3 ./tt/rbw-ts.TestSuite.service.sh
   3 ./tt/rbw-ts.TestSuite.dogfight.sh
   3 ./tt/rbw-tl.Shellcheck.sh
   3 ./tt/rbw-tb.Build.sh
   3 ./tt/rbw-rvv.ValidateVesselRegime.sh
   3 ./tt/rbw-MG.MarshalGenerate.sh
   3 ./tt/rbw-ld.DirectorDivinesLodes.sh
   3 ./tt/rbw-irr.DirectorRekonsReliquary.sh
   3 ./tt/rbw-cQ.Quench.tadmor.sh
   3 ./tt/rbw-cC.Charge.tadmor.sh
   3 ./tt/rbw-ak.ArkKludge.sh
   3 ./tt/rbw-adI.GovernorInvestsDirector.sh
   3 ./tt/rbtd-b.Build.sh
   3 ./tt/buw-st.BukSelfTest.sh
   3 ./tt/buw-rpv.ValidatePrivilegeRegime.sh
   2 which
   2 tt/rbw-tt.Test.sh
   2 tt/rbw-ts.TestSuite.fast.sh
   2 tt/rbw-tP.QualifyPristine.sh
   2 tt/rbw-tl.Shellcheck.sh
   2 tt/rbw-tb.Build.sh
   2 tt/rbw-rvv.ValidateVesselRegime.sh
   2 tt/rbw-rrv.ValidateRepoRegime.sh
   2 tt/rbw-rnr.RenderNameplateRegime.sh
   2 tt/rbw-Ric.RetrieverInspectsCompact.sh
   2 tt/rbw-rfv.ValidateFederationRegime.sh
   2 tt/rbw-rav.ValidateAuthRegime.sh
   2 tt/rbw-MZ.MarshalZeroes.sh
   2 tt/rbw-MG.MarshalGenerate.sh
   2 tt/rbw-ld.DirectorDivinesLodes.sh
   2 tt/rbw-iJe.DirectorJettisonsEnshrinement.sh
   2 tt/rbw-gPR.PayorRefresh.sh
   2 tt/rbw-gO.Onboarding.sh
   2 tt/rbw-fk.LocalKludge.sh
   2 tt/rbw-dU.PayorUnmakesDepot.sh
   2 tt/rbw-dl.PayorListsDepots.sh
   2 tt/rbw-dE.DirectorEnshrinesVessel.sh
   2 tt/rbw-DE.DirectorEnshrinesBaseImages.sh
   2 tt/rbw-DC.DirectorCreatesConsecration.sh
   2 tt/rbw-cQ.Quench.tadmor.sh
   2 tt/rbw-cC.Charge.tadmor.sh
   2 tt/rbw-cC.Charge.srjcl.sh
   2 tt/rbw-cb.Bark.tadmor.sh
   2 tt/rbw-acg.CheckGovernorCredential.sh
   2 tt/rbw-acf.CheckFederatedAccess.sh
   2 tt/rbw-acd.CheckDirectorCredential.sh
   2 tt/rbtd-t.Test.sh
   2 tt/rbtd-s.TestSuite.fast.sh
   2 tt/rbtd-s.TestSuite.crucible.sh
   2 tt/rbtd-s.SingleCase.regime-validation.sh
   2 tt/rbtd-s.FixtureCase.regime-smoke.sh
   2 tt/rbtd-s.FixtureCase.onboarding-sequence.sh
   2 tt/jjw-tfs.TestFundusScenario.localhost.sh
   2 tt/jjw-tfP2.ProvisionPhase2.cerebro.sh
   2 tt/buw-tt-ll.ListLaunchers.sh
   2 tt/buw-st.BukSelfTest.sh
   2 tt/buw-rpv.ValidatePrivilegeRegime.sh
   2 tt/buw-rpr.RenderPrivilegeRegime.sh
   2 tt/buw-rpl.ListPrivilegeRegime.sh
   2 tt/buw-rcv.ValidateBuc.sh
   2 tt/buw-qsc.QualifyShellCheck.sh
   2 tt/buw-jwk.WorkloadKnock.sh
   2 Tools/vvk/bin/vvx
   2 time
   2 TERM=xterm-256color
   2 tailscale
   2 sh
   2 rustup
   2 rustc
   2 ruby
   2 npm
   2 networksetup
   2 md5
   2 gem
   2 claude
   2 BURE_TWEAK_NAME=buorb_credless_guard
   2 BURD_LAUNCHER=".buk/launcher.buw_workbench.sh"
   2 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/vvw-r.RunVVX.sh
   2 /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/vvk/bin/vvx
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-z.Stop.tadmor.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-z.Stop.nsproto.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tw.TestSweep.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.fast.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.regime-validation.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.nsproto-security.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.QualifyFast.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-s.Start.tadmor.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rrv.ValidateRepoRegime.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rnv.ValidateNameplateRegime.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Qf.QualifyFast.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-MG.MarshalGenerate.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hs.RetrieverSummonsHallmark.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gPR.PayorRefresh.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-GD.GovernorCreatesDirector.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fO.DirectorOrdainsHallmark.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fhv.HygieneCheckVessel.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fA.DirectorAbjuresHallmark.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DV.DirectorVouchesConsecrations.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dl.PayorListsDepots.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cC.Charge.tadmor.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-t.Test.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.TestSuite.crucible.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.SingleCase.pristine-lifecycle.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.tadmor.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-b.Build.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-st.BukSelfTest.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-qsc.QualifyShellCheck.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpS.PrivilegedSsh.sh
   2 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpGb.GarrisonBash.sh
   2 /Users/bhyslop/models/stanford-deidentifier/.venv/bin/optimum-cli
   2 /System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister
   2 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh
   2 /bin/bash
   2 [
   2 ./tt/rbw-ts.TestSuite.complete.sh
   2 ./tt/rbw-tq.QualifyFast.sh
   2 ./tt/rbw-tP.QualifyPristine.sh
   2 ./tt/rbw-tf.TestFixture.kick-tires.sh
   2 ./tt/rbw-tf.QualifyFast.sh
   2 ./tt/rbw-Rs.RetrieverSummonsConsecration.sh
   2 ./tt/rbw-rnr.RenderNameplateRegime.sh
   2 ./tt/rbw-rdr.RenderDepotRegime.sh
   2 ./tt/rbw-Qf.QualifyFast.sh
   2 ./tt/rbw-Odf.OnboardingDirectorFirstBuild.sh
   2 ./tt/rbw-o.OnboardingStartHere.sh
   2 ./tt/rbw-o.ONBOARDING.sh
   2 ./tt/rbw-MZ.MarshalZeroes.sh
   2 ./tt/rbw-ir.DirectorRekonsImages.sh
   2 ./tt/rbw-iar.DirectorAuditsReliquaries.sh
   2 ./tt/rbw-iah.DirectorAuditsHallmarks.sh
   2 ./tt/rbw-hO.DirectorOrdainsHallmark.sh
   2 ./tt/rbw-go.OnboardMAIN.sh
   2 ./tt/rbw-gO.Onboarding.sh
   2 ./tt/rbw-ft.RetrieverTalliesHallmarks.sh
   2 ./tt/rbw-fk.LocalKludge.sh
   2 ./tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh
   2 ./tt/rbw-dl.PayorListsDepots.sh
   2 ./tt/rbw-DC.DirectorCreatesConsecration.sh
   2 ./tt/rbw-DC.DirectorConjuresArk.sh
   2 ./tt/rbw-cKB.KludgeBottle.sh
   2 ./tt/rbw-aM.PayorMantlesGovernor.sh
   2 ./tt/rbw-adr.GovernorRostersDirectors.sh
   2 ./tt/rbtd-t.Test.sh
   2 ./tt/rbtd-s.TestSuite.crucible.sh
   2 ./tt/rbtd-s.FixtureCase.sh
   2 ./tt/rbtd-r.Run.regime-validation.sh
   2 ./tt/rbtd-r.Run.regime-smoke.sh
   2 ./tt/rbtd-r.Run.four-mode.sh
   2 ./tt/jjw-tfS.TestFundusSingle.localhost.sh
   2 ./tt/buw-rnv.ValidateNodeRegime.sh
   2 ./tt/buw-qsc.QualifyShellCheck.sh
   1 whois
   1 while
   1 wait
   1 vm_stat
   1 umask
   1 tt/rbw-z.Stop.srjcl.sh
   1 tt/rbw-z.Stop.pluml.sh
   1 tt/rbw-z.Stop.nsproto.sh
   1 tt/rbw-ts.TestSuite.dogfight.sh
   1 tt/rbw-tq.QualifyFast.sh
   1 tt/rbw-tn.TestNameplate.srjcl.sh
   1 tt/rbw-tn.TestNameplate.pluml.sh
   1 tt/rbw-tn.TestNameplate.nsproto.sh
   1 tt/rbw-Tk.KludgeCycle.tadmor.sh
   1 tt/rbw-tK.KludgeCycle.tadmor.sh
   1 tt/rbw-tf.TestFixture.regime-validation.sh
   1 tt/rbw-tf.TestFixture.regime-smoke.sh
   1 tt/rbw-tf.TestFixture.regime-credentials.sh
   1 tt/rbw-tf.TestFixture.qualify-all.sh
   1 tt/rbw-tf.TestFixture.nsproto-security.sh
   1 tt/rbw-tf.TestFixture.kick-tires.sh
   1 tt/rbw-tf.TestFixture.enrollment-validation.sh
   1 tt/rbw-tf.TestFixture.ark-lifecycle.sh
   1 tt/rbw-tf.TestFixture.access-probe.sh
   1 tt/rbw-tf.QualifyFast.sh
   1 tt/rbw-tc.FixtureCase.sh
   1 tt/rbw-ta.TestAll.sh
   1 tt/rbw-s.Start.pluml.sh
   1 tt/rbw-s.Start.nsproto.sh
   1 tt/rbw-rvv.ValidateVessel.sh
   1 tt/rbw-rvl.ListVesselRegime.sh
   1 tt/rbw-rva.DirectorAnointsGraftVessel.sh
   1 tt/rbw-rsr.RenderStationRegime.sh
   1 tt/rbw-Rs.RetrieverSummonsArk.sh
   1 tt/rbw-rrr.RenderRepoRegime.sh
   1 tt/rbw-rov.ValidateOauthRegime.sh
   1 tt/rbw-Rl.RetrieverListsImages.sh
   1 tt/rbw-RiF.RetrieverInspectsFull.sh
   1 tt/rbw-rfr.RenderFederationRegime.sh
   1 tt/rbw-rdr.RenderDepotRegime.sh
   1 tt/rbw-pF.FreeholdProof.sh
   1 tt/rbw-PC.PayorCreatesDepot.sh
   1 tt/rbw-Op.OnboardingPayor.sh
   1 tt/rbw-Ofc.OnboardingFirstCrucible.sh
   1 tt/rbw-Odg.OnboardingDirectorGraft.sh
   1 tt/rbw-Odf.OnboardingDirectorFirstBuild.sh
   1 tt/rbw-Odb.OnboardingDirectorBind.sh
   1 tt/rbw-Oda.OnboardingDirectorAirgap.sh
   1 tt/rbw-Ocd.OnboardingCredentialDirector.sh
   1 tt/rbw-Occ.OnboardingCrashCourse.sh
   1 tt/rbw-Occ.OnboardingConfigureEnvironment.sh
   1 tt/rbw-o.OnboardingStartHere.sh
   1 tt/rbw-o.ONBOARDING.sh
   1 tt/rbw-ni.NameplateInfo.sh
   1 tt/rbw-MR.MarshalReset.sh
   1 tt/rbw-MD.MarshalDuplicate.sh
   1 tt/rbw-mA.PayorAffiancesManor.sh
   1 tt/rbw-LK.LocalKludge.sh
   1 tt/rbw-lI.DirectorImmuresPodvm.sh
   1 tt/rbw-la.DirectorAugursLode.sh
   1 tt/rbw-irr.DirectorRekonsReliquary.sh
   1 tt/rbw-il.DirectorListsRegistry.sh
   1 tt/rbw-iJr.DirectorJettisonsReliquaryImage.sh
   1 tt/rbw-iar.DirectorAuditsReliquaries.sh
   1 tt/rbw-iah.DirectorAuditsHallmarks.sh
   1 tt/rbw-iae.DirectorAuditsEnshrinements.sh
   1 tt/rbw-HWdd.DockerDesktop.sh
   1 tt/rbw-HWdc.DockerContextDiscipline.sh
   1 tt/rbw-hw.HandbookWindows.sh
   1 tt/rbw-hw
   1 tt/rbw-h0.HandbookTOP.sh
   1 tt/rbw-gq.QuotaBuild.sh
   1 tt/rbw-gPo.PayorOnboarding.sh
   1 tt/rbw-gPE.PayorEstablish.sh
   1 tt/rbw-gOR.OnboardRetriever.sh
   1 tt/rbw-go.OnboardMAIN.sh
   1 tt/rbw-fs.RetrieverSummonsHallmark.sh
   1 tt/rbw-fpf.RetrieverPlumbsFull.sh
   1 tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh
   1 tt/rbw-dt.TerrierScaffold.sh
   1 tt/rbw-dr.DepotRecognosce.sh
   1 tt/rbw-DO.DirectorOrdainsConsecration.sh
   1 tt/rbw-DI.DirectorInscribesRubric.sh
   1 tt/rbw-DI.DirectorInscribesReliquary.sh
   1 tt/rbw-dI.DirectorInscribesReliquary.sh
   1 tt/rbw-DE.DirectorEnshrinesVessel.sh
   1 tt/rbw-DC.DirectorConjuresArk.sh
   1 tt/rbw-Dc.DirectorChecksConsecrations.sh
   1 tt/rbw-cw.Writ.tadmor.sh
   1 tt/rbw-cQ.Quench.srjcl.sh
   1 tt/rbw-cKS.KludgeSentry.sh
   1 tt/rbw-cKB.KludgeBottle.tadmor.sh
   1 tt/rbw-cKB.KludgeBottle.sh
   1 tt/rbw-cic.CrucibleIsCharged.sh
   1 tt/rbw-arI.GovernorInvestsRetriever.sh
   1 tt/rbw-aM.PayorMantlesGovernor.sh
   1 tt/rbw-adr.GovernorRostersDirectors.sh
   1 tt/rbw-adI.GovernorInvestsDirector.sh
   1 tt/rbw-acr.CheckRetrieverCredential.sh
   1 tt/rbw-acp.CheckPayorCredential.sh
   1 tt/rbtd-s.TestSuite.service.sh
   1 tt/rbtd-s.TestSuite.dogfight.sh
   1 tt/rbtd-s.SingleCase.srjcl.sh
   1 tt/rbtd-s.SingleCase.handbook-render.sh
   1 tt/rbtd-s.SingleCase.canonical-establish.sh
   1 tt/rbtd-s.FixtureCase.regime-validation.sh
   1 tt/rbtd-s.FixtureCase.moriah.sh
   1 tt/rbtd-r.Run.tadmor.sh
   1 tt/rbtd-r.Run.regime-validation.sh
   1 tt/rbtd-r.Run.regime-smoke.sh
   1 tt/rbtd-r.Run.pristine-lifecycle.sh
   1 tt/rbtd-r.Run.handbook-render.sh
   1 tt/rbtd-r.Run.four-mode.sh
   1 tt/rbtd-r.FixtureRun.srjcl.sh
   1 tt/rbtd-r.FixtureRun.regime-validation.sh
   1 tt/rbtd-r.FixtureRun.regime-smoke.sh
   1 tt/rbtd-r.FixtureRun.onboarding-sequence.sh
   1 tt/rbtd-r.FixtureRun.handbook-render.sh
   1 tt/rbtd-r.FixtureRun.hallmark-lifecycle.sh
   1 tt/rbtd-r.FixtureRun.enrollment-validation.sh
   1 tt/rbtd-ap.AccessProbe.payor.sh
   1 tt/jjw-tfS.TestFundusSingle.localhost.sh
   1 tt/jjw-tfs.TestFundusScenario.cerebro.sh
   1 tt/jjw-tfP2.ProvisionPhase2.localhost.sh
   1 tt/jjw-tfP.ProvisionFundusAccounts.localhost.sh
   1 tt/buw-tt-cbl.CreateTabTargetBatchLogging.sh
   1 tt/buw-rcv.sh
   1 tt/buw-rcr.RenderConfigRegime.sh
   1 tt/buw-jwk.Knock.sh
   1 tt/buw-jpGw.GarrisonWsl.sh
   1 tt/buw-jpGb.GarrisonBash.sh
   1 tt/buw-jpF.Fenestrate.sh
   1 tt/buw-jpCM.CaparisonMacos.sh
   1 tt/buw-jpCL.CaparisonLinux.sh
   1 tt/buw-hw.HandbookWindows.sh
   1 tt/buw-hjw.HandbookJurisdictionWindows.sh
   1 tt/buw-hj0.HandbookJurisdictionTop.sh
   1 tt/buw-h0.HandbookTOP.sh
   1 tt/apcw-t.Test.sh
   1 tt/apcw-nsx.NeuralStanfordExport.sh
   1 tt/apcw-nsi.NeuralStanfordInstall.sh
   1 tt/apcw-nsa.NeuralStanfordAssay.sh
   1 tt/apcw-cx.ContainerStop.sh
   1 tt/apcw-cs.ContainerStart.sh
   1 tt/apcw-ci.ContainerStatus.sh
   1 tt/apcw-cb.ContainerBuild.sh
   1 tt/apcw-ba.BatchAssay.sh
   1 tt/apcw-b.Build.sh
   1 tput
   1 top
   1 then
   1 test
   1 TERM=dumb
   1 system_profiler
   1 sysctl
   1 sshpass
   1 ssh-keygen
   1 ssh-add
   1 skopeo
   1 sftp
   1 set
   1 Read
   1 pstree
   1 ps
   1 plutil
   1 pip
   1 pandoc
   1 open
   1 NO_COLOR=1
   1 mdutil
   1 mdls
   1 mdfind
   1 lsof
   1 LOG_DIR="/Users/bhyslop/projects/rbm_alpha_recipemuster/../_logs_buk"
   1 LC_ALL=C
   1 JJTEST_HOST=localhost
   1 ifconfig
   1 id
   1 iconv
   1 head
   1 getent
   1 gawk
   1 fi
   1 external
   1 export
   1 env
   1 DV
   1 dseditgroup
   1 dscacheutil
   1 done
   1 diskutil
   1 DE
   1 conjure
   1 BURE_VERBOSE=1
   1 BURE_TWEAK_NAME=buost_regime_poison
   1 BURD_NO_HYPERLINKS=1
   1 BURD_LAUNCHER=".buk/launcher.rbw_workbench.sh"
   1 build
   1 BUC_VERBOSE=0
   1 break
   1 arp
   1 ark
   1 2
   1 \\\\
   1 /usr/bin/gcloud
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/vow-t.Test.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/vow-b.Build.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tt.Test.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-ts.TestSuite.fast.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tq.QualifyFast.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tP.QualifyPristine.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tl.Shellcheck.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tf.FixtureRun.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tc.FixtureCase.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tb.Build.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-lE.DirectorEnsconcesBole.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-ld.DirectorDivinesLodes.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-irh.DirectorRekonsHallmark.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-iar.DirectorAuditsReliquaries.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-fO.DirectorOrdainsHallmark.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dl.PayorListsDepots.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbtd-r.Run.four-mode.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbtd-r.FixtureRun.srjcl.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbtd-r.FixtureRun.pluml.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbtd-b.Build.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/tt/buw-st.BukSelfTest.sh
   1 /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/target/debug/theurge
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/vow-t.Test.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/vow-b.Build.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-z.Stop.srjcl.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tt.Test.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tT.QualifyTadmor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSweep.fast.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.skirmish.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.service.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.dogfight.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.crucible.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.complete.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tS.QualifySkirmish.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tq.QualifyFast.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-to.TestOne.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tl.Shellcheck.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.three-mode.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.tadmor-security.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.srjcl-jupyter.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.regime-smoke.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.regime-credentials.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.qualify-all.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.pluml-diagram.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.kick-tires.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.four-mode.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.enrollment-validation.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.ark-lifecycle.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.access-probe.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tc.FixtureCase.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tb.Build.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-s.Start.srjcl.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-s.Start.nsproto.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rw.RetrieverWrestsImage.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvy.DirectorYokesReliquaryAllVessels.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvl.ListVesselRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rv.RetrieverVouchesArk.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rpv.ValidatePayorRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rnr.RenderNameplateRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rnl.ListNameplateRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-RiF.RetrieverInspectsFull.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Ric.RetrieverInspectsCompact.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rav.ValidateAuthRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rar.RenderAuthRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-qa.QualifyAll.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PO.PayorOnboarding.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Pl.PayorListsDepots.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PG.PayorResetsGovernor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PC.PayorCreatesDepot.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Op.OnboardingPayor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Og.OnboardingGovernor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Ofc.OnboardingFirstCrucible.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Odf.OnboardingDirectorFirstBuild.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Odb.OnboardingDirectorBind.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Oda.OnboardingDirectorAirgap.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Ocr.OnboardingCredentialRetriever.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Ocd.OnboardingCredentialDirector.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Occ.OnboardingCrashCourse.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Occ.OnboardingConfigureEnvironment.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-o.OnboardingStartHere.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-o.ONBOARDING.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-nv.ValidateNameplates.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-MR.MarshalReset.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-MD.MarshalDuplicate.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-lp.DirectorPresagesImmure.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-lI.DirectorImmuresPodvm.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-lC.DirectorConclavesReliquary.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-la.DirectorAugursLode.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-irh.DirectorRekonsHallmark.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ir.DirectorRekonsImages.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-iar.DirectorAuditsReliquaries.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-iah.DirectorAuditsHallmarks.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-HWha.HostAvailability.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-HWdw.DockerWSLNative.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-HWdd.DockerDesktop.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-HWdc.DockerContextDiscipline.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hw.HandbookWindows.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-h0.HandbookTOP.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-GS.GovernorDeletesServiceAccount.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-GR.GovernorCreatesRetriever.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gq.QuotaBuild.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gPo.PayorOnboarding.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gPE.PayorEstablish.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOR.OnboardRetriever.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOr.OnboardReference.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOP.OnboardPayor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOG.OnboardGovernor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOD.OnboardDirector.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-go.OnboardMAIN.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gO.Onboarding.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ft.RetrieverTalliesHallmarks.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fs.RetrieverSummonsHallmark.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fpc.RetrieverPlumbsCompact.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fhc.HygieneCheck.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Dt.DirectorTalliesConsecrations.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DPG.DirectorRefreshesGcbPins.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DP.DirectorRefreshesPins.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DI.DirectorInscribesRubric.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-di.DepotInfo.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dE.DirectorEnshrinesVessel.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesArk.rbev-busybox.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorConjuresArk.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Dc.DirectorChecksConsecrations.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Dc.CheckConsecrations.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DB.DirectorBeseechesArk.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-da.DepotAttribution.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cQ.Quench.tadmor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cQ.Quench.ccyolo.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cKS.KludgeSentry.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cic.CrucibleIsCharged.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cC.Charge.ccyolo.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-arD.GovernorDivestsRetriever.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-aM.PayorMantlesGovernor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-adI.GovernorInvestsDirector.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-adD.GovernorDivestsDirector.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.TestSuite.service.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.TestSuite.fast.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.SingleCase.tadmor.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.FixtureCase.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.FixtureCase.canonical-onboarding-sequence.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.tadmor-security.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.regime-validation.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.pristine-lifecycle.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.four-mode.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.access-probe.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.regime-validation.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.regime-smoke.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.handbook-render.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.enrollment-validation.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.dockerfile-hygiene.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbi-iK.IfritKludge.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbi-iB.IfritBuild.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rsv.ValidateStationRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rpl.PrivilegeRegimeList.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rpl.ListPrivilegeRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnva.ValidateAllNodeRegimes.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnr.RenderNodeRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnl.NodeRegimeList.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnl.ListNodeRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcx.ConstructLocalhost.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcw.ConstructWSL.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcp.ConstructPowerShell.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcl.ConstructLinux.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcc.ConstructCygwin.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rev.ValidateEnvironmentRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rer.RenderEnvironmentRegime.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jws.WorkloadInteractiveSession.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jwk.WorkloadKnock.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpS
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpGw.GarrisonWsl.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpF.Fenestrate.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpCW.CaparisonWindows.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWew.EnvironmentWSL.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWec.EnvironmentCygwin.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWax.AccessEntrypoints.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWar.AccessRemote.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWab.AccessBase.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-hw.HandbookWindows.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-hjm.HandbookJurisdictionMacos.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-hjl.HandbookJurisdictionLinux.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-hj0.HandbookJurisdictionTop.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/apcw-t.Test.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/apcw-r.Run.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/apcw-ba.BatchAssay.sh
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvk/bin/vvx-darwin-arm64
   1 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvk/bin/vvr-darwin-arm64
   1 /Users/bhyslop/projects/pb_paneboard02/tt/pbw-t.ProofOfConceptTimed.10.sh
   1 /tmp/vvk1013_install/vvi_install.sh
   1 /tmp/sibling_analysis.sh
   1 /tmp/sc2153_backstop_test.sh
   1 /tmp/rbspike/mint_payor.sh
   1 /tmp/rbrm_verification.txt
   1 /tmp/p1015full/vvi_install.sh
   1 /tmp/p1014/vvi_install.sh
   1 /tmp/count_tests.sh
   1 /tmp/analyze_rbs0.sh
   1 /tmp/analyze_hardcoding.sh
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/vow-b.Build.sh
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tt.Test.sh
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.skirmish.sh
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.fast.sh
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.dogfight.sh
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.crucible.sh
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tl.Shellcheck.sh
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tb.Build.sh
   1 /home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-adI.GovernorInvestsDirector.sh
   1 /context
   1 [[
   1 "
   1 ./tt/vow-R.ParcelRelease.sh
   1 ./tt/vow-F.Freshen.sh
   1 ./tt/rbw-z.Stop.tadmor.sh
   1 ./tt/rbw-ts.TestSuite.tadmor.sh
   1 ./tt/rbw-ts.TestSuite.skirmish.sh
   1 ./tt/rbw-ts.TestSuite.siege.sh
   1 ./tt/rbw-ts.TestSuite.crucible.sh
   1 ./tt/rbw-ts.TestSuite.blockade.sh
   1 ./tt/rbw-tS.QualifySkirmish.sh
   1 ./tt/rbw-to.TestOne.sh
   1 ./tt/rbw-tO.OrdainCycle.tadmor.sh
   1 ./tt/rbw-tK.KludgeCycle.tadmor.sh
   1 ./tt/rbw-tf.TestFixture.three-mode.sh
   1 ./tt/rbw-tf.TestFixture.tadmor-security.sh
   1 ./tt/rbw-tf.TestFixture.srjcl-jupyter.sh
   1 ./tt/rbw-tf.TestFixture.slsa-provenance.sh
   1 ./tt/rbw-tf.TestFixture.regime-validation.sh
   1 ./tt/rbw-tf.TestFixture.regime-smoke.sh
   1 ./tt/rbw-tf.TestFixture.qualify-all.sh
   1 ./tt/rbw-tf.TestFixture.pluml-diagram.sh
   1 ./tt/rbw-tf.TestFixture.enrollment-validation.sh
   1 ./tt/rbw-tf.TestFixture.ark-lifecycle.sh
   1 ./tt/rbw-s.Start.tadmor.sh
   1 ./tt/rbw-rvl.ListVesselRegime.sh
   1 ./tt/rbw-rv.RegimeValidate.sh
   1 ./tt/rbw-rrv.ValidateRepoRegime.sh
   1 ./tt/rbw-rrr.RenderRepoRegime.sh
   1 ./tt/rbw-rpv.ValidatePayorRegime.sh
   1 ./tt/rbw-rpr.RenderPayorRegime.sh
   1 ./tt/rbw-rov.ValidateOauthRegime.sh
   1 ./tt/rbw-rnr.RenderNameplateRegime.tadmor.sh
   1 ./tt/rbw-rnl.ListNameplateRegime.sh
   1 ./tt/rbw-rgr.RenderPinsRegime.sh
   1 ./tt/rbw-rfv.ValidateFederationRegime.sh
   1 ./tt/rbw-rdv.ValidateDepotRegime.sh
   1 ./tt/rbw-ral.ListAuthRegimes.sh
   1 ./tt/rbw-qa.QualifyAll.sh
   1 ./tt/rbw-Ots.OnboardingTadmorSecurity.sh
   1 ./tt/rbw-Op.OnboardingPayor.sh
   1 ./tt/rbw-Og.OnboardingGovernor.sh
   1 ./tt/rbw-Ofc.OnboardingFirstCrucible.sh
   1 ./tt/rbw-Ocr.OnboardingCredentialRetriever.sh
   1 ./tt/rbw-Ocd.OnboardingCredentialDirector.sh
   1 ./tt/rbw-Occ.OnboardingCrashCourse.sh
   1 ./tt/rbw-Occ.OnboardingConfigureEnvironment.sh
   1 ./tt/rbw-nv.ValidateNameplates.sh
   1 ./tt/rbw-ni.NameplateInfo.sh
   1 ./tt/rbw-LK.LocalKludge.sh
   1 ./tt/rbw-Is.IfritSortie.tadmor.sh
   1 ./tt/rbw-iJr.DirectorJettisonsReliquaryImage.sh
   1 ./tt/rbw-iae.DirectorAuditsEnshrinements.sh
   1 ./tt/rbw-HWdw.DockerWSLNative.sh
   1 ./tt/rbw-HWdd.DockerDesktop.sh
   1 ./tt/rbw-HWdc.DockerContextDiscipline.sh
   1 ./tt/rbw-hw.HandbookWindows.sh
   1 ./tt/rbw-hs.RetrieverSummonsHallmark.sh
   1 ./tt/rbw-h0.HandbookTOP.sh
   1 ./tt/rbw-gPR.PayorRefresh.sh
   1 ./tt/rbw-gPE.PayorEstablish.sh
   1 ./tt/rbw-gOR.OnboardRetriever.sh
   1 ./tt/rbw-gOr.OnboardReference.sh
   1 ./tt/rbw-gOP.OnboardPayor.sh
   1 ./tt/rbw-gOG.OnboardGovernor.sh
   1 ./tt/rbw-gOD.OnboardDirector.sh
   1 ./tt/rbw-fs.RetrieverSummonsHallmark.sh
   1 ./tt/rbw-fpc.RetrieverPlumbsCompact.sh
   1 ./tt/rbw-fO.DirectorOrdainsHallmark.sh
   1 ./tt/rbw-fA.DirectorAbjuresHallmark.sh
   1 ./tt/rbw-DV.DirectorVouchesArk.sh
   1 ./tt/rbw-dU.PayorUnmakesDepot.sh
   1 ./tt/rbw-DPG.DirectorRefreshesGcbPins.sh
   1 ./tt/rbw-DP.DirectorRefreshesPins.sh
   1 ./tt/rbw-dL.PayorLeviesDepot.sh
   1 ./tt/rbw-dI.DirectorInscribesReliquary.sh
   1 ./tt/rbw-dE.DirectorEnshrinesVessel.sh
   1 ./tt/rbw-Dc.DirectorChecksConsecrations.sh
   1 ./tt/rbw-DA.DirectorAbjuresArk.sh
   1 ./tt/rbw-cQ.Quench.srjcl.sh
   1 ./tt/rbw-cQ.Quench.ccyolo.sh
   1 ./tt/rbw-cKS.KludgeSentry.tadmor.sh
   1 ./tt/rbw-cKB.KludgeBottle.tadmor.sh
   1 ./tt/rbw-cK.Kludge.tadmor.sh
   1 ./tt/rbw-cic.CrucibleIsCharged.tadmor.sh
   1 ./tt/rbw-ch.Hail.sh
   1 ./tt/rbw-cC.Charge.srjcl.sh
   1 ./tt/rbw-cC.Charge.ccyolo.sh
   1 ./tt/rbw-ca.CrucibleActive.sh
   1 ./tt/rbw-arr.GovernorRostersRetrievers.sh
   1 ./tt/rbw-arI.GovernorInvestsRetriever.sh
   1 ./tt/rbw-adD.GovernorDivestsDirector.sh
   1 ./tt/rbw-acr.CheckRetrieverCredential.sh
   1 ./tt/rbw-acp.CheckPayorCredential.sh
   1 ./tt/rbw-acg.CheckGovernorCredential.sh
   1 ./tt/rbw-acf.CheckFederatedAccess.sh
   1 ./tt/rbw-acd.CheckDirectorCredential.sh
   1 ./tt/rbtd-s.TestSuite.service.sh
   1 ./tt/rbtd-s.TestSuite.fast.sh
   1 ./tt/rbtd-s.SingleCase.regime-smoke.sh
   1 ./tt/rbtd-s.SingleCase.four-mode.sh
   1 ./tt/rbtd-s.SingleCase.enrollment-validation.sh
   1 ./tt/rbtd-r.Run.tadmor.sh
   1 ./tt/rbtd-r.Run.srjcl.sh
   1 ./tt/rbtd-r.Run.pluml.sh
   1 ./tt/rbtd-r.Run.enrollment-validation.sh
   1 ./tt/rbtd-r.Run.access-probe.sh
   1 ./tt/rbtd-r.FixtureRun.tadmor.sh
   1 ./tt/rbtd-r.FixtureRun.tadmor-security.sh
   1 ./tt/rbtd-r.FixtureRun.srjcl.sh
   1 ./tt/rbtd-r.FixtureRun.pluml.sh
   1 ./tt/rbtd-r.FixtureRun.moriah.sh
   1 ./tt/rbtd-r.FixtureRun.handbook-render.sh
   1 ./tt/rbtd-r.FixtureRun.canonical-establish.sh
   1 ./tt/rbtd-r.FixtureRun.access-probe.sh
   1 ./tt/rbi-iB.IfritBuild.sh
   1 ./tt/jjw-tfs.TestFundusScenario.localhost.sh
   1 ./tt/jjw-tfs.TestFundusScenario.cerebro.sh
   1 ./tt/jjw-tfP2.ProvisionPhase2.localhost.sh
   1 ./tt/buw-xd.Delay.sh
   1 ./tt/buw-tt-ll.ListLaunchers.sh
   1 ./tt/buw-ts.TestSweep.sh
   1 ./tt/buw-ta.TestAll.sh
   1 ./tt/buw-SI.StationInit.sh
   1 ./tt/buw-rsr.RenderStationRegime.sh
   1 ./tt/buw-rpr.RenderPrivilegeRegime.sh
   1 ./tt/buw-rnr.RenderNodeRegime.sh
   1 ./tt/buw-rer.RenderEnvironmentRegime.sh
   1 ./tt/buw-rcr.RenderConfigRegime.sh
   1 ./tt/buw-jwk.WorkloadKnock.sh
   1 ./tt/buw-jpW.WslInstall.sh
   1 ./tt/buw-jpS.PrivilegedSsh.sh
   1 ./tt/buw-jpIW.InvigilateWindows.sh
   1 ./tt/buw-jpGw.GarrisonWsl.sh
   1 ./tt/buw-jpF.Fenestrate.sh
   1 ./tt/buw-jpCW.CaparisonWindows.sh
   1 ./tt/buw-HWew.EnvironmentWSL.sh
   1 ./tt/buw-HWec.EnvironmentCygwin.sh
   1 ./tt/buw-HWax.AccessEntrypoints.sh
   1 ./tt/buw-HWar.AccessRemote.sh
   1 ./tt/buw-HWab.AccessBase.sh
   1 ./tt/buw-hw.HandbookWindows.sh
   1 ./tt/buw-hj0.HandbookJurisdictionTop.sh
   1 ./tt/buw-h0.HandbookTOP.sh
   1 ./tt/buw-dly.Delay.sh
   1 ./tt/buw-d.Delay.sh
   1 ./tt/apcw-t.Test.sh
   1 ./Tools/vvk/bin/vvx
```

### MCP / Job-Jockey absorption candidates (git / gh)
```
gh api:*
gh auth:*
gh issue *
gh repo:*
git *
git log *
git push *
git show *
git stash:*
git status *
git:*
```

### Risky / over-broad grants — NOT carried forward (security record)
```
bash -c 'source ../buk/buc_command.sh; source rbfl0_FoundryLedger.sh && declare -F zrbld_cloud_delete_dispatch zrbld_spine_dispatch rbfl_abjure >/dev/null && echo "RBFL OK: ${ZRBLDS_SOURCED:-unset}/${ZRBLDD_SOURCED:-unset} guards set, delete+spine+abjure defined"'
bash -c 'source ../buk/buc_command.sh; source rbld0_Lode.sh && declare -F zrbld_cloud_delete_dispatch zrbld_spine_dispatch rbld_banish >/dev/null && echo "RBLD OK: ${ZRBLDS_SOURCED:-unset}/${ZRBLDD_SOURCED:-unset} guards set, delete+spine+banish defined"'
bash -c 'source Tools/rbk/rbz_zipper.sh && zrbz_kindle && echo "${ZRBZ_COLOPHON_MANIFEST} rbtd-ap"'
bash -n /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgi_IAM.sh
bash -n /Users/bhyslop/projects/rbm_beta_recipemuster/__TRACKED_VAR__
bash -n /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/buc_command.sh
bash -n /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/bujb_jurisdiction.sh
bash -n /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbgp_Payor.sh
bash -n Tools/buk/buc_command.sh
bash -n Tools/buk/burs_regime.sh
bash -n Tools/buk/buym_yelp.sh
bash -n Tools/rbk/rbcc_Constants.sh
bash -n Tools/rbk/rbgg_governor.sh
bash -n Tools/rbk/rbgp_payor.sh
bash -n Tools/rbk/rbob_bottle.sh
bash -n Tools/rbk/rbrr_regime.sh
bash -n Tools/rbk/rbz_zipper.sh
bash /tmp/check_drifts.sh
bash /tmp/clusters.sh
bash /tmp/count_roles.sh
bash /tmp/detailed_analysis.sh
bash /tmp/rbk_dirlog.sh 73a38a23-8901-471f-a20c-6e25fa067908
bash /tmp/validation.sh
bash:*
chmod +x /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-mA.PayorAffiancesManor.sh
chmod +x Tools/rbk/rbld_cli.sh
chmod +x Tools/rbk/rbrf_cli.sh
chmod +x Tools/rbk/rbrf_regime.sh
chmod +x tt/rbw-acf.CheckFederatedAccess.sh
chmod +x tt/rbw-il.DirectorListsRegistry.sh tt/rbw-iw.DirectorWrestsImage.sh tt/rbw-iJ.DirectorJettisonsImage.sh
chmod +x tt/rbw-lC.DirectorConclavesReliquary.sh
chmod +x tt/rbw-lE.DirectorEnsconcesBase.sh tt/rbw-ld.DirectorDivinesLodes.sh tt/rbw-lB.DirectorBanishesLode.sh
chmod +x tt/rbw-lU.DirectorUnderpinsWsl.sh
chmod +x tt/rbw-rfr.RenderFederationRegime.sh tt/rbw-rfv.ValidateFederationRegime.sh
chmod +x:*
chmod:*
env
perl -0pi -e "s/printf '%s' \\"\\\\\\$\\{z_buym_format\\}\\" >&2/printf '%b' \\"\\\\\\${z_buym_format}\\" >&2/g" Tools/buk/buts/butcym_YelpModule.sh
perl -i -pe 's/\(jjtsc_make_tack\\\([^\)]*?\), None\\\)/$1\)/g' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjtsc_scout.rs
perl -i -pe 's/\(make_valid_tack\\\([^\)]*?\), None\\\)/$1\)/g' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjtg_gallops.rs
perl -i -pe 's/\\bRBTDRK_RBRR_FILE\\b/RBTDGC_RBRR_FILE/g; *
perl -i -pe 's/\\bRBTDRM_ROLE_/RBTDGC_ROLE_/g; s/crate::RBTD_MOORINGS_DIR\\b/crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR/g' __TRACKED_VAR__.rs
perl -i -pe 's/\\bRBTDRP_DOT_DIR\\b/RBTDGC_MOORINGS_DIR/g; *
perl -i -pe 's/RBTDRM_ROLE_/RBTDGC_ROLE_/g; s/rbtdrk_canonical_rbra\\\(&root, "assay"\\\)/rbtdrk_canonical_rbra\(\\&root, RBTDGC_ROLE_ASSAY\)/g' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/src/rbtdrk_canonical.rs
perl -i -pe 's/rbtdrp_canonical_rbra\\\(&root, "assay"\\\)/rbtdrp_canonical_rbra\(\\&root, RBTDGC_ROLE_ASSAY\)/g' rbtdrp_pristine.rs
perl:*
python3 -c 'import json,sys; d=json.load\(sys.stdin\); print\("client_email:", d.get\("client_email"\)\); print\("project_id:", d.get\("project_id"\)\)'
python3 -c ":*
python3 *
python3:*
rm -f /tmp/scp_probe.txt
rm -f /tmp/tier5_check.sh /tmp/tier5_check2.sh
rm -f /tmp/tier6_check.sh
rm -rf /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid/target
rm -rf p1014
rm -rf vvk1013_install
rm /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/*.bak
rm Tools/rbk/rbh0/rbhpq_quota_build.sh.bak
rm tt/rbw-Db.DirectorBuildsAbout.sh
ruby --version
ruby -e 'require "asciidoctor"; puts "ASCIIDOCTOR_GEM_OK"'
sh -c *
sh -n /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels/common-sentry-context/rbjs_sentry.sh
sudo -n arp -d 169.254.105.91
sudo -n arp-scan --interface=en0 169.254.0.0/16 --retry=2 --timeout=500
sudo -n ipconfig set en0 DHCP
sudo -n lsof -iUDP:67
sudo -n route delete -net 169.254 -interface en1
sudo -n route delete 169.254.105.91
sudo -n true
sudo pkill *
sudo tcpdump *
sudo tee *
xargs -0 shellcheck --rcfile=Tools/buk/busc_shellcheckrc -S style -f gcc
xargs -I {} basename {}
xargs -I {} grep -l '^export BURD_INTERACTIVE=1$' {}
xargs -I {} sh -c 'echo "=== {} ==="; grep -E "source.*buh_handbook|source.*buym_yelp" "{}"'
xargs -I{} git log -1 --format="%H %s" {}
xargs -I{} sh -c 'test -f "{}rbrn.env" && echo "{}"'
xargs -n1 basename
xargs '-I{}' sh -c 'size=$\(git -C /Users/bhyslop/projects/rbm_alpha_recipemuster show "{}:.claude/jjm/jjg_gallops.json" 2>/dev/null | wc -c\); echo "$size {}"'
xargs basename *
xargs cat
xargs cat:*
xargs ls -la
xargs ls -lt
xargs sed -n '1,30p'
xargs shellcheck --rcfile=Tools/buk/busc_shellcheckrc -S style -f gcc
```

### Non-Bash rules (union)
```
Edit
mcp__vvx__jjx
mcp__vvx__jjx_test_echo
Read
Read(//home/bhyslop/projects/logs-buk/**)
Read(//home/bhyslop/projects/station-files/secrets/**)
Read(//home/bhyslop/projects/temp-buk/temp-20260605-021752-24633-709/rbtd/burv-temp/invoke-00001/temp-20260605-021802-25359-385/**)
Read(//home/bhyslop/projects/temp-buk/temp-20260605-021752-24633-709/rbtd/rbtdrk_director_divest/**)
Read(//home/bhyslop/projects/temp-buk/temp-20260605-021752-24633-709/rbtd/rbtdrk_director_divest//**)
Read(//private/tmp/**)
Read(//private/tmp/piebald-ccsp/**)
Read(//private/tmp/piebald-ccsp/system-prompts/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-14337/rbtdrp_marshal_zero_attestation/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-14972/rbtdrp_depot_lifecycle/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-15319/burv/invoke-00000/output/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-15319/burv/invoke-00000/temp/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-20366/rbtdrp_governor_lifecycle/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-23616/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-2548/rbtdrc_ifrit_dns_allowed/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-28256/burv/invoke-00001/temp/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-28256/rbtdrc_hallmark_lifecycle/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-28632/rbtdrf_rv_rbrv_all_vessels/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-3204/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-32075/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-32075/burv/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-33301/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-36339/burv/invoke-00005/output/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-36339/burv/invoke-00005/output/current/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-36339/rbtdrc_hallmark_lifecycle/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-41269/rbtdrf_rs_rbrn/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-41269/rbtdrf_rs_rbrv/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-45443/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-45679/rbtdrc_jwt_director/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-46724/rbtdrc_ifrit_dns_allowed/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-56968/rbtdrf_dh_all_vessels_pass/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-63466/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-64874/burv/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-64874/burv/invoke-00004/output/current/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-93546/**)
Read(//private/var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-93546/rbtdrk_retriever_invest/**)
Read(//tmp/**)
Read(//Users/bhyslop/.claude/**)
Read(//Users/bhyslop/**)
Read(//Users/bhyslop/projects/_logs_buk/**)
Read(//Users/bhyslop/projects/**)
Read(//Users/bhyslop/projects/logs-buk/**)
Read(//Users/bhyslop/projects/rbm_beta_recipemuster/**)
Read(//Users/bhyslop/projects/rbm_beta_recipemuster/\(.access_token /**)
Read(//Users/bhyslop/projects/station-files/**)
Read(//Users/bhyslop/projects/station-files/secrets/**)
Read(//Users/bhyslop/projects/station-files/secrets//**)
Read(//Users/bhyslop/projects/station-files/secrets/payor/**)
Read(//Users/bhyslop/projects/temp-buk/temp-20260314-204254-12274-214/**)
Read(//Users/bhyslop/projects/temp-buk/temp-20260329-203811-8857-55/**)
Read(//Users/bhyslop/projects/temp-buk/temp-20260603-102505-21344-751/**)
Read(//Users/bhyslop/projects/temp-buk/temp-20260604-092601-3729-496/rbtd/rbtdrk_director_invest/**)
Read(//Users/bhyslop/projects/temp-buk/temp-20260609-140911-2534-480/rbtd/burv-output/invoke-00003/current/**)
Read(//Users/bhyslop/projects/temp-buk/temp-20260609-140911-2534-480/rbtd/burv-temp/invoke-00003/temp-20260609-141111-4074-965/**)
Read(//Users/bhyslop/projects/temp-buk/temp-20260609-140911-2534-480/rbtd/rbtdrc_reliquary_lifecycle/**)
Read(//Users/bhyslop/projects/temp-buk/temp-20260609-140911-2534-480/rbtd/rbtdrc_reliquary_lifecycle//**)
Read(//Users/bhyslop/projects/temp-buk/temp-20260609-202135-51091-695/rbtd/burv-output//**)
Read(//Users/bhyslop/projects/temp-buk/temp-20260609-202135-51091-695/rbtd/burv-output/invoke-00000//**)
Read(//Users/bhyslop/projects/temp-buk/temp-20260609-202135-51091-695/rbtd/burv-temp//**)
Read(//Users/bhyslop/projects/temp-buk/temp-20260609-202135-51091-695/rbtd/burv-temp/invoke-00000//**)
Read(//Users/bhyslop/projects/temp-buk/temp-20260609-202135-51091-695/rbtd/burv-temp/invoke-00000/temp-20260609-204915-5349-210/**)
Read(//usr/lib/google-cloud-sdk/bin/**)
Read(//usr/local/bin/**)
Read(//var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-15319/burv/invoke-00000/output/current/**)
Read(//var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-27217/ordain-abjure-roundtrip/**)
Read(//var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-27217/ordain-abjure-roundtrip/rbtdrc_hallmark_lifecycle/**)
Read(//var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-33301/**)
Read(//var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-46724/rbtdrc_ifrit_dns_allowed/**)
Read(//var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-70009/rbtdrf_rv_rbrv_all_vessels/**)
Skill(claude-api)
Skill(update-config)
WebFetch
WebFetch(domain:cdimage.ubuntu.com)
WebFetch(domain:cloud-images.ubuntu.com)
WebFetch(domain:docs.cloud.google.com)
WebFetch(domain:forum.plantuml.net)
WebFetch(domain:github.com)
WebFetch(domain:plantuml.com)
WebFetch(domain:ubuntu.com)
WebFetch(domain:www.anthropic.com)
WebSearch
```

### Alpha multi-line heredoc junk (dropped) — target files only
```
/tmp/breakthrough.txt
/tmp/bug_analysis.txt
/tmp/file_collision.txt
/tmp/FINAL_ANALYSIS_REPORT.txt
/tmp/final_analysis.txt
/tmp/rbgu_create_1_u_code.txt
/tmp/rbgu_create_u_code.txt
/tmp/rbgu_key_1_u_code.txt
/tmp/rbgu_key_u_code.txt
/tmp/rbgu_list_keys_1_u_code.txt
/tmp/rbgu_list_keys_u_code.txt
/tmp/rbgu_verify-0s_1_u_code.txt
/tmp/rbgu_verify-0s_2_u_code.txt
/tmp/rbgu_verify-0s_u_code.txt
/tmp/REAL_BUG.txt
/tmp/scenario_trace.txt
/tmp/SMOKING_GUN.txt
```

### Appendix A — alpha full Bash list (sorted unique)
```
./tt/apcw-t.Test.sh:*
./tt/buw-d.Delay.sh
./tt/buw-dly.Delay.sh
./tt/buw-h0.HandbookTOP.sh:*
./tt/buw-hj0.HandbookJurisdictionTop.sh *
./tt/buw-hw.HandbookWindows.sh:*
./tt/buw-HWab.AccessBase.sh:*
./tt/buw-HWar.AccessRemote.sh
./tt/buw-HWax.AccessEntrypoints.sh
./tt/buw-HWec.EnvironmentCygwin.sh:*
./tt/buw-HWew.EnvironmentWSL.sh rbtww-main:*
./tt/buw-jpCW.CaparisonWindows.sh bujn-winpc *
./tt/buw-jpF.Fenestrate.sh *
./tt/buw-jpGw.GarrisonWsl.sh bujn-winpc *
./tt/buw-jpIW.InvigilateWindows.sh --help
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc *
./tt/buw-jpW.WslInstall.sh *
./tt/buw-jwk.WorkloadKnock.sh bujn-winpc *
./tt/buw-qsc.QualifyShellCheck.sh 2>&1
./tt/buw-rcr.RenderConfigRegime.sh *
./tt/buw-rer.RenderEnvironmentRegime.sh *
./tt/buw-rnr.RenderNodeRegime.sh *
./tt/buw-rnv.ValidateNodeRegime.sh jjfu-full:*
./tt/buw-rpr.RenderPrivilegeRegime.sh *
./tt/buw-rpv.ValidatePrivilegeRegime.sh bujn-winpc *
./tt/buw-rsr.RenderStationRegime.sh *
./tt/buw-SI.StationInit.sh 2>&1
./tt/buw-st.BukSelfTest.sh
./tt/buw-st.BukSelfTest.sh 2>&1
./tt/buw-ta.TestAll.sh
./tt/buw-ts.TestSweep.sh 2>&1 | tail -40
./tt/buw-tt-ll.ListLaunchers.sh *
./tt/buw-xd.Delay.sh
./tt/jjw-tfP2.ProvisionPhase2.localhost.sh 2>&1
./tt/jjw-tfs.TestFundusScenario.cerebro.sh 2>&1
./tt/jjw-tfs.TestFundusScenario.localhost.sh 2>&1
./tt/jjw-tfS.TestFundusSingle.localhost.sh relay_check_instant 2>&1
./tt/jjw-tfS.TestFundusSingle.localhost.sh relay_concurrent_overlap 2>&1
./tt/rbi-iB.IfritBuild.sh 2>&1
./tt/rbtd-b.Build.sh
./tt/rbtd-b.Build.sh 2>&1
./tt/rbtd-r.FixtureRun.access-probe.sh *
./tt/rbtd-r.FixtureRun.canonical-establish.sh
./tt/rbtd-r.FixtureRun.handbook-render.sh *
./tt/rbtd-r.FixtureRun.pluml.sh *
./tt/rbtd-r.FixtureRun.srjcl.sh *
./tt/rbtd-r.FixtureRun.tadmor-security.sh *
./tt/rbtd-r.FixtureRun.tadmor.sh *
./tt/rbtd-r.Run.access-probe.sh 2>&1
./tt/rbtd-r.Run.enrollment-validation.sh 2>&1
./tt/rbtd-r.Run.four-mode.sh
./tt/rbtd-r.Run.four-mode.sh 2>&1
./tt/rbtd-r.Run.pluml.sh 2>&1
./tt/rbtd-r.Run.regime-smoke.sh
./tt/rbtd-r.Run.regime-smoke.sh 2>&1
./tt/rbtd-r.Run.regime-validation.sh
./tt/rbtd-r.Run.regime-validation.sh 2>&1
./tt/rbtd-r.Run.srjcl.sh 2>&1
./tt/rbtd-r.Run.tadmor.sh
./tt/rbtd-s.FixtureCase.sh
./tt/rbtd-s.FixtureCase.sh tadmor *
./tt/rbtd-s.SingleCase.enrollment-validation.sh 2>&1
./tt/rbtd-s.SingleCase.four-mode.sh
./tt/rbtd-s.SingleCase.regime-smoke.sh rs-burc:*
./tt/rbtd-s.SingleCase.tadmor.sh 2>&1
./tt/rbtd-s.SingleCase.tadmor.sh coordinated-arp-gratuitous:*
./tt/rbtd-s.SingleCase.tadmor.sh pentacle-dnsmasq-responds:*
./tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_coordinated_dns_cache_integrity
./tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_coordinated_dnsmasq_query_audit
./tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_coordinated_mac_flood_resilience
./tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_coordinated_sentry_egress_lockdown
./tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_coordinated_sentry_integrity 2>&1
./tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_sortie_sentry_udp_non_dns
./tt/rbtd-s.TestSuite.crucible.sh
./tt/rbtd-s.TestSuite.crucible.sh 2>&1
./tt/rbtd-s.TestSuite.fast.sh
./tt/rbtd-s.TestSuite.service.sh
./tt/rbtd-t.Test.sh
./tt/rbtd-t.Test.sh 2>&1
./tt/rbw-acd.CheckDirectorCredential.sh
./tt/rbw-acg.CheckGovernorCredential.sh
./tt/rbw-acr.CheckRetrieverCredential.sh
./tt/rbw-adD.GovernorDivestsDirector.sh t3d *
./tt/rbw-adI.GovernorInvestsDirector.sh canest-dir *
./tt/rbw-adI.GovernorInvestsDirector.sh t3d *
./tt/rbw-adr.GovernorRostersDirectors.sh *
./tt/rbw-ak.ArkKludge.sh
./tt/rbw-ak.ArkKludge.sh rbev-busybox:*
./tt/rbw-ak.ArkKludge.sh rbev-sentry-debian-slim:*
./tt/rbw-aM.PayorMantlesGovernor.sh *
./tt/rbw-arI.GovernorInvestsRetriever.sh t3r *
./tt/rbw-arr.GovernorRostersRetrievers.sh *
./tt/rbw-ca.CrucibleActive.sh tadmor:*
./tt/rbw-cb.Bark.tadmor.sh rbid:*
./tt/rbw-cC.Charge.ccyolo.sh
./tt/rbw-cC.Charge.srjcl.sh *
./tt/rbw-cC.Charge.tadmor.sh *
./tt/rbw-cC.Charge.tadmor.sh 2>&1
./tt/rbw-ch.Hail.sh
./tt/rbw-cic.CrucibleIsCharged.sh
./tt/rbw-cic.CrucibleIsCharged.sh srjcl *
./tt/rbw-cic.CrucibleIsCharged.sh tadmor:*
./tt/rbw-cic.CrucibleIsCharged.tadmor.sh
./tt/rbw-cK.Kludge.tadmor.sh 2>&1
./tt/rbw-cKB.KludgeBottle.sh ccyolo:*
./tt/rbw-cKB.KludgeBottle.sh tadmor:*
./tt/rbw-cKB.KludgeBottle.tadmor.sh 2>&1
./tt/rbw-cKS.KludgeSentry.sh pluml *
./tt/rbw-cKS.KludgeSentry.sh srjcl *
./tt/rbw-cKS.KludgeSentry.sh tadmor *
./tt/rbw-cKS.KludgeSentry.tadmor.sh *
./tt/rbw-cQ.Quench.ccyolo.sh
./tt/rbw-cQ.Quench.srjcl.sh
./tt/rbw-cQ.Quench.tadmor.sh *
./tt/rbw-cQ.Quench.tadmor.sh 2>&1
./tt/rbw-cw.Writ.tadmor.sh cat:*
./tt/rbw-cw.Writ.tadmor.sh ls:*
./tt/rbw-cw.Writ.tadmor.sh ps:*
./tt/rbw-cw.Writ.tadmor.sh sh:*
./tt/rbw-DA.DirectorAbjuresArk.sh:*
./tt/rbw-Dc.DirectorChecksConsecrations.sh:*
./tt/rbw-DC.DirectorConjuresArk.sh rbev-vessels/rbev-bottle-anthropic-jupyter 2>&1
./tt/rbw-DC.DirectorConjuresArk.sh rbev-vessels/rbev-bottle-plantuml 2>&1
./tt/rbw-DC.DirectorCreatesArk.sh 2>&1 | head -30
./tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-bottle-plantuml 2>&1
./tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-busybox 2>&1
./tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-busybox-graft 2>&1
./tt/rbw-DC.DirectorCreatesConsecration.sh rbev-bottle-ifrit:*
./tt/rbw-DC.DirectorCreatesConsecration.sh rbev-sentry-debian-slim:*
./tt/rbw-dE.DirectorEnshrinesVessel.sh rbev-bottle-ifrit-forge *
./tt/rbw-dI.DirectorInscribesReliquary.sh *
./tt/rbw-dL.PayorLeviesDepot.sh *
./tt/rbw-dl.PayorListsDepots.sh *
./tt/rbw-DP.DirectorRefreshesPins.sh:*
./tt/rbw-DPG.DirectorRefreshesGcbPins.sh 2>&1
./tt/rbw-DS.DirectorSummonsArk.sh rbev-bottle-anthropic-jupyter:*
./tt/rbw-DS.DirectorSummonsArk.sh rbev-bottle-plantuml:*
./tt/rbw-DS.DirectorSummonsArk.sh rbev-bottle-ubuntu-test:*
./tt/rbw-DS.DirectorSummonsArk.sh rbev-busybox:*
./tt/rbw-DS.DirectorSummonsArk.sh rbev-sentry-ubuntu-large:*
./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhm100002 *
./tt/rbw-DV.DirectorVouchesArk.sh:*
./tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh r260513125123 *
./tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh r260609093011 *
./tt/rbw-fA.DirectorAbjuresHallmark.sh rbev-busybox:*
./tt/rbw-fk.LocalKludge.sh rbev-sentry-deb-tether *
./tt/rbw-fk.LocalKludge.sh rbev-sentry-debian-slim:*
./tt/rbw-fO.DirectorOrdainsHallmark.sh:*
./tt/rbw-fpc.RetrieverPlumbsCompact.sh
./tt/rbw-fs.RetrieverSummonsHallmark.sh:*
./tt/rbw-ft.RetrieverTalliesHallmarks.sh:*
./tt/rbw-gO.Onboarding.sh
./tt/rbw-gO.Onboarding.sh tadmor:*
./tt/rbw-go.OnboardMAIN.sh
./tt/rbw-go.OnboardMAIN.sh 2>&1
./tt/rbw-gOD.OnboardDirector.sh
./tt/rbw-gOG.OnboardGovernor.sh
./tt/rbw-gOP.OnboardPayor.sh
./tt/rbw-gOr.OnboardReference.sh 2>&1
./tt/rbw-gOR.OnboardRetriever.sh 2>&1
./tt/rbw-gPE.PayorEstablish.sh
./tt/rbw-h0.HandbookTOP.sh:*
./tt/rbw-hO.DirectorOrdainsHallmark.sh
./tt/rbw-hO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-bottle-ifrit
./tt/rbw-hs.RetrieverSummonsHallmark.sh rbev-bottle-ifrit:*
./tt/rbw-hw.HandbookWindows.sh
./tt/rbw-HWdc.DockerContextDiscipline.sh
./tt/rbw-HWdd.DockerDesktop.sh
./tt/rbw-HWdw.DockerWSLNative.sh rbtww-main:*
./tt/rbw-iae.DirectorAuditsEnshrinements.sh *
./tt/rbw-iah.DirectorAuditsHallmarks.sh *
./tt/rbw-iar.DirectorAuditsReliquaries.sh *
./tt/rbw-iJr.DirectorJettisonsReliquaryImage.sh
./tt/rbw-ir.DirectorRekonsImages.sh rbev-bottle-ifrit:*
./tt/rbw-ir.DirectorRekonsImages.sh rbev-busybox:*
./tt/rbw-irr.DirectorRekonsReliquary.sh r260513125123 *
./tt/rbw-irr.DirectorRekonsReliquary.sh r260605074843 *
./tt/rbw-irr.DirectorRekonsReliquary.sh r260609093011 *
./tt/rbw-Is.IfritSortie.tadmor.sh 2>&1
./tt/rbw-ld.DirectorDivinesLodes.sh *
./tt/rbw-LK.LocalKludge.sh
./tt/rbw-MG.MarshalGenerate.sh
./tt/rbw-MG.MarshalGenerate.sh 2>&1
./tt/rbw-MZ.MarshalZeroes.sh *
./tt/rbw-ni.NameplateInfo.sh
./tt/rbw-nv.ValidateNameplates.sh
./tt/rbw-o.ONBOARDING.sh
./tt/rbw-o.ONBOARDING.sh --help
./tt/rbw-o.OnboardingStartHere.sh
./tt/rbw-o.OnboardingStartHere.sh --help
./tt/rbw-Occ.OnboardingConfigureEnvironment.sh
./tt/rbw-Occ.OnboardingCrashCourse.sh:*
./tt/rbw-Ocd.OnboardingCredentialDirector.sh:*
./tt/rbw-Ocr.OnboardingCredentialRetriever.sh:*
./tt/rbw-Odf.OnboardingDirectorFirstBuild.sh
./tt/rbw-Odf.OnboardingDirectorFirstBuild.sh --help
./tt/rbw-Ofc.OnboardingFirstCrucible.sh:*
./tt/rbw-Og.OnboardingGovernor.sh:*
./tt/rbw-Op.OnboardingPayor.sh
./tt/rbw-Ots.OnboardingTadmorSecurity.sh
./tt/rbw-qa.QualifyAll.sh 2>&1 | tail -40
./tt/rbw-Qf.QualifyFast.sh 2>&1
./tt/rbw-Qf.QualifyFast.sh 2>&1 | tail -30
./tt/rbw-ral.ListAuthRegimes.sh
./tt/rbw-rdr.RenderDepotRegime.sh
./tt/rbw-rdv.ValidateDepotRegime.sh
./tt/rbw-rgr.RenderPinsRegime.sh:*
./tt/rbw-rnl.ListNameplateRegime.sh
./tt/rbw-rnr.RenderNameplateRegime.sh
./tt/rbw-rnr.RenderNameplateRegime.sh tadmor:*
./tt/rbw-rnr.RenderNameplateRegime.tadmor.sh
./tt/rbw-rnv.ValidateNameplateRegime.sh ccyolo:*
./tt/rbw-rnv.ValidateNameplateRegime.sh moriah *
./tt/rbw-rnv.ValidateNameplateRegime.sh pluml:*
./tt/rbw-rnv.ValidateNameplateRegime.sh srjcl *
./tt/rbw-rnv.ValidateNameplateRegime.sh tadmor *
./tt/rbw-rov.ValidateOauthRegime.sh *
./tt/rbw-rpv.ValidatePayorRegime.sh *
./tt/rbw-rrr.RenderRepoRegime.sh:*
./tt/rbw-rrv.ValidateRepoRegime.sh
./tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-bottle-ifrit:*
./tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-sentry-debian-slim:*
./tt/rbw-rv.RegimeValidate.sh
./tt/rbw-rvl.ListVesselRegime.sh:*
./tt/rbw-rvr.RenderVesselRegime.sh
./tt/rbw-rvr.RenderVesselRegime.sh rbev-bottle-ccyolo:*
./tt/rbw-rvr.RenderVesselRegime.sh rbev-bottle-ifrit:*
./tt/rbw-rvr.RenderVesselRegime.sh rbev-sentry-debian-slim:*
./tt/rbw-rvv.ValidateVesselRegime.sh
./tt/rbw-rvv.ValidateVesselRegime.sh rbev-bottle-ccyolo:*
./tt/rbw-rvv.ValidateVesselRegime.sh rbev-bottle-ifrit:*
./tt/rbw-s.Start.tadmor.sh 2>&1
./tt/rbw-tb.Build.sh *
./tt/rbw-tc.FixtureCase.sh *
./tt/rbw-tf.FixtureRun.sh admission-proof *
./tt/rbw-tf.FixtureRun.sh canonical-invest *
./tt/rbw-tf.FixtureRun.sh cupel *
./tt/rbw-tf.FixtureRun.sh dogfight *
./tt/rbw-tf.FixtureRun.sh foundry-path *
./tt/rbw-tf.FixtureRun.sh lode-lifecycle *
./tt/rbw-tf.FixtureRun.sh onboarding-sequence *
./tt/rbw-tf.FixtureRun.sh recipe-validation *
./tt/rbw-tf.FixtureRun.sh regime-validation *
./tt/rbw-tf.FixtureRun.sh reliquary-lifecycle *
./tt/rbw-tf.FixtureRun.sh terrier-atomicity *
./tt/rbw-tf.FixtureRun.sh wsl-lifecycle *
./tt/rbw-tf.QualifyFast.sh
./tt/rbw-tf.QualifyFast.sh 2>&1
./tt/rbw-tf.TestFixture.ark-lifecycle.sh
./tt/rbw-tf.TestFixture.enrollment-validation.sh
./tt/rbw-tf.TestFixture.kick-tires.sh
./tt/rbw-tf.TestFixture.kick-tires.sh 2>&1 | tail -5
./tt/rbw-tf.TestFixture.pluml-diagram.sh 2>&1
./tt/rbw-tf.TestFixture.qualify-all.sh
./tt/rbw-tf.TestFixture.regime-smoke.sh
./tt/rbw-tf.TestFixture.regime-validation.sh
./tt/rbw-tf.TestFixture.slsa-provenance.sh
./tt/rbw-tf.TestFixture.srjcl-jupyter.sh 2>&1
./tt/rbw-tf.TestFixture.tadmor-security.sh 2>&1
./tt/rbw-tf.TestFixture.three-mode.sh 2>&1
./tt/rbw-tK.KludgeCycle.tadmor.sh 2>&1
./tt/rbw-tl.Shellcheck.sh *
./tt/rbw-tO.OrdainCycle.tadmor.sh
./tt/rbw-to.TestOne.sh 2>&1 | tail -15
./tt/rbw-tP.QualifyPristine.sh *
./tt/rbw-tq.QualifyFast.sh *
./tt/rbw-tS.QualifySkirmish.sh *
./tt/rbw-ts.TestSuite.complete.sh
./tt/rbw-ts.TestSuite.crucible.sh *
./tt/rbw-ts.TestSuite.dogfight.sh *
./tt/rbw-ts.TestSuite.fast.sh *
./tt/rbw-ts.TestSuite.fast.sh 2>&1
./tt/rbw-ts.TestSuite.fast.sh 2>&1 | tail -10
./tt/rbw-ts.TestSuite.fast.sh 2>&1 | tail -15
./tt/rbw-ts.TestSuite.fast.sh 2>&1 | tail -20
./tt/rbw-ts.TestSuite.fast.sh 2>&1 | tail -25
./tt/rbw-ts.TestSuite.fast.sh 2>&1 | tail -40
./tt/rbw-ts.TestSuite.fast.sh 2>&1 | tail -5
./tt/rbw-ts.TestSuite.service.sh
./tt/rbw-ts.TestSuite.service.sh 2>&1
./tt/rbw-ts.TestSuite.sh ark-lifecycle:*
./tt/rbw-ts.TestSuite.sh nsproto-security:*
./tt/rbw-ts.TestSuite.sh pluml-diagram:*
./tt/rbw-ts.TestSuite.sh srjcl-jupyter:*
./tt/rbw-ts.TestSuite.skirmish.sh *
./tt/rbw-ts.TestSuite.tadmor.sh *
./tt/rbw-tt.Test.sh *
./tt/rbw-tw.TestSweep.sh 2>&1
./tt/rbw-tw.TestSweep.sh complete:*
./tt/rbw-tw.TestSweep.sh fast:*
./tt/rbw-tw.TestSweep.sh service:*
./tt/rbw-z.Stop.tadmor.sh 2>&1
./tt/vow-b.Build.sh:*
./tt/vow-F.Freshen.sh
./tt/vow-R.ParcelRelease.sh
./tt/vow-t.Test.sh:*
./tt/vvw-r.RunVVX.sh --help
./tt/vvw-r.RunVVX.sh --version 2>&1
./tt/vvw-r.RunVVX.sh eval:*
./tt/vvw-r.RunVVX.sh jjx:*
./tt/vvw-r.RunVVX.sh schema:*
./tt/vvw-r.RunVVX.sh version:*
" /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/vov_veiled/BCG-BashConsoleGuide.md | head -40
[ -d "$PB" ]
[[ -f "Tools/buk/vov_veiled/$f" ]]
/bin/bash --version
/bin/bash -c *
/context | head -100
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -dump | grep -B2 "/Applications/SlickEditPro2025" 2>&1
/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -dump | grep -i slick 2>&1
/tmp/analyze_hardcoding.sh
/tmp/analyze_rbs0.sh
/tmp/count_tests.sh
/tmp/p1014/vvi_install.sh /Users/bhyslop/projects/pb_paneboard02/.buk/burc.env
/tmp/p1015full/vvi_install.sh /Users/bhyslop/projects/djo-DanielsJupyterObsidian/.buk/burc.env
/tmp/rbrm_verification.txt:*
/tmp/sc2153_backstop_test.sh
/tmp/sibling_analysis.sh
/tmp/vvk1013_install/vvi_install.sh /Users/bhyslop/projects/pb_paneboard02/.buk/burc.env
/Users/bhyslop/models/stanford-deidentifier/.venv/bin/optimum-cli --help
/Users/bhyslop/models/stanford-deidentifier/.venv/bin/optimum-cli export *
/Users/bhyslop/models/stanford-deidentifier/.venv/bin/python -c "import json; d=json.load\(open\('/Users/bhyslop/models/stanford-deidentifier/config.json'\)\); print\(json.dumps\(d.get\('id2label'\), indent=2\)\)"
/Users/bhyslop/models/stanford-deidentifier/.venv/bin/python -m pip install onnxruntime
/Users/bhyslop/models/stanford-deidentifier/.venv/bin/python -m pip install optimum-onnx
/Users/bhyslop/projects/pb_paneboard02/tt/pbw-t.ProofOfConceptTimed.10.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid/target/debug/rbid
/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid/target/debug/rbid --list 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid/target/debug/rbid bogus-attack:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid/target/debug/rbid dns-allowed-anthropic:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvk/bin/vvr-darwin-arm64 --help 2>&1 | head -30
/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvk/bin/vvx --help
/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvk/bin/vvx --help 2>&1 | head -20
/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvk/bin/vvx --help 2>&1 | head -30
/Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvk/bin/vvx-darwin-arm64 --help 2>&1 | head -40
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/apcw-ba.BatchAssay.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/apcw-r.Run.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/apcw-t.Test.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-hj0.HandbookJurisdictionTop.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-hjl.HandbookJurisdictionLinux.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-hjm.HandbookJurisdictionMacos.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-hw.HandbookWindows.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWab.AccessBase.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWar.AccessRemote.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWax.AccessEntrypoints.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWec.EnvironmentCygwin.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-HWew.EnvironmentWSL.sh rbtww-main:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpCW.CaparisonWindows.sh bujn-winpc *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpF.Fenestrate.sh nonesuch-investiture *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpGb.GarrisonBash.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpGb.GarrisonBash.sh nonesuch-investiture *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpGw.GarrisonWsl.sh bujn-winpc *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpS bujn-winpc *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpS.PrivilegedSsh.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jpS.PrivilegedSsh.sh bujn-winpc *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jwk.WorkloadKnock.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-jws.WorkloadInteractiveSession.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-qsc.QualifyShellCheck.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-qsc.QualifyShellCheck.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rer.RenderEnvironmentRegime.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rev.ValidateEnvironmentRegime.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcc.ConstructCygwin.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcl.ConstructLinux.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcp.ConstructPowerShell.sh 192.168.86.27 bhyslop winhost
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcw.ConstructWSL.sh 192.168.86.27 bhyslop winhost
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rhcx.ConstructLocalhost.sh bhyslop:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnl.ListNodeRegime.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnl.NodeRegimeList.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnr.RenderNodeRegime.sh winhost-cyg:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnv.ValidateNodeRegime.sh bujn-winpc *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnv.ValidateNodeRegime.sh devbox-linux:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnv.ValidateNodeRegime.sh winhost-cyg:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnv.ValidateNodeRegime.sh winhost-ps:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnv.ValidateNodeRegime.sh winhost-wsl:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rnva.ValidateAllNodeRegimes.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rpl.ListPrivilegeRegime.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rpl.PrivilegeRegimeList.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rsv.ValidateStationRegime.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-st.BukSelfTest.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-st.BukSelfTest.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbi-iB.IfritBuild.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbi-iK.IfritKludge.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-b.Build.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-b.Build.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.dockerfile-hygiene.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.enrollment-validation.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.handbook-render.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.regime-smoke.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.FixtureRun.regime-validation.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.access-probe.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.four-mode.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.pristine-lifecycle.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.regime-validation.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.tadmor-security.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.tadmor.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-r.Run.tadmor.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.FixtureCase.canonical-onboarding-sequence.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.FixtureCase.sh regime-validation *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.SingleCase.pristine-lifecycle.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.SingleCase.pristine-lifecycle.sh rbtdrp_marshal_zero_attestation
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.SingleCase.tadmor.sh rbtdrc_sortie_http_end_to_end:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.TestSuite.crucible.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.TestSuite.crucible.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.TestSuite.fast.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-s.TestSuite.service.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-t.Test.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbtd-t.Test.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cC.Charge.ccyolo.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cC.Charge.tadmor.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cC.Charge.tadmor.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cic.CrucibleIsCharged.sh tadmor:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cKB.KludgeBottle.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cKB.KludgeBottle.sh ccyolo:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cKB.KludgeBottle.sh tadmor:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cKS.KludgeSentry.sh tadmor *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cQ.Quench.ccyolo.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-cQ.Quench.tadmor.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-da.DepotAttribution.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DB.DirectorBeseechesArk.sh rbev-sentry-ubuntu-large:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Dc.CheckConsecrations.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Dc.DirectorChecksConsecrations.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorConjuresArk.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesArk.rbev-busybox.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesArk.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesArk.sh rbev-busybox:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-bottle-plantuml 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-busybox
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh rbev-bottle-ifrit:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh rbev-sentry-debian-slim:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh rbev-sentry-ubuntu-large:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh rbev-vessels/rbev-bottle-ubuntu-test 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh rbev-vessels/rbev-busybox 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorCreatesConsecration.sh rbev-vessels/rbev-sentry-ubuntu-large 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-sentry-debian-slim:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-bottle-ifrit
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-bottle-ubuntu-test 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-busybox 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-sentry-ubuntu-large 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-di.DepotInfo.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DI.DirectorInscribesRubric.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dl.PayorListsDepots.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DO.DirectorOrdainsConsecration.sh rbev-vessels/rbev-bottle-anthropic-jupyter 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DO.DirectorOrdainsConsecration.sh rbev-vessels/rbev-bottle-ifrit 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DO.DirectorOrdainsConsecration.sh rbev-vessels/rbev-bottle-plantuml 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DO.DirectorOrdainsConsecration.sh rbev-vessels/rbev-busybox 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DO.DirectorOrdainsConsecration.sh rbev-vessels/rbev-sentry-debian-slim 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DP.DirectorRefreshesPins.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DPG.DirectorRefreshesGcbPins.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Dt.DirectorTalliesConsecrations.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DV.DirectorVouchesConsecrations.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DV.DirectorVouchesConsecrations.sh rbev-vessels/rbev-busybox 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh r260610145233 *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh r260610202716 *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fA.DirectorAbjuresHallmark.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fA.DirectorAbjuresHallmark.sh rbev-busybox:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fhc.HygieneCheck.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fhv.HygieneCheckVessel.sh rbev-bottle-ccyolo *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fhv.HygieneCheckVessel.sh rbev-graft-demo *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-busybox:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fpc.RetrieverPlumbsCompact.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fs.RetrieverSummonsHallmark.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ft.RetrieverTalliesHallmarks.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-GD.GovernorCreatesDirector.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-GD.GovernorCreatesDirector.sh director1:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gO.Onboarding.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-go.OnboardMAIN.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOD.OnboardDirector.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOG.OnboardGovernor.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOP.OnboardPayor.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOr.OnboardReference.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gOR.OnboardRetriever.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gPE.PayorEstablish.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gPo.PayorOnboarding.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gPR.PayorRefresh.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gq.QuotaBuild.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-GR.GovernorCreatesRetriever.sh retriever1:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-GS.GovernorDeletesServiceAccount.sh director-director1@rbwg-d-depot10040-260323094315.iam.gserviceaccount.com 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-h0.HandbookTOP.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hO.DirectorOrdainsHallmark.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hO.DirectorOrdainsHallmark.sh rbev-bottle-ifrit:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hO.DirectorOrdainsHallmark.sh rbev-sentry-debian-slim:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hs.RetrieverSummonsHallmark.sh rbev-bottle-ifrit:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hs.RetrieverSummonsHallmark.sh rbev-sentry-debian-slim:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-hw.HandbookWindows.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-HWdc.DockerContextDiscipline.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-HWdd.DockerDesktop.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-HWdw.DockerWSLNative.sh rbtww-main:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-HWha.HostAvailability.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ir.DirectorRekonsImages.sh rbev-busybox:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-la.DirectorAugursLode.sh vn260610115913 *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-lC.DirectorConclavesReliquary.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-lI.DirectorImmuresPodvm.sh podvm-native *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-lp.DirectorPresagesImmure.sh podvm-native *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-MD.MarshalDuplicate.sh /Users/bhyslop/test-marshal 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-MG.MarshalGenerate.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-MG.MarshalGenerate.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-MR.MarshalReset.sh 2>&1 || true
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-nv.ValidateNameplates.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-o.ONBOARDING.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-o.OnboardingStartHere.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Occ.OnboardingConfigureEnvironment.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Occ.OnboardingCrashCourse.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Ocd.OnboardingCredentialDirector.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Ocr.OnboardingCredentialRetriever.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Oda.OnboardingDirectorAirgap.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Odb.OnboardingDirectorBind.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Odf.OnboardingDirectorFirstBuild.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Ofc.OnboardingFirstCrucible.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Og.OnboardingGovernor.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Op.OnboardingPayor.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PC.PayorCreatesDepot.sh depot10040:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PD.PayorDestroysDepot.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PD.PayorDestroysDepot.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PD.PayorDestroysDepot.sh rbwg-d-depot10030-260312153057:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PG.PayorResetsGovernor.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Pl.PayorListsDepots.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PO.PayorOnboarding.sh 2>&1 || true
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-qa.QualifyAll.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Qf.QualifyFast.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Qf.QualifyFast.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rar.RenderAuthRegime.sh retriever:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rav.ValidateAuthRegime.sh retriever:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Ric.RetrieverInspectsCompact.sh rbev-busybox:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-RiF.RetrieverInspectsFull.sh rbev-busybox:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rnl.ListNameplateRegime.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rnr.RenderNameplateRegime.sh tadmor:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rnv.ValidateNameplateRegime.sh ccyolo *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rnv.ValidateNameplateRegime.sh tadmor *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rpv.ValidatePayorRegime.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rrv.ValidateRepoRegime.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rrv.ValidateRepoRegime.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsArk.sh rbev-sentry-ubuntu-large:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsArk.sh rbev-vessels/rbev-busybox i20260312_173753-b20260313_180332 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsArk.sh rbev-vessels/rbev-busybox i20260313_122446-b20260313_210508 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsArk.sh rbev-vessels/rbev-busybox i20260313_142921-b20260313_213400 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh nsproto:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-bottle-anthropic-jupyter:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-bottle-ifrit:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-bottle-plantuml:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-bottle-ubuntu-test:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-sentry-debian-slim:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rs.RetrieverSummonsConsecration.sh rbev-sentry-ubuntu-large:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rv.RetrieverVouchesArk.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvl.ListVesselRegime.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvy.DirectorYokesReliquaryAllVessels.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rw.RetrieverWrestsImage.sh "rbev-busybox:c260401072219-r260401142450-image" 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-s.Start.nsproto.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-s.Start.srjcl.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-s.Start.tadmor.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-s.Start.tadmor.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tb.Build.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tc.FixtureCase.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh conformance *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh enrollment-validation *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh lode-lifecycle *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh podvm-lifecycle *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh regime-poison *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh regime-validation *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh terrier-scaffold *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh wsl-lifecycle *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.QualifyFast.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.QualifyFast.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.access-probe.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.ark-lifecycle.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.enrollment-validation.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.four-mode.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.kick-tires.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.nsproto-security.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.nsproto-security.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.pluml-diagram.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.qualify-all.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.regime-credentials.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.regime-smoke.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.regime-validation.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.regime-validation.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.srjcl-jupyter.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.tadmor-security.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.three-mode.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tl.Shellcheck.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-to.TestOne.sh rbtcsl_provenance_tcase:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tq.QualifyFast.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tS.QualifySkirmish.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.complete.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.crucible.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.dogfight.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.fast.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.fast.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.service.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.sh ark-lifecycle:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.skirmish.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSweep.fast.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tT.QualifyTadmor.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tt.Test.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tw.TestSweep.sh fast:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tw.TestSweep.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-z.Stop.nsproto.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-z.Stop.nsproto.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-z.Stop.srjcl.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-z.Stop.tadmor.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-z.Stop.tadmor.sh 2>&1
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/vow-b.Build.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/vow-t.Test.sh:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/vvw-r.RunVVX.sh jjx_record:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/vvw-r.RunVVX.sh jjx:*
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/vvw-r.RunVVX.sh:*
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbtd-r.Run.four-mode.sh
\\\\
2
ark
arp *
asciidoctor --safe-mode=safe --failure-level=WARN -o /dev/null Tools/jjk/vov_veiled/JJS0_JobJockeySpec.adoc
asciidoctor --version
awk -F'_' '{print substr\($1, 1, 4\), substr\($1, 4\)}'
awk '{ *
awk '{prefix=substr\($0,1,5\); if\(prefix!="apcsc" && prefix!="apcsd" && prefix!="apcsg" && prefix!="apcsn" && prefix!="apcsu"\) print "UNEXPECTED: "$0}'
awk '{print $1, substr\($0, index\($0, " "\)+1, 200\)}'
awk '{printf "%3d %s\\n", length\($0\), $0}'
awk '/\\[\\[apcs_[a-z_]+\\]\\]/' Tools/apck/APCS0-SpecTop.adoc
awk '/^:apcs_/ {print}' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/apck/APCS0-SpecTop.adoc
awk '/^# Probe per-installation depot states/,/^rbgp_payor_oauth_refresh\\\(\\\)/' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
awk '/^In .*line [0-9]+:/{ split\($0,a,"line "\); ln=a[2]+0; if \(\(ln>=159 && ln<=273\) || \(ln>=1058 && ln<=1120\)\) {p=1; print "----"; print} else p=0; next} p{print}'
awk '/^rbgp_depot_levy\\\(\\\)/,/^rbgp_depot_unmake\\\(\\\)/' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
awk '/^rbgp_depot_list\\\(\\\)/,/^}/' Tools/rbk/rbgp_Payor.sh
awk '/^rbgp_depot_list\\\(\\\)/,/^rbgp_payor_oauth_refresh\\\(\\\)/' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
awk '/^rbgp_depot_unmake\\\(\\\)/,/^rbgp_depot_list\\\(\\\)/' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
awk '/^zrbgp_depot_state_emit\\\(\\\)/,/^# Post-lifecycle hook/' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
awk '/Director Subtracks/,/^$/'
awk '/Kludged/,/They diverge/'
awk '/tag::mapping-section/,/end::mapping-section/' Tools/apck/APCS0-SpecTop.adoc
awk '$1+0 >= 1250'
awk 'NR<=1500 && /^[a-zA-Z_]+\\\(\\\)|^rbfd_|^zrbfd_/ {print NR":"$0}' Tools/rbk/rbfd_FoundryDirectorBuild.sh
awk 'NR>=158' /tmp/rbtds-service-260425-1004.log
awk 'NR>=200' /tmp/rbtds-service-260425-1004.log
awk *
awk NR>=120 && NR<=200 *
awk NR>=560 && NR<=750 *
awk NR>=750 && NR<=950 *
bash:*
break
brew install *
brew list:*
BUC_VERBOSE=0 ./tt/buw-jpF.Fenestrate.sh
build
BURD_LAUNCHER=".buk/launcher.buw_workbench.sh" BURD_NO_LOG=1 ./.buk/launcher.buw_workbench.sh "buw-jwc"
BURD_LAUNCHER=".buk/launcher.buw_workbench.sh" BURD_NO_LOG=1 ./.buk/launcher.buw_workbench.sh "buw-jwk"
BURD_LAUNCHER=".buk/launcher.rbw_workbench.sh" .buk/launcher.rbw_workbench.sh rbw-dY.DirectorYokesReliquaryInVessel.sh --help
BURD_NO_HYPERLINKS=1 tt/rbw-Occ.OnboardingCrashCourse.sh:*
BURE_CONFIRM=skip ./tt/rbw-cC.Charge.pluml.sh
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100000 *
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100001 *
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100002 *
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100003 *
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100005 *
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100006 *
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhl100007 *
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhl-d-canest2bhm100002 *
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhm-d-canest2bhm100000 *
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhm-d-canest2bhm100001 *
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhm-d-canest2bhm100002 *
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh cancbhm-d-canest3bhm100000
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh prlc-d-pristl100005 *
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh prlcbhm-d-pristlbhm100000 *
BURE_CONFIRM=skip ./tt/rbw-fA.DirectorAbjuresHallmark.sh rbev-busybox:*
BURE_CONFIRM=skip ./tt/rbw-iJr.DirectorJettisonsReliquaryImage.sh *
BURE_CONFIRM=skip ./tt/rbw-MZ.MarshalZeroes.sh *
BURE_CONFIRM=skip ./tt/rbw-tf.FixtureRun.sh canonical-invest
BURE_CONFIRM=skip ./tt/rbw-tf.FixtureRun.sh lode-lifecycle *
BURE_CONFIRM=skip ./tt/rbw-tf.FixtureRun.sh pluml
BURE_CONFIRM=skip ./tt/rbw-ts.TestSuite.dogfight.sh *
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-adD.GovernorDivestsDirector.sh canest-dir
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-arD.GovernorDivestsRetriever.sh canest-ret
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-arr.GovernorRostersRetrievers.sh
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fA.DirectorAbjuresHallmark.sh rbev-busybox:*
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-lB.DirectorBanishesLode.sh vn260610115913
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PD.PayorDestroysDepot.sh rbwg-d-depot10030-260312153057 2>&1
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-PD.PayorDestroysDepot.sh rbwg-d-depot10040-260323094315
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.dogfight.sh
BURE_CONFIRM=skip BURE_COUNTDOWN=skip tt/rbw-dU.PayorUnmakesDepot.sh depot10041 *
BURE_CONFIRM=skip BURE_COUNTDOWN=skip tt/rbw-dU.PayorUnmakesDepot.sh rbwg-d-depot10041-260327170532 *
BURE_CONFIRM=skip tt/rbw-aM.PayorMantlesGovernor.sh
BURE_CONFIRM=skip tt/rbw-GD.GovernorCreatesDirector.sh mac1 2>&1
BURE_CONFIRM=skip tt/rbw-GR.GovernorCreatesRetriever.sh mac1 2>&1
BURE_CONFIRM=skip tt/rbw-MR.MarshalReset.sh
BURE_CONFIRM=skip tt/rbw-MZ.MarshalZeroes.sh *
BURE_CONFIRM=skip tt/rbw-PG.PayorResetsGovernor.sh 2>&1
BURE_CONFIRM=skip tt/rbw-tf.FixtureRun.sh lode-lifecycle *
BURE_CONFIRM=skip tt/rbw-ts.TestSuite.dogfight.sh *
BURE_CONFIRM=skip tt/rbw-ts.TestSuite.siege.sh *
BURE_COUNTDOWN=skip ./tt/rbw-DP.DirectorRefreshesPins.sh:*
BURE_COUNTDOWN=skip ./tt/rbw-DPB.DirectorRefreshesBinaryPins.sh:*
BURE_COUNTDOWN=skip ./tt/rbw-DPG.DirectorRefreshesGcbPins.sh:*
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/buw-rev.ValidateEnvironmentRegime.sh
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Dc.DirectorChecksConsecrations.sh
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DI.DirectorInscribesRubric.sh
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DPG.DirectorRefreshesGcbPins.sh
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rgv.ValidatePinsRegime.sh
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rrr.RenderRepoRegime.sh
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rrv.ValidateRepoRegime.sh
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvv.ValidateVesselRegime.sh rbev-bottle-plantuml
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvv.ValidateVesselRegime.sh rbev-busybox
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvv.ValidateVesselRegime.sh rbev-busybox-airgap-negative-canary
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvv.ValidateVesselRegime.sh rbev-vessels/rbev-busybox
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvv.ValidateVesselRegime.sh rbev-vessels/rbev-busybox-airgap-negative-canary
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-rvv.ValidateVesselRegime.sh rbev-vessels/rbev-sentry-ubuntu-large
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.ark-lifecycle.sh
BURE_COUNTDOWN=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.TestFixture.regime-validation.sh
BURE_COUNTDOWN=skip BURE_CONFIRM=skip ./tt/rbw-DC.DirectorCreatesConsecration.sh rbev-vessels/rbev-bottle-ifrit
BURE_COUNTDOWN=skip BURE_CONFIRM=skip ./tt/rbw-DC.DirectorCreatesConsecration.sh rbev-vessels/rbev-sentry-debian-slim
BURE_COUNTDOWN=skip BURE_CONFIRM=skip ./tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-bottle-ifrit
BURE_COUNTDOWN=skip BURE_CONFIRM=skip ./tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-sentry-debian-slim
BURE_COUNTDOWN=skip BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DC.DirectorConjuresArk.sh:*
BURE_COUNTDOWN=skip BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DI.DirectorInscribesRubric.sh:*
BURE_COUNTDOWN=skip BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-DP.DirectorRefreshesPins.sh:*
BURE_COUNTDOWN=skip BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-Rv.RetrieverVouchesArk.sh:*
BURE_COUNTDOWN=skip tt/rbw-DC.DirectorCreatesConsecration.sh rbev-vessels/rbev-busybox 2>&1
BURE_COUNTDOWN=skip tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-busybox 2>&1
BURE_COUNTDOWN=skip tt/rbw-DI.DirectorInscribesReliquary.sh 2>&1
BURE_TWEAK_NAME=buost_regime_poison BURE_TWEAK_VALUE="RBRF_WORKFORCE_POOL_ID=rbproof-aff-0617070756" BURE_CONFIRM=skip tt/rbw-mJ.PayorJiltsManor.sh
BURE_VERBOSE=1 tt/apcw-b.Build.sh
cargo --version
cargo +nightly fmt --version
cargo build:*
cargo check:*
cargo install:*
cargo run:*
cargo tauri:*
cargo test:*
cat "../temp-buk/temp-20260609-104733-83286-60/rbtd/rbtdrc_reliquary_lifecycle/05-banish.txt"
cat /var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-4095/burv/invoke-00000/output/*.txt
cat rbmm_moorings/tadmor/rbnnh_compose.yml
cat:*
cd *
cd /Users/bhyslop/projects/rbm_alpha_recipemuster
chmod +x:*
chmod:*
claude:*
command -v asciidoc
command -v asciidoctor
command -v docker
command -v ruby
command -v ruby gem asciidoctor
command -v shellcheck
command -v syft
conjure
cp ../output-buk/current/epic_geriatric_consult.assay.txt ../output-buk/current/epic_progress_note.assay.txt /tmp/apcnsa_compare/
cp ../output-buk/current/epic_progress_note.stanford.assay.txt ../output-buk/current/epic_geriatric_consult.stanford.assay.txt /tmp/apcnsa_compare/
cp /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbj_sentry.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels/rbev-sentry-debian-slim/rbj_sentry.sh
cp /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rboc_censer.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels/rbev-sentry-debian-slim/rboc_censer.sh
curl:*
DE
dig +short A anthropic.com
dig +short A api.anthropic.com
dig +short A api.claude.ai
dig +short A claude.ai
dig +short:*
diskutil list:*
dns-sd -t 3 -B _smb._tcp
dns-sd -t 3 -B _ssh._tcp
dns-sd -t 5 -B _services._dns-sd._udp local.
do echo:*
do grep:*
do if:*
do printf:*
do sed:*
docker --version
docker build:*
docker buildx *
docker compose:*
docker container *
docker context *
docker create:*
docker exec:*
docker image:*
docker images:*
docker info:*
docker manifest:*
docker network:*
docker ps:*
docker pull:*
docker rm:*
docker rmi:*
docker run:*
docker start:*
docker system:*
docker tag:*
docker version:*
done
dscacheutil -q host -a name rocket
dseditgroup:*
DV
echo '=== AT_ SERVICE TERMS \(at_bottle_service, at_censer_container, at_agile_service, at_sessile_service\) BY FILE ===' grep -r "at_bottle_service\\|at_censer_container\\|at_agile_service\\|at_sessile_service" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.json
echo '=== BOTTLE_START/RUN REFERENCES BY FILE ===' grep -r "bottle_start\\|bottle_run\\|bottle_service" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.json
echo '=== CENSER REFERENCES BY FILE ===' grep -r censer /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.yml --include=*.json
echo '=== FRONTISPIECE REFERENCES BY FILE ===' grep -r "ConnectBottle\\|ConnectCenser\\|ConnectSentry\\|ObserveNetworks" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.yml --include=*.json
echo '=== MCM TIER TERMS \(lemma/lemmata/graven/intaglio/quoin/sprue/inlay\) ===' echo 'Searching for MCM vocabulary...' grep -r "lemma\\|lemmata\\|graven\\|intaglio\\|quoin\\|sprue\\|inlay" /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools --include=*.adoc --include=*.md
echo '=== OP*_ TERM REFERENCES \(opbs_, opbr_, opss_\) BY FILE ===' grep -r "opbs_\\|opbr_\\|opss_" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.json
echo "  renamed bare output in $f"
echo "  renamed in $f"
echo "--- exit $? ---"
echo "--- exit $?"
echo "--- exit: $? ---"
echo "----- EXIT $? -----"
echo "---EXIT $?---"
echo "---exit: $?---"
echo "---exit: $?"
echo "---exit:$?---"
echo "---EXIT:$?---"
echo "---EXIT:$?"
echo "---exit=$?---"
echo "---EXIT=$?---"
echo "---exit=$?"
echo "---grep exit: $?"
echo "\(context: $\(docker context show \)\)"
echo "\(exit $? — 1 means clean\)"
echo "\(exit $? — grep exit 1 = no matches = good\)"
echo "\(exit $?\)"
echo "\(grep exit $?\)"
echo "\(reboot command exit: $?\)"
echo "=== augur exit: $? ==="
echo "=== banish exit: $? ==="
echo "=== build exit: $? ==="
echo "=== BUILD EXIT: $? ==="
echo "=== Charge exit: $? ==="
echo "=== CHARGE EXIT: $? ==="
echo "=== emplace exit: $? ==="
echo "=== EXIT $? ==="
echo "=== exit code: $? ==="
echo "=== exit: $? \(1 = no matches = clean\) ==="
echo "=== exit: $? ==="
echo "=== EXIT: $? ==="
echo "=== fast exit $? ==="
echo "=== fast suite exit: $? ==="
echo "=== FAST SUITE EXIT: $? ==="
echo "=== generate exit: $? ==="
echo "=== grep exit \(1 = no matches, clean\): $? ==="
echo "=== KLUDGE-SENTRY EXIT: $? ==="
echo "=== KludgeBottle exit: $? ==="
echo "=== KludgeSentry exit: $? ==="
echo "=== merge exit: $? ==="
echo "=== MERGE EXIT: $? ==="
echo "=== native immure exit: $? ==="
echo "=== podvm-lifecycle fixture exit: $? ==="
echo "=== PUSH EXIT: $? ==="
echo "=== qualify exit: $? ==="
echo "=== rbtd-t exit $? ==="
echo "=== rebase exit: $? ==="
echo "=== REBASE EXIT: $? ==="
echo "=== shellcheck exit: $? ==="
echo "=== SHELLCHECK EXIT: $? ==="
echo "=== ssh exit: $? ==="
echo "=== SUITE EXIT: $? ==="
echo "=== TABTARGET EXIT $? ==="
echo "===APPEND EXIT=$?==="
echo "===BACKUP EXIT=$?==="
echo "===CLASSES UNLOAD EXIT=$?==="
echo "===EXIT $?"
echo "===KNOCK EXIT=$?==="
echo "===KNOCK1 EXIT=$?==="
echo "===KNOCK2 EXIT=$?==="
echo "===NTUSER UNLOAD EXIT=$?==="
echo "===RESTART EXIT=$?==="
echo "===SHUTDOWN EXIT=$?==="
echo "==DOGFIGHT_EXIT=$?=="
echo "==LODE_LIFECYCLE_EXIT=$?=="
echo "ACG_EXIT=$?"
echo "asciidoctor: $\(asciidoctor --version
echo "BUILD exit: $?"
echo "build EXIT=$?"
echo "BUILD_EXIT:$?"
echo "BUILD_EXIT=$?"
echo "buw-st exit: $?"
echo "CANONICAL_INVEST_EXIT=$?"
echo "census exit: $?"
echo "census-clean: $?"
echo "CHARGE_EXIT=$?"
echo "clean: $?"
echo "CrashCourse exit: $?"
echo "DH_EXIT=$?"
echo "diff-establish-exit=$?"
echo "diff-refresh-exit=$?"
echo "director divest rc=$?"
echo "DIRECTOR_PROBE_EXIT=$?"
echo "DOGFIGHT_EXIT=$?"
echo "establish exit=$?"
echo "EV_EXIT=$?"
echo "example.com exit=$?"
echo "exit $?"
echo "exit code: $?"
echo "Exit code: $?"
echo "Exit codes: $?"
echo "EXIT_acd=$?"
echo "EXIT_acg=$?"
echo "EXIT_acr=$?"
echo "EXIT_CODE=$?"
echo "EXIT_RM=$?"
echo "EXIT_RUN=$?"
echo "EXIT_WRITE=$?"
echo "exit: $?  \(1 = no matches, good\)"
echo "exit: $?  \(no hits = clean gate\)"
echo "exit: $? \(1 = no matches = clean\)"
echo "exit: $? \(1 = none, as desired\)"
echo "exit: $? \(1 = truly none\)"
echo "exit: $? \(1 = zero hits, good\)"
echo "exit: $?"
echo "EXIT: $?"
echo "exit: $?" echo '--- 2. RBGC_VOUCHES_PACKAGE ---' grep -rn 'RBGC_VOUCHES_PACKAGE' Tools/
echo "exit: $?" echo '--- 3. Old hyphen-suffix tag concat ---' grep -rnE ':\\$?\\{[^}]*HALLMARK[^}]*\\}-\(image|vouch|pouch|about|diags\)' Tools/ .rbk/
echo "exit: $?" echo '--- 4. Old enshrine tag grammar ---' grep -rnE 'enshrine:\\{?«?[Aa]nchor' Tools/
echo "exit: $?" echo '--- 5. Vouches aggregator phrasing ---' grep -rnE 'vouches \(superdirectory|package|aggregator\)' Tools/rbk/vov_veiled/
echo "exit: $?" echo '--- 6. Stale «HALLMARK»-image / «HALLMARK»-about etc. ---' grep -rnE '«HALLMARK»-\(image|vouch|pouch|about|diags\)' Tools/rbk/vov_veiled/
echo "exit: $?" echo '--- 7. Stale {vessel}:{hallmark}-X ---' grep -rnE '\\{vessel\\}:\\{hallmark\\}-\(image|vouch|pouch|about|diags\)' Tools/rbk/vov_veiled/
echo "exit: $?" echo '=== Done ==='
echo "exit:$?"
echo "EXIT:$?"
echo "exit=$? — empty above = clean tree"
echo "exit=$? \(1 = no hits\)"
echo "exit=$?"
echo "EXIT=$?"
echo "FAST exit: $?"
echo "FAST_EXIT=$?"
echo "FIXTURE_EXIT:$?"
echo "FIXTURE_EXIT=$?"
echo "grep exit: $?"
echo "HANDBOOK_EXIT=$?"
echo "harness exit \(1 = diffs present, expected\): $?"
echo "harness exit: $?"
echo "IMMURE_EXIT=$?"
echo "JILT_EXIT: $?"
echo "JILT_RERUN_EXIT: $?"
echo "local: $\(shellcheck --version
echo "LODE_LIFECYCLE_EXIT=$?"
echo "ls-grep exit: $?"
echo "MACOS_DOGFIGHT2_EXIT=$?"
echo "merge exit: $?"
echo "o exit=$?"
echo "Occ exit=$?"
echo "PLUML_FIXTURE_EXIT=$?"
echo "quota exit=$?"
echo "RAV_EXIT=$?"
echo "RBFC_SMOKE_EXIT=$?"
echo "RBFL_SMOKE_EXIT=$?"
echo "rbw-fhv rbev-busybox RC=$?  \(rbfh furnish — sources rbfc0_FoundryCore.sh\)"
echo "rbw-gq RC=$? \(rbhp0 furnish — sources rbhp0_Payor.sh\)"
echo "rbw-hw exit: $?"
echo "rbw-hw RC=$? \(rbhw0 furnish — sources rbhw0_Windows.sh\)"
echo "rbw-o exit: $?"
echo "rbw-Odf exit: $?"
echo "rbw-Ofc exit: $?"
echo "rbw-tq exit: $?"
echo "RC_EXIT=$?"
echo "RC=$?"
echo "rdv exit: $?"
echo "REBASE_EXIT=$?"
echo "refresh exit=$?"
echo "RELIQUARY_LIFECYCLE_EXIT=$?"
echo "retriever divest rc=$?"
echo "RNV_CCYOLO_EXIT=$?"
echo "RNV_EXIT=$?"
echo "rrr exit: $?"
echo "rrv exit: $?"
echo "RRV_EXIT=$?"
echo "SERVICE_SUITE_EXIT=$?"
echo "SHELLCHECK exit: $?"
echo "shellcheck exit=$? \(0 = clean → gate would pass\)"
echo "SHELLCHECK_EXIT: $?"
echo "SHELLCHECK_EXIT=$?"
echo "SMOKE_EXIT=$?"
echo "ssh exit: $? \(connection drop on reboot is expected\)"
echo "stale-path check exit: $?"
echo "StartHere exit: $?"
echo "status exit: $?"
echo "storage.googleapis.com exit=$?"
echo "TABTARGET_EXIT=$?"
echo "test exit: $?"
echo "test EXIT=$?"
echo "TEST_EXIT=$?"
echo "THEURGE_EXIT=$?"
echo "unstamped check exit: $?"
echo "Updated: $v"
echo "vow-b exit: $?"
echo "vvw-r dispatch exit: $?"
echo "WSL_LIFECYCLE_EXIT=$?"
env
export SOC='echo hi | tr a-z A-Z'
external command\\|exit code\\|stderr suppres" Tools/buk/vov_veiled/BCG-BashConsoleGuide.md | head -60
fi
find .claude/jjm/jje_* -type f
find .claude/jjm/officia/260603-1004/ -type f -size -2k
find "/Users/bhyslop/projects/rbm_alpha_recipemuster" -type f \\\( -name "*.sh" -o -name "*.adoc" -o -name "*.md" -o -name "*.rs" \\\) -exec grep -l "BURN_HOST\\|BURN_USER\\|BURN_ALIAS\\|BURN_SSH_PUBKEY\\|BURN_COMMAND" {} \\;
find /Users/bhyslop/projects/rbm_alpha_recipemuster -maxdepth 3 -name *gallop* -o -name *A4*
find /Users/bhyslop/projects/rbm_alpha_recipemuster -name "*.log" 2>/dev/null | head -5
find /Users/bhyslop/projects/rbm_alpha_recipemuster -name "*.md" -exec grep -l "volume\\|compose\\|rbob_charge" {} \\;
find /Users/bhyslop/projects/rbm_alpha_recipemuster -name *.manifest -o -name *manifest*
find /Users/bhyslop/projects/rbm_alpha_recipemuster -name *RBSIP* -o -name *ifrit*
find /Users/bhyslop/projects/rbm_alpha_recipemuster -name *test*.sh -o -name *tcase*.sh -o -name *fixture*.sh
find /Users/bhyslop/projects/rbm_alpha_recipemuster -name direct_verify.py -o -name *direct*.py
find /Users/bhyslop/projects/rbm_alpha_recipemuster -name launcher* -type f
find /Users/bhyslop/projects/rbm_alpha_recipemuster -name rbrn.env* -type f
find /Users/bhyslop/projects/rbm_alpha_recipemuster -path *station-files* -name *.env
find /Users/bhyslop/projects/rbm_alpha_recipemuster -type d -name "rb*" 2>/dev/null | head -50
find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f -name "*jjx*" -o -name "*bridle*" -o -name "*chalk*" -o -name "*arm*" 2>/dev/null | grep -v node_modules | head -30
find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f -name *compose*.yml
find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f -name *enshrine* -o -name *Enshrine*
find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f -name *paddock* -o -type f -name *AvAAA*
find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f -name *tR* -o -name *test*
find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f -path "*pluml*" -o -path "*plantuml*" 2>/dev/null | sort
find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f \\\(-name *paddock* -o -name *current* -o -name *heat* \\\)
find /Users/bhyslop/projects/rbm_alpha_recipemuster -type f \\\(-name *registry* -o -name *ledger* -o -name *manifest* \\\)
find /Users/bhyslop/projects/rbm_alpha_recipemuster/.buk -name "rbrn*.env" -type f | head -1 | xargs cat 2>/dev/null
find /Users/bhyslop/projects/rbm_alpha_recipemuster/.claude/commands -type f -name "*.md" 2>/dev/null | sort
find /Users/bhyslop/projects/rbm_alpha_recipemuster/.claude/jjm -name *.md -o -name *.json
find /Users/bhyslop/projects/rbm_alpha_recipemuster/.claude/jjm -type f -name *.md
find /Users/bhyslop/projects/rbm_alpha_recipemuster/.rb -name "*.env" -type f 2>/dev/null | head -5
find /Users/bhyslop/projects/rbm_alpha_recipemuster/.rb -type f -name "rbrn*.env" | head -1 | xargs cat
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Memos -type f -name "*.md" 2>/dev/null | head -20
find /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels -name "rbrv.env" -type f -exec head -30 {} +
find /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels -type f -name *kludge* -o -name *compose*
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools -name *enrollment* -o -name *Enrollment*
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools -name *guard* -o -name *size*
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk -name "*resentation*" -o -name "*bupr*" 2>/dev/null | head -5
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk -type f -name *.sh
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk -name "*.sh" -type f ! -path "*ABANDONED*" -exec grep -l 'if [a-z_][a-z_0-9]*;' {} \\; 2>/dev/null | head -20
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk -name *.rs -type f
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk/vov_veiled -name *Gallops* -o -name *Data*
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk -name rbtd* -type f
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgja -type f -name "*.sh" -exec wc -lc {} +
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgjb -type f -name "*.sh" -exec wc -lc {} +
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgje -type f -name "*.sh" -exec wc -lc {} +
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgjm -type f -name "*.sh" -exec wc -lc {} +
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgjv -type f -name "*.sh" -exec wc -lc {} +
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtd/src -name *.rs -type f
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtd/src -type f -name *.rs
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtid -type f -name *.py -o -name *.txt
find /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/vvc -name *.rs -type f
find /Users/bhyslop/projects/rbm_alpha_recipemuster/tt -name jja-* -o -name jjc-*
find /var/folders -maxdepth 5 -name rbtd-* -type d
find Tools/buk Tools/rbk -name "*.sh" -type f -exec grep -l "^[a-z]*_kindle\(\)" {} \\; | head -20
find Tools/rbk Tools/buk -name *.sh -type f -exec grep -Hn 'sed ' {}
find Tools/rbk Tools/buk -name *.sh -type f -exec grep -Hn "^\\s*sed\\||\\s*sed" {}
find Tools/rbk/vov_veiled Tools/vok/vov_veiled -name '*.adoc' -exec grep -l 'axhop_parameter_from_type\\|axhop_parameter_from_arg\\|axhoo_output_of_type' {}
find Tools/vok/vov_veiled -name '*.adoc' -exec grep -l '//axhoo_output$' {}
find:*
for anchor:*
for crate:*
for d:*
for f:*
for file:*
for u:*
for v:*
gawk --version
gcloud --quiet components install beta 2>&1
gcloud artifacts:*
gcloud auth:*
gcloud beta:*
gcloud billing *
gcloud builds:*
gcloud config:*
gcloud iam:*
gcloud projects:*
gcloud services:*
gem install *
getent hosts *
gh api:*
gh auth:*
gh issue *
gh repo:*
git show *
git stash:*
GIT_EDITOR=true git -C /Users/bhyslop/projects/rbm_alpha_recipemuster rebase --continue
GIT_EDITOR=true git rebase --continue
git:*
grep -h "^[a-z][a-z0-9]*_[a-z0-9]*\\\(\\\)" /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/burc_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/burs_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/bure_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/burd_regime.sh 2>/dev/null | sort | uniq
grep -h "^[a-z][a-z0-9]*_[a-z0-9]*\\\(\\\)" /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbra_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbrp_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbrs_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbro_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbrg_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbrn_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbrr_regime.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbrv_regime.sh 2>/dev/null | sort | uniq
grep -n '\\$\(' Tools/rbk/rbf_Foundry.sh | grep -v '\\$\(<' | grep -v '\\$\(\(' | grep -v '_capture' | head -30
grep -n '\\$\(' Tools/rbk/rbf_Foundry.sh | grep -v '\\$\(<' | grep -v '\\$\(\(' | grep -v '_capture' | head -80
grep -n '\\$\(' Tools/rbk/rbgp_Payor.sh | grep -v '\\$\(<' | grep -v '\\$\(\(' | grep -v '_capture'
grep -n '\\$\(' Tools/rbk/rbgp_Payor.sh | grep -v '\\$\(<' | grep -v '\\$\(\(' | grep -v '_capture' | head -30
grep -n "^=\\{1,3\\} " Tools/jjk/vov_veiled/JJS0_JobJockeySpec.adoc
grep -rln "Forthcoming" .
grep -rn -i "CI\\b\\|CI/CD\\|automated\\|unattended\\|headless\\|test run\\|gauntlet\\|nightly\\|pipeline\\|cron" Memos/memo-20260427-google-native-human-auth.md Memos/memo-20260522-org-affiliated-credential-reorientation.md
grep:*
head:*
host 192.168.1.246
host 192.168.1.247
host 93.184.216.34
host rocket *
iconv -t UTF-16LE
id "$u"
ifconfig -l
JJTEST_HOST=localhost cargo test --manifest-path Tools/jjk/vov_veiled/Cargo.toml --test fundus_scenario relay_concurrent_overlap -- --ignored --nocapture 2>&1
jq -r '.privateKeyData' /var/folders/h1/ftrhww8d157ckvszlp_t62hh0000gn/T/rbtd-20366/burv/invoke-00002/temp/temp-20260427-130309-22827-763/rbgu_gov_key-attempt1_u_resp.json
jq:*
kill -9 29018 29104
kill %1
kill 58792
LC_ALL=C uniq -c
LOG_DIR="/Users/bhyslop/projects/rbm_alpha_recipemuster/../_logs_buk" for f in hist-rbw-DI-sh-20260327-171824-18370-204.txt hist-rbw-DI-sh-20260327-172456-18929-881.txt hist-rbw-DE-sh-20260327-182829-22515-574.txt hist-rbw-DC-sh-20260327-183013-23423-580.txt hist-rbw-DC-sh-20260327-201528-53812-528.txt hist-rbw-DC-sh-20260327-202101-55053-710.txt hist-rbw-DC-sh-20260327-202655-57216-703.txt hist-rbw-DE-sh-20260327-203624-60526-870.txt hist-rbw-DC-sh-20260327-203832-61483-888.txt hist-rbw-DE-sh-20260327-205221-65590-120.txt hist-rbw-DC-sh-20260327-205505-66475-551.txt hist-rbw-Rs-sh-20260327-210847-70631-97.txt hist-rbw-Rs-sh-20260327-210923-71464-289.txt
ls -1 RBSD*.adoc
ls -1 RBSM*.adoc
ls -1 RBSP*.adoc
ls -1 RBSR*.adoc
ls -laR .claude/jjm/officia/260603-1004/
ls "../temp-buk/temp-20260609-104733-83286-60/rbtd/rbtdrc_reliquary_lifecycle/"
ls /Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-*.sh | xargs -I{} basename {} | sort
ls:*
lsof -nP -iTCP:7999 -sTCP:LISTEN
md5:*
mdfind:*
mdls -name kMDItemDisplayName -name kMDItemContentType -name kMDItemKind "/Applications/SlickEditPro2025.app" 2>&1
mdutil -s / 2>&1
mkdir -p .claude/jjm/officia/260419-1001
mkdir -p .rbk/tadmor .rbk/srjcl .rbk/pluml
mkdir -p /tmp/a6aar_after
mkdir -p /tmp/a6aar_baseline
mkdir -p /tmp/apcnsa_compare
mkdir -p /tmp/fmttest
mkdir -p /tmp/rbrd-capture
mkdir -p /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels/rbev-bottle-ccyolo/build-context
mkdir -p /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels/rbev-bottle-ccyolo/workspace
mkdir -p rbmm_moorings/rbml_launchers
mkdir p1014 *
mkdir vvk1013_install
mv .claude/jjm/officia/260604-1004/gazette_out.md .claude/jjm/officia/260604-1004/gazette_in.md
mv .rbk/pluml.rbrn.env .rbk/pluml/rbrn.env
mv .rbk/srjcl.rbrn.env .rbk/srjcl/rbrn.env
mv .rbk/tadmor.compose.yml .rbk/tadmor/compose.yml
mv .rbk/tadmor.rbrn.env .rbk/tadmor/rbrn.env
mv /tmp/APCS0-new.adoc Tools/apck/APCS0-SpecTop.adoc
mv /Users/bhyslop/projects/rbm_alpha_recipemuster/.claude/jjm/officia/☉260512-1014/gazette_in.md /Users/bhyslop/projects/rbm_alpha_recipemuster/.claude/jjm/officia/260512-1014/gazette_in.md
mv /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk/jjf_cli.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk/jjfp_cli.sh
mv /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk/jjf_fundus.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk/jjfp_fundus.sh
mv Tools/rbk/rbfck_Kindle.sh        Tools/rbk/rbfc0_FoundryCore.sh
mv Tools/rbk/rbflk_Kindle.sh        Tools/rbk/rbfl0_FoundryLedger.sh
mv Tools/rbk/rbh0/rbhob_base.sh     Tools/rbk/rbh0/rbho0_Onboarding.sh
mv Tools/rbk/rbldk_Kindle.sh        Tools/rbk/rbld0_Lode.sh
mv tt/rbw-DA.DirectorAbjuresArk.sh tt/rbw-DA.DirectorAbjuresConsecration.sh
mv tt/rbw-DC.DirectorCreatesArk.sh tt/rbw-DC.DirectorCreatesConsecration.sh
mv tt/rbw-DE.DirectorEnshrinesBaseImages.sh tt/rbw-DE.DirectorEnshrinesVessel.sh
mv:*
nc -G 5 -w 5 -vz github.com 22
nc -G 5 -w 5 -vz github.com 443
nc -G 5 -w 5 -vz ssh.github.com 443
nc -u -w 2 -b 255.255.255.255 9999
nc -w 5 192.0.46.9 80
nc -w 5 www.internic.net 80
nc -z -G 3 bhyslop-nas2 445
nc -z -G 3 DS223j 445
nc -z -G 3 DS223j 5000
nc -z -G 3 DS223j 5001
nc -z -G 3 DS223j 548
nc -zv -G 2 169.254.105.91 22
nc -zv -G 2 169.254.105.91 5000
nc -zv -G 2 169.254.105.91 5001
networksetup -getairportnetwork en1
networksetup -listallhardwareports
npm view:*
npm:*
open:*
openssl dgst:*
openssl enc:*
openssl pkey *
openssl version *
osascript -e 'quit app "Docker Desktop"'
osascript -e 'quit app "Docker"'
pandoc --version
perl:*
ping -c 1 -W 1 192.168.1.246
ping -c 1 -W 1 192.168.1.247
ping -c 1 -W 1 192.168.86.245
ping -c 1 -W 1 bhyslop-nas2
ping -c 1 -W 1 bhyslop-nas2.local
ping -c 1 -W 1 DS223j
ping -c 1 -W 1 DS223j.local
ping -c 2 -W 1 192.168.1.1
ping -c 2 -W 1 bhyslop-nas2.local
ping -c 2 -W 1 DS223j.local
ping -c 2 -W 1000 169.254.105.91
ping -c 3 -i 0.3 -b 169.254.255.255
ping -c 3 -W 1000 -b en0 169.254.105.91
ping -c 3 -W 1000 169.254.105.91
ping *
pip show *
pkill -9 -f "docker pull"
pkill -f "docker pull"
pkill -f "rbw_workbench.sh"
pkill -f "rbw-cKS"
pkill -f "rbw-tf.TestFixture.four-mode"
pkill -f "rbw-tf.TestFixture.three-mode"
pkill -f "target/debug/apcap"
pkill -f "target/release/apcap"
plutil -p "/Applications/SlickEditPro2025.app/Contents/Info.plist" 2>&1 | head -30
podman images:*
podman machine:*
podman system:*
printf '{""""alg"""":""""RS256"""",""""typ"""":""""JWT""""}'
printf '\\n# eof\\n'
printf '%s' "YzI2MDQwMjA5MTIzMi1yMjYwNDAyMTYxNTIx"
printf '%s\\n' "YzI2MDQwMjA5MTIzMi1yMjYwNDAyMTYxNTIx"
printf 'GET / HTTP/1.1\\r\\nHost: www.internic.net\\r\\nConnection: close\\r\\n\\r\\n'
printf 'GET / HTTP/1.1\\r\\nHost: www.internic.net\\r\\nUser-Agent: rbid/1.0\\r\\nConnection: close\\r\\n\\r\\n'
printf 'Hello World'
printf 'scp-protocol-probe\\n'
printf 'SGVsbG8gV29ybGQ='
printf 'use foo::{aaa, bbb, ccc};\\nfn main\(\) {}\\n'
ps -p 8488 -o pid,etime,command
pstree -p 13501
python3 -c ":*
python3:*
read a *
read f *
Read Tools/rbk/rbtd/src/main.rs
rg *
rm -f /tmp/scp_probe.txt
rm -f /tmp/tier5_check.sh /tmp/tier5_check2.sh
rm -f /tmp/tier6_check.sh
rm -rf /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid/target
rm -rf p1014
rm -rf vvk1013_install
rm /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/*.bak
rm Tools/rbk/rbh0/rbhpq_quota_build.sh.bak
rm tt/rbw-Db.DirectorBuildsAbout.sh
rmdir .buk .rbk
rmdir /Users/bhyslop/projects/rbm_alpha_recipemuster/.claude/jjm/officia/☉260512-1014
rmdir /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbid
ruby -e 'require "asciidoctor"; puts "ASCIIDOCTOR_GEM_OK"'
rustc --version
rustfmt --config imports_layout=Vertical --emit stdout /tmp/fmttest/t.rs
rustfmt --emit stdout --config imports_granularity=Module t.rs
rustfmt --emit stdout t.rs
rustfmt --version
rustup run *
scp:*
script -q /dev/null ./tt/rbw-DI.DirectorInscribesRubric.sh 2>&1
script -q /dev/null ./tt/rbw-DPG.DirectorRefreshesGcbPins.sh 2>&1
script -q /dev/null ./tt/rbw-tf.TestFixture.three-mode.sh 2>&1
sed -E 's/\(=.{6}\).*/\\1…/' ../station-files/secrets/payor/rbro.env
sed -f /tmp/apcs0-rename.sed Tools/apck/APCS0-SpecTop.adoc
sed -i '' -e 's|Memos/memo-20260610-heat-BH-fable-recommendation-convergence-deadline-shape\\.md|Memos/retired/memo-20260610-heat-BH-fable-recommendation-convergence-deadline-shape.md|g' Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc
sed -i '' -e 's|Memos/memo-20260610-heat-BH-image-tabtarget-cleanup\\.md|Memos/retired/memo-20260610-heat-BH-image-tabtarget-cleanup.md|g' Tools/rbk/rbz_zipper.sh Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc
sed -i '' -e 's|Memos/memo-20260610-quoin-minting-introspection\\.md|Memos/retired/memo-20260610-quoin-minting-introspection.md|g' Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
sed -i '' '/shellcheck disable=SC2153/,+3d' /tmp/rbgg_no_disable.sh
sed -i '' 's/^RBRV_RELIQUARY=.*/RBRV_RELIQUARY=r260327172456/' "$v"
sed -i.bak -e 's/RBRR_DEPOT_PROJECT_ID/RBRR_DEPOT_MONIKER/g' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/rbhodf_director_first_build.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/rbhoda_director_airgap.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/rbhodb_director_bind.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/rbhodg_director_graft.sh
sed -i.bak -e s/RBRR_DEPOT_PROJECT_ID/RBDC_DEPOT_PROJECT_ID/g -e s/RBRR_GCB_POOL_STEM/RBDC_GCB_POOL_STEM/g Tools/rbk/rbh0/rbhpq_quota_build.sh
sed -n '/^buc_die\(\)/,/^}/p' Tools/buk/buc_command.sh
sed -n '/^buc_reject\(\)/,/^}/p' Tools/buk/buc_command.sh
sed -n '/furnish\(\) {/,/^}/p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/burc_cli.sh
sed -n '/furnish\(\) {/,/^}/p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/burn_cli.sh
sed -n '/furnish\(\) {/,/^}/p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/burs_cli.sh
sed -n '/Obtain OAuth2 token/,/SLOT_1_ORIGIN/p' /tmp/new_body_rbgjl01-ensconce-capture.sh
sed -n '/rbcc_emit_consts\(\)/,/^}/p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbcc_constants.sh
sed -n '1,14p' Tools/rbk/rbgjs/rbgjs-skopeo-fingerprint.sh
sed -n '1,16p' Tools/rbk/rbgjs/rbgjs-gpg-verify-sums.sh
sed -n '1,20p' Tools/rbk/rbgjs/rbgjs-gcrane-append.sh
sed -n '1,20p' Tools/rbk/rbgjs/rbgjs-gpg-verify-sums.sh
sed -n '1,25p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/bul_launcher.sh
sed -n '1,25p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/bul_nolog_launcher.sh
sed -n '1,2p' .claude/jjm/officia/260604-1004/gazette_in.md
sed -n '1,30p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/but_test.sh
sed -n '1,40p' Tools/buk/bubc_constants.sh
sed -n '1,40p' Tools/rbk/rbtd/src/lib.rs
sed -n '1,40p' Tools/rbk/rbtd/src/main.rs
sed -n '10,14p' RBRN-RegimeNameplate.adoc
sed -n '10,16p' RBSRV-RegimeVessel.adoc
sed -n '100,108p' Tools/rbk/rbtd/src/rbtdrm_manifest.rs
sed -n '100,108p' Tools/rbk/vov_veiled/RBSCJ-CloudBuildJson.adoc
sed -n '100,112p' Tools/rbk/rbz_zipper.sh
sed -n '1033,1037p' Tools/rbk/rbtd/src/rbtdrf_fast.rs
sed -n '106,109p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
sed -n '116,124p' Memos/memo-20260609-federation-canon.md
sed -n '118p;176p;178p;190p;192p;218p;251p' rbmm_moorings/rbmv_vessels/common-sentry-context/rbjs_sentry.sh
sed -n '119,125p' Tools/rbk/rbtd/src/rbtdtp_pristine.rs
sed -n '1239,1243p' Tools/rbk/rbtd/src/rbtdrf_fast.rs
sed -n '125,150p' Tools/rbk/rbfc0_core.sh
sed -n '126,142p' Tools/rbk/rbtd/src/rbtdrp_pristine.rs
sed -n '1270,1320p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
sed -n '128,132p' README.md
sed -n '128,145p' Tools/buk/buut_tabtarget.sh
sed -n '128,150p' Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
sed -n '140,160p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '140,160p' Tools/rbk/rbz_zipper.sh
sed -n '1438,1480p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
sed -n '1462,1466p' Tools/rbk/rbtd/src/rbtdrf_fast.rs
sed -n '150,153p' Tools/rbk/rbtd/src/rbtdtk_canonical.rs
sed -n '150,210p' Tools/rbk/rbrn_cli.sh
sed -n '155,190p' Tools/rbk/vov_veiled/CLAUDE.consumer.md
sed -n '159p' Tools/rbk/rbtd/src/rbtdrm_manifest.rs
sed -n '160,182p' Tools/rbk/rbh0/rbhots_tadmor_security.sh
sed -n '160,190p' Tools/rbk/rbfc_FoundryCore.sh
sed -n '165,170p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '168,171p' Tools/rbk/rbrn_regime.sh
sed -n '178,186p' Tools/rbk/rbfd_director.sh
sed -n '180,205p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbz_zipper.sh
sed -n '188,205p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtd/Tools
sed -n '195,230p' Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
sed -n '19p' Tools/rbk/rbtd/src/rbtdrp_pristine.rs
sed -n '20,40p' Tools/buk/buq_qualify.sh
sed -n '20,60p' Tools/rbk/rbldr_reliquary.sh
sed -n '200,300p'
sed -n '205,212p' Tools/rbk/vov_veiled/RBSAV-ark_vouch.adoc
sed -n '21,25p' Tools/rbk/vov_veiled/RBSAB-ark_about.adoc
sed -n '213,248p' Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
sed -n '218,224p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '23,27p' Tools/rbk/vov_veiled/RBSAC-ark_conjure.adoc
sed -n '230,240p' Tools/buk/buut_tabtarget.sh
sed -n '2306,2313p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
sed -n '246,251p' Memos/memo-20260609-federation-canon.md
sed -n '249,473p' Tools/rbk/rbfc_FoundryCore.sh
sed -n '25,33p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '25,50p' Tools/rbk/rbldb_bole.sh
sed -n '255,275p' Tools/rbk/rbgo_OAuth.sh
sed -n '260,270p' Memos/memo-20260609-bedrock-quire-shaping.md
sed -n '260,270p' Tools/rbk/rbfd_director.sh
sed -n '265,280p' Tools/rbk/vov_veiled/RBSCJ-CloudBuildJson.adoc
sed -n '28,32p' Tools/rbk/vov_veiled/RBSAC-ark_conjure.adoc
sed -n '285,298p' Tools/rbk/rbh0/rbhoda_director_airgap.sh
sed -n '2970,2990p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
sed -n '319,323p' Tools/buk/bujb_jurisdiction.sh
sed -n '320,489p' Tools/rbk/rbld_Lode.sh
sed -n '33,46p' Tools/buk/buut_tabtarget.sh
sed -n '33,92p' Tools/buk/burd_regime.sh
sed -n '330,345p' Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
sed -n '3314,3410p' Tools/rbk/rbtd/src/rbtdrc_crucible.rs
sed -n '336,345p' Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
sed -n '340,375p' Tools/buk/buv_validation.sh
sed -n '35,60p' Tools/rbk/rbldw_underpin.sh
sed -n '350,360p' Tools/rbk/rbgo_OAuth.sh
sed -n '360,380p' Tools/rbk/rbgc_constants.sh
sed -n '363,369p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '364,370p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '37,52p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtd/src/rbtdrk_canonical.rs
sed -n '371,384p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '38,46p' Tools/rbk/rbldv_immure.sh
sed -n '3855,3865p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
sed -n '39,70p' Tools/rbk/rbld_Lode.sh
sed -n '4,8p' Tools/rbk/vov_veiled/RBSAK-ark_kludge.adoc
sed -n '40,57p' Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
sed -n '46,49p' Tools/rbk/rboo_observe.sh
sed -n '472,478p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '52,62p' Tools/rbk/vov_veiled/RBSAC-ark_conjure.adoc
sed -n '524,546p' README.md
sed -n '55,58p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbfv_FoundryVerify.sh
sed -n '55,70p' Tools/rbk/rbfd_director.sh
sed -n '57,60p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbfl_FoundryLedger.sh
sed -n '6,9p' RBRN-RegimeNameplate.adoc
sed -n '6,9p' RBSRM-RegimeMachine.adoc
sed -n '6,9p' RBSRR-RegimeRepo.adoc
sed -n '6,9p' RBSRV-RegimeVessel.adoc
sed -n '60,95p' Tools/rbk/rbyc_common.sh
sed -n '62,70p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '63,69p' Tools/xxx_rbn.info.sh
sed -n '68,72p' Tools/rbk/rbrn_cli.sh
sed -n '68,74p' Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
sed -n '708,716p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '72,318p' Tools/rbk/rbld_Lode.sh
sed -n '730,742p' Tools/buk/buv_validation.sh
sed -n '741p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '743,755p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '75,110p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbcc_constants.sh
sed -n '75,85p' Tools/rbk/rbldv_immure.sh
sed -n '762,768p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '777,783p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '795,802p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '8,11p' RBSRT-RegimeDepot.adoc
sed -n '81,85p' Tools/rbk/vov_veiled/RBSAK-ark_kludge.adoc
sed -n '836,843p' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
sed -n '86,92p' Tools/rbk/rbtd/src/rbtdrk_canonical.rs
sed -n '932,942p;972,976p' Tools/rbk/rbtd/src/rbtdrf_fast.rs
sed -n '95,115p' Tools/rbk/rbfd_director.sh
sed -n '95,125p' Memos/memo-20260609-bedrock-quire-shaping.md
sed -n '95,140p' /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/buc_command.sh
sed -n '9763,9772p' .claude/jjm/jjg_gallops.json
sed -n '9946,9975p' .claude/jjm/jjg_gallops.json
sed -n 118,128p Tools/rbk/vov_veiled/RBSLE-lode_ensconce.adoc
sed -n 122,132p Tools/rbk/vov_veiled/RBSLU-lode_underpin.adoc
sed -n 141,160p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbldr_Reliquary.sh
sed -n 155,175p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbldw_Underpin.sh
sed -n 17,33p Tools/rbk/__TRACKED_VAR__.sh
sed -n 202,225p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbldv_Immure.sh
sed -n 220,260p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbfv_FoundryVerify.sh
sed -n 260,275p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/vov_veiled/RBSCJ-CloudBuildJson.adoc
sed -n 300,320p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
sed -n 330,336p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
sed -n 35,46p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc
sed -n 388,394p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
sed -n 470,500p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbfv_FoundryVerify.sh
sed -n 60,70p Tools/rbk/vov_veiled/RBSLA-lode_augur.adoc
sed -n 75,90p /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
sed -n 90,100p Tools/rbk/vov_veiled/RBSLC-lode_conclave.adoc
sed 's/\\.adoc$//'
sed 's/^/:/;s/$/:/'
set -e
sftp wsl@rocket
sh -c *
sh -n /Users/bhyslop/projects/rbm_alpha_recipemuster/rbev-vessels/common-sentry-context/rbjs_sentry.sh
shasum -a 256
shasum -a 256 /Users/bhyslop/projects/station-files/secrets/payor/bhyslop-gmail-com.20260513.rbro.env
shasum -a 256 /Users/bhyslop/projects/station-files/secrets/payor/rbro.env
shasum -a 256 rbmm_moorings/rbmv_vessels/common-sentry-context/rbjs_sentry.sh rbmm_moorings/rbmv_vessels/common-ifrit-context/src/rbida_sorties.rs
shasum Tools/vvk/bin/vvx
shellcheck --external-sources --shell=bash Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbro_regime.sh Tools/rbk/rbro_cli.sh Tools/rbk/rbgv_AccessProbe.sh
shellcheck --external-sources --shell=bash Tools/rbk/rbgp_Payor.sh
shellcheck --rcfile Tools/buk/busc_shellcheckrc --shell=bash Tools/rbk/rbuh_Http.sh Tools/rbk/rbgc_Constants.sh
shellcheck --rcfile=Tools/buk/busc_shellcheckrc -S style -f gcc Tools/rbk/rbq_Qualify.sh Tools/rbk/rblm_cli.sh
shellcheck --rcfile=Tools/buk/busc_shellcheckrc -x Tools/rbk/rbrr_regime.sh Tools/rbk/rbrd_regime.sh Tools/rbk/rbrv_regime.sh Tools/rbk/rbrn_regime.sh
shellcheck --rcfile=Tools/buk/busc_shellcheckrc Tools/rbk/rbgc_Constants.sh Tools/rbk/rbgi_IAM.sh Tools/rbk/rbgg_Governor.sh
shellcheck --rcfile=Tools/buk/busc_shellcheckrc Tools/rbk/rbgg_Governor.sh Tools/rbk/rbgu_Utility.sh
shellcheck --severity=error Tools/rbk/*.sh 2>&1 | head -40
shellcheck --version
shellcheck -e SC2155,SC1090,SC1091 Tools/rbk/rbgp_Payor.sh
shellcheck -f gcc /tmp/rbgg_no_disable.sh
shellcheck -f gcc /tmp/rbgg_top.sh
shellcheck -f gcc /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgg_Governor.sh
shellcheck -s bash -e SC1090,SC1091,SC2034 /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
shellcheck -s bash /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/buk/bujb_jurisdiction.sh
shellcheck -S error /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgl_GarLayout.sh /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgc_Constants.sh
shellcheck -S error /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbob_bottle.sh
shellcheck -S error Tools/rbk/rbfl_FoundryLedger.sh Tools/rbk/rbfl_cli.sh Tools/rbk/rbz_zipper.sh Tools/rbk/rbfd_FoundryDirectorBuild.sh Tools/rbk/rbh0/rbhodb_director_bind.sh Tools/rbk/rbh0/rbhodf_director_first_build.sh Tools/rbk/rbh0/rbhodg_director_graft.sh
shellcheck -S style -f gcc Tools/buk/*.sh 2>&1 | grep -E 'SC2016|SC2329|SC2004|SC2254|SC2012' | head -10
shellcheck -S style -f gcc Tools/buk/*.sh 2>&1 | grep -oE 'SC[0-9]+' | sort | uniq -c | sort -rn
shellcheck -S style -f gcc Tools/buk/*.sh 2>&1 | grep -oP 'SC[0-9]+' | sort | uniq -c | sort -rn
shellcheck -S style -f gcc Tools/buk/*.sh 2>&1 | grep 'SC2153' | head -5
shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep -oE 'SC[0-9]+' | sort | uniq -c | sort -rn
shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep -oP 'SC[0-9]+' | sort | uniq -c | sort -rn
shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2034' | head -10
shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2059' | head -5
shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2086' | grep -v 'rbo.observe.sh' | head -5
shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2086' | head -10
shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2153' | head -5
shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2188' | head -5
shellcheck -S style -f gcc Tools/rbw/*.sh 2>&1 | grep 'SC2295' | head -5
shellcheck -S style -f gcc Tools/rbw/*.sh Tools/buk/*.sh 2>&1 | grep 'SC2188'
shellcheck -S style -f gcc Tools/rbw/rbf_Foundry.sh 2>&1 | grep 'SC2188' -A0 | head -6
shellcheck -S style Tools/rbw/rbgu_Utility.sh 2>&1 || true
shellcheck -S style Tools/rbw/rbob_cli.sh 2>&1 || true
shellcheck -S warning Tools/rbw/rbgu_Utility.sh 2>&1 || true
shellcheck -S warning Tools/rbw/rbob_cli.sh 2>&1 || true
shellcheck -x -s bash Tools/rbk/rbfc_FoundryCore.sh
shellcheck -x Tools/rbk/rbf_Foundry.sh 2>&1 | head -30
shellcheck -x Tools/rbk/rbfd_FoundryDirectorBuild.sh
shellcheck -x Tools/rbk/rbrr_regime.sh Tools/rbk/rbrd_regime.sh Tools/rbk/rbrv_regime.sh Tools/rbk/rbrn_regime.sh
shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbfd_FoundryDirectorBuild.sh
shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbfl_FoundryLedger.sh
shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_cli.sh
shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgp_Payor.sh
shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/rbhodb_director_bind.sh
shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbh0/rbhodg_director_graft.sh
shellcheck /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbz_zipper.sh
shellcheck Tools/rbk/rbfc_FoundryCore.sh
shellcheck Tools/rbk/rbfd_FoundryDirectorBuild.sh
shellcheck Tools/rbk/rbfv_FoundryVerify.sh
shellcheck Tools/rbk/rbgc_Constants.sh Tools/rbk/rbgp_Payor.sh
shellcheck Tools/rbk/rbgjr/rbgjr01-reliquary-preflight.sh
shellcheck Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbgp_Payor.sh Tools/buk/buh_handbook.sh Tools/rbk/rbgu_Utility.sh Tools/rbk/rbgg_Governor.sh Tools/rbk/rbgc_Constants.sh Tools/rbk/rbrp_regime.sh
shellcheck Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbgp_Payor.sh Tools/rbk/rbgg_Governor.sh
shellcheck Tools/rbk/rbgp_Payor.sh
shellcheck Tools/rbk/rbgp_Payor.sh Tools/buk/buh_handbook.sh
shellcheck Tools/rbk/rbgp_Payor.sh Tools/rbk/rbgp_cli.sh Tools/rbk/rbgc_Constants.sh
shellcheck Tools/rbk/rbgu_Utility.sh Tools/rbk/rbfc_FoundryCore.sh
skopeo inspect:*
sort -k2
sort -k2 -rn
sort -k6,9
sort -t, -k15 -rn
sort -t: -k1
sort -t: -k2 -n -r
sort -t: -k2 -rn
sort -t'\(' -k2 -rn
sort -u cities.txt -o cities.txt
sort -u grep -oE '\\{apcs_[a-z_]+\\}' Tools/apck/APCS0-SpecTop.adoc
sort -u medical_whitelist.txt -o medical_whitelist.txt
source .rbk/rbrn_pluml.env
source /Users/bhyslop/projects/rbm_alpha_recipemuster/.rbk/rbgd.env
source Tools/rbk/rbrn_regime.sh
ssh cerebro:*
ssh-add:*
ssh-keygen -lf -
ssh:*
sshpass -V
sudo -n arp -d 169.254.105.91
sudo -n arp-scan --interface=en0 169.254.0.0/16 --retry=2 --timeout=500
sudo -n ipconfig set en0 DHCP
sudo -n lsof -iUDP:67
sudo -n route delete -net 169.254 -interface en1
sudo -n route delete 169.254.105.91
sudo -n true
sudo pkill *
sudo tcpdump *
sudo tee *
sysctl *
system_profiler SPHardwareDataType
tail -10 /Users/bhyslop/projects/rbm_alpha_recipemuster/../_logs_buk/hist-rbw-Dt*.txt
tail -20 /Users/bhyslop/projects/rbm_alpha_recipemuster/../_logs_buk/hist-rbw-Rw*.txt
tail -45 ../logs-buk/same-rbw-lB-sh.txt
tail -6 ../logs-buk/same-rbw-fhv-sh.txt
tailscale ping *
tailscale status:*
tar -tzf /Users/bhyslop/projects/pb_paneboard02/vvk-parcels/vvk-parcel-1011.tar.gz
tar -tzf /Users/bhyslop/projects/pb_paneboard02/vvk-parcels/vvk-parcel-1013.tar.gz
tar -tzf /Users/bhyslop/projects/rbm_alpha_recipemuster/.jjk/parcels/vvk-parcel-1013.tar.gz
tar -xzf /Users/bhyslop/projects/rbm_alpha_recipemuster/.jjk/parcels/vvk-parcel-1013.tar.gz -C /tmp/vvk1013_install
tar -xzf /Users/bhyslop/projects/rbm_alpha_recipemuster/.jjk/parcels/vvk-parcel-1014.tar.gz -C /tmp/p1014
tee /tmp/apcnsa_assay.log
tee /tmp/apcnsa_export.log
tee /tmp/apcnsa_export2.log
tee /tmp/apcnsa_export3.log
tee /tmp/apcnsa_export4.log
tee /tmp/four-mode-run-260425.log
tee /tmp/inscribe-260425-1004.log
tee /tmp/inscribe2-260425-1004.log
tee /tmp/rbtds-service-260425-1004.log
then echo:*
time gzip *
time zstd *
timeout 30 ./tt/rbw-ld.DirectorDivinesLodes.sh
timeout 60 ./tt/rbw-fhv.HygieneCheckVessel.sh rbev-busybox
timeout 60 ./tt/rbw-gq.QuotaBuild.sh
Tools/rbk/rbtd/target/debug/rbtd
Tools/rbk/rbtd/target/debug/rbtd "" calibrant-progressing
Tools/rbk/rbtd/target/debug/rbtd "" calibrant-sentinel
Tools/rbk/rbtd/target/debug/rbtd "rbw-cC rbw-cQ" calibrant-verdicts
Tools/vvk/bin/vvx --help
Tools/vvk/bin/vvx jjx:*
top -l 1 -o cpu -n 20
tput:*
traceroute -n -m 3 -w 1 192.168.1.247
traceroute -n -w 1 -m 4 192.168.1.246
traceroute -n -w 1 -m 4 192.168.1.247
tt/apcw-b.Build.sh
tt/apcw-ba.BatchAssay.sh Tools/apck/test_fixtures
tt/apcw-cb.ContainerBuild.sh *
tt/apcw-ci.ContainerStatus.sh
tt/apcw-cs.ContainerStart.sh
tt/apcw-cx.ContainerStop.sh
tt/apcw-nsa.NeuralStanfordAssay.sh Tools/apck/test_fixtures
tt/apcw-nsi.NeuralStanfordInstall.sh
tt/apcw-nsx.NeuralStanfordExport.sh
tt/apcw-t.Test.sh
tt/buw-h0.HandbookTOP.sh
tt/buw-hj0.HandbookJurisdictionTop.sh
tt/buw-hjw.HandbookJurisdictionWindows.sh
tt/buw-hw.HandbookWindows.sh
tt/buw-jpCL.CaparisonLinux.sh --help
tt/buw-jpCM.CaparisonMacos.sh --help
tt/buw-jpF.Fenestrate.sh
tt/buw-jpGb.GarrisonBash.sh
tt/buw-jpGw.GarrisonWsl.sh bujn-winpc *
tt/buw-jpS.PrivilegedSsh.sh
tt/buw-jpS.PrivilegedSsh.sh bujn-winpc *
tt/buw-jpS.PrivilegedSsh.sh pwd *
tt/buw-jwk.Knock.sh bujn-winpc *
tt/buw-jwk.WorkloadKnock.sh
tt/buw-jwk.WorkloadKnock.sh bujn-winpc *
tt/buw-qsc.QualifyShellCheck.sh 2>&1 | head -80
tt/buw-qsc.QualifyShellCheck.sh 2>&1 | tail -20
tt/buw-rcr.RenderConfigRegime.sh:*
tt/buw-rcv.sh
tt/buw-rcv.ValidateBuc.sh
tt/buw-rcv.ValidateBuc.sh foo *
tt/buw-rcv.ValidateConfigRegime.sh
tt/buw-rcv.ValidateConfigRegime.sh 2>&1 | head -30
tt/buw-rcv.ValidateConfigRegime.sh foo *
tt/buw-rnl.ListNodeRegime.sh
tt/buw-rnl.ListNodeRegime.sh buj *
tt/buw-rnl.ListNodeRegime.sh bujn-winpc *
tt/buw-rnl.ListNodeRegime.sh foo *
tt/buw-rnr.RenderNodeRegime.sh
tt/buw-rnr.RenderNodeRegime.sh bujn-winpc *
tt/buw-rnr.RenderNodeRegime.sh smoke *
tt/buw-rnv.ValidateNodeRegime.sh
tt/buw-rnv.ValidateNodeRegime.sh bujn-winpc *
tt/buw-rnv.ValidateNodeRegime.sh smoke *
tt/buw-rnv.ValidateNodeRegime.sh testbox *
tt/buw-rpl.ListPrivilegeRegime.sh
tt/buw-rpl.ListPrivilegeRegime.sh foo *
tt/buw-rpr.RenderPrivilegeRegime.sh
tt/buw-rpr.RenderPrivilegeRegime.sh smoke-invest *
tt/buw-rpv.ValidatePrivilegeRegime.sh
tt/buw-rpv.ValidatePrivilegeRegime.sh smoke-invest *
tt/buw-rsv.ValidateStationRegime.sh
tt/buw-rsv.ValidateStationRegime.sh 2>&1 | head -30
tt/buw-rsv.ValidateStationRegime.sh foo *
tt/buw-st.BukSelfTest.sh
tt/buw-tt-cbl.CreateTabTargetBatchLogging.sh *
tt/buw-tt-ll.ListLaunchers.sh
tt/buw-tt-ll.ListLaunchers.sh 2>&1 | head -30
tt/jjw-tfP.ProvisionFundusAccounts.localhost.sh
tt/jjw-tfP2.ProvisionPhase2.cerebro.sh ~/.ssh/id_ed25519.pub 2>&1
tt/jjw-tfP2.ProvisionPhase2.cerebro.sh 2>&1
tt/jjw-tfP2.ProvisionPhase2.localhost.sh 2>&1
tt/jjw-tfs.TestFundusScenario.cerebro.sh
tt/jjw-tfs.TestFundusScenario.localhost.sh
tt/jjw-tfs.TestFundusScenario.localhost.sh --help
tt/jjw-tfS.TestFundusSingle.localhost.sh full::bind_send
tt/rbtd-ap.AccessProbe.payor.sh
tt/rbtd-b.Build.sh
tt/rbtd-b.Build.sh 2>&1
tt/rbtd-r.FixtureRun.handbook-render.sh
tt/rbtd-r.FixtureRun.onboarding-sequence.sh *
tt/rbtd-r.FixtureRun.regime-validation.sh
tt/rbtd-r.FixtureRun.srjcl.sh *
tt/rbtd-r.Run.four-mode.sh
tt/rbtd-r.Run.handbook-render.sh
tt/rbtd-r.Run.pristine-lifecycle.sh
tt/rbtd-r.Run.regime-smoke.sh
tt/rbtd-r.Run.regime-validation.sh
tt/rbtd-r.Run.tadmor.sh 2>&1
tt/rbtd-s.FixtureCase.moriah.sh
tt/rbtd-s.FixtureCase.onboarding-sequence.sh
tt/rbtd-s.FixtureCase.onboarding-sequence.sh rbtdro_onboarding_ordain_airgap *
tt/rbtd-s.FixtureCase.regime-validation.sh rbtdrf_rv_rbrv_all_vessels
tt/rbtd-s.FixtureCase.sh
tt/rbtd-s.FixtureCase.sh bogus-fixture *
tt/rbtd-s.FixtureCase.sh enrollment-validation *
tt/rbtd-s.FixtureCase.sh tadmor *
tt/rbtd-s.SingleCase.canonical-establish.sh
tt/rbtd-s.SingleCase.four-mode.sh
tt/rbtd-s.SingleCase.four-mode.sh rbtdrc_fourmode_bind_lifecycle
tt/rbtd-s.SingleCase.four-mode.sh rbtdrc_fourmode_conjure_lifecycle *
tt/rbtd-s.SingleCase.handbook-render.sh rbtdrf_hb_onboard_first_crucible
tt/rbtd-s.SingleCase.pristine-lifecycle.sh
tt/rbtd-s.SingleCase.pristine-lifecycle.sh rbtdrp_depot_lifecycle
tt/rbtd-s.SingleCase.pristine-lifecycle.sh rbtdrp_governor_lifecycle
tt/rbtd-s.SingleCase.pristine-lifecycle.sh rbtdrp_marshal_zero_attestation
tt/rbtd-s.SingleCase.regime-validation.sh rbtdrf_rv_rbrn_all_nameplates
tt/rbtd-s.SingleCase.regime-validation.sh rbtdrf_rv_rbrr_repo
tt/rbtd-s.SingleCase.srjcl.sh rbtdrc_srjcl_websocket_kernel
tt/rbtd-s.SingleCase.tadmor.sh sortie-icmp-exfil-payload:*
tt/rbtd-s.SingleCase.tadmor.sh sortie-net-srcip-spoof:*
tt/rbtd-s.SingleCase.tadmor.sh sortie-ns-capability-escape:*
tt/rbtd-s.SingleCase.tadmor.sh sortie-proto-smuggle-rawsock:*
tt/rbtd-s.TestSuite.crucible.sh
tt/rbtd-s.TestSuite.crucible.sh 2>&1
tt/rbtd-s.TestSuite.dogfight.sh *
tt/rbtd-s.TestSuite.fast.sh
tt/rbtd-s.TestSuite.service.sh
tt/rbtd-t.Test.sh
tt/rbw-acd.CheckDirectorCredential.sh *
tt/rbw-acf.CheckFederatedAccess.sh
tt/rbw-acg.CheckGovernorCredential.sh
tt/rbw-acp.CheckPayorCredential.sh
tt/rbw-acr.CheckRetrieverCredential.sh
tt/rbw-arI.GovernorInvestsRetriever.sh ret *
tt/rbw-cb.Bark.tadmor.sh cat:*
tt/rbw-cb.Bark.tadmor.sh sh *
tt/rbw-cC.Charge.srjcl.sh *
tt/rbw-cC.Charge.srjcl.sh 2>&1
tt/rbw-cC.Charge.tadmor.sh *
tt/rbw-cC.Charge.tadmor.sh 2>&1
tt/rbw-cic.CrucibleIsCharged.sh tadmor *
tt/rbw-cKB.KludgeBottle.sh tadmor:*
tt/rbw-cKB.KludgeBottle.tadmor.sh
tt/rbw-cKS.KludgeSentry.sh tadmor *
tt/rbw-cQ.Quench.srjcl.sh 2>&1
tt/rbw-cQ.Quench.tadmor.sh
tt/rbw-cQ.Quench.tadmor.sh 2>&1
tt/rbw-cw.Writ.tadmor.sh iptables *
tt/rbw-Dc.DirectorChecksConsecrations.sh 2>&1
tt/rbw-DC.DirectorConjuresArk.sh:*
tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-bottle-plantuml
tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-busybox 2>&1
tt/rbw-DC.DirectorCreatesArk.sh rbev-vessels/rbev-busybox 2>&1 | head -80
tt/rbw-DC.DirectorCreatesConsecration.sh rbev-busybox:*
tt/rbw-DC.DirectorCreatesConsecration.sh rbev-sentry-ubuntu-large:*
tt/rbw-DE.DirectorEnshrinesBaseImages.sh rbev-sentry-ubuntu-large:*
tt/rbw-DE.DirectorEnshrinesBaseImages.sh rbev-vessels/rbev-busybox 2>&1
tt/rbw-dE.DirectorEnshrinesVessel.sh *
tt/rbw-DE.DirectorEnshrinesVessel.sh rbev-sentry-ubuntu-large:*
tt/rbw-dE.DirectorEnshrinesVessel.sh rbev-vessels/rbev-busybox
tt/rbw-dI.DirectorInscribesReliquary.sh
tt/rbw-DI.DirectorInscribesReliquary.sh
tt/rbw-DI.DirectorInscribesRubric.sh 2>&1
tt/rbw-dl.PayorListsDepots.sh
tt/rbw-DO.DirectorOrdainsConsecration.sh rbev-bottle-ifrit:*
tt/rbw-dr.DepotRecognosce.sh *
tt/rbw-DS.DirectorSummonsArk.sh rbev-bottle-anthropic-jupyter:*
tt/rbw-DS.DirectorSummonsArk.sh rbev-bottle-plantuml:*
tt/rbw-DS.DirectorSummonsArk.sh rbev-bottle-ubuntu-test:*
tt/rbw-DS.DirectorSummonsArk.sh rbev-sentry-ubuntu-large:*
tt/rbw-dt.TerrierScaffold.sh
tt/rbw-dU.PayorUnmakesDepot.sh depot10041 *
tt/rbw-DV.DirectorVouchesConsecrations.sh
tt/rbw-DV.DirectorVouchesConsecrations.sh rbev-vessels/rbev-busybox 2>&1
tt/rbw-DV.DirectorVouchesConsecrations.sh rbev-vessels/rbev-busybox c260317210019-r260318040308 2>&1
tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh r260503091950 *
tt/rbw-dY.DirectorYokesReliquaryInVessel.sh
tt/rbw-dY.DirectorYokesReliquaryInVessel.sh --help
tt/rbw-dY.DirectorYokesReliquaryInVessel.sh bogus-vessel *
tt/rbw-dY.DirectorYokesReliquaryInVessel.sh r260425082412 *
tt/rbw-dY.DirectorYokesReliquaryInVessel.sh r260426100632 *
tt/rbw-dY.DirectorYokesReliquaryInVessel.sh rbev-sentry-deb-tether *
tt/rbw-dY.DirectorYokesReliquaryInVessel.sh somestamp *
tt/rbw-fk.LocalKludge.sh
tt/rbw-fk.LocalKludge.sh rbev-sentry-deb-tether:*
tt/rbw-fO.DirectorOrdainsHallmark.sh *
tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-bottle-plantuml
tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-vessels/rbev-busybox
tt/rbw-fpf.RetrieverPlumbsFull.sh c260425094751-r260425164754 *
tt/rbw-fs.RetrieverSummonsHallmark.sh rbev-sentry-deb-tether:*
tt/rbw-gO.Onboarding.sh
tt/rbw-gO.Onboarding.sh 2>&1
tt/rbw-go.OnboardMAIN.sh
tt/rbw-gOR.OnboardRetriever.sh
tt/rbw-gPE.PayorEstablish.sh
tt/rbw-gPo.PayorOnboarding.sh 2>&1
tt/rbw-gPR.PayorRefresh.sh
tt/rbw-gq.QuotaBuild.sh
tt/rbw-h0.HandbookTOP.sh
tt/rbw-hw
tt/rbw-hw.HandbookWindows.sh
tt/rbw-HWdc.DockerContextDiscipline.sh
tt/rbw-HWdd.DockerDesktop.sh
tt/rbw-iae.DirectorAuditsEnshrinements.sh
tt/rbw-iah.DirectorAuditsHallmarks.sh
tt/rbw-iar.DirectorAuditsReliquaries.sh
tt/rbw-iJe.DirectorJettisonsEnshrinement.sh
tt/rbw-iJe.DirectorJettisonsEnshrinement.sh "enshrines/busybox-latest-1487d0af5f:busybox-latest-1487d0af5f" --force
tt/rbw-iJr.DirectorJettisonsReliquaryImage.sh "reliquaries/r260425082412/skopeo:r260425082412" --force
tt/rbw-irr.DirectorRekonsReliquary.sh r260425082412 *
tt/rbw-la.DirectorAugursLode.sh vw260610095327 *
tt/rbw-ld.DirectorDivinesLodes.sh
tt/rbw-lI.DirectorImmuresPodvm.sh podvm-wsl *
tt/rbw-LK.LocalKludge.sh 2>&1
tt/rbw-mA.PayorAffiancesManor.sh
tt/rbw-MD.MarshalDuplicate.sh /Users/bhyslop/test-rbk-002
tt/rbw-MG.MarshalGenerate.sh
tt/rbw-MR.MarshalReset.sh
tt/rbw-MZ.MarshalZeroes.sh
tt/rbw-ni.NameplateInfo.sh
tt/rbw-o.ONBOARDING.sh
tt/rbw-o.OnboardingStartHere.sh
tt/rbw-Occ.OnboardingConfigureEnvironment.sh
tt/rbw-Occ.OnboardingCrashCourse.sh
tt/rbw-Ocd.OnboardingCredentialDirector.sh
tt/rbw-Oda.OnboardingDirectorAirgap.sh
tt/rbw-Odb.OnboardingDirectorBind.sh
tt/rbw-Odf.OnboardingDirectorFirstBuild.sh
tt/rbw-Odg.OnboardingDirectorGraft.sh
tt/rbw-Ofc.OnboardingFirstCrucible.sh:*
tt/rbw-Op.OnboardingPayor.sh
tt/rbw-PC.PayorCreatesDepot.sh depot10041:*
tt/rbw-pF.FreeholdProof.sh
tt/rbw-rav.ValidateAuthRegime.sh governor *
tt/rbw-rav.ValidateAuthRegime.sh rbnae_governor
tt/rbw-rdr.RenderDepotRegime.sh
tt/rbw-rfr.RenderFederationRegime.sh
tt/rbw-rfv.ValidateFederationRegime.sh
tt/rbw-Ric.RetrieverInspectsCompact.sh rbev-bottle-plantuml:*
tt/rbw-Ric.RetrieverInspectsCompact.sh rbev-sentry-ubuntu-large:*
tt/rbw-RiF.RetrieverInspectsFull.sh rbev-bottle-plantuml:*
tt/rbw-Rl.RetrieverListsImages.sh:*
tt/rbw-rnr.RenderNameplateRegime.sh nsproto:*
tt/rbw-rnr.RenderNameplateRegime.sh tadmor *
tt/rbw-rov.ValidateOauthRegime.sh
tt/rbw-rrr.RenderRepoRegime.sh
tt/rbw-rrv.ValidateRepoRegime.sh
tt/rbw-Rs.RetrieverSummonsArk.sh rbev-bottle-plantuml:*
tt/rbw-rsr.RenderStationRegime.sh
tt/rbw-rva.DirectorAnointsGraftVessel.sh
tt/rbw-rvl.ListVesselRegime.sh
tt/rbw-rvv.ValidateVessel.sh rbev-sentry-ubuntu-large:*
tt/rbw-rvv.ValidateVesselRegime.sh rbev-graft-demo *
tt/rbw-rvv.ValidateVesselRegime.sh rbev-sentry-ubuntu-large:*
tt/rbw-s.Start.nsproto.sh:*
tt/rbw-s.Start.pluml.sh
tt/rbw-ta.TestAll.sh:*
tt/rbw-tb.Build.sh *
tt/rbw-tc.FixtureCase.sh
tt/rbw-tf.FixtureRun.sh foedus-freehold *
tt/rbw-tf.FixtureRun.sh foedus-lifecycle *
tt/rbw-tf.FixtureRun.sh lode-lifecycle *
tt/rbw-tf.FixtureRun.sh podvm-lifecycle *
tt/rbw-tf.QualifyFast.sh
tt/rbw-tf.TestFixture.access-probe.sh 2>&1
tt/rbw-tf.TestFixture.ark-lifecycle.sh 2>&1
tt/rbw-tf.TestFixture.enrollment-validation.sh 2>&1
tt/rbw-tf.TestFixture.kick-tires.sh 2>&1
tt/rbw-tf.TestFixture.nsproto-security.sh 2>&1
tt/rbw-tf.TestFixture.qualify-all.sh 2>&1
tt/rbw-tf.TestFixture.regime-credentials.sh 2>&1
tt/rbw-tf.TestFixture.regime-smoke.sh 2>&1
tt/rbw-tf.TestFixture.regime-validation.sh 2>&1
tt/rbw-tK.KludgeCycle.tadmor.sh *
tt/rbw-Tk.KludgeCycle.tadmor.sh 2>&1
tt/rbw-tl.Shellcheck.sh
tt/rbw-tn.TestNameplate.nsproto.sh:*
tt/rbw-tn.TestNameplate.pluml.sh:*
tt/rbw-tn.TestNameplate.srjcl.sh:*
tt/rbw-tP.QualifyPristine.sh *
tt/rbw-tq.QualifyFast.sh *
tt/rbw-ts.TestSuite.dogfight.sh
tt/rbw-ts.TestSuite.fast.sh *
tt/rbw-tt.Test.sh *
tt/rbw-z.Stop.nsproto.sh:*
tt/rbw-z.Stop.pluml.sh:*
tt/rbw-z.Stop.srjcl.sh:*
tt/vow-b.Build.sh:*
tt/vow-t.Test.sh *
tt/vow-t.Test.sh 2>&1
tt/vvw-r.RunVVX.sh --help 2>&1
tt/vvw-r.RunVVX.sh jjx_open 2>&1
tt/vvw-r.RunVVX.sh jjx_show '{}' 2>&1
tt/vvw-r.RunVVX.sh jjx:*
unzip -l /tmp/gaz_test.zip
unzip -l names.zip
unzip -o names.zip Names_2010Census.csv
unzip -p /tmp/gaz_test.zip
unzip -p /tmp/gaz2020.zip
vm_stat
wait
wc -l __NEW_LINE_7157110d9076c725__ echo echo '2. BOTTLE_START / BOTTLE_RUN / BOTTLE_SERVICE REFERENCES' echo 'Total bottl_start/run/service:' grep -r "bottle_start\\|bottle_run\\|bottle_service" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.yml --include=*.yaml --include=*.html --include=*.json
wc -l __NEW_LINE_7157110d9076c725__ echo echo '3. FRONTISPIECE REFERENCES' echo 'Total ConnectBottle/ConnectCenser/ConnectSentry/ObserveNetworks:' grep -r "ConnectBottle\\|ConnectCenser\\|ConnectSentry\\|ObserveNetworks" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.yml --include=*.yaml --include=*.html
wc -l __NEW_LINE_7157110d9076c725__ echo echo '5. OP*_ TERM REFERENCES' echo 'Total opbs_/opbr_/opss_ references:' grep -r "opbs_\\|opbr_\\|opss_" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.json
wc -l __NEW_LINE_7157110d9076c725__ echo echo '6. AT_ SERVICE TERM REFERENCES' echo 'Total at_bottle_service/at_censer_container/at_agile_service/at_sessile_service:' grep -r "at_bottle_service\\|at_censer_container\\|at_agile_service\\|at_sessile_service" /Users/bhyslop/projects/rbm_alpha_recipemuster --include=*.sh --include=*.adoc --include=*.md --include=*.json
wc -l /Users/bhyslop/projects/rbm_alpha_recipemuster/GREP*
wc -l /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/jjk/vov_veiled/src/*.rs
wc -l echo echo 'Total matches found:' grep -r "lemma\\|lemmata\\|graven\\|intaglio\\|quoin\\|sprue\\|inlay" /Users/bhyslop/projects/rbm_alpha_recipemuster/Tools/cmk --include=*.adoc
wc -l Tools/jjk/vov_veiled/src/jjrg_gallops.rs Tools/jjk/vov_veiled/src/jjrt_types.rs Tools/jjk/vov_veiled/src/jjri_io.rs
wc:*
which shellcheck:*
which syft:*
while read:*
whois 93.184.216.34
xargs -0 shellcheck --rcfile=Tools/buk/busc_shellcheckrc -S style -f gcc
xargs -I {} basename {}
xargs -I {} grep -l '^export BURD_INTERACTIVE=1$' {}
xargs -I {} sh -c 'echo "=== {} ==="; grep -E "source.*buh_handbook|source.*buym_yelp" "{}"'
xargs -I{} git log -1 --format="%H %s" {}
xargs -I{} sh -c 'test -f "{}rbrn.env" && echo "{}"'
xargs '-I{}' sh -c 'size=$\(git -C /Users/bhyslop/projects/rbm_alpha_recipemuster show "{}:.claude/jjm/jjg_gallops.json" 2>/dev/null | wc -c\); echo "$size {}"'
xargs basename *
xargs cat:*
xargs sed -n '1,30p'
xargs shellcheck --rcfile=Tools/buk/busc_shellcheckrc -S style -f gcc
xxd
xxd ../logs-buk/last.txt
xxd /tmp/direct.bin
xxd /tmp/psutf8.bin
xxd /tmp/utf8.bin
xxd /Users/bhyslop/projects/temp-buk/temp-20260506-135454-80214-889/bujb_wsl_preflight_stdout.txt
```

### Appendix B — beta full Bash list (sorted unique)
```
./tt/buw-qsc.QualifyShellCheck.sh *
./tt/buw-rnv.ValidateNodeRegime.sh bujn-winpc *
./tt/buw-rpv.ValidatePrivilegeRegime.sh
./tt/buw-rpv.ValidatePrivilegeRegime.sh bujn-winpc *
./tt/buw-st.BukSelfTest.sh *
./tt/rbtd-b.Build.sh
./tt/rbtd-r.FixtureRun.moriah.sh *
./tt/rbw-acf.CheckFederatedAccess.sh
./tt/rbw-acp.CheckPayorCredential.sh
./tt/rbw-adI.GovernorInvestsDirector.sh deltest *
./tt/rbw-adr.GovernorRostersDirectors.sh
./tt/rbw-aM.PayorMantlesGovernor.sh *
./tt/rbw-cb.Bark.tadmor.sh getent *
./tt/rbw-cb.Bark.tadmor.sh rbid *
./tt/rbw-cb.Bark.tadmor.sh sh *
./tt/rbw-cC.Charge.tadmor.sh
./tt/rbw-cic.CrucibleIsCharged.sh tadmor *
./tt/rbw-cKS.KludgeSentry.sh tadmor *
./tt/rbw-cQ.Quench.tadmor.sh
./tt/rbw-cw.Writ.tadmor.sh iptables *
./tt/rbw-cw.Writ.tadmor.sh sh *
./tt/rbw-dl.PayorListsDepots.sh *
./tt/rbw-ft.RetrieverTalliesHallmarks.sh
./tt/rbw-gPR.PayorRefresh.sh *
./tt/rbw-iah.DirectorAuditsHallmarks.sh
./tt/rbw-iar.DirectorAuditsReliquaries.sh
./tt/rbw-ld.DirectorDivinesLodes.sh
./tt/rbw-ld.DirectorDivinesLodes.sh r260609104734 *
./tt/rbw-MG.MarshalGenerate.sh *
./tt/rbw-MZ.MarshalZeroes.sh *
./tt/rbw-rdr.RenderDepotRegime.sh
./tt/rbw-rfv.ValidateFederationRegime.sh
./tt/rbw-rpr.RenderPayorRegime.sh
./tt/rbw-tb.Build.sh *
./tt/rbw-tc.FixtureCase.sh enrollment-validation *
./tt/rbw-tc.FixtureCase.sh handbook-render *
./tt/rbw-tc.FixtureCase.sh tadmor *
./tt/rbw-tf.FixtureRun.sh cupel *
./tt/rbw-tf.FixtureRun.sh foundry-path *
./tt/rbw-tf.FixtureRun.sh lode-lifecycle *
./tt/rbw-tf.FixtureRun.sh pluml *
./tt/rbw-tf.FixtureRun.sh regime-poison *
./tt/rbw-tf.FixtureRun.sh reliquary-lifecycle *
./tt/rbw-tf.FixtureRun.sh tadmor *
./tt/rbw-tf.FixtureRun.sh wsl-lifecycle *
./tt/rbw-tl.Shellcheck.sh *
./tt/rbw-tP.QualifyPristine.sh *
./tt/rbw-tq.QualifyFast.sh *
./tt/rbw-ts.TestSuite.blockade.sh *
./tt/rbw-ts.TestSuite.complete.sh *
./tt/rbw-ts.TestSuite.dogfight.sh *
./tt/rbw-ts.TestSuite.fast.sh
./tt/rbw-ts.TestSuite.service.sh *
./tt/rbw-ts.TestSuite.siege.sh *
./tt/rbw-tt.Test.sh
./tt/vow-b.Build.sh *
./tt/vow-t.Test.sh *
/tmp/rbspike/mint_payor.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-adD.GovernorDivestsDirector.sh bhl *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-adI.GovernorInvestsDirector.sh bhl *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-aM.PayorMantlesGovernor.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-arD.GovernorDivestsRetriever.sh bhl *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dl.PayorListsDepots.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-dY.DirectorYokesReliquaryAllVessels.sh r260515151530 *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-fO.DirectorOrdainsHallmark.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-gPR.PayorRefresh.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-iah.DirectorAuditsHallmarks.sh *
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-iar.DirectorAuditsReliquaries.sh
/Users/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-irh.DirectorRekonsHallmark.sh c260515152058-r260515152101 *
/Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/target/debug/rbtd --help
/Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/target/debug/rbtd --keep-going regime-smoke
/Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/target/debug/rbtd regime-smoke *
/Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/target/debug/theurge --help
/Users/bhyslop/projects/rbm_beta_recipemuster/Tools/vvk/bin/vvx --help
/Users/bhyslop/projects/rbm_beta_recipemuster/Tools/vvk/bin/vvx vvx_unlock --help
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/buw-st.BukSelfTest.sh
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbtd-b.Build.sh
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbtd-r.FixtureRun.pluml.sh *
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbtd-r.FixtureRun.srjcl.sh *
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dl.PayorListsDepots.sh
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-fO.DirectorOrdainsHallmark.sh rbev-bottle-ifrit-forge *
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-iar.DirectorAuditsReliquaries.sh *
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-irh.DirectorRekonsHallmark.sh c260603134154-r260603134156 *
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-ld.DirectorDivinesLodes.sh
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-lE.DirectorEnsconcesBole.sh rbev-bottle-ifrit-forge *
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tb.Build.sh *
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tc.FixtureCase.sh *
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tf.FixtureRun.sh lode-lifecycle *
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tl.Shellcheck.sh *
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tP.QualifyPristine.sh *
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tq.QualifyFast.sh
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-ts.TestSuite.fast.sh
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-tt.Test.sh
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/vow-b.Build.sh *
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/vow-t.Test.sh *
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/vvw-r.RunVVX.sh --help
/Users/bhyslop/projects/rbm_beta_recipemuster/tt/vvw-r.RunVVX.sh jjx_scout ark
asciidoctor --version
asciidoctor -o /dev/null /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc
awk -F: '{print $1":"$2}'
awk -F: '$1>=1060 && $1<=1210'
awk -F: '$1>=1653 && $1<=2260'
awk -F: '$1>=2940 && $1<=3010'
awk -F'|' '{ split\($1,a,"_"\); print a[1]"_"a[2]"\\t"$2 }'
awk '{for\(i=1;i<=length\($0\);i++\)print substr\($0,1,i\)}'
awk '{print $5, $9}'
awk '{print $9, $5}'
awk '{print $NF}'
awk '{print}'
awk '/"racing_order": \\[/{f=1} f{print} /\\]/{if\(f\)exit}'
awk '/#\\[cfg\\\(test\\\)\\]/{p=NR} END{print "cfg\(test\) near line "p}' Tools/rbk/rbtd/src/rbtdte_engine.rs
awk '/^buc_die\\\(\\\)/,/^}/' Tools/buk/buc_command.sh
awk '/^buc_reject\\\(\\\)/,/^}/' Tools/buk/buc_command.sh
awk '/^buc_require\\\(\\\)/,/^}/' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/buc_command.sh
awk '/^rbgg_invest_director\\\(\\\)/,/^}/' Tools/rbk/rbgg_governor.sh
awk '/^rbgg_invest_retriever\\\(\\\)/,/^}/' Tools/rbk/rbgg_governor.sh
awk '/^rbgo_get_token_capture/{f=1} f{print NR": "$0} f&&/^}/{exit}' Tools/rbk/rbgo_OAuth.sh
awk '/^rbgp_depot_unmake\\\(\\\)/,/^rbgp_depot_list\\\(\\\)/' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbgp_Payor.sh
awk '/fn rbtdrc_pluml/{f=NR} END{print "first pluml fn at line "f}' Tools/rbk/rbtd/src/rbtdrc_crucible.rs
awk '/fn rbtdro_onboarding_ordain_bind_impl/,/^}$/' Tools/rbk/rbtd/src/rbtdro_onboarding.rs
awk '/name: "siege"/,/\\]/' Tools/rbk/rbtd/src/rbtdrc_crucible.rs
awk '/rbld_banish\\\(\\\)/,/^}/' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbld_Lode.sh
awk 'NR < 2607 || NR > 3033 { print }' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/src/rbtdrc_crucible.rs
awk 'NR<=1012 && /^== /{h=$0; n=NR} END{print n": "h}' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc
awk 'NR<=200 && /^[a-z_]+\\\(\\\) \\{/ {f=$1} NR==196 {print "Line 196 is in function:", f}' Tools/rbk/rbgg_Governor.sh
awk 'NR<=219 && /\\\(\\\)[[:space:]]*\\{?[[:space:]]*$|\\\(\\\)[[:space:]]*\\{/{last=NR": "$0} END{print last}' Tools/rbk/rbgc_Constants.sh
awk 'NR<=571 && /^#/ {h=$0} END{print "heading text: "h}' README.md
awk 'NR<=571 && /^#/ {h=NR": "$0} END{print h}' README.md
awk 'NR>=1 && NR<=260 && /^[a-zA-Z_]+\\\(\\\)|^z?rbgc_[a-zA-Z_]*\\\(\\\)|_kindle.*\\\(\\\)|function /{print NR": "$0}' Tools/rbk/rbgc_Constants.sh
awk 'NR>=1540 && NR<=1600' Tools/rbk/rbtd/src/rbtdrf_fast.rs
awk 'NR>=540 && NR<=575 {print NR": "$0}' README.md
awk 'NR>=600 && NR<=900 && /recipe|hygiene|dockerfile|RECIPE|HYGIENE|DOCKERFILE|expect_code|BAND/' Tools/rbk/rbtd/src/rbtdrf_fast.rs
awk 'NR>=9 && NR<=581' RBS0-SpecTop.adoc
awk 'NR>=9 && NR<=581' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
awk 'NR>=980 && NR<=1145 && /^===? /{print NR": "$0}' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/cmk/vov_veiled/MCM-MetaConceptModel.adoc
awk 'NR>581' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
awk NR>=155 && NR<=275 {printf "%d\\t%s\\n", NR, $0} *
awk NR>=6 && NR<=22 {printf "%d|%s\\n", NR, $0} *
bash -c 'source ../buk/buc_command.sh; source rbfl0_FoundryLedger.sh && declare -F zrbld_cloud_delete_dispatch zrbld_spine_dispatch rbfl_abjure >/dev/null && echo "RBFL OK: ${ZRBLDS_SOURCED:-unset}/${ZRBLDD_SOURCED:-unset} guards set, delete+spine+abjure defined"'
bash -c 'source ../buk/buc_command.sh; source rbld0_Lode.sh && declare -F zrbld_cloud_delete_dispatch zrbld_spine_dispatch rbld_banish >/dev/null && echo "RBLD OK: ${ZRBLDS_SOURCED:-unset}/${ZRBLDD_SOURCED:-unset} guards set, delete+spine+banish defined"'
bash -c 'source Tools/rbk/rbz_zipper.sh && zrbz_kindle && echo "${ZRBZ_COLOPHON_MANIFEST} rbtd-ap"'
bash -n /Users/bhyslop/projects/rbm_beta_recipemuster/__TRACKED_VAR__
bash -n /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/buc_command.sh
bash -n /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/bujb_jurisdiction.sh
bash -n /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbgp_Payor.sh
bash -n Tools/buk/buc_command.sh
bash -n Tools/buk/burs_regime.sh
bash -n Tools/buk/buym_yelp.sh
bash -n Tools/rbk/rbcc_Constants.sh
bash -n Tools/rbk/rbgg_governor.sh
bash -n Tools/rbk/rbgp_payor.sh
bash -n Tools/rbk/rbob_bottle.sh
bash -n Tools/rbk/rbrr_regime.sh
bash -n Tools/rbk/rbz_zipper.sh
bash /tmp/check_drifts.sh
bash /tmp/rbk_dirlog.sh 73a38a23-8901-471f-a20c-6e25fa067908
brew list *
BURE_CONFIRM=skip ./tt/rbtd-r.FixtureRun.moriah.sh *
BURE_CONFIRM=skip ./tt/rbw-cC.Charge.tadmor.sh *
BURE_CONFIRM=skip ./tt/rbw-cKB.KludgeBottle.sh tadmor *
BURE_CONFIRM=skip ./tt/rbw-cKS.KludgeSentry.sh tadmor *
BURE_CONFIRM=skip ./tt/rbw-cQ.Quench.tadmor.sh
BURE_CONFIRM=skip ./tt/rbw-dU.PayorUnmakesDepot.sh prlc-d-pristl100005
BURE_CONFIRM=skip ./tt/rbw-lB.DirectorBanishesLode.sh b260608094131 *
BURE_CONFIRM=skip ./tt/rbw-lB.DirectorBanishesLode.sh r260609104734 *
BURE_CONFIRM=skip ./tt/rbw-lB.DirectorBanishesLode.sh r260609140912
BURE_CONFIRM=skip ./tt/rbw-MZ.MarshalZeroes.sh
BURE_CONFIRM=skip ./tt/rbw-tP.QualifyPristine.sh
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh canc-d-canest2100001
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh canc-d-canest2100002
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh canc-d-canest2100003
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh canc-d-canest2100004
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh canc-d-canest2100005
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh canc-d-canest2100006
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-dU.PayorUnmakesDepot.sh prlc-d-pristl100000
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-fA.DirectorAbjuresHallmark.sh c260603134154-r260603134156 *
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-lB.DirectorBanishesLode.sh r260609140912 *
BURE_CONFIRM=skip /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-MZ.MarshalZeroes.sh *
BURE_CONFIRM=skip tt/rbw-dU.PayorUnmakesDepot.sh *
BURE_CONFIRM=skip tt/rbw-MZ.MarshalZeroes.sh *
BURE_CONFIRM=skip tt/rbw-tP.QualifyPristine.sh *
BURE_TWEAK_NAME=buorb_credless_guard ./tt/rbw-acf.CheckFederatedAccess.sh
BURE_TWEAK_NAME=buorb_credless_guard tt/rbw-acf.CheckFederatedAccess.sh
cargo build *
cat
cat __TRACKED_VAR__/burv-output/invoke-00000/previous/rbf_fact_lode_brand
cd /dev/null
cd /Users/bhyslop/projects/rbm_beta_recipemuster
chmod +x /Users/bhyslop/projects/rbm_beta_recipemuster/tt/rbw-mA.PayorAffiancesManor.sh
chmod +x Tools/rbk/rbld_cli.sh
chmod +x Tools/rbk/rbrf_cli.sh
chmod +x Tools/rbk/rbrf_regime.sh
chmod +x tt/rbw-acf.CheckFederatedAccess.sh
chmod +x tt/rbw-il.DirectorListsRegistry.sh tt/rbw-iw.DirectorWrestsImage.sh tt/rbw-iJ.DirectorJettisonsImage.sh
chmod +x tt/rbw-lC.DirectorConclavesReliquary.sh
chmod +x tt/rbw-lE.DirectorEnsconcesBase.sh tt/rbw-ld.DirectorDivinesLodes.sh tt/rbw-lB.DirectorBanishesLode.sh
chmod +x tt/rbw-lU.DirectorUnderpinsWsl.sh
chmod +x tt/rbw-rfr.RenderFederationRegime.sh tt/rbw-rfv.ValidateFederationRegime.sh
command -v asciidoctor
command -v bundle
command -v gem
command -v ruby
command -v shellcheck
cp /Users/bhyslop/projects/station-files/secrets/director/rbra.env /Users/bhyslop/projects/station-files/secrets/director/rbra.env.bak-20260516-stale100009
curl -o /dev/null -w "http_code=%{http_code} time_total=%{time_total} time_connect=%{time_connect} time_appconnect=%{time_appconnect}\\n" --max-time 15 https://us-central1-docker.pkg.dev/v2/
curl -o /dev/null -w "http_code=%{http_code} time_total=%{time_total}\\n" --max-time 15 https://cloudbuild.googleapis.com/
curl -sS -H 'Authorization: Bearer __CMDSUB_OUTPUT__' https://artifactregistry.googleapis.com/v1/projects/cancbhm-d-canest3bhm100001/locations/us-central1/repositories
curl -sS -H 'Authorization: Bearer __CMDSUB_OUTPUT__' https://cloudresourcemanager.googleapis.com/v1/organizations:search -X POST -H 'Content-Type: application/json' -d '{}'
curl -sS -H 'Authorization: Bearer __CMDSUB_OUTPUT__' https://iam.googleapis.com/v1/locations/global/workforcePools?parent=organizations/247899326218
curl -sS -H 'Authorization: Bearer __TRACKED_VAR__' https://artifactregistry.googleapis.com/v1/projects/cancbhm-d-canest3bhm100001/locations/us-central1/repositories
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v1/organizations/247899326218:getIamPolicy -d '{}'
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v1/organizations/247899326218:setIamPolicy -d @/tmp/rbspike/org_policy_new.json
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v1/projects/cancbhm-d-canest3bhm100001:getIamPolicy -d '{}'
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v1/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/depot_policy_new.json
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v1/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/depot_policy3_new.json
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:getIamPolicy -d '{"options":{"requestedPolicyVersion":3}}'
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:getIamPolicy -d '{}'
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/depot_policy4_new.json
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/depot_policy5_new.json
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/p6_new.json
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/p8_new.json
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://iam.googleapis.com/v1/locations/global/workforcePools?workforcePoolId=spike-office-test -d '{ *
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://iam.googleapis.com/v1/locations/global/workforcePools/spike-office-test/providers?workforcePoolProviderId=spike-entra -d '{ *
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://iam.googleapis.com/v1/projects/cancbhm-d-canest3bhm100001/serviceAccounts -d '{"accountId": "spike-office-test", "serviceAccount": {"displayName": "spike-office-test", "description": "Federation spike test office SA \(heat BZ pace BZAAA\)"}}'
curl -sS -X POST -H 'Authorization: Bearer __CMDSUB_OUTPUT__' -H 'Content-Type: application/json' https://logging.googleapis.com/v2/entries:list -d '{ *
curl -sS -X POST -H 'Authorization: Bearer __TRACKED_VAR__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:getIamPolicy -d '{}'
curl -sS -X POST -H 'Authorization: Bearer __TRACKED_VAR__' -H 'Content-Type: application/json' https://cloudresourcemanager.googleapis.com/v3/projects/cancbhm-d-canest3bhm100001:setIamPolicy -d @/tmp/rbspike/p7_new.json
curl -sS -X POST -H 'Authorization: Bearer __TRACKED_VAR__' -H 'Content-Type: application/json' https://logging.googleapis.com/v2/entries:list -d '{ *
curl -sS -X POST https://sts.googleapis.com/v1/token -H 'Content-Type: application/x-www-form-urlencoded' --data-urlencode grant_type=urn:ietf:params:oauth:grant-type:token-exchange --data-urlencode audience=//iam.googleapis.com/locations/global/workforcePools/spike-office-test/providers/spike-entra --data-urlencode requested_token_type=urn:ietf:params:oauth:token-type:access_token --data-urlencode subject_token_type=urn:ietf:params:oauth:token-type:id_token --data-urlencode subject_token=__CMDSUB_OUTPUT__ --data-urlencode scope=https://www.googleapis.com/auth/cloud-platform
curl -sS 'https://cloud.google.com/iam/docs/federated-identity-supported-services' -o /tmp/rbspike/matrix.html
curl -sSL -A 'Mozilla/5.0' 'https://cloud.google.com/iam/docs/federated-identity-supported-services' -o /tmp/rbspike/matrix.html
docker context *
docker info *
docker network *
docker run *
docker version *
echo "  exit=$?  \(expect 'tok 3600' / 0\)"
echo "  exit=$?  \(expect abc123 / 0\)"
echo "  exit=$?  \(expect empty / nonzero\)"
echo " [exit $?]"
echo "--- exit $? ---"
echo "--- exit: $? ---"
echo "--- exit: $? \(1 = no hits\) ---"
echo "---BUILD EXIT: $?---"
echo "---EXIT $?---"
echo "---EXIT: $?---"
echo "---exit:$?---"
echo "---TQ EXIT: $?---"
echo "---TS EXIT: $?---"
echo "---TT EXIT: $?---"
echo "=== exit $? \(1=no matches, good\) ==="
echo "=== exit: $? ==="
echo "=== EXIT: $? ==="
echo "=== rebase exit: $? ==="
echo "===== merge exit: $? ====="
echo "===EXIT $?==="
echo "ACF_EXIT=$?"
echo "alpha HEAD: $\(git -C /Users/bhyslop/projects/rbm_alpha_recipemuster rev-parse HEAD\)"
echo "BARK_EXIT=$?"
echo "beta HEAD:  $\(git -C /Users/bhyslop/projects/rbm_beta_recipemuster rev-parse HEAD\)"
echo "BUILD_EXIT=$?"
echo "buw-rnv bujn-winpc exit=$?"
echo "buw-rpv bujn-winpc exit=$?"
echo "CHARGE_EXIT=$?"
echo "CREDLESS_EXIT=$?"
echo "curia-tip=$\(git -C /Users/bhyslop/projects/rbm_beta_recipemuster rev-parse HEAD\)"
echo "deleted rbw-iae tabtarget: $?"
echo "DRIFT_EXIT=$? \(expect 1\)"
echo "DRIFT_EXIT=$?"
echo "empty-arg exit=$?"
echo "EXIT_CODE=$?"
echo "EXIT: $?"
echo "exit:$?"
echo "exit=$? \(empty above = gallops unchanged on disk\)"
echo "EXIT=$? \(expect 104 = BUBC_band_credless\)"
echo "EXIT=$? \(expect nonzero fail-loud, no hang\)"
echo "exit=$? \(grep: 1 = zero hits = free\)"
echo "exit=$? \(nonzero/empty = clean\)"
echo "exit=$?"
echo "EXIT=$?"
echo "fast suite exit: $?"
echo "FAST_EXIT=$?"
echo "FAST_SUITE_EXIT=$?"
echo "FIXTURE_EXIT: $?"
echo "FIXTURE_EXIT=$?"
echo "FRESH_EXIT=$?"
echo "HEADLESS_EXIT=$?"
echo "merge-exit=$?"
echo "POISON_EXIT=$?"
echo "push-exit=$?"
echo "QF_EXIT=$?"
echo "QUALIFY_FAST_EXIT=$?"
echo "rc=$? \(empty above = clean\)"
echo "restore build exit=$?"
echo "RFV_EXIT=$?"
echo "SELFTEST_EXIT=$?"
echo "shellcheck exit=$?"
echo "SHELLCHECK_EXIT=$?"
echo "shellcheck-exit=$?"
echo "TEST_EXIT=$?"
echo "theurge build: $?"
echo "theurge tests exit: $?"
echo "theurge tests: $?"
echo "TT_EXIT=$?"
echo "UNIT_EXIT=$?"
find . -name 'CLAUDE*.md' -not -path '*/node_modules/*'
find . -name 'jji_itch.md' -not -path '*/node_modules/*'
find Tools/rbk -name '*.py'
gcloud --version
gcloud artifacts *
gcloud auth *
gcloud builds *
gcloud config *
gcloud org-policies *
gcloud projects *
gcloud version *
gem list *
git *
git log *
git push *
git status *
GIT_EDITOR=true git rebase --continue
GIT_EDITOR=true GIT_SEQUENCE_EDITOR=true git rebase --continue
grep -nF '=======' .claude/jjm/jjg_gallops.json
grep -nF '>>>>>>>' .claude/jjm/jjg_gallops.json
grep -nF *
grep -rliE 'worksite' Memos/ Tools/ .claude/jjm/jji_itch.md
grep -rln -e 'python' -e '#!/usr/bin/env python' Tools/rbk/rbgj* Tools/rbk/rbgjs
grep -rln 'python' Tools/rbk/rbfca_StepAssembly.sh Tools/rbk/rblds_Spine.sh Tools/rbk/rbgj*
grep -rln "RBRV_RELIQUARY\\|rbrv\\.env" Tools/rbk/rbfli_*.sh Tools/rbk/rbfly_*.sh Tools/rbk/rbfca_*.sh
grep *
jq -er '.access_token' /tmp/rbspike/sts_resp.json
jq -r .permissions.allow[]? /Users/bhyslop/projects/rbm_beta_recipemuster/.claude/settings.local.json
jq -r keys[] /Users/bhyslop/projects/rbm_beta_recipemuster/.claude/settings.local.json
ls -1 __TRACKED_VAR__/burv-output/invoke-00000/previous/
ls .claude/jjm/
ls Tools/rbk/vov_veiled/RBSAE*
ls Tools/rbk/vov_veiled/RBSL*
md5 -r /Users/bhyslop/projects/station-files/secrets/director/rbra.env /Users/bhyslop/projects/station-files/secrets/retriever/rbra.env
mkdir -p /tmp/bench-bo
mv ../station-files/secrets/assay/rbra.env ../station-files/secrets/director/rbra.env
mv ../station-files/secrets/assay/rbra.env ../station-files/secrets/governor/rbra.env
mv /tmp/rbtdrc_crucible.rs.new /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/src/rbtdrc_crucible.rs
mv /Users/bhyslop/projects/station-files/secrets/assay/rbra.env /Users/bhyslop/projects/station-files/secrets/director/rbra.env
NO_COLOR=1 TERM=xterm-256color bash /tmp/buc_smoke.sh
osascript -e 'tell application "iTerm2" to get name of current session of current window'
osascript -e 'tell application "iTerm2" to get name of current window'
osascript -e 'tell application "iTerm2" to get tty of current session of current window'
osascript -e 'tell application "iTerm2" to set name of current session of current window to ""'
osascript -e 'tell application "System Events" to get name of first application process whose frontmost is true'
osascript -e 'tell application "System Events" to tell \(first application process whose frontmost is true\) to get name of front window'
perl -0pi -e "s/printf '%s' \\"\\\\\\$\\{z_buym_format\\}\\" >&2/printf '%b' \\"\\\\\\${z_buym_format}\\" >&2/g" Tools/buk/buts/butcym_YelpModule.sh
perl -i -pe 's/\(jjtsc_make_tack\\\([^\)]*?\), None\\\)/$1\)/g' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjtsc_scout.rs
perl -i -pe 's/\(make_valid_tack\\\([^\)]*?\), None\\\)/$1\)/g' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjtg_gallops.rs
perl -i -pe 's/\\bRBTDRK_RBRR_FILE\\b/RBTDGC_RBRR_FILE/g; *
perl -i -pe 's/\\bRBTDRM_ROLE_/RBTDGC_ROLE_/g; s/crate::RBTD_MOORINGS_DIR\\b/crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR/g' __TRACKED_VAR__.rs
perl -i -pe 's/\\bRBTDRP_DOT_DIR\\b/RBTDGC_MOORINGS_DIR/g; *
perl -i -pe 's/RBTDRM_ROLE_/RBTDGC_ROLE_/g; s/rbtdrk_canonical_rbra\\\(&root, "assay"\\\)/rbtdrk_canonical_rbra\(\\&root, RBTDGC_ROLE_ASSAY\)/g' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/src/rbtdrk_canonical.rs
perl -i -pe 's/rbtdrp_canonical_rbra\\\(&root, "assay"\\\)/rbtdrp_canonical_rbra\(\\&root, RBTDGC_ROLE_ASSAY\)/g' rbtdrp_pristine.rs
python3 -c 'import json,sys; d=json.load\(sys.stdin\); print\("client_email:", d.get\("client_email"\)\); print\("project_id:", d.get\("project_id"\)\)'
python3 *
rg -ln 'invest|divest|roster|mantle' Tools/rbk/vov_veiled/RBSDK*.adoc Tools/rbk/vov_veiled/RBSRK*.adoc Tools/rbk/vov_veiled/RBSDD*.adoc Tools/rbk/vov_veiled/RBSRD*.adoc Tools/rbk/vov_veiled/RBSDR*.adoc Tools/rbk/vov_veiled/RBSRL*.adoc Tools/rbk/vov_veiled/RBSGM*.adoc
rg -n --no-heading '"[a-z]{2,6}_[a-z_]+"\\s*:' Tools/rbk/*.sh
rg -n 'rbi_vouch|rblv_|vouch.*json|jq -n' Tools/rbk/rbfv_verify.sh Tools/rbk/rbldb_*.sh
rg -rin 'muniment|terrier' Tools/rbk --include='*.sh' --include='*.adoc' --include='*.rs'
rg -rno '"rb[a-z]{2,3}_[a-z_]+"' Tools/rbk --include='*.sh'
ruby --version
rustc --version
rustup target *
scp -o ConnectTimeout=15 /Users/bhyslop/projects/rbm_beta_recipemuster/../logs-buk/hist-rbw-tP-sh-20260520-172702-4781-219.txt cerebro:~/
scp 'cerebro:.config/gcloud/legacy_credentials/director-bhl@cancbhl-d-canest2bhl100011.iam.gserviceaccount.com/adc.json' /tmp/bench-bo/adc-director.json
scp 'cerebro:projects/rbm_alpha_recipemuster/../logs-buk/hist-rbw-fO-sh-20260514-2*.txt' /tmp/bench-bo/
scp /Users/bhyslop/projects/station-files/secrets/director/rbra.env cerebro:/home/bhyslop/projects/station-files/secrets/director/rbra.env
scp /Users/bhyslop/projects/station-files/secrets/retriever/rbra.env cerebro:/home/bhyslop/projects/station-files/secrets/retriever/rbra.env
scp cerebro:projects/rbm_alpha_recipemuster/../logs-buk/hist-rbw-tP-sh-20260514-203134-1368806-985.txt /tmp/bench-bo/cerebro-20260514.txt
sed -i '' -e 's/A re-divest, or a/A re-defrock, or a/' -e 's/divest of an identity/defrock of an identity/' -e 's/governor freshly re-mantled/governor freshly re-enrobed/' Tools/rbk/vov_veiled/RBSCIG-IamGrantContracts.adoc
sed -i '' -e 's/capital `I` \(invest\) and `D` \(divest\)/capital `E` \(enrobe\) and `F` \(defrock\)/' -e s/invester/enrober/g -e s/divestiture/defrocking/g -e s/reinvests/re-enrobes/g -e s/reinvest/re-enrobe/g -e s/investing/enrobing/g -e s/invested/enrobed/g -e s/invests/enrobes/g -e s/invest/enrobe/g -e s/Investing/Enrobing/g -e s/Invested/Enrobed/g -e s/Invests/Enrobes/g -e s/Invest/Enrobe/g -e s/INVEST/ENROBE/g -e s/divesting/defrocking/g -e s/divested/defrocked/g -e s/divests/defrocks/g -e s/divest/defrock/g -e s/Divesting/Defrocking/g -e s/Divested/Defrocked/g -e s/Divests/Defrocks/g -e s/Divest/Defrock/g -e s/DIVEST/DEFROCK/g -e s/demantle/defrock/g -e s/governor_mantle/governor_enrobe/g -e s/re-mantles/re-enrobes/g -e s/re-mantle/re-enrobe/g -e s/mantle/enrobe/g -e s/Mantles/Enrobes/g -e s/Mantle/Enrobe/g -e s/MANTLE/ENROBE/g Tools/rbk/vov_veiled/RBS0-SpecTop.adoc Tools/rbk/vov_veiled/RBSDK-director_enrobe.adoc Tools/rbk/vov_veiled/RBSRK-retriever_enrobe.adoc Tools/rbk/vov_veiled/RBSDD-director_defrock.adoc Tools/rbk/vov_veiled/RBSRD-retriever_defrock.adoc
sed -i '' -e 's/invest existence preflight/enrobe existence preflight/' -e 's/standing-depot re-invest/standing-depot re-enrobe/' -e s/deletes-then-reinvests/deletes-then-re-enrobes/ -e 's/a fresh invest/a fresh enrobe/' -e 's/governor re-mantle/governor re-enrobe/' -e 's/divest repo/defrock repo/' Tools/rbk/vov_veiled/RBSCIP-IamPropagation.adoc
sed -i '' -e 's/mantles, charters/enrobes, charters/' Tools/rbk/vov_veiled/RBSHR-HorizonRoadmap.adoc
sed -i '' -e 's/one investiture addition/one capability-set addition/' Tools/rbk/vov_veiled/RBSCB-CloudBuildPosture.adoc
sed -i '' -e 's/RBZ_DEFROCK_RETRIEVER /RBZ_DEFROCK_RETRIEVER/' -e 's/RBZ_DEFROCK_DIRECTOR /RBZ_DEFROCK_DIRECTOR/' -e 's/"rbgg_defrock_retriever" /"rbgg_defrock_retriever"/' -e 's/"rbgg_defrock_director" /"rbgg_defrock_director"/' Tools/rbk/rbz_zipper.sh
sed -i '' -e s/rbgg_invest_retriever/rbgg_enrobe_retriever/g -e s/rbgg_invest_director/rbgg_enrobe_director/g -e s/rbgg_divest_retriever/rbgg_defrock_retriever/g -e s/rbgg_divest_director/rbgg_defrock_director/g -e s/zrbgg_divest_role/zrbgg_defrock_role/g -e 's/the invested/the enrobed/g' -e s/invester/enrober/g -e s/Investing/Enrobing/g -e 's/Invest a /Enrobe a /g' -e 's/the invest body/the enrobe body/g' -e 's/declaring invest complete/declaring enrobe complete/g' -e s/post-invest/post-enrobe/g -e s/invest-side/enrobe-side/g -e 's/same-name invest/same-name enrobe/g' -e 's/Divest a /Defrock a /g' -e s/Divesting/Defrocking/g -e s/divested/defrocked/g -e 's/Divest operation/Defrock operation/g' Tools/rbk/rbgg_governor.sh
sed -i '' -e s/rbgp_governor_mantle/rbgp_enrobe_governor/g -e s/RBZ_MANTLE_GOVERNOR/RBZ_ENROBE_GOVERNOR/g -e s/z_mantle_sa/z_govsa/g -e s/z_mantle_delete/z_govsa_delete/g -e 's/Next: mantle Governor/Next: enrobe Governor/g' -e s/demantle/defrock/g Tools/rbk/rbgp_payor.sh
sed -i '' '365,403d' Tools/rbk/rbfln_Inventory.sh
sed -i '' '886,1136d' Tools/rbk/rbfd_FoundryDirectorBuild.sh
sed -n '/rbrn_probate\(\)/,/^}/p' Tools/rbk/rbrn_regime.sh
sed -n '/rbrr_probate\(\)/,/^}/p' Tools/rbk/rbrr_regime.sh
sed -n '1,18p' rbgjm/rbgjm01-mirror-image.sh
sed -n '1,20p' rbgjl/rbgjl04-underpin-capture.sh
sed -n '1,2p' Tools/buk/buc_command.sh
sed -n '1,2p' Tools/buk/buh_handbook.sh
sed -n '1,60p' Tools/rbk/rbrs_cli.sh
sed -n '1,7p' Tools/jjk/vov_veiled/src/jjtq_query.rs
sed -n '110,140p' Tools/rbk/rbrn_cli.sh
sed -n '1373,1378p' Tools/rbk/rbgp_Payor.sh
sed -n '145,180p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbgp_Payor.sh
sed -n '1578,1584p' Tools/rbk/rbgp_Payor.sh
sed -n '1612,1640p' Tools/rbk/rbgp_Payor.sh
sed -n '1725,1742p' Tools/rbk/rbfd_FoundryDirectorBuild.sh
sed -n '1730,1762p' Tools/rbk/rbtd/src/rbtdrf_fast.rs
sed -n '18,45p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/src/rbtdrm_manifest.rs
sed -n '180,200p;470,485p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
sed -n '22,45p' Tools/rbk/rbh0/rbhpb_base.sh
sed -n '2274,2293p' Tools/buk/bujb_jurisdiction.sh
sed -n '2358,2386p' Tools/rbk/rbtd/src/rbtdrc_crucible.rs
sed -n '240,248p' Tools/rbk/rbgo_OAuth.sh
sed -n '243,247p' Tools/rbk/rbgo_OAuth.sh
sed -n '25,33p' Tools/rbk/rbfly_Yoke.sh
sed -n '252,262p' Tools/buk/bud_dispatch.sh
sed -n '255,259p' Tools/buk/bud_dispatch.sh
sed -n '2553,2578p' Tools/rbk/rbtd/src/rbtdrc_crucible.rs
sed -n '27,35p' Tools/vvk/vvb_cli.sh
sed -n '270,300p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
sed -n '280,330p' Tools/rbk/rbtd/src/rbtdri_invocation.rs
sed -n '30,45p' Tools/rbk/rbgjm/rbgjm01-mirror-image.sh
sed -n '309,314p' Tools/apck/apcc_cli.sh
sed -n '352,358p' Tools/rbk/rbgb_Buckets.sh
sed -n '358,367p' Tools/rbk/rbfln_Inventory.sh
sed -n '36,50p;140,148p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/src/rbtdrp_pristine.rs
sed -n '37,40p' Tools/jjk/vov_veiled/src/jjrg_gallops.rs
sed -n '375,400p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
sed -n '380,387p' Tools/buk/bud_dispatch.sh
sed -n '382,386p' Tools/buk/bud_dispatch.sh
sed -n '40,50p' Tools/rbk/rbh0/rbhw0_cli.sh
sed -n '40,60p;100,110p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbtd/src/rbtdrk_canonical.rs
sed -n '40,80p' Tools/rbk/rbgje/rbgje01-enshrine-copy.sh
sed -n '41,46p' Tools/buk/bux_cli.sh
sed -n '459,462p' Tools/rbk/rbfc_FoundryCore.sh
sed -n '50,80p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbcc_Constants.sh
sed -n '525,610p' Tools/rbk/rbgg_Governor.sh
sed -n '54,56p' Tools/rbk/vov_veiled/RBSDY-director_yoke.adoc
sed -n '60,70p' Tools/rbk/vov_veiled/rbv_cli.sh
sed -n '600,636p' Tools/rbk/rbfl_FoundryLedger.sh
sed -n '668,702p' Tools/rbk/rbfl_FoundryLedger.sh
sed -n '70,130p' Tools/rbk/vov_veiled/RBS0-SpecTop.adoc
sed -n '860,870p' Tools/rbk/rbfd_FoundryDirectorBuild.sh
sed -n '882,895p' Tools/rbk/rbfd_FoundryDirectorBuild.sh
sed -n '90,130p' /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/cmk/vov_veiled/ACG-AllocationCodingGuide.md
sed -n '95,135p' rbldw_Underpin.sh
sed -n '96p' Tools/rbk/rbfly_Yoke.sh
sed -n 1,2p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/buc_command.sh
sed -n 1,2p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/buh_handbook.sh
sed -n 1,35p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjtsc_scout.rs
sed -n 105,133p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjrsc_scout.rs
sed -n 1070,1095p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjrm_mcp.rs
sed -n 120,135p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjrsc_scout.rs
sed -n 55,75p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjri_io.rs
sed -n 5p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjtq_query.rs
sed -n 620,635p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjro_ops.rs
sed -n 620,640p /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/jjk/vov_veiled/src/jjro_ops.rs
sed 's/-.*$//'
sed 's/\\x1b\\[[0-9;]*m//g'
shellcheck --rcfile=Tools/buk/busc_shellcheckrc Tools/rbk/rbob_bottle.sh
shellcheck --version
shellcheck -s bash __TRACKED_VAR__/Tools/buk/buh_handbook.sh
shellcheck -s bash /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/buc_command.sh
shellcheck -s bash Tools/buk/buc_command.sh
shellcheck -S error /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/buk/bujb_jurisdiction.sh
shellcheck -S error /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbfd_FoundryDirectorBuild.sh
shellcheck -S error /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbfl_FoundryLedger.sh /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbcc_Constants.sh
shellcheck -S error /Users/bhyslop/projects/rbm_beta_recipemuster/Tools/rbk/rbgp_Payor.sh
shellcheck -S error Tools/buk/burs_regime.sh Tools/rbk/rbrr_regime.sh
shellcheck -S error Tools/rbk/rbgg_Governor.sh
shellcheck -S error Tools/rbk/rbgp_Payor.sh
shellcheck -S error Tools/rbk/rbob_bottle.sh
shellcheck -S warning Tools/buk/bujb_jurisdiction.sh
shellcheck -x Tools/rbk/rbgo_OAuth.sh Tools/rbk/rbndb_base.sh Tools/rbk/rbgc_Constants.sh
sort -t: -k2 -n -r
sort -t: -k2 -rn
ssh -v cygwin@rocket exit
ssh *
ssh brad@rocket *
ssh cerebro *
ssh cygwin@rocket *
ssh wsl@rocket *
tee /tmp/rbw-tP-attempt2.log
TERM=dumb bash /tmp/buc_smoke.sh
TERM=xterm-256color ./tt/rbw-gPE.PayorEstablish.sh
TERM=xterm-256color bash /tmp/buc_smoke.sh
test -d .git/rebase-merge -o -d .git/rebase-apply
tt/buw-rsv.ValidateStationRegime.sh
tt/buw-st.BukSelfTest.sh *
tt/rbtd-b.Build.sh
tt/rbtd-r.FixtureRun.enrollment-validation.sh
tt/rbtd-r.FixtureRun.hallmark-lifecycle.sh
tt/rbtd-r.FixtureRun.regime-smoke.sh
tt/rbtd-s.FixtureCase.regime-smoke.sh
tt/rbtd-s.FixtureCase.regime-smoke.sh rbtdrf_rs_unmake_empty_arg_refusal
tt/rbtd-s.TestSuite.fast.sh
tt/rbtd-t.Test.sh
tt/rbw-acf.CheckFederatedAccess.sh
tt/rbw-dl.PayorListsDepots.sh *
tt/rbw-dU.PayorUnmakesDepot.sh
tt/rbw-gPR.PayorRefresh.sh *
tt/rbw-il.DirectorListsRegistry.sh
tt/rbw-MG.MarshalGenerate.sh
tt/rbw-MZ.MarshalZeroes.sh *
tt/rbw-rfv.ValidateFederationRegime.sh
tt/rbw-rrv.ValidateRepoRegime.sh
tt/rbw-tb.Build.sh *
tt/rbw-tf.FixtureRun.sh regime-poison *
tt/rbw-tl.Shellcheck.sh
tt/rbw-tP.QualifyPristine.sh *
tt/rbw-ts.TestSuite.fast.sh *
tt/rbw-tt.Test.sh
tt/vow-b.Build.sh
tt/vow-t.Test.sh
umask 077
wc -l .claude/jjm/jjg_gallops.json
xargs -n1 basename
xargs cat
xargs ls -la
xargs ls -lt
```

### Appendix C — cerebro-alpha full Bash list (sorted unique)
```
./Tools/vvk/bin/vvx --version
./tt/rbw-tb.Build.sh
./tt/rbw-tf.FixtureRun.sh canonical-invest *
./tt/rbw-tf.FixtureRun.sh pristine-lifecycle *
./tt/rbw-tl.Shellcheck.sh *
./tt/rbw-ts.TestSuite.dogfight.sh *
./tt/rbw-tt.Test.sh
./tt/vow-b.Build.sh *
./tt/vow-t.Test.sh *
[ -f "/home/bhyslop/projects/rbm_alpha_recipemuster/$p" ]
/home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-adI.GovernorInvestsDirector.sh canest-dir *
/home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tb.Build.sh
/home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh canonical-invest *
/home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tf.FixtureRun.sh lode-lifecycle *
/home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tl.Shellcheck.sh *
/home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.crucible.sh
/home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.dogfight.sh *
/home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.fast.sh *
/home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-ts.TestSuite.skirmish.sh *
/home/bhyslop/projects/rbm_alpha_recipemuster/tt/rbw-tt.Test.sh
/home/bhyslop/projects/rbm_alpha_recipemuster/tt/vow-b.Build.sh *
/usr/bin/gcloud --version
awk '/^buc_die\\\(\\\)/{f=1} f{print} f&&/^}/{exit}' Tools/buk/buc_command.sh
bash -n /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgi_IAM.sh
bash /tmp/clusters.sh
bash /tmp/count_roles.sh
bash /tmp/detailed_analysis.sh
bash /tmp/validation.sh
BURE_CONFIRM=skip tt/rbw-lB.DirectorBanishesLode.sh vn
BURE_CONFIRM=skip tt/rbw-lB.DirectorBanishesLode.sh vn260608213343
BURE_CONFIRM=skip tt/rbw-lB.DirectorBanishesLode.sh vw260608213906
cat
cd *
cd /home/bhyslop/projects/rbm_alpha_recipemuster
claude mcp *
command -v docker
command -v gcloud
command -v shellcheck
docker info *
docker rmi *
echo "--- exit: $? \(1 = no matches = clean\) ---"
echo "---push exit: $?---"
echo "---rebase exit: $?---"
echo "=== BUILD EXIT: $? ==="
echo "=== exit: $? \(empty diff above = both files now match pre-pace\) ==="
echo "=== exit: $? ==="
echo "=== EXIT: $? ==="
echo "=== TEST EXIT: $? ==="
echo "BANISH_EXIT=$?"
echo "DIR_PROBE_EXIT=$?"
echo "DIVINE_EXIT=$?"
echo "GOV_PROBE_EXIT=$?"
echo "HEAD: $\(git -C /home/bhyslop/projects/rbm_alpha_recipemuster rev-parse --short HEAD\)"
echo "INVEST_EXIT=$?"
echo "MANTLE_EXIT=$?"
echo "PROBE_EXIT=$?"
echo "ROSTER_EXIT=$?"
gcloud --version
git *
GIT_EDITOR=true git rebase --continue
grep -rn "case.*director\\|case.*retriever\\|case.*governor\\|case.*\\\\\\$.*role" /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk --include="*.sh"
grep -rnE "RBGD_API_SERVICE_ACCOUNTS=|RBGC_PATH_KEYS=|RBGC_API_ROOT_IAM=|RBGC_IAM_V1=|ZRBGG_INFIX_KEY=|ZRBGG_INFIX_LIST_KEYS=|ZRBGG_INFIX_DELETE=|ZRBGG_PREFIX=" Tools/rbk/
ls -1 /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rb?c_Constants.sh
mv ../station-files/secrets/assay/rbra.env ../station-files/secrets/director/rbra.env
read -r p
rg -n --glob !.git --glob !Memos/** 'rbk-claude-acronyms|rbk-claude-tabtarget-context|rbk-claude-theurge-ifrit-context|vok-claude-context|apck-claude-context|cmk-claude-context|cmk-rules-of-engagement-detail|claude-cmk-rules-of-engagement|buk-claude-context' /home/bhyslop/projects/rbm_alpha_recipemuster
rg -n -i 'skirmish' /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtd/src/*.rs
rg -n 'Tools/' -g '!target' -g '!*.d' -g '!*.md' -g '!*.adoc' -g '!Memos/**' --count-matches
rg -rn -i 'context_file|claude-rbk|claude-vok|claude-cmk|claude-buk|claude-apck' /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbcc_Constants.sh /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgc_Constants.sh
rg -rn -i 'lode.?lifecycle|FIXTURE_LODE|ensconce|divine|banish' /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbtd/src/
sed -E 's/.*\\.\([a-zA-Z0-9]+\)$/\\1/'
shellcheck --version
shellcheck -S warning /home/bhyslop/projects/rbm_alpha_recipemuster/Tools/rbk/rbgi_IAM.sh
sort -t: -k2 -rn
sort -t' ' -k2 -n -u
ssh *
tt/rbw-acd.CheckDirectorCredential.sh
tt/rbw-acg.CheckGovernorCredential.sh
tt/rbw-adI.GovernorInvestsDirector.sh canest-dir *
tt/rbw-adr.GovernorRostersDirectors.sh
tt/rbw-aM.PayorMantlesGovernor.sh
tt/rbw-ld.DirectorDivinesLodes.sh
tt/vow-b.Build.sh *
tt/vow-t.Test.sh *
```
