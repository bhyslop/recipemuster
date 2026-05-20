#!/bin/bash
# Launcher stub - delegates to BUK workbench
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/buk/buw_workbench.sh" "$@"
