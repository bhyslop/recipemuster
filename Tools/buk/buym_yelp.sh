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
# Bash Utility Yelp Module — Diastema wire format with yawp functions
#
# Yelp yawp functions are inert marker-stampers.  They embed non-printing
# diastema byte markers via pure string assignment into a shared group
# return variable (z_buym_yelp).  No subshells, no stdout, no stderr,
# cannot fail.
#
# All terminal capability decisions are deferred to buyf_format_yawp,
# which resolves diastema markers into ANSI color sequences and OSC-8
# hyperlinks at display time.
#
# Namespace:
#   buyy_  — yelp yawp functions (set z_buym_yelp)
#   buyc_  — configurators (set ZBUYM_CONFIG_MODE before kindle)
#   buyf_  — format yawp (set z_buym_format)
#   buym_  — module infrastructure (kindle, sentinel)
#
# Usage pattern:
#   buyy_cmd_yawp "git status"
#   local -r z_cmd="${z_buym_yelp}"
#   buyy_link_yawp "${z_docs}" "Depot"
#   local -r z_depot="${z_buym_yelp}"
#   buh_line "Run ${z_cmd} to see your ${z_depot} status."

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBUYM_SOURCED:-}" || return 0
ZBUYM_SOURCED=1

######################################################################
# Configurators (buyc_*)
#
# Called before kindle.  Each sets ZBUYM_CONFIG_MODE flag.
# Kindle reads the mode and defines readonly BUYC_* palette.

ZBUYM_CONFIG_MODE="dispatch"

buyc_dispatch()      { ZBUYM_CONFIG_MODE="dispatch"; }
buyc_unconditional() { ZBUYM_CONFIG_MODE="unconditional"; }
buyc_plain()         { ZBUYM_CONFIG_MODE="plain"; }

######################################################################
# Module kindle — defines all constants and initializes mutable state

zbuym_kindle() {
  test -z "${ZBUYM_KINDLED:-}" || return 0

  local z_use_color=0

  case "${ZBUYM_CONFIG_MODE}" in
    unconditional)
      z_use_color=1
      ;;
    plain)
      z_use_color=0
      ;;
    dispatch|*)
      if test -z "${NO_COLOR:-}" && test -n "${TERM:-}" && test "${TERM}" != "dumb"; then
        z_use_color=1
      fi
      ;;
  esac

  # --- Public color constants (BUYC_*) ---
  if test "${z_use_color}" = "1"; then
    readonly BUYC_RESET="\033[0m"
    readonly BUYC_CYAN="\033[36m"
    readonly BUYC_MAGENTA="\033[35m"
    readonly BUYC_BRIGHT_YELLOW="\033[1;33m"
    readonly BUYC_BRIGHT_RED="\033[1;31m"
    readonly BUYC_BRIGHT_WHITE="\033[1;37m"
    readonly BUYC_LINK="\033[34;4m"
    readonly BUYC_HREF="\033[34;4m"
  else
    readonly BUYC_RESET=""
    readonly BUYC_CYAN=""
    readonly BUYC_MAGENTA=""
    readonly BUYC_BRIGHT_YELLOW=""
    readonly BUYC_BRIGHT_RED=""
    readonly BUYC_BRIGHT_WHITE=""
    readonly BUYC_LINK=""
    readonly BUYC_HREF=""
  fi

  # --- Hyperlink mode ---
  local z_hyperlinks=0
  if test "${z_use_color}" = "1" && test -z "${BURD_NO_HYPERLINKS:-}"; then
    z_hyperlinks=1
  fi
  readonly ZBUYM_USE_HYPERLINKS="${z_hyperlinks}"

  # --- Diastema markers (non-printing byte sequences) ---
  # Each marker is a unique non-printing sequence that yelp yawp functions
  # stamp into strings.  buyf_format_yawp resolves them at display time.
  # Prefix byte is \x02 (STX) — \x01 (SOH) is reserved by bash internally
  # for BASH_REMATCH group delimiting and gets silently dropped from matches.
  readonly ZBUYM_DIASTEMA_CMD=$'\x02\x11'
  readonly ZBUYM_DIASTEMA_UI=$'\x02\x12'
  readonly ZBUYM_DIASTEMA_HREF_URL=$'\x02\x13'
  readonly ZBUYM_DIASTEMA_HREF_TEXT=$'\x02\x14'
  readonly ZBUYM_DIASTEMA_LINK_URL=$'\x02\x15'
  readonly ZBUYM_DIASTEMA_LINK_TEXT=$'\x02\x16'
  readonly ZBUYM_DIASTEMA_TT=$'\x02\x17'
  readonly ZBUYM_DIASTEMA_END=$'\x02\x18'

  # --- Mutable kindle state for yawp groups ---
  z_buym_yelp=""
  z_buym_format=""

  # --- Legacy capture compatibility constants ---
  # buy_yelp.sh consumers (rbyc_common.sh) still use capture functions.
  # These constants replicate ZBUY_* so the old API continues to work
  # until rbyc migrates to yawp.
  readonly ZBUY_R="${BUYC_RESET}"
  readonly ZBUY_C="${BUYC_CYAN}"
  readonly ZBUY_U="${BUYC_MAGENTA}"
  readonly ZBUY_L="${BUYC_LINK}"
  readonly ZBUY_USE_HYPERLINKS="${z_use_color}"
  if test -n "${BURD_NO_HYPERLINKS:-}"; then
    readonly ZBUY_NO_HYPERLINKS=1
  else
    readonly ZBUY_NO_HYPERLINKS=0
  fi
  readonly ZBUY_KINDLED=1

  readonly ZBUYM_KINDLED=1
}

