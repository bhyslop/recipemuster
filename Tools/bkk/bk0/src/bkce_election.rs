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

//! The binary election: which binary answers for one collar.
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
//!   cannot resolve, git refused the comparison, or under dispatch the borrow
//!   candidate's fence reading refuses it or the seat's own build is current and
//!   answers ahead of it. Where no current own build stands the door REPORTS AND
//!   REFUSES and never converges; the refusal names the act, and carries the
//!   plaint saying which of those it met.
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
//! GEOGRAPHY IS DERIVED AND DECIDES HOW MANY RESIDENCES THERE ARE. A main
//! checkout or a plain clone has ONE: the work tree is the tree the door
//! entered and the delivered root is its own build output, so the residence
//! joins onto the repository and the two questions below collapse into the one
//! this module has always asked.
//!
//! A LINKED WORKTREE HAS TWO, and the election reads them both. The
//! BORROW CANDIDATE joins onto the delivered root — the worktree's main worktree, which is the
//! standing clone every work billet is added from, and whose binaries are what
//! a seat borrows rather than rebuilding. The OWN RESIDENCE joins onto the work
//! tree by the same join, and is what the seat's own converge writes. The own
//! residence is asked the same content question, and a CURRENT OWN BUILD RUNS,
//! ahead of any borrow: a seat that has already built its own work runs it, and
//! only a seat holding neither a borrowable delivered binary nor a current own
//! one is refused.
//!
//! A BORROW IS ALSO ASKED THE FENCE'S QUESTION, because an identical tree is not
//! the only thing a borrowed binary must answer to. A binary carrying a staleness
//! fence of its own judges the LANDED reading it was struck with — the newest
//! trunk landing over its roots — against the record of landings, so a delivered
//! binary struck before a sibling's landing is one its own fence refuses however
//! identical the tree this seat forked at. The kennel reads that question through
//! its own instrument: the exergue files the collar declares, standing at the
//! delivered root where that binary's build struck them, asked whether the trunk
//! counterpart has moved beneath them. The binary itself is never asked, because
//! no contract obliges a binary to answer a fence question, and a binary given a
//! question it does not know answers with whatever its door does with a stranger.
//!
//! NOTHING HERE READS AN ENVIRONMENT. The geography is git's own answer from
//! inside the repository the door entered (`bkce_geography`), taken once at the
//! door face and threaded as a value the same way a repository is, which is
//! what lets a hurdle pose a linked worktree by standing a real linked worktree
//! rather than by mutating the process every other hurdle is also running in.

use crate::bkcr_resolve::bkcr_Collar;
use std::path::{
    Path,
    PathBuf, //
};
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
/// struck from, which is the one question the election asks it.
///
/// A BINARY THAT DOES NOT ANSWER IT CANNOT BE BORROWED, and that is the whole
/// contract — there is no second channel and no fallback. The word is the one
/// the estate's own reader already spells, so an app that carries an exergue at
/// all already answers here.
pub const BKCE_EXERGUE_FLAG: &str = "--exergue";

/// The far side of the election's concord: this seat's `HEAD`.
///
/// NAMED BECAUSE IT IS NOT THE ONLY FAR SIDE. The election asks whether a
/// binary's tree matches the seat's, so its far side is `HEAD`. A struck
/// exergue FILE carries the LANDED reading — the newest trunk landing over its
/// roots — and is asked whether the trunk has moved beneath them, so its far
/// side is the trunk counterpart (`bkce_trunk`). A far side spelled at every
/// call is what keeps the two questions apart on the page.
pub const BKCE_SEAT_FAR: &str = "HEAD";

/// The remote whose recorded head names the trunk counterpart.
///
/// A SECOND STATEMENT OF A FACT THE STRIKER HOLDS, and named as one.
/// `Tools/buk/bue_exergue.sh` takes the remote from its caller and resolves
/// `refs/remotes/<remote>/HEAD`; the kennel, judging a file that striker wrote,
/// must resolve the same ref and has no caller to take the name from, so it
/// states the word here.
pub const BKCE_REMOTE: &str = "origin";

/// The proclamation's word for each geography, in the substrate's enum sprue
/// form under this kit's own plane.
///
/// They are declared rather than spelled at each site because a consumer sources
/// the proclamation and matches on the value: a door and a hurdle spelling the
/// same word twice are two things to keep in step.
pub const BKCE_GEOGRAPHY_SELF: &str = "bknre_self";
pub const BKCE_GEOGRAPHY_DISPATCHED: &str = "bknre_dispatched";

