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

//! Hurdles for reading the folders from the regime files: the road from the
//! trampoline, the optional archive folder, and every nesting refused before
//! anything is written.

use crate::bkhlrg_ground::{
    bkhlrg_read,
    BKHLRG_HAYLOFT_EXAMPLE,
    BKHLRG_HAYLOFT_KEY,
    BKHLRG_LOG_KEY,
    BKHLRG_TEMP_KEY,
    BKHLRG_TINCTURE_KEY, //
};
use crate::bkhlty_yard::{
    bkhlty_Yard,
    bkhlty_confine,
    BKHLTY_HAYLOFT,
    BKHLTY_OUTPUT_APART,
    BKHLTY_RECORDS,
    BKHLTY_RUNS,
    BKHLTY_TINCTURE, //
};

/// Rewrite the yard's per-machine file with these lines.
fn zbkhltg_machine(yard: &bkhlty_Yard, lines: &[(&str, &str)]) {
    let text: String = lines
        .iter()
        .map(|(key, value)| {
            format!(
                "{}={}\n",
                key,
                value, //
            )
        })
        .collect();
    yard.bkhlty_write("machine/burs.env", &text);
}

#[test]
fn bkhltg_the_files_name_every_folder_inside_the_yard() {
    let yard = bkhlty_Yard::bkhlty_compose("road", true, BKHLTY_OUTPUT_APART);
    let ground = bkhlrg_read(&yard.bkhlty_project()).expect("the composed project reads");
    bkhlty_confine(&yard, &ground);

    assert_eq!(ground.tincture, BKHLTY_TINCTURE);
    assert!(ground.log.ends_with("quillery"));
    assert!(ground.temp.ends_with("murkwell"));
    assert!(ground.output.ends_with("outfold"));
    let hayloft = ground
        .hayloft
        .as_ref()
        .expect("the archive folder is named");
    assert_eq!(hayloft.spelled, BKHLTY_HAYLOFT);
    assert!(hayloft.folder.ends_with("strawloft"));
}

#[test]
fn bkhltg_an_absent_archive_folder_refuses_naming_the_line_to_add() {
    let yard = bkhlty_Yard::bkhlty_compose("unnamed", false, BKHLTY_OUTPUT_APART);
    let ground = bkhlrg_read(&yard.bkhlty_project()).expect("the composed project reads");
    bkhlty_confine(&yard, &ground);

    let why = ground
        .bkhlrg_hayloft()
        .expect_err("no archive folder is named");
    let line = format!(
        "{}={}",
        BKHLRG_HAYLOFT_KEY,
        BKHLRG_HAYLOFT_EXAMPLE, //
    );
    assert!(why.contains(&line), "{}", why);
}

#[test]
fn bkhltg_an_archive_folder_inside_a_swept_folder_is_refused() {
    let yard = bkhlty_Yard::bkhlty_compose("inside", true, BKHLTY_OUTPUT_APART);
    zbkhltg_machine(
        &yard,
        &[
            (BKHLRG_TINCTURE_KEY, BKHLTY_TINCTURE),
            (BKHLRG_LOG_KEY, BKHLTY_RECORDS),
            (BKHLRG_HAYLOFT_KEY, "../murkwell/strawloft"), //
        ],
    );
    let why = bkhlrg_read(&yard.bkhlty_project()).expect_err("the archive folder is swept");
    assert!(why.contains(BKHLRG_HAYLOFT_KEY), "{}", why);
    assert!(why.contains(BKHLRG_TEMP_KEY), "{}", why);
}

#[test]
fn bkhltg_one_swept_folder_inside_the_other_is_refused() {
    let yard = bkhlty_Yard::bkhlty_compose("nested", true, BKHLTY_OUTPUT_APART);
    zbkhltg_machine(
        &yard,
        &[
            (BKHLRG_TINCTURE_KEY, BKHLTY_TINCTURE),
            (BKHLRG_LOG_KEY, "../murkwell/quillery"),
            (BKHLRG_HAYLOFT_KEY, BKHLTY_HAYLOFT), //
        ],
    );
    let why = bkhlrg_read(&yard.bkhlty_project()).expect_err("the folders nest");
    assert!(why.contains("one inside the other"), "{}", why);
}

#[test]
fn bkhltg_a_swept_folder_holding_the_project_root_is_refused() {
    for value in [".", ".."] {
        let yard = bkhlty_Yard::bkhlty_compose("rooted", true, BKHLTY_OUTPUT_APART);
        zbkhltg_machine(
            &yard,
            &[
                (BKHLRG_TINCTURE_KEY, BKHLTY_TINCTURE),
                (BKHLRG_LOG_KEY, value),
                (BKHLRG_HAYLOFT_KEY, BKHLTY_HAYLOFT), //
            ],
        );
        let why = bkhlrg_read(&yard.bkhlty_project()).expect_err("the folder holds the root");
        assert!(why.contains("project root"), "{}", why);
    }
}

#[test]
fn bkhltg_an_absolute_value_is_refused() {
    let yard = bkhlty_Yard::bkhlty_compose("absolute", true, BKHLTY_OUTPUT_APART);
    let absolute = yard.bkhlty_at(BKHLTY_RUNS);
    zbkhltg_machine(
        &yard,
        &[
            (BKHLRG_TINCTURE_KEY, BKHLTY_TINCTURE),
            (BKHLRG_LOG_KEY, BKHLTY_RECORDS),
            (BKHLRG_HAYLOFT_KEY, &absolute.display().to_string()), //
        ],
    );
    let why = bkhlrg_read(&yard.bkhlty_project()).expect_err("the value is absolute");
    assert!(why.contains("absolute"), "{}", why);
}

// eof
