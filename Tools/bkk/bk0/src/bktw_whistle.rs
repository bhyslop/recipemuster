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

//! The kit's closure: what this kit's bash is allowed to reach.
//!
//! THE RULE. No file under this kit names a path outside the kit and the
//! substrate in a line the shell executes. A parcel carries this kit's files and
//! the substrate's; a source line reaching for a third kit resolves to nothing on
//! a receiving station, and bash's `source` of an absent file is fatal. So such a
//! line does not degrade the door — it kills the bootstrap at the line, ahead of
//! every refusal the door was written to give, and the station is told about a
//! missing file rather than about its own tree.
//!
//! WHY A HURDLE AND NOT A REVIEW. The breach costs nothing where it is written
//! and everything where it lands: the estate's own seat carries every kit, so a
//! line reaching into a third one works perfectly here and forever, and the
//! defect is observable only on a station nobody in this repository is standing
//! at. That is precisely the shape a reviewer cannot hold and a hurdle can.
//!
//! IT READS THE TREE RATHER THAN A LIST. A roster of the kit's bash files would
//! have to be kept in step with the kit, and the failure of an un-kept roster is
//! silence: a file added and not listed is a file not checked, and the hurdle
//! goes on passing. The walk finds every `.sh` this kit carries, so a new door
//! is under the rule the moment it exists.
//!
//! WHAT IT DOES NOT READ IS PROSE. A comment is free to name any file in the
//! estate and several do, citing a pattern or a home; what the rule governs is a
//! line the shell runs. So the reading is of `source` and `.` statements, and a
//! citation in a doc comment is left alone.

use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use crate::bkcl_leash::{
    BKCL_CARGO_DIR,
    BKCL_CONVERGE_SENTENCE,
    BKCL_RUSTUP_DIR, //
};
use crate::bkco_output::BKCO_COLOPHON_REBED;
use crate::bktu_lure::{
    bktu_Lure,
    bktu_answering,
    bktu_answering_its_channel,
    bktu_answering_marked,
    bktu_toolchain_gutted,
    bktu_toolchain_laid,
    BKTU_PLANTED_HOST,
    BKTU_TACKROOM_STEM, //
};

/// The two directory names a kit file's source line may name: this kit's own,
/// and the substrate's, which every parcel carries beside it.
const BKTW_ADMITTED: &[&str] = &[
    "bkk",
    "buk", //
];

/// The kit's own directory, which is this crate's manifest directory.
fn zbktw_kit() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Every `.sh` file the kit carries, walked rather than listed.
fn zbktw_bash(at: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(at) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // Cargo's own scratch is not the kit's source, and it is large.
        if path.file_name().is_some_and(|name| name == "target") {
            continue;
        }

        if path.is_dir() {
            zbktw_bash(&path, found);
        } else if path.extension().is_some_and(|ext| ext == "sh") {
            found.push(path);
        }
    }
}

/// Whether one line is a shell statement that sources another file.
///
/// BOTH SPELLINGS, because bash reads them identically and a rule that governed
/// only the word would be evaded by the dot without anyone intending to evade
/// it.
fn zbktw_sources(line: &str) -> bool {
    let trimmed = line.trim_start();

    if trimmed.starts_with('#') {
        return false;
    }

    trimmed.starts_with("source ") || trimmed.starts_with(". ")
}

/// The kit directory a source line reaches, where it reaches outside this kit.
///
/// THE READING IS OF THE PATH'S SHAPE AND NOT OF ITS RESOLUTION. A line's path is
/// composed at run time out of variables the hurdle does not hold, so what can be
/// read here is which kit DIRECTORY NAME it spells — which is exactly what the
/// rule is about. A line naming no kit directory at all reaches nothing this rule
/// governs.
fn zbktw_reaches(line: &str) -> Option<String> {
    // THE PATH ENDS WHERE THE GUARD BEGINS. Every source line carries its own
    // guard, and the guard's words — an exit code, a die message — name no kit
    // directory, so only what stands ahead of the first `||` is read.
    let line = line.split("||").next().unwrap_or("");

    for token in line.split(['/', '"', '\'', '{', '}', ' ']) {
        if token.is_empty() {
            continue;
        }
        if BKTW_ADMITTED.contains(&token) {
            return None;
        }
    }

    // Every kit stands as one directory under the tools root, so a source line
    // leaving this kit spells that directory's name between slashes.
    let mut said = None;
    for token in line.split('/') {
        let name = token.rsplit(['"', '\'', '}', ' ']).next().unwrap_or("");
        if name.is_empty() || name.contains('$') || name.contains('.') {
            continue;
        }
        if !BKTW_ADMITTED.contains(&name) {
            said = Some(name.to_string());
        }
    }
    said
}

