#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
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
# Recipe Bottle Common Constants - File paths and naming conventions

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBCC_SOURCED:-}" || buc_die "Module rbcc multiply sourced - check sourcing hierarchy"
ZRBCC_SOURCED=1

# Kit directory — source-time self-location. rbcc lives at the RBK kit root,
# so its own directory IS the kit dir. No launcher environment dependency;
# available the instant this file is sourced (BCG kit-self-location pattern,
# canon: rbtd/rbte_cli.sh).
readonly RBCC_KIT_DIR="${BASH_SOURCE[0]%/*}"

# Generated zipper-derived artifacts — the theurge build materializes both
# (write-on-change), rbq's gates verify them. Absolute, composed from the
# readonly kit dir (source-time, no kindle dependency).
readonly RBCC_rbtdgc_consts_file="${RBCC_KIT_DIR}/rbtd/src/rbtdgc_consts.rs"
readonly RBCC_tabtarget_context_file="${RBCC_KIT_DIR}/claude-rbk-tabtarget-context.md"

# ── Moorings inventory constants ──────────────────────────────────────────
# Moorings-relative path values. Every consumer reads these names directly
# (the transitional RBBC_* aliases were retired by the literal-sweep pace).
# Source-time literals, no kindle dependency.
RBCC_moorings_dir="rbmm_moorings"
RBCC_launchers_subdir="rbml_launchers"
RBCC_users_subdir="rbmu_users"
RBCC_nodes_subdir="rbmn_nodes"
RBCC_vessels_subdir="rbmv_vessels"
RBCC_rbrr_file="${RBCC_moorings_dir}/rbrr.env"
RBCC_rbrp_file="${RBCC_moorings_dir}/rbrp.env"
RBCC_rbrm_file="${RBCC_moorings_dir}/rbrm.env"
RBCC_rbrd_basename="rbrd.env"
RBCC_rbrd_file="${RBCC_moorings_dir}/${RBCC_rbrd_basename}"

# Literal constants (pure string literals, no variable expansion — available at source time)
RBCC_rbrs_file="../station-files/rbrs.env"
RBCC_rbrn_file="rbrn.env"
RBCC_rbra_file="rbra.env"
RBCC_rbro_file="rbro.env"
# Auth-role enum (rbnae_): the RBRA_ROLE value — minted sprue, RBRA_ROLE domain
# (3 members). Written into credential files, validated, carried as the auth
# folio, swizzle-checked against BUZ_FOLIO. Distinct axis from RBCC_account_*
# below: that one is the bare composition label.
RBCC_role_governor="rbnae_governor"
RBCC_role_retriever="rbnae_retriever"
RBCC_role_director="rbnae_director"

# Account composition labels — bare fragments that compose GCP SA account-ids/
# emails AND local secret-directory names (6 members, superset of the 3 roles
# plus payor/assay/mason which are NEVER RBRA_ROLE values). These stay bare:
# a derived resource-name string in cloud/filesystem space, structurally like
# the SA email and the -tether/-airgap pool suffixes the heat keeps bare.
RBCC_account_governor="governor"
RBCC_account_retriever="retriever"
RBCC_account_director="director"
RBCC_account_payor="payor"
RBCC_account_assay="assay"
RBCC_account_mason="mason"
RBCC_onboarding_nameplate="tadmor"

# Operation-verb tinder — the canonical bash home for RBK operation verbs.
# Members are bare verb tokens; the group carries one author here so it is
# projectable under the single-canonical-author rule. Two surfaces:
#
#   SA-management (divest/invest/roster) — composed into the fact-extension
#   constants below; consumed by the governor/director account surface.
RBCC_verb_divest="divest"
RBCC_verb_invest="invest"
RBCC_verb_roster="roster"
#
#   Image/build lifecycle (inscribe/kludge/ordain/yoke) — name the
#   registry and build operations. Previously implicit in command-function
#   names (rbrd_inscribe, rbfd_ordain, …) and tabtarget descriptions; homed
#   here so the group has a single owner rather than being reconstructed by
#   grep across rbfd_/rbfl_/rbfk_/rbob_.
RBCC_verb_inscribe="inscribe"
RBCC_verb_kludge="kludge"
RBCC_verb_ordain="ordain"
RBCC_verb_yoke="yoke"

