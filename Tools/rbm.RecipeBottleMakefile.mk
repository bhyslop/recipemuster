## © 2024 Scale Invariant.  All rights reserved.
##      Reference: https://www.termsfeed.com/blog/sample-copyright-notices/
##
## Unauthorized copying of this file, via any medium is strictly prohibited
## Proprietary and confidential
##
## Written by Brad Hyslop <bhyslop@scaleinvariant.org> August 2024


##############################################
# Sentry and Bottle Experiment
#
#  This is a makefile snippet that implements critical safety mechanisms for using
#  untrusted code in a secure environment.
#
#  Find below a concept where I am creating two docker containers, SENTRY and BOTTLE.
#
#  The one named BOTTLE is a container that runs untrusted code from the internet.
#  While this container has a few select needs to reach the internet, these are
#  very restricted.
#
#  The one named SENTRY is responsible for implementing these rules via dirt simple
#  and ancient linux iptables.  It performs all filtering with 'iptables' and similar
#  configuration.
#
#  This makefile supports multiple concurrent configurations via a MONIKER.  A MONIKER
#  is a parameter makefile variable that selects a configuration and dockerfiles for the
#  two containers.

# This parameter selects a particular instance of an application.
RBM_ARG_MONIKER ?= ""

# If provided, the makefile that defines console variables
RBM_ARG_SUBMAKE_MBC ?=

# Internal variables start with 'z' for easy identification

zRBM_ME := $(abspath $(lastword $(MAKEFILE_LIST)))
zRBM_MAKE = $(MAKE) -f $(zRBM_ME)

zRBM_LOCALHOST_IP = 127.0.0.1

zRBM_NAMEPLATE_DIR     = ./RBM-nameplates
zRBM_RECIPE_DIR        = ./RBM-recipes
zRBM_BUILD_CONTEXT_DIR = ./RBM-build-context
zRBM_TRANSCRIPTS_DIR   = ./RBM-transcripts
zRBM_SCRIPTS_DIR       = ./RBM-scripts
zRBM_HISTORY_DIR       = ./RBM-history

zRBM_NAMEPLATE_FILE    = $(zRBM_NAMEPLATE_DIR)/nameplate.$(RBM_ARG_MONIKER).mk

# Argument is path to the console rules to allow this makefile to be sub-make'd not included
ifneq ($(strip $(RBM_ARG_SUBMAKE_MBC)),)
include        $(RBM_ARG_SUBMAKE_MBC)
endif

-include $(zRBM_NAMEPLATE_FILE)

# Variables for container runtime
# OUCH export unneeeded given rollup var?
#
# FQIN: Fully Qualified Image Name
# RFN:  Repository Full Name: refers to the complete path to the repository, 
#                             including the registry domain, owner, and
#                             repository name. 
export RBEV_SENTRY_FQIN              := $(RBN_SENTRY_REPO_FULL_NAME):$(RBN_SENTRY_IMAGE_TAG)
export RBEV_BOTTLE_FQIN              := $(RBN_BOTTLE_REPO_FULL_NAME):$(RBN_BOTTLE_IMAGE_TAG)
export RBEV_GUARDED_NETMASK          := 16
export RBEV_GUARDED_NETWORK_SUBNET   := $(RBN_IP_HACK).0.0/$(RBEV_GUARDED_NETMASK)
export RBEV_HOST_GATEWAY             := $(RBN_IP_HACK).0.1
export RBEV_SENTRY_GUARDED_IP        := $(RBN_IP_HACK).0.2
export RBEV_BOTTLE_IP                := $(RBN_IP_HACK).0.3
export RBEV_SENTRY_HOST_INTERFACE    := eth0
export RBEV_SENTRY_GUARDED_INTERFACE := eth1
export RBEV_SENTRY_JUPYTER_PORT      := $(RBN_PORT_HOST)
export RBEV_BOTTLE_JUPYTER_PORT      := $(RBN_PORT_GUARDED)
export RBEV_BOTTLE_WORKDIR           := $(RBN_APP_INNER_DIR)
export RBEV_BOTTLE_MOUNT_DIR         := $(RBN_APP_OUTER_DIR)
export RBEV_DNS_SERVER               := 8.8.8.8

