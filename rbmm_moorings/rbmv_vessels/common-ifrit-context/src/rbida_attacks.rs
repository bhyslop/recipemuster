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
// RBIDA — attack definitions and dispatch for ifrit
//
// Each variant of rbtica_Attack represents one security boundary probe.
// Exhaustive match in rbtica_run ensures adding a variant forces handling.
// Attacks shell out to system commands available in the ifrit vessel image.

use std::process::Command;

use crate::rbtics_sorties;

// ── Domain constants ──

/// Test connectivity target — ICANN-owned, stable single /20 CIDR (192.0.32.0/20)
pub const RBTICA_CONNECTIVITY_DOMAIN: &str = "www.internic.net";

// ── Selector constants ──

const RBTICA_SEL_DNS_ALLOWED_EXAMPLE: &str = "dns-allowed-example";
const RBTICA_SEL_DNS_ALLOWED_EXAMPLE_ORG: &str = "dns-allowed-example-org";
const RBTICA_SEL_DNS_BLOCKED_GOOGLE: &str = "dns-blocked-google";
const RBTICA_SEL_APT_GET_BLOCKED: &str = "apt-get-blocked";
const RBTICA_SEL_DNS_NONEXISTENT: &str = "dns-nonexistent";
const RBTICA_SEL_DNS_TCP: &str = "dns-tcp";
const RBTICA_SEL_DNS_UDP: &str = "dns-udp";
const RBTICA_SEL_DNS_BLOCK_DIRECT: &str = "dns-block-direct";
const RBTICA_SEL_DNS_BLOCK_ALTPORT: &str = "dns-block-altport";
const RBTICA_SEL_DNS_BLOCK_CLOUDFLARE: &str = "dns-block-cloudflare";
const RBTICA_SEL_DNS_BLOCK_QUAD9: &str = "dns-block-quad9";
const RBTICA_SEL_DNS_BLOCK_ZONETRANSFER: &str = "dns-block-zonetransfer";
const RBTICA_SEL_DNS_BLOCK_IPV6: &str = "dns-block-ipv6";
const RBTICA_SEL_DNS_BLOCK_MULTICAST: &str = "dns-block-multicast";
const RBTICA_SEL_DNS_BLOCK_SPOOFING: &str = "dns-block-spoofing";
const RBTICA_SEL_DNS_BLOCK_TUNNELING: &str = "dns-block-tunneling";
const RBTICA_SEL_TCP443_CONNECT: &str = "tcp443-connect";
const RBTICA_SEL_TCP443_BLOCK: &str = "tcp443-block";
const RBTICA_SEL_ICMP_FIRST_HOP: &str = "icmp-first-hop";
const RBTICA_SEL_ICMP_SECOND_HOP_BLOCKED: &str = "icmp-second-hop-blocked";
const RBTICA_SEL_DNS_EXFIL_SUBDOMAIN: &str = "dns-exfil-subdomain";
const RBTICA_SEL_META_CLOUD_ENDPOINT: &str = "meta-cloud-endpoint";
const RBTICA_SEL_NET_FORBIDDEN_CIDR: &str = "net-forbidden-cidr";
const RBTICA_SEL_DIRECT_SENTRY_PROBE: &str = "direct-sentry-probe";
const RBTICA_SEL_ICMP_EXFIL_PAYLOAD: &str = "icmp-exfil-payload";
const RBTICA_SEL_NET_IPV6_ESCAPE: &str = "net-ipv6-escape";
const RBTICA_SEL_NET_SRCIP_SPOOF: &str = "net-srcip-spoof";
const RBTICA_SEL_NET_SRCIP_SPOOF_EXTERNAL: &str = "net-srcip-spoof-external";
const RBTICA_SEL_PROTO_SMUGGLE_RAWSOCK: &str = "proto-smuggle-rawsock";
const RBTICA_SEL_NET_FRAGMENT_EVASION: &str = "net-fragment-evasion";
const RBTICA_SEL_DIRECT_ARP_POISON: &str = "direct-arp-poison";
const RBTICA_SEL_NS_CAPABILITY_ESCAPE: &str = "ns-capability-escape";
const RBTICA_SEL_ARP_SEND_GRATUITOUS: &str = "arp-send-gratuitous";
const RBTICA_SEL_ARP_SEND_GATEWAY_POISON: &str = "arp-send-gateway-poison";
const RBTICA_SEL_DNS_FORGE_RESPONSE: &str = "dns-forge-response";
const RBTICA_SEL_MAC_FLOOD_BRIDGE: &str = "mac-flood-bridge";
const RBTICA_SEL_NET_ROUTE_MANIPULATION: &str = "net-route-manipulation";
const RBTICA_SEL_NET_ENCLAVE_SUBNET_ESCAPE: &str = "net-enclave-subnet-escape";
const RBTICA_SEL_NET_DNAT_ENTRY_REFLECTION: &str = "net-dnat-entry-reflection";
const RBTICA_SEL_UDP_NON_DNS_BLOCKED: &str = "udp-non-dns-blocked";
const RBTICA_SEL_CIDR_ALL_PORTS_ALLOWED: &str = "cidr-all-ports-allowed";
const RBTICA_SEL_DNS_REBINDING: &str = "dns-rebinding";
const RBTICA_SEL_PROC_SYS_WRITE: &str = "proc-sys-write";
const RBTICA_SEL_TCP_RST_HIJACK: &str = "tcp-rst-hijack";
const RBTICA_SEL_HTTP_END_TO_END: &str = "http-end-to-end";
const RBTICA_SEL_CONNTRACK_SPOOFED_ACK: &str = "conntrack-spoofed-ack";
const RBTICA_SEL_OFFPATH_BLOCKED_DEST: &str = "offpath-blocked-dest";
const RBTICA_SEL_CONNTRACK_PIPELINE_SELFCHECK: &str = "conntrack-pipeline-selfcheck";
const RBTICA_SEL_SENTRY_UDP_NON_DNS: &str = "sentry-udp-non-dns";

