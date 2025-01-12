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
# Prototype test: create custom netns, veth pair, run SENTRY with host net,
# then connect SENTRY to the netns as eth1. Also launch a BOTTLE container
# in the same netns (eth1) to verify pings.
#
# USAGE EXAMPLE:
#   make rbm-PTmyproto
#
# References:
#   https://chatgpt.com/c/6783fd42-e50c-8007-b23b-7744a7ed58f2
#
# You must define or override the following environment variables for your prototype:
#
RBM_PROTO_NS_NAME          = myproto-ns
RBM_PROTO_VETH_HOST        = myproto_veth0
RBM_PROTO_VETH_ENCLAVE     = myproto_veth1
RBM_PROTO_ENCLAVE_HOST_IP  = 10.242.0.1
RBM_PROTO_SENTRY_IP        = 10.242.0.2
RBM_PROTO_BOTTLE_IP        = 10.242.0.3
RBM_PROTO_SENTRY_CONTAINER = myproto-sentry
RBM_PROTO_BOTTLE_CONTAINER = myproto-bottle
RBM_PROTO_SENTRY_IMAGE     = ghcr.io/bhyslop/recipemuster:sentry_ubuntu_large.20241022__130547
RBM_PROTO_BOTTLE_IMAGE     = ghcr.io/bhyslop/recipemuster:bottle_ubuntu_test.20241207__190758


rbm-PN%: zrbm_proto_namespace_rule
	@echo "Prototype test complete for $(RBM_MONIKER)."

