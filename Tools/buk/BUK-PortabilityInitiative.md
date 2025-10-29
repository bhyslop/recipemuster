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

### The Config Regime Pattern

**Definition:** A Config Regime is a structured configuration system consisting of specification, assignment, validation, and rendering components, all unified by a namespace prefix.

**Core Components:**

1. **Namespace Identity**
   - Unique prefix (e.g., `RBRN_`, `BURC_`, `BURS_`) prevents variable collisions
   - UPPERCASE for variables, lowercase for tool functions
   - Applied consistently across all components

2. **Variable Structure**
   - **Regime Variables** - Named parameters with type constraints
   - **Assignment Values** - Actual configuration data conforming to spec
   - **Feature Groups** - Logically related variables
   - **Enable Flags** - Boolean controls for optional feature groups

3. **Type System**
   - Atomic types: `string`, `xname`, `fqin`, `bool`, `decimal`, `ipv4`, `cidr`, `domain`, `port`
   - List types: `ipv4_list`, `cidr_list`, `domain_list`
   - Each type validated with min/max constraints

4. **File Artifacts**
   - **Specification (.adoc)** - Defines all variables, types, constraints, relationships
   - **Assignment (.sh/.mk)** - Contains actual values (bilingual bash/make syntax)
   - **Validator (.sh)** - Enforces type rules and variable relationships
   - **Renderer (.sh)** - Pretty-prints configuration values
   - **Library (.sh)** - Shared validation functions across regimes

**BURC/BURS as Config Regimes:**

BURC and BURS are Config Regimes that demonstrate the pattern:
- ✅ Have namespace prefixes (`BURC_`, `BURS_`)
- ✅ Have assignment files (`.buk/burc.env`, `../station-files/burs.env`)
- ❌ Missing: specification documents defining allowed variables
- ❌ Missing: validation logic using type system
- ❌ Missing: renderer for human-readable display

**BUK's Dual Role:**
1. **Implementer** - BURC/BURS are working Config Regimes for BUK's own operation
2. **Enabler** - BUK provides tools (in BVU) for others to validate their own regimes

### Current Implementation Status

**Existing pieces:**
- `.buk/buk_launch_ccck.sh` - Launcher for ccck workbench
- `.buk/buk_launch_rbw.sh` - Launcher for rbw workbench
- `.buk/burc.env` - Regime configuration file
- `../station-files/burs.env` - Station file
- `Tools/buk/bdu_BashDispatchUtility.sh` - Dispatch system
- `Tools/buk/bcu_BashCommandUtility.sh` - Command utilities
- `Tools/buk/btu_BashTestUtility.sh` - Test utilities
- `Tools/buk/bvu_BashValidationUtility.sh` - Validation utilities

**Naming convention:**
- `BURC_*` - Bash Utility Regime Configuration (project-level)
- `BURS_*` - Bash Utility Regime Station (developer/machine-level)
- Rationale: Both BURC and BURS start with `BUR*`, making them clearly related

## Work Needed for Portability

### 1. ✅ Standardize Naming (COMPLETED)
- ✅ Renamed `BDRS_*` → `BURS_*` throughout codebase
- ✅ Renamed `bdrs.env` → `burs.env`
- ✅ Updated all references in BDU and burc.env

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

These documents informed the understanding of the Config Regime pattern and enterprise bash standards:

#### **PRIMARY REFERENCE: BCG - Bash Console Guide**
**Path**: `/Users/bhyslop/projects/cnmp_CellNodeMessagePrototype/lenses/bpu-BCG-BashConsoleGuide.md`

**Critical for BUK implementation** - Authoritative guide for enterprise bash patterns. While written for multi-user command scripts, patterns directly apply to BUK utilities.

**Key themes to apply:**
- Module architecture: implementation + CLI files, kindle/sentinel/furnish boilerplate
- Naming: `«PREFIX»_«NAME»` in SCREAMING_SNAKE_CASE for environment vars
- Variable expansion: Always `"${var}"` (braced, quoted)
- Enterprise safety: Crash-fast, explicit error handling, temp files over command substitution
- Bash 3.2 compatibility (no bash 4+ features)
- Module maturity checklist (lines 601-686) for auditing BUK utilities

