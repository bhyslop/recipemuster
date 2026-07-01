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
RBCC_foedera_subdir="rbmf_foedera"
RBCC_rbrr_file="${RBCC_moorings_dir}/rbrr.env"
RBCC_rbrp_file="${RBCC_moorings_dir}/rbrp.env"
RBCC_rbrm_file="${RBCC_moorings_dir}/rbrm.env"
# Federation regime file — the ACTIVE foedus's rbrf.env. The foedera library
# (RBCC_foedera_subdir) holds one rbef_ subdirectory per standing foedus, the
# active one selected by RBRR_ACTIVE_FOEDUS; the configuration is stored once
# with no copied active file (RBSRF). DEGENERATE single-foedus resolution:
# with only rbef_entrada standing, the active path is constant-folded to it
# here. The selector-derived form (.../${RBRR_ACTIVE_FOEDUS}/rbrf.env) lands
# with the deferred federation family-of-named-instances rework; until then
# instate writes the selector and descry resolves any named foedus directly,
# but the singleton accessor reads this constant unchanged.
RBCC_rbrf_file="${RBCC_moorings_dir}/${RBCC_foedera_subdir}/rbef_entrada/rbrf.env"
RBCC_rbrd_basename="rbrd.env"
RBCC_rbrd_file="${RBCC_moorings_dir}/${RBCC_rbrd_basename}"

# Literal constants (pure string literals, no variable expansion — available at source time)
RBCC_rbrn_file="rbrn.env"
RBCC_rbro_file="rbro.env"

# Account composition labels — bare fragments that compose GCP SA account-ids/
# emails AND local secret-directory names. These stay bare: a derived
# resource-name string in cloud/filesystem space, structurally like the SA
# email and the -tether/-airgap pool suffixes the heat keeps bare.
RBCC_account_governor="governor"
RBCC_account_retriever="retriever"
RBCC_account_director="director"
RBCC_account_payor="payor"
RBCC_account_mason="mason"

# Mantle service-account names — the three impersonatable federation identities
# (governor / director / retriever) established at depot levy. Hardcoded literals
# for grep. The rbma- prefix is hyphenated because a GCP service-account id admits
# only lowercase letters, digits, and hyphens (RFC1035) — the underscore sprue form
# cannot appear in this field; grep rbma still finds all three.
RBCC_account_mantle_governor="rbma-governor"
RBCC_account_mantle_director="rbma-director"
RBCC_account_mantle_retriever="rbma-retriever"

# Mantle identity tokens — THE canonical name for "which mantle to don", carried
# by every credential-mint surface (rba_token_capture / rba_don_capture, the
# rbw-am folio, and the theurge patrol) as one form, never a bare-vs-sprued
# two-form. The VALUE carries the pallium value-sprue rbpa_ so an identity token
# is self-typing under grep and can never be mistaken for an SA-id fragment: the
# underscore the sprue mandates is forbidden in a GCP SA id (RFC1035), so the
# token resolves to the mantle SA only through the rba_don_capture case and the
# raw sprued form never reaches a resource name. Distinct from the bare
# RBCC_account_mantle_* SA-name fragments above (which compose rbma-<role>@… SA
# emails); the polity/terrier bare-mantle-name uses are a separate deferred
# migration and intentionally keep the bare role word.
RBCC_mantle_governor="rbpa_governor"
RBCC_mantle_director="rbpa_director"
RBCC_mantle_retriever="rbpa_retriever"
RBCC_onboarding_nameplate="tadmor"

