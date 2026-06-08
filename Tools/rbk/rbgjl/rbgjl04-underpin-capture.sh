#!/bin/bash
# RBGJL Step 04: Underpin a WSL rootfs into a Lode (capture) via curl + gpg + buildx
# Builder: gcr.io/cloud-builders/docker (Google-hosted, always pullable; Debian-
#          based — carries curl + docker buildx, and gnupg is apt-installed by the
#          gpg-verify-sums snippet if absent. The underpin acquisition tool is curl
#          over HTTPS, not a registry pull, so this kind needs neither skopeo nor a
#          reliquary bootstrap. The docker credential helper authenticates the GAR
#          push ambiently via the Mason SA.)
# Substitutions: _RBGL_GAR_HOST, _RBGL_GAR_PATH, _RBGL_LODES_ROOT, _RBGL_LODE_STAMP,
#                _RBGL_TAG_ROOTFS, _RBGL_TRUST_GRADE, _RBGL_VOUCH_SCHEMA,
#                _RBGL_ACQUIRED_BY, _RBGL_WSL_URL, _RBGL_WSL_KEY_FPR
#
# Fetch the vendor rootfs tarball (URL assembled host-side from the version args)
# over HTTPS, DISCOVER its checksum from Canonical's published, GPG-signed
# SHA256SUMS (verified against the pinned signing-key fingerprint — the
# verified-against-published gate), verify the rootfs bytes against it, then wrap
# the OPAQUE tarball as a single-layer OCI member (FROM scratch + COPY — never ADD,
# which would extract) and push it into ONE GAR package rbi_ld/<stamp> under the
# clean member tag :rbi_rootfs. Author the batch provenance envelope (members[]
# length 1 — the cardinality axis) and stage it for step 02 (the :rbi_vouch
# artifact) and for the host capture-file via /builder/outputs/output.
# Single-platform (linux/amd64): the rootfs is an opaque blob, not a multi-arch
# manifest. No digest is pinned here — the published checksum is discovered and
# signature-verified at capture, per the no-FQIN premise (RBSLU).
#
# Package shape:  <host>/<path>/<LODES_ROOT>/<stamp>     (one package = one Lode)
# Member tag on that package (clean scheme — no digest/fingerprint layer):
#   :<TAG_ROOTFS>   e.g. rbi_rootfs   (the opaque rootfs blob)
# The :rbi_vouch tag is a separate manifest pushed by step 02.
#
# Trust grade verified-against-published: the member digest recorded in the
# envelope is the tarball's GPG-verified published SHA-256, tagged verification
# "gpg-sha256-published" with the signing-key fingerprint — NOT an OCI manifest
# digest, because what is attested is capture-fidelity of an opaque blob under the
# vendor's signed checksum.

set -euo pipefail
echo "=== Underpin WSL rootfs into a Lode ==="

STAMP="${_RBGL_LODE_STAMP}"
test -n "${STAMP}" || { echo "FATAL: _RBGL_LODE_STAMP missing" >&2; exit 1; }

URL="${_RBGL_WSL_URL}"
KEY_FPR="${_RBGL_WSL_KEY_FPR}"
test -n "${URL}"     || { echo "FATAL: _RBGL_WSL_URL missing" >&2; exit 1; }
test -n "${KEY_FPR}" || { echo "FATAL: _RBGL_WSL_KEY_FPR missing" >&2; exit 1; }

PKG="${_RBGL_GAR_HOST}/${_RBGL_GAR_PATH}/${_RBGL_LODES_ROOT}/${STAMP}"
MEMBER_TAG="${_RBGL_TAG_ROOTFS}"
DEST="${PKG}:${MEMBER_TAG}"
echo "Lode package: ${PKG}"
echo "Source:       ${URL}"

# Acquisition moment, attested for the single-member cohort.
ACQUIRED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)

# --- Discover the published checksum under the pinned signing key ---
# The sums + signature live beside the tarball; the target filename is its
# basename. The snippet GPG-verifies the sums and provides EXPECTED_SHA.
SUMS_URL="${URL%/*}/SHA256SUMS"
SIG_URL="${SUMS_URL}.gpg"
TARGET_BASENAME="${URL##*/}"
#@rbgjs_include gpg-verify-sums

# --- Fetch the rootfs over HTTPS (cloud-side; the workstation never touches these
# bytes — only the URL was assembled host-side) ---
ROOTFS="/workspace/underpin_${STAMP}.rootfs.tar"
echo "--- Fetching rootfs tarball ---"
curl -fSL --retry 3 -o "${ROOTFS}" "${URL}" \
  || { echo "FATAL: Failed to fetch rootfs from ${URL}" >&2; exit 1; }

# --- Verify the rootfs bytes against the GPG-verified published checksum ---
# Tool availability on the docker builder is a Palisade: sha256sum (coreutils) is
# the expected path; shasum / openssl are honest fallbacks so a missing tool fails
# the compare loud rather than skipping verification.
echo "--- Verifying rootfs against published checksum ---"
if command -v sha256sum >/dev/null 2>&1; then
  ACTUAL_SHA=$(sha256sum "${ROOTFS}" | cut -d' ' -f1)
