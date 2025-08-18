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
source "${ZRBRV_SCRIPT_DIR}/bvu_BashValidationUtility.sh"

# Core Vessel Identity
bvu_env_xname       RBRV_SIGIL                   1     64  # Must match directory name
bvu_env_string      RBRV_DESCRIPTION             0    512  # Optional description

# Binding Configuration (for copying from registry)
if test -n       "${RBRV_BIND_IMAGE:-}"; then
    bvu_env_fqin    RBRV_BIND_IMAGE              1    512  # Source image to copy
fi

# Conjuring Configuration (for building from source)
if test -n       "${RBRV_CONJURE_DOCKERFILE:-}"; then
    bvu_env_string  RBRV_CONJURE_DOCKERFILE      1    512  # Path relative to repo root
    bvu_env_string  RBRV_CONJURE_BLDCONTEXT      1    512  # Build context relative to repo root
fi

# Validate at least one operation mode is configured
if test -z "${RBRV_BIND_IMAGE:-}" && test -z "${RBRV_CONJURE_DOCKERFILE:-}"; then
    bcu_die "Vessel must define either RBRV_BIND_IMAGE (for binding) or RBRV_CONJURE_DOCKERFILE (for conjuring)"
fi

# Validate sigil matches directory name (if we can determine it)
# This assumes the rbrv.env file is in vessels/{sigil}/rbrv.env
if test -n "${BASH_SOURCE[1]:-}"; then
    ZRBRV_ENV_PATH="${BASH_SOURCE[1]}"
    ZRBRV_DIR_NAME="${ZRBRV_ENV_PATH%/*}"
    ZRBRV_DIR_NAME="${ZRBRV_DIR_NAME##*/}"
    
    if test "${ZRBRV_DIR_NAME}" != "${RBRV_SIGIL}"; then
        bcu_die "RBRV_SIGIL '${RBRV_SIGIL}' must match vessel directory name '${ZRBRV_DIR_NAME}'"
    fi
fi

# eof

