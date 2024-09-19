#!/bin/sh
cd "$(dirname "$0")/.." &&  Tools/tabtarget-dispatch.sh 1 "$(basename "$0")" MBSR_ARG_MONIKER=srjcl
