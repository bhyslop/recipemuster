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

//! The configuration line and the dispatch file, driven through doors this
//! hurdle composes in its own seat.
//!
//! A tabtarget says its configuration as an argument and exports nothing; the
//! trampoline finds the launcher stub by the configuration's first word and
//! hands the launcher the configuration, the seat and the moorings directory;
//! the launcher tests that contract before it sources anything; and dispatch
//! writes its regime once to a dispatch file whose pointer, beside the kit
//! directory, is all that crosses into the coordinator. Each refusal here is
//! asserted by its code and by the canonical line it names, because a refusal a
//! reader cannot act on is half a refusal.
//!
//! EVERY DOOR IS COMPOSED IN THE SEAT, never committed for a test: the seat's own
//! tabtarget and trampoline are the substrate seat's, and each hurdle lays the
//! further stubs its case needs beside them.
//!
//! OBSERVED FROM OUTSIDE THE DISPATCH, as every hurdle here is.

#![deny(warnings)]
#![allow(non_camel_case_types)]

use std::path::Path;
use std::path::PathBuf;

use bkk::bktu_lure::bktu_Lure;
use bkp::bkprs_scrivener::bkprs_program_write;

/// The trampoline's canonical configuration line, as its refusals name it.
const BUTJ_CANONICAL: &str = "set -- 'launcher.<id>_workbench.sh[ buuc_…]' \"$@\"";

/// The configuration words the launcher answers to, each spelled once here:
/// this hurdle is a reader answering the spellings the create doors teach, and
/// what it proves is that the launcher admits these three and refuses any other.
const BUTJ_WORD_INTERACTIVE: &str = "buuc_interactive";
const BUTJ_WORD_NOLOG: &str = "buuc_nolog";
const BUTJ_WORD_OWNLOG: &str = "buuc_ownlog";

/// The exec line every stub of the z-launcher family carries.
const BUTJ_EXEC: &str = "exec \"${BASH_SOURCE[0]%/*}/z-launcher.sh\" \"${0##*/}\" \"${@}\"";

/// The seat's own launcher stub.
const BUTJ_LAUNCHER: &str = "launcher.lure_workbench.sh";

/// The desuetude gate's code, the value the substrate's generated constants
/// declare as `BUBG_DESUETUDE_DENTIL`, spelled here as a literal on purpose
/// (BUr_vsm).
const BUTJ_DESUETUDE: i32 = 119;

/// The coordinator every seat here runs: it writes its arguments and the values
/// a hurdle asks after into files under the seat, so the rust side reads what
/// the coordinator was handed rather than what it chose to print. Its dispatch
/// values it reads from the file the pointer names.
const BUTJ_COORDINATOR: &str = "#!/bin/bash\n\
     set -euo pipefail\n\
     source \"${BURD_BURD_FILE}\"\n\
     test \"$#\" -eq 0 || shift\n\
     : > \"${BURD_CONFIG_DIR}/../args.txt\"\n\
     for z_arg in \"$@\"; do printf '[%s]\\n' \"${z_arg}\" >> \"${BURD_CONFIG_DIR}/../args.txt\"; done\n\
     printf '%s\\n' \"${BURD_SEAT}\" > \"${BURD_CONFIG_DIR}/../seat.txt\"\n\
     printf '%s\\n' \"${BUTJ_OWN:-unset}\" > \"${BURD_CONFIG_DIR}/../own.txt\"\n\
     printf '%s\\n' \"${BURD_BURD_FILE}\" > \"${BURD_CONFIG_DIR}/../pointer.txt\"\n";

/// A composed seat, its tabtarget, and what a drive said.
struct butj_Seat {
    lure: bktu_Lure,
    tabtarget: PathBuf,
}

struct butj_Said {
    code: Option<i32>,
    text: String,
}

