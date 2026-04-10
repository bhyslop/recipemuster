#!/bin/bash
# Launcher stub - delegates to APCW workbench
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/apck/apcw_workbench.sh" "$@"
