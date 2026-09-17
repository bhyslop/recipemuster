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

//! The selection pipeline's hurdles — the matching and the two listing
//! membranes, proven in-process.
//!
//! THE RUNNING PROOFS ARE NOT HERE. That a narrowing runs exactly the hurdles it
//! named, that an empty match spawns nothing, and that a bare drive is unchanged
//! are facts about a SPAWNED kennel over a lure, and they stand as such in
//! `tests/bkti_selection.rs`. What stands here is what can be proven without
//! spawning anything: the shapes the membranes admit, the shapes they refuse,
//! and the dialect the matching is done in.

use super::bkci_pipeline::*;
use super::bkcm_mush;
use super::bkcr_resolve::bkcr_Collar;
use bkl::bklrc_catena::bklrc_admit;
use std::path::{Path, PathBuf};

/// The harness's listing, as captured over `suite-bkmk` — several hurdles, a
/// per-binary tally, an empty binary's tally, and a second block.
const ZBKTI_HARNESS: &str = "\
bktj_json::bktj_escapes_resolve: test
bktj_json::bktj_runaway_nesting_refuses: test

70 tests, 0 benchmarks
0 tests, 0 benchmarks
bktd_the_door_stands_over_a_clean_lure: test

3 tests, 0 benchmarks
0 tests, 0 benchmarks
";

/// Nextest's listing, as captured over `suite-vof` — a binary id, one space, the
/// hurdle's name, and no tally at all.
const ZBKTI_NEXTEST: &str = "\
vof vofc_registry::tests::vofc_a_known_prefix_finds_its_cipher
vof vofe_emplace::tests::vofe_is_hook_file
";

#[test]
fn bkti_the_harness_listing_reads_every_block_and_steps_past_the_tallies() {
    let named = bkci_harness_listing(ZBKTI_HARNESS).expect("the captured shape reads");

    // ACROSS BLOCKS, which is the property a per-block parse would lose: the
    // third name stands in a second test target with a tally of its own between.
    assert_eq!(
        named,
        vec![
            "bktj_json::bktj_escapes_resolve",
            "bktj_json::bktj_runaway_nesting_refuses",
            "bktd_the_door_stands_over_a_clean_lure",
        ]
    );
}

#[test]
fn bkti_a_benchmark_is_not_a_hurdle() {
    let named = bkci_harness_listing("a_bench: benchmark\na_hurdle: test\n\n0 tests, 1 benchmark\n")
        .expect("a benchmark line is a surveyed shape");

    assert_eq!(named, vec!["a_hurdle"]);
}

#[test]
fn bkti_an_unsurveyed_harness_line_refuses_rather_than_being_skipped() {
    // A hurdle the kennel silently dropped from a listing is one it would drop
    // from every narrowing taken against that listing, so the membrane fails
    // fast outside its surveyed signature rather than absorbing.
    let grievance = bkci_harness_listing("bktj_json::something: unforeseen\n")
        .expect_err("an unsurveyed shape refuses");

    assert!(
        grievance.contains("has not surveyed"),
        "the refusal names the membrane's own limit: {}",
        grievance
    );
}

#[test]
fn bkti_the_nextest_listing_drops_the_binary_id_and_keeps_the_name() {
    let named = bkci_nextest_listing(ZBKTI_NEXTEST).expect("the captured shape reads");

    assert_eq!(
        named,
        vec![
            "vofc_registry::tests::vofc_a_known_prefix_finds_its_cipher",
            "vofe_emplace::tests::vofe_is_hook_file",
        ]
    );
}

#[test]
fn bkti_an_unsurveyed_nextest_line_refuses_rather_than_being_skipped() {
    let grievance = bkci_nextest_listing("vof a_hurdle and something more\n")
        .expect_err("an unsurveyed shape refuses");

    assert!(
        grievance.contains("has not surveyed"),
        "the refusal names the membrane's own limit: {}",
        grievance
    );
}

#[test]
fn bkti_the_harness_membrane_refuses_the_nextest_shape_and_the_reverse() {
    // NEITHER MEMBRANE IS A GENERAL PARSER, and this is what says so: each
    // refuses the other's captured shape rather than making some partial sense
    // of it, which is the failure a single lenient parse would produce.
    assert!(bkci_harness_listing(ZBKTI_NEXTEST).is_err());
    assert!(bkci_nextest_listing(ZBKTI_HARNESS).is_err());
}

#[test]
fn bkti_a_pattern_matches_where_it_appears_and_the_order_is_the_runners() {
    let held = zbkti_held();

    let narrowing = bkci_choose(&held, &["json".to_string()]).expect("the pattern compiles");

    assert_eq!(narrowing.chosen, vec!["bktj_json::alpha", "bktj_json::beta"]);
    assert_eq!(narrowing.held.len(), 4);
    assert_eq!(narrowing.bkci_verdict(), "ran 2 of 4");
}

#[test]
fn bkti_a_hurdle_two_patterns_both_match_is_chosen_once() {
    let held = zbkti_held();

    let narrowing = bkci_choose(&held, &["json".to_string(), "alpha".to_string()])
        .expect("both patterns compile");

    // Three rather than four: `bktj_json::alpha` answers to both patterns and
    // is one hurdle either way. A selection is a SUBSET of the listing.
    assert_eq!(
        narrowing.chosen,
        vec!["bktj_json::alpha", "bktj_json::beta", "bktl_leash::alpha"]
    );
}

