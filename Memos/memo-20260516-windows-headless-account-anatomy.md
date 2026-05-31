# Windows Headless SSH Account Anatomy

Date: 2026-05-16
Last consolidated: 2026-05-31 — promoted to the single best-memory reference
for Windows SSH access (accounts, privileged admin lever, current live state).
Prefer adding here over spawning sibling docs; a cloud of semi-healthy notes
is how this knowledge gets lost.

## Why this memo exists

When standing up a Windows account that should be reachable via SSH
but should never have an interactive Windows logon (a "headless" or
"service-style" account), the path from `net user /add` to a working
`ssh user@host` is not direct. Each link in the chain has a separate
failure mode that surfaces as the same opaque "Permission denied
(publickey)" log line. This memo captures the chain so the next
investigator does not re-derive it from registry diving and Event
Log spelunking.

The pattern is load-bearing in `bujuw_user` infrastructure
(`Tools/buk/bujb_jurisdiction.sh`) and surfaces again whenever an
operator wants a parallel test account for ad-hoc Windows SSH work.
The brad@rocket setup on 2026-05-16 was the most recent rediscovery.

## Current access state on rocket (bujn-winpc)

Verified 2026-05-31 via `tt/buw-jpS` (admin) and direct `ssh` (ad-hoc
accounts). This is the live picture for manual testing; the recipes that
produced it follow in the sections below. Tailnet host `rocket` =
`bujn-winpc`. All authentication is pubkey-only.

### Accounts present

| Account      | Reach                          | Lands in                                                                    | Role |
|--------------|--------------------------------|-----------------------------------------------------------------------------|------|
| `bhyslop`    | `tt/buw-jpS bujn-winpc <cmd>`  | Windows default shell (cmd.exe); prepend `powershell -Command` / `bash -c`  | **Privileged admin** — the lever that writes ACL-locked files |
| `brad`       | `ssh brad@rocket`              | Interactive Cygwin `bash --login -i`                                        | Human ad-hoc; forced-command ignores a passed command |
| `cygwin`     | `ssh cygwin@rocket "<cmd>"` or `ssh -t cygwin@rocket` | Cygwin bash — one-shot *or* interactive login shell                         | Dual-mode Cygwin (conditional wrapper, 2026-05-31) |
| `wsl`        | `ssh wsl@rocket "<cmd>"` or `ssh -t wsl@rocket`       | WSL Ubuntu 24.04 as **root** — one-shot *or* interactive login shell        | Dual-mode WSL; **Docker 29.1.3 daemon live** |
| `bujuw_user` | `tt/buw-jws bujn-winpc`        | Garrison-determined shell                                                   | Formal BURN/BURP workload account (garrison-managed) — not ad-hoc |

The first four (admin + brad/cygwin/wsl) are the ad-hoc manual-testing
surface. `bujuw_user` is the formal jurisdiction path and is owned by the
garrison ceremony, not by hand-edits.

### sshd posture (global, `C:\ProgramData\ssh\sshd_config`)

- `PubkeyAuthentication yes`, `PasswordAuthentication no`, `PermitEmptyPasswords no`
- `AuthorizedKeysFile .ssh/authorized_keys` (default; overridden per-account
  by the Match blocks below)
- `LogLevel DEBUG3` — verbose, a debugging carry-over; harmless, revert to
  default when convenient (not load-bearing for access)

Match-block map:

| Match                  | AuthorizedKeysFile |
|------------------------|--------------------|
| `Group administrators` | `__PROGRAMDATA__/ssh/administrators_authorized_keys` |
| `User bujuw_user`      | `__PROGRAMDATA__/ssh/users/bujuw_user/authorized_keys` |
| `User brad`            | `__PROGRAMDATA__/ssh/users/brad/authorized_keys` |
| `User cygwin`          | `__PROGRAMDATA__/ssh/users/cygwin/authorized_keys` |
| `User wsl`             | `__PROGRAMDATA__/ssh/users/wsl/authorized_keys` |

### Privileged admin account (`bhyslop`)

