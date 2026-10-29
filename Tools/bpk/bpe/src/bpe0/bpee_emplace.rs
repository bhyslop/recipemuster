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

//! Emplace/Vacate - Install and uninstall kit assets
//!
//! Full install/uninstall operations. Neither stages nor commits in a
//! repository target: the delta is left whole and uncommitted for the caller
//! to seal.
//! The one exception is the seat, whose repository is the install's own as
//! the studbook is the engine's (BUr_k7d): the seat form commits each lay.
//! Bash scripts (bpi_install.sh, bpu_uninstall.sh) are thin bootstraps
//! that detect platform and exec this kit's own binary, `bpx`, which every
//! parcel carries at its root.
//!
//! THE PARCEL CARRIES ITS BEHEST. What an install needs beyond the parcel's
//! kits and its record — the precinct's rosters and freight, the signets the
//! Claude assets wear, the canonical binary a taken kit carries, and a seat's
//! endonym — is known only to the forge that cut the parcel, so the release
//! writes it into the parcel beside the record, never inside it, and the install
//! and the vacate read it there. A parcel carrying none is refused, naming the
//! file; nothing stands in for it.
//!
//! THE BEHEST IS THE TARGET'S, NEVER THE FORGE'S WHOLE. A member carried by a
//! kit the target does not declare is no part of what that target is told: it
//! stands in the behest as `null` where it holds one value and as an empty list
//! where it holds a list, so every member stands in every behest and a missing
//! one still refuses.
//!
//! Emplace behavior (nuclear install):
//! 0. Resolve the target: a burc.env names a repository; a directory names a
//!    seat, which past the record check is read, emptied but for its own
//!    repository, inited on a first lay, and given its regime declaration here
//! 1. Parse burc.env, resolve paths relative to burc.env location
//! 2. Verify git repo is clean — a seat skips both gates
//! 3. Read the record from the parcel
//! 4. Elect the taken set: parcel kits intersected with BURC_MANAGED_KITS,
//!    refusing only an empty intersection, then read the report of what the
//!    consumer's own files owe while the edition being replaced still stands,
//!    and remove the kits the standing record names that the target no longer
//!    declares
//! 5. Delete the record directory, the elder one where it stands, and the taken
//!    kits' directories (nuclear cleanup)
//! 6. Create the record directory, write the record of the taken set
//! 7. Copy the taken kits, route commands/hooks of taken ciphers only
//! 8. When the caller's binary kit is taken: emplace the canonical binary,
//!    purge siblings
//! 9. A seat: write its regime — launcher stubs, the taken kits' tabtarget
//!    exemplars, the station door, the precinct's rosters — and commit the lay
//!
//! The clean-tree check at step 2 is what makes the resulting dirt legible:
//! everything standing uncommitted afterward is this install and nothing else.
//!
//! CLAUDE.md is never touched: the target repo hand-maintains its own
//! `@`-include lines for kit guidance files.
//!
//! Vacate behavior (removes Claude integration, preserves kit scripts):
//! 0. Read the behest from the parcel it is handed
//! 1. Parse burc.env, verify git clean
//! 2. Read the standing record, at either seat, to get kit list
//! 3. Remove commands and hooks from .claude/
//! 4. Delete the canonical binary and its siblings, where the caller names one
//! 5. Delete the record and its directory
//!
//! Note: Kit directories (buk/, jjk/, etc.) are intentionally preserved.
//! They contain open-source utilities usable outside Claude Code.

use std::collections::{
    BTreeMap,
    BTreeSet,
    HashMap, //
};
use std::fs;
use std::path::{
    Path,
    PathBuf, //
};
use std::process::Command;

use bkp::bkprs_scrivener::{
    bkprs_copy,
    bkprs_program_copy,
    bkprs_program_write, //
};

// =============================================================================
// Constants
// =============================================================================

/// Record JSON field tags (must match the release module's)
const BPEE_TAMGA_NUMBER: &str = "vvmt_tamga";
const BPEE_TAMGA_KITS: &str = "vvmk_kits";

/// The record member recording which form of the install laid the target, and
/// its two values: the seat form, handed a directory, and the repository form,
/// handed a burc.env. Written by the install alone and absent from every parcel's
/// record, a parcel being laid by either form.
const BPEE_TAMGA_MANNER: &str = "vvmm_manner";
const BPEE_MANNER_DIRECTORY: &str = "directory";
const BPEE_MANNER_REPOSITORY: &str = "repository";

macro_rules! zbpee_tamga_dir {
    () => {
        ".vvm"
    };
}
macro_rules! zbpee_tamga_file {
    () => {
        "vvmt_tamga.json"
    };
}

/// The record's directory beneath a target's root, which the install
/// replaces whole on every run.
///
/// PUBLIC FOR THE CENSUS's tackle, which claims the directory in a consumer
/// tree and would otherwise spell it a second time.
pub const BPEE_TAMGA_DIR: &str = zbpee_tamga_dir!();

/// The record's name, at a parcel's root and in the record directory alike.
pub const BPEE_TAMGA_FILE: &str = zbpee_tamga_file!();

/// The record's own repo-relative seat in a lit consumer tree.
pub const BPEE_TAMGA_RELPATH: &str = concat!(
    zbpee_tamga_dir!(),
    "/",
    zbpee_tamga_file!(), //
);

/// The behest's name at a parcel's root, beside the record and never inside it.
/// The release writes it and the install and the vacate read it; no target
/// holds one.
pub const BPEE_BEHEST_FILE: &str = "bpee_behest.json";

// The behest's members. Plain words, the behest being this crate's own file
// written and read here alone.
const ZBPEE_BEHEST_DIRECTORIES: &str = "directories";
const ZBPEE_BEHEST_FREIGHT: &str = "freight";
const ZBPEE_BEHEST_FROM: &str = "from";
const ZBPEE_BEHEST_TO: &str = "to";
const ZBPEE_BEHEST_SUFFIXES: &str = "suffixes";
const ZBPEE_BEHEST_COMMAND: &str = "command";
const ZBPEE_BEHEST_HOOK: &str = "hook";
const ZBPEE_BEHEST_BINARY: &str = "binary";
const ZBPEE_BEHEST_KIT: &str = "kit";
const ZBPEE_BEHEST_CANONICAL: &str = "canonical";
const ZBPEE_BEHEST_PREFIX: &str = "prefix";
const ZBPEE_BEHEST_NAME: &str = "name";

// THE ELDER SPELLING, read during the one transition and never written. Every
// record an install laid before the current shape stands at the elder seat under
// the elder member keys, and a target keeps it until its next install, which
// reads it, removes the elder directory and writes the current shape alone. The
// reader retires with these constants once every target the estate holds has
// taken and committed an install of the current shape. The elder record names no
// forge, and nothing here asks it for one.
macro_rules! zbpee_elder_dir {
    () => {
        ".vvk"
    };
}

/// The elder record directory, public for the census's tackle as the current one
/// is.
pub const BPEE_ELDER_DIR: &str = zbpee_elder_dir!();

/// The elder record's repo-relative seat.
pub const BPEE_ELDER_RELPATH: &str = concat!(
    zbpee_elder_dir!(),
    "/vvbf_brand.json", //
);

/// The elder member keys the transition reads: the release number, the kits, and the
/// manner. The elder record's other members are never read, so none is spelled.
const ZBPEE_ELDER_NUMBER: &str = "vvbf_brand";
const ZBPEE_ELDER_KITS: &str = "vvbk_kits";
const ZBPEE_ELDER_MANNER: &str = "vvbm_manner";

/// The member keys one spelling of a standing record wears, beside the seat that
/// spelling stands at: the seat decides the spelling, since no install ever
/// wrote one spelling at the other's seat.
struct zbpee_Spelling {
    seat: &'static str,
    number: &'static str,
    kits: &'static str,
    manner: &'static str,
}

/// The current spelling, the one every writer here writes.
const ZBPEE_CURRENT: zbpee_Spelling = zbpee_Spelling {
    seat: BPEE_TAMGA_RELPATH,
    number: BPEE_TAMGA_NUMBER,
    kits: BPEE_TAMGA_KITS,
    manner: BPEE_TAMGA_MANNER,
};

/// The elder spelling, read and never written.
const ZBPEE_ELDER: zbpee_Spelling = zbpee_Spelling {
    seat: BPEE_ELDER_RELPATH,
    number: ZBPEE_ELDER_NUMBER,
    kits: ZBPEE_ELDER_KITS,
    manner: ZBPEE_ELDER_MANNER,
};

/// Every spelling a standing record may wear, in the order a tree is read: the
/// current seat first, so a tree holding both answers with the current one.
const ZBPEE_STANDING_SPELLINGS: [&zbpee_Spelling; 2] = [
    &ZBPEE_CURRENT,
    &ZBPEE_ELDER, //
];

/// A record standing in a target, read at whichever seat it stands: the
/// spelling that seat wears, and the body parsed, `None` where it does not
/// parse.
struct zbpee_Standing {
    spelling: &'static zbpee_Spelling,
    json: Option<serde_json::Value>,
}

impl zbpee_Standing {
    /// A member of the standing record under its spelling's key.
    fn zbpee_member(
        &self,
        key: impl Fn(&zbpee_Spelling) -> &'static str,
    ) -> Option<&serde_json::Value> {
        self.json.as_ref()?.get(key(self.spelling))
    }
}

/// The spelling of the record standing under `root`, read off the first seat
/// holding a file, the current one first.
fn zbpee_standing_spelling(root: &Path) -> Option<&'static zbpee_Spelling> {
    ZBPEE_STANDING_SPELLINGS
        .into_iter()
        .find(|spelling| root.join(spelling.seat).is_file())
}

/// THE ONE EITHER-SPELLING READER of a target's own tree: the record standing
/// under `root`, at the current seat or else at the elder one, or `None` where
/// neither holds a file. A file that stands and cannot be read refuses, so the
/// install never reads a record it could not open as a record that is absent.
fn zbpee_standing(root: &Path) -> Result<Option<zbpee_Standing>, String> {
    let Some(spelling) = zbpee_standing_spelling(root) else {
        return Ok(None);
    };
    let body = fs::read_to_string(root.join(spelling.seat)).map_err(|e| {
        format!(
            "Failed to read the prior record {}: {}",
            spelling.seat,
            e, //
        )
    })?;
    let json = serde_json::from_str::<serde_json::Value>(&body).ok();
    Ok(Some(zbpee_Standing { spelling, json }))
}

/// The kits a tree's standing record declares — the set this crate's install owns
/// and replaces — read at either seat in either spelling, through `read`, which
/// answers a repo-relative seat's body or nothing, so a caller holding the tree
/// by any road can ask.
///
/// OFFERED IN PLACE OF THE SEATS AND THE FIELD TAGS, which is the whole reason it
/// is public. A caller wanting the kit set would otherwise spell both seats and
/// both tags in its own crate, agreeing with this one by nothing but care — and
/// the wire shape and its one transition are this crate's to know.
///
/// AN UNREADABLE OR ABSENT RECORD ANSWERS EMPTY rather than failing, which parts
/// it from the install's own reads beneath, which stop. A reader ASKING what a
/// tree received is answered by the empty set, a tree whose record will not parse
/// having declared nothing.
pub fn bpee_tamga_kits(read: impl Fn(&Path) -> Option<String>) -> Vec<String> {
    let Some((spelling, body)) = ZBPEE_STANDING_SPELLINGS
        .into_iter()
        .find_map(|spelling| read(Path::new(spelling.seat)).map(|body| (spelling, body)))
    else {
        return Vec::new();
    };
    let Ok(record) = serde_json::from_str::<serde_json::Value>(&body) else {
        return Vec::new();
    };
    let Some(kits) = record.get(spelling.kits).and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    kits.iter()
        .filter_map(|v| v.as_str())
        .filter(|kit| !kit.is_empty())
        .map(String::from)
        .collect()
}

// =============================================================================
// Git Operations
// =============================================================================

/// Check that git working tree is clean (no uncommitted changes).
///
/// `act` names the operation in the refusal, because emplace and vacate share
/// this gate and a caller told to commit "before install" while uninstalling
/// has been handed the wrong remedy.
fn zbpee_check_git_clean(project_root: &Path, act: &str) -> Result<(), String> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(project_root)
        .output()
        .map_err(|e| {
            format!(
                "Failed to run git status: {}",
                e, //
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "git status failed: {}",
            stderr.trim(), //
        ));
    }

    if !output.stdout.is_empty() {
        return Err(format!(
            "Target repo has uncommitted changes. Commit or stash before {}.",
            act, //
        ));
    }

    Ok(())
}

/// Check that path is a git repository.
fn zbpee_check_is_git_repo(project_root: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .current_dir(project_root)
        .output()
        .map_err(|e| {
            format!(
                "Failed to run git rev-parse: {}",
                e, //
            )
        })?;

    if !output.status.success() {
        return Err(format!(
            "Target is not a git repository: {}",
            project_root.display(), //
        ));
    }

    Ok(())
}

// No staging and no commit function for a repository target lives here, and
// that is the design rather than an omission: sealing the delta is the caller's
// act, as the module header says. The seat's functions below commit, the seat's
// repository being the install's own.

/// The seat's own repository directory, the one entry its clear spares.
const ZBPEE_SEAT_REPOSITORY: &str = ".git";

/// The line a seat's repository is inited on, named as the studbook's record
/// line is named.
const ZBPEE_SEAT_LINE: &str = "trunk";

/// One git call over the seat's own repository, addressed by its directory
/// rather than found by search, so a seat whose repository stood removed meets
/// git's refusal instead of a repository above it. Answers git's stdout.
fn zbpee_seat_git(root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .env("GIT_DIR", root.join(ZBPEE_SEAT_REPOSITORY))
        .env("GIT_WORK_TREE", root)
        .env_remove("GIT_INDEX_FILE")
        .output()
        .map_err(|e| {
            format!(
                "Failed to run git {}: {}",
                args.join(" "),
                e, //
            )
        })?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed in the seat {}: {}",
            args.join(" "),
            root.display(),
            String::from_utf8_lossy(&output.stderr).trim(), //
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Init the seat's repository on a first lay, on the seat line. No remote is
/// configured, now or at any lay.
fn zbpee_init_seat(root: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .args(["init", "-q", "-b", ZBPEE_SEAT_LINE])
        .current_dir(root)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .output()
        .map_err(|e| {
            format!(
                "Failed to run git init: {}",
                e, //
            )
        })?;
    if !output.status.success() {
        return Err(format!(
            "git init failed in the seat {}: {}",
            root.display(),
            String::from_utf8_lossy(&output.stderr).trim(), //
        ));
    }
    Ok(())
}

/// Commit the whole lay in the seat's repository under the station's own git
/// identity, empty or not, so every lay stands as one commit on the seat line.
/// Answers the commit.
fn zbpee_commit_seat(root: &Path, number: &str) -> Result<String, String> {
    let message = format!(
        "the seat form laid number {} at {}",
        number,
        root.display(), //
    );
    zbpee_seat_git(root, &["add", "-A"])?;
    zbpee_seat_git(root, &["commit", "-q", "--allow-empty", "-m", &message])?;
    zbpee_seat_git(root, &["rev-parse", "HEAD"])
}

// =============================================================================
// The regime of a seat
// =============================================================================

/// The starter kit's templates a seat's regime is written
/// from. Compiled in rather than read from the parcel, because the parcel cut
/// for the private seat carries no starter kit. Their home is this kit's own
/// directory, beside this crate.
const ZBPEE_TEMPLATE_BURC: &str = include_str!("../../../bpt_burc.env");
const ZBPEE_TEMPLATE_LAUNCHER: &str = include_str!("../../../bpt_launcher.sh");
const ZBPEE_TEMPLATE_STATION: &str = include_str!("../../../bpt_station.sh");

/// The declaration's kit assignment, replaced whole whatever placeholder the
/// template stands in it, and the other templates' one substitution each,
/// spelled as the template spells it.
const ZBPEE_KITS_ASSIGN: &str = "BURC_MANAGED_KITS=";
const ZBPEE_ENDONYM_ASSIGN: &str = "BURC_ENDONYM=";
const ZBPEE_NEEDLE_WORKBENCH: &str = "buk/buw_workbench.sh";
const ZBPEE_NEEDLE_MOORINGS: &str = "«your-moorings-dirname»";

/// The station door's basename, as the station template's own header names it.
const ZBPEE_STATION_DOOR: &str = "buw-SI.StationInit.sh";

/// The directory a kit carries its tabtarget exemplars in.
const ZBPEE_EXEMPLAR_DIR: &str = "tt";

/// The trampoline's moorings assignment, and the opening of a door's
/// configuration line, whose first word names the door's launcher.
const ZBPEE_MOORINGS_ASSIGN: &str = "z_moorings_dir=\"";
const ZBPEE_MOORINGS_REF: &str = "${z_moorings_dir}/";
const ZBPEE_CONFIGURATION_OPEN: &str = "set -- '";

/// The moorings a seat's regime stands under: the dirname
/// and the launcher directory beneath it, both read off a carried trampoline,
/// since every relative path the trampoline enters is the forge's.
struct zbpee_Moorings {
    dirname: String,
    launchers: String,
}

/// KEY=VALUE pairs of a burc-shaped body, comments and blanks skipped.
fn zbpee_assignments(body: &str) -> HashMap<String, String> {
    let mut vars = HashMap::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(eq_pos) = line.find('=') {
            vars.insert(
                line[..eq_pos].trim().to_string(),
                line[eq_pos + 1..].trim().to_string(),
            );
        }
    }
    vars
}

/// The moorings a trampoline's body names, where it names any.
fn zbpee_trampoline_moorings(body: &str) -> Option<zbpee_Moorings> {
    let uncommented = || {
        body.lines()
            .map(str::trim_start)
            .filter(|l| !l.starts_with('#'))
    };
    let dirname = uncommented()
        .find_map(|l| l.strip_prefix(ZBPEE_MOORINGS_ASSIGN))?
        .split('"')
        .next()?
        .to_string();
    let launchers = uncommented()
        .find_map(|l| {
            l.find(ZBPEE_MOORINGS_REF)
                .map(|at| &l[at + ZBPEE_MOORINGS_REF.len()..])
        })?
        .split('/')
        .next()?
        .to_string();
    let literal = |s: &str| !s.is_empty() && !s.contains('$');
    (literal(&dirname) && literal(&launchers)).then_some(zbpee_Moorings { dirname, launchers })
}

/// Read the moorings off the one trampoline the carried kits' exemplars hold.
fn zbpee_read_moorings(kits_dir: &Path, kits: &[String]) -> Result<zbpee_Moorings, String> {
    let mut found: Vec<zbpee_Moorings> = Vec::new();
    for kit in kits {
        let Ok(entries) = fs::read_dir(kits_dir.join(kit).join(ZBPEE_EXEMPLAR_DIR)) else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(body) = fs::read_to_string(entry.path()) else {
                continue;
            };
            if let Some(moorings) = zbpee_trampoline_moorings(&body) {
                if !found
                    .iter()
                    .any(|m| m.dirname == moorings.dirname && m.launchers == moorings.launchers)
                {
                    found.push(moorings);
                }
            }
        }
    }
    match found.len() {
        1 => Ok(found.remove(0)),
        0 => Err(format!(
            "A seat takes its moorings from a carried trampoline, and none of [{}] carries one",
            kits.join(","), //
        )),
        _ => Err(format!(
            "A seat takes its moorings from one carried trampoline, and [{}] carry {} that disagree",
            kits.join(","),
            found.len(), //
        )),
    }
}

/// The tools directory the starter kit's `burc.env` template declares, as the
/// target-relative path a seat's declaration will name.
fn zbpee_template_tools_dir() -> Result<String, String> {
    zbpee_assignments(ZBPEE_TEMPLATE_BURC)
        .remove("BURC_TOOLS_DIR")
        .ok_or_else(|| "the starter kit's burc.env template declares no BURC_TOOLS_DIR".to_string())
}

/// The template's body without its header block: the comment block opening
/// on a line of `#` alone and closing at the blank line after it, which
/// speaks to a person copying the template by hand and tells them to delete it.
fn zbpee_without_header(template: &str) -> Vec<&str> {
    let mut kept = Vec::new();
    let mut header = false;
    let mut passed = false;
    for line in template.lines() {
        if !passed && !header && line.len() > 1 && line.chars().all(|c| c == '#') {
            header = true;
            continue;
        }
        if header {
            if line.trim().is_empty() {
                header = false;
                passed = true;
            }
            continue;
        }
        kept.push(line);
    }
    kept
}

