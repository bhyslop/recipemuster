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
# Recipe Bottle Auth - RBRA/RBRO credential load and role token mint

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBA_SOURCED:-}" || buc_die "Module rba multiply sourced - check sourcing hierarchy"
ZRBA_SOURCED=1

######################################################################
# Internal Functions (zrba_*)

zrba_kindle() {
  test -z "${ZRBA_KINDLED:-}" || buc_die "Module rba already kindled"

  # Ensure dependency kindled first (rba mints tokens via rbgo)
  zrbgo_sentinel

  # Federation protocol constants — RFC 8628 device flow (Leg 1) + RFC 8693 STS
  # exchange (Leg 2). Pure protocol invariants with no config dependency, so they
  # set unconditionally; the per-trust audience is built from RBRF_* at Leg 2, not
  # here. The federated path is opt-in: only rba_compear and its legs read these,
  # and they guard on zrbrf_sentinel / zrbcc_sentinel, so the keyfile-only
  # consumers that source rba are untouched.
  readonly ZRBA_STS_ENDPOINT="https://sts.googleapis.com/v1/token"
  readonly ZRBA_STS_GRANT_TYPE="urn:ietf:params:oauth:grant-type:token-exchange"
  readonly ZRBA_STS_REQUESTED_TOKEN_TYPE="urn:ietf:params:oauth:token-type:access_token"
  readonly ZRBA_STS_SUBJECT_TOKEN_TYPE="urn:ietf:params:oauth:token-type:id_token"
  readonly ZRBA_STS_SCOPE="https://www.googleapis.com/auth/cloud-platform"
  readonly ZRBA_DEVICE_GRANT_TYPE="urn:ietf:params:oauth:grant-type:device_code"

  # Assize cache tuning: skew (treat the federated token as spent this many
  # seconds before its stated expiry) and the device-flow poll ceiling (device
  # codes self-expire in ~15 min).
  readonly ZRBA_ASSIZE_SKEW_SEC=60
  readonly ZRBA_DEVICE_POLL_MAX_SEC=900

  # Per-invocation scratch for the leg curls — BURD_TEMP_DIR (process lifetime),
  # never the assize cache, which must outlive one invocation (see rba_compear).
  readonly ZRBA_FED_DEVICE_RESPONSE_FILE="${BURD_TEMP_DIR}/rba_fed_device.json"
  readonly ZRBA_FED_TOKEN_RESPONSE_FILE="${BURD_TEMP_DIR}/rba_fed_token.json"
  readonly ZRBA_FED_STS_RESPONSE_FILE="${BURD_TEMP_DIR}/rba_fed_sts.json"
  readonly ZRBA_FED_DON_RESPONSE_FILE="${BURD_TEMP_DIR}/rba_fed_don.json"
  readonly ZRBA_FED_CURL_STDERR_FILE="${BURD_TEMP_DIR}/rba_fed_curl_stderr.txt"
  readonly ZRBA_FED_JQ_STDERR_FILE="${BURD_TEMP_DIR}/rba_fed_jq_stderr.txt"
  readonly ZRBA_FED_OPENSSL_STDERR_FILE="${BURD_TEMP_DIR}/rba_fed_openssl_stderr.txt"
  readonly ZRBA_FED_PROBE_STDERR_FILE="${BURD_TEMP_DIR}/rba_fed_probe_stderr.txt"

  # The don's request body is non-secret JSON (the scope list); it is composed
  # here rather than string-interpolated at the call site.
  readonly ZRBA_FED_DON_BODY_FILE="${BURD_TEMP_DIR}/rba_fed_don_body.json"

  # Non-secret scalar fields parsed out of the leg responses land in these temp
  # files: BCG bars $() on external commands, so jq/date write a file and the
  # value is read back with $(<file). The id, federated, and mantle tokens are
  # never among them — jq emits each straight to its function's stdout. The only
  # token-bearing temp files are the STS and don curl responses above (the
  # federated and mantle tokens respectively): both are per-invocation
  # BURD_TEMP_DIR scratch, never the persistent assize cache, which holds the
  # federated token alone — the mantle token is never cached anywhere.
  readonly ZRBA_FED_ASSIZE_EXPIRY_FILE="${BURD_TEMP_DIR}/rba_fed_assize_expiry.txt"
  readonly ZRBA_FED_ASSIZE_NOW_FILE="${BURD_TEMP_DIR}/rba_fed_assize_now.txt"
  readonly ZRBA_FED_COMPEAR_NOW_FILE="${BURD_TEMP_DIR}/rba_fed_compear_now.txt"
  readonly ZRBA_FED_DEVICE_CODE_FILE="${BURD_TEMP_DIR}/rba_fed_device_code.txt"
  readonly ZRBA_FED_USER_CODE_FILE="${BURD_TEMP_DIR}/rba_fed_user_code.txt"
  readonly ZRBA_FED_VERIFY_URI_FILE="${BURD_TEMP_DIR}/rba_fed_verify_uri.txt"
  readonly ZRBA_FED_INTERVAL_FILE="${BURD_TEMP_DIR}/rba_fed_interval.txt"
  readonly ZRBA_FED_POLL_ERROR_FILE="${BURD_TEMP_DIR}/rba_fed_poll_error.txt"
  readonly ZRBA_FED_EXPIRES_IN_FILE="${BURD_TEMP_DIR}/rba_fed_expires_in.txt"
  readonly ZRBA_FED_DON_CODE_FILE="${BURD_TEMP_DIR}/rba_fed_don_code.txt"
  readonly ZRBA_FED_DON_ERROR_FILE="${BURD_TEMP_DIR}/rba_fed_don_error.txt"

  readonly ZRBA_KINDLED=1
}