**Application to BUK:** All BUK utilities must follow BCG patterns. BURC/BURS demonstrate Config Regime using these conventions.

#### TabTarget Documentation
**Paths**:
- `/Users/bhyslop/projects/cnmp_CellNodeMessagePrototype/lenses/lens-console-makefile-reqs.md` (lines 33-37)
- `/Users/bhyslop/projects/cnmp_CellNodeMessagePrototype/lenses/lens-mbc-MakefileBashConsole-cmodel.adoc` (lines 194-210)

**Key content:**
- Shell scripts in `tt/` directory providing user-friendly CLI interface
- Leverage terminal tab completion for discoverability
- Filename contains dot-delimited tokens (e.g., `buk-v.ValidateRegime.sh`)
- Delegate to dispatch script (BDU in BUK systems, makefile in MBC systems)
- Tokens parsed by `BURC_TABTARGET_DELIMITER` and passed as parameters

#### Config Regime Pattern Documentation

**CRR - Config Regime Requirements**
**Path**: `/Users/bhyslop/projects/recipebottle-admin/crg-CRR-ConfigRegimeRequirements.adoc`

**Essential reference** - Defines the complete Config Regime pattern and file structure.

**Key sections:**
- **Introduction** - Explains regime components (spec/assignment/validator/renderer/library) and their relationships
- **Spec Requirements** - Defines specification (.adoc) requirements and variable table format
- **Assignment Variable Names** section - Covers assignment file (.sh/.mk) naming and syntax rules
- **Core Terms** section (starting at `[[crg_regime]]`) - Definitions of regime, variable, value, prefix, group, enable flag
- **Core Type Definitions** section - Type system with atomic types (string, xname, fqin, bool, decimal, ipv4, cidr, domain, port) and list types

**Application to BUK:** CRR is the authoritative definition of Config Regime pattern. BUK should adopt this pattern for BURC/BURS and provide tooling (in BVU) to support it.

---

**RBRN - Regime Nameplate Specification**
**Path**: `/Users/bhyslop/projects/recipebottle-admin/rbw-RBRN-RegimeNameplate.adoc`

**Concrete example** - Shows a complete regime specification for Recipe Bottle service configuration.

**Key sections:**
- **Mapping section** (`// tag::mapping-section[]`) - Shows attribute references for config regime types (crg_atom_string, crg_atom_xname, etc.)
- **Feature Groups** section (starts with "Core Service Identity") - Demonstrates variable tables with Purpose/Type/Required/Constraints format
- **Core Term Definitions** section (starts with `[[term_bottle_service]]`) - Term definitions using AsciiDoc anchor pattern

**Application to BUK:** RBRN demonstrates how to structure a specification document. BURC and BURS specs should follow similar patterns.

---

**crgv.validate.sh - Config Regime Validator Library**
**Path**: `/Users/bhyslop/projects/brm_recipebottle/Tools/crgv.validate.sh`

**Implementation reference** - Provides validation functions for all regime types.

**Key functions:**
- `crgv_print_and_die()` - Error handling pattern
- `crgv_string()` - String validator with length constraints
- `crgv_xname()` - XName validator (system-safe identifiers)
- `crgv_fqin()` - FQIN validator (fully qualified image names)
- `crgv_bool()` and `crgv_opt_bool()` - Boolean validators (required and optional)
- `crgv_decimal()` and `crgv_opt_range()` - Decimal/range validators
- `crgv_ipv4()` and `crgv_opt_ipv4()` - IPv4 validators

**Application to BUK:** These validation functions should be integrated into BVU (Bash Validation Utility) as the standard type system for all Config Regimes.

---

**crgr.render.sh - Config Regime Renderer Library**
**Path**: `/Users/bhyslop/projects/brm_recipebottle/Tools/crgr.render.sh`

**Implementation reference** - Provides rendering functions for displaying regime values.

**Key functions:**
- `crgr_die()` - Error handling
- `crgr_render_header()` and `crgr_render_group()` - Section headers with different formatting
- `crgr_render_value()` - Single value rendering with formatted columns
- `crgr_render_boolean()` - Boolean rendering with enabled/disabled text
- `crgr_render_list()` - List rendering with indentation

**Application to BUK:** Could be integrated into BCU (Bash Command Utility) or remain as separate utility.

---

