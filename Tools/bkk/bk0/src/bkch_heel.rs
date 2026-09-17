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

//! *heel* — the converge (BKSNC-Kennelcraft.adoc "The doors").
//!
//! THE ONE DOOR THAT CONVERGES, AND NOTHING ELSE DOES. Every other door reports
//! and refuses: the launch seam meeting an outrun binary names this act and
//! stops, because a door that quietly rebuilt what a caller asked it to run
//! would make a build happen at the moment least expected and hide the vintage
//! question behind it. Heel is where that act is performed deliberately, by a
//! caller who typed it.
//!
//! IT IS AN APP'S DOOR ALONE. A suite is never borrowed — its runner compiles it
//! from the source the seat holds, so there is no standing artifact between the
//! source and the verdict for a converge to repair — and a suite collar declares
//! no residence to install into. A suite reaching here is refused rather than
//! served, because the only thing that could be done with it is something other
//! than what was asked.
//!
//! TWO ACTS, AND THE SECOND IS NOT THE FIRST'S SIDE EFFECT. Cargo leaves an
//! artifact where cargo leaves artifacts; a collar declares where the binary its
//! consumers reach STANDS. At one collar those are the same directory and at
//! another they are not, and a door that treated the build as the whole converge
//! would work at the first and silently do half the job at the second. So the
//! install is performed and reported in its own right, the same-directory case
//! included — said as a copy that was not needed rather than passed over in
//! silence, since a reader who cannot tell "installed" from "not attempted" has
//! been told nothing.
//!
//! WHERE CARGO LEAVES THE ARTIFACT IS DERIVED AND THEN PROVEN. The derivation is
//! cargo's own documented layout and the estate's standing practice
//! (`Tools/vok/vob_build.sh` spells the same path by hand), but a derivation is
//! a belief about another program's behavior — the Palisade, where our rules do
//! not reach. So it is CHECKED: nothing standing at the derived path after a
//! build that reported success refuses, naming the path, rather than installing
//! whatever else might be lying around or reporting a converge that did not
//! happen.

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use crate::bkca_whereabouts::bkca_Geography;
use crate::bkce_election;
use crate::bkcl_leash;
use crate::bkcr_resolve::bkcr_Collar;

/// Cargo's own verb for the act, AND IT SPELLS A WORD THIS ESTATE RESERVES. The
/// standing is accepted rather than repaired, on the ground the pipeline's
/// listing words and the leash's lock flag already record: `build` here is
/// cargo's own subcommand — a foreign name we did not choose and no rename of
/// ours may move, since renaming it would mean not invoking cargo at all. The
/// reserve governs what this estate MINTS, and this is a spelling quoted rather
/// than minted. Recorded at the site so the next reader meets the ruling instead
/// of re-deriving it.
///
/// Private, as the launch seam's own verb words are: it is spelled inside this
/// module and nowhere else.
const ZBKCH_VERB_BUILD: &str = "build";

/// The flags the collar's declared shape is spelled with, the same pair the
/// launch seam spells for a suite and for the same reason: the profile and the
/// features are the collar's declared build shape and never a per-invocation
/// choice (BKSCL-Collar.adoc "Declared Build Shapes"). A converge that built at
/// some other shape than the one the collar declares would install an artifact
/// no election could account for.
const ZBKCH_PROFILE_FLAG: &str = "--profile";
const ZBKCH_FEATURES_FLAG: &str = "--features";

/// Where cargo roots its output, unless told otherwise.
const ZBKCH_TARGET_DIR: &str = "target";

/// The environment variable that moves that root. READ RATHER THAN CLEARED,
/// because the leash does not set it and a station that does is entitled to: the
/// kennel's business is to know where the artifact went, not to decide where
/// cargo may put it.
pub const BKCH_TARGET_DIR_VAR: &str = "CARGO_TARGET_DIR";

/// The environment variable a door holding the collar states to the crate it is
/// compiling — the converge, the suite launch and the listing alike — carrying
/// the position walked over THAT COLLAR'S declared roots. The envelope rules the
/// shape once (BKSNC-Kennelcraft.adoc "Currency by git position"), and each
/// door states it where it drives the leash.
///
/// THE DOOR'S CONTRACT WITH THE CRATE, and the reason it is its own name: a
/// build compiles the converged crate AND every crate it links, each with its own
/// build script, in ONE environment. The kennel's own build script walks the
/// kennel's election for itself and takes nothing from the environment, and the
/// kennel's library is linked by the very crates this door converges — so a name
/// shared with the kennel's reading would hand a linked kennel the tenant's
/// position and stamp it with a tree it was not measured over. Two elections
/// answer two questions and need two names, and only one of them travels by
/// environment at all.
///
/// A crate that wants the reading declares a build script reading this name; a
/// crate that does not is unaffected, an unread variable costing the child
/// nothing.
pub const BKCH_COLLAR_POSITION_VAR: &str = "BKK_COLLAR_POSITION";

/// The three profiles cargo maps onto a directory named for something else, and
/// the directories they land in. Every other profile — `release`, which names
/// its own, and any the collar invents — is its own name.
const ZBKCH_PROFILE_DEV: &str = "dev";
const ZBKCH_PROFILE_TEST: &str = "test";
const ZBKCH_PROFILE_BENCH: &str = "bench";
const ZBKCH_DIR_DEBUG: &str = "debug";
const ZBKCH_DIR_RELEASE: &str = "release";

/// A composed converge: what the leash is to build, where cargo will leave it,
/// and where the collar says it stands.
///
/// The three ride together because the report owes all three and a caller that
/// re-derived any of them could name a different one than the act performed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkch_Converge {
    /// The manifest the leash is told, joined from the repository root.
    pub manifest: PathBuf,
    /// Cargo's own verb.
    pub verb: String,
    /// Everything after the verb, in the order the leash will spell it.
    pub rest: Vec<OsString>,
    /// Where cargo leaves the artifact.
    pub struck: PathBuf,
    /// Where the collar says the binary its consumers reach stands.
    pub residence: PathBuf,
}

impl bkch_Converge {
    /// The invocation as it will be spelled, for a diagnostic that means to show
    /// what was actually asked. The leash composes the real one; this is the part
    /// this module chose.
    pub fn bkch_spelling(&self) -> String {
        let mut said = self.verb.clone();
        for argument in &self.rest {
            said.push(' ');
            said.push_str(&argument.to_string_lossy());
        }
        said
    }

