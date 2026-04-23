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
# Recipe Bottle Handbook Onboarding - Director Bind Mode
#
# Teaches bind mode using rbev-bottle-plantuml: pin an upstream image
# by digest into GAR, no Dockerfile, no build context, no SLSA. The
# pluml Crucible deliberately mixes a kludged Sentry with a bound
# Bottle — two ordain modes cohabiting in one Crucible is the
# expected shape, not a defect.

set -euo pipefail

test -z "${ZRBHODB_SOURCED:-}" || return 0
ZRBHODB_SOURCED=1

rbho_director_bind() {
  zrbho_sentinel

  buc_doc_brief "${RBHO_TRACK_BIND} — pin upstream image by digest, mode-mixture pluml Crucible"
  buc_doc_shown || return 0

  local -r z_moniker="pluml"
  local -r z_vessel="rbev-bottle-plantuml"
  local -r z_pluml_rbrn="${RBBC_dot_dir}/${z_moniker}/${RBCC_rbrn_file}"

  local z_has_director=0
  local z_secrets_dir=""
  if test -f "${RBBC_rbrr_file}"; then
    z_secrets_dir=$(zrbho_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
  fi
  if test -n "${z_secrets_dir}" && \
     test -f "${z_secrets_dir}/${RBCC_role_director}/${RBCC_rbra_file}"; then
    z_has_director=1
  fi

  local z_has_depot=0
  if test -f "${RBBC_rbrr_file}"; then
    local z_line=""
    while IFS= read -r z_line; do
      case "${z_line}" in RBRR_DEPOT_PROJECT_ID=?*) z_has_depot=1; break ;; esac
    done < "${RBBC_rbrr_file}"
  fi

  local z_sentry_ready=0
  if test -f "${z_pluml_rbrn}"; then
    local z_sentry_hallmark=""
    z_sentry_hallmark=$(zrbho_po_extract_capture "${z_pluml_rbrn}" "RBRN_SENTRY_HALLMARK") || z_sentry_hallmark=""
    case "${z_sentry_hallmark}" in
      ""|PENDING-*) ;;
      *) z_sentry_ready=1 ;;
    esac
  fi

  local z_bottle_bound=0
  if test -f "${z_pluml_rbrn}"; then
    local z_bottle_hallmark=""
    z_bottle_hallmark=$(zrbho_po_extract_capture "${z_pluml_rbrn}" "RBRN_BOTTLE_HALLMARK") || z_bottle_hallmark=""
    case "${z_bottle_hallmark}" in
      ""|PENDING-*) ;;
      *) z_bottle_bound=1 ;;
    esac
  fi

  buyy_link_yawp "${RBRR_PUBLIC_DOCS_URL}" "Vessel"    "${z_vessel}";   local -r z_lk_vessel="${z_buym_yelp}"
  buyy_link_yawp "${RBRR_PUBLIC_DOCS_URL}" "Nameplate" "${z_moniker}";  local -r z_lk_pluml="${z_buym_yelp}"

  buh_section "${RBHO_TRACK_BIND} — Pin Upstream by Digest"
  buh_e
  buh_line "${RBHO_TRACK_FIRST_BUILD} taught ${RBYC_CONJURE} — Cloud Build constructs"
  buh_line "the ${RBYC_VESSEL} image from the project's Dockerfile, and SLSA"
  buh_line "${RBYC_PROVENANCE} attests the build chain. ${RBYC_BIND} is the simplest"
  buh_line "${RBYC_ORDAIN} mode: no Dockerfile, no build context, no Cloud Build."
  buh_line "You pin an upstream image by digest into your ${RBYC_DEPOT}, and the"
  buh_line "${RBYC_DEPOT} mirrors exactly what upstream published."
  buh_e
  buh_line "This track binds ${z_lk_vessel} — the PlantUML server image. PlantUML"
  buh_line "renders diagrams, but its Docker Hub source could phone home if run"
  buh_line "naked. ${RBYC_BIND} pins it by content hash; the ${RBYC_SENTRY} blocks all"
  buh_line "egress at runtime. You get the tool without the risk."
  buh_e

  buh_line "Prerequisites:"
  buh_e
  if test "${z_has_director}" = "1"; then
    buh_line "${RBYC_PROBE_YES}${RBYC_DIRECTOR} credential installed"
  else
    buh_line "${RBYC_PROBE_NO}${RBYC_DIRECTOR} credential missing — run:"
    buh_tt "      " "${RBZ_ONBOARD_CRED_DIRECTOR}"
  fi
  if test "${z_has_depot}" = "1"; then
    buh_line "${RBYC_PROBE_YES}${RBYC_DEPOT} configured (RBRR_DEPOT_PROJECT_ID populated)"
  else
    buh_line "${RBYC_PROBE_NO}${RBYC_DEPOT} not configured — the ${RBYC_PAYOR} must establish the ${RBYC_DEPOT}:"
    buh_tt "      " "${RBZ_ONBOARD_PAYOR_HB}"
  fi
  buh_e

  if test "${z_has_director}" = "0" || test "${z_has_depot}" = "0"; then
    buh_error "Complete the prerequisites above before continuing."
    buh_e
    buh_tt "Return to start: " "${RBZ_ONBOARD_START_HERE}"
    buh_e
    return 0
  fi

  buh_section "Mode mixture — name it explicitly"
  buh_e
  buh_line "The ${z_lk_pluml} ${RBYC_CRUCIBLE} in this track pairs a ${RBYC_KLUDGE_D}"
  buh_line "(or ${RBYC_CONJURED}) ${RBYC_SENTRY} with a ${RBYC_BIND}-pinned PlantUML ${RBYC_BOTTLE}."
  buh_line "Two different ${RBYC_ORDAIN} modes cohabiting in one ${RBYC_CRUCIBLE} is the"
  buh_line "expected shape, not a defect — each ${RBYC_HALLMARK} carries its own"
  buh_line "${RBYC_VOUCH} verdict, and a ${RBYC_NAMEPLATE} can pin ${RBYC_HALLMARKS} of"
  buh_line "different modes without conflict."
  buh_e
  buh_line "The ${RBYC_SENTRY} enforces network policy regardless of how it was"
  buh_line "built; the policy is what's load-bearing, not the build mode behind"
  buh_line "it. The bind lesson is about what happens to the ${RBYC_BOTTLE} image."
  buh_e

  buh_step_style "Step " " — "

  buh_step1 "Ready a ${RBYC_SENTRY} for the ${z_lk_pluml} ${RBYC_CRUCIBLE}"
  buh_e
  buh_line "Before you can ${RBYC_CHARGE} ${z_lk_pluml}, the ${RBYC_NAMEPLATE} needs a"
  buh_line "${RBYC_SENTRY} ${RBYC_HALLMARK}. Two paths reach the same outcome:"
  buh_e
  buh_line "  Primary path — ${RBYC_KLUDGE} the ${RBYC_SENTRY} locally. Same"
  buh_line "  pattern you met in ${RBHO_TRACK_FIRST_CRUCIBLE}: the host builds the"
  buh_line "  image and writes the resulting ${RBYC_HALLMARK} into the ${z_lk_pluml}"
  buh_line "  ${RBYC_NAMEPLATE} automatically:"
  buh_e
  buh_tt "   " "${RBZ_CRUCIBLE_KLUDGE_SENTRY}" "" " ${z_moniker}"
  buh_e
  buh_line "  Alternative — if you completed ${RBHO_TRACK_FIRST_BUILD}, the"
  buh_line "  ${RBYC_CONJURED} ${RBYC_SENTRY} ${RBYC_HALLMARK} is already in your ${RBYC_DEPOT}."
  buh_line "  Read it and write it into ${z_lk_pluml}'s ${RBYC_RBRN}:"
  buh_e
  buyy_cmd_yawp "${z_pluml_rbrn}"; local -r z_lk_pluml_file="${z_buym_yelp}"
  buh_code "   RBRN_SENTRY_HALLMARK=<your-conjured-sentry-hallmark>"
  buh_e
  buh_line "  in ${z_lk_pluml_file}, then commit. The ${RBYC_SENTRY} ${RBYC_HALLMARK} can come"
  buh_line "  from either build path — a ${RBYC_KLUDGE_D} ${RBYC_SENTRY} is fine for"
  buh_line "  learning ${RBYC_BIND}; a ${RBYC_CONJURED} ${RBYC_SENTRY} is fine too. The bind"
  buh_line "  lesson is about the ${RBYC_BOTTLE}, not the ${RBYC_SENTRY}."
  buh_e
  if test "${z_sentry_ready}" = "1"; then
    buh_line "${RBYC_PROBE_YES}${z_lk_pluml} ${RBYC_SENTRY} ${RBYC_HALLMARK} installed"
  else
    buh_line "${RBYC_PROBE_NO}${z_lk_pluml} ${RBYC_SENTRY} ${RBYC_HALLMARK} not set — pick a path above"
  fi
  buh_e

  buh_step1 "${RBYC_BIND} the PlantUML ${RBYC_BOTTLE}"
  buh_e
  buh_line "${RBYC_BIND} mode mirrors an upstream image into your ${RBYC_DEPOT} by"
  buh_line "content hash. The ${RBYC_VESSEL} ${RBYC_RBRV} declares the upstream digest:"
  buh_e
  buh_code "   RBRV_VESSEL_MODE=bind"
  buh_code "   RBRV_BIND_IMAGE=docker.io/plantuml/plantuml-server@sha256:cd3d67a..."
  buh_e
  buh_line "${RBYC_ORDAIN} reads the mode from the ${RBYC_RBRV} and routes accordingly."
  buh_line "For ${RBYC_BIND}, that means: pull the digest-pinned image from upstream,"
  buh_line "push it to your ${RBYC_DEPOT} under a ${RBYC_HALLMARK} tag. No Dockerfile."
  buh_line "No ${RBYC_POUCH}. No Cloud Build. The image in your ${RBYC_DEPOT} is"
  buh_line "byte-identical to what upstream published at that digest."
  buh_e
  buh_line "Three ${RBYC_ORDAIN} modes, three trust contracts:"
  buh_e
  buh_line "  ${RBYC_CONJURE}  Cloud Build constructs the image from project Dockerfile."
  buh_line "            Trust the build chain (SLSA ${RBYC_PROVENANCE})."
  buh_line "  ${RBYC_BIND}     Mirror upstream image by digest. Trust the upstream"
  buh_line "            publisher; the digest pins what you got."
  buh_line "  ${RBYC_GRAFT}    Push a locally-built image. Trust the local machine."
  buh_e
  buh_line "${RBYC_ORDAIN} ${z_lk_vessel}:"
  buh_e
  buh_tt "   " "${RBZ_ORDAIN_HALLMARK}" "" " ${z_vessel}"
  buh_e
  buh_line "Wall-clock: seconds, not minutes. There is nothing to build — only"
  buh_line "a digest-addressed image to mirror."
  buh_e

  buh_step1 "Inspect the ${RBYC_VOUCH} verdict"
  buh_e
  buh_line "${RBYC_VOUCH} is mode-aware. For ${RBYC_CONJURED} ${RBYC_HALLMARKS} it verifies"
  buh_line "the SLSA ${RBYC_PROVENANCE} chain. For ${RBYC_BIND}, the verdict is honest:"
  buh_line "there is no SLSA chain because Cloud Build never ran. The ${RBYC_VOUCH}"
  buh_line "verdict reads digest-pin — what's attested is that the image in"
  buh_line "your ${RBYC_DEPOT} matches the upstream digest exactly."
  buh_e
  buh_line "${RBYC_VOUCH} the ${RBYC_HALLMARKS}:"
  buh_e
  buh_tt "   " "${RBZ_VOUCH_HALLMARKS}"
  buh_e
  buh_line "Then ${RBYC_TALLY} to read each ${RBYC_HALLMARK}'s health:"
  buh_e
  buh_tt "   " "${RBZ_TALLY_HALLMARKS}"
  buh_e
  buh_line "Look for the PlantUML ${RBYC_HALLMARK} with verdict digest-pin. The"
  buh_line "image is pinned by content hash, but the project did not build it"
  buh_line "— that is what the verdict honestly reports."
  buh_e
  if test "${z_bottle_bound}" = "1"; then
    buh_line "${RBYC_PROBE_YES}${z_lk_pluml} ${RBYC_BOTTLE} ${RBYC_HALLMARK} bound and recorded"
  else
    buh_line "${RBYC_PROBE_NO}${z_lk_pluml} ${RBYC_BOTTLE} ${RBYC_HALLMARK} not set — ${RBYC_ORDAIN} above"
  fi
  buh_e

  buh_step1 "${RBYC_CHARGE} the ${z_lk_pluml} ${RBYC_CRUCIBLE}"
  buh_e
  buh_line "Now both ${RBYC_HALLMARKS} are in your ${RBYC_DEPOT} — a ${RBYC_KLUDGE_D} (or"
  buh_line "${RBYC_CONJURED}) ${RBYC_SENTRY} and a ${RBYC_BIND}-pinned PlantUML ${RBYC_BOTTLE}."
  buh_line "${RBYC_CHARGE} the ${RBYC_CRUCIBLE} to assemble them at runtime:"
  buh_e
  buh_tt "   " "${RBZ_CRUCIBLE_CHARGE}" "${z_moniker}"
  buh_e
  buh_line "The ${RBYC_SENTRY} container starts first and installs the network"
  buh_line "policy. The ${RBYC_BOTTLE} container starts second, behind the ${RBYC_SENTRY}"
  buh_line "— it can only reach what the ${RBYC_SENTRY} permits. PlantUML's"
  buh_line "Docker Hub image cannot phone home from inside the ${RBYC_CRUCIBLE}"
  buh_line "regardless of what the upstream image contains."
  buh_e
  buh_line "PlantUML listens on its container port; the ${RBYC_NAMEPLATE} maps"
  buh_line "workstation port 8001 to it. Open a browser:"
  buh_e
  buh_code "   http://localhost:8001"
  buh_e
  buh_line "Render a diagram. The ${RBYC_BOTTLE} can serve the request because"
  buh_line "it does not need outbound network for that — only the inbound"
  buh_line "request from your workstation. The ${RBYC_SENTRY} blocks egress"
  buh_line "regardless of what the ${RBYC_BOTTLE} image might attempt."
  buh_e
  buh_line "When you are done:"
  buh_e
  buh_tt "   " "${RBZ_CRUCIBLE_QUENCH}" "${z_moniker}"
  buh_e

  buh_step1 "The pattern"
  buh_e
  buh_line "Three ${RBYC_ORDAIN} modes, three trust contracts. ${RBYC_VOUCH} is mode-aware;"
  buh_line "its verdict names where trust for this ${RBYC_HALLMARK} comes from."
  buh_e
  buh_line "   ${RBYC_CONJURE}  project Dockerfile built by Cloud Build; SLSA ${RBYC_PROVENANCE}"
  buh_line "   ${RBYC_BIND}     upstream image mirrored by digest; trust the publisher"
  buh_line "   ${RBYC_GRAFT}    locally-built image pushed to GAR; trust this machine"
  buh_e
  buh_line "Read the ${RBYC_VOUCH} verdict: full-SLSA / digest-pin / GRAFTED tells you"
  buh_line "which contract holds. A ${RBYC_NAMEPLATE} can mix modes freely — runtime"
  buh_line "containment is the ${RBYC_SENTRY}'s job, not the build chain's."
  buh_e

  buh_tt "Return to start: " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

# eof
