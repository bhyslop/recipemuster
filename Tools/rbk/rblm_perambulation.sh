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
# RBLM Perambulation - the ship/withhold judgment over every tracked path.
#
# Named for the manorial walking of the bounds: the periodic act that fixes,
# stretch by stretch, what lies within the estate and what lies without. A
# perambulation that does not close is not a perambulation — which is exactly the
# invariant this table is held to.
#
# The proscription (rblm_lustrate.sh) judges every enrolled regime FIELD site or
# common. The perambulation judges every tracked PATH ship or withhold. Same discipline,
# one rung out: the delivered file set is a judgment the project makes once, in a
# table, and proves mechanically — never a list of removals recited in prose.
#
# Expede reads this table to decide what to materialize into the candidate, and to
# sweep the candidate's object graph for anything withheld. The perambulation fixture
# reads it to prove the judgment is TOTAL: a tracked path no row judges is red,
# and a row that judges no path is red. A new file cannot ship silently, and it
# cannot vanish silently either — it is red until someone rules on it.
#
######################################################################
# The matcher
#
# A row is a literal path PREFIX. It matches a repo-relative path when the path
# starts with it. The LONGEST matching row wins — so precedence is a property of
# the rows themselves, not of their order in the table, and no row can be silently
# shadowed by an earlier one.
#
# That single rule spans all three grains the judgment needs:
#
#   - a tree, written with its trailing slash    Tools/jjk/
#   - a stem, written to the discriminating char tt/rbw-M
#   - one file, written as its whole path        RELEASE.md
#
# There is no glob syntax, and deliberately so: a glob's precedence is either
# order-dependent or ambiguous, and both are how a withheld path ends up shipped.
#
# The grain is a judgment in itself. A tree row is a promise that the tree is
# UNIFORM — that anything added under it later may ship (or must not) without
# further thought. Where that promise is false, the tree is judged at a finer
# grain, and the finer rows win by construction. Two places earn it today:
# rbmm_moorings/fdkyclk/, whose two proof-stage scripts carry the operator's org
# id while the rest of the nameplate is the shipped Keycloak test bed; and tt/,
# where the withheld marshal and manor-raze tabtargets sit among the shipped rbw
# family. Both are the same lesson: a directory-grain "all ship" is what carried
# those two fdkyclk scripts into every candidate.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBLM_PERAMBULATION_SOURCED:-}" || buc_die "Module rblm_perambulation multiply sourced - check sourcing hierarchy"
ZRBLM_PERAMBULATION_SOURCED=1

######################################################################
# Dispositions

# Ship: the path is delivered. Materialized into the candidate from committed
# bytes, and expected in the candidate's object graph.
RBLM_perambulation_ship="ship"

# Withhold: the path stays behind. Never materialized, and — the assertion that
# matters — never present in the candidate's object graph at all, at any depth of
# its history. A withheld path found in a candidate is a leak, not a stray.
RBLM_perambulation_withhold="withhold"

