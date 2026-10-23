#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
# RBLM Proscription - the site/common judgment and the two transforms it drives.
#
# The proscription names, for every enrolled regime field and every hardpoint
# constant, whether the value is the SITE's (this station's cloud, this
# operator's federated identity) or COMMON to every installation. Lustration
# writes the sterile value over every site-scoped home inside the release
# ceremony's throwaway clone; the damnatio fixture then proves, on the delivered
# tree, that nothing site-scoped survived.
#
# The proscription is the single home of that judgment. Both transforms below and
# the fixture read THIS table — the fixture reaches bash for it rather than
# carrying a second copy in Rust, so the two can never drift.
#
# The second transform is FEIGNING: the same site rows, written to a shape-valid
# stand-in instead of the sterile value, so the lustrated candidate can be made to
# validate on a throwaway probe branch and run its own reveille from the
# consumer's seat. Lustration erases the station; feigning invents a false one.
# One table, two columns, two verbs — the site/common judgment is made once.
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
test -z "${ZRBLM_PROSCRIPTION_SOURCED:-}" || buc_die_now "Module rblm_proscription multiply sourced - check sourcing hierarchy"
ZRBLM_PROSCRIPTION_SOURCED=1

######################################################################
# Dispositions

# Site-scoped: the value names this station. Erased at lustration, asserted
# sterile by the fixture.
readonly RBLM_disposition_site="site"

# Common: the value is the same at every installation (a default, a mode word, a
# port, an upstream image reference). Untouched, and asserted by nothing — but a
# field must still be declared common to satisfy the completeness case.
readonly RBLM_disposition_common="common"

# Off-tree: the field is enrolled, but its regime file lives outside the shipping
# tree, so no delivered byte can carry it. The payor's OAuth secret and refresh
# token are the case: they live in the operator's secrets directory, named by
# RBRR_SECRETS_DIR, which is not in the repository at all. Declared rather than
# omitted, so the completeness case still accounts for every enrolled field.
readonly RBLM_disposition_offtree="offtree"

######################################################################
# The value columns
#
# Which of a site row's two values a transform writes. Lustration writes the
# sterile column, feigning the feigned column.

readonly RBLM_column_sterile="sterile"
readonly RBLM_column_feigned="feigned"

# The delivery documentation base. The working repo's RBRR_PUBLIC_DOCS_URL points
# at the maintainer's development repo; the delivered tree must point consumers at
# the public home: the public repo's README blob, because blob rendering preserves
# the literal <a id> anchors the handbook links resolve, while staging and
# candidate branches are transient.
readonly RBLM_public_docs_url="https://github.com/scaleinv/recipebottle/blob/main/README.md"

# The freehold subject's sterile value. NOT a blank: rbpc_emit_consts projects
# this constant into the generated Rust (RBTGEC_FREEHOLD_SUBJECT) through
# buz_emit_const_str, which rejects an empty value — so a blank here dies at the
# candidate's own const regeneration. A placeholder that is shape-free (no UUID
# for the fixture's sweep to find) and obviously unset is the form that survives.
readonly RBLM_unset_subject="unset-freehold-subject"

######################################################################
# Internal Functions (zrblm_*)

