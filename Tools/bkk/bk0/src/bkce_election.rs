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

//! The binary election: which binary answers for one collar
//! (BKSNC-Kennelcraft.adoc "The Binary Election").
//!
//! DERIVED FROM FACTS THE SEAT ALREADY OWNS, never configured and never probed.
//! No field of any regime names a binary choice, and none may: a choice a caller
//! could spell would make the most consequential decision of a launch its least
//! visible one. What decides is the collar's spend class and two positions.
//!
//! THE WEIGHING AND THE READING ARE SPLIT, and the split is what makes the
//! election provable. `bkce_weigh` is a pure function of four facts and holds
//! the whole rule; everything below it goes and gets those facts from a
//! repository and a standing binary. A hurdle can therefore compose a stamp and
//! a walk that no tree carries and drive every arm of the rule over them, which
//! is the only way the borrow arm can be exercised at all — a live seat can
//! stand in one arm at a time and never in the arm it is not in.
//!
//! THREE ARMS, AND ONLY ONE OF THEM ASKS ABOUT POSITION.
//!
//!   PINNED. A collar whose spend class names a writer is pinned to the blessed
//!   residence whatever any position says, because a wrong vintage writing shared
//!   durable state writes corruption where a borrowing reader's wrong vintage
//!   writes a wrong report, caught by rerun. The position is not consulted — not
//!   consulted and found agreeable, but never asked.
//!
//!   BORROW. A reader whose seat holds the tree the standing binary was struck
//!   from borrows it. WHAT IS COMPARED IS CONTENT AND NOT POSITION: the binary
//!   states the position it was struck at, and the election asks whether the
//!   tree under the collar's elected roots is identical between that commit and
//!   this seat's HEAD. Identical trees make identical binaries, so borrowing is
//!   provably equivalent rather than a shortcut.
//!
//!   OWN BUILD. Anything else — the trees part under the elected roots, the
//!   binary cannot say where it was struck, it names a position this repository
//!   cannot resolve, or git refused the comparison. The door REPORTS AND REFUSES
//!   here and never converges (BKSCL-Collar.adoc "Door Law"); the refusal names
//!   the act, and carries the plaint saying which of those it met.
//!
//! CONTENT IS COMPARED BECAUSE NEITHER POSITION TEST IS THE QUESTION, and the
//! two fail in opposite directions.
//!
//!   A WALK MEMBERSHIP TEST BORROWS TOO MUCH. Reading the seat's position against
//!   the whole walk behind a binary's stamp borrows whenever the seat stands at
//!   ANY landing that binary carries — and every entry behind the walk's head is
//!   a landing that CHANGED the elected roots, so a seat standing at one holds a
//!   tree the binary was never struck from. That reading called itself provably
//!   equivalent and was not.
//!
//!   AN EXACT STAMP MATCH BORROWS TOO LITTLE, and it fails the case this estate
//!   makes constantly. A refit is a merge, and a merge commit stands on no trunk
//!   line, so a refitted billet could never borrow again however identical its
//!   tree — a rebuild demanded for a merge that moved nothing under the roots.
//!
//! Comparing the trees answers all three cases with one question: it survives a
//! refit, a merge that moved nothing under the roots leaving the trees identical;
//! it refuses a seat forked behind a trunk that moved them; and it refuses a
//! local edit, whoever made it and whatever line it stands on.
//!
//! THE WALK IS THE ENGINE FENCE'S INSTRUMENT AND NOT THIS ONE. That fence reads
//! for a consumer holding a RECORD OF LANDINGS and not the repository, so a walk
//! is the only thing it can resolve a position against. The kennel holds the
//! repository itself, so it reads the stamp alone and then asks git the question
//! the walk was standing in for.
//!
//! AN UNREADABLE BINARY ELECTS ITS OWN BUILD, which is the fail-closed
//! direction: an artifact that cannot state its position may not be presumed
//! current, and the cost of being wrong is a rebuild rather than a run against a
//! vintage nobody can name.
//!
//! GEOGRAPHY IS HANDED IN AND DECIDES HOW MANY RESIDENCES THERE ARE. An
//! undispatched seat at an ordinary repository has ONE: the work tree is the
//! tree it stands in and the delivered root is its own build output, so the
//! residence joins onto the repository and the two questions below collapse into
//! the one this module has always asked.
//!
//! A DISPATCHED SEAT HAS TWO, and the election reads them in order
//! (BKSCL-Collar.adoc "The Dispatch Layer and the Whereabouts"). The BORROW
//! CANDIDATE joins onto the delivered root the whereabouts named — the standing
//! clone of the work billet's own sire, whose binaries are what a seat borrows
//! rather than rebuilding. The OWN RESIDENCE joins onto the work tree by the
//! same join, and is what the seat's own converge writes. Where the borrow
//! candidate is not equivalent the own residence is asked the same content
//! question, and a CURRENT OWN BUILD RUNS: a seat that has already built its own
//! work is not sent to build it again, and only a seat holding neither a
//! borrowable delivered binary nor a current own one is refused.
//!
//! NOTHING HERE READS AN ENVIRONMENT. The geography arrives as a value the door
//! face read once and threaded, the same way a repository does
//! (`bkca_whereabouts`), which is what lets a hurdle pose a dispatched seat by
//! composing two directories instead of mutating the process every other hurdle
//! is also running in.

