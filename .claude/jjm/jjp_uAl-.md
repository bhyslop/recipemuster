## Context

Windows test infrastructure for Recipe Bottle. Goal: run RB's container isolation tests on a Windows host with WSL, Cygwin, and dual Docker daemons. The setup is largely manual (OpenSSH config, Windows Settings, installer GUIs), so it needs handbook procedures — colored `buh_*` combinator output guiding a human through each step.

Raw draft captured in `Memos/memo-20260412-windows-handbook-draft.md` (9 procedures from an earlier agent session, uses placeholder names and wrong prefix `RBWH`).

## Design Decisions

**Prefix**: `RBHW` (child of `rbh` handbook family). Draft used `RBWH` which conflicts with `rbw` (workbench terminal exclusivity).

**Three-kit split** — procedures divided by mechanism vs policy:

| Kit | Scope | Tabtargets |
|-----|-------|------------|
| BUK (`buw-HW*`) | Generic Windows OS mechanisms | 6: AccessBase, AccessRemote, AccessEntrypoints, EnvironmentWSL, EnvironmentCygwin, top-level |
| JJK (existing `jjw-tfP1/P2`) | Fundus user provisioning (`jjfu_*` accounts) | 0 new — existing P1/P2 already work inside WSL |
| RBK (`rbw-HW*`) | Project topology + Docker policy | 4: DockerDesktop, DockerWSLNative, DockerContextDiscipline, orchestrator |

**Colophon pattern**: `buw-hw` / `rbw-hw` lowercase for top-level entry, `buw-HW*` / `rbw-HW*` uppercase for subordinate procedures.

**Key insight**: JJK's `jjfp_fundus.sh` already handles Linux account provisioning (create `jjfu_*` users, install SSH keys, clone repos). It runs unchanged inside a WSL distro. The draft's `alice`/`bob` users were always `jjfu_*` profiles wearing placeholder names.

**Constants classification**:
- Tinder: Windows fixed paths (`C:\ProgramData\ssh\*`, `C:\cygwin64`), TCP/22, firewall rule name
- Kindle: WSL distro name (`rbtww-main` — deferred deconfliction), docker context name (`wsl-native`), SSH key filename, host alias
- Parameters on `buw-HWar`: host, user, key-name, alias (four params)
- Parameters on `buw-HWax`: environment routing entries (key-comment, command pairs) — mechanism is generic, policy comes from caller
- Parameters on `buw-HWew` and `rbw-HWdw`: distro-name
- No regime file needed yet

**Docker stays in RBK** because the dual-daemon topology (Desktop for Windows/Cygwin, native for WSL) is a Recipe Bottle testing decision, not generic Windows setup.

**Furnish weight**: BUK handbook CLI needs only `buh_*` combinators — thin furnish like `rbho_cli.sh`. RBK orchestrator needs BUK constants plus its own Docker constants — still thin, no regime/OAuth/IAM.

**Style rule**: Use `buh_*` combinators exclusively. The `rbhp_establish` function is the template (new style). Do NOT follow `rbhp_refresh` or `rbhp_quota_build`'s old-style `zrbhp_show()` private color variables.

**RBK orchestrator rendering**: `rbw-hw` uses `buh_T` (tabtarget combinator) to render clickable BUK/JJK/RBK tabtarget paths in dependency order.

## Tabtarget Inventory

```
BUK:
  tt/buw-hw.WindowsHandbook.sh          — top-level checklist
  tt/buw-HWab.AccessBase.sh             — OpenSSH server install + lockdown
  tt/buw-HWar.AccessRemote.sh           — client key gen + ssh config (params: host, user, key-name, alias)
  tt/buw-HWax.AccessEntrypoints.sh      — command= routing + icacls (params: environment routing entries)
  tt/buw-HWew.EnvironmentWSL.sh         — WSL distro creation (param: distro-name)
  tt/buw-HWec.EnvironmentCygwin.sh      — Cygwin install (verification: bash >= 3.2, not a specific version)

RBK:
  tt/rbw-hw.WindowsHandbook.sh          — orchestrator referencing BUK + JJK + RBK steps via buh_T
  tt/rbw-HWdd.DockerDesktop.sh          — Docker Desktop install
  tt/rbw-HWdw.DockerWSLNative.sh        — native dockerd in WSL (param: distro-name)
  tt/rbw-HWdc.DockerContextDiscipline.sh — deterministic daemon selection
```

## Open Questions
- `rbtww-main` mint deconfliction deferred to post-MVP
- BUK handbook module naming: `buhw_*` functions in `buhw_windows.sh` (new file, not extending `buh_handbook.sh`)
- `buw-HWax` parameterization: how environment routing entries are passed — positional args, array, or kindle constants sourced from caller's kit? (OPEN — needs design discussion)