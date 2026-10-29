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

//! The roll call names a door its installed kits no longer enroll, and lists
//! what they enroll that no door stands for.
//!
//! A receiver stands the subset of a kit's doors it chooses, so an enrolled
//! colophon with no door is a menu entry and never a fault; a door whose colophon
//! no installed kit enrolls is the fault, left behind when a kit update dropped
//! or renamed its route. This hurdle plants a small repository holding one of
//! each, drives the roll call red over it, removes the stray door and drives it
//! green.
//!
//! THE SCOPE IS PLANTED BESIDE THE FAULT. A script carrying the launcher line but
//! execing no trampoline, and a door over a kit the repository does not declare,
//! both stand in the planted tabtarget directory, so a roll call that judged
//! either would name it — and both drives assert it does not.
//!
//! THE GREEN DRIVE IS THE POSITIVE CONTROL. The same repository, the stray door
//! gone and nothing else changed, exits 0: so the red exit is the stray door's
//! and not a refusal the planted repository raised for some other reason.
//!
//! THE KITS ARE MADE-UP WORDS, matching no governed word, and the roll call is
//! driven as the workbench drives it: its CLI module executed with the command
//! word, the repository's own regime file and moorings handed over in a
//! dispatch file the hurdle composes from its own and names by the pointer.
//!
//! OBSERVED FROM OUTSIDE THE DISPATCH, as every hurdle here is.

#![deny(warnings)]

use but::buth_hurdle::buth_Bench;

/// The planted repository's root, relative to the seat.
const BUTM_PLANT: &str = "plant";

/// The stray door: it names a colophon the declared kit's zipper does not enroll.
const BUTM_STRAY: &str = "tt/quobw-x.Stray.sh";

/// A trampolined door body over the named launcher.
fn zbutm_door(launcher: &str) -> String {
    format!(
        "#!/bin/bash\n\
         set -- '{}' \"$@\"\n\
         exec \"${{BASH_SOURCE[0]%/*}}/z-launcher.sh\" \"${{0##*/}}\" \"${{@}}\"\n",
        launcher, //
    )
}

/// The files of the planted repository, relative to its root.
///
/// One declared kit, `quobk`, keeps a zipper enrolling two colophons: `quobw-s`
/// stands as a door, `quobw-a` does not. A second kit, `marfk`, is not declared.
fn zbutm_plant() -> Vec<(&'static str, String)> {
    vec![
        (
            "moorings/burc.env",
            "BURC_TABTARGET_DIR=tt\n\
             BURC_TABTARGET_DELIMITER=.\n\
             BURC_TOOLS_DIR=Tools\n\
             BURC_MANAGED_KITS=quobk\n"
                .to_string(),
        ),
        (
            "moorings/buml_launchers/launcher.quobw_workbench.sh",
            "#!/bin/bash\n\
             source \"Tools/buk/bul_launcher.sh\"\n\
             bul_launch \"${BURC_TOOLS_DIR}/quobk/quobw_workbench.sh\" \"$@\"\n"
                .to_string(),
        ),
        (
            "moorings/buml_launchers/launcher.marfw_workbench.sh",
            "#!/bin/bash\n\
             source \"Tools/buk/bul_launcher.sh\"\n\
             bul_launch \"${BURC_TOOLS_DIR}/marfk/marfw_workbench.sh\" \"$@\"\n"
                .to_string(),
        ),
        ("Tools/quobk/quobw_workbench.sh", "#!/bin/bash\n".to_string()),
        ("Tools/marfk/marfw_workbench.sh", "#!/bin/bash\n".to_string()),
        (
            "Tools/quobk/quobz_zipper.sh",
            "#!/bin/bash\n\
             test -z \"${ZQUOBZ_SOURCED:-}\" || return 0\n\
             ZQUOBZ_SOURCED=1\n\
             zquobz_kindle() {\n\
             \x20 zbuz_sentinel\n\
             \x20 buz_enroll QUOBZ_STOOD  \"quobw-s\" \"quobc_cli.sh\" \"quobc_stood\"  \"\"       \"a door that stands\"\n\
             \x20 buz_enroll QUOBZ_ABSENT \"quobw-a\" \"quobc_cli.sh\" \"quobc_absent\" \"param1\" \"a door left unstood\"\n\
             }\n"
                .to_string(),
        ),
        ("tt/quobw-s.Stood.sh", zbutm_door("launcher.quobw_workbench.sh")),
        (BUTM_STRAY, zbutm_door("launcher.quobw_workbench.sh")),
        ("tt/marfw-o.Own.sh", zbutm_door("launcher.marfw_workbench.sh")),
        (
            "tt/quobw-q.Script.sh",
            "#!/bin/bash\n\
             echo a script of the repository's own\n\
             exit\n\
             set -- 'launcher.quobw_workbench.sh' \"$@\"\n"
                .to_string(),
        ),
    ]
}

