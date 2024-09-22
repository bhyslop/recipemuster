# © 2024 Scale Invariant.  All rights reserved.
#
#  Beware this file may be used in makefiles as well as shell scripts, so:
#
#  * DO NOT USE SPACES AROUND '='
#  * USE ONLY ${xxx} VARIABLE EXPANSIOIN, NOT $xxx NOR $(xxx)
#

RBN_MONIKER=srjcl

# NOT GOOD YET
RBN_IP_HACK=10.240

# If the nameplate exports services through an application port, specify
RBN_PORT_ENABLED=1
RBN_PORT_HOST=8889
RBN_PORT_GUARDED=8888

# If the nameplate is allowed to reach out to the internet, specify
# OUCH CHANGE THIS!!!  Expedient to try and viz stuff now
RBN_OUTREACH_ENABLED=1
RBN_OUTREACH_CIDR=160.79.104.0/23
RBN_OUTREACH_DOMAIN=anthropic.com

# Volume mounts and app directories: 
# Hmm, do I want a la carte or whole thing?
RBN_APP_OUTER_DIR=./MBSR-environments-srjcl
RBN_APP_INNER_DIR=/mnt/rogue-data

# If an autostart command, specify
RBN_AUTOURL_ENABLED=1
RBN_AUTOURL_URL=http://${zMBSR_LOCALHOST_IP}:${RBN_PORT_HOST}/lab

# eof