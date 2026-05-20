#!/bin/bash
export BURD_INTERACTIVE=1
exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" rbml_rbw "${0##*/}" "${@}"
