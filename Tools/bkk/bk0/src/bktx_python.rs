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

//! Hurdles over the python family: the reading of a collar, and every refusal
//! the settling sitting listed.
//!
//! EVERY ONE OF THEM IS HERMETIC. The reading is over three files of text and a
//! declaration, so a lure holding them proves the whole of it without a network
//! and without a venv. What is NOT proven here is the converge, which is uv's
//! act and the heel pace's hurdle; nothing below spawns uv or reaches a station.
//!
//! EACH REFUSAL IS ONE CONFORMING PROJECT WITH ONE THING WRONG WITH IT, and the
//! conforming project is asserted clean first. A hurdle whose project was wrong
//! in two ways would go green on the wrong finding and stay green after the
//! finding it was written for stopped being raised.

use crate::bkca_whereabouts::bkca_Geography;
use crate::bkct_tattoo::{bkct_proclamation_python, BKCT_LANGUAGE_PYTHON};
use crate::bkcx_python::{bkcx_resolve, BKCX_LOCK_FILE};
use crate::bktu_lure::{bktu_Bent, bktu_Lure};

/// The collar every hurdle below composes, and the project it points at.
const ZBKTX_COLLAR: &str = "suite-lure";
const ZBKTX_AT: &str = "project";

/// The uv kibble a lure lays down so that a `required-version` has something to
/// be weighed against.
///
/// FABRICATED, AND DELIBERATELY NOT THE ESTATE'S PIN. A hurdle carrying the real
/// version would be a second home for it, left behind saying something that used
/// to be true at the next bump; what these hurdles prove is what the READER does
/// with a declaration, never what uv the estate stands on.
const ZBKTX_UV_VERSION: &str = "0.11.30";
const ZBKTX_UV_SEAL: &str =
    "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

/// A lure carrying one uv project, one python collar over it, and the uv kibble.
///
/// THE LURE IS NAMED BY ITS HURDLE, and the parameter is not decoration: the
/// harness runs these in parallel, so a shared name is a shared directory and
/// every hurdle then races its neighbours through one git init. That is not
/// hypothetical — it was driven, and all ten went red together on a failure
/// belonging to none of them.
fn zbktx_seated(named: &str, bent: &bktu_Bent) -> bktu_Lure {
    let lure = bktu_Lure::bktu_compose(&format!("python-{}", named));

    lure.bktu_project(ZBKTX_AT, ZBKTX_COLLAR, bent);

    lure.bktu_write(
        "bki_uv/bkrk.env",
        &format!(
            "BKRK_KIBBLE=\"bki_uv\"\n\
             BKRK_KIND=\"bknre_prebuilt\"\n\
             BKRK_PROGRAM=\"uv\"\n\
             BKRK_VERSION=\"{}\"\n\
             BKRK_SEAL=\"{}\"\n\
             BKRK_BYNAME=\"uv\"\n\
             BKRK_LARDER=\"https://example.invalid/releases\"\n\
             BKRK_BORDEREAU=\"dist-manifest.json\"\n",
            ZBKTX_UV_VERSION, ZBKTX_UV_SEAL
        ),
    );

    lure
}

/// Every finding the reading gathers against the seated collar.
fn zbktx_findings(lure: &bktu_Lure) -> Vec<String> {
    bkcx_resolve(lure.bktu_root(), ZBKTX_COLLAR)
        .expect("the seated collar resolves")
        .findings
}

/// Whether any finding names this text, so a hurdle asserts on the condition it
/// was written for rather than on a count.
fn zbktx_names(findings: &[String], text: &str) -> bool {
    findings.iter().any(|finding| finding.contains(text))
}

#[test]
fn bktx_a_whole_python_collar_over_a_composed_project_carries_no_finding() {
    let lure = zbktx_seated("whole", &bktu_Bent::default());
    let findings = zbktx_findings(&lure);

    assert!(
        findings.is_empty(),
        "a conforming python collar reported: {:?}",
        findings
    );
}

