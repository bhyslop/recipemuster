# Recipe Bottle Service Configuration Regime
# This makefile defines the configuration requirements for Recipe Bottle Services

# Regime Prefix: rbs_
# Assignment Prefix: RBN_

#
# Standard Validation Helpers
#
zrbs_check_exported = test "$(1)" != "1" || \
    (env | grep -q ^$(2)= || (echo "Error: '$(2)' must be exported" && exit 1))

zrbs_check_eq = test "$(1)" != "1" || \
    (test "$(2)" = "$(3)" || (echo "Error: '$(4)'" && exit 1))

zrbs_check_bool = test "$(1)" != "1" || \
    (test "$(2)" = "0" -o "$(2)" = "1" || (echo "Error: '$(2)' must be 0 or 1" && exit 1))

zrbs_check_range = test "$(1)" != "1" || \
    (test $(2) -ge $(3) -a $(2) -le $(4) || (echo "Error: '$(2)' must be between '$(3)' and '$(4)'" && exit 1))

zrbs_check_empty = test "$(1)" != "1" || \
    (test -z "$(2)" || (echo "Error: '$(3)'" && exit 1))

zrbs_check_nonempty = test "$(1)" != "1" || \
    (test -n "$(2)" || (echo "Error: '$(2)' must not be empty" && exit 1))

zrbs_check_matches = test "$(1)" != "1" || \
    (echo '$(2)' | grep -E '$(3)' || (echo "Error: $(4)" && exit 1))

zrbs_check_startswith = test "$(1)" != "1" || \
    (echo '$(2)' | grep -E '^$(3)' || (echo "Error: $(4)" && exit 1))

zrbs_check_endswith = test "$(1)" != "1" || \
    (echo '$(2)' | grep -E '$(3)$$' || (echo "Error: $(4)" && exit 1))

#
# Core Service Definition Rule
#
rbs_define:
	@echo "Recipe Bottle Service Configuration Regime"
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
	@echo "== Port Service Configuration =="
	@echo "RBN_PORT_ENABLED           # Enable flag for port service (0 or 1)"
	@echo "When RBN_PORT_ENABLED=1, requires:"
	@echo "  RBN_PORT_UPLINK         # External port on uplink network"
	@echo "  RBN_PORT_ENCLAVE        # Port between sentry and bottle containers"
	@echo "  RBN_PORT_SERVICE        # Port exposed by bottle container"
	@echo ""
	@echo "== Network Uplink Configuration =="
	@echo "RBN_UPLINK_DNS_ENABLED     # Enable protected DNS resolution (0 or 1)"
	@echo "RBN_UPLINK_ACCESS_ENABLED  # Enable protected IP access (0 or 1)"
	@echo "RBN_UPLINK_DNS_GLOBAL      # Enable unrestricted DNS resolution (0 or 1)"
	@echo "RBN_UPLINK_ACCESS_GLOBAL   # Enable unrestricted IP access (0 or 1)"
	@echo "When configured accordingly:"
	@echo "  RBN_UPLINK_ALLOWED_CIDRS   # Space-separated list of allowed CIDR ranges"
	@echo "  RBN_UPLINK_ALLOWED_DOMAINS # Space-separated list of allowed domains"
	@echo ""
	@echo "== Volume Mount Configuration =="
	@echo "RBN_VOLUME_MOUNTS          # Podman volume mount arguments"
	@echo "                           # Format: -v host:container:opts [-v ...]"

#
# Core Validation Target
#
rbs_validate: zrbs_validate_core \
              zrbs_validate_images \
              zrbs_validate_port \
              zrbs_validate_uplink \
              zrbs_validate_volume

#
# Feature Group: Core Configuration
#
zrbs_validate_core: zrbs_validate_moniker zrbs_validate_description

zrbs_validate_moniker:
	@$(call zrbs_check_exported,1,RBN_MONIKER)
	@$(call zrbs_check_nonempty,1,$(RBN_MONIKER))

zrbs_validate_description:
	@$(call zrbs_check_exported,1,RBN_DESCRIPTION)
	@$(call zrbs_check_nonempty,1,$(RBN_DESCRIPTION))

#
# Feature Group: Image Configuration
#
zrbs_validate_images: zrbs_validate_sentry_image zrbs_validate_bottle_image

