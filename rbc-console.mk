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


# View interim official at: https://github.com/bhyslop/recipemuster/rbc-console.mk
# View planner at: https://github.com/bhyslop/recipemuster/tree/main/PRIVATE_PLANNER

# Prefix used to distinguish commentary created by this makefile
zRBC_THIS_MAKEFILE = rbc-console.mk

zRBC_TOOLS_DIR     = Tools

#########################
# Makefile Bash Console
#
# This is a sub makefile that contains several canned basic
# macros that support regular console interactivity.
#

zRBC_MBC_MAKEFILE = $(zRBC_TOOLS_DIR)/mbc.MakefileBashConsole.mk
zRBC_BGC_MAKEFILE = $(zRBC_TOOLS_DIR)/bgc.BuildGithubContainers.mk


# What console tool will put in prefix of each line
MBC_ARG__CONTEXT_STRING = $(zRBC_THIS_MAKEFILE)

include $(zRBC_MBC_MAKEFILE)
include $(zRBC_BGC_MAKEFILE)

zRBC_START = $(MBC_SHOW_WHITE) "Rule $@: starting..."
zRBC_STEP  = $(MBC_SHOW_WHITE) "Rule $@:"
zRBC_PASS  = $(MBC_PASS)       "Rule $@: no errors."


# Use this to allow a console make target to explicitly trigger other
#   console make targets.  This is really only for making sure entire
#   rules function in 'big test cases': for efficiency, better to use
#   explicit fine grained make dependencies so that make is efficient.
zRBC_MAKE = $(MAKE) -f $(zRBC_THIS_MAKEFILE)

default:
	$(MBC_SHOW_RED) "NO TARGET SPECIFIED.  Check" $(zRBC_TABTARGET_DIR) "directory for options." && $(MBC_FAIL)

# OUCH scrub this out eventually
# $(info RBC_PARAMETER_0: $(RBC_PARAMETER_0))
# $(info RBC_PARAMETER_1: $(RBC_PARAMETER_1))
# $(info RBC_PARAMETER_2: $(RBC_PARAMETER_2))
# $(info RBC_PARAMETER_3: $(RBC_PARAMETER_3))

# Configure and include the Recipe Bottle Makefile
zRBC_RBM_MAKEFILE := $(zRBC_TOOLS_DIR)/rbm.RecipeBottleMakefile.mk

RBM_MONIKER := $(RBC_PARAMETER_2)

include $(zRBC_RBM_MAKEFILE)

zRBC_RBM_SUBMAKE = $(MAKE) -f $(zRBC_RBM_MAKEFILE) RBM_ARG_SUBMAKE_MBC=$(zRBC_MBC_MAKEFILE)



#######################################
#  Config Regime Info
#
#  This section has examples of several sample nameplates, some
#  of which are useful in maintaining this example space.


rbm-Ci.ConfigRegimeInfo.sh: rbs_define rbb_define rbn_define
	$(zRBC_PASS)



#######################################
#  Nameplate Examples
#
#  This section has examples of several sample nameplates, some
#  of which are useful in maintaining this example space.

pyghm-B.BuildPythonGithubMaintenance.sh:
	$(zRBC_STEP) "Assure podman services available..."
	which podman
	podman machine start || echo "Podman probably running already, lets go on..."
	$(zRBC_RBM_SUBMAKE) rbm-BL.sh  RBM_ARG_MONIKER=pyghm

pyghm-s.StartPythonGithubMaintenance.sh:
	$(zRBC_STEP) "Assure podman services available..."
	which podman
	podman machine start || echo "Podman probably running already, lets go on..."
	$(zRBC_RBM_SUBMAKE) rbm-s.sh  RBM_ARG_MONIKER=pyghm


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
	@echo 'cd "$$(dirname "$$0")/.." &&  Tools/tabtarget-dispatch.sh 1 "$$(basename "$$0")"' \
	                 >>         $(zRBC_TABTARGET_FILE)
	@chmod +x                   $(zRBC_TABTARGET_FILE)
	git add                     $(zRBC_TABTARGET_FILE)
	git update-index --chmod=+x $(zRBC_TABTARGET_FILE)
	$(zRBC_PASS)


ttx.FixTabtargetExecutability.sh:
	git update-index --chmod=+x $(zRBC_TABTARGET_DIR)/*
	$(zRBC_PASS)


#######################################
#  Slickedit Project Tabtarget
#
#  Due to filesystem handle entanglements, Slickedit doesn't play well
#  with git.  This tabtarget places a usable copy in a .gitignored location

zRBC_SLICKEDIT_PROJECT_DIR = ./_slickedit

vsr.ReplaceSlickEditWorkspace.sh:
	mkdir -p                                           $(zRBC_SLICKEDIT_PROJECT_DIR)
	-rm -rf                                            $(zRBC_SLICKEDIT_PROJECT_DIR)/*
	cp $(zRBC_TOOLS_DIR)/vsep_VisualSlickEditProject/* $(zRBC_SLICKEDIT_PROJECT_DIR)
	$(zRBC_PASS)

RBC_PARAM_DIR = 

rbcgi.CreateGitIgnore.sh:
	@test -n             "$(RBC_PARAM_DIR)" || { echo "Must provide dir name arg"; exit 1; }
	mkdir -p              $(RBC_PARAM_DIR)
	echo "*"            > $(RBC_PARAM_DIR)/.gitignore
	echo "!.gitignore" >> $(RBC_PARAM_DIR)/.gitignore
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
