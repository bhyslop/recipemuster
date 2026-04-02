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
// RBTDRC — crucible test cases for theurge end-to-end testing
//
// Cases execute inside a charged crucible (sentry + pentacle + bottle).
// Thread-local context bridges the static case function signature with
// the mutable invocation context needed for tabtarget calls.

use std::cell::RefCell;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::rbtdre_engine::{rbtdre_Case, rbtdre_Section, rbtdre_Verdict};
use crate::rbtdri_invocation::{rbtdri_Context, rbtdri_invoke, rbtdri_parse_ifrit_verdict};
use crate::rbtdrm_manifest::{RBTDRM_COLOPHON_BARK, RBTDRM_COLOPHON_FIAT, RBTDRM_COLOPHON_WRIT};

// ── Thread-local invocation context ──────────────────────────

/// Ifrit binary name inside the bottle container.
const RBTDRC_IFRIT_BINARY: &str = "rbid";

thread_local! {
    static RBTDRC_CTX: RefCell<Option<rbtdri_Context>> = RefCell::new(None);
}

/// Store invocation context for case functions. Called before run_sections.
pub fn rbtdrc_set_context(ctx: rbtdri_Context) {
    RBTDRC_CTX.with(|c| *c.borrow_mut() = Some(ctx));
}

/// Retrieve invocation context after cases complete. Called for quench.
pub fn rbtdrc_take_context() -> rbtdri_Context {
    RBTDRC_CTX.with(|c| {
        c.borrow_mut()
            .take()
            .expect("rbtdrc: no context — was rbtdrc_set_context called?")
    })
}

fn rbtdrc_with_ctx<F>(f: F) -> rbtdre_Verdict
where
    F: FnOnce(&mut rbtdri_Context) -> rbtdre_Verdict,
{
    RBTDRC_CTX.with(|c| {
        let mut opt = c.borrow_mut();
        let ctx = opt
            .as_mut()
            .expect("rbtdrc: no invocation context for case execution");
        f(ctx)
    })
}

// ── Helpers ──────────────────────────────────────────────────

/// Invoke ifrit inside the bottle via bark, saving stdout/stderr to case dir.
fn rbtdrc_invoke_ifrit(
    ctx: &mut rbtdri_Context,
    attack: &str,
    dir: &Path,
) -> rbtdre_Verdict {
    let result = match rbtdri_invoke(ctx, RBTDRM_COLOPHON_BARK, &[RBTDRC_IFRIT_BINARY, attack]) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("bark invocation error: {}", e)),
    };
    let _ = std::fs::write(dir.join("bark-stdout.txt"), &result.stdout);
    let _ = std::fs::write(dir.join("bark-stderr.txt"), &result.stderr);
    rbtdri_parse_ifrit_verdict(&result.stdout, result.exit_code)
}

/// Invoke ifrit inside the bottle via bark with extra arguments.
fn rbtdrc_invoke_ifrit_with_args(
    ctx: &mut rbtdri_Context,
    attack: &str,
    extra_args: &[&str],
    dir: &Path,
) -> rbtdre_Verdict {
    let mut bark_args = vec![RBTDRC_IFRIT_BINARY, attack];
    bark_args.extend_from_slice(extra_args);
    let result = match rbtdri_invoke(ctx, RBTDRM_COLOPHON_BARK, &bark_args) {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("bark invocation error: {}", e)),
    };
    let _ = std::fs::write(dir.join("bark-stdout.txt"), &result.stdout);
    let _ = std::fs::write(dir.join("bark-stderr.txt"), &result.stderr);
    rbtdri_parse_ifrit_verdict(&result.stdout, result.exit_code)
}

/// Execute a command in the sentry via writ, returning captured stdout.
fn rbtdrc_writ(ctx: &mut rbtdri_Context, args: &[&str]) -> Result<String, String> {
    let result = rbtdri_invoke(ctx, RBTDRM_COLOPHON_WRIT, args)?;
    if result.exit_code != 0 {
        return Err(format!(
            "writ exit {}\nstdout: {}\nstderr: {}",
            result.exit_code, result.stdout, result.stderr
        ));
    }
    Ok(result.stdout)
}

