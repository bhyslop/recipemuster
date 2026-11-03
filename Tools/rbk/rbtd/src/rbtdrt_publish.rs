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
// RBTDRT — the publish path's standing proof: a credless reveille member that
// drives the path a release takes, every case over scratch of its own and never
// over the tree it runs in.
//
// Lustration. The moorings homes and the hardpoint the proscription names are
// copied into a scratch root beneath the case, lustration runs there through
// the proscription module, and damnatio's own value case reads the result back,
// its value reader proving itself first. No second reader stands in Rust: the
// comparison is damnatio's, over the scratch root. A second case plants a live
// value on an export line after lustration and passes only when the reader
// names the planted field, so the reader is proven whatever the scrub learns.
//
// The pool build poll. The await is driven in a direct-kindle child that kindles
// no credential module: the token is a dummy argument, `curl` and `sleep` are
// shadowed by harness functions, and the HTTP fault seam rides the child's own
// script. The forced-503 arm serves its first poll a 200 carrying no status, so
// only a code-first await polls on to the SUCCESS that follows; a body-first
// await, or a seam that rode inert, dies. The FAILURE arm is the served body's.
//
// The marshal refusals. Each case drives its door in a scratch clone of the
// committed tree, beside a station of its own, the door stood from a shipped
// donor where the tree withholds it, under the skip confirmation so a refusal
// proves it stands ahead of the confirmation. Each asserts the marshal gate's
// band code and its rule's message, since one gate holds three rules.

use std::path::{
    Path,
    PathBuf, //
};
use std::process::Command;

use bkp::bkprs_scrivener::bkprs_program_copy;

use crate::rbtcee_case;
use crate::rbtcee_engine::{
    rbtcee_Case,
    rbtcee_Disposition,
    rbtcee_Fixture,
    rbtcee_Tariff,
    rbtcee_Verdict, //
};
use crate::rbtcei_invocation::{
    rbtcei_bash_program,
    rbtdri_rooted_tabtarget_command,
    RBTCEI_BURE_CONFIRM_KEY,
    RBTCEI_BURE_CONFIRM_SKIP, //
};
use crate::rbtcep_platform::rbtcep_native_to_posix;
use crate::rbtchd_damnatio::{
    zrbtdrq_proscribed_findings,
    RBTCHD_DAMNATIO_SELF,
    ZRBTCHD_DISPOSITION_SITE,
    ZRBTDRQ_DETAIL_UNSANCTIONED, //
};
use crate::rbtchf_fast::{
    rbtchf_run_bash,
    RBTCHF_BUV_VALIDATION,
    RBTCHF_RBK_ROOT, //
};
use crate::rbtchx_pyx::{
    zrbtchx_report,
    zrbtchx_root, //
};
use crate::rbtgeb_breviary::RBTGEB_MARSHAL_DENTIL;
use crate::rbtgec_consts::{
    RBTGEC_MARSHAL_FEIGN,
    RBTGEC_MARSHAL_ZERO,
    RBTGEC_MOORINGS_DIR,
    RBTGEC_QUALIFY_FAST, //
};

pub const RBTDRT_NAME_PUBLISH_PATH: &str = "publish-path";

// Repo-root-relative kit modules the direct-kindle children source, beside the
// two the fast fixtures already name.
const ZRBTDRT_BUC_COMMAND: &str = "Tools/buk/buc_command.sh";
const ZRBTDRT_BUYM_YELP: &str = "Tools/buk/buym_yelp.sh";
const ZRBTDRT_BUH_HANDBOOK: &str = "Tools/buk/buh_handbook.sh";
const ZRBTDRT_BUBC_CONSTANTS: &str = "Tools/buk/bubc_constants.sh";
const ZRBTDRT_BURD_REGIME: &str = "Tools/buk/burd_regime.sh";

fn zrbtdrt_verdict(outcome: Result<(), String>) -> rbtcee_Verdict {
    match outcome {
        Ok(()) => rbtcee_Verdict::Pass,
        Err(e) => rbtcee_Verdict::Fail(e),
    }
}

fn zrbtdrt_mkdir(path: &Path) -> Result<(), String> {
    std::fs::create_dir_all(path).map_err(|e| {
        format!(
            "cannot create {}: {}",
            path.display(),
            e, //
        )
    })
}

fn zrbtdrt_write(path: &Path, body: &str) -> Result<(), String> {
    std::fs::write(path, body).map_err(|e| {
        format!(
            "cannot write {}: {}",
            path.display(),
            e, //
        )
    })
}