/// The name git gives a main worktree's own git directory, which is how a common
/// git directory is known to stand inside a work tree rather than to be a bare
/// repository.
const ZBKCE_GIT_DIR: &str = ".git";

/// Where a seat stands relative to the trees it works.
///
/// GEOGRAPHY IS NEVER ELECTION. No value here names a binary choice — the
/// election stays computed per invocation over the two residences this yields,
/// so a delivered root that is not there fails loudly as a place that is not
/// there and never lies about which binary answers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum bkce_Geography {
    /// A main worktree or a plain clone. The tree the door entered is the whole
    /// geography: one residence, which is its own.
    Undispatched,
    /// A linked worktree, naming its main worktree as the delivered root whose
    /// built binaries are the borrow candidates.
    Dispatched(PathBuf),
}

impl bkce_Geography {
    /// The delivered root, where the repository has one apart from itself.
    pub fn bkce_delivered(&self) -> Option<&Path> {
        match self {
            bkce_Geography::Undispatched => None,
            bkce_Geography::Dispatched(delivered) => Some(delivered.as_path()),
        }
    }

    /// The geography as a proclamation field and as the election line's own
    /// wording — self, or dispatched with the delivered root.
    ///
    /// ONE RENDERING FOR BOTH SEATS. Every verdict says which geography it read,
    /// because the same binary in the same tree is reached by more than one road
    /// and only the transcript can say which answered; two renderings of that
    /// sentence would be two things that could come to disagree about the
    /// reading they were both describing.
    ///
    /// A CATENA WHERE IT CARRIES A ROOT, so the proclamation stays one field
    /// rather than a token and a path that a consumer must join back together:
    /// the substrate's own reader splits it on whitespace, the first element
    /// being the word and the second the root.
    pub fn bkce_stated(&self) -> String {
        match self {
            bkce_Geography::Undispatched => BKCE_GEOGRAPHY_SELF.to_string(),
            bkce_Geography::Dispatched(delivered) => {
                format!(
                    "{} {}",
                    BKCE_GEOGRAPHY_DISPATCHED,
                    delivered.display(), //
                )
            }
        }
    }
}

