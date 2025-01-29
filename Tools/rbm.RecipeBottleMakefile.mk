## � 2024 Scale Invariant.  All rights reserved.
##      Reference: https://www.termsfeed.com/blog/sample-copyright-notices/
##
## Unauthorized copying of this file, via any medium is strictly prohibited
## Proprietary and confidential
##
## Written by Brad Hyslop <bhyslop@scaleinvariant.org> November 2024

# Recipe Bottle Makefile (RBM)
# Implements secure containerized service management

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
include $(RBM_TOOLS_DIR)/rbb.BaseConfigRegime.mk
include $(RBM_TOOLS_DIR)/rbn.NameplateConfigRegime.mk
include $(RBM_TOOLS_DIR)/rbs.StationConfigRegime.mk

# Test rules
include $(RBM_TOOLS_DIR)/rbm.test.nsproto.mk
include $(RBM_TOOLS_DIR)/rbm.test.srjcl.mk

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
	@test -n "$(RBM_MONIKER)"        || (echo "Error: RBM_MONIKER must be set"                    && exit 1)
	@test -f "$(RBM_NAMEPLATE_PATH)" || (echo "Error: Nameplate not found: $(RBM_NAMEPLATE_PATH)" && exit 1)


# Validation rules
rbm-v%: zrbm_validate_regimes_rule
zrbm_validate_regimes_rule: rbb_validate rbn_validate rbs_validate
	@test -n "$(RBM_MONIKER)"        || (echo "Error: RBM_MONIKER must be set"                    && exit 1)
	@test -f "$(RBM_NAMEPLATE_PATH)" || (echo "Error: Nameplate not found: $(RBM_NAMEPLATE_PATH)" && exit 1)


rbm-SS%: zrbm_start_sentry_rule
	@echo "Completed delegate."

