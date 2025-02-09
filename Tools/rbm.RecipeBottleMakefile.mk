## � 2024 Scale Invariant.  All rights reserved.
##      Reference: https://www.termsfeed.com/blog/sample-copyright-notices/
##
## Unauthorized copying of this file, via any medium is strictly prohibited
## Proprietary and confidential
##
## Written by Brad Hyslop <bhyslop@scaleinvariant.org> November 2024

# Recipe Bottle Makefile (RBM)
# Implements secure containerized service management

SHELL := /bin/bash


# Directory structure
RBM_TOOLS_DIR        := Tools
RBM_TRANSCRIPTS_DIR  := RBM-transcripts

# Required argument for service moniker
RBM_MONIKER ?= __MUST_DEFINE_MONIKER__

include ../RBS_STATION.mk
include rbb.base.mk

# File paths
RBM_NAMEPLATE_PATH = $(RBB_NAMEPLATE_PATH)/nameplate.$(RBM_MONIKER).mk
RBM_SENTRY_LOG     = $(RBM_TRANSCRIPTS_DIR)/sentry.$(RBM_MONIKER).log
RBM_BOTTLE_LOG     = $(RBM_TRANSCRIPTS_DIR)/bottle.$(RBM_MONIKER).log

# May not be populated, depending upon entry point rule.
-include $(RBM_NAMEPLATE_PATH)

# Include configuration regimes
include $(RBM_TOOLS_DIR)/mbc.MakefileBashConsole.mk
include $(RBM_TOOLS_DIR)/rbb.BaseConfigRegime.mk
include $(RBM_TOOLS_DIR)/rbn.NameplateConfigRegime.mk
include $(RBM_TOOLS_DIR)/rbs.StationConfigRegime.mk

# Container and network naming
export RBM_SENTRY_CONTAINER   = $(RBM_MONIKER)-sentry
export RBM_BOTTLE_CONTAINER   = $(RBM_MONIKER)-bottle
export RBM_UPLINK_NETWORK     = $(RBM_MONIKER)-uplink
export RBM_ENCLAVE_NETWORK    = $(RBM_MONIKER)-enclave
export RBM_ENCLAVE_NAMESPACE  = $(RBM_MONIKER)-ns
export RBM_ENCLAVE_BRIDGE     = vbr_$(RBM_MONIKER)
export RBM_ENCLAVE_SENTRY_IN  = vsi_$(RBM_MONIKER)
export RBM_ENCLAVE_SENTRY_OUT = vso_$(RBM_MONIKER)
export RBM_ENCLAVE_BOTTLE_IN  = vbi_$(RBM_MONIKER)
export RBM_ENCLAVE_BOTTLE_OUT = vbo_$(RBM_MONIKER)

# Consolidated passed variables
zRBM_ROLLUP_ENV = $(filter RBM_%,$(.VARIABLES))


# Render rules
rbm-r%: rbs_render rbb_render rbn_render
	$(MBC_START) "Rendering regimes"
	@test -n "$(RBM_MONIKER)"        || (echo "Error: RBM_MONIKER must be set"                    && exit 1)
	@test -f "$(RBM_NAMEPLATE_PATH)" || (echo "Error: Nameplate not found: $(RBM_NAMEPLATE_PATH)" && exit 1)


# Validation rules
rbm-v%: zrbm_validate_regimes_rule
zrbm_validate_regimes_rule: rbb_validate rbn_validate rbs_validate
	$(MBC_START) "Validating regimes"
	@test -n "$(RBM_MONIKER)"        || (echo "Error: RBM_MONIKER must be set"                    && exit 1)
	@test -f "$(RBM_NAMEPLATE_PATH)" || (echo "Error: Nameplate not found: $(RBM_NAMEPLATE_PATH)" && exit 1)


rbp-a.%:
	$(MBC_START) "Establish podman machine."
	$(MBC_TERMINAL_SETTINGS) podman machine start
	$(MBC_PASS) "No errors."

rbp-s%: zrbm_start_service_rule
	$(MBC_STEP) "Completed delegate."

