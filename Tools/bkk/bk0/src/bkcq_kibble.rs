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

//! The kibble — the declaration for a program the kennel spawns and did not
//! build, and the reading of one.
//!
//! A KENNEL THAT BUILDS WHAT IT LAUNCHES CAN ANSWER FOR IT: the position is
//! measured, the manifest is locked, the toolchain is pinned. A program it
//! spawns and did NOT build breaks that chain at its first link — nothing in
//! the repository says which artifact it is, and a door that trusts it trusts
//! whatever the station happens to hold. A kibble is what closes that: which
//! thing, which version, and one seal, declared once and pointed at.
//!
//! THIS MODULE READS AND NEVER WRITES. The residence is composed here and
//! TESTED here, and the one door that puts a binary at it is *heel*
//! (`bkch_heel`), which is where the tackroom's single writer stands. Every
//! other door reaching a kibble asks this module whether the declared version
//! stands and refuses naming heel where it does not.
//!
//! THE PIN IS A PATH TEST AND NOT A RECORD, which is why the version rides in
//! the residence path. Whether the declared version stands is answered by
//! looking at the filesystem, so no record can drift out of agreement with what
//! is on disk — the failure a version written into a manifest somewhere invites,
//! and the reason a probe of the station's own PATH is refused outright.
//!
//! THE FIELD ROSTER STANDS HERE AND NOT IN THE READER CRATE. `bkp` reads
//! regime files and stops, by its own charter: nothing in it knows what a collar
//! is, and nothing in it may learn what a kibble is either. What a family's
//! fields MEAN belongs to the family that declared them, which for this family
//! is this module — the same seat the rust family's roster takes in
//! `bkcr_resolve`.

use std::path::{Path, PathBuf};

use crate::bkcl_leash;
use crate::bkco_output::{bkco_call, BKCO_VERB_HEEL};
use crate::bkcr_resolve::{bkcr_drift, bkcr_weaned_family};
use bkp::bkprc_catena::{bkprc_read, bkprc_Regime};

/// The kibble family's file. Its parent directory is the instance and that
/// directory's name is the kibble's name, on the rust family's precedent
/// (`bkcr_resolve`).
pub const BKCQ_FAMILY: &str = "bkrk.env";

/// The fields every kibble declares, whatever its kind.
pub const BKCQ_FIELDS_BOTH: &[&str] = &[
    "BKRK_KIBBLE",
    "BKRK_KIND",
    "BKRK_PROGRAM",
    "BKRK_VERSION",
    "BKRK_SEAL",
    "BKRK_BYNAME",
];

/// The fields a source kibble declares and a prebuilt one does not.
///
/// They are the build's own facts: which registry the archive is fetched from,
/// which channel it is built at, which target cargo strikes, and which features
/// are asked for. A prebuilt kibble has no build, so declaring any of them
/// would be declaring something about an act that never happens.
pub const BKCQ_FIELDS_SOURCE: &[&str] = &[
    "BKRK_REGISTRY",
    "BKRK_CHANNEL",
    "BKRK_TARGET",
    "BKRK_FEATURES",
];

/// The fields a prebuilt kibble declares and a source one does not.
///
/// They are the vendor release's own facts: which store the release stands in,
/// and what the document enumerating its archives is called there. A source
/// kibble is fetched from a registry by a path that registry's layout fixes, so
/// it has no release store to name and no document to be told the name of.
///
/// THE VERSION IS IN NEITHER OF THEM, on the source kind's own law: the door
/// joins the declared version between the store and the document, so a bump is
/// one line and cannot leave a stale URL standing behind it.
pub const BKCQ_FIELDS_PREBUILT: &[&str] = &["BKRK_LARDER", "BKRK_BORDEREAU"];

/// The kind values, in the substrate's enum sprue form under this kit's own
/// plane, as every enum-valued field of this kit wears.
///
/// BOTH ARE ADMITTED AND ONE IS CONVERGEABLE, and the split is deliberate. The
/// roster a validator checks a DECLARATION against is the design's, and the
/// design carries two kinds; what is unbuilt is the prebuilt CONVERGE, and heel
/// is where that fact is stated. A validator that refused the declaration would
/// be refusing the sheaf rather than reporting the code, and would say nothing
/// about which act is missing.
pub const BKCQ_KIND_SOURCE: &str = "bknre_source";
pub const BKCQ_KIND_PREBUILT: &str = "bknre_prebuilt";

