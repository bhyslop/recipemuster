// Copyright 2026 Scale Invariant, Inc.
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
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHDT — the perambulation's self-proofs, ported from the retired theurge
// fixture when the cut was absorbed (the 260718 re-ruling). They, not a green
// lap, are what prove the sweep — the 292-MiB catch: clean tip, leaking
// history.
//
//   TOTALITY  every tracked path is ruled ship or withhold, and every row
//             rules on something. Not that the ruling is right — that is the
//             operator's judgment, and no test can hold it — but that the
//             ruling EXISTS, for every path, before a candidate is cut.
//
//   SWEEP     the object-graph sweep catches a planted withheld path and
//             stays silent on a clean list. A sweep that cannot catch a
//             planted leak cannot be trusted to catch a real one, and a sweep
//             that reddens on a clean list would be worked around in a week.
//
// The tracked set is derived from git at test time, never from a list: the
// judgment is against what the repository actually carries at this commit.

use std::process::Command;

use crate::rbthdr_perambulation::{
    dead_rows,
    judge,
    rbthdr_Disposition,
    shipped,
    sweep,
    unjudged,
    validate,
    RBTHDR_ROWS,
};

/// The maintainer repo root, from git, anchored on the crate directory (tests
/// run with an arbitrary cwd). Panics, not fatals — a fatal would exit the
/// whole test harness.
fn zrbthdt_tracked() -> Vec<String> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let out = Command::new("git")
        .args(["-C", manifest_dir, "rev-parse", "--show-toplevel"])
        .output()
        .expect("cannot launch git");
    assert!(
        out.status.success(),
        "git rev-parse --show-toplevel failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let root = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert!(!root.is_empty(), "git returned an empty repository root");

    let out = Command::new("git")
        .args(["-C", &root, "ls-files"])
        .output()
        .expect("cannot launch git");
    assert!(
        out.status.success(),
        "git ls-files failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let tracked: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();
    assert!(!tracked.is_empty(), "no tracked paths — the perambulation has nothing to judge");
    tracked
}

// ── The totality proof ──────────────────────────────────────

/// Every tracked path is judged, and every row judges something. The
/// completeness case: it does not ask whether the ruling is right, only that
/// it was made.
#[test]
fn rbthdt_totality() {
    validate(RBTHDR_ROWS).expect("the perambulation table is malformed");

    let tracked = zrbthdt_tracked();

    let unruled = unjudged(&tracked);
    assert!(
        unruled.is_empty(),
        "{} tracked path(s) unjudged — rule each ship or withhold in the perambulation table:\n{}",
        unruled.len(),
        unruled.join("\n")
    );

    let dead = dead_rows(&tracked);
    let dead_lines: Vec<String> = dead
        .iter()
        .map(|(prefix, disposition)| format!("{}|{}", prefix, disposition))
        .collect();
    assert!(
        dead.is_empty(),
        "{} perambulation row(s) judge no tracked path — stale, or shadowed by a longer row:\n{}",
        dead.len(),
        dead_lines.join("\n")
    );

    // The judgment ships something and withholds something — a table that has
    // drifted into either extreme is lying about the tree.
    let ship_count = shipped(&tracked).len();
    assert!(ship_count > 0, "the perambulation ships nothing");
    assert!(ship_count < tracked.len(), "the perambulation withholds nothing");
}

// ── The planted-leak sweep proof ────────────────────────────

/// The planted paths, and the clean ones. Both are ordinary repo paths named
/// structurally — a veiled spec, a memo, the operator's own kit, a withheld
/// tabtarget — against a delivered face that must pass untouched. The
/// veiled-spec datum carries a SYNTHETIC basename: what it exercises is the
/// longest-wins override — a veiled half inside a shipping kit — which is a
/// property of the directory, not of any document sitting in it.
const ZRBTHDT_PLANTED: &[&str] = &[
    "Tools/rbk/vov_veiled/planted-spec.adoc",
    "Memos/memo-20260713-the-one-that-got-out.md",
    "Tools/jjk/jjw_workbench.sh",
    "tt/rbw-MZ.MarshalZeroes.sh",
    "rbmm_moorings/fdkyclk/fdkyclk-proof.sh",
];
const ZRBTHDT_CLEAN: &[&str] = &[
    "README.md",
    "LICENSE",
    "Tools/rbk/rba_auth.sh",
    "Tools/buk/buc_command.sh",
    "tt/rbw-cC.Charge.tadmor.sh",
    "rbmm_moorings/fdkyclk/fdkyclk-asserter-key.pem",
];

#[test]
fn rbthdt_sweep_catches_planted_leaks() {
    let dirty: Vec<String> = ZRBTHDT_PLANTED
        .iter()
        .chain(ZRBTHDT_CLEAN.iter())
        .map(|p| p.to_string())
        .collect();
    let leaks = sweep(&dirty);

    for planted in ZRBTHDT_PLANTED {
        assert!(
            leaks.iter().any(|leak| leak == planted),
            "the sweep did not catch planted withheld path {} — it would ride a candidate",
            planted
        );
    }
    for clean in ZRBTHDT_CLEAN {
        assert!(
            !leaks.iter().any(|leak| leak == clean),
            "the sweep reddened on shipped path {} — a sweep that cries wolf gets disabled",
            clean
        );
    }
}

#[test]
fn rbthdt_sweep_silent_on_clean_list() {
    let clean: Vec<String> = ZRBTHDT_CLEAN.iter().map(|p| p.to_string()).collect();
    let leaks = sweep(&clean);
    assert!(
        leaks.is_empty(),
        "the sweep reddened on a clean list: {}",
        leaks.join(", ")
    );
}

// ── The matcher's own grains ────────────────────────────────

/// Longest-wins across the three grains: the veiled half outranks its
/// shipping kit, the marshal stem outranks the shipped tabtarget family, and
/// an unjudged root is None — never a default in either direction.
#[test]
fn rbthdt_matcher_longest_wins() {
    let veiled = judge("Tools/rbk/vov_veiled/anything.rs");
    assert!(matches!(veiled, Some((rbthdr_Disposition::Withhold, _))), "veiled half must outrank the shipping kit");

    let shipped_code = judge("Tools/rbk/rba_auth.sh");
    assert!(matches!(shipped_code, Some((rbthdr_Disposition::Ship, _))), "the delivered kit ships");

    let marshal_tt = judge("tt/rbw-MZ.MarshalZeroes.sh");
    assert!(matches!(marshal_tt, Some((rbthdr_Disposition::Withhold, _))), "the marshal stem must outrank tt/rbw-");

    let hierophant_tt = judge("tt/rbthw-e.Essai.sh");
    assert!(matches!(hierophant_tt, Some((rbthdr_Disposition::Withhold, _))), "the hierophant stem is withheld");

    let shipped_tt = judge("tt/rbw-cC.Charge.tadmor.sh");
    assert!(matches!(shipped_tt, Some((rbthdr_Disposition::Ship, _))), "the shipped tabtarget family ships");

    assert!(judge("no-such-root/file.txt").is_none(), "an unjudged path must be None, not a default");
}

// ── The table's structural invariants ───────────────────────

/// The enrollment-time checks the bash table ran, held as validation: an
/// empty prefix would match every path; a duplicate prefix would tie-break by
/// table order — the shadowing the longest-wins rule exists to abolish.
#[test]
fn rbthdt_validate_refuses_malformed_tables() {
    use rbthdr_Disposition::{Ship, Withhold};

    validate(&[]).expect("an empty table is structurally valid");
    validate(&[("a/", Ship), ("b/", Withhold)]).expect("a well-formed table validates");

    assert!(validate(&[("", Ship)]).is_err(), "an empty prefix must refuse");
    assert!(
        validate(&[("a/", Ship), ("a/", Withhold)]).is_err(),
        "a duplicate prefix must refuse"
    );
}
