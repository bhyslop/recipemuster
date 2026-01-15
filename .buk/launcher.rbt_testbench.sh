#!/bin/bash
# Launcher stub - delegates to RBW testbench
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/rbw/rbt_testbench.sh" "$@"
