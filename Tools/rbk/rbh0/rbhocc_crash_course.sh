#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# Recipe Bottle Handbook Onboarding - Crash Course (configure repo environment)

set -euo pipefail

test -z "${ZRBHOCC_SOURCED:-}" || return 0
ZRBHOCC_SOURCED=1

rbho_crash_course() {
  zrbho_sentinel

  buc_doc_brief "Configure your Repo's Environment — tabtargets, regimes, station setup, logs"
  buc_doc_shown || return 0

  local z_rbrr_project=""
  local z_rbrr_populated=0
  if test -f "${RBBC_rbrr_file}"; then
    z_rbrr_project=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_DEPOT_PROJECT_ID") || z_rbrr_project=""
    test -n "${z_rbrr_project}" && z_rbrr_populated=1
  fi

  local z_station_present=0
  local z_log_dir=""
  if test -n "${BURD_STATION_FILE:-}" && test -f "${BURD_STATION_FILE}"; then
    z_station_present=1
    z_log_dir=$(zrbho_po_extract_capture "${BURD_STATION_FILE}" "BURS_LOG_DIR") || z_log_dir=""
  fi

  buh_section "Recipe Bottle — Configure your Repo's Environment"
  buh_e
  buh_step_style "Step " " — "

  buh_step1 "What you ran to get here"
  buh_e
  buh_line "The command you just ran is a ${RBYC_TABTARGET} — a launcher script"
  buh_line "in the ${BURC_TABTARGET_DIR}/ directory. Tab completion narrows by prefix: type \`${BURC_TABTARGET_DIR}/rbw-<TAB>\` to see every"
  buh_line "${RBYC_RECIPE_BOTTLE} command."
  buh_e

  buh_step1 "View the project config"
  buh_e
  buh_line "A ${RBYC_REGIME} is a configuration file with a schema, a renderer,"
  buh_line "and a validator. Run the renderer for the project config regime:"
  buh_e
  buh_tt   "   " "${BUWZ_RC_RENDER}"
  buh_e
  buh_line "${RBYC_BURC} is checked into git — shared project settings that every"
  buh_line "clone gets. It tells the launcher where to find tools and where"
  buh_line "to look for your personal station file."
  buh_e

  buh_step1 "View your personal station"
  buh_e
  buh_line "${RBYC_BURS} is your per-developer station file: local, gitignored,"
  buh_line "holds things that vary per machine. Run the renderer:"
  buh_e
  buh_tt   "   " "${BUWZ_RS_RENDER}"
  buh_e
  buh_line "The repo-vs-personal split is deliberate: ${RBYC_BURC} travels with the code; ${RBYC_BURS} stays on your machine."
  buh_e

  buh_step1 "Validate your station"
  buh_e
  buh_line "Every ${RBYC_REGIME} has a validate tabtarget that checks the file against"
  buh_line "its schema. This may fail if your station file is missing fields"
  buh_line "beyond the minimum the launcher required — that is expected."
  buh_line "Run it:"
  buh_e
  buh_tt   "   " "${BUWZ_RS_VALIDATE}"
  buh_e
  buh_line "Read the error if it fails — it names the field and tells you"
  buh_line "what to fill in."
  buh_e
  if test "${z_station_present}" = "1"; then
    buyy_cmd_yawp " [*] ";              local -r z_mark="${z_buym_yelp}"
    buyy_cmd_yawp "${BURD_STATION_FILE}"; local -r z_path="${z_buym_yelp}"
    buh_line "${z_mark} Station file present at ${z_path}"
  else
    zrbho_po_status 0 "Station file not found"
  fi
  buh_e

  buh_step1 "Validate the repo regime"
  buh_e
  buh_line "The repository regime (${RBYC_RBRR}) holds your team's ${RBYC_DEPOT}"
  buh_line "identity — the GCP project where container images are built and stored."
  buh_line "Run the validator:"
  buh_e
  buh_tt   "   " "${RBZ_VALIDATE_REPO}"
  buh_e
  buh_line "On a bare fork, ${RBYC_RBRR} fields are blank and validation will fail —"
  buh_line "you need a ${RBYC_PAYOR} account and a ${RBYC_DEPOT} to populate them."
  buh_line "On a team repo, they are already populated and validation passes."
  buh_line "Either way, read the output — it tells you exactly what state you're in."
  buh_e
  if test "${z_rbrr_populated}" = "1"; then
    zrbho_po_status 1 "RBRR populated — depot project: ${z_rbrr_project}"
  else
    zrbho_po_status 0 "RBRR not populated — depot identity fields are blank"
  fi
  buh_e

  buh_step1 "Check your logs"
  buh_e
  buh_line "When you ran the validator, it printed file paths at the top"
  buh_line "of its output. Every state-changing command writes three ${RBYC_LOG}"
  buh_line "files to ${RBYC_BURS}_LOG_DIR:"
  buh_e
  if test -n "${z_log_dir}"; then
    buyy_cmd_yawp "${z_log_dir}/${BURC_LOG_LAST}.${BURC_LOG_EXT}"; local -r z_log_path="${z_buym_yelp}"
    buh_line "   stable    ${z_log_path}  (always the same path, great for Claude)"
  else
    buh_line "   stable    always the same path — tooling reads this one"
  fi
  buh_line "   per-cmd   same filename across runs — diff between executions"
  buh_line "   history   timestamped — permanent record, never overwritten"
  buh_e
  buh_line "Some commands also write a ${RBYC_TRANSCRIPT} — a single file"
  buh_line "capturing key decision points and state transitions. When a"
  buh_line "command fails, the transcript is the first thing to read."
  buh_e
  buh_line "Handbook display commands (like this one) do not log — teaching"
  buh_line "output is ephemeral by design."
  buh_e

  buh_step1 "The pattern"
  buh_e
  buh_line "Every ${RBYC_REGIME} has a render and a validate tabtarget."
  buh_line "The letter after \`r\` is all that changes:"
  buh_e
  buyy_tt_yawp "${BUWZ_RC_RENDER}";       local -r z_rc_r="${z_buym_yelp}"
  buyy_tt_yawp "${BUWZ_RC_VALIDATE}";      local -r z_rc_v="${z_buym_yelp}"
  buyy_tt_yawp "${BUWZ_RS_RENDER}";        local -r z_rs_r="${z_buym_yelp}"
  buyy_tt_yawp "${BUWZ_RS_VALIDATE}";      local -r z_rs_v="${z_buym_yelp}"
  buyy_tt_yawp "${RBZ_RENDER_REPO}";       local -r z_rr_r="${z_buym_yelp}"
  buyy_tt_yawp "${RBZ_VALIDATE_REPO}";     local -r z_rr_v="${z_buym_yelp}"
  buyy_tt_yawp "${RBZ_RENDER_PAYOR}";      local -r z_rp_r="${z_buym_yelp}"
  buyy_tt_yawp "${RBZ_VALIDATE_PAYOR}";    local -r z_rp_v="${z_buym_yelp}"
  buyy_tt_yawp "${RBZ_RENDER_OAUTH}";      local -r z_ro_r="${z_buym_yelp}"
  buyy_tt_yawp "${RBZ_VALIDATE_OAUTH}";    local -r z_ro_v="${z_buym_yelp}"
  buyy_tt_yawp "${RBZ_RENDER_VESSEL}";     local -r z_rv_r="${z_buym_yelp}"
  buyy_tt_yawp "${RBZ_VALIDATE_VESSEL}";   local -r z_rv_v="${z_buym_yelp}"
  buyy_tt_yawp "${RBZ_RENDER_NAMEPLATE}";  local -r z_rn_r="${z_buym_yelp}"
  buyy_tt_yawp "${RBZ_VALIDATE_NAMEPLATE}";local -r z_rn_v="${z_buym_yelp}"
  buyy_tt_yawp "${RBZ_RENDER_AUTH}";       local -r z_ra_r="${z_buym_yelp}"
  buyy_tt_yawp "${RBZ_VALIDATE_AUTH}";     local -r z_ra_v="${z_buym_yelp}"
  buh_line "   c  ${RBYC_BURC}  ${z_rc_r}   ${z_rc_v}"
  buh_line "   s  ${RBYC_BURS}  ${z_rs_r}  ${z_rs_v}"
  buh_line "   r  ${RBYC_RBRR}  ${z_rr_r}     ${z_rr_v}"
  buh_line "   p  ${RBYC_RBRP}  ${z_rp_r}    ${z_rp_v}"
  buh_line "   o  ${RBYC_RBRO}  ${z_ro_r}    ${z_ro_v}"
  buh_e
  buh_line "These take a target name (vessel, nameplate, or role):"
  buh_e
  buh_line "   v  ${RBYC_RBRV}  ${z_rv_r}     ${z_rv_v}"
  buh_line "   n  ${RBYC_RBRN}  ${z_rn_r}  ${z_rn_v}"
  buh_line "   a  ${RBYC_RBRA}  ${z_ra_r}       ${z_ra_v}"
  buh_e
  buh_line "Learn the letter — you can find any regime's tools from it."
  buh_e

  buh_step1 "Next steps"
  buh_e
  buh_line "Your repo environment is configured. The tools work, errors explain"
  buh_line "themselves, and ${RBYC_LOGS} land where you told them to."
  buh_e
  buh_line "Return to the start menu for what to do next:"
  buh_tt   "   " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

# eof
