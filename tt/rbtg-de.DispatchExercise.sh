#!/bin/bash
# Generated tabtarget - delegates to rbtg testbench via BUD launcher
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.rbtg_testbench.sh" "rbtg-de" "${@}"
