# Recipe Bottle Nameplate Configuration
# Network namespace prototype testing environment

# Core Service Definition
RBRN_MONIKER     = nsproto
RBRN_DESCRIPTION = Network namespace prototype testing environment for container networking

# Image Source Configuration
RBRN_SENTRY_REPO_PATH = ghcr.io/bhyslop/recipemuster
RBRN_BOTTLE_REPO_PATH = ghcr.io/bhyslop/recipemuster
RBRN_SENTRY_IMAGE_TAG = sentry_ubuntu_large.20241022__130547
RBRN_BOTTLE_IMAGE_TAG = bottle_ubuntu_test.20241207__190758

# Port Service Configuration
RBRN_ENTRY_ENABLED          = 1
RBRN_ENTRY_PORT_WORKSTATION = 8890
RBRN_ENTRY_PORT_ENCLAVE     = 8888

# Network Uplink Configuration
RBRN_UPLINK_PORT_MIN        = 10000
RBRN_UPLINK_DNS_ENABLED     = 1
RBRN_UPLINK_ACCESS_ENABLED  = 1
RBRN_UPLINK_DNS_GLOBAL      = 0
RBRN_UPLINK_ACCESS_GLOBAL   = 0
RBRN_UPLINK_ALLOWED_CIDRS   = 160.79.104.0/23
RBRN_UPLINK_ALLOWED_DOMAINS = anthropic.com

# Network Configuration
RBRN_ENCLAVE_BASE_IP    = 10.242.0.0
RBRN_ENCLAVE_NETMASK    = 24
RBRN_ENCLAVE_SENTRY_IP  = 10.242.0.2
RBRN_ENCLAVE_BOTTLE_IP  = 10.242.0.3

# Volume Mount Configuratin
RBRN_VOLUME_MOUNTS =

