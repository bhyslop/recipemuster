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

//! The python family — the collar for a launchable uv builds an environment for,
//! and the reading of one.
//!
//! A PYTHON COLLAR POINTS AT A PROJECT AND RESTATES NOTHING UV OWNS. What
//! builds the environment is uv's question, answered by reference: the manifest
//! path joined from the repo root, and the two files that stand beside it. Dependencies, versions and
//! hashes live in `uv.lock` and are read by uv; the collar names none of them.
//!
//! THE VALIDATION IS OVER THE PROJECT FILES AND NEVER OVER A FORMAT OF OUR OWN.
//! The kennel's control of uv is the environment it hands it on every invocation
//! and the reading it takes of the three files a project carries; a project
//! declaring a knob the kennel already states would be two authorities over one
//! setting, and the second is the one no reader thinks to look at.
//!
//! WHAT THIS MODULE PROVES AND WHAT IT LEAVES TO UV is a line drawn once, at
//! the smallest honest reading. Exactness is the kennel's: a pin naming three
//! numeric components is a fact this module can settle from the bytes, and a
//! bare floor is a refusal it can raise. SATISFACTION is uv's, because deciding
//! whether a version answers an arbitrary PEP 440 specifier is a resolver's job
//! and a second implementation of one here would be a second answer free to
//! disagree with the converge's. What this module reads of a specifier is the
//! one unambiguous shape — a lower bound, alone — and every other shape defers
//! to uv's own refusal rather than being guessed at.
//!
//! THE FIELD ROSTER STANDS HERE AND NOT IN THE READER CRATE, on the kibble
//! family's own ground: `bkp` reads regime files and stops, and what a family's
//! fields MEAN belongs to the family that declared them.
//!
//! THE WEANED READING IS THE COLLAR RESOLVER'S, PARAMETERIZED. Two readings of
//! one set are two things to keep in step, and a family read by rules of its own
//! would let one set hold two answers about what stands in it.

use std::ffi::OsString;
use std::path::{
    Path,
    PathBuf, //
};

use crate::bkco_output::{
    bkco_call,
    BKCO_VERB_GANGLINE,
    BKCO_VERB_HEEL, //
};
use crate::bkcq_kibble::bkcq_resolve;
use crate::bkcr_resolve::{
    bkcr_drift,
    bkcr_weaned_family,
    BKCR_KIND_APP,
    BKCR_KIND_SUITE,
    BKCR_TARGET_MANIFEST, //
};
use crate::bkg_breviary::{
    BKG_BAKED_BASE,
    BKG_CRUDE_BASE, //
};
use bkp::bkprc_catena::{
    bkprc_Regime,
    bkprc_read, //
};
use toml::de::{
    DeTable,
    DeValue, //
};
use toml::Spanned;

/// The python family's file. Its parent directory is the instance and that
/// directory's name is the collar name.
pub const BKCX_FAMILY: &str = "bkrp.env";

/// The fields every python collar declares, whatever its kind.
///
/// THE ROSTER IS THE TENANT-BLIND CORE AND THE LAUNCH COORDINATES, AND NOTHING
/// MORE. The rust family's `BKRR_FEATURES` and `BKRR_PROFILE` declare a BUILD
/// SHAPE, and a python launchable has no build to shape: what the environment is
/// made of is the lock, which uv owns whole. `BKRR_TAMED_CRATES` and its feral
/// sibling have their counterpart in the two list fields below, which part the
/// direct dependencies by how they ARRIVE rather than by what their build
/// reaches, the fact this family partitions being its own (`zbkcx_partition`).
/// `BKRR_MUZZLE` and `BKRR_EXERGUE` name a linter
/// list and a generated source, neither of which stands in this tenant: the
/// python guide's linter election is deliberately deferred
/// (`guides/vok/PCG_PythonCodingGuide.md`), and nothing generates python source
/// here.
pub const BKCX_FIELDS_BOTH: &[&str] = &[
    "BKRP_COLLAR",
    "BKRP_KIND",
    "BKRP_MANIFEST",
    "BKRP_ROOTS",
    "BKRP_SPEND",
    "BKRP_BAKED_PACKAGES",
    "BKRP_CRUDE_PACKAGES", //
];

/// The one value the second list field admits: every direct dependency arrives
/// made.
///
/// THE RUST PAIR'S SHAPE WITH THE RUST PAIR'S REASON MOVED. A crate is compiled
/// wherever it is built, so the rust lists part a build that fetches from one
/// that does not; a python dependency mostly arrives as a wheel the lock
/// hashes, and is built only where no wheel answers, so these part arriving
/// made from being made here. The second list admits this one value, as the
/// feral list admits `bknre_tamed`, so that the question is asked at every
/// collar and a reopening changes what the reading does with a name rather
/// than the collar's shape (BKr_hpt).
pub const BKCX_BAKED: &str = "bknre_baked";

/// The fields an app declares and a suite does not.
///
/// ONE FIELD WHERE THE RUST FAMILY CARRIES TWO, and the parting is the store's
/// rather than a simplification. A rust app's residence is a directory the
/// collar must name because a binary stands in one of several — the build output
/// at a source seat, the emplaced home at a delivered one. A python app's entry
/// point stands in exactly one place, the venv the kennel itself sited under the
/// loosebox, so a residence field would ask a collar to declare a fact the
/// kennel already decided. What is left to declare is the name the entry point
/// answers to there, which is `[project.scripts]`' own key.
pub const BKCX_FIELDS_APP: &[&str] = &["BKRP_BYNAME"];

/// The fields a suite declares and an app does not.
pub const BKCX_FIELDS_SUITE: &[&str] = &[
    "BKRP_TARGET",
    "BKRP_RUNNER",
    "BKRP_TONGUE", //
];

/// The runner value for this family, in the substrate's enum sprue form under
/// this kit's own plane.
///
/// ONE VALUE VOICED BY TWO FIELDS, which is the sprue convention's own
/// family-sharing rather than an accident: `bknre_nextest` already stands as
/// both a runner and a tongue, and uniqueness lives in the value with its owning
/// variable rather than in the plane. pytest is the command spawned and pytest's
/// is the output shape recognized, so one spelling answers both honestly.
pub const BKCX_RUNNER_PYTEST: &str = "bknre_pytest";

/// The tongue value for this family. The same spelling as the runner, on the
/// reasoning stated above it.
pub const BKCX_TONGUE_PYTEST: &str = "bknre_pytest";

/// The enum-valued fields and the values each admits.
///
/// The kind and the spend values are the rust family's own constants read from
/// here rather than respelled: they are the collar genus's, not any tenant's,
/// and a second spelling would let one tenant's roster drift from the other's.
const ZBKCX_ENUMS: &[(&str, &[&str])] = &[
    (
        "BKRP_KIND",
        &[
            BKCR_KIND_APP,
            BKCR_KIND_SUITE, //
        ],
    ),
    (
        "BKRP_SPEND",
        &[
            "bknre_reader",
            "bknre_writer", //
        ],
    ),
    ("BKRP_RUNNER", &[BKCX_RUNNER_PYTEST]),
    ("BKRP_TONGUE", &[BKCX_TONGUE_PYTEST]), //
];

/// The three files a uv project carries, all of which stand or the project is
/// not one.
///
/// THE MANIFEST IS THE ONE THE COLLAR NAMES and the other two are its siblings,
/// which is why only one is a field. uv fixes their names and their placement,
/// so a collar naming all three would be restating uv's own layout — and would
/// admit a project whose three files stood in three directories, which uv would
/// then refuse at the converge for reasons the collar had made harder to read.
pub const BKCX_MANIFEST_FILE: &str = "pyproject.toml";
pub const BKCX_LOCK_FILE: &str = "uv.lock";
pub const BKCX_PIN_FILE: &str = ".python-version";

/// The kibble whose pin a declared `required-version` must admit. The kennel
/// spawns uv from its own residence and from nowhere else, so the version a
/// project may demand is the one that declaration carries.
pub const BKCX_UV: &str = "bki_uv";

/// The knobs the kennel states in the environment on every uv invocation, as
/// uv's own `[tool.uv]` table spells the ones it spells at all.
///
/// THE ROSTER IS THE PADDOCK'S SETTLED LIST AND NOT A GUESS AT UV'S SCHEMA. The
/// kennel states the venv path, the interpreter store, the wheel and archive
/// caches, the interpreter preference, the download posture, the bin directory,
/// the path and env-file postures, the locked sync and the offline posture. Most
/// of those reach uv through a variable alone and have NO table key at all, so
/// they cannot be carried by a project and are absent here rather than spelled
/// speculatively; what stands below is exactly the intersection of that list
/// with the keys a `pyproject.toml` can hold.
///
/// `index-strategy` is deliberately NOT here though the kennel states it: it
/// carries a refusal of its own below, beside the index rules it belongs with,
/// and a key in both rosters would be reported twice for one condition.
const ZBKCX_KENNEL_KNOBS: &[&str] = &[
    "cache-dir",
    "python-preference",
    "python-downloads", //
];

/// uv's own key for the strategy the kennel states, refused beside the index
/// rules rather than among the knobs above.
const ZBKCX_INDEX_STRATEGY: &str = "index-strategy";

/// uv's own keys for the build bar and its inverse, refused in a project's
/// table because the kennel states the bar itself.
///
/// FOUR KEYS AND ONE REASON. The kennel bars every dependency's build (BKr_hpt)
/// and states that bar in the environment, where it beats a project's table —
/// benched: the environment wins over a table saying the bar is off — but a key
/// stated in both places is the two-authorities failure the knob roster above
/// refuses, and the two inverse keys would demand the very builds the bar
/// refuses. The refusal here is for the reader, not the resolver.
const ZBKCX_BUILD_KNOBS: &[&str] = &[
    "no-build",
    "no-build-package",
    "no-binary",
    "no-binary-package", //
];

/// uv's own key for a flat index — a directory of files — and the index format
/// word that declares one the modern way.
///
/// REFUSED BECAUSE A FLAT INDEX PROVES NO COVERAGE. Over a flat index uv
/// enforces a declared environment only for a package carrying no sdist, even
/// under the bar (benched on uv 0.11.30, both spellings), so a project served
/// from one has its missing wheel found at the converge of the station that
/// lacks it and never at the lock. A Palisade membrane, retired when uv
/// enforces the environment over a flat index as it does over a simple one.
const ZBKCX_FIND_LINKS: &str = "find-links";
const ZBKCX_INDEX_FORMAT: &str = "format";
const ZBKCX_FORMAT_FLAT: &str = "flat";

/// uv's own keys for a source the converge would have to build — a sibling
/// project by path, a workspace member, a git checkout — and the tables they
/// stand in.
///
/// ONE PROJECT STANDS PER ENVIRONMENT. Each of these arrives as source the lock
/// does not hash and is built on the station at install, so it is refused at
/// the converge by uv's sentence under the bar and here by the kennel's, ahead
/// of any spawn.
///
/// ONE PATH SOURCE IS ADMITTED, TRUSTED BY POSITION as the rust family's tamed
/// local crate is: a sibling project inside the same repository, installed
/// editable, on uv's own backend. Its bytes are the tree's own, committed under
/// the door law, and its build is the project's own build performed once more
/// by the backend the pinned uv carries. A copy uv installs from a directory is
/// refused still, since uv reinstalls one only when its declaration changes,
/// so an edit to its source would stand unseen in the environment.
const ZBKCX_BUILT_SOURCES: &[&str] = &[
    "path",
    "workspace",
    "git", //
];
const ZBKCX_SOURCE_PATH: &str = "path";
const ZBKCX_SOURCE_EDITABLE: &str = "editable";
const ZBKCX_SOURCES: &str = "sources";
const ZBKCX_WORKSPACE: &str = "workspace";

/// The manifest format's own keys for metadata computed at build time and for
/// the build backend, and uv's name for its own backend.
///
/// A PROJECT WHOSE METADATA MUST BE BUILT TO BE READ cannot be locked under the
/// bar at all — uv refuses that build like any other — so every field stands
/// static. And THE PROJECT'S OWN BUILD IS THE ONE BUILD THE CONVERGE PERFORMS,
/// its requirements standing in no lock: a backend other than uv's own would be
/// fetched unpinned from an index the lock records nothing of, so the backend
/// is uv's, in a range the kibble's uv satisfies, which fetches nothing.
const ZBKCX_DYNAMIC: &str = "dynamic";
const ZBKCX_BUILD_SYSTEM: &str = "build-system";
const ZBKCX_BUILD_BACKEND: &str = "build-backend";
const ZBKCX_BUILD_REQUIRES: &str = "requires";
const ZBKCX_UV_BACKEND: &str = "uv_build";

/// uv's own keys inside its lock, read for the project's own entry, its account
/// of the manifest, and which packages carry a wheel.
///
/// THE OWN KEYS NAME A DIRECTORY, AND THE DIRECTORY PARTS THE TWO TREE-OWN
/// ENTRIES. The project's own entry stands at the lock's root, `.`; an editable
/// sibling wears the same key at its own directory, and is no second entry for
/// the project.
const ZBKCX_LOCK_PACKAGE: &str = "package";
const ZBKCX_LOCK_SOURCE: &str = "source";
const ZBKCX_LOCK_OWN: &[&str] = &[
    "editable",
    "virtual", //
];
const ZBKCX_LOCK_ROOT: &str = ".";
const ZBKCX_LOCK_WHEELS: &str = "wheels";
const ZBKCX_LOCK_METADATA: &str = "metadata";
const ZBKCX_LOCK_REQUIRES_DIST: &str = "requires-dist";
const ZBKCX_LOCK_REQUIRES_DEV: &str = "requires-dev";

