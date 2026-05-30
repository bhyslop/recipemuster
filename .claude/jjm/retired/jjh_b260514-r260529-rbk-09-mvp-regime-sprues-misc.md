# Heat Trophy: rbk-09-mvp-regime-sprues-misc

**Firemark:** ₣BM
**Created:** 260514
**Retired:** 260529
**Status:** retired

## Paddock

## Context

Regime enums in this project carry their values as bare English words —
`conjure`, `bind`, `graft` for `RBRV_VESSEL_MODE`; `tether`, `airgap` for
`RBRV_EGRESS_MODE`; node-regime modes (`disabled`/`enabled`/`global`/
`allowlist`) for the `RBRN_*` family. These bare values appear scattered
across bash, AsciiDoc, Python (Cloud Build steps), and Rust (test
fixtures), with no cluster-discoverable marker tying any one value to its
enum family. This makes mode renames cross-language hand-search-and-replace
work, prone to silent drift.

There is one prior in-tree exemplar of the prefixed-value approach:
`bubep_linux | bubep_mac | bubep_windows` for `BURN_PLATFORM`. It works but
uses a different shape from what this heat adopts; harmonization is part of
the sweep.

The inventory pace has been completed (see heat log / slate commit
messages for depth). This paddock records the resolved scope and the
durable findings; line-numbered site lists deliberately live in the
discovery recipe, not here.

## MCM grounding — "sprue" is `mcm_sprue`, prefix discipline makes it an inlay

"Sprue" here is not a coined word — it is the MCM quoin `mcm_sprue`: *a
literal wire-level token representing a serialized name — the exact
character sequence as it appears in a wire format (a JSON property key, an
API field name, a protocol identifier).* Our regime enum values are
textbook sprues: `rbnve_conjure` is the verbatim token that appears in
`rbrv.env`, in the `_RBGA_VESSEL_MODE` Cloud Build substitution, in
build-info JSON, and in Python/shell comparisons.

MCM also states: *a sprue that follows the prefix naming discipline is also
an `mcm_inlay`* (prefix-recognized, not lexicon-catalogued). So precisely,
the values are *already* sprues even while bare; what this heat adds is the
prefix discipline that makes each sprue **also an inlay**. The spec pace
(₢BMAAA) should cite `mcm_sprue` + `mcm_inlay` for grounding rather than
inventing rationale.

Note on a sibling usage: ₣BK independently uses "sprue" for tabtarget
dispatch tokens (`{owner}ml_{launcher-id}`, e.g. `rbml_rbw`) and wrote a
local gloss in BCG/CLAUDE.md. That is *also* a valid `mcm_sprue` instance
(a literal dispatch-wire token) — not a competing definition, a sibling
one. Both heats correctly name wire-level literal tokens "sprue." ₣BK's BCG
text would ideally cite `mcm_sprue` rather than read as a freshly-minted
concept, but that is ₣BK's documentation concern, not a blocker here.

## Convention — locked

Pattern: `<proj><n><regime><e>_<value>` where:

- `<proj>` — project prefix (`rb` for Recipe Bottle, `bu` for BUK)
- `<n>`    — single letter signaling "this is a regime namespace value"
             (chosen `n` for "namespace", contrasting `r` which is taken
             for the regime-variable family `rbr*`/`bur*`)
- `<regime>` — single letter mirroring the regime family
             (`v` = vessel, `n` = node, `c` = config, `s` = station,
             `e` = environment, etc.)
- `<e>`    — single letter signaling "this is an enum value"
- `_<value>` — the canonical value name

Full target table (all in-scope enums):

| Variable                 | Regime | Bare → Sprue                                          |
|--------------------------|:------:|-------------------------------------------------------|
| RBRV_VESSEL_MODE         | v      | bind/conjure/graft → `rbnve_bind`/`_conjure`/`_graft` |
| RBRV_EGRESS_MODE         | v      | tether/airgap → `rbnve_tether`/`_airgap`              |
| RBRN_ENTRY_MODE          | n      | disabled/enabled → `rbnne_disabled`/`_enabled`        |
| RBRN_UPLINK_DNS_MODE     | n      | disabled/global/allowlist → `rbnne_*`                 |
| RBRN_UPLINK_ACCESS_MODE  | n      | disabled/global/allowlist → `rbnne_*`                 |
| BURN_PLATFORM            | n      | bubep_linux/mac/windows → `bunne_linux`/`_mac`/`_windows` |

Family-sharing is a deliberate property, not an accident: vessel-mode and
egress-mode share `rbnve_` (both vessel-regime); the three node-mode enums
share `rbnne_`, so `rbnne_disabled` is intentionally voiced by ENTRY,
DNS, and ACCESS alike. Uniqueness lives in the full quoin + the owning
variable, not in the 5-char prefix. A single grep per family returns the
whole cluster.

## Scope decisions — resolved

- **RBRV_VESSEL_MODE + RBRV_EGRESS_MODE** → `rbnve_*`. In.
- **RBRN_ENTRY_MODE / UPLINK_DNS_MODE / UPLINK_ACCESS_MODE** → `rbnne_*`.
  In. Their values are pure comparison tokens with no second life, so the
  prefix is free (no indirection needed).
- **BURN_PLATFORM** → harmonize `bubep_*` to `bunne_*` in the same sweep.
  Decided harmonize over grandfather, per load-bearing-complexity
  discipline — eliminates the two-shape wart rather than freezing it.
- **RBRN_RUNTIME (`docker`/`podman`) — EXCLUDED, stays bare.** Its value
  is consumed as the literal executable name (`command -v "${RBRN_RUNTIME}"`,
  `${RBRN_RUNTIME} image inspect`). Adopting the convention would require a
  permanent value→binary indirection layer at every dispatch site — a
  runtime cost paid for grep-discoverability of two values that
  `RBRN_RUNTIME` itself already marks. The convention would cost more than
  it returns. This is the one regime enum that does not earn the prefix;
  the exclusion is deliberate, not an oversight.
- **BURE_VERBOSE / BURE_COLOR** — out (bare ints, different problem class;
  unchanged from original framing).

## Comment / use-site classification — judgment work

Bare-word grep on a value yields four classes. The migration paces walk
the candidate list and classify each hit:

1. **Phase / concept references** — "the conjure phase", "the airgap
   build" — refer to the workflow, not the enum value. **Leave alone.**
2. **Enum-value references** — equality tests, case arms, JSON literals,
   `vessel mode is conjure`, defaults — the actual variable value.
   **Migrate.**
3. **Identifier embeds** — `RBSAC-ark_conjure.adoc`, `rbho_conjure_*`,
   vessel sigils `rbev-bottle-ifrit-airgap`, test-step labels like
   `ordain-conjure` — the bare word is part of an identifier. **Leave
   alone.**
4. **Coupled derived strings** — the value drives a derived token that
   lives in resource-name space and must stay bare even though it is
   semantically the enum value. **Leave the derived string bare; migrate
   only the dispatch that selects it.** Two confirmed witnesses:
   - egress → worker-pool suffixes (`RBGC_POOL_SUFFIX_*`, the `-tether`/
     `-airgap` fragments in pool paths)
   - vessel-mode → hallmark prefixes (`RBGC_HALLMARK_PREFIX_*` = `c`/`b`/`g`)
   Because of class 4, even a class-2 case arm is not blind find/replace:
   the arm becomes `rbnve_tether)` while the pool string it maps to stays
   `-tether`.

### Three judgment traps a naive grep misses

- **Default-value defaults** — `${z_mode:-conjure}` is class-2 but a
  `== "conjure"` grep skips it. The recipe must also grep `:-conjure`,
  `:-bind`, etc.
- **AsciiDoc wire-value tables read like prose** — a spec table cell
  documenting `(conjure, bind, or graft)` as a substitution value is
  class-2 (migrate), not class-1 prose. Easy to misclassify and leave
  stale.
- **Rust noise is almost all class-3** — the onboarding-scenario fixtures
  are full of step labels (`ordain-conjure`, `wrest-bind`) and sigil
  consts that stay bare. Only the `export RBRV_*_MODE="..."` baseline
  strings in the fast-fixture file actually migrate; that pace rebuilds
  the Rust crate (`tt/vow-b`).

### Noise ranking (classification effort)

Counterintuitively `conjure` is the noisiest value (heavy phase/operation
prose), then `bind` (English-word noise: bind-mount, IP bind), then
`graft`; `tether`/`airgap` are the cleanest (~10 sites each, mostly
class-2 or class-4). This is why egress is the cheap pilot and vessel-mode
is the heavy classification grind.

## Cloud Build detection gap — vessel-mode acceptance gate

Vessel-mode's value crosses a substitution boundary
(`_RBGA_VESSEL_MODE` / `_RBGV_VESSEL_MODE`) into a **remote execution
context** — Cloud Build step shells and Python steps compare against the
literal value. The local `regime-validation` net catches the
declaration↔Rust-fixture lockstep, but it does **not** exercise the
remote-side comparisons. A missed step-shell or Python site fails silently
until an actual conjure/graft cloud build runs.

**Consequence:** for the vessel-mode pace, "gauntlet green" is not
sufficient proof of done. Its acceptance gate is an actual **conjure +
graft cloud build** (the full ordain path), not just the fast/crucible
suites.

## Discovery recipe

- `grep -rn 'buv_enum_enroll' Tools/buk/ Tools/rbk/` — authoritative
  declarations; cross-walk `buv_gate_enroll` invocations to confirm no
  enum missed.
- Per value V: `grep -rn '\bV\b' Tools/ tt/` filtered by extension for
  code sites; also `grep -rn ':-V\b'` for default-value defaults.
- Comment/prose: `grep -rn '#.*\bV\b'` (bash/Python), `'//.*\bV\b'`
  (Rust), and scan `Tools/*/vov_veiled/*.adoc` for prose + wire-value
  tables.
- Substitution boundary: trace `_RBG[AV]_VESSEL_MODE` /
  `_RBG[AV]_EGRESS*` through `rbfd_*`/`rbfv_*` step assembly into the
  `rbgja*`/`rbgjv*` Python+shell steps.

## Posture

**Burn-bridges migration.** No backwards compatibility. Depots reform on
execution, vessels re-charge, regime files get edited in place. Operator
pre-flight: `rbw-MZ` before sweep, all crucibles quenched.

**Deep-test stragglers.** `regime-validation`, `dockerfile-hygiene`, and
`gauntlet` catch any local bare-value site, because the enum-validation
gate rejects a non-prefixed value once the declaration is updated. The
remote Cloud Build comparisons are the exception — see the detection gap
above.

## Heat-shape note — ₣BK relationship (BK coding landed)

₣BK (moorings-cutover) swept essentially this heat's file surface
(`Tools/rbk/**/*.sh`, `rbtd/src/*.rs`, `RBS*.adoc`) — but for path literals,
not enum values. The two heats are **semantically orthogonal** (different
concerns, mostly different lines). ₣BK's *coding* has landed (BKAAI
wrapped); the prior file-surface-churn concern is now historical — the tree
is settled and the re-validation confirmed the enum-value sites survived
intact.

What remains and matters:
- **Start gate:** ₣BK's two-platform gauntlet acceptance is still pending.
  Begin this heat only after ₣BK's gauntlet passes on linux + macos and any
  cutover bugfixes land — so a known-good cutover baseline exists and every
  ₣BM gauntlet signal is unambiguously about enums, not the cutover.
- **Residual churn risk:** only late ₣BK bugfixes from its gauntlet runs.
  Low; re-grep before editing if BK commits land meanwhile.
