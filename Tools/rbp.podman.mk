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

# Consolidated passed variables
zRBM_ROLLUP_ENV = $(filter RBM_%,$(.VARIABLES))

zRBM_EXPORT_ENV := "$(foreach v,$(RBN__ROLLUP_ENVIRONMENT_VAR),export $v;) " \
                   "$(foreach v,$(zRBM_ROLLUP_ENV),export $v=\"$($v)\";) "   \
                   "PODMAN_IGNORE_CGROUPSV1_WARNING=1 "

zRBM_PODMAN_SSH_CMD = podman machine ssh $(zRBM_EXPORT_ENV) /bin/sh


# Render rules
rbp-r.%: rbs_render rbb_render rbn_render
	$(MBC_START) "Rendering regimes"
	@test -n "$(RBM_MONIKER)"        || (echo "Error: RBM_MONIKER must be set"                    && exit 1)
	@test -f "$(RBM_NAMEPLATE_FILE)" || (echo "Error: Nameplate not found: $(RBM_NAMEPLATE_FILE)" && exit 1)


# Validation rules
rbp-v.%: zrbp_validate_regimes_rule
zrbp_validate_regimes_rule: rbb_validate rbn_validate rbs_validate
	$(MBC_START) "Validating regimes"
	@test -n "$(RBM_MONIKER)"        || (echo "Error: RBM_MONIKER must be set"                    && exit 1)
	@test -f "$(RBM_NAMEPLATE_FILE)" || (echo "Error: Nameplate not found: $(RBM_NAMEPLATE_FILE)" && exit 1)

rbp_podman_machine_start_rule:
	$(MBC_START) "Start up correct podman machine"
	podman machine start
	$(MBC_PASS) "No errors."

rbp-s.%: rbp_start_service_rule
	$(MBC_STEP) "Completed delegate."

rbp_start_service_rule: zrbp_validate_regimes_rule
	$(MBC_START) "Starting Bottle Service -> $(RBM_MONIKER)"

	$(MBC_STEP) "Stopping any prior containers"
	-podman stop -t 2  $(RBM_SENTRY_CONTAINER)
	-podman rm   -f    $(RBM_SENTRY_CONTAINER)
	-podman stop -t 2  $(RBM_BOTTLE_CONTAINER)
	-podman rm   -f    $(RBM_BOTTLE_CONTAINER)

	$(MBC_STEP) "Cleaning up old netns and interfaces inside VM"
	cat $(MBV_TOOLS_DIR)/rbnc.cleanup.sh | $(zRBM_PODMAN_SSH_CMD)

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
	cat $(MBV_TOOLS_DIR)/rbns.sentry.sh | $(zRBM_PODMAN_SSH_CMD)

	$(MBC_STEP) "Configuring SENTRY security"
	cat $(MBV_TOOLS_DIR)/rbss.sentry.sh | podman exec -i $(RBM_SENTRY_CONTAINER) /bin/sh

	$(MBC_STEP) "Executing BOTTLE namespace setup script"
	cat $(MBV_TOOLS_DIR)/rbnb.bottle.sh | $(zRBM_PODMAN_SSH_CMD)

	$(MBC_STEP) "Visualizing network setup in podman machine..."
	cat $(MBV_TOOLS_DIR)/rbni.info.sh | $(zRBM_PODMAN_SSH_CMD)

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


rbp-s.%:
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to SENTRY"
	podman exec -it $(RBM_SENTRY_CONTAINER) /bin/bash
	$(MBC_PASS) "Done, no errors."


rbp-b.%: zrbp_validate_regimes_rule
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to BOTTLE"
	podman exec -it $(RBM_BOTTLE_CONTAINER) /bin/bash


rbp-i.%:  rbb_render rbn_render rbs_render
	$(MBC_PASS) "Done, no errors."


rbp-o.%: zrbp_validate_regimes_rule
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "OBSERVE BOTTLE SERVICE NETWORKS"
	(eval $(zRBM_EXPORT_ENV) && cat $(MBV_TOOLS_DIR)/rbo.observe.sh | /bin/sh)


# eof
