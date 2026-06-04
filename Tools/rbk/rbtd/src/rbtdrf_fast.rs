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
// RBTDRF — fast-tier test cases for theurge
//
// Ports enrollment-validation (47), regime-validation (21), and regime-smoke (7)
// from bash test framework to theurge. Cases shell out to bash — theurge invokes
// the actual bash utilities and asserts on exit codes. No reimplementation.

use std::path::Path;
use std::process::Command;

use crate::case;
use crate::rbtdre_engine::{rbtdre_Case, rbtdre_Disposition, rbtdre_Fixture, rbtdre_Verdict};
use crate::rbtdri_invocation::{rbtdri_find_tabtarget_global, rbtdri_tabtarget_command, rbtdri_bash_program};
use crate::rbtdgc_consts::{
    RBTDGC_HYGIENE_CHECK_DOCKERFILE,
    RBTDGC_HYGIENE_CHECK_VESSEL,
    RBTDGC_LIST_DEPOT,
    RBTDGC_RBRS_FILE,
    RBTDGC_RENDER_NAMEPLATE,
    RBTDGC_RENDER_PAYOR,
    RBTDGC_RENDER_REPO,
    RBTDGC_RENDER_STATION,
    RBTDGC_RENDER_VESSEL,
    RBTDGC_UNMAKE_DEPOT,
    RBTDGC_VALIDATE_NAMEPLATE,
    RBTDGC_VALIDATE_PAYOR,
    RBTDGC_VALIDATE_REPO,
    RBTDGC_VALIDATE_STATION,
    RBTDGC_VALIDATE_VESSEL,
};
use crate::rbtdrm_manifest::{
    RBTDRM_FIXTURE_DOCKERFILE_HYGIENE,
    RBTDRM_FIXTURE_ENROLLMENT_VALIDATION,
    RBTDRM_FIXTURE_FOUNDRY_PATH,
    RBTDRM_FIXTURE_REGIME_SMOKE,
    RBTDRM_FIXTURE_REGIME_VALIDATION,
    RBTDRM_MODULE_BURC,
    RBTDRM_MODULE_BURN,
    RBTDRM_MODULE_BURP,
    RBTDRM_MODULE_BURS,
    RBTDRM_MODULE_RBRA,
    RBTDRM_MODULE_RBRD,
    RBTDRM_MODULE_RBRN,
    RBTDRM_MODULE_RBRO,
    RBTDRM_MODULE_RBRP,
    RBTDRM_MODULE_RBRR,
    RBTDRM_MODULE_RBRS,
    RBTDRM_MODULE_RBRV,
    RBTDRM_PROBATE_BURC,
    RBTDRM_PROBATE_BURN,
    RBTDRM_PROBATE_BURP,
    RBTDRM_PROBATE_BURS,
    RBTDRM_PROBATE_RBRA,
    RBTDRM_PROBATE_RBRD,
    RBTDRM_PROBATE_RBRN,
    RBTDRM_PROBATE_RBRO,
    RBTDRM_PROBATE_RBRP,
    RBTDRM_PROBATE_RBRR,
    RBTDRM_PROBATE_RBRS,
    RBTDRM_PROBATE_RBRV,
};
use crate::rbtdrx_platform::rbtdrx_native_to_posix;

// ── Helpers ──────────────────────────────────────────────────

/// Sub-assertion within a case: run bash snippet, check exit code.
struct RbtdrfSub {
    label: &'static str,
    setup: &'static str,
    command: &'static str,
    expect_ok: bool,
}

impl RbtdrfSub {
    const fn ok(label: &'static str, setup: &'static str) -> Self {
        Self { label, setup, command: "buv_vet \"TEST\"", expect_ok: true }
    }
    const fn fatal(label: &'static str, setup: &'static str) -> Self {
        Self { label, setup, command: "buv_vet \"TEST\"", expect_ok: false }
    }
    const fn ok_cmd(label: &'static str, setup: &'static str, command: &'static str) -> Self {
        Self { label, setup, command, expect_ok: true }
    }
    const fn fatal_cmd(label: &'static str, setup: &'static str, command: &'static str) -> Self {
        Self { label, setup, command, expect_ok: false }
    }
}

/// Run a bash script, return (exit_code, stdout, stderr). Saves traces to case dir.
fn rbtdrf_run_bash(
    project_root: &Path,
    script: &str,
    dir: &Path,
    trace_prefix: &str,
) -> Result<(i32, String, String), String> {
    let output = Command::new(rbtdri_bash_program())
        .arg("-c")
        .arg(script)
        .current_dir(project_root)
        .output()
        .map_err(|e| format!("bash execution failed: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let code = output.status.code().unwrap_or(-1);

    let _ = std::fs::write(dir.join(format!("{}-script.sh", trace_prefix)), script);
    let _ = std::fs::write(dir.join(format!("{}-stdout.txt", trace_prefix)), &stdout);
    let _ = std::fs::write(dir.join(format!("{}-stderr.txt", trace_prefix)), &stderr);

    Ok((code, stdout, stderr))
}

/// Run enrollment-validation sub-assertions against BUV.
fn rbtdrf_run_ev(
    dir: &Path,
    enrollment: &str,
    subs: &[RbtdrfSub],
) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let buv = root.join("Tools/buk/buv_validation.sh");

    for (i, sub) in subs.iter().enumerate() {
        let script = format!(
            "set -euo pipefail\nsource '{}'\nzbuv_kindle\nzbuv_reset_enrollment\n{}\n{}\n{}",
            rbtdrx_native_to_posix(&buv),
            enrollment,
            sub.setup,
            sub.command,
        );

        match rbtdrf_run_bash(&root, &script, dir, &format!("sub-{}", i)) {
            Ok((code, _, _)) => {
                let ok = code == 0;
                if ok != sub.expect_ok {
                    return if sub.expect_ok {
                        rbtdre_Verdict::Fail(format!("{}: expected ok, got exit {}", sub.label, code))
                    } else {
                        rbtdre_Verdict::Fail(format!("{}: expected failure, got exit 0", sub.label))
                    };
                }
            }
            Err(e) => {
                return rbtdre_Verdict::Fail(format!("{}: {}", sub.label, e));
            }
        }
    }
    rbtdre_Verdict::Pass
}

/// Drive a regime's public `*_probate` function against a synthetic
/// baseline+override written to the case temp dir. `expect_ok` selects the
/// verdict polarity: pass-anchors validate the pristine baseline (true);
/// negatives apply one violating override and expect non-zero (false).
/// Sources only the contract surface — `buv_validation.sh` plus the one regime
/// module — mirroring the `buv_vet` bridge that enrollment-validation uses: no
/// internal-module chain, no z-private reach.
///
/// `tools_subdir` is the repo-relative directory holding the module
/// ("Tools/rbk" or "Tools/buk"). `prelude` is raw bash staged after buv's
/// kindle but before the regime module's probate — for regimes whose kindle or
/// enforce reaches a neighbor (rbrp → rbgc's payor regex; burs → bubc, whose
/// constants its enrollment help-string expands). Relative paths resolve
/// against cwd (the repo root, set by rbtdrf_run_bash). Empty for the common
/// case, mirroring how rbtdrf_rs_rbrr_nonempty_prefix stages rbgc in-harness.
fn rbtdrf_run_probate_in(
    dir: &Path,
    tools_subdir: &str,
    module: &str,
    probate_fn: &str,
    prelude: &str,
    baseline: &str,
    override_: &str,
    expect_ok: bool,
    label: &str,
) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let buv = root.join("Tools/buk/buv_validation.sh");
    let module_path = root.join(tools_subdir).join(module);

    let env_file = dir.join(format!("{}.env", label));
    let body = format!("{}\n{}\n", baseline, override_);
    if let Err(e) = std::fs::write(&env_file, &body) {
        return rbtdre_Verdict::Fail(format!("{}: write env failed: {}", label, e));
    }

    let script = format!(
        "set -euo pipefail\n\
         source '{}'\n\
         source '{}'\n\
         zbuv_kindle\n\
         {}\
         {} '{}'",
        rbtdrx_native_to_posix(&buv),
        rbtdrx_native_to_posix(&module_path),
        prelude,
        probate_fn,
        rbtdrx_native_to_posix(&env_file),
    );

    match rbtdrf_run_bash(&root, &script, dir, label) {
        Ok((code, _, _)) => {
            let ok = code == 0;
            if ok == expect_ok {
                rbtdre_Verdict::Pass
            } else if expect_ok {
                rbtdre_Verdict::Fail(format!("{}: expected ok, got exit {}", label, code))
            } else {
                rbtdre_Verdict::Fail(format!("{}: expected failure, got exit 0", label))
            }
        }
        Err(e) => rbtdre_Verdict::Fail(format!("{}: {}", label, e)),
    }
}

/// Common-case wrapper: RBK module, no prereqs. Preserves the call shape used
/// by the reference rv_ quartet (rbrr/rbrd/rbrv/rbrn).
fn rbtdrf_run_probate(
    dir: &Path,
    module: &str,
    probate_fn: &str,
    baseline: &str,
    override_: &str,
    expect_ok: bool,
    label: &str,
) -> rbtdre_Verdict {
    rbtdrf_run_probate_in(
        dir, "Tools/rbk", module, probate_fn, "", baseline, override_, expect_ok, label,
    )
}

/// Run a tabtarget and check exit 0.
fn rbtdrf_run_tt(
    project_root: &Path,
    colophon: &str,
    args: &[&str],
    dir: &Path,
    label: &str,
) -> Result<(), String> {
    let tt = rbtdri_find_tabtarget_global(project_root, colophon)?;
    let output = rbtdri_tabtarget_command(&tt)
        .args(args)
        .current_dir(project_root)
        .output()
        .map_err(|e| format!("{}: failed to run {}: {}", label, tt.display(), e))?;

    let code = output.status.code().unwrap_or(-1);
    let _ = std::fs::write(dir.join(format!("{}-stdout.txt", label)), &output.stdout);
    let _ = std::fs::write(dir.join(format!("{}-stderr.txt", label)), &output.stderr);

    if code != 0 {
        return Err(format!(
            "{}: {} exited {} — {}",
            label,
            colophon,
            code,
            String::from_utf8_lossy(&output.stderr),
        ));
    }
    Ok(())
}

/// Run a tabtarget that is expected to fail. Inverts rbtdrf_run_tt:
/// non-zero exit returns Ok(()), zero exit returns Err. Invocation errors
/// (tabtarget not found, launcher failure) still propagate as Err.
fn rbtdrf_run_tt_neg(
    project_root: &Path,
    colophon: &str,
    args: &[&str],
    dir: &Path,
    label: &str,
) -> Result<(), String> {
    let tt = rbtdri_find_tabtarget_global(project_root, colophon)?;
    let output = rbtdri_tabtarget_command(&tt)
        .args(args)
        .current_dir(project_root)
        .output()
        .map_err(|e| format!("{}: failed to run {}: {}", label, tt.display(), e))?;

    let code = output.status.code().unwrap_or(-1);
    let _ = std::fs::write(dir.join(format!("{}-stdout.txt", label)), &output.stdout);
    let _ = std::fs::write(dir.join(format!("{}-stderr.txt", label)), &output.stderr);

    if code == 0 {
        return Err(format!(
            "{}: {} expected failure, got exit 0",
            label, colophon,
        ));
    }
    Ok(())
}