# zrblm_kindle — enroll the proscription and the hardpoints.
#
# The proscription
#
# One enrollment per enrolled regime field: its scope, its name and its
# disposition, and for a site row its sterile and feigned values besides. Both
# value columns are read only for site rows.
#
# The STERILE value is what lustration writes. An empty one blanks the field to
# the onboarding-start state, which is what a consumer must fill in anyway. A
# NON-EMPTY sterile value is used only where a blank would break the delivered
# tree — never for cosmetics. Today that is RBRR_PUBLIC_DOCS_URL, whose whole
# purpose is to be a live URL in the consumer's hands.
#
# The FEIGNED value is what feigning writes. A lustrated tree cannot VALIDATE:
# thirteen of the site fields carry format checks a blank cannot satisfy, so the
# candidate — correctly sterile — cannot run its own test suite. Feigning writes a
# shape-valid stand-in over each, on a throwaway probe branch, so the ceremony can
# run the consumer's own reveille in the consumer's own tree. An empty feigned
# value means "stay sterile": the eight min-length-0 site fields validate blank
# and need no stand-in, and RBRR_PUBLIC_DOCS_URL must keep the sterile URL it was
# just given.
#
# Every feigned value is deliberately, visibly false — zeros where an id is
# numeric, the .invalid reserved TLD where a host is wanted, the verb's own name
# where a word will do. Nothing here may be borrowed from a live station: the
# whole point of the column is that no agent, blocked by a format check, ever
# goes looking through the clone's git history for a value that validates.
#
# Every field enrolled by Tools/rbk/rbr*_regime.sh must appear here exactly once.
# The damnatio fixture derives the enrolled set from the LIVE enrollment rolls
# and reddens on any field this table has not judged — so a newly enrolled field
# cannot ship undeclared, and this list cannot silently fall behind. Damnatio
# also asserts every site field holds its STERILE value, so a feigned tree can
# never be mistaken for a candidate: cut one by accident and the identity assay
# reddens on all thirteen.
#
# The hardpoints
#
# Site identity that lives in source rather than in a regime file, so no
# enrollment roll enumerates it and the completeness case cannot reach it. Each
# names its home repo-relative, its constant and its sterile value.
#
# One today. It is the operator's standing Entra oid, and it rides out twice:
# here, and in the Rust the build generates from here — which is why the ceremony
# regenerates the derived files AFTER lustration, and why the fixture's shape
# sweep reads the generated Rust as an ordinary shipping file.
zrblm_kindle() {
  test -z "${ZRBLM_KINDLED:-}" || buc_die_now "Module rblm_proscription already kindled"

  local -r z_site="${RBLM_disposition_site}"
  local -r z_common="${RBLM_disposition_common}"
  local -r z_offtree="${RBLM_disposition_offtree}"

  z_rblm_scope_roll=()
  z_rblm_var_roll=()
  z_rblm_disposition_roll=()
  z_rblm_sterile_roll=()
  z_rblm_feigned_roll=()

  # ── RBRR — repo regime ──
  zrblm_enroll RBRR RBRR_RUNTIME_PREFIX              "${z_site}"    "" "feign-"
  zrblm_enroll RBRR RBRR_VESSEL_DIR                  "${z_common}"
  zrblm_enroll RBRR RBRR_BOTTLE_WORKSPACE            "${z_common}"
  zrblm_enroll RBRR RBRR_DNS_SERVER                  "${z_common}"
  zrblm_enroll RBRR RBRR_GCB_TIMEOUT                 "${z_common}"
  zrblm_enroll RBRR RBRR_GCB_MIN_CONCURRENT_BUILDS   "${z_common}"
  zrblm_enroll RBRR RBRR_SECRETS_DIR                 "${z_common}"
  zrblm_enroll RBRR RBRR_PUBLIC_DOCS_URL             "${z_site}"    "${RBLM_public_docs_url}" ""
  zrblm_enroll RBRR RBRR_ACTIVE_FOEDUS               "${z_common}"
  zrblm_enroll RBRR RBRR_SUBSTRATE_RELIQUARY         "${z_site}"    "" ""

  # ── RBRD — depot regime ──
  zrblm_enroll RBRD RBRD_CLOUD_PREFIX                "${z_site}"    "" "feign-"
  zrblm_enroll RBRD RBRD_DEPOT_MONIKER               "${z_site}"    "" "feigned"
  zrblm_enroll RBRD RBRD_GCP_REGION                  "${z_common}"
  zrblm_enroll RBRD RBRD_GCB_MACHINE_TYPE            "${z_common}"

  # ── RBRP — payor regime ──
  zrblm_enroll RBRP RBRP_PAYOR_PROJECT_ID            "${z_site}"    "" "rbwg-p-000000000000"
  zrblm_enroll RBRP RBRP_BILLING_ACCOUNT_ID          "${z_site}"    "" ""
  zrblm_enroll RBRP RBRP_OAUTH_CLIENT_ID             "${z_site}"    "" ""
  zrblm_enroll RBRP RBRP_OPERATOR_EMAIL              "${z_site}"    "" ""

  # ── RBRW — workforce regime ──
  zrblm_enroll RBRW RBRW_ORG_ID                      "${z_site}"    "" "000000000000"
  zrblm_enroll RBRW RBRW_WORKFORCE_POOL_ID           "${z_site}"    "" "feigned-manor"
  zrblm_enroll RBRW RBRW_SESSION_DURATION            "${z_common}"

  # ── RBRF — federation regime ──
  # The interactive fields name the operator's own IdP tenant and app
  # registration; the programmatic fields name only the caged Keycloak realm the
  # test facility ships, which is synthetic at every installation.
  zrblm_enroll RBRF RBRF_PROVIDER_ID                 "${z_site}"    "" "feigned-foedus"
  zrblm_enroll RBRF RBRF_MECHANISM                   "${z_common}"
  zrblm_enroll RBRF RBRF_IDP_ISSUER                  "${z_site}"    "" "https://idp.example.invalid/feigned"
  zrblm_enroll RBRF RBRF_IDP_CLIENT_ID               "${z_site}"    "" "feigned-client"
  zrblm_enroll RBRF RBRF_ATTRIBUTE_MAPPING           "${z_common}"
  zrblm_enroll RBRF RBRF_IDP_SCOPE                   "${z_common}"
  zrblm_enroll RBRF RBRF_IDP_DEVICE_ENDPOINT         "${z_site}"    "" "https://idp.example.invalid/devicecode"
  zrblm_enroll RBRF RBRF_IDP_TOKEN_ENDPOINT          "${z_site}"    "" "https://idp.example.invalid/token"
  zrblm_enroll RBRF RBRF_IDP_JWKS_JSON               "${z_common}"
  zrblm_enroll RBRF RBRF_GRANT_ENDPOINT              "${z_common}"
  zrblm_enroll RBRF RBRF_ASSERTER_KEY_FILE           "${z_common}"
  zrblm_enroll RBRF RBRF_CLIENT_SECRET_FILE          "${z_common}"
  zrblm_enroll RBRF RBRF_ASSERTER_KID                "${z_common}"
  zrblm_enroll RBRF RBRF_ASSERTER_ISSUER             "${z_common}"
  zrblm_enroll RBRF RBRF_ASSERTER_SUBJECT            "${z_common}"

  # ── RBRO — payor OAuth regime ──
  zrblm_enroll RBRO RBRO_CLIENT_SECRET               "${z_offtree}"
  zrblm_enroll RBRO RBRO_REFRESH_TOKEN               "${z_offtree}"

  # ── RBRV — vessel regime ──
  # The origins are upstream vendor references, common to every installation; the
  # anchors, the reliquary and the graft image name Lodes and images in THIS
  # station's depot registry.
  zrblm_enroll RBRV RBRV_SIGIL                       "${z_common}"
  zrblm_enroll RBRV RBRV_DESCRIPTION                 "${z_common}"
  zrblm_enroll RBRV RBRV_USER                        "${z_common}"
  zrblm_enroll RBRV RBRV_VESSEL_MODE                 "${z_common}"
  zrblm_enroll RBRV RBRV_RELIQUARY                   "${z_site}"    "" "r000000000000"
  zrblm_enroll RBRV RBRV_EGRESS_MODE                 "${z_common}"
  zrblm_enroll RBRV RBRV_BIND_IMAGE                  "${z_common}"
  zrblm_enroll RBRV RBRV_BIND_OPTIONAL_DOCKERFILE    "${z_common}"
  zrblm_enroll RBRV RBRV_CONJURE_DOCKERFILE          "${z_common}"
  zrblm_enroll RBRV RBRV_CONJURE_BLDCONTEXT          "${z_common}"
  zrblm_enroll RBRV RBRV_CONJURE_PLATFORMS           "${z_common}"
  zrblm_enroll RBRV RBRV_IMAGE_1_ORIGIN              "${z_common}"
  zrblm_enroll RBRV RBRV_IMAGE_1_ANCHOR              "${z_site}"    "" ""
  zrblm_enroll RBRV RBRV_IMAGE_2_ORIGIN              "${z_common}"
  zrblm_enroll RBRV RBRV_IMAGE_2_ANCHOR              "${z_site}"    "" ""
  zrblm_enroll RBRV RBRV_IMAGE_3_ORIGIN              "${z_common}"
  zrblm_enroll RBRV RBRV_IMAGE_3_ANCHOR              "${z_site}"    "" ""
  zrblm_enroll RBRV RBRV_GRAFT_IMAGE                 "${z_site}"    "" "feigned-graft:feigned"
  zrblm_enroll RBRV RBRV_GRAFT_OPTIONAL_DOCKERFILE   "${z_common}"

  # ── RBRN — nameplate regime ──
  zrblm_enroll RBRN RBRN_MONIKER                     "${z_common}"
  zrblm_enroll RBRN RBRN_DESCRIPTION                 "${z_common}"
  zrblm_enroll RBRN RBRN_RUNTIME                     "${z_common}"
  zrblm_enroll RBRN RBRN_SENTRY_VESSEL               "${z_common}"
  zrblm_enroll RBRN RBRN_BOTTLE_VESSEL               "${z_common}"
  zrblm_enroll RBRN RBRN_SENTRY_HALLMARK             "${z_site}"    "" ""
  zrblm_enroll RBRN RBRN_BOTTLE_HALLMARK             "${z_site}"    "" ""
  zrblm_enroll RBRN RBRN_BOTTLE_READINESS_DELAY_SEC  "${z_common}"
  zrblm_enroll RBRN RBRN_ENTRY_MODE                  "${z_common}"
  zrblm_enroll RBRN RBRN_ENTRY_PORT_WORKSTATION      "${z_common}"
  zrblm_enroll RBRN RBRN_ENTRY_PORT_ENCLAVE          "${z_common}"
  zrblm_enroll RBRN RBRN_ENCLAVE_BASE_IP             "${z_common}"
  zrblm_enroll RBRN RBRN_ENCLAVE_NETMASK             "${z_common}"
  zrblm_enroll RBRN RBRN_ENCLAVE_SENTRY_IP           "${z_common}"
  zrblm_enroll RBRN RBRN_ENCLAVE_BOTTLE_IP           "${z_common}"
  zrblm_enroll RBRN RBRN_UPLINK_PORT_MIN             "${z_common}"
  zrblm_enroll RBRN RBRN_UPLINK_DNS_MODE             "${z_common}"
  zrblm_enroll RBRN RBRN_UPLINK_ACCESS_MODE          "${z_common}"
  zrblm_enroll RBRN RBRN_UPLINK_ALLOWED_DOMAINS      "${z_common}"
  zrblm_enroll RBRN RBRN_UPLINK_ALLOWED_CIDRS        "${z_common}"

  z_rblm_hardpoint_path_roll=()
  z_rblm_hardpoint_var_roll=()
  z_rblm_hardpoint_sterile_roll=()

  zrblm_hardpoint_enroll "Tools/rbk/rbpc_constants.sh" RBPC_freehold_subject "${RBLM_unset_subject}"

  readonly ZRBLM_KINDLED=1
}

