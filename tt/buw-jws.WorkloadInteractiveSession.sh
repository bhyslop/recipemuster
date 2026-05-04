#!/bin/bash
export BURD_LAUNCHER=".buk/launcher.buw_workbench.sh"
export BURD_INTERACTIVE=1
exec "${BASH_SOURCE[0]%/*}/../${BURD_LAUNCHER}" "${0##*/}" "${@}"