zbuym_sentinel() {
  test "${ZBUYM_KINDLED:-}" = "1" || { zbuym_kindle; }
}

# Legacy sentinel alias — rbyc_common.sh calls zbuy_sentinel
zbuy_sentinel() { zbuym_sentinel; }

######################################################################
# Legacy capture functions
#
# These replicate buy_yelp.sh's public API so existing consumers
# (rbyc_common.sh) continue to work without modification.
# They produce stdout for $() capture — the old pattern.

buy_link_capture() {
  zbuym_sentinel
  local -r z_base="${1:-}"
  local -r z_anchor="${2:-}"
  local -r z_display="${3:-${z_anchor}}"
  local -r z_url="${z_base}#${z_anchor}"

  if test "${ZBUY_USE_HYPERLINKS}" = "1" && test "${ZBUY_NO_HYPERLINKS}" = "0"; then
    printf '%b' "${ZBUY_L}\033]8;;${z_url}\033\\\\${z_display}\033]8;;\033\\\\${ZBUY_R}"
  elif test "${ZBUY_USE_HYPERLINKS}" = "1"; then
    printf '%b' "${ZBUY_L}${z_display}${ZBUY_R} <${z_url}>"
  else
    printf '%s' "${z_display}"
  fi
}

buy_cmd_capture() {
  zbuym_sentinel
  local -r z_text="${1:-}"
  if test -n "${ZBUY_C}"; then
    printf '%b' "${ZBUY_C}${z_text}${ZBUY_R}"
  else
    printf '%s' "${z_text}"
  fi
}

buy_ui_capture() {
  zbuym_sentinel
  local -r z_text="${1:-}"
  if test -n "${ZBUY_U}"; then
    printf '%b' "${ZBUY_U}${z_text}${ZBUY_R}"
  else
    printf '%s' "${z_text}"
  fi
}

buy_tt_capture() {
  zbuym_sentinel
  local -r z_colophon="${1:-}"
  local z_matches=("${BURD_TABTARGET_DIR}"/${z_colophon}.*)
  test -e "${z_matches[0]}" || { printf '%s' "??${z_colophon}??"; return 0; }
  if test -n "${ZBUY_C}"; then
    printf '%b' "${ZBUY_C}${z_matches[0]}${ZBUY_R}"
  else
    printf '%s' "${z_matches[0]}"
  fi
}

# Legacy configurator aliases
buy_configure_dispatch()      { buyc_dispatch; }
buy_configure_unconditional() { buyc_unconditional; }
buy_configure_plain()         { buyc_plain; }

# Legacy kindle alias
zbuy_kindle() { zbuym_kindle; }

######################################################################
# Yelp yawp functions (buyy_*)
#
# All set z_buym_yelp via pure assignment.  No stdout, no stderr,
# no process spawning.  Cannot fail.

# buyy_cmd_yawp text — stamps CMD region
buyy_cmd_yawp() {
  zbuym_sentinel
  z_buym_yelp="${ZBUYM_DIASTEMA_CMD}${1:-}${ZBUYM_DIASTEMA_END}"
}

# buyy_ui_yawp text — stamps UI region
buyy_ui_yawp() {
  zbuym_sentinel
  z_buym_yelp="${ZBUYM_DIASTEMA_UI}${1:-}${ZBUYM_DIASTEMA_END}"
}

# buyy_href_yawp url display — stamps HREF region with raw URL
buyy_href_yawp() {
  zbuym_sentinel
  z_buym_yelp="${ZBUYM_DIASTEMA_HREF_URL}${1:-}${ZBUYM_DIASTEMA_HREF_TEXT}${2:-}${ZBUYM_DIASTEMA_END}"
}

# buyy_link_yawp base_url anchor [display] — stamps LINK region
buyy_link_yawp() {
  zbuym_sentinel
  local -r z_url="${1:-}#${2:-}"
  local -r z_display="${3:-${2:-}}"
  z_buym_yelp="${ZBUYM_DIASTEMA_LINK_URL}${z_url}${ZBUYM_DIASTEMA_LINK_TEXT}${z_display}${ZBUYM_DIASTEMA_END}"
}

