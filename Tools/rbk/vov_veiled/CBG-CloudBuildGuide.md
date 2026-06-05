<!--
Copyright 2026 Scale Invariant, Inc.
SPDX-License-Identifier: Apache-2.0
See LICENSE for terms.
-->

# Cloud Build Guide (CBG) — Step-Body Discipline

## Purpose

CBG codifies the discipline for authoring the **body of a single Google Cloud
Build step** — the script that runs inside one step's builder container. It is a
foreign-environment sibling to BCG (host bash) and RCG (host Rust): it shares
their philosophy — crash-fast, no silent failures, load-bearing complexity — but
diverges entirely on mechanics, because a step body executes in an environment
with none of the host kit's scaffolding.

CBG governs **single-step precision**: what goes inside one step's script.
Component wiring, recipe composition, and build architecture are NOT CBG's
domain — those live in the specs (RBSCJ, RBSCB) and the heat paddocks. The one
seam CBG does cover on the host side is **how a step body is composed and
dispatched** (the include splice, the entrypoint switch, the substitution
contract), because that machinery is what a step author must write *against*.

CBG is **polyglot**. A step body is bash, POSIX `sh`, or `python3` — the recipe
selects the interpreter per step. The cloud-step *invariants* are
language-neutral; each language honors them with its own idiom. The guide is
structured to match: a language-neutral invariants core, then a dialect section
per language.

## Core Philosophy

**The step body has no kit runtime.** On the host, BCG code leans on
`buc_die`, `buc_step`, the kindle/sentinel lifecycle, `buv_*` validation, and
the module/CLI gateway. **None of that exists inside a step.** A step body is a
single file shipped into a vendor builder container and run with a bare
interpreter. Every guarantee BCG gets from BUK, a step body must re-establish in
a few lines of raw bash or stdlib python. The philosophy survives the crossing;
the mechanics are rebuilt minimally, in-language.

This is the same relationship WSG has to BCG: BCG's discipline, re-expressed for
a hostile environment that BCG's helpers cannot reach. WSG crosses the
ssh-to-Windows transport; CBG crosses into the cloud builder container.

**Crash-fast is non-negotiable, and louder here.** A step's stdout/stderr *is*
the Cloud Build log — the only forensic surface. There is no transcript file, no
`buc_log_*` sink, no post-mortem temp directory you can `ssh` back into reliably.
A failure that does not print and exit non-zero is a failure that vanishes. Every
fallible operation must die loudly, at its location, with a message naming what
failed.

**The vendor is at the Pale.** Cloud Build, the metadata server, the builder
images, and the substitution machinery are not ours to edit (per the Rules of
Engagement). We characterize their behavior precisely, contain our dependence on
it at named seams, and never paper over it. The metadata-token idiom, the
`automapSubstitutions` env channel, the builder image's pinned interpreter
version — these are surveyed foreign behaviors, absorbed at one membrane each.

**Load-bearing complexity still rules.** Reuse only the irreducibly identical
core; leave kind-specific assembly in the step. The composed-snippet library
(`rbgjs`) shares exactly the steps that are byte-identical across callers and no
more — see RBSCJ's composition rationale.

## The Cloud Step Environment

A step body runs under a set of conditions a host script never faces. Each is a
fault domain.

| Condition | Consequence for the body |
|-----------|--------------------------|
| **Substitutions arrive as environment variables.** `_RBGx_*` keys are expanded at submit time and provided to the step via `automapSubstitutions`. | The body reads them from the environment (`${_RBGx_…}` / `os.environ`), never as positional args. They may be empty; the body must assert presence. |
| **`/workspace` is a shared, ephemeral mount.** All steps in a build see the same `/workspace`; it is destroyed when the build ends. | It is the only inter-step channel. It is non-secret by invariant. Every read must be guarded — the producing step may have been skipped or failed. |
| **Steps can be retried silently.** Cloud Build may re-run a step on transient failure. | Every state-changing operation must be idempotent — safe to run twice. |
| **Authentication is ambient, via the metadata server.** The step inherits the build's service-account identity. | Tokens are fetched in-memory from `metadata.google.internal`, used, and discarded. They never touch `/workspace`. |
| **The interpreter is the builder image's, pinned.** `bash` may be old/busybox; the `gcloud` image ships python 3.10. | Code to the *shipped* runtime, not your laptop's. Guard newer stdlib features. |
| **There is no native step reuse.** Cloud Build has no anchors, no includes. | Shared logic is composed in **on the host** before submit (the `#@rbgjs_include` splice), not referenced at build time. |

