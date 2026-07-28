# Stirrup Environment Composition — Census (pace ₢CAACO)

Checkpointed first artifact for the stirrup-environment-composition pace. Enumerates
every environment variable the billet-launch dispatch chain actually exports today,
classifies each as **operator-shell** (ambient, must pass under the parity rule) or
**chain-export** (computed anew by the dispatch chain on each invocation, must be
stripped at the stirrup boundary), and records one empirical capture from inside a
live stirrup-spawned session as ground truth against the grep-derived table.

Source discipline: every row below traces to a grep hit or a read file; nothing is
guessed from a prefix's shape.

## 1. The chain, as found

Saddle and stirrup are not separate modules: one file,
`Tools/jjk/vov_veiled/src/jjrds_stile.rs`, implements "the stile's approach" —
identify, pedigree, billet ensure, glean, BURV export, provision, launch — ending in
the launch primitive `jjrds_stirrup_command` (line 1392). Saddle is a `jjrds_Door`
enum variant feeding the same approach, not a distinct env-composition step; there is
no separate "saddle report" yet built (the docket's phrase names planned work, not an
existing surface — grep for `BURE` in `Tools/jjk/vov_veiled/` returns zero hits).

Today's stirrup composes exactly:

```rust
// jjrds_stile.rs:1419-1421 — sets (the BURV trio)
cmd.env("BURV_OUTPUT_ROOT_DIR", scratch_root.join("output-buk"));
cmd.env("BURV_TEMP_ROOT_DIR", scratch_root.join("temp-buk"));
cmd.env("BURV_LOG_DIR", scratch_root.join("logs-buk"));

// jjrds_stile.rs:1427-1429 — strips (three explicit names)
cmd.env_remove("BURD_NO_LOG");
cmd.env_remove("BURD_INTERACTIVE");
cmd.env_remove("JJSL_INVOKE_DIR");
```

Tested at `Tools/jjk/vov_veiled/src/jjtds_stile.rs:294-316`
(`jjtds_stirrup_strips_the_doors_dispatch_modes`), which is the convention to extend.

## 2. Every export upstream of the stirrup

### BURD_* — the dispatch-computed family (chain-export, full stop)

`Tools/buk/bud_dispatch.sh:356` carries the fixed allowlist every `BURD_*` name must
belong to (`zbud_die "Unexpected BURD_ variables..."` otherwise) — this list IS the
chain-export census for the family, no name in it is operator-typed:

```
BURD_CONFIG_DIR BURD_MOORINGS_DIR BURD_REGIME_FILE BURD_NO_LOG BURD_INTERACTIVE
BURD_COORDINATOR_SCRIPT BURD_LAUNCHER BURD_STATION_FILE BURD_TERM_COLS
BURD_NOW_STAMP BURD_NOW_EPOCH BURD_TEMP_DIR BURD_OUTPUT_DIR BURD_PREVIOUS_DIR
BURD_TRANSCRIPT BURD_GIT_CONTEXT BURD_LOG_LAST BURD_LOG_SAME BURD_LOG_HIST
BURD_COMMAND BURD_TARGET BURD_CLI_ARGS BURD_TOKEN_1 BURD_TOKEN_2 BURD_TOKEN_3
BURD_TOKEN_4 BURD_TOKEN_5 BURD_TOOLS_DIR BURD_BUK_DIR BURD_TABTARGET_DIR
BURD_OSTYPE
```

29 names. Export sites: `tt/z-launcher.sh:58`; the door tabtargets (`BURD_LAUNCHER`,
`BURD_NO_LOG` on the two dispatch doors); `Tools/buk/bul_launcher.sh:45,53,56,64,162,164`;
`Tools/buk/bud_dispatch.sh:82,171-178,206,227-229`.
**Today's stirrup strips only 2 of these 29** (`BURD_NO_LOG`, `BURD_INTERACTIVE`) — the
other 27 currently leak into every launched billet session (confirmed empirically, §4).

### BURC_* — the regime-config family (chain-export; missed by the grep-for-"export" pass)

Not `export`ed by name anywhere — exported wholesale via
`buv_export_and_lock BURC` at `Tools/buk/burc_regime.sh:68`, which fires on every
kindle (i.e. every dispatch). Enrolled names, same file:

