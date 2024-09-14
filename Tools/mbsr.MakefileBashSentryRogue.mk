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
MBSR_ARG_MONIKER = ""

# If provided, the makefile that defines console variables.  Otherwise, no include
MBSR_ARG_SUBMAKE_MBC =

# Reminders about future improvements, deferred for now:
#
# OUCH Get real clear if I think a docker container advertising a port gets external network view
# OUCH Make the startup script for rogue standalone external too
# OUCH Cut down the diagnostic IP utilities found all over the place
# OUCH Refresh whether rogue --privileged is required for function (and cut if not)
# OUCH Cut outreach from the matplotlib container
# OUCH put pip after apk install
# OUCH Cut startup script 5 second hack, probably with atomizing startup sequence
# OUCH rename dockerfile to recipe or containerfile...  Does this make the environment a 'cake'?  I think it does...
# OUCH try multiline docker arg/env for bettermaint
# OUCH bettername for 'instance of application' for crisp docs.  'environment'?
# OUCH make plantuml server variant that works with -> https://www.plantuml.com/plantuml/uml/SyfFKj2rKt3CoKnELR1Io4ZDoSa70000 or http://localhost:8080/plantuml/png/SyfFKj2rKt3CoKnELR1Io4ZDoSa70000 blended
# OUCH decide if sentry socat should be in a separate startup script rather than dockerfile (prolly with atomization, yes)
# OUCH make pretty non-internet jupyter maths variant
# OUCH make integ check buried jupyter (i.e. flask executor only) with internet variant
# OUCH There may be parameters in the dockerfile that are simpler out here
# OUCH consider rename all to BANDIT or PIRATE for ROGUE
# OUCH cut the network diagnostic that is in the build context
# OUCH strip back dnsmasq to _only_ serve the anthropic.com (or similar)
# OUCH figure out how to inscribe parameters during build so they don't confuse (or risk) the environment later
# OUCH Sort out tests: automatic during build/setup, or explicit?  Not all over the place
# OUCH consistify WORKDIR
# OUCH tee with error capture all the catted in scripts
# OUCH make default dockerfile arg/env values really illegal
# OUCH consider factoring out podman stop and then giving it a zero timeout; prolly ought to manually stop and check logs for why its slow
# OUCH decide whether to mononame the docker env/arg to the makefile name, or drift it.  Do it.
# OUCH scrub out dockerfile environment variables unneeded in final env?
# OUCH Rethink the .sh-ization of the nameplate, specifically for including includes
# OUCH really consider deprivilidgening ROGUE
# OUCH consider a precedent rule that in parallel will stop containers before rmi and use in build and start
# OUCH make choices concerning secondary port opens for the jupyter flask executor on top of jupyter
# OUCH figure suppression of error -> time="2024-08-30T06:54:29-07:00" level=warning msg="Failed to obtain TTY size: The handle is invalid."
# OUCH segregate nameplate checking to a subfile and explain there the unusability of a rule string for include directive
# OUCH Something not sitting right about machenations around the ALL target in above makefile, and double console include, and mbc path include
# OUCH seriously consider making tabtarget names in this file more like zmbsr_xxx_rule and then hardcoding tabtarget to pass var, better for nesting

# Internal variables start with 'z' for easy identification

zMBSR_ME := $(abspath $(lastword $(MAKEFILE_LIST)))
zMBSR_MAKE = $(MAKE) -f $(zMBSR_ME)

zMBSR_LOCALHOST_IP = 127.0.0.1

zMBSR_NAMEPLATE_DIR     = ./MBSR-nameplates
zMBSR_DOCKERFILE_DIR    = ./MBSR-dockerfiles
zMBSR_BUILD_CONTEXT_DIR = ./MBSR-build-context
zMBSR_TRANSCRIPTS_DIR   = ./MBSR-transcripts
zMBSR_SCRIPTS_DIR       = ./MBSR-scripts

zMBSR_NAMEPLATE_FILE     = $(zMBSR_NAMEPLATE_DIR)/nameplate.$(MBSR_ARG_MONIKER).sh


