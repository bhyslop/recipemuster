#!/bin/bash
export BDU_INTERACTIVE=1
exec "${0%/*}/../.buk/buk_launch_ccck.sh" "$0" "$@"