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

//! The converge's composition hurdles.
//!
//! They compose a collar in hand, on the launch seam's own reasoning: what is
//! under test is what the kennel SPELLS and DERIVES from a collar's
//! declarations, both of them pure functions of the collar. The act itself —
//! a build that lands and an install that moves a file — is proven over a lure by
//! a hurdle that spawns the door, because neither can be observed without a real
//! artifact.

use super::bkce_election::bkce_Geography;
use super::bkch_heel::{
    bkch_Kibbled,
    bkch_compose,
    bkch_kibble,
    bkch_profile_dir,
    bkch_python_at,
    bkch_struck,
    bkch_yard, //
};
use super::bkcl_leash;
use super::bkcl_leash::bkcl_host;
use super::bkcq_kibble::{
    bkcq_Feedroom,
    bkcq_Kibble,
    bkcq_feedroom_at,
    bkcq_findings, //
};
use super::bkcr_resolve::bkcr_Collar;
use super::bktr_resolve::bktr_lured;
use super::bktu_lure::{
    bktu_Bent,
    bktu_Haynet,
    bktu_Lure,
    bktu_Stationed, //
};
use bkp::bkprc_catena::bkprc_admit;
use std::ffi::OsStr;
use std::path::{
    Path,
    PathBuf, //
};

/// The seat every composition below is joined from. A collar's manifest is a path
/// reference joined from the repository root, so the root has to be something;
/// this is the something, and nothing reads it off a disk.
const ZBKTH_SEAT: &str = "/seat";

/// An app collar whose residence is NOT where cargo leaves the artifact, AND
/// whose binary wears a different name there — the shipped binary's shape at
/// both ends, and the case an install that leaned on the build's side effect
/// would silently get wrong twice over.
const ZBKTH_ELSEWHERE: &str = "\
BKRR_COLLAR=\"app-lure\"
BKRR_KIND=\"bknre_app\"
BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"
BKRR_TARGET=\"lure\"
BKRR_ROOTS=\"Tools/lure/src\"
BKRR_FEATURES=\"alpha beta\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RESIDENCE=\"Tools/bin\"
BKRR_BYNAME=\"lurex\"
";

/// An app collar whose residence IS cargo's own output directory — the shape a
/// crate takes when it is already built where it is meant to stand, so the
/// install has nothing to move and must say so rather than reporting an act it
/// did not perform.
const ZBKTH_SEATED: &str = "\
BKRR_COLLAR=\"app-lure\"
BKRR_KIND=\"bknre_app\"
BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"
BKRR_TARGET=\"lure\"
BKRR_ROOTS=\"Tools/lure/src\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RESIDENCE=\"Tools/lure/target/release\"
BKRR_BYNAME=\"lure\"
";

/// A suite collar, which this door does not serve.
const ZBKTH_SUITE: &str = "\
BKRR_COLLAR=\"suite-lure\"
BKRR_KIND=\"bknre_suite\"
BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"
BKRR_TARGET=\"bknre_manifest\"
BKRR_ROOTS=\"Tools/lure/src\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"test\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RUNNER=\"bknre_cargo\"
BKRR_TONGUE=\"bknre_harness\"
";

/// Compose a collar without touching a disk.
fn zbkth_collar(name: &str, text: &str) -> bkcr_Collar {
    bkcr_Collar {
        name: name.to_string(),
        instance: PathBuf::from(name),
        regime: bkprc_admit(text, "composed").expect("the collar stands inside the subset"),
    }
}

/// CARGO'S PROFILE-TO-DIRECTORY RULE, held at every arm. The two that do not
/// name their own directory are the whole of what a derivation can get wrong, and
/// a mapping that returned the profile verbatim would pass on `release` alone.
#[test]
fn bkth_a_profile_names_the_directory_cargo_gives_it() {
    assert_eq!(
        bkch_profile_dir("dev"),
        "debug",
        "dev builds into debug", //
    );
    assert_eq!(
        bkch_profile_dir("test"),
        "debug",
        "test builds into debug", //
    );
    assert_eq!(
        bkch_profile_dir("release"),
        "release",
        "release names its own", //
    );
    assert_eq!(
        bkch_profile_dir("bench"),
        "release",
        "bench names release", //
    );
    assert_eq!(
        bkch_profile_dir("thorough"),
        "thorough",
        "a custom profile takes its own name, which is the general case the two \
         above are exceptions to", //
    );
}

/// THE STRUCK PATH IS THE MANIFEST'S OWN TARGET ROOT, the profile's directory
/// and the collar's target — never the repository's root target directory, which
/// is a different crate's output at every collar the roster carries.
#[test]
fn bkth_the_struck_path_is_rooted_at_the_manifest() {
    let struck = bkch_struck(
        Path::new(ZBKTH_SEAT),
        &zbkth_collar("app-lure", ZBKTH_ELSEWHERE),
    );

    assert_eq!(
        struck,
        PathBuf::from("/seat/Tools/lure/target/release/lure"),
        "cargo's own layout, rooted where the manifest stands", //
    );
}

/// THE DECLARED SHAPE IS SPELLED AND NOTHING IS INFERRED. An artifact built at
/// some other profile or feature set than the collar declares is one the
/// election cannot speak for.
#[test]
fn bkth_the_collars_declared_shape_is_what_is_spelled() {
    let converge = bkch_compose(
        Path::new(ZBKTH_SEAT),
        &bkce_Geography::Undispatched,
        &zbkth_collar("app-lure", ZBKTH_ELSEWHERE),
    )
    .expect("an app collar is what this door serves");

    assert_eq!(
        converge.verb, "build",
        "the converge builds", //
    );
    assert_eq!(
        converge.bkch_spelling(),
        "build --profile release --features alpha beta",
        "every declared field spelled, and none invented", //
    );

    // AN EMPTY FIELD IS A DECLARATION AND SPELLS NOTHING, which is the case the
    // roster carries at nearly every collar.
    let bare = bkch_compose(
        Path::new(ZBKTH_SEAT),
        &bkce_Geography::Undispatched,
        &zbkth_collar("app-lure", ZBKTH_SEATED),
    )
    .expect("an app collar is what this door serves");

    assert_eq!(
        bare.bkch_spelling(),
        "build --profile release",
        "an empty feature set spells no flag: {}",
        bare.bkch_spelling(), //
    );
}

/// THE TWO SEATINGS ARE TOLD APART, and this is the reading the install branches
/// on. A collar whose residence is cargo's own output directory has nothing to
/// move; one whose residence stands elsewhere does, and a door that could not
/// tell them apart would either copy a file onto itself or skip the install that
/// the shipped binary depends on.
#[test]
fn bkth_a_residence_away_from_the_output_is_told_from_one_at_it() {
    let elsewhere = bkch_compose(
        Path::new(ZBKTH_SEAT),
        &bkce_Geography::Undispatched,
        &zbkth_collar("app-lure", ZBKTH_ELSEWHERE),
    )
    .expect("an app collar is what this door serves");

    assert_eq!(
        elsewhere.residence,
        PathBuf::from("/seat/Tools/bin/lurex"),
        "the residence is the collar's, joined with the name its binary WEARS there — never the \
         target cargo built, which is a different declaration and differs here on purpose", //
    );
    assert_eq!(
        elsewhere.struck,
        PathBuf::from("/seat/Tools/lure/target/release/lure"),
        "and what cargo struck keeps the target's name: the converge is a rename as well as a \
         move, which is the whole reason the two fields are two", //
    );
    assert!(
        !elsewhere.bkch_seated(),
        "the artifact does not already stand at this collar's residence", //
    );

    let seated = bkch_compose(
        Path::new(ZBKTH_SEAT),
        &bkce_Geography::Undispatched,
        &zbkth_collar("app-lure", ZBKTH_SEATED),
    )
    .expect("an app collar is what this door serves");

    assert_eq!(
        seated.struck, seated.residence,
        "this collar's residence IS where cargo leaves the artifact", //
    );
    assert!(
        seated.bkch_seated(),
        "and the door reads it as already seated: {} against {}",
        seated.struck.display(),
        seated.residence.display(), //
    );
}

