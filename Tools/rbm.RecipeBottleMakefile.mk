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
include $(RBM_TOOLS_DIR)/test.rbm.mk

# Container and network naming
RBM_SENTRY_CONTAINER  = $(RBM_MONIKER)-sentry
RBM_BOTTLE_CONTAINER  = $(RBM_MONIKER)-bottle
RBM_UPLINK_NETWORK    = $(RBM_MONIKER)-uplink
RBM_ENCLAVE_NETWORK   = $(RBM_MONIKER)-enclave


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
	@echo "Stopping any prior containers for $(RBM_MONIKER)"
	-podman stop -t 5  $(RBM_SENTRY_CONTAINER)
	-podman rm -f      $(RBM_SENTRY_CONTAINER)
	-podman stop -t 5  $(RBM_BOTTLE_CONTAINER)
	-podman rm -f      $(RBM_BOTTLE_CONTAINER)

	@echo "Starting Sentry container for $(RBM_MONIKER)"

	# Network Creation Sequence
	-podman network rm -f $(RBM_UPLINK_NETWORK)
	-podman network rm -f $(RBM_ENCLAVE_NETWORK)
	podman network create --driver bridge $(RBM_UPLINK_NETWORK)
	podman network create --subnet $(RBN_ENCLAVE_BASE_IP)/$(RBN_ENCLAVE_NETMASK)  \
	                      --gateway $(RBN_ENCLAVE_SENTRY_IP)                      \
	                      --internal                                              \
	                      $(RBM_ENCLAVE_NETWORK)

	# Sentry Run Sequence
	-podman rm -f $(RBM_SENTRY_CONTAINER)
	podman run -d                                                                                 \
	    --name $(RBM_SENTRY_CONTAINER)                                                            \
	    --network $(RBM_UPLINK_NETWORK)                                                           \
	    --privileged                                                                              \
	    $(if $(RBN_PORT_ENABLED),-p $(RBN_ENTRY_PORT_WORKSTATION):$(RBN_ENTRY_PORT_WORKSTATION))  \
	    $(addprefix -e ,$(RBB__ROLLUP_ENVIRONMENT_VAR))                                           \
	    $(addprefix -e ,$(RBN__ROLLUP_ENVIRONMENT_VAR))                                           \
	    $(RBN_SENTRY_REPO_PATH):$(RBN_SENTRY_IMAGE_TAG)

	# Add debug pause point
	@read -p "Debug pause __BEFORE__ Network connect and IP change. Start SENTRY and ENCLAVE tcpdumps now..."

	# Network Connect and Configure Sequence
	podman network connect                              \
	    --ip $(RBN_ENCLAVE_INITIAL_IP)                  \
	    $(RBM_ENCLAVE_NETWORK) $(RBM_SENTRY_CONTAINER)

	# Verify eth1 presence and initial IP
	timeout 5s sh -c "while ! podman exec $(RBM_SENTRY_CONTAINER) ip addr show eth1 | grep -q 'inet '; do sleep 0.2; done"

	# Remove auto-assigned address and configure gateway
	podman exec $(RBM_SENTRY_CONTAINER) /bin/sh -c "ip link set eth1 arp off"
	podman exec $(RBM_SENTRY_CONTAINER) /bin/sh -c "ip addr del $(RBN_ENCLAVE_INITIAL_IP)/$(RBN_ENCLAVE_NETMASK)  dev eth1"
	podman exec $(RBM_SENTRY_CONTAINER) /bin/sh -c "ip addr add $(RBN_ENCLAVE_SENTRY_IP)/$(RBN_ENCLAVE_NETMASK)   dev eth1"
	podman exec $(RBM_SENTRY_CONTAINER) /bin/sh -c "ip link set eth1 arp on"

	# Add bridge flush and gratuitous ARP
	podman machine ssh "export MYPID=$$(podman inspect -f '{{.State.Pid}}' $(RBM_SENTRY_CONTAINER)) &&" \
	                   "export MYBRIDGE=$$(podman network inspect $(RBM_ENCLAVE_NETWORK) -f '{{.NetworkInterface}}') &&" \
	                   "echo 'pid:' $$MYPID ' and ' $$MYBRIDGE ' here' &&" \
	                   "sudo nsenter -t $$MYPID -n ip neigh flush dev $$MYBRIDGE"
	sleep 5
	podman exec $(RBM_SENTRY_CONTAINER) /bin/sh -c "arping -U -I eth1 -s $(RBN_ENCLAVE_SENTRY_IP) $(RBN_ENCLAVE_SENTRY_IP) -c 3"
	@read -p "Debug pause __AFTER__ IP change. Press enter..." dummy

	# Diagnostic info within namespaces
	podman machine ssh "sudo nsenter -t $$(podman inspect -f '{{.State.Pid}}' $(RBM_SENTRY_CONTAINER)) -n ip addr show"
	podman machine ssh "sudo nsenter -t $$(podman inspect -f '{{.State.Pid}}' $(RBM_SENTRY_CONTAINER)) -n ip neigh show"
	podman machine ssh "ip neigh show"

	# Clear ARP caches at container and bridge level
	podman exec $(RBM_SENTRY_CONTAINER) /bin/sh -c "ip neigh flush dev eth1"
	podman machine ssh "podman network inspect $(RBM_ENCLAVE_NETWORK)"
	podman machine ssh "sudo nsenter -t $$(podman inspect -f '{{.State.Pid}}' $(RBM_SENTRY_CONTAINER)) -n ip neigh flush dev eth1"

	# Verify route exists
	podman exec $(RBM_SENTRY_CONTAINER) /bin/sh -c "ip route show | grep -q '$(RBN_ENCLAVE_BASE_IP)/$(RBN_ENCLAVE_NETMASK) dev eth1'"

	# Verify gateway IP is configured correctly
	podman exec $(RBM_SENTRY_CONTAINER) /bin/sh -c "ip addr show eth1 | grep -q 'inet $(RBN_ENCLAVE_SENTRY_IP)'"

	# Security Configuration
	cat $(RBM_TOOLS_DIR)/rbm-sentry-setup.sh | podman exec -i $(RBM_SENTRY_CONTAINER) /bin/sh