/// The launcher template's lines kept for a stub: its first line, then every
/// line after the blank line that ends its header, which tells a person
/// copying the template by hand to delete the lines between. Answers none
/// where the header never closes.
fn zbpee_launcher_lines(template: &str) -> Option<Vec<&str>> {
    let mut lines = template.lines();
    let first = lines.next()?;
    let mut kept = vec![first];
    let mut ended = false;
    for line in lines {
        if ended {
            kept.push(line);
        } else if line.is_empty() {
            ended = true;
        }
    }
    ended.then_some(kept)
}

/// Write the regime DECLARATION of a seat: its
/// `burc.env`, declaring the parcel's own kits and the endonym its caller
/// names, carrying no template header block, and the tools directory that
/// declaration names. Answers the `burc.env` written.
fn zbpee_write_declaration(
    root: &Path,
    moorings: &zbpee_Moorings,
    kits: &[String],
    endonym: &str,
) -> Result<PathBuf, String> {
    let dir = root.join(&moorings.dirname);
    fs::create_dir_all(&dir).map_err(|e| {
        format!(
            "Failed to create the moorings {}: {}",
            dir.display(),
            e, //
        )
    })?;
    let burc = dir.join("burc.env");
    let body: String = zbpee_without_header(ZBPEE_TEMPLATE_BURC)
        .into_iter()
        .map(|line| {
            if line.starts_with(ZBPEE_KITS_ASSIGN) {
                format!(
                    "{}{}\n",
                    ZBPEE_KITS_ASSIGN,
                    kits.join(","), //
                )
            } else if line.starts_with(ZBPEE_ENDONYM_ASSIGN) {
                format!(
                    "{}{}\n",
                    ZBPEE_ENDONYM_ASSIGN,
                    endonym, //
                )
            } else {
                format!(
                    "{}\n",
                    line, //
                )
            }
        })
        .collect();
    fs::write(&burc, body).map_err(|e| {
        format!(
            "Failed to write {}: {}",
            burc.display(),
            e, //
        )
    })?;
    let tools_dir = root.join(zbpee_template_tools_dir()?);
    fs::create_dir_all(&tools_dir).map_err(|e| {
        format!(
            "Failed to stand the tools directory {}: {}",
            tools_dir.display(),
            e, //
        )
    })?;
    Ok(burc)
}

/// The record check of a seat, ahead of its wipe, where the record tells the
/// seat and never whether a repository stands: an empty directory is a first
/// lay, and one holding a record the seat form wrote a re-lay. Anything else is
/// refused — a record the repository form wrote, an unreadable one, or none,
/// since only this door writes a record. Answers whether this is a first lay.
fn zbpee_require_seat_tamga_or_empty(root: &Path) -> Result<bool, String> {
    let empty = fs::read_dir(root)
        .map_err(|e| {
            format!(
                "Failed to read {}: {}",
                root.display(),
                e, //
            )
        })?
        .next()
        .is_none();
    if empty {
        return Ok(true);
    }
    let Some(standing) = zbpee_standing(root)? else {
        return Err(format!(
            "Target directory is not empty and holds no {}, so nothing says an install laid it - empty it, or name a target the install laid: {}",
            BPEE_TAMGA_RELPATH,
            root.display(), //
        ));
    };
    let manner = standing.zbpee_member(|s| s.manner).and_then(|v| v.as_str());
    if manner == Some(BPEE_MANNER_DIRECTORY) {
        return Ok(false);
    }
    Err(format!(
        "Target directory holds a {} the seat form did not write, so its regime is its own - name its burc.env: {}",
        BPEE_TAMGA_RELPATH,
        root.display(), //
    ))
}

/// Empty a seat but for its own repository, every other entry standing in it
/// removed. Answers the report's line naming what was cleared.
fn zbpee_clear_target(root: &Path) -> Result<String, String> {
    let mut entries: Vec<PathBuf> = fs::read_dir(root)
        .map_err(|e| {
            format!(
                "Failed to read {}: {}",
                root.display(),
                e, //
            )
        })?
        .map(|entry| {
            entry.map(|e| e.path()).map_err(|e| {
                format!(
                    "Dir entry error: {}",
                    e, //
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|path| {
            path.file_name()
                .is_none_or(|name| name != ZBPEE_SEAT_REPOSITORY)
        })
        .collect();
    if entries.is_empty() {
        return Ok("the target stood empty - a first lay, nothing to clear".to_string());
    }
    entries.sort();
    let mut names = Vec::new();
    for path in &entries {
        let kind = fs::symlink_metadata(path).map_err(|e| {
            format!(
                "Failed to read {}: {}",
                path.display(),
                e, //
            )
        })?;
        if kind.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        }
        .map_err(|e| {
            format!(
                "Failed to clear {}: {}",
                path.display(),
                e, //
            )
        })?;
        names.push(
            path.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default(),
        );
    }
    Ok(format!(
        "cleared the earlier lay whole: {}",
        names.join(", "), //
    ))
}

/// Lay one executable file.
fn zbpee_write_door(path: &Path, body: &str) -> Result<(), String> {
    bkprs_program_write(path, body).map_err(|e| {
        format!(
            "Failed to write {}: {}",
            path.display(),
            e, //
        )
    })
}

/// Lay one piece of precinct freight from its tools-relative source into its
/// target-relative seat, replacing it whole: what is laid is derived from the
/// parcel, never authored at the seat. A directory takes the source's files
/// and nothing else; a file is copied. A source standing nowhere — its kit not
/// taken — lays nothing. Answers whether anything was laid.
fn zbpee_lay_freight(
    root: &Path,
    tools_dir: &Path,
    freight: &bpee_Freight,
) -> Result<bool, String> {
    let from = tools_dir.join(&freight.from);
    let to = root.join(&freight.to);
    if from.is_dir() {
        if to.exists() {
            fs::remove_dir_all(&to).map_err(|e| {
                format!(
                    "Failed to clear {}: {}",
                    to.display(),
                    e, //
                )
            })?;
        }
        fs::create_dir_all(&to).map_err(|e| {
            format!(
                "Failed to create {}: {}",
                to.display(),
                e, //
            )
        })?;
        let entries = fs::read_dir(&from).map_err(|e| {
            format!(
                "Failed to read {}: {}",
                from.display(),
                e, //
            )
        })?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let landed = to.join(entry.file_name());
                bkprs_copy(&path, &landed).map_err(|e| {
                    format!(
                        "Failed to lay {} at {}: {}",
                        path.display(),
                        landed.display(),
                        e, //
                    )
                })?;
            }
        }
        return Ok(true);
    }
    if from.is_file() {
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                format!(
                    "Failed to create {}: {}",
                    parent.display(),
                    e, //
                )
            })?;
        }
        bkprs_copy(&from, &to).map_err(|e| {
            format!(
                "Failed to lay {} at {}: {}",
                from.display(),
                to.display(),
                e, //
            )
        })?;
        return Ok(true);
    }
    Ok(false)
}

/// Write the REGIME of a seat, past the kits' copy:
/// a launcher stub for each launcher the taken kits' doors name, the taken
/// kits' tabtarget exemplars, the station door, the precinct's roster
/// directories, leaving whatever entries stand in them, and the precinct's
/// freight laid over them from the taken kits. Answers the step line.
fn zbpee_write_regime(
    root: &Path,
    tools_dir: &Path,
    moorings: &zbpee_Moorings,
    taken: &[String],
    precinct_rosters: &[PathBuf],
    precinct_freight: &[bpee_Freight],
) -> Result<String, String> {
    let declared = zbpee_assignments(ZBPEE_TEMPLATE_BURC);
    let tt_name = declared.get("BURC_TABTARGET_DIR").ok_or_else(|| {
        "the starter kit's burc.env template declares no BURC_TABTARGET_DIR".to_string()
    })?;
    let tt = root.join(tt_name);
    fs::create_dir_all(&tt).map_err(|e| {
        format!(
            "Failed to create {}: {}",
            tt.display(),
            e, //
        )
    })?;

    let mut launchers: BTreeSet<String> = BTreeSet::new();
    let mut doors = 0u32;
    for kit in taken {
        let Ok(entries) = fs::read_dir(tools_dir.join(kit).join(ZBPEE_EXEMPLAR_DIR)) else {
            continue;
        };
        for entry in entries.flatten() {
            let body = fs::read_to_string(entry.path()).map_err(|e| {
                format!(
                    "Failed to read the exemplar {}: {}",
                    entry.path().display(),
                    e, //
                )
            })?;
            for line in body.lines() {
                if let Some(configuration) =
                    line.trim_start().strip_prefix(ZBPEE_CONFIGURATION_OPEN)
                {
                    let name = configuration.split([' ', '\'']).next().unwrap_or_default();
                    if !name.is_empty() {
                        launchers.insert(name.to_string());
                    }
                }
            }
            zbpee_write_door(&tt.join(entry.file_name()), &body)?;
            doors += 1;
        }
    }

    let stubs = root.join(&moorings.dirname).join(&moorings.launchers);
    fs::create_dir_all(&stubs).map_err(|e| {
        format!(
            "Failed to create {}: {}",
            stubs.display(),
            e, //
        )
    })?;
    for launcher in &launchers {
        let workbench = launcher.strip_prefix("launcher.").ok_or_else(|| {
            format!(
                "a door names the launcher '{}', which wears no 'launcher.' head",
                launcher, //
            )
        })?;
        let kit = taken
            .iter()
            .find(|k| tools_dir.join(k).join(workbench).is_file())
            .ok_or_else(|| {
                format!(
                    "a door names the launcher '{}', whose workbench no taken kit carries",
                    launcher, //
                )
            })?;
        let lines = zbpee_launcher_lines(ZBPEE_TEMPLATE_LAUNCHER).ok_or_else(|| {
            "the starter kit's launcher template carries no blank line ending its header"
                .to_string()
        })?;
        let mut stub = lines.join("\n");
        stub.push('\n');
        zbpee_write_door(
            &stubs.join(launcher),
            &stub.replace(
                ZBPEE_NEEDLE_WORKBENCH,
                &format!(
                    "{}/{}",
                    kit,
                    workbench, //
                ),
            ),
        )?;
    }

    zbpee_write_door(
        &tt.join(ZBPEE_STATION_DOOR),
        &ZBPEE_TEMPLATE_STATION.replace(ZBPEE_NEEDLE_MOORINGS, &moorings.dirname),
    )?;

    for roster in precinct_rosters {
        let dir = root.join(roster);
        fs::create_dir_all(&dir).map_err(|e| {
            format!(
                "Failed to stand the roster {}: {}",
                dir.display(),
                e, //
            )
        })?;
    }
    let mut freight_laid = 0u32;
    for freight in precinct_freight {
        if zbpee_lay_freight(root, tools_dir, freight)? {
            freight_laid += 1;
        }
    }

    Ok(format!(
        "regime written: {} launcher stub(s), {} door(s), the station door, the precinct's rosters and {} piece(s) of precinct freight",
        launchers.len(),
        doors,
        freight_laid, //
    ))
}

// =============================================================================
// Public Types
// =============================================================================

/// The signets a kit's Claude assets wear after its cipher: a command is
/// `{cipher}{command}*.md` and a hook `{cipher}{hook}*.md`. Handed in by the
/// caller, whose registry is their one home.
#[derive(Debug, Clone, PartialEq)]
pub struct bpee_Signets {
    pub command: String,
    pub hook: String,
}

/// The one kit whose taking calls for a canonical binary, and the binary's
/// names: the canonical (unsuffixed) name every consumer surface invokes, and
/// the prefix its platform-suffixed siblings share. No target retains a
/// platform-suffixed copy after install. Handed in by the caller, whose binary
/// it is; a caller naming none emplaces and removes no binary.
#[derive(Debug, Clone, PartialEq)]
pub struct bpee_Binary {
    pub kit: String,
    pub canonical: String,
    pub suffixed_prefix: String,
}

/// The forge's behest to the install: what only the binary that cut the parcel
/// knows, composed for the one target the parcel is cut for, written into the
/// parcel as `BPEE_BEHEST_FILE` by the release and read back by the install and
/// the vacate.
#[derive(Debug, Clone, PartialEq)]
pub struct bpee_Behest {
    /// The veiled precinct's roster directories, target-relative, stood in a
    /// seat; empty where the target takes no kit whose seat holds a precinct
    pub precinct_rosters: Vec<PathBuf>,
    /// What a taken kit carries for the precinct, laid over the rosters in a
    /// seat; empty where the target takes no kit carrying any
    pub precinct_freight: Vec<bpee_Freight>,
    /// The signets of the Claude assets the install routes and the vacate
    /// removes
    pub signets: bpee_Signets,
    /// The canonical binary a taken kit carries, where the target takes the
    /// binary's kit; none otherwise
    pub binary: Option<bpee_Binary>,
    /// The endonym a seat's declaration states, where the target takes the kit
    /// whose seat it is; none otherwise, a seat laid under none being refused,
    /// and never empty
    pub seat_endonym: Option<String>,
}

/// Arguments for emplace operation, composed from a parcel and its behest.
#[derive(Debug)]
pub(crate) struct bpee_EmplaceArgs {
    /// Path to extracted parcel directory
    pub parcel_dir: PathBuf,
    /// Path to target repo's burc.env file, or the directory of a seat
    pub burc_path: PathBuf,
    /// The veiled precinct's roster directories, target-relative, stood in a
    /// seat. Handed in by the binary that links the
    /// dispatch, whose names they are.
    pub precinct_rosters: Vec<PathBuf>,
    /// What a taken kit carries for the precinct, laid over the rosters in a
    /// seat. Handed in beside the rosters, by the same
    /// binary, for the same reason.
    pub precinct_freight: Vec<bpee_Freight>,
    /// The signets of the Claude assets the install routes
    pub signets: bpee_Signets,
    /// The canonical binary a taken kit carries, where the caller has one
    pub binary: Option<bpee_Binary>,
    /// The endonym a seat's declaration states. Handed in by the binary that
    /// links the dispatch, whose seat it is; a seat laid with none named is
    /// refused, and a repository target's declaration is its owner's, so the
    /// repository form never reads it.
    pub seat_endonym: Option<String>,
}

/// One piece of precinct freight: a file or a directory of files a taken kit
/// carries, by its path beneath the tools directory, and the target-relative
/// path the install lays it at.
#[derive(Debug, Clone, PartialEq)]
pub struct bpee_Freight {
    pub from: PathBuf,
    pub to: PathBuf,
}

/// Result of emplace operation.
#[derive(Debug)]
pub struct bpee_EmplaceResult {
    /// Release number from the parcel's record
    pub number: String,
    /// Kits that were installed
    pub kits_installed: Vec<String>,
    /// Total files copied
    pub files_copied: u32,
    /// Commands routed to .claude/commands/
    pub commands_routed: u32,
    /// Hooks routed to .claude/hooks/
    pub hooks_routed: u32,
    /// Platform whose binary was emplaced at the canonical path, or None
    /// when the caller's binary kit was not taken
    pub canonical_platform: Option<String>,
    /// Release number the target's taken record named before this install, or
    /// None on a first install
    pub prior_number: Option<String>,
    /// The report of what the consumer's own files now owe, one line apiece,
    /// derived from the replaced and laid-down kit files at install time
    pub owed: Vec<String>,
    /// The regime step's line, where the target is a seat; None
    /// for a repository target, whose regime is its owner's
    pub regime: Option<String>,
    /// The commit the seat form made in the seat's own repository; None for a
    /// repository target, whose delta its caller seals
    pub commit: Option<String>,
}

/// Parsed BURC environment.
#[derive(Debug)]
pub struct bpee_BurcEnv {
    /// BURC_TOOLS_DIR - where kit directories go
    pub tools_dir: PathBuf,
    /// BURC_PROJECT_ROOT - project root for .claude/, CLAUDE.md, the record directory
    pub project_root: PathBuf,
    /// BURC_MANAGED_KITS - the kits this target takes from any parcel
    pub managed_kits: Vec<String>,
}

/// Arguments for vacate operation, composed from a parcel's behest.
#[derive(Debug)]
pub(crate) struct bpee_VacateArgs {
    /// Path to target repo's burc.env file
    pub burc_path: PathBuf,
    /// The signets of the Claude assets the vacate removes
    pub signets: bpee_Signets,
    /// The canonical binary the vacate removes, where the caller has one
    pub binary: Option<bpee_Binary>,
}

/// Result of vacate operation.
#[derive(Debug)]
pub struct bpee_VacateResult {
    /// Kits unlinked from management (directories preserved)
    pub kits_removed: Vec<String>,
    /// Total binaries deleted, canonical and suffixed alike
    pub files_deleted: u32,
    /// Each binary deleted, for the caller to report
    pub removed: Vec<PathBuf>,
    /// Commands removed from .claude/commands/
    pub commands_removed: u32,
    /// Hooks removed from .claude/hooks/
    pub hooks_removed: u32,
}

// =============================================================================
// Public Functions
// =============================================================================

/// The behest as the release writes it: every member present, the binary and
/// the name `null` where the target takes no kit carrying them, and each path
/// spelled as it is held.
pub fn bpee_behest_json(behest: &bpee_Behest) -> Result<serde_json::Value, String> {
    let path = |p: &Path| {
        p.to_str()
            .map(|s| serde_json::Value::String(s.to_string()))
            .ok_or_else(|| {
                format!(
                    "The behest holds a path that is no text: {}",
                    p.display(), //
                )
            })
    };
    let rosters = behest
        .precinct_rosters
        .iter()
        .map(|r| path(r))
        .collect::<Result<Vec<_>, _>>()?;
    let freight = behest
        .precinct_freight
        .iter()
        .map(|f| Ok(serde_json::json!({ ZBPEE_BEHEST_FROM: path(&f.from)?, ZBPEE_BEHEST_TO: path(&f.to)? })))
        .collect::<Result<Vec<_>, String>>()?;
    let binary = match &behest.binary {
        Some(b) => serde_json::json!({
            ZBPEE_BEHEST_KIT: b.kit,
            ZBPEE_BEHEST_CANONICAL: b.canonical,
            ZBPEE_BEHEST_PREFIX: b.suffixed_prefix,
        }),
        None => serde_json::Value::Null,
    };
    Ok(serde_json::json!({
        ZBPEE_BEHEST_DIRECTORIES: rosters,
        ZBPEE_BEHEST_FREIGHT: freight,
        ZBPEE_BEHEST_SUFFIXES: {
            ZBPEE_BEHEST_COMMAND: behest.signets.command,
            ZBPEE_BEHEST_HOOK: behest.signets.hook,
        },
        ZBPEE_BEHEST_BINARY: binary,
        ZBPEE_BEHEST_NAME: behest.seat_endonym,
    }))
}