zrblm_sentinel() {
  test "${ZRBLM_KINDLED:-}" = "1" || buc_die_now "Module rblm_proscription not kindled - call zrblm_kindle first"
}

# zrblm_enroll SCOPE VARNAME DISPOSITION [STERILE FEIGNED] — enroll one regime
# field's judgment. A site row carries both values, empty where it has none; a
# common or offtree row carries neither. A value is written bare into a regime
# line, so it is held to the bare-token alphabet here, before any write.
zrblm_enroll() {
  local -r z_scope="${1:-}"
  local -r z_var="${2:-}"
  local -r z_disposition="${3:-}"
  local -r z_sterile="${4:-}"
  local -r z_feigned="${5:-}"

  test -n "${z_scope}" || buc_die_now "Proscription: empty scope"
  case "${z_var}" in
    "${z_scope}_"?*) ;;
    *) buc_die_now "Proscription: '${z_var}' is not a field of scope ${z_scope}" ;;
  esac
  case "${z_disposition}" in
    "${RBLM_disposition_site}")
      test "$#" -eq 5 || buc_die_now "Proscription: site field ${z_var} takes a sterile and a feigned value"
      ;;
    "${RBLM_disposition_common}"|"${RBLM_disposition_offtree}")
      test "$#" -eq 3 || buc_die_now "Proscription: ${z_disposition} field ${z_var} takes no value"
      ;;
    *)
      buc_die_now "Proscription: ${z_var} carries disposition '${z_disposition}'"
      ;;
  esac
  case "${z_sterile}${z_feigned}" in
    *[!ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789._/:+=-]*)
      buc_die_now "Proscription: ${z_var} carries a value outside the bare-token alphabet"
      ;;
  esac

  local z_i=0
  for z_i in "${!z_rblm_var_roll[@]}"; do
    test "${z_rblm_var_roll[${z_i}]}" != "${z_var}" || buc_die_now "Proscription: ${z_var} enrolled twice"
  done

  z_rblm_scope_roll+=("${z_scope}")
  z_rblm_var_roll+=("${z_var}")
  z_rblm_disposition_roll+=("${z_disposition}")
  z_rblm_sterile_roll+=("${z_sterile}")
  z_rblm_feigned_roll+=("${z_feigned}")
}

