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

//! The selection pipeline: ask the runner to name every hurdle, match the
//! caller's patterns HERE, and hand the runner exactly the names that matched
//! (BKSNC-Kennelcraft.adoc "The selection pipeline").
//!
//! WHY THE MATCHING IS THE KENNEL'S AND NOT THE RUNNER'S. Each runner has a
//! filter grammar of its own — cargo's harness matches substrings, nextest
//! reads a filterset language — so a pattern handed through would mean
//! different things at different collars, and the same drive would narrow
//! differently depending on a field nobody typed. The kennel matches in ONE
//! dialect, the rust regex dialect taken as is, and reaches the runner with
//! exact names only, its own grammar bypassed.
//!
//! NO RUNNER'S ANSWER TO AN EMPTY SELECTION IS TRUSTED. Cargo's harness reports
//! a filter that matched nothing as a PASS, which is a green that means nothing;
//! nextest is stricter but is not asked to be. The kennel counts its own matches
//! and refuses before a runner is spawned at all.
//!
//! THE TWO LISTING PARSERS ARE MEMBRANES, one per runner, and hold the Palisade
//! discipline the voice pace's warrant states: the foreign shape is named at the
//! membrane, contained in one function, and anything outside the surveyed
//! signature fails fast rather than being absorbed. Both signatures were
//! CAPTURED rather than remembered — driven once each at seat
//! 3f6a0e36fa29ced26a8d4a77c38979d79368127d, through this module's own listing
//! invocation and this crate's own leash, and transcribed below from what the
//! runners actually said.

use crate::bkcl_leash;
use crate::bkcm_mush;
use crate::bkcr_resolve::bkcr_Collar;
use regex::Regex;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// A listing invocation: what the leash is to spawn to make a runner name its
/// hurdles, and over which manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkci_Listing {
    pub manifest: PathBuf,
    pub verb: String,
    pub rest: Vec<OsString>,
}

/// What a narrowing came to: the whole suite as the runner named it, and the
/// part of it the caller's patterns matched.
///
/// BOTH COUNTS ARE THE KENNEL'S OWN, taken from the listing rather than from a
/// run, which is what lets the verdict say "of how many" honestly even for a
/// selection the runner would have reported as a bare pass.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkci_Narrowing {
    /// Every hurdle the runner named, in the order it named them.
    pub held: Vec<String>,
    /// The hurdles the patterns matched, in listing order and without repeats.
    pub chosen: Vec<String>,
}

impl bkci_Narrowing {
    /// The narrowing as the verdict says it.
    ///
    /// PASSED AND FAILED ARE NOT SAID HERE, and the omission is deliberate
    /// rather than an oversight: this door hands the runner the terminal and
    /// reads no result stream, so a passed count stated from here would be a
    /// number the kennel did not observe. What it observed is the narrowing, and
    /// the runner's own summary carries the rest until the door that renders a
    /// bounded voice from a captured stream stands.
    pub fn bkci_verdict(&self) -> String {
        format!("ran {} of {}", self.chosen.len(), self.held.len())
    }
}

const ZBKCI_VERB_TEST: &str = "test";
/// The runner's own subcommand, cited from the composer that chooses it
/// (`bkcm_mush::BKCM_VERB_NEXTEST`) rather than spelled again here: the leash
/// resolves which program answers this verb, so a second spelling that drifted
/// would ask cargo for a listing the launch spawns a kibble for.
const ZBKCI_VERB_NEXTEST: &str = bkcm_mush::BKCM_VERB_NEXTEST;

/// The two listing words, one each from two runners that answer the same
/// question in different words. Each spelling is its own tool's, declared as a
/// xenonym spelling line under that authority's carrier
/// (BKSCL-Collar.adoc "The Toolchain Authorities") — nextest's verb and the
/// harness's flag under separate authorities, never merged.
const ZBKCI_NEXTEST_LIST: &str = "list";
const ZBKCI_HARNESS_LIST: &str = "--list";

const ZBKCI_HARNESS_EXACT: &str = "--exact";
const ZBKCI_NEXTEST_FILTERSET: &str = "-E";