######################################################################
# The perambulation
#
# Each row enrolls one PREFIX with its VERDICT. The longest matching prefix wins.
#
# The prose lives HERE rather than among the rows, so the table below stays a bare
# table — which is what a table should be — and every judgment that needs an
# argument is argued in this legend.
#
# THE VEILED HALVES. Each kit's closed record. These rows are finer than the kit
# rows beneath them so the judgment holds even where the whole kit ships. A NEW
# kit's veiled half is deliberately not covered: it lands unjudged and reddens.
# That is correct — a kit's exposure is a ruling; it is never a default.
#
# THE DELIVERED KITS. BUK and RBK ship whole apart from their veiled halves: the
# .sh surface, the shellcheck config, the READMEs, the agent-context markdown, the
# theurge crate, the in-pool python step bodies, the ifrit attack corpus.
#
# THIS FILE STAYS BEHIND, and rblm_expede.sh with it. It is the one module whose
# charter obliges it to name every withheld tree by path — the map of what the
# distribution does not carry is precisely the map it must not carry — and the
# delivered tree has no consumer for it: the marshal tabtargets are withheld,
# expede runs only in the maintainer's own repository, and the perambulation fixture
# that sources it belongs to no suite and is dead in a consumer's hands.
#
# rblm_expede.sh follows it out for a mechanical reason, not a symmetric one: expede
# is the only verb that CALLS the census, so a delivered copy of its body would carry
# call sites to functions no delivered module defines, and the candidate's own cupel
# reads them — correctly — as unknown commands. The two travel together because the
# caller cannot ship where the callee does not.
#
# Their sibling rblm_lustrate.sh does ship, and must: damnatio sources the
# proscription at runtime to prove the candidate's erasure from inside the candidate.
#
# THE WITHHELD KITS. Whole trees. Each is either the operator's own tooling (jjk,
# vok, cmk, vvk) or a project that shares the repo but not the delivery (apck,
# gad, hmk, lmci, vslk). None is uniform by accident: each is a tree the consumer
# has no seat for.
#
# TABTARGETS ARE FILE-GRAIN, because tt/ is not uniform. The marshal family and
# the manor raze are the release rig itself — they zero, clone, lustrate, feign,
# expede, and force-delete the manor's pool. A consumer running any of them holds
# the wrong end of the tool. Note the stem rows: tt/rbw-M outranks tt/rbw- by
# length, so a marshal tabtarget minted tomorrow is withheld without a new row.
# The hierophant's own dispatch stem tt/rbthw- rides the same file-grain logic:
# its colophon is not tt/rbw-, so it is unjudged until named — its whole veiled
# crate conducts the release ceremony and has no consumer seat.
#
# THE MOORINGS. Every nameplate ships: README documents each as an example
# crucible and the onboarding handbooks walk several. The fdkyclk carve-out is the
# reason this table must be file-grain where the tree is not uniform — its two
# proof-stage scripts carry the operator's org id, and a directory-grain "all
# ship" is exactly what carried them into every candidate. Its caged credentials,
# by contrast, DO ship: the asserter keypair and client secret are committed test
# scaffolding the realm expects by value (RBSFK "two-keys"), and removing them
# breaks the test bed's determinism. A secret scanner will flag them; the README's
# fdkyclk caution is the answer, not removal.
#
# LAUNCHERS ship only for the workbenches that ship — file-grain for tt/'s reason:
# a launcher for a withheld workbench is a dangling reference at best.
#
# ROOT FILES THAT STAY BEHIND. RELEASE.md is the release-qualification procedure,
# the rig's own runbook. The MCP config names the operator's servers; the iml is
# IDE furniture; the gateway proposal is an unsettled internal design note.

# zrblm_enroll PREFIX VERDICT — enroll one row into the parallel rolls.
#
# The row is a structural record from birth: two arguments, never a string with a
# delimiter in it. There is nothing to decode, so no malformed row can decode
# SUCCESSFULLY into garbage — which is what an encoded table does, and what it does
# quietly, inside the one module whose charter is that no judgment is silent.
# (Concept: MCM `mcm_phantom_wire`; bash treatment: BCG, Representation axis.)
#
# Every malformation therefore dies HERE, at enrollment, in front of the author who
# wrote the row rather than the operator who is mid-cut:
#
#   - An empty prefix, which would match every path.
#   - A verdict that is not exactly ship or withhold, which no verb would honor.
#   - A DUPLICATE prefix, which would tie-break by roll order — precisely the
#     order-dependent shadowing the longest-wins rule exists to abolish. Without
#     this check, the header's promise that no row can be silently shadowed is
#     false for the one case where two rows are the same length.
zrblm_enroll() {
  local -r z_prefix="${1:-}"
  local -r z_verdict="${2:-}"

  test -n "${z_prefix}" || buc_die "Perambulation row enrolled with an empty prefix"

  case "${z_verdict}" in
    "${RBLM_perambulation_ship}"|"${RBLM_perambulation_withhold}") ;;
    *) buc_die "Perambulation row '${z_prefix}' carries verdict '${z_verdict}' — must be ${RBLM_perambulation_ship} or ${RBLM_perambulation_withhold}" ;;
  esac

  local z_i=0
  for z_i in "${!ZRBLM_PREFIX_ROLL[@]}"; do
    test "${ZRBLM_PREFIX_ROLL[${z_i}]}" = "${z_prefix}" || continue
    buc_die "Perambulation prefix '${z_prefix}' is enrolled twice — equal-length rows would shadow by roll order, which the longest-wins rule exists to abolish"
  done

  ZRBLM_PREFIX_ROLL+=("${z_prefix}")
  ZRBLM_VERDICT_ROLL+=("${z_verdict}")
}