# zrblm_hardpoint_enroll PATH VARNAME STERILE — enroll one hardpoint. The sterile
# value is written quoted into a source line, so it is held to the bare-token
# alphabet here, which carries no character a double-quoted literal expands.
zrblm_hardpoint_enroll() {
  local -r z_path="${1:-}"
  local -r z_var="${2:-}"
  local -r z_sterile="${3:-}"

  test "$#" -eq 3 || buc_die_now "Hardpoint: a path, a variable and a sterile value required"
  test -n "${z_path}" || buc_die_now "Hardpoint: empty path"
  case "${z_var}" in
    ''|[!ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_]*|*[!ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_]*)
      buc_die_now "Hardpoint: '${z_var}' is not a variable name"
      ;;
  esac
  case "${z_sterile}" in
    *[!ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789._/:+=-]*)
      buc_die_now "Hardpoint: ${z_var} carries a value outside the bare-token alphabet"
      ;;
  esac

  local z_i=0
  for z_i in "${!z_rblm_hardpoint_var_roll[@]}"; do
    test "${z_rblm_hardpoint_var_roll[${z_i}]}" != "${z_var}" || buc_die_now "Hardpoint: ${z_var} enrolled twice"
  done

  z_rblm_hardpoint_path_roll+=("${z_path}")
  z_rblm_hardpoint_var_roll+=("${z_var}")
  z_rblm_hardpoint_sterile_roll+=("${z_sterile}")
}