// ── Enrollment-validation cases ─────────────────────────────

// --- Length types ---

fn rbtdrf_ev_string_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Strings\"\n\
         buv_string_enroll TEST_NAME 1 20 \"Test name\"\n\
         buv_string_enroll TEST_DESC 3 50 \"Test description\"",
        &[RbtdrfSub::ok("valid strings",
            "export TEST_NAME=\"hello\"\nexport TEST_DESC=\"a valid description\"")],
    )
}

fn rbtdrf_ev_string_empty_optional(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Strings\"\n\
         buv_string_enroll TEST_OPT 0 20 \"Optional field\"",
        &[RbtdrfSub::ok("empty optional", "export TEST_OPT=\"\"")],
    )
}

fn rbtdrf_ev_string_too_short(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Strings\"\n\
         buv_string_enroll TEST_NAME 5 20 \"Test name\"",
        &[RbtdrfSub::fatal("too short", "export TEST_NAME=\"ab\"")],
    )
}

fn rbtdrf_ev_string_too_long(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Strings\"\n\
         buv_string_enroll TEST_NAME 1 5 \"Test name\"",
        &[RbtdrfSub::fatal("too long", "export TEST_NAME=\"toolongvalue\"")],
    )
}

fn rbtdrf_ev_string_empty_required(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Strings\"\n\
         buv_string_enroll TEST_NAME 1 20 \"Test name\"",
        &[RbtdrfSub::fatal("empty required", "export TEST_NAME=\"\"")],
    )
}

fn rbtdrf_ev_xname_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Xnames\"\n\
         buv_xname_enroll TEST_IDENT 2 12 \"Identifier\"",
        &[
            RbtdrfSub::ok("standard xname", "export TEST_IDENT=\"myName\""),
            RbtdrfSub::ok("underscore and hyphen", "export TEST_IDENT=\"my_var-1\""),
        ],
    )
}

fn rbtdrf_ev_xname_invalid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Xnames\"\n\
         buv_xname_enroll TEST_IDENT 2 12 \"Identifier\"",
        &[
            RbtdrfSub::fatal("starts with digit", "export TEST_IDENT=\"1bad\""),
            RbtdrfSub::fatal("contains dot", "export TEST_IDENT=\"my.name\""),
            RbtdrfSub::fatal("too short", "export TEST_IDENT=\"x\""),
            RbtdrfSub::fatal("too long", "export TEST_IDENT=\"abcdefghijklm\""),
        ],
    )
}

fn rbtdrf_ev_gname_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Gnames\"\n\
         buv_gname_enroll TEST_PROJECT 3 20 \"Project ID\"",
        &[RbtdrfSub::ok("valid gname", "export TEST_PROJECT=\"my-project-01\"")],
    )
}

fn rbtdrf_ev_gname_invalid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Gnames\"\n\
         buv_gname_enroll TEST_PROJECT 3 20 \"Project ID\"",
        &[
            RbtdrfSub::fatal("uppercase", "export TEST_PROJECT=\"MyProject\""),
            RbtdrfSub::fatal("ends with hyphen", "export TEST_PROJECT=\"my-project-\""),
            RbtdrfSub::fatal("starts with digit", "export TEST_PROJECT=\"1project\""),
        ],
    )
}

fn rbtdrf_ev_fqin_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"FQINs\"\n\
         buv_fqin_enroll TEST_IMAGE 5 100 \"Image reference\"",
        &[RbtdrfSub::ok("valid fqin",
            "export TEST_IMAGE=\"us-central1-docker.pkg.dev/my-proj/repo/image:latest\"")],
    )
}

fn rbtdrf_ev_fqin_invalid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"FQINs\"\n\
         buv_fqin_enroll TEST_IMAGE 5 100 \"Image reference\"",
        &[
            RbtdrfSub::fatal("special char", "export TEST_IMAGE=\".invalid/path\""),
            RbtdrfSub::fatal("empty", "export TEST_IMAGE=\"\""),
        ],
    )
}

// --- Choice types ---

fn rbtdrf_ev_bool_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Booleans\"\n\
         buv_bool_enroll TEST_ENABLED \"Feature enabled\"",
        &[
            RbtdrfSub::ok("value 1", "export TEST_ENABLED=\"1\""),
            RbtdrfSub::ok("value 0", "export TEST_ENABLED=\"0\""),
        ],
    )
}

fn rbtdrf_ev_bool_invalid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Booleans\"\n\
         buv_bool_enroll TEST_ENABLED \"Feature enabled\"",
        &[
            RbtdrfSub::fatal("string true", "export TEST_ENABLED=\"true\""),
            RbtdrfSub::fatal("string yes", "export TEST_ENABLED=\"yes\""),
            RbtdrfSub::fatal("number 2", "export TEST_ENABLED=\"2\""),
        ],
    )
}

fn rbtdrf_ev_bool_empty(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Booleans\"\n\
         buv_bool_enroll TEST_ENABLED \"Feature enabled\"",
        &[RbtdrfSub::fatal("empty", "export TEST_ENABLED=\"\"")],
    )
}

fn rbtdrf_ev_enum_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Enums\"\n\
         buv_enum_enroll TEST_MODE \"Operating mode\" debug release test",
        &[
            RbtdrfSub::ok("first choice", "export TEST_MODE=\"debug\""),
            RbtdrfSub::ok("last choice", "export TEST_MODE=\"test\""),
        ],
    )
}

fn rbtdrf_ev_enum_invalid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Enums\"\n\
         buv_enum_enroll TEST_MODE \"Operating mode\" debug release test",
        &[
            RbtdrfSub::fatal("not a choice", "export TEST_MODE=\"production\""),
            RbtdrfSub::fatal("case mismatch", "export TEST_MODE=\"Debug\""),
        ],
    )
}

fn rbtdrf_ev_enum_empty(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Enums\"\n\
         buv_enum_enroll TEST_MODE \"Operating mode\" debug release test",
        &[RbtdrfSub::fatal("empty", "export TEST_MODE=\"\"")],
    )
}

// --- Numeric types ---

fn rbtdrf_ev_decimal_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Numerics\"\n\
         buv_decimal_enroll TEST_COUNT 1 100 \"Item count\"",
        &[
            RbtdrfSub::ok("at minimum", "export TEST_COUNT=\"1\""),
            RbtdrfSub::ok("at maximum", "export TEST_COUNT=\"100\""),
            RbtdrfSub::ok("mid-range", "export TEST_COUNT=\"50\""),
        ],
    )
}

fn rbtdrf_ev_decimal_below(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Numerics\"\n\
         buv_decimal_enroll TEST_COUNT 1 100 \"Item count\"",
        &[RbtdrfSub::fatal("below minimum", "export TEST_COUNT=\"0\"")],
    )
}

fn rbtdrf_ev_decimal_above(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Numerics\"\n\
         buv_decimal_enroll TEST_COUNT 1 100 \"Item count\"",
        &[RbtdrfSub::fatal("above maximum", "export TEST_COUNT=\"101\"")],
    )
}

fn rbtdrf_ev_decimal_empty(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Numerics\"\n\
         buv_decimal_enroll TEST_COUNT 1 100 \"Item count\"",
        &[RbtdrfSub::fatal("empty", "export TEST_COUNT=\"\"")],
    )
}

fn rbtdrf_ev_ipv4_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Network\"\n\
         buv_ipv4_enroll TEST_ADDR \"Server address\"",
        &[RbtdrfSub::ok("valid address", "export TEST_ADDR=\"192.168.1.1\"")],
    )
}

fn rbtdrf_ev_ipv4_invalid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Network\"\n\
         buv_ipv4_enroll TEST_ADDR \"Server address\"",
        &[
            RbtdrfSub::fatal("not dotted-quad", "export TEST_ADDR=\"not-an-ip\""),
            RbtdrfSub::fatal("empty", "export TEST_ADDR=\"\""),
        ],
    )
}

fn rbtdrf_ev_port_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Network\"\n\
         buv_port_enroll TEST_PORT \"Service port\"",
        &[
            RbtdrfSub::ok("common port", "export TEST_PORT=\"8080\""),
            RbtdrfSub::ok("minimum port", "export TEST_PORT=\"1\""),
            RbtdrfSub::ok("maximum port", "export TEST_PORT=\"65535\""),
        ],
    )
}

fn rbtdrf_ev_port_invalid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Network\"\n\
         buv_port_enroll TEST_PORT \"Service port\"",
        &[
            RbtdrfSub::fatal("zero", "export TEST_PORT=\"0\""),
            RbtdrfSub::fatal("above max", "export TEST_PORT=\"65536\""),
            RbtdrfSub::fatal("empty", "export TEST_PORT=\"\""),
        ],
    )
}

// --- Reference types ---

const RBTDRF_VALID_DIGEST: &str =
    "sha256:abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";

fn rbtdrf_ev_odref_valid(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let buv = root.join("Tools/buk/buv_validation.sh");
    let enrollment = format!(
        "set -euo pipefail\nsource '{}'\nzbuv_kindle\nzbuv_reset_enrollment\n\
         buv_regime_enroll \"TEST\"\nbuv_group_enroll \"References\"\n\
         buv_odref_enroll TEST_IMAGE \"Container image\"",
        rbtdrx_native_to_posix(&buv),
    );
    let d = RBTDRF_VALID_DIGEST;

    let subs: &[(&str, &str)] = &[
        ("standard registry", &format!("docker.io/library/alpine@{}", d)),
        ("multi-level repo", &format!("us-central1-docker.pkg.dev/my-proj/my-repo/tool@{}", d)),
        ("registry with port", &format!("registry.local:5000/myimage@{}", d)),
    ];

    for (i, (label, image)) in subs.iter().enumerate() {
        let script = format!("{}\nexport TEST_IMAGE=\"{}\"\nbuv_vet \"TEST\"", enrollment, image);
        match rbtdrf_run_bash(&root, &script, dir, &format!("sub-{}", i)) {
            Ok((0, _, _)) => {}
            Ok((code, _, _)) => {
                return rbtdre_Verdict::Fail(format!("{}: expected ok, got exit {}", label, code));
            }
            Err(e) => return rbtdre_Verdict::Fail(format!("{}: {}", label, e)),
        }
    }
    rbtdre_Verdict::Pass
}

fn rbtdrf_ev_odref_no_digest(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"References\"\n\
         buv_odref_enroll TEST_IMAGE \"Container image\"",
        &[RbtdrfSub::fatal("tag only",
            "export TEST_IMAGE=\"docker.io/library/alpine:latest\"")],
    )
}

fn rbtdrf_ev_odref_malformed(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"References\"\n\
         buv_odref_enroll TEST_IMAGE \"Container image\"",
        &[
            RbtdrfSub::fatal("wrong algorithm",
                "export TEST_IMAGE=\"docker.io/library/alpine@md5:abcdef0123456789\""),
            RbtdrfSub::fatal("short hex",
                "export TEST_IMAGE=\"docker.io/library/alpine@sha256:abcdef\""),
            RbtdrfSub::fatal("uppercase hex",
                "export TEST_IMAGE=\"docker.io/library/alpine@sha256:ABCDEF0123456789abcdef0123456789abcdef0123456789abcdef0123456789\""),
        ],
    )
}

