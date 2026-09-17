#!/bin/bash
export BURD_LAUNCHER=launcher.bkw_workbench.sh
export BURD_AMANUENSIS=1
exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" "${0##*/}" "${@}"
