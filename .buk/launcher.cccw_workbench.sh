#!/bin/bash
# Launcher stub - delegates to CCCK workbench
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/ccck/cccw_workbench.sh" "$@"
