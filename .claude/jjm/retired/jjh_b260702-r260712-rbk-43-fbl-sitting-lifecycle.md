# Heat Trophy: rbk-43-fbl-sitting-lifecycle

**Firemark:** ₣Bq
**Created:** 260702
**Retired:** 260712
**Status:** retired

## Paddock

## Charter — the sitting-runway lifecycle

This heat lands the federated-credential lifecycle feature that the ₣Bf credential-fault disposition study surfaced (260702):
a preflight runway gate on the sitting, a force-fresh renew act, and the mid-flight mantle re-mint the human-present premise already promises.
₣Bf is the parent stream; its disposition pace was restrung in here whole and leads the heat —
it records the study's retire-with-finding determinations and clears the vestigial consts before the build paces run.

## Model — two clocks under one credential

The sitting (the cached federated token, pool+provider-keyed, bounded by the ~12h workforce session window) already carries a stored expiry and a skew-gated read — the deadline exists; nothing here invents one.
The mantle token minted beneath it (the don) self-expires at the ~60-minute generateAccessToken default ceiling and is deliberately never cached.
The failure: the one long-lived consumer — the shared build-completion poll, through which every long cloud build funnels — mints one mantle token and holds it across a worst-case ~95-minute budget that outlives the mantle ceiling while the sitting stays perfectly live; the poll's blind consecutive-failure counter then kills a healthy build.
RBS0's human-present premise already promises the cure ("a long run re-mints mantle tokens mid-flight from the cached federated token while the sitting lives"); the poll violates that promise today.

## Shape — disposition first, then four build parts in dependency order

The restrung disposition pace leads (findings recorded, dead consts cleared);
then contract first (RBS0 gains the proactive runway floor, the renew act, and the census-minted operator verb);
then the BUK band widening (the precision reject band is FULL, so no new gate can take a code until the band grows);
then the gate + renew together (the rejection names the remedy, so they land as one);
then the poll re-mint.

## Cinched (operator rulings 260702)

- Renew only — NO release/clear verb, no sitting-delete primitive anywhere.
  Force-mode avow skipping the reuse branch, riding the existing atomic sitting overwrite, is the whole renew mechanism.
- The gate lives in avow's sitting-reuse branch — automatic for every federated command, including the five direct avow+don side doors; never a per-command preflight step.
  It fires on the reuse path only: a fresh sitting has full runway by construction.
- The payor OAuth path is out of scope — it never avows, re-mints its access token per call, and has its own refresh verb.
- Parameterize the seam, default the value: the runway check takes required-runway as an argument defaulting to the blanket floor (~2h, a kindled constant beside the existing sitting-skew knob).
  NO per-operation bounds are populated until one earns its existence (load-bearing-complexity test).
