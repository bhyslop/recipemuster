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

//! The environment census: what a door's process holds exported, read at every
//! level of a nested chain of doors, and the refusal that keeps it so.
//!
//! A stub exports nothing, the config regime is read from its file and never
//! exported, and a dispatch hands its coordinator the pointer to its own run's
//! dispatch file and the kit directory, and nothing else. Each of those is a
//! claim about what a process DOES NOT hold, and a reading that finds nothing
//! proves nothing without a control: so a name is planted exported at the top,
//! in the outer door's environment, and every level's reading must strike it
//! before its silences count.
//!
//! EACH LEVEL READS ITS OWN EXPORTED NAMES before it sources anything, so what
//! `compgen -e` lists is exactly what that process inherited; the outer level
//! drives the inner door, a second dispatch of its own, which reads again.
//!
//! THE REFUSAL IS THE SCOPE SENTINEL'S SECOND RULE, under the enrollment gate's
//! code: a name of the kit standing exported where its level declares no
//! hand-off. Each refusal below plants a valid value, so the code says the rule
//! fired rather than any check of the value.
//!
//! THE STUBS ARE COMPOSED IN THE SEAT, and no tabtarget is committed for a test.
//!
//! OBSERVED FROM OUTSIDE THE DISPATCH, as every hurdle here is.

#![deny(warnings)]

use bkk::bktu_lure::bktu_Lure;
use bkp::bkprs_scrivener::bkprs_program_write;
use std::ffi::OsStr;

/// The control planted exported at the top of the chain.
/// A made-up name, matching no governed word and no family the estate seats.
const BUTQ_CONTROL: &str = "QUOBQ_PLANTED";

/// The two doors of the chain, and the levels they record under.
const BUTQ_OUTER: &str = "tt/butq-o.Outer.sh";
const BUTQ_INNER: &str = "tt/butq-i.Inner.sh";

/// The environment regime's value planted at the top, admitted at every level.
const BUTQ_AMBIENT: (&str, &str) = ("BURE_LABEL", "quobq-label");

/// The folio the coordinator hands its CLI as the zipper does, by exec.
const BUTQ_FOLIO: &str = "quobq-folio";

/// The kit directory the outer level hands the inner door in place of its own,
/// which the inner launcher must replace with its own run's.
const BUTQ_DECOY_KIT: &str = "/quobq-decoy/buk";

/// The rule every refusal here names, as the kit's line spells it.
const BUTQ_RULE: &str = "exported-name rule";

/// The enrollment gate's code, the value the substrate's generated constants
/// declare as `BUBG_ENROLL_DENTIL`, and the regime gate's, `BUBG_REGIME_DENTIL`,
/// spelled here as literals on purpose (BUr_vsm).
const BUTQ_ENROLL: i32 = 101;
const BUTQ_REGIME: i32 = 100;

/// The coordinator both doors reach. It records its own exported names before it
/// sources anything, then its pointer and kit directory from its dispatch file;
/// it passes its head's exported-name rule, drives a CLI as the zipper does, and
/// the outer level then drives the inner door handing it a decoy kit directory.
const BUTQ_COORDINATOR: &str = "#!/bin/bash\n\
     set -euo pipefail\n\
     z_listing=\"\"\n\
     z_status=0\n\
     z_listing=$(compgen -e) || z_status=$?\n\
     test \"${z_status}\" -le 1\n\
     source \"${BURD_BURD_FILE}\"\n\
     z_root=\"${BURD_CONFIG_DIR}/..\"\n\
     z_level=inner\n\
     case \"${BURD_TARGET}\" in\n\
     \x20 butq-o.*) z_level=outer ;;\n\
     esac\n\
     printf '%s\\n' \"${z_listing}\" > \"${z_root}/exported-${z_level}.txt\"\n\
     printf '%s\\n' \"${BURD_BURD_FILE}\" > \"${z_root}/pointer-${z_level}.txt\"\n\
     printf '%s\\n' \"${BURD_BUK_DIR}\" > \"${z_root}/kit-${z_level}.txt\"\n\
     source \"${BURD_BUK_DIR}/buc_command.sh\"\n\
     buv_export_sentinel \"${BURD_TARGET}\"\n\
     z_cli=\"${BURD_TEMP_DIR}/butq_cli.sh\"\n\
     cat > \"${z_cli}\" <<'BUTQ_CLI'\n\
     #!/bin/bash\n\
     source \"${BURD_BUK_DIR}/buc_command.sh\" || exit 1\n\
     butq_record() {\n\
     \x20 z_listing=\"\"\n\
     \x20 z_status=0\n\
     \x20 z_listing=$(compgen -e) || z_status=$?\n\
     \x20 test \"${z_status}\" -le 1\n\
     \x20 printf '%s\\n' \"${z_listing}\" > \"$1\"\n\
     \x20 printf '%s\\n' \"${BUZ_FOLIO}\" > \"$1.folio\"\n\
     }\n\
     zbutq_furnish() { buc_doc_env_done || return 0; }\n\
     buc_execute butq_ \"Census probe\" zbutq_furnish \"$@\"\n\
     BUTQ_CLI\n\
     chmod +x \"${z_cli}\"\n\
     BUZ_FOLIO=quobq-folio \"${z_cli}\" butq_record \"${z_root}/cli-${z_level}.txt\"\n\
     if test \"${z_level}\" = outer; then\n\
     \x20 env BURD_BUK_DIR=/quobq-decoy/buk \"${z_root}/tt/butq-i.Inner.sh\"\n\
     fi\n";

