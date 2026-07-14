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
# RBLM Expede - the cut: build the delivery candidate by ADDITION into a clone of
# the public repository.
#
# THIS FILE STAYS BEHIND, with the census it reads. Expede is the one marshal verb
# that calls the perambulation, and the perambulation withholds itself from
# delivery — so a delivered rblm_cli.sh carrying expede's body would carry call
# sites to four functions no delivered module defines. The candidate's own cupel
# reads them as unknown commands and reddens, which is the correct verdict on a
# file that names what is not there. The verb, its constants, and its object-graph
# sweep therefore live here, beside the census, and the shipped CLI holds only the
# three verbs a consumer's tree can actually resolve: zero, lustrate, feign.
#
# Nothing is lost by the absence. Expede runs only in the maintainer's own
# repository — it is the act of issuing the candidate, and a consumer holding a
# candidate has no candidate to issue — and its tabtarget (tt/rbw-ME) is withheld
# by the same census, so the delivered tree offers no way to call it either.

set -euo pipefail

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

  # The census is expede's alone, and it is withheld from delivery — sourcing it
  # at furnish would break every delivered marshal verb (see zrblm_furnish).
  local -r z_expede_kit_dir="${BASH_SOURCE[0]%/*}"
  source "${z_expede_kit_dir}/rblm_perambulation.sh" || buc_die "Failed to source rblm_perambulation.sh"

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

  local -r z_verdicts_temp="${BURD_TEMP_DIR}/rblm_expede_verdicts.txt"
  rblm_emit_verdicts > "${z_verdicts_temp}" || buc_die "Failed to judge the tracked tree"

  local z_verdict_line=""
  : > "${z_unjudged_temp}"
  while IFS= read -r z_verdict_line || test -n "${z_verdict_line}"; do
    case "${z_verdict_line}" in
      unjudged*) printf '%s\n' "${z_verdict_line}" >> "${z_unjudged_temp}" ;;
      *)         continue ;;
    esac
  done < "${z_verdicts_temp}"
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

# eof
