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
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# BKZ Zipper - Colophon registry for the Bash Kennel Master's workbench
#
# ONE TABTARGET PER DOOR, THE COLLAR ITS ARGUMENT (operator, 260905). Every door
# of the frozen command surface is enrolled here, whether or not its interior
# stands, and a door still being built is refused by the kennel BY NAME rather
# than by being absent from this registry: an absent colophon is a tab
# completion that finds nothing, which reads as a kit that never had the door.
#
# THE REGISTRY IS WHERE THE COMMAND SURFACE IS DISCOVERABLE, since every door
# wears one tabtarget and the operator face of the whole surface is the listing
# of `tt/` (BKSNC-Kennelcraft.adoc "The doors"). So each description says
# what its door DOES, in a line, and the roster below is complete on purpose.
#
# TWO ENROLLMENTS ARE NOT DOORS of the collar register: the kennelman, which
# runs this kit's hurdles outside the kennel, and the whistle, which is the bash
# bootstrap the envelope names interior. They stand at the foot of the list for
# that reason.

set -euo pipefail

# Multiple inclusion guard
test -z "${ZBKZ_SOURCED:-}" || return 0
ZBKZ_SOURCED=1

######################################################################
# Colophon registry initialization

zbkz_kindle() {
  test -z "${ZBKZ_KINDLED:-}" || buc_die_now "bkz already kindled"

  # Verify buz zipper is kindled (workbench kindles buz first)
  zbuz_sentinel

  local z_mod="bkcw_cli.sh"
  buz_group BKZ__GROUP_KENNEL "bkw-" "The Bash Kennel Master: the launch discipline every build and test door passes through"

  # THE SIX DOORS OF THE FREEZE, in the order the envelope's own table lists
  # them (BKSNC-Kennelcraft.adoc "The doors"). Three the operator types most
  # days - mush, derby, heel - and their first letters stand apart on purpose;
  # three an agent reaches for when the work calls - lineup, tattoo, muzzle.
  #
  # EVERY COLOPHON HERE IS ON param1, INCLUDING THE DOORS THAT TAKE NOTHING, and
  # the uniformity is deliberate. The empty channel does not refuse a stray
  # argument - it WARNS and drops it, and the command then runs as though nothing
  # was typed. For `scoop` that would mean an operator who named one collar
  # watching the whole yard go; for `derby`, a whole-kennel run they did not ask
  # for. On param1 the argument reaches the door, and the door's own law refuses
  # it by name, which is where a grammar belongs: the kennel states what each
  # door takes, and this registry states only how the argument travels.
  buz_enroll BKZ_MUSH      "bkw-m" "${z_mod}" "bkcw_mush"     "param1" "Launch what one collar names: a suite runs under the runner the collar declares, an app runs the binary the election names with its arguments passed through verbatim"
  buz_enroll BKZ_DERBY     "bkw-d" "${z_mod}" "bkcw_derby"    "param1" "Run every suite collar the discovery walk finds, sequentially - one verdict line per collar and one for the set, refusing the whole walk when any collar in it is invalid"
  buz_enroll BKZ_LINEUP    "bkw-l" "${z_mod}" "bkcw_lineup"   "param1" "List a suite collar's hurdles as its runner names them, in the door's own voice, running nothing"
  buz_enroll BKZ_TATTOO    "bkw-t" "${z_mod}" "bkcw_tattoo"   "param1" "Proclaim one collar, report every conformance finding and the election verdict, and change nothing on disk - the resolver's read-only face"
  buz_enroll BKZ_HEEL      "bkw-h" "${z_mod}" "bkcw_heel"     "param1" "Converge one collar: build its launchable current through the leash and install it at its residence, the act every other door refuses to perform"
  buz_enroll BKZ_MUZZLE    "bkw-z" "${z_mod}" "bkcw_muzzle"   "param1" "Lint every collar the walk finds at the muzzle directory it declares (BKSMZ-Muzzle.adoc), each bar naming the surface to adopt"

  # TWO DOORS SLATED AFTER THE FREEZE, on the operator's election in each case.
  # They are doors of the collar register like the six above and differ only in
  # when they were elected.
  buz_enroll BKZ_GANGLINE  "bkw-g" "${z_mod}" "bkcw_gangline" "param1" "Re-derive one collar's lock: the single act in which the leash lifts the lock flag, writing the lock and leaving it dirty for whoever commits it"
  buz_enroll BKZ_SCOOP     "bkw-s" "${z_mod}" "bkcw_scoop"    "param1" "Remove every collar's target directory, on the operator's word alone - no stamp, no cadence and no record left behind"

  # THE KENNELMAN, the one door that stands outside the kennel. The kennel's own
  # suite is a suite collar like any crate's and one day the kennel will launch
  # it, but a kennel that is broken cannot launch the hurdles that would say so -
  # so this runs them through the fence directly, with no kennel door in its
  # path. It is the founding pace's test door and it STAYS. The letter is k, its
  # word's own; the operator kept the word, obscure being admitted for a rare
  # door.
  buz_enroll BKZ_KENNELMAN "bkw-k" "bkck_cli.sh" "bkck_suite" "param1" "The kennelman (BKSLR-Lure.adoc): run this kit's hurdles - the reader's, the kennel's and the substrate's - outside the kennel, through the toolchain fence directly, so a broken kennel can still be told it is broken. Honors the door law and states the pin and the lock as every door does. The only argument is an optional filter, handed to the runner's own test-name filter so one hurdle or one module can be driven alone"

  # THE WHISTLE IS THE ONE DOOR THAT CONVERGES UNASKED, and it is the only one
  # that ever will. Every door the kennel exposes reports and refuses; this one
  # rebuilds, because a launcher that execs a stale binary has lied about
  # everything downstream of it and no report it could print would help. Every
  # door above reaches the kennel THROUGH it, so the converge happens once per
  # invocation wherever the operator entered.
  #
  # BARE NOW. Its door-word argument mode retired with the one-tabtarget-per-door
  # ruling: what it takes is nothing at all, and what it answers is the kennel's
  # own making. A word typed at it anyway is REFUSED and pointed at the door's own
  # tabtarget, rather than dropped - the mode worked yesterday, so an operator
  # reaching for it deserves the forwarding address and not a silence. The letter
  # is w, its word's own.
  buz_enroll BKZ_WHISTLE   "bkw-w" "${z_mod}" "bkcw_kennel"   "param1" "The whistle: refuse an uncommitted repository, stand on the toolchain fence, and rebuild the kennel binary where the seat has outrun it - then report the kennel's own making, the position it was built at, the channel its pin asked for and the compiler that answered"

  readonly ZBKZ_KINDLED=1
}

######################################################################
# Healthcheck (validates all enrolled tabtargets exist on disk)

zbkz_healthcheck() {
  zbkz_sentinel
  buz_healthcheck
}

######################################################################
# Internal sentinel

zbkz_sentinel() {
  test "${ZBKZ_KINDLED:-}" = "1" || buc_die_now "Module bkz not kindled - call zbkz_kindle first"
}

# eof
