## Shape

Cutover of consumer-config + tabtarget path indirection per AAJ findings (₣A_/₢A_AAJ docket). Ten paces, hard cutover, branch-merge expected.

The load-bearing mechanism is `tt/z-launcher.sh` — a trampoline that resolves moorings location, normalizes cwd to repo root, and dispatches to a named launcher. Every other change in this heat orbits that mechanism: the filesystem rename flips its hardcoded path literal; constant pools become downstream consumers of trampoline-established execution context; vocabulary sweeps make the new layout visible to the human reader.

## Locked decisions

- **Hard cutover**, no bridge period. Branch-merge atomic.
- **`rbcc_Constants.sh` is the RBK-side fact locale.** Existing file, extended — not a new file.
- **`bubc_constants.sh` is the BUK-side fact locale.** Established 2026-05-10, ready.
- **Trampoline hardcodes the moorings path literal.** Bash constant pools (RBCC, BUBC) are downstream consumers of trampoline-established execution context, not its inputs.
- **z-launcher.sh chdirs to repo root** before dispatching to the named launcher. Workbenches start with deterministic cwd regardless of user invocation directory.
- **`rbm*_` prefix family**: `rbmm_` (umbrella), `rbml_` (launchers), `rbmu_` (users), `rbmv_` (vessels). `rbmn_` disposition is pace 1.
- **AAJ stays in ₣A_.** Nomination happened in agent conversation; AAJ's reslated form becomes "update A_ paddock with rbm*_ findings + verify ₣BK nominated + wrap."

## Discipline

- **Per-pace fast-suite smoke.** Each pace wrap runs `tt/rbtd-s.TestSuite.fast.sh`; must pass before notch. 75 cases, no GCP deps, ~2-3 minutes. Catches breakage at the pace that caused it.
- **`git mv` for file moves** so history follows.
- **Single notch-commit per pace** where the work permits (`jjx_record` handles staging).
- **Parallel-internal dispatch** where the docket calls for it (paces 5 and 8): spawn agents on disjoint file subtrees within the same pace.

## Out of scope

- Extending moorings to other kits (CMK / JJK / VVK / VOK).
- Eliminating per-nameplate `compose.yml` files (they're consumer-authored).
- Redesigning launcher infrastructure beyond introducing `z-launcher.sh`.
- Per-customer migration of consumer-side trees outside this repo.

## Sources

- ₣A_ / ₢A_AAJ docket — design rationale, renames table, BCG codification commitment
- `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md` — sections 21-30 (load-bearing complexity), 44-77 (CLI as module gateway), 171-181 (dispatch-provided BURD_*)
- `Tools/rbk/rbcc_Constants.sh` — RBK fact locale (extend, don't replace)
- `Tools/buk/bubc_constants.sh` — BUK fact locale (ready)