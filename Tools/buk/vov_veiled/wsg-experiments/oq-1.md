# OQ-1 — Which layer eats `$name` in wsl.exe transit?

## Hypothesis (original WSG framing)

WSG line 251–263 framed OQ-1 as "what eats `$()` and `$var` in `wsl.exe ... bash -c "<body>"`?" with cmd.exe ruled out (SH-4) and the candidates listed as wsl.exe argv parser, parens-in-quotes interaction, or OpenSSH-Win32 spawn side-effect.

The matrix below shows the original framing was half-right: `$(...)` is NOT eaten — it survives to bash unchanged. Only `$name` and `${name}` references are mangled. The eater is wsl.exe's argv parser (or its argv-to-Linux-invocation transform), and the mechanism is environment-variable substitution against wsl.exe's own environment, not nullification.

## Fundus

- Node: `bujn-winpc` (BURN_HOST=rocket, BURN_PLATFORM=bubep_windows)
- Date: 2026-05-08
- Probe vehicle: `tt/buw-jpS.PrivilegedSsh.sh bujn-winpc <command>`
- WSL distribution: `rbtww-main` (admin: `--user root`)
- Cygwin path: `C:/cygwin64/bin/bash --login`

## Probe matrix

Curia bash uses single-quoted argv to the tabtarget, so `$` reaches the tabtarget verbatim. The tabtarget passes argv to ssh as the remote command; Windows OpenSSH hands it to cmd.exe (DefaultShell); cmd.exe forwards to wsl.exe / cygwin / powershell as the case may be.

### 1A — cmd.exe alone, baseline (re-verify SH-4)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'echo "VAR=$X SUB=$(uname)"'
```

stdout:
```
"VAR=$X SUB=$(uname)"
```

cmd.exe emits the body verbatim (including the surrounding `"` — cmd.exe's `echo` is wholly verbatim). Neither `$X` nor `$(uname)` is touched. Re-confirms SH-4 on this fundus.

### 1B — wsl.exe + bash, single statement, mixed `$X` and `$(uname)`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "echo VAR=$X SUB=$(uname)"'
```

stdout:
```
VAR= SUB=Linux
```

`$(uname)` survived to bash (bash ran `uname` → `Linux`). `$X` is empty — undefined env var resolved to empty string, but it was substituted by *something*: bash's own expansion of an undefined `$X` would also yield empty, so this single probe doesn't yet localize the layer.

### 1C — prior cycle's symptom: `ztmp=$(mktemp); echo $ztmp`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=$(mktemp); echo TMPVAL=$ztmp"'
```

stdout:
```
TMPVAL=
```

Reproduces the symptom that motivated OQ-1.

### 1D — replace `$(mktemp)` with `$(echo HELLO)` to rule out missing-binary

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=$(echo HELLO); echo TMPVAL=$ztmp"'
```

stdout:
```
TMPVAL=
```

Same empty result. Rules out "wsl.exe ran `mktemp` Windows-side and got nothing." Something's substituting `$ztmp` to empty before bash sees the body.

### 1E — `$()` alone, single statement, no `;`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "echo SUB=$(echo HELLO)"'
```

stdout:
```
SUB=HELLO
```

`$(...)` worked when not paired with later `$name` reference. This is the first crack in the original "`$()` is eaten" hypothesis: it is not.

### 1F — `;` separator, no `$` references

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "echo A; echo B"'
```

stdout:
```
A
B
```

`;` works fine. Both statements ran.

### 1G — `;` + literal assignment + `$name` reference (the canonical failure)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo TMPVAL=$ztmp"'
```

stdout:
```
TMPVAL=
```

No `$()` involved at all. Plain assignment + bare `$ztmp` reference. Empty result. **The eater operates on `$name`, not `$()`.**

### 1H — same body via cygwin bash (also answers OQ-2)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'C:/cygwin64/bin/bash --login -c "ztmp=HELLO; echo TMPVAL=$ztmp"'
```

stdout:
```
TMPVAL=HELLO
```

**Cygwin works.** The differentiating factor is wsl.exe — cygwin's path (cmd.exe → C:/cygwin64/bin/bash.exe) does not exhibit the substitution. The eater is wsl.exe specifically.

### 1I — does `${braces}` form escape the substitution?

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo TMPVAL=${ztmp}"'
```

stdout:
```
TMPVAL=
```

Braces don't help. wsl.exe substitutes `${name}` form too.

### 1J — `\$name` escape form

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo TMPVAL=\$ztmp"'
```

stdout:
```
TMPVAL=HELLO
```

**`\$name` escape works.** The `\` survives to wsl.exe, wsl.exe treats `\$` as a literal `$` (not a substitution trigger), and bash sees `$ztmp` literally and does its own expansion against its own environment where `ztmp` was just assigned. Round-trip succeeds.

### 1K — `$USERNAME` (set in Windows but probably not in wsl Linux env)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "echo USERNAME_TEST=$USERNAME"'
```

