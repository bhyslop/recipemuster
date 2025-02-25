# © 2024 Scale Invariant.  All rights reserved.

# Core Service Definition
RBN_MONIKER     = pluml
RBN_DESCRIPTION = Local PlantUML Server

# Image Source Configuration
RBN_SENTRY_REPO_PATH = ghcr.io/bhyslop/recipemuster
RBN_BOTTLE_REPO_PATH = ghcr.io/bhyslop/recipemuster
RBN_SENTRY_IMAGE_TAG = sentry_ubuntu_large.20250130__154657
RBN_BOTTLE_IMAGE_TAG = bottle_plantuml.20250206__133847

# Port Service Configuration
RBN_ENTRY_ENABLED          = 1
RBN_ENTRY_PORT_WORKSTATION = 8001
RBN_ENTRY_PORT_ENCLAVE     = 8080

# Network Uplink Configuration
RBN_UPLINK_PORT_MIN        = 10000
RBN_UPLINK_DNS_ENABLED     = 0
RBN_UPLINK_ACCESS_ENABLED  = 0
RBN_UPLINK_DNS_GLOBAL      = 0
RBN_UPLINK_ACCESS_GLOBAL   = 0

# OUCH NOT ACTUALLY PROPER: need to enable optional
RBN_UPLINK_ALLOWED_CIDRS   = 160.79.104.0/23
RBN_UPLINK_ALLOWED_DOMAINS = anthropic.com

# Network Configuration
RBN_ENCLAVE_BASE_IP    = 10.242.0.0
RBN_ENCLAVE_NETMASK    = 24
RBN_ENCLAVE_SENTRY_IP  = 10.242.0.2
RBN_ENCLAVE_BOTTLE_IP  = 10.242.0.3

# Volume Mount Configuration

# OUCH NOT ACTUALLY PROPER: need to enable optional
RBN_VOLUME_MOUNTS = -v ./RBM-environments-srjcl:/workspace:Z


# eof
