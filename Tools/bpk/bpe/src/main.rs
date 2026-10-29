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

//! `bpx` — the parcel kit's install binary.
//!
//! Every parcel carries it at its root, platform-suffixed, beside the two
//! bootstraps that exec it, so a parcel installs and uninstalls through the
//! engine it brought and never through a binary standing in its target. It
//! takes nothing from the forge that cut the parcel but what that forge wrote
//! into the parcel: the record, the kits, and the behest.
//!
//! Two verbs, each handed the extracted parcel and the target:
//!
//! - `emplace --parcel <dir> --burc <burc.env or seat directory>` installs.
//! - `vacate --parcel <dir> --burc <burc.env>` removes the Claude Code
//!   integration, leaving the kit directories standing.
//!
//! Run with `--exergue` as its first argument it prints the position the
//! kennel's converge struck it at, and nothing else.
//!
//! Output discipline: all emission via zbpe_out and zbpe_err — no direct
//! println!/eprintln! elsewhere. The library writes to no console; this file is
//! the one place its results are reported.

#![deny(warnings)]

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

/// The kennel's question, asked before every launch of a reader app.
const ZBPE_EXERGUE_FLAG: &str = "--exergue";

/// The position the kennel's converge states into the compile. A compile
/// standing outside the kennel's doors states none and stops here: a binary
/// answering the kennel with text that means nothing would be believed.
const ZBPE_POSITION: &str = env!(
    "BKCH_COLLAR_POSITION",
    "BKCH_COLLAR_POSITION is unset, so this build cannot state the position it was taken at; bpx is built through the kennel's doors, which state it: tt/bkw-h.Heel.sh bpen_app"
);

/// The two verbs, each spelled by the bootstrap that execs it.
const ZBPE_VERB_EMPLACE: &str = "emplace";
const ZBPE_VERB_VACATE: &str = "vacate";

/// The two arguments each verb takes.
const ZBPE_ARG_PARCEL: &str = "--parcel";
const ZBPE_ARG_BURC: &str = "--burc";

/// A refusal of the act, and a refusal of the call's shape.
const ZBPE_EXIT_REFUSED: u8 = 1;
const ZBPE_EXIT_USAGE: u8 = 2;

const ZBPE_USAGE: &[&str] = &[
    "bpx: install or uninstall an extracted parcel through its own engine",
    "",
    "  bpx emplace --parcel <parcel-dir> --burc <target burc.env, or a seat's directory>",
    "  bpx vacate  --parcel <parcel-dir> --burc <target burc.env>",
    "",
    "The parcel's bootstraps exec it: bpi_install.sh and bpu_uninstall.sh at the parcel's root.", //
];

// THE INSTALL'S RESULT PHRASES, each the fixed text of one line the emplace
// prints once its install has landed, without the `emplace:` tag; a phrase a
// value interrupts stands in pieces, the connecting words staying beside their
// holes. Read by nothing: no track reads a door's output.

/// The success line's opening, ahead of its counts.
const ZBPE_EMPLACE_SUCCESS: &str = "success -";
/// The binary line where the parcel's binary kit was taken, ahead of the platform.
const ZBPE_EMPLACE_BINARY: &str = "canonical binary emplaced for";
/// The binary line where it was not.
const ZBPE_EMPLACE_NO_BINARY: &str = "no canonical binary emplaced - its kit was not taken";
/// The owed heading over a prior edition, ahead of the prior release number.
const ZBPE_EMPLACE_OWED_MOVING: &str = "owed - what moving from number";
/// That heading's close, after the new release number.
const ZBPE_EMPLACE_OWED_ASKS: &str = "asks of your own files:";
/// The owed heading on a first install.
const ZBPE_EMPLACE_OWED_FIRST: &str = "owed - what this install asks of your own files:";
/// The commit line, ahead of the commit.
const ZBPE_EMPLACE_COMMITTED: &str = "committed the lay as";
/// The commit line's close, after the commit.
const ZBPE_EMPLACE_COMMITTED_WHERE: &str = "in the seat's own repository";
/// The line where the install committed nothing.
const ZBPE_EMPLACE_UNCOMMITTED: &str =
    "nothing was committed - review the working tree and commit any install delta";
/// The vacate's closing line.
const ZBPE_VACATE_UNCOMMITTED: &str =
    "nothing was committed - review the working tree and commit any uninstall delta";