Reached by `tt/buw-jpS` (`bujb_privileged_ssh`, pass-through) using
`~/.ssh/id_ed25519_winpc-admin`, key-only / BatchMode. Directly observed on
2026-05-31:

- Member of `BUILTIN\Administrators` and `rocket\docker-users`; High integrity.
- Trust is a **bare** key (no forced command) in
  `C:\ProgramData\ssh\administrators_authorized_keys`:
  `ssh-ed25519 AAAA…R9aZJ bhyslop@winpc-admin`
- ACL: `BUILTIN\Administrators:(F)`, `NT AUTHORITY\SYSTEM:(F)` — the canonical
  `administrators_authorized_keys` lockdown. The principal is the admins
  *group*, not a per-user grant.
- Authenticates via the `Match Group administrators` block.
- Default remote shell is cmd.exe; since the SSH path is pass-through, prepend
  `powershell -Command …` (or `C:\cygwin64\bin\bash -lc …`) as the task needs.

**Origin note (observed, not replayed).** This account's *establishment* —
admin pubkey placement, ACL lockdown, sshd hardening — is the
caparison-windows Phase 1 ceremony's job (`BUSJCW-CaparisonWindows.adoc`). It
pre-existed this consolidation; the above is the verified current state, not a
re-derivation of the original setup steps. When the formal ceremony is rebuilt
later, that is where the authoritative admin-setup recipe belongs; this section
records what is true on the box today so manual testing has a complete picture.

## The chain, in causal order

### 1. `net.exe user /add` creates an account but no profile

```
net.exe user brad "" /add /passwordreq:no /active:yes
```

Creates a Windows account, assigns it a SID, and stops there. It
does NOT create:

- `C:\Users\<name>\` profile directory
- Any entry under `HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion\ProfileList\{SID}`

The profile is registered only on first interactive logon (or by an
explicit `userenv.dll!CreateProfile` call). SSH key authentication
alone does NOT trigger profile creation.

### 2. Windows OpenSSH resolves relative AuthorizedKeysFile via the profile

The shipped sshd_config has `AuthorizedKeysFile .ssh/authorized_keys`
— a relative path. Windows OpenSSH resolves it by calling
`GetUserProfileDirectoryW(token)`, which queries
`HKLM:\…\ProfileList\{SID}\ProfileImagePath`.

For a headless account (no ProfileList entry), this returns failure.
sshd has no home path to use; auth fails. The Event Log
(`OpenSSH/Operational`) shows only `Failed publickey for {user}`
with no further explanation. The pubkey was offered correctly and
the file at `C:\Users\<name>\.ssh\authorized_keys` may even exist
with correct content and correct ACLs — sshd never looks there
because the home-directory resolution failed before any file lookup
was attempted.

### 3. Match block with absolute AuthorizedKeysFile bypasses the profile lookup

The structural fix is a sshd_config Match block giving an absolute
path:

```
Match User <name>
    AuthorizedKeysFile __PROGRAMDATA__/ssh/users/<name>/authorized_keys
