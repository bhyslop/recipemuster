// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]

//! Fundus scenario tests for remote dispatch via SSH.
//!
//! Localhost tests run on every `cargo test` — requires jjfu_* accounts
//! provisioned on localhost (see JJSTF-test-fundus.adoc).
//!
//! Cerebro tests use `#[ignore]` — run via tabtarget or `--ignored`.
//!
//! Relay tests mint real pensa (creating git commits). This is intentional —
//! these are end-to-end scenario tests.

use std::path::PathBuf;

use jjk::jjrlg_legatio::{
    jjrlg_BindArgs, jjrlg_run_bind,
    jjrlg_SendArgs, jjrlg_run_send,
    jjrlg_PlantArgs, jjrlg_run_plant,
    jjrlg_FetchArgs, jjrlg_run_fetch,
    jjrlg_RelayArgs, jjrlg_run_relay,
    jjrlg_CheckArgs, jjrlg_run_check,
};

// ============================================================================
// Account names — jjfu_* (JJ Fundus User)
// ============================================================================

const JJTLG_ACCOUNT_FULL: &str = "jjfu_full";
const JJTLG_ACCOUNT_NOKEY: &str = "jjfu_nokey";
const JJTLG_ACCOUNT_NOREPO: &str = "jjfu_norepo";
const JJTLG_ACCOUNT_NOGIT: &str = "jjfu_nogit";

// ============================================================================
// Shared constants
// ============================================================================

/// Relative directory for fundus projects (shared across all accounts).
const JJTLG_RELDIR: &str = "projects/rbm_alpha_recipemuster";

/// Tabtarget for relay tests — quick, safe, goes through BUD dispatch.
const JJTLG_RELAY_TABTARGET: &str = "buw-rcr.RenderConfigRegime.sh";

/// Delay tabtarget for concurrent overlap tests — 20s sleep through BUD dispatch.
const JJTLG_DELAY_TABTARGET: &str = "buw-xd.Delay.sh";

/// Relay timeout (seconds). Self-enforced remotely via watchdog.
const JJTLG_RELAY_TIMEOUT: u64 = 60;

/// Check poll timeout (seconds). How long to wait for completion.
const JJTLG_CHECK_POLL_TIMEOUT: u64 = 30;

/// Delay check poll timeout (seconds). Must exceed the 20s sleep plus dispatch overhead.
const JJTLG_DELAY_CHECK_TIMEOUT: u64 = 60;

/// Firemark for relay tests. Pensum minting validates against gallops,
/// so this must be a real racing heat.
const JJTLG_RELAY_FIREMARK: &str = "A4";

// ============================================================================
// Project root — cargo test CWD is the crate dir, not the project root.
// JJK library code uses relative paths from project root. Tests run with
// --test-threads=1, so set_current_dir is safe.
// ============================================================================

fn zjjtlg_ensure_project_root() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()    // Tools/jjk
            .parent().unwrap()    // Tools
            .parent().unwrap()    // project root
            .to_path_buf();
        std::env::set_current_dir(&root)
            .unwrap_or_else(|e| panic!("set_current_dir to {}: {}", root.display(), e));
    });
}

// ============================================================================
// Host constants
// ============================================================================

const JJTLG_HOST_LOCALHOST: &str = "localhost";
const JJTLG_HOST_CEREBRO: &str = "cerebro";

// ============================================================================
// Test officium (RAII)
// ============================================================================

/// Temporary officium directory for scenario tests.
/// Cleaned up on drop, even on panic.
struct jjtlg_TestOfficium {
    id: String,
    dir: PathBuf,
}

impl jjtlg_TestOfficium {
    fn new(label: &str) -> Self {
        zjjtlg_ensure_project_root();
        let ts = chrono::Local::now().format("%H%M%S%.3f").to_string();
        let bare_id = format!("test-{}-{}", label, ts);
        let dir = PathBuf::from(".claude/jjm/officia").join(&bare_id);
        std::fs::create_dir_all(&dir)
            .unwrap_or_else(|e| panic!("create test officium {}: {}", dir.display(), e));
        Self {
            id: format!("\u{2609}{}", bare_id), // ☉ prefix
            dir,
        }
    }
}

