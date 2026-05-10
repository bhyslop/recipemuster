# WSG — Windows Scripting Guide

## Purpose

WSG codifies the discipline for embedding remote shell actions (PowerShell or
bash) inside ssh sessions to Windows hosts, with full error trapping at every
layer. WSG extends BCG's bash discipline into the ssh-to-Windows transport
stack — where curia bash code authors a string that traverses ssh → Windows
OpenSSH → cmd.exe → child shell → script execution.

## Core Philosophy

**Every layer is a fault domain.** A character that means one thing in curia
bash means another in cmd.exe, another in the Windows argv parser, another
in PowerShell's `-Command` parser, and another in Linux bash. The transport
**transforms** the body at each boundary. Errors at any layer can be
silently swallowed if not explicitly trapped.

**The cmd.exe layer is invisible from curia.** The user's command is wrapped
in `cmd.exe /c <command>` before any other shell sees it. Cmd.exe's quoting
rules — `"..."` only; no single-quote support; idiosyncratic `\` handling —
bite even bodies that look bash-correct from the curia.

**Some object output formatters are lazy.** PowerShell flushes string
output (Write-Host, Write-Output of strings) eagerly; some cmdlets'
formatter cycles flush only on full pipeline completion, so calling
`exit` mid-body (or trailer-side) discards their unformatted output.
The set of lazy cmdlets is not universal — treat the lazy default as
the safe assumption (PS-2). String emission always survives.

**Trap, don't trust.** Each rule below assumes that any error which can
plausibly be silenced WILL be silenced if not explicitly trapped. The bias
is toward defensive instrumentation: surface every non-zero exit, every
empty stdout, every unflushed buffer, with a forensic temp file naming the
boundary that swallowed it.

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

### ❌ PS-1: Trailing `exit $LASTEXITCODE` without LASTEXITCODE init

In a fresh `powershell -Command` invocation with no native command run,
`$LASTEXITCODE` is `$null`. PowerShell's typed comparison treats `$null -ne 0`
as `True`. A trailer like `if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }`
always fires for cmdlet-only bodies, calling `exit $null` (which exits 0 in
practice). The `exit` short-circuits PowerShell's lazy object-formatter
pipeline, discarding any cmdlet output buffered by Out-Default that hasn't
flushed to stdout yet.

```powershell
# ❌ Eats Get-LocalUser's table output (returns CRLF only)
powershell -Command "Get-LocalUser -Name 'foo' -ErrorAction SilentlyContinue;
                     if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }"

# ✅ Initialize so the branch only fires on real native command failures
powershell -Command "$LASTEXITCODE = 0;
                     Get-LocalUser -Name 'foo' -ErrorAction SilentlyContinue;
                     if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }"
```

### ❌ PS-2: `exit` mid-body before object formatters flush

PowerShell flushes string output eagerly but *some* cmdlet object outputs
lazily (Out-Default → Format-Table → render). `exit` aborts the formatter
pipeline. Lazy outputs that haven't flushed are discarded. Whether a given
cmdlet is lazy is formatter-cycle-specific: assume any cmdlet output is
lazy unless you have verified otherwise.

```powershell
# ❌ Get-LocalUser's table is lost; Get-Date's string survives
powershell -Command "Get-Date; Get-LocalUser -Name 'foo'; exit 0"

# ✅ Materialize objects to strings before any potential exit
powershell -Command "Get-LocalUser -Name 'foo' | Out-String; ..."
```

If a wrapper provides `$LASTEXITCODE = 0` initialization (PS-1), this rule
is moot for happy-path bodies. Failure paths still need string-materialization
guards if any cmdlet output must survive the trailer's exit branch.

### ❌ SH-1: Script via stdin to bash (heredoc form / `bash -s`)

When ssh feeds a script to remote bash via stdin — `bash -s` on the remote
with heredoc on the local side, or `echo BODY | bash` chains on the remote —
bash reads commands from FD 0. Child processes spawned by that bash inherit
FD 0, connected to the script-providing pipe. A child that reads FD 0
(notably Windows binaries called via wsl.exe interop) consumes script bytes
bash hadn't yet parsed. Bash hits EOF prematurely, exits 0 without errors,
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

### ❌ SH-2: Newlines in bodies through cmd.exe

Cmd.exe is line-oriented. Newlines in `"..."` quoted args are unreliable
across the cmd.exe → child-binary boundary. Bodies must be single-line —
including the characters inside one long pipeline written for readability.

```bash
# ❌ Newlines in body; cmd.exe may fragment
ssh ... "wsl.exe ... bash -c \"
useradd foo
\""

