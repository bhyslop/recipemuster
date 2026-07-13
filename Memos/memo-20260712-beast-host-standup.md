# Beast host standup — as-executed

*2026-07-12. Heat ₣Bs, pace ₢BsAAf (beast-host-standup). Run one of two: this
memo is the record of what was done, in the order done, dead-ends kept with
their cause. mimic-bth-intel later replays it; the replay's corrections harden
it toward a durable home (handbook vs mews re-gestation — decided then).*

Goal: normalize `beast` — an RDP-reachable, previously unsurveyed Windows box —
into a controlled Cygwin + Docker Desktop test host. Beast supplies the
persistent-logon property rocket structurally lacks (JJSAM-mews Palisade fact:
Docker Desktop requires a live desktop session; an RDP session survives
disconnect, so the Desktop engine stays up).

## 0. Starting access posture

- Operator held a live RDP session to beast via the macOS "Windows App".
- Agent had **no path in**: no sshd on beast. Until §6, every command was
  operator-pasted into an elevated PowerShell (or Cygwin) window over RDP, with
  output pasted back.
- Beast on the tailnet as `bhyslop-asrock-beast` (100.71.105.3).

## 1. Cold survey (paste-driven)

Windows identity — **Windows 11 Pro 24H2**, build 26100.8655, 64-bit:

- Registry `ProductName` reads "Windows 10 Pro" — a known Windows 11 registry
  artifact, not the truth. `DisplayVersion 24H2` + `CurrentBuild 26100` govern.
- **Pro edition confirms the pace's premise**: real RDP server, persistent
  logon session — the thing rocket (Home, no RDP) cannot hold.

Hardware: i7-6700K (4c/8t, Skylake), 63.9 GB RAM.

Virtualization features — the surprise that reshaped the walk:

- `Microsoft-Hyper-V-*`: **Enabled** (all).
- `VirtualMachinePlatform`: **Disabled**. `Microsoft-Windows-Subsystem-Linux`:
  **Disabled**. `wsl --version` works (WSL 2.4.13.0 app is present) but zero
  distros and WSL1/2 cannot run until the features are enabled + reboot.
- So the docket's assumed "WSL2 backend" target needs a feature-enable + reboot
  step the docket did not anticipate.

Docker state:

- Docker Desktop **4.39.0** installed at `C:\Program Files\Docker\Docker`.
  Given disabled WSL features, it necessarily ran the **Hyper-V backend**
  (`DockerDesktopVM` present in Hyper-V, state Off).
