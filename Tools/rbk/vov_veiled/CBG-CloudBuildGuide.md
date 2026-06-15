<!--
Copyright 2026 Scale Invariant, Inc.
SPDX-License-Identifier: Apache-2.0
See LICENSE for terms.
-->

# Cloud Build Guide (CBG) — Step-Body Discipline

*Guide-family member — see GMG (Guide Meta-Guide) for the family's shared authoring conventions.*

## Purpose

CBG codifies the discipline for authoring the **body of a single Google Cloud
Build step** — the script that runs inside one step's builder container. It is a
foreign-environment sibling to BCG (host bash) and RCG (host Rust): it shares
their philosophy — crash-fast, no silent failures, load-bearing complexity — but
diverges entirely on mechanics, because a step body executes in an environment
with none of the host kit's scaffolding.

CBG governs **single-step precision**: what goes inside one step's script. The
one seam it also covers is **how a step body is composed and dispatched** (the
include splice, the substitution contract, the return channel), because a body
must be authored *against* that machinery.

CBG is **polyglot**. A step body is bash, POSIX `sh`, or `python3` — the recipe
selects the interpreter per step. The cloud-step disciplines are
language-neutral; each language honors them with its own idiom.

## How this document is organized — two genres on purpose

CBG deliberately separates two kinds of knowledge, because they earn their form
differently:

- **Authored Disciplines** (prose, no IDs) — systematic, interderivable patterns
  for code *we* write. Like BCG, these are internalized, not cited by number;
  numbering them would be ceremony with no citers.
- **Cited Rules** (numbered `CB*-`) — discrete facts about the cloud Palisade we do
  *not* control (surprises, divergences, a recorded gap), each of which
  something will *point at*: a step comment justifying a workaround, a memo
  recording a probe, a review flagging a violation, or the dispatch validator.
  An ID earns its existence only when a citer will exist.

This split is itself an instance of the load-bearing test (BCG Core Philosophy):
catalog what is foreign; systematize what is yours.

## Core Philosophy

**The step body has no kit runtime.** On the host, BCG code leans on `buc_die`,
`buc_step`, the kindle/sentinel lifecycle, `buv_*` validation, and the module/CLI
gateway. **None of that exists inside a step.** A step body is a single file
shipped into a vendor builder container and run with a bare interpreter. Every
guarantee BCG gets from BUK, a step body must re-establish in a few lines of raw
bash or stdlib python. The philosophy survives the crossing; the mechanics are
rebuilt minimally, in-language. This is the relationship WSG has to BCG: the same
discipline, re-expressed for an environment BCG's helpers cannot reach.

**Crash-fast is non-negotiable, and louder here.** A step's stdout/stderr *is*
the Cloud Build log — the only forensic surface. There is no transcript file, no
`buc_log_*` sink, no temp directory to inspect afterward. A failure that does not
print and exit non-zero is a failure that vanishes. Every fallible operation dies
loudly, at its location, with a message naming what failed.

**The vendor is at the Palisade.** Cloud Build, the metadata server, the builder
images, and the substitution machinery are not ours to edit (Rules of
Engagement). We characterize their behavior precisely, contain our dependence at
named seams, and never paper over it. The Cited Rules below *are* that
characterization.

**Load-bearing complexity still rules.** Reuse only the irreducibly identical
core; leave kind-specific assembly in the step (RBSCJ composition rationale).

## The Cloud Step Environment

A step body runs under conditions a host script never faces. Each is a fault
domain — the source of most Cited Rules below.

| Condition | Consequence for the body |
|-----------|--------------------------|
| **Substitutions arrive as environment variables** (`_RBGx_*`, expanded at submit via `automapSubstitutions`). | Read from the environment, not argv; they may be empty (CBi_101). |
| **`/workspace` is a shared, ephemeral mount** seen by every step, destroyed at build end. | The only inter-step channel; non-secret by invariant; guard every read (CBi_102). |
| **Steps can be retried silently** on transient failure. | Every state change must be idempotent (CBi_103). |
| **Authentication is ambient**, via the metadata server. | Tokens are fetched in-memory, used, discarded — never to `/workspace` (CBi_102). |
| **The interpreter is the builder image's, pinned** (e.g. `gcloud` ships python 3.10). | Code to the shipped runtime; guard newer features (CBi_104). |
| **No native step reuse** (no anchors, no includes). | Shared bash logic is composed in on the host before submit (CBh_101). |

