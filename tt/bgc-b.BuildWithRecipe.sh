#!/bin/sh
cd "$(dirname "$0")/.." &&  Tools/mbd.dispatch.sh jp_single om_line "$(basename "$0")" \
                              BGC_ARG_RECIPE="$1"       \
                              BGC_ARG_FQIN_OUTPUT="$2"