use crate::bkca_whereabouts::bkca_Geography;
use crate::bkcr_resolve::bkcr_Collar;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The spend class that pins. A writer never enters the position question.
pub const BKCE_SPEND_WRITER: &str = "bknre_writer";

/// The verdicts, in the substrate's enum sprue form under this kit's own plane,
/// so one grep on the plane returns the whole cluster.
///
/// They are the values the proclamation's election field carries, which is why
/// they are declared rather than rendered as prose at each site: the tattoo door
/// answers with one of these and a consumer sources the answer.
pub const BKCE_ELECTED_PINNED: &str = "bknre_pinned";
pub const BKCE_ELECTED_BORROW: &str = "bknre_borrow";
pub const BKCE_ELECTED_OWN: &str = "bknre_own";

/// The word a binary answers its own exergue on.
///
/// ASKED OF THE ARTIFACT, never inferred from a timestamp or read out of the
/// source tree. An mtime comparison answers a question about a file; the source
/// tree answers a question about the seat. Only the binary can say what IT was
/// struck from, which is the one question the election asks it
/// (BKSNC-Kennelcraft.adoc "Currency by git position").
///
/// A BINARY THAT DOES NOT ANSWER IT CANNOT BE BORROWED, and that is the whole
/// contract — there is no second channel and no fallback. The word is the one
/// the estate's own reader already spells, so an app that carries an exergue at
/// all already answers here.
pub const BKCE_EXERGUE_FLAG: &str = "--exergue";

/// Which binary answers, and the two positions that decided it.
///
/// The positions ride the verdict rather than being recomputed by whatever
/// renders it: a refusal that named a position it had gone and read a second
/// time could name a different one than the election ruled on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkce_Verdict {
    /// One of the three declared values above.
    pub elected: &'static str,
    /// Where the blessed binary stands — the borrow candidate, joined onto the
    /// delivered root. Under self-geography this is the seat's own residence,
    /// the two roots being one tree.
    pub residence: PathBuf,
    /// Where the seat's own converge writes, joined onto the work tree. Equal to
    /// `residence` under self-geography, and DELIBERATELY still carried there:
    /// a refusal naming the converge names a path, and a field that stood empty
    /// in the ordinary case would make the ordinary refusal the exceptional one
    /// to compose.
    pub own: PathBuf,
    /// The binary that answers, where one may. `None` is the refusal: no
    /// borrowable delivered binary and no current own build, which is what
    /// `bkce_runs` reads.
    ///
    /// IT IS A PATH AND NOT A FLAG, because under dispatch the two residences
    /// are different files and the caller must spawn the one the election ruled
    /// on. A caller told only THAT it may run would have to re-derive WHICH,
    /// and would re-derive it from the same geography by the same join — a
    /// second implementation of the one decision this module exists to hold.
    pub answers: Option<PathBuf>,
    /// The seat's newest first-parent commit over the collar's elected roots.
    /// `None` where the spend class pinned the collar and no position was asked
    /// for — an absence that says the question was never put, not that it had no
    /// answer.
    pub seat: Option<String>,
    /// The position the standing binary says it was struck from, or `None` where
    /// it could not say.
    pub standing: Option<String>,
    /// What stands between the two trees, where the election found something.
    /// `None` where the spend class pinned and no comparison was made, and where
    /// the trees agreed and there is nothing to say.
    pub plaint: Option<String>,
    /// What the own residence had to say, where it was asked. `None` under
    /// self-geography, where there is no second residence to ask — an absence
    /// that says the question was never put.
    pub own_plaint: Option<String>,
}

