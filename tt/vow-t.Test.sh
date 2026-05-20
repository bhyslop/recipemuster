#!/bin/bash
exec "${BASH_SOURCE[0]%/*}/z-launcher.sh" buml_vow "${0##*/}" "${@}"
