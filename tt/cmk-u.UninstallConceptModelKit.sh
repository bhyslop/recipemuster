#!/bin/bash
# TabTarget - delegates to cmw workbench via launcher
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.cmw_workbench.sh" \
  "${0##*/}" "${@}"
