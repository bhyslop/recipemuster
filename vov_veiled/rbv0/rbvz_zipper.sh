#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
# All rights reserved.
# SPDX-License-Identifier: LicenseRef-Proprietary
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# RBVZ Zipper - the hierophant's OWN colophon registry (veiled). Its colophons
# (rbvw-) live here, invisible to the shipped rbz zipper's completeness check —
# which is how a whole veiled tool stays off every shipped manifest.

set -euo pipefail

# Multiple inclusion guard
test -z "${ZRBVZ_SOURCED:-}" || return 0
ZRBVZ_SOURCED=1

######################################################################
# Colophon registry initialization

zrbvz_kindle() {
  test -z "${ZRBVZ_KINDLED:-}" || buc_die_now "rbvz already kindled"

  zbuz_sentinel

  buz_group RBVZ__GROUP_CEREMONY "rbvw-" "Hierophant — delivery-ceremony conductor (veiled)"
  buz_enroll RBVZ_BUILD "rbvw-b" "rbve_cli.sh" "rbve_build" "" "Build the hierophant crate"
  buz_enroll RBVZ_TEST  "rbvw-t" "rbve_cli.sh" "rbve_test"  "" "Run the hierophant crate tests — the cut's self-proofs"
  buz_enroll RBVZ_ESSAI "rbvw-e" "rbve_cli.sh" "rbve_essai" "" "Essai — the reversible repair lap (gate, cut, prove, temenos; zero remote acts)"
  buz_enroll RBVZ_DOCIMASY "rbvw-d" "rbve_cli.sh" "rbve_docimasy" "param1" "Docimasy — the reveal's reversible proving act (grants the cachet; optional 'rehearse' arg)"
  buz_enroll RBVZ_OSTEND "rbvw-o" "rbve_cli.sh" "rbve_ostend" "param1" "Ostend — the reveal's irreversible showing (optional 'rehearse' arg)"
  buz_enroll RBVZ_HARBINGER "rbvw-h" "rbve_cli.sh" "rbve_harbinger" "" "Harbinger — the stranger walk against promoted public main (zero remote acts)"

  readonly ZRBVZ_KINDLED=1
}

######################################################################
# Healthcheck (validates all enrolled tabtargets exist on disk)

zrbvz_healthcheck() {
  zrbvz_sentinel
  buz_healthcheck
}

######################################################################
# Internal sentinel

zrbvz_sentinel() {
  test "${ZRBVZ_KINDLED:-}" = "1" || buc_die_now "Module rbvz not kindled - call zrbvz_kindle first"
}

# eof
