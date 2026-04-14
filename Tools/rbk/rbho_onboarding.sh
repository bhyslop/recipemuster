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
# Recipe Bottle Handbook - Cross-role Onboarding Walkthroughs
#
# Per-role onboarding tracks with dual-mode rendering (walkthrough when any
# probe red, reference when all green). Filesystem-only probes — functions
# work pre-kindle and require only the display infrastructure (buh_*) plus
# regime constants (rbcc, rbgc, rbz).

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBHO_SOURCED:-}" || buc_die "Module rbho multiply sourced - check sourcing hierarchy"
ZRBHO_SOURCED=1

######################################################################
# Module kindle — verifies prerequisite modules are kindled
#
# rbho walkthroughs do not require rbho-local state (they probe the
# filesystem directly and render via buh_*). The kindle exists to assert
# the dependency ordering is correct at furnish time.

zrbho_kindle() {
  test -z "${ZRBHO_KINDLED:-}" || buc_die "Module rbho already kindled"
  zrbgc_sentinel
  zbuz_sentinel
  zrbz_sentinel
  readonly ZRBHO_KINDLED=1
}

zrbho_sentinel() {
  test "${ZRBHO_KINDLED:-}" = "1" || buc_die "Module rbho not kindled - call zrbho_kindle first"
}

######################################################################
# Probe utilities — no sentinels, all work pre-kindle
#
# These are filesystem probes for onboarding status. They set
# caller-scope variables; callers declare the variables locally first.

# Dashboard status line
zrbho_po_status() {
  local -r z_flag="${1:-}"
  local -r z_text="${2:-}"
  if test "${z_flag}" = "1"; then
    buh_ct " [*] " "${z_text}"
  else
    buh_t " [ ] ${z_text}"
  fi
}

# Extract a KEY=VALUE from a file; stdout empty if missing.  No sourcing.
zrbho_po_extract_capture() {
  local -r z_file="${1:-}"
  local -r z_key="${2:-}"
  test -n "${z_key}"  || return 1
  test -f "${z_file}" || return 1
  local z_line=""
  while IFS= read -r z_line; do
    case "${z_line}" in "${z_key}="*) echo "${z_line#"${z_key}="}"; return 0 ;; esac
  done < "${z_file}"
  return 1
}

######################################################################
# Shared credential probe — sets caller-scoped:
# z_has_payor, z_has_governor, z_has_director, z_has_retriever, z_secrets_dir

zrbho_probe_role_credentials() {
  z_has_payor=0
  z_has_governor=0
  z_has_director=0
  z_has_retriever=0
  z_secrets_dir=""

  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi

  if test -n "${z_secrets_dir}"; then
    test -f "${z_secrets_dir}/${RBCC_rbro_file}"                                && z_has_payor=1
    test -f "${z_secrets_dir}/${RBCC_role_governor}/${RBCC_rbra_file}"  && z_has_governor=1
    test -f "${z_secrets_dir}/${RBCC_role_director}/${RBCC_rbra_file}"  && z_has_director=1
    test -f "${z_secrets_dir}/${RBCC_role_retriever}/${RBCC_rbra_file}" && z_has_retriever=1
  fi
}

######################################################################
# Probe retriever walkthrough units — sets caller-scope z_ru1..z_ru4
# Requires: z_secrets_dir already set (from zrbho_probe_role_credentials
#           or direct extraction).

zrbho_probe_retriever_units() {
  z_ru1=0; z_ru2=0; z_ru3=0; z_ru4=0

  # Unit 1: Retriever credential file exists
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/${RBCC_role_retriever}/${RBCC_rbra_file}"; then
    z_ru1=1
  fi

  # Units 2-4 require Docker
  if ! command -v docker >/dev/null 2>&1; then return 0; fi

  # Unit 2: Any image from this depot's GAR exists locally
  local z_project_id="" z_region=""
  if test -f "${RBBC_rbrr_file}"; then
    z_project_id=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_DEPOT_PROJECT_ID") || z_project_id=""
    z_region=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_GCP_REGION") || z_region=""
  fi
  if test -n "${z_region}" && test -n "${z_project_id}"; then
    local z_gar_prefix="${z_region}${RBGC_GAR_HOST_SUFFIX}/${z_project_id}/"
    if docker images --format "{{.Repository}}" 2>/dev/null | grep -q "^${z_gar_prefix}"; then
      z_ru2=1
    fi
  fi

  # Unit 3: Any crucible is charged (bottle container running)
  if docker ps --format "{{.Names}}" 2>/dev/null | grep -q -- "-bottle$"; then
    z_ru3=1
  fi

  # Unit 4: Kludge-tagged image exists (k-prefixed hallmark)
  if docker images --format "{{.Tag}}" 2>/dev/null | grep -q "^k[0-9]"; then
    z_ru4=1
  fi
}

######################################################################
# Probe director walkthrough units — sets caller-scope z_du1..z_du7
# Requires: z_secrets_dir already set.
#
# Vessel assignments per docket:
#   Unit 2,4,6: rbev-sentry-deb-tether (conjure vessel)
#   Unit 5:     rbev-bottle-plantuml    (bind vessel)
#
# Probes use local filesystem + docker only (no GAR API, no gcloud).
# Units 3-7 check docker image tags — these require the user to have
# summoned (pulled) the result locally after cloud operations.
# Unit 6 (graft) is detectable without summoning because graft tags
# the local image before pushing.

zrbho_probe_director_units() {
  z_du1=0
  z_du2=0
  z_du3=0
  z_du4=0
  z_du5=0
  z_du6=0
  z_du7=0

  # Unit 1: Director credential file exists
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/${RBCC_role_director}/${RBCC_rbra_file}"; then
    z_du1=1
  fi

  # Units 2-7 require Docker
  if ! command -v docker >/dev/null 2>&1; then return 0; fi

  # Build GAR image prefix for this depot
  local z_project_id=""
  local z_region=""
  if test -f "${RBBC_rbrr_file}"; then
    z_project_id=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_DEPOT_PROJECT_ID") || z_project_id=""
    z_region=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_GCP_REGION") || z_region=""
  fi
  if test -z "${z_region}" || test -z "${z_project_id}"; then return 0; fi

  local -r z_gar_prefix="${z_region}${RBGC_GAR_HOST_SUFFIX}/${z_project_id}/"
  local -r z_docker_images="${BURD_TEMP_DIR}/zrbho_probe_director_images.txt"
  local -r z_docker_stderr="${BURD_TEMP_DIR}/zrbho_probe_director_stderr.txt"

  # Collect docker images into temp file
  docker images --format "{{.Repository}}:{{.Tag}}" \
    > "${z_docker_images}" 2>"${z_docker_stderr}" \
    || return 0

  # Load lines matching this depot's GAR prefix into array
  local z_depot_images=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    case "${z_line}" in
      "${z_gar_prefix}"*) z_depot_images+=("${z_line}") ;;
    esac
  done < "${z_docker_images}"

  # Single pass: match hallmark tag patterns via case
  local z_conjure_found=0
  local z_i=0
  for z_i in "${!z_depot_images[@]}"; do
    case "${z_depot_images[$z_i]}" in
      *"rbev-sentry-deb-tether:k"[0-9]*) z_du2=1 ;;
      *"rbev-sentry-deb-tether:c"[0-9]*) z_conjure_found=1 ;;
      *"rbev-bottle-plantuml:b"[0-9]*)    z_du5=1 ;;
      *"rbev-sentry-deb-tether:g"[0-9]*) z_du6=1 ;;
    esac
  done

  # Conjure implies depot foundation (reliquary + enshrine are prerequisites)
  if test "${z_conjure_found}" = "1"; then
    z_du3=1
    z_du4=1
  fi

  # Unit 7: All three modes present
  if test "${z_du4}" = "1" && test "${z_du5}" = "1" && test "${z_du6}" = "1"; then
    z_du7=1
  fi
}

######################################################################
# Probe governor walkthrough units — sets caller-scope z_gu1..z_gu3
# Requires: z_secrets_dir already set.
#
# Unit 1: Governor credential installed
# Unit 2: Retriever AND director credentials installed (governor created them)
# Unit 3: Functional verification — a GAR image has been pulled locally
#         (proves the full charter/knight → IAM → access chain works)

zrbho_probe_governor_units() {
  z_gu1=0; z_gu2=0; z_gu3=0

  # Unit 1: Governor credential file exists
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/${RBCC_role_governor}/${RBCC_rbra_file}"; then
    z_gu1=1
  fi

  # Unit 2: Both retriever AND director credential files exist
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/${RBCC_role_retriever}/${RBCC_rbra_file}" && \
     test -f "${z_secrets_dir}/${RBCC_role_director}/${RBCC_rbra_file}"; then
    z_gu2=1
  fi

  # Unit 3: Functional verification — a GAR image exists locally
  # This proves the SAs the governor created actually work (IAM grants applied).
  if ! command -v docker >/dev/null 2>&1; then return 0; fi

  local z_project_id="" z_region=""
  if test -f "${RBBC_rbrr_file}"; then
    z_project_id=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_DEPOT_PROJECT_ID") || z_project_id=""
    z_region=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_GCP_REGION") || z_region=""
  fi
  if test -n "${z_region}" && test -n "${z_project_id}"; then
    local z_gar_prefix="${z_region}${RBGC_GAR_HOST_SUFFIX}/${z_project_id}/"
    if docker images --format "{{.Repository}}" 2>/dev/null | grep -q "^${z_gar_prefix}"; then
      z_gu3=1
    fi
  fi
}

######################################################################
# Probe payor walkthrough units — sets caller-scope z_pu1..z_pu4
# Requires: z_secrets_dir already set.
#
# Unit 1: OAuth credential present (RBCC_rbro_file in secrets dir)
# Unit 2: Payor project configured (RBRP_PAYOR_PROJECT_ID non-empty)
# Unit 3: Depot provisioned (RBRR_DEPOT_PROJECT_ID non-empty)
# Unit 4: Governor SA exists (governor credential file present)

