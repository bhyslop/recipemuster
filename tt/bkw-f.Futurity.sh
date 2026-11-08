#!/bin/bash
set -- 'launcher.bkw_workbench.sh buuc_ownlog' "$@"
exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" "${0##*/}" "${@}"
