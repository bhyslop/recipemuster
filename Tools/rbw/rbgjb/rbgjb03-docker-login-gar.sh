#!/bin/bash
# RBGJB Step 03: Docker login to Google Artifact Registry
# Builder: gcr.io/cloud-builders/docker
# Substitutions: _RBGY_GAR_LOCATION

set -euo pipefail

# Login using token from previous step (works with buildx docker-container driver)
cat /workspace/.docker-token | docker login -u oauth2accesstoken --password-stdin "https://${_RBGY_GAR_LOCATION}${_RBGY_GAR_HOST_SUFFIX}"
rm /workspace/.docker-token
