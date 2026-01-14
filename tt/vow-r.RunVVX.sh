#!/bin/bash
# TabTarget - delegates to vow workbench via launcher
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.vow_workbench.sh" \
  "${0##*/}" "${@}"