elif command -v shasum >/dev/null 2>&1; then
  ACTUAL_SHA=$(shasum -a 256 "${ROOTFS}" | cut -d' ' -f1)
elif command -v openssl >/dev/null 2>&1; then
  ACTUAL_SHA=$(openssl dgst -sha256 "${ROOTFS}" | awk '{print $NF}')
else
  echo "FATAL: no sha256 tool (sha256sum/shasum/openssl) on builder — cannot verify" >&2
  exit 1
fi
test -n "${ACTUAL_SHA}" || { echo "FATAL: empty digest computed for rootfs" >&2; exit 1; }

if [ "${ACTUAL_SHA}" != "${EXPECTED_SHA}" ]; then
  echo "FATAL: rootfs checksum mismatch — refusing to capture unverified bytes" >&2
  echo "  published (GPG-verified) sha256: ${EXPECTED_SHA}" >&2
  echo "  fetched   sha256:                ${ACTUAL_SHA}" >&2
  exit 1
fi
echo "Verified: sha256:${ACTUAL_SHA} (matches GPG-signed published checksum)"

# --- Wrap the OPAQUE tarball as a single-layer OCI member and push ---
# FROM scratch + COPY (never ADD): COPY stores the tarball byte-for-byte; ADD would
# auto-extract and destroy the opaque blob. The shared buildx builder pushes it.
echo "--- Wrapping rootfs as OCI member -> ${DEST} ---"
CTX="/workspace/underpin_ctx_${STAMP}"
mkdir -p "${CTX}"
cp "${ROOTFS}" "${CTX}/rootfs.tar"
echo "FROM scratch"      >  "${CTX}/Dockerfile"
echo "COPY rootfs.tar /" >> "${CTX}/Dockerfile"

# Ensure the shared buildx builder — shared library snippet (run once).
#@rbgjs_include buildx-bootstrap

# Push the FROM-scratch opaque-blob context — shared library snippet.
PUSH_URI="${DEST}"
PUSH_PLATFORMS="linux/amd64"
PUSH_CTX="${CTX}"
#@rbgjs_include buildx-push

echo "Rootfs captured: ${DEST} (sha256:${ACTUAL_SHA})"

# --- Author the batch provenance envelope (identical content lands in :rbi_vouch
# and the host capture-file). members[] length 1 — the wsl singleton. No jq
# dependency — values are controlled (member tag, URL, hex digest, key fingerprint,
# SA email, build id, ISO timestamp); none can carry a literal quote. ---
ENVELOPE='{'
ENVELOPE="${ENVELOPE}\"schema\":\"${_RBGL_VOUCH_SCHEMA}\","
ENVELOPE="${ENVELOPE}\"kind\":\"wsl\","
ENVELOPE="${ENVELOPE}\"lode\":\"${STAMP}\","
ENVELOPE="${ENVELOPE}\"acquired_at\":\"${ACQUIRED_AT}\","
ENVELOPE="${ENVELOPE}\"acquired_by\":\"${_RBGL_ACQUIRED_BY}\","
ENVELOPE="${ENVELOPE}\"capture_build\":\"${BUILD_ID:-}\","
ENVELOPE="${ENVELOPE}\"trust_grade\":\"${_RBGL_TRUST_GRADE}\","
ENVELOPE="${ENVELOPE}\"signature\":null,"
ENVELOPE="${ENVELOPE}\"members\":[{"
ENVELOPE="${ENVELOPE}\"name\":\"${MEMBER_TAG}\","
ENVELOPE="${ENVELOPE}\"origin\":\"${URL}\","
ENVELOPE="${ENVELOPE}\"digest\":\"sha256:${ACTUAL_SHA}\","
ENVELOPE="${ENVELOPE}\"verification\":\"gpg-sha256-published\","
ENVELOPE="${ENVELOPE}\"signing_key\":\"${KEY_FPR}\","
ENVELOPE="${ENVELOPE}\"tags\":[\"${MEMBER_TAG}\"]"
ENVELOPE="${ENVELOPE}}]}"

# Stage the envelope for step 02 (pushes it as the :rbi_vouch artifact). The
# stamps file is the step-02 contract; underpin produces exactly one Lode.
printf '%s' "${ENVELOPE}" > "/workspace/lode_${STAMP}_vouch.json"
: > /workspace/lode_stamps.txt
echo "${STAMP}" >> /workspace/lode_stamps.txt

# Host-facing result (the capture-file carries the same envelope). One slot —
# underpin produces exactly one Lode.
RESULT="{\"slot_1\":{\"stamp\":\"${STAMP}\",\"vouch\":${ENVELOPE}}}"

echo "=== Writing capture results ==="
echo "${RESULT}"

# Write to buildStepOutputs channel (host extracts the touchmark -> capture-file).
mkdir -p /builder/outputs
printf '%s' "${RESULT}" > /builder/outputs/output

echo "=== Underpin capture step complete ==="