# Fact-file extension tinder — multi-fact registry for buf_write_fact_multi.
# Producers emit "<basename>.<extension>" via filesystem-as-data-bus pattern;
# consumers walk fact files in BURD_OUTPUT_DIR / BURD_TEMP_DIR keyed on extension.
# Roster extensions composed from earlier tinder (BCG tinder-on-tinder).
RBCC_fact_ext_depot="depot"
RBCC_fact_ext_depot_project="depot-project"
RBCC_fact_ext_roster_retriever="${RBCC_verb_roster}-${RBCC_account_retriever}"
RBCC_fact_ext_roster_director="${RBCC_verb_roster}-${RBCC_account_director}"
RBCC_fact_ext_audit_hallmark="audit-hallmark"

# Container-role tinder — the canonical bash home for the crucible's container
# roles. Bare role tokens; the crucible is sentry + pentacle + bottle and every
# container name / compose service derives from these. Distinct from the
# credential auth-role axis above, which itself splits in two: RBCC_role_*
# (minted enum, the RBRA_ROLE value) and RBCC_account_* (bare composition label
# for SA names + secret dirs). None of these words are reused across families,
# keeping each token monosemous.
RBCC_container_bottle="bottle"
RBCC_container_pentacle="pentacle"
RBCC_container_sentry="sentry"

######################################################################
# Rust const projection (rbcc single-homed set → RBTDGC_)

# rbcc_emit_consts() - Emit the RBCC-owned co-maintained constants as Rust
# string consts to stdout, one `pub const` line per name/value pair via the
# shared buz_emit_const primitive (BUK must be kindled). The single-homed set:
# moorings/vessels dirs, account labels, .env filenames, operation verbs, and
# container roles. Each Rust const is
# RBTDGC_ + the RBCC stem (RBCC_ prefix stripped) uppercased; the value is
# carried verbatim. Bash stays mixed-case (RBCC_moorings_dir); the generated
# Rust is SCREAMING (RBTDGC_MOORINGS_DIR) per Rust convention — that casing is
# the sole transform, applied mechanically, so there is no per-entry mapping
# and no drift. rbtd's lib.rs paths, the manifest account-label mirror, and the
# rbtdrk/rbtdrp .env consts all source these instead of hand-copying.
rbcc_emit_consts() {
  printf '%s\n' "// RBCC constants (rbcc_constants.sh single-homed set)"

  local z_name=""
  local z_stem=""
  local z_upper=""
  for z_name in \
    RBCC_moorings_dir    \
    RBCC_vessels_subdir  \
    RBCC_account_governor   \
    RBCC_account_retriever  \
    RBCC_account_director   \
    RBCC_account_payor      \
    RBCC_account_assay      \
    RBCC_account_mason      \
    RBCC_rbrr_file       \
    RBCC_rbrp_file       \
    RBCC_rbrm_file       \
    RBCC_rbrd_basename   \
    RBCC_rbrd_file       \
    RBCC_rbrs_file       \
    RBCC_rbrn_file       \
    RBCC_rbra_file       \
    RBCC_rbro_file       \
    RBCC_verb_divest     \
    RBCC_verb_invest     \
    RBCC_verb_roster     \
    RBCC_verb_inscribe   \
    RBCC_verb_kludge     \
    RBCC_verb_ordain     \
    RBCC_verb_yoke       \
    RBCC_container_bottle    \
    RBCC_container_pentacle  \
    RBCC_container_sentry    \
  ; do
    z_stem="${z_name#RBCC_}"
    z_upper="$(printf '%s' "${z_stem}" | tr '[:lower:]' '[:upper:]')"
    buz_emit_const "RBTDGC_${z_upper}" "${!z_name}" \
      || buc_die "rbcc_emit_consts: emit failed for ${z_name}"
  done
}

######################################################################
# Internal Functions (zrbcc_*)

zrbcc_kindle() {
  test -z "${ZRBCC_KINDLED:-}" || buc_die "Module rbcc already kindled"

  # Curl timeout bounds — all actionable curl sites use these
  readonly RBCC_CURL_CONNECT_TIMEOUT_SEC=10
  readonly RBCC_CURL_MAX_TIME_SEC=60

  readonly ZRBCC_KINDLED=1
}

zrbcc_sentinel() {
  test "${ZRBCC_KINDLED:-}" = "1" || buc_die "Module rbcc not kindled - call zrbcc_kindle first"
}

# eof
