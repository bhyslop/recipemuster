# OQ-5 — Native exit-code propagation through wsl.exe and other transports

## Hypothesis (original WSG framing)

WSG line 285–290: "When a command inside `wsl.exe ... bash -c "set -e; failing_cmd"` fails, does the non-zero exit propagate reliably back through wsl.exe → cmd.exe → ssh? Empirical evidence suggests YES (net.exe failures propagated correctly in cycle-3). The experiment matrix should confirm across nested-shell variants and across set-e vs explicit-exit forms."

## Resolution: YES, propagates reliably across all tested transports

## Probe matrix

All probes confirm `EXIT=N` matches the expected non-zero code from the body. Probe vehicle: `tt/buw-jpS.PrivilegedSsh.sh bujn-winpc <command>; echo "EXIT=$?"`.

### 5A — cmd.exe `exit 7` baseline

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'exit 7'; echo "EXIT=$?"
# EXIT=7
```

ssh propagates the cmd.exe ERRORLEVEL through to the curia.

### 5B — wsl.exe + bash `exit 7`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "exit 7"'; echo "EXIT=$?"
# EXIT=7
```

bash exit 7 → wsl.exe exit 7 → cmd.exe ERRORLEVEL 7 → ssh exit 7. Full chain.

### 5C — wsl.exe + bash with `set -e; false`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "set -e; false; echo UNREACHED"'; echo "EXIT=$?"
# EXIT=1
# (no UNREACHED in stdout)
```

`set -e` aborts the script on `false`'s exit 1. UNREACHED never prints. Curia sees exit 1.

### 5D — cygwin bash `exit 7`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'C:/cygwin64/bin/bash --login -c "exit 7"'; echo "EXIT=$?"
# EXIT=7
```

Cygwin propagates correctly.

### 5E — PowerShell `exit 5`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'powershell -NoProfile -Command "exit 5"'; echo "EXIT=$?"
# EXIT=5
```

PowerShell propagates explicit `exit N` correctly.

### 5F — Native Windows binary failure via wsl.exe interop

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "where.exe nonexistent_xyzpdq_zzz >/dev/null 2>\&1"'; echo "EXIT=$?"
# EXIT=1
```

`where.exe` returns 1 when the target is not found. The exit propagates: wsl.exe interop → bash → wsl.exe → cmd.exe → ssh → curia.

(This corroborates the empirical evidence cited in the original OQ-5 hypothesis from cycle-3 of correct-wsl-user-model: net.exe error 2224 propagated correctly. The probe vehicle differs but the chain is the same.)

## Promotion plan

Replace OQ-5 in WSG with a new **✅ rule**: "Exit-code propagation is reliable across cmd.exe, wsl.exe + bash, cygwin bash, and PowerShell transports. Native Windows binaries called via wsl.exe interop also propagate correctly. Bodies that need to fail-loud should use `set -e` (or explicit `exit N`); the failing exit will reach the curia unchanged."

Numbering suggestion: SH-N (next free number) for the bash/wsl/cygwin variant, perhaps a parallel PS-N for PowerShell, OR consolidate into one transport-agnostic rule referencing all variants.

## Caveats

- These probes are short-running synchronous commands. Long-running commands or commands that detach (start background services, daemonize) were not tested.
- `set -e` interaction with pipelines (`a | b` where `a` fails) was not separately probed; standard bash semantics apply (the pipeline's exit is `b`'s exit unless `pipefail` is set).
- PowerShell's `exit` from inside an `if` branch with `$null -eq $LASTEXITCODE` (the PS-1 trap) was not separately tested here; PS-1 covers that case.