#[test]
fn bktw_the_reading_stops_at_the_guard() {
    assert_eq!(
        zbktw_reaches(r#"source "${BURD_BUK_DIR}/buc_command.sh" || exit 1"#),
        None,
        "a guarded line inside the substrate reaches nothing", //
    );
    assert_eq!(
        zbktw_reaches(r#"  source "${BURD_TOOLS_DIR}/zzfrob/zzfrob_x.sh" || exit 1"#),
        Some("zzfrob".to_string()),
        "a guarded line leaving the kit still names the directory it reaches", //
    );
}

#[test]
fn bktw_no_kit_bash_sources_a_file_outside_the_kit_and_the_substrate() {
    let kit = zbktw_kit();

    let mut bash = Vec::new();
    zbktw_bash(&kit, &mut bash);

    // THE COVERAGE IS ASSERTED BESIDE THE FINDING, because finding nothing is
    // this reading's ordinary answer and a zero-hit reading is valid only behind
    // a control. A walk that reached no file at all would report exactly the
    // clean verdict a closed kit reports, and the two would be indistinguishable.
    assert!(
        bash.len() >= 4,
        "the walk reached the kit's bash: {} file(s) found under {}",
        bash.len(),
        kit.display(), //
    );

    let mut breaches = Vec::new();
    let mut sourced = 0usize;

    for path in &bash {
        let Ok(text) = std::fs::read_to_string(path) else {
            panic!(
                "the kit carries {} but it could not be read",
                path.display(), //
            );
        };

        for (index, line) in text.lines().enumerate() {
            if !zbktw_sources(line) {
                continue;
            }
            sourced += 1;

            if let Some(outside) = zbktw_reaches(line) {
                breaches.push(format!(
                    "{}:{} reaches {}: {}",
                    path.display(),
                    index + 1,
                    outside,
                    line.trim(), //
                ));
            }
        }
    }

    // THE SECOND HALF OF THE CONTROL. The walk found files; this says it found
    // source lines IN them, so a reading defeated by a parse rather than by a
    // path reports as a defeat instead of as a pass.
    assert!(
        sourced >= 4,
        "the reading found source lines to judge: {} across {} file(s)",
        sourced,
        bash.len(), //
    );

    assert!(
        breaches.is_empty(),
        "every source line in this kit names the kit or the substrate, because a parcel \
         carries those two and nothing else - a line reaching further kills the bootstrap \
         at the source line on every receiving station, ahead of every refusal it was \
         written to give:\n  {}",
        breaches.join("\n  "), //
    );
}

/// The kit's bash file holding the channel's proof in the bash register.
const ZBKTW_TACKROOM_BASH: &str = "bkcb_tackroom.sh";

/// THE CONVERGE SENTENCE IS SPELLED ALIKE IN BOTH REGISTERS. A reader meeting a
/// toolchain that cannot answer is told the same thing at the whistle, in bash,
/// as at the leash, in rust; each register spells it once, and this holds the
/// bash spelling to the rust one by reading the kit's file from the crate's own
/// directory, as the closure reading above does.
#[test]
fn bktw_the_converge_sentence_is_spelled_alike_in_both_registers() {
    let path = zbktw_kit().join(ZBKTW_TACKROOM_BASH);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "the kit carries {}: {}",
            path.display(),
            err, //
        )
    });

    assert!(
        text.contains(BKCL_CONVERGE_SENTENCE),
        "{} spells the converge sentence exactly as the leash does: {:?}",
        path.display(),
        BKCL_CONVERGE_SENTENCE, //
    );
    assert!(
        text.contains(BKCO_COLOPHON_REBED),
        "{} names the door by its colophon, {}",
        path.display(),
        BKCO_COLOPHON_REBED, //
    );
}

