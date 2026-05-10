# WSG — Windows Scripting Guide

## Purpose

WSG codifies the discipline for embedding remote shell actions (PowerShell or
bash) inside ssh sessions to Windows hosts, with full error trapping at every
layer. WSG extends BCG's bash discipline into the ssh-to-Windows transport
stack — where curia bash code authors a string that traverses ssh → Windows
OpenSSH → cmd.exe → child shell → script execution.

WSG is empirically derived. Every established rule has a verification probe
that demonstrates the failure mode the rule prevents. Open questions are
flagged explicitly and resolved by separate experiment paces, not by
conjecture.

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
The set of lazy cmdlets is not universal — `Get-LocalUser` is proven
lazy, `Get-Item` is proven eager. Treat the lazy default as the safe
assumption (PS-2). String emission always survives.

**Trap, don't trust.** Each rule below assumes that any error which can
plausibly be silenced WILL be silenced if not explicitly trapped. The bias
is toward defensive instrumentation: surface every non-zero exit, every
empty stdout, every unflushed buffer, with a forensic temp file naming the
boundary that swallowed it.

**Rules enumerate; they don't illustrate.** Each rule below names a closed
set of allowed shapes. Anything not enumerated is forbidden, even if it
*looks* morally similar to an allowed form. Do not reason "this is *like*
one expression because it's *like* one effect" — that path admits a fourth
shape the rule never sanctioned, and the rationalization propagates to
every call site that copies the precedent. When in doubt, decompose into
more round-trips: the cost of an extra ssh hop is small; the cost of a
loophole is structural.

## Transport Stack

A typical chain for `zbujb_admin_powershell` or `zbujb_admin_exec`:

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

Probe pair (verification):

```
./tt/buw-jpS.PrivilegedSsh.sh <node> \
  "powershell -NoProfile -Command \"Get-LocalUser -Name 'foo'; if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE }\""
# stdout → CRLF only

./tt/buw-jpS.PrivilegedSsh.sh <node> \
  "powershell -NoProfile -Command \"\$LASTEXITCODE = 0; Get-LocalUser -Name 'foo'; if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE }\""
# stdout → full LocalUser table
```

### ❌ PS-2: `exit` mid-body before object formatters flush

PowerShell flushes string output eagerly but *some* cmdlet object outputs
lazily (Out-Default → Format-Table → render). `exit` aborts the formatter
pipeline. Lazy outputs that haven't flushed are discarded. The cmdlets
that exhibit this lazy behavior are formatter-cycle-specific: `Get-LocalUser`
from `Microsoft.PowerShell.LocalAccounts` is proven lazy (empty stdout
when followed by `exit 0`); `Get-Item` from the FileSystem provider is
proven eager (table survives `exit 0`). Treat the rule as the safe
default: assume any cmdlet output is lazy unless you've checked. See
the OQ-6 section of `Memos/memo-20260508-windows-transport-experiments.md`
for the empirical baseline.

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

This is BCG's stdin-consumption discipline (BCG line 1388) at a different
scope.

```bash
# ❌ Script via heredoc/stdin; net.exe (or any FD-0 reader) eats bytes
ssh ... "wsl.exe ... bash -s" <<SCRIPT
set -euo pipefail
net.exe user 'foo' /add ... > /dev/null
useradd 'foo'                       # may silently never run
SCRIPT

# ✅ Body as bash -c argument; bash reads body from -c, not FD 0
ssh ... "wsl.exe ... bash -c \"set -euo pipefail; net.exe ...; useradd 'foo'\""
```

Verification: cycle-1 vs cycle-3 of correct-wsl-user-model (`₢A-AAv`).
Heredoc form silently dropped useradd; args form propagated correctly,
proven by `getent passwd bujuw_user` returning the entry post-step-3.

### ❌ SH-2: Multi-line bodies through cmd.exe

Cmd.exe is line-oriented. Newlines in `"..."` quoted args are unreliable
across the cmd.exe → child-binary boundary. Use `;` as the bash command
separator and ship the body as one line.

```bash
# ❌ Newlines in body; cmd.exe may fragment
ssh ... "wsl.exe ... bash -c \"
set -e
useradd foo
\""

# ✅ Single-line body with ; separators
ssh ... "wsl.exe ... bash -c \"set -e;useradd foo\""
```

### ✅ SH-3: Outer `"..."` with `\"` for embedded double-quotes

