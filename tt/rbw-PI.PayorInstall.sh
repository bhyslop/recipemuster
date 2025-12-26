#!/bin/bash
export BUD_INTERACTIVE=1
export BUD_NO_LOG=1
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.rbk_Coordinator.sh" \
  "${0##*/}" "${@}"