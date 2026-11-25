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
// station beside the clone unless the case lays one. What HEAD carries differs
// by seat: a maintainer's tree and a billet carry live site values, a release
// probe branch carries feigned ones. So the success case stands its clone in the
// delivered shape through the proscription's own lustration, run by the clone's
// own modules with the clone as its working directory, and commits it there;
// the door must then admit it. Two controls part a door that judges from one
// that refuses everything: one site value planted in that same delivered shape,
// and the blanked state the release's zero verb leaves a maintainer's tree in,
// whose runtime prefix stands blank where the table gives a working default.
// The station case drives the door beside a station that already stands, on the
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
    RBTGEC_LIST_VESSELS,
    RBTGEC_MOORINGS_DIR,
    RBTGEC_RBRD_FILE,
    RBTGEC_RBRN_FILE,
    RBTGEC_RBRO_FILE,
    RBTGEC_RBRP_FILE,
    RBTGEC_RBRR_FILE,
    RBTGEC_RBRV_FILE,
    RBTGEC_RENDER_VESSEL,
    RBTGEC_VALIDATE_DEPOT,
    RBTGEC_VALIDATE_VESSEL,
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
const ZRBTDRZ_BUH_HANDBOOK: &str = "Tools/buk/buh_handbook.sh";

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
const ZRBTDRZ_STATION_BODY: &str =
    "BURS_USER=zqnewcomer\nBURS_TINCTURE=zqv\nBURS_LOG_DIR=../zqlogs\n";

/// The station body a refusal case lays, which the door must leave as it found.
const ZRBTDRZ_STANDING_STATION: &str =
    "BURS_USER=zqstanding\nBURS_TINCTURE=zqs\nBURS_LOG_DIR=../zqlogs\n";

/// The station field the prepare door sets.
const ZRBTDRZ_TINCTURE_FIELD: &str = "BURS_TINCTURE=";

/// The planted payor credential's body: present, never usable.
const ZRBTDRZ_PLANTED_CREDENTIAL: &str = "# zq planted payor credential, never read\n";

/// The identity a prepare case commits its clone's new shape under.
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

/// The site field the filled case plants, in the home that carries it, and the
/// value it plants there. Mirror: rblm_proscription.sh `zrblm_kindle`, whose row
/// gives this payor field a blank sterile value.
const ZRBTDRZ_PLANTED_FIELD: &str = "RBRP_PAYOR_PROJECT_ID";
const ZRBTDRZ_PLANTED_VALUE: &str = "zqplanted-payor";

/// The field the blanked state is refused on: the repo regime's first site
/// line, which zero leaves blank where the table gives its working default.
/// Mirror: rblm_proscription.sh `zrblm_kindle`, the runtime prefix's row and the
/// homes enrolled repo regime first.
const ZRBTDRZ_BLANKED_FIELD: &str = "RBRR_RUNTIME_PREFIX";

/// Each door's message, asserted beside its exit. Mirror: rblm_cli.sh
/// `rblm_voorloper_prepare` and `rblm_voorloper_leftovers`, rblm_proscription.sh
/// `zrblm_vet_regime`, and rbgp_payor.sh `zrbgp_depot_leftovers_read` — same
/// literals.
const ZRBTDRZ_STATION_STANDS_FRAGMENT: &str = "file already stands at";
const ZRBTDRZ_SHAPE_FRAGMENT: &str = "This tree is not in its delivered shape: ";
const ZRBTDRZ_NO_CREDENTIAL_FRAGMENT: &str = "holds no credential, so it founded nothing";
const ZRBTDRZ_COULD_NOT_ASK_FRAGMENT: &str = "the payor's ask ended with exit";
const ZRBTDRZ_STANDS_FRAGMENT: &str = "Depot stands: ";
const ZRBTDRZ_NONE_FRAGMENT: &str = "No depot stands under cloud prefix";

/// What the vessel doors say. Mirror: rbrv_cli.sh `rbrv_list` and
/// `rbrv_validate`, and buc_command.sh `buc_doc_env_row`, whose warning on an
/// unset variable a door must not print — same literals.
const ZRBTDRZ_VESSELS_LISTED_FRAGMENT: &str = "Available vessels:";
const ZRBTDRZ_VESSEL_VALID_FRAGMENT: &str = "RBRV vessel valid";
const ZRBTDRZ_UNSET_FRAGMENT: &str = "is not set";

/// The depot field the depot's validate is refused on in the delivered shape:
/// the first field its regime enrolls, which the proscription gives blank.
/// Mirror: rbrd_regime.sh `zrbrd_kindle` and rblm_proscription.sh
/// `zrblm_kindle`, the cloud prefix's row.
const ZRBTDRZ_DEPOT_BLANK_FIELD: &str = "RBRD_CLOUD_PREFIX";

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