zrba_sentinel() {
  test "${ZRBA_KINDLED:-}" = "1" || buc_die "Module rba not kindled - call zrba_kindle first"
}

######################################################################
# External / RBTOE Pattern Functions

# The credential accessor — the single place credential material is resolved.
# Keyed by identity (governor | director | retriever): maps the identity to its
# RBDC_<ROLE>_RBRA_FILE and mints through rbgo. No call site outside this
# function touches credential material (source-side grep-gated). The keyfile mint
# here is bridge scaffolding; the federated-token path will branch in this one
# function when it lands.
rba_token_capture() {
  zrba_sentinel

  local -r z_identity="${1:-}"

  local z_rbra_file
  case "${z_identity}" in
    governor)  z_rbra_file="${RBDC_GOVERNOR_RBRA_FILE:-}"  ;;
    director)  z_rbra_file="${RBDC_DIRECTOR_RBRA_FILE:-}"  ;;
    retriever) z_rbra_file="${RBDC_RETRIEVER_RBRA_FILE:-}" ;;
    *) buc_die "rba_token_capture: unknown identity '${z_identity}' (expected governor | director | retriever)" ;;
  esac

  test -n "${z_rbra_file}" || return 1

  # return $? not 1: an in-band rejection from the mint (credless guard) must
  # survive this accessor so the buc_die membrane upstream re-exits it precisely.
  local z_token
  z_token=$(rbgo_get_token_capture "${z_rbra_file}") || return $?

  test -n "${z_token}" || return 1
  echo    "${z_token}"
}

######################################################################
# Federation branch — compearance (Leg 1) + STS exchange (Leg 2)
#
# The accessor's federated-token path. A human compears via the IdP device flow
# (Leg 1); the IdP id_token is exchanged at Google STS for a workforce federated
# access token (Leg 2); that federated token alone is cached, per-assize. The
# mantle token (Leg 3, the don) is a separate artifact, separately scoped, and
# never cached — it is not built here. The persisted assize cache is the clean
# producer/consumer seam between this path and the don.
#
# All federation config is read from RBRF_* (rbrf_regime); the leg curls reuse
# RBCC curl timeouts and rbgo's transient-curl classifier. Callers kindle
# rbrf + rbcc before invoking rba_compear; the functions guard on their sentinels.

# A controlling terminal is writable (a human is present to compear). Probes
# /dev/tty rather than `test -t 1` because a tabtarget's stdout is captured to
# the log, so stdout is never a TTY even interactively (same reason buc prompts
# write to /dev/tty). The redirect on the `:` builtin opens /dev/tty for that one
# command only — no subshell, no leaked shell FD — and fails when it cannot open.
zrba_tty_present_predicate() {
  : >/dev/tty 2>"${ZRBA_FED_PROBE_STDERR_FILE}" || return 1
  return 0
}

# Resolve the per-session assize cache path. Session-scoped — it spans tabtarget
# processes within one operator session — tmpfs-preferred, keyed by the trust so
# switching pools never crosses assizes. Dir 0700; the file is written 0600.
zrba_assize_path_capture() {
  zrba_sentinel
  zrbrf_sentinel

  local z_dir="${XDG_RUNTIME_DIR:-${TMPDIR:-/tmp}}"
  z_dir="${z_dir%/}/rbf-assize"
  mkdir -p  "${z_dir}" || return 1
  chmod 700 "${z_dir}" || return 1
  printf '%s/%s.%s.json' "${z_dir}" "${RBRF_WORKFORCE_POOL_ID}" "${RBRF_PROVIDER_ID}"
}

