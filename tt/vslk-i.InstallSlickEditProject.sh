#!/bin/bash
# TabTarget - delegates to vslw workbench via launcher
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.vslw_workbench.sh" \
  "${0##*/}" "${@}"
