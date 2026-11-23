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
  buz_enroll RBVZ_ESSAI "rbvw-e" "rbve_cli.sh" "rbve_essai" "imprint" "Essai — the reversible repair cohobation (gate, cut, prove, temenos; zero remote acts; the world is the imprint)"
  buz_enroll RBVZ_DOCIMASY "rbvw-d" "rbve_cli.sh" "rbve_docimasy" "imprint" "Docimasy — the reveal's reversible proving act (grants the cachet; the world is the imprint)"
  buz_enroll RBVZ_OSTEND "rbvw-o" "rbve_cli.sh" "rbve_ostend" "imprint" "Ostend — the reveal's irreversible showing (the world is the imprint)"
  buz_enroll RBVZ_HARBINGER "rbvw-h" "rbve_cli.sh" "rbve_harbinger" "imprint" "Harbinger — the introit against promoted public main (zero remote acts; the world is the imprint)"
  buz_enroll RBVZ_ENCAMP "rbvw-p" "rbve_cli.sh" "rbve_encamp" "imprint" "Practice seat — encamp a practice seat beneath an absolute practice home (a chore beside the ceremony; the world is the imprint)"

  # The Qualification Regime's doors: each takes the values file as its one
  # optional argument, rbrq.env beside the station file when given none.
  buz_enroll RBVZ_RBRQ_RENDER   "rbvw-qr" "rbrq_cli.sh" "rbrq_render"   "param1" "Render the Qualification Regime — every field as held, before any check"
  buz_enroll RBVZ_RBRQ_VALIDATE "rbvw-qv" "rbrq_cli.sh" "rbrq_validate" "param1" "Validate the Qualification Regime — shapes, and each filled field by its own regime"
  buz_enroll RBVZ_RBRQ_BLOCK    "rbvw-qb" "rbrq_cli.sh" "rbrq_block"    "param1" "Print the Qualification block a paredros pastes to a walking session, after validating"
  buz_enroll RBVZ_RBRQ_ADVANCE  "rbvw-qa" "rbrq_cli.sh" "rbrq_advance"  "param1" "Advance the Qualification Regime's try index by one, refusing at 9999"
  buz_enroll RBVZ_RBRQ_LAY      "rbvw-ql" "rbrq_cli.sh" "rbrq_lay"      "param1" "Lay a first Qualification Regime file, never over one standing"

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