/// A kit module's absolute POSIX path under the tree the theurge runs in.
fn zrbtdrt_module(root: &Path, rel: &str) -> String {
    rbtcep_native_to_posix(&root.join(rel))
}

// ── Lustration over a scratch copy ───────────────────────────

/// The scratch root beneath a case, where the copied homes stand at their
/// tree-relative paths.
const ZRBTDRT_SCRATCH_ROOT: &str = "zqlustral";

/// Section markers in the listing child's output.
const ZRBTDRT_MARK_PROSCRIPTION: &str = "##PROSCRIPTION";
const ZRBTDRT_MARK_HOMES: &str = "##HOMES";
const ZRBTDRT_MARK_HARDPOINTS: &str = "##HARDPOINTS";

/// The synthetic value the export plant assigns.
const ZRBTDRT_PLANT_VALUE: &str = "zqplantedlive";

/// What the proscription judges, relative to the tree: every path to copy (each
/// home, the repo regime file among them, and each hardpoint), and the plant's
/// target — the first home and the first site field its scope judges.
struct zrbtdrt_Judged {
    paths: Vec<String>,
    plant_home: String,
    plant_field: String,
}

/// Read what the proscription judges from the proscription module itself, in a
/// child started at the tree, so no list and no field is copied here.
fn zrbtdrt_judged_paths(root: &Path, dir: &Path) -> Result<zrbtdrt_Judged, String> {
    let script = format!(
        "set -euo pipefail\n\
         source '{buv}'\n\
         source '{rbk}/rbcc_constants.sh'\n\
         source '{rbk}/rblm_proscription.sh'\n\
         zbuv_kindle\n\
         zrblm_kindle\n\
         echo '{mark_proscription}'\n\
         rblm_emit_proscription\n\
         echo '{mark_homes}'\n\
         rblm_emit_homes\n\
         echo '{mark_hardpoints}'\n\
         rblm_emit_hardpoints\n",
        buv = zrbtdrt_module(root, RBTCHF_BUV_VALIDATION),
        rbk = zrbtdrt_module(root, RBTCHF_RBK_ROOT),
        mark_proscription = ZRBTDRT_MARK_PROSCRIPTION,
        mark_homes = ZRBTDRT_MARK_HOMES,
        mark_hardpoints = ZRBTDRT_MARK_HARDPOINTS, //
    );
    let stdout = match rbtchf_run_bash(root, &script, dir, "judged-paths")? {
        (0, stdout, _) => stdout,
        (code, _, stderr) => {
            return Err(format!(
                "the proscription's listing exited {}: {}",
                code,
                stderr.trim(), //
            ));
        }
    };
    let mut paths = Vec::new();
    let mut homes: Vec<(String, String)> = Vec::new();
    let mut site: Vec<(String, String)> = Vec::new();
    let mut section = "";
    for line in stdout.lines() {
        if line == ZRBTDRT_MARK_PROSCRIPTION
            || line == ZRBTDRT_MARK_HOMES
            || line == ZRBTDRT_MARK_HARDPOINTS
        {
            section = line;
            continue;
        }
        let fields: Vec<&str> = line.split('\t').collect();
        match section {
            ZRBTDRT_MARK_PROSCRIPTION
                if fields.len() >= 3 && fields[2] == ZRBTCHD_DISPOSITION_SITE =>
            {
                site.push((fields[0].to_string(), fields[1].to_string()));
            }
            ZRBTDRT_MARK_HOMES if fields.len() == 2 => {
                paths.push(fields[1].to_string());
                homes.push((fields[0].to_string(), fields[1].to_string()));
            }
            ZRBTDRT_MARK_HARDPOINTS if fields.len() == 3 => paths.push(fields[0].to_string()),
            _ => {}
        }
    }
    let (scope, plant_home) = homes
        .into_iter()
        .next()
        .ok_or_else(|| "the proscription listed no home".to_string())?;
    let plant_field = site
        .into_iter()
        .find(|(s, _)| *s == scope)
        .map(|(_, var)| var)
        .ok_or_else(|| {
            format!(
                "the proscription judges no site field of {}, the first home's scope",
                scope, //
            )
        })?;
    Ok(zrbtdrt_Judged {
        paths,
        plant_home,
        plant_field,
    })
}

