#!/bin/bash
# Launcher stub - delegates to study workbench
source "${BASH_SOURCE[0]%/*}/../Tools/buk/bul_launcher.sh"
bul_launch "Study/study_workbench.sh" "$@"
