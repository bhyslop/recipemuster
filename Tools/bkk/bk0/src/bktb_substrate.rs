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

//! The substrate driven through a seat the composing surface laid down.
//!
//! ONE HURDLE, AND IT PROVES THE SEAT RATHER THAN THE SUBSTRATE. What is under
//! test here is that a composed tree is a tree the substrate will dispatch in:
//! the trampoline finds the moorings, the stub reaches the kit standing outside
//! the seat, the station file answers the BURS regime, and the coordinator is
//! reached with the colophon the inscription spells. The substrate's own
//! behaviour is the next pace's, and it builds on this.
//!
//! OBSERVED FROM OUTSIDE THE SHELL. The exit code and both streams are read in
//! this process, which no part of the dispatch can reach — the posture law
//! (BKSOB-Obedience.adoc "Observation Posture"), whose whole point is that a
//! harness sharing a failure domain with the thing under test reports silence as
//! success.
//!
//! THE EXIT CODE IS A DISTINCTIVE ONE, and deliberately not zero. A door that
//! exited zero whatever its coordinator did would clear a success assertion, so
//! a zero here would prove the dispatch ran and not that it carried the
//! coordinator's verdict back.

use std::path::PathBuf;

use crate::bktu_lure::bktu_Lure;
use crate::bktu_lure::BKTU_COLOPHON;
use crate::bkg_breviary::bkg_burc_loosebox_root_dir_base;
use crate::bkg_breviary::BKG_BURC_LOOSEBOX_ROOT_DIR_BASE;

/// What the seat's coordinator writes to stdout, ahead of the colophon it was
/// handed.
const BKTB_SAID: &str = "the coordinator stood at";

/// What the seat's coordinator writes to stderr. The dispatch's logged mode
/// merges the coordinator's streams into one record, so this is asserted on
/// STDOUT — that merge is the substrate's own law, and the assertion is what
/// reads it from outside.
const BKTB_GRUMBLED: &str = "the coordinator grumbled";

/// The verdict the coordinator takes. Outside the substrate's precision band, so
/// nothing in the kit can be its author.
const BKTB_VERDICT: i32 = 7;

/// The line the dispatch announces its log family on.
const BKTB_ANNOUNCEMENT: &str = "log files:";

/// The line the dispatch announces its scratch on.
const BKTB_TRANSCRIPT: &str = "transcript:";

/// The line the dispatch announces this checkout's loosebox on.
const BKTB_LOOSEBOX: &str = concat!(
    bkg_burc_loosebox_root_dir_base!(),
    ":", //
);

/// The line the dispatch announces its output directory on.
const BKTB_OUTPUT: &str = "output dir:";

/// The loosebox root the composed seat's own regime file declares, relative to
/// the seat's root exactly as its temp and output roots are.
const BKTB_LOOSEBOX_ROOT: &str = BKG_BURC_LOOSEBOX_ROOT_DIR_BASE;

