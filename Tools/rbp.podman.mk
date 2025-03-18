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
export RBM_MACHINE            = pdvm-rbw
export RBM_CONNECTION         = -c $(RBM_MACHINE)

# Consolidated passed variables
zRBM_ROLLUP_ENV = $(filter RBM_%,$(.VARIABLES))

zRBM_EXPORT_ENV := "$(foreach v,$(RBRN__ROLLUP_ENVIRONMENT_VAR),export $v;) " \
                   "$(foreach v,$(RBRR__ROLLUP_ENVIRONMENT_VAR),export $v;) " \
                   "$(foreach v,$(zRBM_ROLLUP_ENV),export $v=\"$($v)\";) "    \
                   "PODMAN_IGNORE_CGROUPSV1_WARNING=1 "

zRBM_PODMAN_RAW_CMD   = podman $(RBM_CONNECTION)
zRBM_PODMAN_SSH_CMD   = podman machine ssh $(RBM_MACHINE) $(zRBM_EXPORT_ENV) 
zRBM_PODMAN_SHELL_CMD = $(zRBM_PODMAN_SSH_CMD) /bin/sh

zrbp_validate_regimes_rule: rbrn_validate rbrr_validate
	$(MBC_START) "Validating regimes"
	@test -n "$(RBM_MONIKER)"        || (echo "Error: RBM_MONIKER must be set"                    && exit 1)
	@test -f "$(RBM_NAMEPLATE_FILE)" || (echo "Error: Nameplate not found: $(RBM_NAMEPLATE_FILE)" && exit 1)


zRBM_STASH_MACHINE         = pdvm-stash
zRBM_STASH_SSH             = podman machine ssh $(zRBM_STASH_MACHINE)
zRBM_STASH_TAG_SAFE        = $(subst :,-,$(subst /,-,$(RBRR_VMDIST_TAG)))
zRBM_STASH_SHA_SHORT       = $(shell echo $(RBRR_VMDIST_BLOB_SHA) | cut -c1-12)
zRBM_STASH_RAW_INIT        = $(MBD_TEMP_DIR)/podman-machine-init-raw.txt
zRBM_STASH_LATEST_INDEX    = $(MBD_TEMP_DIR)/podman-latest-index.json
zRBM_STASH_LATEST_PLATFORM = $(MBD_TEMP_DIR)/podman-latest-platform-manifest.json
zRBM_STASH_PLATFORM_DIGEST = $(MBD_TEMP_DIR)/podman-latest-platform-digest.json
RBP_STASH_IMAGE            = $(zRBG_GIT_REGISTRY)/$(RBRR_REGISTRY_OWNER)/$(RBRR_REGISTRY_NAME):stash-$(zRBM_STASH_TAG_SAFE)-$(zRBM_STASH_SHA_SHORT)