/// The enum-valued fields and the values each admits.
const ZBKCQ_ENUMS: &[(&str, &[&str])] =
    &[("BKRK_KIND", &[BKCQ_KIND_SOURCE, BKCQ_KIND_PREBUILT])];

/// The kennel's own quarter of the tackroom, under which every kibble residence
/// stands.
///
/// THE TACKROOM IS THE STATION'S AND SHARED, so a kennel-owned residence says
/// whose it is in its path rather than standing loose beside the toolchain store
/// and the registry cache that already live there.
const ZBKCQ_QUARTER: &str = "bkk";

/// The kennel's own variable carrying a kibble root posed for one invocation.
///
/// A HURDLE POSES IT AND NOTHING ELSE DOES. A real launch states none and the
/// dispatch composes none, so absent it the root is the kennel's quarter of the
/// tackroom — the one zero this form carries, and the store every real kibble
/// stands in. Posed, it roots kibble residences and the kibble lock and nothing
/// more: the interpreter store, the uv cache and the toolchain store are composed
/// from the tackroom whatever this says, since each is shared by content and a
/// hurdle owning one would re-provision what the station already holds.
pub const BKCQ_FEEDROOM_VAR: &str = "BKCQ_FEEDROOM";

/// Where kibble residences stand, and where the lock their converge takes
/// stands: the posed root, or the kennel's quarter of the tackroom.
///
/// A TYPE OF ITS OWN, AND NEVER A PATH. A tackroom and a kibble root are both
/// directories, and a door handed one where the other belongs composes a
/// residence that stands nowhere it should — a hurdle's root read as a tackroom
/// would seat an interpreter store under it, and a tackroom read as a root would
/// land kibble beside the toolchains rather than in the kennel's quarter. No
/// conversion leads to this from a path or from this to a path, so the mistake
/// does not compile:
///
/// ```compile_fail
/// fn tackroom(_: &std::path::Path) {}
/// let feedroom =
///     bkk::bkcq_kibble::bkcq_feedroom_at(None, None, Some(std::path::Path::new("/station")))
///         .unwrap();
/// tackroom(&feedroom);
/// ```
///
/// and the same reading handed where a root belongs does:
///
/// ```
/// fn root(_: &bkk::bkcq_kibble::bkcq_Feedroom) {}
/// let feedroom =
///     bkk::bkcq_kibble::bkcq_feedroom_at(None, None, Some(std::path::Path::new("/station")))
///         .unwrap();
/// root(&feedroom);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkcq_Feedroom {
    root: PathBuf,
}

impl bkcq_Feedroom {
    /// The directory this root is, for the converge that writes beneath it.
    pub(crate) fn bkcq_root(&self) -> &Path {
        &self.root
    }
}

/// The kibble root this invocation stands on, read out of the environment.
///
/// THE ONE AMBIENT READER, and every door reads it once at its face: the
/// converge, the derby's preflight and the launch-side placement test take what
/// it answered rather than asking again, so one invocation cannot compose two
/// roots. A posed root is announced, since an invocation standing on one is
/// standing somewhere other than the station's store.
pub fn bkcq_feedroom() -> Result<bkcq_Feedroom, String> {
    let posed = zbkcq_stated(BKCQ_FEEDROOM_VAR);
    let temp = zbkcq_stated(crate::bkcc_record::BKCC_TEMP_DIR_VAR);
    let tackroom = zbkcq_stated(bkcl_leash::BKCL_TACKROOM_VAR);

    let feedroom = bkcq_feedroom_at(
        posed.as_deref().map(Path::new),
        temp.as_deref().map(Path::new),
        tackroom.as_deref().map(Path::new),
    )?;

    if posed.is_some() {
        crate::bkco_info_now!(
            "the kibble root is posed at {} by {}, so no residence this invocation reads or \
             places stands in the station's store",
            feedroom.root.display(),
            BKCQ_FEEDROOM_VAR
        );
    }

    Ok(feedroom)
}

