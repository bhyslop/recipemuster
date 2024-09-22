# © 2024 Scale Invariant.  All rights reserved.
#
#  Beware this file may be used in makefiles as well as shell scripts, so:
#

# Python Github Maintenance

RBN_MONIKER=pyghm

# NOT GOOD YET
RBN_IP_HACK=10.242

# If the nameplate exports services through an application port, specify
RBN_PORT_ENABLED=0
RBN_PORT_HOST=8000
RBN_PORT_GUARDED=8000

# If the nameplate is allowed to reach out to the internet, specify
# OUCH CHANGE THIS!!!  Expedient to try and viz stuff now
RBN_OUTREACH_ENABLED=0

# Pretty unsafe thing, but needed for bootstrap
RBN_WIDEREACH_ENABLED=1
RBN_WIDEREACH_DOMAIN=github.com

# Volume mounts and app directories:
# Hmm, do I want a la carte or whole thing?
RBN_APP_OUTER_DIR=./MBSR-environments-${RBN_MONIKER}
RBN_APP_INNER_DIR=/mnt/rogue-data

# If an autostart command, specify
RBN_AUTOURL_ENABLED=0


# eof