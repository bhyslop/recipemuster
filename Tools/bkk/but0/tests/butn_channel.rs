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

//! Each channel hands the command what its enrollment promises, and no more.
//!
//! A channel is how a colophon's command receives its folio, decoded once at the
//! workbench by `buz_exec_lookup`. The passthrough channel is the one that hands
//! over no folio at all and every argument as typed — the channel a door takes
//! when it forwards its arguments to a program of its own. That promise is
//! exactly where a careless decode goes wrong without a sound: an argument
//! carrying a space split in two, an empty argument dropped, the first argument
//! eaten as though it were a folio, and the command still runs.
//!
//! SO THE PLANTED ARGUMENTS ARE THE HARD ONES — a spaced one, an empty one, and
//! one plain — and each channel's drive is read back one argument per line, in
//! order, from a probe that prints what it was given.
//!
//! THE STANDING CHANNELS RIDE THE SAME DRIVE, because the new arm stands in the
//! same case as theirs. The param1 drive is the positive control: it sees the
//! first argument consumed into the folio, so a probe that printed nothing, or a
//! decode that never ran, could not pass the passthrough assertion by accident.
//!
//! OBSERVED FROM OUTSIDE THE DISPATCH, as every hurdle here is.

#![deny(warnings)]

use but::buth_hurdle::buth_Bench;

/// The modules the lookup needs, and no more.
const BUTN_MODULES: &[&str] = &[
    "buym_yelp.sh",
    "buc_command.sh",
    "buz_zipper.sh", //
];

/// Where the probe stands, relative to the seat; the lookup's base directory.
const BUTN_PROBE_DIR: &str = "zorvit";

/// The probe's file, the module every enrollment below names.
const BUTN_PROBE_FILE: &str = "butn_probe.sh";

/// The probe: it prints the folio, the command it was asked for, and each
/// argument on a line of its own, bracketed so an empty one still shows.
const BUTN_PROBE: &str = "#!/bin/bash\n\
                          printf 'folio=[%s]\\n' \"${BUZ_FOLIO}\"\n\
                          printf 'command=[%s]\\n' \"${1}\"\n\
                          shift\n\
                          for z_arg in \"$@\"; do printf 'arg=[%s]\\n' \"${z_arg}\"; done\n";

/// The arguments every drive is handed: a spaced one, an empty one, a plain one.
const BUTN_ARGS: &[&str] = &[
    "first word",
    "",
    "third", //
];

/// Enroll the probe once per channel and drive each through the lookup, every
/// drive in an isolation subshell so the lookup's exec ends the subshell rather
/// than the coordinator, its output landing in a file named for the channel.
fn zbutn_drive() -> buth_Bench {
    let bench = buth_Bench::buth_seat(
        "substrate-channel-decode",
        BUTN_MODULES,
        &format!(
            "zbuz_kindle\n\
             shift\n\
             z_root=\"${{BURD_CONFIG_DIR}}/..\"\n\
             chmod +x \"${{z_root}}/{dir}/{file}\"\n\
             buz_enroll BUTN_THROUGH butn-t {file} butn_probe buz_passthrough \"probe, passthrough\"\n\
             buz_enroll BUTN_PARAM   butn-p {file} butn_probe param1          \"probe, param1\"\n\
             buz_enroll BUTN_IMPRINT butn-i {file} butn_probe imprint         \"probe, imprint\"\n\
             buz_enroll BUTN_NONE    butn-n {file} butn_probe \"\"              \"probe, no channel\"\n\
             ( buz_exec_lookup butn-t \"${{z_root}}/{dir}\" \"$@\" ) > \"${{z_root}}/through.txt\"\n\
             ( buz_exec_lookup butn-p \"${{z_root}}/{dir}\" \"$@\" ) > \"${{z_root}}/param.txt\"\n\
             ( buz_exec_lookup butn-i \"${{z_root}}/{dir}\" \"$@\" ) > \"${{z_root}}/imprint.txt\"\n\
             ( buz_exec_lookup butn-n \"${{z_root}}/{dir}\" \"$@\" ) > \"${{z_root}}/none.txt\"\n\
             printf 'folio=[%s]\\n' \"${{BURD_TOKEN_3}}\" > \"${{z_root}}/token.txt\"\n",
            dir = BUTN_PROBE_DIR,
            file = BUTN_PROBE_FILE, //
        ),
    );

    let probe_dir = bench.buth_root().join(BUTN_PROBE_DIR);
    std::fs::create_dir_all(&probe_dir).expect("the seat takes a probe directory");
    std::fs::write(probe_dir.join(BUTN_PROBE_FILE), BUTN_PROBE).expect("the seat takes the probe");

    bench.buth_drive(BUTN_ARGS).buth_thrived();
    bench
}

#[test]
fn butn_each_channel_hands_the_command_what_it_promises() {
    let bench = zbutn_drive();

    assert_eq!(
        bench.buth_read("through.txt"),
        "folio=[]\n\
         command=[butn_probe]\n\
         arg=[first word]\n\
         arg=[]\n\
         arg=[third]\n",
        "the passthrough channel hands over no folio and every argument as typed, in order", //
    );

    // THE POSITIVE CONTROL: the same drive over param1 sees the first argument
    // consumed into the folio and the rest handed over behind it.
    assert_eq!(
        bench.buth_read("param.txt"),
        "folio=[first word]\n\
         command=[butn_probe]\n\
         arg=[]\n\
         arg=[third]\n",
        "the param1 channel takes the first argument as its folio", //
    );

    // The imprint channel takes its folio from the tabtarget's name and leaves
    // the arguments alone; the coordinator recorded the token it was handed.
    let token = bench.buth_read("token.txt");
    assert_eq!(
        bench.buth_read("imprint.txt"),
        format!(
            "{}command=[butn_probe]\n\
             arg=[first word]\n\
             arg=[]\n\
             arg=[third]\n",
            token, //
        ),
        "the imprint channel takes the name's third token and passes every argument", //
    );

    assert_eq!(
        bench.buth_read("none.txt"),
        "folio=[]\n\
         command=[butn_probe]\n",
        "the empty channel drops every argument", //
    );
}

// eof