```
BURC_STATION_FILE BURC_TABTARGET_DIR BURC_TABTARGET_DELIMITER BURC_TOOLS_DIR
BURC_PROJECT_ROOT BURC_MANAGED_KITS BURC_TEMP_ROOT_DIR BURC_OUTPUT_ROOT_DIR
BURC_LOG_LAST BURC_LOG_EXT BURC_BUK_DIR
```

11 names, all committed-config values re-derived by the billet's own dispatch chain
on its first tabtarget run — none is stripped today; all 11 leak (§4).

### BURS_* — enrolled but never exported (not a concern)

`Tools/buk/burs_regime.sh` enrolls `BURS_USER`, `BURS_TINCTURE`, `BURS_LOG_DIR` and
locks them `readonly` (line ~50) but never calls an export-and-lock — confirmed no
`BURS_` names appear in the empirical capture (§4). Nothing to strip; they never
cross a spawn boundary as bash locals.

### BURE_* — operator-shell, with one exception

`Tools/buk/bure_regime.sh:22` — "not sourced from a file. Callers export BURE_*
variables before invoking" — this is the operator-ambient family the parity rule
means to preserve untouched. **One exception:** `BURE_COLOR` is presently computed,
not operator-typed — `Tools/buk/bud_dispatch.sh:280-297` (`zbud_resolve_color`)
overwrites it every dispatch (`export BURE_COLOR=0|1` per NO_COLOR/tty detection),
so under today's naming it reads as operator input but behaves as a chain-export.
This is exactly what the docket's BURD_COLOR re-filing fixes: once
`zbud_resolve_color` writes `BURD_COLOR` instead of `BURE_COLOR`, the verdict falls
under the ordinary BURD_* strip above with no stirrup-side special case, and
`BURE_COLOR` reverts to pure optional operator override
(`Tools/buk/bure_regime.sh:40`, `${BURE_COLOR:-auto}`), never written by dispatch.

### JJSL_INVOKE_DIR — ungoverned one-off (chain-export, stays stripped)

Written by the installed trampoline stamp at `Tools/jjk/jjsl_cli.sh:71`
(`JJSL_INVOKE_DIR="$PWD" exec ...`), read at lines 60-61 and 172, stripped today at
`jjrds_stile.rs:1429`. No `BURD_`/`BURE_` prefix, no regime enrollment anywhere —
confirmed by repo-wide grep. Per the paddock's cinch this stays a documented
strip-only one-off; formalizing it into a regime is explicitly declined pending the
launch-inversion rung.

### BURV_* — the payload, not a strip target

`BURV_OUTPUT_ROOT_DIR`, `BURV_TEMP_ROOT_DIR`, `BURV_LOG_DIR` — set fresh by the
stirrup itself (`jjrds_stile.rs:1419-1421`); these are the one family that must
*survive* into the billet session.

### rbtdri_invocation.rs — cited, not touched (RBK's own kit, separate strip)

`Tools/rbk/rbtd/src/rbtdri_invocation.rs:562-564` strips exactly `BURD_NO_LOG` and
`BURD_INTERACTIVE` for theurge's own child-tabtarget spawns, citing the same
`BUr_q2m` doctrine. Independent file, independent (narrower, correctly-scoped-for-its-
case) strip list; this pace's scope is the JJK stirrup alone.

## 3. Vendor ambient — out of scope by cinch

`PATH`, `HOME`, `TERM`, `CLAUDE_*`, `ANTHROPIC_*`, and kin are never touched by this
census or the code that follows it — allowlisting vendor-ambient territory is
Palisade-fragile per the paddock's cinch.

## 4. Empirical capture — live, from inside this pace's own stirrup-spawned session

Captured via `env | grep -E '^(BURD_|BURE_|BURV_|BURC_|BURS_|JJSL_)' | sort` from
inside the actual Claude Code session mounted for this pace (spawned by
`tt/jjw-ds.Saddle.sh` against the recipemuster sire):

