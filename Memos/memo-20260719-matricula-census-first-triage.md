# Matricula census — first-batch triage (260719, pace census-first-triage)

First live census through the JJ seam: 470 presentments, 7204 estrays.
Two mechanical rulings landed this pace; the census now reads
178 presentments (88 collision + 90 terminal-exclusivity), 1870 estrays.

## Rulings applied (code, tested)

1. **Historical prose is reference-only scope.** `.claude/jjm/` (heat
   blotters, chat archives, gallops record) and `Study/` (scratch
   investigations) joined `Memos/` in `VOMA_REFERENCE_ONLY` — records *about*
   names, never declarations of them. Removed every self-quoting blotter
   "collision" and vendor-doc fragment site. Deliberately narrower than all
   of `.claude/`: `.claude/commands/` is a live minted namespace (Extended
   Namespace Checklist) and stays in the census.
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

## Second batch (same day, pace census-declaration-vs-inscription)

The declaration-site vs inscription-site distinction, landed as four
mechanical rulings; census now reads **92 presentments (2 collision + 90
terminal-exclusivity), 1913 estrays** (from 178/1876 after batch one;
470/7204 at first jog).

1. **Envelope dispatch.** A file is dressed by the vesture whose envelope
   claims it (VOS0 "Liturgy Domains"); only the dressed vesture's patterns
   claim there. `tt/` is tabtarget formulary (claims nothing — colophons,
   not prefix space); `.md` bodies are reference-only prose (stem claims
   still run, so slash-command names survive).
2. **Bash-const home-file rule.** `NAME=` claims a declaration only when the
   signet head and the file-stem head agree by bidirectional prefix,
   case-folded. Cross-family assignment is an inscription site; an unhomed
   ours wire name (`BURD_REGIME_FILE`, `BURC_OUTPUT_ROOT_DIR`) reads as
   estray — the honest verdict, and gentle pressure toward a spec home
   (BUS0 attribute declarations would give them one).
3. **`mod` lines are registrations.** The module's declaration is the file
   itself (stem claim); claiming `mod x;` twice-declared all 71 modules.
4. **Collision distinctness is per-file.** A module named for its primary
   act (stem + in-content declaration) is one mint through two claim
   mechanisms; 13 same-file "collisions" collapsed. Same-file double
   declaration defers to a future shadowing lint.

Surviving collisions, both real: `rblm_lustrate` (declared in
`rblm_cli.sh:257` AND as `rblm_lustrate.sh` — a genuine duplicate for
operator review) and `build` (cross-crate `build.rs` stems — the
prefix-coincidence residue class below). The 90 terminal-exclusivity
presentments are now the standing operator-question surface: cipher
self-trips, function-family prefixes (`buc_die`/`buc_die_if`), and
const-family prefixes (`RBCC_fact_ext_depot`).

## Known residue (ruled: deferred, visible)

- **False-ours by prefix coincidence**: `BUILD_ARGS` (reads as `bu…`) still
  seats and rides inside `BU`'s child list; the file-stem `build` (build.rs)
  collides across crates. A mechanical fix (require a separator after the
  cipher, or a minted-shape gate on stems) is possible but touches the
  cipher-match primitive — deferred until the pattern annoys.
- **Estray census (1870)** remains the residue-driven classifier queue by
  design; hyphenated release-artifact names (`vvx-darwin-arm64`) dominate.
