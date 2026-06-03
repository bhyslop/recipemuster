# First ifrit run on uncontrolled Cygwin — siege 45/46 and the conntrack_spoofed_ack breach

*2026-06-03. Heat ₣BV (cygwin-spike). This was the **first time the ifrit
adversarial security suite ran on an uncontrolled Cygwin + Docker Desktop
host** (`cygwin@rocket`).*

## Headline

The `siege` suite (renamed this session from `tadmor` = `kludge-tadmor` +
`tadmor` crucible fixture) charged on Cygwin Docker Desktop and ran the full
adversarial sortie set: **45 passed, 1 failed (46 total)**, clean quench.
That is the heat's headline essentially achieved — theurge's security suite
runs, and 45 escape vectors are correctly contained, on a box we do not
provision.

The single failure is recorded below. It does not diminish the result, but it
is a genuine finding that needs adjudication, **not** dismissal.

## The one breach

```
FAILED: rbtdrc_sortie_conntrack_spoofed_ack
BREACH: spoofed ACK to 192.0.46.9:80 got response —
        conntrack RELATED,ESTABLISHED not filtering stateless ACKs
IFRIT_VERDICT: FAIL BREACH ...
```

Sortie: `rbtdrc_sortie_conntrack_spoofed_ack` →
`rbtdrc_invoke_ifrit(ctx, "conntrack-spoofed-ack", ...)` — an ifrit sortie
driven by the `rbtid` adjutant. (Full packet/source construction lives in the
sortie script; not yet read — see Next Steps.)

Sentry FORWARD chain at the time (from the captured `iptables-rules.txt`):

```
-P FORWARD DROP
-A FORWARD -m state --state RELATED,ESTABLISHED -j ACCEPT
-A FORWARD -j RBM-FORWARD
...
-A RBM-FORWARD -d 192.0.32.0/20 -i eth0 -j ACCEPT      # allowlisted CIDR
```

## The crucial nuance — do not over-read this as a clean conntrack failure

The target **`192.0.46.9` is inside `192.0.32.0/20`**, which is an
**allowlisted** destination in the FORWARD chain. So a packet to it is permitted
**by the dest-based allowlist rule**, independent of conntrack state. The
sortie's verdict attributes the breach purely to "conntrack RELATED,ESTABLISHED
not filtering stateless ACKs," but that attribution is **incomplete**: to an
allowed dest, the allowlist alone would let the packet through.

Two non-exclusive readings, both worth testing:

1. **Defense-in-depth expectation** — the sortie asserts that a *stateless*
   forged ACK should not elicit a response *even to an allowed host* (only a
   properly established connection should). The current rules are dest-based,
   not state-gated, for allowed CIDRs.
2. **`nf_conntrack_tcp_loose=1`** — the kernel default makes conntrack adopt a
   lone mid-stream ACK as ESTABLISHED, so the top `RELATED,ESTABLISHED` rule
   accepts it before the allowlist is even consulted.

## The open question (platform divergence vs latent gap)

Does this breach reproduce on **native Linux / WSL** (e.g. `cerebro`)?

- **If it PASSES on native Linux but FAILS on Cygwin DD-WSL2** → a **platform
  verdict-divergence**: the Docker Desktop WSL2 / LinuxKit network stack handles
  conntrack (or `nf_conntrack_tcp_loose`) differently than native netfilter.
  This is a direct hit on the consumer-credibility claim — "a consumer can
  qualify their own install on their own machine and **trust the verdict**" —
  because the same suite would render a different security verdict depending on
  the host's docker backend.
- **If it FAILS everywhere** → a **latent property of the current sentry rules**
  that the cygwin run merely surfaced first (this suite had never run on cygwin
  before). Then it is a real egress-hardening item independent of platform.

Either outcome matters; we cannot tell which from this single run.

## Next steps to adjudicate

1. Read the `conntrack-spoofed-ack` sortie script in `rbtid` — exact packet
   construction (is the source spoofed? is the response in-band or via a
   conntrack-created return path?) and the precise verdict logic.
2. Run `siege` (and/or the `tadmor` crucible fixture) on **`cerebro`** (native
   Linux) and on the **WSL** side of `rocket` — compare the
   `conntrack_spoofed_ack` verdict across backends.
3. Compare `nf_conntrack_tcp_loose` (and related conntrack sysctls) between the
   DD-WSL2 kernel and native Linux.
4. If a real gap: harden the sentry egress — e.g. set `nf_conntrack_tcp_loose=0`,
   add `-m conntrack --ctstate INVALID -j DROP`, and/or state-gate the allowlist
   so allowed CIDRs still require an established connection.

## Provenance

- Run trace (cygwin): `temp-buk/temp-20260603-050730-21760-358/rbtd/`
  (`rbtdrc_sortie_conntrack_spoofed_ack/{trace.txt,bark-stdout.txt}`,
  `rbtdrc_sentry_iptables_loaded/iptables-rules.txt`). Temp dirs are
  operator-mutable and may be reaped; the salient lines are quoted above.
- The bottle/crucible only charged on cygwin at all because of the two
  Windows-Docker-Desktop membranes — see
  `memo-20260603-windows-docker-desktop-bind-mount.md`.

## Addendum (2026-06-03, after blockade)

blockade (moriah — conjure-mode, GAR-summoned, airgap-built) ran on Cygwin and
produced the **identical 45/46** — the same single `conntrack_spoofed_ack`
BREACH. moriah carries a runtime egress allowlist too (`tcp443_allow_example`
and `cidr_all_ports_allowed` both passed), so the breach reproduces
**cross-nameplate** on Cygwin DD — it is *not* asymmetric between the tether and
airgap nameplates. That narrows the cause toward the shared sentry-rule /
conntrack property and makes the **native-Linux/WSL reproduction the decisive
next test**: does it breach there too?