The Windows argv parser (used by wsl.exe and most Windows binaries) honors
`\"` inside `"..."` as an escape for literal `"`. This survives cmd.exe and
reaches Linux bash as `"`. Cmd.exe itself does NOT process `\"`; it passes
the bytes to the child whose argv parser handles the escape.

```bash
# ✅ Embedded " in body
ssh ... "wsl.exe ... bash -c \"trap 'rm -f \\\"\$path\\\"' EXIT\""
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
ssh ... "powershell -NoProfile -Command \"Get-LocalUser -Name 'bujuw_user'\""
```

This is why `zbujb_admin_powershell` consistently uses single quotes for PS
string literals — they nest reliably through every layer.

### ✅ PS-4: PS-2's lazy-flush behavior is transport-agnostic

The PS-2 lazy-flush is a property of the PowerShell body, not of how
PowerShell was launched. The same `Get-LocalUser; exit 0` body emits
empty stdout via direct cmd.exe→powershell, cygwin→powershell.exe, and
wsl.exe→bash→powershell.exe. The fix (Out-String materialization, or
PS-1's `$LASTEXITCODE = 0` so the trailer doesn't fire) is also
transport-agnostic. See `Memos/memo-20260508-windows-transport-experiments.md`
§OQ-6.

### ❌ SH-4: cmd.exe does NOT process `$()` or `$var`

Verified by direct probe: `cmd.exe /c echo "SHELL=$0 X=$(uname)"` emits the
literal string `"SHELL=$0 X=$(uname)"` with no expansion. Cmd.exe's
variable syntax is `%var%` and it has no command substitution. Therefore
the outer cmd.exe layer is NOT the layer that processes shell variable
expansion or command substitution in transit.

This rule is structural; the corollary is OQ-1 (something else does eat `$`
in some chains).

### ❌ SH-5: BCG bans extend across the transport boundary

The bash code we author at the curia AND the bash code we author for remote
execution are both subject to BCG. In particular:

- **No heredocs anywhere.** The transport must not use `bash -s` with
  heredoc input (SH-1 covers the failure mode); module callers must not
  build remote bodies via heredoc.
- **No pipelines inside `$()`.** This applies to module code (e.g. the
  ill-fated `z_b64=$(printf ... | base64 | tr -d ...)` transport) and to
  bodies authored for remote execution.
- **No unguarded `$()`.** Remote bodies that use command substitution must
  guard against silent failure the same way curia code does.

