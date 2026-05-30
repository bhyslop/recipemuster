# Heat Trophy: rbk-12-harmonize-buc-and-buh

**Firemark:** ₣BQ
**Created:** 260521
**Retired:** 260529
**Status:** retired

## Paddock

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

## Paces

### extend-render-core-gray-and-strip (₢BQAAA) [complete]

**[260527-0815] complete**

## Character
Mechanical, purely additive to the render core.

Give `buym` the two capabilities `buc` will need as a front-end, without changing
any existing behavior:

- A gray ambient suitable for `buc`'s operation sigil — the current handbook
  palette has no gray. It must honor the existing terminal-capability kindle
  (empty when color is off).
- A plain resolver that strips diastema markers to text (no color; hyperlinks
  degrade to "text <url>"), for `buc`'s transcript path. Sibling to the existing
  color resolver, independent of terminal mode.

Source: `Tools/buk/buym_yelp.sh` — palette lives in the kindle block; the color
resolver `buyf_format_yawp` is the structural sibling for the strip variant.

Done: the two additions exist with unit coverage; existing `buym` and `buh`
output is byte-unchanged.

**[260521-1009] rough**

## Character
Mechanical, purely additive to the render core.

Give `buym` the two capabilities `buc` will need as a front-end, without changing
any existing behavior:

- A gray ambient suitable for `buc`'s operation sigil — the current handbook
  palette has no gray. It must honor the existing terminal-capability kindle
  (empty when color is off).
- A plain resolver that strips diastema markers to text (no color; hyperlinks
  degrade to "text <url>"), for `buc`'s transcript path. Sibling to the existing
  color resolver, independent of terminal mode.

Source: `Tools/buk/buym_yelp.sh` — palette lives in the kindle block; the color
resolver `buyf_format_yawp` is the structural sibling for the strip variant.

Done: the two additions exist with unit coverage; existing `buym` and `buh`
output is byte-unchanged.

### buc-shared-render-path-kindle-safe (₢BQAAB) [complete]

**[260527-0904] complete**

## Character
The risk pace. Careful — touches the busiest output path and the error path.

Introduce `buc`'s internal display path: gate on `BURE_VERBOSE`, apply the gray
`ZBUC_CONTEXT` operation sigil, and render the message through the shared
`buyf_format_yawp` (color) — superseding the current raw-string display in
`zbuc_print`. Wire the transcript half to render plain via the strip resolver so
logs carry neither ANSI nor diastema bytes.

Locked hazard: `buc` color shifts from source-time literals to kindle-time
`BUYC_*` readonlys, so display now depends on `buym` having kindled. A cold
`buc_die` (no prior kindle) must not throw unbound-variable inside the error
path. Guard with `zbuym_sentinel`; prove with an explicit cold-`buc_die` test.

Source: `Tools/buk/buc_command.sh` — `zbuc_print`, `zbuc_tag_args`, `buc_die`.

Done: the internal path renders terminal-aware and logs plain; the cold-die test
is green.

**[260521-1009] rough**

## Character
The risk pace. Careful — touches the busiest output path and the error path.

Introduce `buc`'s internal display path: gate on `BURE_VERBOSE`, apply the gray
`ZBUC_CONTEXT` operation sigil, and render the message through the shared
`buyf_format_yawp` (color) — superseding the current raw-string display in
`zbuc_print`. Wire the transcript half to render plain via the strip resolver so
logs carry neither ANSI nor diastema bytes.

Locked hazard: `buc` color shifts from source-time literals to kindle-time
`BUYC_*` readonlys, so display now depends on `buym` having kindled. A cold
`buc_die` (no prior kindle) must not throw unbound-variable inside the error
path. Guard with `zbuym_sentinel`; prove with an explicit cold-`buc_die` test.

Source: `Tools/buk/buc_command.sh` — `zbuc_print`, `zbuc_tag_args`, `buc_die`.

Done: the internal path renders terminal-aware and logs plain; the cold-die test
is green.

### reexpress-buc-public-on-core (₢BQAAC) [complete]

**[260527-0917] complete**

## Character
Mechanical, broad but shallow — many functions, each a thin rewrite.

Route every public `buc_*` display function through the new internal path with
its semantic ambient color (step = white, success = green, warn = yellow prefix,
die = red prefix, code/bare = cyan, info = default). Signatures and observable
output stay unchanged except for terminal-awareness. Retire `buc`'s private
`BUC_*` palette.

