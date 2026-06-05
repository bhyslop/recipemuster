# Platform divergence: Docker Desktop delivers off-path to the bottle (conntrack_spoofed_ack adjudication)

*Consolidated 2026-06-04. Heat ₣BV (rbk-10-mvp-uncontrolled-cygwin-test), pace
₢BVAAF. This memo merges the 2026-06-03 discovery (first ifrit run on uncontrolled
Cygwin) with the 2026-06-04 cross-platform adjudication into one authoritative
record. The original discovery memo is reduced to a redirect; its early
`nf_conntrack_tcp_loose` hypothesis is **disproven** — see "Disproven" below, and
do not resurrect it.*

## Bottom line

On **Docker Desktop (Windows, WSL2 backend)**, the container network substrate can
deliver a packet to the bottle that **never traverses the sentry**. This violates
a load-bearing invariant the architecture asserts — RBS0: *"bottle containers have
no direct network access; all connectivity flows through the sentry,"* enforced
from the first packet. The violation was surfaced by the `conntrack_spoofed_ack`
sortie, which BREACHES on Docker Desktop and is SECURE on native Linux.

This is a **platform-fidelity divergence, not a sentry defect**:

- The sentry's containment configuration is **byte-identical** across both
  platforms and behaves correctly on both.
- The breaching "response" is a TCP RST **manufactured by Docker Desktop's
  userspace network stack** (vpnkit/gvisor-tap-vsock lineage) and injected
  directly onto the enclave bridge from a non-sentry MAC. No sentry rule can
  reach it, because it is generated below the control point.
- It is **not an exploitable exfiltration channel** (see "Scope of exposure").

The durable takeaway: **Recipe Bottle's containment guarantee is firmest on
docker-on-Linux** (the canonical model per RBSCO). On Docker Desktop the substrate
itself is not a faithful Linux-bridge equivalent for containment purposes, and the
"trust the verdict" claim must carry that qualification.

## Disproven (do not revisit)

The 2026-06-03 discovery memo's leading hypothesis was that
`nf_conntrack_tcp_loose=1` lets conntrack adopt a lone mid-stream ACK as
ESTABLISHED, so the sentry's `RELATED,ESTABLISHED` rule admits the reply. **This
is wrong.** `nf_conntrack_tcp_loose` is `1` on the *secure* platform (native WSL
docker) too, and that platform does not breach. conntrack state never acts on the
breaching packet at all — the packet bypasses the sentry, so no conntrack/iptables
decision on the sentry is even consulted. The lever is the docker network
substrate, not a conntrack sysctl.

## The proof (packet level)

The `conntrack_spoofed_ack` sortie
(`rbida_sorties.rs::sortie_conntrack_spoofed_ack`) sends a bare TCP ACK (no prior
SYN) from the bottle to an allowlisted dest (`getent` of the connectivity domain
resolves via the sentry's dnsmasq to `192.0.46.9`, inside the allowlisted
`192.0.32.0/20`), then listens on a raw `IPPROTO_TCP` socket; **any** TCP packet
≥20 bytes from that dest is scored a BREACH.

Captures were taken on three legs simultaneously during a single-case run, using
the capture topology `scry`/`rboo_observe.sh` encodes (pentacle+bottle share a
netns; the sentry bridges enclave↔uplink), with `tcpdump -e` to expose L2 source
MACs:

**Cygwin Docker Desktop — bottle's own interface:**
```
16:76:99:27:5f:30 (bottle)      > 9e:1b:dd:ef:50:fb (sentry-enclave)  10.242.0.3.40080 > 192.0.46.9.80: Flags [.]   ACK out
ce:c3:90:4a:af:1a (NOT sentry)  > 16:76:99:27:5f:30 (bottle)          192.0.46.9.80 > 10.242.0.3.40080: Flags [R], win 0   RST in
```
**Cygwin sentry uplink leg:** only the outbound ACK. **Cygwin sentry enclave leg:**
only the outbound ACK. The breaching RST appears on **neither sentry interface** —
its L2 source `ce:c3:…` is Docker Desktop's host-side enclave gateway, neither the
sentry (`9e:1b:…`) nor the bottle (`16:76:…`). It did not transit the containment
boundary.

**WSL native docker — bottle's own interface (mirror):**
```
92:e0:9f:ba:72:48 (bottle) > ea:82:cb:9f:d4:99 (sentry)  10.242.0.3.40080 > 192.0.46.9.80: Flags [.]   ACK out
                                                          (nothing returns)
```
The bottle hears only its own outgoing ACK. SECURE.

| | Bottle sends ACK | Sentry forwards out uplink | Response to bottle | Verdict |
|---|---|---|---|---|
| **WSL** (real kernel bridge) | ✓ | ✓ (ttl decremented) | none | SECURE |
| **Cygwin DD** (userspace bridge) | ✓ | ✓ (ttl decremented) | RST injected on enclave bridge from a **non-sentry MAC** | BREACH |

## Static comparison — everything the sentry controls is identical

Measured inside the live `tadmor` sentry netns on each backend:

| Dimension | WSL | Cygwin DD |
|-----------|-----|-----------|
| kernel | `5.15.167.4-microsoft-standard-WSL2` | same |
| `nf_conntrack_tcp_loose` | `1` | `1` |
| `nf_conntrack_tcp_be_liberal` | `0` | `0` |
| FORWARD + RBM-FORWARD rules | full allowlist incl. `-d 192.0.32.0/20 -j ACCEPT` | byte-identical |

Same kernel, same conntrack config, same iptables — opposite verdict. The cause is
outside the sentry's configuration.

## Mechanism