The `mktemp` exception in BCG line 1473 (introspection allowlist) carries
through; remote bodies may use `$(mktemp)` for temp file creation. Top-level
pipelines (not inside `$()`) are not banned by BCG and are permitted in
remote bodies (see e.g. step-5's `openssl enc -base64 -d`).

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

Probe pair (verification, w-letter):

```
# ❌ Fails on w-letter — wsl.exe substitutes $ztmp to empty before bash runs
./tt/buw-jpS.PrivilegedSsh.sh <node> 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo $ztmp"'
# stdout: (empty)

# ✅ \$ defers expansion to bash
./tt/buw-jpS.PrivilegedSsh.sh <node> 'wsl.exe --distribution rbtww-main --user root bash -c "ztmp=HELLO; echo \$ztmp"'
# stdout: HELLO
```

See `Memos/memo-20260508-windows-transport-experiments.md` §OQ-1, §OQ-2,
§OQ-4.

### ✅ SH-7: Exit-code propagation is reliable across transports

Non-zero exit codes propagate cleanly through every tested transport:
cmd.exe direct, wsl.exe → bash, cygwin bash, PowerShell, and native
Windows binaries called via wsl.exe interop. `set -e` aborts the script
on first failure and the failing exit reaches the curia unchanged.
Bodies that need to fail-loud should use `set -e` or explicit `exit N`.

Probe (w-letter, set -e + false):

```
./tt/buw-jpS.PrivilegedSsh.sh <node> 'wsl.exe --distribution rbtww-main --user root bash -c "set -e; false; echo UNREACHED"'; echo "EXIT=$?"
# (no UNREACHED in stdout)
# EXIT=1
```

See `Memos/memo-20260508-windows-transport-experiments.md` §OQ-5.

### ✅ SH-8: Multi-line bodies via file feed (when SH-2 doesn't fit)

When a body materially exceeds what `;`-join (SH-2) can express, the
canonical pattern is two-phase: phase 1 ships the body as a remote file
via PowerShell `Set-Content` or `[System.IO.File]::WriteAllText`; phase 2
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

Default discipline remains SH-2; SH-8 applies only when bodies don't fit
one line. See `Memos/memo-20260508-windows-transport-experiments.md`
§OQ-7.

### ❌ SH-9: Single quotes are LITERAL characters in cmd.exe-direct transport

When a command line is routed `bash (curia) → ssh → Windows OpenSSH →
cmd.exe → <Windows-native binary>` with no PowerShell or remote bash
layer to interpret quotes, single quotes around args are passed
through to the binary as part of its argv. This is the negative form
of PS-3.

```bash
# ❌ Single quotes survive cmd.exe and become part of wsl.exe's argv.
# wsl.exe sees the path as `'C:\path'` (literal quotes) and fails.
ssh "$WORKLOAD@$HOST" "wsl.exe --import dist 'C:\Users\u\rbtww-fs' '...' --version 2"

# ✅ Double quotes via `\"...\"`. After bash escape they reach the wire
# as `"..."`; cmd.exe strips them per its native argv parser; wsl.exe
# receives clean paths.
ssh "$WORKLOAD@$HOST" "wsl.exe --import dist \"C:\Users\u\rbtww-fs\" \"...\" --version 2"
```

The asymmetry: PS-3 (single quotes survive nesting) applies when there
is a PowerShell layer that recognizes `'...'` as a string-literal
delimiter. SH-9 applies when there is no such layer — only cmd.exe
between the wire and the binary. Single quotes are not a
transport-universal "safe" choice; they require a layer that
interprets them.

Decision rule: pick the quote form by the **innermost interpreter** in
the transport stack:

| Innermost interpreter | Quote form for embedded args |
|-----------------------|-------------------------------|
| PowerShell            | `'...'` (PS string literal)   |
| Remote bash (`bash -c "..."`) | inner `'...'` (bash strips)  |
| cmd.exe → native binary       | `\"...\"` (becomes literal `"..."` on the wire; cmd.exe strips) |

Empirical site: `zbujb_garrison_w_init_wsl` in `bujb_jurisdiction.sh`
— the `wsl --import` invocation in garrison-w's three-namespace
redesign (BUSJGW).

### ❌ PS-5: PowerShell bodies are single expressions

A `zbujb_admin_powershell` (or `zbujb_powershell_capture`) body MUST be
exactly one of these three shapes — no others:

1. One cmdlet call (with arguments, parameters, and pipelined formatters
   like `| Out-String` or `| Out-Null`).
2. One native binary call (with arguments).
3. One native binary with an inner-shell body (e.g.
   `wsl.exe ... bash -c "..."`).

The enumeration is constructive (Core Philosophy, *Rules enumerate; they
don't illustrate*). No other PowerShell construct may appear at the body's
top level. Forbidden constructs include `if`, `try`/`catch`, `foreach`,
`while`, `switch`, `do`, `&&`/`||` operators, `;`-separated statements,
intermediate `$var` assignments, and ternary `?:`. A single `if`-guarded
effect like `if (Test-Path '...') { Remove-Item '...' }` is also forbidden
— even though it has only one "real" effect, the body contains a decision
(the test) and an action (the consequent). Decisions belong in bash. See
"Capture-Decide-Dispatch Pattern" below for the canonical procedure.

If a body needs N intermediate values or N decisions, run N round-trips:
capture state via `zbujb_powershell_capture` into bash variables, decide
in bash, dispatch unconditional effects via `zbujb_admin_powershell`. Bash
owns the state machine; the remote-side body is one effect.

Empirical baseline: every `zbujb_admin_powershell` call site in the module
follows single-expression shape — `Get-LocalUser ...`, `Test-Path ...`,
`Get-Acl ...`, `icacls ...`, `wsl.exe ... bash -c "..."`, etc. The original
violator (`bujb_jurisdiction.sh:715`, profile registration via
`$sid=...; $path=...; New-Item; New-ItemProperty`) produced a quoting
failure (reg.exe `Invalid syntax` on the `Windows NT` space) because the
compound shape interleaved PS interpolation, native-binary argv handling,
and registry-path building in one body. Decomposing into bash-orchestrated
single-expression calls eliminated the failure class. A second documented
violation cycle in
`Memos/memo-20260510-windows-transport-ps5-anti-rationalization.md` records
the `if`-as-guard rationalization that motivates the explicit
forbidden-constructs list above.

Exception: the wrapper-side prelude
(`$ErrorActionPreference='Stop'; $env:WSL_UTF8=1; $LASTEXITCODE=0; ${z_body}; if ...`)
is library code written once and exercised by all callers. Caller-side
bodies follow the single-expression rule.

```bash
# ❌ Compound state machine in PS body
zbujb_admin_powershell "\$sid=(Get-LocalUser '${z_wlu}').SID.Value; \$path='HKLM:\\...\\' + \$sid; New-Item \$path -Force | Out-Null; New-ItemProperty \$path -Name 'X' -Value 'Y' -Force | Out-Null"

# ❌ Single `if`-guard — body decides AND acts; state machine belongs in bash
zbujb_admin_powershell "if (Test-Path '${z_tar_path}') { Remove-Item -Force '${z_tar_path}' }"

# ✅ Bash-orchestrated single-expression calls
z_sid=$(zbujb_powershell_capture zbujb_privileged "(Get-LocalUser '${z_wlu}').SID.Value") || buc_die "..."
z_regkey="HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ProfileList\\${z_sid}"
zbujb_admin_powershell "New-Item -Path '${z_regkey}' -Force | Out-Null"
zbujb_admin_powershell "New-ItemProperty -Path '${z_regkey}' -Name 'X' -Value 'Y' -Force | Out-Null"

# ✅ Single guarded effect repaired via Capture-Decide-Dispatch
z_present=$(zbujb_powershell_capture zbujb_privileged "Test-Path '${z_tar_path}'") || buc_die "..."
if [[ "${z_present}" == "True" ]]; then
  zbujb_admin_powershell "Remove-Item -Force '${z_tar_path}'" || buc_die "..."
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
zbujb_admin_powershell "\$path='HKLM:\\...\\' + \$sid; New-Item \$path -Force | Out-Null"

# ✅ Bash builds the string; PS receives a literal
z_path="HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\ProfileList\\${z_sid}"
zbujb_admin_powershell "New-Item -Path '${z_path}' -Force | Out-Null"
```

### ❌ PS-7: Don't interpolate variables through PowerShell to native binaries

PowerShell's argv handling for native Windows binaries (reg.exe, sc.exe,
schtasks.exe) has documented quirks with embedded spaces and quotes.
A registry path containing `Windows NT` (one space) breaks `reg.exe`
invoked through PS even when the path is held in a PS variable — empirical
in this session's diagnostic cycle (`bujb_jurisdiction.sh:715`,
pre-decomposition).

