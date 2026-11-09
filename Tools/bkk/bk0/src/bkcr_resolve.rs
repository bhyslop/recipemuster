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

//! The resolver — the one road to a collar, and the validation every door gets
//! for free by taking it.
//!
//! NO DOOR VALIDATES AS ITS OWN ACT. A door asks for a collar by name and is
//! handed the collar with every conformance finding against it already
//! gathered; what the door decides is what to do about them, never whether to
//! look. That is what makes "never operate on an invalid collar" a property of
//! the shape rather than a rule each door has to remember, and it is why a door
//! reaching a `bkrr.env` around this module would be a defect however carefully
//! it read one.
//!
//! EVERY FINDING IS REPORTED BEFORE ANYTHING REFUSES. A first-finding exit hands
//! its reader one repair at a time and a fresh drive between each, which is the
//! shape the substrate's own validate door was built to avoid; this module
//! gathers and the door prints. What stops the gathering is a name the weaned
//! set does not hold, or a weaned copy that is not its kit's declaration,
//! because there is then no collar to gather findings against.
//!
//! DISCOVERY READS THE WEANED SET AND NOTHING ELSE. A kit ships its
//! declarations in its brood; a repository runs what it copied into its
//! moorings' weaned set; a file standing anywhere else in the tree is no
//! declaration to any door.

use crate::bkcj_json::{
    bkcj_Value,
    bkcj_read, //
};
use crate::bkcl_leash;
use brp::brprc_catena::{
    brprc_Regime,
    brprc_admit,
    brprc_read, //
};
use std::path::{
    Path,
    PathBuf, //
};

/// The rust family's file. Its parent directory is the instance and that
/// directory's name is the collar name.
pub const BKCR_FAMILY: &str = "bkrr.env";

/// The elected set's file, standing beside the family file of an app collar that
/// strikes an exergue and of no other collar: what that collar's artifact is made
/// from, derived from the record cargo writes beside the artifact and never
/// declared. It is no family file, so an instance holding one still holds one
/// family.
pub const BKCR_SET: &str = "bkrs.env";

/// Every family file an instance directory may hold, one of them and one only.
pub const BKCR_FAMILIES: &[&str] = &[
    BKCR_FAMILY,
    crate::bkcx_python::BKCX_FAMILY,
    crate::bkcq_kibble::BKCQ_FAMILY, //
];

/// The directory under a repository's moorings holding the declarations that
/// repository runs — the weaned set, and the only place discovery reads.
pub const BKCR_WEANED: &str = "bkd_weaned";

/// The directory under a kit holding the declarations that kit ships — its
/// brood, which a repository copies from and never runs.
pub const BKCR_BROOD: &str = "bkd_brood";

/// The tools directory every kit stands under, which the brood wildcard is
/// resolved beneath. A literal and never the substrate's variable, which a lure
/// states absolute into its host tree.
pub const BKCR_TOOLS: &str = "Tools";

/// The dispatch's config directory — the moorings — which the weaned set stands
/// under, read from the dispatch's file.
pub const BKCR_CONFIG_DIR_VAR: &str = "BURD_CONFIG_DIR";

/// The linter's own file, which a declared muzzle directory must hold. Foreign
/// schema, foreign name.
pub const BKCR_MUZZLE_FILE: &str = "clippy.toml";

/// The fields every collar declares, whatever its kind.
pub const BKCR_FIELDS_BOTH: &[&str] = &[
    "BKRR_COLLAR",
    "BKRR_KIND",
    "BKRR_MANIFEST",
    "BKRR_TARGET",
    "BKRR_FEATURES",
    "BKRR_PROFILE",
    "BKRR_SPEND",
    "BKRR_TAMED_CRATES",
    "BKRR_FERAL_CRATES",
    "BKRR_MUZZLE",
    "BKRR_EXERGUE", //
];

/// What `BKRR_EXERGUE` carries where the crate holds no generated source at
/// all — which is the ordinary case, and is DECLARED rather than left silent on
/// the roster's own rule.
pub const BKCR_UNSTRUCK: &str = "bknre_unstruck";

/// What `BKRR_FERAL_CRATES` carries where nothing in the closure is feral —
/// which at MVP is every collar, the field admitting this value alone and
/// refusing every crate named beside it (BKr_4n2).
pub const BKCR_TAMED: &str = "bknre_tamed";

/// The fields an app declares and a suite does not.
pub const BKCR_FIELDS_APP: &[&str] = &[
    "BKRR_RESIDENCE",
    "BKRR_BYNAME", //
];

/// The fields a suite declares and an app does not.
pub const BKCR_FIELDS_SUITE: &[&str] = &[
    "BKRR_RUNNER",
    "BKRR_TONGUE", //
];

/// The suite fields an app may declare as well: the tongue, whose presence makes
/// the app ATTENDED — a child whose stream the voice reads as a suite's is read,
/// where an app declaring none holds the terminal. Owed by a suite and admitted
/// on an app, so it is neither owed nor foreign there.
pub const BKCR_FIELDS_APP_ADMITS: &[&str] = &[
    "BKRR_TONGUE", //
];

/// The tongues each kind admits. A suite speaks a runner's shape and an app the
/// kennel's own, so a tongue declared on the wrong kind names a reader that kind
/// never feeds, and is a finding naming what the kind admits.
const ZBKCR_TONGUES_SUITE: &[&str] = &[
    crate::bkcv_voice::BKCV_TONGUE_HARNESS,
    crate::bkcv_voice::BKCV_TONGUE_NEXTEST, //
];
const ZBKCR_TONGUES_APP: &[&str] = &[
    crate::bkcv_voice::BKCV_TONGUE_PASSFAIL, //
];

/// The kind values, in the substrate's enum sprue form under this kit's own
/// plane. No bare word is admitted anywhere in this roster: bare, a value is
/// invisible to its enum family, and a recognizable prefix makes one grep return
/// the whole cluster.
pub const BKCR_KIND_APP: &str = "bknre_app";
pub const BKCR_KIND_SUITE: &str = "bknre_suite";

/// The value `BKRR_TARGET` carries where the suite is a whole manifest's tests
/// rather than one named target. The token is what keeps the field from standing
/// vacant, so that a door can tell a declaration from an omission.
pub const BKCR_TARGET_MANIFEST: &str = "bknre_manifest";

/// The enum-valued fields and the values each admits.
const ZBKCR_ENUMS: &[(&str, &[&str])] = &[
    (
        "BKRR_KIND",
        &[
            BKCR_KIND_APP,
            BKCR_KIND_SUITE, //
        ],
    ),
    (
        "BKRR_SPEND",
        &[
            "bknre_reader",
            "bknre_writer", //
        ],
    ),
    (
        "BKRR_RUNNER",
        &[
            "bknre_cargo",
            "bknre_nextest", //
        ],
    ), //
];

/// The schema version this module's reading is written against, asked for by
/// number rather than accepted as whatever the installed cargo defaults to.
const ZBKCR_FORMAT_VERSION: f64 = 1.0;

/// Cargo's own machine-readable account of a manifest, and the flags this module
/// asks for it under. Stated once so a second caller cannot ask a
/// differently-shaped question.
const ZBKCR_METADATA: &str = "metadata";
const ZBKCR_NO_DEPS: &str = "--no-deps";
const ZBKCR_FORMAT_FLAG: &str = "--format-version";
const ZBKCR_FORMAT: &str = "1";
const ZBKCR_OFFLINE: &str = "--offline";

