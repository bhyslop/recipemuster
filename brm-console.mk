## Copyright Scale Invariant, Inc - All Rights Reserved
##
## Unauthorized copying of this file, via any medium is strictly prohibited
## Proprietary and confidential
##
## Written by Brad Hyslop <bhyslop@scaleinvariant.org> September 2024


# View interim official at: https://github.com/bhyslop/recipemuster/brm-console.mk

# Prefix used to distinguish commentary created by this makefile
zCPM_SELF = brm-console.mk

CPM_TOOLS_RELDIR      = Tools
zCPM_SUBMAKE_MBC_VARS = $(CPM_TOOLS_RELDIR)/mbc.MakefileBashConsole.variables.mk

zSSIMK_REPONAME        = $(shell basename $(shell pwd))


MBC_ARG__CONTEXT_STRING = $(zCPM_SELF)

# Common utilities for tabtarget implementation including console colors
include $(zCPM_SUBMAKE_MBC_VARS)

zCPM_TABTARGET_DIR  = tt

zCPM_START = $(MBC_SHOW_WHITE) "Rule $@: starting..."
zCPM_PASS  = $(MBC_PASS)       "Rule $@: no errors."


# Use this to allow a console make target to explicitly trigger other
#   console make targets.  This is really only for making sure entire
#   rules function in 'big test cases': for efficiency, better to use
#   explicit fine grained make dependencies so that make can make it
#   efficient.
# ITCH_nestBashColorTrick == z when recurse, use less green for aggregated substeps
zCPM_CONSOLE_MAKE = $(MAKE) -f $(zCPM_SELF)

default:
	$(MBC_SHOW_RED) "NO TARGET SPECIFIED.  Check" $(zCPM_TABTARGET_DIR) "directory for options." && $(MBC_FAIL)

zCPM_MBSR_MAKEFILE := $(CPM_TOOLS_RELDIR)/mbsr.MakefileBashSentryRogue.mk

include $(zCPM_MBSR_MAKEFILE)

zcpm_empty =

zCPM_MBSR_SUBMAKE = $(MAKE) -f $(zCPM_MBSR_MAKEFILE) MBSR_ARG_SUBMAKE_MBC=$(zCPM_SUBMAKE_MBC_VARS)

mbsr-A__BuildAndStartALL.sh:
	$(zMBSR_STEP) "Assure podman services available..."
	podman machine start || echo "Podman probably running already, lets go on..."
	$(zMBSR_STEP) "Use" $(zMBSR_MAKE) "to recurse..."
	$(zCPM_MBSR_SUBMAKE) mbsr-B__BuildImages.srjcl.sh  \
	                        MBSR_ARG_MONIKER=srjcl
	$(zCPM_MBSR_SUBMAKE) mbsr-B__BuildImages.srjsv.sh  \
	                        MBSR_ARG_MONIKER=srjsv
	$(zCPM_MBSR_SUBMAKE) mbsr-s__StartContainers.srjcl.sh  \
	                            MBSR_ARG_MONIKER=srjcl
	$(zCPM_MBSR_SUBMAKE) mbsr-s__StartContainers.srjsv.sh  \
	                            MBSR_ARG_MONIKER=srjsv
	$(MBC_PASS) "Done, no errors."


rmt.RecipeMusterTest.sh:
	$(MBC_PASS) "Done, no errors."


#######################################
#  Tabtarget Maintenance Tabtarget
#
#  Helps you create default form tabtargets in right place.

# Parameter from the tabtarget: what is the full name of the new tabtarget
MRM_TABTARGET_NAME = 

zMRM_TABTARGET_FILE = $(zCPM_TABTARGET_DIR)/$(MRM_TABTARGET_NAME)

ttm.CreateTabtarget.sh:
	@test -n "$(CPM_TABTARGET_NAME)" || { echo "Error: missing name param"; exit 1; }
	@echo '#!/bin/sh' > $(zMRM_TABTARGET_FILE)
	@echo 'cd "$$(dirname "$$0")/.." &&  Tools/tabtarget-dispatch.sh 1 "$$(basename "$$0")"' \
	                 >> $(zMRM_TABTARGET_FILE)
	@chmod +x           $(zMRM_TABTARGET_FILE)


#######################################
#  Slickedit Project Tabtarget
#
#  Due to filesystem handle entanglements, Slickedit doesn't play well
#  with git.  This tabtarget places a usable copy in a .gitignored location

zMRM_SLICKEDIT_PROJECT_DIR = ./_slickedit

vsr.ReplaceSlickEditWorkspace.sh:
	mkdir -p                                             $(zMRM_SLICKEDIT_PROJECT_DIR)
	-rm -rf                                              $(zMRM_SLICKEDIT_PROJECT_DIR)/*
	cp $(CPM_TOOLS_RELDIR)/vsep_VisualSlickEditProject/* $(zMRM_SLICKEDIT_PROJECT_DIR)
	$(zSSIMK_PASS)


# EOF