# ✅ Single-line body
ssh ... "wsl.exe ... bash -c \"useradd foo\""
```

### ✅ SH-3: Outer `"..."` with `\"` for embedded double-quotes

The Windows argv parser (used by wsl.exe and most Windows binaries) honors
`\"` inside `"..."` as an escape for literal `"`. This survives cmd.exe and
reaches Linux bash as `"`. Cmd.exe itself does NOT process `\"`; it passes
the bytes to the child whose argv parser handles the escape.

```bash
# ✅ Embedded " in body
ssh ... "wsl.exe ... bash -c \"echo SAW: \\\"hello\\\"\""
```

Probe: `bash -c "echo SAW: \"hello\""` → stdout `SAW: hello` (quotes
stripped by bash's own parsing of the unescaped `"`).

### ✅ PS-3: PowerShell single-quoted strings survive nesting

Inside the outer `"..."` of `powershell -Command "..."`, single-quoted
strings (`'Stop'`, `'username'`) pass through cleanly. Cmd.exe does not
honor single quotes for arg-grouping but treats them as literal characters;
PowerShell parses its body and recognizes `'...'` as PS string literals.

```bash
# ✅ Single-quoted PS literals inside outer "..."
ssh ... "powershell -NoProfile -Command \"Get-LocalUser -Name 'username'\""
```

Use single quotes for PS string literals — they nest reliably through every
layer.

### ✅ PS-4: PS-2's lazy-flush behavior is transport-agnostic

The PS-2 lazy-flush is a property of the PowerShell body, not of how
PowerShell was launched. The same `Get-LocalUser; exit 0` body emits
empty stdout via direct cmd.exe→powershell, cygwin→powershell.exe, and
wsl.exe→bash→powershell.exe. The fix (Out-String materialization, or
PS-1's `$LASTEXITCODE = 0` so the trailer doesn't fire) is also
transport-agnostic.

### ❌ SH-4: cmd.exe does NOT process `$()` or `$var`

Direct probe: `cmd.exe /c echo "SHELL=$0 X=$(uname)"` emits the literal
string `"SHELL=$0 X=$(uname)"` with no expansion. Cmd.exe's variable
syntax is `%var%` and it has no command substitution. The outer cmd.exe
layer is NOT the layer that processes shell variable expansion or
command substitution in transit.

### ❌ SH-5: BCG bans extend across the transport boundary

Bash authored at the curia AND bash authored for remote execution are
both subject to BCG. In particular:

- **No heredocs anywhere.** The transport must not use `bash -s` with
  heredoc input (SH-1); module callers must not build remote bodies via
  heredoc.
- **No pipelines inside `$()`.** Applies to curia code and to bodies
  authored for remote execution.
- **No unguarded `$()`.** Remote bodies that use command substitution must
  guard against silent failure the same way curia code does.

Top-level pipelines (not inside `$()`) are permitted in remote bodies as
shape 2 of SH-10 (a pipeline is one operation, optionally preceded by
`set -o pipefail`).

### ❌ SH-6: wsl.exe substitutes `$name` and `${name}` in argv

The wsl.exe argv-to-Linux-invocation processor performs `$name` and
`${name}` substitution against its own (Linux-side, root-shell startup)
environment before bash receives the body. Names present in that
environment substitute to their values; undefined names substitute to
empty. **Cygwin bash does not exhibit this behavior. cmd.exe alone (per
SH-4) also does not.** `$(...)` command substitution is unaffected (the
`(` after `$` does not match the `$name`/`${name}` token shape).

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

### ✅ SH-7: Exit-code propagation is reliable across transports

Non-zero exit codes propagate cleanly through every tested transport:
cmd.exe direct, wsl.exe → bash, cygwin bash, PowerShell, and native
Windows binaries called via wsl.exe interop. The single statement in the
body's shape (per SH-10) is also the body's exit code — it reaches the
curia unchanged via the wrapper's ssh exit status, and the caller absorbs
it via `|| die "..."`. No `set -e` discipline is needed inside the body,
because there is nothing after the single statement that `set -e` would
protect.

`set -o pipefail` IS needed for shape 2 (pipeline) when the pipeline's
intent is to fail if any stage fails — pipefail is a precondition of
the pipeline's exit-code semantics, not a separate operation, and is
permitted inside the same statement (see SH-10 exception).

```bash
# Probe: false propagation through w-letter
ssh ... 'wsl.exe --distribution <dist> --user root bash -c "false"'; echo "EXIT=$?"
# EXIT=1
```

### ✅ SH-8: Multi-line bodies via file feed (when SH-2 doesn't fit)

When a body materially exceeds what `;`-join can express, the canonical
pattern is two-phase: phase 1 ships the body as a remote file via
PowerShell `Set-Content` or `[System.IO.File]::WriteAllText`; phase 2
runs `wsl.exe ... bash /path/to/file` (or `cygwin bash --login -c "bash
/path/to/file"`). Side benefit: bypasses wsl.exe's argv `$`-substitution
(SH-6) since the script body comes from disk, not argv.

Caveats:
- PS `Set-Content` writes CRLF. Either use an LF-explicit writer:

  ```powershell
  [System.IO.File]::WriteAllText($path, ($lines -join "`n"))
  ```

  or normalize CRLF→LF into a *second* file before exec.
- Do NOT pipe the file into `bash` via stdin; that reintroduces SH-1's
  stdin-consumption hazard. Use `bash /path/to/file`, not `bash <file`.
- The two-phase approach is not atomic; the caller flow must clean up
  the remote temp file.

Default discipline remains single-line single-statement (SH-2 + SH-10);
SH-8 applies only when bodies don't fit one line.

### ❌ SH-9: Single quotes are LITERAL characters in cmd.exe-direct transport

When a command line is routed `bash (curia) → ssh → Windows OpenSSH →
cmd.exe → <Windows-native binary>` with no PowerShell or remote bash
layer to interpret quotes, single quotes around args are passed
through to the binary as part of its argv. This is the negative form
of PS-3.

```bash
# ❌ Single quotes survive cmd.exe and become part of wsl.exe's argv.
# wsl.exe sees the path as `'C:\path'` (literal quotes) and fails.
ssh "${USER}@${HOST}" "wsl.exe --import dist 'C:\Users\u\dist-fs' '...' --version 2"

# ✅ Double quotes via `\"...\"`. After bash escape they reach the wire
# as `"..."`; cmd.exe strips them per its native argv parser; wsl.exe
# receives clean paths.
ssh "${USER}@${HOST}" "wsl.exe --import dist \"C:\Users\u\dist-fs\" \"...\" --version 2"
```

Single quotes are not a transport-universal "safe" choice; they
require a layer that interprets them. Pick the quote form by the
**innermost interpreter** in the transport stack:

| Innermost interpreter         | Quote form for embedded args                                       |
|-------------------------------|--------------------------------------------------------------------|
| PowerShell                    | `'...'` (PS string literal)                                        |
| Remote bash (`bash -c "..."`) | inner `'...'` (bash strips)                                        |
| cmd.exe → native binary       | `\"...\"` (becomes literal `"..."` on the wire; cmd.exe strips)    |

### ❌ SH-10: Bash bodies are single statements

A remote-bash body (`bash -c "<body>"`) MUST be exactly one of these
three shapes — no others:

1. One simple command (with arguments, options, and redirections).
2. One pipeline (`cmd1 | cmd2 | ...`), optionally preceded by
   `set -o pipefail` to make the pipeline's exit status surface the
   first non-zero stage.
3. One simple command whose argument is itself an inner-shell body
   (e.g. `wsl.exe --user root install ...`, `sudo -n install ...`).

The enumeration is constructive. No other bash construct may appear at
the body's top level. Forbidden constructs include `if`/`then`/`else`/
`fi`, `for`/`while`/`until`, `case`, `&&`/`||` chains, `;`-separated
statements, intermediate `var=...` assignments, `trap`, function
definitions, and `(...)` subshells used to scope state. A single
`if`-guarded effect like
`if test -e '...'; then sudo -n rm '...'; fi` is also forbidden — even
though it has only one "real" effect, the body contains a decision (the
test) and an action (the consequent). Decisions belong in bash on the
curia. See "Capture-Decide-Dispatch Pattern" below for the canonical
procedure.

Exception: `set -o pipefail` is permitted INSIDE the same statement as
a pipeline (shape 2), since pipefail is a precondition of the
pipeline's exit-code semantics, not a separate operation.

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

The `< "${KEY_FILE}"` redirection on the curia attaches the file to
bash's FD 0, which the wrapper's ssh inherits, which sshd forwards to
the remote command's FD 0, which the remote bash exposes to its
single-statement body as `/dev/stdin`. The body itself remains shape 1
(one simple command with options + path argument); the file content
never appears in argv. SH-1's stdin-consumption hazard is structurally
avoided because the body is a single command that explicitly reads
`/dev/stdin` — no second command exists to be silently starved by
leftover bytes.

### ❌ PS-5: PowerShell bodies are single expressions

A PowerShell body (`powershell -Command "<body>"` or its capture variant)
MUST be exactly one of these three shapes — no others:

1. One cmdlet call (with arguments, parameters, and pipelined formatters
   like `| Out-String` or `| Out-Null`).
2. One native binary call (with arguments).
3. One native binary with an inner-shell body (e.g.
   `wsl.exe ... bash -c "..."`).

The enumeration is constructive. No other PowerShell construct may
appear at the body's top level. Forbidden constructs include `if`,
`try`/`catch`, `foreach`, `while`, `switch`, `do`, `&&`/`||` operators,
`;`-separated statements, intermediate `$var` assignments, and ternary
`?:`. A single `if`-guarded effect like
`if (Test-Path '...') { Remove-Item '...' }` is also forbidden — even
though it has only one "real" effect, the body contains a decision (the
test) and an action (the consequent). Decisions belong in bash. See
"Capture-Decide-Dispatch Pattern" below for the canonical procedure.

Exception: the wrapper-side prelude
(`$ErrorActionPreference='Stop'; $env:WSL_UTF8=1; $LASTEXITCODE=0; <body>; if ...`)
is library code written once and exercised by all callers. Caller-side
bodies follow the single-expression rule.

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

### ❌ PS-6: Don't interpolate strings via PowerShell when bash can build them

If a remote command needs a string with embedded variables (registry path,
file path, SID), the bash caller builds the string by expanding
`${bash_var}` into the body before sending. The PS body receives the
already-resolved string as a literal (PS single-quoted, or
argument-bound).

Rationale: bash escape rules cascade once. The bash → ssh → cmd.exe →
PS-literal-string chain is well-understood. Adding PS-side interpolation
(`"...$var..."`) layers another substitution model on top; subtle bugs
(like the PS-7 native-binary space-quoting trap) compound across layers
and become harder to diagnose.

```bash
# ❌ PS-side string concatenation
priv_ps "\$path='HKLM:\\...\\' + \$sid; New-Item \$path -Force | Out-Null"

# ✅ Bash builds the string; PS receives a literal
z_path="HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ProfileList\\${z_sid}"
priv_ps "New-Item -Path '${z_path}' -Force | Out-Null"
```

### ❌ PS-7: Don't interpolate variables through PowerShell to native binaries

PowerShell's argv handling for native Windows binaries (reg.exe, sc.exe,
schtasks.exe) has documented quirks with embedded spaces and quotes.
A registry path containing `Windows NT` (one space) breaks `reg.exe`
invoked through PS even when the path is held in a PS variable.

