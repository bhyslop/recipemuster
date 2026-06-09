# Memo: Lode podvm capture — cerebro experiment (₢BHAAK)

**Date:** 2026-06-08
**Heat / pace:** ₣BH (rbk-11-mvp-lode-universal-capture) / ₢BHAAK (lode-podvm-cerebro-experiment)
**Venue:** cerebro (Linux x86_64, Ubuntu 24.04, glibc 2.39) — by express, throwaway exception (RBK workstation is bash/curl/openssl/jq only and must stay that way).
**Author:** Claude Opus 4.8, driven by Brad.

This memo is the durable record of an experiment whose physical evidence (scratch
binaries, captured GAR Lodes) is destroyed at teardown by design. It establishes
how Recipe Bottle should bring podman-VM machine-os images under control in GAR.
It does **not** build the production cloud capture — that is a separate follow-up
pace this experiment unblocks.

---

## 1. Bottom line (the cloud-layer decision)

- **Tool: `crane`** for the production cloud capture step. Both crane and oras copy
  the podvm artifact with **perfect digest fidelity**; skopeo is confirmed out.
  crane wins on: Google's own go-containerregistry (already proven in the retired
  RBSOB OCI-layout bridge and across our GCB/GAR), `crane cp`-by-digest is
  get-or-error (the loud failure the *recorded* trust grade wants), and it handles
  both the OCI-artifact disk leaves **and** ordinary container images uniformly.
  **oras is a fully-viable fallback** (artifact-native, also digest-faithful) and
  is the better tool if the step ever needs `oras discover`/referrers semantics.
- **Auth: `gcrane`, not plain `crane` — ambient Google keychain, zero token
  plumbing.** The cloud step uses **`gcrane`** (crane's Google-auth sibling:
  identical `cp`/`manifest`/`tag` engine, same go-containerregistry repo) from
  `gcr.io/go-containerregistry/gcrane:debug` (the `:debug` variant carries a
  busybox shell for the bash orchestration). gcrane authenticates via
  `google.Keychain`, which matches `*.pkg.dev` (our GAR) and draws credentials
  from Application Default Credentials → the GCE metadata server — so on Cloud
  Build it auths to GAR ambiently as the Mason SA with **no `crane auth login`, no
  in-memory token-fetch, no credential-helper image**. Plain `crane` uses only the
  docker-config keychain and would need the explicit-login + token dance; the Cloud
  SDK image does not bundle crane at all. This generalizes conclave's existing
  ambient model (`gcr.io/cloud-builders/docker`, no explicit login) from docker to
  crane. Full evidence in §9.
- **skopeo stays ruled out — but the recorded rationale was half-wrong.** The cinch
  ruled it out for *silently skipping foreign/non-distributable layers* (issue 545).
  That failure mode **does not apply** to this content: the disk blobs are
  `application/zstd`, a **distributable** media type, not `nondistributable`.
  skopeo's *actual* disqualifier is that its cooked operations **fatal** on the
  empty-config OCI artifact: `unsupported image-specific operation on artifact with
  type "application/vnd.oci.empty.v1+json"`. So skopeo fails **loud, not silent** —
  still wrong for the job, but for the issue-1608 (strict artifact handling) family,
  not the issue-545 (silent skip) family. Update the spec rationale accordingly.
- **Capture by *leaf digest*, never by copying the whole tag/index.** The upstream
  tag is a multi-arch OCI index that **mixes** real container images and disk
  artifacts (native family) and spans 5–15 GB. The step must read the index, select
  specific `{disktype × arch}` leaves, and `crane cp <repo>@<leaf-digest>` each one.
- **Anti-hollow-mirror guard is a host-side curl, no image tool.** A GAR registry
  v2 blob `HEAD` returns `Content-Length`; assert it equals the manifest's declared
  layer size. Proven here against 1.13 GB / 245 MB / 200 MB / 194 MB blobs. This is
  exactly the cloud-side-curl shape the recorded grade demands.
- **Trust grade: `recorded-at-acquisition`**, confirmed appropriate. quay rotates
  podvm images out within days and publishes no durable checksum; RB attests the
  digest observed at capture (trust-on-first-acquisition).

