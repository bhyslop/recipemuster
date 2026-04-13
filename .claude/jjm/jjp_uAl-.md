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

**BURH — BUK Regime Host** (new regime, discovered during AccessBase practice):

Per-user, per-connection-profile regime for SSH access to remote hosts. Solves three problems discovered during first practice:
1. AccessRemote took 4 free-form params that the user had to guess
2. No way to transport public key material between machines except manual transcription
3. Connective details (host IP, username, key names, aliases) scattered across human memory

Directory structure:
```
.buk/users/${BURS_USER}/<virtual-hostname>/burh.env
```

Each profile is one SSH connection — one key, one alias, one entry point. Three profiles for Windows (cygwin, wsl, ps) share the same physical host but are distinct connections.

Schema (5 fields):
```bash
BURH_HOST=192.168.86.27
BURH_USER=bhyslop
BURH_ALIAS=winhost-cyg
BURH_SSH_PUBKEY='ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAA... bhyslop@macmini'
BURH_COMMAND='C:\cygwin64\bin\bash.exe -l'
```

- `BURH_HOST` — IP or hostname of the physical machine
- `BURH_USER` — username on the remote (may differ from local `BURS_USER`)
- `BURH_ALIAS` — SSH alias and key filename (must match directory name)
- `BURH_SSH_PUBKEY` — full public key line (git-transported, solves chicken-and-egg)
- `BURH_COMMAND` — shell command for `command=` routing in `authorized_keys`

Single-quoted values for Windows backslashes. Git-safe: no private key material.

`BURS_USER` addition: station regime gets a new field identifying the local developer. Routes to the correct `.buk/users/` subdirectory.

**Impact on handbook procedures:**
- `buw-HWar` (AccessRemote): kindles BURH profile instead of taking 4 params. Renders exact `ssh-keygen` and `~/.ssh/config` entries.
- `buw-HWax` (AccessEntrypoints): kindles all BURH profiles for a host, renders complete `authorized_keys` lines with real pubkey material and `command=` prefixes. No more `AAAA... replace me`.
- `rbw-hw` (orchestrator): renders exact invocations per profile instead of generic tabtarget links.

**Step auto-numbering** (`buh_step1`/`buh_step2`):

Discovered during AccessBase practice that hardcoded step numbers cause renumbering bugs on every insertion/deletion. Implemented mutable kindle state (`z_buh_step_n`, `z_buh_substep_n`, `z_buh_body_indent`) following the zipper roll precedent for controlled mutables. `buh_section` resets indent to top level. All 8 Windows/Docker handbook procedures converted.

**sshd_config discovery** (from AccessBase practice):
- `UsePAM` and `ChallengeResponseAuthentication` are unsupported by Windows OpenSSH — service fails to start with any unrecognized directive
- Config file is SYSTEM-owned — procedure uses copy-edit-validate-replace workflow with `buh_step2` substeps
- `sshd -t` validates config before applying

**Docker stays in RBK** because the dual-daemon topology (Desktop for Windows/Cygwin, native for WSL) is a Recipe Bottle testing decision, not generic Windows setup.

**Furnish weight**: BUK handbook CLI needs only `buh_*` combinators — thin furnish like `rbho_cli.sh`. RBK orchestrator needs BUK constants plus its own Docker constants — still thin, no regime/OAuth/IAM.

**Style rule**: Use `buh_*` combinators exclusively. The `rbhp_establish` function is the template (new style). Do NOT follow `rbhp_refresh` or `rbhp_quota_build`'s old-style `zrbhp_show()` private color variables.

**Handbook/tabtarget separation**: Handbooks display, tabtargets do. Following the `rbho_onboarding.sh` pattern.

**Verification tabtargets**: Deferred to after practice walkthroughs.

**Fundus capability registry**: `Tools/rbk/vov_veiled/RBSFR-FundusRegistry.md` — agent-interpreted inventory of all test targets. Prototype, not program-readable. TBD fields fill in as practice paces complete.

## Tabtarget Inventory

```
BUK:
  tt/buw-hw.HandbookWindows.sh           — BUK-level top checklist (generic OS procedures)
  tt/buw-HWab.AccessBase.sh             — OpenSSH server install + lockdown
  tt/buw-HWar.AccessRemote.sh           — client key gen + ssh config (kindles BURH profile)
  tt/buw-HWax.AccessEntrypoints.sh      — command= routing from BURH profiles
  tt/buw-HWew.EnvironmentWSL.sh         — WSL distro creation (param: distro-name)
  tt/buw-HWec.EnvironmentCygwin.sh      — Cygwin install (verification: bash >= 3.2)

RBK:
  tt/rbw-h0.HandbookTOP.sh              — top-level index across all handbook groups
  tt/rbw-hw.HandbookWindows.sh          — orchestrator referencing BUK + JJK + RBK steps via buh_T
  tt/rbw-HWdd.DockerDesktop.sh          — Docker Desktop install
  tt/rbw-HWdw.DockerWSLNative.sh        — native dockerd in WSL (param: distro-name)
  tt/rbw-HWdc.DockerContextDiscipline.sh — deterministic daemon selection
```

## Open Questions
- `rbtww-main` mint deconfliction deferred to post-MVP
- BUK handbook module naming: `buhw_*` functions in `buhw_windows.sh` (new file, not extending `buh_handbook.sh`)
- Handbook family reorganization (renaming existing `rbw-go*`/`rbw-gP*` to `rbw-HO*`/`rbw-HP*`) is a separate heat or pace in `₣A6` (handbook-restart)