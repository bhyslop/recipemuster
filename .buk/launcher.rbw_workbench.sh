#!/bin/bash
# Launcher stub - delegates to RBW workbench
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/rbw/rbw_workbench.sh" "$@"
