#!/bin/bash
# Launcher stub - delegates to CMK workbench
source "Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/cmk/cmw_workbench.sh" "$@"
