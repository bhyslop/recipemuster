# Recipe Bottle Service Configuration Regime
# This makefile defines the configuration requirements for Recipe Bottle Services

# Regime and Assignment Prefixes
RBS_ := RBS_  # Regime Bottle Service prefix
RBN_ := RBN_  # Recipe Bottle Nameplate prefix (Assignment prefix)

# Core Service Definition Target
.PHONY: define-bottle-service
define-bottle-service:
	@echo "Recipe Bottle Service Configuration Definition"
	@echo ""
	@echo "== Required Core Variables =="
	@echo "$(RBN_)MONIKER              # Unique identifier for this service instance"
	@echo "$(RBN_)DESCRIPTION          # Human-readable description of the service purpose"
	@echo ""
	@echo "== Image Source Configuration =="
	@echo "$(RBN_)SENTRY_REPO_FULL_NAME  # Full repository path for sentry image"
	@echo "$(RBN_)BOTTLE_REPO_FULL_NAME  # Full repository path for bottle image"
	@echo "$(RBN_)SENTRY_IMAGE_TAG       # Tag for sentry image"
	@echo "$(RBN_)BOTTLE_IMAGE_TAG       # Tag for bottle image"
	@echo ""
	@echo "== Network Configuration =="
	@echo "$(RBN_)GUARDED_NETWORK_ID     # Unique network ID (e.g., 10.240)"
	@echo ""
	@echo "== Port Service Configuration =="
	@echo "$(RBN_)PORT_ENABLED           # Set to 1 to enable port service"
	@echo "When $(RBN_)PORT_ENABLED=1, requires:"
	@echo "  $(RBN_)PORT_HOST            # External port for service access"
	@echo "  $(RBN_)PORT_GUARDED         # Internal port for container service"
	@echo ""
	@echo "== Internet Outreach Configuration =="
	@echo "$(RBN_)OUTREACH_ENABLED       # Set to 1 to enable internet access"
	@echo "When $(RBN_)OUTREACH_ENABLED=1, requires:"
	@echo "  $(RBN_)OUTREACH_CIDR        # CIDR range for allowed outbound traffic"
	@echo "  $(RBN_)OUTREACH_DOMAIN      # Domain name for DNS resolution"
	@echo ""
	@echo "== Volume Mount Configuration =="
	@echo "$(RBN_)VOLUME_MOUNTS          # Podman volume mount arguments"
	@echo "                              # Format: -v host:container:opts [-v ...]"
	@echo ""
	@echo "== Auto-start Configuration =="
	@echo "$(RBN_)AUTOURL_ENABLED        # Set to 1 to enable URL auto-open"
	@echo "When $(RBN_)AUTOURL_ENABLED=1, requires:"
	@echo "  $(RBN_)AUTOURL_URL          # URL to open after service start"

