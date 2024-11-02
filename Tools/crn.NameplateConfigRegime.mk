# Recipe Bottle Service Configuration Regime
# This makefile defines the configuration requirements for Recipe Bottle Services

# Regime Prefix: rbs_
# Assignment Prefix: RBN_

# Standard validation helper pattern implementation
zrbs_check_exported = env | grep -q ^$(1)= || (echo "Error: $(2)" && exit 1)
zrbs_check_eq = test $(1) = $(2) || (echo "Error: $(3)" && exit 1)
zrbs_check_ne = test $(1) != $(2) || (echo "Error: $(3)" && exit 1)
zrbs_check_lt = test $(1) -lt $(2) || (echo "Error: $(3)" && exit 1)
zrbs_check_le = test $(1) -le $(2) || (echo "Error: $(3)" && exit 1)
zrbs_check_gt = test $(1) -gt $(2) || (echo "Error: $(3)" && exit 1)
zrbs_check_ge = test $(1) -ge $(2) || (echo "Error: $(3)" && exit 1)
zrbs_check_range = test $(1) -ge $(2) -a $(1) -le $(3) || (echo "Error: $(4)" && exit 1)
zrbs_check_empty = test -z "$(1)" || (echo "Error: $(2)" && exit 1)
zrbs_check_nonempty = test -n "$(1)" || (echo "Error: $(2)" && exit 1)
zrbs_check_matches = echo "$(1)" | grep -Eq "$(2)" || (echo "Error: $(3)" && exit 1)
zrbs_check_startswith = echo "$(1)" | grep -q "^$(2)" || (echo "Error: $(3)" && exit 1)
zrbs_check_endswith = echo "$(1)" | grep -q "$(2)$$" || (echo "Error: $(3)" && exit 1)
zrbs_check_bool = test "$(1)" = "0" -o "$(1)" = "1" || (echo "Error: $(2)" && exit 1)

# Core Service Definition Rule
rbs_define:
	@echo "Nameplate Config Regime"
	@echo ""
	@echo "== Required Core Variables =="
	@echo "RBN_MONIKER              # Unique identifier for this service instance"
	@echo "RBN_DESCRIPTION          # Human-readable description of the service purpose"
	@echo ""
	@echo "== Image Source Configuration =="
	@echo "RBN_SENTRY_REPO_FULL_NAME  # Full repository path for sentry image"
	@echo "RBN_BOTTLE_REPO_FULL_NAME  # Full repository path for bottle image"
	@echo "RBN_SENTRY_IMAGE_TAG       # Tag for sentry image"
	@echo "RBN_BOTTLE_IMAGE_TAG       # Tag for bottle image"
	@echo ""
	@echo "== Network Configuration =="
	@echo "RBN_GUARDED_NETWORK_ID     # Unique network ID (e.g., 10.240)"
	@echo ""
	@echo "== Port Service Configuration =="
	@echo "RBN_PORT_ENABLED           # Set to 1 to enable port service, 0 otherwise"
	@echo "When RBN_PORT_ENABLED=1, requires:"
	@echo "  RBN_PORT_HOST            # External port for service access"
	@echo "  RBN_PORT_GUARDED         # Internal port for container service"
	@echo ""
	@echo "== Internet Outreach Configuration =="
	@echo "RBN_OUTREACH_ENABLED       # Set to 1 to enable internet access, 0 otherwise"
	@echo "When RBN_OUTREACH_ENABLED=1, requires:"
	@echo "  RBN_OUTREACH_CIDR        # CIDR range for allowed outbound traffic"
	@echo "  RBN_OUTREACH_DOMAIN      # Domain name for DNS resolution"
	@echo ""
	@echo "== Volume Mount Configuration =="
	@echo "RBN_VOLUME_MOUNTS          # Podman volume mount arguments"
	@echo "                           # Format: -v host:container:opts [-v ...]"
	@echo ""
	@echo "== Auto-start Configuration =="
	@echo "RBN_AUTOURL_ENABLED        # Set to 1 to enable URL auto-open, 0 otherwise"
	@echo "When RBN_AUTOURL_ENABLED=1, requires:"
	@echo "  RBN_AUTOURL_URL          # URL to open after service start"

# Core validation target
rbs_validate: zrbs_validate_core \
              zrbs_validate_images \
              zrbs_validate_network \
              zrbs_validate_port \
              zrbs_validate_outreach \
              zrbs_validate_autourl

# Core variable validation
zrbs_validate_core: zrbs_validate_moniker zrbs_validate_description

