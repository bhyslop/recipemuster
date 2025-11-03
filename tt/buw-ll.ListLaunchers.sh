#!/bin/bash
# Generated tabtarget - delegates to buw workbench
exec "$(dirname "${BASH_SOURCE[0]}")/../Tools/buw/buw_workbench.sh" "buw-ll" "${@}"
