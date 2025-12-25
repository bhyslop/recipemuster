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
# JJW Workbench - Job Jockey installation and management
#
# Commands:
#   jjk-i  Install Job Jockey
#   jjk-u  Uninstall Job Jockey (preserves .claude/jjm/ state)

set -euo pipefail

ZJJW_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${ZJJW_SCRIPT_DIR}/../buk/buc_command.sh"

# Configuration - can be overridden via environment
ZJJW_TARGET_DIR="${ZJJW_TARGET_DIR:-.}"
ZJJW_KIT_PATH="${ZJJW_KIT_PATH:-Tools/jjk/README.md}"

######################################################################
# Command Emitters - one function per command file
#
# Each emits to stdout; caller redirects to file

zjjw_emit_heat_resume() {
  {
    echo "You are resuming the current Job Jockey heat."
    echo ""
    echo "Configuration:"
    echo "- Target repo dir: ${ZJJW_TARGET_DIR}"
    echo "- Kit path: ${ZJJW_KIT_PATH}"
    echo ""
    echo "Steps:"
    echo ""
    echo "1. Check for heat files in .claude/jjm/current/"
    echo "   - Look for files matching pattern \`jjh_b*.md\`"
    echo ""
    echo "2. Branch based on heat count:"
    echo ""
    echo "   **If 0 heats:**"
    echo "   - Announce: \"No active heat found\""
    echo "   - Ask: \"Would you like to start a new heat or promote an itch?\""
    echo "   - Stop and wait for user direction"
    echo ""
    echo "   **If 1 heat:**"
    echo "   - Read the heat file"
    echo "   - Display:"
    echo "     - Heat name (from filename)"
    echo "     - Brief summary from Context section"
    echo "     - Current pace (from ## Current section)"
    echo "   - Ask \"Ready to continue?\" or similar"
    echo ""
    echo "   **If 2+ heats:**"
    echo "   - List all heats by name"
    echo "   - Ask: \"Which heat would you like to work on?\""
    echo "   - Wait for selection, then display as above"
    echo ""
    echo "3. Example output format:"
    echo "   \`\`\`"
    echo "   Resuming heat: **BUK Utility Rename**"
    echo ""
    echo "   Current pace: **Update internal functions**"
    echo ""
    echo "   Ready to continue?"
    echo "   \`\`\`"
    echo ""
    echo "Error handling: If .claude/jjm/current/ doesn't exist, announce issue and stop."
  }
}

zjjw_emit_heat_retire() {
  {
    echo "You are retiring a completed Job Jockey heat."
    echo ""
    echo "Configuration:"
    echo "- Target repo dir: ${ZJJW_TARGET_DIR}"
    echo "- Kit path: ${ZJJW_KIT_PATH}"
    echo ""
    echo "Steps:"
    echo ""
    echo "1. Check for heat files in .claude/jjm/current/"
    echo "   - If 0 heats: announce \"No active heat to retire\" and stop"
    echo "   - If 2+ heats: ask which one to retire"
    echo ""
    echo "2. Read the heat file and verify completion:"
    echo "   - Check for any incomplete paces (\`- [ ]\` items)"
    echo "   - If incomplete paces exist:"
    echo "     - List them"
    echo "     - Ask: \"These paces are incomplete. Mark them as discarded, or continue working?\""
    echo "     - If user wants to discard: mark them with \`- [~]\` prefix and note \"(discarded)\""
    echo "     - If user wants to continue: stop retirement process"
    echo ""
    echo "3. Determine filenames:"
    echo "   - Current filename pattern: \`jjh_bYYMMDD-description.md\`"
    echo "   - Extract the begin date (bYYMMDD) and description"
    echo "   - Generate retire date: today's date as rYYMMDD"
    echo "   - New filename: \`jjh_bYYMMDD-rYYMMDD-description.md\`"
    echo ""
    echo "   Example:"
    echo "   - Before: \`jjh_b251108-buk-rename.md\`"
    echo "   - After: \`jjh_b251108-r251127-buk-rename.md\`"
    echo ""
    echo "4. Move the file:"
    echo "   \`\`\`bash"
    echo "   git mv .claude/jjm/current/jjh_b251108-buk-rename.md .claude/jjm/retired/jjh_b251108-r251127-buk-rename.md"
    echo "   \`\`\`"
    echo ""
    echo "5. Commit the retirement (JJ state repo only, no push):"
    echo "   \`\`\`bash"
    echo "   git commit -m \"JJA: heat-retire - [heat description]\""
    echo "   \`\`\`"
    echo ""
    echo "6. Report completion:"
    echo "   \`\`\`"
    echo "   Retired heat: **BUK Rename**"
    echo "   - Began: 2025-11-08"
    echo "   - Retired: 2025-11-27"
    echo "   - File: .claude/jjm/retired/jjh_b251108-r251127-buk-rename.md"
    echo "   \`\`\`"
    echo ""
    echo "7. Offer next steps:"
    echo "   - \"Would you like to start a new heat or promote an itch from jji_itch.md?\""
    echo ""
    echo "Error handling: If paths wrong or files missing, announce issue and stop."
  }
}

