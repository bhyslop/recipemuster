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

# Get the master configuration
include mbv.variables.sh

#########################
# Makefile Bash Console
#
# This is a sub makefile that contains several canned basic
# macros that support regular console interactivity.
#

zRBC_MBC_MAKEFILE = $(MBV_TOOLS_DIR)/mbc.MakefileBashConsole.mk
zRBC_BGC_MAKEFILE = $(MBV_TOOLS_DIR)/bgc.BuildGithubContainers.mk

# What console tool will put in prefix of each line
MBC_ARG__CONTEXT_STRING = $(MBV_MAKEFILE)

include $(zRBC_MBC_MAKEFILE)
include $(zRBC_BGC_MAKEFILE)

zRBC_START = $(MBC_SHOW_WHITE) "Rule $@: starting..."
zRBC_STEP  = $(MBC_SHOW_WHITE) "Rule $@:"
zRBC_PASS  = $(MBC_PASS)       "Rule $@: no errors."


default:
	$(MBC_SHOW_RED) "NO TARGET SPECIFIED.  Check" $(zRBC_TABTARGET_DIR) "directory for options." && $(MBC_FAIL)


# Configure and include the Recipe Bottle Makefile
zRBC_RBM_MAKEFILE := $(MBV_TOOLS_DIR)/rbm.RecipeBottleMakefile.mk
zRBT_TEST_NSPROTO := $(MBV_TOOLS_DIR)/rbt.test.nsproto.mk
zRBT_TEST_SRJCL   := $(MBV_TOOLS_DIR)/rbt.test.srjcl.mk

RBM_MONIKER := $(MBDM_PARAMETER_2)

include $(zRBC_RBM_MAKEFILE)


#######################################
#  Test Targets
#

rbc-ta.%: rbs_define rbb_define rbn_define
	$(MBC_START) "Run all tests"
	$(MAKE) -f $(zRBT_TEST_NSPROTO) rbt_test_bottle_service_rule RBM_MONIKER=nsproto
	$(MAKE) -f $(zRBT_TEST_SRJCL)   rbt_test_bottle_service_rule RBM_MONIKER=srjcl
	$(MBC_PASS)

rbc-to.%: rbs_define rbb_define rbn_define
	$(MBC_START) "Run tests for nameplate ->" $(RBM_MONIKER)
	$(MAKE) -f rbt.test.$(RBM_MONIKER).mk   rbt_test_bottle_service_rule
	$(MBC_PASS)


# OUCH is this the right place?
oga.OpenGithubAction.sh:
	$(zRBC_STEP) "Assure podman services available..."
	cygstart https://github.com/bhyslop/recipemuster/actions/



#######################################
#  Tabtarget Maintenance Tabtarget
#
#  Helps you create default form tabtargets in right place.

# Location for tabtargets relative to top level project directory
zRBC_TABTARGET_DIR  = ./tt

# Parameter from the tabtarget: what is the full name of the new tabtarget
RBC_TABTARGET_NAME = 

zRBC_TABTARGET_FILE = $(zRBC_TABTARGET_DIR)/$(RBC_TABTARGET_NAME)

ttc.CreateTabtarget.sh:
	@test -n "$(RBC_TABTARGET_NAME)" || { echo "Error: missing name param"; exit 1; }
	@echo '#!/bin/sh' >         $(zRBC_TABTARGET_FILE)
	@echo 'cd "$$(dirname "$$0")/.." &&  Tools/Tools/mbd.dispatch.sh jp_single om_line "$$(basename "$$0")"' \
	                 >>         $(zRBC_TABTARGET_FILE)
	@chmod +x                   $(zRBC_TABTARGET_FILE)
	git add                     $(zRBC_TABTARGET_FILE)
	git update-index --chmod=+x $(zRBC_TABTARGET_FILE)
	$(zRBC_PASS)


ttx.FixTabtargetExecutability.sh:
	git update-index --chmod=+x $(zRBC_TABTARGET_DIR)/*
	$(zRBC_PASS)


#########################################
#  Test Tabtargets
#

Tttl.TestTabtargetLauncher.sh:
	@echo "RBC_PARAMETER_1:" $(RBC_PARAMETER_1)
	@echo "RBC_PARAMETER_2:" $(RBC_PARAMETER_2)
	@echo "RBC_PARAMETER_3:" $(RBC_PARAMETER_3)
	@echo "RBC_PARAMETER_4:" $(RBC_PARAMETER_4)
	$(zRBC_PASS)



# EOF