# Echo the cached federated token if present and not within skew of expiry;
# return 1 on any miss (absent, malformed, or expired).
zrba_assize_read_capture() {
  zrba_sentinel

  local z_path
  z_path=$(zrba_assize_path_capture) || return 1
  test -f "${z_path}" || return 1

  jq -r '.expiry_epoch // 0' "${z_path}" \
     > "${ZRBA_FED_ASSIZE_EXPIRY_FILE}" 2>"${ZRBA_FED_JQ_STDERR_FILE}" || return 1
  local -r z_expiry=$(<"${ZRBA_FED_ASSIZE_EXPIRY_FILE}")
  [[ "${z_expiry}" =~ ^[0-9]+$ ]] || return 1

  date +%s > "${ZRBA_FED_ASSIZE_NOW_FILE}" || return 1
  local -r z_now=$(<"${ZRBA_FED_ASSIZE_NOW_FILE}")
  test -n "${z_now}" || return 1
  test "${z_expiry}" -gt "$(( z_now + ZRBA_ASSIZE_SKEW_SEC ))" || return 1

  # Federated token (secret): jq emits it straight to stdout, never through a
  # shell var or temp file. select(length > 0) yields no output and a non-zero jq
  # exit when the token is absent or empty — the capture miss.
  jq -er '.federated_token // empty | select(length > 0)' "${z_path}" 2>"${ZRBA_FED_JQ_STDERR_FILE}"
}

# Echo the cached assize subject (the compeared oid) if present; return 1 on any
# miss. Informational mirror of zrba_assize_read_capture — the muniment-trail
# subject (decoded best-effort at compearance, non-load-bearing), not a
# credential, so no expiry gate: identity, not a token. select(length > 0) yields
# a non-zero jq exit for an absent/empty subject — the capture miss.
zrba_assize_subject_capture() {
  zrba_sentinel

  local z_path
  z_path=$(zrba_assize_path_capture) || return 1
  test -f "${z_path}" || return 1

  jq -er '.subject // empty | select(length > 0)' "${z_path}" 2>"${ZRBA_FED_JQ_STDERR_FILE}"
}

# A live (unexpired) assize is cached — status only, no output.
zrba_assize_live_predicate() {
  zrba_sentinel
  zrba_assize_read_capture >/dev/null || return 1
  return 0
}

# Atomically write the assize cache (federated token + expiry epoch + subject).
# The dir is 0700 (owner-only traversal); chmod 600 + temp-then-rename keeps the
# file owner-only and never partially visible under its stable name.
zrba_assize_write() {
  zrba_sentinel

  local -r z_token="${1:-}"
  local -r z_expiry_epoch="${2:-}"
  local -r z_subject="${3:-}"

  local z_path
  z_path=$(zrba_assize_path_capture) || return 1
  local -r z_tmp="${z_path}.tmp.$$"

  jq -n --arg t "${z_token}" --argjson e "${z_expiry_epoch}" --arg s "${z_subject}" \
     '{federated_token: $t, expiry_epoch: $e, subject: $s}' \
     > "${z_tmp}" 2>"${ZRBA_FED_JQ_STDERR_FILE}" || return 1
  chmod 600 "${z_tmp}" || return 1
  mv -f "${z_tmp}" "${z_path}" || return 1
}