/// Copy every judged path into a fresh scratch root beneath the case, then
/// lustrate there. The child starts in the scratch root, where every home and
/// the hardpoint resolve, and sources the kit by absolute path from the tree;
/// it must not start anywhere else, since lustration rewrites the homes it
/// finds in place.
fn zrbtdrt_lustrated_root(root: &Path, dir: &Path) -> Result<(PathBuf, zrbtdrt_Judged), String> {
    let scratch = dir.join(ZRBTDRT_SCRATCH_ROOT);
    zrbtdrt_mkdir(&scratch)?;
    let judged = zrbtdrt_judged_paths(root, dir)?;
    for rel in &judged.paths {
        let from = root.join(rel);
        let body = std::fs::read(&from).map_err(|e| {
            format!(
                "cannot read {}: {}",
                from.display(),
                e, //
            )
        })?;
        let to = scratch.join(rel);
        if let Some(parent) = to.parent() {
            zrbtdrt_mkdir(parent)?;
        }
        std::fs::write(&to, body).map_err(|e| {
            format!(
                "cannot write {}: {}",
                to.display(),
                e, //
            )
        })?;
    }

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
        temp = rbtcep_native_to_posix(dir),
        buc = zrbtdrt_module(root, ZRBTDRT_BUC_COMMAND),
        buym = zrbtdrt_module(root, ZRBTDRT_BUYM_YELP),
        buh = zrbtdrt_module(root, ZRBTDRT_BUH_HANDBOOK),
        bubc = zrbtdrt_module(root, ZRBTDRT_BUBC_CONSTANTS),
        buv = zrbtdrt_module(root, RBTCHF_BUV_VALIDATION),
        rbk = zrbtdrt_module(root, RBTCHF_RBK_ROOT), //
    );
    match rbtchf_run_bash(&scratch, &script, dir, "lustrate")? {
        (0, _, _) => Ok((scratch, judged)),
        (code, _, stderr) => Err(format!(
            "lustration over the scratch root exited {}: {}",
            code,
            stderr.trim(), //
        )),
    }
}

/// A LUSTRATED COPY IS STERILE: every site field of every home, and every
/// hardpoint, holds its sterile value once lustration has run, as damnatio's
/// value case reads it.
fn rbtdrt_lustration_sterile(dir: &Path) -> rbtcee_Verdict {
    zrbtdrt_verdict((|| {
        let root = zrbtchx_root()?;
        let (scratch, _) = zrbtdrt_lustrated_root(&root, dir)?;
        match crate::rbtchd_damnatio::zrbtdrq_proscribed_verdict(&root, &scratch, dir) {
            rbtcee_Verdict::Pass => Ok(()),
            rbtcee_Verdict::Fail(e) => Err(e),
            rbtcee_Verdict::Skip(e) => Err(e),
        }
    })())
}

/// A LIVE VALUE PLANTED ON AN EXPORT LINE REDS: after lustration a site field
/// is assigned on an export line in the first scratch home, and the case
/// passes only when the value case names that field, the reader's self-proof
/// clean beside it.
fn rbtdrt_lustration_export_plant(dir: &Path) -> rbtcee_Verdict {
    zrbtdrt_verdict((|| {
        let root = zrbtchx_root()?;
        let (scratch, judged) = zrbtdrt_lustrated_root(&root, dir)?;
        let home = judged.plant_home;
        let field = judged.plant_field;
        let path = scratch.join(&home);
        let mut body = std::fs::read_to_string(&path).map_err(|e| {
            format!(
                "cannot read {}: {}",
                path.display(),
                e, //
            )
        })?;
        body.push_str(&format!(
            "export {}={}\n",
            field,
            ZRBTDRT_PLANT_VALUE, //
        ));
        zrbtdrt_write(&path, &body)?;

        let (findings, inventory) = zrbtdrq_proscribed_findings(&root, &scratch, dir)?;
        let _ = zrbtchx_report(
            dir,
            "proscribed",
            &findings,
            &inventory,
            "unsterilized site value(s)",
        );
        if let Some(blind) = findings.iter().find(|f| f.file == RBTCHD_DAMNATIO_SELF) {
            return Err(format!(
                "the value reader failed its self-proof: {}",
                blind.detail, //
            ));
        }
        let named = format!(
            "{} {}",
            field,
            ZRBTDRQ_DETAIL_UNSANCTIONED, //
        );
        if findings.iter().any(|f| f.file == home && f.detail == named) {
            Ok(())
        } else {
            Err(format!(
                "the export plant of {} in {} went unnamed: {} finding(s), none naming it",
                field,
                home,
                findings.len(), //
            ))
        }
    })())
}