impl bkce_Verdict {
    /// Whether a binary answers, and may be spawned.
    pub fn bkce_runs(&self) -> bool {
        self.answers.is_some()
    }

    /// The refusal an outrun election owes: what stands, why it may not answer,
    /// and the act that would remedy it.
    ///
    /// THE CONVERGE IS NAMED BY THE CALLER, because the act that cures a stale
    /// binary is a fact about THAT binary and is known to the door rather than to
    /// the election — the same parameterization the engine's own fence takes for
    /// the same reason.
    pub fn bkce_grievance(&self, collar: &str, converge: &str) -> String {
        let standing = match &self.standing {
            Some(position) => format!("the binary at {} was struck at {}", self.residence.display(), position),
            None => format!("the binary at {} states no position", self.residence.display()),
        };

        let seat = match &self.seat {
            Some(position) => format!("this seat stands at {} over the collar's elected roots", position),
            None => "this seat's position could not be read".to_string(),
        };

        // THE PLAINT IS WHAT A READER CAN ACT ON. The two positions say what was
        // compared; only the plaint says what came back, and a refusal carrying
        // the positions alone sends its reader to run the diff by hand.
        let plaint = match &self.plaint {
            Some(plaint) => plaint.as_str(),
            None => "the trees under the collar's elected roots could not be compared",
        };

        // THE OWN RESIDENCE IS NAMED ONLY WHERE IT WAS ASKED. Under
        // self-geography there is no second residence and no second reading, so
        // a sentence about one would be a sentence about a question nobody put;
        // under dispatch the reader has just been refused by TWO binaries and a
        // refusal naming one of them sends them to converge the wrong tree.
        let second = match &self.own_plaint {
            Some(own_plaint) => format!(
                ", and the seat's own binary at {} may not answer either — {}",
                self.own.display(),
                own_plaint
            ),
            None => String::new(),
        };

        format!(
            "the collar '{}' elects its own build: {}, and {} — {}{}. A door reports and refuses \
             and never converges on its own — the converge, run deliberately: {}",
            collar, standing, seat, plaint, second, converge
        )
    }
}

/// The whole rule, over facts alone.
///
/// `identical` is the concord reading's verdict: whether the tree under the
/// collar's elected roots stands the same at the binary's stamp and at this
/// seat's HEAD.
///
/// FALSE CARRIES EVERY FAILURE WITH IT — an absent binary, an answer holding no
/// position, a stamp this repository cannot resolve, a git that refused. Each of
/// those leaves the artifact's tree unestablished, and an artifact whose tree
/// cannot be SHOWN to match may not be borrowed; a rule that told them apart
/// would be branching on facts that change its answer not at all. What does tell
/// them apart is the plaint, carried for the refusal to read and never for the
/// rule to weigh.
///
/// `own` is the same reading taken against the SEAT'S OWN residence, and it is
/// an option because under self-geography there is no second residence to take
/// it against: `None` says the question was never put, which is a different fact
/// from `Some(false)` — a seat that has no own residence and a seat whose own
/// build is outrun are refused alike, but only the second has a converge that
/// would help.
///
/// TWO THINGS COME BACK BECAUSE THE RULE DECIDES TWO THINGS, and splitting them
/// across two functions would let a caller pair a word with a posture the rule
/// never put together. The word is what the proclamation carries; the flag is
/// whether a binary answers at all. They part in exactly one arm — a dispatched
/// seat whose own build is current elects its own build AND runs it — and that
/// arm is the whole of what this pace added.
pub fn bkce_weigh(spend: &str, identical: bool, own: Option<bool>) -> (&'static str, bool) {
    if spend == BKCE_SPEND_WRITER {
        return (BKCE_ELECTED_PINNED, true);
    }

    if identical {
        return (BKCE_ELECTED_BORROW, true);
    }

    (BKCE_ELECTED_OWN, own == Some(true))
}