/// A stub of the seat's own launcher, the z-launcher family's exec line beneath.
const BUTQ_STUB: &str = "#!/bin/bash\n\
     set -- 'launcher.lure_workbench.sh' \"$@\"\n\
     exec \"${BASH_SOURCE[0]%/*}/z-launcher.sh\" \"${0##*/}\" \"${@}\"\n";

/// The families a name of the kit or the census wears across an exec.
const BUTQ_CENSUS_FAMILIES: &[&str] = &[
    "BUC_",
    "BURC_",
    "BURD_",
    "BURE_",
    "BURS_",
    "BURV_",
    "BURX_",
    "BUZ_",
    "VOM_", //
];

/// The names a file recorded, one per line.
fn butq_lines(lure: &bktu_Lure, relative: &str) -> Vec<String> {
    let path = lure.bktu_root().join(relative);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| {
            panic!(
                "nothing was recorded at {}: {}",
                path.display(),
                err, //
            )
        })
        .lines()
        .map(str::to_string)
        .filter(|name| !name.is_empty())
        .collect()
}

/// A reading holds the planted control, the two hand-offs and the environment
/// regime's planted value, and no other name of the kit or the census.
fn zbutq_handed(level: &str, names: &[String]) {
    // THE CONTROL FIRST: a reading that cannot find the name planted exported
    // at the top has read nothing, and every silence below would be its.
    assert!(
        names.iter().any(|name| name == BUTQ_CONTROL),
        "the {} reading misses the planted control: {:?}",
        level,
        names, //
    );
    for held in ["BURD_BURD_FILE", "BURD_BUK_DIR", BUTQ_AMBIENT.0] {
        assert!(
            names.iter().any(|name| name == held),
            "the {} reading holds {}: {:?}",
            level,
            held,
            names, //
        );
    }
    let stray: Vec<&String> = names
        .iter()
        .filter(|name| {
            BUTQ_CENSUS_FAMILIES
                .iter()
                .any(|family| name.starts_with(family))
        })
        .filter(|name| {
            !name.starts_with("BURE_")
                && name.as_str() != "BURD_BURD_FILE"
                && name.as_str() != "BURD_BUK_DIR"
        })
        .collect();
    assert!(
        stray.is_empty(),
        "the {} reading holds {:?} exported",
        level,
        stray, //
    );
}

/// One line of what a drive said, both streams.
fn zbutq_said(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr), //
    )
}

