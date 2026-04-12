## Context

Windows test infrastructure for Recipe Bottle. Goal: run RB's container isolation tests on a Windows host with WSL, Cygwin, and dual Docker daemons. The setup is largely manual (OpenSSH config, Windows Settings, installer GUIs), so it needs handbook procedures — colored `buh_*` combinator output guiding a human through each step.

Raw draft captured in `Memos/memo-20260412-windows-handbook-draft.md` (9 procedures from an earlier agent session, uses placeholder names and wrong prefix `RBWH`).

## Design Decisions

**Prefix**: `RBHW` (child of `rbh` handbook family). Draft used `RBWH` which conflicts with `rbw` (workbench terminal exclusivity).

**Handbook colophon family** (`rbw-h`):

The `rbh` family has three real groups: onboarding, payor, windows. Speculative slots (governor, director, retriever) dropped — those are walkthrough steps within onboarding, not standalone groups.

| Colophon | Frontispiece | Role |
|----------|-------------|------|
| `rbw-h0` | HandbookTOP | Top-level index (all groups) |
| `rbw-ho` | HandbookOnboarding | Onboarding group top |
| `rbw-hp` | HandbookPayor | Payor group top |
| `rbw-hw` | HandbookWindows | Windows group top |
| `rbw-HO*` | — | Onboarding subordinate procedures |
| `rbw-HP*` | — | Payor subordinate procedures |
| `rbw-HW*` | — | Windows subordinate procedures |

`0` sentinel sorts first in `ls tt/rbw-h*`, group tops (lowercase) sort next, subordinates (uppercase) follow. Natural reading order.

**Three-kit split** — procedures divided by mechanism vs policy:

| Kit | Scope | Tabtargets |
|-----|-------|------------|
| BUK (`buw-HW*`) | Generic Windows OS mechanisms | 6: AccessBase, AccessRemote, AccessEntrypoints, EnvironmentWSL, EnvironmentCygwin, top-level |
| JJK (existing `jjw-tfP1/P2`) | Fundus user provisioning (`jjfu_*` accounts) | 0 new — existing P1/P2 already work inside WSL |
| RBK (`rbw-HW*`) | Project topology + Docker policy | 4: DockerDesktop, DockerWSLNative, DockerContextDiscipline, orchestrator |

**Key insight**: JJK's `jjfp_fundus.sh` already handles Linux account provisioning (create `jjfu_*` users, install SSH keys, clone repos). It runs unchanged inside a WSL distro. The draft's `alice`/`bob` users were always `jjfu_*` profiles wearing placeholder names.

**Constants classification**:
- Tinder: Windows fixed paths (`C:\ProgramData\ssh\*`, `C:\cygwin64`), TCP/22, firewall rule name
- Kindle: WSL distro name (`rbtww-main` — deferred deconfliction), docker context name (`wsl-native`), SSH key filename, host alias
- Parameters on `buw-HWar`: host, user, key-name, alias (four params)
- Parameters on `buw-HWew` and `rbw-HWdw`: distro-name
- No regime file needed yet

**Docker stays in RBK** because the dual-daemon topology (Desktop for Windows/Cygwin, native for WSL) is a Recipe Bottle testing decision, not generic Windows setup.

**Furnish weight**: BUK handbook CLI needs only `buh_*` combinators — thin furnish like `rbho_cli.sh`. RBK orchestrator needs BUK constants plus its own Docker constants — still thin, no regime/OAuth/IAM.

**Style rule**: Use `buh_*` combinators exclusively. The `rbhp_establish` function is the template (new style). Do NOT follow `rbhp_refresh` or `rbhp_quota_build`'s old-style `zrbhp_show()` private color variables.

**RBK orchestrator rendering**: `rbw-hw` uses `buh_T` (tabtarget combinator) to render clickable BUK/JJK/RBK tabtarget paths in dependency order.

**Handbook/tabtarget separation**: Handbooks display, tabtargets do. Following the `rbho_onboarding.sh` pattern: handbooks render copy-paste commands (`buh_c` for PowerShell, `buh_T` for tabtargets) and the human orchestrates. No attempt to wrap PowerShell or get exit status from it. `buw-HWax` is pure display — shows the `command=` routing format and `icacls` commands, takes no params. Project-specific environment commands appear in `rbw-hw`'s orchestrator output.

**Verification tabtargets**: Deferred to after practice walkthroughs. Once SSH routing works, real tabtargets can probe Windows host status over SSH (like onboarding's probe functions). Practice paces will reveal which verifications are worth automating.

**Fundus capability registry**: `Tools/rbk/vov_veiled/RBSFR-FundusRegistry.md` — agent-interpreted inventory of all test targets. Prototype, not program-readable. TBD fields fill in as practice paces complete.

## Tabtarget Inventory

```
BUK:
  tt/buw-hw.HandbookWindows.sh           — BUK-level top checklist (generic OS procedures)
  tt/buw-HWab.AccessBase.sh             — OpenSSH server install + lockdown
  tt/buw-HWar.AccessRemote.sh           — client key gen + ssh config (params: host, user, key-name, alias)
  tt/buw-HWax.AccessEntrypoints.sh      — command= routing format + icacls (pure display, no params)
  tt/buw-HWew.EnvironmentWSL.sh         — WSL distro creation (param: distro-name)
  tt/buw-HWec.EnvironmentCygwin.sh      — Cygwin install (verification: bash >= 3.2)

RBK:
  tt/rbw-h0.HandbookTOP.sh              — top-level index across all handbook groups
  tt/rbw-hw.HandbookWindows.sh          — orchestrator referencing BUK + JJK + RBK steps via buh_T
  tt/rbw-HWdd.DockerDesktop.sh          — Docker Desktop install
  tt/rbw-HWdw.DockerWSLNative.sh        — native dockerd in WSL (param: distro-name)
  tt/rbw-HWdc.DockerContextDiscipline.sh — deterministic daemon selection
```

Note: `rbw-h0` (HandbookTOP), `rbw-ho` (HandbookOnboarding), `rbw-hp` (HandbookPayor) are part of the handbook family reorganization but are NOT in scope for this heat. Existing onboarding/payor tabtargets (`rbw-go*`, `rbw-gP*`) continue to work as-is.

## Open Questions
- `rbtww-main` mint deconfliction deferred to post-MVP
- BUK handbook module naming: `buhw_*` functions in `buhw_windows.sh` (new file, not extending `buh_handbook.sh`)
- Handbook family reorganization (renaming existing `rbw-go*`/`rbw-gP*` to `rbw-HO*`/`rbw-HP*`) is a separate heat or pace in `₣A6` (handbook-restart)