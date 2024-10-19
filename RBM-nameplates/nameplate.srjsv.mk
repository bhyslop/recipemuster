# © 2024 Scale Invariant.  All rights reserved.

# "Jupyter Notebook with Science and Visualization"

RBN_MONIKER = srjsv

RBN_SENTRY_IMAGE = ghcr.io/bhyslop/recipemuster:rogue_srjcl.20241019__024649
RBN_ROGUE_IMAGE  = ghcr.io/bhyslop/recipemuster:rogue_srjsv.20241019__033933


# NOT GOOD YET
RBN_IP_HACK = 10.241

# If the nameplate exports services through an application port, specify
RBN_PORT_ENABLED = 1
RBN_PORT_HOST    = 8900
RBN_PORT_GUARDED = 8901

# If the nameplate is allowed to reach out to the internet, specify
RBN_OUTREACH_ENABLED = 1
RBN_OUTREACH_CIDR    = 160.79.104.0/23
RBN_OUTREACH_DOMAIN  = anthropic.com

# Volume mounts and app directories: 
# Hmm, do I want a la carte or whole thing?
RBN_APP_OUTER_DIR = ./RBM-environments-${RBN_MONIKER}
RBN_APP_INNER_DIR = /mnt/rogue-data

# If an autostart command, specify
RBN_AUTOURL_ENABLED = 1
RBN_AUTOURL_URL     = http://${zMBSR_LOCALHOST_IP}:${RBN_PORT_HOST}/lab

# eof