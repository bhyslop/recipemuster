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

//! The proclamation's hurdles.
//!
//! They compose a collar in hand rather than over a lure, because what is under
//! test is the RENDER and not the reading: a proclamation is a pure function of a
//! collar, and spending a composed repository on it would prove nothing the
//! resolver's own hurdles do not already prove.

use super::bkca_whereabouts::bkca_Geography;
use super::bkcr_resolve::bkcr_Collar;
use super::bkce_election::{BKCE_ELECTED_BORROW, BKCE_ELECTED_OWN};
use super::bkct_tattoo::{bkct_proclamation, BKCT_LANGUAGE_RUST};
use bkl::bklrc_catena::bklrc_admit;
use std::path::PathBuf;

/// A suite collar in hand.
const ZBKTT_SUITE: &str = "\
BKRR_COLLAR=\"suite-lure\"
BKRR_KIND=\"bknre_suite\"
BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"
BKRR_TARGET=\"bknre_manifest\"
BKRR_ROOTS=\"Tools/lure/src \\
  Tools/lure/Cargo.toml\"
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

/// An app collar in hand, differing from the suite in the coordinates its kind
/// owns and in nothing else.
const ZBKTT_APP: &str = "\
BKRR_COLLAR=\"app-lure\"
BKRR_KIND=\"bknre_app\"
BKRR_MANIFEST=\"Tools/lure/Cargo.toml\"
BKRR_TARGET=\"lurex\"
BKRR_ROOTS=\"Tools/lure/src\"
BKRR_FEATURES=\"\"
BKRR_PROFILE=\"release\"
BKRR_SPEND=\"bknre_writer\"
BKRR_TAMED_CRATES=\"\"
BKRR_FERAL_CRATES=\"bknre_tamed\"
BKRR_MUZZLE=\".\"
BKRR_EXERGUE=\"bknre_unstruck\"
BKRR_RESIDENCE=\"Tools/lure/target/release\"
BKRR_BYNAME=\"lurid\"
";

/// Compose a collar without touching a disk.
fn zbktt_collar(name: &str, text: &str) -> bkcr_Collar {
    bkcr_Collar {
        name: name.to_string(),
        instance: PathBuf::from(name),
        regime: bklrc_admit(text, "composed").expect("the collar stands inside the subset"),
    }
}

/// THE PROCLAMATION IS TENANT-BLIND. Every proclaimed field wears `BKRC_*`, and
/// no `BKRR_*` name reaches the answer at all — a consumer that read the rust
/// family's own spelling here would be coupled to the tenant this surface exists
/// to hide.
#[test]
fn bktt_the_proclamation_names_no_family_field() {
    for (name, text) in [("suite-lure", ZBKTT_SUITE), ("app-lure", ZBKTT_APP)] {
        let said = bkct_proclamation(&zbktt_collar(name, text), BKCE_ELECTED_OWN, &bkca_Geography::Undispatched);

        assert!(
            !said.contains("BKRR_"),
            "the proclamation for {} must name no family field, and it reads:\n{}",
            name,
            said
        );
        assert!(said.contains("BKRC_"), "it reads:\n{}", said);
    }
}

/// The tenant-blind core, field by field, and the language carried as a field
/// INSIDE the proclamation rather than as a different shape.
#[test]
fn bktt_the_proclaimed_core_stands() {
    let said = bkct_proclamation(&zbktt_collar("suite-lure", ZBKTT_SUITE), BKCE_ELECTED_OWN, &bkca_Geography::Undispatched);

    for owed in [
        "BKRC_COLLAR=\"suite-lure\"",
        "BKRC_KIND=\"bknre_suite\"",
        "BKRC_SPEND=\"bknre_reader\"",
        "BKRC_MANIFEST=\"Tools/lure/Cargo.toml\"",
        "BKRC_TARGET=\"bknre_manifest\"",
        "BKRC_PROFILE=\"test\"",
        "BKRC_FEATURES=\"\"",
    ] {
        assert!(said.contains(owed), "'{}' is owed, and it reads:\n{}", owed, said);
    }

    assert!(
        said.contains(&format!("BKRC_LANGUAGE=\"{}\"", BKCT_LANGUAGE_RUST)),
        "it reads:\n{}",
        said
    );
}

