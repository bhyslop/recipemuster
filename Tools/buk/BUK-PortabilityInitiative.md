# BUK Portability Initiative

## Project Goal

Make the Bash Utility Kit (BUK) a fully portable, graftable module that can be dropped into any project tree and configured through the Config Regime pattern.

## Current Understanding

### BUK as a Graftable Module

BUK is designed to be copied wholesale into any project at a standard location (e.g., `Tools/buk/`) and then configured through two layers of regime configuration files:

```
any-project-root/
├── Tools/
│   └── buk/                    # Drop BUK here (portable, reusable)
│       ├── bcu_BashCommandUtility.sh
│       ├── bdu_BashDispatchUtility.sh
│       ├── btu_BashTestUtility.sh
│       └── bvu_BashValidationUtility.sh
├── Tools/burc.env              # Project structure config (checked into git)
└── ../<configurable>/burs.env  # Developer machine config (NOT in git)
```

### Three-Layer Architecture

#### 1. BUK Core (`Tools/buk/*.sh`)
- Portable utilities with no project-specific knowledge
- Can be copied wholesale to any project
- Provides the infrastructure: dispatch, command utilities, testing, validation

#### 2. BURC - Bash Utility Regime Configuration (`Tools/burc.env`)
- **Project-level configuration** (committed to git)
- Defines "how THIS project organizes things"
- Variables:
  - `BURC_STATION_FILE` - Path to developer's personal config
  - `BURC_TABTARGET_DIR` - Where tabtarget scripts live (e.g., `tt/`)
  - `BURC_TABTARGET_DELIMITER` - Token separator in tabtarget names
  - `BURC_TOOLS_DIR` - Location of tool scripts
  - `BURC_TEMP_ROOT_DIR` - Where to create temporary directories
  - `BURC_OUTPUT_ROOT_DIR` - Where to place command outputs
  - `BURC_LOG_LAST` - Name for "last run" log file
  - `BURC_LOG_EXT` - Log file extension

#### 3. BURS - Bash Utility Regime Station (`../<wherever>/burs.env`)
- **Developer/machine-level configuration** (NOT in git)
- Location defined by `BURC_STATION_FILE`
- Personal preferences for this developer's machine
- Variables:
  - `BURS_LOG_DIR` - Where this developer keeps logs
  - `BURS_MAX_JOBS` - Parallelism preference for this machine
  - (Other machine-specific settings)

### Key Design Insights

1. **BURC controls the layout** - The project maintainer decides where developers put personal configs by setting `BURC_STATION_FILE`

2. **BURC/BURS are exemplars** - They demonstrate the Config Regime pattern that BUK defines and promotes

3. **Config Regime is a BUK-owned pattern** - Other projects using BUK create their own regimes (RBWR, CCCR, etc.)

4. **Clean separation of concerns**:
   - BUK utilities = portable, project-agnostic
   - BURC = project conventions (shared by team)
   - BURS = individual developer preferences (not shared)

### The Config Regime Pattern (Abstract)

The Config Regime is a general pattern that BUK defines, documents, and validates:

- **What it is**: Configuration boundary with consistent namespace prefix
- **Why**: Establishes configuration scope and variable namespace
- **Naming convention**: `<PREFIX>_<NAME>` in SCREAMING_SNAKE_CASE
- **File organization**: Separate files for different scopes (project vs. machine)
- **Validation**: BUK provides utilities to validate any regime

### Current Implementation Status

**Existing pieces:**
- `.buk/buk_launch_ccck.sh` - Launcher for ccck workbench
- `.buk/buk_launch_rbw.sh` - Launcher for rbw workbench
- `.buk/burc.env` - Regime configuration file
- `../station-files/bdrs.env` - Station file (should be renamed to burs.env)
- `Tools/buk/bdu_BashDispatchUtility.sh` - Dispatch system
- `Tools/buk/bcu_BashCommandUtility.sh` - Command utilities
- `Tools/buk/btu_BashTestUtility.sh` - Test utilities
- `Tools/buk/bvu_BashValidationUtility.sh` - Validation utilities

**Naming inconsistency found:**
- Current: `BDRS_*` (Bash Dispatch Regime Station)
- Should be: `BURS_*` (Bash Utility Regime Station)
- Rationale: Both BURC and BURS start with `BUR*`, making them clearly related

## Work Needed for Portability

### 1. Standardize Naming
- Rename `BDRS_*` → `BURS_*` throughout codebase
- Rename `bdrs.env` → `burs.env`
- Update all references in BDU and launchers

### 2. Create BUK Documentation

Create `Tools/buk/README.md` with sections:

1. **Config Regime Pattern** (conceptual)
   - What is a regime?
   - Why use regimes?
   - Naming conventions
   - File organization
   - Validation approach

2. **BUK Regimes Reference** (concrete examples)
   - BURC variables and meanings
   - BURS variables and meanings
   - How BDU uses them
   - These serve as canonical examples

3. **Installing BUK in Your Project** (tutorial)
   - Copy BUK to `Tools/buk/`
   - Create `Tools/burc.env` from template
   - Create station file location
   - Setup developer's `burs.env`

