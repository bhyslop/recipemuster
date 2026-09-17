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

//! *gangline* — the one act in which the leash lifts the lock flag.
//!
//! Every other cargo invocation the kennel issues carries `--locked`, which
//! refuses a lock that would change. That enforcement is what makes this door
//! necessary rather than what makes it suspect: a manifest that gained a
//! dependency has no road to a lock that answers it, and two closes hand-wrote
//! lock entries against that wall before this door stood.
//!
//! THE DOOR WRITES THE LOCK AND NOTHING ELSE, AND NEVER COMMITS. It leaves the
//! new lock dirty in the working tree, so the sequence a caller lives is notch,
//! gangline, notch — the door law refusing the drive that stands ahead of it and
//! the operator reading the lock's diff before it is banked. A door that
//! committed its own output would put a lock into the record that nobody read.
//!
//! IT WRITES ONE LOCK AND SURVEYS THE REST, which are two acts and not a
//! widening of the first. Lifting the flag is a deliberate act over a manifest
//! whose dependencies someone just changed, so the write stays where the
//! operator's election put it — one manifest, never a sweep. But a manifest that
//! gained a dependency is upstream of every collar whose closure reaches it, and
//! each of those locks is now behind what it owes; the reading below names them
//! and writes none. A caller told only about the lock it asked for is told the
//! debt one refusal at a time, at whichever door happens to meet it next — which
//! is exactly how the vvr lock came to stop a build on the trunk.
//!
//! WHAT IT REPORTS IS WHETHER THE LOCK MOVED, read from the file's own bytes
//! rather than from cargo's exit. Cargo succeeds either way — re-deriving a lock
//! that already answers its manifest is a no-op it reports as success — so the
//! only honest source for "did anything change" is the bytes before against the
//! bytes after. A caller told merely that cargo succeeded has been told nothing.

use std::path::{Path, PathBuf};

use crate::bkcl_leash;
use crate::bkcr_resolve::{bkcr_closure, bkcr_Collar};
use crate::bkcx_python::{bkcx_Collar, bkcx_Posture, bkcx_Seat};

/// The verb that re-derives a lock from its manifest.
///
/// It is cargo's own word and not one of ours, quoted here for the same reason
/// the lock flag's spelling is quoted at the leash: cargo owns it, and no rename
/// of this estate's may move it.
const ZBKCG_VERB: &str = "generate-lockfile";

/// The lock file a manifest's re-derivation writes, beside the manifest itself.
///
/// THE VALUE SPELLS A WORD THIS ESTATE RESERVES, and the standing is accepted
/// rather than repaired, on the ruling already recorded at the leash's lock
/// flag: cargo names this file, we did not choose the name, and no rename of
/// ours may move it. Recorded here so the next reader meets the ruling instead
/// of re-deriving it — and so nobody "repairs" it into a filename cargo does not
/// write.
const ZBKCG_LOCK_FILE: &str = "Cargo.lock";

/// What came of a re-derivation, said as whether the lock MOVED.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum bkcg_Verdict {
    /// The lock already answered its manifest: the bytes are identical before
    /// and after, and the tree is no dirtier than the door found it.
    Unmoved,
    /// The lock was re-derived and differs. It stands dirty for the notch.
    Moved,
}

/// Re-derive the lock of the collar's manifest, with the lock flag lifted for
/// this act alone.
///
/// THE BYTES ARE READ BEFORE AND AFTER, and the verdict is the comparison. An
/// absent lock reads as absent rather than as an error: a manifest that never
/// had one is exactly the case this door is for, and its arrival is a move.
pub fn bkcg_gangline(
    repository: &Path,
    collar: &bkcr_Collar,
) -> Result<bkcg_Verdict, String> {
    let manifest = repository.join(collar.bkcr_field("BKRR_MANIFEST"));

    let crate_dir = manifest
        .parent()
        .ok_or_else(|| {
            format!(
                "the manifest the collar '{}' declares stands at no directory: {}",
                collar.name,
                manifest.display()
            )
        })?
        .to_path_buf();

    let lock = crate_dir.join(ZBKCG_LOCK_FILE);

    let before = std::fs::read(&lock).ok();

    let run = bkcl_leash::bkcl_gangline(
        repository,
        &manifest,
        ZBKCG_VERB,
        Vec::<&str>::new(),
        &[],
    )?;

    if !run.bkcl_landed() {
        return Err(format!(
            "cargo could not re-derive the lock of the collar '{}' — the invocation was: {}",
            collar.name, run.spelling
        ));
    }

    let after = std::fs::read(&lock).map_err(|err| {
        format!(
            "cargo reported success and no lock stands at {}: {}",
            lock.display(),
            err
        )
    })?;

    if before.as_deref() == Some(after.as_slice()) {
        Ok(bkcg_Verdict::Unmoved)
    } else {
        Ok(bkcg_Verdict::Moved)
    }
}