/// A python collar as it stands, with the regime it declared.
#[derive(Debug, Clone)]
pub struct bkcx_Collar {
    /// The collar name, which is the instance directory's own name.
    pub name: String,
    /// The instance directory, repo-relative.
    pub instance: PathBuf,
    /// Everything the family file declared.
    pub regime: bkprc_Regime,
}

impl bkcx_Collar {
    /// One field's value, or the empty string where the field is absent. The
    /// absence is a finding of its own, so a reader here is never guessing.
    pub fn bkcx_field(&self, key: &str) -> &str {
        self.regime.bkprc_scalar(key).unwrap_or("")
    }

    /// Whether this collar declares an app.
    pub fn bkcx_app(&self) -> bool {
        self.bkcx_field("BKRP_KIND") == BKCR_KIND_APP
    }

    /// The project directory this collar names, repo-relative — the manifest's
    /// own parent, which is where the other two files stand.
    pub fn bkcx_project(&self) -> PathBuf {
        Path::new(self.bkcx_field("BKRP_MANIFEST"))
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_default()
    }
}

/// A python collar and every conformance finding against it.
///
/// AN ABSENT LOCK IS A FINDING LIKE ANY OTHER, AND IT STANDS LAST. Every door
/// refuses on `findings` whole, so a door that never learned of a first lock
/// refuses it exactly as it refuses anything else. The two doors whose acts are
/// the repair — the lock-authoring door, and the converge whose install step
/// must precede it on a cold store — read past it through
/// `bkcx_findings_but_lock` and through nothing else.
#[derive(Debug, Clone)]
pub struct bkcx_Resolved {
    pub collar: bkcx_Collar,
    pub findings: Vec<String>,
    /// Whether the project stands whole save its lock, in which case the lock's
    /// own finding is the last of `findings`.
    pub lockless: bool,
}

impl bkcx_Resolved {
    /// Every finding but the absent lock's: what bars a door whose own act
    /// writes the project's first lock, or must precede it.
    pub fn bkcx_findings_but_lock(&self) -> &[String] {
        let standing = self.findings.len() - usize::from(self.lockless);

        &self.findings[..standing]
    }
}

/// Every python collar instance the weaned set holds, repo-relative where the set stands
/// inside the repository, and sorted — the collar resolver's reading,
/// parameterized, and not a second one.
pub fn bkcx_walk(repository: &Path, weaned: &Path) -> Result<Vec<PathBuf>, String> {
    bkcr_weaned_family(repository, weaned, BKCX_FAMILY)
}

/// Resolve a python collar by name, and gather every finding against it.
///
/// THE INSTANCE IS PROVEN AGAINST ITS BROOD BEFORE ANYTHING ELSE IS READ, on the
/// collar resolver's own ground: the findings of a copy that is not the kit's
/// declaration would be findings about a declaration nobody made.
pub fn bkcx_resolve(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    name: &str,
) -> Result<bkcx_Resolved, String> {
    let instances = bkcx_walk(repository, dispatch.bkcr_weaned())?;

    let Some(instance) = instances
        .iter()
        .find(|instance| zbkcx_named(instance) == name)
        .cloned()
    else {
        return Err(format!(
            "no python collar named '{}' stands in the weaned set {} — it holds {} python collar(s): {}",
            name,
            dispatch.bkcr_weaned().display(),
            instances.len(),
            zbkcx_listed(&instances), //
        ));
    };

    bkcr_drift(repository, &repository.join(&instance), BKCX_FAMILY)?;

    let regime = bkprc_read(&repository.join(&instance).join(BKCX_FAMILY))?;

    let collar = bkcx_Collar {
        name: name.to_string(),
        instance,
        regime,
    };

    let findings = bkcx_findings(repository, dispatch, &collar);
    let lockless = zbkcx_lockless(repository, &collar);

    Ok(bkcx_Resolved {
        collar,
        findings,
        lockless,
    })
}

/// Every conformance finding against one python collar, gathered whole.
///
/// EVERY FINDING IS REPORTED BEFORE ANYTHING REFUSES, which is the rust
/// resolver's discipline held over this family: a first-finding exit hands its
/// reader one repair at a time and a fresh drive between each.
pub fn bkcx_findings(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    collar: &bkcx_Collar,
) -> Vec<String> {
    let mut findings: Vec<String> = Vec::new();

    zbkcx_declared(collar, &mut findings);
    zbkcx_identity(collar, &mut findings);
    zbkcx_project(repository, dispatch, collar, &mut findings);

    // THE ABSENT LOCK STANDS LAST, which is the order the doors reading past it
    // depend on (`bkcx_Resolved`).
    if zbkcx_lockless(repository, collar) {
        findings.push(format!(
            "the project at '{}' carries no {} — a project's first lock is written by the \
             lock-authoring door, and nothing converges or launches until one stands: {}",
            collar.bkcx_project().display(),
            BKCX_LOCK_FILE,
            bkco_call(
                dispatch.bkcr_tabtargets(),
                BKCO_VERB_GANGLINE,
                Some(&collar.name)
            ), //
        ));
    }

    findings
}

/// Whether the project stands whole save its lock: the manifest the collar
/// names and the pin beside it stand, and the lock does not.
///
/// A COLLAR NAMING NO MANIFEST IS NEVER LOCKLESS, whatever stands beside it. Its
/// own declaration's finding bars it, and the lock-authoring door has no project
/// to author over.
fn zbkcx_lockless(repository: &Path, collar: &bkcx_Collar) -> bool {
    if !collar
        .bkcx_field("BKRP_MANIFEST")
        .ends_with(BKCX_MANIFEST_FILE)
    {
        return false;
    }

    let project = repository.join(collar.bkcx_project());

    project.join(BKCX_MANIFEST_FILE).is_file()
        && project.join(BKCX_PIN_FILE).is_file()
        && !project.join(BKCX_LOCK_FILE).is_file()
}

