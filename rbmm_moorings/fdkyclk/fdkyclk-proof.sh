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
# fdkyclk-proof.sh — ride-or-die proof driver (heat ridge ₢BfAAH).
#
# Proves, from a clean headless shell, that a Keycloak-minted OIDC id_token
# federates into GCP via uploaded JWKS and dons a mantle. Reproducible and
# idempotent: re-running advances/confirms rather than duplicating. This is the
# PROOF-STAGE ancestor of the deferred rbx_ establishment/accessor units — NOT
# yet BCG-productized.
#
# Run from the repository root:  bash rbmm_moorings/fdkyclk/fdkyclk-proof.sh
# Prereqs: fdkyclk crucible charged (tt/rbw-cC.Charge.fdkyclk.sh); payor
# credential installed (tt/rbw-gPI...); gcloud + jq + python3 on PATH.
#
# PROVISIONAL NAMES (pending the pre-wrap naming reconsideration):
#   pool=fdkyclk-test  provider=keycloak  client=fdkyclk-gcp  issuer=https://fdkyclk.test
# The Keycloak token below is currently minted via the password grant — a
# deliberate de-risk stepping-stone; the deliverable lands on the RFC 7523 JWT
# Authorization Grant (config-model fork resolution).
#
# Stage B brevet is payor-DIRECT, not via rbgp_brevet: the real admission verbs
# assume the single manor pool (spike-office-test), so admitting a subject in the
# separate fdkyclk-test pool needs multiplicity-aware verbs — a separate build
# unit. The proof grants the IAM directly to prove the don+call chain.

set -euo pipefail

# ---- constants ----
KC=http://localhost:8088
REALM=fdkyclk
KC_CLIENT=fdkyclk-gcp
KC_CLIENT_SECRET=fdkyclk-test-secret
KC_USER=federate
KC_PASS=federate
FRONTEND_URL=https://fdkyclk.test                       # GCP requires an https issuer; never resolved (uploaded JWKS)
GCP_ORG=247899326218
GCP_POOL=fdkyclk-test
GCP_PROVIDER=keycloak
GCP_AUDIENCE=fdkyclk-gcp                                 # GCP --client-id; must equal the id_token aud
ATTR_MAPPING="google.subject=assertion.sub"
ISSUER="${FRONTEND_URL}/realms/${REALM}"
JWKS_FILE=/tmp/fdkyclk-jwks.json
RBRP=rbmm_moorings/rbrp.env
RBRO=../station-files/secrets/payor/rbro.env

# Stage B (don) — depot facts of the levied depot (rbrd.env + kludge image tags).
DEPOT_PROJECT=cancbhm-d-canest3bhm100002
DEPOT_REGION=us-central1
MANTLE=director
MANTLE_SA="rbma-${MANTLE}@${DEPOT_PROJECT}.iam.gserviceaccount.com"

[ -d rbmm_moorings ] || { echo "ERROR: run from the repository root"; exit 1; }

say() { printf '\n=== %s ===\n' "$*"; }
val() { grep -E "^$1=" "$2" | head -1 | cut -d= -f2- | sed "s/^[\"']//; s/[\"']\$//"; }

# ---- payor access token (OAuth refresh-token exchange; never echoes secrets) ----
payor_token() {
  local cid csec rtok
  cid=$(val RBRP_OAUTH_CLIENT_ID "$RBRP")
  csec=$(val RBRO_CLIENT_SECRET  "$RBRO")
  rtok=$(val RBRO_REFRESH_TOKEN  "$RBRO")
  curl -s https://oauth2.googleapis.com/token \
    --data-urlencode "client_id=${cid}" \
    --data-urlencode "client_secret=${csec}" \
    --data-urlencode "refresh_token=${rtok}" \
    --data-urlencode "grant_type=refresh_token" | jq -r '.access_token // empty'
}

kc_admin_token() {
  curl -s "$KC/realms/master/protocol/openid-connect/token" \
    -d grant_type=password -d client_id=admin-cli \
    -d username=admin -d password=admin | jq -r '.access_token // empty'
}

