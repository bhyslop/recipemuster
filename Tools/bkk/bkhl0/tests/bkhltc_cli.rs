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

//! Hurdles for the built binary: its main reads the clock and the regime files
//! from the working directory and calls the library, so a composed project
//! driven through the binary proves the wiring the library hurdles cannot.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

use bkhl::bkhlrd_day;
use bkhl::bkhlrg_ground;

// THE YARD IS THE LIBRARY'S OWN TEST MODULE, reached by path from this second
// compilation unit rather than copied: an integration hurdle cannot see the
// library's test modules, and the yard names only the two modules seated above.
// Each crate uses part of it, so the part this one leaves unused is allowed.
#[path = "../src/bkhlty_yard.rs"]
#[allow(dead_code)]
mod bkhlty_yard;

use bkhl::bkhlrb_bale::bkhlrb_latest;
use bkhl::bkhlrc_cli::{
    BKHLRC_BALE,
    BKHLRC_UNBALE, //
};
use bkhlrd_day::bkhlrd_Day;
use bkhlty_yard::{
    bkhlty_Yard,
    bkhlty_confine,
    BKHLTY_HAYLOFT,
    BKHLTY_OUTPUT_APART, //
};
use std::process::{
    Command,
    Output, //
};
use std::time::SystemTime;

/// Run the built binary in a composed project with these arguments.
fn zbkhltc_run(yard: &bkhlty_Yard, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_bkhl"))
        .args(args)
        .current_dir(yard.bkhlty_project())
        .output()
        .expect("the built binary runs")
}

#[test]
fn bkhltc_the_binary_refuses_a_missing_malformed_or_later_day_naming_the_latest() {
    let yard = bkhlty_Yard::bkhlty_compose("cli-later", true, BKHLTY_OUTPUT_APART);
    let today = bkhlrd_Day::bkhlrd_from_time(SystemTime::now());
    let planted = yard.bkhlty_plant("../murkwell/aged/a.txt", b"liver", today.bkhlrd_back(30));
    let ground = bkhlrg_ground::bkhlrg_read(&yard.bkhlty_project()).expect("the project reads");
    bkhlty_confine(&yard, &ground);
    let tomorrow = today.bkhlrd_back(-1).bkhlrd_digits();
    let yesterday = today.bkhlrd_back(1).bkhlrd_digits();
    let latest = bkhlrb_latest(today).bkhlrd_digits();

    for args in [
        vec![BKHLRC_BALE],
        vec![BKHLRC_BALE, "2026-1-31"],
        vec![BKHLRC_BALE, "20261301"],
        vec![BKHLRC_BALE, tomorrow.as_str()],
        vec![BKHLRC_BALE, yesterday.as_str()],
    ] {
        let ran = zbkhltc_run(&yard, &args);

        assert_eq!(ran.status.code(), Some(1), "{:?} exits failing", args);
        let said = String::from_utf8_lossy(&ran.stderr);
        assert!(
            said.trim_end().ends_with(&latest),
            "{:?} should close on the latest day {}: {}",
            args,
            latest,
            said, //
        );
        assert!(planted.exists());
    }
    assert!(!ground.hayloft.expect("named").folder.exists());
}

#[test]
fn bkhltc_the_binary_lists_the_archive_folder_bare() {
    let yard = bkhlty_Yard::bkhlty_compose("cli-listing", true, BKHLTY_OUTPUT_APART);
    let today = bkhlrd_Day::bkhlrd_from_time(SystemTime::now());
    let ground = bkhlrg_ground::bkhlrg_read(&yard.bkhlty_project()).expect("the project reads");
    bkhlty_confine(&yard, &ground);
    for name in ["zq-20260101-20260131.tar.gz", "zq-20260201-20260228.tar.gz"] {
        yard.bkhlty_plant(
            &format!(
                "{}/{}",
                BKHLTY_HAYLOFT,
                name, //
            ),
            b"x",
            today,
        );
    }

    let ran = zbkhltc_run(&yard, &[BKHLRC_UNBALE]);

    assert!(
        ran.status.success(),
        "{}",
        String::from_utf8_lossy(&ran.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&ran.stdout),
        format!(
            "{}/zq-20260101-20260131.tar.gz\n{}/zq-20260201-20260228.tar.gz\n",
            BKHLTY_HAYLOFT,
            BKHLTY_HAYLOFT, //
        )
    );
}

#[test]
fn bkhltc_the_binary_refuses_an_unknown_shape_with_its_usage() {
    let yard = bkhlty_Yard::bkhlty_compose("cli-usage", true, BKHLTY_OUTPUT_APART);

    let ran = zbkhltc_run(&yard, &["gallop"]);

    assert_eq!(ran.status.code(), Some(1));
    let said = String::from_utf8_lossy(&ran.stderr);
    assert!(said.contains("UTC"), "{}", said);
}

// eof
