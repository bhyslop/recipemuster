#!/bin/sh
cd "$(dirname "$0")/.." &&  Tools/mbd.dispatch.sh jp_bounded om_line "$(basename "$0")"
