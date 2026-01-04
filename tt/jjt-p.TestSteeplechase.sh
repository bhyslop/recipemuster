#!/bin/bash
# TabTarget - JJT testbench steeplechase suite
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.jjt_testbench.sh" \
  "${0##*/}" steeple
