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
// RBTDTI — tests for tabtarget invocation layer

use std::path::PathBuf;

use super::rbtcee_engine::rbtcee_Verdict;
use super::rbtcei_invocation::*;
use super::rbtdgc_consts::{RBTDGC_CRUCIBLE_BARK, RBTDGC_CRUCIBLE_CHARGE, RBTDGC_CRUCIBLE_WRIT, RBTDGC_ORDAIN_HALLMARK, RBTDGC_TWEAK_CREDLESS_GUARD};
use super::rbtcem_manifest::{RBTCEM_FIXTURE_SRJCL, RBTCEM_FIXTURE_TADMOR};
use super::rbtteh_helpers::rbtteh_make_scratch;

fn rbttei_make_tt_dir(root: &PathBuf) -> PathBuf {
    let tt = root.join("tt");
    std::fs::create_dir_all(&tt).unwrap();
    tt
}

fn rbttei_write_script(tt_dir: &PathBuf, name: &str, body: &str) {
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
fn rbttei_finds_matching_tabtarget() {
    let tmp = rbtteh_make_scratch("find-match");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.tadmor.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let result = rbtcei_find_tabtarget(&tmp, RBTDGC_CRUCIBLE_BARK, "tadmor");
    assert!(result.is_ok());
    let path = result.unwrap();
    assert!(path.file_name().unwrap().to_str().unwrap() == &format!("{}.Bark.tadmor.sh", RBTDGC_CRUCIBLE_BARK));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_find_rejects_no_match() {
    let tmp = rbtteh_make_scratch("find-nomatch");
    let _tt = rbttei_make_tt_dir(&tmp);

    let result = rbtcei_find_tabtarget(&tmp, RBTDGC_CRUCIBLE_BARK, "tadmor");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("no tabtarget"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_find_rejects_multiple_matches() {
    let tmp = rbtteh_make_scratch("find-multi");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.tadmor.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");
    rbttei_write_script(&tt, &format!("{}.AlsoBark.tadmor.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let result = rbtcei_find_tabtarget(&tmp, RBTDGC_CRUCIBLE_BARK, "tadmor");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("2 tabtargets"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_find_does_not_match_wrong_nameplate() {
    let tmp = rbtteh_make_scratch("find-wrongnp");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.tadmor.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let result = rbtcei_find_tabtarget(&tmp, RBTDGC_CRUCIBLE_BARK, "srjcl");
    assert!(result.is_err());

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_find_does_not_match_wrong_colophon() {
    let tmp = rbtteh_make_scratch("find-wrongcol");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.tadmor.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let result = rbtcei_find_tabtarget(&tmp, RBTDGC_CRUCIBLE_WRIT, "tadmor");
    assert!(result.is_err());

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_find_no_partial_colophon_match() {
    let tmp = rbtteh_make_scratch("find-partial");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}b.Bark.tadmor.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let result = rbtcei_find_tabtarget(&tmp, RBTDGC_CRUCIBLE_BARK, "tadmor");
    assert!(
        result.is_err(),
        "{}b should not match {}",
        RBTDGC_CRUCIBLE_BARK,
        RBTDGC_CRUCIBLE_BARK
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Ifrit verdict parsing ────────────────────────────────────

#[test]
fn rbttei_parse_ifrit_pass() {
    let verdict = rbtcei_parse_ifrit_verdict("IFRIT_VERDICT: PASS\n", 0);
    assert!(matches!(verdict, rbtcee_Verdict::Pass));
}

#[test]
fn rbttei_parse_ifrit_fail_with_detail() {
    let verdict = rbtcei_parse_ifrit_verdict("IFRIT_VERDICT: FAIL dns leak detected\n", 1);
    match verdict {
        rbtcee_Verdict::Fail(detail) => assert!(detail.contains("dns leak detected")),
        _ => panic!("expected Fail verdict"),
    }
}

#[test]
fn rbttei_parse_ifrit_fail_bare() {
    let verdict = rbtcei_parse_ifrit_verdict("IFRIT_VERDICT: FAIL\n", 1);
    match verdict {
        rbtcee_Verdict::Fail(detail) => assert!(detail.contains("ifrit reported failure")),
        _ => panic!("expected Fail verdict"),
    }
}

#[test]
fn rbttei_parse_ifrit_no_verdict_nonzero_exit() {
    let verdict = rbtcei_parse_ifrit_verdict("some other output\n", 42);
    match verdict {
        rbtcee_Verdict::Fail(detail) => {
            assert!(detail.contains("42"));
            assert!(detail.contains("no verdict line"));
        }
        _ => panic!("expected Fail verdict"),
    }
}

#[test]
fn rbttei_parse_ifrit_no_verdict_zero_exit() {
    let verdict = rbtcei_parse_ifrit_verdict("some output\n", 0);
    match verdict {
        rbtcee_Verdict::Fail(detail) => assert!(detail.contains("no verdict line")),
        _ => panic!("expected Fail verdict"),
    }
}

#[test]
fn rbttei_parse_ifrit_verdict_among_other_lines() {
    let stdout = "ifrit v0.1.0 starting\nprobing dns...\nIFRIT_VERDICT: PASS\ncleaning up\n";
    let verdict = rbtcei_parse_ifrit_verdict(stdout, 0);
    assert!(matches!(verdict, rbtcee_Verdict::Pass));
}

#[test]
fn rbttei_parse_ifrit_empty_stdout() {
    let verdict = rbtcei_parse_ifrit_verdict("", 1);
    match verdict {
        rbtcee_Verdict::Fail(detail) => assert!(detail.contains("no verdict line")),
        _ => panic!("expected Fail verdict"),
    }
}

// ── Invocation with BURV isolation ───────────────────────────

#[test]
fn rbttei_invoke_creates_burv_dirs() {
    let tmp = rbtteh_make_scratch("invoke-burv");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]);

    assert!(result.is_ok());
    assert!(burv_output_root.join(rbtcei_invoke_dir_name(0)).is_dir());
    assert!(burv_temp_root.join(rbtcei_invoke_dir_name(0)).is_dir());
    assert_eq!(ctx.invoke_count, 1);

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_invoke_sequential_burv_isolation() {
    let tmp = rbtteh_make_scratch("invoke-seq");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);

    let _ = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]).unwrap();
    let _ = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]).unwrap();

    assert_eq!(ctx.invoke_count, 2);
    assert!(burv_output_root.join(rbtcei_invoke_dir_name(0)).is_dir());
    assert!(burv_output_root.join(rbtcei_invoke_dir_name(1)).is_dir());
    assert_ne!(
        burv_output_root.join(rbtcei_invoke_dir_name(0)),
        burv_output_root.join(rbtcei_invoke_dir_name(1)),
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_chain_next_reuses_prior_burv_root() {
    let tmp = rbtteh_make_scratch("invoke-chain");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);

    // First invoke takes a fresh root (invoke-00000) and advances the counter.
    let first = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]).unwrap();
    assert_eq!(ctx.invoke_count, 1);

    // A chained invoke reuses the prior invoke's root without advancing the
    // counter — the theurge-side condition that lets bud_dispatch promote the
    // prior invoke's current/ into this invoke's previous/.
    ctx.rbtcei_chain_next_invoke();
    let chained = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]).unwrap();
    assert_eq!(chained.burv_output, first.burv_output);
    assert_eq!(chained.burv_output, burv_output_root.join(rbtcei_invoke_dir_name(0)));
    assert_eq!(ctx.invoke_count, 1, "chained invoke must not advance the counter");

    // Isolation is restored for the next unmarked invoke — it takes a fresh root.
    let third = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]).unwrap();
    assert_eq!(third.burv_output, burv_output_root.join(rbtcei_invoke_dir_name(1)));
    assert_ne!(third.burv_output, first.burv_output);
    assert_eq!(ctx.invoke_count, 2);

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_chain_next_without_prior_invoke_errs() {
    let tmp = rbtteh_make_scratch("invoke-chain-noprior");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);

    // chain_next with no prior invoke has nothing to chain from — it must error
    // loud rather than silently reuse a nonexistent root.
    ctx.rbtcei_chain_next_invoke();
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]);
    assert!(result.is_err());

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_invoke_captures_stdout() {
    let tmp = rbtteh_make_scratch("invoke-stdout");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "echo 'hello stdout'\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]).unwrap();

    assert!(result.stdout.contains("hello stdout"));
    assert_eq!(result.exit_code, 0);

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_invoke_captures_stderr() {
    let tmp = rbtteh_make_scratch("invoke-stderr");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "echo 'hello stderr' >&2\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]).unwrap();

    assert!(result.stderr.contains("hello stderr"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_invoke_captures_nonzero_exit() {
    let tmp = rbtteh_make_scratch("invoke-exit");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "exit 7\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]).unwrap();

    assert_eq!(result.exit_code, 7);

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_invoke_passes_args() {
    let tmp = rbtteh_make_scratch("invoke-args");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "echo \"args: $*\"\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &["alpha", "bravo"]).unwrap();

    assert!(result.stdout.contains("args: alpha bravo"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_invoke_sets_burv_env_vars() {
    let tmp = rbtteh_make_scratch("invoke-env");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(
        &tt,
        &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK),
        "echo \"OUT:${BURV_OUTPUT_ROOT_DIR}\"\necho \"TMP:${BURV_TEMP_ROOT_DIR}\"\n",
    );

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]).unwrap();

    let expected_output = burv_output_root.join(rbtcei_invoke_dir_name(0));
    let expected_temp = burv_temp_root.join(rbtcei_invoke_dir_name(0));
    assert!(result.stdout.contains(&format!("OUT:{}", expected_output.display())));
    assert!(result.stdout.contains(&format!("TMP:{}", expected_temp.display())));

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── BURV output path in result ───────────────────────────────

#[test]
fn rbttei_invoke_returns_burv_output_path() {
    let tmp = rbtteh_make_scratch("invoke-burvpath");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]).unwrap();

    assert_eq!(result.burv_output, burv_output_root.join(rbtcei_invoke_dir_name(0)));
    assert!(result.burv_output.is_dir());

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Global tabtarget discovery ──────────────────────────────

#[test]
fn rbttei_find_global_matches() {
    let tmp = rbtteh_make_scratch("global-match");
    let tt = rbttei_make_tt_dir(&tmp);
    let script_name = format!("{}.DirectorOrdains.sh", RBTDGC_ORDAIN_HALLMARK);
    rbttei_write_script(&tt, &script_name, "exit 0\n");

    let result = rbtcei_find_tabtarget_global(&tmp, RBTDGC_ORDAIN_HALLMARK);
    assert!(result.is_ok());
    assert!(result.unwrap().file_name().unwrap().to_str().unwrap() == script_name);

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_find_global_rejects_imprint_suffix() {
    let tmp = rbtteh_make_scratch("global-imprint");
    let tt = rbttei_make_tt_dir(&tmp);
    // Only an imprint-scoped tabtarget — global discovery should not find it
    let script_name = format!("{}.DirectorOrdains.tadmor.sh", RBTDGC_ORDAIN_HALLMARK);
    rbttei_write_script(&tt, &script_name, "exit 0\n");

    let result = rbtcei_find_tabtarget_global(&tmp, RBTDGC_ORDAIN_HALLMARK);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("no global tabtarget"));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_find_global_rejects_no_match() {
    let tmp = rbtteh_make_scratch("global-nomatch");
    let _tt = rbttei_make_tt_dir(&tmp);

    let result = rbtcei_find_tabtarget_global(&tmp, RBTDGC_ORDAIN_HALLMARK);
    assert!(result.is_err());

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Invoke global ───────────────────────────────────────────

#[test]
fn rbttei_invoke_global_passes_extra_env() {
    let tmp = rbtteh_make_scratch("invoke-global-env");
    let tt = rbttei_make_tt_dir(&tmp);
    let script_name = format!("{}.DirectorOrdains.sh", RBTDGC_ORDAIN_HALLMARK);
    rbttei_write_script(
        &tt,
        &script_name,
        "echo \"TWEAK:${BURE_TWEAK_NAME}\"\n",
    );

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);
    let result = rbtcei_invoke_global(
        &mut ctx,
        RBTDGC_ORDAIN_HALLMARK,
        &[],
        &[("BURE_TWEAK_NAME", "buost_example")],
    )
    .unwrap();

    assert!(result.stdout.contains("TWEAK:buost_example"));

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Credless guard ──────────────────────────────────────────
//
// The guard flag is thread-local, so these tests cannot interfere with the
// other invocation tests running on parallel cargo-test threads. Each test
// disarms before asserting/returning to leave its thread clean.

#[test]
fn rbttei_credless_armed_applies_guard_env() {
    let tmp = rbtteh_make_scratch("credless-env");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(
        &tt,
        &format!("{}.DirectorOrdains.sh", RBTDGC_ORDAIN_HALLMARK),
        "echo \"TWEAK:${BURE_TWEAK_NAME}\"\n",
    );

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);

    rbtcei_arm_credless(true);
    let result = rbtcei_invoke_global(&mut ctx, RBTDGC_ORDAIN_HALLMARK, &[], &[]);
    rbtcei_arm_credless(false);

    let result = result.unwrap();
    assert!(
        result.stdout.contains(&format!("TWEAK:{}", RBTDGC_TWEAK_CREDLESS_GUARD)),
        "guard env missing from spawned tabtarget: {}",
        result.stdout
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_credless_armed_rejects_case_tweak() {
    let tmp = rbtteh_make_scratch("credless-conflict");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(
        &tt,
        &format!("{}.DirectorOrdains.sh", RBTDGC_ORDAIN_HALLMARK),
        "exit 0\n",
    );

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);

    rbtcei_arm_credless(true);
    let result = rbtcei_invoke_global(
        &mut ctx,
        RBTDGC_ORDAIN_HALLMARK,
        &[],
        &[(RBTCEI_BURE_TWEAK_NAME_KEY, "buost_example")],
    );
    rbtcei_arm_credless(false);

    let err = result.unwrap_err();
    assert!(
        err.contains("credless"),
        "conflict error should name the credless guard: {}",
        err
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_credless_disarmed_leaves_tweak_slot_free() {
    let tmp = rbtteh_make_scratch("credless-disarmed");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(
        &tt,
        &format!("{}.DirectorOrdains.sh", RBTDGC_ORDAIN_HALLMARK),
        "echo \"TWEAK:${BURE_TWEAK_NAME:-unset}\"\n",
    );

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);

    let result = rbtcei_invoke_global(&mut ctx, RBTDGC_ORDAIN_HALLMARK, &[], &[]).unwrap();
    assert!(
        result.stdout.contains("TWEAK:unset"),
        "disarmed invoke must not carry the guard: {}",
        result.stdout
    );

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Invoke with explicit imprint ────────────────────────────

#[test]
fn rbttei_invoke_imprint_finds_correct_target() {
    let tmp = rbtteh_make_scratch("invoke-imprint");
    let tt = rbttei_make_tt_dir(&tmp);
    let charge_tadmor = format!("{}.Charge.{}.sh", RBTDGC_CRUCIBLE_CHARGE, RBTCEM_FIXTURE_TADMOR);
    let charge_srjcl = format!("{}.Charge.{}.sh", RBTDGC_CRUCIBLE_CHARGE, RBTCEM_FIXTURE_SRJCL);
    rbttei_write_script(&tt, &charge_tadmor, &format!("echo '{}'\n", RBTCEM_FIXTURE_TADMOR));
    rbttei_write_script(&tt, &charge_srjcl, &format!("echo '{}'\n", RBTCEM_FIXTURE_SRJCL));

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);
    let result =
        rbtcei_invoke_imprint(&mut ctx, RBTDGC_CRUCIBLE_CHARGE, RBTCEM_FIXTURE_TADMOR, &[]).unwrap();

    assert!(result.stdout.contains(RBTCEM_FIXTURE_TADMOR));

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── BURV fact file reading ──────────────────────────────────

#[test]
fn rbttei_read_burv_fact_reads_value() {
    let tmp = rbtteh_make_scratch("burv-fact");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(
        &tt,
        &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK),
        "mkdir -p \"${BURV_OUTPUT_ROOT_DIR}/current\"\necho 'c260305-r260305' > \"${BURV_OUTPUT_ROOT_DIR}/current/rbgc_fact_hallmark\"\n",
    );

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]).unwrap();

    let fact = rbtcei_read_burv_fact(&result, "rbgc_fact_hallmark").unwrap();
    assert_eq!(fact, "c260305-r260305");

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_read_burv_fact_rejects_missing() {
    let tmp = rbtteh_make_scratch("burv-fact-missing");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]).unwrap();

    let fact = rbtcei_read_burv_fact(&result, "rbgc_fact_hallmark");
    assert!(fact.is_err());

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Colophon census ───────────────────────────────────────────
//
// Census state is thread-local, like the credless guard — see that section's
// comment above. Each test disarms (back to None, the disabled default) right
// after its invoke and before asserting/returning, so a worker-thread reuse
// by the test harness cannot leak an armed declared set into an unrelated
// test later in the file (every other test here never arms census at all and
// relies on the disabled default).

#[test]
fn rbttei_census_refuses_undeclared_colophon() {
    let tmp = rbtteh_make_scratch("census-refuse");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);

    // Declares ORDAIN_HALLMARK only — BARK is not in the declared set.
    rbtcei_census_arm(Some(&[RBTDGC_ORDAIN_HALLMARK]), &[]);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]);
    rbtcei_census_arm(None, &[]);

    let err = result.unwrap_err();
    assert!(err.contains(RBTDGC_CRUCIBLE_BARK), "error must name the offending colophon: {}", err);
    assert!(err.contains("testplate"), "error must name the fixture: {}", err);
    // Refused before any BURV isolation dirs are created — no partial state.
    assert!(!burv_output_root.join(rbtcei_invoke_dir_name(0)).exists());

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_census_allows_declared_colophon_and_records_usage() {
    let tmp = rbtteh_make_scratch("census-allow");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);

    rbtcei_census_arm(Some(&[RBTDGC_CRUCIBLE_BARK]), &[]);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]);
    let used = rbtcei_census_used();
    rbtcei_census_arm(None, &[]);

    assert!(result.is_ok());
    assert!(used.contains(RBTDGC_CRUCIBLE_BARK));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_census_allows_permitted_colophon_and_records_usage() {
    // Positive-only tier: a colophon absent from the REQUIRED set is still
    // admitted at the invoke chokepoint when it is PERMITTED — mirrors the
    // required-tier admit test above, but proves the second tier instead of
    // the first. Required is deliberately empty here, so if permitted were
    // not consulted this invoke would refuse exactly like
    // rbttei_census_refuses_undeclared_colophon above.
    let tmp = rbtteh_make_scratch("census-allow-permitted");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);

    rbtcei_census_arm(Some(&[]), &[RBTDGC_CRUCIBLE_BARK]);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]);
    let used = rbtcei_census_used();
    rbtcei_census_arm(None, &[]);

    assert!(result.is_ok(), "a permitted colophon must not be refused: {:?}", result.err());
    assert!(used.contains(RBTDGC_CRUCIBLE_BARK));

    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn rbttei_census_records_bypass_launches() {
    // Used-set recording lives at rbtcei_tabtarget_command — the universal
    // launch chokepoint — so a direct-Command bypass launch (discovery +
    // rbtcei_tabtarget_command, never rbtcei_invoke*) still satisfies the
    // negative census direction. The colophon is derived from the script
    // filename's leading dot-segment; building the Command records, no spawn
    // needed (same moment the tariff bumps).
    rbtcei_census_arm(Some(&[RBTDGC_CRUCIBLE_BARK]), &[]);
    let path = PathBuf::from(format!("tt/{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK));
    let _cmd = rbtcei_tabtarget_command(&path);
    let used = rbtcei_census_used();
    rbtcei_census_arm(None, &[]);

    assert!(
        used.contains(RBTDGC_CRUCIBLE_BARK),
        "bypass launch must record its colophon into the used-set: {:?}",
        used
    );
}

#[test]
fn rbttei_census_disabled_when_no_manifest_entry() {
    let tmp = rbtteh_make_scratch("census-disabled");
    let tt = rbttei_make_tt_dir(&tmp);
    rbttei_write_script(&tt, &format!("{}.Bark.testplate.sh", RBTDGC_CRUCIBLE_BARK), "exit 0\n");

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);

    // None (no manifest entry) disables census tracking entirely — an
    // undeclared-by-construction colophon still invokes cleanly. This is the
    // behavior every other test in this file relies on implicitly (an
    // unregistered "testplate" fixture name never arms census).
    rbtcei_census_arm(None, &[]);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]);
    let used = rbtcei_census_used();

    assert!(result.is_ok());
    assert!(used.is_empty());

    let _ = std::fs::remove_dir_all(&tmp);
}

// ── Invoke error cases ───────────────────────────────────────

#[test]
fn rbttei_invoke_fails_no_tabtarget() {
    let tmp = rbtteh_make_scratch("invoke-notarget");
    let _tt = rbttei_make_tt_dir(&tmp);

    let burv_temp_root = tmp.join("burv-temp");
    let burv_output_root = tmp.join("burv-output");
    let mut ctx = rbtcei_Context::new(&tmp, "testplate", &burv_temp_root, &burv_output_root);
    let result = rbtcei_invoke(&mut ctx, RBTDGC_CRUCIBLE_BARK, &[]);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("no tabtarget"));
    // BURV dirs should NOT have been created since discovery failed first
    assert!(!burv_output_root.join(rbtcei_invoke_dir_name(0)).exists());
    assert!(!burv_temp_root.join(rbtcei_invoke_dir_name(0)).exists());

    let _ = std::fs::remove_dir_all(&tmp);
}
