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

//! Hurdles over the kibble: the reading of a declaration, the composition of a
//! residence, and the seal's own shape.
//!
//! THE CONVERGE ITSELF IS HURDLED OVER A LURE, in `bktd_drive` under `tests/`,
//! because it spawns the door and needs a repository of its own. What stands
//! here is everything answerable without one.

use crate::bkcq_kibble::{
    bkcq_findings, bkcq_Kibble, BKCQ_FAMILY, BKCQ_FIELDS_BOTH, BKCQ_FIELDS_PREBUILT,
    BKCQ_FIELDS_SOURCE, BKCQ_KIND_PREBUILT, BKCQ_KIND_SOURCE,
};
use crate::bkcl_leash;
use bkl::bklrc_catena::bklrc_admit;

/// A kibble composed from text, so a hurdle poses a declaration without a tree.
///
/// THE PARSE IS THE REAL ONE. The text goes through the reader crate exactly as
/// a file on disk does, so a hurdle proving a finding is proving it against the
/// bytes a kibble author would write rather than against a struct built by hand.
fn zbktq_posed(name: &str, text: &str) -> bkcq_Kibble {
    bkcq_Kibble {
        name: name.to_string(),
        instance: std::path::PathBuf::from("Tools/bkk").join(name),
        regime: bklrc_admit(text, "a posed kibble").expect("the posed text admits"),
    }
}

/// A seal and a version of the right SHAPE and no other truth, so a hurdle about
/// some other field is not also failing the seal's.
///
/// PLAINLY FABRICATED, AND THAT IS THE POINT TWICE OVER. A posed declaration
/// carrying the real pin would be a SECOND HOME for it — the version would then
/// stand in two files, and a bump would leave this one behind saying something
/// that used to be true. And a reader meeting the estate's own digest here would
/// reasonably take these hurdles for statements about nextest, where what they
/// actually prove is what the READER does with a declaration of any kind. The
/// digest below is sixty-four hex characters and nothing's sha256.
const ZBKTQ_SEAL: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const ZBKTQ_VERSION: &str = "0.0.1-posed";
const ZBKTQ_BUMPED: &str = "0.0.2-posed";

/// A whole conforming source kibble, which the hurdles below vary one field at a
/// time from.
fn zbktq_whole(name: &str) -> String {
    format!(
        "BKRK_KIBBLE=\"{}\"\n\
         BKRK_KIND=\"{}\"\n\
         BKRK_PROGRAM=\"cargo-nextest\"\n\
         BKRK_VERSION=\"{}\"\n\
         BKRK_SEAL=\"{}\"\n\
         BKRK_BYNAME=\"cargo-nextest\"\n\
         BKRK_REGISTRY=\"https://static.crates.io/crates\"\n\
         BKRK_CHANNEL=\"1.90.0\"\n\
         BKRK_TARGET=\"cargo-nextest\"\n\
         BKRK_FEATURES=\"default-no-update\"\n",
        name, BKCQ_KIND_SOURCE, ZBKTQ_VERSION, ZBKTQ_SEAL
    )
}

#[test]
fn bktq_a_whole_source_kibble_carries_no_finding() {
    let kibble = zbktq_posed("bki_nextest", &zbktq_whole("bki_nextest"));
    let findings = bkcq_findings(&kibble);

    assert!(
        findings.is_empty(),
        "a conforming source kibble reported: {:?}",
        findings
    );
}

#[test]
fn bktq_every_owed_field_is_reported_when_it_is_absent() {
    // THE CONTROL IS THE WHOLE DECLARATION ABOVE, which reports nothing — so a
    // finding here is this field's absence and not a defect the whole one shares.
    for field in BKCQ_FIELDS_BOTH.iter().chain(BKCQ_FIELDS_SOURCE.iter()) {
        let text: String = zbktq_whole("bki_nextest")
            .lines()
            .filter(|line| !line.starts_with(&format!("{}=", field)))
            .collect::<Vec<&str>>()
            .join("\n");

        let findings = bkcq_findings(&zbktq_posed("bki_nextest", &text));

        assert!(
            findings.iter().any(|said| said.starts_with(field)),
            "dropping {} reported {:?}",
            field,
            findings
        );
    }
}

#[test]
fn bktq_a_prebuilt_kibble_declaring_a_build_field_is_reported() {
    // The prebuilt kind is fetched already built, so a field about the build is
    // a declaration about an act that never happens.
    let text = zbktq_whole("bki_uv").replace(BKCQ_KIND_SOURCE, BKCQ_KIND_PREBUILT);
    let findings = bkcq_findings(&zbktq_posed("bki_uv", &text));

    for field in BKCQ_FIELDS_SOURCE {
        assert!(
            findings
                .iter()
                .any(|said| said.starts_with(field) && said.contains("source kind")),
            "{} was not reported as foreign to the prebuilt kind: {:?}",
            field,
            findings
        );
    }
}

