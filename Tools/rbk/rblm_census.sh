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
# RBLM Census - the ship/withhold judgment over every tracked path.
#
# The proscription (rblm_lustrate.sh) judges every enrolled regime FIELD site or
# common. The census judges every tracked PATH ship or withhold. Same discipline,
# one rung out: the delivered file set is a judgment the project makes once, in a
# table, and proves mechanically — never a list of removals recited in prose.
#
# Expede reads this table to decide what to materialize into the candidate, and to
# sweep the candidate's object graph for anything withheld. The census fixture
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
test -z "${ZRBLM_CENSUS_SOURCED:-}" || buc_die "Module rblm_census multiply sourced - check sourcing hierarchy"
ZRBLM_CENSUS_SOURCED=1

######################################################################
# Dispositions

# Ship: the path is delivered. Materialized into the candidate from committed
# bytes, and expected in the candidate's object graph.
RBLM_census_ship="ship"

# Withhold: the path stays behind. Never materialized, and — the assertion that
# matters — never present in the candidate's object graph at all, at any depth of
# its history. A withheld path found in a candidate is a leak, not a stray.
RBLM_census_withhold="withhold"

######################################################################
# The census
#
# Rows are PREFIX|DISPOSITION. Longest matching prefix wins.

ZRBLM_CENSUS=(
  # ── The veiled halves — every kit's closed record ──
  #
  # Finer than the kit rows below, so this holds even where the whole kit ships.
  # A new kit's veiled half is NOT covered here: it lands unjudged and reddens,
  # which is the correct outcome — a kit's exposure is a ruling, not a default.
  "Tools/buk/vov_veiled/|withhold"
  "Tools/cmk/vov_veiled/|withhold"
  "Tools/gad/vov_veiled/|withhold"
  "Tools/jjk/vov_veiled/|withhold"
  "Tools/rbk/vov_veiled/|withhold"
  "Tools/vok/vov_veiled/|withhold"

  # ── The delivered kits ──
  #
  # BUK and RBK ship whole, less their veiled halves above: the .sh surface, the
  # shellcheck config, the READMEs, the agent-context markdown, the theurge crate,
  # the in-pool python step bodies, and the ifrit attack corpus.
  "Tools/buk/|ship"
  "Tools/rbk/|ship"

  # ── The withheld kits ──
  #
  # Whole trees. Every one of these is either the operator's own tooling (jjk, vok,
  # cmk, vvk) or a project that shares the repo but not the delivery (apck, gad,
  # hmk, lmci, vslk). None is uniform-by-accident: each is a tree the consumer has
  # no seat for.
  "Tools/apck/|withhold"
  "Tools/cmk/|withhold"
  "Tools/gad/|withhold"
  "Tools/hmk/|withhold"
  "Tools/jjk/|withhold"
  "Tools/lmci/|withhold"
  "Tools/vok/|withhold"
  "Tools/vslf-rbw/|withhold"
  "Tools/vslk/|withhold"
  "Tools/vvc/|withhold"
  "Tools/vvk/|withhold"

  # ── Loose files under Tools/ — the residue of retired kits ──
  "Tools/cccr.env|withhold"
  "Tools/crgr.render.sh|withhold"
  "Tools/crgv.validate.sh|withhold"
  "Tools/xxx_rbn.info.sh|withhold"

  # ── Tabtargets — file-grain, because tt/ is not uniform ──
  #
  # The marshal family and the manor raze are the release rig itself: they zero,
  # clone, lustrate, feign, expede, and force-delete the manor's pool. A consumer
  # who runs any of them is holding the wrong end of the tool.
  "tt/rbw-M|withhold"
  "tt/rbw-mR.|withhold"
  "tt/rbw-|ship"
  "tt/buw-|ship"
  "tt/z-launcher.sh|ship"
  "tt/apcw-|withhold"
  "tt/jjw-|withhold"
  "tt/study-|withhold"
  "tt/vow-|withhold"
  "tt/vslk-|withhold"
  "tt/vvw-|withhold"

  # ── The moorings — the consumer's config tree, and what it must not carry ──
  #
  # The nameplates all ship: README documents each as an example crucible and the
  # onboarding handbooks walk several. The fdkyclk carve-out is the whole reason
  # this table is file-grain where it has to be — see the matcher note above. Its
  # caged credentials DO ship: the asserter keypair and client secret are committed
  # test scaffolding the realm expects by value (RBSFK "two-keys"), and stripping
  # them breaks the test bed's determinism. A secret scanner will flag them; the
  # README's fdkyclk caution is the answer, not removal.
  "rbmm_moorings/fdkyclk/fdkyclk-proof.sh|withhold"
  "rbmm_moorings/fdkyclk/fdkyclk-teardown.sh|withhold"
  "rbmm_moorings/fdkyclk/|ship"
  "rbmm_moorings/ccyolo/|ship"
  "rbmm_moorings/moriah/|ship"
  "rbmm_moorings/nineveh/|ship"
  "rbmm_moorings/pluml/|ship"
  "rbmm_moorings/srjcl/|ship"
  "rbmm_moorings/tadmor/|ship"
  "rbmm_moorings/rbmf_foedera/|ship"
  "rbmm_moorings/rbmv_vessels/|ship"
  "rbmm_moorings/burc.env|ship"
  "rbmm_moorings/rbrd.env|ship"
  "rbmm_moorings/rbrp.env|ship"
  "rbmm_moorings/rbrr.env|ship"
  "rbmm_moorings/rbrw.env|ship"

  # The operator's remote machines, by name and by account. Nothing here is the
  # consumer's.
  "rbmm_moorings/rbmn_nodes/|withhold"
  "rbmm_moorings/rbmu_users/|withhold"

  # Launchers ship only for the workbenches that ship — file-grain, same reason as
  # tt/: a launcher for a withheld workbench is a dangling reference at best.
  "rbmm_moorings/rbml_launchers/launcher.buw_workbench.sh|ship"
  "rbmm_moorings/rbml_launchers/launcher.rbw_workbench.sh|ship"
  "rbmm_moorings/rbml_launchers/|withhold"

  # ── The operator's own trees ──
  "Memos/|withhold"
  "Study/|withhold"
  ".claude/|withhold"
  ".idea/|withhold"
  ".jjk/|withhold"
  "_slickedit/|withhold"

  # ── The delivered face ──
  "README.md|ship"
  "CLAUDE.md|ship"
  "LICENSE|ship"
  "diagrams/|ship"
  "rbm-abstract-drawio.svg|ship"
  ".gitattributes|ship"
  ".gitignore|ship"

  # ── Root files that stay behind ──
  #
  # RELEASE.md is the release-qualification procedure: the rig's own runbook.
  # The MCP config names the operator's servers; the iml is IDE furniture; the
  # gateway proposal is an unsettled internal design note.
  "RELEASE.md|withhold"
  ".mcp.json|withhold"
  "brm_recipemuster.iml|withhold"
  "podman-gateway-proposal.md|withhold"
)

