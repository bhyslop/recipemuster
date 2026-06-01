# Milestone — First skirmish suite pass under WSL

**Date:** 2026-06-01
**Host:** `wsl@rocket` — WSL Ubuntu 24.04 on the `bujn-winpc` Windows host (tailnet `rocket`)
**Result:** `Suite 'skirmish': 11 fixture(s) run, 252 passed, 0 failed, 1 skipped` (exit 0)

This is the **first time the skirmish suite has ever run — and passed — under WSL.** Every
prior skirmish run was on macOS or Linux test hosts; the full Recipe Bottle stack had never
traversed the WSL transport before today.

## The commit we started from (the point of this memo)

The skirmish run began at:

```
88537cd4bab96c3a2a8889e2160a7702a58b2a70
  (short: 88537cd4b — "jjb:1015-2aedc936f::i: OFFICIUM 260601-1000", 2026-06-01 04:50 -0700)
```

This is the durable anchor. During the run the test infrastructure **self-committed** the
hallmark/ordain adjustments it produced (kludge cycles drove new local hallmarks into nameplates;
the ordain/bind steps propagated them). The rocket working tree finished **clean**, with HEAD
advanced to:

```
7467544b111db52cd7022c370d38b512a2cf931e
  ("ordain-bind: plantuml-bottle hallmark + propagate to pluml")
```

So **rocket is effectively branched.** `88537cd4b` is the common ancestor; the local station's
line and rocket's line diverge from it. rocket's HEAD no longer reveals where this run began —
hence recording `88537cd4b` here.

## What the run proved on WSL

The entire heavy runway traversed the WSL stack cleanly, end to end:

- **GCP IAM provisioning** — governor mantle, retriever invest, director invest
- **GAR reliquary** — inscribe (mirror tool images upstream → GAR)
- **Local kludges** — tadmor, ccyolo
- **Cloud Build conjures** — sentry, jupyter (two full cloud builds)
- **Airgap chain** — enshrine → conjure base → conjure airgap (ifrit-forge, ifrit-airgap)
- **Crucible charge/quench cycles** — across tadmor, srjcl, pluml
- **tadmor adversarial containment suite — 59/59** — sortie + coordinated network attacks
  (DNS/ICMP exfil, forbidden CIDR, IPv6 escape, srcip spoof, ARP gratuitous/gateway-poison/
  table-stability, sentry integrity, DNS-cache integrity, MAC-flood resilience, TCP-RST hijack,
  sentry egress lockdown, dnsmasq query audit). Full containment held under every attack.
- **srjcl (Jupyter) 3/3**, **pluml (PlantUML) 5/5**

## Environment prep required to reach this (Pale conduct — host was a fresh clone)

The WSL host needed tooling installed before the suite would run. Recorded so a future
provisioning of a WSL fundus is not a rediscovery:

1. **shellcheck pinned `0.11.0`** — the fast-qualification gate requires exactly `0.11.0`.
   Ubuntu noble ships `0.9.0` (`apt`), which fails the version check. Installed the official
   static release to `/usr/local/bin/shellcheck` (precedes `/usr/bin` on PATH).
2. **`docker-compose-v2`** — the charge path invokes `docker compose --project-directory …`.
   The native engine (`docker 29.1.3-0ubuntu3~24.04.2`) had no Compose v2 plugin; `apt install
   docker-compose-v2` (2.40.3) supplied the `docker compose` subcommand. Note: the Ubuntu
   package name is `docker-compose-v2`, NOT Docker's own-repo `docker-compose-plugin`.

Also, before any of this: macOS `scp` (OpenSSH 10.2) uses the SFTP protocol by default, and this
host's sshd has no SFTP subsystem — use `scp -O` (legacy protocol) until/unless
`openssh-sftp-server` is installed there.

## Known caveat

The suite reported **1 skipped** case (the same single skip observed in the crucible-tier run
earlier the same day). Not a failure; not yet characterized. Left as a known item.