zrbs_validate_sentry_image:
	@$(call zrbs_check_exported,1,RBN_SENTRY_REPO_FULL_NAME)
	@$(call zrbs_check_nonempty,1,$(RBN_SENTRY_REPO_FULL_NAME))
	@$(call zrbs_check_exported,1,RBN_SENTRY_IMAGE_TAG)
	@$(call zrbs_check_nonempty,1,$(RBN_SENTRY_IMAGE_TAG))

zrbs_validate_bottle_image:
	@$(call zrbs_check_exported,1,RBN_BOTTLE_REPO_FULL_NAME)
	@$(call zrbs_check_nonempty,1,$(RBN_BOTTLE_REPO_FULL_NAME))
	@$(call zrbs_check_exported,1,RBN_BOTTLE_IMAGE_TAG)
	@$(call zrbs_check_nonempty,1,$(RBN_BOTTLE_IMAGE_TAG))

#
# Feature Group: Port Service
#
zrbs_validate_port: zrbs_validate_port_enabled zrbs_validate_port_config

zrbs_validate_port_enabled:
	@$(call zrbs_check_exported,1,RBN_PORT_ENABLED)
	@$(call zrbs_check_bool,1,$(RBN_PORT_ENABLED))

zrbs_validate_port_config:
	@$(call zrbs_check_exported,$(RBN_PORT_ENABLED),RBN_PORT_UPLINK)
	@$(call zrbs_check_exported,$(RBN_PORT_ENABLED),RBN_PORT_ENCLAVE)
	@$(call zrbs_check_exported,$(RBN_PORT_ENABLED),RBN_PORT_SERVICE)
	@$(call zrbs_check_range,$(RBN_PORT_ENABLED),$(RBN_PORT_UPLINK),1,65535)
	@$(call zrbs_check_range,$(RBN_PORT_ENABLED),$(RBN_PORT_ENCLAVE),1,65535)
	@$(call zrbs_check_range,$(RBN_PORT_ENABLED),$(RBN_PORT_SERVICE),1,65535)

#
# Feature Group: Network Uplink
#
zrbs_validate_uplink: zrbs_validate_uplink_basic zrbs_validate_uplink_access zrbs_validate_uplink_dns

zrbs_validate_uplink_basic:
	@$(call zrbs_check_exported,1,RBN_UPLINK_DNS_ENABLED)
	@$(call zrbs_check_bool,1,$(RBN_UPLINK_DNS_ENABLED))
	@$(call zrbs_check_exported,1,RBN_UPLINK_ACCESS_ENABLED)
	@$(call zrbs_check_bool,1,$(RBN_UPLINK_ACCESS_ENABLED))
	@$(call zrbs_check_exported,1,RBN_UPLINK_DNS_GLOBAL)
	@$(call zrbs_check_bool,1,$(RBN_UPLINK_DNS_GLOBAL))
	@$(call zrbs_check_exported,1,RBN_UPLINK_ACCESS_GLOBAL)
	@$(call zrbs_check_bool,1,$(RBN_UPLINK_ACCESS_GLOBAL))

zrbs_validate_uplink_access:
	@test "$(RBN_UPLINK_ACCESS_ENABLED)" != "1" || test "$(RBN_UPLINK_ACCESS_GLOBAL)" = "1" || \
		$(call zrbs_check_exported,1,RBN_UPLINK_ALLOWED_CIDRS)
	@test "$(RBN_UPLINK_ACCESS_ENABLED)" != "1" || test "$(RBN_UPLINK_ACCESS_GLOBAL)" = "1" || \
		$(call zrbs_check_nonempty,1,$(RBN_UPLINK_ALLOWED_CIDRS))
	@test "$(RBN_UPLINK_ACCESS_ENABLED)" != "1" || test "$(RBN_UPLINK_ACCESS_GLOBAL)" = "1" || \
		$(call zrbs_check_matches,1,$(RBN_UPLINK_ALLOWED_CIDRS),'^([0-9]{1,3}\.){3}[0-9]{1,3}/[0-9]{1,2}( ([0-9]{1,3}\.){3}[0-9]{1,3}/[0-9]{1,2})*$$',"RBN_UPLINK_ALLOWED_CIDRS must be space-separated CIDR ranges")