/// What the content reading found: whether the two trees stand in concord, and
/// where they do not, the plaint a refusal carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkce_Concord {
    /// Whether the tree under the collar's elected roots is the same at both
    /// positions.
    pub identical: bool,
    /// What stands between the two trees, or why the question could not be put.
    /// Empty exactly where `identical` is true.
    pub plaint: String,
}

/// Whether the tree under the collar's elected roots stands the same at the
/// binary's stamp and at this seat's `HEAD`.
///
/// TWO GIT QUESTIONS AND NO THIRD. First, whether the stamp resolves to a commit
/// HERE — a binary struck at a seat this repository never held names a position
/// that is no commit in it, and a comparison against a name git cannot look up is
/// not a comparison. Second, whether the tree differs under the elected roots.
/// The roots reach git VERBATIM, exclusion pathspecs and all, the election's
/// grammar being git's own — so the one pathspec decides both what stales an
/// artifact and what is compared, and the two can never come apart.
///
/// `HEAD` IS COMPARED AND NOT THE WORKING TREE, which is exact rather than
/// approximate: the door law refuses a repository carrying an uncommitted or
/// untracked file before any door reaches this reading (BKSCL-Collar.adoc "Door
/// Law"), so at the moment this runs the working tree IS `HEAD`. A reading that
/// went to the working tree instead would be answering a question the door has
/// already refused to let anyone ask.
///
/// THE JUDGMENT IS `--quiet`'S AND THE NAMING IS A SECOND READING. The verdict
/// wants one bit and `--quiet` stops at the first differing path to give it; the
/// listing is taken only where the trees part, only to compose the plaint, and
/// decides nothing.
pub fn bkce_concord(repository: &Path, stamp: Option<&str>, roots: &str) -> bkce_Concord {
    let Some(stamp) = stamp else {
        return zbkce_apart(
            "the binary would not say what it was struck from, so the tree it was made from is \
             unestablished and may not be presumed to be this one",
        );
    };

    let elected: Vec<&str> = roots.split_whitespace().collect();

    if elected.is_empty() {
        return zbkce_apart(
            "the collar's elected roots stand empty, and a comparison over nothing would call \
             every binary current",
        );
    }

    match Command::new("git")
        .arg("-C")
        .arg(repository)
        .arg("cat-file")
        .arg("-e")
        .arg(format!("{}^{{commit}}", stamp))
        .output()
    {
        Err(err) => {
            return zbkce_apart(&format!("git would not run in {}: {}", repository.display(), err))
        }
        Ok(out) if !out.status.success() => {
            return zbkce_apart(&format!(
                "the position {} names no commit in this repository, so the binary was struck from \
                 a tree this seat cannot look up",
                stamp
            ))
        }
        Ok(_) => {}
    }

    let parted = Command::new("git")
        .arg("-C")
        .arg(repository)
        .arg("diff")
        .arg("--quiet")
        .arg(stamp)
        .arg("HEAD")
        .arg("--")
        .args(&elected)
        .output();

    match parted {
        Err(err) => zbkce_apart(&format!("git would not run in {}: {}", repository.display(), err)),
        Ok(out) if out.status.success() => bkce_Concord {
            identical: true,
            plaint: String::new(),
        },
        Ok(out) if out.status.code() == Some(1) => zbkce_apart(&format!(
            "the trees differ under the collar's elected roots: {}",
            zbkce_parted(repository, stamp, &elected)
        )),
        Ok(out) => zbkce_apart(&format!(
            "git would not compare {} against HEAD over the collar's elected roots: {}",
            stamp,
            String::from_utf8_lossy(&out.stderr).trim()
        )),
    }
}

