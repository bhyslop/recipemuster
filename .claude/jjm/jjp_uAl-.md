## Goal

Windows test infrastructure for Recipe Bottle. Run RB's container isolation
tests on a Windows host with WSL, Cygwin, and PowerShell access paths.
Container runtime for release-1 is Docker Desktop for Windows only; the
WSL distro `rbtww-main` consumes Docker Desktop's per-distro WSL
integration rather than running its own dockerd. Native dockerd inside
WSL is named in §Deferred for the shared-kernel-iptables reason.

## Current Concept

**Jurisdiction** is the BUK feature-area for reaching out to remote nodes
deterministically. Logical-only umbrella concept; `bujb_jurisdiction.sh` is
its implementation seat (BCG-compliant module housing all three verbs).
Neither the umbrella nor the implementation module earns AXLA-entity voicing
in BUS0. Architectural spine in BUS0 §Remote Node Access.

**Two regimes (BURN, BURP):**

- BURN — node-shape (host, OS family). Per-node, git-transported, changes
  rarely. Identifier: investiture.
- BURP — privileged + workload credentials. Per-station-user. Three fields:
  `BURP_PRIVILEGED_USER`, `BURP_PRIVILEGED_KEY_FILE`,
  `BURP_WORKLOAD_KEY_FILE`. Pubkeys derived from privkeys via
  `ssh-keygen -y` at use-time; no separate pubkey paths or inline strings.
  Identifier: investiture — by construction equal to a registered
  investiture. The BURP directory name must match a BURN profile dir;
  the 1:1 correspondence is enforced by file-presence check at load time.

**BUJB constant:**

- `BUJB_workload_user` — project-wide convention name for the workload OS
  user that garrison provisions on every node. Single source of truth.
  Lives in `bujb_jurisdiction.sh` as a tinder constant.

**Three verbs (Caparison, Invigilate, Garrison):**

- **Caparison** — admin host posture establishment, per platform
  (Windows/macOS/Linux). Nuclear: every invocation purges and re-establishes
  the caparison-managed posture from scratch — no skip-if-present logic.
  Operator handbook owns security-sensitive edits (UAC-required registry
  keys, Tailscale interactive auth, etc.); caparison owns idempotent
  admin-shell operations.
- **Invigilate** — read-only host posture verification, per platform. Pure
  precondition assertions; no mutations. Each posture fact has a read
  command, expected value, and failure pointer to either the operator
  handbook (for operator-managed posture) or caparison (for
  caparison-managed posture). Single source of truth for "correctly-configured"
  per platform. Used twice: caparison's post-completion check, garrison's
  precondition check.
- **Garrison** — workload account provisioning only. Per shell-letter
  (b/c/w). First step is invigilate-{platform}; failure surfaces immediately.
  Never reads or writes any `sshd_config`; never alters admin
  authorized_keys.

**Garrison-destructive model:**

- Every garrison wipes prior workload state regardless of which investiture
  or operator runs it. Node runtime state belongs to whoever last garrisoned.
- One workload user per node, ever. No conscript / discharge / withdraw /
  vanquish / inventory verbs; garrison subsumes them.
- Admin-side host posture is caparison's scope; the security-sensitive
  remainder is the per-platform handbook's scope.

**Cross-cutting premises:**

- `burn_host_singularity` — host of a node lives only in BURN; BURP reaches
  it by reference, never duplicates.
- `bus_keys_operator_owned` — system never generates or modifies SSH key
  material; operator owns all key administration.

**Tabtarget colophons (20 total; b/c/w shell support, PowerShell deferred):**

Invigilate has no tabtarget — it is invoked only internally (caparison
post-completion, garrison precondition via bujp_preflight). The
`bujb_invigilate_{windows,macos,linux}` functions exist; no standalone
operator invocation.

- `buw-r[np][lrv]` — BURN/BURP regime config (List/Render/Validate) — 6
- `buw-jpC[WML]` — Caparison per platform (W=Windows; M=macOS; L=Linux) — 3
- `buw-jpG[bcw]` — Garrison per shell-letter (b=native bash for Linux/Mac;
  c=Cygwin; w=WSL) — 3
- `buw-jpS` — Privileged SSH (admin pass-through, all platforms) — 1
- `buw-jwk` — Knock (probe workload reachability) — 1
- `buw-jwc` — Run command file as workload (shell determined by garrison) — 1
- `buw-jws` — Interactive SSH as workload — 1
- `buw-hj0` — Handbook jurisdiction top (dispatches to per-platform) — 1
- `buw-hj[WML]` — Handbook per platform — 3

**Operational seat (`bujb_jurisdiction.sh` + `bujb_cli.sh`):**

