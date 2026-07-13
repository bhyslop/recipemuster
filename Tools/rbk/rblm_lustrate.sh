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
# RBLM Lustrate - the proscription and the sterilizing transform.
#
# The proscription names, for every enrolled regime field and every hardpoint
# constant, whether the value is the SITE's (this station's cloud, this
# operator's federated identity) or COMMON to every installation. Lustration
# writes the sterile value over every site-scoped home inside the release
# ceremony's throwaway clone; the damnatio fixture then proves, on the delivered
# tree, that nothing site-scoped survived.
#
# The proscription is the single home of that judgment. Both the transform below
# and the fixture read THIS table — the fixture reaches bash for it rather than
# carrying a second copy in Rust, so the two can never drift.
#
# Deliberately NOT marshal zero. Zero mints the onboarding-start baseline the
# gauntlet's entry contract reads, and it runs against the operator's working
# station; lustration runs only in the ceremony's clone and touches homes zero
# must never disturb (the payor's own project, the manor's pool). Their site
# rows overlap by construction — lustration rewrites every site row, including
# the ones zero already blanked — so the fixture's value case proves BOTH verbs
# in one assertion and a regression in either goes red.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBLM_LUSTRATE_SOURCED:-}" || buc_die "Module rblm_lustrate multiply sourced - check sourcing hierarchy"
ZRBLM_LUSTRATE_SOURCED=1

######################################################################
# Dispositions

# Site-scoped: the value names this station. Erased at lustration, asserted
# sterile by the fixture.
RBLM_disposition_site="site"

# Common: the value is the same at every installation (a default, a mode word, a
# port, an upstream image reference). Untouched, and asserted by nothing — but a
# field must still be declared common to satisfy the completeness case.
RBLM_disposition_common="common"

# Off-tree: the field is enrolled, but its regime file lives outside the shipping
# tree, so no delivered byte can carry it. The payor's OAuth secret and refresh
# token are the case: they live in the operator's secrets directory, named by
# RBRR_SECRETS_DIR, which is not in the repository at all. Declared rather than
# omitted, so the completeness case still accounts for every enrolled field.
RBLM_disposition_offtree="offtree"

######################################################################
# The proscription
#
# Rows are SCOPE|VARNAME|DISPOSITION|STERILE_VALUE, one per enrolled regime
# field. STERILE_VALUE is read only for site rows; an empty one blanks the field
# to the onboarding-start state, which is what a consumer must fill in anyway.
#
# A NON-EMPTY sterile value is used only where a blank would break the delivered
# tree — never for cosmetics. Today that is RBRR_PUBLIC_DOCS_URL, whose whole
# purpose is to be a live URL in the consumer's hands.
#
# Every field enrolled by Tools/rbk/rbr*_regime.sh must appear here exactly once.
# The damnatio fixture derives the enrolled set from the LIVE enrollment rolls
# and reddens on any field this table has not judged — so a newly enrolled field
# cannot ship undeclared, and this list cannot silently fall behind.

# The delivery documentation base. The working repo's RBRR_PUBLIC_DOCS_URL points
# at the maintainer's development repo; the delivered tree must point consumers at
# the public home. Recorded as a delivery decision (2026-07-12): the public repo's
# README blob, because blob rendering preserves the literal <a id> anchors the
# handbook links resolve, while staging and candidate branches are transient.
RBLM_public_docs_url="https://github.com/scaleinv/recipebottle/blob/main/README.md"

# The freehold subject's sterile value. NOT a blank: rbpc_emit_consts projects
# this constant into the generated Rust (RBTDGC_FREEHOLD_SUBJECT) through
# buz_emit_const, which rejects an empty value — so a blank here dies at the
# candidate's own const regeneration. A placeholder that is shape-free (no UUID
# for the fixture's sweep to find) and obviously unset is the form that survives.
RBLM_unset_subject="unset-freehold-subject"

