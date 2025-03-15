#!/bin/bash

# Simplified Container Network Setup Script
# Each step is executed discretely with minimal environment passing

set -e  # Exit on error

# Color codes for output
GREEN='\033[0;32m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Hardcoded configuration values
MACHINE="pdvm-rbw"
MONIKER="nsproto"
SENTRY_CONTAINER="${MONIKER}-sentry"
BOTTLE_CONTAINER="${MONIKER}-bottle"
ENCLAVE_BRIDGE="vbr_${MONIKER}"
ENCLAVE_SENTRY_IN="vsi_${MONIKER}"
ENCLAVE_SENTRY_OUT="vso_${MONIKER}"
ENCLAVE_BOTTLE_IN="vbi_${MONIKER}"
ENCLAVE_BOTTLE_OUT="vbo_${MONIKER}"
ENCLAVE_BASE_IP="10.242.0.0"
ENCLAVE_NETMASK="24"
ENCLAVE_SENTRY_IP="10.242.0.2"
ENCLAVE_BOTTLE_IP="10.242.0.3"
ENTRY_PORT_WORKSTATION="8890"
ENTRY_PORT_ENCLAVE="8888"
SENTRY_REPO_PATH="ghcr.io/bhyslop/recipemuster"
BOTTLE_REPO_PATH="ghcr.io/bhyslop/recipemuster"
SENTRY_IMAGE_TAG="sentry_ubuntu_large.20241022__130547"
BOTTLE_IMAGE_TAG="bottle_ubuntu_test.20241207__190758"
DNS_SERVER="8.8.8.8"
UPLINK_ALLOWED_CIDRS="160.79.104.0/23"
UPLINK_ALLOWED_DOMAINS="anthropic.com"