/// What cargo's harness suffixes a hurdle's name with on a listing line, and
/// what it suffixes a benchmark with. Captured, not remembered.
const ZBKCI_HARNESS_TEST_SUFFIX: &str = ": test";
const ZBKCI_HARNESS_BENCH_SUFFIX: &str = ": benchmark";

/// Compose the listing invocation for one suite collar.
///
/// THE SAME SHAPE THE RUN IS COMPOSED FROM, narrowed to its listing verb: the
/// profile, the features and the target ride exactly as they do at the run,
/// because a listing taken under a different shape would name a different set of
/// hurdles than the run it is about to narrow.
pub fn bkci_listing(
    repository: &std::path::Path,
    collar: &bkcr_Collar,
) -> Result<bkci_Listing, String> {
    let launch = bkcm_mush::bkcm_suite(repository, collar)?;

    match collar.bkcr_field("BKRR_RUNNER") {
        bkcm_mush::BKCM_RUNNER_CARGO => {
            // The listing flag belongs to the test binary and never to cargo, so
            // it rides past the separator.
            let mut rest = launch.rest.clone();
            rest.push(OsString::from(bkcl_leash::BKCL_SEPARATOR));
            rest.push(OsString::from(ZBKCI_HARNESS_LIST));
            Ok(bkci_Listing { manifest: launch.manifest, verb: ZBKCI_VERB_TEST.to_string(), rest })
        }
        bkcm_mush::BKCM_RUNNER_NEXTEST => {
            // Nextest lists under a subcommand of its own rather than under a
            // flag, so the run's leading word is REPLACED rather than added to.
            //
            // THE RUN'S OWN EMPTY-SUITE REFUSAL FLAG MUST NOT CROSS. `nextest
            // run` accepts it and `nextest list` refuses it outright, so a
            // listing that carried it across would die on the runner's own
            // unexpected-argument complaint before naming a single hurdle.
            // Dropped by the SAME constant the run composes it from
            // (`bkcm_mush::BKCM_NO_TESTS_FLAG`) rather than a second literal,
            // so a respelling there cannot leave a stale filter here that
            // silently stops dropping anything.
            let no_tests_flag = OsString::from(bkcm_mush::BKCM_NO_TESTS_FLAG);
            let mut rest: Vec<OsString> = vec![OsString::from(ZBKCI_NEXTEST_LIST)];
            rest.extend(
                launch
                    .rest
                    .iter()
                    .skip(1)
                    .filter(|argument| **argument != no_tests_flag)
                    .cloned(),
            );
            Ok(bkci_Listing { manifest: launch.manifest, verb: ZBKCI_VERB_NEXTEST.to_string(), rest })
        }
        other => Err(format!(
            "the collar declares the runner '{}', which is no runner the kennel spawns",
            other
        )),
    }
}

/// Ask the runner to name its hurdles, and read back the names.
///
/// The recall face is taken rather than the driven one: this is a program
/// reading a runner's answer, not an operator watching a compiler work. A
/// listing that did not land refuses carrying the runner's own account, because
/// a parse over the output of a refused invocation would report an empty suite.
pub fn bkci_recite(
    repository: &std::path::Path,
    collar: &bkcr_Collar,
    listing: &bkci_Listing,
) -> Result<Vec<String>, String> {
    // THE LISTING COMPILES THE TENANT, so the collar's position rides here as it
    // does at the run and at the converge: a runner asked to name its hurdles
    // builds the suite first, and a tenant's build script reading the position
    // dies at the listing exactly as it dies at the launch. One reading, stated
    // by every door holding the collar (BKSNC-Kennelcraft.adoc "Currency by
    // git position"); the walk and its placement are the run's, at
    // `bkcm_mush::bkcm_run`, and not restated here.
    let seat = crate::bkce_election::bkce_seat(repository, collar.bkcr_field("BKRR_ROOTS"))?;

    let recall = bkcl_leash::bkcl_recall(
        repository,
        &listing.manifest,
        &listing.verb,
        &listing.rest,
        &[(crate::bkch_heel::BKCH_COLLAR_POSITION_VAR, OsStr::new(seat.as_str()))],
    )?;

    if !recall.run.bkcl_landed() {
        return Err(format!(
            "the runner would not list the suite's hurdles: {} exited {}. Nothing was narrowed \
             and nothing ran — a selection cannot be matched against a listing that was never \
             taken.\n{}",
            recall.run.spelling,
            match recall.run.code {
                Some(code) => code.to_string(),
                None => "on a signal".to_string(),
            },
            recall.grievance.trim()
        ));
    }

    let said = String::from_utf8_lossy(&recall.said);

    match collar.bkcr_field("BKRR_RUNNER") {
        bkcm_mush::BKCM_RUNNER_CARGO => bkci_harness_listing(&said),
        bkcm_mush::BKCM_RUNNER_NEXTEST => bkci_nextest_listing(&said),
        other => Err(format!(
            "the collar declares the runner '{}', which is no runner the kennel spawns",
            other
        )),
    }
}