/// EVERY LEVEL OF A NESTED CHAIN HOLDS ITS HAND-OFFS AND NOTHING ELSE OF THE KIT.
/// The top plants the control, an environment value and an override; each
/// coordinator holds the pointer, the kit directory and the environment value,
/// passes its head's rule, and hands its CLI the folio as the zipper does; the
/// CLI holds the same and reads the folio no longer exported. The override is
/// admitted at the outer launcher, which is why the chain runs, and absent below
/// it. The inner launcher replaces the inherited pointer and the decoy kit
/// directory with its own run's.
#[test]
fn butq_every_level_of_a_nested_chain_holds_its_hand_offs_alone() {
    let lure = bktu_Lure::bktu_compose("substrate-environment-census");
    lure.bktu_substrate_seat(BUTQ_COORDINATOR);
    for door in [BUTQ_OUTER, BUTQ_INNER] {
        bkprs_program_write(&lure.bktu_root().join(door), BUTQ_STUB).unwrap_or_else(|err| {
            panic!(
                "{}",
                err, //
            )
        });
    }
    let override_logs = lure.bktu_root().join("burv-logs");

    let out = lure.bktu_dispatch_stating(
        &lure.bktu_root().join(BUTQ_OUTER),
        &[],
        &[
            (BUTQ_CONTROL, OsStr::new("planted at the top")),
            (BUTQ_AMBIENT.0, OsStr::new(BUTQ_AMBIENT.1)),
            // The override family's member: admitted at the launcher, folded into
            // the run's record, and withdrawn below the coordinator.
            ("BURV_LOG_DIR", override_logs.as_os_str()),
        ],
    );
    assert_eq!(
        out.status.code(),
        Some(0),
        "the chain ran:\n{}",
        zbutq_said(&out), //
    );

    let mut pointers = Vec::new();
    let mut kits = Vec::new();
    for level in ["outer", "inner"] {
        zbutq_handed(
            &format!(
                "{} coordinator",
                level, //
            ),
            &butq_lines(
                &lure,
                &format!(
                    "exported-{}.txt",
                    level, //
                ),
            ),
        );

        let cli = butq_lines(
            &lure,
            &format!(
                "cli-{}.txt",
                level, //
            ),
        );
        zbutq_handed(
            &format!(
                "{} CLI",
                level, //
            ),
            &cli,
        );
        let folio = butq_lines(
            &lure,
            &format!(
                "cli-{}.txt.folio",
                level, //
            ),
        );
        assert_eq!(
            folio,
            vec![BUTQ_FOLIO.to_string()],
            "the {} CLI still reads its folio",
            level, //
        );

        let pointer = butq_lines(
            &lure,
            &format!(
                "pointer-{}.txt",
                level, //
            ),
        )
        .join("");
        assert!(
            std::path::Path::new(&pointer).is_file(),
            "the {} pointer names a file: {}",
            level,
            pointer, //
        );
        pointers.push(pointer);
        kits.push(
            butq_lines(
                &lure,
                &format!(
                    "kit-{}.txt",
                    level, //
                ),
            )
            .join(""),
        );
    }
    assert_ne!(
        pointers[0], pointers[1],
        "each level reads its own run's dispatch file, never its parent's", //
    );
    assert_ne!(
        kits[1], BUTQ_DECOY_KIT,
        "the inner launcher replaced the inherited kit directory", //
    );
    assert_eq!(
        kits[0], kits[1],
        "with its own run's, the same kit", //
    );
}

/// The names each refusal plants, one per family the rule guards beyond the
/// environment regime: a config value, a station value and a dispatch value,
/// each valid.
fn zbutq_planted(lure: &bktu_Lure) -> Vec<(&'static str, std::ffi::OsString)> {
    vec![
        (
            "BURC_TOOLS_DIR",
            but::buts_seat::buts_substrate()
                .parent()
                .expect("a tools directory")
                .as_os_str()
                .to_os_string(),
        ),
        ("BURS_USER", "kennel".into()),
        ("BURD_TEMP_DIR", lure.bktu_temp().as_os_str().to_os_string()),
    ]
}

/// A refusal reads the enrollment gate's code and names the variable, the rule
/// and the door.
fn zbutq_refused(said: &str, code: Option<i32>, name: &str, door: &str) {
    assert_eq!(
        code,
        Some(BUTQ_ENROLL),
        "{} is refused by the enrollment gate's code:\n{}",
        name,
        said, //
    );
    assert!(
        said.contains(name),
        "naming the variable {}:\n{}",
        name,
        said, //
    );
    assert!(
        said.contains(BUTQ_RULE),
        "naming the rule:\n{}",
        said, //
    );
    assert!(
        said.contains(door),
        "naming the door {}:\n{}",
        door,
        said, //
    );
}

/// A CONFIG, A STATION AND A DISPATCH NAME ARRIVING EXPORTED ARE EACH REFUSED AT
/// A LAUNCHER'S ENTRY, behind a plain drive of the same door running green.
#[test]
fn butq_a_regime_name_arriving_exported_is_refused_at_a_launchers_entry() {
    let lure = bktu_Lure::bktu_compose("substrate-refused-launcher");
    let door = lure.bktu_substrate_seat("#!/bin/bash\necho butq-reached\n");
    let named = door
        .file_name()
        .expect("the door has a name")
        .to_string_lossy()
        .into_owned();

    let plain = lure.bktu_dispatch(&door, &[]);
    assert_eq!(
        plain.status.code(),
        Some(0),
        "the control:\n{}",
        zbutq_said(&plain), //
    );

    for (name, value) in zbutq_planted(&lure) {
        let out = lure.bktu_dispatch_stating(&door, &[], &[(name, value.as_os_str())]);
        let said = zbutq_said(&out);
        zbutq_refused(&said, out.status.code(), name, &named);
        assert!(
            !said.contains("butq-reached"),
            "and the coordinator never ran:\n{}",
            said, //
        );
    }
}

