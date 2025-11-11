#!/bin/bash
export BUD_INTERACTIVE=1
exec "${0%/*}/../.buk/launcher.cccw_workbench.sh" "$0" "$@"