/// Execute a command in the pentacle via fiat, returning the invocation result.
fn rbtdrc_fiat(ctx: &mut rbtdri_Context, args: &[&str]) -> Result<String, String> {
    let result = rbtdri_invoke(ctx, RBTDRM_COLOPHON_FIAT, args)?;
    if result.exit_code != 0 {
        return Err(format!(
            "fiat exit {}\nstdout: {}\nstderr: {}",
            result.exit_code, result.stdout, result.stderr
        ));
    }
    Ok(result.stdout)
}

/// Discover the sentry's enclave IP by reading /etc/resolv.conf from the pentacle via fiat.
/// The pentacle uses the sentry as its DNS server, so resolv.conf nameserver = sentry enclave IP.
/// (Sentry's own resolv.conf points to upstream DNS like 8.8.8.8 — wrong for enclave ops.)
fn rbtdrc_discover_sentry_ip(ctx: &mut rbtdri_Context) -> Result<String, String> {
    let output = rbtdrc_fiat(ctx, &["cat", "/etc/resolv.conf"])?;
    for line in output.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("nameserver") {
            let ip = rest.trim();
            if !ip.is_empty() && rbtdrc_looks_like_ip(ip) {
                return Ok(ip.to_string());
            }
        }
    }
    Err(format!(
        "no nameserver found in pentacle /etc/resolv.conf:\n{}",
        output
    ))
}

/// Resolve a hostname via writ (running dig +short on the sentry).
/// Returns the first line that looks like an IP address (filters BUK log headers).
fn rbtdrc_resolve_via_writ(ctx: &mut rbtdri_Context, hostname: &str) -> Result<String, String> {
    let output = rbtdrc_writ(ctx, &["dig", "+short", hostname])?;
    let ip = output
        .lines()
        .find(|line| {
            let t = line.trim();
            !t.is_empty() && rbtdrc_looks_like_ip(t)
        })
        .map(|line| line.trim().to_string())
        .ok_or_else(|| format!("dig +short {} returned no IP:\n{}", hostname, output))?;
    Ok(ip)
}

/// Quick check: does this string look like an IPv4 address (digits and dots only)?
/// Not a full validator — just enough to reject BUK log headers and DNS comments.
fn rbtdrc_looks_like_ip(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit() || c == '.')
}

// ── Basic infra cases (fiat) ──────────────────────────────────

fn rbtdrc_pentacle_dnsmasq_responds(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let sentry_ip = match rbtdrc_discover_sentry_ip(ctx) {
            Ok(ip) => ip,
            Err(e) => return rbtdre_Verdict::Fail(format!("sentry IP discovery: {}", e)),
        };
        let _ = std::fs::write(dir.join("sentry-ip.txt"), &sentry_ip);

        let output = match rbtdrc_fiat(
            ctx,
            &["dig", "+short", &format!("@{}", sentry_ip), "anthropic.com"],
        ) {
            Ok(o) => o,
            Err(e) => return rbtdre_Verdict::Fail(format!("fiat dig error: {}", e)),
        };
        let _ = std::fs::write(dir.join("fiat-stdout.txt"), &output);

        if output.trim().is_empty() {
            return rbtdre_Verdict::Fail(format!(
                "dnsmasq on sentry {} returned empty response for anthropic.com",
                sentry_ip
            ));
        }
        rbtdre_Verdict::Pass
    })
}

fn rbtdrc_pentacle_ping_sentry(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let sentry_ip = match rbtdrc_discover_sentry_ip(ctx) {
            Ok(ip) => ip,
            Err(e) => return rbtdre_Verdict::Fail(format!("sentry IP discovery: {}", e)),
        };
        let _ = std::fs::write(dir.join("sentry-ip.txt"), &sentry_ip);

        let output = match rbtdrc_fiat(ctx, &["ping", &sentry_ip, "-c", "2"]) {
            Ok(o) => o,
            Err(e) => return rbtdre_Verdict::Fail(format!("fiat ping error: {}", e)),
        };
        let _ = std::fs::write(dir.join("fiat-stdout.txt"), &output);

        rbtdre_Verdict::Pass
    })
}

// ── Ifrit attack cases (bark-only, inside observation) ───────

fn rbtdrc_ifrit_dns_allowed(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-allowed-anthropic", dir))
}

fn rbtdrc_ifrit_dns_blocked(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-blocked-google", dir))
}

