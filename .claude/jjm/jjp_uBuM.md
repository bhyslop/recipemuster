## Context

Regime enums in this project carry their values as bare English words —
`conjure`, `bind`, `graft` for `RBRV_VESSEL_MODE`; `tether`, `airgap` for
`RBRV_EGRESS_MODE`; `0/1/2/3` for `BURE_VERBOSE`. These bare values
appear scattered across bash, AsciiDoc, Python (Cloud Build steps),
YAML (Cloud Build substitutions), and Rust (test fixtures), with no
cluster-discoverable marker tying any one value to its enum family.
This makes mode renames cross-language hand-search-and-replace work,
prone to silent drift.

There is one prior in-tree exemplar of the prefixed-value approach:
`bubep_linux | bubep_mac | bubep_windows` for `BURN_PLATFORM`. It works
but uses a different shape from what this heat will adopt; harmonization
is part of the sweep.

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

Concrete renames:

| Variable          | Bare value | Sprue value     |
|-------------------|------------|-----------------|
| RBRV_VESSEL_MODE  | conjure    | `rbnve_conjure` |
| RBRV_VESSEL_MODE  | bind       | `rbnve_bind`    |
| RBRV_VESSEL_MODE  | graft      | `rbnve_graft`   |
| RBRV_EGRESS_MODE  | tether     | `rbnve_tether`  |
| RBRV_EGRESS_MODE  | airgap     | `rbnve_airgap`  |
| BURN_PLATFORM     | bubep_linux| `bunne_linux`   |
| BURN_PLATFORM     | bubep_mac  | `bunne_mac`     |
| BURN_PLATFORM     | bubep_windows | `bunne_windows` |

Vessel-mode and egress-mode share `rbnve_` because both are vessel-regime
enums — single grep finds both clusters. This is a deliberate property
of the convention, not an accident.

## Existing exemplar — `bubep_*` harmonize-or-grandfather

`burn_regime.sh:44` already declares prefixed values, but with shape
`bubep_*` (5 chars, includes per-variable `p` discriminator) rather than
`bunne_*` (4 chars, no per-variable discriminator). Two coherent
positions; pick at migration time:

- **Harmonize**: rename `bubep_*` → `bunne_*` in the same sweep. ~17 use
  sites in `bujb_*.sh`. Eliminates two-shape-cognitive-cost wart.
- **Grandfather**: leave `bubep_*` alone, apply new convention to
  newly-prefixed enums only. Cheaper, leaves asymmetry permanent.

Lean toward harmonize per load-bearing-complexity discipline.

## Scope — sweep covers all five language ecosystems

| Language       | Where to look                                              |
|----------------|------------------------------------------------------------|
| Bash           | `Tools/buk/*.sh`, `Tools/rbk/**/*.sh`, `tt/*.sh`           |
| AsciiDoc       | `Tools/*/vov_veiled/*.adoc` (regime + spec docs)           |
| Python         | `Tools/rbk/rbgja/*.py`, `Tools/rbk/rbgjv/*.py`             |
| Cloud Build YAML | `_RBGA_VESSEL_MODE`, `_RBGV_VESSEL_MODE` substitutions in `rbfd_*` and `rbfv_*` step assembly |
| Rust           | `Tools/rbk/rbtd/src/*.rs` (test-fixture baseline strings)  |

## Inventory needs

Three classes per enum value:

1. **Declaration sites** — `buv_enum_enroll` and `buv_gate_enroll` lines.
   Authoritative source of truth for the value set.
2. **Code use sites** — equality tests, case-statement arms, JSON
   literals, conditional dispatch. Mechanical to find.
3. **Comment / prose use sites** — bash/Rust/Python comments mentioning
   the value, AsciiDoc prose mentioning the value. Disambiguation is
   judgment work (see comment classification below).

## Discovery recipe

- `grep -rn 'buv_enum_enroll' Tools/buk/ Tools/rbk/` — declarations
- Per discovered value V: `grep -rn '\bV\b' Tools/` filtered by
  extension for code sites
- Comment search per V:
  - `grep -rn '#.*\bV\b' Tools/` — bash/Python comments
  - `grep -rn '//.*\bV\b' Tools/rbk/rbtd/src/` — Rust comments
  - `grep -rn '\bV\b' Tools/*/vov_veiled/*.adoc` — AsciiDoc prose
- Cross-walk `buv_gate_enroll` invocations against the discovered enum
  set to verify no enum was missed
- Cross-check `RB[SR]*` and `BU[S]*` regime spec `.adoc` files for
  prose mentions in spec descriptions

## Comment classification — judgment work

Bare-word grep on comments yields three classes:

1. **Phase / concept references** — "the conjure phase", "the graft
   pipeline" — referring to the workflow, not the enum value. **Leave
   alone**; renaming makes the prose worse.
2. **Enum-value references** — "vessel mode is conjure", "if mode is
   bind" — referring to the actual variable value. **Migrate** to
   sprue value or use canonical reference like `RBRV_VESSEL_MODE =
   {rbnve_conjure}`.
3. **Identifier embeds** — `RBSAC-ark_conjure.adoc`, `rbho_conjure_*`
   function names — the bare word appears as part of an identifier.
   **Leave alone**; identifier names are their own naming concern.

The migration paces' work: walk the candidate list, classify each hit,
migrate class 2, leave classes 1 and 3.

## Posture

**Burn-bridges migration.** No backwards compatibility. Depots reform
on execution, vessels re-charge, regime files get edited in place.
Single-sweep target. Operator pre-flight: `rbw-MZ` before sweep, all
crucibles quenched.

**Deep-test stragglers.** `regime-validation`, `dockerfile-hygiene`,
and `gauntlet` fixtures catch any bare-value site that wasn't migrated
because the enum-validation gate in `buv_enum_enroll` will reject the
non-prefixed value once the regime declaration is updated.

## What done looks like

- All bare enum values (`conjure`, `bind`, `graft`, `tether`, `airgap`,
  optionally `bubep_*`) replaced with sprue-prefixed values across all
  five language ecosystems
- `buv_enum_enroll` declarations updated to expect the new values
- All `rbrv.env` and any other regime files updated in place
- `regime-validation` fast fixture green
- `dockerfile-hygiene` fixture green (smoke proof of vessel-mode logic
  working with new values)
- `gauntlet` suite green end-to-end
- A single grep per sprue family (`grep -rn rbnve_ Tools/`,
  `grep -rn bunne_ Tools/`) returns the complete cluster

## Out of scope for this heat

- Sprue convention applied to non-regime-enum domains (status codes,
  JSON wire-format keys, etc.) — separate concern, separate heat
- Convention reformulation for `BURE_VERBOSE` integer values — bare
  ints aren't the same problem class
- Backwards-compatibility shims of any kind

## Pace shape (deferred to mount-time)

Likely sequence when this heat is mounted:

1. Inventory pace — produce the full site listing across all five
   languages, classify comments, recommend cadence
2. Convention spec authoring — formalize the convention as a spec
   section so future readers find it
3. Migration paces — cadence TBD (per regime, per variable, or per
   language slice; inventory feeds the decision)
4. Final regression sweep — gauntlet end-to-end + grep audit

Cadence and pace decomposition are mount-time decisions; do not
pre-cut paces from this paddock.