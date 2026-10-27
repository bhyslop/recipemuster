#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
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
# Recipe Bottle Auth - RBRO credential load and mantle role token mint

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBAU_SOURCED:-}" || buc_die_now "Module rba multiply sourced - check sourcing hierarchy"
ZRBAU_SOURCED=1

######################################################################
# Internal Functions (zrba_*)

zrbau_kindle() {
  test -z "${ZRBAU_KINDLED:-}" || buc_die_now "Module rba already kindled"

  # Ensure dependency kindled first (rba mints tokens via rbgo)
  zrbgo_sentinel

  # Federation protocol constants — RFC 8628 device flow (Leg 1) + RFC 8693 STS
  # exchange (Leg 2). Pure protocol invariants with no config dependency, so they
  # set unconditionally; the per-trust audience is built from RBRF_* at Leg 2, not
  # here. The federated path is opt-in: only rbau_avow and its legs read these,
  # and they guard on zrbrf_sentinel / zrbcc_sentinel, so the keyfile-only
  # consumers that source rba are untouched.
  readonly ZRBAU_STS_ENDPOINT="https://sts.googleapis.com/v1/token"
  readonly ZRBAU_STS_GRANT_TYPE="urn:ietf:params:oauth:grant-type:token-exchange"
  readonly ZRBAU_STS_REQUESTED_TOKEN_TYPE="urn:ietf:params:oauth:token-type:access_token"
  readonly ZRBAU_STS_SUBJECT_TOKEN_TYPE="urn:ietf:params:oauth:token-type:id_token"
  readonly ZRBAU_STS_SCOPE="https://www.googleapis.com/auth/cloud-platform"
  readonly ZRBAU_DEVICE_GRANT_TYPE="urn:ietf:params:oauth:grant-type:device_code"

  # RFC 7523 JWT-authorization grant — the programmatic Leg 1's grant_type, the
  # no-human sibling of the device-code grant above.
  readonly ZRBAU_JWT_BEARER_GRANT_TYPE="urn:ietf:params:oauth:grant-type:jwt-bearer"

  # Sederunt cache tuning: skew (treat the federated token as spent this many
  # seconds before its stated expiry) and the device-flow poll ceiling (device
  # codes self-expire in ~15 min).
  readonly ZRBAU_SEDERUNT_SKEW_SEC=60
  readonly ZRBAU_DEVICE_POLL_MAX_SEC=900

  # Proactive runway floor — the blanket required-runway default (RBr_r3n).
  readonly ZRBAU_SEDERUNT_RUNWAY_FLOOR_SEC=7200

  # Programmatic (RFC 7523) assertion tuning: the self-supplied JWT's lifetime and
  # the grant scope. Each mint carries a fresh jti and a short exp — single-use.
  readonly ZRBAU_PROG_ASSERTION_TTL_SEC=300
  readonly ZRBAU_PROG_ASSERTION_SCOPE="openid"

  # Per-invocation scratch for the leg curls — BURD_TEMP_DIR (process lifetime),
  # never the sederunt cache, which must outlive one invocation (see rbau_avow).
  # Every leg response carries a secret (the device code, the id_token, the
  # federated token, the mantle token), so each is held in memory and read
  # through the secret pipe; only the legs' stderr lands here.
  readonly ZRBAU_FED_CURL_STDERR_FILE="${BURD_TEMP_DIR}/rbau_fed_curl_stderr.txt"
  readonly ZRBAU_FED_JQ_STDERR_FILE="${BURD_TEMP_DIR}/rbau_fed_jq_stderr.txt"
  readonly ZRBAU_FED_OPENSSL_STDERR_FILE="${BURD_TEMP_DIR}/rbau_fed_openssl_stderr.txt"

  # The don's request body is non-secret JSON (the scope list); it is composed
  # here rather than string-interpolated at the call site.
  readonly ZRBAU_FED_DON_BODY_FILE="${BURD_TEMP_DIR}/rbau_fed_don_body.json"

  # Clock and cache-field reads land in these temp files: jq/date write a file
  # and the value is read back with $(<file). No token is among them, and no
  # leg response stands in a temp file at all; the persistent sederunt cache
  # holds the federated token alone — the mantle token is never cached anywhere.
  readonly ZRBAU_FED_SEDERUNT_EXPIRY_FILE="${BURD_TEMP_DIR}/rbau_fed_sederunt_expiry.txt"
  readonly ZRBAU_FED_SEDERUNT_NOW_FILE="${BURD_TEMP_DIR}/rbau_fed_sederunt_now.txt"
  readonly ZRBAU_FED_AVOW_NOW_FILE="${BURD_TEMP_DIR}/rbau_fed_avow_now.txt"
  readonly ZRBAU_FED_RUNWAY_EXPIRY_FILE="${BURD_TEMP_DIR}/rbau_fed_runway_expiry.txt"
  readonly ZRBAU_FED_RUNWAY_NOW_FILE="${BURD_TEMP_DIR}/rbau_fed_runway_now.txt"

  # Programmatic Leg-1 (RFC 7523 mint) scratch. The JWT header/payload and their
  # base64url encodings, the signing input, the raw+encoded signature, and the
  # assembled assertion are all NON-secret (public JWT parts) — the one durable
  # secret, the asserter private key, is read only by openssl via its regime PATH
  # and never lands here, and the client secret is read only by curl via its file
  # reference. The grant's response bears the minted id_token and is held in
  # memory like every other leg response.
  readonly ZRBAU_FED_PROG_NOW_FILE="${BURD_TEMP_DIR}/rbau_fed_prog_now.txt"
  readonly ZRBAU_FED_PROG_HEADER_FILE="${BURD_TEMP_DIR}/rbau_fed_prog_header.json"
  readonly ZRBAU_FED_PROG_PAYLOAD_FILE="${BURD_TEMP_DIR}/rbau_fed_prog_payload.json"
  readonly ZRBAU_FED_PROG_HEADER_B64_FILE="${BURD_TEMP_DIR}/rbau_fed_prog_header_b64.txt"
  readonly ZRBAU_FED_PROG_PAYLOAD_B64_FILE="${BURD_TEMP_DIR}/rbau_fed_prog_payload_b64.txt"
  readonly ZRBAU_FED_PROG_SIGNING_FILE="${BURD_TEMP_DIR}/rbau_fed_prog_signing.txt"
  readonly ZRBAU_FED_PROG_SIG_RAW_FILE="${BURD_TEMP_DIR}/rbau_fed_prog_sig_raw.bin"
  readonly ZRBAU_FED_PROG_SIG_B64_FILE="${BURD_TEMP_DIR}/rbau_fed_prog_sig_b64.txt"
  readonly ZRBAU_FED_PROG_ASSERTION_FILE="${BURD_TEMP_DIR}/rbau_fed_prog_assertion.jwt"

  readonly ZRBAU_KINDLED=1
}

