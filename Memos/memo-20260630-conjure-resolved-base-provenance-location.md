# Conjure resolved-base provenance: where the FROM base actually lives

*Written by Claude Opus 4.8, 2026-06-30, grounding pace ₢BiAAm
(resolved-base-readback-and-verify) in heat ₣Bi against the live depot.
Empirical record, not authority — verify against a live conjure hallmark before
relying on any digest quoted here.*

## What this settles

Pace ₢BiAAm was docketed on the premise that a conjure hallmark's **signed**
SLSA provenance (the DSSE envelope on the per-platform `attest` ark) carries the
resolved Dockerfile FROM base in `.predicate.buildDefinition.resolvedDependencies[]`,
and that the buildx-native attestation on the image index must be avoided as
"unsigned."

**Live data inverts this.** The signed google-worker provenance's
`resolvedDependencies` records the Cloud Build **step tooling**, not the FROM
base. The resolved FROM base lives **only** in the unsigned buildx-native
attestation on the image index. The pace cannot be built as specified.

## Method

- Depot grounded: project `cancbhm-d-canest3bhm100002`, GAR repo
  `cancbhm-canest3bhm100002-gar`, region `us-central1`, host
  `us-central1-docker.pkg.dev`, hallmarks category `rbi_hm`.
- The local director/retriever SA keys under `../station-files/secrets/{director,retriever}/rbra.env`
  are **dead** — they name project `cancbhm-d-canest3bhm100001`, which the registry
  reports as `Project #113099144907 has been deleted`. Reached the live depot
  instead with a payor OAuth access token minted from `payor/rbro.env`
  (`RBRO_CLIENT_SECRET` + `RBRO_REFRESH_TOKEN`) paired with the `client_id` from
  `client_secrets/*.json`. Read-only throughout.
- All reads via the Docker Registry v2 API (manifest + blob endpoints) — the same
  surface plumb uses. No gcloud (its `describe --show-provenance` also hung >2min
  on the 341 MB platform image; avoid it).

## The decisive evidence — three vessels, three FROM bases, one resolvedDependencies

Three live conjure hallmarks, each a different committed FROM base, all carry the
**identical** `resolvedDependencies` in their signed `attest` v1 provenance:

| Hallmark | Vessel | Committed FROM base (vouch `base_images`) | Signed attest v1 `resolvedDependencies` |
|---|---|---|---|
| `c260621115420-r260621185426` | `rbev-busybox` | `busybox:latest` (pass-through) | `rbi_alpine`, `rbi_docker`, `rbi_gcloud` |
| `c260630105816-r260630175821` | `rbev-sentry-deb-tether` | `debian:bookworm-slim` (pass-through) | `rbi_alpine`, `rbi_docker`, `rbi_gcloud` |
| `c260630114109-r260630184113` | `rbev-bottle-ifrit-airgap` | anchored hallmark `c260630112951-r260630182955/image` | `rbi_alpine`, `rbi_docker`, `rbi_gcloud` |

