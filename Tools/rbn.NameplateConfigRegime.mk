# Recipe Bottle Service Configuration Regime
# This makefile defines the configuration requirements for Recipe Bottle Services

# Regime Prefix: rbn_
# Assignment Prefix: RBN_

#
# Standard Validation Helpers
#
zrbn_check_exported = test "$(1)" != "1" || \
    (env | grep -q ^'$(2)'= || (echo "Error: Variable '$(2)' must be exported" && exit 1))

zrbn_check__boolean = test "$(1)" != "1" || \
    (test '$(2)' = "0" -o '$(2)' = "1" || (echo "Error: Value '$(2)' must be 0 or 1" && exit 1))

zrbn_check_in_range = \
  test "$(1)" != "1" || (test '$(2)' -ge '$(3)' -a '$(2)' -le '$(4)' || \
  (echo "Error: Value '$(2)' must be between '$(3)' and '$(4)'" && exit 1))

zrbn_check_nonempty = \
  test "$(1)" != "1" || (test -n '$(2)' || \
  (echo "Error: Variable '$(2)' must not be empty" && exit 1))

zrbn_check__matches = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '$(3)' || \
  (echo "Error: $(4)" && exit 1))

zrbn_check_startwth = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '^$(3)' || \
  (echo "Error: $(4)" && exit 1))

zrbn_check_endswith = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '$(3)$$' || \
  (echo "Error: $(4)" && exit 1))