impl butj_Seat {
    fn butj_compose(name: &str) -> butj_Seat {
        let lure = bktu_Lure::bktu_compose(name);
        let tabtarget = lure.bktu_substrate_seat(BUTJ_COORDINATOR);
        butj_Seat { lure, tabtarget }
    }

    /// Lay a door of the seat's own, its body the hurdle's.
    fn butj_door(&self, relative: &str, body: &str) -> PathBuf {
        let path = self.lure.bktu_root().join(relative);
        bkprs_program_write(&path, body).unwrap_or_else(|err| {
            panic!(
                "{}",
                err, //
            )
        });
        path
    }

    fn butj_drive(&self, door: &Path, args: &[&str], stated: &[(&str, &str)]) -> butj_Said {
        let stated: Vec<(&str, &std::ffi::OsStr)> = stated
            .iter()
            .map(|(name, value)| (*name, std::ffi::OsStr::new(*value)))
            .collect();
        let out = self.lure.bktu_dispatch_stating(door, args, &stated);
        let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
        text.push_str(&String::from_utf8_lossy(&out.stderr));
        butj_Said {
            code: out.status.code(),
            text,
        }
    }

    fn butj_read(&self, relative: &str) -> String {
        let path = self.lure.bktu_root().join(relative);
        std::fs::read_to_string(&path).unwrap_or_else(|err| {
            panic!(
                "no {}: {}",
                path.display(),
                err, //
            )
        })
    }

    /// The moorings directory the seat's trampoline names, read off its own line.
    fn butj_moorings(&self) -> String {
        let trampoline = self.butj_read("tt/z-launcher.sh");
        let line = trampoline
            .lines()
            .find_map(|line| line.strip_prefix("z_moorings_dir=\""))
            .expect("the seat's trampoline carries its directory line");
        line.trim_end_matches('"').to_string()
    }
}

impl butj_Said {
    fn butj_took(&self, want: i32) -> &butj_Said {
        assert_eq!(
            self.code,
            Some(want),
            "took {:?} where {} was owed:\n{}",
            self.code,
            want,
            self.text, //
        );
        self
    }

    fn butj_carries(&self, needle: &str) -> &butj_Said {
        assert!(
            self.text.contains(needle),
            "never said {:?}:\n{}",
            needle,
            self.text, //
        );
        self
    }
}

/// A stub of the z-launcher family carrying the given configuration line.
fn butj_stub(configuration_line: &str) -> String {
    format!(
        "#!/bin/bash\n{}\n{}\n",
        configuration_line,
        BUTJ_EXEC, //
    )
}

#[test]
fn butj_a_stub_saying_no_configuration_dies_naming_the_canonical_line() {
    let seat = butj_Seat::butj_compose("substrate-config-missing");
    let door = seat.butj_door(
        "tt/butj-m.Missing.sh",
        &format!(
            "#!/bin/bash\n{}\n",
            BUTJ_EXEC, //
        ),
    );
    seat.butj_drive(&door, &[], &[])
        .butj_took(1)
        .butj_carries(BUTJ_CANONICAL);
}

#[test]
fn butj_a_malformed_configuration_word_dies_naming_the_canonical_line() {
    let seat = butj_Seat::butj_compose("substrate-config-malformed");
    let door = seat.butj_door(
        "tt/butj-w.Word.sh",
        &butj_stub(&format!(
            "set -- '{} buuc_loud' \"$@\"",
            BUTJ_LAUNCHER, //
        )),
    );
    seat.butj_drive(&door, &[], &[])
        .butj_took(1)
        .butj_carries(BUTJ_WORD_OWNLOG)
        .butj_carries("buuc_loud");
}

