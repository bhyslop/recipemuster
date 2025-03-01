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

# Recipe Bottle Makefile (RBM)
# Implements secure containerized service management

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
export RBM_ENCLAVE_NS_DIR     = /var/lib/rbm-netns
export RBM_MACHINE            = podman-machine-default
export RBM_CONNECTION         = --connection $(RBM_MACHINE)

# Consolidated passed variables
zRBM_ROLLUP_ENV = $(filter RBM_%,$(.VARIABLES))

zRBM_EXPORT_ENV := "$(foreach v,$(RBRN__ROLLUP_ENVIRONMENT_VAR),export $v;) " \
                   "$(foreach v,$(RBRR__ROLLUP_ENVIRONMENT_VAR),export $v;) " \
                   "$(foreach v,$(zRBM_ROLLUP_ENV),export $v=\"$($v)\";) "    \
                   "PODMAN_IGNORE_CGROUPSV1_WARNING=1 "

zRBM_PODMAN_RAW_CMD   = podman machine ssh $(RBM_MACHINE)
zRBM_PODMAN_SSH_CMD   = podman machine ssh $(RBM_MACHINE) $(zRBM_EXPORT_ENV) 
zRBM_PODMAN_SHELL_CMD = $(zRBM_PODMAN_SSH_CMD) /bin/sh

# Validation rules
rbp-v.%: zrbp_validate_regimes_rule
zrbp_validate_regimes_rule: rbn_validate rbrr_validate rbrr_validate
	$(MBC_START) "Validating regimes"
	@test -n "$(RBM_MONIKER)"        || (echo "Error: RBM_MONIKER must be set"                    && exit 1)
	@test -f "$(RBM_NAMEPLATE_FILE)" || (echo "Error: Nameplate not found: $(RBM_NAMEPLATE_FILE)" && exit 1)

rbp_podman_machine_start_rule:
	$(MBC_START) "Capture some podman info"
	podman --version
	$(MBC_START) "Initialize Podman machine if it doesn't exist"
	podman machine list | grep -q "$(RBM_MACHINE)" || \
	  PODMAN_MACHINE_CGROUP=systemd podman machine init --rootful $(RBM_MACHINE)
	$(MBC_START) "Start up Podman machine $(RBM_MACHINE)"
	podman machine start $(RBM_MACHINE)
	$(MBC_START) "Update utilities..."
	podman $(RBM_CONNECTION) machine ssh sudo dnf install -y tcpdump
	$(MBC_START) "Version info on machine..."
	podman $(RBM_CONNECTION) version
	podman machine inspect $(RBM_MACHINE)
	podman $(RBM_CONNECTION) machine ssh "cat /etc/os-release && uname -r"
	podman $(RBM_CONNECTION) info
	$(MBC_PASS) "No errors."

rbp_podman_machine_stop_rule:
	$(MBC_START) "Stopping machine $(RBM_MACHINE)"
	podman machine stop $(RBM_MACHINE)
	$(MBC_PASS) "No errors."

rbp_podman_machine_nuke_rule:
	$(MBC_START) "Initialize Podman machine if it doesn't exist"
	-podman machine stop $(RBM_MACHINE)
	podman  machine rm   $(RBM_MACHINE)
	$(MBC_PASS) "No errors."

rbp_check_connection:
	$(MBC_START) "Checking connection to $(RBM_MACHINE)"
	podman $(RBM_CONNECTION) info > /dev/null || (echo "Unable to connect to machine" && exit 1)
	$(MBC_PASS) "Connection successful."

rbp-s.%: rbp_start_service_rule rbp_check_connection
	$(MBC_STEP) "Completed delegate."

