# Incident: jjx_record MCP call hung indefinitely after its commit landed

**Date:** 2026-07-26
**Reporter:** Claude Opus 4.8 (dispatched session, officium ☉260726-1051-uqs5)
**Context:** Mounted ₢Bz·CAABt (iptables-legacy-rationale-collapse) in heat
`rbk-23-win-acg-incorporate` (₣Bz). Full-ceremony mount. Ran `jjx_record` to
notch the single-file spec edit. The MCP call never returned.

---

## What happened

`jjx_record` (one `.adoc` file, `identity: CAABt`) was issued and never
returned to the MCP client. The harness moved it to a background task at 120s;
it continued reporting "still running" for 570s+ and had not returned when the
operator halted the investigation.

The underlying work **succeeded**: the git commit landed as HEAD
`790b34dab …₢CAABt:n: Collapse the duplicated iptables-legacy pin rationale …`.
The stall is entirely in the return path — the command did its job and then
hung before answering the client.

## Evidence gathered

- **Harness MCP log** (`~/Library/Caches/claude-cli-nodejs/-Users-bhyslop-projects-jjqb-200306-CAABt/mcp-logs-vvx/2026-07-27T04-07-56-875Z.jsonl`):
  the `jjx` (record) call began at `04:09:36.938Z` and logged
  `still running (30s … 570s elapsed)` continuously — the `vvx mcp` server held
  the call open the whole time, never delivering a response.
- **Commit landed** — verified via `git log` (HEAD `790b34dab`).
- **No worker/subprocess alive** — `ps` showed no `jjx_dispatch` worker and no
  `git` subprocess for this officium at the time of investigation. So the block
  is **in-process inside the `vvx mcp` server**, and **after** the git commit
  (no active git network call was hung *at that moment*).
- **Heartbeat touched once, at entry only** — the officium `heartbeat` file
  (`…/officia_scratch/260726-1051-uqs5/heartbeat`) carried mtime `21:09:37`
  (matching call entry `04:09:36Z`) and was never updated again. The heartbeat
  is written once per call at validate-entry (`jjrm_mcp.rs` ~line 2067), not
  per-phase, so it carries **no information about where a command stalls**.
- **High concurrency at the time** — ~10 concurrent dispatched officia were
  running (mount/groom sessions across many billets); `jjx_open` had reported
  `Exsanguination: 162 active, 0 reaped`. Contention on the shared commit
  lock / journal blotter is a plausible in-process wait, but **this was not
  confirmed** — see below.

## Unconfirmed hypothesis (do not treat as root cause)

Given "commit landed → in-process block → no live git child → heavy
concurrency," the most likely stuck-point is an **unbounded wait on the commit
lock / journal blotter** (the guidon git-ref lock and/or `jjrvb_blotter`
journal-mark path) with a derelict or contended holder — the failure the
`jjc-cashier` skill exists to clear. This is a hypothesis only; the logging
was insufficient to localize the hang, which is itself the primary finding.

## Primary finding — the logging is wrong (operator ruling)

All diagnostic output in the JJ MCP engine is `eprintln!` to stderr. Observed
sites (census, not exhaustive), `Tools/jjk/vov_veiled/src/jjrm_mcp.rs`:
lines 1276, 1289, 2598, 3101, 3920, 3927.

`eprintln!`-to-stderr is **flat wrong** for this engine (operator, this
session):

- **RCG** (`Tools/vok/vov_veiled/RCG-RustCodingGuide.md`) specifies error
  handling / diagnostics for Job Jockey in a different way — the repair must
  conform to RCG's prescribed mechanism rather than ad-hoc stderr prints.
- **JJS\*.adoc** (the Job Jockey spec sheaves, under `Tools/jjk/vov_veiled/`)
  also specify the error/diagnostic discipline. The repair chain must read the
  governing JJS sheaf and RCG first, then replace the `eprintln!` sites with
  the specified mechanism.

  *(Spec-home identification deferred to the repair session — the operator named
  RCG and JJS as the authorities; the exact rivets/sections were not resolved
  here.)*

Beyond the wrong sink, the diagnostics are **not phase-tagged, not timestamped,
and not landed to a durable per-command trace**. A long-running command that
hangs leaves no record of the phase it reached — the heartbeat is a single
entry touch, and stderr eprintln is ephemeral and unstructured. This is why the
hang above could not be localized from the officium files or any log.

## Suggested repair scope (for the separate repair chair)

1. Retire the `eprintln!` diagnostic sink in the JJ MCP engine; adopt the
   RCG-/JJS-specified error-handling + diagnostic mechanism.
2. Give long-running commands a **bounded** git-subprocess invocation
   (`zjjrfg_run_git` currently uses `Command::output()` with **no timeout** —
   `jjrfg_plaingit.rs` ~line 129), so a networked `fetch`/`ls-remote`/`push`
   cannot hang forever.
3. Add **per-phase progress** the operator can read while a command runs
   (periodic heartbeat re-touch and/or a durable per-command trace), so a hang
   is localizable without live `ps`/harness-log spelunking.

## Disposition

- Spec edit is committed and correct (HEAD `790b34dab`); the pace's work is
  done. **The pace was not wrapped** (the MCP layer is wedged; wrap deferred).
- The stalled MCP task is to be killed after this memo lands.
- Repair to be carried in a separate chat, operator-driven, against RCG + JJS.