Source: `Tools/buk/buc_command.sh` for the public wrappers. The few files outside
`buc_command.sh` that reference raw `BUC_*` constants must migrate to the kindled
palette — discover them with
`grep -rl 'BUC_\(red\|yellow\|green\|cyan\|white\|gray\|reset\)' Tools/ --include=*.sh`.

Done: all `buc_*` render via the shared core; the private palette is gone;
`bash -n` clean and the `fast` suite green.

**[260521-1009] rough**

## Character
Mechanical, broad but shallow — many functions, each a thin rewrite.

Route every public `buc_*` display function through the new internal path with
its semantic ambient color (step = white, success = green, warn = yellow prefix,
die = red prefix, code/bare = cyan, info = default). Signatures and observable
output stay unchanged except for terminal-awareness. Retire `buc`'s private
`BUC_*` palette.

Source: `Tools/buk/buc_command.sh` for the public wrappers. The few files outside
`buc_command.sh` that reference raw `BUC_*` constants must migrate to the kindled
palette — discover them with
`grep -rl 'BUC_\(red\|yellow\|green\|cyan\|white\|gray\|reset\)' Tools/ --include=*.sh`.

Done: all `buc_*` render via the shared core; the private palette is gone;
`bash -n` clean and the `fast` suite green.

### collapse-tabtarget-glob-clone (₢BQAAD) [complete]

**[260527-0921] complete**

## Character
Small, self-contained dedup.

`buc_tabtarget` re-implements the colophon-to-filename glob that `buyy_tt_yawp`
already owns. Make `buc_tabtarget` delegate to the yawp resolver so a single glob
exists in the kit.

Source: `buc_tabtarget` in `Tools/buk/buc_command.sh`; `buyy_tt_yawp` in
`Tools/buk/buym_yelp.sh`.

Done: one resolver; existing `buc_tabtarget` call sites render unchanged.

**[260521-1009] rough**

## Character
Small, self-contained dedup.

`buc_tabtarget` re-implements the colophon-to-filename glob that `buyy_tt_yawp`
already owns. Make `buc_tabtarget` delegate to the yawp resolver so a single glob
exists in the kit.

Source: `buc_tabtarget` in `Tools/buk/buc_command.sh`; `buyy_tt_yawp` in
`Tools/buk/buym_yelp.sh`.

Done: one resolver; existing `buc_tabtarget` call sites render unchanged.

### verify-terminal-awareness-both-registers (₢BQAAE) [complete]

**[260527-0931] complete**

## Character
Verification — proves the refactor is a no-op except the intended capability.

Confirm the merge changed nothing observable but terminal-awareness. Exercise a
sampling of operational output (e.g. a payor or regime command) and handbook
output (e.g. `tt/rbw-gPE.PayorEstablish.sh`) under three terminal modes: default,
`NO_COLOR=1`, and `TERM=dumb`. Color must suppress cleanly in the latter two, and
the gray operation sigil must never appear on handbook prose (the two-register
guarantee).

Done: all three modes verified across both registers; full `fast` suite green.

**[260521-1009] rough**

## Character
Verification — proves the refactor is a no-op except the intended capability.

Confirm the merge changed nothing observable but terminal-awareness. Exercise a
sampling of operational output (e.g. a payor or regime command) and handbook
output (e.g. `tt/rbw-gPE.PayorEstablish.sh`) under three terminal modes: default,
`NO_COLOR=1`, and `TERM=dumb`. Color must suppress cleanly in the latter two, and
the gray operation sigil must never appear on handbook prose (the two-register
guarantee).

Done: all three modes verified across both registers; full `fast` suite green.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 A extend-render-core-gray-and-strip
  2 B buc-shared-render-path-kindle-safe
  3 C reexpress-buc-public-on-core
  4 D collapse-tabtarget-glob-clone
  5 E verify-terminal-awareness-both-registers

ABCDE
·xxx· buc_command.sh
·xx·· buv_validation.sh, rbro_cli.sh
x··x· buym_yelp.sh
xx··· butcym_YelpModule.sh, butt_testbench.sh
··x·· bupr_PresentationRegime.sh
·x··· apcc_cli.sh, apcw_workbench.sh, buq_cli.sh, buq_qualify.sh, burc_cli.sh, bure_cli.sh, burs_cli.sh, buut_cli.sh, buw_workbench.sh, bux_cli.sh, cmw_workbench.sh, jjfp_cli.sh, jjw_workbench.sh, rbfc_cli.sh, rbfd_cli.sh, rbfh_cli.sh, rbfk_cli.sh, rbfl_cli.sh, rbfr_cli.sh, rbfv_cli.sh, rbga_cli.sh, rbgb_cli.sh, rbgg_cli.sh, rbgv_cli.sh, rbob_cli.sh, rbq_cli.sh, rbra_cli.sh, rbrd_cli.sh, rbrn_cli.sh, rbrp_cli.sh, rbrr_cli.sh, rbrs_cli.sh, rbrv_cli.sh, rbte_cli.sh, rbv_cli.sh, rbw_workbench.sh, vob_cli.sh, vow_workbench.sh, vslw_workbench.sh, vvb_cli.sh, vvw_workbench.sh

