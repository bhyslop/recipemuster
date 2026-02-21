#!/bin/bash
export BURD_LAUNCHER=".buk/launcher.study_workbench.sh"
exec "${BASH_SOURCE[0]%/*}/../${BURD_LAUNCHER}" "${0##*/}" "${@}"
