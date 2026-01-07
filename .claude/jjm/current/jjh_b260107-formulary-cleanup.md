# Heat: Formulary Cleanup

## Paddock

### Goal
Enforce the "pure formulary" pattern across all BUK formularies: formularies route colophons to CLIs and never call `z*_kindle()` directly. All kindle orchestration moves to CLI `furnish()` functions.

### Background

**Formulary** is the architectural term for the component that BUD (dispatch) invokes to route tabtarget colophons to their implementations. Current implementations include workbenches, testbenches, and coordinators.

**TabTarget Anatomy** (publishing terminology):
- **Colophon** (`BUD_TOKEN_1`) - Routing identifier the formulary matches on (e.g., `rbw-B`)
- **Frontispiece** (`BUD_TOKEN_2`) - Human-readable description (e.g., `ConnectBottle`)
- **Imprint** (`BUD_TOKEN_3+`) - Embedded parameters/target specifier (e.g., `nsproto`)

**The Pattern** (established by `rbk_Coordinator.sh`):
```
TabTarget → Launcher (environment gate) → BUD → Formulary → exec CLI → furnish() kindles → execute
```

Formularies should be pure routing tables:
- Receive colophon from BUD via `BUD_TOKEN_1`
- Route to appropriate CLI via `exec`
- Pass imprints to the implementation as target parameters
- Know nothing about kindle graphs

CLIs own initialization:
- `furnish()` function handles all kindle orchestration
- Reads imprint from `BUD_TOKEN_3` when needed
- Manages inter-module kindle dependencies
- Calls `buc_execute` for final dispatch

### Imprint Convention

**Imprints are optional.** Many tabtargets have only colophon + frontispiece:
```
tt/buw-tt-ll.ListLaunchers.sh
├── Colophon: buw-tt-ll      (BUD_TOKEN_1 - formulary routes on this)
├── Frontispiece: ListLaunchers (BUD_TOKEN_2 - human reads this)
└── (no imprint)
```

For commands requiring a target specifier (e.g., rbw-s, rbt-to), the imprint is the third token:
```
tt/rbw-s.Start.nsproto.sh
├── Colophon: rbw-s          (BUD_TOKEN_1 - formulary routes on this)
├── Frontispiece: Start      (BUD_TOKEN_2 - human reads this)
└── Imprint: nsproto         (BUD_TOKEN_3 - passed to implementation)
```

BUD always explodes tokens. When present, CLIs read `BUD_TOKEN_3` (imprint) directly from environment. Formularies route purely on colophon and don't need to know whether an imprint exists.

### Audit Results

| Formulary | Current Issue | Fix |
|-----------|--------------|-----|
| `buw_workbench.sh` | Calls `zbuut_kindle()` directly | Create `buut_cli.sh` |
| `rbw_workbench.sh` | `rbw_load_nameplate()` kindles chain | Move to `rbob_cli.sh` furnish |
| `rbt_testbench.sh` | `rbt_load_nameplate()` kindles chain | Create `rbt_cli.sh` or share |
| `jjt_testbench.sh` | `zjju_kindle` at module load time | Wrap in furnish pattern |

Already clean: `rbk_Coordinator.sh`, `jjw_workbench.sh`
Borderline (config sourcing, not BCG kindle): `cccw_workbench.sh`, `vslw_workbench.sh`

### Files to Touch

**Create:**
- `Tools/buk/buut_cli.sh` - CLI for tabtarget operations
- `Tools/rbw/rbt_cli.sh` - CLI for testbench operations (or extend rbob_cli.sh)

**Modify:**
- `Tools/buk/buw_workbench.sh` - Pure routing to buut_cli.sh
- `Tools/rbw/rbw_workbench.sh` - Pure routing to rbob_cli.sh
- `Tools/rbw/rbob_cli.sh` - Add furnish with imprint loading from BUD_TOKEN_3
- `Tools/rbw/rbt_testbench.sh` - Pure routing to rbt_cli.sh
- `Tools/jjk/jjt_testbench.sh` - Move kindle into furnish or route structure

### Reference Files

- `Tools/rbw/rbk_Coordinator.sh` - Model pure formulary (already correct)
- `Tools/jjk/jjw_workbench.sh` - Model pure formulary (already correct)
- `Tools/rbw/rbgg_cli.sh` - Model CLI with furnish pattern
- `Tools/buk/README.md` - Formulary/colophon/imprint documentation

### Testing Strategy

Each pace should verify:
1. Existing tabtargets still work after refactor
2. Kindle happens in CLI, not formulary
3. Imprint flows correctly via BUD_TOKEN_3 (where applicable)

### Out of Scope

- `cccw_workbench.sh` - Sources config file, not BCG kindle (different pattern)
- `vslw_workbench.sh` - Sources config file, not BCG kindle (different pattern)
- `cmw_workbench.sh` - Self-contained, no external modules

## Done

- **buw-formulary-pure** - Created buut_cli.sh, refactored buw_workbench.sh to pure colophon routing

## Remaining

- **rbw-formulary-pure** - Refactor rbw_workbench.sh to pure colophon routing via rbob_cli.sh
  Move `rbw_load_nameplate()` logic into `rbob_cli.sh` furnish. This CLI requires imprint: furnish reads `BUD_TOKEN_3` (e.g., `nsproto`), loads nameplate file `rbrn_${imprint}.env`, kindles rbrn→rbrr→rbob chain. Update `rbw_workbench.sh` to pure `exec` routing on colophons. Test with `tt/rbw-s.Start.nsproto.sh`.

- **rbt-formulary-pure** - Refactor rbt_testbench.sh to pure colophon routing with rbt_cli.sh
  Create `rbt_cli.sh` with furnish that handles imprint-based nameplate loading (same pattern as rbob). The imprint specifies which test suite to run. Could potentially share nameplate loading with rbob, or duplicate for isolation. Update `rbt_testbench.sh` to pure routing. Test with `tt/rbt-to.TestBottleService.nsproto.sh`.

- **jjt-formulary-pure** - Refactor jjt_testbench.sh to move kindle into proper structure
  Currently `zjju_kindle` is called at line 36 (module load time). Either: (a) create `jjt_cli.sh` with furnish, or (b) move kindle into a route function. The testbench pattern is slightly different - it's more like a CLI itself. Decide on pattern and implement. Test with `tt/jjt-f.TestFavor.sh`.

- **document-formulary-pattern** - Update README with complete formulary/CLI pattern
  Ensure `Tools/buk/README.md` documents the full pattern including: formulary responsibilities, CLI furnish pattern, imprint via BUD_TOKEN_3, and reference implementations. Verify colophon/frontispiece/imprint terminology is consistently used.
