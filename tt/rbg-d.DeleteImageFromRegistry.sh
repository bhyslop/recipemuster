#!/bin/sh
cd "$(dirname "$0")/.." &&  Tools/mbd.dispatch.sh jp_single om_line "$(basename "$0")" RGB_ARG_FQIN="$1" "$2"
