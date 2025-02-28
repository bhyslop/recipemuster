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

# OUCH consolidate with RBG 
#zRBP_MACHINE = $(RBRR_MACHINE_NAME)
zRBP_MACHINE = podman-machine-default
zRBP_CONN = --connection $(zRBP_MACHINE)

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

# Consolidated passed variables
zRBM_ROLLUP_ENV = $(filter RBM_%,$(.VARIABLES))

zRBM_EXPORT_ENV := "$(foreach v,$(RBRN__ROLLUP_ENVIRONMENT_VAR),export $v;) " \
                   "$(foreach v,$(RBRR__ROLLUP_ENVIRONMENT_VAR),export $v;) " \
                   "$(foreach v,$(zRBM_ROLLUP_ENV),export $v=\"$($v)\";) "    \
                   "PODMAN_IGNORE_CGROUPSV1_WARNING=1 "

# OUCH messy
#zRBM_PODMAN_SSH_CMD   = SSH_STRICT_HOST_KEY_CHECKING=no podman machine ssh $(zRBP_MACHINE) $(zRBM_EXPORT_ENV) 
zRBM_PODMAN_SSH_CMD   = podman machine ssh $(zRBP_MACHINE) $(zRBM_EXPORT_ENV) 
zRBM_PODMAN_SHELL_CMD = $(zRBM_PODMAN_SSH_CMD) /bin/sh
zRBM_BOTTLE_SSH_CMD   = podman machine ssh $(zRBP_MACHINE)

# Validation rules
rbp-v.%: zrbp_validate_regimes_rule
zrbp_validate_regimes_rule: rbn_validate rbrr_validate rbrr_validate
	$(MBC_START) "Validating regimes"
	@test -n "$(RBM_MONIKER)"        || (echo "Error: RBM_MONIKER must be set"                    && exit 1)
	@test -f "$(RBM_NAMEPLATE_FILE)" || (echo "Error: Nameplate not found: $(RBM_NAMEPLATE_FILE)" && exit 1)

rbp_podman_machine_init_rule:
	$(MBC_START) "Initialize Podman machine if it doesn't exist"
	@podman machine list | grep -q "$(zRBP_MACHINE)" || \
	  PODMAN_MACHINE_CGROUP=systemd podman machine init `#--rootful` $(zRBP_MACHINE)
	$(MBC_PASS) "No errors."

rbp_podman_machine_start_rule: rbp_podman_machine_init_rule
	$(MBC_START) "Start up Podman machine $(zRBP_MACHINE)"
	podman machine start $(zRBP_MACHINE)
	$(MBC_PASS) "No errors."

rbp_podman_machine_stop_rule:
	$(MBC_START) "Stopping machine $(zRBP_MACHINE)"
	podman machine stop $(zRBP_MACHINE)
	$(MBC_PASS) "No errors."

rbp_podman_machine_nuke_rule:
	$(MBC_START) "Initialize Podman machine if it doesn't exist"
	-podman machine stop $(zRBP_MACHINE)
	podman  machine rm   $(zRBP_MACHINE)
	$(MBC_PASS) "No errors."

rbp_check_connection:
	$(MBC_START) "Checking connection to $(zRBP_MACHINE)"
	podman $(zRBP_CONN) info > /dev/null || (echo "Unable to connect to machine" && exit 1)
	$(MBC_PASS) "Connection successful."

rbp-s.%: rbp_start_service_rule rbp_check_connection
	$(MBC_STEP) "Completed delegate."

rbp_start_service_rule: zrbp_validate_regimes_rule rbp_check_connection
	$(MBC_START) "Starting Bottle Service -> $(RBM_MONIKER)"

	$(MBC_STEP) "Stopping any prior containers"
	-podman $(zRBP_CONN) stop -t 2  $(RBM_SENTRY_CONTAINER)
	-podman $(zRBP_CONN) rm   -f    $(RBM_SENTRY_CONTAINER)
	-$(zRBM_BOTTLE_SSH_CMD) podman stop -t 2  $(RBM_BOTTLE_CONTAINER)
	-$(zRBM_BOTTLE_SSH_CMD) podman rm   -f    $(RBM_BOTTLE_CONTAINER)

	$(MBC_STEP) "Cleaning up old netns and interfaces inside VM"
	$(zRBM_PODMAN_SHELL_CMD) < $(MBV_TOOLS_DIR)/rbnc.cleanup.sh

	$(MBC_STEP) "Launching SENTRY container with bridging for internet"
	podman $(zRBP_CONN) run -d                         \
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
	podman $(zRBP_CONN) exec -i $(RBM_SENTRY_CONTAINER) /bin/sh < $(MBV_TOOLS_DIR)/rbss.sentry.sh

	$(MBC_STEP) "Executing BOTTLE namespace setup script"
	$(zRBM_PODMAN_SHELL_CMD) < $(MBV_TOOLS_DIR)/rbnb.bottle.sh

	$(MBC_STEP) "Visualizing network setup in podman machine..."
	$(zRBM_PODMAN_SHELL_CMD) < $(MBV_TOOLS_DIR)/rbni.info.sh

	$(MBC_STEP) "SUPERSTITION WAIT for BOTTLE steps settling..."
	sleep 2

	$(MBC_STEP) "Creating BOTTLE container with namespace networking"
	$(zRBM_BOTTLE_SSH_CMD) sudo podman run -d                      \
	  --replace                                                    \
	  --name $(RBM_BOTTLE_CONTAINER)                               \
	  --network ns:/var/run/netns/$(RBM_ENCLAVE_NAMESPACE)         \
	  --dns=$(RBRN_ENCLAVE_SENTRY_IP)                              \
	  --cap-add net_raw                                            \
	  --security-opt label=disable                                 \
	  $(RBRN_VOLUME_MOUNTS)                                        \
	  $(RBRN_BOTTLE_REPO_PATH):$(RBRN_BOTTLE_IMAGE_TAG)

	$(MBC_STEP) "Waiting for BOTTLE container"
	sleep 2
	$(zRBM_BOTTLE_SSH_CMD) "podman ps | grep $(RBM_BOTTLE_CONTAINER) || (echo 'Container not running' && exit 1)"

	$(MBC_STEP) "Bottle service should be available now."


rbp-s.%:
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to SENTRY"
	podman $(zRBP_CONN) exec -it $(RBM_SENTRY_CONTAINER) /bin/bash
	$(MBC_PASS) "Done, no errors."


rbp-b.%: zrbp_validate_regimes_rule
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to BOTTLE"
	podman $(zRBP_CONN) exec -it $(RBM_BOTTLE_CONTAINER) /bin/bash


rbp-o.%: zrbp_validate_regimes_rule
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "OBSERVE BOTTLE SERVICE NETWORKS"
	(eval $(zRBM_EXPORT_ENV) && /bin/sh < $(MBV_TOOLS_DIR)/rbo.observe.shs)



# eof