zrbm_start_sentry_rule: zrbm_validate_regimes_rule
	@echo "Starting containers for $(RBM_MONIKER)"
	
	@echo "Stopping any prior containers"
	-podman stop -t 5  $(RBM_SENTRY_CONTAINER)
	-podman rm   -f    $(RBM_SENTRY_CONTAINER)
	-podman stop -t 5  $(RBM_BOTTLE_CONTAINER)
	-podman rm   -f    $(RBM_BOTTLE_CONTAINER)

	@echo "Cleaning up old netns and interfaces inside VM"
	-podman machine ssh "sudo ip netns del $(RBM_ENCLAVE_NAMESPACE) 2>/dev/null || true"
	-podman machine ssh "sudo ip link del $(RBM_ENCLAVE_SENTRY_OUT) 2>/dev/null || true"
	-podman machine ssh "sudo ip link del $(RBM_ENCLAVE_SENTRY_IN)  2>/dev/null || true"
	-podman machine ssh "sudo ip link del $(RBM_ENCLAVE_BOTTLE_OUT) 2>/dev/null || true"
	-podman machine ssh "sudo ip link del $(RBM_ENCLAVE_BOTTLE_IN)  2>/dev/null || true"
	-podman machine ssh "sudo ip link del $(RBM_ENCLAVE_BRIDGE)     2>/dev/null || true"

	@echo "Launching SENTRY container with bridging for internet"
	podman run -d                                      \
	  --name $(RBM_SENTRY_CONTAINER)                   \
	  --network bridge                                 \
	  --privileged                                     \
	  $(if $(RBN_PORT_ENABLED),-p $(RBN_ENTRY_PORT_WORKSTATION):$(RBN_ENTRY_PORT_WORKSTATION))  \
	  $(addprefix -e ,$(RBB__ROLLUP_ENVIRONMENT_VAR))                                           \
	  $(addprefix -e ,$(RBN__ROLLUP_ENVIRONMENT_VAR))                                           \
	  $(RBN_SENTRY_REPO_PATH):$(RBN_SENTRY_IMAGE_TAG)

	@echo "Waiting for SENTRY container"
	sleep 2
	podman machine ssh "podman ps | grep $(RBM_SENTRY_CONTAINER) || (echo 'Container not running' && exit 1)"

	@echo "Executing SENTRY namespace setup script"
	cat $(RBM_TOOLS_DIR)/rbm-sentry-ns-setup.sh   |\
	  podman machine ssh "$(foreach v,$(RBN__ROLLUP_ENVIRONMENT_VAR),export $v;) "  \
	                     "$(foreach v,$(zRBM_ROLLUP_ENV),export $v=\"$($v)\";) "    \
	                     "PODMAN_IGNORE_CGROUPSV1_WARNING=1 "                       \
	                     "/bin/sh"

	@echo "Configuring SENTRY security"
	cat $(RBM_TOOLS_DIR)/rbm-sentry-setup.sh | podman exec -i $(RBM_SENTRY_CONTAINER) /bin/sh

	@echo "Executing BOTTLE namespace setup script"
	cat $(RBM_TOOLS_DIR)/rbm-bottle-ns-setup.sh   |\
	  podman machine ssh "$(foreach v,$(RBN__ROLLUP_ENVIRONMENT_VAR),export $v;) "  \
	                     "$(foreach v,$(zRBM_ROLLUP_ENV),export $v=\"$($v)\";) "    \
	                     "PODMAN_IGNORE_CGROUPSV1_WARNING=1 "                       \
	                     "/bin/sh"

	#@echo "Setting network namespace permissions..."
	#-podman machine ssh "sudo chmod 555 /var/run/netns"
	#-podman machine ssh "sudo chmod 555 /var/run/netns/$(RBM_ENCLAVE_NAMESPACE)"
	#-podman machine ssh "ls -la /var/run"
	#-podman machine ssh "ls -la /var/run/netns"

	@echo "Verifying network setup in podman machine..."
	podman machine ssh "echo 'Network namespace list:' && sudo ip netns list"
	podman machine ssh "echo 'Namespace file permissions:' && ls -l /var/run/netns/$(RBM_ENCLAVE_NAMESPACE)"
	podman machine ssh "echo 'Network interfaces:'"
	podman machine ssh "sudo ip link show $(RBM_ENCLAVE_BRIDGE)"
	podman machine ssh "sudo ip link show $(RBM_ENCLAVE_BOTTLE_OUT)"
	podman machine ssh "sudo ip netns exec $(RBM_ENCLAVE_NAMESPACE) ip link list"

	@echo "SUPERSTITION WAIT for BOTTLE steps settling..."
	sleep 2

	@echo "Creating BOTTLE container with namespace networking"
	podman run -d                                 \
	  --name $(RBM_BOTTLE_CONTAINER)              \
	  --network ns:/run/netns/$(RBM_ENCLAVE_NAMESPACE) \
	  --dns=$(RBN_ENCLAVE_SENTRY_IP)              \
	  --cap-add net_raw                           \
	  --security-opt label=disable                \
	  $(RBN_VOLUME_MOUNTS)                        \
	  $(RBN_BOTTLE_REPO_PATH):$(RBN_BOTTLE_IMAGE_TAG)

	@echo "Waiting for BOTTLE container"
	sleep 2
	podman machine ssh "podman ps | grep $(RBM_BOTTLE_CONTAINER) || (echo 'Container not running' && exit 1)"

	@echo "Bottle service should be available now."


rbm-BS%: zrbm_start_bottle_rule
	@echo "Completed delegate."
zrbm_start_bottle_rule:
	@echo "UNUSED FOR NOW."
	false


rbm-br%: zrbm_validate_regimes_rule
	@echo "Running Agile Bottle container for $(RBM_MONIKER)"
	
	# Command must be provided
	@test -n "$(CMD)" || (echo "Error: CMD must be set" && exit 1)
	
	# Bottle Create and Execute Sequence
	podman run --rm                                           \
	    --network $(RBM_ENCLAVE_NETWORK)                      \
	    --dns     $(RBN_ENCLAVE_SENTRY_IP)                    \
	    $(RBN_VOLUME_MOUNTS)                                  \
	    $(RBN_BOTTLE_REPO_PATH):$(RBN_BOTTLE_IMAGE_TAG)  \
	    $(CMD)


