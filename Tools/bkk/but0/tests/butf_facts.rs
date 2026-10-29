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

//! Facts travel from one dispatch to the next, and reading one consumes nothing.
//!
//! Ported from the bash bench's `fact-chaining` fixture, nine cases, all nine
//! carried.
//!
//! THIS FIXTURE IS WHY THE PACE'S NO-AMBIENT-STATE CINCH HAS A SPECIMEN. Its
//! bash header records the trap plainly: `BURD_PREVIOUS_DIR` and
//! `BURD_OUTPUT_DIR` are locked readonly by the dispatch regime, so the cases
//! could not point them at scratch and instead seeded uniquely-named files into
//! the LIVE directories of the run that was executing them — asserting around
//! whatever else those directories happened to hold, and leaving their own
//! seedings behind for whatever came next. Every hurdle here owns the directories
//! it seeds, because its seat composed them; the names need no uniqueness, the
//! reach needs no negotiation, and nothing survives the hurdle that made it.
//!
//! THE COORDINATOR SEEDS AND REPORTS; THIS PROCESS JUDGES. Where a case reads a
//! value back, the coordinator prints what it read and the hurdle compares;
//! where a case proves a refusal, the read is raised bare and the hurdle reads
//! the death from outside. Where a case proves a file SURVIVED, this process
//! opens it — which is the reading the bash bench could not take at all, its
//! observer standing in the same directories it was asking about.

#![deny(warnings)]
#![allow(non_camel_case_types)]

use but::buth_hurdle::buth_Bench;

/// The modules a chaining reading needs.
const BUTF_MODULES: &[&str] = &[
    "buym_yelp.sh",
    "buc_command.sh",
    "buf_fact.sh", //
];

/// The tokens the coordinator reports under.
const BUTF_WHERE: &str = "butf_output=";
const BUTF_PRIOR: &str = "butf_previous=";
const BUTF_VALUE: &str = "butf_value=";

/// The prelude every coordinator opens with: make the prior dispatch's directory
/// stand, and say where both directories are.
///
/// THE DIRECTORY IS CREATED RATHER THAN ASSUMED. A seat's first dispatch has no
/// predecessor, so nothing has promoted a `current` into a `previous` yet; the
/// bash cases carried the same `mkdir` for the same reason, and it stayed inline
/// there because the readonly path could not be lifted to a setup step.
const BUTF_PRELUDE: &str = "mkdir -p \"${BURD_PREVIOUS_DIR}\"\n\
                            printf 'butf_output=%s\\n'   \"${BURD_OUTPUT_DIR}\"\n\
                            printf 'butf_previous=%s\\n' \"${BURD_PREVIOUS_DIR}\"\n";

/// Where a drive's two fact directories stood.
struct butf_Where {
    output: String,
    previous: String,
}

/// Drive a coordinator that reports its fact directories and then does the work.
///
/// The bench is handed back because a hurdle reading the seat's own files must
/// outlive the drive — the burx fixture paid a red drive to learn it.
fn butf_drive(name: &str, work: &str) -> (buth_Bench, but::buth_hurdle::buth_Said, butf_Where) {
    let bench = buth_Bench::buth_seat(
        name,
        BUTF_MODULES,
        &format!(
            "{prelude}{work}",
            prelude = BUTF_PRELUDE,
            work = work, //
        ),
    );
    let said = bench.buth_drive(&[]);
    let where_ = butf_Where {
        output: butf_after(&said.buth_text, BUTF_WHERE),
        previous: butf_after(&said.buth_text, BUTF_PRIOR),
    };
    (bench, said, where_)
}