/// A SUITE IS NEVER CONVERGED, and the refusal names why rather than reporting a
/// missing field. A suite declares no residence, so a door that tried anyway
/// would fail on the absence and say something true about a field instead of
/// something true about the act.
#[test]
fn bkth_a_suite_collar_refuses_the_converge() {
    let refusal = bkch_compose(
        Path::new(ZBKTH_SEAT),
        &bkce_Geography::Undispatched,
        &zbkth_collar("suite-lure", ZBKTH_SUITE),
    )
    .expect_err("a suite is never converged");

    assert!(
        refusal.contains("suite-lure"),
        "the refusal names the collar: {}",
        refusal, //
    );
    assert!(
        refusal.contains("residence"),
        "and says what a suite has not got: {}",
        refusal, //
    );
}

/// HEEL WRITES THE SEAT'S OWN RESIDENCE AND REFUSES A TARGET INSIDE THE DELIVERED
/// ROOT. A billet's build never lands on a delivered tree, whatever any position
/// says: the delivered root is the borrow candidate EVERY linked worktree reads,
/// so a binary written there is written into all of their elections at once.
///
/// THE ORDINARY DISPATCHED CASE IS THE FIRST ASSERTION, and it is the one that
/// says the guard is a guard rather than a bar: a seat whose work tree stands
/// apart from the delivered root converges exactly as it always did, and the
/// install target is the join onto the WORK tree.
#[test]
fn bkth_a_dispatched_converge_writes_the_work_tree_and_refuses_the_delivered_root() {
    let composed = bkch_compose(
        Path::new(ZBKTH_SEAT),
        &bkce_Geography::Dispatched(PathBuf::from("/delivered")),
        &zbkth_collar("app-lure", ZBKTH_ELSEWHERE),
    )
    .expect("a linked worktree apart from the delivered root converges");

    assert_eq!(
        composed.residence,
        PathBuf::from("/seat/Tools/bin/lurex"),
        "the install target joins onto the work tree the door entered, never onto the delivered \
         root", //
    );

    // THE DELIVERED ROOT CONTAINING THE WORK TREE IS THE REFUSAL, and it is the
    // shape a misplaced worktree actually takes: one added inside its own main
    // checkout, so the delivered root stands above the tree the door entered.
    let refusal = bkch_compose(
        Path::new(ZBKTH_SEAT),
        &bkce_Geography::Dispatched(PathBuf::from("/seat/Tools")),
        &zbkth_collar("app-lure", ZBKTH_ELSEWHERE),
    )
    .expect_err("a target inside the delivered root is refused");

    assert!(
        refusal.contains("/seat/Tools/bin/lurex"),
        "the refusal names the target it would have written: {}",
        refusal, //
    );
    assert!(
        refusal.contains("/seat/Tools"),
        "the refusal names the delivered root that contains it: {}",
        refusal, //
    );

    // THE ROOT ITSELF IS INSIDE ITSELF, which is the worst case rather than an
    // edge one: a delivered root that is the residence directory outright.
    assert!(
        bkch_compose(
            Path::new(ZBKTH_SEAT),
            &bkce_Geography::Dispatched(PathBuf::from("/seat/Tools/bin/lurex")),
            &zbkth_collar("app-lure", ZBKTH_ELSEWHERE),
        )
        .is_err(),
        "a delivered root that IS the target contains it", //
    );

    // A SIBLING WHOSE NAME IS A PREFIX IS NOT CONTAINMENT. `/seat/Tools/bi` is
    // not above `/seat/Tools/bin`, and a guard comparing spelled text rather
    // than path components would refuse this one — which would make every seat
    // whose delivered root shared an opening with its work tree unable to
    // converge at all.
    assert!(
        bkch_compose(
            Path::new(ZBKTH_SEAT),
            &bkce_Geography::Dispatched(PathBuf::from("/seat/Tools/bi")),
            &zbkth_collar("app-lure", ZBKTH_ELSEWHERE),
        )
        .is_ok(),
        "the control: a path that merely shares an opening with the target contains nothing", //
    );

    // AND SELF-GEOGRAPHY REACHES NO GUARD AT ALL, there being no delivered root
    // to stand inside of. Without this the assertions above could be cleared by
    // a guard that had begun refusing the ordinary case too.
    assert!(
        bkch_compose(
            Path::new(ZBKTH_SEAT),
            &bkce_Geography::Undispatched,
            &zbkth_collar("app-lure", ZBKTH_ELSEWHERE),
        )
        .is_ok(),
        "the control: a main worktree has no delivered root, so nothing is asked", //
    );
}

// ---------------------------------------------------------------------------
// The prebuilt kibble's converge.
//
// THE VENDOR'S RELEASE IS COMPOSED AND SERVED FROM A LOCAL PATH, which is what
// makes the whole road hurdle-able. A kibble declares the store its release
// stands under and the door joins the version and the document's name onto it;
// a `file://` store is a store like any other to curl, so the fetch, the seal,
// the triple lookup, the digest and the placement all run end to end with no
// vendor reached and no network touched. A hurdle pointed at a real release
// would be measuring GitHub's uptime.
//
// THE KIBBLE IS POSED IN HAND AND ITS ROOT IS THE HURDLE'S OWN. The tackroom
// stays the station's — the leash's fence refuses any other, and the toolchain
// a converge builds with stands there — but where a kibble lands is a haynet's
// root beneath the hurdle's own dispatch temp directory, so no hurdle here
// writes, clears or reads the station's kibble quarter.
//
// NOTHING HERE WRITES THE PROCESS ENVIRONMENT. The harness runs these in
// parallel threads with the rest of the suite, and a hurdle that stated a store
// would be deciding its neighbours' answers — a red three files away, already
// paid for once.

/// The version every posed release carries, and the name its binary wears.
/// Plainly fabricated: a hurdle carrying uv's real pin would be a second home
/// for it, and a reader meeting the estate's own digest here would take these
/// for statements about uv where what they prove is what the DOOR does.
const ZBKTH_POSED_VERSION: &str = "0.0.1-posed";
const ZBKTH_POSED_BYNAME: &str = "posed-uv";

/// A triple no `rustc` declares, for the hurdle proving a release that serves
/// this station nothing.
const ZBKTH_FOREIGN_TRIPLE: &str = "moonstone-unknown-none-posed";

/// The repository these hurdles read a pin and a host triple through, which is
/// the tree this suite was built from.
fn zbkth_repository() -> PathBuf {
    zbkth_crate()
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .expect("the crate stands at an address beneath a kit directory inside its repository")
}

/// This crate's own directory, which is its address beneath the kit directory.
///
/// TAKEN FROM THE MANIFEST RATHER THAN SPELLED, so a hurdle reads the seat it was
/// built from and no re-spelling of the kit directory or the address can part
/// from it.
fn zbkth_crate() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

