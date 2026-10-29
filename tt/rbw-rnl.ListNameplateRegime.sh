#!/bin/bash
set -- 'launcher.rbw_workbench.sh buuc_nolog' "$@"
exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" "${0##*/}" "${@}"