zrbm_proto_namespace_rule:
	@echo "=== Prototype netns test: $(RBM_PROTO_NS_NAME) ==="

	########################################################################
	# 1) STOP & REMOVE PRIOR CONTAINERS (host side, but effectively in VM)
	########################################################################
	@echo "1) Stop & remove any prior SENTRY container"
	-podman stop -t 5  $(RBM_PROTO_SENTRY_CONTAINER) || true
	-podman rm   -f    $(RBM_PROTO_SENTRY_CONTAINER) || true

	@echo "2) Stop & remove any prior BOTTLE container"
	-podman stop -t 5  $(RBM_PROTO_BOTTLE_CONTAINER) || true
	-podman rm   -f    $(RBM_PROTO_BOTTLE_CONTAINER) || true

	########################################################################
	# 3) CLEAN UP OLD NETNS & VETHs INSIDE THE VM
	########################################################################
	@echo "3) Clean up old netns and leftover veth interfaces inside the VM"
	@podman machine ssh "sudo ip netns del $(RBM_PROTO_NS_NAME) 2>/dev/null || true"
	@podman machine ssh "sudo ip link del veth_sentry_out  2>/dev/null || true"
	@podman machine ssh "sudo ip link del veth_sentry_in   2>/dev/null || true"
	@podman machine ssh "sudo ip link del veth_bottle_out  2>/dev/null || true"
	@podman machine ssh "sudo ip link del veth_bottle_in   2>/dev/null || true"

	########################################################################
	# 4) OPTIONAL: CREATE A VM-LEVEL NETNS & VETH (NOT FOR CONTAINERS)
	########################################################################
	@echo "4) Create new VM-level netns: $(RBM_PROTO_NS_NAME) [Optional demo]"
	@podman machine ssh "sudo ip netns add $(RBM_PROTO_NS_NAME)"

	@echo "5) Create veth pair: $(RBM_PROTO_VETH_HOST) <-> $(RBM_PROTO_VETH_ENCLAVE)"
	@podman machine ssh "sudo ip link add $(RBM_PROTO_VETH_HOST) type veth peer name $(RBM_PROTO_VETH_ENCLAVE)"

	@echo "6) Move $(RBM_PROTO_VETH_ENCLAVE) into netns $(RBM_PROTO_NS_NAME)"
	@podman machine ssh "sudo ip link set $(RBM_PROTO_VETH_ENCLAVE) netns $(RBM_PROTO_NS_NAME)"

	@echo "7) Assign IP on VM side, bring up link"
	@podman machine ssh "sudo ip addr add $(RBM_PROTO_ENCLAVE_HOST_IP)/24 dev $(RBM_PROTO_VETH_HOST) || true"
	@podman machine ssh "sudo ip link set $(RBM_PROTO_VETH_HOST) up || true"

	@echo "8) Assign IP on netns side, bring up link"
	@podman machine ssh "sudo ip netns exec $(RBM_PROTO_NS_NAME) ip addr add $(RBM_PROTO_SENTRY_IP)/24 dev $(RBM_PROTO_VETH_ENCLAVE) || true"
	@podman machine ssh "sudo ip netns exec $(RBM_PROTO_NS_NAME) ip link set $(RBM_PROTO_VETH_ENCLAVE) up || true"
	@podman machine ssh "sudo ip netns exec $(RBM_PROTO_NS_NAME) ip link set lo up || true"

	@echo "-----------------------------------------------------"
	@echo "Steps #4-#8 are optional. We created a netns in the VM"
	@echo "and a veth pair for demonstration. The containers will"
	@echo "have separate netns. Next we attach SENTRY & BOTTLE."
	@echo "-----------------------------------------------------"

	########################################################################
	# 9) START SENTRY CONTAINER (FROM HOST) BUT RUNS INSIDE THE VM
	########################################################################
	@echo "9) Launch SENTRY container with bridging for internet"
	podman run -d \
	  --name $(RBM_PROTO_SENTRY_CONTAINER) \
	  --network bridge \
	  --privileged \
	  $(RBM_PROTO_SENTRY_IMAGE)

	########################################################################
	# 10) GET THE REAL SENTRY PID FROM INSIDE THE VM, ADD veth_sentry_in
	########################################################################
	@echo "   Gathering the VM-side PID for SENTRY"
	# Use 'podman machine ssh "podman inspect ..." to get the true PID inside VM
	$(eval SENTRY_PID := $(shell podman machine ssh "podman inspect -f '{{.State.Pid}}' $(RBM_PROTO_SENTRY_CONTAINER)"))
	@echo "   The SENTRY PID inside the VM is $(SENTRY_PID)"

	@echo "10a) Create new veth pair 'veth_sentry_out' <-> 'veth_sentry_in' in the VM"
	@podman machine ssh "sudo ip link add veth_sentry_out type veth peer name veth_sentry_in"

	@echo "10b) Move 'veth_sentry_in' into SENTRY container's netns"
	@podman machine ssh "sudo ip link set veth_sentry_in netns $(SENTRY_PID)"
	@podman machine ssh "sudo nsenter -t $(SENTRY_PID) -n ip link set veth_sentry_in name eth1"
	@podman machine ssh "sudo nsenter -t $(SENTRY_PID) -n ip addr add $(RBM_PROTO_SENTRY_IP)/24 dev eth1"
	@podman machine ssh "sudo nsenter -t $(SENTRY_PID) -n ip link set eth1 up"

	@echo "10c) Give 'veth_sentry_out' an IP and bring it up in the VM"
	@podman machine ssh "sudo ip addr add $(RBM_PROTO_ENCLAVE_HOST_IP)/24 dev veth_sentry_out || true"
	@podman machine ssh "sudo ip link set veth_sentry_out up"

	@echo "10d) Enable IP forwarding & NAT inside SENTRY container"
	@podman machine ssh "podman exec $(RBM_PROTO_SENTRY_CONTAINER) sysctl -w net.ipv4.ip_forward=1"
	@podman machine ssh "podman exec $(RBM_PROTO_SENTRY_CONTAINER) iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE"

	@echo "11) Test ping from SENTRY's eth1 to 'veth_sentry_out' IP"
	@podman machine ssh "podman exec $(RBM_PROTO_SENTRY_CONTAINER) ping -c 2 $(RBM_PROTO_ENCLAVE_HOST_IP) || true"

	########################################################################
	# 12) START BOTTLE CONTAINER (NO NETWORK), THEN ADD SECOND INTERFACE
	########################################################################
	@echo "12) Launch BOTTLE container (from host) with --network none"
	podman run -d \
	  --name $(RBM_PROTO_BOTTLE_CONTAINER) \
	  --network none \
	  --privileged \
	  --security-opt label=disable \
	  $(RBM_PROTO_BOTTLE_IMAGE)

	@echo "   Gathering the VM-side PID for BOTTLE"
	$(eval BOTTLE_PID := $(shell podman machine ssh "podman inspect -f '{{.State.Pid}}' $(RBM_PROTO_BOTTLE_CONTAINER)"))
	@echo "   The BOTTLE PID inside the VM is $(BOTTLE_PID)"

	@echo "13) Create veth pair 'veth_bottle_out' <-> 'veth_bottle_in' in the VM"
	@podman machine ssh "sudo ip link add veth_bottle_out type veth peer name veth_bottle_in"

	@echo "13b) Move 'veth_bottle_in' into BOTTLE netns as eth1"
	@podman machine ssh "sudo ip link set veth_bottle_in netns $(BOTTLE_PID)"
	@podman machine ssh "sudo nsenter -t $(BOTTLE_PID) -n ip link set veth_bottle_in name eth1"
	@podman machine ssh "sudo nsenter -t $(BOTTLE_PID) -n ip addr add $(RBM_PROTO_BOTTLE_IP)/24 dev eth1"
	@podman machine ssh "sudo nsenter -t $(BOTTLE_PID) -n ip link set eth1 up"

	@echo "13c) Assign IP to 'veth_bottle_out' and bring it up in the VM"
	@podman machine ssh "sudo ip addr add 10.242.0.4/24 dev veth_bottle_out || true"
	@podman machine ssh "sudo ip link set veth_bottle_out up"

	@echo "14) Make BOTTLE route via SENTRY's eth1 IP: $(RBM_PROTO_SENTRY_IP)"
	@podman machine ssh "sudo nsenter -t $(BOTTLE_PID) -n ip route add default via $(RBM_PROTO_SENTRY_IP) dev eth1"

	@echo "15) Ping SENTRY from BOTTLE"
	@podman machine ssh "podman exec $(RBM_PROTO_BOTTLE_CONTAINER) ping -c 3 $(RBM_PROTO_SENTRY_IP) || true"

	@echo "16) (Optional) If NAT works, BOTTLE can ping external"
	@echo "    Try: podman machine ssh \"podman exec $(RBM_PROTO_BOTTLE_CONTAINER) ping -c 3 1.1.1.1\""



# eof
