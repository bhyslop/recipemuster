# Recipe Bottle Nameplate Configuration
# Jupyter Notebook environment with Claude API access

# Core Service Definition
export RBN_MONIKER     := srjcl
export RBN_DESCRIPTION := Jupyter Notebook environment with Claude API access for AI-assisted development

# Image Source Configuration
export RBN_SENTRY_REPO_PATH := ghcr.io/bhyslop/recipemuster
export RBN_BOTTLE_REPO_PATH := ghcr.io/bhyslop/recipemuster
export RBN_SENTRY_IMAGE_TAG := sentry_ubuntu_large.20250130__154657
export RBN_BOTTLE_IMAGE_TAG := bottle_anthropic_jupyter.20250130__152820

# Port Service Configuration
export RBN_PORT_ENABLED           := 1
export RBN_ENTRY_PORT_WORKSTATION := 8000
export RBN_ENTRY_PORT_ENCLAVE     := 8000

# Network Uplink Configuration
export RBN_UPLINK_PORT_MIN        := 10000
export RBN_UPLINK_DNS_ENABLED     := 1
export RBN_UPLINK_ACCESS_ENABLED  := 1
export RBN_UPLINK_DNS_GLOBAL      := 0
export RBN_UPLINK_ACCESS_GLOBAL   := 0
export RBN_UPLINK_ALLOWED_CIDRS   := 160.79.104.0/23
export RBN_UPLINK_ALLOWED_DOMAINS := anthropic.com

# Network Configuration
export RBN_ENCLAVE_BASE_IP    := 10.242.0.0
export RBN_ENCLAVE_NETMASK    := 24
export RBN_ENCLAVE_SENTRY_IP  := 10.242.0.2
export RBN_ENCLAVE_BOTTLE_IP  := 10.242.0.3

# Volume Mount Configuration
export RBN_VOLUME_MOUNTS := -v ./RBM-environments-srjcl:/workspace:Z
