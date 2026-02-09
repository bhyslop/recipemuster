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
# BUK Zipper - Colophon registry via parallel arrays

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBUZ_SOURCED:-}" || return 0
ZBUZ_SOURCED=1

######################################################################
# Internal kindle boilerplate

zbuz_kindle() {
  test -z "${ZBUZ_KINDLED:-}" || buc_die "buz already kindled"

  # Initialize empty arrays
  zbuz_colophons=()
  zbuz_modules=()
  zbuz_commands=()

  ZBUZ_KINDLED=1
}

######################################################################
# Internal sentinel

zbuz_sentinel() {
  test "${ZBUZ_KINDLED:-}" = "1" || buc_die "Module buz not kindled - call zbuz_kindle first"
}

######################################################################
# Public registry operations

# buz_create_capture() - Appends a tuple to the arrays, prints the index
# Args: colophon, module, command
# Returns: index of newly created entry
buz_create_capture() {
  zbuz_sentinel

  local colophon="$1"
  local module="$2"
  local command="$3"

  zbuz_colophons+=("$colophon")
  zbuz_modules+=("$module")
  zbuz_commands+=("$command")

  local index=$((${#zbuz_colophons[@]} - 1))
  echo "$index"
}

# buz_get_colophon() - Getter for colophon at index
# Args: index
# Returns: colophon string
buz_get_colophon() {
  zbuz_sentinel

  local idx="$1"
  echo "${zbuz_colophons[$idx]}"
}

# buz_get_module() - Getter for module at index
# Args: index
# Returns: module name
buz_get_module() {
  zbuz_sentinel

  local idx="$1"
  echo "${zbuz_modules[$idx]}"
}

# buz_get_command() - Getter for command at index
# Args: index
# Returns: command/function name
buz_get_command() {
  zbuz_sentinel

  local idx="$1"
  echo "${zbuz_commands[$idx]}"
}

# eof