Rule: for PS-native effects (registry as PS-drive via `New-Item` /
`New-ItemProperty`, ACLs via `Get-Acl`, account ops via `Get-LocalUser`),
use PS cmdlets — they handle their own paths transparently. For
native-binary effects (reg.exe, schtasks.exe), either pass already-resolved
bash literals that don't traverse PS interpolation, or drop the PS layer
entirely and invoke through cmd.exe directly (OpenSSH-Win32's default
shell).

```bash
# ❌ Native binary with PS interpolation — argv quoting may eat path spaces
zbujb_admin_powershell "reg.exe add \"HKLM\\SOFTWARE\\Microsoft\\Windows NT\\...\\\$sid\" /v X /t REG_SZ /d Y /f"

# ✅ PS cmdlet handles PS-drive registry paths cleanly
zbujb_admin_powershell "New-ItemProperty -Path '${z_regkey}' -Name 'X' -Value 'Y' -Force | Out-Null"
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
state capture in bash followed by a conditional unconditional action — see
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
zbujb_admin_powershell "Remove-Item -Force '${z_tar_path}' -ErrorAction Ignore"
zbujb_admin_powershell "Remove-Item -Recurse -Force '${z_dir}' -ErrorAction SilentlyContinue"
zbujb_admin_powershell "wsl.exe --unregister ${dist} 2>\$null; \$LASTEXITCODE = 0"

# ✅ CDD: capture, decide, dispatch unconditionally
z_present=$(zbujb_powershell_capture zbujb_privileged "Test-Path '${z_tar_path}'") || buc_die "..."
if [[ "${z_present}" == "True" ]]; then
  zbujb_admin_powershell "Remove-Item -Force '${z_tar_path}'" || buc_die "..."
fi

# ✅ Probe-side suppression — Get-LocalUser is non-destructive, "user not found" is the answer
zbujb_admin_powershell "Get-LocalUser -Name '${z_user}' -ErrorAction SilentlyContinue"
```

