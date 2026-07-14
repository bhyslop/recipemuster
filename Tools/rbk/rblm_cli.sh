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
# RBLM CLI - Lifecycle Marshal operations (zero, lustrate, feign, expede)

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Expede constants

# The candidate clone's directory name beneath the operator's target dir.
RBLM_candidate_subdir="candidate"

# The subject of the single commit the candidate carries. One commit, so one
# subject: it names the act, not the contents.
RBLM_candidate_subject="Recipe Bottle release candidate"

# The sterilize script, repo-relative. Expede runs THE CANDIDATE'S copy of this
# path, never its own — so the path is named once, here, and the perambulation is
# what guarantees the candidate has it.
RBLM_sterilize_path="Tools/rbk/rblm_sterilize.sh"

######################################################################
# Command: zero - Zero regime to blank template for release

rblm_zero() {
  buc_doc_brief "Zero regime to blank template for release qualification"
  buc_doc_param "tree" "Basename of the repository root this call intends to blank"
  buc_doc_shown || return 0

  local -r z_rbrr="${RBCC_rbrr_file}"
  local -r z_rbrd="${RBCC_rbrd_file}"
  test -f "${z_rbrr}" || buc_die "RBRR file not found: ${z_rbrr}"
  test -f "${z_rbrd}" || buc_die "RBRD file not found: ${z_rbrd}"

  mkdir -p "${BURD_TEMP_DIR}" || buc_die "Failed to create temp directory"

  # Tree-identity gate. Zero blanks the regime of whatever tree it runs in, so
  # the caller must NAME that tree and be right. Git attests the identity; the
  # caller's claim is checked against it. This dies before buc_require, so
  # BURE_CONFIRM=skip cannot reach past it — the guard is the naming, never the
  # prompt.
  local -r z_claimed_tree="${BUZ_FOLIO:-}"
  test -n "${z_claimed_tree}" \
    || buc_die "Marshal zero requires the intended tree's basename as its argument — it blanks the regime of the tree it runs in, so that tree must be named, not assumed"

  local -r z_toplevel_temp="${BURD_TEMP_DIR}/rblm_zero_toplevel.txt"
  git rev-parse --show-toplevel > "${z_toplevel_temp}" || buc_die "git rev-parse --show-toplevel failed — marshal zero must run inside a git repository"
  local z_toplevel=$(<"${z_toplevel_temp}")
  local -r z_actual_tree="${z_toplevel##*/}"
  test "${z_claimed_tree}" = "${z_actual_tree}" \
    || buc_die "Marshal zero refuses: caller named tree '${z_claimed_tree}', but this repository root is '${z_actual_tree}' (${z_toplevel})"

  # Pre-checks: working tree clean and HEAD pushed.
  # Marshal-zero auto-commits its mutations; both invariants must hold to keep
  # the resulting commit purely the marshal-zero state.
  local -r z_status_temp="${BURD_TEMP_DIR}/rblm_zero_status.txt"
  local -r z_upstream_temp="${BURD_TEMP_DIR}/rblm_zero_upstream.txt"
  local -r z_unpushed_temp="${BURD_TEMP_DIR}/rblm_zero_unpushed.txt"

  git status --porcelain > "${z_status_temp}" || buc_die "git status failed"
  test ! -s "${z_status_temp}" || buc_die "Working tree not clean — commit or discard before marshal-zero. See: ${z_status_temp}"

  if ! git rev-parse --abbrev-ref --symbolic-full-name '@{u}' > "${z_upstream_temp}" 2>&1; then
    buc_die "Current branch has no upstream — set tracking and push before marshal-zero. See: ${z_upstream_temp}"
  fi

  git rev-list '@{u}..HEAD' > "${z_unpushed_temp}" || buc_die "git rev-list failed"
  test ! -s "${z_unpushed_temp}" || buc_die "HEAD has unpushed commits — push before marshal-zero. See: ${z_unpushed_temp}"

  # Shellcheck gate: the codebase must be lint-clean before marshal-zero mints
  # the pristine baseline commit, so gauntlet entry-state cannot be shellcheck-
  # dirty by construction. Runs the release-tier shellcheck via the sibling
  # rbq_cli; any finding aborts here, before any mutation.
  local -r z_rbk_dir="${BASH_SOURCE[0]%/*}"
  "${z_rbk_dir}/rbq_cli.sh" rbq_qualify_shellcheck || buc_die "Shellcheck findings present — fix before marshal-zero; the pristine baseline must be lint-clean"

  # Colophon-completeness gate: every enrolled colophon (RBW + BUW) must have a
  # tabtarget on disk before marshal-zero mints the pristine baseline. rbw-MZ is
  # withheld from delivery, so this is the source-only completeness proof — a
  # stripped consumer never has this tabtarget and never runs it.
  "${z_rbk_dir}/rbq_cli.sh" rbq_qualify_completeness || buc_die "Enrolled colophon without a tabtarget — the source tabtarget set must be complete before marshal-zero"

  # Discover secrets dir and vessel dir for pre-confirmation inventory
  local z_secrets_dir=""
  local z_vessel_dir=""
  local z_secrets_line=""
  while IFS= read -r z_secrets_line || test -n "${z_secrets_line}"; do
    case "${z_secrets_line}" in
      RBRR_SECRETS_DIR=*) z_secrets_dir="${z_secrets_line#RBRR_SECRETS_DIR=}" ;;
      RBRR_VESSEL_DIR=*)  z_vessel_dir="${z_secrets_line#RBRR_VESSEL_DIR=}"  ;;
    esac
  done < "${z_rbrr}"

  buh_section "Marshal Zero"
  buh_line "  Tree:    ${z_actual_tree} (${z_toplevel})"
  buh_line "  Targets: ${z_rbrr}"
  buh_line "           ${z_rbrd}"
  buh_e
  buh_line "  RBRR fields blanked (zeroed to onboarding start):"
  buh_line "    RBRR_RUNTIME_PREFIX"
  buh_e
  buh_line "  RBRR fields pre-filled to defaults:"
  buh_line "    RBRR_DNS_SERVER, RBRR_GCB_TIMEOUT,"
  buh_line "    RBRR_GCB_MIN_CONCURRENT_BUILDS,"
  buh_line "    RBRR_VESSEL_DIR, RBRR_SECRETS_DIR"
  buh_e
  buh_line "  RBRD fields blanked (depot identity — operator decision):"
  buh_line "    RBRD_CLOUD_PREFIX, RBRD_DEPOT_MONIKER"
  buh_e
  buh_line "  RBRD fields pre-filled to defaults:"
  buh_line "    RBRD_GCP_REGION, RBRD_GCB_MACHINE_TYPE"
  buh_e
  buh_line "  Vessel hallmarks BLANKED (stale after depot change):"
  local z_np_preview=""
  local z_any_np=0
  for z_np_preview in "${RBCC_moorings_dir}"/*/"${RBCC_rbrn_file}"; do
    test -f "${z_np_preview}" || continue
    buh_line "    ${z_np_preview}"
    z_any_np=1
  done
  test "${z_any_np}" = "1" || buh_line "    (no nameplates found)"
  buh_e
  buh_line "  Vessel regime fields BLANKED (depot-scoped, stale after depot change):"
  buh_line "    RBRV_RELIQUARY, RBRV_IMAGE_*_ANCHOR in all rbrv.env"
  if test -n "${z_vessel_dir}" && test -d "${z_vessel_dir}"; then
    local z_vr_preview=""
    local z_any_vr=0
    for z_vr_preview in "${z_vessel_dir}"/*/"${RBCC_rbrv_file}"; do
      test -f "${z_vr_preview}" || continue
      buh_line "    ${z_vr_preview}"
      z_any_vr=1
    done
    test "${z_any_vr}" = "1" || buh_line "    (no vessel regimes found)"
  else
    buh_line "    (vessel dir not configured or missing)"
  fi
  buh_e
  buh_line "  Preserved (payor-scoped, survives depot change):"
  buh_line "    ${z_secrets_dir}/${RBCC_account_unhewn_payor}/${RBCC_rbro_file}"
  buh_e
  buh_line "  On completion, marshal-zero auto-commits the in-tree mutations."
  buh_e
  buc_require "Proceed with marshal zero?" "zero"

  local -r z_tmp="${z_rbrr}.tmp"
  local z_line=""
  while IFS= read -r z_line; do
    case "${z_line}" in
      # Pre-selected defaults
      RBRR_DNS_SERVER=*)                    printf '%s\n' "RBRR_DNS_SERVER=8.8.8.8"                     ;;
      RBRR_GCB_TIMEOUT=*)                   printf '%s\n' "RBRR_GCB_TIMEOUT=2700s"                      ;;
      RBRR_GCB_MIN_CONCURRENT_BUILDS=*)     printf '%s\n' "RBRR_GCB_MIN_CONCURRENT_BUILDS=3"            ;;
      RBRR_VESSEL_DIR=*)                    printf '%s\n' "RBRR_VESSEL_DIR=${RBCC_moorings_dir}/${RBCC_vessels_subdir}" ;;
      RBRR_SECRETS_DIR=*)                   printf '%s\n' "RBRR_SECRETS_DIR=../station-files/secrets"   ;;
      # Site-specific fields blanked
      RBRR_RUNTIME_PREFIX=*)                printf '%s\n' "RBRR_RUNTIME_PREFIX="                        ;;
      # Everything else passes through (comments, shebang, blanks)
      *)                                    printf '%s\n' "${z_line}"                                   ;;
    esac
  done < "${z_rbrr}" > "${z_tmp}" && mv "${z_tmp}" "${z_rbrr}"

  # Apply equivalent transform to rbrd.env.
  local -r z_rbrd_tmp="${z_rbrd}.tmp"
  while IFS= read -r z_line; do
    case "${z_line}" in
      # Pre-selected defaults
      RBRD_GCP_REGION=*)                    printf '%s\n' "RBRD_GCP_REGION=us-central1"                 ;;
      RBRD_GCB_MACHINE_TYPE=*)              printf '%s\n' "RBRD_GCB_MACHINE_TYPE=e2-standard-2"         ;;
      # Site-specific fields blanked
      RBRD_CLOUD_PREFIX=*)                  printf '%s\n' "RBRD_CLOUD_PREFIX="                          ;;
      RBRD_DEPOT_MONIKER=*)                 printf '%s\n' "RBRD_DEPOT_MONIKER="                         ;;
      # Everything else passes through (comments, shebang, blanks)
      *)                                    printf '%s\n' "${z_line}"                                   ;;
    esac
  done < "${z_rbrd}" > "${z_rbrd_tmp}" && mv "${z_rbrd_tmp}" "${z_rbrd}"

  # Blank hallmark values in all vessel nameplates.
  # Hallmarks reference images built against the prior depot — they
  # become stale after reset.  Blanking them causes the onboarding guide
  # to require conjure & vouch before declaring setup complete.
  local z_np=""
  local z_np_tmp=""
  for z_np in "${RBCC_moorings_dir}"/*/"${RBCC_rbrn_file}"; do
    test -f "${z_np}" || continue
    z_np_tmp="${z_np}.tmp"
    while IFS= read -r z_line; do
      case "${z_line}" in
        RBRN_SENTRY_HALLMARK=*)  printf '%s\n' "RBRN_SENTRY_HALLMARK=" ;;
        RBRN_BOTTLE_HALLMARK=*)  printf '%s\n' "RBRN_BOTTLE_HALLMARK=" ;;
        *)                           printf '%s\n' "${z_line}"                  ;;
      esac
    done < "${z_np}" > "${z_np_tmp}" && mv "${z_np_tmp}" "${z_np}"
    buh_line "  Blanked hallmarks: ${z_np}"
  done

  # Blank depot-scoped fields in all vessel regime files.
  # RBRV_RELIQUARY references a reliquary-kind Lode in the prior depot's GAR.
  # RBRV_IMAGE_*_ANCHOR references bole Lodes in the prior depot's GAR.
  # Both become stale after depot change — onboarding requires conclave + ensconce.
  if test -n "${z_vessel_dir}" && test -d "${z_vessel_dir}"; then
    local z_vr=""
    local z_vr_tmp=""
    for z_vr in "${z_vessel_dir}"/*/"${RBCC_rbrv_file}"; do
      test -f "${z_vr}" || continue
      z_vr_tmp="${z_vr}.tmp"
      while IFS= read -r z_line; do
        case "${z_line}" in
          RBRV_RELIQUARY=*)       printf '%s\n' "RBRV_RELIQUARY="       ;;
          RBRV_IMAGE_*_ANCHOR=*)  printf '%s\n' "${z_line%%=*}="        ;;
          *)                      printf '%s\n' "${z_line}"             ;;
        esac
      done < "${z_vr}" > "${z_vr_tmp}" && mv "${z_vr_tmp}" "${z_vr}"
      buh_line "  Blanked depot-scoped fields: ${z_vr}"
    done
  fi

  buh_line "  Zero complete: ${z_rbrr}"
  buh_line "                 ${z_rbrd}"
  buh_e

  # Auto-commit the in-tree mutations so post-zero state is captured as a single
  # commit.
  buc_step "Committing marshal-zero state"
  git add "${z_rbrr}" || buc_die "Failed to stage RBRR file"
  git add "${z_rbrd}" || buc_die "Failed to stage RBRD file"

  local z_stage=""
  for z_stage in "${RBCC_moorings_dir}"/*/"${RBCC_rbrn_file}"; do
    test -f "${z_stage}" || continue
    git add "${z_stage}" || buc_die "Failed to stage: ${z_stage}"
  done
  if test -n "${z_vessel_dir}" && test -d "${z_vessel_dir}"; then
    for z_stage in "${z_vessel_dir}"/*/"${RBCC_rbrv_file}"; do
      test -f "${z_stage}" || continue
      git add "${z_stage}" || buc_die "Failed to stage: ${z_stage}"
    done
  fi

  if git diff --cached --quiet; then
    buh_line "  No changes to commit — already in marshal-zero state"
  else
    git commit -m "Marshal Zero — release qualification reset" || buc_die "Marshal-zero commit failed"
    buh_line "  Marshal-zero state committed"
  fi
  buh_e

  buh_line "  Next: verify onboarding guide detects blank state:"
  buc_tabtarget "${RBZ_ONBOARD_START_HERE}"
  buc_success "Regime zeroed to blank template"
}