fn rbtdrc_ifrit_apt_blocked(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "apt-get-blocked", dir))
}

fn rbtdrc_ifrit_dns_nonexistent(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-nonexistent", dir))
}

fn rbtdrc_ifrit_dns_tcp(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-tcp", dir))
}

fn rbtdrc_ifrit_dns_udp(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-udp", dir))
}

fn rbtdrc_ifrit_dns_block_direct(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-block-direct", dir))
}

fn rbtdrc_ifrit_dns_block_altport(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-block-altport", dir))
}

fn rbtdrc_ifrit_dns_block_cloudflare(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-block-cloudflare", dir))
}

fn rbtdrc_ifrit_dns_block_quad9(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-block-quad9", dir))
}

fn rbtdrc_ifrit_dns_block_zonetransfer(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-block-zonetransfer", dir))
}

fn rbtdrc_ifrit_dns_block_ipv6(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-block-ipv6", dir))
}

fn rbtdrc_ifrit_dns_block_multicast(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-block-multicast", dir))
}

fn rbtdrc_ifrit_dns_block_spoofing(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-block-spoofing", dir))
}

fn rbtdrc_ifrit_dns_block_tunneling(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-block-tunneling", dir))
}

// ── Observation cases (writ + bark, inside/outside) ──────────

fn rbtdrc_sentry_iptables_loaded(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let output = match rbtdrc_writ(ctx, &["iptables", "-S"]) {
            Ok(o) => o,
            Err(e) => return rbtdre_Verdict::Fail(format!("writ error: {}", e)),
        };
        let _ = std::fs::write(dir.join("iptables-rules.txt"), &output);

        if output.trim().is_empty() {
            return rbtdre_Verdict::Fail("iptables -S returned empty output".to_string());
        }
        // Expect at least one policy line (-P) and one append rule (-A)
        if !output.contains("-P") || !output.contains("-A") {
            return rbtdre_Verdict::Fail(format!(
                "iptables rules incomplete (missing -P or -A):\n{}",
                output
            ));
        }
        rbtdre_Verdict::Pass
    })
}

fn rbtdrc_dns_blocked_with_observation(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        // Before: capture iptables state from sentry
        let before = match rbtdrc_writ(ctx, &["iptables", "-L", "-v", "-n", "-x"]) {
            Ok(o) => {
                let _ = std::fs::write(dir.join("iptables-before.txt"), &o);
                o
            }
            Err(e) => return rbtdre_Verdict::Fail(format!("before-capture: {}", e)),
        };

        // Attack: invoke ifrit dns-blocked-google inside bottle
        let ifrit_verdict = rbtdrc_invoke_ifrit(ctx, "dns-blocked-google", dir);

        // After: capture iptables state from sentry
        let after = match rbtdrc_writ(ctx, &["iptables", "-L", "-v", "-n", "-x"]) {
            Ok(o) => {
                let _ = std::fs::write(dir.join("iptables-after.txt"), &o);
                o
            }
            Err(e) => return rbtdre_Verdict::Fail(format!("after-capture: {}", e)),
        };

        // Log observation: did sentry state change during the attack?
        let delta = if before != after {
            "CHANGED — sentry processed traffic during attack"
        } else {
            "UNCHANGED — no observable sentry state change"
        };
        let observation = format!(
            "Ifrit verdict: {}\nIptables delta: {}\n",
            match &ifrit_verdict {
                rbtdre_Verdict::Pass => "PASS",
                rbtdre_Verdict::Fail(_) => "FAIL",
                rbtdre_Verdict::Skip(_) => "SKIP",
            },
            delta,
        );
        let _ = std::fs::write(dir.join("observation.txt"), &observation);

        // Primary verdict is the ifrit result
        ifrit_verdict
    })
}

// ── Correlated cases (writ resolves IP, bark tests) ──────────

fn rbtdrc_tcp443_allow_anthropic(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let ip = match rbtdrc_resolve_via_writ(ctx, "anthropic.com") {
            Ok(ip) => ip,
            Err(e) => return rbtdre_Verdict::Fail(format!("writ resolve anthropic.com: {}", e)),
        };
        let _ = std::fs::write(dir.join("resolved-ip.txt"), &ip);
        rbtdrc_invoke_ifrit_with_args(ctx, "tcp443-connect", &[&ip], dir)
    })
}