/// The host triple this station's pinned toolchain declares — asked for by the
/// same road the door takes.
///
/// ASKED RATHER THAN SPELLED, because a hurdle that wrote a triple into a
/// composed release would pass on this station and fail on every other, and
/// would be measuring agreement with itself rather than the lookup.
fn zbkth_host() -> String {
    let repository = zbkth_repository();

    bkcl_host(
        &repository,
        &super::bktr_resolve::bktr_suite(),
        &zbkth_crate(),
    )
    .expect("the pinned toolchain declares a host triple")
}

/// The dispatch's temp directory, which a haynet's root is admitted beneath, as
/// the suite's own dispatch file names it.
const ZBKTH_TEMP_VAR: &str = "BURD_TEMP_DIR";

/// The sha256 of one file, taken the way the door takes it.
fn zbkth_digest(path: &Path) -> String {
    let opened = std::fs::File::open(path).unwrap_or_else(|err| {
        panic!(
            "could not read {}: {}",
            path.display(),
            err, //
        )
    });

    let out = std::process::Command::new("openssl")
        .arg("dgst")
        .arg("-sha256")
        .stdin(std::process::Stdio::from(opened))
        .output()
        .unwrap_or_else(|err| {
            panic!(
                "could not run openssl: {}",
                err, //
            )
        });

    String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .next_back()
        .unwrap_or_else(|| panic!("openssl said nothing a digest can be read out of"))
        .to_string()
}

/// Compose one gzipped tar carrying the declared binary, nested under a
/// directory as a vendor's own archives are.
fn zbkth_tarball(release: &Path, archive: &str) -> PathBuf {
    let nest = release.join("nested");
    std::fs::create_dir_all(&nest).expect("the nest stands");

    let binary = nest.join(ZBKTH_POSED_BYNAME);
    std::fs::write(&binary, "#!/bin/sh\necho posed\n").expect("the posed binary stands");

    let at = release.join(archive);

    let out = std::process::Command::new("tar")
        .arg("-czf")
        .arg(&at)
        .arg("-C")
        .arg(release)
        .arg("nested")
        .output()
        .unwrap_or_else(|err| {
            panic!(
                "could not run tar: {}",
                err, //
            )
        });

    assert!(
        out.status.success(),
        "the posed archive composes", //
    );

    at
}

/// A whole posed release, and the kibble that reaches it.
struct zbkth_Posed {
    kibble: bkcq_Kibble,
    residence: PathBuf,
}

/// Compose a release on a haynet's lure and pose the kibble that declares it,
/// named as the caller names it.
///
/// KEYED ON THE SEAT, NEVER ON THE NAME. The release stands beside the haynet
/// beneath the lure's own temp directory, and the residence a converge over the
/// haynet lands is the haynet's, so two hurdles holding one kibble name own two
/// residences. Nothing is cleared to make the residence cold: the haynet's root
/// has never stood.
///
/// `digest` AND `seal` ARE THE CALLER'S, which is the whole point of this face.
/// A hurdle proving the converge lets both stand true; one proving a refusal
/// moves exactly one of them, the bytes being identical in either case — so
/// what the two drives differ in is a declaration and nothing else.
#[allow(clippy::too_many_arguments)]
fn zbkth_pose(
    lure: &bktu_Lure,
    haynet: &bktu_Haynet,
    kibble: &str,
    archive: &str,
    triple: &str,
    lay_archive: bool,
    digest: Option<&str>,
    seal: Option<&str>,
) -> zbkth_Posed {
    let root = lure.bktu_temp().join(ZBKTH_RELEASE);

    let posed = zbkth_release(&root, kibble, archive, triple, lay_archive, digest, seal);

    let residence = haynet
        .bktu_root()
        .join(kibble)
        .join(ZBKTH_POSED_VERSION)
        .join(ZBKTH_POSED_BYNAME);

    zbkth_Posed {
        kibble: posed,
        residence,
    }
}

/// Where a haynet's release is composed, beside the haynet itself.
const ZBKTH_RELEASE: &str = "release";

/// Converge a posed kibble over its haynet.
fn zbkth_converge(haynet: &bktu_Haynet, kibble: &bkcq_Kibble) -> Result<bkch_Kibbled, String> {
    bkch_kibble(
        &zbkth_repository(),
        &super::bktr_resolve::bktr_suite(),
        kibble,
        &bkth_feedroom(haynet),
    )
}

/// The kibble root a haynet carries, read through the kennel's own admission
/// against this run's dispatch temp directory — the same reading a child kennel
/// takes of the root the haynet's drive states.
pub(crate) fn bkth_feedroom(haynet: &bktu_Haynet) -> bkcq_Feedroom {
    let temp = super::bktu_lure::bktu_suite_said(ZBKTH_TEMP_VAR).unwrap_or_else(|| {
        panic!(
            "the suite's dispatch names no {}, and a haynet stands beneath it or nowhere",
            ZBKTH_TEMP_VAR, //
        )
    });

    bkcq_feedroom_at(Some(haynet.bktu_root()), Some(Path::new(&temp)), None).unwrap_or_else(|err| {
        panic!(
            "the haynet's root is not admitted: {}",
            err, //
        )
    })
}

/// Compose a release served from a local path under `root`, and the kibble that
/// declares it.
fn zbkth_release(
    root: &Path,
    name: &str,
    archive: &str,
    triple: &str,
    lay_archive: bool,
    digest: Option<&str>,
    seal: Option<&str>,
) -> bkcq_Kibble {
    let release = root.join(ZBKTH_POSED_VERSION);
    std::fs::create_dir_all(&release).expect("the posed release stands");

    let truth = if lay_archive {
        let at = zbkth_tarball(&release, archive);
        zbkth_digest(&at)
    } else {
        // AN ARCHIVE THE RELEASE NEVER PUBLISHED, so a drive that reached the
        // fetch at all would fail differently from the refusal under test —
        // which is how these hurdles observe that NOTHING was fetched.
        String::new()
    };

    let stated = digest.map(str::to_string).unwrap_or(truth);

    let bordereau = format!(
        "{{\n  \"artifacts\": {{\n    \"{}\": {{\n      \"kind\": \"executable-zip\",\n      \
         \"target_triples\": [\"{}\"],\n      \"checksums\": {{ \"sha256\": \"{}\" }}\n    }}\n  \
         }}\n}}\n",
        archive,
        triple,
        stated, //
    );

    let at = release.join("bordereau.json");
    std::fs::write(&at, &bordereau).expect("the posed bordereau stands");

    let sealed = seal
        .map(str::to_string)
        .unwrap_or_else(|| zbkth_digest(&at));

    let declared = format!(
        "BKRK_KIBBLE=\"{}\"\n\
         BKRK_KIND=\"bknre_prebuilt\"\n\
         BKRK_PROGRAM=\"{}\"\n\
         BKRK_VERSION=\"{}\"\n\
         BKRK_SEAL=\"{}\"\n\
         BKRK_BYNAME=\"{}\"\n\
         BKRK_LARDER=\"file://{}\"\n\
         BKRK_BORDEREAU=\"bordereau.json\"\n",
        name,
        ZBKTH_POSED_BYNAME,
        ZBKTH_POSED_VERSION,
        sealed,
        ZBKTH_POSED_BYNAME,
        root.display(), //
    );

    let kibble = bkcq_Kibble {
        name: name.to_string(),
        instance: PathBuf::from("Tools").join("bkk"),
        regime: bkprc_admit(&declared, "a posed prebuilt kibble").expect("the posed text admits"),
    };

    assert!(
        bkcq_findings(&kibble).is_empty(),
        "the control: a posed prebuilt kibble conforms before any converge is asked of it", //
    );

    kibble
}

