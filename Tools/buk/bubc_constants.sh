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
# BUBC — BUK Bootstrap Constants
# Source-time literal constants for BUK kit layout.
# No kindle dependency — available immediately upon sourcing.

# Guard against multiple inclusion
test -z "${ZBUBC_SOURCED:-}" || return 0
ZBUBC_SOURCED=1

# Source-time literal constants
BUBC_rbmn_nodes_subdir="rbmn_nodes"
BUBC_rbmu_users_subdir="rbmu_users"

# Windows OpenSSH layout — forward slashes throughout so identical strings
# work in PowerShell, terminal display, and icacls invocations.
BUBC_windows_sshd_config='C:/ProgramData/ssh/sshd_config'
BUBC_windows_admin_auth_keys='C:/ProgramData/ssh/administrators_authorized_keys'
BUBC_windows_ssh_port="22"
BUBC_windows_fw_rule_name="sshd"
BUBC_windows_fw_display_name="OpenSSH Server"

# eof
