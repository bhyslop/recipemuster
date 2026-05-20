#!/bin/bash
# Launcher stub - delegates to JJK workbench
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/jjk/jjw_workbench.sh" "$@"
