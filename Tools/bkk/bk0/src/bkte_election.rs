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
    bkce_carried, bkce_concord, bkce_elect, bkce_residence, bkce_seat, bkce_weigh,
    BKCE_ELECTED_BORROW, BKCE_ELECTED_OWN, BKCE_ELECTED_PINNED, BKCE_EXERGUE_FLAG,
    BKCE_SPEND_WRITER,
};
use super::bkca_whereabouts::bkca_Geography;
use super::bkcr_resolve::bkcr_Collar;
use super::bktu_lure::bktu_Lure;
use bkl::bklrc_catena::bklrc_admit;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

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
        "a writer never enters the tree question"
    );

    assert_eq!(
        bkce_weigh(ZBKTE_READER, false, None),
        (BKCE_ELECTED_OWN, false),
        "the control: the same fact under a reader elects the seat's own build, so the pin above \
         is the spend class and not a constant"
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
        "an identical tree makes an identical binary"
    );

    assert_eq!(
        bkce_weigh(ZBKTE_READER, false, None),
        (BKCE_ELECTED_OWN, false),
        "a tree that differs is a binary this seat did not make"
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

    let mute = bkce_concord(lure.bktu_root(), None, "src");
    assert!(!mute.identical, "a binary that states no position is not borrowable");
    assert!(
        mute.plaint.contains("would not say what it was struck from"),
        "the plaint names the case: {}",
        mute.plaint
    );

    let unresolvable = bkce_concord(lure.bktu_root(), Some(ZBKTE_ELSEWHERE), "src");
    assert!(!unresolvable.identical, "a position this tree cannot look up is no comparison");
    assert!(
        unresolvable.plaint.contains("names no commit in this repository"),
        "the plaint parts an unresolvable stamp from an absent one: {}",
        unresolvable.plaint
    );

    let standing = zbkte_head(&lure);
    let control = bkce_concord(lure.bktu_root(), Some(&standing), "src");
    assert!(
        control.identical && control.plaint.is_empty(),
        "the control: the same lure stands in concord with its own HEAD, so the refusals above \
         are the cases and not the reading being unable to answer at all"
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
        "the binary's own answer, one position per line, the stamp first"
    );

    let mute = zbkte_plant(&lure, "mute", "bkx 0.0.1 — a banner, and no position at all");
    assert!(
        bkce_carried(&mute).is_empty(),
        "a line that is not position-shaped is no position, and an election handed one would \
         compare against a banner"
    );

    let absent = lure.bktu_root().join("nothing-stands-here");
    assert!(
        bkce_carried(&absent).is_empty(),
        "a residence holding no binary states no position, which is the same answer by the same \
         reasoning"
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
         commit anywhere in the repository would stale an artifact it did not touch"
    );

    lure.bktu_write("src/two.rs", "// elected, and newer\n");
    lure.bktu_commit("touch an elected root");

    let after_elected = bkce_seat(lure.bktu_root(), elected).expect("the reading still answers");
    assert_ne!(
        first, after_elected,
        "the control: a commit the election DOES name moves the position, so the reading above is \
         the election at work rather than a position that never moves"
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
        "an election naming no root is refused, never measured"
    );
    assert!(
        bkce_seat(lure.bktu_root(), "rust-toolchain.toml").is_ok(),
        "the control: the same lure answers over a root it does carry"
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
/// THE PLANT PROVES ITSELF SPAWNABLE BEFORE IT IS HANDED BACK, and that is not
/// belt-and-braces — it is the only way this hurdle can tell its two failures
/// apart. `bkce_carried` is fail-closed by design: an artifact it cannot spawn
/// reaches the election as the same empty answer a mute one does, which is right
/// for a reading whose question is whether the tree is established. Under a
/// parallel runner a script written, made executable and exec'd within the same
/// instant meets `ETXTBSY` — another thread's fork having inherited a write
/// handle to the very inode — and the hurdle would then assert on an empty
/// answer and report the READING as broken when what failed was the plant. So
/// the spawn is taken here, where an error is an error.
fn zbkte_plant(lure: &bktu_Lure, name: &str, answer: &str) -> std::path::PathBuf {
    let relative = format!("residence/{}", name);

    lure.bktu_write(
        &relative,
        &format!(
            "#!/bin/sh\ntest \"$1\" = \"{}\" || exit 64\ncat <<'ANSWER'\n{}\nANSWER\n",
            BKCE_EXERGUE_FLAG, answer
        ),
    );

    let planted = lure.bktu_root().join(&relative);
    std::fs::set_permissions(&planted, std::fs::Permissions::from_mode(0o755))
        .unwrap_or_else(|err| panic!("could not make {} executable: {}", planted.display(), err));

    zbkte_spawnable(&planted);

    planted
}

/// How many times a plant is offered to the kernel before the hurdle calls it
/// unspawnable, and how long it waits between offers.
///
/// A BOUND AND NOT A LOOP. The condition this waits out is another thread
/// closing a handle, which happens on its own in microseconds or does not happen
/// at all; a wait without a bound would turn a real defect into a hung suite.
const ZBKTE_OFFERS: usize = 50;
const ZBKTE_PAUSE: std::time::Duration = std::time::Duration::from_millis(20);

/// Prove the planted answerer actually runs, waiting out the write handle a
/// neighbouring fork may still hold on it.
fn zbkte_spawnable(planted: &Path) {
    let mut last = String::new();

    for _ in 0..ZBKTE_OFFERS {
        match std::process::Command::new(planted).arg(BKCE_EXERGUE_FLAG).output() {
            Ok(_) => return,
            Err(err) => {
                last = err.to_string();
                std::thread::sleep(ZBKTE_PAUSE);
            }
        }
    }

    panic!(
        "the planted answerer at {} never became spawnable: {} — the hurdle's own plant failed,          which the election's fail-closed reading would otherwise have reported as a binary that          states no position",
        planted.display(),
        last
    );
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
        regime: bklrc_admit(ZBKTE_RENAMED, "composed").expect("the collar stands inside the subset"),
    };

    let stood = bkce_residence(Path::new("/seat"), &collar);

    assert_eq!(
        stood,
        PathBuf::from("/seat/kennel/bin/manger"),
        "the residence joins the name the binary wears there"
    );
    assert!(
        !stood.ends_with("crib"),
        "the target is what cargo builds and never what the join names: {}",
        stood.display()
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
        .unwrap_or_else(|err| panic!("could not read the lure's HEAD: {}", err));

    assert!(
        out.status.success(),
        "git would not read the lure's HEAD: {}",
        String::from_utf8_lossy(&out.stderr).trim()
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
        roots
    );

    bkcr_Collar {
        name: "app-lure".to_string(),
        instance: PathBuf::from("app-lure"),
        regime: bklrc_admit(&text, "composed").expect("the collar stands inside the subset"),
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

    let verdict =
        bkce_elect(lure.bktu_root(), &bkca_Geography::Undispatched, &zbkte_app_collar("src")).expect("the election answers");

    assert_eq!(
        verdict.elected, BKCE_ELECTED_OWN,
        "the binary was struck from a tree this seat does not hold"
    );
    assert!(
        verdict.plaint.as_deref().unwrap_or_default().contains("src/two.rs"),
        "the refusal names what the seat is missing: {:?}",
        verdict.plaint
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
         comparison has nothing to match"
    );

    zbkte_plant(&lure, "answerer", &struck);

    assert_eq!(
        bkce_elect(lure.bktu_root(), &bkca_Geography::Undispatched, &zbkte_app_collar("src"))
            .expect("the election answers")
            .elected,
        BKCE_ELECTED_BORROW,
        "the merge brought the roots to the tree the binary was struck from"
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

    let verdict =
        bkce_elect(lure.bktu_root(), &bkca_Geography::Undispatched, &zbkte_app_collar("src")).expect("the election answers");

    assert_eq!(
        verdict.elected, BKCE_ELECTED_OWN,
        "the seat has outrun the binary on its own line"
    );
    assert!(
        verdict.plaint.as_deref().unwrap_or_default().contains("src/one.rs"),
        "the refusal names the difference a reader can act on: {:?}",
        verdict.plaint
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

    lure.bktu_write("src/bktx_hurdle.rs", "// the hurdle moves, and no artifact is made from it\n");
    lure.bktu_commit("touch only what the exclusion carves out");

    zbkte_plant(&lure, "answerer", &struck);

    assert_eq!(
        bkce_elect(lure.bktu_root(), &bkca_Geography::Undispatched, &zbkte_app_collar("src :(exclude)src/bkt*"))
            .expect("the election answers")
            .elected,
        BKCE_ELECTED_BORROW,
        "the exclusion reaches git verbatim, so the carved-out change is no change at all"
    );

    assert_eq!(
        bkce_elect(lure.bktu_root(), &bkca_Geography::Undispatched, &zbkte_app_collar("src"))
            .expect("the election answers")
            .elected,
        BKCE_ELECTED_OWN,
        "the control: the same tree over roots that do NOT carve it out elects own build, so the \
         borrow above is the exclusion at work rather than a diff that saw nothing"
    );
}

/// THE OWN RESIDENCE DECIDES ONLY WHERE THE DELIVERED ONE FAILED, and it decides
/// the POSTURE rather than the word.
///
/// This is the arm the dispatch layer added, and it is the one place the elected
/// word and the run/refuse posture come apart: a dispatched seat whose delivered
/// binary is outrun and whose OWN build is current elects its own build AND runs
/// it, where an undispatched seat reaching the same word is refused. The `None`
/// arm is what keeps the undispatched case exactly what it was — there is no
/// second residence to ask, so the question was never put.
#[test]
fn bkte_the_own_residence_decides_the_posture_and_never_the_word() {
    assert_eq!(
        bkce_weigh(ZBKTE_READER, false, Some(true)),
        (BKCE_ELECTED_OWN, true),
        "a dispatched seat holding a current own build runs it rather than being sent to make one \
         it already has"
    );

    assert_eq!(
        bkce_weigh(ZBKTE_READER, false, Some(false)),
        (BKCE_ELECTED_OWN, false),
        "an own build that is absent or outrun refuses, which is the whole of what the second \
         reading can say against the seat"
    );

    assert_eq!(
        bkce_weigh(ZBKTE_READER, false, None),
        (BKCE_ELECTED_OWN, false),
        "no second residence was asked about, which is the undispatched case and must answer \
         exactly as it did before there was a second reading at all"
    );

    // THE BORROW ARM IS UNMOVED BY THE SECOND READING, which is what says the
    // order is a rule rather than an accident: a delivered binary this seat can
    // borrow is borrowed whatever the seat's own residence holds.
    assert_eq!(
        bkce_weigh(ZBKTE_READER, true, Some(false)),
        (BKCE_ELECTED_BORROW, true),
        "an equivalent delivered binary is borrowed however stale the seat's own"
    );

    // AND SO IS THE PIN, which never enters either question.
    assert_eq!(
        bkce_weigh(BKCE_SPEND_WRITER, false, Some(true)),
        (BKCE_ELECTED_PINNED, true),
        "a writer is pinned to the blessed residence whatever the seat's own build says"
    );
}

/// A DISPATCHED SEAT TAKES THE RESIDENCE JOIN TWICE, over two roots, and an
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
        &bkca_Geography::Dispatched(delivered.bktu_root().to_path_buf()),
        &collar,
    )
    .expect("the election answers");

    assert_eq!(
        dispatched.residence,
        bkce_residence(delivered.bktu_root(), &collar),
        "the borrow candidate joins onto the delivered root"
    );
    assert_eq!(
        dispatched.own,
        bkce_residence(lure.bktu_root(), &collar),
        "the own residence joins onto the work tree"
    );
    assert_ne!(
        dispatched.residence, dispatched.own,
        "under dispatch the two are different files, which is the whole reason there are two"
    );

    // THE CONTROL IS THE SAME COLLAR UNDER SELF-GEOGRAPHY, where the two roots
    // are one tree: without it, a verdict that always carried two paths would
    // clear the assertions above whether or not the geography decided anything.
    let self_read = bkce_elect(lure.bktu_root(), &bkca_Geography::Undispatched, &collar)
        .expect("the election answers");

    assert_eq!(
        self_read.residence, self_read.own,
        "the control: an undispatched seat has one residence under two names"
    );
}

/// A DISPATCHED SEAT BORROWS THE DELIVERED BINARY WHEN THE TREES MATCH, and the
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
        &bkca_Geography::Dispatched(delivered.bktu_root().to_path_buf()),
        &zbkte_app_collar("src"),
    )
    .expect("the election answers");

    assert_eq!(
        verdict.elected, BKCE_ELECTED_BORROW,
        "the delivered binary was struck from the tree this seat holds"
    );
    assert_eq!(
        verdict.answers.as_deref(),
        Some(verdict.residence.as_path()),
        "the delivered binary is the one that runs"
    );
    assert!(
        verdict.own_plaint.is_none(),
        "the own residence is not asked where the delivered one answered: {:?}",
        verdict.own_plaint
    );
}

/// A DISPATCHED SEAT RUNS ITS OWN BUILD WHERE THE DELIVERED ONE IS OUTRUN. This
/// is the arm the pace added, and without it a dispatched seat that had already
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
    // an unreadable stamp, which is the shape a live dispatched seat meets.
    zbkte_plant(&delivered, "answerer", &behind);
    zbkte_plant(&lure, "answerer", &struck);

    let verdict = bkce_elect(
        lure.bktu_root(),
        &bkca_Geography::Dispatched(delivered.bktu_root().to_path_buf()),
        &zbkte_app_collar("src"),
    )
    .expect("the election answers");

    assert_eq!(
        verdict.elected, BKCE_ELECTED_OWN,
        "the delivered binary was struck from a tree this seat has moved past"
    );
    assert!(
        verdict.bkce_runs(),
        "a current own build runs: {:?}",
        verdict.plaint
    );
    assert_eq!(
        verdict.answers.as_deref(),
        Some(verdict.own.as_path()),
        "the binary that runs is the seat's OWN and never the delivered one"
    );

    // THE CONTROL IS THE SAME DISPATCHED SEAT WITH ITS OWN RESIDENCE OUTRUN TOO,
    // which is what says the own reading was CONSULTED rather than the arm being
    // unconditional. Without it, a rule that ran whenever the geography was
    // dispatched would clear every assertion above.
    zbkte_plant(&lure, "answerer", &behind);

    let outrun = bkce_elect(
        lure.bktu_root(),
        &bkca_Geography::Dispatched(delivered.bktu_root().to_path_buf()),
        &zbkte_app_collar("src"),
    )
    .expect("the election answers");

    assert!(
        !outrun.bkce_runs(),
        "the control: the same dispatched seat whose own build is outrun too is refused, so the \
         run above is the own residence's currency and not the geography alone"
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
    // is the shape a freshly dispatched seat stands in before its first converge.
    zbkte_plant(&delivered, "answerer", &behind);

    let verdict = bkce_elect(
        lure.bktu_root(),
        &bkca_Geography::Dispatched(delivered.bktu_root().to_path_buf()),
        &zbkte_app_collar("src"),
    )
    .expect("the election answers");

    assert_eq!(verdict.elected, BKCE_ELECTED_OWN);
    assert!(
        !verdict.bkce_runs(),
        "neither residence holds a binary this seat may run"
    );
    assert!(
        verdict.answers.is_none(),
        "a refused election names no binary to spawn"
    );

    let said = verdict.bkce_grievance("app-lure", "heel");

    assert!(
        said.contains(&verdict.residence.display().to_string()),
        "the refusal names the delivered binary it could not borrow: {}",
        said
    );
    assert!(
        said.contains(&verdict.own.display().to_string()),
        "the refusal names the seat's own residence, which is the tree the converge writes: {}",
        said
    );
    assert!(
        said.contains("heel"),
        "the refusal names the converge rather than performing one: {}",
        said
    );

    // THE CONTROL IS AN UNDISPATCHED REFUSAL OVER THE SAME SEAT, which must NOT
    // carry a second sentence: a grievance that always spoke of two residences
    // would clear the assertions above without the second reading ever happening.
    let self_read = bkce_elect(
        lure.bktu_root(),
        &bkca_Geography::Undispatched,
        &zbkte_app_collar("src"),
    )
    .expect("the election answers");

    assert!(
        self_read.own_plaint.is_none(),
        "the control: an undispatched seat put no second question, so its refusal says nothing \
         about a second residence: {:?}",
        self_read.own_plaint
    );
}

// eof