fn rbtdrf_ev_odref_empty(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"References\"\n\
         buv_odref_enroll TEST_IMAGE \"Container image\"",
        &[RbtdrfSub::fatal("empty", "export TEST_IMAGE=\"\"")],
    )
}

// --- List types ---

fn rbtdrf_ev_list_string_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Lists\"\n\
         buv_list_string_enroll TEST_TAGS 2 10 \"Tags\"",
        &[RbtdrfSub::ok("valid items", "export TEST_TAGS=\"foo bar baz\"")],
    )
}

fn rbtdrf_ev_list_string_empty(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Lists\"\n\
         buv_list_string_enroll TEST_TAGS 2 10 \"Tags\"",
        &[RbtdrfSub::ok("empty list", "export TEST_TAGS=\"\"")],
    )
}

fn rbtdrf_ev_list_string_bad_item(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Lists\"\n\
         buv_list_string_enroll TEST_TAGS 3 10 \"Tags\"",
        &[
            RbtdrfSub::fatal("item too short", "export TEST_TAGS=\"good ab okay\""),
            RbtdrfSub::fatal("item too long", "export TEST_TAGS=\"good toolongvalue okay\""),
        ],
    )
}

fn rbtdrf_ev_list_ipv4_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Lists\"\n\
         buv_list_ipv4_enroll TEST_SERVERS \"Server addresses\"",
        &[RbtdrfSub::ok("valid addresses",
            "export TEST_SERVERS=\"192.168.1.1 10.0.0.1 172.16.0.1\"")],
    )
}

fn rbtdrf_ev_list_ipv4_invalid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Lists\"\n\
         buv_list_ipv4_enroll TEST_SERVERS \"Server addresses\"",
        &[RbtdrfSub::fatal("bad address",
            "export TEST_SERVERS=\"192.168.1.1 not-an-ip 10.0.0.1\"")],
    )
}

fn rbtdrf_ev_list_ipv4_empty(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Lists\"\n\
         buv_list_ipv4_enroll TEST_SERVERS \"Server addresses\"",
        &[RbtdrfSub::ok("empty list", "export TEST_SERVERS=\"\"")],
    )
}

fn rbtdrf_ev_list_gname_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Lists\"\n\
         buv_list_gname_enroll TEST_PROJECTS 3 20 \"Project IDs\"",
        &[RbtdrfSub::ok("valid names",
            "export TEST_PROJECTS=\"my-project other-proj test-01\"")],
    )
}

fn rbtdrf_ev_list_gname_invalid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Lists\"\n\
         buv_list_gname_enroll TEST_PROJECTS 3 20 \"Project IDs\"",
        &[RbtdrfSub::fatal("uppercase in item",
            "export TEST_PROJECTS=\"my-project BadName test-01\"")],
    )
}

// --- Gating ---

fn rbtdrf_ev_gate_active_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Gated Features\"\n\
         buv_enum_enroll TEST_MODE \"Feature mode\" enabled disabled\n\
         buv_gate_enroll TEST_MODE enabled\n\
         buv_port_enroll TEST_PORT \"Feature port\"",
        &[RbtdrfSub::ok("gate active valid",
            "export TEST_MODE=\"enabled\"\nexport TEST_PORT=\"8080\"")],
    )
}

fn rbtdrf_ev_gate_active_invalid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Gated Features\"\n\
         buv_enum_enroll TEST_MODE \"Feature mode\" enabled disabled\n\
         buv_gate_enroll TEST_MODE enabled\n\
         buv_port_enroll TEST_PORT \"Feature port\"",
        &[RbtdrfSub::fatal("gate active invalid",
            "export TEST_MODE=\"enabled\"\nexport TEST_PORT=\"0\"")],
    )
}

fn rbtdrf_ev_gate_inactive(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Gated Features\"\n\
         buv_enum_enroll TEST_MODE \"Feature mode\" enabled disabled\n\
         buv_gate_enroll TEST_MODE enabled\n\
         buv_port_enroll TEST_PORT \"Feature port\"",
        &[RbtdrfSub::ok("gate inactive skips",
            "export TEST_MODE=\"disabled\"\nexport TEST_PORT=\"invalid-not-checked\"")],
    )
}

fn rbtdrf_ev_gate_multi(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\n\
         buv_group_enroll \"Core\"\n\
         buv_xname_enroll TEST_NAME 2 12 \"Service name\"\n\
         buv_group_enroll \"Feature A\"\n\
         buv_enum_enroll TEST_FEAT_A \"Feature A mode\" on off\n\
         buv_gate_enroll TEST_FEAT_A on\n\
         buv_port_enroll TEST_FEAT_A_PORT \"Feature A port\"\n\
         buv_group_enroll \"Feature B\"\n\
         buv_enum_enroll TEST_FEAT_B \"Feature B mode\" on off\n\
         buv_gate_enroll TEST_FEAT_B on\n\
         buv_string_enroll TEST_FEAT_B_LABEL 1 20 \"Feature B label\"",
        &[
            RbtdrfSub::ok("A on, B off",
                "export TEST_NAME=\"myservice\"\n\
                 export TEST_FEAT_A=\"on\"\nexport TEST_FEAT_A_PORT=\"9090\"\n\
                 export TEST_FEAT_B=\"off\"\nexport TEST_FEAT_B_LABEL=\"\""),
            RbtdrfSub::ok("both on",
                "export TEST_NAME=\"myservice\"\n\
                 export TEST_FEAT_A=\"on\"\nexport TEST_FEAT_A_PORT=\"9090\"\n\
                 export TEST_FEAT_B=\"on\"\nexport TEST_FEAT_B_LABEL=\"hello\""),
        ],
    )
}

// --- Enforce/Report ---

fn rbtdrf_ev_enforce_all_pass(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Core\"\n\
         buv_xname_enroll TEST_NAME 2 12 \"Name\"\n\
         buv_bool_enroll TEST_FLAG \"Flag\"\n\
         buv_decimal_enroll TEST_COUNT 1 10 \"Count\"",
        &[RbtdrfSub::ok("all pass",
            "export TEST_NAME=\"myname\"\nexport TEST_FLAG=\"1\"\nexport TEST_COUNT=\"5\"")],
    )
}

fn rbtdrf_ev_enforce_first_bad(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Core\"\n\
         buv_xname_enroll TEST_NAME 2 12 \"Name\"\n\
         buv_bool_enroll TEST_FLAG \"Flag\"\n\
         buv_decimal_enroll TEST_COUNT 1 10 \"Count\"",
        &[
            RbtdrfSub::fatal("first var invalid",
                "export TEST_NAME=\"1\"\nexport TEST_FLAG=\"1\"\nexport TEST_COUNT=\"5\""),
            RbtdrfSub::fatal("middle var invalid",
                "export TEST_NAME=\"myname\"\nexport TEST_FLAG=\"maybe\"\nexport TEST_COUNT=\"5\""),
            RbtdrfSub::fatal("last var invalid",
                "export TEST_NAME=\"myname\"\nexport TEST_FLAG=\"1\"\nexport TEST_COUNT=\"99\""),
        ],
    )
}

fn rbtdrf_ev_report_all_pass(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Core\"\n\
         buv_xname_enroll TEST_NAME 2 12 \"Name\"\n\
         buv_bool_enroll TEST_FLAG \"Flag\"",
        &[RbtdrfSub::ok_cmd("all pass report",
            "export TEST_NAME=\"myname\"\nexport TEST_FLAG=\"0\"",
            "buv_report \"TEST\" \"All-pass report\"")],
    )
}

fn rbtdrf_ev_report_mixed(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Core\"\n\
         buv_xname_enroll TEST_NAME 2 12 \"Name\"\n\
         buv_bool_enroll TEST_FLAG \"Flag\"",
        &[RbtdrfSub::fatal_cmd("mixed report",
            "export TEST_NAME=\"myname\"\nexport TEST_FLAG=\"bad\"",
            "buv_report \"TEST\" \"Mixed report\"")],
    )
}

fn rbtdrf_ev_report_gated(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_ev(dir,
        "buv_regime_enroll \"TEST\"\nbuv_group_enroll \"Gated\"\n\
         buv_enum_enroll TEST_MODE \"Mode\" on off\n\
         buv_gate_enroll TEST_MODE on\n\
         buv_port_enroll TEST_PORT \"Port\"",
        &[RbtdrfSub::ok_cmd("gated report passes",
            "export TEST_MODE=\"off\"\nexport TEST_PORT=\"\"",
            "buv_report \"TEST\" \"Gated report\"")],
    )
}

fn rbtdrf_ev_multiscope(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let buv = root.join("Tools/buk/buv_validation.sh");

    // Two-scope enrollment: ALPHA and BETA
    let enrollment = format!(
        "set -euo pipefail\nsource '{}'\nzbuv_kindle\nzbuv_reset_enrollment\n\
         buv_regime_enroll \"ALPHA\"\nbuv_group_enroll \"Alpha Vars\"\n\
         buv_bool_enroll TEST_ALPHA_FLAG \"Alpha flag\"\n\
         buv_regime_enroll \"BETA\"\nbuv_group_enroll \"Beta Vars\"\n\
         buv_bool_enroll TEST_BETA_FLAG \"Beta flag\"\n\
         export TEST_ALPHA_FLAG=\"1\"\nexport TEST_BETA_FLAG=\"bad\"\n",
        rbtdrx_native_to_posix(&buv),
    );

    // Sub 1: vet ALPHA passes (BETA is bad but not in scope)
    let script1 = format!("{}buv_vet \"ALPHA\"", enrollment);
    match rbtdrf_run_bash(&root, &script1, dir, "sub-0") {
        Ok((code, _, _)) if code == 0 => {}
        Ok((code, _, _)) => {
            return rbtdre_Verdict::Fail(format!("alpha scope: expected ok, got exit {}", code));
        }
        Err(e) => return rbtdre_Verdict::Fail(format!("alpha scope: {}", e)),
    }

    // Sub 2: vet BETA fails
    let script2 = format!("{}buv_vet \"BETA\"", enrollment);
    match rbtdrf_run_bash(&root, &script2, dir, "sub-1") {
        Ok((code, _, _)) if code != 0 => {}
        Ok((_, _, _)) => {
            return rbtdre_Verdict::Fail("beta scope: expected failure, got exit 0".to_string());
        }
        Err(e) => return rbtdre_Verdict::Fail(format!("beta scope: {}", e)),
    }

    rbtdre_Verdict::Pass
}

// ── Regime-validation cases ─────────────────────────────────

// Env-var name consts — single source of truth for prefix-related env vars
// referenced across rv_rbrr negatives and rs_rbrr_nonempty_prefix smoke.
const RBTDRF_VAR_RBRD_CLOUD_PREFIX: &str = "RBRD_CLOUD_PREFIX";
const RBTDRF_VAR_RBRR_RUNTIME_PREFIX: &str = "RBRR_RUNTIME_PREFIX";

// Expected RBGL_HALLMARKS_ROOT value — must match RBGC_GAR_CATEGORY_HALLMARKS.
const RBTDRF_VAL_HALLMARKS_ROOT: &str = "rbi_hm";

// --- RBRR negative tests ---

