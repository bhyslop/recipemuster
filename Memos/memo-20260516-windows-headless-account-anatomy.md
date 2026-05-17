# Windows Headless SSH Account Anatomy

Date: 2026-05-16

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

## Noninteractive forced-command variants

The brad@rocket worked example above puts the account on an interactive
Cygwin shell — useful for ad-hoc human work, less useful for programmatic
access where each SSH connection should run a single command and exit.
The same chain (account creation, `__PROGRAMDATA__/ssh/users/<name>/`
authkeys, ACL lockdown, Match block) also supports noninteractive
substrate-routing personas where the forced-command field encodes which
substrate the session lands in. Two variants were stood up alongside
brad@rocket on 2026-05-17 and verified:

- `cygwin@rocket` — `ssh cygwin@rocket "uname -o"` → `Cygwin`
- `wsl@rocket` — `ssh wsl@rocket "uname -o"` → `GNU/Linux`

The differences from brad@rocket fall in three places.

### Forced-command pattern

Brad's forced-command launches an interactive shell:

```
command="C:\cygwin64\bin\bash --login -i"
```

The noninteractive variants intercept the SSH-protocol-level original
command (which sshd exposes as the `SSH_ORIGINAL_COMMAND` environment
variable in the spawned process) and route it through the substrate.

**cygwin@rocket.** The forced-command launches a non-interactive Cygwin
login shell and feeds it `$SSH_ORIGINAL_COMMAND` via `-c`:

```
command="C:\cygwin64\bin\bash --login -c \"$SSH_ORIGINAL_COMMAND\""
```

The `\"` is sshd's authkeys-string escape for `"`; after parsing, the
inner command is `bash --login -c "$SSH_ORIGINAL_COMMAND"`. When sshd
spawns the forced-command process, the `SSH_ORIGINAL_COMMAND` env var
is set on the child. Bash expands the variable inside the `-c` script
body and executes whatever the client supplied.

**wsl@rocket.** Embedding the wsl.exe invocation directly in the
authkeys command= field works in principle but compounds escape layers
(the inner `$SSH_ORIGINAL_COMMAND` must survive sshd's authkeys parsing,
cmd.exe's interpretation, and wsl.exe's argument splitting). A wrapper
`.cmd` file at `C:\ProgramData\ssh\users\wsl\runcmd.cmd` isolates the
runtime concern:

```
@echo off
set WSLENV=SSH_ORIGINAL_COMMAND/u
C:\Windows\System32\wsl.exe -d Ubuntu-24.04 -u root -- bash -lc "$SSH_ORIGINAL_COMMAND"
```

The authkeys forced-command then becomes:

```
command="C:\ProgramData\ssh\users\wsl\runcmd.cmd"
```

The wrapper does two load-bearing things:

- `set WSLENV=SSH_ORIGINAL_COMMAND/u` tells WSL to forward
  `SSH_ORIGINAL_COMMAND` into the Linux side as a Unix-style env var.
  By default WSL only shares a fixed subset of env vars across the
  Windows ↔ Linux boundary; arbitrary vars require an explicit `WSLENV`
  declaration.
- The `"$SSH_ORIGINAL_COMMAND"` inside the inner `bash -lc` argument is
  literal text at the Windows-side cmd.exe layer (cmd.exe does not
  expand `$`-style variables) and gets expanded later by the WSL bash,
  which sees the value WSLENV forwarded.

The wrapper file is ACL-locked to the same allowlist as authkeys
(`SYSTEM:F`, `BUILTIN\Administrators:F`, plus `wsl:RX`).

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

**ACL re-lock after `[IO.File]::WriteAllText`.** Writing a fresh file
via the .NET method resets the file's ACL to inherited from its parent
directory, undoing the `icacls /inheritance:r` lockdown established in
section 6 above. Re-apply the icacls lockdown after every rewrite of
authkeys (or the wrapper `.cmd`, or any file living under the locked
ProgramData subtree). `Set-Content` has the same behavior. The sequence
is: rewrite file → reapply ACL.

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
- `Tools/buk/vov_veiled/BUSJCW-CaparisonWindows.adoc` — caparison spec
- `Tools/buk/vov_veiled/BUSJGC-GarrisonCygwin.adoc` — garrison-c spec
- `Tools/buk/vov_veiled/BUSJGW-GarrisonWsl.adoc` — garrison-w spec
- Microsoft OpenSSH for Windows StrictModes documentation
