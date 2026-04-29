## Goal

Windows test infrastructure for Recipe Bottle. Run RB's container isolation
tests on a Windows host with WSL, Cygwin, and dual Docker daemons.

## Current Design

The architectural backbone lives in BUS0 §Remote Node Access (`Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc`)
plus three platform subdocs (`BUSNW-NodeWindows.adoc`, `BUSNL-NodeLinux.adoc`,
`BUSNM-NodeMac.adoc`). Read BUS0 §Remote Node Access first; this paddock
captures only state that doesn't fit there.

**Header decisions made in this heat:**

- BURH regime renamed to BURN (n = Node) — decision; execution in AAV/AAW
- BURN_TIER field added: `privileged | workload`
- Two-tier model:
  - Privileged tier: node-level ops (sshd config, user provisioning).
    Multi-resident allowed; one operator at a time by social contract.
  - Workload tier: user-level ops (normal remote work). Exclusive to one
    station user; ephemeral.
- Verb vocabulary (BUSN quoins): Garrison / Conscript / Discharge / Inventory
- Tabtarget colophon families under `buw-`:
  - `buw-rn{l,r,v}` — BURN regime ops (list / render / validate)
  - `buw-rnc` — SSH config aggregator (cross-tier, all profiles)
  - `buw-npg{c,w,p}` — Garrison per Windows shell (Cygwin/WSL/PowerShell)
  - `buw-nwc{c,w,p}` — Conscript per Windows shell
  - `buw-nw{d,i}` — Discharge / Inventory (Windows-side)
  - `buw-nw{c,r,s}` — Check / Run / Ssh (workload operational)
  - `buw-hn0` — Handbook landing for node ops
- Visual proximity note: `buw-nwc` (Check) vs `buw-nwc{platform}` (Conscript)
  — distinct colophons, accepted
- Linux/Mac/Localhost garrison and conscript variants (`buw-npg{l,m,x}`,
  `buw-nwc{l,m,x}`) are not slated and not itched. Mint as new paces when
  a first non-Windows fundus target actually arrives. Legacy `buw-rhc{l,m,x}`
  (BURN-renamed in AAV) survives as transitional code; localhost remains
  exercised by AAc and AAP regression paces.
- AAQ (BUSNW elaboration) dropped: BUS0 backbone is sufficient spec; per-
  platform contracts crystallize through implementation paces and AAC
  practice. BUSNW stub (36 lines) stands as-is.

## Heat Sequence

**Implementation phase (sequenced):**

- ₢A-AAV burh-to-burn-rename-buk — mechanical rename + tier-refusal helper
  + SSH config aggregator move (`buw-HWsc` → `buw-rnc`)
- ₢A-AAW burh-to-burn-rename-jjk — mechanical, depends on AAV
- ₢A-AAX garrison-windows-machinery — three Windows shells; defines key-
  line wire format (`# BURN:<alias>` marker) inline; updates BUS0 catalog
- ₢A-AAY conscript-discharge-inventory-windows — Windows workload-tier
  mirror, depends on AAX; updates BUS0 catalog
- ₢A-AAZ workload-operational-tabtargets — Check / Run / Ssh
- ₢A-AAa node-handbook-landing-and-windows-residue — handbook restructure,
  depends on AAX and AAY

**Curia smoke (FIRST testing — runs on Mac before any Windows trip):**

- ₢A-AAc localhost-smoke-pre-windows — fundus scenario test on Mac;
  validates BURN rename + Windows-only scope didn't break localhost path

**Practice phase (Windows host):**

- ₢A-AAC practice-access-procedures — Garrison/Conscript on real Windows;
  pass criteria specified per shell variant in pace docket
- ₢A-AAD practice-environment-procedures — WSL + Cygwin install
- ₢A-AAE practice-fundus-provisioning — JJK fundus inside WSL
- ₢A-AAF practice-docker-procedures — Docker dual-daemon

**Final regression:**

- ₢A-AAP retest-fundus-after-practice — re-run localhost fundus scenario
  test after Windows practice has potentially amended code

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