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

//! Every tracked bash file under the tools directory, linted at every run.
//!
//! WHY THIS STANDS AS A HURDLE AT ALL. The lint the substrate qualifies a parcel
//! with reaches only the kits that parcel carries, so a defect in a kit no parcel
//! happens to carry sleeps until one does, and then refuses that parcel. Linting
//! the whole tree on every run is what keeps a defect from sleeping.
//!
//! THE PROGRAM ON THE PATH, AND NOT A FIXED EDITION OF IT. Above the floor this
//! hurdle refuses no version, and it states the one it ran, so two machines
//! disagreeing about a finding show which editions disagreed. Below the floor it
//! is red, and a machine holding no shellcheck at all is red naming the tool:
//! a hurdle that skipped for want of its tool would read green while judging
//! nothing.
//!
//! THE SAME FLAGS THE SUBSTRATE'S OWN LINT PASSES. The rc file the substrate
//! keeps, style severity, the gcc line format, and every file read alone. One
//! invocation over the whole listing judges each file as one invocation per kit
//! does, since nothing here follows a sourced file.
//!
//! OBSERVED FROM OUTSIDE THE PROGRAM, as every hurdle here is.

#![deny(warnings)]

use but::buts_seat::buts_substrate;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

/// The program this hurdle runs, found on the path.
const BUTP_PROGRAM: &str = "shellcheck";

/// The oldest edition that honours the rc file.
///
/// Read off the ground the substrate's qualification gives for its own fixed
/// edition (`Tools/buk/buq_qualify.sh`): the rc file flag did not exist before
/// 0.10.0, and an older binary rejects it and runs with none of the estate's
/// rules, which is a false pass rather than a refusal.
const BUTP_FLOOR: [u32; 3] = [0, 10, 0];

/// Where the program is obtained, as the substrate's qualification names it.
const BUTP_UPSTREAM: &str = "https://www.shellcheck.net";

/// The lead-in of the version line in the program's own answer.
const BUTP_VERSION_LEAD: &str = "version:";

/// A file the listing must carry, so an empty or misrooted listing is red rather
/// than a clean pass over nothing.
const BUTP_CONTROL: &str = "Tools/buk/buq_qualify.sh";

/// The floor as the program prints a version.
fn zbutp_floor_text() -> String {
    BUTP_FLOOR
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(".")
}

/// Read the version out of the program's answer, and hold it against the floor.
///
/// The version is the line opening the lead-in, its spaces stripped, as the
/// substrate's qualification reads it. A version this cannot parse is red naming
/// what it read, never taken as passing.
fn zbutp_version(answer: &str) -> Result<String, String> {
    let line = answer
        .lines()
        .find(|line| line.starts_with(BUTP_VERSION_LEAD))
        .ok_or_else(|| {
            format!(
                "{} printed no line opening '{}', so its version cannot be held against the floor {}:\n{}",
                BUTP_PROGRAM,
                BUTP_VERSION_LEAD,
                zbutp_floor_text(),
                answer, //
            )
        })?;
    let version: String = line[BUTP_VERSION_LEAD.len()..]
        .chars()
        .filter(|c| *c != ' ')
        .collect();

    let parts: Vec<Option<u32>> = version.split('.').map(|part| part.parse().ok()).collect();
    if parts.len() != BUTP_FLOOR.len() || parts.iter().any(Option::is_none) {
        return Err(format!(
            "{} stated its version as '{}', which does not read as three numbers, so it cannot be held against the floor {}",
            BUTP_PROGRAM,
            version,
            zbutp_floor_text(), //
        ));
    }
    let numbers: Vec<u32> = parts.into_iter().flatten().collect();
    if numbers.as_slice() < BUTP_FLOOR.as_slice() {
        return Err(format!(
            "{} {} is older than the floor {}: an edition before it does not honour the rc file and runs without the estate's rules - obtain {} or newer from {}",
            BUTP_PROGRAM,
            version,
            zbutp_floor_text(),
            zbutp_floor_text(),
            BUTP_UPSTREAM, //
        ));
    }
    Ok(version)
}

