#!/bin/bash
# Launcher stub - delegates to RBW workbench
source "${BASH_SOURCE[0]%/*}/launcher_common.sh"
bud_launch "${BURC_TOOLS_DIR}/rbw/rbw_workbench.sh" "$@"