#[test]
fn bktx_a_conforming_collar_proclaims_with_the_python_language_value() {
    let lure = zbktx_seated("proclaims", &bktu_Bent::default());
    let resolved = bkcx_resolve(lure.bktu_root(), ZBKTX_COLLAR).expect("the collar resolves");

    let said = bkct_proclamation_python(
        &resolved.collar,
        "bknre_manifest",
        &bkca_Geography::Undispatched,
    );

    assert!(
        said.contains(&format!("BKRC_LANGUAGE=\"{}\"", BKCT_LANGUAGE_PYTHON)),
        "the proclamation named no python language value: {}",
        said
    );

    // THE PROJECT POINTER SURFACES AS THE MANIFEST, which is the whole of what
    // makes the vocabulary tenant-blind: a consumer asks what declares this
    // launchable's dependencies and is never told which language answered.
    assert!(
        said.contains("BKRC_MANIFEST=\"project/pyproject.toml\""),
        "the proclamation carried no manifest row: {}",
        said
    );

    // NO FAMILY SPELLING REACHES THE ANSWER.
    assert!(
        !said.contains("BKRP_"),
        "a family spelling reached the proclamation: {}",
        said
    );
}

#[test]
fn bktx_an_absent_project_file_refuses() {
    let lure = zbktx_seated("lockless", &bktu_Bent {
        lockless: true,
        ..Default::default()
    });
    let findings = zbktx_findings(&lure);

    assert!(
        zbktx_names(&findings, BKCX_LOCK_FILE),
        "a project missing its lock reported: {:?}",
        findings
    );
}

#[test]
fn bktx_a_pin_disagreeing_with_the_manifests_requirement_refuses() {
    let lure = zbktx_seated("pin-below", &bktu_Bent {
        pin: "3.11.9".to_string(),
        requires: ">=3.12".to_string(),
        ..Default::default()
    });
    let findings = zbktx_findings(&lure);

    assert!(
        zbktx_names(&findings, "falls below the project's own floor"),
        "a pin below the manifest's floor reported: {:?}",
        findings
    );
}

#[test]
fn bktx_a_bare_minor_version_floor_refuses() {
    let lure = zbktx_seated("pin-floor", &bktu_Bent {
        pin: "3.12".to_string(),
        ..Default::default()
    });
    let findings = zbktx_findings(&lure);

    assert!(
        zbktx_names(&findings, "not an exact patch"),
        "a bare minor pin reported: {:?}",
        findings
    );
}

#[test]
fn bktx_a_non_default_index_declared_without_explicit_refuses() {
    let lure = zbktx_seated("implicit-index", &bktu_Bent {
        uv: "\n[[tool.uv.index]]\nname = \"house\"\nurl = \"https://example.invalid/simple\"\n\
             \n[tool.uv.sources]\nheld = { index = \"house\" }\n"
            .to_string(),
        ..Default::default()
    });
    let findings = zbktx_findings(&lure);

    assert!(
        zbktx_names(&findings, "non-default and not explicit"),
        "an implicit non-default index reported: {:?}",
        findings
    );
}

#[test]
fn bktx_an_index_strategy_refuses() {
    let lure = zbktx_seated("strategy", &bktu_Bent {
        uv: "\n[tool.uv]\nindex-strategy = \"unsafe-best-match\"\n".to_string(),
        ..Default::default()
    });
    let findings = zbktx_findings(&lure);

    assert!(
        zbktx_names(&findings, "index-strategy"),
        "a declared index strategy reported: {:?}",
        findings
    );
}

#[test]
fn bktx_a_kennel_owned_knob_in_the_uv_table_refuses() {
    let lure = zbktx_seated("knob", &bktu_Bent {
        uv: "\n[tool.uv]\npython-downloads = \"automatic\"\n".to_string(),
        ..Default::default()
    });
    let findings = zbktx_findings(&lure);

    assert!(
        zbktx_names(&findings, "the kennel states in the environment"),
        "a kennel-owned knob reported: {:?}",
        findings
    );
}

