// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Fundus scenario tests for remote dispatch via SSH.
//!
//! Run via tabtargets:
//!   Suite:  `tt/jjw-tfs.TestFundusScenario.<host>.sh`
//!   Single: `tt/jjw-tfS.TestFundusSingle.<host>.sh <test_fn>`
//!
//! Requires jjfu_* accounts on the target host — see JJSTF-test-fundus.adoc.
//!
//! All tests use `#[ignore]` to prevent running during normal `cargo test`.
//! The workbench invokes with `--ignored` after setting JJTEST_HOST.
//!
//! Relay tests mint real pensa (creating git commits). This is intentional —
//! these are end-to-end scenario tests run explicitly via tabtarget.

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

const JJFU_FULL: &str = "jjfu_full";
const JJFU_NOKEY: &str = "jjfu_nokey";
const JJFU_NOREPO: &str = "jjfu_norepo";
const JJFU_NOGIT: &str = "jjfu_nogit";

// ============================================================================
// Shared constants
// ============================================================================

/// Relative directory for fundus projects (shared across all accounts).
const RELDIR: &str = "projects/rbm_alpha_recipemuster";

/// Tabtarget for relay tests — quick, safe, goes through BUD dispatch.
const RELAY_TEST_TABTARGET: &str = "buw-rcr.RenderConfigRegime.sh";

/// Delay tabtarget for concurrent overlap tests — 20s sleep through BUD dispatch.
const DELAY_TABTARGET: &str = "buw-xd.Delay.sh";

/// Relay timeout (seconds). Self-enforced remotely via watchdog.
const RELAY_TEST_TIMEOUT: u64 = 60;

/// Check poll timeout (seconds). How long to wait for completion.
const CHECK_POLL_TIMEOUT: u64 = 30;

/// Delay check poll timeout (seconds). Must exceed the 20s sleep plus dispatch overhead.
const DELAY_CHECK_TIMEOUT: u64 = 60;

/// Gallops JSON path (relative to project root — CWD during tabtarget dispatch).
const GALLOPS_PATH: &str = ".claude/jjm/jjg_gallops.json";

// ============================================================================
// Host resolution
// ============================================================================

/// Read target host from JJTEST_HOST (set by workbench from tabtarget imprint).
fn test_host() -> String {
    std::env::var("JJTEST_HOST")
        .unwrap_or_else(|_| panic!("JJTEST_HOST not set — run via tabtarget"))
}

// ============================================================================
// Test officium (RAII)
// ============================================================================

/// Temporary officium directory for scenario tests.
/// Cleaned up on drop, even on panic.
struct TestOfficium {
    id: String,
    dir: PathBuf,
}