/// Which fields must stand, which must not, and which values each enum admits.
fn zbkcx_declared(collar: &bkcx_Collar, findings: &mut Vec<String>) {
    let kind = collar.bkcx_field("BKRP_KIND");

    let (owed, foreign, elsewhere): (&[&str], &[&str], &str) = match kind {
        BKCR_KIND_APP => (BKCX_FIELDS_APP, BKCX_FIELDS_SUITE, "suite"),
        BKCR_KIND_SUITE => (BKCX_FIELDS_SUITE, BKCX_FIELDS_APP, "app"),
        _ => (&[], &[], ""),
    };

    for field in BKCX_FIELDS_BOTH.iter().chain(owed.iter()) {
        if !collar.regime.bkprc_holds(field) {
            findings.push(format!(
                "{} is not declared — every python collar of this kind declares it, and a door \
                 reading a meaning out of an absence cannot tell a declaration from an omission",
                field, //
            ));
        }
    }

    for field in foreign {
        if collar.regime.bkprc_holds(field) {
            findings.push(format!(
                "{} is declared and belongs to the {} kind — this collar declares {} = {}",
                field,
                elsewhere,
                "BKRP_KIND",
                kind, //
            ));
        }
    }

    for (field, admitted) in ZBKCX_ENUMS {
        let Some(held) = collar.regime.bkprc_scalar(field) else {
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

    for field in ["BKRP_BYNAME", "BKRP_TARGET"] {
        if collar.regime.bkprc_holds(field) && collar.bkcx_field(field).trim().is_empty() {
            findings.push(format!(
                "{} stands vacant — the launch coordinate is composed from it, so a vacant one \
                 names nothing for a door to run",
                field, //
            ));
        }
    }
}

/// The identity law: the instance directory's name and the declared name are one.
fn zbkcx_identity(collar: &bkcx_Collar, findings: &mut Vec<String>) {
    let declared = collar.bkcx_field("BKRP_COLLAR");

    if declared != collar.name {
        findings.push(format!(
            "BKRP_COLLAR declares '{}' and the instance directory is named '{}' — a collar's name \
             is its identity, and the two must be one",
            declared,
            collar.name, //
        ));
    }
}

/// The reading over the project the collar names — the whole of what the
/// settling sitting listed, in the order a reader meets it.
///
/// THE DOCUMENT IS PARSED ONCE AND NEVER ESCAPES THIS BODY. The parse borrows
/// from the text it read, which is exactly the shape that keeps the reading
/// honest: the checks below are handed the table, and nothing downstream holds
/// a document after the bytes it came from are gone.
fn zbkcx_project(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    collar: &bkcx_Collar,
    findings: &mut Vec<String>,
) {
    let declared = collar.bkcx_field("BKRP_MANIFEST");

    if declared.is_empty() {
        return;
    }

    if !declared.ends_with(BKCX_MANIFEST_FILE) {
        findings.push(format!(
            "BKRP_MANIFEST names '{}', which is no {} — a python collar points at the project's \
             own manifest as a rust collar points at its Cargo.toml",
            declared,
            BKCX_MANIFEST_FILE, //
        ));
        return;
    }

    let project = repository.join(collar.bkcx_project());

    // THE THREE FILES ARE REPORTED TOGETHER AND NOT ONE AT A TIME. A project
    // half-composed is the ordinary shape of this failure — a lock authored and
    // never committed, a pin file nobody wrote — and naming the first absent one
    // would hand its reader one repair and a fresh drive between each.
    let mut absent: Vec<&str> = Vec::new();
    for file in [BKCX_MANIFEST_FILE, BKCX_LOCK_FILE, BKCX_PIN_FILE] {
        if !project.join(file).is_file() {
            absent.push(file);
        }
    }

    // A LOCK ABSENT ALONE IS NO HALF-COMPOSED PROJECT but one awaiting its first
    // lock, so the reading below runs whole over it: the lock-authoring door has
    // to meet every other finding before it writes. The lock's own finding is
    // raised last, by the caller.
    if absent == [BKCX_LOCK_FILE] {
        absent.clear();
    }

    if !absent.is_empty() {
        findings.push(format!(
            "the project at '{}' carries no {} — a uv project is these three files standing \
             together ({}, {}, {}), and the environment is declared by all of them or by none",
            collar.bkcx_project().display(),
            absent.join(", "),
            BKCX_MANIFEST_FILE,
            BKCX_LOCK_FILE,
            BKCX_PIN_FILE, //
        ));
        return;
    }

    let read = match std::fs::read_to_string(project.join(BKCX_MANIFEST_FILE)) {
        Ok(read) => read,
        Err(err) => {
            findings.push(format!(
                "the project's {} could not be read: {}",
                BKCX_MANIFEST_FILE,
                err, //
            ));
            return;
        }
    };

    let parsed = match DeTable::parse(&read) {
        Ok(parsed) => parsed,
        Err(err) => {
            findings.push(format!(
                "the project's {} could not be read as toml: {}",
                BKCX_MANIFEST_FILE,
                err, //
            ));
            return;
        }
    };

    let manifest = parsed.get_ref();

    let pin = match std::fs::read_to_string(project.join(BKCX_PIN_FILE)) {
        Ok(read) => read.trim().to_string(),
        Err(err) => {
            findings.push(format!(
                "the project's {} could not be read: {}",
                BKCX_PIN_FILE,
                err, //
            ));
            return;
        }
    };

    zbkcx_pin(&pin, manifest, findings);
    zbkcx_uv_table(manifest, findings);
    zbkcx_indexes(manifest, findings);
    zbkcx_required(repository, dispatch, manifest, findings);
    zbkcx_build_knobs(manifest, findings);
    zbkcx_flat(manifest, findings);
    zbkcx_built_sources(repository, dispatch, &project, manifest, findings);
    zbkcx_dynamic(manifest, findings);
    zbkcx_backend(repository, dispatch, manifest, findings);
    zbkcx_partition(&project, collar, findings);
}

/// The pin names an exact patch, and does not fall below what the manifest
/// requires.
///
/// EXACTNESS IS THE KENNEL'S AND SATISFACTION IS UV'S, which is the line this
/// module's head draws. Three numeric components is a fact settled from the
/// bytes; whether a version answers an arbitrary PEP 440 specifier is a
/// resolver's judgment, and the one specifier shape read below is the
/// unambiguous one — a lower bound standing alone. Every other shape is left to
/// uv's own refusal at the converge rather than guessed at here, because a
/// second resolver would be free to disagree with the first and the disagreement
/// would surface as a project that validated clean and would not sync.
fn zbkcx_pin(pin: &str, manifest: &DeTable<'_>, findings: &mut Vec<String>) {
    let Some(components) = zbkcx_version(pin) else {
        findings.push(format!(
            "{} carries '{}', which names no version at all — the pin is the exact interpreter the \
             environment is built on, and it is read as three numeric components",
            BKCX_PIN_FILE, pin, //
        ));
        return;
    };

    if components.len() != 3 {
        findings.push(format!(
            "{} carries '{}', which is a {}-component floor and not an exact patch — a bare minor \
             version admits whichever patch the station happens to hold, which is the drift the \
             three-layer chain exists to close",
            BKCX_PIN_FILE,
            pin,
            components.len(), //
        ));
        return;
    }

    let Some(requirement) =
        zbkcx_sub(manifest, "project").and_then(|project| zbkcx_text(project, "requires-python"))
    else {
        return;
    };

    let Some(floor) = zbkcx_floor(requirement) else {
        // A specifier this reading does not settle is not a finding: uv reads
        // the whole dialect and refuses at the converge, and a guess here would
        // be a second answer free to disagree with that one.
        return;
    };

    if zbkcx_below(&components, &floor) {
        findings.push(format!(
            "{} carries '{}' and the manifest requires '{}' — the pin falls below the project's \
             own floor, so the environment the pin builds is one the project declares it does not \
             run on",
            BKCX_PIN_FILE,
            pin,
            requirement, //
        ));
    }
}

/// The project's uv table carries none of the knobs the kennel states.
///
/// TWO AUTHORITIES OVER ONE SETTING IS THE FAILURE, and the second is the one no
/// reader thinks to look at: the kennel states these on every invocation, so a
/// project restating one is either agreeing redundantly or disagreeing silently,
/// and nothing downstream can tell which.
fn zbkcx_uv_table(manifest: &DeTable<'_>, findings: &mut Vec<String>) {
    let Some(uv) = zbkcx_uv(manifest) else {
        return;
    };

    for knob in ZBKCX_KENNEL_KNOBS {
        if uv.contains_key(*knob) {
            findings.push(format!(
                "[tool.uv] carries '{}', which the kennel states in the environment on every uv \
                 invocation — the project's table carries only what is the project's own, a \
                 setting declared in both places being one no reader can say which of the two won",
                knob, //
            ));
        }
    }
}

/// A declared non-default index is explicit and has a matching sources entry,
/// and the index strategy is absent.
///
/// AN IMPLICIT NON-DEFAULT INDEX IS THE SUPPLY-CHAIN SHAPE THIS REFUSES. An
/// index uv may consult for any package it likes is an index that can answer for
/// a package the default index also carries; marking it explicit and naming the
/// packages that come from it in `[tool.uv.sources]` is what turns "somewhere in
/// this set" into "this package, from this index". The strategy key is refused
/// beside them because it is the other half of the same reach: it widens which
/// indexes are consulted for a package already found.
fn zbkcx_indexes(manifest: &DeTable<'_>, findings: &mut Vec<String>) {
    let Some(uv) = zbkcx_uv(manifest) else {
        return;
    };

    if uv.contains_key(ZBKCX_INDEX_STRATEGY) {
        findings.push(format!(
            "[tool.uv] carries '{}' — the kennel states the strategy, and a project widening which \
             indexes answer for a package it already located is the reach the explicit-index rule \
             exists to close",
            ZBKCX_INDEX_STRATEGY, //
        ));
    }

    let Some(indexes) = zbkcx_at(uv, "index").and_then(DeValue::as_array) else {
        return;
    };

    // Which index names the sources table actually draws from. Gathered once:
    // an index is answered for by any source entry naming it, and asking per
    // index would walk the table once per declaration.
    let drawn: Vec<&str> = zbkcx_sub(uv, "sources")
        .map(|sources| {
            sources
                .values()
                .map(Spanned::get_ref)
                .filter_map(DeValue::as_table)
                .filter_map(|source| zbkcx_text(source, "index"))
                .collect()
        })
        .unwrap_or_default();

    for declared in indexes.iter() {
        let Some(index) = declared.get_ref().as_table() else {
            continue;
        };

        if zbkcx_flag(index, "default").unwrap_or(false) {
            continue;
        }

        let named = zbkcx_text(index, "name").unwrap_or("");

        if named.is_empty() {
            findings.push(
                "[[tool.uv.index]] declares a non-default index carrying no name — an index \
                 nothing can name is one no source entry can draw from, so it is reachable only by \
                 the implicit search the explicit rule forbids"
                    .to_string(),
            );
            continue;
        }

        if !zbkcx_flag(index, "explicit").unwrap_or(false) {
            findings.push(format!(
                "the index '{}' is non-default and not explicit — an implicit index is consulted \
                 for any package at all, including one the default index already answers for",
                named, //
            ));
        }

        if !drawn.contains(&named) {
            findings.push(format!(
                "the index '{}' is declared and no [tool.uv.sources] entry draws from it — an \
                 explicit index nothing names is an index nothing is fetched from, and the \
                 declaration is either stale or the sources entry was forgotten",
                named, //
            ));
        }
    }
}

/// A declared `required-version` admits the pin the uv kibble carries.
///
/// THE KENNEL SPAWNS UV FROM ITS OWN RESIDENCE AND FROM NOWHERE ELSE, so a
/// project demanding a uv the kibble does not declare is a project that cannot
/// be converged on this station whatever else is true of it — and it is worth
/// saying at validation rather than minutes into a converge.
fn zbkcx_required(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    manifest: &DeTable<'_>,
    findings: &mut Vec<String>,
) {
    let Some(uv) = zbkcx_uv(manifest) else {
        return;
    };

    let Some(requirement) = zbkcx_text(uv, "required-version") else {
        return;
    };

    let kibble = match bkcq_resolve(repository, dispatch.bkcr_weaned(), BKCX_UV) {
        Ok(resolved) => resolved.kibble,
        Err(err) => {
            findings.push(format!(
                "[tool.uv] requires uv '{}' and the kibble that pins uv could not be read: {}",
                requirement,
                err, //
            ));
            return;
        }
    };

    let pinned = kibble.bkcq_field("BKRK_VERSION").to_string();

    let (Some(components), Some(floor)) = (zbkcx_version(&pinned), zbkcx_floor(requirement)) else {
        // As with the interpreter pin: a specifier shape this reading does not
        // settle defers to uv's own refusal rather than being guessed at.
        return;
    };

    if zbkcx_below(&components, &floor) {
        findings.push(format!(
            "[tool.uv] requires uv '{}' and the kibble '{}' pins {} — the kennel spawns uv from \
             its own residence and from nowhere else, so a project requiring a version the kibble \
             does not carry cannot be converged here",
            requirement,
            BKCX_UV,
            pinned, //
        ));
    }
}

/// The project's uv table carries none of the build bar's keys.
fn zbkcx_build_knobs(manifest: &DeTable<'_>, findings: &mut Vec<String>) {
    let Some(uv) = zbkcx_uv(manifest) else {
        return;
    };

    for knob in ZBKCX_BUILD_KNOBS {
        if uv.contains_key(*knob) {
            findings.push(format!(
                "[tool.uv] carries '{}' — the kennel bars every dependency's build and states the \
                 bar in the environment on every uv invocation (BKr_hpt), so a key that lifts or \
                 demands a build is the kennel's to state and never a project's",
                knob, //
            ));
        }
    }
}

/// No flat index, in either of uv's spellings.
fn zbkcx_flat(manifest: &DeTable<'_>, findings: &mut Vec<String>) {
    let Some(uv) = zbkcx_uv(manifest) else {
        return;
    };

    if uv.contains_key(ZBKCX_FIND_LINKS) {
        findings.push(format!(
            "[tool.uv] carries '{}' — over a flat index uv proves no declared environment for a \
             package carrying an sdist, even under the build bar, so a missing wheel is found at \
             the converge that lacks it and never at the lock; serve the files from a simple index",
            ZBKCX_FIND_LINKS, //
        ));
    }

    let Some(indexes) = zbkcx_at(uv, "index").and_then(DeValue::as_array) else {
        return;
    };

    for declared in indexes.iter() {
        let Some(index) = declared.get_ref().as_table() else {
            continue;
        };

        if zbkcx_text(index, ZBKCX_INDEX_FORMAT) == Some(ZBKCX_FORMAT_FLAT) {
            findings.push(format!(
                "the index '{}' is declared {} — over a flat index uv proves no declared \
                 environment for a package carrying an sdist, even under the build bar, so a \
                 missing wheel is found at the converge that lacks it and never at the lock; serve \
                 the files from a simple index",
                zbkcx_text(index, "name").unwrap_or(""),
                ZBKCX_FORMAT_FLAT, //
            ));
        }
    }
}

/// No source the converge would have to build, and no workspace, save an
/// editable sibling inside the repository.
fn zbkcx_built_sources(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    project: &Path,
    manifest: &DeTable<'_>,
    findings: &mut Vec<String>,
) {
    let Some(uv) = zbkcx_uv(manifest) else {
        return;
    };

    if uv.contains_key(ZBKCX_WORKSPACE) {
        findings.push(format!(
            "[tool.uv.{}] is declared — one project stands per environment, and a workspace \
             member is built on the station from source the lock does not hash, which is a {} \
             package the converge never builds (BKr_hpt)",
            ZBKCX_WORKSPACE,
            BKG_CRUDE_BASE, //
        ));
    }

    let Some(sources) = zbkcx_sub(uv, ZBKCX_SOURCES) else {
        return;
    };

    for (named, declared) in sources.iter() {
        // A source is one table or a list of them, uv admitting both shapes.
        let value = declared.get_ref();
        let mut tables: Vec<&DeTable<'_>> = Vec::new();
        if let Some(table) = value.as_table() {
            tables.push(table);
        }
        if let Some(list) = value.as_array() {
            tables.extend(list.iter().filter_map(|held| held.get_ref().as_table()));
        }

        for table in tables {
            for key in ZBKCX_BUILT_SOURCES {
                if !table.contains_key(*key) {
                    continue;
                }

                if *key == ZBKCX_SOURCE_PATH {
                    zbkcx_sibling(
                        repository,
                        dispatch,
                        project,
                        named.get_ref(),
                        table,
                        findings,
                    );
                    continue;
                }

                findings.push(format!(
                    "[tool.uv.{}] draws '{}' from a {} source — a dependency built on the \
                     station from source the lock does not hash is a {} package, which the \
                     converge never builds (BKr_hpt); one project stands per environment, and \
                     the one source by path admitted is a sibling inside the repository, \
                     installed editable",
                    ZBKCX_SOURCES,
                    named.get_ref(),
                    key,
                    BKG_CRUDE_BASE, //
                ));
            }
        }
    }
}

/// A source by path stands as a sibling of the tree's own: editable, a
/// directory inside the repository holding a project of its own, and that
/// project on uv's own backend in the range the project itself is held to.
///
/// THE REPOSITORY BOUND IS READ ON SETTLED PATHS, so neither a symlink nor a
/// `..` can make a directory outside the tree read as one inside it; a path
/// that settles to nothing is outside by the same reading.
fn zbkcx_sibling(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    project: &Path,
    named: &str,
    table: &DeTable<'_>,
    findings: &mut Vec<String>,
) {
    let declared = zbkcx_text(table, ZBKCX_SOURCE_PATH).unwrap_or("");

    if zbkcx_flag(table, ZBKCX_SOURCE_EDITABLE) != Some(true) {
        findings.push(format!(
            "[tool.uv.{}] draws '{}' from a path source not installed editable — uv installs \
             such a directory as a copy built on the station and reinstalls it only when its \
             declaration changes, a {} package the converge never builds (BKr_hpt); a sibling \
             inside the repository is declared with {} = true",
            ZBKCX_SOURCES,
            named,
            BKG_CRUDE_BASE,
            ZBKCX_SOURCE_EDITABLE, //
        ));
        return;
    }

    let settled = std::fs::canonicalize(project.join(declared)).ok();
    let root = std::fs::canonicalize(repository).ok();

    let Some(sibling) =
        settled.filter(|held| root.as_ref().is_some_and(|root| held.starts_with(root)))
    else {
        findings.push(format!(
            "[tool.uv.{}] draws '{}' from the path '{}', which settles to no directory inside \
             the repository — a source by path is trusted by position or not at all, and \
             anything outside the tree is a {} package the converge never builds (BKr_hpt)",
            ZBKCX_SOURCES,
            named,
            declared,
            BKG_CRUDE_BASE, //
        ));
        return;
    };

    let read =
        match std::fs::read_to_string(sibling.join(BKCX_MANIFEST_FILE)) {
            Ok(read) => read,
            Err(err) => {
                findings.push(format!(
                "the sibling '{}' at '{}' carries no readable {}: {} — a source by path names a \
                 project of its own",
                named, declared, BKCX_MANIFEST_FILE, err, //
            ));
                return;
            }
        };

    let parsed = match DeTable::parse(&read) {
        Ok(parsed) => parsed,
        Err(err) => {
            findings.push(format!(
                "the sibling '{}' at '{}' carries a {} that could not be read as toml: {}",
                named,
                declared,
                BKCX_MANIFEST_FILE,
                err, //
            ));
            return;
        }
    };

    let manifest = parsed.get_ref();

    // THE PROJECT MAY DECLARE NO BUILD AT ALL AND THE SIBLING MAY NOT: an
    // editable install is a build, and a manifest naming no backend has uv reach
    // for a default one it fetches.
    if zbkcx_sub(manifest, ZBKCX_BUILD_SYSTEM).is_none() {
        findings.push(format!(
            "the sibling '{}' at '{}' declares no [{}] — an editable sibling is built by the \
             converge as the project is, so it names uv's own backend ({})",
            named,
            declared,
            ZBKCX_BUILD_SYSTEM,
            ZBKCX_UV_BACKEND, //
        ));
        return;
    }

    let mut held: Vec<String> = Vec::new();
    zbkcx_backend(repository, dispatch, manifest, &mut held);

    for finding in held {
        findings.push(format!(
            "the sibling '{}' at '{}': {}",
            named,
            declared,
            finding, //
        ));
    }
}

/// No dynamic metadata.
fn zbkcx_dynamic(manifest: &DeTable<'_>, findings: &mut Vec<String>) {
    let Some(project) = zbkcx_sub(manifest, "project") else {
        return;
    };

    let Some(dynamic) = zbkcx_at(project, ZBKCX_DYNAMIC).and_then(DeValue::as_array) else {
        return;
    };

    let named: Vec<&str> = dynamic
        .iter()
        .filter_map(|held| held.get_ref().as_str())
        .collect();

    if named.is_empty() {
        return;
    }

    findings.push(format!(
        "[project] declares {} = [{}] — a project whose metadata must be built to be read cannot \
         be locked under the build bar, so every field stands static in the manifest",
        ZBKCX_DYNAMIC,
        named.join(", "), //
    ));
}

/// The project's own build, where it declares one, runs on uv's own backend in a
/// range the kibble's uv satisfies.
///
/// THE RANGE IS READ AS THE PIN IS: the two unambiguous clauses, a floor and a
/// ceiling, settled from the bytes, and every other clause left to uv.
fn zbkcx_backend(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    manifest: &DeTable<'_>,
    findings: &mut Vec<String>,
) {
    let Some(system) = zbkcx_sub(manifest, ZBKCX_BUILD_SYSTEM) else {
        return;
    };

    let backend = zbkcx_text(system, ZBKCX_BUILD_BACKEND).unwrap_or("");

    if backend != ZBKCX_UV_BACKEND {
        findings.push(format!(
            "[{}] names '{}' as its {} — the project's own build is the one build the converge \
             performs, and its requirements stand in no lock, so the backend is uv's own ({}), \
             which the pinned uv carries built in and fetches nothing for",
            ZBKCX_BUILD_SYSTEM,
            backend,
            ZBKCX_BUILD_BACKEND,
            ZBKCX_UV_BACKEND, //
        ));
        return;
    }

    let requires: Vec<&str> = zbkcx_at(system, ZBKCX_BUILD_REQUIRES)
        .and_then(DeValue::as_array)
        .map(|list| {
            list.iter()
                .filter_map(|held| held.get_ref().as_str())
                .collect()
        })
        .unwrap_or_default();

    let Some(requirement) = requires
        .iter()
        .find(|held| zbkcx_requirement_name(held) == ZBKCX_UV_BACKEND)
    else {
        findings.push(format!(
            "[{}] names {} and its {} does not require it — the range that keeps the build on the \
             pinned uv's own copy is the requirement's, and a backend required by nothing is \
             fetched from wherever uv looks",
            ZBKCX_BUILD_SYSTEM,
            ZBKCX_UV_BACKEND,
            ZBKCX_BUILD_REQUIRES, //
        ));
        return;
    };

    if requires.len() != 1 {
        findings.push(format!(
            "[{}] requires {} beside other requirements ({}) — the project's build requirements \
             stand in no lock, so the one requirement is the backend the pinned uv carries built \
             in",
            ZBKCX_BUILD_SYSTEM,
            ZBKCX_UV_BACKEND,
            requires.join(", "), //
        ));
    }

    let kibble = match bkcq_resolve(repository, dispatch.bkcr_weaned(), BKCX_UV) {
        Ok(resolved) => resolved.kibble,
        Err(err) => {
            findings.push(format!(
                "[{}] requires '{}' and the kibble that pins uv could not be read: {}",
                ZBKCX_BUILD_SYSTEM,
                requirement,
                err, //
            ));
            return;
        }
    };

    let pinned = kibble.bkcq_field("BKRK_VERSION").to_string();

    if !zbkcx_admits(requirement, &pinned) {
        findings.push(format!(
            "[{}] requires '{}' and the kibble '{}' pins {} — a range the pinned uv does not \
             satisfy would have uv fetch its backend from an index the lock records nothing of, \
             so the range admits the pin",
            ZBKCX_BUILD_SYSTEM,
            requirement,
            BKCX_UV,
            pinned, //
        ));
    }
}

/// The name a requirement string opens with.
fn zbkcx_requirement_name(requirement: &str) -> &str {
    let requirement = requirement.trim();
    let end = requirement
        .find(|held: char| {
            !(held.is_ascii_alphanumeric() || held == '_' || held == '-' || held == '.')
        })
        .unwrap_or(requirement.len());

    &requirement[..end]
}

/// Whether a version answers a requirement's floor and ceiling, the two
/// unambiguous clause shapes; any other clause defers to uv.
fn zbkcx_admits(requirement: &str, version: &str) -> bool {
    let Some(components) = zbkcx_version(version) else {
        return true;
    };

    let name = zbkcx_requirement_name(requirement);
    let clauses = requirement.trim()[name.len()..].trim();

    for clause in clauses.split(',') {
        let clause = clause.trim();

        if let Some(floor) = clause.strip_prefix(">=").and_then(zbkcx_version) {
            if zbkcx_below(&components, &floor) {
                return false;
            }
        } else if let Some(ceiling) = clause.strip_prefix('<').and_then(zbkcx_version) {
            if !zbkcx_below(&components, &ceiling) {
                return false;
            }
        }
    }

    true
}

/// The lists part the lock's own account of the manifest's direct dependencies.
///
/// THE PRECEDENT IS THE RUST PARTITION (`zbkcr_crates`): the none-sprue filtered
/// out before the union is taken, the union read against the tool's own account,
/// a name in both lists a finding of its own, every finding gathered before
/// anything refuses, the refusal citing its rivet. THE ACCOUNT IS THE LOCK'S
/// AND NOT THE MANIFEST'S, which is where this parts from cargo metadata: the
/// lock's entry for the project itself carries its direct dependencies and its
/// groups with every name as uv spells it, so the collar spells them the same
/// way and no second reading of a requirement string stands here. The groups
/// count for an app and a suite alike, one environment per project carrying
/// both.
///
/// COVERAGE IS NOT READ HERE. Whether every declared environment has a wheel is
/// uv's reading at the lock-authoring door, which resolves fresh under the bar;
/// what this reading proves of a baked package is that the lock carries at
/// least one wheel for it, so a package the lock records as source alone — a
/// lock written outside that door — is found before any converge meets it.
fn zbkcx_partition(project: &Path, collar: &bkcx_Collar, findings: &mut Vec<String>) {
    let lock = project.join(BKCX_LOCK_FILE);

    if !lock.is_file() {
        return;
    }

    let Some(baked) = collar.regime.bkprc_catena("BKRP_BAKED_PACKAGES") else {
        return;
    };

    // THE NONE-SPRUE IS NOT A PACKAGE NAME, so it leaves the reading here and the
    // union below is over packages alone.
    let crude: Vec<&str> = match collar.regime.bkprc_catena("BKRP_CRUDE_PACKAGES") {
        None => Vec::new(),
        Some(declared) => {
            if !declared.contains(&BKCX_BAKED) {
                findings.push(format!(
                    "{} carries no {} — the list says none is one by that value, and an empty \
                     list says nothing",
                    "BKRP_CRUDE_PACKAGES",
                    BKCX_BAKED, //
                ));
            }
            declared
                .into_iter()
                .filter(|held| *held != BKCX_BAKED)
                .collect()
        }
    };

    for named in &crude {
        findings.push(format!(
            "{} names '{}' — a {} package is refused, never built (BKr_hpt). The converge \
             installs wheels alone and provisions nothing for a build, so the field admits {} \
             alone: the package leaves the closure, or a wheel is served for it and the collar \
             names it {}",
            "BKRP_CRUDE_PACKAGES",
            named,
            BKG_CRUDE_BASE,
            BKCX_BAKED,
            BKG_BAKED_BASE, //
        ));

        if baked.contains(named) {
            findings.push(format!(
                "'{}' is named in {} and {} both — every direct dependency stands in exactly one \
                 of the two, and the pair the read-only door proves disjoint cannot say both of \
                 one package",
                named,
                "BKRP_BAKED_PACKAGES",
                "BKRP_CRUDE_PACKAGES", //
            ));
        }
    }

    let account = match zbkcx_account(&lock) {
        Ok(account) => account,
        Err(err) => {
            findings.push(err);
            return;
        }
    };

    for named in &baked {
        if !account.direct.iter().any(|held| held == named) {
            findings.push(format!(
                "{} names '{}', which the lock's account of the manifest does not carry as a \
                 direct dependency — the lists name direct dependencies alone, spelled as the \
                 lock spells them, the closure beyond them being the lock's",
                "BKRP_BAKED_PACKAGES",
                named, //
            ));
            continue;
        }

        if !account.wheeled.iter().any(|held| held == named) {
            findings.push(format!(
                "{} names '{}' and the lock carries no wheel for it — a package the lock records \
                 as source alone arrives {} on every platform, which the converge never builds \
                 (BKr_hpt)",
                "BKRP_BAKED_PACKAGES",
                named,
                BKG_CRUDE_BASE, //
            ));
        }
    }

    for named in &account.direct {
        if baked.contains(&named.as_str()) || crude.contains(&named.as_str()) {
            continue;
        }

        findings.push(format!(
            "'{}' is a direct dependency of the manifest and neither {} nor {} names it — every \
             direct dependency is named in allow form, as the lock spells it",
            named,
            "BKRP_BAKED_PACKAGES",
            "BKRP_CRUDE_PACKAGES", //
        ));
    }
}

/// The lock's account of the project it was written for: the direct
/// dependencies its own entry declares, groups included, and every package the
/// lock carries a wheel for.
struct zbkcx_Account {
    direct: Vec<String>,
    wheeled: Vec<String>,
}

/// Read the lock's account, refusing in a sentence a finding can carry.
fn zbkcx_account(lock: &Path) -> Result<zbkcx_Account, String> {
    let packages = zbkcx_lock_read(lock)?;

    let mut direct: Vec<String> = Vec::new();
    let mut wheeled: Vec<String> = Vec::new();
    let mut own = false;

    for package in &packages {
        if package.wheeled {
            wheeled.push(package.name.clone());
        }

        if package.own {
            own = true;
            direct.extend(package.direct.iter().cloned());
        }
    }

    // A SIBLING IS NO DIRECT DEPENDENCY THE LISTS NAME: it is the tree's own,
    // neither baked nor crude, and its own account stands beside the project's
    // rather than in it.
    direct.retain(|named| {
        !packages
            .iter()
            .any(|package| package.sibling && package.name == *named)
    });

    if !own {
        return Err(format!(
            "the project's {} carries no entry for the project itself, so the account the two \
             lists are read against stands nowhere — a lock the lock-authoring door writes \
             carries one",
            BKCX_LOCK_FILE, //
        ));
    }

    direct.sort();
    direct.dedup();

    Ok(zbkcx_Account { direct, wheeled })
}

/// One package entry of a lock, read for what the kennel asks of it.
struct zbkcx_Locked {
    name: String,
    /// Whether this is the project's own entry, sourced from its own directory.
    own: bool,
    /// Whether this is an editable sibling's entry, sourced from a directory of
    /// the tree other than the project's.
    sibling: bool,
    /// Whether the lock carries at least one wheel for it.
    wheeled: bool,
    /// The direct dependencies the entry declares, groups included — populated
    /// for the project's own entry alone.
    direct: Vec<String>,
}

/// Every package entry a lock carries.
fn zbkcx_lock_read(lock: &Path) -> Result<Vec<zbkcx_Locked>, String> {
    let read = std::fs::read_to_string(lock).map_err(|err| {
        format!(
            "the project's {} could not be read: {}",
            BKCX_LOCK_FILE,
            err, //
        )
    })?;

    let parsed = DeTable::parse(&read).map_err(|err| {
        format!(
            "the project's {} could not be read as toml: {}",
            BKCX_LOCK_FILE,
            err, //
        )
    })?;

    let table = parsed.get_ref();

    let Some(packages) = zbkcx_at(table, ZBKCX_LOCK_PACKAGE).and_then(DeValue::as_array) else {
        return Ok(Vec::new());
    };

    let mut read: Vec<zbkcx_Locked> = Vec::new();

    for held in packages.iter() {
        let Some(package) = held.get_ref().as_table() else {
            continue;
        };

        let name = zbkcx_text(package, "name").unwrap_or("").to_string();

        let directory = zbkcx_sub(package, ZBKCX_LOCK_SOURCE).and_then(|source| {
            ZBKCX_LOCK_OWN
                .iter()
                .find_map(|key| zbkcx_text(source, key))
        });

        let own = directory == Some(ZBKCX_LOCK_ROOT);
        let sibling = directory.is_some() && !own;

        let wheeled = zbkcx_at(package, ZBKCX_LOCK_WHEELS)
            .and_then(DeValue::as_array)
            .map(|list| !list.is_empty())
            .unwrap_or(false);

        let mut direct: Vec<String> = Vec::new();

        if own {
            if let Some(metadata) = zbkcx_sub(package, ZBKCX_LOCK_METADATA) {
                if let Some(list) =
                    zbkcx_at(metadata, ZBKCX_LOCK_REQUIRES_DIST).and_then(DeValue::as_array)
                {
                    for entry in list.iter() {
                        if let Some(named) = entry
                            .get_ref()
                            .as_table()
                            .and_then(|entry| zbkcx_text(entry, "name"))
                        {
                            direct.push(named.to_string());
                        }
                    }
                }

                if let Some(groups) = zbkcx_sub(metadata, ZBKCX_LOCK_REQUIRES_DEV) {
                    for group in groups.values() {
                        let Some(list) = group.get_ref().as_array() else {
                            continue;
                        };
                        for entry in list.iter() {
                            if let Some(named) = entry
                                .get_ref()
                                .as_table()
                                .and_then(|entry| zbkcx_text(entry, "name"))
                            {
                                direct.push(named.to_string());
                            }
                        }
                    }
                }
            }
        }

        read.push(zbkcx_Locked {
            name,
            own,
            sibling,
            wheeled,
            direct,
        });
    }

    Ok(read)
}

/// Every package the project's lock carries but the tree's own entries — the
/// project's and an editable sibling's — the roster the converge bars from
/// building (BKr_hpt).
///
/// READ OFF THE LOCK AND NEVER OFF THE MANIFEST, because the bar is over the
/// closure and the manifest names only its head. Every entry is named whether or
/// not this station's markers admit it: a name uv never installs here is a bar
/// nothing meets, and a roster narrowed by platform would be the reading the
/// kennel does not take.
pub fn bkcx_lock_packages(project: &Path) -> Result<Vec<String>, String> {
    let packages = zbkcx_lock_read(&project.join(BKCX_LOCK_FILE))?;

    Ok(packages
        .into_iter()
        .filter(|package| !package.own && !package.sibling)
        .map(|package| package.name)
        .collect())
}

/// The `[tool.uv]` table, where one stands.
fn zbkcx_uv<'t, 'i>(manifest: &'t DeTable<'i>) -> Option<&'t DeTable<'i>> {
    zbkcx_sub(manifest, "tool").and_then(|tool| zbkcx_sub(tool, "uv"))
}

/// One value of a table, with the span the parser kept stripped off.
///
/// FOUR NAMED READINGS RATHER THAN A SPAN AT EVERY CALL SITE. The document type
/// below wraps every value in its source span, which this reading never uses;
/// unwrapping it once here keeps the checks above about the settings they are
/// checking rather than about the parser's bookkeeping.
///
/// THE DOCUMENT TYPE IS THE SERDE-FREE ONE, and that is the whole shape of the
/// crate's admission. Its ergonomic sibling is gated behind the crate's serde
/// feature, which this kit does not take: with the feature off, serde is not
/// linked into the kennel at all and no procedural macro is compiled — a
/// stronger reading of "no derive" than merely declining to derive anything.
fn zbkcx_at<'t, 'i>(table: &'t DeTable<'i>, key: &str) -> Option<&'t DeValue<'i>> {
    table.get(key).map(Spanned::get_ref)
}

/// One sub-table of a table.
fn zbkcx_sub<'t, 'i>(table: &'t DeTable<'i>, key: &str) -> Option<&'t DeTable<'i>> {
    zbkcx_at(table, key).and_then(DeValue::as_table)
}

/// One string-valued key of a table.
fn zbkcx_text<'t>(table: &'t DeTable<'_>, key: &str) -> Option<&'t str> {
    zbkcx_at(table, key).and_then(DeValue::as_str)
}

/// One boolean-valued key of a table.
fn zbkcx_flag(table: &DeTable<'_>, key: &str) -> Option<bool> {
    zbkcx_at(table, key).and_then(DeValue::as_bool)
}

/// A version's numeric components, or `None` where the text names no version.
///
/// Only the numeric release segment is read. A pre-release or local suffix is
/// PEP 440's and not this reading's, and a pin carrying one is caught by the
/// component count rather than parsed.
fn zbkcx_version(text: &str) -> Option<Vec<u64>> {
    let text = text.trim();

    if text.is_empty() {
        return None;
    }

    let components: Vec<Option<u64>> = text
        .split('.')
        .map(|component| component.parse::<u64>().ok())
        .collect();

    if components.iter().any(Option::is_none) {
        return None;
    }

    Some(components.into_iter().flatten().collect())
}

/// The lower bound a specifier states, where it states one ALONE.
///
/// THE ONE UNAMBIGUOUS SHAPE. A specifier carrying a comma carries a second
/// clause this reading does not settle, and one opening with any other operator
/// is not a floor at all; both answer `None`, which defers to uv rather than
/// guessing. That is the whole of the PEP 440 surface this module claims.
fn zbkcx_floor(specifier: &str) -> Option<Vec<u64>> {
    let specifier = specifier.trim();

    if specifier.contains(',') {
        return None;
    }

    zbkcx_version(specifier.strip_prefix(">=")?)
}

/// Whether a version falls below a floor, compared component by component.
///
/// A missing component reads as zero, which is the comparison's own convention
/// and not a guess: `3.12` and `3.12.0` name the same release.
fn zbkcx_below(version: &[u64], floor: &[u64]) -> bool {
    let width = version.len().max(floor.len());

    for index in 0..width {
        let held = version.get(index).copied().unwrap_or(0);
        let owed = floor.get(index).copied().unwrap_or(0);

        if held != owed {
            return held < owed;
        }
    }

    false
}

/// The token a suite collar carries where the target is the whole project's
/// tests rather than one named node.
///
/// THE RUST FAMILY'S OWN TOKEN, READ FROM HERE RATHER THAN RESPELLED. What it
/// says is that the field is DECLARED and not omitted, which is a fact about the
/// collar genus and not about cargo; a second spelling under this family would
/// carry one meaning at two tokens.
pub const BKCX_TARGET_PROJECT: &str = BKCR_TARGET_MANIFEST;

/// The file uv writes into an environment recording what built it.
///
/// READ RATHER THAN SPAWNED, which is what makes the verify posture cheap enough
/// to be routine. The alternative is starting the environment's interpreter to
/// ask its version, and a door that did that would be trusting a program to
/// answer honestly about itself while paying a process to hear it.
const ZBKCX_MARQUE: &str = "pyvenv.cfg";

/// The key in that file carrying the interpreter's version.
const ZBKCX_MARQUE_VERSION: &str = "version_info";

/// uv's own verb and flag for asking whether a lock still answers its manifest
/// WITHOUT authoring one.
///
/// The spelling is uv's own, declared as a xenonym spelling line under that
/// authority's carrier, which is where the ruling lives now; VOr_9ww honours the
/// carrier and stands the value down.
const ZBKCX_UV_LOCK: &str = "lock";
const ZBKCX_UV_CHECK: &str = "--check";
const ZBKCX_UV_PROJECT: &str = "--project";

/// uv's own verb and flag for asking whether an environment still answers its
/// lock WITHOUT syncing it — the same `--check` riding a second verb, with the
/// lock flag beside it so the question is asked of the lock as it stands.
///
/// The spelling is uv's own, declared as a xenonym spelling line under that
/// authority's carrier.
const ZBKCX_UV_SYNC: &str = "sync";
const ZBKCX_UV_LOCKED: &str = "--locked";

/// What one python collar's environment does not answer, read by the verify
/// posture without converging anything.
///
/// A VERDICT RATHER THAN A SENTENCE, because two doors render it. The launch door
/// refuses with the whole account (`bkcx_refusal`), and the derby's preflight
/// gathers it among every other precondition the tree does not meet, as a plaint
/// beside the act that supplies it (`bkcx_plaint`, `bkcx_supply`). One reading
/// with two renderings is what keeps the two doors refusing the same things.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum bkcx_Unanswered {
    /// No environment stands: its marque is absent.
    Absent,
    /// A marque stands and carries no version line.
    Unmarqued,
    /// The interpreter has moved out from under the environment.
    Moved {
        /// The version the environment's own record carries.
        standing: String,
        /// The version the project pins.
        pinned: String,
    },
    /// The lock no longer answers the manifest. uv's own account rides.
    Unlocked {
        /// What uv said, verbatim.
        account: String,
    },
    /// The environment does not answer the lock that stands. uv's own account
    /// rides.
    Behind {
        /// What uv said, verbatim.
        account: String,
    },
}

impl bkcx_Unanswered {
    /// Why an environment that stands may not be presumed current, or `None`
    /// where nothing stands at all — the preflight's two shapes, said apart.
    ///
    /// THE KENNEL'S OWN WORDS AND NEVER UV'S. uv's account closes on advice to
    /// run its own sync or its own lock, which is the act this kennel reserves to
    /// its converge and its lock-authoring door, so a gathered report carrying it
    /// would hand its reader two remedies for one condition.
    pub fn bkcx_plaint(&self, collar: &bkcx_Collar, seat: &bkcx_Seat) -> Option<String> {
        match self {
            bkcx_Unanswered::Absent => None,
            bkcx_Unanswered::Unmarqued => Some(format!(
                "its {} carries no '{}' line, so what interpreter built it cannot be read",
                ZBKCX_MARQUE, ZBKCX_MARQUE_VERSION, //
            )),
            bkcx_Unanswered::Moved { standing, pinned } => Some(format!(
                "it stands on CPython {}, and its project pins {}",
                standing, pinned, //
            )),
            bkcx_Unanswered::Unlocked { .. } => Some(format!(
                "the lock at the project '{}' no longer answers its manifest",
                collar.bkcx_project().display(), //
            )),
            bkcx_Unanswered::Behind { .. } => Some(format!(
                "it does not answer the lock the project '{}' carries — the lock moved after {} was \
                 synced, or the environment was changed beside it",
                collar.bkcx_project().display(),
                seat.environment.display(), //
            )),
        }
    }

    /// The acts that supply what is not answered, composed as a reader types
    /// them.
    ///
    /// THE CONVERGE, SAVE WHERE THE LOCK ITSELF HAS MOVED. A converge syncs to
    /// the lock that stands and refuses one that no longer answers its manifest,
    /// so a lock behind its manifest is supplied by the lock-authoring door first,
    /// the lock it writes committed — the door law's own remedy, since the
    /// converge refuses a tree carrying it uncommitted — and then converged.
    pub fn bkcx_supply(&self, collar: &bkcx_Collar, tabtargets: Option<&Path>) -> String {
        let converge = bkco_call(tabtargets, BKCO_VERB_HEEL, Some(&collar.name));

        match self {
            bkcx_Unanswered::Unlocked { .. } => format!(
                "{}, then a {} of the lock it writes, then {}",
                bkco_call(tabtargets, BKCO_VERB_GANGLINE, Some(&collar.name)),
                crate::bkcf_guard::BKCF_REMEDY,
                converge, //
            ),
            _ => converge,
        }
    }

    /// The collar a heel converges where that heel is the whole of the supply
    /// above, or `None` where it is not.
    ///
    /// A MOVED LOCK IS NOT A HEEL'S TO SUPPLY. Its first act writes a lock, and
    /// a written lock stands dirty for the operator's notch — no tool commits —
    /// so a verb that performed the heels a tree is owed would meet that lock's
    /// door-law refusal on its next act. The reading it is handed is the supply
    /// above, whole, and the operator's.
    pub fn bkcx_heel(&self, collar: &bkcx_Collar) -> Option<String> {
        match self {
            bkcx_Unanswered::Unlocked { .. } => None,
            _ => Some(collar.name.clone()),
        }
    }

    /// The launch door's refusal: the condition, why nothing here repairs it, the
    /// converge named, and uv's own account beneath where uv gave one.
    pub fn bkcx_refusal(
        &self,
        collar: &bkcx_Collar,
        seat: &bkcx_Seat,
        tabtargets: Option<&Path>,
    ) -> String {
        let converge = bkco_call(tabtargets, BKCO_VERB_HEEL, Some(&collar.name));

        match self {
            bkcx_Unanswered::Absent => format!(
                "no environment stands for the python collar '{}': {} carries no {}. The \
                 environment is built by the converge and by no other door, and it stands outside \
                 the source tree by ruling — so there is nothing beside the project to find. \
                 Converge it: {}",
                collar.name,
                seat.environment.display(),
                ZBKCX_MARQUE,
                converge, //
            ),
            bkcx_Unanswered::Unmarqued => format!(
                "{} carries no '{}' line, so what interpreter built this environment cannot be \
                 read. The file is uv's own and its shape is not this kennel's to restate; what is \
                 missing is a record rather than a version, so the environment is rebuilt rather \
                 than inspected: {}",
                seat.environment.join(ZBKCX_MARQUE).display(),
                ZBKCX_MARQUE_VERSION,
                converge, //
            ),
            bkcx_Unanswered::Moved { standing, pinned } => {
                format!(
                "the environment for the python collar '{}' stands on CPython {}, and its project \
                 pins {} — the interpreter has moved out from under it. Nothing here rebuilds it: \
                 the one door that writes an environment is the converge, run deliberately: {}",
                collar.name, standing, pinned, converge, //
            )
            }
            bkcx_Unanswered::Unlocked { account } => zbkcx_accounted(
                format!(
                    "the lock at the project '{}' no longer answers its manifest, so the \
                     environment for the python collar '{}' was built from a resolution the \
                     declarations have moved past. Re-deriving a lock is the lock-authoring door's \
                     act and never this one's; converging to a lock that stands is the converge's: \
                     {}",
                    collar.bkcx_project().display(),
                    collar.name,
                    converge, //
                ),
                account,
            ),
            bkcx_Unanswered::Behind { account } => zbkcx_accounted(
                format!(
                    "the environment for the python collar '{}' at {} does not answer the lock its \
                     project carries — the lock moved after the environment was synced, or the \
                     environment was changed beside it. Nothing here syncs it: the one door that \
                     writes an environment is the converge, run deliberately: {}",
                    collar.name,
                    seat.environment.display(),
                    converge, //
                ),
                account,
            ),
        }
    }
}

/// A refusal with uv's own account beneath it, where uv gave one — the listing
/// refusal's own shape (`bkcx_recite`).
fn zbkcx_accounted(said: String, account: &str) -> String {
    let account = account.trim();

    if account.is_empty() {
        said
    } else {
        format!(
            "{}\n{}",
            said,
            account, //
        )
    }
}

/// The sentence uv states, at the sealed check, where the station stands outside
/// the environments the project declares in `[tool.uv] environments` — read
/// from uv 0.11.30, the uv kibble's pin.
///
/// A PALISADE MEMBRANE, DATED TO THE KIBBLE'S PIN. uv reports a station outside
/// the lock's supported environments in prose alone, at the same exit it gives
/// every other refusal of the check, so the sentence is the only signature
/// there is. It is recognized here and nowhere else, absorbed only where uv
/// says exactly this, and any other refusal at that reading stays a refusal.
/// Retired when uv gives the verdict a structured form; re-read at every bump
/// of the kibble's pin.
///
/// The sentence is uv's own prose, named beside that authority's carrier; a
/// spelling line holds one token and never a sentence.
const ZBKCX_UV_UNHITCHED: &str =
    "The current Python platform is not compatible with the lockfile's supported environments";

/// A python collar whose project declares environments this station stands
/// outside: its courses are unhitched here.
///
/// A VERDICT AND NEVER A CONDITION TO REPAIR. Neither red nor skip: the project
/// itself said which stations its lock covers, so no door here has anything to
/// supply, and every door reads it the same way — the walks name the course and
/// launch nothing, heel and mush refuse naming the declaration, tattoo reports
/// it beside a clean verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkcx_Unhitched {
    /// The environments the project declares, in the words uv repeated from
    /// its lock — the project's own words, never the kennel's.
    pub environments: String,
}

