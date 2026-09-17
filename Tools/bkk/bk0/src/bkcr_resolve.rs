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
//! NO DOOR VALIDATES AS ITS OWN ACT (BKSNC-Kennelcraft.adoc "Validation is
//! implicit"). A door asks for a collar by name and is handed the collar with
//! every conformance finding against it already gathered; what the door decides
//! is what to do about them, never whether to look. That is what makes "never
//! operate on an invalid collar" a property of the shape rather than a rule each
//! door has to remember, and it is why a door reaching a `bkrr.env` around this
//! module would be a defect however carefully it read one.
//!
//! EVERY FINDING IS REPORTED BEFORE ANYTHING REFUSES. A first-finding exit hands
//! its reader one repair at a time and a fresh drive between each, which is the
//! shape the substrate's own validate door was built to avoid
//! (`specs/buk/BUSTLV-NodeRegimeValidate.adoc`); this module gathers and the door
//! prints. The one thing that stops the gathering is a name that does not
//! resolve to exactly one collar, because there is then no collar to gather
//! findings against.

use crate::bkcj_json::{bkcj_read, bkcj_Value};
use crate::bkcl_leash;
use bkl::bklrc_catena::{bklrc_read, bklrc_Regime};
use std::path::{Path, PathBuf};

/// The rust family's file. Its parent directory is the instance and that
/// directory's name is the collar name (BKSCL-Collar.adoc "Container and
/// Discovery").
pub const BKCR_FAMILY: &str = "bkrr.env";

/// The linter's own file, which a declared muzzle directory must hold. Foreign
/// schema, foreign name (BKSMZ-Muzzle.adoc).
pub const BKCR_MUZZLE_FILE: &str = "clippy.toml";

/// The fields every collar declares, whatever its kind.
pub const BKCR_FIELDS_BOTH: &[&str] = &[
    "BKRR_COLLAR",
    "BKRR_KIND",
    "BKRR_MANIFEST",
    "BKRR_TARGET",
    "BKRR_ROOTS",
    "BKRR_FEATURES",
    "BKRR_PROFILE",
    "BKRR_SPEND",
    "BKRR_TAMED_CRATES",
    "BKRR_FERAL_CRATES",
    "BKRR_MUZZLE",
    "BKRR_EXERGUE",
];

/// What `BKRR_EXERGUE` carries where the crate holds no generated source at
/// all — which is the ordinary case, and is DECLARED rather than left silent on
/// the roster's own rule.
pub const BKCR_UNSTRUCK: &str = "bknre_unstruck";

/// What `BKRR_FERAL_CRATES` carries where nothing in the closure is feral —
/// which at MVP is every collar, the field admitting this value alone and
/// refusing every crate named beside it (BKSCL-Collar.adoc BKr_4n2).
pub const BKCR_TAMED: &str = "bknre_tamed";

/// The fields an app declares and a suite does not.
pub const BKCR_FIELDS_APP: &[&str] = &["BKRR_RESIDENCE", "BKRR_BYNAME"];

/// The fields a suite declares and an app does not.
pub const BKCR_FIELDS_SUITE: &[&str] = &["BKRR_RUNNER", "BKRR_TONGUE"];

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
    ("BKRR_KIND", &[BKCR_KIND_APP, BKCR_KIND_SUITE]),
    ("BKRR_SPEND", &["bknre_reader", "bknre_writer"]),
    ("BKRR_RUNNER", &["bknre_cargo", "bknre_nextest"]),
    ("BKRR_TONGUE", &["bknre_harness", "bknre_nextest"]),
];

/// Directories the walk never descends.
///
/// `.git` and a build directory hold no collar and cost the whole walk's time,
/// so they are stepped over by name. The list is short and stated rather than
/// guessed at: a walk that skipped by heuristic could step over a real collar
/// and answer that a name does not resolve.
const ZBKCR_UNWALKED: &[&str] = &[".git", "target"];

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

/// A collar as it stands, with the regime it declared.
#[derive(Debug, Clone)]
pub struct bkcr_Collar {
    /// The collar name, which is the instance directory's own name.
    pub name: String,
    /// The instance directory, repo-relative.
    pub instance: PathBuf,
    /// Everything the family file declared.
    pub regime: bklrc_Regime,
}