/// Read the behest a parcel carries at its root. A parcel carrying none is
/// refused naming the file, and so is a behest missing any member: nothing is
/// defaulted, the forge that cut the parcel being the only one that knew. A
/// member the target does not take is read as the forge wrote it, `null` or an
/// empty list, and a name spelled empty is refused, `null` being the one
/// spelling of a behest naming no seat.
fn zbpee_read_behest(parcel_dir: &Path) -> Result<bpee_Behest, String> {
    let file = parcel_dir.join(BPEE_BEHEST_FILE);
    if !file.is_file() {
        return Err(format!(
            "The parcel carries no {} at its root ({}): the release writes the forge's behest there beside {}, and an install or a vacate reads nothing in its place",
            BPEE_BEHEST_FILE,
            file.display(),
            BPEE_TAMGA_FILE, //
        ));
    }
    let body = fs::read_to_string(&file).map_err(|e| {
        format!(
            "Failed to read {}: {}",
            file.display(),
            e, //
        )
    })?;
    let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
        format!(
            "{} does not parse: {}",
            file.display(),
            e, //
        )
    })?;
    let member = |at: &serde_json::Value, key: &str| -> Result<serde_json::Value, String> {
        at.get(key).cloned().ok_or_else(|| {
            format!(
                "{} carries no member '{}'",
                file.display(),
                key, //
            )
        })
    };
    let text = |at: &serde_json::Value, key: &str| -> Result<String, String> {
        member(at, key)?.as_str().map(String::from).ok_or_else(|| {
            format!(
                "{} member '{}' is not text",
                file.display(),
                key, //
            )
        })
    };
    let list = |key: &str| -> Result<Vec<serde_json::Value>, String> {
        member(&json, key)?.as_array().cloned().ok_or_else(|| {
            format!(
                "{} member '{}' is not a list",
                file.display(),
                key, //
            )
        })
    };

    let precinct_rosters = list(ZBPEE_BEHEST_DIRECTORIES)?
        .iter()
        .map(|r| {
            r.as_str().map(PathBuf::from).ok_or_else(|| {
                format!(
                    "{} lists a roster that is not text",
                    file.display(), //
                )
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let precinct_freight = list(ZBPEE_BEHEST_FREIGHT)?
        .iter()
        .map(|f| {
            Ok(bpee_Freight {
                from: PathBuf::from(text(f, ZBPEE_BEHEST_FROM)?),
                to: PathBuf::from(text(f, ZBPEE_BEHEST_TO)?),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let signets = member(&json, ZBPEE_BEHEST_SUFFIXES)?;
    let signets = bpee_Signets {
        command: text(&signets, ZBPEE_BEHEST_COMMAND)?,
        hook: text(&signets, ZBPEE_BEHEST_HOOK)?,
    };
    let binary = match member(&json, ZBPEE_BEHEST_BINARY)? {
        serde_json::Value::Null => None,
        b => Some(bpee_Binary {
            kit: text(&b, ZBPEE_BEHEST_KIT)?,
            canonical: text(&b, ZBPEE_BEHEST_CANONICAL)?,
            suffixed_prefix: text(&b, ZBPEE_BEHEST_PREFIX)?,
        }),
    };
    let seat_endonym = match member(&json, ZBPEE_BEHEST_NAME)? {
        serde_json::Value::Null => None,
        _ => {
            let name = text(&json, ZBPEE_BEHEST_NAME)?;
            if name.is_empty() {
                return Err(format!(
                    "{} member '{}' is empty: a behest naming no seat spells it null",
                    file.display(),
                    ZBPEE_BEHEST_NAME, //
                ));
            }
            Some(name)
        }
    };
    Ok(bpee_Behest {
        precinct_rosters,
        precinct_freight,
        signets,
        binary,
        seat_endonym,
    })
}

/// Install an extracted parcel into the target its `burc.env` names, or into
/// the seat its directory names, under the behest the parcel carries.
pub fn bpee_emplace_parcel(
    parcel_dir: &Path,
    burc_path: &Path,
) -> Result<bpee_EmplaceResult, String> {
    let behest = zbpee_read_behest(parcel_dir)?;
    bpee_emplace(&bpee_EmplaceArgs {
        parcel_dir: parcel_dir.to_path_buf(),
        burc_path: burc_path.to_path_buf(),
        precinct_rosters: behest.precinct_rosters,
        precinct_freight: behest.precinct_freight,
        signets: behest.signets,
        binary: behest.binary,
        seat_endonym: behest.seat_endonym,
    })
}

/// Remove the Claude Code integration from the target its `burc.env` names,
/// under the signets and the binary the parcel's behest names.
pub fn bpee_vacate_parcel(
    parcel_dir: &Path,
    burc_path: &Path,
) -> Result<bpee_VacateResult, String> {
    let behest = zbpee_read_behest(parcel_dir)?;
    bpee_vacate(&bpee_VacateArgs {
        burc_path: burc_path.to_path_buf(),
        signets: behest.signets,
        binary: behest.binary,
    })
}

/// Install kit assets from parcel to target repo.
/// Nuclear install: deletes existing installation before copying fresh.
///
/// # Arguments
/// * `args` - Emplace arguments (parcel dir and burc.env path)
///
/// # Returns
/// EmplaceResult with installation summary, or error message
pub(crate) fn bpee_emplace(args: &bpee_EmplaceArgs) -> Result<bpee_EmplaceResult, String> {
    // 3, read first. Record identity: a seat is declared from the parcel's own
    // kit list.
    let tamga_path = args.parcel_dir.join(BPEE_TAMGA_FILE);
    let (number, kit_ids) = zbpee_read_tamga(&tamga_path, &ZBPEE_CURRENT)?;

    // 0. Resolve the target. A directory names a seat, which is emptied but
    // for its own repository and laid whole on every run behind the record
    // check; a directory whose record the seat form did not write is refused,
    // its regime being its owner's. A burc.env names a repository target,
    // gated below as ever. Everything the clear would destroy is read ahead of
    // it — the parcel's presence and the report of what the consumer owes —
    // and the repository, on a first lay, and the declaration after it.
    let (burc_path, moorings, cleared) = if args.burc_path.is_dir() {
        let root = &args.burc_path;
        let Some(endonym) = args.seat_endonym.as_deref() else {
            return Err(format!(
                "No BURC_ENDONYM value was named for the seat {}: a seat's declaration states the name its parcel's behest gives it, and this one gives none",
                root.display(), //
            ));
        };
        let first_lay = zbpee_require_seat_tamga_or_empty(root)?;
        let kits_dir = args.parcel_dir.join("kits");
        for kit_id in &kit_ids {
            if !kits_dir.join(kit_id).exists() {
                return Err(format!(
                    "Kit not found in parcel: {}",
                    kit_id, //
                ));
            }
        }
        let moorings = zbpee_read_moorings(&kits_dir, &kit_ids)?;
        let (prior_number, mut owed) = zbpee_owed_report(
            root,
            &root.join(zbpee_template_tools_dir()?),
            &kits_dir,
            &kit_ids,
        )?;
        owed.insert(0, zbpee_clear_target(root)?);
        if first_lay {
            zbpee_init_seat(root)?;
        }
        (
            zbpee_write_declaration(root, &moorings, &kit_ids, endonym)?,
            Some(moorings),
            Some((prior_number, owed)),
        )
    } else {
        (args.burc_path.clone(), None, None)
    };

    // 1. Parse BURC (resolves paths relative to burc.env location)
    let burc = bpee_parse_burc(&burc_path)?;

    // 2. Validate git state — a delta someone will seal. A seat has none, its
    // repository being the install's own, so neither gate reads there.
    if moorings.is_none() {
        zbpee_check_is_git_repo(&burc.project_root)?;
        zbpee_check_git_clean(&burc.project_root, "install")?;
    }

    // 4. Elect the taken set: every kit the parcel carries that the target
    // declares. The target's declaration is the whole election — a carried
    // kit it does not declare stays in the parcel, untouched and unreported,
    // and a declared kit the parcel does not carry stands byte-untouched in
    // the target. Only an empty intersection refuses: an install that took
    // nothing would have written a record over a target it did not serve.
    let taken: Vec<String> = kit_ids
        .iter()
        .filter(|k| burc.managed_kits.contains(k))
        .cloned()
        .collect();

    if taken.is_empty() {
        return Err(format!(
            "Parcel [{}] carries none of the kits target declares [{}]",
            kit_ids.join(","),
            burc.managed_kits.join(","), //
        ));
    }

    // 4b. Validate presence: every taken kit must actually stand in the
    // parcel. Checked here, ahead of the nuclear cleanup below —
    // step 7's copy loop already refused a missing kit, but only after
    // step 5 had already deleted the target's kit directories.
    let kits_dir = args.parcel_dir.join("kits");
    for kit_id in &taken {
        if !kits_dir.join(kit_id).exists() {
            return Err(format!(
                "Kit not found in parcel: {}",
                kit_id, //
            ));
        }
    }

    // 4c. Read the report of what the consumer's own files owe, while the
    // edition this install replaces still stands: step 5 deletes it. A seat
    // had it read ahead of its clear.
    //
    // 4d. Remove the kits the standing record names that the target no longer
    // declares, their lines leading the report. Ahead of step 5, which deletes
    // the record this reads.
    let (prior_number, owed) = match cleared {
        Some(read) => read,
        None => {
            let (prior_number, owed) =
                zbpee_owed_report(&burc.project_root, &burc.tools_dir, &kits_dir, &taken)?;
            let mut lines = zbpee_remove_dropped_kits(
                &burc.project_root,
                &burc.tools_dir,
                &burc.managed_kits,
                &args.signets,
            )?;
            lines.extend(owed);
            (prior_number, lines)
        }
    };

    // 5. Nuclear cleanup - delete the record directory, the elder one with it,
    // and the kit directories. The elder goes here and is never written again.
    for dir in [BPEE_TAMGA_DIR, BPEE_ELDER_DIR] {
        let standing = burc.project_root.join(dir);
        if standing.exists() {
            fs::remove_dir_all(&standing).map_err(|e| {
                format!(
                    "Failed to remove {} directory: {}",
                    dir,
                    e, //
                )
            })?;
        }
    }
    let tamga_dir = burc.project_root.join(BPEE_TAMGA_DIR);

    for kit_id in &taken {
        let kit_dir = burc.tools_dir.join(kit_id);
        if kit_dir.exists() {
            fs::remove_dir_all(&kit_dir).map_err(|e| {
                format!(
                    "Failed to remove kit directory {}: {}",
                    kit_id,
                    e, //
                )
            })?;
        }
    }

    // 6. Create the record directory and write the record of the taken set and the
    // form that laid the target. The uninstall walks the list, so it must name
    // what the target took, never what the parcel carried; the seat's next
    // record check reads the form.
    fs::create_dir_all(&tamga_dir).map_err(|e| {
        format!(
            "Failed to create {} directory: {}",
            BPEE_TAMGA_DIR,
            e, //
        )
    })?;

    let tamga_dest = tamga_dir.join(BPEE_TAMGA_FILE);
    zbpee_write_taken_tamga(&tamga_path, &tamga_dest, &taken, moorings.is_some())?;

    // 7. Copy kit assets and route special files
    let mut total_files = 0u32;
    let mut commands_routed = 0u32;
    let mut hooks_routed = 0u32;

    let kits_dir = args.parcel_dir.join("kits");
    for kit_id in &taken {
        let kit_source = kits_dir.join(kit_id);
        let kit_dest = burc.tools_dir.join(kit_id);

        if !kit_source.exists() {
            return Err(format!(
                "Kit not found in parcel: {}",
                kit_id, //
            ));
        }

        let (files, cmds, hks) = zbpee_copy_kit(
            &kit_source,
            &kit_dest,
            &burc.project_root,
            kit_id,
            &args.signets,
        )?;

        total_files += files;
        commands_routed += cmds;
        hooks_routed += hks;
    }

    // 7b. Route Claude config assets from parcel claude/* to target .claude/*,
    // only those whose cipher is a taken kit's
    let (parcel_cmds, parcel_hks) =
        zbpee_route_claude_assets(&args.parcel_dir, &burc.project_root, &taken, &args.signets)?;
    total_files += parcel_cmds + parcel_hks;
    commands_routed += parcel_cmds;
    hooks_routed += parcel_hks;

    // 8. Emplace the canonical binary — only when the caller's binary kit is
    // taken.
    //
    // The platform binary is staged into every parcel whatever the imprint,
    // so the parcel is self-executing through its own bootstrap; but a target
    // that does not take the binary's kit gets no kit directory and no binary,
    // and an unconditional emplace here would create both. Step 7 has already
    // copied the kit verbatim when it IS taken, so the suffixed sibling stands
    // in the target bin dir at this point and the purge below is what removes
    // it.
    let canonical_platform = match &args.binary {
        Some(binary) if taken.iter().any(|k| *k == binary.kit) => Some(
            zbpee_emplace_canonical_binary(&args.parcel_dir, &burc.tools_dir, binary)?,
        ),
        _ => None,
    };

    // 9. The regime of a seat, on every run, there being no owner to keep one.
    let regime = match &moorings {
        Some(moorings) => Some(zbpee_write_regime(
            &burc.project_root,
            &burc.tools_dir,
            moorings,
            &taken,
            &args.precinct_rosters,
            &args.precinct_freight,
        )?),
        None => None,
    };

    // 10. A seat's lay committed whole in its own repository, its last act.
    let commit = match &moorings {
        Some(_) => Some(zbpee_commit_seat(&burc.project_root, &number)?),
        None => None,
    };

    Ok(bpee_EmplaceResult {
        number,
        kits_installed: taken,
        files_copied: total_files,
        commands_routed,
        hooks_routed,
        canonical_platform,
        prior_number,
        owed,
        regime,
        commit,
    })
}

/// Remove kit assets from target repo.
///
/// # Arguments
/// * `args` - Vacate arguments (burc.env path)
///
/// # Returns
/// VacateResult with removal summary, or error message
pub(crate) fn bpee_vacate(args: &bpee_VacateArgs) -> Result<bpee_VacateResult, String> {
    // 1. Parse BURC
    let burc = bpee_parse_burc(&args.burc_path)?;

    // 2. Validate git state
    zbpee_check_git_clean(&burc.project_root, "uninstall")?;

    // 3. Read the standing record, at either seat
    let Some(spelling) = zbpee_standing_spelling(&burc.project_root) else {
        return Err(format!(
            "No VVK installation found (neither {} nor {} stands)",
            BPEE_TAMGA_RELPATH,
            BPEE_ELDER_RELPATH, //
        ));
    };
    let tamga_path = burc.project_root.join(spelling.seat);
    let (_number, kit_ids) = zbpee_read_tamga(&tamga_path, spelling)?;

    // 4. Remove commands and hooks
    let mut total_files = 0u32;
    let mut commands_removed = 0u32;
    let mut hooks_removed = 0u32;

    let commands_dir = burc.project_root.join(".claude").join("commands");
    let hooks_dir = burc.project_root.join(".claude").join("hooks");

    for kit_id in &kit_ids {
        // Get cipher from kit_id (e.g., "jjk" -> "jj")
        let cipher = &kit_id[..kit_id.len().saturating_sub(1)];

        // Remove commands matching {cipher}c-*.md
        if commands_dir.exists() {
            commands_removed +=
                zbpee_remove_matching_files(&commands_dir, cipher, &args.signets, true)?;
        }

        // Remove hooks matching {cipher}h-*.md
        if hooks_dir.exists() {
            hooks_removed += zbpee_remove_matching_files(&hooks_dir, cipher, &args.signets, false)?;
        }
    }

    // 5. Delete the caller's canonical binary from its kit's bin directory
    // Kit directories are intentionally preserved (open-source utilities)
    let mut removed = Vec::new();
    let binary_dir = args
        .binary
        .as_ref()
        .map(|b| (b, burc.tools_dir.join(&b.kit).join("bin")));
    if let Some((binary, binary_dir)) = binary_dir.filter(|(_, dir)| dir.exists()) {
        let entries = fs::read_dir(&binary_dir).map_err(|e| {
            format!(
                "Failed to read {} bin dir: {}",
                binary.kit,
                e, //
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                format!(
                    "Dir entry error: {}",
                    e, //
                )
            })?;
            let path = entry.path();
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            // Remove the canonical binary and any platform-suffixed sibling
            // (the canonical name, then each `-{platform}` copy)
            if file_name.starts_with(binary.canonical.as_str()) {
                fs::remove_file(&path).map_err(|e| {
                    format!(
                        "Failed to remove {} binary {}: {}",
                        binary.canonical,
                        path.display(),
                        e, //
                    )
                })?;
                total_files += 1;
                removed.push(path);
            }
        }
    }

    // 6. Delete the record and its directory
    fs::remove_file(&tamga_path).map_err(|e| {
        format!(
            "Failed to remove record: {}",
            e, //
        )
    })?;

    for dir in [BPEE_TAMGA_DIR, BPEE_ELDER_DIR] {
        let tamga_dir = burc.project_root.join(dir);
        if tamga_dir.exists() {
            let _ = fs::remove_dir(&tamga_dir);
        }
    }

    Ok(bpee_VacateResult {
        kits_removed: kit_ids,
        files_deleted: total_files,
        removed,
        commands_removed,
        hooks_removed,
    })
}

// =============================================================================
// The Owed Report
// =============================================================================

/// The line closing every report that compared anything: the changes a
/// consumer's files can depend on that no reading of two editions reaches.
const BPEE_OWED_UNDERIVABLE: &str = "cannot derive: changed function arguments or behaviour, \
and changes to templates you copied out of a kit - read the kits' own guidance";

/// How many newly defined functions a vanished function's line names as
/// candidate successors before it counts the rest: a module rewritten whole
/// would otherwise repeat its every new name on every vanished one's line.
const BPEE_OWED_SUCCESSORS_SHOWN: usize = 3;

/// One variable a kit's regime module enrolls.
struct zbpee_Enrolled {
    /// The regime the module enrolled it under (the last `buv_regime_enroll`
    /// above it)
    regime: String,
    /// Whether the regime refuses its absence: a length floor of one or more,
    /// or a kind that carries no floor at all
    required: bool,
    /// Whether it stands in a group behind a `buv_gate_enroll`, so its
    /// requirement binds only where that gate opens
    gated: bool,
    /// The kit-relative module that enrolls it
    module: String,
}

/// What one edition of a kit offers the consumer's own files, read off its
/// bash modules and nothing else: the regime variables it enrolls, the public
/// functions it defines, and the module files it carries.
///
/// THE READING IS LEXICAL, AND THAT IS ITS WHOLE CONTRACT. It reads the lines
/// the kit's own conventions make legible — an enrollment call at the head of
/// a line, a function defined at column zero, a `z` prefix marking a private
/// name — and claims nothing beneath them. What it cannot read, the report
/// says it cannot.
#[derive(Default)]
struct zbpee_Edition {
    variables: BTreeMap<String, zbpee_Enrolled>,
    /// Public function name to the kit-relative module defining it
    functions: BTreeMap<String, String>,
    modules: BTreeSet<String>,
}

/// The name a line defines as a function, when it opens a definition at
/// column zero in either bash spelling (`name() {` or `function name() {`).
fn zbpee_function_defined(line: &str) -> Option<String> {
    let end = line.find("()")?;
    let head = &line[..end];
    let name = head.strip_prefix("function ").unwrap_or(head).trim_end();
    let mut chars = name.chars();
    let first = chars.next()?;
    if !(first.is_ascii_alphabetic() || first == '_') {
        return None;
    }
    if !chars.all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    Some(name.to_string())
}

/// Read one edition of a kit standing at `kit_dir`.
fn zbpee_read_edition(kit_dir: &Path) -> Result<zbpee_Edition, String> {
    let mut edition = zbpee_Edition::default();

    for path in zbpee_walk_dir(kit_dir)? {
        if path.extension().and_then(|e| e.to_str()) != Some("sh") {
            continue;
        }
        let module = path
            .strip_prefix(kit_dir)
            .map_err(|e| {
                format!(
                    "Path strip failed: {}",
                    e, //
                )
            })?
            .to_string_lossy()
            .into_owned();
        let bytes = fs::read(&path).map_err(|e| {
            format!(
                "Failed to read {}: {}",
                path.display(),
                e, //
            )
        })?;
        let body = String::from_utf8_lossy(&bytes);

        let mut regime = String::new();
        let mut gated = false;
        for line in body.lines() {
            if let Some(name) = zbpee_function_defined(line) {
                if !name.starts_with('z') {
                    edition.functions.insert(name, module.clone());
                }
                continue;
            }

            let tokens: Vec<&str> = line.split_whitespace().collect();
            let Some(verb) = tokens.first() else {
                continue;
            };
            if !(verb.starts_with("buv_") && verb.ends_with("_enroll")) {
                continue;
            }
            match *verb {
                "buv_regime_enroll" => {
                    regime = tokens.get(1).copied().unwrap_or_default().to_string();
                    gated = false;
                }
                "buv_group_enroll" => gated = false,
                "buv_gate_enroll" => gated = true,
                "buv_reset_enroll" => {}
                _ => {
                    let Some(name) = tokens.get(1) else {
                        continue;
                    };
                    let required = match tokens.get(2).and_then(|t| t.parse::<u64>().ok()) {
                        Some(floor) => floor >= 1,
                        None => true,
                    };
                    edition.variables.insert(
                        name.to_string(),
                        zbpee_Enrolled {
                            regime: regime.clone(),
                            required,
                            gated,
                            module: module.clone(),
                        },
                    );
                }
            }
        }

        edition.modules.insert(module);
    }

    Ok(edition)
}

/// How alike two function names read: the length of the prefix and of the
/// suffix they share, so a rename that kept either end ranks its successor
/// first among a module's new names. An ordering only, never a pairing.
fn zbpee_kinship(vanished: &str, fresh: &str) -> usize {
    let prefix = vanished
        .bytes()
        .zip(fresh.bytes())
        .take_while(|(a, b)| a == b)
        .count();
    let suffix = vanished
        .bytes()
        .rev()
        .zip(fresh.bytes().rev())
        .take_while(|(a, b)| a == b)
        .count();
    prefix + suffix
}

/// The requirement a variable's enrollment states, worded for the report.
fn zbpee_requirement(enrolled: &zbpee_Enrolled) -> &'static str {
    match (enrolled.required, enrolled.gated) {
        (true, false) => "required",
        (true, true) => "required where its group's gate opens",
        (false, _) => "optional",
    }
}

/// The lines naming what moving one kit from `prior` to `laid` asks of the
/// consumer's own files.
fn zbpee_compare_editions(kit: &str, prior: &zbpee_Edition, laid: &zbpee_Edition) -> Vec<String> {
    let mut lines = Vec::new();

    for (name, now) in &laid.variables {
        match prior.variables.get(name) {
            None if now.required => lines.push(format!(
                "{}: {} - the {} regime now enrolls it as {}; set it wherever you declare {} variables ({})",
                kit, name, now.regime, zbpee_requirement(now), now.regime, now.module, //
            )),
            None => lines.push(format!(
                "{}: {} - the {} regime now enrolls it as optional; nothing owed ({})",
                kit, name, now.regime, now.module, //
            )),
            Some(was) if now.required && !was.required => lines.push(format!(
                "{}: {} - the {} regime enrolled it as optional and now as {}; set it wherever you declare {} variables ({})",
                kit, name, now.regime, zbpee_requirement(now), now.regime, now.module, //
            )),
            Some(_) => {}
        }
    }

    for (name, was) in &prior.variables {
        if !laid.variables.contains_key(name) {
            lines.push(format!(
                "{}: remove {} wherever you set it - the {} regime no longer enrolls it, \
and refuses a variable of its prefix it does not enroll ({})",
                kit,
                name,
                was.regime,
                was.module, //
            ));
        }
    }

    // A function whose whole module is gone is owed by the module's line
    // below, which names every call into it at once.
    for (name, module) in &prior.functions {
        if laid.functions.contains_key(name) || !laid.modules.contains(module) {
            continue;
        }
        let mut successors: Vec<&str> = laid
            .functions
            .iter()
            .filter(|(fresh, home)| *home == module && !prior.functions.contains_key(*fresh))
            .map(|(fresh, _)| fresh.as_str())
            .collect();
        successors.sort_by_key(|fresh| std::cmp::Reverse(zbpee_kinship(name, fresh)));
        let shown = successors.len().min(BPEE_OWED_SUCCESSORS_SHOWN);
        let mut named = successors[..shown].join(", ");
        if successors.len() > shown {
            named.push_str(&format!(
                " and {} more",
                successors.len() - shown, //
            ));
        }
        if successors.is_empty() {
            lines.push(format!(
                "{}: replace every call to {} - {} no longer defines it",
                kit,
                name,
                module, //
            ));
        } else {
            lines.push(format!(
                "{}: replace every call to {} - {} no longer defines it, and newly defines {} \
(a rename cannot be derived; judge by eye)",
                kit,
                name,
                module,
                named, //
            ));
        }
    }

    for module in prior.modules.difference(&laid.modules) {
        lines.push(format!(
            "{}: stop sourcing {}, and replace every call into it - the kit no longer carries it",
            kit,
            module, //
        ));
    }

    for module in laid.modules.difference(&prior.modules) {
        lines.push(format!(
            "{}: {} is new to the kit - whether your own scripts must now source it \
cannot be derived",
            kit,
            module, //
        ));
    }

    if lines.is_empty() {
        lines.push(format!(
            "{}: no change your own files depend on could be derived between the two editions",
            kit, //
        ));
    }

    lines
}

/// The report of what this install asks of the consumer's own files, read
/// before the nuclear cleanup destroys the edition it replaces. Returns the
/// release number the target's taken record named, if one stood, and the lines.
///
/// THE PRIOR EDITION IS THE TAKEN RECORD'S, AND ITS FILES ARE THE KIT
/// DIRECTORIES STANDING IN THE TARGET. The record holds only a release number and
/// a kit set, so it says THAT an edition stood and which kits it laid down;
/// what that edition was is read off the files themselves. A target with no
/// taken record is a first install whatever directories stand in it, since
/// nothing says an install laid them.
fn zbpee_owed_report(
    project_root: &Path,
    tools_dir: &Path,
    parcel_kits_dir: &Path,
    taken: &[String],
) -> Result<(Option<String>, Vec<String>), String> {
    let Some(standing) = zbpee_standing(project_root)? else {
        return Ok((
            None,
            vec!["no prior edition stood - a first install, nothing to compare".to_string()],
        ));
    };

    let prior_number = standing
        .zbpee_member(|s| s.number)
        .and_then(|v| v.as_str())
        .map(String::from);
    let Some(prior_number) = prior_number else {
        return Ok((
            None,
            vec![format!(
                "cannot derive: the prior record ({}) does not parse, so no prior edition could be read",
                standing.spelling.seat, //
            )],
        ));
    };
    let prior_kits: Vec<String> = standing
        .zbpee_member(|s| s.kits)
        .and_then(|v| v.as_array())
        .map(|kits| {
            kits.iter()
                .filter_map(|v| v.as_str())
                .filter(|k| !k.is_empty())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    let mut lines = Vec::new();
    for kit in taken {
        let prior_dir = tools_dir.join(kit);
        if !prior_kits.contains(kit) || !prior_dir.is_dir() {
            lines.push(format!(
                "{}: no prior edition stood in this target - nothing to compare",
                kit, //
            ));
            continue;
        }
        let prior = zbpee_read_edition(&prior_dir)?;
        let laid = zbpee_read_edition(&parcel_kits_dir.join(kit))?;
        lines.extend(zbpee_compare_editions(kit, &prior, &laid));
    }
    lines.push(BPEE_OWED_UNDERIVABLE.to_string());

    Ok((Some(prior_number), lines))
}

/// Whether a record's kit entry names a plain directory, so joining it onto
/// the tools directory can reach nothing but a kit directory beneath it.
fn zbpee_is_plain_kit(kit: &str) -> bool {
    !kit.is_empty() && kit != "." && kit != ".." && !kit.contains('/') && !kit.contains('\\')
}

/// Remove every kit the standing record names that the target no longer
/// declares, with the commands and hooks its cipher matches. The record is the
/// whole bookkeeping: what it names is what installs laid, so a directory the
/// receiver authored under the tools directory is never reached, and a kit
/// still declared is left to the nuclear cleanup or, uncarried, untouched.
/// A record that does not read names nothing, so nothing is removed and the
/// line says so. Answers the report's lines, one per kit removed.
fn zbpee_remove_dropped_kits(
    project_root: &Path,
    tools_dir: &Path,
    declared: &[String],
    signets: &bpee_Signets,
) -> Result<Vec<String>, String> {
    let Some(record) = zbpee_standing(project_root)? else {
        return Ok(Vec::new());
    };
    let standing: Option<Vec<String>> = record.zbpee_member(|s| s.kits).and_then(|kits| {
        kits.as_array()?
            .iter()
            .map(|v| {
                v.as_str()
                    .filter(|k| zbpee_is_plain_kit(k))
                    .map(String::from)
            })
            .collect()
    });
    let Some(standing) = standing else {
        return Ok(vec![format!(
            "cannot remove: the prior record ({}) does not read, so no kit this target stopped declaring could be named - only the taken kits were replaced",
            record.spelling.seat, //
        )]);
    };

    let commands_dir = project_root.join(".claude").join("commands");
    let hooks_dir = project_root.join(".claude").join("hooks");
    let mut lines = Vec::new();
    for kit in standing.iter().filter(|k| !declared.contains(k)) {
        let kit_dir = tools_dir.join(kit);
        if kit_dir.exists() {
            fs::remove_dir_all(&kit_dir).map_err(|e| {
                format!(
                    "Failed to remove kit directory {}: {}",
                    kit,
                    e, //
                )
            })?;
        }
        let cipher = &kit[..kit.len().saturating_sub(1)];
        let commands = match commands_dir.exists() {
            true => zbpee_remove_matching_files(&commands_dir, cipher, signets, true)?,
            false => 0,
        };
        let hooks = match hooks_dir.exists() {
            true => zbpee_remove_matching_files(&hooks_dir, cipher, signets, false)?,
            false => 0,
        };
        lines.push(format!(
            "{}: no longer declared - removed its kit directory, {} command(s) and {} hook(s); your own launchers, doors and context lines over it stay yours to remove",
            kit, commands, hooks, //
        ));
    }
    Ok(lines)
}

// =============================================================================
// Internal Functions (zbpee_*)
// =============================================================================

/// Parse burc.env file to extract environment variables.
/// BURC_PROJECT_ROOT is the path from burc.env's location to the project root.
/// BURC_TOOLS_DIR is resolved relative to the resolved project_root (per BURC spec).
pub fn bpee_parse_burc(path: &Path) -> Result<bpee_BurcEnv, String> {
    if !path.exists() {
        return Err(format!(
            "burc.env not found: {}",
            path.display(), //
        ));
    }

    // Get the parent directory of burc.env for path resolution
    let burc_parent = path
        .parent()
        .ok_or_else(|| "burc.env path has no parent directory".to_string())?;

    let content = fs::read_to_string(path).map_err(|e| {
        format!(
            "Failed to read burc.env: {}",
            e, //
        )
    })?;

    let mut vars: HashMap<String, String> = HashMap::new();

    for line in content.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse KEY=VALUE (handle quoted values)
        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim();
            let mut value = line[eq_pos + 1..].trim();

            // Remove surrounding quotes if present
            if (value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\''))
            {
                value = &value[1..value.len() - 1];
            }

            vars.insert(key.to_string(), value.to_string());
        }
    }

    let tools_dir_str = vars
        .get("BURC_TOOLS_DIR")
        .ok_or_else(|| "burc.env missing required variable: BURC_TOOLS_DIR".to_string())?;

    let project_root_str = vars
        .get("BURC_PROJECT_ROOT")
        .ok_or_else(|| "burc.env missing required variable: BURC_PROJECT_ROOT".to_string())?;

    // BURC_PROJECT_ROOT is the path from burc.env's location to project root
    let project_root = burc_parent.join(project_root_str);
    let project_root = project_root.canonicalize().map_err(|e| {
        format!(
            "BURC_PROJECT_ROOT path invalid ({}): {}",
            project_root.display(),
            e, //
        )
    })?;

    // Resolve tools_dir relative to project_root (per BURC spec)
    let tools_dir = project_root.join(tools_dir_str);
    let tools_dir = tools_dir.canonicalize().map_err(|e| {
        format!(
            "BURC_TOOLS_DIR path invalid ({}): {}",
            tools_dir.display(),
            e, //
        )
    })?;

    // Parse BURC_MANAGED_KITS (comma-separated list)
    let managed_kits_str = vars
        .get("BURC_MANAGED_KITS")
        .ok_or_else(|| "burc.env missing required variable: BURC_MANAGED_KITS".to_string())?;

    let managed_kits: Vec<String> = managed_kits_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if managed_kits.is_empty() {
        return Err("BURC_MANAGED_KITS must specify at least one kit".to_string());
    }

    Ok(bpee_BurcEnv {
        tools_dir,
        project_root,
        managed_kits,
    })
}

/// Read a record and extract its release number and kit list, under the member keys of the
/// spelling its seat wears: a parcel's record is always the current spelling,
/// its own edition having written it.
fn zbpee_read_tamga(
    path: &Path,
    spelling: &zbpee_Spelling,
) -> Result<(String, Vec<String>), String> {
    if !path.exists() {
        return Err(format!(
            "Record not found: {}",
            path.display(), //
        ));
    }

    let content = fs::read_to_string(path).map_err(|e| {
        format!(
            "Failed to read record: {}",
            e, //
        )
    })?;

    let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
        format!(
            "Failed to parse record JSON: {}",
            e, //
        )
    })?;

    // Text, never a number — a number here is a producer bug.
    let number = json
        .get(spelling.number)
        .and_then(|v| v.as_str())
        .map(String::from)
        .ok_or_else(|| "Record missing its number".to_string())?;

    let kits = json
        .get(spelling.kits)
        .and_then(|v| v.as_array())
        .ok_or_else(|| "Record missing kits array".to_string())?;

    let kit_ids: Vec<String> = kits
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    if kit_ids.is_empty() {
        return Err("Record has empty kits array".to_string());
    }

    Ok((number, kit_ids))
}

/// Write the target's record: the parcel's record composed into the
/// installed copy by `bpee_installed_tamga_json`.
fn zbpee_write_taken_tamga(
    source: &Path,
    dest: &Path,
    taken: &[String],
    seat: bool,
) -> Result<(), String> {
    let content = fs::read_to_string(source).map_err(|e| {
        format!(
            "Failed to read record: {}",
            e, //
        )
    })?;

    let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| {
        format!(
            "Failed to parse record JSON: {}",
            e, //
        )
    })?;
    let json = bpee_installed_tamga_json(json, taken, seat)?;

    let body = serde_json::to_string_pretty(&json).map_err(|e| {
        format!(
            "Failed to serialize record JSON: {}",
            e, //
        )
    })?;
    fs::write(dest, body + "\n").map_err(|e| {
        format!(
            "Failed to write record: {}",
            e, //
        )
    })
}

/// The installed copy of a record: the parcel's record with its kit list
/// replaced by the taken set and the form that laid the target recorded —
/// `seat` true for the seat form, handed a directory, and false for the
/// repository form, handed a burc.env. The one composer of what an install
/// writes, public so a reader holding the target's store to its declared wire
/// composes the copy that stands there rather than the parcel's.
///
/// THE RECORD IS HANDLED AS A GENERIC JSON VALUE, never a typed struct, so a
/// member this module does not know rides through untouched. The release
/// owns the record's shape and may add members; a struct naming today's
/// members would silently erase each new one at every install.
pub fn bpee_installed_tamga_json(
    mut json: serde_json::Value,
    taken: &[String],
    seat: bool,
) -> Result<serde_json::Value, String> {
    let manner = match seat {
        true => BPEE_MANNER_DIRECTORY,
        false => BPEE_MANNER_REPOSITORY,
    };
    let root = json
        .as_object_mut()
        .ok_or_else(|| "Record is not a JSON object".to_string())?;
    root.insert(
        BPEE_TAMGA_KITS.to_string(),
        serde_json::Value::Array(
            taken
                .iter()
                .cloned()
                .map(serde_json::Value::String)
                .collect(),
        ),
    );
    root.insert(
        BPEE_TAMGA_MANNER.to_string(),
        serde_json::Value::String(manner.to_string()),
    );
    Ok(json)
}

/// Copy a kit directory and route special files.
/// Returns (files_copied, commands_routed, hooks_routed).
fn zbpee_copy_kit(
    source: &Path,
    dest: &Path,
    project_root: &Path,
    kit_id: &str,
    signets: &bpee_Signets,
) -> Result<(u32, u32, u32), String> {
    let mut files = 0u32;
    let mut commands = 0u32;
    let mut hooks = 0u32;

    // Get cipher from kit_id (e.g., "jjk" -> "jj")
    let cipher = &kit_id[..kit_id.len().saturating_sub(1)];

    // Walk source directory
    let entries = zbpee_walk_dir(source)?;

    for entry in entries {
        let rel_path = entry.strip_prefix(source).map_err(|e| {
            format!(
                "Path strip failed: {}",
                e, //
            )
        })?;

        let file_name = rel_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Determine destination based on file type
        let dest_path = if zbpee_is_command_file(file_name, cipher, signets) {
            // Route commands to .claude/commands/
            let commands_dir = project_root.join(".claude").join("commands");
            fs::create_dir_all(&commands_dir).map_err(|e| {
                format!(
                    "Failed to create commands dir: {}",
                    e, //
                )
            })?;
            commands += 1;
            commands_dir.join(file_name)
        } else if zbpee_is_hook_file(file_name, cipher, signets) {
            // Route hooks to .claude/hooks/
            let hooks_dir = project_root.join(".claude").join("hooks");
            fs::create_dir_all(&hooks_dir).map_err(|e| {
                format!(
                    "Failed to create hooks dir: {}",
                    e, //
                )
            })?;
            hooks += 1;
            hooks_dir.join(file_name)
        } else {
            // Default: preserve relative path in kit directory
            dest.join(rel_path)
        };

        // Create parent directories
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                format!(
                    "Failed to create dir {}: {}",
                    parent.display(),
                    e, //
                )
            })?;
        }

        // Copy file
        bkprs_copy(&entry, &dest_path).map_err(|e| {
            format!(
                "Failed to copy {} to {}: {}",
                entry.display(),
                dest_path.display(),
                e, //
            )
        })?;

        files += 1;
    }

    Ok((files, commands, hooks))
}