/// A concord that does not stand, carrying its plaint.
fn zbkce_apart(plaint: &str) -> bkce_Concord {
    bkce_Concord {
        identical: false,
        plaint: plaint.to_string(),
    }
}

/// The paths that part the two trees, for the plaint alone.
///
/// A LISTING AND NEVER A JUDGMENT. The verdict is already taken when this runs,
/// so a git that refuses here costs the refusal its detail and never its
/// correctness — which is why it answers with a phrase rather than a failure.
///
/// BOUNDED, because a seat far behind trunk parts over hundreds of paths and a
/// refusal nobody can read to the end is a refusal nobody acts on
/// (BKSNC-Kennelcraft.adoc "Bounded output"). The count stands beside the
/// named few, so a reader is told the size of what they are seeing a corner of.
fn zbkce_parted(repository: &Path, stamp: &str, elected: &[&str]) -> String {
    const ZBKCE_NAMED: usize = 5;

    let Ok(out) = Command::new("git")
        .arg("-C")
        .arg(repository)
        .arg("diff")
        .arg("--name-only")
        .arg(stamp)
        .arg("HEAD")
        .arg("--")
        .args(elected)
        .output()
    else {
        return "git would not name them".to_string();
    };

    let named: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    if named.is_empty() {
        return "git would not name them".to_string();
    }

    if named.len() > ZBKCE_NAMED {
        return format!(
            "{}, and {} more",
            named[..ZBKCE_NAMED].join(", "),
            named.len() - ZBKCE_NAMED
        );
    }

    named.join(", ")
}