impl bkcr_Collar {
    /// One field's value, or the empty string where the field is absent. The
    /// absence is a finding of its own, so a reader here is never guessing.
    pub fn bkcr_field(&self, key: &str) -> &str {
        self.regime.bklrc_scalar(key).unwrap_or("")
    }

    /// Whether this collar declares an app.
    pub fn bkcr_app(&self) -> bool {
        self.bkcr_field("BKRR_KIND") == BKCR_KIND_APP
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
        match self.regime.bklrc_catena("BKRR_EXERGUE") {
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
        match self.regime.bklrc_catena("BKRR_STRIKE") {
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
        collar.bkcr_strike().join(" "),
    ))
}

/// A collar and every conformance finding against it.
#[derive(Debug, Clone)]
pub struct bkcr_Resolved {
    pub collar: bkcr_Collar,
    pub findings: Vec<String>,
}

/// Every instance directory the walk finds, repo-relative and sorted.
///
/// Sorted so that two drives over one tree report in one order — a walk whose
/// answer depended on the filesystem's own ordering would make a duplicate-name
/// refusal name its two paths differently on different stations.
pub fn bkcr_walk(repository: &Path) -> Result<Vec<PathBuf>, String> {
    bkcr_walk_family(repository, BKCR_FAMILY)
}

/// Every instance directory of ONE NAMED FAMILY the walk finds, repo-relative
/// and sorted.
///
/// PUBLISHED SO A SECOND FAMILY NEEDS NO SECOND WALK. The kibble family
/// (`bkcq_kibble`) is read from the same tree by the same rules, and a walk of
/// its own would be a second thing to keep in step — the unwalked list among
/// them, so that one tree could come to hold two answers about what stands in
/// it. What parts a family from another here is the file its instance carries
/// and nothing else, which is exactly what this parameter is.
pub fn bkcr_walk_family(repository: &Path, family: &str) -> Result<Vec<PathBuf>, String> {
    let mut found: Vec<PathBuf> = Vec::new();
    zbkcr_descend(repository, Path::new(""), family, &mut found)?;
    found.sort();
    Ok(found)
}

/// One level of the walk.
fn zbkcr_descend(
    repository: &Path,
    relative: &Path,
    family: &str,
    found: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let here = repository.join(relative);

    let listing = std::fs::read_dir(&here)
        .map_err(|err| format!("could not walk {}: {}", here.display(), err))?;

    for entry in listing {
        let entry = entry.map_err(|err| format!("could not walk {}: {}", here.display(), err))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let kind = entry
            .file_type()
            .map_err(|err| format!("could not read {}: {}", entry.path().display(), err))?;

        if kind.is_dir() {
            if ZBKCR_UNWALKED.contains(&name.as_str()) {
                continue;
            }
            zbkcr_descend(repository, &relative.join(&name), family, found)?;
            continue;
        }

        if name == family {
            found.push(relative.to_path_buf());
        }
    }

    Ok(())
}

/// Resolve a collar by name, and gather every finding against it.
///
/// A NAME FOUND TWICE REFUSES HERE rather than reporting a finding, and the
/// difference is not bookkeeping: a finding is something said ABOUT a collar,
/// and a name standing at two instances names no collar to say it about. The
/// refusal carries both paths, because the repair is to rename one of them and a
/// reader told only that the name is doubled has been handed a search.
pub fn bkcr_resolve(repository: &Path, name: &str) -> Result<bkcr_Resolved, String> {
    let instances = bkcr_walk(repository)?;

    let matched: Vec<&PathBuf> = instances
        .iter()
        .filter(|instance| zbkcr_named(instance) == name)
        .collect();

    if matched.is_empty() {
        return Err(format!(
            "no collar named '{}' stands in {} — the walk found {} collar(s): {}",
            name,
            repository.display(),
            instances.len(),
            zbkcr_listed(&instances)
        ));
    }

    if matched.len() > 1 {
        let mut said = format!(
            "the name '{}' stands at {} instances, so it names no collar — rename all but one, \
             a collar's name being its identity (BKSCL-Collar.adoc \"The Identity Law\")",
            name,
            matched.len()
        );
        for instance in &matched {
            said.push_str("\n  ");
            said.push_str(&instance.join(BKCR_FAMILY).display().to_string());
        }
        return Err(said);
    }

    let instance = matched[0].clone();
    let regime = bklrc_read(&repository.join(&instance).join(BKCR_FAMILY))?;

    let collar = bkcr_Collar {
        name: name.to_string(),
        instance,
        regime,
    };

    let findings = zbkcr_findings(repository, &collar);

    Ok(bkcr_Resolved { collar, findings })
}

/// Every conformance finding against one collar, gathered whole.
fn zbkcr_findings(repository: &Path, collar: &bkcr_Collar) -> Vec<String> {
    let mut findings: Vec<String> = Vec::new();

    zbkcr_declared(collar, &mut findings);
    zbkcr_identity(collar, &mut findings);
    zbkcr_muzzle(repository, collar, &mut findings);
    zbkcr_cargo(repository, collar, &mut findings);

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
        if !collar.regime.bklrc_holds(field) {
            findings.push(format!(
                "{} is not declared — every collar declares it, and a door reading a meaning out \
                 of an absence cannot tell a declaration from an omission",
                field
            ));
        }
    }

