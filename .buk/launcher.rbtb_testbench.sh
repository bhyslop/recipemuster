#!/bin/bash
# Launcher stub - delegates to RBTB testbench
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/rbw/rbtb_testbench.sh" "$@"
