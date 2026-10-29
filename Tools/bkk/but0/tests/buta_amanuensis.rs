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

//! Own-log mode: the dispatch names the log family and writes none of it.
//!
//! Ported from the bash bench's `amanuensis-mode` fixture, six cases, all six
//! carried. The mode is the third of the Log Family Law, said by the
//! configuration word `buuc_ownlog`: the dispatch composes the family's three
//! names, exports them across the exec boundary, records them in the dispatch
//! file, creates no file and tees nothing, so the coordinator writes the record
//! itself.
//!
//! A DISPATCH DECLARES ITS MODE BEFORE ANY WORK RUNS, which is why every case
//! here drives a dispatch of its own rather than reading the one it is running
//! under. The bash cases reached that conclusion first and the port keeps their
//! shape: a sub-dispatch is spawned with the mode's words said, pointed at a probe
//! coordinator that reports the environment it was handed. What changed is where
//! the sub-dispatch's scratch stands — under the hurdle's own seat rather than
//! under a bench temp directory six other fixtures were also writing into — and
//! who counts the log directory afterwards, which is now this process.
//!
//! THE LOGGED CONTROL IS NOT DECORATION AND IS NOT DROPPED. Two of these hurdles
//! assert an ABSENCE — that no member was created — and an absence reads green
//! whether the mode worked or the probe looked in the wrong directory entirely.
//! The control drives the same probe through the same helper with no word and
//! asserts all three members appear. That this pace has now twice caught a
//! hurdle passing for the wrong reason is the argument for keeping it.

#![deny(warnings)]
#![allow(non_camel_case_types)]

use but::buth_hurdle::buth_Bench;

/// The coordinator drives a dispatch rather than sourcing the kit, so it sources
/// no module itself, only its dispatch file.
const BUTA_MODULES: &[&str] = &[];

/// The tokens the outer coordinator reports its scratch under.
const BUTA_LOGS: &str = "buta_logs=";
const BUTA_REPORT: &str = "buta_report=";

/// How many members a logged dispatch's family holds.
const BUTA_FAMILY: usize = 3;

/// Where a drive's sub-dispatch scratch stood.
struct buta_Where {
    logs: String,
    report: String,
}

/// Compose a coordinator that lays down a probe and drives one sub-dispatch
/// under the named configuration words.
///
/// THE DISPATCH IS HANDED WHAT A LAUNCHER HANDS IT: the coordinator, the
/// configuration, the seat and the moorings directory ahead of the tabtarget
/// basename. The words follow a made-up launcher's basename, exactly as a
/// tabtarget's configuration line carries them.
///
/// BOTH COORDINATORS READ THEIR DISPATCH FROM ITS FILE, which is the one road a
/// dispatch value takes below a dispatch: the outer one its seat's, the probe
/// the sub-dispatch's.
fn buta_drive(name: &str, words: &str) -> (buth_Bench, but::buth_hurdle::buth_Said, buta_Where) {
    let configuration = if words.is_empty() {
        "launcher.buja_workbench.sh".to_string()
    } else {
        format!(
            "launcher.buja_workbench.sh {}",
            words, //
        )
    };
    let body = format!(
        // The scratch stands under this seat's own temp directory and is reported
        // before anything can fail, so a hurdle whose sub-dispatch is SUPPOSED to
        // die still learns where to look for what it must not find.
        "source \"${{BURD_BURD_FILE}}\"\n\
         z_logs=\"${{BURD_TEMP_DIR}}/buja-logs\"\n\
         z_report=\"${{BURD_TEMP_DIR}}/buja-report.env\"\n\
         z_probe=\"${{BURD_TEMP_DIR}}/buja-coordinator.sh\"\n\
         mkdir -p \"${{z_logs}}\"\n\
         printf 'buta_logs=%s\\n'   \"${{z_logs}}\"\n\
         printf 'buta_report=%s\\n' \"${{z_report}}\"\n\
         cat > \"${{z_probe}}\" <<BUTA_PROBE\n\
         #!/bin/bash\n\
         source \"\\${{BURD_BURD_FILE}}\"\n\
         {{\n\
         \x20 printf 'buta_last=\\\"%s\\\"\\\\n' \"\\${{BURD_LOG_LAST:-}}\"\n\
         \x20 printf 'buta_same=\\\"%s\\\"\\\\n' \"\\${{BURD_LOG_SAME:-}}\"\n\
         \x20 printf 'buta_hist=\\\"%s\\\"\\\\n' \"\\${{BURD_LOG_HIST:-}}\"\n\
         \x20 printf 'buta_temp=\\\"%s\\\"\\\\n' \"\\${{BURD_TEMP_DIR:-}}\"\n\
         }} > \"${{z_report}}\"\n\
         echo \"buja probe coordinator ran\"\n\
         BUTA_PROBE\n\
         chmod +x \"${{z_probe}}\"\n\
         env \"BURV_LOG_DIR=${{z_logs}}\" \\\n\
         \x20 bash \"${{BURD_BUK_DIR}}/bud_dispatch.sh\" \"${{z_probe}}\" '{configuration}' \"${{PWD}}\" \"${{BURD_CONFIG_DIR}}\" \"buja-probe.LogFamily.sh\"\n",
        configuration = configuration, //
    );

    let bench = buth_Bench::buth_seat(name, BUTA_MODULES, &body);
    let said = bench.buth_drive(&[]);
    let where_ = buta_Where {
        logs: buta_after(&said.buth_text, BUTA_LOGS),
        report: buta_after(&said.buth_text, BUTA_REPORT),
    };
    (bench, said, where_)
}