/// Hold the election for one collar against the tree it stands in and the
/// geography it was dispatched under.
///
/// THE TWO RESIDENCES ARE ONE JOIN TAKEN TWICE, over the delivered root and over
/// the work tree, which is what keeps a dispatched seat's coordinates the same
/// coordinates an undispatched one has. Under self-geography the two roots are
/// the same tree and the join answers the same path, so the second reading is
/// skipped rather than taken and thrown away — an election that asked one
/// binary twice would suggest the two answers could differ.
pub fn bkce_elect(
    repository: &Path,
    geography: &bkca_Geography,
    collar: &bkcr_Collar,
) -> Result<bkce_Verdict, String> {
    let own = bkce_residence(repository, collar);
    let residence = match geography.bkca_delivered() {
        Some(delivered) => bkce_residence(delivered, collar),
        None => own.clone(),
    };
    let spend = collar.bkcr_field("BKRR_SPEND");

    // THE PIN IS ANSWERED WITHOUT ASKING A POSITION, and the ordering is the
    // rule rather than an economy: a writer is pinned whatever any position
    // says, so a reading taken here and then discarded would suggest the answer
    // depended on it.
    //
    // A PINNED COLLAR IS PINNED TO THE BLESSED RESIDENCE AND NOT TO THE SEAT'S
    // OWN, which is the whole of what pinning means under dispatch: a wrong
    // vintage writing shared durable state writes corruption, and a billet-built
    // engine is exactly the wrong vintage the pin exists to keep away from the
    // live store.
    if spend == BKCE_SPEND_WRITER {
        return Ok(bkce_Verdict {
            elected: BKCE_ELECTED_PINNED,
            answers: Some(residence.clone()),
            residence,
            own,
            seat: None,
            standing: None,
            plaint: None,
            own_plaint: None,
        });
    }

    let roots = collar.bkcr_field("BKRR_ROOTS");
    let seat = bkce_seat(repository, roots)?;

    // THE SEAT POSITION IS READ THOUGH THE RULE NO LONGER WEIGHS IT, and that is
    // the refusal's doing rather than the election's: a reader told only that the
    // trees part cannot tell a seat that has moved from a binary that is old, and
    // the two positions are what places the grievance in time. The reading also
    // refuses a collar electing nothing, which is the one malformation that would
    // otherwise reach the comparison and be answered quietly.
    let standing = bkce_carried(&residence).first().cloned();
    let concord = bkce_concord(repository, standing.as_deref(), roots);

    // THE OWN RESIDENCE IS ASKED ONLY WHERE THE BORROW CANDIDATE FAILED, and only
    // under dispatch. Asking it first would spawn a binary whose answer changes
    // nothing whenever the delivered one is borrowable, and asking it under
    // self-geography would be asking the same file the same question twice.
    let second = match (geography.bkca_delivered(), concord.identical) {
        (Some(_), false) => {
            let carried = bkce_carried(&own).first().cloned();
            Some(bkce_concord(repository, carried.as_deref(), roots))
        }
        _ => None,
    };

    let (elected, runs) = bkce_weigh(
        spend,
        concord.identical,
        second.as_ref().map(|concord| concord.identical),
    );

    // WHICH BINARY ANSWERS FOLLOWS FROM WHICH READING CARRIED IT, and never from
    // the word alone: the borrow arm answers with the delivered binary and the
    // own arm with the seat's, and both wear the same word in the one case they
    // do not — which is why the path is carried rather than derived by a caller
    // reading `elected`.
    let answers = runs.then(|| match elected {
        BKCE_ELECTED_OWN => own.clone(),
        _ => residence.clone(),
    });

    Ok(bkce_Verdict {
        elected,
        answers,
        residence,
        own,
        standing,
        seat: Some(seat),
        plaint: (!concord.identical).then_some(concord.plaint),
        own_plaint: second.filter(|concord| !concord.identical).map(|concord| concord.plaint),
    })
}

/// The elected value alone, for any collar of either kind — what the
/// proclamation's election field carries.
///
/// A SUITE ALWAYS ELECTS ITS OWN BUILD, and that is the true answer rather than
/// a slot filled to keep the shape. The runner compiles a suite from the source
/// the seat holds, so a billet's suite tests the billet's code and no suite is
/// ever borrowed; there is no standing artifact between the source and the
/// verdict for a vintage question to be asked about. It reaches the same word the
/// app arm reaches when a seat has outrun its binary, because it is the same
/// fact — what runs is built from what this seat holds — and minting a second
/// word for it would say the two were different.
///
/// The full verdict is the app door's, which needs the positions the refusal
/// names; a door that only has to SAY which way the election went takes this.
pub fn bkce_elected(
    repository: &Path,
    geography: &bkca_Geography,
    collar: &bkcr_Collar,
) -> Result<&'static str, String> {
    if !collar.bkcr_app() {
        return Ok(BKCE_ELECTED_OWN);
    }

    Ok(bkce_elect(repository, geography, collar)?.elected)
}

/// A residence: the residence directory joined with the name the binary wears
/// there, under whichever root it is taken over.
///
/// THE ROOT IS A PARAMETER AND NOT ALWAYS THE REPOSITORY, which is what makes a
/// dispatched seat's two residences one join taken twice — over the delivered
/// root for the borrow candidate, over the work tree for the seat's own. A
/// second function for the second root would be two places for the collar's
/// coordinates to be read.
///
/// THE BYNAME IS NOT THE TARGET, and the two part whenever a binary is renamed on
/// its way to a residence. The target is cargo's name for what is built; the
/// byname is what the file is called once it stands. The engine's own artifact is
/// the standing instance — cargo builds `vvr` and the install and the parcel
/// emplacement alike put it down as `vvx` — so a join over the target named a
/// file that does not exist and the door met an absence rather than a refusal it
/// could explain.
///
/// The residence is a DIRECTORY holding one binary already chosen, so the byname
/// is what names the file inside it. A tree served by more than one platform
/// holds platform-keyed siblings in that one home, and the key would join here
/// as opaque text; no such tree stands at MVP.
pub fn bkce_residence(root: &Path, collar: &bkcr_Collar) -> PathBuf {
    root.join(collar.bkcr_field("BKRR_RESIDENCE"))
        .join(collar.bkcr_field("BKRR_BYNAME"))
}