/// Derive the geography of the repository a door entered.
///
/// ONE GIT QUESTION, ASKED FROM INSIDE THE TREE: where the git directory and the
/// common git directory stand. A main worktree or a plain clone answers the same
/// directory twice, which is self-geography. A linked worktree answers its own
/// administrative directory and, as the common one, the main worktree's `.git`,
/// whose parent is the delivered root — the estate stands every work billet up
/// by a worktree add inside its sire's standing clone, so the tree a billet
/// borrows from and the tree it was added from are one fact by construction.
///
/// NOTHING STATES THIS A SECOND TIME. No file, no variable and no walk of the
/// filesystem is consulted; a second statement of one fact is a place for the
/// two to disagree, and the one that would be wrong is the one nobody re-reads.
///
/// AN ANSWER THAT NAMES NO STANDING MAIN WORKTREE REFUSES, naming what git
/// answered, and never falls back to self: a fallback would turn a worktree
/// whose main worktree is gone — or one added from a bare repository, which has
/// no work tree to hold a built binary — into a quiet own build, and the borrow
/// candidate would simply never be consulted.
///
/// THE READING IS LAZY AND THE ELECTION'S ALONE. It is taken where a delivered
/// root is CONSUMED and never ahead of the kennel's bare self-report, which the
/// whistle and the driver gates read as a position: `Tools/bkk/bk0/bkcp_position.sh`
/// treats any non-zero exit as no position, so a geography refusal standing ahead
/// of that report would counterfeit staleness at the whistle and at the build
/// module's gate — a fault presenting as a stale binary, which is a different
/// repair aimed at a different tree.
pub fn bkce_geography(repository: &Path) -> Result<bkce_Geography, String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repository)
        .arg("rev-parse")
        .arg("--path-format=absolute")
        .arg("--git-dir")
        .arg("--git-common-dir")
        .output()
        .map_err(|err| {
            format!(
                "could not run git in {}: {}",
                repository.display(),
                err, //
            )
        })?;

    if !out.status.success() {
        return Err(format!(
            "git would not name the git directories of {}, so the repository's geography cannot \
             be derived — a linked worktree whose main worktree is gone answers this way: {}",
            repository.display(),
            String::from_utf8_lossy(&out.stderr).trim(), //
        ));
    }

    let answered = String::from_utf8_lossy(&out.stdout).to_string();
    let mut lines = answered
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty());

    let (Some(own), Some(common), None) = (lines.next(), lines.next(), lines.next()) else {
        return Err(format!(
            "git answered the git directories of {} with something other than two paths: {}",
            repository.display(),
            answered.trim(), //
        ));
    };

    if own == common {
        return Ok(bkce_Geography::Undispatched);
    }

    let common = Path::new(common);

    // THE COMMON DIRECTORY MUST BE A MAIN WORKTREE'S OWN, and that is read off
    // git's answer rather than probed around it: a `.git` standing as a directory
    // names the tree it stands in, and anything else — a bare repository, or a
    // directory that is gone — names no tree whose binaries could be borrowed.
    let delivered = match common.parent() {
        Some(parent) if common.file_name() == Some(std::ffi::OsStr::new(ZBKCE_GIT_DIR)) => parent,
        _ => {
            return Err(format!(
                "{} is a linked worktree whose common git directory is {}, which is no main \
                 checkout's {} — a worktree added from a bare repository has no delivered root, \
                 and one is never invented",
                repository.display(),
                common.display(),
                ZBKCE_GIT_DIR, //
            ));
        }
    };

    if !common.is_dir() || !delivered.is_dir() {
        return Err(format!(
            "{} is a linked worktree whose main worktree at {} does not stand — the delivered \
             root is where a borrow candidate's binary is looked for, so a main worktree that is \
             gone refuses rather than reading as self",
            repository.display(),
            delivered.display(), //
        ));
    }

    Ok(bkce_Geography::Dispatched(delivered.to_path_buf()))
}

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
    /// Why the borrow candidate does not answer, where it does not: what stands
    /// between the two trees, what its fence reading found, or — where the trees
    /// agreed — that the seat's own current build was elected ahead of it. `None`
    /// where the spend class pinned and no comparison was made, and where the
    /// borrow answered and there is nothing to say.
    ///
    /// IT RIDES THE PROCLAMATION AS WELL AS THE REFUSAL, because an own build
    /// elected over an identical borrow is exactly the reading a watcher cannot
    /// infer from the word alone.
    pub plaint: Option<String>,
    /// What the own residence had to say against the seat, where it was asked and
    /// nothing answered. `None` under self-geography, where there is no second
    /// residence to ask, and wherever a binary answers — an outrun own build says
    /// nothing against a borrow that runs.
    pub own_plaint: Option<String>,
    /// Whether the binary the seat's own converge writes STANDS AND STATES NO
    /// POSITION, on a refused election. It is the one refusal the converge cannot
    /// be trusted to cure: a crate that gives the exergue flag no answer rebuilds
    /// into a binary that gives none either, so the remedy is the crate's.
    ///
    /// False wherever a binary answers, where nothing stands at the own residence
    /// (the converge supplies one there), and where the standing binary named a
    /// position that merely parted from the seat's (the converge restamps it).
    pub mute: bool,
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
        // A MUTE BINARY IS NOT HANDED THE CONVERGE AS ITS CURE, because the
        // converge rebuilds the same crate into the same silence and a refusal
        // naming it sends its reader round a loop. The remedy is the crate's,
        // which the plaint names; the converge follows it.
        if self.mute {
            return format!(
                "the collar '{}' elects its own build: {}. The remedy is the crate's, and the \
                 converge follows it: once the crate answers {}, run deliberately: {}",
                collar,
                self.bkce_plaint(),
                BKCE_EXERGUE_FLAG,
                converge, //
            );
        }

        format!(
            "the collar '{}' elects its own build: {}. A door reports and refuses and never \
             converges on its own — the converge, run deliberately: {}",
            collar,
            self.bkce_plaint(),
            converge, //
        )
    }

    /// What stands and why it may not answer: the standing binary's position,
    /// the seat's, what parted the two trees, and under dispatch what the own
    /// residence said.
    ///
    /// ONE WORDING, TWO FRAMES. The launch seam frames it as a refusal naming
    /// the converge (`bkce_grievance`); the derby's preflight frames it as one
    /// precondition among several, naming the collar's own strike doors. Both
    /// say what stood and why it may not answer in these words, so a reader
    /// refused at either door reads one sentence rather than two accounts of
    /// one binary.
    pub fn bkce_plaint(&self) -> String {
        let standing = match &self.standing {
            Some(position) => format!(
                "the binary at {} was struck at {}",
                self.residence.display(),
                position, //
            ),
            None => format!(
                "the binary at {} states no position",
                self.residence.display(), //
            ),
        };

        let seat = match &self.seat {
            Some(position) => format!(
                "this seat stands at {} over the collar's elected roots",
                position, //
            ),
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
                own_plaint, //
            ),
            None => String::new(),
        };

        // THE CONTRACT IS NAMED IN THE PLAINT, so both frames carry it: a reader
        // refused at the launch and one refused at the derby's preflight meet a
        // mute binary in the same words.
        let contract = if self.mute {
            format!(
                "; a reader app owes {} an answer, printing the position the converge states \
                 under {}, and a crate that gives none rebuilds into a binary that gives none \
                 (the kit README, \"Declaring a collar\")",
                BKCE_EXERGUE_FLAG,
                crate::bkch_heel::BKCH_COLLAR_POSITION_VAR, //
            )
        } else {
            String::new()
        };

        format!(
            "{}, and {} — {}{}{}",
            standing,
            seat,
            plaint,
            second,
            contract, //
        )
    }
}

