#!/bin/bash
# Launcher stub - delegates to RBW testbench
source "${BASH_SOURCE[0]%/*}/launcher_common.sh"
bud_launch "${BURC_TOOLS_DIR}/rbw/rbt_testbench.sh" "$@"