4. **Creating Project-Specific Regimes** (advanced)
   - How to create your own regimes (like CCCR, RBWR)
   - Integration with BDU
   - Validation patterns

### 3. Add Regime Validation to BVU

Extend `bvu_BashValidationUtility.sh` with generic regime validation:

```bash
# Schema validation for BUK regime configuration
bvu_validate_buk_regime() {
  local regime_file=$1
  # Validate BURC variables
}

bvu_validate_buk_station() {
  local station_file=$1
  # Validate BURS variables
}

# Generic regime validation for any prefix
bvu_validate_regime() {
  local regime_file=$1
  local prefix=$2
  # Generic validation logic
}
```

### 4. Create BUK Self-Management Workbench

Create `Tools/buk/bukw_workbench.sh` with commands:
- `buk-v` - Validate regime configuration
- `buk-vs` - Validate station configuration
- `buk-l` - List all launchers in `.buk/`
- `buk-c` - Create new launcher (wizard/template)
- `buk-i` - Initialize BUK in a new project
- `buk-h` - Show help/documentation

Create corresponding tabtargets in `tt/`:
- `tt/buk-v.ValidateRegime.sh`
- `tt/buk-vs.ValidateStation.sh`
- `tt/buk-l.ListLaunchers.sh`
- `tt/buk-c.CreateLauncher.sh`
- `tt/buk-i.InitializeProject.sh`
- `tt/buk-h.Help.sh`

### 5. Create Installation/Bootstrap Script

Create `Tools/buk/buk_init.sh` that:
1. Checks if `Tools/burc.env` exists
2. If not, creates from template with TODOs
3. Prompts for station file location
4. Creates station file template
5. Validates installation
6. Creates `.buk/` directory if needed
7. Reports what the developer needs to do next

### 6. Create Template Files

- `Tools/buk/burc.env.template` - BURC template with comments
- `Tools/buk/burs.env.template` - BURS template with comments
- `Tools/buk/launcher.template.sh` - Template for creating new launchers

### 7. Update Existing Launchers

Make `.buk/buk_launch_*.sh` files reference the standardized names:
- Source `burc.env` (not hardcoded paths)
- Reference `BURS_*` variables (not `BDRS_*`)

## Reference Documents

### From CNMP (CellNodeMessagePrototype) Lenses Directory

These documents informed the understanding of the Config Regime pattern:

#### BCG - Bash Console Guide
**Path**: `/Users/bhyslop/projects/cnmp_CellNodeMessagePrototype/lenses/bpu-BCG-BashConsoleGuide.md`

**Key content:**
- Lines 90-110: Template showing `«PREFIX»_REGIME_FILE` pattern
- Line 99: `bcu_doc_env "«PREFIX»_REGIME_FILE " "Module specific configuration file"`
- Line 102: `source "${«PREFIX»_REGIME_FILE}" || bcu_die "Failed to source regime file"`
- Line 136: `bvu_file_exists "${«PREFIX»_REGIME_FILE}"  # If present`
- Line 139: `source "${REGIME_CRED_FILE}" || bcu_die "Failed to source credentials"`
- Lines 385-395: **Naming convention table** showing all patterns:
  - Environment vars: `«PREFIX»_«NAME»` in SCREAMING_SNAKE_CASE
  - Example: `RBV_REGIME_FILE`
  - Used in both CLI and Implementation modules

**Template patterns documented:**
- Furnish functions (`z«prefix»_furnish`)
- Kindle functions (`z«prefix»_kindle`)
- Module variables (`Z«PREFIX»_«NAME»`)
- Environment variables (`«PREFIX»_«NAME»`)

#### AXLA - Lexicon
**Path**: `/Users/bhyslop/projects/cnmp_CellNodeMessagePrototype/lenses/axl-AXLA-Lexicon.adoc`

**Key content:**
- Lines 113-114: Attribute mapping for regime term
  ```
  :axo_regime:   <<axo_regime,Regime>>
  :axo_regime_s: <<axo_regime,Regimes>>
  ```
- Lines 476-480: **Regime definition**
  ```
  [[axo_regime]]
  {axo_regime}::
  Configuration boundary with consistent namespace prefix.
  From {xref_RBAGS} regime patterns (RBRR, RBRA, RBRP).
  Establishes configuration scope and variable namespace.
  ```

**Key insight:** The Config Regime pattern originates from RBAGS (Admin Google Spec) and shows up as RBRR, RBRA, RBRP implementations.

## Next Session TODO

1. Decide on exact naming: stick with BDRS or rename to BURS?
2. Review and approve the three-layer architecture
3. Prioritize which pieces to implement first
4. Consider: should burc.env stay at `Tools/burc.env` or move to `.buk/burc.env`?
5. Plan the documentation structure for `Tools/buk/README.md`

## Notes

- The Config Regime pattern has been in use across multiple projects (RBAGS, RBW, CCCK)
- BUK's role is to **formalize and document** this pattern
- BURC/BURS are the "reference implementation" that shows the pattern in action
- The portable design allows BUK to be "lifted" into any project tree
- Critical success factor: excellent documentation so others can replicate the pattern