rbp_stash_start_rule:
	$(MBC_STEP) "Your vm will be for architecture $(RBS_PODMAN_ARCHITECTURE)..."
	$(MBC_START) "Set up a podman machine just to stash the desired podman vm image in your container repo"
	-podman machine stop  $(RBM_MACHINE)
	-podman machine stop  $(zRBM_STASH_MACHINE)
	-podman machine rm -f $(zRBM_STASH_MACHINE)
	$(MBC_STEP) "Acquire default podman machine (latest for your podman, uncontrolled)..."
	podman machine init   $(zRBM_STASH_MACHINE) -vvv > $(zRBM_STASH_RAW_INIT) 2>&1
	$(MBC_STEP) "Uncontrolled transcript"
	@cat $(zRBM_STASH_RAW_INIT)
	$(MBC_STEP) "Start uncontrollled..."
	podman machine start  $(zRBM_STASH_MACHINE)
	$(MBC_STEP) "Install crane for bridging your container registry..."
	$(zRBM_STASH_SSH) curl     -o   crane.tar.gz -L $(RBRR_VMDIST_CRANE)
	$(zRBM_STASH_SSH) sudo tar -xzf crane.tar.gz -C /usr/local/bin/ crane
	$(MBC_STEP) "Log in to your container registry with podman..."
	source $(RBRR_GITHUB_PAT_ENV)  && \
	  podman -c $(zRBM_STASH_MACHINE) login $(zRBG_GIT_REGISTRY) -u $$RBV_USERNAME -p $$RBV_PAT
	$(MBC_STEP) "Log into your container registry with crane..."
	source $(RBRR_GITHUB_PAT_ENV) && \
	  $(zRBM_STASH_SSH) crane auth    login $(zRBG_GIT_REGISTRY) -u $$RBV_USERNAME -p $$RBV_PAT
	
	$(MBC_STEP) "Validating image version against pinned values..."
	@echo "Checking tag: $(RBRR_VMDIST_TAG)"

	$(MBC_STEP) "Retrieve latest index..."
	$(zRBM_STASH_SSH) crane manifest $(RBRR_VMDIST_TAG) > $(zRBM_STASH_LATEST_INDEX)
	$(MBC_STEP) "Show latest index..."
	jq < $(zRBM_STASH_LATEST_INDEX)

	$(MBC_STEP) "Extract platform digest for $(RBS_PODMAN_ARCHITECTURE)..."
	jq -r '.manifests[] | select(.platform.architecture == "$(RBS_PODMAN_ARCHITECTURE)") | .digest' \
	   < $(zRBM_STASH_LATEST_INDEX) > $(zRBM_STASH_PLATFORM_DIGEST)

	$(MBC_STEP) "Fetch platform manifest using digest..."
	$(zRBM_STASH_SSH) crane manifest $(RBRR_VMDIST_TAG)@$$(cat $(zRBM_STASH_PLATFORM_DIGEST)) > $(zRBM_STASH_LATEST_PLATFORM)
	$(MBC_STEP) "Show platform manifest..."
	jq < $(zRBM_STASH_LATEST_PLATFORM)

	$(MBC_STEP) "Extract blob digests from platform manifest..."
	jq -r '.layers[].digest' < $(zRBM_STASH_LATEST_PLATFORM)

	$(MBC_STEP) "TODO: REPORT ON WHETHER LATEST MATCHES PINNED."
	false

	$(MBC_STEP) "Get index manifest and verify SHA..."
	$(zRBM_STASH_SSH) "crane digest $(RBRR_VMDIST_TAG) > /tmp/current_index_digest"
	$(zRBM_STASH_SSH) "cat /tmp/current_index_digest"
	$(zRBM_STASH_SSH) "test $$(cat /tmp/current_index_digest) = sha256:$(RBRR_VMDIST_INDEX_SHA)" || \
	  $(MBC_SEE_YELLOW) "WARN: Image index digest mismatch! Expected sha256:$(RBRR_VMDIST_INDEX_SHA)"
	
	$(MBC_STEP) "Get platform-specific manifest for x86_64..."
	$(zRBM_STASH_SSH) "crane manifest $(RBRR_VMDIST_TAG) > /tmp/vm_manifest.json"
	
	$(MBC_STEP) "Extract the x86_64 digest from the manifest and check..."
	$(zRBM_STASH_SSH) "jq -r '.manifests[] | select(.platform.architecture == \"x86_64\") | .digest' /tmp/vm_manifest.json > /tmp/x86_digest"
	$(zRBM_STASH_SSH) "cat /tmp/x86_digest"
	$(zRBM_STASH_SSH) "test $$(cat /tmp/x86_digest) = \"sha256:$(RBRR_VMDIST_X86_SHA)\"" || \
	  $(MBC_SEE_YELLOW) "WARN: x86_64 platform manifest digest mismatch! Expected sha256:$(RBRR_VMDIST_X86_SHA)"
	
	$(MBC_STEP) "Get the x86_64 manifest and check for the expected blob SHA..."
	$(zRBM_STASH_SSH) "crane manifest $$(cat /tmp/x86_digest) > /tmp/platform_manifest.json"
	$(zRBM_STASH_SSH) "jq -r '.layers[].digest' /tmp/platform_manifest.json > /tmp/blob_digest"
	$(zRBM_STASH_SSH) "cat /tmp/blob_digest"
	$(zRBM_STASH_SSH) "test $$(cat /tmp/blob_digest) = \"sha256:$(RBRR_VMDIST_BLOB_SHA)\"" || \
	  $(MBC_SEE_YELLOW) "WARN: Content blob digest mismatch! Expected sha256:$(RBRR_VMDIST_BLOB_SHA)"
	
	$(MBC_PASS) "Resolve warnings above to use latest tag for  $(zRBM_STASH_MACHINE)"

