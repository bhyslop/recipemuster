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
# Recipe Bottle Handbook Onboarding - Director Airgap Cloud Build
#
# Sequel to Your First Cloud Build (rbhodf). Teaches the airgap
# supply chain: enshrine upstream, conjure forge tethered, conjure
# airgap ifrit from enshrined forge, install into moriah, run the
# same 34-case security suite, compare plumb against tadmor baseline.

set -euo pipefail

test -z "${ZRBHODA_SOURCED:-}" || return 0
ZRBHODA_SOURCED=1

rbho_director_airgap() {
  zrbho_sentinel

  buc_doc_brief "${RBHO_TRACK_AIRGAP} — enshrine, conjure forge, conjure airgap, charge moriah, compare plumb"
  buc_doc_shown || return 0

  local -r z_moniker="moriah"
  local -r z_tether_moniker="tadmor"
  local -r z_forge_vessel="rbev-bottle-ifrit-forge"
  local -r z_airgap_vessel="rbev-bottle-ifrit-airgap"
  local -r z_tether_vessel="rbev-bottle-ifrit-tether"
  local -r z_airgap_rbrv="${RBRR_VESSEL_DIR}/${z_airgap_vessel}/rbrv.env"
  local -r z_moriah_rbrn="${RBBC_dot_dir}/${z_moniker}/${RBCC_rbrn_file}"
  local -r z_tether_rbrn="${RBBC_dot_dir}/${z_tether_moniker}/${RBCC_rbrn_file}"

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
      case "${z_line}" in RBRR_DEPOT_MONIKER=?*) z_has_depot=1; break ;; esac
    done < "${RBBC_rbrr_file}"
  fi

  local z_airgap_base_enshrined=0
  if test -f "${z_airgap_rbrv}"; then
    local z_anchor=""
    z_anchor=$(zrbho_po_extract_capture "${z_airgap_rbrv}" "RBRV_IMAGE_1_ANCHOR") || z_anchor=""
    test -n "${z_anchor}" && z_airgap_base_enshrined=1
  fi

  local z_airgap_ordained=0
  if test -f "${z_moriah_rbrn}"; then
    local z_moriah_hallmark=""
    z_moriah_hallmark=$(zrbho_po_extract_capture "${z_moriah_rbrn}" "RBRN_BOTTLE_HALLMARK") || z_moriah_hallmark=""
    case "${z_moriah_hallmark}" in
      ""|PENDING-*) ;;
      *) z_airgap_ordained=1 ;;
    esac
  fi

  local z_tether_ready=0
  local z_tether_hallmark=""
  if test -f "${z_tether_rbrn}"; then
    z_tether_hallmark=$(zrbho_po_extract_capture "${z_tether_rbrn}" "RBRN_BOTTLE_HALLMARK") || z_tether_hallmark=""
    case "${z_tether_hallmark}" in
      ""|PENDING-*) z_tether_hallmark="" ;;
      *) z_tether_ready=1 ;;
    esac
  fi

  buyy_link_yawp "${RBRR_PUBLIC_DOCS_URL}" "Vessel"    "${z_forge_vessel}";   local -r z_lk_forge="${z_buym_yelp}"
  buyy_link_yawp "${RBRR_PUBLIC_DOCS_URL}" "Vessel"    "${z_airgap_vessel}";  local -r z_lk_airgap="${z_buym_yelp}"
  buyy_link_yawp "${RBRR_PUBLIC_DOCS_URL}" "Nameplate" "${z_moniker}";        local -r z_lk_moriah="${z_buym_yelp}"
  buyy_link_yawp "${RBRR_PUBLIC_DOCS_URL}" "Nameplate" "${z_tether_moniker}"; local -r z_lk_tadmor="${z_buym_yelp}"

  buh_section "${RBHO_TRACK_AIRGAP} — Your Own Supply Chain"
  buh_e
  buh_line "In ${RBHO_TRACK_FIRST_BUILD} you ${RBYC_ORDAIN}ed a ${RBYC_VESSEL} on the"
  buh_line "${RBYC_TETHERED} pool — Cloud Build pulled base images from the"
  buh_line "public internet during the build. ${RBYC_AIRGAP} removes that"
  buh_line "dependency: zero network during the build, every input pre-staged"
  buh_line "in your ${RBYC_DEPOT}."
  buh_e
  buh_line "This track builds ${z_lk_airgap} — the ${RBYC_BOTTLE} variant that"
  buh_line "matches the ifrit you met in the ${z_lk_tadmor} adversarial suite,"
  buh_line "now with full supply-chain discipline. The chain has three links:"
  buh_e
  buh_line "  1. Mirror the rust base from upstream into your ${RBYC_DEPOT} (${RBYC_ENSHRINE})"
  buh_line "  2. Build a project-authored toolchain image (the forge) ${RBYC_TETHERED}"
  buh_line "  3. Build the airgap ${RBYC_BOTTLE} ${RBYC_AIRGAP} from the enshrined forge"
  buh_e
  buh_line "Then drive the resulting ${RBYC_HALLMARK} into the ${z_lk_moriah}"
  buh_line "${RBYC_NAMEPLATE} and run the same 34 containment attacks against"
  buh_line "it that you ran against ${z_lk_tadmor}."
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
    buh_line "${RBYC_PROBE_YES}${RBYC_DEPOT} configured (RBRR_DEPOT_MONIKER populated)"
  else
    buh_line "${RBYC_PROBE_NO}${RBYC_DEPOT} not configured — the ${RBYC_PAYOR} must establish the ${RBYC_DEPOT}:"
    buh_tt "      " "${RBZ_ONBOARD_PAYOR_HB}"
  fi
  buh_e
  buh_line "This track assumes you have completed ${RBHO_TRACK_FIRST_BUILD} —"
  buh_line "the ${RBYC_RELIQUARY} is inscribed and you have ${RBYC_ORDAIN}ed at least"
  buh_line "one ${RBYC_VESSEL} on the ${RBYC_TETHERED} pool."
  buh_e

  if test "${z_has_director}" = "0" || test "${z_has_depot}" = "0"; then
    buh_error "Complete the prerequisites above before continuing."
    buh_e
    buh_tt "Return to start: " "${RBZ_ONBOARD_START_HERE}"
    buh_e
    return 0
  fi

  buh_line "Configure this handbook session:"
  buh_e
  buh_code "   export ${RBYC_HANDBOOK_NAMEPLATE_NAME}=${z_moniker}"
  buh_e
  buh_line "The ${RBYC_CHARGE} and suite commands below reference the ${RBYC_NAMEPLATE}"
  buh_line "by name. The two ${RBYC_VESSEL} names shift mid-track (forge →"
  buh_line "airgap), so vessels appear as literals in commands."
  buh_e

  buh_step_style "Step " " — "

  buh_step1 "${RBYC_ENSHRINE} the upstream base"
  buh_e
  buh_line "The airgap chain starts with ownership. ${RBYC_ENSHRINE} mirrors"
  buh_line "an upstream image — here rust:slim-bookworm — into your ${RBYC_DEPOT}"
  buh_line "under a content-addressed anchor. Once enshrined, builds pull the"
  buh_line "base from your ${RBYC_DEPOT} without touching the public internet."
  buh_e
  buh_line "The forge ${RBYC_VESSEL} ${z_lk_forge} declares its upstream base in"
  buh_line "its ${RBYC_RBRV} file:"
  buh_e
  buh_code "   RBRV_IMAGE_1_ORIGIN=rust:slim-bookworm"
  buh_code "   RBRV_IMAGE_1_ANCHOR=rust-slim-bookworm-5ae2d2ef98"
  buh_e
  buh_line "ORIGIN names where the image comes from; ANCHOR is the name it"
  buh_line "takes inside your ${RBYC_DEPOT}. Run ${RBYC_ENSHRINE}:"
  buh_e
  buh_tt "   " "${RBZ_ENSHRINE_VESSEL}"
  buh_e
  buh_line "${RBYC_ENSHRINE} sweeps every ${RBYC_VESSEL} ${RBYC_REGIME}, resolves the"
  buh_line "upstream bases they declare, and creates the mirror tags. It's"
  buh_line "idempotent — already-enshrined bases are skipped."
  buh_e

  buh_step1 "${RBYC_CONJURE} the forge ${RBYC_TETHERED}, then ${RBYC_ENSHRINE} its result"
  buh_e
  buh_line "The forge is a project-authored toolchain image. It pre-stages"
  buh_line "apt packages and warms the cargo cache so the airgap build"
  buh_line "downstream has nothing to fetch."
  buh_e
  buh_line "Build it on the ${RBYC_TETHERED} pool — the forge ${RBYC_RBRV} declares:"
  buh_e
  buh_code "   RBRV_EGRESS_MODE=tether"
  buh_e
  buh_line "${RBYC_ORDAIN} reads this and routes to the ${RBYC_TETHERED} pool:"
  buh_e
  buh_tt "   " "${RBZ_ORDAIN_HALLMARK}" "" " ${z_forge_vessel}"
  buh_e
  buh_line "Wall-clock ~15-20 minutes across the declared platforms. The"
  buh_line "forge is toolchain plumbing that your customer code will be"
  buh_line "built against — it is not itself customer code, which is why"
  buh_line "${RBYC_TETHERED} build is acceptable at this layer."
  buh_e
  buh_line "Now bridge: the airgap ${RBYC_VESSEL} ${z_lk_airgap} declares the"
  buh_line "forge as its base:"
  buh_e
  buh_code "   RBRV_IMAGE_1_ORIGIN=rbev-bottle-ifrit-forge"
  buh_code "   RBRV_IMAGE_1_ANCHOR="
  buh_e
  buh_line "ANCHOR starts empty because it resolves to the forge's freshly"
  buh_line "built ${RBYC_HALLMARK}. Re-run ${RBYC_ENSHRINE} to populate it:"
  buh_e
  buh_tt "   " "${RBZ_ENSHRINE_VESSEL}"
  buh_e
  buh_line "This time ${RBYC_ENSHRINE} sees the internal reference, finds the"
  buh_line "forge ${RBYC_HALLMARK} in your ${RBYC_DEPOT}, and writes the resolved"
  buh_line "anchor into the airgap vessel's ${RBYC_RBRV}. Commit that change."
  buh_e
  if test "${z_airgap_base_enshrined}" = "1"; then
    buh_line "${RBYC_PROBE_YES}Airgap base anchor populated — forge is ready to serve as airgap base"
  else
    buh_line "${RBYC_PROBE_NO}Airgap base anchor empty — ${RBYC_CONJURE} the forge and re-run ${RBYC_ENSHRINE}"
  fi
  buh_e

  buh_step1 "${RBYC_CONJURE} the airgap ${RBYC_BOTTLE} ${RBYC_AIRGAP}"
  buh_e
  buh_line "Now the airgap build has everything it needs inside your"
  buh_line "${RBYC_DEPOT} — rust toolchain, apt packages, cargo cache, all"
  buh_line "pre-staged in the enshrined forge."
  buh_e
  buh_line "The airgap ${RBYC_VESSEL}'s Dockerfile starts FROM the forge:"
  buh_e
  buh_code "   ARG RBF_IMAGE_1"
  buh_code "   FROM \${RBF_IMAGE_1}"
  buh_e
  buh_line "RBF_IMAGE_1 is resolved from RBRV_IMAGE_1_ANCHOR at build time."
  buh_line "The airgap pool has zero external network — Cloud Build reaches"
  buh_line "your ${RBYC_DEPOT} and nothing else."
  buh_e
  buh_line "${RBYC_ORDAIN}:"
  buh_e
  buh_tt "   " "${RBZ_ORDAIN_HALLMARK}" "" " ${z_airgap_vessel}"
  buh_e
  buh_line "Another ~15-20 minutes. The ${RBYC_VOUCH} at the end attests both"
  buh_line "the SLSA ${RBYC_PROVENANCE} chain and the airgap condition — that"
  buh_line "the build saw no public internet."
  buh_e
  buh_line "Your customer code was compiled against a supply chain you"
  buh_line "controlled end-to-end."
  buh_e

  buh_step1 "Install the ${RBYC_HALLMARK} into ${z_lk_moriah} and run the security suite"
  buh_e
  buh_line "The cloud-built ${RBYC_HALLMARK} is now in your ${RBYC_DEPOT}. The"
  buh_line "${z_lk_moriah} ${RBYC_NAMEPLATE} names how that ${RBYC_HALLMARK} runs as"
  buh_line "a ${RBYC_CRUCIBLE}. ${RBYC_HALLMARK} and ${RBYC_NAMEPLATE} are separate"
  buh_line "artifacts — one is the build product, the other is the deployment"
  buh_line "config."
  buh_e
  buh_line "Read the ${RBYC_HALLMARK} from the fact file ${RBYC_ORDAIN} wrote:"
  buh_e
  buh_code "   export ${RBYC_HANDBOOK_HALLMARK_NAME}=\$(cat ${BURD_OUTPUT_DIR}/${RBF_FACT_HALLMARK})"
  buh_e
  buyy_cmd_yawp "${z_moriah_rbrn}"; local -r z_lk_moriah_file="${z_buym_yelp}"
  buh_line "Open ${z_lk_moriah_file} and set the ${RBYC_BOTTLE} hallmark, replacing"
  buh_line "the PENDING-ordination placeholder:"
  buh_e
  buh_code "   RBRN_BOTTLE_HALLMARK=${RBYC_HANDBOOK_HALLMARK_REF}"
  buh_e
  buh_line "Commit the change."
  buh_e
  if test "${z_airgap_ordained}" = "1"; then
    buh_line "${RBYC_PROBE_YES}${z_lk_moriah} ${RBYC_BOTTLE} hallmark installed"
  else
    buh_line "${RBYC_PROBE_NO}${z_lk_moriah} ${RBYC_BOTTLE} hallmark is PENDING-ordination — install it above"
  fi
  buh_e
  buh_line "${RBYC_CHARGE} ${z_lk_moriah} with the airgap-built image:"
  buh_e
  buh_tt "   " "${RBZ_CRUCIBLE_CHARGE}" "${z_moniker}"
  buh_e
  buh_line "Run the full 34-case security suite — the same suite you ran"
  buh_line "against ${z_lk_tadmor} with kludged ${RBYC_HALLMARKS}:"
  buh_e
  buh_tt "   " "rbtd-r" "${z_moniker}"
  buh_e
  buh_line "Expect green across the board. Same attacks, same containment"
  buh_line "boundaries, same expected responses — now against a ${RBYC_BOTTLE}"
  buh_line "built with full airgap supply-chain discipline."
  buh_e
  buh_line "This closes the loop: you built the validated airgap security"
  buh_line "infrastructure yourself. Containment holds regardless of how the"
  buh_line "${RBYC_HALLMARK} was produced."
  buh_e

  buh_step1 "Compare ${RBYC_PLUMB} output — ${RBYC_TETHERED} vs ${RBYC_AIRGAP}"
  buh_e
  buh_line "${RBYC_PLUMB} displays the ${RBYC_PROVENANCE} evidence attached to a"
  buh_line "${RBYC_HALLMARK}. Run it against ${z_lk_moriah}'s airgap ${RBYC_BOTTLE}"
  buh_line "alongside ${z_lk_tadmor}'s ${RBYC_TETHERED} ${RBYC_BOTTLE} to see what"
  buh_line "airgap adds to the record."
  buh_e
  if test "${z_tether_ready}" = "1"; then
    buh_line "${RBYC_PROBE_YES}${z_lk_tadmor} bottle hallmark available for comparison"
  else
    buh_line "${RBYC_PROBE_NO}${z_lk_tadmor} has no bottle hallmark — run the ${z_lk_tadmor} track first for the full comparison"
  fi
  buh_e
  buh_line "Airgap ${RBYC_BOTTLE} full ${RBYC_PROVENANCE}:"
  buh_e
  buh_tt "   " "${RBZ_PLUMB_FULL}" "" " ${z_airgap_vessel} ${RBYC_HANDBOOK_HALLMARK_REF}"
  buh_e
  if test "${z_tether_ready}" = "1"; then
    buyy_cmd_yawp "${z_tether_hallmark}"; local -r z_lk_tether_hallmark="${z_buym_yelp}"
    buh_line "Tethered ${RBYC_BOTTLE} — ${z_lk_tadmor}'s current bottle ${RBYC_HALLMARK} is ${z_lk_tether_hallmark}:"
    buh_e
    buh_tt "   " "${RBZ_PLUMB_FULL}" "" " ${z_tether_vessel} ${z_tether_hallmark}"
    buh_e
  else
    buyy_cmd_yawp "${z_tether_rbrn}"; local -r z_lk_tether_file="${z_buym_yelp}"
    buh_line "Tethered ${RBYC_BOTTLE} — once ${z_lk_tadmor} is ${RBYC_CHARGE}d, read RBRN_BOTTLE_HALLMARK"
    buh_line "from ${z_lk_tether_file} and plumb it:"
    buh_e
    buh_tt "   " "${RBZ_PLUMB_FULL}" "" " ${z_tether_vessel} <tadmor-hallmark>"
    buh_e
  fi
  buh_line "What to look for, side by side:"
  buh_e
  buh_line "  ${RBYC_SBOM}       Same rust/cargo package set if both ${RBYC_HALLMARKS}"
  buh_line "              were ${RBYC_CONJURED}. The ifrit binary is functionally"
  buh_line "              equivalent either way."
  buh_e
  buh_line "  Build info  The airgap ${RBYC_HALLMARK} records a sealed supply"
  buh_line "              chain — base digests resolved to your ${RBYC_DEPOT},"
  buh_line "              no upstream resolution during the build. A ${RBYC_TETHERED}"
  buh_line "              ${RBYC_HALLMARK} shows upstream resolution within the build"
  buh_line "              window. A kludged ${RBYC_HALLMARK} carries no Cloud Build"
  buh_line "              record at all."
  buh_e
  buh_line "  ${RBYC_PROVENANCE}  The airgap ${RBYC_VOUCH} attests the airgap condition"
  buh_line "              in addition to the standard SLSA chain. ${RBYC_TETHERED}"
  buh_line "              ${RBYC_VOUCHED} ${RBYC_HALLMARKS} carry SLSA without that"
  buh_line "              extra attestation."
  buh_e
  buh_line "For a compact summary:"
  buh_e
  buh_tt "   " "${RBZ_PLUMB_COMPACT}" "" " ${z_airgap_vessel} ${RBYC_HANDBOOK_HALLMARK_REF}"
  buh_e

  buh_step1 "The pattern"
  buh_e
  buh_line "An airgap supply chain has three links. Any future airgap build"
  buh_line "follows the same shape:"
  buh_e
  buh_line "   1. ${RBYC_ENSHRINE} the upstream base into your ${RBYC_DEPOT}"
  buh_line "   2. ${RBYC_CONJURE} the forge ${RBYC_TETHERED}, re-${RBYC_ENSHRINE} its ${RBYC_HALLMARK}"
  buh_line "   3. ${RBYC_CONJURE} the final ${RBYC_BOTTLE} ${RBYC_AIRGAP} from the enshrined forge"
  buh_e
  buh_line "${RBYC_PLUMB} distinguishes three build-info signatures:"
  buh_e
  buh_line "   ${RBYC_AIRGAP}     sealed chain — base digests resolved to your ${RBYC_DEPOT}"
  buh_line "   ${RBYC_TETHERED}   upstream resolution during the build window"
  buh_line "   ${RBYC_KLUDGE_D}    no Cloud Build record — local provenance only"
  buh_e

  buh_tt "Return to start: " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

# eof