#[test]
fn butj_each_configuration_word_is_admitted_and_the_own_log_word_stands_alone() {
    // THE CONTROL FOR THE MALFORMED-WORD REFUSAL ABOVE: each word the launcher
    // answers to, and the one pair it admits, dispatch green; the own-log word
    // beside another dies, naming the canonical line.
    let seat = butj_Seat::butj_compose("substrate-config-words");
    for (name, words) in [
        (
            "tt/butj-i.Interactive.sh",
            BUTJ_WORD_INTERACTIVE.to_string(),
        ),
        ("tt/butj-n.Nolog.sh", BUTJ_WORD_NOLOG.to_string()),
        ("tt/butj-o.Ownlog.sh", BUTJ_WORD_OWNLOG.to_string()),
        (
            "tt/butj-p.Pair.sh",
            format!(
                "{} {}",
                BUTJ_WORD_INTERACTIVE,
                BUTJ_WORD_NOLOG, //
            ),
        ),
    ] {
        let door = seat.butj_door(
            name,
            &butj_stub(&format!(
                "set -- '{} {}' \"$@\"",
                BUTJ_LAUNCHER,
                words, //
            )),
        );
        seat.butj_drive(&door, &[], &[]).butj_took(0);
    }
    let door = seat.butj_door(
        "tt/butj-x.Crowded.sh",
        &butj_stub(&format!(
            "set -- '{} {} {}' \"$@\"",
            BUTJ_LAUNCHER,
            BUTJ_WORD_OWNLOG,
            BUTJ_WORD_NOLOG, //
        )),
    );
    seat.butj_drive(&door, &[], &[])
        .butj_took(1)
        .butj_carries(BUTJ_WORD_OWNLOG);
}

#[test]
fn butj_an_absent_launcher_stub_dies_naming_the_canonical_line() {
    let seat = butj_Seat::butj_compose("substrate-config-absent");
    let door = seat.butj_door(
        "tt/butj-a.Absent.sh",
        &butj_stub("set -- 'launcher.absent_workbench.sh' \"$@\""),
    );
    seat.butj_drive(&door, &[], &[])
        .butj_took(1)
        .butj_carries("launcher.absent_workbench.sh")
        .butj_carries(BUTJ_CANONICAL);
}

#[test]
fn butj_a_stale_trampoline_over_the_new_kit_refuses_on_the_desuetude_code() {
    // THE OLD CONTRACT WHOLE: a trampoline exporting its launcher and handing the
    // stub the tabtarget first, under a stub exporting its launcher. The launcher
    // reads the handed arguments alone, so the stale shape is refused before any
    // file is sourced, and the refusal carries the canonical trampoline.
    let seat = butj_Seat::butj_compose("substrate-config-stale-trampoline");
    let moorings = seat.butj_moorings();
    seat.butj_door(
        "tt/z-launcher.sh",
        &format!(
            "#!/bin/bash\n\
             set -u\n\
             z_dir=\"${{BASH_SOURCE[0]%/*}}\"\n\
             case \"${{z_dir}}\" in\n\
             \x20 /*) ;;\n\
             \x20 *)  z_dir=\"${{PWD}}/${{z_dir}}\" ;;\n\
             esac\n\
             z_moorings_dir=\"{moorings}\"\n\
             z_launcher=\"${{z_dir}}/../${{z_moorings_dir}}/launchers/${{BURD_LAUNCHER}}\"\n\
             cd -P \"${{z_dir}}/..\" || exit 1\n\
             export BURD_CONFIG_DIR=\"${{PWD}}/${{z_moorings_dir}}\"\n\
             exec \"${{z_launcher}}\" \"$@\"\n",
            moorings = moorings, //
        ),
    );
    let door = seat.butj_door(
        "tt/butj-s.Stale.sh",
        &format!(
            "#!/bin/bash\nexport BURD_LAUNCHER={}\n{}\n",
            BUTJ_LAUNCHER,
            BUTJ_EXEC, //
        ),
    );
    seat.butj_drive(&door, &[], &[])
        .butj_took(BUTJ_DESUETUDE)
        .butj_carries("STALE TRAMPOLINE")
        .butj_carries("exec \"${z_launcher}\" \"${2}\" \"${PWD}\" \"${PWD}/${z_moorings_dir}\" \"${1}\" \"${@:3}\"");
}