######################################################################
# The judgment

# rblm_census_judge PATH — rule one repo-relative path.
#
# Sets ZRBLM_JUDGMENT to the winning disposition and ZRBLM_JUDGED_BY to the row
# that won. Returns 0 when a row matched, 1 when the path is UNJUDGED — the state
# the whole table exists to make loud. An unjudged path is never treated as either
# disposition: there is no default in either direction.
rblm_census_judge() {
  local -r z_path="${1:-}"

  ZRBLM_JUDGMENT=""
  ZRBLM_JUDGED_BY=""

  local z_row=""
  local z_prefix=""
  local z_best=-1

  for z_row in "${ZRBLM_CENSUS[@]}"; do
    z_prefix="${z_row%%|*}"
    test "${z_path#"${z_prefix}"}" != "${z_path}" || continue
    test "${#z_prefix}" -gt "${z_best}"           || continue
    z_best="${#z_prefix}"
    ZRBLM_JUDGMENT="${z_row#*|}"
    ZRBLM_JUDGED_BY="${z_prefix}"
  done

  test "${z_best}" -ge 0 || return 1
  return 0
}

# rblm_census_tracked_capture — the live tracked set, from git, not from a list.
#
# The census is judged against what the repository actually carries at this
# commit. Any other source of truth is a second copy waiting to drift.
rblm_census_tracked_capture() {
  ZRBLM_TRACKED=()

  local z_path=""
  while IFS= read -r z_path; do
    test -n "${z_path}" || continue
    ZRBLM_TRACKED+=("${z_path}")
  done < <(git ls-files)
}

######################################################################
# Emitters — the fixture's reach
#
# Tab-separated, one row per line, no decoration. The census fixture sources this
# module and calls these, so the table it judges against is the same table expede
# ships from. One matcher, in bash, read by both — never a second implementation
# in Rust to drift against this one.