/// Host platform moniker, matching the table the parcel bootstraps use
/// (bpi_install.sh zbpi_platform_capture, bpu_uninstall.sh zbpu_platform_capture).
/// The two must agree: the bootstrap picks the binary to run, this picks the
/// binary to keep.
fn zbpee_platform_moniker() -> Result<String, String> {
    let moniker = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => "darwin-arm64",
        ("macos", "x86_64") => "darwin-x86_64",
        ("linux", "x86_64") => "linux-x86_64",
        ("linux", "aarch64") => "linux-aarch64",
        (os, arch) => {
            return Err(format!(
                "No binary for platform {}-{}",
                os,
                arch, //
            ));
        }
    };

    Ok(moniker.to_string())
}

/// Place the host's binary at the canonical path and purge every
/// platform-suffixed sibling. Returns the platform emplaced.
///
/// The source is the parcel rather than the just-copied target file: the
/// parcel is what the record names, and reading it directly keeps this step
/// correct regardless of what step 7 happened to lay down.
fn zbpee_emplace_canonical_binary(
    parcel_dir: &Path,
    tools_dir: &Path,
    binary: &bpee_Binary,
) -> Result<String, String> {
    let platform = zbpee_platform_moniker()?;

    let source = parcel_dir
        .join("kits")
        .join(&binary.kit)
        .join("bin")
        .join(format!(
            "{}{}",
            binary.suffixed_prefix,
            platform, //
        ));

    if !source.is_file() {
        return Err(format!(
            "No binary for platform {} in parcel ({})",
            platform,
            source.display(), //
        ));
    }

    let bin_dir = tools_dir.join(&binary.kit).join("bin");
    fs::create_dir_all(&bin_dir).map_err(|e| {
        format!(
            "Failed to create {} bin dir: {}",
            binary.kit,
            e, //
        )
    })?;

    let canonical = bin_dir.join(&binary.canonical);
    // Armed rather than carried: the parcel's mode survives a tar round trip
    // only if the archive kept it, and a canonical binary that cannot execute
    // is a working install by every other measure.
    bkprs_program_copy(&source, &canonical).map_err(|e| {
        format!(
            "Failed to emplace canonical binary: {}",
            e, //
        )
    })?;

    // Purge the platform-suffixed siblings step 7 copied in
    let entries = fs::read_dir(&bin_dir).map_err(|e| {
        format!(
            "Failed to read {} bin dir: {}",
            binary.kit,
            e, //
        )
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            format!(
                "Dir entry error: {}",
                e, //
            )
        })?;
        let path = entry.path();
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if file_name.starts_with(binary.suffixed_prefix.as_str()) {
            fs::remove_file(&path).map_err(|e| {
                format!(
                    "Failed to purge {}: {}",
                    path.display(),
                    e, //
                )
            })?;
        }
    }

    Ok(platform)
}