The transport analogue to WSG's stack: a step body is authored on the host,
spliced and shebang-stamped by the assembler, serialized into a Build resource
JSON, submitted via the REST API, and finally executed by the builder. CBG
constrains the body; the host seam (CBh-) constrains the composition around it.

## Rule Families

Headers tag the rule: ❌ for a failure mode, ✅ for an established correct shape.

- **`CBi-`** — language-neutral cloud-step **invariants** (every body, any language).
- **`CBb-`** — **bash/sh** dialect rules.
- **`CBp-`** — **python** dialect rules.
- **`CBh-`** — **host-side composition** rules (the seam a body is written against).

Numbering starts at 101 within each family to leave room for insertions.

---

## CBi — Cloud Step Invariants (language-neutral)

### ✅ CBi-101: Die loudly, at the point of failure

Every fallible operation surfaces its own failure with a message to stderr and a
non-zero exit. There is no `buc_die` and no distant trap; the guard lives where
the failure happens. This is BCG's "error handling at the point of failure,"
stripped to what runs without BUK.

The message names *what* failed and, where it helps, *why the next step depends
on it* (e.g. "step 01 must run first"). The log is the only post-mortem surface.

### ✅ CBi-102: Substitutions are inputs — assert their presence

`_RBGx_*` values arrive as environment variables (submit-time expansion via
`automapSubstitutions`). They may be empty. A body asserts every required input
before use, and dies if absent (CBi-101). Optional inputs are read with an
explicit empty default and a documented "absent is legitimate" branch.

The substitution contract — which keys a step requires — is documented in the
step's header comment and enforced at dispatch by the spine validator (CBh-104).

### ✅ CBi-103: `/workspace` carries non-secret data only; guard every read

`/workspace` is the inter-step bus. Write provenance envelopes, rosters, build
context — never secrets. The invariant is recorded canonically in RBSCB
(*the `/workspace` carries non-secret build data only — secrets stay step-local*).

Every consumer guards: the producing step may not have run, or may have produced
nothing. Test for presence **and** non-emptiness before consuming, and die with a
message that names the missing producer.

### ✅ CBi-104: Secrets are step-local and in-memory; re-mint per step

Auth tokens are fetched fresh from the metadata server in each step that needs
them, held in a variable, and never written to `/workspace` or a file. A second
step that needs a token fetches its own — tokens are cheap and short-lived;
caching one across steps would force it onto the shared mount and violate
CBi-103. The metadata-token idiom is the canonical skopeo/GAR auth path
(rationale in RBSCB; the gcloud credential helper is unavailable to skopeo, and
grafting `docker-credential-gcr` into the skopeo container is documented
friction).

### ✅ CBi-105: Every state change is idempotent under retry

Cloud Build may re-run a step. Inspect-or-create, never bare-create; retag is a
no-op if the tag exists; a second run of the whole step must reach the same end
state without error. The `buildx-bootstrap` snippet is the exemplar: `inspect ||
create`, then `use` (a no-op if already selected).

### ✅ CBi-106: Use the language's native serializer when it is free; hand-roll only to avoid a container dependency

This is the load-bearing polyglot rule. **Structured output is JSON.** If the
step's interpreter gives you a safe JSON serializer for free, use it. If using
one would mean pulling a tool into the builder container that isn't already
there, hand-roll with controlled data instead.

- **python**: `json.dump` / `json.loads` — stdlib, free. Always use it.
- **bash**: hand-rolled `printf`/concatenation **inside the step**, because
  pulling `jq` into a minimal skopeo container is friction. Permitted *only*
  because every interpolated value is controlled (hex digests, ISO timestamps,
  sanitized origins, SA emails) — none can carry a literal quote. Never
  hand-roll JSON over user-influenced or unsanitized data.
- **host side**: `jq -n --arg/--slurpfile` always (CBh-103) — the host has `jq`,
  so there is no excuse to hand-roll there.

Same rule — "free serializer wins; hand-roll only against a dependency cost" —
opposite mechanic per environment.

### ✅ CBi-107: Code to the builder image's runtime, not your own

The interpreter version is whatever the builder image ships. The `gcloud` image
ships python 3.10; `bash` in a minimal builder may be old or busybox. Guard any
feature newer than the pinned runtime (e.g. python's `tarfile` `filter=` arrived
in 3.12 — the discover-platforms step branches on `sys.version_info`). This is
the cloud analogue of BCG's bash-3.2 compatibility target.

