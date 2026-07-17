# Handoff — ₢BcAAO coronet re-gestalt, code phase (chat restart)

**Provisional handoff, not authority.** Hands off a partially-complete code phase
across a chat restart (context budget). The spec (JJS0 `jjdt_coronet` and the swept
JJS* sheaves) is the authority; this memo is a working map. **Retire when ₢BcAAO lands.**

- Pace: `₢BcAAO` (pace-identity-regestalt-impl), heat `₣Bc`.
- Branch: `bhyslop-260715-BcAAO-pace-identity-regestalt` (the §E schema-change
  quarantine — never pushed to main until the coda). Not yet pushed to origin since
  the WIP began; operator's call whether to push.

## Where we are — the story in commits

1. `3803f92af` — **spec infusion** (11 JJS* sheaves): the durable, truthful home.
   JJS0 `jjdt_coronet` rewritten (immutable-for-life flat global id, `jjdgm_pace_seed`
   global mint seed, retired per-heat `jjdhm_seed`, interpunct `·` heat-qualified
   display, `CAAAA` seed floor, the paces-scan **Resolution** rule), validate rules,
   encode/decode, JSSAS drained, draft/restring/slate/tally/nominate behavior.
2. `f2697a369` — **tack-schema alignment** (heat-affiliated ₣Bc, incidental): fixed a
   pre-existing `jjdkr_tack`/`jjdkm_*` → `jjdcr_tack`/`jjdcm_*` drift in 5 op sheaves.
3. `e74a17cfc`, `fd49efc98` — **source code**: schema + reprieve + mint + re-affiliate
   + lookup-by-scan. **Library source compiles green.**
4. `f9382392f` — **test WIP**: 12 test files reworked. **Does not compile yet.**

## Source is DONE and compiles (do not re-do)

- `jjrf_favor.rs`: `jjrf_Coronet` is a flat 5-char global index (`jjrf_encode(index)`,
  `jjrf_decode()->u32`, `jjrf_successor`; `jjrf_parent_firemark` **removed**). New consts
  `JJRF_CORONET_MAX`, `JJRF_CORONET_QUALIFIER = '·'`, `JJRF_CORONET_SEED_FLOOR = "CAAAA"`.
- `jjrt_types.rs`: retired `jjrg_Heat.next_pace_seed`; added `jjrg_Gallops.next_pace_seed`
  (`#[serde(default)]` so old stores parse).
- `jjri_io.rs`: reprieve episode `"pace-seed heat→global"` (rivet `JJr_a7c`), write-forward
  founds `max(highest+1, CAAAA)` via `zjjdz_found_pace_seed`.
- `jjro_ops.rs`: mint from the global seed; `jjrg_draft`/`jjrg_restring` move under the
  **same key** (no re-key, no per-heat seed), keeping the bridle→rough revert.
- `jjrg_gallops.rs`: new `jjrg_heat_key_of_coronet` paces-scan; `jjrg_resolve_pace` scans.
- `jjrv_validate.rs`: dropped embed-parent + per-heat-seed checks; added global-seed
  root check + cross-heat coronet uniqueness.
- All ~16 display/MCP `jjrf_parent_firemark` sites routed through the scan.

## REMAINING WORK — pick up here

1. **`jjtrs_restring.rs` — the one untouched test file.** Has old re-key behavior asserts:
   line ~61 (Heat `next_pace_seed` fixture → remove), lines ~429/449 (assert the dest heat
   seed advanced after restring — obsolete; restring now moves under the same key with **no**
   seed change and `old_coronet == new_coronet`). Rewrite to the move-under-same-key model.
   Also add `next_pace_seed` to any `jjrg_Gallops` literal it builds.
2. **`tt/vow-t` to green.** Compile first, then sweep runtime test failures. Expect the
   behavior tests that encoded old re-key/embed semantics to fail: notably the draft/relocate
   **revert** tests in `jjtg_gallops.rs` (`make_two_heat_gallops`, ~line 1593) and any assert
   that a moved pace got a *new* coronet — under the re-gestalt the coronet is unchanged.
   `§H`: notch before each vow-t run. `vow-t` compiles+tests but does **not** install the
   binary, so it is safe re: the hazard below.
3. **Step 4 — display/ingest (NOT started).** The scan sites currently render the plain
   firemark, not the heat-qualified `₢Bc·CAAAB` form. Introduce ONE qualified-display helper
   (`₢` + current-heat firemark + `·` + 5-char body) and route the listing/emblem surfaces
   through it; make halter/param ingest interpunct-aware (strip `·` + glyph, then type by
   length). Spec home: JJS0 `jjdt_coronet` "Display and ingest" + `jjdz_encoding` Input
   flexibility. Then tests for it.
4. **`tt/vow-b` LAST + wrap.** See hazard.

## Sequencing hazard (unchanged, still governs)

The running `vvx` MCP server holds the **old** binary in memory. `vow-b` installs a
**new-schema** binary to `Tools/vvk/bin/vvx`; a later jjx store-write on a new-schema binary
would persist the schema conversion — the §E-forbidden premature conversion. So: iterate with
`vow-t` (no install); run `vow-b` only at the very **end**; do **not** restart the session
between `vow-b` and the wrap (the in-memory old server keeps jjx bookkeeping on the old schema).
The on-disk gallops is still old-schema and must stay that way on this branch (the reprieve makes
the new binary tolerant; the conversion is deferred to the coda).

**This chat restart is safe:** no `vow-b` has run, so the server relaunches on the old binary; the
store stays old-schema; jjx keeps working.

## Fresh-chat startup

`jjx_open` → mount `₢BcAAO` (write `# jjezs_halter ₢BcAAO` to gazette_in, then `jjx_orient`).
Read the pace docket + this memo. First action: finish `jjtrs_restring`, then `vow-t`.
Do NOT re-touch the source (it compiles) or the spec (committed). CLAUDE.md §E/§F/JJSCRP
still govern the reprieve — do not improvise it (it is already registered).