---

## Authored Disciplines (prose — internalize, don't cite)

### Crash-fast, spelled per language

The step body re-implements BCG's die-at-point-of-failure without BUK.

**bash/sh** — an inline brace group to stderr, then `exit 1`, on every fallible
command and after every `test`:

```bash
some_command \
  || { echo "FATAL: <what failed> — <why it matters>" >&2; exit 1; }
```

**python** — the two-helper preamble that opens every python step:

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

`die` is python's `buc_die`; `require_env` is the assert-presence discipline for
substitutions (CBi_101). Read required inputs through `require_env`; read
optional ones with `os.environ.get(name, "")` and an explicit absent-branch.

When a python step shells out, use `subprocess.run(..., check=True)` so a
non-zero exit raises — unless a non-zero exit is itself the verdict (e.g. an
`openssl` signature check), in which case omit `check=True` and inspect
`returncode`, then `die` on the failing branch.

### The Native-Serializer Rule

Structured output is JSON. **Use the interpreter's native serializer when it is
free; hand-roll only to avoid pulling a tool into the builder container.** Same
rule everywhere, opposite mechanic by environment:

- **python**: `json.dump` / `json.loads` — stdlib, free. Always.
- **host**: `jq -n --arg/--slurpfile` — the host has `jq`; never concatenate,
  never YAML (Cloud Build's wire format is JSON; RBSCJ).
- **bash, inside a step**: hand-rolled `printf`/concatenation, because pulling
  `jq` into a minimal gcrane container is friction — and permitted *only* because
  every interpolated value is controlled (hex digests, ISO timestamps, sanitized
  origins, SA emails), none of which can carry a literal quote:

```bash
# No jq inside the step — values are controlled; none can carry a quote.
ENVELOPE='{'
ENVELOPE="${ENVELOPE}\"digest\":\"sha256:${SHA}\","
ENVELOPE="${ENVELOPE}\"tags\":[\"${_RBGL_TAG_BOLE}\",\"${FINGERPRINT}\"]"
ENVELOPE="${ENVELOPE}}"
```

If a value could ever carry a quote or newline, it is not controlled — do not
hand-roll it.

Key naming rides ACG's wire-format clause: JSON this project authors on both
sides carries the format's minted key-sprue (e.g. `rblv_*` for the Lode vouch
envelope); JSON conforming to a foreign schema keeps the foreign keys. Rule and
conversion move: ACG "wire formats are named homes" / ACGm_108.

### Separate pure logic from network ops

Resolve inputs, build paths, and author envelopes in a block with no network
calls; isolate `gcrane`/`gcloud`/`curl`/`urllib` calls in their own block. The
pure block is testable without the cloud; the network block can be stubbed.
BCG's visible-transformation philosophy applied to a step's internal structure.

### Step-body skeleton

**bash/sh**: a single file; first executable line `set -euo pipefail`; no module
header, kindle, or sentinel (BUK constructs absent in the builder). The shebang
is stamped by the assembler, not written by the author (CBh_101).

**python**: stdlib only — no `pip install` in-step; the preamble above; `main()`
guarded by `if __name__ == "__main__":`.

### GAR authentication idiom

GAR reads/writes use gcrane's ambient `google.Keychain`, which draws credentials
from Application Default Credentials → the GCE metadata server (the Mason SA's
ambient identity on Cloud Build workers).
No token fetch, no `crane auth login`, no credential-helper image.
The same ambient model the docker-based steps use, extended from docker to crane — one auth story across all steps.

```bash
gcrane cp "${ORIGIN}" "${PKG}:${DIGEST_TAG}" \
  || { echo "FATAL: gcrane cp failed for ${ORIGIN}" >&2; exit 1; }
```

The *invariant* that credentials never reach `/workspace` is CBi_102; the ambient
model honors it structurally — no token is ever minted by the step body.
(Historical: skopeo used an in-memory metadata token via `--dest-creds oauth2accesstoken:${TOKEN}`;
that pattern is retired. Rationale for why a credential-helper was rejected is in RBSCB.)

---

## Cited Rules (numbered — each has, or will have, a citer)

Headers tag the rule: ❌ a failure mode, ✅ an established correct shape.
Families: `CBi_` cloud-step invariants (any language), `CBb_` bash/sh, `CBp_`
python, `CBh_` host-composition seam. Numbered from 101 to leave room for
insertions; once a rule has a citer it is never renumbered.

