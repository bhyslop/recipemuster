## © 2024 Scale Invariant.  All rights reserved.
##      Reference: https://www.termsfeed.com/blog/sample-copyright-notices/
##
## Unauthorized copying of this file, via any medium is strictly prohibited
## Proprietary and confidential
##
## Written by Brad Hyslop <bhyslop@scaleinvariant.org> November 2024

# Recipe Bottle Makefile (RBM)
# Implements secure containerized service management

# Directory structure
RBM_SCRIPTS_DIR      := RBM-scripts
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
include Tools/rbb.BaseConfigRegime.mk
include Tools/rbn.NameplateConfigRegime.mk
include Tools/rbs.StationConfigRegime.mk

# Container and network naming
RBM_SENTRY_CONTAINER  = $(RBM_MONIKER)-sentry
RBM_BOTTLE_CONTAINER  = $(RBM_MONIKER)-bottle
RBM_UPLINK_NETWORK    = $(RBM_MONIKER)-host
RBM_ENCLAVE_NETWORK   = $(RBM_MONIKER)-enclave

# Render rules
rbm-r%: rbs_render rbb_render rbn_render
	@test -n "$(RBM_MONIKER)" || (echo "Error: RBM_MONIKER must be set" && exit 1)
	@test -f "$(RBM_NAMEPLATE_PATH)" || (echo "Error: Nameplate not found: $(RBM_NAMEPLATE_PATH)" && exit 1)

# Validation rules
rbm-v%: rbb_validate rbn_validate rbs_validate
	@test -n "$(RBM_MONIKER)" || (echo "Error: RBM_MONIKER must be set" && exit 1)
	@test -f "$(RBM_NAMEPLATE_PATH)" || (echo "Error: Nameplate not found: $(RBM_NAMEPLATE_PATH)" && exit 1)

# Sentry Start Rule
.PHONY: opss_sentry_start
opss_sentry_start: rbm_validate
	@echo "Starting Sentry container for $(RBM_MONIKER)"
	
	# Network Creation Sequence
	-podman network rm -f $(RBM_UPLINK_NETWORK)
	-podman network rm -f $(RBM_ENCLAVE_NETWORK)
	podman network create --driver bridge $(RBM_UPLINK_NETWORK)
	podman network create --subnet $(RBB_ENCLAVE_SUBNET) \
	                     --gateway $(RBB_ENCLAVE_GATEWAY) \
	                     --internal \
	                     $(RBM_ENCLAVE_NETWORK)
	
	# Sentry Run Sequence
	-podman rm -f $(RBM_SENTRY_CONTAINER)
	podman run -d \
	    --name $(RBM_SENTRY_CONTAINER) \
	    --network $(RBM_UPLINK_NETWORK) \
	    --privileged \
	    $(if $(RBN_PORT_ENABLED),-p $(RBN_PORT_UPLINK):$(RBN_PORT_UPLINK)) \
	    $(RBN_SENTRY_REPO_FULL_NAME):$(RBN_SENTRY_IMAGE_TAG)
	
	# Network Connect Sequence
	podman network connect $(RBM_ENCLAVE_NETWORK) $(RBM_SENTRY_CONTAINER)
	timeout 5s sh -c "while ! podman exec $(RBM_SENTRY_CONTAINER) ip addr show eth1 | grep -q 'inet '; do sleep 0.2; done"
	
	# Security Configuration
	podman exec $(RBM_SENTRY_CONTAINER) /bin/sh $(RBM_SCRIPTS_DIR)/rbm-sentry-setup.sh > $(RBM_SENTRY_LOG) 2>&1

# Sessile Bottle Start Rule
.PHONY: opbs_bottle_start
opbs_bottle_start: opss_sentry_start
	@echo "Starting Sessile Bottle container for $(RBM_MONIKER)"
	
	# Bottle Cleanup Sequence
	-podman stop -t 30 $(RBM_BOTTLE_CONTAINER)
	-podman rm -f $(RBM_BOTTLE_CONTAINER)
	
	# Bottle Launch Sequence
	podman run -d \
	    --name $(RBM_BOTTLE_CONTAINER) \
	    --network $(RBM_ENCLAVE_NETWORK) \
	    --dns $(RBB_ENCLAVE_GATEWAY) \
	    $(RBN_VOLUME_MOUNTS) \
	    --restart unless-stopped \
	    $(RBN_BOTTLE_REPO_FULL_NAME):$(RBN_BOTTLE_IMAGE_TAG)

# Agile Bottle Run Rule
.PHONY: opbr_bottle_run
opbr_bottle_run: opss_sentry_start
	@echo "Running Agile Bottle container for $(RBM_MONIKER)"
	
	# Command must be provided
	@test -n "$(CMD)" || (echo "Error: CMD must be set" && exit 1)
	
	# Bottle Create and Execute Sequence
	podman run --rm \
	    --network $(RBM_ENCLAVE_NETWORK) \
	    --dns $(RBB_ENCLAVE_GATEWAY) \
	    $(RBN_VOLUME_MOUNTS) \
	    $(RBN_BOTTLE_REPO_FULL_NAME):$(RBN_BOTTLE_IMAGE_TAG) \
	    $(CMD)

# Sentry Stop Rule
.PHONY: opsx_sentry_stop
opsx_sentry_stop: rbm_validate
	@echo "Stopping Sentry container for $(RBM_MONIKER)"
	
	# Network disconnection
	-podman network disconnect $(RBM_ENCLAVE_NETWORK) $(RBM_SENTRY_CONTAINER)
	-podman network disconnect $(RBM_UPLINK_NETWORK) $(RBM_SENTRY_CONTAINER)
	
	# Container termination
	-podman stop -t 30 $(RBM_SENTRY_CONTAINER)
	-podman rm -f $(RBM_SENTRY_CONTAINER)
	
	# Network cleanup
	-podman network rm -f $(RBM_ENCLAVE_NETWORK)
	-podman network rm -f $(RBM_UPLINK_NETWORK)

# Bottle Stop Rule
.PHONY: opbx_bottle_stop
opbx_bottle_stop: rbm_validate
	@echo "Stopping Bottle container for $(RBM_MONIKER)"
	-podman stop -t 30 $(RBM_BOTTLE_CONTAINER)
	-podman rm -f $(RBM_BOTTLE_CONTAINER)

.PHONY: help
help:
	@echo "Recipe Bottle Makefile"
	@echo "Usage: make <target> RBM_MONIKER=<service-name>"
	@echo ""
	@echo "Targets:"
	@echo "  opss_sentry_start  - Start Sentry container with security configuration"
	@echo "  opbs_bottle_start  - Start persistent Bottle service"
	@echo "  opbr_bottle_run    - Run one-off Bottle command (requires CMD=<command>)"
	@echo "  opsx_sentry_stop   - Stop Sentry container and clean up networks"
	@echo "  opbx_bottle_stop   - Stop Bottle container"
	@echo ""
	@echo "Configuration from:"
	@echo "  Base Config:     Tools/rbb.BaseConfigRegime.mk"
	@echo "  Station Config:  Tools/rbs.StationConfigRegime.mk"
	@echo "  Nameplate:       $(RBB_NAMEPLATE_PATH)/<moniker>.mk"
	