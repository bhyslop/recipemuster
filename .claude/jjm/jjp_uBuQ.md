## Shape: one renderer, two registers

The kit has two console-output systems that grew years apart: `buc_*` (Bash
Console — operational output during command execution) and `buh_*`/`buym`
(handbook prose with the yawp/diastema inline-span concept). They overlap on
"color text on a line" but each owns capabilities the other lacks. The genuine
duplication is the **color/terminal-capability decision**: `buc` carries its own
*unconditional* ANSI palette (latent bug — ignores `NO_COLOR` and `TERM=dumb`),
while `buym` already does terminal-aware rendering via `buyf_format_yawp`.

The merge: collapse the two palettes/kitchens into one renderer, keep the two
public vocabularies. Mental model — **one kitchen, two storefronts.** `buh` is
already a thin front-end on the `buym` render core; this heat brings `buc` down
to the same structure (a front-end), so both consume one terminal-capability
brain.

## Locked decisions

- **Public signatures unchanged.** No churn across the ~142 `buc_*` files or ~42
  `buh_*` files. The merge is internal; call sites are untouched.
- **`buc` becomes a front-end** on the `buym` renderer (`buyf_format_yawp` over the
  `BUYC_*` palette). Its private source-time palette is retired.
- **Deep core.** `buc` messages may carry inline yawp spans (cmd/ui/link), not just
  whole-line color. This is the capability the merge unlocks.
- **Framing stays split — this is the two-register guarantee.** `buc` keeps
  operational framing (gray `ZBUC_CONTEXT` sigil, `BURE_VERBOSE` gating, transcript
  with `file:line`); `buh` keeps editorial framing (section headers, body indent,
  blank-line rhythm). The gray operation prefix lives in `buc` framing and must be
  structurally incapable of reaching handbook prose.
- **Render to two capabilities.** Display path resolves spans to color
  (`buyf_format_yawp`); transcript path resolves them to plain text
  (`buyf_strip_yawp`), so the log carries neither ANSI nor diastema bytes.
- **`buym` is the renderer's home for now.** Internal naming (the module reads as
  "handbook yelp") is a known sin, deferred to post-release.
- **Refactor-first.** `buc` output stays byte-identical except for new
  terminal-awareness. Actually *using* inline spans at `buc` call sites is a
  separate, later appetite — not this heat.

## Locked hazard

`buc` display shifts from source-time color literals to **kindle-time** `BUYC_*`
readonlys, so it now depends on `buym` having kindled. A cold `buc_die` (called
before any kindle) must not throw an unbound-variable error inside the error
path. Guarded via `zbuym_sentinel`; proven by an explicit cold-`buc_die` test.

## Done looks like

- One terminal-capability decision exists in the kit; `buc` and `buh` share it.
- `buc` output respects `NO_COLOR` and `TERM=dumb` (the latent bug is fixed).
- The duplicated tabtarget-glob (`buc_tabtarget` vs `buyy_tt_yawp`) is collapsed to
  one resolver.
- Full `fast` suite green; sampled operational *and* handbook output verified under
  `NO_COLOR=1` and `TERM=dumb`.