# Best-effort: decode the federated subject (oid, else sub) from the IdP id_token
# payload. Informational only — cached toward the muniment trail, never load-bearing.
zrba_idtoken_subject_capture() {
  zrba_sentinel

  local z_payload="${1#*.}"
  z_payload="${z_payload%%.*}"
  test -n "${z_payload}" || return 1
  z_payload="${z_payload//-/+}"
  z_payload="${z_payload//_/\/}"
  case $(( ${#z_payload} % 4 )) in
    2) z_payload="${z_payload}==" ;;
    3) z_payload="${z_payload}="  ;;
  esac

  # Decode and select in one pipeline (capture-final): the decoded id_token
  # payload exists only in the pipe, never in a var or temp file. pipefail makes
  # a base64 or jq failure fail the pipeline, which the caller tolerates.
  printf '%s' "${z_payload}" \
    | openssl enc -base64 -d -A 2>"${ZRBA_FED_OPENSSL_STDERR_FILE}" \
    | jq -r '.oid // .sub // empty' 2>"${ZRBA_FED_JQ_STDERR_FILE}"
}

# Leg 1 — device-flow compearance (RFC 8628). Requests a device + user code,
# surfaces the verification URL and code to the human on /dev/tty, polls the IdP
# token endpoint until the human approves, and echoes the OIDC id_token. The
# id_token is never persisted — Leg 2 consumes it in-process.
zrba_leg1_idtoken_capture() {
  zrba_sentinel
  zrbrf_sentinel
  zrbcc_sentinel

  local z_status=0
  curl -sS -X POST "${RBRF_IDP_DEVICE_ENDPOINT}"         \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time        "${RBCC_CURL_MAX_TIME_SEC}"        \
    -H "Content-Type: application/x-www-form-urlencoded" \
    --data-urlencode "client_id=${RBRF_IDP_CLIENT_ID}"   \
    --data-urlencode "scope=${RBRF_IDP_SCOPE}"           \
    > "${ZRBA_FED_DEVICE_RESPONSE_FILE}" 2>"${ZRBA_FED_CURL_STDERR_FILE}" || z_status=$?
  test "${z_status}" -eq 0 \
    || { buc_log_args "Device-code request failed (curl ${z_status}); see ${ZRBA_FED_CURL_STDERR_FILE}"; return 1; }

  jq -r '.device_code // empty' "${ZRBA_FED_DEVICE_RESPONSE_FILE}" \
     > "${ZRBA_FED_DEVICE_CODE_FILE}" 2>"${ZRBA_FED_JQ_STDERR_FILE}" || return 1
  local -r z_device_code=$(<"${ZRBA_FED_DEVICE_CODE_FILE}")
  jq -r '.user_code // empty' "${ZRBA_FED_DEVICE_RESPONSE_FILE}" \
     > "${ZRBA_FED_USER_CODE_FILE}" 2>"${ZRBA_FED_JQ_STDERR_FILE}" || return 1
  local -r z_user_code=$(<"${ZRBA_FED_USER_CODE_FILE}")
  jq -r '.verification_uri // .verification_url // empty' "${ZRBA_FED_DEVICE_RESPONSE_FILE}" \
     > "${ZRBA_FED_VERIFY_URI_FILE}" 2>"${ZRBA_FED_JQ_STDERR_FILE}" || return 1
  local -r z_verification_uri=$(<"${ZRBA_FED_VERIFY_URI_FILE}")
  jq -r '.interval // 5' "${ZRBA_FED_DEVICE_RESPONSE_FILE}" \
     > "${ZRBA_FED_INTERVAL_FILE}" 2>"${ZRBA_FED_JQ_STDERR_FILE}" || return 1
  local z_interval=$(<"${ZRBA_FED_INTERVAL_FILE}")
  test -n "${z_device_code}"      || return 1
  test -n "${z_user_code}"        || return 1
  test -n "${z_verification_uri}" || return 1
  [[ "${z_interval}" =~ ^[0-9]+$ ]] || z_interval=5

  # Surface the prompt live on the controlling terminal (stdout is log-captured;
  # the user code is short-lived and single-use, kept off the persistent log).
  { printf '\nCompearance — sign in to open your assize:\n'
    printf '    %s\n'         "${z_verification_uri}"
    printf '    code: %s\n\n' "${z_user_code}"
  } >/dev/tty 2>"${ZRBA_FED_CURL_STDERR_FILE}" || return 1
  buc_log_args "Compearance prompt surfaced to terminal; polling for sign-in"

  local z_elapsed=0
  local z_err=""
  while test "${z_elapsed}" -lt "${ZRBA_DEVICE_POLL_MAX_SEC}"; do
    sleep "${z_interval}" || return 1
    z_elapsed=$(( z_elapsed + z_interval ))

    z_status=0
    curl -sS -X POST "${RBRF_IDP_TOKEN_ENDPOINT}"            \
      --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}"   \
      --max-time        "${RBCC_CURL_MAX_TIME_SEC}"          \
      -H "Content-Type: application/x-www-form-urlencoded"   \
      --data-urlencode "grant_type=${ZRBA_DEVICE_GRANT_TYPE}" \
      --data-urlencode "client_id=${RBRF_IDP_CLIENT_ID}"     \
      --data-urlencode "device_code=${z_device_code}"        \
      > "${ZRBA_FED_TOKEN_RESPONSE_FILE}" 2>"${ZRBA_FED_CURL_STDERR_FILE}" || z_status=$?
    if test "${z_status}" -ne 0; then
      rbgo_curl_status_is_transient_predicate "${z_status}" \
        || { buc_log_args "Device-flow poll failed (curl ${z_status})"; return 1; }
      continue
    fi

    # id_token present → jq emits it (secret) straight to stdout and we finish,
    # with no token landing in a var or temp file. select(length > 0) yields a
    # non-zero jq exit for an absent/empty token, keeping it on the polling path.
    if jq -er '.id_token // empty | select(length > 0)' \
         "${ZRBA_FED_TOKEN_RESPONSE_FILE}" 2>"${ZRBA_FED_JQ_STDERR_FILE}"; then
      return 0
    fi

    jq -r '.error // empty' "${ZRBA_FED_TOKEN_RESPONSE_FILE}" \
       > "${ZRBA_FED_POLL_ERROR_FILE}" 2>"${ZRBA_FED_JQ_STDERR_FILE}" || : > "${ZRBA_FED_POLL_ERROR_FILE}"
    z_err=$(<"${ZRBA_FED_POLL_ERROR_FILE}")
    case "${z_err}" in
      authorization_pending) ;;
      slow_down)             z_interval=$(( z_interval + 5 )) ;;
      *)                     buc_log_args "Device-flow authorization failed: ${z_err:-no id_token in response}"; return 1 ;;
    esac
  done

  buc_log_args "Device-flow timed out after ${ZRBA_DEVICE_POLL_MAX_SEC}s without approval"
  return 1
}