/// Each kind proclaims the launch coordinates it owns, and neither proclaims the
/// other's.
#[test]
fn bktt_each_kind_proclaims_its_own_coordinates() {
    let suite = bkct_proclamation(&zbktt_collar("suite-lure", ZBKTT_SUITE), BKCE_ELECTED_OWN, &bkca_Geography::Undispatched);
    assert!(suite.contains("BKRC_RUNNER=\"bknre_cargo\""), "it reads:\n{}", suite);
    assert!(suite.contains("BKRC_TONGUE=\"bknre_harness\""), "it reads:\n{}", suite);
    assert!(!suite.contains("BKRC_RESIDENCE"), "it reads:\n{}", suite);

    let app = bkct_proclamation(&zbktt_collar("app-lure", ZBKTT_APP), BKCE_ELECTED_OWN, &bkca_Geography::Undispatched);
    assert!(
        app.contains("BKRC_RESIDENCE=\"Tools/lure/target/release\""),
        "it reads:\n{}",
        app
    );
    assert!(app.contains("BKRC_BYNAME=\"lurid\""), "it reads:\n{}", app);
    assert!(!suite.contains("BKRC_BYNAME"), "it reads:\n{}", suite);
    assert!(!app.contains("BKRC_RUNNER"), "it reads:\n{}", app);
    assert!(!app.contains("BKRC_TONGUE"), "it reads:\n{}", app);
}

/// The election slot is a DECLARED VALUE, not an absent field: a consumer reading
/// an election out of an absence could not tell a declaration from an omission.
///
/// IT CARRIES WHAT THE DOOR RULED, which the second value proves and the first
/// alone could not — a render that spelled one constant into this field whatever
/// it was handed would pass a single-value assertion and be wrong about every
/// collar the election went the other way for.
#[test]
fn bktt_the_election_slot_carries_what_the_door_ruled() {
    for elected in [BKCE_ELECTED_OWN, BKCE_ELECTED_BORROW] {
        let said = bkct_proclamation(&zbktt_collar("suite-lure", ZBKTT_SUITE), elected, &bkca_Geography::Undispatched);

        assert!(
            said.contains(&format!("BKRC_ELECTION=\"{}\"", elected)),
            "handed {}, it reads:\n{}",
            elected,
            said
        );
    }
}

/// THE ANSWER ROUND-TRIPS THROUGH THE READER THAT PRODUCED THE COLLAR. The
/// proclamation is env-shaped text a person reads and bash sources, so the
/// catena fields are rendered in the substrate's own authored layout — and the
/// proof that the layout is right is that the reader gives back the elements
/// that went in.
#[test]
fn bktt_the_answer_reads_back_through_the_reader() {
    let said = bkct_proclamation(&zbktt_collar("suite-lure", ZBKTT_SUITE), BKCE_ELECTED_OWN, &bkca_Geography::Undispatched);

    let read = bklrc_admit(&said, "the proclamation")
        .expect("the proclamation stands inside the catena law it was rendered to");

    assert_eq!(read.bklrc_scalar("BKRC_COLLAR"), Some("suite-lure"));
    assert_eq!(
        read.bklrc_catena("BKRC_ROOTS").unwrap(),
        vec!["Tools/lure/src", "Tools/lure/Cargo.toml"],
        "a multi-element field is rendered one element per line and reads back as its elements"
    );
    assert_eq!(read.bklrc_catena("BKRC_FEATURES"), Some(Vec::new()));
    assert_eq!(
        read.bklrc_scalar("BKRC_ELECTION"),
        Some(BKCE_ELECTED_OWN)
    );
}

// eof