/// Cargo's own name for a manifest, which is how a path dependency's directory
/// becomes the manifest to ask about next.
const ZBKCR_MANIFEST_FILE: &str = "Cargo.toml";

/// Cargo's own word for a dependency compiled only into a crate's TESTS.
///
/// WHICH DEPENDENCIES COUNT IS DECIDED BY THE KIND, and the first roster is the
/// evidence rather than a reading of the sheaf: an app collar and a suite collar
/// can declare one manifest between them while only the suite's compile reaches
/// a crate that manifest declares dev — because cargo compiles a dev-dependency
/// into a test target and never into a binary. A validator blind to that would
/// demand of an app a declaration that would be false about it.
///
/// The reach is the ROOT'S OWN, one level and no further: cargo builds a
/// dependency's dev-dependencies for nothing, so following a dev edge past the
/// crate under test would elect source no artifact is made from.
const ZBKCR_KIND_DEV: &str = "dev";

/// What a collar's currency is measured over: the paths a position is walked
/// over and two trees are compared over.
///
/// TWO ARMS, AND THE COLLAR CHOOSES NEITHER. Where an elected set stands beside
/// the collar its files are the measure, read off the set; where none stands the
/// measure is the closure's local package directories, read off cargo metadata.
/// No field of the collar names either, because a declaration restating what the
/// record or the closure already says is a second statement free to drift.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum bkcr_Measure {
    /// The elected set's files.
    Elected(Vec<String>),
    /// The closure's local package directories, the measure of a collar holding
    /// no set — wider than what is compiled, which is the safe direction.
    Packages(Vec<String>),
}

impl bkcr_Measure {
    /// The measured paths whitespace-joined, the form the position walk and the
    /// tree comparison split. No path in either arm carries whitespace: a catena
    /// cannot hold one, and the closure's directories are this estate's own.
    pub fn bkcr_pathspecs(&self) -> String {
        match self {
            bkcr_Measure::Elected(paths) | bkcr_Measure::Packages(paths) => paths.join(" "),
        }
    }

    /// Whether the measure is an elected set's.
    pub fn bkcr_elected(&self) -> bool {
        matches!(self, bkcr_Measure::Elected(_))
    }
}

/// An elected set as it stands on disk, or as a derivation composed it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct bkcr_Set {
    /// The files the artifact is made from, repo-relative and sorted.
    pub files: Vec<String>,
    /// The closure's local package directories, repo-relative and sorted.
    ///
    /// THEY RIDE FOR ONE QUESTION A FILE LIST CANNOT ANSWER: a file added
    /// beneath one of them since the record was taken, which no build has yet
    /// proven read. The landing refuses such a file only where a source the set
    /// lists compiles it in, the one case in which a build would have listed it.
    pub packages: Vec<String>,
}

/// The set file's opening lines. The set is generated and committed, so a
/// reader who meets it learns here that it is edited by a build and never by
/// hand.
const ZBKCR_SET_HEADER: &str = "\
# The elected set of the collar beside it: every file its artifact is made
# from and the local package directories that artifact's closure stands in.
#
# Generated by the kennel's scent door from the dep-info cargo writes beside the
# artifact, filtered by rule, with the manifests, the lockfile, the toolchain
# pin, the collar's own file and this file beside it. Commit it as you commit a
# lockfile and never edit it by hand: a build at a linked worktree that finds
# the dep-info naming a file this set does not rewrites it and refuses, and the
# remedy is to commit what it wrote and build again.
";

/// Read the elected set standing at a path, `None` where no set stands there.
///
/// A SET THAT STANDS AND DOES NOT READ REFUSES rather than reading as absent: a
/// malformed set read as no set would hand its collar the package measure
/// silently, and a collar whose election quietly widened is a collar whose
/// currency nobody asked about.
pub fn bkcr_set_read(at: &Path) -> Result<Option<bkcr_Set>, String> {
    if !at.is_file() {
        return Ok(None);
    }

    let source = std::fs::read_to_string(at).map_err(|err| {
        format!(
            "could not read the elected set at {}: {}",
            at.display(),
            err, //
        )
    })?;

    bkcr_set_admit(&source, &at.display().to_string()).map(Some)
}

/// Read an elected set's text already in hand, `whence` naming where it came
/// from for the refusals.
///
/// Split from the disk reading so a consumer holding bytes — a set read out of a
/// commit's tree rather than a working tree, and a hurdle composing one — reaches
/// the same parse and the same catena names rather than a second copy of either.
pub fn bkcr_set_admit(source: &str, whence: &str) -> Result<bkcr_Set, String> {
    let regime = brprc_admit(source, whence)?;

    let catena = |key: &str| -> Result<Vec<String>, String> {
        regime
            .brprc_catena(key)
            .map(|held| held.into_iter().map(str::to_string).collect())
            .ok_or_else(|| {
                format!(
                    "the elected set at {} declares no {} — the set states both its catenas, \
                     and a build at a linked worktree writes it whole",
                    whence,
                    key, //
                )
            })
    };

    Ok(bkcr_Set {
        files: catena("BKRS_FILES")?,
        packages: catena("BKRS_PACKAGES")?,
    })
}

/// The set's text, as the derivation writes it: the header, then each catena one
/// element per line, so a change to the set reads as lines added and removed.
pub fn bkcr_set_render(set: &bkcr_Set) -> String {
    let catena = |key: &str, held: &[String]| -> String {
        let mut said = format!("{}=\"", key);
        for (index, element) in held.iter().enumerate() {
            if index > 0 {
                said.push_str(" \\\n  ");
            }
            said.push_str(element);
        }
        said.push_str("\"\n");
        said
    };

    format!(
        "{}\n{}\n{}",
        ZBKCR_SET_HEADER,
        catena("BKRS_FILES", &set.files),
        catena("BKRS_PACKAGES", &set.packages), //
    )
}

/// A collar as it stands, with the regime it declared.
#[derive(Debug, Clone)]
pub struct bkcr_Collar {
    /// The collar name, which is the instance directory's own name.
    pub name: String,
    /// The instance directory, repo-relative.
    pub instance: PathBuf,
    /// Everything the family file declared.
    pub regime: brprc_Regime,
    /// What this collar's currency is measured over, read at resolution.
    pub measure: bkcr_Measure,
}

impl bkcr_Collar {
    /// Whether this collar takes an elected set: an app that strikes an exergue.
    /// A suite, and an app striking none, measures over its closure instead.
    pub fn bkcr_takes_set(&self) -> bool {
        self.bkcr_app() && !self.bkcr_exergue().is_empty()
    }

    /// One field's value, or the empty string where the field is absent. The
    /// absence is a finding of its own, so a reader here is never guessing.
    pub fn bkcr_field(&self, key: &str) -> &str {
        self.regime.brprc_scalar(key).unwrap_or("")
    }

    /// Whether this collar declares an app.
    pub fn bkcr_app(&self) -> bool {
        self.bkcr_field("BKRR_KIND") == BKCR_KIND_APP
    }

    /// Whether this collar declares an ATTENDED app: an app holding a tongue,
    /// which the launch door runs as a captured child and reads as it reads a
    /// suite. An app holding none holds the terminal. Asked here once, so no
    /// door reads the field to decide it.
    pub fn bkcr_attended(&self) -> bool {
        self.bkcr_app() && self.regime.brprc_holds("BKRR_TONGUE")
    }

