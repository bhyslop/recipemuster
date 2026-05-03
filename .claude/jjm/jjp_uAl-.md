## Goal

Windows test infrastructure for Recipe Bottle. Run RB's container isolation
tests on a Windows host with WSL, Cygwin, and PowerShell access paths.

## Current Concept

**Jurisdiction** is the BUK feature-area for reaching out to remote nodes
deterministically. Logical-only umbrella concept; `bujb_jurisdiction.sh` is
its implementation seat (BCG-compliant module). Neither the umbrella nor the
implementation module earns AXLA-entity voicing in BUS0. Architectural spine
in BUS0 §Remote Node Access.

**Two regimes (BURN, BURP):**

- BURN — node-shape (host, OS family). Per-node, git-transported, changes
  rarely. Identifier: viceroyalty.
- BURP — privileged + workload credentials. Per-station-user. Four fields:
  `BURP_NODE` (viceroyalty reference), `BURP_PRIVILEGED_USER`,
  `BURP_PRIVILEGED_KEY_FILE`, `BURP_WORKLOAD_KEY_FILE`. Pubkeys derived from
  privkeys via `ssh-keygen -y` at use-time; no separate pubkey paths or inline
  strings. Identifier: investiture.

**BURC addition:**

- `BURC_WORKLOAD_USER` — project-wide convention name for the workload OS
  user that garrison provisions on every node. Single source of truth.

**Garrison-destructive model:**

- Every garrison wipes prior workload state regardless of which investiture
  or operator runs it. Node runtime state belongs to whoever last garrisoned.
- One workload user per node, ever. No conscript / discharge / withdraw /
  vanquish / inventory verbs; garrison subsumes them.

**Cross-cutting premises:**

- `burn_host_singularity` — host of a node lives only in BURN; BURP reaches
  it by reference, never duplicates.
- `bus_keys_operator_owned` — system never generates or modifies SSH key
  material; operator owns all key administration.

**Tabtarget colophons (13 total; b/c/w shell support, PowerShell deferred):**

- `buw-r[np][lrv]` — BURN/BURP regime config (List/Render/Validate) — 6
- `buw-jpg[bcw]` — Garrison per shell-letter (b=native bash for Linux/Mac;
  c=Cygwin; w=WSL) — 3
- `buw-jwk` — Knock (probe workload reachability) — 1
- `buw-jwc` — Run command file as workload (shell determined by garrison) — 1
- `buw-jws` — Interactive SSH as workload — 1
- `buw-hj0` — Handbook jurisdiction top — 1

**Operational seat (`bujb_jurisdiction.sh` + `bujb_cli.sh`):**

BCG-compliant module hardcoding the three shell-letter→`command=` directive
mappings (b/c/w), the workload privkey destination path on remote per
shell-letter, and the canonical WSL distribution name (`rbtww-main`). SSH
transport mechanics live in code; BURP carries no shell selection.

**Garrison ceremony (per shell-letter):**

1. SSH-as-`BURP_PRIVILEGED_USER` using `BURP_PRIVILEGED_KEY_FILE` (key auth
   only — no password path; first-time bootstrap is operator-manual via
   handbook before garrison runs)
2. Idempotently place admin pubkey (derived from privkey via `ssh-keygen -y`)
   in admin's authorized_keys with the shell-letter's `command=` directive
3. Destroy existing workload user (if present) via `userdel -r` or platform
   equivalent — removes account + home directory; stray files outside home
   are workload's own concern, not garrison's
4. Create fresh workload user named `BURC_WORKLOAD_USER` (unprivileged: no
   sudo, no admin group, ssh-only access)
5. Place workload pubkey in workload's authorized_keys with `command=`
6. Copy workload privkey to remote workload account at the path hardcoded in
   `bujb_jurisdiction.sh` per shell-letter (so workload can authenticate
   outbound to GitHub)
7. Validate round-trip — SSH as workload + exec no-op + verify exit 0

## Heat Sequence

**Spec foundation:**
- spec-bus0-jurisdiction — BUS0 rewrite reflecting current concept

**Implementation:**
- implement-jurisdiction — drop BURW infrastructure, four-field BURP,
  `BURC_WORKLOAD_USER`, `bujb_jurisdiction.sh` module, 13 tabtargets,
  garrison ceremony, cleanup superseded colophons + handbook entries

**Investigation (independent, runs anytime):**
- AAe — localhost-fundus-parallel-saturation diagnosis

**Practice (Windows host, blocks on implementation):**
- AAD — WSL distro + Cygwin install
- AAE — JJK fundus inside WSL
- AAF — Docker dual-daemon

## Deferred (named for future return)

- **PowerShell garrison + dispatch** — `buw-jpgp` tabtarget + the `p`
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

## Standing Notes

- **Garrison assumes admin SSH key trust is pre-established.** First-time
  bootstrap (initial pubkey placement, password-auth disable on the node) is
  operator-manual via the handbook. Garrison uses key auth only; on key
  failure it errors out with a pointer to the bootstrap procedure.
- **Encrypted (passphrase-protected) privkeys not supported.** `ssh-keygen -y`
  prompts interactively; garrison would hang. Operator policy: privkeys
  unencrypted; station-level security covers privkey-at-rest protection.
- **Workload pubkey must be pre-registered with GitHub out-of-band.** Operator
  pastes the pubkey into GitHub (deploy key per repo or personal SSH key)
  before garrison runs. Garrison cannot validate this; it is a precondition.
- **WSL distribution is hardcoded to `rbtww-main`** in
  `bujb_jurisdiction.sh`. If the current default WSL distribution differs,
  garrison-WSL fails fast with a clear error.
- **Some `BURP_PRIVILEGED_USER` values may contain spaces** (e.g., Windows
  admin user `b hyslop`); audit shell quoting at every variable expansion and
  path construction.
- Tailscale provides stable transport (Mac sees `rocket` as `100.x.y.z` /
  tailnet hostname); not adopted as a BUK dependency.
- Fundus capability registry: `Tools/rbk/vov_veiled/RBSFR-FundusRegistry.md`.
- PowerShell-from-WSL pattern: bash owns control flow, PowerShell is the
  Windows syscall layer; one atomic command per call.
- `command=` in authorized_keys is OpenSSH-standard (used by gitolite, GitHub,
  rsync, borg); shrinks key blast radius rather than expanding it.