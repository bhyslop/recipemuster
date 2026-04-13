#!/bin/bash
# No-log launcher stub - delegates to RBW workbench without station file
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_nolog_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/rbk/rbw_workbench.sh" "$@"
