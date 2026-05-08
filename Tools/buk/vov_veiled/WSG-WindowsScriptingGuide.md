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

**Object output formatters are lazy.** PowerShell formatters flush at the
end of script execution, not eagerly. Calling `exit` mid-body discards
unformatted output. String emission (Write-Host, Write-Output of strings) is
eager and survives `exit`; cmdlet object output (Get-LocalUser, Get-Item,
etc.) is lazy and gets eaten.

**Trap, don't trust.** Each rule below assumes that any error which can
plausibly be silenced WILL be silenced if not explicitly trapped. The bias
is toward defensive instrumentation: surface every non-zero exit, every
empty stdout, every unflushed buffer, with a forensic temp file naming the
boundary that swallowed it.

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

PowerShell flushes string output eagerly but cmdlet object output lazily
(Out-Default → Format-Table → render). `exit` aborts the formatter
pipeline. Lazy outputs that haven't flushed are discarded.

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

### Bash-via-wsl.exe wrapper (open — see OQ-1, OQ-4)

The body-as-arg form (no heredoc, no base64, no inner bash-from-stdin) is
the right structural shape (SH-1). Body should be `;`-joined for one-line
transit (SH-2) and have `"` escaped to `\"` (SH-3). Escape rules for `$`
and `$()` in transit are not yet pinned down; do not author new wrappers
of this shape until OQ-1 and OQ-4 land.

## Open Questions (to be resolved by experiment paces)

These rules are not yet established. The experiment pace
**₢A-AAY-windows-transport-experiments** populates them empirically.

### OQ-1: What eats `$()` and `$var` in `wsl.exe ... bash -c "<body>"`?

Probe evidence:
- Plain `bash -c "ztmp=$(mktemp); echo $ztmp"` → ztmp is empty; something
  eats `$(mktemp)` and `$ztmp`
- Escaped `bash -c "ztmp=\$(mktemp); echo \$ztmp"` → works; bash gets
  `$(mktemp)` and `$ztmp` after escape strip

Cmd.exe is proven NOT the eater (SH-4). The eater lives between cmd.exe
and Linux bash — candidates: wsl.exe argv parser, Windows command-line
tokenizer interaction with parens, or a side-effect of how OpenSSH-Win32
spawns cmd.exe. Resolving this determines the canonical escape rule
(probably involves `\$` for body-side variables and command substitutions).

### OQ-2: Does cygwin bash (c-letter) exhibit the same `$`-eating?

Untested. The c-letter path is cmd.exe → C:/cygwin64/bin/bash.exe (no
wsl.exe intermediate). Likely behavior differs and may admit a simpler
escape rule.

### OQ-3: Does Linux bash on a non-Windows ssh target (b-letter) exhibit any of these?

Untested. Likely no — no cmd.exe in path, no Windows argv parser, just
remote bash directly. But verify; the ssh-to-Linux path may have its own
quirks.

### OQ-4: Canonical escape for body-side `$var` and `$(...)` (depends on OQ-1)

If OQ-1 settles on a specific layer eating `$`, the canonical escape may
be `\$` (which empirically survives), or a different mechanism. The rule
that goes here governs how `zbujb_admin_exec` callers write bodies that
need remote-side variable expansion or command substitution.

### OQ-5: Native exit-code propagation through wsl.exe

When a command inside `wsl.exe ... bash -c "set -e; failing_cmd"` fails,
does the non-zero exit propagate reliably back through wsl.exe → cmd.exe
→ ssh? Empirical evidence suggests YES (net.exe failures propagated
correctly in cycle-3). The experiment matrix should confirm across
nested-shell variants and across set-e vs explicit-exit forms.

### OQ-6: Object-output flush semantics through cygwin bash and wsl.exe

PS-2 was proven for `powershell -Command` directly via cmd.exe. Same
semantics through other transports (powershell launched from cygwin bash,
powershell launched from inside WSL via wsl.exe interop) untested.

### OQ-7: Multi-line bodies via process substitution / file feed

SH-2 mandates `;`-join for cmd.exe transit. Whether process substitution
(`bash <(echo body)`) or remote-side file-feed (write to remote temp file
via separate scp, then `bash <file>`) cleanly avoid the cmd.exe newline
issue is open. May enable larger bodies; tests temp-file discipline
boundaries for remote-side artifacts.

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