# Validation Target
.PHONY: validate-bottle-service
validate-bottle-service:
	@# Core variables
	@test -n "$($(RBN_)MONIKER)"      || (echo "Error: $(RBN_)MONIKER must be set" && exit 1)
	@test -n "$($(RBN_)DESCRIPTION)"  || (echo "Error: $(RBN_)DESCRIPTION must be set" && exit 1)
	
	@# Image configuration
	@test -n "$($(RBN_)SENTRY_REPO_FULL_NAME)" || (echo "Error: $(RBN_)SENTRY_REPO_FULL_NAME must be set" && exit 1)
	@test -n "$($(RBN_)BOTTLE_REPO_FULL_NAME)" || (echo "Error: $(RBN_)BOTTLE_REPO_FULL_NAME must be set" && exit 1)
	@test -n "$($(RBN_)SENTRY_IMAGE_TAG)"      || (echo "Error: $(RBN_)SENTRY_IMAGE_TAG must be set" && exit 1)
	@test -n "$($(RBN_)BOTTLE_IMAGE_TAG)"      || (echo "Error: $(RBN_)BOTTLE_IMAGE_TAG must be set" && exit 1)
	
	@# Network configuration
	@test -n "$($(RBN_)GUARDED_NETWORK_ID)"    || (echo "Error: $(RBN_)GUARDED_NETWORK_ID must be set" && exit 1)
	
	@# Port service validation
	@if [ "$($(RBN_)PORT_ENABLED)" = "1" ]; then \
		test -n "$($(RBN_)PORT_HOST)"    || (echo "Error: $(RBN_)PORT_HOST required when PORT_ENABLED=1" && exit 1); \
		test -n "$($(RBN_)PORT_GUARDED)" || (echo "Error: $(RBN_)PORT_GUARDED required when PORT_ENABLED=1" && exit 1); \
		test $($(RBN_)PORT_HOST) -gt 0 -a $($(RBN_)PORT_HOST) -lt 65536    || (echo "Error: $(RBN_)PORT_HOST must be between 1-65535" && exit 1); \
		test $($(RBN_)PORT_GUARDED) -gt 0 -a $($(RBN_)PORT_GUARDED) -lt 65536 || (echo "Error: $(RBN_)PORT_GUARDED must be between 1-65535" && exit 1); \
	fi
	
	@# Outreach validation
	@if [ "$($(RBN_)OUTREACH_ENABLED)" = "1" ]; then \
		test -n "$($(RBN_)OUTREACH_CIDR)"   || (echo "Error: $(RBN_)OUTREACH_CIDR required when OUTREACH_ENABLED=1" && exit 1); \
		test -n "$($(RBN_)OUTREACH_DOMAIN)" || (echo "Error: $(RBN_)OUTREACH_DOMAIN required when OUTREACH_ENABLED=1" && exit 1); \
		echo "$($(RBN_)OUTREACH_CIDR)" | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}/[0-9]{1,2}$$' || (echo "Error: $(RBN_)OUTREACH_CIDR must be valid CIDR notation" && exit 1); \
	fi
	
	@# Autourl validation
	@if [ "$($(RBN_)AUTOURL_ENABLED)" = "1" ]; then \
		test -n "$($(RBN_)AUTOURL_URL)" || (echo "Error: $(RBN_)AUTOURL_URL required when AUTOURL_ENABLED=1" && exit 1); \
	fi

# Render Target
.PHONY: render-bottle-service
render-bottle-service:
	@echo "Recipe Bottle Service Configuration: $($(RBN_)MONIKER)"
	@echo "Description: $($(RBN_)DESCRIPTION)"
	@echo ""
	@echo "Image Configuration:"
	@echo "  Sentry Image: $($(RBN_)SENTRY_REPO_FULL_NAME):$($(RBN_)SENTRY_IMAGE_TAG)"
	@echo "  Bottle Image: $($(RBN_)BOTTLE_REPO_FULL_NAME):$($(RBN_)BOTTLE_IMAGE_TAG)"
	@echo ""
	@echo "Network Configuration:"
	@echo "  Guarded Network ID: $($(RBN_)GUARDED_NETWORK_ID)"
	@echo ""
	@if [ "$($(RBN_)PORT_ENABLED)" = "1" ]; then \
		echo "Port Service: ENABLED"; \
		echo "  Host Port: $($(RBN_)PORT_HOST)"; \
		echo "  Container Port: $($(RBN_)PORT_GUARDED)"; \
	else \
		echo "Port Service: DISABLED"; \
	fi
	@echo ""
	@if [ "$($(RBN_)OUTREACH_ENABLED)" = "1" ]; then \
		echo "Internet Outreach: ENABLED"; \
		echo "  Allowed CIDR: $($(RBN_)OUTREACH_CIDR)"; \
		echo "  Allowed Domain: $($(RBN_)OUTREACH_DOMAIN)"; \
	else \
		echo "Internet Outreach: DISABLED"; \
	fi
	@echo ""
	@if [ -n "$($(RBN_)VOLUME_MOUNTS)" ]; then \
		echo "Volume Mounts:"; \
		echo "  $($(RBN_)VOLUME_MOUNTS)" | tr ' ' '\n' | sed 's/^/  /'; \
	else \
		echo "Volume Mounts: None configured"; \
	fi
	@echo ""
	@if [ "$($(RBN_)AUTOURL_ENABLED)" = "1" ]; then \
		echo "Auto-start URL: ENABLED"; \
		echo "  URL: $($(RBN_)AUTOURL_URL)"; \
	else \
		echo "Auto-start URL: DISABLED"; \
	fi