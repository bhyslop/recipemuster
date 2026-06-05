# First ifrit run on uncontrolled Cygwin — CONSOLIDATED, see redirect

*2026-06-03 discovery, superseded 2026-06-04.*

This memo recorded the first ifrit `siege` run on an uncontrolled Cygwin + Docker
Desktop host and the lone `conntrack_spoofed_ack` breach it surfaced. Its original
body proposed an `nf_conntrack_tcp_loose` explanation that was **subsequently
disproven**. To prevent that stale hypothesis from misleading future readers, the
full content — discovery context, cross-platform adjudication, packet-level proof,
mechanism, scope, and repair direction — has been consolidated into a single
authoritative record:

> **→ `Memos/memo-20260604-conntrack-spoofed-ack-adjudication.md`**

Verdict in one line: the Cygwin breach is a **Docker Desktop network-emulation
artifact** (an off-path RST injected onto the enclave bridge, bypassing the sentry),
**not a sentry egress gap**. The containment is firmest on docker-on-Linux.

The original discovery text remains in git history (this file's prior revisions) for
provenance.