/// THE MEMBRANE FOR CARGO'S OWN HARNESS. The surveyed signature, captured at
/// seat 3f6a0e36fa29ced26a8d4a77c38979d79368127d by driving this module's
/// listing invocation over `suite-bkmk`:
///
/// ```text
/// bktj_json::bktj_escapes_resolve: test
/// bktl_leash::bktl_a_pin_naming_no_channel_refuses: test
///
/// 70 tests, 0 benchmarks
/// 0 tests, 0 benchmarks
/// bktd_the_door_stands_over_a_clean_lure: test
///
/// 3 tests, 0 benchmarks
/// ```
///
/// Three shapes and no fourth: a hurdle line ending in the test suffix, a
/// benchmark line, and a per-binary tally. ONE MANIFEST YIELDS SEVERAL BLOCKS —
/// one per test target, each with its own tally, and a target holding nothing
/// still tallies — so the parse is over the whole stream rather than over a
/// block, and the tallies are stepped past rather than summed: summing them
/// would be a second count of a thing already counted by reading, free to
/// disagree with it.
///
/// A LINE OUTSIDE THE THREE REFUSES. Absorbing an unsurveyed shape is how a
/// membrane turns into a guess, and a hurdle silently dropped from a listing is
/// a hurdle silently dropped from every narrowing taken against it.
pub fn bkci_harness_listing(said: &str) -> Result<Vec<String>, String> {
    let tally = Regex::new(r"^\d+ tests?, \d+ benchmarks?$")
        .map_err(|err| format!("the harness per-binary count line does not compile: {}", err))?;

    let mut named = Vec::new();

    for line in said.lines() {
        let trimmed = line.trim_end();

        if trimmed.trim().is_empty() || tally.is_match(trimmed.trim()) {
            continue;
        }

        if let Some(name) = trimmed.strip_suffix(ZBKCI_HARNESS_TEST_SUFFIX) {
            named.push(name.to_string());
            continue;
        }

        // A benchmark is named by the listing and is not a hurdle: the suite the
        // kennel narrows is its tests. Stepped past deliberately, and named here
        // so the next reader meets the ruling rather than a silent branch.
        if trimmed.strip_suffix(ZBKCI_HARNESS_BENCH_SUFFIX).is_some() {
            continue;
        }

        return Err(format!(
            "cargo's harness named a hurdle in a shape the kennel has not surveyed: {:?}. The \
             shapes it holds are a name suffixed '{}', a name suffixed '{}', and a per-binary \
             tally. Refused rather than skipped: a listing line the kennel cannot read is a \
             hurdle it would silently drop from every narrowing taken against it",
            trimmed, ZBKCI_HARNESS_TEST_SUFFIX, ZBKCI_HARNESS_BENCH_SUFFIX
        ));
    }

    Ok(named)
}