/// One line on stdout. A stream that cannot be written leaves nowhere to say
/// so, and the exit is the one report left.
fn zbpe_out(line: &str) {
    if writeln!(
        std::io::stdout(),
        "{}",
        line, //
    )
    .is_err()
    {
        std::process::exit(i32::from(ZBPE_EXIT_REFUSED));
    }
}

/// One line on stderr, on the same terms.
fn zbpe_err(line: &str) {
    if writeln!(
        std::io::stderr(),
        "{}",
        line, //
    )
    .is_err()
    {
        std::process::exit(i32::from(ZBPE_EXIT_REFUSED));
    }
}

/// The usage, and the exit a refused shape takes.
fn zbpe_usage(why: &str) -> ExitCode {
    zbpe_err(&format!(
        "bpx: {}",
        why, //
    ));
    for line in ZBPE_USAGE {
        zbpe_err(line);
    }
    ExitCode::from(ZBPE_EXIT_USAGE)
}

/// The parcel and the target a verb is handed: both named, each once, and
/// nothing else.
fn zbpe_paths(rest: &[String]) -> Result<(PathBuf, PathBuf), String> {
    let mut parcel: Option<PathBuf> = None;
    let mut burc: Option<PathBuf> = None;
    let mut at = rest.iter();
    while let Some(arg) = at.next() {
        let slot = match arg.as_str() {
            ZBPE_ARG_PARCEL => &mut parcel,
            ZBPE_ARG_BURC => &mut burc,
            other => {
                return Err(format!(
                    "unknown argument '{}'",
                    other, //
                ));
            }
        };
        if slot.is_some() {
            return Err(format!(
                "'{}' is named twice",
                arg, //
            ));
        }
        let value = at.next().ok_or_else(|| {
            format!(
                "'{}' names no path",
                arg, //
            )
        })?;
        *slot = Some(PathBuf::from(value));
    }
    match (parcel, burc) {
        (Some(parcel), Some(burc)) => Ok((parcel, burc)),
        (None, _) => Err(format!(
            "'{}' is required",
            ZBPE_ARG_PARCEL, //
        )),
        (_, None) => Err(format!(
            "'{}' is required",
            ZBPE_ARG_BURC, //
        )),
    }
}

/// A JSON value on stdout, pretty, for a caller that reads it.
fn zbpe_out_json(value: serde_json::Value) {
    match serde_json::to_string_pretty(&value) {
        Ok(text) => zbpe_out(&text),
        Err(e) => zbpe_err(&format!(
            "bpx: the summary would not compose: {}",
            e, //
        )),
    }
}

/// An optional text as JSON: the text, or null.
fn zbpe_text_or_null(text: &Option<String>) -> serde_json::Value {
    match text {
        Some(t) => serde_json::Value::String(t.clone()),
        None => serde_json::Value::Null,
    }
}