ZRBLM_PROSCRIPTION=(
  # ── RBRR — repo regime ──
  "RBRR|RBRR_RUNTIME_PREFIX|site|"
  "RBRR|RBRR_VESSEL_DIR|common|"
  "RBRR|RBRR_BOTTLE_WORKSPACE|common|"
  "RBRR|RBRR_DNS_SERVER|common|"
  "RBRR|RBRR_GCB_TIMEOUT|common|"
  "RBRR|RBRR_GCB_MIN_CONCURRENT_BUILDS|common|"
  "RBRR|RBRR_SECRETS_DIR|common|"
  "RBRR|RBRR_PUBLIC_DOCS_URL|site|${RBLM_public_docs_url}"
  "RBRR|RBRR_ACTIVE_FOEDUS|common|"

  # ── RBRD — depot regime ──
  "RBRD|RBRD_CLOUD_PREFIX|site|"
  "RBRD|RBRD_DEPOT_MONIKER|site|"
  "RBRD|RBRD_GCP_REGION|common|"
  "RBRD|RBRD_GCB_MACHINE_TYPE|common|"

  # ── RBRP — payor regime ──
  "RBRP|RBRP_PAYOR_PROJECT_ID|site|"
  "RBRP|RBRP_BILLING_ACCOUNT_ID|site|"
  "RBRP|RBRP_OAUTH_CLIENT_ID|site|"
  "RBRP|RBRP_OPERATOR_EMAIL|site|"

  # ── RBRW — workforce regime ──
  "RBRW|RBRW_ORG_ID|site|"
  "RBRW|RBRW_WORKFORCE_POOL_ID|site|"
  "RBRW|RBRW_SESSION_DURATION|common|"

  # ── RBRF — federation regime ──
  # The interactive fields name the operator's own IdP tenant and app
  # registration; the programmatic fields name only the caged Keycloak realm the
  # test facility ships, which is synthetic at every installation.
  "RBRF|RBRF_PROVIDER_ID|site|"
  "RBRF|RBRF_MECHANISM|common|"
  "RBRF|RBRF_IDP_ISSUER|site|"
  "RBRF|RBRF_IDP_CLIENT_ID|site|"
  "RBRF|RBRF_ATTRIBUTE_MAPPING|common|"
  "RBRF|RBRF_IDP_SCOPE|common|"
  "RBRF|RBRF_IDP_DEVICE_ENDPOINT|site|"
  "RBRF|RBRF_IDP_TOKEN_ENDPOINT|site|"
  "RBRF|RBRF_IDP_JWKS_JSON|common|"
  "RBRF|RBRF_GRANT_ENDPOINT|common|"
  "RBRF|RBRF_ASSERTER_KEY_FILE|common|"
  "RBRF|RBRF_CLIENT_SECRET_FILE|common|"
  "RBRF|RBRF_ASSERTER_KID|common|"
  "RBRF|RBRF_ASSERTER_ISSUER|common|"
  "RBRF|RBRF_ASSERTER_SUBJECT|common|"

  # ── RBRO — payor OAuth regime ──
  "RBRO|RBRO_CLIENT_SECRET|offtree|"
  "RBRO|RBRO_REFRESH_TOKEN|offtree|"

  # ── RBRV — vessel regime ──
  # The origins are upstream vendor references, common to every installation; the
  # anchors, the reliquary and the graft image name Lodes and images in THIS
  # station's depot registry.
  "RBRV|RBRV_SIGIL|common|"
  "RBRV|RBRV_DESCRIPTION|common|"
  "RBRV|RBRV_USER|common|"
  "RBRV|RBRV_VESSEL_MODE|common|"
  "RBRV|RBRV_RELIQUARY|site|"
  "RBRV|RBRV_EGRESS_MODE|common|"
  "RBRV|RBRV_BIND_IMAGE|common|"
  "RBRV|RBRV_BIND_OPTIONAL_DOCKERFILE|common|"
  "RBRV|RBRV_CONJURE_DOCKERFILE|common|"
  "RBRV|RBRV_CONJURE_BLDCONTEXT|common|"
  "RBRV|RBRV_CONJURE_PLATFORMS|common|"
  "RBRV|RBRV_IMAGE_1_ORIGIN|common|"
  "RBRV|RBRV_IMAGE_1_ANCHOR|site|"
  "RBRV|RBRV_IMAGE_2_ORIGIN|common|"
  "RBRV|RBRV_IMAGE_2_ANCHOR|site|"
  "RBRV|RBRV_IMAGE_3_ORIGIN|common|"
  "RBRV|RBRV_IMAGE_3_ANCHOR|site|"
  "RBRV|RBRV_GRAFT_IMAGE|site|"
  "RBRV|RBRV_GRAFT_OPTIONAL_DOCKERFILE|common|"

  # ── RBRN — nameplate regime ──
  "RBRN|RBRN_MONIKER|common|"
  "RBRN|RBRN_DESCRIPTION|common|"
  "RBRN|RBRN_RUNTIME|common|"
  "RBRN|RBRN_SENTRY_VESSEL|common|"
  "RBRN|RBRN_BOTTLE_VESSEL|common|"
  "RBRN|RBRN_SENTRY_HALLMARK|site|"
  "RBRN|RBRN_BOTTLE_HALLMARK|site|"
  "RBRN|RBRN_BOTTLE_READINESS_DELAY_SEC|common|"
  "RBRN|RBRN_ENTRY_MODE|common|"
  "RBRN|RBRN_ENTRY_PORT_WORKSTATION|common|"
  "RBRN|RBRN_ENTRY_PORT_ENCLAVE|common|"
  "RBRN|RBRN_ENCLAVE_BASE_IP|common|"
  "RBRN|RBRN_ENCLAVE_NETMASK|common|"
  "RBRN|RBRN_ENCLAVE_SENTRY_IP|common|"
  "RBRN|RBRN_ENCLAVE_BOTTLE_IP|common|"
  "RBRN|RBRN_UPLINK_PORT_MIN|common|"
  "RBRN|RBRN_UPLINK_DNS_MODE|common|"
  "RBRN|RBRN_UPLINK_ACCESS_MODE|common|"
  "RBRN|RBRN_UPLINK_ALLOWED_DOMAINS|common|"
  "RBRN|RBRN_UPLINK_ALLOWED_CIDRS|common|"
)

