#!/usr/bin/env bash
#
# nineveh post-charge hook
#
# Prints kroki-core render usage — the core engine paths and a sample POST — so
# the operator can drive the diagram-render trial by hand.
#
# Self-contained by contract: no BUK/RBK utility-module dependencies. Inherits
# RBRN_*/RBRR_* environment from the charge process.

set -eu

readonly z_bold=$'\033[1m'
readonly z_reset=$'\033[0m'

readonly z_base="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}"

printf '%sKroki-core ready:%s %s\n' "${z_bold}" "${z_reset}" "${z_base}"
printf 'Core engines: graphviz, plantuml, d2 — POST diagram source to /<engine>/svg\n'
printf 'Sample (graphviz):\n'
printf "  echo 'digraph { a -> b }' | curl -s --data-binary @- %s/graphviz/svg\n" "${z_base}"
