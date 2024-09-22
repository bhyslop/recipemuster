## ©️ 2024 Scale Invariant, Inc.  All rights reserved.
##      Reference: https://www.termsfeed.com/blog/sample-copyright-notices/
##
## Unauthorized copying of this file, via any medium is strictly prohibited
## Proprietary and confidential
##
## Written by Brad Hyslop <bhyslop@scaleinvariant.org> August 2024


##############################################
# Sentry and Rogue Experiment
#
#  This is a makefile snippet that implements critical safety mechanisms for using
#  untrusted code in a secure environment.
#
#  Find below a concept where I am creating two docker containers, SENTRY and ROGUE.
#
#  The one named ROGUE is a container that runs untrusted code from the internet.
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

# If provided, the makefile that defines console variables.  Otherwise, no include
RBM_ARG_SUBMAKE_MBC ?=


# Internal variables start with 'z' for easy identification

zRBM_ME := $(abspath $(lastword $(MAKEFILE_LIST)))
zRBM_MAKE = $(MAKE) -f $(zRBM_ME)

zRBM_LOCALHOST_IP = 127.0.0.1

zRBN_DIR     = ./RBM-nameplates
zRBM_DOCKERFILE_DIR    = ./RBM-dockerfiles
zRBM_BUILD_CONTEXT_DIR = ./RBM-build-context
zRBM_TRANSCRIPTS_DIR   = ./RBM-transcripts
zRBM_SCRIPTS_DIR       = ./RBM-scripts

zRBN_FILE     = $(zRBN_DIR)/nameplate.$(RBM_ARG_MONIKER).sh


# Argument is path to the console rules to allow this makefile to be sub-make'd not included
ifneq ($(strip $(RBM_ARG_SUBMAKE_MBC)),)
include        $(RBM_ARG_SUBMAKE_MBC)
endif

-include $(zRBN_FILE)

# Network and interface variables
zRBM_GUARDED_NETMASK          = 16
zRBM_GUARDED_NETWORK_SUBNET   = $(RBN_IP_HACK).0.0/$(zRBM_GUARDED_NETMASK)
zRBM_HOST_GATEWAY             = $(RBN_IP_HACK).0.1
zRBM_SENTRY_GUARDED_IP        = $(RBN_IP_HACK).0.2
zRBM_ROGUE_IP                 = $(RBN_IP_HACK).0.3
zRBM_SENTRY_HOST_INTERFACE    = eth0
zRBM_SENTRY_GUARDED_INTERFACE = eth1
zRBM_SENTRY_JUPYTER_PORT      = $(RBN_PORT_HOST)
zRBM_ROGUE_JUPYTER_PORT       = $(RBN_PORT_GUARDED)
zRBM_ROGUE_WORKDIR            = $(RBN_APP_INNER_DIR)
zRBM_ROGUE_MOUNT_DIR          = $(RBN_APP_OUTER_DIR)

zRBM_START = $(MBC_SHOW_WHITE) "Moniker:"$(RBM_ARG_MONIKER)
zRBM_STEP  = $(MBC_SHOW_WHITE) "Moniker:"$(RBM_ARG_MONIKER)

zRBM_SENTRY_DOCKERFILE = $(zRBM_DOCKERFILE_DIR)/$(RBM_ARG_MONIKER).sentry.dockerfile
zRBM_ROGUE_DOCKERFILE  = $(zRBM_DOCKERFILE_DIR)/$(RBM_ARG_MONIKER).rogue.dockerfile

zRBM_SENTRY_IMAGE = $(RBM_ARG_MONIKER)-sentry-image:$(RBM_ARG_MONIKER)
zRBM_ROGUE_IMAGE  = $(RBM_ARG_MONIKER)-rogue-image

zRBM_SENTRY_CONTAINER = $(RBM_ARG_MONIKER)-sentry-container
zRBM_ROGUE_CONTAINER  = $(RBM_ARG_MONIKER)-rogue-container

zRBM_HOST_NETWORK    = $(RBM_ARG_MONIKER)-host-network
zRBM_GUARDED_NETWORK = $(RBM_ARG_MONIKER)-guarded-network