### CBi — Cloud Step Invariants

#### ✅ CBi_101: Substitutions are inputs — assert presence

`_RBGx_*` values arrive as environment variables (submit-time expansion via
`automapSubstitutions`), not as arguments, and **may be empty**. Assert every
required input before use and die if absent; read optional inputs with an
explicit empty default and a documented absent-branch. The set a step requires is
declared in its header comment and enforced at dispatch (CBh_102).

*Cited by:* step header contracts; the dispatch validator.

#### ✅ CBi_102: The `/workspace` boundary — non-secret only, secrets step-local, guard every read

`/workspace` is the inter-step bus. Write provenance envelopes, rosters, build
context — **never secrets**. Tokens and credentials stay step-local and
in-memory, re-minted per step (RBSCB records the canonical invariant). Every
consumer guards: the producing step may not have run or may have produced
nothing — test presence **and** non-emptiness, and die naming the missing
producer:

```bash
test -f /workspace/lode_stamps.txt \
  || { echo "FATAL: lode_stamps.txt not found — step 01 must run first" >&2; exit 1; }
test -s /workspace/lode_stamps.txt \
  || { echo "FATAL: lode_stamps.txt is empty — nothing ensconced" >&2; exit 1; }
```

*Cited by:* RBSCB; every `/workspace` guard and the GAR-auth idiom.

#### ✅ CBi_103: Every state change is idempotent under retry

Cloud Build may silently re-run a step. Inspect-or-create, never bare-create;
retag is a no-op if the tag exists; a second run reaches the same end state
without error. Exemplar — the `buildx-bootstrap` snippet: `inspect || create`,
then `use`.

*Cited by:* the bootstrap snippet header; any retry post-mortem.

#### ✅ CBi_104: Code to the builder image's pinned runtime

The interpreter version is whatever the builder image ships — not your laptop's.
Guard any feature newer than the pinned runtime:

```python
if sys.version_info >= (3, 12):
    tar.extractall(filter="data")   # filter= is 3.12+
else:
    tar.extractall()                # gcloud image ships 3.10
```

*Cited by:* version-guard sites; a memo if a runtime mismatch bites.

#### ✅ CBi_105: A step uses only the tools its builder image ships — never install at build time

A body that `apt-get install`s or `pip install`s a tool pulls unpinned bytes from
a public repository into a privileged build at run time — a supply-chain hole no
digest pin can see, and friction every silent retry pays again. A missing tool is
a **builder-selection problem, not an installation problem**: move the work to a
step whose builder ships the tool (the JSON-work ↔ registry-work split across the
gcloud/gcrane builders is the worked form), or take another in-language path
(python stdlib instead of jq). The skeleton's "no `pip install`" is this rule's
python face; `apt-get install` is its bash face — and the bash face is the
sneakier one, because `apt-get` itself is allowlisted for the conformance walk.

*Cited by:* step reviews (first citer: the immure select-step draft that
apt-installed jq, reworked 2026-06-10); builder-selection notes in step headers.

### CBb — Bash / sh Dialect

#### ✅ CBb_101: Guarded `$( … )` with `pipefail` is permitted in-step — BCG's temp-file mandate is relaxed

BCG bans pipelines inside `$()` and unguarded command substitution, mandating
temp files so each transformation is visible and recoverable. **Inside a step
body that mandate is relaxed:** temp-file forensics are low-value in an ephemeral
container whose stdout is already the build log, and `set -o pipefail` plus an
immediate presence check covers the failure surface.

```bash
TOKEN=$(printf '%s' "${TOKEN_JSON}" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
test -n "${TOKEN}" || { echo "FATAL: no access_token in metadata response" >&2; exit 1; }
```

The relaxation is **characterized, not careless**: it holds because (a) the log
is the forensic surface, (b) `pipefail` is set, and (c) the result is immediately
validated. Outside a step body, BCG's rule stands. This is the rule a reviewer
needs a handle for — a step comment can cite `CBb_101` to preempt "this violates
BCG."

*Cited by:* step-body comments; code review.

#### ✅ CBb_102: Capture substitutions into shell vars before snippets read them

Substitutions are not shell variables you can loop over; capture them into plain
shell vars at the top of the body. Snippets (CBh_101) read these **plain shell
vars**, never the `_RBGx_` names — that is what lets one snippet serve callers in
disjoint substitution namespaces.