/// The seat's own position: its newest first-parent commit touching an elected
/// root.
///
/// WALKED OVER THE ELECTION, never bare `HEAD`. What stales an artifact is a
/// change to what it is made from, so a bare-`HEAD` reading would report a new
/// position for every commit anywhere in the repository and call an artifact that
/// did not change outrun.
///
/// THE SEAT'S OWN LINE, never the trunk counterpart. A commit made at a billet
/// never enters the landed walk `Tools/buk/bue_exergue.sh` takes, so a seat
/// judging itself by landings would sit still while its own source moved.
///
/// The election's grammar is git's own — a directory reaches everything beneath
/// it and an exclusion pathspec carves back out — so nothing here interprets an
/// element.
pub fn bkce_seat(repository: &Path, roots: &str) -> Result<String, String> {
    let elected: Vec<&str> = roots.split_whitespace().collect();

    if elected.is_empty() {
        return Err(
            "the collar's elected roots stand empty, and a position measured over nothing would \
             call every binary current"
                .to_string(),
        );
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
        .args(&elected)
        .output()
        .map_err(|err| format!("could not run git in {}: {}", repository.display(), err))?;

    if !out.status.success() {
        return Err(format!(
            "git would not walk this seat's line over the collar's elected roots in {}: {}",
            repository.display(),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }

    let position = String::from_utf8_lossy(&out.stdout).trim().to_string();

    if position.is_empty() {
        return Err(
            "no commit on this seat's line touches any elected root, so the collar's election \
             names nothing this tree has ever carried"
                .to_string(),
        );
    }

    Ok(position)
}

/// What a standing binary says it was struck from: its stamp and the walk behind
/// it, newest first, the stamp its own first element.
///
/// ONLY THE FIRST LINE IS THE STAMP, AND ONLY IT IS COMPARED. The whole sequence
/// is still taken because it is the shape the exergue writes and the shape the
/// engine's own fence consumes, and a reading that dropped the tail would make
/// this the one consumer that reads a different answer than the artifact gives.
/// The election reads the head and asks git the rest.
///
/// EMPTY WHERE THE BINARY CANNOT SAY, and every failure reads that way — no
/// binary stands, it would not run, it exited nonzero, it answered nothing. The
/// collapse is deliberate: each of those means the same thing to the election,
/// which is that this artifact's position is unestablished, and a caller that
/// had to tell them apart would be branching on facts that change nothing.
///
/// THE ANSWER IS THE BINARY'S STDOUT, one position per line. A line carrying
/// anything but a position is dropped rather than refused, because the binaries
/// this asks are not the kennel's to shape: what is read is the answer's
/// position-shaped lines, and a binary with more to say says it without
/// confusing the reading.
pub fn bkce_carried(binary: &Path) -> Vec<String> {
    let Ok(out) = Command::new(binary).arg(BKCE_EXERGUE_FLAG).output() else {
        return Vec::new();
    };

    if !out.status.success() {
        return Vec::new();
    }

    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| zbkce_position_shaped(line))
        .collect()
}

/// Whether a line is shaped like a git object name: forty lowercase hex
/// characters and nothing else.
///
/// A SHAPE TEST AND NEVER A LOOKUP. The election compares positions for equality
/// and never resolves one, so what this must exclude is a line that could be
/// mistaken for a position — a banner, a version, a warning — rather than a
/// position that names no object.
fn zbkce_position_shaped(line: &str) -> bool {
    line.len() == 40 && line.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}

// eof