######################################################################
# The hardpoints
#
# Site identity that lives in source rather than in a regime file, so no
# enrollment roll enumerates it and the completeness case cannot reach it. Rows
# are PATH|VARNAME|STERILE_VALUE, repo-relative.
#
# One row today. It is the operator's standing Entra oid, and it rides out twice:
# here, and in the Rust the build generates from here — which is why the ceremony
# regenerates the derived files AFTER lustration, and why the fixture's shape
# sweep reads the generated Rust as an ordinary shipping file.

ZRBLM_HARDPOINTS=(
  "Tools/rbk/rbpc_constants.sh|RBPC_freehold_subject|${RBLM_unset_subject}"
)

######################################################################
# Emitters — the fixture's reach
#
# Tab-separated, one row per line, no decoration. The damnatio fixture sources
# this module and calls these, so the proscription it judges against is the same
# table lustration writes from.

rblm_emit_proscription() {
  local z_row=""
  local z_rest=""
  local z_scope=""
  local z_var=""
  local z_disposition=""
  local z_sterile=""

  for z_row in "${ZRBLM_PROSCRIPTION[@]}"; do
    z_scope="${z_row%%|*}"
    z_rest="${z_row#*|}"
    z_var="${z_rest%%|*}"
    z_rest="${z_rest#*|}"
    z_disposition="${z_rest%%|*}"
    z_sterile="${z_rest#*|}"
    printf '%s\t%s\t%s\t%s\n' "${z_scope}" "${z_var}" "${z_disposition}" "${z_sterile}"
  done
}

rblm_emit_hardpoints() {
  local z_row=""
  local z_rest=""
  local z_path=""
  local z_var=""
  local z_sterile=""

  for z_row in "${ZRBLM_HARDPOINTS[@]}"; do
    z_path="${z_row%%|*}"
    z_rest="${z_row#*|}"
    z_var="${z_rest%%|*}"
    z_sterile="${z_rest#*|}"
    printf '%s\t%s\t%s\n' "${z_path}" "${z_var}" "${z_sterile}"
  done
}

rblm_emit_homes() {
  zrblm_homes_capture

  local z_home=""
  for z_home in "${ZRBLM_HOMES[@]}"; do
    printf '%s\t%s\n' "${z_home%%|*}" "${z_home#*|}"
  done
}

######################################################################
# The transform

