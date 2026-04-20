#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# CMW Workbench - Concept Model Kit installation and management
#
# Commands:
#   cmk-i  Install Concept Model Kit
#   cmk-u  Uninstall Concept Model Kit

set -euo pipefail

ZCMW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${BURD_BUK_DIR}/buc_command.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

# Configuration - can be overridden via environment
ZCMW_KIT_PATH="${ZCMW_KIT_PATH:-Tools/cmk/README.md}"
ZCMW_KIT_DIR="$(dirname "${ZCMW_KIT_PATH}")"

# Hardcoded defaults
ZCMW_LENSES_DIR="lenses"
ZCMW_UPSTREAM_REMOTE="OPEN_SOURCE_UPSTREAM"

######################################################################
# Command Emitters - one function per command file
#
# Each emits to stdout; caller redirects to file

zcmw_emit_prep_pr() {
  {
    echo "---"
    echo "description: Prepare branch for upstream PR contribution"
    echo "---"
    echo ""
    echo "You are preparing a PR branch for upstream contribution, stripping proprietary content."
    echo ""
    echo "**Configuration:**"
    echo "- Lenses directory: ${ZCMW_LENSES_DIR}"
    echo "- Kit directory: ${ZCMW_KIT_DIR}"
    echo "- Kit path: ${ZCMW_KIT_PATH}"
    echo "- Upstream remote: ${ZCMW_UPSTREAM_REMOTE}"
    echo ""
    echo "**Execute these steps:**"
    echo ""
    echo "0. **Request permissions upfront:**"
    echo "   - Ask user for permission to execute all git operations needed"
    echo "   - Get approval before proceeding"
    echo ""
    echo "1. **Verify develop is clean and pushed:**"
    echo "   - Check \`git status\` on develop branch"
    echo "   - Push any uncommitted changes to origin/develop"
    echo ""
    echo "2. **Sync main with upstream:**"
    echo "   - \`git checkout main\`"
    echo "   - \`git fetch ${ZCMW_UPSTREAM_REMOTE}\`"
    echo "   - \`git pull ${ZCMW_UPSTREAM_REMOTE} main\`"
    echo "   - If pull fails, **ABORT** and ask user to resolve"
    echo "   - \`git push origin main\`"
    echo ""
    echo "3. **Determine candidate branch (ask user):**"
    echo "   - List existing local candidate branches: \`git branch -a | grep candidate-\`"
    echo "   - Find highest batch number (NNN) and revision (R) from local branches"
    echo "   - Ask user using AskUserQuestion tool:"
    echo "     - **New delivery**: Previous PR was merged → create candidate-{N+1}-1"
    echo "     - **Reissue/fix**: Previous PR pending or rejected → create candidate-{N}-{R+1}"
    echo "   - If no prior candidates exist, default to candidate-001-1"
    echo "   - Confirm the branch name with user before proceeding"
    echo ""
    echo "4. **Create candidate branch:**"
    echo "   - \`git checkout -b candidate-NNN-R main\`"
    echo ""
    echo "5. **Squash merge develop:**"
    echo "   - Show commits: \`git log main..develop --oneline\`"
    echo "   - Execute: \`git merge --squash develop\`"
    echo ""
    echo "6. **Strip proprietary content:**"
    echo "   - \`git rm -rf --ignore-unmatch .claude/\`"
    echo "   - \`git rm -rf --ignore-unmatch ${ZCMW_LENSES_DIR}/\`"
    echo "   - Verify removal with \`git ls-files\`"
    echo ""
    echo "7. **Generate commit:**"
    echo "   - Analyze changes for consolidated message"
    echo "   - Filter out commits that only touch stripped files"
    echo "   - Create commit (no attribution footer)"
    echo ""
    echo "8. **Final review:**"
    echo "   - Show \`git log -1 --stat\`"
    echo "   - Display push instructions"
    echo "   - **STOP** - user reviews and pushes manually"
    echo ""
    echo "**Important:**"
    echo "- Be methodical, show output at each step"
    echo "- Stop immediately on errors"
    echo "- User maintains control over final push"
  }
}

## RETIRED: zcmw_emit_claudemd_section
## CMK context is now maintained as Tools/cmk/cmk-claude-context.md
## and included via @-directive in CLAUDE.md. No patching needed.

######################################################################
# Main Commands

cmw_install() {
  buc_step "Clearing previous installation"
  cmw_uninstall

  buc_step "Creating directory structure"
  mkdir -p ".claude/commands"
  mkdir -p "${ZCMW_LENSES_DIR}"

  buc_step "Emitting command files"
  zcmw_emit_prep_pr     > ".claude/commands/cma-prep-pr.md"

  buc_step "Verifying CLAUDE.md include directive"
  if ! grep -q '@Tools/cmk/cmk-claude-context.md' CLAUDE.md 2>/dev/null; then
    buc_warn "CLAUDE.md does not contain '@Tools/cmk/cmk-claude-context.md' include directive"
    buc_warn "Add this line to CLAUDE.md: @Tools/cmk/cmk-claude-context.md"
  fi

  buc_success "Concept Model Kit installed"
  echo ""
  echo "Restart Claude Code for new commands to become available."
}

cmw_uninstall() {
  buc_step "Removing command files"
  rm -f .claude/commands/cma-*.md

  buc_success "Concept Model Kit uninstalled (context file preserved in kit directory)"
}

## RETIRED: zcmw_patch_claudemd, zcmw_unpatch_claudemd
## CLAUDE.md patching replaced by @-directive includes.
## Context file: Tools/cmk/cmk-claude-context.md

######################################################################
# Routing

cmw_route() {
  local z_command="${1:-}"
  shift || true

  case "${z_command}" in
    cmk-i) cmw_install ;;
    cmk-u) cmw_uninstall ;;
    *)     buc_die "Unknown command: ${z_command}\nAvailable: cmk-i (install), cmk-u (uninstall)" ;;
  esac
}

cmw_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  cmw_route "${z_command}" "$@"
}

cmw_main "$@"

# eof
