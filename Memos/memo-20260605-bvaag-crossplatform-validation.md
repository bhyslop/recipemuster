# ₢BVAAG cross-platform validation record

*2026-06-05. Heat ₣BV, pace ₢BVAAG — consolidate the three byte-identical
`/cygdrive→Windows` path normalizers into one BUK `buc_native_path_capture`.
Test record for the consolidation, captured before wrap.*

## What was validated

One canonical `buc_native_path_capture` in `Tools/buk/buc_command.sh` replacing
the three byte-identical copies `zrbfc_/zrbndb_/zrbob_native_path_capture`
(rbfcb_BuildHost.sh, rbndb_base.sh, rbob_bottle.sh), with all 13 call sites
repointed and the theurge `foundry-path` parity test re-homed onto the BUK
function. The `rbte_engine.sh` inline variant was deliberately left (different
raw-`OSTYPE`/`buc_die` contract).

Commit: `cb798526` on origin/main — a replay of the original notch `2d416129`
(orphaned by a concurrent `pull --rebase`; identical content). Fundus hosts ran
at origin/main tip `6e41a31`, which carries `cb798526`; darwin ran at the
original `2d416129`.

## Matrix (passed / failed / skipped)

| Platform | Commit | Suites | Result |
|----------|--------|--------|--------|
| darwin (curia) | `2d416129` | build, shellcheck, unit, fast | green — shellcheck 200 clean, 137 unit, fast 146/146 |
| Cygwin (cygwin@rocket) | `6e41a31` | fast, dogfight, blockade | 145/0/1, 6/0/0, 64/0/0 |
| WSL (wsl@rocket) | `6e41a31` | siege | 60/0/0 |
| cerebro (HEAD `34cf3695`) | `6e41a31` | siege | 19/1/0 |

## Coverage of the consolidated function's call paths

- **`np_*` parity cases** (fast) — the unit transform itself, on the live Cygwin
  `/cygdrive→X:/` branch (`BURD_OSTYPE=cygwin`) and the off-Cygwin identity
  branch (`linux-gnu`, darwin).
- **charge / `zrbob_compose`** (siege, blockade) — the bottle compose caller.
- **kludge build / rbfk** (siege) — the local-build caller.
- **Cloud Build foundry lifecycle / rbfd** (dogfight) — the director-build caller.

Both branches of the function were exercised green.

## The one near-pass

cerebro siege 19/20 — sole failure `rbtdrc_sentry_config_rp_filter`. **Not a
BVAAG regression**: charge and every BVAAG-touching path passed. The failure is a
pre-existing Interface Contamination in the writ value-read —
`rbtdrc_filter_writ_output` discriminates buc status lines by their ANSI gray
prefix, which is absent on a cold render (unset `TERM` over plain ssh), so a
status line is parsed as the value. Slated separately as ₢BVAAK
(writ-capture-color-decoupling).

Two cerebro-only environment issues surfaced and were handled: the writ-color
contamination above (slated), and a non-interactive-ssh cargo PATH gap (fixed;
see `memo-20260605-cerebro-noninteractive-cargo-path.md`).

## Conclusion

The path-normalizer consolidation is fully green on darwin, Cygwin, and WSL, and
green on cerebro across every path it touches. ₢BVAAG's Done bar — duplicate
normalizers removed; suites green on Cygwin and at least one non-Windows
platform — is met several times over. Scope note: the docket's "one canonical
Windows-docker adapter" framing was narrowed in conversation to the
normalizer consolidation alone; the rbgo headless-login bend and the
`rbtid/project/.gitkeep` mountpoint were left in place (₢BVAAH cleanup scope),
and no adapter module was built.
