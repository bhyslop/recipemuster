#!/bin/bash
# Launcher stub - delegates to vow workbench
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/vok/vow_workbench.sh" "$@"