######################################################################
# Emitters — the fixture's reach
#
# Tab-separated, one row per line, no decoration. The damnatio fixture sources
# this module and calls these, so the proscription it judges against is the same
# table lustration writes from.

# Four columns, not five: the fixture judges the DELIVERED tree, which no feigned
# value ever reaches. The feigned column is the ceremony's alone, so it does not
# cross this wire and the fixture's parser is untouched by its arrival.
rblm_emit_proscription() {
  zrblm_sentinel

  local z_i=0
  for z_i in "${!z_rblm_var_roll[@]}"; do
    printf '%s\t%s\t%s\t%s\n' \
      "${z_rblm_scope_roll[${z_i}]}" \
      "${z_rblm_var_roll[${z_i}]}" \
      "${z_rblm_disposition_roll[${z_i}]}" \
      "${z_rblm_sterile_roll[${z_i}]}" \
      || buc_die_now "Failed to emit proscription row: ${z_rblm_var_roll[${z_i}]}"
  done
}

rblm_emit_hardpoints() {
  zrblm_sentinel

  local z_i=0
  for z_i in "${!z_rblm_hardpoint_var_roll[@]}"; do
    printf '%s\t%s\t%s\n' \
      "${z_rblm_hardpoint_path_roll[${z_i}]}" \
      "${z_rblm_hardpoint_var_roll[${z_i}]}" \
      "${z_rblm_hardpoint_sterile_roll[${z_i}]}" \
      || buc_die_now "Failed to emit hardpoint row: ${z_rblm_hardpoint_var_roll[${z_i}]}"
  done
}