/// Walk directory recursively, returning all file paths.
fn zbpee_walk_dir(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();

    if !dir.is_dir() {
        return Ok(files);
    }

    let entries = fs::read_dir(dir).map_err(|e| {
        format!(
            "Failed to read dir {}: {}",
            dir.display(),
            e, //
        )
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            format!(
                "Dir entry error: {}",
                e, //
            )
        })?;
        let path = entry.path();

        if path.is_dir() {
            files.extend(zbpee_walk_dir(&path)?);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}

/// Check if file is a command file (matches {cipher}c-*.md pattern).
fn zbpee_is_command_file(file_name: &str, cipher: &str, signets: &bpee_Signets) -> bool {
    let prefix = format!(
        "{}{}",
        cipher,
        signets.command, //
    );
    file_name.starts_with(&prefix) && file_name.ends_with(".md")
}

/// Check if file is a hook file (matches {cipher}h-*.md pattern).
fn zbpee_is_hook_file(file_name: &str, cipher: &str, signets: &bpee_Signets) -> bool {
    let prefix = format!(
        "{}{}",
        cipher,
        signets.hook, //
    );
    file_name.starts_with(&prefix) && file_name.ends_with(".md")
}

/// Route Claude config assets from parcel claude/* to target .claude/*,
/// only those whose cipher is a taken kit's: a command or hook of a kit the
/// target did not take stays in the parcel.
/// Returns (commands_routed, hooks_routed).
fn zbpee_route_claude_assets(
    parcel_dir: &Path,
    project_root: &Path,
    taken: &[String],
    signets: &bpee_Signets,
) -> Result<(u32, u32), String> {
    let ciphers: Vec<&str> = taken
        .iter()
        .map(|k| &k[..k.len().saturating_sub(1)])
        .collect();

    let mut commands_count = 0u32;
    let mut hooks_count = 0u32;

    // Route commands from parcel claude/commands/ to target .claude/commands/
    let commands_source = parcel_dir.join("claude").join("commands");
    if commands_source.exists() {
        let commands_dest = project_root.join(".claude").join("commands");
        fs::create_dir_all(&commands_dest).map_err(|e| {
            format!(
                "Failed to create .claude/commands dir: {}",
                e, //
            )
        })?;

        commands_count = zbpee_copy_files_where(&commands_source, &commands_dest, |name| {
            ciphers
                .iter()
                .any(|c| zbpee_is_command_file(name, c, signets))
        })?;
    }

    // Route hooks from parcel claude/hooks/ to target .claude/hooks/
    let hooks_source = parcel_dir.join("claude").join("hooks");
    if hooks_source.exists() {
        let hooks_dest = project_root.join(".claude").join("hooks");
        fs::create_dir_all(&hooks_dest).map_err(|e| {
            format!(
                "Failed to create .claude/hooks dir: {}",
                e, //
            )
        })?;

        hooks_count = zbpee_copy_files_where(&hooks_source, &hooks_dest, |name| {
            ciphers.iter().any(|c| zbpee_is_hook_file(name, c, signets))
        })?;
    }

    Ok((commands_count, hooks_count))
}

/// Copy the files from source directory to dest directory whose names the
/// predicate admits. Returns count of files copied.
fn zbpee_copy_files_where(
    source_dir: &Path,
    dest_dir: &Path,
    admit: impl Fn(&str) -> bool,
) -> Result<u32, String> {
    let mut count = 0u32;

    let entries = fs::read_dir(source_dir).map_err(|e| {
        format!(
            "Failed to read directory {}: {}",
            source_dir.display(),
            e, //
        )
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            format!(
                "Dir entry error: {}",
                e, //
            )
        })?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_name = path.file_name().ok_or_else(|| {
            format!(
                "No file name for {}",
                path.display(), //
            )
        })?;

        if !admit(&file_name.to_string_lossy()) {
            continue;
        }

        let dest_path = dest_dir.join(file_name);
        bkprs_copy(&path, &dest_path).map_err(|e| {
            format!(
                "Failed to copy {} to {}: {}",
                path.display(),
                dest_path.display(),
                e, //
            )
        })?;
        count += 1;
    }

    Ok(count)
}