zjjw_emit_pace_find() {
  {
    echo "You are showing the current pace from the active Job Jockey heat."
    echo ""
    echo "Configuration:"
    echo "- Target repo dir: ${ZJJW_TARGET_DIR}"
    echo "- Kit path: ${ZJJW_KIT_PATH}"
    echo ""
    echo "Steps:"
    echo ""
    echo "1. Check for heat files in .claude/jjm/current/"
    echo "   - If 0 heats: announce \"No active heat\" and stop"
    echo "   - If 2+ heats: ask which one"
    echo ""
    echo "2. Read the heat file"
    echo ""
    echo "3. Find the current pace:"
    echo "   - Look for the ## Current section"
    echo "   - Extract the pace title (bold text) and any working notes"
    echo ""
    echo "4. Determine the mode:"
    echo "   - Look for \`mode: manual\` or \`mode: delegated\` in the pace spec"
    echo "   - Default to \`manual\` if not specified"
    echo ""
    echo "5. Display:"
    echo "   \`\`\`"
    echo "   Current pace: **[title]** [mode]"
    echo ""
    echo "   [working notes if any]"
    echo "   \`\`\`"
    echo ""
    echo "Error handling: If no current pace found, announce \"No current pace - check ## Remaining for next pace\""
  }
}

zjjw_emit_pace_left() {
  {
    echo "You are listing all remaining paces in the current Job Jockey heat."
    echo ""
    echo "Configuration:"
    echo "- Target repo dir: ${ZJJW_TARGET_DIR}"
    echo "- Kit path: ${ZJJW_KIT_PATH}"
    echo ""
    echo "Steps:"
    echo ""
    echo "1. Check for heat files in .claude/jjm/current/"
    echo "   - If 0 heats: announce \"No active heat\" and stop"
    echo "   - If 2+ heats: ask which one"
    echo ""
    echo "2. Read the heat file"
    echo ""
    echo "3. Collect remaining paces:"
    echo "   - Current pace from ## Current section"
    echo "   - Future paces from ## Remaining section"
    echo ""
    echo "4. For each pace, determine mode:"
    echo "   - Look for \`mode: manual\` or \`mode: delegated\`"
    echo "   - Default to \`manual\` if not specified"
    echo ""
    echo "5. Display terse list:"
    echo "   \`\`\`"
    echo "   Remaining paces (N):"
    echo "   1. [mode] Current pace title"
    echo "   2. [mode] Next pace title"
    echo "   3. [mode] Another pace title"
    echo "   ..."
    echo "   \`\`\`"
    echo ""
    echo "Error handling: If no remaining paces, announce \"All paces complete - ready to retire heat?\""
  }
}

