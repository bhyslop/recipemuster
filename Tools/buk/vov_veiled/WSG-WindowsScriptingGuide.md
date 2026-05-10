# WSG — Windows Scripting Guide

## Purpose

WSG codifies the discipline for embedding remote shell actions (PowerShell or
bash) inside ssh sessions to Windows hosts, with full error trapping at every
layer. WSG extends BCG's bash discipline into the ssh-to-Windows transport
stack — where curia bash code authors a string that traverses ssh → Windows
OpenSSH → cmd.exe → child shell → script execution.

## Core Philosophy

**Every layer is a fault domain.** A character that means one thing in
curia bash means another in cmd.exe, another in the Windows argv parser,
another in PowerShell's `-Command` parser, and another in Linux bash. The
transport transforms the body at each boundary; errors at any layer can be
silently swallowed if not trapped.

**The cmd.exe layer is invisible from curia.** Every command is wrapped in
`cmd.exe /c <command>` before any other shell sees it. Cmd.exe's quoting
rules (`"..."` only; no single-quote support; idiosyncratic `\` handling)
bite bodies that look bash-correct from the curia.

**Some object output formatters are lazy.** PowerShell flushes string
output eagerly but some cmdlet formatter cycles flush only on full
pipeline completion; `exit` mid-body discards unflushed output. Treat the
lazy default as the safe assumption (WSp-102).

**Trap, don't trust.** Assume any error which can plausibly be silenced
WILL be silenced if not trapped. Surface every non-zero exit, every empty
stdout, every unflushed buffer.

## Transport Stack

A typical chain:

```
1. Local bash (curia)            builds ssh argument string
2. ssh client                    forwards command to remote
3. Windows OpenSSH sshd          spawns DefaultShell (cmd.exe)
4. cmd.exe /c <command>          parses tokens, respects "...", spawns child
5. Child shell (one of):
   a. powershell.exe -Command "<body>"     runs body in PowerShell
   b. wsl.exe ... bash -c "<body>"          forwards to Linux bash in WSL
   c. C:/cygwin64/bin/bash --login -c "..." runs Cygwin bash
6. Body subprocess(es)           child processes inherit parent's FD 0
```

Each step has its own quoting model. Each can fail silently.

## Established Rules

Headers tag the rule's *named* phenomenon: ❌ for a failure mode, ✅ for an
established correct shape. `WSp-` rules constrain PowerShell body
authoring; `WSs-` rules constrain shell/bash body authoring; `WSt-` rules
constrain transport quoting, escaping, and exit-code propagation across
boundaries (any body language). Numbering starts at 101 within each
family to leave room for insertions without renumbering.

### ❌ WSp-101: Trailing `exit $LASTEXITCODE` without LASTEXITCODE init

In a fresh `powershell -Command` with no native command, `$LASTEXITCODE`
is `$null`; the typed comparison `$null -ne 0` evaluates `True`, so the
trailer fires unconditionally and `exit $null` short-circuits the lazy
object-formatter pipeline, discarding buffered cmdlet output. Initialize
`$LASTEXITCODE = 0` so the trailer only fires on real native-command
failures.

```powershell
# ❌ Eats Get-LocalUser's table output (returns CRLF only)
powershell -Command "Get-LocalUser -Name 'foo' -ErrorAction SilentlyContinue;
                     if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }"

# ✅ Initialize so the branch only fires on real native command failures
powershell -Command "$LASTEXITCODE = 0;
                     Get-LocalUser -Name 'foo' -ErrorAction SilentlyContinue;
                     if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }"
```

### ❌ WSp-102: `exit` mid-body before object formatters flush

PowerShell flushes string output eagerly but some cmdlet object outputs
lazily (Out-Default → Format-Table → render); `exit` aborts the formatter
pipeline before lazy outputs render. Assume any cmdlet output is lazy
unless verified otherwise; materialize with `| Out-String` before any
exit. WSp-101's `$LASTEXITCODE = 0` makes this moot for happy paths;
failure paths still need string-materialization.

```powershell
# ❌ Get-LocalUser's table is lost; Get-Date's string survives
powershell -Command "Get-Date; Get-LocalUser -Name 'foo'; exit 0"

