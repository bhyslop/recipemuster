#!/bin/bash
export BURD_LAUNCHER=".buk/launcher.rbw_workbench.sh"
export BURD_NO_LOG=1
export BURD_INTERACTIVE=1
exec "${BASH_SOURCE[0]%/*}/../${BURD_LAUNCHER}" "${0##*/}" "${@}"