#[test]
fn bktx_a_required_version_excluding_the_kibbles_pin_refuses() {
    let lure = zbktx_seated("required", &bktu_Bent {
        uv: "\n[tool.uv]\nrequired-version = \">=0.12\"\n".to_string(),
        ..Default::default()
    });
    let findings = zbktx_findings(&lure);

    assert!(
        zbktx_names(&findings, "cannot be converged here"),
        "a required-version beyond the kibble's pin reported: {:?}",
        findings
    );
}

#[test]
fn bktx_a_declared_index_no_source_draws_from_refuses() {
    let lure = zbktx_seated("undrawn-index", &bktu_Bent {
        uv: "\n[[tool.uv.index]]\nname = \"house\"\nurl = \"https://example.invalid/simple\"\n\
             explicit = true\n"
            .to_string(),
        ..Default::default()
    });
    let findings = zbktx_findings(&lure);

    assert!(
        zbktx_names(&findings, "no [tool.uv.sources] entry draws from it"),
        "an index nothing draws from reported: {:?}",
        findings
    );
}

// eof

/// The seat every composition hurdle below poses, over roots that stand nowhere.
///
/// POSED AND NEVER READ FROM THE ENVIRONMENT, which is the whole reason the
/// composition was split at its edge: the reading consults process-wide state
/// that every parallel neighbour also reads, and a hurdle writing it would be
/// deciding their answers.
fn zbktx_posed_seat(lure: &bktu_Lure) -> crate::bkcx_python::bkcx_Seat {
    let collar = bkcx_resolve(lure.bktu_root(), ZBKTX_COLLAR)
        .expect("the seated collar resolves")
        .collar;

    crate::bkcx_python::bkcx_seat_at(
        std::path::Path::new("/posed/derived"),
        std::path::Path::new("/posed/tackroom"),
        &collar,
    )
}

#[test]
fn bktx_the_environment_stands_under_the_loosebox_and_never_in_the_project() {
    let lure = zbktx_seated("seat-environment", &bktu_Bent::default());
    let seat = zbktx_posed_seat(&lure);

    let said = seat.environment.to_string_lossy().into_owned();

    assert!(
        said.starts_with("/posed/derived"),
        "the environment stands at {} rather than under the derived root",
        said
    );

    // THE KENNEL'S QUARTER STANDS BETWEEN THE ROOT AND THE PROJECT, which is what
    // says whose the directory is in a root the kennel shares with other tenants.
    assert!(
        said.contains("/bkk/"),
        "the environment {} names no kennel quarter",
        said
    );

    // KEYED ON THE PROJECT'S PATH AND NOT ON THE COLLAR'S NAME, which is the
    // ruling that lets several collars share one project without each getting a
    // full copy of its environment.
    assert!(
        said.ends_with(ZBKTX_AT),
        "the environment {} is not keyed on the project's own path",
        said
    );

    assert!(
        !said.contains(ZBKTX_COLLAR),
        "the environment {} is keyed on the collar, so two collars over one project would build \
         two environments",
        said
    );
}

#[test]
fn bktx_the_stores_stand_in_the_tackroom_quarter_and_the_executables_do_not() {
    let lure = zbktx_seated("seat-stores", &bktu_Bent::default());
    let seat = zbktx_posed_seat(&lure);

    for (label, path) in [("store", &seat.store), ("cache", &seat.cache)] {
        let said = path.to_string_lossy().into_owned();
        assert!(
            said.starts_with("/posed/tackroom/bkk/"),
            "the {} stands at {} rather than in the kennel's tackroom quarter",
            label,
            said
        );
    }

    // THE EXECUTABLE DIRECTORY IS A GUARD AND STANDS WITH THE CHECKOUT, never in
    // the shared store: what it exists to displace is uv's default, a directory
    // on the operator's own PATH in the operator's own home, and a guard placed
    // in the tackroom would be shared by every checkout on the station.
    let executables = seat.executables.to_string_lossy().into_owned();
    assert!(
        executables.starts_with("/posed/derived"),
        "the executable guard stands at {} rather than under the derived root",
        executables
    );
}

