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
# RBBC — RBK Bootstrap Constants
# Source-time literal constants for .rbk/ file locations.
# No kindle dependency — available immediately upon sourcing.

# Guard against multiple inclusion
test -z "${ZRBBC_SOURCED:-}" || return 0
ZRBBC_SOURCED=1

# Source-time literal constants
RBBC_dot_dir=".rbk"
RBBC_rbrr_file="${RBBC_dot_dir}/rbrr.env"
RBBC_rbrp_file="${RBBC_dot_dir}/rbrp.env"

# eof