#[test]
fn butj_a_users_own_variable_passes_through_a_stub_untouched() {
    let seat = butj_Seat::butj_compose("substrate-config-own");
    let tabtarget = seat.tabtarget.clone();
    seat.butj_drive(&tabtarget, &[], &[("BUTJ_OWN", "kept as stated")])
        .butj_took(0);
    assert_eq!(seat.butj_read("own.txt").trim_end(), "kept as stated");
}

#[test]
fn butj_arguments_reach_the_coordinator_intact_and_in_order() {
    // A SPACE, AN EMPTY ONE, AND A CONFIGURATION WORD: the three a positional
    // handoff loses first. The last proves a user's argument spelled like a
    // buuc_ word is the user's and never read as configuration.
    let seat = butj_Seat::butj_compose("substrate-config-args");
    let tabtarget = seat.tabtarget.clone();
    seat.butj_drive(&tabtarget, &["a b", "", BUTJ_WORD_NOLOG], &[])
        .butj_took(0);
    assert_eq!(
        seat.butj_read("args.txt"),
        format!(
            "[a b]\n[]\n[{}]\n",
            BUTJ_WORD_NOLOG, //
        )
    );
}

#[test]
fn butj_the_seat_is_the_directory_whose_tt_the_door_was_reached_through() {
    let seat = butj_Seat::butj_compose("substrate-config-seat");
    let tabtarget = seat.tabtarget.clone();
    seat.butj_drive(&tabtarget, &[], &[]).butj_took(0);
    let want = seat
        .lure
        .bktu_root()
        .canonicalize()
        .expect("the seat resolves");
    assert_eq!(
        Path::new(seat.butj_read("seat.txt").trim_end()),
        want.as_path()
    );
}

#[test]
fn butj_the_dispatch_file_stands_once_in_temp_and_output_and_carries_no_argument_list() {
    let seat = butj_Seat::butj_compose("substrate-config-file");
    let tabtarget = seat.tabtarget.clone();
    seat.butj_drive(&tabtarget, &["an argument"], &[])
        .butj_took(0);
    let pointer = PathBuf::from(seat.butj_read("pointer.txt").trim_end());
    let body = std::fs::read_to_string(&pointer).unwrap_or_else(|err| {
        panic!(
            "{}: {}",
            pointer.display(),
            err, //
        )
    });
    // The seat's regime roots its output beside the seat, as its burc.env says.
    let output = seat
        .lure
        .bktu_root()
        .join("output")
        .join("current")
        .join("burd.env");
    let copy = std::fs::read_to_string(&output).unwrap_or_else(|err| {
        panic!(
            "{}: {}",
            output.display(),
            err, //
        )
    });
    assert_eq!(
        body, copy,
        "the output copy is the temp file byte for byte", //
    );

    let mut keys: Vec<&str> = body
        .lines()
        .map(|line| line.split('=').next().unwrap_or_default())
        .collect();
    let total = keys.len();
    keys.sort();
    keys.dedup();
    assert_eq!(
        keys.len(),
        total,
        "every key stands once:\n{}",
        body, //
    );
    assert!(
        !body.contains("an argument"),
        "the argument list never enters the file:\n{}",
        body, //
    );
    assert!(
        keys.contains(&"BURD_BURD_FILE") && keys.contains(&"BURD_SEAT"),
        "{}",
        body, //
    );
}