#[test]
fn bkth_a_prebuilt_kibble_is_fetched_against_its_bordereau_and_placed() {
    let triple = zbkth_host();
    let archive = format!(
        "posed-{}.tar.gz",
        triple, //
    );
    let lure = bktu_Lure::bktu_compose("heel-kibble-placed");
    let haynet = lure.bktu_haynet();
    let posed = zbkth_pose(
        &lure,
        &haynet,
        ZBKTH_KIBBLE,
        &archive,
        &triple,
        true,
        None,
        None,
    );

    let landed = zbkth_converge(&haynet, &posed.kibble).unwrap_or_else(|err| {
        panic!(
            "the converge refused: {}",
            err, //
        )
    });

    assert!(
        matches!(landed, bkch_Kibbled::Placed(_)),
        "a cold residence answered {:?} rather than a placement",
        landed, //
    );

    assert!(
        posed.residence.is_file(),
        "nothing stands at {}",
        posed.residence.display(), //
    );

    // THE VERSION IS IN THE PATH, which is what makes the pin a path test.
    assert!(
        posed
            .residence
            .to_string_lossy()
            .contains(ZBKTH_POSED_VERSION),
        "the residence {} does not carry the declared version",
        posed.residence.display(), //
    );

    // DRIVEN AGAIN IT FETCHES NOTHING. The store the release was served from is
    // taken away first, so a second converge that reached for it would refuse
    // rather than quietly re-place — which is the only way this hurdle can tell
    // a residence test from a repeated download.
    let _ = std::fs::remove_dir_all(lure.bktu_temp().join(ZBKTH_RELEASE));

    let again = zbkth_converge(&haynet, &posed.kibble).unwrap_or_else(|err| {
        panic!(
            "the second converge refused: {}",
            err, //
        )
    });

    assert_eq!(
        again,
        bkch_Kibbled::Current,
        "a standing residence answered {:?} rather than current",
        again, //
    );
}

#[test]
fn bkth_a_bordereau_that_does_not_answer_its_seal_refuses_naming_both_digests() {
    let triple = zbkth_host();
    let archive = format!(
        "posed-{}.tar.gz",
        triple, //
    );

    // THE ARCHIVE IS NEVER LAID DOWN, so a drive that got past the seal would
    // refuse on a missing file and say so — which is how this hurdle observes
    // that no archive was fetched.
    let lure = bktu_Lure::bktu_compose("heel-kibble-unsealed");
    let haynet = lure.bktu_haynet();
    let posed = zbkth_pose(
        &lure,
        &haynet,
        ZBKTH_KIBBLE,
        &archive,
        &triple,
        false,
        Some(ZBKTH_WRONG_DIGEST),
        Some(ZBKTH_WRONG_SEAL),
    );

    let said = zbkth_converge(&haynet, &posed.kibble)
        .expect_err("a bordereau that does not answer its seal refuses");

    assert!(
        said.contains(ZBKTH_WRONG_SEAL),
        "the refusal does not name the declared digest: {}",
        said, //
    );
    assert!(
        said.contains("bordereau"),
        "the refusal does not say which file failed: {}",
        said, //
    );
    assert!(
        !said.contains(&archive),
        "the refusal names the archive, so the fetch was reached: {}",
        said, //
    );
    assert!(
        !posed.residence.exists(),
        "something stands at {}",
        posed.residence.display(), //
    );
    assert!(
        haynet.bktu_root().is_dir(),
        "the haynet's root does not stand at {}, so the converge never read it",
        haynet.bktu_root().display(), //
    );
}

#[test]
fn bkth_an_archive_that_does_not_answer_the_bordereau_refuses_and_places_nothing() {
    let triple = zbkth_host();
    let archive = format!(
        "posed-{}.tar.gz",
        triple, //
    );

    // THE SEAL IS TRUE AND THE STATED ARCHIVE DIGEST IS NOT, which is the one
    // variation between this hurdle and the placement above.
    let lure = bktu_Lure::bktu_compose("heel-kibble-mismatched");
    let haynet = lure.bktu_haynet();
    let posed = zbkth_pose(
        &lure,
        &haynet,
        ZBKTH_KIBBLE,
        &archive,
        &triple,
        true,
        Some(ZBKTH_WRONG_DIGEST),
        None,
    );

    let said = zbkth_converge(&haynet, &posed.kibble)
        .expect_err("an archive that does not answer the bordereau refuses");

    assert!(
        said.contains(ZBKTH_WRONG_DIGEST),
        "the refusal does not name the stated digest: {}",
        said, //
    );
    assert!(
        said.contains(&archive),
        "the refusal does not name the archive it fetched: {}",
        said, //
    );
    assert!(
        !posed.residence.exists(),
        "something stands at {}",
        posed.residence.display(), //
    );
    assert!(
        haynet.bktu_root().is_dir(),
        "the haynet's root does not stand at {}, so the converge never read it",
        haynet.bktu_root().display(), //
    );
}

#[test]
fn bkth_a_bordereau_lacking_the_host_triple_refuses_naming_the_triple() {
    let triple = zbkth_host();
    let archive = format!(
        "posed-{}.tar.gz",
        ZBKTH_FOREIGN_TRIPLE, //
    );

    let lure = bktu_Lure::bktu_compose("heel-kibble-hostless");
    let haynet = lure.bktu_haynet();
    let posed = zbkth_pose(
        &lure,
        &haynet,
        ZBKTH_KIBBLE,
        &archive,
        ZBKTH_FOREIGN_TRIPLE,
        false,
        None,
        None,
    );

    let said = zbkth_converge(&haynet, &posed.kibble)
        .expect_err("a bordereau serving no archive for this host refuses");

    assert!(
        said.contains(&triple),
        "the refusal does not name the host triple rustc declared: {}",
        said, //
    );
    assert!(
        said.contains(ZBKTH_FOREIGN_TRIPLE),
        "the refusal does not name what the document does carry: {}",
        said, //
    );
    assert!(
        !posed.residence.exists(),
        "something stands at {}",
        posed.residence.display(), //
    );
    assert!(
        haynet.bktu_root().is_dir(),
        "the haynet's root does not stand at {}, so the converge never read it",
        haynet.bktu_root().display(), //
    );
}

#[test]
fn bkth_a_zip_archive_refuses_naming_the_format() {
    let triple = zbkth_host();
    let archive = format!(
        "posed-{}.zip",
        triple, //
    );

    // A ZIP IS WHAT UV ACTUALLY PUBLISHES FOR EVERY WINDOWS TRIPLE, so this is
    // the archive a station on the wrong host would really be handed rather than
    // a shape invented to be refused.
    let lure = bktu_Lure::bktu_compose("heel-kibble-zipped");
    let haynet = lure.bktu_haynet();
    let posed = zbkth_pose(
        &lure,
        &haynet,
        ZBKTH_KIBBLE,
        &archive,
        &triple,
        false,
        None,
        None,
    );

    let said = zbkth_converge(&haynet, &posed.kibble)
        .expect_err("an archive format MVP does not unpack refuses");

    assert!(
        said.contains(".tar.gz"),
        "the refusal does not name the format the kennel unpacks: {}",
        said, //
    );
    assert!(
        said.contains(&archive),
        "the refusal does not name the archive it would have fetched: {}",
        said, //
    );
    assert!(
        !posed.residence.exists(),
        "something stands at {}",
        posed.residence.display(), //
    );
    assert!(
        haynet.bktu_root().is_dir(),
        "the haynet's root does not stand at {}, so the converge never read it",
        haynet.bktu_root().display(), //
    );
}