impl TestOfficium {
    fn new(label: &str) -> Self {
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

impl Drop for TestOfficium {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

// ============================================================================
// Profile configuration
// ============================================================================

struct FundusProfile {
    host: String,
    user: String,
    reldir: String,
}

// ============================================================================
// Preflight
// ============================================================================

fn preflight_ssh(host: &str, user: &str) -> bool {
    std::process::Command::new("ssh")
        .args(["-o", "ConnectTimeout=5", "-o", "BatchMode=yes"])
        .arg(format!("{}@{}", user, host))
        .arg("exit 0")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn preflight_happy(p: &FundusProfile) -> bool {
    if !preflight_ssh(&p.host, &p.user) { return false; }
    let ssh_test = |cmd: &str| -> bool {
        std::process::Command::new("ssh")
            .args(["-o", "ConnectTimeout=5", "-o", "BatchMode=yes"])
            .arg(format!("{}@{}", p.user, p.host))
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    };
    ssh_test(&format!("test -d $HOME/{}", p.reldir))
        && ssh_test(&format!("test -f $HOME/{}/.buk/burc.env", p.reldir))
}

// ============================================================================
// Output parsing helpers
// ============================================================================

/// Parse legatio token from bind output ("Legatio L0 bound to ...").
fn parse_legatio_token(output: &str) -> String {
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
fn parse_pensum_display(output: &str) -> String {
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
fn pensum_raw_token(display: &str) -> &str {
    display.trim_start_matches('\u{20B1}')
}

// ============================================================================
// Shared test operations
// ============================================================================

/// Bind to a fundus profile and return the legatio token.
fn bind_profile(p: &FundusProfile, officium: &TestOfficium) -> String {
    let (code, output) = jjrlg_run_bind(
        jjrlg_BindArgs {
            host: p.host.clone(),
            user: p.user.clone(),
            reldir: p.reldir.clone(),
        },
        &officium.id,
    );
    assert_eq!(code, 0, "bind failed:\n{}", output);
    parse_legatio_token(&output)
}

/// Find a racing heat firemark from gallops JSON (for relay tests).
fn find_racing_firemark() -> Option<String> {
    let content = std::fs::read_to_string(GALLOPS_PATH).ok()?;
    let v: serde_json::Value = serde_json::from_str(&content).ok()?;
    let heats = v.get("heats")?.as_object()?;
    for (key, heat) in heats {
        if heat.get("status").and_then(|s| s.as_str()) == Some("racing") {
            return Some(key.trim_start_matches('\u{20A3}').to_string()); // ₣
        }
    }
    None
}

// ============================================================================
// Output extraction helpers
// ============================================================================

/// Extract a BURX field value from check output (e.g., "BURX_LABEL:value" → "value").
fn extract_burx_field(output: &str, field: &str) -> String {
    let prefix = format!("{}:", field);
    output.lines()
        .find(|l| l.starts_with(&prefix))
        .map(|l| l[prefix.len()..].trim().to_string())
        .unwrap_or_else(|| panic!("missing {} in output:\n{}", field, output))
}

// ============================================================================
// Happy-path test implementations (jjfu_full)
// ============================================================================

fn test_bind_send_impl(p: &FundusProfile) {
    let officium = TestOfficium::new("bind-send");
    let token = bind_profile(p, &officium);

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

fn test_plant_impl(p: &FundusProfile) {
    let officium = TestOfficium::new("plant");
    let token = bind_profile(p, &officium);

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

fn test_relay_check_instant_impl(p: &FundusProfile) {
    let firemark = match find_racing_firemark() {
        Some(f) => f,
        None => { eprintln!("SKIP: no racing heat found for relay test"); return; }
    };

    let officium = TestOfficium::new("relay-inst");
    let token = bind_profile(p, &officium);

    let (code, output) = jjrlg_run_relay(
        jjrlg_RelayArgs {
            legatio: token,
            tabtarget: RELAY_TEST_TABTARGET.to_string(),
            timeout: RELAY_TEST_TIMEOUT,
            firemark,
        },
        &officium.id,
    );
    assert_eq!(code, 0, "relay failed:\n{}", output);
    let pensum = parse_pensum_display(&output);

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

fn test_relay_check_poll_impl(p: &FundusProfile) {
    let firemark = match find_racing_firemark() {
        Some(f) => f,
        None => { eprintln!("SKIP: no racing heat found for relay test"); return; }
    };

    let officium = TestOfficium::new("relay-poll");
    let token = bind_profile(p, &officium);

    let (code, output) = jjrlg_run_relay(
        jjrlg_RelayArgs {
            legatio: token,
            tabtarget: RELAY_TEST_TABTARGET.to_string(),
            timeout: RELAY_TEST_TIMEOUT,
            firemark,
        },
        &officium.id,
    );
    assert_eq!(code, 0, "relay failed:\n{}", output);
    let pensum = parse_pensum_display(&output);
    let raw = pensum_raw_token(&pensum);

    // Poll to completion
    let (cc, co) = jjrlg_run_check(
        jjrlg_CheckArgs { pensum: pensum.clone(), timeout: CHECK_POLL_TIMEOUT },
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

fn test_relay_parallel_impl(p: &FundusProfile) {
    let firemark = match find_racing_firemark() {
        Some(f) => f,
        None => { eprintln!("SKIP: no racing heat found for parallel test"); return; }
    };

    let officium = TestOfficium::new("relay-par");
    let token = bind_profile(p, &officium);

    // Dispatch 3 concurrent jobs
    let mut pensa = Vec::new();
    for i in 0..3 {
        let (code, output) = jjrlg_run_relay(
            jjrlg_RelayArgs {
                legatio: token.clone(),
                tabtarget: RELAY_TEST_TABTARGET.to_string(),
                timeout: RELAY_TEST_TIMEOUT,
                firemark: firemark.clone(),
            },
            &officium.id,
        );
        assert_eq!(code, 0, "relay {} failed:\n{}", i, output);
        pensa.push(parse_pensum_display(&output));
    }

    // Verify all 3 complete independently with distinct labels
    let mut labels = Vec::new();
    for (i, pensum) in pensa.iter().enumerate() {
        let (cc, co) = jjrlg_run_check(
            jjrlg_CheckArgs { pensum: pensum.clone(), timeout: CHECK_POLL_TIMEOUT },
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

fn test_relay_concurrent_overlap_impl(p: &FundusProfile) {
    let firemark = match find_racing_firemark() {
        Some(f) => f,
        None => { eprintln!("SKIP: no racing heat found for concurrent test"); return; }
    };

    let officium = TestOfficium::new("relay-conc");
    let token = bind_profile(p, &officium);

    // Plant curia HEAD to ensure delay tabtarget exists on fundus
    let head = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("git rev-parse HEAD");
    let commit = String::from_utf8_lossy(&head.stdout).trim().to_string();
    assert!(!commit.is_empty(), "failed to get HEAD commit");

    let (pc, po) = jjrlg_run_plant(
        jjrlg_PlantArgs {
            legatio: token.clone(),
            commit,
        },
        &officium.id,
    );
    assert_eq!(pc, 0, "plant failed (needed for delay tabtarget):\n{}", po);

    // Dispatch 3 concurrent relay jobs using the delay tabtarget
    let mut pensa = Vec::new();
    for i in 0..3 {
        let (code, output) = jjrlg_run_relay(
            jjrlg_RelayArgs {
                legatio: token.clone(),
                tabtarget: DELAY_TABTARGET.to_string(),
                timeout: RELAY_TEST_TIMEOUT,
                firemark: firemark.clone(),
            },
            &officium.id,
        );
        assert_eq!(code, 0, "relay {} failed:\n{}", i, output);
        pensa.push(parse_pensum_display(&output));
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
            jjrlg_CheckArgs { pensum: pensum.clone(), timeout: DELAY_CHECK_TIMEOUT },
            &officium.id,
        );
        assert_eq!(cc, 0, "poll check {} failed:\n{}", i, co);
        assert!(co.contains("Report: stopped"), "pensum {} not stopped:\n{}", i, co);

        // Verify BURX_LABEL correlates with pensum token
        let raw = pensum_raw_token(pensum);
        let label = extract_burx_field(&co, "BURX_LABEL");
        assert!(
            label.contains(raw),
            "pensum {} BURX_LABEL mismatch: expected '{}' in '{}'", i, raw, label
        );

        labels.push(label);
        temp_dirs.push(extract_burx_field(&co, "BURX_TEMP_DIR"));
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

fn test_fetch_impl(p: &FundusProfile) {
    let officium = TestOfficium::new("fetch");
    let token = bind_profile(p, &officium);

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
// full — happy-path (jjfu_full account)
// ============================================================================

mod full {
    use super::*;

    fn profile() -> FundusProfile {
        let p = FundusProfile {
            host: test_host(),
            user: JJFU_FULL.to_string(),
            reldir: RELDIR.to_string(),
        };
        assert!(preflight_happy(&p),
            "full profile not available ({}@{}) — provision accounts first: sudo tt/jjw-tfP.ProvisionFundusAccounts.localhost.sh",
            JJFU_FULL, p.host);
        p
    }

    #[test] #[ignore]
    fn bind_send() { test_bind_send_impl(&profile()); }

    #[test] #[ignore]
    fn plant() { test_plant_impl(&profile()); }

    #[test] #[ignore]
    fn relay_check_instant() { test_relay_check_instant_impl(&profile()); }

    #[test] #[ignore]
    fn relay_check_poll() { test_relay_check_poll_impl(&profile()); }

    #[test] #[ignore]
    fn relay_parallel() { test_relay_parallel_impl(&profile()); }

    #[test] #[ignore]
    fn relay_concurrent_overlap() { test_relay_concurrent_overlap_impl(&profile()); }

    #[test] #[ignore]
    fn fetch() { test_fetch_impl(&profile()); }
}

// ============================================================================
// nokey — bind auth failure (jjfu_nokey account)
// ============================================================================

mod nokey {
    use super::*;

    #[test] #[ignore]
    fn bind_fails_auth() {
        let host = test_host();
        // Preflight: verify the account exists on the target host
        let id_check = std::process::Command::new("ssh")
            .args(["-o", "ConnectTimeout=5", "-o", "BatchMode=yes"])
            .arg(format!("{}@{}", JJFU_FULL, host))
            .arg(format!("id {}", JJFU_NOKEY))
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        assert!(id_check,
            "{} account does not exist on {} — provision accounts first",
            JJFU_NOKEY, host);

        let officium = TestOfficium::new("nokey");
        let (code, output) = jjrlg_run_bind(
            jjrlg_BindArgs {
                host,
                user: JJFU_NOKEY.to_string(),
                reldir: RELDIR.to_string(),
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

    #[test] #[ignore]
    fn bind_fails_probe() {
        let host = test_host();
        // Preflight: SSH to jjfu_norepo must work
        assert!(preflight_ssh(&host, JJFU_NOREPO),
            "{} not reachable on {} — provision accounts first",
            JJFU_NOREPO, host);

        let officium = TestOfficium::new("norepo");
        let (code, output) = jjrlg_run_bind(
            jjrlg_BindArgs {
                host,
                user: JJFU_NOREPO.to_string(),
                reldir: RELDIR.to_string(),
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

    fn profile() -> FundusProfile {
        let host = test_host();
        let p = FundusProfile {
            host,
            user: JJFU_NOGIT.to_string(),
            reldir: RELDIR.to_string(),
        };
        assert!(preflight_ssh(&p.host, &p.user),
            "nogit profile: SSH to {}@{} failed — provision accounts first",
            JJFU_NOGIT, p.host);
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
            JJFU_NOGIT, p.host);
        p
    }

    #[test] #[ignore]
    fn bind_succeeds() {
        let p = profile();
        let officium = TestOfficium::new("nogit-bind");
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

    #[test] #[ignore]
    fn send_succeeds() {
        let p = profile();
        let officium = TestOfficium::new("nogit-send");
        let token = bind_profile(&p, &officium);

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

    #[test] #[ignore]
    fn plant_fails() {
        let p = profile();
        let officium = TestOfficium::new("nogit-plant");
        let token = bind_profile(&p, &officium);

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