Commit swim lanes (x = commit affiliated with pace):

  1 A extend-render-core-gray-and-strip
  2 B buc-shared-render-path-kindle-safe
  3 C reexpress-buc-public-on-core
  4 D collapse-tabtarget-glob-clone
  5 E verify-terminal-awareness-both-registers

123456789abcdefghijklm
··········xxx·········  A  3c
·············xxxx·····  B  4c
·················xx···  C  2c
···················xx·  D  2c
·····················x  E  1c
```

## Steeplechase

### 2026-05-27 09:31 - ₢BQAAE - W

Verified the buc/buh harmonization is a no-op except the intended terminal-awareness, across both registers. Exercised operational output (rbw-rrv ValidateRepoRegime) and handbook output (rbw-gPE PayorEstablish) under three modes: default, NO_COLOR=1, TERM=dumb. Operational color present by default (11 ANSI lines, gray ZBUC_CONTEXT sigil on all 11) and fully suppressed under both NO_COLOR and TERM=dumb (0 lines each) — confirming the latent buc unconditional-palette bug is fixed. Handbook 121 ANSI codes by default collapse to 0 under both suppression modes, degrading cleanly (headers/spans/OSC-8 links become readable plain text). Two-register guarantee holds: gray [90m operation sigil never appears on handbook prose (0 across all three modes, including default). No diastema wire-format bytes (0x02) leak in any suppressed mode. Full fast suite green 112/112. Captured via direct stdout/stderr redirects to unique files after discovering last.txt is clobbered by concurrent officia and same-/hist- logs carry only the plain stripped transcript.

### 2026-05-27 09:21 - ₢BQAAD - W

Collapsed the duplicated colophon->tabtarget-filename glob to a single resolver. Extracted zbuym_tt_path (colophon [imprint] -> z_buym_tt_path, empty on no-match) in buym_yelp.sh as the kit's sole tabtarget glob, owning both the bare colophon.* form and the imprint-qualified colophon.*.imprint.sh form; added z_buym_tt_path to kindle mutable state. buyy_tt_yawp delegates to it, retaining only its ??colophon??/??colophon.imprint?? placeholder and diastema wrapping; buc_tabtarget delegates too, retaining its buc_die-on-no-match and byte-identical buc_bare output. The no-match policy (placeholder vs die) stays load-bearing in each caller. Closes the paddock's tabtarget-glob dedup line. bash -n clean both files; buw-st 36/36; fast suite 112/112 green.

### 2026-05-27 09:21 - ₢BQAAD - n

Collapse the duplicated colophon->tabtarget-filename glob to a single resolver. Extract zbuym_tt_path (colophon [imprint] -> z_buym_tt_path, empty on no-match) as the kit's sole tabtarget glob, owning both the bare colophon.* form and the imprint-qualified colophon.*.imprint.sh form. buyy_tt_yawp now delegates to it, keeping only its no-match placeholder (??colophon?? / ??colophon.imprint??) and diastema wrapping; buc_tabtarget delegates too, keeping its buc_die-on-no-match and buc_bare output (byte-identical). The no-match policy (placeholder vs die) stays in each caller — that distinction is load-bearing. z_buym_tt_path added to kindle mutable state alongside z_buym_yelp/z_buym_format. bash -n clean on both; buw-st 36/36 green.

### 2026-05-27 09:17 - ₢BQAAC - W

Reexpressed every public buc_* display function on the buym render core and retired buc's private BUC_* palette kit-wide. Deleted the source-time BUC_black..BUC_reset literals; color is now wholly the core's concern (buyf_format_yawp over the kindle-time BUYC_* palette), fixing the latent NO_COLOR/TERM=dumb bug across all buc output. New zbuc_tint <BUYC_name> <text> helper sentinel-kindles then resolves a named ambient by indirect expansion, keeping every BUYC_* dereference after kindle (the locked cold-buc_die set -u hazard); zbuc_print gained an ambient-name param (private, no external callers). Whole-line wrappers pass a BUYC name (step->BRIGHT_WHITE, code->CYAN, info/debug/trace->default); buc_warn/buc_die/buc_die_if/buc_die_unless render their colored prefix via buym span markers (warn-yawp orange WARNING:, fail-yawp bright-red ERROR:) with default body; sigil-less buc_success/buc_bare route color through zbuc_tint, fixing their always-on-color bug too; doc-mode helpers, zbuc_usage, buc_usage_die, buc_countdown, buc_require all migrated. Three external raw-BUC_* consumers migrated: buv_validation buv_render, rbro_cli render, bupr_PresentationRegime printf templates (blue->magenta, zbuym_sentinel on its two emitting functions). Precise bright/non-bright bytes shift where the core palette differs (operator: colors not critical). zbuc_hyperlink/buc_link left untouched — local literals, not the BUC_* palette. bash -n clean on all four files; buw-st 36/36 (both cold-die cases green); fast suite 112/112.

### 2026-05-27 09:15 - ₢BQAAC - n

Reexpress every public buc_* display function on the buym render core and retire buc's private BUC_* palette. The source-time BUC_black..BUC_reset literals are deleted; color is now wholly the core's concern (buyf_format_yawp over the kindle-time BUYC_* palette), which owns the single terminal-capability decision and honors NO_COLOR/TERM=dumb. New zbuc_tint <BUYC_name> <text> helper sentinel-kindles then resolves a named ambient via indirect expansion, keeping every BUYC_* dereference after kindle (the locked cold-buc_die set -u hazard). zbuc_print gains an ambient-name param (private, no external callers): whole-line wrappers pass a name (buc_step->BUYC_BRIGHT_WHITE, buc_code->BUYC_CYAN, buc_info/debug/trace->default). buc_warn/buc_die/buc_die_if/buc_die_unless render their colored prefix via buym span markers (buyy_warn_yawp orange WARNING:, buyy_fail_yawp bright-red ERROR:), body default. Sigil-less buc_success/buc_bare route color through zbuc_tint, fixing their latent always-on-color bug too; doc-mode helpers, zbuc_usage, buc_usage_die, buc_countdown, buc_require likewise migrated. Three external consumers of raw BUC_* migrated to BUYC_*/zbuc_tint: buv_validation buv_render, rbro_cli render, bupr_PresentationRegime printf templates (blue->magenta since precise colors are not critical; zbuym_sentinel added to its two emitting functions). Precise bright/non-bright byte values shift where the core palette differs (operator: colors not critical). bash -n clean on all four; buw-st 36/36 green including both cold-die cases.

### 2026-05-27 09:04 - ₢BQAAB - W

buc becomes a front-end on the buym render core. Two-part landing: (1) fan-out — every file that sources buc_command.sh now also sources buym_yelp.sh at its header (console substrate, not a furnish dependency — buc_warn/buc_die fire on doc-mode/unknown-command paths where furnish never runs); (2) flip — zbuc_print routes message bodies through buyf_format_yawp and renders the ZBUC_CONTEXT operation sigil via kindle-time BUYC_GRAY/BUYC_RESET instead of unconditional source-time BUC_gray, fixing the latent bug where buc ignored NO_COLOR/TERM=dumb (byte-confirmed: both now emit plain); zbuc_tag_args strips transcript args via buyf_strip_yawp (identity for current no-marker messages, transcripts byte-unchanged). Cold-path hazard guarded by zbuym_sentinel before any BUYC_* dereference; buc_die guarded transitively. Two cold-die self-test cases prove no unbound crash and sigil terminal-awareness (buym-yelp 13->15, buw-st 36/36, fast 112/112). The governing invariant 'sources buc_command => sources buym' is uniform except the abandoned rbhr_cli.sh; a mid-flight fast-fail (exit 127 in rbtdrf_ev_report_all_pass) corrected an initial gateway-vs-module misclassification, repairing buv_validation and buq_qualify. Deferred (later appetites): caller-baked body colors (BUC_red/BUC_white) remain unconditional, buc private-palette retirement, and the duplicated tabtarget-glob resolver.

### 2026-05-27 09:02 - ₢BQAAB - n

Close the buym fan-out invariant: buv_validation.sh and buq_qualify.sh source buc_command.sh at their own headers and were wrongly skipped as 'modules' during the fan-out, but they emit buc output and can root a process (the theurge enrollment-validation fixture sources buv_validation directly and calls buv_report). After the buc render path began hard-requiring buym, that path fast-failed with 'buyf_strip_yawp: command not found' (exit 127) in rbtdrf_ev_report_all_pass. Source buym_yelp.sh at each header alongside buc_command.sh, using the file's own ZBUV_/ZBUQ_SCRIPT_DIR. The governing invariant is now uniform: every file that sources buc_command.sh also sources buym_yelp.sh (the only exception is the abandoned rbhr_cli.sh). buw-st 36/36 green.

### 2026-05-27 08:58 - ₢BQAAB - n

Make buc a front-end on the buym render core. zbuc_print now routes each message body through buyf_format_yawp (inline yawp spans resolve) and renders the ZBUC_CONTEXT operation sigil via the kindle-time BUYC_GRAY/BUYC_RESET palette instead of buc's unconditional source-time BUC_gray — fixing the latent bug where buc output ignored NO_COLOR and TERM=dumb. zbuc_tag_args strips each transcript arg through buyf_strip_yawp so logs carry neither ANSI nor diastema bytes (identity for current no-marker messages, so transcripts stay byte-unchanged). Cold-path hazard guarded: zbuc_print calls zbuym_sentinel before any BUYC_* dereference, so a cold buc_die (buym sourced but unkindled) lazy-kindles rather than tripping set -u; buc_die needs no direct edit, guarded transitively through both calls. Two new cold-die self-test cases (color renders gray sigil from lazy kindle; plain suppresses it) prove no unbound crash and terminal-awareness; buym-yelp fixture 13->15, buw-st 36/36 green. Color-on output gains a trailing reset from the shared resolver (matches buh); NO_COLOR/TERM=dumb now plain.

### 2026-05-27 08:42 - ₢BQAAB - n

Fan out buym renderer into the console substrate: every gateway (CLI/workbench) that sources buc_command.sh now also sources buym_yelp.sh at the header, immediately alongside buc_command. This prepares buc to become a front-end on the buym render core (heat BQ) — buc output paths (buc_warn on unknown command, buc_die) fire on doc-mode/unknown-command paths where furnish never runs, so buym must be header-level console substrate, not a furnish-time dependency. Purely additive: buc does not yet consume buym, so behavior and output bytes are unchanged (buw-st 34/34 green). Modules (buv_validation, buq_qualify, rbob_bottle, rboo_observe) skipped — their consuming gateways carry the dependency.

### 2026-05-27 08:15 - ₢BQAAA - W

Extended buym render core additively: BUYC_GRAY ambient (gray in color mode, empty when off) and buyf_strip_yawp plain resolver (mode-independent, HREF/LINK degrade to 'text <url>', other markers vanish). Six new buym-yelp unit cases; fixture 7->13, self-test 34/34 green; existing format/buh output byte-unchanged. Also repaired the pre-existing SC2206 in buyy_tt_yawp surfaced by the BCG compliance pass, leaving buym_yelp.sh shellcheck-clean.

### 2026-05-27 08:09 - ₢BQAAA - n

Repair pre-existing SC2206 in buyy_tt_yawp: quote the literal/variable runs in the tabtarget-glob array builds, leaving only * exposed as a glob. Behavior-neutral (self-test 34/34); buym_yelp.sh now shellcheck-clean.

### 2026-05-27 08:04 - ₢BQAAA - n

Extend buym render core additively: BUYC_GRAY ambient (gray in color mode, empty when off) and buyf_strip_yawp plain resolver (mode-independent, HREF/LINK degrade to 'text <url>', other markers vanish). Six new buym-yelp unit cases (gray color/plain, strip cmd/link/href/fast-path); fixture 7->13, self-test 34/34 green. Existing format/buh output byte-unchanged.

### 2026-05-22 13:25 - Heat - f

silks=rbk-12-harmonize-buc-and-buh

### 2026-05-21 10:10 - Heat - f

silks=rbk-12-console-register-merge

### 2026-05-21 10:09 - Heat - S

verify-terminal-awareness-both-registers

### 2026-05-21 10:09 - Heat - S

collapse-tabtarget-glob-clone

### 2026-05-21 10:09 - Heat - S

reexpress-buc-public-on-core

### 2026-05-21 10:09 - Heat - S

buc-shared-render-path-kindle-safe

### 2026-05-21 10:09 - Heat - S

extend-render-core-gray-and-strip

### 2026-05-21 10:08 - Heat - d

paddock curried: initial shape at nomination

### 2026-05-21 10:06 - Heat - f

racing, silks=buk-12-console-register-merge

### 2026-05-21 10:01 - Heat - N

buk-console-register-merge

