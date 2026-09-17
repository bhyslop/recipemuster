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

//! The whereabouts reader — the geography a dispatched seat is provisioned with
//! (BKSCL-Collar.adoc "The Dispatch Layer and the Whereabouts").
//!
//! THE KENNEL LEARNS NOTHING OF THE KRAAL. A dispatched seat is an ordinary
//! substrate seat and a door driven from it is an ordinary tabtarget: the
//! dispatch already hands every door one config directory, and the geography
//! stands in a file there exactly as the temp root and the tackroom reach the
//! kennel through the same channel. There is no engine reading here, no register
//! and no variable of the kennel's own.
//!
//! THE DIRECTORY IS HANDED IN, NEVER READ HERE. The library touches no
//! environment: the door face reads the substrate's config directory once and
//! passes it, the same way a repository travels. That is what lets a hurdle pose
//! a geography by composing a directory rather than by mutating the process it
//! is running in — and a mutated environment is shared by every hurdle a runner
//! drives in one process, which is a lure of the worst kind.
//!
//! ABSENT MEANS SELF, PRESENT MEANS DISPATCHED, AND THERE IS NO THIRD POSTURE.
//! An ordinary repository's moorings carries no such file, so the simple case
//! stays configuration-free: the work tree is the tree the door entered and the
//! delivered root is its own build output. A file that STANDS and does not read
//! refuses; it never falls back to self, because a fallback would turn every
//! provisioning fault into a quiet own build — the seat would look ordinary and
//! the borrow candidate the stile went to the trouble of standing up would
//! simply never be consulted.
//!
//! THE ROSTER IS CLOSED, which is the same posture the collar resolver keeps
//! toward a family file and for the same reason: the reader beside this crate
//! admits any key the catena law allows, so a field outside the roster reaches
//! here silently and means nothing to anybody. A misspelled field name that
//! parsed and was ignored is a provisioning fault that presents as correct
//! behavior, and the whole point of refusing is that the stile hears about it.

use std::path::{Path, PathBuf};

use bkl::bklrc_catena::bklrc_read;

/// The regime file a seat's geography stands in: a scalar regime, one file per
/// seat — no instance directories and no identity field, the seat itself being
/// the identity.
pub const BKCA_FILE: &str = "bkrw.env";

/// The one field of the roster at MVP: the root whose built binaries are the
/// borrow candidates of the binary election — the standing clone of the work
/// billet's own sire, stated absolutely.
///
/// THE WORK TREE IS NOT A FIELD, because it is the process's working directory
/// the trampoline established, and a second statement of one fact is a place for
/// the two to disagree. Where seat-local builds land is not a field either: no
/// door reads one, and a geography nobody honors is worse than none.
pub const BKCA_DELIVERED_DIR: &str = "BKRW_DELIVERED_DIR";

/// The whole roster, which is what makes the reading closed. A second field
/// enters here and in the sheaf's legend together, by deliberate edit.
const ZBKCA_ROSTER: &[&str] = &[BKCA_DELIVERED_DIR];

/// The proclamation's word for each geography, in the substrate's enum sprue
/// form under this kit's own plane.
///
/// They are declared rather than spelled at each site because a consumer sources
/// the proclamation and matches on the value: a door and a hurdle spelling the
/// same word twice are two things to keep in step.
pub const BKCA_GEOGRAPHY_SELF: &str = "bknre_self";
pub const BKCA_GEOGRAPHY_DISPATCHED: &str = "bknre_dispatched";

/// Where a seat stands relative to the trees it works.
///
/// GEOGRAPHY IS NEVER ELECTION. No value here names a binary choice — the
/// election stays computed per invocation over the two residences this yields,
/// so a misprovisioned directory fails loudly as a place that is not there and
/// never lies about which binary answers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum bkca_Geography {
    /// No whereabouts stands. The tree the door entered is the whole geography:
    /// one residence, which is its own.
    Undispatched,
    /// A whereabouts stands, naming the delivered root whose built binaries are
    /// the borrow candidates.
    Dispatched(PathBuf),
}