/// Ask the program its version, refusing where no program answers.
fn zbutp_ask(program: &Path) -> Result<String, String> {
    let out = Command::new(program)
        .arg("--version")
        .output()
        .map_err(|error| {
            format!(
                "no {} answers at '{}' ({}): this hurdle lints with the program on the path and is red without it - obtain {} or newer from {}",
                BUTP_PROGRAM,
                program.display(),
                error,
                zbutp_floor_text(),
                BUTP_UPSTREAM, //
            )
        })?;
    let answer = String::from_utf8_lossy(&out.stdout).to_string();
    if !out.status.success() {
        return Err(format!(
            "'{} --version' exited {}, so its version cannot be held against the floor {}:\n{}{}",
            program.display(),
            out.status,
            zbutp_floor_text(),
            answer,
            String::from_utf8_lossy(&out.stderr), //
        ));
    }
    zbutp_version(&answer)
}

/// Every tracked bash file under the tools directory, repository-relative.
fn zbutp_listing(root: &Path, tools: &str) -> Vec<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["ls-files", "-z", "--", tools])
        .output()
        .expect("git answers");
    assert!(
        out.status.success(),
        "git refused to list the tracked files under {}: {}",
        tools,
        String::from_utf8_lossy(&out.stderr), //
    );
    String::from_utf8_lossy(&out.stdout)
        .split('\0')
        .filter(|path| path.ends_with(".sh"))
        .map(str::to_string)
        .collect()
}

#[test]
fn butp_lint_every_tracked_bash_file() {
    let version =
        zbutp_ask(Path::new(BUTP_PROGRAM)).unwrap_or_else(|refusal| panic!("{}", refusal));

    // UNCAPTURED ON PURPOSE. The harness discards a green run's captured output,
    // and takes only the print macros into its capture, so this line reaches the
    // run's whole stream whatever the verdict.
    writeln!(
        std::io::stdout(),
        "{} version: {}",
        BUTP_PROGRAM,
        version, //
    )
    .expect("the version line is written");

    let substrate = buts_substrate();
    let tools_dir = substrate
        .parent()
        .expect("the substrate stands in a tools directory");
    let root: PathBuf = tools_dir
        .parent()
        .expect("the tools directory stands in a repository")
        .to_path_buf();
    let tools = tools_dir
        .file_name()
        .expect("the tools directory has a name")
        .to_string_lossy()
        .to_string();
    let rcfile = substrate.join("busc_shellcheckrc");
    assert!(
        rcfile.is_file(),
        "no rc file stands at {}",
        rcfile.display(), //
    );

    let listing = zbutp_listing(&root, &tools);
    assert!(
        listing.iter().any(|path| path == BUTP_CONTROL),
        "the listing under {} does not carry {}, so it cannot be trusted to carry anything ({} paths listed)",
        tools,
        BUTP_CONTROL,
        listing.len(), //
    );

    let out = Command::new(BUTP_PROGRAM)
        .current_dir(&root)
        .arg(format!("--rcfile={}", rcfile.display()))
        .args(["-S", "style", "-f", "gcc"])
        .args(&listing)
        .output()
        .expect("the program answered its version, so it runs");
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr), //
    );
    assert!(
        out.status.success() && said.is_empty(),
        "{} {} over {} bash files under {} exited {} with these findings:\n{}",
        BUTP_PROGRAM,
        version,
        listing.len(),
        tools,
        out.status,
        said, //
    );
}

#[test]
fn butp_floor_refuses_older() {
    let refusal = zbutp_version("ShellCheck - shell script analysis tool\nversion: 0.9.0\n")
        .expect_err("an edition below the floor is red");
    assert!(
        refusal.contains(&zbutp_floor_text()),
        "the refusal names the floor: {}",
        refusal, //
    );
}

#[test]
fn butp_floor_takes_floor_and_newer() {
    for given in ["0.10.0", "0.11.0"] {
        let answer = format!(
            "ShellCheck - shell script analysis tool\nversion: {}\n",
            given
        );
        assert_eq!(
            zbutp_version(&answer).as_deref(),
            Ok(given),
            "an edition at or above the floor passes", //
        );
    }
}

#[test]
fn butp_absent_program_is_red() {
    let nowhere = Path::new(env!("CARGO_MANIFEST_DIR")).join(BUTP_PROGRAM);
    assert!(
        !nowhere.exists(),
        "the path standing for an absent program must stand nowhere: {}",
        nowhere.display(), //
    );
    let refusal = zbutp_ask(&nowhere).expect_err("a program that stands nowhere is red");
    assert!(
        refusal.contains(BUTP_PROGRAM) && refusal.contains(&nowhere.display().to_string()),
        "the refusal names the program: {}",
        refusal, //
    );
}

// eof
