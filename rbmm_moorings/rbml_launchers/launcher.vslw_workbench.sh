#!/bin/bash
# Launcher stub - delegates to VSLK workbench
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/vslk/vslw_workbench.sh" "$@"