# ✅ Materialize objects to strings before any potential exit
powershell -Command "Get-LocalUser -Name 'foo' | Out-String; ..."
```

### ✅ WSp-103: PowerShell single-quoted strings survive nesting

Cmd.exe treats single quotes as literal characters; PowerShell recognizes
`'...'` as a string literal. Use single quotes for PS literals inside the
outer `"..."` — they nest reliably through every layer.

```bash
# ✅ Single-quoted PS literals inside outer "..."
ssh ... "powershell -NoProfile -Command \"Get-LocalUser -Name 'username'\""
```

### ✅ WSp-104: WSp-102's lazy-flush behavior is transport-agnostic

Lazy-flush is a property of the body, not of how PowerShell was launched —
the same `Get-LocalUser; exit 0` body emits empty stdout through every
transport. The fix (Out-String materialization, or WSp-101's `$LASTEXITCODE = 0`)
applies uniformly.

### ❌ WSp-105: PowerShell bodies are single expressions

A PowerShell body (`powershell -Command "<body>"` or capture variant)
MUST be exactly one of:

1. One cmdlet call (arguments, parameters, pipelined formatters like
   `| Out-String` or `| Out-Null`).
2. One native binary call.
3. One native binary with an inner-shell body
   (e.g. `wsl.exe ... bash -c "..."`).

The enumeration is constructive. Forbidden at body top level: `if`,
`try`/`catch`, `foreach`, `while`, `switch`, `do`, `&&`/`||`,
`;`-separated statements, intermediate `$var` assignments, ternary `?:`.
A single `if`-guarded effect like
`if (Test-Path '...') { Remove-Item '...' }` is also forbidden — the
body decides AND acts; decisions belong in bash. See CDD below.

The wrapper-side prelude
(`$ErrorActionPreference='Stop'; $env:WSL_UTF8=1; $LASTEXITCODE=0; <body>; if ...`)
is library code; the rule constrains caller bodies only.

```bash
# ❌ Compound state machine in PS body
priv_ps "\$sid=(Get-LocalUser '${z_user}').SID.Value; \$path='HKLM:\\...\\' + \$sid; New-Item \$path -Force | Out-Null; New-ItemProperty \$path -Name 'X' -Value 'Y' -Force | Out-Null"

# ❌ Single `if`-guard — body decides AND acts; state machine belongs in bash
priv_ps "if (Test-Path '${z_tar_path}') { Remove-Item -Force '${z_tar_path}' }"

# ✅ Bash-orchestrated single-expression calls
z_sid=$(ps_capture "(Get-LocalUser '${z_user}').SID.Value") || die "..."
z_regkey="HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ProfileList\\${z_sid}"
priv_ps "New-Item -Path '${z_regkey}' -Force | Out-Null"
priv_ps "New-ItemProperty -Path '${z_regkey}' -Name 'X' -Value 'Y' -Force | Out-Null"

# ✅ Single guarded effect repaired via Capture-Decide-Dispatch
z_present=$(ps_capture "Test-Path '${z_tar_path}'") || die "..."
if [[ "${z_present}" == "True" ]]; then
  priv_ps "Remove-Item -Force '${z_tar_path}'" || die "..."
fi
```

### ❌ WSp-106: Don't interpolate strings via PowerShell when bash can build them

The bash caller builds the string by expanding `${bash_var}` before
sending; the PS body receives a literal. Bash escape rules cascade once;
adding PS-side interpolation (`"...$var..."`) layers another substitution
model on top and compounds with WSp-107's native-binary quoting traps.

```bash
# ❌ PS-side string concatenation
priv_ps "\$path='HKLM:\\...\\' + \$sid; New-Item \$path -Force | Out-Null"

