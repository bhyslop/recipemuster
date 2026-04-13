## Context

Windows test infrastructure for Recipe Bottle. Goal: run RB's container isolation tests on a Windows host with WSL, Cygwin, and dual Docker daemons. Setup splits into automation tabtargets (SSH keys, sshd config, authorized_keys, connectivity verification) and handbook residue for irreducibly manual steps (WSL install, Cygwin install, Docker Desktop install).

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

**Three-kit split** — divided by mechanism vs policy:

| Kit | Scope | Tabtargets |
|-----|-------|------------|
| BUK (`buw-HW*`) | Generic OS mechanisms + BURH constructors | 6 constructors (Linux, macOS, Cygwin, WSL, PowerShell, Localhost), 3 automation (SshConfig, VerifySsh, BootstrapSshd), 6 handbooks (top, AccessBase, AccessRemote, AccessEntrypoints, EnvironmentWSL, EnvironmentCygwin) |
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

**Architectural pivot — handbooks → automation with handbook residue:**

BURH profiles make most SSH setup automatable. Rather than handbooks that display commands for humans to copy, tabtargets execute directly. Two-phase connectivity model:

| Phase | Boundary | How tabtargets run |
|-------|----------|-------------------|
| Bootstrap | No SSH yet — human at Windows console, WSL available (the feature, not a specific distro) | Run tabtargets directly in WSL terminal |
| Operational | SSH works | Relay from Mac via `jjx_bind`/`jjx_relay` |

The bootstrap tabtarget's job is to get from phase 1 to phase 2.

**PowerShell-from-WSL pattern** (BCG-compliant):

`powershell.exe` is callable from WSL bash as a plain external command. No subshells needed — BCG's standard temp-file capture and `|| buc_die` patterns apply directly:
```bash
powershell.exe -Command "Get-Service sshd" > "${Z_MODULE_TEMP1}" || buc_die "Failed"
```
Bash owns control flow, error handling, and output. PowerShell is just the syscall layer for Windows operations. Complex PowerShell avoided entirely — one atomic command per call.

**Impact — handbook procedures absorbed into tabtargets:**
- `buw-HWab` (AccessBase) → absorbed into `buw-HWbs` (BootstrapSshd) steps 1-6
- `buw-HWar` (AccessRemote) → absorbed into platform constructors + `buw-HWsc` (SshConfig). Key generation is manual — constructors display the `ssh-keygen` command, user runs it, re-runs constructor to populate pubkey. No tabtarget writes to `~/.ssh/`.
- `buw-HWax` (AccessEntrypoints) → absorbed into `buw-HWbs` step 4 (authorized_keys from BURH)
- Original handbook files become thin wrappers: "run this tabtarget" + context for remaining manual steps

**BURH profile constructors** — one per target platform:

| Constructor | Alias suffix | BURH_COMMAND | Notes |
|-------------|-------------|-------------|-------|
| Linux | `-linux` | empty | Remote Linux, default shell |
| macOS | `-mac` | empty | Remote Mac, default shell |
| Cygwin | `-cyg` | `C:\cygwin64\bin\bash.exe -l` | Windows Cygwin |
| WSL | `-wsl` | `C:\...\wsl.exe -d {DISTRO_CONST} ...` | WSL distro name from kindle constant |
| PowerShell | `-ps` | `C:\...\powershell.exe` | Windows PowerShell |
| Localhost | special | empty | `host=localhost`, no command= routing |

Common params: `host`, `user`, `moniker`. Alias = `{moniker}-{suffix}`. `BURH_COMMAND` empty is valid (validation min-length changed to 0). Constructors read `~/.ssh/{alias}.pub` if present; otherwise display keygen command for user to run manually.

**Step auto-numbering** (`buh_step1`/`buh_step2`):

Discovered during AccessBase practice that hardcoded step numbers cause renumbering bugs on every insertion/deletion. Implemented mutable kindle state (`z_buh_step_n`, `z_buh_substep_n`, `z_buh_body_indent`) following the zipper roll precedent for controlled mutables. `buh_section` resets indent to top level. All 8 Windows/Docker handbook procedures converted.

**sshd_config discovery** (from AccessBase practice — now baked into `buw-HWbs` template):
- `UsePAM` and `ChallengeResponseAuthentication` are unsupported by Windows OpenSSH — service fails to start with any unrecognized directive
- `buw-HWbs` writes sshd_config from a known-good template (incorporating these constraints), then validates with `sshd -t` before applying
- The original handbook's copy-edit-validate-replace workflow is superseded by template-write

**Docker stays in RBK** because the dual-daemon topology (Desktop for Windows/Cygwin, native for WSL) is a Recipe Bottle testing decision, not generic Windows setup.

**Furnish weight**: BUK handbook CLI needs only `buh_*` combinators — thin furnish like `rbho_cli.sh`. RBK orchestrator needs BUK constants plus its own Docker constants — still thin, no regime/OAuth/IAM.

**Style rule**: Use `buh_*` combinators exclusively. The `rbhp_establish` function is the template (new style). Do NOT follow `rbhp_refresh` or `rbhp_quota_build`'s old-style `zrbhp_show()` private color variables.

**Handbook/tabtarget separation**: Handbooks display, tabtargets do — but BURH sharpens this: most "display" handbooks become "run this tabtarget" wrappers. The `rbho_onboarding.sh` pattern still holds for irreducibly manual steps (GUI installs).

**Verification tabtargets**: SSH verify (`buw-HWvs`) built in ₢A-AAL. Post-SSH health probes (WSL, fundus, Docker) deferred to ₢A-AAH after practice walkthroughs.

**Fundus capability registry**: `Tools/rbk/vov_veiled/RBSFR-FundusRegistry.md` — agent-interpreted inventory of all test targets. Prototype, not program-readable. TBD fields fill in as practice paces complete.

## Tabtarget Inventory

```
BUK — Constructors (new, from ₢A-AAL — one per platform):
  tt/buw-HWcl.ConstructLinux.sh          — BURH constructor: Linux target
  tt/buw-HWcm.ConstructMac.sh           — BURH constructor: macOS target
  tt/buw-HWcc.ConstructCygwin.sh         — BURH constructor: Windows Cygwin
  tt/buw-HWcw.ConstructWSL.sh           — BURH constructor: Windows WSL
  tt/buw-HWcp.ConstructPowerShell.sh     — BURH constructor: Windows PowerShell
  tt/buw-HWcx.ConstructLocalhost.sh      — BURH constructor: localhost (host=localhost, no command=)

BUK — Automation (new, from ₢A-AAL):
  tt/buw-HWsc.SshConfig.sh              — kindle all BURH profiles, write ~/.ssh/config
  tt/buw-HWvs.VerifySsh.sh              — kindle BURH, ssh test
  tt/buw-HWbs.BootstrapSshd.sh          — idempotent sshd setup via PowerShell (WSL-side)

BUK — Handbooks (existing, becoming thin wrappers):
  tt/buw-hw.HandbookWindows.sh           — BUK-level top checklist (generic OS procedures)
  tt/buw-HWab.AccessBase.sh             — handbook residue: context for buw-HWbs
  tt/buw-HWar.AccessRemote.sh           — handbook residue: context for constructors + buw-HWsc
  tt/buw-HWax.AccessEntrypoints.sh      — handbook residue: context for buw-HWbs step 4
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