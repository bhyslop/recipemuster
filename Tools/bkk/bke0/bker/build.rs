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

//! The hello exemplar's build script: it compiles in the position the kennel's
//! converge states, so the binary can say what it was struck from when the
//! kennel asks it `--exergue` before a launch.
//!
//! THIS IS THE ONE OBLIGATION A READER APP OWES THE LAUNCH, and it needs no
//! generated file: the converge states the position into the build's
//! environment, this script hands it to the compile, and the app prints it back.
//!
//! A BUILD THAT CANNOT STATE THE POSITION DIES HERE. There is no "unknown" to
//! fall back on: a binary answering the kennel's question with text that means
//! nothing would be believed.

// EVERY DECLARATION STANDS INSIDE `main`, which is the naming law rather than a
// style: a file cargo names carries no prefix of its own. The kennel's own build
// script states the law at length.
fn main() {
    /// The variable the kennel's converge states into the build. Its home is the
    /// converge, `BKCH_COLLAR_POSITION_VAR` in `Tools/bkk/bk0/src/bkch_heel.rs`;
    /// it is spelled here because a build script links nothing. Cargo is told to
    /// watch it, so a commit touching the collar's roots rebuilds the binary
    /// rather than leaving it claiming a position it no longer holds.
    const ZBKER_SEAT_VAR: &str = "BKCH_COLLAR_POSITION";

    println!("cargo:rerun-if-env-changed={}", ZBKER_SEAT_VAR);

    let seat = match std::env::var(ZBKER_SEAT_VAR) {
        Ok(value) if !value.trim().is_empty() => value,
        _ => panic!(
            "{} is unset, so this build cannot state the position it was taken at. \
             The exemplar is built through the kennel's doors, which state it: \
             tt/bkw-h.Heel.sh bkern_app for the app, tt/bkw-m.Mush.sh bkern_suite for the suite",
            ZBKER_SEAT_VAR, //
        ),
    };

    // RE-EMITTED UNDER THE NAME IT ARRIVED UNDER, so `env!` reads what cargo set
    // for this compile and never whatever the ambient environment held.
    println!("cargo:rustc-env={}={}", ZBKER_SEAT_VAR, seat.trim());
}

// eof
