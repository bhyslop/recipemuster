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

//! The dispatch reads the operator's hyperlink preference and writes its own verdict.
//!
//! The sibling of the colour hurdles: `BURE_HYPERLINKS` is the operator's optional
//! say and is never written; the resolved `0`/`1` lands under `BURD_HYPERLINKS`, and
//! the yelp palette reads that verdict. The probe drives both halves in one child,
//! so a dispatched tabtarget's route from the operator's environment to the
//! plain-text fallback is proven end to end rather than half at a time.

#![deny(warnings)]

use but::buth_hurdle::buth_Bench;

/// The dispatch spine is entered by path in a process of its own, as the colour
/// hurdles enter it.
const BUTC_LINKS_MODULES: &[&str] = &[];

/// The token the probe prints its reading under.
const BUTC_LINKS_SAID: &str = "butc_hyperlinks=";

/// Resolve in a fresh process under a controlled environment, then kindle the
/// palette under unconditional colour, and report
/// `<BURD_HYPERLINKS>|<palette's hyperlink mode>|<BURE_HYPERLINKS or UNSET>`.
fn butc_hyperlinks_resolve(name: &str, env: &[&str]) -> but::buth_hurdle::buth_Said {
    let mut probe = String::from("env -u BURD_HYPERLINKS");
    for word in env {
        probe.push(' ');
        probe.push_str(word);
    }
    probe.push_str(
        " bash -c 'source \"${BURD_BUK_DIR}/bud_dispatch.sh\"\n\
         zbud_resolve_hyperlinks\n\
         source \"${BURD_BUK_DIR}/buym_yelp.sh\"\n\
         zbuym_kindle\n\
         buym_unconditional\n\
         printf \"",
    );
    probe.push_str(BUTC_LINKS_SAID);
    probe.push_str(
        "%s|%s|%s\\n\" \"${BURD_HYPERLINKS}\" \"${z_buym_use_hyperlinks}\" \"${BURE_HYPERLINKS:-UNSET}\"'\n",
    );

    buth_Bench::buth_seat(name, BUTC_LINKS_MODULES, &probe).buth_drive(&[])
}

#[test]
fn butc_an_operator_preference_against_hyperlinks_reaches_the_palette_fallback() {
    // THE FALLBACK IS REACHABLE FROM THE OPERATOR'S ENVIRONMENT: the operator
    // sets only the BURE_ input, and the palette ends up rendering no escape.
    // The input survives the resolve, unwritten.
    butc_hyperlinks_resolve("substrate-hyperlinks-off", &["BURE_HYPERLINKS=0"])
        .buth_thrived()
        .buth_carries(&format!(
            "{}0|0|0",
            BUTC_LINKS_SAID, //
        ));
}

#[test]
fn butc_an_absent_preference_resolves_lit_and_is_never_invented() {
    // THE PAIR IS EACH OTHER'S CONTROL: a resolver hard-wired dark would pass the
    // test above and fail this one, and the input stays unset behind the verdict.
    butc_hyperlinks_resolve("substrate-hyperlinks-auto", &["-u", "BURE_HYPERLINKS"])
        .buth_thrived()
        .buth_carries(&format!(
            "{}1|1|UNSET",
            BUTC_LINKS_SAID, //
        ));
}

#[test]
fn butc_an_explicit_preference_for_hyperlinks_resolves_lit() {
    butc_hyperlinks_resolve("substrate-hyperlinks-on", &["BURE_HYPERLINKS=1"])
        .buth_thrived()
        .buth_carries(&format!(
            "{}1|1|1",
            BUTC_LINKS_SAID, //
        ));
}

#[test]
fn butc_a_value_outside_the_enum_is_refused() {
    butc_hyperlinks_resolve("substrate-hyperlinks-bad", &["BURE_HYPERLINKS=maybe"])
        .buth_carries("BURE_HYPERLINKS must be 0 or 1");
}
// eof
