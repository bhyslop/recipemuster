#!/bin/bash
export BURD_LAUNCHER=launcher.rbthw_workbench.sh
exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" "${0##*/}" "${@}"
