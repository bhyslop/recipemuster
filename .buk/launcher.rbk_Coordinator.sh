#!/bin/bash
# Launcher stub - delegates to RBW coordinator
source "${BASH_SOURCE[0]%/*}/launcher_common.sh"
bud_launch "${BURC_TOOLS_DIR}/rbw/rbk_Coordinator.sh" "$@"
