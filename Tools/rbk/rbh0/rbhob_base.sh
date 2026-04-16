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
# Recipe Bottle Handbook Onboarding - Base (kindle, sentinel, probes, shared helpers)

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

  # BCG stderr-capture prefixes for docker probes — discriminator appended at use site.
  # BURD_TEMP_DIR is dispatcher-provided (rbho is thin furnish — does not kindle burd).
  readonly ZRBHO_DOCKER_IMAGES_PREFIX="${BURD_TEMP_DIR}/zrbho_docker_images_"
  readonly ZRBHO_DOCKER_PS_PREFIX="${BURD_TEMP_DIR}/zrbho_docker_ps_"
  readonly ZRBHO_DOCKER_STDERR_PREFIX="${BURD_TEMP_DIR}/zrbho_docker_stderr_"

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
    buh_line "${RBYC_PROBE_YES}${z_text}"
  else
    buh_line "${RBYC_PROBE_NO}${z_text}"
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

  local z_line=""

  # Unit 2: Any image from this depot's GAR exists locally
  local z_project_id="" z_region=""
  if test -f "${RBBC_rbrr_file}"; then
    z_project_id=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_DEPOT_PROJECT_ID") || z_project_id=""
    z_region=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_GCP_REGION") || z_region=""
  fi
  if test -n "${z_region}" && test -n "${z_project_id}"; then
    local -r z_gar_prefix="${z_region}${RBGC_GAR_HOST_SUFFIX}/${z_project_id}/"
    local -r z_ru2_out="${ZRBHO_DOCKER_IMAGES_PREFIX}1_repo.txt"
    local -r z_ru2_err="${ZRBHO_DOCKER_STDERR_PREFIX}1_repo.txt"
    if docker images --format "{{.Repository}}" > "${z_ru2_out}" 2>"${z_ru2_err}"; then
      while IFS= read -r z_line || test -n "${z_line}"; do
        case "${z_line}" in "${z_gar_prefix}"*) z_ru2=1; break ;; esac
      done < "${z_ru2_out}"
    fi
  fi

  # Unit 3: Any crucible is charged (bottle container running)
  local -r z_ru3_out="${ZRBHO_DOCKER_PS_PREFIX}1_names.txt"
  local -r z_ru3_err="${ZRBHO_DOCKER_STDERR_PREFIX}2_ps.txt"
  if docker ps --format "{{.Names}}" > "${z_ru3_out}" 2>"${z_ru3_err}"; then
    while IFS= read -r z_line || test -n "${z_line}"; do
      case "${z_line}" in *-bottle) z_ru3=1; break ;; esac
    done < "${z_ru3_out}"
  fi

  # Unit 4: Kludge-tagged image exists (k-prefixed hallmark)
  local -r z_ru4_out="${ZRBHO_DOCKER_IMAGES_PREFIX}2_tag.txt"
  local -r z_ru4_err="${ZRBHO_DOCKER_STDERR_PREFIX}3_tag.txt"
  if docker images --format "{{.Tag}}" > "${z_ru4_out}" 2>"${z_ru4_err}"; then
    while IFS= read -r z_line || test -n "${z_line}"; do
      case "${z_line}" in k[0-9]*) z_ru4=1; break ;; esac
    done < "${z_ru4_out}"
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
    local -r z_gar_prefix="${z_region}${RBGC_GAR_HOST_SUFFIX}/${z_project_id}/"
    local -r z_gu3_out="${ZRBHO_DOCKER_IMAGES_PREFIX}3_repo.txt"
    local -r z_gu3_err="${ZRBHO_DOCKER_STDERR_PREFIX}4_repo.txt"
    if docker images --format "{{.Repository}}" > "${z_gu3_out}" 2>"${z_gu3_err}"; then
      local z_line=""
      while IFS= read -r z_line || test -n "${z_line}"; do
        case "${z_line}" in "${z_gar_prefix}"*) z_gu3=1; break ;; esac
      done < "${z_gu3_out}"
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
  buyy_link_yawp "${RBRR_PUBLIC_DOCS_URL}" "${z_name}"; local -r z_link="${z_buym_yelp}"
  # Column pad lives OUTSIDE the link envelope so the underline/OSC-8 stop
  # at the end of the role name.  Width 13 = 12-char column + 1 separator.
  local z_pad=""
  printf -v z_pad '%*s' $((13 - ${#z_name})) ''
  if test "${z_detected}" = "1"; then
    buyy_tt_yawp "${z_colophon}"; local -r z_tt="${z_buym_yelp}"
    buh_line " [*] ${z_link}${z_pad}${z_tt}"
  else
    buh_line " [ ] ${z_link}"
  fi
}

######################################################################
# Legacy role-track functions (triage, reference, retriever, director,
# governor, payor) removed — replaced by intent-organized handbook
# tracks below. See ₣A6 paddock "Context — The Malformation".

######################################################################
# Credential installation — shared utility for retriever/director
#
# Args: role_display  role_constant  knight_tabtarget_constant  role_description
# Probes RBRR for secrets dir, checks credential file, shows install
# path, points to auth regime validator.

zrbho_credential_install() {
  local -r z_role_constant="${1}"

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
  buh_line "A ${RBYC_GOVERNOR} produces ${RBYC_RBRA} credential files for ${RBYC_DIRECTORS} and ${RBYC_RETRIEVERS}."
  buh_line "Your ${RBYC_GOVERNOR} hands you this file out-of-band — it is a"
  buh_line "secret, never committed to the repo."
  buh_e

  # --- Step 2: Install the key file ---
  buh_step1 "Install the key file"
  buh_e
  if test -n "${z_secrets_dir}"; then
    buh_line "Place the file at the path derived from ${RBYC_RBRR}:"
    buh_e
    buh_code "   ${z_secrets_dir}/${z_role_constant}/${RBCC_rbra_file}"
    buh_e
    buh_line "Create the directory if it does not exist."
  else
    buh_warn "RBRR not populated — cannot determine credential path."
    buyy_link_yawp "${RBRR_PUBLIC_DOCS_URL}" "BURC" "Configure your Repo's Environment"; local -r z_env_link="${z_buym_yelp}"
    buh_line "Run ${z_env_link} first."
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
  buh_line "Run the ${RBYC_RBRA} validator for your role:"
  buh_e
  buh_tt   "   " "${RBZ_VALIDATE_AUTH}" "" " ${z_role_constant}"
  buh_e
  buh_line "Read the output — it checks the file format and reports"
  buh_line "what the credential grants."
  buh_e

  # Callers append role-specific verification and closing steps
}

# eof
