## Bash Utility Kit (BUK) Concepts

BUK provides tabtarget/launcher infrastructure for bash-based tooling.

### TabTarget System

TabTargets are lightweight shell scripts in `tt/` that serve as the CLI entry point for all operations. They delegate to workbenches via launchers — no business logic lives in tabtargets.

**Discoverability**: `ls tt/` shows all available commands. Tab completion narrows by prefix: `tt/rbw-<TAB>`.

**Naming pattern**: `{colophon}.{frontispiece}[.{imprint}].sh`

| Part | Purpose | Example |
|------|---------|---------|
| **Colophon** | Routing identifier (includes hyphen, workbench matches on this) | `rbw-B`, `buw-tt-ll` |
| **Frontispiece** | Human-readable description (PascalCase) | `ConnectBottle` |
| **Imprint** | Optional target parameter (nameplate moniker, fixture name, etc.) | `tadmor` |

- The `.` is the delimiter between parts
- The hyphen is part of the colophon (not a separator)

Example: `tt/rbw-B.ConnectBottle.tadmor.sh` — colophon `rbw-B` routes to the bottle connect command, frontispiece tells you what it does, imprint `tadmor` selects the nameplate.

Multiple tabtargets can share the same colophon but differ by imprint:
```
tt/rbw-cC.Charge.tadmor.sh
tt/rbw-cC.Charge.srjcl.sh
tt/rbw-cC.Charge.pluml.sh
```

### BUK Vocabulary

| Term | Definition |
|------|------------|
| **Zipper** | BCG-compliant module kindling colophon→module→command array constants |
| **Workbench** | Routes commands: `{prefix}w_workbench.sh` |
| **Testbench** | Routes tests: `{prefix}t_testbench.sh` |

**Key files:**
- `Tools/buk/buc_command.sh` — command utilities
- `Tools/buk/bud_dispatch.sh` — dispatch utilities
- `Tools/buk/buw_workbench.sh` — workbench

Full spec: `Tools/buk/README.md`

## Forbidden Shell Operations

**Never use `cd` in Bash commands — NO exceptions.**

The working directory persists between Bash tool calls. A single `cd` corrupts ALL subsequent commands that use relative paths, including every `./tt/` tabtarget.

- Use absolute paths instead of cd'ing

**There is no safe cd.** Do not reason that "I'll cd back" — the next tool call may be yours or another Claude Code session's, and it will break.

## Test Execution Discipline

Run test fixture tabtargets **sequentially, never in parallel**. Test fixtures share regime state and container/network namespaces — parallel execution causes resource conflicts and false failures.

```
# Correct: run one at a time
tt/rbw-tf.TestFixture.regime-validation.sh
tt/rbw-tf.TestFixture.tadmor-security.sh

# Wrong: never run fixtures concurrently
tt/rbw-tf.TestFixture.regime-validation.sh & tt/rbw-tf.TestFixture.tadmor-security.sh &
```