#[test]
fn bktq_a_kind_outside_the_roster_is_reported_and_names_both_values() {
    let text = zbktq_whole("bki_nextest").replace(BKCQ_KIND_SOURCE, "bknre_borrowed");
    let findings = bkcq_findings(&zbktq_posed("bki_nextest", &text));

    let said = findings
        .iter()
        .find(|said| said.starts_with("BKRK_KIND"))
        .expect("an unadmitted kind is reported");

    assert!(said.contains(BKCQ_KIND_SOURCE), "{}", said);
    assert!(said.contains(BKCQ_KIND_PREBUILT), "{}", said);
}

#[test]
fn bktq_a_name_disagreeing_with_its_directory_is_reported() {
    // THE IDENTITY LAW. The declared name and the instance directory are one,
    // and a disagreement leaves a reader no way to say which a collar meant.
    let kibble = zbktq_posed("bki_nextest", &zbktq_whole("bki_uv"));
    let findings = bkcq_findings(&kibble);

    let said = findings
        .iter()
        .find(|said| said.starts_with("BKRK_KIBBLE"))
        .expect("a disagreeing name is reported");

    assert!(said.contains("bki_uv"), "{}", said);
    assert!(said.contains("bki_nextest"), "{}", said);
}

#[test]
fn bktq_a_seal_of_the_wrong_width_is_reported_before_anything_is_fetched() {
    let text = zbktq_whole("bki_nextest").replace(ZBKTQ_SEAL, "d8a41c20");
    let findings = bkcq_findings(&zbktq_posed("bki_nextest", &text));

    assert!(
        findings
            .iter()
            .any(|said| said.starts_with("BKRK_SEAL") && said.contains("64")),
        "a short seal reported {:?}",
        findings
    );
}

#[test]
fn bktq_a_seal_spelled_in_uppercase_is_reported() {
    // OPENSSL ANSWERS IN LOWERCASE and the seal is compared against that answer
    // as text, so an uppercase digest would refuse a correct archive.
    let text = zbktq_whole("bki_nextest").replace(ZBKTQ_SEAL, &ZBKTQ_SEAL.to_uppercase());
    let findings = bkcq_findings(&zbktq_posed("bki_nextest", &text));

    assert!(
        findings.iter().any(|said| said.starts_with("BKRK_SEAL")),
        "an uppercase seal reported {:?}",
        findings
    );
}

#[test]
fn bktq_the_residence_carries_the_version_in_its_path() {
    // THE PIN IS A PATH TEST, and this is the property that makes it one: a
    // residence keyed on the name alone would go on answering "something is
    // here" after a bump.
    //
    // THE STORE IS POSED AS A VALUE AND NEVER AS AN ENVIRONMENT. Writing the
    // process's environment would decide the answers of every hurdle the harness
    // is running beside this one, which is not hypothetical — it was driven, and
    // it reddened an election hurdle three files away.
    let store = std::path::Path::new("/posed/tackroom");
    let kibble = zbktq_posed("bki_nextest", &zbktq_whole("bki_nextest"));

    let residence = kibble.bkcq_residence_at(store);
    let said = residence.display().to_string();

    assert!(said.starts_with("/posed/tackroom"), "{}", said);
    assert!(said.contains(ZBKTQ_VERSION), "{}", said);
    assert!(said.contains("bki_nextest"), "{}", said);
    assert!(said.ends_with("cargo-nextest"), "{}", said);

    // A BUMPED VERSION MOVES THE PATH, which is the discriminator: without it
    // the assertions above would pass over a residence that ignored the version.
    let bumped = zbktq_posed(
        "bki_nextest",
        &zbktq_whole("bki_nextest").replace(ZBKTQ_VERSION, ZBKTQ_BUMPED),
    );

    assert_ne!(residence, bumped.bkcq_residence_at(store));
}

#[test]
fn bktq_an_absent_tackroom_refuses_naming_the_station_variable() {
    // NEVER A FALLBACK TO A PERSONAL TOOL HOME. The standing breach this regime
    // ends is nextest found in the station user's own cargo home, so a residence
    // composed without a tackroom would reinstate exactly that.
    //
    // THE SENTENCE IS ASSERTED WHERE IT IS COMPOSED, rather than by removing the
    // variable and driving the reading: the removal is what would race the
    // suite, and the composed refusal is the thing a station actually meets.
    let kibble = zbktq_posed("bki_nextest", &zbktq_whole("bki_nextest"));
    let refusal = kibble.bkcq_unstationed();

    assert!(
        refusal.contains(bkcl_leash::BKCL_TACKROOM_VAR),
        "{}",
        refusal
    );
    assert!(refusal.contains("personal tool home"), "{}", refusal);
    assert!(refusal.contains("bki_nextest"), "{}", refusal);
}

#[test]
fn bktq_the_archive_joins_the_version_onto_the_declared_registry() {
    let kibble = zbktq_posed("bki_nextest", &zbktq_whole("bki_nextest"));

    assert_eq!(
        kibble.bkcq_archive(),
        format!(
            "https://static.crates.io/crates/cargo-nextest/cargo-nextest-{}.crate",
            ZBKTQ_VERSION
        )
    );

    // A TRAILING SEPARATOR ON THE BASE CHANGES NOTHING, so a kibble author's
    // spelling cannot compose a doubled slash the registry would not answer.
    let slashed = zbktq_posed(
        "bki_nextest",
        &zbktq_whole("bki_nextest").replace(
            "https://static.crates.io/crates\"",
            "https://static.crates.io/crates/\"",
        ),
    );

    assert_eq!(slashed.bkcq_archive(), kibble.bkcq_archive());
}

