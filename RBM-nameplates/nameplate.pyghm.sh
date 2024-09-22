# © 2024 Scale Invariant.  All rights reserved.
#
#  Beware this file may be used in makefiles as well as shell scripts, so:
#

# Python Github Maintenance

MBSR_NAMEPLATE_MONIKER=pyghm

# NOT GOOD YET
MBSR_NAMEPLATE_IP_HACK=10.242

# If the nameplate exports services through an application port, specify
MBSR_NAMEPLATE_PORT_ENABLED=0
MBSR_NAMEPLATE_PORT_HOST=8000
MBSR_NAMEPLATE_PORT_GUARDED=8000

# If the nameplate is allowed to reach out to the internet, specify
# OUCH CHANGE THIS!!!  Expedient to try and viz stuff now
MBSR_NAMEPLATE_OUTREACH_ENABLED=0

# Pretty unsafe thing, but needed for bootstrap
MBSR_NAMEPLATE_WIDEREACH_ENABLED=1
MBSR_NAMEPLATE_WIDEREACH_DOMAIN=github.com

# Volume mounts and app directories:
# Hmm, do I want a la carte or whole thing?
MBSR_NAMEPLATE_APP_OUTER_DIR=./MBSR-environments-${MBSR_NAMEPLATE_MONIKER}
MBSR_NAMEPLATE_APP_INNER_DIR=/mnt/rogue-data

# If an autostart command, specify
MBSR_NAMEPLACE_AUTOURL_ENABLED=0


# eof