#[test]
fn bktb_the_substrate_dispatches_in_a_composed_seat() {
    let lure = bktu_Lure::bktu_compose("substrate-seat");

    let tabtarget = lure.bktu_substrate_seat(&format!(
        "#!/bin/bash\n\
         set -euo pipefail\n\
         echo \"{said} ${{1}}\"\n\
         echo \"{grumbled}\" >&2\n\
         exit {verdict}\n",
        said = BKTB_SAID,
        grumbled = BKTB_GRUMBLED,
        verdict = BKTB_VERDICT
    ));

    let out = lure.bktu_dispatch(&tabtarget, &[]);
    let said = String::from_utf8_lossy(&out.stdout);
    let grumbled = String::from_utf8_lossy(&out.stderr);

    // THE VERDICT CARRIED. The whole chain — trampoline, stub, shared launcher,
    // dispatch, coordinator — stands between the exit this asserts and the exit
    // the coordinator took, so this is the one assertion that says the seat is a
    // seat rather than a directory that happens to hold scripts.
    assert_eq!(
        out.status.code(),
        Some(BKTB_VERDICT),
        "the seat carries the coordinator's own verdict back: {}\n{}",
        said,
        grumbled
    );

    // THE COLOPHON REACHED THE COORDINATOR. The dispatch splits it out of the
    // tabtarget's inscription and hands it over as the first argument, so a
    // coordinator that can name it was reached through the seat's own door and
    // not by some shorter road.
    assert!(
        said.contains(&format!("{} {}", BKTB_SAID, BKTU_COLOPHON)),
        "the coordinator names the colophon the inscription spells: {}",
        said
    );

    // BOTH STREAMS, READ WHERE THE LAW PUTS THEM. The logged mode merges the
    // coordinator's stderr into the record it tees, so what the coordinator
    // grumbled arrives on the child's stdout and the child's own stderr stands
    // empty. Asserting both is what tells a merge from a lost diagnostic.
    assert!(
        said.contains(BKTB_GRUMBLED),
        "the dispatch merges the coordinator's stderr into the record: {}",
        said
    );
    assert!(
        grumbled.is_empty(),
        "nothing in the chain wrote past the record: {}",
        grumbled
    );

    // THE SEAT'S OWN LOG FAMILY, WRITTEN WHERE THE SEAT PUT IT. This is the
    // falsification of the hazard the dispatch guards against by stripping the
    // regimes it inherited: a seat that took the suite's own would have written
    // its record into the tree under test, and every door would still have
    // reported success. The three announced names are read relative to the seat's
    // root, which is how the station file spells a log directory.
    let root = lure.bktu_root();
    let announced = zbktb_announced(&said, BKTB_ANNOUNCEMENT);

    let mut family = 0;
    for name in announced.split_whitespace() {
        assert!(
            root.join(name).is_file(),
            "the seat's own log directory holds {}, under {}",
            name,
            root.display()
        );
        family += 1;
    }
    assert_eq!(
        family, 3,
        "a dispatch that logs writes three files, one record read three ways: {}",
        announced
    );

    // AND ITS OWN SCRATCH. The transcript is announced absolute, the dispatch
    // absolutizing a relative temp root against the repository root it was
    // normalized to, so this one names the seat outright.
    assert!(
        zbktb_announced(&said, BKTB_TRANSCRIPT).starts_with(&root.display().to_string()),
        "the seat's own temp root carries the dispatch's scratch: {}",
        said
    );
}

/// The loosebox the substrate composes for a seat, keyed on that seat's own
/// dirname.
///
/// THE KEY IS WHAT IS UNDER TEST, not the root. A root is a value the regime
/// file states and a harness may pose; the key beneath it is the substrate's own
/// composition, and it is the whole reason two checkouts sharing one root never
/// hand each other their build products. Asserting the root alone would pass
/// over exactly the composition this reads.
///
/// DRIVEN THROUGH A REAL DISPATCH rather than by reading the script, because the
/// value is composed from the dispatch's own working directory: a reading that
/// recomposed it here would be a second implementation agreeing with itself.
#[test]
fn bktb_the_substrate_composes_a_loosebox_keyed_on_the_seat() {
    let lure = bktu_Lure::bktu_compose("substrate-loosebox");

    let tabtarget = lure.bktu_substrate_seat("#!/bin/bash\nexit 0\n");

    let out = lure.bktu_dispatch(&tabtarget, &[]);
    let said = String::from_utf8_lossy(&out.stdout);

    let root = lure.bktu_root();
    let key = root
        .file_name()
        .expect("the lure's seat stands in a named directory");

    let announced = PathBuf::from(zbktb_announced(&said, BKTB_LOOSEBOX));

    assert_eq!(
        announced,
        root.join(BKTB_LOOSEBOX_ROOT).join(key),
        "the {} stands beneath the declared root under the seat's own dirname: {}",
        BKG_BURC_LOOSEBOX_ROOT_DIR_BASE,
        said, //
    );

    // AND IT STANDS. The composition is worth nothing if the dispatch names a
    // directory it did not make — every door beneath it would meet an absent
    // path and the refusal would read as a misprovisioned seat.
    assert!(
        announced.is_dir(),
        "the dispatch makes the {} it announces: {}",
        BKG_BURC_LOOSEBOX_ROOT_DIR_BASE,
        announced.display(), //
    );
}

