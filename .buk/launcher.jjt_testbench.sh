#!/bin/bash
# Launcher stub - delegates to JJK testbench
source "${BASH_SOURCE[0]%/*}/launcher_common.sh"
bud_launch "${BURC_TOOLS_DIR}/jjk/jjt_testbench.sh" "$@"
