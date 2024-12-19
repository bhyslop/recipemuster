# Recipe Bottle Nameplate Configuration
# Jupyter Notebook environment with Claude API access

# Core Service Definition
export RBN_MONIKER     := srjcl
export RBN_DESCRIPTION := Jupyter Notebook environment with Claude API access for AI-assisted development

# Image Source Configuration
export RBN_SENTRY_REPO_FULL_NAME := ghcr.io/bhyslop/recipemuster
export RBN_BOTTLE_REPO_FULL_NAME := ghcr.io/bhyslop/recipemuster
export RBN_SENTRY_IMAGE_TAG      := sentry_alpine_large.20241022__125927

# TEMPORARY CUT
# export RBN_BOTTLE_IMAGE_TAG      := bottle_anthropic_jupyter.20241020__173503
export RBN_BOTTLE_IMAGE_TAG      := bottle_ubuntu_test.20241206__014156

# Port Service Configuration
export RBN_PORT_ENABLED           := 1
export RBN_ENTRY_PORT_WORKSTATION := 8889
export RBN_ENTRY_PORT_ENCLAVE     := 8888
export RBN_PORT_SERVICE           := 8888

# Network Uplink Configuration
export RBN_UPLINK_DNS_ENABLED     := 1
export RBN_UPLINK_ACCESS_ENABLED  := 1
export RBN_UPLINK_DNS_GLOBAL      := 1
export RBN_UPLINK_ACCESS_GLOBAL   := 1
export RBN_UPLINK_ALLOWED_CIDRS   := 160.79.104.0/23
export RBN_UPLINK_ALLOWED_DOMAINS := anthropic.com

# Volume Mount Configuration
export RBN_VOLUME_MOUNTS := -v ./RBM-environments-srjcl:/mnt/bottle-data:Z