    /// Whether cargo already leaves the artifact at the collar's residence.
    ///
    /// COMPARED AS RESOLVED PATHS, so a residence reached through a symlink or
    /// spelled with a `.` still answers the same. An unresolvable path — one that
    /// does not stand yet, which is the ordinary case before the first build —
    /// falls back to the spelled comparison rather than answering false, because
    /// false here means "copy", and a copy onto its own source truncates the file
    /// it is reading.
    pub fn bkch_seated(&self) -> bool {
        match (
            std::fs::canonicalize(&self.struck),
            std::fs::canonicalize(&self.residence),
        ) {
            (Ok(struck), Ok(residence)) => struck == residence,
            _ => self.struck == self.residence,
        }
    }
}

/// The directory cargo names for a profile.
///
/// CARGO'S RULE, RESTATED AND NOT INVENTED: `dev` and `test` share `debug`,
/// `release` and `bench` share `release`, and a custom profile takes its own
/// name. The three that need saying are the three that do not name themselves;
/// `release` and every custom profile fall out of the general case, which is why
/// they are absent from the match rather than spelled into it.
///
/// `bench` IS ONE OF THE THREE and reads as the exception it is: it is the only
/// one whose directory is another profile's name rather than a word of its own,
/// and a mapping written from the debug pair alone gets it wrong silently — the
/// artifact lands in `release` and the derivation looks in `bench`.
pub fn bkch_profile_dir(profile: &str) -> &str {
    match profile {
        ZBKCH_PROFILE_DEV | ZBKCH_PROFILE_TEST => ZBKCH_DIR_DEBUG,
        ZBKCH_PROFILE_BENCH => ZBKCH_DIR_RELEASE,
        other => other,
    }
}

/// Where cargo will leave this collar's artifact.
///
/// THE TARGET ROOT IS THE MANIFEST'S OWN unless the environment moves it, which
/// is cargo's rule and the estate's shape both: the workspace election is
/// deferred and every crate stands its own root, so a manifest's `target/` is
/// this crate's and no other's.
///
/// A relative `CARGO_TARGET_DIR` joins onto the REPOSITORY rather than onto the
/// process's working directory. They are the same directory for every door,
/// which is why the choice can be made honestly here rather than left to
/// whichever the caller happened to hold.
pub fn bkch_struck(repository: &Path, collar: &bkcr_Collar) -> PathBuf {
    let manifest = repository.join(collar.bkcr_field("BKRR_MANIFEST"));

    let root = match std::env::var_os(BKCH_TARGET_DIR_VAR) {
        Some(declared) => repository.join(declared),
        None => match manifest.parent() {
            Some(dir) => dir.join(ZBKCH_TARGET_DIR),
            None => repository.join(ZBKCH_TARGET_DIR),
        },
    };

    root.join(bkch_profile_dir(collar.bkcr_field("BKRR_PROFILE")))
        .join(collar.bkcr_field("BKRR_TARGET"))
}

/// Compose the converge for one app collar.
///
/// EVERY DECLARED FIELD IS SPELLED AND NONE IS INFERRED, which is the launch
/// seam's discipline held at the other end of the same collar: an artifact built
/// at a shape the collar did not declare is one the election cannot speak for,
/// and the election is the whole reason this act exists.
pub fn bkch_compose(
    repository: &Path,
    geography: &bkca_Geography,
    collar: &bkcr_Collar,
) -> Result<bkch_Converge, String> {
    if !collar.bkcr_app() {
        return Err(format!(
            "the collar '{}' names a suite, and a suite is never converged — its runner compiles \
             it from the source this seat holds, so no artifact stands between the source and the \
             verdict for a converge to repair, and a suite collar declares no residence to install \
             into",
            collar.name
        ));
    }

    let mut rest: Vec<OsString> = Vec::new();

    let profile = collar.bkcr_field("BKRR_PROFILE");
    if !profile.trim().is_empty() {
        rest.push(OsString::from(ZBKCH_PROFILE_FLAG));
        rest.push(OsString::from(profile.trim()));
    }

    let features = collar.bkcr_field("BKRR_FEATURES");
    if !features.trim().is_empty() {
        rest.push(OsString::from(ZBKCH_FEATURES_FLAG));
        rest.push(OsString::from(
            features.split_whitespace().collect::<Vec<&str>>().join(" "),
        ));
    }

    // THE INSTALL TARGET IS THE SEAT'S OWN RESIDENCE, joined onto the work tree
    // and never onto the delivered root. Under self-geography the two are one
    // tree and the join has always answered this; under dispatch they are not,
    // and the sentence below is what keeps the join honest when they part.
    let residence = bkce_election::bkce_residence(repository, collar);

    if let Some(delivered) = geography.bkca_delivered() {
        if zbkch_within(&residence, delivered) {
            return Err(format!(
                "the collar '{}' would install at {}, which stands inside the delivered root at \
                 {} — a seat's build never lands on a delivered tree, whatever any position says. \
                 The delivered root is the borrow candidate every dispatched seat reads and a \
                 binary written there is written into every one of their elections at once; this \
                 seat converges its OWN residence in the tree it entered, and the whereabouts \
                 that named this root is where the two are told apart",
                collar.name,
                residence.display(),
                delivered.display()
            ));
        }
    }

    Ok(bkch_Converge {
        manifest: repository.join(collar.bkcr_field("BKRR_MANIFEST")),
        verb: ZBKCH_VERB_BUILD.to_string(),
        rest,
        struck: bkch_struck(repository, collar),
        residence,
    })
}

/// Whether an install target resolves inside a delivered root.
///
/// CANONICALIZED WHERE BOTH STAND AND COMPARED AS SPELLED WHERE THEY DO NOT,
/// which is the same fallback `bkch_seated` takes and for the same reason: a
/// residence that has never been built does not stand yet, and that ordinary
/// case must not be the one the guard goes quiet on. Resolving is a
/// STRENGTHENING — it catches a residence reached through a symlink or spelled
/// with a `.` — and the lexical reading underneath it is what makes this act
/// answerable with no repository on disk, which is what lets its own hurdles
/// pose a fabricated seat.
///
/// THE ROOT ITSELF COUNTS AS INSIDE. A residence that IS the delivered root is
/// the worst case rather than an edge one, and `starts_with` on a path answers
/// component by component, so no prefix of a sibling directory name can be
/// mistaken for containment.
fn zbkch_within(target: &Path, delivered: &Path) -> bool {
    zbkch_resolved(target).starts_with(zbkch_resolved(delivered))
}