// ── Attack Enum ─────────────────────────────────────────────────

/// Security boundary attack. Each variant probes one aspect of the
/// sentry's network security posture from inside the bottle.
pub enum rbtica_Attack {
    /// DNS resolution of www.internic.net should succeed (allowed domain)
    DnsAllowedExample,
    /// DNS resolution of example.org should succeed (second allowed domain — exercises list treatment)
    DnsAllowedExampleOrg,
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
    /// Spoof source as arbitrary external-routable IP, target sentry's entry
    /// port — probes whether per-IP RETURN exclusion + rp_filter=2 loose
    /// allows DNAT-reflection back to the bottle
    NetSrcipSpoofExternal,
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
    // ── Egress control verification ──
    /// UDP datagram to non-allowed IP on non-DNS port should be blocked
    UdpNonDnsBlocked,
    /// TCP to allowed CIDR on multiple ports should succeed (CIDR is protocol-agnostic)
    CidrAllPortsAllowed,
    // ── Advanced adversarial probes ──
    /// DNS rebinding — re-resolve allowed domain to check dnsmasq cache manipulation
    DnsRebinding,
    /// Kernel tunable writes — attempt to modify /proc/sys network parameters
    ProcSysWrite,
    /// TCP RST connection hijack — forge RST packets targeting sentry DNS connection
    TcpRstHijack,
    // ── Network path verification ──
    /// Full HTTP GET from bottle to www.internic.net — proves NAT masquerade returns actual data
    HttpEndToEnd,
    /// Spoofed ACK without prior SYN — conntrack RELATED,ESTABLISHED should drop it
    ConntrackSpoofedAck,
    /// Lone ACK to a blocked destination — negative control proving the
    /// rbsiq_wdd_offpath_reply quirk's "blocked dests are dropped before the
    /// substrate sees them" premise; any reply of any provenance is a BREACH
    OffpathBlockedDest,
    /// Self-check of the conntrack provenance capture/classify pipeline — feeds
    /// rbtics_inspect_capture_frame synthetic frames and asserts correct classification.
    /// Load-bearing control proving SECURE verdicts are not masking a dead detector.
    ConntrackPipelineSelfcheck,
    // ── Sentry self-protection ──
    /// UDP to sentry on non-53 ports — INPUT DROP should block all non-DNS UDP
    SentryUdpNonDns,
}

// ── Verdict ─────────────────────────────────────────────────────

/// Result of running one attack.
pub struct rbtica_Verdict {
    pub passed: bool,
    pub detail: String,
}

// ── Selector Mapping ────────────────────────────────────────────