/// The door a coordinator's entry is reached through when a hurdle hands a
/// dispatch its arguments itself.
const BUTQ_DIRECT: &str = "butq-c.Coordinator.sh";

/// A CONFIG, A STATION AND A DISPATCH NAME ARRIVING EXPORTED ARE EACH REFUSED AT
/// A COORDINATOR'S ENTRY. The seat's coordinator hands a dispatch the launcher's
/// arguments itself, the name exported, so nothing ahead of the inner
/// coordinator refuses it; the inner coordinator is a workbench's head, and the
/// control is the same dispatch handed no plant.
#[test]
fn butq_a_regime_name_arriving_exported_is_refused_at_a_coordinators_entry() {
    let lure = bktu_Lure::bktu_compose("substrate-refused-coordinator");
    let door = lure.bktu_substrate_seat(&format!(
        "#!/bin/bash\n\
         set -euo pipefail\n\
         test \"$#\" -eq 0 || shift\n\
         source \"${{BURD_BURD_FILE}}\"\n\
         z_probe=\"${{BURD_TEMP_DIR}}/butq-probe.sh\"\n\
         printf '%s\\n' '#!/bin/bash' 'source \"${{BURD_BUK_DIR}}/buc_command.sh\"' 'buv_export_sentinel \"${{BURD_TARGET}}\"' 'echo butq-reached' > \"${{z_probe}}\"\n\
         chmod +x \"${{z_probe}}\"\n\
         z_status=0\n\
         env \"$@\" bash \"${{BURD_BUK_DIR}}/bud_dispatch.sh\" \"${{z_probe}}\" 'launcher.lure_workbench.sh buuc_nolog' \"${{PWD}}\" \"${{BURD_CONFIG_DIR}}\" '{direct}' || z_status=$?\n\
         printf 'butq-status=%s\\n' \"${{z_status}}\"\n",
        direct = BUTQ_DIRECT, //
    ));

    let plain = lure.bktu_dispatch(&door, &["QUOBQ_NOTHING=1"]);
    let said = zbutq_said(&plain);
    assert!(
        said.contains("butq-status=0") && said.contains("butq-reached"),
        "the control:\n{}",
        said, //
    );

    for (name, value) in zbutq_planted(&lure) {
        let plant = format!(
            "{}={}",
            name,
            value.to_string_lossy(), //
        );
        let out = lure.bktu_dispatch(&door, &[&plant]);
        let said = zbutq_said(&out);
        assert_eq!(
            out.status.code(),
            Some(0),
            "the seat's own door runs:\n{}",
            said, //
        );
        let code = said
            .lines()
            .find_map(|line| line.strip_prefix("butq-status="))
            .and_then(|status| status.trim().parse::<i32>().ok());
        zbutq_refused(&said, code, name, BUTQ_DIRECT);
        assert!(
            !said.contains("butq-reached"),
            "and the coordinator went no further:\n{}",
            said, //
        );
    }
}

/// Run one regime's kindle directly, in a harness with every family of the kit
/// withdrawn from its environment, the body assigning and exporting what the
/// case plants.
fn zbutq_kindle(regime: &str, body: &str) -> (Option<i32>, String) {
    let mut child = std::process::Command::new("bash");
    for (name, _) in std::env::vars() {
        if BUTQ_CENSUS_FAMILIES
            .iter()
            .any(|family| name.starts_with(family))
        {
            child.env_remove(name);
        }
    }
    let out = child
        .arg("-c")
        .arg(format!(
            "set -euo pipefail\n\
             source \"$1/buc_command.sh\"\n\
             source \"$1/buv_validation.sh\"\n\
             source \"$1/bubc_constants.sh\"\n\
             source \"$1/{regime}\"\n\
             zbuv_kindle\n\
             {body}\n\
             echo butq-kindled\n"
        ))
        .arg("butq")
        .arg(but::buts_seat::buts_substrate())
        .output()
        .expect("bash spawns");
    (out.status.code(), zbutq_said(&out))
}

