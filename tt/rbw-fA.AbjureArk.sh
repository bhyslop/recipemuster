#!/bin/bash
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.rbk_Coordinator.sh" \
  "${0##*/}" "${@}"