On Docker Desktop, container bridge traffic is serviced by a userspace network
stack. Docker's own documentation confirms this component **replies with a TCP RST
for rejected/unknown connections**. Empirically, that synthesized RST (source IP
spoofed as the dest) reaches the bottle via Docker Desktop's host-side enclave
gateway, short-circuiting the sentry's forward/return path. Native docker uses real
kernel bridges with no such userspace responder, so the spoofed ACK egresses to the
real upstream and nothing returns to the bottle. The exact internal reason DD routes
the RST back via the enclave gateway rather than the sentry's reverse path is
Docker-Desktop-internal and unobservable from our side — and immaterial to the
verdict.

## Scope of exposure (why this is bounded, not alarming)

- The injected packet is an **empty RST** (`win 0`, no payload) — no data, no
  covert channel.
- It is generated only in **response to the bottle's own egress to an allowlisted
  dest**. Traffic to non-allowlisted dests is dropped at the sentry before DD's
  uplink stack ever sees it, so no off-path response is produced for blocked dests.
- It is not unsolicited third-party ingress and not exfiltration of operator
  assets. The primary containment goal (no egress to non-allowed destinations) is
  tested by other sorties, which pass identically on both backends.

The concern is structural fidelity, not a live exploit: DD's substrate *can* place
frames on the enclave segment to the bottle outside the sentry's view. Today that
capability manifests only as benign RSTs.

## What the sortie actually tests, and where it is valid

`conntrack_spoofed_ack` is a defense-in-depth probe of the sentry's **return-path
state enforcement**: a forged stateless ACK should not have its reply admitted by
the FORWARD `RELATED,ESTABLISHED` rule. On docker-on-Linux that maps to a real
sentry rule and the probe is valid. On Docker Desktop the reply bypasses that rule
entirely, so the probe's premise (sole-boundary) is broken and its BREACH maps to
no sentry rule — an epistemic violation of the ifrit discipline (RBSIP: every
breach should trace to a missing/weakened sentry rule).

## Repair direction (under discussion — not yet decided)

Two parts, decision pending:

1. **Make the sortie adjudicate only sentry-mediated traffic** (provenance): count a
   response as a breach only if it arrived via the containment gateway (L2 source =
   sentry enclave gateway). This restores breach⟺sentry-rule on every backend — a
   genuine return-path failure still fires; DD's off-path RST correctly reads
   SECURE. Preferred over a blunt backend-skip, which suppresses without restoring
   the invariant.
2. **Document DD's sole-boundary limitation as a platform scope qualification** in
   the security spec (RBS0/RBSIP), so part 1 does not silently bury the real
   platform finding behind a green check.

A sentry-side fix is **impossible** by construction — the artifact is injected below
the sentry — so the docket's generic "harden the sentry egress" branch does not
apply here.

## How we got here (discovery history)

**2026-06-03 — first ifrit run on uncontrolled Cygwin.** The `siege` suite (renamed
from `tadmor`; `kludge-tadmor` + the `tadmor` crucible fixture) ran for the first
time on an unprovisioned Cygwin + Docker Desktop host (`cygwin@rocket`): 45 passed,
1 failed — the lone failure `conntrack_spoofed_ack`. `blockade` (the `moriah`
airgap/conjure nameplate) reproduced the **identical** single breach, confirming it
is not asymmetric between tether and airgap nameplates — it tracks the shared
sentry-rule / network property. The open question was logged as platform-divergence
vs latent egress gap, with `nf_conntrack_tcp_loose` the leading (later disproven)
suspect.

**2026-06-04 — cross-platform adjudication.** Both rocket funduses were attached
directly to GitHub and force-synced to the branch tip. `siege` run on WSL (native
docker) came back 60/60 green; on Cygwin DD, 45/1 (fail-fast halts at
`conntrack_spoofed_ack`, so the 13 cases after it never run — "45/46" means "stopped
at first failure," not "1 of the full set"). Single-case reruns plus the three-leg
packet captures above isolated the cause to Docker Desktop's off-path RST.

## Reproduction recipe

1. Charge `tadmor` on the target fundus **within one ssh session** — WSL2 distros
   tear down dockerd when the charging session closes, so charge+probe+quench must
   share a session.
2. Capture three legs for `host <resolved-dest>` with `tcpdump -e -nn -vvv`:
   bottle/pentacle enclave interface, and both sentry interfaces.
3. Fire only the case: `rbw-tc tadmor rbtdrc_sortie_conntrack_spoofed_ack`.
4. Compare: on DD a `Flags [R]` reaches the bottle from a non-sentry MAC and appears
   on neither sentry leg; on native docker the bottle sees only its outbound ACK.

(Interface names are not stable across hosts — sentry `eth0`/`eth1` map to
uplink/enclave on Cygwin but enclave/uplink on the WSL host. Key on addresses/roles,
not names. This same hardcoded-name assumption is a latent bug in `scry` itself;
tracked as a repair pace on ₣BB.)

## Provenance and sources

- Branch `bv-conntrack-20260604-BVAAF` (off requal wrap `80b681e62`). Captures
  2026-06-04, nameplate `tadmor`, dest `192.0.46.9` (sentry dnsmasq hardcodes
  `www.internic.net`). MACs/ttls quoted verbatim from the runs; temp/trace dirs are
  operator-mutable and may be reaped.
- How Docker Desktop Networking Works Under the Hood — https://www.docker.com/blog/how-docker-desktop-networking-works-under-the-hood/
- containers/gvisor-tap-vsock — https://github.com/containers/gvisor-tap-vsock
- gvproxy (gvisor-tap-vsock) — https://deepwiki.com/containers/gvisor-tap-vsock/2.1-gvproxy
- WSL Mirrored Mode Networking in Docker Desktop 4.26.0 — https://dev.to/docker/wsl-mirrored-mode-networking-in-docker-desktop-4260-4b86
