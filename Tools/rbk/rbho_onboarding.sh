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
  buh_line "The command you just ran is a ${RBYC_TABTARGET} — a launcher script"
  buh_line "in the ${BURC_TABTARGET_DIR}/ directory. Tab completion narrows by prefix: type \`${BURC_TABTARGET_DIR}/rbw-<TAB>\` to see every"
  buh_line "${RBYC_RECIPE_BOTTLE} command."
  buh_e

  # --- Step 2: View the project config (BURC) ---
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

  # --- Step 3: View your station (BURS) ---
  buh_step1 "View your personal station"
  buh_e
  buh_line "${RBYC_BURS} is your per-developer station file: local, gitignored,"
  buh_line "holds things that vary per machine. Run the renderer:"
  buh_e
  buh_tt   "   " "${BUWZ_RS_RENDER}"
  buh_e
  buh_line "The repo-vs-personal split is deliberate: ${RBYC_BURC} travels with the code; ${RBYC_BURS} stays on your machine."
  buh_e

  # --- Step 4: Validate your station ---
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

  # --- Step 5: Validate the repo regime ---
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

  # --- Step 6: Check your logs ---
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

  # --- Step 7: The pattern ---
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

  # --- Step 8: Next steps ---
  buh_step1 "Next steps"
  buh_e
  buh_line "Your repo environment is configured. The tools work, errors explain"
  buh_line "themselves, and ${RBYC_LOGS} land where you told them to."
  buh_e
  buh_line "Return to the start menu for what to do next:"
  buh_tt   "   " "${RBZ_ONBOARD_START_HERE}"
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

######################################################################
# Install Retriever Credentials

