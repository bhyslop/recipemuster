# Recipe Bottle Nameplate Configuration
# Network namespace prototype testing environment

# Core Service Definition
export RBN_MONIKER     := nsproto
export RBN_DESCRIPTION := Network namespace prototype testing environment for container networking

# Image Source Configuration
export RBN_SENTRY_REPO_PATH := ghcr.io/bhyslop/recipemuster
export RBN_BOTTLE_REPO_PATH := ghcr.io/bhyslop/recipemuster
export RBN_SENTRY_IMAGE_TAG := sentry_ubuntu_large.20241022__130547
export RBN_BOTTLE_IMAGE_TAG := bottle_ubuntu_test.20241207__190758

# Port Service Configuration
export RBN_PORT_ENABLED           := 1
export RBN_ENTRY_PORT_WORKSTATION := 8890
export RBN_ENTRY_PORT_ENCLAVE     := 8888

# Network Uplink Configuration
export RBN_UPLINK_DNS_ENABLED     := 1
export RBN_UPLINK_ACCESS_ENABLED  := 1
export RBN_UPLINK_DNS_GLOBAL      := 1
export RBN_UPLINK_ACCESS_GLOBAL   := 1
export RBN_UPLINK_ALLOWED_CIDRS   := 160.79.104.0/23
export RBN_UPLINK_ALLOWED_DOMAINS := anthropic.com

# Network Configuration
export RBN_ENCLAVE_BASE_IP    := 10.242.0.0
export RBN_ENCLAVE_NETMASK    := 24
export RBN_ENCLAVE_INITIAL_IP := 10.242.0.3
export RBN_ENCLAVE_SENTRY_IP  := 10.242.0.2

# Volume Mount Configuration
export RBN_VOLUME_MOUNTS :=