impl bkcx_Unhitched {
    /// The declaration as one clause, naming the collar.
    pub fn bkcx_said(&self, collar: &bkcx_Collar) -> String {
        format!(
            "the python collar '{}' is unhitched on this station: its project at {} declares the \
             environments {}, and this station stands outside them",
            collar.name,
            collar.bkcx_project().display(),
            self.environments, //
        )
    }

    /// The refusal a door that would build or run the collar gives: the
    /// declaration, and why nothing here supplies what it lacks.
    pub fn bkcx_refusal(&self, collar: &bkcx_Collar) -> String {
        format!(
            "{}. No environment is built for it here and nothing of it runs: the declaration is \
             the project's own, in its manifest's uv table, and no door of this kennel overrides \
             it",
            self.bkcx_said(collar), //
        )
    }
}

/// Whether the project declares environments this station stands outside,
/// read ahead of every environment reading.
///
/// THE SEALED CHECK, WHICH RUNS BEFORE ANY ENVIRONMENT STANDS. uv reads the
/// lock's supported environments against the interpreter it would build on and
/// refuses there, creating nothing, where the real sync creates the environment
/// first and leaves it standing hollow — so this reading is the check's and
/// never the sync's. `None` where uv's account is anything but the declared
/// sentence, the readings behind it then answering for what it is.
pub fn bkcx_unhitched_at(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    collar: &bkcx_Collar,
    seat: &bkcx_Seat,
) -> Result<Option<bkcx_Unhitched>, String> {
    let project = repository.join(collar.bkcx_project());
    let stated = bkcx_stated(seat, bkcx_Posture::Sealed);

    let checked = crate::bkcl_leash::bkcl_uv_recall(
        repository,
        dispatch,
        &project,
        [
            std::ffi::OsStr::new(ZBKCX_UV_SYNC),
            std::ffi::OsStr::new(ZBKCX_UV_LOCKED),
            std::ffi::OsStr::new(ZBKCX_UV_CHECK),
            std::ffi::OsStr::new(ZBKCX_UV_PROJECT),
            project.as_os_str(),
        ],
        &bkcx_borne(&stated),
    )?;

    if checked.run.bkcl_landed() {
        return Ok(None);
    }

    Ok(zbkcx_unhitched(&checked.grievance))
}