rbho_credential_retriever() {
  buc_doc_brief "Install retriever credentials — place RBRA key, validate, confirm pull access"
  buc_doc_shown || return 0

  buh_section "Install Retriever Credentials"
  buh_e
  buh_line "A ${RBYC_RETRIEVER} pulls container images from the"
  buh_line "  ${RBYC_DEPOT} — read-only access to what others have built."
  buh_e

  zrbho_credential_install "${RBCC_role_retriever}"

  # --- Step 4: Confirm live access ---
  buh_step1 "Confirm live access"
  buh_e
  buh_line "Run ${RBYC_TALLY} to list ${RBYC_HALLMARKS} in the registry using your retriever credential:"
  buh_e
  buh_tt   "   " "${RBZ_TALLY_HALLMARKS}"
  buh_e
  buh_line "If the command succeeds you have working pull access to the ${RBYC_DEPOT}."
  buh_line "If it fails, re-check the file placement in Step 2."
  buh_e

  # --- Step 5: Next steps ---
  buh_step1 "Next steps"
  buh_e
  buh_line "Return to the start menu:"
  buh_tt   "   " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

######################################################################
# Install Director Credentials

rbho_credential_director() {
  buc_doc_brief "Install director credentials — place RBRA key, validate, confirm build access"
  buc_doc_shown || return 0

  buh_section "Install Director Credentials"
  buh_e
  buh_line "A ${RBYC_DIRECTOR} causes cloud builds and publishes container images to the"
  buh_line "  ${RBYC_DEPOT} — write access to the registry."
  buh_e

  zrbho_credential_install "${RBCC_role_director}"

  # --- Step 4: Confirm live access ---
  buh_step1 "Confirm live access"
  buh_e
  buh_line "Run ${RBYC_REKON} to list raw image tags in the registry"
  buh_line "using your director credential:"
  buh_e
  buh_tt   "   " "${RBZ_REKON_IMAGE}" "" " rbev-busybox"
  buh_e
  buh_line "If the command succeeds you have working build access to the"
  buh_line "  ${RBYC_DEPOT}."
  buh_line "If it fails, re-check the file placement in Step 2."
  buh_e

  # --- Step 5: Next steps ---
  buh_step1 "Next steps"
  buh_e
  buh_line "Return to the start menu:"
  buh_tt   "   " "${RBZ_ONBOARD_START_HERE}"
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

  # Hardcoded for this track — ccyolo is the teaching nameplate
  local -r z_moniker="ccyolo"
  local -r z_sentry_vessel="rbev-sentry-deb-tether"
  local -r z_bottle_vessel="rbev-bottle-ccyolo"
  local -r z_nameplate_file="${RBBC_dot_dir}/${z_moniker}/${RBCC_rbrn_file}"
  local -r z_ssh_tabtarget="tt/rbw-cS.SshTo.${z_moniker}.sh"

  # Inline yelps (not worth kindle constants — track-specific)
  buyy_cmd_yawp "rbw-fk";                local -r z_cmd_rbw_fk="${z_buym_yelp}"
  buyy_cmd_yawp "rbw-cKB";               local -r z_cmd_rbw_cKB="${z_buym_yelp}"
  buyy_cmd_yawp "RBRN_SENTRY_HALLMARK=";  local -r z_cmd_sentry_hallmark="${z_buym_yelp}"
  buyy_cmd_yawp "${z_ssh_tabtarget}";     local -r z_cmd_ssh="${z_buym_yelp}"
  buyy_cmd_yawp "RBRN_SENTRY_HALLMARK";   local -r z_code_sentry_field="${z_buym_yelp}"
  buyy_cmd_yawp "RBRN_BOTTLE_HALLMARK";   local -r z_code_bottle_field="${z_buym_yelp}"
  buyy_cmd_yawp "k260413155458-f9dcb6d9"; local -r z_example_tag="${z_buym_yelp}"

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
  buh_line "A ${RBYC_CRUCIBLE} is a sandboxed container environment with enforced"
  buh_line "network isolation. You are going to build one on your workstation"
  buh_line "and run Claude Code inside it — no cloud account, no credentials"
  buh_line "beyond your own Claude subscription."
  buh_e
  buh_line "This track uses the ${RBYC_CCYOLO} ${RBYC_NAMEPLATE}: a Claude Code sandbox that can"
  buh_line "only reach Anthropic. Everything else is blocked."
  buh_e

  # Docker gate
  if test "${z_has_docker}" = "0"; then
    buh_error "Docker is not available on this machine."
    buh_line  "Install Docker Desktop (or dockerd in WSL) and re-run this handbook."
    buh_e
    buh_tt  "Return to start: " "${RBZ_ONBOARD_START_HERE}"
    buh_e
    return 0
  fi

  buh_line "Prerequisite: a Claude OAuth subscription (you will authenticate"
  buh_line "inside the container via copy/paste from your browser)."
  buh_e

  buh_step_style "Step " " — "

  # =================================================================
  # Step 1: Build images locally
  # =================================================================
  buh_step1 "Build images locally"
  buh_e
  buh_line "A ${RBYC_VESSEL} is a specification for a container image — a Dockerfile"
  buh_line "and build context. A ${RBYC_HALLMARK} is a specific build instance,"
  buh_line "identified by a timestamp tag."
  buh_e
  buh_line "${RBYC_KLUDGE} builds a vessel image locally using Docker — no cloud, no"
  buh_line "registry, no credentials. The fastest path from Dockerfile to running"
  buh_line "container without cloud image backup."
  buh_e

  # --- Substep 1a: Kludge the sentry ---
  buh_step2 "Kludge the sentry"
  buh_e
  buh_line "The ${RBYC_SENTRY} is the gatekeeper container. It runs iptables"
  buh_line "and dnsmasq to enforce network policy — only domains you whitelist"
  buh_line "are reachable from inside."
  buh_e
  buh_line "Kludge it:"
  buh_e
  buh_tt  "   " "${RBZ_KLUDGE_VESSEL}" "" " ${z_sentry_vessel}"
  buh_e
  buh_line "The output prints a hallmark tag — something like"
  buh_line "${z_example_tag}"
  buh_line "Copy it. You will paste it into the nameplate in Step 2."
  buh_e

  # Sentry kludge image probe
  if test "${z_sentry_image_exists}" = "1"; then
    buh_line "${RBYC_PROBE_YES}Kludge-tagged sentry image found locally"
  else
    buh_line "${RBYC_PROBE_NO}No kludge-tagged sentry image found"
  fi
  buh_e

  buh_warn "Kludge requires a clean git tree."
  buh_line "If you have uncommitted changes, commit them first.  This is by"
  buh_line "design — container images not backed by git commits easy to confuse."
  buh_e

  # --- Substep 1b: Kludge the bottle ---
  buh_step2 "Kludge the bottle"
  buh_e
  buh_line "The ${RBYC_BOTTLE} is your workload container — where Claude Code"
  buh_line "runs. The ${RBYC_CCYOLO} bottle includes SSH, node, and the Claude CLI."
  buh_e
  buh_line "This command is different from the sentry kludge. It builds the"
  buh_line "bottle AND drives the ${RBYC_HALLMARK} into the nameplate"
  buh_line "automatically:"
  buh_e
  buh_tt  "   " "${RBZ_CRUCIBLE_KLUDGE_BOTTLE}" "" " ${z_moniker}"
  buh_e
  buh_line "Two kludge commands, two different jobs:"
  buh_line "  ${z_cmd_rbw_fk}  — standalone vessel kludge. Produces a hallmark, touches nothing else."
  buh_line "  ${z_cmd_rbw_cKB} — nameplate-aware bottle kludge. Builds AND installs the hallmark."
  buh_e
  buh_line "The sentry is shared across nameplates, so you kludge it standalone"
  buh_line "and drive the hallmark yourself. The bottle belongs to one nameplate,"
  buh_line "so the tool does the driving for you."
  buh_e

  # Bottle kludge image probe
  if test "${z_bottle_image_exists}" = "1"; then
    buh_line "${RBYC_PROBE_YES}Kludge-tagged bottle image found locally"
  else
    buh_line "${RBYC_PROBE_NO}No kludge-tagged bottle image found"
  fi
  buh_e

  # =================================================================
  # Step 2: Drive sentry hallmark into the nameplate
  # =================================================================
  buh_step1 "Drive sentry hallmark into the nameplate"
  buh_e
  buh_line "A ${RBYC_NAMEPLATE} is the file that defines a Crucible — it specifies"
  buh_line "what ${RBYC_HALLMARK} to use for ${RBYC_SENTRY}, ${RBYC_PENTACLE}, and"
  buh_line "${RBYC_BOTTLE} containers. It lives at:"
  buh_e
  buh_code "   ${z_nameplate_file}"
  buh_e
  buh_line "The ${RBYC_RBRN} file has two hallmark fields — one for the sentry"
  buh_line "and one for the bottle. The bottle kludge (Step 1.2) already drove"
  buh_line "its hallmark. Now you drive the sentry hallmark by hand."
  buh_e
  buh_line "Open the nameplate file and paste the sentry hallmark from Step 1.1"
  buh_line "into the ${z_cmd_sentry_hallmark} field."
  buh_e
  buh_line "This is deliberately manual — it teaches you what ${RBYC_HALLMARKS} are and"
  buh_line "where they live. You just saw the automated version with the ${RBYC_BOTTLE};"
  buh_line "now you see the mechanism underneath."
  buh_e

  # Hallmark probes
  if test "${z_sentry_hallmark_present}" = "1"; then
    buh_line " [*] ${z_code_sentry_field} = ${z_sentry_hallmark}"
  else
    buh_line "${RBYC_PROBE_NO}RBRN_SENTRY_HALLMARK is empty — paste the sentry hallmark from Step 1.1"
  fi
  if test "${z_bottle_hallmark_present}" = "1"; then
    buh_line " [*] ${z_code_bottle_field} = ${z_bottle_hallmark}"
  else
    buh_line "${RBYC_PROBE_NO}RBRN_BOTTLE_HALLMARK is empty — run Step 1.2 (bottle kludge)"
  fi
  buh_e

  buh_line "After editing, commit the change — the next ${RBYC_KLUDGE} will need a"
  buh_line "clean tree:"
  buh_e
  buh_code "   git add ${z_nameplate_file}"
  buh_code "   git commit -m \"Drive sentry hallmark into ${z_moniker} nameplate\""
  buh_e

  # =================================================================
  # Step 3: Charge the crucible
  # =================================================================
  buh_step1 "Charge the crucible"
  buh_e
  buh_line "${RBYC_CHARGE} starts three containers from your nameplate:"
  buh_e
  buh_line "  ${RBYC_SENTRY}   — runs iptables + dnsmasq, enforces the network allowlist"
  buh_line "  ${RBYC_PENTACLE}  — establishes the network namespace shared with the bottle"
  buh_line "  ${RBYC_BOTTLE}    — your workload container (Claude Code lives here)"
  buh_e
  buh_line "The sentry mediates all traffic. The bottle never touches the"
  buh_line "network directly — everything routes through the sentry's rules."
  buh_e
  buh_tt  "   " "${RBZ_CRUCIBLE_CHARGE}" "${z_moniker}"
  buh_e
  buh_line "Charge takes 10-30 seconds. It pulls the images from your local"
  buh_line "Docker, creates the enclave network, starts the containers, waits"
  buh_line "for the sentry to confirm its iptables rules are applied, then"
  buh_line "starts the bottle."
  buh_e

  # Charged probe
  if test "${z_crucible_charged}" = "1"; then
    buh_line "${RBYC_PROBE_YES}Crucible ${z_moniker} is charged (bottle container running)"
  else
    buh_line "${RBYC_PROBE_NO}Crucible ${z_moniker} is not charged"
  fi
  buh_e

  # =================================================================
  # Step 4: Enter the container and run Claude Code
  # =================================================================
  buh_step1 "Enter the container and run Claude Code"
  buh_e
  buh_line "SSH into the bottle:"
  buh_e
  buh_line "   ${z_cmd_ssh}"
  buh_e
  buh_line "You land as the claude user in ~/workspace, which contains"
  buh_line "a small sample project. Run Claude Code:"
  buh_e
  buh_code "   claude"
  buh_e
  buh_line "Claude Code will prompt you to authenticate. It opens a URL —"
  buh_line "copy it to your workstation browser, sign in with your Claude"
  buh_line "subscription, and paste the code back into the terminal."
  buh_e
  buh_warn "The ccyolo bottle pins Claude Code to a specific version."
  buh_line "Versions after v2.1.89 have a regression where paste does not"
  buh_line "work in the OAuth input prompt through SSH or docker exec"
  buh_line "(github.com/anthropics/claude-code/issues/47745). If you"
  buh_line "update the pin and paste stops working, type the code manually."
  buh_e
  buh_line "Once authenticated, Claude Code starts in full autonomy mode —"
  buh_line "no permission prompts. Inside a network-contained crucible,"
  buh_line "this is the correct posture: the sentry enforces the real"
  buh_line "security boundary, not the tool permission system."
  buh_e
  buh_line "Try your first interaction:"
  buh_e
  buh_code "   The count_words.sh script has bugs — can you find and fix them?"
  buh_e
  buh_line "Watch Claude read the files, identify issues, and edit the code."
  buh_line "The workspace persists across charge/quench cycles — your changes"
  buh_line "survive restarts."
  buh_e
  buh_line "Why SSH instead of docker exec?"
  buh_e
  buh_line "Docker exec is laggy and breaks terminal resize — Claude Code's"
  buh_line "interactive display needs correct dimensions. SSH gives a proper"
  buh_line "login session with full terminal negotiation."
  buh_e
  buh_line "If SSH fails, ${RBYC_RACK} is the diagnostic fallback —"
  buh_line "docker exec into the bottle to inspect state:"
  buh_e
  buh_tt  "   " "${RBZ_CRUCIBLE_RACK}" "" " ${z_moniker}"
  buh_e

  # =================================================================
  # Step 5: Verify network containment
  # =================================================================
  buh_step1 "Verify network containment"
  buh_e
  buh_line "From inside the ${RBYC_BOTTLE} (while SSH'd in), you can test what's reachable."
  buh_line "The ${RBYC_CCYOLO} nameplate allows Anthropic and example.com (a test"
  buh_line "target). Everything else is blocked."
  buh_e
  buh_line "Run these curl commands inside the bottle:"
  buh_e
  buh_code "   curl -s -o /dev/null -w '%{http_code}' https://api.anthropic.com"
  buh_line "   Expected: 404 (API wants auth, not bare GET)"
  buh_e
  buh_code "   curl -s -o /dev/null -w '%{http_code}' https://claude.ai"
  buh_line "   Expected: 403 (web app)"
  buh_e
  buh_code "   curl -s -o /dev/null -w '%{http_code}' https://example.com"
  buh_line "   Expected: 200 (test target on allowlist)"
  buh_e
  buh_code "   curl -s -o /dev/null -w '%{http_code}' --max-time 5 https://google.com"
  buh_line "   Expected: 000 or timeout (blocked)"
  buh_e
  buh_code "   curl -s -o /dev/null -w '%{http_code}' --max-time 5 https://registry.npmjs.org"
  buh_line "   Expected: 000 or timeout (blocked)"
  buh_e
  buh_line "The ${RBYC_SENTRY} enforces this with two layers:"
  buh_line "dnsmasq resolves only whitelisted domains; iptables drops"
  buh_line "packets to any IP not in the CIDR allowlist. Both layers"
  buh_line "must agree for traffic to pass."
  buh_e
  buh_line "example.com is included in the ${RBYC_CCYOLO} ${RBYC_NAMEPLATE} specifically"
  buh_line "for this verification step — it proves the allowlist works"
  buh_line "for a non-Anthropic domain."
  buh_e

  # =================================================================
  # Step 6: Quench the crucible
  # =================================================================
  buh_step1 "Quench the crucible"
  buh_e
  buh_line "${RBYC_QUENCH} stops and removes all three containers and the"
  buh_line "enclave network:"
  buh_e
  buh_tt  "   " "${RBZ_CRUCIBLE_QUENCH}" "${z_moniker}"
  buh_e
  buh_line "Clean shutdown. The images stay cached locally — your next charge"
  buh_line "reuses them instantly."
  buh_e

  # =================================================================
  # Iteration loop
  # =================================================================
  buh_section "The iteration loop"
  buh_e
  buh_line "You now have the full local cycle:"
  buh_e
  buh_line "  1. Edit the Dockerfile or entrypoint"
  buh_line "  2. Commit your changes (${RBYC_KLUDGE} needs a clean tree)"
  buh_line "  3. ${RBYC_KLUDGE} the ${RBYC_BOTTLE}:"
  buh_tt    "       " "${RBZ_CRUCIBLE_KLUDGE_BOTTLE}" "" " ${z_moniker}"
  buh_line  "  4. Commit the ${RBYC_HALLMARK} change"
  buh_line  "  5. ${RBYC_CHARGE}: Stop and restart"
  buh_tt    "       " "${RBZ_CRUCIBLE_CHARGE}" "${z_moniker}"
  buh_line  "  6. SSH in:"
  buh_line  "       ${z_cmd_ssh}"
  buh_line  "  7. Test your changes"
  buh_e
  buh_line "${RBYC_CHARGE} tears down any prior state before starting, so you"
  buh_line "don't need to Quench between iterations — just Charge again."
  buh_e
  buh_line "Steps 3-7 take under a minute. The ${RBYC_SENTRY} rarely changes, so"
  buh_line "you almost never re-kludge it — the ${RBYC_BOTTLE} is your iteration"
  buh_line "target."
  buh_e

  # --- Return to start ---
  buh_tt  "Return to start: " "${RBZ_ONBOARD_START_HERE}"
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

######################################################################
# Payor handbook — establish a Manor and provision the Depot
#
# Linear step sequence, no conditional probes. The payor owns the GCP
# project and funds it. This handbook walks through the full ceremony:
# OAuth credentials, project setup, depot provisioning, governor handoff.

rbho_payor_handbook() {
  buc_doc_brief "Payor — establish a Manor and provision the Depot"
  buc_doc_shown || return 0

  # --- Header ---
  buh_section "Payor — Establish a Manor and Provision the Depot"
  buh_e
  buh_line "The ${RBYC_PAYOR} establishes a ${RBYC_MANOR} — an administrative seat"
  buh_line "holding the billing account, OAuth client, and operator identity."
  buh_line "Unlike other roles that use service account keys, the ${RBYC_PAYOR}"
  buh_line "authenticates via OAuth — representing the human project owner."
  buh_e
  buh_line "By the end of this handbook you will have a ${RBYC_MANOR}, a ${RBYC_DEPOT}"
  buh_line "funded under it, and a ${RBYC_GOVERNOR} service account ready to administer it."
  buh_e

  buh_line "This ceremony takes about 15 minutes."
  buh_e

  buh_step_style "Step " " — "

  # =================================================================
  # Step 1: Establish the Manor
  # =================================================================
  buh_step1 "Establish the Manor"
  buh_e
  buh_line "The ${RBYC_MANORS} GCP project hosts the OAuth client and billing"
  buh_line "account. It must be created before any infrastructure can be"
  buh_line "provisioned."
  buh_e
  buh_line "Run the guided setup:"
  buh_tt  "  " "${RBZ_PAYOR_ESTABLISH}"
  buh_e
  buh_line "This guides you through creating the ${RBYC_MANORS} GCP project,"
  buh_line "enabling billing, and configuring the OAuth consent screen. The ${RBYC_MANOR}"
  buh_line "identity is recorded in ${RBYC_RBRP}."
  buh_e

  # =================================================================
  # Step 2: Install OAuth credentials
  # =================================================================
  buh_step1 "Install OAuth credentials"
  buh_e
  buh_line "Step 1 ended with downloading a JSON client secret file from the"
  buh_line "OAuth client you just created. Install it:"
  buh_e
  buh_tt  "  " "${RBZ_PAYOR_INSTALL}" "" " \${HOME}/Downloads/client_secret_*.json"
  buh_e
  buh_line "This walks you through the OAuth authorization flow and stores"
  buh_line "the credential securely."
  buh_e
  buh_line "If you are refreshing an existing credential that has expired:"
  buh_tt  "  " "${RBZ_PAYOR_REFRESH}"
  buh_e

  # =================================================================
  # Step 3: Provision the Depot
  # =================================================================
  buh_step1 "Provision the Depot"
  buh_e
  buh_line "A ${RBYC_DEPOT} is the facility where container images are built and"
  buh_line "stored — a GCP project with a container repository, storage bucket,"
  buh_line "and build infrastructure, funded under the ${RBYC_MANORS} billing account."
  buh_line "A ${RBYC_GOVERNOR} administers the ${RBYC_DEPOT} — creating"
  buh_line "${RBYC_RETRIEVER} and ${RBYC_DIRECTOR} accounts for those who build and"
  buh_line "retrieve container images."
  buh_e
  buh_line "${RBYC_PAYOR} creates the Depot:"
  buh_tt  "  " "${RBZ_LEVY_DEPOT}"
  buh_e
  buh_line "This enables APIs, creates the Artifact Registry repository and"
  buh_line "Cloud Storage bucket, and configures Cloud Build."
  buh_e
  buh_line "${RBYC_PAYOR} can list Depots for verification:"
  buh_tt  "  " "${RBZ_LIST_DEPOT}"
  buh_e
  buh_line "${RBYC_PAYOR} creates the Governor service account:"
  buh_tt  "  " "${RBZ_MANTLE_GOVERNOR}"
  buh_e
  buh_line "Hand the resulting key file to the person who will administer"
  buh_line "this ${RBYC_DEPOT}. After this handoff, the ${RBYC_GOVERNOR} can create"
  buh_line "${RBYC_RETRIEVER} and ${RBYC_DIRECTOR} accounts independently."
  buh_e
  buh_line "The ${RBYC_PAYORS} job for this ${RBYC_DEPOT} is done unless billing or"
  buh_line "project-level changes are needed."
  buh_e

  # --- Return to start ---
  buh_tt  "Return to start: " "${RBZ_ONBOARD_START_HERE}"
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

  # --- Header ---
  buh_section "Governor — Administer Service Accounts"
  buh_e
  buh_line "A ${RBYC_GOVERNOR} administers a ${RBYC_DEPOT} — creating service accounts"
  buh_line "and managing access for those who build and run container images."
  buh_e

  buh_step_style "Step " " — "

  # =================================================================
  # Step 1: Install governor credentials
  # =================================================================
  buh_step1 "Install governor credentials"
  buh_e
  buh_line "The ${RBYC_GOVERNOR} works within a ${RBYC_DEPOT} provisioned under the"
  buh_line "${RBYC_PAYORS} ${RBYC_MANOR}. Your Payor creates your credential by running:"
  buh_tt  "  " "${RBZ_MANTLE_GOVERNOR}"
  buh_e
  buh_line "If no ${RBYC_DEPOT} exists yet, the ${RBYC_PAYOR} establishes one first:"
  buh_tt  "  " "${RBZ_ONBOARD_PAYOR_HB}"
  buh_e
  buh_line "Install the resulting key file into the secrets directory under"
  buh_line "the governor role subdirectory. The path is derived from your"
  buh_line "${RBYC_RBRR} configuration — check RBRR_SECRETS_DIR for the location."
  buh_e

  # =================================================================
  # Step 2: Provision downstream service accounts
  # =================================================================
  buh_step1 "Provision downstream service accounts"
  buh_e
  buh_line "The governor provisions access for two downstream roles:"
  buh_e
  buh_line "A ${RBYC_RETRIEVER} has read access to the ${RBYC_DEPOT} — they pull and run"
  buh_line "container images that others have built."
  buh_line "A ${RBYC_DIRECTOR} has build and publish access — they create container"
  buh_line "images and push them to the registry."
  buh_e
  buh_line "Create a ${RBYC_RETRIEVER} with read access (${RBYC_CHARTER}):"
  buh_tt  "  " "${RBZ_CHARTER_RETRIEVER}"
  buh_e
  buh_line "Create a ${RBYC_DIRECTOR} with build access (${RBYC_KNIGHT}):"
  buh_tt  "  " "${RBZ_KNIGHT_DIRECTOR}"
  buh_e
  buh_line "Each command creates the service account and applies the IAM"
  buh_line "grants it needs. The output is an ${RBYC_RBRA} key file — hand it to"
  buh_line "the Retriever or Director user."
  buh_e
  buh_line "List issued service accounts:"
  buh_tt  "  " "${RBZ_LIST_SERVICE_ACCOUNTS}"
  buh_e

  # =================================================================
  # Step 3: Verify the chain
  # =================================================================
  buh_step1 "Verify the chain"
  buh_e
  buh_line "The service accounts you created include IAM grants — each SA"
  buh_line "gets exactly the permissions its role requires, no more."
  buh_line "${RBYC_RETRIEVER} gets read access."
  buh_line "${RBYC_DIRECTOR} gets read, write, and build trigger access."
  buh_e
  buh_line "Verify the complete chain works by installing both credentials"
  buh_line "locally and running the credential handbook tracks:"
  buh_tt  "  " "${RBZ_ONBOARD_CRED_RETRIEVER}"
  buh_tt  "  " "${RBZ_ONBOARD_CRED_DIRECTOR}"
  buh_e
  buh_line "If the ${RBYC_RETRIEVER} can pull from the ${RBYC_DEPOT} and the"
  buh_line "${RBYC_DIRECTOR} can see the registry, your grants are correct."
  buh_e

  # --- Return to start ---
  buh_tt  "Return to start: " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

# eof