// Valid synthetic RBRR baseline — every enrolled field passes kindle+enforce.
// VESSEL_DIR/SECRETS_DIR point at /tmp (existence-checked by buv_dir_exists;
// the bad-dir cases below override to a nonexistent path). Each negative
// applies exactly one violating override.
const RBTDRF_RBRR_BASELINE: &str = "\
export RBRR_RUNTIME_PREFIX=\"rb-\"\n\
export RBRR_VESSEL_DIR=\"/tmp\"\n\
export RBRR_BOTTLE_WORKSPACE=\"/workspace\"\n\
export RBRR_DNS_SERVER=\"8.8.8.8\"\n\
export RBRR_GCB_TIMEOUT=\"1200s\"\n\
export RBRR_GCB_MIN_CONCURRENT_BUILDS=\"1\"\n\
export RBRR_SECRETS_DIR=\"/tmp\"\n\
export RBRR_PUBLIC_DOCS_URL=\"https://example.com/docs\"";

fn rbtdrf_rv_rbrr_neg(dir: &Path, label: &str, override_: &str) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRR, RBTDRM_PROBATE_RBRR,
        RBTDRF_RBRR_BASELINE, override_, false, label)
}

fn rbtdrf_rv_rbrr_baseline_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRR, RBTDRM_PROBATE_RBRR,
        RBTDRF_RBRR_BASELINE, "", true, "rbrr-baseline-valid")
}

fn rbtdrf_rv_rbrr_bad_timeout(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrr_neg(dir, "rbrr-bad-timeout", "export RBRR_GCB_TIMEOUT=\"1200\"")
}

fn rbtdrf_rv_rbrr_unexpected_var(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrr_neg(dir, "rbrr-unexpected-var", "export RBRR_BOGUS=\"foo\"")
}

fn rbtdrf_rv_rbrr_bad_vessel_dir(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrr_neg(dir, "rbrr-bad-vessel-dir",
        "export RBRR_VESSEL_DIR=\"/tmp/nonexistent-rbtdrf-vessel-dir\"")
}

fn rbtdrf_rv_rbrr_bad_secrets_dir(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrr_neg(dir, "rbrr-bad-secrets-dir",
        "export RBRR_SECRETS_DIR=\"/tmp/nonexistent-rbtdrf-secrets-dir\"")
}

// Runtime-prefix format rule: ^[a-z][a-z0-9-]*-$ length 2..=11 (rbrr_regime.sh).
// Three independent failure modes — one case each.

fn rbtdrf_rv_rbrr_bad_runtime_prefix_uppercase(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrr_neg(dir, "rbrr-bad-runtime-prefix-uppercase",
        &format!("export {}=\"BAD-\"", RBTDRF_VAR_RBRR_RUNTIME_PREFIX))
}

fn rbtdrf_rv_rbrr_bad_runtime_prefix_no_trailing_hyphen(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrr_neg(dir, "rbrr-bad-runtime-prefix-no-trailing-hyphen",
        &format!("export {}=\"acme\"", RBTDRF_VAR_RBRR_RUNTIME_PREFIX))
}

fn rbtdrf_rv_rbrr_bad_runtime_prefix_too_long(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrr_neg(dir, "rbrr-bad-runtime-prefix-too-long",
        &format!("export {}=\"twelvechars-\"", RBTDRF_VAR_RBRR_RUNTIME_PREFIX))
}

// --- RBRD negative tests ---

// Valid synthetic RBRD baseline — every enrolled field passes kindle+enforce
// (joint prefix+moniker length 16 <= 30). Recategorized here from the former
// rbrr-named block: these exercise RBRD_DEPOT_MONIKER and RBRD_CLOUD_PREFIX,
// independent of RBRR after the RBRD-from-RBRR split.
const RBTDRF_RBRD_BASELINE: &str = "\
export RBRD_CLOUD_PREFIX=\"acme-\"\n\
export RBRD_DEPOT_MONIKER=\"testdepot\"\n\
export RBRD_GCP_REGION=\"us-central1\"\n\
export RBRD_GCB_MACHINE_TYPE=\"e2-standard-2\"";

fn rbtdrf_rv_rbrd_neg(dir: &Path, label: &str, override_: &str) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRD, RBTDRM_PROBATE_RBRD,
        RBTDRF_RBRD_BASELINE, override_, false, label)
}

fn rbtdrf_rv_rbrd_baseline_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRD, RBTDRM_PROBATE_RBRD,
        RBTDRF_RBRD_BASELINE, "", true, "rbrd-baseline-valid")
}

fn rbtdrf_rv_rbrd_missing_moniker(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrd_neg(dir, "rbrd-missing-moniker", "unset RBRD_DEPOT_MONIKER")
}

fn rbtdrf_rv_rbrd_bad_moniker(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrd_neg(dir, "rbrd-bad-moniker", "export RBRD_DEPOT_MONIKER=\"BAD-MONIKER\"")
}

// Cloud-prefix format rule: ^[a-z][a-z0-9-]*-$ length 2..=11 (rbrd_regime.sh).
// Three independent failure modes — one case each.

fn rbtdrf_rv_rbrd_bad_cloud_prefix_uppercase(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrd_neg(dir, "rbrd-bad-cloud-prefix-uppercase",
        &format!("export {}=\"BAD-\"", RBTDRF_VAR_RBRD_CLOUD_PREFIX))
}

fn rbtdrf_rv_rbrd_bad_cloud_prefix_no_trailing_hyphen(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrd_neg(dir, "rbrd-bad-cloud-prefix-no-trailing-hyphen",
        &format!("export {}=\"acme\"", RBTDRF_VAR_RBRD_CLOUD_PREFIX))
}

fn rbtdrf_rv_rbrd_bad_cloud_prefix_too_long(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrd_neg(dir, "rbrd-bad-cloud-prefix-too-long",
        &format!("export {}=\"twelvechars-\"", RBTDRF_VAR_RBRD_CLOUD_PREFIX))
}

// --- RBRV negative tests ---

const RBTDRF_RBRV_BASELINE_CONJURE: &str = "\
export RBRV_SIGIL=\"test-vessel\"\n\
export RBRV_DESCRIPTION=\"Test vessel for validation\"\n\
export RBRV_VESSEL_MODE=\"rbnve_conjure\"\n\
export RBRV_RELIQUARY=\"r260101000000\"\n\
export RBRV_EGRESS_MODE=\"rbnve_tether\"\n\
export RBRV_CONJURE_DOCKERFILE=\"path/to/Dockerfile\"\n\
export RBRV_CONJURE_BLDCONTEXT=\"path/to\"\n\
export RBRV_CONJURE_PLATFORMS=\"linux/amd64\"";

const RBTDRF_RBRV_BASELINE_BIND: &str = "\
export RBRV_SIGIL=\"test-vessel\"\n\
export RBRV_DESCRIPTION=\"Test vessel for validation\"\n\
export RBRV_VESSEL_MODE=\"rbnve_bind\"\n\
export RBRV_RELIQUARY=\"r260101000000\"\n\
export RBRV_EGRESS_MODE=\"rbnve_tether\"\n\
export RBRV_BIND_IMAGE=\"us-docker.pkg.dev/project/repo/image:latest\"";

fn rbtdrf_rv_rbrv_neg(dir: &Path, label: &str, baseline: &str, override_: &str) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRV, RBTDRM_PROBATE_RBRV, baseline, override_, false, label)
}

fn rbtdrf_rv_rbrv_conjure_baseline_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRV, RBTDRM_PROBATE_RBRV,
        RBTDRF_RBRV_BASELINE_CONJURE, "", true, "rbrv-conjure-baseline-valid")
}

fn rbtdrf_rv_rbrv_bind_baseline_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRV, RBTDRM_PROBATE_RBRV,
        RBTDRF_RBRV_BASELINE_BIND, "", true, "rbrv-bind-baseline-valid")
}

fn rbtdrf_rv_rbrv_missing_sigil(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrv_neg(dir, "rbrv-missing-sigil",
        RBTDRF_RBRV_BASELINE_CONJURE, "unset RBRV_SIGIL")
}

fn rbtdrf_rv_rbrv_no_bind_image(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrv_neg(dir, "rbrv-no-bind-image",
        RBTDRF_RBRV_BASELINE_BIND, "unset RBRV_BIND_IMAGE")
}

fn rbtdrf_rv_rbrv_unexpected_var(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrv_neg(dir, "rbrv-unexpected-var",
        RBTDRF_RBRV_BASELINE_CONJURE, "export RBRV_BOGUS=\"foo\"")
}

fn rbtdrf_rv_rbrv_partial_conjure(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrv_neg(dir, "rbrv-partial-conjure",
        RBTDRF_RBRV_BASELINE_CONJURE, "unset RBRV_CONJURE_PLATFORMS")
}

// --- RBRN negative tests ---

const RBTDRF_RBRN_BASELINE_DISABLED: &str = "\
export RBRN_MONIKER=\"testrv\"\n\
export RBRN_DESCRIPTION=\"Test nameplate\"\n\
export RBRN_RUNTIME=\"docker\"\n\
export RBRN_SENTRY_VESSEL=\"test-sentry\"\n\
export RBRN_BOTTLE_VESSEL=\"test-bottle\"\n\
export RBRN_SENTRY_HALLMARK=\"c260101000000-r260101000000\"\n\
export RBRN_BOTTLE_HALLMARK=\"c260101000000-r260101000000\"\n\
export RBRN_BOTTLE_READINESS_DELAY_SEC=\"0\"\n\
export RBRN_ENTRY_MODE=\"rbnne_disabled\"\n\
export RBRN_ENCLAVE_BASE_IP=\"10.200.0.0\"\n\
export RBRN_ENCLAVE_NETMASK=\"24\"\n\
export RBRN_ENCLAVE_SENTRY_IP=\"10.200.0.2\"\n\
export RBRN_ENCLAVE_BOTTLE_IP=\"10.200.0.3\"\n\
export RBRN_UPLINK_PORT_MIN=\"10000\"\n\
export RBRN_UPLINK_DNS_MODE=\"rbnne_disabled\"\n\
export RBRN_UPLINK_ACCESS_MODE=\"rbnne_disabled\"";

fn rbtdrf_rv_rbrn_neg(dir: &Path, label: &str, override_: &str) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRN, RBTDRM_PROBATE_RBRN,
        RBTDRF_RBRN_BASELINE_DISABLED, override_, false, label)
}

fn rbtdrf_rv_rbrn_baseline_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRN, RBTDRM_PROBATE_RBRN,
        RBTDRF_RBRN_BASELINE_DISABLED, "", true, "rbrn-baseline-valid")
}

fn rbtdrf_rv_rbrn_missing_moniker(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrn_neg(dir, "rbrn-missing-moniker", "unset RBRN_MONIKER")
}

fn rbtdrf_rv_rbrn_invalid_runtime(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrn_neg(dir, "rbrn-invalid-runtime", "export RBRN_RUNTIME=\"invalid\"")
}

fn rbtdrf_rv_rbrn_invalid_entry_mode(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrn_neg(dir, "rbrn-invalid-entry-mode", "export RBRN_ENTRY_MODE=\"bogus\"")
}

fn rbtdrf_rv_rbrn_invalid_dns_mode(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrn_neg(dir, "rbrn-invalid-dns-mode", "export RBRN_UPLINK_DNS_MODE=\"bogus\"")
}