/// The declared sentence found in uv's account, and the environments uv names
/// after it, or `None` where the account says anything else.
///
/// THE ENVIRONMENTS ARE TAKEN AS UV REPEATS THEM FROM THE LOCK, which carries
/// them as the project declared them: they are the project's own words, and the
/// kennel reports them without reading what they say.
fn zbkcx_unhitched(account: &str) -> Option<bkcx_Unhitched> {
    account.lines().find_map(|line| {
        let (_, after) = line.split_once(ZBKCX_UV_UNHITCHED)?;

        Some(bkcx_Unhitched {
            environments: after.trim_start_matches(':').trim().to_string(),
        })
    })
}

/// Whether one python collar's environment still answers its declarations.
///
/// THE VERIFY POSTURE, AND IT VERIFIES RATHER THAN CONVERGING. Every door but the
/// converge asks this question and stops, naming the converge where the answer is
/// no — the routine-download ruling's own shape. Nothing here downloads, nothing
/// here writes, and the sealed posture is what makes that true of the spawn as
/// well as of the reads.
pub fn bkcx_verify(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    collar: &bkcx_Collar,
) -> Result<(), String> {
    let seat = bkcx_seat(dispatch, collar)?;

    bkcx_verify_at(repository, dispatch, collar, &seat)
}