/// A whole conforming PREBUILT kibble, which the hurdles below vary one field at
/// a time from.
///
/// IT SHARES NO BUILD FIELD WITH THE SOURCE POSER ABOVE, which is the whole
/// shape of the two kinds: the roster a kibble owes follows the kind it
/// declares, and the fields of the other kind are not merely unused but foreign.
fn zbktq_whole_prebuilt(name: &str) -> String {
    format!(
        "BKRK_KIBBLE=\"{}\"\n\
         BKRK_KIND=\"{}\"\n\
         BKRK_PROGRAM=\"uv\"\n\
         BKRK_VERSION=\"{}\"\n\
         BKRK_SEAL=\"{}\"\n\
         BKRK_BYNAME=\"uv\"\n\
         BKRK_LARDER=\"https://example.invalid/releases/download\"\n\
         BKRK_BORDEREAU=\"dist-manifest.json\"\n",
        name, BKCQ_KIND_PREBUILT, ZBKTQ_VERSION, ZBKTQ_SEAL
    )
}

#[test]
fn bktq_a_whole_prebuilt_kibble_carries_no_finding() {
    let kibble = zbktq_posed("bki_uv", &zbktq_whole_prebuilt("bki_uv"));
    let findings = bkcq_findings(&kibble);

    assert!(
        findings.is_empty(),
        "a conforming prebuilt kibble reported: {:?}",
        findings
    );
}

#[test]
fn bktq_every_field_the_prebuilt_kind_owes_is_reported_when_it_is_absent() {
    // THE CONTROL IS THE WHOLE DECLARATION ABOVE, which reports nothing.
    for field in BKCQ_FIELDS_BOTH.iter().chain(BKCQ_FIELDS_PREBUILT.iter()) {
        let text: String = zbktq_whole_prebuilt("bki_uv")
            .lines()
            .filter(|line| !line.starts_with(&format!("{}=", field)))
            .collect::<Vec<&str>>()
            .join("\n");

        let findings = bkcq_findings(&zbktq_posed("bki_uv", &text));

        assert!(
            findings.iter().any(|said| said.starts_with(field)),
            "dropping {} reported {:?}",
            field,
            findings
        );
    }
}

#[test]
fn bktq_a_source_kibble_declaring_a_release_field_is_reported() {
    // THE MIRROR OF THE HURDLE ABOVE, and it is owed: a roster policed in one
    // direction alone would admit a source kibble carrying a release store it
    // never fetches from, which is a declaration about an act that never happens
    // exactly as a build field on a prebuilt kibble is.
    let text = format!(
        "{}\
         BKRK_LARDER=\"https://example.invalid/releases/download\"\n\
         BKRK_BORDEREAU=\"dist-manifest.json\"\n",
        zbktq_whole("bki_nextest")
    );
    let findings = bkcq_findings(&zbktq_posed("bki_nextest", &text));

    for field in BKCQ_FIELDS_PREBUILT {
        assert!(
            findings
                .iter()
                .any(|said| said.starts_with(field) && said.contains("prebuilt kind")),
            "{} was not reported as foreign to the source kind: {:?}",
            field,
            findings
        );
    }
}

#[test]
fn bktq_the_bordereau_and_its_siblings_join_the_version_between_store_and_name() {
    let kibble = zbktq_posed("bki_uv", &zbktq_whole_prebuilt("bki_uv"));

    assert_eq!(
        kibble.bkcq_bordereau(),
        format!(
            "https://example.invalid/releases/download/{}/dist-manifest.json",
            ZBKTQ_VERSION
        )
    );

    // AN ARCHIVE IS REACHED AS THE DOCUMENT'S SIBLING, the name coming from the
    // document rather than from anything this kibble declares.
    assert_eq!(
        kibble.bkcq_beside("uv-posed-triple.tar.gz"),
        format!(
            "https://example.invalid/releases/download/{}/uv-posed-triple.tar.gz",
            ZBKTQ_VERSION
        )
    );

    // A TRAILING SEPARATOR ON THE STORE CHANGES NOTHING.
    let slashed = zbktq_posed(
        "bki_uv",
        &zbktq_whole_prebuilt("bki_uv").replace(
            "https://example.invalid/releases/download\"",
            "https://example.invalid/releases/download/\"",
        ),
    );

    assert_eq!(slashed.bkcq_bordereau(), kibble.bkcq_bordereau());
}

#[test]
fn bktq_the_family_file_is_not_the_collar_family_file() {
    // TWO FAMILIES SHARE ONE WALK AND ARE TOLD APART BY THIS FILE ALONE, so the
    // two names agreeing would make one walk answer for both.
    assert_ne!(BKCQ_FAMILY, crate::bkcr_resolve::BKCR_FAMILY);
}

// eof