fn rbtdrc_tcp443_block_google(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let ip = match rbtdrc_resolve_via_writ(ctx, "google.com") {
            Ok(ip) => ip,
            Err(e) => return rbtdre_Verdict::Fail(format!("writ resolve google.com: {}", e)),
        };
        let _ = std::fs::write(dir.join("resolved-ip.txt"), &ip);
        rbtdrc_invoke_ifrit_with_args(ctx, "tcp443-block", &[&ip], dir)
    })
}

fn rbtdrc_icmp_first_hop(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "icmp-first-hop", dir))
}

fn rbtdrc_icmp_second_hop_blocked(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "icmp-second-hop-blocked", dir))
}

// ── Ported sortie cases (bark-only) ──────────────────────────

fn rbtdrc_sortie_dns_exfil_subdomain(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "dns-exfil-subdomain", dir))
}

fn rbtdrc_sortie_meta_cloud_endpoint(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "meta-cloud-endpoint", dir))
}

fn rbtdrc_sortie_net_forbidden_cidr(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "net-forbidden-cidr", dir))
}

fn rbtdrc_sortie_direct_sentry_probe(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "direct-sentry-probe", dir))
}

fn rbtdrc_sortie_icmp_exfil_payload(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "icmp-exfil-payload", dir))
}

fn rbtdrc_sortie_net_ipv6_escape(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "net-ipv6-escape", dir))
}

fn rbtdrc_sortie_net_srcip_spoof(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "net-srcip-spoof", dir))
}

fn rbtdrc_sortie_proto_smuggle_rawsock(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "proto-smuggle-rawsock", dir))
}

fn rbtdrc_sortie_net_fragment_evasion(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "net-fragment-evasion", dir))
}

fn rbtdrc_sortie_direct_arp_poison(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "direct-arp-poison", dir))
}

fn rbtdrc_sortie_ns_capability_escape(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| rbtdrc_invoke_ifrit(ctx, "ns-capability-escape", dir))
}

// ── Host-side helpers (HTTP probes, port discovery) ──────────

/// Test container image for Python networking tests (srjcl WebSocket).
const RBTDRC_SRJCL_TEST_IMAGE: &str =
    "ghcr.io/bhyslop/recipemuster:rbtest_python_networking.20250215__171409";

/// Read RBRN_ENTRY_PORT_WORKSTATION from the nameplate's rbrn.env file.
fn rbtdrc_read_nameplate_port(ctx: &rbtdri_Context) -> Result<u16, String> {
    let env_path = ctx
        .project_root()
        .join(".rbk")
        .join(ctx.nameplate())
        .join("rbrn.env");
    let content = std::fs::read_to_string(&env_path)
        .map_err(|e| format!("cannot read {}: {}", env_path.display(), e))?;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("RBRN_ENTRY_PORT_WORKSTATION=") {
            return rest
                .trim()
                .parse::<u16>()
                .map_err(|e| format!("invalid port '{}': {}", rest.trim(), e));
        }
    }
    Err(format!(
        "RBRN_ENTRY_PORT_WORKSTATION not found in {}",
        env_path.display()
    ))
}

/// Simple curl GET, returns (body, exit_code).
fn rbtdrc_curl_get(url: &str) -> Result<(String, i32), String> {
    let output = Command::new("curl")
        .args(["-s", "--connect-timeout", "5", "--max-time", "10", url])
        .output()
        .map_err(|e| format!("curl exec failed: {}", e))?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let code = output.status.code().unwrap_or(-1);
    Ok((stdout, code))
}

