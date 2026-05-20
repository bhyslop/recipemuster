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
discovery recipe, not here, because ₣BK churns this heat's file surface
(see Heat-shape note below).

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

## Heat-shape note — ₣BK file-surface overlap

₣BK (moorings-cutover) is racing in parallel and sweeps essentially this
heat's entire file surface (`Tools/rbk/**/*.sh`, `rbtd/src/*.rs`,
`RBS*.adoc`) — but for path literals, not enum values. The two heats are
**semantically orthogonal** (different concerns, mostly different lines),
yet share files, so ₣BK causes continuous line-number churn here. This is
why this paddock records shape over site lists.

Implications, durable:
- Implement this heat against a **settled** tree (after ₣BK's literal and
  doc sweeps land), or interleave with git-pull discipline between
  commits. Avoid running two tree-wide sweeps blind.
- Sharpest co-edit point: the fast-fixture Rust file (₣BK's canonical
  sweep example and home of this heat's `export RBRV_*_MODE` baselines)
  and the `RBS*.adoc` specs.
- The bubep→bunne harmonize touches BUK files and is the one place this
  heat and ₣BK overlap *semantically*. It is the natural last slice;
  coordinate with ₣BK before mounting it.

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
   including the class-4 "coupled derived string stays bare" rule (two
   witnesses) and the `RBRN_RUNTIME` exclusion rationale.
2. Egress-mode pilot — `rbnve_tether`/`_airgap`. Smallest, cleanest;
   proves the mechanical pattern and the class-4 distinction.
3. Vessel-mode — `rbnve_bind`/`_conjure`/`_graft`. Heaviest; crosses
   bash/adoc/Python(Cloud Build)/Rust. Acceptance gate = real conjure +
   graft cloud build.
4. Node-mode — `rbnne_*` for ENTRY/DNS/ACCESS. High English-word noise,
   heavy classification.
5. bubep → bunne harmonize — BUK files; last slice, coordinate with ₣BK.
6. Final regression sweep — gauntlet end-to-end + per-family grep audit.

Slate as thin dockets at mount-time (depth in slate commit messages); do
not pre-cut detailed site lists from this paddock.