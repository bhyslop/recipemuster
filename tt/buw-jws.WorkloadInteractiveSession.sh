#!/bin/bash
export BURD_LAUNCHER=launcher.buw_workbench.sh
export BURD_INTERACTIVE=1
exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" "${0##*/}" "${@}"
