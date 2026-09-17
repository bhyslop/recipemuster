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

//! *mush* — the launch seam (BKSNC-Kennelcraft.adoc "The doors").
//!
//! One door launches a launchable named by its collar. What it launches is the
//! collar's kind: an app runs the binary the election names, a suite runs under
//! the runner the collar declares. Both go through the leash, so the pin is
//! stated, the lock is enforced and the tackroom fence is stood on for a launch
//! exactly as for a build.
//!
//! THIS FILE COMPOSES AND NEVER REFUSES, which is the crate's own split rather
//! than a shape that happened here. The door holds the posture — it refuses an
//! uncommitted repository, refuses an invalid collar, and takes an exit — and
//! the library states what a launch IS. A refusal seated here would put a door's
//! posture in a library that has consumers holding their own.
//!
//! THE KENNEL SPAWNS WHAT THE COLLAR SAYS AND INFERS NO RUNNER. Both admitted
//! runners are declared values, and the tongue is declared beside the runner
//! rather than read off it, because a runner can be asked for an output shape
//! other than its own. Nothing here guesses either.
//!
//! A SUITE IS NEVER BORROWED, and that is why no election reaches this half of
//! the door. The runner compiles the suite from the source the seat holds, so a
//! billet's suite tests the billet's code by construction; there is no artifact
//! standing between the source and the verdict for a vintage question to be
//! asked about. The election is over apps alone.

use crate::bkcl_leash;
use crate::bkcv_voice;
use crate::bkcr_resolve::bkcr_Collar;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// The runners the collar may name, in the substrate's enum sprue form under
/// this kit's own plane. Both are declared values with standing precedent in the
/// estate, and the kennel spawns the one the collar names.
pub const BKCM_RUNNER_CARGO: &str = "bknre_cargo";
pub const BKCM_RUNNER_NEXTEST: &str = "bknre_nextest";

/// Cargo's own verb for each runner. Nextest is a cargo subcommand, so it takes
/// two words where cargo's own harness takes one.
const ZBKCM_VERB_TEST: &str = "test";

/// PUBLISHED, ALONE AMONG THE VERB WORDS, because two other modules must name
/// the same subcommand and a foreign word spelled three times is three things to
/// keep in step. The leash's kibble table asks which program answers this verb
/// and the selection pipeline spells it to ask the runner for its own listing;
/// both cite this seat rather than repeating the word. It stays the composer's
/// because this is where a launch's verb is CHOSEN — the other two consume the
/// choice.
pub const BKCM_VERB_NEXTEST: &str = "nextest";
const ZBKCM_NEXTEST_RUN: &str = "run";

/// The flags the collar's declared shape is spelled with.
///
/// THE PROFILE IS THE COLLAR'S AND NEVER A PER-INVOCATION FLAG. The obedience
/// dialect's cost bands are calibrated against a build shape and the wall-clock
/// floor is verdict-affecting, so a caller-spelled profile would make the
/// vacuity gate dishonest; wanting a fast shape and a thorough shape of one
/// suite is two collars with two calibrations
/// (BKSCL-Collar.adoc "Declared Build Shapes").
///
/// THE PROFILE'S FLAG IS THE RUNNER'S, AND THE TWO RUNNERS SPELL IT
/// DIFFERENTLY. `BKRR_PROFILE` names a CARGO BUILD profile at every collar,
/// which is what `--profile` means to cargo's own harness. It is not what
/// `--profile` means to nextest: there the word selects a NEXTEST profile, a
/// separate namespace of that runner's own configuration, and the cargo build
/// profile is `--cargo-profile`. One flag spelled for both runners therefore
/// hands nextest a value from the wrong namespace, and nextest refuses the
/// invocation outright — `error: profile `test` not found (known profiles:
/// default, default-miri)` — so every nextest collar in the estate was
/// unlaunchable through this door.
///
/// The collar is not what moves. A field declaring a build shape means one
/// thing, and a door that spawns two runners owes each of them the spelling
/// that reaches that meaning; making the collar carry a runner's flag grammar
/// would surface the runner the collar exists to hide.
const ZBKCM_PROFILE_FLAG_CARGO: &str = "--profile";
const ZBKCM_PROFILE_FLAG_NEXTEST: &str = "--cargo-profile";
const ZBKCM_FEATURES_FLAG: &str = "--features";
const ZBKCM_TEST_FLAG: &str = "--test";

