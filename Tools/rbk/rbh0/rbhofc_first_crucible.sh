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
# Recipe Bottle Handbook Onboarding - First Crucible (local builds)

set -euo pipefail

test -z "${ZRBHOFC_SOURCED:-}" || return 0
ZRBHOFC_SOURCED=1

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

# eof
