// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! What a generated track renders through the handbook module: a control per
//! motif, and the line that ends it.
//!
//! EVERY PLANTED WORD IS MADE UP. The track, the keys, the values and the text a
//! control carries are words no tree governs, so nothing here reads as a name
//! and no assertion can be satisfied by a word the module happens to print for
//! a reason of its own.

#![deny(warnings)]

use but::buth_hurdle::buth_Bench;
use but::buth_hurdle::BUTH_DIASTEMA;

/// The modules a track renders through: the yelp module the handbook resolves
/// through, the command module its refusals die through, and the handbook itself.
const BUTI_MODULES: &[&str] = &[
    "buym_yelp.sh",
    "buc_command.sh",
    "buh_handbook.sh", //
];

/// Render one control and hold the line to its label.
///
/// THE LABEL AND THE TEXT ARE ASSERTED AS ONE NEEDLE, because a control is the
/// label standing at the head of the text it governs: a label printed on a line
/// of its own, or the text printed without one, would satisfy either half alone.
fn zbuti_control_prints(seat: &str, renderer: &str, label: &str) {
    buth_Bench::buth_seat(
        seat,
        BUTI_MODULES,
        &format!(
            "{} \"brambet quillow\"\n",
            renderer, //
        ),
    )
    .buth_drive(&[])
    .buth_thrived()
    .buth_carries(&format!(
        "{}: brambet quillow",
        label, //
    ))
    .buth_lacks(BUTH_DIASTEMA);
}

#[test]
fn buti_the_fatal_control_prints_its_label() {
    zbuti_control_prints("render-fatal", "buh_control_fatal", "STOP");
}

#[test]
fn buti_the_warn_control_prints_its_label() {
    zbuti_control_prints("render-warn", "buh_control_warn", "CAUTION");
}

#[test]
fn buti_the_store_control_prints_its_label() {
    zbuti_control_prints("render-store", "buh_control_store", "RECORD");
}

#[test]
fn buti_the_require_control_prints_its_label() {
    zbuti_control_prints("render-require", "buh_control_require", "VERIFY");
}

#[test]
fn buti_the_show_control_prints_its_label() {
    zbuti_control_prints("render-show", "buh_control_show", "NOTE");
}

#[test]
fn buti_the_ending_closes_the_track() {
    buth_Bench::buth_seat(
        "render-ending",
        BUTI_MODULES,
        "buh_ending \"trumble zorrel\"\n",
    )
    .buth_drive(&[])
    .buth_thrived()
    .buth_carries("[*]")
    .buth_carries("Done: trumble zorrel")
    .buth_lacks(BUTH_DIASTEMA);
}

// eof
