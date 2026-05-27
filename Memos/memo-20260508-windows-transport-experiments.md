# Windows Transport Experiments — OQ-1 through OQ-7

Date: 2026-05-08

## Context

WSG (`Tools/buk/vov_veiled/WSG-WindowsScriptingGuide.md`) was first
populated from diagnostic cycles in the predecessor pace (`₢A-AAv`,
correct-wsl-user-model). That work landed eight established rules but
flagged seven open questions (OQ-1 through OQ-7) for empirical
resolution. This memo records the probe matrix run against `bujn-winpc`
in pace `₢A-AA0` (windows-transport-experiments) that resolved them.

Probe vehicle: `tt/buw-jpS.PrivilegedSsh.sh bujn-winpc <command>`. Curia
single-quoted argv to the tabtarget so `$` reaches it verbatim; the
tabtarget passes argv to ssh, ssh forwards a single command string to
remote sshd, sshd hands it to cmd.exe (DefaultShell). From there, the
chain branches by which child shell is invoked: `wsl.exe ... bash` (w),
`C:/cygwin64/bin/bash --login` (c), or `powershell -NoProfile -Command`
(direct PS).

Fundus: `bujn-winpc` (BURN_HOST=rocket, BURN_PLATFORM=bunne_windows).
WSL distribution: `rbtww-main` (admin: `--user root`). Cygwin path:
`C:/cygwin64/bin/bash --login`.

Findings are summarized at the end of this memo (Summary). The body is
the per-OQ probe matrix.

## OQ-1 — Which layer eats `$name` in wsl.exe transit?

### Hypothesis (original WSG framing)

WSG framed OQ-1 as "what eats `$()` and `$var` in `wsl.exe ... bash -c
"<body>"`?" with cmd.exe ruled out (SH-4) and the candidates listed as
wsl.exe argv parser, parens-in-quotes interaction, or OpenSSH-Win32
spawn side-effect.

The matrix shows the original framing was half-right: `$(...)` is NOT
eaten — it survives to bash unchanged. Only `$name` and `${name}`
references are mangled. The eater is wsl.exe's argv parser (or its
argv-to-Linux-invocation transform), and the mechanism is
environment-variable substitution against wsl.exe's own environment,
not nullification.

### Probe matrix

#### 1A — cmd.exe alone, baseline (re-verify SH-4)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'echo "VAR=$X SUB=$(uname)"'
# stdout: "VAR=$X SUB=$(uname)"
```

