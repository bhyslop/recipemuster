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
RBM_NAMEPLATE_FILE = $(RBRR_NAMEPLATE_PATH)/nameplate.$(RBM_MONIKER).mk
RBM_TEST_FILE      = RBM-tests/rbt.test.$(RBM_MONIKER).mk
RBM_STATION_SH     = ../station-files/rbrs-station.sh

RBM_RECIPE_BOTTLE_GITHUB_SH = BURD_TEMP_DIR="$(MBD_TEMP_DIR)"                 \
                              BURD_NOW_STAMP="$(MBD_NOW_STAMP)"               \
                              RBG_RBRR_FILE="rbrr.env"                       \
                              RBG_RUNTIME="podman"                           \
                              RBG_RUNTIME_ARG="--connection=rbw-vm-deploy"   \
                              $(MBV_TOOLS_DIR)/rbg_RecipeBottleGithub.sh

RBM_RECIPE_BOTTLE_VM_SH = RBV_TEMP_DIR="$(MBD_TEMP_DIR)"                   \
                          RBV_NOW_STAMP="$(MBD_NOW_STAMP)"                 \
                          RBV_RBRR_FILE="rbrr.env"                         \
                          RBV_RBRS_FILE="$(RBM_STATION_SH)"                \
                          $(MBV_TOOLS_DIR)/rbv_PodmanVM.sh


-include $(RBM_NAMEPLATE_FILE)
-include $(RBM_TEST_FILE)
include $(MBV_TOOLS_DIR)/mbc.console.mk
include $(MBV_TOOLS_DIR)/rbrn.nameplate.mk


RBW_RECIPES_DIR  = RBM-recipes

default:
	$(MBC_SHOW_RED) "NO TARGET SPECIFIED.  Check" $(MBV_TABTARGET_DIR) "directory for options." && $(MBC_FAIL)


#######################################
#  Github Container Registry
#

RBG_ARG_TAG =

rbw-hv.%:
	$(MBC_START) "Podman VM Command Help"
	$(RBM_RECIPE_BOTTLE_VM_SH)
	$(MBC_PASS) "No errors."


#######################################
#  Podman automation
#
# These rules are designed to allow the pattern match to parameterize
# the operation via $(RBM_MONIKER).

rbw-N.%:
	$(MBC_START) "New nuke..."
	$(RBM_RECIPE_BOTTLE_VM_SH) rbv_nuke
	$(MBC_PASS) "VM image check complete."

rbw-e.%:
	$(MBC_START) "Run experiment..."
	$(RBM_RECIPE_BOTTLE_VM_SH) rbv_experiment
	$(MBC_PASS) "VM image check complete."

rbw-c.%:
	$(MBC_START) "Check on latest podman VM image..."
	$(RBM_RECIPE_BOTTLE_VM_SH) rbv_check
	$(MBC_PASS) "VM image check complete."

rbw-m.%:
	$(MBC_START) "Mirror latest podman VM image..."
	$(RBM_RECIPE_BOTTLE_VM_SH) rbv_mirror
	$(MBC_PASS) "VM image check complete."

rbw-f.%:
	$(MBC_START) "Fetch chosen podman VM image..."
	$(RBM_RECIPE_BOTTLE_VM_SH) rbv_fetch
	$(MBC_PASS) "VM image check complete."

rbw-i.%:
	$(MBC_START) "Initialize podman VM..."
	$(RBM_RECIPE_BOTTLE_VM_SH) rbv_init
	$(MBC_PASS) "VM image check complete."

# Lifecycle routes now delegate to bash workbench via coordinator tabtargets:
# rbw-S.% -> tt/rbw-S.ConnectSentry.<moniker>.sh -> rbw_workbench.sh rbw-connect-sentry
# rbw-C.% -> tt/rbw-C.ConnectCenser.<moniker>.sh -> rbw_workbench.sh rbw-connect-censer
# rbw-B.% -> tt/rbw-B.ConnectBottle.<moniker>.sh -> rbw_workbench.sh rbw-connect-bottle
# rbw-o.% -> tt/rbw-o.ObserveNetworks.<moniker>.sh -> rbw_workbench.sh rbw-observe
# rbw-s.% -> tt/rbw-s.Start.<moniker>.sh -> rbw_workbench.sh rbw-start

rbw-v.%: zrbp_validate_regimes_rule
	$(MBC_PASS) "No errors."


#######################################
#  Test Targets
#
# Test execution migrated to bash:
# - MBT_PODMAN_* macros -> rbt_exec_* functions in Tools/rbw/rbt_testbench.sh
# - rbt_test_bottle_service_rule -> rbt_suite_* functions in Tools/rbw/rbt_testbench.sh
#
# All tests now route through:
# - tt/rbw-to.TestBottleService.<moniker>.sh -> rbt_testbench.sh rbt-to <moniker>

RBT_TESTS_DIR = RBM-tests

# Legacy test rule - now delegates to bash testbench via coordinator tabtargets
rbw-to.%:
	@echo "Test execution now handled by rbt_testbench.sh via tabtargets"
	@echo "Use: tt/rbw-to.TestBottleService.<moniker>.sh"
	$(MBC_FAIL)

# Batch test rule (rbw-tb.%) not yet migrated to bash.
# Original pattern was: for each nameplate, restart service, run tests
# To run individual nameplate tests, use:
#   tt/rbw-to.TestBottleService.nsproto.sh
#   tt/rbw-to.TestBottleService.srjcl.sh
#   tt/rbw-to.TestBottleService.pluml.sh

