#!/bin/bash
# Launcher stub - delegates to VSLK workbench
source "${BASH_SOURCE[0]%/*}/launcher_common.sh"
bud_launch "${BURC_TOOLS_DIR}/vslk/vslw_workbench.sh" "$@"
