#!/bin/bash
# Launcher stub - delegates to vow workbench
source "${BASH_SOURCE[0]%/*}/launcher_common.sh"
bud_launch "${BURC_TOOLS_DIR}/vok/vow_workbench.sh" "$@"
