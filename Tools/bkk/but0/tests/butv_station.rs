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

//! A station file names its stores relative to the project root.
//!
//! THE DOOR IS THE LAUNCHER'S, AND THESE HURDLES DRIVE IT WHOLE. The station
//! regime is judged by the launcher every dispatch passes through, ahead of any
//! coordinator, so each hurdle rewrites the composed seat's station file and
//! drives its door: a refusal is the launcher's, and the coordinator's own line
//! standing in the output says the launcher let the station through.
//!
//! THE REFUSAL NAMES THE VARIABLE AND THE RELATIVE FORM, and both are asserted:
//! a refusal naming neither tells the operator nothing about which line to
//! repair or what to write there instead.
//!
//! THE ABSOLUTE STORE IS ASSERTED ABSENT AFTER THE REFUSAL, since the property
//! these hurdles hold is that a door refusing a path makes nothing at it.
//!
//! THE RELATIVE TACKROOM IS THE POSITIVE CONTROL, AND THE OMITTED ONE IS THE
//! REFUSAL. A relative value passing says the check reads the form and not the
//! field's presence; a station file omitting the field is refused naming it,
//! since the field is required as the others are.

#![deny(warnings)]

use std::path::Path;

use but::buth_hurdle::buth_Bench;
use but::buth_hurdle::buth_Said;

/// The station file the composed seat's regime names, under its root.
const BUTV_STATION: &str = ".buk/burs.env";

/// What the coordinator says once the launcher has let the station through.
const BUTV_ADMITTED: &str = "butv station admitted";

/// The station file's fields other than the two a hurdle varies.
const BUTV_STEADY: &str = "BURS_USER=kennel\nBURS_TINCTURE=k\n";

/// Compose a seat, write its station file, and drive its door.
///
/// THE STATION'S PATH LINES ARE THE HURDLE'S, composed against the seat's own
/// root, so an absolute value a hurdle plants names ground the seat owns. The
/// bench is handed back because the seat stands only while it does, and an
/// absence asserted after the seat has dropped would assert nothing.
fn butv_station(name: &str, paths: impl Fn(&Path) -> String) -> (buth_Bench, buth_Said) {
    let bench = buth_Bench::buth_seat(
        name,
        &[],
        &format!(
            "echo \"{}\"",
            BUTV_ADMITTED, //
        ),
    );
    let station = bench.buth_root().join(BUTV_STATION);
    std::fs::write(
        &station,
        format!(
            "{}{}",
            paths(bench.buth_root()),
            BUTV_STEADY, //
        ),
    )
    .unwrap_or_else(|err| {
        panic!(
            "could not write {}: {}",
            station.display(),
            err, //
        )
    });
    let said = bench.buth_drive(&[]);
    (bench, said)
}

#[test]
fn butv_an_absolute_tackroom_is_refused_naming_the_relative_form() {
    let (bench, said) = butv_station("substrate-station-absolute-tackroom", |root| {
        format!(
            "BURS_LOG_DIR=logs\nBURS_TACKROOM={}\n",
            root.join("absolute-tackroom").display(), //
        )
    });
    said.buth_died()
        .buth_carries("BURS_TACKROOM is absolute")
        .buth_carries("BURS_TACKROOM=../tackroom-buk")
        .buth_lacks(BUTV_ADMITTED);
    assert!(
        !bench.buth_root().join("absolute-tackroom").exists(),
        "the refused tackroom under {} was made",
        bench.buth_root().display(), //
    );
}

#[test]
fn butv_an_absolute_log_dir_is_refused_naming_the_relative_form() {
    let (bench, said) = butv_station("substrate-station-absolute-log-dir", |root| {
        format!(
            "BURS_LOG_DIR={}\nBURS_TACKROOM=../tackroom-buk\n",
            root.join("absolute-logs").display(), //
        )
    });
    said.buth_died()
        .buth_carries("BURS_LOG_DIR is absolute")
        .buth_carries("BURS_LOG_DIR=../logs-buk")
        .buth_lacks(BUTV_ADMITTED);
    assert!(
        !bench.buth_root().join("absolute-logs").exists(),
        "the refused log directory under {} was made",
        bench.buth_root().display(), //
    );
}

#[test]
fn butv_a_relative_tackroom_is_admitted() {
    let (_bench, said) = butv_station("substrate-station-relative-tackroom", |_| {
        "BURS_LOG_DIR=logs\nBURS_TACKROOM=../tackroom-buk\n".to_string()
    });
    said.buth_thrived().buth_carries(BUTV_ADMITTED);
}

/// The clean environment is the lure's own strip, which removes every `BURS_`
/// and `BURV_` variable from a seat's dispatch, so no inherited value fills the
/// omitted field.
#[test]
fn butv_an_omitted_tackroom_is_refused_naming_it() {
    let (_bench, said) = butv_station("substrate-station-omitted-tackroom", |_| {
        "BURS_LOG_DIR=logs\n".to_string()
    });
    said.buth_died()
        .buth_carries("BURS_TACKROOM is not set")
        .buth_lacks(BUTV_ADMITTED);
}
// eof