fn rbtdrf_rv_rbrn_invalid_access_mode(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrn_neg(dir, "rbrn-invalid-access-mode", "export RBRN_UPLINK_ACCESS_MODE=\"bogus\"")
}

fn rbtdrf_rv_rbrn_port_conflict(dir: &Path) -> rbtdre_Verdict {
    // Enabled-mode override with workstation port >= uplink min trips the
    // cross-port enforce check.
    rbtdrf_rv_rbrn_neg(dir, "rbrn-port-conflict",
        "export RBRN_ENTRY_MODE=\"rbnne_enabled\"\n\
         export RBRN_ENTRY_PORT_WORKSTATION=\"10001\"\n\
         export RBRN_ENTRY_PORT_ENCLAVE=\"8888\"\n\
         export RBRN_UPLINK_PORT_MIN=\"10000\"")
}

fn rbtdrf_rv_rbrn_unexpected_var(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrn_neg(dir, "rbrn-unexpected-var", "export RBRN_BOGUS=\"foo\"")
}

fn rbtdrf_rv_rbrn_bad_ip(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrn_neg(dir, "rbrn-bad-ip", "export RBRN_ENCLAVE_BASE_IP=\"not-an-ip\"")
}

// --- RBRP negative tests ---

// Valid synthetic RBRP baseline — payor project matches RBGC_GLOBAL_PAYOR_REGEX
// (^rbwg-p-[0-9]{12}$); the three optional fields stay empty so their format
// guards self-skip. The violation trips the payor-project regex, whose pattern
// lives in RBGC — supplied to the harness as a kindle prereq.
const RBTDRF_RBRP_BASELINE: &str = "\
export RBRP_PAYOR_PROJECT_ID=\"rbwg-p-260101000000\"\n\
export RBRP_BILLING_ACCOUNT_ID=\"\"\n\
export RBRP_OAUTH_CLIENT_ID=\"\"\n\
export RBRP_OPERATOR_EMAIL=\"\"";

// rbrp enforce reaches RBGC_GLOBAL_PAYOR_REGEX; stage rbgc kindled ahead of the
// probate, mirroring rbtdrf_rs_rbrr_nonempty_prefix.
const RBTDRF_RBGC_PRELUDE: &str = "source 'Tools/rbk/rbgc_Constants.sh'\nzrbgc_kindle\n";

fn rbtdrf_rv_rbrp_baseline_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate_in(dir, "Tools/rbk", RBTDRM_MODULE_RBRP, RBTDRM_PROBATE_RBRP,
        RBTDRF_RBGC_PRELUDE, RBTDRF_RBRP_BASELINE, "", true, "rbrp-baseline-valid")
}

fn rbtdrf_rv_rbrp_bad_payor_project(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate_in(dir, "Tools/rbk", RBTDRM_MODULE_RBRP, RBTDRM_PROBATE_RBRP,
        RBTDRF_RBGC_PRELUDE, RBTDRF_RBRP_BASELINE,
        "export RBRP_PAYOR_PROJECT_ID=\"not-a-payor-project\"", false, "rbrp-bad-payor-project")
}

// --- RBRS negative tests ---

const RBTDRF_RBRS_BASELINE: &str = "\
export RBRS_PODMAN_ROOT_DIR=\"/tmp/podman-root\"\n\
export RBRS_VMIMAGE_CACHE_DIR=\"/tmp/vmimage-cache\"\n\
export RBRS_VM_PLATFORM=\"linux/arm64\"";

fn rbtdrf_rv_rbrs_baseline_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRS, RBTDRM_PROBATE_RBRS,
        RBTDRF_RBRS_BASELINE, "", true, "rbrs-baseline-valid")
}

fn rbtdrf_rv_rbrs_missing_platform(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRS, RBTDRM_PROBATE_RBRS,
        RBTDRF_RBRS_BASELINE, "unset RBRS_VM_PLATFORM", false, "rbrs-missing-platform")
}

// --- RBRO negative tests ---

// Synthetic non-secret RBRO baseline — a malformed installed OAuth credential
// is a real user-facing failure, but the values here carry no secret.
const RBTDRF_RBRO_BASELINE: &str = "\
export RBRO_CLIENT_SECRET=\"synthetic-non-secret-client-secret\"\n\
export RBRO_REFRESH_TOKEN=\"synthetic-non-secret-refresh-token\"";

fn rbtdrf_rv_rbro_baseline_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRO, RBTDRM_PROBATE_RBRO,
        RBTDRF_RBRO_BASELINE, "", true, "rbro-baseline-valid")
}

fn rbtdrf_rv_rbro_missing_refresh_token(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRO, RBTDRM_PROBATE_RBRO,
        RBTDRF_RBRO_BASELINE, "unset RBRO_REFRESH_TOKEN", false, "rbro-missing-refresh-token")
}

// --- RBRA negative tests ---

// Synthetic non-secret RBRA baseline — role enum, SA-email regex, and PEM BEGIN
// material all satisfied. The violation strips the PEM marker, tripping rbra's
// custom private-key check.
const RBTDRF_RBRA_BASELINE: &str = "\
export RBRA_ROLE=\"rbnae_retriever\"\n\
export RBRA_CLIENT_EMAIL=\"synthetic@synthetic-project.iam.gserviceaccount.com\"\n\
export RBRA_PRIVATE_KEY=\"-----BEGIN PRIVATE KEY----- synthetic-non-secret-material -----END PRIVATE KEY-----\"\n\
export RBRA_PROJECT_ID=\"synthetic-project\"\n\
export RBRA_TOKEN_LIFETIME_SEC=\"3600\"";

fn rbtdrf_rv_rbra_baseline_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRA, RBTDRM_PROBATE_RBRA,
        RBTDRF_RBRA_BASELINE, "", true, "rbra-baseline-valid")
}

fn rbtdrf_rv_rbra_bad_private_key(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate(dir, RBTDRM_MODULE_RBRA, RBTDRM_PROBATE_RBRA,
        RBTDRF_RBRA_BASELINE, "export RBRA_PRIVATE_KEY=\"no-pem-material\"", false, "rbra-bad-private-key")
}

// --- BURC negative tests ---

const RBTDRF_BURC_BASELINE: &str = "\
export BURC_STATION_FILE=\"/tmp/station/burs.env\"\n\
export BURC_TABTARGET_DIR=\"tt\"\n\
export BURC_TABTARGET_DELIMITER=\".\"\n\
export BURC_TOOLS_DIR=\"Tools\"\n\
export BURC_PROJECT_ROOT=\"..\"\n\
export BURC_MANAGED_KITS=\"rbk,buk\"\n\
export BURC_TEMP_ROOT_DIR=\"/tmp/burc-temp\"\n\
export BURC_OUTPUT_ROOT_DIR=\"/tmp/burc-output\"\n\
export BURC_LOG_LAST=\"last\"\n\
export BURC_LOG_EXT=\"txt\"";

fn rbtdrf_rv_burc_baseline_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate_in(dir, "Tools/buk", RBTDRM_MODULE_BURC, RBTDRM_PROBATE_BURC,
        "", RBTDRF_BURC_BASELINE, "", true, "burc-baseline-valid")
}

fn rbtdrf_rv_burc_missing_station_file(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate_in(dir, "Tools/buk", RBTDRM_MODULE_BURC, RBTDRM_PROBATE_BURC,
        "", RBTDRF_BURC_BASELINE, "unset BURC_STATION_FILE", false, "burc-missing-station-file")
}

// --- BURN negative tests ---

const RBTDRF_BURN_BASELINE: &str = "\
export BURN_HOST=\"192.0.2.10\"\n\
export BURN_PLATFORM=\"bunne_linux\"";

fn rbtdrf_rv_burn_baseline_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate_in(dir, "Tools/buk", RBTDRM_MODULE_BURN, RBTDRM_PROBATE_BURN,
        "", RBTDRF_BURN_BASELINE, "", true, "burn-baseline-valid")
}

fn rbtdrf_rv_burn_bad_platform(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate_in(dir, "Tools/buk", RBTDRM_MODULE_BURN, RBTDRM_PROBATE_BURN,
        "", RBTDRF_BURN_BASELINE, "export BURN_PLATFORM=\"bunne_solaris\"", false, "burn-bad-platform")
}

// --- BURP negative tests ---

// Synthetic non-secret BURP baseline — the credential fields are filesystem
// paths to operator-managed keys, not secret material themselves.
const RBTDRF_BURP_BASELINE: &str = "\
export BURP_PRIVILEGED_USER=\"admin\"\n\
export BURP_PRIVILEGED_KEY_FILE=\"/tmp/keys/privileged_id_ed25519\"\n\
export BURP_WORKLOAD_KEY_FILE=\"/tmp/keys/workload_id_ed25519\"";

fn rbtdrf_rv_burp_baseline_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate_in(dir, "Tools/buk", RBTDRM_MODULE_BURP, RBTDRM_PROBATE_BURP,
        "", RBTDRF_BURP_BASELINE, "", true, "burp-baseline-valid")
}

fn rbtdrf_rv_burp_missing_workload_key(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate_in(dir, "Tools/buk", RBTDRM_MODULE_BURP, RBTDRM_PROBATE_BURP,
        "", RBTDRF_BURP_BASELINE, "unset BURP_WORKLOAD_KEY_FILE", false, "burp-missing-workload-key")
}

// --- BURS negative tests ---

const RBTDRF_BURS_BASELINE: &str = "\
export BURS_USER=\"devuser\"\n\
export BURS_TINCTURE=\"ab1\"\n\
export BURS_LOG_DIR=\"/tmp/burs-logs\"";

// burs kindle expands BUBC_moorings_dir / BUBC_rbmu_users_subdir in the
// BURS_USER enrollment help-string; under set -u those must exist. bubc derives
// the former from BURD_CONFIG_DIR, so stage a synthetic value and source bubc.
// The values feed only display text — no validation reads them.
const RBTDRF_BUBC_PRELUDE: &str =
    "export BURD_CONFIG_DIR='rbmm_moorings'\nsource 'Tools/buk/bubc_constants.sh'\n";

fn rbtdrf_rv_burs_baseline_valid(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate_in(dir, "Tools/buk", RBTDRM_MODULE_BURS, RBTDRM_PROBATE_BURS,
        RBTDRF_BUBC_PRELUDE, RBTDRF_BURS_BASELINE, "", true, "burs-baseline-valid")
}

fn rbtdrf_rv_burs_bad_tincture(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_probate_in(dir, "Tools/buk", RBTDRM_MODULE_BURS, RBTDRM_PROBATE_BURS,
        RBTDRF_BUBC_PRELUDE, RBTDRF_BURS_BASELINE, "export BURS_TINCTURE=\"A1\"", false, "burs-bad-tincture")
}

// --- Positive tests ---

fn rbtdrf_rv_rbrr_repo(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    match rbtdrf_run_tt(&root, RBTDGC_VALIDATE_REPO, &[], dir, "rbrr-repo-validate") {
        Ok(()) => rbtdre_Verdict::Pass,
        Err(e) => rbtdre_Verdict::Fail(e),
    }
}