rbp_stash_finish_rule:
	$(MBC_START) "Finish steps of acquiring a controlled machine version..."
	@echo "Working with VM distribution: $(RBRR_VMDIST_TAG)"

	$(MBC_STEP) "Gather information about your chosen vm..."
	$(zRBM_STASH_SSH) "crane manifest $(RBRR_VMDIST_TAG) > /tmp/vm_manifest.json"
	$(zRBM_STASH_SSH) "cat /tmp/vm_manifest.json"

	$(MBC_STEP) "Creating controlled VM image reference..."
	@echo "Stashed image name:      $(RBP_STASH_IMAGE)"
	@echo "Registry:                $(zRBG_GIT_REGISTRY)"
	@echo "Owner:                   $(RBRR_REGISTRY_OWNER)"
	@echo "Repository:              $(RBRR_REGISTRY_NAME)"
	@echo "Selected VM Blob SHA:    $(RBRR_VMDIST_BLOB_SHA)"

	$(MBC_STEP) "Checking if controlled image exists in registry..."
	-$(zRBM_STASH_SSH) "crane manifest $(RBP_STASH_IMAGE) > /tmp/inspect_result 2>&1" && \
	  cat /tmp/inspect_result && echo "Image already exists in registry" || \
	  (echo "Controlled image not found in registry" && \
	   echo "Starting copy from $(RBRR_VMDIST_TAG) to $(RBP_STASH_IMAGE)..." && \
	   source $(RBRR_GITHUB_PAT_ENV) && \
	   $(zRBM_STASH_SSH) "crane copy $(RBRR_VMDIST_TAG) $(RBP_STASH_IMAGE)" && \
	   echo "Copy completed successfully")

	$(MBC_STEP) "Verifying controlled image matches source image..."
	@echo "Retrieving source image digest..."
	$(zRBM_STASH_SSH) "crane digest $(RBRR_VMDIST_TAG) > /tmp/source_digest"
	$(zRBM_STASH_SSH) "cat /tmp/source_digest"

	@echo "Retrieving controlled image digest..."
	$(zRBM_STASH_SSH) "crane digest $(RBP_STASH_IMAGE) > /tmp/controlled_digest"
	$(zRBM_STASH_SSH) "cat /tmp/controlled_digest"

	$(MBC_STEP) "Comparing digests..."
	$(zRBM_STASH_SSH) "cmp -s /tmp/source_digest /tmp/controlled_digest" && \
	  echo "? Digests match - image integrity verified" || \
	  (echo "? FAILURE: Digests do not match!" && false)

	$(MBC_PASS) "Ready to use controlled VM image $(RBP_STASH_IMAGE)"

