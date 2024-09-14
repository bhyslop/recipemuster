# © 2024 Scale Invariant.  All rights reserved.
#
#  Beware this file may be used in makefiles as well as shell scripts, so:
#
#  * DO NOT USE SPACES AROUND '='
#  * USE ONLY ${xxx} VARIABLE EXPANSIOIN, NOT $xxx NOR $(xxx)
#

MBSR_NAMEPLATE_MONIKER=srjsv

# NOT GOOD YET
MBSR_NAMEPLATE_IP_HACK=10.241

# If the nameplate exports services through an application port, specify
MBSR_NAMEPLATE_PORT_ENABLED=1
MBSR_NAMEPLATE_PORT_HOST=8900
MBSR_NAMEPLATE_PORT_GUARDED=8901

# If the nameplate is allowed to reach out to the internet, specify
MBSR_NAMEPLATE_OUTREACH_ENABLED=1
MBSR_NAMEPLATE_OUTREACH_CIDR=160.79.104.0/23
MBSR_NAMEPLATE_OUTREACH_DOMAIN=anthropic.com

# Volume mounts and app directories: 
# Hmm, do I want a la carte or whole thing?
MBSR_NAMEPLATE_APP_OUTER_DIR=./MBSR-environments-${MBSR_NAMEPLATE_MONIKER}
MBSR_NAMEPLATE_APP_INNER_DIR=/mnt/rogue-data

# If an autostart command, specify
MBSR_NAMEPLACE_AUTOURL_ENABLED=1
MBSR_NAMEPLACE_AUTOURL_URL=http://${zMBSR_LOCALHOST_IP}:${MBSR_NAMEPLATE_PORT_HOST}/lab

# eof