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

zRBC_MBC_MAKEFILE = $(MBV_TOOLS_DIR)/mbc.MakefileBashConsole.mk
zRBC_BGC_MAKEFILE = $(MBV_TOOLS_DIR)/bgc.BuildGithubContainers.mk
zRBC_RBM_MAKEFILE = $(MBV_TOOLS_DIR)/rbm.RecipeBottleMakefile.mk

# Submake config: What console tool will put in prefix of each line
MBC_ARG__CTXT = $(MBV_CONSOLE_MAKEFILE)

# Submake config: How selection of a bottle service is done
RBM_MONIKER = $(MBDM_PARAMETER_2)

include $(zRBC_MBC_MAKEFILE)
include $(zRBC_BGC_MAKEFILE)
include $(zRBC_RBM_MAKEFILE)


default:
	$(MBC_SHOW_RED) "NO TARGET SPECIFIED.  Check" $(MBV_TABTARGET_DIR) "directory for options." && $(MBC_FAIL)


#######################################
#  Clean Temporary directory creation
#
# This might better be done in the dispatch script.  Not sure:
# problems of dispatches calling dispatches may be unique to
# testing Recipe Bottle, not using it.

zRBC_TEMP_DIR = $(MBV_TEMP_ROOT_DIR)/temp-$(MBV_NOW_STAMP)

zrbc_prepare_temporary_dir:
	$(MBC_START) "Set up temporary dir ->" $(zRBC_TEMP_DIR)
	@$(call MBC_CHECK_NONEMPTY,1,$(zRBC_TEMP_DIR))
	@$(call MBC_CHECK_NONEMPTY,1,$(MBV_TEMP_ROOT_DIR))
	@$(call MBC_CHECK_NONEMPTY,1,$(MBV_NOW_STAMP))
	mkdir -p    $(zRBC_TEMP_DIR)
	@test -d   "$(zRBC_TEMP_DIR)"   || ($(MBC_SEE_RED) "Failed to create directory" && exit 1)
	@test ! -f "$(zRBC_TEMP_DIR)/*" || ($(MBC_SEE_RED) "Directory contains files"   && exit 1)


#######################################
#  Startup
#

rbc-a%:  rbp_podman_machine_start_rule  bgc_container_registry_login_rule
	$(MBC_START) "Podman started and logged into container registry"
	$(MBC_PASS) "No errors."


#######################################
#  Test Targets
#

zRBC_START_TEST_CMD = $(MAKE) -f $(MBV_CONSOLE_MAKEFILE) zrbm_start_service_rule
zRBC_MAKE_TEST_CMD  = $(MAKE) -f $(MBV_CONSOLE_MAKEFILE) rbm_test_nameplate_rule RBM_TEMP_DIR=$(zRBC_TEMP_DIR)

rbc-to%: zrbc_prepare_temporary_dir
	$(MBC_START) "Test for $(RBM_MONIKER) beginning"
	$(MBC_STEP)  "Test the bottle service"
	$(zRBC_MAKE_TEST_CMD)
	$(MBC_PASS) "No errors."

rbc-tb%: zrbc_prepare_temporary_dir
	$(MBC_START) "For each well known nameplate"
	$(zRBC_START_TEST_CMD) RBM_MONIKER=nsproto
	$(zRBC_MAKE_TEST_CMD)  RBM_MONIKER=nsproto
	$(zRBC_START_TEST_CMD) RBM_MONIKER=srjcl
	$(zRBC_MAKE_TEST_CMD)  RBM_MONIKER=srjcl
	$(zRBC_START_TEST_CMD) RBM_MONIKER=pluml
	$(zRBC_MAKE_TEST_CMD)  RBM_MONIKER=pluml
	$(MBC_PASS) "No errors."

zRBC_TEST_RECIPE = test_busybox.recipe

zRBC_FQIN_FILE     = $(zRBC_TEMP_DIR)/fqin.txt
zBGC_FQIN_CONTENTS = $$(cat $(zRBC_FQIN_FILE))

rbc-tg%: zrbc_prepare_temporary_dir
	$(MBC_START) "Test github action build, retrieval, use"
	$(MBC_STEP) "Validate list before..."
	tt/bgc-l.ListCurrentRegistryImages.sh
	$(MBC_STEP) "Validate build..."
	tt/bgc-b.BuildWithRecipe.sh $(BGCV_RECIPES_DIR)/$(zRBC_TEST_RECIPE) $(zRBC_FQIN_FILE)
	$(MBC_STEP) "Validate list during..."
	tt/bgc-l.ListCurrentRegistryImages.sh
	$(MBC_STEP) "Validate retrieval..."
	tt/bgc-r.RetrieveImage.sh $(zBGC_FQIN_CONTENTS)
	$(MBC_STEP) "Validate deletion..."
	tt/bgc-d.DeleteImageFromRegistry.sh $(zBGC_FQIN_CONTENTS) BGC_ARG_SKIP_DELETE_CONFIRMATION=SKIP
	$(MBC_STEP) "Validate list after..."
	tt/bgc-l.ListCurrentRegistryImages.sh
	$(MBC_PASS) "No errors."

rbc-ta%:
	$(MBC_START) "RUN REPOWIDE TESTS"
	$(MBC_STEP) "Github tests..."
	tt/rbc-tg.TestGithubWorkflow.sh
	$(MBC_STEP) "Bottle service tests..."
	tt/rbc-tb.TestBottles.sh
	$(MBC_PASS) "No errors."


#######################################
#  Tabtarget Maintenance Tabtarget
#
#  Helps you create default form tabtargets in right place.

# Parameter from the tabtarget: what is the full name of the new tabtarget, no directory 
RBC_TABTARGET_NAME = 

zRBC_TABTARGET_FILE  = $(MBV_TABTARGET_DIR)/$(RBC_TABTARGET_NAME)
zRBC_DISPATCH_SCRIPT = $(MBV_TOOLS_DIR)/mbd.dispatch.sh

ttc.CreateTabtarget.sh:
	@test -n "$(RBC_TABTARGET_NAME)" || { echo "Error: missing name param"; exit 1; }
	@echo '#!/bin/sh' >         $(zRBC_TABTARGET_FILE)
	@echo 'cd "$$(dirname "$$0")/.." &&  $(zRBC_DISPATCH_SCRIPT) jp_single om_line "$$(basename "$$0")"' \
	                 >>         $(zRBC_TABTARGET_FILE)
	@chmod +x                   $(zRBC_TABTARGET_FILE)
	git add                     $(zRBC_TABTARGET_FILE)
	git update-index --chmod=+x $(zRBC_TABTARGET_FILE)
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
