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

//! The binary election's hurdles.
//!
//! THE ARMS ARE DRIVEN OVER COMPOSED POSITIONS, NEVER OVER THE LAUNCH STAND, and
//! that is forced rather than tidy: a live seat stands in exactly one arm of this
//! election at a time and can never be made to stand in the arm it is not in. A
//! seat that borrows cannot also have outrun, and neither can be a writer. So the
//! rule is driven over stamps and walks composed for the purpose, and the
//! readings that go and get those facts are driven over a lure.

use super::bkce_election::{
    bkce_Geography,
    bkce_geography,
    BKCE_GEOGRAPHY_DISPATCHED,
    BKCE_GEOGRAPHY_SELF, //
};
use super::bkce_election::{
    bkce_carried,
    bkce_concord,
    bkce_elect,
    bkce_residence,
    bkce_seat,
    bkce_weigh,
    BKCE_ELECTED_BORROW,
    BKCE_ELECTED_OWN,
    BKCE_ELECTED_PINNED,
    BKCE_EXERGUE_FLAG,
    BKCE_SEAT_FAR,
    BKCE_SPEND_WRITER, //
};
use super::bkch_heel::BKCH_COLLAR_POSITION_VAR;
use super::bkcr_resolve::bkcr_Collar;
use super::bktd_preflight::bktd_strike_at;
use super::bktu_lure::bktu_Lure;
use bkp::bkprc_catena::bkprc_admit;
use bkp::bkprs_scrivener::bkprs_program_write;
use std::path::{
    Path,
    PathBuf, //
};

/// The spend class that does not pin. Spelled here because every arm below needs
/// a class to contrast the writer against.
const ZBKTE_READER: &str = "bknre_reader";

/// A composed stamp-and-walk: the stamp its own first element, exactly as the
/// exergue writes one and as a binary hands it back.
///
/// The shapes are real object names rather than short tokens, because the reading
/// that gathers them tests a line for a position's SHAPE and a short token would
/// clear no such test.
fn zbkte_carried() -> Vec<String> {
    vec![
        "1111111111111111111111111111111111111111".to_string(),
        "2222222222222222222222222222222222222222".to_string(),
        "3333333333333333333333333333333333333333".to_string(),
    ]
}

/// A position no composed walk holds.
const ZBKTE_ELSEWHERE: &str = "9999999999999999999999999999999999999999";

/// THE PIN ARM. A writer is pinned to the blessed residence whatever any position
/// says, and the position it is handed here is one that would elect its own build
/// under any other class.
///
/// THE CONTROL IS THE SAME FACTS UNDER A READER, and it is not decoration: an
/// election that returned the pin for everything would clear the first assertion.
/// What says the spend class is doing the work is that one letter of the input
/// changes the answer and nothing else does.
#[test]
fn bkte_a_writer_is_pinned_whatever_the_position_says() {
    assert_eq!(
        bkce_weigh(BKCE_SPEND_WRITER, false, None),
        (BKCE_ELECTED_PINNED, true),
        "a writer never enters the tree question", //
    );

    assert_eq!(
        bkce_weigh(ZBKTE_READER, false, None),
        (BKCE_ELECTED_OWN, false),
        "the control: the same fact under a reader elects the seat's own build, so the pin above \
         is the spend class and not a constant", //
    );
}

/// THE BORROW AND OWN-BUILD ARMS. A reader borrows exactly when the tree under
/// the collar's elected roots is the same at both positions, and elects its own
/// build otherwise.
///
/// THE RULE IS ONE BIT WIDE NOW, so what a hurdle over it can prove is that the
/// bit decides and the spend class gates it. WHERE THE BIT COMES FROM is the
/// whole substance of this election and is not provable here at all — it is
/// proven over lures below, against repositories that actually fork, merge and
/// diverge.
#[test]
fn bkte_the_tree_decides_and_the_spend_class_gates() {
    assert_eq!(
        bkce_weigh(ZBKTE_READER, true, None),
        (BKCE_ELECTED_BORROW, true),
        "an identical tree makes an identical binary", //
    );

    assert_eq!(
        bkce_weigh(ZBKTE_READER, false, None),
        (BKCE_ELECTED_OWN, false),
        "a tree that differs is a binary this seat did not make", //
    );
}

/// FAIL CLOSED ON A BINARY THAT CANNOT SAY. An artifact whose tree is
/// unestablished may not be presumed to match: the cost of being wrong here is a
/// rebuild, and the cost of the opposite error is a run against a vintage nobody
/// can name.
///
/// THE PLAINT IS ASSERTED AND NOT ONLY THE VERDICT, because every fail-closed
/// case reaches the rule as the same `false` and the plaint is the only thing
/// that tells a reader which one they met.
#[test]
fn bkte_a_binary_that_states_no_position_stands_in_no_concord() {
    let lure = bktu_Lure::bktu_compose("election-mute");
    lure.bktu_write("src/one.rs", "// elected\n");
    lure.bktu_commit("plant an elected root");

    let mute = bkce_concord(lure.bktu_root(), None, BKCE_SEAT_FAR, "src");
    assert!(
        !mute.identical,
        "a binary that states no position is not borrowable", //
    );
    assert!(
        mute.plaint
            .contains("would not say what it was struck from"),
        "the plaint names the case: {}",
        mute.plaint, //
    );

    let unresolvable = bkce_concord(
        lure.bktu_root(),
        Some(ZBKTE_ELSEWHERE),
        BKCE_SEAT_FAR,
        "src",
    );
    assert!(
        !unresolvable.identical,
        "a position this tree cannot look up is no comparison", //
    );
    assert!(
        unresolvable
            .plaint
            .contains("names no commit in this repository"),
        "the plaint parts an unresolvable stamp from an absent one: {}",
        unresolvable.plaint, //
    );

    let standing = zbkte_head(&lure);
    let control = bkce_concord(lure.bktu_root(), Some(&standing), BKCE_SEAT_FAR, "src");
    assert!(
        control.identical && control.plaint.is_empty(),
        "the control: the same lure stands in concord with its own HEAD, so the refusals above \
         are the cases and not the reading being unable to answer at all", //
    );
}

