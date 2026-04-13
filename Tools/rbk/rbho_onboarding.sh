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
    test -f "${z_secrets_dir}/rbro-payor.env"                          && z_has_payor=1
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
#   Unit 2,4,6: rbev-sentry-debian-slim (conjure vessel)
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
      *"rbev-sentry-debian-slim:k"[0-9]*) z_du2=1 ;;
      *"rbev-sentry-debian-slim:c"[0-9]*) z_conjure_found=1 ;;
      *"rbev-bottle-plantuml:b"[0-9]*)    z_du5=1 ;;
      *"rbev-sentry-debian-slim:g"[0-9]*) z_du6=1 ;;
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
# Unit 1: OAuth credential present (rbro-payor.env in secrets dir)
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
     test -f "${z_secrets_dir}/rbro-payor.env"; then
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

rbho_triage() {
  # No sentinel — works pre-kindle (probes filesystem only)

  buc_doc_brief "Detect credential roles and route to per-role onboarding walkthrough"
  buc_doc_shown || return 0

  local z_has_payor z_has_governor z_has_director z_has_retriever z_secrets_dir
  zrbho_probe_role_credentials

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  buh_section "Recipe Bottle Onboarding"
  buh_e
  buh_tlt "  " "Recipe Bottle" "${z_docs}" " builds container images with supply-chain provenance"
  buh_t   "  and runs untrusted containers behind enforced network isolation."
  buh_e

  # Each role: detected → walkthrough tabtarget, absent → docs link
  zrbho_triage_role "${z_has_retriever}" "Retriever" "${RBZ_ONBOARD_RETRIEVER}"
  zrbho_triage_role "${z_has_director}"  "Director"  "${RBZ_ONBOARD_DIRECTOR}"
  zrbho_triage_role "${z_has_governor}"  "Governor"  "${RBZ_ONBOARD_GOVERNOR}"
  zrbho_triage_role "${z_has_payor}"     "Payor"     "${RBZ_ONBOARD_PAYOR}"

  buh_e
  buh_t  "  For a full health dashboard across all roles:"
  buh_tT "    " "${RBZ_ONBOARD_REFERENCE}"

}

######################################################################
# Onboarding reference — all roles, all units, single health dashboard

rbho_reference() {
  # No sentinel — works pre-kindle (probes filesystem only)

  buc_doc_brief "Reference dashboard — all roles, all units, current probe status"
  buc_doc_shown || return 0

  local z_has_payor z_has_governor z_has_director z_has_retriever z_secrets_dir
  zrbho_probe_role_credentials

  buh_section "Recipe Bottle — Onboarding Reference"
  buh_e
  buh_t  "  Health dashboard across all roles. Re-run anytime to check status."
  buh_e

  # Retriever — full per-unit probes
  buh_section "Retriever"
  local z_ru1 z_ru2 z_ru3 z_ru4
  zrbho_probe_retriever_units
  zrbho_po_status "${z_ru1}" "  Credential gate — SA key installed"
  zrbho_po_status "${z_ru2}" "  First artifact — hallmark summoned locally"
  zrbho_po_status "${z_ru3}" "  Container runtime — crucible charged"
  zrbho_po_status "${z_ru4}" "  Local experimentation — kludge image present"
  buh_tT "  Walkthrough: " "${RBZ_ONBOARD_RETRIEVER}"
  buh_e

  # Director — full per-unit probes
  buh_section "Director"
  local z_du1=0
  local z_du2=0
  local z_du3=0
  local z_du4=0
  local z_du5=0
  local z_du6=0
  local z_du7=0
  zrbho_probe_director_units
  zrbho_po_status "${z_du1}" "  Credential gate — director SA key installed"
  zrbho_po_status "${z_du2}" "  Local build — kludge image present"
  zrbho_po_status "${z_du3}" "  Depot foundation — base images enshrined"
  zrbho_po_status "${z_du4}" "  Conjure — production build image present"
  zrbho_po_status "${z_du5}" "  Bind — pinned upstream image present"
  zrbho_po_status "${z_du6}" "  Graft — locally-built image pushed"
  zrbho_po_status "${z_du7}" "  Full ark — all three modes compared"
  buh_tT "  Walkthrough: " "${RBZ_ONBOARD_DIRECTOR}"
  buh_e

  # Governor — full per-unit probes
  buh_section "Governor"
  local z_gu1 z_gu2 z_gu3
  zrbho_probe_governor_units
  zrbho_po_status "${z_gu1}" "  Project access — governor credentials installed"
  zrbho_po_status "${z_gu2}" "  Service accounts — retriever and director SAs provisioned"
  zrbho_po_status "${z_gu3}" "  Verification — downstream roles can access the depot"
  buh_tT "  Walkthrough: " "${RBZ_ONBOARD_GOVERNOR}"
  buh_e

  # Payor — full per-unit probes
  buh_section "Payor"
  local z_pu1=0
  local z_pu2=0
  local z_pu3=0
  local z_pu4=0
  zrbho_probe_payor_units
  zrbho_po_status "${z_pu1}" "  OAuth bootstrap — credentials installed"
  zrbho_po_status "${z_pu2}" "  Project setup — GCP project configured"
  zrbho_po_status "${z_pu3}" "  Depot provisioning — infrastructure levied"
  zrbho_po_status "${z_pu4}" "  Governor handoff — governor SA created"
  buh_tT "  Walkthrough: " "${RBZ_ONBOARD_PAYOR}"
  buh_e

}

######################################################################
# Onboarding role walkthroughs — dual-mode rendering