# Leg 2 — STS token exchange (RFC 8693). Exchanges the IdP id_token for a Google
# workforce federated access token. Unauthenticated POST; audience = the provider
# resource name; nothing else — no userProject, no auth header (spike F3). Echoes
# "<federated_token> <expires_in>".
zrba_leg2_federated_capture() {
  zrba_sentinel
  zrbrf_sentinel
  zrbcc_sentinel

  local -r z_idtoken="${1:-}"
  test -n "${z_idtoken}" || return 1

  local -r z_audience="//iam.googleapis.com/locations/global/workforcePools/${RBRF_WORKFORCE_POOL_ID}/providers/${RBRF_PROVIDER_ID}"

  local z_status=0
  curl -sS -X POST "${ZRBA_STS_ENDPOINT}"                                    \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}"                     \
    --max-time        "${RBCC_CURL_MAX_TIME_SEC}"                            \
    -H "Content-Type: application/x-www-form-urlencoded"                     \
    --data-urlencode "grant_type=${ZRBA_STS_GRANT_TYPE}"                     \
    --data-urlencode "audience=${z_audience}"                                \
    --data-urlencode "scope=${ZRBA_STS_SCOPE}"                               \
    --data-urlencode "requested_token_type=${ZRBA_STS_REQUESTED_TOKEN_TYPE}" \
    --data-urlencode "subject_token_type=${ZRBA_STS_SUBJECT_TOKEN_TYPE}"     \
    --data-urlencode "subject_token=${z_idtoken}"                            \
    > "${ZRBA_FED_STS_RESPONSE_FILE}" 2>"${ZRBA_FED_CURL_STDERR_FILE}" || z_status=$?
  test "${z_status}" -eq 0 \
    || { buc_log_args "STS exchange failed (curl ${z_status}); see ${ZRBA_FED_CURL_STDERR_FILE}"; return 1; }

  jq -r '.expires_in // 0' "${ZRBA_FED_STS_RESPONSE_FILE}" \
     > "${ZRBA_FED_EXPIRES_IN_FILE}" 2>"${ZRBA_FED_JQ_STDERR_FILE}" || return 1
  local z_expires=$(<"${ZRBA_FED_EXPIRES_IN_FILE}")
  [[ "${z_expires}" =~ ^[0-9]+$ ]] || z_expires=0

  # Federated access token (secret): jq emits "<token> <expires_in>" straight to
  # stdout, the validated expiry passed in as a jq arg, so the token never passes
  # through a shell var or temp file. select(length > 0) yields a non-zero jq exit
  # for an absent/empty token, and the forensic log rides that exit status.
  local z_status=0
  jq -er --argjson e "${z_expires}" \
     '(.access_token // "") | select(length > 0) | "\(.) \($e)"' \
     "${ZRBA_FED_STS_RESPONSE_FILE}" 2>"${ZRBA_FED_JQ_STDERR_FILE}" || z_status=$?
  test "${z_status}" -eq 0 \
    || { buc_log_args "STS exchange returned no access_token; see ${ZRBA_FED_STS_RESPONSE_FILE}"; return 1; }
}