// ── The pool build poll ──────────────────────────────────────

/// The await's identities in the harness, all synthetic. A poll's infix is the
/// prefix, the variant and the poll's ordinal joined as the await joins them.
/// Mirror: rbgp_payor.sh `zrbgp_pool_build_submit_await`, its poll infix line.
const ZRBTDRT_POLL_PREFIX: &str = "zqpoll";
const ZRBTDRT_POLL_VARIANT: &str = "zqvariant";
const ZRBTDRT_POLL_INFIX_JOIN: &str = "_poll_";
const ZRBTDRT_POLL_LABEL: &str = "PollProof";

/// The HTTP code the seam forces over the first poll.
const ZRBTDRT_POLL_FORCED: &str = "503";

/// The served bodies: the submit naming the build, a 200 carrying no status, and
/// the two terminal statuses. Synthetic throughout.
const ZRBTDRT_SERVED_SUBMIT: &str = "200\t{\"metadata\":{\"build\":{\"id\":\"zqbuild\"}}}";
const ZRBTDRT_SERVED_STATUSLESS: &str = "200\t{\"id\":\"zqbuild\"}";
const ZRBTDRT_SERVED_SUCCESS: &str = "200\t{\"status\":\"SUCCESS\"}";
const ZRBTDRT_SERVED_FAILURE: &str = "200\t{\"status\":\"FAILURE\"}";

/// The code a served row opens with where the await read a 200.
const ZRBTDRT_SERVED_OK_HEAD: &str = "200\t";

/// The await's lines the arms assert. Mirror: rbgp_payor.sh
/// `zrbgp_pool_build_submit_await` — same literals.
const ZRBTDRT_POLL_ON_FRAGMENT: &str = "HTTP 503, polling on";
const ZRBTDRT_TERMINAL_SUCCESS_FRAGMENT: &str = "terminal = SUCCESS";
const ZRBTDRT_ENDED_FAILURE_FRAGMENT: &str = "ended FAILURE, not SUCCESS";

/// The harness's stand-ins for the transport and the wait. `curl` reads the
/// response file from its options, drains the header list from its standard
/// input, serves the next row of the sequence file — the body to the response
/// file and the code on its standard output — and logs each row it served;
/// a sequence run dry serves a 404. `sleep` returns at once. A child the
/// await's substitutions spawn inherits both, so the count rides a file.
const ZRBTDRT_POLL_STUBS: &str = r#"sleep() { :; }
curl() {
  local z_out=""
  while test "$#" -gt 0; do
    case "$1" in
      -o) z_out="${2:-}"; shift ;;
    esac
    shift
  done
  test -n "${z_out}" || return 2
  cat > /dev/null
  local z_n=""
  z_n=$(<"${z_count}")
  z_n=$((z_n + 1))
  printf '%s' "${z_n}" > "${z_count}"
  local z_line=""
  z_line=$(sed -n "${z_n}p" "${z_sequence}")
  test -n "${z_line}" || z_line="${z_exhausted}"
  printf '%s\n' "${z_line}" >> "${z_served}"
  printf '%s' "${z_line#*$'\t'}" > "${z_out}"
  printf '%s' "${z_line%%$'\t'*}"
}
"#;

/// What one harness run left behind: its exit, both streams together, and the
/// rows the stub served.
struct zrbtdrt_Polled {
    code: i32,
    said: String,
    served: Vec<String>,
}

