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

//! The voice's hurdles.
//!
//! They read the two things a voice can get wrong without anyone noticing: what
//! a recognizer takes off a runner's line, and what the record's curation makes
//! of a line on its way to each member. Both are pure functions of text, so they
//! are driven over CAPTURED SPECIMENS rather than over a live runner — a hurdle
//! that spawned cargo to check a substring would prove the same thing far more
//! slowly and would go red for reasons that have nothing to do with the reader.
//!
//! THE DIGEST IS CHECKED AGAINST THE STANDARD'S OWN VECTORS and never against a
//! second implementation here. A hash checked against itself is checked against
//! nothing; these are the published answers, so a transcription slip in the
//! constants has somewhere to show up.

use super::bkcv_voice::{
    bkcv_case, zbkcv_normalized, zbkcv_sha256, zbkcv_uncolored, BKCV_TONGUE_HARNESS,
    BKCV_TONGUE_NEXTEST,
};

/// A green nextest case line, verbatim from the capture this membrane was
/// written against.
const ZBKTV_NEXTEST_PASS: &str =
    "        PASS [   0.003s] ( 1/24) vof vofc_registry::tests::vofc_prefix_matching";

/// The same shape reporting a failure.
const ZBKTV_NEXTEST_FAIL: &str =
    "        FAIL [   0.012s] (13/24) vof vofe_emplace::tests::test_emplace_subset_parcel";

/// Nextest's summary, which is NOT a case and must not read as one.
const ZBKTV_NEXTEST_SUMMARY: &str = "     Summary [   0.029s] 24 tests run: 24 passed, 0 skipped";

/// A green harness case line, verbatim from the capture.
const ZBKTV_HARNESS_PASS: &str = "test bujd_the_die_path_speaks_with_no_transcript_standing ... ok";

/// The same shape reporting a failure.
const ZBKTV_HARNESS_FAIL: &str = "test bujw_a_bumped_pin_reaches_the_compiler_that_answers ... FAILED";

/// The harness summary, which wears the SAME FIRST WORD as a case line and is
/// the whole reason that reader parts them by the separator.
const ZBKTV_HARNESS_SUMMARY: &str =
    "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.62s";

#[test]
fn bktv_the_nextest_reader_takes_a_case_and_leaves_the_summary() {
    let passed = bkcv_case(BKCV_TONGUE_NEXTEST, ZBKTV_NEXTEST_PASS)
        .expect("the tongue is one the kennel recognizes")
        .expect("a PASS line reports a case");
    assert!(passed.passed, "a PASS line reports a passing case");
    assert_eq!(
        passed.name, "vof vofc_registry::tests::vofc_prefix_matching",
        "the binary id and the case name are rejoined as the case's spelling"
    );

    let failed = bkcv_case(BKCV_TONGUE_NEXTEST, ZBKTV_NEXTEST_FAIL)
        .expect("the tongue is one the kennel recognizes")
        .expect("a FAIL line reports a case");
    assert!(!failed.passed, "a FAIL line reports a failing case");

    // THE SUMMARY IS NOT A CASE. The verdict is the exit code, and a summary
    // counted as a case would put one phantom case in every tally.
    assert_eq!(
        bkcv_case(BKCV_TONGUE_NEXTEST, ZBKTV_NEXTEST_SUMMARY).expect("recognized tongue"),
        None,
        "the summary is not a case: {}",
        ZBKTV_NEXTEST_SUMMARY
    );
}

#[test]
fn bktv_the_harness_reader_parts_a_case_from_the_summary_by_the_separator() {
    let passed = bkcv_case(BKCV_TONGUE_HARNESS, ZBKTV_HARNESS_PASS)
        .expect("recognized tongue")
        .expect("an ok line reports a case");
    assert!(passed.passed);
    assert_eq!(passed.name, "bujd_the_die_path_speaks_with_no_transcript_standing");

    let failed = bkcv_case(BKCV_TONGUE_HARNESS, ZBKTV_HARNESS_FAIL)
        .expect("recognized tongue")
        .expect("a FAILED line reports a case");
    assert!(!failed.passed);

    // THE CONTROL THAT MATTERS FOR THIS RUNNER: the summary begins with the very
    // word a case line begins with, so a reader keying on `test ` alone would
    // take it. It carries no ` ... `, and that is what excludes it.
    assert_eq!(
        bkcv_case(BKCV_TONGUE_HARNESS, ZBKTV_HARNESS_SUMMARY).expect("recognized tongue"),
        None,
        "the summary wears the same first word and is still not a case: {}",
        ZBKTV_HARNESS_SUMMARY
    );

    // An ignored case is outside the surveyed signature: the kennel has no skip,
    // so a case the runner declined to run is not a case that ran.
    assert_eq!(
        bkcv_case(BKCV_TONGUE_HARNESS, "test some::case ... ignored").expect("recognized tongue"),
        None,
        "an ignored case is not counted as having run"
    );
}