BCG-compliant module housing caparison, invigilate, and garrison
implementations, hardcoding the three shell-letter→`command=` directive
mappings (b/c/w) used by garrison on the workload account, the workload
privkey destination path on remote per shell-letter, the canonical WSL
distribution name (`rbtww-main`), and the Windows OpenSSH `sshd_config`
hardening directive set used by caparison-windows. SSH transport mechanics
live in code; BURP carries no shell selection.

**Caparison-Windows ceremony (admin posture; Windows OpenSSH host):**

Three-phase: SSH-trust hardening (former fenestrate scope), then post-trust
admin posture, then post-completion verification.

Phase 1 (over the password-or-key admin SSH session):

1. SSH-as-`BURP_PRIVILEGED_USER` with
   `PreferredAuthentications=publickey,password`. Steady-state:
   `BURP_PRIVILEGED_KEY_FILE` authenticates by key. First-run: ssh prompts
   on /dev/tty; operator types the Windows admin password once. Caparison
   process never sees the password.
2. Idempotently place admin pubkey (derived from `BURP_PRIVILEGED_KEY_FILE`
   via `ssh-keygen -y`) in `administrators_authorized_keys` as a bare entry
   (no `command=`; uniform across shell-letters). Performed before any
   sshd_config change so key auth is available for phase 2's reconnect.
3. `icacls` lockdown on `administrators_authorized_keys`.
4. Write hardened directives to `sshd_config`: `PubkeyAuthentication yes`,
   `PasswordAuthentication no`, `PermitEmptyPasswords no`. Idempotent.
5. Verify written state via PowerShell `Get-Content` returning raw bytes,
   then bash-side BCG-compliant matching extracting effective directive
   values.
6. `sshd -t` validates the config (penultimate atomic op of phase 1; a
   malformed config aborts caparison before it could brick running sshd).
7. `Restart-Service sshd` (final atomic op of phase 1; ssh session
   terminates here — bash orchestrator treats the disconnect as the
   expected phase boundary).

Phase 2 (key auth only):

8. Reconnect as `BURP_PRIVILEGED_USER` with
   `PreferredAuthentications=publickey` (no password fallback). Failure
   here surfaces a brick — the operator's manual password no longer opens
   a session.
9. Verify post-restart admin session authenticates by key alone.

Phase 3 (post-trust admin posture):

10. Stage `rbtww-main` WSL distribution under admin user via `wsl --import`
    from Ubuntu-24.04 seed. Always purge-and-reimport; no skip-if-present.
11. Apply powercfg sleep/hibernate disable (idempotent).
12. Set Tailscale service StartupType Automatic; start service
    (idempotent).
13. Run invigilate-windows post-completion check; non-zero indicates
    caparison's operations did not take or operator-managed posture is
    wrong.

**Caparison-macOS ceremony (admin posture; macOS host):**

Compact: enable Remote Login (`systemsetup -setremotelogin on`), orchestrate
ssh-copy-id, apply `pmset` power config, ensure Tailscale service
auto-start. Closes with invigilate-macos post-check.

**Caparison-Linux ceremony (admin posture; Linux host):**

Compact: orchestrate ssh-copy-id, mask sleep/suspend/hibernate targets via
systemctl (laptop/desktop hosts only), enable tailscaled. Closes with
invigilate-linux post-check.

**Invigilate ceremony (per platform; read-only):**

Each invigilate-{platform} reads its posture fact list and asserts each
fact matches expected. Failure surfaces a diagnostic naming the wrong fact
and pointing to either the operator handbook (for operator-managed
posture) or caparison (for caparison-managed posture). No mutations, no
remediation. Invocation contexts: caparison's post-completion check,
garrison's precondition check.

**Garrison ceremony (per shell-letter; workload-only):**

1. Invigilate-{platform} precondition check; failure surfaces a missing or
   stale admin posture before any destructive workload work.
2. SSH-as-`BURP_PRIVILEGED_USER` with `BURP_PRIVILEGED_KEY_FILE`; key auth
   only (no password fallback).
3. Destroy existing workload user (if present) via `userdel -r` or
   platform equivalent — removes account + home directory; stray files
   outside home are workload's own concern, not garrison's.
4. Create fresh workload user named `BUJB_workload_user` (unprivileged: no
   sudo, no admin group, ssh-only access).
5. Place workload pubkey in workload's authorized_keys with the
   shell-letter's `command=` directive.
6. Copy workload privkey to remote workload account at the path hardcoded
   in `bujb_jurisdiction.sh` per shell-letter (so workload can
   authenticate outbound to GitHub).
7. Validate round-trip — SSH as workload + exec no-op + verify exit 0.

## Heat Sequence

**Spec foundation:**
- spec-bus0-jurisdiction — BUS0 rewrite reflecting current concept
- introduce-fenestrate-narrow-garrison — split admin trust + sshd harden
  out of garrison into a uniform fenestrate verb (since broadened to
  caparison)