impl bkca_Geography {
    /// The delivered root, where one was provisioned.
    pub fn bkca_delivered(&self) -> Option<&Path> {
        match self {
            bkca_Geography::Undispatched => None,
            bkca_Geography::Dispatched(delivered) => Some(delivered.as_path()),
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
    pub fn bkca_stated(&self) -> String {
        match self {
            bkca_Geography::Undispatched => BKCA_GEOGRAPHY_SELF.to_string(),
            bkca_Geography::Dispatched(delivered) => {
                format!("{} {}", BKCA_GEOGRAPHY_DISPATCHED, delivered.display())
            }
        }
    }
}

/// Read the geography out of the config directory a door was handed.
///
/// `config` is `None` where the substrate exported no config directory at all,
/// which is the same answer as a directory holding no whereabouts: an
/// undispatched seat. The two are one case rather than two because they mean the
/// identical thing — nobody provisioned a geography — and a rule telling them
/// apart would branch on a fact that changes its answer not at all.
///
/// THE READING IS LAZY AND THE ELECTION'S ALONE. It is taken where a delivered
/// root is CONSUMED and never ahead of the kennel's bare self-report, which the
/// whistle and the driver gates read as a position: `Tools/bkk/bk0/bkcp_position.sh`
/// treats any non-zero exit as no position, so a geography refusal standing ahead
/// of that report would counterfeit staleness at the whistle and at the build
/// module's gate — a provisioning fault presenting as a stale binary, which is a
/// different repair aimed at a different tree.
pub fn bkca_read(config: Option<&Path>) -> Result<bkca_Geography, String> {
    let Some(config) = config else {
        return Ok(bkca_Geography::Undispatched);
    };

    let whereabouts = config.join(BKCA_FILE);

    if !whereabouts.is_file() {
        return Ok(bkca_Geography::Undispatched);
    }

    let regime = bklrc_read(&whereabouts)?;

    // THE FOREIGN FIELD IS REPORTED BEFORE THE MISSING ONE, because a file
    // carrying a misspelling carries BOTH findings and only the first says what
    // actually happened: told its roster field is missing, a reader looks for a
    // line that is right there in front of them under another name.
    for held in regime.bklrc_keys() {
        if !ZBKCA_ROSTER.contains(&held) {
            return Err(format!(
                "the whereabouts at {} declares {}, which stands outside this regime's roster — \
                 the roster is closed at {} and a field outside it is a provisioning fault rather \
                 than something to read past",
                whereabouts.display(),
                held,
                ZBKCA_ROSTER.join(", ")
            ));
        }
    }

    let Some(delivered) = regime.bklrc_scalar(BKCA_DELIVERED_DIR) else {
        return Err(format!(
            "the whereabouts at {} does not declare {} — a file that stands says where the \
             delivered root is, and a door reading a meaning out of an absence cannot tell a \
             declaration from an omission",
            whereabouts.display(),
            BKCA_DELIVERED_DIR
        ));
    };

    let delivered = delivered.trim();
    let root = Path::new(delivered);

    if !root.is_absolute() {
        return Err(format!(
            "the whereabouts at {} declares {} as '{}', which is not an absolute path — a \
             delivered root is stated absolutely because the seat that reads it stands in a \
             different tree than the one that wrote it, and a relative root would name whichever \
             directory a door happened to be standing in",
            whereabouts.display(),
            BKCA_DELIVERED_DIR,
            delivered
        ));
    }

    if !root.is_dir() {
        return Err(format!(
            "the whereabouts at {} declares {} as '{}', and no directory stands there — the \
             delivered root is where a borrow candidate's binary is looked for, so a root that is \
             not there is a seat nobody finished provisioning",
            whereabouts.display(),
            BKCA_DELIVERED_DIR,
            delivered
        ));
    }

    Ok(bkca_Geography::Dispatched(root.to_path_buf()))
}

// eof
