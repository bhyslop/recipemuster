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
# Recipe Bottle Handbook Onboarding - Governor Handbook (administer service accounts)

set -euo pipefail

test -z "${ZRBHOGW_SOURCED:-}" || return 0
ZRBHOGW_SOURCED=1

######################################################################
# Governor handbook — administer service accounts for directors and
# retrievers.
#
# Linear step sequence, no conditional probes. The governor operates
# within a depot the payor created: installs governor credentials,
# provisions downstream SAs, verifies the chain.

rbho_governor_handbook() {
  buc_doc_brief "Governor — administer service accounts for directors and retrievers"
  buc_doc_shown || return 0

  # --- Header ---
  buh_section "Governor — Administer Service Accounts"
  buh_e
  buh_line "A ${RBYC_GOVERNOR} administers a ${RBYC_DEPOT} — creating service accounts"
  buh_line "and managing access for those who build and run container images."
  buh_e

  buh_step_style "Step " " — "

  # =================================================================
  # Step 1: Install governor credentials
  # =================================================================
  buh_step1 "Install governor credentials"
  buh_e
  buh_line "The ${RBYC_GOVERNOR} works within a ${RBYC_DEPOT} provisioned under the"
  buh_line "${RBYC_PAYORS} ${RBYC_MANOR}. Your Payor creates your credential by running:"
  buh_tt  "  " "${RBZ_MANTLE_GOVERNOR}"
  buh_e
  buh_line "If no ${RBYC_DEPOT} exists yet, the ${RBYC_PAYOR} establishes one first:"
  buh_tt  "  " "${RBZ_ONBOARD_PAYOR_HB}"
  buh_e
  buh_line "Install the resulting key file into the secrets directory under"
  buh_line "the governor role subdirectory. The path is derived from your"
  buh_line "${RBYC_RBRR} configuration — check RBRR_SECRETS_DIR for the location."
  buh_e

  # =================================================================
  # Step 2: Provision downstream service accounts
  # =================================================================
  buh_step1 "Provision downstream service accounts"
  buh_e
  buh_line "The governor provisions access for two downstream roles:"
  buh_e
  buh_line "A ${RBYC_RETRIEVER} has read access to the ${RBYC_DEPOT} — they pull and run"
  buh_line "container images that others have built."
  buh_line "A ${RBYC_DIRECTOR} has build and publish access — they create container"
  buh_line "images and push them to the registry."
  buh_e
  buh_line "Create a ${RBYC_RETRIEVER} with read access (${RBYC_CHARTER}):"
  buh_tt  "  " "${RBZ_CHARTER_RETRIEVER}"
  buh_e
  buh_line "Create a ${RBYC_DIRECTOR} with build access (${RBYC_KNIGHT}):"
  buh_tt  "  " "${RBZ_KNIGHT_DIRECTOR}"
  buh_e
  buh_line "Each command creates the service account and applies the IAM"
  buh_line "grants it needs. The output is an ${RBYC_RBRA} key file — hand it to"
  buh_line "the Retriever or Director user."
  buh_e
  buh_line "List issued service accounts:"
  buh_tt  "  " "${RBZ_LIST_SERVICE_ACCOUNTS}"
  buh_e

  # =================================================================
  # Step 3: Verify the chain
  # =================================================================
  buh_step1 "Verify the chain"
  buh_e
  buh_line "The service accounts you created include IAM grants — each SA"
  buh_line "gets exactly the permissions its role requires, no more."
  buh_line "${RBYC_RETRIEVER} gets read access."
  buh_line "${RBYC_DIRECTOR} gets read, write, and build trigger access."
  buh_e
  buh_line "Verify the complete chain works by installing both credentials"
  buh_line "locally and running the credential handbook tracks:"
  buh_tt  "  " "${RBZ_ONBOARD_CRED_RETRIEVER}"
  buh_tt  "  " "${RBZ_ONBOARD_CRED_DIRECTOR}"
  buh_e
  buh_line "If the ${RBYC_RETRIEVER} can pull from the ${RBYC_DEPOT} and the"
  buh_line "${RBYC_DIRECTOR} can see the registry, your grants are correct."
  buh_e

  # --- Return to start ---
  buh_tt  "Return to start: " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

# eof