    /// Every exergue this collar's compile requires and cargo does not produce,
    /// as repo-relative paths — empty where the collar declares that none
    /// stands.
    ///
    /// A LIST BECAUSE ONE COMPILE CAN OWE SEVERAL. A collar's crate is compiled
    /// with its local dependencies, and a dependency carrying an exergue of its
    /// own puts a second generated file in the way of the same invocation: a
    /// collar over a crate that carries an exergue AND depends on one owes both,
    /// struck by two different doors. A single-valued field would have made one
    /// invisible, and the invisible one is exactly the case that presented as an
    /// unrelated compiler error.
    ///
    /// The unstruck token answers empty and so does an absent field, which is
    /// not a second reading of silence: the field is on the owed roster, so an
    /// absent one is already a finding, and a door reached in spite of it
    /// behaves as it did before this field existed rather than inventing a
    /// meaning the collar never declared.
    pub fn bkcr_exergue(&self) -> Vec<&str> {
        match self.regime.brprc_catena("BKRR_EXERGUE") {
            None => Vec::new(),
            Some(declared) => declared
                .into_iter()
                .filter(|held| *held != BKCR_UNSTRUCK)
                .collect(),
        }
    }

    /// The doors that strike this collar's exergues, as an operator would type
    /// them.
    ///
    /// A SET BESIDE A SET, NEVER A PAIRING. The two lists are not read
    /// positionally and the kennel never asks which door writes which file: what
    /// an operator does about any absence is run the doors, and a cold seat owes
    /// the whole chain in the order the tree already fixes. Pairing them would
    /// have bought nothing a refusal renders and would have made the collar's
    /// two lists silently order-dependent.
    pub fn bkcr_strike(&self) -> Vec<&str> {
        match self.regime.brprc_catena("BKRR_STRIKE") {
            None => Vec::new(),
            Some(declared) => declared
                .into_iter()
                .filter(|held| *held != BKCR_UNSTRUCK)
                .collect(),
        }
    }
}

/// Whether this collar's exergue stands, and what to do about it where it does
/// not.
///
/// THE ONE PLACE EVERY COMPILE ROAD ASKS, and it is asked AHEAD OF CARGO on all
/// three. A crate whose exergue is absent does not fail to build in any way a
/// reader can act on: cargo reaches rustc, rustc meets a `mod` line naming a
/// file nobody wrote, and the operator is handed E0583 — a diagnostic about a
/// missing module, from a compiler that has no idea a bash door was supposed to
/// write it. Every other cold-seat refusal in this estate names the door to run,
/// and this is the reading that lets these three do the same.
///
/// THE KENNEL STRIKES NOTHING. This reads and refuses; the strike is the
/// declaring tree's own act, through the door the collar names, and doors report
/// and refuse rather than converging on their own.
///
/// STALENESS IS NOT ASKED HERE. An exergue that stands but records an older
/// position is caught by the fence in the tenant's own crate, which already
/// refuses on it correctly; a second implementation here would be a second thing
/// to keep in step, and would have to parse a generated file whose shape is the
/// striking door's rather than this kit's. What the kennel owes that case is
/// legibility, and heel pays it by naming the striking door on its verdict.
pub fn bkcr_exergue_stands(repository: &Path, collar: &bkcr_Collar) -> Result<(), String> {
    let absent: Vec<&str> = collar
        .bkcr_exergue()
        .into_iter()
        .filter(|declared| !repository.join(declared).is_file())
        .collect();

    if absent.is_empty() {
        return Ok(());
    }

    // EVERY ABSENT ONE IS NAMED, not the first. A cold seat holds none of them,
    // and a refusal that named one file at a time would make the operator drive
    // the door once per exergue to discover a chain the collar could have stated
    // whole.
    Err(format!(
        "the collar '{}' declares {} exergue(s) that do not stand: {} — these are generated and \
         never committed, so a seat that has not run the doors that write them holds none. Run {} \
         and drive this door again. Refused ahead of cargo deliberately: the compiler's own answer \
         to an absent module names the file and not the door that writes it",
        collar.name,
        absent.len(),
        absent.join(", "),
        collar.bkcr_strike().join(" "), //
    ))
}

/// A collar and every conformance finding against it.
#[derive(Debug, Clone)]
pub struct bkcr_Resolved {
    pub collar: bkcr_Collar,
    pub findings: Vec<String>,
}

/// Every rust instance the weaned set holds, repo-relative where the set stands
/// inside the repository, and sorted.
///
/// Sorted so that two drives over one tree report in one order — a listing
/// whose answer depended on the filesystem's own ordering would make a refusal
/// name its instances differently on different stations.
pub fn bkcr_walk(repository: &Path, weaned: &Path) -> Result<Vec<PathBuf>, String> {
    bkcr_weaned_family(repository, weaned, BKCR_FAMILY)
}

/// Every instance of ONE NAMED FAMILY the weaned set holds, repo-relative where
/// the set stands inside the repository, and sorted.
///
/// ONE READING FOR EVERY FAMILY. The kibble family (`bkcq_kibble`) and the
/// python family (`bkcx_python`) are read from the same set by the same rules,
/// and a reading of their own would be a second thing to keep in step. What
/// parts a family from another here is the file its instance carries and
/// nothing else, which is exactly what this parameter is.
///
/// AN INSTANCE HOLDING OTHER THAN ONE FAMILY FILE REFUSES the whole reading,
/// whichever family was asked for: a name stands once across the set whatever
/// its family, and an instance carrying two family files is one name claimed
/// by two families.
///
/// A WEANED SET THAT DOES NOT STAND HOLDS NOTHING, which is not a fallback to
/// anything: a repository that weaned nothing gives a derby nothing to run, and
/// the derby refuses that as it always has.
pub fn bkcr_weaned_family(
    repository: &Path,
    weaned: &Path,
    family: &str,
) -> Result<Vec<PathBuf>, String> {
    let mut found: Vec<PathBuf> = Vec::new();

    for (instance, held) in zbkcr_instances(weaned)? {
        if held == family {
            found.push(zbkcr_within(repository, &instance));
        }
    }

    found.sort();
    Ok(found)
}

/// The weaned set under a config directory — the moorings every dispatched
/// process holds.
pub fn bkcr_weaned(config: &Path) -> PathBuf {
    config.join(BKCR_WEANED)
}

/// The pointer a dispatch hands across its exec: the one name the kennel reads
/// from its environment for the dispatch's values, naming the file that holds
/// them all.
pub const BKCR_DISPATCH_FILE_VAR: &str = "BURD_BURD_FILE";

/// One dispatch's values, as the dispatch wrote them into its file.
///
/// READ ONCE, AT A PROCESS'S EDGE, AND CARRIED AS A VALUE. The door face reads
/// the file the pointer names and every reading below it takes this, so a hurdle
/// composing a lure hands its own dispatch in rather than inheriting the run
/// executing it, and no reading beneath a door face asks the environment again.
///
/// WHAT A FIELD MEANS IS ITS READER'S, never this type's. Each consumer names the
/// field it reads and refuses its own absence in its own words; what this type
/// answers is what the file said, a blank value standing as no value, since the
/// dispatch writes a field it composed nothing for as an empty assignment.
///
/// THE MOORINGS ARE PROVEN AT THE READ. A dispatch names its config directory
/// always, so a file lacking one was composed by no dispatch and refuses here;
/// the weaned set beneath it is then carried as a fact rather than re-asked by
/// every reading that walks it.
#[derive(Debug, Clone)]
pub struct bkcr_Dispatch {
    regime: brprc_Regime,
    whence: String,
    weaned: PathBuf,
}

