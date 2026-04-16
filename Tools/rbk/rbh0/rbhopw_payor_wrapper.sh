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
# Recipe Bottle Handbook Onboarding - Payor Handbook (establish Manor and Depot)

set -euo pipefail

test -z "${ZRBHOPW_SOURCED:-}" || return 0
ZRBHOPW_SOURCED=1

######################################################################
# Payor handbook — establish a Manor and provision the Depot
#
# Linear step sequence, no conditional probes. The payor owns the GCP
# project and funds it. This handbook walks through the full ceremony:
# OAuth credentials, project setup, depot provisioning, governor handoff.

rbho_payor_handbook() {
  zrbho_sentinel

  buc_doc_brief "Payor — establish a Manor and provision the Depot"
  buc_doc_shown || return 0

  # --- Header ---
  buh_section "Payor — Establish a Manor and Provision the Depot"
  buh_e
  buh_line "The ${RBYC_PAYOR} establishes a ${RBYC_MANOR} — an administrative seat"
  buh_line "holding the billing account, OAuth client, and operator identity."
  buh_line "Unlike other roles that use service account keys, the ${RBYC_PAYOR}"
  buh_line "authenticates via OAuth — representing the human project owner."
  buh_e
  buh_line "By the end of this handbook you will have a ${RBYC_MANOR}, a ${RBYC_DEPOT}"
  buh_line "funded under it, and a ${RBYC_GOVERNOR} service account ready to administer it."
  buh_e

  buh_line "This ceremony takes about 15 minutes."
  buh_e

  buh_step_style "Step " " — "

  # =================================================================
  # Step 1: Establish the Manor
  # =================================================================
  buh_step1 "Establish the Manor"
  buh_e
  buh_line "The ${RBYC_MANORS} GCP project hosts the OAuth client and billing"
  buh_line "account. It must be created before any infrastructure can be"
  buh_line "provisioned."
  buh_e
  buh_line "Run the guided setup:"
  buh_tt  "  " "${RBZ_PAYOR_ESTABLISH}"
  buh_e
  buh_line "This guides you through creating the ${RBYC_MANORS} GCP project,"
  buh_line "enabling billing, and configuring the OAuth consent screen. The ${RBYC_MANOR}"
  buh_line "identity is recorded in ${RBYC_RBRP}."
  buh_e

  # =================================================================
  # Step 2: Install OAuth credentials
  # =================================================================
  buh_step1 "Install OAuth credentials"
  buh_e
  buh_line "Step 1 ended with downloading a JSON client secret file from the"
  buh_line "OAuth client you just created. Install it:"
  buh_e
  buh_tt  "  " "${RBZ_PAYOR_INSTALL}" "" " \${HOME}/Downloads/client_secret_*.json"
  buh_e
  buh_line "This walks you through the OAuth authorization flow and stores"
  buh_line "the credential securely."
  buh_e
  buh_line "If you are refreshing an existing credential that has expired:"
  buh_tt  "  " "${RBZ_PAYOR_REFRESH}"
  buh_e

  # =================================================================
  # Step 3: Provision the Depot
  # =================================================================
  buh_step1 "Provision the Depot"
  buh_e
  buh_line "A ${RBYC_DEPOT} is the facility where container images are built and"
  buh_line "stored — a GCP project with a container repository, storage bucket,"
  buh_line "and build infrastructure, funded under the ${RBYC_MANORS} billing account."
  buh_line "A ${RBYC_GOVERNOR} administers the ${RBYC_DEPOT} — creating"
  buh_line "${RBYC_RETRIEVER} and ${RBYC_DIRECTOR} accounts for those who build and"
  buh_line "retrieve container images."
  buh_e
  buh_line "${RBYC_PAYOR} creates the Depot:"
  buh_tt  "  " "${RBZ_LEVY_DEPOT}"
  buh_e
  buh_line "This enables APIs, creates the Artifact Registry repository and"
  buh_line "Cloud Storage bucket, and configures Cloud Build."
  buh_e
  buh_line "${RBYC_PAYOR} can list Depots for verification:"
  buh_tt  "  " "${RBZ_LIST_DEPOT}"
  buh_e
  buh_line "${RBYC_PAYOR} creates the Governor service account:"
  buh_tt  "  " "${RBZ_MANTLE_GOVERNOR}"
  buh_e
  buh_line "Hand the resulting key file to the person who will administer"
  buh_line "this ${RBYC_DEPOT}. After this handoff, the ${RBYC_GOVERNOR} can create"
  buh_line "${RBYC_RETRIEVER} and ${RBYC_DIRECTOR} accounts independently."
  buh_e
  buh_line "The ${RBYC_PAYORS} job for this ${RBYC_DEPOT} is done unless billing or"
  buh_line "project-level changes are needed."
  buh_e

  # --- Return to start ---
  buh_tt  "Return to start: " "${RBZ_ONBOARD_START_HERE}"
  buh_e
}

# eof
