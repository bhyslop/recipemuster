#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
# TEMPLATE — copy to `tt/buw-tt-cbl.CreateTabTargetBatchLogging.sh` in your own
# repository and make it executable. Then delete this whole header: every line
# after the first, through the blank line that ends it. What remains is three
# lines, byte for byte what the bash utility kit's batch-and-logging tabtarget
# door writes for the same name, which is why this template carries no
# substitution.
#
# This is the bootstrap seed's tabtarget, and the one door it reaches writes
# every other tabtarget, its own siblings included.

set -- 'launcher.buw_workbench.sh' "$@"
exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" "${0##*/}" "${@}"
