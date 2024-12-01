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
	@echo "Starting Sentry container for $(RBM_MONIKER)"

	# Network Creation Sequence
	-podman network rm -f $(RBM_UPLINK_NETWORK)
	-podman network rm -f $(RBM_ENCLAVE_NETWORK)
	podman network create --driver bridge $(RBM_UPLINK_NETWORK)
	podman network create --subnet $(RBB_ENCLAVE_SUBNET)           \
	                     --gateway $(RBB_ENCLAVE_GATEWAY)          \
	                     --internal                                \
	                     $(RBM_ENCLAVE_NETWORK)

	# Sentry Run Sequence
	-podman rm -f $(RBM_SENTRY_CONTAINER)
	podman run -d                                                            \
	    --name $(RBM_SENTRY_CONTAINER)                                       \
	    --network $(RBM_UPLINK_NETWORK)                                      \
	    --privileged                                                         \
	    $(if $(RBN_PORT_ENABLED),-p $(RBN_PORT_UPLINK):$(RBN_PORT_UPLINK))   \
	    $(addprefix -e ,$(RBB__ROLLUP_ENVIRONMENT_VAR))                      \
	    $(addprefix -e ,$(RBN__ROLLUP_ENVIRONMENT_VAR))                      \
	    $(RBN_SENTRY_REPO_FULL_NAME):$(RBN_SENTRY_IMAGE_TAG)

	# Network Connect Sequence
	podman network connect $(RBM_ENCLAVE_NETWORK) $(RBM_SENTRY_CONTAINER)
	timeout 5s sh -c "while ! podman exec $(RBM_SENTRY_CONTAINER) ip addr show eth1 | grep -q 'inet '; do sleep 0.2; done"

	# Security Configuration
	cat $(RBM_SCRIPTS_DIR)/rbm-sentry-setup.sh | podman exec -i $(RBM_SENTRY_CONTAINER) /bin/sh


rbm-BS%: zrbm_start_bottle_rule
	@echo "Completed delegate."
zrbm_start_bottle_rule:
	@echo "Starting Sessile Bottle container for $(RBM_MONIKER)"
	
	# Bottle Cleanup Sequence
	-podman stop -t 30 $(RBM_BOTTLE_CONTAINER)
	-podman rm -f      $(RBM_BOTTLE_CONTAINER)
	
	# Bottle Launch Sequence
	podman run -d                            \
	    --name    $(RBM_BOTTLE_CONTAINER)    \
	    --network $(RBM_ENCLAVE_NETWORK)     \
	    --dns     $(RBB_ENCLAVE_GATEWAY)     \
	    --restart unless-stopped             \
	    $(RBN_VOLUME_MOUNTS)                 \
	    $(RBN_BOTTLE_REPO_FULL_NAME):$(RBN_BOTTLE_IMAGE_TAG)


rbm-br%: zrbm_validate_regimes_rule
	@echo "Running Agile Bottle container for $(RBM_MONIKER)"
	
	# Command must be provided
	@test -n "$(CMD)" || (echo "Error: CMD must be set" && exit 1)
	
	# Bottle Create and Execute Sequence
	podman run --rm                                           \
	    --network $(RBM_ENCLAVE_NETWORK)                      \
	    --dns     $(RBB_ENCLAVE_GATEWAY)                      \
	    $(RBN_VOLUME_MOUNTS)                                  \
	    $(RBN_BOTTLE_REPO_FULL_NAME):$(RBN_BOTTLE_IMAGE_TAG)  \
	    $(CMD)


# Sentry Stop Rule
rbm-SX%: zrbm_validate_regimes_rule
	@echo "Stopping Sentry container for $(RBM_MONIKER)"
	
	# Network disconnection
	-podman network disconnect $(RBM_ENCLAVE_NETWORK) $(RBM_SENTRY_CONTAINER)
	-podman network disconnect $(RBM_UPLINK_NETWORK)  $(RBM_SENTRY_CONTAINER)
	
	# Container termination
	-podman stop -t 30 $(RBM_SENTRY_CONTAINER)
	-podman rm -f      $(RBM_SENTRY_CONTAINER)
	
	# Network cleanup
	-podman network rm -f $(RBM_ENCLAVE_NETWORK)
	-podman network rm -f $(RBM_UPLINK_NETWORK)


# Bottle Stop Rule
rbm-BX%: zrbm_validate_regimes_rule
	@echo "Stopping Bottle container for $(RBM_MONIKER)"
	-podman stop -t 30 $(RBM_BOTTLE_CONTAINER)
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
	podman exec -it $(RBM_SENTRY_CONTAINER) /bin/sh
	$(MBC_PASS) "Done, no errors."


rbm-cb%: zrbm_validate_regimes_rule
	@echo "Moniker:"$(RBM_ARG_MONIKER) "Connecting to BOTTLE"
	podman exec -it $(RBM_BOTTLE_CONTAINER) /bin/bash
