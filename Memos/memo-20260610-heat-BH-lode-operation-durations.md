# Lode operation durations — log manifest (heat ₣BH)

Date: 2026-06-10
Status: **filename manifest only** — preserves which of today's passed-test logs to mine
for per-operation duration expectations. The actual duration extraction is a later pace
(logs in `../logs-buk/` rotate, so the filenames are captured here before they age out).
Wall-clock figures already in hand from the capture/banish logs are recorded inline as a
head start; the read-only and fixture spans are left for the extraction pace.

## Extraction method (for the later pace)

- Cloud ops (immure/ensconce/conclave/underpin/banish): `grep 'Wall clock' <log>` →
  the GCB build duration directly (e.g. `Immure: Wall clock 5m 4s`).
- Any op, total span: last `[YYYY-MM-DD HH:MM:SS]` line − first. Covers the read-only
  ops (augur/divine) and the lifecycle-fixture bundles, which carry no Wall-clock line.
- Caveats: Wall clock = the cloud-build portion only (host auth/poll adds a few seconds);
  capture time is payload-dominated; banish ~12s holds for FLAT packages — index-web
  convergence banishes were not sampled today and run longer.

## Capture ops — `../logs-buk/` (wall-clock in hand)

| Operation | Colophon | Payload | Wall clock | Log filename(s) |
|-----------|----------|---------|-----------|-----------------|
| immure podvm-wsl, fresh | `rbw-lI` | 2 wsl leaves | 0m22s / 0m12s / 0m13s | `hist-rbw-lI-sh-20260610-095327-32614-471.txt`, `-100942-61140-235.txt`, `-115358-25257-765.txt` |
| immure podvm-wsl, refresh (all-preserved) | `rbw-lI` | 0 new leaves, re-vouch | 0m10s | `hist-rbw-lI-sh-20260610-115508-26667-10.txt` |
| immure podvm-native, full 8-leaf | `rbw-lI` | 8 leaves, multi-GB | 5m04s | `hist-rbw-lI-sh-20260610-115913-30588-874.txt` |
| ensconce (bole, single image) | `rbw-lE` | 1 image | 0m16s / 0m13s / 0m10s | `hist-rbw-lE-sh-20260610-080649-36629-587.txt`, `-081040-39326-308.txt`, `-081139-40059-671.txt` |
| conclave (reliquary cohort) | `rbw-lC` | tool cohort, single-platform | 0m38s | `hist-rbw-lC-sh-20260610-081905-42474-105.txt` |
| underpin (wsl rootfs) | `rbw-lU` | rootfs blob | 0m15s | `hist-rbw-lU-sh-20260610-082544-46818-576.txt` |

## Delete op — banish (FLAT packages, ~0m11–14s)

`rbw-lB` logs (all today, all passed):
`hist-rbw-lB-sh-20260610-`{`080926-38335-608`, `081412-41410-820`, `082045-45677-735`,
`082701-48230-294`, `095652-34969-615`, `101054-63422-328`, `115629-29299-743`,
`120659-33279-943`}`.txt`

## Read-only ops — durations TBD (span = last−first timestamp)

- augur (`rbw-la`): `hist-rbw-la-sh-20260610-`{`080924`, `082037`, `082658`, `095510`,
  `101046`, `115622`, `120554`}`-*.txt`
- divine (`rbw-ld`): many today — `hist-rbw-ld-sh-20260610-*.txt` (08:09–11:57 cluster).

## Lifecycle-fixture bundles — `rbw-tf` (full round-trips; span TBD)

All passed today:
- `lode-lifecycle` — `hist-rbw-tf-sh-20260610-080648-36455-4.txt`
- `reliquary-lifecycle` — `hist-rbw-tf-sh-20260610-081904-42303-501.txt`
- `wsl-lifecycle` — `hist-rbw-tf-sh-20260610-082544-46646-972.txt`
- `podvm-lifecycle` — `hist-rbw-tf-sh-20260610-100942-60977-335.txt`, `-115357-25087-998.txt`
  (the second bundles fresh immure + the new all-preserved refresh + jettison + banish)
