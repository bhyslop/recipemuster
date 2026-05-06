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
its implementation seat (BCG-compliant module housing both verbs). Neither
the umbrella nor the implementation module earns AXLA-entity voicing in
BUS0. Architectural spine in BUS0 §Remote Node Access.

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

**BURC addition:**

- `BURC_WORKLOAD_USER` — project-wide convention name for the workload OS
  user that garrison provisions on every node. Single source of truth.

**Two verbs (Fenestrate, Garrison):**

- **Fenestrate** — Windows-OpenSSH-only `sshd_config` harden plus admin SSH
  key-trust establishment. Uniform across shell-letters (no variant).
  Two-phase ceremony forced by `Restart-Service sshd` killing its own
  session. Tabtarget `buw-jpF` (capital F = persistent config application).
- **Garrison** — workload account provisioning only. Per-shell-letter
  (b/c/w). Never reads or writes any `sshd_config`; never alters admin
  authorized_keys. First action is admin SSH via key auth; failure surfaces
  immediately.
- For Linux/Mac there is no fenestrate verb — admin trust is operator-manual
  (e.g., `ssh-copy-id`); garrison runs against an existing key-trusted admin
  foothold. For Cygwin/WSL sshd hardening on Windows nodes (in addition to
  Windows OpenSSH), hardening is operator-manual.

**Garrison-destructive model:**

- Every garrison wipes prior workload state regardless of which investiture
  or operator runs it. Node runtime state belongs to whoever last garrisoned.
- One workload user per node, ever. No conscript / discharge / withdraw /
  vanquish / inventory verbs; garrison subsumes them.
- Admin-side sshd configuration is fenestrate's scope on Windows;
  operator-manual elsewhere.

**Cross-cutting premises:**

- `burn_host_singularity` — host of a node lives only in BURN; BURP reaches
  it by reference, never duplicates.
- `bus_keys_operator_owned` — system never generates or modifies SSH key
  material; operator owns all key administration.

**Tabtarget colophons (14 total; b/c/w shell support, PowerShell deferred):**

- `buw-r[np][lrv]` — BURN/BURP regime config (List/Render/Validate) — 6
- `buw-jpF` — Fenestrate (Windows OpenSSH only; uniform) — 1
- `buw-jpG[bcw]` — Garrison per shell-letter (b=native bash for Linux/Mac;
  c=Cygwin; w=WSL) — 3
- `buw-jwk` — Knock (probe workload reachability) — 1
- `buw-jwc` — Run command file as workload (shell determined by garrison) — 1
- `buw-jws` — Interactive SSH as workload — 1
- `buw-hj0` — Handbook jurisdiction top — 1

**Operational seat (`bujb_jurisdiction.sh` + `bujb_cli.sh`):**

BCG-compliant module housing both fenestrate and garrison implementations,
hardcoding the three shell-letter→`command=` directive mappings (b/c/w)
used by garrison on the workload account, the workload privkey destination
path on remote per shell-letter, the canonical WSL distribution name
(`rbtww-main`), and the Windows OpenSSH `sshd_config` hardening directive
set used by fenestrate. SSH transport mechanics live in code; BURP carries
no shell selection.

**Fenestrate ceremony (Windows OpenSSH only; uniform):**

Two-phase remote-run because `Restart-Service sshd` terminates its own
ssh session.

Phase 1 (over the password-or-key admin SSH session):

1. SSH-as-`BURP_PRIVILEGED_USER` with
   `PreferredAuthentications=publickey,password`. Steady-state:
   `BURP_PRIVILEGED_KEY_FILE` authenticates by key. First-run: ssh prompts
   on /dev/tty; operator types the Windows admin password once. Fenestrate
   process never sees the password.
2. Idempotently place admin pubkey (derived from `BURP_PRIVILEGED_KEY_FILE`
   via `ssh-keygen -y`) in `administrators_authorized_keys` as a bare entry
   (no `command=`; fenestrate is uniform). Performed before any sshd_config
   change so key auth is available for phase 2's reconnect.
3. `icacls` lockdown on `administrators_authorized_keys`.
4. Write hardened directives to `sshd_config`: `PubkeyAuthentication yes`,
   `PasswordAuthentication no`, `PermitEmptyPasswords no`. Idempotent.
5. Verify written state via PowerShell `Get-Content` returning raw bytes,
   then bash-side BCG-compliant matching extracting effective directive
   values. Keeps PowerShell out of parsing (its error semantics are
   unreliable).
6. `sshd -t` validates the config (penultimate atomic op of phase 1; a
   malformed config aborts fenestrate before it could brick running sshd).
7. `Restart-Service sshd` (final atomic op of phase 1; ssh session
   terminates here — bash orchestrator treats the disconnect as the
   expected phase boundary).

Phase 2 (key auth only):

