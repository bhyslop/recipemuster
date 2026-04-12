#!/bin/bash
export BURD_LAUNCHER=".buk/launcher.buw_workbench.sh"
exec "${BASH_SOURCE[0]%/*}/../${BURD_LAUNCHER}" "${0##*/}" "${@}"