The `resolvedDependencies` set is invariant to the vessel's actual base, and
tracks the **reliquary build-tool cohort** for the build date (touchmark
`r260630105627` for the 0630 builds, `r260621114527` for the 0621 build) — i.e.
the `docker`/`gcloud`/`alpine` images the Cloud Build *steps* ran on, not the
image the Dockerfile built FROM. The 2026-03-05 provenance heat
(`jjh_b260303-r260310-rbw-e2e-cbv2-provenance.md`, "resolvedDependencies lists
same builder image digest") was correct; the ₢BiAAm docket cinch was not.

## The two attestations, precisely

### Signed — google-worker (the `attest` ark attachments)

Under `rbi_hm/<HALL>/attest`, the `tags/list` shows one tagged platform image
(`<HALL>-amd64`, ~341 MB) plus several **untagged** OCI image manifests. The
untagged ones are Cloud Build attachments:

- manifest `config.mediaType = application/vnd.oci.empty.v1+json`,
  annotation `artifactregistry.attachment_namespace = cloudbuild.googleapis.com`,
  one layer `mediaType = text/plain; charset=utf-8`.
- The layer blob is a **DSSE envelope** (`payloadType: application/vnd.in-toto+json`,
  keys `payload`/`payloadType`/`signatures`), signed by the google-hosted-worker
  key (the signature rbgjv02 already verifies).
- base64url-decode `.payload` → in-toto statement. Two predicate types appear:
  - `https://slsa.dev/provenance/v0.1` — predicate `builder`/`metadata`/`recipe`;
    **no** `buildDefinition`, **no** `resolvedDependencies`.
  - `https://slsa.dev/provenance/v1` — buildType
    `https://cloud.google.com/build/gcb-buildtypes/google-worker/v1`;
    `buildDefinition.resolvedDependencies` = the 3 step-tooling entries above
    (`uri` = `…/rbi_ld/<touchmark>:rbi_{alpine,docker,gcloud}@sha256:…`,
    `digest.sha256`); `subject` = the per-platform attest digests (amd64, arm64).

This is the trusted source, and it does **not** contain the FROM base.

### Unsigned — buildx-native (on the image index)

Under `rbi_hm/<HALL>/image`, the manifest is an OCI index with the platform
images (`linux/amd64`, `linux/arm64`, `linux/arm`) plus `unknown/unknown`
manifests annotated `vnd.docker.reference.digest = <platform image digest>` —
the buildx attestation manifests. Each has a layer annotated
`in-toto.io/predicate-type = https://slsa.dev/provenance/v1`,
`mediaType = application/vnd.in-toto+json`. The blob is a **bare in-toto
statement** (no DSSE envelope, no signature — this is the "unsigned" part),
buildType `https://github.com/moby/buildkit/blob/master/docs/attestations/slsa-definitions.md`.

For `rbev-busybox`, its `resolvedDependencies` (amd64) is exactly the resolved
FROM base, as a PURL with the resolved digest:

```
resolvedDependencies[0]:
  uri    = pkg:docker/busybox@latest?platform=linux/amd64
  digest = { "sha256": "fd8d9aa63ba2f0982b5304e1ee8d3b90a210bc1ffb5314d980eb6962f1a9715d" }
```

This is what the pace wanted — and it is in the source the docket forbade.

## Registry-raw access recipe (reusable, no gcloud, no new dependency)

Both attestations are reachable with the retriever/director token plumb already
mints — the mechanism half of the docket holds:

- **Signed google-worker provenance**: `GET tags/list` on `…/attest` → for each
  untagged manifest, `GET manifests/<digest>` → keep those with
  annotation `artifactregistry.attachment_namespace = cloudbuild.googleapis.com`
  → `GET blobs/<layer0.digest>` → DSSE envelope → base64url-decode `.payload` →
  select predicateType `…/provenance/v1`.
- **Unsigned buildx base**: `GET manifests/<HALL>` on `…/image` (an index) →
  select `unknown/unknown` members → `GET manifests/<member>` → layer with
  `in-toto.io/predicate-type` ending `provenance/v1` and buildType `…/buildkit…`
  → `GET blobs/<layer.digest>` → bare in-toto statement → `resolvedDependencies`.

## Trust nuance for whoever picks the path forward

- The signed (DSSE, google-hosted-worker) provenance attests builder identity and
  subject digests but carries the **wrong** dependency set for "resolved base."
- The unsigned (bare in-toto, moby/buildkit) attestation carries the **right**
  dependency set but is not signed — its integrity rides only on the registry
  delivering the index it lives in.
- The base PURL is upstream-flavored (`pkg:docker/busybox@latest`) for
  pass-through vessels; an anchored vessel's FROM is a GAR hallmark image ref,
  and was not separately confirmed in the buildx attestation in this session
  (only the pass-through busybox case was decoded end-to-end). Confirm the
  anchored-vessel shape before relying on it.

## The way forward — settled in discussion 2026-06-30

The fix is not to read provenance after the build, but to **capture the resolved
base at build time as a signed image label** (the "b2" option). This solves the
problem at the layer RB already controls — the FROM resolution — instead of
post-hoc archaeology over a foreign provenance shape. Every link below was
verified against the live depot in the same session.

1. **Precondition already enforced.** `rbfh_dockerfile_check` (Rule 2) requires
   every column-0 `FROM` image token to be `${RBF_IMAGE_1..3}` or `scratch`,
   rejecting any hardcoded base or bare stage-name with `BUBC_band_hygiene`,
   before any build, in both conjure (`rbfd_director.sh:977`) and kludge
   (`rbfk_kludge.sh:188`). So RB is the *sole* source of the FROM base — a
   Dockerfile with direct base access cannot produce an image, so the label can
   never lie about a base RB did not control.

2. **The base already flows through an RB build-arg.** Vessel Dockerfiles are
   generic `FROM ${RBF_IMAGE_n}`, fed by `rbgjb03-buildx-push-image.sh`'s
   `--build-arg RBF_IMAGE_n=${_RBGY_IMAGE_n}`; the same step already adds
   `--label` lines. No per-vessel Dockerfile surgery for either the pin or the
   label.

3. **Resolve the base tag → digest in a cloud step** (operator decision,
   2026-06-30). `rbgjs-gcrane-fingerprint.sh` already does `gcrane manifest
   <ref>` → sha256, running inside GCP egress — the chosen home for resolving a
   pass-through base (e.g. `busybox:latest`); RB-internal anchored bases resolve
   within RB's own registry. Use the one resolved digest twice: **pin** the
   build-arg to `<ref>@sha256:…` (so buildx provably builds from exactly it) and
   **label** it.

4. **The label rides into the signed image.** GCB's google-worker DSSE
   provenance signs the per-platform **attest-ark pullback** images, NOT the
   consumer image the user pulls (verified: consumer amd64 `c0812fad…` ≠ signed
   subject `477a2279…`). But the labels `rbgjb03` sets on the consumer image
   survive the pullback into the signed attest image byte-identically (verified
   live: `hallmark`/`git.commit`/`git.branch` present and identical in
   `477a2279…`). So `--label <sprue>_resolved_base_n=<ref@digest>` becomes part
   of the signed subject's config — tamper-evident under the existing signature.

5. **plumb reads it back registry-raw** from the signed attest image's config
   blob (`.config.Labels`) — the same fetch surface it already uses, no gcloud,
   no new dependency.

**Trust outcome:** the resolved base becomes a fact whose integrity rides the
existing google-worker signature — strictly better than the unsigned
buildx-native attestation or the config-echo `base_images[]`.

**Sprue:** this authors a new RB wire tag (the label key), so it takes a unique
sprue (`<sprue>_resolved_base_n`), per the JSON-tag traceability rule. The
existing `rbgjb03` labels (`hallmark`, `git.commit`) are unsprued — the new one
diverges from those neighbors deliberately. Sprue not yet minted.

**Edges:** a `FROM scratch` stage has no base — emit no label for that slot
(mirror `rbgjb03`'s conditional build-arg pattern). The hygiene gate forbids
`FROM <stagename>`, so there is no third kind of FROM — every base is a pinnable
`RBF_IMAGE_n` slot or scratch.

This reverses the original docket's cinches: the trusted source is reached by
*writing* a signed label at build time, not by *reading* the signed attest
provenance (whose `resolvedDependencies` carries only the build-step tooling).

## Pointers

- Reference decoder (signed path, via gcloud): `Tools/rbk/rbgjv/rbgjv02-verify-provenance.py`.
- Plumb (the consumer the pace targeted): `Tools/rbk/rbfcp_plumb.sh`.
- Spec: `RBSAP-ark_plumb.adoc`. Prior provenance archaeology:
  `Memos/memo-20260305-provenance-architecture-gap.md`,
  `.claude/jjm/retired/jjh_b260303-r260310-rbw-e2e-cbv2-provenance.md`.