# Roll all Recipe Bottle Environment Variables up for podman exec
RBEV__ALL := $(foreach var,$(filter RBEV_%,$(.VARIABLES)),-e $(var)='$($(var))')


zRBM_START = $(MBC_SHOW_WHITE) "Moniker:"$(RBM_ARG_MONIKER)
zRBM_STEP  = $(MBC_SHOW_WHITE) "Moniker:"$(RBM_ARG_MONIKER)

zRBM_SENTRY_DOCKERFILE = $(zRBM_RECIPE_DIR)/sentry.$(RBM_ARG_MONIKER).recipe
zRBM_BOTTLE_DOCKERFILE  = $(zRBM_RECIPE_DIR)/bottle.$(RBM_ARG_MONIKER).recipe

zRBM_SENTRY_CONTAINER  = $(RBM_ARG_MONIKER)-sentry-container
zRBM_BOTTLE_CONTAINER   = $(RBM_ARG_MONIKER)-bottle-container

zRBM_HOST_NETWORK      = $(RBM_ARG_MONIKER)-host-network
zRBM_GUARDED_NETWORK   = $(RBM_ARG_MONIKER)-guarded-network

zRBM_LAST_SENTRY_CONTAINER_FACTFILE = $(zRBM_TRANSCRIPTS_DIR)/container.$(RBM_ARG_MONIKER).sentry.txt
zRBM_LAST_BOTTLE_CONTAINER_FACTFILE = $(zRBM_TRANSCRIPTS_DIR)/container.$(RBM_ARG_MONIKER).bottle.txt
zRBM_LAST_SENTRY_LOGS_FACTFILE      = $(zRBM_TRANSCRIPTS_DIR)/logs.$(RBM_ARG_MONIKER).sentry.txt
zRBM_LAST_BOTTLE_LOGS_FACTFILE      = $(zRBM_TRANSCRIPTS_DIR)/logs.$(RBM_ARG_MONIKER).bottle.txt
ZRBM_LAST_SENTRY_BUILD_FACTFILE     = $(zRBM_TRANSCRIPTS_DIR)/build.$(RBM_ARG_MONIKER).sentry.txt
ZRBM_LAST_BOTTLE_BUILD_FACTFILE     = $(zRBM_TRANSCRIPTS_DIR)/build.$(RBM_ARG_MONIKER).bottle.txt

zRBM_DNS       = 8.8.8.8

zRBM_ARGCHECK_NONZERO_CMD = test -n "$(RBM_ARG_MONIKER)"  || \
  ($(MBC_SEE_RED) "Error: In tabtarget, RBM_ARG_MONIKER must be set." && false)

zRBM_ARGCHECK_NAMEPLATE_CMD = test "$(RBM_ARG_MONIKER)" = "$(RBN_MONIKER)" || (\
  $(MBC_SEE_RED) "Error: Rule only works if proper moniker selection.  Mismatch:"   &&\
  $(MBC_SEE_RED) "      RBM_ARG_MONIKER =" $(RBM_ARG_MONIKER)                       &&\
  $(MBC_SEE_RED) "      RBN_MONIKER     =" $(RBN_MONIKER)                           &&\
  exit 1)

zrbm_argcheck_rule:
	@echo RBM_ARG_MONIKER is $(RBM_ARG_MONIKER)
	@echo RBN_MONIKER is $(RBN_MONIKER)
	@$(zRBM_ARGCHECK_NONZERO_CMD)
	@$(zRBM_ARGCHECK_NAMEPLATE_CMD)


# OUCH make a user level RBM config file...
rbm-P.SetupPodman.sh:
	$(zRBM_START) "SETUP PODMAN SESSION"
	-podman machine stop
	podman machine start
	@source ../secrets/github-ghcr-play.env &&\
	   echo $$GITHUB_GHCR_PLAY_PAT | podman login ghcr.io -u $$GITHUB_GHCR_PLAY_USERNAME --password-stdin
	$(MBC_PASS) "Done, no errors."


rbm-i%: zrbm_argcheck_rule
	$(MBC_PASS) "Done, no errors."