# Argument is path to the console rules to allow this makefile to be sub-make'd not included
ifneq ($(strip $(MBSR_ARG_SUBMAKE_MBC)),)
include        $(MBSR_ARG_SUBMAKE_MBC)
endif

-include $(zMBSR_NAMEPLATE_FILE)

# Network and interface variables
zMBSR_GUARDED_NETMASK          = 16
zMBSR_GUARDED_NETWORK_SUBNET   = $(MBSR_NAMEPLATE_IP_HACK).0.0/$(zMBSR_GUARDED_NETMASK)
zMBSR_HOST_GATEWAY             = $(MBSR_NAMEPLATE_IP_HACK).0.1
zMBSR_SENTRY_GUARDED_IP        = $(MBSR_NAMEPLATE_IP_HACK).0.2
zMBSR_ROGUE_IP                 = $(MBSR_NAMEPLATE_IP_HACK).0.3
zMBSR_SENTRY_HOST_INTERFACE    = eth0
zMBSR_SENTRY_GUARDED_INTERFACE = eth1
zMBSR_SENTRY_JUPYTER_PORT      = $(MBSR_NAMEPLATE_PORT_HOST)
zMBSR_ROGUE_JUPYTER_PORT       = $(MBSR_NAMEPLATE_PORT_GUARDED)
zMBSR_ROGUE_WORKDIR            = $(MBSR_NAMEPLATE_APP_INNER_DIR)
zMBSR_ROGUE_MOUNT_DIR          = $(MBSR_NAMEPLATE_APP_OUTER_DIR)

zMBSR_STEP = $(MBC_SHOW_WHITE) "Moniker:"$(MBSR_ARG_MONIKER)

zMBSR_SENTRY_DOCKERFILE = $(zMBSR_DOCKERFILE_DIR)/$(MBSR_ARG_MONIKER).sentry.dockerfile
zMBSR_ROGUE_DOCKERFILE  = $(zMBSR_DOCKERFILE_DIR)/$(MBSR_ARG_MONIKER).rogue.dockerfile

zMBSR_SENTRY_IMAGE = $(MBSR_ARG_MONIKER)-sentry-image:$(MBSR_ARG_MONIKER)
zMBSR_ROGUE_IMAGE  = $(MBSR_ARG_MONIKER)-rogue-image

zMBSR_SENTRY_CONTAINER = $(MBSR_ARG_MONIKER)-sentry-container
zMBSR_ROGUE_CONTAINER  = $(MBSR_ARG_MONIKER)-rogue-container

zMBSR_HOST_NETWORK    = $(MBSR_ARG_MONIKER)-host-network
zMBSR_GUARDED_NETWORK = $(MBSR_ARG_MONIKER)-guarded-network

zMBSR_LAST_SENTRY_CONTAINER_FACTFILE = $(zMBSR_TRANSCRIPTS_DIR)/container.$(MBSR_ARG_MONIKER).sentry.txt
zMBSR_LAST_ROGUE_CONTAINER_FACTFILE  = $(zMBSR_TRANSCRIPTS_DIR)/container.$(MBSR_ARG_MONIKER).rogue.txt
zMBSR_LAST_SENTRY_LOGS_FACTFILE      = $(zMBSR_TRANSCRIPTS_DIR)/logs.$(MBSR_ARG_MONIKER).sentry.txt
zMBSR_LAST_ROGUE_LOGS_FACTFILE       = $(zMBSR_TRANSCRIPTS_DIR)/logs.$(MBSR_ARG_MONIKER).rogue.txt
ZMBSR_LAST_SENTRY_BUILD_FACTFILE     = $(zMBSR_TRANSCRIPTS_DIR)/build.$(MBSR_ARG_MONIKER).sentry.txt
ZMBSR_LAST_ROGUE_BUILD_FACTFILE      = $(zMBSR_TRANSCRIPTS_DIR)/build.$(MBSR_ARG_MONIKER).rogue.txt

