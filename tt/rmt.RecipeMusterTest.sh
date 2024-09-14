#!/bin/sh
cd "$(dirname "$0")/.." && echo "CURRENT DIR IS `pwd` and base is `basename $0`" && Tools/tabtarget-dispatch.sh 1 "$(basename "$0")"
