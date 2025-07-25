#!/bin/bash

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

set -e

ZRBRS_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBRS_SCRIPT_DIR}/bvu_BashValidationUtility.sh"

bvu_env_string      RBRS_PODMAN_ROOT_DIR         1     64
bvu_env_string      RBRS_VMIMAGE_CACHE_DIR       1     64
bvu_env_string      RBRS_VM_PLATFORM             1     64


# eof