---

## 2. Environment & tools (exact versions)

| Tool | Version | Acquisition | Linkage |
|------|---------|-------------|---------|
| crane | **v0.21.6** | `google/go-containerregistry` release, `go-containerregistry_Linux_x86_64.tar.gz` | static ELF |
| oras | **v1.3.2** | `oras-project/oras` release, `oras_1.3.2_linux_amd64.tar.gz` | static ELF |
| skopeo | **1.22.2** | `quay.io/skopeo/stable@sha256:c7d3c512612f52805023cd38351081dad7e2729fc13d14b701e47c7c8bdd6615` via docker (no official static binary exists) | container |
| docker (host) | 29.5.3 | pre-installed on cerebro | — |

All three image tools were installed into a single throwaway scratch dir
(`~/lode-podvm-scratch/bin`); skopeo as a thin docker-wrapper script. Teardown
removes the scratch dir and `docker rmi quay.io/skopeo/stable`.

**Credential note:** the experiment used the **Director** identity `canest-dir`
(`director-canest-dir@cancbhm-d-canest3bhm100001.iam.gserviceaccount.com`). The
Director RBRA already resided on cerebro from prior work. A self-contained
throwaway token-mint (`mint-token.sh`, SA JWT-bearer flow via openssl+curl) issued
the GAR OAuth token, independent of the project kindle — this is also a useful
finding (see §6). Post-pace credential hygiene (payor reauth + governor remantle)
is the operator's, per the cinch.

---

## 3. Upstream characterization (the core finding)

Anchor version **podman 5.6** (latest version common to both families).

### 3.1 Provenance anchors (quay index digests at 2026-06-08)

| Family | Tag | Index digest |
|--------|-----|--------------|
| native `quay.io/podman/machine-os` | `5.6` | `sha256:6aa8bfeb41a8ce76b52277f5a609e5adeb097725ca4c3591c8ce2b4f4446dd47` |
| wsl `quay.io/podman/machine-os-wsl` | `5.6` | `sha256:61bb25d36420d8eedf240cbb12deee90b6ff6e31e2d5aba224f537f8bcafdb05` |

(quay rotates podvm; these index digests are valid as observed on 2026-06-08 and
may age out within days — the reason the trust grade is *recorded*.)

### 3.2 Tag landscape — the families diverge

- **native** carries `5.0 … 6.0` + `next`. Versions `5.0–5.3` also publish per-arch
  tags (`5.x-amd64`, `5.x-arm64`); **at 5.4 the per-arch tags were dropped** — a
  single multi-arch index per version since.
- **wsl** carries `5.3–5.6` only. No per-arch tags. **Lags the native family.**
- **Consequence:** a capture pipeline must pin **per-family** version + scheme;
  it cannot assume version parity or a stable tag convention across families.

### 3.3 Manifest structure

Both top-level tags are `application/vnd.oci.image.index.v1+json` (schemaVersion 2,
no top-level `artifactType`).

- native:5.6 → **10** child manifests, **two heterogeneous axes**:
  - 2 **plain** children `{arm64, amd64}`, no disktype → **real container OS images**
    (config `oci.image.config.v1+json`, ~65 normal `tar+gzip` layers).
  - 8 **disktype** children `{aarch64, x86_64} × {applehv, hyperv, qemu, wsl}`,
    disktype carried in `annotations["disktype"]`, arch in the alt spelling
    (`aarch64`/`x86_64`, vs `arm64`/`amd64` on the plain children).
- wsl:5.6 → **2** child manifests, `wsl` disktype × `{x86_64, aarch64}`.
- The native family's wsl-disktype leaves are **distinct digests** from the
  standalone wsl family's — one family does not characterize the other.

### 3.4 Leaf shapes

| Leaf class | mediaType | config | layers |
|---|---|---|---|
| **disktype** (qemu/wsl/applehv/hyperv) | `oci.image.manifest.v1+json` | `oci.empty.v1+json` (2 bytes) | **single** `application/zstd` blob; filename in `org.opencontainers.image.title` |
| **plain** (amd64/arm64) | oci manifest | `oci.image.config.v1+json` | ~65 `tar+gzip` layers (genuine container image) |

