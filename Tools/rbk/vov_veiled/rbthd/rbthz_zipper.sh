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
# RBTHZ Zipper - the hierophant's OWN colophon registry (veiled). Its colophons
# (rbthw-) live here, invisible to the shipped rbz zipper's completeness check —
# which is how a whole veiled tool stays off every shipped manifest.

set -euo pipefail

# Multiple inclusion guard
test -z "${ZRBTHZ_SOURCED:-}" || return 0
ZRBTHZ_SOURCED=1

######################################################################
# Colophon registry initialization

zrbthz_kindle() {
  test -z "${ZRBTHZ_KINDLED:-}" || buc_die "rbthz already kindled"

  zbuz_sentinel

  buz_group RBTHZ__GROUP_CEREMONY "rbthw-" "Hierophant — delivery-ceremony conductor (veiled)"
  buz_enroll RBTHZ_BUILD "rbthw-b" "rbthe_cli.sh" "rbthe_build" "" "Build the hierophant crate"
  buz_enroll RBTHZ_TEST  "rbthw-t" "rbthe_cli.sh" "rbthe_test"  "" "Run the hierophant crate tests — the cut's self-proofs"
  buz_enroll RBTHZ_ESSAI "rbthw-e" "rbthe_cli.sh" "rbthe_essai" "" "Essai — the reversible repair lap (gate, cut, prove, rig; zero remote acts)"
  buz_enroll RBTHZ_DOCIMASY "rbthw-d" "rbthe_cli.sh" "rbthe_docimasy" "param1" "Docimasy — the reveal's reversible proving act (grants the cachet; optional 'rehearse' arg)"
  buz_enroll RBTHZ_OSTEND "rbthw-o" "rbthe_cli.sh" "rbthe_ostend" "param1" "Ostend — the reveal's irreversible showing (optional 'rehearse' arg)"
  buz_enroll RBTHZ_HARBINGER "rbthw-h" "rbthe_cli.sh" "rbthe_harbinger" "" "Harbinger — the stranger rig against promoted public main (zero remote acts)"

  readonly ZRBTHZ_KINDLED=1
}

######################################################################
# Healthcheck (validates all enrolled tabtargets exist on disk)

zrbthz_healthcheck() {
  zrbthz_sentinel
  buz_healthcheck
}

######################################################################
# Internal sentinel

zrbthz_sentinel() {
  test "${ZRBTHZ_KINDLED:-}" = "1" || buc_die "Module rbthz not kindled - call zrbthz_kindle first"
}

# eof