/// A SUITE THAT RAN NOTHING IS RED, STATED RATHER THAN INHERITED (operator,
/// 260905). Nextest already refuses an empty selection by its own default, and
/// that is exactly why this is spelled: a rule the kennel depends on but never
/// asks for is a rule held by another program's release notes. Spelling it makes
/// the demand ours, and a nextest that changed its default would change nothing
/// here.
///
/// CARGO'S OWN HARNESS HAS NO SUCH FLAG and reports a clean zero, so this rule
/// reaches only the runner that can be told it. Closing the other half means
/// reading how many tests the child ran, which is the selection pipeline's
/// listing step and not this composer's to anticipate.
///
/// PUBLISHED, ON `BKCM_VERB_NEXTEST`'s OWN GROUND: the selection pipeline's
/// listing arm must NOT carry this flag across into nextest's `list`
/// subcommand, which refuses it outright, so that arm drops it by this same
/// name rather than a second literal — a respelling here cannot then leave a
/// stale filter there that silently stops dropping anything.
pub const BKCM_NO_TESTS_FLAG: &str = "--no-tests=fail";

/// What `BKRR_TARGET` carries where the suite is a whole manifest's tests
/// rather than one named target.
const ZBKCM_TARGET_MANIFEST: &str = "bknre_manifest";

/// A composed launch: what the leash is to spawn, and over which manifest.
///
/// The manifest rides beside the verb because the leash spells it itself — a
/// caller that carried it in the argument list would reach cargo with the flag
/// twice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkcm_Launch {
    /// The manifest the leash is told, joined from the repository root.
    pub manifest: PathBuf,
    /// Cargo's own verb: the runner's first word.
    pub verb: String,
    /// Everything after the verb, in the order the leash will spell it.
    pub rest: Vec<OsString>,
    /// Whether this launch spawns nextest — a program the kennel did not build
    /// and whose version it declares (`bkcl_leash::BKCL_NEXTEST_VERSION`).
    ///
    /// IT RIDES THE COMPOSITION RATHER THAN BEING RE-READ AT THE DOOR. The
    /// runner is already decided here, and a door that read the collar's field
    /// a second time to learn what this composition spawns would be a second
    /// answer to a question this one already answered — free to disagree the
    /// moment a runner is added. What the door does with it is refuse, which is
    /// the door's half and not this file's.
    pub nextest: bool,
}

impl bkcm_Launch {
    /// The invocation as it will be spelled, for a diagnostic that means to show
    /// what was actually asked. The leash composes the real one; this is the part
    /// this module chose.
    pub fn bkcm_spelling(&self) -> String {
        let mut said = self.verb.clone();
        for argument in &self.rest {
            said.push(' ');
            said.push_str(&argument.to_string_lossy());
        }
        said
    }
}

