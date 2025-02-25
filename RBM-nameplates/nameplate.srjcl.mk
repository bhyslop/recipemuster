# Recipe Bottle Nameplate Configuration
# Jupyter Notebook environment with Claude API access

# Core Service Definition
RBN_MONIKER     = srjcl
RBN_DESCRIPTION = Jupyter Notebook environment with Claude API access for AI-assisted development

# Image Source Configuration
RBN_SENTRY_REPO_PATH = ghcr.io/bhyslop/recipemuster
RBN_BOTTLE_REPO_PATH = ghcr.io/bhyslop/recipemuster
RBN_SENTRY_IMAGE_TAG = sentry_ubuntu_large.20250211__153900
RBN_BOTTLE_IMAGE_TAG = bottle_anthropic_jupyter.20250130__152820

# Port Service Configuration
RBN_ENTRY_ENABLED          = 1
RBN_ENTRY_PORT_WORKSTATION = 7999
RBN_ENTRY_PORT_ENCLAVE     = 8000

# Network Uplink Configuration
RBN_UPLINK_PORT_MIN        = 10000
RBN_UPLINK_DNS_ENABLED     = 1
RBN_UPLINK_ACCESS_ENABLED  = 1
RBN_UPLINK_DNS_GLOBAL      = 0
RBN_UPLINK_ACCESS_GLOBAL   = 0
RBN_UPLINK_ALLOWED_CIDRS   = 160.79.104.0/23
RBN_UPLINK_ALLOWED_DOMAINS = anthropic.com

# Network Configuration
RBN_ENCLAVE_BASE_IP    = 10.242.0.0
RBN_ENCLAVE_NETMASK    = 24
RBN_ENCLAVE_SENTRY_IP  = 10.242.0.2
RBN_ENCLAVE_BOTTLE_IP  = 10.242.0.3

# Volume Mount Configuration
RBN_VOLUME_MOUNTS = -v ./RBM-environments-srjcl:/workspace:Z