impl rbtica_Attack {
    /// Parse a kebab-case selector string into an attack variant.
    pub fn rbtica_from_selector(s: &str) -> Option<Self> {
        match s {
            RBTICA_SEL_DNS_ALLOWED_EXAMPLE => Some(Self::DnsAllowedExample),
            RBTICA_SEL_DNS_ALLOWED_EXAMPLE_ORG => Some(Self::DnsAllowedExampleOrg),
            RBTICA_SEL_DNS_BLOCKED_GOOGLE => Some(Self::DnsBlockedGoogle),
            RBTICA_SEL_APT_GET_BLOCKED => Some(Self::AptGetBlocked),
            RBTICA_SEL_DNS_NONEXISTENT => Some(Self::DnsNonexistent),
            RBTICA_SEL_DNS_TCP => Some(Self::DnsTcp),
            RBTICA_SEL_DNS_UDP => Some(Self::DnsUdp),
            RBTICA_SEL_DNS_BLOCK_DIRECT => Some(Self::DnsBlockDirect),
            RBTICA_SEL_DNS_BLOCK_ALTPORT => Some(Self::DnsBlockAltport),
            RBTICA_SEL_DNS_BLOCK_CLOUDFLARE => Some(Self::DnsBlockCloudflare),
            RBTICA_SEL_DNS_BLOCK_QUAD9 => Some(Self::DnsBlockQuad9),
            RBTICA_SEL_DNS_BLOCK_ZONETRANSFER => Some(Self::DnsBlockZonetransfer),
            RBTICA_SEL_DNS_BLOCK_IPV6 => Some(Self::DnsBlockIpv6),
            RBTICA_SEL_DNS_BLOCK_MULTICAST => Some(Self::DnsBlockMulticast),
            RBTICA_SEL_DNS_BLOCK_SPOOFING => Some(Self::DnsBlockSpoofing),
            RBTICA_SEL_DNS_BLOCK_TUNNELING => Some(Self::DnsBlockTunneling),
            RBTICA_SEL_TCP443_CONNECT => Some(Self::Tcp443Connect),
            RBTICA_SEL_TCP443_BLOCK => Some(Self::Tcp443Block),
            RBTICA_SEL_ICMP_FIRST_HOP => Some(Self::IcmpFirstHop),
            RBTICA_SEL_ICMP_SECOND_HOP_BLOCKED => Some(Self::IcmpSecondHopBlocked),
            RBTICA_SEL_DNS_EXFIL_SUBDOMAIN => Some(Self::DnsExfilSubdomain),
            RBTICA_SEL_META_CLOUD_ENDPOINT => Some(Self::MetaCloudEndpoint),
            RBTICA_SEL_NET_FORBIDDEN_CIDR => Some(Self::NetForbiddenCidr),
            RBTICA_SEL_DIRECT_SENTRY_PROBE => Some(Self::DirectSentryProbe),
            RBTICA_SEL_ICMP_EXFIL_PAYLOAD => Some(Self::IcmpExfilPayload),
            RBTICA_SEL_NET_IPV6_ESCAPE => Some(Self::NetIpv6Escape),
            RBTICA_SEL_NET_SRCIP_SPOOF => Some(Self::NetSrcipSpoof),
            RBTICA_SEL_NET_SRCIP_SPOOF_EXTERNAL => Some(Self::NetSrcipSpoofExternal),
            RBTICA_SEL_PROTO_SMUGGLE_RAWSOCK => Some(Self::ProtoSmuggleRawsock),
            RBTICA_SEL_NET_FRAGMENT_EVASION => Some(Self::NetFragmentEvasion),
            RBTICA_SEL_DIRECT_ARP_POISON => Some(Self::DirectArpPoison),
            RBTICA_SEL_NS_CAPABILITY_ESCAPE => Some(Self::NsCapabilityEscape),
            RBTICA_SEL_ARP_SEND_GRATUITOUS => Some(Self::ArpSendGratuitous),
            RBTICA_SEL_ARP_SEND_GATEWAY_POISON => Some(Self::ArpSendGatewayPoison),
            RBTICA_SEL_DNS_FORGE_RESPONSE => Some(Self::DnsForgeResponse),
            RBTICA_SEL_MAC_FLOOD_BRIDGE => Some(Self::MacFloodBridge),
            RBTICA_SEL_NET_ROUTE_MANIPULATION => Some(Self::NetRouteManipulation),
            RBTICA_SEL_NET_ENCLAVE_SUBNET_ESCAPE => Some(Self::NetEnclaveSubnetEscape),
            RBTICA_SEL_NET_DNAT_ENTRY_REFLECTION => Some(Self::NetDnatEntryReflection),
            RBTICA_SEL_UDP_NON_DNS_BLOCKED => Some(Self::UdpNonDnsBlocked),
            RBTICA_SEL_CIDR_ALL_PORTS_ALLOWED => Some(Self::CidrAllPortsAllowed),
            RBTICA_SEL_DNS_REBINDING => Some(Self::DnsRebinding),
            RBTICA_SEL_PROC_SYS_WRITE => Some(Self::ProcSysWrite),
            RBTICA_SEL_TCP_RST_HIJACK => Some(Self::TcpRstHijack),
            RBTICA_SEL_HTTP_END_TO_END => Some(Self::HttpEndToEnd),
            RBTICA_SEL_CONNTRACK_SPOOFED_ACK => Some(Self::ConntrackSpoofedAck),
            RBTICA_SEL_OFFPATH_BLOCKED_DEST => Some(Self::OffpathBlockedDest),
            RBTICA_SEL_CONNTRACK_PIPELINE_SELFCHECK => Some(Self::ConntrackPipelineSelfcheck),
            RBTICA_SEL_SENTRY_UDP_NON_DNS => Some(Self::SentryUdpNonDns),
            _ => None,
        }
    }