/// THE MEMBRANE FOR NEXTEST. The surveyed signature, captured at seat
/// 3f6a0e36fa29ced26a8d4a77c38979d79368127d by driving this module's listing
/// invocation over `suite-vof`:
///
/// ```text
/// vof vofc_registry::tests::vofc_a_known_prefix_finds_its_cipher
/// vof vofe_emplace::tests::vofe_is_hook_file
/// ```
///
/// One shape: a binary id, one space, the hurdle's name. NO TALLY AND NO
/// HEADING, which is the whole of what parts this membrane from the harness's.
///
/// THE BINARY ID IS READ AND DROPPED. What the kennel hands back is the name a
/// filterset will match on, and the id is how nextest says which target holds
/// it; carrying it would put a word into the selection that the runner's own
/// exact form does not take.
pub fn bkci_nextest_listing(said: &str) -> Result<Vec<String>, String> {
    let named_line = Regex::new(r"^(?P<binary>\S+) (?P<hurdle>\S+)$")
        .map_err(|err| format!("the nextest listing shape does not compile: {}", err))?;

    let mut named = Vec::new();

    for line in said.lines() {
        let trimmed = line.trim_end();

        if trimmed.trim().is_empty() {
            continue;
        }

        let Some(caught) = named_line.captures(trimmed) else {
            return Err(format!(
                "nextest named a hurdle in a shape the kennel has not surveyed: {:?}. The shape it \
                 holds is a binary id, one space, and the hurdle's name. Refused rather than \
                 skipped: a listing line the kennel cannot read is a hurdle it would silently drop \
                 from every narrowing taken against it",
                trimmed
            ));
        };

        named.push(caught["hurdle"].to_string());
    }

    Ok(named)
}

/// THE MEMBRANE FOR PYTEST. The surveyed signature, captured at seat
/// ace9b08ec84a229f4c57372371c514705c6ad789 by driving this module's listing
/// invocation over the capture lure — a project holding four cases, one holding
/// a single case, and one holding none:
///
/// ```text
/// tests/test_alpha.py::test_one
/// tests/test_alpha.py::test_two
/// tests/test_beta.py::test_three
/// tests/test_beta.py::test_red
///
/// 4 tests collected in 0.00s
/// ```
///
/// ```text
/// tests/test_beta.py::test_red
///
/// 1 test collected in 0.00s
/// ```
///
/// ```text
///
/// no tests collected in 0.00s
/// ```
///
/// Two shapes and no third: a bare node id on its own line, and one closing
/// tally. THE TALLY IS STEPPED PAST RATHER THAN SUMMED, on the harness
/// membrane's own ground — summing it would be a second count of a thing already
/// counted by reading, free to disagree with it — and its THREE spellings are
/// all admitted, the count being singular at one case and the word `no` standing
/// where a numeral would at none.
///
/// THE NODE ID IS TAKEN WHOLE AND NEVER SPLIT. It is what pytest is handed back
/// as an exact selection, so a reader that parted the file from the case would
/// be inventing a spelling the runner does not take.
///
/// A LINE OUTSIDE THE TWO REFUSES. Absorbing an unsurveyed shape is how a
/// membrane turns into a guess, and a hurdle silently dropped from a listing is
/// a hurdle silently dropped from every narrowing taken against it.
///
/// RETIREMENT CONDITION. Retire this membrane when the kennel can ask pytest for
/// a machine-readable listing without provisioning a plugin. The captured log
/// stands at `hist-bkmw-k-sh-20260910-145316-1350227-453.txt`.
pub fn bkci_pytest_listing(said: &str) -> Result<Vec<String>, String> {
    let tally = Regex::new(r"^(?:\d+ tests?|no tests) collected(?: in [\d.]+s)?$")
        .map_err(|err| format!("the pytest collection count line does not compile: {}", err))?;

    let mut named = Vec::new();

    for line in said.lines() {
        let trimmed = line.trim_end();

        if trimmed.trim().is_empty() || tally.is_match(trimmed.trim()) {
            continue;
        }

        // A NODE ID CARRIES NO WHITESPACE, which is what tells one from any other
        // line pytest might print. Checked rather than assumed: a line with a
        // space in it is a shape this membrane has not surveyed, whatever it
        // looks like.
        if !trimmed.trim().contains(char::is_whitespace) {
            named.push(trimmed.trim().to_string());
            continue;
        }

        return Err(format!(
            "pytest named a hurdle in a shape the kennel has not surveyed: {:?}. The shapes it \
             holds are a bare node id on its own line and one closing collection tally. Refused \
             rather than skipped: a listing line the kennel cannot read is a hurdle it would \
             silently drop from every narrowing taken against it",
            trimmed
        ));
    }

    Ok(named)
}

