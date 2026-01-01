#!/bin/bash
# Generated tabtarget - delegates to rbt testbench via BUD launcher
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.rbt_testbench.sh" "rbt-to" "nsproto" "${@}"
