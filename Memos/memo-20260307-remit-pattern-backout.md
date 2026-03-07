# Memo: BCG _remit Pattern — Backout and Future Reference

**Date**: 2026-03-07
**Context**: Heat ₣Ap (rbk-use-bash-remits) stabled; decided to back out remit infrastructure and reconsider later.

## What Was Built

The `_remit` function pattern was added to BCG as a new special function type alongside `_capture` and `_predicate`. It solves one specific problem: **exit-status-swallowing when `_capture` output is destructured via `IFS read <<<`**.

When you do `IFS=' ' read -r z_a z_b <<< "$(func_capture)"`, the `read` builtin always succeeds even if `func_capture` fails — the exit status is silently swallowed. The `_remit` pattern embeds a sentinel (`BUC_REMIT_VALID`) as the first field so the caller can verify the function completed its contract.

## Git Commits (all 2026-03-04, pace ₢AlAAG)

1. **4c9f965b** — Define _remit function pattern in BCG; add BUC_REMIT_VALID, BUC_REMIT_DELIMITER, buc_remit_assert to buc_command.sh
   - Files: `Tools/buk/buc_command.sh`, `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`

2. **58348244** — Add _remit to error handling decision table; clarify return-1-on-failure contract
   - Files: `Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`

3. **b74534ef** — Add rbgu_http_json_remit + rbgu_http_ok_remit (also fixed legacy rbgu_http_json cp to bare-infix paths — that fix is unrelated and was kept)
   - Files: `Tools/rbw/rbgu_Utility.sh`

Note: Commit 7918c3e9 (curl timeout bounds) later added `--connect-timeout` and `--max-time` to the remit functions too.

## Why It Was Backed Out

The _remit pattern was created to support migrating ~97 rbgu_http_json call sites to a new return convention. The migration (heat ₣Ap) was never started — the two implementation functions (`rbgu_http_json_remit`, `rbgu_http_ok_remit`) had zero external callers. Keeping unused infrastructure in the codebase adds maintenance burden and conceptual load to BCG without delivering value yet. Better to remove it and re-introduce when there's a concrete consumer.

## Preserved Code: buc_command.sh Infrastructure

```bash
######################################################################
# Remit infrastructure — structured multi-value return with sentinel

readonly BUC_REMIT_VALID="REMIT_OK"
readonly BUC_REMIT_DELIMITER="|"

buc_remit_assert() {
  local z_sentinel="${1:-}"
  local z_context="${2:-}"

  test "${z_sentinel}" = "${BUC_REMIT_VALID}" \
    || buc_die "${z_context}: remit sentinel invalid (got '${z_sentinel}')"
}
```

## Preserved Code: rbgu_http_json_remit (final version with curl timeouts)

```bash
rbgu_http_json_remit() {
  zrbgu_sentinel

  local -r z_method="${1}"
  local -r z_url="${2}"
  local -r z_token="${3}"
  local -r z_infix="${4}"
  local -r z_body_file="${5:-}"

  local z_curl_status=0
  local z_attempt=0
  local -r z_max_attempts=3
  local -r z_retry_sleep=3

  while :; do
    z_curl_status=0
    z_attempt=$((z_attempt + 1))

    local z_resp_file="${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_JSON}"
    local z_code_file="${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_CODE}"
    local z_code_errs="${ZRBGU_PREFIX}${z_infix}${ZRBGU_POSTFIX_CODE}.stderr"

    if test -n "${z_body_file}"; then
      curl                                              \
          -sS                                           \
          --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
          --max-time "${RBCC_CURL_MAX_TIME_SEC}"        \
          -X "${z_method}"                              \
          -H "Authorization: Bearer ${z_token}"         \
          -H "Content-Type: application/json"           \
          -H "Accept: application/json"                 \
          -d @"${z_body_file}"                          \
          -o "${z_resp_file}"                           \
          -w "%{http_code}"                             \
          "${z_url}" > "${z_code_file}"                 \
                    2> "${z_code_errs}"                 \
        || z_curl_status=$?
    else
      curl                                              \
          -sS                                           \
          --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" \
          --max-time "${RBCC_CURL_MAX_TIME_SEC}"        \
          -X "${z_method}"                              \
          -H "Authorization: Bearer ${z_token}"         \
          -H "Content-Type: application/json"           \
          -H "Accept: application/json"                 \
          -o "${z_resp_file}"                           \
          -w "%{http_code}"                             \
          "${z_url}" > "${z_code_file}"                 \
                    2> "${z_code_errs}"                 \
        || z_curl_status=$?
    fi

    buc_log_args 'Curl status' "${z_curl_status}"
    buc_log_pipe < "${z_code_errs}"

    test "${z_curl_status}" -ne 0 || break

    case "${z_curl_status}" in
      7|28|35|56) ;;
      *)
        buc_log_args "HTTP request failed (curl exit ${z_curl_status})"
        return 1
        ;;
    esac

    if test "${z_attempt}" -ge "${z_max_attempts}"; then
      buc_log_args "HTTP request failed after ${z_max_attempts} attempts (curl exit ${z_curl_status})"
      return 1
    fi

    buc_log_args "Transient curl error (exit ${z_curl_status}), retry ${z_attempt}/${z_max_attempts} in ${z_retry_sleep}s"
    sleep "${z_retry_sleep}"
  done

  local z_code
  z_code=$(<"${z_code_file}") || { buc_log_args "Failed to read code file"; return 1; }
  test -n "${z_code}"         || { buc_log_args "Empty HTTP code from curl"; return 1; }

  buc_log_args "HTTP ${z_method} ${z_url} returned code ${z_code}"
  printf '%s' "${BUC_REMIT_VALID}${BUC_REMIT_DELIMITER}${z_code}${BUC_REMIT_DELIMITER}${z_resp_file}"
}
```