**Implementation:**
- implement-jurisdiction — four-field BURP, `BURC_WORKLOAD_USER`,
  `bujb_jurisdiction.sh` module, fenestrate + garrison ceremonies, cleanup
  superseded colophons + handbook entries

**Caparison/Invigilate restructuring:**
- BUS0 vocabulary + skeleton stubs (BUSJPF→BUSJCW; new BUSJC[ML],
  BUSJI[WML], BUSJH[WML])
- Per-platform caparison and invigilate spec content (Windows depth first,
  then macOS/Linux)
- Bash alignment via comment annotations (no symbol renames; preserves
  grep-archaeology for the upcoming spec/implementation review cycles)

**Investigation (independent, runs anytime):**
- localhost-fundus-parallel-saturation diagnosis

**Practice (Windows host, blocks on implementation):**
- WSL distribution install (`rbtww-main` set as default)
- Garrison-WSL exercise
- Garrison-Cygwin exercise
- Docker Desktop WSL integration enable + verify (no native dockerd)

## Deferred (named for future return)

- **PowerShell garrison + dispatch** — `buw-jpGp` tabtarget + the `p`
  shell-letter mapping in `bujb_jurisdiction.sh`. Current heat ships b/c/w
  only.
- **Adopt** — cross-station-user pubkey install via existing privileged trust.
  Today: each station user runs the ceremony fresh from their own machine.
- **User regime** — centralized per-user pubkey for shared admin scenarios.
- **Separate workload→GitHub keypair** — current shape reuses workload SSH
  keypair for outbound; tighten to per-purpose separation if security demands.
- **BURP override fields for non-conventional key layouts** — operators with
  centralized SSH key storage, non-`.pub`-suffix layouts.
- **Localhost garrison/dispatch variants** — deferred; remote nodes only.
- **Git normalization on remote** — JJK has `jjx_plant`; BUK doesn't duplicate.
- **Cygwin / WSL inner sshd hardening** — caparison covers Windows OpenSSH
  only. If c/w garrisons ever require their own sshd to be hardened
  programmatically (rather than operator-managed), that is a future
  caparison phase or sibling verb.
- **Native dockerd inside `rbtww-main`** — Docker Desktop and a per-distro
  native daemon cannot safely coexist on the same Windows host: WSL2 distros
  share one network namespace, so two daemons fight for kernel iptables/NAT
  rules. Release-1 ships Docker Desktop only; Desktop's WSL integration
  serves the rbtww-main shells. If isolation needs ever require a native
  daemon, it would be operator-managed exclusive lifecycle (one daemon at
  a time, never both running concurrently).
- **JJK fundus user provisioning under WSL** — fundus is a JJK concept and
  currently deferred; the Windows handbook does not include a fundus phase.

## Standing Notes

- **Heat deferred by design — converge against the known host first.** This
  heat is shaped to land first-time Windows setup against a *known* box —
  `bujn-winpc` (tailnet `rocket`) — before generalizing to fresh specimens.
  Standing up a brand-new node does not strictly require the old host (the node
  model is self-contained: BURN profile + local provisioning + Tailscale
  self-auth), but the convergence design prefers proving the ceremony against
  the known box first; it stays stabled until the operator elects to converge
  it — against rocket, or a deliberately-adopted new specimen. Orientation map
  for the whole Windows remote-node facility (spine, bring-up sequence, entry
  points, the two live handbook lineages, residue-already-culled):
  `Memos/memo-20260604-windows-node-orientation.md`.

- **Caparison-Windows handles first-time admin trust establishment plus the
  broader admin posture (WSL distribution staging, sleep/hibernate disable,
  Tailscale service auto-start).** SSH-trust phases retain the prior
  fenestrate two-phase structure (password fallback on /dev/tty for first
  run; key auth thereafter; sshd restart kills the phase-1 session).
  Operator-manual Windows scope: install OpenSSH server, enable the
  service, set `PasswordAuthentication yes` temporarily, allow the firewall
  port, ensure the admin user has a known password, apply
  security-sensitive registry edits per BUSJHW
  (DevicePasswordLessBuildVersion etc.), install Tailscale + Run-Unattended
  + first auth, configure netplwiz auto-login.
- **Linux/Mac admin trust enters caparison via ssh-copy-id orchestration.**
  Caparison-{linux,macos} wraps the operator-manual ssh-copy-id flow plus
  per-platform host posture (sleep masking, Tailscale service). The
  operator still does the actual ssh-copy-id one-time setup (or equivalent
  pubkey-placement); caparison verifies and applies the rest.