/// Drive the pool build await once over a served sequence, the fault seam aimed
/// at `fault` where one is given and the slot left empty otherwise. The child
/// kindles the HTTP road and the payor module and nothing that mints: the
/// regime values the await reads are set synthetic, and no regime kindles.
fn zrbtdrt_poll(
    dir: &Path,
    label: &str,
    sequence: &[&str],
    fault: Option<&str>,
) -> Result<zrbtdrt_Polled, String> {
    let root = zrbtchx_root()?;
    let temp = dir.join(format!(
        "{}-temp",
        label, //
    ));
    zrbtdrt_mkdir(&temp)?;
    let sequence_path = dir.join(format!(
        "{}-sequence.txt",
        label, //
    ));
    let served_path = dir.join(format!(
        "{}-served.txt",
        label, //
    ));
    let count_path = dir.join(format!(
        "{}-count.txt",
        label, //
    ));
    let build_path = dir.join(format!(
        "{}-build.json",
        label, //
    ));
    let mut rows = sequence.join("\n");
    rows.push('\n');
    zrbtdrt_write(&sequence_path, &rows)?;
    zrbtdrt_write(&served_path, "")?;
    zrbtdrt_write(&count_path, "0")?;
    zrbtdrt_write(&build_path, "{}")?;

    let tweak = match fault {
        Some(spec) => format!(
            "BURE_TWEAK_NAME=\"${{RBCC_tweak_http_fault}}\"\n\
             BURE_TWEAK_VALUE='{}'\n",
            spec, //
        ),
        None => "unset BURE_TWEAK_NAME BURE_TWEAK_VALUE\n".to_string(),
    };
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
         z_sequence='{sequence}'\n\
         z_served='{served}'\n\
         z_count='{count}'\n\
         z_exhausted=$'404\\t{{}}'\n\
         {stubs}\
         {tweak}\
         zrbgp_pool_build_submit_await 'zqtoken' '{variant}' 'zqpool' '{build}' '{prefix}' '{op}' ''\n",
        temp = rbtcep_native_to_posix(&temp),
        moorings = RBTGEC_MOORINGS_DIR,
        buc = zrbtdrt_module(&root, ZRBTDRT_BUC_COMMAND),
        buym = zrbtdrt_module(&root, ZRBTDRT_BUYM_YELP),
        bubc = zrbtdrt_module(&root, ZRBTDRT_BUBC_CONSTANTS),
        buv = zrbtdrt_module(&root, RBTCHF_BUV_VALIDATION),
        burd = zrbtdrt_module(&root, ZRBTDRT_BURD_REGIME),
        rbk = zrbtdrt_module(&root, RBTCHF_RBK_ROOT),
        sequence = rbtcep_native_to_posix(&sequence_path),
        served = rbtcep_native_to_posix(&served_path),
        count = rbtcep_native_to_posix(&count_path),
        stubs = ZRBTDRT_POLL_STUBS,
        tweak = tweak,
        variant = ZRBTDRT_POLL_VARIANT,
        build = rbtcep_native_to_posix(&build_path),
        prefix = ZRBTDRT_POLL_PREFIX,
        op = ZRBTDRT_POLL_LABEL, //
    );
    let (code, stdout, stderr) = rbtchf_run_bash(&root, &script, dir, label)?;
    let served = std::fs::read_to_string(&served_path)
        .map_err(|e| {
            format!(
                "cannot read {}: {}",
                served_path.display(),
                e, //
            )
        })?
        .lines()
        .map(str::to_string)
        .collect();
    Ok(zrbtdrt_Polled {
        code,
        said: format!(
            "--- stdout ---\n{}\n--- stderr ---\n{}",
            stdout,
            stderr, //
        ),
        served,
    })
}

/// A 5XX ON A POLL POLLS ON: the seam forces the first poll's code to 503 over a
/// 200 carrying no status, and the await polls on to the SUCCESS served next.
/// Exactly three rows are served — the submit and two polls — so no cancel ran.
fn rbtdrt_poll_forced_503(dir: &Path) -> rbtcee_Verdict {
    zrbtdrt_verdict((|| {
        let fault = format!(
            "{}_{}{}{}={}",
            ZRBTDRT_POLL_PREFIX,
            ZRBTDRT_POLL_VARIANT,
            ZRBTDRT_POLL_INFIX_JOIN,
            1,
            ZRBTDRT_POLL_FORCED, //
        );
        let polled = zrbtdrt_poll(
            dir,
            "poll-503",
            &[
                ZRBTDRT_SERVED_SUBMIT,
                ZRBTDRT_SERVED_STATUSLESS,
                ZRBTDRT_SERVED_SUCCESS, //
            ],
            Some(&fault),
        )?;
        if polled.code != 0 {
            return Err(format!(
                "the await exited {} where the forced 503 should have polled on\n{}",
                polled.code,
                polled.said, //
            ));
        }
        for needle in [ZRBTDRT_POLL_ON_FRAGMENT, ZRBTDRT_TERMINAL_SUCCESS_FRAGMENT] {
            if !polled.said.contains(needle) {
                return Err(format!(
                    "the await did not say '{}'\n{}",
                    needle,
                    polled.said, //
                ));
            }
        }
        if polled.served.len() != 3 {
            return Err(format!(
                "the stub served {} row(s), not the submit and two polls: {:?}",
                polled.served.len(),
                polled.served, //
            ));
        }
        if !polled.served[1].starts_with(ZRBTDRT_SERVED_OK_HEAD) {
            return Err(format!(
                "the first poll was served '{}', not a 200 for the seam to force",
                polled.served[1], //
            ));
        }
        Ok(())
    })())
}

