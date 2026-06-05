# conntrack_spoofed_ack adjudicated: Docker Desktop network-emulation artifact, not a sentry egress gap

*2026-06-04. Heat ₣BV (rbk-10-mvp-uncontrolled-cygwin-test), pace ₢BVAAF
(conntrack-spoofed-ack-adjudication). Resolves the open question left by
`memo-20260603-cygwin-ifrit-first-run-conntrack.md`.*

## Verdict

The lone `conntrack_spoofed_ack` BREACH observed on uncontrolled Cygwin + Docker
Desktop is a **platform verdict-divergence**, proven at packet level. It is **not**
a latent egress gap in the sentry. The sentry's containment is correct and
byte-identical across backends; the "response" the sortie detects on Docker
Desktop is a TCP RST **manufactured by Docker Desktop's userspace network stack
and injected directly onto the enclave bridge, bypassing the sentry entirely.**
On native Linux (WSL2, real kernel bridge) no such response exists and the case
passes.

The original memo's leading hypothesis — `nf_conntrack_tcp_loose=1` letting a
lone mid-stream ACK be adopted as ESTABLISHED — is **disproven**: that sysctl is
`1` on the *secure* platform too (see static comparison below). conntrack state
never enters into it, because the breaching packet never traverses the sentry's
conntrack at all.

## How it was adjudicated

All work rode branch `bv-conntrack-20260604-BVAAF` (off the requal wrap
`80b681e62`). Both rocket funduses were attached directly to GitHub (WSL via SSH,
Cygwin via HTTPS) and force-synced to the branch tip before each run. WSL is
native Docker (`docker 29.1.3`); Cygwin is Docker Desktop (`docker 28.3.2`).

### 1. Recreate — clean cross-platform split

`siege` (= `kludge-tadmor` + `tadmor` crucible) run on each backend from the same
integral source tip:

| Backend | siege tally | `conntrack_spoofed_ack` |
|---------|-------------|-------------------------|
| WSL (native docker) | 60 passed / 0 failed | **PASSED (SECURE)** |
| Cygwin (Docker Desktop) | 45 passed / 1 failed | **FAILED (BREACH)** |

This reproduces the original memo's `45/46` exactly. Note the crucible fixture
**fail-fasts**: on Cygwin it halts at `conntrack_spoofed_ack` (case 46) and
quenches, so the 13 cases that run after it on WSL never execute on Cygwin. "45/46"
means "stopped at first failure," not "1 of the full 59 failed." Single-case
reruns via `rbw-tc tadmor rbtdrc_sortie_conntrack_spoofed_ack` confirmed the split
independently (WSL PASS, Cygwin FAIL).

### 2. Static comparison — everything the sentry controls is identical

Measured inside the live `tadmor` sentry netns on each backend:

| Dimension | WSL | Cygwin DD |
|-----------|-----|-----------|
| kernel | `5.15.167.4-microsoft-standard-WSL2` | **same** |
| `nf_conntrack_tcp_loose` | `1` | **`1`** |
| `nf_conntrack_tcp_be_liberal` | `0` | **`0`** |
| FORWARD chain | `-P DROP` → `RELATED,ESTABLISHED ACCEPT` → `RBM-FORWARD` | **byte-identical** |
| RBM-FORWARD allowlist | `-d 192.0.32.0/20 -i <enclave> -j ACCEPT` (+ DNS, DNAT-state, icmp-drop) | **byte-identical** |

Same kernel, same conntrack config, same iptables — opposite verdict. The cause
is therefore *outside* the sentry's configuration. The sentry sets no conntrack
sysctl and has no `ctstate INVALID -j DROP`; it runs on the kernel default
`tcp_loose=1` on both platforms, and that platform is secure on WSL.

### 3. Packet-level proof — the breaching RST never crosses the sentry

