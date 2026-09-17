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

//! The launch seam's composition hurdles.
//!
//! They compose a collar in hand rather than over a lure, because what is under
//! test is what the kennel SPELLS from a collar's declarations — a pure function
//! of the collar — and a composed repository would prove nothing the resolver's
//! own hurdles do not. The drive that actually spawns a runner is the suite's own
//! green, which the kennel's door proves over the committed roster.

use super::bkcm_mush::{bkcm_suite, BKCM_RUNNER_CARGO, BKCM_RUNNER_NEXTEST};
use super::bkcr_resolve::bkcr_Collar;
use bkl::bklrc_catena::bklrc_admit;
use std::path::{Path, PathBuf};

/// Cargo's own build-profile flag, spelled here INDEPENDENTLY of the module
/// under test rather than imported from it.
///
/// A hurdle that read the implementation's own constant would assert that the
/// module agrees with itself, which it always does; what is under test is that
/// the word reaching each runner is the word THAT RUNNER defines. Both flags are
/// foreign names — cargo's and nextest's — so the hurdle quotes them, as the
/// leash quotes `--locked`.
const ZBKTM_PROFILE_FLAG_CARGO: &str = "--profile";

/// The seat every composition below is joined from. A collar's manifest is a path
/// reference joined from the repository root, so the root has to be something;
/// this is the something, and nothing reads it off a disk.
const ZBKTM_SEAT: &str = "/seat";

/// A suite collar in hand, under cargo's own runner with the whole manifest's
/// tests as its target — the shape the kennel's own collar carries.
const ZBKTM_CARGO: &str = "\
BKRR_COLLAR=\"suite-lure\"
BKRR_KIND=\"bknre_suite\"
BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"
BKRR_TARGET=\"bknre_manifest\"
BKRR_ROOTS=\"Tools/lure/src\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"test\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RUNNER=\"bknre_cargo\"
BKRR_TONGUE=\"bknre_harness\"
";

/// The same suite under nextest, with a named test target and a feature
/// selection — every field that changes what is spelled, moved at once so the
/// contrast with the collar above is the whole of what the assertions read.
const ZBKTM_NEXTEST: &str = "\
BKRR_COLLAR=\"suite-lure\"
BKRR_KIND=\"bknre_suite\"
BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"
BKRR_TARGET=\"lure_integration\"
BKRR_ROOTS=\"Tools/lure/src\"
BKRR_FEATURES=\"alpha beta\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RUNNER=\"bknre_nextest\"
BKRR_TONGUE=\"bknre_nextest\"
";

/// A collar naming a runner the kennel does not spawn.
const ZBKTM_STRANGER: &str = "\
BKRR_COLLAR=\"suite-lure\"
BKRR_KIND=\"bknre_suite\"
BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"
BKRR_TARGET=\"bknre_manifest\"
BKRR_ROOTS=\"Tools/lure/src\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"test\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RUNNER=\"bknre_ketchtest\"
BKRR_TONGUE=\"bknre_harness\"
";

/// Compose a collar without touching a disk.
fn zbktm_collar(text: &str) -> bkcr_Collar {
    bkcr_Collar {
        name: "suite-lure".to_string(),
        instance: PathBuf::from("suite-lure"),
        regime: bklrc_admit(text, "composed").expect("the collar stands inside the subset"),
    }
}

/// THE KENNEL SPAWNS WHAT THE COLLAR SAYS AND INFERS NO RUNNER. Cargo's own
/// harness is one word; nextest is a cargo subcommand and takes two.
///
/// The pair is one rule read from both sides, and either alone would pass under a
/// composer that had hardcoded the other.
#[test]
fn bktm_the_runner_the_collar_names_is_the_one_spelled() {
    let cargo = bkcm_suite(Path::new(ZBKTM_SEAT), &zbktm_collar(ZBKTM_CARGO))
        .expect("cargo is a runner the kennel spawns");
    assert_eq!(cargo.verb, "test", "cargo's own harness, spelled: {}", cargo.bkcm_spelling());

    let nextest = bkcm_suite(Path::new(ZBKTM_SEAT), &zbktm_collar(ZBKTM_NEXTEST))
        .expect("nextest is a runner the kennel spawns");
    assert_eq!(nextest.verb, "nextest", "spelled: {}", nextest.bkcm_spelling());
    assert_eq!(
        nextest.rest.first().map(|word| word.to_string_lossy().into_owned()),
        Some("run".to_string()),
        "nextest's own run word follows the subcommand: {}",
        nextest.bkcm_spelling()
    );
}