### ✅ CBi-108: Separate pure logic from network ops

Resolve inputs, build paths, and author envelopes in a block with no network
calls; isolate `skopeo`/`gcloud`/`curl`/`urllib` calls in their own block. The
pure block is testable without the cloud; the network block can be stubbed. This
is BCG's temp-file/visible-transformation philosophy applied to the step's
internal structure.

---

## CBb — Bash / sh Dialect

### ✅ CBb-101: One file, `set -euo pipefail` first

A bash/sh step body is a single file. Its first executable line is
`set -euo pipefail`. There is no module header, no kindle, no sentinel — those
are BUK host constructs that do not exist in the builder. The shebang is stamped
by the assembler, not written by the author (CBh-101).

### ✅ CBb-102: Inline FATAL guard replaces `buc_die`

With no BUK, the die idiom is an inline brace group:

```bash
some_command \
  || { echo "FATAL: <what failed> — <why it matters>" >&2; exit 1; }
```

`FATAL:` to stderr, `exit 1`. This is the in-step spelling of CBi-101. Use it on
every fallible command and after every `test`.

### ✅ CBb-103: Guarded `$( … )` with `pipefail` is permitted in-step — BCG's temp-file mandate is relaxed

BCG bans pipelines inside `$()` and unguarded command substitution, mandating
temp files so each transformation is visible and recoverable. **In a step body,
that mandate is relaxed.** Temp-file forensics are low-value in an ephemeral
container whose stdout is already the build log, and `set -o pipefail` plus an
immediate presence check covers the failure surface:

```bash
TOKEN=$(printf '%s' "${TOKEN_JSON}" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
test -n "${TOKEN}" || { echo "FATAL: no access_token in metadata response" >&2; exit 1; }

SHA=$(sha256sum "${RAW_FILE}" | cut -d' ' -f1)
test -n "${SHA}" || { echo "FATAL: empty digest for ${ORIGIN}" >&2; exit 1; }
```

The relaxation is **characterized, not careless**: it holds because (a) the log
is the forensic surface, (b) `pipefail` is set, and (c) the result is immediately
validated. Outside a step body, BCG's rule stands.

### ✅ CBb-104: Capture substitutions into shell vars immediately

Substitutions are not shell variables you can loop over directly; capture them
into plain shell vars at the top of the body, then work with those:

```bash
SLOT_1_ORIGIN="${_RBGL_IMAGE_1_ORIGIN}"
SLOT_2_ORIGIN="${_RBGL_IMAGE_2_ORIGIN}"
SLOT_3_ORIGIN="${_RBGL_IMAGE_3_ORIGIN}"
```

Snippets (CBh-102) read these **plain shell vars**, never the `_RBGx_` names —
that is what lets one snippet serve callers in disjoint substitution namespaces.

### ✅ CBb-105: Hand-rolled JSON, controlled data only

Per CBi-106, in-step JSON is `printf`/concatenation over controlled values:

```bash
# No jq inside the step — values are controlled (sanitized origin, hex digest,
# SA email, build id, ISO timestamp); none can carry a literal quote.
ENVELOPE='{'
ENVELOPE="${ENVELOPE}\"schema\":\"${_RBGL_VOUCH_SCHEMA}\","
ENVELOPE="${ENVELOPE}\"digest\":\"sha256:${SHA}\","
ENVELOPE="${ENVELOPE}\"tags\":[\"${_RBGL_TAG_BOLE}\",\"${FINGERPRINT}\"]"
ENVELOPE="${ENVELOPE}}"
```

If a value could ever carry a quote or newline, it is not controlled — do not
hand-roll it.

### ✅ CBb-106: skopeo authenticates with the in-memory token; copy `--all`

GAR reads/writes use the metadata token as credentials, never a credential
helper; multi-arch safety requires `--all`:

```bash
skopeo copy --all \
  "docker://${ORIGIN}" \
  "docker://${PKG}:${DIGEST_TAG}" \
  --dest-creds "oauth2accesstoken:${TOKEN}" \
  || { echo "FATAL: skopeo copy failed for ${ORIGIN}" >&2; exit 1; }
```

GAR→GAR retag (same blobs, manifest re-tag) supplies both `--src-creds` and
`--dest-creds`.

---

## CBp — Python Dialect

Python steps run on the builder image's interpreter (CBi-107 — currently 3.10 on
the `gcloud` image) with **stdlib only** — no pip install in-step. The cloud
invariants are honored through a small set of helper functions.

