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

use crate::rbida_sorties;

// ── Selector constants (Single Definition Rule — RCG String Boundary Discipline) ──

const RBIDA_SEL_DNS_FORGE_RESPONSE: &str = "dns-forge-response";
const RBIDA_SEL_MAC_FLOOD_BRIDGE: &str = "mac-flood-bridge";

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
    /// Non-existent domain should fail to resolve
    DnsNonexistent,
    /// DNS over TCP should succeed for allowed domains
    DnsTcp,
    /// DNS over UDP should succeed for allowed domains
    DnsUdp,
    /// Direct external DNS queries should fail (both dig and nc)
    DnsBlockDirect,
    /// Alternate DNS port queries should fail
    DnsBlockAltport,
    /// Cloudflare DNS (1.1.1.1) should be blocked
    DnsBlockCloudflare,
    /// Quad9 DNS (9.9.9.9) should be blocked
    DnsBlockQuad9,
    /// Zone transfer attempts should fail
    DnsBlockZonetransfer,
    /// IPv6 DNS servers should be blocked
    DnsBlockIpv6,
    /// Multicast DNS should be blocked
    DnsBlockMulticast,
    /// DNS spoofing source IP should be blocked
    DnsBlockSpoofing,
    /// DNS tunneling via nc should be blocked
    DnsBlockTunneling,
    /// TCP 443 connection to IP should succeed (pass IP in extra_args[0])
    Tcp443Connect,
    /// TCP 443 connection to IP should fail (pass IP in extra_args[0])
    Tcp443Block,
    /// First traceroute hop should be sentry IP or blocked (* * *)
    IcmpFirstHop,
    /// Second traceroute hop should be blocked (* * *)
    IcmpSecondHopBlocked,
    // ── Ported python sorties (rbtis_*.py) ──
    /// DNS exfiltration via subdomain encoding of allowed domains
    DnsExfilSubdomain,
    /// Cloud metadata endpoint probe (169.254.169.254)
    MetaCloudEndpoint,
    /// TCP/UDP to non-allowed CIDRs
    NetForbiddenCidr,
    /// Sentry service enumeration (port scanning)
    DirectSentryProbe,
    /// ICMP covert channel via payload encoding
    IcmpExfilPayload,
    /// IPv6 unconfigured firewall escape
    NetIpv6Escape,
    /// Source IP spoofing via raw sockets
    NetSrcipSpoof,
    /// Protocol smuggling via raw sockets (GRE, SCTP, IP-in-IP)
    ProtoSmuggleRawsock,
    /// IP fragment reassembly bypass
    NetFragmentEvasion,
    /// ARP cache poisoning via AF_PACKET
    DirectArpPoison,
    /// Namespace and capability escape probe
    NsCapabilityEscape,
    // ── Coordinated attack primitives (theurge observes effect) ──
    /// Send gratuitous ARP claiming sentry IP — theurge checks sentry ARP table
    ArpSendGratuitous,
    /// Send targeted ARP reply poisoning gateway entry — theurge checks sentry ARP table
    ArpSendGatewayPoison,
    // ── Coordinated integrity primitives (theurge observes sentry state) ──
    /// Send forged DNS responses to sentry's dnsmasq — theurge checks DNS cache
    DnsForgeResponse,
    /// Flood bridge MAC table with random source MACs — theurge checks connectivity
    MacFloodBridge,
    // ── Novel unilateral attacks ──
    /// Route table manipulation — attempt ip route replace/add to bypass sentry gateway
    NetRouteManipulation,
    /// Enclave subnet escape — probe hosts outside /24 enclave within bridge network range
    NetEnclaveSubnetEscape,
    /// DNAT entry port reflection — TCP connect to sentry entry port from inside bottle
    NetDnatEntryReflection,
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
            "dns-nonexistent" => Some(Self::DnsNonexistent),
            "dns-tcp" => Some(Self::DnsTcp),
            "dns-udp" => Some(Self::DnsUdp),
            "dns-block-direct" => Some(Self::DnsBlockDirect),
            "dns-block-altport" => Some(Self::DnsBlockAltport),
            "dns-block-cloudflare" => Some(Self::DnsBlockCloudflare),
            "dns-block-quad9" => Some(Self::DnsBlockQuad9),
            "dns-block-zonetransfer" => Some(Self::DnsBlockZonetransfer),
            "dns-block-ipv6" => Some(Self::DnsBlockIpv6),
            "dns-block-multicast" => Some(Self::DnsBlockMulticast),
            "dns-block-spoofing" => Some(Self::DnsBlockSpoofing),
            "dns-block-tunneling" => Some(Self::DnsBlockTunneling),
            "tcp443-connect" => Some(Self::Tcp443Connect),
            "tcp443-block" => Some(Self::Tcp443Block),
            "icmp-first-hop" => Some(Self::IcmpFirstHop),
            "icmp-second-hop-blocked" => Some(Self::IcmpSecondHopBlocked),
            "dns-exfil-subdomain" => Some(Self::DnsExfilSubdomain),
            "meta-cloud-endpoint" => Some(Self::MetaCloudEndpoint),
            "net-forbidden-cidr" => Some(Self::NetForbiddenCidr),
            "direct-sentry-probe" => Some(Self::DirectSentryProbe),
            "icmp-exfil-payload" => Some(Self::IcmpExfilPayload),
            "net-ipv6-escape" => Some(Self::NetIpv6Escape),
            "net-srcip-spoof" => Some(Self::NetSrcipSpoof),
            "proto-smuggle-rawsock" => Some(Self::ProtoSmuggleRawsock),
            "net-fragment-evasion" => Some(Self::NetFragmentEvasion),
            "direct-arp-poison" => Some(Self::DirectArpPoison),
            "ns-capability-escape" => Some(Self::NsCapabilityEscape),
            "arp-send-gratuitous" => Some(Self::ArpSendGratuitous),
            "arp-send-gateway-poison" => Some(Self::ArpSendGatewayPoison),
            RBIDA_SEL_DNS_FORGE_RESPONSE => Some(Self::DnsForgeResponse),
            RBIDA_SEL_MAC_FLOOD_BRIDGE => Some(Self::MacFloodBridge),
            "net-route-manipulation" => Some(Self::NetRouteManipulation),
            "net-enclave-subnet-escape" => Some(Self::NetEnclaveSubnetEscape),
            "net-dnat-entry-reflection" => Some(Self::NetDnatEntryReflection),
            _ => None,
        }
    }

    /// Kebab-case selector for this attack (inverse of from_selector).
    pub fn selector(&self) -> &'static str {
        match self {
            Self::DnsAllowedAnthropic => "dns-allowed-anthropic",
            Self::DnsBlockedGoogle => "dns-blocked-google",
            Self::AptGetBlocked => "apt-get-blocked",
            Self::DnsNonexistent => "dns-nonexistent",
            Self::DnsTcp => "dns-tcp",
            Self::DnsUdp => "dns-udp",
            Self::DnsBlockDirect => "dns-block-direct",
            Self::DnsBlockAltport => "dns-block-altport",
            Self::DnsBlockCloudflare => "dns-block-cloudflare",
            Self::DnsBlockQuad9 => "dns-block-quad9",
            Self::DnsBlockZonetransfer => "dns-block-zonetransfer",
            Self::DnsBlockIpv6 => "dns-block-ipv6",
            Self::DnsBlockMulticast => "dns-block-multicast",
            Self::DnsBlockSpoofing => "dns-block-spoofing",
            Self::DnsBlockTunneling => "dns-block-tunneling",
            Self::Tcp443Connect => "tcp443-connect",
            Self::Tcp443Block => "tcp443-block",
            Self::IcmpFirstHop => "icmp-first-hop",
            Self::IcmpSecondHopBlocked => "icmp-second-hop-blocked",
            Self::DnsExfilSubdomain => "dns-exfil-subdomain",
            Self::MetaCloudEndpoint => "meta-cloud-endpoint",
            Self::NetForbiddenCidr => "net-forbidden-cidr",
            Self::DirectSentryProbe => "direct-sentry-probe",
            Self::IcmpExfilPayload => "icmp-exfil-payload",
            Self::NetIpv6Escape => "net-ipv6-escape",
            Self::NetSrcipSpoof => "net-srcip-spoof",
            Self::ProtoSmuggleRawsock => "proto-smuggle-rawsock",
            Self::NetFragmentEvasion => "net-fragment-evasion",
            Self::DirectArpPoison => "direct-arp-poison",
            Self::NsCapabilityEscape => "ns-capability-escape",
            Self::ArpSendGratuitous => "arp-send-gratuitous",
            Self::ArpSendGatewayPoison => "arp-send-gateway-poison",
            Self::DnsForgeResponse => RBIDA_SEL_DNS_FORGE_RESPONSE,
            Self::MacFloodBridge => RBIDA_SEL_MAC_FLOOD_BRIDGE,
            Self::NetRouteManipulation => "net-route-manipulation",
            Self::NetEnclaveSubnetEscape => "net-enclave-subnet-escape",
            Self::NetDnatEntryReflection => "net-dnat-entry-reflection",
        }
    }

    /// All known attack selectors, in definition order.
    pub fn all_selectors() -> &'static [&'static str] {
        &[
            "dns-allowed-anthropic",
            "dns-blocked-google",
            "apt-get-blocked",
            "dns-nonexistent",
            "dns-tcp",
            "dns-udp",
            "dns-block-direct",
            "dns-block-altport",
            "dns-block-cloudflare",
            "dns-block-quad9",
            "dns-block-zonetransfer",
            "dns-block-ipv6",
            "dns-block-multicast",
            "dns-block-spoofing",
            "dns-block-tunneling",
            "tcp443-connect",
            "tcp443-block",
            "icmp-first-hop",
            "icmp-second-hop-blocked",
            "dns-exfil-subdomain",
            "meta-cloud-endpoint",
            "net-forbidden-cidr",
            "direct-sentry-probe",
            "icmp-exfil-payload",
            "net-ipv6-escape",
            "net-srcip-spoof",
            "proto-smuggle-rawsock",
            "net-fragment-evasion",
            "direct-arp-poison",
            "ns-capability-escape",
            "arp-send-gratuitous",
            "arp-send-gateway-poison",
            RBIDA_SEL_DNS_FORGE_RESPONSE,
            RBIDA_SEL_MAC_FLOOD_BRIDGE,
            "net-route-manipulation",
            "net-enclave-subnet-escape",
            "net-dnat-entry-reflection",
        ]
    }
}

