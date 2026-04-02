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
use std::path::Path;

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

/// Discover the sentry IP by reading /etc/resolv.conf from inside the bottle via writ.
/// Returns the first nameserver IP found.
fn rbtdrc_discover_sentry_ip(ctx: &mut rbtdri_Context) -> Result<String, String> {
    let output = rbtdrc_writ(ctx, &["cat", "/etc/resolv.conf"])?;
    for line in output.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("nameserver") {
            let ip = rest.trim();
            if !ip.is_empty() {
                return Ok(ip.to_string());
            }
        }
    }
    Err(format!(
        "no nameserver found in /etc/resolv.conf:\n{}",
        output
    ))
}

/// Resolve a hostname via writ (running dig +short on the sentry).
/// Returns the first IP address line from dig output.
fn rbtdrc_resolve_via_writ(ctx: &mut rbtdri_Context, hostname: &str) -> Result<String, String> {
    let output = rbtdrc_writ(ctx, &["dig", "+short", hostname])?;
    let ip = output
        .lines()
        .find(|line| {
            let t = line.trim();
            !t.is_empty() && !t.starts_with(';')
        })
        .map(|line| line.trim().to_string())
        .ok_or_else(|| format!("dig +short {} returned no IP:\n{}", hostname, output))?;
    Ok(ip)
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

// ── Section registry ─────────────────────────────────────────

pub static RBTDRC_SECTIONS: &[rbtdre_Section] = &[
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
];