# ✅ Bash builds the string; PS receives a literal
z_path="HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ProfileList\\${z_sid}"
priv_ps "New-Item -Path '${z_path}' -Force | Out-Null"
```

### ❌ WSp-107: Don't interpolate variables through PowerShell to native binaries

PowerShell's argv handling for native Windows binaries (reg.exe, sc.exe,
schtasks.exe) mishandles embedded spaces — a path containing `Windows NT`
breaks `reg.exe` invoked through PS even via a PS variable. Use PS cmdlets
for PS-native effects (registry as PS-drive, ACLs, account ops); for
native-binary effects, pass already-resolved bash literals or invoke
through cmd.exe directly.

```bash
# ❌ Native binary with PS interpolation — argv quoting may eat path spaces
priv_ps "reg.exe add \"HKLM\\SOFTWARE\\Microsoft\\Windows NT\\...\\\$sid\" /v X /t REG_SZ /d Y /f"

# ✅ PS cmdlet handles PS-drive registry paths cleanly
priv_ps "New-ItemProperty -Path '${z_regkey}' -Name 'X' -Value 'Y' -Force | Out-Null"
```

### ❌ WSp-108: Error suppression is not idempotency

Error-suppression flags on destructive cmdlets (`-ErrorAction Ignore`,
`-ErrorAction SilentlyContinue`, `2>$null`, `try { ... } catch { }`)
swallow **every** error class — permission denied, file locked, ACL
refusal, path-too-long — converting failure into apparent success.
Forbidden on destructive actions. Idempotency MUST come from explicit
state capture, then unconditional action (see CDD).

Distinguish suppressing **output** from suppressing **errors**:
`| Out-Null` discards the return object (cmdlet still throws); `-ErrorAction
Ignore` discards the error (cmdlet returns success on real failure).
WSp-108 forbids the latter on destructive actions.

Probe-side use is permitted on cmdlets whose purpose is to *return state*
(`Get-LocalUser -ErrorAction SilentlyContinue` where "absent" is the
answer); forbidden on cmdlets whose purpose is to *change state*.

```bash
# ❌ Error-suppression masquerading as idempotency
priv_ps "Remove-Item -Force '${z_tar_path}' -ErrorAction Ignore"
priv_ps "Remove-Item -Recurse -Force '${z_dir}' -ErrorAction SilentlyContinue"
priv_ps "wsl.exe --unregister ${z_dist} 2>\$null; \$LASTEXITCODE = 0"

# ✅ CDD: capture, decide, dispatch unconditionally
z_present=$(ps_capture "Test-Path '${z_tar_path}'") || die "..."
if [[ "${z_present}" == "True" ]]; then
  priv_ps "Remove-Item -Force '${z_tar_path}'" || die "..."
fi

# ✅ Probe-side suppression — Get-LocalUser is non-destructive, "user not found" is the answer
priv_ps "Get-LocalUser -Name '${z_user}' -ErrorAction SilentlyContinue"
```

### ❌ WSs-101: Script via stdin to bash (heredoc form / `bash -s`)

When ssh feeds a script to bash via stdin (`bash -s`, heredoc), bash
reads commands from FD 0 and child processes inherit that FD; any child
that reads FD 0 (notably Windows binaries via wsl.exe interop) consumes
script bytes bash hasn't parsed yet. Bash hits EOF prematurely, exits 0,
and remaining commands silently never execute.

```bash
# ❌ Script via heredoc/stdin; net.exe (or any FD-0 reader) eats bytes
ssh ... "wsl.exe ... bash -s" <<SCRIPT
set -euo pipefail
net.exe user 'foo' /add ... > /dev/null
useradd 'foo'                       # may silently never run
SCRIPT