#[test]
fn bktv_an_undeclared_tongue_refuses_rather_than_falling_back() {
    // A door that fell back on a default would render a suite's cases under the
    // wrong reader and report a green run as having no cases at all.
    assert!(
        bkcv_case("bknre_esperanto", ZBKTV_HARNESS_PASS).is_err(),
        "an undeclared tongue refuses"
    );
}

#[test]
fn bktv_each_reader_declines_the_other_runners_lines() {
    // THE CROSS CONTROL. Each reader is shown the other's case line, and must
    // take nothing from it. Without this the two readers could both be matching
    // on something loose and the per-runner assertions above would still pass.
    assert_eq!(
        bkcv_case(BKCV_TONGUE_HARNESS, ZBKTV_NEXTEST_PASS).expect("recognized tongue"),
        None,
        "the harness reader takes nothing from a nextest line"
    );
    assert_eq!(
        bkcv_case(BKCV_TONGUE_NEXTEST, ZBKTV_HARNESS_PASS).expect("recognized tongue"),
        None,
        "the nextest reader takes nothing from a harness line"
    );
}

#[test]
fn bktv_a_case_the_runner_repeats_is_one_case_under_nextest_and_two_names_are_two_under_the_harness() {
    use super::bkcv_voice::zbkcv_recapitulates;

    // Nextest prints a failing case twice — inline, then again beneath its
    // summary — and qualifies every name with its binary id, so a repeat of a
    // name is a repeat of the case.
    assert!(
        zbkcv_recapitulates(BKCV_TONGUE_NEXTEST),
        "nextest recapitulates its failures and its names carry the binary id"
    );

    // THE CONTROL, AND THE ONE THAT MATTERS. The standard harness prints a BARE
    // case name, so two test binaries of one manifest may each hold a case of
    // the same name; counting distinct names there would report two real cases
    // as one. This assertion is what keeps a future tidy-up from making the
    // dedup uniform across both readers.
    assert!(
        !zbkcv_recapitulates(BKCV_TONGUE_HARNESS),
        "the harness prints bare case names, so two binaries may honestly report the same name \
         and a name-keyed dedup would undercount them"
    );
}

#[test]
fn bktv_the_normalized_member_removes_one_run_to_run_variable_apiece() {
    // Each assertion below removes exactly one variable, and the member is
    // diffable in proportion to how completely they are applied (BUr_amx).

    // Colour.
    assert_eq!(
        zbkcv_uncolored("\u{1b}[32mgreen\u{1b}[0m"),
        "green",
        "a terminal control sequence is stripped"
    );

    // The ephemeral path.
    assert_eq!(
        zbkcv_normalized("built under /tmp/temp-1234 today", Some("/tmp/temp-1234")),
        vec!["built under BURD_EPHEMERAL_DIR today".to_string()],
        "the ephemeral directory is replaced by the literal"
    );

    // A declared-volatile line, dropped whole.
    assert!(
        zbkcv_normalized("elapsed 3.4s VOLATILE", None).is_empty(),
        "a line carrying the marker is dropped whole"
    );

    // Cursor motion, split into the states it redrew through.
    assert_eq!(
        zbkcv_normalized("10%\r55%\r100%", None),
        vec!["10%".to_string(), "55%".to_string(), "100%".to_string()],
        "a redraw lands as its successive states rather than one unreadable line"
    );

    // Emptiness.
    assert!(zbkcv_normalized("", None).is_empty(), "empty lines are dropped");

    // THE CONTROL: an ordinary line survives all five rules unchanged. Without
    // it every assertion above is satisfied by a curator that drops everything.
    assert_eq!(
        zbkcv_normalized("    Finished `test` profile in 0.03s", None),
        vec!["    Finished `test` profile in 0.03s".to_string()],
        "an ordinary line passes through, indentation and all"
    );
}

#[test]
fn bktv_the_digest_answers_the_standards_own_vectors() {
    // FIPS 180-4's published answers. Checked against the standard rather than
    // against a second implementation here, so a transcription slip in the round
    // constants has somewhere to show up.
    assert_eq!(
        zbkcv_sha256(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "the empty input"
    );
    assert_eq!(
        zbkcv_sha256(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        "the one-block vector"
    );
    assert_eq!(
        zbkcv_sha256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
        "the two-block vector, which is what exercises the message schedule past the first block"
    );
}

// eof