/// A RUNNER THE KENNEL DOES NOT SPAWN REFUSES, rather than being handed to cargo
/// as a verb. The declared pair is the whole roster, and a value outside it names
/// a command nobody elected.
#[test]
fn bktm_a_runner_outside_the_declared_pair_refuses() {
    let refusal = bkcm_suite(Path::new(ZBKTM_SEAT), &zbktm_collar(ZBKTM_STRANGER))
        .expect_err("a runner outside the pair is refused");

    assert!(
        refusal.contains("bknre_ketchtest"),
        "the refusal names the value it read, so a reader is handed the repair rather than a \
         search: {}",
        refusal
    );

    for admitted in [BKCM_RUNNER_CARGO, BKCM_RUNNER_NEXTEST] {
        assert!(
            bkcm_suite(Path::new(ZBKTM_SEAT), &zbktm_collar(&ZBKTM_STRANGER.replace("bknre_ketchtest", admitted)))
                .is_ok(),
            "the control: {} composes, so the refusal above is about the value and not about the \
             collar around it",
            admitted
        );
    }
}

/// THE TARGET IS A DECLARATION EITHER WAY. The whole-manifest token names every
/// test of the manifest and spells no target flag; a named target spells one.
///
/// The pair is why the token exists at all: a door reading *whole manifest* out
/// of an ABSENCE could not tell a declaration from an omission.
#[test]
fn bktm_a_named_target_is_spelled_and_the_whole_manifest_is_not() {
    let whole = bkcm_suite(Path::new(ZBKTM_SEAT), &zbktm_collar(ZBKTM_CARGO)).expect("composes");
    assert!(
        !whole.bkcm_spelling().contains("--test"),
        "the whole manifest's tests name no one target: {}",
        whole.bkcm_spelling()
    );

    let named = bkcm_suite(Path::new(ZBKTM_SEAT), &zbktm_collar(ZBKTM_NEXTEST)).expect("composes");
    assert!(
        named.bkcm_spelling().contains("--test lure_integration"),
        "a named target is spelled: {}",
        named.bkcm_spelling()
    );
}