/// THE TWO HOMES BENEATH THE TACKROOM ARE SPELLED ALIKE IN BOTH REGISTERS. The
/// whistle's bash composes them for its own cargo and rustup acts before the
/// kennel exists, and the leash derives them for every command it composes; a
/// home spelled differently at either seat would be a second store the other
/// never fills. This holds each bash composition to the rust spelling, read from
/// the kit's file as the converge sentence's hurdle above reads it.
#[test]
fn bktw_the_tackroom_homes_are_spelled_alike_in_both_registers() {
    let path = zbktw_kit().join(ZBKTW_TACKROOM_BASH);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        panic!(
            "the kit carries {}: {}",
            path.display(),
            err, //
        )
    });

    for (bash, dir) in [
        ("BKCB_CARGO_HOME", BKCL_CARGO_DIR),
        ("BKCB_RUSTUP_HOME", BKCL_RUSTUP_DIR),
    ] {
        let composed = format!(
            "readonly {}=\"${{ZBKCB_TACKROOM}}/{}\"",
            bash,
            dir, //
        );
        assert!(
            text.contains(&composed),
            "{} composes {} beneath the tackroom as the leash does: {}",
            path.display(),
            bash,
            composed, //
        );
    }
}

/// The channel a seat's posed tackroom lays, the toolchain directory it lays it
/// under, and the posed kit directory a door seat's pin stands in: made-up, so
/// nothing a seat poses could be a station's.
const ZBKTW_POSED_CHANNEL: &str = "7.7.7";
const ZBKTW_POSED_TOOLCHAIN: &str = "7.7.7-frabjous-host";
const ZBKTW_POSED_TOOLS: &str = "lt";

/// What a seat's coordinator says once the proof has passed.
const ZBKTW_ANSWERED: &str = "the proof passed and the coordinator went on";

/// Rustup's own control over fetching a toolchain a proxy is told to run,
/// spelled as rustup's notice names it: "you may opt out with
/// `RUSTUP_AUTO_INSTALL=0`". Stated off on every drive here that reaches a
/// rustup program, the hurdle's own included, so nothing these hurdles reach
/// can fetch.
const ZBKTW_RUSTUP_AUTO_INSTALL: (&str, &str) = ("RUSTUP_AUTO_INSTALL", "0");

/// The sourcing every seat coordinator below opens with: the substrate's command
/// and voice modules, then the kit's own, from the tree the suite runs in. The
/// kit's own stand beside the substrate under the kit directory, the one name
/// of the dispatch a pointerless child still holds.
const ZBKTW_COORDINATOR_HEAD: &str = "#!/bin/bash\n\
     set -euo pipefail\n\
     source \"${BURD_BUK_DIR}/buc_command.sh\"\n\
     source \"${BURD_BUK_DIR}/buym_yelp.sh\"\n\
     source \"${BURD_BUK_DIR}/../bkk/bk0/bkcp_position.sh\"\n\
     source \"${BURD_BUK_DIR}/../bkk/bk0/bkcb_tackroom.sh\"\n\
     source \"${BURD_BUK_DIR}/../bkk/bk0/bkcw_whistle.sh\"\n\
     zbkcb_kindle\n";

/// Lay a substrate seat whose station file poses a tackroom of the hurdle's own,
/// and hand back the seat's tabtarget and that tackroom.
///
/// THE STATION FILE IS THE HURDLE'S TO WRITE, which is how its tackroom is
/// posed: the seat's own declaration is rewritten with the tackroom beside the
/// lure, so the fence the coordinator stands on exports homes under it and the
/// station's real store is reached by nothing.
fn zbktw_seat(lure: &bktu_Lure, name: &str, body: &str) -> (PathBuf, PathBuf) {
    let tabtarget = lure.bktu_substrate_seat(&format!(
        "{}{}",
        ZBKTW_COORDINATOR_HEAD,
        body, //
    ));
    let tackroom = lure.bktu_tackroom(name);

    lure.bktu_write(
        ".buk/burs.env",
        &format!(
            "BURS_LOG_DIR=logs\nBURS_USER=kennel\nBURS_TINCTURE=k\nBURS_TACKROOM=../{}{}\n",
            BKTU_TACKROOM_STEM,
            name, //
        ),
    );
    lure.bktu_commit("pose the seat's own tackroom");

    (tabtarget, tackroom)
}

