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
//! THE FIELD ROSTER STANDS HERE AND NOT IN THE READER CRATE. `bkl` reads
//! regime files and stops, by its own charter: nothing in it knows what a collar
//! is, and nothing in it may learn what a kibble is either. What a family's
//! fields MEAN belongs to the family that declared them, which for this family
//! is this module — the same seat the rust family's roster takes in
//! `bkcr_resolve`.

use std::path::{Path, PathBuf};

use crate::bkcl_leash;
use crate::bkcr_resolve::bkcr_walk_family;
use bkl::bklrc_catena::{bklrc_read, bklrc_Regime};

/// The kibble family's file. Its parent directory is the instance and that
/// directory's name is the kibble's name, on the rust family's precedent
/// (`bkcr_resolve`, and BKSCL-Collar.adoc "Container and Discovery").
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
/// plane, as every enum-valued field of this kit wears
/// (BKSCL-Collar.adoc "The Family Legend").
///
/// BOTH ARE ADMITTED AND ONE IS CONVERGEABLE, and the split is deliberate. The
/// roster a validator checks a DECLARATION against is the design's, and the
/// design carries two kinds (BKSBL-Kibble.adoc "Two Kinds"); what is unbuilt
/// is the prebuilt CONVERGE, and heel is where that fact is stated. A validator
/// that refused the declaration would be refusing the sheaf rather than
/// reporting the code, and would say nothing about which act is missing.
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
    pub regime: bklrc_Regime,
}

impl bkcq_Kibble {
    /// One field's value, or the empty string where the field is absent. The
    /// absence is a finding of its own, so a reader here is never guessing.
    pub fn bkcq_field(&self, key: &str) -> &str {
        self.regime.bklrc_scalar(key).unwrap_or("")
    }

    /// Whether this kibble is built from source rather than fetched prebuilt.
    pub fn bkcq_source(&self) -> bool {
        self.bkcq_field("BKRK_KIND") == BKCQ_KIND_SOURCE
    }

    /// Where this kibble's binary stands once placed.
    ///
    /// THE VERSION IS IN THE PATH, which is what makes the pin a path test: a
    /// door asks the filesystem whether the declared version stands, and gets
    /// an answer no record can contradict. A residence keyed on the name alone
    /// would answer "something is here" and would go on answering it after a
    /// bump, which is the drift the whole regime exists to close.
    ///
    /// THE TACKROOM IS ASKED FOR RATHER THAN DERIVED. It is the station's own
    /// store, carried across the dispatch exec boundary, and a kennel that fell
    /// back to a personal tool home would be placing a pinned artifact where
    /// the station's other clones cannot see it and the fence does not reach.
    pub fn bkcq_residence(&self) -> Result<PathBuf, String> {
        let tackroom = std::env::var(bkcl_leash::BKCL_TACKROOM_VAR)
            .ok()
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| self.bkcq_unstationed())?;