/// The verify posture taken against a NAMED seat, so a hurdle can pose one
/// without writing an environment its parallel neighbours also read.
pub fn bkcx_verify_at(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    collar: &bkcx_Collar,
    seat: &bkcx_Seat,
) -> Result<(), String> {
    // THE DECLARATION AHEAD OF THE ENVIRONMENT. On a station the project
    // declares itself out of, no environment stands or ever will, so a reading
    // of the environment first would send its reader to a converge that refuses.
    if let Some(unhitched) = bkcx_unhitched_at(repository, dispatch, collar, seat)? {
        return Err(unhitched.bkcx_refusal(collar));
    }

    match bkcx_unanswered_at(repository, dispatch, collar, seat)? {
        None => Ok(()),
        Some(unanswered) => Err(unanswered.bkcx_refusal(collar, seat, dispatch.bkcr_tabtargets())),
    }
}

/// The verify posture's reading: `None` where the environment answers every
/// declaration, the first thing it does not answer otherwise, and `Err` only
/// where the reading itself could not be taken.
///
/// THREE READINGS, BECAUSE THREE THINGS CAN HAVE MOVED. The interpreter is read
/// from the environment's own record against the pin the project declares; the
/// lock is read against the manifest; and the environment is read against the
/// lock. A door that read the first two and not the third passed a lock that had
/// moved past its environment — and the launch's own `uv run`, which syncs the
/// environment it is handed, then installed the difference from the wheel cache
/// under the sealed posture, which is a converge performed by a door that is not
/// the converge.
///
/// THE LOCK AND ENVIRONMENT READINGS ARE UV'S AND ARE NOT REIMPLEMENTED HERE.
/// Whether a lock answers a manifest is a resolver's judgment, and whether an
/// environment answers a lock is an installer's; a second implementation of
/// either here would be a second answer free to disagree with the converge's.
///
/// UV IS HEARD AND NOT HANDED THE TERMINAL, on the listing step's own ground: this
/// is a program reading an answer, run ahead of every python course a walk takes,
/// and uv's own lines on the console would stand between the verdicts the quiet
/// flavor keeps. What uv said rides the verdict, and the launch door's refusal
/// carries it whole.
pub fn bkcx_unanswered_at(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    collar: &bkcx_Collar,
    seat: &bkcx_Seat,
) -> Result<Option<bkcx_Unanswered>, String> {
    if let Some(unanswered) = bkcx_marque_at(repository, collar, seat)? {
        return Ok(Some(unanswered));
    }

    let project = repository.join(collar.bkcx_project());
    let stated = bkcx_stated(seat, bkcx_Posture::Sealed);

    // THE LOCK BEFORE THE ENVIRONMENT, because the second reading spells the
    // lock flag and so refuses a lock behind its manifest too: asked first, it
    // would name the environment for a condition the lock-authoring door repairs.
    let locked = crate::bkcl_leash::bkcl_uv_recall(
        repository,
        dispatch,
        &project,
        [
            std::ffi::OsStr::new(ZBKCX_UV_LOCK),
            std::ffi::OsStr::new(ZBKCX_UV_CHECK),
            std::ffi::OsStr::new(ZBKCX_UV_PROJECT),
            project.as_os_str(),
        ],
        &bkcx_borne(&stated),
    )?;

    if !locked.run.bkcl_landed() {
        return Ok(Some(bkcx_Unanswered::Unlocked {
            account: locked.grievance,
        }));
    }

    let synced = crate::bkcl_leash::bkcl_uv_recall(
        repository,
        dispatch,
        &project,
        [
            std::ffi::OsStr::new(ZBKCX_UV_SYNC),
            std::ffi::OsStr::new(ZBKCX_UV_LOCKED),
            std::ffi::OsStr::new(ZBKCX_UV_CHECK),
            std::ffi::OsStr::new(ZBKCX_UV_PROJECT),
            project.as_os_str(),
        ],
        &bkcx_borne(&stated),
    )?;

    if !synced.run.bkcl_landed() {
        return Ok(Some(bkcx_Unanswered::Behind {
            account: synced.grievance,
        }));
    }

    Ok(None)
}

/// The verify posture's first half: the environment's own record against the
/// pin, read from the file and spawning nothing.
///
/// STANDING APART SO A READER WITH NO UV CAN STILL TAKE IT. The derby's preflight
/// gathers an unplaced uv as a precondition of its own, and every later reading
/// reaches uv through its residence; this half needs none, so a cold seat is
/// told in one report both that uv is owed and which environments stand nowhere.
pub fn bkcx_marque_at(
    repository: &Path,
    collar: &bkcx_Collar,
    seat: &bkcx_Seat,
) -> Result<Option<bkcx_Unanswered>, String> {
    let pinned = bkcx_pinned(repository, collar)?;

    let marque = seat.environment.join(ZBKCX_MARQUE);

    if !marque.is_file() {
        return Ok(Some(bkcx_Unanswered::Absent));
    }

    let said = std::fs::read_to_string(&marque).map_err(|err| {
        format!(
            "could not read {}: {}",
            marque.display(),
            err, //
        )
    })?;

    let Some(standing) = zbkcx_marqued(&said) else {
        return Ok(Some(bkcx_Unanswered::Unmarqued));
    };

    // COMPARED ON THE PIN'S OWN COMPONENTS AND NOT AS TEXT. Python spells a
    // version_info with a release qualifier at some interpreters and without one
    // at others, so an equality test over the whole string would refuse a
    // correct environment on a station whose python spells more than three
    // components. The pin is exact by the validation above, so its components
    // are the honest thing to require.
    if !zbkcx_answers(standing, &pinned) {
        return Ok(Some(bkcx_Unanswered::Moved {
            standing: standing.to_string(),
            pinned,
        }));
    }

    Ok(None)
}

/// The version one environment's own record carries.
///
/// READ DEFENSIVELY, THE FILE BEING PYTHON'S RATHER THAN OURS. This is the
/// Palisade: a shape nobody here elected, so an absent key produces a refusal
/// naming what was missing rather than a default that would read as agreement.
fn zbkcx_marqued(said: &str) -> Option<&str> {
    for line in said.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        if key.trim() == ZBKCX_MARQUE_VERSION {
            let value = value.trim();
            if !value.is_empty() {
                return Some(value);
            }
        }
    }

    None
}

/// Whether a standing version answers an exact pin, compared component-wise over
/// the pin's own length.
fn zbkcx_answers(standing: &str, pinned: &str) -> bool {
    let mut held = standing.split('.');

    for want in pinned.split('.') {
        match held.next() {
            Some(found) if found == want => continue,
            _ => return false,
        }
    }

    true
}

/// Every python collar the weaned set holds, named, for a door whose refusal must
/// account for a family the caller may have meant.
///
/// A DOOR SERVING TWO COLLAR FAMILIES OWES BOTH ROSTERS. The rust family's
/// refusal names the rust family's collars and is correct about what it says;
/// what it cannot say is that the name might have been a python collar's, since
/// that family's reading is not its to take. So the door joins the two, and this is
/// the half that was not otherwise reachable.
///
/// A READING THAT REFUSES ANSWERS NOTHING RATHER THAN PROPAGATING. This reader
/// exists to enrich another refusal, and a refusal that failed to compose because
/// its enrichment failed would replace a useful sentence with an unrelated one.
pub fn bkcx_roster(repository: &Path, dispatch: &crate::bkcr_resolve::bkcr_Dispatch) -> String {
    match bkcx_walk(repository, dispatch.bkcr_weaned()) {
        Ok(instances) => zbkcx_listed(&instances),
        Err(_) => String::new(),
    }
}