zrbho_probe_payor_units() {
  z_pu1=0
  z_pu2=0
  z_pu3=0
  z_pu4=0

  # Unit 1: OAuth credential present
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/${RBCC_rbro_file}"; then
    z_pu1=1
  fi

  # Unit 2: Payor project configured
  if test -f "${RBBC_rbrp_file}"; then
    local z_probe_line=""
    while IFS= read -r z_probe_line; do
      case "${z_probe_line}" in RBRP_PAYOR_PROJECT_ID=?*) z_pu2=1; break ;; esac
    done < "${RBBC_rbrp_file}"
  fi

  # Unit 3: Depot provisioned
  if test -f "${RBBC_rbrr_file}"; then
    local z_probe_line=""
    while IFS= read -r z_probe_line; do
      case "${z_probe_line}" in RBRR_DEPOT_PROJECT_ID=?*) z_pu3=1; break ;; esac
    done < "${RBBC_rbrr_file}"
  fi

  # Unit 4: Governor SA credential file exists
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/${RBCC_role_governor}/${RBCC_rbra_file}"; then
    z_pu4=1
  fi
}

######################################################################
# Onboarding triage — helper and entry point

# Args: detected(0|1) role_name colophon
zrbho_triage_role() {
  local -r z_detected="${1}" z_name="${2}" z_colophon="${3}"
  local -r z_url="${RBRR_PUBLIC_DOCS_URL}#${z_name}"
  # Column pad lives OUTSIDE the link envelope so the underline/OSC-8 stop
  # at the end of the role name.  Width 13 = 12-char column + 1 separator.
  local z_pad=""
  printf -v z_pad '%*s' $((13 - ${#z_name})) ''
  if test "${z_detected}" = "1"; then
    buh_tltT " [*] " "${z_name}" "${z_url}" "${z_pad}" "${z_colophon}"
  else
    buh_tl   " [ ] " "${z_name}" "${z_url}"
  fi
}

######################################################################
# Legacy role-track functions (triage, reference, retriever, director,
# governor, payor) removed — replaced by intent-organized handbook
# tracks below. See ₣A6 paddock "Context — The Malformation".

######################################################################
# Handbook — intent-organized tracks (Frame 4-refined)
#
# Organizing axis: learner intent + repo state, NOT role/authorization.

# rbho_start_here — probe-aware menu into the handbook family.
# Probes are deliberately narrow: just RBRR populated + any-credential-
# present. Highlighting only arrows foundation actions based on repo
# state; it does not infer intent from detected roles.
rbho_start_here() {
  # No sentinel — works pre-kindle (probes filesystem only)

  buc_doc_brief "Probe-aware menu — route learner into the handbook track that fits their state"
  buc_doc_shown || return 0

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  # --- Preamble ---
  buh_section "Recipe Bottle — Onboarding Start"
  buh_e
  buh_tlt "  " "Recipe Bottle" "${z_docs}" " builds container images with supply-chain provenance"
  buh_t   "  and runs untrusted containers behind enforced network isolation."
  buh_e
  buh_t   "  This menu points you at handbook tracks — self-describing teaching"
  buh_t   "  documents that explain concepts and show you live probe status."
  buh_e

  # --- Foundation ---
  buh_section "Foundation"
  buh_e
  buh_t   "    Configure your Repo's Environment"
  buh_tltlt "      Universal prerequisite. " "Tabtargets" "${z_docs}#Tabtarget" ", " "Regimes" "${z_docs}#Regime" ","
  buh_tltlt "      " "BURS" "${z_docs}#BURS" " setup, validation, " "Logs" "${z_docs}#Log" ". Local-only, no cloud."
  buh_tT  "        " "${RBZ_ONBOARD_CRASH_COURSE}"
  buh_e
  buh_tlt "    Install " "Retriever" "${z_docs}#Retriever" " Credentials"
  buh_t   "      For joining an established project."
  buh_tlt "      Place your " "RBRA" "${z_docs}#RBRA" " credential file, verify, confirm you can pull images."
  buh_tT  "        " "${RBZ_ONBOARD_CRED_RETRIEVER}"
  buh_e
  buh_tlt "    Install " "Director" "${z_docs}#Director" " Credentials"
  buh_t   "      For joining an established project."
  buh_tlt "      Place your " "RBRA" "${z_docs}#RBRA" " credential file, verify, confirm you can build and publish."
  buh_tT  "        " "${RBZ_ONBOARD_CRED_DIRECTOR}"
  buh_e
  buh_tlt "    Start a " "Crucible" "${z_docs}#Crucible" " Using Local Builds"
  buh_tltlt "      The " "ccyolo" "${z_docs}#ccyolo" " " "Crucible" "${z_docs}#Crucible" " runs Claude Code in a container that can"
  buh_t   "      only reach Anthropic. Requires a Claude OAuth subscription."
  buh_t   "      Steps:"
  buh_tltltltlt "        * Build images locally      — " "Kludge" "${z_docs}#Kludge" " " "Sentry" "${z_docs}#Sentry" "/" "Pentacle" "${z_docs}#Pentacle" " and " "Bottle" "${z_docs}#Bottle" ""
  buh_tltlt   "        * Configure local network   — amend " "Nameplate" "${z_docs}#Nameplate" " " "RBRN" "${z_docs}#RBRN" " file"
  buh_tltlt   "        * Start the sandbox         — " "Charge" "${z_docs}#Charge" " the " "Crucible" "${z_docs}#Crucible" ""
  buh_tltlt   "        * Shell into the container  — " "Rack" "${z_docs}#Rack" " the " "Bottle" "${z_docs}#Bottle" ""
  buh_t   "      No cloud, no credentials beyond your own."
  buh_tT  "        " "${RBZ_ONBOARD_FIRST_CRUCIBLE}"
  buh_e

  # --- Create Payor and Depot ---
  buh_section "Create Payor and Depot"
  buh_e
  buh_tlt "  A " "Depot" "${z_docs}#Depot" " is the facility where the team's container images are"
  buh_t   "  built and stored — the ground truth other tracks rest on."
  buh_e
  buh_tltltlt "    " "Payor" "${z_docs}#Payor" " — establish a " "Manor" "${z_docs}#Manor" " and provision the " "Depot" "${z_docs}#Depot" ""
  buh_tT  "        " "${RBZ_ONBOARD_PAYOR_HB}"
  buh_e
  buh_tltltlt "    " "Governor" "${z_docs}#Governor" " — administer service accounts for " "Directors" "${z_docs}#Director" " and " "Retrievers" "${z_docs}#Retriever" ""
  buh_tT  "        " "${RBZ_ONBOARD_GOVERNOR_HB}"
  buh_e

  # --- Director subtracks ---
  buh_section "Director Subtracks"
  buh_e
  buh_t   "    Your First Cloud Build"
  buh_tlt "      Provision the builder toolchain, " "Ordain" "${z_docs}#Ordain" " your first"
  buh_tlt "      " "Vessel" "${z_docs}#Vessel" " via Cloud Build, and tour the result."
  buh_t   "      Steps:"
  buh_tlt "        * Inscribe the " "Reliquary" "${z_docs}#Reliquary" " — provision builder tool images"
  buh_tltlt "        * " "Conjure" "${z_docs}#Conjure" " " "Sentry" "${z_docs}#Sentry" " — first tethered cloud build"
  buh_tltltltlt "        * Tour: " "Tally" "${z_docs}#Tally" ", " "Vouch" "${z_docs}#Vouch" ", " "Plumb" "${z_docs}#Plumb" ", " "Pouch" "${z_docs}#Pouch" " — inspect images and SLSA"
  buh_tlt "        * " "Summon" "${z_docs}#Summon" " — pull the hallmark locally"
  buh_tltlt "        * " "Abjure" "${z_docs}#Abjure" " and " "Rekon" "${z_docs}#Rekon" " — hallmark lifecycle"
  buh_t   "      Requires: Director credentials and a provisioned Depot."
  buh_tT  "        " "${RBZ_ONBOARD_DIR_FIRST_BUILD}"
  buh_e
  buh_tlt "    " "Airgap" "${z_docs}#Airgap" " Cloud Build"
  buh_tlt "      Build with zero upstream access. " "Enshrine" "${z_docs}#Enshrine" " mirrors base images"
  buh_t   "      into the Depot so Cloud Build never reaches the internet."
  buh_t   "      Steps:"
  buh_tlt "        * " "Enshrine" "${z_docs}#Enshrine" " base images — mirror upstream into the Depot"
  buh_tltlt "        * " "Conjure" "${z_docs}#Conjure" " " "Sentry" "${z_docs}#Sentry" " airgapped — same vessel, full isolation"
  buh_tlt "        * Compare " "Plumb" "${z_docs}#Plumb" " output — tethered vs airgap side by side"
  buh_t   "      Requires: Reliquary inscribed (previous track)."
  buh_e
  buh_tlt "    " "Bind" "${z_docs}#Bind" " — Safe PlantUML Container"
  buh_t   "      Mirror an upstream image by digest — no Dockerfile, no build."
  buh_t   "      PlantUML renders diagrams but its Docker Hub image could"
  buh_t   "      phone home. Bind pins it; the Sentry blocks all egress."
  buh_t   "      Steps:"
  buh_tltlt "        * " "Bind" "${z_docs}#Bind" " " "PlantUML" "${z_docs}#Bind" " — pin upstream image by digest"
  buh_tlt "        * Inspect " "Vouch" "${z_docs}#Vouch" " verdict — digest-pin, no SLSA (image not built here)"
  buh_tltlt "        * " "Charge" "${z_docs}#Charge" " the " "pluml" "${z_docs}#Nameplate" " Crucible — render a diagram, observe blocked egress"
  buh_t   "      You get the tool without the risk."
  buh_e
  buh_tlt "    " "Graft" "${z_docs}#Graft" " — Local Image Publishing"
  buh_t   "      Push a locally-built image to the Depot. The user owns the"
  buh_t   "      entire build — SLSA cannot vouch for this image. Vouch verdict"
  buh_t   "      is GRAFTED: an explicit signal that provenance stops at the"
  buh_t   "      local machine. Not the enterprise path for safe image creation."
  buh_t   "      Steps:"
  buh_tlt "        * " "Kludge" "${z_docs}#Kludge" " a vessel image locally"
  buh_tlt "        * " "Graft" "${z_docs}#Graft" " — push local image to the Depot"
  buh_tlt "        * Inspect " "Vouch" "${z_docs}#Vouch" " verdict — GRAFTED, no provenance chain"
  buh_t   "      Development and prototyping workflow, not production supply chain."
  buh_e

}

######################################################################
# Configure your Repo's Environment — universal prerequisite handbook
#
# Hands-on walk-through: run commands, read output, configure your
# station. By the end, the learner has working local tooling and
# understands tabtargets, regimes, and the render/validate pattern.

rbho_crash_course() {
  # No sentinel — works pre-kindle (probes filesystem only)

  buc_doc_brief "Configure your Repo's Environment — tabtargets, regimes, station setup, logs"
  buc_doc_shown || return 0

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  # --- Probes ---
  local z_rbrr_project="" z_rbrr_populated=0
  if test -f "${RBBC_rbrr_file}"; then
    z_rbrr_project=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_DEPOT_PROJECT_ID") || z_rbrr_project=""
    test -n "${z_rbrr_project}" && z_rbrr_populated=1
  fi

  local z_station_present=0 z_log_dir=""
  if test -n "${BURD_STATION_FILE:-}" && test -f "${BURD_STATION_FILE}"; then
    z_station_present=1
    z_log_dir=$(zrbho_po_extract_capture "${BURD_STATION_FILE}" "BURS_LOG_DIR") || z_log_dir=""
  fi

  # --- Header ---
  buh_section "Recipe Bottle — Configure your Repo's Environment"
  buh_e
  buh_step_style "Step " " — "

  # --- Step 1: What you ran to get here ---
  buh_step1 "What you ran to get here"
  buh_e
  buh_tlt "The command you just ran is a " "Tabtarget" "${z_docs}#Tabtarget" " — a launcher script"
  buh_t   "in the ${BURC_TABTARGET_DIR}/ directory. Tab completion narrows by prefix: type \`${BURC_TABTARGET_DIR}/rbw-<TAB>\` to see every"
  buh_tlt "" "Recipe Bottle" "${z_docs}" " command."
  buh_e

  # --- Step 2: View the project config (BURC) ---
  buh_step1 "View the project config"
  buh_e
  buh_tlt "A " "Regime" "${z_docs}#Regime" " is a configuration file with a schema, a renderer,"
  buh_t   "and a validator. Run the renderer for the project config regime:"
  buh_e
  buh_tT  "   " "${BUWZ_RC_RENDER}"
  buh_e
  buh_tlt "" "BURC" "${z_docs}#BURC" " is checked into git — shared project settings that every"
  buh_t   "clone gets. It tells the launcher where to find tools and where"
  buh_t   "to look for your personal station file."
  buh_e

  # --- Step 3: View your station (BURS) ---
  buh_step1 "View your personal station"
  buh_e
  buh_tlt "" "BURS" "${z_docs}#BURS" " is your per-developer station file: local, gitignored,"
  buh_t   "holds things that vary per machine. Run the renderer:"
  buh_e
  buh_tT  "   " "${BUWZ_RS_RENDER}"
  buh_e
  buh_tltlt "The repo-vs-personal split is deliberate: " "BURC" "${z_docs}#BURC" " travels with the code; " "BURS" "${z_docs}#BURS" " stays on your machine."
  buh_e

  # --- Step 4: Validate your station ---
  buh_step1 "Validate your station"
  buh_e
  buh_tlt "Every " "Regime" "${z_docs}#Regime" " has a validate tabtarget that checks the file against"
  buh_t   "its schema. This may fail if your station file is missing fields"
  buh_t   "beyond the minimum the launcher required — that is expected."
  buh_t   "Run it:"
  buh_e
  buh_tT  "   " "${BUWZ_RS_VALIDATE}"
  buh_e
  buh_t   "Read the error if it fails — it names the field and tells you"
  buh_t   "what to fill in."
  buh_e
  if test "${z_station_present}" = "1"; then
    buh_tctct "" " [*] " " Station file present at " "${BURD_STATION_FILE}" ""
  else
    zrbho_po_status 0 "Station file not found"
  fi
  buh_e

  # --- Step 5: Validate the repo regime ---
  buh_step1 "Validate the repo regime"
  buh_e
  buh_tltlt "The repository regime (" "RBRR" "${z_docs}#RBRR" ") holds your team's " "Depot" "${z_docs}#Depot" ""
  buh_t   "identity — the GCP project where container images are built and stored."
  buh_t   "Run the validator:"
  buh_e
  buh_tT  "   " "${RBZ_VALIDATE_REPO}"
  buh_e
  buh_tlt "On a bare fork, " "RBRR" "${z_docs}#RBRR" " fields are blank and validation will fail —"
  buh_tltlt "you need a " "Payor" "${z_docs}#Payor" " account and a " "Depot" "${z_docs}#Depot" " to populate them."
  buh_t   "On a team repo, they are already populated and validation passes."
  buh_t   "Either way, read the output — it tells you exactly what state you're in."
  buh_e
  if test "${z_rbrr_populated}" = "1"; then
    zrbho_po_status 1 "RBRR populated — depot project: ${z_rbrr_project}"
  else
    zrbho_po_status 0 "RBRR not populated — depot identity fields are blank"
  fi
  buh_e

  # --- Step 6: Check your logs ---
  buh_step1 "Check your logs"
  buh_e
  buh_t   "When you ran the validator, it printed file paths at the top"
  buh_tlt "of its output. Every state-changing command writes three " "Log" "${z_docs}#Log" ""
  buh_tlt "files to " "BURS" "${z_docs}#BURS" "_LOG_DIR:"
  buh_e
  if test -n "${z_log_dir}"; then
    buh_tct "   stable    " "${z_log_dir}/${BURC_LOG_LAST}.${BURC_LOG_EXT}" "  (always the same path, great for Claude)"
  else
    buh_t   "   stable    always the same path — tooling reads this one"
  fi
  buh_t   "   per-cmd   same filename across runs — diff between executions"
  buh_t   "   history   timestamped — permanent record, never overwritten"
  buh_e
  buh_tlt "Some commands also write a " "Transcript" "${z_docs}#Transcript" " — a single file"
  buh_t   "capturing key decision points and state transitions. When a"
  buh_t   "command fails, the transcript is the first thing to read."
  buh_e
  buh_t   "Handbook display commands (like this one) do not log — teaching"
  buh_t   "output is ephemeral by design."
  buh_e

  # --- Step 7: The pattern ---
  buh_step1 "The pattern"
  buh_e
  buh_tlt "Every " "Regime" "${z_docs}#Regime" " has a render and a validate tabtarget."
  buh_t   "The letter after \`r\` is all that changes:"
  buh_e
  buh_tltTtT "   c  " "BURC" "${z_docs}#BURC" "  " "${BUWZ_RC_RENDER}" "   " "${BUWZ_RC_VALIDATE}" ""
  buh_tltTtT "   s  " "BURS" "${z_docs}#BURS" "  " "${BUWZ_RS_RENDER}" "  " "${BUWZ_RS_VALIDATE}" ""
  buh_tltTtT "   r  " "RBRR" "${z_docs}#RBRR" "  " "${RBZ_RENDER_REPO}" "     " "${RBZ_VALIDATE_REPO}" ""
  buh_tltTtT "   p  " "RBRP" "${z_docs}#RBRP" "  " "${RBZ_RENDER_PAYOR}" "    " "${RBZ_VALIDATE_PAYOR}" ""
  buh_tltTtT "   o  " "RBRO" "${z_docs}#RBRO" "  " "${RBZ_RENDER_OAUTH}" "    " "${RBZ_VALIDATE_OAUTH}" ""
  buh_e
  buh_t   "These take a target name (vessel, nameplate, or role):"
  buh_e
  buh_tltTtT "   v  " "RBRV" "${z_docs}#RBRV" "  " "${RBZ_RENDER_VESSEL}" "     " "${RBZ_VALIDATE_VESSEL}" ""
  buh_tltTtT "   n  " "RBRN" "${z_docs}#RBRN" "  " "${RBZ_RENDER_NAMEPLATE}" "  " "${RBZ_VALIDATE_NAMEPLATE}" ""
  buh_tltTtT "   a  " "RBRA" "${z_docs}#RBRA" "  " "${RBZ_RENDER_AUTH}" "       " "${RBZ_VALIDATE_AUTH}" ""
  buh_e
  buh_t   "Learn the letter — you can find any regime's tools from it."
  buh_e

  # --- Step 8: Next steps ---
  buh_step1 "Next steps"
  buh_e
  buh_t  "Your repo environment is configured. The tools work, errors explain"
  buh_tlt "themselves, and " "Logs" "${z_docs}#Log" " land where you told them to."
  buh_e
  buh_t  "Return to the start menu for what to do next:"
  buh_tT "   " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

######################################################################
# Credential installation — shared utility for retriever/director
#
# Args: role_display  role_constant  knight_tabtarget_constant  role_description
# Probes RBRR for secrets dir, checks credential file, shows install
# path, points to auth regime validator.

zrbho_credential_install() {
  local -r z_role_constant="${1}"
  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  # --- Probes ---
  local z_secrets_dir=""
  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi

  local z_cred_present=0
  if test -n "${z_secrets_dir}" \
     && test -f "${z_secrets_dir}/${z_role_constant}/${RBCC_rbra_file}"; then
    z_cred_present=1
  fi

  buh_step_style "Step " " — "

  # --- Step 1: Get the key file ---
  buh_step1 "Get the key file"
  buh_e
  buh_tltltltlt "A " "Governor" "${z_docs}#Governor" " produces " "RBRA" "${z_docs}#RBRA" " credential files for " "Directors" "${z_docs}#Director" " and " "Retrievers" "${z_docs}#Retriever" "."
  buh_tlt "Your " "Governor" "${z_docs}#Governor" " hands you this file out-of-band — it is a"
  buh_t   "secret, never committed to the repo."
  buh_e

  # --- Step 2: Install the key file ---
  buh_step1 "Install the key file"
  buh_e
  if test -n "${z_secrets_dir}"; then
    buh_tlt "Place the file at the path derived from " "RBRR" "${z_docs}#RBRR" ":"
    buh_e
    buh_c   "   ${z_secrets_dir}/${z_role_constant}/${RBCC_rbra_file}"
    buh_e
    buh_t   "Create the directory if it does not exist."
  else
    buh_tW  "" "RBRR not populated — cannot determine credential path."
    buh_tlt "Run " "Configure your Repo's Environment" "${z_docs}#BURC" " first."
  fi
  buh_e
  if test "${z_cred_present}" = "1"; then
    zrbho_po_status 1 "Credential file present"
  else
    zrbho_po_status 0 "Credential file not found"
  fi
  buh_e

  # --- Step 3: Validate ---
  buh_step1 "Validate"
  buh_e
  buh_tlt "Run the " "RBRA" "${z_docs}#RBRA" " validator for your role:"
  buh_e
  buh_tTc "   " "${RBZ_VALIDATE_AUTH}" " ${z_role_constant}"
  buh_e
  buh_t   "Read the output — it checks the file format and reports"
  buh_t   "what the credential grants."
  buh_e

  # Callers append role-specific verification and closing steps
}

######################################################################
# Install Retriever Credentials

rbho_credential_retriever() {
  buc_doc_brief "Install retriever credentials — place RBRA key, validate, confirm pull access"
  buc_doc_shown || return 0
  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  buh_section "Install Retriever Credentials"
  buh_e
  buh_tlt "A " "Retriever" "${z_docs}#Retriever" " pulls container images from the"
  buh_tlt "  " "Depot" "${z_docs}#Depot" " — read-only access to what others have built."
  buh_e

  zrbho_credential_install "${RBCC_role_retriever}"

  # --- Step 4: Confirm live access ---
  buh_step1 "Confirm live access"
  buh_e
  buh_tltlt "Run " "Tally" "${z_docs}#Tally" " to list " "Hallmarks" "${z_docs}#Hallmark" " in the registry using your retriever credential:"
  buh_e
  buh_tT  "   " "${RBZ_TALLY_HALLMARKS}"
  buh_e
  buh_tlt "If the command succeeds you have working pull access to the " "Depot" "${z_docs}#Depot" "."
  buh_t   "If it fails, re-check the file placement in Step 2."
  buh_e

  # --- Step 5: Next steps ---
  buh_step1 "Next steps"
  buh_e
  buh_t  "Return to the start menu:"
  buh_tT "   " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

######################################################################
# Install Director Credentials

rbho_credential_director() {
  buc_doc_brief "Install director credentials — place RBRA key, validate, confirm build access"
  buc_doc_shown || return 0
  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  buh_section "Install Director Credentials"
  buh_e
  buh_tlt "A " "Director" "${z_docs}#Director" " causes cloud builds and publishes container images to the"
  buh_tlt "  " "Depot" "${z_docs}#Depot" " — write access to the registry."
  buh_e

  zrbho_credential_install "${RBCC_role_director}"

  # --- Step 4: Confirm live access ---
  buh_step1 "Confirm live access"
  buh_e
  buh_tlt "Run " "Rekon" "${z_docs}#Rekon" " to list raw image tags in the registry"
  buh_t   "using your director credential:"
  buh_e
  buh_tT  "   " "${RBZ_REKON_IMAGE}"
  buh_e
  buh_t   "If the command succeeds you have working build access to the"
  buh_tlt "  " "Depot" "${z_docs}#Depot" "."
  buh_t   "If it fails, re-check the file placement in Step 2."
  buh_e

  # --- Step 5: Next steps ---
  buh_step1 "Next steps"
  buh_e
  buh_t  "Return to the start menu:"
  buh_tT "   " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

######################################################################
# First Crucible — local builds onboarding
#
# Frame 4-refined handbook: teaching prose + probes + tabtarget refs.
# Target learner: crucible explorer — local-only, zero cloud, fast
# iteration for security exploration. Zero registry, zero SA credentials.
#
# Nameplate: ccyolo  (Claude Code sandbox, Anthropic-only network)
# Vessels:   rbev-sentry-deb-tether (sentry), rbev-bottle-ccyolo (bottle)
#
# ₢A6AAC DRAFT — content ready for implementation review.
# Infrastructure gaps are marked with [INFRA-NEEDED] comments.
# Assumptions are marked with [ASSUMPTION].

rbho_first_crucible() {
  buc_doc_brief "Start a Crucible using local builds — kludge, charge, SSH, verify containment"
  buc_doc_shown || return 0

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  # Hardcoded for this track — ccyolo is the teaching nameplate
  local -r z_moniker="ccyolo"
  local -r z_sentry_vessel="rbev-sentry-deb-tether"
  local -r z_bottle_vessel="rbev-bottle-ccyolo"
  local -r z_nameplate_file="${RBBC_dot_dir}/${z_moniker}/${RBCC_rbrn_file}"
  local -r z_ssh_tabtarget="tt/rbw-cS.SshTo.${z_moniker}.sh"

  # --- Probes ---

  # Docker runtime
  local z_has_docker=0
  command -v docker >/dev/null 2>&1 && z_has_docker=1

  # Nameplate exists
  local z_nameplate_exists=0
  test -f "${z_nameplate_file}" && z_nameplate_exists=1

  # Hallmarks populated in nameplate
  local z_sentry_hallmark="" z_bottle_hallmark=""
  local z_sentry_hallmark_present=0 z_bottle_hallmark_present=0
  if test "${z_nameplate_exists}" = "1"; then
    z_sentry_hallmark=$(zrbho_po_extract_capture "${z_nameplate_file}" "RBRN_SENTRY_HALLMARK") || z_sentry_hallmark=""
    z_bottle_hallmark=$(zrbho_po_extract_capture "${z_nameplate_file}" "RBRN_BOTTLE_HALLMARK") || z_bottle_hallmark=""
    test -n "${z_sentry_hallmark}" && z_sentry_hallmark_present=1
    test -n "${z_bottle_hallmark}" && z_bottle_hallmark_present=1
  fi

  # Kludge-tagged images exist locally (k-prefixed hallmarks)
  local z_sentry_image_exists=0 z_bottle_image_exists=0
  if test "${z_has_docker}" = "1"; then
    if docker images --format "{{.Repository}}:{{.Tag}}" 2>/dev/null \
       | grep -q "${z_sentry_vessel}:k[0-9]"; then
      z_sentry_image_exists=1
    fi
    if docker images --format "{{.Repository}}:{{.Tag}}" 2>/dev/null \
       | grep -q "${z_bottle_vessel}:k[0-9]"; then
      z_bottle_image_exists=1
    fi
  fi

  # Crucible charged
  # [INFRA-NEEDED] Currently rbw-cic is a tabtarget (param1 channel).
  # For probe use, need a scriptable function that returns 0/1 without
  # running the full dispatch. For now, check docker containers directly.
  local z_crucible_charged=0
  if test "${z_has_docker}" = "1"; then
    if docker ps --format "{{.Names}}" 2>/dev/null \
       | grep -q "^${z_moniker}-bottle\$"; then
      z_crucible_charged=1
    fi
  fi

  # --- Header ---
  buh_section "Start a Crucible Using Local Builds"
  buh_e
  buh_tlt "A " "Crucible" "${z_docs}#Crucible" " is a sandboxed container environment with enforced"
  buh_t   "network isolation. You are going to build one on your workstation"
  buh_t   "and run Claude Code inside it — no cloud account, no credentials"
  buh_t   "beyond your own Claude subscription."
  buh_e
  buh_tltlt "This track uses the " "ccyolo" "${z_docs}#ccyolo" " nameplate: a Claude Code sandbox that can only reach " "Anthropic" "${z_docs}" "."
  buh_t   "Everything else is blocked."
  buh_e

  # Docker gate
  if test "${z_has_docker}" = "0"; then
    buh_E   "Docker is not available on this machine."
    buh_t   "Install Docker Desktop (or dockerd in WSL) and re-run this handbook."
    buh_e
    buh_tT  "Return to start: " "${RBZ_ONBOARD_START_HERE}"
    buh_e
    return 0
  fi

  buh_t   "Prerequisite: a Claude OAuth subscription (you will authenticate"
  buh_t   "inside the container via copy/paste from your browser)."
  buh_e

  buh_step_style "Step " " — "

  # =================================================================
  # Step 1: Build images locally
  # =================================================================
  buh_step1 "Build images locally"
  buh_e
  buh_tlt "A " "Vessel" "${z_docs}#Vessel" " is a specification for a container image — a Dockerfile"
  buh_tlt "and build context. A " "Hallmark" "${z_docs}#Hallmark" " is a specific build instance,"
  buh_t   "identified by a timestamp tag."
  buh_e
  buh_tlt "" "Kludge" "${z_docs}#Kludge" " builds a vessel image locally using Docker — no cloud, no"
  buh_t   "registry, no credentials. The fastest path from Dockerfile to running"
  buh_t   "container."
  buh_e

  # --- Substep 1a: Kludge the sentry ---
  buh_step2 "Kludge the sentry"
  buh_e
  buh_tlt "The " "Sentry" "${z_docs}#Sentry" " is the gatekeeper container. It runs iptables"
  buh_t   "and dnsmasq to enforce network policy — only domains you whitelist"
  buh_t   "are reachable from inside."
  buh_e
  buh_t   "Kludge it:"
  buh_e
  buh_tTc "   " "${RBZ_KLUDGE_VESSEL}" " ${z_sentry_vessel}"
  buh_e
  buh_t   "The output prints a hallmark tag — something like"
  buh_tc  "" "k260413155458-f9dcb6d9"
  buh_t   "Copy it. You will paste it into the nameplate in Step 2."
  buh_e

  # Sentry kludge image probe
  if test "${z_sentry_image_exists}" = "1"; then
    buh_ct  " [*] " "Kludge-tagged sentry image found locally"
  else
    buh_t   " [ ] No kludge-tagged sentry image found"
  fi
  buh_e

  buh_tW  "" "Kludge requires a clean git tree."
  buh_t   "If you have uncommitted changes, commit them first."
  buh_t   "This is by design — the hallmark tag encodes the commit hash."
  buh_e

  # --- Substep 1b: Kludge the bottle ---
  buh_step2 "Kludge the bottle"
  buh_e
  buh_tlt "The " "Bottle" "${z_docs}#Bottle" " is your workload container — where Claude Code"
  buh_tlt "runs. The " "ccyolo" "${z_docs}#ccyolo" " bottle includes SSH, node, and the Claude CLI."
  buh_e
  buh_t   "This command is different from the sentry kludge. It builds the"
  buh_tlt "bottle AND drives the " "Hallmark" "${z_docs}#Hallmark" " into the nameplate"
  buh_t   "automatically:"
  buh_e
  buh_tTc "   " "${RBZ_CRUCIBLE_KLUDGE_BOTTLE}" " ${z_moniker}"
  buh_e
  buh_t   "Two kludge commands, two different jobs:"
  buh_tct "  " "rbw-fk" "  — standalone vessel kludge. Produces a hallmark, touches nothing else."
  buh_tct "  " "rbw-cKB" " — nameplate-aware bottle kludge. Builds AND installs the hallmark."
  buh_e
  buh_t   "The sentry is shared across nameplates, so you kludge it standalone"
  buh_t   "and drive the hallmark yourself. The bottle belongs to one nameplate,"
  buh_t   "so the tool does the driving for you."
  buh_e

  # Bottle kludge image probe
  if test "${z_bottle_image_exists}" = "1"; then
    buh_ct  " [*] " "Kludge-tagged bottle image found locally"
  else
    buh_t   " [ ] No kludge-tagged bottle image found"
  fi
  buh_e

  # =================================================================
  # Step 2: Drive sentry hallmark into the nameplate
  # =================================================================
  buh_step1 "Drive sentry hallmark into the nameplate"
  buh_e
  buh_tlt "A " "Nameplate" "${z_docs}#Nameplate" " is the file that ties a sentry and bottle"
  buh_t   "together into a runnable unit. It lives at:"
  buh_e
  buh_c   "   ${z_nameplate_file}"
  buh_e
  buh_tlt "The " "RBRN" "${z_docs}#RBRN" " file has two hallmark fields — one for the sentry"
  buh_t   "and one for the bottle. The bottle kludge (Step 1.2) already drove"
  buh_t   "its hallmark. Now you drive the sentry hallmark by hand."
  buh_e
  buh_t   "Open the nameplate file and paste the sentry hallmark from Step 1.1"
  buh_tc  "into the " "RBRN_SENTRY_HALLMARK=" " field."
  buh_e
  buh_t   "This is deliberately manual — it teaches you what hallmarks are and"
  buh_t   "where they live. You just saw the automated version with the bottle;"
  buh_t   "now you see the mechanism underneath."
  buh_e

  # Hallmark probes
  if test "${z_sentry_hallmark_present}" = "1"; then
    buh_tct " [*] " "RBRN_SENTRY_HALLMARK" " = ${z_sentry_hallmark}"
  else
    buh_tE  " [ ] " "RBRN_SENTRY_HALLMARK is empty — paste the sentry hallmark from Step 1.1"
  fi
  if test "${z_bottle_hallmark_present}" = "1"; then
    buh_tct " [*] " "RBRN_BOTTLE_HALLMARK" " = ${z_bottle_hallmark}"
  else
    buh_tE  " [ ] " "RBRN_BOTTLE_HALLMARK is empty — run Step 1.2 (bottle kludge)"
  fi
  buh_e

  buh_t   "After editing, commit the change — the next kludge will need a"
  buh_t   "clean tree:"
  buh_e
  buh_c   "   git add ${z_nameplate_file}"
  buh_c   "   git commit -m \"Drive sentry hallmark into ${z_moniker} nameplate\""
  buh_e

  # =================================================================
  # Step 3: Charge the crucible
  # =================================================================
  buh_step1 "Charge the crucible"
  buh_e
  buh_tlt "" "Charge" "${z_docs}#Charge" " starts three containers from your nameplate:"
  buh_e
  buh_tlt "  " "Sentry" "${z_docs}#Sentry" "   — runs iptables + dnsmasq, enforces the network allowlist"
  buh_tlt "  " "Pentacle" "${z_docs}#Pentacle" "  — establishes the network namespace shared with the bottle"
  buh_tlt "  " "Bottle" "${z_docs}#Bottle" "    — your workload container (Claude Code lives here)"
  buh_e
  buh_t   "The sentry mediates all traffic. The bottle never touches the"
  buh_t   "network directly — everything routes through the sentry's rules."
  buh_e
  buh_tI  "   " "${RBZ_CRUCIBLE_CHARGE}" "${z_moniker}"
  buh_e
  buh_t   "Charge takes 10-30 seconds. It pulls the images from your local"
  buh_t   "Docker, creates the enclave network, starts the containers, waits"
  buh_t   "for the sentry to confirm its iptables rules are applied, then"
  buh_t   "starts the bottle."
  buh_e

  # Charged probe
  if test "${z_crucible_charged}" = "1"; then
    buh_ct  " [*] " "Crucible ${z_moniker} is charged (bottle container running)"
  else
    buh_t   " [ ] Crucible ${z_moniker} is not charged"
  fi
  buh_e

  # =================================================================
  # Step 4: Enter the container and run Claude Code
  # =================================================================
  buh_step1 "Enter the container and run Claude Code"
  buh_e
  buh_t   "SSH into the bottle:"
  buh_e
  buh_tc  "   " "${z_ssh_tabtarget}"
  buh_e
  buh_t   "You land as the claude user in ~/workspace, which contains"
  buh_t   "a small sample project. Run Claude Code:"
  buh_e
  buh_c   "   claude"
  buh_e
  buh_t   "Claude Code will prompt you to authenticate. It opens a URL —"
  buh_t   "copy it to your workstation browser, sign in with your Claude"
  buh_t   "subscription, and paste the code back into the terminal."
  buh_e
  buh_tW  "" "The ccyolo bottle pins Claude Code to a specific version."
  buh_t   "Versions after v2.1.89 have a regression where paste does not"
  buh_t   "work in the OAuth input prompt through SSH or docker exec"
  buh_t   "(github.com/anthropics/claude-code/issues/47745). If you"
  buh_t   "update the pin and paste stops working, type the code manually."
  buh_e
  buh_t   "Once authenticated, Claude Code starts in full autonomy mode —"
  buh_t   "no permission prompts. Inside a network-contained crucible,"
  buh_t   "this is the correct posture: the sentry enforces the real"
  buh_t   "security boundary, not the tool permission system."
  buh_e
  buh_t   "Try your first interaction:"
  buh_e
  buh_c   "   The count_words.sh script has bugs — can you find and fix them?"
  buh_e
  buh_t   "Watch Claude read the files, identify issues, and edit the code."
  buh_t   "The workspace persists across charge/quench cycles — your changes"
  buh_t   "survive restarts."
  buh_e
  buh_t   "Why SSH instead of docker exec?"
  buh_e
  buh_t   "Docker exec is laggy and breaks terminal resize — Claude Code's"
  buh_t   "interactive display needs correct dimensions. SSH gives a proper"
  buh_t   "login session with full terminal negotiation."
  buh_e
  buh_tlt "If SSH fails, " "Rack" "${z_docs}#Rack" " is the diagnostic fallback —"
  buh_t   "docker exec into the bottle to inspect state:"
  buh_e
  buh_tTc "   " "${RBZ_CRUCIBLE_RACK}" " ${z_moniker}"
  buh_e

  # =================================================================
  # Step 5: Verify network containment
  # =================================================================
  buh_step1 "Verify network containment"
  buh_e
  buh_tlt "From inside the " "Bottle" "${z_docs}#Bottle" " (while SSH'd in), you can test what's reachable."
  buh_tlt "The " "ccyolo" "${z_docs}#ccyolo" " nameplate allows Anthropic and example.com (a test"
  buh_t   "target). Everything else is blocked."
  buh_e
  buh_t   "Run these curl commands inside the bottle:"
  buh_e
  buh_c   "   curl -s -o /dev/null -w '%{http_code}' https://api.anthropic.com"
  buh_t   "   Expected: 404 (API wants auth, not bare GET)"
  buh_e
  buh_c   "   curl -s -o /dev/null -w '%{http_code}' https://claude.ai"
  buh_t   "   Expected: 403 (web app)"
  buh_e
  buh_c   "   curl -s -o /dev/null -w '%{http_code}' https://example.com"
  buh_t   "   Expected: 200 (test target on allowlist)"
  buh_e
  buh_c   "   curl -s -o /dev/null -w '%{http_code}' --max-time 5 https://google.com"
  buh_t   "   Expected: 000 or timeout (blocked)"
  buh_e
  buh_c   "   curl -s -o /dev/null -w '%{http_code}' --max-time 5 https://registry.npmjs.org"
  buh_t   "   Expected: 000 or timeout (blocked)"
  buh_e
  buh_tlt "The " "Sentry" "${z_docs}#Sentry" " enforces this with two layers:"
  buh_t   "dnsmasq resolves only whitelisted domains; iptables drops"
  buh_t   "packets to any IP not in the CIDR allowlist. Both layers"
  buh_t   "must agree for traffic to pass."
  buh_e
  buh_tltlt "example.com is included in the " "ccyolo" "${z_docs}#ccyolo" " " "Nameplate" "${z_docs}#Nameplate" " specifically"
  buh_t   "for this verification step — it proves the allowlist works"
  buh_t   "for a non-Anthropic domain."
  buh_e

  # =================================================================
  # Step 6: Quench the crucible
  # =================================================================
  buh_step1 "Quench the crucible"
  buh_e
  buh_tlt "" "Quench" "${z_docs}#Quench" " stops and removes all three containers and the"
  buh_t   "enclave network:"
  buh_e
  buh_tI  "   " "${RBZ_CRUCIBLE_QUENCH}" "${z_moniker}"
  buh_e
  buh_t   "Clean shutdown. The images stay cached locally — your next charge"
  buh_t   "reuses them instantly."
  buh_e

  # =================================================================
  # Iteration loop
  # =================================================================
  buh_section "The iteration loop"
  buh_e
  buh_t   "You now have the full local cycle:"
  buh_e
  buh_t   "  1. Edit the Dockerfile or entrypoint"
  buh_tlt "  2. Commit your changes (" "Kludge" "${z_docs}#Kludge" " needs a clean tree)"
  buh_tltlt "  3. " "Kludge" "${z_docs}#Kludge" " the " "Bottle" "${z_docs}#Bottle" ":"
  buh_tTc   "       " "${RBZ_CRUCIBLE_KLUDGE_BOTTLE}" " ${z_moniker}"
  buh_tlt   "  4. Commit the " "Hallmark" "${z_docs}#Hallmark" " change"
  buh_tlt   "  5. " "Charge" "${z_docs}#Charge" ": Stop and restart"
  buh_tI    "       " "${RBZ_CRUCIBLE_CHARGE}" "${z_moniker}"
  buh_t     "  6. SSH in:"
  buh_tc    "       " "${z_ssh_tabtarget}"
  buh_t     "  7. Test your changes"
  buh_e
  buh_tlt "" "Charge" "${z_docs}#Charge" " tears down any prior state before starting, so you"
  buh_t   "don't need to Quench between iterations — just Charge again."
  buh_e
  buh_tlt "Steps 3-7 take under a minute. The " "Sentry" "${z_docs}#Sentry" " rarely changes, so"
  buh_tlt "you almost never re-kludge it — the " "Bottle" "${z_docs}#Bottle" " is your iteration"
  buh_t   "target."
  buh_e

  # --- Return to start ---
  buh_tT  "Return to start: " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

######################################################################
# Director First Cloud Build — inscribe, conjure, tour, summon, abjure
#
# Frame 4-refined handbook: teaching prose + probes + tabtarget refs.
# Target learner: director doing their first cloud build.
#
# Vessel: rbev-sentry-deb-tether (conjure mode, tethered)
# Teaches: full conjure lifecycle from reliquary through cleanup.
#
# ₢A6AAU — Director subtracks, first cloud build track.

rbho_director_first_build() {
  buc_doc_brief "Your First Cloud Build — inscribe, conjure, tour, summon, abjure"
  buc_doc_shown || return 0

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"
  local -r z_vessel="rbev-sentry-deb-tether"

  # --- Probes ---

  # Director credential present
  local z_has_director=0 z_secrets_dir=""
  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/${RBCC_role_director}/${RBCC_rbra_file}"; then
    z_has_director=1
  fi

  # Depot configured
  local z_has_depot=0
  if test -f "${RBBC_rbrr_file}"; then
    local z_line=""
    while IFS= read -r z_line; do
      case "${z_line}" in RBRR_DEPOT_PROJECT_ID=?*) z_has_depot=1; break ;; esac
    done < "${RBBC_rbrr_file}"
  fi

  # Conjured sentry image summoned locally (c-prefixed hallmark from GAR)
  local z_conjure_summoned=0
  if command -v docker >/dev/null 2>&1; then
    local z_project_id="" z_region=""
    if test -f "${RBBC_rbrr_file}"; then
      z_project_id=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_DEPOT_PROJECT_ID") || z_project_id=""
      z_region=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_GCP_REGION") || z_region=""
    fi
    if test -n "${z_region}" && test -n "${z_project_id}"; then
      if docker images --format "{{.Repository}}:{{.Tag}}" 2>/dev/null \
         | grep -q "${z_region}${RBGC_GAR_HOST_SUFFIX}/${z_project_id}/.*${z_vessel}:c[0-9]"; then
        z_conjure_summoned=1
      fi
    fi
  fi

  # --- Header ---
  buh_section "Your First Cloud Build"
  buh_e
  buh_tlt "This track walks you through the complete " "Conjure" "${z_docs}#Conjure" " lifecycle:"
  buh_tltlt "provision the builder toolchain, " "Ordain" "${z_docs}#Ordain" " your first " "Vessel" "${z_docs}#Vessel" " via"
  buh_t   "Cloud Build, inspect the result, pull it locally, and clean up."
  buh_e
  buh_tltlt "You will build " "rbev-sentry-deb-tether" "${z_docs}#Vessel" " — the same " "Sentry" "${z_docs}#Sentry" " you"
  buh_tlt "already know from the " "Crucible" "${z_docs}#Crucible" " track, but this time built by"
  buh_t   "Google Cloud Build with full SLSA provenance."
  buh_e

  # Prerequisite probes
  buh_t   "Prerequisites:"
  buh_e
  if test "${z_has_director}" = "1"; then
    buh_ct  " [*] " "Director credential installed"
  else
    buh_tEt " [ ] " "Director credential missing" " — run:"
    buh_tT  "      " "${RBZ_ONBOARD_CRED_DIRECTOR}"
  fi
  if test "${z_has_depot}" = "1"; then
    buh_ct  " [*] " "Depot configured (RBRR_DEPOT_PROJECT_ID populated)"
  else
    buh_tEt " [ ] " "Depot not configured" " — the Payor must establish the Depot first:"
    buh_tT  "      " "${RBZ_ONBOARD_PAYOR_HB}"
  fi
  buh_e

  if test "${z_has_director}" = "0" || test "${z_has_depot}" = "0"; then
    buh_E   "Complete the prerequisites above before continuing."
    buh_e
    buh_tT  "Return to start: " "${RBZ_ONBOARD_START_HERE}"
    buh_e
    return 0
  fi

  buh_step_style "Step " " — "

  # =================================================================
  # Step 1: Inscribe the Reliquary
  # =================================================================
  buh_step1 "Inscribe the Reliquary"
  buh_e
  buh_tlt "The " "Reliquary" "${z_docs}#Reliquary" " is a set of builder tool images (skopeo,"
  buh_tlt "docker, gcloud, syft) that Cloud Build uses during " "Vessel" "${z_docs}#Vessel" ""
  buh_t   "construction. Without it, conjure's preflight check fails."
  buh_e
  buh_t   "Think of it as installing the toolchain before your first build."
  buh_t   "This is a one-time operation — once inscribed, the reliquary"
  buh_tlt "stays in the " "Depot" "${z_docs}#Depot" " until you choose to refresh it."
  buh_e
  buh_tlt "Periodically re-inscribe to pick up newer tool versions. All " "Vessels" "${z_docs}#Vessel" ""
  buh_tlt "share the same " "Reliquary" "${z_docs}#Reliquary" " — one inscribe updates the toolchain"
  buh_t   "for every build."
  buh_e
  buh_t   "Inscribe:"
  buh_e
  buh_tT  "   " "${RBZ_INSCRIBE_RELIQUARY}"
  buh_e
  buh_t   "This mirrors four tool images from upstream into your Depot's"
  buh_t   "GAR. Takes 2-5 minutes depending on network speed."
  buh_e
  buh_t   "When inscribe completes, it prints a reliquary datestamp"
  buh_tct "(e.g., " "r260324193326" "). Every vessel that uses Cloud Build"
  buh_t   "needs this value in its regime file:"
  buh_e
  buh_c   "   RBRV_RELIQUARY=<datestamp>"
  buh_e
  buh_tct "Open " "rbev-sentry-deb-tether/rbrv.env" " and set the field,"
  buh_t   "then commit the change."
  buh_e

  # =================================================================
  # Step 2: Conjure the sentry (tethered)
  # =================================================================
  buh_step1 "Conjure the sentry"
  buh_e
  buh_tlt "" "Conjure" "${z_docs}#Conjure" " is the build mode where Cloud Build constructs a"
  buh_t   "vessel image from the project's Dockerfile and build context."
  buh_e
  buh_tlt "" "Ordain" "${z_docs}#Ordain" " is the command that triggers the full pipeline —"
  buh_tlt "it reads the vessel's " "RBRV" "${z_docs}#RBRV" " regime to determine the mode"
  buh_t   "(conjure, bind, or graft) and acts accordingly:"
  buh_e
  buh_tTc "   " "${RBZ_ORDAIN_HALLMARK}" " ${z_vessel}"
  buh_e
  buh_tlt "This builds on the " "Tethered" "${z_docs}#Tethered" " pool — Cloud Build has"
  buh_t   "public internet access and pulls base images from upstream"
  buh_tlt "registries during the build. (The " "Airgap" "${z_docs}#Airgap" " track removes"
  buh_t   "that dependency.)"
  buh_e
  buh_t   "The pipeline:"
  buh_e
  buh_tlt "  1. The host mints a " "Hallmark" "${z_docs}#Hallmark" " — a timestamped tag"
  buh_t   "     identifying this build"
  buh_tlt "  2. A " "Pouch" "${z_docs}#Pouch" " (build context archive) is pushed to GAR"
  buh_t   "  3. Cloud Build constructs the image across platforms"
  buh_t   "  4. SLSA provenance is generated per platform digest"
  buh_tlt "  5. " "Vouch" "${z_docs}#Vouch" " verifies the provenance chain"
  buh_e
  buh_tW  "" "Wall-clock: ~15-20 minutes for a 3-platform build."
  buh_t   "The command blocks until Cloud Build finishes. Use the time"
  buh_t   "to read ahead — the next steps explain what to look for."
  buh_e

  # =================================================================
  # Step 3: Capture the hallmark
  # =================================================================
  buh_step1 "Capture the hallmark"
  buh_e
  buh_tlt "When " "Ordain" "${z_docs}#Ordain" " completes, it prints the " "Hallmark" "${z_docs}#Hallmark" ""
  buh_t   "— a timestamped tag identifying this build. Set these"
  buh_t   "environment variables so you can copy-paste the commands"
  buh_t   "in the remaining steps:"
  buh_e
  buh_c   "   export ONBOARD_VESSEL=${z_vessel}"
  buh_c   "   export ONBOARD_HALLMARK=<paste hallmark from ordain output>"
  buh_e

  # =================================================================
  # Step 4: Tour the hallmark artifacts
  # =================================================================
  buh_step1 "Tour the hallmark artifacts"
  buh_e
  buh_tlt "Every conjured " "Hallmark" "${z_docs}#Hallmark" " produces a set of tagged"
  buh_t   "artifacts in GAR. Each suffix serves a specific role:"
  buh_e
  buh_tc  "   " "{hallmark}${RBGC_ARK_SUFFIX_POUCH}"
  buh_t   "      A FROM SCRATCH OCI image pushed from host to GAR before"
  buh_t   "      the build. Contains the Dockerfile, scripts, and"
  buh_t   "      configuration Cloud Build needs. Identical for tethered"
  buh_t   "      and airgapped builds — the pool determines network"
  buh_t   "      access, not the pouch."
  buh_e
  buh_tc  "   " "{hallmark}${RBGC_ARK_SUFFIX_IMAGE}"
  buh_t   "      The consumer image — a multiplatform manifest list."
  buh_t   "      This is what you pull and run."
  buh_e
  buh_tc  "   " "{hallmark}${RBGC_ARK_SUFFIX_ATTEST}-{arch}"
  buh_t   "      Per-platform provenance-carrying image (one per platform)."
  buh_t   "      Shares all layers with -image — only the manifest differs."
  buh_t   "      These carry the GCB-attested digests used by vouch."
  buh_e
  buh_tc  "   " "{hallmark}${RBGC_ARK_SUFFIX_ABOUT}"
  buh_t   "      SBOM (software bill of materials) + build info."
  buh_e
  buh_tc  "   " "{hallmark}${RBGC_ARK_SUFFIX_VOUCH}"
  buh_t   "      SLSA provenance verification record."
  buh_e
  buh_tc  "   " "{hallmark}${RBGC_ARK_SUFFIX_DIAGS}"
  buh_t   "      Diagnostics from the build."
  buh_e
  buh_t   "Inspect them:"
  buh_e

  # --- Substep 3a: Tally ---
  buh_step2 "Tally"
  buh_e
  buh_tlt "" "Tally" "${z_docs}#Tally" " lists all hallmarks and their health state:"
  buh_e
  buh_tT  "   " "${RBZ_TALLY_HALLMARKS}"
  buh_e
  buh_tlt "Look for your " "Hallmark" "${z_docs}#Hallmark" " with health state 'vouched' — that"
  buh_t   "means SLSA provenance was verified."
  buh_e

  # --- Substep 3b: Vouch ---
  buh_step2 "Vouch"
  buh_e
  buh_tlt "" "Vouch" "${z_docs}#Vouch" " verifies SLSA provenance for each platform"
  buh_tltlt "digest in the " "Hallmark" "${z_docs}#Hallmark" ". The ordain pipeline runs " "Vouch" "${z_docs}#Vouch" ""
  buh_tlt "automatically. If a build was interrupted before " "Vouch" "${z_docs}#Vouch" ""
  buh_tltlt "completed, run this to reattempt " "Vouch" "${z_docs}#Vouch" " on untreated " "Hallmarks" "${z_docs}#Hallmark" ":"
  buh_e
  buh_tT  "   " "${RBZ_VOUCH_HALLMARKS}"
  buh_e
  buh_tlt "The " "Conjure" "${z_docs}#Conjure" " verdict is full SLSA — Cloud Build produced"
  buh_t   "this image, and the provenance chain proves it."
  buh_e

  # --- Substep 3c: Plumb ---
  buh_step2 "Plumb"
  buh_e
  buh_tlt "" "Plumb" "${z_docs}#Plumb" " displays the SBOM, build info, and Dockerfile"
  buh_t   "that produced the hallmark. Two modes:"
  buh_e
  buh_tTc "   " "${RBZ_PLUMB_FULL}" ' ${ONBOARD_VESSEL} ${ONBOARD_HALLMARK}'
  buh_t   "   Full provenance display — SBOM packages, build parameters,"
  buh_t   "   Dockerfile content."
  buh_e
  buh_tTc "   " "${RBZ_PLUMB_COMPACT}" ' ${ONBOARD_VESSEL} ${ONBOARD_HALLMARK}'
  buh_t   "   Compact summary — one-line-per-artifact overview."
  buh_e

  # =================================================================
  # Step 5: Summon the hallmark
  # =================================================================
  buh_step1 "Summon the hallmark"
  buh_e
  buh_tlt "" "Summon" "${z_docs}#Summon" " pulls a vouched hallmark image to your local"
  buh_t   "Docker daemon:"
  buh_e
  buh_tTc "   " "${RBZ_SUMMON_HALLMARK}" ' ${ONBOARD_VESSEL} ${ONBOARD_HALLMARK}'
  buh_e
  buh_tc  "   {hallmark}" "${RBGC_ARK_SUFFIX_IMAGE}"
  buh_t   "   is a multiplatform manifest list."
  buh_t   "   Docker resolves it to the image matching your host"
  buh_t   "   architecture — the same image that Charge uses when"
  buh_tlt "   starting a " "Crucible" "${z_docs}#Crucible" " from cloud-built hallmarks."
  buh_e

  # Summoned probe
  if test "${z_conjure_summoned}" = "1"; then
    buh_ct  " [*] " "Conjured sentry image found locally (summoned from GAR)"
  else
    buh_t   " [ ] No conjured sentry image found locally — run Summon above"
  fi
  buh_e

  # =================================================================
  # Step 6: Abjure and Rekon — hallmark lifecycle
  # =================================================================
  buh_step1 "Abjure and Rekon — hallmark lifecycle"
  buh_e
  buh_tltlt "" "Rekon" "${z_docs}#Rekon" " lists the raw tags for a " "Vessel" "${z_docs}#Vessel" ""
  buh_tlt "package in GAR. Run it before and after " "Abjure" "${z_docs}#Abjure" " to see"
  buh_t   "the full lifecycle:"
  buh_e
  buh_tTc "   " "${RBZ_REKON_IMAGE}" " ${z_vessel}"
  buh_e
  buh_tlt "You should see all five durable tags for your " "Hallmark" "${z_docs}#Hallmark" ":"
  buh_tc  "   " "${RBGC_ARK_SUFFIX_IMAGE}, ${RBGC_ARK_SUFFIX_ABOUT}, ${RBGC_ARK_SUFFIX_VOUCH}, ${RBGC_ARK_SUFFIX_POUCH}, ${RBGC_ARK_SUFFIX_DIAGS}"
  buh_e
  buh_tltlt "" "Abjure" "${z_docs}#Abjure" " removes all artifacts for a " "Hallmark" "${z_docs}#Hallmark" ""
  buh_t   "from GAR. This is permanent — the hallmark and all its"
  buh_t   "tags are deleted:"
  buh_e
  buh_tTc "   " "${RBZ_ABJURE_HALLMARK}" ' ${ONBOARD_VESSEL} ${ONBOARD_HALLMARK}'
  buh_e
  buh_t   "After abjure, run Rekon again:"
  buh_e
  buh_tTc "   " "${RBZ_REKON_IMAGE}" " ${z_vessel}"
  buh_e
  buh_tlt "The tags for your " "Hallmark" "${z_docs}#Hallmark" " should be gone. The image is no"
  buh_tlt "longer in the " "Depot" "${z_docs}#Depot" "."
  buh_e

  # =================================================================
  # What you learned
  # =================================================================
  buh_section "What you learned"
  buh_e
  buh_t   "You just completed the full conjure lifecycle:"
  buh_e
  buh_tlt "  1. " "Reliquary" "${z_docs}#Reliquary" " — builder toolchain provisioned"
  buh_tlt "  2. " "Conjure" "${z_docs}#Conjure" " — vessel built by Cloud Build with SLSA provenance"
  buh_tltlt "  3. " "Tally" "${z_docs}#Tally" "/" "Vouch" "${z_docs}#Vouch" " — health and provenance verified"
  buh_tlt "  4. " "Plumb" "${z_docs}#Plumb" " — SBOM and build info inspected"
  buh_tlt "  5. " "Summon" "${z_docs}#Summon" " — consumer image pulled locally"
  buh_tltlt "  6. " "Abjure" "${z_docs}#Abjure" "/" "Rekon" "${z_docs}#Rekon" " — lifecycle cleanup"
  buh_e
  buh_tlt "This was a " "Tethered" "${z_docs}#Tethered" " build — Cloud Build had"
  buh_t   "internet access. The next track teaches you to remove"
  buh_t   "that dependency entirely."
  buh_e

  # --- Return to start ---
  buh_tT  "Return to start: " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

######################################################################
# Payor handbook — establish a Manor and provision the Depot
#
# Linear step sequence, no conditional probes. The payor owns the GCP
# project and funds it. This handbook walks through the full ceremony:
# OAuth credentials, project setup, depot provisioning, governor handoff.

rbho_payor_handbook() {
  buc_doc_brief "Payor — establish a Manor and provision the Depot"
  buc_doc_shown || return 0

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  # --- Header ---
  buh_section "Payor — Establish a Manor and Provision the Depot"
  buh_e
  buh_tltlt "The " "Payor" "${z_docs}#Payor" " establishes a " "Manor" "${z_docs}#Manor" " — an administrative seat"
  buh_t   "holding the billing account, OAuth client, and operator identity."
  buh_tlt "Unlike other roles that use service account keys, the " "Payor" "${z_docs}#Payor" ""
  buh_t   "authenticates via OAuth — representing the human project owner."
  buh_e
  buh_tltlt "By the end of this handbook you will have a " "Manor" "${z_docs}#Manor" ", a " "Depot" "${z_docs}#Depot" ""
  buh_tlt "funded under it, and a " "Governor" "${z_docs}#Governor" " service account ready to administer it."
  buh_e

  buh_t   "This ceremony takes about 15 minutes."
  buh_e

  buh_step_style "Step " " — "

  # =================================================================
  # Step 1: Establish the Manor
  # =================================================================
  buh_step1 "Establish the Manor"
  buh_e
  buh_tlt "The " "Manor's" "${z_docs}#Manor" " GCP project hosts the OAuth client and billing"
  buh_t   "account. It must be created before any infrastructure can be"
  buh_t   "provisioned."
  buh_e
  buh_t   "Run the guided setup:"
  buh_tT  "  " "${RBZ_PAYOR_ESTABLISH}"
  buh_e
  buh_tlt "This guides you through creating the " "Manor's" "${z_docs}#Manor" " GCP project,"
  buh_tlt "enabling billing, and configuring the OAuth consent screen. The " "Manor" "${z_docs}#Manor" ""
  buh_tlt "identity is recorded in " "RBRP" "${z_docs}#RBRP" "."
  buh_e

  # =================================================================
  # Step 2: Install OAuth credentials
  # =================================================================
  buh_step1 "Install OAuth credentials"
  buh_e
  buh_t   "Step 1 ended with downloading a JSON client secret file from the"
  buh_t   "OAuth client you just created. Install it:"
  buh_e
  buh_tTc "  " "${RBZ_PAYOR_INSTALL}" " \${HOME}/Downloads/client_secret_*.json"
  buh_e
  buh_t   "This walks you through the OAuth authorization flow and stores"
  buh_t   "the credential securely."
  buh_e
  buh_t   "If you are refreshing an existing credential that has expired:"
  buh_tT  "  " "${RBZ_PAYOR_REFRESH}"
  buh_e

  # =================================================================
  # Step 3: Provision the Depot
  # =================================================================
  buh_step1 "Provision the Depot"
  buh_e
  buh_tlt "A " "Depot" "${z_docs}#Depot" " is the facility where container images are built and"
  buh_t   "stored — a GCP project with a container repository, storage bucket,"
  buh_tlt "and build infrastructure, funded under the " "Manor's" "${z_docs}#Manor" " billing account."
  buh_tltlt "A " "Governor" "${z_docs}#Governor" " administers the " "Depot" "${z_docs}#Depot" " — creating"
  buh_tltlt "" "Retriever" "${z_docs}#Retriever" " and " "Director" "${z_docs}#Director" " accounts for those who build and"
  buh_t   "retrieve container images."
  buh_e
  buh_tlt "" "Payor" "${z_docs}#Payor" " creates the Depot:"
  buh_tT  "  " "${RBZ_LEVY_DEPOT}"
  buh_e
  buh_t   "This enables APIs, creates the Artifact Registry repository and"
  buh_t   "Cloud Storage bucket, and configures Cloud Build."
  buh_e
  buh_tlt "" "Payor" "${z_docs}#Payor" " can list Depots for verification:"
  buh_tT  "  " "${RBZ_LIST_DEPOT}"
  buh_e
  buh_tlt "" "Payor" "${z_docs}#Payor" " creates the Governor service account:"
  buh_tT  "  " "${RBZ_MANTLE_GOVERNOR}"
  buh_e
  buh_t   "Hand the resulting key file to the person who will administer"
  buh_tltltlt "this " "Depot" "${z_docs}#Depot" ". After this handoff, the " "Governor" "${z_docs}#Governor" " can create"
  "" "Retriever" "${z_docs}#Retriever" " and " "Director" "${z_docs}#Director" " accounts independently."
  buh_e
  buh_tltlt "The " "Payor's" "${z_docs}#Payor" " job for this " "Depot" "${z_docs}#Depot" " is done unless billing or"
  buh_t   "project-level changes are needed."
  buh_e

  # --- Return to start ---
  buh_tT  "Return to start: " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

######################################################################
# Governor handbook — administer service accounts for directors and
# retrievers.
#
# Linear step sequence, no conditional probes. The governor operates
# within a depot the payor created: installs governor credentials,
# provisions downstream SAs, verifies the chain.

rbho_governor_handbook() {
  buc_doc_brief "Governor — administer service accounts for directors and retrievers"
  buc_doc_shown || return 0

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  # --- Header ---
  buh_section "Governor — Administer Service Accounts"
  buh_e
  buh_tltlt "A " "Governor" "${z_docs}#Governor" " administers a " "Depot" "${z_docs}#Depot" " — creating service accounts"
  buh_t   "and managing access for those who build and run container images."
  buh_e

  buh_step_style "Step " " — "

  # =================================================================
  # Step 1: Install governor credentials
  # =================================================================
  buh_step1 "Install governor credentials"
  buh_e
  buh_tltlt "The " "Governor" "${z_docs}#Governor" " works within a " "Depot" "${z_docs}#Depot" " provisioned under the"
  buh_tltlt "" "Payor's" "${z_docs}#Payor" " " "Manor" "${z_docs}#Manor" ". Your Payor creates your credential by running:"
  buh_tT  "  " "${RBZ_MANTLE_GOVERNOR}"
  buh_e
  buh_tltlt "If no " "Depot" "${z_docs}#Depot" " exists yet, the " "Payor" "${z_docs}#Payor" " establishes one first:"
  buh_tT  "  " "${RBZ_ONBOARD_PAYOR_HB}"
  buh_e
  buh_t   "Install the resulting key file into the secrets directory under"
  buh_t   "the governor role subdirectory. The path is derived from your"
  buh_tlt "" "RBRR" "${z_docs}#RBRR" " configuration — check RBRR_SECRETS_DIR for the location."
  buh_e

  # =================================================================
  # Step 2: Provision downstream service accounts
  # =================================================================
  buh_step1 "Provision downstream service accounts"
  buh_e
  buh_t   "The governor provisions access for two downstream roles:"
  buh_e
  buh_tltlt "A " "Retriever" "${z_docs}#Retriever" " has read access to the " "Depot" "${z_docs}#Depot" " — they pull and run"
  buh_t   "container images that others have built."
  buh_tlt "A " "Director" "${z_docs}#Director" " has build and publish access — they create container"
  buh_t   "images and push them to the registry."
  buh_e
  buh_tltlt "Create a " "Retriever" "${z_docs}#Retriever" " with read access (" "Charter" "${z_docs}#Charter" "):"
  buh_tT  "  " "${RBZ_CHARTER_RETRIEVER}"
  buh_e
  buh_tltlt "Create a " "Director" "${z_docs}#Director" " with build access (" "Knight" "${z_docs}#Knight" "):"
  buh_tT  "  " "${RBZ_KNIGHT_DIRECTOR}"
  buh_e
  buh_t   "Each command creates the service account and applies the IAM"
  buh_tlt "grants it needs. The output is an " "RBRA" "${z_docs}#RBRA" " key file — hand it to"
  buh_t   "the Retriever or Director user."
  buh_e
  buh_t   "List issued service accounts:"
  buh_tT  "  " "${RBZ_LIST_SERVICE_ACCOUNTS}"
  buh_e

  # =================================================================
  # Step 3: Verify the chain
  # =================================================================
  buh_step1 "Verify the chain"
  buh_e
  buh_t   "The service accounts you created include IAM grants — each SA"
  buh_t   "gets exactly the permissions its role requires, no more."
  buh_tlt "" "Retriever" "${z_docs}#Retriever" " gets read access."
  buh_tlt "" "Director" "${z_docs}#Director" " gets read, write, and build trigger access."
  buh_e
  buh_t   "Verify the complete chain works by installing both credentials"
  buh_t   "locally and running the credential handbook tracks:"
  buh_tT  "  " "${RBZ_ONBOARD_CRED_RETRIEVER}"
  buh_tT  "  " "${RBZ_ONBOARD_CRED_DIRECTOR}"
  buh_e
  buh_tltlt "If the " "Retriever" "${z_docs}#Retriever" " can pull from the " "Depot" "${z_docs}#Depot" " and the"
  buh_tlt "" "Director" "${z_docs}#Director" " can see the registry, your grants are correct."
  buh_e

  # --- Return to start ---
  buh_tT  "Return to start: " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

# eof
