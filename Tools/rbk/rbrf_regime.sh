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
# Recipe Bottle Federation Regime - Validator Module

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBRF_SOURCED:-}" || buc_die "Module rbrf multiply sourced - check sourcing hierarchy"
ZRBRF_SOURCED=1

######################################################################
# Internal Functions (zrbrf_*)

zrbrf_kindle() {
  test -z "${ZRBRF_KINDLED:-}" || buc_die "Module rbrf already kindled"

  # No defaults set — buv uses ${!varname:-} for safe indirect expansion under set -u.
  # Unset variables are detected distinctly from empty by zbuv_check_capture.

  # Enroll all RBRF variables — single source of truth for validation and rendering

  buv_regime_enroll RBRF

  buv_group_enroll "Workforce Pool Identity"
  buv_string_enroll  RBRF_ORG_ID               6   32  "GCP organization numeric ID owning the workforce pool (affiance creates the pool under it)"
  buv_string_enroll  RBRF_WORKFORCE_POOL_ID    4   32  "Workforce identity pool ID — org-scoped, serves every depot under the manor"
  buv_string_enroll  RBRF_PROVIDER_ID          4   32  "Workforce pool provider ID — the IdP trust root within the pool"
  buv_string_enroll  RBRF_SESSION_DURATION     2   10  "Workforce pool session duration — the assize cap (e.g. 3600s)"

  buv_group_enroll "IdP Trust"
  buv_string_enroll  RBRF_IDP_ISSUER           8  512  "OIDC issuer URI of the external IdP"
  buv_string_enroll  RBRF_IDP_CLIENT_ID        1  256  "Public client (application) ID registered at the IdP for the device flow"
  buv_string_enroll  RBRF_IDP_SCOPE            5  256  "Device-flow OAuth scope — must request openid, must not request offline_access"
  buv_string_enroll  RBRF_IDP_DEVICE_ENDPOINT  8  512  "IdP device authorization endpoint (RFC 8628) — Leg 1 device-code request"
  buv_string_enroll  RBRF_IDP_TOKEN_ENDPOINT   8  512  "IdP token endpoint — Leg 1 device-code polling"
  buv_string_enroll  RBRF_ATTRIBUTE_MAPPING    1  512  "Workforce provider attribute mapping — must map google.subject"

  # Guard against unexpected RBRF_ variables not in enrollment
  buv_scope_sentinel RBRF RBRF_

  # Lock all enrolled RBRF_ variables against mutation
  buv_lock RBRF

  readonly ZRBRF_KINDLED=1
}

zrbrf_sentinel() {
  test "${ZRBRF_KINDLED:-}" = "1" || buc_die "Module rbrf not kindled - call zrbrf_kindle first"
}

# Enforce all RBRF enrollment validations and custom format checks
zrbrf_enforce() {
  zrbrf_sentinel

  buv_vet RBRF

  [[ "${RBRF_ORG_ID}" =~ ^[0-9]{6,}$ ]] \
    || buc_reject "${BUBC_band_regime}" "RBRF_ORG_ID must be a numeric GCP organization ID: ${RBRF_ORG_ID}"

  [[ "${RBRF_WORKFORCE_POOL_ID}" =~ ^[a-z][a-z0-9-]{2,30}[a-z0-9]$ ]] \
    || buc_reject "${BUBC_band_regime}" "Invalid RBRF_WORKFORCE_POOL_ID: ${RBRF_WORKFORCE_POOL_ID} (GCP workforce-pool id: lowercase letter-led, [a-z0-9-], no trailing hyphen, 4-32 chars)"

  [[ "${RBRF_PROVIDER_ID}" =~ ^[a-z][a-z0-9-]{2,30}[a-z0-9]$ ]] \
    || buc_reject "${BUBC_band_regime}" "Invalid RBRF_PROVIDER_ID: ${RBRF_PROVIDER_ID} (GCP provider id: lowercase letter-led, [a-z0-9-], no trailing hyphen, 4-32 chars)"

  [[ "${RBRF_SESSION_DURATION}" =~ ^[0-9]+s$ ]] \
    || buc_reject "${BUBC_band_regime}" "Invalid RBRF_SESSION_DURATION: ${RBRF_SESSION_DURATION} (expected NNNs, e.g. 3600s)"

  [[ "${RBRF_IDP_ISSUER}" =~ ^https:// ]] \
    || buc_reject "${BUBC_band_regime}" "RBRF_IDP_ISSUER must be an https:// URI: ${RBRF_IDP_ISSUER}"

  [[ "${RBRF_IDP_DEVICE_ENDPOINT}" =~ ^https:// ]] \
    || buc_reject "${BUBC_band_regime}" "RBRF_IDP_DEVICE_ENDPOINT must be an https:// URI: ${RBRF_IDP_DEVICE_ENDPOINT}"

  [[ "${RBRF_IDP_TOKEN_ENDPOINT}" =~ ^https:// ]] \
    || buc_reject "${BUBC_band_regime}" "RBRF_IDP_TOKEN_ENDPOINT must be an https:// URI: ${RBRF_IDP_TOKEN_ENDPOINT}"

  # OIDC requires openid; the human-present premise forbids offline_access — a
  # refresh token would let a run begin outside a live assize. Both enforced here
  # so a misconfigured scope fails at the regime boundary, not mid-compearance.
  case " ${RBRF_IDP_SCOPE} " in
    *" openid "*) ;;
    *) buc_reject "${BUBC_band_regime}" "RBRF_IDP_SCOPE must request the openid scope: ${RBRF_IDP_SCOPE}" ;;
  esac
  case " ${RBRF_IDP_SCOPE} " in
    *offline_access*) buc_reject "${BUBC_band_regime}" "RBRF_IDP_SCOPE must not request offline_access — the no-refresh-token premise (a live human compears at each run)" ;;
  esac

  case "${RBRF_ATTRIBUTE_MAPPING}" in
    *google.subject*) ;;
    *) buc_reject "${BUBC_band_regime}" "RBRF_ATTRIBUTE_MAPPING must map google.subject: ${RBRF_ATTRIBUTE_MAPPING}" ;;
  esac
}

# eof