# Operation-verb tinder — the canonical bash home for RBK operation verbs.
# Members are bare verb tokens; the group carries one author here so it is
# projectable under the single-canonical-author rule. Two surfaces:
#
#   SA-management (defrock/enrobe/roster) — composed into the fact-extension
#   constants below; consumed by the governor/director account surface.
RBCC_verb_defrock="defrock"
RBCC_verb_enrobe="enrobe"
RBCC_verb_roster="roster"
#
#   Image/build lifecycle (anoint/inscribe/kludge/ordain/yoke) — name the
#   registry and build operations. Previously implicit in command-function
#   names (rbrd_inscribe, rbfd_ordain, …) and tabtarget descriptions; homed
#   here so the group has a single owner rather than being reconstructed by
#   grep across rbfd_/rbfl_/rbfk_/rbob_.
RBCC_verb_anoint="anoint"
RBCC_verb_inscribe="inscribe"
RBCC_verb_kludge="kludge"
RBCC_verb_ordain="ordain"
RBCC_verb_yoke="yoke"
#
#   Clean-tree-gated operation verbs (the federation founding verb affiance, the
#   Lode captures conclave/ensconce/immure/underpin, and the Director builds
#   conjure/mirror) — each announces its name through bug_require_clean_tree;
#   homed here so the guard sites reference the verb rather than a bare literal.
RBCC_verb_affiance="affiance"
RBCC_verb_conclave="conclave"
RBCC_verb_conjure="conjure"
RBCC_verb_ensconce="ensconce"
RBCC_verb_immure="immure"
RBCC_verb_mirror="mirror"
RBCC_verb_underpin="underpin"

# Creed tinder — RB convictions supplied as the rationale parameter to a
# kit-agnostic BUG gate, keeping the opinion RB-side and out of BUK. The
# clean-build creed rides bug_require_clean_tree_creed: RB refuses to build a
# container image from an uncommitted tree because the image cannot then be
# traced to a commit. Consumed only by bash gate sites (no theurge assertion),
# so it is not projected to the Rust band; no call site is wired yet.
RBCC_creed_clean_build="a container image built from an uncommitted tree cannot be traced to a commit; commit before building"

# Fact-file extension tinder — multi-fact registry for buf_write_fact_multi.
# Producers emit "<basename>.<extension>" via filesystem-as-data-bus pattern;
# consumers walk fact files in BURD_OUTPUT_DIR / BURD_TEMP_DIR keyed on extension.
# Roster extensions composed from earlier tinder (BCG tinder-on-tinder).
RBCC_fact_ext_depot="depot"
RBCC_fact_ext_depot_project="depot-project"
RBCC_fact_ext_roster_retriever="${RBCC_verb_roster}-${RBCC_account_retriever}"
RBCC_fact_ext_roster_director="${RBCC_verb_roster}-${RBCC_account_director}"
RBCC_fact_ext_audit_hallmark="audit-hallmark"
# Foedus descry health verdict — descry writes <foedus>.foedus-health carrying
# one of healthy / pool-absent / pool-deleted / provider-absent for the
# reuse-or-establish fixture to branch on (reuse iff healthy).
RBCC_fact_ext_foedus_health="foedus-health"

# Tweak-name tinder — RB-owned BURE_TWEAK_NAME values (buo sprue, BUS0 Tweak
# Mechanism). The credless guard is the reveille-tier slot reservation: theurge
# sets it on every tabtarget a reveille-tier fixture spawns, and the Payor OAuth
# token-mint membrane (zrbgp_authenticate_capture) rejects under it with
# BUBC_band_credless — a passing reveille run can never use credentials.
RBCC_tweak_credless_guard="buorb_credless_guard"

# Container-role tinder — the canonical bash home for the crucible's container
# roles. Bare role tokens; the crucible is sentry + pentacle + bottle and every
# container name / compose service derives from these. Distinct from the
# RBCC_account_* composition labels above (bare fragments for SA names + secret
# dirs). None of these words are reused across families, keeping each token
# monosemous.
RBCC_container_bottle="bottle"
RBCC_container_pentacle="pentacle"
RBCC_container_sentry="sentry"

######################################################################
# Rust const projection (rbcc single-homed set → RBTDGC_)

