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
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTDRZ — the voorloper doors' standing proof: a credless reveille member that
// drives the two doors a fresh clone carries for a newcomer, every case over
// scratch of its own and never over the tree it runs in.
//
// The prepare door. Each case clones the committed tree into the case, with no
// station beside the clone unless the case lays one. The committed tree carries
// site values marshal zero blanks, so the success case blanks them in its clone
// and commits there first, standing the clone in marshal-zero state; the
// refusal cases drive the door beside a station that already stands, and on the
// clone as cloned.
//
// The leftovers door. The no-credential case drives the door in a clone beside
// a station holding no payor credential. The could-not-ask case plants one, and
// the reveille guard every launch here carries makes the payor's token mint
// refuse before any credential touch, so the door must not answer that nothing
// stands. The something-stands answer is fed through the door's one reading,
// zrbgp_depot_leftovers_read, in a direct-kindle child over planted depot facts
// beside a foreign prefix's; a live payor's answer is beyond this fixture.

use std::path::{
    Path,
    PathBuf, //
};
use std::process::Command;

use crate::rbtcee_case;
use crate::rbtcee_engine::{
    rbtcee_Case,
    rbtcee_Disposition,
    rbtcee_Fixture,
    rbtcee_Tariff,
    rbtcee_Verdict, //
};
use crate::rbtcei_invocation::rbtdri_rooted_tabtarget_command;
use crate::rbtcep_platform::rbtcep_native_to_posix;
use crate::rbtchf_fast::{
    rbtchf_run_bash,
    RBTCHF_BUV_VALIDATION,
    RBTCHF_RBK_ROOT, //
};
use crate::rbtchx_pyx::zrbtchx_root;
use crate::rbtgeb_breviary::RBTGEB_CREDLESS_DENTIL;
use crate::rbtgec_consts::{
    RBTGEC_ACCOUNT_PAYOR,
    RBTGEC_MOORINGS_DIR,
    RBTGEC_RBRD_FILE,
    RBTGEC_RBRN_FILE,
    RBTGEC_RBRO_FILE,
    RBTGEC_RBRR_FILE,
    RBTGEC_RBRV_FILE,
    RBTGEC_VESSELS_SUBDIR,
    RBTGEC_VOORLOPER_LEFTOVERS,
    RBTGEC_VOORLOPER_PREPARE,
    RBTGEC_VOORLOPER_TINCTURE, //
};

pub const RBTDRZ_NAME_VOORLOPER_DOORS: &str = "voorloper";

// Repo-root-relative kit modules the direct-kindle child sources, beside the
// two the fast fixtures already name.
const ZRBTDRZ_BUC_COMMAND: &str = "Tools/buk/buc_command.sh";
const ZRBTDRZ_BUYM_YELP: &str = "Tools/buk/buym_yelp.sh";
const ZRBTDRZ_BUBC_CONSTANTS: &str = "Tools/buk/bubc_constants.sh";
const ZRBTDRZ_BURD_REGIME: &str = "Tools/buk/burd_regime.sh";

/// The scratch clone's basename, and the tabtarget directory inside it.
const ZRBTDRZ_CLONE: &str = "zqtree";
const ZRBTDRZ_TT_DIR: &str = "tt";

/// The config regime naming the station file, and the key it is named under.
const ZRBTDRZ_CONFIG_REGIME: &str = "burc.env";
const ZRBTDRZ_STATION_KEY: &str = "BURC_STATION_FILE=";

/// The key the repo regime names the secrets directory under.
const ZRBTDRZ_SECRETS_KEY: &str = "RBRR_SECRETS_DIR=";

/// The station a leftovers case lays beside its clone, and the log directory it
/// names one level up from the clone.
const ZRBTDRZ_STATION_LOGS: &str = "zqlogs";
const ZRBTDRZ_STATION_BODY: &str = "BURS_USER=zqnewcomer\nBURS_TINCTURE=zqv\nBURS_LOG_DIR=../zqlogs\n";

/// The station body a refusal case lays, which the door must leave as it found.
const ZRBTDRZ_STANDING_STATION: &str = "BURS_USER=zqstanding\nBURS_TINCTURE=zqs\nBURS_LOG_DIR=../zqlogs\n";