# ✅ Body as bash -c argument; bash reads body from -c, not FD 0
ssh ... "wsl.exe ... bash -c \"net.exe user 'foo' /add ...\""
```

### ❌ WSs-102: BCG bans extend across the transport boundary

Bash authored at the curia AND bash authored for remote execution are
both subject to BCG: no heredocs anywhere, no pipelines inside `$()`,
no unguarded `$()`. Top-level pipelines (outside `$()`) are permitted
as WSs-104 shape 2.

### ✅ WSs-103: Multi-line bodies via file feed (when WSt-101 doesn't fit)

When a body materially exceeds what `;`-join can express: phase 1 ships
the body as a remote file via PowerShell; phase 2 runs
`wsl.exe ... bash /path/to/file` (or `cygwin bash --login -c "bash
/path/to/file"`). Side benefit: bypasses WSt-104 since the body comes
from disk, not argv.

Caveats:
- PS `Set-Content` writes CRLF. Use `[System.IO.File]::WriteAllText($path, ($lines -join "`n"))`
  or normalize CRLF→LF into a second file before exec.
- Do NOT pipe the file into `bash` via stdin — reintroduces WSs-101. Use
  `bash /path/to/file`, not `bash <file`.
- Two-phase is not atomic; the caller flow must clean up the remote temp.

Default discipline remains WSt-101 + WSs-104; WSs-103 applies only when
bodies don't fit one line.

### ❌ WSs-104: Bash bodies are single statements

A remote-bash body (`bash -c "<body>"`) MUST be exactly one of:

1. One simple command (with arguments, options, redirections).
2. One pipeline, optionally preceded by `set -o pipefail` (same
   statement — pipefail is precondition, not separate operation).
3. One simple command whose argument is itself an inner-shell body
   (e.g. `wsl.exe --user root install ...`, `sudo -n install ...`).

The enumeration is constructive. Forbidden at body top level:
`if`/`then`/`else`/`fi`, `for`/`while`/`until`, `case`, `&&`/`||` chains,
`;`-separated statements, intermediate `var=...` assignments, `trap`,
function definitions, `(...)` subshells. A single `if`-guarded effect
like `if test -e '...'; then sudo -n rm '...'; fi` is also forbidden —
the body contains a decision (test) and an action (consequent);
decisions belong in bash on the curia. See CDD below.

```bash
# ❌ Compound state machine — six statements, cross-statement variable, trap
priv_bash b "set -euo pipefail" \
  "ztmp=\$(mktemp)" \
  "trap 'rm -f \"\${ztmp}\"' EXIT" \
  "echo '<b64>' | openssl enc -base64 -d -A > \"\${ztmp}\"" \
  "sudo -n install -m 600 -o user -g user \"\${ztmp}\" '<target>'"

# ❌ Single `if`-guarded effect — body decides AND acts
priv_bash b "if test -e '${z_path}'; then sudo -n rm '${z_path}'; fi"

# ✅ Single simple command; key flows from curia file via ssh stdin into /dev/stdin
priv_bash b "sudo -n install -m 600 -o '${z_user}' -g '${z_user}' /dev/stdin '${z_target}'" \
  < "${KEY_FILE}"

# ✅ Single guarded effect repaired via Capture-Decide-Dispatch
local z_exit=0
priv_bash b "stat '${z_path}' > /dev/null 2>&1" || z_exit=$?
if [[ ${z_exit} -eq 0 ]]; then
  priv_bash b "sudo -n rm '${z_path}'" || die "Failed to remove ${z_path}"
fi
```

The `< "${KEY_FILE}"` redirection attaches the file to ssh's FD 0; sshd
forwards to the remote command's FD 0; the body reads it as `/dev/stdin`.
File content never enters argv. WSs-101 is structurally avoided — the
body is one command that explicitly consumes `/dev/stdin`, so no later
command can be starved by leftover bytes.

### ❌ WSt-101: Newlines in bodies through cmd.exe

