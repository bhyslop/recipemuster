#!/bin/bash
export BURD_LAUNCHER=launcher.study_workbench.sh
exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" "${0##*/}" "${@}"