Empirical site: this rule was minted after a session in which `-ErrorAction
Ignore` was reached for to escape a PS-5 `if`-guard violation, substituting
an OQ-1 silent-swallow for the structural one. See
`Memos/memo-20260510-windows-transport-ps5-anti-rationalization.md`.

## Capture-Decide-Dispatch Pattern

PS-5's closing line — *"Bash owns the state machine; the remote-side body
is one effect"* — names the canonical procedure for any decision that
would otherwise live in a PS body. Capture-Decide-Dispatch (CDD) is a
three-step recipe:

1. **Capture** — `zbujb_powershell_capture ROLE "<single-expression probe>"`
   runs a PS-5-clean probe (cmdlet call or native binary call, no `if`,
   no compounds) and returns its stdout into a bash variable. The wrapper
   strips Windows CR (`bujb_jurisdiction.sh:487`); the bash variable holds
   clean text. Absorb capture failure with `|| buc_die "..."` if the probe
   must succeed, or `|| z_var=""` if a fundus-side absence is tolerable
   (e.g. probing for state on a host that may not yet have the relevant
   subsystem installed).

2. **Decide** — bash conditional. For PS booleans, see "PowerShell boolean
   serialization" below. For text/list output, use
   `grep -qFx PATTERN <<<"$z_var"` for exact-line membership, or
   `case ... in ... esac` / `[[ "$z_var" == ... ]]` for value compare.

3. **Dispatch** — `zbujb_admin_powershell "<single-expression effect>"`
   runs the unconditional action only on the bash-side branches that need
   it. Each branch uses one zbujb call.

Concrete shapes for common cases — file/dir presence, WSL distro
membership, local user existence, service state, registry key existence —
appear in the "Idempotency Exemplars" section below.

### PowerShell boolean serialization

`Test-Path`, `Test-Connection`, and other PS-bool-emitting cmdlets serialize
as the literal strings `"True"` or `"False"` through
`zbujb_powershell_capture` (after WSL_UTF8 normalization and CR stripping).
Canonical bash check:

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
z_present=$(zbujb_powershell_capture zbujb_privileged "Test-Path '${z_path}'") \
  || buc_die "Failed to probe ${z_path}"

# Empty result on probe failure is a valid pre-state — absorb silently
z_wsl_list=""
z_wsl_list=$(zbujb_powershell_capture zbujb_privileged "wsl.exe --list --quiet") \
  || z_wsl_list=""
```

Use the loud form by default. Use the silent form only when the absent
state is a legitimate pre-condition (e.g. WSL not yet installed during
first-run purge — the next steps install it).

### Open question: bool-returning predicate variant

A `zbujb_powershell_predicate` (BCG `_predicate` class — never dies, status
only, idiomatic in `if`/`while`) would collapse the capture+compare pair
into `if zbujb_powershell_predicate ROLE "Test-Path '...'"; then ...`. The
shape was tabled because ssh-layer transport failure adds a third value
that doesn't fit either bool answer cleanly: strict BCG predicate semantics
("never dies, status only") would conflate transport failure with "false,"
reintroducing the OQ-1 silent-swallow this section is meant to close. The
canonical CDD shape uses `zbujb_powershell_capture` + `[[ "$x" == "True" ]]`
explicitly so transport failures stay loud. The predicate variant remains
an open design question.

## Idempotency Exemplars

Ready-made CDD shapes for the patterns this codebase actually uses. Copy
and adjust the variable names; do not derive from principles each time.
Adding a new exemplar earns a paragraph here once verified.

### File presence → conditional remove

```bash
local z_present
z_present=$(zbujb_powershell_capture zbujb_privileged "Test-Path '${z_path}'") \
  || buc_die "Failed to probe ${z_path}"
if [[ "${z_present}" == "True" ]]; then
  zbujb_admin_powershell "Remove-Item -Force '${z_path}'" \
    || buc_die "Failed to remove ${z_path}"
fi
```

### Directory presence → conditional remove (recursive)

```bash
local z_present
z_present=$(zbujb_powershell_capture zbujb_privileged "Test-Path '${z_dir}'") \
  || buc_die "Failed to probe ${z_dir}"
