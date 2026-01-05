#!/bin/bash
# TabTarget - JJT testbench all suites
exec "$(dirname "${BASH_SOURCE[0]}")/../.buk/launcher.jjt_testbench.sh" \
  "${0##*/}" all