rbm-a%: zrbm_argcheck_rule
	$(zRBM_START) "Acquire and validate" $(RBM_ARG_MONIKER) "images from repo..."
	podman pull $(RBEV_SENTRY_FQIN)
	podman pull $(RBEV_BOTTLE_FQIN)
	$(zRBM_STEP)  "Verify Sentry image against history..."
	@test "$$(cat $(zRBM_HISTORY_DIR)/$(RBN_SENTRY_IMAGE_TAG)/docker_inspect_Id.txt)" = \
	              "$$(podman inspect $(RBEV_SENTRY_FQIN) | jq -r '.[0].Id')" || \
	     ($(MBC_SEE_RED) "Sentry image mismatch." && false)
	$(zRBM_STEP)  "Verify Bottle image against history..."
	@test "$$(cat $(zRBM_HISTORY_DIR)/$(RBN_BOTTLE_IMAGE_TAG)/docker_inspect_Id.txt)" = \
	              "$$(podman inspect $(RBEV_BOTTLE_FQIN) | jq -r '.[0].Id')" || \
	     ($(MBC_SEE_RED) "Bottle image mismatch." && false)
	$(MBC_PASS) "Done, no errors."


rbm-h%: zrbm_argcheck_rule
	$(zRBM_START) "Halt all related containers..."
	$(zRBM_STEP) "Cleaning up previous runs..."
	-podman stop  $(zRBM_SENTRY_CONTAINER) $(zRBM_BOTTLE_CONTAINER) || true
	-podman rm -f $(zRBM_SENTRY_CONTAINER) $(zRBM_BOTTLE_CONTAINER) || true
	$(MBC_PASS) "Done, no errors."


rbm-s%: zrbm_argcheck_rule
	$(zRBM_START) "START THE RECIPE SERVICE"
	$(zRBM_STEP) "Cleaning up previous runs..."
	-podman stop  $(zRBM_BOTTLE_CONTAINER) || true
	-podman rm -f $(zRBM_BOTTLE_CONTAINER) || true
	-podman stop  $(zRBM_SENTRY_CONTAINER) || true
	-podman rm -f $(zRBM_SENTRY_CONTAINER) || true
	-podman network rm -f $(zRBM_HOST_NETWORK)    || true
	-podman network rm -f $(zRBM_GUARDED_NETWORK) || true
	$(zRBM_STEP) "Creating networks..."
	podman network create --driver bridge $(zRBM_HOST_NETWORK)
	podman network create --subnet $(RBEV_GUARDED_NETWORK_SUBNET)     \
	                      --gateway $(RBEV_SENTRY_GUARDED_IP)         \
	                      --internal                                  \
	                      $(zRBM_GUARDED_NETWORK)
	$(zRBM_STEP) "Running the Sentry container with host network..."
	podman run -d                                                     \
	           --name $(zRBM_SENTRY_CONTAINER)                        \
	           --network $(zRBM_HOST_NETWORK)                         \
	           --env-file ../secrets/claude.env                       \
	           $(RBEV__ALL)                                           \
	           -p $(zRBM_LOCALHOST_IP):$(RBEV_SENTRY_JUPYTER_PORT):$(RBEV_SENTRY_JUPYTER_PORT) \
	           --privileged                                           \
	           $(RBEV_SENTRY_FQIN) > $(zRBM_LAST_SENTRY_CONTAINER_FACTFILE)
	$(zRBM_STEP) "Executing host setup script..."
	cat $(zRBM_SCRIPTS_DIR)/sentry-setup-host.sh     | podman exec -i $(RBEV__ALL) $(zRBM_SENTRY_CONTAINER) /bin/sh
	$(zRBM_STEP) "Attaching guarded network to Sentry container..."
	podman network connect $(zRBM_GUARDED_NETWORK) $(zRBM_SENTRY_CONTAINER) --ip $(RBEV_SENTRY_GUARDED_IP)
	$(zRBM_STEP) "Executing guarded setup script..."
	cat $(zRBM_SCRIPTS_DIR)/sentry-setup-guarded.sh  | podman exec -i $(RBEV__ALL) $(zRBM_SENTRY_CONTAINER) /bin/sh
	$(zRBM_STEP) "Executing service setup script..."
	cat $(zRBM_SCRIPTS_DIR)/sentry-setup-service.sh  | podman exec -i $(RBEV__ALL) $(zRBM_SENTRY_CONTAINER) /bin/sh
	$(zRBM_STEP) "Executing outreach setup script..."
	cat $(zRBM_SCRIPTS_DIR)/sentry-setup-outreach.sh | podman exec -i $(RBEV__ALL) $(zRBM_SENTRY_CONTAINER) /bin/sh
	$(zRBM_STEP) "Running the Bottle container..."
	podman run -d                                                     \
	           --name $(zRBM_BOTTLE_CONTAINER)                        \
	           --network $(zRBM_GUARDED_NETWORK):ip=$(RBEV_BOTTLE_IP) \
	           --network-alias $(zRBM_BOTTLE_CONTAINER)               \
	           --env-file ../secrets/claude.env                       \
	           $(RBEV__ALL)                                           \
	           --dns $(RBEV_SENTRY_GUARDED_IP)                        \
	           -v $(RBEV_BOTTLE_MOUNT_DIR):$(RBEV_BOTTLE_WORKDIR):Z   \
	           --privileged                                           \
	  $(RBEV_BOTTLE_FQIN) > $(zRBM_LAST_BOTTLE_CONTAINER_FACTFILE)
	$(zRBM_STEP) "Pulling logs..."
	podman logs $$(cat $(zRBM_LAST_SENTRY_CONTAINER_FACTFILE)) > $(zRBM_LAST_SENTRY_LOGS_FACTFILE) 2>&1
	podman logs $$(cat $(zRBM_LAST_BOTTLE_CONTAINER_FACTFILE)) > $(zRBM_LAST_BOTTLE_LOGS_FACTFILE) 2>&1
	$(zRBM_STEP) "Inspecting the guarded network..."
	podman network inspect $(zRBM_GUARDED_NETWORK)
	$(zRBM_STEP) "Setup complete... Find jupyter at:"
	$(MBC_SHOW_WHITE)
	$(MBC_SHOW_YELLOW) "    -> http://$(zRBM_LOCALHOST_IP):$(RBEV_SENTRY_JUPYTER_PORT)/lab"
	@echo http://$(zRBM_LOCALHOST_IP):$(RBEV_SENTRY_JUPYTER_PORT)/lab | clip
	$(MBC_SHOW_WHITE)