rbm-BS%: zrbm_start_bottle_rule
	@echo "Completed delegate."
zrbm_start_bottle_rule:
	@echo "Starting Sessile Bottle container for $(RBM_MONIKER)"
	
	# Bottle Cleanup Sequence
	-podman stop -t 5  $(RBM_BOTTLE_CONTAINER)
	-podman rm -f      $(RBM_BOTTLE_CONTAINER)
	
	# Bottle Launch Sequence
	podman run -d                                \
	    --name    $(RBM_BOTTLE_CONTAINER)        \
	    --network $(RBM_ENCLAVE_NETWORK)         \
	    --dns     $(RBN_ENCLAVE_SENTRY_IP)       \
	    --restart unless-stopped                 \
	    $(RBN_VOLUME_MOUNTS)                     \
	    $(RBN_BOTTLE_REPO_PATH):$(RBN_BOTTLE_IMAGE_TAG)


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


###
# Network namespace testing: creates custom netns, veth pairs, runs SENTRY with host net,
# then connects SENTRY to the netns as eth1. Also launches a BOTTLE container
# in the same netns (eth1) to verify connectivity.
#
# USAGE EXAMPLE:
#   make rbm-PNnsproto
#
# References:
#   https://chatgpt.com/c/6783fd42-e50c-8007-b23b-7744a7ed58f2
#
# Custom network namespace variables (not in nameplate):
#
RBM_PROTO_NS_NAME          = $(RBM_MONIKER)-ns
RBM_PROTO_BRIDGE           = vbr_$(RBM_MONIKER)
RBM_PROTO_VETH_SENTRY_IN   = vsi_$(RBM_MONIKER)
RBM_PROTO_VETH_SENTRY_OUT  = vso_$(RBM_MONIKER)
RBM_PROTO_VETH_BOTTLE_IN   = vbi_$(RBM_MONIKER)
RBM_PROTO_VETH_BOTTLE_OUT  = vbo_$(RBM_MONIKER)
RBM_PROTO_NETMASK          = $(RBN_ENCLAVE_NETMASK)
RBM_PROTO_SENTRY_IP        = $(RBN_ENCLAVE_SENTRY_IP)
RBM_PROTO_BOTTLE_IP        = 10.242.0.3
NSPROTO_SENTRY_NS_SCRIPT   = $(RBM_TOOLS_DIR)/nsproto-sentry-namespace-setup.sh
NSPROTO_BOTTLE_NS_SCRIPT   = $(RBM_TOOLS_DIR)/nsproto-bottle-namespace-setup.sh


rbm-PN%: zrbm_proto_namespace_rule
	@echo "Prototype test complete for $(RBM_MONIKER)."

