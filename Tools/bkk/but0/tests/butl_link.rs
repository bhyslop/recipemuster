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

//! `buh_link` renders a hyperlink, and degrades to a readable URL where it cannot.
//!
//! Ported from the bash bench's `buh-link` fixture, three cases, all three
//! carried. The subject is unchanged; what moved is the observer. The bash cases
//! read the rendered bytes out of a capture the bench took inside its own
//! process; these read them out of a child this process spawned.

#![deny(warnings)]

use but::buth_hurdle::buth_Bench;
use but::buth_hurdle::BUTH_ESC;
use but::buth_hurdle::BUTH_OSC8;

/// The modules a link render needs: the yelp module the handbook resolves
/// through, and the handbook itself.
const BUTL_MODULES: &[&str] = &[
    "buym_yelp.sh",
    "buc_command.sh",
    "buh_handbook.sh", //
];

/// The URL every case links to, and the fragment that makes each anchor its own.
const BUTL_URL: &str = "https://example.com#hallmark";

#[test]
fn butl_a_link_renders_as_an_osc8_hyperlink() {
    buth_Bench::buth_seat(
        "substrate-link-osc8",
        BUTL_MODULES,
        &format!(
            "buym_unconditional\nbuh_link \"A \" \"hallmark\" \"{url}\" \" is a named artifact.\"\n",
            url = BUTL_URL, //
        ),
    )
    .buth_drive(&[])
    .buth_thrived()
    .buth_carries(BUTH_OSC8);
}

#[test]
fn butl_the_operators_fallback_falls_back_to_an_angle_bracket_url() {
    let said = buth_Bench::buth_seat(
        "substrate-link-fallback",
        BUTL_MODULES,
        &format!(
            "buym_unconditional\n\
             buh_link \"A \" \"hallmark\" \"{url}\" \" is a named artifact.\"\n",
            url = BUTL_URL, //
        ),
    )
    .buth_drive_stating(&[], &[("BURE_HYPERLINKS", "0")]);

    // BOTH HALVES ARE ASSERTED, because either alone is satisfiable by a
    // rendering nobody wants: the URL can stand in angle brackets INSIDE an
    // OSC-8 sequence, and the absence of OSC-8 says nothing about whether the
    // URL reached the reader at all.
    said.buth_thrived()
        .buth_carries(&format!(
            "<{}>",
            BUTL_URL, //
        ))
        .buth_lacks(BUTH_OSC8);
}

#[test]
fn butl_plain_mode_keeps_the_url_and_carries_no_escape() {
    // PLAIN IS THE THIRD MODE, and the one where the URL is the only thing left
    // that can reach the reader: no hyperlink escape and no style either.
    buth_Bench::buth_seat(
        "substrate-link-plain",
        BUTL_MODULES,
        &format!(
            "buym_plain\nbuh_link \"A \" \"hallmark\" \"{url}\" \" is a named artifact.\"\n",
            url = BUTL_URL, //
        ),
    )
    .buth_drive(&[])
    .buth_thrived()
    .buth_carries(&format!(
        "hallmark <{}>",
        BUTL_URL, //
    ))
    .buth_lacks(BUTH_ESC);
}

#[test]
fn butl_buc_link_renders_all_three_modes() {
    // `buc_link` shares the yelp module's one decision with `buh_link`: hyperlinks
    // on carries the escape, the operator's fallback and plain both keep the URL.
    let drive = |name: &str, setup: &str, stated: &[(&str, &str)]| {
        buth_Bench::buth_seat(
            name,
            BUTL_MODULES,
            &format!(
                "{setup}buc_link \"See\" \"hallmark\" \"{url}\"\n",
                setup = setup,
                url = BUTL_URL, //
            ),
        )
        .buth_drive_stating(&[], stated)
    };

    drive("substrate-buclink-osc8", "buym_unconditional\n", &[])
        .buth_thrived()
        .buth_carries(BUTH_OSC8);
    drive(
        "substrate-buclink-fallback",
        "buym_unconditional\n",
        &[("BURE_HYPERLINKS", "0")],
    )
    .buth_thrived()
    .buth_carries(&format!(
        "<{}>",
        BUTL_URL, //
    ))
    .buth_lacks(BUTH_OSC8);
    drive("substrate-buclink-plain", "buym_plain\n", &[])
        .buth_thrived()
        .buth_carries(&format!(
            "hallmark <{}>",
            BUTL_URL, //
        ))
        .buth_lacks(BUTH_ESC);
}

#[test]
fn butl_a_link_whose_text_is_its_url_shows_it_once() {
    // THE TEXT-EQUALS-URL EDGE: a fallback that always appended the URL would
    // print it twice where the link text already is the URL.
    let said = buth_Bench::buth_seat(
        "substrate-link-text-is-url",
        BUTL_MODULES,
        &format!(
            "buym_plain\nbuh_link \"Open \" \"{url}\" \"{url}\" \" to continue.\"\n",
            url = BUTL_URL, //
        ),
    )
    .buth_drive(&[]);
    said.buth_thrived().buth_carries(&format!(
        "Open {} to continue.",
        BUTL_URL, //
    ));
}

#[test]
fn butl_every_argument_shape_renders() {
    // THE THREE SHAPES ARE THE ARITY'S OWN EDGES: an empty leading phrase, an
    // empty trailing one, and both filled. `buh_link` takes four arguments
    // always, so a shape emptying one of them is where a body indexing its
    // arguments by position would fail.
    buth_Bench::buth_seat(
        "substrate-link-variants",
        BUTL_MODULES,
        "buh_link \"\" \"click here\" \"https://example.com\" \" for details\"\n\
         buh_link \"See \" \"docs\" \"https://example.com/docs\" \"\"\n\
         buh_link \"A \" \"vessel\" \"https://example.com#vessel\" \" is a container image.\"\n",
    )
    .buth_drive(&[])
    .buth_thrived()
    .buth_carries("click here")
    .buth_carries("docs")
    .buth_carries("vessel");
}
// eof
