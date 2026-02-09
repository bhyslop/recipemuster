#!/bin/bash
# Launcher stub - delegates to RBTG testbench
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/rbw/rbtg_testbench.sh" "$@"
