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

//! Hurdles over the kennel's report of its own making (`bkcs_stamp`).
//!
//! Every fact here was fixed when the binary was struck, so these assert the
//! SHAPE of the reading rather than any particular value: a hurdle asserting
//! today's sha would have to be edited on every landing, which is a test that
//! reports the calendar rather than the code.

use crate::bkcs_stamp;

#[test]
fn bkts_the_position_carries_every_fact() {
    let position = bkcs_stamp::bkcs_position();

    assert_eq!(position.seat.len(), 40, "the position is a resolved sha");
    assert!(!position.pin.is_empty(), "the pin names a channel");
    assert!(!position.compiler.is_empty(), "the compiler names itself");
}

#[test]
fn bkts_the_compiler_answers_the_pin_it_was_asked_for() {
    let position = bkcs_stamp::bkcs_position();

    // The request and the answer, read together. This is what makes a bumped pin
    // provable rather than assumed: the pin is what was asked, the compiler is
    // what ran, and a build where they disagree is exactly the silently-inert
    // bump the leash exists to close.
    assert!(
        position.compiler.contains(position.pin),
        "the rustc that ran ({}) is the channel the pin named ({})",
        position.compiler,
        position.pin
    );
}

#[test]
fn bkts_the_render_names_each_fact() {
    let said = bkcs_stamp::bkcs_position().bkcs_render();

    for expected in ["seat", "pin", "compiler"] {
        assert!(said.contains(expected), "the report names {}: {}", expected, said);
    }
}

// eof