/// A BUILD ENDING FAILURE DIES: the first poll is served FAILURE and the await
/// dies naming the build's end.
fn rbtdrt_poll_ended_failure(dir: &Path) -> rbtcee_Verdict {
    zrbtdrt_verdict((|| {
        let polled = zrbtdrt_poll(
            dir,
            "poll-failure",
            &[
                ZRBTDRT_SERVED_SUBMIT,
                ZRBTDRT_SERVED_FAILURE, //
            ],
            None,
        )?;
        if polled.code == 0 {
            return Err(format!(
                "the await exited 0 over a build that ended FAILURE\n{}",
                polled.said, //
            ));
        }
        if !polled.said.contains(ZRBTDRT_ENDED_FAILURE_FRAGMENT) {
            return Err(format!(
                "the await exited {} without saying '{}'\n{}",
                polled.code,
                ZRBTDRT_ENDED_FAILURE_FRAGMENT,
                polled.said, //
            ));
        }
        Ok(())
    })())
}

// ── The marshal refusals ─────────────────────────────────────

/// The scratch clone's basename, and the folio zero is handed, which is not it.
const ZRBTDRT_CLONE: &str = "zqtree";
const ZRBTDRT_ZERO_FOLIO: &str = "zqothertree";

/// The branch the regulus-shaped clone stands on: any branch but the probe.
const ZRBTDRT_OFF_PROBE_BRANCH: &str = "zqregulus";

/// The clone's remote, as git names the origin a clone keeps.
const ZRBTDRT_ORIGIN: &str = "origin";

/// The station a clone's dispatch reads: the key its config regime names the
/// station file under, the synthetic station's values, and the stores it names
/// one level up from the clone.
const ZRBTDRT_STATION_KEY: &str = "BURC_STATION_FILE=";
const ZRBTDRT_STATION_USER: &str = "zqmarshal";
const ZRBTDRT_STATION_TINCTURE: &str = "zqm";
const ZRBTDRT_STATION_LOGS: &str = "logs-buk";
const ZRBTDRT_STATION_TACKROOM: &str = "tackroom-buk";
const ZRBTDRT_CONFIG_REGIME: &str = "burc.env";

/// The tabtarget directory, and the frontispieces a door stood from a donor is
/// given (dispatch reads only the colophon, the first dot-token).
const ZRBTDRT_TT_DIR: &str = "tt";
const ZRBTDRT_FRONT_ZERO: &str = "MarshalZeroes";
const ZRBTDRT_FRONT_FEIGN: &str = "MarshalFeigns";

/// The clone's own constants module, the one declaration of the probe branch,
/// and the bash child that prints it.
const ZRBTDRT_CONSTANTS_MODULE: &str = "Tools/rbk/rbcc_constants.sh";
const ZRBTDRT_PROBE_READ: &str = r#"source "$1" && printf '%s\n' "${RBCC_probe_branch}""#;

/// Each rule's message, asserted beside the gate's code. Mirror: rblm_cli.sh,
/// `rblm_zero` and `rblm_feign` refusals — same literals.
const ZRBTDRT_ZERO_FRAGMENT: &str = "Marshal zero refuses: caller named tree";
const ZRBTDRT_BRANCH_FRAGMENT: &str = "Refusing to feign on branch";
const ZRBTDRT_REMOTE_FRAGMENT: &str = "Refusing to feign in a clone naming remote(s)";

/// git -C <tree> <args>, its failure an error naming the act.
fn zrbtdrt_git(tree: &Path, args: &[&str], act: &str) -> Result<String, String> {
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

/// The tabtarget in `tt` whose colophon is `colophon`, if one stands.
fn zrbtdrt_door(tt: &Path, colophon: &str) -> Result<Option<PathBuf>, String> {
    let head = format!(
        "{}.",
        colophon, //
    );
    let entries = std::fs::read_dir(tt).map_err(|e| {
        format!(
            "cannot list {}: {}",
            tt.display(),
            e, //
        )
    })?;
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with(&head) && name.ends_with(".sh") {
            return Ok(Some(entry.path()));
        }
    }
    Ok(None)
}

