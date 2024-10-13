#!/bin/sh
cd "$(dirname "$0")/.." &&  Tools/tabtarget-dispatch.sh 1 "$(basename "$0")" BGC_ARG_RECIPE="$1"