######################################################################
# The kindle — the table itself
#
# One enroll call per judgment, argued in the legend above. The rolls are the
# running matcher's shape (parallel arrays, index-aligned — BUV's own enrollment
# rolls, and what bash 3.2 offers in place of the associative array BCG forbids).
#
# Idempotent: the rolls are built once per shell, so every entry point may call it.
zrblm_perambulation_kindle() {
  test -z "${ZRBLM_KINDLED:-}" || return 0

  ZRBLM_PREFIX_ROLL=()
  ZRBLM_VERDICT_ROLL=()

  # Veiled halves
  zrblm_enroll "Tools/buk/vov_veiled/" "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/cmk/vov_veiled/" "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/gad/vov_veiled/" "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/jjk/vov_veiled/" "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/rbk/vov_veiled/" "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/vok/vov_veiled/" "${RBLM_perambulation_withhold}"

  # Delivered kits
  zrblm_enroll "Tools/buk/" "${RBLM_perambulation_ship}"
  zrblm_enroll "Tools/rbk/" "${RBLM_perambulation_ship}"

  # This file, the verb that reads it, and the other release-rig verb module
  zrblm_enroll "Tools/rbk/rblm_perambulation.sh" "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/rbk/rblm_expede.sh"        "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/rbk/rblm_harbinger.sh"     "${RBLM_perambulation_withhold}"

  # Withheld kits
  zrblm_enroll "Tools/apck/"     "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/cmk/"      "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/gad/"      "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/hmk/"      "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/jjk/"      "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/lmci/"     "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/vok/"      "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/vslf-rbw/" "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/vslk/"     "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/vvc/"      "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/vvk/"      "${RBLM_perambulation_withhold}"

  # Residue of retired kits
  zrblm_enroll "Tools/cccr.env"        "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/crgr.render.sh"  "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/crgv.validate.sh" "${RBLM_perambulation_withhold}"
  zrblm_enroll "Tools/xxx_rbn.info.sh" "${RBLM_perambulation_withhold}"

  # Tabtargets — the rig withheld from the delivered families
  zrblm_enroll "tt/rbw-M"        "${RBLM_perambulation_withhold}"
  zrblm_enroll "tt/rbw-mR."      "${RBLM_perambulation_withhold}"
  zrblm_enroll "tt/rbthw-"       "${RBLM_perambulation_withhold}"
  zrblm_enroll "tt/rbw-"         "${RBLM_perambulation_ship}"
  zrblm_enroll "tt/buw-"         "${RBLM_perambulation_ship}"
  zrblm_enroll "tt/z-launcher.sh" "${RBLM_perambulation_ship}"
  zrblm_enroll "tt/apcw-"        "${RBLM_perambulation_withhold}"
  zrblm_enroll "tt/jjw-"         "${RBLM_perambulation_withhold}"
  zrblm_enroll "tt/study-"       "${RBLM_perambulation_withhold}"
  zrblm_enroll "tt/vow-"         "${RBLM_perambulation_withhold}"
  zrblm_enroll "tt/vslk-"        "${RBLM_perambulation_withhold}"
  zrblm_enroll "tt/vvw-"         "${RBLM_perambulation_withhold}"

  # Moorings — the consumer's config tree
  zrblm_enroll "rbmm_moorings/fdkyclk/fdkyclk-proof.sh"    "${RBLM_perambulation_withhold}"
  zrblm_enroll "rbmm_moorings/fdkyclk/fdkyclk-teardown.sh" "${RBLM_perambulation_withhold}"
  zrblm_enroll "rbmm_moorings/fdkyclk/"       "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/ccyolo/"        "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/moriah/"        "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/nineveh/"       "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/pluml/"         "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/srjcl/"         "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/tadmor/"        "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/rbmf_foedera/"  "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/rbmv_vessels/"  "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/burc.env"       "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/rbrd.env"       "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/rbrp.env"       "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/rbrr.env"       "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/rbrw.env"       "${RBLM_perambulation_ship}"

  # The operator's remote machines
  zrblm_enroll "rbmm_moorings/rbmn_nodes/" "${RBLM_perambulation_withhold}"
  zrblm_enroll "rbmm_moorings/rbmu_users/" "${RBLM_perambulation_withhold}"

  # Launchers — only for workbenches that ship
  zrblm_enroll "rbmm_moorings/rbml_launchers/launcher.buw_workbench.sh" "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/rbml_launchers/launcher.rbw_workbench.sh" "${RBLM_perambulation_ship}"
  zrblm_enroll "rbmm_moorings/rbml_launchers/" "${RBLM_perambulation_withhold}"

  # The operator's own trees
  zrblm_enroll "Memos/"      "${RBLM_perambulation_withhold}"
  zrblm_enroll "Study/"      "${RBLM_perambulation_withhold}"
  zrblm_enroll ".claude/"    "${RBLM_perambulation_withhold}"
  zrblm_enroll ".idea/"      "${RBLM_perambulation_withhold}"
  zrblm_enroll ".jjk/"       "${RBLM_perambulation_withhold}"
  zrblm_enroll "_slickedit/" "${RBLM_perambulation_withhold}"

  # The delivered face
  zrblm_enroll "README.md"               "${RBLM_perambulation_ship}"
  zrblm_enroll "CLAUDE.md"               "${RBLM_perambulation_ship}"
  zrblm_enroll "LICENSE"                 "${RBLM_perambulation_ship}"
  zrblm_enroll "diagrams/"               "${RBLM_perambulation_ship}"
  zrblm_enroll "rbm-abstract-drawio.svg" "${RBLM_perambulation_ship}"
  zrblm_enroll ".gitattributes"          "${RBLM_perambulation_ship}"
  zrblm_enroll ".gitignore"              "${RBLM_perambulation_ship}"

  # Root files that stay behind
  zrblm_enroll "RELEASE.md"                 "${RBLM_perambulation_withhold}"
  zrblm_enroll ".mcp.json"                  "${RBLM_perambulation_withhold}"
  zrblm_enroll "brm_recipemuster.iml"       "${RBLM_perambulation_withhold}"
  zrblm_enroll "podman-gateway-proposal.md" "${RBLM_perambulation_withhold}"

  ZRBLM_KINDLED=1
}