#[test]
fn bktx_the_stated_roster_names_every_root_the_seat_carries() {
    let lure = zbktx_seated("stated-roots", &bktu_Bent::default());
    let seat = zbktx_posed_seat(&lure);

    let stated =
        crate::bkcx_python::bkcx_stated(&seat, crate::bkcx_python::bkcx_Posture::Sealed);

    let held = |name: &str| -> String {
        stated
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| value.to_string_lossy().into_owned())
            .unwrap_or_else(|| panic!("the roster states no {}", name))
    };

    assert_eq!(held("UV_PROJECT_ENVIRONMENT"), seat.environment.to_string_lossy());
    assert_eq!(held("UV_PYTHON_INSTALL_DIR"), seat.store.to_string_lossy());
    assert_eq!(held("UV_CACHE_DIR"), seat.cache.to_string_lossy());
    assert_eq!(held("UV_PYTHON_BIN_DIR"), seat.executables.to_string_lossy());

    // PYTHON'S OWN NAME, AND THE ONLY ONE ON THIS ROSTER THAT IS. It stands here
    // because the roster is what reaches the child's child: what the kennel
    // spawns is uv, and what uv spawns is an interpreter that would otherwise
    // write a `__pycache__` beside every module it imports — inside the source
    // tree, which was observed leaving a lure dirty enough that the next drive
    // refused the repository it had just run over.
    assert_eq!(held("PYTHONPYCACHEPREFIX"), seat.bytecode.to_string_lossy());

    // THE PREFERENCE AND THE SUPPRESSION ARE STATED ON EVERY POSTURE, the second
    // being the guard that must not depend on a door remembering a flag.
    assert_eq!(held("UV_PYTHON_PREFERENCE"), "only-managed");
    assert_eq!(held("UV_PYTHON_INSTALL_BIN"), "0");
}

#[test]
fn bktx_only_the_fetching_posture_lifts_downloads_and_only_the_sealed_one_is_offline() {
    let lure = zbktx_seated("postures", &bktu_Bent::default());
    let seat = zbktx_posed_seat(&lure);

    let read = |posture: crate::bkcx_python::bkcx_Posture, name: &str| -> String {
        crate::bkcx_python::bkcx_stated(&seat, posture)
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| value.to_string_lossy().into_owned())
            .unwrap_or_else(|| panic!("the roster states no {}", name))
    };

    use crate::bkcx_python::bkcx_Posture;

    // THE ROUTINE-DOWNLOAD RULING, READ OFF THE ROSTER. Only the converge's
    // install step may fetch an interpreter, and it is the only posture that
    // lifts the download knob.
    assert_eq!(read(bkcx_Posture::Sealed, "UV_PYTHON_DOWNLOADS"), "never");
    assert_eq!(read(bkcx_Posture::Reaching, "UV_PYTHON_DOWNLOADS"), "never");
    assert_eq!(read(bkcx_Posture::Fetching, "UV_PYTHON_DOWNLOADS"), "manual");

    // EVERY POSTURE STATES THE OFFLINE KNOB, and the reaching ones state it
    // FALSE rather than leaving it off: uv refuses an empty boolean outright, so
    // clearing one by stating nothing is not available, and an unstated knob
    // would be inherited from whatever door launched this one.
    assert_eq!(read(bkcx_Posture::Sealed, "UV_OFFLINE"), "1");
    assert_eq!(read(bkcx_Posture::Reaching, "UV_OFFLINE"), "0");
    assert_eq!(read(bkcx_Posture::Fetching, "UV_OFFLINE"), "0");
}

#[test]
fn bktx_no_stated_value_is_empty_because_uv_refuses_an_empty_boolean() {
    let lure = zbktx_seated("stated-nonempty", &bktu_Bent::default());
    let seat = zbktx_posed_seat(&lure);

    use crate::bkcx_python::bkcx_Posture;

    // DRIVEN AT THE BENCH AND HELD HERE. uv answers an empty boolean with
    // `expected a boolish value` and refuses the whole invocation, so a roster
    // that cleared a knob by stating nothing would kill every converge under it
    // — and would do it in a sentence naming uv rather than this kennel.
    for posture in [
        bkcx_Posture::Sealed,
        bkcx_Posture::Reaching,
        bkcx_Posture::Fetching,
    ] {
        for (name, value) in crate::bkcx_python::bkcx_stated(&seat, posture) {
            assert!(
                !value.is_empty(),
                "{} is stated empty under {:?}, and uv refuses an empty value",
                name,
                posture
            );
        }
    }
}