zjjw_emit_pace_add() {
  {
    echo "You are adding a new pace to the current Job Jockey heat."
    echo ""
    echo "Configuration:"
    echo "- Target repo dir: ${ZJJW_TARGET_DIR}"
    echo "- Kit path: ${ZJJW_KIT_PATH}"
    echo ""
    echo "Steps:"
    echo ""
    echo "1. Check for heat files in .claude/jjm/current/"
    echo "   - If 0 heats: announce \"No active heat\" and stop"
    echo "   - If 2+ heats: ask which one"
    echo ""
    echo "2. Read the heat file to understand context and existing paces"
    echo ""
    echo "3. Analyze the heat context and existing paces"
    echo ""
    echo "4. Propose a new pace:"
    echo "   - Title (bold format)"
    echo "   - Optional description"
    echo "   - Position in the list (explain reasoning)"
    echo "   - Default mode: manual"
    echo ""
    echo "5. Present proposal:"
    echo "   \`\`\`"
    echo "   I propose adding pace '**[title]**' after '[existing pace]'"
    echo "   because [reasoning]."
    echo "   Should I add it there?"
    echo "   \`\`\`"
    echo ""
    echo "6. Wait for user approval or amendment"
    echo ""
    echo "7. If approved, update the heat file:"
    echo "   - Add pace to ## Remaining section at proposed position"
    echo "   - Include \`mode: manual\` (default)"
    echo ""
    echo "8. Do NOT commit (preparatory work, accumulates until /jja-pace-wrap or /jja-sync)"
    echo ""
    echo "9. Report what was added"
    echo ""
    echo "Error handling: If paths wrong or files missing, announce issue and stop."
  }
}

zjjw_emit_pace_refine() {
  {
    echo "You are helping refine a pace's specification in the current Job Jockey heat."
    echo ""
    echo "Configuration:"
    echo "- Target repo dir: ${ZJJW_TARGET_DIR}"
    echo "- Kit path: ${ZJJW_KIT_PATH}"
    echo ""
    echo "Steps:"
    echo ""
    echo "1. Check for current heat in .claude/jjm/current/"
    echo "   - If no heat: announce \"No active heat\" and stop"
    echo "   - If multiple: ask which one"
    echo ""
    echo "2. Identify which pace to refine (infer from context or ask)"
    echo ""
    echo "3. Read the pace and assess delegatability:"
    echo "   - Mechanical vs judgment: Are steps explicit or require decisions?"
    echo "   - Scope clarity: Are boundaries well-defined?"
    echo "   - Verifiability: Can success be checked objectively?"
    echo "   - Risk: What happens if it goes wrong?"
    echo ""
    echo "4. Make a recommendation with brief rationale:"
    echo "   \`\`\`"
    echo "   Refining pace: **[title]**"
    echo ""
    echo "   Current state: [summary of what's defined]"
    echo ""
    echo "   **Recommendation: [delegated (model-hint) | manual]**"
    echo "   - [1-3 bullet points explaining why]"
    echo ""
    echo "   Accept recommendation, or override?"
    echo "   \`\`\`"
    echo ""
    echo "5. If user accepts delegated, ensure spec covers:"
    echo "   - Objective: What specifically to achieve?"
    echo "   - Scope: What files/systems to touch or avoid?"
    echo "   - Success: How do we know it's done?"
    echo "   - Failure: What to do if stuck? (stop/report/retry)"
    echo "   - Model hint: haiku-ok / needs-sonnet / needs-opus"
    echo ""
    echo "   Draft spec from available context. Ask only for missing elements."
    echo ""
    echo "6. Final clarity check (delegated only):"
    echo "   Read the spec as if you have no prior context. Assess:"
    echo "   - Objective: clear or ambiguous?"
    echo "   - Scope: bounded or unclear?"
    echo "   - Success: measurable or vague?"
    echo "   - Stuck: know when to stop or might spin?"
    echo ""
    echo "   If any check fails, explain why and ask clarifying question."
    echo "   Loop until all checks pass."
    echo ""
    echo "7. Update the pace in the heat file with refined spec"
    echo ""
    echo "8. Do NOT commit (preparatory work, accumulates until /jja-pace-wrap or /jja-sync)"
    echo ""
    echo "9. Report what was updated"
    echo ""
    echo "Error handling: If paths wrong or files missing, announce issue and stop."
  }
}

