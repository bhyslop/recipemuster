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

ZRBRE_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
source "${ZRBRE_SCRIPT_DIR}/buv_BashValidationUtility.sh"

# RBRE-specific environment variables
buv_env_string      RBRE_AWS_CREDENTIALS_ENV     1    255
buv_env_string      RBRE_AWS_ACCESS_KEY_ID      20     20
buv_env_string      RBRE_AWS_SECRET_ACCESS_KEY  40     40
buv_env_string      RBRE_AWS_ACCOUNT_ID         12     12
buv_env_string      RBRE_AWS_REGION              1     32
buv_env_xname       RBRE_REPOSITORY_NAME         2     64

# eof