impl bkcr_Dispatch {
    /// The dispatch stated by regime text already in hand, `whence` naming where
    /// it came from for the refusals — the road a hurdle composing one takes.
    pub fn bkcr_admit(source: &str, whence: &str) -> Result<bkcr_Dispatch, String> {
        zbkcr_dispatch(brp::brprc_catena::brprc_admit(source, whence)?, whence)
    }

    /// The dispatch stated by the file at `path`, read through the estate's one
    /// regime reader.
    pub fn bkcr_read(path: &Path) -> Result<bkcr_Dispatch, String> {
        zbkcr_dispatch(brprc_read(path)?, &path.display().to_string())
    }

    /// One field's value, or `None` where the file does not assign it or
    /// assigns it blank.
    pub fn bkcr_said(&self, field: &str) -> Option<&str> {
        self.regime
            .brprc_scalar(field)
            .map(str::trim)
            .filter(|value| !value.is_empty())
    }

    /// Where these values were read from, for a refusal naming a field the file
    /// lacks.
    pub fn bkcr_whence(&self) -> &str {
        &self.whence
    }

    /// The weaned set this dispatch names, beneath its config directory.
    pub fn bkcr_weaned(&self) -> &Path {
        &self.weaned
    }

    /// The tabtarget directory this dispatch names, for a refusal naming a door.
    pub fn bkcr_tabtargets(&self) -> Option<&Path> {
        self.bkcr_said(crate::bkco_output::BKCO_TABTARGET_DIR_VAR)
            .map(Path::new)
    }
}

/// A dispatch from a regime already read, its moorings proven.
///
/// ABSENT MOORINGS ARE A REFUSAL, never a fallback: there is no other weaned set
/// to read.
fn zbkcr_dispatch(regime: brprc_Regime, whence: &str) -> Result<bkcr_Dispatch, String> {
    let config = regime
        .brprc_scalar(BKCR_CONFIG_DIR_VAR)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            format!(
                "{} carries no {}, so no weaned set can be read — the kennel reads the \
                 declarations a repository runs from its moorings, which the dispatch names",
                whence,
                BKCR_CONFIG_DIR_VAR, //
            )
        })?;

    let weaned = bkcr_weaned(Path::new(config));

    Ok(bkcr_Dispatch {
        regime,
        whence: whence.to_string(),
        weaned,
    })
}

/// The dispatch this process was launched under, read from the file its pointer
/// names.
///
/// THE ONE ENVIRONMENT READ, TAKEN AT A PROCESS'S EDGE AND NEVER BENEATH IT.
/// Every reading of a dispatch value takes the answer as a value — the same
/// split the kibble root keeps (`bkcq_feedroom_at`). A caller below a door face
/// that reached for this would be the second road.
///
/// AN ABSENT POINTER, OR ONE NAMING NO READABLE FILE, IS A REFUSAL and never a
/// fallback: the dispatch hands the pointer to every coordinator, so a process
/// lacking it was not launched through a tabtarget, and there is no other place
/// the dispatch's values stand.
pub fn bkcr_dispatched() -> Result<bkcr_Dispatch, String> {
    let pointer = std::env::var_os(BKCR_DISPATCH_FILE_VAR)
        .filter(|pointer| !pointer.is_empty())
        .ok_or_else(|| {
            format!(
                "{} is not set, so no dispatch's values can be read — the kennel reads its roots, \
                 its log names, its tackroom and its moorings from the file a dispatch names; run \
                 it through a tabtarget",
                BKCR_DISPATCH_FILE_VAR, //
            )
        })?;

    bkcr_Dispatch::bkcr_read(Path::new(&pointer)).map_err(|err| {
        format!(
            "{} names a dispatch file the kennel cannot read: {}",
            BKCR_DISPATCH_FILE_VAR,
            err, //
        )
    })
}

/// Prove the whole weaned set conforms, before any door reads a declaration.
///
/// FAIL-FAST AND WHOLE-SET. Every instance is proven to hold one family file
/// and to equal its brood, and the first that does not refuses; a drift is
/// never batched into a report, because a difference between a copy and its
/// kit's declaration is a change to be taken consciously rather than one of
/// several findings to be read past. Answers how many instances stand.
pub fn bkcr_weaned_audit(repository: &Path, weaned: &Path) -> Result<usize, String> {
    let instances = zbkcr_instances(weaned)?;

    for (instance, family) in &instances {
        bkcr_drift(repository, instance, family)?;
    }

    Ok(instances.len())
}

/// Prove one weaned instance equals the brood that carries its name, if any.
///
/// THE BROOD IS FOUND BY ONE WILDCARD, `Tools/*/bkd_brood/<name>/`, resolved
/// under the repository the door entered and never from a variable: the lure
/// states the tools directory absolute into the host tree, so a reading taken
/// from it would compare a composed repository's copy against the host's own
/// kits.
///
/// NO BROOD MEANS THE INSTANCE IS THIS REPOSITORY'S OWN, and nothing is
/// compared. TWO BROODS REFUSE, because a name two kits both declare names no
/// declaration a copy could be proven against. ONE BROOD AND DIFFERENT REFUSES,
/// naming both files and the copy that repairs it — and "different" includes a
/// brood holding the name under another family, which is the same fact.
pub fn bkcr_drift(repository: &Path, instance: &Path, family: &str) -> Result<(), String> {
    let name = zbkcr_named(instance);
    let broods = zbkcr_broods(repository, &name)?;

    if broods.len() > 1 {
        let mut said = format!(
            "the name '{}' is declared by {} kits' broods, so a weaned copy of it can be proven \
             against none — one name stands once across every kit:",
            name,
            broods.len(), //
        );
        for brood in &broods {
            said.push_str("\n  ");
            said.push_str(&zbkcr_within(repository, brood).display().to_string());
        }
        return Err(said);
    }

    let Some(brood) = broods.first() else {
        return Ok(());
    };

    zbkcr_copied(
        repository,
        &instance.join(family),
        &brood.join(family),
        true,
    )?;

    // THE ELECTED SET IS HELD TO ITS BROOD AS THE FAMILY FILE IS, and absent on
    // both sides is the one agreement a set adds: a collar taking none holds none
    // in either place. A set on one side alone is a drift like any other, the
    // copy that repairs it being the brood's.
    let copy = instance.join(BKCR_SET);
    let declared = brood.join(BKCR_SET);
    if copy.is_file() || declared.is_file() {
        zbkcr_copied(repository, &copy, &declared, false)?;
    }

    Ok(())
}

