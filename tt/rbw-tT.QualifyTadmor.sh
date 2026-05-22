#!/bin/bash
exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" rbml_rbw "${0##*/}" "${@}"