# rba_compear — the compearance accessor step. Ensures a live assize; its side
# effect is the per-session cache, and consumers read the federated token with
# zrba_assize_read_capture. Cache-hit → done. Miss/expired with a terminal → run
# Legs 1+2 and cache. Miss with no terminal → fail loud: the headless fail-fast
# membrane, also the suite-head seam (an automated run compears once at suite
# head; the headless cases thereafter take the cache-hit path).
rba_compear() {
  zrba_sentinel
  zrbrf_sentinel
  zrbcc_sentinel

  # Credless guard — the fast tier must never touch the IdP or the network.
  # Mirrors the keyfile mint's guard so the federated path honors the same invariant.
  test "${BURE_TWEAK_NAME:-}" != "${RBCC_tweak_credless_guard}" \
    || buc_reject "${BUBC_band_credless}" "Credless guard: compearance refused — this run carries the fast-tier guard (fast cases must never reach the IdP)"

  if zrba_assize_live_predicate; then
    buc_step "Assize already live — reusing the cached federated token"
    return 0
  fi

  zrba_tty_present_predicate \
    || buc_die "No live assize and no terminal — a human must compear interactively to open one (this headless run cannot). Open an assize from a terminal, then re-run."

  buc_step "No live assize — opening one via device-flow compearance"

  local z_idtoken
  z_idtoken=$(zrba_leg1_idtoken_capture) || buc_die "Compearance failed at Leg 1 (device flow); see the transcript"

  local z_fed
  z_fed=$(zrba_leg2_federated_capture "${z_idtoken}") || buc_die "Compearance failed at Leg 2 (STS exchange); see the transcript"

  local -r z_federated="${z_fed%% *}"
  local -r z_expires_in="${z_fed##* }"
  [[ "${z_expires_in}" =~ ^[0-9]+$ ]] || buc_die "Leg 2 returned a non-numeric expiry: ${z_expires_in}"

  date +%s > "${ZRBA_FED_COMPEAR_NOW_FILE}" || buc_die "Failed to read the clock"
  local -r z_now=$(<"${ZRBA_FED_COMPEAR_NOW_FILE}")
  test -n "${z_now}" || buc_die "Empty clock reading"
  local -r z_expiry_epoch=$(( z_now + z_expires_in ))

  local z_subject
  z_subject=$(zrba_idtoken_subject_capture "${z_idtoken}") || z_subject=""

  zrba_assize_write "${z_federated}" "${z_expiry_epoch}" "${z_subject}" \
    || buc_die "Compearance succeeded but caching the assize failed"

  buc_step "Assize opened (federated token expires in ${z_expires_in}s)"
}

