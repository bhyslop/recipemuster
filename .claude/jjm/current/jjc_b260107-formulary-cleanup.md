# Steeplechase: Formulary Cleanup

---
### 2026-01-07 - buw-formulary-pure - APPROACH
**Proposed approach**:
- Create `buut_cli.sh` with furnish that kindles buut, uses `buc_execute` for dispatch
- Update `buw_workbench.sh` to `exec` to `buut_cli.sh` for all `buw-tt-*` colophons
- Remove direct `zbuut_kindle` call and `buut_*` function calls from workbench
- Test with `tt/buw-tt-ll.ListLaunchers.sh` and a create variant
---

---
### 2026-01-07 14:32 - buw-formulary-pure - WRAP
**Outcome**: Created buut_cli.sh, refactored buw_workbench.sh to pure colophon routing
---

---
### 2026-01-07 14:35 - rbw-formulary-pure - APPROACH
**Proposed approach**:
- Refactor `rbw_workbench.sh` to pure routing: `exec rbob_cli.sh` for rbw-s/z/S/C/B/o commands
- Update `rbob_cli.sh` furnish to read moniker from `BUD_TOKEN_3` instead of `RBOB_MONIKER`
- Add rbob_start/stop/connect_* as CLI commands routed via `buc_execute`
- Keep `rbw-lB` separate (local build doesn't use nameplate pattern)
---
