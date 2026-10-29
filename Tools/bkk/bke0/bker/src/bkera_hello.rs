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

//! The hello exemplar's app: it greets, naming this crate and the binary it
//! runs as, and answers the kennel's `--exergue` with the position it was
//! struck from.
//!
//! This is the crate's declared output module. The greeting and the position
//! are the crate's whole output, so this file is its one throat.

// Output discipline: all emission via bkera_main — no direct println!/eprintln! elsewhere

use std::process::ExitCode;

/// The flag the kennel's election asks a standing binary before it launches one.
/// Its home is `BKCE_EXERGUE_FLAG` in the kennel's election; it is spelled here
/// because this crate links nothing.
const BKERA_FLAG_EXERGUE: &str = "--exergue";

/// The position this binary was struck from: what the kennel's converge stated
/// to the build script, which handed it to this compile.
pub fn bkera_position() -> &'static str {
    env!("BKCH_COLLAR_POSITION")
}

/// Compose the greeting: this crate's name and version, and the path of the
/// running binary. Errs, naming why, only where that path does not resolve.
pub fn bkera_greeting() -> Result<String, String> {
    let here = std::env::current_exe()
        .map_err(|err| format!("the running binary's path does not resolve: {err}"))?;

    Ok(format!(
        "hello from rust, {} {} at {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        here.display(), //
    ))
}

/// Answer the kennel's `--exergue` with the position alone, or print the
/// greeting to stdout and answer success, or print why it could not be composed
/// to stderr and answer failure.
///
/// THE POSITION IS ANSWERED AHEAD OF EVERYTHING ELSE, because the kennel asks it
/// of a binary it has not yet decided to run: a reader app that cannot say what
/// it was struck from is never launched.
pub fn bkera_main() -> ExitCode {
    if std::env::args().nth(1).as_deref() == Some(BKERA_FLAG_EXERGUE) {
        println!(
            "{}",
            bkera_position(), //
        );
        return ExitCode::SUCCESS;
    }

    match bkera_greeting() {
        Ok(greeting) => {
            println!("{greeting}");
            ExitCode::SUCCESS
        }
        Err(why) => {
            eprintln!("bker: {why}");
            ExitCode::FAILURE
        }
    }
}

// eof
