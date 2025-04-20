#!/bin/bash

set -e

source "$(dirname "${BASH_SOURCE[0]}")/vswb.builder.sh"

vswb_define_project    all           "Unifferentiated Files"
vswb_add_files_recurse all .                 "*.sh"
vswb_add_files_recurse all .                 "*.mk"
vswb_add_files_recurse all RBM-recipes       "*.recipe"

vswb_init_workspace ALL vsusi all

vswb_generate_files ../vswb_test .. brm_recipemuster vsrbm


