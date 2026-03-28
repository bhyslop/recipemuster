## Bash Utility Kit (BUK) Concepts

BUK provides tabtarget/launcher infrastructure for bash-based tooling. Key vocabulary:

| Term | Definition |
|------|------------|
| **Tabtarget** | Launcher script in `tt/` that delegates to a workbench |
| **Colophon** | Routing identifier (includes hyphen): `rbw-B`, `buw-tt-ll` |
| **Frontispiece** | Human-readable description (PascalCase): `ConnectBottle` |
| **Imprint** | Optional target parameter: `nsproto`, `srjcl` |
| **Zipper** | BCG-compliant module kindling colophon→module→command array constants |
| **Workbench** | Routes commands: `{prefix}w_workbench.sh` |
| **Testbench** | Routes tests: `{prefix}t_testbench.sh` |

**Tabtarget pattern**: `{colophon}.{frontispiece}[.{imprint}].sh`

- The `.` is the delimiter between parts
- The hyphen is part of the colophon (not a separator)
- Colophon naming follows **Prefix Naming Discipline** above

**Key files:**
- `Tools/buk/buc_command.sh` — command utilities
- `Tools/buk/bud_dispatch.sh` — dispatch utilities
- `Tools/buk/buw_workbench.sh` — workbench

Full spec: `Tools/buk/README.md`
