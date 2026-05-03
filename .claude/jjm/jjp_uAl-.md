## Goal

Windows test infrastructure for Recipe Bottle. Run RB's container isolation
tests on a Windows host with WSL, Cygwin, and PowerShell access paths.

## Current Concept

**Jurisdiction** is the BUK feature-area for reaching out to remote nodes
deterministically. Logical-only umbrella — no entity manifestation beyond
affiliation with `buw-j*` tabtargets and the `bujb_bash.sh` BCG operational
module. Architectural spine in BUS0 §Remote Node Access.

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

**Single workload user model:**

- One workload user per node, ever. Garrison destroys+recreates it on every
  invocation — utter reset. No conscript / discharge / withdraw / vanquish /
  inventory verbs; garrison subsumes them.

**Cross-cutting premises:**

- `burn_host_singularity` — host of a node lives only in BURN; BURP reaches
  it by reference, never duplicates.
- `bus_keys_operator_owned` — system never generates or modifies SSH key
  material; operator owns all key administration.
- single-admin-at-a-time — one operator garrisons a node at a time;
  re-garrison from a different operator wipes prior state.

**Tabtarget colophons (14 total):**

- `buw-r[np][lrv]` — BURN/BURP regime config (List/Render/Validate) — 6
- `buw-jpg[bcwp]` — Garrison per shell-letter (b=native bash for Linux/Mac;
  c=Cygwin; w=WSL; p=PowerShell) — 4
- `buw-jwk` — Knock (probe workload reachability) — 1
- `buw-jwc` — Run command file as workload (shell determined by garrison) — 1
- `buw-jws` — Interactive SSH as workload — 1
- `buw-hj0` — Handbook jurisdiction top — 1

**Operational seat (`bujb_bash.sh` + `bujb_cli.sh`):**

BCG-compliant module hardcoding the four shell-letter→`command=` directive
mappings. SSH transport mechanics live in code; BURP carries no shell
selection.

**Garrison ceremony (per shell-letter):**

1. Authenticate as `BURP_PRIVILEGED_USER` (password first time, key thereafter
   via `BURP_PRIVILEGED_KEY_FILE`)
2. Idempotently place admin pubkey (derived from privkey via `ssh-keygen -y`)
   in admin's authorized_keys with the shell-letter's `command=` directive
3. Destroy existing workload user (if present) — clean slate
4. Create fresh workload user named `BURC_WORKLOAD_USER`
5. Place workload pubkey in workload's authorized_keys with `command=`
6. Copy workload privkey to remote workload account (`~/.ssh/id_*`) so workload
   can authenticate outbound to GitHub
7. Validate round-trip

## Heat Sequence

**Spec foundation:**
- spec-bus0-jurisdiction — BUS0 rewrite reflecting current concept

**Implementation:**
- implement-jurisdiction — drop BURW infrastructure, four-field BURP,
  `BURC_WORKLOAD_USER`, `bujb_bash.sh` module, 14 tabtargets, garrison
  ceremony, cleanup superseded colophons + handbook entries

**Investigation (independent, runs anytime):**
- AAe — localhost-fundus-parallel-saturation diagnosis

**Practice (Windows host, blocks on implementation):**
- AAD — WSL distro + Cygwin install
- AAE — JJK fundus inside WSL
- AAF — Docker dual-daemon

## Deferred (named for future return)

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

- Tailscale provides stable transport (Mac sees `rocket` as `100.x.y.z` /
  tailnet hostname); not adopted as a BUK dependency.
- Windows username `b hyslop` (with space) — audit shell quoting at every
  `${BURP_PRIVILEGED_USER}` expansion during garrison implementation.
- Fundus capability registry: `Tools/rbk/vov_veiled/RBSFR-FundusRegistry.md`.
- PowerShell-from-WSL pattern: bash owns control flow, PowerShell is the
  Windows syscall layer; one atomic command per call.
- `command=` in authorized_keys is OpenSSH-standard (used by gitolite, GitHub,
  rsync, borg); shrinks key blast radius rather than expanding it.