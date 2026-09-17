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

//! build.rs - the three facts about its own making that the kennel carries.
//!
//! Each is here because it can be observed nowhere else: cargo's own choices are
//! visible to a build script and to nothing downstream of one.
//!
//!   SEAT       the position the build was taken at - the seat's newest
//!              first-parent commit touching an elected root, WALKED HERE. It is
//!              the seat's own line rather than the trunk counterpart
//!              `Tools/buk/bue_exergue.sh` walks: that module reads LANDINGS and
//!              never HEAD so a reader holding a record rather than the
//!              repository can resolve its answer, and a commit made at a billet
//!              never enters it. A binary whose only consumer is the whistle that
//!              built it needs the reading that moves when this tree's source
//!              moves.
//!
//! THE WALK IS TAKEN HERE AND NOT HANDED IN, and that is this script's whole
//! shape. What stood before had the whistle walk the position, compare it against
//! the standing binary and decide whether to build - a staleness reading in bash,
//! ahead of the tool whose trade is staleness readings, and reachable only by the
//! doors that ran that bash. A bare `cargo build` took no such reading and no
//! such refusal. Now every build of this crate passes here, whoever started it.
//!
//! ONE READING, NOT A SECOND. The door law, the election parse and the seat walk
//! are `src/bkcf_guard.rs`'s, reached by `#[path]` rather than copied: the crate
//! links that module and so does this script, so the build and the binary cannot
//! come to define currency differently. The module stands on `std` alone, which
//! is what makes it reachable from a build script at all.
//!
//!   DOOR LAW   a build of this crate refuses a repository carrying uncommitted
//!              work, so that every artifact maps to a position. Held here rather
//!              than in each door's bash for the same reason the walk is: a
//!              refusal the bash holds refuses only the doors that ran the bash.
//!
//!   COMPILER   the version of the rustc that actually ran. The pin is stated at
//!              the invocation rather than inherited, but a stated pin is still
//!              only a request; this is the observed answer, and it is what makes
//!              a bumped pin provable rather than assumed.
//!
//!   PIN        the channel the manifest's toolchain file names, read at build
//!              time. Carried beside the compiler so a reader can see the request
//!              and the answer together and judge whether they agree.
//!
//! A build that cannot state all three DIES here. There is no sentinel value and
//! no "unknown": an unstamped kennel would answer questions about itself with
//! text that means nothing, which is worse than refusing to exist.

use std::path::{Path, PathBuf};
use std::process::Command;

// THE CRATE'S OWN READING, REACHED AND NEVER COPIED. A build script cannot link
// the library it is building, so the alternative to this line is a second walk
// living here - and two walks are two answers about what makes this binary
// stale. The module names nothing but `std`, which is what lets it be read from
// both sides of that boundary.
// The crate states these once at its own margin; a `#[path]` module is compiled
// twice, so the second compilation states them too. The dead-code allow is this
// side's alone: a build script reaches three of the module's readings and the
// crate reaches the rest, and neither is short anything the other holds.
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[path = "src/bkcf_guard.rs"]
mod bkcf_guard;