/// The one kibble name every haynet hurdle poses, which keys nothing: two
/// hurdles holding it own two residences because they hold two seats.
const ZBKTH_KIBBLE: &str = "kibble-haynet";

#[test]
fn bkth_a_cold_haynet_places_a_kibble_another_haynet_already_holds() {
    let triple = zbkth_host();
    let archive = format!(
        "posed-{}.tar.gz",
        triple, //
    );

    let holding = bktu_Lure::bktu_compose("heel-haynet-holding");
    let held = holding.bktu_haynet();
    let first = zbkth_pose(
        &holding,
        &held,
        ZBKTH_KIBBLE,
        &archive,
        &triple,
        true,
        None,
        None,
    );

    let cold = bktu_Lure::bktu_compose("heel-haynet-cold");
    let empty = cold.bktu_haynet();
    let second = zbkth_pose(
        &cold,
        &empty,
        ZBKTH_KIBBLE,
        &archive,
        &triple,
        true,
        None,
        None,
    );

    zbkth_converge(&held, &first.kibble).unwrap_or_else(|err| {
        panic!(
            "the holding haynet's converge refused: {}",
            err, //
        )
    });

    let said = zbkth_converge(&empty, &second.kibble).unwrap_or_else(|err| {
        panic!(
            "the cold haynet's converge refused: {}",
            err, //
        )
    });

    // THE ASSERTION THAT REDDENED, asserted before any other: one kibble name
    // standing placed in another seat says nothing about this one.
    assert!(
        matches!(said, bkch_Kibbled::Placed(_)),
        "a cold residence answered {:?} rather than a placement",
        said, //
    );

    assert!(
        second.residence.is_file(),
        "nothing stands at {}",
        second.residence.display(), //
    );
    assert!(
        first.residence.is_file(),
        "the holding haynet's residence is gone from {}",
        first.residence.display(), //
    );
}

#[test]
fn bkth_a_lock_held_in_one_haynet_does_not_stall_a_converge_in_another() {
    let triple = zbkth_host();
    let archive = format!(
        "posed-{}.tar.gz",
        triple, //
    );

    // A LOCK STANDS IN ONE ROOT and no converge there is running, which is the
    // shape a converge interrupted mid-act leaves behind.
    let holding = bktu_Lure::bktu_compose("heel-haynet-locked");
    let held = holding.bktu_haynet();
    let lock = held.bktu_root().join(super::bkch_heel::BKCH_HELD);
    std::fs::create_dir_all(&lock).expect("the stranded lock stands");

    let free = bktu_Lure::bktu_compose("heel-haynet-unlocked");
    let open = free.bktu_haynet();
    let posed = zbkth_pose(
        &free,
        &open,
        ZBKTH_KIBBLE,
        &archive,
        &triple,
        true,
        None,
        None,
    );

    let said = zbkth_converge(&open, &posed.kibble).unwrap_or_else(|err| {
        panic!(
            "the converge in the unlocked haynet refused: {}",
            err, //
        )
    });

    assert!(
        posed.residence.is_file(),
        "the converge answered {:?} and nothing stands beneath its own root at {}",
        said,
        posed.residence.display(), //
    );
    assert!(
        matches!(said, bkch_Kibbled::Placed(_)),
        "a cold residence answered {:?} rather than a placement",
        said, //
    );
    assert!(
        lock.is_dir(),
        "the lock in the other haynet was released by a converge that never held it: {}",
        lock.display(), //
    );
}

#[test]
fn bkth_the_lures_posed_root_variable_is_the_kennels_own() {
    // THE LURE SPELLS IT RATHER THAN IMPORTING IT, being compiled a second time
    // into the integration crate where the kennel is another crate; this is what
    // holds the two spellings together.
    assert_eq!(
        super::bktu_lure::BKTU_FEEDROOM_VAR,
        super::bkcq_kibble::BKCQ_FEEDROOM_VAR
    );
}

/// Sixty-four hex characters and nothing's sha256, for the two hurdles that
/// declare a digest deliberately wrong.
const ZBKTH_WRONG_DIGEST: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const ZBKTH_WRONG_SEAL: &str = "fedcba9876543210fedcba9876543210fedcba9876543210fedcba9876543210";

/// The pin the converge hurdles' project declares — an exact patch, which the
/// validation demands and the install is spelled with.
pub(crate) const BKTH_PYTHON_PIN: &str = "3.12.7";

/// A dependency-free project declaring, in uv's own key, an environment no
/// station satisfies — so it is unhitched on every station, and the hurdles
/// reading it name no real platform.
pub(crate) const BKTH_PYTHON_UNHITCHED: &str = "\
[project]
name = \"lure\"
version = \"0.0.1\"
requires-python = \">=3.12\"

[tool.uv]
environments = [\"sys_platform == 'nowhere'\"]
";

/// A lure carrying one dependency-free uv project, the estate's own uv kibble,
/// an installed interpreter and an authored lock — everything a converge needs
/// to have been done by somebody else.
pub(crate) struct bkth_Seated {
    pub lure: bktu_Lure,
    /// The station seat the estate's own uv kibble was laid through, which a
    /// hurdle moving that declaration reaches it by again.
    pub station: bktu_Stationed,
    pub collar: crate::bkcx_python::bkcx_Collar,
    pub seat: crate::bkcx_python::bkcx_Seat,
    pub project: PathBuf,
}

/// The stated roster as the leash takes it.
fn zbkth_borne<'a>(
    stated: &'a [(&'static str, std::ffi::OsString)],
) -> Vec<(&'static str, &'a OsStr)> {
    stated
        .iter()
        .map(|(name, value)| (*name, value.as_os_str()))
        .collect()
}

/// Compose a dependency-free project awaiting its first lock: the project, its
/// collar, the estate's own uv kibble and the seat — and nothing installed or
/// authored, so the first lock's whole order is the hurdle's own to drive.
pub(crate) fn bkth_python_lockless(
    named: &str,
    requires: &str,
    dependencies: &str,
    baked: &str,
) -> bkth_Seated {
    let lure = bktu_Lure::bktu_compose(named);

    let bent = bktu_Bent {
        pin: BKTH_PYTHON_PIN.to_string(),
        requires: requires.to_string(),
        lockless: true,
        baked: baked.to_string(),
        ..bktu_Bent::default()
    };

    lure.bktu_project("project", "suite-lure", &bent);

    // THE STATION'S OWN uv, reached at the residence heel already wrote: the
    // converge below spawns it through the kennel's reader, which this run poses
    // no root for.
    let station = lure.bktu_stationed();
    station.bktu_uv_seat(&lure);

    if !dependencies.is_empty() {
        lure.bktu_write("project/pyproject.toml", dependencies);
    }

    let collar =
        crate::bkcx_python::bkcx_resolve(lure.bktu_root(), &bktr_lured(&lure), "suite-lure")
            .expect("the seated collar resolves")
            .collar;

    let tackroom = bkcl_leash::bkcl_fenced(&super::bktr_resolve::bktr_suite())
        .expect("the suite runs through a door that states the tackroom");

    let seat = crate::bkcx_python::bkcx_seat_at(lure.bktu_loosebox(), &tackroom, &collar);
    let project = lure.bktu_root().join(collar.bkcx_project());

    bkth_Seated {
        lure,
        station,
        collar,
        seat,
        project,
    }
}