######################################################################
# Federation branch — the don (Leg 3)
#
# rba_don_capture — the impersonation act, as a capture. The federation-path
# sibling of rba_token_capture: resolves a usable bearer token for an identity,
# here by minting a mantle service-account access token from the cached federated
# token via iamcredentials generateAccessToken. Emits the mantle token on stdout
# once on success, or returns 1 — never buc_die, never stderr (BCG capture
# contract); the consuming verb supplies the loud buc_die over the returned 1,
# and the forensic lines below carry the operator instruction it dies with
# (matching rba_compear's "failed at Leg N; see the transcript" division of
# labor). The unknown-identity guard buc_dies — a caller bug, not a runtime
# condition — exactly as rba_token_capture does.
#
# Custody: the mantle token reaches only this function's stdout (jq straight to
# stdout, never a shell var) and the per-invocation curl response (BURD_TEMP_DIR,
# process lifetime, like the Leg-2 STS response) — never the persistent assize
# cache. It carries exactly one mantle's authority and self-expires (1 h default
# ceiling, spike V1); donning again re-mints. A long run re-dons mid-flight while
# the assize lives; the re-mint ceiling is the assize itself — the
# cached-federated-token read below returns 1 once the assize lapses, carrying the
# compear instruction.
#
# Single attempt by design. The Leg-3 403 is the structural admission-deficit
# Palisade signature (spike F2): a workforce federated token carries no
# API-consumer project, so a citizen not yet brevetted onto the mantle, or a
# missing quota-project header, 403s with "Method doesn't allow unregistered
# callers" — which reads like an API-key complaint, not an IAM denial, and no
# propagation wait fixes it. It is logged as the admission deficit it is and
# returned, NEVER retried as a propagation race, unlike the SA-propagation loops
# the keyfile accessor carries.
#
# The fast-tier credless guard lives at the compearance entry (rba_compear): no
# verb dons without a live assize, and the assize read below returns 1 when none
# is cached, so a credless run never reaches the mint.
rba_don_capture() {
  zrba_sentinel
  zrbrf_sentinel
  zrbcc_sentinel
  zrbgc_sentinel
  zrbdc_sentinel

  local -r z_identity="${1:-}"

  local z_mantle_account
  case "${z_identity}" in
    governor)  z_mantle_account="${RBCC_account_mantle_governor}"  ;;
    director)  z_mantle_account="${RBCC_account_mantle_director}"  ;;
    retriever) z_mantle_account="${RBCC_account_mantle_retriever}" ;;
    *) buc_die "rba_don_capture: unknown identity '${z_identity}' (expected governor | director | retriever)" ;;
  esac

  # The mantle SA lives in the depot project; the depot is also the quota project
  # named in the x-goog-user-project header below (spike F2). Raw email in the
  # path, matching the spike — the ':generateAccessToken' custom-method suffix
  # must stay literal, so the email is not urlencoded here.
  local -r z_mantle_email="${z_mantle_account}@${RBDC_DEPOT_PROJECT_ID}.${RBGC_SA_EMAIL_DOMAIN}"
  local -r z_don_url="${RBGC_API_ROOT_IAMCREDENTIALS}${RBGC_IAMCREDENTIALS_V1}/projects/-/serviceAccounts/${z_mantle_email}${RBGC_IAMCREDENTIALS_GENERATE_ACCESS_TOKEN_SUFFIX}"
  buc_log_args "Donning the ${z_identity} mantle: ${z_mantle_email}"

  # The bearer is the cached federated token. A miss (absent or within skew of
  # expiry) is the re-mint ceiling — the assize has lapsed; the forensic line
  # carries the compear instruction and the caller fails loud on the return 1.
  local z_federated
  z_federated=$(zrba_assize_read_capture) || {
    buc_log_args "Assize lapsed — no live federated token is cached; compear to open a fresh assize, then re-run (the mantle re-mint is capped by the assize, not by the mantle token's own lifetime)"
    return 1
  }

  # Non-secret request body: cloud-platform scope, default lifetime (1 h ceiling).
  jq -n --arg scope "${RBGC_SCOPE_CLOUD_PLATFORM}" '{scope: [$scope]}' \
     > "${ZRBA_FED_DON_BODY_FILE}" 2>"${ZRBA_FED_JQ_STDERR_FILE}" || {
    buc_log_args "Failed to compose the don request body; see ${ZRBA_FED_JQ_STDERR_FILE}"
    return 1
  }

  # Single generateAccessToken call. -o writes the response (mantle token) to the
  # curl-response scratch; -w prints the HTTP code to stdout, captured to a file.
  local z_status=0
  curl -sS -X POST "${z_don_url}"                        \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
    --max-time        "${RBCC_CURL_MAX_TIME_SEC}"        \
    -H "Authorization: Bearer ${z_federated}"            \
    -H "x-goog-user-project: ${RBDC_DEPOT_PROJECT_ID}"   \
    -H "Content-Type: application/json"                  \
    --data "@${ZRBA_FED_DON_BODY_FILE}"                  \
    -o "${ZRBA_FED_DON_RESPONSE_FILE}"                   \
    -w '%{http_code}'                                    \
    > "${ZRBA_FED_DON_CODE_FILE}" 2>"${ZRBA_FED_CURL_STDERR_FILE}" || z_status=$?
  test "${z_status}" -eq 0 || {
    buc_log_args "Leg 3 (don) curl failed (exit ${z_status}); see ${ZRBA_FED_CURL_STDERR_FILE}"
    return 1
  }

  local -r z_code=$(<"${ZRBA_FED_DON_CODE_FILE}")
  case "${z_code}" in
    200) ;;
    403)
      jq -r '.error.message // empty' "${ZRBA_FED_DON_RESPONSE_FILE}" \
         > "${ZRBA_FED_DON_ERROR_FILE}" 2>"${ZRBA_FED_JQ_STDERR_FILE}" \
         || : > "${ZRBA_FED_DON_ERROR_FILE}"
      local -r z_errmsg=$(<"${ZRBA_FED_DON_ERROR_FILE}")
      buc_log_args "Leg 3 (don) denied (HTTP 403) for mantle ${z_mantle_email}: ${z_errmsg} — admission deficit, not a propagation race; brevet the compeared citizen onto the mantle (tokenCreator on the mantle SA + serviceUsageConsumer on the depot project); not retried"
      return 1 ;;
    *)
      buc_log_args "Leg 3 (don) failed (HTTP ${z_code}) for mantle ${z_mantle_email}; see ${ZRBA_FED_DON_RESPONSE_FILE}"
      return 1 ;;
  esac

  # Mantle access token (secret): jq emits it straight to stdout, never through a
  # shell var or the persistent cache. select(length > 0) yields a non-zero jq
  # exit for an absent/empty token, and the forensic line rides that exit status.
  local z_jq_status=0
  jq -er '.accessToken // empty | select(length > 0)' \
     "${ZRBA_FED_DON_RESPONSE_FILE}" 2>"${ZRBA_FED_JQ_STDERR_FILE}" || z_jq_status=$?
  test "${z_jq_status}" -eq 0 || {
    buc_log_args "Leg 3 (don) returned no accessToken; see ${ZRBA_FED_DON_RESPONSE_FILE}"
    return 1
  }
}