/// A path with as much of it resolved as actually stands, the rest left as
/// spelled.
///
/// A WHOLE-PATH `canonicalize` IS THE WRONG INSTRUMENT HERE, and its failure mode
/// is the one that matters: an install target does not stand before the first
/// converge, so the ordinary case is precisely the one where resolving the whole
/// path fails — and a guard that fell back to a bare lexical reading there would
/// go quiet exactly when a symlinked work tree made the lexical reading wrong.
/// Resolving the longest standing ANCESTOR gives the containment question a real
/// answer whether or not the leaf has ever been built.
fn zbkch_resolved(path: &Path) -> PathBuf {
    if let Ok(real) = std::fs::canonicalize(path) {
        return real;
    }

    match (path.parent(), path.file_name()) {
        (Some(parent), Some(name)) => zbkch_resolved(parent).join(name),
        _ => path.to_path_buf(),
    }
}

/// Build a composed converge through the leash and hand back cargo's own exit.
///
/// THE DRIVE FACE, NEVER THE RECALL, for the launch seam's own reason: a caller
/// who typed a converge is watching a compiler work, and taking the streams to
/// read them would hold the whole build silent and then replay it.
/// THE COLLAR'S POSITION RIDES ACROSS THE EXEC BOUNDARY, because a converged
/// crate cannot walk it for itself: a build script runs with cargo's environment
/// and no repository to ask, and the reading wants the collar's declared roots,
/// which only the door holding the collar has. A crate declaring a build script
/// that reads it is stamped with the tree it was built from; a crate declaring
/// none is unaffected, an unread variable costing the child nothing.
///
/// WALKED HERE RATHER THAN AT THE COMPOSITION, and the line is the one the
/// composition draws for itself: composing is pure over the collar and answerable
/// with no repository on disk, which is what lets its own hurdles pose a
/// fabricated seat and read back a spelling. Walking a position asks git, so a
/// reading placed there would make the pure act fallible on the filesystem and
/// would take the reading whether or not anything was ever built with it. The
/// drive is already impure and already holds the repository.
///
/// The collar is taken here rather than read back off the converge for the same
/// reason: the converge carries what the REPORT owes, and this position is not
/// reported.
pub fn bkch_build(
    repository: &Path,
    collar: &bkcr_Collar,
    converge: &bkch_Converge,
) -> Result<bkcl_leash::bkcl_Run, String> {
    // AHEAD OF THE SEAT AS WELL AS AHEAD OF CARGO. Walking a position asks git
    // and costs a process; refusing first on a file that is simply not there
    // hands the operator the door to run without spending it.
    crate::bkcr_resolve::bkcr_exergue_stands(repository, collar)?;

    let seat = bkce_election::bkce_seat(repository, collar.bkcr_field("BKRR_ROOTS"))?;

    bkcl_leash::bkcl_drive(
        repository,
        &converge.manifest,
        &converge.verb,
        &converge.rest,
        &[(BKCH_COLLAR_POSITION_VAR, OsStr::new(seat.as_str()))],
    )
}

/// Put the struck artifact at the collar's residence, and say whether anything
/// moved.
///
/// THE DERIVED PATH IS PROVEN BEFORE ANYTHING IS COPIED. A build that reported
/// success and left nothing where this module said it would means the
/// derivation is wrong, which is a fact worth a loud refusal: installing
/// whatever else stands at the residence, or reporting a converge that did not
/// happen, are the two ways to turn a wrong belief into a wrong artifact.
///
/// THE RESIDENCE IS THE ONLY THING WRITTEN, and its directory is made where it
/// does not stand. Nothing else on disk is touched by this door.
pub fn bkch_install(converge: &bkch_Converge) -> Result<bool, String> {
    if !converge.struck.is_file() {
        return Err(format!(
            "the build reported success and nothing stands at {} — that path is where cargo's own \
             layout says this collar's artifact lands, derived from the manifest's directory, the \
             declared profile and the declared target. Refused rather than installing whatever \
             else may stand at the residence: a converge that cannot find what it built has not \
             converged",
            converge.struck.display()
        ));
    }

    if converge.bkch_seated() {
        return Ok(false);
    }

    if let Some(home) = converge.residence.parent() {
        std::fs::create_dir_all(home).map_err(|err| {
            format!(
                "could not make the residence at {}: {}",
                home.display(),
                err
            )
        })?;
    }

    std::fs::copy(&converge.struck, &converge.residence).map_err(|err| {
        format!(
            "could not install {} at {}: {}",
            converge.struck.display(),
            converge.residence.display(),
            err
        )
    })?;

    Ok(true)
}

/// The station tools this door spawns, each named at
/// BKSNC-Kennelcraft.adoc "What stands outside the kennel" and none of them
/// a kibble: a station tool is the ground the kennel stands on rather than an
/// artifact it answers for. Absent, each refuses by naming itself.
const ZBKCH_CURL: &str = "curl";
const ZBKCH_TAR: &str = "tar";
const ZBKCH_OPENSSL: &str = "openssl";

/// The lock every write of the tackroom is taken under, and the seconds a
/// caller waits for it.
///
/// `mkdir` IS THE PORTABLE ATOMIC PRIMITIVE — one caller wins and every other
/// gets `AlreadyExists` — and the pattern is `Tools/vok/vot_toolchain.sh`'s,
/// cited rather than redesigned: the same store is provisioned by that fence,
/// and a second locking scheme over one directory would be two schemes that do
/// not see each other.
/// THE DIRECTORY'S NAME SPELLS NO GOVERNED WORD, which the value and not the
/// concept is what decides. `lock` stands as a governed word's blocked face, and
/// the concurrency sense keeps the word in prose and in the identifiers below —
/// what may not carry it is a DECLARED VALUE, which duplicates a governed
/// spelling away from its home and would move under a remint whatever it meant.
/// So the directory is named for what holding it grants, which is also what the
/// guard below is called.
const ZBKCH_HELD: &str = "bkch-kibble.held";
const ZBKCH_LOCK_WAIT: u64 = 300;