Rule: for PS-native effects (registry as PS-drive via `New-Item` /
`New-ItemProperty`, ACLs via `Get-Acl`, account ops via `Get-LocalUser`),
use PS cmdlets — they handle their own paths transparently. For
native-binary effects (reg.exe, schtasks.exe), either pass already-resolved
bash literals that don't traverse PS interpolation, or drop the PS layer
entirely and invoke through cmd.exe directly (OpenSSH-Win32's default
shell).

```bash
# ❌ Native binary with PS interpolation — argv quoting may eat path spaces
priv_ps "reg.exe add \"HKLM\\SOFTWARE\\Microsoft\\Windows NT\\...\\\$sid\" /v X /t REG_SZ /d Y /f"

# ✅ PS cmdlet handles PS-drive registry paths cleanly
priv_ps "New-ItemProperty -Path '${z_regkey}' -Name 'X' -Value 'Y' -Force | Out-Null"
```

### ❌ PS-8: Error suppression is not idempotency

When a destructive cmdlet (`Remove-Item`, `Unregister-*`, `Stop-Service`,
`Remove-LocalUser`) might fail because its target is absent, the temptation
is to reach for an error-suppression flag and call the body "idempotent":

- `-ErrorAction Ignore`
- `-ErrorAction SilentlyContinue`
- `2>$null` redirection
- `try { ... } catch { }` with empty or discarding catch