rbho_retriever() {
  buc_doc_brief "Retriever walkthrough — pull and run vessel images"
  buc_doc_shown || return 0

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  # --- Extract config and probe ---
  local z_secrets_dir=""
  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi

  local z_ru1 z_ru2 z_ru3 z_ru4
  zrbho_probe_retriever_units

  # --- Count progress ---
  local z_done=0
  if test "${z_ru1}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_ru2}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_ru3}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_ru4}" = "1"; then z_done=$((z_done + 1)); fi
  local -r z_total=4

  # --- Header ---
  buh_section "Retriever Walkthrough"
  buh_e

  if test "${z_done}" = "${z_total}"; then
    # ============ REFERENCE MODE — all probes green ============
    buh_t  "  All steps complete. Re-run anytime to verify health."
    buh_e
    zrbho_po_status 1 "  Credential gate — SA key installed"
    zrbho_po_status 1 "  First artifact — hallmark summoned locally"
    zrbho_po_status 1 "  Container runtime — crucible charged"
    zrbho_po_status 1 "  Local experimentation — kludge image present"
  else
    # ============ WALKTHROUGH MODE — show frontier unit ============
    local -r z_frontier=$((z_done + 1))
    buh_t  "  Step ${z_frontier} of ${z_total}"
    buh_e

    if test "${z_ru1}" = "0"; then
      # ---- Unit 1: Credential Gate ----
      buh_section "  Credential Gate"
      buh_e
      buh_tlt "  A " "depot" "${z_docs}#Depot" " is the facility where container images are built and stored."
      buh_tlt "  A " "retriever" "${z_docs}#Retriever" " is a role with read access to a depot — you pull and run"
      buh_t   "  container images that others have built."
      buh_e
      buh_t   "  To access a depot, you need a service account key. Your governor creates"
      buh_t   "  one by running:"
      buh_tT  "    " "${RBZ_CHARTER_RETRIEVER}"
      buh_e
      if test -n "${z_secrets_dir}"; then
        buh_t   "  Install the key file to:"
        buh_tc  "    " "${z_secrets_dir}/${RBCC_role_retriever}/${RBCC_rbra_file}"
      else
        buh_tW  "  " "Project not configured — .rbk/rbrr.env not found."
        buh_t   "  Run the payor walkthrough first, or ask your payor for the project files."
      fi
      buh_e
      buh_t   "  Once installed, re-run this walkthrough to continue."

    elif test "${z_ru2}" = "0"; then
      # ---- Unit 2: First Artifact ----
      buh_section "  First Artifact"
      buh_e
      buh_tlt "  A " "vessel" "${z_docs}#Vessel" " is a specification for a container image."
      buh_tlt "  A " "hallmark" "${z_docs}#Hallmark" " is a specific build instance of a vessel, identified by"
      buh_t   "  timestamp."
      buh_e
      buh_tlt "  " "Summon" "${z_docs}#Summon" " pulls a hallmark image from the depot to your local machine:"
      buh_tT  "    " "${RBZ_SUMMON_HALLMARK}"
      buh_e
      buh_t   "  After summoning, inspect the artifact's provenance:"
      buh_tT  "    " "${RBZ_PLUMB_FULL}"
      buh_tT  "    " "${RBZ_PLUMB_COMPACT}"
      buh_e
      buh_tlt "  A " "vouch" "${z_docs}#Vouch" " is cryptographic attestation proving the artifact was built"
      buh_t   "  by trusted infrastructure."
      buh_tlt "  " "Plumb" "${z_docs}#Plumb" " lets you inspect the SBOM, build info, and vouch chain —"
      buh_t   "  this is how you know what you're running."

    elif test "${z_ru3}" = "0"; then
      # ---- Unit 3: Container Runtime ----
      buh_section "  Container Runtime"
      buh_e
      buh_tlt "  A " "bottle" "${z_docs}#Bottle" " is your workload container, running unmodified in a controlled"
      buh_t   "  network environment."
      buh_tlt "  A " "nameplate" "${z_docs}#Nameplate" " ties a sentry and bottle together into a runnable unit."
      buh_e
      buh_tlt "  The " "sentry" "${z_docs}#Sentry" " enforces network policies via iptables and dnsmasq."
      buh_tlt "  The " "pentacle" "${z_docs}#Pentacle" " establishes the network namespace shared with the bottle."
      buh_e
      buh_tlt "  " "Charge" "${z_docs}#Charge" " starts the sentry/pentacle/bottle triad:"
      buh_tI  "    " "${RBZ_CRUCIBLE_CHARGE}" "${RBCC_onboarding_nameplate}"
      buh_e
      buh_t   "  Shell into the bottle and look around:"
      buh_tTc "    " "${RBZ_CRUCIBLE_RACK}" " ${RBCC_onboarding_nameplate}"
      buh_e
      buh_tlt "  When done, " "quench" "${z_docs}#Quench" " stops and cleans up:"
      buh_tI  "    " "${RBZ_CRUCIBLE_QUENCH}" "${RBCC_onboarding_nameplate}"

    else
      # ---- Unit 4: Local Experimentation ----
      buh_section "  Local Experimentation"
      buh_e
      buh_tlt "  " "Kludge" "${z_docs}#Kludge" " builds a vessel image locally for fast iteration — no registry"
      buh_t   "  push, no director credentials needed:"
      buh_tT  "    " "${RBZ_KLUDGE_VESSEL}"
      buh_e
      buh_t   "  After kludging, charge a nameplate to test your local build, then rack in"
      buh_t   "  and look around. Kludge is the retriever's experimentation tool — iterate"
      buh_t   "  on your local environment without Cloud Build."
    fi
  fi

  buh_e
  buh_tT "  Triage: " "${RBZ_ONBOARD_TRIAGE}"

}