/// An instance directory's own name, which is the collar name.
fn zbkcx_named(instance: &Path) -> String {
    instance
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Every instance the walk found, for a refusal that hands its reader the roster
/// rather than a search.
fn zbkcx_listed(instances: &[PathBuf]) -> String {
    if instances.is_empty() {
        return "none".to_string();
    }

    let mut named: Vec<String> = instances.iter().map(|held| zbkcx_named(held)).collect();
    named.sort();
    named.join(", ")
}

/// The kennel's own quarter, the segment standing between a shared root and
/// anything this kit places beneath it.
///
/// ONE SPELLING FOR TWO ROOTS. It names the kennel's quarter of the station's
/// tackroom, where the kibble residences already stand, and the kennel's quarter
/// of the checkout's loosebox, where a project's environment stands. Both are
/// roots the kennel shares with other tenants, and a quarter is what says whose
/// a directory is without asking the root to know.
const ZBKCX_QUARTER: &str = "bkk";

/// Where the managed interpreters stand under the tackroom quarter.
const ZBKCX_INTERPRETERS: &str = "python";

/// Where uv keeps what it downloaded, under the tackroom quarter.
///
/// ONE DIRECTORY AND NOT TWO. The store's ruling names uv's wheel cache and its
/// archive cache, and uv holds both beneath ONE cache root it takes from one
/// variable — read from uv's own roster rather than supposed. A kennel stating
/// two names would be inventing a knob uv does not carry.
const ZBKCX_CACHE: &str = "uv-cache";

/// Where the interpreter's compiled bytecode stands, under the loosebox quarter.
///
/// KEYED ON THE PROJECT LIKE THE ENVIRONMENT BESIDE IT, and for the environment's
/// own reason: one project is shared by several collars, so keying on the collar
/// would give one project as many bytecode trees as it has collars.
const ZBKCX_BYTECODE: &str = "pycache";

/// Where an interpreter executable would land if one were ever written, under
/// the loosebox quarter.
///
/// IT IS A GUARD AND NOT A DESTINATION. The kennel spawns interpreters by
/// absolute path and reads no executable from here; what this directory is for
/// is to stand where uv's default would otherwise be — a directory on the
/// operator's own `PATH`, in the operator's own home — so that a write the
/// suppression below failed to stop lands somewhere the checkout owns and the
/// disband can reach. It is expected to stay empty, and an executable found here
/// is the suppression having failed rather than this directory having worked.
const ZBKCX_EXECUTABLES: &str = "bin";

/// Where one collar's environment and the stores it draws on stand.
///
/// THE FOUR RIDE TOGETHER BECAUSE THE ENVIRONMENT NAMES ALL FOUR, and a caller
/// that re-derived any of them could state a path the converge did not use.
#[derive(Debug, Clone)]
pub struct bkcx_Seat {
    /// This collar's own project environment, under the checkout's loosebox.
    pub environment: PathBuf,
    /// The managed interpreter store, shared by every collar on the station.
    pub store: PathBuf,
    /// uv's cache, shared by every collar on the station.
    pub cache: PathBuf,
    /// The executable directory the interpreter install is fenced into.
    pub executables: PathBuf,
    /// Where the interpreter writes the bytecode it compiles.
    ///
    /// A FENCE AND NOT A STORE. Python writes a `__pycache__` beside every module
    /// it imports, WHICH MEANS INSIDE THE SOURCE TREE — and it was observed doing
    /// exactly that: a suite drive left the lure's own project dirty, so the
    /// door's next drive refused the repository it had just run over. Bytecode is
    /// derived and rebuildable, which the store's ruling places under the
    /// loosebox; this names where, so the interpreter writes it there instead of
    /// beside the sources.
    pub bytecode: PathBuf,
}

/// Compose one collar's seat under NAMED roots.
///
/// THE PURE HALF, AND IT IS PURE SO IT CAN BE HURDLED, on the kibble residence's
/// own precedent: the reading below consults the process environment, which a
/// hurdle could only pose by WRITING — and a written environment is shared by
/// every hurdle the harness runs in parallel, so a hurdle posing a root would be
/// deciding its neighbours' answers.
///
/// THE ENVIRONMENT IS KEYED ON THE PROJECT'S PATH WITHIN THE CHECKOUT AND NOT ON
/// THE COLLAR'S NAME. The store's ruling is that one project is shared by several
/// collars, so keying on the collar would give one project as many environments
/// as it has collars — each synced from the same lock, and each a full copy. The
/// project path is what the environment is OF; the collar is only what asked for
/// it.
///
/// NO SEGMENT IS ADDED BENEATH THE PROJECT PATH, which is the identity ruling
/// spelled as a path: no word was minted for an environment, so an environment is
/// spoken of by its project and stands AT the directory its project's path names.
pub fn bkcx_seat_at(loosebox: &Path, tackroom: &Path, collar: &bkcx_Collar) -> bkcx_Seat {
    let quartered = loosebox.join(ZBKCX_QUARTER);

    bkcx_Seat {
        environment: quartered.join(collar.bkcx_project()),
        store: tackroom.join(ZBKCX_QUARTER).join(ZBKCX_INTERPRETERS),
        cache: tackroom.join(ZBKCX_QUARTER).join(ZBKCX_CACHE),
        executables: quartered.join(ZBKCX_EXECUTABLES),
        bytecode: quartered.join(ZBKCX_BYTECODE).join(collar.bkcx_project()),
    }
}

/// What the python plane provisions under NAMED roots, collar by collar or
/// not: the managed interpreter store and uv's cache under the tackroom's
/// quarter, and the checkout's quarter of the loosebox, where every
/// environment, the fenced executables and the bytecode stand.
///
/// THE ROOTS A BLEACH EMPTIES, composed from the same spellings the seat is, so
/// the door that empties them cannot come to hold a second idea of where they
/// stand.
pub fn bkcx_provisions_at(loosebox: &Path, tackroom: &Path) -> Vec<PathBuf> {
    vec![
        tackroom.join(ZBKCX_QUARTER).join(ZBKCX_INTERPRETERS),
        tackroom.join(ZBKCX_QUARTER).join(ZBKCX_CACHE),
        loosebox.join(ZBKCX_QUARTER),
    ]
}

/// Compose one collar's seat from the roots this station declared.
///
/// The two readings refuse in their own sentences — the loosebox names the
/// dispatch that composes it, the tackroom names the fence — so a station
/// misprovisioned on either axis meets the sentence its own root owns rather
/// than one this module invented.
pub fn bkcx_seat(
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    collar: &bkcx_Collar,
) -> Result<bkcx_Seat, String> {
    let loosebox = crate::bkcc_record::bkcc_loosebox(dispatch)?;
    let tackroom = crate::bkcl_leash::bkcl_fenced(dispatch)?;

    Ok(bkcx_seat_at(&loosebox, &tackroom, collar))
}

/// How much of the network one invocation may reach, and whether it may build.
///
/// THE POSTURE IS THE ONLY AXIS THE ROSTER VARIES ON, and it carries exactly the
/// two lifts the launch envelope's routine-download ruling admits — the converge
/// reaches an index, the converge's install step alone reaches an interpreter,
/// and every other door on every other verb is sealed — and the one bar the
/// tamed-build premise states: the lock-authoring door resolves under the global
/// build bar, so a resolution that would have to build refuses instead. A caller
/// elects a posture by name, so no door can reach a lift or lose a bar by
/// threading a boolean it did not read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum bkcx_Posture {
    /// Every door but the converge and the lock-authoring door: offline, and no
    /// interpreter is downloaded.
    Sealed,
    /// The converge's sync: an index is reachable, and no interpreter is
    /// downloaded. The bar it states is per package and rides beside the roster
    /// (`bkcx_barred`), because the global form refuses the project's own
    /// editable build.
    Reaching,
    /// The lock-authoring door alone: an index is reachable, no interpreter is
    /// downloaded, and every build is barred, so a dependency with no wheel for
    /// an environment the project declares refuses the resolution naming the
    /// package and the environment (BKr_hpt's reading at the lock).
    Authoring,
    /// The converge's install step, and nothing else in the kennel: an
    /// interpreter may be downloaded.
    Fetching,
}

/// uv's own names for what the kennel states, spelled once.
///
/// EVERY ONE OF THESE WAS READ OUT OF UV RATHER THAN REMEMBERED, and two of them
/// are why that matters: the project-environment variable is documented under one
/// subcommand's help and honoured by the others, and the preference variable
/// appears in no help text this kennel greps yet is real — uv refuses a bogus
/// value for it, naming its four. A roster composed from what the help pages
/// happen to list would have carried neither.
///
/// The environment names are uv's own, declared as xenonym spelling lines under
/// that authority's carrier.
/// An environment variable a neighbour reads is that neighbour's spelling as
/// much as a subcommand is, and the carrier declares it on that ground.
const ZBKCX_VAR_ENVIRONMENT: &str = "UV_PROJECT_ENVIRONMENT";
const ZBKCX_VAR_STORE: &str = "UV_PYTHON_INSTALL_DIR";
const ZBKCX_VAR_CACHE: &str = "UV_CACHE_DIR";
const ZBKCX_VAR_EXECUTABLES: &str = "UV_PYTHON_BIN_DIR";
const ZBKCX_VAR_PREFERENCE: &str = "UV_PYTHON_PREFERENCE";
const ZBKCX_VAR_DOWNLOADS: &str = "UV_PYTHON_DOWNLOADS";
const ZBKCX_VAR_INSTALL_BIN: &str = "UV_PYTHON_INSTALL_BIN";
const ZBKCX_VAR_ENV_FILE: &str = "UV_NO_ENV_FILE";
const ZBKCX_VAR_OFFLINE: &str = "UV_OFFLINE";
const ZBKCX_VAR_NO_BUILD: &str = "UV_NO_BUILD";
const ZBKCX_VAR_NO_BUILD_PACKAGE: &str = "UV_NO_BUILD_PACKAGE";

/// PYTHON'S OWN NAME AND NOT UV'S, and the only one on this roster that is.
///
/// IT IS STATED HERE BECAUSE THE ROSTER IS WHAT REACHES THE CHILD'S CHILD. What
/// the kennel spawns is uv, and what uv spawns is an interpreter; a knob spelled
/// as a flag would hold for the one invocation that spelled it, and the write it
/// suppresses is one EVERY python this kennel reaches would otherwise make. The
/// guard that matters is the one no future door can forget, which is the same
/// reasoning the executable suppression above records.
const ZBKCX_VAR_PYCACHE: &str = "PYTHONPYCACHEPREFIX";

/// The one variable an app's entry point is handed: where the interpreter writes
/// the bytecode it compiles.
///
/// AN APP IS SPAWNED BY ABSOLUTE PATH AND NEVER THROUGH UV, so it meets none of
/// the roster the kennel states to uv — and the converge installs the project
/// itself as editable, so its modules are imported from the source tree, where
/// the interpreter would otherwise write bytecode beside them. Only this variable
/// rides: the app's environment is otherwise its caller's, as a rust app's is.
pub fn bkcx_app_bytecode(seat: &bkcx_Seat) -> (&'static str, OsString) {
    (ZBKCX_VAR_PYCACHE, seat.bytecode.clone().into_os_string())
}

/// The interpreter preference the kennel states, which is uv's own spelling for
/// "the managed store and nothing the station happens to hold".
const ZBKCX_ONLY_MANAGED: &str = "only-managed";

/// The download postures, in uv's own spelling.
const ZBKCX_NEVER: &str = "never";
const ZBKCX_MANUAL: &str = "manual";

/// uv's spelling for a boolean stated in the environment.
const ZBKCX_ON: &str = "1";
const ZBKCX_OFF: &str = "0";

/// What the kennel states to uv on every invocation.
///
/// THIS IS THE WHOLE OF THE KENNEL'S CONTROL OVER UV, and the design says so
/// twice: there is no file format of the kennel's own, and the validation over
/// the project's files is the other half rather than a second authority. What is
/// stated here beats both the project's own uv table and any `uv.toml` standing
/// above the project, so a knob on this roster is a knob no file on the station
/// can move.
///
/// THE CONFIGURATION FLAG IS NOT HERE AND MUST NOT BE ADDED. uv's no-config flag
/// would drop the project's own uv table — its index declarations with it, and
/// silently — so a project pinning a private index would resolve from the public
/// one under a green exit. The residue this leaves is honest and is named rather
/// than papered over: a knob the kennel does NOT state can still be moved by a
/// discovered `uv.toml`, and closing that is the validation's work and not the
/// environment's.
///
/// THE EXECUTABLE SUPPRESSION IS STATED AS ENVIRONMENT AND NOT AS A FLAG, which
/// is what makes it total. The install step spells the flag too, but a flag holds
/// only where it is spelled, and the guard that matters is the one no future door
/// can forget: an interpreter write is suppressed for every uv this kennel
/// spawns, on every verb, whether or not that verb was thought about here.
pub fn bkcx_stated(seat: &bkcx_Seat, posture: bkcx_Posture) -> Vec<(&'static str, OsString)> {
    let mut stated: Vec<(&'static str, OsString)> = vec![
        (
            ZBKCX_VAR_ENVIRONMENT,
            seat.environment.clone().into_os_string(),
        ),
        (ZBKCX_VAR_STORE, seat.store.clone().into_os_string()),
        (ZBKCX_VAR_CACHE, seat.cache.clone().into_os_string()),
        (
            ZBKCX_VAR_EXECUTABLES,
            seat.executables.clone().into_os_string(),
        ),
        (ZBKCX_VAR_PREFERENCE, OsString::from(ZBKCX_ONLY_MANAGED)),
        (ZBKCX_VAR_INSTALL_BIN, OsString::from(ZBKCX_OFF)),
        (ZBKCX_VAR_ENV_FILE, OsString::from(ZBKCX_ON)),
        (ZBKCX_VAR_PYCACHE, seat.bytecode.clone().into_os_string()),
    ];

    stated.push((
        ZBKCX_VAR_DOWNLOADS,
        OsString::from(match posture {
            bkcx_Posture::Fetching => ZBKCX_MANUAL,
            bkcx_Posture::Sealed | bkcx_Posture::Reaching | bkcx_Posture::Authoring => ZBKCX_NEVER,
        }),
    ));

    // EVERY POSTURE STATES THE VARIABLE, AND THE REACHING ONES STATE IT FALSE
    // RATHER THAN LEAVING IT OFF THE ROSTER. Two reasons, and the second is the
    // sharper: every door that spawns uv was itself launched by a door that may
    // hold the variable, so an unstated offline would seal a converge that is
    // entitled to reach — and it would do it by making a fetch fail, which reads
    // as a network fault rather than as a posture. And uv REFUSES AN EMPTY
    // BOOLEAN outright, naming the variable and expecting a boolish value, so
    // clearing one by stating nothing is not available here even where it would
    // have read well.
    stated.push((
        ZBKCX_VAR_OFFLINE,
        OsString::from(match posture {
            bkcx_Posture::Sealed => ZBKCX_ON,
            bkcx_Posture::Reaching | bkcx_Posture::Authoring | bkcx_Posture::Fetching => ZBKCX_OFF,
        }),
    ));

    // THE GLOBAL BUILD BAR IS STATED ON EVERY POSTURE AND TRUE ON ONE. Stated
    // everywhere for the offline knob's reason — a door that inherited the bar
    // from its launcher would refuse the project's own build and read as a build
    // fault rather than a posture. True at the lock-authoring door alone, where
    // locking a project with static metadata reads its manifest and builds
    // nothing (benched: the three exemplars re-lock byte-identical under it),
    // so every dependency that would have to be built refuses there, naming the
    // package and the environment. The converge's bar is per package and rides
    // beside this roster (`bkcx_barred`): under the global form uv refuses the
    // project's own editable build, its help text's exemption notwithstanding.
    stated.push((
        ZBKCX_VAR_NO_BUILD,
        OsString::from(match posture {
            bkcx_Posture::Authoring => ZBKCX_ON,
            bkcx_Posture::Sealed | bkcx_Posture::Reaching | bkcx_Posture::Fetching => ZBKCX_OFF,
        }),
    ));

    stated
}

/// The converge's bar: every package the lock carries but the project's own,
/// named to uv as the packages it may not build (BKr_hpt).
///
/// THE PROJECT ITSELF IS NEVER ON THIS ROSTER, and the reader composing it
/// (`bkcx_lock_packages`) leaves it off by the lock's own source entry rather
/// than by name: named here, the project's editable build refuses exactly as it
/// does under the global bar, benched. An empty roster is stated empty rather
/// than withheld, uv accepting it, so that a dependency-free project meets the
/// same variable every other project does.
pub fn bkcx_barred(packages: &[String]) -> (&'static str, OsString) {
    (
        ZBKCX_VAR_NO_BUILD_PACKAGE,
        OsString::from(packages.join(" ")),
    )
}