fn rbtdrf_rv_rbrv_all_vessels(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };

    // Discover vessels: source rbrr.env to get RBRR_VESSEL_DIR, scan it
    let buv = root.join("Tools/buk/buv_validation.sh");
    let rbk = root.join("Tools/rbk");
    let buv_p = rbtdrx_native_to_posix(&buv);
    let rbk_p = rbtdrx_native_to_posix(&rbk);
    let script = format!(
        "set -euo pipefail\n\
         source '{}'\n\
         source '{}/rbcc_Constants.sh'\n\
         source '{}/rbgc_Constants.sh'\n\
         source '{}/rbrr_regime.sh'\n\
         source '{}/rbrd_regime.sh'\n\
         source '{}/rbdc_DerivedConstants.sh'\n\
         zbuv_kindle\nzrbcc_kindle\n\
         source \"${{PWD}}/{moorings}/rbrr.env\"\n\
         source \"${{PWD}}/{moorings}/rbrd.env\"\n\
         zrbrr_kindle\nzrbrd_kindle\nzrbrr_enforce\nzrbrd_enforce\nzrbdc_kindle\n\
         echo \"${{RBRR_VESSEL_DIR}}\"",
        buv_p,
        rbk_p, rbk_p, rbk_p, rbk_p, rbk_p,
        moorings = crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR,
    );

    let vessel_dir = match rbtdrf_run_bash(&root, &script, dir, "rbrv-discover") {
        Ok((0, stdout, _)) => stdout.trim().to_string(),
        Ok((code, _, _)) => {
            return rbtdre_Verdict::Fail(format!("vessel discovery failed (exit {})", code));
        }
        Err(e) => return rbtdre_Verdict::Fail(e),
    };

    let entries = match std::fs::read_dir(&vessel_dir) {
        Ok(e) => e,
        Err(e) => {
            return rbtdre_Verdict::Fail(format!("cannot read {}: {}", vessel_dir, e));
        }
    };

    let mut found = false;
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() && path.join("rbrv.env").exists() {
            found = true;
            let sigil = entry.file_name().to_string_lossy().to_string();
            if let Err(e) = rbtdrf_run_tt(
                &root, RBTDGC_VALIDATE_VESSEL, &[&sigil], dir,
                &format!("rbrv-{}-validate", sigil),
            ) {
                return rbtdre_Verdict::Fail(e);
            }
        }
    }

    if !found {
        return rbtdre_Verdict::Fail(format!("no vessels found in {}", vessel_dir));
    }
    rbtdre_Verdict::Pass
}

fn rbtdrf_rv_rbrn_all_nameplates(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };

    // Discover nameplates by listing rbmm_moorings/*/rbrn.env
    let rbk_dir = root.join(crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR);
    let entries = match std::fs::read_dir(&rbk_dir) {
        Ok(e) => e,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot read {}: {}", crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR, e)),
    };

    let mut found = false;
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() && path.join("rbrn.env").exists() {
            found = true;
            let moniker = entry.file_name().to_string_lossy().to_string();
            if let Err(e) = rbtdrf_run_tt(
                &root, RBTDGC_VALIDATE_NAMEPLATE, &[&moniker], dir,
                &format!("rbrn-{}-validate", moniker),
            ) {
                return rbtdre_Verdict::Fail(e);
            }
        }
    }

    if !found {
        return rbtdre_Verdict::Fail(format!("no nameplates found in {}/", crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR));
    }
    rbtdre_Verdict::Pass
}

// ── Regime-smoke cases ──────────────────────────────────────

fn rbtdrf_rs_render_validate(dir: &Path, render: &str, validate: &str, label: &str) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    if let Err(e) = rbtdrf_run_tt(&root, render, &[], dir, &format!("{}-render", label)) {
        return rbtdre_Verdict::Fail(e);
    }
    if let Err(e) = rbtdrf_run_tt(&root, validate, &[], dir, &format!("{}-validate", label)) {
        return rbtdre_Verdict::Fail(e);
    }
    rbtdre_Verdict::Pass
}

fn rbtdrf_rs_burc(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rs_render_validate(dir, "buw-rcr", "buw-rcv", "burc")
}

fn rbtdrf_rs_burs(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rs_render_validate(dir, "buw-rsr", "buw-rsv", "burs")
}

// The RBK station regime (RBRS) lives outside the repo, in the operator's
// station-files tree — present only on a configured workstation. Unlike the
// repo-baseline regimes above it self-skips when absent, so the fast suite
// stays green on a fresh checkout while a configured station validates for
// real. This is the surviving slice of the orphaned regime-credentials suite;
// its RBRA/RBRO siblings are covered more strongly by the service-tier
// access-probe, which mints real tokens against those credentials.
fn rbtdrf_rs_rbrs(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    if !root.join(RBTDGC_RBRS_FILE).exists() {
        return rbtdre_Verdict::Skip(format!(
            "station file {} absent — requires a configured workstation",
            RBTDGC_RBRS_FILE
        ));
    }
    rbtdrf_rs_render_validate(dir, RBTDGC_RENDER_STATION, RBTDGC_VALIDATE_STATION, "rbrs")
}

fn rbtdrf_rs_rbrn(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };

    // Discover nameplates by listing rbmm_moorings/*/rbrn.env
    let rbk_dir = root.join(crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR);
    let entries = match std::fs::read_dir(&rbk_dir) {
        Ok(e) => e,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot read {}: {}", crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR, e)),
    };

    let mut found = false;
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() && path.join("rbrn.env").exists() {
            found = true;
            let moniker = entry.file_name().to_string_lossy().to_string();
            if let Err(e) = rbtdrf_run_tt(
                &root, RBTDGC_RENDER_NAMEPLATE, &[&moniker], dir,
                &format!("rbrn-{}-render", moniker),
            ) {
                return rbtdre_Verdict::Fail(e);
            }
            if let Err(e) = rbtdrf_run_tt(
                &root, RBTDGC_VALIDATE_NAMEPLATE, &[&moniker], dir,
                &format!("rbrn-{}-validate", moniker),
            ) {
                return rbtdre_Verdict::Fail(e);
            }
        }
    }

    if !found {
        return rbtdre_Verdict::Fail(format!("no nameplates found in {}/", crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR));
    }
    rbtdre_Verdict::Pass
}

fn rbtdrf_rs_rbrr(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rs_render_validate(dir, RBTDGC_RENDER_REPO, RBTDGC_VALIDATE_REPO, "rbrr")
}

fn rbtdrf_rs_rbrr_nonempty_prefix(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let buv = root.join("Tools/buk/buv_validation.sh");
    let rbk = root.join("Tools/rbk");

    let buv_p = rbtdrx_native_to_posix(&buv);
    let rbk_p = rbtdrx_native_to_posix(&rbk);
    let script = format!(
        "set -euo pipefail\n\
         source '{}'\n\
         source '{}/rbcc_Constants.sh'\n\
         source '{}/rbgc_Constants.sh'\n\
         source '{}/rbrr_regime.sh'\n\
         source '{}/rbrd_regime.sh'\n\
         source '{}/rbgl_GarLayout.sh'\n\
         zbuv_kindle\nzrbcc_kindle\nzrbgc_kindle\n\
         source \"${{PWD}}/{moorings}/rbrr.env\"\n\
         source \"${{PWD}}/{moorings}/rbrd.env\"\n\
         {cloud_var}=\"acme-\"\n\
         {runtime_var}=\"acme-\"\n\
         zrbrr_kindle\nzrbrd_kindle\nzrbrr_enforce\nzrbrd_enforce\nzrbgl_kindle\n\
         echo \"hallmarks_root=${{RBGL_HALLMARKS_ROOT}}\"\n\
         echo \"runtime_prefix=${{{runtime_var}}}\"",
        buv_p,
        rbk_p, rbk_p, rbk_p, rbk_p, rbk_p,
        cloud_var = RBTDRF_VAR_RBRD_CLOUD_PREFIX,
        runtime_var = RBTDRF_VAR_RBRR_RUNTIME_PREFIX,
        moorings = crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR,
    );

    match rbtdrf_run_bash(&root, &script, dir, "rbrr-nonempty-prefix") {
        Ok((0, stdout, _)) => {
            let expected_line = format!("hallmarks_root={}", RBTDRF_VAL_HALLMARKS_ROOT);
            let hallmarks_ok = stdout
                .lines()
                .any(|l| l.trim() == expected_line);
            let runtime_ok = stdout
                .lines()
                .any(|l| l.trim() == "runtime_prefix=acme-");
            if !hallmarks_ok {
                return rbtdre_Verdict::Fail(format!(
                    "RBGL_HALLMARKS_ROOT did not match category constant; stdout:\n{}",
                    stdout
                ));
            }
            if !runtime_ok {
                return rbtdre_Verdict::Fail(format!(
                    "RBRR_RUNTIME_PREFIX did not propagate; stdout:\n{}",
                    stdout
                ));
            }
            rbtdre_Verdict::Pass
        }
        Ok((code, _, stderr)) => {
            rbtdre_Verdict::Fail(format!("kindle failed (exit {}); stderr:\n{}", code, stderr))
        }
        Err(e) => rbtdre_Verdict::Fail(e),
    }
}

fn rbtdrf_rs_rbrv(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };

    // Discover vessels: source rbrr.env to get RBRR_VESSEL_DIR, scan it
    let buv = root.join("Tools/buk/buv_validation.sh");
    let rbk = root.join("Tools/rbk");

    let buv_p = rbtdrx_native_to_posix(&buv);
    let rbk_p = rbtdrx_native_to_posix(&rbk);
    let script = format!(
        "set -euo pipefail\n\
         source '{}'\n\
         source '{}/rbcc_Constants.sh'\n\
         source '{}/rbgc_Constants.sh'\n\
         source '{}/rbrr_regime.sh'\n\
         source '{}/rbrd_regime.sh'\n\
         source '{}/rbdc_DerivedConstants.sh'\n\
         zbuv_kindle\nzrbcc_kindle\n\
         source \"${{PWD}}/{moorings}/rbrr.env\"\n\
         source \"${{PWD}}/{moorings}/rbrd.env\"\n\
         zrbrr_kindle\nzrbrd_kindle\nzrbrr_enforce\nzrbrd_enforce\nzrbdc_kindle\n\
         echo \"${{RBRR_VESSEL_DIR}}\"",
        buv_p,
        rbk_p, rbk_p, rbk_p, rbk_p, rbk_p,
        moorings = crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR,
    );

    let vessel_dir = match rbtdrf_run_bash(&root, &script, dir, "rbrv-discover") {
        Ok((0, stdout, _)) => stdout.trim().to_string(),
        Ok((code, _, _)) => {
            return rbtdre_Verdict::Fail(format!("vessel discovery failed (exit {})", code));
        }
        Err(e) => return rbtdre_Verdict::Fail(e),
    };

    let entries = match std::fs::read_dir(&vessel_dir) {
        Ok(e) => e,
        Err(e) => {
            return rbtdre_Verdict::Fail(format!("cannot read {}: {}", vessel_dir, e));
        }
    };

    let mut found = false;
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() && path.join("rbrv.env").exists() {
            found = true;
            let sigil = entry.file_name().to_string_lossy().to_string();
            if let Err(e) = rbtdrf_run_tt(
                &root, RBTDGC_RENDER_VESSEL, &[&sigil], dir,
                &format!("rbrv-{}-render", sigil),
            ) {
                return rbtdre_Verdict::Fail(e);
            }
            if let Err(e) = rbtdrf_run_tt(
                &root, RBTDGC_VALIDATE_VESSEL, &[&sigil], dir,
                &format!("rbrv-{}-validate", sigil),
            ) {
                return rbtdre_Verdict::Fail(e);
            }
        }
    }

    if !found {
        return rbtdre_Verdict::Fail(format!("no vessels found in {}", vessel_dir));
    }
    rbtdre_Verdict::Pass
}