```
BURC_BUK_DIR=Tools/buk
BURC_LOG_EXT=txt
BURC_LOG_LAST=last
BURC_MANAGED_KITS=buk,cmk,jjk,vvk
BURC_OUTPUT_ROOT_DIR=../output-buk
BURC_PROJECT_ROOT=..
BURC_STATION_FILE=../station-files/burs.env
BURC_TABTARGET_DELIMITER=.
BURC_TABTARGET_DIR=tt
BURC_TEMP_ROOT_DIR=../temp-buk
BURC_TOOLS_DIR=Tools
BURD_BUK_DIR=Tools/buk
BURD_COMMAND=jjw-ds
BURD_CONFIG_DIR=/Users/bhyslop/projects/rbm_alpha_recipemuster/rbmm_moorings
BURD_COORDINATOR_SCRIPT=Tools/jjk/jjw_workbench.sh
BURD_GIT_CONTEXT=pre-coda-BcAAD-main-723-g8073a484f
BURD_LAUNCHER=launcher.jjw_workbench.sh
BURD_MOORINGS_DIR=rbmm_moorings
BURD_NO_LOG=1
BURD_NOW_EPOCH=1785264403
BURD_NOW_STAMP=20260728-114643-58058-846
BURD_OSTYPE=darwin25
BURD_OUTPUT_DIR=.../output-buk/current
BURD_PREVIOUS_DIR=.../output-buk/previous
BURD_REGIME_FILE=.../rbmm_moorings/burc.env
BURD_STATION_FILE=.../station-files/burs.env
BURD_TABTARGET_DIR=tt
BURD_TARGET=jjw-ds.Saddle.sh
BURD_TEMP_DIR=.../temp-buk/temp-20260728-114643-58058-846
BURD_TERM_COLS=143
BURD_TOKEN_1=jjw-ds
BURD_TOKEN_2=Saddle
BURD_TOKEN_3=sh
BURD_TOOLS_DIR=Tools
BURD_TRANSCRIPT=.../temp-20260728-114643-58058-846/transcript.txt
BURE_COLOR=1
BURV_LOG_DIR=/Users/bhyslop/projects/jjqd_scratch/jjqb_200468_CAACO/logs-buk
BURV_OUTPUT_ROOT_DIR=/Users/bhyslop/projects/jjqd_scratch/jjqb_200468_CAACO/output-buk
BURV_TEMP_ROOT_DIR=/Users/bhyslop/projects/jjqd_scratch/jjqb_200468_CAACO/temp-buk
JJSL_INVOKE_DIR=/Users/bhyslop/projects/rbm_alpha_recipemuster
```
(paths abbreviated with `...` for readability; full paths were absolute)

**Finding:** every BURC_* name (11/11) and every BURD_* name including the two the
source claims to strip (`BURD_NO_LOG`, `JJSL_INVOKE_DIR` present despite
`env_remove` calls citing them at `jjrds_stile.rs:1427,1429`) are live in this
session. The discrepancy between source and observed behavior is recorded as-is
without an asserted root cause (candidates: the installed `vvx` binary predates this
source revision; a build/install step is pending) — it does not change the fix this
pace owes, and the fix's own unit test (composing the command and inspecting
`cmd.get_envs()`) is process-boundary-exact regardless of what binary is currently
installed. Re-running this capture after the fix lands and the binary rebuilds is
the acceptance proof.

## 5. Classification summary — the strip list this pace builds

| Family | Count | Verdict | Disposition |
|---|---|---|---|
| `BURD_*` | 29 (full allowlist, `bud_dispatch.sh:356`) | chain-export | strip all, by prefix — not 2 of 29 |
| `BURC_*` | 11 (`burc_regime.sh` enrollment) | chain-export | strip all, by prefix |
| `BURS_*` | 3 | never exported | no action — confirmed absent |
| `BURE_*` (general) | operator-set, unbounded | operator-shell | pass through untouched |
| `BURE_COLOR` | 1 | chain-export masquerading as operator-shell | re-file: `bud_dispatch.sh` writes `BURD_COLOR`, not `BURE_COLOR`; then falls under the `BURD_*` strip above with no special case |
| `JJSL_INVOKE_DIR` | 1 | chain-export (ungoverned) | strip explicitly (no prefix to key off) |
| `BURV_*` | 3 | the payload | compose fresh at the stirrup; must survive |
| vendor ambient | — | operator-shell | untouched, out of scope |

Mechanism this drives: rather than three named `env_remove` calls, the stirrup
strips by **prefix match** (`BURD_`, `BURC_`) plus the one **named exception**
(`JJSL_INVOKE_DIR`) — covering the family as it stands today and as it grows,
without the strip list drifting out of sync with the allowlists it derives from.