/// EACH REGIME'S KINDLE REFUSES AN EXPORTED NAME OF ITS OWN SCOPE ITS LEVEL DOES
/// NOT DECLARE, naming the name, the rule and the kindle. The controls are the
/// same kindles over the same values unexported, the dispatch kindle with its two
/// hand-offs exported, and the environment regime's, whose exported names every
/// level declares.
#[test]
fn butq_each_regime_kindle_refuses_an_exported_name_of_its_scope() {
    let cases = [
        (
            "burc_regime.sh",
            "BURC",
            "BURC_TOOLS_DIR=Tools",
            "BURC_TOOLS_DIR",
            "zburc_kindle",
        ),
        (
            "burs_regime.sh",
            "BURS",
            "BURD_MOORINGS_DIR=quobmm\nBURS_USER=kennel",
            "BURS_USER",
            "zburs_kindle",
        ),
        (
            "burd_regime.sh",
            "BURD",
            "BURD_MOORINGS_DIR=quobmm\nBURD_TEMP_DIR=/quobq-temp",
            "BURD_TEMP_DIR",
            "zburd_kindle",
        ),
    ];
    for (regime, scope, assigned, name, kindle) in cases {
        let (code, said) = zbutq_kindle(
            regime,
            &format!(
                "{}\n{}",
                assigned,
                kindle, //
            ),
        );
        assert_eq!(
            code,
            Some(0),
            "the {} control kindles:\n{}",
            scope,
            said, //
        );
        assert!(
            said.contains("butq-kindled"),
            "{}",
            said, //
        );

        let (code, said) = zbutq_kindle(
            regime,
            &format!(
                "{}\nexport {}\n{}",
                assigned,
                name,
                kindle, //
            ),
        );
        assert_eq!(
            code,
            Some(BUTQ_ENROLL),
            "the {} kindle refuses by the enrollment gate's code:\n{}",
            scope,
            said, //
        );
        assert!(
            said.contains(name),
            "naming the variable:\n{}",
            said, //
        );
        assert!(
            said.contains(BUTQ_RULE),
            "naming the rule:\n{}",
            said, //
        );
        assert!(
            said.contains(&format!(
                "arrives exported at the {} ",
                scope, //
            )),
            "naming the kindle:\n{}",
            said, //
        );
        assert!(
            !said.contains("butq-kindled"),
            "{}",
            said, //
        );
    }

    let (code, said) = zbutq_kindle(
        "burd_regime.sh",
        "BURD_MOORINGS_DIR=quobmm\nexport BURD_BURD_FILE=/quobq-burd.env BURD_BUK_DIR=/quobq-kit\nzburd_kindle",
    );
    assert_eq!(
        code,
        Some(0),
        "the dispatch kindle admits its two hand-offs exported:\n{}",
        said, //
    );

    let (code, said) = zbutq_kindle(
        "bure_regime.sh",
        "export BURE_LABEL=quobq-label\nzbure_kindle",
    );
    assert_eq!(
        code,
        Some(0),
        "the environment kindle admits its own names exported:\n{}",
        said, //
    );
    assert!(
        said.contains("butq-kindled"),
        "{}",
        said, //
    );
}

/// A SET-FORM POISON READS THE GATE OF THE RULE IT BREAKS AND NEVER THE
/// EXPORTED-NAME RULE: the seam assigns without the export attribute, so the
/// poisoned value is judged as a file's would be. An enforce rule's poison
/// reads the regime gate's code; a vet rule's the enrollment gate's, naming the
/// vet's rule.
#[test]
fn butq_a_set_form_poison_reads_the_gate_of_the_rule_it_breaks() {
    let lure = bktu_Lure::bktu_compose("substrate-poison-gates");
    let door = lure.bktu_substrate_seat("#!/bin/bash\necho butq-reached\n");

    for (spec, code, carried) in [
        (
            "BURC_ENDONYM=buon_Bad1",
            BUTQ_REGIME,
            "Invalid BURC_ENDONYM format",
        ),
        (
            "BURC_TABTARGET_DELIMITER=..",
            BUTQ_ENROLL,
            "BURC_TABTARGET_DELIMITER:",
        ),
    ] {
        let out = lure.bktu_dispatch_stating(
            &door,
            &[],
            // The seam's name, the value `BUBC_tweak_regime_poison` declares, spelled
            // here as a literal on purpose (BUr_vsm).
            &[
                ("BURE_TWEAK_NAME", OsStr::new("buost_regime_poison")),
                ("BURE_TWEAK_VALUE", OsStr::new(spec)),
            ],
        );
        let said = zbutq_said(&out);
        assert_eq!(
            out.status.code(),
            Some(code),
            "the poison {} reads its gate:\n{}",
            spec,
            said, //
        );
        assert!(
            said.contains(carried),
            "naming its rule:\n{}",
            said, //
        );
        assert!(
            !said.contains(BUTQ_RULE),
            "never the exported-name rule:\n{}",
            said, //
        );
    }
}