- Two services: `com.docker.service` (DD's privileged helper) and `docker`
  ("Docker Engine") — the latter is **DD's own Windows-containers daemon**
  (`C:\Program Files\Docker\Docker\resources\dockerd.exe --run-service ...`),
  NOT a foreign/native install. Its `C:\ProgramData\Docker\config\daemon.json`
  pins `npipe:////./pipe/docker_engine_windows`; real Windows-container
  storage exists under `C:\ProgramData\Docker\windowsfilter`. The box had been
  switched to Windows-containers mode at some point.
- `docker version` (client 28.0.1) failed to reach `//./pipe/docker_engine` —
  nothing serves the Linux-engine pipe because the DD GUI app was not running.
- Existing install's `channelUrl` = `https://desktop-stage.docker.com/...` —
  the **stage** channel, not production. One more reason to replace rather
  than update.

OpenSSH: client FoD Installed; **Server NotPresent**; `ssh-agent` Disabled.

Cygwin: `C:\cygwin64` exists — cygcheck 3.6.0 (2025-03-18), **159 packages**.
Missing vs the theurge substrate needs: `jq`, `python3`, `gcc`, all of rust.
*[Corrected in session two: `gcc` was never a need — rocket, the proven
reference, has no gcc; rust's windows-gnu toolchain ships its own linker
(§14). And `python3` is unproven as a station-side need — see §13.]*
Reference: rocket's proven `cygwin` account has 3.6.9, **190 packages**, rustup
`stable-x86_64-pc-windows-gnu` rustc 1.95.0 (package list captured for the
top-up diff). Git-for-Windows 2.49.0 at `c:\git-for-win\`.

## 2. Dead-end resolved: the two-installer confusion

Operator observed a fresh DD download differing byte-wise from the cached
installer while "version definitions match". Forensics
(`VersionInfo` + `Get-FileHash` + `Get-AuthenticodeSignature`):

- `Downloads\Docker Desktop Installer.exe` = **4.81.0.232925**, 631,263,152
  bytes, signed Docker Inc, downloaded 2026-07-12.
- `C:\INSTALL\docker\DockerDesktopInstaller.4p39p0.exe` = **4.39.0.184744**,
  526,999,408 bytes, signed Docker Inc, cached 2025-04-03.

They differ because they are different products. The "match" impression was a
namespace mixup: **Docker Desktop 4.39.0 ships Docker Engine/CLI 28.0.1** —
`docker --version` reports the engine number, the installer reports the
Desktop number. Two version universes, one install.

## 3. Dead-end kept: OpenSSH Server FoD install fails silently

From elevated PowerShell:
`Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0`

- Stalled silently for several minutes (no output, Ctrl-C inert), then
  returned **nothing**. State remained `NotPresent`; no `sshd.exe`, no service,
  no `C:\ProgramData\ssh\sshd_config`. `C:\Windows\System32\OpenSSH\` holds
  client binaries only.
- `C:\Windows\Logs\DISM\dism.log` tail: pure session setup/teardown — **no
  error recorded at all**. The failure signature is: multi-minute stall,
  success-shaped silence, nothing installed, nothing logged.
- Cause class: the capability is a Feature-on-Demand fetched from Windows
  Update; the WU source path is broken/blocked on this box. Deliberately NOT
  diagnosed further — the MSI go-around (§5) is deterministic and supported.

## 4. Palisade findings: driving PowerShell through RDP paste

Discovered by failure while the operator was the transport:

- **Conhost paste wrap.** Pasting a line longer than the console width
  (~118 cols here) into conhost PowerShell inserts a hard newline at the wrap
  point — one command becomes two. Signature: "Missing an argument for
  parameter X" at the split point, then the orphaned tail evaluates as its own
  command. **Rule: every pasted PowerShell line stays under ~100 chars.**
- **One code block per message.** Interstitial prose between two code blocks
  got swept into one paste and executed (`Then, as a second paste...` →
  parser error). Agent-side rule, same family as the wrap rule.
- **`$LASTEXITCODE` staleness.** A PowerShell command-not-found is a
  cmdlet-level error that does NOT touch `$LASTEXITCODE`; printing it after a
  failed `& sshd.exe -t` reported a stale `0` from the previous `icacls`.
  Never treat `$LASTEXITCODE` as evidence across a command that may not have
  launched.
- **GitHub asset names are version-stamped.** Win32-OpenSSH MSI assets are
  named `OpenSSH-Win64-v10.0.0.0.msi`, not `OpenSSH-Win64.msi`; an exact-name
  filter silently matched nothing and the empty value cascaded. Verify asset
  names from the release API before scripting against them; guard empties.

## 5. Go-around: Win32-OpenSSH MSI

Verified from the curia against the GitHub API, then installed by operator
paste (all lines < 100 chars):

- Release `10.0.0.0p2-Preview` (the project's GitHub MSI channel carries
  Beta/Preview labels as its normal convention; it is Microsoft's supported
  path when the FoD/WU route is unavailable).
- Asset `OpenSSH-Win64-v10.0.0.0.msi`, **6,586,368 bytes** — downloaded to
  `C:\INSTALL\openssh\` (same caching convention as `C:\INSTALL\docker\`, so
  the mimic replay uses identical bits), byte count verified against the API
  size before install.
- `msiexec /i <msi> /qn /norestart`, then `Set-Service sshd -StartupType
  Automatic; Start-Service sshd` → **sshd Running/Automatic**.
- The MSI's default `sshd_config` carries the
  `Match Group administrators` → `administrators_authorized_keys` block
  (line 86), so the key planted in §6 is honored with zero config edits.

## 6. Admin trust: same key as rocket

Planted **before** the server existed (survived §3's dead-end unharmed, used
the moment sshd came up):

- Key: the standing `~/.ssh/id_ed25519_winpc-admin` pub
  (`ssh-ed25519 AAAA...R9aZJ bhyslop@winpc-admin`) — the same admin trust
  rocket uses; beast's `bhyslop` is in `BUILTIN\Administrators`, so the
  `administrators_authorized_keys` mechanism applies unchanged.
- `C:\ProgramData\ssh\administrators_authorized_keys` written, then ACL-locked
  to the canonical pair:
  `icacls <file> /inheritance:r /grant BUILTIN\Administrators:F /grant SYSTEM:F`
- Inbound firewall rule `OpenSSH-Server-In-TCP` (TCP 22, allow) created.
- First contact from the curia:
  `ssh -i ~/.ssh/id_ed25519_winpc-admin bhyslop@bhyslop-asrock-beast whoami`
  → `bhyslop-asrock-\bhyslop`. Default shell cmd.exe (as on rocket: prepend
  `powershell -Command` / `C:\cygwin64\bin\bash -lc` per task).

## 7. Docker Desktop 4.39 removal — GUI-bound, and it lies about finishing

**The DD installer/uninstaller shares the GUI's desktop-session requirement.**
Dispatched over ssh (`DockerDesktopInstaller.4p39p0.exe uninstall`), the process
launched into **session 0**, wrote **zero** bytes to its log after the banner,
and hung indefinitely. This is the same Palisade fact JJSAM-mews records for the
DD *engine*, now shown to extend to its *installer*. **Corollary for the mimic
replay: never script Docker Desktop lifecycle over ssh — it is console-bound.**

Second trap, and the more dangerous one: after the wedged ssh-launched
uninstaller was killed and the uninstall was re-run from the RDP console, it
logged **"No installation found"** and displayed an *empty* progress dialog that
blocked `Start-Process -Wait` until closed by hand. The natural reading — "the
uninstall failed" — is **wrong**. Ground truth after closing the dialog:

- `Get-Service *docker*` → nothing
- `C:\Program Files\Docker` → absent
- `C:\ProgramData\Docker` → absent
- `Get-VM` → no `DockerDesktopVM`

The **wedged session-0 run had in fact completed the uninstall** before it was
killed; it simply died before reporting. So the second run correctly found
nothing to do and said so confusingly. **Never trust the DD uninstaller's own
verdict — assert on services, directories, and VMs.**

Residue purge (over ssh, plain file removal — not GUI-bound): removed
`C:\ProgramData\DockerDesktop`, `%APPDATA%\Docker`, `%LOCALAPPDATA%\Docker`,
`%APPDATA%\Docker Desktop`. All four verified absent. Beast now carries zero
Docker state — the one-daemon cinch is true by construction, not by policy.

## 8. WSL2 platform enable

Over ssh, no console needed:

```
dism.exe /online /enable-feature /featurename:VirtualMachinePlatform /all /norestart
dism.exe /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /all /norestart
```

Both reported "The operation completed successfully." Verified:
`VirtualMachinePlatform`, `Microsoft-Windows-Subsystem-Linux`,
`Microsoft-Hyper-V-All` all **Enabled**; `RebootPending` = **True**.

**This retires a hypothesis worth recording**: DISM itself works perfectly over
ssh on this box. So §3's OpenSSH-Server failure was **not** a broken DISM/CBS
stack — it was specifically the **Windows Update fetch** for Feature-on-Demand
payloads. The servicing stack is healthy; only the FoD source is unreachable.
Hyper-V left enabled deliberately (WSL2 shares the same hypervisor; disabling
buys nothing and risks more).

## 9. Transport findings, agent-driving-Windows-over-ssh

Beast's default ssh shell is **cmd.exe** (as on rocket). Two rules earned here:

- **cmd.exe eats `|` inside quoted PowerShell.** A `powershell -Command "... -match \"a|b\" ..."` invocation over ssh died with
  `'VirtualMachinePlatform' is not recognized as an internal or external
  command` — cmd tokenized the regex alternation as a shell pipe before
  PowerShell ever saw the string. Remedy is the one already codified in
  `memo-20260516-windows-headless-account-anatomy.md`: base64 the PowerShell
  and use `-EncodedCommand`.
  (`printf '%s' "$PS" | iconv -f UTF-8 -t UTF-16LE | openssl enc -base64 -A`)
- **Unquoted paths with spaces.** `$env:APPDATA\Docker Desktop` passed bare to
  `Remove-Item` binds `Desktop` as a stray positional arg and rejects the whole
  call. Build path lists as a quoted array.
- `-EncodedCommand` output arrives with a `#< CLIXML` progress preamble/postamble
  on stderr — cosmetic, ignore it.

## 10. Remaining walk (planned at this point)

1. **Reboot** — `RebootPending` is True; the two features land on restart.
2. Operator logs back in over RDP — that logon session is load-bearing (DD
   dies without a desktop session; disconnected-RDP suffices).
3. Install DD **4.81.0** from the fresh installer, **at the console** (§7: DD
   install is GUI/session-bound, never scriptable over ssh): WSL2 backend, skip
   sign-in, autostart on login. **No user Ubuntu distro** — DD provisions its
   own `docker-desktop` distro; the target path is Cygwin → Windows named pipe,
   and a user distro just invites a second daemon (the cinch forbids it).
4. Cygwin: in-place upgrade + top-up via `setup-x86_64.exe` against rocket's
   proven package set; rustup `x86_64-pc-windows-gnu` per the three gotchas in
   `memo-20260517-windows-substrate-landscape-for-theurge.md`.
5. Clone repo + station-files skeleton; prove a credless suite green over
   `ssh bhyslop@bhyslop-asrock-beast` from Cygwin. (Two membranes already
   landed make this a proven configuration, not a hope:
   `memo-20260603-windows-docker-desktop-bind-mount.md`.)

## 11. Post-reboot verification (session two begins here)

Operator rebooted beast between sessions. First contact from the curia, all
over ssh, no console needed:

- `whoami` → `bhyslop-asrock-\bhyslop` — **sshd came up on its own** after
  reboot (StartupType Automatic from §5 held).
- `VirtualMachinePlatform` → **Enabled**;
  `Microsoft-Windows-Subsystem-Linux` → **Enabled** (dism per-feature query,
  post-reboot).
- `wsl --status` → `Default Version: 2`, no missing-feature complaint — the
  WSL2 platform is live. Zero distros, zero Docker state, as designed.

Beast is exactly the substrate step 3 of §10 wants: live WSL2 platform, no
container runtime, admin ssh trust standing.

## 12. Docker Desktop 4.81.0 install — console, as executed

Operator, at the RDP console (per §7's corollary: DD lifecycle is never
scripted over ssh):

- Copied `Downloads\Docker Desktop Installer.exe` →
  `C:\INSTALL\docker\20260712_DockerDesktopInstaller.4p81p0pp232925.exe`
  (the §5 caching convention: date-stamped, version in the p-encoded suffix;
  the 4.39 installer renamed to `OLD_DockerDesktopInstaller.4p39p0.exe`) and
  ran the installer **from the cached copy**, not Downloads.
- Configuration page: **accepted defaults** — "Use WSL 2 instead of Hyper-V"
  checked, installed for all users. UAC: yes.
- Installer finished with **"Close and log out"**; operator logged back in.
  DD auto-launched on that logon and the onboarding was dismissed with no
  sign-in.

Verified over ssh afterward:

- Installer bytes at rest: 631,263,152 — byte-identical to the §2 forensic of
  the 4.81.0 download.
- `docker version` through the default `desktop-linux` context: **Server
  Docker Desktop 4.81.0 (232925), Engine 29.6.1 linux/amd64** — the Linux
  engine serves over the Windows named pipe with no console help.
- `wsl -l -v`: exactly one distro, `docker-desktop`, Running, version 2 — the
  no-user-distro cinch holds by construction.
- `com.docker.service`: Stopped (on-demand privileged helper; normal). All DD
  processes in the operator's session (SI 3), per the Palisade fact.
- `%APPDATA%\Docker\settings-store.json`: **`AutoStart: true`** (survives
  future logons by setting, not luck), `DisplayedOnboarding: true`,
  `LicenseTermsVersion: 2` (terms accepted).
- Substrate note for the record: DD 4.81 defaults to the **containerd
  snapshotter** image store (`UseContainerdSnapshotter: true`) — a difference
  from rocket's native WSL dockerd (classic overlayfs store). Not adjudicated
  here; noted so a future image-behavior delta has its cause on file.

## 13. Cygwin upgrade + top-up — unattended over ssh

Reference captured live from both boxes (`cygcheck -cd`): rocket 189 packages
at 3.6.9, beast 159 at 3.6.0. Diff: beast lacked `binutils`, `cygutils-extra`,
`desktop-file-utils`, `jq`, the whole `python39` stack, and ~30 dependency
libs; beast carried harmless interactive extras (vim, zsh, screen, gnupg2…)
rocket lacks.

**Package policy:** the missing *top-level* names (34) were passed explicitly;
all `lib*` names were left to dependency resolution — rocket's lib list is a
frozen snapshot and stale version-suffixed lib names are exactly what an
unattended `-P` chokes on.

**Python finding (operator asked "are we sure we need python?"):** station-side
python3 is **not load-bearing** in RB today. Every `python3` reference in
`Tools/` is (a) cloud-step *content* executed in Google's gcloud builder
container, (b) in-container execution (`rbob_ifrit_sortie` runs the adjutant
inside the bottle via `exec`), or (c) theurge's *static scan* of `.py` files —
no station interpreter invoked. `jq` IS station-load-bearing (41 bash files).
Python installed anyway to match the proven reference; **the mimic replay is
the natural place to prove the minimal set by omitting it.**

As executed:

- Fresh `setup-x86_64.exe` fetched on the curia (1,574,328 bytes), staged as
  `C:\INSTALL\cygwin\20260712_setup-x86_64.exe`. Found in that directory: the
  original 2025-03 setup binary and a `cygwin.mirrors.hoobly.com` package
  cache — beast's Cygwin was born from this same convention.
- Run over ssh (no console): PowerShell `Start-Process -Wait -PassThru` via
  `-EncodedCommand` (§9 transport rules), arguments
  `-q --upgrade-also --no-desktop --no-shortcuts --no-startmenu
  --site https://mirrors.kernel.org/sourceware/cygwin/
  --local-package-dir C:\INSTALL\cygwin --root C:\cygwin64 -P <34 names>`.
  Exit code 0. **Cygwin setup is not GUI-bound the way DD is** — quiet mode
  completes headless over ssh.
- Result: cygwin **3.6.9** (byte-matches rocket's), 159 → **224 packages**
  (rocket's 189 + beast's pre-existing extras), `jq 1.8.1`,
  `python3 3.9.16` both resolve and run from a login shell.
- Cygwin HOME on beast resolves to the Windows profile
  (`/cygdrive/c/Users/bhyslop`) — same shape as rocket's `cygwin` account
  (`/cygdrive/c/Users/cygwin`).

## 14. rustup — windows-gnu, per the three gotchas

Cached bits for the replay: `C:\INSTALL\rust\20260712_rustup-init.sh`
(29,250 bytes, from sh.rustup.rs) and `20260712_rustup-drive.sh`, a 14-line
driver encoding the three gotchas of
`memo-20260517-windows-substrate-landscape-for-theurge.md`:
no MinGW/gcc; `TMPDIR="$(cygpath -w ~/rust-install-tmp)"`; never pass
`--default-host` (the sh wrapper injects it).

- Ran `bash -lc "sh …/20260712_rustup-drive.sh"` over ssh:
  `-y --default-toolchain stable --profile minimal --no-modify-path`.
- Installer line `setting default host triple to x86_64-pc-windows-gnu`
  confirmed the auto-detect. Landed **stable 1.97.0** (rocket has 1.95.0 —
  version skew accepted; stable-current is the policy).
- **Surprise, FLAGGED for end-of-pace discussion:** rustup warned of a
  **pre-existing `C:\Users\bhyslop\.rustup\settings.toml`** — residue of some
  prior rust attempt on this box (no toolchains present, only the settings
  file). Post-state exactly matches intent (windows-gnu host, minimal profile,
  single fresh toolchain), so the residue happened to be harmless — but the
  operator's standing concern applies: **no un-versioned files silently
  influencing builds.** Disposition to be decided at pace end; a fresh mimic
  will not see this warning.
- `.bash_profile` created (none existed), mirroring rocket's verbatim rationale
  comment: rustup wires only the Windows user PATH, which sshd-spawned login
  shells don't inherit, so the profile prepends
  `/cygdrive/c/Users/bhyslop/.cargo/bin`. Verified: fresh `bash -lc` resolves
  `cargo`/`rustc` 1.97.0.

## 15. Curia ssh config, clone, station skeleton — and the engine reached

- **Curia `~/.ssh/config` entry added** (hand-managed, outside the BURH block,
  cerebro's style): `Host beast` → `bhyslop-asrock-beast`, user `bhyslop`,
  the winpc-admin key. `ssh beast` and `scp … beast:…` round-trip verified —
  operator can now move files with plain scp/cat tricks.
- Repo cloned on beast with **Cygwin git over https** (rocket's exact remote,
  `https://github.com/bhyslop/recipemuster.git`) to
  `~/projects/rbm_alpha_recipemuster`; curia's session-one/-two commits were
  unpushed, so the clone landed stale — memo edits notched, curia pushed,
  beast pulled to current main. Sibling skeleton laid per rocket:
  `station-files/secrets/{assay,client_secrets,director,governor,payor}`
  (empty — secret *content* is operator-moved, never agent-read),
  `logs-buk`, `output-buk`, `temp-buk`.
- `burs.env` written from the launcher's own SETUP-NEEDED diagnostic (three
  non-secret variables): `BURS_LOG_DIR=../logs-buk`, `BURS_USER=bhyslop`,
  `BURS_TINCTURE=bhb` (following the curia's `bhm` bh+machine shape; distinct
  tincture keeps per-station upstream resources disjoint). The diagnostic
  failure behaved exactly as designed — `rbw-tn` named the missing file and
  its required content, then ran green on the next attempt.
- **Done-when clause one, proven:** from a Cygwin login shell over
  `ssh beast`, `docker version` reaches the Docker Desktop engine — Server
  29.6.1 over the Windows named pipe. No DOCKER_HOST, no context surgery —
  DD's system-PATH CLI resolves in the Cygwin login shell as-is.

## 16. DefaultShell → Cygwin bash: beast ssh goes Linux-shaped

Operator asked for `ssh beast` to behave like ssh to macOS/Linux. Rocket
answers this per-account with the dual-mode `runcmd.sh` forced-command wrapper
(memo-20260516) because rocket must keep cmd.exe for its other personas — and
that wrapper carries a documented wart: one-shot `$var` never expands
(memo-20260517). Beast has one account and no such constraint, so it takes the
mechanism rocket couldn't:

- Registry `HKLM:\SOFTWARE\OpenSSH` → `DefaultShell =
  C:\cygwin64\bin\bash.exe`, `DefaultShellCommandOption = "-lc"` (set over
  ssh, effective per-connection, **no sshd restart needed**; delete the two
  values to revert to cmd.exe).
- Startup-file split, because the two entry modes read different files:
  sshd's interactive spawn is a non-login tty shell (reads only `~/.bashrc`);
  `-lc` one-shots are login shells (read only `~/.bash_profile`). The cargo
  PATH export moved to `.bashrc`; `.bash_profile` now just sources `.bashrc`.
- Verified matrix over fresh connections: `$OSTYPE` expands (**the rocket-
  wrapper wart is absent** — no cmd.exe layer exists to eat `$`), pipes and
  `;` parse, `cargo`/`jq`/`docker` all resolve from a plain one-shot, scp
  round-trips (sftp subsystem bypasses the shell), stdin cat-tricks work.
  Cygwin `whoami` now answers `bhyslop` (was Windows `bhyslop-asrock-\…`).
- **Gap the matrix missed, caught by the operator's first real login:**
  interactive `ssh beast` landed in a shell where `pwd`/`cd` worked but
  `ls` was `command not found`. Mechanism: every matrix probe was a
  one-shot, which `-lc` makes a *login* shell (`/etc/profile` runs and
  prepends the Cygwin dirs); the interactive spawn is **non-login** —
  `/etc/profile` never runs, and `C:\cygwin64\bin` is not on beast's
  Windows system PATH, so externals vanish while builtins survive. Fix in
  `.bashrc` (the one file the non-login interactive shell reads): prepend
  `/usr/local/bin:/usr/bin` idempotently, ahead of the cargo prepend (also
  now guarded). Verified by simulating the sshd condition — stripped PATH +
  forced-interactive `/usr/bin/bash -i` — where `ls` resolves to
  `/usr/bin/ls`; one-shots unchanged. Replay note: test BOTH entry modes;
  a green one-shot matrix says nothing about the interactive spawn.
- Cameo during that probe: with PATH stripped to System32, bare `bash`
  resolved to the **WSL launcher stub** (`C:\WINDOWS\system32\bash.exe`,
  which errored `execvpe(/bin/bash)` against the docker-desktop distro) —
  the very collision memo-20260517 recorded as the May theurge blocker,
  reproduced in one line on beast.
- **Transport consequence for everything after this section:** the §9 cmd.exe
  rules (pipe-eating, `-EncodedCommand` armor) are HISTORICAL for beast — they
  applied while cmd.exe was the default shell and apply still on rocket.
  Driving PowerShell from here on is plain `powershell -Command '…'` inside
  bash quoting. A mimic replay that wants the Linux-shaped endpoint should
  apply this section early and skip the §9 contortions for the rest.
- Curia `~/.ssh/config` gained `Host beast` (§15), so the operator's muscle
  memory is literally `ssh beast`.

## 17. First reveille on beast: one real platform bug, found and fixed

First run (`./tt/rbw-ts.TestSuite.reveille.sh` over plain `ssh beast`):
**98 passed, 1 failed** — `rbtdrq_secret_shapes` (pyx fixture) reported 4
secret-shape violations, and the suite's fixture-grain break-on-failure
discarded the trailing fixtures (10 of 13 ran).

The 4 violations were exactly the two entries of the scanner's own exemption
table (`rbtdrq_pyx.rs` self-exempt ×2 hits, the synthetic
`fdkyclk-asserter-key.pem` ×2 hits). Mechanism: `ZRBTDRQ_EXEMPT` holds
repo-canonical **forward-slash exact paths**, while the native-Windows walker
builds `rel` from `strip_prefix().to_string_lossy()` — **backslash**
separators — so the exact-match exemption never fires. Green on macOS and
rocket-WSL because their walkers emit `/`; beast's cygwin path runs theurge
as a *native Windows binary*, the first platform where `std::path` speaks
backslash. The finding lines' mixed separators (`Tools/rbk\rbtd\…`) were the
tell.

Fix (one site): normalize `rel` to forward slashes at construction
(`.replace('\\', "/")`), repairing the exemption match, the inventory trace,
and the finding-line rendering together. pyx then 4/4 on macOS (no
regression; the fourth case, `readme_anchors`, was the one fail-fast had
been discarding on beast). Fix notched and pushed; beast re-pulled.

**Second organ, same disease.** Reveille run two got past pyx (108 passed)
and stopped at conformance `rbtdrn_curl_containment`: 18 violations, exactly
the files under the two `ZRBTDRN_CURL_EXEMPT_PREFIXES` entries
(`vov_veiled/ABANDONED-github/`, `rbgj*` cloud steps) — forward-slash
prefixes, backslash `rel`, `starts_with` never fires. Rather than discover
the class one ~20-minute suite run at a time, a crate-wide grep of every
`strip_prefix` rel-construction found three sibling sites —
`rbtdrn_conformance.rs:222` (vocabulary-scan reporting), `:513` (the live
failure), `rbtdru_bash.rs:703` (cupel finding paths) — all normalized
identically. Reveille's two not-yet-reached fixtures (chaining-fact-band,
touchstone) audited clean: display-only formatting and a filename-prefix
check, nothing separator-sensitive. conformance 9/9, cupel 3/3, pyx 4/4 on
macOS; pushed; beast re-run follows.

*Standup lesson, corrected by the operator: beast is NOT the first
native-Windows theurge host — rocket's cygwin account has run the same
`x86_64-pc-windows-gnu` binary since May. The bug class was invisible for a
different reason: the affected scanners all postdate the last native-Windows
fast-tier run. Timeline: cygwin@rocket's only fast-suite attempt was
2026-05-17 (₢BOAAM) and died at case 1-of-75 on the System32-bash-lookup
blocker; the conformance module landed June 8, its curl-containment case
July 4, the pyx scanner July 9. The June cygwin work on rocket was
container-tier, which never walks the repo tree. So beast's reveille was the
first time ANY tree-scanner met backslash paths.*

**Homogenization audit (operator-directed).** With the wound open, the crate
was audited for heterogeneous path handling. Two legitimate idioms coexist,
split on principle: **component-based compares** (cupel's walker excludes by
directory basename; `zrbtdru_is_gcb` walks `path.components()`) are
inherently separator-portable — which is exactly why cupel passed on beast
while its siblings failed — and **repo-canonical rel strings** wherever a
path meets a string table (exemption lists, PathPrefix keeps, finding
reports). The inline fix had left the same normalization expression at four
sites; it now lives once as `rbtdrx_repo_rel` in `rbtdrx_platform.rs` (the
module that owns cross-platform path transmutation), with all four scanners
calling it and three unit tests pinning strip/non-child/backslash behavior.
`rbtdrx_platform`'s own two `.replace('\\',"/")` sites are a different
concern (absolute native→posix conversion) and were correctly left alone.

Third reveille on beast (inline-fix tree): **13 fixtures, 145 passed, 0
failed, 0 skipped** — count-identical to rocket-WSL's verdict. A final run
on the refactored tree is the done-when proof.

## 18. Verdict, and the standing residue

**Definitive reveille on the final (helper-refactored) tree: 13 fixtures,
145 passed, 0 failed, 0 skipped — count-identical to the rocket-WSL
rebaseline verdict.** All three done-when clauses met: Cygwin ssh as
`bhyslop` reaches the DD engine (§15), the credless suite runs green over
that path, and this memo stands.

Tariff observation for when bounds harden: beast breaches four Linux-seeded
advisory maxes (regime-validation 57s vs 30s, regime-smoke 169s vs 60s,
dockerfile-hygiene 27s vs 20s, chaining-fact-band 26s vs 20s) — advisory
only, no verdict effect. Cygwin's fork-heavy bash on a 2015-era 4-core runs
the tabtarget-dense fixtures ~2-3× the declared ceilings; the declarations
were seeded exact from a Linux host and carry no cygwin allowance yet.

Secrets: operator piped `station-files/secrets` from the curia over
`ssh beast` (tar pipe; agent never read content). macOS bsdtar emitted 15
AppleDouble `._*` companions (9 files + 6 dirs — the LIBARCHIVE.xattr pax
warnings were the benign half of the same mechanism); deleted on beast,
file count then 9 = curia's 9. Replay note: `COPYFILE_DISABLE=1` on the
macOS side suppresses both.

**Un-versioned residue flagged for disposition (operator's standing
concern: nothing outside revision control may influence builds).** Found
this session: the pre-existing `~/.rustup/settings.toml` (§14 — harmless
this time by luck of agreement). Standing un-versioned state beast now
carries, listed for the record: `~/.rustup` + `~/.cargo` (toolchain,
rustup-managed), `~/.bashrc` / `~/.bash_profile` (§16, agent-authored),
`station-files/` (secrets + burs.env, deliberately out-of-repo),
`C:\INSTALL\*` caches (replay bits), and the DD
`settings-store.json` + registry `DefaultShell` pair. Disposition is an
operator conversation, not a pace act; the mimic replay will re-create all
of it from this memo, which is itself a versioned account of the
un-versioned state.