# rblm_emit_verdicts — one line per tracked path: DISPOSITION \t PATH.
#
# An unjudged path is emitted with the literal disposition "unjudged", not
# omitted: the fixture must see it to redden on it.
rblm_emit_verdicts() {
  rblm_census_tracked_capture

  local z_path=""
  for z_path in "${ZRBLM_TRACKED[@]}"; do
    if rblm_census_judge "${z_path}"; then
      printf '%s\t%s\n' "${ZRBLM_JUDGMENT}" "${z_path}"
    else
      printf 'unjudged\t%s\n' "${z_path}"
    fi
  done
}

# rblm_emit_dead_rows — every census row that wins for no tracked path.
#
# Two failures wear this one face. A STALE row judges a path that no longer
# exists (the ceremony's strip lists still name RBM-nameplates/, index.html, and
# a file called wsl@rocket — none of which the tree has carried for some time).
# A SHADOWED row is outranked everywhere by a longer one, so its judgment never
# lands. Both mean the table is lying about the tree, and both go red.
rblm_emit_dead_rows() {
  rblm_census_tracked_capture

  # Newline-delimited winner set, not an associative array: bash 3.2 is the floor
  # (the operator's macOS station ships it), and this table is small enough that a
  # linear membership test over a string costs nothing worth naming.
  local z_won=""
  local z_path=""
  local z_row=""
  local z_prefix=""

  for z_path in "${ZRBLM_TRACKED[@]}"; do
    rblm_census_judge "${z_path}" || continue
    case $'\n'"${z_won}" in
      *$'\n'"${ZRBLM_JUDGED_BY}"$'\n'*) ;;
      *) z_won="${z_won}${ZRBLM_JUDGED_BY}"$'\n' ;;
    esac
  done

  for z_row in "${ZRBLM_CENSUS[@]}"; do
    z_prefix="${z_row%%|*}"
    case $'\n'"${z_won}" in
      *$'\n'"${z_prefix}"$'\n'*) continue ;;
    esac
    printf '%s\t%s\n' "${z_prefix}" "${z_row#*|}"
  done
}

# rblm_emit_shipped — every tracked path the census ships, one per line.
#
# Expede's materialization list. Ordinary paths, in git's order.
rblm_emit_shipped() {
  rblm_census_tracked_capture

  local z_path=""
  for z_path in "${ZRBLM_TRACKED[@]}"; do
    rblm_census_judge "${z_path}" || continue
    test "${ZRBLM_JUDGMENT}" = "${RBLM_census_ship}" || continue
    printf '%s\n' "${z_path}"
  done
}

######################################################################
# The sweep
#
# rblm_census_sweep_capture FILE — judge a list of paths as an OBJECT GRAPH, not
# as a tree. Reads paths from FILE, one per line; sets ZRBLM_LEAKS to every one the
# census withholds.
#
# This is the assertion the 2026-07-13 candidate failed. Its TIP was clean — every
# strip had landed, every tip-reading assay passed it — and it went to the remote
# at 292 MiB because the private objects rode in its HISTORY, reachable from the
# branch, invisible to anything that looked only at the face. The caller feeds this
# the candidate's whole object graph (git rev-list --objects --all), so a withheld
# path is caught wherever in the graph it is reachable from, at any depth.
#
# A FILE, deliberately, not stdin. A caller who pipes into a capture function gets
# a subshell, and the array it fills dies with it — leaving ZRBLM_LEAKS empty and
# the sweep silently reporting a clean candidate. The self-proof caught exactly
# that on its first run. An empty leak list is the sweep's success verdict, so the
# one input form that can fabricate it is the one form this must not accept.
#
# Pure over its input: the sweep is a function of the census and a path list, so
# the fixture proves it on a planted list without cloning anything.
rblm_census_sweep_capture() {
  local -r z_file="${1:-}"

  test -n "${z_file}" || buc_die "rblm_census_sweep_capture: no path list given"
  test -f "${z_file}" || buc_die "rblm_census_sweep_capture: no such path list: ${z_file}"

  ZRBLM_LEAKS=()

  local z_path=""
  while IFS= read -r z_path; do
    test -n "${z_path}" || continue
    rblm_census_judge "${z_path}" || continue
    test "${ZRBLM_JUDGMENT}" = "${RBLM_census_withhold}" || continue
    ZRBLM_LEAKS+=("${z_path}")
  done < "${z_file}"
}