/// The coordinator body driving the whistle's own proof over the posed channel.
fn zbktw_proof_body() -> String {
    format!(
        "bkcb_ensure\nbkcb_channel_answers_or_die {}\necho \"{}\"\n",
        ZBKTW_POSED_CHANNEL,
        ZBKTW_ANSWERED, //
    )
}

/// THE POSITIVE CONTROL for the whistle's refusal: a toolchain whose compiler
/// answers the pinned release passes the bash proof, and the coordinator goes on.
#[test]
fn bktw_a_toolchain_whose_compiler_answers_passes_the_whistles_proof() {
    let lure = bktu_Lure::bktu_compose("whistle-answers");
    let (tabtarget, tackroom) = zbktw_seat(&lure, "whistle-answers", &zbktw_proof_body());
    bktu_toolchain_laid(
        &tackroom,
        ZBKTW_POSED_TOOLCHAIN,
        Some(&bktu_answering(ZBKTW_POSED_CHANNEL)),
    );

    let out = lure.bktu_dispatch_stating(&tabtarget, &[], &[zbktw_auto_install_off()]);
    let said = String::from_utf8_lossy(&out.stdout);

    assert_eq!(
        out.status.code(),
        Some(0),
        "the proof passes: {}",
        said, //
    );
    assert!(
        said.contains(ZBKTW_ANSWERED),
        "the coordinator went on past the proof: {}",
        said, //
    );
}

/// A GUTTED TOOLCHAIN REFUSES AT THE WHISTLE, naming the door: the directory
/// rustup's ledger lists stands, its compiler does not, and the bash proof says
/// so before cargo is ever reached.
#[test]
fn bktw_a_gutted_toolchain_refuses_at_the_whistle_naming_the_door() {
    let lure = bktu_Lure::bktu_compose("whistle-gutted");
    let (tabtarget, tackroom) = zbktw_seat(&lure, "whistle-gutted", &zbktw_proof_body());
    bktu_toolchain_laid(&tackroom, ZBKTW_POSED_TOOLCHAIN, None);

    let out = lure.bktu_dispatch_stating(&tabtarget, &[], &[zbktw_auto_install_off()]);
    let said = String::from_utf8_lossy(&out.stdout);

    assert_ne!(
        out.status.code(),
        Some(0),
        "a gutted toolchain is refused: {}",
        said, //
    );
    assert!(
        said.contains(BKCL_CONVERGE_SENTENCE) && said.contains(BKCO_COLOPHON_REBED),
        "the refusal carries the converge sentence and names the door: {}",
        said, //
    );
    assert!(
        !said.contains(ZBKTW_ANSWERED),
        "nothing ran past the refusal: {}",
        said, //
    );
}

/// A TOOLCHAIN WHOSE DRIVER LIBRARY IS GONE REFUSES AT THE WHISTLE WITH NO
/// COMPILER SPAWNED. Its compiler stands and would answer the pin if it could
/// load, and it leaves a mark when spawned; the proof reads the library's
/// absence first, so the refusal names the door and the mark is never left —
/// which is what keeps a station's crash dialog from being raised by a proof.
#[test]
fn bktw_a_toolchain_whose_driver_library_is_gone_refuses_spawning_no_compiler() {
    let lure = bktu_Lure::bktu_compose("whistle-unlibraried");
    let (tabtarget, tackroom) = zbktw_seat(&lure, "whistle-unlibraried", &zbktw_proof_body());
    let mark = tackroom.join("spawned");
    bktu_toolchain_gutted(
        &tackroom,
        ZBKTW_POSED_TOOLCHAIN,
        &bktu_answering_marked(ZBKTW_POSED_CHANNEL, &mark),
    );

    let out = lure.bktu_dispatch_stating(&tabtarget, &[], &[zbktw_auto_install_off()]);
    let said = String::from_utf8_lossy(&out.stdout);

    assert_ne!(
        out.status.code(),
        Some(0),
        "a toolchain without its driver library is refused: {}",
        said, //
    );
    assert!(
        said.contains(BKCL_CONVERGE_SENTENCE) && said.contains(BKCO_COLOPHON_REBED),
        "the refusal carries the converge sentence and names the door: {}",
        said, //
    );
    assert!(
        !mark.exists(),
        "no compiler was spawned on the way to the refusal: {}",
        said, //
    );
    assert!(
        !said.contains(ZBKTW_ANSWERED),
        "nothing ran past the refusal: {}",
        said, //
    );
}

