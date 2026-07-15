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
#
# EXPEDE IS A PURE LOCAL CONSTRUCTOR. It clones the real public base, builds the
# candidate atop it, and PUSHES NOTHING. It even severs the clone's origin, so the
# finished candidate holds zero remotes: the reveal to the public is human-hands-
# only not by discipline but by structural incapacity — expede wires no remote that
# could reach the public target, so no bug in it can. The reversible quarantine
# preview and the irreversible public reveal are numbered human steps in RELEASE.md.

set -euo pipefail

######################################################################
# Expede constants

# The candidate clone's directory name beneath the operator's target dir.
RBLM_candidate_subdir="candidate"

# The base remote. Expede clones this — the REAL public repository — and cuts the
# candidate one commit atop its live main. It is the ONLY endpoint expede knows: it
# is read (cloned) and never pushed to, and expede strips the clone's origin after
# cloning so the name cannot be pushed to even by accident. The eventual public
# reveal (this same repository) and the private quarantine are reached by explicit
# URL in RELEASE.md — expede contains neither, so it cannot reveal even if a bug
# tried. Its name is loud on purpose (_UPSTREAM, all caps): it can never be confused
# with origin, main, or a hand-typed default.
RBLM_base_remote="ENGROSSMENT_UPSTREAM"

# The base is READ-ONLY, and that is asserted, not merely intended. A push-capable
# remote to the public target in the maintainer repo is the one catastrophe the
# ceremony forbids: a stray `git push ENGROSSMENT_UPSTREAM` from the maintainer's own
# main would put the private tree on public main. So the base remote's PUSH url must
# be neutered to this sentinel, and expede refuses to cut until it is. The clone
# still reads (fetch) the real URL; only the push side is dead.
RBLM_base_push_disabled="DISABLED-ENGROSSMENT_UPSTREAM-IS-READ-ONLY"

# The base's expected fetch URL, hardcoded and asserted before anything is cloned.
# Expede clones whatever the base remote names, and the base-inventory sweep below is
# NON-FATAL by design — it tolerates already-disclosed withheld history because the
# base is the genuinely public repository. That tolerance is safe ONLY if the base
# truly IS that repository: a fetch URL fat-fingered at the private maintainer repo
# would clone the whole private history, the inventory would wave it through as
# "already disclosed", the delta sweep would see only the clean one commit, and the
# candidate would green-light — then a push would upload the entire private ancestry
# (the 292 MiB catastrophe, back through the one unlocked door). Hardcoded like the
# quarantine URL, and for the same reason: the endpoint is a load-bearing fact, not a
# runtime input.
RBLM_base_url="git@github.com:scaleinv/recipebottle.git"

# The candidate's local branch — and, reused by operator ruling (260715), the public
# staging branch name too: one official name across both. Deliberately NOT "main": a
# branch named main would let a default-shaped push land on public main. It is loud
# and unmistakable, it is the only branch the finished clone carries, the
# reversible-preview and irreversible-staging pushes both spell
# POSTULANT_LOCAL:POSTULANT_LOCAL, and only the far-side promotion spells
# POSTULANT_LOCAL:main — by hand, exactly once in the whole corpus.
RBLM_candidate_branch="POSTULANT_LOCAL"

# The subject of the single commit the candidate carries. One commit, so one
# subject: it names the act, not the contents.
RBLM_candidate_subject="Recipe Bottle release candidate"

# The sterilize script, repo-relative. Expede runs THE CANDIDATE'S copy of this
# path, never its own — so the path is named once, here, and the perambulation is
# what guarantees the candidate has it.
RBLM_sterilize_path="Tools/rbk/rblm_sterilize.sh"

# The consumer CLAUDE.md template, repo-relative. Expede transposes THIS file's
# committed bytes onto the candidate's root CLAUDE.md between materialization and
# the commit: the candidate must carry the consumer's context, never the
# maintainer's veiled-path-laden one. Named here because expede stays behind
# (withheld), so it may name a veiled source path.
RBLM_consumer_claude_path="Tools/rbk/vov_veiled/CLAUDE.consumer.md"