/// Cargo's own verb for building a binary out of an unpacked crate, and the
/// flags a kibble's build is spelled with.
///
/// `--no-default-features` IS THE REGIME'S LAW AND NOT A KIBBLE'S CHOICE, which
/// is why it is spelled here rather than declared per instance. A kibble's
/// build has its self-update feature off so that one seal pins the whole
/// closure (BKSBL-Kibble.adoc "Source"); a program that can update itself is a
/// program whose version stops being the one the pin declares, which is the
/// entire failure the regime exists to close. What a kibble declares is the
/// features it wants BACK, in `BKRK_FEATURES`.
const ZBKCH_NO_DEFAULTS: &str = "--no-default-features";
const ZBKCH_RELEASE: &str = "--release";
const ZBKCH_BIN_FLAG: &str = "--bin";

/// The profile directory cargo leaves a `--release` build in.
const ZBKCH_KIBBLE_DIR: &str = "release";

/// What a kibble converge did, for a caller that must report it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum bkch_Kibbled {
    /// The declared version already stood at its residence; nothing was fetched
    /// and nothing was built.
    Current,
    /// The archive was fetched, proven, and the binary placed — carrying the
    /// archive it came from.
    ///
    /// THE URL IS CARRIED RATHER THAN RECOMPOSED, because the two kinds reach
    /// their archives by roads that share nothing: a source kibble joins its
    /// version onto a registry base, and a prebuilt one is told the archive's
    /// name by a document it fetched. A caller that spelled the road itself
    /// could only spell one of them, and would report the wrong one for the
    /// other kind while looking entirely correct.
    Placed(String),
}

/// Converge one kibble of either kind, and land its binary at its residence.
///
/// THE TWO KINDS ARE TWO ROADS TO ONE ACT. A source kibble fetches a published
/// crate archive against its own seal and builds it through the leash; a
/// prebuilt one fetches a sealed vendor bordereau, reads out of it the archive
/// this station's host triple is served by, and proves that archive against the
/// digest the document states. What they share is everything after: the same
/// lock, the same scratch directory, the same rename onto the same residence
/// (BKSBL-Kibble.adoc "Two Kinds").
///
/// THE ONE DOOR THAT WRITES THE TACKROOM, and everything about the act is shaped
/// by that (BKSBL-Kibble.adoc "Where a Kibble Stands"). Every other door tests
/// the residence path and refuses naming this one.
///
/// THE SEAL IS PROVEN BEFORE ANYTHING IS UNPACKED, which is the whole order of
/// the act rather than a step in it. An archive whose digest does not answer is
/// not a wrong build, it is an unknown artifact — so it is never unpacked, never
/// built, and nothing is placed. The refusal names BOTH digests, because a
/// reader told only that a seal failed cannot tell a bumped version from a
/// tampered archive, and those are different emergencies.
///
/// THE LANDING IS A RENAME, UNDER THE LOCK. A residence written in place reads
/// as whole the moment its first byte lands, so a second door testing the path
/// while a build is copying would spawn a truncated binary. The build lands
/// beside the residence and is renamed onto it, which is atomic within one
/// filesystem, and the lock is what keeps two converges of one kibble from
/// racing at all.
///
/// A CURRENT RESIDENCE IS ANSWERED WITHOUT THE LOCK, deliberately: the ordinary
/// call is the one that has nothing to do, and making it queue behind another
/// clone's download would price the common case at the rare one's cost. The
/// test is repeated inside the lock, where it is the one that decides.
pub fn bkch_kibble(
    repository: &Path,
    kibble: &crate::bkcq_kibble::bkcq_Kibble,
) -> Result<bkch_Kibbled, String> {
    let residence = kibble.bkcq_residence()?;

    if residence.is_file() {
        return Ok(bkch_Kibbled::Current);
    }

    let held = zbkch_lock(&residence)?;
    let landed = zbkch_kibble_held(repository, kibble, &residence);
    held.zbkch_release();

    landed
}

/// The converge proper, performed with the lock held.
fn zbkch_kibble_held(
    repository: &Path,
    kibble: &crate::bkcq_kibble::bkcq_Kibble,
    residence: &Path,
) -> Result<bkch_Kibbled, String> {
    // THE TEST THAT DECIDES, taken inside the lock. The one above it is an
    // economy; this one is the answer, because another clone may have placed the
    // very residence this call waited behind.
    if residence.is_file() {
        return Ok(bkch_Kibbled::Current);
    }

    let home = residence
        .parent()
        .ok_or_else(|| format!("the residence {} names no directory", residence.display()))?;

    let work = home.join(zbkch_work(kibble));

    let _ = std::fs::remove_dir_all(&work);
    std::fs::create_dir_all(&work)
        .map_err(|err| format!("could not make {}: {}", work.display(), err))?;

    // THE KIND DECIDES THE ROAD AND NOTHING ELSE HERE DOES. Everything around
    // this branch — the lock, the scratch directory, the rename onto the
    // residence, the sweep afterward — is one act performed identically for both
    // kinds, which is what makes a prebuilt residence indistinguishable from a
    // built one once it stands.
    let (struck, from) = if kibble.bkcq_source() {
        zbkch_kibble_source(repository, kibble, &work)?
    } else {
        zbkch_kibble_prebuilt(repository, kibble, &work)?
    };

    // LANDED BY RENAME, which is what makes a half-written residence
    // unobservable: the file appears whole or not at all.
    std::fs::rename(&struck, residence).map_err(|err| {
        format!(
            "could not land {} at {}: {}",
            struck.display(),
            residence.display(),
            err
        )
    })?;

    let _ = std::fs::remove_dir_all(&work);

    Ok(bkch_Kibbled::Placed(from))
}

/// The source kind's road: fetch the published crate archive, prove it against
/// the kibble's own seal, unpack it, and build it through the leash.
fn zbkch_kibble_source(
    repository: &Path,
    kibble: &crate::bkcq_kibble::bkcq_Kibble,
    work: &Path,
) -> Result<(PathBuf, String), String> {
    let from = kibble.bkcq_archive();
    let archive = work.join(format!("{}.crate", kibble.bkcq_unpacked()));

    zbkch_fetch(&from, &archive)?;
    zbkch_sealed(
        kibble,
        &archive,
        kibble.bkcq_field("BKRK_SEAL").trim(),
        ZBKCH_THE_ARCHIVE,
        &from,
    )?;
    zbkch_unpack(&archive, work)?;

    let unpacked = work.join(kibble.bkcq_unpacked());
    let struck = zbkch_kibble_build(repository, kibble, &unpacked)?;

    Ok((struck, from))
}