```bash
SLOT_1_ORIGIN="${_RBGL_IMAGE_1_ORIGIN}"
SLOT_2_ORIGIN="${_RBGL_IMAGE_2_ORIGIN}"
```

*Cited by:* the snippet contract (CBh_101).

### CBp — Python Dialect

#### ❌ CBp_101: The python helper preamble is duplicated — a known reuse gap

The bash steps share a composed-snippet library (`rbgjs`, CBh_101); the python
steps do **not**. `die`, `require_env`, `metadata_token`, `gar_fetch`, and
`gar_json` appear **verbatim** in every python step (`rbgja01`, `rbgjv02`, …).

This is a recorded asymmetry, not an endorsed pattern. Until a python-side
composition mechanism exists, keep the preamble **byte-identical** across steps
so it can be lifted into a shared snippet later without reconciliation; treat
divergence as a bug. Closing the gap (a python `#@`-include analogue) is out of
the capture-unification heat's scope — a future itch.

**Probe (2026-06-10): the expander is already nearly language-blind.** The
`#@rbgjs_include` marker is a valid python comment, the splice is pure line
copying, and the first-line shebang strip handles a python shebang — so a
shared python preamble snippet would splice through `zrbfc_expand_includes`
with zero expander surgery. The one bash-shaped assumption is the hardcoded
snippet filename `rbgjs-«name».sh`: a python snippet must either ride that
`.sh` suffix despite its content or wait for a naming decision (suffix from
the body's extension, or a python-named sibling). The itch is a snippet-naming
choice, not expander work.

*Cited by:* a review flagging preamble drift; the future itch that closes it.
*Removal condition:* delete this rule when python steps gain a shared preamble.

#### ✅ CBp_102: Python bodies are inside the conformance walk — the cupel holds the tool floor

The supply-chain cupel walks the python cloud steps (`rbgj*/*.py`) alongside
the bash domains: every import is held to a stdlib floor anchored on its module
root, dynamic-import surface (`importlib`/`__import__`/`exec`/`eval`) is banned
outright, and `subprocess` argv[0] literals are classified against the same GCB
tool floor as bash command positions — one floor, two languages. What an author
must know:

- **Imports: stdlib only.** The floor's authoritative home is the doc comment
  on `ZRBTDRU_PY_IMPORT_ALLOWED` (`rbtdru_cupel.rs`), beside the tool floor on
  `ZRBTDRU_GCB_ALLOWED` — reference the home, never restate the list (ACG). A
  third-party import binds the step to the floating builder's unpinned pip set;
  the fixture fails it.
- **Shell out as `subprocess.«fn»([...])` with a literal argv[0]**, importing
  the module unaliased. `from subprocess import …` is rejected (it hides
  argv[0] from the scan); an aliased import (`import subprocess as sp`) evades
  the lexer-grade scan — review holds that line. Prefer in-process
  `urllib`/REST over shelling out at all.

*Cited by:* memo-20260610-heat-BH-fable-recommendation-python-import-allowlist;
the cupel python case (`rbtdru_gcb_python`).

### CBh — Host-Composition Seam (what a body is authored against)

The author writes the body, not the shebang, the Build JSON, or the dispatch.
These rules describe the machinery a body must fit.

#### ✅ CBh_101: Shared bash logic is spliced on the host via `#@rbgjs_include`

Cloud Build has no native step reuse, so shared **bash** logic is composed
*before submit*. A step marks an include point; the host expander
(`zrbfc_expand_includes`, `rbfcb_host.sh`) replaces each marker with the
body of `rbgjs/rbgjs-<name>.sh`, shebang stripped. A marker-free body is rewritten
unchanged (safe to call on every step); a missing snippet crashes the expander —
no silent skip.

```bash
#@rbgjs_include token-fetch
#@rbgjs_include gcrane-fingerprint
```

**Snippet contract** (RBSCJ): snippets read **plain shell vars** set before the
marker (CBb_102), not `_RBGx_` substitutions, and are idempotent (CBi_103). The
library shares only the irreducibly identical core; kind-specific assembly stays
in the step.

| Snippet | Requires | Provides |
|---------|----------|----------|
| `token-fetch` | none (ambient metadata) | `TOKEN` |
| `gcrane-fingerprint` | `ORIGIN`, `RAW_FILE` | `SHA`, `FINGERPRINT` |
| `gcrane-append` | `APPEND_CTX`, `APPEND_URI` | the FROM-scratch single-layer image pushed to `APPEND_URI` |
| `gpg-verify-sums` | `SUMS_URL`, `SIG_URL`, `TARGET_BASENAME`, `KEY_FPR` | `EXPECTED_SHA` |
| `buildx-bootstrap` | none | the `rb-builder` buildx builder |
| `buildx-push` | `PUSH_URI`, `PUSH_PLATFORMS`, `PUSH_CTX` | the image pushed |

Bash-only today — see CBp_101 for the python gap. The recipe row's `entrypoint`
field (`bash`/`sh`/`busybox`/`python3`) selects the shebang the spine stamps; the
author declares the interpreter there (`busybox` → `#!/busybox/sh`, the only shell
in a distroless `:debug` builder such as `gcrane:debug`).

*Cited by:* every `#@rbgjs_include` site; the snippet contract.

#### ✅ CBh_102: The substitution blob is opaque to the spine; coverage is validated at dispatch

A per-kind body builds the substitution blob (`jq -n` → `_RBGx_` keys) and hands
it to the spine, which passes it through without reading any individual key
(keeping the spine kind-agnostic). The spine adds exactly one key of its own —
`_RBGL_GIT_COMMIT`, the dispatching HEAD commit, dispatch provenance it owns
like the pool and timeout; bodies never declare it. Before the expensive submit,
the validator scans each include-expanded step body for the `_RBGx_` tokens it
references (*requires*) and fails if any is absent from the blob's keys
(*provides*) — the heat's one declared behavior-add, whose reject path carries
its own unit tests. A body author's obligation: every `_RBGx_` the body reads
must be a key the kind's blob provides (or the spine's one injected key).