// ── Dispatch ────────────────────────────────────────────────────

/// Run the specified attack and return its verdict.
/// Exhaustive match — adding a variant without handling is a compile error.
/// extra_args: optional additional parameters (e.g. IP address for tcp443 attacks)
pub fn rbida_run(attack: &rbida_Attack, extra_args: &[&str]) -> rbida_Verdict {
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
        rbida_Attack::DnsNonexistent => rbida_expect_command_fails(
            "getent",
            &["hosts", "nonexistentdomain123.test"],
            "DNS resolution of nonexistentdomain123.test (non-existent domain)",
        ),
        rbida_Attack::DnsTcp => rbida_expect_command_succeeds(
            "dig",
            &["+tcp", "anthropic.com"],
            "DNS over TCP for anthropic.com (allowed domain)",
        ),
        rbida_Attack::DnsUdp => rbida_expect_command_succeeds(
            "dig",
            &["+notcp", "anthropic.com"],
            "DNS over UDP for anthropic.com (allowed domain)",
        ),
        rbida_Attack::DnsBlockDirect => rbida_expect_all_fail(
            &[
                ("dig", &["@8.8.8.8", "anthropic.com"] as &[&str]),
                ("nc", &["-w", "2", "-zv", "8.8.8.8", "53"]),
            ],
            "direct external DNS query bypass (dig @8.8.8.8 and nc 8.8.8.8:53)",
        ),
        rbida_Attack::DnsBlockAltport => rbida_expect_all_fail(
            &[
                ("dig", &["@8.8.8.8", "-p", "5353", "example.com"] as &[&str]),
                ("dig", &["@8.8.8.8", "-p", "443", "example.com"]),
            ],
            "alternate DNS port bypass (ports 5353 and 443)",
        ),
        rbida_Attack::DnsBlockCloudflare => rbida_expect_command_fails(
            "dig",
            &["@1.1.1.1", "example.com"],
            "Cloudflare DNS bypass (1.1.1.1)",
        ),
        rbida_Attack::DnsBlockQuad9 => rbida_expect_command_fails(
            "dig",
            &["@9.9.9.9", "example.com"],
            "Quad9 DNS bypass (9.9.9.9)",
        ),
        rbida_Attack::DnsBlockZonetransfer => rbida_expect_command_fails(
            "dig",
            &["@8.8.8.8", "example.com", "AXFR"],
            "DNS zone transfer attempt",
        ),
        rbida_Attack::DnsBlockIpv6 => rbida_expect_command_fails(
            "dig",
            &["@2001:4860:4860::8888", "example.com"],
            "IPv6 DNS server bypass (2001:4860:4860::8888)",
        ),
        rbida_Attack::DnsBlockMulticast => rbida_expect_command_fails(
            "dig",
            &["@224.0.0.251", "-p", "5353", "example.local"],
            "multicast DNS bypass (224.0.0.251:5353)",
        ),
        rbida_Attack::DnsBlockSpoofing => rbida_expect_command_fails(
            "dig",
            &["@8.8.8.8", "+nsid", "example.com", "-b", "192.168.1.2"],
            "DNS spoofing source IP bypass",
        ),
        rbida_Attack::DnsBlockTunneling => rbida_expect_command_fails(
            "nc",
            &["-z", "-w", "1", "8.8.8.8", "53"],
            "DNS tunneling via nc to 8.8.8.8:53",
        ),
        rbida_Attack::Tcp443Connect => {
            let ip = extra_args.first().copied().unwrap_or("");
            if ip.is_empty() {
                return rbida_Verdict {
                    passed: false,
                    detail: "ERROR: tcp443-connect requires IP address as extra arg".to_string(),
                };
            }
            rbida_expect_command_succeeds(
                "nc",
                &["-w", "2", "-zv", ip, "443"],
                &format!("TCP 443 connection to {} (should be allowed)", ip),
            )
        }
        rbida_Attack::Tcp443Block => {
            let ip = extra_args.first().copied().unwrap_or("");
            if ip.is_empty() {
                return rbida_Verdict {
                    passed: false,
                    detail: "ERROR: tcp443-block requires IP address as extra arg".to_string(),
                };
            }
            rbida_expect_command_fails(
                "nc",
                &["-w", "2", "-zv", ip, "443"],
                &format!("TCP 443 connection to {} (should be blocked)", ip),
            )
        }
        rbida_Attack::IcmpFirstHop => rbida_check_icmp_first_hop(),
        rbida_Attack::IcmpSecondHopBlocked => rbida_check_icmp_second_hop_blocked(),
        // Ported python sorties
        rbida_Attack::DnsExfilSubdomain => rbida_sorties::sortie_dns_exfil_subdomain(extra_args),
        rbida_Attack::MetaCloudEndpoint => rbida_sorties::sortie_meta_cloud_endpoint(extra_args),
        rbida_Attack::NetForbiddenCidr => rbida_sorties::sortie_net_forbidden_cidr(extra_args),
        rbida_Attack::DirectSentryProbe => rbida_sorties::sortie_direct_sentry_probe(extra_args),
        rbida_Attack::IcmpExfilPayload => rbida_sorties::sortie_icmp_exfil_payload(extra_args),
        rbida_Attack::NetIpv6Escape => rbida_sorties::sortie_net_ipv6_escape(extra_args),
        rbida_Attack::NetSrcipSpoof => rbida_sorties::sortie_net_srcip_spoof(extra_args),
        rbida_Attack::ProtoSmuggleRawsock => rbida_sorties::sortie_proto_smuggle_rawsock(extra_args),
        rbida_Attack::NetFragmentEvasion => rbida_sorties::sortie_net_fragment_evasion(extra_args),
        rbida_Attack::DirectArpPoison => rbida_sorties::sortie_direct_arp_poison(extra_args),
        rbida_Attack::NsCapabilityEscape => rbida_sorties::sortie_ns_capability_escape(extra_args),
        // Coordinated attack primitives — execute action, theurge judges outcome
        rbida_Attack::ArpSendGratuitous => rbida_sorties::sortie_arp_send_gratuitous(extra_args),
        rbida_Attack::ArpSendGatewayPoison => rbida_sorties::sortie_arp_send_gateway_poison(extra_args),
        // Coordinated integrity primitives — execute action, theurge judges state
        rbida_Attack::DnsForgeResponse => rbida_sorties::sortie_dns_forge_response(extra_args),
        rbida_Attack::MacFloodBridge => rbida_sorties::sortie_mac_flood_bridge(extra_args),
        // Novel unilateral attacks
        rbida_Attack::NetRouteManipulation => rbida_sorties::sortie_net_route_manipulation(extra_args),
        rbida_Attack::NetEnclaveSubnetEscape => rbida_sorties::sortie_net_enclave_subnet_escape(extra_args),
        rbida_Attack::NetDnatEntryReflection => rbida_sorties::sortie_net_dnat_entry_reflection(extra_args),
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

/// Run multiple commands and require ALL to fail (nonzero exit).
/// PASS only when the security boundary blocks every attempted bypass.
fn rbida_expect_all_fail(checks: &[(&str, &[&str])], description: &str) -> rbida_Verdict {
    for (cmd, args) in checks {
        match Command::new(cmd).args(*args).output() {
            Ok(output) => {
                if output.status.success() {
                    return rbida_Verdict {
                        passed: false,
                        detail: format!(
                            "BREACH: {} — '{}' succeeded unexpectedly",
                            description, cmd
                        ),
                    };
                }
                // This command failed as expected; continue checking the rest
            }
            Err(e) => {
                return rbida_Verdict {
                    passed: false,
                    detail: format!(
                        "ERROR: {} — failed to execute '{}': {}",
                        description, cmd, e
                    ),
                };
            }
        }
    }
    rbida_Verdict {
        passed: true,
        detail: format!("SECURE: {}", description),
    }
}

/// Run traceroute -I -m 1 and verify first hop is sentry IP or * * *.
/// Reads sentry IP from /etc/resolv.conf nameserver line.
fn rbida_check_icmp_first_hop() -> rbida_Verdict {
    // Discover sentry IP from resolv.conf
    let sentry_ip = match rbida_read_nameserver() {
        Ok(ip) => ip,
        Err(e) => {
            return rbida_Verdict {
                passed: false,
                detail: format!("ERROR: icmp-first-hop — cannot read sentry IP: {}", e),
            }
        }
    };

    let output = match Command::new("traceroute")
        .args(["-I", "-m", "1", "8.8.8.8"])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            return rbida_Verdict {
                passed: false,
                detail: format!("ERROR: icmp-first-hop — failed to execute traceroute: {}", e),
            }
        }
    };

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    // Accept sentry IP visible OR fully blocked (* * *)
    if combined.contains(&sentry_ip) {
        return rbida_Verdict {
            passed: true,
            detail: format!(
                "SECURE: icmp-first-hop — sentry IP {} visible in traceroute",
                sentry_ip
            ),
        };
    }

    // Check for "1  * * *" pattern (blocked at first hop)
    for line in combined.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("1 ") || trimmed.starts_with("1\t") {
            if trimmed.contains("* * *") {
                return rbida_Verdict {
                    passed: true,
                    detail: "SECURE: icmp-first-hop — first hop blocked (* * *)".to_string(),
                };
            }
        }
    }

    rbida_Verdict {
        passed: false,
        detail: format!(
            "BREACH: icmp-first-hop — unexpected traceroute output (expected sentry IP {} or * * *):\n{}",
            sentry_ip, combined
        ),
    }
}

