<!--
Copyright 2026 Scale Invariant, Inc.
SPDX-License-Identifier: Apache-2.0
See LICENSE for terms.
-->

# Jailer Dialect Guide (JDG) — In-Vessel POSIX sh for the Security Envelope

## Purpose

JDG codifies the dialect of the **jailer scripts** (`rbj*`) — the
zero-dependency POSIX `sh` baked into the sentry and pentacle vessel images and
run inside the container security envelope. `rbjs_sentry.sh` is the type
specimen; `rbjp_pentacle.sh` is its smaller sibling.

It is a foreign-environment sibling to BCG (host bash), CBG (cloud step bodies),
and WSG (ssh-to-Windows transport): it shares their philosophy — crash-fast, no
silent failures, load-bearing complexity — but its mechanics are dominated by
one fact: **a jailer script is sealed inside the envelope at runtime, and its
only contract with the world outside is its container log and its exit code.**
There is no kit runtime to lean on and no way to instrument the process after it
starts. Everything the dialect mandates serves the one end of making a sealed,
un-instrumentable process fully legible from those two channels.

This is a v1 guide: it **blesses and names emergent practice**. It mandates
nothing the jailer scripts do not already do. Where the type specimen deviates
from a rule it otherwise follows, the guide names the deviation as a citable
defect rather than silently repairing it.

## How this document is organized — two genres on purpose

Like CBG and PCG, JDG separates two kinds of knowledge:

- **Authored Disciplines** (prose, no IDs) — the *shape* of the dialect: how a
  jailer script is built. Systematic, interderivable, internalized — numbering
  them would be ceremony with no citers.
- **Cited Rules** (numbered `JD*_`) — discrete facts that something *points at*:
  a review flagging a violation, a comment justifying a bend, or a standing
  deviation in the type specimen itself.

One honest divergence from CBG/PCG. There, the Cited Rules are purely *foreign*
facts about a Palisade we do not control, and everything *ours* stays an Authored
Discipline. JDG cites some of its **own** contract — the observability rules
(`JDo_`) — because external legibility *is* the dialect's defining purpose: a
log-reader, a reviewer, and the known type-specimen inconsistency all point at
those rules, so the citer exists even though the behavior is ours. The
parameter-transport rule (`JDp_`) is a foreign fact in the ordinary mold. The
shape-of-the-dialect patterns, which nothing points at, stay uncited Authored
Disciplines.

## Core Philosophy

**No kit runtime, by construction.** A jailer script is a single file shipped
into a container image and launched with a bare `/bin/sh`. None of BCG's
scaffolding exists inside it — no `buc_die`, no `buc_step`, no kindle/sentinel
lifecycle, no `buv_*` validation, no module/CLI gateway. Every guarantee BCG gets
from BUK, a jailer script re-establishes in a few lines of raw `sh` or does
without. The philosophy survives the crossing; the mechanics are rebuilt
minimally, in-language — the same relationship CBG and WSG have to BCG.

**The log is the only forensic surface, and the exit code is the only verdict.**
Louder here than anywhere else in the family. The process runs sealed inside the
security envelope: there is no debugger to attach, no temp directory to inspect
afterward, no way to add tracing once it is running. You read its stdout/stderr
after the fact, and you read its exit code on death. So the script narrates
itself as it goes (phase-anchor announcements, `JDo_101`) and dies with a
phase-coded exit (exit-code families, `JDo_102`). A failure that does not both
print *and* exit a meaningful code is a failure that cannot be diagnosed from
outside the envelope.

**Trust nothing the substrate hands you.** Both the parameter transport (the
Docker/compose env-file) and the runtime (Docker's interface naming and route
assignment) are foreign and unreliable. The dialect *probes and re-derives*
rather than trusting: the self-probing prologue (`JDp_101`) validates the
transport before reading any real parameter, and the same stance recurs as
domain logic — the type specimen discovers interfaces by IP because "Docker does
not guarantee eth0/eth1 ordering," and seizes the default route because compose
`priority` is not honored. The prologue is the dialect-universal instance of a
stance the whole script embodies.

**Load-bearing complexity still rules.** Stay dependency-free and straight-line
not for austerity's sake but because every dependency is a thing that can fail
unobserved inside the envelope. Reuse nothing that would have to be sourced;
factor nothing that straight-line code reads more plainly.

## The Vessel Environment

A jailer script runs under conditions a host script never faces. Each shapes the
dialect.