/// The prebuilt kind's road: fetch the vendor bordereau, prove the seal over
/// IT, read the archive this station's host triple is served by, fetch that
/// archive, prove it against the digest the bordereau states, and take the
/// binary out of it.
///
/// THE SEAL IS OVER THE DOCUMENT AND THE DOCUMENT ANSWERS FOR THE ARCHIVES,
/// which is what lets one declared digest pin a release the kennel has no
/// platform table for. The kennel maintains one hash per version; the table is
/// the vendor's, and it is sealed rather than kept true by hand
/// (BKSBL-Kibble.adoc "Prebuilt").
///
/// THE TRIPLE IS JOINED AND NEVER COMPARED. What rustc declared is looked up in
/// the bordereau as opaque text: a host the document does not carry refuses by
/// naming the TRIPLE, never a platform, because the kennel has read no platform
/// out of it and could not name one honestly.
///
/// NOTHING IS BUILT ON THIS ROAD, and the whole reason the kind exists is that
/// building would fail: uv asks a rustc above the estate's pin and cmake for the
/// TLS library it links (BKSBL-Kibble.adoc "What sorts them").
fn zbkch_kibble_prebuilt(
    repository: &Path,
    kibble: &crate::bkcq_kibble::bkcq_Kibble,
    work: &Path,
) -> Result<(PathBuf, String), String> {
    let triple = bkcl_leash::bkcl_host(repository, &repository.join(&kibble.instance))?;

    let source = kibble.bkcq_bordereau();
    let bordereau = work.join(ZBKCH_BORDEREAU_FILE);

    zbkch_fetch(&source, &bordereau)?;
    zbkch_sealed(
        kibble,
        &bordereau,
        kibble.bkcq_field("BKRK_SEAL").trim(),
        ZBKCH_THE_BORDEREAU,
        &source,
    )?;

    let said = std::fs::read_to_string(&bordereau)
        .map_err(|err| format!("could not read {}: {}", bordereau.display(), err))?;

    let (name, digest) = zbkch_borne(kibble, &said, &triple, &source)?;

    // THE FORMAT IS REFUSED BY NAME AND BEFORE THE FETCH. uv publishes a zip for
    // every windows triple and a gzipped tar everywhere else, so this is not a
    // hypothetical shape — it is the one archive a station on the wrong host
    // would actually be handed. MVP unpacks a gzipped tar and nothing else
    // (BKSBL-Kibble.adoc "Prebuilt"), and a refusal naming the format tells its
    // reader what would have to be built, where a failure inside tar would not.
    if !name.ends_with(ZBKCH_TARBALL) {
        return Err(format!(
            "the bordereau serves {} for the host triple {} and the kennel unpacks {} alone at \
             MVP, so NOTHING was fetched. The format is the vendor's own election per triple and \
             not a thing this kibble can restate; what is missing is an unpacker, and the \
             declaration is sound (BKSBL-Kibble.adoc \"Prebuilt\")",
            name, triple, ZBKCH_TARBALL
        ));
    }

    let from = kibble.bkcq_beside(&name);
    let archive = work.join(&name);

    zbkch_fetch(&from, &archive)?;
    zbkch_sealed(kibble, &archive, &digest, ZBKCH_THE_ARCHIVE, &from)?;

    let unpacked = work.join(ZBKCH_UNPACKED_DIR);
    std::fs::create_dir_all(&unpacked)
        .map_err(|err| format!("could not make {}: {}", unpacked.display(), err))?;

    zbkch_unpack(&archive, &unpacked)?;

    let byname = kibble.bkcq_field("BKRK_BYNAME");
    let struck = zbkch_borrowed(&unpacked, byname).ok_or_else(|| {
        format!(
            "the archive {} carries no file named '{}', which is what this kibble declares its \
             binary is called — nothing was placed. The declared name is what a residence is \
             composed from and what every door spawns, so a placement under any other name would \
             stand at a path no door looks at",
            name, byname
        )
    })?;

    Ok((struck, from))
}

/// The archive one host triple is served by, read out of the bordereau: its
/// name and the digest the document states for it.
///
/// THE DOCUMENT'S SHAPE IS THE VENDOR'S AND IS READ DEFENSIVELY. This is the
/// Palisade — a schema nobody here elected — so every step of the descent that
/// does not answer produces a refusal naming what was missing, rather than a
/// default that would send the fetch somewhere unintended.
fn zbkch_borne(
    kibble: &crate::bkcq_kibble::bkcq_Kibble,
    said: &str,
    triple: &str,
    source: &str,
) -> Result<(String, String), String> {
    let read = crate::bkcj_json::bkcj_read(said)
        .map_err(|err| format!("the bordereau at {} is not readable JSON: {}", source, err))?;

    let Some(crate::bkcj_json::bkcj_Value::Object(artifacts)) =
        read.bkcj_field(ZBKCH_ARTIFACTS)
    else {
        return Err(format!(
            "the bordereau at {} carries no '{}' object, so it names no archive for any host — \
             either the release publishes a document of another shape, or the declared one is not \
             the document that enumerates archives",
            source, ZBKCH_ARTIFACTS
        ));
    };

    let mut carried: Vec<&str> = Vec::new();

    for (name, artifact) in artifacts {
        let Some(triples) = artifact
            .bkcj_field(ZBKCH_TRIPLES)
            .and_then(|held| held.bkcj_array())
        else {
            continue;
        };

        let mut answers = false;

        for held in triples {
            let Some(said) = held.bkcj_string() else {
                continue;
            };
            carried.push(said);
            if said == triple {
                answers = true;
            }
        }

        if !answers {
            continue;
        }

        let Some(digest) = artifact
            .bkcj_field(ZBKCH_CHECKSUMS)
            .and_then(|held| held.bkcj_field(ZBKCH_SHA256))
            .and_then(|held| held.bkcj_string())
        else {
            continue;
        };

        return Ok((name.clone(), digest.to_string()));
    }

    carried.sort_unstable();
    carried.dedup();

    Err(format!(
        "the bordereau at {} names no sealed archive for the host triple {}, so the kibble '{}' \
         cannot be placed on this station and NOTHING was fetched. The triple is what rustc \
         declared and is joined as text — the kennel reads no platform out of it and keeps no \
         table of its own, so what is missing is a publication rather than support. The document \
         carries: {}",
        source,
        triple,
        kibble.name,
        if carried.is_empty() {
            "no triple at all".to_string()
        } else {
            carried.join(", ")
        }
    ))
}