```

`__PROGRAMDATA__` is sshd's token for `C:\ProgramData`. The
absolute path means sshd never asks the OS where the home is — it
opens the file directly. This is Microsoft's documented pattern for
service-style accounts.

This is the design choice in `BUJB_sshd_match_block_text`
(`Tools/buk/bujb_jurisdiction.sh:124-125`). It is not a Cygwin or
Windows path organization preference — it is structurally required
for any account that will never have an interactive profile. The
spec text presents the Match block as a routing detail; the
underlying reason is the `GetUserProfileDirectoryW` chain.

### 4. Orphan `C:\Users\<name>` causes silent `.machinename` profile suffix

If `C:\Users\<name>` exists as a directory at the moment a Windows
session first creates the account's profile, User Profile Service
disambiguates by minting `C:\Users\<name>.<machinename>` instead.
The original directory becomes an orphan.

This bit during the brad@rocket setup: a pre-existing
`C:\Users\brad\` from an earlier failed
`%h/.ssh/authorized_keys` attempt caused brad's actual profile to
land at `C:\Users\brad.rocket\`.

Operational consequence for `bujuw_user`: garrison's destructive
cleanup must remove both `C:\Users\<workload>` and any
`C:\Users\<workload>.*` siblings before the next profile creation.
A residual orphan silently splits the workload's home for the next
garrison cycle.

### 5. Cygwin and Windows disagree on "home" for the same user

After profile creation, two different APIs return two different
home paths:

| Caller                  | Returns for brad on rocket           |
|-------------------------|--------------------------------------|
| `getpwnam("brad")`      | `/home/brad` (POSIX default)         |
| `$HOME` in shell        | `/cygdrive/c/Users/brad.rocket`      |
| `~brad` (tilde expand)  | `/home/brad`                         |

SSH tools (ssh, ssh-keygen, scp, git-over-ssh) use `getpwnam` and
land on `/home/brad`. Login bash inherits `$HOME` from the Windows
session and lands on the Windows profile path. They disagree.

Symptoms during the brad setup: `mkdir -p ~/.ssh` succeeded but
ssh-keygen with default path still failed because `~` and
ssh-keygen's path resolution diverged.

Mitigation for a target account: set `HOME=/home/<name>` in
`.bash_profile` and place `.ssh/` there. SSH tools and bash then
agree on a single home.

This concern applies to `bujuw_user` too. Worth a directed probe
during practice runs:

```
ssh -t bujuw_user@rocket 'echo $HOME; getent passwd bujuw_user | cut -d: -f6'
```

If split-HOME is observed, the workload's planted privkey at
`z_wlhome/.ssh/id_ed25519` may not be findable by `git push` (which
uses `$HOME` expansion). If login bash already auto-aligns via
`/etc/profile`, no action needed.

### 6. StrictModes ACL allowlist

sshd's strict-mode check on authkeys files allows ACL entries only
for: the user, SYSTEM, BUILTIN\Administrators. Any other principal
in the ACL causes silent refusal — even read-only entries like
`Everyone:(RX)` or `BUILTIN\Users:(RX)`.

Inherited ACLs from `C:\Users\` and `C:\ProgramData\ssh\users\`
include these by default. Explicit lockdown is required:

```
icacls C:\Path\To\authorized_keys /inheritance:r /grant:r SYSTEM:F "BUILTIN\Administrators":F <user>:R
```

Owner must be the user OR a member of Administrators. Both are
accepted.

## Worked example: stand up a parallel test account

(brad@rocket, 2026-05-16. Substitute names as needed.)

Admin SSH session on rocket:

```
net.exe user brad "" /add /passwordreq:no /active:yes
mkdir C:\ProgramData\ssh\users\brad
# Write authorized_keys content: command="<shell-launch>" <pubkey>
icacls C:\ProgramData\ssh\users\brad\authorized_keys /inheritance:r /grant:r SYSTEM:F "BUILTIN\Administrators":F brad:R
icacls C:\ProgramData\ssh\users\brad /inheritance:r /grant:r SYSTEM:F "BUILTIN\Administrators":F
# Append Match block to C:\ProgramData\ssh\sshd_config:
#   Match User brad
#       AuthorizedKeysFile __PROGRAMDATA__/ssh/users/brad/authorized_keys
sshd.exe -t -f C:\ProgramData\ssh\sshd_config   # validate
Restart-Service sshd
```

From any client whose private key matches the pubkey:

```
ssh brad@rocket
```

Inside the new session, align HOME to avoid the Cygwin split:

```
mkdir -p /home/brad/.ssh
chmod 700 /home/brad/.ssh
printf 'export HOME=/home/brad\ncd "$HOME"\n' > /home/brad/.bash_profile
```

## Substrate-routing forced-command variants (cygwin / wsl)

The brad@rocket worked example puts the account on a hardcoded interactive
Cygwin shell. The same chain (account creation,
`__PROGRAMDATA__/ssh/users/<name>/` authkeys, ACL lockdown, Match block) also
supports substrate-routing personas whose forced-command wrapper encodes which
substrate the session lands in. Two were stood up on 2026-05-17 and, after the
2026-05-31 repair + broadening, both are **dual-mode** — one-shot *and*
interactive on the same account:

- `cygwin@rocket` — `ssh cygwin@rocket "<cmd>"` runs a command in Cygwin;
  `ssh -t cygwin@rocket` drops to an interactive Cygwin login shell.
- `wsl@rocket` — `ssh wsl@rocket "<cmd>"` runs a command in WSL Ubuntu 24.04
  (as root); `ssh -t wsl@rocket` drops to an interactive WSL login shell.

The dual-mode behavior comes from a wrapper that branches on
`$SSH_ORIGINAL_COMMAND`: empty (no command supplied → interactive intent) execs
an interactive login shell; non-empty execs `bash -c` with the client command.
Real-TTY `ssh -t` interactivity to both accounts was operator-confirmed
2026-05-31. The differences from brad@rocket fall in a few places.

### Forced-command pattern

Brad's forced-command launches an interactive shell:

```
command="C:\cygwin64\bin\bash --login -i"
```

The noninteractive variants intercept the SSH-protocol-level original
command (which sshd exposes as the `SSH_ORIGINAL_COMMAND` environment
variable in the spawned process) and route it through the substrate.

**cygwin@rocket.** The intuitive *inline* form is a trap. It was deployed
through 2026-05-31 and silently mangled every non-trivial command:

```
command="C:\cygwin64\bin\bash --login -c \"$SSH_ORIGINAL_COMMAND\""   # BROKEN
```

Why it fails: Windows OpenSSH runs the forced command through the default
shell, **cmd.exe**, which does not expand `$`-style variables (it uses
`%VAR%`). So bash's `-c` argument arrives as the literal token
`$SSH_ORIGINAL_COMMAND`. The single bash then expands it *during execution*,
and the result is only word-split — never re-parsed for shell metacharacters.
Net effect: the client command runs as one bare command with whitespace-split
argv. No pipes, no `;`, no quoting, no `$var`, no redirection.
`ssh cygwin@rocket 'uname -o; whoami'` fails with `uname: unknown option --
;` because `;` is handed to uname as an argument. (Contrast wsl below, which
re-parses correctly — same symptom, opposite outcome, which is what made this
confusing.)

The fix mirrors the wsl variant: a wrapper that adds a second bash layer, so
the inner `bash -c` receives a fully-expanded, re-parseable script. Wrapper at
`C:\ProgramData\ssh\users\cygwin\runcmd.sh` (LF line endings — CRLF breaks the
shebang/exec):

```
#!/bin/bash
if [ -z "$SSH_ORIGINAL_COMMAND" ]; then
  exec /bin/bash --login -i