rbp_podman_machine_start_rule:
	$(MBC_START) "Start the podman machine needed for Bottle Services"
	$(MBC_STEP) "Shutdown the wrangler, if started"
	-podman machine stop $(zRBM_STASH_MACHINE)
	$(MBC_STEP) "Log version info"
	podman --version
	$(MBC_STEP) "Initialize Podman machine if it doesn't exist"
	podman machine list | grep -q "$(RBM_MACHINE)" || \
	  PODMAN_MACHINE_CGROUP=systemd podman machine init --image docker://$(RBP_STASH_IMAGE) $(RBM_MACHINE)
	$(MBC_STEP) "Start up Podman machine $(RBM_MACHINE)"
	podman machine start $(RBM_MACHINE)
	$(MBC_STEP) "Update utilities..."
	podman machine ssh $(RBM_MACHINE) \
	  sudo dnf install -y tcpdump --setopt=subscription-manager.disable=1
	$(MBC_STEP) "Version info on machine..."
	podman $(RBM_CONNECTION) version
	podman machine inspect $(RBM_MACHINE)
	podman machine ssh $(RBM_MACHINE) "cat /etc/os-release && uname -r"
	podman $(RBM_CONNECTION) info
	$(MBC_PASS) "No errors."

rbp_podman_machine_stop_rule:
	$(MBC_START) "Stopping machine $(RBM_MACHINE)"
	podman machine stop $(RBM_MACHINE)
	$(MBC_PASS) "No errors."

rbp_podman_machine_nuke_rule:
	$(MBC_START) "Try stopping before removal"
	-podman machine stop  $(RBM_MACHINE)
	$(MBC_STEP) "Now remove"
	podman  machine rm -f $(RBM_MACHINE)
	$(MBC_PASS) "No errors."

rbp_check_connection:
	$(MBC_START) "Checking connection to $(RBM_MACHINE)"
	podman $(RBM_CONNECTION) info > /dev/null || (echo "Unable to connect to machine" && exit 1)
	$(MBC_PASS) "Connection successful."

rbp_start_service_rule: zrbp_validate_regimes_rule rbp_check_connection
	$(MBC_START) "Starting Bottle Service -> $(RBM_MONIKER)"

	$(MBC_STEP) "Stopping any prior containers"
	-podman $(RBM_CONNECTION) stop -t 2  $(RBM_SENTRY_CONTAINER)
	-podman $(RBM_CONNECTION) rm   -f    $(RBM_SENTRY_CONTAINER)
	-$(zRBM_PODMAN_RAW_CMD) stop -t 2  $(RBM_BOTTLE_CONTAINER)
	-$(zRBM_PODMAN_RAW_CMD) rm   -f    $(RBM_BOTTLE_CONTAINER)

	$(MBC_STEP) "Cleaning up old netns and interfaces inside VM"
	$(zRBM_PODMAN_SHELL_CMD) < $(MBV_TOOLS_DIR)/rbnc.cleanup.sh

	$(MBC_STEP) "Launching SENTRY container with bridging for internet"
	podman $(RBM_CONNECTION) run -d                    \
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

	$(MBC_STEP) "Creating but not starting BOTTLE container"
	$(zRBM_PODMAN_RAW_CMD) create                                    \
	  --name $(RBM_BOTTLE_CONTAINER)                                 \
	  --privileged                                                   \
	  --network none                                                 \
	  --cap-add net_raw                                              \
	  --security-opt label=disable                                   \
	  $(RBRN_BOTTLE_REPO_PATH):$(RBRN_BOTTLE_IMAGE_TAG)

	$(MBC_STEP) "Executing BOTTLE namespace setup using container PID"
	$(zRBM_PODMAN_SHELL_CMD) < $(MBV_TOOLS_DIR)/rbnb.bottle.sh

	$(MBC_STEP) "Visualizing network setup in podman machine..."
	$(zRBM_PODMAN_SHELL_CMD) < $(MBV_TOOLS_DIR)/rbni.info.sh

	$(MBC_STEP) "Starting BOTTLE container now that networking is configured"
	$(zRBM_PODMAN_RAW_CMD) start $(RBM_BOTTLE_CONTAINER)

	$(MBC_STEP) "Waiting for BOTTLE container"
	sleep 2
	$(zRBM_PODMAN_RAW_CMD) "ps | grep $(RBM_BOTTLE_CONTAINER) || (echo 'Container not running' && exit 1)"

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
