#!/bin/sh
cd "$(dirname "$0")/.." &&  Tools/tabtarget-dispatch.sh 1 "$(basename "$0")" RMC_TABTARGET_NAME="$1"
