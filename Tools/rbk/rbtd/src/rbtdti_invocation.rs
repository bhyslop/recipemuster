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
// RBTDTI — tests for tabtarget invocation layer

use std::path::PathBuf;

use super::rbtdre_engine::rbtdre_Verdict;
use super::rbtdri_invocation::*;
use super::rbtdrm_manifest::RBTDRM_COLOPHON_ORDAIN;

fn rbtdti_make_temp(label: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rbtd-test-{}-{}", std::process::id(), label));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn rbtdti_make_tt_dir(root: &PathBuf) -> PathBuf {
    let tt = root.join("tt");
    std::fs::create_dir_all(&tt).unwrap();
    tt
}

fn rbtdti_write_script(tt_dir: &PathBuf, name: &str, body: &str) {
    let path = tt_dir.join(name);
    std::fs::write(&path, format!("#!/bin/bash\n{}", body)).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }
}

// ── Tabtarget discovery ──────────────────────────────────────

#[test]
fn rbtdti_finds_matching_tabtarget() {
    let tmp = rbtdti_make_temp("find-match");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbw-cb.Bark.tadmor.sh", "exit 0\n");

    let result = rbtdri_find_tabtarget(&tmp, "rbw-cb", "tadmor");
    assert!(result.is_ok());
    let path = result.unwrap();
    assert!(path.file_name().unwrap().to_str().unwrap() == "rbw-cb.Bark.tadmor.sh");

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_find_rejects_no_match() {
    let tmp = rbtdti_make_temp("find-nomatch");
    let _tt = rbtdti_make_tt_dir(&tmp);

    let result = rbtdri_find_tabtarget(&tmp, "rbw-cb", "tadmor");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("no tabtarget"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_find_rejects_multiple_matches() {
    let tmp = rbtdti_make_temp("find-multi");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbw-cb.Bark.tadmor.sh", "exit 0\n");
    rbtdti_write_script(&tt, "rbw-cb.AlsoBark.tadmor.sh", "exit 0\n");

    let result = rbtdri_find_tabtarget(&tmp, "rbw-cb", "tadmor");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("2 tabtargets"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_find_does_not_match_wrong_nameplate() {
    let tmp = rbtdti_make_temp("find-wrongnp");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbw-cb.Bark.tadmor.sh", "exit 0\n");

    let result = rbtdri_find_tabtarget(&tmp, "rbw-cb", "srjcl");
    assert!(result.is_err());

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_find_does_not_match_wrong_colophon() {
    let tmp = rbtdti_make_temp("find-wrongcol");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbw-cb.Bark.tadmor.sh", "exit 0\n");

    let result = rbtdri_find_tabtarget(&tmp, "rbw-cw", "tadmor");
    assert!(result.is_err());

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_find_no_partial_colophon_match() {
    let tmp = rbtdti_make_temp("find-partial");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbw-cbb.Bark.tadmor.sh", "exit 0\n");

    let result = rbtdri_find_tabtarget(&tmp, "rbw-cb", "tadmor");
    assert!(result.is_err(), "rbw-cbb should not match rbw-cb");

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Ifrit verdict parsing ────────────────────────────────────

#[test]
fn rbtdti_parse_ifrit_pass() {
    let verdict = rbtdri_parse_ifrit_verdict("IFRIT_VERDICT: PASS\n", 0);
    assert!(matches!(verdict, rbtdre_Verdict::Pass));
}

#[test]
fn rbtdti_parse_ifrit_fail_with_detail() {
    let verdict = rbtdri_parse_ifrit_verdict("IFRIT_VERDICT: FAIL dns leak detected\n", 1);
    match verdict {
        rbtdre_Verdict::Fail(detail) => assert!(detail.contains("dns leak detected")),
        _ => panic!("expected Fail verdict"),
    }
}

#[test]
fn rbtdti_parse_ifrit_fail_bare() {
    let verdict = rbtdri_parse_ifrit_verdict("IFRIT_VERDICT: FAIL\n", 1);
    match verdict {
        rbtdre_Verdict::Fail(detail) => assert!(detail.contains("ifrit reported failure")),
        _ => panic!("expected Fail verdict"),
    }
}

#[test]
fn rbtdti_parse_ifrit_no_verdict_nonzero_exit() {
    let verdict = rbtdri_parse_ifrit_verdict("some other output\n", 42);
    match verdict {
        rbtdre_Verdict::Fail(detail) => {
            assert!(detail.contains("42"));
            assert!(detail.contains("no verdict line"));
        }
        _ => panic!("expected Fail verdict"),
    }
}

#[test]
fn rbtdti_parse_ifrit_no_verdict_zero_exit() {
    let verdict = rbtdri_parse_ifrit_verdict("some output\n", 0);
    match verdict {
        rbtdre_Verdict::Fail(detail) => assert!(detail.contains("no verdict line")),
        _ => panic!("expected Fail verdict"),
    }
}

#[test]
fn rbtdti_parse_ifrit_verdict_among_other_lines() {
    let stdout = "ifrit v0.1.0 starting\nprobing dns...\nIFRIT_VERDICT: PASS\ncleaning up\n";
    let verdict = rbtdri_parse_ifrit_verdict(stdout, 0);
    assert!(matches!(verdict, rbtdre_Verdict::Pass));
}

#[test]
fn rbtdti_parse_ifrit_empty_stdout() {
    let verdict = rbtdri_parse_ifrit_verdict("", 1);
    match verdict {
        rbtdre_Verdict::Fail(detail) => assert!(detail.contains("no verdict line")),
        _ => panic!("expected Fail verdict"),
    }
}

// ── Invocation with BURV isolation ───────────────────────────

#[test]
fn rbtdti_invoke_creates_burv_dirs() {
    let tmp = rbtdti_make_temp("invoke-burv");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbw-cb.Bark.testplate.sh", "exit 0\n");

    let burv_root = tmp.join("burv");
    let mut ctx = rbtdri_Context::new(&tmp, "testplate", &burv_root);
    let result = rbtdri_invoke(&mut ctx, "rbw-cb", &[]);

    assert!(result.is_ok());
    assert!(burv_root.join("invoke-00000").join("output").is_dir());
    assert!(burv_root.join("invoke-00000").join("temp").is_dir());
    assert_eq!(ctx.invoke_count, 1);

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_invoke_sequential_burv_isolation() {
    let tmp = rbtdti_make_temp("invoke-seq");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbw-cb.Bark.testplate.sh", "exit 0\n");

    let burv_root = tmp.join("burv");
    let mut ctx = rbtdri_Context::new(&tmp, "testplate", &burv_root);

    let _ = rbtdri_invoke(&mut ctx, "rbw-cb", &[]).unwrap();
    let _ = rbtdri_invoke(&mut ctx, "rbw-cb", &[]).unwrap();

    assert_eq!(ctx.invoke_count, 2);
    assert!(burv_root.join("invoke-00000").join("output").is_dir());
    assert!(burv_root.join("invoke-00001").join("output").is_dir());
    // Dirs are distinct
    assert_ne!(
        burv_root.join("invoke-00000"),
        burv_root.join("invoke-00001"),
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_invoke_captures_stdout() {
    let tmp = rbtdti_make_temp("invoke-stdout");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbw-cb.Bark.testplate.sh", "echo 'hello stdout'\n");

    let burv_root = tmp.join("burv");
    let mut ctx = rbtdri_Context::new(&tmp, "testplate", &burv_root);
    let result = rbtdri_invoke(&mut ctx, "rbw-cb", &[]).unwrap();

    assert!(result.stdout.contains("hello stdout"));
    assert_eq!(result.exit_code, 0);

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_invoke_captures_stderr() {
    let tmp = rbtdti_make_temp("invoke-stderr");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbw-cb.Bark.testplate.sh", "echo 'hello stderr' >&2\n");

    let burv_root = tmp.join("burv");
    let mut ctx = rbtdri_Context::new(&tmp, "testplate", &burv_root);
    let result = rbtdri_invoke(&mut ctx, "rbw-cb", &[]).unwrap();

    assert!(result.stderr.contains("hello stderr"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_invoke_captures_nonzero_exit() {
    let tmp = rbtdti_make_temp("invoke-exit");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbw-cb.Bark.testplate.sh", "exit 7\n");

    let burv_root = tmp.join("burv");
    let mut ctx = rbtdri_Context::new(&tmp, "testplate", &burv_root);
    let result = rbtdri_invoke(&mut ctx, "rbw-cb", &[]).unwrap();

    assert_eq!(result.exit_code, 7);

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_invoke_passes_args() {
    let tmp = rbtdti_make_temp("invoke-args");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbw-cb.Bark.testplate.sh", "echo \"args: $*\"\n");

    let burv_root = tmp.join("burv");
    let mut ctx = rbtdri_Context::new(&tmp, "testplate", &burv_root);
    let result = rbtdri_invoke(&mut ctx, "rbw-cb", &["alpha", "bravo"]).unwrap();

    assert!(result.stdout.contains("args: alpha bravo"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_invoke_sets_burv_env_vars() {
    let tmp = rbtdti_make_temp("invoke-env");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(
        &tt,
        "rbw-cb.Bark.testplate.sh",
        "echo \"OUT:${BURV_OUTPUT_ROOT_DIR}\"\necho \"TMP:${BURV_TEMP_ROOT_DIR}\"\n",
    );

    let burv_root = tmp.join("burv");
    let mut ctx = rbtdri_Context::new(&tmp, "testplate", &burv_root);
    let result = rbtdri_invoke(&mut ctx, "rbw-cb", &[]).unwrap();

    let expected_output = burv_root.join("invoke-00000").join("output");
    let expected_temp = burv_root.join("invoke-00000").join("temp");
    assert!(result.stdout.contains(&format!("OUT:{}", expected_output.display())));
    assert!(result.stdout.contains(&format!("TMP:{}", expected_temp.display())));

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── BURV output path in result ───────────────────────────────

#[test]
fn rbtdti_invoke_returns_burv_output_path() {
    let tmp = rbtdti_make_temp("invoke-burvpath");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbw-cb.Bark.testplate.sh", "exit 0\n");

    let burv_root = tmp.join("burv");
    let mut ctx = rbtdri_Context::new(&tmp, "testplate", &burv_root);
    let result = rbtdri_invoke(&mut ctx, "rbw-cb", &[]).unwrap();

    assert_eq!(result.burv_output, burv_root.join("invoke-00000").join("output"));
    assert!(result.burv_output.is_dir());

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Global tabtarget discovery ──────────────────────────────

#[test]
fn rbtdti_find_global_matches() {
    let tmp = rbtdti_make_temp("global-match");
    let tt = rbtdti_make_tt_dir(&tmp);
    let script_name = format!("{}.DirectorOrdains.sh", RBTDRM_COLOPHON_ORDAIN);
    rbtdti_write_script(&tt, &script_name, "exit 0\n");

    let result = rbtdri_find_tabtarget_global(&tmp, RBTDRM_COLOPHON_ORDAIN);
    assert!(result.is_ok());
    assert!(result.unwrap().file_name().unwrap().to_str().unwrap() == script_name);

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_find_global_rejects_imprint_suffix() {
    let tmp = rbtdti_make_temp("global-imprint");
    let tt = rbtdti_make_tt_dir(&tmp);
    // Only an imprint-scoped tabtarget — global discovery should not find it
    let script_name = format!("{}.DirectorOrdains.tadmor.sh", RBTDRM_COLOPHON_ORDAIN);
    rbtdti_write_script(&tt, &script_name, "exit 0\n");

    let result = rbtdri_find_tabtarget_global(&tmp, RBTDRM_COLOPHON_ORDAIN);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("no global tabtarget"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_find_global_rejects_no_match() {
    let tmp = rbtdti_make_temp("global-nomatch");
    let _tt = rbtdti_make_tt_dir(&tmp);

    let result = rbtdri_find_tabtarget_global(&tmp, RBTDRM_COLOPHON_ORDAIN);
    assert!(result.is_err());

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Invoke global ───────────────────────────────────────────

#[test]
fn rbtdti_invoke_global_passes_extra_env() {
    let tmp = rbtdti_make_temp("invoke-global-env");
    let tt = rbtdti_make_tt_dir(&tmp);
    let script_name = format!("{}.DirectorOrdains.sh", RBTDRM_COLOPHON_ORDAIN);
    rbtdti_write_script(
        &tt,
        &script_name,
        "echo \"TWEAK:${BURE_TWEAK_NAME}\"\n",
    );

    let burv_root = tmp.join("burv");
    let mut ctx = rbtdri_Context::new(&tmp, "testplate", &burv_root);
    let result = rbtdri_invoke_global(
        &mut ctx,
        RBTDRM_COLOPHON_ORDAIN,
        &[],
        &[("BURE_TWEAK_NAME", "threemodegraft")],
    )
    .unwrap();

    assert!(result.stdout.contains("TWEAK:threemodegraft"));

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Invoke with explicit imprint ────────────────────────────

#[test]
fn rbtdti_invoke_imprint_finds_correct_target() {
    let tmp = rbtdti_make_temp("invoke-imprint");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbtd-ap.AccessProbe.governor.sh", "echo 'governor'\n");
    rbtdti_write_script(&tt, "rbtd-ap.AccessProbe.director.sh", "echo 'director'\n");

    let burv_root = tmp.join("burv");
    let mut ctx = rbtdri_Context::new(&tmp, "testplate", &burv_root);
    let result = rbtdri_invoke_imprint(&mut ctx, "rbtd-ap", "governor", &[]).unwrap();

    assert!(result.stdout.contains("governor"));

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── BURV fact file reading ──────────────────────────────────

#[test]
fn rbtdti_read_burv_fact_reads_value() {
    let tmp = rbtdti_make_temp("burv-fact");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(
        &tt,
        "rbw-cb.Bark.testplate.sh",
        "mkdir -p \"${BURV_OUTPUT_ROOT_DIR}/current\"\necho 'c260305-r260305' > \"${BURV_OUTPUT_ROOT_DIR}/current/rbf_fact_hallmark\"\n",
    );

    let burv_root = tmp.join("burv");
    let mut ctx = rbtdri_Context::new(&tmp, "testplate", &burv_root);
    let result = rbtdri_invoke(&mut ctx, "rbw-cb", &[]).unwrap();

    let fact = rbtdri_read_burv_fact(&result, "rbf_fact_hallmark").unwrap();
    assert_eq!(fact, "c260305-r260305");

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbtdti_read_burv_fact_rejects_missing() {
    let tmp = rbtdti_make_temp("burv-fact-missing");
    let tt = rbtdti_make_tt_dir(&tmp);
    rbtdti_write_script(&tt, "rbw-cb.Bark.testplate.sh", "exit 0\n");

    let burv_root = tmp.join("burv");
    let mut ctx = rbtdri_Context::new(&tmp, "testplate", &burv_root);
    let result = rbtdri_invoke(&mut ctx, "rbw-cb", &[]).unwrap();

    let fact = rbtdri_read_burv_fact(&result, "rbf_fact_hallmark");
    assert!(fact.is_err());

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Invoke error cases ───────────────────────────────────────

#[test]
fn rbtdti_invoke_fails_no_tabtarget() {
    let tmp = rbtdti_make_temp("invoke-notarget");
    let _tt = rbtdti_make_tt_dir(&tmp);

    let burv_root = tmp.join("burv");
    let mut ctx = rbtdri_Context::new(&tmp, "testplate", &burv_root);
    let result = rbtdri_invoke(&mut ctx, "rbw-cb", &[]);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("no tabtarget"));
    // BURV dirs should NOT have been created since discovery failed first
    assert!(!burv_root.join("invoke-00000").exists());

    let _ = std::fs::remove_dir_all(&tmp);
}