######################################################################
# The judgment

# rblm_perambulation_judge PATH — rule one repo-relative path.
#
# Sets ZRBLM_JUDGMENT to the winning verdict, ZRBLM_JUDGED_BY to the prefix that
# won, and ZRBLM_JUDGED_INDEX to that prefix's roll index — the index is what lets
# a caller mark the winning row without a second lookup. Returns 0 when a row
# matched, 1 when the path is UNJUDGED: the state the whole table exists to make
# loud. An unjudged path is never treated as either verdict — there is no default
# in either direction.
rblm_perambulation_judge() {
  local -r z_path="${1:-}"

  zrblm_perambulation_kindle

  ZRBLM_JUDGMENT=""
  ZRBLM_JUDGED_BY=""
  ZRBLM_JUDGED_INDEX=-1

  local z_i=0
  local z_prefix=""
  local z_best=-1

  for z_i in "${!ZRBLM_PREFIX_ROLL[@]}"; do
    z_prefix="${ZRBLM_PREFIX_ROLL[${z_i}]}"
    test "${z_path#"${z_prefix}"}" != "${z_path}" || continue
    test "${#z_prefix}" -gt "${z_best}"           || continue
    z_best="${#z_prefix}"
    ZRBLM_JUDGMENT="${ZRBLM_VERDICT_ROLL[${z_i}]}"
    ZRBLM_JUDGED_BY="${z_prefix}"
    ZRBLM_JUDGED_INDEX="${z_i}"
  done

  test "${z_best}" -ge 0 || return 1
  return 0
}

# rblm_perambulation_tracked_capture — the live tracked set, from git, not from a
# list. Sets ZRBLM_TRACKED.
#
# The perambulation is judged against what the repository actually carries at this
# commit. Any other source of truth is a second copy waiting to drift.
#
# Every tracked path is judgeable: a row is a literal prefix, so any character git
# admits in a path a row may carry verbatim. Nothing about a path's spelling can
# make it unrulable.
rblm_perambulation_tracked_capture() {
  test -n "${BURD_TEMP_DIR:-}" || buc_die "BURD_TEMP_DIR not set - launch via tabtarget"
  mkdir -p "${BURD_TEMP_DIR}" || buc_die "Failed to create temp directory"

  local -r z_tracked_temp="${BURD_TEMP_DIR}/rblm_perambulation_tracked.txt"
  git ls-files > "${z_tracked_temp}" || buc_die "git ls-files failed"

  ZRBLM_TRACKED=()

  local z_path=""
  while IFS= read -r z_path || test -n "${z_path}"; do
    test -n "${z_path}" || continue
    ZRBLM_TRACKED+=("${z_path}")
  done < "${z_tracked_temp}"

  test "${#ZRBLM_TRACKED[@]}" -gt 0 || buc_die "No tracked paths — the perambulation has nothing to judge"
}

######################################################################
# Emitters — the fixture's reach
#
# Tab-separated, one row per line, no decoration. The perambulation fixture sources
# this module and calls these, so the table it judges against is the same table
# expede ships from. One matcher, in bash, read by both — never a second
# implementation in Rust to drift against this one.