zMBSR_DNS       = 8.8.8.8

zMBSR_ARGCHECK_NONZERO_CMD = test -n "$(MBSR_ARG_MONIKER)"                      || (\
  $(MBC_SEE_RED) "Error: MBSR_ARG_MONIKER must be set in the tabtarget."    &&\
  exit 1)

zMBSR_ARGCHECK_NAMEPLATE_CMD = test "$(MBSR_ARG_MONIKER)" = "$(MBSR_NAMEPLATE_MONIKER)" || (\
  $(MBC_SEE_RED) "Error: Rule only works if proper moniker selection.  Mismatch:"   &&\
  $(MBC_SEE_RED) "      MBSR_ARG_MONIKER       =" $(MBSR_ARG_MONIKER)               &&\
  $(MBC_SEE_RED) "      MBSR_NAMEPLATE_MONIKER =" $(MBSR_NAMEPLATE_MONIKER)         &&\
  exit 1)

zmbsr_argcheck_rule:
	@echo MBSR_ARG_MONIKER is $(MBSR_ARG_MONIKER)
	@echo MBSR_NAMEPLATE_MONIKER is $(MBSR_NAMEPLATE_MONIKER)
	@$(zMBSR_ARGCHECK_NONZERO_CMD)
	@$(zMBSR_ARGCHECK_NAMEPLATE_CMD)

mbsr-i__Info.%.sh: zmbsr_argcheck_rule
	$(MBC_PASS) "Done, no errors."

mbsr-B__BuildImages.%.sh: zmbsr_argcheck_rule
	$(zMBSR_STEP) "Cleaning up previous runs..."
	-podman stop  $(zMBSR_SENTRY_CONTAINER) $(zMBSR_ROGUE_CONTAINER) || true
	-podman rm -f $(zMBSR_SENTRY_CONTAINER) $(zMBSR_ROGUE_CONTAINER) || true
	$(zMBSR_STEP)  "Building image"               $(zMBSR_SENTRY_IMAGE) "..."
	-podman rmi -f                                $(zMBSR_SENTRY_IMAGE)
	podman build -f $(zMBSR_SENTRY_DOCKERFILE) -t $(zMBSR_SENTRY_IMAGE)  \
	  --build-arg NAMEPLATE_MONIKER=$(MBSR_NAMEPLATE_MONIKER)            \
	  --build-arg DNS_SERVER=$(zMBSR_DNS)                                \
	  --build-arg NETWORK_MASK=$(zMBSR_GUARDED_NETMASK)                  \
	  --build-arg ROGUE_IP=$(zMBSR_ROGUE_IP)                             \
	  --build-arg ROGUE_JUPYTER_PORT=$(zMBSR_ROGUE_JUPYTER_PORT)         \
	  --build-arg SENTRY_JUPYTER_PORT=$(zMBSR_SENTRY_JUPYTER_PORT)       \
	  --build-arg GUARDED_INTERFACE=$(zMBSR_SENTRY_GUARDED_INTERFACE)    \
	  --build-arg HOST_INTERFACE=$(zMBSR_SENTRY_HOST_INTERFACE)          \
	  --build-arg SENTRY_GUARDED_IP=$(zMBSR_SENTRY_GUARDED_IP)           \
	  --build-arg GUARDED_NETWORK_SUBNET=$(zMBSR_GUARDED_NETWORK_SUBNET) \
	  --progress=plain                                                   \
	  $(zMBSR_BUILD_CONTEXT_DIR)      > $(ZMBSR_LAST_SENTRY_BUILD_FACTFILE)  2>&1
	$(zMBSR_STEP)  "Building image"              $(zMBSR_ROGUE_IMAGE) "..."
	-podman rmi -f                               $(zMBSR_ROGUE_IMAGE)
	podman build -f $(zMBSR_ROGUE_DOCKERFILE) -t $(zMBSR_ROGUE_IMAGE)    \
	  --build-arg NAMEPLATE_MONIKER=$(MBSR_NAMEPLATE_MONIKER)            \
	  --build-arg ROGUE_IP=$(zMBSR_ROGUE_IP)                             \
	  --build-arg JUPYTER_PORT=$(zMBSR_ROGUE_JUPYTER_PORT)               \
	  --build-arg ROGUE_WORKDIR=$(zMBSR_ROGUE_WORKDIR)                   \
	  --build-arg GUARDED_INTERFACE=$(zMBSR_SENTRY_GUARDED_INTERFACE)    \
	  --build-arg SENTRY_GUARDED_IP=$(zMBSR_SENTRY_GUARDED_IP)           \
	  --build-arg GUARDED_NETWORK_SUBNET=$(zMBSR_GUARDED_NETWORK_SUBNET) \
	  --progress=plain                                                   \
	  $(zMBSR_BUILD_CONTEXT_DIR)      > $(ZMBSR_LAST_ROGUE_BUILD_FACTFILE)   2>&1
	$(MBC_PASS) "Done, no errors."

