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
# TEMPLATE — copy to `«moorings»/buml_launchers/launcher.buw_workbench.sh` in
# your own repository and make it executable. Then delete this whole header:
# every line after the first, through the blank line that ends it. What remains
# is four lines, byte for byte what the bash utility kit's launcher door writes
# for its own workbench, which is why this template carries no substitution.
#
# This is the bootstrap seed's launcher stub. No door runs before a stub and a
# tabtarget exist to reach it, so this one is placed by hand; every later stub
# is written by that door.

# Launcher stub - delegates to buw workbench
source "Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/buk/buw_workbench.sh" "$@"
