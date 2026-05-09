## Goal

Mint the unprefixed JSON property names and enum values in `.claude/jjm/jjg_gallops.json` so wire-level tokens follow the project's prefix discipline. The kit's `jjrt_types.rs` types are already prefixed (quoins, in MCM terms); this heat introduces the matching wire-side names (sprues — inlays once they follow discipline).

## Sprue scheme

Three sub-letters after `jj`:

- **X** (constant): `g` — Gallops domain
- **Y** (table-letter): the carrier struct in `jjrt_types.rs` — `r` for `jjrg_Gallops` (root), `h` for `jjrg_Heat`, `p` for `jjrg_Pace`, `t` for `jjrg_Tack`. For enum values, Y is the carrier of the field that holds the enum
- **Z** (role-letter): `n` for property name, `e` for enum value

The suffix after the underscore is the Rust field name (for properties) or the lowercase variant name (for enum values).

## Migration recipe

Per target, two edits:

1. Add `#[serde(rename = "<minted>", alias = "<old-wire-name>")]`. Build. Run any state-mutating jjx call to round-trip the gallops file through the new wire-names.
2. Remove the transitional `alias` line.

The blanket `#[serde(rename_all = "lowercase")]` comes off `jjrg_PaceState` and `jjrg_HeatStatus` once explicit per-variant renames replace it. The pre-existing `alias = "primed"` on `Bridled` stays as legacy compat — outside this heat's window.

## Batches and order

Carrier-isolated batches keep the silks doppelganger (Heat + Tack) and the three Gallops-vs-Heat near-doppelgangers (heats/paces, heat_order/order, next_heat_seed/next_pace_seed) in distinct paces:

1. **mint-heat-silks-pace-states** — `jjghn_silks` plus the four `jjrg_PaceState` variants. Proves the recipe end-to-end.
2. **mint-gallops-root-sprues** — the four properties on `jjrg_Gallops`.
3. **mint-heat-rest-and-status** — the five remaining `jjrg_Heat` properties plus the three `jjrg_HeatStatus` variants.
4. **mint-pace-and-tack-sprues** — `jjgpn_tacks` plus the six `jjrg_Tack` properties.

Subsequent dockets reference these by silks rather than re-listing targets.

## Spec touch

JJS0's prescriptive sprue declarations follow the AXLA mechanism: property-name sites (`axr_member` under `axr_record_json`) carry `axd_property` or `axd_positional` with the sprue inlined in body prose; enum-value sites (`axt_enum_value`) carry `axd_string` with the sprue in a `Serialized as <sprue> in JSON.` body line. JJS0 already carries these declarations for the gallops surface; the four mint paces below carry no JJS0 obligation — only the Rust serde renames and gallops-file rewrite. The same AXLA discipline extends across other specs via the foreign-sweep pace, which adds `axd_string` to existing-wire enum-value voicings outside the gallops surface.

## Out of scope

`.claude/jjm/jjs_studbook.json` (unconsumed by Rust source); the MCP tool wire format (different boundary); the gazette wire format (already prefixed `jjezs_*`); CLAUDE.md ceiling rule edit (completed at groom-time); a memo footnote about the relaxed ceiling (deferred, not blocking).

## Done when

`.claude/jjm/jjg_gallops.json` carries zero bare property names from the original 16 and zero bare enum values from the original 7. Build green; tests green.