/// THE ARTIFACT IS ASKED, and this drives the asking over a lure: a binary
/// planted at a residence that answers the exergue word with a composed
/// stamp-and-walk.
///
/// THE CONTROL IS A BINARY THAT ANSWERS PROSE. A reading that returned whatever
/// the child printed would hand the election a banner line to compare positions
/// against, and the shape test is what stops it — proven by driving one rather
/// than by reading the code that holds it.
#[test]
fn bkte_the_walk_is_read_from_the_standing_binary() {
    let lure = bktu_Lure::bktu_compose("election-carried");
    let carried = zbkte_carried();

    let speaking = zbkte_plant(&lure, "speaking", &carried.join("\n"));
    assert_eq!(
        bkce_carried(&speaking),
        carried,
        "the binary's own answer, one position per line, the stamp first", //
    );

    let mute = zbkte_plant(
        &lure,
        "mute",
        "bkx 0.0.1 — a banner, and no position at all",
    );
    assert!(
        bkce_carried(&mute).is_empty(),
        "a line that is not position-shaped is no position, and an election handed one would \
         compare against a banner", //
    );

    let absent = lure.bktu_root().join("nothing-stands-here");
    assert!(
        bkce_carried(&absent).is_empty(),
        "a residence holding no binary states no position, which is the same answer by the same \
         reasoning", //
    );
}

/// THE SEAT POSITION IS WALKED OVER THE ELECTION, NEVER BARE `HEAD`, and this is
/// the property driven rather than asserted. A commit touching no elected root
/// must not move the position: an artifact that did not change may not read as
/// outrun.
///
/// The commit that DOES touch an elected root is the control, and it is the half
/// that would catch a reading frozen at the wrong end — a position that never
/// moved at all would clear the first assertion and fail this one.
#[test]
fn bkte_the_seat_position_moves_only_for_an_elected_root() {
    let lure = bktu_Lure::bktu_compose("election-seat");
    let elected = "src";

    lure.bktu_write("src/one.rs", "// elected\n");
    lure.bktu_write("notes/one.md", "unelected\n");
    lure.bktu_commit("plant both trees");

    let first = bkce_seat(lure.bktu_root(), elected).expect("the elected root has been touched");

    lure.bktu_write("notes/two.md", "still unelected\n");
    lure.bktu_commit("touch nothing the election names");

    let after_unelected = bkce_seat(lure.bktu_root(), elected).expect("the reading still answers");
    assert_eq!(
        first, after_unelected,
        "a commit outside the election leaves the position where it stood — otherwise every \
         commit anywhere in the repository would stale an artifact it did not touch", //
    );

    lure.bktu_write("src/two.rs", "// elected, and newer\n");
    lure.bktu_commit("touch an elected root");

    let after_elected = bkce_seat(lure.bktu_root(), elected).expect("the reading still answers");
    assert_ne!(
        first, after_elected,
        "the control: a commit the election DOES name moves the position, so the reading above is \
         the election at work rather than a position that never moves", //
    );
}

/// AN ELECTION NAMING NOTHING REFUSES. A position measured over an empty set
/// would be the same for every tree and would call every binary current, which is
/// the one answer this reading may never give quietly.
#[test]
fn bkte_an_empty_election_refuses_rather_than_answering() {
    let lure = bktu_Lure::bktu_compose("election-empty");

    assert!(
        bkce_seat(lure.bktu_root(), "   ").is_err(),
        "an election naming no root is refused, never measured", //
    );
    assert!(
        bkce_seat(lure.bktu_root(), "rust-toolchain.toml").is_ok(),
        "the control: the same lure answers over a root it does carry", //
    );
}

/// Plant an executable at the lure that answers the exergue word with the text
/// given, and hand back its path.
///
/// A SHELL SCRIPT STANDS IN FOR A BINARY DELIBERATELY. What the election asks of
/// an artifact is that it answer one word with its positions; nothing about that
/// contract is a property of being compiled, and a composed answerer is what lets
/// a hurdle drive a stamp-and-walk no real binary in this estate carries.
///
/// THE PLANT IS WRITTEN BY A CHILD, which is what lets the fail-closed reading
/// be trusted here. `bkce_carried` reaches an artifact it cannot spawn as the
/// same empty answer a mute one gives, so a plant this process wrote itself —
/// which a sibling thread's fork could still hold open for writing when the
/// election execs it — would read as a broken READING when what failed was the
/// plant. The lure's program writer leaves no such handle anywhere
/// (`bktu_lure.rs` "A PROGRAM IS WRITTEN BY A CHILD").
fn zbkte_plant(lure: &bktu_Lure, name: &str, answer: &str) -> std::path::PathBuf {
    bkte_plant_at(
        lure,
        &format!(
            "residence/{}",
            name, //
        ),
        answer,
    )
}

/// Plant the same answerer at a path the caller spells whole.
///
/// THE PREFLIGHT'S HURDLES POSE AN OUTRUN RESIDENCE THIS WAY, at the residence
/// THEIR collar joins, which is why the path is a parameter here while the
/// name-only face above stays for this file's own arms. One plant serves both
/// files, so what a posed residence answers is composed in one place.
pub(crate) fn bkte_plant_at(lure: &bktu_Lure, relative: &str, answer: &str) -> std::path::PathBuf {
    let planted = lure.bktu_root().join(relative);
    bkprs_program_write(
        &planted,
        format!(
            "#!/bin/sh\ntest \"$1\" = \"{}\" || exit 64\ncat <<'ANSWER'\n{}\nANSWER\n",
            BKCE_EXERGUE_FLAG,
            answer, //
        ),
    )
    .unwrap_or_else(|err| {
        panic!(
            "{}",
            err, //
        )
    });

    planted
}

/// An app collar whose BYNAME DIFFERS FROM ITS TARGET, which is the whole case
/// this field exists for. `crib` is what cargo builds; `manger` is what stands at
/// the residence once something renamed it on the way there.
const ZBKTE_RENAMED: &str = "\
BKRR_COLLAR=\"app-lure\"
BKRR_KIND=\"bknre_app\"
BKRR_MANIFEST=\"Cargo.toml\"
BKRR_TARGET=\"crib\"
BKRR_ROOTS=\"src\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RESIDENCE=\"kennel/bin\"
BKRR_BYNAME=\"manger\"
";