# rbcc_emit_consts() - Emit the RBCC-owned co-maintained constants as Rust
# string consts to stdout, one `pub const` line per name/value pair via the
# shared buz_emit_const primitive (BUK must be kindled). The single-homed set:
# moorings/vessels dirs, account labels, mantle identity tokens, .env filenames,
# operation verbs, and container roles. Each Rust const is
# RBTDGC_ + the RBCC stem (RBCC_ prefix stripped) uppercased; the value is
# carried verbatim. Bash stays mixed-case (RBCC_moorings_dir); the generated
# Rust is SCREAMING (RBTDGC_MOORINGS_DIR) per Rust convention — that casing is
# the sole transform, applied mechanically, so there is no per-entry mapping
# and no drift. rbtd's lib.rs paths, the manifest account-label mirror, and the
# rbtdrk/rbtdrp .env consts all source these instead of hand-copying.
# A second section projects the BUBC precision exit-code band as i32 consts
# (same mechanical transform, BUBC_ prefix stripped) — theurge asserts exit
# codes as integers. bubc is sourced by the launcher on every dispatch, so the
# band values are present at emission time with no cross-module tinder trick.
# A third section projects BUBC string tinder (the regime-poison tweak name)
# through the same transform via the string primitive.
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
    RBCC_account_mason      \
    RBCC_mantle_governor    \
    RBCC_mantle_director    \
    RBCC_mantle_retriever   \
    RBCC_rbrr_file       \
    RBCC_rbrp_file       \
    RBCC_rbrm_file       \
    RBCC_rbrf_file       \
    RBCC_rbrd_basename   \
    RBCC_rbrd_file       \
    RBCC_rbrn_file       \
    RBCC_rbro_file       \
    RBCC_foedera_subdir  \
    RBCC_fact_ext_foedus_health \
    RBCC_verb_defrock     \
    RBCC_verb_enrobe     \
    RBCC_verb_roster     \
    RBCC_verb_anoint     \
    RBCC_verb_inscribe   \
    RBCC_verb_kludge     \
    RBCC_verb_ordain     \
    RBCC_verb_yoke       \
    RBCC_container_bottle    \
    RBCC_container_pentacle  \
    RBCC_container_sentry    \
    RBCC_tweak_credless_guard \
  ; do
    z_stem="${z_name#RBCC_}"
    z_upper="$(printf '%s' "${z_stem}" | tr '[:lower:]' '[:upper:]')"
    buz_emit_const "RBTDGC_${z_upper}" "${!z_name}" \
      || buc_die "rbcc_emit_consts: emit failed for ${z_name}"
  done

  printf '%s\n' ""
  printf '%s\n' "// BUBC precision exit-code band (bubc_constants.sh) — numeric"
  for z_name in \
    BUBC_band_base      \
    BUBC_band_width     \
    BUBC_band_regime    \
    BUBC_band_enroll    \
    BUBC_band_recipe    \
    BUBC_band_hygiene   \
    BUBC_band_credless  \
    BUBC_band_chain     \
    BUBC_band_descry    \
    BUBC_band_instate   \
    BUBC_band_admission \
    BUBC_band_vacant    \
    BUBC_band_selftest  \
  ; do
    z_stem="${z_name#BUBC_}"
    z_upper="$(printf '%s' "${z_stem}" | tr '[:lower:]' '[:upper:]')"
    buz_emit_const_i32 "RBTDGC_${z_upper}" "${!z_name}" \
      || buc_die "rbcc_emit_consts: emit failed for ${z_name}"
  done

  printf '%s\n' ""
  printf '%s\n' "// BUBC regime-poison tweak (bubc_constants.sh) — string"
  z_name="BUBC_tweak_regime_poison"
  z_stem="${z_name#BUBC_}"
  z_upper="$(printf '%s' "${z_stem}" | tr '[:lower:]' '[:upper:]')"
  buz_emit_const "RBTDGC_${z_upper}" "${!z_name}" \
    || buc_die "rbcc_emit_consts: emit failed for ${z_name}"
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