# ---- configure realm (idempotent): client, user, frontendUrl ----
configure_realm() {
  local at code uid
  at=$(kc_admin_token); [ -n "$at" ] || { echo "no kc admin token"; return 1; }

  code=$(curl -s -o /dev/null -w '%{http_code}' -X POST "$KC/admin/realms/$REALM/clients" \
    -H "Authorization: Bearer $at" -H "Content-Type: application/json" \
    -d "{\"clientId\":\"$KC_CLIENT\",\"enabled\":true,\"protocol\":\"openid-connect\",\"publicClient\":false,\"secret\":\"$KC_CLIENT_SECRET\",\"standardFlowEnabled\":false,\"implicitFlowEnabled\":false,\"directAccessGrantsEnabled\":true,\"serviceAccountsEnabled\":true,\"fullScopeAllowed\":true,\"protocolMappers\":[{\"name\":\"gcp-aud\",\"protocol\":\"openid-connect\",\"protocolMapper\":\"oidc-audience-mapper\",\"config\":{\"included.custom.audience\":\"$GCP_AUDIENCE\",\"id.token.claim\":\"true\",\"access.token.claim\":\"false\"}}]}")
  echo "client create http=$code (201 new / 409 exists both ok)"

  code=$(curl -s -o /dev/null -w '%{http_code}' -X POST "$KC/admin/realms/$REALM/users" \
    -H "Authorization: Bearer $at" -H "Content-Type: application/json" \
    -d "{\"username\":\"$KC_USER\",\"enabled\":true,\"email\":\"$KC_USER@fdkyclk.test\",\"emailVerified\":true,\"firstName\":\"Federate\",\"lastName\":\"Subject\"}")
  echo "user create http=$code (201 new / 409 exists both ok)"

  uid=$(curl -s "$KC/admin/realms/$REALM/users?username=$KC_USER" -H "Authorization: Bearer $at" | jq -r '.[0].id')
  curl -s -o /dev/null -w 'user profile http=%{http_code}\n' -X PUT "$KC/admin/realms/$REALM/users/$uid" \
    -H "Authorization: Bearer $at" -H "Content-Type: application/json" \
    -d '{"firstName":"Federate","lastName":"Subject","emailVerified":true,"enabled":true,"requiredActions":[]}'
  curl -s -o /dev/null -w 'set password http=%{http_code}\n' -X PUT "$KC/admin/realms/$REALM/users/$uid/reset-password" \
    -H "Authorization: Bearer $at" -H "Content-Type: application/json" \
    -d "{\"type\":\"password\",\"value\":\"$KC_PASS\",\"temporary\":false}"

  curl -s "$KC/admin/realms/$REALM" -H "Authorization: Bearer $at" > /tmp/realm.json
  jq --arg fu "$FRONTEND_URL" '.attributes = ((.attributes // {}) + {"frontendUrl":$fu})' /tmp/realm.json > /tmp/realm2.json
  curl -s -o /dev/null -w 'set frontendUrl http=%{http_code}\n' -X PUT "$KC/admin/realms/$REALM" \
    -H "Authorization: Bearer $at" -H "Content-Type: application/json" --data @/tmp/realm2.json
}

fetch_jwks() {
  # GCP's uploaded-JWKS parser is strict — strip Keycloak's x5c/x5t cert fields
  # down to the standard RSA public-key JWK members (kty/kid/use/alg/n/e), else
  # GCP rejects with a misleading "Only RSA, EC key types are supported".
  curl -s "$KC/realms/$REALM/protocol/openid-connect/certs" \
    | jq '{keys: [.keys[] | select(.use=="sig") | {kty, use, kid, alg, n, e}]}' > "$JWKS_FILE"
  jq -c '.keys[] | {kid,alg,use}' "$JWKS_FILE"
}

# ---- ensure GCP pool + programmatic provider (idempotent) ----
ensure_gcp_provider() {
  export CLOUDSDK_AUTH_ACCESS_TOKEN; CLOUDSDK_AUTH_ACCESS_TOKEN=$(payor_token)
  [ -n "$CLOUDSDK_AUTH_ACCESS_TOKEN" ] || { echo "no payor token"; return 1; }

  if gcloud iam workforce-pools describe "$GCP_POOL" --location=global >/dev/null 2>&1; then
    echo "pool $GCP_POOL exists"
  else
    gcloud iam workforce-pools create "$GCP_POOL" --organization="$GCP_ORG" --location=global \
      --display-name="fdkyclk Keycloak caged test" \
      --description="Provisional caged programmatic test trust (ride-or-die proof); name pending."
  fi

  if gcloud iam workforce-pools providers describe "$GCP_PROVIDER" \
        --workforce-pool="$GCP_POOL" --location=global >/dev/null 2>&1; then
    echo "provider $GCP_PROVIDER exists"
  else
    gcloud iam workforce-pools providers create-oidc "$GCP_PROVIDER" \
      --workforce-pool="$GCP_POOL" --location=global \
      --display-name="Keycloak programmatic (caged)" \
      --issuer-uri="$ISSUER" --client-id="$GCP_AUDIENCE" \
      --jwk-json-path="$JWKS_FILE" --attribute-mapping="$ATTR_MAPPING" \
      --web-sso-response-type=id-token \
      --web-sso-assertion-claims-behavior=only-id-token-claims
  fi
}

# ---- mint Keycloak id_token (password grant; RFC 7523 to follow) ----
mint_idtoken() {
  curl -s "$KC/realms/$REALM/protocol/openid-connect/token" \
    -d grant_type=password -d client_id="$KC_CLIENT" -d client_secret="$KC_CLIENT_SECRET" \
    -d username="$KC_USER" -d password="$KC_PASS" -d scope=openid | jq -r '.id_token // empty'
}