- **Two-phase SSH-trust within caparison-windows is structural, not
  optional.** `Restart-Service sshd` kills its own ssh session, so any
  post-restart verification must happen over a fresh session. Bash
  orchestrator treats the post-restart disconnect as the expected phase
  boundary, not a failure. The post-trust admin posture phases (WSL stage,
  powercfg, Tailscale service) run after the phase-2 reconnect.
- **Encrypted (passphrase-protected) privkeys not supported.** `ssh-keygen -y`
  prompts interactively; caparison and garrison would hang. Operator
  policy: privkeys unencrypted; station-level security covers
  privkey-at-rest protection.
- **Workload pubkey must be pre-registered with GitHub out-of-band.**
  Operator pastes the pubkey into GitHub (deploy key per repo or personal
  SSH key) before garrison runs. Garrison cannot validate this; it is a
  precondition.
- **WSL distribution is hardcoded to `rbtww-main`** in
  `bujb_jurisdiction.sh`. Caparison-Windows stages it; garrison-WSL
  exports it as a seed for the workload's HKCU\\Lxss. Invigilate-windows
  asserts its presence as a posture fact.
- **Docker Desktop is the release-1 container runtime on Windows.** WSL
  shells reach the daemon via Docker Desktop's per-distro WSL integration
  toggle (Settings → Resources → WSL Integration), enabled for `rbtww-main`.
  Cygwin and Windows-side shells reach the same daemon via the default
  Docker context. No separate dockerd inside the WSL distro; only one
  daemon ever runs concurrently.
- **Some `BURP_PRIVILEGED_USER` values may contain spaces** (e.g., Windows
  admin user `b hyslop`); audit shell quoting at every variable expansion
  and path construction.
- Tailscale provides stable transport (Mac sees `rocket` as `100.x.y.z` /
  tailnet hostname); not adopted as a BUK dependency.
- PowerShell-from-WSL pattern: bash owns control flow, PowerShell is the
  Windows syscall layer; one atomic command per call.
- `command=` in authorized_keys is OpenSSH-standard (used by gitolite,
  GitHub, rsync, borg); shrinks key blast radius rather than expanding it.
- **Mac Remote Login enabled is an operator prerequisite for any Mac BURN
  target** (including the operator's own machine). The first admin SSH
  must reach the host before caparison can do anything; only the operator
  can flip Remote Login on a fresh machine.
- **Linux nodes require `openssh-server` installed and `sshd` enabled** —
  operator-managed, not in caparison's scope.
- **Sudo NOPASSWD for `BURP_PRIVILEGED_USER` is an operator-managed
  precondition** wherever garrison needs sudo elevation. On macOS,
  admin-group membership for `BURP_PRIVILEGED_USER` is also operator-
  managed (equivalent in posture to the ssh-copy-id prerequisite).
  Blanket `NOPASSWD: ALL` acceptable on personal workstations; scoped
  is preferred for shared hosts.
- **Workload shell is `/bin/bash` on every platform** regardless of
  platform default. macOS ships /bin/bash 3.2 at the canonical path —
  bash is the BUK execution substrate everywhere.
- **Headless-account anatomy memo gates mounting of remaining rough paces.**
  `Memos/memo-20260516-windows-headless-account-anatomy.md` captures the
  chain from `net user /add` through SSH-reachable (profile registration,
  GetUserProfileDirectoryW, Match-block-as-load-bearing, orphan
  `C:\Users\<name>` causing `.machinename` suffix, Cygwin split-HOME,
  StrictModes ACL allowlist). Required reading at mount-time for the
  rough paces that follow. Two specific items surfaced 2026-05-16 that
  do not have explicit paces yet: (a) `BUJB_command_w` drops workload
  stdout via UTF-16LE because `$env:WSL_UTF8=1` is not set in the
  forced-command directive — diagnose+repair fits naturally inside ABG's
  convergence scope; (b) garrison obliterate may need wildcard cleanup
  of `C:\Users\bujuw_user.*` orphans to prevent silent profile-suffix
  mints on the next cycle — fits naturally inside ABJ's obliterate
  reshaping. ABK and AAx carry explicit memo citations in their dockets.
- **Remote theurge fundus hosts must expose `~/.cargo/bin` to non-interactive
  ssh.** Otherwise `tt/rbw-ts.*` run over `ssh <host> "…"` dies at
  `cargo: command not found` before any suite builds — rustup's PATH line sits
  after the non-interactive guard in stock `~/.bashrc`. Check
  `ssh <host> "command -v cargo"`; if empty, prepend `. "$HOME/.cargo/env"` to
  that host's `~/.bashrc` above the interactive-shell guard. Mechanism, the
  per-host fix, and the cerebro application (2026-06-05, done) are in
  `Memos/memo-20260605-cerebro-noninteractive-cargo-path.md`. cygwin and WSL
  were already provisioned; audit any new remote host here when provisioning it.