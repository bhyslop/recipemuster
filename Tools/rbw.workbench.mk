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

RBM_RECIPE_BOTTLE_GITHUB_SH = RBG_TEMP_DIR="$(MBD_TEMP_DIR)"               \
                              RBG_NOW_STAMP="$(MBD_NOW_STAMP)"             \
                              RBG_RBRR_FILE="rbrr.repo.sh"                 \
                              RBG_RUNTIME="podman"                         \
                              RBG_RUNTIME_ARG="--connection=rbw-vm-deploy" \
                              $(MBV_TOOLS_DIR)/rbg_RecipeBottleGithub.sh

RBW_IMAGE_MANAGEMENT_SH = RBG_TEMP_DIR="$(MBD_TEMP_DIR)"                   \
                          RBG_NOW_STAMP="$(MBD_NOW_STAMP)"                 \
                          RBG_RBRR_FILE="rbrr_RecipeBottleRegimeRepo.sh"   \
                          RBG_RUNTIME="podman"                             \
                          RBG_RUNTIME_ARG="--connection=rbw-vm-deploy"     \
                          $(MBV_TOOLS_DIR)/rbim_cli.sh

RBM_RECIPE_BOTTLE_VM_SH = RBV_TEMP_DIR="$(MBD_TEMP_DIR)"                   \
                          RBV_NOW_STAMP="$(MBD_NOW_STAMP)"                 \
                          RBV_RBRR_FILE="rbrr_RecipeBottleRegimeRepo.sh"   \
                          RBV_RBRS_FILE="$(RBM_STATION_SH)"                \
                          $(MBV_TOOLS_DIR)/rbv_PodmanVM.sh


-include $(RBM_NAMEPLATE_FILE)
-include $(RBM_TEST_FILE)
include $(RBV_GITHUB_PAT_ENV)
include $(MBV_TOOLS_DIR)/mbc.console.mk
include $(MBV_TOOLS_DIR)/rbrn.nameplate.mk
include $(MBV_TOOLS_DIR)/rbp.podman.mk


RBW_RECIPES_DIR  = RBM-recipes

default:
	$(MBC_SHOW_RED) "NO TARGET SPECIFIED.  Check" $(MBV_TABTARGET_DIR) "directory for options." && $(MBC_FAIL)


#######################################
#  Github Container Registry
#

RBG_ARG_TAG =

rbw-hi.%:
	$(MBC_START) "Image Management Command Help"
	$(RBW_IMAGE_MANAGEMENT_SH)
	$(MBC_PASS) "No errors."

rbw-hv.%:
	$(MBC_START) "Podman VM Command Help"
	$(RBM_RECIPE_BOTTLE_VM_SH)
	$(MBC_PASS) "No errors."

rbw-l.%:
	$(MBC_START) "List Current Registry Images"
	$(RBW_IMAGE_MANAGEMENT_SH) rbim_list
	$(MBC_PASS) "No errors."

rbw-II.%:
	$(MBC_START) "List Registry Image Info"
	$(RBW_IMAGE_MANAGEMENT_SH) rbim_image_info $(MBD_CLI_ARGS)
	$(MBC_PASS) "No errors."

rbw-r.%:
	$(MBC_START) "Retrieve Image From Registry given $(MBD_CLI_ARGS)"
	$(RBW_IMAGE_MANAGEMENT_SH) rbim_retrieve $(MBD_CLI_ARGS)
	$(MBC_PASS) "No errors."

rbw-b.%:
	$(MBC_START) "Build Container From Recipe given $(MBD_CLI_ARGS)"
	$(RBW_IMAGE_MANAGEMENT_SH) rbim_build $(MBD_CLI_ARGS)
	$(MBC_PASS) "No errors."

rbw-d.%:
	$(MBC_START) "Delete Image From Registry given $(MBD_CLI_ARGS)"
	$(RBM_RECIPE_BOTTLE_GITHUB_SH) rbg_delete $(MBD_CLI_ARGS)
	$(MBC_PASS) "No errors."


#######################################
#  Podman automation
#
# These rules are designed to allow the pattern match to parameterize
# the operation via $(RBM_MONIKER).

rbw-a.%: zrbw_prestart_rule rbp_podman_machine_start_rule rbp_check_connection
	$(MBC_PASS) "Podman started and logged into container registry."

rbw-z.%: zrbw_prestop_rule rbp_podman_machine_stop_rule
	$(MBC_PASS) "Podman stopped."

rbw-Z.%: zrbw_prenuke_rule rbp_podman_machine_nuke_rule
	$(MBC_PASS) "Nuke completed."

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

rbw-S.%: rbp_connect_sentry_rule
	$(MBC_PASS) "No errors."

rbw-C.%: rbp_connect_censer_rule
	$(MBC_PASS) "No errors."

rbw-B.%: rbp_connect_bottle_rule
	$(MBC_PASS) "No errors."

rbw-o.%: rbp_observe_networks_rule
	$(MBC_PASS) "No errors."

rbw-s.%: rbp_check_connection rbp_start_service_rule
	$(MBC_STEP) "Completed delegate."

rbw-v.%: zrbp_validate_regimes_rule
	$(MBC_PASS) "No errors."

zrbw_prestart_rule:
	$(MBC_START) "Starting podman and logging in to container registry..."

zrbw_prestop_rule:
	$(MBC_START) "Stopping podman..."

zrbw_prenuke_rule:
	$(MBC_START) "Nuking podman..."


#######################################
#  Test Targets
#

RBT_TESTS_DIR            = RBM-tests
MBT_PODMAN_BASE          = podman --connection $(RBM_MACHINE)
MBT_PODMAN_EXEC_SENTRY   = $(MBT_PODMAN_BASE) exec    $(RBM_SENTRY_CONTAINER)
MBT_PODMAN_EXEC_CENSER   = $(MBT_PODMAN_BASE) exec    $(RBM_CENSER_CONTAINER)
MBT_PODMAN_EXEC_CENSER_I = $(MBT_PODMAN_BASE) exec -i $(RBM_CENSER_CONTAINER)
MBT_PODMAN_EXEC_BOTTLE   = $(MBT_PODMAN_BASE) exec    $(RBM_BOTTLE_CONTAINER)
MBT_PODMAN_EXEC_BOTTLE_I = $(MBT_PODMAN_BASE) exec -i $(RBM_BOTTLE_CONTAINER)

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
	$(MBV_TOOLS_DIR)/test/trbg_suite.sh $(MBD_TEMP_DIR)
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
	$(MBC_PASS) "TEST ALL PASSED WITHOUT ERRORS."


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
	$(MBV_TOOLS_DIR)/lmci/bundle.sh $(MBD_CLI_ARGS) | clip
	$(MBC_PASS) "No errors."

lmci-s.%:
	$(MBC_START) "Strip: Cut trailing whitespace and assure terminal newline..."
	$(MBV_TOOLS_DIR)/lmci/strip.sh $(MBD_CLI_ARGS)
	$(MBC_PASS) "No errors."


#######################################
#  Visual SlickEdit TabTargets
#

vsp-g.%:
	$(MBC_START) "Regenerating slickedit project..."
	$(MBV_TOOLS_DIR)/yeti-generate-rbm.sh
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
