#!/bin/bash
# Launcher stub - delegates to CCCK workbench
source "${BASH_SOURCE[0]%/*}/launcher_common.sh"
bud_launch "${BURC_TOOLS_DIR}/ccck/cccw_workbench.sh" "$@"
