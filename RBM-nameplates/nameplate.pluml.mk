# © 2024 Scale Invariant.  All rights reserved.

# Core Service Definition
export RBN_MONIKER     := pluml
export RBN_DESCRIPTION := Local PlantUML Server

# Image Source Configuration
export RBN_SENTRY_REPO_PATH := ghcr.io/bhyslop/recipemuster
export RBN_BOTTLE_REPO_PATH := ghcr.io/bhyslop/recipemuster
export RBN_SENTRY_IMAGE_TAG := sentry_ubuntu_large.20250130__154657
export RBN_BOTTLE_IMAGE_TAG := bottle_plantuml.20250206__133847

# Port Service Configuration
export RBN_PORT_ENABLED           := 1
export RBN_ENTRY_PORT_WORKSTATION := 8001
export RBN_ENTRY_PORT_ENCLAVE     := 8080

# Network Uplink Configuration
export RBN_UPLINK_PORT_MIN        := 10000
export RBN_UPLINK_DNS_ENABLED     := 0
export RBN_UPLINK_ACCESS_ENABLED  := 0
export RBN_UPLINK_DNS_GLOBAL      := 0
export RBN_UPLINK_ACCESS_GLOBAL   := 0
export RBN_UPLINK_ALLOWED_CIDRS   :=
export RBN_UPLINK_ALLOWED_DOMAINS :=

# Network Configuration
export RBN_ENCLAVE_BASE_IP    := 10.242.0.0
export RBN_ENCLAVE_NETMASK    := 24
export RBN_ENCLAVE_SENTRY_IP  := 10.242.0.2
export RBN_ENCLAVE_BOTTLE_IP  := 10.242.0.3

# Volume Mount Configuration
export RBN_VOLUME_MOUNTS := 


# eof