zrbm_proto_namespace_rule:
	@echo "=== Network namespace test: $(RBM_PROTO_NS_NAME) ==="

	########################################################################
	# 1) STOP & REMOVE PRIOR CONTAINERS
	########################################################################
	@echo "1) Stop & remove any prior SENTRY container"
	-podman stop -t 5  $(RBM_SENTRY_CONTAINER) || true
	-podman rm   -f    $(RBM_SENTRY_CONTAINER) || true

	@echo "2) Stop & remove any prior BOTTLE container"
	-podman stop -t 5  $(RBM_BOTTLE_CONTAINER) || true
	-podman rm   -f    $(RBM_BOTTLE_CONTAINER) || true

	########################################################################
	# 3) CLEAN UP OLD NETNS & VETHs INSIDE THE VM
	########################################################################
	@echo "3) Clean up old netns and leftover veth interfaces inside the VM"
	podman machine ssh "sudo ip netns del $(RBM_PROTO_NS_NAME)        2>/dev/null || true"
	podman machine ssh "sudo ip link del $(RBM_PROTO_VETH_SENTRY_OUT) 2>/dev/null || true"
	podman machine ssh "sudo ip link del $(RBM_PROTO_VETH_SENTRY_IN)  2>/dev/null || true"
	podman machine ssh "sudo ip link del $(RBM_PROTO_VETH_BOTTLE_OUT) 2>/dev/null || true"
	podman machine ssh "sudo ip link del $(RBM_PROTO_VETH_BOTTLE_IN)  2>/dev/null || true"
	podman machine ssh "sudo ip link del $(RBM_PROTO_BRIDGE)          2>/dev/null || true"

	########################################################################
	# 4) START SENTRY CONTAINER
	########################################################################
	@echo "4) Launch SENTRY container with bridging for internet"
	podman run -d                                      \
	  --name $(RBM_SENTRY_CONTAINER)                   \
	  --network bridge                                 \
	  --privileged                                     \
	  $(RBN_SENTRY_REPO_PATH):$(RBN_SENTRY_IMAGE_TAG)

	@echo "4a) Waiting for SENTRY container..."
	sleep 2
	podman machine ssh "podman ps | grep $(RBM_SENTRY_CONTAINER) || (echo 'Container not running' && exit 1)"

	########################################################################
	# 5) CONSTRUCT SENTRY NAMESPACE SCRIPT
	########################################################################
	@echo "5) Construct SENTRY namespace setup script"
	echo "#!/bin/sh"                                                                                          >  $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "set -e"                                                                                             >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "echo 'Can we see sentry here, inspect intimate...'"                                                 >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "podman inspect -f '{{.State.Pid}}' $(RBM_SENTRY_CONTAINER)"                                         >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "echo 'Getting SENTRY PID...'"                                                                       >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "SENTRY_PID=\$$(PODMAN_IGNORE_CGROUPSV1_WARNING=1 podman inspect -f '{{.State.Pid}}' $(RBM_SENTRY_CONTAINER))" >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "[ -n \"\$$SENTRY_PID\" ] || exit 31"                                                                >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "echo 'SENTRY PID: '\$$SENTRY_PID"                                                                   >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "echo 'Setting up bridge and SENTRY networking...'"                                                  >> $(NSPROTO_SENTRY_NS_SCRIPT)
	# Create and configure bridge
	echo "echo 'Creating bridge $(RBM_PROTO_BRIDGE)...'"                                                      >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "sudo ip link add name $(RBM_PROTO_BRIDGE) type bridge"                                              >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "sudo ip link set $(RBM_PROTO_BRIDGE) up"                                                            >> $(NSPROTO_SENTRY_NS_SCRIPT)
	# Create and configure SENTRY veth pair
	echo "sudo ip link add $(RBM_PROTO_VETH_SENTRY_OUT) type veth peer name $(RBM_PROTO_VETH_SENTRY_IN)"      >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "sudo ip link set $(RBM_PROTO_VETH_SENTRY_IN) netns \$$SENTRY_PID"                                   >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "sudo nsenter -t \$$SENTRY_PID -n ip link set $(RBM_PROTO_VETH_SENTRY_IN) name eth1"                 >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "sudo nsenter -t \$$SENTRY_PID -n ip addr add $(RBM_PROTO_SENTRY_IP)/$(RBM_PROTO_NETMASK) dev eth1"  >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "sudo nsenter -t \$$SENTRY_PID -n ip link set eth1 up"                                               >> $(NSPROTO_SENTRY_NS_SCRIPT)
	# Connect SENTRY veth to bridge
	echo "sudo ip link set $(RBM_PROTO_VETH_SENTRY_OUT) master $(RBM_PROTO_BRIDGE)"                           >> $(NSPROTO_SENTRY_NS_SCRIPT)
	echo "sudo ip link set $(RBM_PROTO_VETH_SENTRY_OUT) up"                                                   >> $(NSPROTO_SENTRY_NS_SCRIPT)

	########################################################################
	# 6) EXECUTE SENTRY NAMESPACE SCRIPT
	########################################################################
	@echo "6) Execute SENTRY namespace setup script"
	cat $(NSPROTO_SENTRY_NS_SCRIPT) | podman machine ssh "/bin/sh"

	########################################################################
	# 7) CREATE BOTTLE CONTAINER (BUT DON'T START)
	########################################################################
	@echo "7) Create (but don't start) BOTTLE container"
	podman create                                      \
	  --name $(RBM_BOTTLE_CONTAINER)                   \
	  --network none                                   \
	  --cap-add net_raw                                \
	  --security-opt label=disable                     \
	  $(RBN_BOTTLE_REPO_PATH):$(RBN_BOTTLE_IMAGE_TAG)

	########################################################################
	# 8) CONSTRUCT BOTTLE NAMESPACE SCRIPT
	########################################################################
	@echo "8) Construct BOTTLE namespace setup script"
	echo "#!/bin/sh"                                                                                          >  $(NSPROTO_BOTTLE_NS_SCRIPT)
	echo "set -e"                                                                                             >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	echo "echo 'Getting BOTTLE PID...'"                                                                       >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	echo "BOTTLE_PID=\$$(podman inspect -f '{{.State.Pid}}' $(RBM_BOTTLE_CONTAINER))"                         >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	echo "[ -n \"\$$BOTTLE_PID\" ] || exit 31"                                                                >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	echo "echo 'BOTTLE PID: '\$$BOTTLE_PID"                                                                   >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	echo "echo 'Setting up BOTTLE networking...'"                                                             >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	# Create and configure BOTTLE veth pair
	echo "sudo ip link add $(RBM_PROTO_VETH_BOTTLE_OUT) type veth peer name $(RBM_PROTO_VETH_BOTTLE_IN)"      >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	echo "sudo ip link set $(RBM_PROTO_VETH_BOTTLE_IN) netns \$$BOTTLE_PID"                                   >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	echo "sudo nsenter -t \$$BOTTLE_PID -n ip link set $(RBM_PROTO_VETH_BOTTLE_IN) name eth1"                 >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	echo "sudo nsenter -t \$$BOTTLE_PID -n ip addr add $(RBM_PROTO_BOTTLE_IP)/$(RBM_PROTO_NETMASK) dev eth1"  >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	echo "sudo nsenter -t \$$BOTTLE_PID -n ip link set eth1 up"                                               >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	# Connect BOTTLE veth to bridge
	echo "sudo ip link set $(RBM_PROTO_VETH_BOTTLE_OUT) master $(RBM_PROTO_BRIDGE)"                           >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	echo "sudo ip link set $(RBM_PROTO_VETH_BOTTLE_OUT) up"                                                   >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	echo "echo 'Configuring BOTTLE routing...'"                                                               >> $(NSPROTO_BOTTLE_NS_SCRIPT)
	echo "sudo nsenter -t \$$BOTTLE_PID -n ip route add default via $(RBM_PROTO_SENTRY_IP) dev eth1"          >> $(NSPROTO_BOTTLE_NS_SCRIPT)

	########################################################################
	# 9) START BOTTLE CONTAINER
	########################################################################
	@echo "9) Start BOTTLE container"
	podman start $(RBM_BOTTLE_CONTAINER)

	########################################################################
	# 10) EXECUTE BOTTLE NAMESPACE SCRIPT
	########################################################################
	@echo "10) Execute BOTTLE namespace setup script"
	cat $(NSPROTO_BOTTLE_NS_SCRIPT) | podman machine ssh "/bin/sh"

	########################################################################
	# 11) TEST CONNECTIVITY
	########################################################################
	@echo "11) Testing BOTTLE to SENTRY connectivity"
	-podman machine ssh "podman exec $(RBM_BOTTLE_CONTAINER) ping -c 3 $(RBM_PROTO_SENTRY_IP)"

	@echo "12) Testing BOTTLE external connectivity"
	-podman machine ssh "podman exec $(RBM_BOTTLE_CONTAINER) ping -c 3 1.1.1.1"


# eof