/// The whole rule, over facts alone.
///
/// `borrowable` is whether the borrow candidate may answer: the concord
/// reading's verdict — whether the tree under the collar's elected roots stands
/// the same at the binary's stamp and at this seat's HEAD — and, under dispatch,
/// the fence reading beside it, which asks whether the trunk has moved beneath
/// the exergue that binary was struck with (`bkce_elect`).
///
/// FALSE CARRIES EVERY FAILURE WITH IT — an absent binary, an answer holding no
/// position, a stamp this repository cannot resolve, a git that refused, a
/// binary its own staleness fence would refuse. Each of those leaves the
/// artifact unfit to borrow, and an artifact that cannot be SHOWN fit may not be
/// borrowed; a rule that told them apart would be branching on facts that change
/// its answer not at all. What does tell them apart is the plaint, carried for
/// the refusal to read and never for the rule to weigh.
///
/// `own` is the concord reading taken against the SEAT'S OWN residence, and it is
/// an option because under self-geography there is no second residence to take
/// it against: `None` says the question was never put, which is a different fact
/// from `Some(false)` — a seat that has no own residence and a seat whose own
/// build is outrun are refused alike, but only the second has a converge that
/// would help.
///
/// A CURRENT OWN BUILD ANSWERS AHEAD OF A BORROW. The seat's own binary is built
/// from the very tree the seat holds and was struck against the trunk as this
/// seat last gleaned it, so it is the one binary whose fitness is never a
/// second-hand inference; a borrow is the economy for a seat that holds none,
/// and is taken only there.
///
/// TWO THINGS COME BACK BECAUSE THE RULE DECIDES TWO THINGS, and splitting them
/// across two functions would let a caller pair a word with a posture the rule
/// never put together. The word is what the proclamation carries; the flag is
/// whether a binary answers at all. They part in exactly one arm — a dispatched
/// seat whose own build is current elects its own build AND runs it.
pub fn bkce_weigh(spend: &str, borrowable: bool, own: Option<bool>) -> (&'static str, bool) {
    if spend == BKCE_SPEND_WRITER {
        return (BKCE_ELECTED_PINNED, true);
    }

    if own == Some(true) {
        return (BKCE_ELECTED_OWN, true);
    }

    if borrowable {
        return (BKCE_ELECTED_BORROW, true);
    }

    (BKCE_ELECTED_OWN, false)
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

/// Whether the tree under the elected roots stands the same at the artifact's
/// stamp and at the far side the caller names.
///
/// TWO GIT QUESTIONS AND NO THIRD. First, whether the stamp resolves to a commit
/// HERE — a binary struck at a seat this repository never held names a position
/// that is no commit in it, and a comparison against a name git cannot look up is
/// not a comparison. Second, whether the tree differs under the elected roots.
/// The roots reach git VERBATIM, exclusion pathspecs and all, the election's
/// grammar being git's own — so the one pathspec decides both what stales an
/// artifact and what is compared, and the two can never come apart.
///
/// THE FAR SIDE IS THE CALLER'S, because two artifacts carry two readings and
/// each is asked a question of its own. The election asks whether a binary's
/// tree is the SEAT's, so it passes `BKCE_SEAT_FAR`; the preflight asks whether the TRUNK has moved
/// beneath a struck exergue file, so it passes the counterpart `bkce_trunk`
/// names. One reading serves both because the question is the same shape —
/// are two trees identical under one pathspec — and only which two differs.
///
/// `HEAD` IS COMPARED AND NOT THE WORKING TREE where the far side is the seat,
/// which is exact rather than approximate: the door law refuses a repository
/// carrying an uncommitted or untracked file before any door reaches this
/// reading, so at the moment this runs the working tree IS `HEAD`. A reading that went to the working tree instead
/// would be answering a question the door has already refused to let anyone
/// ask.
///
/// THE JUDGMENT IS `--quiet`'S AND THE NAMING IS A SECOND READING. The verdict
/// wants one bit and `--quiet` stops at the first differing path to give it; the
/// listing is taken only where the trees part, only to compose the plaint, and
/// decides nothing.
pub fn bkce_concord(
    repository: &Path,
    stamp: Option<&str>,
    far: &str,
    roots: &str,
) -> bkce_Concord {
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
        .arg(format!(
            "{}^{{commit}}",
            stamp, //
        ))
        .output()
    {
        Err(err) => {
            return zbkce_apart(&format!(
                "git would not run in {}: {}",
                repository.display(),
                err, //
            ));
        }
        Ok(out) if !out.status.success() => {
            return zbkce_apart(&format!(
                "the position {} names no commit in this repository, so the binary was struck from \
                 a tree this seat cannot look up",
                stamp, //
            ));
        }
        Ok(_) => {}
    }

    let parted = Command::new("git")
        .arg("-C")
        .arg(repository)
        .arg("diff")
        .arg("--quiet")
        .arg(stamp)
        .arg(far)
        .arg("--")
        .args(&elected)
        .output();

    match parted {
        Err(err) => zbkce_apart(&format!(
            "git would not run in {}: {}",
            repository.display(),
            err, //
        )),
        Ok(out) if out.status.success() => bkce_Concord {
            identical: true,
            plaint: String::new(),
        },
        Ok(out) if out.status.code() == Some(1) => zbkce_apart(&format!(
            "the trees differ under the elected roots: {}",
            zbkce_parted(repository, stamp, far, &elected), //
        )),
        Ok(out) => zbkce_apart(&format!(
            "git would not compare {} against {} over the elected roots: {}",
            stamp,
            far,
            String::from_utf8_lossy(&out.stderr).trim(), //
        )),
    }
}

/// The trunk counterpart as this repository last gleaned it: the ref the
/// remote's recorded head names, spelled `<remote>/<branch>`.
///
/// RESOLVED THE WAY THE STRIKER RESOLVES IT, never taken as a branch name.
/// `Tools/buk/bue_exergue.sh` reads `refs/remotes/<remote>/HEAD` so that no
/// caller carries a copy of which branch a repository calls its trunk, and a
/// reader judging what that striker wrote must name the same ref by the same
/// road, or the two will one day disagree about which line the trunk is.
///
/// NO FETCH IS PERFORMED, as the striker performs none: the counterpart is read
/// as it stands, a preflight being no more a place for network than a build.
///
/// A REPOSITORY RECORDING NO HEAD FOR THE REMOTE REFUSES, naming the act that
/// records one, and never falls back to a guessed branch — a guess would judge
/// a file against a line it was never struck from.
pub fn bkce_trunk(repository: &Path) -> Result<String, String> {
    let recorded = Command::new("git")
        .arg("-C")
        .arg(repository)
        .arg("symbolic-ref")
        .arg("--quiet")
        .arg(format!(
            "refs/remotes/{}/HEAD",
            BKCE_REMOTE, //
        ))
        .output()
        .map_err(|err| {
            format!(
                "could not run git in {}: {}",
                repository.display(),
                err, //
            )
        })?;

    if !recorded.status.success() {
        return Err(format!(
            "{} records no head for the remote '{}', so the trunk counterpart cannot be named — \
             run: git -C {} remote set-head {} -a",
            repository.display(),
            BKCE_REMOTE,
            repository.display(),
            BKCE_REMOTE, //
        ));
    }

    let named = String::from_utf8_lossy(&recorded.stdout).trim().to_string();

    let trunk = match named.strip_prefix("refs/remotes/") {
        Some(trunk) if !trunk.is_empty() => trunk.to_string(),
        _ => {
            return Err(format!(
                "{} answers the remote '{}' head with '{}', which names no remote-tracking ref",
                repository.display(),
                BKCE_REMOTE,
                named, //
            ));
        }
    };

    let held = Command::new("git")
        .arg("-C")
        .arg(repository)
        .arg("rev-parse")
        .arg("--verify")
        .arg("--quiet")
        .arg(format!(
            "{}^{{commit}}",
            trunk, //
        ))
        .output()
        .map_err(|err| {
            format!(
                "could not run git in {}: {}",
                repository.display(),
                err, //
            )
        })?;

    if !held.status.success() {
        return Err(format!(
            "{} knows no commit at {} — the trunk counterpart must be a ref this clone already \
             holds, so fetch it first",
            repository.display(),
            trunk, //
        ));
    }

    Ok(trunk)
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
/// refusal nobody can read to the end is a refusal nobody acts on. The count
/// stands beside the named few, so a reader is told the size of what they are
/// seeing a corner of.
fn zbkce_parted(repository: &Path, stamp: &str, far: &str, elected: &[&str]) -> String {
    const ZBKCE_NAMED: usize = 5;

    let Ok(out) = Command::new("git")
        .arg("-C")
        .arg(repository)
        .arg("diff")
        .arg("--name-only")
        .arg(stamp)
        .arg(far)
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
            named.len() - ZBKCE_NAMED, //
        );
    }

    named.join(", ")
}