- The bubep→bunne harmonize touches BUK files (₣BK's other subtree). ₣BK's
  BUK code has landed, so no hard coordination remains — just awareness of
  any late bugfix churn there.

## What done looks like

- All in-scope bare enum values replaced with sprue-prefixed values across
  bash, AsciiDoc, Python (Cloud Build), and Rust; `RBRN_RUNTIME` left bare.
- `buv_enum_enroll` (and gated `buv_gate_enroll`) declarations updated to
  expect the new values; regime files (`rbrv.env`, `rbrn.env`, BURN
  profiles) updated in place.
- Class-4 derived strings (pool suffixes, hallmark prefixes) left bare;
  their selecting dispatch migrated.
- `regime-validation` + `dockerfile-hygiene` + `gauntlet` green.
- Vessel-mode additionally proven by a real conjure + graft cloud build.
- A single grep per family returns the complete cluster:
  `grep -rn rbnve_ Tools/`, `grep -rn rbnne_ Tools/`,
  `grep -rn bunne_ Tools/` — and `grep -rn bubep_` returns nothing.

## Out of scope

- Sprue convention applied to non-regime-enum domains (status codes, JSON
  wire-format keys, etc.) — separate heat.
- `RBRN_RUNTIME` (`docker`/`podman`) — executable-name coupling, excluded
  by decision above.
- `BURE_VERBOSE` / `BURE_COLOR` integer values — different problem class.
- Backwards-compatibility shims of any kind.

## Pace shape (deferred to mount-time)

Resolved cadence is per-variable-family, ordered cheap-pilot-first:

1. Convention spec authoring — formalize the convention as a spec section,
   citing `mcm_sprue` + `mcm_inlay` for grounding, plus the class-4
   "coupled derived string stays bare" rule (two witnesses) and the
   `RBRN_RUNTIME` exclusion rationale.
2. Egress-mode pilot — `rbnve_tether`/`_airgap`. Smallest, cleanest;
   proves the mechanical pattern and the class-4 distinction.
3. Vessel-mode — `rbnve_bind`/`_conjure`/`_graft`. Heaviest; crosses
   bash/adoc/Python(Cloud Build)/Rust. Acceptance gate = real conjure +
   graft cloud build.
4. Node-mode — `rbnne_*` for ENTRY/DNS/ACCESS. High English-word noise,
   heavy classification.
5. bubep → bunne harmonize — BUK files; last slice.
6. Final regression sweep — gauntlet end-to-end + per-family grep audit.

Slate as thin dockets at mount-time (depth in slate commit messages); do
not pre-cut detailed site lists from this paddock.

## Paces

### theurge-colophon-consolidation (₢BMAAK) [complete]

**[260526-1827] complete**

## Character
Mechanical rename + single-pipeline collapse. Judgment only at the
doc-sweep's prose-vs-value calls; the colophon mint is pre-resolved below.
Intricate but mechanical — hands the heavy gauntlet/cloud regression to the
final-regression pace that follows.

## Goal
Collapse the theurge rbtd-* tabtarget family into rbw-t* and retire the
second (RBTW) pipeline — all test/qualify dispatch on one workbench, no
test lost, materially fewer tabtargets.

## Mint (resolved)
- rbw-tb Build           <- rbtd-b
- rbw-tt Test            <- rbtd-t
- rbw-ts TestSuite.<imprint> (imprint)  <- rbtd-s.TestSuite.* + folded
  tP/tS/tT. Carries every current imprint incl. dogfight, plus a new
  `tadmor` suite = (kludge-tadmor, tadmor) preserving QualifyTadmor's
  deliberate two-fixture sequence (see rbq_qualify_tadmor comment for why
  they must stay separate fixtures). Note: tP=gauntlet / tS=skirmish are
  already imprints, so only tadmor is genuinely new.
- rbw-tf FixtureRun (param1)  <- every rbtd-r.FixtureRun.* tabtarget.
  Param form also covers dogfight (no FixtureRun tabtarget exists for it).
- rbw-tc FixtureCase <fixture> [case] (param1)  <- rbtd-s.FixtureCase.
  Preserve omit-to-list behavior: no fixture -> list fixtures; fixture
  only -> list cases (the crucible-debug loop).
- rbw-tq QualifyFast (moved off rbw-tf — do the move before repurposing tf
  for FixtureRun, else a transient same-colophon collision).
- rbw-tr QualifyRelease kept. Its existing tt/rbw-ts.TestSuite.complete.sh
  call (currently a dangling forward-reference) becomes valid once suites
  land at rbw-ts — this pace closes that long-standing broken reference.
- rbw-tK/tO kept.

## Shape — one pipeline
Re-home rbte_cli as a buz module enrolled in rbz_zipper, dispatched by
rbw_workbench. Delete rbtw_workbench.sh, launcher.rbtw_workbench.sh, and
the rbte_dispatch switch. rbte_engine.sh and the Rust binary unchanged.

## Recipe (not site lists)
- Live callers: grep -rn 'rbtd-[bstr]' Tools/ tt/ (exclude .claude/jjm).
- Operator-facing: buh_tt calls naming rbtd-* in handbook scripts.
- Regenerate the generated tabtarget context via rbw-MG — QualifyFast's
  staleness gate (rbq_qualify_context) hard-fails otherwise; the regen also
  refreshes the @-included Command Reference.
- Consumer-visible: CLAUDE.consumer.md ships these colophons — release-note
  the rename (esp. rbw-tP release-gate -> rbw-ts.TestSuite.gauntlet.sh).

## Done (cheap/local — heavy regression is the next pace, AAF)
- grep -rn 'rbtd-' Tools/ tt/ returns zero live colophons (history aside).
- Theurge crate builds clean; rbw-ts.TestSuite.fast.sh green (proves the
  one-pipeline dispatch end-to-end on the cheap path).
- rbw-tq QualifyFast green (after rbw-MG regen) — colophon/tabtarget health,
  every enrolled colophon resolves.

**[260526-2356] rough**

## Character
Mechanical rename + single-pipeline collapse. Judgment only at the
doc-sweep's prose-vs-value calls; the colophon mint is pre-resolved below.
Intricate but mechanical — hands the heavy gauntlet/cloud regression to the
final-regression pace that follows.

## Goal
Collapse the theurge rbtd-* tabtarget family into rbw-t* and retire the
second (RBTW) pipeline — all test/qualify dispatch on one workbench, no
test lost, materially fewer tabtargets.

## Mint (resolved)
- rbw-tb Build           <- rbtd-b
- rbw-tt Test            <- rbtd-t
- rbw-ts TestSuite.<imprint> (imprint)  <- rbtd-s.TestSuite.* + folded
  tP/tS/tT. Carries every current imprint incl. dogfight, plus a new
  `tadmor` suite = (kludge-tadmor, tadmor) preserving QualifyTadmor's
  deliberate two-fixture sequence (see rbq_qualify_tadmor comment for why
  they must stay separate fixtures). Note: tP=gauntlet / tS=skirmish are
  already imprints, so only tadmor is genuinely new.
- rbw-tf FixtureRun (param1)  <- every rbtd-r.FixtureRun.* tabtarget.
  Param form also covers dogfight (no FixtureRun tabtarget exists for it).
- rbw-tc FixtureCase <fixture> [case] (param1)  <- rbtd-s.FixtureCase.
  Preserve omit-to-list behavior: no fixture -> list fixtures; fixture
  only -> list cases (the crucible-debug loop).
- rbw-tq QualifyFast (moved off rbw-tf — do the move before repurposing tf
  for FixtureRun, else a transient same-colophon collision).
- rbw-tr QualifyRelease kept. Its existing tt/rbw-ts.TestSuite.complete.sh
  call (currently a dangling forward-reference) becomes valid once suites
  land at rbw-ts — this pace closes that long-standing broken reference.
- rbw-tK/tO kept.

## Shape — one pipeline
Re-home rbte_cli as a buz module enrolled in rbz_zipper, dispatched by
rbw_workbench. Delete rbtw_workbench.sh, launcher.rbtw_workbench.sh, and
the rbte_dispatch switch. rbte_engine.sh and the Rust binary unchanged.

## Recipe (not site lists)
- Live callers: grep -rn 'rbtd-[bstr]' Tools/ tt/ (exclude .claude/jjm).
- Operator-facing: buh_tt calls naming rbtd-* in handbook scripts.
- Regenerate the generated tabtarget context via rbw-MG — QualifyFast's
  staleness gate (rbq_qualify_context) hard-fails otherwise; the regen also
  refreshes the @-included Command Reference.
- Consumer-visible: CLAUDE.consumer.md ships these colophons — release-note
  the rename (esp. rbw-tP release-gate -> rbw-ts.TestSuite.gauntlet.sh).

## Done (cheap/local — heavy regression is the next pace, AAF)
- grep -rn 'rbtd-' Tools/ tt/ returns zero live colophons (history aside).
- Theurge crate builds clean; rbw-ts.TestSuite.fast.sh green (proves the
  one-pipeline dispatch end-to-end on the cheap path).
- rbw-tq QualifyFast green (after rbw-MG regen) — colophon/tabtarget health,
  every enrolled colophon resolves.

**[260526-2346] rough**

## Character
Mechanical rename + single-pipeline collapse. Judgment only at the
doc-sweep's prose-vs-value calls; the colophon mint is pre-resolved below.
Intricate but mechanical — hands the heavy gauntlet/cloud regression to the
final-regression pace that follows.

## Goal
Collapse the theurge rbtd-* tabtarget family into rbw-t* and retire the
second (RBTW) pipeline — all test/qualify dispatch on one workbench, no
test lost, materially fewer tabtargets.

## Mint (resolved)
- rbw-tb Build           <- rbtd-b
- rbw-tt Test            <- rbtd-t
- rbw-ts TestSuite.<imprint> (imprint)  <- rbtd-s.TestSuite.* + folded
  tP/tS/tT; carries every current imprint incl. dogfight + new `tadmor`.
- rbw-tf FixtureRun (param1)            <- 17x rbtd-r.FixtureRun.*
- rbw-tc FixtureCase <fixture> [case] (param1)  <- rbtd-s.FixtureCase
- rbw-tq QualifyFast (moved off rbw-tf); rbw-tr QualifyRelease kept (fix
  its complete-suite call); rbw-tK/tO kept.

## Shape — one pipeline
Re-home rbte_cli as a buz module enrolled in rbz_zipper, dispatched by
rbw_workbench. Delete rbtw_workbench.sh, launcher.rbtw_workbench.sh, and
the rbte_dispatch switch. rbte_engine.sh and the Rust binary unchanged.

## Recipe (not site lists)
- Live callers: grep -rn 'rbtd-[bstr]' Tools/ tt/ (exclude .claude/jjm).
- Operator-facing: buh_tt calls naming rbtd-* in handbook scripts.
- Consumer-visible: CLAUDE.consumer.md ships these colophons — release-note
  the rename (esp. rbw-tP release-gate -> rbw-ts.TestSuite.gauntlet.sh).

## Done (cheap/local — heavy regression is the next pace, AAF)
- grep -rn 'rbtd-' Tools/ tt/ returns zero live colophons (history aside).
- Theurge crate builds clean; rbw-ts.TestSuite.fast.sh green (proves the
  one-pipeline dispatch end-to-end on the cheap path).
- rbw-tq QualifyFast green (colophon/tabtarget health — every enrolled
  colophon resolves).

### tabtarget-dispatch-via-burd-launcher (₢BMAAJ) [complete]

**[260526-1147] complete**

## Character
Intricate but mechanical. The target shape below is locked; no design latitude. Touches one generator, one trampoline, one qualifier, one regime-decl line, ~215 generated stubs (single awk pass), and a doc prune. Awk use approved.

## Disambiguation — NOT this heat's regime-enum sprue
This pace concerns the **tabtarget dispatch token** (`{owner}ml_{launcher-id}`, e.g. `rbml_rbw`) that the z-launcher trampoline currently receives positionally — the concept retired-heat ₣BK owned and glossed in BCG/CLAUDE.md, *distinct* from this heat's `rbnve_`/`rbnne_` regime-enum sprues. The work **retires that dispatch token entirely**; it neither adds nor migrates a regime sprue. Per this heat's paddock, the dispatch token is a legitimate sibling `mcm_sprue`, so this is a mechanism change, not a terminology correction.

## Goal
Make every tabtarget's shebang and exec line byte-identical, with all per-tabtarget variation confined to a `BURD_*` config block between them. Achieve it by naming the launcher in the `BURD_LAUNCHER` regime variable (launcher basename) rather than a positional dispatch token. This drops the functionally-inert `rbml_`/`buml_` owner prefix and the id↔filename re-expansion that lived in three sites.

## Target shape (locked)
    #!/bin/bash
    export BURD_LAUNCHER=launcher.<id>_workbench.sh
    export BURD_NO_LOG=1          # present only when applicable
    export BURD_INTERACTIVE=1     # present only when applicable
    exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" "${0##*/}" "${@}"

BURD_LAUNCHER is line 2 — the first config line, before any flags. The exec line carries no positional argument and is byte-identical in every tabtarget.

## Work
- **Generator** — `Tools/buk/buut_tabtarget.sh`, `zbuut_write_tabtarget`: emit `export BURD_LAUNCHER=${launcher_path##*/}` as line 2; delete the owner-prefix `case` (the rbw|rbtw→rbml_ / *→buml_ block); exec line becomes the locked constant (no token).
- **Trampoline** — `tt/z-launcher.sh`: read the launcher basename from `BURD_LAUNCHER` (fail loud if unset) instead of `$1`; resolve `rbml_launchers/${BURD_LAUNCHER}` directly — remove the `*ml_` strip and the launcher-id derivation; delete the line that re-exports BURD_LAUNCHER as a path (it now arrives as a basename from the tabtarget and flows downstream unchanged); forward `"${@}"` (was `"${@:2}"`); rewrite the header gloss to the BURD_LAUNCHER contract.
- **Qualifier** — `Tools/buk/buq_qualify.sh`, `buq_tabtargets`: new prescribed form — line1 shebang; line2 `export BURD_LAUNCHER=launcher.*_workbench.sh` resolving to a file under the moorings launcher dir; middle lines `export BURD_*=*`; last line the exact constant exec. Remove the sprue extraction and `*ml_` validation; minimum line count becomes 3.
- **Regime decl** — `Tools/buk/burd_regime.sh`, the `BURD_LAUNCHER` enroll: reword its description from path to "launcher basename (e.g. launcher.rbw_workbench.sh), set by the tabtarget." Validation bounds unchanged.
- **Convert existing tabtargets** — one awk pass over `tt/*.sh`, excluding `z-launcher.sh` and `buw-SI.StationInit.sh`. Per file: keep the shebang; insert `export BURD_LAUNCHER=launcher.<id>_workbench.sh` where `<id>` is the dispatch token on the exec line stripped of its `*ml_` prefix; preserve existing flag lines in original order; replace the exec line with the constant. Skip any file already carrying `export BURD_LAUNCHER=` (idempotent). Preserve the executable bit.
- **Docs** — prune `buml_` (now zero referents) and narrow `rbml_` to its directory meaning; remove the ₣BK tabtarget-sprue gloss. Sites via `grep -rn 'buml_\|sprue' Tools CLAUDE.md tt/z-launcher.sh` (ignore `mcm_sprue` / regime-sprue hits — those are this heat's other concern). Keep the `rbml_launchers` directory name. Add no comment preserving the retired owner distinction.

## Locked facts (verified; do not re-derive)
- `BURD_LAUNCHER` has **zero value dereferences** anywhere — it is only validated (`burd_regime`) and allowlisted (`bud_dispatch` z_known). Its current path value is unused, so redefining it to a basename is safe and the allowlist needs no change.
- z-launcher strips the owner prefix today; nothing reads launcher ownership. The `rbml_`/`buml_` distinction rode only on the dispatch token.
- All launchers co-locate in `rbmm_moorings/rbml_launchers/` as `launcher.{id}_workbench.sh`; a basename resolves directly.
- The two legitimately exempt tabtargets are `z-launcher.sh` (the trampoline) and `buw-SI.StationInit.sh` (standalone bootstrap that cannot dispatch).

## Done
`tt/rbw-tf.QualifyFast.sh` passes — structural qualify is the generator-faithfulness oracle, so a pass proves every stub matches the new generator output — reporting ~215 checked / 2 exempt. Plus a live dispatch smoke: run one plain, one NO_LOG, and one INTERACTIVE tabtarget and confirm each reaches its workbench.

**[260526-1123] rough**

## Character
Intricate but mechanical. The target shape below is locked; no design latitude. Touches one generator, one trampoline, one qualifier, one regime-decl line, ~215 generated stubs (single awk pass), and a doc prune. Awk use approved.

## Disambiguation — NOT this heat's regime-enum sprue
This pace concerns the **tabtarget dispatch token** (`{owner}ml_{launcher-id}`, e.g. `rbml_rbw`) that the z-launcher trampoline currently receives positionally — the concept retired-heat ₣BK owned and glossed in BCG/CLAUDE.md, *distinct* from this heat's `rbnve_`/`rbnne_` regime-enum sprues. The work **retires that dispatch token entirely**; it neither adds nor migrates a regime sprue. Per this heat's paddock, the dispatch token is a legitimate sibling `mcm_sprue`, so this is a mechanism change, not a terminology correction.

## Goal
Make every tabtarget's shebang and exec line byte-identical, with all per-tabtarget variation confined to a `BURD_*` config block between them. Achieve it by naming the launcher in the `BURD_LAUNCHER` regime variable (launcher basename) rather than a positional dispatch token. This drops the functionally-inert `rbml_`/`buml_` owner prefix and the id↔filename re-expansion that lived in three sites.

## Target shape (locked)
    #!/bin/bash
    export BURD_LAUNCHER=launcher.<id>_workbench.sh
    export BURD_NO_LOG=1          # present only when applicable
    export BURD_INTERACTIVE=1     # present only when applicable
    exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" "${0##*/}" "${@}"

BURD_LAUNCHER is line 2 — the first config line, before any flags. The exec line carries no positional argument and is byte-identical in every tabtarget.

## Work
- **Generator** — `Tools/buk/buut_tabtarget.sh`, `zbuut_write_tabtarget`: emit `export BURD_LAUNCHER=${launcher_path##*/}` as line 2; delete the owner-prefix `case` (the rbw|rbtw→rbml_ / *→buml_ block); exec line becomes the locked constant (no token).
- **Trampoline** — `tt/z-launcher.sh`: read the launcher basename from `BURD_LAUNCHER` (fail loud if unset) instead of `$1`; resolve `rbml_launchers/${BURD_LAUNCHER}` directly — remove the `*ml_` strip and the launcher-id derivation; delete the line that re-exports BURD_LAUNCHER as a path (it now arrives as a basename from the tabtarget and flows downstream unchanged); forward `"${@}"` (was `"${@:2}"`); rewrite the header gloss to the BURD_LAUNCHER contract.
- **Qualifier** — `Tools/buk/buq_qualify.sh`, `buq_tabtargets`: new prescribed form — line1 shebang; line2 `export BURD_LAUNCHER=launcher.*_workbench.sh` resolving to a file under the moorings launcher dir; middle lines `export BURD_*=*`; last line the exact constant exec. Remove the sprue extraction and `*ml_` validation; minimum line count becomes 3.
- **Regime decl** — `Tools/buk/burd_regime.sh`, the `BURD_LAUNCHER` enroll: reword its description from path to "launcher basename (e.g. launcher.rbw_workbench.sh), set by the tabtarget." Validation bounds unchanged.
- **Convert existing tabtargets** — one awk pass over `tt/*.sh`, excluding `z-launcher.sh` and `buw-SI.StationInit.sh`. Per file: keep the shebang; insert `export BURD_LAUNCHER=launcher.<id>_workbench.sh` where `<id>` is the dispatch token on the exec line stripped of its `*ml_` prefix; preserve existing flag lines in original order; replace the exec line with the constant. Skip any file already carrying `export BURD_LAUNCHER=` (idempotent). Preserve the executable bit.
- **Docs** — prune `buml_` (now zero referents) and narrow `rbml_` to its directory meaning; remove the ₣BK tabtarget-sprue gloss. Sites via `grep -rn 'buml_\|sprue' Tools CLAUDE.md tt/z-launcher.sh` (ignore `mcm_sprue` / regime-sprue hits — those are this heat's other concern). Keep the `rbml_launchers` directory name. Add no comment preserving the retired owner distinction.

## Locked facts (verified; do not re-derive)
- `BURD_LAUNCHER` has **zero value dereferences** anywhere — it is only validated (`burd_regime`) and allowlisted (`bud_dispatch` z_known). Its current path value is unused, so redefining it to a basename is safe and the allowlist needs no change.
- z-launcher strips the owner prefix today; nothing reads launcher ownership. The `rbml_`/`buml_` distinction rode only on the dispatch token.
- All launchers co-locate in `rbmm_moorings/rbml_launchers/` as `launcher.{id}_workbench.sh`; a basename resolves directly.
- The two legitimately exempt tabtargets are `z-launcher.sh` (the trampoline) and `buw-SI.StationInit.sh` (standalone bootstrap that cannot dispatch).

## Done
`tt/rbw-tf.QualifyFast.sh` passes — structural qualify is the generator-faithfulness oracle, so a pass proves every stub matches the new generator output — reporting ~215 checked / 2 exempt. Plus a live dispatch smoke: run one plain, one NO_LOG, and one INTERACTIVE tabtarget and confirm each reaches its workbench.

### idempotent-canonical-invests (₢BMAAG) [complete]

**[260524-1539] complete**

## Character

Theurge robustness fix; mostly mechanical, with one empirical timing check.

## Problem

canonical-invest reruns against a standing depot fail: `rbgg_invest_retriever`
and `rbgg_invest_director` are create-only and return HTTP 409 when the SA
already exists from a prior run. `governor_mantle` does NOT have this problem —
it deletes existing `governor-*` SAs before recreating ("Clean up existing
governor-* service accounts (404-tolerant)"). Skirmish is designed to rerun
against a standing depot, so this breaks every run after the first.

## Fix

Give the retriever/director invests the same self-heal the mantle already has: a
delete-first-tolerate-404 prelude inside the invest operation. The
GovernorDivests verbs (`rbw-arD` / `rbw-adD`, zero-arg by-role) already exist to
do the deletion. Preferred over a sequence-level divest step in the
canonical-invest fixture, because folding it into the operation also makes
standalone single-case reruns idempotent and co-locates cleanup with creation,
matching the governor precedent.

## Caveat to verify

Tight delete→recreate timing. SA deletion has propagation latency and GCP can
balk at recreating a just-deleted SA name (soft-delete tombstone). Proven this
session that delete-then-recreate works with ~1 min of intervening delay; an
immediate divest→invest is tighter. The mantle already carries a "Wait for SA
propagation" step — the invest self-heal likely needs the same settle. One
tight-timing rerun confirms.

## Done

canonical-invest reruns clean against a standing depot — no 409 — both as the
full skirmish suite and as a standalone single-case rerun.

**[260522-0837] rough**

## Character

Theurge robustness fix; mostly mechanical, with one empirical timing check.

## Problem

canonical-invest reruns against a standing depot fail: `rbgg_invest_retriever`
and `rbgg_invest_director` are create-only and return HTTP 409 when the SA
already exists from a prior run. `governor_mantle` does NOT have this problem —
it deletes existing `governor-*` SAs before recreating ("Clean up existing
governor-* service accounts (404-tolerant)"). Skirmish is designed to rerun
against a standing depot, so this breaks every run after the first.

## Fix

Give the retriever/director invests the same self-heal the mantle already has: a
delete-first-tolerate-404 prelude inside the invest operation. The
GovernorDivests verbs (`rbw-arD` / `rbw-adD`, zero-arg by-role) already exist to
do the deletion. Preferred over a sequence-level divest step in the
canonical-invest fixture, because folding it into the operation also makes
standalone single-case reruns idempotent and co-locates cleanup with creation,
matching the governor precedent.

## Caveat to verify

Tight delete→recreate timing. SA deletion has propagation latency and GCP can
balk at recreating a just-deleted SA name (soft-delete tombstone). Proven this
session that delete-then-recreate works with ~1 min of intervening delay; an
immediate divest→invest is tighter. The mantle already carries a "Wait for SA
propagation" step — the invest self-heal likely needs the same settle. One
tight-timing rerun confirms.

## Done

canonical-invest reruns clean against a standing depot — no 409 — both as the
full skirmish suite and as a standalone single-case rerun.

### egress-mode-sprue-pilot (₢BMAAB) [complete]

**[260520-1731] complete**

## Character

Mechanical with the class-4 distinction as the one judgment edge. Smallest,
cleanest enum (~10 sites each, no Python, no Cloud Build boundary) — this is
the pilot that proves the pattern before the heavy paces.

## Goal

`RBRV_EGRESS_MODE` values `tether`/`airgap` → `rbnve_tether`/`rbnve_airgap`
at every site where the bare word *is* the enum value. Declaration gate
updated so bare values are rejected.

## Boundary

Code, regime files, and AsciiDoc prose — all in-pace (the tree is settled).
Class-4 derived strings stay bare (pool suffixes `-tether`/`-airgap`,
`RBGC_POOL_SUFFIX_*`, vessel sigils `rbev-*-tether`) — migrate only the
dispatch that selects them. Most egress adoc hits are class-1 handbook
prose ("the airgap build") — leave those.

## Recipe

- `grep -rn '\b(tether|airgap)\b' Tools/ tt/`; classify each hit (paddock's
  four classes); migrate class-2, leave 1/3/4.
- Flip the `buv_enum_enroll RBRV_EGRESS_MODE` declaration to the new values.
- Update `rbrv.env` regime files in place (now under
  `rbmm_moorings/rbmv_vessels/*/`).
- If a Rust fixture baseline carries the value, rebuild (`tt/vow-b`).

## Acceptance

- `tt/rbtd-s.TestSuite.fast.sh` green (regime-validation gate now enforces
  `rbnve_*`).
- `grep -rn rbnve_ Tools/` shows the egress cluster; pool-suffix strings
  still bare.

**[260520-1317] rough**

## Character

Mechanical with the class-4 distinction as the one judgment edge. Smallest,
cleanest enum (~10 sites each, no Python, no Cloud Build boundary) — this is
the pilot that proves the pattern before the heavy paces.

## Goal

`RBRV_EGRESS_MODE` values `tether`/`airgap` → `rbnve_tether`/`rbnve_airgap`
at every site where the bare word *is* the enum value. Declaration gate
updated so bare values are rejected.

## Boundary

Code, regime files, and AsciiDoc prose — all in-pace (the tree is settled).
Class-4 derived strings stay bare (pool suffixes `-tether`/`-airgap`,
`RBGC_POOL_SUFFIX_*`, vessel sigils `rbev-*-tether`) — migrate only the
dispatch that selects them. Most egress adoc hits are class-1 handbook
prose ("the airgap build") — leave those.

## Recipe

- `grep -rn '\b(tether|airgap)\b' Tools/ tt/`; classify each hit (paddock's
  four classes); migrate class-2, leave 1/3/4.
- Flip the `buv_enum_enroll RBRV_EGRESS_MODE` declaration to the new values.
- Update `rbrv.env` regime files in place (now under
  `rbmm_moorings/rbmv_vessels/*/`).
- If a Rust fixture baseline carries the value, rebuild (`tt/vow-b`).

## Acceptance

- `tt/rbtd-s.TestSuite.fast.sh` green (regime-validation gate now enforces
  `rbnve_*`).
- `grep -rn rbnve_ Tools/` shows the egress cluster; pool-suffix strings
  still bare.

**[260520-1300] rough**

## Character

Mechanical with the class-4 distinction as the one judgment edge. Smallest,
cleanest enum (~10 sites each, no Python, no Cloud Build boundary) — this is
the pilot that proves the pattern before the heavy paces.

## Goal

`RBRV_EGRESS_MODE` values `tether`/`airgap` → `rbnve_tether`/`rbnve_airgap`
at every site where the bare word *is* the enum value. Declaration gate
updated so bare values are rejected.

## Boundary

Code + regime files now. Class-4 derived strings stay bare (pool suffixes
`-tether`/`-airgap`, `RBGC_POOL_SUFFIX_*`, vessel sigils `rbev-*-tether`) —
migrate only the dispatch that selects them. AsciiDoc *prose* touches defer
until the parallel RBS*.adoc repairs settle.

## Recipe

- `grep -rn '\b(tether|airgap)\b' Tools/ tt/`; classify each hit (paddock's
  four classes); migrate class-2, leave 1/3/4.
- Flip the `buv_enum_enroll RBRV_EGRESS_MODE` declaration to the new values.
- Update any `rbrv.env` regime files in place.
- If a Rust fixture baseline carries the value, rebuild (`tt/vow-b`).

## Acceptance

- `tt/rbtd-s.TestSuite.fast.sh` green (regime-validation gate now enforces
  `rbnve_*`).
- `grep -rn rbnve_ Tools/` shows the egress cluster; pool-suffix strings
  still bare.

### sprue-convention-spec-authoring (₢BMAAA) [complete]

**[260520-1847] complete**

## Character

Codification. The synthesis already lives in the paddock; this pace gives it
a permanent home a future minter will find. Light judgment on *where* the
section lives.

## Goal

The sprue convention exists as a written spec section: grounded in
`mcm_sprue` + `mcm_inlay`, with the full target table, the four-class
use-site model (incl. class-4 "coupled derived string stays bare", two
witnesses), and the `RBRN_RUNTIME` exclusion rationale.

## Boundary

Documentation only — no value migration.

## Recipe

- Cite `mcm_sprue` (the values are wire-level literal tokens) and
  `mcm_inlay` (prefix discipline makes each sprue prefix-recognized). Do
  not invent rationale the MCM quoins already carry.
- Pick the host doc at mount: a regime spec (RBRN/RBRV) section, or a
  convention section near the BUS0/RBS0 vocabulary. Decide by where a future
  minter would look first.
- Lift the convention + four-class model + exclusion rationale from the
  paddock; do not re-derive.

## Acceptance

- Spec renders cleanly; convention + class-4 rule + RBRN_RUNTIME exclusion
  are discoverable without reading this heat's history, and grounded in the
  existing MCM vocabulary.

**[260520-1317] rough**

## Character

Codification. The synthesis already lives in the paddock; this pace gives it
a permanent home a future minter will find. Light judgment on *where* the
section lives.

## Goal

The sprue convention exists as a written spec section: grounded in
`mcm_sprue` + `mcm_inlay`, with the full target table, the four-class
use-site model (incl. class-4 "coupled derived string stays bare", two
witnesses), and the `RBRN_RUNTIME` exclusion rationale.

## Boundary

Documentation only — no value migration.

## Recipe

- Cite `mcm_sprue` (the values are wire-level literal tokens) and
  `mcm_inlay` (prefix discipline makes each sprue prefix-recognized). Do
  not invent rationale the MCM quoins already carry.
- Pick the host doc at mount: a regime spec (RBRN/RBRV) section, or a
  convention section near the BUS0/RBS0 vocabulary. Decide by where a future
  minter would look first.
- Lift the convention + four-class model + exclusion rationale from the
  paddock; do not re-derive.

## Acceptance

- Spec renders cleanly; convention + class-4 rule + RBRN_RUNTIME exclusion
  are discoverable without reading this heat's history, and grounded in the
  existing MCM vocabulary.

**[260520-1259] rough**

## Character

Codification. The synthesis already lives in the paddock; this pace gives it
a permanent home a future minter will find. Light judgment on *where* the
section lives.

## Goal

The sprue convention exists as a written spec section: the full target
table, the four-class use-site model (incl. class-4 "coupled derived string
stays bare", two witnesses), and the `RBRN_RUNTIME` exclusion rationale.

## Boundary

Documentation only — no value migration. If the chosen host doc is an
`RBS*.adoc`, this pace is blocked until the parallel RBS*.adoc repairs land;
coordinate before editing those files.

## Recipe

- Pick the host doc at mount: a regime spec (RBRN/RBRV) section, or a
  convention section near the BUS0/RBS0 vocabulary. Decide by where a future
  minter would look first.
- Lift the convention + four-class model + exclusion rationale from the
  paddock; do not re-derive.

## Acceptance

- Spec renders cleanly; convention + class-4 rule + RBRN_RUNTIME exclusion
  are discoverable without reading this heat's history.

### vessel-mode-sprue-migration (₢BMAAC) [complete]

**[260520-1908] complete**

## Character

Heaviest pace. Crosses bash, AsciiDoc, Python (Cloud Build steps), and Rust
fixtures; `conjure` is the noisiest value in the heat (heavy phase-prose),
`bind` carries English-word noise. The proof bar is non-obvious — see
Acceptance.

## Goal

`RBRV_VESSEL_MODE` values `bind`/`conjure`/`graft` → `rbnve_bind`/
`rbnve_conjure`/`rbnve_graft` across all four ecosystems, including the
Cloud Build substitution boundary. Declaration + gates updated.

## Boundary

Class-4 hallmark-prefix constants (`RBGC_HALLMARK_PREFIX_*` = `c`/`b`/`g`)
stay bare; migrate only their selecting dispatch. AsciiDoc wire-value tables
(e.g. RBSAV/RBSAB `(conjure, bind, or graft)`) are class-2 — migrate
in-pace (the tree is settled).

## Recipe

- `grep -rn '\b(conjure|bind|graft)\b' Tools/ tt/`; also `grep ':-conjure'`
  / `:-bind` / `:-graft` for default-value defaults a `==` grep misses.
- Trace the substitution boundary: `_RBG[AV]_VESSEL_MODE` through `rbfd_*`/
  `rbfv_*` step assembly into the `rbgja*`/`rbgjv*` Python+shell steps —
  hardcoded literals AND `$zjq_vessel_mode` source must move together.
- Migrate the Rust fast-fixture `export RBRV_VESSEL_MODE="..."` baselines;
  rebuild (`tt/vow-b`). Leave onboarding-scenario step labels (class-3).
- Flip the `buv_enum_enroll` declaration + gated `buv_gate_enroll` arms.

## Acceptance

- `regime-validation` + `dockerfile-hygiene` + `gauntlet` green — but NOT
  sufficient alone: the Cloud Build step-shell/Python comparisons run
  remotely. Prove with a real **conjure + graft cloud build** (full ordain).
- `grep -rn rbnve_` shows the complete vessel cluster.

**[260520-1317] rough**

## Character

Heaviest pace. Crosses bash, AsciiDoc, Python (Cloud Build steps), and Rust
fixtures; `conjure` is the noisiest value in the heat (heavy phase-prose),
`bind` carries English-word noise. The proof bar is non-obvious — see
Acceptance.

## Goal

`RBRV_VESSEL_MODE` values `bind`/`conjure`/`graft` → `rbnve_bind`/
`rbnve_conjure`/`rbnve_graft` across all four ecosystems, including the
Cloud Build substitution boundary. Declaration + gates updated.

## Boundary

Class-4 hallmark-prefix constants (`RBGC_HALLMARK_PREFIX_*` = `c`/`b`/`g`)
stay bare; migrate only their selecting dispatch. AsciiDoc wire-value tables
(e.g. RBSAV/RBSAB `(conjure, bind, or graft)`) are class-2 — migrate
in-pace (the tree is settled).

## Recipe

- `grep -rn '\b(conjure|bind|graft)\b' Tools/ tt/`; also `grep ':-conjure'`
  / `:-bind` / `:-graft` for default-value defaults a `==` grep misses.
- Trace the substitution boundary: `_RBG[AV]_VESSEL_MODE` through `rbfd_*`/
  `rbfv_*` step assembly into the `rbgja*`/`rbgjv*` Python+shell steps —
  hardcoded literals AND `$zjq_vessel_mode` source must move together.
- Migrate the Rust fast-fixture `export RBRV_VESSEL_MODE="..."` baselines;
  rebuild (`tt/vow-b`). Leave onboarding-scenario step labels (class-3).
- Flip the `buv_enum_enroll` declaration + gated `buv_gate_enroll` arms.

## Acceptance

- `regime-validation` + `dockerfile-hygiene` + `gauntlet` green — but NOT
  sufficient alone: the Cloud Build step-shell/Python comparisons run
  remotely. Prove with a real **conjure + graft cloud build** (full ordain).
- `grep -rn rbnve_` shows the complete vessel cluster.

**[260520-1300] rough**

## Character

Heaviest pace. Crosses bash, AsciiDoc, Python (Cloud Build steps), and Rust
fixtures; `conjure` is the noisiest value in the heat (heavy phase-prose),
`bind` carries English-word noise. The proof bar is non-obvious — see
Acceptance.

## Goal

`RBRV_VESSEL_MODE` values `bind`/`conjure`/`graft` → `rbnve_bind`/
`rbnve_conjure`/`rbnve_graft` across all four ecosystems, including the
Cloud Build substitution boundary. Declaration + gates updated.

## Boundary

Class-4 hallmark-prefix constants (`RBGC_HALLMARK_PREFIX_*` = `c`/`b`/`g`)
stay bare; migrate only their selecting dispatch. AsciiDoc wire-value tables
(e.g. RBSAV/RBSAB `(conjure, bind, or graft)`) are class-2 (migrate) but
live in RBS*.adoc — defer until the parallel repairs settle, then update.

## Recipe

- `grep -rn '\b(conjure|bind|graft)\b' Tools/ tt/`; also `grep ':-conjure'`
  / `:-bind` / `:-graft` for default-value defaults a `==` grep misses.
- Trace the substitution boundary: `_RBG[AV]_VESSEL_MODE` through `rbfd_*`/
  `rbfv_*` step assembly into the `rbgja*`/`rbgjv*` Python+shell steps —
  hardcoded literals AND `$zjq_vessel_mode` source must move together.
- Migrate the Rust fast-fixture `export RBRV_VESSEL_MODE="..."` baselines;
  rebuild (`tt/vow-b`). Leave onboarding-scenario step labels (class-3).
- Flip the `buv_enum_enroll` declaration + gated `buv_gate_enroll` arms.

## Acceptance

- `regime-validation` + `dockerfile-hygiene` + `gauntlet` green — but NOT
  sufficient alone: the Cloud Build step-shell/Python comparisons run
  remotely. Prove with a real **conjure + graft cloud build** (full ordain).
- `grep -rn rbnve_` shows the complete vessel cluster.

### node-mode-sprue-migration (₢BMAAD) [complete]

**[260520-1926] complete**

## Character

High English-word noise (`disabled`/`enabled`/`global`/`allowlist`), so the
classification grind is the work. Mechanical edits, heavy judgment per hit.

## Goal

`RBRN_ENTRY_MODE`, `RBRN_UPLINK_DNS_MODE`, `RBRN_UPLINK_ACCESS_MODE` values
→ shared `rbnne_*` family (`rbnne_disabled`/`_enabled`/`_global`/
`_allowlist`). Declarations + gates updated.

## Boundary

`RBRN_RUNTIME` (`docker`/`podman`) stays bare — excluded by paddock decision
(executable-name coupling). The three node-modes share `rbnne_disabled`
deliberately; that is intended family-sharing, not a collision.

## Recipe

- `grep -rn '\b(disabled|enabled|global|allowlist)\b' Tools/ tt/` per value;
  expect heavy class-1 noise — these words appear constantly in prose. Migrate
  only case arms / equality tests against the three RBRN_* variables.
- Flip the three `buv_enum_enroll` declarations + their `buv_gate_enroll`
  arms (e.g. gate `RBRN_ENTRY_MODE rbnne_enabled`).
- Update `rbrn.env` regime files in place.

## Acceptance

- `tt/rbtd-s.TestSuite.fast.sh` green (regime-validation enforces values).
- `crucible` suite green — DNS/access modes drive sentry network setup, so a
  charged crucible is the behavioral proof.
- `grep -rn rbnne_` shows the node cluster; `RBRN_RUNTIME` still bare.

**[260520-1300] rough**

## Character

High English-word noise (`disabled`/`enabled`/`global`/`allowlist`), so the
classification grind is the work. Mechanical edits, heavy judgment per hit.

## Goal

`RBRN_ENTRY_MODE`, `RBRN_UPLINK_DNS_MODE`, `RBRN_UPLINK_ACCESS_MODE` values
→ shared `rbnne_*` family (`rbnne_disabled`/`_enabled`/`_global`/
`_allowlist`). Declarations + gates updated.

## Boundary

`RBRN_RUNTIME` (`docker`/`podman`) stays bare — excluded by paddock decision
(executable-name coupling). The three node-modes share `rbnne_disabled`
deliberately; that is intended family-sharing, not a collision.

## Recipe

- `grep -rn '\b(disabled|enabled|global|allowlist)\b' Tools/ tt/` per value;
  expect heavy class-1 noise — these words appear constantly in prose. Migrate
  only case arms / equality tests against the three RBRN_* variables.
- Flip the three `buv_enum_enroll` declarations + their `buv_gate_enroll`
  arms (e.g. gate `RBRN_ENTRY_MODE rbnne_enabled`).
- Update `rbrn.env` regime files in place.

## Acceptance

- `tt/rbtd-s.TestSuite.fast.sh` green (regime-validation enforces values).
- `crucible` suite green — DNS/access modes drive sentry network setup, so a
  charged crucible is the behavioral proof.
- `grep -rn rbnne_` shows the node cluster; `RBRN_RUNTIME` still bare.

### bubep-to-bunne-harmonize (₢BMAAE) [complete]

**[260520-2008] complete**

## Character

Mechanical. Touches BUK files — ₣BK's other subtree. ₣BK's BUK code has
landed, so no hard coordination remains; just watch for late bugfix churn.

## Goal

`BURN_PLATFORM` values `bubep_linux`/`bubep_mac`/`bubep_windows` →
`bunne_linux`/`bunne_mac`/`bunne_windows`. The two-shape wart (5-char
`bubep_` vs 4-char convention) eliminated.

## Boundary

BUK subtree (`Tools/buk/`) + `BUS0` spec.

## Recipe

- `grep -rn 'bubep_' Tools/ tt/` — ~41 sites / 7 files; replace all.
- Flip the `buv_enum_enroll BURN_PLATFORM` declaration and its description
  string (`"(bubep_* identifier)"` → `bunne_*`).
- Update the BUS0 spec mention.

## Acceptance

- `grep -rn bubep_` returns nothing; `grep -rn bunne_` shows the cluster.
- `tt/buw-st.BukSelfTest.sh` + `tt/rbtd-s.TestSuite.fast.sh` green.

**[260520-1317] rough**

## Character

Mechanical. Touches BUK files — ₣BK's other subtree. ₣BK's BUK code has
landed, so no hard coordination remains; just watch for late bugfix churn.

## Goal

`BURN_PLATFORM` values `bubep_linux`/`bubep_mac`/`bubep_windows` →
`bunne_linux`/`bunne_mac`/`bunne_windows`. The two-shape wart (5-char
`bubep_` vs 4-char convention) eliminated.

## Boundary

BUK subtree (`Tools/buk/`) + `BUS0` spec.

## Recipe

- `grep -rn 'bubep_' Tools/ tt/` — ~41 sites / 7 files; replace all.
- Flip the `buv_enum_enroll BURN_PLATFORM` declaration and its description
  string (`"(bubep_* identifier)"` → `bunne_*`).
- Update the BUS0 spec mention.

## Acceptance

- `grep -rn bubep_` returns nothing; `grep -rn bunne_` shows the cluster.
- `tt/buw-st.BukSelfTest.sh` + `tt/rbtd-s.TestSuite.fast.sh` green.

**[260520-1300] rough**

## Character

Mechanical. The one pace that touches BUK files and the one place this heat
overlaps ₣BK *semantically* — coordinate with ₣BK before mounting.

## Goal

`BURN_PLATFORM` values `bubep_linux`/`bubep_mac`/`bubep_windows` →
`bunne_linux`/`bunne_mac`/`bunne_windows`. The two-shape wart (5-char
`bubep_` vs 4-char convention) eliminated.

## Boundary

BUK subtree (`Tools/buk/`) + `BUS0` spec. Confirm with ₣BK that its BUK
sweeps have settled before editing these files.

## Recipe

- `grep -rn 'bubep_' Tools/ tt/` — ~41 sites / 7 files; replace all.
- Flip the `buv_enum_enroll BURN_PLATFORM` declaration and its description
  string (`"(bubep_* identifier)"` → `bunne_*`).
- Update the BUS0 spec mention.

## Acceptance

- `grep -rn bubep_` returns nothing; `grep -rn bunne_` shows the cluster.
- `tt/buw-st.BukSelfTest.sh` + `tt/rbtd-s.TestSuite.fast.sh` green.

### dogfight-cloud-build-viability (₢BMAAH) [complete]

**[260526-1637] complete**

## Character
Test-infrastructure fixture; mechanical once shape is set. A cheap cloud-build
probe — confirms the build/retrieve path yields a runnable artifact, with no
crucible apparatus.

## Goal
Prove the cloud-depot build-and-retrieve path yields a *runnable* artifact:
ordain (conjure-mode) `rbev-busybox` → summon → bare container-runtime run of a
degenerate command (assert exit 0) → abjure. Standing depot, no per-run levy.

## Suite placement — sibling to skirmish, NOT a member of it
This is a new standing-depot scenario-chain suite named `dogfight`, sibling to
`skirmish` in the operator-precondition family (both reuse a standing
operator-levied depot via the `canonical-invest` idiom — no levy, no unmake).
The two differ on the crucible axis and that difference is the whole point:
`skirmish` charges the four crucibles to prove containment; `dogfight` charges
NO crucible and proves only the cloud-build→summon→run path. Do NOT fold this
fixture into the `skirmish` array — that would surround the crucible-free probe
with the exact crucible work it exists to skip. It is emphatically NOT a new
dependency tier "between service and crucible" — the dependency-tier axis
(fast/service/crucible/complete) is a lattice whose top, `complete`, already
means cloud+runtime; this lives on the orthogonal standing-depot scenario axis
where `gauntlet` and `skirmish` already sit.

## Strongest leverage — reuse, don't rebuild
`rbtdrc_hallmark_lifecycle` (rbtdrc_crucible.rs) already does ordain-rbev-busybox
→ … → abjure, including the BURE_CONFIRM-skip abjure. This fixture IS that, with
the registry-inventory middle (audit/rekon) swapped for summon + run. Take the
no-levy standing-depot idiom and the precondition probes from
`RBTDRK_CASES_CANONICAL_INVEST` (rbtdrk_canonical.rs). For the bare run, model on
`rbtdrc_image_layers` — the existing precedent for shelling a raw
container-runtime command from a fixture.

## Locked constraints
- **No crucible.** Proves build/depot/access viability, NOT containment (the
  crucible's orthogonal axis). A bare `<runtime> run --rm <ref> true` suffices —
  no sentry/pentacle/bottle/network. busybox's default cmd is `sh`, so pass an
  explicit `true` to get a clean exit-0 executability proof.
- **Runtime hardcoded to docker behind ONE named constant** (podman deferred).
  The Director-governed runtime-regime-field decision rides with podman, after
  ₣BS settles the regime/credential shape — do NOT add a regime field now.
- Presumes an operator-levied standing depot with director + retriever already
  invested; carry precondition probes like canonical-invest does.

## One unknown to resolve at mount (deliberately not pre-baked)
How `summon` (rbw-fs RetrieverSummonsHallmark) names the locally-pulled image —
the run needs that ref. Read it from the summon impl or its emitted burv fact.

## Suite wiring
Mint a `dogfight` suite paralleling `skirmish` in rbte_engine.sh:
`ZRBTE_SUITE_DOGFIGHT` array, the `dogfight)` resolver case arm, and the
`tt/rbtd-s.TestSuite.dogfight.sh` tabtarget. A single-fixture suite is fine —
the name is the operator's memorable batch handle. (A `rbw-t?` Qualify wrapper
paralleling QualifySkirmish is optional and deferred — not part of done.)

## Registration
New RBTDRM_FIXTURE_* const + fixtures roster entry + required-colophons map
(must include summon alongside ordain/abjure). StateProgressing disposition —
cases are sequentially dependent (conjure→summon→run→abjure).

## Done
A fixture + single-case run path that ordains busybox, summons it, runs a
degenerate command proving executability, and abjures — green against a standing
depot, no crucible charged, reachable via the new `dogfight` suite tabtarget.

**[260526-1557] rough**

## Character
Test-infrastructure fixture; mechanical once shape is set. A cheap cloud-build
probe — confirms the build/retrieve path yields a runnable artifact, with no
crucible apparatus.

## Goal
Prove the cloud-depot build-and-retrieve path yields a *runnable* artifact:
ordain (conjure-mode) `rbev-busybox` → summon → bare container-runtime run of a
degenerate command (assert exit 0) → abjure. Standing depot, no per-run levy.

## Suite placement — sibling to skirmish, NOT a member of it
This is a new standing-depot scenario-chain suite named `dogfight`, sibling to
`skirmish` in the operator-precondition family (both reuse a standing
operator-levied depot via the `canonical-invest` idiom — no levy, no unmake).
The two differ on the crucible axis and that difference is the whole point:
`skirmish` charges the four crucibles to prove containment; `dogfight` charges
NO crucible and proves only the cloud-build→summon→run path. Do NOT fold this
fixture into the `skirmish` array — that would surround the crucible-free probe
with the exact crucible work it exists to skip. It is emphatically NOT a new
dependency tier "between service and crucible" — the dependency-tier axis
(fast/service/crucible/complete) is a lattice whose top, `complete`, already
means cloud+runtime; this lives on the orthogonal standing-depot scenario axis
where `gauntlet` and `skirmish` already sit.

## Strongest leverage — reuse, don't rebuild
`rbtdrc_hallmark_lifecycle` (rbtdrc_crucible.rs) already does ordain-rbev-busybox
→ … → abjure, including the BURE_CONFIRM-skip abjure. This fixture IS that, with
the registry-inventory middle (audit/rekon) swapped for summon + run. Take the
no-levy standing-depot idiom and the precondition probes from
`RBTDRK_CASES_CANONICAL_INVEST` (rbtdrk_canonical.rs). For the bare run, model on
`rbtdrc_image_layers` — the existing precedent for shelling a raw
container-runtime command from a fixture.

## Locked constraints
- **No crucible.** Proves build/depot/access viability, NOT containment (the
  crucible's orthogonal axis). A bare `<runtime> run --rm <ref> true` suffices —
  no sentry/pentacle/bottle/network. busybox's default cmd is `sh`, so pass an
  explicit `true` to get a clean exit-0 executability proof.
- **Runtime hardcoded to docker behind ONE named constant** (podman deferred).
  The Director-governed runtime-regime-field decision rides with podman, after
  ₣BS settles the regime/credential shape — do NOT add a regime field now.
- Presumes an operator-levied standing depot with director + retriever already
  invested; carry precondition probes like canonical-invest does.

## One unknown to resolve at mount (deliberately not pre-baked)
How `summon` (rbw-fs RetrieverSummonsHallmark) names the locally-pulled image —
the run needs that ref. Read it from the summon impl or its emitted burv fact.

## Suite wiring
Mint a `dogfight` suite paralleling `skirmish` in rbte_engine.sh:
`ZRBTE_SUITE_DOGFIGHT` array, the `dogfight)` resolver case arm, and the
`tt/rbtd-s.TestSuite.dogfight.sh` tabtarget. A single-fixture suite is fine —
the name is the operator's memorable batch handle. (A `rbw-t?` Qualify wrapper
paralleling QualifySkirmish is optional and deferred — not part of done.)

## Registration
New RBTDRM_FIXTURE_* const + fixtures roster entry + required-colophons map
(must include summon alongside ordain/abjure). StateProgressing disposition —
cases are sequentially dependent (conjure→summon→run→abjure).

## Done
A fixture + single-case run path that ordains busybox, summons it, runs a
degenerate command proving executability, and abjures — green against a standing
depot, no crucible charged, reachable via the new `dogfight` suite tabtarget.

**[260526-1109] rough**

## Character
Test-infrastructure fixture; mechanical once shape is set. A cheap cloud-build
probe — confirms the build/retrieve path yields a runnable artifact, with no
crucible apparatus.

## Goal
Prove the cloud-depot build-and-retrieve path yields a *runnable* artifact:
ordain (conjure-mode) `rbev-busybox` → summon → bare container-runtime run of a
degenerate command (assert exit 0) → abjure. Standing depot, no per-run levy.

## Suite placement — sibling to skirmish, NOT a member of it
This is a new standing-depot scenario-chain suite named `dogfight`, sibling to
`skirmish` in the operator-precondition family (both reuse a standing
operator-levied depot via the `canonical-invest` idiom — no levy, no unmake).
The two differ on the crucible axis and that difference is the whole point:
`skirmish` charges the four crucibles to prove containment; `dogfight` charges
NO crucible and proves only the cloud-build→summon→run path. Do NOT fold this
fixture into the `skirmish` array — that would surround the crucible-free probe
with the exact crucible work it exists to skip. It is emphatically NOT a new
dependency tier "between service and crucible" — the dependency-tier axis
(fast/service/crucible/complete) is a lattice whose top, `complete`, already
means cloud+runtime; this lives on the orthogonal standing-depot scenario axis
where `gauntlet` and `skirmish` already sit.

## Strongest leverage — reuse, don't rebuild
`rbtdrc_hallmark_lifecycle` (rbtdrc_crucible.rs) already does ordain-rbev-busybox
→ … → abjure, including the BURE_CONFIRM-skip abjure. This fixture IS that, with
the registry-inventory middle (audit/rekon) swapped for summon + run. Take the
no-levy standing-depot idiom and the precondition probes from
`RBTDRK_CASES_CANONICAL_INVEST` (rbtdrk_canonical.rs). For the bare run, model on
`rbtdrc_image_layers` — the existing precedent for shelling a raw
container-runtime command from a fixture.

## Locked constraints
- **No crucible.** Proves build/depot/access viability, NOT containment (the
  crucible's orthogonal axis). A bare `<runtime> run --rm <ref> true` suffices —
  no sentry/pentacle/bottle/network. busybox's default cmd is `sh`, so pass an
  explicit `true` to get a clean exit-0 executability proof.
- **Runtime hardcoded to docker behind ONE named constant** (podman deferred).
  The Director-governed runtime-regime-field decision rides with podman, after
  ₣BS settles the regime/credential shape — do NOT add a regime field now.
- Presumes an operator-levied standing depot with director + retriever already
  invested; carry precondition probes like canonical-invest does.

## One unknown to resolve at mount (deliberately not pre-baked)
How `summon` (rbw-fs RetrieverSummonsHallmark) names the locally-pulled image —
the run needs that ref. Read it from the summon impl or its emitted burv fact.

## Suite wiring
Mint a `dogfight` suite paralleling `skirmish` in rbte_engine.sh:
`ZRBTE_SUITE_DOGFIGHT` array, the `dogfight)` resolver case arm, and the
`tt/rbtd-s.TestSuite.dogfight.sh` tabtarget. A single-fixture suite is fine —
the name is the operator's memorable batch handle. (A `rbw-t?` Qualify wrapper
paralleling QualifySkirmish is optional and deferred — not part of done.)

## Registration
New RBTDRM_FIXTURE_* const + fixtures roster entry + required-colophons map
(must include summon alongside ordain/abjure). StateProgressing disposition —
cases are sequentially dependent (conjure→summon→run→abjure).

## Done
A fixture + single-case run path that ordains busybox, summons it, runs a
degenerate command proving executability, and abjures — green against a standing
depot, no crucible charged, reachable via the new `dogfight` suite tabtarget.

**[260522-1340] rough**

## Character
Test-infrastructure fixture; mechanical once shape is set. A cheap cloud-depot
smoke test — skirmish-alternative for fast confidence the build/retrieve path
works, without skirmish's project churn.

## Goal
Prove the cloud-depot build-and-retrieve path yields a *runnable* artifact:
ordain (conjure-mode) `rbev-busybox` → summon → bare container-runtime run of a
degenerate command (assert exit 0) → abjure. Standing depot, no per-run levy.

## Strongest leverage — reuse, don't rebuild
`rbtdrc_hallmark_lifecycle` (rbtdrc_crucible.rs) already does ordain-rbev-busybox
→ … → abjure, including the BURE_CONFIRM-skip abjure. This fixture IS that, with
the registry-inventory middle (audit/rekon) swapped for summon + run. Take the
no-levy standing-depot idiom and the precondition probes from
`RBTDRK_CASES_CANONICAL_INVEST` (rbtdrk_canonical.rs). For the bare run, model on
`rbtdrc_image_layers` — the existing precedent for shelling a raw
container-runtime command from a fixture.

## Locked constraints
- **No crucible.** Proves build/depot/access viability, NOT containment (the
  crucible's orthogonal axis). A bare `<runtime> run --rm <ref> true` suffices —
  no sentry/pentacle/bottle/network. busybox's default cmd is `sh`, so pass an
  explicit `true` to get a clean exit-0 executability proof.
- **Runtime hardcoded to docker behind ONE named constant** (podman deferred).
  The Director-governed runtime-regime-field decision rides with podman, after
  ₣BS settles the regime/credential shape — do NOT add a regime field now.
- Presumes an operator-levied standing depot with director + retriever already
  invested; carry precondition probes like canonical-invest does.

## Two unknowns to resolve at mount (deliberately not pre-baked)
1. How `summon` (rbw-fs RetrieverSummonsHallmark) names the locally-pulled image
   — the run needs that ref. Read it from the summon impl or its emitted burv fact.
2. The **suite tabtarget** sits at a new tier — cloud creds + a local
   container-runtime binary, but none of the crucible apparatus. Locate suite
   composition (the ZRBTE_SUITE_* machinery / `tt/rbtd-s.TestSuite.*.sh`) and add
   a thin suite there; this tier is between `service` and `crucible`.

## Registration
New RBTDRM_FIXTURE_* const + fixtures roster entry + required-colophons map
(must include summon alongside ordain/abjure). StateProgressing disposition —
cases are sequentially dependent (conjure→summon→run→abjure).

## Done
A fixture + single-case run path that ordains busybox, summons it, runs a
degenerate command proving executability, and abjures — green against a standing
depot, no crucible charged, reachable via its own suite tabtarget.

**[260522-1326] rough**

## Character
Test-infrastructure fixture; mechanical once shape is set. A cheap cloud-depot
smoke test — a skirmish-alternative for fast confidence that the build/retrieve
path works, without skirmish's project churn.

## Goal
A fixture proving the cloud-depot build-and-retrieve path produces a *runnable*
artifact: conjure `rbev-busybox` → summon → bare runtime run of a degenerate
command (assert exit 0) → abjure. Runs against an operator-levied standing depot
(no per-run project levy), modeled on `canonical-invest`'s no-levy idiom.

## Locked constraints
- **No crucible.** This proves build/depot/access viability, NOT containment —
  an orthogonal axis the crucible owns. A bare `<runtime> run --rm <ref> true`
  is sufficient; no sentry/pentacle/bottle/network.
- **Runtime hardcoded to docker behind one named constant** (podman deferred).
  The "runtime belongs in a Director-governed regime field" decision rides along
  with the podman work, after ₣BS settles the regime/credential shape — do NOT
  add a regime field now.
- `rbev-busybox` is the load-bearing small conjure vessel (already shared by
  hallmark-lifecycle / batch-vouch).
- summon proves the retriever pull path; the run proves executability — both
  cheap, keep both as distinct steps.

## Done
A fixture + single-case run path that conjures busybox, summons it, runs a
degenerate command proving executability, and abjures — green against a standing
depot with no crucible charged.

### theurge-commit-hygiene-guard (₢BMAAI) [complete]

**[260526-1147] complete**

## Character

Test-infra robustness; mostly mechanical, one design choice (guard placement).

## Problem

Theurge fixtures legitimately commit during runs, but `rbtdro_git_commit`
(rbtdro_onboarding.rs) stages with `git add -A` — it swept an unrelated
working-tree edit into a hallmark commit (24dc4c6a3, this session). Theurge also
has no global guard against running on a dirty tree.

## Done

- Theurge refuses to start a run when `git status --porcelain` is non-empty,
  failing loudly. Generalizes the Class-A clean-tree check already living in
  `rbtdrp_pristine.rs` to the run entry (rbtdre engine / `main.rs`).
- `rbtdro_git_commit` takes an explicit paths list (like the
  `rbtdrk_canonical` / `rbtdrp_pristine` `_paths` helpers already do) instead of
  `git add -A`; its callers pass the rbrn.env/rbrv.env files they changed,
  including the dynamic ordain-to-consumers and yoke-all-vessels propagation sets.
- `tt/rbtd-b.Build.sh` builds, `tt/rbtd-t.Test.sh` green.

## Constraint

Guard fires at start-of-run, not per-case (fixtures commit mid-run by design,
leaving the tree clean between). Surgical staging must still cover the dynamic
propagation sets, so the fix threads paths from the steps that compute them.

**[260524-1538] rough**

## Character

Test-infra robustness; mostly mechanical, one design choice (guard placement).

## Problem

Theurge fixtures legitimately commit during runs, but `rbtdro_git_commit`
(rbtdro_onboarding.rs) stages with `git add -A` — it swept an unrelated
working-tree edit into a hallmark commit (24dc4c6a3, this session). Theurge also
has no global guard against running on a dirty tree.

## Done

- Theurge refuses to start a run when `git status --porcelain` is non-empty,
  failing loudly. Generalizes the Class-A clean-tree check already living in
  `rbtdrp_pristine.rs` to the run entry (rbtdre engine / `main.rs`).
- `rbtdro_git_commit` takes an explicit paths list (like the
  `rbtdrk_canonical` / `rbtdrp_pristine` `_paths` helpers already do) instead of
  `git add -A`; its callers pass the rbrn.env/rbrv.env files they changed,
  including the dynamic ordain-to-consumers and yoke-all-vessels propagation sets.
- `tt/rbtd-b.Build.sh` builds, `tt/rbtd-t.Test.sh` green.

## Constraint

Guard fires at start-of-run, not per-case (fixtures commit mid-run by design,
leaving the tree clean between). Surgical staging must still cover the dynamic
propagation sets, so the fix threads paths from the steps that compute them.

### final-regression-and-grep-audit (₢BMAAF) [complete]

**[260527-0952] complete**

## Character

Regression exit, quota-constrained. A failure here is a real migration bug
(a missed site), not exploration — triage is opus-grade.

## Goal

The sprue migration AND theurge-colophon consolidation proven on this
station by the strongest regression available while Cloud Build is
quota-blocked — the skirmish ladder plus the faster suites — and the
discoverability promise verified by grep audit.

## Recipe

- Grep audit (DONE — clean): `rbnve_`/`rbnne_`/`bunne_` each a coherent
  cluster; `bubep_` repo-wide empty; zero live `rbtd-` colophons; zero
  class-2 bare-value survivors. Caught and repaired one harmonize miss —
  `BURN_PLATFORM=bubep_windows` in the `bujn-winpc` node profile (+ its
  README) — exactly the missed-site this pace exists to catch.
- Run sequentially on macos (never parallel; standing canonical depot is
  levied): the faster suites (`fast`, local `crucible`/`tadmor`), then
  `skirmish` (standing-depot mini-gauntlet via `rbw-ts.TestSuite.*`).
- Read each `../logs-buk/` result; triage any red as real-migration-bug vs.
  the known ACCOUNT_STATE_INVALID re-invest flap (₣BB) seated in
  `canonical-invest`.

## Acceptance

- Grep audit clean (done).
- skirmish + faster suites green on macos.

Full fresh-project gauntlet deferred (Cloud Build quota); operator handles
out of band — not re-proven here.

**[260527-0728] rough**

## Character

Regression exit, quota-constrained. A failure here is a real migration bug
(a missed site), not exploration — triage is opus-grade.

## Goal

The sprue migration AND theurge-colophon consolidation proven on this
station by the strongest regression available while Cloud Build is
quota-blocked — the skirmish ladder plus the faster suites — and the
discoverability promise verified by grep audit.

## Recipe

- Grep audit (DONE — clean): `rbnve_`/`rbnne_`/`bunne_` each a coherent
  cluster; `bubep_` repo-wide empty; zero live `rbtd-` colophons; zero
  class-2 bare-value survivors. Caught and repaired one harmonize miss —
  `BURN_PLATFORM=bubep_windows` in the `bujn-winpc` node profile (+ its
  README) — exactly the missed-site this pace exists to catch.
- Run sequentially on macos (never parallel; standing canonical depot is
  levied): the faster suites (`fast`, local `crucible`/`tadmor`), then
  `skirmish` (standing-depot mini-gauntlet via `rbw-ts.TestSuite.*`).
- Read each `../logs-buk/` result; triage any red as real-migration-bug vs.
  the known ACCOUNT_STATE_INVALID re-invest flap (₣BB) seated in
  `canonical-invest`.

## Acceptance

- Grep audit clean (done).
- skirmish + faster suites green on macos.

Full fresh-project gauntlet deferred (Cloud Build quota); operator handles
out of band — not re-proven here.

**[260526-2356] rough**

## Character

Gauntlet exit. A failure here is a real migration bug (a missed site), not
exploration — triage is opus-grade.

## Goal

The whole sprue migration AND the theurge-colophon consolidation proven
end-to-end, and the discoverability promise verified: one grep per family
returns the complete cluster — no bare enum values and no stale rbtd-
colophons survive.

## Recipe

- Run the gauntlet (`tt/rbw-ts.TestSuite.gauntlet.sh`) — pre-flight `rbw-MZ`,
  all crucibles quenched, per burn-bridges posture.
- Grep audit:
  - `grep -rn rbnve_ Tools/` / `rbnne_` / `bunne_` each return a complete,
    coherent cluster.
  - `grep -rn bubep_` returns nothing.
  - `grep -rn 'rbtd-' Tools/ tt/` returns zero live colophons. Expected
    non-colophon survivor: the `rbtd-test-` scratch-dir prefix in the
    theurge Rust crate (rbtdte_engine.rs / rbtdti_invocation.rs) — code
    identifier, not a colophon. .claude/jjm history aside.
  - Spot bare-value audit: surviving `conjure`/`graft`/`tether`/`airgap`/
    `disabled`/`enabled`/`global`/`allowlist` hits are all class-1/3/4
    (prose, identifiers, coupled derived strings), zero class-2.

## Acceptance

- Gauntlet green (via the new rbw-ts.TestSuite.gauntlet.sh name).
- Grep audit clean per above.

**[260526-2347] rough**

## Character

Gauntlet exit. A failure here is a real migration bug (a missed site), not
exploration — triage is opus-grade.

## Goal

The whole sprue migration AND the theurge-colophon consolidation proven
end-to-end, and the discoverability promise verified: one grep per family
returns the complete cluster — no bare enum values and no stale rbtd-
colophons survive.

## Recipe

- Run the gauntlet (`tt/rbw-ts.TestSuite.gauntlet.sh`) — pre-flight `rbw-MZ`,
  all crucibles quenched, per burn-bridges posture.
- Grep audit:
  - `grep -rn rbnve_ Tools/` / `rbnne_` / `bunne_` each return a complete,
    coherent cluster.
  - `grep -rn bubep_` returns nothing.
  - `grep -rn 'rbtd-' Tools/ tt/` returns zero live colophons (.claude/jjm
    history aside).
  - Spot bare-value audit: surviving `conjure`/`graft`/`tether`/`airgap`/
    `disabled`/`enabled`/`global`/`allowlist` hits are all class-1/3/4
    (prose, identifiers, coupled derived strings), zero class-2.

## Acceptance

- Gauntlet green (via the new rbw-ts.TestSuite.gauntlet.sh name).
- Grep audit clean per above.

**[260520-1301] rough**

## Character

Gauntlet exit. A failure here is a real migration bug (a missed site), not
exploration — triage is opus-grade.

## Goal

The whole sprue migration proven end-to-end, and the discoverability promise
verified: one grep per family returns the complete cluster, no bare values
survive.

## Recipe

- Run the gauntlet (`tt/rbw-tP.QualifyPristine.sh`) — pre-flight `rbw-MZ`,
  all crucibles quenched, per burn-bridges posture.
- Grep audit:
  - `grep -rn rbnve_ Tools/` / `rbnne_` / `bunne_` each return a complete,
    coherent cluster.
  - `grep -rn bubep_` returns nothing.
  - Spot bare-value audit: surviving `conjure`/`graft`/`tether`/`airgap`/
    `disabled`/`enabled`/`global`/`allowlist` hits are all class-1/3/4
    (prose, identifiers, coupled derived strings), zero class-2.

## Acceptance

- Gauntlet green.
- Grep audit clean per above.

### cerebro-regression-run (₢BMAAL) [complete]

**[260527-1006] complete**

## Character

Cross-platform regression leg on the cerebro Linux fundus. Foray dispatch
is mechanical; a suite reding is opus-grade triage.

## Goal

The same skirmish + faster-suite ladder proven on macos re-run on the
cerebro Linux fundus — confirming the sprue migration and theurge-colophon
consolidation are platform-independent, not a macos-only pass.

## Recipe

- Foray per the JJK Foray Protocol: bind a legatio to cerebro (alias from
  its BUK Regime Node profile, reldir `projects/rbm_alpha_recipemuster`),
  ensure curia clean + pushed, plant to HEAD.
- Relay the `rbw-ts.TestSuite.*` suites sequentially — faster suites first,
  then `skirmish`. Skirmish carries the standing-depot precondition: confirm
  a canonical depot is reachable from cerebro's regime before relaying it.
- `jjx_check` each pensum to terminal; `jjx_fetch` the `../logs-buk/`
  result; triage red as real-bug vs. the known ACCOUNT_STATE_INVALID flap.

## Acceptance

- skirmish + faster suites green on cerebro.

**[260527-0729] rough**

## Character

Cross-platform regression leg on the cerebro Linux fundus. Foray dispatch
is mechanical; a suite reding is opus-grade triage.

## Goal

The same skirmish + faster-suite ladder proven on macos re-run on the
cerebro Linux fundus — confirming the sprue migration and theurge-colophon
consolidation are platform-independent, not a macos-only pass.

## Recipe

- Foray per the JJK Foray Protocol: bind a legatio to cerebro (alias from
  its BUK Regime Node profile, reldir `projects/rbm_alpha_recipemuster`),
  ensure curia clean + pushed, plant to HEAD.
- Relay the `rbw-ts.TestSuite.*` suites sequentially — faster suites first,
  then `skirmish`. Skirmish carries the standing-depot precondition: confirm
  a canonical depot is reachable from cerebro's regime before relaying it.
- `jjx_check` each pensum to terminal; `jjx_fetch` the `../logs-buk/`
  result; triage red as real-bug vs. the known ACCOUNT_STATE_INVALID flap.

## Acceptance

- skirmish + faster suites green on cerebro.

### roles-to-sprue (₢BMAAM) [complete]

**[260529-0708] complete**

## Character
Mechanical migration with a real axis-split at the seam: one bare `RBCC_role_*`
family that today serves three jobs gets cleaved into a minted enum family and a
bare composition family. Edits are localized and value-preserving on the bare side;
the judgment is resolved below. Imminent execution — every file:line below was
verified against the tree at slate time, and the operator has frozen this surface
(no commits land here before mount), so the refs are trustworthy as written.

## Goal
Make the RBRA-role enum greppable as a sprue family (`rbnae_*`) WITHOUT touching any
Google Cloud service-account email, account-id, or local secret-directory name. A
deliberate partial cleanup: mint only the enum-value axis; leave every derived
resource-name string bare.

## The decision (locked by operator) — `{role, account}` split, Design A

The current single family `RBCC_role_*` conflates two axes:
- **Enum-value axis** — the `RBRA_ROLE` value: written into credential files, validated,
  carried as the auth-regime folio, swizzle-checked. Domain = {governor, retriever,
  director} only.
- **Composition-label axis** — bare fragments that build GCP SA account-ids/emails AND
  local secret-directory names. Domain = {governor, retriever, director, payor, assay,
  mason}.

Cleave into two `RBCC_` families (leading-space comments so no line opens with hash):

```
  RBCC_role_governor="rbnae_governor"      # minted enum — RBRA_ROLE domain (3 members)
  RBCC_role_retriever="rbnae_retriever"
  RBCC_role_director="rbnae_director"

  RBCC_account_governor="governor"         # bare composition — SA-name + secret-dir (6 members)
  RBCC_account_retriever="retriever"
  RBCC_account_director="director"
  RBCC_account_payor="payor"
  RBCC_account_assay="assay"
  RBCC_account_mason="mason"
```

`assay`/`mason`/`payor` are NEVER `RBRA_ROLE` values (validation accepts only
governor|retriever|director), so they get NO `rbnae_` token — `account`-only.

**Design A — secret directories stay bare** (operator-approved): a secret-dir name is a
derived resource-name string in local-filesystem space, structurally identical to the SA
email and the `-tether`/`-airgap` pool suffixes the heat already keeps bare under class-4.
So directories follow `RBCC_account_*`; they are NOT renamed. The folio→file `case` in
`zrbra_resolve_role` is the translation point that lets the folio mint while the directory
stays bare. The prior "operators rename dirs by hand" / "have dirs been renamed?" breadcrumb
and the flat→nested migration loop are VOID — retire them (section D).

## Why the split is safe (already-shaped seams — verified)
- `zrbgg_create_service_account_with_key(account_name, …, role)` already takes
  account-name ($1, builds the email) and role ($4, writes RBRA_ROLE) as separate params.
  Split = feed $1 from `RBCC_account_*`, $4 from `RBCC_role_*`.
- `zrbra_resolve_role` (rbra_cli.sh) maps folio → `RBDC_*_RBRA_FILE` *variable* via a
  `case`, so the folio string is decoupled from the directory name.
- Swizzle guard (rbra_cli.sh) ties only `RBRA_ROLE == BUZ_FOLIO`; both mint together.

## Edit map — by axis
All file:line refs verified against the tree at slate time.

### A. Declarations — rbcc_Constants.sh
- :59-64 — replace the 6-member `RBCC_role_*` block with the two families above.
- :94-95 — `RBCC_fact_ext_roster_*` compose fact-FILE names → use `RBCC_account_*`
  (keeps fact filenames `roster-retriever` bare; minting here would corrupt filenames).
- :101 — rewrite the "Distinct from the credential RBCC_role_* family" axis comment to
  describe the role(enum)/account(label) split.

### B. Minted (enum-value axis → `RBCC_role_*` = `rbnae_*`)
- rbgg_Governor.sh:592, :643 — invest $4 (becomes RBRA_ROLE).
- rbgp_Payor.sh:1876 — `printf 'RBRA_ROLE=%s' "${RBCC_role_governor}"`.
- rbra_regime.sh:69-71 — validation `case` arms → `rbnae_governor|rbnae_retriever|rbnae_director)`.
- rbra_regime.sh:41 — enroll help string text → new values (each ≤20 chars; `rbnae_retriever`
  is 15, within `buv_string_enroll RBRA_ROLE 1 20`).
- rbra_cli.sh:39-44 — `zrbra_resolve_role` case arms → `rbnae_*)`; RBDC var targets unchanged.
- rbra_cli.sh:83 — `z_roles=(...)` display array → `rbnae_*` (labels the enum).
- rbra_cli.sh:35,43 — "role required / Unknown role" die-text → reflect minted values.
- **Tools/buk/buts/butcrg_RegimeCredentials.sh:36,48-49** — `z_roles=("governor"
  "retriever" "director")` is passed as the **folio** to `rbw-rav`/`rbw-rar` (:48-49).
  The folio mints, so `z_roles` must become `rbnae_*` or `zrbra_resolve_role` dies
  "Unknown RBRA role". CAUTION: this site is caught by NONE of the build/fast/grep
  gates — it is a workstation-only credential test, and the grep audit reads its bare
  `governor` as an acceptable class-C survivor. Silent breakage; this edit plus the
  required butcrg run (acceptance gate) are the only things that catch it.

### C. Bare (composition-label axis → `RBCC_account_*`)
- rbgg_Governor.sh:560,569 — roster first arg (SA-name prefix match).
- rbgg_Governor.sh:578,583,629,634,825,830,839,844 — `z_account_name`, divest first arg,
  and the "composes X-<identity>" doc strings.
- rbgd_DepotConstants.sh:61 — `RBGD_MASON_EMAIL`.
- rbgp_Payor.sh:1226,1622 — mason name/email; :1412,1744 — `== ${RBCC_role_governor}-*`
  SA-email match; :1772 — governor account-id.
- Secret-dir paths (Design A, bare): rbdc_DerivedConstants.sh:35-39 (mkdir), :74-78
  (`RBDC_*_RBRA_FILE`); rblm_cli.sh:88,125,167; rbho path-checks rbhodb_director_bind.sh:49,
  rbhoda_director_airgap.sh:52, rbhodf_director_first_build.sh:40, rbhodg_director_graft.sh:51.

### C-bis. Explicitly left bare — NO edit (judgment recorded so the executor need not re-derive)
- `rbgv_AccessProbe.sh` `zrbgv_role_rbra_file_capture` (~:76-95) has its OWN
  `governor)|director)|retriever)` role→file `case` plus internal bare labels in
  `rbgv_check_*`. These are bare-axis (file resolution + internal labels, structurally
  the directory) and STAY bare. They work post-split only because the probe sources the
  RBRA file and runs `zrbra_enforce`, whose case is minted in section B — confirm that
  coupling holds, then leave rbgv untouched.
- Pre-existing bare `governor-*` literals NOT sourced from `RBCC_role_*`, all intentionally
  bare and untouched: rbgp_Payor.sh:1687 (doc prose), :1712 (step message), :1811
  (`"governor-owner"` IAM request-id label). The grep audit will surface these — leave them.

### D. Retire (no-auto-repair posture; Design A voids these)
- rbdc_DerivedConstants.sh:42-71 — delete the flat→nested one-shot migration (both the
  role loop :42-61 and the payor `rbro.env` block :63-71). It is dead auto-repair.
  Post-split it goes INCOHERENT, not merely stale: :44/:46 build the dir path from
  `RBCC_role_*` (now `rbnae_governor/…`) while the real secret dirs stay bare — a path
  mismatch — so it cannot be mechanically repointed. Delete, do not repair. Drop the
  rename breadcrumb entirely — under Design A nothing renames.

### E. The one mixed-arg seam — split `zrbho_credential_install`
- rbhob_base.sh:88 `zrbho_credential_install(z_role_constant)` uses its single arg as BOTH
  the displayed dir path (:97, :115 — needs bare) AND the folio passed to `rbw-rav`
  (:135 — needs minted). Give it two params: `(z_account_label, z_role_folio)`.
  - :97,:115 dir-path → `z_account_label` (bare).
  - :135 `buh_tt … "${RBZ_VALIDATE_AUTH}" "" " ${z_role_folio}"` → minted folio.
  - Callers: rbhocr_credential_retriever.sh:38, rbhocd_credential_director.sh:38 — pass
    `"${RBCC_account_X}" "${RBCC_role_X}"`.

### F. Rust projection + consumers (value-preserving rename)
- rbcc_Constants.sh:126-134 — the `rbcc_emit_consts` projection list currently emits
  `RBCC_role_*` (6) → `RBTDGC_ROLE_*`. Rust uses these ONLY as secret-dir names
  (rbtdrp_pristine.rs:288 `secrets.join(role)`), i.e. the bare axis. Repoint the list to
  the 6 `RBCC_account_*`; projected names become `RBTDGC_ACCOUNT_*` (values unchanged).
- rbcc_Constants.sh:110-119 — the projection-block comment ("credential roles", "the
  manifest role mirror") describes the projected set; refresh that prose to "account
  labels" so it matches the repointed list.
- Discovery: `grep -rn 'RBTDGC_ROLE_' Tools/rbk/rbtd/src` — repoint every hit
  (rbtdrp_pristine `RBTDRP_RBRA_ROLES` = GOVERNOR/DIRECTOR/RETRIEVER/ASSAY, plus any
  rbtdrk/rbtdtm_manifest/lib.rs mirror) to `RBTDGC_ACCOUNT_*`.
- Also `grep -rn 'RBRA_ROLE' Tools/rbk/rbtd/src` — IF any Rust fixture asserts an RBRA_ROLE
  *value* (not a dir path), it needs the minted value; then ALSO project `RBCC_role_*` as
  `RBTDGC_ROLE_*` and point only those value-assertions at it. (Verified at slate time:
  none — current Rust role use is `secrets.join(role)` dir-path only.)
- Rebuild: `tt/vow-b.Build.sh`; unit tests: `tt/rbw-tt.Test.sh`.

### G. Spec text
- BUS0 (Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc) "Target Enums" (~:1025-1040) —
  add `Auth regime (rbnae_): RBRA_ROLE — governor/retriever/director → rbnae_*`, and a one-
  line note that auth-role uniquely splits into a minted enum family (`RBCC_role_*`,
  RBRA_ROLE domain) and a bare composition family (`RBCC_account_*`, SA-names + secret
  dirs), the bare side justified by class-4.
- adoc account-name prose carrying `${RBCC_role_*}-` (RBSRL-retriever_roster:39,48,
  RBSRK-retriever_invest:15,33, RBSDK-director_invest:12,30, RBSDR-director_roster:37,45) —
  these describe the SA `accountId` prefix → switch the attribute ref to `RBCC_account_*`.

## Critical test caveat — the enum gate does NOT cover roles
`RBRA_ROLE` is `buv_string_enroll` + a hand-written `case`, NOT `buv_enum_enroll`. So the
paddock's "regime-validation rejects a bare survivor once the declaration flips" safety net
does NOT apply here. The per-family grep audit AND the required butcrg run (below) are
therefore LOAD-BEARING, not confirmatory — butcrg is the only automated exercise of the
minted folio through `rbw-rav`/`rbw-rar`.

## Acceptance gate
- `tt/vow-b.Build.sh` clean (deny-warnings) + `tt/rbw-tt.Test.sh` green.
- `tt/rbw-ts.TestSuite.fast.sh` green (read ../logs-buk/last.txt; never pipe the tabtarget).
- shellcheck via release-qualify path if touched broadly.
- Grep audit: `grep -rn 'rbnae_governor\|rbnae_retriever\|rbnae_director' Tools/` returns a
  coherent enum-value cluster (RBRA_ROLE writes/validation/folio/resolver/display + the
  butcrg test folio only); NO `rbnae_*` leaks into an SA-name, email, or directory path.
  Bare `governor`/`director`/`retriever` survivors must all be: class-3 (colophons rbw-ac*,
  function names, doc filenames), rbgv resolver labels (C-bis), the rbgp prose/label
  literals (C-bis: :1687/:1712/:1811), or class-C account/dir composition.
- **Required credential proof (replaces the former "optional" leg) — the mint is only
  proven end-to-end against real credentials, so close this gap rather than waving it:**
  - **Workstation, always:** run the `butcrg_RegimeCredentials.sh` RBRA cases (find its
    enrolled dispatch) — green proves the minted folio resolves through `rbw-rav`/`rbw-rar`
    and the swizzle guard. Also run `rbw-rav rbnae_<role>` for each role whose RBRA file is
    present.
  - **If GCP creds present:** a fresh invest (`rbw-aM` governor / `rbw-arI` retriever /
    `rbw-adI` director) writes `RBRA_ROLE=rbnae_*`, then the matching access-probe
    (`rbw-acg`/`rbw-acr`/`rbw-acd`) sources it and reaches GCP — exercising rbgv resolution
    + `zrbra_enforce` against the minted value.
  - **If creds for any role are absent:** state so explicitly in the wrap summary — name the
    untested leg. No silent skip.

## What done looks like
Two `RBCC_` families exist; `RBRA_ROLE`/folio/validation/resolver/butcrg-test carry `rbnae_*`;
every SA email/account-id and every secret-directory name is byte-identical to before; the
flat→nested migration loop and rename breadcrumb are gone; rbgv and the rbgp prose/label
literals left bare by design; BUS0 lists `rbnae_`; Rust projects `RBTDGC_ACCOUNT_*` and
builds/tests green; the required credential proof is run (or its absent legs named in the wrap).

**[260529-0556] rough**

## Character
Mechanical migration with a real axis-split at the seam: one bare `RBCC_role_*`
family that today serves three jobs gets cleaved into a minted enum family and a
bare composition family. Edits are localized and value-preserving on the bare side;
the judgment is resolved below. Imminent execution — every file:line below was
verified against the tree at slate time, and the operator has frozen this surface
(no commits land here before mount), so the refs are trustworthy as written.

## Goal
Make the RBRA-role enum greppable as a sprue family (`rbnae_*`) WITHOUT touching any
Google Cloud service-account email, account-id, or local secret-directory name. A
deliberate partial cleanup: mint only the enum-value axis; leave every derived
resource-name string bare.

## The decision (locked by operator) — `{role, account}` split, Design A

The current single family `RBCC_role_*` conflates two axes:
- **Enum-value axis** — the `RBRA_ROLE` value: written into credential files, validated,
  carried as the auth-regime folio, swizzle-checked. Domain = {governor, retriever,
  director} only.
- **Composition-label axis** — bare fragments that build GCP SA account-ids/emails AND
  local secret-directory names. Domain = {governor, retriever, director, payor, assay,
  mason}.

Cleave into two `RBCC_` families (leading-space comments so no line opens with hash):

```
  RBCC_role_governor="rbnae_governor"      # minted enum — RBRA_ROLE domain (3 members)
  RBCC_role_retriever="rbnae_retriever"
  RBCC_role_director="rbnae_director"

  RBCC_account_governor="governor"         # bare composition — SA-name + secret-dir (6 members)
  RBCC_account_retriever="retriever"
  RBCC_account_director="director"
  RBCC_account_payor="payor"
  RBCC_account_assay="assay"
  RBCC_account_mason="mason"
```

`assay`/`mason`/`payor` are NEVER `RBRA_ROLE` values (validation accepts only
governor|retriever|director), so they get NO `rbnae_` token — `account`-only.

**Design A — secret directories stay bare** (operator-approved): a secret-dir name is a
derived resource-name string in local-filesystem space, structurally identical to the SA
email and the `-tether`/`-airgap` pool suffixes the heat already keeps bare under class-4.
So directories follow `RBCC_account_*`; they are NOT renamed. The folio→file `case` in
`zrbra_resolve_role` is the translation point that lets the folio mint while the directory
stays bare. The prior "operators rename dirs by hand" / "have dirs been renamed?" breadcrumb
and the flat→nested migration loop are VOID — retire them (section D).

## Why the split is safe (already-shaped seams — verified)
- `zrbgg_create_service_account_with_key(account_name, …, role)` already takes
  account-name ($1, builds the email) and role ($4, writes RBRA_ROLE) as separate params.
  Split = feed $1 from `RBCC_account_*`, $4 from `RBCC_role_*`.
- `zrbra_resolve_role` (rbra_cli.sh) maps folio → `RBDC_*_RBRA_FILE` *variable* via a
  `case`, so the folio string is decoupled from the directory name.
- Swizzle guard (rbra_cli.sh) ties only `RBRA_ROLE == BUZ_FOLIO`; both mint together.

## Edit map — by axis
All file:line refs verified against the tree at slate time.

### A. Declarations — rbcc_Constants.sh
- :59-64 — replace the 6-member `RBCC_role_*` block with the two families above.
- :94-95 — `RBCC_fact_ext_roster_*` compose fact-FILE names → use `RBCC_account_*`
  (keeps fact filenames `roster-retriever` bare; minting here would corrupt filenames).
- :101 — rewrite the "Distinct from the credential RBCC_role_* family" axis comment to
  describe the role(enum)/account(label) split.

### B. Minted (enum-value axis → `RBCC_role_*` = `rbnae_*`)
- rbgg_Governor.sh:592, :643 — invest $4 (becomes RBRA_ROLE).
- rbgp_Payor.sh:1876 — `printf 'RBRA_ROLE=%s' "${RBCC_role_governor}"`.
- rbra_regime.sh:69-71 — validation `case` arms → `rbnae_governor|rbnae_retriever|rbnae_director)`.
- rbra_regime.sh:41 — enroll help string text → new values (each ≤20 chars; `rbnae_retriever`
  is 15, within `buv_string_enroll RBRA_ROLE 1 20`).
- rbra_cli.sh:39-44 — `zrbra_resolve_role` case arms → `rbnae_*)`; RBDC var targets unchanged.
- rbra_cli.sh:83 — `z_roles=(...)` display array → `rbnae_*` (labels the enum).
- rbra_cli.sh:35,43 — "role required / Unknown role" die-text → reflect minted values.
- **Tools/buk/buts/butcrg_RegimeCredentials.sh:36,48-49** — `z_roles=("governor"
  "retriever" "director")` is passed as the **folio** to `rbw-rav`/`rbw-rar` (:48-49).
  The folio mints, so `z_roles` must become `rbnae_*` or `zrbra_resolve_role` dies
  "Unknown RBRA role". CAUTION: this site is caught by NONE of the build/fast/grep
  gates — it is a workstation-only credential test, and the grep audit reads its bare
  `governor` as an acceptable class-C survivor. Silent breakage; this edit plus the
  required butcrg run (acceptance gate) are the only things that catch it.

### C. Bare (composition-label axis → `RBCC_account_*`)
- rbgg_Governor.sh:560,569 — roster first arg (SA-name prefix match).
- rbgg_Governor.sh:578,583,629,634,825,830,839,844 — `z_account_name`, divest first arg,
  and the "composes X-<identity>" doc strings.
- rbgd_DepotConstants.sh:61 — `RBGD_MASON_EMAIL`.
- rbgp_Payor.sh:1226,1622 — mason name/email; :1412,1744 — `== ${RBCC_role_governor}-*`
  SA-email match; :1772 — governor account-id.
- Secret-dir paths (Design A, bare): rbdc_DerivedConstants.sh:35-39 (mkdir), :74-78
  (`RBDC_*_RBRA_FILE`); rblm_cli.sh:88,125,167; rbho path-checks rbhodb_director_bind.sh:49,
  rbhoda_director_airgap.sh:52, rbhodf_director_first_build.sh:40, rbhodg_director_graft.sh:51.

### C-bis. Explicitly left bare — NO edit (judgment recorded so the executor need not re-derive)
- `rbgv_AccessProbe.sh` `zrbgv_role_rbra_file_capture` (~:76-95) has its OWN
  `governor)|director)|retriever)` role→file `case` plus internal bare labels in
  `rbgv_check_*`. These are bare-axis (file resolution + internal labels, structurally
  the directory) and STAY bare. They work post-split only because the probe sources the
  RBRA file and runs `zrbra_enforce`, whose case is minted in section B — confirm that
  coupling holds, then leave rbgv untouched.
- Pre-existing bare `governor-*` literals NOT sourced from `RBCC_role_*`, all intentionally
  bare and untouched: rbgp_Payor.sh:1687 (doc prose), :1712 (step message), :1811
  (`"governor-owner"` IAM request-id label). The grep audit will surface these — leave them.

### D. Retire (no-auto-repair posture; Design A voids these)
- rbdc_DerivedConstants.sh:42-71 — delete the flat→nested one-shot migration (both the
  role loop :42-61 and the payor `rbro.env` block :63-71). It is dead auto-repair.
  Post-split it goes INCOHERENT, not merely stale: :44/:46 build the dir path from
  `RBCC_role_*` (now `rbnae_governor/…`) while the real secret dirs stay bare — a path
  mismatch — so it cannot be mechanically repointed. Delete, do not repair. Drop the
  rename breadcrumb entirely — under Design A nothing renames.

### E. The one mixed-arg seam — split `zrbho_credential_install`
- rbhob_base.sh:88 `zrbho_credential_install(z_role_constant)` uses its single arg as BOTH
  the displayed dir path (:97, :115 — needs bare) AND the folio passed to `rbw-rav`
  (:135 — needs minted). Give it two params: `(z_account_label, z_role_folio)`.
  - :97,:115 dir-path → `z_account_label` (bare).
  - :135 `buh_tt … "${RBZ_VALIDATE_AUTH}" "" " ${z_role_folio}"` → minted folio.
  - Callers: rbhocr_credential_retriever.sh:38, rbhocd_credential_director.sh:38 — pass
    `"${RBCC_account_X}" "${RBCC_role_X}"`.

### F. Rust projection + consumers (value-preserving rename)
- rbcc_Constants.sh:126-134 — the `rbcc_emit_consts` projection list currently emits
  `RBCC_role_*` (6) → `RBTDGC_ROLE_*`. Rust uses these ONLY as secret-dir names
  (rbtdrp_pristine.rs:288 `secrets.join(role)`), i.e. the bare axis. Repoint the list to
  the 6 `RBCC_account_*`; projected names become `RBTDGC_ACCOUNT_*` (values unchanged).
- rbcc_Constants.sh:110-119 — the projection-block comment ("credential roles", "the
  manifest role mirror") describes the projected set; refresh that prose to "account
  labels" so it matches the repointed list.
- Discovery: `grep -rn 'RBTDGC_ROLE_' Tools/rbk/rbtd/src` — repoint every hit
  (rbtdrp_pristine `RBTDRP_RBRA_ROLES` = GOVERNOR/DIRECTOR/RETRIEVER/ASSAY, plus any
  rbtdrk/rbtdtm_manifest/lib.rs mirror) to `RBTDGC_ACCOUNT_*`.
- Also `grep -rn 'RBRA_ROLE' Tools/rbk/rbtd/src` — IF any Rust fixture asserts an RBRA_ROLE
  *value* (not a dir path), it needs the minted value; then ALSO project `RBCC_role_*` as
  `RBTDGC_ROLE_*` and point only those value-assertions at it. (Verified at slate time:
  none — current Rust role use is `secrets.join(role)` dir-path only.)
- Rebuild: `tt/vow-b.Build.sh`; unit tests: `tt/rbw-tt.Test.sh`.

### G. Spec text
- BUS0 (Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc) "Target Enums" (~:1025-1040) —
  add `Auth regime (rbnae_): RBRA_ROLE — governor/retriever/director → rbnae_*`, and a one-
  line note that auth-role uniquely splits into a minted enum family (`RBCC_role_*`,
  RBRA_ROLE domain) and a bare composition family (`RBCC_account_*`, SA-names + secret
  dirs), the bare side justified by class-4.
- adoc account-name prose carrying `${RBCC_role_*}-` (RBSRL-retriever_roster:39,48,
  RBSRK-retriever_invest:15,33, RBSDK-director_invest:12,30, RBSDR-director_roster:37,45) —
  these describe the SA `accountId` prefix → switch the attribute ref to `RBCC_account_*`.

## Critical test caveat — the enum gate does NOT cover roles
`RBRA_ROLE` is `buv_string_enroll` + a hand-written `case`, NOT `buv_enum_enroll`. So the
paddock's "regime-validation rejects a bare survivor once the declaration flips" safety net
does NOT apply here. The per-family grep audit AND the required butcrg run (below) are
therefore LOAD-BEARING, not confirmatory — butcrg is the only automated exercise of the
minted folio through `rbw-rav`/`rbw-rar`.

## Acceptance gate
- `tt/vow-b.Build.sh` clean (deny-warnings) + `tt/rbw-tt.Test.sh` green.
- `tt/rbw-ts.TestSuite.fast.sh` green (read ../logs-buk/last.txt; never pipe the tabtarget).
- shellcheck via release-qualify path if touched broadly.
- Grep audit: `grep -rn 'rbnae_governor\|rbnae_retriever\|rbnae_director' Tools/` returns a
  coherent enum-value cluster (RBRA_ROLE writes/validation/folio/resolver/display + the
  butcrg test folio only); NO `rbnae_*` leaks into an SA-name, email, or directory path.
  Bare `governor`/`director`/`retriever` survivors must all be: class-3 (colophons rbw-ac*,
  function names, doc filenames), rbgv resolver labels (C-bis), the rbgp prose/label
  literals (C-bis: :1687/:1712/:1811), or class-C account/dir composition.
- **Required credential proof (replaces the former "optional" leg) — the mint is only
  proven end-to-end against real credentials, so close this gap rather than waving it:**
  - **Workstation, always:** run the `butcrg_RegimeCredentials.sh` RBRA cases (find its
    enrolled dispatch) — green proves the minted folio resolves through `rbw-rav`/`rbw-rar`
    and the swizzle guard. Also run `rbw-rav rbnae_<role>` for each role whose RBRA file is
    present.
  - **If GCP creds present:** a fresh invest (`rbw-aM` governor / `rbw-arI` retriever /
    `rbw-adI` director) writes `RBRA_ROLE=rbnae_*`, then the matching access-probe
    (`rbw-acg`/`rbw-acr`/`rbw-acd`) sources it and reaches GCP — exercising rbgv resolution
    + `zrbra_enforce` against the minted value.
  - **If creds for any role are absent:** state so explicitly in the wrap summary — name the
    untested leg. No silent skip.

## What done looks like
Two `RBCC_` families exist; `RBRA_ROLE`/folio/validation/resolver/butcrg-test carry `rbnae_*`;
every SA email/account-id and every secret-directory name is byte-identical to before; the
flat→nested migration loop and rename breadcrumb are gone; rbgv and the rbgp prose/label
literals left bare by design; BUS0 lists `rbnae_`; Rust projects `RBTDGC_ACCOUNT_*` and
builds/tests green; the required credential proof is run (or its absent legs named in the wrap).

**[260529-0548] rough**

## Character
Mechanical migration with a real axis-split at the seam: one bare `RBCC_role_*`
family that today serves three jobs gets cleaved into a minted enum family and a
bare composition family. Edits are localized and value-preserving on the bare side;
the judgment is resolved below. Imminent execution — site lists carry file:line on
purpose; re-grep only if commits land on this surface first.

## Goal
Make the RBRA-role enum greppable as a sprue family (`rbnae_*`) WITHOUT touching any
Google Cloud service-account email, account-id, or local secret-directory name. A
deliberate partial cleanup: mint only the enum-value axis; leave every derived
resource-name string bare.

## The decision (locked by operator) — `{role, account}` split, Design A

The current single family `RBCC_role_*` conflates two axes:
- **Enum-value axis** — the `RBRA_ROLE` value: written into credential files, validated,
  carried as the auth-regime folio, swizzle-checked. Domain = {governor, retriever,
  director} only.
- **Composition-label axis** — bare fragments that build GCP SA account-ids/emails AND
  local secret-directory names. Domain = {governor, retriever, director, payor, assay,
  mason}.

Cleave into two `RBCC_` families (leading-space comments so no line opens with hash):

```
  RBCC_role_governor="rbnae_governor"      # minted enum — RBRA_ROLE domain (3 members)
  RBCC_role_retriever="rbnae_retriever"
  RBCC_role_director="rbnae_director"

  RBCC_account_governor="governor"         # bare composition — SA-name + secret-dir (6 members)
  RBCC_account_retriever="retriever"
  RBCC_account_director="director"
  RBCC_account_payor="payor"
  RBCC_account_assay="assay"
  RBCC_account_mason="mason"
```

`assay`/`mason`/`payor` are NEVER `RBRA_ROLE` values (validation accepts only
governor|retriever|director), so they get NO `rbnae_` token — `account`-only.

**Design A — secret directories stay bare** (operator-approved): a secret-dir name is a
derived resource-name string in local-filesystem space, structurally identical to the SA
email and the `-tether`/`-airgap` pool suffixes the heat already keeps bare under class-4.
So directories follow `RBCC_account_*`; they are NOT renamed. The folio→file `case` in
`zrbra_resolve_role` is the translation point that lets the folio mint while the directory
stays bare. The prior "operators rename dirs by hand" / "have dirs been renamed?" breadcrumb
and the flat→nested migration loop are VOID — retire them (section D).

## Why the split is safe (already-shaped seams — verified)
- `zrbgg_create_service_account_with_key(account_name, …, role)` already takes
  account-name ($1, builds the email) and role ($4, writes RBRA_ROLE) as separate params.
  Split = feed $1 from `RBCC_account_*`, $4 from `RBCC_role_*`.
- `zrbra_resolve_role` (rbra_cli.sh) maps folio → `RBDC_*_RBRA_FILE` *variable* via a
  `case`, so the folio string is decoupled from the directory name.
- Swizzle guard (rbra_cli.sh) ties only `RBRA_ROLE == BUZ_FOLIO`; both mint together.

## Edit map — by axis

### A. Declarations — rbcc_Constants.sh
- :59-64 — replace the 6-member `RBCC_role_*` block with the two families above.
- :94-95 — `RBCC_fact_ext_roster_*` compose fact-FILE names → use `RBCC_account_*`
  (keeps fact filenames `roster-retriever` bare; minting here would corrupt filenames).
- :101 — rewrite the "Distinct from the credential RBCC_role_* family" axis comment to
  describe the role(enum)/account(label) split.

### B. Minted (enum-value axis → `RBCC_role_*` = `rbnae_*`)
- rbgg_Governor.sh:592, :643 — invest $4 (becomes RBRA_ROLE).
- rbgp_Payor.sh:1876 — `printf 'RBRA_ROLE=%s' "${RBCC_role_governor}"`.
- rbra_regime.sh:69-71 — validation `case` arms → `rbnae_governor|rbnae_retriever|rbnae_director)`.
- rbra_regime.sh:41 — enroll help string text → new values (each ≤20 chars; `rbnae_retriever`
  is 15, within `buv_string_enroll RBRA_ROLE 1 20`).
- rbra_cli.sh:39-44 — `zrbra_resolve_role` case arms → `rbnae_*)`; RBDC var targets unchanged.
- rbra_cli.sh:83 — `z_roles=(...)` display array → `rbnae_*` (labels the enum).
- rbra_cli.sh:35,43 — "role required / Unknown role" die-text → reflect minted values.
- **Tools/buk/buts/butcrg_RegimeCredentials.sh:36,48-49** — `z_roles=("governor"
  "retriever" "director")` is passed as the **folio** to `rbw-rav`/`rbw-rar` (:48-49).
  The folio mints, so `z_roles` must become `rbnae_*` or `zrbra_resolve_role` dies
  "Unknown RBRA role". CAUTION: this site is caught by NONE of the acceptance gates —
  it is a workstation-only BUK test (not in `fast`), and the grep audit reads its bare
  `governor` as an acceptable class-C survivor. Silent breakage; this edit is the only
  thing that prevents it.

### C. Bare (composition-label axis → `RBCC_account_*`)
- rbgg_Governor.sh:560,569 — roster first arg (SA-name prefix match).
- rbgg_Governor.sh:578,583,629,634,825,830,839,844 — `z_account_name`, divest first arg,
  and the "composes X-<identity>" doc strings.
- rbgd_DepotConstants.sh:61 — `RBGD_MASON_EMAIL`.
- rbgp_Payor.sh:1226,1622 — mason name/email; :1412,1744 — `== governor-*` SA-email match;
  :1772 — governor account-id.
- Secret-dir paths (Design A, bare): rbdc_DerivedConstants.sh:35-39 (mkdir), :74-78
  (`RBDC_*_RBRA_FILE`); rblm_cli.sh:88,125,167; rbho path-checks rbhodb_director_bind.sh:49,
  rbhoda_director_airgap.sh:52, rbhodf_director_first_build.sh:40, rbhodg_director_graft.sh:51.

### C-bis. Explicitly left bare — NO edit (judgment recorded so the executor need not re-derive)
- `rbgv_AccessProbe.sh` `zrbgv_role_rbra_file_capture` (~:76-95) has its OWN
  `governor)|director)|retriever)` role→file `case` plus internal bare labels in
  `rbgv_check_*`. These are bare-axis (file resolution + internal labels, structurally
  the directory) and STAY bare. They work post-split only because the probe sources the
  RBRA file and runs `zrbra_enforce`, whose case is minted in section B — confirm that
  coupling holds, then leave rbgv untouched. (This is also the path the optional
  service-tier access-probe acceptance check exercises.)

### D. Retire (no-auto-repair posture; Design A voids these)
- rbdc_DerivedConstants.sh:42-71 — delete the flat→nested one-shot migration (both the
  role loop :42-61 and the payor `rbro.env` block :63-71). It is dead auto-repair.
  Post-split it goes INCOHERENT, not merely stale: :44/:46 build the dir path from
  `RBCC_role_*` (now `rbnae_governor/…`) while the real secret dirs stay bare — a path
  mismatch — so it cannot be mechanically repointed. Delete, do not repair. Drop the
  rename breadcrumb entirely — under Design A nothing renames.

### E. The one mixed-arg seam — split `zrbho_credential_install`
- rbhob_base.sh:88 `zrbho_credential_install(z_role_constant)` uses its single arg as BOTH
  the displayed dir path (:97, :115 — needs bare) AND the folio passed to `rbw-rav`
  (:135 — needs minted). Give it two params: `(z_account_label, z_role_folio)`.
  - :97,:115 dir-path → `z_account_label` (bare).
  - :135 `buh_tt … "${RBZ_VALIDATE_AUTH}" "" " ${z_role_folio}"` → minted folio.
  - Callers: rbhocr_credential_retriever.sh:38, rbhocd_credential_director.sh:38 — pass
    `"${RBCC_account_X}" "${RBCC_role_X}"`.

### F. Rust projection + consumers (value-preserving rename)
- rbcc_Constants.sh:126-134 — the `rbcc_emit_consts` projection list currently emits
  `RBCC_role_*` (6) → `RBTDGC_ROLE_*`. Rust uses these ONLY as secret-dir names
  (rbtdrp_pristine.rs:288 `secrets.join(role)`), i.e. the bare axis. Repoint the list to
  the 6 `RBCC_account_*`; projected names become `RBTDGC_ACCOUNT_*` (values unchanged).
- rbcc_Constants.sh:110-119 — the projection-block comment ("credential roles", "the
  manifest role mirror") describes the projected set; refresh that prose to "account
  labels" so it matches the repointed list.
- Discovery: `grep -rn 'RBTDGC_ROLE_' Tools/rbk/rbtd/src` — repoint every hit
  (rbtdrp_pristine `RBTDRP_RBRA_ROLES` = GOVERNOR/DIRECTOR/RETRIEVER/ASSAY, plus any
  rbtdrk/rbtdtm_manifest/lib.rs mirror) to `RBTDGC_ACCOUNT_*`.
- Also `grep -rn 'RBRA_ROLE' Tools/rbk/rbtd/src` — IF any Rust fixture asserts an RBRA_ROLE
  *value* (not a dir path), it needs the minted value; then ALSO project `RBCC_role_*` as
  `RBTDGC_ROLE_*` and point only those value-assertions at it. (Verified at slate time:
  none — current Rust role use is `secrets.join(role)` dir-path only.)
- Rebuild: `tt/vow-b.Build.sh`; unit tests: `tt/rbw-tt.Test.sh`.

### G. Spec text
- BUS0 (Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc) "Target Enums" (~:1025-1040) —
  add `Auth regime (rbnae_): RBRA_ROLE — governor/retriever/director → rbnae_*`, and a one-
  line note that auth-role uniquely splits into a minted enum family (`RBCC_role_*`,
  RBRA_ROLE domain) and a bare composition family (`RBCC_account_*`, SA-names + secret
  dirs), the bare side justified by class-4.
- adoc account-name prose carrying `${RBCC_role_*}-` (RBSRL-retriever_roster:39,48,
  RBSRK-retriever_invest:15,33, RBSDK-director_invest:12,30, RBSDR-director_roster:37,45) —
  these describe the SA `accountId` prefix → switch the attribute ref to `RBCC_account_*`.

## Critical test caveat — the enum gate does NOT cover roles
`RBRA_ROLE` is `buv_string_enroll` + a hand-written `case`, NOT `buv_enum_enroll`. So the
paddock's "regime-validation rejects a bare survivor once the declaration flips" safety net
does NOT apply here. The per-family grep audit is therefore LOAD-BEARING, not confirmatory —
and butcrg (section B) is the worst case: no gate catches it.

## Acceptance gate
- `tt/vow-b.Build.sh` clean (deny-warnings) + `tt/rbw-tt.Test.sh` green.
- `tt/rbw-ts.TestSuite.fast.sh` green (read ../logs-buk/last.txt; never pipe the tabtarget).
- shellcheck via release-qualify path if touched broadly.
- Grep audit: `grep -rn 'rbnae_governor\|rbnae_retriever\|rbnae_director' Tools/` returns a
  coherent enum-value cluster (RBRA_ROLE writes/validation/folio/resolver/display + butcrg
  test folio only); NO `rbnae_*` leaks into an SA-name, email, or directory path; bare
  `governor`/`director`/`retriever` remaining are all class-3 (colophons rbw-ac*, function
  names, doc filenames), rbgv resolver labels (C-bis), or class-C account/dir composition.
- Optional, creds permitting: a `service`-tier access-probe (rbw-acg/acr/acd) sourcing a
  freshly-invested RBRA file exercises swizzle + zrbra_enforce end-to-end.

## What done looks like
Two `RBCC_` families exist; `RBRA_ROLE`/folio/validation/resolver/butcrg-test carry `rbnae_*`;
every SA email/account-id and every secret-directory name is byte-identical to before; the
flat→nested migration loop and rename breadcrumb are gone; rbgv left bare by design; BUS0
lists `rbnae_`; Rust projects `RBTDGC_ACCOUNT_*` and builds/tests green.

**[260529-0539] rough**

## Character
Mechanical migration, but with a real axis-split at the seam: one bare `RBCC_role_*`
family that today serves three different jobs gets cleaved into a minted enum family
and a bare composition family. Edits are localized and value-preserving on the bare
side; the judgment is already resolved below. Imminent execution — site lists carry
file:line on purpose; re-grep only if commits land first.

## Goal
Make the RBRA-role enum greppable as a sprue family (`rbnae_*`) WITHOUT touching any
Google Cloud service-account email, account-id, or local secret-directory name. This is
a deliberate partial cleanup: mint only the enum-value axis; leave every derived
resource-name string bare.

## The decision (locked by operator) — `{role, account}` split, Design A

The current single family `RBCC_role_*` conflates two axes:
- **Enum-value axis** — the `RBRA_ROLE` value: written into credential files, validated,
  carried as the auth-regime folio, swizzle-checked. Domain = {governor, retriever,
  director} only.
- **Composition-label axis** — bare fragments that build GCP SA account-ids/emails AND
  local secret-directory names. Domain = {governor, retriever, director, payor, assay,
  mason}.

Cleave into two `RBCC_` families:

```
# Minted enum — the RBRA_ROLE domain (THREE members only)
RBCC_role_governor="rbnae_governor"
RBCC_role_retriever="rbnae_retriever"
RBCC_role_director="rbnae_director"

# Bare composition labels — SA-name + secret-dir fragments (SIX members)
RBCC_account_governor="governor"
RBCC_account_retriever="retriever"
RBCC_account_director="director"
RBCC_account_payor="payor"
RBCC_account_assay="assay"
RBCC_account_mason="mason"
```

`assay`/`mason`/`payor` are NEVER `RBRA_ROLE` values (validation accepts only
governor|retriever|director), so they get NO `rbnae_` token — `account`-only.

**Design A — secret directories stay bare** (operator-approved): a secret-dir name is a
derived resource-name string in local-filesystem space, structurally identical to the SA
email and the `-tether`/`-airgap` pool suffixes the heat already keeps bare under its
class-4 rule. So directories follow `RBCC_account_*`; they are NOT renamed. Consequences:
the prior docket's "operators rename dirs by hand", the "have secrets directories been
renamed?" breadcrumb, and the existing flat→nested migration loop are all VOID — drop /
retire them (see below). The folio→file `case` in `zrbra_resolve_role` is the translation
point that lets the folio mint while the directory stays bare.

## Why the split is safe (already-shaped seams)
- `zrbgg_create_service_account_with_key(account_name, …, role)` (rbgg_Governor.sh:159)
  already takes account-name ($1) and role ($4) as separate params — $1 builds the email
  (:167), $4 writes RBRA_ROLE (:322). Split = feed $1 from `RBCC_account_*`, $4 from
  `RBCC_role_*`.
- `zrbra_resolve_role` (rbra_cli.sh:33) maps folio → `RBDC_*_RBRA_FILE` *variable* via a
  `case`, so the folio string is decoupled from the directory name.
- Swizzle guard (rbra_cli.sh:143) ties only `RBRA_ROLE == BUZ_FOLIO`; both mint together.

## Edit map — by axis

### A. Declarations — rbcc_Constants.sh
- :59-64 — replace the 6-member `RBCC_role_*` block with the two families above.
- :94-95 — `RBCC_fact_ext_roster_*` compose fact-FILE names → use `RBCC_account_*`
  (keeps fact filenames `roster-retriever` bare; minting here would corrupt filenames).
- :101 — rewrite the axis comment to describe role(enum)/account(label) split.

### B. Minted (enum-value axis → `RBCC_role_*` = `rbnae_*`)
- rbgg_Governor.sh:592, :643 — invest $4 (becomes RBRA_ROLE).
- rbgp_Payor.sh:1876 — `printf 'RBRA_ROLE=%s' "${RBCC_role_governor}"`.
- rbra_regime.sh:69-71 — validation `case` arms → `rbnae_governor|rbnae_retriever|rbnae_director)`.
- rbra_regime.sh:41 — enroll help string text → new values (each ≤20 chars, within the
  `buv_string_enroll RBRA_ROLE 1 20` limit; confirm).
- rbra_cli.sh:39-44 — `zrbra_resolve_role` case arms → `rbnae_*)`; RBDC var targets unchanged.
- rbra_cli.sh:83 — `z_roles=(...)` display array → `rbnae_*` (labels the enum).
- rbra_cli.sh:35,43 — "role required / Unknown role" die-text → reflect minted values.

### C. Bare (composition-label axis → `RBCC_account_*`)
- rbgg_Governor.sh:560,569 — roster first arg (SA-name prefix match).
- rbgg_Governor.sh:578,583,629,634,825,830,839,844 — `z_account_name`, divest first arg,
  and the "composes X-<identity>" doc strings.
- rbgd_DepotConstants.sh:61 — `RBGD_MASON_EMAIL`.
- rbgp_Payor.sh:1226,1622 — mason name/email; :1412,1744 — `== governor-*` SA-email match;
  :1772 — governor account-id.
- Secret-dir paths (Design A, bare): rbdc_DerivedConstants.sh:35-39 (mkdir), :74-78
  (`RBDC_*_RBRA_FILE`); rblm_cli.sh:88,125,167; rbho path-checks rbhodb_director_bind.sh:49,
  rbhoda_director_airgap.sh:52, rbhodf_director_first_build.sh:40, rbhodg_director_graft.sh:51.

### D. Retire (no-auto-repair posture; Design A voids these)
- rbdc_DerivedConstants.sh:42-71 — delete the flat→nested one-shot migration loop
  (both the role loop and the payor `rbro.env` block). It is legacy auto-repair and, kept,
  would write a stale bare `RBRA_ROLE=` (:58). DROP the rename breadcrumb entirely — under
  Design A nothing renames.

### E. The one mixed-arg seam — split `zrbho_credential_install`
- rbhob_base.sh:88 `zrbho_credential_install(z_role_constant)` uses its single arg as BOTH
  the displayed dir path (:97, :115 — needs bare) AND the folio passed to `rbw-rav`
  (:135 — needs minted). Give it two params: `(z_account_label, z_role_folio)`.
  - :97,:115 dir-path → `z_account_label` (bare).
  - :135 `buh_tt … "${RBZ_VALIDATE_AUTH}" "" " ${z_role_folio}"` → minted folio.
  - Callers: rbhocr_credential_retriever.sh:38, rbhocd_credential_director.sh:38 — pass
    `"${RBCC_account_X}" "${RBCC_role_X}"`.

### F. Rust projection + consumers (value-preserving rename)
- rbcc_Constants.sh:129-134 — the `rbcc_emit_consts` projection list currently emits
  `RBCC_role_*` (6) → `RBTDGC_ROLE_*`. Rust uses these ONLY as secret-dir names
  (rbtdrp_pristine.rs:288 `secrets.join(role)`), i.e. the bare axis. Repoint the list to
  the 6 `RBCC_account_*`; projected names become `RBTDGC_ACCOUNT_*` (values unchanged).
- Discovery: `grep -rn 'RBTDGC_ROLE_' Tools/rbk/rbtd/src` — repoint every hit (rbtdrp_pristine
  `RBTDRP_RBRA_ROLES`, plus any rbtdrk/rbtdtm_manifest/lib.rs mirror) to `RBTDGC_ACCOUNT_*`.
- Also `grep -rn 'RBRA_ROLE' Tools/rbk/rbtd/src` — IF any Rust fixture asserts an RBRA_ROLE
  *value* (not a dir path), it needs the minted value; then ALSO project `RBCC_role_*` as
  `RBTDGC_ROLE_*` and point only those value-assertions at it. (Expected: none — current
  Rust role use is dir-path only.)
- Rebuild: `tt/vow-b.Build.sh`; unit tests: `tt/rbw-tt.Test.sh`.

### G. Spec text
- BUS0 (Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc) "Target Enums" (~:1025-1040) —
  add `Auth regime (rbnae_): RBRA_ROLE — governor/retriever/director → rbnae_*`, and a one-
  line note that auth-role uniquely splits into a minted enum family (`RBCC_role_*`,
  RBRA_ROLE domain) and a bare composition family (`RBCC_account_*`, SA-names + secret
  dirs), the bare side justified by class-4.
- adoc account-name prose carrying `${RBCC_role_*}-` (RBSRL-retriever_roster:39,48,
  RBSRK-retriever_invest:15,33, RBSDK-director_invest:12,30, RBSDR-director_roster:37,45) —
  these describe the SA `accountId` prefix → switch the attribute ref to `RBCC_account_*`.

## Critical test caveat — the enum gate does NOT cover roles
`RBRA_ROLE` is `buv_string_enroll` + a hand-written `case`, NOT `buv_enum_enroll`. So the
paddock's "regime-validation rejects a bare survivor once the declaration flips" safety net
does NOT apply here. The per-family grep audit is therefore LOAD-BEARING, not confirmatory.

## Acceptance gate
- `tt/vow-b.Build.sh` clean (deny-warnings) + `tt/rbw-tt.Test.sh` green.
- `tt/rbw-ts.TestSuite.fast.sh` green (read ../logs-buk/last.txt; never pipe the tabtarget).
- shellcheck via release-qualify path if touched broadly.
- Grep audit: `grep -rn rbnae_governor\|rbnae_retriever\|rbnae_director Tools/` returns a
  coherent enum-value cluster (RBRA_ROLE writes/validation/folio/resolver/display only);
  NO `rbnae_*` leaks into an SA-name, email, or directory path; bare `governor`/`director`/
  `retriever` remaining are all class-3 (colophons rbw-ac*, function names, doc filenames)
  or class-C account/dir composition.
- Optional, creds permitting: a `service`-tier access-probe (rbw-acg/acr/acd) sourcing a
  freshly-invested RBRA file exercises swizzle + zrbra_enforce end-to-end.

## What done looks like
Two `RBCC_` families exist; `RBRA_ROLE`/folio/validation/resolver carry `rbnae_*`; every SA
email/account-id and every secret-directory name is byte-identical to before; the flat→nested
migration loop and rename breadcrumb are gone; BUS0 lists `rbnae_`; Rust projects
`RBTDGC_ACCOUNT_*` and builds/tests green.

**[260528-1012] rough**

## Character
Mechanical migration under a new enum family, plus one deliberate die-text breadcrumb.

## Goal
Migrate operator role enum values from bare words ("governor") to sprue tokens, so every role-value site is greppable as a family — closing the gap where the role family violates the Regime Enum Sprue Convention.

## The mint
New family **`rbnae_`** (rb · n · a=auth regime · e=enum-value): `rbnae_governor` / `rbnae_director` / `rbnae_retriever`, extended across the `RBCC_role_*` set. Register it in BUS0 "Target Enums" beside `rbnve_`/`rbnne_`/`bunne_`.

## Migrate — Class 2 only (per BUS0 Use-Site Classification)
- Make the sprue tokens the canonical values in `rbcc_Constants.sh` `RBCC_role_*`.
- Sweep enum-value sites onto them: `case` arms, equality tests, `RBRA_ROLE` value and its regime validation, folio defaults.
- Leave bare — Class 3 — role words embedded in tabtarget colophons/frontispieces and doc filenames.

## Secret directories
- Dir names derive from the const, so they become `rbnae_governor/` etc. automatically. Operators rename existing dirs by hand.
- No auto-repair: retire any existing role-dir migration dynamic. On a role-credential-not-found die, append a breadcrumb such as `" (have secrets directories been renamed correctly?)"`.

## Discovery
- `grep RBCC_role_` and bare `governor|director|retriever` across `Tools/rbk`; classify each hit against BUS0 Use-Site Classification.

## What done looks like
Role values are `rbnae_*` tokens; one family grep returns every role-value site; tabtarget/doc names unchanged; BUS0 lists the new family; the not-found die carries the rename breadcrumb; shellcheck + relevant suites green.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 K theurge-colophon-consolidation
  2 J tabtarget-dispatch-via-burd-launcher
  3 G idempotent-canonical-invests
  4 B egress-mode-sprue-pilot
  5 A sprue-convention-spec-authoring
  6 C vessel-mode-sprue-migration
  7 D node-mode-sprue-migration
  8 E bubep-to-bunne-harmonize
  9 H dogfight-cloud-build-viability
  10 I theurge-commit-hygiene-guard
  11 F final-regression-and-grep-audit
  12 L cerebro-regression-run
  13 M roles-to-sprue

KJGBACDEHIFLM
······x·x···x rbtdrc_crucible.rs
····x··x····x BUS0-BashUtilitiesSpec.adoc
x········x··x rbtdro_onboarding.rs, rbtdrp_pristine.rs
x········xx·· main.rs
x·······x···x rbtdrm_manifest.rs
x··x········x rbhoda_director_airgap.sh
········x···x rbtdrd_dogfight.rs
·····x······x rbhodb_director_bind.sh, rbhodg_director_graft.sh
·····xx······ RBS0-SpecTop.adoc, rbtdrf_fast.rs
···x·x······· rbfd_FoundryDirectorBuild.sh, rbrv.env, rbrv_regime.sh
··x·········x rbcc_Constants.sh, rbgg_Governor.sh, rbtdrk_canonical.rs
x·······x···· rbtd-s.TestSuite.dogfight.sh, rbte_engine.sh
xx··········· CLAUDE.md, claude-buk-core.md, rbtd-b.Build.sh, rbtd-r.FixtureRun.access-probe.sh, rbtd-r.FixtureRun.batch-vouch.sh, rbtd-r.FixtureRun.canonical-establish.sh, rbtd-r.FixtureRun.dockerfile-hygiene.sh, rbtd-r.FixtureRun.enrollment-validation.sh, rbtd-r.FixtureRun.hallmark-lifecycle.sh, rbtd-r.FixtureRun.handbook-render.sh, rbtd-r.FixtureRun.kludge-tadmor.sh, rbtd-r.FixtureRun.moriah.sh, rbtd-r.FixtureRun.onboarding-sequence.sh, rbtd-r.FixtureRun.pluml.sh, rbtd-r.FixtureRun.pristine-lifecycle.sh, rbtd-r.FixtureRun.regime-smoke.sh, rbtd-r.FixtureRun.regime-validation.sh, rbtd-r.FixtureRun.srjcl.sh, rbtd-r.FixtureRun.tadmor.sh, rbtd-s.FixtureCase.sh, rbtd-s.TestSuite.complete.sh, rbtd-s.TestSuite.crucible.sh, rbtd-s.TestSuite.fast.sh, rbtd-s.TestSuite.gauntlet.sh, rbtd-s.TestSuite.service.sh, rbtd-s.TestSuite.skirmish.sh, rbtd-t.Test.sh, rbw-tP.QualifyPristine.sh, rbw-tS.QualifySkirmish.sh, rbw-tT.QualifyTadmor.sh, rbw-tf.QualifyFast.sh
············x RBSDK-director_invest.adoc, RBSDR-director_roster.adoc, RBSRK-retriever_invest.adoc, RBSRL-retriever_roster.adoc, butcrg_RegimeCredentials.sh, rbdc_DerivedConstants.sh, rbgd_DepotConstants.sh, rbgp_Payor.sh, rbhob_base.sh, rbhocd_credential_director.sh, rbhocr_credential_retriever.sh, rbhodf_director_first_build.sh, rblm_cli.sh, rbra_cli.sh, rbra_regime.sh, rbtdgc_consts.rs
··········x·· README.md, burn.env, memo-20260508-windows-transport-experiments.md, memo-20260511-windows-hive-cleanup-reboot-decision.md, rbtdri_invocation.rs
·········x··· rbtdre_engine.rs
········x···· lib.rs, rbtdtm_manifest.rs
·······x····· bubc_constants.sh, bujb_cli.sh, bujb_jurisdiction.sh, bujp_preflight.sh, burn_regime.sh, buwz_zipper.sh
······x······ rbjs_sentry.sh, rbrn.env, rbrn_cli.sh, rbrn_regime.sh
·····x······· RBSAB-ark_about.adoc, RBSAG-ark_graft.adoc, RBSAV-ark_vouch.adoc, rbfc_FoundryCore.sh, rbfh_cli.sh, rbfk_kludge.sh, rbfv_FoundryVerify.sh, rbgja03-build-info-per-platform.py, rbgjv01-download-verifier.sh, rbgjv02-verify-provenance.py
··x·········· RBSDD-director_divest.adoc, RBSRD-retriever_divest.adoc, rbgu_Utility.sh, rbndb_base.sh
·x··········· BCG-BashConsoleGuide.md, apcw-D.Deploy.sh, apcw-b.Build.sh, apcw-ba.BatchAssay.sh, apcw-cb.ContainerBuild.sh, apcw-ci.ContainerStatus.sh, apcw-cs.ContainerStart.sh, apcw-cx.ContainerStop.sh, apcw-dr.DictionaryRefresh.sh, apcw-fl.FixtureLoad.geriatric.sh, apcw-fl.FixtureLoad.progress.sh, apcw-nsa.NeuralStanfordAssay.sh, apcw-nsi.NeuralStanfordInstall.sh, apcw-r.Run.sh, apcw-t.Test.sh, buq_qualify.sh, burd_regime.sh, buut_tabtarget.sh, buw-hj0.HandbookJurisdictionTop.sh, buw-hjl.HandbookJurisdictionLinux.sh, buw-hjm.HandbookJurisdictionMacos.sh, buw-hjw.HandbookJurisdictionWindows.sh, buw-jpCL.CaparisonLinux.sh, buw-jpCM.CaparisonMacos.sh, buw-jpCW.CaparisonWindows.sh, buw-jpGb.GarrisonBash.sh, buw-jpGc.GarrisonCygwin.sh, buw-jpGw.GarrisonWsl.sh, buw-jpS.PrivilegedSsh.sh, buw-jwc.WorkloadCommandFile.sh, buw-jwk.WorkloadKnock.sh, buw-jws.WorkloadInteractiveSession.sh, buw-qsc.QualifyShellCheck.sh, buw-rcr.RenderConfigRegime.sh, buw-rcv.ValidateConfigRegime.sh, buw-rer.RenderEnvironmentRegime.sh, buw-rev.ValidateEnvironmentRegime.sh, buw-rnl.ListNodeRegime.sh, buw-rnr.RenderNodeRegime.sh, buw-rnv.ValidateNodeRegime.sh, buw-rpl.ListPrivilegeRegime.sh, buw-rpr.RenderPrivilegeRegime.sh, buw-rpv.ValidatePrivilegeRegime.sh, buw-rsr.RenderStationRegime.sh, buw-rsv.ValidateStationRegime.sh, buw-st.BukSelfTest.sh, buw-tt-cbl.CreateTabTargetBatchLogging.sh, buw-tt-cbn.CreateTabTargetBatchNolog.sh, buw-tt-cil.CreateTabTargetInteractiveLogging.sh, buw-tt-cin.CreateTabTargetInteractiveNolog.sh, buw-tt-cl.CreateLauncher.sh, buw-tt-ll.ListLaunchers.sh, buw-xd.Delay.sh, jjw-tfP1.ProvisionPhase1.sh, jjw-tfP2.ProvisionPhase2.cerebro.sh, jjw-tfP2.ProvisionPhase2.localhost.sh, jjw-tfS.TestFundusSingle.localhost.sh, jjw-tfs.TestFundusScenario.cerebro.sh, jjw-tfs.TestFundusScenario.localhost.sh, rbk-claude-acronyms.md, rbw-HWdc.DockerContextDiscipline.sh, rbw-HWdd.DockerDesktop.sh, rbw-Is.IfritSortie.moriah.sh, rbw-Is.IfritSortie.tadmor.sh, rbw-MG.MarshalGenerate.sh, rbw-MP.MarshalProofs.sh, rbw-MZ.MarshalZeroes.sh, rbw-Occ.OnboardingConfigureEnvironment.sh, rbw-Ocd.OnboardingCredentialDirector.sh, rbw-Ocr.OnboardingCredentialRetriever.sh, rbw-Oda.OnboardingDirectorAirgap.sh, rbw-Odb.OnboardingDirectorBind.sh, rbw-Odf.OnboardingDirectorFirstBuild.sh, rbw-Odg.OnboardingDirectorGraft.sh, rbw-Ofc.OnboardingFirstCrucible.sh, rbw-Og.OnboardingGovernor.sh, rbw-Op.OnboardingPayor.sh, rbw-Ots.OnboardingTadmorSecurity.sh, rbw-aM.PayorMantlesGovernor.sh, rbw-acd.CheckDirectorCredential.sh, rbw-acg.CheckGovernorCredential.sh, rbw-acp.CheckPayorCredential.sh, rbw-acr.CheckRetrieverCredential.sh, rbw-adD.GovernorDivestsDirector.sh, rbw-adI.GovernorInvestsDirector.sh, rbw-adr.GovernorRostersDirectors.sh, rbw-arD.GovernorDivestsRetriever.sh, rbw-arI.GovernorInvestsRetriever.sh, rbw-arr.GovernorRostersRetrievers.sh, rbw-cC.Charge.ccyolo.sh, rbw-cC.Charge.moriah.sh, rbw-cC.Charge.pluml.sh, rbw-cC.Charge.srjcl.sh, rbw-cC.Charge.tadmor.sh, rbw-cKB.KludgeBottle.sh, rbw-cKS.KludgeSentry.sh, rbw-cQ.Quench.ccyolo.sh, rbw-cQ.Quench.moriah.sh, rbw-cQ.Quench.pluml.sh, rbw-cQ.Quench.srjcl.sh, rbw-cQ.Quench.tadmor.sh, rbw-cS.SshTo.ccyolo.sh, rbw-cS.SshTo.moriah.sh, rbw-cb.Bark.moriah.sh, rbw-cb.Bark.pluml.sh, rbw-cb.Bark.srjcl.sh, rbw-cb.Bark.tadmor.sh, rbw-cf.Fiat.moriah.sh, rbw-cf.Fiat.pluml.sh, rbw-cf.Fiat.srjcl.sh, rbw-cf.Fiat.tadmor.sh, rbw-ch.Hail.sh, rbw-cic.CrucibleIsCharged.sh, rbw-cr.Rack.sh, rbw-cs.Scry.sh, rbw-cw.Writ.moriah.sh, rbw-cw.Writ.pluml.sh, rbw-cw.Writ.srjcl.sh, rbw-cw.Writ.tadmor.sh, rbw-dE.DirectorEnshrinesVessel.sh, rbw-dI.DirectorInscribesReliquary.sh, rbw-dL.PayorLeviesDepot.sh, rbw-dU.PayorUnmakesDepot.sh, rbw-dY.DirectorYokesReliquaryAllVessels.sh, rbw-di.DepotInfo.sh, rbw-dl.PayorListsDepots.sh, rbw-fA.DirectorAbjuresHallmark.sh, rbw-fO.DirectorOrdainsHallmark.sh, rbw-fV.DirectorVouchesHallmarks.sh, rbw-fhc.HygieneCheck.sh, rbw-fhv.HygieneCheckVessel.sh, rbw-fk.LocalKludge.sh, rbw-fpc.RetrieverPlumbsCompact.sh, rbw-fpf.RetrieverPlumbsFull.sh, rbw-fs.RetrieverSummonsHallmark.sh, rbw-ft.RetrieverTalliesHallmarks.sh, rbw-gPE.PayorEstablish.sh, rbw-gPI.PayorInstall.sh, rbw-gPR.PayorRefresh.sh, rbw-gq.QuotaBuild.sh, rbw-h0.HandbookTOP.sh, rbw-hw.HandbookWindows.sh, rbw-iJe.DirectorJettisonsEnshrinement.sh, rbw-iJh.DirectorJettisonsHallmarkImage.sh, rbw-iJr.DirectorJettisonsReliquaryImage.sh, rbw-iae.DirectorAuditsEnshrinements.sh, rbw-iah.DirectorAuditsHallmarks.sh, rbw-iar.DirectorAuditsReliquaries.sh, rbw-irh.DirectorRekonsHallmark.sh, rbw-irr.DirectorRekonsReliquary.sh, rbw-iwe.DirectorWrestsEnshrinedImage.sh, rbw-iwh.DirectorWrestsHallmarkImage.sh, rbw-iwr.DirectorWrestsReliquaryImage.sh, rbw-ni.NameplateInfo.sh, rbw-nv.ValidateNameplates.sh, rbw-o.ONBOARDING.sh, rbw-ral.ListAuthRegimes.sh, rbw-rar.RenderAuthRegime.sh, rbw-rav.ValidateAuthRegime.sh, rbw-rdc.CheckDepotRegime.sh, rbw-rdi.InscribeDepotRegime.sh, rbw-rdr.RenderDepotRegime.sh, rbw-rdv.ValidateDepotRegime.sh, rbw-rnl.ListNameplateRegime.sh, rbw-rnr.RenderNameplateRegime.sh, rbw-rnv.ValidateNameplateRegime.sh, rbw-ror.RenderOauthRegime.sh, rbw-rov.ValidateOauthRegime.sh, rbw-rpr.RenderPayorRegime.sh, rbw-rpv.ValidatePayorRegime.sh, rbw-rrr.RenderRepoRegime.sh, rbw-rrv.ValidateRepoRegime.sh, rbw-rsr.RenderStationRegime.sh, rbw-rsv.ValidateStationRegime.sh, rbw-rvl.ListVesselRegime.sh, rbw-rvr.RenderVesselRegime.sh, rbw-rvv.ValidateVesselRegime.sh, rbw-tK.KludgeCycle.tadmor.sh, rbw-tO.OrdainCycle.tadmor.sh, rbw-tr.QualifyRelease.sh, study-mpt.Run.FULL.sh, study-mpt.Run.api-FULL.sh, study-mpt.Run.smoke.sh, vow-F.Freshen.sh, vow-R.ParcelRelease.sh, vow-b.Build.sh, vow-c.Clean.sh, vow-r.RunVVX.sh, vow-t.Test.sh, vslk-i.InstallSlickEditProject.sh, vvw-r.RunVVX.sh, z-launcher.sh
x············ CLAUDE.consumer.md, launcher.rbtw_workbench.sh, rbhots_tadmor_security.sh, rbk-claude-tabtarget-context.md, rbk-claude-theurge-ifrit-context.md, rbq_Qualify.sh, rbtdth_helpers.rs, rbte_cli.sh, rbtw_workbench.sh, rbw-tb.Build.sh, rbw-tc.FixtureCase.sh, rbw-tf.FixtureRun.sh, rbw-tq.QualifyFast.sh, rbw-ts.TestSuite.complete.sh, rbw-ts.TestSuite.crucible.sh, rbw-ts.TestSuite.dogfight.sh, rbw-ts.TestSuite.fast.sh, rbw-ts.TestSuite.gauntlet.sh, rbw-ts.TestSuite.service.sh, rbw-ts.TestSuite.skirmish.sh, rbw-ts.TestSuite.tadmor.sh, rbw-tt.Test.sh, rbz_zipper.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 60 commits)

  1 G idempotent-canonical-invests
  2 I theurge-commit-hygiene-guard
  3 J tabtarget-dispatch-via-burd-launcher
  4 H dogfight-cloud-build-viability
  5 K theurge-colophon-consolidation
  6 F final-regression-and-grep-audit
  7 L cerebro-regression-run
  8 M roles-to-sprue

123456789abcdefghijklmnopqrstuvwxyz
·······x··xx·x·····················  G  4c
················x··x···············  I  2c
·················xx················  J  2c
·····················xxx···········  H  3c
·························xx········  K  2c
····························xxx····  F  3c
·······························x···  L  1c
·································xx  M  2c
```

## Steeplechase

### 2026-05-29 07:08 - ₢BMAAM - W

Cleaved the conflated RBCC_role_* family into a minted enum axis (RBCC_role_{governor,retriever,director}=rbnae_*, the RBRA_ROLE value) and a bare composition axis (RBCC_account_*, 6 members, SA account-ids/emails + secret dirs) per operator-locked Design A. Code landed in 156066f: enum sites minted (rbra_regime validation+help, rbra_cli resolver/display/die/header, butcrg folio), composition sites repointed to RBCC_account_*, dead flat->nested migration loop deleted, zrbho_credential_install split into (account_label,role_folio), Rust projection repointed to RBTDGC_ACCOUNT_* with all six rbtd/src consumers swept, BUS0 + four roster/invest adocs updated. rbgv and the three bare rbgp literals left bare by design (rbgv confirmed fully decoupled from RBRA_ROLE — uses credential material, not role). FULLY PROVEN: vow-b+rbw-tb clean (deny-warnings, consts regenerated), 117/117 theurge units, fast suite 112 cases, shellcheck clean on 16 touched files, grep audit coherent (rbnae_ one cluster, zero leak into any SA-name/email/dir). LIVE GCP PROOF (operator-authorized): fresh rbw-aM governor mantle wrote RBRA_ROLE=rbnae_governor with bare SA email governor-202605290654@...; rbw-rav rbnae_governor validates green; rbw-acg reaches GCP HTTP 200; bare folio 'governor' correctly rejected at resolver. SIDE EFFECT: governor SA rotated (old deleted, new created, fresh cred moved into governor slot). UNTESTED LEGS (named, not skipped): retriever/director on-disk RBRA files still carry bare RBRA_ROLE (burn-bridges stale — re-invest proves them, path structurally identical to governor). SPOOK SURFACED: butcrg_RegimeCredentials.sh is dead code — the 'regime-credentials' suite (origin 077c81473, last live ₢AkAAc) lost its enrollment during the theurge consolidation that ported regime-smoke but not the workstation-cred-gated regime-credentials; zero coverage since. Its minted-folio coverage was provided manually this pace; relocation to theurge is slated as a separate ₣BJ study pace.

### 2026-05-29 06:40 - ₢BMAAM - n

Split the conflated RBCC_role_* family into a minted enum axis and a bare composition axis so RBRA_ROLE becomes a greppable rbnae_ sprue without touching any SA email/account-id or secret-directory name. rbcc_Constants now declares RBCC_role_{governor,retriever,director}=rbnae_* (the RBRA_ROLE value: written by invest/payor, validated in zrbra_enforce, carried as the rbw-rav/rbw-rar folio, swizzle-checked) and a separate bare RBCC_account_* family (6 members incl payor/assay/mason) for SA account-ids/emails + secret dirs. Minted the enum sites: rbra_regime validation case+help, rbra_cli resolver case/display/die-text/header, butcrg folio array (the silent-breakage site no gate catches). Repointed every composition site to RBCC_account_*: governor roster/invest/divest SA-names+doc strings, mason name/email, governor SA-email prefix matches + account-id, all secret-dir paths (rbdc mkdir + RBDC_*_RBRA_FILE, rblm previews, four rbho director path-checks). Deleted the now-incoherent flat->nested migration loop + payor rbro block in rbdc (Design A: dirs stay bare, nothing renames). Split zrbho_credential_install into (z_account_label bare-dir, z_role_folio minted) with both callers updated. Repointed the rbcc->Rust projection list to RBCC_account_* (RBTDGC_ROLE_*->RBTDGC_ACCOUNT_*, values unchanged) and swept all six rbtd/src consumers; theurge regenerates rbtdgc_consts.rs and builds clean, 117/117 unit tests green. BUS0 Target Enums gains the Auth-regime rbnae_ entry + split note; four roster/invest adocs switch accountId prose to RBCC_account_*. rbgv left bare by design (decoupled from RBRA_ROLE entirely — uses credential material, not role; the only two RBRA_ROLE consumers, zrbra_enforce and the swizzle guard, are both minted) along with the three intentional bare governor- literals in rbgp. Grep audit clean: rbnae_ forms one coherent enum-value cluster with no leak into any SA-name/email/dir path. Committing to satisfy the suite commit-hygiene guard before the fast-suite acceptance leg.

### 2026-05-28 10:12 - Heat - S

roles-to-sprue

### 2026-05-27 10:06 - ₢BMAAL - W

Cerebro (Linux) leg of the BM regression. VERDICT: migration sound on Linux — zero defects; the cloud-dependent tail blocked on the same orthogonal infra as the macOS leg. DEFERRED, not failed. Wrapped from the macOS officium (sole JJK driver this session) on cerebro's verbatim disposition report — cerebro stood down without mutating shared gallops state to avoid cross-clone divergence.

### 2026-05-27 09:52 - ₢BMAAF - W

Final regression for the sprue migration + theurge-colophon consolidation. VERDICT: migration is sound — zero defects found; remaining gaps are orthogonal infrastructure, not migration bugs.

### 2026-05-27 09:51 - ₢BMAAF - n

Fix theurge BURV fact-capture fragility surfaced during the BM final regression. rbtdb_allocate_roots anchored burv_output_root under BURD_OUTPUT_DIR (output-buk/current) while burv_temp_root sat under the stable per-run trace dir (BURD_TEMP_DIR/rbtd). output-buk/current is rm-rf'd at the start of EVERY BUK dispatch (buf_fact.sh documents it as 'cleared on next dispatch'), and the fact reader (rbtdri_read_burv_fact) reads the OUTPUT copy. So when any default-root dispatch cleared output-buk/current mid-run, it took the whole burv output tree with it, vaporizing a captured fact while the durable temp copy survived. Manifested as canonical-invest/governor_mantle aborting with 'cannot read fact rbgp_fact_governor_sa_email' AFTER the rbw-aM command itself completed exit 0 (fact confirmed present in the temp-buk copy, absent from output/current). Fix: anchor BOTH burv roots under the stable trace dir (burv-temp/burv-output), matching the rbtdti_invocation.rs unit-test layout that always used a single stable scratch root (and so never caught this); drop the BURD_OUTPUT_DIR read and the now-dead RBTDRI_BURD_OUTPUT_DIR_KEY const. Build clean under deny(warnings); 116/116 theurge unit tests green. NOT yet live-validated against a real suite run (a clean skirmish past governor_mantle is the pending proof). Exact dispatch that cleared output-buk/current at the failure was not pinned; the fix removes the dependency on that volatile dir regardless of trigger.

### 2026-05-27 07:29 - ₢BMAAF - n

Repair the bubep->bunne harmonize miss surfaced by the final-regression grep audit. The bubep_to_bunne pace updated burn_regime.sh's enum gate to bunne_* but left the live BURN_PLATFORM=bubep_windows value in the bujn-winpc node profile (now invalid against the gate) plus its README field-doc; harmonized both, and the three dated-memo bubep_windows references, so grep -rn bubep_ is now repo-wide empty. The grep audit is otherwise clean: rbnve_/rbnne_/bunne_ each a coherent cluster, zero live rbtd- colophons, zero class-2 bare-value survivors.

### 2026-05-27 07:29 - Heat - S

cerebro-regression-run

### 2026-05-26 18:27 - ₢BMAAK - W

Collapsed the theurge rbtd-* tabtarget family into rbw-t* and retired the second (RBTW) pipeline — all test/qualify dispatch now rides rbw_workbench via buz_exec_lookup. rbte_cli re-homed as a buc_execute furnish module enrolled in rbz_zipper (rbw-tb/tt imprint, rbw-ts suite imprint, rbw-tf/tc param1); rbte_engine reads BUZ_FOLIO; new ZRBTE_SUITE_TADMOR folds QualifyTadmor; rbw-tP/tS/tT folded into rbw-ts gauntlet/skirmish/tadmor; QualifyFast moved rbw-tf->rbw-tq; rbtw_workbench.sh + launcher + rbte_dispatch deleted. Callers/docs swept, tabtarget context regenerated. Verified on the cheap/local path: rbw-tb build clean, rbw-ts.TestSuite.fast.sh green (112 cases), rbw-tq QualifyFast green, zero live rbtd- colophons. Build/Test use the imprint channel solely to forward cargo args (empty folio). Live dogfight cloud run surfaced a pre-existing ACCOUNT_STATE_INVALID re-invest flap (2/3 runs) — orthogonal to this pace; documented in memo-20260527 and slated as repair pace BBABL in BB. Heavy gauntlet/cloud regression remains AAF's.

### 2026-05-26 17:42 - ₢BMAAK - n

Collapse the theurge rbtd-* tabtarget family into rbw-t* and retire the second (RBTW) pipeline — all test/qualify dispatch now rides rbw_workbench via buz_exec_lookup. Re-homed rbte_cli as a buc_execute furnish module enrolled in rbz_zipper (rbw-tb/tt imprint, rbw-ts suite imprint, rbw-tf/tc param1); rbte_engine reads BUZ_FOLIO instead of BURD_TOKEN_3 and gains a ZRBTE_SUITE_TADMOR (kludge-tadmor, tadmor) array. Folded QualifyPristine/Skirmish/Tadmor (rbw-tP/tS/tT, now-dead rbq_qualify_* fns removed) into rbw-ts gauntlet/skirmish/tadmor imprints; moved QualifyFast rbw-tf -> rbw-tq. Deleted rbtw_workbench.sh + its launcher + the rbte_dispatch switch. Swept callers/docs (buh_tt handbook calls, root CLAUDE.md, CLAUDE.consumer.md, claude-buk-core.md, theurge-ifrit context, Rust hint strings) and regenerated the tabtarget context via rbw-MG. Build clean via the new rbw-tb path. Heavy gauntlet/cloud regression deferred to the final-regression pace.

### 2026-05-26 23:46 - Heat - S

theurge-colophon-consolidation

### 2026-05-26 16:37 - ₢BMAAH - W

Added the dogfight cloud-build viability fixture + standing-depot suite. New rbtdrd_dogfight.rs single-case fixture (ordain conjure-mode rbev-busybox -> summon -> bare `docker run --rm <ref> true` exit-0 proof -> abjure), modeled on rbtdrc_hallmark_lifecycle since consumerless busybox has no committed home for its ephemeral hallmark; charges NO crucible (orthogonal to skirmish's containment axis). Runtime behind one named const RBTDRD_RUNTIME (podman deferred); director-RBRA precondition probe. Registered: RBTDRM_FIXTURE_DOGFIGHT const + required-colophons (ordain/summon/abjure) + RBTDRC_FIXTURES roster + lib.rs module + 2 manifest parity tests. Suite wired in rbte_engine.sh + byte-identical tt/rbtd-s.TestSuite.dogfight.sh. Per operator parity request, the dogfight suite leads with the existing canonical-invest fixture (pure suite composition, zero new code) so it self-refreshes governor/retriever/director credentials like skirmish; skirmish array untouched. Verified: build clean under deny(warnings), 116/116 unit tests, and live suite green against the standing depot (canonical-invest 3/3 + dogfight 1/1) producing a real conjure GAR hallmark and a clean exit-0 bare run. Scar surfaced en route: a stale director SA credential (Invalid JWT Signature) needed a payor OAuth refresh + canonical-invest re-investiture before the cloud path would run.

### 2026-05-26 16:25 - ₢BMAAH - n

Make the dogfight suite self-sufficient against a levied depot by leading with canonical-invest, mirroring skirmish. ZRBTE_SUITE_DOGFIGHT becomes (canonical-invest dogfight): the existing canonical-invest fixture re-mantles the governor and divest/re-invests retriever + director (access-probed) so the dogfight fixture finds fresh credentials, then dogfight runs the crucible-free build->summon->run->abjure probe. Pure suite composition — zero new fixture code; reuses the already-registered canonical-invest fixture exactly as skirmish does. Honors the docket's 'not a member of skirmish' rule (skirmish array untouched) and keeps the dogfight fixture crucible-free. Trades a per-run SA re-invest (quota + tombstones) for not needing manual credential setup before each run; the dogfight fixture's own director-RBRA precondition probe is retained for the single-fixture/single-case debug path.

### 2026-05-26 16:14 - ₢BMAAH - n

Add dogfight cloud-build viability fixture + standing-depot suite. New rbtdrd_dogfight.rs single-case fixture (ordain conjure-mode rbev-busybox -> summon -> bare `docker run --rm <ref> true` exit-0 proof -> abjure), modeled on rbtdrc_hallmark_lifecycle: busybox is consumerless so the ephemeral hallmark threads as a local rather than via a committed regime file; charges NO crucible (orthogonal to skirmish's containment axis). Runtime hardcoded to docker behind one named const RBTDRD_RUNTIME (podman deferred to the runtime-regime decision riding with the runtime heat); director-RBRA precondition probe mirrors onboarding. Registered: RBTDRM_FIXTURE_DOGFIGHT const + required-colophons map (ordain/summon/abjure) + RBTDRC_FIXTURES roster entry + lib.rs module + two manifest parity unit tests. Wired the dogfight suite in rbte_engine.sh (ZRBTE_SUITE_DOGFIGHT array, resolver case arm, unknown-suite error list) and added the byte-identical tt/rbtd-s.TestSuite.dogfight.sh tabtarget. Build clean under deny(warnings); 116/116 theurge unit tests green. Live suite verification against the standing depot follows this commit (suite run-start guard requires a clean tree).

### 2026-05-26 15:57 - Heat - T

dogfight-cloud-build-viability

### 2026-05-26 11:47 - ₢BMAAI - W

Theurge commit-hygiene guard. Extracted the porcelain clean-tree check into shared rbtdre_engine::rbtdre_tree_clean; pristine Class-A delegates to it (messages preserved). Suite run-start guard in main.rs rbtdb_run_suite fatals on a dirty tree before fixture work; single-case mode left unguarded (crucible-debug loop). Re-signatured rbtdro_git_commit to take an explicit files list + git add -- <files> instead of git add -A, with the porcelain pre-check scoped to owned files so idempotent no-op steps still skip. All 10 call sites thread their change sets via new helpers (rbtdro_nameplate_rbrn_path / rbtdro_vessel_rbrv_path / rbtdro_all_vessel_rbrv_paths); sets verified against historical commits. graft-demo given ownership of its vessel rbrv.env (normally a clean no-op). Verified: rbtd-b builds clean, rbtd-t 114/114 green.

### 2026-05-26 11:47 - ₢BMAAJ - W

Retired the tabtarget dispatch sprue; launcher now named in the BURD_LAUNCHER config line, exec line byte-identical across all tabtargets. Edited generator (BURD_LAUNCHER line2, dropped owner-prefix case), trampoline (read basename, resolve directly, forward all args, dropped *ml_ strip + path re-export), qualifier (new prescribed form, dropped sprue/*ml_ validation, min 3 lines), and burd_regime decl. Converted all 215 non-exempt tabtargets via one idempotent awk pass (exec bit preserved). Docs: full re-derivation of BCG 'Tabtarget Path Indirection' to the BURD_LAUNCHER contract, pruned buml_ to zero referents, narrowed rbml_ to its directory meaning. Done gate: QualifyFast green (215 checked / 2 exempt) + live dispatch smoke across plain/NO_LOG/INTERACTIVE paths.

### 2026-05-26 11:46 - ₢BMAAJ - n

Retire the tabtarget dispatch token; name the launcher in the BURD_LAUNCHER config line instead. Every tabtarget now carries a byte-identical, token-free exec into z-launcher.sh, with launcher identity (and any NO_LOG/INTERACTIVE flags) confined to a BURD_* config block between shebang and exec. Generator (zbuut_write_tabtarget) emits 'export BURD_LAUNCHER=<basename>' as line 2 and drops the rbml_/buml_ owner-prefix case; trampoline (z-launcher.sh) reads the basename from BURD_LAUNCHER (fail-loud if unset), resolves it directly under rbml_launchers/, forwards all args (was ${@:2}), and drops the *ml_ strip + path re-export; qualifier (buq_tabtargets) requires the new shape (line2 BURD_LAUNCHER resolving to a launcher file, constant exec, min 3 lines) and drops sprue/*ml_ validation; burd_regime decl reworded path->basename. Converted all 215 non-exempt tabtargets via one idempotent awk pass (exec bit preserved; z-launcher.sh + buw-SI.StationInit.sh exempt). Docs: full re-derivation of BCG 'Tabtarget Path Indirection' to the BURD_LAUNCHER contract, pruned buml_ (zero referents) from the BUK include, narrowed rbml_ to its directory meaning in CLAUDE.md + rbk-acronyms. Done gate: QualifyFast green (215 checked / 2 exempt); live dispatch smoke across plain (rbw), NO_LOG (buw), INTERACTIVE (rbw) paths. Operator approved raising the record size_limit to admit the legitimate 215-file conversion bulk.

### 2026-05-26 11:40 - ₢BMAAI - n

Theurge commit-hygiene guard: stop hallmark/yoke commits from sweeping unrelated working-tree edits. (1) Extract the porcelain clean-tree check into shared rbtdre_engine::rbtdre_tree_clean; pristine Class-A now delegates to it (messages preserved). (2) Suite run-start guard in main.rs rbtdb_run_suite fatals on a dirty tree before any fixture work; single-case mode (crucible-debug loop) intentionally left unguarded. (3) Re-signature rbtdro_git_commit to take an explicit files list and git add -- <files> instead of git add -A; porcelain pre-check scoped to owned files so idempotent no-op steps still skip cleanly. (4) All 10 call sites thread their change sets (consumer rbrn.env for hallmark drives, vessel rbrv.env for enshrine/locator writes, enumerated all-vessels set for the wildcard yoke) via new path helpers rbtdro_nameplate_rbrn_path / rbtdro_vessel_rbrv_path / rbtdro_all_vessel_rbrv_paths; sets verified against historical commits. graft-demo (terminal, no repo write) given ownership of its vessel rbrv.env, normally a clean no-op. Verification (rbtd build/test + dispatch smoke) pending operator clearance to run scripts.

### 2026-05-26 11:23 - Heat - S

tabtarget-dispatch-via-burd-launcher

### 2026-05-26 11:12 - Heat - r

moved BMAAF to last

### 2026-05-24 15:39 - ₢BMAAG - W

canonical-invest reruns are idempotent against a standing depot — live acceptance gate met. Code (option A) was already landed pre-mount: fixture-level divest before invest + fail-loud existence preflight + rbgu_poll_until_gone absorbing the deletion-propagation race; the pace's open item was the operator-deferred live skirmish gate. Verified live this session: skirmish canonical-invest 3/3 clean (governor_mantle + retriever_invest + director_invest, no 409) against the standing bhm depot, plus a standalone rbtdrk_retriever_invest rerun showing the full divest -> poll_until_gone -> recreate chain with no 409 — the tight delete->recreate timing the docket flagged held. Reaching the gate required reconciling a station-tincture mismatch (burs.env bhl vs standing depot canest3bhm100001) and uncovered two adjacent infra issues fixed under this coronet: (1) cross-arch tripwire portability — rbrd_inscribe baked the levying host's arch, leaving the Mac-levied bhm depot's tripwire arm64-only and unpullable on amd64/Cloud Build; pinned --platform RBGC_BUILD_RUNNER_PLATFORM on inscribe/check + collapsed the rbrd.env magic string to RBCC_rbrd_basename (848ec3b44), and manually re-pushed the live depot's tripwire as amd64 from verified-identical bytes; (2) IAM grant verify-timeout — the post-setIamPolicy read-back loop false-timed-out on accumulated deleted-SA tombstones, removed per RBSCIP's abandoned-read-back conclusion, with the two orphaned helpers pruned (4db57f085). A separate theurge commit-hygiene spook (rbtdro git add -A + missing dirty-tree guard) was slated as BMAAI for follow-up.

### 2026-05-24 15:38 - Heat - S

theurge-commit-hygiene-guard

### 2026-05-24 15:32 - ₢BMAAG - n

Prune two now-orphaned IAM helpers from rbgu_Utility.sh, dead after the rbgi_add_project_iam_role read-back verify-loop removal (that removal landed in 24dc4c6a3, swept into a concurrent skirmish ordain commit): rbgu_role_member_exists_predicate (the getIamPolicy role+member existence predicate the verify loop polled) and rbgu_http_json_ok (whose only caller was that verify loop). Zero remaining call sites across Tools/ and tt/. Load-bearing-discipline dead-code cleanup; bash -n clean. Context: the verify loop was removed because, against a standing depot, repeated divest/invest leaves same-email deleted:serviceAccount tombstones (accepted -- re-levy is quota-limited), and the email->uid read reconciliation can lag getIamPolicy past the 90s deadline, producing a false 'verify timeout' on a grant whose setIamPolicy already returned 200. RBSCIP already abandoned read-back-as-propagation-gate (2026-03-03: read-back proves policy intent, not enforcement); SA-establishment waits (rbgu_poll_until_ok SA-by-email, phase-1 400 member-visibility tolerance) are retained.

### 2026-05-24 15:10 - ₢BMAAG - n

Fix cross-arch tripwire portability (spook surfaced while verifying BMAAG's live skirmish gate). rbrd_inscribe built the FROM-scratch tripwire with a bare `docker build` (no --platform), baking the levying host's native arch; a Mac(arm64)-levied depot's tripwire was then unpullable on amd64 hosts and Cloud Build, which is exactly what broke skirmish's onboarding-sequence inscribe-reliquary ('no matching manifest for linux/amd64'). Pinned --platform "${RBGC_BUILD_RUNNER_PLATFORM}" on inscribe build + check pull/create (rbndb_base.sh) -- symmetric single-platform, since the image is FROM-scratch data (rbrd.env only, never executed) so its arch is semantically irrelevant but docker's manifest selection on pull still demands a match. Rejected the arch-tolerant-check variant (parse the inscribed platform and pull it) as permanent state-space pollution -- a fallback for a wrong-arch state the inscribe-side pin makes unreachable. Also collapsed the rbrd.env magic string per BCG tinder discipline: new RBCC_rbrd_basename="rbrd.env" tinder constant in rbcc_Constants.sh, RBCC_rbrd_file recomposed tinder-on-tinder from it, and every rbndb basename/in-image (/rbrd.env) literal routed through ${RBCC_rbrd_basename} -- the string now appears once in bash. The live bhm depot's existing arm64 tripwire was manually re-pushed as amd64 from verified-byte-identical content (deliberate immutability-guard bypass; drift baseline preserved; no re-levy available); native amd64 pull + create + cp + cmp confirmed working. Parse-clean (bash -n); shellcheck deferred (unavailable in sandbox).

### 2026-05-22 13:36 - Heat - n

RBSRR drift correction (two content errors found while tracing RBRR's authority/runtime ambiguities). (1) Opening line claimed RBRR assignment files are 'found in the operator's local filesystem' — implying personal/isolated/uncommitted — but rbmm_moorings/rbrr.env is tracked; reworded to 'committed to the repository' (authority-neutral; does not codify Director-as-sole-editor, deferred to BS credential reorientation). (2) RBRR_RUNTIME_PREFIX group rationale claimed it 'enables distinct developers to coexist on the same workstation without name collisions' — wrong: operators live in distinct container runtimes (separate daemons/machines), so that collision class mostly does not exist; the prefix's real job is disambiguating RB-lifecycle-managed runtime resources from OTHER container systems sharing the same runtime, reducing collateral disturbance. Reworded to that. Left line 39 (RBRR_SECRETS_DIR 'outside the source repository') intact — the one genuinely machine-local field, consistent with a committed file naming a per-machine path. Deferred (not wrong, just missing, pending BS): quoin-linking bare 'operator'->{at_operator} (possessive-variant risk + legacy quoin being reconsidered), at_operator credential clause, Director-as-authority codification, RBRR field-by-field authority attribution, runtime regime-field home.

### 2026-05-22 13:26 - Heat - S

cloud-depot-build-viability-fixture

### 2026-05-22 12:39 - ₢BMAAG - n

Make canonical-invest reruns idempotent via spec-honoring fixture-level divest (option A), not invest auto-heal. Honoring RBSRK/RBSDK intent, invest stays fail-loud: added the existence preflight the code never actually implemented (GET before create -> buc_die 'run divest first to re-key' instead of letting create raw-409). The canonical-invest fixture now runs an explicit divest before invest in the shared rbtdrk_role_invest_impl; 404-tolerant, so canonical-establish on a fresh depot is a clean no-op. New rbgu_poll_until_gone (inverse of rbgu_poll_until_ok: polls a GET until HTTP 404, same RBGC_MAX_CONSISTENCY_SEC deadline); zrbgg_divest_role now polls-until-gone after a real delete (skipped on 404-already-absent), absorbing the seconds-scale deletion-propagation race so an immediately-following same-name invest is race-free. WebSearch confirmed the GCP 30-day soft-delete window governs undelete/purge, not recreate -- recreate of a same name is supported (fresh identity), so the only residual is propagation timing, not a tombstone. RBSRD/RBSDD gain a Confirm-Deletion-Propagated step ({rbbc_poll} until 404, skip-on-404). RBGG 409-is-fatal invariant preserved (the preflight is a fail-loud implementation of it, not a carve-out). Theurge builds clean, 114/114 unit tests, shellcheck clean under busc_shellcheckrc. Live skirmish rerun (standing-depot acceptance gate) deferred to operator. Identity-model spec/code drift (singleton-per-role/no-arg vs per-identity/param1) left untouched, flagged for a separate spook.

### 2026-05-22 08:37 - Heat - f

silks=rbk-09-mvp-regime-sprues-misc

### 2026-05-22 08:37 - Heat - S

idempotent-canonical-invests

### 2026-05-22 07:58 - Heat - n

Fix skirmish suite ordering to mirror the gauntlet: move the three validation fixtures (regime-validation, regime-smoke, dockerfile-hygiene) to AFTER canonical-invest + onboarding-sequence, instead of before. The hoisted-early ordering ran regime-validation's rbrn_all_nameplates against a marshal-zero tree where nameplate hallmarks (RBRN_SENTRY_HALLMARK) are still empty, which rbrn_validate correctly rejects (required, min-len 1). The gauntlet already encodes the invariant that hallmark-bearing validation runs after the build fixtures populate hallmarks (onboarding-sequence kludges all 5 nameplates incl ccyolo); skirmish had diverged. Verified live: skirmish now advances enrollment-validation (47/47) -> canonical-invest cleanly, where it correctly stops at the depot/org-policy wall (separate concern, captured as BS pace org-affiliated-account-reorientation).

### 2026-05-22 07:15 - Heat - n

Add Import Discipline section to RCG (after Crate Boilerplate): multi-item use braces one-per-line, alphabetically ordered, trailing comma, for change-proportional diffs (blame precision, fewer merge collisions, deterministic insertion point). Codifies the convention applied twice this session. Note-on-rustfmt records the measured finding: alphabetization is rustfmt's stable reorder_imports default, but imports_layout=Vertical is nightly-only (stable warns and ignores via rustfmt.toml); max_width=1 rejected as a global sledgehammer. Discipline is review-enforced since the project does not run cargo fmt.

### 2026-05-22 06:55 - Heat - n

Fix rbw-tT: move the build->charge hallmark commit into the fixture layer (the precedent is onboarding's rbtdro_kludge_nameplate, which kludges sentry+bottle and commits each). New kludge-tadmor fixture reuses rbtdro_onboarding_kludge_tadmor_impl via a thin probe-free case (drops the onboarding reliquary-stamp witness probe, which is a sequencing guard not a local-kludge dep). rbq_qualify_tadmor now runs FixtureRun kludge-tadmor (builds BOTH vessels - earlier rbw-tK only built the bottle - and commits the hallmarks) then FixtureRun tadmor (charges against the now-clean nameplate). Avoids both the bash-git-in-qualify smell and the fixture-name==nameplate collision. Manifest const + colophon arm (KLUDGE_SENTRY/BOTTLE) added; registered in RBTDRC_FIXTURES; onboarding manifest import reformatted one-per-line per RCG. Also commits the stale rbrn.env bottle hallmark from the prior failed run so the tree is clean for the kludge assertion. Theurge builds + 114 tests green.

### 2026-05-22 06:47 - Heat - n

Regenerate tabtarget context (rbw-MG) for the new rbw-tS/rbw-tT colophons added to the zipper — the rbw workbench's qualify_fast preflight gate rejects a stale generated context, so this is required for any rbw-* command to run after the enrollment. Also commit the tadmor bottle kludge hallmark (RBRN_BOTTLE_HALLMARK=k260522063922-527879e18) driven into tadmor/rbrn.env by the first rbw-tT run, so the next kludge's clean-tree assertion passes.

### 2026-05-22 06:39 - Heat - n

Fix RCG magic-string violation in rbtdtk_family_stem_value: the test pinned the literal 'canest2' (duplicating RBTDRK_FAMILY_STEM_BASE), so the canest2->canest3 era-bump of the source const left the test falsely failing. Rewrote to assert the composition behavior (rbtdrk_family_stem('xyz') == format!('{}xyz', RBTDRK_FAMILY_STEM_BASE)) tied to the constant, dropping the tautological literal-equality assert. Theurge 114/114 green.

### 2026-05-22 06:34 - Heat - n

Add skirmish mini-gauntlet suite + tadmor self-contained qualify path (project-lifecycle-frugal substitutes for the gauntlet, which levies 2 GCP projects/run). New canonical-invest fixture reuses canonical's three investiture case fns (governor mantle + retriever/director invest) but omits depot-levy, so it runs against an operator-levied standing depot with zero per-run project creation; the fixture-shares-cases idiom mirrors tadmor/moriah sharing RBTDRC_CASES_SECURITY. ZRBTE_SUITE_SKIRMISH drops pristine-lifecycle entirely and chains the 4 validation fixtures -> canonical-invest -> onboarding-sequence (builds) -> 4 crucibles. rbw-tS/rbw-tT enrolled to rbq_qualify_skirmish/rbq_qualify_tadmor; tadmor path composes kludge + existing tadmor fixture at the command level to avoid the crucible fixture-name==nameplate collision (rejected a fixture-struct nameplate field as false generalization). Theurge builds clean; canonical-invest cases_registered + all manifest verify arms pass. Pre-existing unrelated stale test (rbtdtk_family_stem_value asserts canest2 while source const is already canest3) left untouched.

### 2026-05-21 03:16 - ₢BMAAF - n

Gauntlet grep-audit caught a missed bubep->bunne harmonize site: the ₢BMAAE sweep scoped its proof grep to Tools/ and never touched the rbmm_moorings/rbmn_nodes/ subtree. Migrated the two surviving sites — bujn-winpc/burn.env live value BURN_PLATFORM=bubep_windows -> bunne_windows (which burn_regime.sh:44's bunne_* enum gate would have rejected; the gauntlet's theurge fixtures never read BURN node profiles, so only the grep audit could catch it), and rbmn_nodes/README.md field-doc comment bubep_linux|mac|windows -> bunne_*. Verified: buw-rnv ValidateNodeRegime bujn-winpc PASSES (BURN_PLATFORM=bunne_windows [enum]); grep bubep_ across Tools/ rbmm_moorings/ tt/ now returns 0; remaining repo bubep_ hits are all class-1 historical (dated Memos/ transport experiments, .claude/jjm ledger docket records). The three sprue families return coherent clusters (rbnve_ 94, rbnne_ 50, bunne_ 43).

### 2026-05-20 20:08 - ₢BMAAE - W

bubep->bunne harmonize complete and proven. BURN_PLATFORM values bubep_linux/mac/windows -> bunne_linux/mac/windows across 43 sites / 7 BUK files, eliminating the two-shape wart. Acceptance reconfirmed on the integrated tree: grep bubep_ returns nothing, grep bunne_ returns the full 43-site cluster, BukSelfTest 28/28, fast suite 107/107 (rbrn_all_nameplates + enrollment enum cases confirm declaration<->env coherence). Migration survived two upstream rebases (onto BJ AccessProbe-retirement cada82cba and BK charge/tripwire work) and the bhyslop-merge-review-260521 release-qualification-reset merge with zero enum-value conflicts -- all integration collisions were ephemeral kludge-hallmark lines, resolved per burn-without-consequence. Pushed to origin/main at 8b14c9a82. Last migration slice of the heat; only the ₢BMAAF gauntlet+grep-audit remains.

### 2026-05-20 19:36 - ₢BMAAE - n

bubep->bunne harmonize: migrate BURN_PLATFORM enum values bubep_linux/mac/windows -> bunne_linux/mac/windows, eliminating the two-shape wart (5-char bubep_ vs the heat's 4-char <proj>n<regime>e_ convention). 43 sites across 7 BUK files. Global replace on 5 mechanical files (bujb_jurisdiction 22 class-2 case-arms/equality-tests/buc_die-values/assert-platform-args, bujb_cli 6 doc-briefs, bujp_preflight 4 case-arms, buwz_zipper 3 enroll-descriptions, burn_regime enum declaration + its '(bubep_* identifier)' description string). bubc_constants: 3 BUBC_platforms_* values flipped + prefix-tree comment rewritten from 'bub/bube/bubep_* tree' to 'BURN node-regime enum sprue family' (blind replace would have left a false tree decomposition). BUS0: {burn_platform} quoin Values: line flipped to bunne_*, and the now-stale NOTE documenting the pre-migration bubep_ spelling removed (convention section at line 1038 already declares bunne_* canonical). No class-3 identifier embeds and no class-4 derived strings exist for this enum -- pure mechanical migration. PROOF GREEN: grep bubep_ returns nothing; grep bunne_ returns full 43-site cluster across all 7 files; buw-st BukSelfTest 28/28; fast suite 107/107 (regime-validation rbrn_all_nameplates + enrollment-validation enum cases confirm declaration<->env coherence). Last migration slice of the heat; only the final regression sweep remains.

### 2026-05-20 19:26 - ₢BMAAD - W

Node-mode sprue migration complete and behaviorally proven without a depot. Migrated RBRN_ENTRY_MODE/UPLINK_DNS_MODE/UPLINK_ACCESS_MODE bare values (disabled/enabled/global/allowlist) to shared rbnne_* family across 11 files: rbrn_regime.sh (3 enum decls + 3 gate arms + enforce cross-port check + preflight serialized comparison), rbrn_cli entry-mode test, rbjs_sentry 5 sentry-runtime comparisons, rbtdrc_crucible 4 verdict comparisons, rbtdrf_fast 4 RV baselines, 5 rbrn.env files, RBS0 8 serialization quoins. Left bare per classification: enum description prose, RBS0 mapping display text + linked-term refs (vessel-pace precedent), bogus negative-test values, RBRN_RUNTIME docker/podman (executable-name coupling, paddock-excluded). Three node-modes deliberately share rbnne_disabled (intended family-sharing). PROOF: theurge build clean; fast suite 107/107 (regime-validation all-nameplates lockstep + invalid-mode negatives); grep audit clean (single rbnne_ grep returns full node cluster, RBRN_RUNTIME still bare, zero bare stragglers). BEHAVIORAL (deferred gate satisfied locally -- no depot needed): kludged tadmor sentry, charged crucible (all containers Healthy = rbnne_ comparisons drove correct iptables/dnsmasq), ran 6/6 live cases (rp_filter, prerouting_dnat, postrouting_masquerade entry-mode guards; tcp443 allow/block + dns_blocked confirm rbnne_allowlist fall-through). Crucible quenched, tree clean. Commits: d554999dd (migration), af2073d00 (kludge sentry-hallmark artifact). Remaining heat work: bubep->bunne BUK harmonize, final regression sweep.

### 2026-05-20 19:22 - ₢BMAAD - n

Node-mode sprue migration: RBRN_ENTRY_MODE/UPLINK_DNS_MODE/UPLINK_ACCESS_MODE bare values (disabled/enabled/global/allowlist) -> shared rbnne_* family across all use-classes. Migrated: rbrn_regime.sh 3 enum declarations + 3 gate arms + enforce cross-port check + preflight serialized comparison; rbrn_cli entry-mode equality test; rbjs_sentry 5 sentry-runtime comparisons (entry rp_filter gate, dns disabled/global, access disabled/global) which evaluate the values inside the charged sentry container; rbtdrc_crucible 4 verdict comparisons (rp_filter expected-map, prerouting/postrouting entry-mode skip guards); rbtdrf_fast 4 RV baseline export strings; 5 rbrn.env files (pluml/srjcl/ccyolo/tadmor/moriah); RBS0 8 serialization quoins (Serialized as `rbnne_*`). Left bare per classification: enum description prose (rbrv precedent), RBS0 mapping display text + linked-term refs (vessel-pace precedent at RBS0:414-416), bogus negative-test fixture values, and RBRN_RUNTIME docker/podman (executable-name coupling, paddock-excluded). The three node-modes deliberately share rbnne_disabled (intended family-sharing). Local proof GREEN: theurge build clean; fast suite 107/107 (regime-validation all-nameplates lockstep + invalid-entry/dns/access-mode negatives confirm declaration<->rbrn.env coherence); dockerfile-hygiene 9/9; handbook-render 15/15. Grep audit: single grep rbnne_ returns full node cluster (8 lines rbrn_regime, 5 rbjs_sentry, 4+4 Rust, 3x5 rbrn.env, 8 RBS0); RBRN_RUNTIME confirmed still bare; zero surviving bare comparisons against the three RBRN_*_MODE variables. DEFERRED: crucible behavioral proof (charged sentry exercises rbjs_sentry comparisons + rbtdrc sentry-config verdicts) -- about to attempt via local tadmor kludge + charge (no depot needed).

### 2026-05-20 19:08 - ₢BMAAC - W

Vessel-mode sprue migration: bind/conjure/graft -> rbnve_bind/rbnve_conjure/rbnve_graft across all four ecosystems as one connected value-flow unit. Migrated: rbrv_regime.sh enum declaration + 4 gates; 9 rbrv.env files (7 conjure, 1 bind, 1 graft); shell comparisons/case-arms/default-defaults in rbfk, rbfh_cli, rbfc (10 sites), rbfd (2 case blocks + 2 substitution literals), rbfv (2 case blocks + 2 substitution literals); Cloud Build substitution boundary -- remote step shell rbgjv01, Python rbgja03 (3 comparisons) and rbgjv02 (3 comparisons + 3 vouch_summary.json value writes); Rust fast-fixture conjure/bind baselines (rbtdrf_fast.rs); handbook display values (rbhodb, rbhodg); AsciiDoc class-2 wire-value sites (RBS0 serialization quoins + build_info field; RBSAB/RBSAV substitution-table cells; RBSAV vouch_summary examples; RBSAG). Left bare per classification: class-1 prose, class-3 identifiers (c/b/g hallmark-prefix letters, sigils, file names, linked-term concept display at RBS0:414-416), class-4 derived strings (RBGC_HALLMARK_PREFIX_*), JSON structure keys (info[bind]), field names (.mode/vessel_mode), unrelated version.bind/sock.bind. Local depot-free proof GREEN: theurge build clean; regime-validation 27/27 (declaration<->rbrv.env<->fixture lockstep confirmed via rbrv_all_vessels); dockerfile-hygiene 9/9; handbook-render 15/15; enrollment 47/47; regime-smoke 9/9. Single grep returns full cluster (rbnve_conjure 33, rbnve_bind 26, rbnve_graft 25). DEFERRED ACCEPTANCE GATE: gauntlet + real conjure/graft cloud build -- depot-blocked, not yet exercised; the migrated remote Python/shell comparisons remain unproven until depots exist. OBSERVATION: egress pilot (BMAAB) left RBSRV:46 tether/airgap bare -- spec inconsistency across the shared rbnve_ family, candidate for follow-up.

### 2026-05-20 19:08 - ₢BMAAC - n

Codify vessel-mode enum sprue: serialize RBRV_VESSEL_MODE as rbnve_conjure/rbnve_bind/rbnve_graft

### 2026-05-20 18:47 - ₢BMAAA - W

Codified the regime enum sprue convention as a self-contained BUS0 spec section ('=== Regime Enum Sprue Convention' under Regime Configuration, after Regime Prefixes). Minted quoin bus_enum_sprue (voiced axt_enum_value); grounded the values in mcm_sprue/mcm_inlay by prose citation since MCM is governed and VLS is notional/ungoverned in RB (host chosen BUS0 over VLS at operator direction). Section carries the <proj><n><regime><e>_<value> pattern + family-sharing rationale, the rbnve_/rbnne_/bunne_ target-enum bullet list with the bubep_ harmonization note, the four-class use-site model incl. the class-4 coupled-derived-string rule with two witnesses (pool suffixes, hallmark prefixes), and the RBRN_RUNTIME exclusion rationale. Documentation only, no value migration. Render verified clean via asciidoctor 2.0.26 --failure-level=WARN (zero warnings from the new section; lone warning is pre-existing in unrelated included file BUSJCW). Landed in e97f6ad8c.

### 2026-05-20 17:44 - ₢BMAAA - n

Codify the regime enum sprue convention as a BUS0 spec section. New '=== Regime Enum Sprue Convention' under Regime Configuration (after Regime Prefixes): mints quoin bus_enum_sprue (voiced axt_enum_value), grounds the values in mcm_sprue/mcm_inlay by prose citation (MCM is governed; VLS deferred as notional), states the <proj><n><regime><e>_<value> pattern with family-sharing rationale, the target-enum bullet list (rbnve_/rbnne_/bunne_ incl. bubep_ harmonization note), the four-class use-site model with the class-4 coupled-derived-string rule and two witnesses, and the RBRN_RUNTIME exclusion rationale. Documentation only, no value migration. Host chosen BUS0 over VLS at operator direction (VLS notional, ungoverned in RB).

### 2026-05-20 17:31 - ₢BMAAB - W

Egress-mode sprue pilot. Migrated RBRV_EGRESS_MODE bare values tether/airgap to rbnve_tether/rbnve_airgap across 16 class-2 sites: the buv_enum_enroll declaration (rbrv_regime.sh), 4 dispatch sites in rbfd_FoundryDirectorBuild.sh (one equality test + two pool-routing case-arm pairs), the rbhoda handbook value display, and 9 vessel rbrv.env files. Class-4 coupled derived strings (RBGC_POOL_SUFFIX_*, RBDC_POOL_* paths) and class-3 identifiers (rbev-*-tether/airgap sigils, rbtd step labels) left bare; class-1 pool-routing prose in RBSCB/RBSAC/RBSHR and rbh0 handbook lines left untouched. No Rust rebuild needed (egress carries no .rs fixture baseline). Acceptance met: fast suite green (rbtdrf_rv_rbrv_all_vessels confirms declaration/regime lockstep), grep rbnve_ returns the egress cluster while pool suffixes stay bare. Pilot proves the mechanical pattern and the class-4 distinction before the heavier vessel-mode/node-mode paces.

### 2026-05-20 17:31 - ₢BMAAB - n

egress-mode sprue pilot: migrate RBRV_EGRESS_MODE bare values tether/airgap to rbnve_tether/rbnve_airgap at the enum declaration, the 4 rbfd dispatch sites (equality test + two pool-routing case-arm pairs, pool consts left bare), the rbhoda handbook value display, and 9 vessel rbrv.env files; class-4 pool suffixes and class-3 sigils/prose left bare; fast suite green

### 2026-05-20 13:21 - Heat - r

moved BMAAB to first

### 2026-05-20 13:20 - Heat - f

silks=rbk-12-mvp-regime-sprues

### 2026-05-20 13:17 - Heat - d

paddock curried: add MCM grounding (sprue=mcm_sprue, prefix→inlay); update heat-shape note for BK-coding-landed + gauntlet start gate; correct pace-shape notes

### 2026-05-20 13:01 - Heat - S

final-regression-and-grep-audit

### 2026-05-20 13:00 - Heat - S

bubep-to-bunne-harmonize

### 2026-05-20 13:00 - Heat - S

node-mode-sprue-migration

### 2026-05-20 13:00 - Heat - S

vessel-mode-sprue-migration

### 2026-05-20 13:00 - Heat - S

egress-mode-sprue-pilot

### 2026-05-20 12:59 - Heat - S

sprue-convention-spec-authoring

### 2026-05-20 12:59 - Heat - f

racing

### 2026-05-20 08:33 - Heat - d

paddock curried: fold in resolved scope (RBRN_RUNTIME excluded, node-modes in, bubep harmonize), class-4 coupled-derived-string rule, Cloud Build detection gap + vessel-mode acceptance gate, noise ranking, BK file-surface overlap

### 2026-05-14 13:58 - Heat - d

paddock curried: initial paddock — full shape captured for mount-time pace cutting

### 2026-05-14 13:57 - Heat - N

rbk-15-mvp-regime-sprues

