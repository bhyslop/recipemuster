#!/bin/bash
# TabTarget - delegates to buk via launcher
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.rbk_Coordinator.sh" \
  "${0##*/}" "${@}"