| Condition | Consequence for the script |
|-----------|----------------------------|
| **Baked into the image, launched with a bare `/bin/sh`** that may be dash or busybox `ash`. | No runtime to source, no functions worth factoring; POSIX-only, straight-line (Authored Disciplines). |
| **Sealed inside the security envelope at runtime** — no debugger, no post-mortem temp dir, no added tracing. | The container log and the exit code are the only forensic surface (`JDo_`). |
| **Parameters arrive through the Docker/compose env-file transport**, whose quoting and word-splitting the script does not control. | Probe the transport before trusting any parameter (`JDp_101`). |
| **The substrate does not guarantee interface ordering and does not honor compose route priority.** | Trust nothing the substrate hands you; re-derive and seize ownership (Core Philosophy; in-script citers). |
| **The script is usually the container's long-lived process**, configuring then holding. | Touch a health sentinel, then `exec sleep infinity` (Authored Disciplines). |

---

## Authored Disciplines (prose — internalize, don't cite)

### Self-contained: no sourcing, no functions

A jailer script sources nothing and defines no functions. It runs top to bottom
as one straight-line sequence. There is no kit runtime to pull in, and factoring
a sealed ~300-line script into functions buys nothing a reader needs while adding
a layer between the log line and the source line it came from. The whole file is
the unit.

### POSIX `sh` only — no bashisms

The shebang is `#!/bin/sh` and the image's `/bin/sh` may be dash or busybox
`ash`. Use only what POSIX guarantees:

- `test` / `[`, never `[[`.
- `$(( ))` arithmetic (the type specimen does bitwise subnet math this way),
  never `let` or `(( ))`.
- Parameter expansion for slicing — `${v%/*}`, `${v#*/}`, `${v:?}`, `${v:-default}`.
- `read` with a here-doc or a redirected file for word-splitting and field
  capture, never arrays.
- No `local`; see the scratch convention below.

### The `z_` scratch convention

POSIX `sh` has no `local`, so every variable is global. To keep that legible,
loop variables and short-lived intermediates carry a `z_` prefix (`z_word`,
`z_temp_file`, `z_uplink_ip`). The prefix marks them as scratch and keeps them
visually distinct from the three other namespaces a jailer script touches:
incoming parameters (`RBRN_*`, `RBRR_*`, `RBJE_*`), and durable script-derived
values (`RBJ_*`, `RBJP_*`).

### The prologue line: `set -e`, then the verbose toggle

Every jailer script opens with `set -e` (the crash-fast floor) followed by an
opt-in trace toggle:

```sh
set -e
test "${RBJ_VERBOSE:-0}" -ge 1 && set -x
```

`set -e` is the floor, not the contract: it guarantees *an* exit on an unhandled
failure, but the exit code it carries is the failing command's, not a
phase-coded one. `JDo_103` is what turns "something failed" into "phase 3
failed."

### Configure-then-hold ending

A jailer script that stays alive as the container's process ends by signalling
readiness and then replacing itself:

```sh
echo "RBJp5: Signaling health"
touch /tmp/rbjh_healthy || exit 50
exec sleep infinity
```

The health sentinel (`/tmp/rbj*h_healthy`) is the readiness signal the harness
watches for; the `exec` replaces the shell so the holding process is PID 1's
direct child and signals reach it cleanly.

---

## Cited Rules (numbered — each has a citer)

### JDo_ — Observability Contract (the log and the exit code are the surface)

#### ✅ JDo_101: Phase-anchor announcements — the execution-time census

Every phase of the script opens by echoing a phase-anchor label:

```sh
echo "RBJp1: Validate parameters"
```

The label (`RBJp<N>`) is a token greppable in two directions: from a line in the
container log back to the source phase that emitted it, and from the source
forward to "did this phase reach the log." Together with the exit code, the last
phase label printed before death locates the failure to a span of source. This
is the execution-time census: the script tells you where it is as it runs,
because nothing else can.

The contract on the label is **consistency and monotonicity**: a line's
announced phase tag matches the phase it sits in, and phase labels advance in
source order. A tag that lies about its phase, or that goes backwards, breaks the
log-to-source mapping the census depends on.

*Known deviation (citer):* the type specimen `rbjs_sentry.sh` violates JDo_101 in
two places, blessed-but-unrepaired under the v1 posture. (1) The
parameter-dump echoes under the `RBJp1: Validate parameters` announcement are
themselves tagged `RBJp0:`, so they report a phase they do not sit in. (2) The
phase-2 band announces `RBJp2`, then `RBJp2c`, then `RBJp2b` — the suffix
ordering is non-monotonic (`c` precedes `b`). Both are citable against this rule;
neither is fixed by the pace that wrote this guide.

*Cited by:* the deviation above; future reviews of jailer scripts.

#### ✅ JDo_102: Exit-code families locate the failure

A jailer script exits with a code whose **tens digit is the phase number** and
whose units digit distinguishes the failure site within that phase. The type
specimen runs phase 1 → `exit 10`–`13`, phase 2 → `exit 20`/`25`/`28`, phase 3 →
`exit 30`–`32`, phase 4 → `exit 40`–`43`, phase 5 → `exit 50`. On container
death, the exit code alone names the phase — the forensic complement to the last
phase label in the log (`JDo_101`).

