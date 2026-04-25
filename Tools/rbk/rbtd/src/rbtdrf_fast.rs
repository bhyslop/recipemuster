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
use crate::rbtdre_engine::{rbtdre_Section, rbtdre_Verdict};
use crate::rbtdri_invocation::rbtdri_find_tabtarget_global;

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
    let output = Command::new("bash")
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
            buv.display(),
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

/// Run a regime-validation case: source modules, set up state, kindle+enforce.
fn rbtdrf_run_rv(
    dir: &Path,
    preamble: &str,
    setup: &str,
    expect_ok: bool,
    label: &str,
) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let buv = root.join("Tools/buk/buv_validation.sh");
    let rbk = root.join("Tools/rbk");

    let script = format!(
        "set -euo pipefail\n\
         source '{}'\n\
         source '{}/rbcc_Constants.sh'\n\
         source '{}/rbrr_regime.sh'\n\
         source '{}/rbrv_regime.sh'\n\
         source '{}/rbrn_regime.sh'\n\
         source '{}/rbdc_DerivedConstants.sh'\n\
         zbuv_kindle\nzrbcc_kindle\n\
         {}\n\
         {}",
        buv.display(),
        rbk.display(), rbk.display(), rbk.display(),
        rbk.display(), rbk.display(),
        preamble, setup,
    );

    match rbtdrf_run_bash(&root, &script, dir, label) {
        Ok((code, _, _)) => {
            let ok = code == 0;
            if ok != expect_ok {
                return if expect_ok {
                    rbtdre_Verdict::Fail(format!("{}: expected ok, got exit {}", label, code))
                } else {
                    rbtdre_Verdict::Fail(format!("{}: expected failure, got exit 0", label))
                };
            }
            rbtdre_Verdict::Pass
        }
        Err(e) => rbtdre_Verdict::Fail(format!("{}: {}", label, e)),
    }
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
    let output = Command::new(&tt)
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
        buv.display(),
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
        buv.display(),
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

// --- RBRR negative tests ---

fn rbtdrf_rv_rbrr_neg(dir: &Path, label: &str, setup: &str) -> rbtdre_Verdict {
    rbtdrf_run_rv(dir,
        "source \"${PWD}/.rbk/rbrr.env\"",
        &format!("{}\nzrbrr_kindle\nzrbrr_enforce", setup),
        false, label,
    )
}

fn rbtdrf_rv_rbrr_missing_project_id(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrr_neg(dir, "rbrr-missing-project-id", "unset RBRR_DEPOT_PROJECT_ID")
}

fn rbtdrf_rv_rbrr_bad_timeout(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrr_neg(dir, "rbrr-bad-timeout", "export RBRR_GCB_TIMEOUT=\"1200\"")
}