# zrblm_homes_capture — resolve every regime file the tree carries, tagged with
# the scope that governs it. Sets ZRBLM_HOMES to SCOPE|FILE rows.
#
# The singleton regimes sit at their own paths; the multi-instance regimes are
# globbed across every instance present. The foedera glob deliberately matches
# the live rbrf.env files only — rbef_keycloak's committed rbrf.env.template
# describes the caged Keycloak realm, which is synthetic at every installation
# and ships as authored.
#
# RBRO has no row: its file lives under RBRR_SECRETS_DIR, outside the repository,
# which is what its offtree disposition records.
#
# The single home of the scope-to-file mapping: lustration writes through it, and
# the damnatio fixture reads it back rather than reimplementing the globs.
zrblm_homes_capture() {
  ZRBLM_HOMES=(
    "RBRR|${RBCC_rbrr_file}"
    "RBRD|${RBCC_rbrd_file}"
    "RBRP|${RBCC_rbrp_file}"
    "RBRW|${RBCC_rbrw_file}"
  )

  local z_instance=""

  for z_instance in "${RBCC_foedera_dir}"/*/rbrf.env; do
    test -f "${z_instance}" || continue
    ZRBLM_HOMES+=("RBRF|${z_instance}")
  done

  for z_instance in "${RBCC_moorings_dir}/${RBCC_vessels_subdir}"/*/"${RBCC_rbrv_file}"; do
    test -f "${z_instance}" || continue
    ZRBLM_HOMES+=("RBRV|${z_instance}")
  done

  for z_instance in "${RBCC_moorings_dir}"/*/"${RBCC_rbrn_file}"; do
    test -f "${z_instance}" || continue
    ZRBLM_HOMES+=("RBRN|${z_instance}")
  done
}

# zrblm_sterile_capture SCOPE VARNAME — resolve one field's sterile value.
# Sets ZRBLM_STERILE and returns 0 when the field is site-scoped in SCOPE;
# returns 1 otherwise, leaving the caller to pass the line through untouched.
zrblm_sterile_capture() {
  local -r z_scope="${1:-}"
  local -r z_var="${2:-}"

  ZRBLM_STERILE=""

  local z_row=""
  local z_rest=""
  local z_row_var=""
  local z_row_disposition=""

  for z_row in "${ZRBLM_PROSCRIPTION[@]}"; do
    test "${z_row%%|*}" = "${z_scope}" || continue
    z_rest="${z_row#*|}"
    z_row_var="${z_rest%%|*}"
    test "${z_row_var}" = "${z_var}" || continue
    z_rest="${z_rest#*|}"
    z_row_disposition="${z_rest%%|*}"
    test "${z_row_disposition}" = "${RBLM_disposition_site}" || return 1
    ZRBLM_STERILE="${z_rest#*|}"
    return 0
  done

  return 1
}

# zrblm_scrub_regime FILE SCOPE — rewrite every site-scoped field in one regime
# file to its sterile value. A field the file does not carry is not an error: a
# bind vessel has no graft image, a conjure vessel no bind image. Comments,
# blanks and common fields pass through byte-for-byte.
zrblm_scrub_regime() {
  local -r z_file="${1:-}"
  local -r z_scope="${2:-}"
  test -n "${z_file}"  || buc_die "zrblm_scrub_regime: file required"
  test -n "${z_scope}" || buc_die "zrblm_scrub_regime: scope required"
  test -f "${z_file}"  || return 0

  local -r z_tmp="${z_file}.tmp"
  local z_line=""
  local z_var=""

  while IFS= read -r z_line || test -n "${z_line}"; do
    case "${z_line}" in
      [A-Z]*=*)
        z_var="${z_line%%=*}"
        if zrblm_sterile_capture "${z_scope}" "${z_var}"; then
          printf '%s\n' "${z_var}=${ZRBLM_STERILE}"
        else
          printf '%s\n' "${z_line}"
        fi
        ;;
      *)
        printf '%s\n' "${z_line}"
        ;;
    esac
  done < "${z_file}" > "${z_tmp}" || buc_die "Failed to scrub: ${z_file}"

  mv "${z_tmp}" "${z_file}" || buc_die "Failed to replace: ${z_file}"
  buh_line "  Lustrated: ${z_file}"
}

# zrblm_scrub_hardpoint PATH VARNAME STERILE — rewrite one source constant.
# The assignment may be quoted or bare; the sterile value is written quoted,
# which every hardpoint home accepts.
zrblm_scrub_hardpoint() {
  local -r z_file="${1:-}"
  local -r z_var="${2:-}"
  local -r z_sterile="${3:-}"
  test -n "${z_file}" || buc_die "zrblm_scrub_hardpoint: file required"
  test -n "${z_var}"  || buc_die "zrblm_scrub_hardpoint: variable required"
  test -f "${z_file}" || buc_die "Hardpoint file not found: ${z_file}"

  local -r z_tmp="${z_file}.tmp"
  local z_line=""
  local z_found=0

  while IFS= read -r z_line || test -n "${z_line}"; do
    case "${z_line}" in
      "${z_var}="*)
        printf '%s\n' "${z_var}=\"${z_sterile}\""
        z_found=1
        ;;
      *)
        printf '%s\n' "${z_line}"
        ;;
    esac
  done < "${z_file}" > "${z_tmp}" || buc_die "Failed to scrub hardpoint: ${z_file}"

  test "${z_found}" = "1" || buc_die "Hardpoint ${z_var} not found in ${z_file} — the proscription names a home that moved"

  mv "${z_tmp}" "${z_file}" || buc_die "Failed to replace: ${z_file}"
  buh_line "  Lustrated: ${z_file} (${z_var})"
}

# rblm_lustrate_apply — write every site-scoped home to its sterile value, across
# every regime instance the tree carries and every hardpoint constant.
rblm_lustrate_apply() {
  zrblm_homes_capture

  local z_home=""
  for z_home in "${ZRBLM_HOMES[@]}"; do
    zrblm_scrub_regime "${z_home#*|}" "${z_home%%|*}"
  done

  local z_row=""
  local z_rest=""
  local z_path=""
  local z_var=""

  for z_row in "${ZRBLM_HARDPOINTS[@]}"; do
    z_path="${z_row%%|*}"
    z_rest="${z_row#*|}"
    z_var="${z_rest%%|*}"
    zrblm_scrub_hardpoint "${z_path}" "${z_var}" "${z_rest#*|}"
  done
}

# eof