rbho_director() {
  buc_doc_brief "Director walkthrough — build and publish vessel images"
  buc_doc_shown || return 0

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  # --- Extract config and probe ---
  local z_secrets_dir=""
  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi

  local z_du1=0
  local z_du2=0
  local z_du3=0
  local z_du4=0
  local z_du5=0
  local z_du6=0
  local z_du7=0
  zrbho_probe_director_units

  # --- Count progress ---
  local z_done=0
  if test "${z_du1}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_du2}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_du3}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_du4}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_du5}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_du6}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_du7}" = "1"; then z_done=$((z_done + 1)); fi
  local -r z_total=7

  # --- Header ---
  buh_section "Director Walkthrough"
  buh_e

  if test "${z_done}" = "${z_total}"; then
    # ============ REFERENCE MODE — all probes green ============
    buh_t  "  All steps complete. Re-run anytime to verify health."
    buh_e
    zrbho_po_status 1 "  Credential gate — director SA key installed"
    zrbho_po_status 1 "  Local build — kludge image present"
    zrbho_po_status 1 "  Depot foundation — base images enshrined"
    zrbho_po_status 1 "  Conjure — production build image present"
    zrbho_po_status 1 "  Bind — pinned upstream image present"
    zrbho_po_status 1 "  Graft — locally-built image pushed"
    zrbho_po_status 1 "  Full ark — all three modes compared"
  else
    # ============ WALKTHROUGH MODE — show frontier unit ============
    local -r z_frontier=$((z_done + 1))
    buh_t  "  Step ${z_frontier} of ${z_total}"
    buh_e

    if test "${z_du1}" = "0"; then
      # ---- Unit 1: Credential Gate ----
      buh_section "  Credential Gate"
      buh_e
      buh_tlt "  A " "depot" "${z_docs}#Depot" " is the facility where container images are built and stored."
      buh_tlt "  A " "director" "${z_docs}#Director" " is a role with build and publish access to a depot —"
      buh_t   "  you create container images and push them to the registry."
      buh_e
      buh_t   "  Where a retriever can only pull, a director can build, push, and manage"
      buh_t   "  artifacts. Your governor knighted this service account for build operations."
      buh_e
      buh_t   "  To access a depot, you need a service account key. Your governor creates"
      buh_t   "  one by running:"
      buh_tT  "    " "${RBZ_KNIGHT_DIRECTOR}"
      buh_e
      if test -n "${z_secrets_dir}"; then
        buh_t   "  Install the key file to:"
        buh_tc  "    " "${z_secrets_dir}/${RBCC_role_director}/${RBCC_rbra_file}"
      else
        buh_tW  "  " "Project not configured — .rbk/rbrr.env not found."
        buh_t   "  Run the payor walkthrough first, or ask your payor for the project files."
      fi
      buh_e
      buh_t   "  Once installed, re-run this walkthrough to continue."

    elif test "${z_du2}" = "0"; then
      # ---- Unit 2: Kludge — Local Build ----
      buh_section "  Kludge: Local Build"
      buh_e
      buh_tlt "  A " "vessel" "${z_docs}#Vessel" " is a specification for a container image — a Dockerfile,"
      buh_t   "  build context, and metadata defining what gets built."
      buh_e
      buh_tlt "  " "Kludge" "${z_docs}#Kludge" " builds a vessel image locally using Docker — no Cloud Build"
      buh_t   "  setup needed, no registry push. The fastest way to see a vessel come to life."
      buh_e
      buh_t   "  Build the sentry vessel locally:"
      buh_tT  "    " "${RBZ_KLUDGE_VESSEL}"
      buh_e
      buh_tlt "  After kludging, test by " "charging" "${z_docs}#Charge" " a crucible and shelling in:"
      buh_tI  "    " "${RBZ_CRUCIBLE_CHARGE}" "${RBCC_onboarding_nameplate}"
      buh_tTc "    " "${RBZ_CRUCIBLE_RACK}" " ${RBCC_onboarding_nameplate}"
      buh_e
      buh_t   "  Later units teach how to build this same vessel via Cloud Build for production,"
      buh_t   "  and how to push your local build to the registry."

    elif test "${z_du3}" = "0"; then
      # ---- Unit 3: Depot Foundation — Reliquary and Enshrine ----
      buh_section "  Depot Foundation: Reliquary and Enshrine"
      buh_e
      buh_t   "  Before Cloud Build can create production images, the depot needs two kinds"
      buh_t   "  of upstream images mirrored into your registry:"
      buh_e
      buh_tlt "  An " "ark" "${z_docs}#Ark" " is an immutable container image artifact in the registry,"
      buh_t   "  produced from a vessel."
      buh_e
      buh_t   "  Tool images (reliquary): gcloud, docker, syft, skopeo, binfmt — the"
      buh_t   "  tools that Cloud Build steps consume during a build."
      buh_e
      buh_tlt "  Base images (" "enshrine" "${z_docs}#Enshrine" "): the upstream images that vessels build FROM."
      buh_t   "  Mirrored into your depot's registry with content-addressed anchors."
      buh_e
      buh_t   "  Mirror tool images into the depot:"
      buh_tT  "    " "${RBZ_INSCRIBE_RELIQUARY}"
      buh_e
      buh_t   "  Enshrine base images for the sentry vessel:"
      buh_tT  "    " "${RBZ_ENSHRINE_VESSEL}"
      buh_e
      buh_t   "  Reliquary provides the tools; enshrine provides the foundations."
      buh_tltlt "  Both must be in place before " "conjure" "${z_docs}#Conjure" " or " "bind" "${z_docs}#Bind" "."
      buh_e
      buh_t   "  After completing both, proceed to conjure your first production build."
      buh_tlt "  The probe for this step turns green when a conjure " "hallmark" "${z_docs}#Hallmark" ""
      buh_tlt "  is " "summoned" "${z_docs}#Summon" " locally (next step)."

    elif test "${z_du4}" = "0"; then
      # ---- Unit 4: Conjure — Production Build ----
      # (Frontier only if du3 green but du4 red — rare due to shared probe,
      #  but shown in reference mode as separate unit)
      buh_section "  Conjure: Production Build"
      buh_e
      buh_tlt "  A " "hallmark" "${z_docs}#Hallmark" " is a specific build instance of a vessel, identified by"
      buh_t   "  timestamp."
      buh_e
      buh_tlt "  " "Ordain" "${z_docs}#Ordain" " creates a hallmark with full attestation — the production build"
      buh_t   "  command."
      buh_tlt "  " "Conjure" "${z_docs}#Conjure" " is the ordain mode where Cloud Build creates the image from"
      buh_t   "  source. Every conjure produces a three-part ark: image, about (SBOM + build"
      buh_t   "  info), and vouch (DSSE signature verification)."
      buh_e
      buh_t   "  This is the same vessel you kludged locally — now Cloud Build creates it"
      buh_t   "  with full SLSA provenance:"
      buh_tT  "    " "${RBZ_ORDAIN_HALLMARK}"
      buh_e
      buh_tlt "  Verify with " "vouch" "${z_docs}#Vouch" " (cryptographic attestation) and"
      buh_tlt "  " "tally" "${z_docs}#Tally" " (registry inventory):"
      buh_tT  "    " "${RBZ_VOUCH_HALLMARKS}"
      buh_tT  "    " "${RBZ_TALLY_HALLMARKS}"
      buh_e
      buh_tlt "  Then " "summon" "${z_docs}#Summon" " the hallmark locally to confirm the full pipeline:"
      buh_tT  "    " "${RBZ_SUMMON_HALLMARK}"

    elif test "${z_du5}" = "0"; then
      # ---- Unit 5: Bind — Pin Upstream Image ----
      buh_section "  Bind: Pin Upstream Image"
      buh_e
      buh_tlt "  " "Bind" "${z_docs}#Bind" " mirrors a pinned upstream image into your depot. No Dockerfile,"
      buh_t   "  no build — just a content-addressed copy."
      buh_e
      buh_t   "  PlantUML is useful for rendering architecture diagrams, but its Docker Hub"
      buh_tlt "  image could send your private diagrams anywhere. Bind pins it by " "digest" "${z_docs}#Bind" " —"
      buh_tlt "  no silent updates. Then " "charge" "${z_docs}#Charge" " it as a bottle: the sentry blocks"
      buh_t   "  all egress. You get the tool without the risk."
      buh_e
      buh_t   "  Ordain the plantuml vessel in bind mode:"
      buh_tT  "    " "${RBZ_ORDAIN_HALLMARK}"
      buh_e
      buh_t   "  The upstream image is pulled by digest, pushed to GAR, about metadata"
      buh_tlt "  generated, and " "vouch" "${z_docs}#Vouch" " records a digest-pin verdict. No SLSA provenance —"
      buh_t   "  the image was not built here, but it is pinned and bottled."
      buh_e
      buh_tlt "  Verify and " "summon" "${z_docs}#Summon" ":"
      buh_tT  "    " "${RBZ_VOUCH_HALLMARKS}"
      buh_tT  "    " "${RBZ_SUMMON_HALLMARK}"

    elif test "${z_du6}" = "0"; then
      # ---- Unit 6: Graft — Push Local to Registry ----
      buh_section "  Graft: Push Local to Registry"
      buh_e
      buh_tlt "  " "Graft" "${z_docs}#Graft" " pushes a locally-built image to GAR. The image push is local"
      buh_t   "  (docker push), but about and vouch still run in Cloud Build."
      buh_e
      buh_t   "  You kludged the sentry in step 2 and conjured it in step 4. Now push your"
      buh_t   "  local build to the registry via graft:"
      buh_tT  "    " "${RBZ_ORDAIN_HALLMARK}"
      buh_e
      buh_t   "  One combined Cloud Build job runs about + vouch. The vouch verdict is"
      buh_t   "  GRAFTED — meaning this image was locally built, trust it at your own"
      buh_t   "  assessment."
      buh_e
      buh_t   "  The development cycle: kludge, test, graft when satisfied."

    else
      # ---- Unit 7: Full Ark — About and Vouch Pipeline ----
      buh_section "  The Full Ark: About and Vouch Pipeline"
      buh_e
      buh_t   "  Every hallmark — regardless of mode — produces the same three-part"
      buh_t   "  structure: image, about, and vouch. About contains the SBOM and"
      buh_t   "  build_info.json. Vouch contains the mode-specific verification."
      buh_e
      buh_tlt "  " "Plumb" "${z_docs}#Plumb" " lets you inspect an artifact's provenance — SBOM, build info,"
      buh_t   "  and vouch chain:"
      buh_tT  "    " "${RBZ_PLUMB_FULL}"
      buh_e
      buh_t   "  Run plumb against each mode's hallmark and compare:"
      buh_tlt "    - " "Conjure" "${z_docs}#Conjure" " (sentry): DSSE vouch, SLSA provenance"
      buh_tlt "    - " "Bind" "${z_docs}#Bind" " (plantuml): digest-pin vouch, no provenance"
      buh_tlt "    - " "Graft" "${z_docs}#Graft" " (sentry): GRAFTED vouch, no provenance chain"
      buh_e
      buh_t   "  The tally command shows the full registry health view — the director's"
      buh_t   "  operational dashboard:"
      buh_tT  "    " "${RBZ_TALLY_HALLMARKS}"
    fi
  fi

  buh_e
  buh_tT "  Triage: " "${RBZ_ONBOARD_TRIAGE}"

}