/// Match the caller's patterns against the listing, IN THE KENNEL.
///
/// THE DIALECT IS TAKEN AS IS and no subset is invented: a pattern is a rust
/// regular expression, compiled as written, and one that does not compile
/// refuses naming itself rather than being reinterpreted as a literal. A
/// pattern is unanchored, so it matches a hurdle whose name CONTAINS it, which
/// is what makes a module name a usable selection.
///
/// A hurdle matched by two patterns is chosen once, and the order is the
/// runner's own rather than the caller's: the selection is a subset of the
/// listing, never a re-ordering of it.
pub fn bkci_choose(held: &[String], patterns: &[String]) -> Result<bkci_Narrowing, String> {
    let mut compiled = Vec::with_capacity(patterns.len());

    for pattern in patterns {
        let one = Regex::new(pattern).map_err(|err| {
            format!(
                "the selection pattern {:?} is not a rust regular expression: {}. The dialect is \
                 taken as is and no subset of it is invented here, so a pattern that does not \
                 compile is refused rather than read as a literal",
                pattern, err
            )
        })?;
        compiled.push(one);
    }

    let chosen = held
        .iter()
        .filter(|hurdle| compiled.iter().any(|one| one.is_match(hurdle)))
        .cloned()
        .collect();

    Ok(bkci_Narrowing { held: held.to_vec(), chosen })
}

/// Spell the chosen hurdles as the runner's own EXACT selection.
///
/// EXACT AT BOTH RUNNERS, and each spells it its own way. Cargo's harness takes
/// names past the separator under its exactness flag; nextest takes a filterset,
/// where `test(=name)` is the exact form and `+` is its union. Neither is handed
/// the caller's pattern: the matching already happened here, and what crosses
/// the boundary is a set of names.
pub fn bkci_selection(runner: &str, chosen: &[String]) -> Result<Vec<OsString>, String> {
    match runner {
        bkcm_mush::BKCM_RUNNER_CARGO => {
            let mut spelled = vec![
                OsString::from(bkcl_leash::BKCL_SEPARATOR),
                OsString::from(ZBKCI_HARNESS_EXACT),
            ];
            spelled.extend(chosen.iter().map(OsString::from));
            Ok(spelled)
        }
        bkcm_mush::BKCM_RUNNER_NEXTEST => {
            let filterset = chosen
                .iter()
                .map(|hurdle| format!("test(={})", hurdle))
                .collect::<Vec<String>>()
                .join(" + ");
            Ok(vec![OsString::from(ZBKCI_NEXTEST_FILTERSET), OsString::from(filterset)])
        }
        // PYTEST TAKES ITS SELECTION POSITIONALLY AND IS EXACT BY CONSTRUCTION.
        // A node id names one case outright, so there is no exactness flag to
        // spell and no filter grammar to bypass: the names go across as
        // arguments, in listing order, exactly as the listing spelled them.
        //
        // THE SPELLING AGREEING WITH THE LISTING'S IS WHAT MAKES THIS WORK, and
        // it is not free — pytest renders a node id relative to its ROOTDIR when
        // it lists and relative to the INVOCATION DIRECTORY when it runs. The
        // leash states the working directory for exactly this reason
        // (`bkcl_leash::bkcl_uv`), so the two renderings are one.
        crate::bkcx_python::BKCX_RUNNER_PYTEST => {
            Ok(chosen.iter().map(OsString::from).collect())
        }
        other => Err(format!(
            "the collar declares the runner '{}', which is no runner the kennel spawns",
            other
        )),
    }
}

// eof