/// Every `export` in a script's text naming a family of the kit, as `file:line
/// name`, its comments set aside.
///
/// THE TEXT IS READ, never run, so a name an expansion spells (`export
/// "${…}"`) is no finding here: the reading the close takes by hand covers those.
fn zbutq_exports(file: &str, text: &str) -> Vec<String> {
    let families = [
        "BURC_",
        "BURS_",
        "BURD_",
        "BURE_",
        "BURV_",
        "BURX_",
        "BUZ_",
        "BUC_",
        "VOM_",
    ];
    let mut found = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let code = line.split('#').next().unwrap_or("");
        let mut rest = code;
        while let Some(at) = rest.find("export") {
            let before = &rest[..at];
            let after = &rest[at + "export".len()..];
            rest = after;
            if before
                .chars()
                .last()
                .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
            {
                continue;
            }
            if !after.starts_with(' ') {
                continue;
            }
            for word in after.split_whitespace() {
                let name = word.split('=').next().unwrap_or("");
                if families.iter().any(|family| name.starts_with(family)) {
                    found.push(format!(
                        "{}:{} {}",
                        file,
                        index + 1,
                        name, //
                    ));
                    continue;
                }
                let plain = name
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
                    && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
                if !plain || word.contains('=') {
                    break;
                }
            }
        }
    }
    found
}

/// Every shipped bash file beneath a kit directory, build output set aside.
fn zbutq_shipped(dir: &std::path::Path, into: &mut Vec<std::path::PathBuf>) {
    let entries = std::fs::read_dir(dir).unwrap_or_else(|err| {
        panic!(
            "{}: {}",
            dir.display(),
            err, //
        )
    });
    for entry in entries {
        let path = entry.expect("a directory entry").path();
        if path.is_dir() {
            if path.file_name().is_some_and(|name| name != "target") {
                zbutq_shipped(&path, into);
            }
        } else if path.extension().is_some_and(|ext| ext == "sh") {
            into.push(path);
        }
    }
}

/// THE KIT'S SHIPPED BASH EXPORTS NO NAME OF ITS FAMILIES SAVE THE TWO HAND-OFFS,
/// and those in the dispatch alone, so the policy is mechanical rather than
/// remembered. The control is the same reading over a composed line that
/// exports a config name, which it must strike.
#[test]
fn butq_the_kits_shipped_bash_exports_no_name_of_its_families_save_the_two() {
    let control = zbutq_exports(
        "composed.sh",
        "z_x=1\n  export QUOBQ_A BURC_TOOLS_DIR # a comment export BURS_USER\n",
    );
    assert_eq!(
        control,
        vec!["composed.sh:2 BURC_TOOLS_DIR".to_string()],
        "the control strikes", //
    );

    let substrate = but::buts_seat::buts_substrate();
    let tools = substrate.parent().expect("a tools directory");
    let mut files = Vec::new();
    zbutq_shipped(&substrate, &mut files);
    zbutq_shipped(&tools.join("bkk"), &mut files);
    assert!(
        files.iter().any(|file| file.ends_with("bud_dispatch.sh")),
        "the reading reaches the dispatch", //
    );

    let mut found = Vec::new();
    for file in &files {
        let text = std::fs::read_to_string(file).unwrap_or_else(|err| {
            panic!(
                "{}: {}",
                file.display(),
                err, //
            )
        });
        let named = file.display().to_string();
        for finding in zbutq_exports(&named, &text) {
            let admitted = file.ends_with("bud_dispatch.sh")
                && (finding.ends_with(" BURD_BURD_FILE") || finding.ends_with(" BURD_BUK_DIR"));
            if !admitted {
                found.push(finding);
            }
        }
    }
    assert!(
        found.is_empty(),
        "the kit's shipped bash exports a name of its families: {:?}",
        found, //
    );
}

/// The kennel the whistle laid in the tree this suite runs in — the binary a
/// course reading drives, as the substrate's doors drive it.
fn zbutq_kennel() -> std::path::PathBuf {
    let kennel = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../bk0/target/release/bkx");
    assert!(
        kennel.is_file(),
        "no kennel stands at {}; sound the whistle",
        kennel.display(), //
    );
    kennel
}

