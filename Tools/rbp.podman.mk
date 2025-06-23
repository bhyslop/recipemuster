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
export RBM_MACHINE            = pdvm-rbw
export RBM_CONNECTION         = -c $(RBM_MACHINE)
export RBM_SERVICE_POD        = $(RBM_MONIKER)-pod

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


zRBM_STASH_MACHINE            = pdvm-stash
zRBM_STASH_SSH                = podman machine ssh $(zRBM_STASH_MACHINE)
zRBM_STASH_TAG_SAFE           = $(subst :,-,$(subst /,-,$(RBRR_VMDIST_TAG)))
zRBM_STASH_SHA_SHORT          = $(shell echo $(RBRR_VMDIST_BLOB_SHA) | cut -c1-12)
zRBM_STASH_RAW_INIT           = $(MBD_TEMP_DIR)/podman-machine-init-raw.txt
zRBM_STASH_IMAGE_INDEX_DIGEST = $(MBD_TEMP_DIR)/podvm-image-index-digest.txt
zRBM_STASH_LATEST_INDEX       = $(MBD_TEMP_DIR)/podman-latest-index.json
zRBM_STASH_LATEST_PLATFORM    = $(MBD_TEMP_DIR)/podman-latest-platform-manifest.json
zRBM_STASH_PLATFORM_DIGEST    = $(MBD_TEMP_DIR)/podman-latest-platform-digest.json
zRBM_STASH_ALL_PLATFORM_DIGESTS = $(MBD_TEMP_DIR)/podman-all-platform-digests.json
zRBM_STASH_CONCAT_BLOB_DIGESTS  = $(MBD_TEMP_DIR)/podman-concat-blob-digests.json
zRBM_STASH_LATEST_EXTRACT     = $(MBD_TEMP_DIR)/podman_manifest_info.json
zRBM_STASH_CURRENTLY_CHOSEN   = $(MBD_TEMP_DIR)/podman_manifest_chosen.json
RBP_STASH_IMAGE               = $(zRBG_GIT_REGISTRY)/$(RBRR_REGISTRY_OWNER)/$(RBRR_REGISTRY_NAME):stash-$(zRBM_STASH_TAG_SAFE)-$(zRBM_STASH_SHA_SHORT)


rbp_stash_check_rule: mbc_demo_rule
	@### $(MBC_STEP) "Your vm will be for architecture:" $(RBRS_PODMAN_ARCHITECTURE)
	@### $(MBC_START) "Download default podman machine to be a safe place to learn about latest machine version"
	@### -podman machine stop  $(RBM_MACHINE)
	@### -podman machine stop  $(zRBM_STASH_MACHINE)
	@### -podman machine rm -f $(zRBM_STASH_MACHINE)
	@### ## $(MBC_STEP) "OUCH RESTORE: Nuke all prior machine cache"
	@### ## rm -rf $(RBRS_PODMAN_ROOT_DIR)/machine/*
	@### $(MBC_STEP) "Acquire default podman machine (latest for your podman, uncontrolled)..."
	@### podman machine init  $(zRBM_STASH_MACHINE) > $(zRBM_STASH_RAW_INIT) 2>&1
	@### $(MBC_STEP) "Uncontrolled transcript"
	@### @cat $(zRBM_STASH_RAW_INIT)
	@### $(MBC_STEP) "Start uncontrolled..."
	@### podman machine start  $(zRBM_STASH_MACHINE)
	@### $(MBC_STEP) "Install crane for bridging your container registry..."
	@### $(zRBM_STASH_SSH) curl     -o   crane.tar.gz -L $(RBRR_VMDIST_CRANE)
	@### $(zRBM_STASH_SSH) sudo tar -xzf crane.tar.gz -C /usr/local/bin/ crane
	@### $(MBC_STEP) "Log in to your container registry with podman and crane..."
	@### source $(RBRR_GITHUB_PAT_ENV)  &&  podman -c $(zRBM_STASH_MACHINE) login $(zRBG_GIT_REGISTRY) -u $$RBV_USERNAME -p $$RBV_PAT
	@### source $(RBRR_GITHUB_PAT_ENV)  &&  $(zRBM_STASH_SSH) crane auth    login $(zRBG_GIT_REGISTRY) -u $$RBV_USERNAME -p $$RBV_PAT
	@### $(MBC_STEP) "Install jq using dnf package manager (Fedora's default)"
	@### $(zRBM_STASH_SSH) sudo dnf install -y jq --setopt=subscription-manager.disable=1
	@### false

	$(MBC_STEP) "Validating image version against pinned values..."
	@echo "Checking tag: $(RBRR_VMDIST_TAG)"

	$(MBC_STEP) "OUCH de-hardcode below"
	$(MBC_STEP) "Execute the script by piping it directly to the VM and capture the JSON output..."
	cat $(MBV_TOOLS_DIR)/vme.extractor.sh | $(zRBM_STASH_SSH) "sh -s quay.io podman/machine-os-wsl 5.3 crane" \
	  > $(zRBM_STASH_LATEST_EXTRACT)

	$(MBC_STEP) "Visualize" $(zRBM_STASH_LATEST_EXTRACT)
	cat $(zRBM_STASH_LATEST_EXTRACT) | jq

	$(MBC_SHOW_WHITE) "What I see at https://quay.io/repository/podman/machine-os-wsl?tab=tags looks like below:"
	$(MBC_SHOW_CYAN)   "Top Manifest Digest: " $$(jq -r '.index_digest'        < $(zRBM_STASH_LATEST_EXTRACT))
	$(MBC_SHOW_VIOLET) "Blob filter pattern: " $$(jq -r '.blob_filter_pattern' < $(zRBM_STASH_LATEST_EXTRACT))
	$(MBC_SHOW_ORANGE) "Canonical Tag:       " $$(jq -r '.canonical_tag'       < $(zRBM_STASH_LATEST_EXTRACT))
	echo

	$(MBC_STEP) "Determine if your selected VM is available and valid in your Github Container Registry..."

	$(MBC_STEP) "Extracting manifest for stashed image..."
	cat $(MBV_TOOLS_DIR)/vme.extractor.sh | $(zRBM_STASH_SSH) "sh -s ghcr.io bhyslop/recipemuster stash-quay.io-podman-machine-os-wsl-5.3-6898117ca935 crane" \
	  > $(zRBM_STASH_CURRENTLY_CHOSEN)
	cat $(zRBM_STASH_CURRENTLY_CHOSEN) | jq

	$(MBC_SHOW_RED) "OUCH FINISH"
	false