/// Compose everything a converge hurdle needs to have happened already: the
/// project, the kibble, the interpreter, and the lock.
///
/// THE INTERPRETER IS INSTALLED BEFORE THE LOCK IS AUTHORED, AND THE ORDER IS THE
/// ROUTINE-DOWNLOAD RULING SHOWING ITSELF. The lock-authoring door resolves
/// against an interpreter and stands in the REACHING posture, which may reach an
/// index and may NOT fetch an interpreter — so on a station whose store is cold
/// there is no lock to author until the converge's install step has run. That is
/// not a wrinkle of this hurdle: it is the ordering the ruling imposes in
/// service, observed here because a cold store is what a hurdle meets.
///
/// EACH STEP IS ASSERTED LANDED RATHER THAN MERELY REACHED. A setup step that
/// failed quietly would leave the hurdle proving something about an absent lock
/// while reading as a statement about a stale one — which is exactly the false
/// green this testbench's own charter warns against.
pub(crate) fn bkth_python_seated(
    named: &str,
    requires: &str,
    dependencies: &str,
    baked: &str,
) -> bkth_Seated {
    let seated = bkth_python_lockless(named, requires, dependencies, baked);

    bkth_python_installed(&seated);

    // THE LOCK IS AUTHORED HERE, standing in for the lock-authoring door, so that
    // the converge driven below is converging a resolution somebody else settled
    // — which is the posture the converge actually holds in service. AUTHORED
    // PLAIN, under no bar: what stands is a lock written outside the door, which
    // is exactly what a converge meets in service and what the door's own fresh
    // resolution is proven against.
    let reaching =
        crate::bkcx_python::bkcx_stated(&seated.seat, crate::bkcx_python::bkcx_Posture::Reaching);

    let authored = bkcl_leash::bkcl_uv(
        seated.lure.bktu_root(),
        &bktr_lured(&seated.lure),
        &seated.project,
        [
            OsStr::new("lock"),
            OsStr::new("--project"),
            seated.project.as_os_str(),
        ],
        &zbkth_borne(&reaching),
    )
    .expect("the lock-authoring invocation reaches uv");

    assert!(
        authored.bkcl_landed(),
        "the lock was not authored, so there is nothing to converge: {}",
        authored.spelling, //
    );

    seated
}

/// The install step, standing in for the converge's own, over a seated project.
///
/// The store is the station's real one and is never cleared: it is immutable and
/// content-keyed, so it passes the kibble sheaf's placement test and is shared
/// exactly as the toolchain store beside it — a hurdle clearing it would make
/// every other clone on the station pay for this one's cold drive.
pub(crate) fn bkth_python_installed(seated: &bkth_Seated) {
    let fetching =
        crate::bkcx_python::bkcx_stated(&seated.seat, crate::bkcx_python::bkcx_Posture::Fetching);

    let installed = bkcl_leash::bkcl_uv(
        seated.lure.bktu_root(),
        &bktr_lured(&seated.lure),
        seated.lure.bktu_root(),
        [
            OsStr::new("python"),
            OsStr::new("install"),
            OsStr::new("--no-bin"),
            OsStr::new(BKTH_PYTHON_PIN),
        ],
        &zbkth_borne(&fetching),
    )
    .expect("the install invocation reaches uv");

    assert!(
        installed.bkcl_landed(),
        "the pinned interpreter was not installed, so nothing below proves what it claims: {}",
        installed.spelling, //
    );
}

/// The manifest of a project drawing one dependency from a local simple index.
pub(crate) fn bkth_served_manifest(dependency: &str, url: &str, uv: &str) -> String {
    format!(
        "[project]\nname = \"lure\"\nversion = \"0.0.1\"\nrequires-python = \">=3.12\"\n\
         dependencies = [\"{}\"]\n\n[tool.uv]\n{}\n[[tool.uv.index]]\nname = \"served\"\nurl = \"{}\"\n\
         default = true\n",
        dependency, uv, url, //
    )
}

/// The one converge hurdle that reaches the station, on the prebuilt kibble's
/// own precedent.
///
/// IT DRIVES A DEPENDENCY-FREE PROJECT, which is what keeps it a converge hurdle
/// rather than a resolution one: nothing is fetched from an index, so what it
/// proves is the interpreter install, the environment's placement and the lock
/// enforcement — and a dependency-bearing sync is proven on the record by a real
/// consumer's environment rather than here.
///
/// WHAT IS THIS HURDLE'S OWN IS THE ENVIRONMENT, which stands under the lure's
/// own loosebox and dies with it; the interpreter store it draws on is the
/// station's and outlives every drive.
#[test]
fn bkth_a_python_collar_converges_its_project_into_the_loosebox() {
    let seated = bkth_python_seated("heel-python-converge", ">=3.12", "", "");

    let environed = bkch_python_at(
        seated.lure.bktu_root(),
        &bktr_lured(&seated.lure),
        &seated.collar,
        &seated.seat,
    )
    .unwrap_or_else(|err| {
        panic!(
            "the converge refused: {}",
            err, //
        )
    });

    assert_eq!(
        environed.pinned,
        BKTH_PYTHON_PIN,
        "the converge installed {} where the project pins {}",
        environed.pinned,
        BKTH_PYTHON_PIN, //
    );

    // THE ENVIRONMENT STANDS UNDER THE LOOSEBOX AND NOWHERE ELSE.
    assert!(
        environed
            .environment
            .starts_with(seated.lure.bktu_loosebox()),
        "the environment landed at {}, outside the checkout's own derived root",
        environed.environment.display(), //
    );

    assert!(
        environed.environment.join("pyvenv.cfg").is_file(),
        "no environment stands at {}",
        environed.environment.display(), //
    );

    // AND NOTHING LANDED BESIDE THE PROJECT, which is the placement ruling read
    // from the other side: uv's own default would have put a `.venv` here.
    assert!(
        !seated.project.join(".venv").exists(),
        "an environment landed beside the project at {}, so the stated path did not hold",
        seated.project.display(), //
    );

    // THE VERIFY POSTURE AGREES WITH THE CONVERGE THAT JUST RAN, which is the
    // only assertion proving the two read one environment rather than two.
    crate::bkcx_python::bkcx_verify_at(
        seated.lure.bktu_root(),
        &bktr_lured(&seated.lure),
        &seated.collar,
        &seated.seat,
    )
    .unwrap_or_else(|err| {
        panic!(
            "the verify posture refused a freshly converged tree: {}",
            err, //
        )
    });

    // DRIVEN AGAIN IT IS CURRENT AND FETCHES NOTHING. The sealed posture can
    // reach neither an index nor a download, so a second converge needing either
    // would refuse rather than quietly succeed — which is how this hurdle tells a
    // converged environment from a repeated one.
    let sealed =
        crate::bkcx_python::bkcx_stated(&seated.seat, crate::bkcx_python::bkcx_Posture::Sealed);

    let again = bkcl_leash::bkcl_uv(
        seated.lure.bktu_root(),
        &bktr_lured(&seated.lure),
        &seated.project,
        [
            OsStr::new("sync"),
            OsStr::new("--locked"),
            OsStr::new("--project"),
            seated.project.as_os_str(),
        ],
        &zbkth_borne(&sealed),
    )
    .expect("the second sync reaches uv");

    assert!(
        again.bkcl_landed(),
        "a converged environment did not answer a sealed sync, so the first converge left \
         something to fetch: {}",
        again.spelling, //
    );
}