/// The station field the prepare door sets.
const ZRBTDRZ_TINCTURE_FIELD: &str = "BURS_TINCTURE=";

/// The planted payor credential's body: present, never usable.
const ZRBTDRZ_PLANTED_CREDENTIAL: &str = "# zq planted payor credential, never read\n";

/// The identity the success case commits its zeroed clone under.
const ZRBTDRZ_COMMIT_NAME: &str = "user.name=zqfixture";
const ZRBTDRZ_COMMIT_EMAIL: &str = "user.email=zqfixture@invalid";

/// Marshal zero's blank set, the fields it leaves blank. Mirror: rblm_cli.sh
/// `zrblm_zero_rewrite`, the fields it rewrites to a bare "FIELD=", the vessel
/// image-anchor family spelled out as rbrv_regime.sh enrolls its slots.
const ZRBTDRZ_BLANK_RBRR: &[&str] = &["RBRR_RUNTIME_PREFIX", "RBRR_SUBSTRATE_RELIQUARY"];
const ZRBTDRZ_BLANK_RBRD: &[&str] = &["RBRD_CLOUD_PREFIX", "RBRD_DEPOT_MONIKER"];
const ZRBTDRZ_BLANK_RBRN: &[&str] = &["RBRN_SENTRY_HALLMARK", "RBRN_BOTTLE_HALLMARK"];
const ZRBTDRZ_BLANK_RBRV: &[&str] = &[
    "RBRV_RELIQUARY",
    "RBRV_IMAGE_1_ANCHOR",
    "RBRV_IMAGE_2_ANCHOR",
    "RBRV_IMAGE_3_ANCHOR", //
];

/// The first blank-set field the committed tree carries filled, in the order
/// the door reads them. Mirror: rblm_cli.sh `zrblm_voorloper_blank_vet` and
/// `zrblm_zero_rewrite`, which read the repo regime first.
const ZRBTDRZ_FIRST_FILLED: &str = "RBRR_RUNTIME_PREFIX";

/// Each door's message, asserted beside its exit. Mirror: rblm_cli.sh
/// `rblm_voorloper_prepare`, `zrblm_voorloper_blank_vet` and
/// `rblm_voorloper_leftovers`, and rbgp_payor.sh `zrbgp_depot_leftovers_read` —
/// same literals.
const ZRBTDRZ_STATION_STANDS_FRAGMENT: &str = "file already stands at";
const ZRBTDRZ_FILLED_FRAGMENT: &str = "stands filled in";
const ZRBTDRZ_NO_CREDENTIAL_FRAGMENT: &str = "holds no credential, so it founded nothing";
const ZRBTDRZ_COULD_NOT_ASK_FRAGMENT: &str = "the payor's ask ended with exit";
const ZRBTDRZ_STANDS_FRAGMENT: &str = "Depot stands: ";
const ZRBTDRZ_NONE_FRAGMENT: &str = "No depot stands under cloud prefix";

/// The leftovers door's three answers, spelled as literals: sound, barred and
/// broke, the exit verdicts BUK declares (BUr_vsm).
const ZRBTDRZ_EXIT_NOTHING_STANDS: i32 = 0;
const ZRBTDRZ_EXIT_SOMETHING_STANDS: i32 = 3;
const ZRBTDRZ_EXIT_COULD_NOT_ASK: i32 = 1;

/// The planted depot facts: this tree's own cloud prefix, and a foreign one
/// whose standing depot the reading must pass over.
const ZRBTDRZ_FACTS_ROOT: &str = "zqfacts";
const ZRBTDRZ_OWN_PREFIX: &str = "zqmine-";
const ZRBTDRZ_FOREIGN_PREFIX: &str = "zqother-";
const ZRBTDRZ_DEPOT_INFIX: &str = "d-";
const ZRBTDRZ_MONIKER_STANDING: &str = "zqaa";
const ZRBTDRZ_MONIKER_UNMADE: &str = "zqbb";
const ZRBTDRZ_MONIKER_FOREIGN: &str = "zqcc";