/// Compose the launch for one suite collar.
///
/// EVERY DECLARED FIELD IS SPELLED AND NONE IS INFERRED. The runner decides the
/// verb, the target decides whether a test target is named, and the profile and
/// the features ride whatever the collar declared. A field standing empty is the
/// collar saying there is nothing to spell, which is a declaration rather than an
/// omission — the roster's own `BKRR_FEATURES` is empty at nearly every collar.
pub fn bkcm_suite(repository: &std::path::Path, collar: &bkcr_Collar) -> Result<bkcm_Launch, String> {
    // AHEAD OF THE COMPOSITION, so the launch seam refuses for the same reason
    // and in the same words as the other two roads. It is asked here rather than
    // at the run because this is where the collar is still in hand — and a suite
    // whose crate cannot be compiled at all has no launch to compose, which
    // makes the absence a fact about the composition rather than about the run.
    crate::bkcr_resolve::bkcr_exergue_stands(repository, collar)?;

    let runner = collar.bkcr_field("BKRR_RUNNER");

    let (verb, mut rest, profile_flag) = match runner {
        BKCM_RUNNER_CARGO => (
            ZBKCM_VERB_TEST.to_string(),
            Vec::new(),
            ZBKCM_PROFILE_FLAG_CARGO,
        ),
        BKCM_RUNNER_NEXTEST => (
            BKCM_VERB_NEXTEST.to_string(),
            vec![
                OsString::from(ZBKCM_NEXTEST_RUN),
                OsString::from(BKCM_NO_TESTS_FLAG),
            ],
            ZBKCM_PROFILE_FLAG_NEXTEST,
        ),
        other => {
            return Err(format!(
                "the collar declares the runner '{}', which is no runner the kennel spawns — it \
                 spawns what a collar says and infers none, so a value outside the declared pair \
                 names a command nobody elected",
                other
            ))
        }
    };

    let profile = collar.bkcr_field("BKRR_PROFILE");
    if !profile.trim().is_empty() {
        rest.push(OsString::from(profile_flag));
        rest.push(OsString::from(profile.trim()));
    }

    let features = collar.bkcr_field("BKRR_FEATURES");
    if !features.trim().is_empty() {
        rest.push(OsString::from(ZBKCM_FEATURES_FLAG));
        rest.push(OsString::from(features.split_whitespace().collect::<Vec<&str>>().join(" ")));
    }

    // THE TARGET IS A DECLARATION EITHER WAY, which is why the whole-manifest
    // form carries a token rather than standing vacant: a door reading "every
    // test of this manifest" out of an ABSENCE could not tell a declaration from
    // an omission.
    let target = collar.bkcr_field("BKRR_TARGET");
    if target != ZBKCM_TARGET_MANIFEST {
        rest.push(OsString::from(ZBKCM_TEST_FLAG));
        rest.push(OsString::from(target));
    }

    Ok(bkcm_Launch {
        manifest: repository.join(collar.bkcr_field("BKRR_MANIFEST")),
        verb,
        rest,
        nextest: runner == BKCM_RUNNER_NEXTEST,
    })
}

/// Run a composed suite as a CAPTURED CHILD, rendering the door's own voice from
/// its stream and writing the record as it goes.
///
/// THE ATTENDED FACE, WHICH IS WHY EVERY TEST DOOR IN THE ESTATE SOUNDS THE SAME.
/// The tenant's streams are consumed live and the console is the kennel's own —
/// a heartbeat, a bounded line per failure, a verdict — whatever ran underneath
/// (BKSNC-Kennelcraft.adoc "Bounded output"). The driven face this once took
/// could not do it: a child holding this process's streams renders itself, and
/// two runners rendering themselves is the unevenness the voice exists to end.
///
/// THE TALLY RIDES BACK BESIDE THE EXIT because they answer different questions
/// and the caller needs both. The exit is the verdict; the tally is what the
/// kennel counted for itself, which is what a narrowing is reported against.
///
/// THE PREAMBLE IS WHAT HAPPENED BEFORE THE LAUNCH, in the door's own words, and
/// it is taken here because the record cannot be written anywhere else. The
/// family opens truncating, so a step running ahead of this one cannot open its
/// own and keep what it wrote — the second open would erase the first. Handing
/// the lines in is what lets the muzzle's clean stream stand in the record ahead
/// of the run it cleared, in the order the two actually happened.
pub fn bkcm_run(
    repository: &std::path::Path,
    collar: &bkcr_Collar,
    launch: &bkcm_Launch,
    tongue: &str,
    preamble: &[String],
) -> Result<(bkcl_leash::bkcl_Run, bkcv_voice::bkcv_Tally), String> {
    // THE COLLAR'S POSITION RIDES ACROSS THE EXEC BOUNDARY here exactly as it
    // does at the converge (`bkch_heel::bkch_build`), and for the envelope's
    // own reason: the door that holds the collar walks it and states it, and the
    // crate's build script compiles it in (BKSNC-Kennelcraft.adoc "Currency
    // by git position"). A suite is compiled from source by the runner, so a
    // tenant declaring a build script that reads the position dies under this
    // door unless this door states it — which is how one tenant's suite stood
    // red under every kennel road while green under its own launcher.
    //
    // WALKED AHEAD OF THE RECORD, so a refusal here leaves no half-written
    // family behind it: the walk asks git and can refuse, and a voice opened
    // ahead of a launch that never happens would stand as a record of nothing.
    // Walked at the drive and not at the composition, on the heel's own line —
    // composing is pure over the collar and provable with no repository on disk;
    // walking asks git.
    //
    // THE COLLAR IS TAKEN HERE rather than carried on the launch, for the reason
    // the heel gives: the launch carries what the report owes, and this position
    // is not reported.
    let seat = crate::bkce_election::bkce_seat(repository, collar.bkcr_field("BKRR_ROOTS"))?;

    let mut voice = bkcv_voice::bkcv_Voice::bkcv_open(tongue, &bkcv_voice::bkcv_Record::bkcv_invocation())?;

    for line in preamble {
        voice.bkcv_mark(line)?;
    }

    // THE COMPOSED INVOCATION IS A DOOR MARK IN THE RECORD, which is what the
    // record is for: the child's whole stream interleaved with the door's own
    // marks. It is a fact about what the kennel DECIDED from the collar — which
    // runner, which profile, which target — and a reader asking why a suite ran
    // the way it did reads it here rather than re-deriving it from the collar.
    voice.bkcv_mark(&format!("launching: {}", launch.bkcm_spelling()))?;

    // A HEED THAT CANNOT REFUSE, so a failing record does not lose the run. What
    // the child said is gone the moment it is not taken, and a write that failed
    // mid-stream is worth a diagnostic rather than an abandonment; the refusal is
    // held and surfaced at the close, where the run still exists to be reported.
    let mut grievance: Option<String> = None;
    let run = bkcl_leash::bkcl_attend(
        repository,
        &launch.manifest,
        &launch.verb,
        &launch.rest,
        &[(crate::bkch_heel::BKCH_COLLAR_POSITION_VAR, OsStr::new(seat.as_str()))],
        bkcv_voice::BKCV_CADENCE,
        |heard| {
            if let Err(err) = voice.bkcv_heed(heard) {
                if grievance.is_none() {
                    grievance = Some(err);
                }
            }
        },
    )?;

    let tally = voice.bkcv_close(run.code)?;

    if let Some(err) = grievance {
        return Err(err);
    }

    Ok((run, tally))
}