/// Curl GET returning HTTP status code only.
fn rbtdrc_curl_status(url: &str, headers: &[(&str, &str)]) -> Result<String, String> {
    let mut cmd = Command::new("curl");
    cmd.args([
        "-s",
        "-o",
        "/dev/null",
        "-w",
        "%{http_code}",
        "--connect-timeout",
        "5",
        "--max-time",
        "10",
    ]);
    for (name, value) in headers {
        cmd.arg("-H").arg(format!("{}: {}", name, value));
    }
    cmd.arg(url);
    let output = cmd
        .output()
        .map_err(|e| format!("curl exec failed: {}", e))?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Curl POST with body from stdin, returns response body.
fn rbtdrc_curl_post_stdin(url: &str, body: &str) -> Result<String, String> {
    let mut child = Command::new("curl")
        .args(["-s", "--data-binary", "@-", url])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("curl POST spawn failed: {}", e))?;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(body.as_bytes())
        .map_err(|e| format!("curl POST write failed: {}", e))?;
    let output = child
        .wait_with_output()
        .map_err(|e| format!("curl POST wait failed: {}", e))?;
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

// ── SRJCL Jupyter cases (host-side probes) ───────────────────

fn rbtdrc_srjcl_jupyter_running(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let result =
            match rbtdri_invoke(ctx, RBTDRM_COLOPHON_BARK, &["ps", "aux"]) {
                Ok(r) => r,
                Err(e) => return rbtdre_Verdict::Fail(format!("bark ps aux: {}", e)),
            };
        let _ = std::fs::write(dir.join("bark-stdout.txt"), &result.stdout);
        let _ = std::fs::write(dir.join("bark-stderr.txt"), &result.stderr);

        if result.exit_code != 0 {
            return rbtdre_Verdict::Fail(format!(
                "ps aux exited {}\n{}",
                result.exit_code, result.stderr
            ));
        }
        if !result.stdout.contains("jupyter") {
            return rbtdre_Verdict::Fail("jupyter not running in bottle".to_string());
        }
        rbtdre_Verdict::Pass
    })
}

fn rbtdrc_srjcl_jupyter_connectivity(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let port = match rbtdrc_read_nameplate_port(ctx) {
            Ok(p) => p,
            Err(e) => return rbtdre_Verdict::Fail(format!("port discovery: {}", e)),
        };
        let url = format!("http://localhost:{}/lab", port);
        let status = match rbtdrc_curl_status(
            &url,
            &[
                ("User-Agent", "Mozilla/5.0"),
                ("Accept", "text/html,application/xhtml+xml"),
            ],
        ) {
            Ok(s) => s,
            Err(e) => return rbtdre_Verdict::Fail(format!("curl error: {}", e)),
        };
        let _ = std::fs::write(dir.join("http-status.txt"), &status);

        if status != "200" {
            return rbtdre_Verdict::Fail(format!(
                "expected HTTP 200 from Jupyter, got: {}",
                status
            ));
        }
        rbtdre_Verdict::Pass
    })
}

fn rbtdrc_srjcl_websocket_kernel(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let port = match rbtdrc_read_nameplate_port(ctx) {
            Ok(p) => p,
            Err(e) => return rbtdre_Verdict::Fail(format!("port discovery: {}", e)),
        };
        let test_script = ctx
            .project_root()
            .join("Tools/rbk/rbts/rbt_test_srjcl.py");
        let script_content = match std::fs::read_to_string(&test_script) {
            Ok(c) => c,
            Err(e) => {
                return rbtdre_Verdict::Fail(format!(
                    "cannot read test script {}: {}",
                    test_script.display(),
                    e
                ))
            }
        };

        let mut child = match Command::new("docker")
            .args([
                "run",
                "--rm",
                "-i",
                "--network",
                "host",
                "-e",
                &format!("RBRN_ENTRY_PORT_WORKSTATION={}", port),
                RBTDRC_SRJCL_TEST_IMAGE,
                "python3",
                "-",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                return rbtdre_Verdict::Fail(format!("docker run spawn failed: {}", e))
            }
        };
        let _ = child
            .stdin
            .take()
            .unwrap()
            .write_all(script_content.as_bytes());
        let output = match child.wait_with_output() {
            Ok(o) => o,
            Err(e) => {
                return rbtdre_Verdict::Fail(format!("docker run wait failed: {}", e))
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        let _ = std::fs::write(dir.join("docker-stdout.txt"), &stdout);
        let _ = std::fs::write(dir.join("docker-stderr.txt"), &stderr);

        let exit_code = output.status.code().unwrap_or(-1);
        if exit_code != 0 {
            return rbtdre_Verdict::Fail(format!(
                "Python WebSocket test exited {}\nstdout: {}\nstderr: {}",
                exit_code, stdout, stderr
            ));
        }
        rbtdre_Verdict::Pass
    })
}

// ── PLUML PlantUML cases (host-side HTTP probes) ─────────────