zjjw_emit_pace_delegate() {
  {
    echo "You are executing a delegated pace from the current Job Jockey heat."
    echo ""
    echo "Configuration:"
    echo "- Target repo dir: ${ZJJW_TARGET_DIR}"
    echo "- Kit path: ${ZJJW_KIT_PATH}"
    echo ""
    echo "Steps:"
    echo ""
    echo "1. Check for current heat in .claude/jjm/current/"
    echo "   - If no heat: announce \"No active heat\" and stop"
    echo ""
    echo "2. Identify the pace to delegate (from context or ask)"
    echo ""
    echo "3. Validate the pace:"
    echo "   - Is mode \`delegated\`?"
    echo "     - If \`manual\`: refuse with \"This pace is manual - work on it conversationally\""
    echo "     - If unset: refuse with \"Run /jja-pace-refine first to set mode\""
    echo "   - Is spec healthy? Check for:"
    echo "     - Objective defined"
    echo "     - Scope bounded"
    echo "     - Success criteria clear"
    echo "     - Failure behavior specified"
    echo "   - If unhealthy: refuse with \"This pace needs refinement - [specific gap]\""
    echo ""
    echo "4. If valid, present the pace spec clearly:"
    echo "   \`\`\`"
    echo "   Executing delegated pace: **[title]**"
    echo ""
    echo "   Objective: [objective]"
    echo "   Scope: [scope]"
    echo "   Success: [criteria]"
    echo "   On failure: [behavior]"
    echo "   \`\`\`"
    echo ""
    echo "5. Execute the pace based solely on the spec"
    echo "   - If target repo != \`.\`, work in target repo directory: ${ZJJW_TARGET_DIR}"
    echo "   - Work from the spec, not from refinement conversation context"
    echo "   - Stay within defined scope"
    echo "   - Stop when success criteria met OR failure condition hit"
    echo ""
    echo "6. Report outcome:"
    echo "   - Success: what was accomplished, evidence of success criteria"
    echo "   - Failure: what was attempted, why stopped, what's needed"
    echo "   - Modified files: list absolute paths for easy editor access"
    echo "     Example: \`/Users/name/project/src/file.ts\`"
    echo ""
    echo "7. Do NOT auto-complete the pace. User decides via /jja-pace-wrap"
    echo "   Work in target repo is NOT auto-committed. User can review and use /jja-sync."
    echo ""
    echo "Error handling: If paths wrong or files missing, announce issue and stop."
  }
}

zjjw_emit_pace_wrap() {
  {
    echo "You are helping mark a pace complete in the current Job Jockey heat."
    echo ""
    echo "Configuration:"
    echo "- Target repo dir: ${ZJJW_TARGET_DIR}"
    echo "- Kit path: ${ZJJW_KIT_PATH}"
    echo ""
    echo "Steps:"
    echo ""
    echo "1. Check for current heat in .claude/jjm/current/"
    echo "   - If no heat: announce \"No active heat\" and stop"
    echo "   - If multiple: ask which one"
    echo ""
    echo "2. Ask which pace to mark done (or infer from context)"
    echo ""
    echo "3. Summarize the pace completion based on chat context"
    echo "   - Focus on WHAT changed, not WHERE (no line numbers - they go stale)"
    echo "   - Include: file names, function/section names, nature of change"
    echo "   - Keep brief: one sentence, under 100 chars if possible"
    echo ""
    echo "4. Show proposed summary and ask for approval"
    echo ""
    echo "5. Update the heat file in .claude/jjm/current/:"
    echo "   - Number the pace and move to ## Done section"
    echo "   - Replace description with brief summary"
    echo "   - Move next pace from ## Remaining to ## Current (if any)"
    echo ""
    echo "6. Commit JJ state (this repo only, no push):"
    echo "   \`\`\`bash"
    echo "   git add .claude/jjm/current/jjh-*.md"
    echo "   git commit -m \"JJA: pace-wrap - [brief description]\""
    echo "   \`\`\`"
    echo ""
    echo "7. Report what was done"
    echo ""
    echo "Error handling: If files missing or paths wrong, announce issue and stop."
  }
}

