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
# RBTCQA - Qualification test cases for RBTB testbench

set -euo pipefail

######################################################################
# Qualification cases

rbtcqa_qualify_all_tcase() {
  buto_trace "Qualify-all: running full qualification via rbq"
  buto_unit_expect_ok rbq_qualify_all
}

# eof