#[test]
fn bkti_the_dialect_is_the_rust_one_taken_as_is() {
    let held = zbkti_held();

    // An anchor, an alternation and a character class — spellings a substring
    // filter would not honour, proving the pattern is compiled rather than
    // compared.
    let narrowing = bkci_choose(&held, &[r"::(alpha|gamma)$".to_string()]).expect("it compiles");

    assert_eq!(narrowing.chosen, vec!["bktj_json::alpha", "bktl_leash::alpha"]);
}

#[test]
fn bkti_a_pattern_that_does_not_compile_refuses_rather_than_reading_as_a_literal() {
    let held = zbkti_held();

    let grievance = bkci_choose(&held, &["a(".to_string()]).expect_err("it does not compile");

    assert!(
        grievance.contains("taken as is"),
        "the refusal names the ruling it stands on: {}",
        grievance
    );
}

#[test]
fn bkti_a_pattern_matching_nothing_chooses_nothing_rather_than_everything() {
    let held = zbkti_held();

    // THE FAILURE THIS GUARDS is a matcher whose empty answer is read as "no
    // narrowing asked for" one layer up, which would run the suite whole under
    // a caller who asked for two hurdles.
    let narrowing = bkci_choose(&held, &["nothing_by_that_name".to_string()]).expect("it compiles");

    assert!(narrowing.chosen.is_empty());
    assert_eq!(narrowing.held.len(), 4);
}

#[test]
fn bkti_each_runner_spells_the_selection_its_own_exact_way() {
    let chosen = vec!["alpha".to_string(), "beta".to_string()];

    let harness = bkci_selection(bkcm_mush::BKCM_RUNNER_CARGO, &chosen).expect("cargo spells it");
    let harness: Vec<String> = harness.iter().map(|word| word.to_string_lossy().into_owned()).collect();
    assert_eq!(harness, vec!["--", "--exact", "alpha", "beta"]);

    let nextest = bkci_selection(bkcm_mush::BKCM_RUNNER_NEXTEST, &chosen).expect("nextest spells it");
    let nextest: Vec<String> = nextest.iter().map(|word| word.to_string_lossy().into_owned()).collect();
    assert_eq!(nextest, vec!["-E", "test(=alpha) + test(=beta)"]);
}

#[test]
fn bkti_a_runner_outside_the_declared_pair_refuses_to_spell_a_selection() {
    assert!(bkci_selection("bknre_invented", &["alpha".to_string()]).is_err());
}

/// The seat every composition below is joined from — a collar's manifest is a
/// path reference joined from the repository root, so the root has to be
/// something, and nothing here reads it off a disk.
const ZBKTI_SEAT: &str = "/seat";

/// A suite collar under nextest, standing for a collar the lineup door must
/// list — the same shape `bkcm_mush`'s own composition hurdles compose,
/// reproduced here rather than shared across modules since neither owns the
/// other's fixture.
const ZBKTI_NEXTEST_COLLAR: &str = "\
BKRR_COLLAR=\"suite-lure\"
BKRR_KIND=\"bknre_suite\"
BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"
BKRR_TARGET=\"bknre_manifest\"
BKRR_ROOTS=\"Tools/lure/src\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_reader\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RUNNER=\"bknre_nextest\"
BKRR_TONGUE=\"bknre_nextest\"
";

/// Compose a collar without touching a disk.
fn zbkti_collar(text: &str) -> bkcr_Collar {
    bkcr_Collar {
        name: "suite-lure".to_string(),
        instance: PathBuf::from("suite-lure"),
        regime: bklrc_admit(text, "composed").expect("the collar stands inside the subset"),
    }
}

/// THE EMPTY-SUITE FLAG STAYS ON THE RUN AND LEAVES THE LISTING. `nextest run`
/// accepts the operator's fail-on-empty demand (`bkcm_mush::BKCM_NO_TESTS_FLAG`);
/// `nextest list` refuses it outright as an unexpected argument, so a listing
/// composed from the run's own argument vector must not carry it across.
///
/// THE PAIR IS THE POINT: a repair that dropped the flag from BOTH compositions
/// would satisfy the listing assertion alone while quietly undoing the
/// operator's ruling that an empty suite is red — so the run is asserted to
/// still carry it in the same breath the listing is asserted not to.
#[test]
fn bkti_the_listing_drops_the_run_s_empty_suite_flag_and_the_run_keeps_it() {
    let collar = zbkti_collar(ZBKTI_NEXTEST_COLLAR);

    let run = bkcm_mush::bkcm_suite(Path::new(ZBKTI_SEAT), &collar).expect("the collar composes a run");
    assert!(
        run.rest.iter().any(|argument| argument == bkcm_mush::BKCM_NO_TESTS_FLAG),
        "the run carries the operator's fail-on-empty demand: {}",
        run.bkcm_spelling()
    );

    let listing = bkci_listing(Path::new(ZBKTI_SEAT), &collar).expect("the collar composes a listing");
    assert!(
        listing.rest.iter().all(|argument| argument != bkcm_mush::BKCM_NO_TESTS_FLAG),
        "nextest's list subcommand refuses this flag as an unexpected argument, so the listing \
         must not carry it across from the run: {:?}",
        listing.rest
    );
}

/// A listing to match against, standing for a runner's own answer.
fn zbkti_held() -> Vec<String> {
    vec![
        "bktj_json::alpha".to_string(),
        "bktj_json::beta".to_string(),
        "bktl_leash::alpha".to_string(),
        "bktl_leash::delta".to_string(),
    ]
}

// eof