/// The converge refuses a dependency it would have to build from its source,
/// and names the rivet; uv's own account names the package.
///
/// THE LOCK IS A FIXTURE, as the converge rule's own shape demands: the
/// lock-authoring door is barred from writing a lock over a package with no
/// wheel, so the lock a converge could meet carrying one is a lock written
/// outside that door — composed here by hand, exactly as such a lock would be.
/// The package is served from a simple index as an sdist alone, a placeholder
/// the bar keeps uv from ever opening.
///
/// TWO READINGS, EACH PROVING ITS OWN HALF. The door's refusal is the kennel's
/// sentence, which cites the rivet; uv's sentence goes to the terminal under
/// the door, so it is recalled here under the door's own roster, where it names
/// the package and says it has no binary distribution.
#[test]
fn bkth_the_converge_refuses_a_dependencys_source_build_naming_it_and_the_rivet() {
    let seated = bkth_python_lockless("heel-python-source-build", ">=3.12", "", "lure-marrow");

    let url = seated.lure.bktu_simple_index(
        "lure-marrow",
        &[(
            "lure_marrow-1.0.tar.gz".to_string(),
            b"not an archive\n".to_vec(),
        )],
    );

    seated.lure.bktu_write(
        "project/pyproject.toml",
        &bkth_served_manifest("lure-marrow", &url, ""),
    );

    let registry = url.trim_start_matches("file://").to_string();
    let served = seated
        .lure
        .bktu_temp()
        .join("index")
        .join("files")
        .join("lure_marrow-1.0.tar.gz");

    seated.lure.bktu_write(
        "project/uv.lock",
        &format!(
            "version = 1\nrevision = 3\nrequires-python = \">=3.12\"\n\n\
             [[package]]\nname = \"lure\"\nversion = \"0.0.1\"\nsource = {{ virtual = \".\" }}\n\
             dependencies = [\n    {{ name = \"lure-marrow\" }},\n]\n\n\
             [package.metadata]\nrequires-dist = [{{ name = \"lure-marrow\" }}]\n\n\
             [[package]]\nname = \"lure-marrow\"\nversion = \"1.0\"\nsource = {{ registry = \"{}\" }}\n\
             sdist = {{ path = \"{}\" }}\n",
            registry,
            served.display(), //
        ),
    );

    bkth_python_installed(&seated);

    let lock = seated.project.join("uv.lock");
    let before = std::fs::read(&lock).expect("the fixture lock reads");

    let err = bkch_python_at(
        seated.lure.bktu_root(),
        &bktr_lured(&seated.lure),
        &seated.collar,
        &seated.seat,
    )
    .expect_err("a dependency with no wheel refuses the converge rather than being built");

    assert!(
        err.contains("BKr_hpt"),
        "the refusal does not cite the rule that bars the build: {}",
        err, //
    );

    assert_eq!(
        before,
        std::fs::read(&lock).expect("the lock still reads"),
        "the converge moved the lock at {} while refusing",
        lock.display(), //
    );

    // UV'S OWN SENTENCE, under the roster the door states: every lock package
    // but the project's own barred from building.
    let barred = crate::bkcx_python::bkcx_lock_packages(&seated.project)
        .expect("the fixture lock's packages read");

    assert_eq!(
        barred,
        vec!["lure-marrow".to_string()],
        "the roster is read off the lock", //
    );

    let mut syncing =
        crate::bkcx_python::bkcx_stated(&seated.seat, crate::bkcx_python::bkcx_Posture::Reaching);
    syncing.push(crate::bkcx_python::bkcx_barred(&barred));

    let recalled = bkcl_leash::bkcl_uv_recall(
        seated.lure.bktu_root(),
        &bktr_lured(&seated.lure),
        &seated.project,
        [
            OsStr::new("sync"),
            OsStr::new("--locked"),
            OsStr::new("--project"),
            seated.project.as_os_str(),
        ],
        &zbkth_borne(&syncing),
    )
    .expect("the sync reaches uv");

    assert!(
        !recalled.run.bkcl_landed(),
        "uv synced a package it was barred from building: {}",
        recalled.grievance, //
    );

    assert!(
        recalled.grievance.contains("lure-marrow")
            && recalled.grievance.contains("no binary distribution"),
        "uv's account does not name the package and the absent wheel: {}",
        recalled.grievance, //
    );
}

#[test]
fn bkth_a_manifest_the_lock_does_not_answer_refuses_and_authors_nothing() {
    let seated = bkth_python_seated("heel-python-stale", ">=3.12", "", "");

    let lock = seated.project.join("uv.lock");

    assert!(
        lock.is_file(),
        "the control: a lock stands at {} before the manifest is moved past it",
        seated.project.display(), //
    );

    let before = std::fs::read(&lock).expect("the authored lock reads");

    // THE MANIFEST MOVES OUT FROM UNDER THE LOCK, by declaring a dependency the
    // authored resolution never accounted for.
    seated.lure.bktu_write(
        "project/pyproject.toml",
        "[project]\nname = \"lure\"\nversion = \"0.0.1\"\nrequires-python = \">=3.12\"\n\
         dependencies = [\"a-name-no-index-carries\"]\n",
    );

    let err = bkch_python_at(
        seated.lure.bktu_root(),
        &bktr_lured(&seated.lure),
        &seated.collar,
        &seated.seat,
    )
    .expect_err("a manifest the lock does not answer refuses the converge");

    // WHAT THIS HURDLE PROVES IS THE KENNEL'S HALF, AND ITS NAME SAYS SO. Which
    // sentence uv gave — a lock that would have to change, a name no index
    // carries, a pin the requirement excludes — is uv's own and does not reach
    // this refusal at all: the converge hands uv this process's streams, so uv's
    // account goes to the operator's terminal rather than into the error. An
    // assertion here naming the lock's staleness would be reading a sentence
    // nothing in this process ever saw, which is how the first draft of this
    // hurdle went green while proving something about an absent lock. The stale
    // lock's own sentence is proven on the bench instead.
    assert!(
        err.contains("did not sync"),
        "the refusal is not the converge's own: {}",
        err, //
    );

    // AND IT NAMES THE DOOR ENTITLED TO RE-DERIVE A LOCK, rather than re-deriving.
    assert!(
        err.contains("lock-authoring door"),
        "the refusal does not name where a lock is authored: {}",
        err, //
    );

    // THE LOAD-BEARING ASSERTION: THE LOCK IS UNTOUCHED. This is the property the
    // whole posture exists for and the only one that would catch a converge that
    // quietly relocked to make itself succeed — a refusal can be read from the
    // exit, but authorship can only be read from the bytes.
    let after = std::fs::read(&lock).expect("the lock still reads");

    assert_eq!(
        before,
        after,
        "the converge re-derived the lock at {} instead of refusing, so a resolution nobody \
         reviewed would have landed under a green exit",
        lock.display(), //
    );
}

/// THE CONVERGE REFUSES AN UNHITCHED PROJECT, naming its declaration, and leaves
/// no environment standing. uv's real sync creates the environment before it
/// refuses a station outside the lock's supported environments, so a converge
/// that reached the sync would leave one standing hollow; the reading stands
/// between the install and the sync.
#[test]
fn bkth_the_converge_refuses_an_unhitched_project_and_builds_no_environment() {
    let seated = bkth_python_seated("heel-python-unhitched", ">=3.12", BKTH_PYTHON_UNHITCHED, "");

    let refused = match bkch_python_at(
        seated.lure.bktu_root(),
        &bktr_lured(&seated.lure),
        &seated.collar,
        &seated.seat,
    ) {
        Ok(environed) => panic!(
            "the converge synced a project whose declaration no station satisfies, at {}",
            environed.environment.display(), //
        ),
        Err(refused) => refused,
    };

    assert!(
        refused.contains(crate::bkg_breviary::BKG_UNHITCHED_BASE) && refused.contains("nowhere"),
        "the converge's refusal does not name the declaration: {}",
        refused, //
    );
    assert!(
        !seated.seat.environment.exists(),
        "the converge left an environment standing at {}",
        seated.seat.environment.display(), //
    );
}