/// The second channel a door seat's posed hurdles demand beside the pin, and
/// the declaration naming both, written where rebed reads the hurdles' own: the
/// pin listed first and again, as the estate's declaration lists it, so the
/// door is proven to lay it once.
const ZBKTW_POSED_SECOND: &str = "7.7.9";
const ZBKTW_POSED_DECLARATION: &str =
    "pub const BKTU_REQUIRED_TOOLCHAINS: &[&str] = &[\"7.7.7\", \"7.7.9\"];\n";

/// The coordinator body driving the door's toolchain half over a posed kit pin
/// — the half that stands in bash, the rest being the binary's.
///
/// THE HALF RUNS IN A CHILD OF THE COORDINATOR, whose opening is the coordinator's
/// own with the dispatch pointer withdrawn, so nothing is kindled there from a
/// dispatch and nothing is locked. The child is a harness sourcing the kit for its
/// functions alone, so what it reads it assigns in its own script: the tackroom
/// and the temp directory the coordinator hands it by argument, and the posed
/// tools directory, never assigned over the locked config regime.
fn zbktw_door_body() -> String {
    format!(
        "z_child=\"${{BURD_TEMP_DIR}}/bktw-rebed.sh\"\n\
         cat > \"${{z_child}}\" <<'BKTW_CHILD'\n\
         #!/bin/bash\n\
         BURD_TACKROOM=\"$1\"\n\
         BURD_TEMP_DIR=\"$2\"\n\
         BURC_TOOLS_DIR={posed}\n\
         {head}\
         zbkcw_kindle\n\
         zbkcw_rebed_toolchains\n\
         BKTW_CHILD\n\
         exec env -u BURD_BURD_FILE bash \"${{z_child}}\" \"${{BURD_TACKROOM}}\" \"${{BURD_TEMP_DIR}}\"\n",
        head = ZBKTW_COORDINATOR_HEAD.trim_start_matches("#!/bin/bash\n"),
        posed = ZBKTW_POSED_TOOLS, //
    )
}

/// Lay a door seat: the posed tackroom holding a gutted toolchain for the
/// pin's channel, a posed kit pin naming it, the hurdles' posed declaration
/// where `declared` asks for one, and a rustup of the hurdle's own ahead on the
/// path whose install lays a toolchain whose compiler runs `compiler`.
fn zbktw_door_seat(
    name: &str,
    declared: bool,
    compiler: &str,
) -> (bktu_Lure, PathBuf, PathBuf, std::ffi::OsString, PathBuf) {
    let lure = bktu_Lure::bktu_compose(name);
    lure.bktu_write(
        &format!(
            "{}/bkk/bk0/rust-toolchain.toml",
            ZBKTW_POSED_TOOLS, //
        ),
        &format!(
            "[toolchain]\nchannel = \"{}\"\n",
            ZBKTW_POSED_CHANNEL, //
        ),
    );
    if declared {
        lure.bktu_write(
            &format!(
                "{}/bkk/bk0/src/bktu_lure.rs",
                ZBKTW_POSED_TOOLS, //
            ),
            ZBKTW_POSED_DECLARATION,
        );
    }
    let (tabtarget, tackroom) = zbktw_seat(&lure, name, &zbktw_door_body());
    bktu_toolchain_laid(&tackroom, ZBKTW_POSED_TOOLCHAIN, None);
    let (path, record) = lure.bktu_rustup_shimmed(name, compiler);
    (lure, tabtarget, tackroom, path, record)
}

