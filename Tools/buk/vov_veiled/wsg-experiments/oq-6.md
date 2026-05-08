# OQ-6 — Object-output flush semantics through cygwin bash and wsl.exe

## Hypothesis (original WSG framing)

WSG line 293–296: "PS-2 was proven for `powershell -Command` directly via cmd.exe. Same semantics through other transports (powershell launched from cygwin bash, powershell launched from inside WSL via wsl.exe interop) untested."

## Resolution: PS-2 is transport-agnostic

The lazy-formatter discard behavior is internal to PowerShell, independent of how `powershell.exe` is launched. The same `Get-LocalUser; exit 0` body produces empty stdout via all three transports tested.

## Probe matrix

### 6A — Direct cmd.exe → powershell, lazy form (PS-2 baseline)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'powershell -NoProfile -Command "Get-LocalUser -Name '\''Administrator'\''; exit 0"'; echo "EXIT=$?"
```

stdout: (single CRLF only)
exit: `EXIT=0`

Reproduces PS-2 — `Get-LocalUser`'s table is eaten by `exit 0`.

### 6B — cygwin bash → powershell.exe, lazy form

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'C:/cygwin64/bin/bash --login -c "powershell.exe -NoProfile -Command \"Get-LocalUser -Name '\''Administrator'\''; exit 0\""'; echo "EXIT=$?"
```

stdout: (single CRLF only)
exit: `EXIT=0`

Same behavior. Cygwin's transport does not change the PS internal formatter cycle.

### 6C — wsl.exe → bash → powershell.exe, lazy form

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "powershell.exe -NoProfile -Command \"Get-LocalUser -Name '\''Administrator'\''; exit 0\""'; echo "EXIT=$?"
```

stdout: (single CRLF only)
exit: `EXIT=0`

Same behavior. wsl.exe interop to powershell.exe also passes through the lazy-flush semantics.

### Side observation: PS-2 is NOT universal across all object cmdlets

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'powershell -NoProfile -Command "Get-Item C:/Users; exit 0"'; echo "EXIT=$?"
```

stdout:
```
    Directory: C:\

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d-r---          5/8/2026   8:33 AM                Users
```

`Get-Item` (FileSystem provider's formatter) survives `exit 0`. The lazy-flush behavior depends on the cmdlet's formatter pipeline, not just on whether the cmdlet emits objects vs strings. The existing PS-2 in WSG should be tightened to call out this nuance — the rule isn't "all object cmdlets are lazy," it's "some cmdlets (notably Microsoft.PowerShell.LocalAccounts's Get-LocalUser) have a formatter that flushes only on full pipeline completion; calling `exit` short-circuits it."

## Promotion plan

Replace OQ-6 in WSG with a brief note: "PS-2's lazy-flush behavior is transport-agnostic — same body emits empty stdout via direct cmd.exe→powershell, cygwin→powershell.exe, and wsl.exe→bash→powershell.exe. The fix (Out-String materialization, or rely on PS-1's $LASTEXITCODE init so the trailer doesn't fire) is also transport-agnostic, since it's a property of the PowerShell body, not the launching transport."

Optional: tighten PS-2 itself to clarify it is cmdlet-specific (Get-LocalUser proven; Get-Item disproven), not universal across all object output. Keeps the rule honest while preserving the discipline.

## Caveats

- Only `Get-LocalUser` (lazy) and `Get-Item` (eager) were probed across transports. The full universe of cmdlets and their formatter cycles is out of scope; the rule pattern (use Out-String when in doubt before exit) is the safe default.
- The Out-String fix probe through wsl.exe was attempted but failed at the cmd.exe layer because the pipe `|` survived only one level of quoting — cmd.exe interpreted it as an external pipe and broke the body. This is a separate failure mode about pipe-in-nested-quoted-bodies, not a property of the Out-String fix itself; the fix works inside any single PowerShell body that doesn't traverse cmd.exe's pipe interpretation.