# ---- STS exchange: Keycloak id_token -> Google federated token ----
sts_exchange() {
  local idtok="$1"
  local aud="//iam.googleapis.com/locations/global/workforcePools/${GCP_POOL}/providers/${GCP_PROVIDER}"
  curl -s -X POST "https://sts.googleapis.com/v1/token" \
    -H "Content-Type: application/x-www-form-urlencoded" \
    --data-urlencode "grant_type=urn:ietf:params:oauth:grant-type:token-exchange" \
    --data-urlencode "audience=${aud}" \
    --data-urlencode "scope=https://www.googleapis.com/auth/cloud-platform" \
    --data-urlencode "requested_token_type=urn:ietf:params:oauth:token-type:access_token" \
    --data-urlencode "subject_token_type=urn:ietf:params:oauth:token-type:id_token" \
    --data-urlencode "subject_token=${idtok}"
}

decode_jwt() {
  python3 -c "import sys,base64,json;t=sys.argv[1].split('.')[1];t+='='*(-len(t)%4);d=json.loads(base64.urlsafe_b64decode(t));print(json.dumps({k:d.get(k) for k in ['iss','aud','sub','typ','exp']},indent=2))" "$1" 2>/dev/null || true
}
decode_sub() {
  python3 -c "import sys,base64,json;t=sys.argv[1].split('.')[1];t+='='*(-len(t)%4);print(json.loads(base64.urlsafe_b64decode(t))['sub'])" "$1"
}

# ---- Stage B: brevet (payor-direct), don, authorized AR call ----
brevet() {
  local principal="$1"
  export CLOUDSDK_AUTH_ACCESS_TOKEN; CLOUDSDK_AUTH_ACCESS_TOKEN=$(payor_token)
  gcloud iam service-accounts add-iam-policy-binding "$MANTLE_SA" --project="$DEPOT_PROJECT" \
    --member="$principal" --role="roles/iam.serviceAccountTokenCreator" --condition=None >/dev/null \
    && echo "granted tokenCreator on $MANTLE_SA"
  gcloud projects add-iam-policy-binding "$DEPOT_PROJECT" \
    --member="$principal" --role="roles/serviceusage.serviceUsageConsumer" --condition=None >/dev/null \
    && echo "granted serviceUsageConsumer on $DEPOT_PROJECT"
}

don() {
  local fed="$1" i tok
  for i in 1 2 3 4 5 6 7 8; do
    tok=$(curl -s -X POST \
      "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/${MANTLE_SA}:generateAccessToken" \
      -H "Authorization: Bearer ${fed}" \
      -H "x-goog-user-project: ${DEPOT_PROJECT}" \
      -H "Content-Type: application/json" \
      --data '{"scope":["https://www.googleapis.com/auth/cloud-platform"]}' \
      | jq -r '.accessToken // empty')
    [ -n "$tok" ] && { printf '%s' "$tok"; return 0; }
    echo "don attempt $i: not yet (IAM propagation) — sleeping 10s" >&2
    sleep 10
  done
  return 1
}

ar_call() {
  local tok="$1"
  local url="https://artifactregistry.googleapis.com/v1/projects/${DEPOT_PROJECT}/locations/${DEPOT_REGION}/repositories"
  curl -s -H "Authorization: Bearer ${tok}" "$url" \
    | jq 'if has("repositories") then {RESULT:"AUTHORIZED-AR-CALL-OK", repo_count:(.repositories|length), repos:[.repositories[].name]} else {RESULT:"FAILED", error:.error} end'
}

main() {
  say "Stage A.1 — configure Keycloak realm (client / user / frontendUrl)"
  configure_realm

  say "Stage A.2 — fetch signing JWKS"
  fetch_jwks

  say "Stage A.3 — ensure GCP pool + programmatic provider"
  ensure_gcp_provider

  say "Stage A.4 — mint Keycloak id_token"
  local idtok; idtok=$(mint_idtoken)
  [ -n "$idtok" ] || { echo "FAILED to mint id_token"; return 1; }
  decode_jwt "$idtok"

  say "Stage A.5 — STS exchange (Keycloak id_token -> Google federated token)"
  local sts fed
  sts=$(sts_exchange "$idtok")
  fed=$(echo "$sts" | jq -r '.access_token // empty')
  [ -n "$fed" ] || { echo "STS FAILED:"; echo "$sts" | jq '{error,error_description}'; return 1; }
  echo "{\"RESULT\":\"FEDERATED-TOKEN-OK\",\"fed_token_len\":${#fed}}"

  local sub principal
  sub=$(decode_sub "$idtok")
  principal="principal://iam.googleapis.com/locations/global/workforcePools/${GCP_POOL}/subject/${sub}"

  say "Stage B.1 — brevet (payor-direct): grant the fdkyclk subject tokenCreator on the ${MANTLE} mantle + serviceUsageConsumer"
  brevet "$principal"

  say "Stage B.2 — don the ${MANTLE} mantle (generateAccessToken with the federated token)"
  local mtok; mtok=$(don "$fed") || { echo "DON FAILED"; return 1; }
  echo "mantle token len: ${#mtok}"

  say "Stage B.3 — authorized depot-API call (Artifact Registry repositories.list)"
  ar_call "$mtok"
}

main "$@"
