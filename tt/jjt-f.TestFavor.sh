#!/bin/bash
# TabTarget - JJT testbench favor suite
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.jjt_testbench.sh" \
  "${0##*/}" favor