/// Run a composed PYTHON suite as an attended child, rendering the door's own
/// voice from its stream and writing the record as it goes.
///
/// THE SAME POSTURE AS THE CARGO RUN BESIDE IT, and deliberately so: every test
/// door in the estate sounds the same, whatever ran underneath
/// (BKSNC-Kennelcraft.adoc "Bounded output"). What parts the two is the
/// leash face they reach — uv answers to none of the cargo composition's
/// discipline — and nothing else. The preamble, the tally, and the refusal that
/// is held rather than abandoned are all the cargo run's own reasoning, stated
/// there and not restated here.
///
/// THE MUZZLE HAS NO PREAMBLE TO HAND IN, WHICH IS A RULING RATHER THAN AN
/// OMISSION. The muzzle is the rust plane's lint step and a python suite has
/// none — the python guide's linter election is deliberately deferred — so this
/// arm records no muzzle marks and invents no python lint. The parameter stands
/// anyway because the record's opening is what it is for: a step running ahead
/// of this one could not open its own family and keep what it wrote.
pub fn bkcm_python(
    repository: &std::path::Path,
    at: &std::path::Path,
    spelled: &[OsString],
    stated: &[(&str, &std::ffi::OsStr)],
    tongue: &str,
    preamble: &[String],
) -> Result<(bkcl_leash::bkcl_Run, bkcv_voice::bkcv_Tally), String> {
    let mut voice = bkcv_voice::bkcv_Voice::bkcv_open(tongue, &bkcv_voice::bkcv_Record::bkcv_invocation())?;

    for line in preamble {
        voice.bkcv_mark(line)?;
    }

    voice.bkcv_mark(&format!(
        "launching: {}",
        spelled
            .iter()
            .map(|one| one.to_string_lossy().into_owned())
            .collect::<Vec<String>>()
            .join(" ")
    ))?;

    let mut grievance: Option<String> = None;
    let run = bkcl_leash::bkcl_uv_attend(
        repository,
        at,
        spelled,
        stated,
        bkcv_voice::BKCV_CADENCE,
        |heard| {
            if let Err(err) = voice.bkcv_heed(heard) {
                if grievance.is_none() {
                    grievance = Some(err);
                }
            }
        },
    )?;

    let tally = voice.bkcv_close(run.code)?;

    if let Some(err) = grievance {
        return Err(err);
    }

    Ok((run, tally))
}

// eof