else
  exec /bin/bash --login -c "$SSH_ORIGINAL_COMMAND"
fi
```

The wrapper's bash is a real shell: in the command branch it expands
`"$SSH_ORIGINAL_COMMAND"` (Cygwin inherits the Windows env var sshd set on the
child) and hands the *value* to the inner login `bash -c`, which parses it
normally; in the empty branch it execs an interactive login shell. The authkeys
forced-command points cmd.exe at the wrapper via Cygwin bash, passing the script
as a POSIX path:

```
command="C:\cygwin64\bin\bash /cygdrive/c/ProgramData/ssh/users/cygwin/runcmd.sh"
```

ACL-lock the wrapper to the StrictModes allowlist **plus the account's own
read+execute** — `SYSTEM:F`, `BUILTIN\Administrators:F`, `rocket\cygwin:(RX)`
— exactly as the wsl wrapper carries `wsl:RX`. This grant is load-bearing and
easy to miss: the forced command runs as the unprivileged `cygwin` user, so
testing the wrapper *as admin* passes while the real SSH connection fails
`Permission denied` on read. **Validate through the target account, never as
admin.** Verified 2026-05-31: one-shot `ssh cygwin@rocket 'echo hi | tr a-z
A-Z'` → `HI`; interactive (no command) lands a live Cygwin `bash --login -i`.

