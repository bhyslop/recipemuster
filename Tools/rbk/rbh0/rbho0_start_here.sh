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

# rbho_start_here — probe-aware menu into the handbook family.
# Probes are deliberately narrow: just RBRR populated + any-credential-
# present. Highlighting only arrows foundation actions based on repo
# state; it does not infer intent from detected roles.
rbho_start_here() {
  zrbho_sentinel

  buc_doc_brief "Probe-aware menu — route learner into the handbook track that fits their state"
  buc_doc_shown || return 0

  local -r z_docs="${RBRR_PUBLIC_DOCS_URL}"
  buyy_link_yawp "${z_docs}" "ccyolo"; local -r z_ccyolo="${z_buym_yelp}"
  buyy_link_yawp "${z_docs}" "tadmor"; local -r z_tadmor="${z_buym_yelp}"

  buh_section "Recipe Bottle — Onboarding Start"
  buh_e
  buh_line "  ${RBYC_RECIPE_BOTTLE} builds container images with supply-chain provenance"
  buh_line "  and runs untrusted containers behind enforced network isolation."
  buh_e
  buh_line "  This menu points you at handbook tracks — self-describing teaching"
  buh_line "  documents that explain concepts and show you live probe status."
  buh_e

  buh_section "Foundation"
  buh_e
  buh_line "    Configure your Repo's Environment"
  buh_line "      Universal prerequisite. ${RBYC_TABTARGETS}, ${RBYC_REGIMES},"
  buh_line "      ${RBYC_BURS} setup, validation, ${RBYC_LOGS}. Local-only, no cloud."
  buh_tt   "        " "${RBZ_ONBOARD_CRASH_COURSE}"
  buh_e
  buh_line "    Install ${RBYC_RETRIEVER} Credentials (existing projects only)"
  buh_line "      A ${RBYC_RETRIEVER} retrieves assets from the ${RBYC_DEPOT} for an"
  buh_line "      established project — read-only access to pull existing images from"
  buh_line "      the project's registry; no build, no publish."
  buh_line "      Place your ${RBYC_RBRA} credential file, verify, confirm you can pull images."
  buh_tt   "        " "${RBZ_ONBOARD_CRED_RETRIEVER}"
  buh_e
  buh_line "    Install ${RBYC_DIRECTOR} Credentials (existing projects only)"
  buh_line "      A ${RBYC_DIRECTOR} conducts cloud operations on an established project —"
  buh_line "      submits Cloud Builds, publishes and manages images in the ${RBYC_DEPOT}."
  buh_line "      Place your ${RBYC_RBRA} credential file, verify, confirm you can build and publish."
  buh_tt   "        " "${RBZ_ONBOARD_CRED_DIRECTOR}"
  buh_e

  buh_section "Kludged ${RBYC_CRUCIBLES}"
  buh_e
  buh_line "  A ${RBYC_CRUCIBLE} is this project's container sandbox: a ${RBYC_BOTTLE} runs"
  buh_line "  the workload, a ${RBYC_SENTRY} gates egress, and a ${RBYC_PENTACLE} owns their"
  buh_line "  shared network namespace. ${z_ccyolo} and ${z_tadmor} below both start"
  buh_line "  the same way:"
  buh_e
  buh_line "    * Build images locally — ${RBYC_KLUDGE} ${RBYC_SENTRY}/${RBYC_PENTACLE} and ${RBYC_BOTTLE}"
  buh_line "    * Start the sandbox    — ${RBYC_CHARGE} the ${RBYC_CRUCIBLE}"
  buh_e
  buh_line "  They diverge on what happens once the ${RBYC_CRUCIBLE} is ${RBYC_CHARGE_D}:"
  buh_e
  buh_line "    Inhabit the sandbox — explorer track"
  buh_line "      The ${RBYC_CCYOLO} ${RBYC_CRUCIBLE} runs Claude Code in a ${RBYC_BOTTLE} that"
  buh_line "      can only reach Anthropic. Requires a Claude OAuth subscription."
  buh_line "      After ${RBYC_CHARGE}:"
  buh_line "        * Run Claude Code in the sandbox (SSH into the ${RBYC_BOTTLE})"
  buh_line "        * Verify network containment     (manual curl against the allowlist)"
  buh_tt   "        " "${RBZ_ONBOARD_FIRST_CRUCIBLE}"
  buh_e
  buh_line "    Prove containment under attack — evaluator track"
  buh_line "      The tadmor ${RBYC_CRUCIBLE} pairs the same ${RBYC_SENTRY} with a hostile"
  buh_line "      ${RBYC_BOTTLE} carrying the ${RBYC_IFRIT} attack binary; >30 authored cases"
  buh_line "      exercise the containment primitives."
  buh_line "      After ${RBYC_CHARGE}:"
  buh_line "        * Tour the architecture      — ${RBYC_SENTRY}/${RBYC_PENTACLE}/${RBYC_BOTTLE} layers, defense-in-depth"
  buh_line "        * Run the adversarial suite  — ${RBYC_THEURGE} + ${RBYC_IFRIT}, per-case result reading"
  buh_tt   "        " "${RBZ_ONBOARD_TADMOR_SECURITY}"
  buh_e

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

  buh_section "Director Subtracks"
  buh_e
  buh_line "  All tracks below require ${RBYC_DIRECTOR} credentials and a provisioned ${RBYC_DEPOT}."
  buh_e
  buh_line "    ${RBHO_TRACK_FIRST_BUILD}"
  buh_line "      Build your first image in the cloud with verified provenance."
  buh_line "      Steps:"
  buh_line "        * Provision the builder toolchain (Inscribe the ${RBYC_RELIQUARY})"
  buh_line "        * Run your first tethered build   (${RBYC_CONJURE} a ${RBYC_SENTRY})"
  buh_line "        * Inspect images and SLSA         (${RBYC_TALLY}, ${RBYC_VOUCH}, ${RBYC_PLUMB}, ${RBYC_POUCH})"
  buh_line "        * Pull the image locally          (${RBYC_SUMMON} the ${RBYC_HALLMARK})"
  buh_line "        * Clean up                        (${RBYC_ABJURE}, ${RBYC_REKON})"
  buh_tt   "        " "${RBZ_ONBOARD_DIR_FIRST_BUILD}"
  buh_e
  buh_line "    ${RBYC_AIRGAP} Cloud Build"
  buh_line "      Build a ${RBYC_BOTTLE} with zero external network during Cloud Build."
  buh_line "      Pre-stage every input in the ${RBYC_DEPOT} first — rust upstream base,"
  buh_line "      then a project-authored toolchain image (the forge) — then build the"
  buh_line "      airgap ${RBYC_BOTTLE} against those pre-staged layers."
  buh_line "      Steps:"
  buh_line "        * Mirror the upstream base                 (${RBYC_ENSHRINE} into the ${RBYC_DEPOT})"
  buh_line "        * Build the toolchain as the new base      (${RBYC_CONJURE} the forge ${RBYC_TETHERED}, then re-${RBYC_ENSHRINE})"
  buh_line "        * Build the final image with zero network  (${RBYC_CONJURE} the airgap ${RBYC_BOTTLE} in ${RBYC_AIRGAP} mode)"
  buh_line "        * Start the sandbox and attack it          (${RBYC_CHARGE} moriah, run the adversarial suite)"
  buh_line "        * Compare provenance side by side          (airgap vs ${RBYC_TETHERED} ${RBYC_PLUMB})"
  buh_tt   "        " "${RBZ_ONBOARD_DIR_AIRGAP}"
  buh_e
  buh_line "    ${RBYC_BIND} — Safe PlantUML Container"
  buyy_link_yawp "${z_docs}" "Bind" "PlantUML"; local -r z_plantuml="${z_buym_yelp}"
  buyy_link_yawp "${z_docs}" "Nameplate" "pluml"; local -r z_pluml="${z_buym_yelp}"
  buh_line "      Mirror an upstream image by digest — no Dockerfile, no build."
  buh_line "      ${z_plantuml} renders diagrams but its Docker Hub image could"
  buh_line "      phone home. ${RBYC_BIND} pins it; the ${RBYC_SENTRY} blocks all egress."
  buh_line "      The ${z_pluml} ${RBYC_CRUCIBLE} deliberately mixes a ${RBYC_KLUDGE_D} ${RBYC_SENTRY} with"
  buh_line "      a bound ${RBYC_BOTTLE} — two ${RBYC_ORDAIN} modes in one ${RBYC_CRUCIBLE} is the"
  buh_line "      expected shape, not a defect."
  buh_line "      Steps:"
  buh_line "        * Prepare the guard image                  (${RBYC_KLUDGE} ${RBYC_SENTRY} or reuse ${RBYC_CONJURED})"
  buh_line "        * Pin the upstream image by digest         (${RBYC_BIND} ${z_plantuml})"
  buh_line "        * Check the provenance verdict             (${RBYC_VOUCH} reads digest-pin — image not built here)"
  buh_line "        * Render a diagram, observe blocked egress (${RBYC_CHARGE} the ${z_pluml} ${RBYC_CRUCIBLE})"
  buh_tt   "        " "${RBZ_ONBOARD_DIR_BIND}"
  buh_e
  buh_line "    ${RBYC_GRAFT} — Local Image Publishing"
  buh_line "      Push a locally-built image to the Depot. The ${RBYC_DIRECTOR} owns"
  buh_line "      the entire build — SLSA cannot vouch for this image. The Vouch"
  buh_line "      verdict reads GRAFTED: an explicit signal that provenance stops"
  buh_line "      at the local machine. Development and prototyping workflow, not"
  buh_line "      the enterprise path for production supply chain."
  buh_line "      Steps:"
  buh_line "        * Build a local image          (trivial busybox tag on your machine)"
  buh_line "        * Publish the local image      (${RBYC_GRAFT} it to the ${RBYC_DEPOT})"
  buh_line "        * Check the provenance verdict (${RBYC_VOUCH} reads GRAFTED, no chain)"
  buh_tt   "        " "${RBZ_ONBOARD_DIR_GRAFT}"
  buh_e

}

# eof