/// The seat's own default regime, with each of the three roots a hurdle can
/// pose — temp, output, loosebox — respelled through a parent segment: the
/// shape a station's own regime states (`BURC_TEMP_ROOT_DIR=../temp-buk` and
/// its siblings), a sibling of the checkout rather than a subdirectory
/// beneath it. A kraal's own regime always states absolute roots, so this is
/// the one shape no kraal-driven hurdle exercises on its own; the lure poses
/// it explicitly here.
fn zbktb_posed_parent_regime(lure: &bktu_Lure) -> String {
    let regime_path = lure.bktu_root().join(".buk/burc.env");
    let standing = std::fs::read_to_string(&regime_path).unwrap_or_else(|err| {
        panic!(
            "could not read the seat's own regime at {}: {}",
            regime_path.display(),
            err
        )
    });

    let posed = standing
        .replace(
            "BURC_TEMP_ROOT_DIR=temp\n",
            "BURC_TEMP_ROOT_DIR=../temp-lure-parent\n",
        )
        .replace(
            "BURC_OUTPUT_ROOT_DIR=output\n",
            "BURC_OUTPUT_ROOT_DIR=../output-lure-parent\n",
        )
        .replace(
            &format!(
                "BURC_LOOSEBOX_ROOT_DIR={}\n",
                BKG_BURC_LOOSEBOX_ROOT_DIR_BASE
            ),
            &format!(
                "BURC_LOOSEBOX_ROOT_DIR=../{}-lure-parent\n",
                BKG_BURC_LOOSEBOX_ROOT_DIR_BASE
            ),
        );

    assert_ne!(
        posed, standing,
        "the seat's default regime spells all three roots plainly, so posing a parent segment on \
         each must change the file: {}",
        standing
    );

    posed
}

/// A REGIME ROOT SPELLED THROUGH A PARENT SEGMENT IS SETTLED BEFORE IT IS
/// ANNOUNCED, exactly as a plain relative root is.
///
/// THE STATION'S OWN SHAPE. A station's regime may state its temp, output
/// and loosebox roots as siblings of the checkout rather than as
/// subdirectories beneath it, and the dispatch composes every path it hands
/// a door from the process's own working directory rather than from a
/// canonical repository root — so the sibling segment lands in whatever this
/// dispatch announces and exports unless it is resolved through the
/// filesystem first. A consumer that compares such a path against a
/// canonical answer — cargo's own, or a nested shell that re-derives its
/// working directory — never matches an unsettled one.
#[test]
fn bktb_the_dispatch_settles_a_root_spelled_through_a_parent_segment() {
    let lure = bktu_Lure::bktu_compose("substrate-posed-parent");
    let tabtarget = lure.bktu_substrate_seat("#!/bin/bash\nexit 0\n");

    let posed = zbktb_posed_parent_regime(&lure);
    lure.bktu_write(".buk/burc.env", &posed);
    lure.bktu_commit("pose every regime root through a parent segment");

    let out = lure.bktu_dispatch(&tabtarget, &[]);
    let said = String::from_utf8_lossy(&out.stdout);
    let grumbled = String::from_utf8_lossy(&out.stderr);

    assert!(
        out.status.success(),
        "the dispatch settles a parent-relative root rather than refusing it: {}\n{}",
        said,
        grumbled
    );

    for (label, must_be_dir) in [
        (BKTB_TRANSCRIPT, false),
        (BKTB_OUTPUT, true),
        (BKTB_LOOSEBOX, true),
    ] {
        let announced = zbktb_announced(&said, label);
        let path = std::path::Path::new(announced);

        assert!(
            !path
                .components()
                .any(|c| matches!(c, std::path::Component::ParentDir)),
            "the {} path carries no parent-segment component once the dispatch settles it: {}",
            label,
            announced
        );
        assert!(
            path.is_absolute(),
            "the {} path stands absolute once the dispatch settles it: {}",
            label,
            announced
        );
        if must_be_dir {
            assert!(
                path.is_dir(),
                "the {} path is the directory the dispatch made: {}",
                label,
                announced
            );
        }
    }
}

/// The rest of the line the dispatch announced under `label`, trimmed.
///
/// Dying rather than answering empty: an announcement the dispatch did not make
/// is a chain that did not run, and an empty answer would pass every assertion
/// taken over it.
fn zbktb_announced<'a>(said: &'a str, label: &str) -> &'a str {
    said.lines()
        .find(|line| line.starts_with(label))
        .unwrap_or_else(|| panic!("the dispatch announces '{}': {}", label, said))
        .trim_start_matches(label)
        .trim()
}

// eof