#[test]
fn butj_a_cli_whose_furnish_sources_nothing_holds_both_regimes_kindled() {
    // THE COMMAND MODULE IS THE ONE LOADER. The coordinator writes a CLI whose
    // furnish sources nothing and execs it through buc_execute; the CLI reads a
    // dispatch value and a config value behind both regimes' sentinels.
    let lure = bktu_Lure::bktu_compose("substrate-config-kindled");
    let tabtarget = lure.bktu_substrate_seat(
        "#!/bin/bash\n\
         set -euo pipefail\n\
         source \"${BURD_BURD_FILE}\"\n\
         z_cli=\"${BURD_TEMP_DIR}/butj_cli.sh\"\n\
         cat > \"${z_cli}\" <<'BUTJ_CLI'\n\
         #!/bin/bash\n\
         source \"${BURD_BUK_DIR}/buc_command.sh\" || exit 1\n\
         source \"${BURD_BUK_DIR}/buym_yelp.sh\" || exit 1\n\
         butj_show() {\n\
         \x20 zburd_sentinel\n\
         \x20 zburc_sentinel\n\
         \x20 printf 'seat=%s tools=%s\\n' \"${BURD_SEAT}\" \"${BURC_TOOLS_DIR}\"\n\
         }\n\
         zbutj_furnish() { buc_doc_env_done || return 0; }\n\
         buc_execute butj_ \"Configuration probe\" zbutj_furnish \"$@\"\n\
         BUTJ_CLI\n\
         chmod +x \"${z_cli}\"\n\
         exec \"${z_cli}\" butj_show\n",
    );
    let out = lure.bktu_dispatch(&tabtarget, &[]);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr), //
    );
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        said, //
    );
    let seat = lure.bktu_root().canonicalize().expect("the seat resolves");
    assert!(
        said.contains(&format!(
            "seat={} tools=",
            seat.display(), //
        )),
        "{}",
        said, //
    );
}

#[test]
fn butj_a_coordinator_handed_no_readable_file_dies_naming_the_pointer() {
    let lure = bktu_Lure::bktu_compose("substrate-config-pointer");
    let tabtarget = lure.bktu_substrate_seat(
        "#!/bin/bash\n\
         set -euo pipefail\n\
         source \"${BURD_BURD_FILE}\"\n\
         z_cli=\"${BURD_TEMP_DIR}/butj_cli.sh\"\n\
         printf '%s\\n' '#!/bin/bash' 'source \"${BURD_BUK_DIR}/buc_command.sh\"' 'echo reached' > \"${z_cli}\"\n\
         z_status=0\n\
         env BURD_BURD_FILE=\"${BURD_TEMP_DIR}/absent.env\" bash \"${z_cli}\" || z_status=$?\n\
         printf 'status=%s\\n' \"${z_status}\"\n",
    );
    let out = lure.bktu_dispatch(&tabtarget, &[]);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr), //
    );
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}",
        said, //
    );
    assert!(
        said.contains("status=1"),
        "the coordinator died imprecisely:\n{}",
        said, //
    );
    assert!(
        said.contains("BURD_BURD_FILE"),
        "naming the pointer:\n{}",
        said, //
    );
    assert!(
        !said.contains("reached"),
        "and went no further:\n{}",
        said, //
    );
}

#[test]
fn butj_a_dispatch_line_carrying_a_space_and_a_parenthesis_reads_alike_in_bash_and_the_regime_reader(
) {
    // THE LINE WRITER IS THE DISPATCH'S OWN, sourced for its functions in a child
    // that runs no dispatch; bash reads the line back by sourcing it, and the
    // estate's one regime reader reads the same file outside a shell.
    let value = "a value (with) a space";
    let lure = bktu_Lure::bktu_compose("substrate-config-grammar");
    let tabtarget = lure.bktu_substrate_seat(&format!(
        "#!/bin/bash\n\
         set -euo pipefail\n\
         source \"${{BURD_BURD_FILE}}\"\n\
         z_line=\"${{BURD_CONFIG_DIR}}/../line.env\"\n\
         env -u BURD_BURD_FILE bash -c 'source \"${{BURD_BUK_DIR}}/bud_dispatch.sh\"; z_bud_burx_text=\"\"; zbud_burx_append BUTJ_SPACED \"$1\"; printf \"%s\" \"${{z_bud_burx_text}}\" > \"$2\"' butj '{value}' \"${{z_line}}\"\n\
         bash -c 'source \"$1\"; printf \"%s\\n\" \"${{BUTJ_SPACED}}\"' butj \"${{z_line}}\" > \"${{BURD_CONFIG_DIR}}/../bash.txt\"\n",
        value = value, //
    ));
    let out = lure.bktu_dispatch(&tabtarget, &[]);
    assert_eq!(
        out.status.code(),
        Some(0),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr), //
    );
    let line = lure.bktu_root().join("line.env");
    let regime = bkp::bkprc_catena::bkprc_read(&line).unwrap_or_else(|err| {
        panic!(
            "{}",
            err, //
        )
    });
    assert_eq!(regime.bkprc_scalar("BUTJ_SPACED"), Some(value));
    let bash =
        std::fs::read_to_string(lure.bktu_root().join("bash.txt")).expect("bash read the line");
    assert_eq!(bash.trim_end(), value);
}