/// The verb the arrears probe asks a manifest under.
///
/// A SECOND COMPOSITION OF CARGO'S ACCOUNT, DELIBERATELY, and the resolver's own
/// statement of the same verb says why it must be: that one asks with
/// `--no-deps` and offline, because it wants the manifest's own declarations and
/// nothing resolved. This asks the opposite question — resolve the WHOLE closure
/// with the lock flag standing — and the two would answer differently about the
/// very thing this reading is for. Sharing one composition would make one of
/// them wrong.
const ZBKCG_PROBE: &str = "metadata";
const ZBKCG_FORMAT_FLAG: &str = "--format-version";
const ZBKCG_FORMAT: &str = "1";

/// A lock standing BEHIND WHAT IT OWES: its manifest's closure reaches a manifest
/// whose lock was just re-derived, and it no longer answers under the lock flag.
///
/// IT NAMES A MANIFEST AND THE COLLARS OVER IT, in that order of what it is
/// about, because the debt belongs to the LOCK and a lock stands beside a
/// manifest rather than beside a collar. Two collars declare `Tools/vok/Cargo.toml`
/// between them and three declare `vov_veiled/jjk/Cargo.toml`, so a reading keyed
/// on collars would report one debt three times and invite three drives to
/// settle it — the second and third of which would find nothing to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkcg_Arrear {
    /// The manifest whose lock is behind, repo-relative, as the collars declare it.
    pub manifest: String,
    /// Every collar declaring that manifest, sorted. Any one of them names the
    /// drive that settles the debt.
    pub collars: Vec<String>,
}

/// Every lock the just-written one leaves in arrears.
///
/// THE READING RUNS AFTER THE WRITE AND WRITES NOTHING, which is the whole of how
/// it keeps the operator's election intact: the door still re-derives one
/// manifest's lock, and this says what that re-derivation now owes elsewhere.
/// A reading that repaired what it found would be the sweep the election bars,
/// arrived at by a second road.
///
/// THE REACH IS THE COLLAR'S OWN CLOSURE, ASKED FROM THE OTHER END. A collar is
/// in arrears when the path-dependency closure of ITS manifest reaches the
/// manifest just written — not the other way round — because a lock covers what
/// its own crate compiles, and it is the downstream crate whose lock names the
/// upstream package. The closure is the resolver's, dev edges and all: cargo
/// derives a lock over the root's dev-dependencies too, so a reach that dropped
/// them would call a suite's lock current while its own dev edge had moved.
///
/// THE PROBE IS AN ORDINARY LEASHED DRIVE AND THE EXIT IS THE FINDING. A lock
/// that does not answer its manifest refuses under the lock flag — the very
/// refusal a build meets — so asking cargo and reading its exit is the same
/// question the next door would ask, put early. Nothing is parsed from cargo's
/// text: a reading that matched on a sentence would go silent the day cargo
/// reworded it, and go silent as a GREEN.
///
/// IT HANDS BACK HOW MANY LOCKS IT PROBED, AND THAT COUNT IS THE READING'S OWN
/// CONTROL. Finding nothing is this reading's ordinary answer, so a walk that
/// reached no collar at all — a closure that silently resolved nothing, a target
/// that matched nothing — reports the identical silence as a tree that is
/// genuinely square. The count is what parts the two, and it is returned rather
/// than logged so the door cannot render a green without having been told the
/// coverage behind it.
pub fn bkcg_arrears(
    repository: &Path,
    written: &bkcr_Collar,
    roster: &[bkcr_Collar],
) -> Result<(usize, Vec<bkcg_Arrear>), String> {
    let declared = written.bkcr_field("BKRR_MANIFEST");

    // THE PARENT IS TAKEN BEFORE THE PATH IS SETTLED, and the order is the whole
    // of it: settling `<manifest>/..` asks the filesystem to walk THROUGH a file,
    // which no manifest permits, so the reading refused every collar it was given
    // until a real drive said so.
    let target = repository
        .join(declared)
        .parent()
        .and_then(zbkcg_settled)
        .ok_or_else(|| {
            format!(
                "the manifest '{}' settles to no real path, so nothing can be said about what stands \
                 downstream of it",
                declared
            )
        })?;

    let mut arrears: Vec<bkcg_Arrear> = Vec::new();
    let mut probed = 0usize;

    for (manifest, collars) in zbkcg_manifests(roster) {
        // The written manifest's own collars are not downstream of it: its lock
        // is the one just re-derived, and it answers by construction.
        if manifest == declared {
            continue;
        }

        let closure = bkcr_closure(repository, &repository.join(&manifest), true)?;

        let reaches = closure
            .iter()
            .filter_map(|held| zbkcg_settled(&repository.join(held)))
            .any(|held| held == target);

        if !reaches {
            continue;
        }

        probed += 1;

        if zbkcg_answers(repository, &repository.join(&manifest))? {
            continue;
        }

        arrears.push(bkcg_Arrear { manifest, collars });
    }

    Ok((probed, arrears))
}

