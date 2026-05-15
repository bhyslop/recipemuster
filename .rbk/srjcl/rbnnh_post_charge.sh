#!/usr/bin/env bash
#
# srjcl post-charge hook
#
# Emits the JupyterLab URL and warns if the operator's shell did not export
# ANTHROPIC_API_KEY (in which case notebook cells that call the Anthropic API
# will fail authentication).
#
# Self-contained by contract: no BUK/RBK utility-module dependencies. Inherits
# RBRN_*/RBRR_* environment from the charge process.

set -eu

readonly z_yellow=$'\033[33m'
readonly z_bold=$'\033[1m'
readonly z_reset=$'\033[0m'

readonly z_url="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/lab"
printf '%sJupyterLab ready:%s %s\n' "${z_bold}" "${z_reset}" "${z_url}"

if test -z "${ANTHROPIC_API_KEY:-}"; then
  printf '%sWARNING:%s ANTHROPIC_API_KEY is not set in your shell — Anthropic API calls in the notebook will fail.\n' "${z_yellow}" "${z_reset}"
  printf '  Remediation: %sexport ANTHROPIC_API_KEY=sk-...%s in your shell, then re-charge srjcl to pick it up.\n' "${z_bold}" "${z_reset}"
fi
