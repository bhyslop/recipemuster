#!/bin/bash
# Generated tabtarget - delegates to rbw workbench via BUD launcher
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.rbw_workbench.sh" "rbw-s" "nsproto" "${@}"
