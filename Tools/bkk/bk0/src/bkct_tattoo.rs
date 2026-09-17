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

//! *tattoo* — the read-only examination of one collar, and the resolver's
//! read-only face (BKSNC-Kennelcraft.adoc "The doors").
//!
//! It changes nothing on disk and it is the door a reader reaches for to ask
//! what a collar says and whether it conforms. Everything it knows it learns
//! from the resolver, which is what makes "never operate on an invalid collar"
//! true of this door without this door holding a rule about it.
//!
//! THE PROCLAMATION IS TENANT-BLIND (BKSCL-Collar.adoc "The Common
//! Proclamation"). What a collar says to the rest of the system wears `BKRC_*`
//! whatever family declared it, with the language carried as a FIELD INSIDE the
//! proclamation rather than as a different shape — which is the charter's own
//! sentence made mechanical, and the seam a python or typescript tenant will
//! surface through without this door learning anything new. No `BKRR_*` name
//! reaches the answer: a consumer that read the rust family's own spelling here
//! would be coupled to the tenant the proclamation exists to hide.
//!
//! ITS TRANSPORT IS ENV-SHAPED TEXT, settled at the command-surface freeze: a
//! person reads it and bash sources it, and a JSON transport arrives as a verb of
//! its own when a consumer stands for one. The catena fields are rendered in the
//! substrate's own authored layout, so the answer round-trips through the very
//! reader that produced it.

use crate::bkca_whereabouts::bkca_Geography;
use crate::bkcx_python::bkcx_Collar;
use crate::bkcr_resolve::{bkcr_Collar, BKCR_KIND_APP, BKCR_KIND_SUITE};

/// The language field's value for the rust family, in the substrate's enum sprue
/// form under this kit's plane.
pub const BKCT_LANGUAGE_RUST: &str = "bknre_rust";

/// The language field's value for the python family, on the same plane.
pub const BKCT_LANGUAGE_PYTHON: &str = "bknre_python";

/// The proclamation, field by field: which `BKRC_*` name carries it, and which
/// `BKRR_*` field of the rust family answered.
///
/// The pairing is the seam itself, and it is a table rather than a run of
/// statements so that a second family joins by writing its own table rather than
/// by teaching this door another dialect.
const ZBKCT_PROCLAIMED: &[(&str, &str)] = &[
    ("BKRC_COLLAR", "BKRR_COLLAR"),
    ("BKRC_KIND", "BKRR_KIND"),
    ("BKRC_SPEND", "BKRR_SPEND"),
    ("BKRC_MANIFEST", "BKRR_MANIFEST"),
    ("BKRC_TARGET", "BKRR_TARGET"),
    ("BKRC_PROFILE", "BKRR_PROFILE"),
    ("BKRC_FEATURES", "BKRR_FEATURES"),
    ("BKRC_ROOTS", "BKRR_ROOTS"),
];

/// The app's own launch coordinates — the directory and the name the binary
/// wears inside it, which are two facts and not one.
const ZBKCT_PROCLAIMED_APP: &[(&str, &str)] = &[
    ("BKRC_RESIDENCE", "BKRR_RESIDENCE"),
    ("BKRC_BYNAME", "BKRR_BYNAME"),
];

/// The suite's own launch coordinates.
const ZBKCT_PROCLAIMED_SUITE: &[(&str, &str)] = &[
    ("BKRC_RUNNER", "BKRR_RUNNER"),
    ("BKRC_TONGUE", "BKRR_TONGUE"),
];

/// The python family's own table, and the seam working as the charter says it
/// does: a second family joins by writing its table rather than by teaching this
/// door another dialect.
///
/// THE PROJECT POINTER IS PROCLAIMED AS THE MANIFEST, and the row is where the
/// tenant-blindness is actually paid for. A consumer asks what file declares
/// this launchable's dependencies; a `pyproject.toml` answers that question
/// exactly as a `Cargo.toml` does, and a consumer told which of the two it was
/// looking at would be coupled to the tenant this vocabulary exists to hide.
///
/// THE BUILD SHAPE IS ABSENT BECAUSE THE FAMILY DECLARES NONE. The rust rows for
/// the profile and the features have no counterpart here: what a python
/// environment is made of is the lock, and a row rendering an empty value would
/// tell a consumer that a shape was declared and left blank.
const ZBKCT_PROCLAIMED_PYTHON: &[(&str, &str)] = &[
    ("BKRC_COLLAR", "BKRP_COLLAR"),
    ("BKRC_KIND", "BKRP_KIND"),
    ("BKRC_SPEND", "BKRP_SPEND"),
    ("BKRC_MANIFEST", "BKRP_MANIFEST"),
    ("BKRC_ROOTS", "BKRP_ROOTS"),
];

/// The python app's own launch coordinate — one where the rust family carries
/// two, the residence being the venv the kennel itself sited.
const ZBKCT_PROCLAIMED_PYTHON_APP: &[(&str, &str)] = &[("BKRC_BYNAME", "BKRP_BYNAME")];

