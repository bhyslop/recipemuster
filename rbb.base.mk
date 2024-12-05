# Base Configuration Makefile
# This file should be placed in Tools/rbb.BaseConfigRegime.mk and customized

# Network Configuration
# Example: Use a unique subnet for bottle services that won't conflict with local networks
export RBB_ENCLAVE_SUBNET         := 172.16.0.0/24
export RBB_ENCLAVE_PRIMAL_GATEWAY := 172.16.0.1
export RBB_ENCLAVE_SENTRY_GATEWAY := 172.16.0.2

# DNS Configuration
# Example: Use Google's public DNS, or specify your preferred DNS server
export RBB_DNS_SERVER         := 8.8.8.8

# Path Configuration
# Location for nameplate configuration files
export RBB_NAMEPLATE_PATH     := RBM-nameplates

# Container Registry Configuration
# Specify your container registry server (optional)
# Example for GitHub Container Registry:
export RBB_REGISTRY_SERVER    := ghcr.io

