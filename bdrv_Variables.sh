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

##########################################
# Bash Dispatch Regime Variables File
#
# This is a reference variables file for the BDU dispatch system.
# The dispatch script uses this to set up the environment.
#
# This file is sourced by bash. No spaces are allowed around `=`.
# Only declare variables here. If you define any variables in terms
# of others, use ${xxx} to expand.

BDRV_STATION_FILE=../station-files/BDS.STATION.sh

BDRV_TABTARGET_DIR=tt

BDRV_TABTARGET_DELIMITER=.

BDRV_TOOLS_DIR=Tools

BDRV_COORDINATOR_SCRIPT=${BDRV_TOOLS_DIR}/rbk_Coordinator.sh

BDRV_TEMP_ROOT_DIR=../temp-bdu

BDRV_LOG_LAST=last

BDRV_LOG_EXT=txt