if [[ "${z_present}" == "True" ]]; then
  zbujb_admin_powershell "Remove-Item -Recurse -Force '${z_dir}'" \
    || buc_die "Failed to remove ${z_dir}"
fi
```

### WSL distro membership → conditional unregister

A fresh-WSL host with no installed distros is a legitimate pre-state, so
the list capture absorbs failure silently rather than dying.

```bash
local z_wsl_list=""
z_wsl_list=$(zbujb_powershell_capture zbujb_privileged "wsl.exe --list --quiet") \
  || z_wsl_list=""
if grep -qFx "${z_dist}" <<<"${z_wsl_list}"; then
  zbujb_admin_powershell "wsl.exe --unregister ${z_dist}" \
    || buc_die "Failed to unregister ${z_dist}"
fi
```

### Local user existence → conditional removal

`Get-LocalUser`'s `-ErrorAction SilentlyContinue` is permitted here per
PS-8's probe-side carve-out — the cmdlet is non-destructive and "user
absent" reads as empty stdout.

```bash
local z_user_state
z_user_state=$(zbujb_powershell_capture zbujb_privileged \
  "Get-LocalUser -Name '${z_user}' -ErrorAction SilentlyContinue") \
  || buc_die "Failed to probe local user ${z_user}"
z_user_state="${z_user_state//[$'\r\n\t ']/}"
if [[ -n "${z_user_state}" ]]; then
  zbujb_admin_powershell "Remove-LocalUser -Name '${z_user}'" \
    || buc_die "Failed to remove local user ${z_user}"
fi
```

### Service running state → conditional start

```bash
local z_service_state
z_service_state=$(zbujb_powershell_capture zbujb_privileged \
  "(Get-Service -Name '${z_svc}').Status") \
  || buc_die "Failed to probe service ${z_svc}"
if [[ "${z_service_state}" == "Stopped" ]]; then
  zbujb_admin_powershell "Start-Service -Name '${z_svc}'" \
    || buc_die "Failed to start service ${z_svc}"
fi
```

### Registry key absence → conditional creation

```bash
local z_present
z_present=$(zbujb_powershell_capture zbujb_privileged "Test-Path '${z_regkey}'") \
  || buc_die "Failed to probe registry key ${z_regkey}"
if [[ "${z_present}" != "True" ]]; then
  zbujb_admin_powershell "New-Item -Path '${z_regkey}' -Force | Out-Null" \
    || buc_die "Failed to create registry key ${z_regkey}"
fi
```

## Wrapper Discipline

### PowerShell wrapper (proven shape)

Mirrors `zbujb_admin_powershell` after the LASTEXITCODE-init repair:

```bash
ssh ... "${USER}@${HOST}" \
    "powershell -NoProfile -Command \"\$ErrorActionPreference = 'Stop'; \$env:WSL_UTF8 = 1; \$LASTEXITCODE = 0; ${z_body}; if (\$LASTEXITCODE -ne 0) { exit \$LASTEXITCODE }\""
```

Discipline:
- `\$ErrorActionPreference = 'Stop'` — PS errors terminate
- `\$env:WSL_UTF8 = 1` — wsl.exe (when called from PS body) emits UTF-8
- `\$LASTEXITCODE = 0` — initialize to defeat the null trap (PS-1)
- Trailing `if ... exit ...` — propagate native command exit codes
- Body uses single quotes for PS literals (PS-3)
- Body avoids object-emitting cmdlets immediately before exit, or pipes
  them through Out-String first (PS-2)

### Bash-via-wsl.exe wrapper (proven shape, w-letter)

Mirrors `zbujb_admin_exec w` after the cycle-3 BCG-clean repair. Body is
`;`-joined for one-line transit (SH-2), `"` escaped to `\"` (SH-3), and
all `$name`/`${name}` references escaped to `\$name`/`\${name}` for
deferred bash-side expansion (SH-6). `$(...)` command substitution does
not need escape (SH-6).

```bash
ssh ... "${USER}@${HOST}" \
    "wsl.exe --distribution ${DIST} --user root bash -c \"${z_body}\""
```

