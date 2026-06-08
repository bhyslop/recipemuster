#!/bin/bash
# RBGJL Step 03: Conclave the build-tool cohort into a Lode (capture) via docker
# Builder: gcr.io/cloud-builders/docker (Google-hosted, always pullable — conclave
#          captures the reliquary tools themselves, so it cannot bootstrap from a
#          reliquary; the cloud-builders/docker credential helper authenticates
#          the GAR push ambiently via the Mason SA, no explicit login)
# Substitutions: _RBGL_GAR_HOST, _RBGL_GAR_PATH, _RBGL_LODES_ROOT, _RBGL_LODE_STAMP,
#                _RBGL_TAG_SPRUE, _RBGL_TRUST_GRADE, _RBGL_VOUCH_SCHEMA,
#                _RBGL_ACQUIRED_BY
#
# Pull each build-tool image from upstream, tag it into ONE GAR package
# rbi_ld/<stamp> under the clean member tag :rbi_<tool>, and push. Author the
# batch provenance envelope (members[] one per tool — the cardinality axis) and
# stage it for step 02 (the :rbi_vouch artifact) and for the host capture-file via
# /builder/outputs/output. Single-platform (linux/amd64) — tool images run as GCB
# steps on amd64 workers, so docker pull/tag/push is sufficient.
#
# Package shape:  <host>/<path>/<LODES_ROOT>/<stamp>     (one package = one Lode)
# Member tags on that package, each a distinct tool manifest:
#   :<TAG_SPRUE><tool>   e.g. rbi_gcloud, rbi_skopeo   (clean scheme — no digest layer)
# The :rbi_vouch tag is a separate manifest pushed by step 02.

set -euo pipefail
echo "=== Conclave build-tool cohort into a Lode ==="

STAMP="${_RBGL_LODE_STAMP}"
test -n "${STAMP}" || { echo "FATAL: _RBGL_LODE_STAMP missing" >&2; exit 1; }

PKG="${_RBGL_GAR_HOST}/${_RBGL_GAR_PATH}/${_RBGL_LODES_ROOT}/${STAMP}"
echo "Lode package: ${PKG}"

# Tool image cohort: short-name|upstream-ref. Authoritative co-versioned set for
# GCB step execution — mirrors the legacy inscribe manifest (rbgji01) verbatim;
# the two coexist during the reliquary cutover. gcloud and docker are Google-
# hosted (gcr.io); the rest are third-party.
MANIFEST=(
  "gcloud|gcr.io/cloud-builders/gcloud:latest"
  "docker|gcr.io/cloud-builders/docker:latest"
  "alpine|docker.io/library/alpine:latest"
  "syft|docker.io/anchore/syft:latest"
  "binfmt|docker.io/tonistiigi/binfmt:latest"
  "skopeo|quay.io/skopeo/stable:latest"
)

# Acquisition moment, attested once for the whole cohort.
ACQUIRED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)

# Accumulate the envelope members[] as we capture (one element per tool). No jq
# dependency — values are controlled (tool name, upstream ref, hex digest, SA
# email, build id, ISO timestamp); none can carry a literal quote.
MEMBERS=''
MFIRST=true

for ENTRY in "${MANIFEST[@]}"; do
  NAME="${ENTRY%%|*}"
  UPSTREAM="${ENTRY#*|}"
  MEMBER_TAG="${_RBGL_TAG_SPRUE}${NAME}"
  DEST="${PKG}:${MEMBER_TAG}"

  echo "--- ${NAME}: ${UPSTREAM} -> ${DEST} ---"

  docker pull "${UPSTREAM}" \
    || { echo "FATAL: Failed to pull ${UPSTREAM}" >&2; exit 1; }

  # Record the pulled digest from RepoDigests (format: repo@sha256:...).
  DIGEST=$(docker inspect --format='{{index .RepoDigests 0}}' "${UPSTREAM}" 2>/dev/null) || DIGEST=""
  DIGEST="${DIGEST##*@}"

  docker tag "${UPSTREAM}" "${DEST}" \
    || { echo "FATAL: Failed to tag ${NAME}" >&2; exit 1; }
  docker push "${DEST}" \
    || { echo "FATAL: Failed to push ${NAME} to GAR" >&2; exit 1; }

  echo "${NAME} captured: ${DEST} (${DIGEST})"

  if [ "${MFIRST}" = "true" ]; then MFIRST=false; else MEMBERS="${MEMBERS},"; fi
  MEMBERS="${MEMBERS}{"
  MEMBERS="${MEMBERS}\"name\":\"${MEMBER_TAG}\","
  MEMBERS="${MEMBERS}\"origin\":\"${UPSTREAM}\","
  MEMBERS="${MEMBERS}\"digest\":\"${DIGEST}\","
  MEMBERS="${MEMBERS}\"verification\":\"oci-digest\","
  MEMBERS="${MEMBERS}\"tags\":[\"${MEMBER_TAG}\"]"
  MEMBERS="${MEMBERS}}"
done

# Author the batch provenance envelope (identical content lands in :rbi_vouch and
# the host capture-file). members[] is the cardinality axis — N for the reliquary
# cohort, where bole carries 1.
ENVELOPE='{'
ENVELOPE="${ENVELOPE}\"schema\":\"${_RBGL_VOUCH_SCHEMA}\","
ENVELOPE="${ENVELOPE}\"kind\":\"reliquary\","
ENVELOPE="${ENVELOPE}\"lode\":\"${STAMP}\","
ENVELOPE="${ENVELOPE}\"acquired_at\":\"${ACQUIRED_AT}\","
ENVELOPE="${ENVELOPE}\"acquired_by\":\"${_RBGL_ACQUIRED_BY}\","
ENVELOPE="${ENVELOPE}\"capture_build\":\"${BUILD_ID:-}\","
ENVELOPE="${ENVELOPE}\"trust_grade\":\"${_RBGL_TRUST_GRADE}\","
ENVELOPE="${ENVELOPE}\"signature\":null,"
ENVELOPE="${ENVELOPE}\"members\":[${MEMBERS}]}"

# Stage the envelope for step 02 (pushes it as the :rbi_vouch artifact). The
# stamps file is the step-02 contract; conclave produces exactly one Lode.
printf '%s' "${ENVELOPE}" > "/workspace/lode_${STAMP}_vouch.json"
: > /workspace/lode_stamps.txt
echo "${STAMP}" >> /workspace/lode_stamps.txt

# Host-facing result (the capture-file carries the same envelope). One slot —
# conclave produces exactly one Lode (the cohort is one package).
RESULT="{\"slot_1\":{\"stamp\":\"${STAMP}\",\"vouch\":${ENVELOPE}}}"

echo "=== Writing capture results ==="
echo "${RESULT}"

# Write to buildStepOutputs channel (host extracts the touchmark -> capture-file).
mkdir -p /builder/outputs
printf '%s' "${RESULT}" > /builder/outputs/output

echo "=== Conclave capture step complete ==="