# The candidate's root CLAUDE.md — the transposition's target, and a path the
# perambulation ships so the sweep expects it in the candidate graph. The bytes it
# carries are the consumer template's, not the maintainer's.
RBLM_candidate_claude_path="CLAUDE.md"

# The private quarantine — an explicit URL, printed only as reversible-preview
# advice. Expede never pushes it; it is the operator's own next step in RELEASE.md.
# The public reveal target is deliberately NOT named here, in any form.
RBLM_quarantine_url="git@github.com:scaleinv/recipebottle-staging.git"

######################################################################
# Command: expede - Cut the delivery candidate by ADDITION into a public clone
#
# To expede is to issue an instrument from the chancery once due process is done.
# That is the act: gate, copy the shipped bytes out of the committed record, dress
# them, transpose the consumer context, and issue exactly one commit — then push
# nothing.
#
# THE CANDIDATE IS BUILT BY ADDITION, and everything else here follows from it.
# The clone is of the PUBLIC repository, so the object graph the push walks began
# with no private object in it and never receives one. Nothing is stripped, because
# nothing withheld is ever put in. The 2026-07-13 candidate was built the other way
# — the whole repository, then removals — and its TIP was spotless: every strip had
# landed, and every assay this project owned read the tip and passed it. It went to
# the remote at 292 MiB because the strip cleans the face while the push sends
# everything reachable from the branch. Construction is the prevention. The delta
# sweep below is not the guard; it is the proof that the guard held.
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

  # The base. A candidate is one commit atop the real PUBLIC main, so the base
  # remote is not optional scenery — it is the thing being added to. Expede only
  # ever CLONES it; it is severed from the clone below and never pushed to.
  local -r z_upstream_temp="${BURD_TEMP_DIR}/rblm_expede_upstream.txt"
  local -r z_upstream_err_temp="${BURD_TEMP_DIR}/rblm_expede_upstream_err.txt"
  git remote get-url "${RBLM_base_remote}" > "${z_upstream_temp}" 2>"${z_upstream_err_temp}" \
    || buc_die "${RBLM_base_remote} is not configured — the candidate is built by addition atop the real public repository, so a remote pointing at it is required (git remote add ${RBLM_base_remote} <public-repo-url>). See: ${z_upstream_err_temp}"
  local -r z_upstream_url=$(<"${z_upstream_temp}")
  test -n "${z_upstream_url}" || buc_die "${RBLM_base_remote} URL is empty"

  # The base's IDENTITY, asserted before anything is cloned. The base-inventory sweep
  # below tolerates already-disclosed withheld history without dying — safe ONLY
  # because the base is the genuinely public repository. Cutting atop the wrong repo (a
  # fetch URL aimed at the private maintainer tree) would wave its whole history
  # through as "already disclosed" and green-light a candidate that leaks it, so the
  # fetch URL must be exactly the expected public base.
  test "${z_upstream_url}" = "${RBLM_base_url}" \
    || buc_die "${RBLM_base_remote} points at ${z_upstream_url}, not the expected public base ${RBLM_base_url} — refusing to cut atop the wrong repository (the base-inventory sweep is non-fatal ONLY because the base is the genuinely public repo)"

  # The base is read-only, asserted. Its PUSH url must be the neutered sentinel — a
  # live push url to the public target is refused before a single object is cloned, so
  # no maintainer-side push can ever reach public main. The clone below still uses the
  # fetch url (z_upstream_url) to read.
  local -r z_push_temp="${BURD_TEMP_DIR}/rblm_expede_push.txt"
  local -r z_push_err_temp="${BURD_TEMP_DIR}/rblm_expede_push_err.txt"
  git remote get-url --push "${RBLM_base_remote}" > "${z_push_temp}" 2>"${z_push_err_temp}" \
    || buc_die "Cannot read ${RBLM_base_remote} push url — see ${z_push_err_temp}"
  local -r z_push_url=$(<"${z_push_temp}")
  test "${z_push_url}" = "${RBLM_base_push_disabled}" \
    || buc_die "${RBLM_base_remote} has a live push url (${z_push_url}) — the base must be read-only. Neuter it: git remote set-url --push ${RBLM_base_remote} ${RBLM_base_push_disabled}"

  local -r z_head_temp="${BURD_TEMP_DIR}/rblm_expede_head.txt"
  git rev-parse HEAD > "${z_head_temp}" || buc_die "git rev-parse HEAD failed"
  local -r z_head=$(<"${z_head_temp}")

  # The consumer CLAUDE.md template must exist in the committed record before the
  # cut begins — the transposition below reads its bytes, and a missing template
  # would surface only after the whole candidate was built.
  local -r z_tmpl_err_temp="${BURD_TEMP_DIR}/rblm_expede_template_err.txt"
  git cat-file -e "${z_head}:${RBLM_consumer_claude_path}" 2>"${z_tmpl_err_temp}" \
    || buc_die "The consumer CLAUDE.md template is absent from ${z_head}: ${RBLM_consumer_claude_path} — see ${z_tmpl_err_temp}"

  local -r z_shipped_temp="${BURD_TEMP_DIR}/rblm_expede_shipped.txt"
  rblm_emit_shipped > "${z_shipped_temp}" || buc_die "Failed to enumerate the shipped paths"
  test -s "${z_shipped_temp}" || buc_die "The perambulation ships nothing — refusing to cut an empty candidate"

  local -r z_clone_dir="${z_target_dir}/${RBLM_candidate_subdir}"

  buh_section "Marshal Expede"
  buh_line "  Source commit:        ${z_head}"
  buh_line "  Public base remote:   ${RBLM_base_remote}"
  buh_line "  Public base URL:      ${z_upstream_url}"
  buh_line "  Candidate clone:      ${z_clone_dir}"
  buh_line "  Candidate branch:     ${RBLM_candidate_branch}"
  buh_e
  buh_line "  The candidate is built by ADDITION in a clone of the real PUBLIC"
  buh_line "  repository. No private object enters the object graph, because none"
  buh_line "  is ever put there. Nothing is stripped."
  buh_e
  buh_line "  Every shipped path is materialized from the committed bytes of"
  buh_line "  ${z_head}, judged by the perambulation"
  buh_line "  (Tools/rbk/rblm_perambulation.sh), then lustrated and regenerated"
  buh_line "  by the clone's own copy of rblm_sterilize.sh. The root CLAUDE.md is"
  buh_line "  transposed to the consumer template and byte-asserted."
  buh_e
  buh_line "  The clone receives NO station and NO secrets directory, and it is"
  buh_line "  SEVERED from its origin: the finished candidate holds zero remotes,"
  buh_line "  so expede cannot push it anywhere. The reveal is a human step."
  buh_e

  buc_step "Cloning the public base"
  mkdir "${z_target_dir}" || buc_die "Failed to create target directory: ${z_target_dir}"
  git clone "${z_upstream_url}" "${z_clone_dir}" || buc_die "Failed to clone the public base"

  # The base is surveyed BEFORE anything is added to it, over its whole object
  # graph. Under a real public base this is an INVENTORY, not a gate: the public
  # history may already carry withheld paths that a prior era disclosed, and
  # already-disclosed history can be known but not un-disclosed. It is surfaced for
  # the operator to acknowledge — loud, never fatal, never silent. What this cut is
  # answerable for is the DELTA it adds, swept fatally below.
  buc_step "Surveying the public base (inventory)"
  zrblm_expede_graph_leaks "${z_clone_dir}" "base" --all
  if test "${#ZRBLM_LEAKS[@]}" -gt 0; then
    local -r z_base_inv_temp="${BURD_TEMP_DIR}/rblm_expede_base_inventory.txt"
    local z_inv_i=0
    : > "${z_base_inv_temp}"
    for z_inv_i in "${!ZRBLM_LEAKS[@]}"; do
      printf '%s\n' "${ZRBLM_LEAKS[${z_inv_i}]}" >> "${z_base_inv_temp}"
    done
    buh_e
    buh_line "  BASE INVENTORY — ${#ZRBLM_LEAKS[@]} withheld path(s) already in the"
    buh_line "  public history. Already disclosed; cannot be un-disclosed by this"
    buh_line "  cut. This is not a leak of THIS cut — acknowledge and proceed."
    buh_line "  See: ${z_base_inv_temp}"
    buh_e
  else
    buh_line "  Base object graph clean: no withheld path in the public history"
  fi

  # The public repository may carry no commit at all — an empty base is the
  # legitimate state before the first candidate is ever published. Then the
  # candidate is a ROOT commit, and "one commit atop the base" means one commit,
  # full stop. The base SHA is captured now, as a value, so the delta range and the
  # commit count below survive the branch surgery that follows.
  local -r z_base_temp="${BURD_TEMP_DIR}/rblm_expede_base.txt"
  local -r z_base_err_temp="${BURD_TEMP_DIR}/rblm_expede_base_err.txt"
  local z_base=""
  if git -C "${z_clone_dir}" rev-parse HEAD > "${z_base_temp}" 2>"${z_base_err_temp}"; then
    z_base=$(<"${z_base_temp}")
    buh_line "  Public base commit:   ${z_base}"
  else
    buh_line "  Public base commit:   (none — the base is empty; this candidate is a root commit)"
  fi

  # Sever the clone from its origin. From here the clone holds no remote, so nothing
  # it does can reach the public repository. Materialization is from the maintainer
  # repo's own object store (git archive of ${z_head}), never from the clone's
  # remote, so the sever costs the build nothing. Zero remotes is asserted again at
  # the end, as the finished candidate's standing property.
  buc_step "Severing the clone from its origin"
  git -C "${z_clone_dir}" remote remove origin || buc_die "Failed to sever the clone's origin"

  # Open the candidate's own branch, and drop every other. The clone must carry
  # exactly ONE branch, named POSTULANT_LOCAL — so even a forbidden fan-out push
  # (--all) could name nothing but POSTULANT_LOCAL, never main. On an empty base
  # there is no other head to drop; the unborn branch is simply renamed.
  buc_step "Opening the candidate branch ${RBLM_candidate_branch}"
  git -C "${z_clone_dir}" checkout -b "${RBLM_candidate_branch}" || buc_die "Failed to open the candidate branch"

  local -r z_heads_temp="${BURD_TEMP_DIR}/rblm_expede_heads.txt"
  git -C "${z_clone_dir}" for-each-ref --format='%(refname:short)' refs/heads/ > "${z_heads_temp}" \
    || buc_die "Failed to list the clone's branches"
  local z_head_ref=""
  while IFS= read -r z_head_ref || test -n "${z_head_ref}"; do
    test -n "${z_head_ref}" || continue
    test "${z_head_ref}" != "${RBLM_candidate_branch}" || continue
    git -C "${z_clone_dir}" branch -D "${z_head_ref}" || buc_die "Failed to drop base branch ${z_head_ref}"
  done < "${z_heads_temp}"

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

  # Transpose the consumer context onto the candidate's CLAUDE.md. The perambulation
  # ships CLAUDE.md, so the materialization above wrote the MAINTAINER's copy into
  # the working tree — veiled paths on its face. That copy is now overwritten, in
  # the working tree, before the commit, so the maintainer's CLAUDE.md never enters
  # the candidate's object graph: the committed blob is the consumer template's. The
  # swap is performed HERE, by expede, not assumed by a comment somewhere downstream.
  buc_step "Transposing the consumer context onto ${RBLM_candidate_claude_path}"
  local -r z_claude_target="${z_clone_dir}/${RBLM_candidate_claude_path}"
  git show "${z_head}:${RBLM_consumer_claude_path}" > "${z_claude_target}" \
    || buc_die "Failed to transpose the consumer CLAUDE.md template"

  # Byte-assert the result equals the committed template. A transposition that
  # silently wrote nothing, or wrote a truncated stream, would ship the wrong
  # context under a green battery — the exact failure this pace exists to close.
  # Byte-exact match via openssl sha256 digests (declared dependency; replaces
  # cmp, matching the rbndb_base.sh RBRD-drift precedent) — no new command.
  local -r z_claude_expect_temp="${BURD_TEMP_DIR}/rblm_expede_claude_expect.txt"
  git show "${z_head}:${RBLM_consumer_claude_path}" > "${z_claude_expect_temp}" \
    || buc_die "Failed to re-read the consumer CLAUDE.md template for assertion"
  local -r z_claude_expect_digest_temp="${BURD_TEMP_DIR}/rblm_expede_claude_expect_digest.txt"
  local -r z_claude_target_digest_temp="${BURD_TEMP_DIR}/rblm_expede_claude_target_digest.txt"
  openssl dgst -sha256 -r < "${z_claude_expect_temp}" > "${z_claude_expect_digest_temp}" \
    || buc_die "Failed to digest the consumer CLAUDE.md template for assertion"
  openssl dgst -sha256 -r < "${z_claude_target}" > "${z_claude_target_digest_temp}" \
    || buc_die "Failed to digest ${RBLM_candidate_claude_path}"
  local -r z_claude_expect_digest=$(<"${z_claude_expect_digest_temp}")
  local -r z_claude_target_digest=$(<"${z_claude_target_digest_temp}")
  test -n "${z_claude_expect_digest}" || buc_die "Failed to read or empty: ${z_claude_expect_digest_temp}"
  test -n "${z_claude_target_digest}" || buc_die "Failed to read or empty: ${z_claude_target_digest_temp}"
  test "${z_claude_expect_digest}" = "${z_claude_target_digest}" \
    || buc_die "Transposition byte-mismatch: ${RBLM_candidate_claude_path} does not equal ${RBLM_consumer_claude_path}"

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

  # Sweep the DELTA, fatally. The object graph the candidate adds atop the base —
  # ${z_base}..HEAD, or the whole graph when the base was empty — must carry no
  # withheld path. This is the assertion the base inventory is not: the base's
  # already-disclosed history is tolerated, but THIS cut adds nothing withheld, and
  # a leak here is fatal.
  buc_step "Sweeping the candidate delta"
  local z_delta="HEAD"
  test -z "${z_base}" || z_delta="${z_base}..HEAD"
  zrblm_expede_graph_leaks "${z_clone_dir}" "candidate" "${z_delta}"
  test "${#ZRBLM_LEAKS[@]}" -eq 0 || {
    local -r z_leaks_temp="${BURD_TEMP_DIR}/rblm_expede_leaks_candidate.txt"
    local z_leak_i=0
    : > "${z_leaks_temp}"
    for z_leak_i in "${!ZRBLM_LEAKS[@]}"; do
      printf '%s\n' "${ZRBLM_LEAKS[${z_leak_i}]}" >> "${z_leaks_temp}"
    done
    buc_die "The candidate delta adds ${#ZRBLM_LEAKS[@]} withheld path(s). See: ${z_leaks_temp}"
  }
  buh_line "  Candidate delta clean: this cut adds no withheld path"

  # Zero remotes, asserted as the finished candidate's standing property. The clone
  # was severed above; this proves the sever held and nothing re-wired a remote.
  # With no remote, no command in the clone can reach the public target: the reveal
  # is human-hands-only by structural incapacity, not by a rule anyone remembered.
  buc_step "Asserting the candidate holds zero remotes"
  local -r z_remotes_temp="${BURD_TEMP_DIR}/rblm_expede_remotes.txt"
  git -C "${z_clone_dir}" remote > "${z_remotes_temp}" || buc_die "Failed to read the clone's remotes"
  test ! -s "${z_remotes_temp}" || buc_die "The candidate clone carries a remote — expede must leave zero. See: ${z_remotes_temp}"

  buh_e
  buh_line "  Candidate:  ${z_clone_dir}"
  buh_line "  Branch:     ${RBLM_candidate_branch}"
  buh_line "  Base:       ${z_base:-(root commit — the base was empty)}"
  buh_line "  Commits:    1"
  buh_line "  Remotes:    0 (severed — expede can reach nothing)"
  buh_e
  buh_line "  Prove it from the consumer's seat before it goes anywhere:"
  buc_tabtarget "${RBZ_THEURGE_FIXTURE}" "damnatio"
  buh_e
  buh_line "  Two operator-hand pushes precede the walk — expede runs neither. Each"
  buh_line "  is an explicit URL and an explicit refspec, and NEITHER touches main:"
  buh_e
  buh_line "  1. Reversible preview into the PRIVATE quarantine. Inspect it on GitHub,"
  buh_line "     then delete the quarantine and nothing escaped:"
  buh_line "       git -C ${z_clone_dir} push ${RBLM_quarantine_url} ${RBLM_candidate_branch}:${RBLM_candidate_branch}"
  buh_e
  buh_line "  2. The IRREVERSIBLE public staging reveal — the candidate onto the real"
  buh_line "     public repository as an unmerged branch. This is the point of no"
  buh_line "     return; it is the disclosure the greenfield walk then clones:"
  buh_line "       git -C ${z_clone_dir} push ${z_upstream_url} ${RBLM_candidate_branch}:${RBLM_candidate_branch}"
  buh_e
  buh_line "  Promotion to public main is the ONE main-touching push, on the far side"
  buh_line "  of the walk — RELEASE.md holds it. Expede prints no main refspec and"
  buh_line "  holds no remote that could perform any of these."
  buh_e
  buc_success "Candidate expedited — one commit atop the public base, zero remotes, no withheld path in the delta"
}

