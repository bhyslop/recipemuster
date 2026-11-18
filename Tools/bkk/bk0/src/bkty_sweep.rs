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

//! The sweep's hurdles, driven over lures against a real cargo.
//!
//! EVERY SIGHTING HERE ASKS CARGO. A hurdle that asserted over a composed path
//! would prove that this module joins two strings the way it believes it should,
//! which is the one thing that cannot go wrong — what can go wrong is cargo
//! naming a directory somewhere this module did not expect, and only a drive
//! asks that.
//!
//! THE DOOR'S OWN ORDERING IS PROVEN ELSEWHERE, in the integration seat, because
//! validate-then-sight-then-remove is a fact about the binary rather than about
//! this mechanism (`Tools/bkk/bk0/tests/bktd_drive.rs`).
//!
//! SO IS THE REDIRECTED BUILD DIRECTORY, and its seat is forced rather than
//! chosen. The leash sets no working directory on the cargo it spawns, so cargo
//! discovers `.cargo/config.toml` from the CALLING PROCESS's directory — which
//! for a unit hurdle is the tree the suite is running in, never the lure. A
//! redirect written into a lure is therefore invisible from this seat and
//! perfectly visible from the spawned one, where the kennel child stands in the
//! lure itself. A hurdle here would assert against a redirect cargo never read.

use std::path::{
    Path,
    PathBuf, //
};

use super::bkcr_resolve::bkcr_resolve;
use super::bkcx_python::{
    bkcx_Seat,
    bkcx_resolve,
    bkcx_seat_at, //
};
use super::bkcy_sweep::{
    bkcy_Sighting,
    bkcy_classed,
    bkcy_orphans,
    bkcy_remove,
    bkcy_shelters,
    bkcy_sight,
    bkcy_sight_provision,
    bkcy_sight_python, //
};
use super::bkg_breviary::{
    BKG_BLEACH_BASE,
    BKG_BUS_LOOSEBOX_BASE, //
};
use super::bktr_resolve::bktr_lured;
use super::bktu_lure::{
    bktu_Bent,
    bktu_Lure, //
};

/// Sight one of a lure's collars.
fn zbkty_sight(lure: &bktu_Lure, collar: &str) -> bkcy_Sighting {
    let resolved = bkcr_resolve(lure.bktu_root(), &bktr_lured(lure), collar)
        .expect("the lure's collar should resolve");

    assert!(
        resolved.findings.is_empty(),
        "the lure's own collar should conform: {:?}",
        resolved.findings, //
    );

    bkcy_sight(lure.bktu_root(), &bktr_lured(lure), None, &resolved.collar)
        .expect("cargo should be reachable")
}

/// Cargo's build-directory marker and the ignore rule a crate's tree keeps over
/// its build directories, spelled here INDEPENDENTLY of the module under test: the
/// marker is cargo's name, and a hurdle importing the module's own would assert
/// that the module agrees with itself.
const ZBKTY_MARKER: &str = "CACHEDIR.TAG";
const ZBKTY_IGNORED: &str = "target/\n";

/// Pose a build directory at a lure-relative path: cargo's marker at its root and
/// one built file beneath, as a build leaves it.
fn zbkty_built(lure: &bktu_Lure, at: &str) -> PathBuf {
    lure.bktu_write(
        &format!(
            "{}/{}",
            at,
            ZBKTY_MARKER, //
        ),
        "Signature: posed\n",
    );
    lure.bktu_write(
        &format!(
            "{}/release/built",
            at, //
        ),
        "posed build output\n",
    );
    lure.bktu_root().join(at)
}

/// Warm a sighted directory the way a build would, its marker at its root.
fn zbkty_warmed(sighting: &bkcy_Sighting) -> PathBuf {
    let directory = match sighting {
        bkcy_Sighting::Within(directory) => directory.clone(),
        other => panic!(
            "the lure's crate sights within it: {:?}",
            other, //
        ),
    };
    std::fs::create_dir_all(directory.join("release"))
        .expect("the hurdle should warm the directory");
    std::fs::write(directory.join(ZBKTY_MARKER), b"Signature: posed\n")
        .expect("the hurdle should pose the marker");
    directory
}