/// What the planted rustup recorded, each line split into the home it was
/// handed — resolved, since the fence composes it through the seat's root — and
/// the arguments it was asked.
fn zbktw_asked(record: &Path) -> Vec<(PathBuf, String)> {
    std::fs::read_to_string(record)
        .unwrap_or_default()
        .lines()
        .map(|line| {
            let (home, asked) = line.split_once(" toolchain ").unwrap_or_else(|| {
                panic!(
                    "the planted rustup records home and arguments: {}",
                    line, //
                )
            });
            (
                Path::new(home)
                    .canonicalize()
                    .unwrap_or_else(|_| PathBuf::from(home)),
                format!(
                    "toolchain {}",
                    asked, //
                ),
            )
        })
        .collect()
}

/// THE DOOR'S SEQUENCE: the fence stood on, then for the pin's channel and
/// again for the other channel the hurdles demand — the pin's named twice in
/// the declaration and laid once — the channel uninstalled and then installed
/// at the minimal profile with clippy and rustfmt, every act asked of the posed
/// tackroom's rustup home, and each channel's own proof read off a planted
/// compiler answering as itself, its success said by channel.
#[test]
fn bktw_rebed_lays_the_pin_and_every_channel_the_hurdles_demand() {
    let (lure, tabtarget, tackroom, path, record) =
        zbktw_door_seat("rebed-sequence", true, &bktu_answering_its_channel());

    let out = lure.bktu_dispatch_stating(
        &tabtarget,
        &[],
        &[("PATH", path.as_os_str()), zbktw_auto_install_off()],
    );
    let said = String::from_utf8_lossy(&out.stdout);
    let told = String::from_utf8_lossy(&out.stderr);

    assert_eq!(
        out.status.code(),
        Some(0),
        "the door lays every channel and each proof passes: {}",
        said, //
    );

    let home = tackroom
        .join("rustup")
        .canonicalize()
        .expect("the posed tackroom's rustup home stands");
    let laid = |channel: &str| {
        vec![
            (
                home.clone(),
                format!(
                    "toolchain uninstall {}",
                    channel, //
                ),
            ),
            (
                home.clone(),
                format!(
                    "toolchain install {} --profile minimal --component clippy --component rustfmt \
                     --no-self-update",
                    channel, //
                ),
            ),
        ]
    };
    let mut wanted = laid(ZBKTW_POSED_CHANNEL);
    wanted.extend(laid(ZBKTW_POSED_SECOND));

    assert_eq!(
        zbktw_asked(&record),
        wanted,
        "the door lays the pin and then the other demanded channel, once each, under the \
         tackroom's rustup home: {}",
        said, //
    );

    for channel in [ZBKTW_POSED_CHANNEL, ZBKTW_POSED_SECOND] {
        let line = format!(
            "The tackroom's {} is laid again, and its compiler answers",
            channel, //
        );
        assert!(
            said.contains(&line) || told.contains(&line),
            "the door says each channel answers: {}\n{}",
            said,
            told, //
        );
    }
}