#[test]
fn bktx_an_absent_environment_refuses_naming_the_converge() {
    let lure = zbktx_seated("verify-absent", &bktu_Bent::default());
    let collar = bkcx_resolve(lure.bktu_root(), ZBKTX_COLLAR)
        .expect("the seated collar resolves")
        .collar;

    // THE SEAT IS POSED AT A DIRECTORY THAT STANDS AND HOLDS NOTHING, so the
    // refusal is about an absent environment rather than an absent root.
    let seat = crate::bkcx_python::bkcx_seat_at(
        lure.bktu_loosebox(),
        std::path::Path::new("/posed/tackroom"),
        &collar,
    );

    let err = crate::bkcx_python::bkcx_verify_at(lure.bktu_root(), &collar, &seat)
        .expect_err("an absent environment refuses");

    assert!(
        err.contains("heel"),
        "the refusal does not name the one door that writes an environment: {}",
        err
    );

    // AND IT SAYS WHY THERE IS NOTHING TO FIND BESIDE THE PROJECT, which is the
    // half a reader coming from a `.venv` habit actually needs.
    assert!(
        err.contains("outside the source tree"),
        "the refusal does not say where an environment stands: {}",
        err
    );
}

////////////////////////////////////////////////////////////////////////////////
// The launch — hermetic
//
// The reading half of mush's python arm: what the listing membrane yields, what
// a pattern narrows to, what an empty match refuses, and what a seat carrying no
// environment says. All of it over text and a posed seat; nothing here spawns uv
// or reaches a station, the converge and the drive being the real-tier hurdles'
// half.

/// pytest's own collect-only answer, TRANSCRIBED from the captured drive at seat
/// ace9b08ec84a229f4c57372371c514705c6ad789 rather than composed to suit the
/// parser. A specimen written to please a reader proves the reader against
/// itself.
const ZBKTX_COLLECTED: &str = "\
tests/test_alpha.py::test_one
tests/test_alpha.py::test_two
tests/test_beta.py::test_three
tests/test_beta.py::test_red

4 tests collected in 0.00s
";

#[test]
fn bktx_the_listing_parser_yields_the_captured_node_ids_and_nothing_else() {
    let named = crate::bkci_pipeline::bkci_pytest_listing(ZBKTX_COLLECTED)
        .expect("the captured listing reads");

    // THE WHOLE SET AND IN ORDER, which is the assertion that matters: a parser
    // that dropped the tally would still be wrong if it also dropped a case, and
    // a count alone could not tell the two apart.
    assert_eq!(
        named,
        vec![
            "tests/test_alpha.py::test_one".to_string(),
            "tests/test_alpha.py::test_two".to_string(),
            "tests/test_beta.py::test_three".to_string(),
            "tests/test_beta.py::test_red".to_string(),
        ],
        "the listing membrane did not yield exactly the node ids pytest named"
    );
}

#[test]
fn bktx_the_listing_parser_admits_all_three_tally_spellings() {
    // The count is singular at one case, and the word `no` stands where a
    // numeral would at none. Each was captured; none is supposed.
    let one = crate::bkci_pipeline::bkci_pytest_listing(
        "tests/test_beta.py::test_red\n\n1 test collected in 0.00s\n",
    )
    .expect("a singular count line reads");

    assert_eq!(one, vec!["tests/test_beta.py::test_red".to_string()]);

    let none = crate::bkci_pipeline::bkci_pytest_listing("\nno tests collected in 0.00s\n")
        .expect("an empty listing reads");

    // AN EMPTY LISTING IS A READING AND NOT A REFUSAL. What a project holding no
    // case costs is the hollow gate at the run, which is a verdict about a
    // course; a parser that refused here would turn that verdict into a parse
    // error and send its reader to the membrane instead of to the suite.
    assert!(none.is_empty(), "an empty listing yielded {:?}", none);
}