/// The kibble root composed from NAMED values: the posed root if any, the
/// invocation's dispatch temp directory, and the station's tackroom.
///
/// THE PURE HALF, AND IT IS PURE SO IT CAN BE HURDLED. Posing a root by writing
/// the process's environment would decide the answer of every hurdle the harness
/// runs beside the one that wrote it, so what a hurdle poses is a value and the
/// one line consulting the environment stands above.
///
/// A POSED ROOT IS ADMITTED ONLY BENEATH THE DISPATCH TEMP DIRECTORY, which is
/// unique to one invocation and is what parts two hurdles' roots from each other
/// and from the station's store. One anywhere else refuses rather than being
/// followed: a root that could name the tackroom would be the shared store again
/// under a second name, and the converge's lock would be shared with it. The
/// containment is read over the standing ancestors resolved, since the root
/// itself is made by the converge and stands nowhere before it, and a parent
/// step is refused outright rather than reasoned about.
pub fn bkcq_feedroom_at(
    posed: Option<&Path>,
    temp: Option<&Path>,
    tackroom: Option<&Path>,
) -> Result<bkcq_Feedroom, String> {
    let Some(root) = posed else {
        let tackroom = tackroom.ok_or_else(zbkcq_unstationed)?;
        return Ok(bkcq_Feedroom {
            root: tackroom.join(ZBKCQ_QUARTER),
        });
    };

    let Some(temp) = temp else {
        return Err(format!(
            "{} poses a kibble root at {} and no dispatch temp directory is declared, so there is \
             nothing to admit it beneath. A posed root stands beneath the invocation's own temp \
             directory or nowhere: drive the kennel through its own door",
            BKCQ_FEEDROOM_VAR,
            root.display()
        ));
    };

    let stepped = root
        .components()
        .any(|part| part == std::path::Component::ParentDir);

    let resolved = zbkcq_resolved(root);
    let within = zbkcq_resolved(temp);
    let beneath = resolved.starts_with(&within) && resolved != within;

    if !root.is_absolute() || stepped || !beneath {
        return Err(format!(
            "{} poses a kibble root at {}, which does not stand beneath this invocation's dispatch \
             temp directory {} — refused rather than followed. A posed root is one invocation's \
             own, and one standing anywhere else could be the station's shared store under a \
             second name, which is the race a posed root exists to end",
            BKCQ_FEEDROOM_VAR,
            root.display(),
            temp.display()
        ));
    }

    Ok(bkcq_Feedroom {
        root: root.to_path_buf(),
    })
}