## Preserved Code: rbgu_http_ok_remit

```bash
rbgu_http_ok_remit() {
  zrbgu_sentinel

  local -r z_label="${1}"
  local -r z_token="${2}"
  local -r z_method="${3}"
  local -r z_url="${4}"
  local -r z_infix="${5}"
  local -r z_body_file="${6}"
  local -r z_warn_code="${7:-}"
  local -r z_warn_msg="${8:-}"

  buc_log_args "${z_label}"

  local z_remit_valid z_code z_resp
  IFS="${BUC_REMIT_DELIMITER}" read -r z_remit_valid z_code z_resp \
    <<< "$(rbgu_http_json_remit "${z_method}" "${z_url}" "${z_token}" "${z_infix}" "${z_body_file:-}")"

  test "${z_remit_valid}" = "${BUC_REMIT_VALID}" || return 1

  case "${z_code}" in
    200|201|204)
      printf '%s' "${BUC_REMIT_VALID}${BUC_REMIT_DELIMITER}${z_code}${BUC_REMIT_DELIMITER}${z_resp}"
      return 0
      ;;
  esac

  if test -n "${z_warn_code}" && test "${z_code}" = "${z_warn_code}"; then
    printf '%s' "${BUC_REMIT_VALID}${BUC_REMIT_DELIMITER}${z_code}${BUC_REMIT_DELIMITER}${z_resp}"
    return 0
  fi

  local z_err=""
  if jq -e . "${z_resp}" >/dev/null 2>&1; then
    z_err=$(rbgu_json_field_capture "${z_infix}" '.error.message') || z_err="Unknown error"
  else
    local z_content
    z_content=$(<"${z_resp}") || z_content=""
    z_err="${z_content:0:200}"
    z_err="${z_err//$'\n'/ }"
    z_err="${z_err//$'\r'/ }"
    test -n "${z_err}" || z_err="Non-JSON error body"
  fi

  buc_log_args "${z_label} (HTTP ${z_code}): ${z_err}"
  return 1
}
```

## BCG Pattern Documentation (summary of what was added)

The _remit pattern was documented in BCG with:
- Function type table entry (line 656)
- Full section "Remit Functions" with fixed-arity and variable-length examples
- Caller two-line pattern: `IFS="${BUC_REMIT_DELIMITER}" read -r z_remit_valid ... <<< "$(func_remit)"` then `buc_remit_assert "${z_remit_valid}" "context"`
- Contract: sentinel first, pipe-delimited, no trailing newline, return 1 on failure (no buc_die), no spaces in field values
- Error handling decision table entry
- Naming table entries (Remit Internal/Public functions)
- Structured return quick-reference entry
- Checklist items (7 items covering emit format, caller destructuring, _capture safety)
- Quoting exception for variable-length iteration

## Heat ₣Ap Paces (preserved for future reference)

- **₢ApAAA** — investigate-or-true-suppression-patterns (|| true on rbgu_http_json calls)
- **₢ApAAB** — design-lro-polling-as-remit (Long-Running Operation wrapper architecture)
- **₢ApAAC** — migrate-rbgu-callers-to-remit (~97 call sites across 6 files)
- **₢ApAAD** — evict-http-legacy-and-evaluate-capture-unification (remove old infra + consider _capture -> _remit unification)
