## Goal

Windows test infrastructure for Recipe Bottle. Run RB's container isolation
tests on a Windows host with WSL, Cygwin, and dual Docker daemons.

## Current Design

The architectural backbone lives in BUS0 §Remote Node Access (`Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc`)
plus three platform subdocs (`BUSNW-NodeWindows.adoc`, `BUSNL-NodeLinux.adoc`,
`BUSNM-NodeMac.adoc`). Read BUS0 §Remote Node Access first; this paddock
captures only state that doesn't fit there.

**Header decisions made in this heat:**

- BURH regime renamed to BURN (n = Node)
- BURN_TIER field added: `privileged | workload`
- Two-tier model:
  - Privileged tier: node-level ops (sshd config, user provisioning).
    Multi-resident allowed; one operator at a time by social contract.
  - Workload tier: user-level ops (normal remote work). Exclusive to one
    station user; ephemeral.
- Verb vocabulary (BUSN quoins): Garrison / Conscript / Discharge / Inventory
- Tabtarget colophon families under `buw-`:
  - `buw-rn{l,r,v}` — BURN regime ops (list / render / validate)
  - `buw-npg{l,m,c,w,p,x}` — Garrison per platform (privileged setup)
  - `buw-nwc{l,m,c,w,p,x}` — Conscript per platform (workload mint)
  - `buw-nw{d,i}` — Discharge / Inventory
  - `buw-nw{c,r,s}` — Check / Run / Ssh (workload operational)
  - `buw-hn0` — Handbook landing for node ops
- Visual proximity note: `buw-nwc` (Check) vs `buw-nwc{platform}` (Conscript)
  — distinct colophons, accepted

## Heat Sequence (post-triage)

Spec exists. Remaining: implement, walk, practice.

Pending:

- **Implement** the BUS0 tabtarget set (BURH → BURN code rename, new
  colophon families, Garrison/Conscript/Discharge/Inventory machinery)
- **₢A-AAC** reslate to: walk access ceremony on real hardware
- **₢A-AAD** practice WSL + Cygwin install on Windows host
- **₢A-AAE** practice fundus provisioning inside WSL
- **₢A-AAF** practice Docker dual-daemon setup
- **₢A-AAP** regression-retest fundus after practice amendments
- **₢A-AAR** specify BUWC module + key builder in BUS0
- **₢A-AAQ** specify per-command subdocs (burn_install_key etc.)

Triaged out:

- ₢A-AAH (verification tabtargets) — fold into practice wrap notes
- ₢A-AAI (reconsider Cygwin install) — fold into ₢A-AAD wrap notes
- ₢A-AAJ (autonumber sweep) — relocate to ₣A6 (handbook restart)
- ₢A-AAS, ₢A-AAT (handbook-render fixture coverage) — relocate to ₣BB

## Deferred (named for future return)

- **Adopt** — cross-station-user pubkey install via existing privileged
  trust. Today: each station user runs the ceremony fresh from their
  own machine.
- **User regime** — centralized per-user pubkey for shared admin
  scenarios. Pairs naturally with Adopt when both are needed.
- **Git normalization on remote** — JJK has `jjx_plant`; BUK doesn't
  duplicate.

## Standing Notes

- Tailscale provides stable transport (Mac sees `rocket` as
  `100.x.y.z` / tailnet hostname); not adopted as a BUK dependency
- Windows username `b hyslop` (with space) — audit shell quoting at
  every `${BURN_USER}` expansion during practice
- Fundus capability registry: `Tools/rbk/vov_veiled/RBSFR-FundusRegistry.md`
- PowerShell-from-WSL pattern: bash owns control flow, PowerShell is
  the Windows syscall layer; one atomic command per call
- BURS_USER station regime field routes profiles to
  `.buk/users/${BURS_USER}/` subdirectory
- Three Windows BURN profiles for `rocket` distinguished by
  `BURN_COMMAND`: cygwin (`...bash.exe -l`), WSL (`wsl.exe -d ...`),
  PowerShell (`powershell.exe`)