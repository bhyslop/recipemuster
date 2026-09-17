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

//! The kit's closure: what this kit's bash is allowed to reach.
//!
//! THE RULE. No file under this kit names a path outside the kit and the
//! substrate in a line the shell executes. A parcel carries this kit's files and
//! the substrate's; a source line reaching for a third kit resolves to nothing on
//! a receiving station, and bash's `source` of an absent file is fatal. So such a
//! line does not degrade the door — it kills the bootstrap at the line, ahead of
//! every refusal the door was written to give, and the station is told about a
//! missing file rather than about its own tree.
//!
//! WHY A HURDLE AND NOT A REVIEW. The breach costs nothing where it is written
//! and everything where it lands: the estate's own seat carries every kit, so a
//! line reaching into a third one works perfectly here and forever, and the
//! defect is observable only on a station nobody in this repository is standing
//! at. That is precisely the shape a reviewer cannot hold and a hurdle can.
//!
//! IT READS THE TREE RATHER THAN A LIST. A roster of the kit's bash files would
//! have to be kept in step with the kit, and the failure of an un-kept roster is
//! silence: a file added and not listed is a file not checked, and the hurdle
//! goes on passing. The walk finds every `.sh` this kit carries, so a new door
//! is under the rule the moment it exists.
//!
//! WHAT IT DOES NOT READ IS PROSE. A comment is free to name any file in the
//! estate and several do, citing a pattern or a home; what the rule governs is a
//! line the shell runs. So the reading is of `source` and `.` statements, and a
//! citation in a doc comment is left alone.

use std::path::Path;
use std::path::PathBuf;

/// The two directory names a kit file's source line may name: this kit's own,
/// and the substrate's, which every parcel carries beside it.
const BKTW_ADMITTED: &[&str] = &["bkk", "buk"];

/// The kit's own directory, which is this crate's manifest directory.
fn zbktw_kit() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Every `.sh` file the kit carries, walked rather than listed.
fn zbktw_bash(at: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(at) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // Cargo's own scratch is not the kit's source, and it is large.
        if path.file_name().is_some_and(|name| name == "target") {
            continue;
        }

        if path.is_dir() {
            zbktw_bash(&path, found);
        } else if path.extension().is_some_and(|ext| ext == "sh") {
            found.push(path);
        }
    }
}

/// Whether one line is a shell statement that sources another file.
///
/// BOTH SPELLINGS, because bash reads them identically and a rule that governed
/// only the word would be evaded by the dot without anyone intending to evade
/// it.
fn zbktw_sources(line: &str) -> bool {
    let trimmed = line.trim_start();

    if trimmed.starts_with('#') {
        return false;
    }

    trimmed.starts_with("source ") || trimmed.starts_with(". ")
}

/// The kit directory a source line reaches, where it reaches outside this kit.
///
/// THE READING IS OF THE PATH'S SHAPE AND NOT OF ITS RESOLUTION. A line's path is
/// composed at run time out of variables the hurdle does not hold, so what can be
/// read here is which kit DIRECTORY NAME it spells — which is exactly what the
/// rule is about. A line naming no kit directory at all reaches nothing this rule
/// governs.
fn zbktw_reaches(line: &str) -> Option<String> {
    for token in line.split(['/', '"', '\'', '{', '}', ' ']) {
        if token.is_empty() {
            continue;
        }
        if BKTW_ADMITTED.contains(&token) {
            return None;
        }
    }

    // Every kit stands as one directory under the tools root, so a source line
    // leaving this kit spells that directory's name between slashes.
    let mut said = None;
    for token in line.split('/') {
        let name = token.rsplit(['"', '\'', '}', ' ']).next().unwrap_or("");
        if name.is_empty() || name.contains('$') || name.contains('.') {
            continue;
        }
        if !BKTW_ADMITTED.contains(&name) {
            said = Some(name.to_string());
        }
    }
    said
}

#[test]
fn bktw_no_kit_bash_sources_a_file_outside_the_kit_and_the_substrate() {
    let kit = zbktw_kit();

    let mut bash = Vec::new();
    zbktw_bash(&kit, &mut bash);

    // THE COVERAGE IS ASSERTED BESIDE THE FINDING, because finding nothing is
    // this reading's ordinary answer and a zero-hit reading is valid only behind
    // a control. A walk that reached no file at all would report exactly the
    // clean verdict a closed kit reports, and the two would be indistinguishable.
    assert!(
        bash.len() >= 4,
        "the walk reached the kit's bash: {} file(s) found under {}",
        bash.len(),
        kit.display()
    );

    let mut breaches = Vec::new();
    let mut sourced = 0usize;

    for path in &bash {
        let Ok(text) = std::fs::read_to_string(path) else {
            panic!("the kit carries {} but it could not be read", path.display());
        };

        for (index, line) in text.lines().enumerate() {
            if !zbktw_sources(line) {
                continue;
            }
            sourced += 1;

            if let Some(outside) = zbktw_reaches(line) {
                breaches.push(format!(
                    "{}:{} reaches {}: {}",
                    path.display(),
                    index + 1,
                    outside,
                    line.trim()
                ));
            }
        }
    }

    // THE SECOND HALF OF THE CONTROL. The walk found files; this says it found
    // source lines IN them, so a reading defeated by a parse rather than by a
    // path reports as a defeat instead of as a pass.
    assert!(
        sourced >= 4,
        "the reading found source lines to judge: {} across {} file(s)",
        sourced,
        bash.len()
    );

    assert!(
        breaches.is_empty(),
        "every source line in this kit names the kit or the substrate, because a parcel \
         carries those two and nothing else - a line reaching further kills the bootstrap \
         at the source line on every receiving station, ahead of every refusal it was \
         written to give:\n  {}",
        breaches.join("\n  ")
    );
}

// eof