stdout:
```
USERNAME_TEST=
```

Empty. Compare to 1L below.

### 1L — confirm USERNAME exists in Windows (cmd.exe %VAR% form)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'echo USERNAME_TEST=%USERNAME%'
```

stdout:
```
USERNAME_TEST=bhyslop
```

USERNAME exists in the Windows ssh session's environment. wsl.exe did NOT use it for substitution. **wsl.exe's substitution does not pull from Windows env.** The substitution scope is the wsl.exe Linux-side environment (or some intermediate empty environment).

### 1M — `$PATH` (definitely set in wsl.exe Linux root env)

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "echo PATH_TEST=$PATH"'
```

stdout:
```
PATH_TEST=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:...
```

`$PATH` got substituted to the Linux PATH. **The substitution scope is the wsl.exe Linux-side environment** (or at least, that environment is the lookup table for `$name`). Names defined there → value; names not defined → empty.

(This also means wsl.exe's argv processor is substituting BEFORE bash runs — not after — because bash inside the body would see the same value, but the proof that the substitution happens upstream is 1G/1J: `ztmp=HELLO; echo $ztmp` returns empty in 1G even though the same bash process assigned `ztmp` mid-body; only `\$` defers the lookup to bash, which then sees its own assignment.)

### 1N — `;` + assignment, no `$` at all

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo TMPVAL=ZZZ"'
```

stdout:
```
TMPVAL=ZZZ
```

Confirms `;` itself is not the culprit; only `;` + `$varname-undefined-in-wsl-startup-env` together.

### 1O — `;` + `$PATH`

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "echo A; echo X=$PATH"'
```

stdout:
```
A
X=/usr/local/sbin:/usr/local/bin:...
```

`;` + `$PATH` works because PATH IS in wsl.exe's startup env. Substitution succeeds with the right value.

### 1P — full canonical escape: `\$(mktemp)` and `\$ztmp` together

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=\$(mktemp); echo TMPVAL=\$ztmp"'
```

stdout:
```
TMPVAL=/tmp/tmp.hNpO0X8BXI
```

Canonical pattern. Both `\$(mktemp)` and `\$ztmp` reach bash literally; bash performs its own expansion in its own scope. (Note: leaves a temp file; cleaned by tmpwatch.)

### 1Q — `\${braces}` escape form

```
./tt/buw-jpS.PrivilegedSsh.sh bujn-winpc 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo TMPVAL=\${ztmp}"'
```

stdout:
```
TMPVAL=HELLO
```

`\${name}` also works.

## Resolution

**Mechanism**: wsl.exe's argv-to-Linux-invocation processor performs `$name` and `${name}` substitution against its own (Linux-side, root-shell startup) environment before constructing the bash invocation. Names present in that environment substitute to their values; undefined names substitute to empty.

**Not affected**:
- cmd.exe (proven by 1A; matches existing SH-4)
- Cygwin bash (proven by 1H; this resolves OQ-2)
- `$(...)` command substitution (proven by 1E; the `(` after `$` does not match the `$name` / `${name}` token shape wsl.exe is looking for)

**Escape rule**: `\$name` and `\${name}` reach bash with literal `$` (the `\` is consumed by wsl.exe's parser and absorbed; bash sees `$name`/`${name}` and does its own expansion).

## Promotion plan

The original OQ-1 was framed across `$()` and `$var` together. The empirical answer splits along that line:

- **New rule SH-N (proposed)**: "wsl.exe substitutes `$name` and `${name}` in argv before bash sees the body. Use `\$name` and `\${name}` to defer substitution to remote bash. Does not apply to `$(...)` (not affected) or `$$`, `$@`, `$*`, `$1` (untested but follow the same `\$` escape pattern as a safety).""
- **OQ-2 resolves to**: "Cygwin does NOT substitute. No escape needed in cygwin -c bodies."  
- **OQ-4 resolves to**: the canonical escape is `\$` (single backslash before `$`); braces optional.

The "wrapper discipline" section's open block on bash-via-wsl.exe can be filled with: body-as-arg form (SH-1), `;`-joined for one-line (SH-2), `\"` for embedded `"` (SH-3), `\$` for variable references and `${...}` when bash needs them (new rule).

## Caveats and untested edges

- Substitution semantics for `$$` (PID), `$?`, `$1`, `$@`, `$*` not explicitly tested. Recommend the same `\$` escape as the safety default.
- Whether wsl.exe substitutes against `WSLENV`-promoted vars vs the Linux startup env exclusively was not characterized. The rule is the same either way: escape `\$` defers to bash.
- This is empirical for wsl 2 on Windows 10/11 as installed on rocket; the underlying behavior is plausibly a wsl.exe argv-quoting feature documented in WSL release notes (not researched here — the empirical behavior is what callers must work against).