zrbs_validate_uplink_dns:
	@test "$(RBN_UPLINK_DNS_ENABLED)" != "1" || test "$(RBN_UPLINK_DNS_GLOBAL)" = "1" || \
		$(call zrbs_check_exported,1,RBN_UPLINK_ALLOWED_DOMAINS)
	@test "$(RBN_UPLINK_DNS_ENABLED)" != "1" || test "$(RBN_UPLINK_DNS_GLOBAL)" = "1" || \
		$(call zrbs_check_nonempty,1,$(RBN_UPLINK_ALLOWED_DOMAINS))
	@test "$(RBN_UPLINK_DNS_ENABLED)" != "1" || test "$(RBN_UPLINK_DNS_GLOBAL)" = "1" || \
		$(call zrbs_check_matches,1,$(RBN_UPLINK_ALLOWED_DOMAINS),'^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]( [a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9])*$$',"RBN_UPLINK_ALLOWED_DOMAINS must be space-separated domain names")

#
# Feature Group: Volume Mounts
#
zrbs_validate_volume:
	@test ! -n "$(RBN_VOLUME_MOUNTS)" || $(call zrbs_check_exported,1,RBN_VOLUME_MOUNTS)

#
# Render Rules
#
rbs_render: zrbs_render_header \
           zrbs_render_images \
           zrbs_render_port \
           zrbs_render_uplink \
           zrbs_render_volume

zrbs_render_header:
	@echo "Recipe Bottle Service Configuration: $(RBN_MONIKER)"
	@echo "Description: $(RBN_DESCRIPTION)"
	@echo ""

zrbs_render_images:
	@echo "Image Configuration:"
	@echo "  Sentry Image: $(RBN_SENTRY_REPO_FULL_NAME):$(RBN_SENTRY_IMAGE_TAG)"
	@echo "  Bottle Image: $(RBN_BOTTLE_REPO_FULL_NAME):$(RBN_BOTTLE_IMAGE_TAG)"
	@echo ""

zrbs_render_port:
	@echo "Port Service:"
	@test "$(RBN_PORT_ENABLED)" != "1" && echo "  DISABLED" || echo "  ENABLED"
	@test "$(RBN_PORT_ENABLED)" != "1" || echo "  Uplink Port: $(RBN_PORT_UPLINK)"
	@test "$(RBN_PORT_ENABLED)" != "1" || echo "  Enclave Port: $(RBN_PORT_ENCLAVE)"
	@test "$(RBN_PORT_ENABLED)" != "1" || echo "  Service Port: $(RBN_PORT_SERVICE)"
	@echo ""

zrbs_render_uplink:
	@echo "Network Uplink Configuration:"
	@test "$(RBN_UPLINK_DNS_ENABLED)" != "1" && echo "  DNS Resolution: DISABLED" || echo "  DNS Resolution: ENABLED"
	@test "$(RBN_UPLINK_ACCESS_ENABLED)" != "1" && echo "  IP Access: DISABLED" || echo "  IP Access: ENABLED"
	@test "$(RBN_UPLINK_DNS_GLOBAL)" != "1" && echo "  Global DNS: DISABLED" || echo "  Global DNS: ENABLED"
	@test "$(RBN_UPLINK_ACCESS_GLOBAL)" != "1" && echo "  Global Access: DISABLED" || echo "  Global Access: ENABLED"
	@test "$(RBN_UPLINK_ACCESS_ENABLED)" != "1" || test "$(RBN_UPLINK_ACCESS_GLOBAL)" = "1" || \
		echo "  Allowed CIDRs: $(RBN_UPLINK_ALLOWED_CIDRS)"
	@test "$(RBN_UPLINK_DNS_ENABLED)" != "1" || test "$(RBN_UPLINK_DNS_GLOBAL)" = "1" || \
		echo "  Allowed Domains: $(RBN_UPLINK_ALLOWED_DOMAINS)"
	@echo ""

zrbs_render_volume:
	@echo "Volume Configuration:"
	@test ! -n "$(RBN_VOLUME_MOUNTS)" && echo "  None configured" || echo "  Mounts: $(RBN_VOLUME_MOUNTS)"
	@echo ""