# zrblm_expede_graph_leaks CLONE_DIR LABEL RANGE_ARG... — walk an object-graph
# range and set ZRBLM_LEAKS to every withheld path reachable in it. The disposition
# is the CALLER's: the base tolerates its already-disclosed history (inventory), the
# candidate delta tolerates nothing (fatal). One reader, two verdicts.
#
# The object graph, not the tree. rev-list --objects over the given range walks
# every object it names, so a withheld path is caught wherever it hides — in an
# ancestor commit, at any depth. Fed --all it reads the whole graph (the base); fed
# ${base}..HEAD it reads only what the candidate added. This is precisely the
# reading that would have caught the 292 MiB candidate, whose face was clean and
# whose history was not.
zrblm_expede_graph_leaks() {
  local -r z_clone_dir="${1:-}"
  local -r z_label="${2:-}"
  test -n "${z_clone_dir}" || buc_die "zrblm_expede_graph_leaks: clone directory required"
  test -n "${z_label}"     || buc_die "zrblm_expede_graph_leaks: label required"
  shift 2
  test "$#" -gt 0          || buc_die "zrblm_expede_graph_leaks: no rev-list range given"

  local -r z_objects_temp="${BURD_TEMP_DIR}/rblm_expede_objects_${z_label}.txt"
  local -r z_paths_temp="${BURD_TEMP_DIR}/rblm_expede_paths_${z_label}.txt"

  git -C "${z_clone_dir}" rev-list --objects "$@" > "${z_objects_temp}" \
    || buc_die "Failed to walk the object graph of ${z_clone_dir} (${z_label})"

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
}

# eof
