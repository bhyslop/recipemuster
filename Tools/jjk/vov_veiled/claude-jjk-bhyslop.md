## JJK — bhyslop project specifics (veiled, not distributed)

This file holds the rbm-only scenery carved out of the public JJK guidance
(`Tools/jjk/claude-jjk-core.md`). It stays in this repo and is never
distributed to consumer projects.

### A. File Acronym Mappings — JJK Subdirectory (`Tools/jjk/`)

- **JJS0** → `jjk/vov_veiled/JJS0_JobJockeySpec.adoc` (Job Jockey specification - main file)
- **JJSCCH** → `jjk/vov_veiled/JJSCCH-chalk.adoc`
- **JJSCCU** → `jjk/vov_veiled/JJSCCU-curry.adoc` (Paddock operation - read/write heat paddock files)
- **JJSCDR** → `jjk/vov_veiled/JJSCDR-draft.adoc`
- **JJSCFU** → `jjk/vov_veiled/JJSCFU-furlough.adoc`
- **JJSCMU** → `jjk/vov_veiled/JJSCMU-muster.adoc`
- **JJSCNC** → `jjk/vov_veiled/JJSCNC-notch.adoc`
- **JJSCNO** → `jjk/vov_veiled/JJSCNO-nominate.adoc`
- **JJSCPD** → `jjk/vov_veiled/JJSCPD-parade.adoc`
- **JJSCRL** → `jjk/vov_veiled/JJSCRL-rail.adoc`
- **JJSCRN** → `jjk/vov_veiled/JJSCRN-rein.adoc`
- **JJSCRT** → `jjk/vov_veiled/JJSCRT-retire.adoc`
- **JJSCSC** → `jjk/vov_veiled/JJSCSC-scout.adoc`
- **JJSCSD** → `jjk/vov_veiled/JJSCSD-saddle.adoc`
- **JJSCSL** → `jjk/vov_veiled/JJSCSL-slate.adoc`
- **JJSCTL** → `jjk/vov_veiled/JJSCTL-tally.adoc`
- **JJSCVL** → `jjk/vov_veiled/JJSCVL-validate.adoc`
- **JJSCWP** → `jjk/vov_veiled/JJSCWP-wrap.adoc` (Close/wrap operation - mark pace complete and commit)
- **JJSRLD** → `jjk/vov_veiled/JJSRLD-load.adoc`
- **JJSRPS** → `jjk/vov_veiled/JJSRPS-persist.adoc`
- **JJSRSV** → `jjk/vov_veiled/JJSRSV-save.adoc`
- **JJSRWP** → `jjk/vov_veiled/JJSRWP-wrap.adoc`
- **JJSTF** → `jjk/vov_veiled/JJSTF-test-fundus.adoc` (Test Fundus — fundus scenario profiles and preflight contracts)
- **JJW**  → `jjk/jjw_workbench.sh` (workbench)

### B. Foray Protocol — Fundus constants (rbm values)

**Fundus constants** (use these for `jjx_bind`):
- Default reldir: `projects/rbm_alpha_recipemuster`
- BURN alias: use the alias from the target's BUK Regime Node profile (e.g., `winhost-wsl`, `winhost-cyg`)

Hosts in play for this repo:
- `winhost-wsl` / `winhost-cyg` — Windows host transports (WSL and Cygwin).
- `cerebro` — Linux test host used as a fundus by JJK fundus scenario tests.

In the public Foray Workflow, the bind example reldir is genericized to
`<your-project-reldir>`; the rbm-specific value is `projects/rbm_alpha_recipemuster`.

### C. Build & Run Discipline (rbm/VOK-specific)

**Build & Run Discipline:**
Always run these after Rust code changes:
- `tt/vow-b.Build.sh` — Build
- `tt/vvw-r.RunVVX.sh` — Run VVX
