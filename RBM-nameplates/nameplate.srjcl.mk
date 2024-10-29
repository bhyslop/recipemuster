# Recipe Bottle Service Assignment Makefile
# Jupyter Notebook with Claude Access

# Core Configuration
RBN_MONIKER := srjcl
RBN_DESCRIPTION := Jupyter Notebook environment with Claude API access for AI-assisted development

# Image Configuration
RBN_SENTRY_REPO_FULL_NAME := ghcr.io/bhyslop/recipemuster
RBN_BOTTLE_REPO_FULL_NAME := ghcr.io/bhyslop/recipemuster
RBN_SENTRY_IMAGE_TAG := sentry_alpine.20241020__171441
RBN_BOTTLE_IMAGE_TAG := bottle_anthropic_jupyter.20241020__173503

# Network Configuration
RBN_GUARDED_NETWORK_ID := 10.240

# Port Service Configuration
RBN_PORT_ENABLED := 1
RBN_PORT_HOST := 8889
RBN_PORT_GUARDED := 8888

# Internet Outreach Configuration
RBN_PORT_ENABLED := 1
RBN_OUTREACH_CIDR := 160.79.104.0/23
RBN_OUTREACH_DOMAIN := anthropic.com

# Volume Mount Configuration
RBN_VOLUME_MOUNTS := -v ./RBM-environments-srjcl:/mnt/bottle-data:Z

# Auto-start Configuration
RBN_AUTOURL_ENABLED := 1
RBN_AUTOURL_URL := http://127.0.0.1:$(RBN_PORT_HOST)/lab
