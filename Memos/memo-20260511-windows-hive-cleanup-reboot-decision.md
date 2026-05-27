# Windows hive cleanup — reboot-as-canonical-primitive decision

Captured during pace ₢A-ABG (rbk-mvp-3-draft-windows-procedures /
windows-setup-first-time-debug) on 2026-05-11. This memo preserves the
design exploration that landed on **unconditional host reboot** as the
canonical Windows state-reset primitive for destructive cleanup, so
future maintainers can re-derive (or revise) the choice without
re-discovering.

## Symptom

On bujn-winpc, garrison-w's obliterate Phase 2 `Remove-CimInstance` on
`Win32_UserProfile` failed with:

> The process cannot access the file because it is being used by another
> process.
> HRESULT 0x80041017 (WBEM_E_INVALID_QUERY in the WQL probe;
> file-locked Win32 error in the destructive call).

Triage surfaced two stacked bugs:

1. **WQL backslash-escape bug.** The probe filter `LocalPath = 'C:\Users\<user>'`
   bombed at the WMI provider because WQL treats `\` as the literal-escape
   character in string literals; the path needs `\\` doubling. The
   bash-side `|| z_profiles_raw=""` fallback silently absorbed the
   provider error and downgraded "query failed" to "no rows," yielding
   *false-positive garrison-w success* while leaving six accumulating
   Win32_UserProfile rows (canonical + five `.rocket.NNN` demotion
   fallbacks).

2. **Hive-stuck race.** Even after fixing the WQL escape so the probe
   correctly returned the canonical row, `Remove-CimInstance` itself
   failed because the workload's NTUSER.DAT was still mounted under
   `HKU\<workload-SID>` (and `HKU\<workload-SID>_Classes`). The SSH-as-
   workload sessions garrison-w runs (`[w-session-1/4]` through
   `[w-session-4/4]`) cause UserProfileSvc to mount the hive on logon;
   on logoff, UserProfileSvc *eventually* unmounts but the race window
   between session end and unmount is large enough to hit destructive
   admin operations.

## Websearch — prior art

The phenomenon is well-known under the canonical observable **Event ID
1552** ("User hive is loaded by another process (Registry Lock)").
Microsoft historically shipped a separate tool **UPHClean** ("User
Profile Hive Cleanup Service") that monitored for logged-off users with
still-loaded hives and forcibly released holding handles; post-Vista,
UPHClean was integrated into UserProfileSvc with its own retry timer.
Industry-named contributing causes: antivirus scanners, Windows Fast
Startup, helper-process races (in our case WSL helpers `wslhost` /
`wslservice` / `vmwp` briefly holding handles to HKCU\Lxss subkeys).
Standard remedies in escalation order: wait/retry, manual
`reg unload HKU\<SID>`, `RUNDLL32 advapi32.dll,ProcessIdleTasks`,
reboot, safe-mode delete.

## Levels considered

A four-tier mitigation ladder mapped to escalating aggressiveness:

| Tier | Mechanism | Reliability | Cost |
|---|---|---|---|
| 1 | `reg unload HKU\<SID>_Classes` then `HKU\<SID>` before Remove-CimInstance, gated on `Loaded=True` probe | ~80% (fails when non-WSL holders — Defender, Search indexer, vmwp — hold the hive) | ~40 LOC + WQL fix |
| 2 | Tier 1 + enumerate open handles via P/Invoke to `NtQuerySystemInformation`/`SystemHandleInformation`, kill non-essential PIDs holding workload-related handles | ~98% (UserProfileSvc itself remains possible holder) | ~150 LOC + empirical filter tuning |
| 3 | Tier 2 + `Stop-Service profsvc -Force; Start-Service profsvc` | ~99.5% (collateral risk on shared hosts) | ~30 LOC additional |
| 4 | Reboot (`Restart-Computer -Force`) | **100%** as long as the host boots | ~30 LOC |

A separate "Level 3 redesign" was explored: admin pre-creates the
workload profile via `CreateProfile` P/Invoke (userenv.dll), `wsl --import`
runs as admin, the vhdx is moved to a workload-owned path, admin deletes
its own HKCU\Lxss entry via raw registry (not `wsl --unregister`, which
would delete the vhdx), and the Lxss subtree is offline-injected into
workload's NTUSER.DAT via `reg load` / `reg unload`. **This was rejected
as a race fix** because the first real workload SSH use (the entire
point of having a workload account) re-creates the hive mount and
re-opens the race window. Shifting *when* the first hive mount occurs
(from setup to first use) doesn't eliminate the race. The
architecture's *independent* value (smaller code, no SSH-as-workload
during setup) is captured in the admin-no-WSL paddock discussion (see
pace ₢A-ABJ as reframed).

## Decision: Tier 4 (reboot) as the primitive

Empirically validated on bujn-winpc: pre-reboot showed workload SID
`S-1-5-21-…-1029` + `_Classes` mounted under HKU with `Loaded=True`;
post-reboot (uptime delta 15h → 48s, ~14s SSH return; second-cycle
~42s) all hives unmounted, all six demoted Win32_UserProfile rows
cleanly removable. Garrison-w converges end-to-end with reboot as
prelude.

**Rationale for picking Tier 4 over Tier 1/2/3:**

- **Determinism.** Tiers 1-3 are probabilistic; Tier 4 is binary
  (it works or the host won't boot).
- **No undocumented internals.** Tiers 2-3 depend on
  `NtQuerySystemInformation` handle enumeration, UserProfileSvc
  internals, and empirical tuning of process-kill exclusion lists.
  Tier 4 depends only on `Restart-Computer -Force` and SSH
  reachability — both first-class, stable interfaces.
- **Code surface.** Tier 4 is the smallest (~30 LOC for the helper)
  despite being the most reliable. Tiers 1-3 grew code without
  matching reliability gains.
- **Diagnostic surface.** A failed reboot is a single, well-
  understood failure mode (Group Policy block, pending operations,
  hardware refusing to boot). A failed Tier 2 handle-hunt requires
  bespoke debugging.
- **Cost.** ~30-60s per garrison invocation in our deployment (modern
  hardware, NVMe SSD). Garrison-w is a multi-minute operation; the
  reboot cost is rounding error.

**The reboot helper's load-bearing preconditions** (all delivered by
caparison-windows; cross-referenced in BUSJCW):

- sshd is a Windows service starting at boot.
- Tailscale service `StartType = Automatic` (BURN_HOST resolves on
  Tailnet after reboot without operator action).
- `administrators_authorized_keys` + sshd_config Match block persist
  across reboot (they live in `C:\ProgramData\ssh`).

## Note: Windows Fast Startup is not what we observed — and we bypass it explicitly anyway

It's tempting to attribute the ~14-42s reboot times to Windows Fast
Startup, but **Fast Startup applies to `shutdown` (hybrid shutdown
saving the kernel session), not `restart`**. `Restart-Computer` /
`shutdown /r` does a full cold boot per documented PowerShell semantics.
The fast wall-clock times we saw are modern UEFI fast boot + NVMe SSD
performance, not OS-level fast startup. The polling-cap rationale
(`BUJB_reboot_poll_cap_s='600'`, 10 minutes) is dominated by the
pending-Windows-update worst case, not by normal boot variance.

**Belt-and-suspenders explicit cold restart.** The helper dispatches
`shutdown.exe /full /r /f /t 0` rather than `Restart-Computer -Force`.
The `/full` flag is the documented hatch to bypass Fast Startup
behavior; while redundant on `/r` (restart) under current Windows
semantics, it makes the intent visible in code and is robust to any
future MS default changes. We are not relying on "restart happens to
do a cold boot by default" — we are asking for a cold boot explicitly.

## Implementation pointers

- Helper: `zbujb_reboot_and_await_ssh` in
  `Tools/buk/bujb_jurisdiction.sh`.
- Wired unconditionally on `BURN_PLATFORM = bunne_windows` in
  `bujb_garrison()` between `bujp_preflight` and
  `zbujb_obliterate_workload`.
- Tinder constants: `BUJB_ssh_opt_connecttimeout_5`,
  `BUJB_reboot_poll_interval_s`, `BUJB_reboot_poll_cap_s`.
- Phase 2 obliterate simplified to WQL-escape + buc_die-on-probe-
  failure; no in-process unload remediation.
- Pace ₢A-ABG commits (`ff8d1e36`, `3fceebb3`, `086ab90d`,
  `08e26ec5`) carry the diff.

## When to revisit this decision

- If Windows ever ships a deterministic in-process hive-unload API,
  Tier 4 may stop being smallest.
- If we deploy to hosts where reboot is unacceptably invasive (shared
  workstations, long-running stateful Windows services) — currently
  out of scope per BUSJGW §Deferred.
- If pending Windows updates routinely extend reboot beyond 10
  minutes, we may need to raise `BUJB_reboot_poll_cap_s` or split the
  cap into "first-attempt cap" vs "update-install cap" with operator
  notification on the threshold cross.