/// The roster's manifests, each with the collars declaring it, sorted by manifest.
///
/// A COLLAR DECLARING NO MANIFEST IS DROPPED rather than reported: the resolver
/// already carries that as a finding against the collar itself, and a second
/// voice saying it here would report one fault as two.
fn zbkcg_manifests(roster: &[bkcr_Collar]) -> Vec<(String, Vec<String>)> {
    let mut held: Vec<(String, Vec<String>)> = Vec::new();

    for collar in roster {
        let manifest = collar.bkcr_field("BKRR_MANIFEST");
        if manifest.is_empty() {
            continue;
        }

        match held.iter_mut().find(|(seen, _)| seen == manifest) {
            Some((_, collars)) => collars.push(collar.name.clone()),
            None => held.push((manifest.to_string(), vec![collar.name.clone()])),
        }
    }

    for (_, collars) in held.iter_mut() {
        collars.sort();
    }
    held.sort();
    held
}

/// Whether a manifest's lock still answers it, asked WITH THE LOCK FLAG STANDING.
///
/// THE RECALLED FACE, because this reads an exit and nothing else. The driven
/// face would hand cargo this process's streams, and the verb asked here answers
/// with a whole metadata document — a page of JSON per collar on the operator's
/// console, saying nothing the verdict reads.
fn zbkcg_answers(repository: &Path, manifest: &Path) -> Result<bool, String> {
    let recalled = bkcl_leash::bkcl_recall(
        repository,
        manifest,
        ZBKCG_PROBE,
        [ZBKCG_FORMAT_FLAG, ZBKCG_FORMAT],
        &[],
    )?;

    Ok(recalled.run.bkcl_landed())
}

/// A path as the filesystem finally spells it, or nothing where it does not stand.
fn zbkcg_settled(path: &Path) -> Option<PathBuf> {
    std::fs::canonicalize(path).ok()
}

/// uv's own verb for authoring a lock, and the flag naming the project it is
/// authored over.
///
/// The spelling is the tool's own, declared as a xenonym spelling line under
/// that authority's carrier (BKSCL-Collar.adoc "The Toolchain Authorities"),
/// which is where the ruling lives now; VOr_9ww honours the carrier and stands
/// the value down.
const ZBKCG_UV_VERB: &str = "lock";
const ZBKCG_UV_PROJECT: &str = "--project";

/// The lock a python project's re-derivation writes, beside its manifest.
const ZBKCG_UV_LOCK_FILE: &str = "uv.lock";

/// Re-derive one python collar's lock through the leash.
///
/// THE VERDICT IS READ THE RUST ARM'S WAY, from the lock's own bytes before
/// against after, and for its exact reason: uv reports success either way — a
/// lock that already answers its manifest is a no-op it exits 0 on — so cargo's
/// exit and uv's are equally silent about whether anything changed.
///
/// THE POSTURE IS REACHING AND NOT FETCHING. Authoring a lock resolves against
/// an index and against an interpreter; it may reach the first and may not fetch
/// the second, which is the posture the python module's own roster assigns this
/// door and the routine-download ruling behind it. The interpreter it resolves
/// against is therefore one the converge already installed, and the guard above
/// says so before uv is spawned rather than leaving uv's account of a cold store
/// to reach the operator's terminal alone.
///
/// IT WRITES A LOCK AND NEVER COMMITS ONE, exactly as the rust arm does: the
/// file is left dirty for the operator's notch, and the diff is read before it
/// is banked.
pub fn bkcg_python(
    repository: &Path,
    collar: &bkcx_Collar,
    seat: &bkcx_Seat,
) -> Result<bkcg_Verdict, String> {
    crate::bkcx_python::bkcx_interpreter_stands(repository, collar, seat)?;

    let project = repository.join(collar.bkcx_project());
    let lock = project.join(ZBKCG_UV_LOCK_FILE);

    let before = std::fs::read(&lock).ok();

    let stated = crate::bkcx_python::bkcx_stated(seat, bkcx_Posture::Reaching);
    let borne = crate::bkcx_python::bkcx_borne(&stated);

    let run = bkcl_leash::bkcl_uv(
        repository,
        &project,
        [
            std::ffi::OsStr::new(ZBKCG_UV_VERB),
            std::ffi::OsStr::new(ZBKCG_UV_PROJECT),
            project.as_os_str(),
        ],
        &borne,
    )?;

    if !run.bkcl_landed() {
        return Err(format!(
            "uv could not re-derive the lock of the python collar '{}' — the invocation was: {}",
            collar.name, run.spelling
        ));
    }

    let after = std::fs::read(&lock).map_err(|err| {
        format!(
            "uv reported success and no lock stands at {}: {}",
            lock.display(),
            err
        )
    })?;

    if before.as_deref() == Some(after.as_slice()) {
        Ok(bkcg_Verdict::Unmoved)
    } else {
        Ok(bkcg_Verdict::Moved)
    }
}

// eof
