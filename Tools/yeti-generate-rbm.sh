#!/usr/bin/env bash
#
# YETI Generator: RBM Sample Project
#
# Defines cohorts and spaces for the RBM sample and emits all IDE configs

set -euo pipefail

# Load the YETI builder
source "$(dirname "${BASH_SOURCE[0]}")/yeti/builder.sh"

# --- cohort definitions ---
yeti_cohort all "Unifferentiated Files"
yeti_add_deep_suffix all . .sh .mk
yeti_add_deep_suffix all RBM-recipes .recipe

# --- space definition ---
yeti_space ALL vsusi all

# --- generate everything ---
yeti_generate ../vswb_test . vsrbm ../brm_recipemuster

echo "Done."