- The floor guarantees the composition: at least 2h of runway at gate time means the sitting outlives any build (~95-minute worst case), so the poll's mid-flight re-mint reads a live sitting by construction and never prompts.
- Census line — the operator verb is ELECTED (260704): **novate**, the force-fresh renewal act
  (law: novation — extinguishing an obligation by replacing it with a new one; mechanism-exact for the atomic sitting overwrite).
  Ashlar, low-traffic (the gate's rejection text + the renew tabtarget), bondstone-adjacent;
  grep-clean at mint; rbtf_novate seated in RBS0 beside avow/sitting.
  Census verdicts: "renew" disqualified (trodden ambient prose + near-neighbor collision with the payor refresh estate, RBSPR/RBSAO);
  "reavow" runner-up (mechanism-honest but derivative — names the ceremony, not the fresh sitting it buys);
  "prorogate" failed the cold probe (Westminster prorogue reads as suspend/end — backwards).
- Band widening is blessed: widen the definition, re-pin the self-test sentinel, update its fixture; allocation rules and existing codes unchanged.

## Done when

The study findings are recorded where the surviving code lives and the vestigial consts are gone;
a short-runway sitting reuse rejects with a named band code advising renew;
the renew tabtarget restores a full-life sitting on both mechanisms;
the shared poll re-mints mid-flight so a conjure-length build no longer dies of mantle expiry;
and the RBS0 contracts say all of this before the code does.

## Sources

The ₣Bf mount study (260702) mapped every surface; durable anchors:
avow's sitting-reuse branch, the sitting read/write helpers, and the don's single-attempt design comment in rba_auth.sh;
the one long-lived consumer zrbfc_wait_build_completion in rbfcb_host.sh (director conjure/mirror and the whole Lode capture spine all funnel through it — one fix, not several);
the full band census in bubc_constants.sh (16/16 allocated, self-test sentinel pinning the top, BUTT fixture asserting it);
the spec homes in RBS0-SpecTop.adoc — the sitting and avow quoins and the human-present premise (its re-mint promise, reactive fail-at-lapse only at study time, now extended by the contract pace: runway floor on the sitting, novate quoin beside avow).
Wrong homes ruled out: RBSFA is the programmatic, sittingless sibling and does not own this contract; the in-pool preflight quoin is an unrelated Cloud-Build step check.
The keyfile-era memo trail lives with the leading disposition pace's docket; this heat's build paces do not re-litigate it.

## Paces

### federation-credential-fault-disposition (₢BqAAE) [complete]

**[260704-0841] complete**

The credential-fault DISPOSITION (restrung from ₣Bf, 260702, where the study ran at mount) —
record the two determinations against the settled federation auth path and clean the vestigial consts.
The lifecycle feature the study surfaced became this heat's four build paces; no build-poll code changes in this pace.

The determinations to record, each where the surviving code lives:

Determination A (flap tolerance) — retire with finding.
The memo's trigger (per-run director-SA re-invest) is demolished: mantle SAs are standing, levy-created, never re-invested per run.
The fresh-levy analogue is unobserved post-federation, and first-creation settling surfaces as the propagation family, not the runtime 401.
No tolerance added — the gauntlet-clause posture: await real evidence.

Determination B (terminal fail-fast) — retire with finding.
The auth legs and cold probes already discriminate correctly:
the device-flow poll is dual-gated (transient curl band + OAuth pending states, wall-clock bounded);
programmatic acquire, STS, and the don are single-attempt fail-fast;
the don's 403 rides the admission band unretried by design;
the probes die on any 401/403 and retry only 5xx.
The blind-403 propagation waits (the IAM grant sites and the capabilities GAR loop) are post-grant by construction and cinch-kept —
record WHY at their constants' home, so a future reader does not "optimize" the deliberate budget away.

The build poll (zrbfc_wait_build_completion, rbfcb_host.sh) — record as finding here, fixed by this heat's re-mint pace.
Its single non-refreshed mantle token plus blind consecutive-failure counter is real,
but it is a lifecycle contradiction of the human-present premise's mid-flight re-mint promise (mantle ceiling vs conjure budget),
not a fault-classification gap.

Code actions in this pace:
remove the vestigial RBGC_SA_KEY_CONSUMER_RETRY_* trio from rbgc_constants.sh (zero consumers repo-wide; the mechanism they tuned was demolished with the keyfile estate),
and prune the single stale RBSCIP sentence naming them —
leaving the broader keyfile-tier re-anchor batch to its existing FABLE-REANCHOR flag, not swallowing it here.

## Source
Memos/memo-20260527-account-state-invalid-reinvest-flap.md — the original 401 signature and two-host evidence; historical record now, its trigger and touchpoint both demolished.

## Done when
Both determinations recorded where the surviving code lives (form of the record — comment, spec note — is mount judgment);
the dead consts and the orphaned spec sentence are gone;
the build-poll finding names this heat's re-mint pace as the fix home;
shellcheck and reveille green.

## Cinched
The terminal-vs-transient disambiguator is call context, never the response body alone.
Do not reintroduce the keyfile SA-propagation loop; keep genuine post-mint propagation tolerance wherever it survives.
No build-poll code changes in this pace — that surface belongs to the re-mint pace at the heat's tail.

## Character
Recording plus small deletions; the judgment is already made, this pace writes it down.

## Provenance
Drafted from ₣Bi lineage (originally ₣BB-era against the keyfile estate); the terminal-fail-fast sibling folded in 260702;
narrowed 260702 after the mount study and restrung from ₣Bf into this heat, the build work cut as the sibling paces.

### sitting-runway-contract (₢BqAAA) [complete]

**[260704-0858] complete**

Extend the RBS0 contracts ahead of any code (contract-first is a hard predecessor here, per parent-heat doctrine):
the sitting quoin gains the proactive runway floor (its contract today speaks only of lapse);
the human-present premise's mid-flight re-mint promise extends from reactive fail-at-lapse to the preflight gate + renew advisory;
a new quoin beside avow homes the force-fresh renew act.
Mint the operator verb via the federation vocabulary census with full Lapidary gates —
"renew" is the working name and likely trodden;
the word is ashlar (it rides the gate's rejection message) and bondstone-adjacent (the operator reasons through it when the gate turns them away).

## Done when
The three RBS0 definition sites carry the floor, the extended promise, and the renew act;
the verb is minted, grep-clean, and recorded in the paddock census line;
no code in this pace.

## Cinched
Renew-only (no release verb) and the reuse-branch gate placement are settled — spec them as such, do not reopen.
RBSFA does not own this contract (programmatic, sittingless); the interactive sitting surface in RBS0 does.

## Character
Spec authoring plus one word mint; judgment on register, mechanical on structure.

### bubc-band-widen (₢BqAAB) [complete]

**[260704-0910] complete**

Widen the BUK precision reject band: all sixteen codes are allocated and the self-test sentinel pins the current top,
its BUTT fixture asserting full-width propagation.
Widen the band definition against BCG's band doctrine (choose the new width there),
re-pin the sentinel at the new top, update the self-test expectation;
allocation rules and every existing code unchanged.
No new RBK band constant in this pace — the consuming gate pace appends its own code (the escheat precedent: the const lands with its gate).

## Done when
The band has free codes; buw-st self-test green; buc_reject's bounds enforce the new width.

## Cinched
Widening over reuse: sharing an existing code is rule-barred (the runway gate co-occurs with its neighbors on the avow-to-don spawn path).

## Character
Small, mechanical, BUK-pure.

### runway-gate-and-renew (₢BqAAC) [complete]

**[260704-1054] complete**

Build the preflight runway gate and the renew verb together — the rejection names the remedy, so landing one without the other ships a dead end.
Gate: in avow's sitting-reuse branch only, computing remaining runway from the stored expiry;
below the floor, reject in a fresh band code (the prior pace widened the band) advising the census-minted renew verb.
Threshold: a kindled constant beside the existing sitting-skew knob, default two hours;
the check takes required-runway as a parameter defaulting to the floor — the seam is parameterized, no per-operation bounds populated (cinched at the paddock).
Renew: avow force-mode skipping the reuse branch, riding the existing atomic sitting overwrite; no delete primitive.
Surface: a new access-family colophon fronting a thin CLI arm, zipper enrollment, regenerated consts and tabtarget context riding the build.
Coverage is automatic at the chokepoint: every accessor site and all five direct avow+don side doors pass through avow; the payor path never does.

## Done when
A short-runway reuse rejects with the named band code and the renew advisory;
the renew tabtarget restores a full-life sitting (device-flow interactive, RFC 7523 programmatic);
the deliberate rejection asserts its named code per the precision-band doctrine;
the credential-surface test tier green.

## Cinched
Gate fires on the reuse path only; renew-only, no release; payor out of scope.

## Character
Focused bash plus enrollment mechanics; the design is settled, the negative-test seam wants mount-time judgment.

### sitting-liveness-probe (₢BqAAH) [complete]

**[260704-1117] complete**

Mint and build the read-only sitting probe:
an access-family tabtarget reporting whether a sitting is live and how much runway remains —
never opening one, never prompting, never mutating, reading the cache alone (no network).
The verb word wants a Lapidary census at mount
(descry, rehearse, and canvass are taken by their own read verbs; grep gate + cold probe per MCM);
the colophon is a lowercase rbw-a member per the group's UPPER-mutates convention.
Shape: a thin arm over the sitting read/runway helpers in rba_auth.sh
(the runway capture landed with the gate pace);
an absent or lapsed sitting is a reported verdict, not a rejection —
the descry exit-0 verdict precedent (RBSFD), with machine-branchable output (fact or stdout, mount-time call).
Adopt it for fail-fast:
the theurge gate arc (access-probe fixture) consults the probe
and fails immediately with the open-a-sitting instruction when no sitting is live,
replacing the blind device-window poll the operator hit (260704) —
honoring the ruling that no interactive act may live inside a fixture
(the sitting-novate fixture was retired under it; novate's positive proof is the operator ceremony, rbw-aN from a terminal).
Consider echoing the probe colophon in the runway-gate rejection text if it sharpens the advisory.

## Done when
The probe reports live-with-runway or absent from the cache alone, without prompting or network;
the gate arc fails fast with instructions on a dead sitting instead of polling the device window;
zipper enrollment plus regenerated consts and tabtarget context ride the build;
reveille and the access-probe fixture green.

## Cinched
Read-only — never opens, never prompts; access family, lowercase colophon; absent is a verdict, not a band rejection;
no interactive act inside any theurge fixture.

## Character
Small focused bash + enrollment mechanics riding the gate pace's helpers; the verb mint wants census care.

### avowal-code-clipboard (₢BqAAI) [complete]

**[260704-1152] complete**

Copy the device-flow user code to the operator clipboard at avowal-prompt emission, alongside the existing yawp display.
Security determination recorded (260704 session): the user code is display-safe by design
(RBS0 rbtf_avow — possession grants nothing without the human's own IdP sign-in; a substituted sign-in cannot pass admission),
and any clipboard-reading local process could already read the tabtarget logs or the sitting cache itself;
the small residual is clipboard sync/history spreading the single-use ~15-minute code to synced devices — accepted, note it at the site.
Custody rule: ONLY the user code ever touches the clipboard — never the device code, the federated token, or a mantle token.
Best-effort fail-soft: clipboard copy is a convenience, never load-bearing —
a missing tool must not break avowal; announce a successful copy on the progress stream (it also clobbers the prior clipboard).
Platform-variant dependency per BCG Command Dependency Discipline:
pbcopy (macOS), clip.exe (Windows), the X/Wayland pair on Linux — probe-and-skip, declared per the guidance.
Site: the prompt emission in rba_auth.sh's device-flow leg; both the avow-miss and novate paths reach it.

## Done when
An interactive avowal (avow miss or novate) lands the user code on the clipboard where a clipboard tool exists, announced on the progress stream;
absent tooling degrades to display-only without error;
no other credential material ever reaches the clipboard;
reveille green.

## Cinched
User code only; fail-soft; never load-bearing.

## Character
Small platform-aware bash at one site; the clipboard-command survey is the only research.

### poll-mantle-remint (₢BqAAD) [complete]

**[260704-1237] complete**

Honor the human-present premise's mid-flight re-mint promise at the one long-lived mantle consumer:
the shared build-completion poll in rbfcb_host.sh, through which every long cloud build funnels (director conjure and mirror, the whole Lode capture spine).
The initial mint stays the full accessor; inside the loop, re-don directly on a cadence that beats the ~60-minute mantle ceiling
(poll count times the fixed interval is the available clock) —
the don alone, never the full accessor, so a lapsed sitting fails clean instead of re-entering avow mid-loop.
The runway gate (prior pace) guarantees the sitting outlives the build, so the re-mint read is live by construction.
Decide at mount how a mid-poll don rejection surfaces (the don's admission band versus the poll's die),
and design the test seam — the cadence must be exercisable without an hour-long build (a tweak shortening it, per the BURE seam pattern).

## Done when
A conjure-length build survives mantle expiry (re-mint observed on cadence);
a lapsed-sitting mid-poll fails loud with the avow advisory;
the test seam exercises the cadence in a short build.

## Cinched
The re-mint composes with, and does not replace, the poll's failure counter;
the blind-counter discrimination question was retired with findings by the ₣Bf disposition pace — do not reopen it here.

## Character
Small code change carrying a real test-design question.

### suite-credential-preamble (₢BqAAF) [complete]

**[260704-1308] complete**

Restore the credential-readiness leader the release suites lost with the keyfile estate demolition:
skirmish, dogfight, and blockade led with a keyfile credential-heal fixture (the re-enrobe dance) that was removed whole;
they currently carry no credential-readiness step and were left credential-incomplete — a deficit surfaces mid-suite instead of up front.
The federation replacement is not machinery of its own (parent-heat lean, carried in):
a standing-freehold readiness check — confirm a live sitting with runway and the donnable mantles — one more consumer of this heat's gate and renew.
With the runway gate live, any suite's first cloud call auto-gates;
this pace adds the UP-FRONT leader so a credential deficit fails in seconds with the renew advisory, never minutes into a build.
Design at mount: whether the leader is a fixture invoking the gate's check surface or the existing probe verbs;
suite membership edits ride the RBTDRA_SUITES literal registry.

## Done when
The three suites lead with a credential-readiness step that passes on a healthy freehold sitting
and rejects fast with the renew advisory on a short or absent one.

## Cinched
Consumer of the gate and renew — mount after they land (heat order already says so).
No re-enrobe dance, no keyfile shapes.

## Character
Small test-infrastructure composition; the design question is the leader's form, not its need.

### curl-band-overlap-doctrine (₢BqAAG) [complete]

**[260705-0924] complete**

Built and notched (260704), test-first at operator direction:
the curl-band overlap doctrine at both homes (BCG "Precision Exit-Code Band" placement paragraph + the bubc_constants.sh tinder placement comment);
the positive-form curl-containment scan in the theurge conformance fixture
(canon: a curl invocation leads its line, terminator ends byte-exact `|| z_curl_status=$?`, captured status reappears within a 24-line window;
`command -v curl` sole carve-out; rbgj cloud-step trees and ABANDONED-github exempt; unscannable-quoting-plus-curl is itself a violation),
with three hermetic self-tests;
and the 28-site sweep the red run demanded —
19 bare curl-into-buc_die chains repaired to capture-and-classify,
2 rbgp_payor pipe-forms restructured onto a process substitution (request secrets stay off disk),
5 rba_auth z_status captures renamed to the forced name,
2 rbxk_keycloak capture-assignments restructured via a new ZRBXK_HTTP_CODE temp.
Green so far: conformance 9/9, reveille 122/122, shellcheck 229 clean.

## Remaining
Verification only — rerun the picket suite to green (foundry/credential change tier).
The 260704 picket run was stopped by the operator mid lode-lifecycle after 138 green verdicts,
including hallmark-lifecycle, the live proof of the swept rbfd/rbfcg/rbfv paths;
the kill may have left a stray test Lode in GAR — divine and banish it if present.
Then wrap.

## Done when
Both doctrine homes state the overlap and the containment rule (done);
the conformance scan enforces the canonical form repo-wide (done);
picket green post-sweep.

## Cinched
Doctrine over re-base (zero-sum window under the 124 ceiling — see the heat paddock).
Byte-exact forced capture name `z_curl_status`, so theurge verifies mechanically without interpretation (operator ruling 260704).

## Character
One suite run plus possible Lode cleanup; wrap follows.

## Commit Activity

```

File touches (file: the paces whose commits touched it):

  E  ₢BqAAE  federation-credential-fault-disposition
  A  ₢BqAAA  sitting-runway-contract
  B  ₢BqAAB  bubc-band-widen
  C  ₢BqAAC  runway-gate-and-renew
  H  ₢BqAAH  sitting-liveness-probe
  I  ₢BqAAI  avowal-code-clipboard
  D  ₢BqAAD  poll-mantle-remint
  F  ₢BqAAF  suite-credential-preamble
  G  ₢BqAAG  curl-band-overlap-doctrine

  RBS0-SpecTop.adoc                           E A H I
  rbtdgc_consts.rs                            B C H D
  rbtdrv_patrol.rs                            C H D F
  bubc_constants.sh                           B C G
  rba_auth.sh                                 C I G
  rbcc_constants.sh                           C H D
  rbtdra_almanac.rs                           C I F
  rbtdrm_manifest.rs                          C I F
  BCG-BashConsoleGuide.md                     B G
  claude-rbk-acronyms.md                      C H
  claude-rbk-tabtarget-context.md             C H
  rba_cli.sh                                  C H
  rbfcb_host.sh                               E D
  rbtdrf_fast.rs                              I D
  rbz_zipper.sh                               C H
  CLAUDE.md                                   D
  Cargo.lock                                  I
  Cargo.toml                                  I
  RBSCIP-IamPropagation.adoc                  E
  buc_command.sh                              I
  memo-20260705-wifi-tls-timeout-incident.md  G
  rbfc0_core.sh                               D
  rbfcg_gar.sh                                G
  rbfd_director.sh                            G
  rbfr_retriever.sh                           G
  rbfv_verify.sh                              G
  rbgc_constants.sh                           E
  rbgp_payor.sh                               G
  rbgv_cli.sh                                 C
  rbtdrn_conformance.rs                       G
  rbtdru_cupel.rs                             I
  rbw-aN.NovateSitting.sh                     C
  rbw-as.EspySitting.sh                       H
  rbxk_keycloak.sh                            G

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 39 commits)

  1 E federation-credential-fault-disposition
  2 A sitting-runway-contract
  3 B bubc-band-widen
  4 C runway-gate-and-renew
  5 H sitting-liveness-probe
  6 I avowal-code-clipboard
  7 D poll-mantle-remint
  8 F suite-credential-preamble
  9 G curl-band-overlap-doctrine

123456789abcdefghijklmnopqrstuvwxyz
··xx·······························  E  2c
····x·x····························  A  2c
·······xx··························  B  2c
··········xxxx··x·x················  C  6c
···················xx··············  H  2c
·····················xxx···········  I  3c
························xxx········  D  3c
···························xx······  F  2c
·····························xxx·xx  G  5c
```

## Steeplechase

### 2026-07-05 09:24 - ₢BqAAG - W

Verification closed the pace: picket green 22 fixtures / 163 passed / 0 failed, the full ladder including hallmark-lifecycle and lode-lifecycle — the live proof of every swept rbfd/rbfcg/rbfr/rbfv path riding the canonical curl form. The stray busybox test Lode from the 260704 killed run (b260704140155) was divined and banished first. Two environmental failures interrupted the runs and were dispositioned: a payor OAuth invalid_rapt lapse (operator refreshed) and a ~45s TLS-handshake timeout burst on the station's open guest Wi-Fi that felled the post-abjure summon mid-fixture — investigated to the airportd telemetry layer, sweep exonerated, closed with no code change via the incident memo memo-20260705-wifi-tls-timeout-incident.md (operator ruling: memo is the complete disposition).

### 2026-07-05 08:42 - ₢BqAAG - n

Incident report memo for the 260705 picket kill: a ~45s TLS-handshake timeout burst (curl 28, SSL connection timeout, HTTP 000) against two googleapis endpoints felled hallmark-lifecycle's post-abjure summon mid-verification. Investigation exonerated the curl-containment sweep (no timeout semantics touched; curl's own stderr names the wire) and the Wi-Fi association (RSSI steady, txFail=0, no roam/deauth); the correlate is a channel-contention burst on the station's open guest Wi-Fi (cca 22->59%, interference tripled) with Ethernet unplugged. One prior occurrence in 34,574 logged invocations (260604, same signature). Closed with no code change: the outage outlasted rbuh's full 39s retry window, so retry tuning fails the load-bearing test; wired Ethernet noted as the operator-side lever. Memo is the complete disposition per operator ruling.

### 2026-07-04 14:05 - Heat - d

batch: 1 reslate

### 2026-07-04 13:46 - ₢BqAAG - n

Curl containment sweep: all 28 red-run violations repaired to the canonical form the scan legislates. The 19 bare curl-into-buc_die chains (rbfcg_gar 8, rbfv_verify 6, rbfr_retriever 3, rbfd_director 2) each gained the capture terminator `|| z_curl_status=$?` plus a classify test whose die message carries the curl exit — no resets needed in the die-shaped functions since every capture dies on nonzero, so the var is always 0 at the next site. The two rbgp_payor pipe-forms restructured to line-leading curl with the jq-built request body riding a process substitution (`-d @<(jq …)`) — the secrets-never-touch-disk custody the original pipe existed for is preserved, and jq's program compacted to one line so the continuation chain reaches the terminator. rba_auth's five federation-leg captures renamed z_status→z_curl_status per the forced-name ruling (the jq-only z_status capture in the sibling function untouched). rbxk_keycloak's two capture-assignments restructured to line-leading curl writing the http_code to a new ZRBXK_HTTP_CODE temp (poll loop keeps its tolerate-and-retry classify; the JWKS fetch now dies with the precise curl exit instead of folding into HTTP 000). Shellcheck 229 clean. Not yet green-run — notch-before-test.

### 2026-07-04 13:36 - ₢BqAAG - n

Curl containment scan built in the conformance fixture as a positive-form patrol (operator design, ruling 260704): rather than hunting bad shapes, the scan legislates the canonical curl form and errors on everything else — a curl invocation must lead its line, its terminator must end with the byte-exact capture `|| z_curl_status=$?` (forced name so theurge verifies mechanically, no interpretation), the captured status must reappear within a 24-line window (corpus max: rbuh_http's dual-variant if/else at 21), a curl token anywhere else is misplaced (sole carve-out: the `command -v curl` presence probe), and a line whose quoting the deliberately-dumb line-local paired strip cannot resolve is itself a violation — the scan demands scannable lines instead of growing bash-quoting sophistication. The positive form also closes the set-e propagation hole the buc_die-chain hunt would have missed. Exempt prefixes: ABANDONED-github (retired) and the rbgj in-pool cloud-step trees (CBG/JDG dialect — no band membrane there). Three hermetic self-test cases pin canon-clears, five deviant kinds, and unscannable; the live-tree case walks .sh files under Tools/ and tt/. Not yet run — notch-before-test; red run expected to demand the 21 chain repairs, 2 rbxk restructures, and 5 rba_auth z_status renames.

### 2026-07-04 13:21 - ₢BqAAG - n

Curl-band overlap doctrine recorded at both homes: BCG's placement paragraph drops curl from the placed-clear-of list and gains a dedicated paragraph — curl 8.6.0/8.8.0 minted CURLE_TOO_LARGE=100 and CURLE_ECH_REQUIRED=101 onto band_regime/band_enroll, growing ~1 code/year; re-base weighed and declined (zero-sum window under the 124 ceiling, renumbers the census, rents years only); containment stated normatively — curl exits captured-and-classified at the call site, a bare curl-into-buc_die chain rule-barred. The bubc tinder placement comment carries the same content in comment form. Grep gate NOT yet clean: the docket's survey premise proved false — 21 bare curl-into-buc_die chains stand on the shipped surface (rbfcg_gar 8, rbfv_verify 6, rbfr_retriever 3, rbfd_director 2, rbgp_payor 2); theurge scan case + repair sweep follow test-first.

### 2026-07-04 13:08 - ₢BqAAF - W

Credential-readiness leader restored to the release suites as a pure consumer fixture: espy fail-fast (absent sitting rejects in seconds with the rbw-aa/rbw-aN advisory), promptless baseline avow riding the runway gate (short sitting band-rejects with the novate advisory), then dons of director + retriever — the mantles the ladders' inner bodies wield (governor deliberately excluded, proven where wielded by the polity fixtures). The espy+avow leader extracted into a shared sitting-ready arc consumed by both the access-probe gate case (which keeps the impossible-runway negative) and the new leader, so gate and leader cannot drift. Membership: dogfight and blockade lead outright, skirmish leads after the state-indifferent enrollment-validation, and gauntlet carries it right after freehold-establish — forced by the cosmology ladder-containment law (blockade ⊆ skirmish ⊆ gauntlet); it cannot lead gauntlet, which starts from marshal-zero with no depot to don against. Manifest census updated (new fixture const + espy/avow/don colophons) and the access-probe espy census gap closed. Proofs: theurge 156/156 with the cosmology laws green over the new membership; fixture positive PASSED live against the standing sitting; deterministic credless negative (XDG_RUNTIME_DIR at empty scratch) failed fast with the exact advisory; access-probe 2/2 post-refactor; reveille 118/118.

### 2026-07-04 13:04 - ₢BqAAF - n

Credential-readiness leader built as a pure consumer fixture: the release ladders' up-front credential step (espy fail-fast + promptless baseline avow through the runway gate, then don director + retriever — the mantles the ladders' inner bodies wield; governor deliberately excluded, proven where wielded by the polity fixtures). The shared sitting-ready arc extracted from the access-probe gate case (which keeps the impossible-runway negative on top of it) so leader and gate ride one arc. Suite membership: skirmish leads with it after the state-indifferent enrollment-validation, dogfight and blockade lead outright, and gauntlet re-verifies right after freehold-establish — required by the cosmology ladder-containment law (blockade ⊆ skirmish ⊆ gauntlet); it cannot lead gauntlet, which starts from marshal-zero with no depot to don against. Manifest census: new fixture const + colophon roster (espy/avow/don), plus the access-probe census gap closed (espy was invoked but unlisted since the gate case landed). Not yet tested — notch-before-test.

### 2026-07-04 12:37 - ₢BqAAD - W

Mid-flight re-don landed at the one long-lived mantle consumer and proven live: zrbfc_redon_tick in rbfcb_host.sh (the don alone, never the avow-folding accessor; process-frame call with the z_rbfc_redon_token result-global per BCG's $()-for-pure-captures rule) integrated into the shared build-completion poll on a since-don count ahead of each status fetch — ZRBFC_BUILD_POLL_REDON_CADENCE=360 (30 min, 2x margin under the ~60-min mantle ceiling; worst-case conjure spans ~3 re-dons). Rejection surfacing decided: lapsed sitting buc_dies with the open-a-sitting advisory (rbw-aa or rbw-aN), admission deficit buc_rejects BUBC_band_admission directly, transient don failure with a live sitting warns and retries next poll with the consecutive-failure counter as backstop (composes-with-counter cinch honored). The ₣Bf known-gap comment in the poll retired; RBS0 unchanged — the premise already spoke in the post-fix voice. Test seam buorb_redon_cadence (RBCC_tweak_redon_cadence, BURE_TWEAK_VALUE = poll count) read at the poll's one membrane; family renamed remint->redon mid-build by operator ruling (mint reserved for the naming discipline; MCM/AXLA hold unhyphenated remint), grep gate clean, CLAUDE.md tweak census updated. Proofs: hallmark-lifecycle ordain now rides the tweak (cadence 4) asserting the Re-donned announcement — live run PASSED with nine re-dons at exactly polls 4/8/12/16/20 (Conjure) and 4/8/12/16 (Vouch), proving every consumer of the shared poll, build SUCCESS on the re-donned tokens; new foundry-path rt-lapse-advisory proves the lapse death deterministically and credless (XDG_RUNTIME_DIR pointed at empty scratch, full furnish from committed regimes, no network by construction). theurge 156/156, shellcheck 229 clean, foundry-path 6/6, reveille 118/118. Residual by design: the literal 60-minute expiry survival is the by-construction leg (cadence margin + runway floor); no hour-long build was spent observing a real lapse.

### 2026-07-04 12:24 - ₢BqAAD - n

rt-lapse-advisory furnish gap: rbgo's kindle reaches zburd_sentinel, so the kit-bash ceremony now sources burd_regime.sh and zburd_kindles (enrollment registers names only — no enforce, BURD_TEMP_DIR still supplied directly). First run tripped exactly here; iterating empirically as planned.

### 2026-07-04 12:22 - ₢BqAAD - n

Mid-flight re-don built at the one long-lived mantle consumer: zrbfc_redon_tick in rbfcb_host.sh — the don alone (never the avow-folding accessor), called in the process frame with the z_rbfc_redon_token result-global (BCG: $() reserved for pure captures), lapsed sitting buc_dies with the open-a-sitting advisory, admission deficit buc_rejects on its band, transient don failure with a live sitting warns and retries next poll while the consecutive-failure counter stays the backstop. Poll integrates the tick on a since-don count ahead of each status fetch; ZRBFC_BUILD_POLL_REDON_CADENCE=360 (30 min, 2x margin under the ~60-min mantle ceiling; conjure worst case spans ~3 re-dons); the rbfcb known-gap comment retired. Test seam RBCC_tweak_redon_cadence (buorb_redon_cadence, BURE_TWEAK_VALUE = poll count) read at the poll's one membrane; family renamed remint->redon mid-build by operator ruling (mint is reserved naming-discipline vocabulary; MCM/AXLA already hold unhyphenated remint for name re-minting) — grep gate clean on redon. Tests: hallmark-lifecycle ordain now runs under the cadence tweak (value 4) asserting the Re-donned announcement in the transcript (positive rides the existing picket build, no second build); foundry-path gains rt-lapse-advisory, a deterministic credless kit-bash negative (XDG_RUNTIME_DIR pointed at empty scratch, full furnish ceremony from committed regimes, tick must die with the lapse advisory before any network touch). CLAUDE.md tweak census gains the re-don cadence override. Not yet tested — notch-before-test.

### 2026-07-04 11:52 - ₢BqAAI - W

Avowal user-code clipboard copy landed as a BUK mechanism + RB policy split (operator-directed hoist mid-mount): buc_clipboard_copy_predicate in buc_command.sh beside its genre-sibling buc_native_path_capture — probe chain pbcopy/clip.exe/wl-copy/xclip, existence-probing AS the platform discrimination (no OSTYPE sniffing, per BCG Platform-Variant guidance naming BUK the wrapping home), kindle-free and silent, exit-status verdict + z_buc_clipboard_tool result-global. Thin zrba_user_code_clipboard wrapper holds the RB policy: custody (user code ONLY — never device code or tokens), the display-safe determination + accepted clipboard-sync residual noted at the site, buc_step success announcement, fail-soft display-only degradation; called once at the device-flow prompt emission both avow-miss and novate reach. Tools inventoried in RBS0 (optional probe-and-skip row) AND in cupel's compiled census ZRBTDRU_DECLARED_DEPS — the first reveille pass failed on exactly that gap, the discipline catching the undeclared deps as designed. Testing settled by operator ruling: clipboard-READ capability confined to the theurge test binary (arboard 3.6.1), never on the shipped bash surface; new roster-only clipboard fixture (foundry-path precedent homes BUK-function unit tests in theurge — the planned BUTT case folded in here) with the deterministic PATH-emptied decline case and the arboard round-trip (save/restore, WSL split-surface skip, headless self-skip); member of no suite since the round-trip mutates the live desktop clipboard. Proven: round-trip PASSED live on this host (real pbcopy copy, arboard read-back, prior clipboard restored); theurge 156/156, buw-st 49/49, clipboard 2/2, shellcheck 229 clean, reveille 117/117. Residual by design: the site announcement itself is the operator-ceremony positive — visible at the next real avow-miss/novate; no follow-on pace needed.

### 2026-07-04 11:50 - ₢BqAAI - n

Cupel declared-deps census follows the RBS0 inventory row: the four optional probe-and-skip clipboard tools (clip.exe/pbcopy/wl-copy/xclip) join ZRBTDRU_DECLARED_DEPS as a comment-grouped tier — reveille's cupel kit-bash case had correctly flagged them as undeclared (the discipline working), proving the census-mirrors-inventory seam.

### 2026-07-04 11:47 - ₢BqAAI - n

Avowal user-code clipboard copy built as a BUK mechanism + RB policy split (operator-directed hoist): buc_clipboard_copy_predicate in buc_command.sh — platform-normalized probe chain pbcopy/clip.exe/wl-copy/xclip, existence-probing as the platform discrimination (no OSTYPE sniffing per BCG Platform-Variant guidance), kindle-free and silent with the z_buc_clipboard_tool result-global, exit-status verdict; thin zrba_user_code_clipboard wrapper in rba_auth.sh holding the custody rule (user code ONLY — never device code or tokens), the security determination (display-safe per RBS0 rbtf_avow; accepted clipboard-sync residual), success announcement on the progress stream, fail-soft display-only degradation — called once at the device-flow prompt emission both avow-miss and novate reach. Optional probe-and-skip tools inventoried in RBS0 Dependency Inventory. Theurge clipboard fixture (foundry-path sibling, roster-only — member of no suite since the round-trip mutates the live desktop clipboard): cb-no-tool-decline proves the fail-soft contract deterministically via PATH-emptied probe; cb-round-trip proves a real copy lands by reading back via arboard (read capability deliberately confined to the test binary per operator ruling — no clipboard-read primitive on the shipped bash surface), with save/restore of prior text, WSL split-surface skip, and headless self-skip. arboard 3.6.1 added to the theurge crate. Not yet tested — notch-before-test.

### 2026-07-04 11:17 - ₢BqAAH - W

Read-only sitting probe minted and landed: espy elected by operator census (candle runner-up; appraise/apprise/witness/gauge disqualified per MCM Lapidary, grep gate clean). rba_espy_sitting in rba_cli.sh reports verdict live/lapsed/absent + raw runway from the cache alone via the new <foedus>.sitting fact (RBCC_fact_ext_sitting → RBTDGC_FACT_EXT_SITTING), absent-is-a-verdict exit 0; colophon rbw-as beside UPPER rbw-aN, zipper-enrolled with regenerated consts and tabtarget context. Theurge gate arc now espies first and fails fast on a dead sitting with the open-a-sitting instruction, retiring the interim device-window log-pointer; baseline avow promptless by construction. RBS0 rbtf_espy seated beside novate (whose one-surface claim was amended); acronyms RBA entry updated. Proven live against the standing sitting (LIVE ~11h39m); theurge 156/156, reveille 117/117, access-probe 2/2, shellcheck 229 clean. Fail-fast negative proven by construction only — a live proof would extinguish the standing sitting and no delete verb exists by design. Runway-gate rejection text deliberately unchanged (already carries exact runway numbers). Observation flagged: foedus-reuse fixture's credential-heal avow may still prompt — same ruling arguably applies, operator to itch or decline.

### 2026-07-04 11:14 - ₢BqAAH - n

Sitting probe MINTED and BUILT: espy elected by operator census (candle runner-up — the egg-candling read; fathom/auscultate also-rans; appraise taken by JJK zjjrvl_appraise, apprise its one-letter neighbor, witness trodden in-house, gauge shadowed by rba prose 'gauging' — all per MCM Lapidary, grep gate clean on espy). rba_espy_sitting in rba_cli.sh: read-only over the gate pace's helpers (path/runway/live-predicate), verdict live/lapsed/absent + raw runway riding the new <foedus>.sitting fact (RBCC_fact_ext_sitting, projected to RBTDGC_FACT_EXT_SITTING), absent-is-a-verdict exit 0 per the descry precedent, only a broken read dies. Colophon rbw-as (lowercase = reads, beside UPPER novate rbw-aN), enrolled channel-"", tabtarget + regenerated consts and context riding. Theurge gate arc now fails FAST: espy first, dead sitting reported in seconds with the open-a-sitting instruction (rbw-aa or rbw-aN), the interim device-window log-pointer advisory retired; baseline avow promptless by construction. RBS0: rbtf_espy quoin seated beside novate; novate's 'one operator-invoked surface' amended to 'one mutating surface beside the read-only espy'. Proven live: rbw-as reported LIVE runway 41991s against the standing sitting; theurge 156/156; shellcheck 229 clean. Runway-gate rejection text left unchanged (docket's 'consider': the rejection already carries exact runway numbers; the probe adds nothing at that moment).

### 2026-07-04 10:54 - ₢BqAAC - W

Runway gate + novate landed as one. Gate: rba_avow's sitting-reuse branch computes remaining runway (zrba_sitting_runway_capture, own forensic temp pair) against a required-runway parameter seam defaulting to the kindled 2h floor; short sittings reject in the freshly-minted BUBC_band_runway=115 naming the rbw-aN remedy; fresh path extracted to zrba_sitting_open. Novate: rba_novate = credless-guarded force-fresh riding the shared open path, surfaced as tt/rbw-aN.NovateSitting.sh over the new thin rba_cli.sh; rbw-aa gained the required-runway folio (param1). Tests: access-probe gained the deterministic gate negative (exact band + advisory asserted on the merged stream, impossible 999999s demand riding the parameter seam — no tweak, no cache forgery), proven green against a live sitting; interactive novate proven twice by operator ceremony (full 12h window, second run extinguishing a standing live sitting); programmatic arm by construction (novate rides the proven RFC 7523 open path; live exercise awaits the Keycloak facility stream). Mid-pace rulings recorded: NO interactive act may live inside a theurge fixture (sitting-novate fixture retired under it), and the launcher merges spawned-tabtarget stderr into stdout (documented at the assert site). Follow-on paces slated: ₢BqAAH sitting-liveness-probe (fail-fast for the gate arc), ₢BqAAI avowal-code-clipboard. Verified: theurge 156/156, reveille 117/117, shellcheck 229 clean, buw-st 49/49, access-probe 2/2.

### 2026-07-04 10:52 - Heat - S

avowal-code-clipboard

### 2026-07-04 10:42 - ₢BqAAC - n

Sitting-novate fixture RETIRED under the operator ruling that no interactive act may live inside a theurge fixture (its novate step polled the device window blind for the full 15-minute expiry — the verb behaved to spec, the fixture design was wrong): fixture static, case fn, almanac registration, and manifest entries removed; the deterministic gate negative stays in access-probe (proven green against a live sitting), and novate's positive proof becomes the operator ceremony — rbw-aN from a terminal, then a promptless plain avow. The gate case comment records the ruling. Message truth fix riding along: zrba_sitting_open's step lines are now mechanism-descriptive (Opening/Acquiring a sitting via ...) — the old No-live-sitting prefix lied under novate, which deliberately replaces a live one; rba_avow's miss branch keeps the state announcement. Probe pace ₢BqAAH redocketed to match (gate-arc fail-fast only, ruling cinched).

### 2026-07-04 10:42 - Heat - d

batch: 1 reslate

### 2026-07-04 10:21 - Heat - S

sitting-liveness-probe

### 2026-07-04 10:19 - ₢BqAAC - n

Gate-arc stream survey folded in: the launcher's self-logging merges the spawned tabtarget's stderr into stdout (operator run 260704 proved captured stderr arrives empty), so the novate-advisory assert now checks both streams, the demand's stdout joins the trace files, and every failure forensic in the gate/novate arcs prints stdout+stderr. Interim operability: rbtdrg_info_now pointers before each may-prompt step (baseline avow, novate) name the spawned tabtarget's log as where the device-flow sign-in surfaces — the blind 15-minute device-window poll the operator hit gets a guided wait until the coming sitting-probe pace lands fail-fast.

### 2026-07-04 09:50 - ₢BqAAC - n

Runway seam moved to the sanctioned folio channel + BCG compliance pass: rbw-aa re-enrolled param1 (channel-"" dispatch drops args by design), rbgv_check_avowal reads the demand from BUZ_FOLIO per the check_mantle precedent (buc_doc_param dropped — the value is a folio, not a positional; furnish doc_env line covers both consumers); runway capture gets its own forensic temp-file pair (ZRBA_FED_RUNWAY_EXPIRY/NOW) instead of overwriting the sitting-read pair, per the AVOW_NOW precedent; rba_cli.sh executable bit set (dispatch execs the module). Wiring proven offline via the credless-guard membrane: rbw-aN → 104 through the full furnish, rbw-aa 999999 → 104 (param flows, validates), rbw-aa notanumber → loud die 1, bare rbw-aa → 104 (default floor path). Tabtarget context regenerated (rbw-aa folio column param1).

### 2026-07-04 09:41 - ₢BqAAC - n

Rust projection + tabtarget context regenerated via theurge build: RBTDGC_NOVATE_SITTING (rbw-aN) and RBTDGC_BAND_RUNWAY (115) landed; access group table row + retitle carried.

### 2026-07-04 09:41 - ₢BqAAC - n

Runway gate + novate landed as one: BUBC_band_runway=115 minted (free codes now 116-122) and projected via rbcc_emit_consts; rba_avow gains the reuse-path runway gate over a new zrba_sitting_runway_capture, parameterized on required-runway defaulting to the kindled 2h floor (ZRBA_SITTING_RUNWAY_FLOOR_SEC), rejecting short sittings in-band with the rbw-aN advisory; the fresh path extracted to zrba_sitting_open, shared by the new force-fresh rba_novate (credless-guarded like avow). Surface: rbw-aN NovateSitting tabtarget fronting new thin rba_cli.sh (avowal-path furnish only); rbw-aa gains the optional required-runway passthrough. Tests: access-probe gains the deterministic gate negative (exact band + advisory assert, impossible 999999s demand riding the parameter seam — no tweak, no cache forgery); new operator-invoked no-suite sitting-novate fixture runs the full round-trip (gate -> novate -> full-window reuse). Consts + tabtarget context regeneration rides the next build.

### 2026-07-04 09:16 - Heat - S

curl-band-overlap-doctrine

### 2026-07-04 09:10 - ₢BqAAB - W

BUK precision reject band widened 16->24 — the terminal width: the timeout/container ceiling at 124 fixes the maximum extent at 100-123 and the band now claims that whole window, so widening can never recur (determination recorded in BCG's band doctrine). Self-test sentinel re-pinned at the new top (115->123), freeing codes 115-122 for the coming gates; BUTT fixture expectations ride the sentinel symbolically so the re-pin carried them. Rust projection regenerated via theurge build. Allocation rules and every existing code unchanged. Verified: buw-st 49/49, theurge units 156/156.

### 2026-07-04 09:06 - ₢BqAAB - n

BUK precision reject band widened 16->24, the terminal width: the timeout/container ceiling at 124 fixes the maximum extent at 100-123 and the band now claims that whole window, so it can never widen again (future capacity comes from the allocation rule, never growth — determination recorded in BCG's band doctrine). Self-test sentinel re-pinned at the new top (115->123), freeing codes 115-122 for coming gates; BUTT fixture expectations ride the sentinel symbolically, so the re-pin carries them. Rust projection regenerated via the theurge build. Allocation rules and every existing code unchanged.

### 2026-07-04 08:58 - ₢BqAAA - W

RBS0 sitting-lifecycle contracts extended ahead of code (spec only, no code). Three definition sites landed: rbtf_sitting gains the proactive runway floor (reuse-branch gate, ~2h blanket default on a required-runway seam, no per-operation bounds until earned, named band rejection advising novate, fresh sitting = full runway by construction); rbsk_human_present's mid-flight re-mint promise extended from reactive fail-at-lapse to live-by-construction via the gate, lapse demoted to backstop; new rbtf_novate quoin seated beside avow/sitting/don, registered in the mapping section (novation = extinguish-by-replacement riding the atomic sitting overwrite; renewal-only, the one tabtarget on the sitting lifecycle where avow itself never is). Operator verb ELECTED via the federation vocabulary census: novate, over reavow (runner-up, derivative) and prorogate (failed cold probe); renew disqualified (trodden + payor-refresh near-neighbor RBSPR/RBSAO); grep-clean at mint. Paddock census line records the election with full verdict trail; stale study-time parenthetical amended.

### 2026-07-04 08:57 - Heat - d

paddock curried: census line records the novate election; stale study-time parenthetical amended

### 2026-07-04 08:56 - ₢BqAAA - n

RBS0 sitting-lifecycle contracts extended ahead of code: the sitting quoin gains the proactive runway floor (reuse-branch gate, ~2h blanket default on a required-runway seam, no per-operation bounds until earned, named band rejection); the human-present premise's mid-flight re-mint promise extends from reactive fail-at-lapse to live-by-construction via the gate, lapse demoted to backstop; new rbtf_novate quoin beside avow homes the force-fresh renewal act (novation: extinguish-by-replacement riding the atomic sitting overwrite; renewal-only, no release verb; the one tabtarget on the sitting lifecycle). Word elected via federation vocabulary census: renew disqualified (trodden + payor-refresh near-neighbor), reavow runner-up, prorogate failed cold probe; novate grep-clean at mint

### 2026-07-04 08:41 - ₢BqAAE - W

Credential-fault disposition recorded at spec homes per ACG three-homes correction: RBS0 don quoin gains the fault posture (single-attempt legs, call-context disambiguator, no flap tolerance by decision, await post-federation evidence), RBSCIP gains the blind-403 post-grant-by-construction rationale; vestigial RBGC_SA_KEY_CONSUMER_RETRY_* trio and its RBSCIP sentence removed; stale rbgg_enrobe_director refs fixed to rbgw_capabilities.sh in both RBSCIP and the constants comment, which collapsed to census + citation; build-poll re-mint gap marked at the poll naming rbsk_human_present and the sitting-lifecycle re-mint work; memo-20260527 retired; rbgc comment-collapse survey slated as ₢BbAAU in the ACG scrub heat. Shellcheck 228 clean, reveille 117/117.

### 2026-07-04 08:37 - ₢BqAAE - n

credential-fault disposition recorded at spec homes: RBS0 don quoin gains the fault posture (single-attempt legs, call-context disambiguator, no flap tolerance by decision), RBSCIP gains the blind-403 post-grant rationale; vestigial RBGC_SA_KEY_CONSUMER_RETRY_* trio and its RBSCIP sentence removed; stale rbgg_enrobe_director site refs fixed to rbgw_capabilities.sh; build-poll re-mint gap marked at the poll; memo-20260527 retired

### 2026-07-04 08:18 - Heat - f

racing

### 2026-07-02 15:19 - Heat - S

suite-credential-preamble

### 2026-07-02 15:12 - Heat - d

batch: paddock, 1 reslate

### 2026-07-02 15:11 - Heat - D

₢BfAAl → ₢BqAAE

### 2026-07-02 15:07 - Heat - d

batch: paddock, 4 slate

### 2026-07-02 15:06 - Heat - N

rbk-43-fbl-sitting-lifecycle