rblm_emit_homes() {
  zrblm_sentinel

  local z_homes
  z_homes=$(zrblm_homes_capture) || buc_die_now "Failed to resolve the regime homes"

  local z_rest="${z_homes}"
  local z_home=""
  while test -n "${z_rest}"; do
    z_home="${z_rest%%$'\n'*}"
    case "${z_rest}" in
      *$'\n'*) z_rest="${z_rest#*$'\n'}" ;;
      *)       z_rest=""                 ;;
    esac
    printf '%s\t%s\n' "${z_home%%|*}" "${z_home#*|}" \
      || buc_die_now "Failed to emit home row: ${z_home}"
  done
}

######################################################################
# The transform

# zrblm_homes_capture — resolve every regime file the tree carries, tagged with
# the scope that governs it. Prints SCOPE|FILE rows, one per line.
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
  zrblm_sentinel

  local z_homes=(
    "RBRR|${RBCC_rbrr_file}"
    "RBRD|${RBCC_rbrd_file}"
    "RBRP|${RBCC_rbrp_file}"
    "RBRW|${RBCC_rbrw_file}"
  )

  local z_instance=""

  for z_instance in "${RBCC_foedera_dir}"/*/rbrf.env; do
    test -f "${z_instance}" || continue
    z_homes+=("RBRF|${z_instance}")
  done

  for z_instance in "${RBCC_moorings_dir}/${RBCC_vessels_subdir}"/*/"${RBCC_rbrv_file}"; do
    test -f "${z_instance}" || continue
    z_homes+=("RBRV|${z_instance}")
  done

  for z_instance in "${RBCC_moorings_dir}"/*/"${RBCC_rbrn_file}"; do
    test -f "${z_instance}" || continue
    z_homes+=("RBRN|${z_instance}")
  done

  printf '%s\n' "${z_homes[@]}" || return 1
}

# zrblm_value_recite SCOPE VARNAME COLUMN — recite the value one transform
# writes into one field. Prints the value and returns 0 when the field is
# site-scoped in SCOPE; returns 1 otherwise, leaving the caller to pass the line
# through untouched. COLUMN is the caller's to validate.
#
# A feigned column that is empty resolves to the STERILE value, not to a blank:
# most site fields validate blank and want no stand-in, and the one field with a
# non-blank sterile value (the docs URL) must keep it.
zrblm_value_recite() {
  zrblm_sentinel

  local -r z_scope="${1:-}"
  local -r z_var="${2:-}"
  local -r z_column="${3:-}"

  local z_i=0
  for z_i in "${!z_rblm_var_roll[@]}"; do
    test "${z_rblm_var_roll[${z_i}]}" = "${z_var}" || continue
    test "${z_rblm_scope_roll[${z_i}]}" = "${z_scope}" || return 1
    test "${z_rblm_disposition_roll[${z_i}]}" = "${RBLM_disposition_site}" || return 1
    case "${z_column}" in
      "${RBLM_column_sterile}") printf '%s' "${z_rblm_sterile_roll[${z_i}]}" || return 1 ;;
      "${RBLM_column_feigned}") printf '%s' "${z_rblm_feigned_roll[${z_i}]:-${z_rblm_sterile_roll[${z_i}]}}" || return 1 ;;
      *) return 1 ;;
    esac
    return 0
  done
  return 1
}

# zrblm_scrub_regime FILE SCOPE COLUMN — rewrite every site-scoped field in one
# regime file to the named column's value. A field the file does not carry is not
# an error: a bind vessel has no graft image, a conjure vessel no bind image.
# Comments, blanks and common fields pass through byte-for-byte.
zrblm_scrub_regime() {
  zrblm_sentinel

  local -r z_file="${1:-}"
  local -r z_scope="${2:-}"
  local -r z_column="${3:-}"
  test -n "${z_file}"   || buc_die_now "zrblm_scrub_regime: file required"
  test -n "${z_scope}"  || buc_die_now "zrblm_scrub_regime: scope required"
  test -n "${z_column}" || buc_die_now "zrblm_scrub_regime: column required"
  case "${z_column}" in
    "${RBLM_column_sterile}"|"${RBLM_column_feigned}") ;;
    *) buc_die_now "zrblm_scrub_regime: unknown column: ${z_column}" ;;
  esac
  test -f "${z_file}"   || return 0

  local -r z_tmp="${z_file}.tmp"
  local z_line=""
  local z_var=""
  local z_value=""
  local z_site_status=0
  local z_lines=()
  local z_i=0

  while IFS= read -r z_line || test -n "${z_line}"; do
    z_lines+=("${z_line}")
  done < "${z_file}" || buc_die_now "Failed to read: ${z_file}"

  # Seed the temp file from the original so it inherits the original's mode, then
  # truncate it by redirection: a bare `> tmp` on a fresh path would take the
  # umask's mode instead, and the rename would silently rewrite the delivered
  # file's permission bits alongside its content.
  cp -p "${z_file}" "${z_tmp}" || buc_die_now "Failed to seed temp file for: ${z_file}"

  for z_i in "${!z_lines[@]}"; do
    z_line="${z_lines[${z_i}]}"
    case "${z_line}" in
      [ABCDEFGHIJKLMNOPQRSTUVWXYZ]*=*)
        z_var="${z_line%%=*}"
        z_site_status=0
        z_value=$(zrblm_value_recite "${z_scope}" "${z_var}" "${z_column}") || z_site_status=$?
        if test "${z_site_status}" -eq 0; then
          printf '%s\n' "${z_var}=${z_value}" || buc_die_now "Failed to write: ${z_tmp}"
        else
          printf '%s\n' "${z_line}" || buc_die_now "Failed to write: ${z_tmp}"
        fi
        ;;
      *)
        printf '%s\n' "${z_line}" || buc_die_now "Failed to write: ${z_tmp}"
        ;;
    esac
  done > "${z_tmp}" || buc_die_now "Failed to scrub: ${z_file}"

  mv "${z_tmp}" "${z_file}" || buc_die_now "Failed to replace: ${z_file}"
  buh_line "  Rewrote (${z_column}): ${z_file}"
}

# zrblm_scrub_hardpoint PATH VARNAME STERILE — rewrite one source constant.
# A hardpoint is tinder, and every tinder assignment carries readonly, so its
# line opens 'readonly VARNAME=' and is found by that opening, never by a line
# opening on the bare name. It is written back sealed, the sterile value quoted.
zrblm_scrub_hardpoint() {
  zrblm_sentinel

  local -r z_file="${1:-}"
  local -r z_var="${2:-}"
  local -r z_sterile="${3:-}"
  test -n "${z_file}" || buc_die_now "zrblm_scrub_hardpoint: file required"
  test -n "${z_var}"  || buc_die_now "zrblm_scrub_hardpoint: variable required"
  test -f "${z_file}" || buc_die_now "Hardpoint file not found: ${z_file}"

  local -r z_tmp="${z_file}.tmp"
  local z_line=""
  local z_found=0
  local z_lines=()
  local z_i=0

  while IFS= read -r z_line || test -n "${z_line}"; do
    z_lines+=("${z_line}")
  done < "${z_file}" || buc_die_now "Failed to read: ${z_file}"

  cp -p "${z_file}" "${z_tmp}" || buc_die_now "Failed to seed temp file for: ${z_file}"

  for z_i in "${!z_lines[@]}"; do
    z_line="${z_lines[${z_i}]}"
    case "${z_line}" in
      "readonly ${z_var}="*)
        printf '%s\n' "readonly ${z_var}=\"${z_sterile}\"" || buc_die_now "Failed to write: ${z_tmp}"
        z_found=1
        ;;
      *)
        printf '%s\n' "${z_line}" || buc_die_now "Failed to write: ${z_tmp}"
        ;;
    esac
  done > "${z_tmp}" || buc_die_now "Failed to scrub hardpoint: ${z_file}"

  test "${z_found}" = "1" || buc_die_now "Hardpoint ${z_var} not found in ${z_file} — the proscription names a home that moved"

  mv "${z_tmp}" "${z_file}" || buc_die_now "Failed to replace: ${z_file}"
  buh_line "  Lustrated: ${z_file} (${z_var})"
}

# rblm_lustrate_apply — write every site-scoped home to its sterile value, across
# every regime instance the tree carries and every hardpoint constant.
rblm_lustrate_apply() {
  zrblm_sentinel

  local z_homes
  z_homes=$(zrblm_homes_capture) || buc_die_now "Failed to resolve the regime homes"

  local z_rest="${z_homes}"
  local z_home=""
  while test -n "${z_rest}"; do
    z_home="${z_rest%%$'\n'*}"
    case "${z_rest}" in
      *$'\n'*) z_rest="${z_rest#*$'\n'}" ;;
      *)       z_rest=""                 ;;
    esac
    zrblm_scrub_regime "${z_home#*|}" "${z_home%%|*}" "${RBLM_column_sterile}"
  done

  local z_i=0
  for z_i in "${!z_rblm_hardpoint_var_roll[@]}"; do
    zrblm_scrub_hardpoint \
      "${z_rblm_hardpoint_path_roll[${z_i}]}" \
      "${z_rblm_hardpoint_var_roll[${z_i}]}" \
      "${z_rblm_hardpoint_sterile_roll[${z_i}]}"
  done
}

# rblm_feign_apply — write every site-scoped home to its feigned value, across
# every regime instance the tree carries.
#
# The hardpoints are deliberately untouched. The freehold subject's sterile value
# is already a shape-free placeholder the generated Rust accepts, and no credless
# fixture reads it — a station that never reaches a cloud needs no subject to be
# it.
rblm_feign_apply() {
  zrblm_sentinel

  local z_homes
  z_homes=$(zrblm_homes_capture) || buc_die_now "Failed to resolve the regime homes"

  local z_rest="${z_homes}"
  local z_home=""
  while test -n "${z_rest}"; do
    z_home="${z_rest%%$'\n'*}"
    case "${z_rest}" in
      *$'\n'*) z_rest="${z_rest#*$'\n'}" ;;
      *)       z_rest=""                 ;;
    esac
    zrblm_scrub_regime "${z_home#*|}" "${z_home%%|*}" "${RBLM_column_feigned}"
  done
}

# eof