zrbn_check__is_cidr = \
  test "$(1)" != "1" || (echo $(2) | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$$' || \
  (echo "Error: Value '$(2)' is not in valid CIDR notation" && exit 1))

zrbn_check_isdomain = \
  test "$(1)" != "1" || (echo $(2) | grep -E '^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]$$' || \
  (echo "Error: Value '$(2)' must be a valid domain name" && exit 1))

#
# Core Service Definition Rule
#
rbn_define:
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
rbn_validate: zrbn_validate_core \
              zrbn_validate_images \
              zrbn_validate_port \
              zrbn_validate_uplink \
              zrbn_validate_volume

#
# Feature Group: Core Configuration
#
zrbn_validate_core: zrbn_validate_moniker zrbn_validate_description

zrbn_validate_moniker:
	@$(call zrbn_check_exported,1,RBN_MONIKER)
	@$(call zrbn_check_nonempty,1,$(RBN_MONIKER))

zrbn_validate_description:
	@$(call zrbn_check_exported,1,RBN_DESCRIPTION)
	@$(call zrbn_check_nonempty,1,$(RBN_DESCRIPTION))

#
# Feature Group: Image Configuration
#
zrbn_validate_images: zrbn_validate_sentry_image zrbn_validate_bottle_image

zrbn_validate_sentry_image:
	@$(call zrbn_check_exported,1,RBN_SENTRY_REPO_FULL_NAME)
	@$(call zrbn_check_nonempty,1,$(RBN_SENTRY_REPO_FULL_NAME))
	@$(call zrbn_check_exported,1,RBN_SENTRY_IMAGE_TAG)
	@$(call zrbn_check_nonempty,1,$(RBN_SENTRY_IMAGE_TAG))

zrbn_validate_bottle_image:
	@$(call zrbn_check_exported,1,RBN_BOTTLE_REPO_FULL_NAME)
	@$(call zrbn_check_nonempty,1,$(RBN_BOTTLE_REPO_FULL_NAME))
	@$(call zrbn_check_exported,1,RBN_BOTTLE_IMAGE_TAG)
	@$(call zrbn_check_nonempty,1,$(RBN_BOTTLE_IMAGE_TAG))

#
# Feature Group: Port Service
#
zrbn_validate_port: zrbn_validate_port_enabled zrbn_validate_port_config

zrbn_validate_port_enabled:
	@$(call zrbn_check_exported,1,RBN_PORT_ENABLED)
	@$(call zrbn_check__boolean,1,$(RBN_PORT_ENABLED))

zrbn_validate_port_config:
	@$(call zrbn_check_exported,$(RBN_PORT_ENABLED),RBN_PORT_UPLINK)
	@$(call zrbn_check_exported,$(RBN_PORT_ENABLED),RBN_PORT_ENCLAVE)
	@$(call zrbn_check_exported,$(RBN_PORT_ENABLED),RBN_PORT_SERVICE)
	@$(call zrbn_check_in_range,$(RBN_PORT_ENABLED),$(RBN_PORT_UPLINK),1,65535)
	@$(call zrbn_check_in_range,$(RBN_PORT_ENABLED),$(RBN_PORT_ENCLAVE),1,65535)
	@$(call zrbn_check_in_range,$(RBN_PORT_ENABLED),$(RBN_PORT_SERVICE),1,65535)

#
# Feature Group: Network Uplink
#
zrbn_validate_uplink: zrbn_validate_uplink_basic zrbn_validate_uplink_access zrbn_validate_uplink_dns

zrbn_validate_uplink_basic:
	@$(call zrbn_check_exported,1,RBN_UPLINK_DNS_ENABLED)
	@$(call zrbn_check__boolean,1,$(RBN_UPLINK_DNS_ENABLED))
	@$(call zrbn_check_exported,1,RBN_UPLINK_ACCESS_ENABLED)
	@$(call zrbn_check__boolean,1,$(RBN_UPLINK_ACCESS_ENABLED))
	@$(call zrbn_check_exported,1,RBN_UPLINK_DNS_GLOBAL)
	@$(call zrbn_check__boolean,1,$(RBN_UPLINK_DNS_GLOBAL))
	@$(call zrbn_check_exported,1,RBN_UPLINK_ACCESS_GLOBAL)
	@$(call zrbn_check__boolean,1,$(RBN_UPLINK_ACCESS_GLOBAL))

zrbn_validate_uplink_access:
	@test "$(RBN_UPLINK_ACCESS_ENABLED)" != "1" || test "$(RBN_UPLINK_ACCESS_GLOBAL)" = "1" || \
		$(call zrbn_check_exported,1,RBN_UPLINK_ALLOWED_CIDRS)
	@test "$(RBN_UPLINK_ACCESS_ENABLED)" != "1" || test "$(RBN_UPLINK_ACCESS_GLOBAL)" = "1" || \
		$(call zrbn_check_nonempty,1,$(RBN_UPLINK_ALLOWED_CIDRS))
	@if [ "$(RBN_UPLINK_ACCESS_ENABLED)" = "1" ] && [ "$(RBN_UPLINK_ACCESS_GLOBAL)" = "0" ]; then \
		for cidr in $(RBN_UPLINK_ALLOWED_CIDRS); do \
			$(call zrbn_check__is_cidr,1,$$cidr); \
		done \
	fi

zrbn_validate_uplink_dns:
	@test "$(RBN_UPLINK_DNS_ENABLED)" != "1" || test "$(RBN_UPLINK_DNS_GLOBAL)" = "1" || \
		$(call zrbn_check_exported,1,RBN_UPLINK_ALLOWED_DOMAINS)
	@test "$(RBN_UPLINK_DNS_ENABLED)" != "1" || test "$(RBN_UPLINK_DNS_GLOBAL)" = "1" || \
		$(call zrbn_check_nonempty,1,$(RBN_UPLINK_ALLOWED_DOMAINS))
	@if [ "$(RBN_UPLINK_DNS_ENABLED)" = "1" ] && [ "$(RBN_UPLINK_DNS_GLOBAL)" = "0" ]; then \
		for domain in $(RBN_UPLINK_ALLOWED_DOMAINS); do \
			$(call zrbn_check_isdomain,1,$$domain); \
		done \
	fi

#
# Feature Group: Volume Mounts
#
zrbn_validate_volume:
	@$(call zrbn_check_exported,1,RBN_VOLUME_MOUNTS)

#
# Render Rules
#
rbn_render: zrbn_render_header \
           zrbn_render_images \
           zrbn_render_port \
           zrbn_render_uplink \
           zrbn_render_volume

zrbn_render_header:
	@echo "Recipe Bottle Service Configuration: $(RBN_MONIKER)"
	@echo "Description: $(RBN_DESCRIPTION)"
	@echo ""

zrbn_render_images:
	@echo "Image Configuration:"
	@echo "  Sentry Image: $(RBN_SENTRY_REPO_FULL_NAME):$(RBN_SENTRY_IMAGE_TAG)"
	@echo "  Bottle Image: $(RBN_BOTTLE_REPO_FULL_NAME):$(RBN_BOTTLE_IMAGE_TAG)"
	@echo ""

zrbn_render_port:
	@echo "Port Service:"
	@test "$(RBN_PORT_ENABLED)" != "1" && echo "  DISABLED" || echo "  ENABLED"
	@test "$(RBN_PORT_ENABLED)" != "1" || echo "  Uplink Port: $(RBN_PORT_UPLINK)"
	@test "$(RBN_PORT_ENABLED)" != "1" || echo "  Enclave Port: $(RBN_PORT_ENCLAVE)"
	@test "$(RBN_PORT_ENABLED)" != "1" || echo "  Service Port: $(RBN_PORT_SERVICE)"
	@echo ""

zrbn_render_uplink:
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

zrbn_render_volume:
	@echo "Volume Configuration:"
	@echo "  Mounts: $(RBN_VOLUME_MOUNTS)"
	@echo ""

#
# Environment variable rollup for container usage
#
RBN__ROLLUP_ENVIRONMENT_VAR := \
  RBN_MONIKER='$(RBN_MONIKER)' \
  RBN_DESCRIPTION='$(RBN_DESCRIPTION)' \
  RBN_SENTRY_REPO_FULL_NAME='$(RBN_SENTRY_REPO_FULL_NAME)' \
  RBN_BOTTLE_REPO_FULL_NAME='$(RBN_BOTTLE_REPO_FULL_NAME)' \
  RBN_SENTRY_IMAGE_TAG='$(RBN_SENTRY_IMAGE_TAG)' \
  RBN_BOTTLE_IMAGE_TAG='$(RBN_BOTTLE_IMAGE_TAG)' \
  RBN_PORT_ENABLED='$(RBN_PORT_ENABLED)' \
  RBN_PORT_UPLINK='$(RBN_PORT_UPLINK)' \
  RBN_PORT_ENCLAVE='$(RBN_PORT_ENCLAVE)' \
  RBN_PORT_SERVICE='$(RBN_PORT_SERVICE)' \
  RBN_UPLINK_DNS_ENABLED='$(RBN_UPLINK_DNS_ENABLED)' \
  RBN_UPLINK_ACCESS_ENABLED='$(RBN_UPLINK_ACCESS_ENABLED)' \
  RBN_UPLINK_DNS_GLOBAL='$(RBN_UPLINK_DNS_GLOBAL)' \
  RBN_UPLINK_ACCESS_GLOBAL='$(RBN_UPLINK_ACCESS_GLOBAL)' \
  RBN_UPLINK_ALLOWED_CIDRS='$(RBN_UPLINK_ALLOWED_CIDRS)' \
  RBN_UPLINK_ALLOWED_DOMAINS='$(RBN_UPLINK_ALLOWED_DOMAINS)' \
  RBN_VOLUME_MOUNTS='$(RBN_VOLUME_MOUNTS)'

