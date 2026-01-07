#!/bin/bash
# Launcher stub - delegates to JJK workbench
source "${BASH_SOURCE[0]%/*}/launcher_common.sh"
bud_launch "${BURC_TOOLS_DIR}/jjk/jjw_workbench.sh" "$@"