    /// Kebab-case selector for this attack (inverse of rbtica_from_selector).
    pub fn rbtica_selector(&self) -> &'static str {
        match self {
            Self::DnsAllowedExample => RBTICA_SEL_DNS_ALLOWED_EXAMPLE,
            Self::DnsAllowedExampleOrg => RBTICA_SEL_DNS_ALLOWED_EXAMPLE_ORG,
            Self::DnsBlockedGoogle => RBTICA_SEL_DNS_BLOCKED_GOOGLE,
            Self::AptGetBlocked => RBTICA_SEL_APT_GET_BLOCKED,
            Self::DnsNonexistent => RBTICA_SEL_DNS_NONEXISTENT,
            Self::DnsTcp => RBTICA_SEL_DNS_TCP,
            Self::DnsUdp => RBTICA_SEL_DNS_UDP,
            Self::DnsBlockDirect => RBTICA_SEL_DNS_BLOCK_DIRECT,
            Self::DnsBlockAltport => RBTICA_SEL_DNS_BLOCK_ALTPORT,
            Self::DnsBlockCloudflare => RBTICA_SEL_DNS_BLOCK_CLOUDFLARE,
            Self::DnsBlockQuad9 => RBTICA_SEL_DNS_BLOCK_QUAD9,
            Self::DnsBlockZonetransfer => RBTICA_SEL_DNS_BLOCK_ZONETRANSFER,
            Self::DnsBlockIpv6 => RBTICA_SEL_DNS_BLOCK_IPV6,
            Self::DnsBlockMulticast => RBTICA_SEL_DNS_BLOCK_MULTICAST,
            Self::DnsBlockSpoofing => RBTICA_SEL_DNS_BLOCK_SPOOFING,
            Self::DnsBlockTunneling => RBTICA_SEL_DNS_BLOCK_TUNNELING,
            Self::Tcp443Connect => RBTICA_SEL_TCP443_CONNECT,
            Self::Tcp443Block => RBTICA_SEL_TCP443_BLOCK,
            Self::IcmpFirstHop => RBTICA_SEL_ICMP_FIRST_HOP,
            Self::IcmpSecondHopBlocked => RBTICA_SEL_ICMP_SECOND_HOP_BLOCKED,
            Self::DnsExfilSubdomain => RBTICA_SEL_DNS_EXFIL_SUBDOMAIN,
            Self::MetaCloudEndpoint => RBTICA_SEL_META_CLOUD_ENDPOINT,
            Self::NetForbiddenCidr => RBTICA_SEL_NET_FORBIDDEN_CIDR,
            Self::DirectSentryProbe => RBTICA_SEL_DIRECT_SENTRY_PROBE,
            Self::IcmpExfilPayload => RBTICA_SEL_ICMP_EXFIL_PAYLOAD,
            Self::NetIpv6Escape => RBTICA_SEL_NET_IPV6_ESCAPE,
            Self::NetSrcipSpoof => RBTICA_SEL_NET_SRCIP_SPOOF,
            Self::NetSrcipSpoofExternal => RBTICA_SEL_NET_SRCIP_SPOOF_EXTERNAL,
            Self::ProtoSmuggleRawsock => RBTICA_SEL_PROTO_SMUGGLE_RAWSOCK,
            Self::NetFragmentEvasion => RBTICA_SEL_NET_FRAGMENT_EVASION,
            Self::DirectArpPoison => RBTICA_SEL_DIRECT_ARP_POISON,
            Self::NsCapabilityEscape => RBTICA_SEL_NS_CAPABILITY_ESCAPE,
            Self::ArpSendGratuitous => RBTICA_SEL_ARP_SEND_GRATUITOUS,
            Self::ArpSendGatewayPoison => RBTICA_SEL_ARP_SEND_GATEWAY_POISON,
            Self::DnsForgeResponse => RBTICA_SEL_DNS_FORGE_RESPONSE,
            Self::MacFloodBridge => RBTICA_SEL_MAC_FLOOD_BRIDGE,
            Self::NetRouteManipulation => RBTICA_SEL_NET_ROUTE_MANIPULATION,
            Self::NetEnclaveSubnetEscape => RBTICA_SEL_NET_ENCLAVE_SUBNET_ESCAPE,
            Self::NetDnatEntryReflection => RBTICA_SEL_NET_DNAT_ENTRY_REFLECTION,
            Self::UdpNonDnsBlocked => RBTICA_SEL_UDP_NON_DNS_BLOCKED,
            Self::CidrAllPortsAllowed => RBTICA_SEL_CIDR_ALL_PORTS_ALLOWED,
            Self::DnsRebinding => RBTICA_SEL_DNS_REBINDING,
            Self::ProcSysWrite => RBTICA_SEL_PROC_SYS_WRITE,
            Self::TcpRstHijack => RBTICA_SEL_TCP_RST_HIJACK,
            Self::HttpEndToEnd => RBTICA_SEL_HTTP_END_TO_END,
            Self::ConntrackSpoofedAck => RBTICA_SEL_CONNTRACK_SPOOFED_ACK,
            Self::OffpathBlockedDest => RBTICA_SEL_OFFPATH_BLOCKED_DEST,
            Self::ConntrackPipelineSelfcheck => RBTICA_SEL_CONNTRACK_PIPELINE_SELFCHECK,
            Self::SentryUdpNonDns => RBTICA_SEL_SENTRY_UDP_NON_DNS,
        }
    }

    /// All known attack selectors, in definition order.
    pub fn rbtica_all_selectors() -> &'static [&'static str] {
        &[
            RBTICA_SEL_DNS_ALLOWED_EXAMPLE,
            RBTICA_SEL_DNS_ALLOWED_EXAMPLE_ORG,
            RBTICA_SEL_DNS_BLOCKED_GOOGLE,
            RBTICA_SEL_APT_GET_BLOCKED,
            RBTICA_SEL_DNS_NONEXISTENT,
            RBTICA_SEL_DNS_TCP,
            RBTICA_SEL_DNS_UDP,
            RBTICA_SEL_DNS_BLOCK_DIRECT,
            RBTICA_SEL_DNS_BLOCK_ALTPORT,
            RBTICA_SEL_DNS_BLOCK_CLOUDFLARE,
            RBTICA_SEL_DNS_BLOCK_QUAD9,
            RBTICA_SEL_DNS_BLOCK_ZONETRANSFER,
            RBTICA_SEL_DNS_BLOCK_IPV6,
            RBTICA_SEL_DNS_BLOCK_MULTICAST,
            RBTICA_SEL_DNS_BLOCK_SPOOFING,
            RBTICA_SEL_DNS_BLOCK_TUNNELING,
            RBTICA_SEL_TCP443_CONNECT,
            RBTICA_SEL_TCP443_BLOCK,
            RBTICA_SEL_ICMP_FIRST_HOP,
            RBTICA_SEL_ICMP_SECOND_HOP_BLOCKED,
            RBTICA_SEL_DNS_EXFIL_SUBDOMAIN,
            RBTICA_SEL_META_CLOUD_ENDPOINT,
            RBTICA_SEL_NET_FORBIDDEN_CIDR,
            RBTICA_SEL_DIRECT_SENTRY_PROBE,
            RBTICA_SEL_ICMP_EXFIL_PAYLOAD,
            RBTICA_SEL_NET_IPV6_ESCAPE,
            RBTICA_SEL_NET_SRCIP_SPOOF,
            RBTICA_SEL_NET_SRCIP_SPOOF_EXTERNAL,
            RBTICA_SEL_PROTO_SMUGGLE_RAWSOCK,
            RBTICA_SEL_NET_FRAGMENT_EVASION,
            RBTICA_SEL_DIRECT_ARP_POISON,
            RBTICA_SEL_NS_CAPABILITY_ESCAPE,
            RBTICA_SEL_ARP_SEND_GRATUITOUS,
            RBTICA_SEL_ARP_SEND_GATEWAY_POISON,
            RBTICA_SEL_DNS_FORGE_RESPONSE,
            RBTICA_SEL_MAC_FLOOD_BRIDGE,
            RBTICA_SEL_NET_ROUTE_MANIPULATION,
            RBTICA_SEL_NET_ENCLAVE_SUBNET_ESCAPE,
            RBTICA_SEL_NET_DNAT_ENTRY_REFLECTION,
            RBTICA_SEL_UDP_NON_DNS_BLOCKED,
            RBTICA_SEL_CIDR_ALL_PORTS_ALLOWED,
            RBTICA_SEL_DNS_REBINDING,
            RBTICA_SEL_PROC_SYS_WRITE,
            RBTICA_SEL_TCP_RST_HIJACK,
            RBTICA_SEL_HTTP_END_TO_END,
            RBTICA_SEL_CONNTRACK_SPOOFED_ACK,
            RBTICA_SEL_SENTRY_UDP_NON_DNS,
        ]
    }
}