Cmd.exe is line-oriented; newlines in `"..."` quoted args fragment across
the cmd.exe → child-binary boundary. Bodies must be single-line, including
characters inside one long pipeline written for readability.

```bash
# ❌ Newlines in body; cmd.exe may fragment
ssh ... "wsl.exe ... bash -c \"
useradd foo
\""

# ✅ Single-line body
ssh ... "wsl.exe ... bash -c \"useradd foo\""
```

### ✅ WSt-102: Outer `"..."` with `\"` for embedded double-quotes

The Windows argv parser (wsl.exe and most Windows binaries) honors `\"`
inside `"..."` as escape for literal `"`. Cmd.exe passes bytes through; the
child's argv parser handles the escape, reaching Linux bash as `"`.

```bash
# ✅ Embedded " in body
ssh ... "wsl.exe ... bash -c \"echo SAW: \\\"hello\\\"\""
```

### ❌ WSt-103: cmd.exe does NOT process `$()` or `$var`

Cmd.exe's variable syntax is `%var%` and it has no command substitution.
`cmd.exe /c echo "X=$(uname)"` emits the literal `X=$(uname)`. The outer
cmd.exe layer is not what processes shell variable expansion in transit
(see WSt-104 for what does, on the w-letter path).

### ❌ WSt-104: wsl.exe substitutes `$name` and `${name}` in argv

Wsl.exe's argv-to-Linux processor performs `$name`/`${name}` substitution
against its Linux-side root-shell environment before bash receives the
body; undefined names substitute to empty. Cygwin and cmd.exe alone do
not. `$(...)` is unaffected (the `(` after `$` does not match the token
shape).

Per-letter escape table:

| Letter | Path                                       | `$name`     | `${name}`     | `$(...)` |
|--------|--------------------------------------------|-------------|---------------|----------|
| b      | direct ssh to Linux/Mac                    | none        | none          | none     |
| c      | cmd.exe → cygwin bash                      | none        | none          | none     |
| w      | cmd.exe → wsl.exe → bash                   | `\$name`    | `\${name}`    | none     |

```bash
# ❌ Fails on w-letter — wsl.exe substitutes $ztmp to empty before bash runs
ssh ... 'wsl.exe --distribution <dist> --user root bash -c "ztmp=HELLO; echo $ztmp"'
# stdout: (empty)

# ✅ \$ defers expansion to bash
ssh ... 'wsl.exe --distribution <dist> --user root bash -c "ztmp=HELLO; echo \$ztmp"'
# stdout: HELLO
```

### ✅ WSt-105: Exit-code propagation is reliable across transports

Non-zero exit codes propagate cleanly through cmd.exe direct, wsl.exe →
bash, cygwin bash, PowerShell, and Windows native binaries via wsl.exe
interop. WSs-104's single statement IS the body's exit code; the caller
absorbs via `|| die "..."`. No `set -e` needed inside the body (nothing
after one statement). `set -o pipefail` IS needed for shape-2 pipelines
when first-failure surfacing is the intent.

```bash
# Probe: false propagation through w-letter
ssh ... 'wsl.exe --distribution <dist> --user root bash -c "false"'; echo "EXIT=$?"
# EXIT=1
```

### ❌ WSt-106: Single quotes are LITERAL characters in cmd.exe-direct transport

When routed `bash (curia) → ssh → cmd.exe → <Windows-native binary>` with
no PowerShell or remote-bash layer, single quotes pass through to the
binary as argv bytes. Pick the quote form by the **innermost interpreter**:

```bash
# ❌ wsl.exe sees paths with literal `'...'` quotes and fails.
ssh "${USER}@${HOST}" "wsl.exe --import dist 'C:\Users\u\dist-fs' '...' --version 2"

# ✅ \"...\" reaches the wire as "..."; cmd.exe strips, wsl.exe sees clean paths.
ssh "${USER}@${HOST}" "wsl.exe --import dist \"C:\Users\u\dist-fs\" \"...\" --version 2"
```