zrbs_validate_moniker:
	@$(call zrbs_check_exported,RBN_MONIKER,"RBN_MONIKER must be exported")
	@$(call zrbs_check_nonempty,$(RBN_MONIKER),"RBN_MONIKER must not be empty")

zrbs_validate_description:
	@$(call zrbs_check_exported,RBN_DESCRIPTION,"RBN_DESCRIPTION must be exported")
	@$(call zrbs_check_nonempty,$(RBN_DESCRIPTION),"RBN_DESCRIPTION must not be empty")

# Image validation
zrbs_validate_images: zrbs_validate_sentry_image zrbs_validate_bottle_image

zrbs_validate_sentry_image:
	@$(call zrbs_check_exported,RBN_SENTRY_REPO_FULL_NAME,"RBN_SENTRY_REPO_FULL_NAME must be exported")
	@$(call zrbs_check_nonempty,$(RBN_SENTRY_REPO_FULL_NAME),"RBN_SENTRY_REPO_FULL_NAME must not be empty")
	@$(call zrbs_check_exported,RBN_SENTRY_IMAGE_TAG,"RBN_SENTRY_IMAGE_TAG must be exported")
	@$(call zrbs_check_nonempty,$(RBN_SENTRY_IMAGE_TAG),"RBN_SENTRY_IMAGE_TAG must not be empty")

zrbs_validate_bottle_image:
	@$(call zrbs_check_exported,RBN_BOTTLE_REPO_FULL_NAME,"RBN_BOTTLE_REPO_FULL_NAME must be exported")
	@$(call zrbs_check_nonempty,$(RBN_BOTTLE_REPO_FULL_NAME),"RBN_BOTTLE_REPO_FULL_NAME must not be empty")
	@$(call zrbs_check_exported,RBN_BOTTLE_IMAGE_TAG,"RBN_BOTTLE_IMAGE_TAG must be exported")
	@$(call zrbs_check_nonempty,$(RBN_BOTTLE_IMAGE_TAG),"RBN_BOTTLE_IMAGE_TAG must not be empty")

# Network validation
zrbs_validate_network:
	@$(call zrbs_check_exported,RBN_GUARDED_NETWORK_ID,"RBN_GUARDED_NETWORK_ID must be exported")
	@$(call zrbs_check_nonempty,$(RBN_GUARDED_NETWORK_ID),"RBN_GUARDED_NETWORK_ID must not be empty")

# Port feature validation
zrbs_validate_port: zrbs_validate_port_enabled zrbs_validate_port_$(RBN_PORT_ENABLED)

zrbs_validate_port_enabled:
	@$(call zrbs_check_exported,RBN_PORT_ENABLED,"RBN_PORT_ENABLED must be exported")
	@$(call zrbs_check_bool,$(RBN_PORT_ENABLED),"RBN_PORT_ENABLED must be 0 or 1")

zrbs_validate_port_1:
	@$(call zrbs_check_exported,RBN_PORT_HOST,"RBN_PORT_HOST must be exported when PORT_ENABLED=1")
	@$(call zrbs_check_nonempty,$(RBN_PORT_HOST),"RBN_PORT_HOST must not be empty when PORT_ENABLED=1")
	@$(call zrbs_check_exported,RBN_PORT_GUARDED,"RBN_PORT_GUARDED must be exported when PORT_ENABLED=1")
	@$(call zrbs_check_nonempty,$(RBN_PORT_GUARDED),"RBN_PORT_GUARDED must not be empty when PORT_ENABLED=1")
	@$(call zrbs_check_range,$(RBN_PORT_HOST),1,65535,"RBN_PORT_HOST must be between 1 and 65535")
	@$(call zrbs_check_range,$(RBN_PORT_GUARDED),1,65535,"RBN_PORT_GUARDED must be between 1 and 65535")

zrbs_validate_port_0:
	@:

# Outreach feature validation
zrbs_validate_outreach: zrbs_validate_outreach_enabled zrbs_validate_outreach_$(RBN_OUTREACH_ENABLED)

zrbs_validate_outreach_enabled:
	@$(call zrbs_check_exported,RBN_OUTREACH_ENABLED,"RBN_OUTREACH_ENABLED must be exported")
	@$(call zrbs_check_bool,$(RBN_OUTREACH_ENABLED),"RBN_OUTREACH_ENABLED must be 0 or 1")