    for field in foreign {
        if collar.regime.bklrc_holds(field) {
            findings.push(format!(
                "{} is declared and belongs to the other kind — this collar declares {} = {}",
                field, "BKRR_KIND", kind
            ));
        }
    }

    for (field, admitted) in ZBKCR_ENUMS {
        let Some(held) = collar.regime.bklrc_scalar(field) else {
            continue;
        };
        if !admitted.contains(&held) {
            findings.push(format!(
                "{} carries '{}', which is no admitted value — it takes one of: {}",
                field,
                held,
                admitted.join(", ")
            ));
        }
    }

    zbkcr_exergue(collar, findings);

    if collar.regime.bklrc_holds("BKRR_MUZZLE") && collar.bkcr_field("BKRR_MUZZLE").is_empty() {
        findings.push(
            "BKRR_MUZZLE stands vacant — it is required and never vacant, the consuming tree's \
             own root being the ordinary value"
                .to_string(),
        );
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
    if !collar.regime.bklrc_holds("BKRR_EXERGUE") {
        return;
    }

    let exergues = collar.bkcr_exergue();
    let strikes = collar.bkcr_strike();

    if exergues.is_empty() && !strikes.is_empty() {
        findings.push(format!(
            "BKRR_STRIKE names {} and BKRR_EXERGUE declares {} — doors that strike nothing are \
             not owed, and the two fields are declared together or not at all",
            strikes.join(" "),
            BKCR_UNSTRUCK
        ));
    }

    if !exergues.is_empty() && strikes.is_empty() {
        findings.push(format!(
            "BKRR_EXERGUE names {} and BKRR_STRIKE stands vacant — a refusal over an absent \
             exergue exists to name the doors that write it, and one composed from a vacant field \
             would name nothing",
            exergues.join(" ")
        ));
    }
}

/// The identity law: the instance directory's name and the declared name are one
/// (BKSCL-Collar.adoc "The Identity Law").
fn zbkcr_identity(collar: &bkcr_Collar, findings: &mut Vec<String>) {
    let declared = collar.bkcr_field("BKRR_COLLAR");

    if declared != collar.name {
        findings.push(format!(
            "BKRR_COLLAR declares '{}' and the instance directory is named '{}' — a collar's name \
             is its identity, and the two must be one",
            declared, collar.name
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
            declared
        ));
        return;
    }

    if !directory.join(BKCR_MUZZLE_FILE).is_file() {
        findings.push(format!(
            "BKRR_MUZZLE names '{}', which holds no {} — a muzzle directory holding no lint list \
             is a muzzle nothing wears",
            declared, BKCR_MUZZLE_FILE
        ));
    }
}

/// The two proofs that need cargo: the elected roots against the local-dependency
/// closure, and the permitted crates against the manifest's direct dependencies.
///
/// CARGO IS THE ORACLE AND NOTHING HERE RECOMPUTES IT. What each manifest
/// declares is read back from cargo's own account of it; a reader that parsed
/// manifests itself would be a second resolver, and the whole election of this
/// backend is that nothing reimplements what cargo knows
/// (BKSNC-Kennelcraft.adoc "The Rust Backend").
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
/// and tattoo is as routine as an invocation gets (BKSNC-Kennelcraft.adoc
/// "Open Elections"). So each LOCAL manifest is asked about on its own, with no
/// closure resolved at any step, and the local edges are followed from one
/// answer to the next. Every registry package is thereby out of the question by
/// construction, which is also the honest scope: what the elected roots must
/// reach is the source compiled from OUR trees, and a registry crate is pinned
/// by the lock rather than elected by a collar.
fn zbkcr_cargo(repository: &Path, collar: &bkcr_Collar, findings: &mut Vec<String>) {
    let declared = collar.bkcr_field("BKRR_MANIFEST");

    if declared.is_empty() {
        return;
    }

    let manifest = repository.join(declared);

    if !manifest.is_file() {
        findings.push(format!(
            "BKRR_MANIFEST names '{}', where no manifest stands",
            declared
        ));
        return;
    }

    let account = match zbkcr_account(repository, &manifest) {
        Ok(account) => account,
        Err(err) => {
            findings.push(err);
            return;
        }
    };

    // A suite is compiled with the crate's dev-dependencies and an app is not,
    // so the kind decides what both proofs below are taken over.
    let tested = !collar.bkcr_app();

    zbkcr_crates(collar, &account, tested, findings);

    match bkcr_closure(repository, &manifest, tested) {
        Ok(closure) => zbkcr_roots(collar, &closure, findings),
        Err(err) => findings.push(err),
    }
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
pub fn bkcr_target_directory(repository: &Path, manifest: &Path) -> Result<PathBuf, String> {
    let answer = zbkcr_answer(repository, manifest)?;

    answer
        .bkcj_field("target_directory")
        .and_then(bkcj_Value::bkcj_string)
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!(
                "cargo's account of '{}' names no build directory",
                manifest.display()
            )
        })
}