fn rbtdrf_rs_rbrp(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rs_render_validate(dir, RBTDGC_RENDER_PAYOR, RBTDGC_VALIDATE_PAYOR, "rbrp")
}

fn rbtdrf_rs_burd(dir: &Path) -> rbtdre_Verdict {
    // BURD dispatch environment verification — invoke a minimal tabtarget
    // (buw-rcv is fast, side-effect-free) and confirm the dispatch machinery
    // ran successfully. The original bash test verified BURD sentinel+enforce
    // inside a live dispatch context; here we verify dispatch works by observing
    // a tabtarget that goes through the full BUK dispatch path.
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    if let Err(e) = rbtdrf_run_tt(&root, "buw-rcv", &[], dir, "burd-dispatch") {
        return rbtdre_Verdict::Fail(e);
    }
    rbtdre_Verdict::Pass
}

// ── Tabtarget-refusal cases ─────────────────────────────────

/// rbw-dU empty-arg refusal — BBAA9 contract. Invoking rbw-dU with no
/// argument must die non-zero and emit the rbw-dl pointer (operator
/// discovery for candidate depot project IDs). BUW dispatch merges
/// stderr→stdout via `2>&1` (bud_dispatch.sh:372), so the captured
/// stdout carries the buc_warn/buc_info/buc_tabtarget/buc_die output
/// from rbgp_depot_unmake's no-arg branch (rbgp_Payor.sh:937-942).
///
/// Pure shell, no GCP traffic — refusal lands before authenticate.
fn rbtdrf_rs_unmake_empty_arg_refusal(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };

    let tt = match rbtdri_find_tabtarget_global(&root, RBTDGC_UNMAKE_DEPOT) {
        Ok(p) => p,
        Err(e) => return rbtdre_Verdict::Fail(e),
    };

    let output = match rbtdri_tabtarget_command(&tt).current_dir(&root).output() {
        Ok(o) => o,
        Err(e) => {
            return rbtdre_Verdict::Fail(format!(
                "failed to run {}: {}",
                tt.display(),
                e
            ));
        }
    };

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let code = output.status.code().unwrap_or(-1);
    let _ = std::fs::write(dir.join("empty-arg-stdout.txt"), &stdout);
    let _ = std::fs::write(dir.join("empty-arg-stderr.txt"), &stderr);

    if code == 0 {
        return rbtdre_Verdict::Fail(format!(
            "{} exited 0 with no argument — BBAA9 empty-arg refusal contract violated",
            RBTDGC_UNMAKE_DEPOT
        ));
    }

    let combined = format!("{}{}", stdout, stderr);
    if !combined.contains(RBTDGC_LIST_DEPOT) {
        return rbtdre_Verdict::Fail(format!(
            "{} empty-arg refusal did not point at {} for operator discovery\n\
             stdout:\n{}\n\nstderr:\n{}",
            RBTDGC_UNMAKE_DEPOT, RBTDGC_LIST_DEPOT, stdout, stderr
        ));
    }

    rbtdre_Verdict::Pass
}

// ── Dockerfile-hygiene cases ────────────────────────────────
//
// Drives the Dockerfile FROM-line hygiene contract through the rbw-fhc and
// rbw-fhv tabtargets — exercising the contract surface, not module internals.
// Eight synthetic cases (5 positive, 3 negative) feed inline Dockerfile bodies
// to rbw-fhc; one all-vessels case iterates real conjure vessels through
// rbw-fhv with a Rust-side counter that fails verdict on zero iterations.

const RBTDRF_DH_BODY_PARAMETERIZED: &str = "FROM ${RBF_IMAGE_1}\n";
const RBTDRF_DH_BODY_SCRATCH: &str = "FROM scratch\n";
const RBTDRF_DH_BODY_MULTISTAGE_AS: &str = "FROM ${RBF_IMAGE_1} AS builder\n";
const RBTDRF_DH_BODY_EMPTY: &str = "";
const RBTDRF_DH_BODY_COMMENTS_ONLY: &str = "# top-level comment\n# another comment\n";
const RBTDRF_DH_BODY_HARDCODED_LITERAL: &str = "FROM python:3.12-slim\n";
const RBTDRF_DH_BODY_TAB_IN_FROM: &str = "FROM\t${RBF_IMAGE_1}\n";
const RBTDRF_DH_BODY_TRAILING_BACKSLASH: &str = "FROM ${RBF_IMAGE_1} \\\n";

fn rbtdrf_dh_run_synthetic(
    dir: &Path,
    label: &str,
    body: &str,
    expect_pass: bool,
) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let dockerfile = dir.join(format!("{}-Dockerfile", label));
    if let Err(e) = std::fs::write(&dockerfile, body) {
        return rbtdre_Verdict::Fail(format!("{}: write Dockerfile failed: {}", label, e));
    }
    let path_str = dockerfile.to_string_lossy().into_owned();
    let result = if expect_pass {
        rbtdrf_run_tt(&root, RBTDGC_HYGIENE_CHECK_DOCKERFILE, &[&path_str], dir, label)
    } else {
        rbtdrf_run_tt_neg(&root, RBTDGC_HYGIENE_CHECK_DOCKERFILE, &[&path_str], dir, label)
    };
    match result {
        Ok(()) => rbtdre_Verdict::Pass,
        Err(e) => rbtdre_Verdict::Fail(e),
    }
}

fn rbtdrf_dh_accept_parameterized(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_dh_run_synthetic(dir, "dh-accept-parameterized", RBTDRF_DH_BODY_PARAMETERIZED, true)
}

fn rbtdrf_dh_accept_scratch(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_dh_run_synthetic(dir, "dh-accept-scratch", RBTDRF_DH_BODY_SCRATCH, true)
}

fn rbtdrf_dh_accept_multistage_as(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_dh_run_synthetic(dir, "dh-accept-multistage-as", RBTDRF_DH_BODY_MULTISTAGE_AS, true)
}

fn rbtdrf_dh_accept_empty(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_dh_run_synthetic(dir, "dh-accept-empty", RBTDRF_DH_BODY_EMPTY, true)
}

fn rbtdrf_dh_accept_comments_only(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_dh_run_synthetic(dir, "dh-accept-comments-only", RBTDRF_DH_BODY_COMMENTS_ONLY, true)
}

fn rbtdrf_dh_reject_hardcoded_literal(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_dh_run_synthetic(dir, "dh-reject-hardcoded-literal", RBTDRF_DH_BODY_HARDCODED_LITERAL, false)
}

fn rbtdrf_dh_reject_tab_in_from(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_dh_run_synthetic(dir, "dh-reject-tab-in-from", RBTDRF_DH_BODY_TAB_IN_FROM, false)
}

fn rbtdrf_dh_reject_trailing_backslash(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_dh_run_synthetic(dir, "dh-reject-trailing-backslash", RBTDRF_DH_BODY_TRAILING_BACKSLASH, false)
}

fn rbtdrf_dh_all_vessels_pass(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };

    // Resolve RBRR_VESSEL_DIR via one-shot bash (kindle ceremony unavoidable
    // for derived-path resolution).
    let buv = root.join("Tools/buk/buv_validation.sh");
    let rbk = root.join("Tools/rbk");
    let buv_p = rbtdrx_native_to_posix(&buv);
    let rbk_p = rbtdrx_native_to_posix(&rbk);
    let resolve_script = format!(
        "set -euo pipefail\n\
         source '{}'\n\
         source '{}/rbcc_Constants.sh'\n\
         source '{}/rbgc_Constants.sh'\n\
         source '{}/rbrr_regime.sh'\n\
         source '{}/rbrd_regime.sh'\n\
         source '{}/rbdc_DerivedConstants.sh'\n\
         zbuv_kindle\nzrbcc_kindle\n\
         source \"${{PWD}}/{moorings}/rbrr.env\"\n\
         source \"${{PWD}}/{moorings}/rbrd.env\"\n\
         zrbrr_kindle\nzrbrd_kindle\nzrbrr_enforce\nzrbrd_enforce\nzrbdc_kindle\n\
         printf '%s' \"${{RBRR_VESSEL_DIR}}\"",
        buv_p,
        rbk_p, rbk_p, rbk_p, rbk_p, rbk_p,
        moorings = crate::rbtdgc_consts::RBTDGC_MOORINGS_DIR,
    );
    let vessel_dir = match rbtdrf_run_bash(&root, &resolve_script, dir, "resolve-vessel-dir") {
        Ok((0, stdout, _)) => stdout,
        Ok((code, _, stderr)) => {
            return rbtdre_Verdict::Fail(format!(
                "resolve RBRR_VESSEL_DIR failed (exit {}): {}",
                code, stderr
            ));
        }
        Err(e) => return rbtdre_Verdict::Fail(e),
    };
    let vessel_dir = vessel_dir.trim();
    if vessel_dir.is_empty() {
        return rbtdre_Verdict::Fail("RBRR_VESSEL_DIR resolved to empty string".to_string());
    }

    let entries = match std::fs::read_dir(vessel_dir) {
        Ok(e) => e,
        Err(e) => {
            return rbtdre_Verdict::Fail(format!(
                "read_dir({}) failed: {}",
                vessel_dir, e
            ));
        }
    };

    // rbw-fhv silently succeeds on non-conjure vessels (hygiene contract is
    // vacuously satisfied where there's no local Dockerfile), so theurge
    // iterates without pre-filtering — surface integrity stays intact and
    // no rbrv.env internals are touched here.
    let mut count: usize = 0;
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => return rbtdre_Verdict::Fail(format!("dir entry error: {}", e)),
        };
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        if !path.join("rbrv.env").is_file() {
            continue;
        }
        let sigil = match path.file_name().and_then(|s| s.to_str()) {
            Some(s) => s.to_string(),
            None => continue,
        };

        if let Err(e) = rbtdrf_run_tt(
            &root,
            RBTDGC_HYGIENE_CHECK_VESSEL,
            &[&sigil],
            dir,
            &format!("vessel-{}", sigil),
        ) {
            return rbtdre_Verdict::Fail(e);
        }
        count += 1;
    }

    if count == 0 {
        return rbtdre_Verdict::Fail(format!(
            "zero vessels iterated under {} — busted RBRR_VESSEL_DIR resolution",
            vessel_dir
        ));
    }

    rbtdre_Verdict::Pass
}

// ── Foundry-path cases ──────────────────────────────────────
//
// Drives zrbfc_native_path_capture directly: source rbfcb_BuildHost.sh, force
// BURD_OSTYPE, assert the normalized stdout (or, for the bare-absolute
// unsurveyed shape, that the capture returns non-zero). The normalizer is
// sentinel-free and reads only its argument plus BURD_OSTYPE, so this stays a
// dependency-free unit test — no foundry kindle, no regime, no credentials —
// and exercises the Cygwin transform on any host by forcing the platform fact.