zrbs_validate_outreach_1:
	@$(call zrbs_check_exported,RBN_OUTREACH_CIDR,"RBN_OUTREACH_CIDR must be exported when OUTREACH_ENABLED=1")
	@$(call zrbs_check_nonempty,$(RBN_OUTREACH_CIDR),"RBN_OUTREACH_CIDR must not be empty when OUTREACH_ENABLED=1")
	@$(call zrbs_check_exported,RBN_OUTREACH_DOMAIN,"RBN_OUTREACH_DOMAIN must be exported when OUTREACH_ENABLED=1")
	@$(call zrbs_check_nonempty,$(RBN_OUTREACH_DOMAIN),"RBN_OUTREACH_DOMAIN must not be empty when OUTREACH_ENABLED=1")
	@$(call zrbs_check_matches,$(RBN_OUTREACH_CIDR),^([0-9]{1,3}\.){3}[0-9]{1,3}/[0-9]{1,2}$$,"RBN_OUTREACH_CIDR must be valid CIDR notation")

zrbs_validate_outreach_0:
	@:

# Autourl feature validation
zrbs_validate_autourl: zrbs_validate_autourl_enabled zrbs_validate_autourl_$(RBN_AUTOURL_ENABLED)

zrbs_validate_autourl_enabled:
	@$(call zrbs_check_exported,RBN_AUTOURL_ENABLED,"RBN_AUTOURL_ENABLED must be exported")
	@$(call zrbs_check_bool,$(RBN_AUTOURL_ENABLED),"RBN_AUTOURL_ENABLED must be 0 or 1")

zrbs_validate_autourl_1:
	@$(call zrbs_check_exported,RBN_AUTOURL_URL,"RBN_AUTOURL_URL must be exported when AUTOURL_ENABLED=1")
	@$(call zrbs_check_nonempty,$(RBN_AUTOURL_URL),"RBN_AUTOURL_URL must not be empty when AUTOURL_ENABLED=1")

zrbs_validate_autourl_0:
	@:

# Render target with component subrules
rbs_render: zrbs_render_header \
            zrbs_render_images \
            zrbs_render_network \
            zrbs_render_port \
            zrbs_render_outreach \
            zrbs_render_volumes \
            zrbs_render_autourl

zrbs_render_header:
	@echo "Recipe Bottle Service Configuration: $(RBN_MONIKER)"
	@echo "Description: $(RBN_DESCRIPTION)"
	@echo ""

zrbs_render_images:
	@echo "Image Configuration:"
	@echo "  Sentry Image: $(RBN_SENTRY_REPO_FULL_NAME):$(RBN_SENTRY_IMAGE_TAG)"
	@echo "  Bottle Image: $(RBN_BOTTLE_REPO_FULL_NAME):$(RBN_BOTTLE_IMAGE_TAG)"
	@echo ""

zrbs_render_network:
	@echo "Network Configuration:"
	@echo "  Guarded Network ID: $(RBN_GUARDED_NETWORK_ID)"
	@echo ""

# Port feature render rules
zrbs_render_port: zrbs_render_port_status \
                  zrbs_render_port_$(RBN_PORT_ENABLED)

zrbs_render_port_status:
    @echo "Feature: $(if $(filter 1,$(RBN_PORT_ENABLED)),ENABLED,DISABLED)"

zrbs_render_port_1:
    @echo "  Host Port: $(RBN_PORT_HOST)"
    @echo "  Container Port: $(RBN_PORT_GUARDED)"

zrbs_render_port_0:
    @:

zrbs_render_port_:
    @echo "Error: RBN_PORT_ENABLED must be defined as 0 or 1" && exit 1

# Outreach feature render rules
zrbs_render_outreach: zrbs_render_outreach_status \
                      zrbs_render_outreach_$(RBN_OUTREACH_ENABLED)

zrbs_render_outreach_status:
    @echo "Feature: $(if $(filter 1,$(RBN_OUTREACH_ENABLED)),ENABLED,DISABLED)"

zrbs_render_outreach_1:
    @echo "  CIDR: $(RBN_OUTREACH_CIDR)"
    @echo "  Domain: $(RBN_OUTREACH_DOMAIN)"

zrbs_render_outreach_0:
    @:

zrbs_render_outreach_:
    @echo "Error: RBN_OUTREACH_ENABLED must be defined as 0 or 1" && exit 1

# Autourl feature render rules
zrbs_render_autourl: zrbs_render_autourl_status \
                     zrbs_render_autourl_$(RBN_AUTOURL_ENABLED)

zrbs_render_autourl_status:
    @echo "Feature: $(if $(filter 1,$(RBN_AUTOURL_ENABLED)),ENABLED,DISABLED)"

zrbs_render_autourl_1:
    @echo "  URL: $(RBN_AUTOURL_URL)"

zrbs_render_autourl_0:
    @:

zrbs_render_autourl_:
    @echo "Error: RBN_AUTOURL_ENABLED must be defined as 0 or 1" && exit 1