### ✅ CBp-101: The crash-fast preamble — `die()` + `require_env()`

Every python step opens with the same two helpers — the in-language spelling of
CBi-101 and CBi-102:

```python
def die(msg):
    print(f"FATAL: {msg}", file=sys.stderr)
    sys.exit(1)

def require_env(name):
    val = os.environ.get(name, "")
    if not val:
        die(f"{name} missing")
    return val
```

Read required substitutions through `require_env`; read optional ones with
`os.environ.get(name, "")` and an explicit absent-branch.

### ✅ CBp-102: Metadata token via `urllib`, in memory

```python
METADATA_TOKEN_URL = (
    "http://metadata.google.internal/computeMetadata/v1/"
    "instance/service-accounts/default/token"
)

def metadata_token():
    req = urllib.request.Request(METADATA_TOKEN_URL, headers={"Metadata-Flavor": "Google"})
    resp = urllib.request.urlopen(req)
    return json.loads(resp.read())["access_token"]
```

The token is a local, passed to `gar_fetch` as a Bearer header. It is never
written to `/workspace` (CBi-104).

### ✅ CBp-103: `json.dump` / `json.loads` for all structured I/O

Per CBi-106, python always uses the stdlib serializer — never hand-rolled
strings. Output files are written to `/workspace`:

```python
with open("/workspace/vouch_summary.json", "w") as f:
    json.dump(vouch_summary, f, indent=2)
    f.write("\n")
```

### ✅ CBp-104: Guard stdlib features newer than the builder runtime

```python
if sys.version_info >= (3, 12):
    tar.extractall(filter="data")   # filter= is 3.12+
else:
    tar.extractall()                # gcloud image ships 3.10
```

CBi-107 in python form. Pin your assumptions to the builder image, not your
laptop.

### ✅ CBp-105: Subprocess to external tools with `check=True`

When a python step shells out (e.g. `gcloud`, `openssl`), use `subprocess.run`
with `check=True` so a non-zero exit raises rather than passing silently;
capture output for the log. Where a non-zero exit is itself the verdict (e.g. an
`openssl` signature check), omit `check=True` and inspect `returncode`
explicitly, then `die` on the failing branch:

```python
result = subprocess.run([...], capture_output=True, text=True, check=True)

verify = subprocess.run([...], capture_output=True, text=True)   # verdict by code
if verify.returncode != 0:
    die(f"DSSE verify FAILED for {ps}")
```

### ❌ CBp-106: The python helper preamble is duplicated — a known reuse gap

The bash steps share a composed-snippet library (`rbgjs`, CBh-102); the python
steps do **not** yet. `die`, `require_env`, `metadata_token`, `gar_fetch`, and
`gar_json` appear **verbatim** in every python step (`rbgja01`, `rbgjv02`, …).

This is a recorded asymmetry, not an endorsed pattern. Until a python-side
composition mechanism exists, keep the preamble **byte-identical** across steps
so it can be lifted into a shared snippet later without reconciliation. Treat
divergence in these helpers as a bug. (Closing the gap — a python `#@`-include
analogue — is out of the capture-unification heat's scope; it belongs to a
future itch.)

---

## CBh — Host-Side Composition (the seam a body is written against)

A step author does not write the shebang, the JSON envelope, or the dispatch.
The assembler and spine do. CBh rules describe that machinery so a body is
authored to fit it.

### ✅ CBh-101: A step is a recipe row; the entrypoint selects the interpreter

The assembler (`rbfca_StepAssembly.sh`) and spine (`rblds_Spine.sh`) describe a
build as ordered recipe rows:

```
script_path | builder | id | entrypoint
```

`entrypoint` is `bash`, `sh`, or `python3`, and selects the shebang the spine
stamps onto the body:

```bash
python3) z_shebang="#!/usr/bin/env python3" ;;
```

The author writes the body and declares its interpreter via the row's entrypoint
— nothing else about shebang or invocation.

### ✅ CBh-102: Shared bash logic is spliced in on the host via `#@rbgjs_include`

Cloud Build has no native step reuse (CBi: "no native step reuse"), so shared
**bash** logic is composed *before submit*. A step marks an include point:

```bash
#@rbgjs_include token-fetch
#@rbgjs_include skopeo-fingerprint
```

The host expander (`zrbfc_expand_includes` in `rbfcb_BuildHost.sh`) replaces each
marker with the body of `rbgjs/rbgjs-<name>.sh`, shebang stripped. A body with no
markers is rewritten unchanged (the expander is safe to call on every step). A
missing snippet crashes the expander — no silent skip.