/// The kit's families, as the prefixes every name it speaks across an exec
/// wears.
const BUTQ_KIT_FAMILIES: &[&str] = &[
    "BUC_",
    "BURC_",
    "BURD_",
    "BURE_",
    "BURS_",
    "BURV_",
    "BURX_",
    "BUZ_", //
];

/// The pointer, the one name of the kit a course is handed.
const BUTQ_POINTER: &str = "BURD_BURD_FILE";

/// A decoy pair planted in the drive's own environment, which the leash's own
/// homes must stand over.
const BUTQ_DECOY_CARGO: &str = "/quobq-decoy/cargo";
const BUTQ_DECOY_RUSTUP: &str = "/quobq-decoy/rustup";

/// One course's recorded environment, as `NAME=value` lines.
fn zbutq_recorded(record: &std::path::Path, said: &str) -> Vec<(String, String)> {
    std::fs::read_to_string(record)
        .unwrap_or_else(|err| {
            panic!(
                "the case wrote nothing at {}: {}\n{}",
                record.display(),
                err,
                said, //
            )
        })
        .lines()
        .filter_map(|line| {
            line.split_once('=')
                .map(|(name, value)| (name.to_string(), value.to_string()))
        })
        .collect()
}

/// A course's record holds the planted control, the pointer, and no other name
/// of the kit.
fn zbutq_coursed(names: &[(String, String)]) {
    assert!(
        names.iter().any(|(name, _)| name == BUTQ_CONTROL),
        "the control: the reading holds the planted name: {:?}",
        names, //
    );
    assert!(
        names.iter().any(|(name, _)| name == BUTQ_POINTER),
        "the case is handed the pointer: {:?}",
        names, //
    );
    let kit: Vec<&String> = names
        .iter()
        .map(|(name, _)| name)
        .filter(|name| {
            *name != BUTQ_POINTER
                && BUTQ_KIT_FAMILIES
                    .iter()
                    .any(|family| name.starts_with(family))
        })
        .collect();
    assert!(
        kit.is_empty(),
        "the case was handed names of the kit beside the pointer: {:?}",
        kit, //
    );
}

/// A RUST COURSE IS HANDED THE POINTER AND NOTHING ELSE OF THE KIT, and the
/// tackroom's two homes over whatever the drive's own environment held. The
/// course's one case records its environment; the planted control is the proof
/// it read one, and the decoy pair the proof the homes are the leash's own.
#[test]
fn butq_a_rust_course_is_handed_the_pointer_alone_and_the_tackroom_homes() {
    let lure = bktu_Lure::bktu_compose("quiet-rust-course");
    lure.bktu_collar_seat();
    let record = lure.bktu_temp().join("rust-course-environment.txt");
    lure.bktu_write(
        "Tools/lure/src/lib.rs",
        &format!(
            "#[test]\nfn {}() {{\n    let mut said = String::new();\n    for (name, value) in std::env::vars_os() {{\n        said.push_str(&format!(\"{{}}={{}}\\n\", name.to_string_lossy(), value.to_string_lossy()));\n    }}\n    std::fs::write(\"{}\", said).expect(\"the case writes its environment\");\n}}\n",
            bktu_Lure::BKTU_CASE,
            record.display(), //
        ),
    );
    lure.bktu_commit("the case writes its environment");

    let out = lure.bktu_drive_stating(
        &zbutq_kennel(),
        &["mush", bktu_Lure::BKTU_COLLAR_CARGO],
        &[
            (BUTQ_CONTROL, std::ffi::OsStr::new("1")),
            ("CARGO_HOME", std::ffi::OsStr::new(BUTQ_DECOY_CARGO)),
            ("RUSTUP_HOME", std::ffi::OsStr::new(BUTQ_DECOY_RUSTUP)),
        ],
    );
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr), //
    );
    assert!(
        out.status.success(),
        "the rust drive runs green: {}",
        said, //
    );

    let names = zbutq_recorded(&record, &said);
    zbutq_coursed(&names);

    let tackroom = bkk::bktu_lure::bktu_suite_tackroom();
    for (home, under) in [("CARGO_HOME", "cargo"), ("RUSTUP_HOME", "rustup")] {
        let held = names
            .iter()
            .find(|(name, _)| name == home)
            .map(|(_, value)| std::path::PathBuf::from(value))
            .unwrap_or_else(|| {
                panic!(
                    "the case holds no {}: {:?}",
                    home,
                    names, //
                )
            });
        assert!(
            held.ends_with(under)
                && held.canonicalize().ok() == tackroom.join(under).canonicalize().ok(),
            "{} names the tackroom's {} over the decoy: {}",
            home,
            under,
            held.display(), //
        );
    }
}