rbho_governor() {
  buc_doc_brief "Governor walkthrough — manage service accounts and access"
  buc_doc_shown || return 0

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  # --- Extract config and probe ---
  local z_secrets_dir=""
  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi

  local z_gu1 z_gu2 z_gu3
  zrbho_probe_governor_units

  # --- Count progress ---
  local z_done=0
  if test "${z_gu1}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_gu2}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_gu3}" = "1"; then z_done=$((z_done + 1)); fi
  local -r z_total=3

  # --- Header ---
  buh_section "Governor Walkthrough"
  buh_e

  if test "${z_done}" = "${z_total}"; then
    # ============ REFERENCE MODE — all probes green ============
    buh_t  "  All steps complete. Re-run anytime to verify health."
    buh_e
    zrbho_po_status 1 "  Project access — governor credentials installed"
    zrbho_po_status 1 "  Service accounts — retriever and director SAs provisioned"
    zrbho_po_status 1 "  Verification — downstream roles can access the depot"
  else
    # ============ WALKTHROUGH MODE — show frontier unit ============
    local -r z_frontier=$((z_done + 1))
    buh_t  "  Step ${z_frontier} of ${z_total}"
    buh_e

    if test "${z_gu1}" = "0"; then
      # ---- Unit 1: Project Access ----
      buh_section "  Project Access"
      buh_e
      buh_tlt "  A " "depot" "${z_docs}#Depot" " is the facility where container images are built and stored."
      buh_tlt "  A " "governor" "${z_docs}#Governor" " administers a depot — creating service accounts and"
      buh_t   "  managing access for those who build and run container images."
      buh_e
      buh_t   "  The governor works within a depot that the payor created. If no depot exists"
      buh_t   "  yet, that is a payor responsibility:"
      buh_tT  "    " "${RBZ_ONBOARD_PAYOR}"
      buh_e
      buh_t   "  To administer a depot, you need a governor service account key. Your payor"
      buh_t   "  creates one by running:"
      buh_tT  "    " "${RBZ_MANTLE_GOVERNOR}"
      buh_e
      if test -n "${z_secrets_dir}"; then
        buh_t   "  Install the key file to:"
        buh_tc  "    " "${z_secrets_dir}/${RBCC_role_governor}/${RBCC_rbra_file}"
      else
        buh_tW  "  " "Project not configured — .rbk/rbrr.env not found."
        buh_t   "  Run the payor walkthrough first, or ask your payor for the project files."
      fi
      buh_e
      buh_t   "  Once installed, re-run this walkthrough to continue."

    elif test "${z_gu2}" = "0"; then
      # ---- Unit 2: Service Account Lifecycle ----
      buh_section "  Service Account Lifecycle"
      buh_e
      buh_t   "  The governor provisions access for two downstream roles:"
      buh_e
      buh_tlt "  A " "retriever" "${z_docs}#Retriever" " has read access to the depot — they pull and run"
      buh_t   "  container images that others have built."
      buh_tlt "  A " "director" "${z_docs}#Director" " has build and publish access — they create container"
      buh_t   "  images and push them to the registry."
      buh_e
      buh_tlt "  " "Charter" "${z_docs}#Charter" " creates a retriever service account with read access:"
      buh_tT  "    " "${RBZ_CHARTER_RETRIEVER}"
      buh_e
      buh_tlt "  " "Knight" "${z_docs}#Knight" " creates a director service account with build access:"
      buh_tT  "    " "${RBZ_KNIGHT_DIRECTOR}"
      buh_e
      buh_t   "  Each command creates the service account and applies the IAM grants it needs."
      buh_t   "  The output is an RBRA key file — hand it to the retriever or director user."
      buh_e
      buh_t   "  List issued service accounts:"
      buh_tT  "    " "${RBZ_LIST_SERVICE_ACCOUNTS}"
      buh_e
      buh_t   "  Install both credentials locally to advance this walkthrough."
      if test -n "${z_secrets_dir}"; then
        buh_tc  "    " "${z_secrets_dir}/${RBCC_role_retriever}/${RBCC_rbra_file}"
        buh_tc  "    " "${z_secrets_dir}/${RBCC_role_director}/${RBCC_rbra_file}"
      fi

    else
      # ---- Unit 3: Verification ----
      buh_section "  Verification"
      buh_e
      buh_t   "  The service accounts you created include IAM grants — each SA gets exactly"
      buh_t   "  the permissions its role requires, no more. Retriever gets read access."
      buh_t   "  Director gets read, write, and build trigger access."
      buh_e
      buh_t   "  Verify the complete chain works by pulling an artifact with the retriever"
      buh_t   "  credentials. If the retriever can access the depot, your grants are correct."
      buh_e
      buh_t   "  Run the retriever walkthrough to summon a hallmark:"
      buh_tT  "    " "${RBZ_ONBOARD_RETRIEVER}"
      buh_e
      buh_t   "  This probe turns green when a GAR image from your depot exists locally —"
      buh_t   "  proving the retriever SA you chartered can actually access the registry."
    fi
  fi

  buh_e
  buh_tT "  Triage: " "${RBZ_ONBOARD_TRIAGE}"

}