zjjw_emit_sync() {
  {
    echo "You are synchronizing JJ state and target repo work."
    echo ""
    echo "Configuration:"
    echo "- Target repo dir: ${ZJJW_TARGET_DIR}"
    echo "- Kit path: ${ZJJW_KIT_PATH}"
    echo ""
    echo "Steps:"
    echo ""
    echo "1. Check if .claude/jjm/ is gitignored"
    echo "   - If yes: warn \"JJ state is gitignored - cannot sync\" and stop"
    echo ""
    echo "2. Commit and push JJ state (this repo):"
    echo "   git add -A .claude/jjm/"
    echo "   git commit -m \"JJA: sync\" --allow-empty"
    echo "   git push"
    echo "   Report: \"JJ state: committed and pushed\""
    echo ""
    if test "${ZJJW_TARGET_DIR}" = "."; then
      echo "3. Target repo = \`.\`:"
      echo "   - JJ state and work are same repo, already handled"
      echo "   - Report: \"Target repo: same as JJ state (direct mode)\""
    else
      echo "3. Target repo != \`.\`:"
      echo "   cd ${ZJJW_TARGET_DIR}"
      echo "   git add -A"
      echo "   git commit -m \"JJA: sync\" --allow-empty"
      echo "   git push"
      echo "   cd - > /dev/null"
      echo "   Report: \"Target repo: committed and pushed\""
    fi
    echo ""
    echo "4. If any git operation fails, report the specific failure"
    echo ""
    echo "Error handling: If paths wrong or repos inaccessible, announce issue and stop."
  }
}

zjjw_emit_itch_list() {
  {
    echo "You are listing all Job Jockey itches and scars."
    echo ""
    echo "Configuration:"
    echo "- Target repo dir: ${ZJJW_TARGET_DIR}"
    echo "- Kit path: ${ZJJW_KIT_PATH}"
    echo ""
    echo "Steps:"
    echo ""
    echo "1. Read .claude/jjm/jji_itch.md for itches (future work)"
    echo ""
    echo "2. Read .claude/jjm/jjs_scar.md for scars (closed with lessons)"
    echo ""
    echo "3. Display all entries:"
    echo "   \`\`\`"
    echo "   Itches (N):"
    echo "   1. [section header] - [brief description]"
    echo "   2. [section header] - [brief description]"
    echo "   ..."
    echo ""
    echo "   Scars (N):"
    echo "   1. [section header] - [closed reason]"
    echo "   2. [section header] - [closed reason]"
    echo "   ..."
    echo "   \`\`\`"
    echo ""
    echo "4. If either file is empty or missing, report appropriately:"
    echo "   - \"Itches: none\""
    echo "   - \"Scars: none\""
    echo ""
    echo "Error handling: If files missing, report which ones and continue with available data."
  }
}

zjjw_emit_itch_find() {
  {
    echo "You are searching for a Job Jockey itch by keyword."
    echo ""
    echo "Configuration:"
    echo "- Target repo dir: ${ZJJW_TARGET_DIR}"
    echo "- Kit path: ${ZJJW_KIT_PATH}"
    echo ""
    echo "Steps:"
    echo ""
    echo "1. Ask for search term (or use term from context)"
    echo ""
    echo "2. Search both files:"
    echo "   - .claude/jjm/jji_itch.md (itches)"
    echo "   - .claude/jjm/jjs_scar.md (scars)"
    echo ""
    echo "3. Report matches with context:"
    echo "   \`\`\`"
    echo "   Found N matches for \"[term]\":"
    echo ""
    echo "   In Itches:"
    echo "   - [matching entry with surrounding context]"
    echo ""
    echo "   In Scars:"
    echo "   - [matching entry with surrounding context]"
    echo "   \`\`\`"
    echo ""
    echo "4. If no matches found, report \"No matches found for '[term]'\""
    echo ""
    echo "Error handling: If files missing, report which ones and search available files."
  }
}