        Ok(self.bkcq_residence_at(Path::new(&tackroom)))
    }

    /// Where this kibble's binary stands under a NAMED store.
    ///
    /// THE PURE HALF, AND IT IS PURE SO IT CAN BE HURDLED. The composition above
    /// reads the station out of the process's environment, which a hurdle could
    /// only pose by WRITING that environment — and a written environment is
    /// shared by every hurdle the harness is running in parallel, so a hurdle
    /// posing a store would be deciding its neighbours' answers. That is not a
    /// hypothetical: it was driven, and it reddened an election hurdle three
    /// files away that passed alone and failed in company. So the reading is
    /// split at the edge, exactly where the composition is split from the drive
    /// everywhere else in this kit — what a hurdle poses is a value, and the one
    /// line that consults the environment consults it nowhere else.
    pub fn bkcq_residence_at(&self, tackroom: &Path) -> PathBuf {
        tackroom
            .join(ZBKCQ_QUARTER)
            .join(&self.name)
            .join(self.bkcq_field("BKRK_VERSION"))
            .join(self.bkcq_field("BKRK_BYNAME"))
    }

    /// The refusal a kibble gives where no tackroom is declared.
    ///
    /// Composed rather than spelled at the one site that raises it, so the
    /// sentence a station meets is the sentence a hurdle proves.
    pub fn bkcq_unstationed(&self) -> String {
        format!(
            "{} is unset, so the kibble '{}' has no residence to stand at. A kibble is placed in \
             the station's shared tackroom and NEVER in a personal tool home: the fence that \
             declares the store is Tools/bkk/bk0/bkcb_tackroom.sh, reached through the whistle, and a \
             kennel reaching an absent store was not launched through it",
            bkcl_leash::BKCL_TACKROOM_VAR,
            self.name
        )
    }

    /// Whether the declared version stands at its residence.
    ///
    /// THE WHOLE QUESTION IS A PATH TEST, and every door but heel answers it and
    /// stops. Nothing here spawns the binary to ask what version it is: the
    /// version is in the path, so a binary standing there IS the declared one or
    /// the path is not the one this composed — and a door that asked the program
    /// instead would be trusting the program to answer honestly about itself.
    pub fn bkcq_placed(&self) -> Result<bool, String> {
        Ok(self.bkcq_residence()?.is_file())
    }

    /// The refusal every door but heel gives for a kibble that is not placed.
    ///
    /// COMPOSED IN ONE PLACE, because a refusal spelled at each door is a
    /// sentence that drifts: the doors would come to name different remedies for
    /// one condition, and the remedy is the only part of a refusal a reader
    /// acts on.
    pub fn bkcq_absent(&self) -> Result<String, String> {
        let residence = self.bkcq_residence()?;

        Ok(format!(
            "the kibble '{}' declares {} {} and nothing stands at {} — the kennel spawns a program \
             it did not build only from its own residence, never from whatever the station's path \
             happens to hold. The converge is heel, which is the one door that writes the \
             tackroom: {} {}",
            self.name,
            self.bkcq_field("BKRK_PROGRAM"),
            self.bkcq_field("BKRK_VERSION"),
            residence.display(),
            ZBKCQ_HEEL,
            self.name
        ))
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
    /// one digest per version rather than a per-platform table kept true by hand
    /// (BKSBL-Kibble.adoc "Prebuilt").
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

/// The door named in every not-placed refusal. Spelled once so the remedy a
/// reader is handed cannot come to differ from the door that performs it.
const ZBKCQ_HEEL: &str = "heel";

/// A kibble and every conformance finding against it.
#[derive(Debug, Clone)]
pub struct bkcq_Resolved {
    pub kibble: bkcq_Kibble,
    pub findings: Vec<String>,
}

/// Every kibble instance the walk finds, repo-relative and sorted.
///
/// THE WALK IS THE COLLAR RESOLVER'S, PARAMETERIZED, and is not a second one.
/// Two walks over one tree are two things to keep in step — the unwalked list
/// among them — and a kibble found by a walk that stepped over a directory the
/// collar walk descends would make one tree hold two answers about what stands
/// in it.
pub fn bkcq_walk(repository: &Path) -> Result<Vec<PathBuf>, String> {
    bkcr_walk_family(repository, BKCQ_FAMILY)
}

/// Resolve a kibble by name, and gather every finding against it.
///
/// A NAME FOUND TWICE REFUSES HERE rather than reporting a finding, on the
/// resolver's own ground: a finding is something said ABOUT a kibble, and a name
/// standing at two instances names no kibble to say it about.
pub fn bkcq_resolve(repository: &Path, name: &str) -> Result<bkcq_Resolved, String> {
    let instances = bkcq_walk(repository)?;

    let matched: Vec<&PathBuf> = instances
        .iter()
        .filter(|instance| zbkcq_named(instance) == name)
        .collect();

    if matched.is_empty() {
        return Err(format!(
            "no kibble named '{}' stands in {} — the walk found {} kibble(s): {}",
            name,
            repository.display(),
            instances.len(),
            zbkcq_listed(&instances)
        ));
    }

    if matched.len() > 1 {
        let mut said = format!(
            "the name '{}' stands at {} instances, so it names no kibble — rename all but one, a \
             kibble's name being its identity",
            name,
            matched.len()
        );
        for instance in &matched {
            said.push_str("\n  ");
            said.push_str(&instance.join(BKCQ_FAMILY).display().to_string());
        }
        return Err(said);
    }

    let instance = matched[0].clone();
    let regime = bklrc_read(&repository.join(&instance).join(BKCQ_FAMILY))?;

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
        if !kibble.regime.bklrc_holds(field) {
            findings.push(format!(
                "{} is not declared — every kibble of this kind declares it, and a door reading a \
                 meaning out of an absence cannot tell a declaration from an omission",
                field
            ));
        }
    }

    for field in foreign {
        if kibble.regime.bklrc_holds(field) {
            findings.push(format!(
                "{} is declared and belongs to the {} kind — this kibble declares {} = {}, and \
                 the two kinds are fetched by roads that share no fact: a source kibble has a \
                 build to declare things about and no release store, a prebuilt one the reverse",
                field, elsewhere, "BKRK_KIND", kind
            ));
        }
    }

    for (field, admitted) in ZBKCQ_ENUMS {
        let Some(held) = kibble.regime.bklrc_scalar(field) else {
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
        if kibble.regime.bklrc_holds(field) && kibble.bkcq_field(field).trim().is_empty() {
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
    let Some(seal) = kibble.regime.bklrc_scalar("BKRK_SEAL") else {
        return;
    };

    let seal = seal.trim();

    if seal.len() != ZBKCQ_SEAL_WIDTH {
        findings.push(format!(
            "BKRK_SEAL carries {} character(s) and a sha256 is {} — the seal is the archive's \
             sha256 and no other digest is kept (BKSBL-Kibble.adoc \"The Seal\")",
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