/// A scratch clone of the committed tree inside the case, its station beside
/// it, and its door for `colophon` — the tree's own, or one stood from the
/// shipped donor where the tree withholds it.
fn zrbtdrt_marshal_clone(
    dir: &Path,
    colophon: &str,
    frontispiece: &str,
) -> Result<(PathBuf, PathBuf), String> {
    let root = zrbtchx_root()?;
    let clone = dir.join(ZRBTDRT_CLONE);
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

    let regime_path = clone.join(RBTGEC_MOORINGS_DIR).join(ZRBTDRT_CONFIG_REGIME);
    let regime = std::fs::read_to_string(&regime_path).map_err(|e| {
        format!(
            "cannot read {}: {}",
            regime_path.display(),
            e, //
        )
    })?;
    let station_rel = regime
        .lines()
        .find_map(|line| line.strip_prefix(ZRBTDRT_STATION_KEY))
        .ok_or_else(|| {
            format!(
                "{} carries no {} line",
                regime_path.display(),
                ZRBTDRT_STATION_KEY, //
            )
        })?;
    let station = clone.join(station_rel.trim());
    if let Some(parent) = station.parent() {
        zrbtdrt_mkdir(parent)?;
    }
    zrbtdrt_mkdir(&dir.join(ZRBTDRT_STATION_LOGS))?;
    zrbtdrt_mkdir(&dir.join(ZRBTDRT_STATION_TACKROOM))?;
    zrbtdrt_write(
        &station,
        &format!(
            "BURS_USER={}\nBURS_TINCTURE={}\nBURS_LOG_DIR=../{}\nBURS_TACKROOM=../{}\n",
            ZRBTDRT_STATION_USER,
            ZRBTDRT_STATION_TINCTURE,
            ZRBTDRT_STATION_LOGS,
            ZRBTDRT_STATION_TACKROOM, //
        ),
    )?;

    let tt = clone.join(ZRBTDRT_TT_DIR);
    let door = match zrbtdrt_door(&tt, colophon)? {
        Some(door) => door,
        None => {
            let donor = zrbtdrt_door(&tt, RBTGEC_QUALIFY_FAST)?.ok_or_else(|| {
                format!(
                    "{} carries neither {} nor its donor {}",
                    tt.display(),
                    colophon,
                    RBTGEC_QUALIFY_FAST, //
                )
            })?;
            let door = tt.join(format!(
                "{}.{}.sh",
                colophon,
                frontispiece, //
            ));
            bkprs_program_copy(&donor, &door)?;
            door
        }
    };
    Ok((clone, door))
}

/// The probe branch's one name, read from the clone's own constants module by a
/// bash child.
fn zrbtdrt_probe_branch(clone: &Path) -> Result<String, String> {
    let module = rbtcep_native_to_posix(&clone.join(ZRBTDRT_CONSTANTS_MODULE));
    let output = Command::new(rbtcei_bash_program())
        .arg("-c")
        .arg(ZRBTDRT_PROBE_READ)
        .arg("bash")
        .arg(&module)
        .output()
        .map_err(|e| {
            format!(
                "cannot run the probe-branch read: {}",
                e, //
            )
        })?;
    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if !output.status.success() || branch.is_empty() {
        return Err(format!(
            "the probe branch could not be read from {} (exit {:?}): {}",
            module,
            output.status.code(),
            String::from_utf8_lossy(&output.stderr).trim(), //
        ));
    }
    Ok(branch)
}

/// Drive a marshal door in its clone under the skip confirmation, and assert the
/// marshal gate's code and the rule's message.
fn zrbtdrt_marshal_refuses(
    dir: &Path,
    clone: &Path,
    door: &Path,
    args: &[&str],
    label: &str,
    fragment: &str,
) -> Result<(), String> {
    let mut cmd = rbtdri_rooted_tabtarget_command(door, dir)?;
    let output = cmd
        .args(args)
        .env(RBTCEI_BURE_CONFIRM_KEY, RBTCEI_BURE_CONFIRM_SKIP)
        .current_dir(clone)
        .output()
        .map_err(|e| {
            format!(
                "{}: cannot run {}: {}",
                label,
                door.display(),
                e, //
            )
        })?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let code = output.status.code().unwrap_or(-1);
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
    if code != RBTGEB_MARSHAL_DENTIL {
        return Err(format!(
            "{}: exited {} — expected the marshal gate's {}\nstdout:\n{}\n\nstderr:\n{}",
            label,
            code,
            RBTGEB_MARSHAL_DENTIL,
            stdout,
            stderr, //
        ));
    }
    if !stdout.contains(fragment) && !stderr.contains(fragment) {
        return Err(format!(
            "{}: the gate's code without the rule's message '{}'\nstdout:\n{}\n\nstderr:\n{}",
            label,
            fragment,
            stdout,
            stderr, //
        ));
    }
    Ok(())
}