zjjw_emit_itch_move() {
  {
    echo "You are moving a Job Jockey itch between locations."
    echo ""
    echo "Configuration:"
    echo "- Target repo dir: ${ZJJW_TARGET_DIR}"
    echo "- Kit path: ${ZJJW_KIT_PATH}"
    echo ""
    echo "Steps:"
    echo ""
    echo "1. Identify the itch to move (from context or ask)"
    echo ""
    echo "2. Determine current location:"
    echo "   - .claude/jjm/jji_itch.md (Itches)"
    echo "   - .claude/jjm/jjs_scar.md (Scars)"
    echo ""
    echo "3. Ask for destination:"
    echo "   - Itch (jji_itch.md) - future work"
    echo "   - Scar (jjs_scar.md) - closing with lessons learned"
    echo "   - Promote to Heat - create new heat file"
    echo ""
    echo "4. Execute the move:"
    echo ""
    echo "   **If moving to Scar:**"
    echo "   - Remove section from jji_itch.md"
    echo "   - Add section to jjs_scar.md"
    echo "   - Append **Closed**: [reason] and Learned: [lesson] lines"
    echo ""
    echo "   **If restoring from Scar to Itch:**"
    echo "   - Remove section from jjs_scar.md"
    echo "   - Add section to jji_itch.md"
    echo "   - Remove the Closed/Learned lines"
    echo ""
    echo "   **If promoting to Heat:**"
    echo "   - Remove section from source file"
    echo "   - Create new heat file: .claude/jjm/current/jjh_bYYMMDD-[description].md"
    echo "   - Use itch as initial context"
    echo "   - Create initial pace from itch objective"
    echo "   - Ask user to confirm heat structure"
    echo ""
    echo "5. Do NOT commit (accumulates until /jja-sync)"
    echo ""
    echo "6. Report what was moved"
    echo ""
    echo "Error handling: If entry not found or files missing, announce issue and stop."
  }
}

zjjw_emit_claudemd_section() {
  {
    echo "## Job Jockey Configuration"
    echo ""
    echo "Job Jockey (JJ) is installed for managing project initiatives."
    echo ""
    echo "**Concepts:**"
    echo "- **Heat**: Bounded initiative with coherent goals that are clear and present (3-50 sessions). Location: \`current/\` (active) or \`retired/\` (done)."
    echo "- **Pace**: Discrete action within a heat; mode is \`manual\` (human drives) or \`delegated\` (model drives from spec)"
    echo "- **Itch**: Future work (any detail level), lives in jji_itch.md"
    echo "- **Scar**: Closed work with lessons learned, lives in jjs_scar.md"
    echo ""
    echo "- Target repo dir: \`${ZJJW_TARGET_DIR}\`"
    echo "- JJ Kit path: \`${ZJJW_KIT_PATH}\`"
    echo ""
    echo "**Available commands:**"
    echo "- \`/jja-heat-resume\` - Resume current heat, show current pace"
    echo "- \`/jja-heat-retire\` - Move completed heat to retired with datestamp"
    echo "- \`/jja-pace-find\` - Show current pace (with mode)"
    echo "- \`/jja-pace-left\` - List all remaining paces (with mode)"
    echo "- \`/jja-pace-add\` - Add a new pace (defaults to manual)"
    echo "- \`/jja-pace-refine\` - Refine pace spec, set mode (manual or delegated)"
    echo "- \`/jja-pace-delegate\` - Execute a delegated pace"
    echo "- \`/jja-pace-wrap\` - Mark pace complete"
    echo "- \`/jja-sync\` - Commit and push JJ state and target repo"
    echo "- \`/jja-itch-list\` - List all itches and scars"
    echo "- \`/jja-itch-find\` - Find an itch by keyword"
    echo "- \`/jja-itch-move\` - Move itch to scar or promote to heat"
    echo ""
    echo "**Important**: New commands are not available in this installation session. You must restart Claude Code before the new commands become available."
  }
}

######################################################################
# Main Commands