/// The declared binary, found inside an unpacked archive.
///
/// SEARCHED FOR RATHER THAN COMPOSED, because where a vendor puts a binary
/// inside its own archive is that vendor's shape and not a fact this kennel may
/// hold: uv nests its two executables under a directory named for the triple,
/// and a kennel that spelled that layout would be carrying a second copy of the
/// vendor's decisions for the pin's seal to be unable to protect.
fn zbkch_borrowed(root: &Path, byname: &str) -> Option<PathBuf> {
    let mut standing = vec![root.to_path_buf()];

    while let Some(at) = standing.pop() {
        let listing = std::fs::read_dir(&at).ok()?;

        for held in listing.flatten() {
            let path = held.path();

            if path.is_dir() {
                standing.push(path);
                continue;
            }

            if path.file_name().map(|name| name == byname).unwrap_or(false) {
                return Some(path);
            }
        }
    }

    None
}

/// What the bordereau is called on disk while it is being proven, the shape of
/// the archive MVP unpacks, and the vendor keys the document is read by.
///
/// THE VENDOR'S KEYS KEEP THEIR VENDOR SPELLING, as the JSON reader's own
/// charter has it: these are names this estate did not choose.
const ZBKCH_BORDEREAU_FILE: &str = "bordereau.json";
const ZBKCH_UNPACKED_DIR: &str = "unpacked";
const ZBKCH_TARBALL: &str = ".tar.gz";
const ZBKCH_ARTIFACTS: &str = "artifacts";
const ZBKCH_TRIPLES: &str = "target_triples";
const ZBKCH_CHECKSUMS: &str = "checksums";
const ZBKCH_SHA256: &str = "sha256";

/// What a sealed file is called in the refusal that names it. Spelled here so
/// the two roads' refusals differ in the noun alone.
const ZBKCH_THE_ARCHIVE: &str = "the archive";
const ZBKCH_THE_BORDEREAU: &str = "the bordereau";

/// The scratch directory one converge unpacks and builds in, named for the
/// kibble so two converges under one quarter cannot collide.
fn zbkch_work(kibble: &crate::bkcq_kibble::bkcq_Kibble) -> String {
    format!("{}.work", kibble.bkcq_unpacked())
}

/// Fetch one archive.
///
/// THE TOOL IS THE STATION'S AND IS NAMED WHEN IT IS ABSENT. Curl stands outside
/// every declaration the kennel reads, so a station without one meets a sentence
/// naming curl rather than a failure to fetch.
fn zbkch_fetch(from: &str, to: &Path) -> Result<(), String> {
    let asked = std::process::Command::new(ZBKCH_CURL)
        .arg("--fail")
        .arg("--location")
        .arg("--silent")
        .arg("--show-error")
        .arg("--output")
        .arg(to)
        .arg(from)
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(|err| {
            format!(
                "no '{}' answered on this station: {}. A source kibble is fetched as its published \
                 archive, and the kennel provisions nothing it spawns — converge it through the \
                 station's own packaging",
                ZBKCH_CURL, err
            )
        })?;

    if !asked.status.success() {
        return Err(format!(
            "could not fetch {}: {}",
            from,
            String::from_utf8_lossy(&asked.stderr).trim()
        ));
    }

    Ok(())
}

/// Prove one archive against the seal its kibble declares.
///
/// HASHED BY OPENSSL IN THE STDIN FORM, which is the form
/// `specs/bkk/BKSPY-Python.adoc` carries for this estate and is
/// cited rather than re-elected: the file is fed on stdin so the answer carries
/// the digest alone and never a filename whose spelling would differ by
/// platform.
fn zbkch_sealed(
    kibble: &crate::bkcq_kibble::bkcq_Kibble,
    file: &Path,
    declared: &str,
    what: &str,
    from: &str,
) -> Result<(), String> {
    let opened = std::fs::File::open(file)
        .map_err(|err| format!("could not read {}: {}", file.display(), err))?;

    let asked = std::process::Command::new(ZBKCH_OPENSSL)
        .arg("dgst")
        .arg("-sha256")
        .stdin(std::process::Stdio::from(opened))
        .output()
        .map_err(|err| {
            format!(
                "no '{}' answered on this station: {}. A kibble's archive is proven by its sha256 \
                 before anything is unpacked, and the kennel provisions nothing it spawns — \
                 converge it through the station's own packaging",
                ZBKCH_OPENSSL, err
            )
        })?;

    if !asked.status.success() {
        return Err(format!(
            "'{}' could not hash {}: {}",
            ZBKCH_OPENSSL,
            file.display(),
            String::from_utf8_lossy(&asked.stderr).trim()
        ));
    }

    let said = String::from_utf8_lossy(&asked.stdout);

    let Some(found) = zbkch_digest(&said) else {
        return Err(format!(
            "'{}' answered something this kennel cannot read a digest out of: {}",
            ZBKCH_OPENSSL,
            said.trim()
        ));
    };

    if found == declared {
        return Ok(());
    }

    Err(format!(
        "{} fetched for the kibble '{}' does not answer the digest stated for it, so NOTHING was \
         unpacked, built or placed.\n  stated:  {}\n  fetched: {}\nIt is {}. A digest that does \
         not answer is an UNKNOWN artifact rather than a wrong one: either the declared version \
         moved under its own name, or what arrived is not what the vendor published, and those \
         are different emergencies — which is why both digests stand here rather than a verdict",
        what, kibble.name, declared, found, from
    ))
}

/// The digest openssl's stdin form answers with, which is the last
/// whitespace-delimited word of its one line.
///
/// CITED FROM A CAPTURED LINE rather than from a memory of the format. On this
/// station, 260906:
///
/// ```text
/// SHA2-256(stdin)= d8a41c206ff63dc9f9d45514ed3309adf4704ddd74f57dacacd5f7e613af9a98
/// ```
///
/// The label before the digest is openssl's own and has changed spelling across
/// its releases — `SHA256(stdin)=` in older builds, `SHA2-256(stdin)=` here — so
/// the reading takes the LAST word and never the label, which is the one part of
/// that line this estate can be sure of.
fn zbkch_digest(said: &str) -> Option<String> {
    said.lines()
        .next()?
        .split_whitespace()
        .next_back()
        .map(str::to_string)
}