######################################################################
# Command: lustrate - Erase site identity from the release candidate's clone

rblm_lustrate() {
  buc_doc_brief "Lustrate the release clone - erase site identity from every proscribed home"
  buc_doc_shown || return 0

  # Clean-tree gate. Lustration rewrites tracked config and auto-commits the
  # result, so anything already dirty would ride into that commit. Deliberately
  # WITHOUT marshal-zero's pushed-state gate: lustration runs immediately after
  # marshal zero, whose own auto-commit leaves HEAD unpushed by construction.
  mkdir -p "${BURD_TEMP_DIR}" || buc_die "Failed to create temp directory"
  local -r z_status_temp="${BURD_TEMP_DIR}/rblm_lustrate_status.txt"
  git status --porcelain > "${z_status_temp}" || buc_die "git status failed"
  test ! -s "${z_status_temp}" || buc_die "Working tree not clean — commit or discard before lustration. See: ${z_status_temp}"

  buh_section "Marshal Lustrate"
  buh_line "  Erases every site-scoped home named by the proscription"
  buh_line "  (Tools/rbk/rblm_lustrate.sh) — the payor and workforce regimes,"
  buh_line "  the federation regimes, the depot-scoped vessel and nameplate"
  buh_line "  fields, and the freehold subject in source."
  buh_e
  buh_line "  Run this ONLY in the release ceremony's throwaway clone."
  buh_line "  Against a working station it erases the live configuration."
  buh_e
  buh_line "  Proof of erasure is the damnatio fixture, run post-strip:"
  buc_tabtarget "${RBZ_THEURGE_FIXTURE}" "damnatio"
  buh_e
  buc_require "Proceed with lustration?" "lustrate"

  rblm_lustrate_apply
  buh_e

  buc_step "Committing lustrated state"
  git add --update || buc_die "Failed to stage lustrated files"

  if git diff --cached --quiet; then
    buh_line "  No changes to commit — already lustrated"
  else
    git commit -m "Marshal Lustrate — site identity erased for release" || buc_die "Lustration commit failed"
    buh_line "  Lustrated state committed"
  fi
  buh_e

  buc_success "Site identity erased from every proscribed home"
}