The sortie (`rbida_sorties.rs::sortie_conntrack_spoofed_ack`) sends a bare TCP ACK
(no prior SYN) from the bottle to an allowlisted dest (`getent` of the
connectivity domain resolves via the sentry's dnsmasq to `192.0.46.9`, inside
`192.0.32.0/20`), then listens on a raw `IPPROTO_TCP` socket; **any** TCP packet
≥20 bytes from that dest is scored a BREACH.

Captures were taken on three legs simultaneously during a single-case run, using
the topology that `scry` (`rboo_observe.sh`) encodes — pentacle/bottle share a
netns; their enclave interface is `eth0`; the sentry bridges enclave↔uplink:

**Cygwin DD — bottle's own interface (`-e` shows L2 source):**
```
16:76:99:27:5f:30 (bottle)          > 9e:1b:dd:ef:50:fb (sentry-enclave)  10.242.0.3.40080 > 192.0.46.9.80: Flags [.]   ACK out
ce:c3:90:4a:af:1a (NOT sentry)      > 16:76:99:27:5f:30 (bottle)          192.0.46.9.80 > 10.242.0.3.40080: Flags [R], win 0   RST in
```
**Cygwin DD — sentry uplink leg:** only the outbound ACK (ttl 63, forwarded).
**Cygwin DD — sentry enclave leg:** only the outbound ACK (ttl 64, ingress).

The breaching RST's L2 source is `ce:c3:90:4a:af:1a` — **neither the sentry's
enclave MAC (`9e:1b:…`) nor the bottle (`16:76:…`).** It is Docker Desktop's
host-side enclave gateway. The RST appears on **neither sentry interface** — it
did not transit the containment boundary. The sentry forwarded the ACK out the
uplink (the dest is legitimately allowlisted) and would have governed any real
return; none arrived. Docker Desktop answered the spoofed ACK *on the bridge
itself* and delivered the RST to the bottle off-path.

