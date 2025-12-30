#!/bin/bash
# RBGJB Step 02: Get Docker access token from gcloud
# Builder: gcr.io/cloud-builders/gcloud

set -euo pipefail

# Write access token to workspace for docker login step
gcloud auth print-access-token > /workspace/.docker-token
