#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# fdkyclk-teardown.sh — POC teardown, the inverse of fdkyclk-proof.sh (₢BfAAH).
#
# Idempotent: tolerant of already-removed state, so it converges whether run
# after a full proof, a partial one, or twice. Run from the repository root:
#     bash rbmm_moorings/fdkyclk/fdkyclk-teardown.sh
#
# Federation-vocabulary mapping for the BCG conversion (gcloud here is throwaway
# POC scaffolding; the durable transform is REST + the payor OAuth token, NO
# direct gcloud):
#   - delete workforce pool   == JILT     (rbgp_manor_jilt, against the programmatic foedus)
#   - remove the brevet IAM   == ATTAINT  (rbgp_attaint / rbgp_unseat of the citizen)
#   - quench the crucible     == the existing tt/rbw-cQ.Quench.fdkyclk.sh tabtarget
# This sequence is what the establishment module's un-establish path becomes.

set -uo pipefail   # deliberately NOT -e: teardown tolerates already-gone state

KC_POOL=fdkyclk-test
DEPOT_PROJECT=cancbhm-d-canest3bhm100002
MANTLE_SA="rbma-director@${DEPOT_PROJECT}.iam.gserviceaccount.com"
SUBJECT=67c2d407-fbd7-466b-aafc-2aca99a17de2
PRINCIPAL="principal://iam.googleapis.com/locations/global/workforcePools/${KC_POOL}/subject/${SUBJECT}"
RBRP=rbmm_moorings/rbrp.env
RBRO=../station-files/secrets/payor/rbro.env

[ -d rbmm_moorings ] || { echo "ERROR: run from the repository root"; exit 1; }

val() { grep -E "^$1=" "$2" | head -1 | cut -d= -f2- | sed "s/^[\"']//; s/[\"']\$//"; }

payor_token() {
  curl -s https://oauth2.googleapis.com/token \
    --data-urlencode "client_id=$(val RBRP_OAUTH_CLIENT_ID "$RBRP")" \
    --data-urlencode "client_secret=$(val RBRO_CLIENT_SECRET "$RBRO")" \
    --data-urlencode "refresh_token=$(val RBRO_REFRESH_TOKEN "$RBRO")" \
    --data-urlencode "grant_type=refresh_token" | jq -r '.access_token // empty'
}

# Remove an IAM binding, reporting cleanly whether it was present or already gone.
attaint_binding() {
  local label="$1"; shift
  local out
  out=$(gcloud "$@" --condition=None 2>&1) || true
  if   printf '%s' "$out" | grep -qiE 'not found|does not have'; then echo "  ${label}: already removed"
  elif printf '%s' "$out" | grep -qiE 'etag';                     then echo "  ${label}: removed"
  else echo "  ${label}: $(printf '%s' "$out" | tail -1)"; fi
}

export CLOUDSDK_AUTH_ACCESS_TOKEN; CLOUDSDK_AUTH_ACCESS_TOKEN=$(payor_token)
[ -n "$CLOUDSDK_AUTH_ACCESS_TOKEN" ] || { echo "FATAL: no payor token"; exit 1; }

echo "=== attaint: remove the brevet IAM bindings (idempotent) ==="
attaint_binding "tokenCreator @ ${MANTLE_SA}" \
  iam service-accounts remove-iam-policy-binding "$MANTLE_SA" --project="$DEPOT_PROJECT" \
  --member="$PRINCIPAL" --role="roles/iam.serviceAccountTokenCreator"
attaint_binding "serviceUsageConsumer @ ${DEPOT_PROJECT}" \
  projects remove-iam-policy-binding "$DEPOT_PROJECT" \
  --member="$PRINCIPAL" --role="roles/serviceusage.serviceUsageConsumer"

echo "=== jilt: delete the workforce pool (cascades the provider; 30-day soft-delete) ==="
if gcloud iam workforce-pools describe "$KC_POOL" --location=global >/dev/null 2>&1; then
  gcloud iam workforce-pools delete "$KC_POOL" --location=global --quiet 2>&1 | tail -3
else
  echo "  pool ${KC_POOL} already absent"
fi

echo "=== quench: the local crucible (existing tabtarget) ==="
if [ -x tt/rbw-cQ.Quench.fdkyclk.sh ]; then
  ./tt/rbw-cQ.Quench.fdkyclk.sh
else
  echo "  run: tt/rbw-cQ.Quench.fdkyclk.sh"
fi

echo "TEARDOWN_DONE"
