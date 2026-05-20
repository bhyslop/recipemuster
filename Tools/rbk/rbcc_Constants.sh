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
RBCC_rbrd_file="${RBCC_moorings_dir}/rbrd.env"

# Literal constants (pure string literals, no variable expansion — available at source time)
RBCC_rbrs_file="../station-files/rbrs.env"
RBCC_rbrn_file="rbrn.env"
RBCC_rbra_file="rbra.env"
RBCC_rbro_file="rbro.env"
RBCC_role_governor="governor"
RBCC_role_retriever="retriever"
RBCC_role_director="director"
RBCC_role_payor="payor"
RBCC_role_assay="assay"
RBCC_role_mason="mason"
RBCC_onboarding_nameplate="tadmor"

# Cult-verb tinder — used by SA-management surface (invest/divest/roster) and
# composed into fact-extension constants below.
RBCC_verb_invest="invest"
RBCC_verb_divest="divest"
RBCC_verb_roster="roster"

# Fact-file extension tinder — multi-fact registry for buf_write_fact_multi.
# Producers emit "<basename>.<extension>" via filesystem-as-data-bus pattern;
# consumers walk fact files in BURD_OUTPUT_DIR / BURD_TEMP_DIR keyed on extension.
# Roster extensions composed from earlier tinder (BCG tinder-on-tinder).
RBCC_fact_ext_depot="depot"
RBCC_fact_ext_depot_project="depot-project"
RBCC_fact_ext_roster_retriever="${RBCC_verb_roster}-${RBCC_role_retriever}"
RBCC_fact_ext_roster_director="${RBCC_verb_roster}-${RBCC_role_director}"
RBCC_fact_ext_audit_hallmark="audit-hallmark"

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