/// Known PlantUML diagram hash for Alice/Bob conversation.
const RBTDRC_PLUML_KNOWN_HASH: &str =
    "SyfFKj2rKt3CoKnELR1Io4ZDoSbNACb8BKhbWeZf0cMTyfEi59Boym40";

fn rbtdrc_pluml_text_rendering(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let port = match rbtdrc_read_nameplate_port(ctx) {
            Ok(p) => p,
            Err(e) => return rbtdre_Verdict::Fail(format!("port discovery: {}", e)),
        };
        let url = format!(
            "http://localhost:{}/txt/{}",
            port, RBTDRC_PLUML_KNOWN_HASH
        );
        let (body, _) = match rbtdrc_curl_get(&url) {
            Ok(r) => r,
            Err(e) => return rbtdre_Verdict::Fail(format!("curl error: {}", e)),
        };
        let _ = std::fs::write(dir.join("curl-response.txt"), &body);

        for expected in &["Bob", "Alice", "hello there", "boo"] {
            if !body.contains(expected) {
                return rbtdre_Verdict::Fail(format!(
                    "expected '{}' in response:\n{}",
                    expected, body
                ));
            }
        }
        rbtdre_Verdict::Pass
    })
}

fn rbtdrc_pluml_local_diagram(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let port = match rbtdrc_read_nameplate_port(ctx) {
            Ok(p) => p,
            Err(e) => return rbtdre_Verdict::Fail(format!("port discovery: {}", e)),
        };
        let url = format!("http://localhost:{}/txt/uml", port);
        let diagram = "@startuml\nBob -> Alice: hello there\nAlice --> Bob: boo\n@enduml";
        let body = match rbtdrc_curl_post_stdin(&url, diagram) {
            Ok(b) => b,
            Err(e) => return rbtdre_Verdict::Fail(format!("curl POST error: {}", e)),
        };
        let _ = std::fs::write(dir.join("curl-response.txt"), &body);

        for expected in &["Bob", "Alice", "hello there", "boo"] {
            if !body.contains(expected) {
                return rbtdre_Verdict::Fail(format!(
                    "expected '{}' in response:\n{}",
                    expected, body
                ));
            }
        }
        rbtdre_Verdict::Pass
    })
}

fn rbtdrc_pluml_http_headers(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let port = match rbtdrc_read_nameplate_port(ctx) {
            Ok(p) => p,
            Err(e) => return rbtdre_Verdict::Fail(format!("port discovery: {}", e)),
        };
        let url = format!(
            "http://localhost:{}/txt/{}",
            port, RBTDRC_PLUML_KNOWN_HASH
        );
        let status = match rbtdrc_curl_status(
            &url,
            &[
                ("User-Agent", "Mozilla/5.0"),
                ("Accept", "text/plain"),
            ],
        ) {
            Ok(s) => s,
            Err(e) => return rbtdre_Verdict::Fail(format!("curl error: {}", e)),
        };
        let _ = std::fs::write(dir.join("http-status.txt"), &status);

        if status != "200" {
            return rbtdre_Verdict::Fail(format!("expected HTTP 200, got: {}", status));
        }
        rbtdre_Verdict::Pass
    })
}

fn rbtdrc_pluml_invalid_hash(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let port = match rbtdrc_read_nameplate_port(ctx) {
            Ok(p) => p,
            Err(e) => return rbtdre_Verdict::Fail(format!("port discovery: {}", e)),
        };
        let url = format!("http://localhost:{}/txt/invalid_hash", port);
        let (body, _) = match rbtdrc_curl_get(&url) {
            Ok(r) => r,
            Err(e) => return rbtdre_Verdict::Fail(format!("curl error: {}", e)),
        };
        let _ = std::fs::write(dir.join("curl-response.txt"), &body);

        if body.contains("Bob") {
            return rbtdre_Verdict::Fail(
                "expected no 'Bob' in invalid hash response".to_string(),
            );
        }
        rbtdre_Verdict::Pass
    })
}