/// Two heels of one posed root, driven at once, take the checkout's yard in
/// turn: the second says its one line and waits, and both converge green.
///
/// THE PROJECT IS ONE WHOSE LOCK IS ALREADY SATISFIED, seated and installed
/// before either heel starts, so neither converge reaches past the station and
/// what is under test is the ordering alone. Both converge the SAME environment
/// under the same loosebox — the shape two heels of one collar in one checkout
/// take — and the yard is taken exactly as the door takes it, ahead of the
/// converge and held through it.
///
/// THE FIRST HEEL HOLDS THE YARD UNTIL THE SECOND HAS SAID IT IS WAITING, so the
/// wait is observed rather than raced for: a yard that took no lock would let
/// the second heel through without a word, and the first would wait out its
/// bound and fail here, naming that.
#[test]
fn bkth_two_heels_of_one_root_take_the_yard_in_turn_and_both_converge() {
    use std::sync::{
        mpsc,
        Mutex, //
    };
    use std::time::Duration;

    let seated = bkth_python_seated("heel-two-at-once", ">=3.12", "", "");
    let loosebox = seated.lure.bktu_loosebox().to_path_buf();
    let order: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

    let converge = || {
        bkch_python_at(
            seated.lure.bktu_root(),
            &bktr_lured(&seated.lure),
            &seated.collar,
            &seated.seat,
        )
    };

    let first = bkch_yard(&loosebox, |at| {
        panic!(
            "the first heel met a held yard at {}",
            at.display(), //
        )
    })
    .unwrap_or_else(|err| {
        panic!(
            "the first heel could not take the yard: {}",
            err, //
        )
    });

    let (said, heard) = mpsc::channel::<PathBuf>();

    std::thread::scope(|scope| {
        let second = scope.spawn(|| {
            let yard = bkch_yard(&loosebox, |at| {
                let _ = said.send(at.to_path_buf());
            })
            .unwrap_or_else(|err| {
                panic!(
                    "the second heel could not take the yard: {}",
                    err, //
                )
            });
            order
                .lock()
                .expect("the order reads")
                .push("second took the yard");
            let environed = converge();
            yard.bkch_release();
            environed
        });

        let line = heard.recv_timeout(Duration::from_secs(30));
        assert!(
            line.is_ok(),
            "the second heel said nothing while the first held the yard, so it was not made to \
             wait", //
        );
        assert_eq!(
            line.expect("the line was said"),
            loosebox.join(super::bkch_heel::BKCH_HEEL_HELD),
            "the waiting line named some other file than the yard's", //
        );

        converge().unwrap_or_else(|err| {
            panic!(
                "the first heel's converge refused: {}",
                err, //
            )
        });
        order
            .lock()
            .expect("the order reads")
            .push("first converged");
        first.bkch_release();

        second
            .join()
            .expect("the second heel ran to its end")
            .unwrap_or_else(|err| {
                panic!(
                    "the second heel's converge refused: {}",
                    err, //
                )
            });
    });

    assert_eq!(
        *order.lock().expect("the order reads"),
        vec!["first converged", "second took the yard"],
        "the second heel took the yard before the first had finished converging", //
    );
}

/// The cadence the slow build below is attended at — short, so a hurdle can wait
/// out several of them — and how long its build script sleeps past it.
const ZBKTH_SLOW_CADENCE: std::time::Duration = std::time::Duration::from_millis(100);
const ZBKTH_SLOW_SLEEP_MS: u64 = 1500;

/// A SLOW BUILD SPEAKS HEARTBEATS ALONE AT THE CADENCE between its launch mark
/// and its verdict.
///
/// IN-PROCESS, AND THAT IS WHAT MAKES THE CADENCE A HURDLE'S TO CHOOSE. The
/// door's cadence is ten seconds, which no hurdle should wait out; the build
/// road takes the voice as a value, so this one hands it a voice attended at a
/// tenth of a second and speaking onto a console this hurdle holds, rather than
/// onto the stdout every hurdle thread shares.
///
/// THE PLANT IS A BUILD SCRIPT THAT SLEEPS, so the child is genuinely silent past
/// many cadences — cargo says nothing while a build script runs — and the
/// heartbeat has a silence of the child's own to answer, as well as the quiet
/// console's.
#[test]
fn bkth_a_slow_build_speaks_heartbeats_alone_at_the_cadence() {
    use super::bkch_heel::zbkch_voiced_build;
    use super::bkcv_voice::bkcv_Cargo;
    use std::cell::RefCell;
    use std::rc::Rc;

    let lure = bktu_Lure::bktu_compose("heel-slow-build");
    lure.bktu_write(".gitignore", "target/\n");
    lure.bktu_write(
        "Tools/dawdle/Cargo.toml",
        "[package]\nname = \"dawdle\"\nversion = \"0.0.1\"\nedition = \"2021\"\n",
    );
    lure.bktu_write(
        "Tools/dawdle/Cargo.lock",
        "version = 4\n\n[[package]]\nname = \"dawdle\"\nversion = \"0.0.1\"\n",
    );
    lure.bktu_write("Tools/dawdle/src/main.rs", "fn main() {}\n");
    lure.bktu_write(
        "Tools/dawdle/build.rs",
        &format!(
            "fn main() {{\n    std::thread::sleep(std::time::Duration::from_millis({}));\n}}\n",
            ZBKTH_SLOW_SLEEP_MS, //
        ),
    );
    lure.bktu_commit("plant a crate whose build script sleeps past the cadence");

    let heard: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let console = {
        let heard = Rc::clone(&heard);
        Box::new(move |line: &str| heard.borrow_mut().push(line.to_string()))
    };

    let voice = bkcv_Cargo::zbkcv_build_with(None, None, ZBKTH_SLOW_CADENCE, console);
    let manifest = lure.bktu_root().join("Tools/dawdle/Cargo.toml");

    let (run, built) = zbkch_voiced_build(
        voice,
        lure.bktu_root(),
        &bktr_lured(&lure),
        &manifest,
        &[],
        &[],
    )
    .expect("the build road reaches cargo");

    let spoken = heard.borrow().clone();

    assert!(
        built,
        "the slow build lands: {} {:?}",
        run.spelling,
        spoken, //
    );
    assert!(
        spoken
            .first()
            .is_some_and(|line| line.starts_with("launching: ")),
        "the console opens on the launch mark: {:?}",
        spoken, //
    );
    assert!(
        spoken
            .last()
            .is_some_and(|line| line.starts_with("green: ")),
        "the console closes on the verdict: {:?}",
        spoken, //
    );

    let between = &spoken[1..spoken.len() - 1];
    assert!(
        between.iter().all(|line| line.starts_with("  · ")),
        "between the launch mark and the verdict the console carries heartbeats alone: {:?}",
        spoken, //
    );

    // AT THE CADENCE: a sleep of fifteen cadences cannot pass under fewer than a
    // handful of marks, and a voice that marked only once — keyed on something
    // other than the console's own silence — would fall short of it.
    assert!(
        between.len() >= 5,
        "a build silent for {}ms at a {}ms cadence carries a heartbeat per cadence: {:?}",
        ZBKTH_SLOW_SLEEP_MS,
        ZBKTH_SLOW_CADENCE.as_millis(),
        spoken, //
    );
}

// eof