**wsl@rocket.** Embedding the wsl.exe invocation directly in the authkeys
command= field works in principle but compounds escape layers (the inner
`$SSH_ORIGINAL_COMMAND` must survive sshd's authkeys parsing, cmd.exe's
interpretation, and wsl.exe's argument splitting). A wrapper `.cmd` file at
`C:\ProgramData\ssh\users\wsl\runcmd.cmd` isolates the runtime concern. To get
**dual-mode** (one-shot + interactive) without re-introducing that escape
graveyard inside the `.cmd`, keep the conditional on the *Linux* side: the
`.cmd` just invokes a clean script inside the distro.

Linux-side `/usr/local/bin/sshrun` (mode 755, inside the `Ubuntu-24.04` distro,
LF endings):

```
#!/bin/bash
if [ -z "$SSH_ORIGINAL_COMMAND" ]; then
  exec bash -li
else
  exec bash -lc "$SSH_ORIGINAL_COMMAND"
fi
```

Windows-side `C:\ProgramData\ssh\users\wsl\runcmd.cmd` (CRLF):

```
@echo off
set WSLENV=SSH_ORIGINAL_COMMAND/u
C:\Windows\System32\wsl.exe -d Ubuntu-24.04 -u root -- /usr/local/bin/sshrun
```

The authkeys forced-command then becomes:

```
command="C:\ProgramData\ssh\users\wsl\runcmd.cmd"
```

The wrapper does two load-bearing things:

- `set WSLENV=SSH_ORIGINAL_COMMAND/u` tells WSL to forward
  `SSH_ORIGINAL_COMMAND` into the Linux side as a Unix-style env var. By
  default WSL only shares a fixed subset of env vars across the Windows ↔ Linux
  boundary; arbitrary vars require an explicit `WSLENV` declaration. `sshrun`
  then reads it natively — no `$`-through-cmd.exe escaping at all.
- Routing the conditional through a Linux file (rather than an inline
  `bash -lc "…"` branch in the `.cmd`) keeps cmd.exe's tokenizer away from the
  nested quotes a conditional would otherwise need.

The `.cmd` wrapper is ACL-locked to the same allowlist as authkeys (`SYSTEM:F`,
`BUILTIN\Administrators:F`, plus `wsl:RX`). **Caveat:** `/usr/local/bin/sshrun`
lives inside the distro, so a distro re-import (e.g. a future garrison
re-provision) wipes it — recreate it as part of any wsl re-provisioning.
Verified 2026-05-31: one-shot `ssh wsl@rocket 'docker --version'` → `Docker
version 29.1.3`; interactive (no command) lands a live WSL `bash -li` as root.

### Per-user WSL state and provisioning without elevation

WSL distros are registered per Windows user under
`HKCU\Software\Microsoft\Windows\CurrentVersion\Lxss`. A distro
installed by user A is not visible to user B. The headless `wsl`
Windows account therefore needs its own distro registration — but
`wsl --install`, the obvious provisioning command, requires admin
elevation (it enables the WSL Windows feature, which is system-wide)
and the wsl account is not (and should not be) an admin.

`wsl --import` works around this. It registers an existing distro
tarball into the *current* user's `Lxss` registry without elevation —
because the WSL Windows feature is already enabled at the system level,
and `--import` only adds the per-user registration. The bootstrap
pattern:

1. As an admin user that already has the distro installed:
   ```
   wsl --export Ubuntu-24.04 C:\ProgramData\bootstrap\ubuntu.tar
   ```
2. Grant the headless user read on the tarball and modify on the install
   location:
   ```
   icacls C:\ProgramData\bootstrap\ubuntu.tar /grant wsl:R
   icacls C:\WSL /grant 'wsl:(OI)(CI)M'
   ```
3. Temporarily set the headless user's authkeys forced-command to a
   one-shot import:
   ```
   command="C:\Windows\System32\wsl.exe --import Ubuntu-24.04 C:\WSL\Ubuntu-24.04 C:\ProgramData\bootstrap\ubuntu.tar"
   ```
4. Trigger from the SSH client — any command fires the forced-command
   and the connection blocks until import completes:
   ```
   ssh wsl@host trigger
   ```
5. Restore the runtime forced-command (the `runcmd.cmd`-based variant
   above), re-lock its ACL, and delete the tarball.

The wsl account installs its own WSL state through its own SSH session,
because `--import` does not need the rights that `--install` does.
Principle-clean: the registration runs as the target user, no rights
modifications on the headless account, no cross-user impersonation.

### Operational notes

**ACL re-lock after .NET / PowerShell file writes.** `[IO.File]::WriteAllText`,
`[IO.File]::WriteAllBytes`, and `Set-Content` all behave the same way, and the
behavior differs by whether the target already exists:

- **New file** (e.g. a freshly created `runcmd.sh`): inherits its parent
  directory's ACL, which does *not* include the account's own read+execute.
  You must apply the full lockdown including `<user>:(RX)` — otherwise the
  forced command runs as the unprivileged account and fails `Permission
  denied` reading its own wrapper. (This is exactly the 2026-05-31 cygwin
  wrapper miss — see "Noninteractive forced-command variants" above.)
- **Existing file overwritten in place** (e.g. truncating and rewriting an
  already-locked `authorized_keys`): the existing ACL/owner is preserved, so
  no re-lock is needed.

When in doubt, the safe sequence is always: write file → (re)apply the icacls
lockdown → verify with `icacls <file>`. And remember the trap above: a wrapper
that the unprivileged account must *read* needs that account in the ACL, which
you cannot confirm by testing as admin.

**Transit content past cmd.exe / PowerShell.** Writing arbitrary file
content from a remote bash session over `ssh ... cmd.exe ... powershell
... wsl.exe` is a quoting graveyard — `&&`, `|`, `$VAR`, `\"`, and `'`
all behave differently at each layer. The reliable transit is to
base64-encode content on the curia side and decode in PowerShell:

```
B64=$(printf '%s\n' "$CONTENT" | openssl enc -base64 -A)
PS_CMD="[IO.File]::WriteAllText('C:\\path\\file', [System.Text.Encoding]::UTF8.GetString([Convert]::FromBase64String('${B64}')))"
ENCODED=$(printf '%s' "$PS_CMD" | iconv -f UTF-8 -t UTF-16LE | openssl enc -base64 -A)
ssh user@host powershell -NoProfile -EncodedCommand "$ENCODED"
```

The two base64 layers protect different segments: the inner one keeps
file content opaque to PowerShell's parser; the outer one
(`-EncodedCommand`) keeps the entire PowerShell command opaque to
cmd.exe's tokenizer. The matching code in this project's bash utilities
uses `openssl enc -base64 -A` consistently — see
`Tools/buk/bujb_jurisdiction.sh` for prior art in the same shape.

**`wsl.exe` emits UTF-16LE.** Output from `wsl -l -v`, `wsl --status`,
and `wsl --install` is UTF-16LE on stdout. Through cmd-mode SSH
transport the interleaved null bytes terminate C-string handling and
the output appears empty or garbled (single visible characters spaced
out). Force UTF-8 with `$env:WSL_UTF8=1` set before the wsl invocation,
or invoke via a PowerShell wrapper that handles UTF-16 natively.

## See also

- `Tools/buk/bujb_jurisdiction.sh:124-125` — `BUJB_sshd_match_block_text`
- `Tools/buk/bujb_jurisdiction.sh:166-168` — `BUJB_command_b/c/w` forced-command directives
- `Tools/buk/bujb_jurisdiction.sh` `bujb_privileged_ssh` — admin pass-through, surfaced as `tt/buw-jpS bujn-winpc <cmd>` (the privileged lever used for all admin-side edits in this memo)
- `Tools/buk/vov_veiled/BUSJCW-CaparisonWindows.adoc` — caparison spec
- `Tools/buk/vov_veiled/BUSJGC-GarrisonCygwin.adoc` — garrison-c spec
- `Tools/buk/vov_veiled/BUSJGW-GarrisonWsl.adoc` — garrison-w spec
- Microsoft OpenSSH for Windows StrictModes documentation