/// THE JOIN READS THE BYNAME AND NEVER THE TARGET. A join over the target is what
/// this pace found in the field: the engine's collar elected its residence
/// correctly and then named a file nobody had ever put there, because cargo's name
/// for what it builds is not the name the artifact wears once a parcel has
/// emplaced it. The two are spelled apart here so that a join quietly reverted to
/// the target fails rather than passing on a collar where they happen to agree.
#[test]
fn bkte_the_residence_joins_the_byname_rather_than_the_target() {
    let collar = bkcr_Collar {
        name: "app-lure".to_string(),
        instance: PathBuf::from("app-lure"),
        regime: bkprc_admit(ZBKTE_RENAMED, "composed")
            .expect("the collar stands inside the subset"),
    };

    let stood = bkce_residence(Path::new("/seat"), &collar);

    assert_eq!(
        stood,
        PathBuf::from("/seat/kennel/bin/manger"),
        "the residence joins the name the binary wears there", //
    );
    assert!(
        !stood.ends_with("crib"),
        "the target is what cargo builds and never what the join names: {}",
        stood.display(), //
    );
}

/// The lure's own `HEAD`, which a hurdle needs to compose an answerer stating a
/// position the repository actually holds.
///
/// LOCAL TO THIS FILE. The lure's git surface writes and does not read, and one
/// consumer is not a reason to widen a shared composing surface; a second hurdle
/// file wanting this is what would move it.
fn zbkte_head(lure: &bktu_Lure) -> String {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(lure.bktu_root())
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .unwrap_or_else(|err| {
            panic!(
                "could not read the lure's HEAD: {}",
                err, //
            )
        });

    assert!(
        out.status.success(),
        "git would not read the lure's HEAD: {}",
        String::from_utf8_lossy(&out.stderr).trim(), //
    );

    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// An app collar over a lure: a reader whose residence and byname name the
/// answerer a hurdle plants, and whose elected roots are whatever that hurdle is
/// proving.
fn zbkte_app_collar(roots: &str) -> bkcr_Collar {
    let text = format!(
        "\
BKRR_COLLAR=\"app-lure\"
BKRR_KIND=\"bknre_app\"
BKRR_MANIFEST=\"Cargo.toml\"
BKRR_TARGET=\"answerer\"
BKRR_ROOTS=\"{}\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RESIDENCE=\"residence\"
BKRR_BYNAME=\"answerer\"
",
        roots, //
    );

    bkcr_Collar {
        name: "app-lure".to_string(),
        instance: PathBuf::from("app-lure"),
        regime: bkprc_admit(&text, "composed").expect("the collar stands inside the subset"),
    }
}

/// A SEAT FORKED BEHIND A TRUNK THAT MOVED THE ROOTS ELECTS ITS OWN BUILD.
///
/// This is the arm the retired walk reading got WRONG, and it got it wrong in the
/// direction that runs a stale binary: the binary's walk carries the landing this
/// seat is forked at, so membership found the seat and borrowed — while the tree
/// the binary was struck from holds a file this seat has never seen.
#[test]
fn bkte_a_seat_forked_behind_a_moved_trunk_elects_its_own_build() {
    let lure = bktu_Lure::bktu_compose("election-forked");
    lure.bktu_git(&["branch", "-M", "trunk"]);

    lure.bktu_write("src/one.rs", "// the founding source\n");
    lure.bktu_commit("plant the elected root");

    lure.bktu_git(&["checkout", "--quiet", "-b", "seat"]);
    lure.bktu_git(&["checkout", "--quiet", "trunk"]);
    lure.bktu_write("src/two.rs", "// trunk moved the roots\n");
    lure.bktu_commit("trunk moves an elected root");
    let struck = zbkte_head(&lure);

    lure.bktu_git(&["checkout", "--quiet", "seat"]);
    zbkte_plant(&lure, "answerer", &struck);

    let verdict = bkce_elect(
        lure.bktu_root(),
        &bkce_Geography::Undispatched,
        &zbkte_app_collar("src"),
    )
    .expect("the election answers");

    assert_eq!(
        verdict.elected, BKCE_ELECTED_OWN,
        "the binary was struck from a tree this seat does not hold", //
    );
    assert!(
        verdict
            .plaint
            .as_deref()
            .unwrap_or_default()
            .contains("src/two.rs"),
        "the refusal names what the seat is missing: {:?}",
        verdict.plaint, //
    );
}

/// THAT TRUNK MERGED INTO THE SEAT BORROWS, and this is the arm an exact stamp
/// match would get wrong forever.
///
/// A refit is a merge, and a merge commit stands on no trunk line, so a reading
/// comparing positions could never find these two equal however identical the
/// trees. The `assert_ne!` below is not decoration: it is what says this hurdle
/// composed the refit shape rather than a fast-forward that would have made the
/// borrow trivially true.
#[test]
fn bkte_a_seat_that_merged_the_moved_trunk_borrows() {
    let lure = bktu_Lure::bktu_compose("election-merged");
    lure.bktu_git(&["branch", "-M", "trunk"]);

    lure.bktu_write("src/one.rs", "// the founding source\n");
    lure.bktu_commit("plant the elected root");

    lure.bktu_git(&["checkout", "--quiet", "-b", "seat"]);
    lure.bktu_write("notes/seat.md", "the seat's own work, off the election\n");
    lure.bktu_commit("the seat moves, outside the roots");

    lure.bktu_git(&["checkout", "--quiet", "trunk"]);
    lure.bktu_write("src/two.rs", "// trunk moved the roots\n");
    lure.bktu_commit("trunk moves an elected root");
    let struck = zbkte_head(&lure);

    lure.bktu_git(&["checkout", "--quiet", "seat"]);
    lure.bktu_git(&["merge", "--quiet", "--no-ff", "--no-edit", "trunk"]);

    assert_ne!(
        zbkte_head(&lure),
        struck,
        "the refit's own shape: HEAD is a merge commit standing on no trunk line, so a position \
         comparison has nothing to match", //
    );

    zbkte_plant(&lure, "answerer", &struck);

    assert_eq!(
        bkce_elect(
            lure.bktu_root(),
            &bkce_Geography::Undispatched,
            &zbkte_app_collar("src")
        )
        .expect("the election answers")
        .elected,
        BKCE_ELECTED_BORROW,
        "the merge brought the roots to the tree the binary was struck from", //
    );
}

/// A LOCAL COMMIT UNDER THE ROOTS ELECTS ITS OWN BUILD, whatever line it stands
/// on. This is the arm that must survive the widening — a reading loose enough to
/// forgive a refit must still refuse the seat's own edit.
#[test]
fn bkte_a_local_commit_under_the_roots_elects_its_own_build() {
    let lure = bktu_Lure::bktu_compose("election-edited");

    lure.bktu_write("src/one.rs", "// the founding source\n");
    lure.bktu_commit("plant the elected root");
    let struck = zbkte_head(&lure);

    lure.bktu_write("src/one.rs", "// the seat's own edit\n");
    lure.bktu_commit("the seat edits an elected root");

    zbkte_plant(&lure, "answerer", &struck);

    let verdict = bkce_elect(
        lure.bktu_root(),
        &bkce_Geography::Undispatched,
        &zbkte_app_collar("src"),
    )
    .expect("the election answers");

    assert_eq!(
        verdict.elected, BKCE_ELECTED_OWN,
        "the seat has outrun the binary on its own line", //
    );
    assert!(
        verdict
            .plaint
            .as_deref()
            .unwrap_or_default()
            .contains("src/one.rs"),
        "the refusal names the difference a reader can act on: {:?}",
        verdict.plaint, //
    );
}

/// A COMMIT UNDER AN EXCLUDED PATH BORROWS. The exclusion pathspecs are part of
/// the elected roots and reach git verbatim, so a change the collar carves out is
/// no change to what the artifact is made from.
///
/// THE CONTROL IS THE SAME COMMIT READ WITHOUT THE CARVE-OUT, and it is what
/// makes this hurdle mean anything: a comparison that saw no difference for some
/// unrelated reason would pass the first assertion and fail the second.
#[test]
fn bkte_a_commit_under_an_excluded_path_borrows() {
    let lure = bktu_Lure::bktu_compose("election-excluded");

    lure.bktu_write("src/one.rs", "// the founding source\n");
    lure.bktu_write("src/bktx_hurdle.rs", "// a hurdle, carved back out\n");
    lure.bktu_commit("plant the elected root and the carve-out");
    let struck = zbkte_head(&lure);

    lure.bktu_write(
        "src/bktx_hurdle.rs",
        "// the hurdle moves, and no artifact is made from it\n",
    );
    lure.bktu_commit("touch only what the exclusion carves out");

    zbkte_plant(&lure, "answerer", &struck);

    assert_eq!(
        bkce_elect(
            lure.bktu_root(),
            &bkce_Geography::Undispatched,
            &zbkte_app_collar("src :(exclude)src/bkt*")
        )
        .expect("the election answers")
        .elected,
        BKCE_ELECTED_BORROW,
        "the exclusion reaches git verbatim, so the carved-out change is no change at all", //
    );

    assert_eq!(
        bkce_elect(
            lure.bktu_root(),
            &bkce_Geography::Undispatched,
            &zbkte_app_collar("src")
        )
        .expect("the election answers")
        .elected,
        BKCE_ELECTED_OWN,
        "the control: the same tree over roots that do NOT carve it out elects own build, so the \
         borrow above is the exclusion at work rather than a diff that saw nothing", //
    );
}

/// THE OWN RESIDENCE DECIDES ONLY WHERE THE DELIVERED ONE FAILED, and it decides
/// the POSTURE rather than the word.
///
/// This is the arm the dispatch layer added, and it is the one place the elected
/// word and the run/refuse posture come apart: a linked worktree whose delivered
/// binary is outrun and whose OWN build is current elects its own build AND runs
/// it, where a main worktree reaching the same word is refused. The `None`
/// arm is what keeps the undispatched case exactly what it was — there is no
/// second residence to ask, so the question was never put.
#[test]
fn bkte_the_own_residence_decides_the_posture_and_never_the_word() {
    assert_eq!(
        bkce_weigh(ZBKTE_READER, false, Some(true)),
        (BKCE_ELECTED_OWN, true),
        "a linked worktree holding a current own build runs it rather than being sent to make one \
         it already has", //
    );

    assert_eq!(
        bkce_weigh(ZBKTE_READER, false, Some(false)),
        (BKCE_ELECTED_OWN, false),
        "an own build that is absent or outrun refuses, which is the whole of what the second \
         reading can say against the seat", //
    );

    assert_eq!(
        bkce_weigh(ZBKTE_READER, false, None),
        (BKCE_ELECTED_OWN, false),
        "no second residence was asked about, which is the undispatched case and must answer \
         exactly as it did before there was a second reading at all", //
    );

    // THE BORROW ARM IS UNMOVED BY THE SECOND READING, which is what says the
    // order is a rule rather than an accident: a delivered binary this seat can
    // borrow is borrowed whatever the seat's own residence holds.
    assert_eq!(
        bkce_weigh(ZBKTE_READER, true, Some(false)),
        (BKCE_ELECTED_BORROW, true),
        "an equivalent delivered binary is borrowed however stale the seat's own", //
    );

    // AND SO IS THE PIN, which never enters either question.
    assert_eq!(
        bkce_weigh(BKCE_SPEND_WRITER, false, Some(true)),
        (BKCE_ELECTED_PINNED, true),
        "a writer is pinned to the blessed residence whatever the seat's own build says", //
    );
}

/// A LINKED WORKTREE TAKES THE RESIDENCE JOIN TWICE, over two roots, and an
/// undispatched one takes it once.
///
/// THE COLLAR'S COORDINATES ARE THE SAME COORDINATES EITHER WAY. What dispatch
/// changes is which ROOT the join is taken over, never how it is taken — so the
/// two residences are one function called twice, and a collar renamed in one
/// place moves both.
#[test]
fn bkte_a_dispatched_seat_joins_two_residences_and_a_self_one_joins_one() {
    let lure = bktu_Lure::bktu_compose("election-two-residences");
    let delivered = bktu_Lure::bktu_compose("election-two-residences-delivered");

    lure.bktu_write("src/one.rs", "// the founding source\n");
    lure.bktu_commit("plant the elected root");
    let struck = zbkte_head(&lure);

    zbkte_plant(&lure, "answerer", &struck);
    zbkte_plant(&delivered, "answerer", &struck);

    let collar = zbkte_app_collar("src");

    let dispatched = bkce_elect(
        lure.bktu_root(),
        &bkce_Geography::Dispatched(delivered.bktu_root().to_path_buf()),
        &collar,
    )
    .expect("the election answers");

    assert_eq!(
        dispatched.residence,
        bkce_residence(delivered.bktu_root(), &collar),
        "the borrow candidate joins onto the delivered root", //
    );
    assert_eq!(
        dispatched.own,
        bkce_residence(lure.bktu_root(), &collar),
        "the own residence joins onto the work tree", //
    );
    assert_ne!(
        dispatched.residence, dispatched.own,
        "under dispatch the two are different files, which is the whole reason there are two", //
    );

    // THE CONTROL IS THE SAME COLLAR UNDER SELF-GEOGRAPHY, where the two roots
    // are one tree: without it, a verdict that always carried two paths would
    // clear the assertions above whether or not the geography decided anything.
    let self_read = bkce_elect(lure.bktu_root(), &bkce_Geography::Undispatched, &collar)
        .expect("the election answers");

    assert_eq!(
        self_read.residence, self_read.own,
        "the control: a main worktree has one residence under two names", //
    );
}

/// A LINKED WORKTREE BORROWS THE DELIVERED BINARY WHEN THE TREES MATCH, and the
/// binary it runs is the DELIVERED one.
///
/// THE PATH IS ASSERTED AND NOT ONLY THE WORD, because the word alone cannot say
/// which of two files was elected — and running the wrong one of them is exactly
/// the failure the two residences exist to prevent.
#[test]
fn bkte_a_dispatched_seat_borrows_the_delivered_binary_when_the_trees_match() {
    let lure = bktu_Lure::bktu_compose("election-dispatched-borrow");
    let delivered = bktu_Lure::bktu_compose("election-dispatched-borrow-delivered");

    lure.bktu_write("src/one.rs", "// the founding source\n");
    lure.bktu_commit("plant the elected root");
    let struck = zbkte_head(&lure);

    zbkte_plant(&delivered, "answerer", &struck);
    // THE SEAT'S OWN RESIDENCE IS PLANTED OUTRUN ON PURPOSE, so a borrow that was
    // in fact reading the own residence would fail rather than pass.
    zbkte_plant(&lure, "answerer", ZBKTE_ELSEWHERE);

    let verdict = bkce_elect(
        lure.bktu_root(),
        &bkce_Geography::Dispatched(delivered.bktu_root().to_path_buf()),
        &zbkte_app_collar("src"),
    )
    .expect("the election answers");

    assert_eq!(
        verdict.elected, BKCE_ELECTED_BORROW,
        "the delivered binary was struck from the tree this seat holds", //
    );
    assert_eq!(
        verdict.answers.as_deref(),
        Some(verdict.residence.as_path()),
        "the delivered binary is the one that runs", //
    );
    assert!(
        verdict.own_plaint.is_none(),
        "an outrun own build says nothing against a borrow that answered: {:?}",
        verdict.own_plaint, //
    );
    assert!(
        verdict.plaint.is_none(),
        "a borrow that answered carries no plaint for the proclamation to say: {:?}",
        verdict.plaint, //
    );
}

/// A LINKED WORKTREE RUNS ITS OWN BUILD WHERE THE DELIVERED ONE IS OUTRUN. This
/// is the arm the pace added, and without it a linked worktree that had already
/// built its own work would be sent to build it again on every launch — an
/// own-build verdict under dispatch could never run at all.
#[test]
fn bkte_a_dispatched_seat_runs_a_current_own_build() {
    let lure = bktu_Lure::bktu_compose("election-dispatched-own");
    let delivered = bktu_Lure::bktu_compose("election-dispatched-own-delivered");

    lure.bktu_write("src/one.rs", "// the founding source\n");
    lure.bktu_commit("plant the elected root");
    let behind = zbkte_head(&lure);

    lure.bktu_write("src/one.rs", "// the seat's own work\n");
    lure.bktu_commit("the seat moves an elected root");
    let struck = zbkte_head(&lure);

    // THE DELIVERED BINARY IS REAL AND SIMPLY BEHIND: it states a position this
    // repository resolves, so the refusal it earns is a parted tree rather than
    // an unreadable stamp, which is the shape a live linked worktree meets.
    zbkte_plant(&delivered, "answerer", &behind);
    zbkte_plant(&lure, "answerer", &struck);

    let verdict = bkce_elect(
        lure.bktu_root(),
        &bkce_Geography::Dispatched(delivered.bktu_root().to_path_buf()),
        &zbkte_app_collar("src"),
    )
    .expect("the election answers");

    assert_eq!(
        verdict.elected, BKCE_ELECTED_OWN,
        "the delivered binary was struck from a tree this seat has moved past", //
    );
    assert!(
        verdict.bkce_runs(),
        "a current own build runs: {:?}",
        verdict.plaint, //
    );
    assert_eq!(
        verdict.answers.as_deref(),
        Some(verdict.own.as_path()),
        "the binary that runs is the seat's OWN and never the delivered one", //
    );

    // THE CONTROL IS THE SAME LINKED WORKTREE WITH ITS OWN RESIDENCE OUTRUN TOO,
    // which is what says the own reading was CONSULTED rather than the arm being
    // unconditional. Without it, a rule that ran whenever the geography was
    // dispatched would clear every assertion above.
    zbkte_plant(&lure, "answerer", &behind);

    let outrun = bkce_elect(
        lure.bktu_root(),
        &bkce_Geography::Dispatched(delivered.bktu_root().to_path_buf()),
        &zbkte_app_collar("src"),
    )
    .expect("the election answers");

    assert!(
        !outrun.bkce_runs(),
        "the control: the same linked worktree whose own build is outrun too is refused, so the \
         run above is the own residence's currency and not the geography alone", //
    );
}

/// A LINKED WORKTREE RUNS ITS CURRENT OWN BUILD AHEAD OF AN IDENTICAL BORROW,
/// and the plaint says why. A delivered binary matching the seat's tree was
/// borrowed whatever the seat had built, which is the road that ran a binary its
/// own staleness fence then refused while the billet's own fresh build stood
/// beside it, unasked.
///
/// THE CONTROL IS THE SAME SEAT WITH ITS OWN BUILD OUTRUN, which borrows: without
/// it, a rule that elected the own build whenever one stood would clear the
/// assertions above whether or not its currency was read.
#[test]
fn bkte_a_dispatched_seat_with_a_current_own_build_runs_it_ahead_of_an_identical_borrow() {
    assert_eq!(
        bkce_weigh(ZBKTE_READER, true, Some(true)),
        (BKCE_ELECTED_OWN, true),
        "a current own build answers ahead of a borrow the trees would allow", //
    );

    let lure = bktu_Lure::bktu_compose("election-dispatched-own-ahead");
    let delivered = bktu_Lure::bktu_compose("election-dispatched-own-ahead-delivered");

    lure.bktu_write("src/one.rs", "// the founding source\n");
    lure.bktu_commit("plant the elected root");
    let struck = zbkte_head(&lure);

    // BOTH BINARIES ARE STRUCK FROM THE SEAT'S OWN TREE, so the delivered one is
    // borrowable on the trees alone and only the order of the rule decides.
    zbkte_plant(&delivered, "answerer", &struck);
    zbkte_plant(&lure, "answerer", &struck);

    let geography = bkce_Geography::Dispatched(delivered.bktu_root().to_path_buf());
    let collar = zbkte_app_collar("src");

    let verdict = bkce_elect(lure.bktu_root(), &geography, &collar).expect("the election answers");

    assert_eq!(
        verdict.elected, BKCE_ELECTED_OWN,
        "the seat's own build is current, so it is elected over the identical borrow", //
    );
    assert_eq!(
        verdict.answers.as_deref(),
        Some(verdict.own.as_path()),
        "the binary that runs is the seat's OWN and never the delivered one", //
    );

    let why = verdict
        .plaint
        .as_deref()
        .expect("an own build elected over an identical borrow says why");
    assert!(
        why.contains("answers ahead of a borrow")
            && why.contains(&verdict.residence.display().to_string()),
        "the plaint names the borrow passed over and the reason: {}",
        why, //
    );

    zbkte_plant(&lure, "answerer", ZBKTE_ELSEWHERE);

    let outrun = bkce_elect(lure.bktu_root(), &geography, &collar).expect("the election answers");

    assert_eq!(
        outrun.elected, BKCE_ELECTED_BORROW,
        "the control: an outrun own build leaves the identical borrow to answer", //
    );
    assert_eq!(
        outrun.answers.as_deref(),
        Some(outrun.residence.as_path()),
        "the control: the delivered binary is the one that runs", //
    );
}

/// A LINKED WORKTREE WHOSE BORROW CANDIDATE ITS OWN FENCE WOULD REFUSE ELECTS ITS
/// OWN BUILD. The delivered binary was struck from the very tree this seat holds,
/// so the trees match; but the trunk has since moved beneath the exergue its
/// build struck, which is what that binary's staleness fence judges, and a
/// borrow here is a run refused at the binary's own door.
///
/// THREE READINGS OVER ONE SEAT: the trunk unmoved, which is the control and
/// borrows; the trunk moved beneath the delivered exergue with no own build,
/// which elects the own build and refuses, naming the fence reading; and the
/// same with a current own build standing, which runs it.
#[test]
fn bkte_a_dispatched_seat_whose_borrow_the_fence_refuses_elects_its_own_build() {
    let lure = bktu_Lure::bktu_compose("election-dispatched-fenced");
    let delivered = bktu_Lure::bktu_compose("election-dispatched-fenced-delivered");

    lure.bktu_git(&["branch", "-M", "trunk"]);
    lure.bktu_write("src/one.rs", "// the founding source\n");
    lure.bktu_commit("plant the elected root");

    // THE TRUNK COUNTERPART IS A REAL REMOTE with its head recorded, because the
    // fence reading resolves the remote's recorded head exactly as the striker
    // and the preflight do.
    let bare = lure.bktu_temp().join("counterpart.git");
    let spelled = bare.display().to_string();
    lure.bktu_git(&["init", "--quiet", "--bare", &spelled]);
    lure.bktu_git(&["remote", "add", "origin", &spelled]);
    lure.bktu_git(&["push", "--quiet", "origin", "trunk"]);
    lure.bktu_git(&["remote", "set-head", "origin", "trunk"]);
    let landed = zbkte_head(&lure);

    // THE DELIVERED ROOT HOLDS WHAT ITS BUILD LEAVES: a binary stating the seat
    // position it was built at, and the exergue struck at the landing it was
    // built against, both of the tree this seat holds.
    zbkte_plant(&delivered, "answerer", &landed);
    bktd_strike_at(&delivered, "planchet/gen_exergue.rs", &landed, &["src"]);

    let text = "\
BKRR_COLLAR=\"app-lure\"
BKRR_KIND=\"bknre_app\"
BKRR_MANIFEST=\"Cargo.toml\"
BKRR_TARGET=\"answerer\"
BKRR_ROOTS=\"src\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"planchet/gen_exergue.rs\"
BKRR_STRIKE=\"tt/lure-b.Strike.sh\"
BKRR_RESIDENCE=\"residence\"
BKRR_BYNAME=\"answerer\"
";
    let collar = bkcr_Collar {
        name: "app-lure".to_string(),
        instance: PathBuf::from("app-lure"),
        regime: bkprc_admit(text, "composed").expect("the collar stands inside the subset"),
    };
    let geography = bkce_Geography::Dispatched(delivered.bktu_root().to_path_buf());

    let unmoved = bkce_elect(lure.bktu_root(), &geography, &collar).expect("the election answers");

    assert_eq!(
        unmoved.elected, BKCE_ELECTED_BORROW,
        "the control: a delivered exergue struck at the trunk's own landing admits the borrow: {:?}",
        unmoved.plaint, //
    );

    // THE TRUNK MOVES AND THE SEAT DOES NOT, so the seat still holds the tree the
    // delivered binary was struck from and only the counterpart has moved.
    lure.bktu_git(&["checkout", "--quiet", "-b", "seat"]);
    lure.bktu_git(&["checkout", "--quiet", "trunk"]);
    lure.bktu_write("src/two.rs", "// the trunk moves an elected root\n");
    lure.bktu_commit("the trunk moves an elected root");
    lure.bktu_git(&["push", "--quiet", "origin", "trunk"]);
    lure.bktu_git(&["checkout", "--quiet", "seat"]);

    let fenced = bkce_elect(lure.bktu_root(), &geography, &collar).expect("the election answers");

    assert_eq!(
        fenced.elected, BKCE_ELECTED_OWN,
        "a borrow the fence refuses is not borrowed however identical the trees", //
    );
    assert!(
        !fenced.bkce_runs(),
        "no own build stands, so the seat is refused rather than handed the fenced binary", //
    );

    let said = fenced.bkce_grievance("app-lure", "heel");
    assert!(
        said.contains("staleness fence refuses it")
            && said.contains("origin/trunk")
            && said.contains("src/two.rs"),
        "the refusal names the fence reading, the counterpart and what moved: {}",
        said, //
    );

    let own_landed = zbkte_head(&lure);
    zbkte_plant(&lure, "answerer", &own_landed);

    let built = bkce_elect(lure.bktu_root(), &geography, &collar).expect("the election answers");

    assert_eq!(built.elected, BKCE_ELECTED_OWN);
    assert_eq!(
        built.answers.as_deref(),
        Some(built.own.as_path()),
        "a current own build runs where the borrow is fenced", //
    );
}

/// NEITHER RESIDENCE ANSWERING REFUSES, AND THE REFUSAL NAMES BOTH. A reader
/// refused by two binaries and told about one of them goes and converges the
/// wrong tree — which under dispatch means running heel against a delivered root
/// that is not theirs to write.
#[test]
fn bkte_a_dispatched_seat_holding_neither_refuses_naming_both() {
    let lure = bktu_Lure::bktu_compose("election-dispatched-neither");
    let delivered = bktu_Lure::bktu_compose("election-dispatched-neither-delivered");

    lure.bktu_write("src/one.rs", "// the founding source\n");
    lure.bktu_commit("plant the elected root");
    let behind = zbkte_head(&lure);

    lure.bktu_write("src/one.rs", "// the seat's own work\n");
    lure.bktu_commit("the seat moves an elected root");

    // THE DELIVERED ONE IS BEHIND AND THE SEAT'S OWN IS SIMPLY NOT THERE, which
    // is the shape a freshly added linked worktree stands in before its first converge.
    zbkte_plant(&delivered, "answerer", &behind);

    let verdict = bkce_elect(
        lure.bktu_root(),
        &bkce_Geography::Dispatched(delivered.bktu_root().to_path_buf()),
        &zbkte_app_collar("src"),
    )
    .expect("the election answers");

    assert_eq!(verdict.elected, BKCE_ELECTED_OWN);
    assert!(
        !verdict.bkce_runs(),
        "neither residence holds a binary this seat may run", //
    );
    assert!(
        verdict.answers.is_none(),
        "a refused election names no binary to spawn", //
    );

    let said = verdict.bkce_grievance("app-lure", "heel");

    assert!(
        said.contains(&verdict.residence.display().to_string()),
        "the refusal names the delivered binary it could not borrow: {}",
        said, //
    );
    assert!(
        said.contains(&verdict.own.display().to_string()),
        "the refusal names the seat's own residence, which is the tree the converge writes: {}",
        said, //
    );
    assert!(
        said.contains("heel"),
        "the refusal names the converge rather than performing one: {}",
        said, //
    );

    // THE CONTROL IS AN UNDISPATCHED REFUSAL OVER THE SAME SEAT, which must NOT
    // carry a second sentence: a grievance that always spoke of two residences
    // would clear the assertions above without the second reading ever happening.
    let self_read = bkce_elect(
        lure.bktu_root(),
        &bkce_Geography::Undispatched,
        &zbkte_app_collar("src"),
    )
    .expect("the election answers");

    assert!(
        self_read.own_plaint.is_none(),
        "the control: a main worktree put no second question, so its refusal says nothing \
         about a second residence: {:?}",
        self_read.own_plaint, //
    );
}

/// A MUTE BINARY WHERE THE CONVERGE WRITES REFUSES NAMING THE CONTRACT, never the
/// converge as its cure. A crate that gives the exergue flag no answer rebuilds
/// into a binary that gives none, so a refusal sending its reader to the converge
/// alone sends them round a loop. Both geographies are driven, since the binary
/// the converge writes is read off a different residence in each.
///
/// THE CONTROL IS AN EMPTY RESIDENCE OVER THE SAME SEAT, which the converge does
/// cure and which keeps naming it as the act — so the arm is the binary's silence
/// and not every refusal reworded.
#[test]
fn bkte_a_mute_binary_where_the_converge_writes_names_the_contract() {
    let lure = bktu_Lure::bktu_compose("election-mute-own");
    let delivered = bktu_Lure::bktu_compose("election-mute-own-delivered");
    lure.bktu_write("src/one.rs", "// elected\n");
    lure.bktu_commit("plant an elected root");
    let collar = zbkte_app_collar("src");

    let empty = bkce_elect(lure.bktu_root(), &bkce_Geography::Undispatched, &collar)
        .expect("the election answers");
    let unsaid = empty.bkce_grievance("app-lure", "heel");
    assert!(
        !empty.bkce_runs() && !empty.mute,
        "the control: nothing stands at the residence, so nothing is mute", //
    );
    assert!(
        !unsaid.contains(BKCE_EXERGUE_FLAG)
            && unsaid.contains("the converge, run deliberately: heel"),
        "the control names the converge as the act and no contract: {}",
        unsaid, //
    );

    zbkte_plant(
        &lure,
        "answerer",
        "hello from a crate that never learned the flag",
    );

    for (geography, which) in [
        (bkce_Geography::Undispatched, "a main worktree"),
        (
            bkce_Geography::Dispatched(delivered.bktu_root().to_path_buf()),
            "a linked worktree whose delivered residence is empty",
        ),
    ] {
        let verdict =
            bkce_elect(lure.bktu_root(), &geography, &collar).expect("the election answers");
        let said = verdict.bkce_grievance("app-lure", "heel");

        assert!(
            !verdict.bkce_runs(),
            "{}: a binary that cannot say may not run",
            which, //
        );
        assert!(
            verdict.mute,
            "{}: the binary the converge writes stands mute",
            which, //
        );
        assert!(
            said.contains(BKCE_EXERGUE_FLAG) && said.contains(BKCH_COLLAR_POSITION_VAR),
            "{}: the refusal names the contract the crate owes: {}",
            which,
            said, //
        );
        assert!(
            said.contains("once the crate answers")
                && !said.contains("the converge, run deliberately"),
            "{}: the converge follows the crate's remedy and is not offered as the cure: {}",
            which,
            said, //
        );
    }
}

/// A lure's root as git names it: resolved, since git answers a directory by its
/// real path and a temp root may be reached through a symlink.
fn zbkte_real(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|err| {
        panic!(
            "could not resolve {}: {}",
            path.display(),
            err, //
        )
    })
}

