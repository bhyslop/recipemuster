#!/bin/bash
# RBGJB Step 05: Create and bootstrap buildx builder
# Builder: gcr.io/cloud-builders/docker

set -euo pipefail

docker buildx create --name rbia-builder --driver docker-container --use
docker buildx inspect --bootstrap