rbm-Ts%: zrbm_argcheck_rule
	$(zRBM_START) "TEST SENTRY ASPECTS OF SERVICE"
	$(MBC_PASS) "No current tests."


rbm-Tb%: zrbm_argcheck_rule
	$(zRBM_START) "TEST BOTTLE ASPECTS OF SERVICE"
	$(zRBM_STEP) "Test 0: Verifying DNS forwarding..."
	podman exec $(zRBM_BOTTLE_CONTAINER) nslookup api.anthropic.com || (echo "FAIL: BOTTLE unable to resolve critical domain name" && exit 1)
	$(zRBM_STEP) "Test 1: Checking if BOTTLE can reach allowed domains..."
	podman exec $(zRBM_BOTTLE_CONTAINER) sh -c 'curl https://api.anthropic.com/v1/messages \
	  --header "x-api-key: $$ANTHROPIC_API_KEY" \
	  --header "anthropic-version: 2023-06-01" \
	  --header "content-type: application/json" \
	  --data '"'"'{"model": "claude-3-5-sonnet-20240620", "max_tokens": 1024, "messages": [{"role": "user", "content": "Hello, Claude"}]}'"'"''
	$(zRBM_STEP) "Test 2: Verifying Jupyter accessibility..."
	curl -s -o /dev/null -w "%{http_code}" http://localhost:$(zRBM_SENTRY_JUPYTER_PORT) | grep 200 || (echo "FAIL: Jupyter not accessible" && exit 1)
	$(zRBM_STEP) "All security tests passed successfully."


rbm-cb%: zrbm_argcheck_rule
	$(MBC_SHOW_WHITE) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to BOTTLE"
	podman exec -it $(zRBM_BOTTLE_CONTAINER) /bin/bash
	$(MBC_PASS) "Done, no errors."


rbm-cs%: zrbm_argcheck_rule
	$(MBC_SHOW_WHITE) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to SENTRY"
	podman exec -it $(zRBM_SENTRY_CONTAINER) /bin/sh
	$(MBC_PASS) "Done, no errors."


# EOF