rba_extract_json_to_rbra() {
  zrba_sentinel

  local -r z_json_path="$1"
  local -r z_rbra_path="$2"
  local -r z_lifetime_sec="$3"
  local -r z_expected_project_id="${4:-}"

  test -f "${z_json_path}" || buc_die "Service account JSON not found: ${z_json_path}"

  buc_info "Extracting service account credentials from JSON"

  buc_log_args 'Extract fields'
  local z_client_email
  z_client_email=$(jq -r '.client_email' "${z_json_path}") \
                                        || buc_die "Failed to extract client_email"
  test -n "${z_client_email}"           || buc_die "Empty client_email in JSON"
  test    "${z_client_email}" != "null" || buc_die "Null client_email in JSON"

  local z_private_key
  z_private_key=$(jq -r '.private_key' "${z_json_path}") \
                                       || buc_die "Failed to extract private_key"
  test -n "${z_private_key}"           || buc_die "Empty private_key in JSON"
  test    "${z_private_key}" != "null" || buc_die "Null private_key in JSON"

  local z_project_id
  z_project_id=$(jq -r '.project_id' "${z_json_path}") \
                                      || buc_die "Failed to extract project_id"
  test -n "${z_project_id}"           || buc_die "Empty project_id in JSON"
  test    "${z_project_id}" != "null" || buc_die "Null project_id in JSON"

  if test -n "${z_expected_project_id}"; then
    buc_log_args "Verify project matches expected: ${z_expected_project_id}"
    test "${z_project_id}" = "${z_expected_project_id}" \
      || buc_die "Project mismatch: JSON has '${z_project_id}', expected '${z_expected_project_id}'"
  else
    buc_log_args "No project validation - accepting JSON project_id: ${z_project_id}"
  fi

  buc_log_args 'Write RBRA file'
  # CAUTION: jq -r unescapes JSON \n to real newlines, so z_private_key holds a
  # multi-line PEM string. printf '%s' below preserves real newlines into the
  # RBRA file. Consumer (rbgo_oauth.sh:zrbgo_build_jwt_capture) tolerates either
  # real-newline or '\n'-escape form via printf '%b'; do not "normalize" to one
  # form without auditing the consumer.
  {
    printf 'RBRA_CLIENT_EMAIL="%s"\n'      "${z_client_email}"
    printf 'RBRA_PRIVATE_KEY="'; printf '%s' "${z_private_key}"; printf '"\n'
    printf 'RBRA_PROJECT_ID="%s"\n'        "${z_project_id}"
    printf 'RBRA_TOKEN_LIFETIME_SEC=%s\n'  "${z_lifetime_sec}"
  } > "${z_rbra_path}" || buc_die "Failed to write RBRA file ${z_rbra_path}"

  test -f "${z_rbra_path}" || buc_die "Failed to write RBRA file: ${z_rbra_path}"

  buc_warn "Consider deleting source JSON after verification: ${z_json_path}"
}

# RBTOE: RBRA Load Pattern
# Sources an RBRA file and validates required fields
rba_rbra_load() {
  zrba_sentinel

  local -r z_rbra_file="${1}"

  test -n "${z_rbra_file}" || buc_die "rba_rbra_load: RBRA file path required"
  test -f "${z_rbra_file}" || buc_die "rba_rbra_load: RBRA file not found: ${z_rbra_file}"

  buc_log_args "Loading and validating RBRA credentials from ${z_rbra_file}"

  # Source the RBRA file
  source "${z_rbra_file}" || buc_die "rba_rbra_load: failed to source RBRA file"

  # Validate required fields
  test -n "${RBRA_CLIENT_EMAIL:-}" || buc_die "rba_rbra_load: RBRA_CLIENT_EMAIL missing from ${z_rbra_file}"
  test -n "${RBRA_PRIVATE_KEY:-}" || buc_die "rba_rbra_load: RBRA_PRIVATE_KEY missing from ${z_rbra_file}"
  test -n "${RBRA_PROJECT_ID:-}" || buc_die "rba_rbra_load: RBRA_PROJECT_ID missing from ${z_rbra_file}"

  # Check for null values
  test "${RBRA_CLIENT_EMAIL}" != "null" || buc_die "rba_rbra_load: RBRA_CLIENT_EMAIL is null in ${z_rbra_file}"
  test "${RBRA_PRIVATE_KEY}" != "null" || buc_die "rba_rbra_load: RBRA_PRIVATE_KEY is null in ${z_rbra_file}"
  test "${RBRA_PROJECT_ID}" != "null" || buc_die "rba_rbra_load: RBRA_PROJECT_ID is null in ${z_rbra_file}"

  buc_log_args "RBRA validation successful: ${RBRA_CLIENT_EMAIL} in project ${RBRA_PROJECT_ID}"
}

# RBTOE: RBRO Load Pattern
# Thin wrapper: defensively sources rbro_regime.sh (callers don't need to know
# its path, which moved under AAD's payor/ subdirectory migration), then
# delegates to rbro_load. Parallels rba_rbra_load at the call-signature level
# even though that function carries its own validation; the uniform rba_*
# load-through-utility convention is the load-bearing reason this wrapper exists.
rba_rbro_load() {
  zrba_sentinel

  buc_log_args "Loading RBRO OAuth credentials"

  # Source regime module if not already loaded
  if test -z "${ZRBRO_SOURCED:-}"; then
    source "${BASH_SOURCE[0]%/*}/rbro_regime.sh"
  fi

  # Delegate to regime's canonical load
  rbro_load

  buc_log_args "RBRO validation successful"
}

# eof