/// Drive the roll call over the planted repository, red, then again with the
/// stray door gone, each run's words and exit status landing in files.
fn zbutm_drive() -> buth_Bench {
    let bench = buth_Bench::buth_seat(
        "substrate-roll-call",
        &["buym_yelp.sh", "buc_command.sh"],
        &format!(
            "cd \"${{BURD_CONFIG_DIR}}/../{plant}\"\n\
             z_burd=\"${{BURD_TEMP_DIR}}/butm-burd.env\"\n\
             grep -v -E '^BURD_(REGIME_FILE|CONFIG_DIR|BURD_FILE)=' \"${{BURD_BURD_FILE}}\" > \"${{z_burd}}\"\n\
             printf '%s\\n' \"BURD_REGIME_FILE=\\\"${{PWD}}/moorings/burc.env\\\"\" \\\n\
             \x20 \"BURD_CONFIG_DIR=\\\"${{PWD}}/moorings\\\"\" \"BURD_BURD_FILE=\\\"${{z_burd}}\\\"\" >> \"${{z_burd}}\"\n\
             zbutm_call() {{\n\
             \x20 env BUUR_ALSO_KITS=\"${{z_also:-}}\" BURD_BURD_FILE=\"${{z_burd}}\" \\\n\
             \x20     \"${{BURD_BUK_DIR}}/buur_cli.sh\" buur_roll_call\n\
             }}\n\
             z_status=0\n\
             zbutm_call > ../red.txt 2>&1 || z_status=$?\n\
             printf '%s\\n' \"${{z_status}}\" > ../red-status.txt\n\
             rm {stray}\n\
             z_status=0\n\
             zbutm_call > ../green.txt 2>&1 || z_status=$?\n\
             printf '%s\\n' \"${{z_status}}\" > ../green-status.txt\n\
             z_also=marfk\n\
             z_status=0\n\
             zbutm_call > ../also.txt 2>&1 || z_status=$?\n\
             printf '%s\\n' \"${{z_status}}\" > ../also-status.txt\n",
            plant = BUTM_PLANT,
            stray = BUTM_STRAY, //
        ),
    );

    let root = bench.buth_root().join(BUTM_PLANT);
    for (relative, body) in zbutm_plant() {
        let path = root.join(relative);
        std::fs::create_dir_all(path.parent().expect("a planted file stands in a directory"))
            .expect("the seat takes the planted directories");
        std::fs::write(&path, body).expect("the seat takes the planted file");
    }

    bench.buth_drive(&[]).buth_thrived();
    bench
}

#[test]
fn butm_the_roll_call_names_the_stray_and_lists_the_unstood() {
    let bench = zbutm_drive();

    let red = bench.buth_read("red.txt");
    assert_ne!(
        bench.buth_read("red-status.txt").trim(),
        "0",
        "the roll call exits red while a stray door stands:\n{}",
        red, //
    );
    assert!(
        red.contains("tt/quobw-x.Stray.sh names quobw-x, which no zipper of kit quobk enrolls"),
        "the red run names the stray door and its colophon:\n{}",
        red, //
    );
    assert!(
        red.contains("quobw-a  [takes one argument]  a door left unstood"),
        "the red run lists the enrolled colophon no door stands for, with its folio and purpose:\n{}",
        red, //
    );
    for unjudged in ["quobw-s", "marfw-o", "quobw-q"] {
        assert!(
            !red.contains(&format!(
                "tt/{}.",
                unjudged, //
            )),
            "the red run names only the stray, never {}:\n{}",
            unjudged,
            red, //
        );
    }

    // THE POSITIVE CONTROL: the stray gone, the same repository reads green and
    // still lists the unstood colophon.
    let green = bench.buth_read("green.txt");
    assert_eq!(
        bench.buth_read("green-status.txt").trim(),
        "0",
        "the roll call exits 0 once the stray door is gone:\n{}",
        green, //
    );
    assert!(
        green.contains("quobw-a  [takes one argument]  a door left unstood"),
        "the green run still lists the unstood colophon:\n{}",
        green, //
    );
    assert!(
        !green.contains("Unenrolled"),
        "the green run names nothing unenrolled:\n{}",
        green, //
    );

    // A FURTHER KIT: marfk named beside the declaration, its door is judged as a
    // declared kit's would be. The green run above is this arm's control, where
    // the same door stood unjudged.
    let also = bench.buth_read("also.txt");
    assert_ne!(
        bench.buth_read("also-status.txt").trim(),
        "0",
        "naming marfk as a further kit judges its door, which no zipper enrolls:\n{}",
        also, //
    );
    assert!(
        also.contains("tt/marfw-o.Own.sh names marfw-o, which no zipper of kit marfk enrolls"),
        "the further kit's door is named:\n{}",
        also, //
    );
    assert!(
        also.contains("kits quobk,marfk"),
        "the roll call names the further kit beside the declared one:\n{}",
        also, //
    );
}

// eof