# rblm_emit_verdicts — one line per tracked path: VERDICT \t PATH.
#
# An unjudged path is emitted with the literal verdict "unjudged", not omitted:
# the fixture must see it to redden on it.
rblm_emit_verdicts() {
  rblm_perambulation_tracked_capture

  local z_i=0
  local z_path=""

  for z_i in "${!ZRBLM_TRACKED[@]}"; do
    z_path="${ZRBLM_TRACKED[${z_i}]}"
    if rblm_perambulation_judge "${z_path}"; then
      printf '%s\t%s\n' "${ZRBLM_JUDGMENT}" "${z_path}"
    else
      printf 'unjudged\t%s\n' "${z_path}"
    fi
  done
}

# rblm_emit_dead_rows — every perambulation row that wins for no tracked path.
#
# Two failures wear this one face. A STALE row judges a path that no longer exists
# (the prose strip lists this table replaced still named three such paths). A
# SHADOWED row is outranked everywhere by a longer one, so its judgment never
# lands. Both mean the table is lying about the tree, and both go red.
#
# The won-set is a roll parallel to the prefix roll, marked by the winning index
# the judgment already returns — no membership scan, no second lookup.
rblm_emit_dead_rows() {
  zrblm_perambulation_kindle
  rblm_perambulation_tracked_capture

  local z_won_roll=()
  local z_i=0

  for z_i in "${!ZRBLM_PREFIX_ROLL[@]}"; do
    z_won_roll+=("")
  done

  for z_i in "${!ZRBLM_TRACKED[@]}"; do
    rblm_perambulation_judge "${ZRBLM_TRACKED[${z_i}]}" || continue
    z_won_roll["${ZRBLM_JUDGED_INDEX}"]=1
  done

  for z_i in "${!ZRBLM_PREFIX_ROLL[@]}"; do
    test -z "${z_won_roll[${z_i}]}" || continue
    printf '%s\t%s\n' "${ZRBLM_PREFIX_ROLL[${z_i}]}" "${ZRBLM_VERDICT_ROLL[${z_i}]}"
  done
}

# rblm_emit_shipped — every tracked path the perambulation ships, one per line.
#
# Expede's materialization list. Ordinary paths, in git's order.
rblm_emit_shipped() {
  rblm_perambulation_tracked_capture

  local z_i=0
  local z_path=""

  for z_i in "${!ZRBLM_TRACKED[@]}"; do
    z_path="${ZRBLM_TRACKED[${z_i}]}"
    rblm_perambulation_judge "${z_path}" || continue
    test "${ZRBLM_JUDGMENT}" = "${RBLM_perambulation_ship}" || continue
    printf '%s\n' "${z_path}"
  done
}

######################################################################
# The sweep

# rblm_perambulation_sweep_capture FILE — judge a list of paths as an OBJECT
# GRAPH, not as a tree. Reads paths from FILE, one per line; sets ZRBLM_LEAKS to
# every one the perambulation withholds.
#
# This is the assertion the 2026-07-13 candidate had no version of. Its TIP was
# clean — every strip had landed, and every assay this project owned read the tip
# and passed it — while its HISTORY carried the whole pre-strip repository to the
# remote at 292 MiB. The caller feeds this the candidate's entire object graph
# (git rev-list --objects --all), so a withheld path is caught wherever it is
# reachable from the branch, at any depth.
#
# A FILE, deliberately, not stdin. A caller who pipes into a capture function gets
# a subshell, and the roll it fills dies with it — leaving ZRBLM_LEAKS empty and
# the sweep silently reporting a clean candidate. An empty leak roll IS the
# sweep's success verdict, so the one input form that can fabricate it is the one
# form this must not accept.
#
# Pure over its input: the sweep is a function of the perambulation and a path
# list, so the fixture proves it on a planted list without cloning anything.
rblm_perambulation_sweep_capture() {
  local -r z_file="${1:-}"

  test -n "${z_file}" || buc_die "rblm_perambulation_sweep_capture: no path list given"
  test -f "${z_file}" || buc_die "rblm_perambulation_sweep_capture: no such path list: ${z_file}"

  local z_graph=()
  local z_path=""

  while IFS= read -r z_path || test -n "${z_path}"; do
    test -n "${z_path}" || continue
    z_graph+=("${z_path}")
  done < "${z_file}"

  ZRBLM_LEAKS=()

  local z_i=0
  for z_i in "${!z_graph[@]}"; do
    z_path="${z_graph[${z_i}]}"
    rblm_perambulation_judge "${z_path}" || continue
    test "${ZRBLM_JUDGMENT}" = "${RBLM_perambulation_withhold}" || continue
    ZRBLM_LEAKS+=("${z_path}")
  done
}

# eof