fn rbtdrf_np_run(
    dir: &Path,
    label: &str,
    ostype: &str,
    input: &str,
    expect: Option<&str>,
) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let rbfcb = root.join("Tools/rbk/rbfcb_BuildHost.sh");

    let assertion = match expect {
        Some(out) => format!("test \"$(zrbfc_native_path_capture '{}')\" = '{}'", input, out),
        None => format!("zrbfc_native_path_capture '{}'", input),
    };
    let script = format!(
        "set -euo pipefail\nsource '{}'\nexport BURD_OSTYPE='{}'\n{}",
        rbtdrx_native_to_posix(&rbfcb),
        ostype,
        assertion,
    );

    let expect_ok = expect.is_some();
    match rbtdrf_run_bash(&root, &script, dir, label) {
        Ok((code, _, _)) => {
            let ok = code == 0;
            if ok == expect_ok {
                rbtdre_Verdict::Pass
            } else if expect_ok {
                rbtdre_Verdict::Fail(format!("{}: expected ok, got exit {}", label, code))
            } else {
                rbtdre_Verdict::Fail(format!("{}: expected non-zero (unsurveyed), got exit 0", label))
            }
        }
        Err(e) => rbtdre_Verdict::Fail(format!("{}: {}", label, e)),
    }
}

fn rbtdrf_np_cygdrive_transform(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_np_run(dir, "np-cygdrive-transform", "cygwin",
        "/cygdrive/c/Users/foo", Some("c:/Users/foo"))
}

fn rbtdrf_np_relative_passthrough(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_np_run(dir, "np-relative-passthrough", "cygwin",
        "rbmv_vessels/rbev-busybox", Some("rbmv_vessels/rbev-busybox"))
}

fn rbtdrf_np_native_passthrough(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_np_run(dir, "np-native-passthrough", "cygwin",
        "c:/Users/foo", Some("c:/Users/foo"))
}

fn rbtdrf_np_offcygwin_identity(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_np_run(dir, "np-offcygwin-identity", "linux-gnu",
        "/cygdrive/c/Users/foo", Some("/cygdrive/c/Users/foo"))
}

fn rbtdrf_np_bare_absolute_unsurveyed(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_np_run(dir, "np-bare-absolute-unsurveyed", "cygwin",
        "/etc/hosts", None)
}

// ── Case arrays ─────────────────────────────────────────────

pub static RBTDRF_CASES_ENROLLMENT_VALIDATION: &[rbtdre_Case] = &[
    case!(rbtdrf_ev_string_valid),
    case!(rbtdrf_ev_string_empty_optional),
    case!(rbtdrf_ev_string_too_short),
    case!(rbtdrf_ev_string_too_long),
    case!(rbtdrf_ev_string_empty_required),
    case!(rbtdrf_ev_xname_valid),
    case!(rbtdrf_ev_xname_invalid),
    case!(rbtdrf_ev_gname_valid),
    case!(rbtdrf_ev_gname_invalid),
    case!(rbtdrf_ev_fqin_valid),
    case!(rbtdrf_ev_fqin_invalid),
    case!(rbtdrf_ev_bool_valid),
    case!(rbtdrf_ev_bool_invalid),
    case!(rbtdrf_ev_bool_empty),
    case!(rbtdrf_ev_enum_valid),
    case!(rbtdrf_ev_enum_invalid),
    case!(rbtdrf_ev_enum_empty),
    case!(rbtdrf_ev_decimal_valid),
    case!(rbtdrf_ev_decimal_below),
    case!(rbtdrf_ev_decimal_above),
    case!(rbtdrf_ev_decimal_empty),
    case!(rbtdrf_ev_ipv4_valid),
    case!(rbtdrf_ev_ipv4_invalid),
    case!(rbtdrf_ev_port_valid),
    case!(rbtdrf_ev_port_invalid),
    case!(rbtdrf_ev_odref_valid),
    case!(rbtdrf_ev_odref_no_digest),
    case!(rbtdrf_ev_odref_malformed),
    case!(rbtdrf_ev_odref_empty),
    case!(rbtdrf_ev_list_string_valid),
    case!(rbtdrf_ev_list_string_empty),
    case!(rbtdrf_ev_list_string_bad_item),
    case!(rbtdrf_ev_list_ipv4_valid),
    case!(rbtdrf_ev_list_ipv4_invalid),
    case!(rbtdrf_ev_list_ipv4_empty),
    case!(rbtdrf_ev_list_gname_valid),
    case!(rbtdrf_ev_list_gname_invalid),
    case!(rbtdrf_ev_gate_active_valid),
    case!(rbtdrf_ev_gate_active_invalid),
    case!(rbtdrf_ev_gate_inactive),
    case!(rbtdrf_ev_gate_multi),
    case!(rbtdrf_ev_enforce_all_pass),
    case!(rbtdrf_ev_enforce_first_bad),
    case!(rbtdrf_ev_report_all_pass),
    case!(rbtdrf_ev_report_mixed),
    case!(rbtdrf_ev_report_gated),
    case!(rbtdrf_ev_multiscope),
];

pub static RBTDRF_CASES_REGIME_VALIDATION: &[rbtdre_Case] = &[
    case!(rbtdrf_rv_rbrr_baseline_valid),
    case!(rbtdrf_rv_rbrr_bad_timeout),
    case!(rbtdrf_rv_rbrr_unexpected_var),
    case!(rbtdrf_rv_rbrr_bad_vessel_dir),
    case!(rbtdrf_rv_rbrr_bad_secrets_dir),
    case!(rbtdrf_rv_rbrr_bad_runtime_prefix_uppercase),
    case!(rbtdrf_rv_rbrr_bad_runtime_prefix_no_trailing_hyphen),
    case!(rbtdrf_rv_rbrr_bad_runtime_prefix_too_long),
    case!(rbtdrf_rv_rbrd_baseline_valid),
    case!(rbtdrf_rv_rbrd_missing_moniker),
    case!(rbtdrf_rv_rbrd_bad_moniker),
    case!(rbtdrf_rv_rbrd_bad_cloud_prefix_uppercase),
    case!(rbtdrf_rv_rbrd_bad_cloud_prefix_no_trailing_hyphen),
    case!(rbtdrf_rv_rbrd_bad_cloud_prefix_too_long),
    case!(rbtdrf_rv_rbrv_conjure_baseline_valid),
    case!(rbtdrf_rv_rbrv_bind_baseline_valid),
    case!(rbtdrf_rv_rbrv_missing_sigil),
    case!(rbtdrf_rv_rbrv_no_bind_image),
    case!(rbtdrf_rv_rbrv_unexpected_var),
    case!(rbtdrf_rv_rbrv_partial_conjure),
    case!(rbtdrf_rv_rbrn_baseline_valid),
    case!(rbtdrf_rv_rbrn_missing_moniker),
    case!(rbtdrf_rv_rbrn_invalid_runtime),
    case!(rbtdrf_rv_rbrn_invalid_entry_mode),
    case!(rbtdrf_rv_rbrn_invalid_dns_mode),
    case!(rbtdrf_rv_rbrn_invalid_access_mode),
    case!(rbtdrf_rv_rbrn_port_conflict),
    case!(rbtdrf_rv_rbrn_unexpected_var),
    case!(rbtdrf_rv_rbrn_bad_ip),
    case!(rbtdrf_rv_rbrp_baseline_valid),
    case!(rbtdrf_rv_rbrp_bad_payor_project),
    case!(rbtdrf_rv_rbrs_baseline_valid),
    case!(rbtdrf_rv_rbrs_missing_platform),
    case!(rbtdrf_rv_rbro_baseline_valid),
    case!(rbtdrf_rv_rbro_missing_refresh_token),
    case!(rbtdrf_rv_rbra_baseline_valid),
    case!(rbtdrf_rv_rbra_bad_private_key),
    case!(rbtdrf_rv_burc_baseline_valid),
    case!(rbtdrf_rv_burc_missing_station_file),
    case!(rbtdrf_rv_burn_baseline_valid),
    case!(rbtdrf_rv_burn_bad_platform),
    case!(rbtdrf_rv_burp_baseline_valid),
    case!(rbtdrf_rv_burp_missing_workload_key),
    case!(rbtdrf_rv_burs_baseline_valid),
    case!(rbtdrf_rv_burs_bad_tincture),
    case!(rbtdrf_rv_rbrr_repo),
    case!(rbtdrf_rv_rbrv_all_vessels),
    case!(rbtdrf_rv_rbrn_all_nameplates),
];

pub static RBTDRF_CASES_REGIME_SMOKE: &[rbtdre_Case] = &[
    case!(rbtdrf_rs_burc),
    case!(rbtdrf_rs_burs),
    case!(rbtdrf_rs_rbrs),
    case!(rbtdrf_rs_rbrn),
    case!(rbtdrf_rs_rbrr),
    case!(rbtdrf_rs_rbrr_nonempty_prefix),
    case!(rbtdrf_rs_rbrv),
    case!(rbtdrf_rs_rbrp),
    case!(rbtdrf_rs_burd),
    case!(rbtdrf_rs_unmake_empty_arg_refusal),
];

// ── Fixture statics ──────────────────────────────────────────

pub static RBTDRF_FIXTURE_ENROLLMENT_VALIDATION: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_ENROLLMENT_VALIDATION,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRF_CASES_ENROLLMENT_VALIDATION,
};

pub static RBTDRF_FIXTURE_REGIME_VALIDATION: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_REGIME_VALIDATION,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRF_CASES_REGIME_VALIDATION,
};

pub static RBTDRF_FIXTURE_REGIME_SMOKE: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_REGIME_SMOKE,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRF_CASES_REGIME_SMOKE,
};

pub static RBTDRF_CASES_DOCKERFILE_HYGIENE: &[rbtdre_Case] = &[
    case!(rbtdrf_dh_accept_parameterized),
    case!(rbtdrf_dh_accept_scratch),
    case!(rbtdrf_dh_accept_multistage_as),
    case!(rbtdrf_dh_accept_empty),
    case!(rbtdrf_dh_accept_comments_only),
    case!(rbtdrf_dh_reject_hardcoded_literal),
    case!(rbtdrf_dh_reject_tab_in_from),
    case!(rbtdrf_dh_reject_trailing_backslash),
    case!(rbtdrf_dh_all_vessels_pass),
];

pub static RBTDRF_FIXTURE_DOCKERFILE_HYGIENE: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_DOCKERFILE_HYGIENE,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRF_CASES_DOCKERFILE_HYGIENE,
};

pub static RBTDRF_CASES_FOUNDRY_PATH: &[rbtdre_Case] = &[
    case!(rbtdrf_np_cygdrive_transform),
    case!(rbtdrf_np_relative_passthrough),
    case!(rbtdrf_np_native_passthrough),
    case!(rbtdrf_np_offcygwin_identity),
    case!(rbtdrf_np_bare_absolute_unsurveyed),
];

pub static RBTDRF_FIXTURE_FOUNDRY_PATH: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_FOUNDRY_PATH,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRF_CASES_FOUNDRY_PATH,
};