fn rbtdrc_pluml_malformed_diagram(dir: &Path) -> rbtdre_Verdict {
    rbtdrc_with_ctx(|ctx| {
        let port = match rbtdrc_read_nameplate_port(ctx) {
            Ok(p) => p,
            Err(e) => return rbtdre_Verdict::Fail(format!("port discovery: {}", e)),
        };
        let url = format!("http://localhost:{}/txt/uml", port);
        let body = match rbtdrc_curl_post_stdin(&url, "invalid uml content") {
            Ok(b) => b,
            Err(e) => return rbtdre_Verdict::Fail(format!("curl POST error: {}", e)),
        };
        let _ = std::fs::write(dir.join("curl-response.txt"), &body);

        if body.contains("Bob") {
            return rbtdre_Verdict::Fail(
                "expected no 'Bob' in malformed diagram response".to_string(),
            );
        }
        rbtdre_Verdict::Pass
    })
}

// ── Section registry ─────────────────────────────────────────

/// Readiness delay in seconds after charge for service-bearing nameplates.
/// Matches RBCC_BOTTLE_TEST_READINESS_DELAY_SEC from rbcc_Constants.sh.
pub const RBTDRC_SERVICE_READINESS_DELAY_SECS: u64 = 30;

/// Returns whether this nameplate needs a post-charge readiness delay.
pub fn rbtdrc_needs_readiness_delay(nameplate: &str) -> bool {
    matches!(nameplate, "srjcl" | "pluml")
}

/// Returns the section array appropriate for the given nameplate.
pub fn rbtdrc_sections_for_nameplate(nameplate: &str) -> &'static [rbtdre_Section] {
    match nameplate {
        "tadmor" => RBTDRC_SECTIONS_TADMOR,
        "srjcl" => RBTDRC_SECTIONS_SRJCL,
        "pluml" => RBTDRC_SECTIONS_PLUML,
        _ => {
            eprintln!(
                "rbtdrc: no sections defined for nameplate '{}' — running empty",
                nameplate
            );
            &[]
        }
    }
}

pub static RBTDRC_SECTIONS_SRJCL: &[rbtdre_Section] = &[rbtdre_Section {
    name: "srjcl-jupyter",
    cases: &[
        rbtdre_Case {
            name: "jupyter-running",
            func: rbtdrc_srjcl_jupyter_running,
        },
        rbtdre_Case {
            name: "jupyter-connectivity",
            func: rbtdrc_srjcl_jupyter_connectivity,
        },
        rbtdre_Case {
            name: "websocket-kernel",
            func: rbtdrc_srjcl_websocket_kernel,
        },
    ],
}];

pub static RBTDRC_SECTIONS_PLUML: &[rbtdre_Section] = &[rbtdre_Section {
    name: "pluml-diagram",
    cases: &[
        rbtdre_Case {
            name: "text-rendering",
            func: rbtdrc_pluml_text_rendering,
        },
        rbtdre_Case {
            name: "local-diagram",
            func: rbtdrc_pluml_local_diagram,
        },
        rbtdre_Case {
            name: "http-headers",
            func: rbtdrc_pluml_http_headers,
        },
        rbtdre_Case {
            name: "invalid-hash",
            func: rbtdrc_pluml_invalid_hash,
        },
        rbtdre_Case {
            name: "malformed-diagram",
            func: rbtdrc_pluml_malformed_diagram,
        },
    ],
}];