rbp_stash_update_rule:
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

	$(MBC_STEP) "Stopping any existing containers"
	-podman $(RBM_CONNECTION) stop -t 2 $(RBM_SENTRY_CONTAINER)
	-podman $(RBM_CONNECTION) rm -f $(RBM_SENTRY_CONTAINER)
	-podman $(RBM_CONNECTION) stop -t 2 $(RBM_BOTTLE_CONTAINER)
	-podman $(RBM_CONNECTION) rm -f $(RBM_BOTTLE_CONTAINER)

	$(MBC_STEP) "Removing any existing pod"
	-podman $(RBM_CONNECTION) pod rm -f $(RBM_SERVICE_POD)

	$(MBC_STEP) "Creating new pod"
	podman $(RBM_CONNECTION) pod create \
	  --name $(RBM_SERVICE_POD) \
	  --dns 127.0.0.1 \
	  $(if $(RBRN_ENTRY_ENABLED),-p $(RBRN_ENTRY_PORT_WORKSTATION):$(RBRN_ENTRY_PORT_WORKSTATION))

	$(MBC_STEP) "Launching SENTRY container"
	podman $(RBM_CONNECTION) run -d \
	  --name $(RBM_SENTRY_CONTAINER) \
	  --pod $(RBM_SERVICE_POD) \
	  --cap-add NET_ADMIN \
	  --cap-add NET_RAW \
	  $(addprefix -e ,$(RBRR__ROLLUP_ENVIRONMENT_VAR)) \
	  $(addprefix -e ,$(RBRN__ROLLUP_ENVIRONMENT_VAR)) \
	  $(RBRN_SENTRY_REPO_PATH):$(RBRN_SENTRY_IMAGE_TAG)

	$(MBC_STEP) "Waiting for SENTRY container"
	sleep 2
	podman $(RBM_CONNECTION) ps | grep $(RBM_SENTRY_CONTAINER) || (echo 'Container not running' && exit 1)

	$(MBC_STEP) "Configuring SENTRY security"
	podman $(RBM_CONNECTION) exec -i $(RBM_SENTRY_CONTAINER) /bin/sh < $(MBV_TOOLS_DIR)/rbss.sentry.sh

	$(MBC_STEP) "Launching BOTTLE container"
	podman $(RBM_CONNECTION) run -d \
	  --name $(RBM_BOTTLE_CONTAINER) \
	  --pod $(RBM_SERVICE_POD) \
	  --cap-drop ALL \
	  --user $(RBRR_BOTTLE_UID) \
	  --restart unless-stopped \
	  $(RBRN_VOLUME_MOUNTS) \
	  $(addprefix -e ,$(RBRR__ROLLUP_ENVIRONMENT_VAR)) \
	  $(addprefix -e ,$(RBRN__ROLLUP_ENVIRONMENT_VAR)) \
	  $(RBRN_BOTTLE_REPO_PATH):$(RBRN_BOTTLE_IMAGE_TAG)

	$(MBC_STEP) "Waiting for BOTTLE container"
	sleep 2
	podman $(RBM_CONNECTION) ps | grep $(RBM_BOTTLE_CONTAINER) || (echo 'Container not running' && exit 1)

	$(MBC_PASS) "Bottle service should be available now."

rbp_connect_sentry_rule:
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to SENTRY"
	podman $(RBM_CONNECTION) exec -it $(RBM_SENTRY_CONTAINER) /bin/bash
	$(MBC_PASS) "Done, no errors."


rbp_connect_bottle_rule: zrbp_validate_regimes_rule
	$(MBC_START) "Moniker:"$(RBM_ARG_MONIKER) "Connecting to BOTTLE"
	$(zRBM_PODMAN_RAW_CMD) exec -it $(RBM_BOTTLE_CONTAINER) /bin/bash


# eof
