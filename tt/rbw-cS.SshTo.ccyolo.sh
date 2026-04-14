#!/bin/bash
# SSH into ccyolo bottle
source "${BASH_SOURCE[0]%/*}/../.rbk/ccyolo/rbrn.env"
exec ssh -p "${RBRN_ENTRY_PORT_WORKSTATION}" \
  -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
  claude@localhost