| Innermost interpreter         | Quote form for embedded args                                       |
|-------------------------------|--------------------------------------------------------------------|
| PowerShell                    | `'...'` (PS string literal)                                        |
| Remote bash (`bash -c "..."`) | inner `'...'` (bash strips)                                        |
| cmd.exe → native binary       | `\"...\"` (becomes literal `"..."` on the wire; cmd.exe strips)    |

## Capture-Decide-Dispatch Pattern

Bash owns the state machine; the remote-side body is one effect. Any
decision that would otherwise live in a remote body decomposes into:

1. **Capture** — `ps_capture "<single-expression probe>"` (WSp-105-clean)
   or `$(priv_bash LETTER "<single-statement probe>")` (WSs-104-clean)
   returns stdout into a bash variable. Absorb with `|| die "..."` if the
   probe must succeed, or `|| z_var=""` / `|| z_exit=$?` if an absent
   fundus state is legitimate pre-condition.

2. **Decide** — bash conditional. PS booleans: `[[ "$x" == "True" ]]`
   (see boolean serialization below). Text/list: `grep -qFx PATTERN <<<"$x"`,
   `[[ "$x" == ... ]]`, or `case`. Bash exit-status probes:
   `[[ ${z_exit} -eq 0 ]]`.

3. **Dispatch** — `priv_ps "<effect>"` or `priv_bash LETTER "<effect>"`
   runs the unconditional action on branches that need it.

### PowerShell boolean serialization

`Test-Path`, `Test-Connection`, and other PS-bool-emitting cmdlets
serialize as literal strings `"True"` / `"False"` through `ps_capture`.
Use `[[ "${z_var}" == "True" ]]`, not `[[ -n "${z_var}" ]]` — `False` is
also non-empty.

### Capture failure absorption

`ps_capture` returns non-zero on ssh transport failure, PS body throwing
under `$ErrorActionPreference=Stop`, native-binary non-zero exit, or
wrapper-trailer fire. Two absorption shapes:

```bash
# Probe must succeed — die loudly on transport/body failure
z_present=$(ps_capture "Test-Path '${z_path}'") \
  || die "Failed to probe ${z_path}"

# Empty result on probe failure is a valid pre-state — absorb silently
z_wsl_list=""
z_wsl_list=$(ps_capture "wsl.exe --list --quiet") \
  || z_wsl_list=""
```

Use the loud form by default. Use the silent form only when the absent
state is a legitimate pre-condition.

## Idempotency Exemplars

Ready-made CDD shapes; copy and adjust variable names.

### File presence → conditional remove

```bash
local z_present
z_present=$(ps_capture "Test-Path '${z_path}'") \
  || die "Failed to probe ${z_path}"
if [[ "${z_present}" == "True" ]]; then
  priv_ps "Remove-Item -Force '${z_path}'" \
    || die "Failed to remove ${z_path}"
fi
```

### Directory presence → conditional remove (recursive)

```bash
local z_present
z_present=$(ps_capture "Test-Path '${z_dir}'") \
  || die "Failed to probe ${z_dir}"
if [[ "${z_present}" == "True" ]]; then
  priv_ps "Remove-Item -Recurse -Force '${z_dir}'" \
    || die "Failed to remove ${z_dir}"
fi
```

### WSL distro membership → conditional unregister

Fresh-WSL host with no installed distros is a legitimate pre-state, so
the list capture absorbs failure silently.

```bash
local z_wsl_list=""
z_wsl_list=$(ps_capture "wsl.exe --list --quiet") \
  || z_wsl_list=""
if grep -qFx "${z_dist}" <<<"${z_wsl_list}"; then
  priv_ps "wsl.exe --unregister ${z_dist}" \
    || die "Failed to unregister ${z_dist}"
fi
```