######################################################################
# Command: feign - Invent a false station so the candidate can validate

rblm_feign() {
  buc_doc_brief "Feign a station on the probe branch - write shape-valid stand-ins over the lustrated site fields"
  buc_doc_shown || return 0

  # Branch guard. Feigning writes false-but-valid identity into the regime tree.
  # On the candidate branch that would be a catastrophe wearing a valid shape, so
  # the two branches the value must never reach are refused by name: the clone's
  # main (which lustration just sterilized) and any candidate branch (which is
  # what gets pushed). The probe branch is a throwaway cut from the candidate and
  # named nothing in particular, so it is what remains.
  mkdir -p "${BURD_TEMP_DIR}" || buc_die "Failed to create temp directory"
  local -r z_branch_temp="${BURD_TEMP_DIR}/rblm_feign_branch.txt"
  git rev-parse --abbrev-ref HEAD > "${z_branch_temp}" || buc_die "git rev-parse failed"
  local -r z_branch=$(<"${z_branch_temp}")

  case "${z_branch}" in
    main|candidate-*)
      buc_die "Refusing to feign on '${z_branch}' — feigned values must never ride a branch that ships. Cut a throwaway probe branch from the candidate first."
      ;;
  esac

  # Clean-tree gate. Feigning auto-commits, and the commit must carry the seed
  # alone — the probe branch exists to be the candidate plus exactly this.
  local -r z_status_temp="${BURD_TEMP_DIR}/rblm_feign_status.txt"
  git status --porcelain > "${z_status_temp}" || buc_die "git status failed"
  test ! -s "${z_status_temp}" || buc_die "Working tree not clean — commit or discard before feigning. See: ${z_status_temp}"

  buh_section "Marshal Feign"
  buh_line "  Branch: ${z_branch}"
  buh_e
  buh_line "  Writes a shape-valid stand-in over every site-scoped field the"
  buh_line "  proscription (Tools/rbk/rblm_lustrate.sh) carries a feigned value"
  buh_line "  for. A lustrated tree is correctly sterile and therefore cannot"
  buh_line "  validate; this invents a false station so the candidate can run the"
  buh_line "  consumer's own reveille from the consumer's own seat."
  buh_e
  buh_line "  Every value is visibly false. None is borrowed from a live station."
  buh_e
  buh_line "  THIS BRANCH IS A THROWAWAY. It is never pushed, never merged, and"
  buh_line "  never becomes the candidate. Delete it when the probe is read."
  buh_e
  buc_require "Proceed with feigning?" "feign"

  rblm_feign_apply
  buh_e

  buc_step "Committing feigned state"
  git add --update || buc_die "Failed to stage feigned files"

  if git diff --cached --quiet; then
    buh_line "  No changes to commit — already feigned"
  else
    git commit -m "Marshal Feign — false station for the consumer-seat probe (throwaway)" || buc_die "Feign commit failed"
    buh_line "  Feigned state committed"
  fi
  buh_e

  buh_line "  Next: run the consumer's credless suite against this tree:"
  buc_tabtarget "${RBZ_THEURGE_SUITE}" "reveille"
  buc_success "Station feigned — the candidate can now validate"
}

