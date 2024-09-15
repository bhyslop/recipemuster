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


# View interim official at: https://github.com/bhyslop/recipemuster/brm-console.mk

# Prefix used to distinguish commentary created by this makefile
zMRM_THIS_MAKEFILE = brm-console.mk

zMRM_TOOLS_DIR     = Tools

#########################
# Makefile Bash Console
#
# This is a sub makefile that contains several canned basic macros
# for pretty printing and similar.

zMRM_MBC_MAKEFILE = $(zMRM_TOOLS_DIR)/mbc.MakefileBashConsole.mk

# Configure the sub makefile
MBC_ARG__CONTEXT_STRING = $(zMRM_THIS_MAKEFILE)

include $(zMRM_MBC_MAKEFILE)

zMRM_START = $(MBC_SHOW_WHITE) "Rule $@: starting..."
zMRM_STEP  = $(MBC_SHOW_WHITE) "Rule $@:"
zMRM_PASS  = $(MBC_PASS)       "Rule $@: no errors."


# Use this to allow a console make target to explicitly trigger other
#   console make targets.  This is really only for making sure entire
#   rules function in 'big test cases': for efficiency, better to use
#   explicit fine grained make dependencies so that make can make it
#   efficient.
zMRM_MAKE = $(MAKE) -f $(zMRM_THIS_MAKEFILE)

default:
	$(MBC_SHOW_RED) "NO TARGET SPECIFIED.  Check" $(zMRM_TABTARGET_DIR) "directory for options." && $(MBC_FAIL)

zCPM_MBSR_MAKEFILE := $(zMRM_TOOLS_DIR)/mbsr.MakefileBashSentryRogue.mk

include $(zCPM_MBSR_MAKEFILE)

zcpm_empty =

zCPM_MBSR_SUBMAKE = $(MAKE) -f $(zCPM_MBSR_MAKEFILE) MBSR_ARG_SUBMAKE_MBC=$(zMRM_MBC_MAKEFILE)

mbsr-A__BuildAndStartALL.sh:
	$(zMRM_STEP) "Assure podman services available..."
	podman machine start || echo "Podman probably running already, lets go on..."
	$(zMRM_STEP) "Use" $(zMBSR_MAKE) "to recurse..."
	$(zCPM_MBSR_SUBMAKE) mbsr-B__BuildImages.srjcl.sh  \
	                        MBSR_ARG_MONIKER=srjcl
	$(zCPM_MBSR_SUBMAKE) mbsr-B__BuildImages.srjsv.sh  \
	                        MBSR_ARG_MONIKER=srjsv
	$(zCPM_MBSR_SUBMAKE) mbsr-s__StartContainers.srjcl.sh  \
	                            MBSR_ARG_MONIKER=srjcl
	$(zCPM_MBSR_SUBMAKE) mbsr-s__StartContainers.srjsv.sh  \
	                            MBSR_ARG_MONIKER=srjsv
	$(MBC_PASS) "Done, no errors."


#######################################
#  Tabtarget Maintenance Tabtarget
#
#  Helps you create default form tabtargets in right place.

# Location for tabtargets relative to top level project directory
zMRM_TABTARGET_DIR  = ./tt

# Parameter from the tabtarget: what is the full name of the new tabtarget
MRM_TABTARGET_NAME = 

zMRM_TABTARGET_FILE = $(zMRM_TABTARGET_DIR)/$(MRM_TABTARGET_NAME)

ttm.CreateTabtarget.sh:
	@test -n "$(CPM_TABTARGET_NAME)" || { echo "Error: missing name param"; exit 1; }
	@echo '#!/bin/sh' > $(zMRM_TABTARGET_FILE)
	@echo 'cd "$$(dirname "$$0")/.." &&  Tools/tabtarget-dispatch.sh 1 "$$(basename "$$0")"' \
	                 >> $(zMRM_TABTARGET_FILE)
	@chmod +x           $(zMRM_TABTARGET_FILE)
	$(zMRM_PASS)


#######################################
#  Slickedit Project Tabtarget
#
#  Due to filesystem handle entanglements, Slickedit doesn't play well
#  with git.  This tabtarget places a usable copy in a .gitignored location

zMRM_SLICKEDIT_PROJECT_DIR = ./_slickedit

vsr.ReplaceSlickEditWorkspace.sh:
	mkdir -p                                             $(zMRM_SLICKEDIT_PROJECT_DIR)
	-rm -rf                                              $(zMRM_SLICKEDIT_PROJECT_DIR)/*
	cp $(zMRM_TOOLS_DIR)/vsep_VisualSlickEditProject/* $(zMRM_SLICKEDIT_PROJECT_DIR)
	$(zMRM_PASS)


# EOF
