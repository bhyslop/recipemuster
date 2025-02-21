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

SHELL := /bin/bash -eo pipefail

# Get the master configuration
include mbv.variables.sh

# Submake config: What console tool will put in prefix of each line
MBC_ARG__CTXT = $(MBV_CONSOLE_MAKEFILE)

# Submake config: Select bottle service from a token in the rule (parsed by dispatch)
RBM_MONIKER = $(MBD_PARAMETER_2)

# Submake config: Where to find derived nameplate and test files
RBM_NAMEPLATE_FILE = $(RBB_NAMEPLATE_PATH)/nameplate.$(RBM_MONIKER).mk
RBM_TEST_FILE      = RBM-tests/rbt.test.$(RBM_MONIKER).mk

include ../RBS.STATION.sh
include ../RBS_STATION.mk
include rbb.base.mk
-include $(RBM_NAMEPLATE_FILE)
-include $(RBM_TEST_FILE)
include rbv.variables.mk
include $(RBV_GITHUB_PAT_ENV)
include $(MBV_TOOLS_DIR)/rbvc.checker.mk
include $(MBV_TOOLS_DIR)/mbc.console.mk
include $(MBV_TOOLS_DIR)/rbg.github.mk
include $(MBV_TOOLS_DIR)/rbrb.base.mk
include $(MBV_TOOLS_DIR)/rbrn.nameplate.mk
include $(MBV_TOOLS_DIR)/rbrs.station.mk
include $(MBV_TOOLS_DIR)/rbp.podman.mk

default:
	$(MBC_SHOW_RED) "NO TARGET SPECIFIED.  Check" $(MBV_TABTARGET_DIR) "directory for options." && $(MBC_FAIL)


#######################################
#  Startup
#

rbw-a.%:  rbp_podman_machine_start_rule  rbg_container_registry_login_rule
	$(MBC_START) "Podman started and logged into container registry"
	$(MBC_PASS) "No errors."


#######################################
#  Test Targets
#

RBT_TESTS_DIR = RBM-tests

# Each test defines same rule
rbw-to.%:  rbt_test_bottle_service_rule
	$(MBC_PASS) "No errors."

zRBC_RESTART_SERVICE_CMD  = $(MAKE) -f $(MBV_CONSOLE_MAKEFILE) rbp_start_service_rule
zRBC_RUN_SERVICE_TEST_CMD = $(MAKE) -f $(MBV_CONSOLE_MAKEFILE) rbt_test_bottle_service_rule RBM_TEMP_DIR=$(MBD_TEMP_DIR) -j $(MBD_JOB_PROFILE)

rbw-tb.%:
	$(MBC_START) "For each well known nameplate, and threads:$(MBD_JOB_PROFILE)"
	$(zRBC_RESTART_SERVICE_CMD)  RBM_MONIKER=nsproto
	$(zRBC_RUN_SERVICE_TEST_CMD) RBM_MONIKER=nsproto
	$(zRBC_RESTART_SERVICE_CMD)  RBM_MONIKER=srjcl
	$(zRBC_RUN_SERVICE_TEST_CMD) RBM_MONIKER=srjcl
	$(zRBC_RESTART_SERVICE_CMD)  RBM_MONIKER=pluml
	$(zRBC_RUN_SERVICE_TEST_CMD) RBM_MONIKER=pluml
	$(MBC_PASS) "No errors."

zRBC_TEST_RECIPE = test_busybox.recipe

zRBC_FQIN_FILE     = $(MBD_TEMP_DIR)/fqin.txt
zRBC_FQIN_CONTENTS = $$(cat $(zRBC_FQIN_FILE))

rbw-tg.%:
	$(MBC_START) "Test github action build, retrieval, use"
	$(MBC_STEP) "Validate list before..."
	tt/rbg-l.ListCurrentRegistryImages.sh
	$(MBC_STEP) "Validate build..."
	tt/rbg-b.BuildWithRecipe.sh $(RBV_RECIPES_DIR)/$(zRBC_TEST_RECIPE) $(zRBC_FQIN_FILE)
	$(MBC_STEP) "Validate list during..."
	tt/rbg-l.ListCurrentRegistryImages.sh
	$(MBC_STEP) "Validate retrieval..."
	tt/rbg-r.RetrieveImage.sh $(zRBC_FQIN_CONTENTS)
	$(MBC_STEP) "Validate deletion..."
	tt/rbg-d.DeleteImageFromRegistry.sh $(zRBC_FQIN_CONTENTS) RBG_ARG_SKIP_DELETE_CONFIRMATION=SKIP
	$(MBC_STEP) "Validate list after..."
	tt/rbg-l.ListCurrentRegistryImages.sh
	$(MBC_PASS) "No errors."

rbw-tf.%:
	$(MBC_START) "Fast test..."
	tt/rbg-l.ListCurrentRegistryImages.sh
	$(zRBC_RESTART_SERVICE_CMD)  RBM_MONIKER=pluml
	$(zRBC_RUN_SERVICE_TEST_CMD) RBM_MONIKER=pluml
	$(MBC_PASS) "No errors."

rbw-ta.%:
	$(MBC_START) "RUN REPOWIDE TESTS"
	$(MBC_STEP) "Github tests..."
	tt/rbw-tg.TestGithubWorkflow.sh
	$(MBC_STEP) "Bottle service tests..."
	tt/rbw-tb.TestBottles.parallel.sh
	$(MBC_PASS) "No errors."


#######################################
#  Tabtarget Maintenance Tabtarget
#
#  Helps you create default form tabtargets in right place.

# Parameter from the tabtarget: what is the full name of the new tabtarget, no directory 
RBC_TABTARGET_NAME   = 

zRBC_TABTARGET_FILE  = $(MBV_TABTARGET_DIR)/$(RBC_TABTARGET_NAME)
zRBC_DISPATCH_SCRIPT = $(MBV_TOOLS_DIR)/mbd.dispatch.sh
zRBC_TABTARGET_CMD   = 'cd "$$(dirname "$$0")/.." &&  $(zRBC_DISPATCH_SCRIPT) jp_single om_line "$$(basename "$$0")"'

ttc.CreateTabtarget.sh:
	@test -n "$(RBC_TABTARGET_NAME)" || { echo "Error: missing name param"; exit 1; }
	@echo '#!/bin/sh'           >  $(zRBC_TABTARGET_FILE)
	@echo $(zRBC_TABTARGET_CMD) >> $(zRBC_TABTARGET_FILE)
	@chmod +x                      $(zRBC_TABTARGET_FILE)
	git add                        $(zRBC_TABTARGET_FILE)
	git update-index --chmod=+x    $(zRBC_TABTARGET_FILE)
	$(MBC_PASS) "No errors."

ttx.FixTabtargetExecutability.sh:
	$(MBC_START) "Repair windows proclivity to goof up executable privileges"
	git update-index --chmod=+x $(MBV_TABTARGET_DIR)/*
	$(MBC_PASS) "No errors."


#########################################
#  Legacy helpers
#

oga.OpenGithubAction.sh:
	$(MBC_STEP) "Assure podman services available..."
	cygstart https://github.com/bhyslop/recipemuster/actions/


# EOF