#[test]
fn bktx_the_listing_parser_refuses_an_unsurveyed_line() {
    let err = crate::bkci_pipeline::bkci_pytest_listing(
        "tests/test_alpha.py::test_one\nERROR: not found: tests/test_alpha.py::test_absent\n",
    )
    .expect_err("a line outside the surveyed shapes refuses");

    assert!(
        err.contains("has not surveyed"),
        "the refusal does not name the membrane's own bound: {}",
        err
    );
}

#[test]
fn bktx_the_tongue_reads_a_case_off_the_verbose_line() {
    let green = crate::bkcv_voice::bkcv_case(
        crate::bkcv_voice::BKCV_TONGUE_PYTEST,
        "tests/test_alpha.py::test_one PASSED                                     [ 25%]",
    )
    .expect("the tongue is declared")
    .expect("a verbose case line reports a case");

    assert_eq!(green.name, "tests/test_alpha.py::test_one");
    assert!(green.passed, "a PASSED case did not read as passed");

    let red = crate::bkcv_voice::bkcv_case(
        crate::bkcv_voice::BKCV_TONGUE_PYTEST,
        "tests/test_beta.py::test_red FAILED                                      [100%]",
    )
    .expect("the tongue is declared")
    .expect("a verbose case line reports a case");

    assert_eq!(red.name, "tests/test_beta.py::test_red");
    assert!(!red.passed, "a FAILED case did not read as failed");
}

#[test]
fn bktx_the_tongue_steps_over_the_short_summary_that_repeats_a_failure() {
    // THE LOAD-BEARING ONE. pytest prints its short summary for every failing
    // case, so a reader that took both orders would count each failure twice and
    // report a tally its own runner disagrees with.
    let said = crate::bkcv_voice::bkcv_case(
        crate::bkcv_voice::BKCV_TONGUE_PYTEST,
        "FAILED tests/test_beta.py::test_red - assert 1 == 2",
    )
    .expect("the tongue is declared");

    assert!(
        said.is_none(),
        "the short summary was read as a second report of the same case: {:?}",
        said
    );
}

#[test]
fn bktx_the_tongue_steps_over_the_outcomes_outside_the_signature() {
    for line in [
        "tests/test_alpha.py::test_one SKIPPED                                    [ 25%]",
        "tests/test_alpha.py::test_one XFAIL                                      [ 25%]",
        "tests/test_alpha.py::test_one XPASS                                      [ 25%]",
        "collecting ... collected 4 items",
        "=========================== short test summary info ============================",
    ] {
        let said = crate::bkcv_voice::bkcv_case(crate::bkcv_voice::BKCV_TONGUE_PYTEST, line)
            .expect("the tongue is declared");

        assert!(said.is_none(), "an unsurveyed line reported a case: {:?}", line);
    }

    // ERROR IS INSIDE THE SIGNATURE AND COUNTS AS A FAILURE — pytest's word for a
    // case whose setup died, which is a case that did not pass. Dropping it would
    // let a suite whose fixtures collapsed report as having run nothing.
    let errored = crate::bkcv_voice::bkcv_case(
        crate::bkcv_voice::BKCV_TONGUE_PYTEST,
        "tests/test_alpha.py::test_one ERROR                                      [ 25%]",
    )
    .expect("the tongue is declared")
    .expect("an ERROR line reports a case");

    assert!(!errored.passed, "an ERROR case did not read as failed");
}