rbw-tb.%:
	$(MBC_START) "Batch test execution not yet migrated to bash"
	@echo "Run individual test suites via tabtargets instead:"
	@echo "  tt/rbw-to.TestBottleService.nsproto.sh"
	@echo "  tt/rbw-to.TestBottleService.srjcl.sh"
	@echo "  tt/rbw-to.TestBottleService.pluml.sh"
	$(MBC_FAIL)

zRBC_TEST_RECIPE = test_busybox.recipe

zRBC_FQIN_FILE     = $(MBD_TEMP_DIR)/fqin.txt
zRBC_FQIN_CONTENTS = $$(cat $(zRBC_FQIN_FILE))

rbw-tg.%:
	$(MBC_START) "Test github action build, retrieval, use"
	$(MBV_TOOLS_DIR)/test/trbim_suite.sh $(MBD_TEMP_DIR)
	$(MBC_PASS) "No errors."

rbw-tf.%:
	$(MBC_START) "Fast test - now using bash testbench"
	tt/rbg-l.ListCurrentRegistryImages.sh
	tt/rbw-to.TestBottleService.pluml.sh
	$(MBC_PASS) "No errors."

rbw-ta.%:
	$(MBC_START) "RUN REPOWIDE TESTS"
	$(MBC_STEP) "Github tests..."
	tt/rbw-tg.TestGithubWorkflow.sh
	$(MBC_STEP) "Bottle service tests (manual - run each tabtarget)..."
	@echo "  tt/rbw-to.TestBottleService.nsproto.sh"
	@echo "  tt/rbw-to.TestBottleService.srjcl.sh"
	@echo "  tt/rbw-to.TestBottleService.pluml.sh"
	@echo "Note: parallel batch test execution not yet migrated to bash"
	$(MBC_FAIL)


#######################################
#  TabTarget Maintenance TabTargets
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


#######################################
#  Bash File TabTargets
#

lmci-b.%:
	$(MBC_START) "Bundle: Placing file batch on clipboard..."
	$(MBV_TOOLS_DIR)/lmci/bundle.sh $(MBD_CLI_ARGS) | putclip
	$(MBC_PASS) "No errors."

lmci-s.%:
	$(MBC_START) "Strip: Cut trailing whitespace and assure terminal newline..."
	$(MBV_TOOLS_DIR)/lmci/strip.sh $(MBD_CLI_ARGS)
	$(MBC_PASS) "No errors."



#########################################
#  Legacy helpers
#

rbw-hw.%:
	$(MBC_START) "Helper for WSL Distribution Management:"
	$(MBC_SHOW_NORMAL) Stop wsl:
	@echo
	$(MBC_RAW_YELLOW)  "     wsl --shutdown"
	@echo
	$(MBC_SHOW_NORMAL) "List current distributions:"
	@echo
	$(MBC_RAW_YELLOW)  "     wsl -l -v"
	@echo
	$(MBC_SHOW_NORMAL) "List available distributions:"
	@echo
	$(MBC_RAW_YELLOW)  "     wsl --list --online"
	@echo
	$(MBC_SHOW_NORMAL) "Delete a distribution:"
	@echo
	$(MBC_RAW_YELLOW)  "     wsl --unregister <DistroName>"
	@echo
	$(MBC_SHOW_NORMAL) "Install a distribution:"
	@echo
	$(MBC_RAW_YELLOW)  "     wsl --install <DistroName>"
	@echo
	$(MBC_SHOW_NORMAL) "Install Required Podman Dependencies:"
	@echo
	$(MBC_RAW_YELLOW)  "     sudo dnf install make"
	$(MBC_RAW_YELLOW)  "     sudo dnf install ncurses   # for tput"
	$(MBC_RAW_YELLOW)  "     sudo dnf install qemu-img"
	$(MBC_RAW_YELLOW)  "     sudo dnf install qemu-system-x86"
	$(MBC_RAW_YELLOW)  "     sudo dnf install libvirt-daemon-driver-qemu"
	$(MBC_RAW_YELLOW)  "     sudo dnf install virtiofsd"
	@echo
	$(MBC_SHOW_NORMAL) "Access Windows C: drive:"
	@echo
	$(MBC_RAW_YELLOW)  "     cd /mnt/c"
	@echo
	$(MBC_SHOW_NORMAL) "Validate Podman Installation:"
	@echo
	$(MBC_RAW_YELLOW)  "     podman --version"
	$(MBC_RAW_YELLOW)  "     podman machine ls"
	@echo
	$(MBC_PASS) "No errors."


zRBW_PODMAN_INSTALL_ROOT = /cygdrive/c/podman-remote
zRBW_PODMAN_INSTALL_WIN  = $(shell cygpath -wa $(zRBW_PODMAN_INSTALL_ROOT))

zRBW_SAMPLE_HTML_ROOT = /cygdrive/c/podman-remote/podman-5.4.0/docs

zRBW_DOC_CONSOLIDATOR_PYTHON = Study/study-strip-podman-docs/spd.strip-podman-docs.py

zRBW_DOC_CONSOLIDATION_IMAGE = ghcr.io/bhyslop/recipemuster:bottle_deftextpro.20250227__172342

rbw-D.DigestPodmanHtml.sh:
	echo path is -> $(zRBW_PODMAN_INSTALL_WIN)
	podman -c podman-machine-default run --rm \
	  -v '$(zRBW_PODMAN_INSTALL_WIN)':/podman-remote:ro \
	  -v ./Study/study-strip-podman-docs:/app/study:rw \
	  $(zRBW_DOC_CONSOLIDATION_IMAGE) \
	  python /app/study/spd.strip-podman-docs.py /podman-remote /app/study/output

oga.OpenGithubAction.sh:
	$(MBC_STEP) "Assure podman services available..."
	cygstart https://github.com/bhyslop/recipemuster/actions/


# EOF