#[test]
fn butj_the_structure_check_admits_the_configuration_form_and_refuses_an_export_line_by_name() {
    // THE KIT'S OWN STRUCTURE CHECK, buq_tabtargets, over a composed yard: a stub
    // in the configuration form passes, and the same yard holding a stub that
    // carries an export line is refused, the refusal naming that stub. It runs
    // in a child with the dispatch pointer withdrawn, a harness sourcing the
    // substrate for its functions alone, the moorings name assigned in that
    // child's own script.
    let lure = bktu_Lure::bktu_compose("substrate-config-structure");
    let root = lure.bktu_root().to_path_buf();
    std::fs::create_dir_all(root.join("tt")).expect("tt");
    std::fs::create_dir_all(root.join("quobmm").join("buml_launchers")).expect("launchers");
    std::fs::write(
        root.join("quobmm")
            .join("buml_launchers")
            .join("launcher.quob_workbench.sh"),
        "#!/bin/bash\n",
    )
    .expect("launcher");
    std::fs::write(root.join("tt").join("z-launcher.sh"), "#!/bin/bash\n").expect("trampoline");
    std::fs::write(
        root.join("tt").join("quob-s.Sound.sh"),
        butj_stub(&format!(
            "set -- 'launcher.quob_workbench.sh {}' \"$@\"",
            BUTJ_WORD_NOLOG, //
        )),
    )
    .expect("sound stub");
    let substrate = but::buts_seat::buts_substrate();
    let check = |yard: &Path| {
        std::process::Command::new("bash")
            .env_remove("BURD_BURD_FILE")
            .arg("-c")
            .arg(
                "set -euo pipefail\n\
                 source \"$1/buc_command.sh\"\n\
                 source \"$1/buym_yelp.sh\"\n\
                 source \"$1/bubc_constants.sh\"\n\
                 source \"$1/buq_qualify.sh\"\n\
                 BURD_MOORINGS_DIR='quobmm'\n\
                 buq_tabtargets \"$2/tt\" \"$2\" 'z-*.sh'\n\
                 echo structure-holds\n",
            )
            .arg("butj")
            .arg(&substrate)
            .arg(yard)
            .output()
            .expect("bash spawns")
    };
    let out = check(&root);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr), //
    );
    assert_eq!(
        out.status.code(),
        Some(0),
        "the configuration form passes:\n{}",
        said, //
    );

    std::fs::write(
        root.join("tt").join("quob-e.Exported.sh"),
        format!(
            "#!/bin/bash\nexport QUOB_FLAG=1\n{}\n",
            BUTJ_EXEC, //
        ),
    )
    .expect("exported stub");
    let out = check(&root);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr), //
    );
    assert_ne!(
        out.status.code(),
        Some(0),
        "an export line is refused:\n{}",
        said, //
    );
    assert!(
        said.contains("quob-e.Exported.sh") && said.contains("export line"),
        "by name:\n{}",
        said, //
    );
}
