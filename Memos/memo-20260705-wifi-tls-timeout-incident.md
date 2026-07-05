# Incident report: TLS-handshake timeout burst killed a picket run (2026-07-05)

Status: closed — no code change; environmental cause at the Palisade.
This memo is the complete disposition (operator ruling 260705).

## Signature (grep anchors)

- `curl: (28) SSL connection timeout` — curl's stderr, every affected attempt
- Captured HTTP code `000`; script-level report `HTTP request failed after 3 attempts (curl exit 28)`
- TCP connect succeeds, TLS handshake stalls past the 10s `RBCC_CURL_CONNECT_TIMEOUT_SEC` window
- Burst duration ≤ ~1 minute, then full self-heal

## What happened

During a picket suite run (verifying the ₢BqAAG curl-containment sweep), the
`rbtdrv_hallmark_lifecycle` fixture failed at its post-abjure summon: expected the
vacant band 110, got exit 1. The exit 1 was a genuine network death, not a wrong
classification:

- 08:26:56–08:27:07 — post-abjure rekon made 18 consecutive successful HTTPS calls
  to `artifactregistry.googleapis.com`.
- 08:27:07–08:27:40 — request 19 timed out three times (10s TLS stall each, HTTP 000),
  exhausting rbuh's full transient-retry cycle (3 attempts x 3s sleep); the rekon died
  naming curl exit 28. The fixture tolerated this (it asserts only non-zero exit).
- ~08:27:40–08:27:50 — the summon reused the cached sitting (no network), then its don
  leg (single-attempt by design) hit the identical SSL timeout against
  `iamcredentials.googleapis.com` and died — exit 1 where the fixture demanded band 110.
- By 08:30:55 — payor OAuth probe 5/5 HTTP 200; path fully healed.

## What it was not

- **Not the curl-containment sweep.** The sweep diff (172c465ab) touched no timeout or
  retry semantics; curl's own stderr names the failure. The sweep is why the transcripts
  could name "curl exit 28" at each site at all — pre-sweep these were unclassified bare
  deaths.
- **Not DNS or TCP.** "SSL connection timeout" means the TCP handshake completed and the
  TLS handshake stalled; the same host had just answered 18 requests.
- **Not the Wi-Fi association.** airportd link telemetry (5s cadence) across the window:
  RSSI steady −47 dBm, `txFail=0` every interval, zero deauth/roam/link-down events.
- **Not Tailscale.** tailscaled logged nothing in the window; no exit node configured.

## What correlates

airportd channel telemetry shows a contention burst exactly spanning the failures:
channel-busy (`cca`) 22% → 35% → 59% → 49% at 08:27:10/25/40, `interferenceTotal`
tripling (14–16 → 48), retransmission bursts to 153 frames/5s. Something saturated
channel 48 for about a minute.

The station's default route at the time: **open (Security: NONE) guest-style Wi-Fi**
(en1, gateway 172.16.224.1) among WPA2-Enterprise neighbor SSIDs; Ethernet (en0)
present but unplugged.

Two mechanisms fit, both foreign infrastructure:

1. **Downstream airtime starvation** — the TLS ServerHello+certificate flight is the
   handshake's one large downstream burst; under heavy contention the AP's frames to the
   station die invisibly (station-side `txFail` counts only its own transmissions), while
   small upstream packets (SYN, ClientHello) get through. Exactly yields
   "TCP connects, TLS times out."
2. **Guest-network middlebox** — captive-portal/firewall gear that SYN-proxies, with its
   upstream leg briefly failing.

Not distinguishable retroactively; the distinction changes nothing downstream.

## Frequency

One prior occurrence in 34,574 logged invocations:
`logs-buk/hist-rbw-aM-sh-20260604-212927-85961-781.txt` (2026-06-04), identical
signature against Google's OAuth endpoint. Rare, recurrent, environmental.

## Disposition (all closed)

- **No retry tuning.** The outage (~45s+) outlasted rbuh's full 39s retry window; only
  minutes-scale backoff would have survived it — a temperament change for interactive
  commands that fails the load-bearing-complexity test at ~2 events/month observed.
- **No new membrane.** The existing rbuh transient-retry membrane already contains the
  surveyed signature (curl 28 is classified transient); it cycled correctly and failed
  honestly. The don's single-attempt design stands.
- **Operator-side lever noted, not mandated:** wired Ethernet (or a managed SSID) removes
  both candidate mechanisms. The run environment is the operator's estate.
- Recovery at the time: reran the picket suite once the path healed.
