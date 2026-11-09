#!/bin/bash
# Teardown of the Keycloak federation proof driver; moved to Study as a reference artifact, no longer run and repairing no path.
#
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
#     bash rbmm_moorings/rben_fdkyclk/fdkyclk-teardown.sh
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
SUBJECT=67c2d407-fbd7-466b-aafc-2aca99a17de2
PRINCIPAL="principal://iam.googleapis.com/locations/global/workforcePools/${KC_POOL}/subject/${SUBJECT}"
RBRP=rbmm_moorings/rbrp.env
RBRO=../station-files/secrets/payor/rbro.env
RBRD=rbmm_moorings/rbrd.env

[ -d rbmm_moorings ] || { echo "ERROR: run from the repository root"; exit 1; }

val() { grep -E "^$1=" "$2" | head -1 | cut -d= -f2- | sed "s/^[\"']//; s/[\"']\$//"; }

# Depot project read from the depot regime (rbrd.env); derivation copied from
# RBGC_depot_project_infix (Tools/rbk/rbgc_constants.sh) and RBDC_DEPOT_PROJECT_ID
# (Tools/rbk/rbdc_derived.sh): prefix, then "d-", then moniker.
z_depot_prefix=$(val RBRD_CLOUD_PREFIX "$RBRD")
z_depot_moniker=$(val RBRD_DEPOT_MONIKER "$RBRD")
[ -n "${z_depot_prefix}" ] && [ -n "${z_depot_moniker}" ] \
  || { echo "ERROR: RBRD_CLOUD_PREFIX or RBRD_DEPOT_MONIKER missing in rbmm_moorings/rbrd.env"; exit 1; }
DEPOT_PROJECT="${z_depot_prefix}d-${z_depot_moniker}"
MANTLE_SA="rbma-director@${DEPOT_PROJECT}.iam.gserviceaccount.com"

# The secrets ride jq's environment by command prefix, and the body jq composes in
# memory reaches curl through a pipe: no secret stands in any process's arguments.
payor_token() {
  local z_cid zSECRET_csec zSECRET_rtok
  z_cid=$(val RBRP_OAUTH_CLIENT_ID "$RBRP")
  zSECRET_csec=$(val RBRO_zSECRET_CLIENT_SECRET "$RBRO")
  zSECRET_rtok=$(val RBRO_zSECRET_REFRESH_TOKEN "$RBRO")
  Z_CID="${z_cid}" Z_CSEC="${zSECRET_csec}" Z_RTOK="${zSECRET_rtok}" \
    jq -nr '"client_id=\(env.Z_CID|@uri)&client_secret=\(env.Z_CSEC|@uri)&refresh_token=\(env.Z_RTOK|@uri)&grant_type=refresh_token"' \
    | curl -s https://oauth2.googleapis.com/token --data @- \
    | jq -r '.access_token // empty'
}

# Remove an IAM binding, reporting cleanly whether it was present or already gone.
attaint_binding() {
  local z_label="$1"; shift
  local z_out
  z_out=$(gcloud "$@" --condition=None 2>&1) || true
  if   printf '%s' "$z_out" | grep -qiE 'not found|does not have'; then echo "  ${z_label}: already removed"
  elif printf '%s' "$z_out" | grep -qiE 'etag';                     then echo "  ${z_label}: removed"
  else echo "  ${z_label}: $(printf '%s' "$z_out" | tail -1)"; fi
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
