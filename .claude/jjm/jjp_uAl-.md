## Goal

Windows test infrastructure for Recipe Bottle. Run RB's container isolation
tests on a Windows host with WSL, Cygwin, and dual Docker daemons.

## Current Concept

Architectural backbone in BUS0 (`Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc`).
Read BUS0 §Remote Node Access first; this paddock captures only state that
doesn't fit there.

**Three regimes (BURN/BURP/BURW):**

- BURN — node-shape (host, OS family, valid workload-template list).
  Per-node, git-transported, changes rarely. Identifier: viceroyalty.
- BURP — privileged credentials. Per-station-user, operator-authored.
  Identifier: investiture. References a viceroyalty via `burp_node`.
- BURW — workload state. Per-station-user, system-written by conscript.
  Identifier: lieutenancy. References a viceroyalty via `burw_node`.

**Cross-cutting premises:**

- `burn_host_singularity` — host of a node lives only in BURN; BURP and BURW
  reach it by reference, never duplicate.
- `bus_keys_operator_owned` — system never generates or modifies SSH key
  material; operator owns all key administration.
- single-admin-at-a-time — one privileged admin per node at a time;
  sequential transitions, not concurrent residence.

**Verb vocabulary:**

- Privileged tier: garrison, withdraw, conscript-{platform}, discharge,
  vanquish, inventory
- Workload tier: knock, remote_run, interactive_session

**Tabtarget colophons under `buw-`:**

- `buw-rn{l,r,v}` — BURN regime ops; analogous `buw-rp/rw{l,r,v}` for BURP/BURW
- `buw-np*` — privileged-tier verbs (np = uses privileged authority)
- `buw-nw*` — workload-tier verbs (nw = uses workload identity)
- `buw-hn0` — handbook landing for node ops

**BURS additions:**

- `BURS_KEY_DIR` — operator-managed SSH key directory (full path, no tilde)
- `BURS_WORKLOAD_STATE_DIR` — where BURW state files live (per-station-user)

## Heat Sequence

**Spec finalization (gates implementation):**

- reshape-spec-finalize — finish BUS0 verb table, cross-cutting principles
  section, subdoc consolidation

**Cleanup (depends on spec finalize):**

- AAf (scope-expanded) — delete deprecated BURN fields, residue prose,
  retired subdocs, legacy buhw handbook tabtargets, legacy
  `buw-rhc{l,m,x}` profile-minting if confirmed unused

**Investigation (independent, runs anytime):**

- AAe — localhost-fundus-parallel-saturation diagnosis

**Practice phase (Windows host) — blocked on yet-to-mint implementation paces:**

- AAD — WSL distro + Cygwin install
- AAE — JJK fundus inside WSL
- AAF — Docker dual-daemon

**Implementation paces:** TBD. Minted fresh after spec finalize + cleanup land.
Will cover garrison, withdraw, conscript-{platform}, discharge, vanquish,
BURS field additions, BURP/BURW regime tabtargets.

## Deferred (named for future return)

- **Adopt** — cross-station-user pubkey install via existing privileged trust.
  Today: each station user runs the ceremony fresh from their own machine.
- **User regime** — centralized per-user pubkey for shared admin scenarios.
  Pairs naturally with Adopt when both are needed.
- **Git normalization on remote** — JJK has `jjx_plant`; BUK doesn't duplicate.

## Standing Notes

- Tailscale provides stable transport (Mac sees `rocket` as
  `100.x.y.z` / tailnet hostname); not adopted as a BUK dependency
- Windows username `b hyslop` (with space) — audit shell quoting at
  every `${BURP_USER}` expansion during practice
- Fundus capability registry: `Tools/rbk/vov_veiled/RBSFR-FundusRegistry.md`
- PowerShell-from-WSL pattern: bash owns control flow, PowerShell is the
  Windows syscall layer; one atomic command per call
- BURS_USER station regime field routes BURP files to
  `.buk/users/${BURS_USER}/` subdirectory; BURN files are shared
  (project-tracked); BURW state lives at `${BURS_WORKLOAD_STATE_DIR}`