/// Run traceroute -I -m 2 and verify second hop is * * * (blocked).
fn rbida_check_icmp_second_hop_blocked() -> rbida_Verdict {
    let output = match Command::new("traceroute")
        .args(["-I", "-m", "2", "8.8.8.8"])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            return rbida_Verdict {
                passed: false,
                detail: format!(
                    "ERROR: icmp-second-hop-blocked — failed to execute traceroute: {}",
                    e
                ),
            }
        }
    };

    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    // Check for "2  * * *" pattern
    for line in combined.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("2 ") || trimmed.starts_with("2\t") {
            if trimmed.contains("* * *") {
                return rbida_Verdict {
                    passed: true,
                    detail: "SECURE: icmp-second-hop-blocked — second hop blocked (* * *)".to_string(),
                };
            }
        }
    }

    rbida_Verdict {
        passed: false,
        detail: format!(
            "BREACH: icmp-second-hop-blocked — expected blocked second hop (* * *) in traceroute:\n{}",
            combined
        ),
    }
}

/// Read the nameserver IP from /etc/resolv.conf.
/// Returns the first nameserver entry found.
fn rbida_read_nameserver() -> Result<String, String> {
    let content = std::fs::read_to_string("/etc/resolv.conf")
        .map_err(|e| format!("cannot read /etc/resolv.conf: {}", e))?;

    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("nameserver") {
            let ip = rest.trim();
            if !ip.is_empty() {
                return Ok(ip.to_string());
            }
        }
    }

    Err("no nameserver entry found in /etc/resolv.conf".to_string())
}