/// Cargo's own account of ONE manifest, no closure resolved.
///
/// The package asked for is the one whose manifest path is the one asked about,
/// matched after both are settled: a manifest naming a workspace answers for its
/// members too, and the answer must be about the crate the collar declares.
fn zbkcr_account(repository: &Path, manifest: &Path) -> Result<bkcj_Value, String> {
    let answer = zbkcr_answer(repository, manifest)?;

    let packages = answer
        .bkcj_field("packages")
        .and_then(bkcj_Value::bkcj_array)
        .ok_or_else(|| format!("cargo's account of '{}' names no packages", manifest.display()))?;

    let wanted = zbkcr_settled(manifest).ok_or_else(|| {
        format!(
            "'{}' settles to no real path, so cargo's answer cannot be matched to it",
            manifest.display()
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
                manifest.display()
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
fn zbkcr_answer(repository: &Path, manifest: &Path) -> Result<bkcj_Value, String> {
    let recalled = bkcl_leash::bkcl_recall(
        repository,
        manifest,
        ZBKCR_METADATA,
        [ZBKCR_NO_DEPS, ZBKCR_FORMAT_FLAG, ZBKCR_FORMAT, ZBKCR_OFFLINE],
        &[],
    )
    .map_err(|err| format!("cargo could not be asked about '{}': {}", manifest.display(), err))?;

    // THE CODE IS READ BEFORE THE ANSWER IS, which is the shape the recalling
    // face asks of its callers: it answers `Ok` for a cargo that exited nonzero,
    // the refusal being about reaching cargo at all, so a caller that read the
    // answer first would parse whatever a failed run happened to leave behind.
    if !recalled.run.bkcl_landed() {
        return Err(format!(
            "cargo refused to describe '{}': {}",
            manifest.display(),
            recalled.grievance.trim()
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
            err
        )
    })?;

    let answer = bkcj_read(said).map_err(|err| {
        format!(
            "cargo's account of '{}' could not be read: {}",
            manifest.display(),
            err
        )
    })?;

    if answer.bkcj_field("version").and_then(bkcj_Value::bkcj_number) != Some(ZBKCR_FORMAT_VERSION)
    {
        return Err(format!(
            "cargo answered about '{}' in a schema this reading was not written against",
            manifest.display()
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
/// from the answer: no root of this collar's could elect it, and the leash
/// already refuses to build a crate that stands outside the tree it is bounded
/// by.
pub fn bkcr_closure(repository: &Path, manifest: &Path, tested: bool) -> Result<Vec<String>, String> {
    let mut walked: Vec<PathBuf> = Vec::new();
    let mut owed: Vec<(PathBuf, bool)> = vec![(manifest.to_path_buf(), true)];
    let mut local: Vec<String> = Vec::new();

    while let Some((next, own)) = owed.pop() {
        let settled = std::fs::canonicalize(&next).unwrap_or_else(|_| next.clone());
        if walked.contains(&settled) {
            continue;
        }
        walked.push(settled);

        let account = zbkcr_account(repository, &next)?;

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

/// Every LOCAL package in the closure is reached by an elected root.
///
/// A package standing in a tree of ours has its sources compiled into the
/// artifact this collar declares. So the elected roots must reach it, or the
/// position measured over those roots is a position that does not move when the
/// artifact does — a binary reading as current after the source it is made from
/// has changed.
///
/// REACHING IS EITHER DIRECTION. A root may name the package's own directory or
/// anything above it, and it may equally name paths INSIDE it, which is the
/// ordinary shape: the kennel's own collar elects `src`, `build.rs` and the
/// manifests rather than the crate directory whole, so that a landing touching
/// only its test modules moves no position.
fn zbkcr_roots(collar: &bkcr_Collar, closure: &[String], findings: &mut Vec<String>) {
    let Some(roots) = collar.regime.bklrc_catena("BKRR_ROOTS") else {
        return;
    };

    let elected: Vec<&str> = roots
        .iter()
        .copied()
        .filter(|root| !root.starts_with(':'))
        .collect();

    for directory in closure {
        if elected.iter().any(|root| zbkcr_reaches(root, directory)) {
            continue;
        }

        findings.push(format!(
            "BKRR_ROOTS elects nothing reaching '{}', where a local dependency stands — its \
             sources are compiled into what this collar declares, so a position measured over \
             these roots would not move when they do",
            directory
        ));
    }
}

/// Every direct dependency the manifest declares is named in exactly one of the
/// collar's two crate lists (BKSCL-Collar.adoc "The tamed and the feral").
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
    let Some(tamed) = collar.regime.bklrc_catena("BKRR_TAMED_CRATES") else {
        return;
    };

    // The none-sprue is not a crate name, so it leaves the reading here and the
    // union below is over crates alone.
    let feral: Vec<&str> = match collar.regime.bklrc_catena("BKRR_FERAL_CRATES") {
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
            named, BKCR_TAMED
        ));

        if tamed.contains(named) {
            findings.push(format!(
                "'{}' is named in BKRR_TAMED_CRATES and BKRR_FERAL_CRATES both — every direct \
                 dependency stands in exactly one of the two, and the pair the validate door \
                 proves disjoint cannot say both of one crate",
                named
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
            named
        ));
    }
}

/// A path as the filesystem settles it, so a symlink cannot make one path read
/// as two.
fn zbkcr_settled(path: &Path) -> Option<PathBuf> {
    std::fs::canonicalize(path).ok()
}

/// Whether an elected root reaches a directory, in either direction.
fn zbkcr_reaches(root: &str, directory: &str) -> bool {
    if root == directory {
        return true;
    }

    let inside = |outer: &str, inner: &str| {
        !outer.is_empty() && inner.starts_with(outer) && inner.as_bytes()[outer.len()] == b'/'
    };

    // A root at the repository root reaches everything, which is what an empty
    // relative directory means.
    if directory.is_empty() {
        return true;
    }

    inside(directory, root) || inside(root, directory)
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