- The disk artifacts are **OCI artifacts** (empty-config sentinel + one arbitrary
  compressed blob), **not** container images — the modern `oras push`-style shape.
- Blobs are `application/zstd` = **distributable**. No foreign/non-distributable
  markers anywhere. (This is what invalidates the issue-545 silent-skip concern.)
- **Annotation caveat:** the layer `image.title` is unreliable for version/arch —
  the 5.6 wsl-family x86_64 leaf is titled `5.0-rootfs-amd64.tar.zst` (stale
  version *and* alt arch spelling). **Selection must key on the index descriptor's
  `platform` + `annotations.disktype`, never the layer filename.**

---

## 4. Tool verdict (empirical)

| Operation | crane 0.21.6 | oras 1.3.2 | skopeo 1.22.2 |
|---|---|---|---|
| read index | ✅ `crane manifest` | ✅ `oras manifest fetch` | ✅ `inspect --raw` |
| read artifact leaf | ✅ | ✅ (sees descriptor, 474 B) | ✅ raw only |
| **cooked inspect of artifact** | n/a | n/a | ❌ **FATAL** empty-config artifact |
| **copy artifact → GAR (digest-faithful)** | ✅ (qemu 1.13 GB, exact digest) | ✅ (245 MB, exact digest) | not attempted (ruled out) |

`crane cp` and `oras cp` both produced a GAR copy whose manifest digest is
**byte-identical** to the upstream leaf digest. crane reported `pushed blob` for the
zstd layer and reused the well-known empty-config blob
(`sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a`).

---

## 5. What was captured (and proven, then banished)

Two Lodes in the depot GAR (`rbi_ld`, repo
`cancbhm-d-canest3bhm100001/cancbhm-canest3bhm100001-gar`, us-central1):

**`rbi_ld/vn260608213343`** (kind podvm-native, recorded grade):

| member tag | tool | upstream leaf digest | size |
|---|---|---|---|
| `:rbi_qemu-x86_64` | crane cp | `sha256:a754dbe92b9171538872419734cb6b406eeb15928829f461718bf6e438bd910c` | 1,130,075,172 |
| `:rbi_wsl-x86_64` | oras cp | `sha256:8d03454f33c9a21079c4f3d4b1b046abc336c3e2448eb0f3563f1e3faf9f7db7` | 244,634,175 |
| `:rbi_vouch` | docker FROM-scratch | (provenance envelope) | — |

**`rbi_ld/vw260608213906`** (kind podvm-wsl, recorded grade):

| member tag | tool | upstream leaf digest | size |
|---|---|---|---|
| `:rbi_wsl-x86_64` | crane cp | `sha256:3d6f30202266fc9d6a2e2129bce3702dd489686075f24a4b2694221a8a9ae1d1` | 199,835,738 |
| `:rbi_wsl-aarch64` | oras cp | `sha256:0131613d9fdc688eb6688dedc394589fb99bbaa21236d87e78cf1fbf86a0b0dc` | 193,808,928 |
| `:rbi_vouch` | docker FROM-scratch | (provenance envelope) | — |

Control proofs (all host-side, no image tool):
- **divine** (`rbw-ld`) enumerated both as `(cohort: N members)`. divine is
  **kind-agnostic for enumeration** — the absent `vn`/`vw` legend lines are
  cosmetic (the follow-up pace registers them); the Lodes list correctly anyway.
- **anti-hollow-mirror guard**: GAR registry-v2 blob `HEAD` returned HTTP 200 with
  `Content-Length` exactly equal to each manifest's declared layer size, for all
  four members. No hollow mirror.
- **banish** (`rbw-lB`) deleted a Lode (the stray, see §6) in a single
  `packages delete`; divine then showed it absent. The two evidence Lodes were
  banished at teardown (§7).

The `:rbi_vouch` envelope used the existing `rbld-vouch-1` schema
(schema/kind/lode/acquired_at/acquired_by/capture_build/trust_grade/signature/
members[]), with `trust_grade:"recorded-at-acquisition"` and each member carrying
`{name, origin, digest, verification:"recorded", tags[]}`. `members[]` is the
cardinality axis (length 2 here), identical in shape to bole's length-1.