/// ZERO REFUSES A TREE THAT IS NOT ITS OWN: handed a basename that is not the
/// clone's, zero refuses under the marshal gate before any write.
fn rbtdrt_marshal_zero_wrong_tree(dir: &Path) -> rbtcee_Verdict {
    zrbtdrt_verdict((|| {
        let (clone, door) = zrbtdrt_marshal_clone(dir, RBTGEC_MARSHAL_ZERO, ZRBTDRT_FRONT_ZERO)?;
        zrbtdrt_marshal_refuses(
            dir,
            &clone,
            &door,
            &[ZRBTDRT_ZERO_FOLIO],
            "zero-wrong-tree",
            ZRBTDRT_ZERO_FRAGMENT,
        )
    })())
}

/// FEIGN REFUSES OFF THE PROBE BRANCH: in a clone naming no remote, on a branch
/// of the case's own naming, feign refuses under the marshal gate.
fn rbtdrt_marshal_feign_off_probe(dir: &Path) -> rbtcee_Verdict {
    zrbtdrt_verdict((|| {
        let (clone, door) = zrbtdrt_marshal_clone(dir, RBTGEC_MARSHAL_FEIGN, ZRBTDRT_FRONT_FEIGN)?;
        zrbtdrt_git(
            &clone,
            &["remote", "remove", ZRBTDRT_ORIGIN],
            "remove the clone's remote",
        )?;
        zrbtdrt_git(
            &clone,
            &["checkout", "-q", "-b", ZRBTDRT_OFF_PROBE_BRANCH],
            "cut the off-probe branch",
        )?;
        zrbtdrt_marshal_refuses(
            dir,
            &clone,
            &door,
            &[],
            "feign-off-probe",
            ZRBTDRT_BRANCH_FRAGMENT,
        )
    })())
}

/// FEIGN REFUSES A CLONE NAMING A REMOTE: on the probe branch, in a clone that
/// keeps its remote, feign refuses under the marshal gate.
fn rbtdrt_marshal_feign_remote(dir: &Path) -> rbtcee_Verdict {
    zrbtdrt_verdict((|| {
        let (clone, door) = zrbtdrt_marshal_clone(dir, RBTGEC_MARSHAL_FEIGN, ZRBTDRT_FRONT_FEIGN)?;
        let probe = zrbtdrt_probe_branch(&clone)?;
        zrbtdrt_git(
            &clone,
            &["checkout", "-q", "-B", &probe],
            "cut the probe branch",
        )?;
        zrbtdrt_marshal_refuses(
            dir,
            &clone,
            &door,
            &[],
            "feign-remote",
            ZRBTDRT_REMOTE_FRAGMENT,
        )
    })())
}

// ── Cases and fixture ────────────────────────────────────────

pub static RBTDRT_CASES_PUBLISH_PATH: &[rbtcee_Case] = &[
    rbtcee_case!(rbtdrt_lustration_sterile),
    rbtcee_case!(rbtdrt_lustration_export_plant),
    rbtcee_case!(rbtdrt_poll_forced_503),
    rbtcee_case!(rbtdrt_poll_ended_failure),
    rbtcee_case!(rbtdrt_marshal_zero_wrong_tree),
    rbtcee_case!(rbtdrt_marshal_feign_off_probe),
    rbtcee_case!(rbtdrt_marshal_feign_remote), //
];

/// credless: a reveille member. Its three tabtarget launches are the marshal
/// doors, each guarded; the poll's seam rides a direct-kindle child, never a
/// launch.
pub static RBTDRT_FIXTURE_PUBLISH_PATH: rbtcee_Fixture = rbtcee_Fixture {
    name: RBTDRT_NAME_PUBLISH_PATH,
    disposition: rbtcee_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRT_CASES_PUBLISH_PATH,
    credless: true,
    tariff: rbtcee_Tariff {
        min_secs: None,
        max_secs: Some(20),
        invocations: Some(3),
    },
};