/// Prove one weaned file byte-equal to the brood file it was copied from, and
/// refuse naming both files and the copy that repairs it where it is not.
///
/// `owed` says whether the weaned file must stand: a family file always must, so
/// a copy that will not read is a fault to name; a set file may stand on the
/// brood side alone, which is a drift and not a read failure.
fn zbkcr_copied(repository: &Path, copy: &Path, declared: &Path, owed: bool) -> Result<(), String> {
    let equal = match (std::fs::read(copy), std::fs::read(declared)) {
        (Ok(held), Ok(kit)) => held == kit,
        (Err(err), _) if owed => {
            return Err(format!(
                "could not read {}: {}",
                copy.display(),
                err, //
            ));
        }
        _ => false,
    };

    if equal {
        return Ok(());
    }

    let copy_shown = zbkcr_within(repository, copy);
    let declared_shown = zbkcr_within(repository, declared);

    Err(format!(
        "the weaned copy {} differs from its brood {} — activation is copying, never authoring, so \
         a copy may not drift from the kit that declares it. Take the kit's declaration: cp {} {} \
         — or, where the kit's declaration is what is wrong, repair it in the kit",
        copy_shown.display(),
        declared_shown.display(),
        declared_shown.display(),
        copy_shown.display(), //
    ))
}

/// Every place a collar's elected set stands: its weaned copy first, then the
/// brood it was copied from where one carries the collar's name.
///
/// THE TWO ARE WRITTEN TOGETHER OR NOT AT ALL, which is what keeps the drift
/// proof above from refusing the very build that derived the set: a copy that a
/// derivation wrote without its brood would read at the next door as a weaned
/// file that is not the kit's declaration.
pub fn bkcr_set_homes(repository: &Path, collar: &bkcr_Collar) -> Result<Vec<PathBuf>, String> {
    let mut homes = vec![repository.join(&collar.instance).join(BKCR_SET)];

    let broods = zbkcr_broods(repository, &collar.name)?;
    if let Some(brood) = broods.first() {
        homes.push(brood.join(BKCR_SET));
    }

    Ok(homes)
}

/// Every instance the weaned set holds, with the one family file each carries.
fn zbkcr_instances(weaned: &Path) -> Result<Vec<(PathBuf, &'static str)>, String> {
    let listing = match std::fs::read_dir(weaned) {
        Ok(listing) => listing,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => {
            return Err(format!(
                "could not read {}: {}",
                weaned.display(),
                err, //
            ));
        }
    };

    let mut instances: Vec<(PathBuf, &'static str)> = Vec::new();

    for entry in listing {
        let entry = entry.map_err(|err| {
            format!(
                "could not read {}: {}",
                weaned.display(),
                err, //
            )
        })?;
        let instance = entry.path();

        if !instance.is_dir() {
            return Err(format!(
                "{} stands in the weaned set and is no instance directory — the set holds one \
                 directory per declaration and nothing else",
                instance.display(), //
            ));
        }

        let held: Vec<&'static str> = BKCR_FAMILIES
            .iter()
            .copied()
            .filter(|family| instance.join(family).is_file())
            .collect();

        if held.len() != 1 {
            return Err(format!(
                "the weaned instance {} holds {} family file(s) where it must hold one — one of {}",
                instance.display(),
                held.len(),
                BKCR_FAMILIES.join(", "), //
            ));
        }

        instances.push((instance, held[0]));
    }

    instances.sort();
    Ok(instances)
}

/// Every brood instance carrying one name, found under the repository's own
/// tools directory.
fn zbkcr_broods(repository: &Path, name: &str) -> Result<Vec<PathBuf>, String> {
    let tools = repository.join(BKCR_TOOLS);

    let listing = match std::fs::read_dir(&tools) {
        Ok(listing) => listing,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => {
            return Err(format!(
                "could not read {}: {}",
                tools.display(),
                err, //
            ));
        }
    };

    let mut broods: Vec<PathBuf> = Vec::new();

    for entry in listing {
        let entry = entry.map_err(|err| {
            format!(
                "could not read {}: {}",
                tools.display(),
                err, //
            )
        })?;
        let candidate = entry.path().join(BKCR_BROOD).join(name);
        if candidate.is_dir() {
            broods.push(candidate);
        }
    }

    broods.sort();
    Ok(broods)
}

/// A path shown relative to the repository where it stands inside it, and as it
/// is where it does not.
fn zbkcr_within(repository: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(repository)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| path.to_path_buf())
}

/// Resolve a collar by name, and gather every finding against it.
///
/// THE INSTANCE IS PROVEN AGAINST ITS BROOD BEFORE ANYTHING ELSE IS READ, so a
/// drifted copy refuses rather than being validated: the findings of a copy that
/// is not the kit's declaration would be findings about a declaration nobody
/// made.
pub fn bkcr_resolve(
    repository: &Path,
    dispatch: &bkcr_Dispatch,
    name: &str,
) -> Result<bkcr_Resolved, String> {
    let instances = bkcr_walk(repository, dispatch.bkcr_weaned())?;

    let Some(instance) = instances
        .iter()
        .find(|instance| zbkcr_named(instance) == name)
        .cloned()
    else {
        return Err(format!(
            "no collar named '{}' stands in the weaned set {} — it holds {} collar(s): {}",
            name,
            zbkcr_within(repository, dispatch.bkcr_weaned()).display(),
            instances.len(),
            zbkcr_listed(&instances), //
        ));
    };

    bkcr_drift(repository, &repository.join(&instance), BKCR_FAMILY)?;

    let regime = brprc_read(&repository.join(&instance).join(BKCR_FAMILY))?;
    let set = bkcr_set_read(&repository.join(&instance).join(BKCR_SET))?;

    // THE MEASURE IS READ HERE, ONCE, and carried on the collar, because every
    // door that walks a position or compares two trees needs it and only this
    // reading holds the dispatch the closure is asked through. A closure that
    // cannot be read leaves the measure empty and says why as a finding, so a
    // door refuses on the reason rather than on an empty walk.
    let mut unmeasured: Option<String> = None;
    let measure = match &set {
        Some(set) => bkcr_Measure::Elected(set.files.clone()),
        None => bkcr_Measure::Packages(zbkcr_packages(
            repository,
            dispatch,
            &regime,
            &mut unmeasured,
        )),
    };

    let collar = bkcr_Collar {
        name: name.to_string(),
        instance,
        regime,
        measure,
    };

    let mut findings = zbkcr_findings(repository, dispatch, &collar);
    findings.extend(unmeasured);
    zbkcr_set(&collar, set.as_ref(), &mut findings);

    Ok(bkcr_Resolved { collar, findings })
}

/// The closure's local package directories, the measure of a collar holding no
/// set, read through cargo metadata as the closure always is.
///
/// A MANIFEST THAT IS NOT THERE IS ALREADY A FINDING, so it measures nothing and
/// says nothing more; a closure that will not read is said once, as the reason
/// the measure stands empty.
fn zbkcr_packages(
    repository: &Path,
    dispatch: &bkcr_Dispatch,
    regime: &brprc_Regime,
    unmeasured: &mut Option<String>,
) -> Vec<String> {
    let declared = regime.brprc_scalar("BKRR_MANIFEST").unwrap_or("");
    let manifest = repository.join(declared);

    if declared.is_empty() || !manifest.is_file() {
        return Vec::new();
    }

    let tested = regime.brprc_scalar("BKRR_KIND") != Some(BKCR_KIND_APP);

    match bkcr_closure(repository, dispatch, &manifest, tested) {
        Ok(packages) => packages.into_iter().map(bkcr_package_spec).collect(),
        Err(err) => {
            *unmeasured = Some(format!(
                "the closure's package directories could not be read, so this collar holds no \
                 measure for its currency: {}",
                err, //
            ));
            Vec::new()
        }
    }
}