/// The rest of the line a reported token opens.
fn buta_after(text: &str, token: &str) -> String {
    let at = text.find(token).unwrap_or_else(|| {
        panic!(
            "the coordinator never reported {}:\n{}",
            token,
            text, //
        )
    });
    text[at + token.len()..]
        .lines()
        .next()
        .unwrap_or_else(|| {
            panic!(
                "{} opened an empty line",
                token, //
            )
        })
        .trim_end()
        .to_string()
}

/// What the probe coordinator said it was handed.
fn buta_report(at: &buta_Where) -> String {
    std::fs::read_to_string(&at.report).unwrap_or_else(|err| {
        panic!(
            "the probe coordinator wrote no {}: {}",
            at.report,
            err, //
        )
    })
}

/// The value a reported assignment carries, quotes taken off.
fn buta_field(report: &str, name: &str) -> String {
    let prefix = format!(
        "{}=",
        name, //
    );
    let line = report
        .lines()
        .find(|line| line.starts_with(&prefix))
        .unwrap_or_else(|| {
            panic!(
                "no {} in the probe's report:\n{}",
                name,
                report, //
            )
        });
    line[prefix.len()..].trim().trim_matches('"').to_string()
}

/// How many members stand in the driven log directory.
fn buta_members(at: &buta_Where) -> usize {
    std::fs::read_dir(&at.logs)
        .unwrap_or_else(|err| {
            panic!(
                "no log directory at {}: {}",
                at.logs,
                err, //
            )
        })
        .count()
}

#[test]
fn buta_the_coordinator_is_handed_all_three_log_names() {
    let (_seat, said, at) = buta_drive("substrate-amanuensis-exports", "buuc_ownlog");
    said.buth_thrived();
    let report = buta_report(&at);

    // THE NAMES ARE THE DISPATCH'S, WHICH IS THE HALF THAT MATTERS. A coordinator
    // could compose three plausible paths of its own and satisfy a presence
    // check; each name is therefore required to stand under the directory THIS
    // dispatch was pointed at, which the coordinator had no way to invent.
    for field in ["buta_last", "buta_same", "buta_hist"] {
        let path = buta_field(&report, field);
        assert!(
            !path.is_empty(),
            "{} never reached the coordinator",
            field, //
        );
        assert!(
            path.starts_with(&at.logs),
            "{} names {} which stands outside the dispatch's log directory {}",
            field,
            path,
            at.logs, //
        );
    }
}

#[test]
fn buta_the_dispatch_creates_the_directory_and_none_of_its_members() {
    let (_seat, said, at) = buta_drive("substrate-amanuensis-none", "buuc_ownlog");
    said.buth_thrived();

    assert!(
        std::path::Path::new(&at.logs).is_dir(),
        "the log directory is absent: {}",
        at.logs, //
    );
    assert_eq!(
        buta_members(&at),
        0,
        "the dispatch wrote log members it owed none of, under {}",
        at.logs, //
    );
}

