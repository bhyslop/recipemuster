#!/bin/bash
# Launcher stub - delegates to CMK workbench
source "${BASH_SOURCE[0]%/*}/launcher_common.sh"
bud_launch "${BURC_TOOLS_DIR}/cmk/cmw_workbench.sh" "$@"
