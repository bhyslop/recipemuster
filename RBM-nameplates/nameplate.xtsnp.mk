# Recipe Bottle Nameplate Configuration
# Jupyter Notebook environment with Claude API access

# Core Service Definition
export RBN_MONIKER     := xtsnp
export RBN_DESCRIPTION := Example Test No Port: nameplate for testing without advertising a port

# Image Source Configuration
export RBN_SENTRY_REPO_FULL_NAME := ghcr.io/bhyslop/recipemuster
export RBN_BOTTLE_REPO_FULL_NAME := ghcr.io/bhyslop/recipemuster
export RBN_SENTRY_IMAGE_TAG      := bottle_ubuntu_test.20241206__014156

# TEMPORARY CUT
# export RBN_BOTTLE_IMAGE_TAG      := bottle_anthropic_jupyter.20241020__173503
export RBN_BOTTLE_IMAGE_TAG      := bottle_ubuntu_test.20241206__014156

# Port Service Configuration
export RBN_PORT_ENABLED := 1
export RBN_PORT_UPLINK  := 8889
export RBN_PORT_ENCLAVE := 8888
export RBN_PORT_SERVICE := 8888

# Network Uplink Configuration
#    Worksheet: https://claude.ai/chat/3b81ecc4-c3bd-4e71-82af-4f0feec7ce97
export RBN_UPLINK_DNS_ENABLED     := 1
export RBN_UPLINK_ACCESS_ENABLED  := 1
export RBN_UPLINK_DNS_GLOBAL      := 0
export RBN_UPLINK_ACCESS_GLOBAL   := 0
export RBN_UPLINK_ALLOWED_CIDRS   := 93.184.216.0/26
export RBN_UPLINK_ALLOWED_DOMAINS := example.com


