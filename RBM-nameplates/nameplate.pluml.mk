# © 2024 Scale Invariant.  All rights reserved.

# Core Service Definition
RBRN_MONIKER     = pluml
RBRN_DESCRIPTION = Local PlantUML Server

# Image Source Configuration
RBRN_SENTRY_REPO_PATH = ghcr.io/bhyslop/recipemuster
RBRN_BOTTLE_REPO_PATH = ghcr.io/bhyslop/recipemuster
RBRN_SENTRY_IMAGE_TAG = sentry_ubuntu_large.20250130__154657
RBRN_BOTTLE_IMAGE_TAG = bottle_plantuml.20250206__133847

# Port Service Configuration
RBRN_ENTRY_ENABLED          = 1
RBRN_ENTRY_PORT_WORKSTATION = 8001
RBRN_ENTRY_PORT_ENCLAVE     = 8080

# Network Uplink Configuration
RBRN_UPLINK_PORT_MIN        = 10000
RBRN_UPLINK_DNS_ENABLED     = 0
RBRN_UPLINK_ACCESS_ENABLED  = 0
RBRN_UPLINK_DNS_GLOBAL      = 0
RBRN_UPLINK_ACCESS_GLOBAL   = 0

# OUCH NOT ACTUALLY PROPER: need to enable optional
RBRN_UPLINK_ALLOWED_CIDRS   = 160.79.104.0/23
RBRN_UPLINK_ALLOWED_DOMAINS = anthropic.com

# Network Configuration
RBRN_ENCLAVE_BASE_IP    = 10.242.0.0
RBRN_ENCLAVE_NETMASK    = 24
RBRN_ENCLAVE_SENTRY_IP  = 10.242.0.2
RBRN_ENCLAVE_BOTTLE_IP  = 10.242.0.3

# Volume Mount Configuration

# OUCH NOT ACTUALLY PROPER: need to enable optional
RBRN_VOLUME_MOUNTS = -v ./RBM-environments-srjcl:/workspace:Z


# eof