#[test]
fn buta_the_logged_control_writes_the_whole_family() {
    // THE CONTROL FOR THE ABSENCE ABOVE, driven through the same helper with no
    // word. Without it, a probe that had looked at the wrong directory would
    // report the same zero the working mode reports.
    let (_seat, said, at) = buta_drive("substrate-amanuensis-control", "");
    said.buth_thrived();

    // THE DISPATCH FILE OF A LOGGED RUN carries the historical member, which a
    // later reader finds the record by, and not the last and same pair, whose
    // standing is how a reader knows the own-log mode.
    let temp = buta_field(&buta_report(&at), "buta_temp");
    let burd = std::path::Path::new(&temp).join("burd.env");
    let body = std::fs::read_to_string(&burd).unwrap_or_else(|err| {
        panic!(
            "no dispatch file at {}: {}",
            burd.display(),
            err, //
        )
    });
    assert!(
        body.lines().any(|l| l.starts_with("BURD_LOG_HIST=")),
        "the historical member is recorded:\n{}",
        body, //
    );
    assert!(
        !body.lines().any(|l| l.starts_with("BURD_LOG_LAST=")),
        "no last member outside own-log:\n{}",
        body, //
    );
    assert!(
        !body.lines().any(|l| l.starts_with("BURD_LOG_SAME=")),
        "no same member outside own-log:\n{}",
        body, //
    );
    assert_eq!(
        buta_members(&at),
        BUTA_FAMILY,
        "a logged dispatch wrote {} member(s) under {}, where {} were owed — and the \
         absence the flagged hurdles assert is unproven without this",
        buta_members(&at),
        at.logs,
        BUTA_FAMILY, //
    );
}

#[test]
fn buta_the_fact_file_names_the_same_historical_member_the_coordinator_was_handed() {
    // TWO CHANNELS MUST AGREE. The coordinator is handed the path in its
    // environment and a later reader finds it in the fact file; a mode that
    // exported one path and recorded another would leave the two consumers of the
    // same run pointing at different records.
    let (_seat, said, at) = buta_drive("substrate-amanuensis-burx", "buuc_ownlog");
    said.buth_thrived();
    let report = buta_report(&at);

    // AND THE DISPATCH FILE NAMES ALL THREE, the pair beside the historical
    // member being the own-log mode's mark for a reader of the file.
    let temp_dir = buta_field(&report, "buta_temp");
    let burd = std::fs::read_to_string(std::path::Path::new(&temp_dir).join("burd.env"))
        .unwrap_or_else(|err| {
            panic!(
                "no dispatch file under {}: {}",
                temp_dir,
                err, //
            )
        });
    for (member, field) in [
        ("BURD_LOG_LAST", "buta_last"),
        ("BURD_LOG_SAME", "buta_same"),
        ("BURD_LOG_HIST", "buta_hist"),
    ] {
        assert_eq!(
            buta_field(&burd, member),
            buta_field(&report, field),
            "the dispatch file and the coordinator's environment name different paths for {}",
            member, //
        );
    }
}

#[test]
fn buta_the_mode_refuses_to_stand_beside_no_log() {
    // THE TWO WORDS ASK FOR OPPOSITE THINGS — one says the coordinator will write
    // the record, the other says there is no record — so admitting both would
    // make the outcome depend on which branch happened to run last.
    let (_seat, said, at) = buta_drive("substrate-amanuensis-nolog", "buuc_ownlog buuc_nolog");
    said.buth_died()
        .buth_carries("buuc_ownlog")
        .buth_carries("buuc_nolog");

    // AND IT REFUSES BEFORE THE COORDINATOR RUNS, which is the part that makes
    // the refusal safe rather than merely correct: a mode conflict caught after
    // the work had started would leave half a record behind.
    assert!(
        !std::path::Path::new(&at.report).is_file(),
        "the coordinator ran despite the refusal", //
    );
}

#[test]
fn buta_the_mode_refuses_to_stand_beside_interactive() {
    let (_seat, said, at) = buta_drive(
        "substrate-amanuensis-interactive",
        "buuc_ownlog buuc_interactive",
    );
    said.buth_died()
        .buth_carries("buuc_ownlog")
        .buth_carries("buuc_interactive");
    assert!(
        !std::path::Path::new(&at.report).is_file(),
        "the coordinator ran despite the refusal", //
    );
}
// eof