**Snippet contract** (from RBSCJ): snippets read **plain shell vars** the step
set before the marker (CBb-104), not `_RBGx_` substitutions, and are idempotent
(CBi-105). The library shares only the irreducibly identical core; kind-specific
assembly stays in the step.

| Snippet | Requires | Provides |
|---------|----------|----------|
| `token-fetch` | none (ambient metadata) | `TOKEN` |
| `skopeo-fingerprint` | `ORIGIN`, `RAW_FILE` | `SHA`, `FINGERPRINT` |
| `buildx-bootstrap` | none | the `rb-builder` buildx builder |
| `buildx-push` | `PUSH_URI`, `PUSH_PLATFORMS`, `PUSH_CTX` | the image pushed |

This mechanism is **bash-only today** — see CBp-106 for the python gap.

### ✅ CBh-103: The Build resource JSON is composed on the host with `jq`

The spine builds the Build resource via `jq -n` with `--arg`/`--slurpfile` — safe
composition, never string concatenation, never YAML (Cloud Build's REST wire
format is JSON; YAML's multiline/indentation/`$$`-escaping concerns die with it —
see RBSCJ). This is the host half of CBi-106.

### ✅ CBh-104: The substitution blob is opaque to the spine; coverage is validated at dispatch

A per-kind body file builds the substitution blob (`jq -n` → `_RBGx_` keys) and
hands it to the spine, which passes it through **without reading any individual
key** — keeping the spine kind-agnostic. Before the expensive submit, the
validator scans each include-expanded step body for the `_RBGx_` tokens it
references (*requires*) and fails if any is absent from the blob's keys
(*provides*). This dispatch-time check is the heat's one declared behavior-add;
its reject path carries its own unit tests.

A step author's obligation: every `_RBGx_` the body reads must be a key the
kind's substitution blob provides.

### ✅ CBh-105: A step returns data to the host via `buildStepOutputs`

A step that must hand a value back to the host writes it to the
`buildStepOutputs` channel (`/builder/outputs/output`); the spine extracts the
slot and base64-decodes it host-side into a capture file. This is the only
step→host return path; `/workspace` does not survive the build.

---

## Snippet & Exemplar Reference

The patterns above are distilled from these exemplars — read them as the worked
forms:

- **Bash snippets** — `Tools/rbk/rbgjs/rbgjs-{token-fetch,skopeo-fingerprint,buildx-bootstrap,buildx-push}.sh`
- **Bash steps** — `rbgjl01-ensconce-capture.sh`, `rbgjl02-assemble-push-vouch.sh`, `rbgjv03-assemble-push-vouch.sh`
- **Python steps** — `rbgja/rbgja01-discover-platforms.py`, `rbgja/rbgja03-build-info-per-platform.py`, `rbgjv/rbgjv02-verify-provenance.py`
- **Host composition** — `rblds_Spine.sh` (spine, validator, entrypoint switch, `buildStepOutputs` extract), `rbfca_StepAssembly.sh` (recipe rows), `rbfcb_BuildHost.sh` (`zrbfc_expand_includes`)
- **Per-kind body** — `rbldb_Bole.sh` (recipe + substitution blob + envelope intent)

## Related Documents

- **BCG** — Bash Console Guide. The host bash discipline CBb relaxes/diverges from in-step.
- **RCG** — Rust Coding Guide. Host Rust sibling.
- **WSG** — Windows Scripting Guide. The structural precedent: BCG's discipline re-expressed for a hostile foreign environment.
- **RBSCJ** — CloudBuildJson. The JSON-composition trade study and the composed-snippet/contract decision (the home of the composition rationale CBh points at).
- **RBSCB** — CloudBuildPosture. The skopeo-token/credential-helper posture and the canonical `/workspace`-no-secrets invariant (CBi-103/CBi-104).

## Acronym Registry

| Term | Expansion |
|------|-----------|
| CBG | Cloud Build Guide (this document) |
| Step body | The script run inside one Cloud Build step's builder container |
| Substitution | A `_RBGx_` value expanded at submit time, provided to the step as an env var |
| `/workspace` | The shared, ephemeral inter-step filesystem mount (non-secret only) |
| Snippet | A composed-once bash fragment spliced into a step via `#@rbgjs_include` |
| Spine | The host capture-assembly composer (`rblds_`) that submits and polls a build |
