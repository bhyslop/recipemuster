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

//! `bkerc` — the hello exemplar's attended app: a binary that speaks the
//! kennel's own tongue, so the launch door runs it as a captured child and
//! reads its cases as it reads a suite's.
//!
//! It answers the kennel's `--exergue` with the position it was struck from, as
//! the greeting binary beside it does, and run any other way it evaluates its
//! cases and prints one bare line apiece: a verdict word, one space, and the
//! case's name. One case is skipped on purpose, so the skip is shown named in the
//! door's verdict and never counted as run.
//!
//! This file is the binary's whole output, and `main` is its one throat.

// Output discipline: all emission via main — no direct println!/eprintln! elsewhere

#![deny(warnings)]

use std::process::ExitCode;

/// The flag the kennel's election asks a standing binary before it launches one.
/// Its home is `BKCE_EXERGUE_FLAG` in the kennel's election; it is spelled here
/// because this crate links nothing.
const BKERC_FLAG_EXERGUE: &str = "--exergue";

/// The position this binary was struck from: what the kennel's converge stated
/// to the build script, which handed it to this compile.
fn bkerc_position() -> &'static str {
    env!("BKCH_COLLAR_POSITION")
}

/// The cases this binary speaks, each as the verdict word and the case's name.
///
/// THE FIRST IS EVALUATED, and passes or fails on what it reads: the compiled
/// position is an object name. THE SECOND IS SKIPPED ON PURPOSE, so the door's
/// verdict has a skip to name; it evaluates nothing.
fn bkerc_cases() -> Vec<(&'static str, &'static str)> {
    let position = bkerc_position();
    let named = position.len() == 40 && position.chars().all(|one| one.is_ascii_hexdigit());

    vec![
        (
            if named { "pass" } else { "fail" },
            "the compiled position is forty hex characters",
        ),
        (
            "skip",
            "a case skipped on purpose to show the skip in the verdict",
        ),
    ]
}

/// Answer the kennel's `--exergue` with the position alone, or print each case's
/// line to stdout and answer success where none failed and failure otherwise.
fn main() -> ExitCode {
    if std::env::args().nth(1).as_deref() == Some(BKERC_FLAG_EXERGUE) {
        println!(
            "{}",
            bkerc_position(), //
        );
        return ExitCode::SUCCESS;
    }

    let cases = bkerc_cases();
    for (word, name) in &cases {
        println!("{word} {name}");
    }

    if cases.iter().any(|(word, _)| *word == "fail") {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

// eof