### Local user existence → conditional removal

`-ErrorAction SilentlyContinue` permitted per WSp-108's probe-side carve-out.

```bash
local z_user_state
z_user_state=$(ps_capture \
  "Get-LocalUser -Name '${z_user}' -ErrorAction SilentlyContinue") \
  || die "Failed to probe local user ${z_user}"
z_user_state="${z_user_state//[$'\r\n\t ']/}"
if [[ -n "${z_user_state}" ]]; then
  priv_ps "Remove-LocalUser -Name '${z_user}'" \
    || die "Failed to remove local user ${z_user}"
fi
```

### Service running state → conditional start

```bash
local z_service_state
z_service_state=$(ps_capture \
  "(Get-Service -Name '${z_svc}').Status") \
  || die "Failed to probe service ${z_svc}"
if [[ "${z_service_state}" == "Stopped" ]]; then
  priv_ps "Start-Service -Name '${z_svc}'" \
    || die "Failed to start service ${z_svc}"
fi
```

### Registry key absence → conditional creation

```bash
local z_present
z_present=$(ps_capture "Test-Path '${z_regkey}'") \
  || die "Failed to probe registry key ${z_regkey}"
if [[ "${z_present}" != "True" ]]; then
  priv_ps "New-Item -Path '${z_regkey}' -Force | Out-Null" \
    || die "Failed to create registry key ${z_regkey}"
fi
```

## Wrapper Discipline

### PowerShell wrapper

```bash
ssh ... "${USER}@${HOST}" \
    "powershell -NoProfile -Command \"\$ErrorActionPreference = 'Stop'; \$env:WSL_UTF8 = 1; \$LASTEXITCODE = 0; ${z_body}; if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE }\""
```

Wrapper prelude:
- `\$ErrorActionPreference = 'Stop'` — PS errors terminate
- `\$env:WSL_UTF8 = 1` — wsl.exe (when called from PS body) emits UTF-8
- `\$LASTEXITCODE = 0` — initialize to defeat the null trap (WSp-101)
- Trailing `if ... exit ...` — propagate native command exit codes

Body authoring follows WSp-102 through WSp-108.

### Bash-via-wsl.exe wrapper (w-letter)

```bash
ssh ... "${USER}@${HOST}" \
    "wsl.exe --distribution ${DIST} --user root bash -c \"${z_body}\""
```

Body authoring follows the WSs- and WSt- rule families, with WSt-104's
w-letter escape table: `$name` → `\$name`; `${name}` → `\${name}`;
`$(...)` unescaped.

### Bash-via-cygwin wrapper (c-letter)

```bash
ssh ... "${USER}@${HOST}" \
    "C:/cygwin64/bin/bash --login -c \"${z_body}\""
```

Body authoring follows the WSs- and WSt- rule families. Per WSt-104's
c-letter row, `$name`, `${name}`, and `$(...)` all pass unescaped —
cmd.exe alone does no shell substitution.

## Deferred

The b-letter path (direct ssh to Linux/Mac) bypasses every Windows layer;
standard BCG body discipline applies. When adding a non-Windows target,
run analog probes substituting plain bash for wsl.exe / cygwin.

## Verification Probe Template

```
# Probe N: <claim>
# Pre-state: <relevant remote state>

<priv-ssh wrapper> <node> "<failing form>"
# stdout: <observed>; exit: <observed>

<priv-ssh wrapper> <node> "<repaired form>"
# stdout: <observed>; exit: <observed>
```

Probes are read-only where possible; mutating probes must declare cleanup.

## Acronym Registry

| Term | Expansion |
|------|-----------|
| WSG | Windows Scripting Guide (this document) |
| OpenSSH-Win32 | The Win32 port of OpenSSH bundled with Windows 10+ |
| Curia | The local invoking machine |
| Fundus | The remote target machine |

## Related Documents

- BCG — base bash discipline