zRBM_LAST_SENTRY_CONTAINER_FACTFILE = $(zRBM_TRANSCRIPTS_DIR)/container.$(RBM_ARG_MONIKER).sentry.txt
zRBM_LAST_ROGUE_CONTAINER_FACTFILE  = $(zRBM_TRANSCRIPTS_DIR)/container.$(RBM_ARG_MONIKER).rogue.txt
zRBM_LAST_SENTRY_LOGS_FACTFILE      = $(zRBM_TRANSCRIPTS_DIR)/logs.$(RBM_ARG_MONIKER).sentry.txt
zRBM_LAST_ROGUE_LOGS_FACTFILE       = $(zRBM_TRANSCRIPTS_DIR)/logs.$(RBM_ARG_MONIKER).rogue.txt
ZRBM_LAST_SENTRY_BUILD_FACTFILE     = $(zRBM_TRANSCRIPTS_DIR)/build.$(RBM_ARG_MONIKER).sentry.txt
ZRBM_LAST_ROGUE_BUILD_FACTFILE      = $(zRBM_TRANSCRIPTS_DIR)/build.$(RBM_ARG_MONIKER).rogue.txt

zRBM_DNS       = 8.8.8.8

zRBM_ARGCHECK_NONZERO_CMD = test -n "$(RBM_ARG_MONIKER)"                      || (\
  $(MBC_SEE_RED) "Error: RBM_ARG_MONIKER must be set in the tabtarget."    &&\
  exit 1)

zRBM_ARGCHECK_NAMEPLATE_CMD = test "$(RBM_ARG_MONIKER)" = "$(RBN_MONIKER)" || (\
  $(MBC_SEE_RED) "Error: Rule only works if proper moniker selection.  Mismatch:"   &&\
  $(MBC_SEE_RED) "      RBM_ARG_MONIKER       =" $(RBM_ARG_MONIKER)                 &&\
  $(MBC_SEE_RED) "      RBN_MONIKER =" $(RBN_MONIKER)           &&\
  exit 1)

zrbm_argcheck_rule:
	@echo RBM_ARG_MONIKER is $(RBM_ARG_MONIKER)
	@echo RBN_MONIKER is $(RBN_MONIKER)
	@$(zRBM_ARGCHECK_NONZERO_CMD)
	@$(zRBM_ARGCHECK_NAMEPLATE_CMD)

rbm-i.%.sh: zrbm_argcheck_rule
	$(MBC_PASS) "Done, no errors."


