# Base Configuration Makefile
# This file should be placed in Tools/rbb.BaseConfigRegime.mk and customized

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

