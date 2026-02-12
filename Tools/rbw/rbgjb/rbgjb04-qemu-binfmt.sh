#!/bin/bash
# RBGJB Step 04: Register QEMU for cross-platform builds
# Builder: gcr.io/cloud-builders/docker

set -euo pipefail

# Register QEMU for cross-platform builds (arm64, arm/v7)
docker run --privileged --rm ${RBRR_GCB_BINFMT_IMAGE_REF} --install arm64,arm