rbm-BL.%: zrbm_argcheck_rule
	$(zRBM_START) "Start building recipes locally..."
	$(zRBM_STEP) "Cleaning up previous runs..."
	-podman stop  $(zRBM_SENTRY_CONTAINER) $(zRBM_ROGUE_CONTAINER) || true
	-podman rm -f $(zRBM_SENTRY_CONTAINER) $(zRBM_ROGUE_CONTAINER) || true
	$(zRBM_STEP)  "Building image"               $(zRBM_SENTRY_IMAGE) "..."
	-podman rmi -f                               $(zRBM_SENTRY_IMAGE)
	podman build -f $(zRBM_SENTRY_DOCKERFILE) -t $(zRBM_SENTRY_IMAGE)   \
	  --build-arg NAMEPLATE_MONIKER=$(RBN_MONIKER)            \
	  --build-arg DNS_SERVER=$(zRBM_DNS)                                \
	  --build-arg NETWORK_MASK=$(zRBM_GUARDED_NETMASK)                  \
	  --build-arg ROGUE_IP=$(zRBM_ROGUE_IP)                             \
	  --build-arg ROGUE_JUPYTER_PORT=$(zRBM_ROGUE_JUPYTER_PORT)         \
	  --build-arg SENTRY_JUPYTER_PORT=$(zRBM_SENTRY_JUPYTER_PORT)       \
	  --build-arg GUARDED_INTERFACE=$(zRBM_SENTRY_GUARDED_INTERFACE)    \
	  --build-arg HOST_INTERFACE=$(zRBM_SENTRY_HOST_INTERFACE)          \
	  --build-arg SENTRY_GUARDED_IP=$(zRBM_SENTRY_GUARDED_IP)           \
	  --build-arg GUARDED_NETWORK_SUBNET=$(zRBM_GUARDED_NETWORK_SUBNET) \
	  --progress=plain                                                   \
	  $(zRBM_BUILD_CONTEXT_DIR)      > $(ZRBM_LAST_SENTRY_BUILD_FACTFILE)  2>&1
	$(zRBM_STEP)  "Building image"              $(zRBM_ROGUE_IMAGE) "..."
	-podman rmi -f                               $(zRBM_ROGUE_IMAGE)
	podman build -f $(zRBM_ROGUE_DOCKERFILE) -t $(zRBM_ROGUE_IMAGE)    \
	  --build-arg NAMEPLATE_MONIKER=$(RBN_MONIKER)            \
	  --build-arg ROGUE_IP=$(zRBM_ROGUE_IP)                             \
	  --build-arg JUPYTER_PORT=$(zRBM_ROGUE_JUPYTER_PORT)               \
	  --build-arg ROGUE_WORKDIR=$(zRBM_ROGUE_WORKDIR)                   \
	  --build-arg GUARDED_INTERFACE=$(zRBM_SENTRY_GUARDED_INTERFACE)    \
	  --build-arg SENTRY_GUARDED_IP=$(zRBM_SENTRY_GUARDED_IP)           \
	  --build-arg GUARDED_NETWORK_SUBNET=$(zRBM_GUARDED_NETWORK_SUBNET) \
	  --progress=plain                                                   \
	  $(zRBM_BUILD_CONTEXT_DIR)      > $(ZRBM_LAST_ROGUE_BUILD_FACTFILE)   2>&1
	$(MBC_PASS) "Done, no errors."

rbm-s.%: zrbm_argcheck_rule
	$(zRBM_STEP) "Cleaning up previous runs..."
	-podman stop  $(zRBM_SENTRY_CONTAINER) $(zRBM_ROGUE_CONTAINER) || true
	-podman rm -f $(zRBM_SENTRY_CONTAINER) $(zRBM_ROGUE_CONTAINER) || true
	-podman network rm $(zRBM_HOST_NETWORK)    || true
	-podman network rm $(zRBM_GUARDED_NETWORK) || true
	$(zRBM_STEP) "Creating networks..."
	podman network create --driver bridge $(zRBM_HOST_NETWORK)
	podman network create --subnet $(zRBM_GUARDED_NETWORK_SUBNET) \
	  --gateway $(zRBM_SENTRY_GUARDED_IP) \
	  --internal \
	  $(zRBM_GUARDED_NETWORK)
	$(zRBM_STEP) "Running the Sentry container with host network..."
	podman run -d --name $(zRBM_SENTRY_CONTAINER) \
	  --network $(zRBM_HOST_NETWORK) \
	  --env-file ../secrets/claude.env \
	  -p $(zRBM_LOCALHOST_IP):$(zRBM_SENTRY_JUPYTER_PORT):$(zRBM_SENTRY_JUPYTER_PORT) \
	  --privileged \
	  $(zRBM_SENTRY_IMAGE) > $(zRBM_LAST_SENTRY_CONTAINER_FACTFILE)
	$(zRBM_STEP) "Executing host setup script..."
	cat $(zRBM_SCRIPTS_DIR)/sentry-setup-host.sh | podman exec -i $(zRBM_SENTRY_CONTAINER) /bin/sh
	$(zRBM_STEP) "Attaching guarded network to Sentry container..."
	podman network connect $(zRBM_GUARDED_NETWORK) $(zRBM_SENTRY_CONTAINER) --ip $(zRBM_SENTRY_GUARDED_IP)
	$(zRBM_STEP) "Executing guarded setup script..."
	cat $(zRBM_SCRIPTS_DIR)/sentry-setup-guarded.sh | podman exec -i $(zRBM_SENTRY_CONTAINER) /bin/sh
	$(zRBM_STEP) "Executing service setup script..."
	cat $(zRBM_SCRIPTS_DIR)/sentry-setup-service.sh | podman exec -i $(zRBM_SENTRY_CONTAINER) /bin/sh
	$(zRBM_STEP) "Executing outreach setup script..."
	cat $(zRBM_SCRIPTS_DIR)/sentry-setup-outreach.sh | podman exec -i $(zRBM_SENTRY_CONTAINER) /bin/sh
	$(zRBM_STEP) "Running the Rogue container..."
	podman run -d --name $(zRBM_ROGUE_CONTAINER) \
	  --network $(zRBM_GUARDED_NETWORK):ip=$(zRBM_ROGUE_IP) \
	  --network-alias $(zRBM_ROGUE_CONTAINER) \
	  --env-file ../secrets/claude.env \
	  --dns $(zRBM_SENTRY_GUARDED_IP) \
	  -v $(zRBM_ROGUE_MOUNT_DIR):$(zRBM_ROGUE_WORKDIR):Z \
	  --privileged \
	  $(zRBM_ROGUE_IMAGE) > $(zRBM_LAST_ROGUE_CONTAINER_FACTFILE)
	$(zRBM_STEP) "Pulling logs..."
	podman logs $$(cat $(zRBM_LAST_SENTRY_CONTAINER_FACTFILE)) > $(zRBM_LAST_SENTRY_LOGS_FACTFILE) 2>&1
	podman logs $$(cat $(zRBM_LAST_ROGUE_CONTAINER_FACTFILE)) > $(zRBM_LAST_ROGUE_LOGS_FACTFILE) 2>&1
	$(zRBM_STEP) "Inspecting the guarded network..."
	podman network inspect $(zRBM_GUARDED_NETWORK)
	$(zRBM_STEP) "Setup complete... Find jupyter at:"
	$(MBC_SHOW_WHITE)
	$(MBC_SHOW_YELLOW) "    -> http://$(zRBM_LOCALHOST_IP):$(zRBM_SENTRY_JUPYTER_PORT)/lab"
	@echo http://$(zRBM_LOCALHOST_IP):$(zRBM_SENTRY_JUPYTER_PORT)/lab | clip
	$(MBC_SHOW_WHITE)