/// Two paths name one directory, settled as the filesystem settles them.
fn zbkty_same(left: &Path, right: &Path) -> bool {
    match (std::fs::canonicalize(left), std::fs::canonicalize(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

/// A BUILD DIRECTORY A MOVED CRATE LEFT BEHIND IS CLEARED, in the incident's own
/// shape: a crate stands with its build directory ignored, git moves it away, and
/// the old path keeps a build directory holding only ignored files beside nothing
/// git tracks. The collar names the new path alone, so no collar owns the old.
///
/// THE LEFTOVER IS POSED AFTER THE MOVE, because a move by git renames the whole
/// directory and carries its ignored contents along; what a checkout receiving the
/// move by pull keeps at the old path is exactly what this poses.
#[test]
fn bkty_a_build_directory_a_moved_crate_left_behind_is_cleared() {
    let lure = bktu_Lure::bktu_compose("sweep-orphan-moved");
    lure.bktu_write(".gitignore", ZBKTY_IGNORED);
    lure.bktu_crate("old", "suite-lure");
    lure.bktu_commit("stand a crate up at its old path");

    lure.bktu_git(&["mv", "old", "new"]);
    lure.bktu_crate("new", "suite-lure");
    lure.bktu_commit("move the crate by git, its collar following");

    let left = zbkty_built(&lure, "old/target");
    let owned = zbkty_sight(&lure, "suite-lure");
    let owned_directory = zbkty_warmed(&owned);

    let orphans = bkcy_orphans(lure.bktu_root(), std::slice::from_ref(&owned))
        .expect("git should answer for the lure");

    assert_eq!(
        orphans.len(),
        1,
        "the one build directory no collar owns is sighted, and nothing else: {:?}",
        orphans, //
    );
    match &orphans[0] {
        bkcy_Sighting::Within(found) => assert!(
            zbkty_same(found, &left),
            "the orphan sighted is the old path's build directory: {} against {}",
            found.display(),
            left.display(), //
        ),
        other => panic!(
            "an orphan is sighted within the worktree and nowhere else: {:?}",
            other, //
        ),
    }

    let taken = bkcy_remove("suite-lure", &orphans[0]).expect("an orphan is removable");

    assert!(
        taken.stood,
        "the orphan stood before the clear", //
    );
    assert!(
        !left.exists(),
        "the old path's build directory is gone: {}",
        left.display(), //
    );
    assert!(
        owned_directory.is_dir(),
        "the collar's own build directory stands: {}",
        owned_directory.display(), //
    );
}

/// EVERY OWNED BUILD DIRECTORY STANDS, AND SO DOES ONE BESIDE A TRACKED FILE that
/// no collar names: a build directory beside tracked ground is a crate's, and the
/// clear takes only what stands beside nothing at all. A directory nested inside an
/// owned one, wearing the marker, stands as well. The control is an orphan posed in
/// the same lure and removed, so the survivals are the clear's judgment and never
/// its inaction.
#[test]
fn bkty_owned_directories_and_one_beside_a_tracked_file_survive_the_clear() {
    let lure = bktu_Lure::bktu_compose("sweep-orphan-spared");
    lure.bktu_write(".gitignore", ZBKTY_IGNORED);
    lure.bktu_crate("", "suite-lure");
    lure.bktu_write("loose/notes.txt", "a tracked file no collar names\n");
    lure.bktu_commit("stand a crate and a loose tracked file up");

    let owned = zbkty_sight(&lure, "suite-lure");
    let owned_directory = zbkty_warmed(&owned);
    let nested = zbkty_built(&lure, "target/nested/target");
    let beside = zbkty_built(&lure, "loose/target");
    let gone = zbkty_built(&lure, "gone/target");

    let orphans = bkcy_orphans(lure.bktu_root(), std::slice::from_ref(&owned))
        .expect("git should answer for the lure");

    for orphan in &orphans {
        bkcy_remove("suite-lure", orphan).expect("an orphan is removable");
    }

    assert!(
        owned_directory.is_dir(),
        "the collar's own build directory stands: {}",
        owned_directory.display(), //
    );
    assert!(
        nested.is_dir(),
        "a marked directory inside an owned one stands: {}",
        nested.display(), //
    );
    assert!(
        beside.is_dir(),
        "a build directory beside a tracked file stands, though no collar names it: {}",
        beside.display(), //
    );
    assert!(
        !gone.exists(),
        "the control: the build directory beside nothing tracked is cleared in the same drive: \
         {} — orphans sighted {:?}",
        gone.display(),
        orphans, //
    );
}

/// THE DIRECTORY IS CARGO'S OWN ANSWER, and the sighting lands within the lure.
#[test]
fn bkty_a_crate_in_the_tree_sights_within_it() {
    let lure = bktu_Lure::bktu_compose("sweep-within");
    lure.bktu_crate("", "suite-lure");
    lure.bktu_commit("stand a crate up");

    match zbkty_sight(&lure, "suite-lure") {
        bkcy_Sighting::Within(directory) => assert!(
            directory.starts_with(lure.bktu_root()),
            "cargo's answer should stand under the lure: {}",
            directory.display(), //
        ),
        other => panic!(
            "a crate in the tree sights within it: {:?}",
            other, //
        ),
    }
}

/// REMOVAL TAKES ONLY WHAT A SIGHTING YIELDED, and a `Beyond` one refuses rather
/// than deleting. The hurdle asserts the directory still stands afterwards,
/// because a refusal that reported and deleted anyway would pass an assertion
/// about its own message.
#[test]
fn bkty_removal_refuses_a_directory_sighted_beyond() {
    let lure = bktu_Lure::bktu_compose("sweep-refuse");
    let beyond = lure.bktu_root().join("standing");
    std::fs::create_dir_all(&beyond).expect("the hurdle should compose its own directory");

    let sighted = bkcy_Sighting::Beyond(beyond.clone());

    assert!(
        bkcy_remove("suite-lure", &sighted).is_err(),
        "a directory sighted beyond is never removed", //
    );
    assert!(
        beyond.is_dir(),
        "the refusal must leave the directory standing: {}",
        beyond.display(), //
    );
}

/// AN ABSENT DIRECTORY IS A CLEAN YARD RATHER THAN A FAULT, and the report says
/// which. A sweep that refused an already-clean crate would make the second drive
/// of any pair fail.
#[test]
fn bkty_an_absent_directory_is_swept_without_fault() {
    let lure = bktu_Lure::bktu_compose("sweep-absent");
    let never = lure.bktu_root().join("never-built");

    let taken = bkcy_remove("suite-lure", &bkcy_Sighting::Within(never.clone()))
        .expect("an absent directory is not a fault");

    assert!(
        !taken.stood,
        "nothing stood at {}",
        never.display(), //
    );
    assert_eq!(taken.directory, never);
}

/// A WARM DIRECTORY GOES, and the hurdle proves it stood first — an assertion
/// that only checked the absence afterwards would pass against a build that never
/// happened.
#[test]
fn bkty_a_warm_directory_is_removed() {
    let lure = bktu_Lure::bktu_compose("sweep-warm");
    lure.bktu_crate("", "suite-lure");
    lure.bktu_commit("stand a crate up");

    let sighted = zbkty_sight(&lure, "suite-lure");
    let directory = match &sighted {
        bkcy_Sighting::Within(directory) => directory.clone(),
        other => panic!(
            "the lure's crate sights within it: {:?}",
            other, //
        ),
    };

    std::fs::create_dir_all(directory.join("release"))
        .expect("the hurdle should warm the directory");
    assert!(
        directory.is_dir(),
        "the directory must stand before the sweep", //
    );

    let taken = bkcy_remove("suite-lure", &sighted).expect("a warm directory is removable");

    assert!(
        taken.stood,
        "the sweep should report that it stood", //
    );
    assert!(
        !Path::new(&directory).exists(),
        "the directory should be gone: {}",
        directory.display(), //
    );
}

/// A POSED KENNEL DIRECTORY SURVIVES THE SWEEP, and its neighbour does not: the
/// delouse's own exemption is the running executable's filesystem ancestry,
/// never a collar's name, so `bkcy_shelters` answers true for the directory
/// holding a posed copy of it and false for a directory that holds nothing of
/// the kind — and the removal the delouse loop would perform on that answer
/// takes the neighbour while leaving the sheltering directory standing.
#[test]
fn bkty_a_posed_kennel_directory_survives_the_sweep() {
    let lure = bktu_Lure::bktu_compose("sweep-kennel");
    lure.bktu_crate("", "suite-lure");
    lure.bktu_crate("neighbour", "suite-neighbour");
    lure.bktu_commit("stand two crates up");

    let kennel = zbkty_sight(&lure, "suite-lure");
    let kennel_directory = match &kennel {
        bkcy_Sighting::Within(directory) => directory.clone(),
        other => panic!(
            "the lure's crate sights within it: {:?}",
            other, //
        ),
    };

    let neighbour = zbkty_sight(&lure, "suite-neighbour");
    let neighbour_directory = match &neighbour {
        bkcy_Sighting::Within(directory) => directory.clone(),
        other => panic!(
            "the lure's crate sights within it: {:?}",
            other, //
        ),
    };

    // A POSED COPY STANDS IN FOR THE RUNNING EXECUTABLE, on the warm-directory
    // hurdle's own ground: what is proven is the judgment over a path, and a
    // hurdle that needed a real, running binary to prove it could never drive
    // this in a suite of its own.
    let executable = kennel_directory.join("release").join("bkx");
    std::fs::create_dir_all(executable.parent().unwrap())
        .expect("the hurdle should warm the kennel's own directory");
    std::fs::write(&executable, b"posed executable")
        .expect("the hurdle should pose a copy of the running executable");

    std::fs::create_dir_all(neighbour_directory.join("release"))
        .expect("the hurdle should warm the neighbour");

    assert!(
        bkcy_shelters(&kennel_directory, &executable),
        "the directory holding the posed executable should shelter it", //
    );
    assert!(
        !bkcy_shelters(&neighbour_directory, &executable),
        "a directory holding no copy of the executable should not shelter it", //
    );

    // THE DELOUSE LOOP'S OWN JUDGMENT: spare a sheltering directory and remove
    // its neighbour, exactly as `zbkm_deloused` does over every sighting.
    if !bkcy_shelters(&kennel_directory, &executable) {
        bkcy_remove("suite-lure", &kennel).expect("a non-sheltering directory is removable");
    }
    assert!(
        kennel_directory.is_dir(),
        "the sweep must leave the sheltering directory standing: {}",
        kennel_directory.display(), //
    );

    let taken = bkcy_remove("suite-neighbour", &neighbour).expect("the neighbour is removable");
    assert!(
        taken.stood,
        "the neighbour should have stood before removal", //
    );
    assert!(
        !neighbour_directory.exists(),
        "the neighbour should be gone: {}",
        neighbour_directory.display(), //
    );
}

/// THE LOOSEBOX IS ADMISSIBLE AND THE TACKROOM IS NOT, and the two are read in
/// one hurdle because the danger is exactly that they look alike: both stand
/// outside every source tree, which is the one rule that kept the tackroom safe
/// before a second admissible class existed. What tells them apart is the posed
/// root and nothing else — so the tackroom here is a real sibling directory,
/// standing where a station's own would, and it must still sight `Beyond`.
#[test]
fn bkty_a_posed_loosebox_is_deletable_and_a_posed_tackroom_is_not() {
    let lure = bktu_Lure::bktu_compose("sweep-pomfrish");
    let root = lure.bktu_root();

    let loosebox = root.join(BKG_BUS_LOOSEBOX_BASE);
    let tackroom = root.join("tackroom");

    // Both stand OUTSIDE the repository the collar would be judged against, so
    // the containment rule alone cannot separate them.
    let repository = root.join("checkout");

    let in_loosebox = loosebox.join("checkout").join("bkk").join("target");
    let in_tackroom = tackroom.join("rustup").join("toolchains");

    for made in [&repository, &in_loosebox, &in_tackroom] {
        std::fs::create_dir_all(made).expect("the hurdle composes the directories it poses");
    }

    assert_eq!(
        bkcy_classed(
            &repository,
            Some(&loosebox),
            &in_loosebox,
            in_loosebox.clone()
        ),
        bkcy_Sighting::Loosebox(in_loosebox.clone()),
        "a path under the posed {} is one the sweep may delete",
        BKG_BUS_LOOSEBOX_BASE, //
    );

    assert_eq!(
        bkcy_classed(
            &repository,
            Some(&loosebox),
            &in_tackroom,
            in_tackroom.clone()
        ),
        bkcy_Sighting::Beyond(in_tackroom.clone()),
        "the tackroom stands beneath no {} and stays refused",
        BKG_BUS_LOOSEBOX_BASE, //
    );

    // AND THE REFUSAL BITES. A class that reported `Beyond` and removed anyway
    // would pass the assertion above and take the station's toolchain store.
    assert!(
        bkcy_remove("suite-lure", &bkcy_Sighting::Beyond(in_tackroom.clone())).is_err(),
        "nothing sighted beyond is ever removed", //
    );
    assert!(
        in_tackroom.is_dir(),
        "the tackroom must still stand", //
    );

    // THE LOOSEBOX ROOT ITSELF IS NOT A LOOSEBOX PATH, on the same reading that
    // spares a repository root: a path equal to the posed root would take every
    // checkout's products at once.
    assert_eq!(
        bkcy_classed(&repository, Some(&loosebox), &loosebox, loosebox.clone()),
        bkcy_Sighting::Beyond(loosebox.clone()),
        "the {} root itself is never the directory to remove",
        BKG_BUS_LOOSEBOX_BASE, //
    );
}

/// The python collar every hurdle below composes, and where its project stands.
const ZBKTY_PYTHON_COLLAR: &str = "suite-lure";
const ZBKTY_PYTHON_AT: &str = "plindox";

/// A lure carrying one uv project and a python collar over it, with the seat
/// that names where its environment stands.
///
/// THE SEAT IS POSED AND NEVER READ FROM THE ENVIRONMENT, on the python module's
/// own precedent: the reading consults process-wide state every parallel
/// neighbour also reads, so a hurdle posing a root by writing one would be
/// deciding its neighbours' answers.
///
/// THE LURE IS NAMED BY ITS HURDLE, for the reason banked at the python family's
/// own hurdles: the harness runs these in parallel, and a shared name is a
/// shared directory that races its neighbours through one git init.
fn zbkty_python(named: &str) -> (bktu_Lure, bkcx_Seat, PathBuf) {
    let lure = bktu_Lure::bktu_compose(named);
    lure.bktu_project(ZBKTY_PYTHON_AT, ZBKTY_PYTHON_COLLAR, &bktu_Bent::default());

    let collar = bkcx_resolve(lure.bktu_root(), &bktr_lured(&lure), ZBKTY_PYTHON_COLLAR)
        .expect("the seated python collar resolves")
        .collar;

    // A POSED TACKROOM, standing beside the lure's own roots rather than at the
    // station's: nothing below reaches it, and one that named the real store
    // would put a hurdle's assertions next to a directory every clone shares.
    let tackroom = lure.bktu_temp().join("posed-tackroom");
    let seat = bkcx_seat_at(lure.bktu_loosebox(), &tackroom, &collar);

    (lure, seat, tackroom)
}

/// An environment standing where the seat sites it is admitted and removed.
///
/// THE ENVIRONMENT IS MADE BY HAND RATHER THAN CONVERGED, which is what keeps
/// this hurdle hermetic. What it proves is the SIGHTING — that the sweep admits
/// a directory standing under the loosebox and takes it — and a converge here
/// would buy an interpreter download to answer a question about a path.
#[test]
fn bkty_a_python_environment_under_the_loosebox_is_sighted_and_removed() {
    let (lure, seat, _tackroom) = zbkty_python("sweep-python-pomfrish");

    std::fs::create_dir_all(seat.environment.join("lib"))
        .expect("the posed environment stands before it is sighted");

    let sighted = bkcy_sight_python(lure.bktu_root(), lure.bktu_loosebox(), &seat)
        .expect("the posed environment sights");

    assert!(
        matches!(sighted, bkcy_Sighting::Loosebox(_)),
        "an environment under the derived root sighted {:?} rather than at that root's landing",
        sighted, //
    );

    let swept =
        bkcy_remove(ZBKTY_PYTHON_COLLAR, &sighted).expect("an admitted environment is removable");

    assert!(
        swept.stood,
        "the posed environment at {} was reported absent",
        seat.environment.display(), //
    );

    assert!(
        !seat.environment.exists(),
        "the environment at {} stands after the sweep took it",
        seat.environment.display(), //
    );
}

/// An environment standing in the tackroom is refused, and nothing there is
/// touched.
///
/// THE SEAT IS POSED WITH THE TACKROOM WHERE THE LOOSEBOX BELONGS, because that
/// is the only way to compose the path this refusal is about: the seat sites an
/// environment under the loosebox BY CONSTRUCTION, so a hurdle proving the
/// refusal has to hand it the other root and then sight it against the real one.
/// What is proven is the sweep's own rule rather than the seat's arithmetic —
/// the managed interpreter store and uv's caches stand in the tackroom, and this
/// is the reading that keeps them there.
#[test]
fn bkty_a_python_environment_in_the_tackroom_is_beyond_and_untouched() {
    let (lure, _seat, tackroom) = zbkty_python("sweep-python-tackroom");

    let collar = bkcx_resolve(lure.bktu_root(), &bktr_lured(&lure), ZBKTY_PYTHON_COLLAR)
        .expect("the seated python collar resolves")
        .collar;

    let strayed = bkcx_seat_at(&tackroom, &tackroom, &collar);

    std::fs::create_dir_all(strayed.environment.join("lib"))
        .expect("the strayed environment stands before it is sighted");

    let sighted = bkcy_sight_python(lure.bktu_root(), lure.bktu_loosebox(), &strayed)
        .expect("the strayed environment sights");

    assert!(
        matches!(sighted, bkcy_Sighting::Beyond(_)),
        "an environment in the tackroom sighted {:?} rather than beyond",
        sighted, //
    );

    bkcy_remove(ZBKTY_PYTHON_COLLAR, &sighted)
        .expect_err("a directory sighted beyond is not removable");

    assert!(
        strayed.environment.is_dir(),
        "the refused environment at {} was removed anyway",
        strayed.environment.display(), //
    );
}

/// A PROVISION IS SIGHTED AGAINST THE TACKROOM AND THE LOOSEBOX, never against
/// a repository: what bleach may remove stands strictly inside the tackroom the
/// fence proved or strictly inside the loosebox, and the two roots themselves,
/// and a sibling of either, sight `Beyond` — so the rule keeping scoop off the
/// toolchain store keeps bleach off everything else the station holds.
#[test]
fn bkty_a_provision_is_sighted_against_the_tackroom_and_the_loosebox() {
    let lure = bktu_Lure::bktu_compose("sweep-provision");
    let root = lure.bktu_root();

    let tackroom = root.join("tackroom");
    let loosebox = root.join(BKG_BUS_LOOSEBOX_BASE).join("checkout");
    let beside = root.join("beside");

    let rustup = tackroom.join("rustup");
    let quarter = loosebox.join("bkk");
    for made in [&rustup, &quarter, &beside] {
        std::fs::create_dir_all(made).expect("the hurdle composes the directories it poses");
    }

    assert_eq!(
        bkcy_sight_provision(&tackroom, &loosebox, rustup.clone()),
        bkcy_Sighting::Within(rustup.clone()),
        "a home inside the tackroom is one {} may remove",
        BKG_BLEACH_BASE, //
    );
    assert_eq!(
        bkcy_sight_provision(&tackroom, &loosebox, quarter.clone()),
        bkcy_Sighting::Loosebox(quarter.clone()),
        "the quarter under the posed root is one {} may remove",
        BKG_BLEACH_BASE, //
    );
    assert_eq!(
        bkcy_sight_provision(&tackroom, &loosebox, tackroom.join("bkk").join("uv-cache")),
        bkcy_Sighting::Within(tackroom.join("bkk").join("uv-cache")),
        "a root not yet made is sighted by where it would stand", //
    );
    for refused in [tackroom.clone(), loosebox.clone(), beside.clone()] {
        assert_eq!(
            bkcy_sight_provision(&tackroom, &loosebox, refused.clone()),
            bkcy_Sighting::Beyond(refused.clone()),
            "{} is no provision {} may remove",
            refused.display(),
            BKG_BLEACH_BASE, //
        );
    }
}

// eof