/// A package directory as a pathspec git and a catena can both carry: the
/// closure answers a crate standing at the repository's root as the empty
/// directory, which a whitespace-joined measure would drop and a catena could
/// not hold, so it reads as the root itself.
pub fn bkcr_package_spec(package: String) -> String {
    if package.is_empty() {
        ".".to_string()
    } else {
        package
    }
}

/// An elected set stands beside exactly the collars that take one, and names
/// at least one file where it stands.
///
/// A SET BESIDE A COLLAR THAT TAKES NONE IS A FINDING AND NEVER A MEASURE IT IS
/// HELD TO: only an app striking an exergue has a build door that derives one, so
/// a set beside any other collar is a file no build will ever bring current.
fn zbkcr_set(collar: &bkcr_Collar, set: Option<&bkcr_Set>, findings: &mut Vec<String>) {
    let Some(set) = set else {
        return;
    };

    if !collar.bkcr_takes_set() {
        findings.push(format!(
            "an elected set {} stands beside a collar that takes none — only an app collar \
             striking an exergue holds one, and every other collar measures over its closure's \
             package directories",
            BKCR_SET, //
        ));
    }

    if set.files.is_empty() {
        findings.push(format!(
            "the elected set {} names no file — a position measured over nothing would call \
             every binary current",
            BKCR_SET, //
        ));
    }
}

/// Every conformance finding against one collar, gathered whole.
fn zbkcr_findings(
    repository: &Path,
    dispatch: &bkcr_Dispatch,
    collar: &bkcr_Collar,
) -> Vec<String> {
    let mut findings: Vec<String> = Vec::new();

    zbkcr_declared(collar, &mut findings);
    zbkcr_identity(collar, &mut findings);
    zbkcr_muzzle(repository, collar, &mut findings);
    zbkcr_cargo(repository, dispatch, collar, &mut findings);

    findings
}

/// Which fields must stand, which must not, and which values each enum admits.
///
/// The kind decides the roster, which is why the kind is a declared field rather
/// than something inferred from what else the file happens to carry: a validator
/// that guessed the kind from the fields present could never report a MISSING
/// one.
fn zbkcr_declared(collar: &bkcr_Collar, findings: &mut Vec<String>) {
    let kind = collar.bkcr_field("BKRR_KIND");

    let (owed, foreign): (&[&str], &[&str]) = match kind {
        BKCR_KIND_APP => (BKCR_FIELDS_APP, BKCR_FIELDS_SUITE),
        BKCR_KIND_SUITE => (BKCR_FIELDS_SUITE, BKCR_FIELDS_APP),
        _ => (&[], &[]),
    };

    for field in BKCR_FIELDS_BOTH.iter().chain(owed.iter()) {
        if !collar.regime.brprc_holds(field) {
            findings.push(format!(
                "{} is not declared — every collar declares it, and a door reading a meaning out \
                 of an absence cannot tell a declaration from an omission",
                field, //
            ));
        }
    }

    for field in foreign {
        if kind == BKCR_KIND_APP && BKCR_FIELDS_APP_ADMITS.contains(field) {
            continue;
        }
        if collar.regime.brprc_holds(field) {
            findings.push(format!(
                "{} is declared and belongs to the other kind — this collar declares {} = {}",
                field,
                "BKRR_KIND",
                kind, //
            ));
        }
    }

    for (field, admitted) in ZBKCR_ENUMS {
        let Some(held) = collar.regime.brprc_scalar(field) else {
            continue;
        };
        if !admitted.contains(&held) {
            findings.push(format!(
                "{} carries '{}', which is no admitted value — it takes one of: {}",
                field,
                held,
                admitted.join(", "), //
            ));
        }
    }

    zbkcr_tongue(collar, kind, findings);

    zbkcr_exergue(collar, findings);

    if collar.regime.brprc_holds("BKRR_MUZZLE") && collar.bkcr_field("BKRR_MUZZLE").is_empty() {
        findings.push(
            "BKRR_MUZZLE stands vacant — it is required and never vacant, the consuming tree's \
             own root being the ordinary value"
                .to_string(),
        );
    }
}

/// The tongue a collar declares is one its kind admits.
///
/// THE ROSTER IS THE KIND'S, NEVER ONE FOR EVERY COLLAR. A suite speaks the shape
/// of the runner it declares beside the tongue, and an app speaks the kennel's
/// own, so a runner's tongue on an app and the kennel's tongue on a suite each
/// name a reader that kind never feeds. A collar of no admitted kind is already a
/// finding, and its tongue is not judged against a roster it has none of.
fn zbkcr_tongue(collar: &bkcr_Collar, kind: &str, findings: &mut Vec<String>) {
    let Some(held) = collar.regime.brprc_scalar("BKRR_TONGUE") else {
        return;
    };

    let (admitted, owner): (&[&str], &str) = match kind {
        BKCR_KIND_APP => (ZBKCR_TONGUES_APP, "an app"),
        BKCR_KIND_SUITE => (ZBKCR_TONGUES_SUITE, "a suite"),
        _ => return,
    };

    if !admitted.contains(&held) {
        findings.push(format!(
            "BKRR_TONGUE carries '{}', which is no tongue {} admits — {} takes one of: {}",
            held,
            owner,
            owner,
            admitted.join(", "), //
        ));
    }
}

/// The exergue and the door that strikes it move together, or neither stands.
///
/// A DECLARATION THAT NAMES A FILE AND NOT A DOOR IS THE WORSE HALF TO BE
/// MISSING, which is why this is validated rather than left to the refusal to
/// discover: the whole point of the field is that a refusal can name the door to
/// run, and one composed from a vacant field would name nothing and read as a
/// defect in the kennel. The other direction is checked too, and is the ordinary
/// authoring slip — a strike left standing under a collar whose generated file
/// has since moved into the tree.
fn zbkcr_exergue(collar: &bkcr_Collar, findings: &mut Vec<String>) {
    if !collar.regime.brprc_holds("BKRR_EXERGUE") {
        return;
    }

    let exergues = collar.bkcr_exergue();
    let strikes = collar.bkcr_strike();

    if exergues.is_empty() && !strikes.is_empty() {
        findings.push(format!(
            "BKRR_STRIKE names {} and BKRR_EXERGUE declares {} — doors that strike nothing are \
             not owed, and the two fields are declared together or not at all",
            strikes.join(" "),
            BKCR_UNSTRUCK, //
        ));
    }

    if !exergues.is_empty() && strikes.is_empty() {
        findings.push(format!(
            "BKRR_EXERGUE names {} and BKRR_STRIKE stands vacant — a refusal over an absent \
             exergue exists to name the doors that write it, and one composed from a vacant field \
             would name nothing",
            exergues.join(" "), //
        ));
    }
}

/// The identity law: the instance directory's name and the declared name are one.
fn zbkcr_identity(collar: &bkcr_Collar, findings: &mut Vec<String>) {
    let declared = collar.bkcr_field("BKRR_COLLAR");

    if declared != collar.name {
        findings.push(format!(
            "BKRR_COLLAR declares '{}' and the instance directory is named '{}' — a collar's name \
             is its identity, and the two must be one",
            declared,
            collar.name, //
        ));
    }
}

