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
# Recipe Bottle Manual Procedures - Command Line Interface

set -euo pipefail

ZRBMP_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source all dependencies
source "${ZRBMP_SCRIPT_DIR}/bcu_BashCommandUtility.sh"
source "${ZRBMP_SCRIPT_DIR}/rbl_Locator.sh"
source "${ZRBMP_SCRIPT_DIR}/rbmp_ManualProcedures.sh"

zrbmp_furnish() {

  zrbl_kindle
  zrbmp_kindle
}

bcu_execute rbmp_ "Manual Setup Procedures" zrbmp_furnish "$@"


# eof