8. Reconnect as `BURP_PRIVILEGED_USER` with
   `PreferredAuthentications=publickey` (no password fallback). Failure
   here surfaces a brick — the operator's manual password no longer opens
   a session.
9. Verify post-state: the admin session authenticates by key alone and the
   running sshd serves the hardened configuration.

**Garrison ceremony (per shell-letter; workload-only):**

1. SSH-as-`BURP_PRIVILEGED_USER` with `BURP_PRIVILEGED_KEY_FILE`; key auth
   only (no password fallback). On Windows, presupposes fenestrate has run
   for this investiture. On Linux/Mac, admin trust is operator-managed
   (e.g., `ssh-copy-id` placed the admin pubkey).
2. Destroy existing workload user (if present) via `userdel -r` or platform
   equivalent — removes account + home directory; stray files outside home
   are workload's own concern, not garrison's.
3. Create fresh workload user named `BURC_WORKLOAD_USER` (unprivileged: no
   sudo, no admin group, ssh-only access).
4. Place workload pubkey in workload's authorized_keys with the
   shell-letter's `command=` directive.
5. Copy workload privkey to remote workload account at the path hardcoded in
   `bujb_jurisdiction.sh` per shell-letter (so workload can authenticate
   outbound to GitHub).
6. Validate round-trip — SSH as workload + exec no-op + verify exit 0.

## Heat Sequence

**Spec foundation:**
- spec-bus0-jurisdiction — BUS0 rewrite reflecting current concept
- introduce-fenestrate-narrow-garrison — split admin trust + sshd harden
  out of garrison into a uniform fenestrate verb

**Implementation:**
- implement-jurisdiction — four-field BURP, `BURC_WORKLOAD_USER`,
  `bujb_jurisdiction.sh` module, 14 tabtargets, fenestrate + garrison
  ceremonies, cleanup superseded colophons + handbook entries

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
- **Cygwin / WSL sshd hardening verb** — fenestrate covers Windows OpenSSH
  only. If c/w garrisons ever require their own sshd to be hardened
  programmatically (rather than operator-managed), that is a future
  fenestrate sibling.
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

- **Fenestrate handles first-time admin trust establishment on Windows.**
  On a fresh node, ssh's `PreferredAuthentications=publickey,password` falls
  through to a /dev/tty password prompt; operator types the Windows admin
  password once. Fenestrate then installs the admin pubkey, sets the icacls
  ACL, hardens sshd_config (`PasswordAuthentication no`), restarts sshd,
  and reconnects to verify by key. Subsequent runs use key auth
  automatically. The operator's manual Windows scope reduces to: install
  OpenSSH server, enable the service, set `PasswordAuthentication yes`
  temporarily, allow the firewall port, ensure the admin user has a known
  password.
- **Linux/Mac admin trust is operator-managed.** No fenestrate verb;
  operator places admin pubkey via `ssh-copy-id` (or equivalent), and
  garrison's first admin SSH succeeds by key alone.
- **Two-phase fenestrate is structural, not optional.** `Restart-Service
  sshd` kills its own ssh session, so any post-restart verification must
  happen over a fresh session. Bash orchestrator treats the post-restart
  disconnect as the expected phase boundary, not a failure.
- **Encrypted (passphrase-protected) privkeys not supported.** `ssh-keygen -y`
  prompts interactively; fenestrate and garrison would hang. Operator
  policy: privkeys unencrypted; station-level security covers
  privkey-at-rest protection.
- **Workload pubkey must be pre-registered with GitHub out-of-band.** Operator
  pastes the pubkey into GitHub (deploy key per repo or personal SSH key)
  before garrison runs. Garrison cannot validate this; it is a precondition.
- **WSL distribution is hardcoded to `rbtww-main`** in
  `bujb_jurisdiction.sh`. If the current default WSL distribution differs,
  garrison-WSL fails fast with a clear error.
- **Docker Desktop is the release-1 container runtime on Windows.** WSL
  shells reach the daemon via Docker Desktop's per-distro WSL integration
  toggle (Settings → Resources → WSL Integration), enabled for `rbtww-main`.
  Cygwin and Windows-side shells reach the same daemon via the default
  Docker context. No separate dockerd inside the WSL distro; only one
  daemon ever runs concurrently.
- **Some `BURP_PRIVILEGED_USER` values may contain spaces** (e.g., Windows
  admin user `b hyslop`); audit shell quoting at every variable expansion and
  path construction.
- Tailscale provides stable transport (Mac sees `rocket` as `100.x.y.z` /
  tailnet hostname); not adopted as a BUK dependency.
- PowerShell-from-WSL pattern: bash owns control flow, PowerShell is the
  Windows syscall layer; one atomic command per call.
- `command=` in authorized_keys is OpenSSH-standard (used by gitolite, GitHub,
  rsync, borg); shrinks key blast radius rather than expanding it.