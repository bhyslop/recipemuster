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
# Co-maintained with its sibling in the other dialect, Tools/bkk/bk0/src/bkg_breviary.rs - do not hand-edit.
# The human-presentation surface forms of the governed words homed in BKK.
# A string that spells one of these words in its own bytes has left the home.
# Beside them, the colophons and exit codes attached at definitions, each named for the definition it is attached at.

test -z "${ZBKV_SOURCED:-}" || return 0
ZBKV_SOURCED=1

readonly BKV_BAKED_BASE="baked"
readonly BKV_BAKED_BASE_VERSAL="BAKED"
readonly BKV_BAKED_BASE_LETTRINE="Baked"
readonly BKV_COURSE_RECORD_BASE="course record"
readonly BKV_COURSE_RECORD_BASE_VERSAL="COURSE RECORD"
readonly BKV_COURSE_RECORD_BASE_LETTRINE="Course record"
readonly BKV_COURSE_RECORD_PLURAL="course records"
readonly BKV_COURSE_RECORD_PLURAL_VERSAL="COURSE RECORDS"
readonly BKV_COURSE_RECORD_PLURAL_LETTRINE="Course records"
readonly BKV_COURSE_RECORD_POSSESSIVE="course record's"
readonly BKV_COURSE_RECORD_POSSESSIVE_VERSAL="COURSE RECORD'S"
readonly BKV_COURSE_RECORD_POSSESSIVE_LETTRINE="Course record's"
readonly BKV_CRUDE_BASE="crude"
readonly BKV_CRUDE_BASE_VERSAL="CRUDE"
readonly BKV_CRUDE_BASE_LETTRINE="Crude"
readonly BKV_LYAM_BASE="lyam"
readonly BKV_LYAM_BASE_VERSAL="LYAM"
readonly BKV_LYAM_BASE_LETTRINE="Lyam"
readonly BKV_LYAM_PLURAL="lyams"
readonly BKV_LYAM_PLURAL_VERSAL="LYAMS"
readonly BKV_LYAM_PLURAL_LETTRINE="Lyams"
readonly BKV_LYAM_POSSESSIVE="lyam's"
readonly BKV_LYAM_POSSESSIVE_VERSAL="LYAM'S"
readonly BKV_LYAM_POSSESSIVE_LETTRINE="Lyam's"
readonly BKV_BLEACH_BASE="bleach"
readonly BKV_BLEACH_BASE_VERSAL="BLEACH"
readonly BKV_BLEACH_BASE_LETTRINE="Bleach"
readonly BKV_BLEACH_THIRD_SINGULAR="bleaches"
readonly BKV_BLEACH_THIRD_SINGULAR_VERSAL="BLEACHES"
readonly BKV_BLEACH_THIRD_SINGULAR_LETTRINE="Bleaches"
readonly BKV_BLEACH_PAST="bleached"
readonly BKV_BLEACH_PAST_VERSAL="BLEACHED"
readonly BKV_BLEACH_PAST_LETTRINE="Bleached"
readonly BKV_BLEACH_PROGRESSIVE="bleaching"
readonly BKV_BLEACH_PROGRESSIVE_VERSAL="BLEACHING"
readonly BKV_BLEACH_PROGRESSIVE_LETTRINE="Bleaching"
readonly BKV_BLEACH_PLURAL="bleaches"
readonly BKV_BLEACH_PLURAL_VERSAL="BLEACHES"
readonly BKV_BLEACH_PLURAL_LETTRINE="Bleaches"
readonly BKV_BLEACH_POSSESSIVE="bleach's"
readonly BKV_BLEACH_POSSESSIVE_VERSAL="BLEACH'S"
readonly BKV_BLEACH_POSSESSIVE_LETTRINE="Bleach's"
readonly BKV_BLEACH_NOMINALIZATION="bleaching"
readonly BKV_BLEACH_NOMINALIZATION_VERSAL="BLEACHING"
readonly BKV_BLEACH_NOMINALIZATION_LETTRINE="Bleaching"
readonly BKV_COURSE_BASE="course"
readonly BKV_COURSE_BASE_VERSAL="COURSE"
readonly BKV_COURSE_BASE_LETTRINE="Course"
readonly BKV_COURSE_PLURAL="courses"
readonly BKV_COURSE_PLURAL_VERSAL="COURSES"
readonly BKV_COURSE_PLURAL_LETTRINE="Courses"
readonly BKV_COURSE_POSSESSIVE="course's"
readonly BKV_COURSE_POSSESSIVE_VERSAL="COURSE'S"
readonly BKV_COURSE_POSSESSIVE_LETTRINE="Course's"
readonly BKV_FUTURITY_BASE="futurity"
readonly BKV_FUTURITY_BASE_VERSAL="FUTURITY"
readonly BKV_FUTURITY_BASE_LETTRINE="Futurity"
readonly BKV_FUTURITY_PLURAL="futurities"
readonly BKV_FUTURITY_PLURAL_VERSAL="FUTURITIES"
readonly BKV_FUTURITY_PLURAL_LETTRINE="Futurities"
readonly BKV_FUTURITY_POSSESSIVE="futurity's"
readonly BKV_FUTURITY_POSSESSIVE_VERSAL="FUTURITY'S"
readonly BKV_FUTURITY_POSSESSIVE_LETTRINE="Futurity's"
readonly BKV_UNHITCHED_BASE="unhitched"
readonly BKV_UNHITCHED_BASE_VERSAL="UNHITCHED"
readonly BKV_UNHITCHED_BASE_LETTRINE="Unhitched"
readonly BKV_BKCK_SUITE_PEDIMENT="bkw-k"
readonly BKV_BKCW_BLEACH_PEDIMENT="bkw-B"
readonly BKV_BKCW_KENNEL_PEDIMENT="bkw-w"
readonly BKV_BKCW_MUZZLE_PEDIMENT="bkw-z"
readonly BKV_BKCW_REBED_PEDIMENT="bkw-r"
readonly BKV_DELOUSE_PEDIMENT="bkw-n"
readonly BKV_DERBY_PEDIMENT="bkw-d"
readonly BKV_FUTURITY_PEDIMENT="bkw-f"
readonly BKV_GANGLINE_PEDIMENT="bkw-g"
readonly BKV_HEEL_PEDIMENT="bkw-h"
readonly BKV_LINEUP_PEDIMENT="bkw-l"
readonly BKV_MUSH_PEDIMENT="bkw-m"
readonly BKV_SCOOP_PEDIMENT="bkw-s"
readonly BKV_TATTOO_PEDIMENT="bkw-t"

# eof