---

## 6. Process notes / spooks

- **Credential collapse at mount.** Both Governor (`account not found`) and
  Director (`Invalid JWT Signature`) probes failed. Root cause: the Governor SA had
  been torn down; the Director key was stale downstream. Repair chain:
  `rbw-aM` (payor mantles governor) → `rbw-adr` (roster to confirm identity
  `canest-dir`) → `rbw-adI canest-dir` (governor invests director) → re-probe green.
  The probe retry loop **mislabels both as "SA propagation race"** even when the
  failure is terminal (account-not-found / invalid-signature persist all 10
  attempts, 104 s) — a 104 s wait for a condition that will never clear. Worth a
  spook: distinguish terminal IAM errors from genuine propagation races and
  fail-fast on the former.
- **Empty-stamp capture bug (mine).** A `VN_STAMP` read via `$(grep …)` under
  `set -u` tripped a harness shell-snapshot quirk (`ZSH_VERSION: unbound variable`)
  and returned empty, so an `oras cp` landed in package `rbi_ld/vn` (no stamp)
  instead of `rbi_ld/vn260608213343`. Caught immediately by the post-push digest
  check; the stray became the banish test subject. Lesson for the cloud step: the
  stamp must be computed once and passed as an explicit substitution, never re-read
  mid-pipeline.
- **Self-contained token mint.** `rbgo_get_token_capture` needs the full project
  kindle. For an off-station experiment, a 20-line openssl+curl SA-JWT-bearer mint
  reading only the RBRA fields was simpler and is reusable for any
  RBRA-on-a-foreign-host situation. Not a production path (production stays
  cloud-side), but a handy diagnostic seam.
- **skopeo via docker, not static.** No official static skopeo binary exists; the
  authoritative throwaway is the `quay.io/skopeo/stable` image. Cleaner than a
  third-party static rebuild of unknown provenance.

---

## 7. The follow-up cloud pace (what this unblocks)

Build the production podvm capture **cloud-side**, riding the spine exactly like
underpin/conclave (do NOT touch the workstation to acquire bytes):

1. **`rbgjl05-immure-capture.sh`** (new in-pool step, builder =
   `gcr.io/go-containerregistry/gcrane:debug`, ambient GAR auth via the Google
   keychain — see §9; entrypoint the busybox shell). It must:
   - read the family index at the nameplate-pinned podman version,
   - select the curated `{disktype × arch}` leaf set from the index **by descriptor
     platform+disktype** (not by layer filename),
   - `crane cp <family>@<leaf-digest> <GAR>/rbi_ld/<vn|vw><stamp>:rbi_<variant>`
     per selected leaf (by-digest, loud-fail),
   - run the **blob-residency guard** (registry-v2 `HEAD`, Content-Length ==
     declared size) before vouching,
   - then hand off to the existing **`rbgjl02`** vouch step unchanged.
2. **`rbldv_*.sh`** body — podvm is **opaque-blob × multi-member**, a blend of
   `rbldw_Underpin` (opaque-blob) and `rbldr_Reliquary` (multi-member cohort).
   It composes a spine recipe (capture step + vouch step) plus a substitutions blob
   carrying family, version, and the declarative selected-platform list. **One verb
   `immure` (`rbw-lI`)** spans both quay families via a family/archive argument
   (per the paddock Vocabulary), not two verbs.
3. **Registration surface** (the 12f48cbab spook — don't miss the single-fixture
   registry): `rbgc_Constants.sh` brands/tags (kind-letters `vn`/`vw` already
   reserved), `rbldl_Lifecycle.sh` divine legend (add vn/vw lines), and the theurge
   fixture registry across `rbtdrc_crucible.rs` + `rbtdrm_manifest.rs` **including
   `RBTDRC_FIXTURES`**.
4. **Theurge fixture** `podvm-lifecycle` (service tier): immure → divine
   (enumerate + cohort count) → inspect members + `:rbi_vouch` → per-member +
   whole-Lode banish → absent. Add to service + complete suites.