(`/proc/net/nf_conntrack` is not exposed in the sentry netns on DD, so conntrack
state could not be dumped there — moot, since the packet never reaches the
sentry's conntrack.)

**WSL native — bottle's own interface (mirror experiment):**
```
92:e0:9f:ba:72:48 (bottle) > ea:82:cb:9f:d4:99 (sentry)  10.242.0.3.40080 > 192.0.46.9.80: Flags [.]   ACK out
                                                          (nothing returns)
```
The bottle hears only its own outgoing ACK. The sentry legs show the ACK
ingressing (ttl 64) and forwarded out the uplink (ttl 63). No RST anywhere. SECURE.

(Incidental: Docker assigns sentry interface *names* in opposite order across
backends — on Cygwin `eth0`=uplink/`eth1`=enclave; on WSL `eth0`=enclave/`eth1`=uplink.
The captures key on addresses/roles, not names, so this is cosmetic.)

## Mechanism

On Docker Desktop (Windows, WSL2 backend) container bridge traffic is serviced by
a userspace network stack (historically vpnkit, currently gvisor-tap-vsock).
Docker's own documentation confirms that this component **replies with a TCP RST
when a connection is rejected/unknown** — standard stateless-TCP behavior for a
host with no matching socket. Empirically here, that synthesized RST (source IP
spoofed as the dest `192.0.46.9`) is delivered to the original source via Docker
Desktop's host-side enclave gateway (`ce:c3:…`), short-circuiting the sentry's
SNAT/forward return path. Native docker uses real kernel bridges with no such
userspace responder, so the same spoofed ACK egresses to the real upstream and
nothing returns to the bottle.

The exact internal reason DD routes the RST back via the enclave gateway rather
than the sentry's reverse-SNAT path is Docker-Desktop-internal and unobservable
from our side — and irrelevant to the verdict. The load-bearing, proven fact is
that **the response is generated below the containment boundary and never passes
through the control point we own.**

Sources:
- How Docker Desktop Networking Works Under the Hood — https://www.docker.com/blog/how-docker-desktop-networking-works-under-the-hood/
- containers/gvisor-tap-vsock — https://github.com/containers/gvisor-tap-vsock
- gvproxy (gvisor-tap-vsock) — https://deepwiki.com/containers/gvisor-tap-vsock/2.1-gvproxy
- WSL Mirrored Mode Networking in Docker Desktop 4.26.0 — https://dev.to/docker/wsl-mirrored-mode-networking-in-docker-desktop-4260-4b86

## Why this is divergence, not a real gap (the skeptical case, answered)

The uncomfortable surface reading: *the bottle received a TCP response to a
spoofed ACK aimed at an allowlisted host.* Answered:

- The response is an **empty RST** (`win 0`, no payload) **synthesized by DD's
  emulation** — not a reply that reached the real internet host and returned
  through the sentry. There is no data channel and no real return path.
- The sentry forwards to the allowlisted CIDR **by design** (dest-based egress is
  the intended posture for allowed CIDRs) and **saw no return to govern**.
- The sortie is a **defense-in-depth observability probe** ("a stateless ACK
  should not draw a response"). On DD that property *appears* violated only
  because DD answers on the bridge — a divergence in **test observability**, not
  in **security posture**. The 45 sorties that test real reachability with
  payloads pass identically on both backends.

An attacker in the bottle gains nothing exploitable from a locally-manufactured
RST: no exfiltration, no real return channel, no reachability the sentry permits.

## Implication for the heat thesis

₣BV's claim is consumer-credibility: *"a consumer can qualify their own install on
their own machine and trust the verdict."* This sortie, unmodified, renders a
**false-positive breach on Docker Desktop backends** — a direct dent in "trust the
verdict." Closing that is the remaining work (reconciliation, below). The
containment itself is sound; the test's verdict is what needs to be made
backend-honest.

## Conduct at the Pale

Docker Desktop is foreign engineering we cannot edit — our realm's edge. The
disciplined response is not to "fix" DD but to characterize its behavior precisely
(done: the off-path RST signature, captured), contain it at one membrane (the
sortie or its backend gating), and tie that membrane to the specific grievance so
it carries a removal condition (retire if/when DD stops answering on the bridge).

## Reconciliation options (decision pending — see discussion)

How the suite should treat `conntrack_spoofed_ack` on Docker Desktop backends:

- **(a) Backend-gate the sortie** — mark xfail/skip on Docker-Desktop backends
  with documented rationale. Cleanest restoration of "trust the verdict": the
  suite *knows* this probe is platform-blind on DD. Requires a backend-detection
  signal.
- **(b) Harden the sortie** — validate that a detected response actually transited
  the sentry (e.g., require the response's L2 source to be the sentry enclave
  gateway; ignore off-path injections). Most thorough, most code; makes the probe
  robust rather than gated.
- **(c) Document only, leave behavior** — DD users continue to see a red breach
  that isn't one. Rejected as inconsistent with the heat thesis.

Lean: (a) now to make the verdict backend-honest, with (b) noted as the more
thorough follow-up. Final approach to be settled in discussion.

## Reproduction recipe

1. Charge `tadmor` on the target fundus (single ssh session — WSL2 distros tear
   down dockerd when the charging session closes; charge+probe+quench in one).
2. Capture on three legs for `host <resolved-dest>` with `tcpdump -e -nn -vvv`:
   pentacle/bottle `eth0`, and both sentry interfaces.
3. Fire only the case: `rbw-tc tadmor rbtdrc_sortie_conntrack_spoofed_ack`.
4. Compare: on DD a `Flags [R]` reaches the bottle from a non-sentry MAC and
   appears on neither sentry leg; on native docker the bottle sees only its
   outbound ACK.

## Provenance

- Branch `bv-conntrack-20260604-BVAAF`; siege stamps `3f916ffac`/`bb1f628f9` (WSL),
  `e620e60`/`fa21750` (Cygwin, host-local).
- Captures 2026-06-04, nameplate `tadmor`, dest `192.0.46.9` (sentry dnsmasq
  hardcodes `www.internic.net`). MACs and ttls quoted above are verbatim from the
  runs. Temp/trace dirs are operator-mutable and may be reaped; salient lines
  inlined here.
