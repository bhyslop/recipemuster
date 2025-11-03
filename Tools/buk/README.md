# Bash Utility Kit (BUK)

A portable, graftable bash infrastructure for building maintainable command-line tools with configuration management, dispatch routing, and validation.

## Table of Contents

- [Overview](#overview)
- [Core Concepts](#core-concepts)
  - [Launchers](#launchers)
  - [Workbenches](#workbenches)
  - [TabTargets](#tabtargets)
  - [Config Regimes](#config-regimes)
- [Architecture](#architecture)
- [Installation](#installation)
- [BUK Components](#buk-components)
- [Creating a New Workbench](#creating-a-new-workbench)
- [Reference Implementation: BURC/BURS](#reference-implementation-burcburs)

---

## Overview

BUK provides a three-layer architecture for bash-based CLI tools:

1. **BUK Core** (`Tools/buk/*.sh`) - Portable utilities with no project-specific knowledge
2. **BURC** (`.buk/burc.env`) - Project-level configuration defining repository structure
3. **BURS** (`../station-files/burs.env`) - Developer/machine-level configuration (not in git)

This separation allows BUK to be copied wholesale into any project and configured through regime files rather than code modification.

---

## Core Concepts

### Launchers

**Definition**: A launcher is a bootstrap script that validates configuration, loads regime files, and delegates to the BDU (Bash Dispatch Utility).

**Naming Pattern**: `launcher.{workbench_name}.sh`

**Location**: `.buk/` directory at project root

**Examples**:
- `.buk/launcher.buw_workbench.sh` - BUK workbench launcher
- `.buk/launcher.cccw_workbench.sh` - CCCK workbench launcher
- `.buk/launcher.rbk_Coordinator.sh` - RBW coordinator launcher

**Canonical Structure**:

```bash
#!/bin/bash
# Compatible with Bash 3.2 (e.g., macOS default shell)

z_project_root_dir="${0%/*}/.."
cd "${z_project_root_dir}" || exit 1

# Load BURC configuration
export BDU_REGIME_FILE="${z_project_root_dir}/.buk/burc.env"
source "${BDU_REGIME_FILE}" || exit 1

# Validate config regimes that are known at launch time
# NOTE: BURC and BURS are the standard BUK regimes (project structure + station config)
# These are always validated here because they're required for BDU operation.
#
# Other project-specific regimes (like RBRN, RBRR, etc.) may be validated later
# during workbench dispatch, when runtime context is available.
"${BURC_TOOLS_DIR}/buk/burc_regime.sh" validate "${z_project_root_dir}/.buk/burc.env" || {
  echo "ERROR: BURC validation failed" >&2
  "${BURC_TOOLS_DIR}/buk/burc_regime.sh" info
  exit 1
}

z_station_file="${z_project_root_dir}/${BURC_STATION_FILE}"
"${BURC_TOOLS_DIR}/buk/burs_regime.sh" validate "${z_station_file}" || {
  echo "ERROR: BURS validation failed: ${z_station_file}" >&2
  "${BURC_TOOLS_DIR}/buk/burs_regime.sh" info
  exit 1
}

# Set coordinator script (the workbench for this launcher)
export BDU_COORDINATOR_SCRIPT="${BURC_TOOLS_DIR}/buk/buw_workbench.sh"

# Delegate to BDU
exec "${BURC_TOOLS_DIR}/buk/bdu_BashDispatchUtility.sh" "${1##*/}" "${@:2}"
```

**Key Responsibilities**:
1. Establish project root context
2. Load BURC configuration
3. Validate required config regimes (fail early if misconfigured)
4. Specify which workbench coordinates commands
5. Delegate execution to BDU

**Design Rationale**:
- Launchers catch configuration errors before BDU starts
- Clear naming ties launcher to its workbench
- Validation output helps developers fix configuration issues
- `exec` replaces the launcher process (no extra process overhead)

**Regime Validation Timing**:

Config regimes fall into two categories based on when they can be validated:

1. **Launch-time regimes** (validated in launcher):
   - **BURC** - Project structure configuration (always required by BUK)
   - **BURS** - Developer station configuration (always required by BUK)
   - **RBRR** - Recipe Bottle Regime Repo (RBW project config, if using RBW)
   - These are known immediately at launch time

2. **Runtime regimes** (validated in workbench):
   - **RBRN** - Recipe Bottle Regime Nameplate (RBW service config, runtime-specific)
   - Other project-specific regimes that depend on runtime context
   - These are validated during workbench dispatch when context is available

**Guideline**: All launchers created by BUK should validate BURC and BURS. Additional regime validation is optional and workbench-specific.

---

### Workbenches

**Definition**: A workbench is a multi-call bash script (BCG style) that routes commands to their implementations.

**Naming Pattern**: `{prefix}w_workbench.sh` or `{prefix}k_Coordinator.sh`

**Location**: `Tools/{workbench}/` subdirectory

**Examples**:
- `Tools/buk/buw_workbench.sh` - BUK workbench (manages BUK itself)
- `Tools/ccck/cccw_workbench.sh` - CCCK workbench (container control)
- `Tools/rbw/rbk_Coordinator.sh` - RBW coordinator (recipe bottle management)

**Structure**:

```bash
#!/bin/bash
set -euo pipefail

# Route function
workbench_route() {
  local z_command="$1"
  shift

  case "${z_command}" in
    cmd1) workbench_cmd1 "$@" ;;
    cmd2) workbench_cmd2 "$@" ;;
    *)
      echo "ERROR: Unknown command: ${z_command}" >&2
      exit 1
      ;;
  esac
}

# Command implementations
workbench_cmd1() {
  # Implementation
}

workbench_cmd2() {
  # Implementation
}

# Main entry point
workbench_main() {
  local z_command="${1:-}"
  shift || true

  if [ -z "${z_command}" ]; then
    echo "ERROR: No command specified" >&2
    exit 1
  fi

  workbench_route "${z_command}" "$@"
}

workbench_main "$@"
```

**Key Characteristics**:
- Single-file coordinator that routes commands
- Follows BCG multi-call pattern
- Loads configuration (BURC/BURS) as needed
- Can delegate to other scripts for complex operations
- Crash-fast error handling (`set -euo pipefail`)

---

### TabTargets

**Definition**: TabTargets are lightweight shell scripts in the `tt/` directory that provide tab-completion-friendly command names and delegate to workbenches via launchers.

**Naming Pattern**: `{command}.{description}.sh`

**Location**: `tt/` directory at project root (configurable via `BURC_TABTARGET_DIR`)

**Token Delimiter**: Configurable via `BURC_TABTARGET_DELIMITER` (typically `.`)

**Examples**:
- `tt/buw-ll.ListLaunchers.sh` - List launchers
- `tt/buw-rv.ValidateRegimes.sh` - Validate regimes
- `tt/ccck-ps.ProcessStatus.sh` - Show container processes

**Canonical Structure**:

```bash
#!/bin/bash
# TabTarget - delegates to {workbench} via launcher
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.{workbench}.sh" \
  "${0##*/}" "${@}"
```

**Command Token Parsing**:

The filename `buw-ll.ListLaunchers.sh` is parsed as:
- Full filename: `buw-ll.ListLaunchers.sh`
- First token (command): `buw-ll` (everything before first delimiter)
- Subsequent tokens: `ListLaunchers` (descriptive, for human readability)

BDU extracts the command token using `${filename%%${BURC_TABTARGET_DELIMITER}*}`.

**Key Benefits**:
1. **Tab completion**: Type `tt/buw-` then press TAB to see all BUK commands
2. **Self-documenting**: Filename describes what the command does
3. **Discoverability**: `ls tt/` shows all available commands
4. **Consistency**: All commands follow same invocation pattern
5. **Lightweight**: No logic in tabtargets, just delegation

**Design Rationale**:
- Leverages shell tab completion for command discovery
- Descriptive filenames serve as inline documentation
- Delegating to launchers ensures validation happens on every invocation
- Token-based parsing allows flexible, hierarchical command names

---

### Config Regimes

**Definition**: A Config Regime is a structured configuration system consisting of:
- **Specification** - Markdown document defining variables, types, and constraints
- **Assignment** - Shell-sourceable file (`.env`) containing actual values
- **Validator** - Script that enforces type rules and constraints
- **Renderer** - Script that displays configuration in human-readable format

**Namespace Identity**: Unique uppercase prefix (e.g., `BURC_`, `BURS_`, `RBRN_`, `RBRR_`) prevents variable collisions.

**Core Components**:

1. **Assignment File** (`{regime}.env`)
   - Concise filename (frequently sourced)
   - Shell-sourceable: `VAR=value` syntax, no spaces around `=`
   - Can use `${VAR}` expansion for derived values
   - Example: `.buk/burc.env`

2. **Specification File** (`{regime}_specification.md`)
   - Documents all variables, types, and constraints
   - Self-documenting, readable
   - Example: `Tools/buk/burc_specification.md`

3. **Regime Script** (`{regime}_regime.sh`)
   - Multi-call BCG-style script
   - Subcommands: `validate`, `render`, `info`
   - Example: `Tools/buk/burc_regime.sh`

**File Naming Pattern**:
- **Assignment**: `{regime}.env` (concise, frequently sourced)
- **Support files**: `{regime}_{full_word}.{ext}` (readable, self-documenting)

**Examples**:

| Regime | Assignment | Specification | Validator/Renderer |
|--------|-----------|---------------|-------------------|
| BURC | `.buk/burc.env` | `Tools/buk/burc_specification.md` | `Tools/buk/burc_regime.sh` |
| BURS | `../station-files/burs.env` | `Tools/buk/burs_specification.md` | `Tools/buk/burs_regime.sh` |

**Type System**:

BUK provides validation functions in `bvu_BashValidationUtility.sh`:
- **Atomic types**: `string`, `xname`, `fqin`, `bool`, `decimal`, `ipv4`, `cidr`, `domain`, `port`
- **List types**: `ipv4_list`, `cidr_list`, `domain_list`
- Each type validated with min/max constraints

**Why Config Regimes?**

1. **Separation of concerns**: Code is portable, configuration adapts it
2. **Type safety**: Validation catches errors early
3. **Documentation**: Specifications are authoritative and version-controlled
4. **Tooling**: Generic validators and renderers reduce boilerplate
5. **Scalability**: Multiple regimes can coexist without conflicts

---

## Architecture

```
Project Root/
├── .buk/                              # Launcher directory (project-specific bootstrap)
│   ├── burc.env                       # BURC assignment (project structure config)
│   ├── launcher.buw_workbench.sh      # BUK launcher (with validation)
│   ├── launcher.cccw_workbench.sh     # CCCK launcher (with validation)
│   └── launcher.rbk_Coordinator.sh    # RBW launcher (with validation)
│
├── tt/                                # TabTargets (tab-completion-friendly commands)
│   ├── buw-ll.ListLaunchers.sh        # List all launchers
│   ├── buw-rv.ValidateRegimes.sh      # Validate BURC/BURS
│   └── ccck-ps.ProcessStatus.sh       # Container status
│
├── Tools/                             # Tool scripts (portable, reusable)
│   ├── buk/                           # BUK core utilities (graftable module)
│   │   ├── bdu_BashDispatchUtility.sh # Dispatch system
│   │   ├── bcu_BashCommandUtility.sh  # Command utilities
│   │   ├── btu_BashTestUtility.sh     # Test utilities
│   │   ├── bvu_BashValidationUtility.sh # Validation (type system)
│   │   ├── buw_workbench.sh           # BUK workbench
│   │   ├── burc_specification.md      # BURC spec
│   │   ├── burc_regime.sh             # BURC validator/renderer
│   │   ├── burs_specification.md      # BURS spec
│   │   ├── burs_regime.sh             # BURS validator/renderer
│   │   └── README.md                  # This file
│   │
│   ├── ccck/                          # CCCK workbench
│   │   └── cccw_workbench.sh
│   │
│   └── rbw/                           # RBW workbench
│       └── rbk_Coordinator.sh
│
└── ../station-files/                  # Developer machine configs (NOT in git)
    └── burs.env                       # BURS assignment (station config)
```

**Execution Flow**:

```
User invokes TabTarget:
  $ tt/buw-ll.ListLaunchers.sh

1. TabTarget delegates to Launcher
   → .buk/launcher.buw_workbench.sh buw-ll

2. Launcher validates regimes
   → burc_regime.sh validate .buk/burc.env
   → burs_regime.sh validate ../station-files/burs.env
   → (If validation fails, display info and exit)

3. Launcher delegates to BDU
   → bdu_BashDispatchUtility.sh buw-ll

4. BDU sets up environment
   → Creates temp/output directories
   → Sources BURS (station config)
   → Sets up logging

5. BDU invokes Workbench
   → buw_workbench.sh buw-ll

6. Workbench routes command
   → Case statement routes "buw-ll" to implementation
   → Executes command logic
   → Returns exit status

7. BDU cleans up
   → Writes transcript
   → Propagates exit status
```

---

## Installation

### Quick Start: Copy BUK into Your Project

1. **Copy BUK directory**:
   ```bash
   cp -r /path/to/source/Tools/buk ./Tools/
   ```

2. **Create `.buk` directory and BURC file**:
   ```bash
   mkdir -p .buk
   cat > .buk/burc.env <<'EOF'
   # Bash Utility Regime Configuration (BURC)
   # Project-level configuration for BUK

   BURC_STATION_FILE=../station-files/burs.env
   BURC_TABTARGET_DIR=tt
   BURC_TABTARGET_DELIMITER=.
   BURC_TOOLS_DIR=Tools
   BURC_TEMP_ROOT_DIR=../temp-buk
   BURC_OUTPUT_ROOT_DIR=../output-buk
   BURC_LOG_LAST=last
   BURC_LOG_EXT=txt
   EOF
   ```

3. **Create TabTarget directory**:
   ```bash
   mkdir -p tt
   ```

4. **Create station file location**:
   ```bash
   mkdir -p ../station-files
   cat > ../station-files/burs.env <<'EOF'
   # Bash Utility Regime Station (BURS)
   # Developer/machine-level configuration for BUK

   BURS_LOG_DIR=../_logs_buk
   EOF
   ```

5. **Validate installation**:
   ```bash
   Tools/buk/burc_regime.sh validate .buk/burc.env
   Tools/buk/burs_regime.sh validate ../station-files/burs.env
   ```

---

## BUK Components

### BDU - Bash Dispatch Utility

**File**: `Tools/buk/bdu_BashDispatchUtility.sh`

**Purpose**: Central dispatch system that sets up execution environment and delegates to workbenches.

**Key Functions**:
- `bdu_launch` - Main entry point from launchers
- Environment setup (temp dirs, output dirs, logging)
- Sources BURS (station configuration)
- Color policy resolution
- Exit status propagation
- Transcript generation

**Environment Variables Set**:
- `BDU_TEMP_DIR` - Ephemeral temp directory for this invocation
- `BDU_OUTPUT_DIR` - Output directory for this invocation
- `BDU_NOW_STAMP` - Timestamp for this invocation
- `BDU_LOG_LAST` - Path to "last run" log
- `BDU_LOG_SAME` - Path to current log
- `BDU_LOG_HIST` - Path to historical log (timestamped)

---

### BCU - Bash Command Utility

**File**: `Tools/buk/bcu_BashCommandUtility.sh`

**Purpose**: Common command-line utilities and helpers.

**Key Functions**:
- Command execution helpers
- Output formatting
- Error handling patterns

---

### BTU - Bash Test Utility

**File**: `Tools/buk/btu_BashTestUtility.sh`

**Purpose**: Testing framework for bash scripts.

**Key Functions**:
- Test case definition
- Assertion helpers
- Test runner

---

### BVU - Bash Validation Utility

**File**: `Tools/buk/bvu_BashValidationUtility.sh`

**Purpose**: Type system for Config Regime validation.

**Validation Functions**:

**Atomic Types**:
- `bvu_string` - String with length constraints
- `bvu_xname` - System-safe identifier (xname = cross-platform name)
- `bvu_fqin` - Fully Qualified Image Name
- `bvu_bool` - Boolean (`true`/`false`)
- `bvu_decimal` - Decimal number with range constraints
- `bvu_ipv4` - IPv4 address
- `bvu_cidr` - CIDR notation
- `bvu_domain` - Domain name
- `bvu_port` - Port number (1-65535)

**List Types**:
- `bvu_ipv4_list` - Comma-separated IPv4 addresses
- `bvu_cidr_list` - Comma-separated CIDR blocks
- `bvu_domain_list` - Comma-separated domains

**Usage Example**:

```bash
# Validate a string variable
bvu_string "BURC_TABTARGET_DIR" "${BURC_TABTARGET_DIR}" 1 255 || exit 1

# Validate an xname variable
bvu_xname "BURC_LOG_LAST" "${BURC_LOG_LAST}" || exit 1
```

---

### BUW - BUK Workbench

**File**: `Tools/buk/buw_workbench.sh`

**Purpose**: Self-management workbench for BUK itself.

**Commands**:

**Launcher Management**:
- `buw-ll` - List launchers in `.buk/`
- `buw-lc <name>` - Create new launcher from template
- `buw-lv <name>` - Validate existing launcher

**TabTarget Management**:
- `buw-tc <workbench> <name>` - Create new tabtarget

**Regime Management**:
- `buw-rv` - Validate BURC and BURS regimes
- `buw-rr` - Render BURC and BURS configurations
- `buw-ri` - Show regime specification info

---

## Creating a New Workbench

### Step 1: Plan Your Workbench

Decide:
- **Workbench name**: `{prefix}w_workbench.sh` (e.g., `myw_workbench.sh`)
- **Commands**: What operations will it provide?
- **Config Regime** (optional): Does it need project-specific config?

### Step 2: Create Workbench Directory

```bash
mkdir -p Tools/myw
```

### Step 3: Create Workbench Script

```bash
cat > Tools/myw/myw_workbench.sh <<'EOF'
#!/bin/bash
set -euo pipefail

MYW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

myw_route() {
  local z_command="$1"
  shift

  case "${z_command}" in
    myw-hello)
      echo "Hello from myw workbench!"
      ;;
    myw-info)
      echo "Workbench info goes here"
      ;;
    *)
      echo "ERROR: Unknown command: ${z_command}" >&2
      exit 1
      ;;
  esac
}

myw_main() {
  local z_command="${1:-}"
  shift || true

  if [ -z "${z_command}" ]; then
    echo "ERROR: No command specified" >&2
    exit 1
  fi

  myw_route "${z_command}" "$@"
}

myw_main "$@"
EOF

chmod +x Tools/myw/myw_workbench.sh
```

### Step 4: Create Launcher

```bash
cat > .buk/launcher.myw_workbench.sh <<'EOF'
#!/bin/bash
z_project_root_dir="${0%/*}/.."
cd "${z_project_root_dir}" || exit 1
export BDU_REGIME_FILE="${z_project_root_dir}/.buk/burc.env"
source "${BDU_REGIME_FILE}" || exit 1

# Validate regimes
"${BURC_TOOLS_DIR}/buk/burc_regime.sh" validate "${z_project_root_dir}/.buk/burc.env" || {
  echo "ERROR: BURC validation failed" >&2
  "${BURC_TOOLS_DIR}/buk/burc_regime.sh" info
  exit 1
}

z_station_file="${z_project_root_dir}/${BURC_STATION_FILE}"
"${BURC_TOOLS_DIR}/buk/burs_regime.sh" validate "${z_station_file}" || {
  echo "ERROR: BURS validation failed: ${z_station_file}" >&2
  "${BURC_TOOLS_DIR}/buk/burs_regime.sh" info
  exit 1
}

export BDU_COORDINATOR_SCRIPT="${BURC_TOOLS_DIR}/myw/myw_workbench.sh"
exec "${BURC_TOOLS_DIR}/buk/bdu_BashDispatchUtility.sh" "${1##*/}" "${@:2}"
EOF

chmod +x .buk/launcher.myw_workbench.sh
```

### Step 5: Create TabTargets

```bash
cat > tt/myw-hello.SayHello.sh <<'EOF'
#!/bin/bash
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.myw_workbench.sh" \
  "${0##*/}" "${@}"
EOF

chmod +x tt/myw-hello.SayHello.sh

cat > tt/myw-info.ShowInfo.sh <<'EOF'
#!/bin/bash
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.myw_workbench.sh" \
  "${0##*/}" "${@}"
EOF

chmod +x tt/myw-info.ShowInfo.sh
```

### Step 6: Test Your Workbench

```bash
tt/myw-hello.SayHello.sh
tt/myw-info.ShowInfo.sh
```

---

## Reference Implementation: BURC/BURS

BURC and BURS are BUK's own Config Regimes, serving as both:
1. **Implementation** - Working regimes for BUK's operation
2. **Example** - Canonical demonstration of the Config Regime pattern

### BURC - Bash Utility Regime Configuration

**Purpose**: Project-level configuration defining repository structure.

**Assignment File**: `.buk/burc.env`

**Variables**:

| Variable | Type | Purpose |
|----------|------|---------|
| `BURC_STATION_FILE` | string | Path to developer's BURS file (relative to project root) |
| `BURC_TABTARGET_DIR` | string | Directory containing tabtarget scripts |
| `BURC_TABTARGET_DELIMITER` | string | Token separator in tabtarget filenames |
| `BURC_TOOLS_DIR` | string | Directory containing tool scripts |
| `BURC_TEMP_ROOT_DIR` | string | Parent directory for temp directories |
| `BURC_OUTPUT_ROOT_DIR` | string | Parent directory for output directories |
| `BURC_LOG_LAST` | xname | Basename for "last run" log file |
| `BURC_LOG_EXT` | xname | Extension for log files (without dot) |

**Example**:
```bash
BURC_STATION_FILE=../station-files/burs.env
BURC_TABTARGET_DIR=tt
BURC_TABTARGET_DELIMITER=.
BURC_TOOLS_DIR=Tools
BURC_TEMP_ROOT_DIR=../temp-buk
BURC_OUTPUT_ROOT_DIR=../output-buk
BURC_LOG_LAST=last
BURC_LOG_EXT=txt
```

**Key Insight**: BURC allows projects to organize directories differently while using the same BUK utilities.

---

### BURS - Bash Utility Regime Station

**Purpose**: Developer/machine-level configuration for personal preferences.

**Assignment File**: `../station-files/burs.env` (location defined by `BURC_STATION_FILE`)

**Variables**:

| Variable | Type | Purpose |
|----------|------|---------|
| `BURS_LOG_DIR` | string | Where this developer stores logs |

**Example**:
```bash
BURS_LOG_DIR=../_logs_buk
```

**Key Insight**: BURS is NOT checked into git. Each developer can have different logging preferences, parallelism settings, etc.

---

## Design Philosophy

### Portability

BUK is designed to be **graftable**: copy `Tools/buk/` into any project, configure via regime files, and it works. No modification to BUK code is needed.

### Immutability

The `Tools/buk/` directory remains unchanged across projects. All project-specific behavior comes from configuration, not code changes.

### Configuration as Data

Config Regimes treat configuration as structured data with types, validation, and documentation. This eliminates an entire class of runtime errors.

### Discoverability

TabTargets + tab completion make commands discoverable. Type `tt/buw-<TAB>` to see all BUK commands.

### Fail Fast

Launchers validate regimes before execution. This catches configuration errors immediately, with helpful error messages.

### BCG Compliance

All BUK utilities follow the Bash Console Guide (BCG) enterprise patterns:
- Bash 3.2 compatibility (macOS default)
- Multi-call script pattern
- Crash-fast error handling (`set -euo pipefail`)
- Braced, quoted variable expansion (`"${var}"`)
- Kindle/sentinel boilerplate for module loading

---

## Contributing

When extending BUK:

1. **Follow BCG patterns** - See `../cnmp_CellNodeMessagePrototype/lenses/bpu-BCG-BashConsoleGuide.md`
2. **Maintain portability** - No project-specific logic in `Tools/buk/`
3. **Use Config Regimes** - Configuration belongs in regime files, not code
4. **Write specifications** - Document new regimes in `{regime}_specification.md`
5. **Add validation** - Use BVU type system for all config variables
6. **Update README** - Keep this file as the authoritative source

---

## License

Copyright 2025 Scale Invariant, Inc.

Licensed under the Apache License, Version 2.0.

---

## Author

Brad Hyslop <bhyslop@scaleinvariant.org>