// ── Dispatch ────────────────────────────────────────────────────

/// Run the specified attack and return its verdict.
/// Exhaustive match — adding a variant without handling is a compile error.
/// extra_args: optional additional parameters (e.g. IP address for tcp443 attacks)
pub fn rbtica_run(attack: &rbtica_Attack, extra_args: &[&str]) -> rbtica_Verdict {
    match attack {
        rbtica_Attack::DnsAllowedExample => rbtica_expect_command_succeeds(
            "getent",
            &["hosts", RBTICA_CONNECTIVITY_DOMAIN],
            &format!("DNS resolution of {} (allowed domain)", RBTICA_CONNECTIVITY_DOMAIN),
        ),
        rbtica_Attack::DnsAllowedExampleOrg => rbtica_expect_command_succeeds(
            "getent",
            &["hosts", "example.org"],
            "DNS resolution of example.org (second allowed domain)",
        ),
        rbtica_Attack::DnsBlockedGoogle => rbtica_expect_command_fails(
            "getent",
            &["hosts", "google.com"],
            "DNS resolution of google.com (blocked domain)",
        ),
        rbtica_Attack::AptGetBlocked => rbtica_expect_command_fails(
            "timeout",
            &["5", "apt-get", "-qq", "update"],
            "apt-get update (package repos unreachable)",
        ),
        rbtica_Attack::DnsNonexistent => rbtica_expect_command_fails(
            "getent",
            &["hosts", "nonexistentdomain123.test"],
            "DNS resolution of nonexistentdomain123.test (non-existent domain)",
        ),
        rbtica_Attack::DnsTcp => rbtica_expect_command_succeeds(
            "dig",
            &["+tcp", RBTICA_CONNECTIVITY_DOMAIN],
            &format!("DNS over TCP for {} (allowed domain)", RBTICA_CONNECTIVITY_DOMAIN),
        ),
        rbtica_Attack::DnsUdp => rbtica_expect_command_succeeds(
            "dig",
            &["+notcp", RBTICA_CONNECTIVITY_DOMAIN],
            &format!("DNS over UDP for {} (allowed domain)", RBTICA_CONNECTIVITY_DOMAIN),
        ),
        rbtica_Attack::DnsBlockDirect => rbtica_expect_all_fail(
            &[
                ("dig", &["@8.8.8.8", RBTICA_CONNECTIVITY_DOMAIN] as &[&str]),
                ("nc", &["-w", "2", "-zv", "8.8.8.8", "53"]),
            ],
            "direct external DNS query bypass (dig @8.8.8.8 and nc 8.8.8.8:53)",
        ),
        rbtica_Attack::DnsBlockAltport => rbtica_expect_all_fail(
            &[
                ("dig", &["@8.8.8.8", "-p", "5353", RBTICA_CONNECTIVITY_DOMAIN] as &[&str]),
                ("dig", &["@8.8.8.8", "-p", "443", RBTICA_CONNECTIVITY_DOMAIN]),
            ],
            "alternate DNS port bypass (ports 5353 and 443)",
        ),
        rbtica_Attack::DnsBlockCloudflare => rbtica_expect_command_fails(
            "dig",
            &["@1.1.1.1", RBTICA_CONNECTIVITY_DOMAIN],
            "Cloudflare DNS bypass (1.1.1.1)",
        ),
        rbtica_Attack::DnsBlockQuad9 => rbtica_expect_command_fails(
            "dig",
            &["@9.9.9.9", RBTICA_CONNECTIVITY_DOMAIN],
            "Quad9 DNS bypass (9.9.9.9)",
        ),
        rbtica_Attack::DnsBlockZonetransfer => rbtica_expect_command_fails(
            "dig",
            &["@8.8.8.8", RBTICA_CONNECTIVITY_DOMAIN, "AXFR"],
            "DNS zone transfer attempt",
        ),
        rbtica_Attack::DnsBlockIpv6 => rbtica_expect_command_fails(
            "dig",
            &["@2001:4860:4860::8888", RBTICA_CONNECTIVITY_DOMAIN],
            "IPv6 DNS server bypass (2001:4860:4860::8888)",
        ),
        rbtica_Attack::DnsBlockMulticast => rbtica_expect_command_fails(
            "dig",
            &["@224.0.0.251", "-p", "5353", "example.local"],
            "multicast DNS bypass (224.0.0.251:5353)",
        ),
        rbtica_Attack::DnsBlockSpoofing => rbtica_expect_command_fails(
            "dig",
            &["@8.8.8.8", "+nsid", RBTICA_CONNECTIVITY_DOMAIN, "-b", "192.168.1.2"],
            "DNS spoofing source IP bypass",
        ),
        rbtica_Attack::DnsBlockTunneling => rbtica_expect_command_fails(
            "nc",
            &["-z", "-w", "1", "8.8.8.8", "53"],
            "DNS tunneling via nc to 8.8.8.8:53",
        ),
        rbtica_Attack::Tcp443Connect => {
            let ip = extra_args.first().copied().unwrap_or("");
            if ip.is_empty() {
                return rbtica_Verdict {
                    passed: false,
                    detail: "ERROR: tcp443-connect requires IP address as extra arg".to_string(),
                };
            }
            rbtica_expect_command_succeeds(
                "nc",
                &["-w", "2", "-zv", ip, "443"],
                &format!("TCP 443 connection to {} (should be allowed)", ip),
            )
        }
        rbtica_Attack::Tcp443Block => {
            let ip = extra_args.first().copied().unwrap_or("");
            if ip.is_empty() {
                return rbtica_Verdict {
                    passed: false,
                    detail: "ERROR: tcp443-block requires IP address as extra arg".to_string(),
                };
            }
            rbtica_expect_command_fails(
                "nc",
                &["-w", "2", "-zv", ip, "443"],
                &format!("TCP 443 connection to {} (should be blocked)", ip),
            )
        }
        rbtica_Attack::IcmpFirstHop => rbtica_check_icmp_first_hop(),
        rbtica_Attack::IcmpSecondHopBlocked => rbtica_check_icmp_second_hop_blocked(),
        // Ported python sorties
        rbtica_Attack::DnsExfilSubdomain => rbtics_sorties::rbtics_sortie_dns_exfil_subdomain(extra_args),
        rbtica_Attack::MetaCloudEndpoint => rbtics_sorties::rbtics_sortie_meta_cloud_endpoint(extra_args),
        rbtica_Attack::NetForbiddenCidr => rbtics_sorties::rbtics_sortie_net_forbidden_cidr(extra_args),
        rbtica_Attack::DirectSentryProbe => rbtics_sorties::rbtics_sortie_direct_sentry_probe(extra_args),
        rbtica_Attack::IcmpExfilPayload => rbtics_sorties::rbtics_sortie_icmp_exfil_payload(extra_args),
        rbtica_Attack::NetIpv6Escape => rbtics_sorties::rbtics_sortie_net_ipv6_escape(extra_args),
        rbtica_Attack::NetSrcipSpoof => rbtics_sorties::rbtics_sortie_net_srcip_spoof(extra_args),
        rbtica_Attack::NetSrcipSpoofExternal => rbtics_sorties::rbtics_sortie_net_srcip_spoof_external(extra_args),
        rbtica_Attack::ProtoSmuggleRawsock => rbtics_sorties::rbtics_sortie_proto_smuggle_rawsock(extra_args),
        rbtica_Attack::NetFragmentEvasion => rbtics_sorties::rbtics_sortie_net_fragment_evasion(extra_args),
        rbtica_Attack::DirectArpPoison => rbtics_sorties::rbtics_sortie_direct_arp_poison(extra_args),
        rbtica_Attack::NsCapabilityEscape => rbtics_sorties::rbtics_sortie_ns_capability_escape(extra_args),
        // Coordinated attack primitives — execute action, theurge judges outcome
        rbtica_Attack::ArpSendGratuitous => rbtics_sorties::rbtics_sortie_arp_send_gratuitous(extra_args),
        rbtica_Attack::ArpSendGatewayPoison => rbtics_sorties::rbtics_sortie_arp_send_gateway_poison(extra_args),
        // Coordinated integrity primitives — execute action, theurge judges state
        rbtica_Attack::DnsForgeResponse => rbtics_sorties::rbtics_sortie_dns_forge_response(extra_args),
        rbtica_Attack::MacFloodBridge => rbtics_sorties::rbtics_sortie_mac_flood_bridge(extra_args),
        // Novel unilateral attacks
        rbtica_Attack::NetRouteManipulation => rbtics_sorties::rbtics_sortie_net_route_manipulation(extra_args),
        rbtica_Attack::NetEnclaveSubnetEscape => rbtics_sorties::rbtics_sortie_net_enclave_subnet_escape(extra_args),
        rbtica_Attack::NetDnatEntryReflection => rbtics_sorties::rbtics_sortie_net_dnat_entry_reflection(extra_args),
        // Egress control verification
        rbtica_Attack::UdpNonDnsBlocked => rbtics_sorties::rbtics_sortie_udp_non_dns_blocked(extra_args),
        rbtica_Attack::CidrAllPortsAllowed => rbtics_sorties::rbtics_sortie_cidr_all_ports_allowed(extra_args),
        // Advanced adversarial probes
        rbtica_Attack::DnsRebinding => rbtics_sorties::rbtics_sortie_dns_rebinding(extra_args),
        rbtica_Attack::ProcSysWrite => rbtics_sorties::rbtics_sortie_proc_sys_write(extra_args),
        rbtica_Attack::TcpRstHijack => rbtics_sorties::rbtics_sortie_tcp_rst_hijack(extra_args),
        // Network path verification
        rbtica_Attack::HttpEndToEnd => rbtics_sorties::rbtics_sortie_http_end_to_end(extra_args),
        rbtica_Attack::ConntrackSpoofedAck => rbtics_sorties::rbtics_sortie_conntrack_spoofed_ack(extra_args),
        rbtica_Attack::OffpathBlockedDest => rbtics_sorties::rbtics_sortie_offpath_blocked_dest(extra_args),
        rbtica_Attack::ConntrackPipelineSelfcheck => {
            rbtics_sorties::rbtics_sortie_conntrack_pipeline_selfcheck(extra_args)
        }
        rbtica_Attack::SentryUdpNonDns => rbtics_sorties::rbtics_sortie_sentry_udp_non_dns(extra_args),
    }
}