/// A MAIN WORKTREE IS SELF AND ITS LINKED WORKTREE IS DISPATCHED, the delivered
/// root being that main worktree.
///
/// GEOGRAPHY IS DERIVED FROM A REAL TREE, NEVER POSED. The worktree is stood by
/// git's own worktree add, the same act that stands every work billet inside its
/// sire's standing clone, so the reading here is the reading a dispatched door
/// takes and not a stand-in for it.
///
/// THE CONTROL IS THE MAIN WORKTREE ITSELF, asked the same question: a reader
/// that answered dispatched for everything would clear the first assertion, and
/// only the tree it borrows from reading as self says the two answers are the
/// geography and not a constant.
#[test]
fn bkte_a_linked_worktree_derives_its_main_checkout_as_the_delivered_root() {
    let main = bktu_Lure::bktu_compose("geography-derived");
    let billet = main.bktu_billet("geography-derived");

    assert_eq!(
        bkce_geography(billet.bktu_root()).expect("a linked worktree derives"),
        bkce_Geography::Dispatched(zbkte_real(main.bktu_root())),
        "a linked worktree's delivered root is its main worktree", //
    );

    assert_eq!(
        bkce_geography(main.bktu_root()).expect("a main worktree derives"),
        bkce_Geography::Undispatched,
        "the control: the main worktree it was added from is self-geography", //
    );
}