/// THE DECLARED SHAPE RIDES THE COLLAR AND NEVER A CALLER. The profile is spelled
/// from the collar at every launch, because a caller-spelled profile would make
/// the obedience dialect's cost bands dishonest — they are calibrated against a
/// build shape, and the wall-clock floor is verdict-affecting.
///
/// An EMPTY feature field is a declaration that there is nothing to spell, which
/// is the state nearly every collar of the first roster stands in; a field
/// carrying elements is spelled as one selection.
#[test]
fn bktm_the_declared_shape_is_spelled_and_an_empty_field_is_not() {
    let plain = bkcm_suite(Path::new(ZBKTM_SEAT), &zbktm_collar(ZBKTM_CARGO)).expect("composes");
    assert!(
        plain
            .bkcm_spelling()
            .split_whitespace()
            .any(|word| word == ZBKTM_PROFILE_FLAG_CARGO),
        "the collar's profile, spelled in the flag cargo's own harness reads: {}",
        plain.bkcm_spelling()
    );
    assert!(
        !plain.bkcm_spelling().contains("--features"),
        "an empty selection spells no flag: {}",
        plain.bkcm_spelling()
    );

    let featured = bkcm_suite(Path::new(ZBKTM_SEAT), &zbktm_collar(ZBKTM_NEXTEST)).expect("composes");

    // THE SAME DECLARED FIELD, SPELLED WITH THE OTHER RUNNER'S FLAG. Under
    // cargo's own harness `--profile` names the cargo build profile; under
    // nextest that flag names a NEXTEST profile — its own namespace, holding
    // `default` and `default-miri` — and the cargo build profile is
    // `--cargo-profile`. Spelling cargo's flag at nextest does not build under a
    // wrong profile: nextest refuses outright with "profile `release` not
    // found", which is how the divergence was found, by the selection pipeline
    // taking the kit's first listing over a nextest collar.
    assert!(
        featured.bkcm_spelling().contains("--cargo-profile release"),
        "the other collar's profile, spelled in the flag ITS runner reads: {}",
        featured.bkcm_spelling()
    );

    // THE ASSERTION ABOVE CANNOT BE A SUBSTRING TEST ALONE, which is the defect
    // that let this ship: `--cargo-profile release` CONTAINS `--profile
    // release`, so the reading that stood here passed against a flag nextest
    // refuses. The word-boundary check is what parts the two namespaces, and it
    // is spelled as its own assertion rather than folded above so that a failure
    // says which of the two went wrong.
    assert!(
        !featured
            .bkcm_spelling()
            .split_whitespace()
            .any(|word| word == ZBKTM_PROFILE_FLAG_CARGO),
        "nextest's own --profile names a NEXTEST profile, a namespace the collar's build shape is \
         not drawn from, and spelling it there is the invocation nextest refuses: {}",
        featured.bkcm_spelling()
    );
    assert!(
        featured.bkcm_spelling().contains("--features alpha beta"),
        "the control: a field carrying elements IS spelled, so the silence above is the emptiness \
         and not a flag nobody spells: {}",
        featured.bkcm_spelling()
    );
}

/// THE EMPTY-SUITE DEMAND IS SPELLED AND NOT INHERITED (operator, 260905). A
/// suite that ran nothing proved nothing, and nextest would refuse it by its own
/// default — which is exactly why the flag is spelled: a rule the kennel leans on
/// but never states is held by another program's release notes rather than by us.
///
/// CARGO'S ARM IS THE DECLARED NARROWING. Its harness carries no such flag and
/// reports a clean zero, so the composer spells nothing there; the rule reaches
/// that runner only when the kennel can read how many tests a child ran, which is
/// the selection pipeline's listing step. Asserted as an absence so the narrowing
/// is a statement rather than a silence.
#[test]
fn bktm_the_empty_suite_demand_is_spelled_where_a_runner_can_hear_it() {
    let nextest = bkcm_suite(Path::new(ZBKTM_SEAT), &zbktm_collar(ZBKTM_NEXTEST)).expect("composes");
    assert!(
        nextest.bkcm_spelling().contains("--no-tests=fail"),
        "the kennel asks for the refusal rather than inheriting it: {}",
        nextest.bkcm_spelling()
    );

    let cargo = bkcm_suite(Path::new(ZBKTM_SEAT), &zbktm_collar(ZBKTM_CARGO)).expect("composes");
    assert!(
        !cargo.bkcm_spelling().contains("--no-tests"),
        "and spells nothing at a runner that carries no such flag, rather than a word cargo          would refuse: {}",
        cargo.bkcm_spelling()
    );
}

/// THE MANIFEST IS JOINED FROM THE REPOSITORY ROOT and rides beside the verb
/// rather than inside the argument list, because the leash spells it itself — an
/// argument list still carrying it would reach cargo with the flag twice.
#[test]
fn bktm_the_manifest_is_joined_from_the_seat_and_rides_no_argument() {
    let launch = bkcm_suite(Path::new(ZBKTM_SEAT), &zbktm_collar(ZBKTM_CARGO)).expect("composes");

    assert_eq!(launch.manifest, PathBuf::from("/seat/Tools/lure/Cargo.toml"));
    assert!(
        !launch.bkcm_spelling().contains("--manifest-path"),
        "the leash spells the manifest, and a second spelling would be refused for the \
         duplication: {}",
        launch.bkcm_spelling()
    );
}

// eof
