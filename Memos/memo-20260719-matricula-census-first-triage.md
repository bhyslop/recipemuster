# Matricula census — first-batch triage (260719, pace census-first-triage)

First live census through the JJ seam: 470 presentments, 7204 estrays.
Two mechanical rulings landed this pace; the census now reads
178 presentments (88 collision + 90 terminal-exclusivity), 1870 estrays.

## Rulings applied (code, tested)

1. **Historical prose is reference-only scope.** `.claude/` (heat blotters,
   chat captures) and `Study/` (scratch investigations) joined `Memos/` in
   `VOMA_REFERENCE_ONLY` — records *about* names, never declarations of them.
   Removed every self-quoting blotter "collision" and vendor-doc fragment site.
2. **The ours-cipher gate now bounds seating, not just estrays.** Line-claim
   seating passes `zvomrb_is_ours_token`, the same gate the estray net and the
   file-stem claim already used (VOSMM "Classify by Subtraction": the
   ours-or-foreign gate is the project cipher). Foreign declarations
   (`TOKEN=`, `ENV`, `val`, `to`, `ACQUIRED_AT`, `HALLMARK`) neither seat nor
   present, which also killed their string-prefix terminal-exclusivity noise
   (`ENV` → `ENVELOPE`).

## Surviving real corpus questions (banked, not fixed)

- **Cipher self-trips**: `BU`, `GAD`, `JJ`, `RB` are seated by their own
  `pub const` declarations in `vofc_registry.rs` and each "has children" —
  the whole kit tree. Question for the operator: is a cipher const a *name*
  (terminal-exclusivity breach by design) or is the registry the one exempt
  declaration site? Today it presents; a ruling either way is cheap.
- **Function-family terminal-exclusivity in live code**: e.g. `buc_die`
  seated *and* parent of `buc_die_if` / `buc_die_unless`; similar across
  `but_`/`buto_`/`rbfk_`/`rblm_` families. These are the census's genuine
  product — real prefix-discipline findings in the living corpus.
- **Doc-vs-code duplicate declaration**: e.g. `BURC_OUTPUT_ROOT_DIR` declared
  in `Tools/buk/README.md` (example lines matching the bash-const vesture)
  and in `bul_launcher.sh`. The declaration-bearing vs index-of-record vs
  reference-only *file-role* layer is deferred to the tackle-table projection
  (allowlist module header) — these presentments are true duplicates to the
  MVP's eye and go quiet once file roles land.

## Known residue (ruled: deferred, visible)

- **False-ours by prefix coincidence**: `BUILD_ARGS` (reads as `bu…`) still
  seats and rides inside `BU`'s child list; the file-stem `build` (build.rs)
  collides across crates. A mechanical fix (require a separator after the
  cipher, or a minted-shape gate on stems) is possible but touches the
  cipher-match primitive — deferred until the pattern annoys.
- **Estray census (1870)** remains the residue-driven classifier queue by
  design; hyphenated release-artifact names (`vvx-darwin-arm64`) dominate.