zrbau_sentinel() {
  test "${ZRBAU_KINDLED:-}" = "1" || buc_die_now "Module rba not kindled - call zrbau_kindle first"
}

######################################################################
# External / RBTOE Pattern Functions

# The credential accessor — the single place credential material is resolved.
# Keyed by identity (governor | director | retriever): ensures a live sederunt
# (avowal) then dons the matching mantle SA, minting a short-lived mantle
# access token. No call site outside this function touches credential material
# (source-side grep-gated); the production callers and their bearer-blind
# downstream are unchanged — the keyfile→federation swap lives entirely here.
#
# Deliberately NOT a pure _capture: rbau_avow is folded in so callers never
# learn the avowal dance, so this accessor emits rbau_avow's buc_step
# progress to stderr and may buc_reject or buc_die_now on a failed avowal —
# under the interactive mechanism a missing sederunt rejects naming novate,
# since this accessor never signs in. The stdout contract
# still holds — only the mantle token reaches stdout (avow writes stderr
# only; the don emits the token straight to stdout) — and the reveille-tier
# credless guard's in-band buc_reject still propagates: avow's exit
# terminates the caller's command substitution with the credless band code,
# which the caller's `|| buc_die_now` re-exits through the band membrane. The sederunt
# cache rbau_avow writes is a file, so it survives this command-substitution
# subshell and the next caller takes the cache-hit path.
rbau_token_capture() {
  zrbau_sentinel
  zrbcc_sentinel

  local -r z_identity="${1:-}"

  # Validate up front so a typo'd identity refuses before an interactive avowal.
  # The identity is the pallium-sprued mantle token (rbpa_governor, …), THE
  # canonical form — no bare-role alias is accepted (RBCC_mantle_* is the home).
  case "${z_identity}" in
    "${RBCC_mantle_governor}"|"${RBCC_mantle_director}"|"${RBCC_mantle_retriever}") ;;
    *) buc_log_args "rbau_token_capture: unknown mantle token '${z_identity}' (expected ${RBCC_mantle_governor} | ${RBCC_mantle_director} | ${RBCC_mantle_retriever})"; return 1 ;;
  esac

  rbau_avow
  rbau_don_capture "${z_identity}"
}

######################################################################
# Federation branch — avowal (Leg 1) + STS exchange (Leg 2)
#
# The accessor's federated-token path. Leg 1 obtains an IdP id_token by one of two
# mechanism-gated arms (RBRF_MECHANISM): the interactive device flow (a human avows,
# RFC 8628) or the programmatic RFC 7523 grant (a self-supplied JWT, no human).
# Leg 2 exchanges that id_token at Google STS for a workforce federated
# access token, mechanism-invariant; that federated token alone is cached,
# per-sederunt. The mantle token (Leg 3, the don) is a separate artifact, separately
# scoped, and never cached — it is not built here. The persisted sederunt cache is the
# clean producer/consumer seam between this path and the don, and its key is already
# pool+provider, so an interactive and a programmatic foedus never cross sederunts.
#
# Federation config is read from two regimes under the one-pool Model: the manor
# pool id from RBRW_* (rbrw_regime, manor-level) and the per-foedus provider from
# RBRF_* (rbrf_regime). The leg curls reuse RBCC curl timeouts and rbgo's
# transient-curl classifier. Callers kindle rbrw + rbrf + rbcc before invoking
# rbau_avow; the functions guard on their sentinels.

# Resolve the sederunt cache path. The cache spans tabtarget processes within one
# operator session and is a credential store in the station secrets directory
# (RBDC_SEDERUNT_DIR), keyed by the trust so switching pools never crosses
# sederunts. Dir 0700; the file is written 0600.
zrbau_sederunt_path_capture() {
  zrbau_sentinel
  zrbdc_sentinel
  zrbrw_sentinel
  zrbrf_sentinel

  mkdir -p  "${RBDC_SEDERUNT_DIR}" || return 1
  chmod 700 "${RBDC_SEDERUNT_DIR}" || return 1
  printf '%s/%s.%s.json' "${RBDC_SEDERUNT_DIR}" "${RBRW_WORKFORCE_POOL_ID}" "${RBRF_PROVIDER_ID}"
}

# Echo the cached federated token if present and not within skew of expiry;
# return 1 on any miss (absent, malformed, or expired).
zrbau_sederunt_read_capture() {
  zrbau_sentinel

  local z_path
  z_path=$(zrbau_sederunt_path_capture) || return 1
  test -f "${z_path}" || return 1

  jq -r '.expiry_epoch // 0' "${z_path}" \
     > "${ZRBAU_FED_SEDERUNT_EXPIRY_FILE}" 2>"${ZRBAU_FED_JQ_STDERR_FILE}" || return 1
  local z_expiry
  z_expiry=$(<"${ZRBAU_FED_SEDERUNT_EXPIRY_FILE}") || return 1
  case "${z_expiry}" in ''|*[!0123456789]*) return 1 ;; esac

  date +%s > "${ZRBAU_FED_SEDERUNT_NOW_FILE}" || return 1
  local z_now
  z_now=$(<"${ZRBAU_FED_SEDERUNT_NOW_FILE}") || return 1
  case "${z_now}" in ''|*[!0123456789]*) return 1 ;; esac
  test "${z_expiry}" -gt "$(( 10#${z_now} + ZRBAU_SEDERUNT_SKEW_SEC ))" || return 1

  # Federated token (secret): jq emits it straight to stdout, never through a
  # shell var or temp file. select(length > 0) yields no output and a non-zero jq
  # exit when the token is absent or empty — the capture miss.
  jq -er '.federated_token // empty | select(length > 0)' "${z_path}" 2>"${ZRBAU_FED_JQ_STDERR_FILE}"
}

# Echo the cached sederunt subject (the avowed oid) if present; return 1 on any
# miss. Informational mirror of zrbau_sederunt_read_capture — the muniment-trail
# subject (decoded best-effort at avowal, non-load-bearing), not a
# credential, so no expiry gate: identity, not a token. select(length > 0) yields
# a non-zero jq exit for an absent/empty subject — the capture miss.
zrbau_sederunt_subject_capture() {
  zrbau_sentinel

  local z_path
  z_path=$(zrbau_sederunt_path_capture) || return 1
  test -f "${z_path}" || return 1

  jq -er '.subject // empty | select(length > 0)' "${z_path}" 2>"${ZRBAU_FED_JQ_STDERR_FILE}"
}