Body authoring discipline:
- Statements joined with `;` (SH-2)
- Embedded `"` → `\"` (SH-3)
- `$name` → `\$name`; `${name}` → `\${name}` (SH-6)
- `$(...)` left as-is (SH-6)
- No heredocs, no `bash -s`, no inner pipe-then-bash (SH-1, SH-5)
- No newlines in body (SH-2); use SH-8 if the body genuinely won't fit

### Bash-via-cygwin wrapper (proven shape, c-letter)

Cygwin's path is simpler. cmd.exe → C:/cygwin64/bin/bash.exe. No wsl.exe,
no Windows-side `$` substitution.

```bash
ssh ... "${USER}@${HOST}" \
    "C:/cygwin64/bin/bash --login -c \"${z_body}\""
```

Body authoring discipline:
- Statements joined with `;` (SH-2)
- Embedded `"` → `\"` (SH-3)
- `$name`, `${name}`, `$(...)` all unescaped — bash sees them directly (SH-6)
- No heredocs, no `bash -s` (SH-1, SH-5)

## Deferred — out of release-1 scope

### b-letter (Linux/Mac native ssh) verification

No Linux or Mac BURN profile is registered in the release-1 matrix; only
`bujn-winpc` (`BURN_PLATFORM=bubep_windows`). The b-letter path bypasses
every Windows-specific layer (no cmd.exe DefaultShell, no wsl.exe argv
substitution, no Windows argv parser); standard BCG body discipline
applies. If a Linux/Mac BURN profile is added, run the OQ-1 probes from
`Memos/memo-20260508-windows-transport-experiments.md` (probes 1A, 1G,
1J, 1P) substituting plain bash for wsl.exe / cygwin to confirm absence
of Windows-layer quirks. See §OQ-3 of that memo.

## Empirical Record

Probe matrices, raw transcripts, and the bisection narratives that
generated the rules in this document are filed under `Memos/`. WSG cites
those memos but does not duplicate their contents.

Active references:

- `Memos/memo-20260508-windows-transport-experiments.md` — initial OQ-1
  through OQ-7 resolution matrix run against `bujn-winpc` (pace `₢A-AA0`).
- `Memos/memo-20260510-windows-transport-ps5-anti-rationalization.md` —
  PS-5 `if`-as-guard rationalization and `-ErrorAction Ignore` substitution
  cycles; motivates the "Rules enumerate" Core Philosophy clause, the PS-5
  exclusive-enumeration tightening, the new PS-8 rule, and the
  Capture-Decide-Dispatch named pattern.

### Convention for future Windows-transport experiments

When a new open question surfaces around the Windows transport stack
(wsl.exe, cygwin, cmd.exe, OpenSSH-Win32, PowerShell), file the probe
matrix and resolution at:

```
Memos/memo-YYYYMMDD-windows-transport-{topic}.md
```

Cite the memo from WSG once findings stabilize into rules. WSG retains
the distilled rules and one-line probe pairs; memos retain the full
empirical record.

## Verification Probe Template

Every WSG rule has a probe that demonstrates the failure mode and the
fix. Standard form:

```
# Probe N: <claim>
# Pre-state: <relevant remote state>

./tt/buw-jpS.PrivilegedSsh.sh <node> "<failing form>"
# stdout: <observed>
# exit:   <observed>

./tt/buw-jpS.PrivilegedSsh.sh <node> "<repaired form>"
# stdout: <observed>
# exit:   <observed>
```

Probes are read-only and idempotent where possible. Probes that mutate
remote state (creating users, files) must declare cleanup or rely on a
subsequent obliterate cycle.

## Acronym Registry

| Term | Expansion |
|------|-----------|
| WSG | Windows Scripting Guide (this document) |
| OpenSSH-Win32 | The Win32 port of OpenSSH bundled with Windows 10+ |
| Curia | The local invoking machine (per JJK terminology) |
| Fundus | The remote target machine (per JJK terminology) |
| LASTEXITCODE-null trap | The `$null -ne 0 → True` issue in PS-1 |
| Stdin-consumption | The FD-0-inherited-by-child issue in SH-1 |
| Body-as-arg | The `bash -c "<body>"` shape that avoids SH-1 |

## Related Documents

- BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) — base bash discipline
- BUS0 (`Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc`) — BUK utilities
  spec; covers temp file forensic preservation under `BURD_TEMP_DIR`
- `Tools/buk/bujb_jurisdiction.sh` — current implementation seat for
  fenestrate/garrison wrappers (`zbujb_admin_powershell`, `zbujb_admin_exec`)