/// Hold the election for one collar against the tree it stands in and the
/// geography it was dispatched under.
///
/// THE TWO RESIDENCES ARE ONE JOIN TAKEN TWICE, over the delivered root and over
/// the work tree, which is what keeps a linked worktree's coordinates the same
/// coordinates an undispatched one has. Under self-geography the two roots are
/// the same tree and the join answers the same path, so the second reading is
/// skipped rather than taken and thrown away — an election that asked one
/// binary twice would suggest the two answers could differ.
pub fn bkce_elect(
    repository: &Path,
    geography: &bkce_Geography,
    collar: &bkcr_Collar,
) -> Result<bkce_Verdict, String> {
    let own = bkce_residence(repository, collar);
    let residence = match geography.bkce_delivered() {
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
            mute: false,
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
    let concord = bkce_concord(repository, standing.as_deref(), BKCE_SEAT_FAR, roots);

    // THE FENCE'S QUESTION IS PUT ONLY TO A BORROW, and only where the trees
    // agreed: a candidate whose tree parts is refused already and its plaint
    // already says why, and under self-geography the residence is the seat's own
    // build, whose struck exergues the preflight reads at the seat itself.
    let fenced = match geography.bkce_delivered() {
        Some(delivered) if concord.identical => zbkce_fenced(repository, delivered, collar),
        _ => None,
    };

    // THE OWN RESIDENCE IS ASKED WHEREVER THERE IS ONE, which is under dispatch
    // alone: a current own build answers ahead of a borrow, so its reading
    // decides the election whatever the delivered binary says. Under
    // self-geography it would be asking the same file the same question twice.
    let own_carried = geography
        .bkce_delivered()
        .map(|_| bkce_carried(&own).first().cloned());
    let second = own_carried
        .as_ref()
        .map(|carried| bkce_concord(repository, carried.as_deref(), BKCE_SEAT_FAR, roots));
    let own_current = second.as_ref().map(|concord| concord.identical);

    let (elected, runs) = bkce_weigh(spend, concord.identical && fenced.is_none(), own_current);

    // MUTE IS READ OFF THE BINARY THE CONVERGE WRITES, never the borrow
    // candidate's: under dispatch a mute delivered binary was built from another
    // tree and says nothing of what this seat's converge would make. Under
    // self-geography the two are one file, and its stamp is the one already read.
    let own_stamp = match geography.bkce_delivered() {
        Some(_) => own_carried.flatten(),
        None => standing.clone(),
    };
    let mute = !runs && own.is_file() && own_stamp.is_none();

    // WHICH BINARY ANSWERS FOLLOWS FROM WHICH READING CARRIED IT, and never from
    // the word alone: the borrow arm answers with the delivered binary and the
    // own arm with the seat's, and both wear the same word in the one case they
    // do not — which is why the path is carried rather than derived by a caller
    // reading `elected`.
    let answers = runs.then(|| match elected {
        BKCE_ELECTED_OWN => own.clone(),
        _ => residence.clone(),
    });

    // WHY THE BORROW DID NOT ANSWER, in the order the readings were taken: a
    // parted tree, then a fence reading against it, then — the trees agreeing
    // and the fence admitting it — the seat's own current build elected ahead.
    let plaint = if elected == BKCE_ELECTED_BORROW {
        None
    } else if !concord.identical {
        Some(concord.plaint)
    } else if let Some(fenced) = fenced {
        Some(fenced)
    } else {
        Some(format!(
            "the delivered binary at {} matches this seat's tree, but the seat's own build at {} \
             is current, and a current own build answers ahead of a borrow",
            residence.display(),
            own.display(), //
        ))
    };

    Ok(bkce_Verdict {
        elected,
        answers,
        residence,
        own,
        standing,
        seat: Some(seat),
        plaint,
        own_plaint: second
            .filter(|concord| !runs && !concord.identical)
            .map(|concord| concord.plaint),
        mute,
    })
}

/// The fence's question put to a borrow candidate: whether the trunk counterpart
/// has moved beneath any exergue the collar declares, as it stands at the
/// delivered root where that binary's build struck it. `None` where every
/// declared exergue stands current, and where the collar declares none — a
/// binary struck with no landed reading carries no fence that reading could arm.
///
/// THE STRUCK FILE STANDS FOR THE BINARY BESIDE IT. A strike is the first act of
/// the build that lays the binary, so the file at the delivered root carries the
/// landed reading that binary compiled in; reading it is the kennel asking that
/// binary's fence question with the instrument the preflight already holds,
/// against the same far side the striker resolves.
///
/// FAIL CLOSED, as an unreadable binary is. A declared file that is not there,
/// that will not read, or a trunk counterpart that cannot be named leaves the
/// borrow candidate's fitness unestablished, and the cost of being wrong is the
/// seat's own build rather than a run its own fence refuses.
fn zbkce_fenced(repository: &Path, delivered: &Path, collar: &bkcr_Collar) -> Option<String> {
    let declared = collar.bkcr_exergue();

    if declared.is_empty() {
        return None;
    }

    let far = match bkce_trunk(repository) {
        Ok(far) => far,
        Err(why) => {
            return Some(format!(
                "the delivered binary's fence reading could not be taken: {}",
                why, //
            ));
        }
    };

    for each in declared {
        let at = delivered.join(each);

        let struck = match bkce_struck(&at) {
            Ok(struck) => struck,
            Err(why) => {
                return Some(format!(
                    "the delivered binary's tree matches this seat's, but the exergue its build \
                     struck cannot say what it was struck from, so its own staleness fence cannot \
                     be presumed to admit it: {}",
                    why, //
                ));
            }
        };

        let concord = bkce_concord(repository, Some(&struck.sha), &far, &struck.roots);

        if !concord.identical {
            return Some(format!(
                "the delivered binary's tree matches this seat's, but the exergue its build struck \
                 at {} was struck at {} and {} has since moved the tree beneath its roots, so its \
                 own staleness fence refuses it — {}",
                at.display(),
                struck.sha,
                far,
                concord.plaint, //
            ));
        }
    }

    None
}

/// What a struck exergue file says about its own striking: the sha, and the
/// roots it was measured over.
pub struct bkce_Struck {
    /// The landed position the file was struck at.
    pub sha: String,
    /// The roots, whitespace-joined into the form `bkce_concord` splits.
    pub roots: String,
}

/// Read the sha and the roots off a struck exergue file.
///
/// A FACE OF THIS MODULE AND NOT THE PREFLIGHT'S LOCAL, because two readers put
/// the landed question to a struck file: the preflight to the seat's own, and the
/// election's fence reading to the delivered root's. One parse for both, or the
/// two would part the day the striker's shape moved.
///
/// THE SHAPE IS THE STRIKER'S AND THE CONSTANT'S NAME IS THE CALLER'S.
/// `Tools/buk/bue_exergue.sh` writes `pub const <NAME>: &str = "<sha>";` and
/// `pub const <NAME>_ROOTS: &[&str] = &[ ... ];` with one quoted root per line,
/// exclusion pathspecs included, and the striker's caller states `<NAME>`. So
/// the sha is the first string constant shaped like a position, and the roots
/// are the quoted elements of the `_ROOTS` block; the ancestry block is stepped
/// over, being the record-holding reader's instrument and not this one's.
///
/// CONSUMED AND NEVER RE-DERIVED. The file states what it was struck from;
/// walking the trunk a second time here would be a second striker, and the two
/// would part the day the walk changed. What the kennel asks is whether what the
/// file states is still the tree, and that question consumes the sha and the
/// roots as they stand.
///
/// A FILE THAT DOES NOT SAY REFUSES, fail-closed like a binary that states no
/// position: an exergue whose shape has moved from under this reader cannot be
/// presumed current, and the plaint says what was not found.
pub fn bkce_struck(at: &Path) -> Result<bkce_Struck, String> {
    let text = std::fs::read_to_string(at).map_err(|err| {
        format!(
            "could not read the exergue at {}: {}",
            at.display(),
            err, //
        )
    })?;

    let mut sha: Option<String> = None;
    let mut roots: Vec<String> = Vec::new();
    let mut within_roots = false;

    for line in text.lines() {
        let line = line.trim();

        if within_roots {
            if line.starts_with("];") {
                within_roots = false;
            } else if let Some(root) = zbkce_quoted(line) {
                roots.push(root);
            }
            continue;
        }

        let Some(declared) = line.strip_prefix("pub const ") else {
            continue;
        };

        if declared.contains("_ROOTS:") && declared.ends_with("&[") {
            within_roots = true;
            continue;
        }

        if sha.is_none() && declared.contains(": &str = ") {
            if let Some(value) = zbkce_quoted(declared) {
                if bkce_position_shaped(&value) {
                    sha = Some(value);
                }
            }
        }
    }

    let Some(sha) = sha else {
        return Err(format!(
            "the exergue at {} carries no position constant in the striker's shape — `pub const \
             <NAME>: &str = \"<sha>\";` — so what it was struck from cannot be read, and it may not \
             be presumed current",
            at.display(), //
        ));
    };

    if roots.is_empty() {
        return Err(format!(
            "the exergue at {} carries no roots constant in the striker's shape — `pub const \
             <NAME>_ROOTS: &[&str] = &[ ... ];` — so the set its sha was measured over cannot be \
             read, and it may not be presumed current",
            at.display(), //
        ));
    }

    Ok(bkce_Struck {
        sha,
        roots: roots.join(" "),
    })
}

/// The text between the first pair of double quotes on a line, where the line
/// carries one.
fn zbkce_quoted(line: &str) -> Option<String> {
    let open = line.find('"')?;
    let rest = &line[open + 1..];
    let close = rest.find('"')?;
    Some(rest[..close].to_string())
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
    geography: &bkce_Geography,
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
/// linked worktree's two residences one join taken twice — over the delivered
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
        .map_err(|err| {
            format!(
                "could not run git in {}: {}",
                repository.display(),
                err, //
            )
        })?;

    if !out.status.success() {
        return Err(format!(
            "git would not walk this seat's line over the collar's elected roots in {}: {}",
            repository.display(),
            String::from_utf8_lossy(&out.stderr).trim(), //
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
        .filter(|line| bkce_position_shaped(line))
        .collect()
}

/// Whether a line is shaped like a git object name: forty lowercase hex
/// characters and nothing else.
///
/// A SHAPE TEST AND NEVER A LOOKUP. The election compares positions for equality
/// and never resolves one, so what this must exclude is a line that could be
/// mistaken for a position — a banner, a version, a warning — rather than a
/// position that names no object. The preflight reads a struck exergue file by
/// the same test, which is why it is a face of the module rather than a local.
pub fn bkce_position_shaped(line: &str) -> bool {
    line.len() == 40
        && line
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}

// eof