/// The muzzle directory stands, and holds the linter's file.
fn zbkcr_muzzle(repository: &Path, collar: &bkcr_Collar, findings: &mut Vec<String>) {
    let declared = collar.bkcr_field("BKRR_MUZZLE");

    if declared.is_empty() {
        return;
    }

    let directory = repository.join(declared);

    if !directory.is_dir() {
        findings.push(format!(
            "BKRR_MUZZLE names '{}', where no directory stands",
            declared, //
        ));
        return;
    }

    if !directory.join(BKCR_MUZZLE_FILE).is_file() {
        findings.push(format!(
            "BKRR_MUZZLE names '{}', which holds no {} — a muzzle directory holding no lint list \
             is a muzzle nothing wears",
            declared,
            BKCR_MUZZLE_FILE, //
        ));
    }
}

/// The proof that needs cargo: the permitted crates against the manifest's
/// direct dependencies.
///
/// CARGO IS THE ORACLE AND NOTHING HERE RECOMPUTES IT. What each manifest
/// declares is read back from cargo's own account of it; a reader that parsed
/// manifests itself would be a second resolver, and the whole election of this
/// backend is that nothing reimplements what cargo knows.
///
/// IT GOES THROUGH THE LEASH, never a bare command, so the pin is stated, the
/// lock is enforced and the fence is proven for this reading exactly as for a
/// build.
///
/// THE CLOSURE IS WALKED LOCALLY RATHER THAN RESOLVED WHOLE, and that is forced
/// rather than preferred. A resolved closure is every package in the lock, and
/// asking for one offline refuses wherever the lock names a crate this station
/// never fetched — a platform-gated dependency is the standing case, resolved
/// into the lock on every platform and downloaded on none but its own. Asking
/// for it ONLINE is worse: a routine invocation verifies and never downloads,
/// and tattoo is as routine as an invocation gets. So each LOCAL manifest is
/// asked about on its own, with no closure resolved at any step, and the local
/// edges are followed from one answer to the next. Every registry package is
/// thereby out of the question by construction, which is also the honest scope:
/// what a collar measures is the source compiled from OUR trees, and a registry
/// crate is pinned by the lock rather than elected by a collar.
fn zbkcr_cargo(
    repository: &Path,
    dispatch: &bkcr_Dispatch,
    collar: &bkcr_Collar,
    findings: &mut Vec<String>,
) {
    let declared = collar.bkcr_field("BKRR_MANIFEST");

    if declared.is_empty() {
        return;
    }

    let manifest = repository.join(declared);

    if !manifest.is_file() {
        findings.push(format!(
            "BKRR_MANIFEST names '{}', where no manifest stands",
            declared, //
        ));
        return;
    }

    let account = match zbkcr_account(repository, dispatch, &manifest) {
        Ok(account) => account,
        Err(err) => {
            findings.push(err);
            return;
        }
    };

    // A suite is compiled with the crate's dev-dependencies and an app is not,
    // so the kind decides what the proof below is taken over.
    let tested = !collar.bkcr_app();

    zbkcr_crates(collar, &account, tested, findings);
}

/// Whether a dependency of this kind is compiled into what the collar declares.
///
/// `own` says whether the edge leaves the crate the collar names, which is the
/// only place a dev edge is ever followed.
fn zbkcr_compiled(kind: &str, own: bool, tested: bool) -> bool {
    if kind == ZBKCR_KIND_DEV {
        return own && tested;
    }
    true
}

/// One dependency's kind, cargo's own word for it. A normal dependency states
/// null, which reads here as the empty word.
fn zbkcr_kind(dependency: &bkcj_Value) -> &str {
    dependency
        .bkcj_field("kind")
        .and_then(bkcj_Value::bkcj_string)
        .unwrap_or("")
}

/// THE BUILD DIRECTORY CARGO ITSELF REPORTS for a manifest, absolute and as
/// cargo spells it.
///
/// TAKEN FROM CARGO RATHER THAN COMPOSED. A manifest's directory plus `target`
/// is right for most crates and wrong for exactly the ones a sweep must not get
/// wrong: a crate whose build directory is moved by configuration or by
/// environment would be missed, and a workspace member's would be named where
/// nothing stands. Cargo holds the answer, so it is asked.
///
/// It is a fact about the ANSWER rather than about a package, which is why it is
/// read from the account's own root: the field stands beside `packages` and not
/// inside one, and a workspace answers one build directory for every member.
pub fn bkcr_target_directory(
    repository: &Path,
    dispatch: &bkcr_Dispatch,
    manifest: &Path,
) -> Result<PathBuf, String> {
    let answer = zbkcr_answer(repository, dispatch, manifest)?;

    answer
        .bkcj_field("target_directory")
        .and_then(bkcj_Value::bkcj_string)
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!(
                "cargo's account of '{}' names no build directory",
                manifest.display(), //
            )
        })
}

/// Cargo's own account of ONE manifest, no closure resolved.
///
/// The package asked for is the one whose manifest path is the one asked about,
/// matched after both are settled: a manifest naming a workspace answers for its
/// members too, and the answer must be about the crate the collar declares.
fn zbkcr_account(
    repository: &Path,
    dispatch: &bkcr_Dispatch,
    manifest: &Path,
) -> Result<bkcj_Value, String> {
    let answer = zbkcr_answer(repository, dispatch, manifest)?;

    let packages = answer
        .bkcj_field("packages")
        .and_then(bkcj_Value::bkcj_array)
        .ok_or_else(|| {
            format!(
                "cargo's account of '{}' names no packages",
                manifest.display(), //
            )
        })?;

    let wanted = zbkcr_settled(manifest).ok_or_else(|| {
        format!(
            "'{}' settles to no real path, so cargo's answer cannot be matched to it",
            manifest.display(), //
        )
    })?;

    packages
        .iter()
        .find(|package| {
            package
                .bkcj_field("manifest_path")
                .and_then(bkcj_Value::bkcj_string)
                .and_then(|seen| zbkcr_settled(Path::new(seen)))
                .is_some_and(|seen| seen == wanted)
        })
        .cloned()
        .ok_or_else(|| {
            format!(
                "cargo's account names {} package(s) and none of them stands at '{}'",
                packages.len(),
                manifest.display(), //
            )
        })
}

/// CARGO'S WHOLE ANSWER about a manifest, decoded and its schema checked.
///
/// ONE QUESTION, TWO READINGS. The package a collar declares and the build
/// directory that manifest builds into are both answered by this one invocation,
/// and homing it here is what keeps a second caller from asking a
/// differently-shaped question — the flags above say as much, and a second
/// composition of them would make that sentence false.
fn zbkcr_answer(
    repository: &Path,
    dispatch: &bkcr_Dispatch,
    manifest: &Path,
) -> Result<bkcj_Value, String> {
    let recalled = bkcl_leash::bkcl_recall(
        repository,
        dispatch,
        manifest,
        ZBKCR_METADATA,
        [
            ZBKCR_NO_DEPS,
            ZBKCR_FORMAT_FLAG,
            ZBKCR_FORMAT,
            ZBKCR_OFFLINE,
        ],
        &[],
    )
    .map_err(|err| {
        format!(
            "cargo could not be asked about '{}': {}",
            manifest.display(),
            err, //
        )
    })?;

    // THE CODE IS READ BEFORE THE ANSWER IS, which is the shape the recalling
    // face asks of its callers: it answers `Ok` for a cargo that exited nonzero,
    // the refusal being about reaching cargo at all, so a caller that read the
    // answer first would parse whatever a failed run happened to leave behind.
    if !recalled.run.bkcl_landed() {
        return Err(format!(
            "cargo refused to describe '{}': {}",
            manifest.display(),
            recalled.grievance.trim(), //
        ));
    }

    // THE DECODE IS CHECKED RATHER THAN LOSSY, which is the whole reason the
    // face hands back bytes: a lossy decode would replace an unreadable byte
    // with a valid character and hand the parser text that is still well formed,
    // corrupting an answer where no reader downstream could detect it. An answer
    // that is not text is a refusal here.
    let said = std::str::from_utf8(&recalled.said).map_err(|err| {
        format!(
            "cargo's account of '{}' is not text: {}",
            manifest.display(),
            err, //
        )
    })?;

    let answer = bkcj_read(said).map_err(|err| {
        format!(
            "cargo's account of '{}' could not be read: {}",
            manifest.display(),
            err, //
        )
    })?;

    if answer
        .bkcj_field("version")
        .and_then(bkcj_Value::bkcj_number)
        != Some(ZBKCR_FORMAT_VERSION)
    {
        return Err(format!(
            "cargo answered about '{}' in a schema this reading was not written against",
            manifest.display(), //
        ));
    }

    Ok(answer)
}