#[test]
fn bktx_a_pattern_narrows_to_the_exact_set_and_the_selection_is_those_node_ids() {
    let held = crate::bkci_pipeline::bkci_pytest_listing(ZBKTX_COLLECTED)
        .expect("the captured listing reads");

    let narrowing = crate::bkci_pipeline::bkci_choose(&held, &["test_beta".to_string()])
        .expect("the pattern compiles");

    assert_eq!(
        narrowing.chosen,
        vec![
            "tests/test_beta.py::test_three".to_string(),
            "tests/test_beta.py::test_red".to_string(),
        ],
        "the pattern did not narrow to the set it names"
    );

    assert_eq!(narrowing.held.len(), 4, "the narrowing lost the whole set");

    let selection = crate::bkci_pipeline::bkci_selection(
        crate::bkcx_python::BKCX_RUNNER_PYTEST,
        &narrowing.chosen,
    )
    .expect("the runner is one the kennel spawns");

    // THE SPELLED SELECTION IS THE NODE IDS THEMSELVES — positional, exact by
    // construction, and carrying no flag: a node id names one case outright, so
    // there is no exactness to ask for and no filter grammar to bypass.
    assert_eq!(
        selection,
        vec![
            std::ffi::OsString::from("tests/test_beta.py::test_three"),
            std::ffi::OsString::from("tests/test_beta.py::test_red"),
        ],
        "the selection was spelled as something other than the node ids"
    );
}

#[test]
fn bktx_a_pattern_matching_nothing_chooses_nothing_so_the_door_may_refuse_before_a_spawn() {
    let held = crate::bkci_pipeline::bkci_pytest_listing(ZBKTX_COLLECTED)
        .expect("the captured listing reads");

    let narrowing = crate::bkci_pipeline::bkci_choose(&held, &["test_absent".to_string()])
        .expect("the pattern compiles");

    // The refusal itself is the door's and is proven over a lure; what stands
    // here is the reading the door refuses ON, which is what makes the refusal
    // possible AHEAD of the spawn rather than after it.
    assert!(
        narrowing.chosen.is_empty(),
        "a pattern naming no case chose {:?}",
        narrowing.chosen
    );
    assert_eq!(narrowing.held.len(), 4, "the refusal could not say of how many");
}

#[test]
fn bktx_a_posed_seat_with_no_marque_refuses_naming_heel() {
    let lure = zbktx_seated("marqueless", &bktu_Bent::default());

    let collar = bkcx_resolve(lure.bktu_root(), ZBKTX_COLLAR)
        .expect("the seated collar resolves")
        .collar;

    // THE SEAT IS POSED RATHER THAN WRITTEN, on the kibble residence's own
    // precedent: the roots reading consults process-wide state every parallel
    // hurdle also reads, so a hurdle posing one by writing would decide its
    // neighbours' answers.
    let seat = crate::bkcx_python::bkcx_seat_at(
        &lure.bktu_loosebox().join("posed"),
        &lure.bktu_root().join("tackroom"),
        &collar,
    );

    let err = crate::bkcx_python::bkcx_verify_at(lure.bktu_root(), &collar, &seat)
        .expect_err("a seat carrying no environment refuses");

    assert!(
        err.contains("no environment stands"),
        "the refusal does not say what is missing: {}",
        err
    );

    // AND IT NAMES THE ONE DOOR THAT WRITES AN ENVIRONMENT. A python environment
    // stands outside the source tree by ruling, so a reader who is not sent to
    // the converge has nothing beside the project to find.
    assert!(
        err.contains("heel"),
        "the refusal does not name the converge: {}",
        err
    );
}

#[test]
fn bktx_the_bytecode_root_stands_under_the_loosebox_and_never_beside_the_sources() {
    let lure = zbktx_seated("bytecode", &bktu_Bent::default());

    let collar = bkcx_resolve(lure.bktu_root(), ZBKTX_COLLAR)
        .expect("the seated collar resolves")
        .collar;

    let loosebox = lure.bktu_loosebox().join("posed");
    let seat = crate::bkcx_python::bkcx_seat_at(
        &loosebox,
        &lure.bktu_root().join("tackroom"),
        &collar,
    );

    assert!(
        seat.bytecode.starts_with(&loosebox),
        "the bytecode root stands outside the derived root: {}",
        seat.bytecode.display()
    );

    // AND NEVER INSIDE THE CHECKOUT, which is the property that matters: bytecode
    // written beside a module is bytecode written into a tracked tree, and the
    // door refuses a repository carrying it.
    assert!(
        !seat.bytecode.starts_with(lure.bktu_root()),
        "the bytecode root stands inside the source tree: {}",
        seat.bytecode.display()
    );
}
