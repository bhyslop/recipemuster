#!/bin/bash
#
# Copyright 2024 Scale Invariant, Inc.
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
# Purpose: interface Tabtargets to the console makefile.  Any build
# specific logging could be done in a customized version of this file.
#

set -euo pipefail

# First parameter is the number of jobs we'll let MAKE use. 1 is default non-parallel
JOBS=$1
shift

# Second parameter is the rule to run, typically quite related to tabtarget invoking it
TABTARGET_BASENAME=$1
shift

# All the rest of args are passed to make verbatim
ARGS="$@"

# Determine output synchronization
case "$JOBS" in
    1) OUTPUT_SYNC="-Oline"    ;;
    *) OUTPUT_SYNC="-Orecurse" ;;
esac

# Split $EXE by '.' and store in an array
IFS='.' read -ra EXE_PARTS <<< "$TABTARGET_BASENAME"

# Invoke make.  The tabtarget name
make -f rbc-console.mk                             \
    "$OUTPUT_SYNC" -j "$JOBS"                      \
    "$TABTARGET_BASENAME"                          \
    ${EXE_PARTS[0]:+RMC_PARAM_1="${EXE_PARTS[0]}"} \
    ${EXE_PARTS[1]:+RMC_PARAM_2="${EXE_PARTS[1]}"} \
    ${EXE_PARTS[2]:+RMC_PARAM_3="${EXE_PARTS[2]}"} \
    ${EXE_PARTS[3]:+RMC_PARAM_4="${EXE_PARTS[3]}"} \
    $ARGS