The pre-flight band is distinct from the numbered families: the transport probe
exits `1`, and a missing required parameter exits through the bare `${VAR:?}`
expansion's own non-zero. Both are deliberately *below* the phase families,
marking "the script never reached a real phase."

A script too small for numbered phase announcements (the pentacle) still uses the
tens-digit families: `rbjp_pentacle.sh` exits `10`/`11`/`20`/`30`/`31`/`40` along
the same scheme without printing `RBJp<N>` headers. The exit-code family is the
load-bearing half; the printed label is its in-log companion.

#### ✅ JDo_103: `|| exit N` on every fallible operation

`set -e` guarantees an exit but carries the wrong code (the failing command's,
not the phase's). So every fallible operation pins its phase code explicitly:

```sh
iptables -P INPUT DROP || exit 10
```

For a *validation* (a `test`, not a command that fails loudly on its own), the
failure also prints a single-line, `FATAL:`-led message naming the expectation
and the actual value before exiting the phase code:

```sh
test -n "${RBJ_UPLINK_IF}" || { echo "FATAL: No uplink interface found"; exit 11; }
```

The message is for the log (`JDo_101`'s surface); the code is for the exit verdict
(`JDo_102`). A bare `set -e` death gives neither.

*Known deviation (citer):* the FATAL-message *shape* is not yet uniform across the
jailer scripts — `rbjs_sentry.sh` leads with `FATAL:` while `rbjp_pentacle.sh`
writes `RBJP: FATAL - …`. v1 blesses the type specimen's `FATAL:`-led form as
canonical and names the pentacle's form as the citable divergence.

### JDp_ — Parameter Transport (the substrate the dialect cannot trust)

#### ✅ JDp_101: Probe the env-file transport before trusting any parameter

Parameters reach a jailer script through the Docker/compose env-file mechanism,
whose quoting and word-splitting behavior is **at the Palisade** — not ours to
edit, and not guaranteed to deliver a multi-token value intact. Before reading
any real parameter, the script validates a known sentinel injected for exactly
this purpose:

```sh
echo "RBJp0: Validate compose env-file quoting"
: "${RBJE_PROBE:?}"
# assert RBJE_PROBE is exactly two tokens, "alpha" and "bravo"
```

If the transport mangled the sentinel (collapsed the quoting, split on the wrong
boundary), the probe dies at `exit 1` *before* any cargo parameter is trusted —
the channel is proven before its contents are read. This is the dialect-universal
instance of "trust nothing the substrate hands you."

As a membrane (CMK Rules of Engagement): the foreign behavior is *characterized*
(compose env-file quoting is not guaranteed to preserve token structure),
*contained* at one prologue, *absorbs only* the surveyed signature (a known
two-token sentinel) while everything else fails fast, and is *logged* as it
fires. It carries no removal condition because the env-file transport is a
standing dependency, not a transient bug awaiting a neighbor's fix.

---

## Related Documents

- **BCG** — Bash Console Guide. Host bash discipline; JDG is its in-vessel
  sibling — the philosophy crosses, the helpers do not.
- **CBG** — Cloud Build Guide. The closest structural sibling: foreign
  environment, no kit runtime, the log is the only surface. CBG's environment is
  a vendor builder container; JDG's is the security envelope.
- **WSG** — Windows Scripting Guide. The foreign-environment-sibling precedent:
  BCG's discipline re-expressed for an environment its helpers cannot reach.
- **ACG** Related Guides — the one-file-one-guide allocation: a jailer script
  answers to JDG (and to no other guide). Its *behavior* answers to its RBS
  specification (below) — a spec, not a guide, on an orthogonal axis.
- **RBSII**, **RBSSC**, **RBSSR**, **RBSSS** — the iptables/sentry behavior specs
  the type specimen implements. They govern *what* the script does; JDG governs
  *how* it is written.

## Acronym Registry

| Term | Expansion |
|------|-----------|
| JDG | Jailer Dialect Guide (this document) |
| Jailer (`rbj*`) | The zero-dependency in-vessel POSIX `sh` family — sentry (`rbjs`), pentacle (`rbjp`) |
| Phase anchor (`RBJp<N>`) | The greppable phase-announcement label — the execution-time census (`JDo_101`) |
| Exit-code family | Tens-digit-is-phase exit scheme; the exit code locates the failure (`JDo_102`) |
| Transport probe | The self-probing prologue that validates the env-file channel before trusting parameters (`JDp_101`) |
| `z_` scratch | The prefix marking short-lived globals, POSIX `sh` having no `local` |
| Health sentinel (`/tmp/rbj*h_healthy`) | The readiness file a configure-then-hold script touches before `exec sleep infinity` |