jjw_install() {
  buc_step "Clearing previous installation"
  jjw_uninstall

  buc_step "Creating directory structure"
  mkdir -p ".claude/commands"
  mkdir -p ".claude/jjm/current"
  mkdir -p ".claude/jjm/retired"
  test -f ".claude/jjm/jji_itch.md" || echo "# Itches" > ".claude/jjm/jji_itch.md"
  test -f ".claude/jjm/jjs_scar.md" || echo "# Scars" > ".claude/jjm/jjs_scar.md"

  buc_step "Emitting command files"
  zjjw_emit_heat_resume   > ".claude/commands/jja-heat-resume.md"
  zjjw_emit_heat_retire   > ".claude/commands/jja-heat-retire.md"
  zjjw_emit_pace_find     > ".claude/commands/jja-pace-find.md"
  zjjw_emit_pace_left     > ".claude/commands/jja-pace-left.md"
  zjjw_emit_pace_add      > ".claude/commands/jja-pace-add.md"
  zjjw_emit_pace_refine   > ".claude/commands/jja-pace-refine.md"
  zjjw_emit_pace_delegate > ".claude/commands/jja-pace-delegate.md"
  zjjw_emit_pace_wrap     > ".claude/commands/jja-pace-wrap.md"
  zjjw_emit_sync          > ".claude/commands/jja-sync.md"
  zjjw_emit_itch_list     > ".claude/commands/jja-itch-list.md"
  zjjw_emit_itch_find     > ".claude/commands/jja-itch-find.md"
  zjjw_emit_itch_move     > ".claude/commands/jja-itch-move.md"

  buc_step "Patching CLAUDE.md"
  zjjw_emit_claudemd_section > "${BUD_TEMP_DIR}/jjw_claudemd_section.md"
  zjjw_patch_claudemd

  buc_success "Job Jockey installed"
  echo ""
  echo "Restart Claude Code for new commands to become available."
}

jjw_uninstall() {
  buc_step "Removing command files"
  rm -f .claude/commands/jja-*.md

  buc_step "Removing CLAUDE.md section"
  zjjw_unpatch_claudemd

  buc_success "Job Jockey uninstalled (state preserved in .claude/jjm/)"
}

zjjw_patch_claudemd() {
  local z_section_file="${BUD_TEMP_DIR}/jjw_claudemd_section.md"
  local z_prompt_file="${BUD_TEMP_DIR}/jjw_patch_prompt.txt"

  test -f "${z_section_file}" || buc_die "Section file not found: ${z_section_file}"

  local z_section_content
  z_section_content=$(<"${z_section_file}")
  test -n "${z_section_content}" || buc_die "Section content is empty"

  {
    echo "In CLAUDE.md, find the '## Job Jockey Configuration' section."
    echo "Replace everything from that header until the next '## ' header (or end of file) with this exact content:"
    echo ""
    echo "${z_section_content}"
    echo ""
    echo "If no such section exists, insert it before '## Current Context' if that section exists, otherwise append at end of file."
    echo "Preserve all other content exactly as-is."
  } > "${z_prompt_file}"

  local z_prompt
  z_prompt=$(<"${z_prompt_file}")

  claude --setting-sources user \
    --allowedTools "Read,Edit,Write" \
    -p "${z_prompt}" || buc_die "Claude CLI failed to patch CLAUDE.md"
}

zjjw_unpatch_claudemd() {
  local z_prompt="In CLAUDE.md, find the '## Job Jockey Configuration' section. Remove everything from that header until the next '## ' header (or end of file). If no such section exists, do nothing. Preserve all other content exactly as-is."

  claude --setting-sources user \
    --allowedTools "Read,Edit,Write" \
    -p "${z_prompt}" 2>/dev/null || true
}

######################################################################
# Routing

jjw_route() {
  local z_command="${1:-}"
  shift || true

  case "${z_command}" in
    jjk-i) jjw_install ;;
    jjk-u) jjw_uninstall ;;
    *)     buc_die "Unknown command: ${z_command}\nAvailable: jjk-i (install), jjk-u (uninstall)" ;;
  esac
}

jjw_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  jjw_route "${z_command}" "$@"
}

jjw_main "$@"

# eof