fn rbtdrf_rv_rbrr_bad_pool_stem(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrr_neg(dir, "rbrr-bad-pool-stem", "export RBRR_GCB_POOL_STEM=\"BAD_POOL_NAME\"")
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

fn rbtdrf_rv_rbrr_bad_cloud_prefix(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrr_neg(dir, "rbrr-bad-cloud-prefix",
        "export RBRR_CLOUD_PREFIX=\"BAD-\"")
}

fn rbtdrf_rv_rbrr_bad_runtime_prefix(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrr_neg(dir, "rbrr-bad-runtime-prefix",
        "export RBRR_RUNTIME_PREFIX=\"acme\"")
}

// --- RBRV negative tests ---

const RBTDRF_RBRV_BASELINE_CONJURE: &str = "\
export RBRV_SIGIL=\"test-vessel\"\n\
export RBRV_DESCRIPTION=\"Test vessel for validation\"\n\
export RBRV_VESSEL_MODE=\"conjure\"\n\
export RBRV_CONJURE_DOCKERFILE=\"path/to/Dockerfile\"\n\
export RBRV_CONJURE_BLDCONTEXT=\"path/to\"\n\
export RBRV_CONJURE_PLATFORMS=\"linux/amd64\"";

const RBTDRF_RBRV_BASELINE_BIND: &str = "\
export RBRV_SIGIL=\"test-vessel\"\n\
export RBRV_DESCRIPTION=\"Test vessel for validation\"\n\
export RBRV_VESSEL_MODE=\"bind\"\n\
export RBRV_BIND_IMAGE=\"us-docker.pkg.dev/project/repo/image:latest\"";

fn rbtdrf_rv_rbrv_neg(dir: &Path, label: &str, baseline: &str, override_: &str) -> rbtdre_Verdict {
    rbtdrf_run_rv(dir, "",
        &format!("{}\n{}\nzrbrv_kindle\nzrbrv_enforce", baseline, override_),
        false, label,
    )
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
export RBRN_ENTRY_MODE=\"disabled\"\n\
export RBRN_ENCLAVE_BASE_IP=\"10.200.0.0\"\n\
export RBRN_ENCLAVE_NETMASK=\"24\"\n\
export RBRN_ENCLAVE_SENTRY_IP=\"10.200.0.2\"\n\
export RBRN_ENCLAVE_BOTTLE_IP=\"10.200.0.3\"\n\
export RBRN_UPLINK_PORT_MIN=\"10000\"\n\
export RBRN_UPLINK_DNS_MODE=\"disabled\"\n\
export RBRN_UPLINK_ACCESS_MODE=\"disabled\"";

fn rbtdrf_rv_rbrn_neg(dir: &Path, label: &str, override_: &str) -> rbtdre_Verdict {
    rbtdrf_run_rv(dir, "",
        &format!("{}\n{}\nzrbrn_kindle\nzrbrn_enforce", RBTDRF_RBRN_BASELINE_DISABLED, override_),
        false, label,
    )
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
    // Use enabled baseline with port conflict: workstation >= uplink min
    rbtdrf_run_rv(dir, "",
        &format!("{}\n\
            export RBRN_ENTRY_MODE=\"enabled\"\n\
            export RBRN_ENTRY_PORT_WORKSTATION=\"10001\"\n\
            export RBRN_ENTRY_PORT_ENCLAVE=\"8888\"\n\
            export RBRN_UPLINK_PORT_MIN=\"10000\"\n\
            zrbrn_kindle\nzrbrn_enforce",
            RBTDRF_RBRN_BASELINE_DISABLED),
        false, "rbrn-port-conflict",
    )
}

fn rbtdrf_rv_rbrn_unexpected_var(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrn_neg(dir, "rbrn-unexpected-var", "export RBRN_BOGUS=\"foo\"")
}

fn rbtdrf_rv_rbrn_bad_ip(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rv_rbrn_neg(dir, "rbrn-bad-ip", "export RBRN_ENCLAVE_BASE_IP=\"not-an-ip\"")
}

// --- Positive tests ---

fn rbtdrf_rv_rbrr_repo(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_run_rv(dir,
        "source \"${PWD}/.rbk/rbrr.env\"",
        "zrbrr_kindle\nzrbrr_enforce",
        true, "rbrr-repo",
    )
}

fn rbtdrf_rv_rbrv_all_vessels(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let buv = root.join("Tools/buk/buv_validation.sh");
    let rbk = root.join("Tools/rbk");

    // Script: source rbrr, kindle, iterate vessels, kindle+enforce each
    let script = format!(
        "set -euo pipefail\n\
         source '{}'\n\
         source '{}/rbcc_Constants.sh'\n\
         source '{}/rbrr_regime.sh'\n\
         source '{}/rbrv_regime.sh'\n\
         source '{}/rbdc_DerivedConstants.sh'\n\
         zbuv_kindle\nzrbcc_kindle\n\
         source \"${{PWD}}/.rbk/rbrr.env\"\n\
         zrbrr_kindle\nzrbrr_enforce\nzrbdc_kindle\n\
         for z_d in \"${{RBRR_VESSEL_DIR}}\"/*; do\n\
           test -d \"$z_d\" || continue\n\
           test -f \"$z_d/rbrv.env\" || continue\n\
           (\n\
             source \"$z_d/rbrv.env\"\n\
             zrbrv_kindle\n\
             zrbrv_enforce\n\
           )\n\
         done",
        buv.display(),
        rbk.display(), rbk.display(), rbk.display(), rbk.display(),
    );

    match rbtdrf_run_bash(&root, &script, dir, "rbrv-all-vessels") {
        Ok((0, _, _)) => rbtdre_Verdict::Pass,
        Ok((code, _, _)) => {
            rbtdre_Verdict::Fail(format!("vessel validation failed (exit {})", code))
        }
        Err(e) => rbtdre_Verdict::Fail(e),
    }
}

fn rbtdrf_rv_rbrn_all_nameplates(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let buv = root.join("Tools/buk/buv_validation.sh");
    let rbk = root.join("Tools/rbk");

    // Script: source modules, iterate nameplates, kindle+enforce each
    let script = format!(
        "set -euo pipefail\n\
         source '{}'\n\
         source '{}/rbcc_Constants.sh'\n\
         source '{}/rbrn_regime.sh'\n\
         zbuv_kindle\nzrbcc_kindle\n\
         for z_f in \"${{RBBC_dot_dir}}\"/*/${{RBCC_rbrn_file}}; do\n\
           test -f \"$z_f\" || continue\n\
           (\n\
             source \"$z_f\"\n\
             zrbrn_kindle\n\
             zrbrn_enforce\n\
           )\n\
         done",
        buv.display(),
        rbk.display(), rbk.display(),
    );

    match rbtdrf_run_bash(&root, &script, dir, "rbrn-all-nameplates") {
        Ok((0, _, _)) => rbtdre_Verdict::Pass,
        Ok((code, _, _)) => {
            rbtdre_Verdict::Fail(format!("nameplate validation failed (exit {})", code))
        }
        Err(e) => rbtdre_Verdict::Fail(e),
    }
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

fn rbtdrf_rs_rbrn(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };

    // Discover nameplates by listing .rbk/*/rbrn.env
    let rbk_dir = root.join(".rbk");
    let entries = match std::fs::read_dir(&rbk_dir) {
        Ok(e) => e,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot read .rbk: {}", e)),
    };

    let mut found = false;
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() && path.join("rbrn.env").exists() {
            found = true;
            let moniker = entry.file_name().to_string_lossy().to_string();
            if let Err(e) = rbtdrf_run_tt(
                &root, "rbw-rnr", &[&moniker], dir,
                &format!("rbrn-{}-render", moniker),
            ) {
                return rbtdre_Verdict::Fail(e);
            }
            if let Err(e) = rbtdrf_run_tt(
                &root, "rbw-rnv", &[&moniker], dir,
                &format!("rbrn-{}-validate", moniker),
            ) {
                return rbtdre_Verdict::Fail(e);
            }
        }
    }

    if !found {
        return rbtdre_Verdict::Fail("no nameplates found in .rbk/".to_string());
    }
    rbtdre_Verdict::Pass
}

fn rbtdrf_rs_rbrr(dir: &Path) -> rbtdre_Verdict {
    rbtdrf_rs_render_validate(dir, "rbw-rrr", "rbw-rrv", "rbrr")
}

fn rbtdrf_rs_rbrr_nonempty_prefix(dir: &Path) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let buv = root.join("Tools/buk/buv_validation.sh");
    let rbk = root.join("Tools/rbk");

    let script = format!(
        "set -euo pipefail\n\
         source '{}'\n\
         source '{}/rbcc_Constants.sh'\n\
         source '{}/rbgc_Constants.sh'\n\
         source '{}/rbrr_regime.sh'\n\
         source '{}/rbgl_GarLayout.sh'\n\
         zbuv_kindle\nzrbcc_kindle\nzrbgc_kindle\n\
         source \"${{PWD}}/.rbk/rbrr.env\"\n\
         RBRR_CLOUD_PREFIX=\"acme-\"\n\
         RBRR_RUNTIME_PREFIX=\"acme-\"\n\
         zrbrr_kindle\nzrbrr_enforce\nzrbgl_kindle\n\
         echo \"hallmarks_root=${{RBGL_HALLMARKS_ROOT}}\"\n\
         echo \"runtime_prefix=${{RBRR_RUNTIME_PREFIX}}\"",
        buv.display(),
        rbk.display(), rbk.display(), rbk.display(), rbk.display(),
    );

    match rbtdrf_run_bash(&root, &script, dir, "rbrr-nonempty-prefix") {
        Ok((0, stdout, _)) => {
            let hallmarks_ok = stdout
                .lines()
                .any(|l| l.starts_with("hallmarks_root=acme-hallmarks"));
            let runtime_ok = stdout
                .lines()
                .any(|l| l.trim() == "runtime_prefix=acme-");
            if !hallmarks_ok {
                return rbtdre_Verdict::Fail(format!(
                    "RBGL_HALLMARKS_ROOT did not propagate from prefix; stdout:\n{}",
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

    let script = format!(
        "set -euo pipefail\n\
         source '{}'\n\
         source '{}/rbcc_Constants.sh'\n\
         source '{}/rbrr_regime.sh'\n\
         source '{}/rbdc_DerivedConstants.sh'\n\
         zbuv_kindle\nzrbcc_kindle\n\
         source \"${{PWD}}/.rbk/rbrr.env\"\n\
         zrbrr_kindle\nzrbrr_enforce\nzrbdc_kindle\n\
         echo \"${{RBRR_VESSEL_DIR}}\"",
        buv.display(),
        rbk.display(), rbk.display(), rbk.display(),
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
                &root, "rbw-rvr", &[&sigil], dir,
                &format!("rbrv-{}-render", sigil),
            ) {
                return rbtdre_Verdict::Fail(e);
            }
            if let Err(e) = rbtdrf_run_tt(
                &root, "rbw-rvv", &[&sigil], dir,
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
    rbtdrf_rs_render_validate(dir, "rbw-rpr", "rbw-rpv", "rbrp")
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

// ── Section arrays ──────────────────────────────────────────

pub static RBTDRF_SECTIONS_ENROLLMENT_VALIDATION: &[rbtdre_Section] = &[
    rbtdre_Section {
        name: "ev-length-types",
        cases: &[
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
        ],
    },
    rbtdre_Section {
        name: "ev-choice-types",
        cases: &[
            case!(rbtdrf_ev_bool_valid),
            case!(rbtdrf_ev_bool_invalid),
            case!(rbtdrf_ev_bool_empty),
            case!(rbtdrf_ev_enum_valid),
            case!(rbtdrf_ev_enum_invalid),
            case!(rbtdrf_ev_enum_empty),
        ],
    },
    rbtdre_Section {
        name: "ev-numeric-types",
        cases: &[
            case!(rbtdrf_ev_decimal_valid),
            case!(rbtdrf_ev_decimal_below),
            case!(rbtdrf_ev_decimal_above),
            case!(rbtdrf_ev_decimal_empty),
            case!(rbtdrf_ev_ipv4_valid),
            case!(rbtdrf_ev_ipv4_invalid),
            case!(rbtdrf_ev_port_valid),
            case!(rbtdrf_ev_port_invalid),
        ],
    },
    rbtdre_Section {
        name: "ev-reference-types",
        cases: &[
            case!(rbtdrf_ev_odref_valid),
            case!(rbtdrf_ev_odref_no_digest),
            case!(rbtdrf_ev_odref_malformed),
            case!(rbtdrf_ev_odref_empty),
        ],
    },
    rbtdre_Section {
        name: "ev-list-types",
        cases: &[
            case!(rbtdrf_ev_list_string_valid),
            case!(rbtdrf_ev_list_string_empty),
            case!(rbtdrf_ev_list_string_bad_item),
            case!(rbtdrf_ev_list_ipv4_valid),
            case!(rbtdrf_ev_list_ipv4_invalid),
            case!(rbtdrf_ev_list_ipv4_empty),
            case!(rbtdrf_ev_list_gname_valid),
            case!(rbtdrf_ev_list_gname_invalid),
        ],
    },
    rbtdre_Section {
        name: "ev-gating",
        cases: &[
            case!(rbtdrf_ev_gate_active_valid),
            case!(rbtdrf_ev_gate_active_invalid),
            case!(rbtdrf_ev_gate_inactive),
            case!(rbtdrf_ev_gate_multi),
        ],
    },
    rbtdre_Section {
        name: "ev-enforce-report",
        cases: &[
            case!(rbtdrf_ev_enforce_all_pass),
            case!(rbtdrf_ev_enforce_first_bad),
            case!(rbtdrf_ev_report_all_pass),
            case!(rbtdrf_ev_report_mixed),
            case!(rbtdrf_ev_report_gated),
            case!(rbtdrf_ev_multiscope),
        ],
    },
];

pub static RBTDRF_SECTIONS_REGIME_VALIDATION: &[rbtdre_Section] = &[
    rbtdre_Section {
        name: "rv-rbrr-negative",
        cases: &[
            case!(rbtdrf_rv_rbrr_missing_project_id),
            case!(rbtdrf_rv_rbrr_bad_timeout),
            case!(rbtdrf_rv_rbrr_bad_pool_stem),
            case!(rbtdrf_rv_rbrr_unexpected_var),
            case!(rbtdrf_rv_rbrr_bad_vessel_dir),
            case!(rbtdrf_rv_rbrr_bad_secrets_dir),
            case!(rbtdrf_rv_rbrr_bad_cloud_prefix),
            case!(rbtdrf_rv_rbrr_bad_runtime_prefix),
        ],
    },
    rbtdre_Section {
        name: "rv-rbrv-negative",
        cases: &[
            case!(rbtdrf_rv_rbrv_missing_sigil),
            case!(rbtdrf_rv_rbrv_no_bind_image),
            case!(rbtdrf_rv_rbrv_unexpected_var),
            case!(rbtdrf_rv_rbrv_partial_conjure),
        ],
    },
    rbtdre_Section {
        name: "rv-rbrn-negative",
        cases: &[
            case!(rbtdrf_rv_rbrn_missing_moniker),
            case!(rbtdrf_rv_rbrn_invalid_runtime),
            case!(rbtdrf_rv_rbrn_invalid_entry_mode),
            case!(rbtdrf_rv_rbrn_invalid_dns_mode),
            case!(rbtdrf_rv_rbrn_invalid_access_mode),
            case!(rbtdrf_rv_rbrn_port_conflict),
            case!(rbtdrf_rv_rbrn_unexpected_var),
            case!(rbtdrf_rv_rbrn_bad_ip),
        ],
    },
    rbtdre_Section {
        name: "rv-positive",
        cases: &[
            case!(rbtdrf_rv_rbrr_repo),
            case!(rbtdrf_rv_rbrv_all_vessels),
            case!(rbtdrf_rv_rbrn_all_nameplates),
        ],
    },
];

pub static RBTDRF_SECTIONS_REGIME_SMOKE: &[rbtdre_Section] = &[rbtdre_Section {
    name: "regime-smoke",
    cases: &[
        case!(rbtdrf_rs_burc),
        case!(rbtdrf_rs_burs),
        case!(rbtdrf_rs_rbrn),
        case!(rbtdrf_rs_rbrr),
        case!(rbtdrf_rs_rbrr_nonempty_prefix),
        case!(rbtdrf_rs_rbrv),
        case!(rbtdrf_rs_rbrp),
        case!(rbtdrf_rs_burd),
    ],
}];