mbsr-s__StartContainers.%.sh: zmbsr_argcheck_rule
	$(zMBSR_STEP) "Cleaning up previous runs..."
	-podman stop  $(zMBSR_SENTRY_CONTAINER) $(zMBSR_ROGUE_CONTAINER) || true
	-podman rm -f $(zMBSR_SENTRY_CONTAINER) $(zMBSR_ROGUE_CONTAINER) || true
	-podman network rm $(zMBSR_HOST_NETWORK)    || true
	-podman network rm $(zMBSR_GUARDED_NETWORK) || true
	$(zMBSR_STEP) "Creating networks..."
	podman network create --driver bridge $(zMBSR_HOST_NETWORK)
	podman network create --subnet $(zMBSR_GUARDED_NETWORK_SUBNET) \
	  --gateway $(zMBSR_SENTRY_GUARDED_IP) \
	  --internal \
	  $(zMBSR_GUARDED_NETWORK)
	$(zMBSR_STEP) "Running the Sentry container with host network..."
	podman run -d --name $(zMBSR_SENTRY_CONTAINER) \
	  --network $(zMBSR_HOST_NETWORK) \
	  --env-file ../secrets/claude.env \
	  -p $(zMBSR_LOCALHOST_IP):$(zMBSR_SENTRY_JUPYTER_PORT):$(zMBSR_SENTRY_JUPYTER_PORT) \
	  --privileged \
	  $(zMBSR_SENTRY_IMAGE) > $(zMBSR_LAST_SENTRY_CONTAINER_FACTFILE)
	$(zMBSR_STEP) "Executing host setup script..."
	cat $(zMBSR_SCRIPTS_DIR)/sentry-setup-host.sh | podman exec -i $(zMBSR_SENTRY_CONTAINER) /bin/sh
	$(zMBSR_STEP) "Attaching guarded network to Sentry container..."
	podman network connect $(zMBSR_GUARDED_NETWORK) $(zMBSR_SENTRY_CONTAINER) --ip $(zMBSR_SENTRY_GUARDED_IP)
	$(zMBSR_STEP) "Executing guarded setup script..."
	cat $(zMBSR_SCRIPTS_DIR)/sentry-setup-guarded.sh | podman exec -i $(zMBSR_SENTRY_CONTAINER) /bin/sh
	$(zMBSR_STEP) "Executing service setup script..."
	cat $(zMBSR_SCRIPTS_DIR)/sentry-setup-service.sh | podman exec -i $(zMBSR_SENTRY_CONTAINER) /bin/sh
	$(zMBSR_STEP) "Executing outreach setup script..."
	cat $(zMBSR_SCRIPTS_DIR)/sentry-setup-outreach.sh | podman exec -i $(zMBSR_SENTRY_CONTAINER) /bin/sh
	$(zMBSR_STEP) "Running the Rogue container..."
	podman run -d --name $(zMBSR_ROGUE_CONTAINER) \
	  --network $(zMBSR_GUARDED_NETWORK):ip=$(zMBSR_ROGUE_IP) \
	  --network-alias $(zMBSR_ROGUE_CONTAINER) \
	  --env-file ../secrets/claude.env \
	  --dns $(zMBSR_SENTRY_GUARDED_IP) \
	  -v $(zMBSR_ROGUE_MOUNT_DIR):$(zMBSR_ROGUE_WORKDIR):Z \
	  --privileged \
	  $(zMBSR_ROGUE_IMAGE) > $(zMBSR_LAST_ROGUE_CONTAINER_FACTFILE)
	$(zMBSR_STEP) "Pulling logs..."
	podman logs $$(cat $(zMBSR_LAST_SENTRY_CONTAINER_FACTFILE)) > $(zMBSR_LAST_SENTRY_LOGS_FACTFILE) 2>&1
	podman logs $$(cat $(zMBSR_LAST_ROGUE_CONTAINER_FACTFILE)) > $(zMBSR_LAST_ROGUE_LOGS_FACTFILE) 2>&1
	$(zMBSR_STEP) "Inspecting the guarded network..."
	podman network inspect $(zMBSR_GUARDED_NETWORK)
	$(zMBSR_STEP) "Setup complete... Find jupyter at:"
	$(MBC_SHOW_WHITE)
	$(MBC_SHOW_YELLOW) "    -> http://$(zMBSR_LOCALHOST_IP):$(zMBSR_SENTRY_JUPYTER_PORT)/lab"
	@echo http://$(zMBSR_LOCALHOST_IP):$(zMBSR_SENTRY_JUPYTER_PORT)/lab | clip
	$(MBC_SHOW_WHITE)
