# Recipe Bottle Nameplate Configuration
# Jupyter Notebook environment with Claude API access

# Core Service Definition
export RBN_MONIKER     := xtsnp
export RBN_DESCRIPTION := Example Test No Port: nameplate for testing without advertising a port

# Image Source Configuration
export RBN_SENTRY_REPO_FULL_NAME := ghcr.io/bhyslop/recipemuster
export RBN_BOTTLE_REPO_FULL_NAME := ghcr.io/bhyslop/recipemuster
export RBN_SENTRY_IMAGE_TAG      := bottle_ubuntu_test.20241203__154457

# TEMPORARY CUT
# export RBN_BOTTLE_IMAGE_TAG      := bottle_anthropic_jupyter.20241020__173503
export RBN_BOTTLE_IMAGE_TAG      := bottle_ubuntu_test.20241203__154457

# Port Service Configuration
export RBN_PORT_ENABLED            := 1
export RBN_ENTRY_PORT_WORKSTATION  := 8889
export RBN_ENTRY_PORT_ENCLAVE      := 8888
export RBN_PORT_SERVICE            := 8888

# Network Uplink Configuration
#    Worksheet: https://claude.ai/chat/3b81ecc4-c3bd-4e71-82af-4f0feec7ce97
#    Vexingly, example.com seems to have a large range.  I've experimentally
#    extended the cidr a number of times based off of nslookups at various
#    times.
export RBN_UPLINK_DNS_ENABLED     := 1
export RBN_UPLINK_ACCESS_ENABLED  := 1
export RBN_UPLINK_DNS_GLOBAL      := 0
export RBN_UPLINK_ACCESS_GLOBAL   := 0
export RBN_UPLINK_ALLOWED_DOMAINS := example.com
export RBN_UPLINK_ALLOWED_CIDRS   := 93.184.214.0/22

# Network Configuration
export RBN_ENCLAVE_NETWORK_BASE   := 172.16.0.0
export RBN_ENCLAVE_NETMASK        := 24
export RBN_ENCLAVE_INITIAL_IP     := 172.16.0.2
export RBN_ENCLAVE_SENTRY_IP      := 172.16.0.1