/// The exact patch version this collar's project pins.
///
/// READ FROM THE PIN FILE AND NEVER FROM THE MANIFEST, because they answer
/// different questions: the manifest states which versions the project is
/// COMPATIBLE with, and the pin states which one this environment is BUILT on.
/// The validation above already refuses a pin that is a bare floor rather than an
/// exact patch, so a collar reaching this reader carries three components.
pub fn bkcx_pinned(repository: &Path, collar: &bkcx_Collar) -> Result<String, String> {
    let path = repository.join(collar.bkcx_project()).join(BKCX_PIN_FILE);

    let said = std::fs::read_to_string(&path).map_err(|err| {
        format!(
            "could not read {}: {}",
            path.display(),
            err, //
        )
    })?;

    let pinned = said.trim().to_string();

    if pinned.is_empty() {
        return Err(format!(
            "{} stands but names no version — the pin is what the interpreter install is spelled \
             with, so an empty one would install whatever uv thought newest",
            path.display(), //
        ));
    }

    Ok(pinned)
}

////////////////////////////////////////////////////////////////////////////////
// The launch
//
// What mush spells over a python collar. The four steps are the launch
// envelope's own and the kennel's matching dialect is unchanged; what stands
// here is the pieces this tenant answers with — uv's verbs, pytest's flags, and
// where an entry point stands.

/// uv's own verb for running a program inside a project's environment.
///
/// The spelling is uv's own, declared as a xenonym spelling line under that
/// authority's carrier, which is where the ruling lives now; VOr_9ww honours the
/// carrier and stands the value down.
const ZBKCX_UV_RUN: &str = "run";

/// The runner this tenant spawns, as uv is asked for it.
const ZBKCX_PYTEST: &str = "pytest";

/// pytest's own flags: the listing pair, and the verbose flag the run rides.
///
/// THE LISTING IS QUIET AS WELL AS COLLECT-ONLY, and the pairing is what makes
/// the answer parseable: collect-only alone renders a nested tree of module and
/// function objects, and the quiet flag is what turns it into one node id per
/// line — the form pytest also TAKES BACK as an exact selection. Asking for the
/// tree and flattening it here would be inventing a spelling out of a rendering.
const ZBKCX_PYTEST_COLLECT: &str = "--collect-only";
const ZBKCX_PYTEST_QUIET: &str = "-q";

/// pytest's cache plugin, switched OFF on every invocation the kennel spells.
///
/// IT WRITES INTO THE SOURCE TREE, which is the whole reason. The plugin keeps a
/// `.pytest_cache` directory beside the project it collected, so a launch would
/// leave the repository dirty and the next door would refuse it — the same
/// breach the bytecode prefix above closes, by a second road. THE KENNEL HAS NO
/// USE FOR WHAT IT HOLDS: the cache remembers which cases last failed so a
/// runner can re-run them, and narrowing is the selection pipeline's act, taken
/// in the kennel against the runner's own listing. Nothing here would ever read
/// it.
const ZBKCX_PYTEST_NO_CACHE: &[&str] = &[
    "-p",
    "no:cacheprovider", //
];

/// Every warning a run raises made an error, on every invocation the kennel
/// spells.
///
/// A WARNING IS A FAILURE, as it is on the rust plane, where a crate denies its
/// warnings and the muzzle bars its lints. pytest's own default prints a warning
/// in a summary the kennel's reading steps over, so a run that warned read green;
/// made an error, a warning fails the case that raised it, and one raised at
/// import fails the collection, so a course cannot pass over one.
const ZBKCX_PYTEST_WARNINGS_FATAL: &[&str] = &[
    "-W",
    "error", //
];

/// THE RUN IS VERBOSE BECAUSE THE VOICE READS CASES AND NOT A SUMMARY. pytest's
/// default is a row of dots carrying no name, so a course rendered from it could
/// be counted but never have a failing case named — and the bounded failure line
/// is the whole of what the quiet flavor says. The flag is what lets one reader
/// answer for a python course as it does for a rust one
/// (`bkcv_voice::BKCV_TONGUE_PYTEST` carries the membrane).
const ZBKCX_PYTEST_VERBOSE: &str = "-v";

/// pytest's own exit for a run that COLLECTED NOTHING.
///
/// ---- PALISADE MEMBRANE ----
///
/// FOREIGN SIGNATURE. pytest parts its exits by kind, and a session that
/// collected no case at all takes this one rather than the zero a green run
/// takes or the one a failing case takes. Captured at seat
/// ace9b08ec84a229f4c57372371c514705c6ad789: a drive over a file declaring no
/// test function printed `no tests ran in 0.00s` and exited 5.
///
/// WHY THE KENNEL READS IT AT ALL, HAVING A GATE OF ITS OWN. The hollow gate
/// asks whether a course LANDED and counted nothing, which is the only shape a
/// cargo harness can present — it reports an empty selection as a clean zero.
/// pytest never presents it: an empty collection is already nonzero here, so the
/// gate could never fire and a python course would take pytest's number instead
/// of the kennel's. Reading this one value is what lets the kennel answer in its
/// own vocabulary, and it is the same move the nextest launch already makes by
/// spelling `--no-tests=fail` — a rule the kennel depends on but never states is
/// a rule held by another program's release notes.
///
/// ABSORB ONLY THIS ONE VALUE. Every other nonzero exit passes through as
/// pytest's own: what this recognizes is an empty collection and nothing else,
/// and a run that died for any other reason must not be renamed hollow.
///
/// RETIREMENT CONDITION. Retire this when the kennel can ask pytest to refuse an
/// empty collection by a flag of its own, as nextest is asked.
pub const BKCX_PYTEST_EMPTY: i32 = 5;

/// Where a venv keeps the programs it installed.
const ZBKCX_VENV_BIN: &str = "bin";

/// The stated roster as the leash takes it.
///
/// SPELLED ONCE FOR THE WHOLE FAMILY. Every door that spawns uv needs this
/// conversion and none of them needs a different one, so a second copy would be
/// a second chance for one door to hand the leash a roster another would not.
pub fn bkcx_borne<'a>(
    stated: &'a [(&'static str, OsString)],
) -> Vec<(&'static str, &'a std::ffi::OsStr)> {
    stated
        .iter()
        .map(|(name, value)| (*name, value.as_os_str()))
        .collect()
}

/// The absolute path to this collar's project, joined from the repository root.
///
/// IT IS WHAT THE LEASH IS TOLD TO STAND IN, and it is absolute because the
/// child's working directory cannot be a relative one — a caller that handed the
/// leash a repo-relative path would be composing a directory that resolves
/// against wherever the door happened to be standing.
pub fn bkcx_at(repository: &Path, collar: &bkcx_Collar) -> PathBuf {
    repository.join(collar.bkcx_project())
}

/// Ask pytest to name every case it holds, and read the names back.
///
/// THE RECALL FACE IS TAKEN RATHER THAN THE DRIVEN ONE: this is a program
/// reading a runner's answer, not an operator watching work. A listing that did
/// not land refuses carrying pytest's own account, because a parse over the
/// output of a refused invocation would report an empty suite — and an empty
/// suite is exactly what the hollow gate exists to catch, so a listing failure
/// rendered as one would be the false green this whole pipeline is drawn to
/// prevent.
///
/// THE SEALED POSTURE, like every door but the converge. A listing reaches no
/// index and downloads nothing: the environment it lists already stands, or the
/// verify ahead of it already refused naming the converge.
pub fn bkcx_recite(
    repository: &Path,
    dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
    collar: &bkcx_Collar,
    seat: &bkcx_Seat,
) -> Result<Vec<String>, String> {
    let at = bkcx_at(repository, collar);
    let stated = bkcx_stated(seat, bkcx_Posture::Sealed);

    let mut spelled: Vec<OsString> = vec![
        OsString::from(ZBKCX_UV_RUN),
        OsString::from(ZBKCX_UV_PROJECT),
        at.as_os_str().to_os_string(),
        OsString::from(ZBKCX_PYTEST),
        OsString::from(ZBKCX_PYTEST_COLLECT),
        OsString::from(ZBKCX_PYTEST_QUIET),
    ];
    spelled.extend(ZBKCX_PYTEST_NO_CACHE.iter().map(OsString::from));
    spelled.extend(ZBKCX_PYTEST_WARNINGS_FATAL.iter().map(OsString::from));

    let recall = crate::bkcl_leash::bkcl_uv_recall(
        repository,
        dispatch,
        &at,
        &spelled,
        &bkcx_borne(&stated),
    )?;

    if !recall.run.bkcl_landed() {
        return Err(format!(
            "the runner would not list the suite's hurdles for the python collar '{}': {} exited \
             {}. Nothing was narrowed and nothing ran — a selection cannot be matched against a \
             listing that was never taken.\n{}",
            collar.name,
            recall.run.spelling,
            match recall.run.code {
                Some(code) => code.to_string(),
                None => "on a signal".to_string(),
            },
            recall.grievance.trim(), //
        ));
    }

    crate::bkci_pipeline::bkci_pytest_listing(&String::from_utf8_lossy(&recall.said))
}

/// Spell the run: uv's verb, the project, pytest, the verbose flag, and whatever
/// selection the kennel matched.
///
/// AN EMPTY SELECTION IS A WHOLE SUITE AND NEVER A NARROWED ONE. The bare drive
/// spawns no listing and spells no names, exactly as the rust arm's does, so a
/// caller that asked for whole gets pytest's own whole collection rather than a
/// selection the kennel composed for it.
pub fn bkcx_run_call(project: &Path, selection: &[OsString]) -> Vec<OsString> {
    let mut spelled = vec![
        OsString::from(ZBKCX_UV_RUN),
        OsString::from(ZBKCX_UV_PROJECT),
        project.as_os_str().to_os_string(),
        OsString::from(ZBKCX_PYTEST),
        OsString::from(ZBKCX_PYTEST_VERBOSE),
    ];
    spelled.extend(ZBKCX_PYTEST_NO_CACHE.iter().map(OsString::from));
    spelled.extend(ZBKCX_PYTEST_WARNINGS_FATAL.iter().map(OsString::from));
    spelled.extend(selection.iter().cloned());
    spelled
}

/// Where this collar's entry point stands.
///
/// THE KENNEL DECIDED THIS PATH AND THE COLLAR DECLARES ONLY THE NAME, which is
/// the family's one-field ruling spelled as a join: a python app's entry point
/// stands in the venv the kennel itself sited under the loosebox, so a residence
/// field would ask a collar to restate a fact the kennel already settled.
pub fn bkcx_entry(seat: &bkcx_Seat, collar: &bkcx_Collar) -> PathBuf {
    seat.environment
        .join(ZBKCX_VENV_BIN)
        .join(collar.bkcx_field("BKRP_BYNAME"))
}

/// Whether the interpreter this collar's project pins stands in the managed
/// store, refusing by name where it does not.
///
/// THE GUARD BELONGS TO EVERY DOOR THAT RESOLVES AND NEVER CONVERGES. The
/// lock-authoring door stands in the reaching posture, which may reach an index
/// and may NOT fetch an interpreter, so a cold store makes uv refuse for a
/// reason that reads as a resolution fault. This says the true thing first and
/// names the one door entitled to install one.
///
/// IT REFUSES AHEAD OF THE SPAWN, deliberately: uv is handed this process's
/// streams, so its own account goes to the operator's terminal and never into a
/// refusal a caller could enrich afterwards. A sentence owed to the reader has
/// to be composed before the child runs or not at all.
pub fn bkcx_interpreter_stands(
    repository: &Path,
    collar: &bkcx_Collar,
    seat: &bkcx_Seat,
    tabtargets: Option<&Path>,
) -> Result<(), String> {
    let pinned = bkcx_pinned(repository, collar)?;

    match zbkcx_stored(&seat.store, &pinned) {
        None | Some(true) => Ok(()),
        Some(false) => Err(format!(
            "the interpreter the python collar '{}' pins ({}) stands nowhere in the managed store \
             at {}. Nothing here downloads one: the converge is the only door in this kennel \
             entitled to fetch an interpreter, and this one resolves against what it finds. \
             Converge it first: {}",
            collar.name,
            pinned,
            seat.store.display(),
            bkco_call(tabtargets, BKCO_VERB_HEEL, Some(&collar.name)), //
        )),
    }
}

/// Whether the store holds an interpreter at this pin — and `None` where the
/// store's own shape could not be read at all.
///
/// THE PALISADE, MARKED AS ONE. The store's layout is uv's and nobody here
/// elected it; what this reads of it is the narrowest signature that answers the
/// question — the pin standing as one whole hyphen-separated field of an entry's
/// name, which is how uv spells `cpython-3.12.7-<triple>` — contained in this
/// one function so no other reader comes to hold an idea of the layout.
///
/// AND IT STANDS ASIDE RATHER THAN REFUSING where the directory cannot be walked.
/// This guard exists to compose a better sentence than uv's, never to be a second
/// authority over what uv can do, so a shape it fails to recognize must let the
/// drive through to uv's own answer. An absent store is not that case: nothing
/// standing anywhere is a reading rather than a failure to read.
fn zbkcx_stored(store: &Path, pinned: &str) -> Option<bool> {
    if !store.exists() {
        return Some(false);
    }

    for entry in std::fs::read_dir(store).ok()? {
        let entry = entry.ok()?;

        if entry
            .file_name()
            .to_string_lossy()
            .split('-')
            .any(|field| field == pinned)
        {
            return Some(true);
        }
    }

    Some(false)
}

// eof