/// One variable's value, absent where it is unset or blank.
fn zbkcq_stated(variable: &str) -> Option<String> {
    std::env::var(variable)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

/// A path with as much of it resolved as actually stands, the rest left as
/// spelled — the containment reading `bkch_heel` takes for an install target,
/// and for its reason: the root is made by the converge, so the ordinary case is
/// the one where resolving the whole path fails.
fn zbkcq_resolved(path: &Path) -> PathBuf {
    if let Ok(real) = std::fs::canonicalize(path) {
        return real;
    }

    match (path.parent(), path.file_name()) {
        (Some(parent), Some(name)) => zbkcq_resolved(parent).join(name),
        _ => path.to_path_buf(),
    }
}

/// The refusal a kibble root gives where no tackroom is declared and no root is
/// posed.
///
/// Composed rather than spelled at the one site that raises it, so the
/// sentence a station meets is the sentence a hurdle proves.
fn zbkcq_unstationed() -> String {
    format!(
        "{} is unset, so no kibble has a residence to stand at. A kibble is placed in the \
         station's shared tackroom and NEVER in a personal tool home: the fence that declares the \
         store is Tools/bkk/bk0/bkcb_tackroom.sh, reached through the whistle, and a kennel \
         reaching an absent store was not launched through it",
        bkcl_leash::BKCL_TACKROOM_VAR
    )
}

/// The kibble that serves nextest, named here because the kennel spawns nextest
/// for a collar whose runner declares it and no collar names the kibble.
///
/// THE POINTER IS THE KENNEL'S AND THE VERSION IS THE KIBBLE'S, which is the
/// whole shape of the retirement this constant records. A version constant once
/// stood beside this one and was the kennel's own claim about a station; what
/// stands now is a name, and the version it resolves to is read from the
/// declaration every time. So a bumped pin is one edit in one file, and nothing
/// in this crate can go on naming a version the kibble no longer declares.
pub const BKCQ_NEXTEST: &str = "bki_nextest";

/// A kibble as it stands, with the regime it declared.
#[derive(Debug, Clone)]
pub struct bkcq_Kibble {
    /// The kibble's name, which is the instance directory's own name.
    pub name: String,
    /// The instance directory, repo-relative.
    pub instance: PathBuf,
    /// Everything the family file declared.
    pub regime: bkprc_Regime,
}

impl bkcq_Kibble {
    /// One field's value, or the empty string where the field is absent. The
    /// absence is a finding of its own, so a reader here is never guessing.
    pub fn bkcq_field(&self, key: &str) -> &str {
        self.regime.bkprc_scalar(key).unwrap_or("")
    }

    /// Whether this kibble is built from source rather than fetched prebuilt.
    pub fn bkcq_source(&self) -> bool {
        self.bkcq_field("BKRK_KIND") == BKCQ_KIND_SOURCE
    }

    /// Where this kibble's binary stands once placed, beneath a kibble root.
    ///
    /// THE VERSION IS IN THE PATH, which is what makes the pin a path test: a
    /// door asks the filesystem whether the declared version stands, and gets
    /// an answer no record can contradict. A residence keyed on the name alone
    /// would answer "something is here" and would go on answering it after a
    /// bump, which is the drift the whole regime exists to close.
    ///
    /// THE ROOT IS HANDED IN AND NEVER READ HERE. It is read once at a door's
    /// face (`bkcq_feedroom`), so every residence one invocation composes stands
    /// beneath one root, and a hurdle hands the root it owns as a value.
    pub fn bkcq_residence(&self, feedroom: &bkcq_Feedroom) -> PathBuf {
        feedroom
            .root
            .join(&self.name)
            .join(self.bkcq_field("BKRK_VERSION"))
            .join(self.bkcq_field("BKRK_BYNAME"))
    }

    /// Whether the declared version stands at its residence.
    ///
    /// THE WHOLE QUESTION IS A PATH TEST, and every door but heel answers it and
    /// stops. Nothing here spawns the binary to ask what version it is: the
    /// version is in the path, so a binary standing there IS the declared one or
    /// the path is not the one this composed — and a door that asked the program
    /// instead would be trusting the program to answer honestly about itself.
    pub fn bkcq_placed(&self, feedroom: &bkcq_Feedroom) -> bool {
        self.bkcq_residence(feedroom).is_file()
    }

    /// The refusal every door but heel gives for a kibble that is not placed.
    ///
    /// COMPOSED IN ONE PLACE, because a refusal spelled at each door is a
    /// sentence that drifts: the doors would come to name different remedies for
    /// one condition, and the remedy is the only part of a refusal a reader
    /// acts on.
    pub fn bkcq_absent(&self, feedroom: &bkcq_Feedroom) -> String {
        let residence = self.bkcq_residence(feedroom);

        format!(
            "the kibble '{}' declares {} {} and nothing stands at {} — the kennel spawns a program \
             it did not build only from its own residence, never from whatever the station's path \
             happens to hold. The converge is the one door that writes the tackroom: {}",
            self.name,
            self.bkcq_field("BKRK_PROGRAM"),
            self.bkcq_field("BKRK_VERSION"),
            residence.display(),
            bkco_call(BKCO_VERB_HEEL, Some(&self.name))
        )
    }

    /// Where the archive this kibble seals is fetched from.
    ///
    /// CARGO'S OWN REGISTRY LAYOUT, RESTATED AND CHECKED RATHER THAN TRUSTED.
    /// The path a published crate archive stands at under a registry's static
    /// host is that registry's shape and not ours — the Palisade, where our
    /// rules do not reach — so it is composed here from the declared base and
    /// then PROVEN by the seal: a layout that moved fetches nothing, or fetches
    /// something whose digest does not answer, and either way the refusal is
    /// loud rather than a silently wrong artifact.
    ///
    /// THE VERSION IS SPELLED FROM THE DECLARATION AND NEVER TYPED INTO THE
    /// BASE, which is why the base is a field and the join is code. A registry
    /// URL carrying the version would be a second home for the pin, and one a
    /// reader bumping the version would have no reason to look at.
    pub fn bkcq_archive(&self) -> String {
        let program = self.bkcq_field("BKRK_PROGRAM");

        format!(
            "{}/{}/{}-{}.crate",
            self.bkcq_field("BKRK_REGISTRY").trim_end_matches('/'),
            program,
            program,
            self.bkcq_field("BKRK_VERSION")
        )
    }

    /// Where this kibble's vendor bordereau stands — the document naming every
    /// archive of the release by rust target triple, with its digest.
    ///
    /// THE SEAL IS OVER THIS AND NOT OVER AN ARCHIVE, which is the whole shape
    /// of the prebuilt kind. One hash answers for a document that already
    /// answers for every archive the vendor published, so the kennel maintains
    /// one digest per version rather than a per-platform table kept true by hand.
    pub fn bkcq_bordereau(&self) -> String {
        self.bkcq_beside(self.bkcq_field("BKRK_BORDEREAU"))
    }

    /// Where one named file of this release stands — the bordereau's own
    /// sibling under the declared store and version.
    ///
    /// THE ARCHIVE IS REACHED AS A SIBLING OF THE DOCUMENT THAT NAMED IT, which
    /// is what keeps a vendor's file names out of this kennel entirely: the
    /// bordereau states the name, this states where a name of that release
    /// stands, and nothing here spells either.
    pub fn bkcq_beside(&self, name: &str) -> String {
        format!(
            "{}/{}/{}",
            self.bkcq_field("BKRK_LARDER").trim_end_matches('/'),
            self.bkcq_field("BKRK_VERSION"),
            name
        )
    }

    /// The directory the archive unpacks into, which is cargo's own naming for a
    /// published crate: the program and its version, joined by a hyphen.
    pub fn bkcq_unpacked(&self) -> String {
        format!(
            "{}-{}",
            self.bkcq_field("BKRK_PROGRAM"),
            self.bkcq_field("BKRK_VERSION")
        )
    }
}

/// A kibble and every conformance finding against it.
#[derive(Debug, Clone)]
pub struct bkcq_Resolved {
    pub kibble: bkcq_Kibble,
    pub findings: Vec<String>,
}

/// Every kibble instance the weaned set holds, repo-relative where the set stands
/// inside the repository, and sorted — the collar resolver's reading,
/// parameterized, and not a second one.
pub fn bkcq_walk(repository: &Path, weaned: &Path) -> Result<Vec<PathBuf>, String> {
    bkcr_weaned_family(repository, weaned, BKCQ_FAMILY)
}

/// Resolve a kibble by name, and gather every finding against it.
///
/// THE INSTANCE IS PROVEN AGAINST ITS BROOD BEFORE ANYTHING ELSE IS READ, on the
/// collar resolver's own ground: the findings of a copy that is not the kit's
/// declaration would be findings about a declaration nobody made.
pub fn bkcq_resolve(repository: &Path, weaned: &Path, name: &str) -> Result<bkcq_Resolved, String> {
    let instances = bkcq_walk(repository, weaned)?;

    let Some(instance) = instances
        .iter()
        .find(|instance| zbkcq_named(instance) == name)
        .cloned()
    else {
        return Err(format!(
            "no kibble named '{}' stands in the weaned set {} — it holds {} kibble(s): {}",
            name,
            weaned.display(),
            instances.len(),
            zbkcq_listed(&instances)
        ));
    };

    bkcr_drift(repository, &repository.join(&instance), BKCQ_FAMILY)?;

    let regime = bkprc_read(&repository.join(&instance).join(BKCQ_FAMILY))?;

    let kibble = bkcq_Kibble {
        name: name.to_string(),
        instance,
        regime,
    };

    let findings = bkcq_findings(&kibble);

    Ok(bkcq_Resolved { kibble, findings })
}

/// Every conformance finding against one kibble, gathered whole.
///
/// EVERY FINDING IS REPORTED BEFORE ANYTHING REFUSES, which is the collar
/// resolver's discipline held over this family: a first-finding exit hands its
/// reader one repair at a time and a fresh drive between each.
pub fn bkcq_findings(kibble: &bkcq_Kibble) -> Vec<String> {
    let mut findings: Vec<String> = Vec::new();

    zbkcq_declared(kibble, &mut findings);
    zbkcq_identity(kibble, &mut findings);
    zbkcq_seal(kibble, &mut findings);

    findings
}

/// Which fields must stand, which must not, and which values each enum admits.
///
/// The kind decides the roster, which is why the kind is a declared field rather
/// than something inferred from what else the file happens to carry: a validator
/// that guessed the kind from the fields present could never report a MISSING
/// one.
fn zbkcq_declared(kibble: &bkcq_Kibble, findings: &mut Vec<String>) {
    let kind = kibble.bkcq_field("BKRK_KIND");

    // THE FOREIGN ROSTER CARRIES THE NAME OF THE KIND THAT OWNS IT, so the
    // finding tells its reader where the field belongs rather than only that it
    // does not belong here — which is the difference between a repair and a
    // deletion.
    let (owed, foreign, elsewhere): (&[&str], &[&str], &str) = match kind {
        BKCQ_KIND_SOURCE => (BKCQ_FIELDS_SOURCE, BKCQ_FIELDS_PREBUILT, "prebuilt"),
        BKCQ_KIND_PREBUILT => (BKCQ_FIELDS_PREBUILT, BKCQ_FIELDS_SOURCE, "source"),
        _ => (&[], &[], ""),
    };

    for field in BKCQ_FIELDS_BOTH.iter().chain(owed.iter()) {
        if !kibble.regime.bkprc_holds(field) {
            findings.push(format!(
                "{} is not declared — every kibble of this kind declares it, and a door reading a \
                 meaning out of an absence cannot tell a declaration from an omission",
                field
            ));
        }
    }

    for field in foreign {
        if kibble.regime.bkprc_holds(field) {
            findings.push(format!(
                "{} is declared and belongs to the {} kind — this kibble declares {} = {}, and \
                 the two kinds are fetched by roads that share no fact: a source kibble has a \
                 build to declare things about and no release store, a prebuilt one the reverse",
                field, elsewhere, "BKRK_KIND", kind
            ));
        }
    }

    for (field, admitted) in ZBKCQ_ENUMS {
        let Some(held) = kibble.regime.bkprc_scalar(field) else {
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

    for field in ["BKRK_VERSION", "BKRK_PROGRAM", "BKRK_BYNAME"] {
        if kibble.regime.bkprc_holds(field) && kibble.bkcq_field(field).trim().is_empty() {
            findings.push(format!(
                "{} stands vacant — the residence is composed from these three, so a vacant one \
                 makes the pin a path that answers for nothing",
                field
            ));
        }
    }
}

/// The identity law: the instance directory's name and the declared name are
/// one, on the collar family's own precedent.
fn zbkcq_identity(kibble: &bkcq_Kibble, findings: &mut Vec<String>) {
    let declared = kibble.bkcq_field("BKRK_KIBBLE");

    if declared != kibble.name {
        findings.push(format!(
            "BKRK_KIBBLE declares '{}' and the instance directory is named '{}' — a kibble's name \
             is its identity, and a declaration disagreeing with the directory it stands in leaves \
             a reader no way to say which one a collar meant",
            declared, kibble.name
        ));
    }
}

/// The seal's own shape, which is the one field a validator can check without
/// fetching anything.
///
/// A MALFORMED SEAL IS CAUGHT BEFORE THE FETCH, deliberately. The seal is
/// proven against an archive at the converge, which is minutes of download away;
/// a digest that is the wrong length or carries a character no hex digest can
/// carry is wrong whatever the archive holds, and saying so at validation costs
/// nothing and saves the download.
fn zbkcq_seal(kibble: &bkcq_Kibble, findings: &mut Vec<String>) {
    let Some(seal) = kibble.regime.bkprc_scalar("BKRK_SEAL") else {
        return;
    };

    let seal = seal.trim();

    if seal.len() != ZBKCQ_SEAL_WIDTH {
        findings.push(format!(
            "BKRK_SEAL carries {} character(s) and a sha256 is {} — the seal is the archive's \
             sha256 and no other digest is kept",
            seal.len(),
            ZBKCQ_SEAL_WIDTH
        ));
        return;
    }

    if !seal.chars().all(|held| held.is_ascii_hexdigit() && !held.is_ascii_uppercase()) {
        findings.push(
            "BKRK_SEAL carries a character no lowercase hex digest carries — openssl answers in \
             lowercase hex, and a seal compared against that answer must be spelled the way the \
             answer is spelled"
                .to_string(),
        );
    }
}

/// A sha256 as openssl spells it: 64 lowercase hex digits.
const ZBKCQ_SEAL_WIDTH: usize = 64;

/// An instance directory's own name, which is the kibble's name.
fn zbkcq_named(instance: &Path) -> String {
    instance
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Every instance the walk found, for a refusal that hands its reader the roster
/// rather than a search.
fn zbkcq_listed(instances: &[PathBuf]) -> String {
    if instances.is_empty() {
        return "none".to_string();
    }

    instances
        .iter()
        .map(|instance| zbkcq_named(instance))
        .collect::<Vec<String>>()
        .join(", ")
}

// eof