/// Drive the clone's door for `colophon` from inside the clone, handing it
/// `args`, its run roots under the case, and keep what it said.
fn zrbtdrz_drive(
    dir: &Path,
    clone: &Path,
    colophon: &str,
    label: &str,
    args: &[&str],
) -> Result<(i32, String), String> {
    let door = zrbtdrz_door(clone, colophon)?;
    let mut cmd = rbtdri_rooted_tabtarget_command(&door, dir)?;
    for arg in args {
        cmd.arg(arg);
    }
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

/// Commit whatever the case changed in its clone, under the fixture's identity,
/// a shape that changed nothing included, so the door meets a clean tree.
fn zrbtdrz_commit(clone: &Path, message: &str) -> Result<(), String> {
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
            "--allow-empty",
            "-m",
            message,
        ],
        "commit the clone's new shape",
    )?;
    Ok(())
}

/// Stand the clone in the delivered shape: the proscription's own lustration,
/// sourced from the clone's own modules in a child whose working directory is
/// the clone, where every home and the hardpoint resolve. It must start nowhere
/// else, since lustration rewrites the homes it finds in place. The shape is
/// committed in the clone.
fn zrbtdrz_deliver_clone(dir: &Path, clone: &Path, label: &str) -> Result<(), String> {
    let temp = dir.join(format!(
        "{}-lustrate-temp",
        label, //
    ));
    zrbtdrz_mkdir(&temp)?;
    let module = |rel: &str| rbtcep_native_to_posix(&clone.join(rel));
    let script = format!(
        "set -euo pipefail\n\
         export BURD_TEMP_DIR='{temp}'\n\
         source '{buc}'\n\
         source '{buym}'\n\
         source '{buh}'\n\
         source '{bubc}'\n\
         source '{buv}'\n\
         source '{rbk}/rbcc_constants.sh'\n\
         source '{rbk}/rblm_proscription.sh'\n\
         zbuh_kindle\n\
         zbuv_kindle\n\
         zrblm_kindle\n\
         rblm_lustrate_apply\n",
        temp = rbtcep_native_to_posix(&temp),
        buc = module(ZRBTDRZ_BUC_COMMAND),
        buym = module(ZRBTDRZ_BUYM_YELP),
        buh = module(ZRBTDRZ_BUH_HANDBOOK),
        bubc = module(ZRBTDRZ_BUBC_CONSTANTS),
        buv = module(RBTCHF_BUV_VALIDATION),
        rbk = module(RBTCHF_RBK_ROOT), //
    );
    let lustrate_label = format!(
        "{}-lustrate",
        label, //
    );
    match rbtchf_run_bash(clone, &script, dir, &lustrate_label)? {
        (0, _, _) => {}
        (code, _, stderr) => {
            return Err(format!(
                "{}: lustration of the clone exited {}: {}",
                label,
                code,
                stderr.trim(), //
            ));
        }
    }
    zrbtdrz_commit(clone, "zq delivered shape")
}

/// Plant one value on a regime file's `FIELD=` line, keeping every other line
/// as it stood; a file carrying no such line is an error.
fn zrbtdrz_plant(path: &Path, field: &str, value: &str) -> Result<(), String> {
    let body = zrbtdrz_read(path)?;
    let mut out = String::new();
    let mut found = false;
    for line in body.lines() {
        match line.split_once('=') {
            Some((f, _)) if f == field => {
                out.push_str(&format!(
                    "{}={}",
                    field,
                    value, //
                ));
                found = true;
            }
            _ => out.push_str(line),
        }
        out.push('\n');
    }
    if !found {
        return Err(format!(
            "{} carries no {} line to plant",
            path.display(),
            field, //
        ));
    }
    zrbtdrz_write(path, &out)
}

/// Require a prepare refusal naming `field` in `file`, with no station written.
fn zrbtdrz_expect_shape_refusal(
    label: &str,
    code: i32,
    said: &str,
    station: &Path,
    field: &str,
    file: &str,
) -> Result<(), String> {
    if code == 0 {
        return Err(format!(
            "{}: exited 0 over a tree not in its delivered shape\n{}",
            label,
            said, //
        ));
    }
    let fragment = format!(
        "{}{} in {}",
        ZRBTDRZ_SHAPE_FRAGMENT,
        field,
        file, //
    );
    zrbtdrz_expect(label, code, said, code, &fragment)?;
    if station.exists() {
        return Err(format!(
            "{}: the refusal left {} standing",
            label,
            station.display(), //
        ));
    }
    Ok(())
}