/// REBED LAYS ONLY WHAT DOES NOT ANSWER, which is what makes it safe to drive at
/// any time. The pin's channel stands whole and answers; the other demanded
/// channel stands gutted, its compiler present and its driver library gone, and
/// that compiler leaves a mark when spawned. The door asks rustup nothing of the
/// pin's channel, uninstalls and installs the gutted one alone, and never spawns
/// the gutted compiler on the way to deciding so.
#[test]
fn bktw_rebed_relays_only_the_channel_that_does_not_answer() {
    let (lure, tabtarget, tackroom, path, record) = zbktw_door_seat(
        "rebed-only-unanswering",
        true,
        &bktu_answering_its_channel(),
    );
    bktu_toolchain_laid(
        &tackroom,
        ZBKTW_POSED_TOOLCHAIN,
        Some(&bktu_answering(ZBKTW_POSED_CHANNEL)),
    );
    let mark = tackroom.join("spawned");
    bktu_toolchain_gutted(
        &tackroom,
        &format!(
            "{}-{}",
            ZBKTW_POSED_SECOND,
            BKTU_PLANTED_HOST, //
        ),
        &bktu_answering_marked(ZBKTW_POSED_SECOND, &mark),
    );

    let out = lure.bktu_dispatch_stating(
        &tabtarget,
        &[],
        &[("PATH", path.as_os_str()), zbktw_auto_install_off()],
    );
    let said = String::from_utf8_lossy(&out.stdout);
    let told = String::from_utf8_lossy(&out.stderr);

    assert_eq!(
        out.status.code(),
        Some(0),
        "the door lays what does not answer and ends green: {}\n{}",
        said,
        told, //
    );

    let asked: Vec<String> = zbktw_asked(&record)
        .into_iter()
        .map(|(_, asked)| asked)
        .collect();
    assert_eq!(
        asked,
        vec![
            format!(
                "toolchain uninstall {}",
                ZBKTW_POSED_SECOND, //
            ),
            format!(
                "toolchain install {} --profile minimal --component clippy --component rustfmt \
                 --no-self-update",
                ZBKTW_POSED_SECOND, //
            ),
        ],
        "rustup was asked of the gutted channel alone: {}\n{}",
        said,
        told, //
    );
    assert!(
        !mark.exists(),
        "the gutted compiler was never spawned: {}\n{}",
        said,
        told, //
    );
}

/// THE ARMED NEGATIVE: the same drive, its planted install answering another
/// release, and the door's own proof refuses naming the door — after the pin's
/// install, and before the second channel is touched.
#[test]
fn bktw_rebed_refuses_where_the_laid_compiler_answers_another_release() {
    let (lure, tabtarget, _tackroom, path, record) =
        zbktw_door_seat("rebed-other-release", true, &bktu_answering("7.7.8"));

    let out = lure.bktu_dispatch_stating(
        &tabtarget,
        &[],
        &[("PATH", path.as_os_str()), zbktw_auto_install_off()],
    );
    let said = String::from_utf8_lossy(&out.stdout);

    assert_ne!(
        out.status.code(),
        Some(0),
        "a laid compiler answering another release is refused: {}",
        said, //
    );
    assert!(
        said.contains("7.7.8") && said.contains(BKCO_COLOPHON_REBED),
        "the refusal names what answered and the door: {}",
        said, //
    );
    assert_eq!(
        zbktw_asked(&record).len(),
        2,
        "the refusal came after the pin's install: {}",
        said, //
    );
}

/// A KIT WHOSE HURDLES' DECLARATION CANNOT BE READ IS REFUSED BEFORE ANYTHING
/// IS TAKEN DOWN. A door that re-laid the pin alone on that ground would call a
/// tackroom whole that the hurdles then found short, so it asks rustup nothing
/// and names the declaration it could not read.
#[test]
fn bktw_rebed_refuses_an_unread_demand_before_it_uninstalls_anything() {
    let (lure, tabtarget, _tackroom, path, record) =
        zbktw_door_seat("rebed-undeclared", false, &bktu_answering_its_channel());

    let out = lure.bktu_dispatch_stating(
        &tabtarget,
        &[],
        &[("PATH", path.as_os_str()), zbktw_auto_install_off()],
    );
    let said = String::from_utf8_lossy(&out.stdout);

    assert_ne!(
        out.status.code(),
        Some(0),
        "an unread demand is refused: {}",
        said, //
    );
    assert!(
        said.contains("BKTU_REQUIRED_TOOLCHAINS"),
        "the refusal names the declaration it could not read: {}",
        said, //
    );
    assert!(
        zbktw_asked(&record).is_empty(),
        "rustup was asked nothing: {}",
        said, //
    );
}

/// Rustup's auto-install control, stated off, as a drive's stated pair.
fn zbktw_auto_install_off() -> (&'static str, &'static OsStr) {
    (
        ZBKTW_RUSTUP_AUTO_INSTALL.0,
        OsStr::new(ZBKTW_RUSTUP_AUTO_INSTALL.1),
    )
}

// eof
