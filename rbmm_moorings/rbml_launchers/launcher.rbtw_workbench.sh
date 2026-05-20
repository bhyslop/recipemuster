#!/bin/bash
# Launcher stub - delegates to RBTW workbench (theurge Rust build/test)
source "Tools/buk/bul_launcher.sh"
bul_launch "${BURC_TOOLS_DIR}/rbk/rbtd/rbtw_workbench.sh" "$@"