/// A PLAIN CLONE IS SELF, like a main worktree, and is the geography every
/// consumer's cold clone stands in: no worktree, one residence, nothing stated.
#[test]
fn bkte_a_plain_clone_is_self_geography() {
    let lure = bktu_Lure::bktu_compose("geography-plain");

    assert_eq!(
        bkce_geography(lure.bktu_root()).expect("a plain repository derives"),
        bkce_Geography::Undispatched,
        "a repository with no worktrees works the tree it entered", //
    );
}

/// THE STATED FORM IS WHAT THE PROCLAMATION AND THE ELECTION LINE BOTH SPELL, so
/// it is asserted here rather than at either seat: a catena whose first element
/// is the word and whose second is the root, read off a derived geography.
#[test]
fn bkte_a_derived_geography_states_its_word_and_root() {
    let main = bktu_Lure::bktu_compose("geography-stated");
    let billet = main.bktu_billet("geography-stated");

    let read = bkce_geography(billet.bktu_root()).expect("a linked worktree derives");

    assert_eq!(
        read.bkce_stated(),
        format!(
            "{} {}",
            BKCE_GEOGRAPHY_DISPATCHED,
            zbkte_real(main.bktu_root()).display(), //
        ),
        "a dispatched geography states its word and its root as one catena", //
    );

    assert_eq!(
        bkce_Geography::Undispatched.bkce_stated(),
        BKCE_GEOGRAPHY_SELF,
        "the control: self-geography states one element and names no root, so the two forms are \
         told apart by a consumer reading the first element", //
    );
}

