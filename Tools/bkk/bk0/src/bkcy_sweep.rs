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

//! The sweep — a collar's build directory sighted, then removed.
//!
//! THE MECHANISM AND NOT THE DOOR. Scoop is the operator's on-demand drive over
//! it and the delouse will be the scheduled one under its own stamp; what is
//! here is the act both perform, so neither can come to hold its own idea of
//! which directory a collar's crate builds into or which one it may delete.
//!
//! THE DIRECTORY IS CARGO'S ANSWER, NEVER A COMPOSITION. Manifest-directory plus
//! `target` is right for most crates and silently wrong for the ones a sweep
//! must not be wrong about — a build directory moved by configuration, a
//! workspace member whose artifacts stand at the workspace root. The resolver
//! already asks cargo about every manifest, so the sighting is one more reading
//! of the account it already takes rather than a second question.
//!
//! SIGHTING IS SEPARATE FROM REMOVING, and the split is the whole safety of this
//! module: every collar is sighted before any directory is removed, so a
//! refusal lands with the yard untouched rather than half-swept. A caller that
//! could remove without sighting would be able to compose that order wrongly;
//! this one cannot, because removal takes only what a sighting yielded.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::bkcr_resolve::{bkcr_target_directory, bkcr_Collar};
use crate::bkcx_python::bkcx_Seat;

/// A collar's build directory, and whether it is one this sweep may delete.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum bkcy_Sighting {
    /// The directory stands under the repository the collar stands in.
    Within(PathBuf),
    /// It stands under the checkout's own loosebox — outside every source tree,
    /// and holding nothing but derived products this sweep exists to drop.
    ///
    /// THE SECOND ADMISSIBLE CLASS, and it is judged against the loosebox the
    /// caller poses rather than against cargo's answer. A build directory is
    /// redirected by configuration and by environment; the loosebox is a place
    /// the substrate composed and named, so a path is a loosebox path because it
    /// stands under that name and never because a tool reported it. The
    /// station's tackroom stands nowhere beneath it and therefore stays
    /// `Beyond`, which is the whole reason the reading is posed a root instead
    /// of a general "outside the tree" licence.
    Loosebox(PathBuf),
    /// It does not, and nothing may be deleted on account of this collar.
    ///
    /// THE REFUSAL IS THE POINT rather than a guard against a mistake nobody
    /// makes. A build directory is redirected by configuration and by
    /// environment, and the station's own tackroom is exactly such a
    /// redirection — so a sweep that trusted cargo's answer without asking where
    /// it landed would one day be handed the shared toolchain store and would
    /// remove it.
    Beyond(PathBuf),
}

/// What one collar's removal came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkcy_Swept {
    /// The collar the directory was sighted for.
    pub collar: String,
    /// The directory, as cargo named it.
    pub directory: PathBuf,
    /// Whether anything stood there to remove. An absent directory is a clean
    /// yard rather than a fault, and saying which is what makes the report worth
    /// reading twice.
    pub stood: bool,
}

/// Sight one collar's build directory: cargo's answer for its manifest, judged
/// against the repository the collar stands in.
pub fn bkcy_sight(
    repository: &Path,
    loosebox: Option<&Path>,
    collar: &bkcr_Collar,
) -> Result<bkcy_Sighting, String> {
    let manifest = repository.join(collar.bkcr_field("BKRR_MANIFEST"));
    let directory = bkcr_target_directory(repository, &manifest)?;

    let root = zbkcy_rooted(repository)?;

    Ok(bkcy_classed(
        &root,
        loosebox,
        &zbkcy_settled(&root.join(&directory)),
        directory,
    ))
}

/// Sight one python collar's environment: the seat's own, judged against the
/// repository the collar stands in and the loosebox the caller posed.
///
/// IT TAKES A SEAT AND NOT A COLLAR, so the composition that reads the station's
/// roots out of process-wide state stands at the caller and never here. That is
/// the split the seat itself is already drawn at and it is kept for its reason:
/// a hurdle poses a seat, and a hurdle that could only pose one by WRITING the
/// environment would be deciding its parallel neighbours' answers.
///
/// THE CLASS IS THE RUST ARM'S, UNCHANGED, and this is the whole reason the
/// python sighting stands in this module rather than at a door. An environment
/// stands under the loosebox by the store's ruling, so it is admitted by the
/// loosebox landing and by nothing else; the tackroom, where the managed
/// interpreter store and uv's caches stand, falls to `Beyond` under the same
/// rule that keeps the toolchain store safe from the rust arm. A second rule
/// written at a door would be a second idea of what a python collar may delete,
/// and the delouse would then be free to hold a third.
///
/// NOTHING IS ASKED OF UV. The rust arm asks cargo where a crate builds because
/// cargo owns that answer; where an environment stands is the KENNEL's answer,
/// composed by the seat and stated to uv on every invocation, so asking uv would
/// be reading back our own declaration through a child process.
pub fn bkcy_sight_python(
    repository: &Path,
    loosebox: &Path,
    seat: &bkcx_Seat,
) -> Result<bkcy_Sighting, String> {
    let root = zbkcy_rooted(repository)?;

    Ok(bkcy_classed(
        &root,
        Some(loosebox),
        &zbkcy_settled(&seat.environment),
        seat.environment.clone(),
    ))
}