# OUCH consider if keep parse of -> $ curl -v -s -I -X OPTIONS https://api.anthropic.com/v1/messages
# OUCH decide what to keep of below

mbsr-tr__TestRogue.%.sh: zmbsr_argcheck_rule
	$(zMBSR_STEP) "Test 0: Verifying DNS forwarding..."
	podman exec $(zMBSR_ROGUE_CONTAINER) nslookup api.anthropic.com || (echo "FAIL: ROGUE unable to resolve critical domain name" && exit 1)
	$(zMBSR_STEP) "Test 1: Checking if ROGUE can reach allowed domains..."
	podman exec $(zMBSR_ROGUE_CONTAINER) sh -c 'curl https://api.anthropic.com/v1/messages \
	  --header "x-api-key: $$ANTHROPIC_API_KEY" \
	  --header "anthropic-version: 2023-06-01" \
	  --header "content-type: application/json" \
	  --data '"'"'{"model": "claude-3-5-sonnet-20240620", "max_tokens": 1024, "messages": [{"role": "user", "content": "Hello, Claude"}]}'"'"''
	$(zMBSR_STEP) "Test 2: Verifying Jupyter accessibility..."
	curl -s -o /dev/null -w "%{http_code}" http://localhost:$(zMBSR_SENTRY_JUPYTER_PORT) | grep 200 || (echo "FAIL: Jupyter not accessible" && exit 1)
	$(zMBSR_STEP) "All security tests passed successfully."

mbsr-ts__TestSentry.%.sh: zmbsr_argcheck_rule
	$(MBC_PASS) "No current tests."

mbsr-cr__ConnectRogue.%.sh: zmbsr_argcheck_rule
	$(MBC_SHOW_WHITE) "Moniker:"$(MBSR_ARG_MONIKER) "Connecting to ROGUE"
	podman exec -it $(zMBSR_ROGUE_CONTAINER) /bin/bash
	$(MBC_PASS) "Done, no errors."

mbsr-cs__ConnectSentry.%.sh: zmbsr_argcheck_rule
	$(MBC_SHOW_WHITE) "Moniker:"$(MBSR_ARG_MONIKER) "Connecting to SENTRY"
	podman exec -it $(zMBSR_SENTRY_CONTAINER) /bin/sh
	$(MBC_PASS) "Done, no errors."


# EOF
