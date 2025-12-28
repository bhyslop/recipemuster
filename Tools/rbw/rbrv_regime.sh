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
# Recipe Bottle Regime Vessel Validator

set -euo pipefail

ZRBRV_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
source "${ZRBRV_SCRIPT_DIR}/buv_validation.sh"

# Core Vessel Identity
buv_env_xname       RBRV_SIGIL                   1     64  # Must match directory name
buv_env_string      RBRV_DESCRIPTION             0    512  # Optional description

# Binding Configuration (for copying from registry)
if test -n       "${RBRV_BIND_IMAGE:-}"; then
    buv_env_fqin    RBRV_BIND_IMAGE              1    512  # Source image to copy
fi

# Conjuring Configuration (for building from source)
if test -n       "${RBRV_CONJURE_DOCKERFILE:-}"; then
    buv_env_string  RBRV_CONJURE_DOCKERFILE      1    512  # Path relative to repo root
    buv_env_string  RBRV_CONJURE_BLDCONTEXT      1    512  # Build context relative to repo root
    
    # Platform configuration - only meaningful for builds
    buv_env_string  RBRV_CONJURE_PLATFORMS       1    512  # Space-separated platforms (e.g. "linux/amd64 linux/arm64")
    buv_env_string  RBRV_CONJURE_BINFMT_POLICY   1     16  # Either "allow" or "forbid"
    case "${RBRV_CONJURE_BINFMT_POLICY}" in
        allow|forbid) : ;;
        *) buc_die "Invalid RBRV_CONJURE_BINFMT_POLICY: '${RBRV_CONJURE_BINFMT_POLICY}' (must be 'allow' or 'forbid')" ;;
    esac
fi

# Validate at least one operation mode is configured
if test -z "${RBRV_BIND_IMAGE:-}" && test -z "${RBRV_CONJURE_DOCKERFILE:-}"; then
    buc_die "Vessel must define either RBRV_BIND_IMAGE (for binding) or RBRV_CONJURE_DOCKERFILE (for conjuring)"
fi

# eof