# Sentry Stop Rule
rbm-SX%: zrbm_validate_regimes_rule
	@echo "Stopping Sentry container for $(RBM_MONIKER)"
	
	# Network disconnection
	-podman network disconnect $(RBM_ENCLAVE_NETWORK) $(RBM_SENTRY_CONTAINER)
	-podman network disconnect $(RBM_UPLINK_NETWORK)  $(RBM_SENTRY_CONTAINER)
	
	# Container termination
	-podman stop -t 5  $(RBM_SENTRY_CONTAINER)
	-podman rm -f      $(RBM_SENTRY_CONTAINER)
	
	# Network cleanup
	-podman network rm -f $(RBM_ENCLAVE_NETWORK)
	-podman network rm -f $(RBM_UPLINK_NETWORK)


# Bottle Stop Rule
rbm-BX%: zrbm_validate_regimes_rule
	@echo "Stopping Bottle container for $(RBM_MONIKER)"
	-podman stop -t 5  $(RBM_BOTTLE_CONTAINER)
	-podman rm -f      $(RBM_BOTTLE_CONTAINER)


zrbm_start_sessile_rule:
	@echo "Starting Sessile Service $(RBM_MONIKER)"


# zrbm_start_sessile_rule  rbm-SS rbm-BS
rbm-ss%:                  \
  zrbm_start_sessile_rule \
  zrbm_start_sentry_rule  \
  zrbm_start_bottle_rule  \
  # Game on...
	@echo "Started Sessile Service $(RBM_MONIKER)"

# zrbm_validate_regimes_rule
rbm-cs%:
	@echo "Moniker:"$(RBM_ARG_MONIKER) "Connecting to SENTRY"
	podman exec -it $(RBM_SENTRY_CONTAINER) /bin/bash
	$(MBC_PASS) "Done, no errors."


rbm-cb%: zrbm_validate_regimes_rule
	@echo "Moniker:"$(RBM_ARG_MONIKER) "Connecting to BOTTLE"
	podman exec -it $(RBM_BOTTLE_CONTAINER) /bin/bash

rbm-i%:  rbb_render rbn_render rbs_render
	$(MBC_PASS) "Done, no errors."


rbm-d%:
	@echo "Moniker:"$(RBM_ARG_MONIKER) "Explicit dnsmasq run"
	# OUCH /bin/sh or /bin/bash ?
	podman exec $(RBM_SENTRY_CONTAINER) /bin/bash -c "dnsmasq --keep-in-foreground"


rbm-OIS%:
	@echo "Moniker:"$(RBM_ARG_MONIKER) "OBSERVE INSIDE SENTRY"
	@echo "Nuke any tcpdump there before..."
	podman exec $(RBM_SENTRY_CONTAINER) pkill tcpdump || true
	@echo "First, lets get process info so we know the dnsmasq is up..."
	podman exec $(RBM_SENTRY_CONTAINER) ps aux
	@echo "Now, lets tcpdump..."
	podman exec $(RBM_SENTRY_CONTAINER) tcpdump -n


rbm-OIB%:
	@echo "Moniker:"$(RBM_ARG_MONIKER) "TCPDUMPER AT BOTTLE"
	@echo "Nuke any tcpdump there before..."
	podman exec $(RBM_BOTTLE_CONTAINER) pkill tcpdump || true
	@echo "Now, lets tcpdump..."
	podman exec $(RBM_BOTTLE_CONTAINER) tcpdump -n


rbm-OPS%:
	@echo "Moniker:"$(RBM_ARG_MONIKER) "OBSERVE PODMAN MACHINE SENTRY"
	podman machine ssh "sudo dnf install -y tcpdump || true"
	podman machine ssh "sudo nsenter -t $$(podman inspect -f '{{.State.Pid}}' $(RBM_SENTRY_CONTAINER)) -n tcpdump -i any -n -vvv"


rbm-OPB%:
	@echo "Moniker:"$(RBM_ARG_MONIKER) "OBSERVE PODMAN MACHINE SENTRY"
	podman machine ssh "sudo dnf install -y tcpdump || true"
	podman machine ssh "sudo nsenter -t $$(podman inspect -f '{{.State.Pid}}' $(RBM_BOTTLE_CONTAINER)) -n tcpdump -i any -n -vvv"


rbm-OPE%:
	@echo "Moniker:"$(RBM_ARG_MONIKER) "OBSERVE ENCLAVE NETWORK"
	podman machine ssh "sudo nsenter --net=/proc/$$(podman inspect -f '{{.State.Pid}}' $(RBM_MONIKER)-sentry)/ns/net tcpdump -i any -n -vvv"


# eof
