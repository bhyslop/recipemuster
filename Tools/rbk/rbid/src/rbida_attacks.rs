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
// RBIDA — attack definitions and dispatch for ifrit
//
// Each variant of rbida_Attack represents one security boundary probe.
// Exhaustive match in rbida_run ensures adding a variant forces handling.
// Attacks shell out to system commands available in the ifrit vessel image.

use std::process::Command;

// ── Attack Enum ─────────────────────────────────────────────────

/// Security boundary attack. Each variant probes one aspect of the
/// sentry's network security posture from inside the bottle.
pub enum rbida_Attack {
    /// DNS resolution of anthropic.com should succeed (allowed domain)
    DnsAllowedAnthropic,
    /// DNS resolution of google.com should fail (blocked domain)
    DnsBlockedGoogle,
    /// apt-get update should fail (package repos unreachable)
    AptGetBlocked,
}

// ── Verdict ─────────────────────────────────────────────────────

/// Result of running one attack.
pub struct rbida_Verdict {
    pub passed: bool,
    pub detail: String,
}

// ── Selector Mapping ────────────────────────────────────────────

impl rbida_Attack {
    /// Parse a kebab-case selector string into an attack variant.
    pub fn from_selector(s: &str) -> Option<Self> {
        match s {
            "dns-allowed-anthropic" => Some(Self::DnsAllowedAnthropic),
            "dns-blocked-google" => Some(Self::DnsBlockedGoogle),
            "apt-get-blocked" => Some(Self::AptGetBlocked),
            _ => None,
        }
    }

    /// Kebab-case selector for this attack (inverse of from_selector).
    pub fn selector(&self) -> &'static str {
        match self {
            Self::DnsAllowedAnthropic => "dns-allowed-anthropic",
            Self::DnsBlockedGoogle => "dns-blocked-google",
            Self::AptGetBlocked => "apt-get-blocked",
        }
    }

    /// All known attack selectors, in definition order.
    pub fn all_selectors() -> &'static [&'static str] {
        &[
            "dns-allowed-anthropic",
            "dns-blocked-google",
            "apt-get-blocked",
        ]
    }
}

// ── Dispatch ────────────────────────────────────────────────────

/// Run the specified attack and return its verdict.
/// Exhaustive match — adding a variant without handling is a compile error.
pub fn rbida_run(attack: &rbida_Attack) -> rbida_Verdict {
    match attack {
        rbida_Attack::DnsAllowedAnthropic => rbida_expect_command_succeeds(
            "getent",
            &["hosts", "anthropic.com"],
            "DNS resolution of anthropic.com (allowed domain)",
        ),
        rbida_Attack::DnsBlockedGoogle => rbida_expect_command_fails(
            "getent",
            &["hosts", "google.com"],
            "DNS resolution of google.com (blocked domain)",
        ),
        rbida_Attack::AptGetBlocked => rbida_expect_command_fails(
            "timeout",
            &["5", "apt-get", "-qq", "update"],
            "apt-get update (package repos unreachable)",
        ),
    }
}

// ── Attack Helpers ──────────────────────────────────────────────

/// Run a command and expect it to succeed (exit 0).
/// PASS when the security boundary correctly allows the operation.
fn rbida_expect_command_succeeds(cmd: &str, args: &[&str], description: &str) -> rbida_Verdict {
    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            if output.status.success() {
                rbida_Verdict {
                    passed: true,
                    detail: format!("SECURE: {}", description),
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                rbida_Verdict {
                    passed: false,
                    detail: format!(
                        "BREACH: {} — command failed (exit {}): {}",
                        description,
                        output.status.code().unwrap_or(-1),
                        stderr.trim()
                    ),
                }
            }
        }
        Err(e) => rbida_Verdict {
            passed: false,
            detail: format!("ERROR: {} — failed to execute '{}': {}", description, cmd, e),
        },
    }
}

/// Run a command and expect it to fail (nonzero exit).
/// PASS when the security boundary correctly blocks the operation.
fn rbida_expect_command_fails(cmd: &str, args: &[&str], description: &str) -> rbida_Verdict {
    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            if output.status.success() {
                rbida_Verdict {
                    passed: false,
                    detail: format!("BREACH: {} — command succeeded unexpectedly", description),
                }
            } else {
                rbida_Verdict {
                    passed: true,
                    detail: format!("SECURE: {}", description),
                }
            }
        }
        Err(e) => rbida_Verdict {
            passed: false,
            detail: format!("ERROR: {} — failed to execute '{}': {}", description, cmd, e),
        },
    }
}