/// The python course's pinned interpreter, and its collar.
const BUTQ_PYTHON_PIN: &str = "3.12.7";
const BUTQ_PY_COLLAR: &str = "suite-lure";

/// A python lure converged and committed: its project, its lock authored, its
/// environment synced — the road the kennel's own python drives take.
fn zbutq_pythoned(named: &str, case: &str) -> bktu_Lure {
    let lure = bktu_Lure::bktu_compose(named);
    lure.bktu_project(
        "project",
        BUTQ_PY_COLLAR,
        &bkk::bktu_lure::bktu_Bent {
            pin: BUTQ_PYTHON_PIN.to_string(),
            requires: ">=3.12".to_string(),
            lockless: true,
            baked: "pytest".to_string(),
            ..Default::default()
        },
    );
    lure.bktu_write(
        "project/pyproject.toml",
        "[project]\nname = \"lure\"\nversion = \"0.0.1\"\nrequires-python = \">=3.12\"\ndependencies = [\"pytest\"]\n",
    );
    lure.bktu_write("project/tests/test_quiet.py", case);
    lure.bktu_stationed().bktu_uv_seat(&lure);

    let dispatch = bkk::bkcr_resolve::bkcr_Dispatch::bkcr_read(&lure.bktu_dispatch_file())
        .expect("the lure's dispatch file reads");
    let collar = bkk::bkcx_python::bkcx_resolve(lure.bktu_root(), &dispatch, BUTQ_PY_COLLAR)
        .expect("the python collar resolves")
        .collar;
    let tackroom =
        bkk::bkcl_leash::bkcl_fenced(&dispatch).expect("the dispatch states the tackroom");
    let seat = bkk::bkcx_python::bkcx_seat_at(lure.bktu_loosebox(), &tackroom, &collar);
    let project = lure.bktu_root().join(collar.bkcx_project());

    for (posture, args) in [
        (
            bkk::bkcx_python::bkcx_Posture::Fetching,
            vec![
                std::ffi::OsString::from("python"),
                "install".into(),
                "--no-bin".into(),
                BUTQ_PYTHON_PIN.into(),
            ],
        ),
        (
            bkk::bkcx_python::bkcx_Posture::Reaching,
            vec![
                "lock".into(),
                "--project".into(),
                project.clone().into_os_string(),
            ],
        ),
    ] {
        let stated = bkk::bkcx_python::bkcx_stated(&seat, posture);
        let run = bkk::bkcl_leash::bkcl_uv(
            lure.bktu_root(),
            &dispatch,
            &project,
            &args,
            &bkk::bkcx_python::bkcx_borne(&stated),
        )
        .expect("uv is reached");
        assert!(
            run.bkcl_landed(),
            "the python lure did not stand: {}",
            run.spelling, //
        );
    }

    bkk::bkch_heel::bkch_python_at(lure.bktu_root(), &dispatch, &collar, &seat).unwrap_or_else(
        |err| {
            panic!(
                "the python lure would not converge: {}",
                err, //
            )
        },
    );
    lure.bktu_commit("the converged python lure");
    lure
}

/// A PYTHON COURSE IS HANDED THE POINTER AND NOTHING ELSE OF THE KIT, the
/// environment regime included, behind the same planted control.
#[test]
fn butq_a_python_course_is_handed_the_pointer_alone() {
    let record = bkk::bktu_lure::bktu_temp_root().join("quiet-python-course-environment.txt");
    let _ = std::fs::remove_file(&record);
    let lure = zbutq_pythoned(
        "quiet-python-course",
        &format!(
            "import os\n\n\ndef test_writes():\n    with open({:?}, \"w\") as said:\n        for name, value in os.environ.items():\n            said.write(name + \"=\" + value + \"\\n\")\n",
            record.display().to_string(), //
        ),
    );

    let out = lure.bktu_drive_stating(
        &zbutq_kennel(),
        &["mush", BUTQ_PY_COLLAR],
        &[(BUTQ_CONTROL, std::ffi::OsStr::new("1"))],
    );
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr), //
    );
    assert!(
        out.status.success(),
        "the python drive runs green: {}",
        said, //
    );

    zbutq_coursed(&zbutq_recorded(&record, &said));
}