/// A WORKTREE ADDED FROM A BARE REPOSITORY REFUSES, naming the common directory
/// git answered, and never reads as self.
///
/// IT IS THE ONE ANSWER GIT GIVES THAT NAMES NO MAIN WORKTREE, and a bare
/// repository holds no work tree for a built binary to stand in, so there is no
/// delivered root to borrow from and none may be invented — the parent of a bare
/// repository is an unrelated directory that merely happens to hold it.
#[test]
fn bkte_a_worktree_of_a_bare_repository_refuses() {
    let main = bktu_Lure::bktu_compose("geography-bare");
    let billet = main.bktu_billet_of_bare("geography-bare");

    let said = bkce_geography(billet.bktu_root())
        .expect_err("a worktree of a bare repository names no delivered root");

    assert!(
        said.contains("lure.git"),
        "the refusal names the common directory git answered: {}",
        said, //
    );

    // THE CONTROL IS A WORKTREE OF THE SAME CONTENT ADDED FROM A MAIN WORKTREE,
    // so the refusal above is the bareness and not the worktree.
    let control = main.bktu_billet("geography-bare-control");

    assert!(
        bkce_geography(control.bktu_root()).is_ok(),
        "the control: a worktree added from a main worktree derives", //
    );
}

/// A LINKED WORKTREE WHOSE MAIN WORKTREE IS GONE REFUSES, naming the tree, and
/// never falls back to self.
///
/// THE FALLBACK IS WHAT THIS FORBIDS. A reader that read past a missing main
/// checkout would turn it into a quiet own build: the seat would look ordinary,
/// and the delivered root it was added from would never be consulted by anybody.
#[test]
fn bkte_a_worktree_whose_main_checkout_is_gone_refuses() {
    let main = bktu_Lure::bktu_compose("geography-orphan");
    let billet = main.bktu_billet("geography-orphan");

    // THE CONTROL COMES FIRST, over the same worktree while its main worktree
    // stands: the refusal below is then the removal and not the worktree.
    assert!(
        bkce_geography(billet.bktu_root()).is_ok(),
        "the control: the worktree derives while its main worktree stands", //
    );

    std::fs::remove_dir_all(main.bktu_root()).expect("the main worktree comes away");

    let said = bkce_geography(billet.bktu_root())
        .expect_err("a worktree whose main worktree is gone derives nothing");

    assert!(
        said.contains(&billet.bktu_root().display().to_string()),
        "the refusal names the tree whose geography could not be derived: {}",
        said, //
    );
}

// eof