/// Stand the clone in the blanked state: blank zero's blank set and commit it
/// in the clone, as the release's zero verb leaves a maintainer's tree.
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
    for vessel in zrbtdrz_regimes_beneath(&moorings.join(RBTGEC_VESSELS_SUBDIR), RBTGEC_RBRV_FILE)?
    {
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

/// PREPARE STANDS A FRESH CLONE'S STATION: in a clone stood in the delivered
/// shape and committed, beside no station file, the door exits 0, leaves the
/// station file beside the clone wearing the fixed tincture on its one tincture
/// line, and leaves the clone's tree clean.
fn rbtdrz_prepare_fresh_clone(dir: &Path) -> rbtcee_Verdict {
    zrbtdrz_verdict((|| {
        let clone = zrbtdrz_clone(dir)?;
        zrbtdrz_deliver_clone(dir, &clone, "prepare-fresh-clone")?;
        let station = zrbtdrz_station(&clone)?;
        if station.exists() {
            return Err(format!(
                "prepare-fresh-clone: {} stands before the door ran",
                station.display(), //
            ));
        }
        let (code, said) =
            zrbtdrz_drive(dir, &clone, RBTGEC_VOORLOPER_PREPARE, "prepare-fresh-clone", &[])?;
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
        let status = zrbtdrz_git(
            &clone,
            &["status", "--porcelain"],
            "read the clone's status",
        )?;
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
        let (code, said) = zrbtdrz_drive(
            dir,
            &clone,
            RBTGEC_VOORLOPER_PREPARE,
            "prepare-already-standing",
            &[],
        )?;
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

/// PREPARE REFUSES A SITE VALUE THE TABLE GIVES BLANK: in a clone stood in the
/// delivered shape, one payor field the table gives blank is planted and
/// committed, and the door refuses naming that field and its file, and writes no
/// station.
fn rbtdrz_prepare_field_filled(dir: &Path) -> rbtcee_Verdict {
    zrbtdrz_verdict((|| {
        let label = "prepare-field-filled";
        let clone = zrbtdrz_clone(dir)?;
        zrbtdrz_deliver_clone(dir, &clone, label)?;
        zrbtdrz_plant(
            &clone.join(RBTGEC_RBRP_FILE),
            ZRBTDRZ_PLANTED_FIELD,
            ZRBTDRZ_PLANTED_VALUE,
        )?;
        zrbtdrz_commit(&clone, "zq one site value planted")?;
        let station = zrbtdrz_station(&clone)?;
        let (code, said) = zrbtdrz_drive(dir, &clone, RBTGEC_VOORLOPER_PREPARE, label, &[])?;
        zrbtdrz_expect_shape_refusal(
            label,
            code,
            &said,
            &station,
            ZRBTDRZ_PLANTED_FIELD,
            RBTGEC_RBRP_FILE,
        )
    })())
}

/// PREPARE REFUSES THE BLANKED STATE: in a clone blanked as the release's zero
/// verb leaves a maintainer's tree and committed, its runtime prefix blank where
/// the table gives a working default and its payor's fields as the tree had
/// them, the door refuses naming the runtime prefix and its file, and writes no
/// station.
fn rbtdrz_prepare_blanked_state(dir: &Path) -> rbtcee_Verdict {
    zrbtdrz_verdict((|| {
        let label = "prepare-blanked-state";
        let clone = zrbtdrz_clone(dir)?;
        zrbtdrz_zero_clone(&clone)?;
        let station = zrbtdrz_station(&clone)?;
        let (code, said) = zrbtdrz_drive(dir, &clone, RBTGEC_VOORLOPER_PREPARE, label, &[])?;
        zrbtdrz_expect_shape_refusal(
            label,
            code,
            &said,
            &station,
            ZRBTDRZ_BLANKED_FIELD,
            RBTGEC_RBRR_FILE,
        )
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
        let (code, said) = zrbtdrz_drive(
            dir,
            &clone,
            RBTGEC_VOORLOPER_LEFTOVERS,
            "leftovers-no-credential",
            &[],
        )?;
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
        let (code, said) = zrbtdrz_drive(
            dir,
            &clone,
            RBTGEC_VOORLOPER_LEFTOVERS,
            "leftovers-could-not-ask",
            &[],
        )?;
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

// ── The vessel doors ─────────────────────────────────────────

/// What a door said with its terminal escapes taken out, so a tinted line reads
/// as its plain text.
fn zrbtdrz_plain(said: &str) -> String {
    let mut out = String::new();
    let mut chars = said.chars();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            for e in chars.by_ref() {
                if e.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// The sigils the vessel listing printed: each indented one-word line that
/// follows the line announcing them, up to the first line that is not one.
fn zrbtdrz_listed_sigils(said: &str) -> Vec<String> {
    let plain = zrbtdrz_plain(said);
    let mut lines = plain
        .lines()
        .skip_while(|line| !line.contains(ZRBTDRZ_VESSELS_LISTED_FRAGMENT));
    lines.next();
    lines
        .take_while(|line| {
            let word = line.trim();
            line.starts_with(' ') && !word.is_empty() && !word.contains(char::is_whitespace)
        })
        .map(|line| line.trim().to_string())
        .collect()
}

/// THE VESSEL DOORS ANSWER ON A FRESH CLONE: in a clone stood in the delivered
/// shape and committed, beside a station, the vessel listing exits 0 and prints
/// its vessels with no warning of an unset variable; the vessel render and the
/// vessel validate each exit 0 for every vessel it printed, the validate saying
/// the vessel is valid; and the depot's own validate still refuses, naming the
/// blank cloud prefix.
fn rbtdrz_vessel_doors_fresh_clone(dir: &Path) -> rbtcee_Verdict {
    zrbtdrz_verdict((|| {
        let label = "vessel-doors-fresh-clone";
        let clone = zrbtdrz_clone(dir)?;
        zrbtdrz_deliver_clone(dir, &clone, label)?;
        zrbtdrz_lay_station(dir, &clone)?;
        let list_label = format!(
            "{}-list",
            label, //
        );
        let (code, said) = zrbtdrz_drive(dir, &clone, RBTGEC_LIST_VESSELS, &list_label, &[])?;
        zrbtdrz_expect(&list_label, code, &said, 0, ZRBTDRZ_VESSELS_LISTED_FRAGMENT)?;
        if said.contains(ZRBTDRZ_UNSET_FRAGMENT) {
            return Err(format!(
                "{}: the listing warned '{}'\n{}",
                list_label,
                ZRBTDRZ_UNSET_FRAGMENT,
                said, //
            ));
        }
        let sigils = zrbtdrz_listed_sigils(&said);
        if sigils.is_empty() {
            return Err(format!(
                "{}: the listing printed no vessel\n{}",
                list_label,
                said, //
            ));
        }
        for sigil in &sigils {
            let render_label = format!(
                "{}-render-{}",
                label,
                sigil, //
            );
            let (code, said) = zrbtdrz_drive(
                dir,
                &clone,
                RBTGEC_RENDER_VESSEL,
                &render_label,
                &[sigil.as_str()],
            )?;
            zrbtdrz_expect(&render_label, code, &said, 0, sigil)?;
            let validate_label = format!(
                "{}-validate-{}",
                label,
                sigil, //
            );
            let (code, said) = zrbtdrz_drive(
                dir,
                &clone,
                RBTGEC_VALIDATE_VESSEL,
                &validate_label,
                &[sigil.as_str()],
            )?;
            zrbtdrz_expect(&validate_label, code, &said, 0, ZRBTDRZ_VESSEL_VALID_FRAGMENT)?;
        }
        let depot_label = format!(
            "{}-depot",
            label, //
        );
        let (code, said) = zrbtdrz_drive(dir, &clone, RBTGEC_VALIDATE_DEPOT, &depot_label, &[])?;
        if code == 0 {
            return Err(format!(
                "{}: exited 0 over a blank depot\n{}",
                depot_label,
                said, //
            ));
        }
        zrbtdrz_expect(&depot_label, code, &said, code, ZRBTDRZ_DEPOT_BLANK_FIELD)
    })())
}

// ── Cases and fixture ────────────────────────────────────────

pub static RBTDRZ_CASES_VOORLOPER_DOORS: &[rbtcee_Case] = &[
    rbtcee_case!(rbtdrz_prepare_fresh_clone),
    rbtcee_case!(rbtdrz_prepare_station_stands),
    rbtcee_case!(rbtdrz_prepare_field_filled),
    rbtcee_case!(rbtdrz_prepare_blanked_state),
    rbtcee_case!(rbtdrz_leftovers_no_credential),
    rbtcee_case!(rbtdrz_leftovers_could_not_ask),
    rbtcee_case!(rbtdrz_reading_something_stands),
    rbtcee_case!(rbtdrz_reading_nothing_stands),
    rbtcee_case!(rbtdrz_vessel_doors_fresh_clone), //
];

/// credless: a reveille member. Its thirty tabtarget launches, each guarded:
/// six are the two doors, one per prepare and leftovers case; twenty-four are
/// the vessel-doors case's, one vessel listing, a render and a validate for
/// each of the eleven vessels the tree carries, and one depot validate. A vessel
/// added or removed moves the count by two. The lustrations and the reading's
/// planted answers ride direct-kindle children, never a launch.
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
        invocations: Some(30),
    },
};