rbp_start_service_rule: zrbp_validate_regimes_rule rbp_check_connection
	$(MBC_START) "Starting Bottle Service -> $(RBM_MONIKER)"

	$(MBC_STEP) "Stopping any prior containers"
	-podman $(RBM_CONNECTION) stop -t 2  $(RBM_SENTRY_CONTAINER)
	-podman $(RBM_CONNECTION) rm   -f    $(RBM_SENTRY_CONTAINER)
	-$(zRBM_PODMAN_RAW_CMD) sudo podman stop -t 2  $(RBM_BOTTLE_CONTAINER)
	-$(zRBM_PODMAN_RAW_CMD) sudo podman rm   -f    $(RBM_BOTTLE_CONTAINER)

	$(MBC_STEP) "Cleaning up old netns and interfaces inside VM"
	$(zRBM_PODMAN_SHELL_CMD) < $(MBV_TOOLS_DIR)/rbnc.cleanup.sh

	$(MBC_STEP) "Launching SENTRY container with bridging for internet"
	podman $(RBM_CONNECTION) run -d                         \
	  --name $(RBM_SENTRY_CONTAINER)                   \
	  --network bridge                                 \
	  --privileged                                     \
	  $(if $(RBRN_ENTRY_ENABLED),-p $(RBRN_ENTRY_PORT_WORKSTATION):$(RBRN_ENTRY_PORT_WORKSTATION)) \
	  $(addprefix -e ,$(RBRR__ROLLUP_ENVIRONMENT_VAR))                                             \
	  $(addprefix -e ,$(RBRN__ROLLUP_ENVIRONMENT_VAR))                                             \
	  $(RBRN_SENTRY_REPO_PATH):$(RBRN_SENTRY_IMAGE_TAG)

	$(MBC_STEP) "Waiting for SENTRY container"
	sleep 2
	$(zRBM_PODMAN_SSH_CMD) "podman ps | grep $(RBM_SENTRY_CONTAINER) || (echo 'Container not running' && exit 1)"

	$(MBC_STEP) "Executing SENTRY namespace setup script"
	$(zRBM_PODMAN_SHELL_CMD) < $(MBV_TOOLS_DIR)/rbns.sentry.sh

	$(MBC_STEP) "Configuring SENTRY security"
	podman $(RBM_CONNECTION) exec -i $(RBM_SENTRY_CONTAINER) /bin/sh < $(MBV_TOOLS_DIR)/rbss.sentry.sh

	$(MBC_STEP) "Executing BOTTLE namespace setup script"
	$(zRBM_PODMAN_SHELL_CMD) < $(MBV_TOOLS_DIR)/rbnb.bottle.sh

	$(MBC_STEP) "Visualizing network setup in podman machine..."
	$(zRBM_PODMAN_SHELL_CMD) < $(MBV_TOOLS_DIR)/rbni.info.sh

	$(MBC_STEP) "SUPERSTITION WAIT for BOTTLE steps settling..."
	sleep 2

	$(MBC_STEP) "Creating BOTTLE container with namespace networking (SKIPPING RBRN_VOLUME_MOUNTS FOR DEBUG)"
	$(zRBM_PODMAN_RAW_CMD) sudo podman run -d                      \
	  --name $(RBM_BOTTLE_CONTAINER)                               \
	  --privileged                                                 \
	  --network ns:/var/run/netns/$(RBM_ENCLAVE_NAMESPACE)         \
	  --dns=$(RBRN_ENCLAVE_SENTRY_IP)                              \
	  --cap-add net_raw                                            \
	  --security-opt label=disable                                 \
	  $(RBRN_BOTTLE_REPO_PATH):$(RBRN_BOTTLE_IMAGE_TAG)

	$(MBC_STEP) "Waiting for BOTTLE container"
	sleep 2
	$(zRBM_PODMAN_RAW_CMD) "sudo podman ps | grep $(RBM_BOTTLE_CONTAINER) || (echo 'Container not running' && exit 1)"

	$(MBC_STEP) "Bottle service should be available now."


rbp_connect_sentry_rule:
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to SENTRY"
	podman $(RBM_CONNECTION) exec -it $(RBM_SENTRY_CONTAINER) /bin/bash
	$(MBC_PASS) "Done, no errors."


rbp_connect_bottle_rule: zrbp_validate_regimes_rule
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to BOTTLE"
	$(zRBM_PODMAN_RAW_CMD) sudo podman exec -it $(RBM_BOTTLE_CONTAINER) /bin/bash


rbp_observe_networks_rule: zrbp_validate_regimes_rule
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "OBSERVE BOTTLE SERVICE NETWORKS"
	(eval $(zRBM_EXPORT_ENV) && /bin/sh < $(MBV_TOOLS_DIR)/rbo.observe.sh)


# eof