rbho_payor() {
  buc_doc_brief "Payor walkthrough — GCP project, billing, and OAuth setup"
  buc_doc_shown || return 0

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"

  # --- Extract config and probe ---
  local z_secrets_dir=""
  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi

  local z_pu1=0
  local z_pu2=0
  local z_pu3=0
  local z_pu4=0
  zrbho_probe_payor_units

  # --- Count progress ---
  local z_done=0
  if test "${z_pu1}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_pu2}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_pu3}" = "1"; then z_done=$((z_done + 1)); fi
  if test "${z_pu4}" = "1"; then z_done=$((z_done + 1)); fi
  local -r z_total=4

  # --- Header ---
  buh_section "Payor Walkthrough"
  buh_e

  if test "${z_done}" = "${z_total}"; then
    # ============ REFERENCE MODE — all probes green ============
    buh_t  "  All steps complete. Re-run anytime to verify health."
    buh_e
    zrbho_po_status 1 "  OAuth bootstrap — credentials installed"
    zrbho_po_status 1 "  Project setup — GCP project configured"
    zrbho_po_status 1 "  Depot provisioning — infrastructure levied"
    zrbho_po_status 1 "  Governor handoff — governor SA created"
  else
    # ============ WALKTHROUGH MODE — show frontier unit ============
    local -r z_frontier=$((z_done + 1))
    buh_t  "  Step ${z_frontier} of ${z_total}"
    buh_e

    if test "${z_pu1}" = "0"; then
      # ---- Unit 1: OAuth Bootstrap ----
      buh_section "  OAuth Bootstrap"
      buh_e
      buh_tlt "  The " "payor" "${z_docs}#Payor" " owns the GCP project and funds it. Unlike other roles"
      buh_t   "  that use service account keys, the payor authenticates via OAuth — representing"
      buh_t   "  the human project owner."
      buh_e
      buh_t   "  To get started, download an OAuth client secret JSON file from your GCP"
      buh_t   "  project's API credentials page, then run:"
      buh_tTc "    " "${RBZ_PAYOR_INSTALL}" " \${HOME}/Downloads/client_secret_*.json"
      buh_e
      buh_t   "  This walks you through the OAuth authorization flow and stores the credential"
      buh_t   "  securely. If you have an existing credential that has expired:"
      buh_tT  "    " "${RBZ_PAYOR_REFRESH}"
      buh_e
      buh_t   "  Once installed, re-run this walkthrough to continue."

    elif test "${z_pu2}" = "0"; then
      # ---- Unit 2: Project Setup ----
      buh_section "  Project Setup"
      buh_e
      buh_t   "  A funded GCP project is required before any infrastructure can be provisioned."
      buh_t   "  The project must have billing enabled and the OAuth consent screen configured."
      buh_e
      buh_t   "  Run the guided setup:"
      buh_tT  "    " "${RBZ_PAYOR_ESTABLISH}"
      buh_e
      buh_t   "  This will guide you through project creation, billing enablement, and OAuth"
      buh_t   "  consent screen configuration. The project ID is recorded in regime files"
      buh_t   "  and becomes the identity for all depot operations."
      buh_e
      buh_t   "  Once complete, re-run this walkthrough to continue."

    elif test "${z_pu3}" = "0"; then
      # ---- Unit 3: Depot Provisioning ----
      buh_section "  Depot Provisioning"
      buh_e
      buh_tlt "  A " "depot" "${z_docs}#Depot" " is the facility where container images are built and stored"
      buh_t   "  — a GCP project with a registry, storage bucket, and build infrastructure."
      buh_e
      buh_tlt "  To " "levy" "${z_docs}#Levy" " a depot is to provision this infrastructure. Run:"
      buh_tT  "    " "${RBZ_LEVY_DEPOT}"
      buh_e
      buh_t   "  This enables APIs, creates the Artifact Registry repository and Cloud Storage"
      buh_t   "  bucket, and configures Cloud Build. The depot is now ready for use."
      buh_e
      buh_t   "  List your depots to verify:"
      buh_tT  "    " "${RBZ_LIST_DEPOT}"
      buh_e
      buh_t   "  Once provisioned, re-run this walkthrough to continue."

    else
      # ---- Unit 4: Governor Handoff ----
      buh_section "  Governor Handoff"
      buh_e
      buh_tlt "  A " "governor" "${z_docs}#Governor" " administers a depot — creating service accounts and"
      buh_t   "  managing access for those who build and run container images."
      buh_e
      buh_t   "  The payor funds the infrastructure; the governor operates it. After this"
      buh_t   "  handoff, the governor can charter retrievers and knight directors"
      buh_t   "  independently. Run:"
      buh_tT  "    " "${RBZ_MANTLE_GOVERNOR}"
      buh_e
      buh_t   "  This creates the governor service account with administrative permissions"
      buh_t   "  over the depot. Hand the resulting key file to the person who will"
      buh_t   "  administer this depot."
      buh_e
      buh_t   "  The payor's job for this depot is done unless billing or project-level"
      buh_t   "  changes are needed."
    fi
  fi

  buh_e
  buh_tT "  Triage: " "${RBZ_ONBOARD_TRIAGE}"

}