/// Every LOCAL package feeding the artifact this manifest declares, repo-relative
/// and including the manifest's own.
///
/// PUBLISHED SO A SECOND READER NEEDS NO SECOND WALK. The gangline's arrears
/// reading asks which collars a re-derived manifest stands upstream of, which is
/// this same question asked from the other end — and a walk of its own would be a
/// second thing to keep in step, the dev-edge rule among them, so that one tree
/// could come to hold two answers about what feeds what.
///
/// A dependency cargo reports with a path is one standing in a tree of ours, and
/// its own dependencies are followed in turn. The visited set is a CYCLE GUARD
/// rather than an efficiency: a dev-dependency may point back at a crate already
/// walked, which cargo permits and which would otherwise walk forever.
///
/// A local package standing OUTSIDE the repository is walked and then dropped
/// from the answer: no measure of this collar's could hold it, and the leash
/// already refuses to build a crate that stands outside the tree it is bounded
/// by.
pub fn bkcr_closure(
    repository: &Path,
    dispatch: &bkcr_Dispatch,
    manifest: &Path,
    tested: bool,
) -> Result<Vec<String>, String> {
    let mut walked: Vec<PathBuf> = Vec::new();
    let mut owed: Vec<(PathBuf, bool)> = vec![(manifest.to_path_buf(), true)];
    let mut local: Vec<String> = Vec::new();

    while let Some((next, own)) = owed.pop() {
        let settled = std::fs::canonicalize(&next).unwrap_or_else(|_| next.clone());
        if walked.contains(&settled) {
            continue;
        }
        walked.push(settled);

        let account = zbkcr_account(repository, dispatch, &next)?;

        if let Some(directory) = zbkcr_relative(repository, next.parent()) {
            if !local.contains(&directory) {
                local.push(directory);
            }
        }

        let Some(dependencies) = account
            .bkcj_field("dependencies")
            .and_then(bkcj_Value::bkcj_array)
        else {
            continue;
        };

        for dependency in dependencies {
            if !zbkcr_compiled(zbkcr_kind(dependency), own, tested) {
                continue;
            }

            let Some(path) = dependency
                .bkcj_field("path")
                .and_then(bkcj_Value::bkcj_string)
            else {
                continue;
            };

            owed.push((Path::new(path).join(ZBKCR_MANIFEST_FILE), false));
        }
    }

    Ok(local)
}

/// Every direct dependency the manifest declares is named in exactly one of the
/// collar's two crate lists.
///
/// LOCAL AND EXTERNAL ALIKE. The field's job is that nothing links in unnoticed,
/// and a sibling crate in this repository is as much a link as a registry crate
/// is; exempting the local ones would leave the one class of dependency a
/// reviewer is least likely to look for outside the declaration.
///
/// WHAT IS EXEMPT IS DECIDED BY THE KIND AND NEVER BY THE CRATE. An app is not
/// compiled with its crate's dev-dependencies, so demanding that its collar
/// permit one would demand a declaration that is false about the artifact.
///
/// THE GUARD READS THE UNION, which is what makes the partition a partition
/// rather than two rosters: a name in either list has been declared, and a name
/// in both declares two contradictory things about one dependency.
fn zbkcr_crates(
    collar: &bkcr_Collar,
    account: &bkcj_Value,
    tested: bool,
    findings: &mut Vec<String>,
) {
    let Some(tamed) = collar.regime.brprc_catena("BKRR_TAMED_CRATES") else {
        return;
    };

    // The none-sprue is not a crate name, so it leaves the reading here and the
    // union below is over crates alone.
    let feral: Vec<&str> = match collar.regime.brprc_catena("BKRR_FERAL_CRATES") {
        None => Vec::new(),
        Some(declared) => declared
            .into_iter()
            .filter(|held| *held != BKCR_TAMED)
            .collect(),
    };

    for named in &feral {
        findings.push(format!(
            "BKRR_FERAL_CRATES names '{}' — a feral crate is refused, never provisioned \
             (BKr_4n2). The kennel provisions nothing for a build script and exports no variable \
             into one, so the field admits {} alone: the crate leaves the closure, or its fetching \
             half is replaced by something the tree holds and the collar declares it tamed",
            named,
            BKCR_TAMED, //
        ));

        if tamed.contains(named) {
            findings.push(format!(
                "'{}' is named in BKRR_TAMED_CRATES and BKRR_FERAL_CRATES both — every direct \
                 dependency stands in exactly one of the two, and the pair the validate door \
                 proves disjoint cannot say both of one crate",
                named, //
            ));
        }
    }

    let Some(dependencies) = account
        .bkcj_field("dependencies")
        .and_then(bkcj_Value::bkcj_array)
    else {
        return;
    };

    for dependency in dependencies {
        if !zbkcr_compiled(zbkcr_kind(dependency), true, tested) {
            continue;
        }

        let Some(named) = dependency
            .bkcj_field("name")
            .and_then(bkcj_Value::bkcj_string)
        else {
            continue;
        };

        if tamed.contains(&named) || feral.contains(&named) {
            continue;
        }

        findings.push(format!(
            "'{}' is a direct dependency of the manifest and neither BKRR_TAMED_CRATES nor \
             BKRR_FERAL_CRATES names it — every direct dependency is named in allow form, the \
             lock pinning the closure beyond them",
            named, //
        ));
    }
}

/// A path as the filesystem settles it, so a symlink cannot make one path read
/// as two.
fn zbkcr_settled(path: &Path) -> Option<PathBuf> {
    std::fs::canonicalize(path).ok()
}

/// A path made relative to the repository, both settled first so that a symlink
/// on either side cannot make one path read as two.
fn zbkcr_relative(repository: &Path, path: Option<&Path>) -> Option<String> {
    let path = path?;
    let settled = std::fs::canonicalize(path).ok()?;
    let root = std::fs::canonicalize(repository).ok()?;

    settled
        .strip_prefix(&root)
        .ok()
        .map(|rest| rest.to_string_lossy().replace('\\', "/"))
}

/// An instance directory's own name, which is the collar name.
fn zbkcr_named(instance: &Path) -> String {
    instance
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// The collar names a walk found, for a refusal that would otherwise leave its
/// reader guessing what IS there.
fn zbkcr_listed(instances: &[PathBuf]) -> String {
    let mut named: Vec<String> = instances.iter().map(|held| zbkcr_named(held)).collect();
    named.sort();
    named.join(", ")
}

// eof