impl Drop for jjtlg_TestOfficium {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

// ============================================================================
// Profile configuration
// ============================================================================

struct jjtlg_FundusProfile {
    host: String,
    user: String,
    reldir: String,
}

// ============================================================================
// Preflight
// ============================================================================

fn zjjtlg_preflight_ssh(host: &str, user: &str) -> bool {
    std::process::Command::new("ssh")
        .args(["-o", "ConnectTimeout=5", "-o", "BatchMode=yes"])
        .arg(format!("{}@{}", user, host))
        .arg("exit 0")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn zjjtlg_preflight_happy(p: &jjtlg_FundusProfile) -> Result<(), String> {
    if !zjjtlg_preflight_ssh(&p.host, &p.user) {
        return Err(format!("SSH to {}@{} failed", p.user, p.host));
    }
    let ssh_test = |cmd: &str| -> bool {
        std::process::Command::new("ssh")
            .args(["-o", "ConnectTimeout=5", "-o", "BatchMode=yes"])
            .arg(format!("{}@{}", p.user, p.host))
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    };
    if !ssh_test(&format!("test -d $HOME/{}", p.reldir)) {
        return Err(format!("reldir not found: $HOME/{}", p.reldir));
    }
    if !ssh_test(&format!("test -f $HOME/{}/.buk/burc.env", p.reldir)) {
        return Err("BUK not installed: .buk/burc.env missing".to_string());
    }
    // Dispatch smoke: run a trivial tabtarget through BUD to verify station regime etc.
    let smoke = std::process::Command::new("ssh")
        .args(["-o", "ConnectTimeout=10", "-o", "BatchMode=yes"])
        .arg(format!("{}@{}", p.user, p.host))
        .arg(format!("cd $HOME/{} && ./tt/buw-rcr.RenderConfigRegime.sh >/dev/null 2>&1", p.reldir))
        .output();
    match smoke {
        Ok(o) if o.status.success() => Ok(()),
        Ok(o) => Err(format!("BUD dispatch smoke failed (exit {}) — check station regime (burs.env)",
            o.status.code().unwrap_or(-1))),
        Err(e) => Err(format!("BUD dispatch smoke SSH error: {}", e)),
    }
}

// ============================================================================
// Output parsing helpers
// ============================================================================

/// Parse legatio token from bind output ("Legatio L0 bound to ...").
fn zjjtlg_parse_legatio_token(output: &str) -> String {
    output.lines()
        .find(|l| l.contains("Legatio ") && l.contains(" bound to "))
        .and_then(|l| {
            let rest = l.split("Legatio ").nth(1)?;
            rest.split_whitespace().next()
        })
        .unwrap_or_else(|| panic!("cannot parse legatio token from:\n{}", output))
        .to_string()
}

/// Parse pensum display from relay output ("Pensum ₱XX%YY active.").
fn zjjtlg_parse_pensum_display(output: &str) -> String {
    output.lines()
        .find(|l| l.contains("Pensum ") && l.contains(" active."))
        .and_then(|l| {
            let rest = l.split("Pensum ").nth(1)?;
            rest.split_whitespace().next()
        })
        .unwrap_or_else(|| panic!("cannot parse pensum from:\n{}", output))
        .to_string()
}

/// Strip ₱ (U+20B1) prefix from pensum display to get raw token.
fn zjjtlg_pensum_raw_token(display: &str) -> &str {
    display.trim_start_matches('\u{20B1}')
}

// ============================================================================
// Shared test operations
// ============================================================================

/// Bind to a fundus profile and return the legatio token.
fn zjjtlg_bind_profile(p: &jjtlg_FundusProfile, officium: &jjtlg_TestOfficium) -> String {
    let (code, output) = jjrlg_run_bind(
        jjrlg_BindArgs {
            host: p.host.clone(),
            user: p.user.clone(),
            reldir: p.reldir.clone(),
        },
        &officium.id,
    );
    assert_eq!(code, 0, "bind failed:\n{}", output);
    zjjtlg_parse_legatio_token(&output)
}

// ============================================================================
// Output extraction helpers
// ============================================================================

/// Extract a BURX field value from check output (e.g., "BURX_LABEL:value" → "value").
fn zjjtlg_extract_burx_field(output: &str, field: &str) -> String {
    let prefix = format!("{}:", field);
    output.lines()
        .find(|l| l.starts_with(&prefix))
        .map(|l| l[prefix.len()..].trim().to_string())
        .unwrap_or_else(|| panic!("missing {} in output:\n{}", field, output))
}

// ============================================================================
// Curia readiness guard (RAII helpers + implementations)
// ============================================================================

/// RAII marker file that creates a dirty working tree. Cleaned up on drop.
struct jjtlg_DirtyMarker {
    path: PathBuf,
}

impl jjtlg_DirtyMarker {
    fn create() -> Self {
        zjjtlg_ensure_project_root();
        let path = PathBuf::from(".jjtest_dirty_marker");
        std::fs::write(&path, "curia readiness test marker")
            .unwrap_or_else(|e| panic!("failed to create dirty marker: {}", e));
        Self { path }
    }
}

impl Drop for jjtlg_DirtyMarker {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

fn jjtlg_relay_refuses_dirty_tree_impl(p: &jjtlg_FundusProfile) {
    let officium = jjtlg_TestOfficium::new("dirty-guard");
    let token = zjjtlg_bind_profile(p, &officium);

    let _marker = jjtlg_DirtyMarker::create();

    let (code, output) = jjrlg_run_relay(
        jjrlg_RelayArgs {
            legatio: token,
            tabtarget: JJTLG_RELAY_TABTARGET.to_string(),
            timeout: JJTLG_RELAY_TIMEOUT,
            firemark: JJTLG_RELAY_FIREMARK.to_string(),
        },
        &officium.id,
    );
    assert_ne!(code, 0, "relay should refuse dirty tree:\n{}", output);
    assert!(
        output.contains("dirty") && output.contains("relay refused"),
        "expected dirty-tree refusal message:\n{}", output
    );
    // _marker dropped here, cleaning up the dirty file
}

// ============================================================================
// Happy-path test implementations (jjfu_full)
// ============================================================================

fn jjtlg_bind_send_impl(p: &jjtlg_FundusProfile) {
    let officium = jjtlg_TestOfficium::new("bind-send");
    let token = zjjtlg_bind_profile(p, &officium);

    let (code, output) = jjrlg_run_send(
        jjrlg_SendArgs {
            legatio: token,
            command: "echo hello_from_fundus".to_string(),
        },
        &officium.id,
    );
    assert_eq!(code, 0, "send failed:\n{}", output);
    assert!(output.contains("hello_from_fundus"), "missing echo output:\n{}", output);
    assert!(output.contains("Exit: 0"), "expected exit 0:\n{}", output);
}

fn jjtlg_plant_impl(p: &jjtlg_FundusProfile) {
    let officium = jjtlg_TestOfficium::new("plant");
    let token = zjjtlg_bind_profile(p, &officium);

    // Get curia HEAD
    let head = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("git rev-parse HEAD");
    let commit = String::from_utf8_lossy(&head.stdout).trim().to_string();
    assert!(!commit.is_empty(), "failed to get HEAD commit");

    let (code, output) = jjrlg_run_plant(
        jjrlg_PlantArgs {
            legatio: token.clone(),
            commit: commit.clone(),
        },
        &officium.id,
    );
    assert_eq!(code, 0, "plant failed:\n{}", output);
    assert!(output.contains("Plant succeeded"), "no success message:\n{}", output);

    // Verify fundus HEAD matches
    let (sc, so) = jjrlg_run_send(
        jjrlg_SendArgs {
            legatio: token,
            command: "git rev-parse HEAD".to_string(),
        },
        &officium.id,
    );
    assert_eq!(sc, 0, "send after plant failed:\n{}", so);
    assert!(so.contains(&commit), "fundus HEAD mismatch:\n{}", so);
}

fn jjtlg_relay_check_instant_impl(p: &jjtlg_FundusProfile) {
    let firemark = JJTLG_RELAY_FIREMARK.to_string();

    let officium = jjtlg_TestOfficium::new("relay-inst");
    let token = zjjtlg_bind_profile(p, &officium);

    let (code, output) = jjrlg_run_relay(
        jjrlg_RelayArgs {
            legatio: token,
            tabtarget: JJTLG_RELAY_TABTARGET.to_string(),
            timeout: JJTLG_RELAY_TIMEOUT,
            firemark,
        },
        &officium.id,
    );
    assert_eq!(code, 0, "relay failed:\n{}", output);
    let pensum = zjjtlg_parse_pensum_display(&output);

    // Instant probe (timeout=0) — tabtarget may already be done
    let (cc, co) = jjrlg_run_check(
        jjrlg_CheckArgs { pensum, timeout: 0 },
        &officium.id,
    );
    assert_eq!(cc, 0, "check failed:\n{}", co);
    assert!(
        co.contains("Report: running") || co.contains("Report: stopped"),
        "unexpected report:\n{}", co
    );
}

fn jjtlg_relay_check_poll_impl(p: &jjtlg_FundusProfile) {
    let firemark = JJTLG_RELAY_FIREMARK.to_string();

    let officium = jjtlg_TestOfficium::new("relay-poll");
    let token = zjjtlg_bind_profile(p, &officium);

    let (code, output) = jjrlg_run_relay(
        jjrlg_RelayArgs {
            legatio: token,
            tabtarget: JJTLG_RELAY_TABTARGET.to_string(),
            timeout: JJTLG_RELAY_TIMEOUT,
            firemark,
        },
        &officium.id,
    );
    assert_eq!(code, 0, "relay failed:\n{}", output);
    let pensum = zjjtlg_parse_pensum_display(&output);
    let raw = zjjtlg_pensum_raw_token(&pensum);

    // Poll to completion
    let (cc, co) = jjrlg_run_check(
        jjrlg_CheckArgs { pensum: pensum.clone(), timeout: JJTLG_CHECK_POLL_TIMEOUT },
        &officium.id,
    );
    assert_eq!(cc, 0, "check failed:\n{}", co);
    assert!(co.contains("Report: stopped"), "expected stopped:\n{}", co);
    assert!(co.contains("BURX_EXIT_STATUS:"), "missing exit status:\n{}", co);

    // BURX_LABEL correlation: pensum token round-trips through BURE_LABEL → BURX_LABEL
    let label_line = co.lines()
        .find(|l| l.starts_with("BURX_LABEL:"))
        .unwrap_or_else(|| panic!("no BURX_LABEL in check output:\n{}", co));
    assert!(
        label_line.contains(raw),
        "BURX_LABEL mismatch: expected '{}' in '{}'\nfull output:\n{}", raw, label_line, co
    );

    // File list on terminal status
    assert!(co.contains("Files:"), "no file list on terminal status:\n{}", co);
    assert!(co.contains("burx.env"), "burx.env not in file list:\n{}", co);
}

fn jjtlg_relay_parallel_impl(p: &jjtlg_FundusProfile) {
    let firemark = JJTLG_RELAY_FIREMARK.to_string();

    let officium = jjtlg_TestOfficium::new("relay-par");
    let token = zjjtlg_bind_profile(p, &officium);

    // Dispatch 3 concurrent jobs
    let mut pensa = Vec::new();
    for i in 0..3 {
        let (code, output) = jjrlg_run_relay(
            jjrlg_RelayArgs {
                legatio: token.clone(),
                tabtarget: JJTLG_RELAY_TABTARGET.to_string(),
                timeout: JJTLG_RELAY_TIMEOUT,
                firemark: firemark.clone(),
            },
            &officium.id,
        );
        assert_eq!(code, 0, "relay {} failed:\n{}", i, output);
        pensa.push(zjjtlg_parse_pensum_display(&output));
    }

    // Verify all 3 complete independently with distinct labels
    let mut labels = Vec::new();
    for (i, pensum) in pensa.iter().enumerate() {
        let (cc, co) = jjrlg_run_check(
            jjrlg_CheckArgs { pensum: pensum.clone(), timeout: JJTLG_CHECK_POLL_TIMEOUT },
            &officium.id,
        );
        assert_eq!(cc, 0, "check {} failed:\n{}", i, co);
        assert!(co.contains("Report: stopped"), "pensum {} not stopped:\n{}", i, co);

        if let Some(line) = co.lines().find(|l| l.starts_with("BURX_LABEL:")) {
            labels.push(line.to_string());
        }
    }

    // No cross-contamination: all labels must be distinct
    for i in 0..labels.len() {
        for j in (i + 1)..labels.len() {
            assert_ne!(
                labels[i], labels[j],
                "cross-contamination: pensa {} and {} share label", i, j
            );
        }
    }
}

fn jjtlg_relay_concurrent_overlap_impl(p: &jjtlg_FundusProfile) {
    let firemark = JJTLG_RELAY_FIREMARK.to_string();

    let officium = jjtlg_TestOfficium::new("relay-conc");
    let token = zjjtlg_bind_profile(p, &officium);

    // Dispatch 3 concurrent relay jobs using the delay tabtarget
    let mut pensa = Vec::new();
    for i in 0..3 {
        let (code, output) = jjrlg_run_relay(
            jjrlg_RelayArgs {
                legatio: token.clone(),
                tabtarget: JJTLG_DELAY_TABTARGET.to_string(),
                timeout: JJTLG_RELAY_TIMEOUT,
                firemark: firemark.clone(),
            },
            &officium.id,
        );
        assert_eq!(code, 0, "relay {} failed:\n{}", i, output);
        pensa.push(zjjtlg_parse_pensum_display(&output));
    }

    // Instant-probe all 3 — 20s sleep guarantees temporal overlap
    for (i, pensum) in pensa.iter().enumerate() {
        let (cc, co) = jjrlg_run_check(
            jjrlg_CheckArgs { pensum: pensum.clone(), timeout: 0 },
            &officium.id,
        );
        assert_eq!(cc, 0, "instant check {} failed:\n{}", i, co);
        assert!(
            co.contains("Report: running"),
            "pensum {} not running (20s sleep should guarantee overlap):\n{}", i, co
        );
    }

    // Poll all 3 to completion, collecting labels and temp dirs
    let mut labels = Vec::new();
    let mut temp_dirs = Vec::new();
    for (i, pensum) in pensa.iter().enumerate() {
        let (cc, co) = jjrlg_run_check(
            jjrlg_CheckArgs { pensum: pensum.clone(), timeout: JJTLG_DELAY_CHECK_TIMEOUT },
            &officium.id,
        );
        assert_eq!(cc, 0, "poll check {} failed:\n{}", i, co);
        assert!(co.contains("Report: stopped"), "pensum {} not stopped:\n{}", i, co);

        // Verify BURX_LABEL correlates with pensum token
        let raw = zjjtlg_pensum_raw_token(pensum);
        let label = zjjtlg_extract_burx_field(&co, "BURX_LABEL");
        assert!(
            label.contains(raw),
            "pensum {} BURX_LABEL mismatch: expected '{}' in '{}'", i, raw, label
        );

        labels.push(label);
        temp_dirs.push(zjjtlg_extract_burx_field(&co, "BURX_TEMP_DIR"));
    }

    // No cross-contamination: all labels must be distinct
    for i in 0..labels.len() {
        for j in (i + 1)..labels.len() {
            assert_ne!(
                labels[i], labels[j],
                "cross-contamination: pensa {} and {} share BURX_LABEL", i, j
            );
        }
    }

    // Isolation: all temp dirs must be distinct
    for i in 0..temp_dirs.len() {
        for j in (i + 1)..temp_dirs.len() {
            assert_ne!(
                temp_dirs[i], temp_dirs[j],
                "isolation failure: pensa {} and {} share BURX_TEMP_DIR", i, j
            );
        }
    }
}

fn jjtlg_fetch_impl(p: &jjtlg_FundusProfile) {
    let officium = jjtlg_TestOfficium::new("fetch");
    let token = zjjtlg_bind_profile(p, &officium);

    // Fetch relative to RELDIR
    let (code, output) = jjrlg_run_fetch(
        jjrlg_FetchArgs {
            legatio: token.clone(),
            path: "CLAUDE.md".to_string(),
        },
        &officium.id,
    );
    assert_eq!(code, 0, "fetch relative failed:\n{}", output);
    assert!(!output.is_empty(), "fetch returned empty content");

    // Fetch absolute path
    let (ac, ao) = jjrlg_run_fetch(
        jjrlg_FetchArgs {
            legatio: token,
            path: "/etc/shells".to_string(),
        },
        &officium.id,
    );
    assert_eq!(ac, 0, "fetch absolute failed:\n{}", ao);
    assert!(ao.contains("/bin/"), "expected shell paths in /etc/shells:\n{}", ao);
}

// ============================================================================
// localhost — happy-path against localhost (always available)
// ============================================================================

mod localhost {
    use super::*;

    fn profile() -> jjtlg_FundusProfile {
        let p = jjtlg_FundusProfile {
            host: JJTLG_HOST_LOCALHOST.to_string(),
            user: JJTLG_ACCOUNT_FULL.to_string(),
            reldir: JJTLG_RELDIR.to_string(),
        };
        zjjtlg_preflight_happy(&p).unwrap_or_else(|e| panic!(
            "localhost profile not available ({}@{}): {}\nProvision: sudo tt/jjw-tfP.ProvisionFundusAccounts.localhost.sh",
            JJTLG_ACCOUNT_FULL, p.host, e));
        p
    }

    #[test]
    fn bind_send() { jjtlg_bind_send_impl(&profile()); }

    #[test]
    fn plant() { jjtlg_plant_impl(&profile()); }

    #[test]
    fn relay_check_instant() { jjtlg_relay_check_instant_impl(&profile()); }

    #[test]
    fn relay_check_poll() { jjtlg_relay_check_poll_impl(&profile()); }

    #[test]
    fn relay_parallel() { jjtlg_relay_parallel_impl(&profile()); }

    #[test]
    fn relay_concurrent_overlap() { jjtlg_relay_concurrent_overlap_impl(&profile()); }

    #[test]
    fn fetch() { jjtlg_fetch_impl(&profile()); }

    #[test]
    fn relay_refuses_dirty_tree() { jjtlg_relay_refuses_dirty_tree_impl(&profile()); }
}

// ============================================================================
// cerebro — happy-path against cerebro (remote, may not be reachable)
// ============================================================================

mod cerebro {
    use super::*;

    fn profile() -> jjtlg_FundusProfile {
        let p = jjtlg_FundusProfile {
            host: JJTLG_HOST_CEREBRO.to_string(),
            user: JJTLG_ACCOUNT_FULL.to_string(),
            reldir: JJTLG_RELDIR.to_string(),
        };
        zjjtlg_preflight_happy(&p).unwrap_or_else(|e| panic!(
            "cerebro profile not available ({}@{}): {}\nProvision fundus accounts on cerebro first.",
            JJTLG_ACCOUNT_FULL, p.host, e));
        p
    }

    #[test] #[ignore]
    fn bind_send() { jjtlg_bind_send_impl(&profile()); }

    #[test] #[ignore]
    fn plant() { jjtlg_plant_impl(&profile()); }

    #[test] #[ignore]
    fn relay_check_instant() { jjtlg_relay_check_instant_impl(&profile()); }

    #[test] #[ignore]
    fn relay_check_poll() { jjtlg_relay_check_poll_impl(&profile()); }

    #[test] #[ignore]
    fn relay_parallel() { jjtlg_relay_parallel_impl(&profile()); }

    #[test] #[ignore]
    fn relay_concurrent_overlap() { jjtlg_relay_concurrent_overlap_impl(&profile()); }

    #[test] #[ignore]
    fn fetch() { jjtlg_fetch_impl(&profile()); }

    #[test] #[ignore]
    fn relay_refuses_dirty_tree() { jjtlg_relay_refuses_dirty_tree_impl(&profile()); }
}

// ============================================================================
// nokey — bind auth failure (jjfu_nokey account)
// ============================================================================

mod nokey {
    use super::*;

    #[test]
    fn bind_fails_auth() {
        let host = JJTLG_HOST_LOCALHOST.to_string();
        // Preflight: verify the account exists on the target host
        let id_check = std::process::Command::new("ssh")
            .args(["-o", "ConnectTimeout=5", "-o", "BatchMode=yes"])
            .arg(format!("{}@{}", JJTLG_ACCOUNT_FULL, host))
            .arg(format!("id {}", JJTLG_ACCOUNT_NOKEY))
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        assert!(id_check,
            "{} account does not exist on {} — provision accounts first",
            JJTLG_ACCOUNT_NOKEY, host);

        let officium = jjtlg_TestOfficium::new("nokey");
        let (code, output) = jjrlg_run_bind(
            jjrlg_BindArgs {
                host,
                user: JJTLG_ACCOUNT_NOKEY.to_string(),
                reldir: JJTLG_RELDIR.to_string(),
            },
            &officium.id,
        );
        assert_ne!(code, 0, "bind should have failed for nokey profile:\n{}", output);
        assert!(
            output.contains("SSH probe failed"),
            "expected SSH probe failure:\n{}", output
        );
    }
}

// ============================================================================
// norepo — bind probe failure (jjfu_norepo account, no project at RELDIR)
// ============================================================================

mod norepo {
    use super::*;

    #[test]
    fn bind_fails_probe() {
        let host = JJTLG_HOST_LOCALHOST.to_string();
        // Preflight: SSH to jjfu_norepo must work
        assert!(zjjtlg_preflight_ssh(&host, JJTLG_ACCOUNT_NOREPO),
            "{} not reachable on {} — provision accounts first",
            JJTLG_ACCOUNT_NOREPO, host);

        let officium = jjtlg_TestOfficium::new("norepo");
        let (code, output) = jjrlg_run_bind(
            jjrlg_BindArgs {
                host,
                user: JJTLG_ACCOUNT_NOREPO.to_string(),
                reldir: JJTLG_RELDIR.to_string(),
            },
            &officium.id,
        );
        assert_ne!(code, 0, "bind should have failed for norepo profile:\n{}", output);
        assert!(
            output.contains("SSH probe failed"),
            "expected probe failure:\n{}", output
        );
    }
}

// ============================================================================
// nogit — plant failure, send works (jjfu_nogit account)
// ============================================================================

mod nogit {
    use super::*;

    fn profile() -> jjtlg_FundusProfile {
        let host = JJTLG_HOST_LOCALHOST.to_string();
        let p = jjtlg_FundusProfile {
            host,
            user: JJTLG_ACCOUNT_NOGIT.to_string(),
            reldir: JJTLG_RELDIR.to_string(),
        };
        assert!(zjjtlg_preflight_ssh(&p.host, &p.user),
            "nogit profile: SSH to {}@{} failed — provision accounts first",
            JJTLG_ACCOUNT_NOGIT, p.host);
        let ssh_test = |cmd: &str| -> bool {
            std::process::Command::new("ssh")
                .args(["-o", "ConnectTimeout=5", "-o", "BatchMode=yes"])
                .arg(format!("{}@{}", p.user, p.host))
                .arg(cmd)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        };
        assert!(
            ssh_test(&format!("test -d $HOME/{}", p.reldir))
                && ssh_test(&format!("test -f $HOME/{}/.buk/burc.env", p.reldir)),
            "nogit profile: reldir or BUK not found for {}@{} — provision accounts first",
            JJTLG_ACCOUNT_NOGIT, p.host);
        p
    }

    #[test]
    fn bind_succeeds() {
        let p = profile();
        let officium = jjtlg_TestOfficium::new("nogit-bind");
        let (code, output) = jjrlg_run_bind(
            jjrlg_BindArgs {
                host: p.host,
                user: p.user,
                reldir: p.reldir,
            },
            &officium.id,
        );
        assert_eq!(code, 0, "bind should succeed for nogit profile:\n{}", output);
    }

    #[test]
    fn send_succeeds() {
        let p = profile();
        let officium = jjtlg_TestOfficium::new("nogit-send");
        let token = zjjtlg_bind_profile(&p, &officium);

        let (code, output) = jjrlg_run_send(
            jjrlg_SendArgs {
                legatio: token,
                command: "echo nogit_works".to_string(),
            },
            &officium.id,
        );
        assert_eq!(code, 0, "send should work without git:\n{}", output);
        assert!(output.contains("nogit_works"), "missing echo output:\n{}", output);
    }

    #[test]
    fn plant_fails() {
        let p = profile();
        let officium = jjtlg_TestOfficium::new("nogit-plant");
        let token = zjjtlg_bind_profile(&p, &officium);

        let (code, output) = jjrlg_run_plant(
            jjrlg_PlantArgs {
                legatio: token,
                commit: "main".to_string(),
            },
            &officium.id,
        );
        assert_ne!(code, 0, "plant should fail without git origin:\n{}", output);
    }
}