# OUCH consider if keep parse of -> $ curl -v -s -I -X OPTIONS https://api.anthropic.com/v1/messages
# OUCH decide what to keep of below

rbm-tr__TestRogue.%.sh: zrbm_argcheck_rule
	$(zRBM_STEP) "Test 0: Verifying DNS forwarding..."
	podman exec $(zRBM_ROGUE_CONTAINER) nslookup api.anthropic.com || (echo "FAIL: ROGUE unable to resolve critical domain name" && exit 1)
	$(zRBM_STEP) "Test 1: Checking if ROGUE can reach allowed domains..."
	podman exec $(zRBM_ROGUE_CONTAINER) sh -c 'curl https://api.anthropic.com/v1/messages \
	  --header "x-api-key: $$ANTHROPIC_API_KEY" \
	  --header "anthropic-version: 2023-06-01" \
	  --header "content-type: application/json" \
	  --data '"'"'{"model": "claude-3-5-sonnet-20240620", "max_tokens": 1024, "messages": [{"role": "user", "content": "Hello, Claude"}]}'"'"''
	$(zRBM_STEP) "Test 2: Verifying Jupyter accessibility..."
	curl -s -o /dev/null -w "%{http_code}" http://localhost:$(zRBM_SENTRY_JUPYTER_PORT) | grep 200 || (echo "FAIL: Jupyter not accessible" && exit 1)
	$(zRBM_STEP) "All security tests passed successfully."

rbm-ts__TestSentry.%.sh: zrbm_argcheck_rule
	$(MBC_PASS) "No current tests."

rbm-cr__ConnectRogue.%.sh: zrbm_argcheck_rule
	$(MBC_SHOW_WHITE) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to ROGUE"
	podman exec -it $(zRBM_ROGUE_CONTAINER) /bin/bash
	$(MBC_PASS) "Done, no errors."

rbm-cs__ConnectSentry.%.sh: zrbm_argcheck_rule
	$(MBC_SHOW_WHITE) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to SENTRY"
	podman exec -it $(zRBM_SENTRY_CONTAINER) /bin/sh
	$(MBC_PASS) "Done, no errors."


# EOF
