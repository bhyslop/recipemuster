# BUK Portability Architecture

## Overview

BUK (Bash Utility Kit) is a portable, graftable module that can be dropped into any project tree and configured through the Config Regime pattern.

## Three-Layer Architecture

### 1. BUK Core (`Tools/buk/*.sh`)
- Portable utilities with no project-specific knowledge
- Can be copied wholesale to any project
- Provides the infrastructure: dispatch, command utilities, testing, validation

### 2. BURC - Bash Utility Regime Configuration (`.buk/burc.env`)
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

### 3. BURS - Bash Utility Regime Station (`../<configurable>/burs.env`)
- **Developer/machine-level configuration** (NOT in git)
- Location defined by `BURC_STATION_FILE`
- Personal preferences for this developer's machine
- Variables:
  - `BURS_LOG_DIR` - Where this developer keeps logs
  - `BURS_MAX_JOBS` - Parallelism preference for this machine

## Key Design Principles

1. **Tools/buk/ is immutable** - The `Tools/buk/` directory contains portable BUK utilities that remain unchanged across projects

2. **burc.env configures BUK for a repo** - The `.buk/burc.env` file adapts the immutable BUK utilities to a specific project's structure

3. **BURC controls the layout** - The project maintainer decides where developers put personal configs by setting `BURC_STATION_FILE`

4. **BURC/BURS demonstrate the pattern** - Working regimes that follow Config Regime conventions

5. **Config Regime is a BUK-owned pattern** - Other projects using BUK create their own regimes (RBRR, RBRN, etc.)

6. **Clean separation of concerns**:
   - BUK utilities (`Tools/buk/*.sh`) = portable, project-agnostic
   - BURC (`.buk/burc.env`) = project conventions (shared by team)
   - BURS (`../station-files/burs.env`) = individual developer preferences (not shared)

## The Config Regime Pattern

**Definition:** A Config Regime is a structured configuration system consisting of specification, assignment, validation, and rendering components, all unified by a namespace prefix.

### Core Components

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

### BUK's Dual Role
1. **Implementer** - BURC/BURS are working Config Regimes for BUK's own operation
2. **Enabler** - BUK provides tools (in BVU) for others to validate their own regimes

## Reference Documents

### BCG - Bash Console Guide
**Path**: `Tools/buk/lenses/bpu-BCG-BashConsoleGuide.md`

Authoritative guide for enterprise bash patterns. Key themes:
- Module architecture: implementation + CLI files, kindle/sentinel/furnish boilerplate
- Naming: `«PREFIX»_«NAME»` in SCREAMING_SNAKE_CASE for environment vars
- Variable expansion: Always `"${var}"` (braced, quoted)
- Enterprise safety: Crash-fast, explicit error handling, temp files over command substitution
- Bash 3.2 compatibility (no bash 4+ features)
- Module maturity checklist for auditing BUK utilities

### External References (paths may be stale)

**TabTarget Documentation** (cnmp lenses):
- `lens-console-makefile-reqs.md` - TabTarget concept
- `lens-mbc-MakefileBashConsole-cmodel.adoc` - MBC implementation

**Config Regime Documentation** (recipebottle-admin):
- `crg-CRR-ConfigRegimeRequirements.adoc` - Authoritative Config Regime definition
- `rbw-RBRN-RegimeNameplate.adoc` - Example regime specification

**Validation/Rendering Libraries** (brm_recipebottle):
- `crgv.validate.sh` - Validation functions for regime types
- `crgr.render.sh` - Rendering functions for regime display

**Lexicon** (cnmp lenses):
- `axl-AXLA-Lexicon.adoc` - Regime definition at anchor `[[axo_regime]]`

## Philosophy

- The Config Regime pattern has been in use across multiple projects (RBAGS, RBW, CCCK)
- BUK's role is to **formalize and document** this pattern
- BURC/BURS demonstrate the pattern for BUK's own configuration needs
- The portable design allows BUK to be "lifted" into any project tree