######################################################################
# Handbook restart — foundation
#
# First deployed artifacts of the Frame 4-refined handbook restart (see
# ₣A6 paddock). These coexist with the role-track sequences above until
# the restart has full coverage, then replace them.
#
# Organizing axis: learner intent + repo state, NOT role/authorization.
# These sequences borrow idioms from the rbho_* family above but NOT
# the role-decomposition shape — that's precisely what A6 is curing.

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
  buh_tT  "      Run: " "${RBZ_ONBOARD_CRASH_COURSE}"
  buh_tltlt "      Universal prerequisite. " "Tabtargets" "${z_docs}#Tabtarget" ", " "Regimes" "${z_docs}#Regime" ","
  buh_tlt "      " "Station" "${z_docs}#Station" " setup, validation, logs. Local-only, no cloud."
  buh_e
  buh_tlt "    User installs " "Retriever" "${z_docs}#Retriever" " credentials"
  buh_tlt "      If joining an existing project: place your " "RBRA" "${z_docs}#RBRA" " credential file,"
  buh_t   "      verify via access probe, confirm you can pull images."
  buh_e
  buh_tlt "    User installs " "Director" "${z_docs}#Director" " credentials"
  buh_tlt "      Place your " "RBRA" "${z_docs}#RBRA" " credential file, verify via access probe, confirm"
  buh_t   "      you can build and publish images."
  buh_e
  buh_tlt "    User sets up " "Crucible" "${z_docs}#Crucible" " with local builds"
  buh_tltlt "      Local sandbox with " "Kludged" "${z_docs}#Kludge" " " "Hallmarks" "${z_docs}#Hallmark" ". The tightest feedback loop"
  buh_t   "      in the system. No cloud."
  buh_e

  # --- Create Payor and Depot ---
  buh_section "Create Payor and Depot"
  buh_e
  buh_tlt "  A " "Depot" "${z_docs}#Depot" " is the facility where the team's container images are"
  buh_t   "  built and stored — the ground truth other tracks rest on."
  buh_e
  buh_tlt "    User creates " "Payor" "${z_docs}#Payor" " account"
  buh_t   "      ~15 minutes: create a GCP account, attach a credit card, configure"
  buh_t   "      OAuth consent screen, initial RBRR population. Required before any"
  buh_t   "      cloud operations."
  buh_e
  buh_tltltlt "    " "Payor" "${z_docs}#Payor" " creates the " "Depot" "${z_docs}#Depot" " and its " "Governor" "${z_docs}#Governor" ""
  buh_t   "      GCP project provisioning, billing linkage, service account creation."
  buh_e
  buh_tltltlt "    " "Governor" "${z_docs}#Governor" " administers " "Directors" "${z_docs}#Director" " and " "Retrievers" "${z_docs}#Retriever" ""
  buh_t   "      Create service accounts, issue credentials, distribute securely."
  buh_e

  # --- Director subtracks ---
  buh_section "Director subtracks"
  buh_e
  buh_tltltlt "    Your First " "Ordination" "${z_docs}#Ordain" "    — one " "Vessel" "${z_docs}#Vessel" ", one " "Ordain" "${z_docs}#Ordain" " mode"
  buh_tlt     "    " "Enshrine" "${z_docs}#Enshrine" " the Ancestors   — base image enshrinement"
  buh_t       "    Inscribe the Reliquary   — reliquary management"
  buh_t       "    Tethered Cloud Builds    — cloud build from workstation"
  buh_tlt     "    Airgap Your Builds       — most restrictive " "Ordain" "${z_docs}#Ordain" " mode"
  buh_tlt     "    Building for " "Crucibles" "${z_docs}#Crucible" "   — build crucible test targets"
  buh_e

  # --- Evaluation ---
  buh_section "Evaluation"
  buh_e
  buh_t   "    Assay the Realm"
  buh_tltlt "      Enterprise evaluator — tour of " "Ordain" "${z_docs}#Ordain" " modes, " "Enshrinement" "${z_docs}#Enshrine" ","
  buh_t   "      reliquaries, airgap. Breadth over depth."
  buh_e

  # --- Reference ---
  buh_section "Reference"
  buh_e
  buh_tT  "    Realm Dashboard: " "${RBZ_ONBOARD_REFERENCE}"
  buh_t   "      Current health of all roles and units, reference format."
  buh_e

  # --- Find your role / legacy fallback ---
  buh_section "Find your role"
  buh_e
  buh_t   "  If you don't know which path to pick: run the Crash Course first."
  buh_t   "  It explains the apparatus and points you forward."
  buh_e
  buh_t   "  A legacy role-organized triage is preserved during this heat as a"
  buh_t   "  fallback for anyone who already knows their role:"
  buh_tT  "    " "${RBZ_ONBOARD_TRIAGE}"
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

  local z_station_present=0
  if test -n "${BURD_STATION_FILE:-}" && test -f "${BURD_STATION_FILE}"; then
    z_station_present=1
  fi

  # --- Header ---
  buh_section "Recipe Bottle — Configure your Repo's Environment"
  buh_e
  buh_tlt "  " "Recipe Bottle" "${z_docs}" " builds container images with supply-chain provenance"
  buh_t   "  and runs untrusted containers behind enforced network isolation."
  buh_e
  buh_t   "  This handbook walks you through the configuration files that control"
  buh_t   "  the project's tooling. Each step asks you to run a command and read"
  buh_t   "  what it tells you."
  buh_e

  # --- Step 1: What you ran to get here ---
  buh_section "  Step 1 — What you ran to get here"
  buh_e
  buh_tlt "  The command you just ran is a " "Tabtarget" "${z_docs}#Tabtarget" " — a launcher script"
  buh_t   "  in the tt/ directory. Its filename encodes what it does:"
  buh_e
  buh_t   "      tt/rbw-Occ.OnboardingCrashCourse.sh"
  buh_t   "         \`rbw-Occ\` is the colophon (routes to the right handler)"
  buh_t   "         \`OnboardingCrashCourse\` is the frontispiece (what it does)"
  buh_e
  buh_t   "  Tab completion narrows by prefix: type \`tt/rbw-<TAB>\` to see every"
  buh_tlt "  " "Recipe Bottle" "${z_docs}" " command."
  buh_e

  # --- Step 2: View the project config (BURC) ---
  buh_section "  Step 2 — View the project config"
  buh_e
  buh_tlt "  A " "Regime" "${z_docs}#Regime" " is a configuration file with a schema, a renderer,"
  buh_t   "  and a validator. Run the renderer for the project config regime:"
  buh_e
  buh_tT  "      " "${BUWZ_RC_RENDER}"
  buh_e
  buh_tlt "  " "BURC" "${z_docs}#BURC" " is checked into git — shared project settings that every"
  buh_t   "  clone gets. It tells the launcher where to find tools and where"
  buh_t   "  to look for your personal station file."
  buh_e

  # --- Step 3: View your station (BURS) ---
  buh_section "  Step 3 — View your personal station"
  buh_e
  buh_tlt "  " "BURS" "${z_docs}#BURS" " is your per-developer station file: local, gitignored,"
  buh_t   "  holds things that vary per machine. Most tabtargets require it —"
  buh_t   "  the launcher will tell you what's missing if it's absent. Run the renderer:"
  buh_e
  buh_tT  "      " "${BUWZ_RS_RENDER}"
  buh_e
  buh_t   "  The repo-vs-personal split is deliberate: BURC travels with the code;"
  buh_t   "  BURS stays on your machine."
  buh_e

  # --- Step 4: Validate your station ---
  buh_section "  Step 4 — Validate your station"
  buh_e
  buh_t   "  Every regime has a validate tabtarget that checks the file against"
  buh_t   "  its schema. Run it:"
  buh_e
  buh_tT  "      " "${BUWZ_RS_VALIDATE}"
  buh_e
  buh_t   "  NOTE: This may fail if your station file is missing fields beyond"
  buh_t   "  the minimum the launcher required. That is expected — read the"
  buh_t   "  error, it names the field and tells you what to fill in."
  buh_e
  if test "${z_station_present}" = "1"; then
    zrbho_po_status 1 "  Station file present at ${BURD_STATION_FILE}"
  else
    zrbho_po_status 0 "  Station file not found"
  fi
  buh_e

  # --- Step 5: Validate the repo regime ---
  buh_section "  Step 5 — Validate the repo regime"
  buh_e
  buh_tlt "  The repository regime (" "RBRR" "${z_docs}#RBRR" ") holds your team's " "Depot" "${z_docs}#Depot" ""
  buh_t   "  identity — the GCP project where container images are built and stored."
  buh_t   "  Run the validator:"
  buh_e
  buh_tT  "      " "${RBZ_VALIDATE_REPO}"
  buh_e
  buh_t   "  On a bare fork, RBRR fields are blank and validation will fail."
  buh_t   "  On a team repo, they are already populated and validation passes."
  buh_t   "  Either way, read the output — it tells you exactly what state you're in."
  buh_e
  if test "${z_rbrr_populated}" = "1"; then
    zrbho_po_status 1 "  RBRR populated — depot project: ${z_rbrr_project}"
  else
    zrbho_po_status 0 "  RBRR not populated — depot identity fields are blank"
  fi
  buh_e

  # --- Step 6: Check your logs ---
  buh_section "  Step 6 — Check your logs"
  buh_e
  buh_t   "  When you ran the validator, it printed log file paths at the top"
  buh_t   "  of its output. Open one — you'll find the full validation results"
  buh_t   "  captured there. Logs land in the directory you set as \`BURS_LOG_DIR\`"
  buh_t   "  in your station file."
  buh_e
  buh_t   "  State-changing commands and validators log their output. Handbook"
  buh_t   "  display commands (like this one) do not — teaching output is"
  buh_t   "  ephemeral by design."
  buh_e

  # --- Step 7: The pattern ---
  buh_section "  Step 7 — The pattern"
  buh_e
  buh_tlt "  Every " "Regime" "${z_docs}#Regime" " follows the same tabtarget pattern:"
  buh_e
  buh_t   "      tt/{W}-r{L}r.Render{R}Regime.sh   — show current values"
  buh_t   "      tt/{W}-r{L}v.Validate{R}Regime.sh — check against rules"
  buh_e
  buh_t   "  \`{W}\` is the workbench (\`buw\` for BUK, \`rbw\` for Recipe Bottle)."
  buh_t   "  \`{L}\` is the regime letter (\`c\` for BURC, \`s\` for BURS, \`r\` for RBRR)."
  buh_t   "  Learn the pattern — you can find any regime's tools from two letters."
  buh_e

  # --- Step 8: Next steps ---
  buh_section "  Step 8 — Next steps"
  buh_e
  buh_t  "  Your repo environment is configured. The tools work, errors explain"
  buh_t  "  themselves, and logs land where you told them to."
  buh_e
  buh_t  "  Return to the start menu for what to do next:"
  buh_tT "    " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

# eof
