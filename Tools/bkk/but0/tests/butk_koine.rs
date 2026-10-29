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

//! A regime handed to docker compose carries no `${` reference in any value.
//!
//! THE CARRIER IS READ, NOT THE VALUE, and these hurdles are shaped to prove it.
//! Bash expands a reference when it sources the carrier, so by the time enforce
//! reads `BUTK_BRAVO` it holds `bare-reference` and nothing of the `${` that
//! compose will meet in the same file. A check over the sourced value would pass
//! every carrier these hurdles plant; only a check over the carrier's own lines
//! can refuse the first one.
//!
//! THE TWO POSITIVES ARE THE CONTROLS, and each answers a different wrong check.
//! A koine regime with no reference passing says the declaration alone refuses
//! nothing; a regime that never declared itself passing with the very reference
//! the first hurdle refuses says the check reads the declaration and not the
//! reference alone. The comment hurdle says a line neither reader decodes is not
//! a value.

#![deny(warnings)]

use but::buth_hurdle::buth_Bench;

/// The modules a regime reading needs, in the substrate's own order.
/// The validation module and the constants beside it are the command module's
/// to load, the seat's dispatch handing its coordinator the pointer.
const BUTK_MODULES: &[&str] = &[
    "buym_yelp.sh",
    "buc_command.sh", //
];

/// What the coordinator says once enforce has passed.
const BUTK_ENFORCED: &str = "butk regime enforced";

/// The carrier lines the reference hurdles plant: a bare token, then a value
/// built from it by a reference.
const BUTK_REFERENCED: &[&str] = &[
    "BUTK_ALPHA=bare",
    "BUTK_BRAVO=\"${BUTK_ALPHA}-reference\"", //
];

/// Drive one regime reading over a carrier of the given lines: write the carrier,
/// source it as a launcher would, enroll the two fields, then vet.
///
/// THE LINES ARE WRITTEN SINGLE-QUOTED so the coordinator's own shell expands
/// nothing on the way to the file; the carrier holds exactly the bytes a hurdle
/// names.
fn butk_regime(name: &str, koine: bool, lines: &[&str]) -> but::buth_hurdle::buth_Said {
    let mut body = String::from(
        "for z_v in ${!BUTK_@}; do unset \"${z_v}\"; done\n\
         buc_context \"butk\"\n\
         z_carrier=\"${BURD_TEMP_DIR}/butk.env\"\n\
         printf '%s\\n'",
    );
    for line in lines {
        body.push_str(&format!(
            " '{}'",
            line, //
        ));
    }
    body.push_str(" > \"${z_carrier}\"\n");
    body.push_str(
        "source \"${z_carrier}\"\n\
         buv_regime_enroll BUTK\n",
    );
    if koine {
        body.push_str("buv_koine_enroll \"${z_carrier}\"\n");
    }
    body.push_str(&format!(
        "buv_string_enroll BUTK_ALPHA 1 64 \"first field\"\n\
         buv_string_enroll BUTK_BRAVO 1 64 \"second field\"\n\
         buv_scope_sentinel BUTK BUTK_\n\
         buv_vet BUTK\n\
         echo \"{}\"\n",
        BUTK_ENFORCED, //
    ));

    buth_Bench::buth_seat(name, BUTK_MODULES, &body).buth_drive(&[])
}

#[test]
fn butk_a_compose_regime_carrying_a_reference_is_refused_naming_the_field() {
    // THE FIELD IS ASSERTED BY NAME, and the field that carries no reference is
    // asserted absent: a refusal naming every field would satisfy a bare
    // death-check while telling the operator nothing about which line to repair.
    butk_regime("substrate-koine-reference", true, BUTK_REFERENCED)
        .buth_died()
        .buth_carries("reference in: BUTK_BRAVO")
        .buth_lacks("BUTK_ALPHA BUTK_BRAVO")
        .buth_lacks(BUTK_ENFORCED);
}

#[test]
fn butk_a_compose_regime_carrying_no_reference_is_admitted() {
    butk_regime(
        "substrate-koine-plain",
        true,
        &["BUTK_ALPHA=bare", "BUTK_BRAVO=\"quoted free text\""],
    )
    .buth_thrived()
    .buth_carries(BUTK_ENFORCED);
}

#[test]
fn butk_a_regime_not_handed_to_compose_may_carry_a_reference() {
    butk_regime("substrate-koine-undeclared", false, BUTK_REFERENCED)
        .buth_thrived()
        .buth_carries(BUTK_ENFORCED);
}

#[test]
fn butk_a_reference_in_a_comment_line_is_no_value() {
    butk_regime(
        "substrate-koine-comment",
        true,
        &[
            "# BUTK_BRAVO was once ${BUTK_ALPHA}-reference",
            "BUTK_ALPHA=bare",
            "BUTK_BRAVO=plain",
        ],
    )
    .buth_thrived()
    .buth_carries(BUTK_ENFORCED);
}
// eof
