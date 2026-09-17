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

//! The door law's reading: whether the repository a collar stands in carries
//! anything uncommitted (BKSCL-Collar.adoc "Door Law").
//!
//! THIS MODULE READS AND NEVER REFUSES, and the split is the cinch rather than a
//! shape that happened. The refusal belongs to the DOOR: a door builds nothing,
//! launches nothing and sweeps nothing while the tree is dirty, so that every
//! record maps to a position. The LIBRARY holds no such posture, because a
//! library consumer has its own — one such consumer reads cargo metadata
//! in-process on exactly the dirty tree its own census is driven against, before
//! every commit, and a refusal seated here would break it.
//!
//! So a consumer asks this module what stands, and decides for itself. The
//! binary's answer is to refuse and name the remedy; that consumer's is to read
//! on. Both are correct, and neither is expressible if the refusal lives here.
//!
//! CLEANLINESS IS THE WHOLE TREE'S, never the elected roots'. Elected roots
//! decide CURRENCY — whether a built artifact has been outrun — which is a
//! different question with a different answer. A dirty test file refuses exactly
//! as a dirty source file does, because the record a commit makes is a record of
//! the whole tree's state and a partial one would be a lie about the rest.

use std::path::Path;
use std::process::Command;

/// What a repository is carrying that no commit holds.
///
/// The two arms are kept apart because they read differently to the person who
/// has to fix them: a modification is work in progress, an untracked file is
/// often work the author does not yet know git cannot see.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkcf_Standing {
    /// Every path `git status --porcelain` named, in the order it named them,
    /// each still carrying its two-character status field.
    pub entries: Vec<String>,
}

/// The act a dirty tree needs.
///
/// GIT'S OWN WORD, not the estate's. The kennel ships as source and is read by
/// consumers who hold no part of this estate's vocabulary, so the library face
/// names the act in the dialect anyone with a repository already speaks. The
/// estate's own word for it is generated vocabulary homed in a kit no parcel
/// carries, so the kit's bash doors name the act this way too rather than
/// reaching outside the kit for a declaration a receiving station does not hold.
///
/// Declared once and cited everywhere else, the grievance included: a hurdle
/// that spelled it a second time to assert the grievance carries it would be a
/// second authority for a word this crate states in one place.
pub const BKCF_REMEDY: &str = "commit";

impl bkcf_Standing {
    /// Whether the tree is clean — nothing modified, nothing staged, nothing
    /// untracked.
    pub fn bkcf_clean(&self) -> bool {
        self.entries.is_empty()
    }

    /// The refusal a door owes when this standing is not clean: what is wrong,
    /// and the remedy, in the estate's own vocabulary.
    ///
    /// The count is bounded in the sentence and the whole list follows, because
    /// a door that says "the tree is dirty" and stops has handed its reader a
    /// second search to run.
    pub fn bkcf_grievance(&self, repository: &Path) -> String {
        let mut said = format!(
            "the repository at {} carries {} uncommitted change(s), and every kennel door \
             refuses one so that each record maps to a position — {} it and drive again",
            repository.display(),
            self.entries.len(),
            BKCF_REMEDY, //
        );
        for entry in &self.entries {
            said.push_str("\n  ");
            said.push_str(entry);
        }
        said
    }
}

/// Read what stands uncommitted in a repository.
///
/// `--porcelain` is asked for by name rather than plain `status`: it is git's
/// own stable machine face, so nothing here parses prose that a git release is
/// free to reword. UNTRACKED FILES ARE INCLUDED, which is porcelain's default
/// and is stated here because it is the arm that matters — a door law that
/// admitted untracked files would let a whole unwritten module ride along
/// invisibly under a clean verdict.
pub fn bkcf_standing(repository: &Path) -> Result<bkcf_Standing, String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repository)
        .arg("status")
        .arg("--porcelain")
        .output()
        .map_err(|err| format!("could not run git in {}: {}", repository.display(), err))?;

    if !out.status.success() {
        let said = String::from_utf8_lossy(&out.stderr);
        return Err(format!(
            "{} is no git repository, or git refused to read it: {}",
            repository.display(),
            said.trim()
        ));
    }

    let text = String::from_utf8(out.stdout)
        .map_err(|err| format!("git status emitted no readable text: {}", err))?;

    Ok(bkcf_Standing {
        entries: text
            .lines()
            .map(str::trim_end)
            .filter(|line| !line.is_empty())
            .map(str::to_string)
            .collect(),
    })
}

/// Read an election — the elected source roots, one repo-relative pathspec per
/// line, `#`-comments and blanks ignored.
///
/// The grammar is git's own, so a directory reaches everything beneath it and an
/// exclusion pathspec carves back out. Nothing here interprets a line: the roots
/// file is the single authoritative statement of what counts as source, and a
/// reader that second-guessed it would be a second election.
pub fn bkcf_election(path: &Path) -> Result<Vec<String>, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|err| format!("could not read the election at {}: {}", path.display(), err))?;

    let roots: Vec<String> = text
        .lines()
        .map(|line| line.split('#').next().unwrap_or("").trim())
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect();

    if roots.is_empty() {
        return Err(format!(
            "the election at {} names no root — an empty election would measure a position over nothing",
            path.display()
        ));
    }

    Ok(roots)
}

/// The seat's own position: its newest first-parent commit touching an elected
/// root.
///
/// WALKED OVER THE ELECTION, never bare HEAD. What stales an artifact is a
/// change to what it is made from; bare HEAD would report a new position for
/// every commit anywhere in the repository, so a binary that had not changed
/// would read as outrun.
///
/// THE SEAT'S OWN LINE, never the trunk counterpart. The exergue's stamp reads
/// landings so that a reader holding a record of landings rather than the
/// repository can resolve it — an engine's need. A seat asking whether its own
/// binary is current holds the repository, and a commit made here never enters
/// the trunk walk at all, so that reading would sit still while the source
/// moved.
///
/// The election is the caller's to supply rather than discovered here, so that a
/// consumer over another repository — a lure, a collar's tree — states its own
/// rather than inheriting the kennel's.
pub fn bkcf_seat_position(repository: &Path, election: &[String]) -> Result<String, String> {
    if election.is_empty() {
        return Err(format!(
            "no election was given for {} — a position measured over nothing is not a position",
            repository.display()
        ));
    }

    let out = Command::new("git")
        .arg("-C")
        .arg(repository)
        .arg("log")
        .arg("-n")
        .arg("1")
        .arg("--first-parent")
        .arg("--format=%H")
        .arg("HEAD")
        .arg("--")
        .args(election)
        .output()
        .map_err(|err| format!("could not run git in {}: {}", repository.display(), err))?;

    if !out.status.success() {
        let said = String::from_utf8_lossy(&out.stderr);
        return Err(format!(
            "could not walk the seat's line in {}: {}",
            repository.display(),
            said.trim()
        ));
    }

    let text = String::from_utf8(out.stdout)
        .map_err(|err| format!("git log emitted no readable text: {}", err))?;

    let sha = text.trim();
    if sha.is_empty() {
        return Err(format!(
            "no commit on this seat's line in {} touches any elected root",
            repository.display()
        ));
    }

    Ok(sha.to_string())
}

// eof