#### AXLA - Lexicon (Config Regime Definition)
**Path**: `/Users/bhyslop/projects/cnmp_CellNodeMessagePrototype/lenses/axl-AXLA-Lexicon.adoc`

**Key content:**
- **Regime definition** at anchor `[[axo_regime]]` - Defines regime as "Configuration boundary with consistent namespace prefix"
- References RBAGS regime patterns (RBRR, RBRA, RBRP) as origin of the pattern


## Current Status - Ready for Implementation

### BUK Regime Files - Naming Convention Established

**File Naming Pattern:**
- **Prefix**: `bu` = Bash Utility namespace (will apply to ALL BUK scripts)
- **Function token**: Single letter indicating purpose
  - `v` = validates
  - `p` = proclaims (spec)
  - `d` = dispatch
  - `c` = command
  - `t` = test
  - `u` = utility (validation)
- **Target token**: Identifies what is being acted upon
  - `s` = station (BURS)
  - `c` = config (BURC)

### New Regime Files to Create (in Tools/buk/):

**Validators:**
- `buvs_station.sh` - Validates BURS (station regime) at `../station-files/burs.env`
- `buvc_config.sh` - Validates BURC (config regime) at `.buk/burc.env`

**Specifications:**
- `bups_station.md` - Proclaims BURS spec (markdown format)
- `bupc_config.md` - Proclaims BURC spec (markdown format)

### Future Renames (not now, but inevitable):
- `bdu_BashDispatchUtility.sh` → `bud_dispatch.sh`
- `bcu_BashCommandUtility.sh` → `buc_command.sh`
- `btu_BashTestUtility.sh` → `but_test.sh`
- `bvu_BashValidationUtility.sh` → `buv_validation.sh`

### Key Decisions Made:
- ✅ All files in `Tools/buk/` (no subdirectories until necessary)
- ✅ Specs in markdown (not asciidoc - keeping pattern as internal trade secret)
- ✅ "Proclaim" verb for spec files
- ✅ Consistent `s`/`c` suffix for station/config distinction
- ✅ BVU already implements Config Regime type system (no need to copy from crgv.validate.sh)

### Regime Variables Currently Known:

**BURC (Config Regime):**
- `BURC_STATION_FILE` - Path to station file
- `BURC_TABTARGET_DIR` - TabTarget directory (e.g., `tt/`)
- `BURC_TABTARGET_DELIMITER` - Token separator in tabtarget names
- `BURC_TOOLS_DIR` - Location of tool scripts
- `BURC_TEMP_ROOT_DIR` - Where to create temporary directories
- `BURC_OUTPUT_ROOT_DIR` - Where to place command outputs
- `BURC_LOG_LAST` - Name for "last run" log file
- `BURC_LOG_EXT` - Log file extension

**BURS (Station Regime):**
- `BURS_LOG_DIR` - Where this developer keeps logs
- `BURS_MAX_JOBS` - (proposed) Parallelism preference for this machine
- (Other machine-specific settings TBD)

## Next Session TODO

**PRIORITY: Create the four regime files**

1. Draft `bups_station.md` - BURS specification
2. Draft `bupc_config.md` - BURC specification
3. Implement `buvs_station.sh` - BURS validator using BVU functions
4. Implement `buvc_config.sh` - BURC validator using BVU functions
5. Test validators against existing `.buk/burc.env` and `../station-files/burs.env`

**LATER:**
1. ✅ ~~Decide on exact naming: stick with BDRS or rename to BURS?~~ - COMPLETED: Renamed to BURS
2. Review and approve the three-layer architecture
3. ✅ ~~Consider: should burc.env stay at `Tools/burc.env` or move to `.buk/burc.env`?~~ - DECIDED: Stays at `.buk/burc.env`
4. Plan the documentation structure for `Tools/buk/README.md`
5. Consider renaming existing BUK utilities to new convention (bud_dispatch.sh, etc.)

## Notes

- The Config Regime pattern has been in use across multiple projects (RBAGS, RBW, CCCK)
- BUK's role is to **formalize and document** this pattern
- BURC/BURS are the "reference implementation" that shows the pattern in action
- The portable design allows BUK to be "lifted" into any project tree
- Critical success factor: excellent documentation so others can replicate the pattern