5. **Declarative intent (no FQIN).** The nameplate/host-tier config declares family
   + version + desired `{disktype, arch}` set; `rbgjl05` resolves leaf digests from
   the live index at capture time. Support a "widen selected set against the same
   upstream version" refresh mode, separate from "bump podman version" — quay's
   hostile retention can make later expansion impossible once a version ages out.

---

## 8. Premises confirmed / nuanced

- **Cloud-side acquisition premise STANDS.** cerebro was a one-time lab bench; the
  experiment did not tempt a workstation-hosted production path.
- **podvm-selective retention STANDS** — captured 2 curated leaves per family, not
  the 5–15 GB full index.
- **recorded grade STANDS** and is correctly the *only* honest grade for podvm.
- **skopeo-out STANDS, rationale corrected** (artifact-rejection, not silent-skip;
  see §1). RBSPV / the Lode spec should be updated to state the real failure mode.
- **crane lean CONFIRMED**, with oras documented as an equal-fidelity fallback.

---

## 9. Cloud-side auth — gcrane ambient Google keychain (2026-06-08, source-read follow-up)

Settled after the cerebro teardown by reading go-containerregistry source, not by
re-running the bench. Resolves the auth axis the bole-eviction pace cinched as
"decided here, inherited downstream" — the experiment used an explicit off-station
token mint (§2 credential note, §6 spook) and never characterized the *cloud*
auth path. This section is that path.

**The binary distinction is load-bearing — `crane` and `gcrane` authenticate differently:**

- **`crane`** authenticates only via `authn.DefaultKeychain` (the docker config
  file). On a Cloud Build worker that config is empty, so plain crane needs an
  explicit `crane auth login` fed by an in-memory metadata-server token — the
  skopeo token-fetch dance, ported, not removed.
- **`gcrane`** is the Google variant: same `cp`/`manifest`/`tag`/`digest` engine,
  but it authenticates through `google.Keychain`. Per `pkg/v1/google/keychain.go`,
  that keychain (a) matches `gcr.io`, `*.gcr.io`, **`*.pkg.dev`** (Artifact
  Registry — our GAR host), and `*.google.com`, and (b) draws credentials from
  Application Default Credentials → the **GCE metadata server** first, gcloud
  second, anonymous last. On a Cloud Build worker the Mason SA *is* the
  metadata-server identity, so gcrane auths to GAR ambiently with zero auth code.
- **The Cloud SDK / gcloud image does not contain crane** — it is gcloud+gsutil+bq
  on Debian/Alpine. "A Google image that has gcloud and presumably crane" is a
  false premise; that path means installing crane into a fat SDK image yourself.

**Decision.** The bole capture step — and every later crane-embrace capture step —
uses **`gcrane`** from **`gcr.io/go-containerregistry/gcrane:debug`**. The `:debug`
variant carries `/busybox/sh` (the bash orchestration) and busybox `sha256sum`
(the fingerprint); the non-debug image is distroless — no shell, unusable as a
script step. Pin by digest in production, not the floating `:debug` tag. The
in-memory `token-fetch` snippet is **dropped** from the bole path; it survives only
where a non-keychain tool still needs it, and retires entirely as skopeo and docker
are evicted.

**This is conclave's model generalized, not a new one.** The reliquary capture step
(rbgjl03, RBSLC) already auths ambiently — `gcr.io/cloud-builders/docker`, "no
explicit login," cred-helper via the Mason SA. gcrane extends that same ambient
posture from docker to crane, so the whole capture family shares one auth story
instead of two. The durable canon for this lives in **RBSCB** (Cloud Build
posture), which today still enshrines the skopeo "cannot use the credential helper,
fetch the token in-memory" rationale that `token-fetch.sh` cites — that line is
superseded by this finding and updated as the eviction lands.

**Sources:**
- go-containerregistry README — crane / gcrane / krane variants and their keychains
- `pkg/v1/google/keychain.go` — `isGoogle()` host matching (`*.pkg.dev`) + ADC/metadata credential acquisition
- crane image docs — distroless default, `:debug` busybox shell at `/busybox/sh`
- Cloud SDK Docker image contents — gcloud/gsutil/bq, no crane
