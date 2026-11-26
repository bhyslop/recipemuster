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
  buz_enroll RBVZ_RBVQ_RENDER   "rbvw-qr" "rbvq_cli.sh" "rbvq_render"   "param1" "Displays every Qualification field as the file holds it, before any check, so a broken value stands visible beside the rest."
  buz_enroll RBVZ_RBVQ_VALIDATE "rbvw-qv" "rbvq_cli.sh" "rbvq_validate" "param1" "Checks every Qualification field's shape, and each field that fills a clone's field where that field is defined, and fails naming the field that does not hold."
  buz_enroll RBVZ_RBVQ_BLOCK    "rbvw-qb" "rbvq_cli.sh" "rbvq_block"    "param1" "Checks the Qualification fields, then prints the block pasted to a walking session: each value beside its plain name, the try's joined values among them; or nothing but the refusal."
  buz_enroll RBVZ_RBVQ_ADVANCE  "rbvw-qa" "rbvq_cli.sh" "rbvq_advance"  "param1" "Adds one to the try index and writes the file, refusing at 9999."
  buz_enroll RBVZ_RBVQ_LAY      "rbvw-ql" "rbvq_cli.sh" "rbvq_lay"      "param1" "Writes a first Qualification file, its index reading 1000 and its other fields blank, and never writes over a file standing."

  # The guide to the browser profile the windowed walks open on; it takes no argument.
  buz_enroll RBVZ_RBVP_CATECHESIS "rbvw-m" "rbvp_cli.sh" "rbvp_catechesis" "" "Prints the guide to the Chrome profile the windowed walks open on: making it or renaming it, its Claude extension, and its sign-ins in their order."

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
