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
# Bash Utility Yelp — Formatted string capture for subshell assignment
#
# Capture functions produce formatted string fragments ("yelps") on stdout
# for $() capture into local variables.  Yelps embed ANSI color sequences
# and OSC-8 hyperlinks suitable for later interpolation into buh_line or
# printf calls.
#
# Dispatch-aware: respects terminal capabilities detected at kindle time.
# Three configurators select formatting behavior before kindle:
#   buy_configure_dispatch      — honor BURD_*/NO_COLOR/TERM (default)
#   buy_configure_unconditional — always apply ANSI/OSC-8
#   buy_configure_plain         — never format, plain text only
#
# Usage pattern:
#   local z_depot=$(buy_link_capture "$url" "Depot")
#   local z_cmd=$(buy_cmd_capture "git status")
#   buh_line "A ${z_depot} is where images live."
#
# Yelp content must not contain literal % characters.  This premise
# allows safe direct interpolation into printf format strings.
#
# All capture functions write to stdout.  They do NOT write to stderr.
# They produce fragments, not lines.

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBUY_INCLUDED:-}" || return 0
ZBUY_INCLUDED=1

######################################################################
# Internal: Configuration and Kindle

# Configuration mode — set before kindle, read at kindle time
# Values: "dispatch" (default), "unconditional", "plain"
ZBUY_CONFIG_MODE="dispatch"

buy_configure_dispatch()      { ZBUY_CONFIG_MODE="dispatch"; }
buy_configure_unconditional() { ZBUY_CONFIG_MODE="unconditional"; }
buy_configure_plain()         { ZBUY_CONFIG_MODE="plain"; }

zbuy_kindle() {
  test -z "${ZBUY_KINDLED:-}" || return 0

  local z_use_color=0

  case "${ZBUY_CONFIG_MODE}" in
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

  if test "${z_use_color}" = "1"; then
    readonly ZBUY_R="\033[0m"
    readonly ZBUY_C="\033[36m"
    readonly ZBUY_U="\033[35m"
    readonly ZBUY_L="\033[34;4m"
    readonly ZBUY_USE_HYPERLINKS=1
  else
    readonly ZBUY_R=""
    readonly ZBUY_C=""
    readonly ZBUY_U=""
    readonly ZBUY_L=""
    readonly ZBUY_USE_HYPERLINKS=0
  fi

  # Hyperlink fallback — honor BURD_NO_HYPERLINKS even when color is on
  if test -n "${BURD_NO_HYPERLINKS:-}"; then
    readonly ZBUY_NO_HYPERLINKS=1
  else
    readonly ZBUY_NO_HYPERLINKS=0
  fi

  readonly ZBUY_KINDLED=1
}

zbuy_sentinel() {
  test "${ZBUY_KINDLED:-}" = "1" || { zbuy_kindle; }
}

######################################################################
# Public: Capture functions
#
# All produce stdout for $() capture.  No stderr output.

# buy_link_capture base_url anchor [display_text]
#   OSC-8 hyperlink; display defaults to anchor
buy_link_capture() {
  zbuy_sentinel
  local -r z_base="${1:-}"
  local -r z_anchor="${2:-}"
  local -r z_display="${3:-${z_anchor}}"
  local -r z_url="${z_base}#${z_anchor}"

  if test "${ZBUY_USE_HYPERLINKS}" = "1" && test "${ZBUY_NO_HYPERLINKS}" = "0"; then
    printf '%b' "${ZBUY_L}\033]8;;${z_url}\033\\\\${z_display}\033]8;;\033\\\\${ZBUY_R}"
  elif test "${ZBUY_USE_HYPERLINKS}" = "1"; then
    # Color but no OSC-8
    printf '%b' "${ZBUY_L}${z_display}${ZBUY_R} <${z_url}>"
  else
    printf '%s' "${z_display}"
  fi
}

# buy_cmd_capture text
#   Command/code styling (cyan)
buy_cmd_capture() {
  zbuy_sentinel
  local -r z_text="${1:-}"
  if test -n "${ZBUY_C}"; then
    printf '%b' "${ZBUY_C}${z_text}${ZBUY_R}"
  else
    printf '%s' "${z_text}"
  fi
}

# buy_ui_capture text
#   UI element styling (magenta)
buy_ui_capture() {
  zbuy_sentinel
  local -r z_text="${1:-}"
  if test -n "${ZBUY_U}"; then
    printf '%b' "${ZBUY_U}${z_text}${ZBUY_R}"
  else
    printf '%s' "${z_text}"
  fi
}

# buy_tt_capture colophon
#   Resolved tabtarget path in command styling
buy_tt_capture() {
  zbuy_sentinel
  local -r z_colophon="${1:-}"
  local z_matches=("${BURD_TABTARGET_DIR}"/${z_colophon}.*)
  test -e "${z_matches[0]}" || { printf '%s' "??${z_colophon}??"; return 0; }
  if test -n "${ZBUY_C}"; then
    printf '%b' "${ZBUY_C}${z_matches[0]}${ZBUY_R}"
  else
    printf '%s' "${z_matches[0]}"
  fi
}

# eof
