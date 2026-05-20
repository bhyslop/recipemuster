#!/bin/bash
# Launcher stub - delegates to CMK workbench
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/cmk/cmw_workbench.sh" "$@"