static RBTDRC_SECTIONS_TADMOR: &[rbtdre_Section] = &[
    rbtdre_Section {
        name: "tadmor-basic-infra",
        cases: &[
            rbtdre_Case {
                name: "pentacle-dnsmasq-responds",
                func: rbtdrc_pentacle_dnsmasq_responds,
            },
            rbtdre_Case {
                name: "pentacle-ping-sentry",
                func: rbtdrc_pentacle_ping_sentry,
            },
        ],
    },
    rbtdre_Section {
        name: "tadmor-ifrit-attacks",
        cases: &[
            rbtdre_Case {
                name: "ifrit-dns-allowed",
                func: rbtdrc_ifrit_dns_allowed,
            },
            rbtdre_Case {
                name: "ifrit-dns-blocked",
                func: rbtdrc_ifrit_dns_blocked,
            },
            rbtdre_Case {
                name: "ifrit-apt-blocked",
                func: rbtdrc_ifrit_apt_blocked,
            },
            rbtdre_Case {
                name: "ifrit-dns-nonexistent",
                func: rbtdrc_ifrit_dns_nonexistent,
            },
            rbtdre_Case {
                name: "ifrit-dns-tcp",
                func: rbtdrc_ifrit_dns_tcp,
            },
            rbtdre_Case {
                name: "ifrit-dns-udp",
                func: rbtdrc_ifrit_dns_udp,
            },
            rbtdre_Case {
                name: "ifrit-dns-block-direct",
                func: rbtdrc_ifrit_dns_block_direct,
            },
            rbtdre_Case {
                name: "ifrit-dns-block-altport",
                func: rbtdrc_ifrit_dns_block_altport,
            },
            rbtdre_Case {
                name: "ifrit-dns-block-cloudflare",
                func: rbtdrc_ifrit_dns_block_cloudflare,
            },
            rbtdre_Case {
                name: "ifrit-dns-block-quad9",
                func: rbtdrc_ifrit_dns_block_quad9,
            },
            rbtdre_Case {
                name: "ifrit-dns-block-zonetransfer",
                func: rbtdrc_ifrit_dns_block_zonetransfer,
            },
            rbtdre_Case {
                name: "ifrit-dns-block-ipv6",
                func: rbtdrc_ifrit_dns_block_ipv6,
            },
            rbtdre_Case {
                name: "ifrit-dns-block-multicast",
                func: rbtdrc_ifrit_dns_block_multicast,
            },
            rbtdre_Case {
                name: "ifrit-dns-block-spoofing",
                func: rbtdrc_ifrit_dns_block_spoofing,
            },
            rbtdre_Case {
                name: "ifrit-dns-block-tunneling",
                func: rbtdrc_ifrit_dns_block_tunneling,
            },
        ],
    },
    rbtdre_Section {
        name: "tadmor-observation",
        cases: &[
            rbtdre_Case {
                name: "sentry-iptables-loaded",
                func: rbtdrc_sentry_iptables_loaded,
            },
            rbtdre_Case {
                name: "dns-blocked-with-observation",
                func: rbtdrc_dns_blocked_with_observation,
            },
        ],
    },
    rbtdre_Section {
        name: "tadmor-correlated",
        cases: &[
            rbtdre_Case {
                name: "tcp443-allow-anthropic",
                func: rbtdrc_tcp443_allow_anthropic,
            },
            rbtdre_Case {
                name: "tcp443-block-google",
                func: rbtdrc_tcp443_block_google,
            },
            rbtdre_Case {
                name: "icmp-first-hop",
                func: rbtdrc_icmp_first_hop,
            },
            rbtdre_Case {
                name: "icmp-second-hop-blocked",
                func: rbtdrc_icmp_second_hop_blocked,
            },
        ],
    },
    rbtdre_Section {
        name: "tadmor-sortie-attacks",
        cases: &[
            rbtdre_Case {
                name: "sortie-dns-exfil-subdomain",
                func: rbtdrc_sortie_dns_exfil_subdomain,
            },
            rbtdre_Case {
                name: "sortie-meta-cloud-endpoint",
                func: rbtdrc_sortie_meta_cloud_endpoint,
            },
            rbtdre_Case {
                name: "sortie-net-forbidden-cidr",
                func: rbtdrc_sortie_net_forbidden_cidr,
            },
            rbtdre_Case {
                name: "sortie-direct-sentry-probe",
                func: rbtdrc_sortie_direct_sentry_probe,
            },
            rbtdre_Case {
                name: "sortie-icmp-exfil-payload",
                func: rbtdrc_sortie_icmp_exfil_payload,
            },
            rbtdre_Case {
                name: "sortie-net-ipv6-escape",
                func: rbtdrc_sortie_net_ipv6_escape,
            },
            rbtdre_Case {
                name: "sortie-net-srcip-spoof",
                func: rbtdrc_sortie_net_srcip_spoof,
            },
            rbtdre_Case {
                name: "sortie-proto-smuggle-rawsock",
                func: rbtdrc_sortie_proto_smuggle_rawsock,
            },
            rbtdre_Case {
                name: "sortie-net-fragment-evasion",
                func: rbtdrc_sortie_net_fragment_evasion,
            },
            rbtdre_Case {
                name: "sortie-direct-arp-poison",
                func: rbtdrc_sortie_direct_arp_poison,
            },
            rbtdre_Case {
                name: "sortie-ns-capability-escape",
                func: rbtdrc_sortie_ns_capability_escape,
            },
        ],
    },
];