// ── Attack Helpers ──────────────────────────────────────────────

/// Run a command and expect it to succeed (exit 0).
/// PASS when the security boundary correctly allows the operation.
fn rbtica_expect_command_succeeds(cmd: &str, args: &[&str], description: &str) -> rbtica_Verdict {
    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            if output.status.success() {
                rbtica_Verdict {
                    passed: true,
                    detail: format!("SECURE: {}", description),
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                rbtica_Verdict {
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
        Err(e) => rbtica_Verdict {
            passed: false,
            detail: format!("ERROR: {} — failed to execute '{}': {}", description, cmd, e),
        },
    }
}

/// Run a command and expect it to fail (nonzero exit).
/// PASS when the security boundary correctly blocks the operation.
fn rbtica_expect_command_fails(cmd: &str, args: &[&str], description: &str) -> rbtica_Verdict {
    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            if output.status.success() {
                rbtica_Verdict {
                    passed: false,
                    detail: format!("BREACH: {} — command succeeded unexpectedly", description),
                }
            } else {
                rbtica_Verdict {
                    passed: true,
                    detail: format!("SECURE: {}", description),
                }
            }
        }
        Err(e) => rbtica_Verdict {
            passed: false,
            detail: format!("ERROR: {} — failed to execute '{}': {}", description, cmd, e),
        },
    }
}

