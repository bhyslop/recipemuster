#!/bin/bash
# TabTarget - JJT testbench studbook suite
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.jjt_testbench.sh" \
  "${0##*/}" studbook
