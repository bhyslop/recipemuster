#!/bin/bash
# Launcher stub - delegates to JJK testbench
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/jjk/jjt_testbench.sh" "$@"