/// Run multiple commands and require ALL to fail (nonzero exit).
/// PASS only when the security boundary blocks every attempted bypass.
fn rbtica_expect_all_fail(checks: &[(&str, &[&str])], description: &str) -> rbtica_Verdict {
    for (cmd, args) in checks {
        match Command::new(cmd).args(*args).output() {
            Ok(output) => {
                if output.status.success() {
                    return rbtica_Verdict {
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
                return rbtica_Verdict {
                    passed: false,
                    detail: format!(
                        "ERROR: {} — failed to execute '{}': {}",
                        description, cmd, e
                    ),
                };
            }
        }
    }
    rbtica_Verdict {
        passed: true,
        detail: format!("SECURE: {}", description),
    }
}

/// Run traceroute -I -m 1 and verify first hop is sentry IP or * * *.
/// Reads sentry IP from /etc/resolv.conf nameserver line.
fn rbtica_check_icmp_first_hop() -> rbtica_Verdict {
    // Discover sentry IP from resolv.conf
    let sentry_ip = match rbtica_read_nameserver() {
        Ok(ip) => ip,
        Err(e) => {
            return rbtica_Verdict {
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
            return rbtica_Verdict {
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
        return rbtica_Verdict {
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
                return rbtica_Verdict {
                    passed: true,
                    detail: "SECURE: icmp-first-hop — first hop blocked (* * *)".to_string(),
                };
            }
        }
    }

    rbtica_Verdict {
        passed: false,
        detail: format!(
            "BREACH: icmp-first-hop — unexpected traceroute output (expected sentry IP {} or * * *):\n{}",
            sentry_ip, combined
        ),
    }
}

/// Run traceroute -I -m 2 and verify second hop is * * * (blocked).
fn rbtica_check_icmp_second_hop_blocked() -> rbtica_Verdict {
    let output = match Command::new("traceroute")
        .args(["-I", "-m", "2", "8.8.8.8"])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            return rbtica_Verdict {
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
                return rbtica_Verdict {
                    passed: true,
                    detail: "SECURE: icmp-second-hop-blocked — second hop blocked (* * *)".to_string(),
                };
            }
        }
    }

    rbtica_Verdict {
        passed: false,
        detail: format!(
            "BREACH: icmp-second-hop-blocked — expected blocked second hop (* * *) in traceroute:\n{}",
            combined
        ),
    }
}

/// Read the nameserver IP from /etc/resolv.conf.
/// Returns the first nameserver entry found.
fn rbtica_read_nameserver() -> Result<String, String> {
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