/// The rest of the line a reported token opens.
fn butf_after(text: &str, token: &str) -> String {
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

/// A shell line seeding one fact under a named directory.
///
/// THE TRAILING NEWLINE IS THE PRODUCER'S OWN FORMAT, written deliberately so
/// the read path exercises the stripping it owes rather than being handed
/// something already bare.
fn butf_seed(dir: &str, name: &str, value: &str) -> String {
    format!(
        "printf '%s\\n' '{}' > \"${{{}}}/{}\"\n",
        value,
        dir,
        name, //
    )
}

/// Read a fact this process can see, naming it when it is absent.
fn butf_read(dir: &str, name: &str) -> String {
    let path = std::path::Path::new(dir).join(name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| {
            panic!(
                "no {} stands: {}",
                path.display(),
                err, //
            )
        })
        .trim_end()
        .to_string()
}

#[test]
fn butf_a_relay_forwards_a_prior_fact_into_the_current_dispatch() {
    let (_seat, said, at) = butf_drive(
        "substrate-fact-forward",
        &format!(
            "{seed}buf_relay\n",
            seed = butf_seed("BURD_PREVIOUS_DIR", "butf_fwd", "forwarded-value"), //
        ),
    );
    said.buth_thrived();
    assert_eq!(butf_read(&at.output, "butf_fwd"), "forwarded-value");
}

#[test]
fn butf_a_relay_never_clobbers_a_fact_the_current_dispatch_already_holds() {
    // THE CURRENT DISPATCH OUTRANKS ITS PREDECESSOR, always. A relay that
    // overwrote would let a stale value from the last run silently replace one
    // this run computed — the failure being that both are well-formed facts under
    // the same name, so nothing downstream could tell which it got.
    let (_seat, said, at) = butf_drive(
        "substrate-fact-preserve",
        &format!(
            "{prev}{curr}buf_relay\n",
            prev = butf_seed("BURD_PREVIOUS_DIR", "butf_pres", "from-previous"),
            curr = butf_seed("BURD_OUTPUT_DIR", "butf_pres", "from-current"), //
        ),
    );
    said.buth_thrived();
    assert_eq!(butf_read(&at.output, "butf_pres"), "from-current");
}

#[test]
fn butf_a_second_relay_is_a_no_op_that_still_succeeds() {
    // IDEMPOTENCE IS ASSERTED ON BOTH AXES: the second call must not fail, and it
    // must not alter what the first forwarded. A relay refusing on its second
    // call would make the order of a coordinator's own steps load-bearing.
    let (_seat, said, at) = butf_drive(
        "substrate-fact-idempotent",
        &format!(
            "{seed}buf_relay\nbuf_relay\n",
            seed = butf_seed("BURD_PREVIOUS_DIR", "butf_idem", "v1"), //
        ),
    );
    said.buth_thrived();
    assert_eq!(butf_read(&at.output, "butf_idem"), "v1");
}

#[test]
fn butf_a_read_emits_the_bare_value_with_the_newline_stripped() {
    let (_seat, said, _) = butf_drive(
        "substrate-fact-read",
        &format!(
            "{seed}printf 'butf_value=[%s]\\n' \"$(buf_read_fact_capture butf_greeting)\"\n",
            seed = butf_seed("BURD_PREVIOUS_DIR", "butf_greeting", "hello world"), //
        ),
    );
    // THE BRACKETS ARE THE ASSERTION'S TEETH. The value is compared inside
    // delimiters so a trailing newline the reader failed to strip shows up as a
    // mismatch rather than vanishing into the line break it would have become.
    said.buth_thrived().buth_carries(&format!(
        "{}[hello world]",
        BUTF_VALUE, //
    ));
}

#[test]
fn butf_a_read_of_an_absent_fact_fails_hard() {
    // SILENCE WOULD BE THE WORST ANSWER HERE. A reader returning empty for an
    // absent fact hands its caller a value indistinguishable from a fact that was
    // genuinely written empty. The capture refuses with its status alone, as its
    // class does, and the caller's guard names the fact; the value line after it
    // is the tripwire a read that wrongly succeeded would reach.
    let (_seat, said, _) = butf_drive(
        "substrate-fact-absent",
        "z_fact=$(buf_read_fact_capture butf_definitely_absent) || buc_die_now \"absent: butf_definitely_absent\"\n\
         printf 'butf_value=[%s]\\n' \"${z_fact}\"\n",
    );
    said.buth_died()
        .buth_carries("butf_definitely_absent")
        .buth_lacks(BUTF_VALUE);
}

#[test]
fn butf_a_non_empty_express_value_wins_and_the_chain_is_never_read() {
    // THE ABSENT FACT IS THE INSTRUMENT. It is named deliberately and never
    // seeded, so an elect that consulted the chain despite holding an express
    // value would die on the read — which turns "the express won" from something
    // asserted about a returned string into something the drive's survival
    // proves.
    let (_seat, said, _) = butf_drive(
        "substrate-fact-express",
        "printf 'butf_value=[%s]\\n' \
         \"$(buf_elect_fact_capture 'express-wins' butf_definitely_absent)\"\n",
    );
    said.buth_thrived().buth_carries(&format!(
        "{}[express-wins]",
        BUTF_VALUE, //
    ));
}

#[test]
fn butf_an_elect_still_reads_the_prior_dispatch_after_a_relay() {
    // A RELAY COPIES FORWARD AND MOVES NOTHING, which is what makes both halves
    // true at once: the elect still finds the baton where it was written, and the
    // forwarded copy stands in `current` ready to become the next run's
    // `previous`.
    let (_seat, said, at) = butf_drive(
        "substrate-fact-elect-relay",
        &format!(
            "{seed}buf_relay\n\
             printf 'butf_value=[%s]\\n' \"$(buf_elect_fact_capture '' butf_baton)\"\n",
            seed = butf_seed("BURD_PREVIOUS_DIR", "butf_baton", "baton-value"), //
        ),
    );
    said.buth_thrived().buth_carries(&format!(
        "{}[baton-value]",
        BUTF_VALUE, //
    ));
    assert_eq!(butf_read(&at.output, "butf_baton"), "baton-value");
}

#[test]
fn butf_reading_a_fact_consumes_nothing() {
    // BOTH GENERATIONS SURVIVE REPEATED READS, and this process opens both files
    // to say so — the reading the bash bench structurally could not take, its
    // observer standing inside the very directories it was asking about.
    let (_seat, said, at) = butf_drive(
        "substrate-fact-survives",
        &format!(
            "{seed}buf_relay\n\
             printf 'butf_value=[%s]\\n' \"$(buf_read_fact_capture butf_survive)\"\n\
             printf 'butf_value=[%s]\\n' \"$(buf_read_fact_capture butf_survive)\"\n",
            seed = butf_seed("BURD_PREVIOUS_DIR", "butf_survive", "immortal"), //
        ),
    );
    said.buth_thrived();

    assert_eq!(
        said.buth_tally(&format!(
            "{}[immortal]",
            BUTF_VALUE, //
        )),
        2,
        "the fact did not read the same twice:\n{}",
        said.buth_text, //
    );
    assert_eq!(butf_read(&at.previous, "butf_survive"), "immortal");
    assert_eq!(butf_read(&at.output, "butf_survive"), "immortal");
}

#[test]
fn butf_an_empty_express_falls_back_to_the_chain_and_a_broken_chain_dies() {
    // ONE COORDINATOR CARRIES BOTH HALVES because they are one rule read in both
    // directions: an empty express means "go and look", and going to look at
    // something absent is the broken chain. The successful fallback is reported
    // before the broken one is raised, so a drive that died at the wrong step
    // says which step it reached.
    let (_seat, said, _) = butf_drive(
        "substrate-fact-chain",
        &format!(
            "{seed}printf 'butf_value=[%s]\\n' \"$(buf_elect_fact_capture '' butf_chained)\"\n\
             z_fact=$(buf_elect_fact_capture '' butf_definitely_absent) || buc_die_now \"absent: butf_definitely_absent\"\n\
             printf 'butf_value=[%s]\\n' \"${{z_fact}}\"\n",
            seed = butf_seed("BURD_PREVIOUS_DIR", "butf_chained", "from-chain"), //
        ),
    );
    said.buth_died()
        .buth_carries(&format!(
            "{}[from-chain]",
            BUTF_VALUE, //
        ))
        .buth_carries("butf_definitely_absent");
    assert_eq!(
        said.buth_tally(BUTF_VALUE),
        1,
        "the broken chain reached its value line:\n{}",
        said.buth_text, //
    );
}
// eof