/// One depot a reading case plants: its prefix, its moniker, and whether it
/// stands in the complete state or the unmade one. The child writes its two
/// facts from the payor module's own state words and fact names.
struct zrbtdrz_Plant {
    prefix: &'static str,
    moniker: &'static str,
    complete: bool,
}

/// The child's planting function: FACTS_ROOT PREFIX MONIKER STATE, laying the
/// state fact and the project fact under the prefix's own subdirectory.
const ZRBTDRZ_PLANT_FN: &str = r#"zq_plant() {
  local -r z_dir="$1/${2%-}"
  mkdir -p "${z_dir}"
  printf '%s' "$4" > "${z_dir}/$3.${RBCC_fact_ext_depot}"
  printf '%s%s%s' "$2" "${RBGC_depot_project_infix}" "$3" > "${z_dir}/$3.${RBCC_fact_ext_depot_project}"
}
"#;

fn zrbtdrz_verdict(outcome: Result<(), String>) -> rbtcee_Verdict {
    match outcome {
        Ok(()) => rbtcee_Verdict::Pass,
        Err(e) => rbtcee_Verdict::Fail(e),
    }
}

fn zrbtdrz_mkdir(path: &Path) -> Result<(), String> {
    std::fs::create_dir_all(path).map_err(|e| {
        format!(
            "cannot create {}: {}",
            path.display(),
            e, //
        )
    })
}

fn zrbtdrz_write(path: &Path, body: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        zrbtdrz_mkdir(parent)?;
    }
    std::fs::write(path, body).map_err(|e| {
        format!(
            "cannot write {}: {}",
            path.display(),
            e, //
        )
    })
}

fn zrbtdrz_read(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| {
        format!(
            "cannot read {}: {}",
            path.display(),
            e, //
        )
    })
}

