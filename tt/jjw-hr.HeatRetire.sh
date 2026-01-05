#!/bin/bash
# TabTarget - JJW workbench heat retire
exec "${BASH_SOURCE[0]%/*}/../.buk/launcher.jjw_workbench.sh" \
  "${0##*/}" "${@}"
