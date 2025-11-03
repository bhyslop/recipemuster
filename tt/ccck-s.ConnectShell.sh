#!/bin/bash
export BDU_INTERACTIVE=1
exec "${0%/*}/../.buk/launcher.cccw_workbench.sh" "$0" "$@"