/// Unpack one archive into a directory.
fn zbkch_unpack(archive: &Path, into: &Path) -> Result<(), String> {
    let asked = std::process::Command::new(ZBKCH_TAR)
        .arg("-xzf")
        .arg(archive)
        .arg("-C")
        .arg(into)
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(|err| {
            format!(
                "no '{}' answered on this station: {}. A published crate archive is a gzipped tar, \
                 and the kennel provisions nothing it spawns — converge it through the station's \
                 own packaging",
                ZBKCH_TAR, err
            )
        })?;

    if !asked.status.success() {
        return Err(format!(
            "could not unpack {}: {}",
            archive.display(),
            String::from_utf8_lossy(&asked.stderr).trim()
        ));
    }

    Ok(())
}

/// Build the unpacked crate through the leash, and answer where cargo left the
/// binary.
///
/// THROUGH THE LEASH LIKE EVERY OTHER BUILD, which is what makes the seal pin
/// the whole closure rather than the top crate alone: the archive ships its own
/// `Cargo.lock`, and the leash's `--locked` is what refuses to resolve anything
/// the lock does not already answer for. A build that reached cargo around the
/// chokepoint would resolve whatever the registry offered that day, and the
/// declared version would name a binary nobody could reproduce.
///
/// THE CHANNEL IS THE KIBBLE'S OWN, stated by a pin written beside the unpacked
/// manifest. The leash reads the nearest pin at or above the crate's directory
/// and is bounded by the repository it is given — and the repository here is the
/// unpacked crate itself, standing in the tackroom and outside every repository
/// on the station, so the pin this writes is the ONLY one the walk can reach.
/// That is the point rather than a convenience: a foreign crate must be built at
/// the channel its kibble declares, and nothing about where the tackroom
/// happens to sit may answer that question.
fn zbkch_kibble_build(
    _repository: &Path,
    kibble: &crate::bkcq_kibble::bkcq_Kibble,
    unpacked: &Path,
) -> Result<PathBuf, String> {
    let channel = kibble.bkcq_field("BKRK_CHANNEL").trim();

    std::fs::write(
        unpacked.join(bkcl_leash::BKCL_PIN_FILE),
        format!("[toolchain]\nchannel = \"{}\"\n", channel),
    )
    .map_err(|err| {
        format!(
            "could not state the kibble's channel at {}: {}",
            unpacked.display(),
            err
        )
    })?;

    let manifest = unpacked.join("Cargo.toml");
    let target = kibble.bkcq_field("BKRK_TARGET");

    let mut rest: Vec<OsString> = vec![
        OsString::from(ZBKCH_RELEASE),
        OsString::from(ZBKCH_BIN_FLAG),
        OsString::from(target),
        OsString::from(ZBKCH_NO_DEFAULTS),
    ];

    let features = kibble.bkcq_field("BKRK_FEATURES");
    if !features.trim().is_empty() {
        rest.push(OsString::from(ZBKCH_FEATURES_FLAG));
        rest.push(OsString::from(
            features.split_whitespace().collect::<Vec<&str>>().join(" "),
        ));
    }

    let run = bkcl_leash::bkcl_drive(unpacked, &manifest, ZBKCH_VERB_BUILD, &rest, &[])?;

    if !run.bkcl_landed() {
        return Err(format!(
            "the kibble '{}' did not build, so nothing was placed: {}",
            kibble.name, run.spelling
        ));
    }

    let struck = unpacked
        .join(ZBKCH_TARGET_DIR)
        .join(ZBKCH_KIBBLE_DIR)
        .join(target);

    if !struck.is_file() {
        return Err(format!(
            "the kibble '{}' built and nothing stands at {} — that path is where cargo's own \
             layout says a release binary of the declared target lands. Refused rather than \
             placing whatever else may be lying around",
            kibble.name,
            struck.display()
        ));
    }

    Ok(struck)
}

/// The tackroom lock, held for one converge.
///
/// A GUARD RATHER THAN A PAIR OF CALLS, so the release cannot be forgotten on a
/// road a caller did not think about.
pub struct bkch_Held {
    at: PathBuf,
}

impl bkch_Held {
    /// Give the lock back.
    pub fn zbkch_release(self) {
        let _ = std::fs::remove_dir(&self.at);
    }
}

