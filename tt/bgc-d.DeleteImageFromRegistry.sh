#!/bin/sh
cd "$(dirname "$0")/.." &&  Tools/mbd.dispatch.sh jp_single om_line "$(basename "$0")" BGC_ARG_TAG="$1"