# A live (unexpired) sederunt is cached — status only, no output.
zrbau_sederunt_live_predicate() {
  zrbau_sentinel
  zrbau_sederunt_read_capture >/dev/null || return 1
  return 0
}

# Echo the cached sederunt's remaining runway in whole seconds (stored expiry
# minus now, floored at 0); return 1 on any structural miss (absent cache,
# malformed expiry, failed clock read). A lapsed sederunt is runway 0, not a
# miss — liveness and sufficiency judgments belong to the callers.
zrbau_sederunt_runway_capture() {
  zrbau_sentinel

  local z_path
  z_path=$(zrbau_sederunt_path_capture) || return 1
  test -f "${z_path}" || return 1

  jq -r '.expiry_epoch // 0' "${z_path}" \
     > "${ZRBAU_FED_RUNWAY_EXPIRY_FILE}" 2>"${ZRBAU_FED_JQ_STDERR_FILE}" || return 1
  local z_expiry
  z_expiry=$(<"${ZRBAU_FED_RUNWAY_EXPIRY_FILE}") || return 1
  case "${z_expiry}" in ''|*[!0123456789]*) return 1 ;; esac

  date +%s > "${ZRBAU_FED_RUNWAY_NOW_FILE}" || return 1
  local z_now
  z_now=$(<"${ZRBAU_FED_RUNWAY_NOW_FILE}") || return 1
  case "${z_now}" in ''|*[!0123456789]*) return 1 ;; esac

  local z_runway=$(( 10#${z_expiry} - 10#${z_now} ))
  test "${z_runway}" -ge 0 || z_runway=0
  printf '%s' "${z_runway}"
}

# Atomically write the sederunt cache (federated token + expiry epoch + subject).
# The record is composed in memory, jq reading the token from its environment,
# and written under umask 077 into a temp name beside the cache, then chmod 600
# and renamed, so the file is owner-only and never partially visible under its
# stable name; the dir is 0700 (owner-only traversal).
zrbau_sederunt_write() {
  zrbau_sentinel

  local -r zSECRET_token="${1:-}"
  local -r z_expiry_epoch="${2:-}"
  local -r z_subject="${3:-}"

  local z_path
  z_path=$(zrbau_sederunt_path_capture) || buc_die_now "Failed to resolve the ${RBCC_noun_sederunt} cache path"
  local -r z_tmp="${z_path}.tmp.${$}"

  local -r z_record_filter='{federated_token: env.zSECRET_federated, expiry_epoch: $e, subject: $s}'
  local zSECRET_record=""
  zSECRET_record=$(zSECRET_federated="${zSECRET_token}" jq -n --argjson e "${z_expiry_epoch}" --arg s "${z_subject}" "${z_record_filter}" 2>"${ZRBAU_FED_JQ_STDERR_FILE}") \
    || buc_die_now "Failed to compose the ${RBCC_noun_sederunt} cache — see ${ZRBAU_FED_JQ_STDERR_FILE}"
  (
    umask 077 || buc_die_now "Failed to set umask for the ${RBCC_noun_sederunt} cache"
    printf '%s\n' "${zSECRET_record}" > "${z_tmp}" || buc_die_now "Failed to write the ${RBCC_noun_sederunt} cache: ${z_tmp}"
  ) || buc_die_now "Failed to write the ${RBCC_noun_sederunt} cache"
  chmod 600 "${z_tmp}" || buc_die_now "Failed to restrict the ${RBCC_noun_sederunt} cache: ${z_tmp}"
  mv -f "${z_tmp}" "${z_path}" || buc_die_now "Failed to place the ${RBCC_noun_sederunt} cache: ${z_path}"
}

# Best-effort: decode the federated subject (oid, else sub) from the IdP id_token
# payload. Informational only — cached toward the muniment trail, never load-bearing.
# jq takes the payload segment, restores base64url to padded base64, decodes and
# selects, so the id_token is read through the secret pipe alone.
zrbau_idtoken_subject_capture() {
  zrbau_sentinel

  local -r zSECRET_idtoken="${1:-}"
  test -n "${zSECRET_idtoken}" || return 1

  local -r z_subject_filter='split(".")[1] // "" | gsub("-";"+") | gsub("_";"/") | . + ("=" * ((4 - (length % 4)) % 4)) | @base64d | fromjson | .oid // .sub // empty'
  local z_subject=""
  z_subject=$(printf '%s' "${zSECRET_idtoken}" | jq -Rr "${z_subject_filter}" 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || return 1

  echo "${z_subject}"
}

# Best-effort clipboard copy of the device-flow user code at avowal-prompt
# emission. Custody rule: ONLY the user code ever rides this path — never the
# device code, the federated token, or a mantle token. The user code is
# display-safe by design (RBr_u9k); accepted residual: clipboard sync/history may
# spread the single-use ~15-minute code to synced devices. Convenience only,
# never load-bearing — no tool found or a failed copy degrades to
# display-only, and only a successful copy is announced (it replaces the
# operator's prior clipboard contents). Mechanism is the BUK platform
# normalizer buc_clipboard_copy_predicate.
zrbau_user_code_clipboard() {
  zrbau_sentinel

  local -r z_code="${1:?zrbau_user_code_clipboard: user code required}"

  if buc_clipboard_copy_predicate "${z_code}"; then
    buc_step "    (code copied to your clipboard)"
  elif test -n "${z_buc_clipboard_tool}"; then
    buc_log_args "Clipboard copy via ${z_buc_clipboard_tool} failed; user code display-only"
  else
    buc_log_args "No clipboard tool present; user code display-only"
  fi
  return 0
}

# Leg 1 — device-flow avowal (RFC 8628). Requests a device + user code,
# surfaces the verification URL and code as a yawp on the progress stream,
# polls the IdP token endpoint until the human approves, and echoes the OIDC
# id_token. The id_token is never persisted — Leg 2 consumes it in-process.
zrbau_leg1_idtoken_capture() {
  zrbau_sentinel
  zrbrf_sentinel
  zrbcc_sentinel

  local -ra z_device_options=("-sS" -X POST --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" --max-time "${RBCC_CURL_MAX_TIME_SEC}" -H "Content-Type: application/x-www-form-urlencoded" --data-urlencode "client_id=${RBRF_IDP_CLIENT_ID}" --data-urlencode "scope=${RBRF_IDP_SCOPE}")
  local z_curl_status=0
  local zSECRET_device=""
  zSECRET_device=$(curl "${z_device_options[@]}" "${RBRF_IDP_DEVICE_ENDPOINT}" 2>"${ZRBAU_FED_CURL_STDERR_FILE}") || z_curl_status=$?
  test "${z_curl_status}" -eq 0 \
    || { buc_log_args "Device-code request failed (curl ${z_curl_status}); see ${ZRBAU_FED_CURL_STDERR_FILE}"; return 1; }

  local zSECRET_device_code=""
  zSECRET_device_code=$(printf '%s' "${zSECRET_device}" | jq -r '.device_code // empty' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || return 1
  local z_user_code=""
  z_user_code=$(printf '%s' "${zSECRET_device}" | jq -r '.user_code // empty' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || return 1
  local z_verification_uri=""
  z_verification_uri=$(printf '%s' "${zSECRET_device}" | jq -r '.verification_uri // .verification_url // empty' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || return 1
  local z_interval=""
  z_interval=$(printf '%s' "${zSECRET_device}" | jq -r '.interval // 5' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || return 1
  test -n "${zSECRET_device_code}" || return 1
  test -n "${z_user_code}"         || return 1
  test -n "${z_verification_uri}"  || return 1
  case "${z_interval}" in ''|*[!0123456789]*) buc_log_args "Device-flow interval is not a decimal number: ${z_interval}"; return 1 ;; esac

  # Surface the prompt as a yawp on the shared progress stream — console, log,
  # and any watching relay alike — so a headless-but-human-reachable caller can
  # complete the sign-in. The user code rides the stream deliberately: RFC 8628
  # designs it for open display (possession grants nothing without the human's
  # own IdP sign-in, and a substituted sign-in cannot pass admission), so the
  # retired /dev/tty emission and its headless fail-fast gate defended no
  # threat.
  buym_href_yawp "${z_verification_uri}" "${z_verification_uri}"; local -r z_uri_yp="${z_buym_yelp}"
  buym_ui_yawp   "${z_user_code}";                                local -r z_code_yp="${z_buym_yelp}"
  buc_step "Avowal — sign in to open your ${RBCC_noun_sederunt}:"
  buc_step "    ${z_uri_yp}"
  buc_step "    code: ${z_code_yp}"
  zrbau_user_code_clipboard "${z_user_code}"
  buc_log_args "Avowal prompt emitted; polling for sign-in"

  # Each poll sends the device code urlencoded from curl's stdin through the
  # secret pipe and holds the response in memory; an id_token ends the polling.
  local -ra z_poll_options=("-sS" -X POST --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" --max-time "${RBCC_CURL_MAX_TIME_SEC}" -H "Content-Type: application/x-www-form-urlencoded" --data-urlencode "grant_type=${ZRBAU_DEVICE_GRANT_TYPE}" --data-urlencode "client_id=${RBRF_IDP_CLIENT_ID}" --data-urlencode "device_code@-")
  local z_elapsed=0
  local z_err=""
  local z_err_status=0
  local z_jq_status=0
  local zSECRET_poll=""
  local zSECRET_idtoken=""
  while test "${z_elapsed}" -lt "${ZRBAU_DEVICE_POLL_MAX_SEC}"; do
    sleep "${z_interval}" || return 1
    z_elapsed=$(( z_elapsed + 10#${z_interval} ))

    z_curl_status=0
    zSECRET_poll=$(printf '%s' "${zSECRET_device_code}" | curl "${z_poll_options[@]}" "${RBRF_IDP_TOKEN_ENDPOINT}" 2>"${ZRBAU_FED_CURL_STDERR_FILE}") || z_curl_status=$?
    if test "${z_curl_status}" -ne 0; then
      rbgo_curl_status_is_transient_predicate "${z_curl_status}" \
        || { buc_log_args "Device-flow poll failed (curl ${z_curl_status})"; return 1; }
      continue
    fi

    # select(length > 0) yields a non-zero jq exit for an absent/empty token,
    # keeping it on the polling path.
    z_jq_status=0
    zSECRET_idtoken=$(printf '%s' "${zSECRET_poll}" | jq -er '.id_token // empty | select(length > 0)' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || z_jq_status=$?
    test "${z_jq_status}" -ne 0 || break

    z_err_status=0
    z_err=$(printf '%s' "${zSECRET_poll}" | jq -r '.error // empty' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || z_err_status=$?
    test "${z_err_status}" -eq 0 || z_err=""
    case "${z_err}" in
      authorization_pending) ;;
      slow_down)             z_interval=$(( 10#${z_interval} + 5 )) ;;
      *)                     buc_log_args "Device-flow authorization failed: ${z_err:-no id_token in response}"; return 1 ;;
    esac
  done

  test -n "${zSECRET_idtoken}" \
    || { buc_log_args "Device-flow timed out after ${ZRBAU_DEVICE_POLL_MAX_SEC}s without approval"; return 1; }

  echo "${zSECRET_idtoken}"
}

# base64url-encode a file's bytes (RFC 7515): standard base64 via openssl (the
# declared dependency — base64/tr are evicted),
# then +/=->-_ stripped by bash parameter expansion. A non-secret transform over
# public JWT parts. Args: $1 input file, $2 the raw-base64 scratch file (unique per
# part, forensic). Echoes the b64url string or returns 1.
zrbau_b64url_capture() {
  zrbau_sentinel

  local -r z_in="${1:-}"
  local -r z_raw="${2:-}"

  openssl enc -base64 -A -in "${z_in}" > "${z_raw}" 2>"${ZRBAU_FED_OPENSSL_STDERR_FILE}" || return 1
  local z_b64
  z_b64=$(<"${z_raw}") || return 1
  test -n "${z_b64}" || return 1
  z_b64="${z_b64//+/-}"
  z_b64="${z_b64//\//_}"
  z_b64="${z_b64//=/}"
  printf '%s' "${z_b64}"
}

# Leg 1 (programmatic) — the RFC 7523 JWT-authorization grant. The no-human sibling
# of the device-flow avowal: mint a subject id_token by signing an assertion with the
# caged asserter private key and POSTing it to the reachable grant endpoint, the
# confidential client authenticated by its secret. Echoes the OIDC id_token; Leg 2
# consumes it in-process, never persisted. Reads its inputs solely from the
# programmatic RBRF_ self-supply fields — it never learns "Keycloak".
#
# Custody: the asserter private key is read ONLY by openssl
# via its regime PATH and never enters a shell var; the client secret is read ONLY by
# curl via its file reference (--data-urlencode name@file) and never enters a shell
# var or the argument list; the minted id_token is held in memory and emitted as
# this capture's value. The assertion and its parts are public (a signed JWT), so they
# ride files and vars freely — only the two durable secrets are fenced.
zrbau_leg1_programmatic_idtoken_capture() {
  zrbau_sentinel
  zrbrf_sentinel
  zrbcc_sentinel

  # The two secret-path references (never the material). cwd is the repo root
  # (dispatch normalizes it), so these repo-root-relative paths resolve here.
  local -r z_key_file="${RBRF_ASSERTER_KEY_FILE}"
  local -r z_secret_file="${RBRF_CLIENT_SECRET_FILE}"
  test -f "${z_key_file}" \
    || { buc_log_args "Asserter key file absent: ${z_key_file} (RBRF_ASSERTER_KEY_FILE)"; return 1; }
  test -f "${z_secret_file}" \
    || { buc_log_args "Client secret file absent: ${z_secret_file} (RBRF_CLIENT_SECRET_FILE)"; return 1; }

  # Fresh assertion timestamps: date writes a
  # file read back with the $(<file) builtin. exp = iat + TTL; a unique jti per mint
  # (one-time use — Keycloak disables reuse by default).
  date +%s > "${ZRBAU_FED_PROG_NOW_FILE}" || return 1
  local z_iat
  z_iat=$(<"${ZRBAU_FED_PROG_NOW_FILE}") || return 1
  case "${z_iat}" in ''|*[!0123456789]*) return 1 ;; esac
  local -r z_exp=$(( 10#${z_iat} + ZRBAU_PROG_ASSERTION_TTL_SEC ))
  local -r z_jti="rba-${z_iat}-${$}"

  # Compose the non-secret JWT header and payload as JSON via jq (safe quoting of the
  # regime-sourced values). aud = RBRF_IDP_ISSUER — the assertion aud is cinched to
  # that existing field, no separate field; the IdP resolves the asserter
  # subject to its federated-linked user through the realm's asserting-trust link.
  jq -cn --arg kid "${RBRF_ASSERTER_KID}" \
     '{alg:"RS256",typ:"JWT",kid:$kid}' \
     > "${ZRBAU_FED_PROG_HEADER_FILE}" 2>"${ZRBAU_FED_JQ_STDERR_FILE}" \
    || { buc_log_args "Failed to compose the assertion header; see ${ZRBAU_FED_JQ_STDERR_FILE}"; return 1; }
  jq -cn --arg iss "${RBRF_ASSERTER_ISSUER}" --arg sub "${RBRF_ASSERTER_SUBJECT}" \
     --arg aud "${RBRF_IDP_ISSUER}" --argjson iat "${z_iat}" --argjson exp "${z_exp}" \
     --arg jti "${z_jti}" \
     '{iss:$iss,sub:$sub,aud:$aud,iat:$iat,exp:$exp,jti:$jti}' \
     > "${ZRBAU_FED_PROG_PAYLOAD_FILE}" 2>"${ZRBAU_FED_JQ_STDERR_FILE}" \
    || { buc_log_args "Failed to compose the assertion payload; see ${ZRBAU_FED_JQ_STDERR_FILE}"; return 1; }

  # base64url the header and payload (non-secret).
  local z_h64
  local z_h64_status=0
  z_h64=$(zrbau_b64url_capture "${ZRBAU_FED_PROG_HEADER_FILE}" "${ZRBAU_FED_PROG_HEADER_B64_FILE}") || z_h64_status=$?
  test "${z_h64_status}" -eq 0 \
    || { buc_log_args "Failed to base64url the assertion header; see ${ZRBAU_FED_OPENSSL_STDERR_FILE}"; return 1; }
  local z_p64
  local z_p64_status=0
  z_p64=$(zrbau_b64url_capture "${ZRBAU_FED_PROG_PAYLOAD_FILE}" "${ZRBAU_FED_PROG_PAYLOAD_B64_FILE}") || z_p64_status=$?
  test "${z_p64_status}" -eq 0 \
    || { buc_log_args "Failed to base64url the assertion payload; see ${ZRBAU_FED_OPENSSL_STDERR_FILE}"; return 1; }

  # Signing input = <b64url header>.<b64url payload>, to a file (the thing signed).
  printf '%s.%s' "${z_h64}" "${z_p64}" > "${ZRBAU_FED_PROG_SIGNING_FILE}" || return 1

  # RS256-sign with the asserter private key BY PATH (never a shell var); raw binary
  # signature to a file, then base64url it.
  openssl dgst -sha256 -sign "${z_key_file}" -binary "${ZRBAU_FED_PROG_SIGNING_FILE}" \
     > "${ZRBAU_FED_PROG_SIG_RAW_FILE}" 2>"${ZRBAU_FED_OPENSSL_STDERR_FILE}" \
    || { buc_log_args "Assertion signing failed; see ${ZRBAU_FED_OPENSSL_STDERR_FILE}"; return 1; }
  local z_sig
  local z_sig_status=0
  z_sig=$(zrbau_b64url_capture "${ZRBAU_FED_PROG_SIG_RAW_FILE}" "${ZRBAU_FED_PROG_SIG_B64_FILE}") || z_sig_status=$?
  test "${z_sig_status}" -eq 0 \
    || { buc_log_args "Failed to base64url the assertion signature; see ${ZRBAU_FED_OPENSSL_STDERR_FILE}"; return 1; }

  # Assemble the assertion = signing_input.signature, to a file curl reads by
  # reference (keeps it off the argument list; single-use, short-lived).
  local z_signing_input
  z_signing_input=$(<"${ZRBAU_FED_PROG_SIGNING_FILE}") || return 1
  printf '%s.%s' "${z_signing_input}" "${z_sig}" \
     > "${ZRBAU_FED_PROG_ASSERTION_FILE}" || return 1

  # POST the RFC 7523 grant. The client secret and the assertion both ride by FILE
  # REFERENCE (--data-urlencode name@file), so neither the secret nor the assertion
  # enters a shell var or the argument list; the response, bearing the id_token,
  # is held in memory.
  local -ra z_grant_options=("-sS" -X POST --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" --max-time "${RBCC_CURL_MAX_TIME_SEC}" -H "Content-Type: application/x-www-form-urlencoded" --data-urlencode "grant_type=${ZRBAU_JWT_BEARER_GRANT_TYPE}" --data-urlencode "client_id=${RBRF_IDP_CLIENT_ID}" --data-urlencode "scope=${ZRBAU_PROG_ASSERTION_SCOPE}" --data-urlencode "client_secret@${z_secret_file}" --data-urlencode "assertion@${ZRBAU_FED_PROG_ASSERTION_FILE}")
  local z_curl_status=0
  local zSECRET_grant=""
  zSECRET_grant=$(curl "${z_grant_options[@]}" "${RBRF_GRANT_ENDPOINT}" 2>"${ZRBAU_FED_CURL_STDERR_FILE}") || z_curl_status=$?
  test "${z_curl_status}" -eq 0 \
    || { buc_log_args "RFC 7523 grant POST failed (curl ${z_curl_status}); see ${ZRBAU_FED_CURL_STDERR_FILE}"; return 1; }

  # select(length > 0) yields a non-zero jq exit for an absent/empty token — the
  # mint miss; the forensic log then surfaces the grant's own error string.
  local z_jq_status=0
  local zSECRET_idtoken=""
  zSECRET_idtoken=$(printf '%s' "${zSECRET_grant}" | jq -er '.id_token // empty | select(length > 0)' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || z_jq_status=$?
  if test "${z_jq_status}" -ne 0; then
    local z_err_status=0
    local z_err=""
    z_err=$(printf '%s' "${zSECRET_grant}" | jq -r '.error // empty' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || z_err_status=$?
    test "${z_err_status}" -eq 0 || z_err=""
    buc_log_args "RFC 7523 grant returned no id_token (error: ${z_err:-none})"
    return 1
  fi

  echo "${zSECRET_idtoken}"
}

# Leg 2 — STS token exchange (RFC 8693). Exchanges the IdP id_token for a Google
# workforce federated access token. Unauthenticated POST; audience = the provider
# resource name; nothing else — no userProject, no auth header (spike F3). The
# id_token rides curl's stdin through the secret pipe, urlencoded as the subject
# token. Echoes the STS response, held in memory, once it carries an access token.
zrbau_leg2_federated_capture() {
  zrbau_sentinel
  zrbrw_sentinel
  zrbrf_sentinel
  zrbcc_sentinel

  local -r zSECRET_idtoken="${1:-}"
  test -n "${zSECRET_idtoken}" || return 1

  local -r z_audience="//iam.googleapis.com/locations/global/workforcePools/${RBRW_WORKFORCE_POOL_ID}/providers/${RBRF_PROVIDER_ID}"

  local -ra z_sts_options=("-sS" -X POST --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" --max-time "${RBCC_CURL_MAX_TIME_SEC}" -H "Content-Type: application/x-www-form-urlencoded" --data-urlencode "grant_type=${ZRBAU_STS_GRANT_TYPE}" --data-urlencode "audience=${z_audience}" --data-urlencode "scope=${ZRBAU_STS_SCOPE}" --data-urlencode "requested_token_type=${ZRBAU_STS_REQUESTED_TOKEN_TYPE}" --data-urlencode "subject_token_type=${ZRBAU_STS_SUBJECT_TOKEN_TYPE}" --data-urlencode "subject_token@-")
  local z_curl_status=0
  local zSECRET_sts=""
  zSECRET_sts=$(printf '%s' "${zSECRET_idtoken}" | curl "${z_sts_options[@]}" "${ZRBAU_STS_ENDPOINT}" 2>"${ZRBAU_FED_CURL_STDERR_FILE}") || z_curl_status=$?
  test "${z_curl_status}" -eq 0 \
    || { buc_log_args "STS exchange failed (curl ${z_curl_status}); see ${ZRBAU_FED_CURL_STDERR_FILE}"; return 1; }

  # select(length > 0) yields a non-zero jq exit for an absent/empty token.
  local z_status=0
  local zSECRET_federated=""
  zSECRET_federated=$(printf '%s' "${zSECRET_sts}" | jq -er '(.access_token // "") | select(length > 0)' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || z_status=$?
  test "${z_status}" -eq 0 \
    || { buc_log_args "STS exchange returned no access_token"; return 1; }

  echo "${zSECRET_sts}"
}

# Open a fresh sederunt: run Legs 1+2 and atomically cache the result. The
# mechanism-gated fresh path shared by rbau_avow (on a cache miss) and
# rbau_novate (unconditionally). Leg 1 is gated on RBRF_MECHANISM: the
# interactive arm is the device-flow avowal (a human signs in; the prompt rides
# the progress stream, so a terminal operator and a headless-but-human-reachable
# relay complete the same sign-in — no terminal gate, human presence enforced by
# the IdP sign-in, and a truly unattended miss polls to the bounded device-code
# expiry and dies loud); the programmatic arm is the RFC 7523 grant (no human,
# no sederunt to open — a self-supplied JWT). Leg 2 (STS) and the sederunt
# cache are mechanism-invariant, so only the id_token's origin differs.
zrbau_sederunt_open() {
  zrbau_sentinel
  zrbrf_sentinel
  zrbcc_sentinel

  # Leg 1 — mechanism-gated. buv_vet has already proven RBRF_MECHANISM is one of the
  # two enum values, but a bare-case fallthrough dies loud rather than silently.
  local zSECRET_idtoken=""
  case "${RBRF_MECHANISM}" in
    rbnfe_interactive)
      buc_step "Opening a ${RBCC_noun_sederunt} via device-flow avowal"
      zSECRET_idtoken=$(zrbau_leg1_idtoken_capture) \
        || buc_die_now "Avowal failed at Leg 1 (device flow); see the transcript"
      ;;
    rbnfe_programmatic)
      buc_step "Acquiring a ${RBCC_noun_sederunt} via the RFC 7523 programmatic grant"
      zSECRET_idtoken=$(zrbau_leg1_programmatic_idtoken_capture) \
        || buc_die_now "Acquisition failed at Leg 1 (RFC 7523 grant); see the transcript"
      ;;
    *)
      buc_die_now "rbau_avow: unknown RBRF_MECHANISM '${RBRF_MECHANISM}' (expected rbnfe_interactive | rbnfe_programmatic)"
      ;;
  esac

  local zSECRET_sts
  zSECRET_sts=$(zrbau_leg2_federated_capture "${zSECRET_idtoken}") || buc_die_now "Avowal failed at Leg 2 (STS exchange); see the transcript"

  local zSECRET_federated
  zSECRET_federated=$(printf '%s' "${zSECRET_sts}" | jq -r '.access_token' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || buc_die_now "Leg 2 (STS exchange): cannot read the federated token — see ${ZRBAU_FED_JQ_STDERR_FILE}"
  local z_expires_in
  z_expires_in=$(printf '%s' "${zSECRET_sts}" | jq -r '.expires_in // 0' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || buc_die_now "Leg 2 (STS exchange): cannot read the token expiry — see ${ZRBAU_FED_JQ_STDERR_FILE}"
  [[ "${z_expires_in}" =~ ^[0-9]+$ ]] || z_expires_in=0
  case "${z_expires_in}" in ''|*[!0123456789]*) buc_die_now "Leg 2 returned a non-numeric expiry: ${z_expires_in}" ;; esac

  date +%s > "${ZRBAU_FED_AVOW_NOW_FILE}" || buc_die_now "Failed to read the clock"
  local z_now
  z_now=$(<"${ZRBAU_FED_AVOW_NOW_FILE}") || buc_die_now "Failed to read: ${ZRBAU_FED_AVOW_NOW_FILE}"
  case "${z_now}" in ''|*[!0123456789]*) buc_die_now "Not a decimal number: ${z_now}" ;; esac
  local -r z_expiry_epoch=$(( 10#${z_now} + 10#${z_expires_in} ))

  local z_subject
  local z_subject_status=0
  z_subject=$(zrbau_idtoken_subject_capture "${zSECRET_idtoken}") || z_subject_status=$?
  test "${z_subject_status}" -eq 0 || z_subject=""

  zrbau_sederunt_write "${zSECRET_federated}" "${z_expiry_epoch}" "${z_subject}"

  buc_step "${RBCC_noun_sederunt_lettrine} opened (federated token expires in ${z_expires_in}s)"
}

# rbau_avow — the avowal accessor step. Ensures a live sederunt with sufficient
# runway; its side effect is the per-session cache, and consumers read the
# federated token with zrbau_sederunt_read_capture. Cache-hit → gate the
# remaining runway, then reuse. Miss/expired → mechanism-gated: interactive
# rejects in the runway band naming novate (rbau_novate is the one road to a
# sederunt, so no sign-in starts inside another command); programmatic opens a
# fresh one (zrbau_sederunt_open), no human prompt being at stake. An automated
# run avows once at suite head after the operator's novation; cases thereafter
# take the cache-hit path.
#
# Runway gate (RBr_r3n). Required runway: the optional first argument,
# defaulting to the blanket floor.
rbau_avow() {
  zrbau_sentinel
  zrbrf_sentinel
  zrbcc_sentinel

  local -r z_required_runway="${1:-${ZRBAU_SEDERUNT_RUNWAY_FLOOR_SEC}}"
  [[ "${z_required_runway}" =~ ^[0-9]+$ ]] \
    || buc_die_now "rbau_avow: required runway must be a non-negative integer of seconds, got '${z_required_runway}'"

  # Credless guard — the reveille tier must never touch the IdP or the network.
  # Mirrors the Payor OAuth token-mint guard (rbgp_payor.sh) so the federated
  # path honors the same invariant, under either mechanism arm.
  test "${BURE_TWEAK_NAME:-}" != "${RBCC_tweak_credless_guard}" \
    || buc_reject "${BUBC_band_credless}" "Credless guard: ${RBCC_noun_sederunt} acquisition refused — this run carries the reveille-tier guard (reveille cases must never reach the IdP)"

  if zrbau_sederunt_live_predicate; then
    local z_runway
    z_runway=$(zrbau_sederunt_runway_capture) \
      || buc_die_now "Live ${RBCC_noun_sederunt} became unreadable while gauging its runway"
    test "${z_runway}" -ge "${z_required_runway}" \
      || buc_reject "${BUBC_band_runway}" "${RBCC_noun_sederunt_lettrine} runway too short: ${z_runway}s remain, ${z_required_runway}s required — novate to open a fresh full-window ${RBCC_noun_sederunt} (rbw-aN), then re-run"
    buc_step "${RBCC_noun_sederunt_lettrine} already live — reusing the cached federated token (runway ${z_runway}s)"
    return 0
  fi

  test "${RBRF_MECHANISM}" != "rbnfe_interactive" \
    || buc_reject "${BUBC_band_runway}" "No live ${RBCC_noun_sederunt} — open a fresh full-window ${RBCC_noun_sederunt} with rbw-aN, the one door that signs in, then re-run"

  buc_step "No live ${RBCC_noun_sederunt} — acquiring a fresh one"
  zrbau_sederunt_open
}

# rbau_novate — the force-fresh renewal act: a deliberate
# avowal that bypasses the sederunt-reuse branch and atomically overwrites any
# standing sederunt with a freshly-opened, full-window one (novation:
# extinguish-by-replacement, riding zrbau_sederunt_write's temp-then-rename).
# The remedy the runway floor names when it turns a short sederunt away.
# Renewal-only by decision: no release or clear verb exists anywhere — the
# fresh sederunt extinguishing the old is the entire mechanism.
rbau_novate() {
  zrbau_sentinel
  zrbrf_sentinel
  zrbcc_sentinel

  # Credless guard — novation is a sederunt acquisition, so the reveille-tier
  # invariant holds here exactly as at the rbau_avow entry.
  test "${BURE_TWEAK_NAME:-}" != "${RBCC_tweak_credless_guard}" \
    || buc_reject "${BUBC_band_credless}" "Credless guard: ${RBCC_noun_sederunt} acquisition refused — this run carries the reveille-tier guard (reveille cases must never reach the IdP)"

  buc_step "Novating the ${RBCC_noun_sederunt} — opening a fresh one, extinguishing any preexisting ${RBCC_noun_sederunt}"
  zrbau_sederunt_open
}

######################################################################
# Federation branch — the don (Leg 3)
#
# rbau_don_capture — the impersonation act, as a capture. The federation-path
# sibling of rbau_token_capture: resolves a usable bearer token for an identity,
# here by minting a mantle service-account access token from the cached federated
# token via iamcredentials generateAccessToken. Emits the mantle token on stdout
# once on success; failure returns 1, except the Leg-3 403 admission deficit,
# which returns the distinguished BUBC_band_admission code — never buc_die_now,
# never stderr; the consuming verb supplies the loud
# buc_die_now (or a band-aware buc_reject) over the returned code, and the
# forensic lines below carry the operator instruction it dies with
# (matching rbau_avow's "failed at Leg N; see the transcript" division of
# labor). The unknown-identity guard returns 1 behind a forensic line — a caller
# bug, not a runtime condition — exactly as rbau_token_capture does.
#
# Custody: the mantle token reaches only this function's stdout, read in memory
# from the response the HTTP module's bearer road holds in memory — never a
# temp file and never the persistent sederunt cache. It carries exactly one mantle's authority and self-expires (1 h default
# ceiling, spike V1); donning again re-mints. A long run re-dons mid-flight while
# the sederunt lives; the re-mint ceiling is the sederunt itself — the
# cached-federated-token read below returns 1 once the sederunt lapses, carrying the
# avow instruction.
#
# The reveille-tier credless guard lives at the avowal entry (rbau_avow): no
# verb dons without a live sederunt, and the sederunt read below returns 1 when none
# is cached, so a credless run never reaches the mint.
rbau_don_capture() {
  zrbau_sentinel
  zrbrf_sentinel
  zrbcc_sentinel
  zrbgc_sentinel
  zrbdc_sentinel
  zrbuh_sentinel

  local -r z_identity="${1:-}"

  # Resolve the pallium-sprued mantle token to its mantle SA-name fragment. The
  # sprued token (rbpa_governor) is THE identity form every caller passes; the
  # underscore it carries never reaches the SA-id — it is consumed here and only
  # the bare rbma-<role> fragment composes the SA email below.
  local z_mantle_account
  case "${z_identity}" in
    "${RBCC_mantle_governor}")  z_mantle_account="${RBCC_account_mantle_governor}"  ;;
    "${RBCC_mantle_director}")  z_mantle_account="${RBCC_account_mantle_director}"  ;;
    "${RBCC_mantle_retriever}") z_mantle_account="${RBCC_account_mantle_retriever}" ;;
    *) buc_log_args "rbau_don_capture: unknown mantle token '${z_identity}' (expected ${RBCC_mantle_governor} | ${RBCC_mantle_director} | ${RBCC_mantle_retriever})"; return 1 ;;
  esac

  # The mantle SA lives in the depot project; the depot is also the quota project
  # named in the x-goog-user-project header below (spike F2). Raw email in the
  # path, matching the spike — the ':generateAccessToken' custom-method suffix
  # must stay literal, so the email is not urlencoded here.
  local -r z_mantle_email="${z_mantle_account}@${RBDC_DEPOT_PROJECT_ID}.${RBGC_SA_EMAIL_DOMAIN}"
  local -r z_don_url="${RBGC_API_ROOT_IAMCREDENTIALS}${RBGC_IAMCREDENTIALS_V1}/projects/-/serviceAccounts/${z_mantle_email}${RBGC_IAMCREDENTIALS_GENERATE_ACCESS_TOKEN_SUFFIX}"
  buc_log_args "Donning the ${z_identity} mantle: ${z_mantle_email}"

  # The bearer is the cached federated token. A miss (absent or within skew of
  # expiry) is the re-mint ceiling — the sederunt has lapsed; the forensic line
  # carries the avow instruction and the caller fails loud on the return 1.
  local zSECRET_federated=""
  local z_federated_status=0
  zSECRET_federated=$(zrbau_sederunt_read_capture) || z_federated_status=$?
  test "${z_federated_status}" -eq 0 || {
    buc_log_args "${RBCC_noun_sederunt_lettrine} lapsed — no live federated token is cached; avow to open a fresh ${RBCC_noun_sederunt}, then re-run (the mantle re-mint is capped by the ${RBCC_noun_sederunt}, not by the mantle token's own lifetime)"
    return 1
  }

  # Non-secret request body: cloud-platform scope, default lifetime (1 h ceiling).
  jq -n --arg scope "${RBGC_SCOPE_CLOUD_PLATFORM}" '{scope: [$scope]}' \
     > "${ZRBAU_FED_DON_BODY_FILE}" 2>"${ZRBAU_FED_JQ_STDERR_FILE}" || {
    buc_log_args "Failed to compose the don request body; see ${ZRBAU_FED_JQ_STDERR_FILE}"
    return 1
  }

  # Single generateAccessToken call through the bearer road's in-memory mode: the
  # response, carrying the mantle token, comes back as the body, a newline and the
  # HTTP code.
  local -r z_don_headers="x-goog-user-project: ${RBDC_DEPOT_PROJECT_ID}"$'\n'"Content-Type: application/json"
  local zSECRET_don=""
  local z_don_status=0
  zSECRET_don=$(rbuh_request_capture "POST" "${z_don_url}" "${zSECRET_federated}" "${z_don_headers}" "${ZRBAU_FED_CURL_STDERR_FILE}" "${ZRBAU_FED_DON_BODY_FILE}") || z_don_status=$?
  test "${z_don_status}" -eq 0 || {
    buc_log_args "Leg 3 (don) curl failed (its exit status stands above); see ${ZRBAU_FED_CURL_STDERR_FILE}"
    return 1
  }

  local -r z_newline=$'\n'
  local -r zSECRET_don_body="${zSECRET_don%"${z_newline}"*}"
  local z_code
  z_code=$(printf '%s' "${zSECRET_don}" | jq -Rrs 'split("\n") | last' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || return 1
  case "${z_code}" in
    200) ;;
    403)
      local z_errmsg_status=0
      local z_errmsg=""
      z_errmsg=$(printf '%s' "${zSECRET_don_body}" | jq -r '.error.message // empty' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || z_errmsg_status=$?
      test "${z_errmsg_status}" -eq 0 || z_errmsg=""
      # Deliberately not retried here (RBr_7a9, RBr_b93).
      buc_log_args "Leg 3 (don) denied (HTTP 403) for mantle ${z_mantle_email}: ${z_errmsg} — either an admission deficit (brevet the avowed citizen onto the mantle: tokenCreator on the mantle SA + serviceUsageConsumer on the depot project) or, immediately after gird/brevet, a just-written grant still propagating (wait a minute and retry); not retried here"
      return "${BUBC_band_admission}" ;;
    *)
      local z_failmsg_status=0
      local z_failmsg=""
      z_failmsg=$(printf '%s' "${zSECRET_don_body}" | jq -r '.error.message // empty' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || z_failmsg_status=$?
      test "${z_failmsg_status}" -eq 0 || z_failmsg=""
      buc_log_args "Leg 3 (don) failed (HTTP ${z_code}) for mantle ${z_mantle_email}: ${z_failmsg:-no error message in the response}"
      return 1 ;;
  esac

  # select(length > 0) yields a non-zero jq exit for an absent/empty token, and
  # the forensic line rides that exit status.
  local z_jq_status=0
  local zSECRET_mantle=""
  zSECRET_mantle=$(printf '%s' "${zSECRET_don_body}" | jq -er '.accessToken // empty | select(length > 0)' 2>"${ZRBAU_FED_JQ_STDERR_FILE}") || z_jq_status=$?
  test "${z_jq_status}" -eq 0 || {
    buc_log_args "Leg 3 (don) returned no accessToken"
    return 1
  }

  echo "${zSECRET_mantle}"
}

# eof
