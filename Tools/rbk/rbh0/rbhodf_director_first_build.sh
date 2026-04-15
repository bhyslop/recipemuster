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
# Recipe Bottle Handbook Onboarding - Director First Cloud Build

set -euo pipefail

test -z "${ZRBHODF_SOURCED:-}" || return 0
ZRBHODF_SOURCED=1

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

  # --- Function-specific yelp captures (not in RBYC vocabulary) ---

  buyy_link_yawp "${RBRR_PUBLIC_DOCS_URL}" "Vessel" "rbev-sentry-deb-tether"; local -r z_lk_vessel_name="${z_buym_yelp}"

  local z_tt=""

  # --- Header ---
  buh_section "Your First Cloud Build"
  buh_e
  buh_line "This track walks you through the complete ${RBYC_CONJURE} lifecycle:"
  buh_line "provision the builder toolchain, ${RBYC_ORDAIN} your first ${RBYC_VESSEL} via"
  buh_line "Cloud Build, inspect the result, pull it locally, and clean up."
  buh_e
  buh_line "You will build ${z_lk_vessel_name} — the same ${RBYC_SENTRY} you"
  buh_line "already know from the ${RBYC_CRUCIBLE} track, but this time built by"
  buh_line "Google Cloud Build with full SLSA provenance."
  buh_e

  # Prerequisite probes
  buh_line "Prerequisites:"
  buh_e
  if test "${z_has_director}" = "1"; then
    buyy_cmd_yawp "[*]"
    buh_line " ${z_buym_yelp} Director credential installed"
  else
    buh_error " [ ] Director credential missing — run:"
    buyy_tt_yawp "${RBZ_ONBOARD_CRED_DIRECTOR}"
    buh_line "      ${z_buym_yelp}"
  fi
  if test "${z_has_depot}" = "1"; then
    buyy_cmd_yawp "[*]"
    buh_line " ${z_buym_yelp} Depot configured (RBRR_DEPOT_PROJECT_ID populated)"
  else
    buh_error " [ ] Depot not configured — the Payor must establish the Depot first:"
    buyy_tt_yawp "${RBZ_ONBOARD_PAYOR_HB}"
    buh_line "      ${z_buym_yelp}"
  fi
  buh_e

  if test "${z_has_director}" = "0" || test "${z_has_depot}" = "0"; then
    buh_error "Complete the prerequisites above before continuing."
    buh_e
    buyy_tt_yawp "${RBZ_ONBOARD_START_HERE}"
    buh_line "Return to start: ${z_buym_yelp}"
    buh_e
    return 0
  fi

  buh_line "Configure this handbook session:"
  buh_e
  buh_code "   export ONBOARD_VESSEL=${z_vessel}"
  buh_e
  buh_line "This sets the vessel you will build throughout the track."
  buh_line "The remaining steps reference it by name."
  buh_e

  buh_step_style "Step " " — "

  buh_step1 "Inscribe the Reliquary"
  buh_e
  buh_line "The ${RBYC_RELIQUARY} is a set of builder tool images (skopeo,"
  buh_line "docker, gcloud, syft) that Cloud Build uses during ${RBYC_VESSEL}"
  buh_line "construction. Without it, conjure's preflight check fails."
  buh_e
  buh_line "Think of it as installing the toolchain before your first build."
  buh_line "This is a one-time operation — once inscribed, the reliquary"
  buh_line "stays in the ${RBYC_DEPOT} until you choose to refresh it."
  buh_e
  buh_line "Periodically re-inscribe to pick up newer tool versions. All ${RBYC_VESSELS}"
  buh_line "share the same ${RBYC_RELIQUARY} — one inscribe updates the toolchain"
  buh_line "for every build."
  buh_e
  buh_line "Inscribe:"
  buh_e
  buyy_tt_yawp "${RBZ_INSCRIBE_RELIQUARY}"
  buh_line "   ${z_buym_yelp}"
  buh_e
  buh_line "This mirrors four tool images from upstream into your Depot's"
  buh_line "GAR. Takes 2-5 minutes depending on network speed."
  buh_e
  buyy_cmd_yawp "r260324193326"
  buh_line "When inscribe completes, it prints a reliquary datestamp"
  buh_line "(e.g., ${z_buym_yelp}). Every vessel that uses Cloud Build"
  buh_line "needs this value in its regime file:"
  buh_e
  buh_code "   RBRV_RELIQUARY=<datestamp>"
  buh_e
  buyy_cmd_yawp "rbev-sentry-deb-tether/rbrv.env"
  buh_line "Open ${z_buym_yelp} and set the field,"
  buh_line "then commit the change."
  buh_e

  buh_step1 "${RBYC_CONJURE} the ${RBYC_SENTRY}"
  buh_e
  buh_line "${RBYC_CONJURE} is the build mode where Cloud Build constructs a"
  buh_line "vessel image from the project's Dockerfile and build context."
  buh_e
  buh_line "${RBYC_ORDAIN} is the command that triggers the full pipeline —"
  buh_line "it reads the vessel's ${RBYC_RBRV} regime to determine the mode"
  buh_line "(conjure, bind, or graft) and acts accordingly:"
  buh_e
  buyy_tt_yawp "${RBZ_ORDAIN_HALLMARK}"
  z_tt="${z_buym_yelp}"
  buyy_cmd_yawp ' ${ONBOARD_VESSEL}'
  buh_line "   ${z_tt}${z_buym_yelp}"
  buh_e
  buh_line "This builds on the ${RBYC_TETHERED} pool — Cloud Build has"
  buh_line "public internet access and pulls base images from upstream"
  buh_line "registries during the build. (The ${RBYC_AIRGAP} track removes"
  buh_line "that dependency.)"
  buh_e
  buh_line "The pipeline:"
  buh_e
  buh_line "  1. The host mints a ${RBYC_HALLMARK} — a timestamped tag"
  buh_line "     identifying this build"
  buh_line "  2. A ${RBYC_POUCH} (build context archive) is pushed to GAR"
  buh_line "  3. Cloud Build constructs the image across platforms"
  buh_line "  4. SLSA provenance is generated per platform digest"
  buh_line "  5. ${RBYC_VOUCH} verifies the provenance chain"
  buh_e
  buh_warn "Wall-clock: ~15-20 minutes for a 3-platform build."
  buh_line "The command blocks until Cloud Build finishes. Use the time"
  buh_line "to read ahead — the next steps explain what to look for."
  buh_e

  buh_step1 "Capture the hallmark"
  buh_e
  buh_line "When ${RBYC_ORDAIN} completes, it writes the ${RBYC_HALLMARK}"
  buh_line "to the ${RBYC_OUTPUT} directory — a fixed-path staging area"
  buh_line "that each tabtarget clears and recreates on entry."
  buh_line "Read the hallmark from the fact file and export it so"
  buh_line "you can copy-paste the commands in the remaining steps:"
  buh_e
  buh_code "   export ONBOARD_HALLMARK=\$(cat ${BURD_OUTPUT_DIR}/${RBF_FACT_HALLMARK})"
  buh_e

  buh_step1 "Tour the hallmark artifacts"
  buh_e
  buh_line "Every conjured ${RBYC_HALLMARK} produces a set of tagged"
  buh_line "artifacts in GAR. Each suffix serves a specific role:"
  buh_e
  buyy_cmd_yawp "{hallmark}${RBGC_ARK_SUFFIX_POUCH}"
  buh_line "   ${z_buym_yelp}"
  buh_line "      A FROM SCRATCH OCI image pushed from host to GAR before"
  buh_line "      the build. Contains the Dockerfile, scripts, and"
  buh_line "      configuration Cloud Build needs. Identical for tethered"
  buh_line "      and airgapped builds — the pool determines network"
  buh_line "      access, not the pouch."
  buh_e
  buyy_cmd_yawp "{hallmark}${RBGC_ARK_SUFFIX_IMAGE}"
  buh_line "   ${z_buym_yelp}"
  buh_line "      The consumer image — a multiplatform manifest list."
  buh_line "      This is what you pull and run."
  buh_e
  buyy_cmd_yawp "{hallmark}${RBGC_ARK_SUFFIX_ATTEST}-{arch}"
  buh_line "   ${z_buym_yelp}"
  buh_line "      Per-platform provenance-carrying image (one per platform)."
  buh_line "      Shares all layers with -image — only the manifest differs."
  buh_line "      These carry the GCB-attested digests used by vouch."
  buh_e
  buyy_cmd_yawp "{hallmark}${RBGC_ARK_SUFFIX_ABOUT}"
  buh_line "   ${z_buym_yelp}"
  buh_line "      SBOM (software bill of materials) + build info."
  buh_e
  buyy_cmd_yawp "{hallmark}${RBGC_ARK_SUFFIX_VOUCH}"
  buh_line "   ${z_buym_yelp}"
  buh_line "      SLSA provenance verification record."
  buh_e
  buyy_cmd_yawp "{hallmark}${RBGC_ARK_SUFFIX_DIAGS}"
  buh_line "   ${z_buym_yelp}"
  buh_line "      Diagnostics from the build."
  buh_e
  buh_line "Inspect them:"
  buh_e

  buh_step2 "Tally"
  buh_e
  buh_line "${RBYC_TALLY} lists all hallmarks and their health state:"
  buh_e
  buyy_tt_yawp "${RBZ_TALLY_HALLMARKS}"
  buh_line "   ${z_buym_yelp}"
  buh_e
  buh_line "Look for your ${RBYC_HALLMARK} with health state 'vouched' — that"
  buh_line "means SLSA provenance was verified."
  buh_e

  buh_step2 "Vouch"
  buh_e
  buh_line "${RBYC_VOUCH} verifies SLSA provenance for each platform"
  buh_line "digest in the ${RBYC_HALLMARK}. The ordain pipeline runs ${RBYC_VOUCH}"
  buh_line "automatically. If a build was interrupted before ${RBYC_VOUCH}"
  buh_line "completed, run this to reattempt ${RBYC_VOUCH} on untreated ${RBYC_HALLMARKS}:"
  buh_e
  buyy_tt_yawp "${RBZ_VOUCH_HALLMARKS}"
  buh_line "   ${z_buym_yelp}"
  buh_e
  buh_line "The ${RBYC_CONJURE} verdict is full SLSA — Cloud Build produced"
  buh_line "this image, and the provenance chain proves it."
  buh_e

  buh_step2 "Plumb"
  buh_e
  buh_line "${RBYC_PLUMB} displays the SBOM, build info, and Dockerfile"
  buh_line "that produced the hallmark. Two modes:"
  buh_e
  buyy_tt_yawp "${RBZ_PLUMB_FULL}"
  z_tt="${z_buym_yelp}"
  buyy_cmd_yawp ' ${ONBOARD_VESSEL} ${ONBOARD_HALLMARK}'
  buh_line "   ${z_tt}${z_buym_yelp}"
  buh_line "   Full provenance display — SBOM packages, build parameters,"
  buh_line "   Dockerfile content."
  buh_e
  buyy_tt_yawp "${RBZ_PLUMB_COMPACT}"
  z_tt="${z_buym_yelp}"
  buyy_cmd_yawp ' ${ONBOARD_VESSEL} ${ONBOARD_HALLMARK}'
  buh_line "   ${z_tt}${z_buym_yelp}"
  buh_line "   Compact summary — one-line-per-artifact overview."
  buh_e

  buh_step1 "Summon the hallmark"
  buh_e
  buh_line "${RBYC_SUMMON} pulls a set of images affiliated with a"
  buh_line "${RBYC_HALLMARK} that has been ${RBYC_VOUCHED} to your local"
  buh_line "Docker image cache:"
  buh_e
  buyy_tt_yawp "${RBZ_SUMMON_HALLMARK}"
  z_tt="${z_buym_yelp}"
  buyy_cmd_yawp ' ${ONBOARD_VESSEL} ${ONBOARD_HALLMARK}'
  buh_line "   ${z_tt}${z_buym_yelp}"
  buh_e
  buyy_cmd_yawp "{hallmark}${RBGC_ARK_SUFFIX_IMAGE}"
  buh_line "   ${z_buym_yelp}"
  buh_line "   is a multiplatform manifest list."
  buh_line "   Docker resolves it to the image matching your host"
  buh_line "   architecture — the same image that ${RBYC_CHARGE} uses when"
  buh_line "   starting a ${RBYC_CRUCIBLE} from cloud-built ${RBYC_HALLMARKS}."
  buh_e

  # Summoned probe
  if test "${z_conjure_summoned}" = "1"; then
    buyy_cmd_yawp "[*]"
    buh_line " ${z_buym_yelp} Conjured sentry image found locally (summoned from GAR)"
  else
    buh_line " [ ] No conjured sentry image found locally — run ${RBYC_SUMMON} above"
  fi
  buh_e

  buh_step1 "Abjure and Rekon — hallmark lifecycle"
  buh_e
  buh_line "${RBYC_REKON} lists the raw tags for a ${RBYC_VESSEL}"
  buh_line "package in GAR. Run it before and after ${RBYC_ABJURE} to see"
  buh_line "the full lifecycle:"
  buh_e
  buyy_tt_yawp "${RBZ_REKON_IMAGE}"
  z_tt="${z_buym_yelp}"
  buyy_cmd_yawp ' ${ONBOARD_VESSEL}'
  buh_line "   ${z_tt}${z_buym_yelp}"
  buh_e
  buh_line "You should see all five durable tags for your ${RBYC_HALLMARK}:"
  buyy_cmd_yawp "${RBGC_ARK_SUFFIX_IMAGE}, ${RBGC_ARK_SUFFIX_ABOUT}, ${RBGC_ARK_SUFFIX_VOUCH}, ${RBGC_ARK_SUFFIX_POUCH}, ${RBGC_ARK_SUFFIX_DIAGS}"
  buh_line "   ${z_buym_yelp}"
  buh_e
  buh_line "${RBYC_ABJURE} removes all artifacts for a ${RBYC_HALLMARK}"
  buh_line "from GAR. This is permanent — the hallmark and all its"
  buh_line "tags are deleted:"
  buh_e
  buyy_tt_yawp "${RBZ_ABJURE_HALLMARK}"
  z_tt="${z_buym_yelp}"
  buyy_cmd_yawp ' ${ONBOARD_VESSEL} ${ONBOARD_HALLMARK}'
  buh_line "   ${z_tt}${z_buym_yelp}"
  buh_e
  buh_line "After abjure, run Rekon again:"
  buh_e
  buyy_tt_yawp "${RBZ_REKON_IMAGE}"
  z_tt="${z_buym_yelp}"
  buyy_cmd_yawp ' ${ONBOARD_VESSEL}'
  buh_line "   ${z_tt}${z_buym_yelp}"
  buh_e
  buh_line "The tags for your ${RBYC_HALLMARK} should be gone. The image is no"
  buh_line "longer in the ${RBYC_DEPOT}."
  buh_e

  buh_section "What you learned"
  buh_e
  buh_line "You just completed the full conjure lifecycle:"
  buh_e
  buh_line "  1. ${RBYC_RELIQUARY} — builder toolchain provisioned"
  buh_line "  2. ${RBYC_CONJURE} — vessel built by Cloud Build with SLSA provenance"
  buh_line "  3. ${RBYC_TALLY}/${RBYC_VOUCH} — health and provenance verified"
  buh_line "  4. ${RBYC_PLUMB} — SBOM and build info inspected"
  buh_line "  5. ${RBYC_SUMMON} — consumer image pulled locally"
  buh_line "  6. ${RBYC_ABJURE}/${RBYC_REKON} — lifecycle cleanup"
  buh_e
  buh_line "This was a ${RBYC_TETHERED} build — Cloud Build had"
  buh_line "internet access. The next track teaches you to remove"
  buh_line "that dependency entirely."
  buh_e

  # --- Return to start ---
  buyy_tt_yawp "${RBZ_ONBOARD_START_HERE}"
  buh_line "Return to start: ${z_buym_yelp}"
  buh_e
}

# eof