// EVERY DECLARATION HERE STANDS INSIDE `main`, and that is the naming law rather
// than a style. A file cargo names carries no prefix of its own, so a prefixed
// declaration at this file's margin wears a prefix the file cannot answer for.
// What the law exempts is the function body: a const or a nested fn declared
// inside one is exported by nothing and reachable from nowhere else, so no grep
// can land wrong and no discussion can be ambiguous about which one is meant.
// Moving them inward is therefore the repair, not a dodge around the reading.
fn main() {
    /// The kennel's own pin file, beside this script.
    const ZBKK_PIN_FILE: &str = "rust-toolchain.toml";

    /// The election, beside this script: the one authoritative statement of what
    /// counts as source for this binary. Named here and never restated - the
    /// crate's own reader parses the same file.
    const ZBKK_ELECTION_FILE: &str = "bkce_roots.txt";

    /// Where the repository this build stands in begins.
    ///
    /// ASKED OF GIT AND NEVER ASSUMED. Cargo runs a build script with the CRATE
    /// directory as its working directory, while the election's pathspecs are
    /// repository-relative, so a script that walked upward by a fixed count would
    /// be pinned to one layout of the tree it happens to sit in.
    fn zbkk_repository() -> PathBuf {
        let out = Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output()
            .unwrap_or_else(|err| panic!("could not run git to find the repository: {}", err));

        if !out.status.success() {
            panic!(
                "git would not name the repository this crate stands in: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }

        let said = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if said.is_empty() {
            panic!("git named no repository for this crate");
        }

        PathBuf::from(said)
    }

    /// Tell cargo to watch one of git's own files, resolved through git.
    ///
    /// RESOLVED AND NEVER SPELLED. A billet is a worktree, so its `.git` is a
    /// FILE naming a common directory elsewhere; a literal `.git/HEAD` names
    /// nothing there, and a trigger that names nothing never fires - which
    /// presents as a binary that quietly stops noticing commits rather than as an
    /// error. `--git-path` is git's own answer to where a given file actually
    /// stands, and it answers correctly at a worktree and at a plain clone alike.
    ///
    /// A path git names but that does not yet exist is watched anyway: cargo
    /// treats an absent watched path as a reason to re-run, which is the right
    /// answer for a ref file that appears on the next commit.
    fn zbkk_watch_git_path(relative: &str) {
        let Ok(out) = Command::new("git").arg("rev-parse").arg("--git-path").arg(relative).output()
        else {
            return;
        };

        if !out.status.success() {
            return;
        }

        let said = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !said.is_empty() {
            println!("cargo:rerun-if-changed={}", said);
        }
    }

    /// Ask the compiler that is compiling this build script for its own version.
    ///
    /// `RUSTC` is cargo's own statement of which compiler it is driving, so this
    /// reads the answer rather than re-deriving it: a pin stated at the
    /// invocation and a compiler that actually ran are two different facts, and
    /// asking anything but cargo's own choice would collapse them.
    fn zbkk_compiler_version() -> String {
        let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());

        let out = Command::new(&rustc)
            .arg("--version")
            .output()
            .unwrap_or_else(|err| panic!("could not run {} --version: {}", rustc, err));

        if !out.status.success() {
            panic!("{} --version exited {}", rustc, out.status);
        }

        let text = String::from_utf8(out.stdout)
            .unwrap_or_else(|err| panic!("{} --version emitted no readable text: {}", rustc, err));

        let line = text.trim();
        if line.is_empty() {
            panic!("{} --version emitted nothing", rustc);
        }

        line.to_string()
    }

    /// Read the channel out of the pin file standing beside this script.
    ///
    /// Parsed here rather than taken from a dependency: the file is two lines of
    /// TOML, and a manifest-parsing crate in the kennel's closure would be a
    /// permitted-dependency question asked for nothing.
    fn zbkk_pin_channel(pin_file: &str) -> String {
        let path = Path::new(pin_file);

        let text = std::fs::read_to_string(path)
            .unwrap_or_else(|err| panic!("could not read the pin at {}: {}", pin_file, err));

        for line in text.lines() {
            let trimmed = line.trim();
            let Some(rest) = trimmed.strip_prefix("channel") else {
                continue;
            };
            let Some(rest) = rest.trim_start().strip_prefix('=') else {
                continue;
            };
            let value = rest.trim().trim_matches('"');
            if !value.is_empty() {
                return value.to_string();
            }
        }

        panic!(
            "the pin at {} names no channel, so this build cannot say what it was asked to run under",
            pin_file
        )
    }

    println!("cargo:rerun-if-changed={}", ZBKK_PIN_FILE);
    println!("cargo:rerun-if-changed={}", ZBKK_ELECTION_FILE);

    // WHAT MOVES THE POSITION IS A COMMIT, so what cargo must watch is where git
    // records one. Watching the elected roots themselves would be the wrong
    // reading twice over: an edit that is never committed would restamp the
    // binary at a position the tree no longer stands at, and a commit that
    // changed nothing on disk - a rebase, a branch move - would not restamp it at
    // all.
    zbkk_watch_git_path("HEAD");
    zbkk_watch_git_path("logs/HEAD");

    let repository = zbkk_repository();

    // THE DOOR LAW, HELD BEFORE THE POSITION IS WALKED. A dirty tree's position
    // is a true answer to the wrong question: it names the commit this build
    // would claim while the build is taken from something else entirely.
    match bkcf_guard::bkcf_standing(&repository) {
        // THE GRIEVANCE IS THE MODULE'S, not one composed here. It names the
        // remedy from the constant that declares it, so a build script spelling
        // its own sentence would be a second authority for a word the crate
        // states in one place - and would drift the day the crate's own doors
        // reworded theirs.
        Ok(standing) if standing.bkcf_clean() => {}
        Ok(standing) => panic!("{}", standing.bkcf_grievance(&repository)),
        Err(said) => panic!("this build could not survey the repository it stands in: {}", said),
    }

    let election = bkcf_guard::bkcf_election(Path::new(ZBKK_ELECTION_FILE))
        .unwrap_or_else(|said| panic!("this build could not read its own election: {}", said));

    let seat = bkcf_guard::bkcf_seat_position(&repository, &election).unwrap_or_else(|said| {
        panic!("this build cannot state the position it was taken at: {}", said)
    });

    println!("cargo:rustc-env=BKCS_SEAT={}", seat.trim());
    println!("cargo:rustc-env=BKCS_COMPILER={}", zbkk_compiler_version());
    println!("cargo:rustc-env=BKCS_PIN={}", zbkk_pin_channel(ZBKK_PIN_FILE));
}

// eof