/// The repository as the filesystem settles it, which is what every class is
/// judged against.
fn zbkcy_rooted(repository: &Path) -> Result<PathBuf, String> {
    std::fs::canonicalize(repository).map_err(|err| {
        format!(
            "the repository at {} does not resolve, so no derived directory can be judged against \
             it: {}",
            repository.display(),
            err
        )
    })
}

/// Which class a settled build directory falls in, given the repository it was
/// judged against and the loosebox the caller posed.
///
/// SEPARATE FROM THE SIGHTING BECAUSE THE RULE IS SEPARATE FROM WHERE THE PATH
/// CAME FROM. A sighting asks cargo where a collar builds; this asks only where
/// the answer landed, so the rule can be driven over a posed loosebox and a
/// posed tackroom without a cargo redirect standing between the hurdle and the
/// thing it reads. That distance is the point: the class must be composed from
/// the posed root and never from cargo's answer, and a reading that could see
/// cargo could not prove it.
pub fn bkcy_classed(
    root: &Path,
    loosebox: Option<&Path>,
    settled: &Path,
    directory: PathBuf,
) -> bkcy_Sighting {
    // The loosebox is read BEFORE the containment reading, because a loosebox
    // stands outside every source tree by design and would otherwise be refused
    // by the very rule that keeps the tackroom safe. THE STATION'S TACKROOM
    // STANDS NOWHERE BENEATH IT and therefore falls through to that rule
    // unchanged — which is the whole reason this admits a posed root rather than
    // a general licence for anything outside the tree.
    //
    // The loosebox root itself is excluded for the same reason the repository
    // root is below: a path equal to the root would take the whole store.
    if let Some(loosebox) = loosebox {
        let posed = zbkcy_settled(loosebox);
        if settled != posed && settled.starts_with(&posed) {
            return bkcy_Sighting::Loosebox(directory);
        }
    }

    // THE ROOT ITSELF IS BEYOND, and stating it is not pedantry: a cargo
    // configuration naming the repository root as its build directory would
    // otherwise pass the containment reading and take the whole tree with it.
    if settled == root || !settled.starts_with(root) {
        return bkcy_Sighting::Beyond(directory);
    }

    bkcy_Sighting::Within(directory)
}

/// Remove a directory a sighting yielded, answering whether anything stood
/// there.
///
/// IT TAKES THE SIGHTING AND NOT A PATH, which is what makes the sight-then-
/// remove order structural rather than a rule a caller is trusted to keep: there
/// is no way to spell a removal of a directory this module has not judged.
pub fn bkcy_remove(collar: &str, sighted: &bkcy_Sighting) -> Result<bkcy_Swept, String> {
    let directory = match sighted {
        bkcy_Sighting::Within(directory) | bkcy_Sighting::Loosebox(directory) => directory.clone(),
        bkcy_Sighting::Beyond(directory) => {
            return Err(format!(
                "the build directory '{}' stands outside the repository the collar '{}' stands \
                 in, and nothing outside the work tree is ever removed",
                directory.display(),
                collar
            ))
        }
    };

    let stood = directory.is_dir();

    if stood {
        std::fs::remove_dir_all(&directory).map_err(|err| {
            format!(
                "the build directory '{}' could not be removed: {}",
                directory.display(),
                err
            )
        })?;
    }

    Ok(bkcy_Swept {
        collar: collar.to_string(),
        directory,
        stood,
    })
}

/// Whether a directory shelters an executable — is its filesystem ancestor,
/// settled on both sides the way every sighting here is.
///
/// THE DELOUSE'S OWN QUESTION, NEVER THE SWEEP'S. Scoop calls `bkcy_remove`
/// over every directory the operator names and must keep doing so
/// undiminished, so this predicate stands beside the sighting rather than
/// folded into it: the one caller that must spare a directory — the delouse,
/// sparing its own running binary — consults it and skips, while removal
/// itself takes whatever a sighting yielded, unchanged.
pub fn bkcy_shelters(directory: &Path, executable: &Path) -> bool {
    let settled_directory = zbkcy_settled(directory);
    let settled_executable = zbkcy_settled(executable);
    settled_executable.starts_with(&settled_directory)
}

/// A path settled as far as the filesystem will settle it, with whatever does
/// not yet exist rejoined on the end.
///
/// A BUILD DIRECTORY NEED NOT EXIST, so plain canonicalization is not available:
/// it refuses an absent path, and a sweep of an already-clean yard is the
/// ordinary case rather than the exception. Settling the deepest ancestor that
/// does exist is what catches the hazard canonicalization is wanted for — a
/// symlinked ancestor that makes a path inside the tree land outside it — while
/// still answering about a directory nothing has built yet.
fn zbkcy_settled(path: &Path) -> PathBuf {
    let mut tail: Vec<OsString> = Vec::new();
    let mut here = path.to_path_buf();

    loop {
        if let Ok(settled) = std::fs::canonicalize(&here) {
            let mut whole = settled;
            for part in tail.iter().rev() {
                whole.push(part);
            }
            return whole;
        }

        match (here.file_name(), here.parent()) {
            (Some(name), Some(parent)) => {
                tail.push(name.to_os_string());
                here = parent.to_path_buf();
            }
            _ => return path.to_path_buf(),
        }
    }
}

// eof