/// Remove files matching a pattern from a directory.
/// If is_command is true, matches {cipher}c-*.md; otherwise matches {cipher}h-*.md
/// Returns count of files removed.
fn zbpee_remove_matching_files(
    dir: &Path,
    cipher: &str,
    signets: &bpee_Signets,
    is_command: bool,
) -> Result<u32, String> {
    let mut count = 0u32;

    let entries = fs::read_dir(dir).map_err(|e| {
        format!(
            "Failed to read directory {}: {}",
            dir.display(),
            e, //
        )
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| {
            format!(
                "Dir entry error: {}",
                e, //
            )
        })?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        let matches = if is_command {
            zbpee_is_command_file(file_name, cipher, signets)
        } else {
            zbpee_is_hook_file(file_name, cipher, signets)
        };

        if matches {
            fs::remove_file(&path).map_err(|e| {
                format!(
                    "Failed to remove {}: {}",
                    path.display(),
                    e, //
                )
            })?;
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The signets the tests' assets wear, spelled here because their own home
    /// is the caller's registry.
    fn zbpee_test_signets() -> bpee_Signets {
        bpee_Signets {
            command: "c-".to_string(),
            hook: "h-".to_string(),
        }
    }

    /// The binary the tests' parcels carry, named as the caller names its own.
    fn zbpee_test_binary() -> bpee_Binary {
        bpee_Binary {
            kit: "vvk".to_string(),
            canonical: "vvx".to_string(),
            suffixed_prefix: "vvx-".to_string(),
        }
    }

    #[test]
    fn bpee_is_command_file() {
        assert!(zbpee_is_command_file(
            "jjc-heat-groom.md",
            "jj",
            &zbpee_test_signets()
        ));
        assert!(zbpee_is_command_file(
            "vvc-commit.md",
            "vv",
            &zbpee_test_signets()
        ));
        assert!(!zbpee_is_command_file(
            "jjw_workbench.sh",
            "jj",
            &zbpee_test_signets()
        ));
        assert!(!zbpee_is_command_file(
            "README.md",
            "jj",
            &zbpee_test_signets()
        ));
    }

    #[test]
    fn bpee_is_hook_file() {
        assert!(zbpee_is_hook_file(
            "jjh-post-commit.md",
            "jj",
            &zbpee_test_signets()
        ));
        assert!(zbpee_is_hook_file(
            "vvh-pre-push.md",
            "vv",
            &zbpee_test_signets()
        ));
        assert!(!zbpee_is_hook_file(
            "jjc-heat-groom.md",
            "jj",
            &zbpee_test_signets()
        ));
        assert!(!zbpee_is_hook_file(
            "jjw_workbench.sh",
            "jj",
            &zbpee_test_signets()
        ));
    }

    #[test]
    fn bpee_platform_moniker_matches_bootstrap_table() {
        // The bootstraps (bpi_install.sh, bpu_uninstall.sh) name the same
        // four; anything this returns must be one the release can stage.
        let moniker = zbpee_platform_moniker().expect("test host must be a supported platform");
        assert!(
            [
                "darwin-arm64",
                "darwin-x86_64",
                "linux-x86_64",
                "linux-aarch64"
            ]
            .contains(&moniker.as_str()),
            "unexpected moniker: {}",
            moniker, //
        );
    }

    /// A parcel bearing vvk must leave a canonical binary and no suffixed
    /// sibling. Both halves matter: the canonical path is what every
    /// consumer surface invokes, and a surviving sibling is untracked dirt
    /// in the install delta the caller has to read.
    #[test]
    fn bpee_emplace_canonical_binary() {
        let platform = zbpee_platform_moniker().unwrap();
        let temp_dir = std::env::temp_dir().join("bpee_test_canonical");
        let _ = fs::remove_dir_all(&temp_dir);

        // Parcel side: the staged platform binary
        let parcel_bin = temp_dir.join("parcel/kits/vvk/bin");
        fs::create_dir_all(&parcel_bin).unwrap();
        fs::write(
            parcel_bin.join(format!(
                "vvx-{}",
                platform, //
            )),
            b"PARCEL BINARY",
        )
        .unwrap();

        // Target side: what step 7's verbatim kit copy leaves behind
        let tools_dir = temp_dir.join("target/Tools");
        let target_bin = tools_dir.join("vvk/bin");
        fs::create_dir_all(&target_bin).unwrap();
        fs::write(
            target_bin.join(format!(
                "vvx-{}",
                platform, //
            )),
            b"PARCEL BINARY",
        )
        .unwrap();
        fs::write(target_bin.join("vvx-linux-aarch64"), b"OTHER PLATFORM").unwrap();
        fs::write(target_bin.join("README.md"), b"consumer note\n").unwrap();

        let emplaced = zbpee_emplace_canonical_binary(
            &temp_dir.join("parcel"),
            &tools_dir,
            &zbpee_test_binary(),
        )
        .unwrap();
        assert_eq!(emplaced, platform);

        let canonical = target_bin.join("vvx");
        assert!(
            canonical.is_file(),
            "canonical binary must exist", //
        );
        assert_eq!(fs::read(&canonical).unwrap(), b"PARCEL BINARY");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = fs::metadata(&canonical).unwrap().permissions().mode();
            assert_eq!(
                mode & 0o111,
                0o111,
                "canonical binary must be executable", //
            );
        }

        // No platform-suffixed sibling survives, this host's included
        for entry in fs::read_dir(&target_bin).unwrap() {
            let name = entry.unwrap().file_name().into_string().unwrap();
            assert!(
                !name.starts_with(zbpee_test_binary().suffixed_prefix.as_str()),
                "suffixed binary survived: {}",
                name, //
            );
        }

        // Non-binary content in the same directory is untouched
        assert!(target_bin.join("README.md").is_file());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// A parcel with no vvk installs no binary and creates no vvk directory.
    /// The platform binary is staged into every parcel whatever the imprint,
    /// so an unconditional emplace would silently plant one in a consumer
    /// that never asked for the kit.
    #[test]
    fn bpee_no_canonical_binary_without_vvk() {
        let temp_dir = std::env::temp_dir().join("bpee_test_no_vvk");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_bin = temp_dir.join("parcel/kits/vvk/bin");
        fs::create_dir_all(&parcel_bin).unwrap();
        fs::write(parcel_bin.join("vvx-darwin-arm64"), b"PARCEL BINARY").unwrap();

        let tools_dir = temp_dir.join("target/Tools");
        fs::create_dir_all(&tools_dir).unwrap();

        // bpee_emplace calls the canonical step only when the binary's kit is
        // in the kit list; a kit list naming buk alone therefore never reaches
        // it, and the vvk tree must not appear.
        let kit_ids = vec!["buk".to_string()];
        assert!(!kit_ids.iter().any(|k| *k == zbpee_test_binary().kit));
        assert!(
            !tools_dir.join("vvk").exists(),
            "no vvk directory may appear in a non-vvk target", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn bpee_parse_burc_simple() {
        use std::io::Write;
        let temp_dir = std::env::temp_dir().join("bpee_test_burc");
        let _ = fs::remove_dir_all(&temp_dir);

        // Create directory structure: temp/buk/burc.env, temp/project, temp/project/Tools
        let config_dir = temp_dir.join("buk");
        let project_dir = temp_dir.join("project");
        let tools_dir = project_dir.join("Tools");
        fs::create_dir_all(&config_dir).unwrap();
        fs::create_dir_all(&tools_dir).unwrap();

        let burc_path = config_dir.join("burc.env");
        let mut f = fs::File::create(&burc_path).unwrap();
        writeln!(
            f,
            "# Comment", //
        )
        .unwrap();
        writeln!(
            f,
            "BURC_PROJECT_ROOT=../project", //
        )
        .unwrap();
        writeln!(
            f,
            "BURC_TOOLS_DIR=Tools", //
        )
        .unwrap();
        writeln!(
            f,
            "BURC_MANAGED_KITS=buk,jjk", //
        )
        .unwrap();

        let result = bpee_parse_burc(&burc_path).unwrap();
        assert_eq!(
            result.project_root.canonicalize().unwrap(),
            project_dir.canonicalize().unwrap()
        );
        assert_eq!(
            result.tools_dir.canonicalize().unwrap(),
            tools_dir.canonicalize().unwrap()
        );
        assert_eq!(result.managed_kits, vec!["buk", "jjk"]);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn bpee_parse_burc_quoted() {
        use std::io::Write;
        let temp_dir = std::env::temp_dir().join("bpee_test_burc_quoted");
        let _ = fs::remove_dir_all(&temp_dir);

        // Create directory structure
        let config_dir = temp_dir.join("buk");
        let project_dir = temp_dir.join("project");
        let tools_dir = project_dir.join("Tools");
        fs::create_dir_all(&config_dir).unwrap();
        fs::create_dir_all(&tools_dir).unwrap();

        let burc_path = config_dir.join("burc.env");
        let mut f = fs::File::create(&burc_path).unwrap();
        writeln!(
            f,
            "BURC_PROJECT_ROOT=\"../project\"", //
        )
        .unwrap();
        writeln!(
            f,
            "BURC_TOOLS_DIR='Tools'", //
        )
        .unwrap();
        writeln!(
            f,
            "BURC_MANAGED_KITS=\"vvk\"", //
        )
        .unwrap();

        let result = bpee_parse_burc(&burc_path).unwrap();
        assert_eq!(
            result.project_root.canonicalize().unwrap(),
            project_dir.canonicalize().unwrap()
        );
        assert_eq!(
            result.tools_dir.canonicalize().unwrap(),
            tools_dir.canonicalize().unwrap()
        );
        assert_eq!(result.managed_kits, vec!["vvk"]);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Build a clean git repo at `dir` (init + commit whatever is already
    /// on disk there) so `bpee_emplace`'s git-state checks pass.
    fn zbpee_init_clean_git_repo(dir: &Path) {
        let run = |args: &[&str]| {
            let output = Command::new("git")
                .args(args)
                .current_dir(dir)
                .output()
                .expect("git invocation failed");
            assert!(
                output.status.success(),
                "git {:?} failed: {}",
                args,
                String::from_utf8_lossy(&output.stderr), //
            );
        };
        run(&["init", "-q"]);
        run(&["config", "user.email", "test@example.com"]);
        run(&["config", "user.name", "Test"]);
        run(&["add", "-A"]);
        run(&["commit", "-q", "-m", "initial"]);
    }

    /// A subset parcel (buk only) lands into a superset consumer (declaring
    /// buk, jjk, vvk): it installs its own kit and leaves the target's other
    /// declared-but-unmentioned kits byte-untouched. This is the containment
    /// law's affirmative case — equality would have refused this parcel
    /// outright, which is exactly what the 1055 parcel carrying buk alone hit against
    /// the buk,jjk,vvk consumer.
    #[test]
    fn bpee_emplace_subset_parcel_into_superset_consumer() {
        let temp_dir = std::env::temp_dir().join("bpee_test_containment_subset");
        let _ = fs::remove_dir_all(&temp_dir);

        // Parcel side: carries buk only.
        let parcel_dir = temp_dir.join("parcel");
        let parcel_buk = parcel_dir.join("kits/buk");
        fs::create_dir_all(&parcel_buk).unwrap();
        fs::write(parcel_buk.join("buc_command.sh"), b"# fresh buk content").unwrap();
        fs::write(
            parcel_dir.join(BPEE_TAMGA_FILE),
            br#"{"vvmt_tamga":"1099","vvmk_kits":["buk"]}"#,
        )
        .unwrap();

        // Target side: declares buk, jjk, vvk; already carries jjk and vvk
        // content that the parcel carrying buk alone must not touch.
        let project_dir = temp_dir.join("target");
        let tools_dir = project_dir.join("Tools");
        fs::create_dir_all(tools_dir.join("buk")).unwrap();
        fs::write(tools_dir.join("buk/buc_command.sh"), b"# stale buk content").unwrap();
        fs::create_dir_all(tools_dir.join("jjk")).unwrap();
        fs::write(tools_dir.join("jjk/marker.txt"), b"JJK UNTOUCHED").unwrap();
        fs::create_dir_all(tools_dir.join("vvk")).unwrap();
        fs::write(tools_dir.join("vvk/marker.txt"), b"VVK UNTOUCHED").unwrap();
        zbpee_init_clean_git_repo(&project_dir);

        let burc_path = temp_dir.join("burc.env");
        {
            use std::io::Write;
            let mut f = fs::File::create(&burc_path).unwrap();
            writeln!(
                f,
                "BURC_PROJECT_ROOT=target", //
            )
            .unwrap();
            writeln!(
                f,
                "BURC_TOOLS_DIR=Tools", //
            )
            .unwrap();
            writeln!(
                f,
                "BURC_MANAGED_KITS=buk,jjk,vvk", //
            )
            .unwrap();
        }

        let result = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir: parcel_dir.clone(),
            burc_path,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect("subset parcel must emplace into superset consumer");

        assert_eq!(result.kits_installed, vec!["buk"]);
        assert_eq!(
            fs::read(tools_dir.join("buk/buc_command.sh")).unwrap(),
            b"# fresh buk content"
        );
        assert_eq!(
            fs::read(tools_dir.join("jjk/marker.txt")).unwrap(),
            b"JJK UNTOUCHED",
            "jjk must survive an install of buk alone byte-untouched", //
        );
        assert_eq!(
            fs::read(tools_dir.join("vvk/marker.txt")).unwrap(),
            b"VVK UNTOUCHED",
            "vvk must survive an install of buk alone byte-untouched", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Build a parcel at `parcel_dir` carrying each named kit (a marker file
    /// apiece) and a record whose body is `record` verbatim.
    fn zbpee_stage_parcel(parcel_dir: &Path, kits: &[&str], record: &str) {
        for kit in kits {
            let kit_dir = parcel_dir.join("kits").join(kit);
            fs::create_dir_all(&kit_dir).unwrap();
            fs::write(
                kit_dir.join("marker.txt"),
                format!(
                    "PARCEL {}",
                    kit, //
                ),
            )
            .unwrap();
        }
        fs::write(parcel_dir.join(BPEE_TAMGA_FILE), record).unwrap();
    }

    /// Write a burc.env at `temp_dir` rooting the target at `target` and
    /// declaring `kits`. Returns its path.
    fn zbpee_stage_burc(temp_dir: &Path, kits: &str) -> PathBuf {
        use std::io::Write;
        let burc_path = temp_dir.join("burc.env");
        let mut f = fs::File::create(&burc_path).unwrap();
        writeln!(
            f,
            "BURC_PROJECT_ROOT=target", //
        )
        .unwrap();
        writeln!(
            f,
            "BURC_TOOLS_DIR=Tools", //
        )
        .unwrap();
        writeln!(
            f,
            "BURC_MANAGED_KITS={}",
            kits, //
        )
        .unwrap();
        burc_path
    }

    /// An empty intersection refuses before the nuclear cleanup: the
    /// target's declared kit, which the parcel does not carry, stands
    /// byte-untouched and no record directory is written.
    #[test]
    fn bpee_emplace_empty_intersection_refuses_before_cleanup() {
        let temp_dir = std::env::temp_dir().join("bpee_test_empty_intersection_cleanup");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_parcel(
            &parcel_dir,
            &["cmk"],
            r#"{"vvmt_tamga":"1099","vvmk_kits":["cmk"]}"#,
        );

        let project_dir = temp_dir.join("target");
        let tools_dir = project_dir.join("Tools");
        fs::create_dir_all(tools_dir.join("buk")).unwrap();
        fs::write(tools_dir.join("buk/marker.txt"), b"BUK UNTOUCHED").unwrap();
        zbpee_init_clean_git_repo(&project_dir);
        let burc_path = zbpee_stage_burc(&temp_dir, "buk");

        bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect_err("an empty intersection must refuse");

        assert_eq!(
            fs::read(tools_dir.join("buk/marker.txt")).unwrap(),
            b"BUK UNTOUCHED"
        );
        assert!(
            !project_dir.join(BPEE_TAMGA_DIR).exists(),
            "no record may be written", //
        );
        assert!(!tools_dir.join("cmk").exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a parcel wider than its target installs the taken set alone.
    /// The undeclared kit is left in the parcel — no directory for it in the
    /// target, nothing in the result, and no fault.
    #[test]
    fn bpee_emplace_wider_parcel_installs_taken_set_alone() {
        let temp_dir = std::env::temp_dir().join("bpee_test_wider_parcel");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_parcel(
            &parcel_dir,
            &["bkk", "buk", "cmk"],
            r#"{"vvmt_tamga":"1099","vvmk_kits":["bkk","buk","cmk"]}"#,
        );

        let project_dir = temp_dir.join("target");
        let tools_dir = project_dir.join("Tools");
        fs::create_dir_all(tools_dir.join("buk")).unwrap();
        fs::write(tools_dir.join("buk/marker.txt"), b"STALE BUK").unwrap();
        zbpee_init_clean_git_repo(&project_dir);
        let burc_path = zbpee_stage_burc(&temp_dir, "buk");

        let result = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir: parcel_dir.clone(),
            burc_path,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect("a parcel wider than its target must install its taken set");

        assert_eq!(result.kits_installed, vec!["buk"]);
        assert_eq!(
            fs::read(tools_dir.join("buk/marker.txt")).unwrap(),
            b"PARCEL buk"
        );
        assert!(
            !tools_dir.join("bkk").exists(),
            "undeclared bkk must not be installed", //
        );
        assert!(
            !tools_dir.join("cmk").exists(),
            "undeclared cmk must not be installed", //
        );
        assert!(
            parcel_dir.join("kits/cmk/marker.txt").is_file(),
            "the undeclared kit stays in the parcel untouched", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: an empty intersection refuses in the one-parcel road's own
    /// sentence, naming both sides.
    #[test]
    fn bpee_emplace_empty_intersection_refuses_in_vosh_sentence() {
        let temp_dir = std::env::temp_dir().join("bpee_test_empty_intersection_sentence");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_parcel(
            &parcel_dir,
            &["bkk", "cmk"],
            r#"{"vvmt_tamga":"1099","vvmk_kits":["bkk","cmk"]}"#,
        );

        let project_dir = temp_dir.join("target");
        fs::create_dir_all(project_dir.join("Tools")).unwrap();
        fs::write(project_dir.join("README"), b"target").unwrap();
        zbpee_init_clean_git_repo(&project_dir);
        let burc_path = zbpee_stage_burc(&temp_dir, "buk,jjk");

        let err = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect_err("an empty intersection must refuse");

        assert_eq!(
            err,
            "Parcel [bkk,cmk] carries none of the kits target declares [buk,jjk]"
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: the target's record names the taken set and carries through a
    /// member this module has never heard of. The release owns the record's
    /// shape; an install that dropped an unknown member would erase a
    /// sibling's record at every install.
    #[test]
    fn bpee_emplace_tamga_records_taken_set_and_keeps_unknown_member() {
        let temp_dir = std::env::temp_dir().join("bpee_test_tamga_taken_unknown");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_parcel(
            &parcel_dir,
            &["bkk", "buk"],
            r#"{"vvmt_tamga":"1099","vvmk_kits":["bkk","buk"],"vvbz_unheard":{"position":"abc123","n":7}}"#,
        );

        let project_dir = temp_dir.join("target");
        fs::create_dir_all(project_dir.join("Tools")).unwrap();
        fs::write(project_dir.join("README"), b"target").unwrap();
        zbpee_init_clean_git_repo(&project_dir);
        let burc_path = zbpee_stage_burc(&temp_dir, "buk");

        bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect("install must succeed");

        let body = fs::read_to_string(project_dir.join(BPEE_TAMGA_RELPATH)).unwrap();
        assert_eq!(
            bpee_tamga_kits(|seat| fs::read_to_string(project_dir.join(seat)).ok()),
            vec!["buk"]
        );
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json[BPEE_TAMGA_NUMBER], "1099");
        assert_eq!(
            json["vvbz_unheard"],
            serde_json::json!({"position":"abc123","n":7}),
            "an unknown record member must ride through untouched", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a parcel carrying vvk, installed into a target that does not
    /// declare it, emplaces no canonical binary and creates no vvk tree.
    #[test]
    fn bpee_emplace_untaken_vvk_emplaces_no_binary() {
        let platform = zbpee_platform_moniker().unwrap();
        let temp_dir = std::env::temp_dir().join("bpee_test_untaken_vvk");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_parcel(
            &parcel_dir,
            &["buk", "vvk"],
            r#"{"vvmt_tamga":"1099","vvmk_kits":["buk","vvk"]}"#,
        );
        let parcel_bin = parcel_dir.join("kits/vvk/bin");
        fs::create_dir_all(&parcel_bin).unwrap();
        fs::write(
            parcel_bin.join(format!(
                "vvx-{}",
                platform, //
            )),
            b"PARCEL BINARY",
        )
        .unwrap();

        let project_dir = temp_dir.join("target");
        fs::create_dir_all(project_dir.join("Tools")).unwrap();
        fs::write(project_dir.join("README"), b"target").unwrap();
        zbpee_init_clean_git_repo(&project_dir);
        let burc_path = zbpee_stage_burc(&temp_dir, "buk");

        let result = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect("install must succeed");

        assert_eq!(result.canonical_platform, None);
        assert!(
            !project_dir.join("Tools/vvk").exists(),
            "no vvk tree may appear in a target that did not take vvk", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a Claude asset whose cipher belongs to a kit the target did
    /// not take is not routed; a taken kit's asset is.
    #[test]
    fn bpee_emplace_untaken_cipher_asset_not_routed() {
        let temp_dir = std::env::temp_dir().join("bpee_test_untaken_asset");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_parcel(
            &parcel_dir,
            &["buk", "jjk"],
            r#"{"vvmt_tamga":"1099","vvmk_kits":["buk","jjk"]}"#,
        );
        let cmds = parcel_dir.join("claude/commands");
        let hks = parcel_dir.join("claude/hooks");
        fs::create_dir_all(&cmds).unwrap();
        fs::create_dir_all(&hks).unwrap();
        fs::write(cmds.join("buc-taken.md"), b"BU CMD").unwrap();
        fs::write(cmds.join("jjc-untaken.md"), b"JJ CMD").unwrap();
        fs::write(hks.join("buh-taken.md"), b"BU HOOK").unwrap();
        fs::write(hks.join("jjh-untaken.md"), b"JJ HOOK").unwrap();

        let project_dir = temp_dir.join("target");
        fs::create_dir_all(project_dir.join("Tools")).unwrap();
        fs::write(project_dir.join("README"), b"target").unwrap();
        zbpee_init_clean_git_repo(&project_dir);
        let burc_path = zbpee_stage_burc(&temp_dir, "buk");

        let result = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect("install must succeed");

        let target_cmds = project_dir.join(".claude/commands");
        let target_hks = project_dir.join(".claude/hooks");
        assert!(target_cmds.join("buc-taken.md").is_file());
        assert!(target_hks.join("buh-taken.md").is_file());
        assert!(
            !target_cmds.join("jjc-untaken.md").exists(),
            "untaken cipher's command routed", //
        );
        assert!(
            !target_hks.join("jjh-untaken.md").exists(),
            "untaken cipher's hook routed", //
        );
        assert_eq!(result.commands_routed, 1);
        assert_eq!(result.hooks_routed, 1);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// A parcel whose record claims a kit but whose `kits/<kit_id>/` directory
    /// does not actually exist on disk must refuse before the nuclear
    /// cleanup runs, not after: the target's declared kit directories must
    /// stand intact when this refusal fires.
    #[test]
    fn bpee_emplace_short_parcel_refuses_before_cleanup() {
        let temp_dir = std::env::temp_dir().join("bpee_test_short_parcel");
        let _ = fs::remove_dir_all(&temp_dir);

        // Parcel claims buk and jjk in its record, but only buk actually
        // stands under kits/ — jjk is missing.
        let parcel_dir = temp_dir.join("parcel");
        let parcel_buk = parcel_dir.join("kits/buk");
        fs::create_dir_all(&parcel_buk).unwrap();
        fs::write(parcel_buk.join("marker.txt"), b"BUK CONTENT").unwrap();
        fs::write(
            parcel_dir.join(BPEE_TAMGA_FILE),
            br#"{"vvmt_tamga":"1099","vvmk_kits":["buk","jjk"]}"#,
        )
        .unwrap();

        // Target already carries both kits; a short parcel must not disturb
        // either.
        let project_dir = temp_dir.join("target");
        let tools_dir = project_dir.join("Tools");
        fs::create_dir_all(tools_dir.join("buk")).unwrap();
        fs::write(tools_dir.join("buk/marker.txt"), b"BUK UNTOUCHED").unwrap();
        fs::create_dir_all(tools_dir.join("jjk")).unwrap();
        fs::write(tools_dir.join("jjk/marker.txt"), b"JJK UNTOUCHED").unwrap();
        zbpee_init_clean_git_repo(&project_dir);

        let burc_path = temp_dir.join("burc.env");
        {
            use std::io::Write;
            let mut f = fs::File::create(&burc_path).unwrap();
            writeln!(
                f,
                "BURC_PROJECT_ROOT=target", //
            )
            .unwrap();
            writeln!(
                f,
                "BURC_TOOLS_DIR=Tools", //
            )
            .unwrap();
            writeln!(
                f,
                "BURC_MANAGED_KITS=buk,jjk", //
            )
            .unwrap();
        }

        let err = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect_err("a parcel missing a claimed kit directory must refuse");

        assert!(
            err.contains("Kit not found in parcel"),
            "unexpected error: {}",
            err, //
        );
        assert!(
            err.contains("jjk"),
            "error must name the missing kit: {}",
            err, //
        );

        // Refused before the nuclear cleanup step: both of the target's
        // declared kit directories must survive untouched.
        assert!(
            tools_dir.join("buk").exists(),
            "buk directory must survive a short-parcel refusal", //
        );
        assert!(
            tools_dir.join("jjk").exists(),
            "jjk directory must survive a short-parcel refusal", //
        );
        assert_eq!(
            fs::read(tools_dir.join("buk/marker.txt")).unwrap(),
            b"BUK UNTOUCHED"
        );
        assert_eq!(
            fs::read(tools_dir.join("jjk/marker.txt")).unwrap(),
            b"JJK UNTOUCHED"
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Build a target repo shaped like an installed consumer: kit
    /// directories under Tools/, Claude assets for each kit's cipher, and a
    /// record naming the kit set. Returns the burc.env path.
    fn zbpee_installed_target(
        temp_dir: &Path,
        kits: &[&str],
        with_binary: bool,
    ) -> (PathBuf, PathBuf) {
        let project_dir = temp_dir.join("target");
        let tools_dir = project_dir.join("Tools");

        for kit in kits {
            fs::create_dir_all(tools_dir.join(kit)).unwrap();
            fs::write(tools_dir.join(kit).join("marker.txt"), b"KIT CONTENT").unwrap();
        }

        if with_binary {
            let bin_dir = tools_dir.join("vvk/bin");
            fs::create_dir_all(&bin_dir).unwrap();
            fs::write(bin_dir.join("vvx"), b"CANONICAL BINARY").unwrap();
        }

        let commands_dir = project_dir.join(".claude/commands");
        let hooks_dir = project_dir.join(".claude/hooks");
        fs::create_dir_all(&commands_dir).unwrap();
        fs::create_dir_all(&hooks_dir).unwrap();
        for kit in kits {
            let cipher = &kit[..kit.len() - 1];
            fs::write(
                commands_dir.join(format!(
                    "{}c-thing.md",
                    cipher, //
                )),
                b"CMD",
            )
            .unwrap();
            fs::write(
                hooks_dir.join(format!(
                    "{}h-thing.md",
                    cipher, //
                )),
                b"HOOK",
            )
            .unwrap();
        }
        // An operator's own asset, matching no kit cipher, must survive.
        fs::write(commands_dir.join("xyc-operator.md"), b"OPERATOR").unwrap();

        let tamga_dir = project_dir.join(BPEE_TAMGA_DIR);
        fs::create_dir_all(&tamga_dir).unwrap();
        let kit_list = kits
            .iter()
            .map(|k| {
                format!(
                    "\"{}\"",
                    k, //
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        fs::write(
            tamga_dir.join(BPEE_TAMGA_FILE),
            format!(
                r#"{{"vvmt_tamga":"1099","vvmk_kits":[{}]}}"#,
                kit_list, //
            ),
        )
        .unwrap();

        zbpee_init_clean_git_repo(&project_dir);

        let burc_path = temp_dir.join("burc.env");
        {
            use std::io::Write;
            let mut f = fs::File::create(&burc_path).unwrap();
            writeln!(
                f,
                "BURC_PROJECT_ROOT=target", //
            )
            .unwrap();
            writeln!(
                f,
                "BURC_TOOLS_DIR=Tools", //
            )
            .unwrap();
            writeln!(
                f,
                "BURC_MANAGED_KITS={}",
                kits.join(","), //
            )
            .unwrap();
        }

        (burc_path, project_dir)
    }

    /// A target whose parcel carried no vvk vacates whole. This is the case
    /// the old bootstrap could not reach at all — the target holds no vvk
    /// directory, so it held neither the uninstall script nor a binary to
    /// run it. The vacate procedure itself never depended on either: it
    /// reads its kit list from the target's own record, and its
    /// binary-removal step is guarded on the vvk bin directory existing.
    #[test]
    fn bpee_vacate_non_vvk_target() {
        let temp_dir = std::env::temp_dir().join("bpee_test_vacate_non_vvk");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let (burc_path, project_dir) = zbpee_installed_target(&temp_dir, &["buk"], false);

        let result = bpee_vacate(&bpee_VacateArgs {
            burc_path,
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
        })
        .expect("a target holding buk alone must vacate");

        assert_eq!(result.kits_removed, vec!["buk"]);
        assert_eq!(result.commands_removed, 1);
        assert_eq!(result.hooks_removed, 1);
        // No binary to remove: the target never received one.
        assert_eq!(result.files_deleted, 0);

        assert!(
            !project_dir.join(BPEE_TAMGA_DIR).exists(),
            "the record's directory must be gone after vacate", //
        );
        assert!(
            project_dir.join("Tools/buk/marker.txt").is_file(),
            "kit directories are preserved", //
        );
        assert!(
            !project_dir.join(".claude/commands/buc-thing.md").exists(),
            "the kit's command must be removed", //
        );
        assert!(
            project_dir
                .join(".claude/commands/xyc-operator.md")
                .is_file(),
            "an asset matching no kit cipher must survive", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// A target whose parcel carried vvk vacates the same way and takes the
    /// canonical binary with it. Paired with the non-vvk case above, this is
    /// what proves the procedure is kit-set-agnostic: the only difference
    /// between the two is the binary-removal step, which is guarded.
    #[test]
    fn bpee_vacate_vvk_target_removes_canonical_binary() {
        let temp_dir = std::env::temp_dir().join("bpee_test_vacate_vvk");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let (burc_path, project_dir) = zbpee_installed_target(&temp_dir, &["buk", "vvk"], true);

        let result = bpee_vacate(&bpee_VacateArgs {
            burc_path,
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
        })
        .expect("a target holding vvk must vacate");

        assert_eq!(result.kits_removed, vec!["buk", "vvk"]);
        assert_eq!(result.commands_removed, 2);
        assert_eq!(result.hooks_removed, 2);
        assert_eq!(
            result.files_deleted, 1,
            "the canonical binary", //
        );
        assert_eq!(
            result.removed.len(),
            1,
            "the vacate names what it removed", //
        );
        assert!(
            result.removed[0].ends_with("Tools/vvk/bin/vvx"),
            "{:?}",
            result.removed, //
        );

        assert!(
            !project_dir.join("Tools/vvk/bin/vvx").exists(),
            "canonical binary must be removed", //
        );
        assert!(
            project_dir.join("Tools/vvk/marker.txt").is_file(),
            "the vvk kit directory itself is preserved", //
        );
        assert!(!project_dir.join(BPEE_TAMGA_DIR).exists());
        assert!(
            project_dir
                .join(".claude/commands/xyc-operator.md")
                .is_file(),
            "an asset matching no kit cipher must survive", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Write `body` at `kit_dir/rel`, creating its parents.
    fn zbpee_plant(kit_dir: &Path, rel: &str, body: &str) {
        let path = kit_dir.join(rel);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, body).unwrap();
    }

    /// One edition of a bash kit: a regime module enrolling `vars` and a git
    /// module defining `gate` beside a private helper.
    fn zbpee_plant_edition(kit_dir: &Path, vars: &[&str], gate: &str) {
        let mut regime = String::from(
            "zburc_kindle() {\n  buv_regime_enroll BURC\n  buv_group_enroll \"Structure\"\n",
        );
        for var in vars {
            regime.push_str(&format!(
                "  buv_string_enroll  {}  1  64  \"a setting\"\n",
                var, //
            ));
        }
        regime.push_str("}\n");
        zbpee_plant(kit_dir, "burc_regime.sh", &regime);
        zbpee_plant(
            kit_dir,
            "bug_git.sh",
            &format!(
                "zbug_helper() {{\n  :\n}}\n\n{}() {{\n  zbug_helper\n}}\n",
                gate, //
            ),
        );
    }

    /// Hurdle: an install over an older edition names what the consumer's
    /// own files owe — here a newly required regime variable and a public
    /// gate renamed — read off the two editions' kit files.
    #[test]
    fn bpee_emplace_report_names_what_the_consumer_owes() {
        let temp_dir = std::env::temp_dir().join("bpee_test_owed_report");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_plant_edition(
            &parcel_dir.join("kits/buk"),
            &["BURC_TOOLS_DIR", "BURC_FRESH_SETTING"],
            "bug_require_tidy_creed",
        );
        fs::write(
            parcel_dir.join(BPEE_TAMGA_FILE),
            r#"{"vvmt_tamga":"1101","vvmk_kits":["buk"]}"#,
        )
        .unwrap();

        let project_dir = temp_dir.join("target");
        zbpee_plant_edition(
            &project_dir.join("Tools/buk"),
            &["BURC_TOOLS_DIR"],
            "bug_require_tidy",
        );
        zbpee_plant(
            &project_dir.join("Tools/buk"),
            "bux_retired.sh",
            "bux_retired() {\n  :\n}\n",
        );
        zbpee_plant(
            &project_dir.join(BPEE_TAMGA_DIR),
            BPEE_TAMGA_FILE,
            r#"{"vvmt_tamga":"1099","vvmk_kits":["buk"]}"#,
        );
        zbpee_init_clean_git_repo(&project_dir);
        let burc_path = zbpee_stage_burc(&temp_dir, "buk");

        let result = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect("an install over an older edition must emplace");

        assert_eq!(result.prior_number.as_deref(), Some("1099"));
        let report = result.owed.join("\n");
        let names = |needle: &str| result.owed.iter().any(|l| l.contains(needle));
        assert!(
            names("BURC_FRESH_SETTING"),
            "the new required variable is named:\n{}",
            report, //
        );
        assert!(
            names("bug_require_tidy "),
            "the vanished gate is named:\n{}",
            report, //
        );
        assert!(
            names("bug_require_tidy_creed"),
            "its candidate successor is named:\n{}",
            report, //
        );
        assert!(
            !names("zbug_helper"),
            "a private function is never reported:\n{}",
            report, //
        );
        assert!(
            names("stop sourcing bux_retired.sh"),
            "the vanished module is named:\n{}",
            report, //
        );
        assert!(
            !names("call to bux_retired "),
            "a vanished module's functions ride its own line:\n{}",
            report, //
        );
        assert!(
            !names("BURC_TOOLS_DIR"),
            "an unchanged variable is never reported:\n{}",
            report, //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a first install has no prior edition to compare, and the
    /// report says so rather than reporting success alone.
    #[test]
    fn bpee_emplace_report_says_no_prior_edition_stood() {
        let temp_dir = std::env::temp_dir().join("bpee_test_owed_first_install");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_plant_edition(
            &parcel_dir.join("kits/buk"),
            &["BURC_TOOLS_DIR"],
            "bug_require_tidy",
        );
        fs::write(
            parcel_dir.join(BPEE_TAMGA_FILE),
            r#"{"vvmt_tamga":"1101","vvmk_kits":["buk"]}"#,
        )
        .unwrap();

        let project_dir = temp_dir.join("target");
        zbpee_plant(&project_dir.join("Tools"), "README.md", "consumer tree");
        zbpee_init_clean_git_repo(&project_dir);
        let burc_path = zbpee_stage_burc(&temp_dir, "buk");

        let result = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect("a first install must emplace");

        assert_eq!(result.prior_number, None);
        assert!(
            result.owed.iter().any(|l| l.contains("no prior edition")),
            "the report says no prior edition stood: {:?}",
            result.owed, //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Commit whatever stands dirty in the repository at `dir`, so a target
    /// shaped after its first commit passes the clean-tree gate.
    fn zbpee_recommit(dir: &Path) {
        for args in [&["add", "-A"][..], &["commit", "-q", "-m", "reshaped"][..]] {
            let output = Command::new("git")
                .args(args)
                .current_dir(dir)
                .output()
                .expect("git invocation failed");
            assert!(
                output.status.success(),
                "git {:?} failed: {}",
                args,
                String::from_utf8_lossy(&output.stderr), //
            );
        }
    }

    /// The kit directories standing under a target's tools directory.
    fn zbpee_standing_dirs(tools_dir: &Path) -> Vec<String> {
        let mut dirs: Vec<String> = fs::read_dir(tools_dir)
            .unwrap()
            .flatten()
            .filter(|e| e.path().is_dir())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        dirs.sort();
        dirs
    }

    /// Hurdle: a narrowing install over a wider edition. The standing record
    /// names three kits; the target now declares two, and the parcel cut for
    /// it carries one. The kit the target stopped declaring leaves with its
    /// command and hook, the declared kit the parcel does not carry stands
    /// byte-untouched, and the receiver's own directory under the tools
    /// directory survives — so the declared kits alone remain. A standing
    /// record that does not read removes nothing beyond the taken set, and the
    /// report says so.
    #[test]
    fn bpee_emplace_narrowing_install_leaves_the_declared_kits_alone() {
        let temp_dir = std::env::temp_dir().join("bpee_test_narrowing_install");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_parcel(
            &parcel_dir,
            &["quobk"],
            r#"{"vvmt_tamga":"1102","vvmk_kits":["quobk"]}"#,
        );

        let project_dir = temp_dir.join("target");
        zbpee_plant(
            &project_dir.join("Tools/frobnix"),
            "own.txt",
            "RECEIVER'S OWN",
        );
        let (_, _) = zbpee_installed_target(&temp_dir, &["quobk", "zentk", "marfk"], false);
        let burc_path = zbpee_stage_burc(&temp_dir, "quobk,marfk");

        let result = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir: parcel_dir.clone(),
            burc_path,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect("a narrowing install must emplace");

        let tools_dir = project_dir.join("Tools");
        assert_eq!(
            zbpee_standing_dirs(&tools_dir),
            vec!["frobnix", "marfk", "quobk"],
            "the declared kits and the receiver's own directory alone remain", //
        );
        assert_eq!(
            fs::read(tools_dir.join("quobk/marker.txt")).unwrap(),
            b"PARCEL quobk"
        );
        assert_eq!(
            fs::read(tools_dir.join("marfk/marker.txt")).unwrap(),
            b"KIT CONTENT",
            "a declared kit the parcel does not carry stands byte-untouched", //
        );
        assert_eq!(
            fs::read(tools_dir.join("frobnix/own.txt")).unwrap(),
            b"RECEIVER'S OWN",
            "the receiver's own directory survives", //
        );
        let commands = project_dir.join(".claude/commands");
        let hooks = project_dir.join(".claude/hooks");
        assert!(
            !commands.join("zentc-thing.md").exists(),
            "the dropped kit's command leaves", //
        );
        assert!(
            !hooks.join("zenth-thing.md").exists(),
            "the dropped kit's hook leaves", //
        );
        assert!(
            commands.join("marfc-thing.md").is_file(),
            "an untouched kit's command stands", //
        );
        assert!(
            commands.join("xyc-operator.md").is_file(),
            "the receiver's own command stands", //
        );
        assert_eq!(
            bpee_tamga_kits(|seat| fs::read_to_string(project_dir.join(seat)).ok()),
            vec!["quobk"]
        );
        assert!(
            result.owed.iter().any(|l| l
                == "zentk: no longer declared - removed its kit directory, 1 command(s) and 1 hook(s); \
                    your own launchers, doors and context lines over it stay yours to remove"),
            "the report names the dropped kit: {:?}",
            result.owed, //
        );

        // A STANDING RECORD OF A FORMAT THIS DOOR DOES NOT READ names nothing.
        let unread_dir = temp_dir.join("unread");
        fs::create_dir_all(&unread_dir).unwrap();
        let (_, unread_project) = zbpee_installed_target(&unread_dir, &["quobk", "zentk"], false);
        fs::write(
            unread_project.join(BPEE_TAMGA_RELPATH),
            r#"{"plinth":1099,"kits":"quobk zentk"}"#,
        )
        .unwrap();
        zbpee_recommit(&unread_project);
        let unread_burc = zbpee_stage_burc(&unread_dir, "quobk");

        let unread = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path: unread_burc,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect("an install over an unreadable record must emplace");

        assert!(
            unread_project.join("Tools/zentk/marker.txt").is_file(),
            "an unreadable record removes nothing beyond the taken set", //
        );
        assert!(
            unread.owed.iter().any(|l| l
                == "cannot remove: the prior record (.vvm/vvmt_tamga.json) does not read, \
                    so no kit this target stopped declaring could be named - only the taken kits were replaced"),
            "the report says the record did not read: {:?}",
            unread.owed, //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Stage a parcel whose one kit carries what a target carrying no
    /// repository is laid from: a trampoline naming its moorings and launcher
    /// directory, a door naming a launcher, and the workbench it reaches.
    fn zbpee_stage_bare_parcel(parcel_dir: &Path) {
        let kit = parcel_dir.join("kits/plimk");
        zbpee_plant(
            &kit,
            "tt/z-trampo.sh",
            "#!/bin/bash\nz_moorings_dir=\"plim_moorings\"\nexec \"${z_moorings_dir}/plim_launchers/${2%% *}\" \"$@\"\n",
        );
        zbpee_plant(
            &kit,
            "tt/plw-x.Thing.sh",
            "#!/bin/bash\nset -- 'launcher.plw_workbench.sh buuc_nolog' \"$@\"\nexec \"${BASH_SOURCE[0]%/*}/z-trampo.sh\" \"${0##*/}\" \"${@}\"\n",
        );
        zbpee_plant(&kit, "plw_workbench.sh", "#!/bin/bash\n");
        fs::write(
            parcel_dir.join(BPEE_TAMGA_FILE),
            r#"{"vvmt_tamga":"1102","vvmk_kits":["plimk"]}"#,
        )
        .unwrap();
    }

    /// Hurdle: a seat is emptied but for its repository and laid whole. A
    /// first lay over an empty directory says nothing stood; a stale door in
    /// its tabtarget directory and a stale launcher directory in its moorings
    /// are gone after a re-lay, which names what it cleared; the declaration
    /// it writes carries no template header block; and a non-empty directory
    /// holding no record is refused, untouched.
    #[test]
    fn bpee_emplace_bare_target_laid_whole_behind_the_tamga_check() {
        let temp_dir = std::env::temp_dir().join("bpee_test_bare_target_whole");
        let _ = fs::remove_dir_all(&temp_dir);
        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_bare_parcel(&parcel_dir);
        let lay = |target: &Path| {
            bpee_emplace(&bpee_EmplaceArgs {
                parcel_dir: parcel_dir.clone(),
                burc_path: target.to_path_buf(),
                precinct_rosters: Vec::new(),
                precinct_freight: Vec::new(),
                signets: zbpee_test_signets(),
                binary: Some(zbpee_test_binary()),
                seat_endonym: Some("buon_plimseat".to_string()),
            })
        };

        let seat = temp_dir.join("seat");
        fs::create_dir_all(&seat).unwrap();
        let first = lay(&seat).expect("a first lay over an empty directory");
        assert!(
            first
                .owed
                .iter()
                .any(|l| l == "the target stood empty - a first lay, nothing to clear"),
            "the first lay says nothing stood: {:?}",
            first.owed, //
        );

        let declaration = fs::read_to_string(seat.join("plim_moorings/burc.env")).unwrap();
        assert!(
            declaration.lines().any(|l| l == "BURC_MANAGED_KITS=plimk"),
            "{}",
            declaration, //
        );
        assert!(
            declaration
                .lines()
                .any(|l| l == "BURC_ENDONYM=buon_plimseat"),
            "the declaration states the BURC_ENDONYM value its caller named: {}",
            declaration, //
        );
        assert!(
            !declaration.contains('\u{ab}'),
            "the declaration carries no placeholder mark: {}",
            declaration, //
        );
        assert!(
            !declaration
                .lines()
                .any(|l| l.len() > 1 && l.chars().all(|c| c == '#')),
            "the declaration carries no header block's opening rule:\n{}",
            declaration, //
        );
        assert!(
            !declaration.contains("delete this template header block"),
            "the declaration carries no header block:\n{}",
            declaration, //
        );

        let launcher_template =
            fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../bpt_launcher.sh"))
                .unwrap();
        let (template_first, template_rest) = launcher_template
            .split_once('\n')
            .expect("the launcher template has a first line");
        let (_, template_body) = template_rest
            .split_once("\n\n")
            .expect("the launcher template's header ends at a blank line");
        let expected_stub = format!(
            "{}\n{}",
            template_first,
            template_body.replace("buk/buw_workbench.sh", "plimk/plw_workbench.sh"), //
        );
        let stub =
            fs::read_to_string(seat.join("plim_moorings/plim_launchers/launcher.plw_workbench.sh"))
                .unwrap();
        assert_eq!(stub, expected_stub, "the laid launcher stub is not the template's first line and the lines after its header", //
        );
        assert!(
            !stub.contains('\u{ab}'),
            "the laid launcher stub carries a placeholder mark:\n{}",
            stub, //
        );

        let stale_door = seat.join("tt/plw-q.Stale.sh");
        let stale_launchers = seat.join("plim_moorings/old_launchers");
        zbpee_plant(&seat, "tt/plw-q.Stale.sh", "#!/bin/bash\n");
        zbpee_plant(&stale_launchers, "launcher.gone.sh", "#!/bin/bash\n");

        let second = lay(&seat).expect("a re-lay over a recorded directory");
        assert!(
            !stale_door.exists(),
            "the stale door is gone after the re-lay", //
        );
        assert!(
            !stale_launchers.exists(),
            "the stale launcher directory is gone after the re-lay", //
        );
        assert!(
            seat.join("tt/plw-x.Thing.sh").is_file(),
            "the carried door stands again", //
        );
        assert!(
            seat.join("plim_moorings/plim_launchers/launcher.plw_workbench.sh")
                .is_file(),
            "the launcher stub stands again", //
        );
        assert!(
            second
                .owed
                .iter()
                .any(|l| l == "cleared the earlier lay whole: .vvm, Tools, plim_moorings, tt"),
            "the re-lay names what it cleared: {:?}",
            second.owed, //
        );

        let unrecorded = temp_dir.join("unrecorded");
        zbpee_plant(&unrecorded, "notes.txt", "SOMEONE'S");
        let refused =
            lay(&unrecorded).expect_err("a non-empty directory holding no record refuses");
        assert!(
            refused.contains("is not empty and holds no .vvm/vvmt_tamga.json"),
            "{}",
            refused, //
        );
        assert_eq!(
            fs::read(unrecorded.join("notes.txt")).unwrap(),
            b"SOMEONE'S",
            "and stands untouched", //
        );
        assert!(
            !unrecorded.join("plim_moorings").exists(),
            "nothing was laid over it", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// What git answers in `dir`, trimmed; a git refusal fails the hurdle.
    fn zbpee_git_read(dir: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .args(args)
            .current_dir(dir)
            .output()
            .expect("git invocation failed");
        assert!(
            output.status.success(),
            "git {:?} in {} failed: {}",
            args,
            dir.display(),
            String::from_utf8_lossy(&output.stderr), //
        );
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    /// The manner member of the record standing in `root`.
    fn zbpee_standing_manner(root: &Path) -> serde_json::Value {
        let body = fs::read_to_string(root.join(BPEE_TAMGA_RELPATH)).unwrap();
        serde_json::from_str::<serde_json::Value>(&body).unwrap()[BPEE_TAMGA_MANNER].clone()
    }

    /// Hurdle: a first lay into an empty directory leaves a seat carrying a
    /// repository of its own, holding one commit and no remote, on the line the
    /// studbook's record line is named for, and a record saying the seat form
    /// laid it; the report names the commit.
    #[test]
    fn bpee_emplace_seat_first_lay_inits_a_repository_holding_one_commit() {
        let temp_dir = std::env::temp_dir().join("bpee_test_seat_first_lay");
        let _ = fs::remove_dir_all(&temp_dir);
        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_bare_parcel(&parcel_dir);
        let seat = temp_dir.join("seat");
        fs::create_dir_all(&seat).unwrap();

        let first = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path: seat.clone(),
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: Some("buon_plimseat".to_string()),
        })
        .expect("a first lay over an empty directory");

        assert!(
            seat.join(".git").is_dir(),
            "the first lay inits a repository in the seat", //
        );
        assert_eq!(
            zbpee_git_read(&seat, &["rev-list", "--count", "HEAD"]),
            "1",
            "one commit after a first lay", //
        );
        assert_eq!(
            zbpee_git_read(&seat, &["remote"]),
            "",
            "and no remote", //
        );
        assert_eq!(
            zbpee_git_read(&seat, &["symbolic-ref", "--short", "HEAD"]),
            "trunk",
            "on the record line's name", //
        );
        assert_eq!(
            zbpee_git_read(&seat, &["status", "--short"]),
            "",
            "the lay stands committed whole", //
        );
        assert_eq!(
            zbpee_standing_manner(&seat),
            "directory",
            "the record says the seat form laid it", //
        );
        let head = zbpee_git_read(&seat, &["rev-parse", "HEAD"]);
        assert_eq!(
            first.commit.as_deref(),
            Some(head.as_str()),
            "the report names the commit it made", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a second lay over a seat the seat form laid commits again,
    /// leaving two commits and a clean tree, and its clear spares the
    /// repository's own directory, which the cleared list does not name.
    #[test]
    fn bpee_emplace_seat_relay_commits_again_sparing_the_repository() {
        let temp_dir = std::env::temp_dir().join("bpee_test_seat_relay");
        let _ = fs::remove_dir_all(&temp_dir);
        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_bare_parcel(&parcel_dir);
        let seat = temp_dir.join("seat");
        fs::create_dir_all(&seat).unwrap();
        let lay = || {
            bpee_emplace(&bpee_EmplaceArgs {
                parcel_dir: parcel_dir.clone(),
                burc_path: seat.clone(),
                precinct_rosters: Vec::new(),
                precinct_freight: Vec::new(),
                signets: zbpee_test_signets(),
                binary: Some(zbpee_test_binary()),
                seat_endonym: Some("buon_plimseat".to_string()),
            })
        };

        lay().expect("a first lay over an empty directory");
        zbpee_plant(&seat, "tt/plw-q.Stale.sh", "#!/bin/bash\n");
        let second = lay().expect("a re-lay over a seat the seat form laid");

        assert_eq!(
            zbpee_git_read(&seat, &["rev-list", "--count", "HEAD"]),
            "2",
            "two commits after a re-lay", //
        );
        assert_eq!(
            zbpee_git_read(&seat, &["status", "--short"]),
            "",
            "the re-lay stands committed whole", //
        );
        assert!(
            !seat.join("tt/plw-q.Stale.sh").exists(),
            "the stale door is gone", //
        );
        assert!(
            second
                .owed
                .iter()
                .any(|l| l == "cleared the earlier lay whole: .vvm, Tools, plim_moorings, tt"),
            "the cleared list names the lay and never the repository's directory: {:?}",
            second.owed, //
        );
        assert_eq!(
            zbpee_git_read(&seat, &["remote"]),
            "",
            "still no remote", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a seat laid with no endonym named is refused ahead of the lay,
    /// naming the seat, and the directory stands untouched.
    #[test]
    fn bpee_emplace_seat_refuses_a_lay_naming_no_endonym() {
        let temp_dir = std::env::temp_dir().join("bpee_test_seat_unnamed");
        let _ = fs::remove_dir_all(&temp_dir);
        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_bare_parcel(&parcel_dir);
        let seat = temp_dir.join("seat");
        fs::create_dir_all(&seat).unwrap();

        let refused = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path: seat.clone(),
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect_err("a seat laid naming no BURC_ENDONYM value refuses");
        assert!(
            refused.contains("No BURC_ENDONYM value was named for the seat"),
            "{}",
            refused, //
        );
        assert_eq!(
            fs::read_dir(&seat).unwrap().count(),
            0,
            "the seat stands untouched", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a repository the repository form installed into, its record
    /// saying so, is refused when named by its path, and stands untouched.
    #[test]
    fn bpee_emplace_seat_refuses_a_repository_named_by_its_path() {
        let temp_dir = std::env::temp_dir().join("bpee_test_seat_refuses_repository");
        let _ = fs::remove_dir_all(&temp_dir);
        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_bare_parcel(&parcel_dir);

        let project_dir = temp_dir.join("target");
        fs::create_dir_all(project_dir.join("Tools")).unwrap();
        fs::write(project_dir.join("README"), b"a workload").unwrap();
        zbpee_init_clean_git_repo(&project_dir);
        bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir: parcel_dir.clone(),
            burc_path: zbpee_stage_burc(&temp_dir, "plimk"),
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect("the repository form installs");
        assert_eq!(
            zbpee_standing_manner(&project_dir),
            "repository",
            "the record says the repository form laid it", //
        );
        zbpee_recommit(&project_dir);

        let refused = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path: project_dir.clone(),
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: Some("buon_plimseat".to_string()),
        })
        .expect_err("a repository named by its path refuses");
        assert!(
            refused.contains("holds a .vvm/vvmt_tamga.json the seat form did not write"),
            "{}",
            refused, //
        );
        assert_eq!(
            fs::read(project_dir.join("README")).unwrap(),
            b"a workload",
            "the workload stands", //
        );
        assert!(
            !project_dir.join("plim_moorings").exists(),
            "no regime was written into it", //
        );
        assert_eq!(
            zbpee_git_read(&project_dir, &["rev-list", "--count", "HEAD"]),
            "2",
            "and nothing was committed in it", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// The keys of a record body, sorted, for asserting which spelling it wears.
    fn zbpee_keys_of(path: &Path) -> Vec<String> {
        let body = fs::read_to_string(path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        json.as_object().unwrap().keys().cloned().collect()
    }

    /// A target laid by a repository install, its standing record planted at
    /// `seat` with `body`, declaring `declared` and carrying `kits` under Tools.
    fn zbpee_tamga_stood_target(
        temp_dir: &Path,
        kits: &[&str],
        declared: &str,
        seat: &str,
        body: &str,
    ) -> (PathBuf, PathBuf) {
        let project_dir = temp_dir.join("target");
        for kit in kits {
            zbpee_plant(
                &project_dir.join("Tools").join(kit),
                "marker.txt",
                "PRIOR EDITION",
            );
        }
        let (dir, file) = seat.rsplit_once('/').unwrap();
        zbpee_plant(&project_dir.join(dir), file, body);
        zbpee_init_clean_git_repo(&project_dir);
        (zbpee_stage_burc(temp_dir, declared), project_dir)
    }

    /// Hurdle: an install over a record of the ELDER spelling — the shape a
    /// consumer installed before the current one carries, its forge unnamed and
    /// its manner absent — reads the prior record and the kits it names, removes
    /// the kit the target stopped declaring, takes the elder directory away, and
    /// writes the current shape alone.
    #[test]
    fn bpee_emplace_over_an_elder_tamga_writes_the_current_shape_alone() {
        let temp_dir = std::env::temp_dir().join("bpee_test_over_elder_tamga");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_parcel(
            &parcel_dir,
            &["buk"],
            r#"{"vvmt_tamga":"1102","vvmd_date":"261022-1200","vvmc_commit":"abc123","vvmf_forge":"buon_quorge","vvmr_target":"target","vvmk_kits":["buk"]}"#,
        );
        let (burc_path, project_dir) = zbpee_tamga_stood_target(
            &temp_dir,
            &["bkk", "buk"],
            "buk",
            BPEE_ELDER_RELPATH,
            r#"{"vvbc_commit":"dbc7197d8","vvbd_date":"261015-2154","vvbf_brand":"1085","vvbk_kits":["bkk","buk"],"vvbt_target":"target"}"#,
        );

        let result = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect("an install over an elder record must emplace");

        assert_eq!(
            result.prior_number.as_deref(),
            Some("1085"),
            "the elder record is read as the prior edition", //
        );
        assert!(
            !project_dir.join("Tools/bkk").exists(),
            "the kit the elder record named and the target dropped leaves", //
        );
        assert!(
            !project_dir.join(BPEE_ELDER_DIR).exists(),
            "no elder directory remains", //
        );
        let written = project_dir.join(BPEE_TAMGA_RELPATH);
        assert_eq!(
            zbpee_keys_of(&written),
            vec![
                "vvmc_commit",
                "vvmd_date",
                "vvmf_forge",
                "vvmk_kits",
                "vvmm_manner",
                "vvmr_target",
                "vvmt_tamga"
            ],
            "the install writes the current spelling and no elder key", //
        );
        assert_eq!(
            bpee_tamga_kits(|seat| fs::read_to_string(project_dir.join(seat)).ok()),
            vec!["buk"]
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: an install over a record of the CURRENT spelling reads it as the
    /// prior edition and writes the current shape alone, leaving no elder
    /// directory where none stood.
    #[test]
    fn bpee_emplace_over_a_current_tamga_writes_the_current_shape_alone() {
        let temp_dir = std::env::temp_dir().join("bpee_test_over_current_tamga");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_parcel(
            &parcel_dir,
            &["buk"],
            r#"{"vvmt_tamga":"1103","vvmf_forge":"buon_quorge","vvmk_kits":["buk"]}"#,
        );
        let (burc_path, project_dir) = zbpee_tamga_stood_target(
            &temp_dir,
            &["bkk", "buk"],
            "buk",
            BPEE_TAMGA_RELPATH,
            r#"{"vvmt_tamga":"1102","vvmf_forge":"buon_quorge","vvmk_kits":["bkk","buk"],"vvmm_manner":"repository"}"#,
        );

        let result = bpee_emplace(&bpee_EmplaceArgs {
            parcel_dir,
            burc_path,
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
            seat_endonym: None,
        })
        .expect("an install over a current record must emplace");

        assert_eq!(result.prior_number.as_deref(), Some("1102"));
        assert!(
            !project_dir.join("Tools/bkk").exists(),
            "the dropped kit leaves", //
        );
        assert!(
            !project_dir.join(BPEE_ELDER_DIR).exists(),
            "no elder directory is written", //
        );
        assert_eq!(
            zbpee_keys_of(&project_dir.join(BPEE_TAMGA_RELPATH)),
            vec!["vvmf_forge", "vvmk_kits", "vvmm_manner", "vvmt_tamga"]
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a seat laid before the current shape holds the elder directory,
    /// its record carrying the seat form's manner under the elder key, and must
    /// read as a re-lay, since a refusal there would refuse the operator's
    /// rebuild of the seat. The re-lay clears the elder directory and writes the
    /// current shape.
    #[test]
    fn bpee_emplace_seat_laid_in_the_elder_shape_reads_as_a_relay() {
        let temp_dir = std::env::temp_dir().join("bpee_test_seat_elder_relay");
        let _ = fs::remove_dir_all(&temp_dir);
        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_bare_parcel(&parcel_dir);
        let seat = temp_dir.join("seat");
        fs::create_dir_all(&seat).unwrap();
        let lay = || {
            bpee_emplace(&bpee_EmplaceArgs {
                parcel_dir: parcel_dir.clone(),
                burc_path: seat.clone(),
                precinct_rosters: Vec::new(),
                precinct_freight: Vec::new(),
                signets: zbpee_test_signets(),
                binary: Some(zbpee_test_binary()),
                seat_endonym: Some("buon_plimseat".to_string()),
            })
        };

        lay().expect("a first lay over an empty directory");
        fs::remove_dir_all(seat.join(BPEE_TAMGA_DIR)).unwrap();
        let (dir, file) = BPEE_ELDER_RELPATH.rsplit_once('/').unwrap();
        zbpee_plant(
            &seat.join(dir),
            file,
            r#"{"vvbf_brand":"1101","vvbk_kits":["plimk"],"vvbm_manner":"directory"}"#,
        );
        zbpee_recommit(&seat);

        let second = lay().expect("a seat laid in the elder shape re-lays");

        assert!(
            !seat.join(BPEE_ELDER_DIR).exists(),
            "the elder directory is cleared", //
        );
        assert_eq!(
            zbpee_standing_manner(&seat),
            "directory",
            "the current record says the seat form laid it", //
        );
        assert_eq!(
            second.prior_number.as_deref(),
            Some("1101"),
            "the elder record is read as the prior edition", //
        );
        assert!(
            second
                .owed
                .iter()
                .any(|l| l == "cleared the earlier lay whole: .vvk, Tools, plim_moorings, tt"),
            "the cleared list names the elder directory: {:?}",
            second.owed, //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: the vacate finds a record of the elder spelling, removes what its
    /// kits name, and takes the elder directory away.
    #[test]
    fn bpee_vacate_reads_an_elder_tamga() {
        let temp_dir = std::env::temp_dir().join("bpee_test_vacate_elder");
        let _ = fs::remove_dir_all(&temp_dir);
        let (burc_path, project_dir) = zbpee_tamga_stood_target(
            &temp_dir,
            &["buk"],
            "buk",
            BPEE_ELDER_RELPATH,
            r#"{"vvbf_brand":"1085","vvbk_kits":["buk"]}"#,
        );

        let result = bpee_vacate(&bpee_VacateArgs {
            burc_path,
            signets: zbpee_test_signets(),
            binary: Some(zbpee_test_binary()),
        })
        .expect("a vacate over an elder record");

        assert_eq!(result.kits_removed, vec!["buk"]);
        assert!(
            !project_dir.join(BPEE_ELDER_DIR).exists(),
            "the elder directory leaves with its record", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// The behest a forge composes for a target taking the kit whose seat holds
    /// a precinct, naming the binary or not, with signets the caller chooses so
    /// a routing that obeys it can only have read it.
    fn zbpee_test_behest(signets: bpee_Signets, binary: Option<bpee_Binary>) -> bpee_Behest {
        bpee_Behest {
            precinct_rosters: vec![PathBuf::from("plim_precinct/plim_rosters")],
            precinct_freight: vec![bpee_Freight {
                from: PathBuf::from("plimk/plim_freight"),
                to: PathBuf::from("plim_precinct/plim_rosters/plim_entry"),
            }],
            signets,
            binary,
            seat_endonym: Some("buon_plimseat".to_string()),
        }
    }

    /// The behest a forge composes for a target taking neither the binary's kit
    /// nor the kit whose seat holds a precinct: the signets alone carry a value.
    fn zbpee_test_narrowed_behest(signets: bpee_Signets) -> bpee_Behest {
        bpee_Behest {
            precinct_rosters: Vec::new(),
            precinct_freight: Vec::new(),
            signets,
            binary: None,
            seat_endonym: None,
        }
    }

    fn zbpee_write_behest(parcel_dir: &Path, behest: &bpee_Behest) {
        fs::create_dir_all(parcel_dir).unwrap();
        fs::write(
            parcel_dir.join(BPEE_BEHEST_FILE),
            serde_json::to_vec_pretty(&bpee_behest_json(behest).unwrap()).unwrap(),
        )
        .unwrap();
    }

    /// Hurdle: the behest reads back as it was written, every member kept,
    /// and a behest naming no binary reads back naming none.
    #[test]
    fn bpee_behest_reads_back_as_written() {
        let temp_dir = std::env::temp_dir().join("bpee_test_behest_round_trip");
        let _ = fs::remove_dir_all(&temp_dir);

        let named = zbpee_test_behest(zbpee_test_signets(), Some(zbpee_test_binary()));
        zbpee_write_behest(&temp_dir.join("named"), &named);
        assert_eq!(zbpee_read_behest(&temp_dir.join("named")).unwrap(), named);

        let unnamed = zbpee_test_behest(zbpee_test_signets(), None);
        zbpee_write_behest(&temp_dir.join("unnamed"), &unnamed);
        assert_eq!(
            zbpee_read_behest(&temp_dir.join("unnamed")).unwrap(),
            unnamed
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a behest missing a member is refused naming the member, never
    /// read with a default in its place.
    #[test]
    fn bpee_behest_missing_a_member_is_refused_naming_it() {
        let temp_dir = std::env::temp_dir().join("bpee_test_behest_member");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let mut json = bpee_behest_json(&zbpee_test_behest(zbpee_test_signets(), None)).unwrap();
        json.as_object_mut().unwrap().remove(ZBPEE_BEHEST_NAME);
        fs::write(temp_dir.join(BPEE_BEHEST_FILE), json.to_string()).unwrap();

        let err = zbpee_read_behest(&temp_dir).expect_err("a behest missing its seat's name");
        assert!(
            err.contains(BPEE_BEHEST_FILE) && err.contains(ZBPEE_BEHEST_NAME),
            "{}",
            err, //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a behest naming neither a binary nor a seat still carries every
    /// member, the binary and the name spelled null and the precinct's lists
    /// empty, and reads back as it was written.
    #[test]
    fn bpee_behest_naming_neither_binary_nor_seat_reads_back_as_written() {
        let temp_dir = std::env::temp_dir().join("bpee_test_behest_narrowed");
        let _ = fs::remove_dir_all(&temp_dir);

        let narrowed = zbpee_test_narrowed_behest(zbpee_test_signets());
        let wire = bpee_behest_json(&narrowed).unwrap();
        for member in [
            ZBPEE_BEHEST_DIRECTORIES,
            ZBPEE_BEHEST_FREIGHT,
            ZBPEE_BEHEST_SUFFIXES,
            ZBPEE_BEHEST_BINARY,
            ZBPEE_BEHEST_NAME,
        ] {
            assert!(
                wire.get(member).is_some(),
                "the behest carries its member '{}': {}",
                member,
                wire, //
            );
        }
        assert!(
            wire[ZBPEE_BEHEST_BINARY].is_null(),
            "no binary is spelled null: {}",
            wire, //
        );
        assert!(
            wire[ZBPEE_BEHEST_NAME].is_null(),
            "no seat's name is spelled null: {}",
            wire, //
        );
        assert_eq!(
            wire[ZBPEE_BEHEST_DIRECTORIES],
            serde_json::json!([]),
            "no precinct stands no directory", //
        );
        assert_eq!(
            wire[ZBPEE_BEHEST_FREIGHT],
            serde_json::json!([]),
            "no precinct carries no freight", //
        );

        zbpee_write_behest(&temp_dir, &narrowed);
        assert_eq!(zbpee_read_behest(&temp_dir).unwrap(), narrowed);

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a behest spelling its seat's name empty is refused naming the
    /// member, null being the one spelling of a behest naming no seat.
    #[test]
    fn bpee_behest_naming_an_empty_name_is_refused() {
        let temp_dir = std::env::temp_dir().join("bpee_test_behest_empty_name");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let mut json = bpee_behest_json(&zbpee_test_behest(zbpee_test_signets(), None)).unwrap();
        json[ZBPEE_BEHEST_NAME] = serde_json::Value::String(String::new());
        fs::write(temp_dir.join(BPEE_BEHEST_FILE), json.to_string()).unwrap();

        let err = zbpee_read_behest(&temp_dir).expect_err("a behest naming an empty name");
        assert!(
            err.contains(BPEE_BEHEST_FILE) && err.contains(ZBPEE_BEHEST_NAME),
            "{}",
            err, //
        );
        assert!(
            err.contains("null"),
            "the refusal names the one spelling of none: {}",
            err, //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a parcel whose behest names neither a binary nor a seat installs
    /// into a repository exactly as one naming both does — its kit laid, the
    /// behest's suffix routed, no binary emplaced — and is refused as a seat,
    /// naming the missing name, the directory left untouched.
    #[test]
    fn bpee_emplace_parcel_lays_a_narrowed_behest_and_refuses_it_a_seat() {
        let temp_dir = std::env::temp_dir().join("bpee_test_behest_narrowed_emplace");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_parcel(
            &parcel_dir,
            &["buk"],
            r#"{"vvmt_tamga":"1099","vvmk_kits":["buk"]}"#,
        );
        let commands = parcel_dir.join("claude/commands");
        fs::create_dir_all(&commands).unwrap();
        fs::write(commands.join("buq-thing.md"), b"ROUTED").unwrap();
        fs::write(commands.join("buc-thing.md"), b"NOT ROUTED").unwrap();
        let signets = bpee_Signets {
            command: "q-".to_string(),
            hook: "h-".to_string(),
        };
        zbpee_write_behest(&parcel_dir, &zbpee_test_narrowed_behest(signets));

        let project_dir = temp_dir.join("target");
        fs::create_dir_all(project_dir.join("Tools")).unwrap();
        fs::write(project_dir.join("README.md"), b"a consumer\n").unwrap();
        zbpee_init_clean_git_repo(&project_dir);
        let burc_path = zbpee_stage_burc(&temp_dir, "buk");

        let result = bpee_emplace_parcel(&parcel_dir, &burc_path)
            .expect("a narrowed behest lays a repository");
        assert_eq!(result.kits_installed, vec!["buk"]);
        assert_eq!(
            result.canonical_platform, None,
            "no binary is emplaced", //
        );
        assert_eq!(
            result.regime, None,
            "a repository's regime is its owner's", //
        );
        assert_eq!(result.commands_routed, 1);
        assert!(
            project_dir.join(".claude/commands/buq-thing.md").is_file(),
            "the behest's suffix routes", //
        );
        assert!(
            !project_dir.join(".claude/commands/buc-thing.md").exists(),
            "a suffix the behest does not name is left", //
        );
        assert!(project_dir.join("Tools/buk/marker.txt").is_file());

        let seat = temp_dir.join("seat");
        fs::create_dir_all(&seat).unwrap();
        let refused =
            bpee_emplace_parcel(&parcel_dir, &seat).expect_err("a narrowed behest lays no seat");
        assert!(
            refused.contains("No BURC_ENDONYM value was named for the seat"),
            "{}",
            refused, //
        );
        assert_eq!(
            fs::read_dir(&seat).unwrap().count(),
            0,
            "the seat stands untouched", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a parcel carrying no behest is refused at install, naming the
    /// file, before anything in the target is touched.
    #[test]
    fn bpee_emplace_parcel_refuses_a_parcel_carrying_no_behest() {
        let temp_dir = std::env::temp_dir().join("bpee_test_no_behest_emplace");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_parcel(
            &parcel_dir,
            &["buk"],
            r#"{"vvmt_tamga":"1099","vvmk_kits":["buk"]}"#,
        );

        let project_dir = temp_dir.join("target");
        let tools_dir = project_dir.join("Tools");
        fs::create_dir_all(tools_dir.join("buk")).unwrap();
        fs::write(tools_dir.join("buk/marker.txt"), b"BUK UNTOUCHED").unwrap();
        zbpee_init_clean_git_repo(&project_dir);
        let burc_path = zbpee_stage_burc(&temp_dir, "buk");

        let err =
            bpee_emplace_parcel(&parcel_dir, &burc_path).expect_err("a parcel carrying no behest");
        assert!(
            err.contains(BPEE_BEHEST_FILE),
            "the refusal names the file: {}",
            err, //
        );
        assert_eq!(
            fs::read(tools_dir.join("buk/marker.txt")).unwrap(),
            b"BUK UNTOUCHED"
        );
        assert!(
            !project_dir.join(BPEE_TAMGA_DIR).exists(),
            "nothing may be written", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: a parcel carrying no behest is refused at uninstall, naming the
    /// file, its target's record and assets left standing.
    #[test]
    fn bpee_vacate_parcel_refuses_a_parcel_carrying_no_behest() {
        let temp_dir = std::env::temp_dir().join("bpee_test_no_behest_vacate");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(temp_dir.join("parcel")).unwrap();
        let (burc_path, project_dir) = zbpee_installed_target(&temp_dir, &["buk"], false);

        let err = bpee_vacate_parcel(&temp_dir.join("parcel"), &burc_path)
            .expect_err("a parcel carrying no behest");
        assert!(
            err.contains(BPEE_BEHEST_FILE),
            "the refusal names the file: {}",
            err, //
        );
        assert!(
            project_dir
                .join(BPEE_TAMGA_DIR)
                .join(BPEE_TAMGA_FILE)
                .is_file(),
            "what the install laid stands", //
        );
        assert!(
            project_dir.join(".claude/commands/buc-thing.md").is_file(),
            "the kit's command stands", //
        );

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: the install reads the behest and lays a parcel carrying no vvk.
    /// The behest names the vvk binary and signets the parcel's kit wears, so
    /// the asset it names is routed and the one it does not is left; no vvk
    /// directory and no binary appear in the target.
    #[test]
    fn bpee_emplace_parcel_reads_the_behest_and_lays_a_parcel_carrying_no_vvk() {
        let temp_dir = std::env::temp_dir().join("bpee_test_behest_emplace");
        let _ = fs::remove_dir_all(&temp_dir);

        let parcel_dir = temp_dir.join("parcel");
        zbpee_stage_parcel(
            &parcel_dir,
            &["buk"],
            r#"{"vvmt_tamga":"1099","vvmk_kits":["buk"]}"#,
        );
        let commands = parcel_dir.join("claude/commands");
        fs::create_dir_all(&commands).unwrap();
        fs::write(commands.join("buq-thing.md"), b"ROUTED").unwrap();
        fs::write(commands.join("buc-thing.md"), b"NOT ROUTED").unwrap();
        let signets = bpee_Signets {
            command: "q-".to_string(),
            hook: "h-".to_string(),
        };
        zbpee_write_behest(
            &parcel_dir,
            &zbpee_test_behest(signets, Some(zbpee_test_binary())),
        );

        let project_dir = temp_dir.join("target");
        fs::create_dir_all(project_dir.join("Tools")).unwrap();
        fs::write(project_dir.join("README.md"), b"a consumer\n").unwrap();
        zbpee_init_clean_git_repo(&project_dir);
        let burc_path = zbpee_stage_burc(&temp_dir, "buk");

        let result =
            bpee_emplace_parcel(&parcel_dir, &burc_path).expect("a parcel carrying no vvk lays");
        assert_eq!(result.kits_installed, vec!["buk"]);
        assert_eq!(
            result.canonical_platform, None,
            "no binary is emplaced", //
        );
        assert_eq!(result.commands_routed, 1);
        assert!(
            project_dir.join(".claude/commands/buq-thing.md").is_file(),
            "the behest's suffix routes", //
        );
        assert!(
            !project_dir.join(".claude/commands/buc-thing.md").exists(),
            "a suffix the behest does not name is left", //
        );
        assert!(
            !project_dir.join("Tools/vvk").exists(),
            "no vvk directory appears", //
        );
        assert!(project_dir.join("Tools/buk/marker.txt").is_file());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    /// Hurdle: the vacate reads the behest: the binary it names is removed,
    /// and under a behest naming none the same target keeps its binary.
    #[test]
    fn bpee_vacate_parcel_reads_the_behest() {
        let temp_dir = std::env::temp_dir().join("bpee_test_behest_vacate");
        let _ = fs::remove_dir_all(&temp_dir);

        let named = temp_dir.join("named");
        zbpee_write_behest(
            &named.join("parcel"),
            &zbpee_test_behest(zbpee_test_signets(), Some(zbpee_test_binary())),
        );
        let (burc_path, project_dir) = zbpee_installed_target(&named, &["buk", "vvk"], true);
        let result = bpee_vacate_parcel(&named.join("parcel"), &burc_path)
            .expect("a vacate under its behest");
        assert_eq!(result.kits_removed, vec!["buk", "vvk"]);
        assert_eq!(result.commands_removed, 2);
        assert_eq!(
            result.files_deleted, 1,
            "the binary the behest names is removed", //
        );
        assert!(!project_dir.join("Tools/vvk/bin/vvx").exists());

        let unnamed = temp_dir.join("unnamed");
        zbpee_write_behest(
            &unnamed.join("parcel"),
            &zbpee_test_behest(zbpee_test_signets(), None),
        );
        let (burc_path, project_dir) = zbpee_installed_target(&unnamed, &["buk", "vvk"], true);
        let result = bpee_vacate_parcel(&unnamed.join("parcel"), &burc_path)
            .expect("a vacate under its behest");
        assert_eq!(
            result.files_deleted, 0,
            "a behest naming no binary removes none", //
        );
        assert!(project_dir.join("Tools/vvk/bin/vvx").is_file());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