/// git -C <tree> <args>, its failure an error naming the act.
fn zrbtdrz_git(tree: &Path, args: &[&str], act: &str) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(tree)
        .args(args)
        .output()
        .map_err(|e| {
            format!(
                "cannot run git to {}: {}",
                act,
                e, //
            )
        })?;
    if !output.status.success() {
        return Err(format!(
            "git failed to {} (exit {:?}): {}",
            act,
            output.status.code(),
            String::from_utf8_lossy(&output.stderr).trim(), //
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// A scratch clone of the committed tree inside the case.
fn zrbtdrz_clone(dir: &Path) -> Result<PathBuf, String> {
    let root = zrbtchx_root()?;
    let clone = dir.join(ZRBTDRZ_CLONE);
    let output = Command::new("git")
        .arg("clone")
        .arg("-q")
        .arg(&root)
        .arg(&clone)
        .output()
        .map_err(|e| {
            format!(
                "cannot run git clone: {}",
                e, //
            )
        })?;
    if !output.status.success() {
        return Err(format!(
            "git clone of {} failed: {}",
            root.display(),
            String::from_utf8_lossy(&output.stderr).trim(), //
        ));
    }
    Ok(clone)
}

/// The value one `KEY=` line of a regime file carries, the first such line.
fn zrbtdrz_value(path: &Path, key: &str) -> Result<String, String> {
    let body = zrbtdrz_read(path)?;
    body.lines()
        .find_map(|line| line.strip_prefix(key))
        .map(|v| v.trim().to_string())
        .ok_or_else(|| {
            format!(
                "{} carries no {} line",
                path.display(),
                key, //
            )
        })
}

/// The station file the clone's config regime names, beside the clone.
fn zrbtdrz_station(clone: &Path) -> Result<PathBuf, String> {
    let regime = clone.join(RBTGEC_MOORINGS_DIR).join(ZRBTDRZ_CONFIG_REGIME);
    let rel = zrbtdrz_value(&regime, ZRBTDRZ_STATION_KEY)?;
    Ok(clone.join(rel))
}

/// The payor credential the clone's repo regime names, beside the clone.
fn zrbtdrz_credential(clone: &Path) -> Result<PathBuf, String> {
    let rel = zrbtdrz_value(&clone.join(RBTGEC_RBRR_FILE), ZRBTDRZ_SECRETS_KEY)?;
    Ok(clone
        .join(rel)
        .join(RBTGEC_ACCOUNT_PAYOR)
        .join(RBTGEC_RBRO_FILE))
}

/// The clone's door for `colophon`, the one tabtarget it carries under it.
fn zrbtdrz_door(clone: &Path, colophon: &str) -> Result<PathBuf, String> {
    let tt = clone.join(ZRBTDRZ_TT_DIR);
    let head = format!(
        "{}.",
        colophon, //
    );
    let entries = std::fs::read_dir(&tt).map_err(|e| {
        format!(
            "cannot list {}: {}",
            tt.display(),
            e, //
        )
    })?;
    let mut found: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with(&head) && name.ends_with(".sh") {
            found.push(entry.path());
        }
    }
    match found.len() {
        1 => Ok(found.remove(0)),
        n => Err(format!(
            "{} carries {} files for colophon {}, where one was expected",
            tt.display(),
            n,
            colophon, //
        )),
    }
}

/// Drive the clone's door for `colophon` from inside the clone, its run roots
/// under the case, and keep what it said.
fn zrbtdrz_drive(
    dir: &Path,
    clone: &Path,
    colophon: &str,
    label: &str,
) -> Result<(i32, String), String> {
    let door = zrbtdrz_door(clone, colophon)?;
    let mut cmd = rbtdri_rooted_tabtarget_command(&door, dir)?;
    let output = cmd.current_dir(clone).output().map_err(|e| {
        format!(
            "{}: cannot run {}: {}",
            label,
            door.display(),
            e, //
        )
    })?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let _ = std::fs::write(
        dir.join(format!(
            "{}-stdout.txt",
            label, //
        )),
        &stdout,
    );
    let _ = std::fs::write(
        dir.join(format!(
            "{}-stderr.txt",
            label, //
        )),
        &stderr,
    );
    Ok((
        output.status.code().unwrap_or(-1),
        format!(
            "--- stdout ---\n{}\n--- stderr ---\n{}",
            stdout,
            stderr, //
        ),
    ))
}

/// Require the exit and a fragment of what the door said.
fn zrbtdrz_expect(
    label: &str,
    code: i32,
    said: &str,
    expect_code: i32,
    fragment: &str,
) -> Result<(), String> {
    if code != expect_code {
        return Err(format!(
            "{}: exited {} — expected {}\n{}",
            label,
            code,
            expect_code,
            said, //
        ));
    }
    if !said.contains(fragment) {
        return Err(format!(
            "{}: exited {} without '{}'\n{}",
            label,
            code,
            fragment,
            said, //
        ));
    }
    Ok(())
}

/// Blank, in one regime file, each `FIELD=` line whose field `blank` names,
/// keeping every other line as it stood.
fn zrbtdrz_blank(path: &Path, blank: &dyn Fn(&str) -> bool) -> Result<(), String> {
    let body = zrbtdrz_read(path)?;
    let mut out = String::new();
    for line in body.lines() {
        match line.split_once('=') {
            Some((field, _)) if blank(field) => {
                out.push_str(field);
                out.push('=');
            }
            _ => out.push_str(line),
        }
        out.push('\n');
    }
    zrbtdrz_write(path, &out)
}

/// The regime files one level beneath `parent` named `file`.
fn zrbtdrz_regimes_beneath(parent: &Path, file: &str) -> Result<Vec<PathBuf>, String> {
    let entries = std::fs::read_dir(parent).map_err(|e| {
        format!(
            "cannot list {}: {}",
            parent.display(),
            e, //
        )
    })?;
    let mut found: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path().join(file);
        if path.is_file() {
            found.push(path);
        }
    }
    Ok(found)
}

/// Stand the clone in marshal-zero state: blank zero's blank set and commit it
/// in the clone, so the door meets the tree a newcomer is handed.
fn zrbtdrz_zero_clone(clone: &Path) -> Result<(), String> {
    zrbtdrz_blank(&clone.join(RBTGEC_RBRR_FILE), &|f| {
        ZRBTDRZ_BLANK_RBRR.contains(&f)
    })?;
    zrbtdrz_blank(&clone.join(RBTGEC_RBRD_FILE), &|f| {
        ZRBTDRZ_BLANK_RBRD.contains(&f)
    })?;
    let moorings = clone.join(RBTGEC_MOORINGS_DIR);
    for nameplate in zrbtdrz_regimes_beneath(&moorings, RBTGEC_RBRN_FILE)? {
        zrbtdrz_blank(&nameplate, &|f| ZRBTDRZ_BLANK_RBRN.contains(&f))?;
    }
    for vessel in zrbtdrz_regimes_beneath(&moorings.join(RBTGEC_VESSELS_SUBDIR), RBTGEC_RBRV_FILE)? {
        zrbtdrz_blank(&vessel, &|f| ZRBTDRZ_BLANK_RBRV.contains(&f))?;
    }
    zrbtdrz_git(
        clone,
        &[
            "-c",
            ZRBTDRZ_COMMIT_NAME,
            "-c",
            ZRBTDRZ_COMMIT_EMAIL,
            "commit",
            "-q",
            "-a",
            "-m",
            "zq marshal-zero state",
        ],
        "commit the zeroed clone",
    )?;
    Ok(())
}

// ── The prepare door ─────────────────────────────────────────

/// PREPARE STANDS A FRESH CLONE'S STATION: in a clone in marshal-zero state
/// beside no station file, the door exits 0, leaves the station file beside the
/// clone wearing the fixed tincture on its one tincture line, and leaves the
/// clone's tree clean.
fn rbtdrz_prepare_fresh_clone(dir: &Path) -> rbtcee_Verdict {
    zrbtdrz_verdict((|| {
        let clone = zrbtdrz_clone(dir)?;
        zrbtdrz_zero_clone(&clone)?;
        let station = zrbtdrz_station(&clone)?;
        if station.exists() {
            return Err(format!(
                "prepare-fresh-clone: {} stands before the door ran",
                station.display(), //
            ));
        }
        let (code, said) = zrbtdrz_drive(dir, &clone, RBTGEC_VOORLOPER_PREPARE, "prepare-fresh-clone")?;
        if code != 0 {
            return Err(format!(
                "prepare-fresh-clone: exited {} — expected 0\n{}",
                code,
                said, //
            ));
        }
        let body = zrbtdrz_read(&station)?;
        let tinctures: Vec<&str> = body
            .lines()
            .filter(|line| line.starts_with(ZRBTDRZ_TINCTURE_FIELD))
            .collect();
        let expected = format!(
            "{}{}",
            ZRBTDRZ_TINCTURE_FIELD,
            RBTGEC_VOORLOPER_TINCTURE, //
        );
        if tinctures != [expected.as_str()] {
            return Err(format!(
                "prepare-fresh-clone: the tincture lines of {} read {:?} — expected exactly [{:?}]\n{}",
                station.display(),
                tinctures,
                expected,
                body, //
            ));
        }
        let status = zrbtdrz_git(&clone, &["status", "--porcelain"], "read the clone's status")?;
        if !status.trim().is_empty() {
            return Err(format!(
                "prepare-fresh-clone: the door left the clone's tree dirty:\n{}",
                status, //
            ));
        }
        Ok(())
    })())
}

/// PREPARE NEVER OVERWRITES A STATION: beside a station file that already
/// stands, the door refuses naming it, and the file stands byte-identical.
fn rbtdrz_prepare_station_stands(dir: &Path) -> rbtcee_Verdict {
    zrbtdrz_verdict((|| {
        let clone = zrbtdrz_clone(dir)?;
        let station = zrbtdrz_station(&clone)?;
        zrbtdrz_write(&station, ZRBTDRZ_STANDING_STATION)?;
        let (code, said) = zrbtdrz_drive(dir, &clone, RBTGEC_VOORLOPER_PREPARE, "prepare-already-standing")?;
        if code == 0 {
            return Err(format!(
                "prepare-already-standing: exited 0 beside {}\n{}",
                station.display(),
                said, //
            ));
        }
        zrbtdrz_expect(
            "prepare-already-standing",
            code,
            &said,
            code,
            ZRBTDRZ_STATION_STANDS_FRAGMENT,
        )?;
        let after = zrbtdrz_read(&station)?;
        if after != ZRBTDRZ_STANDING_STATION {
            return Err(format!(
                "prepare-already-standing: {} changed:\n{}",
                station.display(),
                after, //
            ));
        }
        Ok(())
    })())
}

/// PREPARE REFUSES A FIELD ZERO BLANKS STANDING FILLED: in the clone as cloned,
/// whose regimes carry the committed tree's site values, the door refuses naming
/// the first such field, and writes no station.
fn rbtdrz_prepare_field_filled(dir: &Path) -> rbtcee_Verdict {
    zrbtdrz_verdict((|| {
        let clone = zrbtdrz_clone(dir)?;
        let station = zrbtdrz_station(&clone)?;
        let (code, said) = zrbtdrz_drive(dir, &clone, RBTGEC_VOORLOPER_PREPARE, "prepare-field-filled")?;
        if code == 0 {
            return Err(format!(
                "prepare-field-filled: exited 0 over a tree carrying site values\n{}",
                said, //
            ));
        }
        let fragment = format!(
            "{} {}",
            ZRBTDRZ_FIRST_FILLED,
            ZRBTDRZ_FILLED_FRAGMENT, //
        );
        zrbtdrz_expect("prepare-field-filled", code, &said, code, &fragment)?;
        if station.exists() {
            return Err(format!(
                "prepare-field-filled: the refusal left {} standing",
                station.display(), //
            ));
        }
        Ok(())
    })())
}

// ── The leftovers door ───────────────────────────────────────

/// Lay the station a leftovers case runs beside, and its log directory.
fn zrbtdrz_lay_station(dir: &Path, clone: &Path) -> Result<(), String> {
    zrbtdrz_mkdir(&dir.join(ZRBTDRZ_STATION_LOGS))?;
    zrbtdrz_write(&zrbtdrz_station(clone)?, ZRBTDRZ_STATION_BODY)
}

/// NO CREDENTIAL, NOTHING STANDS: beside a station holding no payor credential,
/// the door answers clear and says why.
fn rbtdrz_leftovers_no_credential(dir: &Path) -> rbtcee_Verdict {
    zrbtdrz_verdict((|| {
        let clone = zrbtdrz_clone(dir)?;
        zrbtdrz_lay_station(dir, &clone)?;
        let credential = zrbtdrz_credential(&clone)?;
        if credential.exists() {
            return Err(format!(
                "leftovers-no-credential: a credential stands at {} before the door ran",
                credential.display(), //
            ));
        }
        let (code, said) = zrbtdrz_drive(dir, &clone, RBTGEC_VOORLOPER_LEFTOVERS, "leftovers-no-credential")?;
        zrbtdrz_expect(
            "leftovers-no-credential",
            code,
            &said,
            ZRBTDRZ_EXIT_NOTHING_STANDS,
            ZRBTDRZ_NO_CREDENTIAL_FRAGMENT,
        )
    })())
}

/// A CREDENTIAL IT CANNOT USE, COULD NOT ASK: beside a planted payor credential,
/// under the reveille guard the launch carries, the payor's ask refuses at the
/// token mint, and the door answers on its could-not-ask exit, naming the
/// guard's code as the ask's.
fn rbtdrz_leftovers_could_not_ask(dir: &Path) -> rbtcee_Verdict {
    zrbtdrz_verdict((|| {
        let clone = zrbtdrz_clone(dir)?;
        zrbtdrz_lay_station(dir, &clone)?;
        zrbtdrz_write(&zrbtdrz_credential(&clone)?, ZRBTDRZ_PLANTED_CREDENTIAL)?;
        let (code, said) = zrbtdrz_drive(dir, &clone, RBTGEC_VOORLOPER_LEFTOVERS, "leftovers-could-not-ask")?;
        let fragment = format!(
            "{} {}",
            ZRBTDRZ_COULD_NOT_ASK_FRAGMENT,
            RBTGEB_CREDLESS_DENTIL, //
        );
        zrbtdrz_expect(
            "leftovers-could-not-ask",
            code,
            &said,
            ZRBTDRZ_EXIT_COULD_NOT_ASK,
            &fragment,
        )
    })())
}

/// Feed the planted facts through the one reading in a direct-kindle child that
/// kindles no credential module, for this tree's own prefix.
fn zrbtdrz_read_planted(
    dir: &Path,
    label: &str,
    plants: &[zrbtdrz_Plant],
) -> Result<(i32, String), String> {
    let root = zrbtchx_root()?;
    let temp = dir.join(format!(
        "{}-temp",
        label, //
    ));
    zrbtdrz_mkdir(&temp)?;
    let module = |rel: &str| rbtcep_native_to_posix(&root.join(rel));
    let facts = rbtcep_native_to_posix(&dir.join(ZRBTDRZ_FACTS_ROOT));
    let mut planting = String::from(ZRBTDRZ_PLANT_FN);
    for plant in plants {
        planting.push_str(&format!(
            "zq_plant '{}' '{}' '{}' \"{}\"\n",
            facts,
            plant.prefix,
            plant.moniker,
            if plant.complete {
                "${RBGP_DEPOT_STATE_COMPLETE}"
            } else {
                "${RBGP_DEPOT_STATE_DELETE_REQUESTED}"
            }, //
        ));
    }
    let script = format!(
        "set -euo pipefail\n\
         BURD_TEMP_DIR='{temp}'\n\
         BURD_MOORINGS_DIR='{moorings}'\n\
         source '{buc}'\n\
         source '{buym}'\n\
         source '{bubc}'\n\
         source '{buv}'\n\
         source '{burd}'\n\
         source '{rbk}/rbcc_constants.sh'\n\
         source '{rbk}/rbgc_constants.sh'\n\
         source '{rbk}/rbgo_oauth.sh'\n\
         source '{rbk}/rbuh_http.sh'\n\
         source '{rbk}/rbge_rest.sh'\n\
         source '{rbk}/rbgi_iam.sh'\n\
         source '{rbk}/rbgp_payor.sh'\n\
         zbuv_kindle\n\
         zburd_kindle\n\
         RBRD_GCP_REGION='zqregion'\n\
         RBDC_DEPOT_PROJECT_ID='zqdepot'\n\
         RBRP_PAYOR_PROJECT_ID='zqpayor'\n\
         zrbcc_kindle\n\
         zrbgc_kindle\n\
         zrbgo_kindle\n\
         zrbuh_kindle\n\
         zrbge_kindle\n\
         zrbgi_kindle\n\
         zrbgp_kindle\n\
         {planting}\
         zrbgp_depot_leftovers_read '{facts}' '{prefix}'\n",
        temp = rbtcep_native_to_posix(&temp),
        moorings = RBTGEC_MOORINGS_DIR,
        buc = module(ZRBTDRZ_BUC_COMMAND),
        buym = module(ZRBTDRZ_BUYM_YELP),
        bubc = module(ZRBTDRZ_BUBC_CONSTANTS),
        buv = module(RBTCHF_BUV_VALIDATION),
        burd = module(ZRBTDRZ_BURD_REGIME),
        rbk = module(RBTCHF_RBK_ROOT),
        planting = planting,
        facts = facts,
        prefix = ZRBTDRZ_OWN_PREFIX, //
    );
    let (code, stdout, stderr) = rbtchf_run_bash(&root, &script, dir, label)?;
    Ok((
        code,
        format!(
            "--- stdout ---\n{}\n--- stderr ---\n{}",
            stdout,
            stderr, //
        ),
    ))
}

/// SOMETHING STANDS, BY A PLANTED ANSWER: under this tree's own prefix one depot
/// stands complete and one stands unmade, beside a foreign prefix's standing
/// depot. The reading names the complete one alone, passes over the unmade one
/// and the foreign one, and answers on its something-stands exit.
fn rbtdrz_reading_something_stands(dir: &Path) -> rbtcee_Verdict {
    zrbtdrz_verdict((|| {
        let plants = [
            zrbtdrz_Plant {
                prefix: ZRBTDRZ_OWN_PREFIX,
                moniker: ZRBTDRZ_MONIKER_STANDING,
                complete: true,
            },
            zrbtdrz_Plant {
                prefix: ZRBTDRZ_OWN_PREFIX,
                moniker: ZRBTDRZ_MONIKER_UNMADE,
                complete: false,
            },
            zrbtdrz_Plant {
                prefix: ZRBTDRZ_FOREIGN_PREFIX,
                moniker: ZRBTDRZ_MONIKER_FOREIGN,
                complete: true,
            },
        ];
        let (code, said) = zrbtdrz_read_planted(dir, "reading-something-stands", &plants)?;
        let standing = format!(
            "{}{}{}{}",
            ZRBTDRZ_STANDS_FRAGMENT,
            ZRBTDRZ_OWN_PREFIX,
            ZRBTDRZ_DEPOT_INFIX,
            ZRBTDRZ_MONIKER_STANDING, //
        );
        zrbtdrz_expect(
            "reading-something-stands",
            code,
            &said,
            ZRBTDRZ_EXIT_SOMETHING_STANDS,
            &standing,
        )?;
        for passed in [
            format!(
                "{}{}{}{}",
                ZRBTDRZ_STANDS_FRAGMENT,
                ZRBTDRZ_OWN_PREFIX,
                ZRBTDRZ_DEPOT_INFIX,
                ZRBTDRZ_MONIKER_UNMADE, //
            ),
            format!(
                "{}{}",
                ZRBTDRZ_FOREIGN_PREFIX,
                ZRBTDRZ_DEPOT_INFIX, //
            ),
        ] {
            if said.contains(&passed) {
                return Err(format!(
                    "reading-something-stands: the reading named '{}', which it must pass over\n{}",
                    passed,
                    said, //
                ));
            }
        }
        Ok(())
    })())
}

/// NOTHING STANDS, BY A PLANTED ANSWER: under this tree's own prefix only an
/// unmade depot stands, beside a foreign prefix's standing depot, and the
/// reading answers that nothing stands.
fn rbtdrz_reading_nothing_stands(dir: &Path) -> rbtcee_Verdict {
    zrbtdrz_verdict((|| {
        let plants = [
            zrbtdrz_Plant {
                prefix: ZRBTDRZ_OWN_PREFIX,
                moniker: ZRBTDRZ_MONIKER_UNMADE,
                complete: false,
            },
            zrbtdrz_Plant {
                prefix: ZRBTDRZ_FOREIGN_PREFIX,
                moniker: ZRBTDRZ_MONIKER_FOREIGN,
                complete: true,
            },
        ];
        let (code, said) = zrbtdrz_read_planted(dir, "reading-nothing-stands", &plants)?;
        zrbtdrz_expect(
            "reading-nothing-stands",
            code,
            &said,
            ZRBTDRZ_EXIT_NOTHING_STANDS,
            ZRBTDRZ_NONE_FRAGMENT,
        )
    })())
}

// ── Cases and fixture ────────────────────────────────────────

pub static RBTDRZ_CASES_VOORLOPER_DOORS: &[rbtcee_Case] = &[
    rbtcee_case!(rbtdrz_prepare_fresh_clone),
    rbtcee_case!(rbtdrz_prepare_station_stands),
    rbtcee_case!(rbtdrz_prepare_field_filled),
    rbtcee_case!(rbtdrz_leftovers_no_credential),
    rbtcee_case!(rbtdrz_leftovers_could_not_ask),
    rbtcee_case!(rbtdrz_reading_something_stands),
    rbtcee_case!(rbtdrz_reading_nothing_stands), //
];

/// credless: a reveille member. Its five tabtarget launches are the two doors,
/// each guarded; the reading's planted answers ride a direct-kindle child,
/// never a launch.
pub static RBTDRZ_FIXTURE_VOORLOPER_DOORS: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTDRZ_NAME_VOORLOPER_DOORS,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRZ_CASES_VOORLOPER_DOORS,
    credless: true,
    tariff: rbtcee_Tariff {
        min_secs: None,
        max_secs: Some(60),
        invocations: Some(5),
    },
};