zrbm_start_service_rule: zrbm_validate_regimes_rule
	$(MBC_START) "Starting Bottle Service -> $(RBM_MONIKER)"

	$(MBC_STEP) "Stopping any prior containers"
	-podman stop -t 5  $(RBM_SENTRY_CONTAINER)
	-podman rm   -f    $(RBM_SENTRY_CONTAINER)
	-podman stop -t 5  $(RBM_BOTTLE_CONTAINER)
	-podman rm   -f    $(RBM_BOTTLE_CONTAINER)

	$(MBC_STEP) "Cleaning up old netns and interfaces inside VM"
	-podman machine ssh "sudo ip netns del $(RBM_ENCLAVE_NAMESPACE) 2>/dev/null || true"
	-podman machine ssh "sudo ip link del $(RBM_ENCLAVE_SENTRY_OUT) 2>/dev/null || true"
	-podman machine ssh "sudo ip link del $(RBM_ENCLAVE_SENTRY_IN)  2>/dev/null || true"
	-podman machine ssh "sudo ip link del $(RBM_ENCLAVE_BOTTLE_OUT) 2>/dev/null || true"
	-podman machine ssh "sudo ip link del $(RBM_ENCLAVE_BOTTLE_IN)  2>/dev/null || true"
	-podman machine ssh "sudo ip link del $(RBM_ENCLAVE_BRIDGE)     2>/dev/null || true"

	$(MBC_STEP) "Launching SENTRY container with bridging for internet"
	podman run -d                                      \
	  --name $(RBM_SENTRY_CONTAINER)                   \
	  --network bridge                                 \
	  --privileged                                     \
	  $(if $(RBN_PORT_ENABLED),-p $(RBN_ENTRY_PORT_WORKSTATION):$(RBN_ENTRY_PORT_WORKSTATION))  \
	  $(addprefix -e ,$(RBB__ROLLUP_ENVIRONMENT_VAR))                                           \
	  $(addprefix -e ,$(RBN__ROLLUP_ENVIRONMENT_VAR))                                           \
	  $(RBN_SENTRY_REPO_PATH):$(RBN_SENTRY_IMAGE_TAG)

	$(MBC_STEP) "Waiting for SENTRY container"
	sleep 2
	podman machine ssh "podman ps | grep $(RBM_SENTRY_CONTAINER) || (echo 'Container not running' && exit 1)"

	$(MBC_STEP) "Executing SENTRY namespace setup script"
	cat $(RBM_TOOLS_DIR)/rbm-sentry-ns-setup.sh   |\
	  podman machine ssh "$(foreach v,$(RBN__ROLLUP_ENVIRONMENT_VAR),export $v;) "  \
	                     "$(foreach v,$(zRBM_ROLLUP_ENV),export $v=\"$($v)\";) "    \
	                     "PODMAN_IGNORE_CGROUPSV1_WARNING=1 "                       \
	                     "/bin/sh"

	$(MBC_STEP) "Configuring SENTRY security"
	cat $(RBM_TOOLS_DIR)/rbm-sentry-setup.sh | podman exec -i $(RBM_SENTRY_CONTAINER) /bin/sh

	$(MBC_STEP) "Executing BOTTLE namespace setup script"
	cat $(RBM_TOOLS_DIR)/rbm-bottle-ns-setup.sh   |\
	  podman machine ssh "$(foreach v,$(RBN__ROLLUP_ENVIRONMENT_VAR),export $v;) "  \
	                     "$(foreach v,$(zRBM_ROLLUP_ENV),export $v=\"$($v)\";) "    \
	                     "PODMAN_IGNORE_CGROUPSV1_WARNING=1 "                       \
	                     "/bin/sh"

	$(MBC_STEP) "Verifying network setup in podman machine..."
	podman machine ssh "echo 'Network namespace list:' && sudo ip netns list"
	podman machine ssh "echo 'Namespace file permissions:' && ls -l /var/run/netns/$(RBM_ENCLAVE_NAMESPACE)"
	podman machine ssh "echo 'Network interfaces:'"
	podman machine ssh "sudo ip link show $(RBM_ENCLAVE_BRIDGE)"
	podman machine ssh "sudo ip link show $(RBM_ENCLAVE_BOTTLE_OUT)"
	podman machine ssh "sudo ip netns exec $(RBM_ENCLAVE_NAMESPACE) ip link list"

	$(MBC_STEP) "SUPERSTITION WAIT for BOTTLE steps settling..."
	sleep 2

	$(MBC_STEP) "Creating BOTTLE container with namespace networking"
	podman run -d                                      \
	  --name $(RBM_BOTTLE_CONTAINER)                   \
	  --network ns:/run/netns/$(RBM_ENCLAVE_NAMESPACE) \
	  --dns=$(RBN_ENCLAVE_SENTRY_IP)                   \
	  --cap-add net_raw                                \
	  --security-opt label=disable                     \
	  $(RBN_VOLUME_MOUNTS)                             \
	  $(RBN_BOTTLE_REPO_PATH):$(RBN_BOTTLE_IMAGE_TAG)

	$(MBC_STEP) "Waiting for BOTTLE container"
	sleep 2
	podman machine ssh "podman ps | grep $(RBM_BOTTLE_CONTAINER) || (echo 'Container not running' && exit 1)"

	$(MBC_STEP) "Bottle service should be available now."


rbm-br%: zrbm_validate_regimes_rule
	$(MBC_STEP) "Running Agile Bottle container for $(RBM_MONIKER)"
	
	# Command must be provided
	@test -n "$(CMD)" || (echo "Error: CMD must be set" && exit 1)
	
	# Bottle Create and Execute Sequence
	podman run --rm                                           \
	    --network $(RBM_ENCLAVE_NETWORK)                      \
	    --dns     $(RBN_ENCLAVE_SENTRY_IP)                    \
	    $(RBN_VOLUME_MOUNTS)                                  \
	    $(RBN_BOTTLE_REPO_PATH):$(RBN_BOTTLE_IMAGE_TAG)  \
	    $(CMD)


# zrbm_validate_regimes_rule
rbm-cs%:
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to SENTRY"
	podman exec -it $(RBM_SENTRY_CONTAINER) /bin/bash
	$(MBC_PASS) "Done, no errors."


rbm-cb%: zrbm_validate_regimes_rule
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to BOTTLE"
	podman exec -it $(RBM_BOTTLE_CONTAINER) /bin/bash

rbm-i%:  rbb_render rbn_render rbs_render
	$(MBC_PASS) "Done, no errors."


rbp-o%: zrbm_validate_regimes_rule
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "OBSERVE BOTTLE SERVICE NETWORKS"
	(                                                                    \
	  $(foreach v,$(RBN__ROLLUP_ENVIRONMENT_VAR),export $v && )          \
	  $(foreach v,$(zRBM_ROLLUP_ENV),export $v="$($v)" && )              \
	  cat $(RBM_TOOLS_DIR)/rbo.ObserveBottleServiceNetworks.sh | /bin/sh \
	)


# eof
