#!/bin/bash
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.jjw_workbench.sh" \
  "${0##*/}" "${@}"