/// Installs the kits an extracted parcel carries into the target repository
/// whose `burc.env` it is handed, under the behest the parcel carries, laying
/// the canonical binary where a taken kit carries one.
fn zbpe_emplace(rest: &[String]) -> ExitCode {
    let (parcel, burc) = match zbpe_paths(rest) {
        Ok(paths) => paths,
        Err(why) => {
            return zbpe_usage(&format!(
                "{}: {}",
                ZBPE_VERB_EMPLACE,
                why, //
            ));
        }
    };
    zbpe_err("emplace: installing kit assets...");
    zbpe_err(&format!(
        "  parcel: {}",
        parcel.display(), //
    ));
    zbpe_err(&format!(
        "  burc: {}",
        burc.display(), //
    ));

    let result = match bpe::bpee_emplace_parcel(&parcel, &burc) {
        Ok(result) => result,
        Err(e) => {
            zbpe_err(&format!(
                "emplace: error: {}",
                e, //
            ));
            return ExitCode::from(ZBPE_EXIT_REFUSED);
        }
    };

    zbpe_out_json(serde_json::json!({
        "number": result.number,
        "kits_installed": result.kits_installed,
        "files_copied": result.files_copied,
        "commands_routed": result.commands_routed,
        "hooks_routed": result.hooks_routed,
        "canonical_platform": zbpe_text_or_null(&result.canonical_platform),
        "prior_number": zbpe_text_or_null(&result.prior_number),
        "owed": result.owed,
        "commit": zbpe_text_or_null(&result.commit),
    }));

    zbpe_err(&format!(
        "emplace: {} {} files, {} commands, {} hooks",
        ZBPE_EMPLACE_SUCCESS,
        result.files_copied,
        result.commands_routed,
        result.hooks_routed, //
    ));
    match &result.canonical_platform {
        Some(platform) => zbpe_err(&format!(
            "emplace: {} {}",
            ZBPE_EMPLACE_BINARY,
            platform, //
        )),
        None => zbpe_err(&format!(
            "emplace: {}",
            ZBPE_EMPLACE_NO_BINARY, //
        )),
    }
    match &result.prior_number {
        Some(prior) => zbpe_err(&format!(
            "emplace: {} {} to number {} {}",
            ZBPE_EMPLACE_OWED_MOVING,
            prior,
            result.number,
            ZBPE_EMPLACE_OWED_ASKS, //
        )),
        None => zbpe_err(&format!(
            "emplace: {}",
            ZBPE_EMPLACE_OWED_FIRST, //
        )),
    }
    for line in &result.owed {
        zbpe_err(&format!(
            "emplace:   {}",
            line, //
        ));
    }
    if let Some(regime) = &result.regime {
        zbpe_err(&format!(
            "emplace: {}",
            regime, //
        ));
    }
    match &result.commit {
        Some(commit) => zbpe_err(&format!(
            "emplace: {} {} {}",
            ZBPE_EMPLACE_COMMITTED,
            commit,
            ZBPE_EMPLACE_COMMITTED_WHERE, //
        )),
        None => zbpe_err(&format!(
            "emplace: {}",
            ZBPE_EMPLACE_UNCOMMITTED, //
        )),
    }
    ExitCode::SUCCESS
}

/// Removes the Claude Code integration the kits installed from the target
/// repository whose `burc.env` it is handed, leaving the kit directories
/// standing.
fn zbpe_vacate(rest: &[String]) -> ExitCode {
    let (parcel, burc) = match zbpe_paths(rest) {
        Ok(paths) => paths,
        Err(why) => {
            return zbpe_usage(&format!(
                "{}: {}",
                ZBPE_VERB_VACATE,
                why, //
            ));
        }
    };
    zbpe_err("vacate: removing kit assets...");
    zbpe_err(&format!(
        "  parcel: {}",
        parcel.display(), //
    ));
    zbpe_err(&format!(
        "  burc: {}",
        burc.display(), //
    ));

    let result = match bpe::bpee_vacate_parcel(&parcel, &burc) {
        Ok(result) => result,
        Err(e) => {
            zbpe_err(&format!(
                "vacate: error: {}",
                e, //
            ));
            return ExitCode::from(ZBPE_EXIT_REFUSED);
        }
    };

    for path in &result.removed {
        zbpe_err(&format!(
            "  removed: {}",
            path.display(), //
        ));
    }
    zbpe_out_json(serde_json::json!({
        "kits_removed": result.kits_removed,
        "files_deleted": result.files_deleted,
        "commands_removed": result.commands_removed,
        "hooks_removed": result.hooks_removed,
    }));
    zbpe_err(&format!(
        "vacate: success - {} files, {} commands, {} hooks removed",
        result.files_deleted,
        result.commands_removed,
        result.hooks_removed, //
    ));
    zbpe_err(&format!(
        "vacate: {}",
        ZBPE_VACATE_UNCOMMITTED, //
    ));
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let mut args: Vec<String> = Vec::new();
    for arg in std::env::args_os().skip(1) {
        match arg.into_string() {
            Ok(text) => args.push(text),
            Err(raw) => {
                return zbpe_usage(&format!(
                    "an argument is not text: {:?}",
                    raw, //
                ));
            }
        }
    }
    match args.first().map(String::as_str) {
        None => {
            for line in ZBPE_USAGE {
                zbpe_out(line);
            }
            ExitCode::SUCCESS
        }
        Some(ZBPE_EXERGUE_FLAG) => {
            zbpe_out(ZBPE_POSITION);
            ExitCode::SUCCESS
        }
        Some(ZBPE_VERB_EMPLACE) => zbpe_emplace(&args[1..]),
        Some(ZBPE_VERB_VACATE) => zbpe_vacate(&args[1..]),
        Some(other) => zbpe_usage(&format!(
            "unknown verb '{}'",
            other, //
        )),
    }
}

// eof