/// The python suite's own launch coordinates.
const ZBKCT_PROCLAIMED_PYTHON_SUITE: &[(&str, &str)] = &[
    ("BKRC_TARGET", "BKRP_TARGET"),
    ("BKRC_RUNNER", "BKRP_RUNNER"),
    ("BKRC_TONGUE", "BKRP_TONGUE"),
];

/// The proclamation for one collar, ready to be read or sourced.
///
/// THE ELECTION IS HANDED IN RATHER THAN COMPUTED HERE, and the parameter is what
/// keeps this render a pure function of a collar. The election is a reading of a
/// repository and of a standing binary; a proclamation that took one itself would
/// be a render that touched a disk, and the door — which already holds a
/// repository — is where that reading belongs
/// (`bkce_election::bkce_elected`).
///
/// It is a DECLARED VALUE either way and never an absent field, on the collar
/// sheaf's own reasoning for `bknre_manifest`: a consumer reading an election out
/// of an ABSENCE cannot tell a declaration from an omission.
///
/// THE GEOGRAPHY IS HANDED IN FOR THE SAME REASON THE ELECTION IS. It is a
/// reading of a config directory the door face took, and a render that took one
/// itself would touch an environment — which is the one thing this side of the
/// crate does not do. It is proclaimed because the same binary in the same tree
/// is reached by more than one road and only the transcript can say which
/// answered.
pub fn bkct_proclamation(
    collar: &bkcr_Collar,
    elected: &str,
    geography: &bkca_Geography,
) -> String {
    let owned = match collar.bkcr_field("BKRR_KIND") {
        BKCR_KIND_APP => ZBKCT_PROCLAIMED_APP,
        BKCR_KIND_SUITE => ZBKCT_PROCLAIMED_SUITE,
        _ => &[],
    };

    zbkct_rendered(
        |declared| collar.bkcr_field(declared),
        ZBKCT_PROCLAIMED,
        owned,
        BKCT_LANGUAGE_RUST,
        elected,
        geography,
    )
}

/// The proclamation for one PYTHON collar, in the same vocabulary.
///
/// A SECOND ENTRY POINT AND NOT A SECOND RENDER. The two families answer through
/// different family files and therefore through different reader types, which is
/// the one thing that cannot be shared; everything downstream of the field
/// lookup is the render below, so a consumer reading either answer reads one
/// shape and the door holds no dialect of its own.
pub fn bkct_proclamation_python(
    collar: &bkcx_Collar,
    elected: &str,
    geography: &bkca_Geography,
) -> String {
    let owned = match collar.bkcx_field("BKRP_KIND") {
        BKCR_KIND_APP => ZBKCT_PROCLAIMED_PYTHON_APP,
        BKCR_KIND_SUITE => ZBKCT_PROCLAIMED_PYTHON_SUITE,
        _ => &[],
    };

    zbkct_rendered(
        |declared| collar.bkcx_field(declared),
        ZBKCT_PROCLAIMED_PYTHON,
        owned,
        BKCT_LANGUAGE_PYTHON,
        elected,
        geography,
    )
}

/// The render every family's proclamation passes through.
///
/// THE FAMILY IS THE PARAMETER AND THE VOCABULARY IS NOT, which is the seam
/// stated as code: what a tenant supplies is its two tables, its field lookup
/// and its language value, and the `BKRC_*` names, their order, the language
/// row's position and the trailing election and geography are the proclamation's
/// own and identical for every tenant that will ever join.
fn zbkct_rendered<'a, F: Fn(&str) -> &'a str>(
    field: F,
    core: &[(&str, &str)],
    owned: &[(&str, &str)],
    language: &str,
    elected: &str,
    geography: &bkca_Geography,
) -> String {
    let mut said = String::new();

    for (proclaimed, declared) in core {
        said.push_str(&zbkct_stated(proclaimed, field(declared)));
    }

    // The language is the proclamation's own field and is answered by the FAMILY
    // that declared the collar rather than by any field inside it: a rust collar
    // does not say it is rust, it says it in the file it is.
    said.push_str(&zbkct_stated("BKRC_LANGUAGE", language));

    for (proclaimed, declared) in owned {
        said.push_str(&zbkct_stated(proclaimed, field(declared)));
    }

    said.push_str(&zbkct_stated("BKRC_ELECTION", elected));
    said.push_str(&zbkct_stated("BKRC_GEOGRAPHY", &geography.bkca_stated()));

    said
}

/// One field, in the substrate's own authored layout.
///
/// A value carrying several elements is written one per line, the value opening
/// with a double quote and every line but the last closing with a SPACE and a
/// backslash. The space is load-bearing: bash removes the backslash-newline pair
/// and nothing else, so a line ending without it would join two elements into
/// one.
fn zbkct_stated(field: &str, value: &str) -> String {
    let elements: Vec<&str> = value.split_whitespace().collect();

    if elements.len() < 2 {
        return format!("{}=\"{}\"\n", field, value.trim());
    }

    let mut said = format!("{}=\"{}", field, elements[0]);
    for element in &elements[1..] {
        said.push_str(" \\\n  ");
        said.push_str(element);
    }
    said.push_str("\"\n");
    said
}

// eof