*Cited by:* the validator's reject message; its unit tests.

#### ✅ CBh_103: A step returns data to the host via `buildStepOutputs`

`/workspace` does not survive the build. A step that must hand a value back writes
it to the `buildStepOutputs` channel (`/builder/outputs/output`); the spine
extracts the slot and base64-decodes it host-side into a capture file. This is
the only step→host return path. The slot is addressed **by step index**
(`.results.buildStepOutputs[N]`) — N is the producing step's position in the
recipe (the API contract orders outputs "corresponding to build step indices"),
so reordering recipe rows silently shifts slots; the body's extract call pins N
with a comment naming the producing step id. The channel is contract-capped:
**only the first 4KB is stored** (Build API, `results.buildStepOutputs`) — a
value that can outgrow 4KB does not belong on this channel.

*Cited by:* the spine's extract logic; any step producing a host-consumed value.

---

## Snippet & Exemplar Reference

Worked forms live where the code lives — read the current family members rather
than a curated filename list that drifts:

- **Bash snippets** — `Tools/rbk/rbgjs/`; the CBh_101 table is the contract home
  (requires/provides per snippet)
- **Step bodies, all languages** — `Tools/rbk/rbgj*/`; the newest step in each
  language is usually the best exemplar
- **Host composition** — the spine (`rblds_`), recipe assembly (`rbfca_`), and
  the include expander (`rbfcb_`)
- **Per-kind bodies** — the `rbld` family; bole is the smallest worked form

## Related Documents

- **BCG** — Bash Console Guide. Host bash discipline; CBb_101 relaxes one of its rules in-step.
- **RCG** — Rust Coding Guide. Host Rust sibling.
- **WSG** — Windows Scripting Guide. Structural precedent: BCG's discipline re-expressed for a hostile foreign environment, catalogued as cited rules.
- **RBSCJ** — CloudBuildJson. JSON-composition trade study; home of the composed-snippet/contract decision CBh points at.
- **RBSCB** — CloudBuildPosture. GAR authentication posture (gcrane ambient auth; skopeo credential-helper rejection history) and the canonical `/workspace`-no-secrets invariant (CBi_102).

## Acronym Registry

| Term | Expansion |
|------|-----------|
| CBG | Cloud Build Guide (this document) |
| Step body | The script run inside one Cloud Build step's builder container |
| Substitution | A `_RBGx_` value expanded at submit time, provided to the step as an env var |
| `/workspace` | The shared, ephemeral inter-step filesystem mount (non-secret only) |
| Snippet | A composed-once bash fragment spliced into a step via `#@rbgjs_include` |
| Spine | The host capture-assembly composer (`rblds_`) that submits and polls a build |
