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

//! The starter templates the engine embeds carry no carriage return.
//!
//! The bytes are taken by `include_str!` from the same files the engine
//! embeds, so they are the bytes cargo embeds from this checkout.

const BPET_BURC: &str = include_str!("../../../bpk/bpt_burc.env");
const BPET_LAUNCHER: &str = include_str!("../../../bpk/bpt_launcher.sh");
const BPET_STATION: &str = include_str!("../../../bpk/bpt_station.sh");

fn bpet_is_lf_text(text: &str) -> bool {
    !text.is_empty() && text.contains('\n') && !text.contains('\r')
}

#[test]
fn bpet_control_carriage_return_fails_predicate() {
    assert!(!bpet_is_lf_text("a\r\nb"));
    assert!(bpet_is_lf_text("a\nb"));
}

#[test]
fn bpet_burc_template_is_lf_text() {
    assert!(bpet_is_lf_text(BPET_BURC));
}

#[test]
fn bpet_launcher_template_is_lf_text() {
    assert!(bpet_is_lf_text(BPET_LAUNCHER));
}

#[test]
fn bpet_station_template_is_lf_text() {
    assert!(bpet_is_lf_text(BPET_STATION));
}
