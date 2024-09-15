# Copyright 2024 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>


# View interim official at: https://github.com/bhyslop/recipemuster/rmc-console.mk
# View planner at: https://github.com/bhyslop/recipemuster/tree/main/PRIVATE_PLANNER

# Prefix used to distinguish commentary created by this makefile
zRMC_THIS_MAKEFILE = brm-console.mk

zRMC_TOOLS_DIR     = Tools

#########################
# Makefile Bash Console
#
# This is a sub makefile that contains several canned basic macros
# that help with very regular cons

zRMC_MBC_MAKEFILE = $(zRMC_TOOLS_DIR)/mbc.MakefileBashConsole.mk

# Configure the sub makefile
MBC_ARG__CONTEXT_STRING = $(zRMC_THIS_MAKEFILE)

include $(zRMC_MBC_MAKEFILE)

zRMC_START = $(MBC_SHOW_WHITE) "Rule $@: starting..."
zRMC_STEP  = $(MBC_SHOW_WHITE) "Rule $@:"
zRMC_PASS  = $(MBC_PASS)       "Rule $@: no errors."


# Use this to allow a console make target to explicitly trigger other
#   console make targets.  This is really only for making sure entire
#   rules function in 'big test cases': for efficiency, better to use
#   explicit fine grained make dependencies so that make can make it
#   efficient.
zRMC_MAKE = $(MAKE) -f $(zRMC_THIS_MAKEFILE)

default:
	$(MBC_SHOW_RED) "NO TARGET SPECIFIED.  Check" $(zRMC_TABTARGET_DIR) "directory for options." && $(MBC_FAIL)

zRMC_MBSR_MAKEFILE := $(zRMC_TOOLS_DIR)/mbsr.MakefileBashSentryRogue.mk

include $(zRMC_MBSR_MAKEFILE)


zRMC_MBSR_SUBMAKE = $(MAKE) -f $(zRMC_MBSR_MAKEFILE) MBSR_ARG_SUBMAKE_MBC=$(zRMC_MBC_MAKEFILE)

mbsr-A__BuildAndStartALL.sh:
	$(zRMC_STEP) "Assure podman services available..."
	podman machine start || echo "Podman probably running already, lets go on..."
	$(zRMC_STEP) "Use" $(zMBSR_MAKE) "to recurse..."
	$(zRMC_MBSR_SUBMAKE) mbsr-B__BuildImages.srjcl.sh  \
	                        MBSR_ARG_MONIKER=srjcl
	$(zRMC_MBSR_SUBMAKE) mbsr-B__BuildImages.srjsv.sh  \
	                        MBSR_ARG_MONIKER=srjsv
	$(zRMC_MBSR_SUBMAKE) mbsr-s__StartContainers.srjcl.sh  \
	                            MBSR_ARG_MONIKER=srjcl
	$(zRMC_MBSR_SUBMAKE) mbsr-s__StartContainers.srjsv.sh  \
	                            MBSR_ARG_MONIKER=srjsv
	$(MBC_PASS) "Done, no errors."


#######################################
#  Tabtarget Maintenance Tabtarget
#
#  Helps you create default form tabtargets in right place.

# Location for tabtargets relative to top level project directory
zRMC_TABTARGET_DIR  = ./tt

# Parameter from the tabtarget: what is the full name of the new tabtarget
RMC_TABTARGET_NAME = 

zRMC_TABTARGET_FILE = $(zRMC_TABTARGET_DIR)/$(MRM_TABTARGET_NAME)

ttc.CreateTabtarget.sh:
	@test -n "$(CPM_TABTARGET_NAME)" || { echo "Error: missing name param"; exit 1; }
	@echo '#!/bin/sh' > $(zRMC_TABTARGET_FILE)
	@echo 'cd "$$(dirname "$$0")/.." &&  Tools/tabtarget-dispatch.sh 1 "$$(basename "$$0")"' \
	                 >> $(zRMC_TABTARGET_FILE)
	@chmod +x           $(zRMC_TABTARGET_FILE)
	$(zRMC_PASS)


#######################################
#  Slickedit Project Tabtarget
#
#  Due to filesystem handle entanglements, Slickedit doesn't play well
#  with git.  This tabtarget places a usable copy in a .gitignored location

zRMC_SLICKEDIT_PROJECT_DIR = ./_slickedit

vsr.ReplaceSlickEditWorkspace.sh:
	mkdir -p                                             $(zRMC_SLICKEDIT_PROJECT_DIR)
	-rm -rf                                              $(zRMC_SLICKEDIT_PROJECT_DIR)/*
	cp $(zRMC_TOOLS_DIR)/vsep_VisualSlickEditProject/* $(zRMC_SLICKEDIT_PROJECT_DIR)
	$(zRMC_PASS)


# EOF