cmd.exe emits the body verbatim (including the surrounding `"` —
cmd.exe's `echo` is wholly verbatim). Neither `$X` nor `$(uname)` is
touched. Re-confirms SH-4 on this fundus.

#### 1B — wsl.exe + bash, single statement, mixed `$X` and `$(uname)`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "echo VAR=$X SUB=$(uname)"'
# stdout: VAR= SUB=Linux
```

`$(uname)` survived to bash (bash ran `uname` → `Linux`). `$X` is empty.

#### 1C — prior cycle's symptom: `ztmp=$(mktemp); echo $ztmp`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=$(mktemp); echo TMPVAL=$ztmp"'
# stdout: TMPVAL=
```

Reproduces the symptom that motivated OQ-1.

#### 1D — replace `$(mktemp)` with `$(echo HELLO)` (rule out missing-binary)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=$(echo HELLO); echo TMPVAL=$ztmp"'
# stdout: TMPVAL=
```

Same empty result. Rules out "wsl.exe ran `mktemp` Windows-side and got
nothing." Something's substituting `$ztmp` to empty before bash sees the
body.

#### 1E — `$()` alone, single statement, no `;`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "echo SUB=$(echo HELLO)"'
# stdout: SUB=HELLO
```

`$(...)` worked when not paired with later `$name` reference. First
crack in the original "`$()` is eaten" hypothesis: it is not.

#### 1F — `;` separator, no `$` references

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "echo A; echo B"'
# stdout:
# A
# B
```

`;` works fine. Both statements ran.

#### 1G — `;` + literal assignment + `$name` reference (the canonical failure)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo TMPVAL=$ztmp"'
# stdout: TMPVAL=
```

No `$()` involved at all. Plain assignment + bare `$ztmp` reference.
Empty result. **The eater operates on `$name`, not `$()`.**

#### 1H — same body via cygwin bash (also answers OQ-2)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'C:/cygwin64/bin/bash --login -c "ztmp=HELLO; echo TMPVAL=$ztmp"'
# stdout: TMPVAL=HELLO
```

**Cygwin works.** The differentiating factor is wsl.exe — cygwin's path
(cmd.exe → C:/cygwin64/bin/bash.exe) does not exhibit the substitution.
The eater is wsl.exe specifically.

#### 1I — does `${braces}` form escape the substitution?

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo TMPVAL=${ztmp}"'
# stdout: TMPVAL=
```

Braces don't help. wsl.exe substitutes `${name}` form too.

#### 1J — `\$name` escape form

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo TMPVAL=\$ztmp"'
# stdout: TMPVAL=HELLO
```

**`\$name` escape works.** The `\` survives to wsl.exe; wsl.exe treats
`\$` as a literal `$` (not a substitution trigger); bash sees `$ztmp`
literally and does its own expansion against the current shell's
environment where `ztmp` was just assigned. Round-trip succeeds.

#### 1K — `$USERNAME` (Windows env var)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "echo USERNAME_TEST=$USERNAME"'
# stdout: USERNAME_TEST=
```

#### 1L — confirm USERNAME exists in Windows (cmd.exe %VAR%)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'echo USERNAME_TEST=%USERNAME%'
# stdout: USERNAME_TEST=bhyslop
```

USERNAME exists in the Windows ssh session's environment. wsl.exe did
NOT use it for substitution. **wsl.exe's substitution does not pull
from Windows env.**

#### 1M — `$PATH` (set in wsl.exe Linux root env)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "echo PATH_TEST=$PATH"'
# stdout: PATH_TEST=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:...
```

`$PATH` got substituted to the Linux PATH. **The substitution scope is
the wsl.exe Linux-side environment.** Names defined there → value;
undefined → empty.

(Proof that the substitution happens upstream of bash, not by bash: 1G
sets `ztmp=HELLO` mid-body and then `echo $ztmp` returns empty in the
same bash process; only `\$` defers the lookup to bash, which then sees
its own assignment.)

#### 1N — `;` + assignment, no `$` at all

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo TMPVAL=ZZZ"'
# stdout: TMPVAL=ZZZ
```

Confirms `;` itself is not the culprit; only `;` + `$varname` (where
the name is undefined in wsl.exe's startup env) together.

#### 1O — `;` + `$PATH`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "echo A; echo X=$PATH"'
# stdout:
# A
# X=/usr/local/sbin:/usr/local/bin:...
```

`;` + `$PATH` works because PATH IS in wsl.exe's startup env.
Substitution succeeds with the right value.

#### 1P — full canonical escape: `\$(mktemp)` and `\$ztmp`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=\$(mktemp); echo TMPVAL=\$ztmp"'
# stdout: TMPVAL=/tmp/tmp.hNpO0X8BXI
```

Canonical pattern. Both `\$(mktemp)` and `\$ztmp` reach bash literally;
bash performs its own expansion in its own scope. (Probe leaves a temp
file on the WSL distro; reaped by tmpwatch.)

#### 1Q — `\${braces}` escape form

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo TMPVAL=\${ztmp}"'
# stdout: TMPVAL=HELLO
```

`\${name}` also works.

### Resolution

**Mechanism**: wsl.exe's argv-to-Linux-invocation processor performs
`$name` and `${name}` substitution against its own (Linux-side,
root-shell startup) environment before constructing the bash invocation.
Names present in that environment substitute to their values; undefined
names substitute to empty.

**Not affected**:
- cmd.exe (1A; matches existing SH-4)
- Cygwin bash (1H; this resolves OQ-2)
- `$(...)` command substitution (1E; the `(` after `$` does not match
  the `$name` / `${name}` token shape wsl.exe is looking for)

**Escape rule**: `\$name` and `\${name}` reach bash with literal `$`.

### Caveats and untested edges

- Substitution semantics for `$$` (PID), `$?`, `$1`, `$@`, `$*` not
  explicitly tested. Recommend the same `\$` escape as the safety
  default.
- Whether wsl.exe substitutes against `WSLENV`-promoted vars vs the
  Linux startup env exclusively was not characterized. The rule is
  the same either way: escape `\$` defers to bash.
- Empirical for wsl 2 on Windows 10/11 as installed on rocket. The
  underlying behavior is plausibly a documented wsl.exe argv-quoting
  feature; the empirical behavior is what callers must work against.

## OQ-2 — Does cygwin bash exhibit the same `$`-eating?

Resolved by probe 1H above: cygwin bash does NOT pre-substitute
`$name`. The same body that returns empty through wsl.exe
(`ztmp=HELLO; echo TMPVAL=$ztmp`) returns `TMPVAL=HELLO` through
cygwin.

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'C:/cygwin64/bin/bash --login -c "ztmp=HELLO; echo TMPVAL=$ztmp"'
# stdout: TMPVAL=HELLO
```

Consistent with cygwin's argv path: cmd.exe → Windows argv parser →
C:/cygwin64/bin/bash.exe. There is no wsl.exe intermediate to
substitute env vars in argv. Bash receives the body as-passed (modulo
cmd.exe and Windows argv-parser conventions, which do not include
`$name` substitution per SH-4).

### Caveats

- This was tested with `--login`. Whether cygwin bash without `--login`
  behaves the same was not separately verified; the cygwin transport in
  `bujb_jurisdiction.sh` uses `--login` so the rule covers production
  usage.

## OQ-3 — Linux/Mac native ssh body semantics

### Resolution: deferred (no Linux/Mac fundus in matrix)

The only registered BURN profile in this regime is `bujn-winpc`
(`BURN_PLATFORM=bunne_windows`). There is no Linux or Mac investiture
available for the probe vehicle.

Rationale for deferral being acceptable:

- The b-letter ssh path on a Linux/Mac fundus bypasses every
  Windows-specific layer that produces the quirks in OQ-1, OQ-2, OQ-5,
  OQ-6: no cmd.exe DefaultShell, no Windows argv parser, no wsl.exe
  argv substitution. The remote command goes ssh → remote sshd → user
  shell (bash) directly.
- The body-as-arg shape (SH-1) is the canonical one, untouched by the
  Windows-only failure modes.
- The `$` escape rule from OQ-1/OQ-4 is wsl.exe-specific by mechanism.
  Linux bash via direct ssh has the bash quoting model exclusively.

When a Linux or Mac fundus is added, run probes 1A, 1B, 1G, 1J, 1P from
OQ-1 (substituting `bash -c "..."` directly, no wsl.exe / cygwin
wrapper). Expectation: every probe behaves as plain Linux bash quoting
predicts; no `\$` escape needed for body-side variables.

## OQ-4 — Canonical escape rule for body-side `$var` and `$(...)`

### Resolution

Drops out of OQ-1. The escape rule is per-letter:

| Letter | Path                                       | `$name` escape | `${name}` escape | `$(...)` escape |
|--------|--------------------------------------------|----------------|------------------|-----------------|
| b      | direct ssh to Linux/Mac (deferred — OQ-3)  | (deferred)     | (deferred)       | (deferred)      |
| c      | cmd.exe → cygwin bash                      | none needed    | none needed      | none needed     |
| w      | cmd.exe → wsl.exe → bash                   | `\$name`       | `\${name}`       | none needed     |

Probe pair (verification):

```
# ❌ Fails on w-letter — wsl.exe substitutes $ztmp to empty before bash runs
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo $ztmp"'
# stdout: (empty line)

# ✅ Works on w-letter — \$ defers expansion to bash
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo \$ztmp"'
# stdout: HELLO
```

### Caller contract notes

`zbujb_admin_exec` (`Tools/buk/bujb_jurisdiction.sh:306-335`) ships the
body literally — caller writes the body string, the function joins
multi-arg form with `;` and does `"` → `\"` escaping. **It does not
transform `$`.** Therefore: callers calling `zbujb_admin_exec w` must
write `\$varname` and `\${varname}` for body-side expansion; callers
calling `zbujb_admin_exec c` may use unescaped `$varname`.

The caller contract becomes asymmetric across letters. This is
empirically required (wsl.exe vs cygwin behave differently), not a
discipline choice. Workarounds:

1. **Status quo (asymmetric)**: caller knows the letter, caller escapes
   per-letter. Current bujb_jurisdiction.sh callers already do this.
2. **Symmetric (uniform)**: have `zbujb_admin_exec` apply `$` → `\$`
   substitution for w-letter only. Hidden cost: callers writing `\$`
   for compatibility would get `\\$` after the transform; not robust
   against caller-already-escaped bodies.
3. **Symmetric via temp-file (OQ-7)**: ship body as a remote temp file,
   then `bash <file>`. Bypasses wsl.exe's argv substitution entirely.

For release-1 the **status quo** is the rule of record.

## OQ-5 — Native exit-code propagation

### Resolution: YES, propagates reliably across all tested transports

Probe vehicle: `tt/buw-jpS.PrivilegedSsh.sh bujn-winpc <command>; echo "EXIT=$?"`.

#### 5A — cmd.exe `exit 7` baseline

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'exit 7'; echo "EXIT=$?"
# EXIT=7
```

#### 5B — wsl.exe + bash `exit 7`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "exit 7"'; echo "EXIT=$?"
# EXIT=7
```

#### 5C — wsl.exe + bash with `set -e; false`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "set -e; false; echo UNREACHED"'; echo "EXIT=$?"
# EXIT=1
# (no UNREACHED in stdout)
```

#### 5D — cygwin bash `exit 7`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'C:/cygwin64/bin/bash --login -c "exit 7"'; echo "EXIT=$?"
# EXIT=7
```

#### 5E — PowerShell `exit 5`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'powershell -NoProfile -Command "exit 5"'; echo "EXIT=$?"
# EXIT=5
```

#### 5F — Native Windows binary failure via wsl.exe interop

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "where.exe nonexistent_xyzpdq_zzz >/dev/null 2>\&1"'; echo "EXIT=$?"
# EXIT=1
```

`where.exe` returns 1 when target not found. Exit propagates: wsl.exe
interop → bash → wsl.exe → cmd.exe → ssh → curia. Corroborates
empirical evidence cited in original OQ-5 hypothesis from cycle-3 of
correct-wsl-user-model: net.exe error 2224 propagated correctly.

### Caveats

- These probes are short-running synchronous commands. Long-running
  commands or commands that detach (start background services,
  daemonize) were not tested.
- `set -e` interaction with pipelines (`a | b` where `a` fails) was not
  separately probed; standard bash semantics apply (the pipeline's
  exit is `b`'s exit unless `pipefail` is set).
- PowerShell's `exit` from inside an `if` branch with `$null -eq
  $LASTEXITCODE` (the PS-1 trap) was not separately tested here; PS-1
  covers that case.

## OQ-6 — Object-output flush semantics through cygwin and wsl.exe

### Resolution: PS-2 is transport-agnostic

The lazy-formatter discard behavior is internal to PowerShell,
independent of how `powershell.exe` is launched. The same `Get-LocalUser;
exit 0` body produces empty stdout via all three transports tested.

#### 6A — Direct cmd.exe → powershell, lazy form (PS-2 baseline)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'powershell -NoProfile -Command "Get-LocalUser -Name '\''Administrator'\''; exit 0"'; echo "EXIT=$?"
# stdout: (single CRLF only)
# EXIT=0
```

Reproduces PS-2 — `Get-LocalUser`'s table is eaten by `exit 0`.

#### 6B — cygwin bash → powershell.exe, lazy form

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'C:/cygwin64/bin/bash --login -c "powershell.exe -NoProfile -Command \"Get-LocalUser -Name '\''Administrator'\''; exit 0\""'; echo "EXIT=$?"
# stdout: (single CRLF only)
# EXIT=0
```

#### 6C — wsl.exe → bash → powershell.exe, lazy form

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "powershell.exe -NoProfile -Command \"Get-LocalUser -Name '\''Administrator'\''; exit 0\""'; echo "EXIT=$?"
# stdout: (single CRLF only)
# EXIT=0
```

### Side observation: PS-2 is NOT universal across all object cmdlets

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'powershell -NoProfile -Command "Get-Item C:/Users; exit 0"'; echo "EXIT=$?"
# stdout:
#     Directory: C:\
#
# Mode                 LastWriteTime         Length Name
# ----                 -------------         ------ ----
# d-r---          5/8/2026   8:33 AM                Users
```

`Get-Item` (FileSystem provider's formatter) survives `exit 0`. The
lazy-flush behavior depends on the cmdlet's formatter pipeline, not
just on whether the cmdlet emits objects vs strings. This nuance was
folded into the WSG PS-2 rule.

### Caveats

- Only `Get-LocalUser` (lazy) and `Get-Item` (eager) were probed across
  transports. The full universe of cmdlets and their formatter cycles
  is out of scope; the rule pattern (use Out-String when in doubt
  before exit) is the safe default.
- The Out-String fix probe through wsl.exe was attempted but failed at
  the cmd.exe layer because the pipe `|` survived only one level of
  quoting — cmd.exe interpreted it as an external pipe and broke the
  body. This is a separate failure mode about pipe-in-nested-quoted-bodies,
  not a property of the Out-String fix itself; the fix works inside any
  single PowerShell body that doesn't traverse cmd.exe's pipe
  interpretation.

## OQ-7 — Multi-line bodies via temp-file feed

### Resolution: file feed works, with normalization caveat

Two-phase ship-and-run cleanly avoids both cmd.exe newline fragility
and wsl.exe argv `$`-substitution. Recommended only for bodies that
materially exceed `;`-join's reach; SH-2 remains the default.

#### 7A — Single-line `;`-joined body via file feed (sanity baseline)

Phase 1 (write):
```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'powershell -NoProfile -Command "Set-Content -Path C:\Users\bhyslop\probe.sh -Value '\''echo LINE1; echo LINE2; ztmp=HELLO; echo VAL=$ztmp'\''"'
# EXIT_WRITE=0
```

Phase 2 (run):
```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash /mnt/c/Users/bhyslop/probe.sh'
# stdout:
# LINE1
# LINE2
# VAL=HELLO
# EXIT_RUN=0
```

`$ztmp` evaluated as `HELLO` — bash's own expansion against the
assignment within the script. **wsl.exe's argv `$`-substitution does
not apply** because the script body comes from a file, not from argv.
This makes file feed a workaround for OQ-1's escape requirement when
the body is heavy in `$` references.

#### 7B — Multi-line body via PS array → `Set-Content`

Phase 1:
```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'powershell -NoProfile -Command "Set-Content -Path C:\Users\bhyslop\probe.sh -Value @('\''#!/bin/bash'\'','\''set -e'\'','\''ztmp=$(mktemp)'\'','\''echo TMPVAL=$ztmp'\'','\''rm -f $ztmp'\'')"'
# EXIT_WRITE=0
```

Phase 2:
```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash /mnt/c/Users/bhyslop/probe.sh'
# stderr: /mnt/c/Users/bhyslop/probe.sh: line 2: set: -: invalid option
# stderr: set: usage: set [-abefhkmnptuvxBCEHPT] [-o option-name] [--] [-] [arg ...]
# stdout: TMPVAL=/tmp/tmp.W0BUvegbUi
# EXIT_RUN=0
```

`Set-Content` writes CRLF line endings by default. bash on Linux reads
`set -e\r` and treats the `\r` as part of the option, which fails.
The script keeps running because `set -e` itself never activated.
Output partial.

This is a normalization issue, not a transport issue. The file is on
disk; cmd.exe newline fragility was avoided. The remaining problem is
the encoding choice on the writer side.

#### 7C — CRLF normalization via `tr` before exec

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "tr -d \"\\r\" </mnt/c/Users/bhyslop/probe.sh | bash"'
# stdout: TMPVAL=/tmp/tmp.lNbvGRvEbD
# EXIT_RUN=0
```

CRLF stripped, body runs cleanly.

**Caveat**: this form pipes into `bash` (no `-c` arg, no source file),
which means bash reads its script from FD 0. **This re-introduces SH-1's
stdin-consumption hazard** if the body spawns any subprocess that
itself reads FD 0 (notably `wsl.exe`-interop binaries). For the simple
probe body this is fine; for production bodies this form is unsafe.

The safer normalization: write to a *second* file with LF endings, then
exec from that file. Two extra steps. Or use a writer that produces LF
on the first pass (PS `[System.IO.File]::WriteAllText` with explicit
`\n`-joined content; or `Out-File -Encoding utf8` with manual newline
construction; or shell-side `printf` via cygwin/wsl).

### Cleanup

Probe.sh removed:
```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'powershell -NoProfile -Command "Remove-Item -Path C:\Users\bhyslop\probe.sh"'
# EXIT_RM=0
```

### Caveats (operational)

- The probe used `C:\Users\bhyslop\probe.sh` directly — production code
  should use a process-unique path under `$env:TEMP` or similar with
  cleanup discipline.
- The two-phase approach is not atomic: if phase 1 succeeds and phase 2
  never runs, the temp file persists. Cleanup belongs in the caller's
  flow, ideally with a `trap`-equivalent on the curia side.
- Process substitution (`bash <(...)`) was not separately probed; the
  syntactic complexity through cmd.exe quoting layers is high, and the
  file-feed approach already covers the use case.

## Summary

| OQ | Status   | WSG promotion |
|----|----------|---------------|
| 1  | resolved | SH-6 (wsl.exe argv `$name`/`${name}` substitution; per-letter escape table) |
| 2  | resolved | subsumed by SH-6 c-letter row (cygwin: no escape needed) |
| 3  | deferred | no Linux/Mac fundus in release-1 matrix; named in WSG's "Deferred" section |
| 4  | resolved | subsumed by SH-6 (per-letter escape table) |
| 5  | resolved | SH-7 (exit-code propagation across transports) |
| 6  | resolved | PS-4 (lazy-flush is transport-agnostic); PS-2 tightened for cmdlet-specificity |
| 7  | resolved | SH-8 (multi-line bodies via file feed; CRLF caveat; no-stdin-pipe rule) |

## Convention for future Windows-transport experiments

When new open questions surface around the Windows transport stack
(wsl.exe, cygwin, cmd.exe, OpenSSH-Win32, PowerShell), file the probe
matrix and resolution at:

`Memos/memo-YYYYMMDD-windows-transport-{topic}.md`

Cite the memo from WSG once findings stabilize into rules. WSG retains
the distilled rules and one-line probe pairs; memos retain the full
empirical record.