# buyy_tt_yawp colophon [imprint] — resolves tabtarget, stamps TT region
buyy_tt_yawp() {
  zbuym_sentinel
  local -r z_colophon="${1:-}"
  local z_path=""
  if test -n "${2:-}"; then
    local z_matches=("${BURD_TABTARGET_DIR}"/${z_colophon}.*.${2}.sh)
    test -e "${z_matches[0]}" && z_path="${z_matches[0]}" || z_path="??${z_colophon}.${2}??"
  else
    local z_matches=("${BURD_TABTARGET_DIR}"/${z_colophon}.*)
    test -e "${z_matches[0]}" && z_path="${z_matches[0]}" || z_path="??${z_colophon}??"
  fi
  z_buym_yelp="${ZBUYM_DIASTEMA_TT}${z_path}${ZBUYM_DIASTEMA_END}"
}

######################################################################
# Format yawp (buyf_*)
#
# buyf_format_yawp color string
#   The single intelligence point.  Takes a BUYC_* color constant
#   (the line's ambient color) and a diastema-marked string.
#   Sets z_buym_format with the resolved string.
#
# Resolution:
#   1. Simple markers (CMD, UI, TT) → corresponding BUYC_* color
#   2. Structured markers (HREF, LINK) → OSC-8 or fallback
#   3. All DIASTEMA_END → ambient color
#   4. Prepend ambient, append BUYC_RESET

buyf_format_yawp() {
  zbuym_sentinel
  local z_ambient="${1:-}"
  local z_str="${2:-}"

  # If string has no diastema markers, fast path
  case "${z_str}" in
    *$'\x02'*) ;;
    *)
      z_buym_format="${z_ambient}${z_str}${BUYC_RESET}"
      return 0
      ;;
  esac

  # --- Resolve structured markers first (HREF and LINK) ---
  # These have URL data between opener and text marker, so they need
  # regex extraction before simple replacement can work.

  # Process HREF markers
  local z_href_pattern="${ZBUYM_DIASTEMA_HREF_URL}([^${ZBUYM_DIASTEMA_HREF_TEXT}]*)${ZBUYM_DIASTEMA_HREF_TEXT}([^${ZBUYM_DIASTEMA_END}]*)${ZBUYM_DIASTEMA_END}"
  while [[ "${z_str}" =~ ${z_href_pattern} ]]; do
    local z_href_url="${BASH_REMATCH[1]}"
    local z_href_text="${BASH_REMATCH[2]}"
    local z_href_full="${BASH_REMATCH[0]}"
    local z_href_replacement=""
    if test "${ZBUYM_USE_HYPERLINKS}" = "1"; then
      z_href_replacement="${BUYC_HREF}\033]8;;${z_href_url}\033\\\\${z_href_text}\033]8;;\033\\\\${z_ambient}"
    elif test -n "${BUYC_HREF}"; then
      z_href_replacement="${BUYC_HREF}${z_href_text}${z_ambient} <${z_href_url}>"
    else
      z_href_replacement="${z_href_text}"
    fi
    z_str="${z_str/${z_href_full}/${z_href_replacement}}"
  done

  # Process LINK markers
  local z_link_pattern="${ZBUYM_DIASTEMA_LINK_URL}([^${ZBUYM_DIASTEMA_LINK_TEXT}]*)${ZBUYM_DIASTEMA_LINK_TEXT}([^${ZBUYM_DIASTEMA_END}]*)${ZBUYM_DIASTEMA_END}"
  while [[ "${z_str}" =~ ${z_link_pattern} ]]; do
    local z_link_url="${BASH_REMATCH[1]}"
    local z_link_text="${BASH_REMATCH[2]}"
    local z_link_full="${BASH_REMATCH[0]}"
    local z_link_replacement=""
    if test "${ZBUYM_USE_HYPERLINKS}" = "1"; then
      z_link_replacement="${BUYC_LINK}\033]8;;${z_link_url}\033\\\\${z_link_text}\033]8;;\033\\\\${z_ambient}"
    elif test -n "${BUYC_LINK}"; then
      z_link_replacement="${BUYC_LINK}${z_link_text}${z_ambient} <${z_link_url}>"
    else
      z_link_replacement="${z_link_text}"
    fi
    z_str="${z_str/${z_link_full}/${z_link_replacement}}"
  done

  # --- Resolve simple markers ---
  z_str="${z_str//${ZBUYM_DIASTEMA_CMD}/${BUYC_CYAN}}"
  z_str="${z_str//${ZBUYM_DIASTEMA_UI}/${BUYC_MAGENTA}}"
  z_str="${z_str//${ZBUYM_DIASTEMA_TT}/${BUYC_CYAN}}"
  z_str="${z_str//${ZBUYM_DIASTEMA_END}/${z_ambient}}"

  z_buym_format="${z_ambient}${z_str}${BUYC_RESET}"
}

# eof