######################################################################
# Command: expede - Cut the delivery candidate by ADDITION into a public clone
#
# To expede is to issue an instrument from the chancery once due process is done.
# That is the act: gate, copy the shipped bytes out of the committed record, dress
# them, and issue exactly one commit.
#
# THE CANDIDATE IS BUILT BY ADDITION, and everything else here follows from it.
# The clone is of the PUBLIC repository, so the object graph the push walks began
# with no private object in it and never receives one. Nothing is stripped, because
# nothing withheld is ever put in. The 2026-07-13 candidate was built the other way
# — the whole repository, then removals — and its TIP was spotless: every strip had
# landed, and every assay this project owned read the tip and passed it. It went to
# the remote at 292 MiB because the strip cleans the face while the push sends
# everything reachable from the branch. Construction is the prevention. The sweep
# below is not the guard; it is the proof that the guard held.
rblm_expede() {
  buc_doc_brief "Expede the delivery candidate - build it by addition in a clone of the public repository"
  buc_doc_param "target_dir" "Absolute path to target directory (must not exist)"
  buc_doc_shown || return 0

  local -r z_target_dir="${BUZ_FOLIO:-}"
  test -n "${z_target_dir}" || buc_die "Target directory path is required"

  case "${z_target_dir}" in
    /*) ;;
    *)  buc_die "Target directory must be an absolute path: ${z_target_dir}" ;;
  esac
  test ! -e "${z_target_dir}" || buc_die "Target directory already exists: ${z_target_dir}"

  mkdir -p "${BURD_TEMP_DIR}" || buc_die "Failed to create temp directory"

  # Clean-tree gate. Every shipped byte is taken from the COMMITTED record, so an
  # uncommitted edit would be silently absent from the candidate — the tree the
  # operator is looking at would not be the tree that shipped.
  local -r z_status_temp="${BURD_TEMP_DIR}/rblm_expede_status.txt"
  git status --porcelain > "${z_status_temp}" || buc_die "git status failed"
  test ! -s "${z_status_temp}" || buc_die "Working tree not clean — commit before expede; the candidate is cut from committed bytes. See: ${z_status_temp}"

  # The perambulation must be TOTAL before anything is cut from it. An unjudged
  # path is not a warning to be carried forward: it means the project has not
  # ruled on a file it tracks, and a candidate cut in that state would silently
  # ship it or silently drop it. Red until judged, and expede is where that bites.
  local -r z_unjudged_temp="${BURD_TEMP_DIR}/rblm_expede_unjudged.txt"
  local -r z_dead_temp="${BURD_TEMP_DIR}/rblm_expede_dead.txt"

  rblm_emit_verdicts > "${BURD_TEMP_DIR}/rblm_expede_verdicts.txt" || buc_die "Failed to judge the tracked tree"
  grep '^unjudged' "${BURD_TEMP_DIR}/rblm_expede_verdicts.txt" > "${z_unjudged_temp}" || true
  test ! -s "${z_unjudged_temp}" || buc_die "Tracked paths the perambulation has not judged — rule them ship or withhold in Tools/rbk/rblm_perambulation.sh. See: ${z_unjudged_temp}"

  rblm_emit_dead_rows > "${z_dead_temp}" || buc_die "Failed to read the perambulation's dead rows"
  test ! -s "${z_dead_temp}" || buc_die "Perambulation rows that judge no tracked path — stale or shadowed. See: ${z_dead_temp}"

  # The base. A candidate is one commit atop the PUBLIC main, so the public remote
  # is not optional scenery — it is the thing being added to.
  local -r z_upstream_temp="${BURD_TEMP_DIR}/rblm_expede_upstream.txt"
  git remote get-url OPEN_SOURCE_UPSTREAM > "${z_upstream_temp}" 2>/dev/null \
    || buc_die "OPEN_SOURCE_UPSTREAM is not configured — the candidate is built by addition INTO the public repository, so there is nothing to add to"
  local -r z_upstream_url=$(<"${z_upstream_temp}")
  test -n "${z_upstream_url}" || buc_die "OPEN_SOURCE_UPSTREAM URL is empty"

  local -r z_head_temp="${BURD_TEMP_DIR}/rblm_expede_head.txt"
  git rev-parse HEAD > "${z_head_temp}" || buc_die "git rev-parse HEAD failed"
  local -r z_head=$(<"${z_head_temp}")

  local -r z_shipped_temp="${BURD_TEMP_DIR}/rblm_expede_shipped.txt"
  rblm_emit_shipped > "${z_shipped_temp}" || buc_die "Failed to enumerate the shipped paths"
  test -s "${z_shipped_temp}" || buc_die "The perambulation ships nothing — refusing to cut an empty candidate"

  local -r z_clone_dir="${z_target_dir}/${RBLM_candidate_subdir}"

  buh_section "Marshal Expede"
  buh_line "  Source commit:        ${z_head}"
  buh_line "  Public base:          ${z_upstream_url}"
  buh_line "  Candidate clone:      ${z_clone_dir}"
  buh_e
  buh_line "  The candidate is built by ADDITION in a clone of the PUBLIC"
  buh_line "  repository. No private object enters the object graph that gets"
  buh_line "  pushed, because none is ever put there. Nothing is stripped."
  buh_e
  buh_line "  Every shipped path is materialized from the committed bytes of"
  buh_line "  ${z_head}, judged by the perambulation"
  buh_line "  (Tools/rbk/rblm_perambulation.sh), then lustrated and regenerated"
  buh_line "  by the clone's own copy of rblm_sterilize.sh."
  buh_e
  buh_line "  The clone receives NO station and NO secrets directory."
  buh_e
  buc_require "Proceed with expede?" "expede"

  buc_step "Cloning the public repository"
  mkdir "${z_target_dir}" || buc_die "Failed to create target directory: ${z_target_dir}"
  git clone "${z_upstream_url}" "${z_clone_dir}" || buc_die "Failed to clone the public repository"

  # The base is swept BEFORE anything is added to it. If a previous candidate
  # leaked, the leak is in this graph already, and a clean cut on a dirty base
  # would carry it forward wearing this cut's name.
  buc_step "Sweeping the public base"
  zrblm_expede_sweep "${z_clone_dir}" "base"

  # The public repository may carry no commit at all — an empty upstream is the
  # legitimate state before the first candidate is ever published, and it is where
  # a repository that had to be emptied begins again. Then the candidate is a ROOT
  # commit, and "one commit atop the base" means one commit, full stop.
  local -r z_base_temp="${BURD_TEMP_DIR}/rblm_expede_base.txt"
  local z_base=""
  if git -C "${z_clone_dir}" rev-parse HEAD > "${z_base_temp}" 2>/dev/null; then
    z_base=$(<"${z_base_temp}")
    buh_line "  Public base commit:   ${z_base}"
  else
    buh_line "  Public base commit:   (none — the upstream is empty; this candidate is a root commit)"
  fi

  buc_step "Clearing the base tree"
  local -r z_base_files_temp="${BURD_TEMP_DIR}/rblm_expede_base_files.txt"
  git -C "${z_clone_dir}" ls-files > "${z_base_files_temp}" || buc_die "git ls-files failed in the clone"

  local z_base_files=()
  local z_line=""
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    z_base_files+=("${z_line}")
  done < "${z_base_files_temp}"

  # Removed rather than overwritten. A path the previous release shipped and this
  # one withholds must LEAVE the tree; materializing on top of the base would let
  # it persist forever, unnoticed, because nothing would ever name it again.
  local z_i=0
  for z_i in "${!z_base_files[@]}"; do
    rm -f "${z_clone_dir}/${z_base_files[${z_i}]}" || buc_die "Failed to clear: ${z_base_files[${z_i}]}"
  done

  # git archive, not a per-path copy: it reads the committed bytes of the named
  # commit and preserves the mode bits. A shipped tree of several hundred scripts
  # that arrived without their executable bit would be a candidate that cannot run.
  buc_step "Materializing the shipped paths from ${z_head}"

  # The pathspec is handed to git archive as arguments, loaded from the shipped
  # list rather than piped: git archive takes no pathspec file, and routing several
  # hundred paths through xargs would split any one of them that carried a space.
  local z_shipped=()
  while IFS= read -r z_line || test -n "${z_line}"; do
    test -n "${z_line}" || continue
    z_shipped+=("${z_line}")
  done < "${z_shipped_temp}"

  local -r z_archive_temp="${BURD_TEMP_DIR}/rblm_expede_shipped.tar"
  git archive --format=tar -o "${z_archive_temp}" "${z_head}" -- "${z_shipped[@]}" \
    || buc_die "Failed to archive the shipped paths from ${z_head}"
  tar -x -f "${z_archive_temp}" -C "${z_clone_dir}" \
    || buc_die "Failed to materialize the shipped paths into the clone"

  # The clone runs ITS OWN copy. A process cannot regenerate from lustrated values
  # unless its modules are the lustrated modules — see rblm_sterilize.sh.
  buc_step "Lustrating and regenerating in the clone"
  test -f "${z_clone_dir}/${RBLM_sterilize_path}" \
    || buc_die "The materialized candidate carries no sterilize script — the perambulation must ship ${RBLM_sterilize_path}"
  bash "${z_clone_dir}/${RBLM_sterilize_path}" || buc_die "Sterilization failed in the clone"

  buc_step "Committing the candidate"
  git -C "${z_clone_dir}" add --all || buc_die "Failed to stage the candidate"
  git -C "${z_clone_dir}" commit -m "${RBLM_candidate_subject}" || buc_die "Candidate commit failed"

  # One commit. Not a convention — the property that makes the candidate mergeable
  # by construction and provable by inspection.
  local -r z_count_temp="${BURD_TEMP_DIR}/rblm_expede_count.txt"
  local z_range="HEAD"
  test -z "${z_base}" || z_range="${z_base}..HEAD"

  git -C "${z_clone_dir}" rev-list --count "${z_range}" > "${z_count_temp}" \
    || buc_die "git rev-list failed in the clone"
  local -r z_count=$(<"${z_count_temp}")
  test "${z_count}" = "1" \
    || buc_die "The candidate is ${z_count} commits atop the public base, not 1 — refusing to call it a candidate"

  buc_step "Sweeping the candidate"
  zrblm_expede_sweep "${z_clone_dir}" "candidate"

  buh_e
  buh_line "  Candidate:  ${z_clone_dir}"
  buh_line "  Base:       ${z_base:-(root commit — the upstream was empty)}"
  buh_line "  Commits:    1"
  buh_e
  buh_line "  Prove it from the consumer's seat before it goes anywhere:"
  buc_tabtarget "${RBZ_THEURGE_FIXTURE}" "damnatio"
  buh_e
  buc_success "Candidate expedited — one commit atop the public base, no withheld path in its graph"
}

# zrblm_expede_sweep CLONE_DIR LABEL — prove no withheld path is reachable in the
# clone's object graph.
#
# The object graph, not the tree. rev-list --objects --all walks every object
# reachable from every ref, so a withheld path is caught wherever it hides — in an
# ancestor commit, in a branch nobody looks at, at any depth of history. This is
# precisely the reading that would have caught the 292 MiB candidate, whose face
# was clean and whose history was not.
zrblm_expede_sweep() {
  local -r z_clone_dir="${1:-}"
  local -r z_label="${2:-}"
  test -n "${z_clone_dir}" || buc_die "zrblm_expede_sweep: clone directory required"
  test -n "${z_label}"     || buc_die "zrblm_expede_sweep: label required"

  local -r z_objects_temp="${BURD_TEMP_DIR}/rblm_expede_objects_${z_label}.txt"
  local -r z_paths_temp="${BURD_TEMP_DIR}/rblm_expede_paths_${z_label}.txt"

  git -C "${z_clone_dir}" rev-list --objects --all > "${z_objects_temp}" \
    || buc_die "Failed to walk the object graph of ${z_clone_dir}"

  # rev-list --objects emits "SHA path" for anything with a path, and a bare SHA
  # for commits. Cut to the path and drop the pathless lines.
  local z_line=""
  local z_path=""
  : > "${z_paths_temp}"
  while IFS= read -r z_line || test -n "${z_line}"; do
    case "${z_line}" in
      *" "*) z_path="${z_line#* }" ;;
      *)     continue ;;
    esac
    test -n "${z_path}" || continue
    printf '%s\n' "${z_path}" >> "${z_paths_temp}"
  done < "${z_objects_temp}"

  rblm_perambulation_sweep_capture "${z_paths_temp}"

  test "${#ZRBLM_LEAKS[@]}" -eq 0 || {
    local -r z_leaks_temp="${BURD_TEMP_DIR}/rblm_expede_leaks_${z_label}.txt"
    local z_i=0
    : > "${z_leaks_temp}"
    for z_i in "${!ZRBLM_LEAKS[@]}"; do
      printf '%s\n' "${ZRBLM_LEAKS[${z_i}]}" >> "${z_leaks_temp}"
    done
    buc_die "The ${z_label} object graph carries ${#ZRBLM_LEAKS[@]} withheld path(s). See: ${z_leaks_temp}"
  }

  buh_line "  Object graph clean (${z_label}): no withheld path reachable"
}

######################################################################
# Furnish and Main

zrblm_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env "BURD_TEMP_DIR         " "Temporary directory for this invocation (dispatch-provided)"
  # BUZ_FOLIO (param1 channel) carries expede's target directory and zero's tree
  # identity, and is legitimately empty for the folioless verbs, so it is not a
  # buc_doc_env here: an empty doc_env var warns, and the warn path needs
  # buym_yelp, which this furnish has not yet sourced (rbgv_cli optional-folio
  # precedent).
  buc_doc_env_done || return 0

  local z_rbk_kit_dir="${BASH_SOURCE[0]%/*}"
  source "${z_rbk_kit_dir}/rbcc_constants.sh"      || buc_die "Failed to source rbcc_constants.sh"
  source "${z_rbk_kit_dir}/rblm_lustrate.sh"       || buc_die "Failed to source rblm_lustrate.sh"
  source "${z_rbk_kit_dir}/rblm_perambulation.sh"  || buc_die "Failed to source rblm_perambulation.sh"

  source "${BURD_BUK_DIR}/buym_yelp.sh"         || buc_die "Failed to source buym_yelp.sh"
  source "${BURD_BUK_DIR}/buh_handbook.sh"      || buc_die "Failed to source buh_handbook.sh"
  source "${BURD_BUK_DIR}/buz_zipper.sh"     || buc_die "Failed to source buz_zipper.sh"
  source "${z_rbk_kit_dir}/rbz_zipper.sh"    || buc_die "Failed to source rbz_zipper.sh"
  zbuz_kindle
  zrbz_kindle
}

buc_execute rblm_ "Lifecycle Marshal" zrblm_furnish "$@"

# eof
