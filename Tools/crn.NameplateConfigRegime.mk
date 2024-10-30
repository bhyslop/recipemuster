# Recipe Bottle Service Configuration Regime
# This makefile defines the configuration requirements for Recipe Bottle Services

# Regime and Assignment Prefixes
#
# Regime Prefix: rbs
#
# Assignment Prefix: rbn

# Core Service Definition Rule
zrbs_define_rule:
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
	@echo "                              # Format: -v host:container:opts [-v ...]"
	@echo ""
	@echo "== Auto-start Configuration =="
	@echo "RBN_AUTOURL_ENABLED        # Set to 1 to enable URL auto-open, 0 otherwise"
	@echo "When RBN_AUTOURL_ENABLED=1, requires:"
	@echo "  RBN_AUTOURL_URL          # URL to open after service start"

# Core Validation Rules
zrbs_validate_rule: \
  zrbs_validate_moniker \
  zrbs_validate_description \
  zrbs_validate_sentry_image zrbs_validate_bottle_image \
  zrbs_validate_network \
  zrbs_validate_port_enabled \
  zrbs_validate_port_enabled_$(RBN_PORT_ENABLED) \
  zrbs_validate_outreach_enabled \
  zrbs_validate_outreach_$(RBN_OUTREACH_ENABLED) \
  zrbs_validate_autourl_enabled \
  zrbs_validate_autourl_$(RBN_AUTOURL_ENABLED)

zrbs_validate_moniker:
	@test -n "$(RBN_MONIKER)" || (echo "Error: RBN_MONIKER must be set" && exit 1)

zrbs_validate_description:
	@test -n "$(RBN_DESCRIPTION)" || (echo "Error: RBN_DESCRIPTION must be set" && exit 1)

zrbs_validate_sentry_image:
	@test -n "$(RBN_SENTRY_REPO_FULL_NAME)" || (echo "Error: RBN_SENTRY_REPO_FULL_NAME must be set" && exit 1)
	@test -n "$(RBN_SENTRY_IMAGE_TAG)"      || (echo "Error: RBN_SENTRY_IMAGE_TAG must be set" && exit 1)

zrbs_validate_bottle_image:
	@test -n "$(RBN_BOTTLE_REPO_FULL_NAME)" || (echo "Error: RBN_BOTTLE_REPO_FULL_NAME must be set" && exit 1)
	@test -n "$(RBN_BOTTLE_IMAGE_TAG)"      || (echo "Error: RBN_BOTTLE_IMAGE_TAG must be set" && exit 1)

zrbs_validate_network:
	@test -n "$(RBN_GUARDED_NETWORK_ID)" || (echo "Error: RBN_GUARDED_NETWORK_ID must be set" && exit 1)

zrbs_validate_port_enabled:
	@test "$(RBN_PORT_ENABLED)" = "0" -o "$(RBN_PORT_ENABLED)" = "1" || (echo "Error: RBN_PORT_ENABLED must be 0 or 1" && exit 1)

zrbs_validate_outreach_enabled:
	@test "$(RBN_OUTREACH_ENABLED)" = "0" -o "$(RBN_OUTREACH_ENABLED)" = "1" || (echo "Error: RBN_OUTREACH_ENABLED must be 0 or 1" && exit 1)

zrbs_validate_autourl_enabled:
	@test "$(RBN_AUTOURL_ENABLED)" = "0" -o "$(RBN_AUTOURL_ENABLED)" = "1" || (echo "Error: RBN_AUTOURL_ENABLED must be 0 or 1" && exit 1)

# Port validation rules
zrbs_validate_port_enabled_1:
	@test -n "$(RBN_PORT_HOST)"    || (echo "Error: RBN_PORT_HOST required when PORT_ENABLED=1"    && exit 1)
	@test -n "$(RBN_PORT_GUARDED)" || (echo "Error: RBN_PORT_GUARDED required when PORT_ENABLED=1" && exit 1)
	@test $(RBN_PORT_HOST)    -gt 0 -a $(RBN_PORT_HOST)    -lt 65536 || (echo "Error: RBN_PORT_HOST must be between 1-65535"    && exit 1)
	@test $(RBN_PORT_GUARDED) -gt 0 -a $(RBN_PORT_GUARDED) -lt 65536 || (echo "Error: RBN_PORT_GUARDED must be between 1-65535" && exit 1)

zrbs_validate_port_enabled_0:
	@: # No validation needed when ports disabled

# Outreach validation rules
zrbs_validate_outreach_1:
	@test -n "$(RBN_OUTREACH_CIDR)"   || (echo "Error: RBN_OUTREACH_CIDR required when OUTREACH_ENABLED=1"   && exit 1)
	@test -n "$(RBN_OUTREACH_DOMAIN)" || (echo "Error: RBN_OUTREACH_DOMAIN required when OUTREACH_ENABLED=1" && exit 1)
	@echo "$(RBN_OUTREACH_CIDR)" | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}/[0-9]{1,2}$$' ||\
	  (echo "Error: RBN_OUTREACH_CIDR must be valid CIDR notation" && exit 1)

zrbs_validate_outreach_0:
	@: # No validation needed when outreach disabled

# Autourl validation rules
zrbs_validate_autourl_1:
	@test -n "$(RBN_AUTOURL_URL)" || (echo "Error: RBN_AUTOURL_URL required when AUTOURL_ENABLED=1" && exit 1)

zrbs_validate_autourl_0:
	@: # No validation needed when autourl disabled

# Render Rule with Component Subrules
zrbs_render_rule: \
  zrbs_render_header \
  zrbs_render_images \
  zrbs_render_network \
  zrbs_render_port_$(RBN_PORT_ENABLED) \
  zrbs_render_outreach_$(RBN_OUTREACH_ENABLED) \
  zrbs_render_volumes \
  zrbs_render_autourl_$(RBN_AUTOURL_ENABLED)

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

zrbs_render_port_1:
	@echo "Port Service: ENABLED"
	@echo "  Host Port: $(RBN_PORT_HOST)"
	@echo "  Container Port: $(RBN_PORT_GUARDED)"
	@echo ""

zrbs_render_port_0:
	@echo "Port Service: DISABLED"
	@echo ""

zrbs_render_outreach_1:
	@echo "Internet Outreach: ENABLED"
	@echo "  Allowed CIDR: $(RBN_OUTREACH_CIDR)"
	@echo "  Allowed Domain: $(RBN_OUTREACH_DOMAIN)"
	@echo ""

zrbs_render_outreach_0:
	@echo "Internet Outreach: DISABLED"
	@echo ""

zrbs_render_volumes:
	@if [ -n "$(RBN_VOLUME_MOUNTS)" ]; then \
		echo "Volume Mounts:"; \
		echo "$(RBN_VOLUME_MOUNTS)" | tr ' ' '\n' | sed 's/^/  /'; \
	else \
		echo "Volume Mounts: None configured"; \
	fi
	@echo ""

zrbs_render_autourl_1:
	@echo "Auto-start URL: ENABLED"
	@echo "  URL: $(RBN_AUTOURL_URL)"

zrbs_render_autourl_0:
	@echo "Auto-start URL: DISABLED"

