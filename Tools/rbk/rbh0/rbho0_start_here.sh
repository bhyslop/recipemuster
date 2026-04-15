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
# Recipe Bottle Handbook Onboarding - Start Here (probe-aware menu)

set -euo pipefail

test -z "${ZRBHO0_SOURCED:-}" || return 0
ZRBHO0_SOURCED=1

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
  buh_line "  ${RBYC_RECIPE_BOTTLE} builds container images with supply-chain provenance"
  buh_line "  and runs untrusted containers behind enforced network isolation."
  buh_e
  buh_line "  This menu points you at handbook tracks — self-describing teaching"
  buh_line "  documents that explain concepts and show you live probe status."
  buh_e

  # --- Foundation ---
  buh_section "Foundation"
  buh_e
  buh_line "    Configure your Repo's Environment"
  buh_line "      Universal prerequisite. ${RBYC_TABTARGETS}, ${RBYC_REGIMES},"
  buh_line "      ${RBYC_BURS} setup, validation, ${RBYC_LOGS}. Local-only, no cloud."
  buh_tt   "        " "${RBZ_ONBOARD_CRASH_COURSE}"
  buh_e
  buh_line "    Install ${RBYC_RETRIEVER} Credentials"
  buh_line "      For joining an established project."
  buh_line "      Place your ${RBYC_RBRA} credential file, verify, confirm you can pull images."
  buh_tt   "        " "${RBZ_ONBOARD_CRED_RETRIEVER}"
  buh_e
  buh_line "    Install ${RBYC_DIRECTOR} Credentials"
  buh_line "      For joining an established project."
  buh_line "      Place your ${RBYC_RBRA} credential file, verify, confirm you can build and publish."
  buh_tt   "        " "${RBZ_ONBOARD_CRED_DIRECTOR}"
  buh_e
  buh_line "    Start a ${RBYC_CRUCIBLE} Using Local Builds"
  buh_line "      The ${RBYC_CCYOLO} ${RBYC_CRUCIBLE} runs Claude Code in a container that can"
  buh_line "      only reach Anthropic. Requires a Claude OAuth subscription."
  buh_line "      Steps:"
  buh_line "        * Build images locally      — ${RBYC_KLUDGE} ${RBYC_SENTRY}/${RBYC_PENTACLE} and ${RBYC_BOTTLE}"
  buh_line "        * Configure local network   — amend ${RBYC_NAMEPLATE} ${RBYC_RBRN} file"
  buh_line "        * Start the sandbox         — ${RBYC_CHARGE} the ${RBYC_CRUCIBLE}"
  buh_line "        * Shell into the container  — ${RBYC_RACK} the ${RBYC_BOTTLE}"
  buh_line "      No cloud, no credentials beyond your own."
  buh_tt   "        " "${RBZ_ONBOARD_FIRST_CRUCIBLE}"
  buh_e

  # --- Create Payor and Depot ---
  buh_section "Create Payor and Depot"
  buh_e
  buh_line "  A ${RBYC_DEPOT} is the facility where the team's container images are"
  buh_line "  built and stored — the ground truth other tracks rest on."
  buh_e
  buh_line "    ${RBYC_PAYOR} — establish a ${RBYC_MANOR} and provision the ${RBYC_DEPOT}"
  buh_tt   "        " "${RBZ_ONBOARD_PAYOR_HB}"
  buh_e
  buh_line "    ${RBYC_GOVERNOR} — administer service accounts for ${RBYC_DIRECTORS} and ${RBYC_RETRIEVERS}"
  buh_tt   "        " "${RBZ_ONBOARD_GOVERNOR_HB}"
  buh_e

  # --- Director subtracks ---
  buh_section "Director Subtracks"
  buh_e
  buh_line "    Your First Cloud Build"
  buh_line "      Provision the builder toolchain, ${RBYC_ORDAIN} your first"
  buh_line "      ${RBYC_VESSEL} via Cloud Build, and tour the result."
  buh_line "      Steps:"
  buh_line "        * Inscribe the ${RBYC_RELIQUARY} — provision builder tool images"
  buh_line "        * ${RBYC_CONJURE} ${RBYC_SENTRY} — first tethered cloud build"
  buh_line "        * Tour: ${RBYC_TALLY}, ${RBYC_VOUCH}, ${RBYC_PLUMB}, ${RBYC_POUCH} — inspect images and SLSA"
  buh_line "        * ${RBYC_SUMMON} — pull the hallmark locally"
  buh_line "        * ${RBYC_ABJURE} and ${RBYC_REKON} — hallmark lifecycle"
  buh_line "      Requires: Director credentials and a provisioned Depot."
  buh_tt   "        " "${RBZ_ONBOARD_DIR_FIRST_BUILD}"
  buh_e
  buh_line "    ${RBYC_AIRGAP} Cloud Build"
  buh_line "      Build with zero upstream access. ${RBYC_ENSHRINE} mirrors base images"
  buh_line "      into the Depot so Cloud Build never reaches the internet."
  buh_line "      Steps:"
  buh_line "        * ${RBYC_ENSHRINE} base images — mirror upstream into the Depot"
  buh_line "        * ${RBYC_CONJURE} ${RBYC_SENTRY} airgapped — same vessel, full isolation"
  buh_line "        * Compare ${RBYC_PLUMB} output — tethered vs airgap side by side"
  buh_line "      Requires: Reliquary inscribed (previous track)."
  buh_e
  buh_line "    ${RBYC_BIND} — Safe PlantUML Container"
  buh_line "      Mirror an upstream image by digest — no Dockerfile, no build."
  buh_line "      PlantUML renders diagrams but its Docker Hub image could"
  buh_line "      phone home. Bind pins it; the Sentry blocks all egress."
  buh_line "      Steps:"
  buyy_link_yawp "${z_docs}" "Bind" "PlantUML"; local -r z_plantuml="${z_buym_yelp}"
  buh_line "        * ${RBYC_BIND} ${z_plantuml} — pin upstream image by digest"
  buh_line "        * Inspect ${RBYC_VOUCH} verdict — digest-pin, no SLSA (image not built here)"
  buyy_link_yawp "${z_docs}" "Nameplate" "pluml"; local -r z_pluml="${z_buym_yelp}"
  buh_line "        * ${RBYC_CHARGE} the ${z_pluml} ${RBYC_CRUCIBLE} — render a diagram, observe blocked egress"
  buh_line "      You get the tool without the risk."
  buh_e
  buh_line "    ${RBYC_GRAFT} — Local Image Publishing"
  buh_line "      Push a locally-built image to the Depot. The user owns the"
  buh_line "      entire build — SLSA cannot vouch for this image. Vouch verdict"
  buh_line "      is GRAFTED: an explicit signal that provenance stops at the"
  buh_line "      local machine. Not the enterprise path for safe image creation."
  buh_line "      Steps:"
  buh_line "        * ${RBYC_KLUDGE} a vessel image locally"
  buh_line "        * ${RBYC_GRAFT} — push local image to the Depot"
  buh_line "        * Inspect ${RBYC_VOUCH} verdict — GRAFTED, no provenance chain"
  buh_line "      Development and prototyping workflow, not production supply chain."
  buh_e

}

# eof