/// Take the tackroom lock, or die naming the stale directory to remove.
///
/// THE WAIT AND ITS SENTENCE ARE THE FENCE'S, cited rather than redesigned
/// (`Tools/vok/vot_toolchain.sh`): a caller that timed out without naming the
/// directory would leave its reader to find a lock they cannot see.
fn zbkch_lock(residence: &Path) -> Result<bkch_Held, String> {
    let quarter = residence
        .parent()
        .and_then(|home| home.parent())
        .and_then(|named| named.parent())
        .ok_or_else(|| {
            format!(
                "the residence {} does not stand under a kennel quarter of the tackroom",
                residence.display()
            )
        })?;

    std::fs::create_dir_all(quarter)
        .map_err(|err| format!("could not make {}: {}", quarter.display(), err))?;

    let at = quarter.join(ZBKCH_HELD);

    let mut waited = 0;
    loop {
        match std::fs::create_dir(&at) {
            Ok(()) => return Ok(bkch_Held { at }),
            Err(err) if err.kind() != std::io::ErrorKind::AlreadyExists => {
                return Err(format!("could not take {}: {}", at.display(), err))
            }
            Err(_) => {}
        }

        if waited >= ZBKCH_LOCK_WAIT {
            return Err(format!(
                "timed out after {}s waiting for the tackroom's kibble lock. If no other clone is \
                 converging a kibble, the lock is stale — remove it: {}",
                ZBKCH_LOCK_WAIT,
                at.display()
            ));
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
        waited += 1;
    }
}

// eof


/// What a python converge did, reported so a caller can say it without asking
/// the filesystem a second question.
#[derive(Debug, Clone)]
pub struct bkch_Environed {
    /// The environment this converge built or found current.
    pub environment: PathBuf,
    /// The interpreter version the project pins, which is the one installed.
    pub pinned: String,
    /// The install step as spelled, for the record.
    pub installed: String,
    /// The sync step as spelled, for the record.
    pub synced: String,
}

/// Converge one python collar's project: the pinned interpreter under the
/// station's store, and the environment under the checkout's loosebox, synced
/// from the lock.
///
/// TWO INVOCATIONS AND NOT ONE, because they carry different network postures and
/// the difference is the routine-download ruling itself. The install step is the
/// ONE act in this kennel entitled to fetch an interpreter, and it says so by
/// electing the fetching posture by name; the sync that follows may reach an
/// index and may not fetch an interpreter. A single invocation lifting both would
/// make the interpreter download a side effect of every converge rather than a
/// step someone can point at.
///
/// THE PIN IS SPELLED AT THE INSTALL AND NEVER LEFT TO DISCOVERY. uv would read
/// the pin file itself, and that would be one authority too many: this door has
/// already read it, the validation has already refused a floor where a patch is
/// owed, and an install spelled with what this door read is an install that
/// cannot disagree with what this door reports.
///
/// THE LOCK IS ENFORCED AT THE SYNC AND THE ENFORCEMENT IS A FLAG, not a posture:
/// a lock that would have to change refuses, naming this door, and the one place
/// in the kennel entitled to re-derive it is the lock-authoring door. A converge
/// that quietly relocked would be authoring a resolution nobody reviewed.
///
/// NOTHING IS MADE BY HAND HERE. uv composes the environment's whole path when it
/// creates it, so a door that pre-made directories would be guessing at a layout
/// uv owns — and would make an absent loosebox look like a converge that worked.
pub fn bkch_python(
    repository: &Path,
    collar: &crate::bkcx_python::bkcx_Collar,
) -> Result<bkch_Environed, String> {
    let seat = crate::bkcx_python::bkcx_seat(collar)?;

    bkch_python_at(repository, collar, &seat)
}

/// The converge performed against a NAMED seat.
///
/// THE PURE HALF, on the kibble residence's own precedent and for its exact
/// reason: the composition above reads the loosebox and the tackroom out of the
/// process environment, which a hurdle could only pose by WRITING — and a
/// written environment is shared by every hurdle the harness runs in parallel,
/// so a hurdle posing a store would be deciding its neighbours' answers. The one
/// line that consults the environment consults it above, and this act takes what
/// it was handed.
pub fn bkch_python_at(
    repository: &Path,
    collar: &crate::bkcx_python::bkcx_Collar,
    seat: &crate::bkcx_python::bkcx_Seat,
) -> Result<bkch_Environed, String> {
    let pinned = crate::bkcx_python::bkcx_pinned(repository, collar)?;
    let project = repository.join(collar.bkcx_project());

    let installing = crate::bkcx_python::bkcx_stated(seat, crate::bkcx_python::bkcx_Posture::Fetching);
    let install = bkcl_leash::bkcl_uv(
        repository,
        repository,
        [
            OsStr::new(ZBKCH_UV_PYTHON),
            OsStr::new(ZBKCH_UV_INSTALL),
            OsStr::new(ZBKCH_UV_NO_BIN),
            OsStr::new(pinned.as_str()),
        ],
        &zbkch_borne_env(&installing),
    )?;

    if !install.bkcl_landed() {
        return Err(format!(
            "the interpreter the project pins ({}) was not installed, so nothing was synced and \
             {} stands as it did: {}",
            pinned,
            seat.environment.display(),
            install.spelling
        ));
    }

    let syncing = crate::bkcx_python::bkcx_stated(seat, crate::bkcx_python::bkcx_Posture::Reaching);
    let sync = bkcl_leash::bkcl_uv(
        repository,
        &project,
        [
            OsStr::new(ZBKCH_UV_SYNC),
            OsStr::new(ZBKCH_UV_LOCKED),
            OsStr::new(ZBKCH_UV_PROJECT),
            project.as_os_str(),
        ],
        &zbkch_borne_env(&syncing),
    )?;

    if !sync.bkcl_landed() {
        return Err(format!(
            "the project at {} did not sync, so {} stands as it did. A lock that no longer answers \
             its manifest refuses here rather than being re-derived: the one door entitled to \
             author a lock is the lock-authoring door, and this one converges what that door \
             settled ({})",
            collar.bkcx_project().display(),
            seat.environment.display(),
            sync.spelling
        ));
    }

    Ok(bkch_Environed {
        environment: seat.environment.clone(),
        pinned,
        installed: install.spelling,
        synced: sync.spelling,
    })
}

/// The stated environment as the leash takes it — a borrowed slice over the
/// composed roster.
///
/// The roster is composed as owned values because the paths in it are, and the
/// leash's face borrows because every other caller of it states literals. This
/// is the one line that joins the two, kept here rather than widening the leash's
/// face for one caller's convenience.
fn zbkch_borne_env<'a>(stated: &'a [(&'static str, std::ffi::OsString)]) -> Vec<(&'static str, &'a OsStr)> {
    stated
        .iter()
        .map(|(name, value)| (*name, value.as_os_str()))
        .collect()
}

/// uv's own verbs and flags, spelled once each.
///
/// THE SPELLINGS ARE UV'S AND THEY DIFFER PER SUBCOMMAND, which is why they are
/// constants rather than literals at the call. The lock-authoring door takes a
/// negative flag where a reader would expect a valued one — `--no-python-downloads`
/// and not a `--python-downloads <value>` pair — so uv's flag surface is not
/// uniform enough to spell from memory at a call site
/// (BKSPY-Python.adoc "The Configuration Bench").
///
/// The spelling is the tool's own, declared as a xenonym spelling line under
/// that authority's carrier (BKSCL-Collar.adoc "The Toolchain Authorities"),
/// which is where the ruling lives now; VOr_9ww honours the carrier and stands
/// the value down.
const ZBKCH_UV_PYTHON: &str = "python";
const ZBKCH_UV_INSTALL: &str = "install";
const ZBKCH_UV_NO_BIN: &str = "--no-bin";
const ZBKCH_UV_SYNC: &str = "sync";
const ZBKCH_UV_LOCKED: &str = "--locked";
const ZBKCH_UV_PROJECT: &str = "--project";