Forbidden on destructive actions. Error-suppression flags swallow **every**
error class, not just "target absent." Permission denied, file locked, ACL
refusal, path-too-long, provider-not-loaded — all silently absorbed. The
cmdlet returns success, `$LASTEXITCODE` stays 0, the wrapper's trailer
takes the no-fire branch, and bash sees a clean exit. The destructive
action might not have actually happened, and we have no way to know.

This is the "silently swallowed" failure class WSG opens with (Core
Philosophy, *Trap, don't trust*). Idempotency MUST come from explicit
state capture in bash followed by an unconditional action — see
"Capture-Decide-Dispatch Pattern" below.

Distinguish suppressing **output** from suppressing **errors**. `New-Item
... | Out-Null` discards the cmdlet's *return object* and is a routine
hygiene practice (the cmdlet still throws on actual error). `Remove-Item
... -ErrorAction Ignore` suppresses *errors*, converting "permission
denied" into apparent success. PS-8 forbids the latter on destructive
actions.

Probe-side use is permitted. `Get-LocalUser -Name '...' -ErrorAction
SilentlyContinue` on a non-destructive query, where "user not found" is
the answer we want and stdout-empty is how we read it, is exactly correct.
The carve-out: error suppression is permitted on cmdlets whose purpose is
to *return state*, forbidden on cmdlets whose purpose is to *change state*.

```bash
# ❌ Error-suppression masquerading as idempotency
priv_ps "Remove-Item -Force '${z_tar_path}' -ErrorAction Ignore"
priv_ps "Remove-Item -Recurse -Force '${z_dir}' -ErrorAction SilentlyContinue"
priv_ps "wsl.exe --unregister ${dist} 2>\$null; \$LASTEXITCODE = 0"

# ✅ CDD: capture, decide, dispatch unconditionally
z_present=$(ps_capture "Test-Path '${z_tar_path}'") || die "..."
if [[ "${z_present}" == "True" ]]; then
  priv_ps "Remove-Item -Force '${z_tar_path}'" || die "..."
fi

# ✅ Probe-side suppression — Get-LocalUser is non-destructive, "user not found" is the answer
priv_ps "Get-LocalUser -Name '${z_user}' -ErrorAction SilentlyContinue"
```

## Capture-Decide-Dispatch Pattern

Bash owns the state machine; the remote-side body is one effect. Any
decision that would otherwise live in a remote body (PS or bash)
decomposes into three steps on the curia:

1. **Capture** — run a single-expression probe and return its stdout
   into a bash variable on the curia.
   - PS bodies: `ps_capture "<single-expression probe>"` (PS-5-clean:
     cmdlet call or native binary call, no `if`, no compounds). The
     wrapper strips Windows CR; the bash variable holds clean text.
   - Bash bodies: `$(priv_bash LETTER "<single-statement probe>")`
     (SH-10-clean: one simple command with redirections, e.g.
     `stat '${z_path}' > /dev/null 2>&1`).
   - Absorb capture failure with `|| die "..."` if the probe must
     succeed, or `|| z_var=""` (PS) / `|| z_exit=$?` (bash, when probing
     by exit status) if a fundus-side absence is a legitimate pre-state.

2. **Decide** — bash conditional. For PS booleans, see "PowerShell
   boolean serialization" below. For text/list output, use
   `grep -qFx PATTERN <<<"$z_var"` for exact-line membership, or
   `case ... in ... esac` / `[[ "$z_var" == ... ]]` for value compare.
   For bash exit-status probes, `[[ ${z_exit} -eq 0 ]]`.

3. **Dispatch** — `priv_ps "<single-expression effect>"` or
   `priv_bash LETTER "<single-statement effect>"` runs the unconditional
   action only on the bash-side branches that need it. Each branch uses
   one privileged call.

### PowerShell boolean serialization

`Test-Path`, `Test-Connection`, and other PS-bool-emitting cmdlets serialize
as the literal strings `"True"` or `"False"` through `ps_capture` (after
WSL_UTF8 normalization and CR stripping). Canonical bash check:

```bash
if [[ "${z_var}" == "True" ]]; then ...
```

Do not use `[[ -n "${z_var}" ]]` — `False` is also non-empty.

### Capture failure absorption

The capture call's bash exit-status is non-zero on any of: ssh transport
failure, PS body trapping its own error under `$ErrorActionPreference=Stop`,
native binary in the body returning non-zero, or wrapper-trailer fire on
`$LASTEXITCODE`. Two absorption shapes:

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

Ready-made CDD shapes for the patterns this discipline routinely uses.
Copy and adjust the variable names; do not derive from principles each
time.

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

A fresh-WSL host with no installed distros is a legitimate pre-state, so
the list capture absorbs failure silently rather than dying.

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

`Get-LocalUser`'s `-ErrorAction SilentlyContinue` is permitted here per
PS-8's probe-side carve-out — the cmdlet is non-destructive and "user
absent" reads as empty stdout.

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
- `\$LASTEXITCODE = 0` — initialize to defeat the null trap (PS-1)
- Trailing `if ... exit ...` — propagate native command exit codes

Body authoring follows PS-2 through PS-8.

### Bash-via-wsl.exe wrapper (w-letter)

```bash
ssh ... "${USER}@${HOST}" \
    "wsl.exe --distribution ${DIST} --user root bash -c \"${z_body}\""
```

Body authoring follows SH-1 through SH-10, with SH-6's w-letter escape
table: `$name` → `\$name`; `${name}` → `\${name}`; `$(...)` unescaped.

### Bash-via-cygwin wrapper (c-letter)

```bash
ssh ... "${USER}@${HOST}" \
    "C:/cygwin64/bin/bash --login -c \"${z_body}\""
```

Body authoring follows SH-1 through SH-10. Per SH-6's c-letter row,
`$name`, `${name}`, and `$(...)` all pass unescaped — cmd.exe alone does
no shell substitution.

## Deferred

### Non-Windows (b-letter) verification

The b-letter path (direct ssh to Linux/Mac) bypasses every Windows-specific
layer (no cmd.exe DefaultShell, no wsl.exe argv substitution, no Windows
argv parser); standard BCG body discipline applies. When adding a
non-Windows target, run analog probes substituting plain bash for
wsl.exe / cygwin to confirm absence of Windows-layer quirks.

## Verification Probe Template

Every WSG rule has a probe that demonstrates the failure mode and the
fix. Standard form:

```
# Probe N: <claim>
# Pre-state: <relevant remote state>

<priv-ssh wrapper> <node> "<failing form>"
# stdout: <observed>
# exit:   <observed>

<priv-ssh wrapper> <node> "<repaired form>"
# stdout: <observed>
# exit:   <observed>
```

Probes are read-only and idempotent where possible. Probes that mutate
remote state (creating users, files) must declare cleanup.

## Acronym Registry

| Term | Expansion |
|------|-----------|
| WSG | Windows Scripting Guide (this document) |
| OpenSSH-Win32 | The Win32 port of OpenSSH bundled with Windows 10+ |
| Curia | The local invoking machine |
| Fundus | The remote target machine |

## Related Documents

- BCG — base bash discipline
