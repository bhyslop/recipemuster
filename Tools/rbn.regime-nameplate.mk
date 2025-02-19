# Regime Prefix: rbn_
# Assignment Prefix: RBN_

#
# Core Identity Validation
#
rbn_validate_core: \
	zrbn_validate_moniker \
	zrbn_validate_description

zrbn_validate_moniker:
	@$(call MBC_CHECK_EXPORTED,1,RBN_MONIKER)
	@$(call MBC_CHECK_NONEMPTY,1,$(RBN_MONIKER))

zrbn_validate_description:
	@$(call MBC_CHECK_EXPORTED,1,RBN_DESCRIPTION)
	@$(call MBC_CHECK_NONEMPTY,1,$(RBN_DESCRIPTION))

#
# Container Image Validation 
#
rbn_validate_images: \
	zrbn_validate_sentry_image \
	zrbn_validate_bottle_image

zrbn_validate_sentry_image:
	@$(call MBC_CHECK_EXPORTED,1,RBN_SENTRY_REPO_PATH)
	@$(call MBC_CHECK_NONEMPTY,1,$(RBN_SENTRY_REPO_PATH))
	@$(call MBC_CHECK_EXPORTED,1,RBN_SENTRY_IMAGE_TAG)
	@$(call MBC_CHECK_NONEMPTY,1,$(RBN_SENTRY_IMAGE_TAG))

zrbn_validate_bottle_image:
	@$(call MBC_CHECK_EXPORTED,1,RBN_BOTTLE_REPO_PATH)
	@$(call MBC_CHECK_NONEMPTY,1,$(RBN_BOTTLE_REPO_PATH))
	@$(call MBC_CHECK_EXPORTED,1,RBN_BOTTLE_IMAGE_TAG)
	@$(call MBC_CHECK_NONEMPTY,1,$(RBN_BOTTLE_IMAGE_TAG))

#
# Port Service Validation
#
rbn_validate_ports: \
	zrbn_validate_port_enabled \
	zrbn_validate_port_config \
	zrbn_validate_port_separation \
	zrbn_validate_uplink_ports

zrbn_validate_port_enabled:
	@$(call MBC_CHECK_EXPORTED,1,RBN_PORT_ENABLED)
	@$(call MBC_CHECK__BOOLEAN,1,$(RBN_PORT_ENABLED))

zrbn_validate_port_config:
	@$(call MBC_CHECK_EXPORTED,$(RBN_PORT_ENABLED),RBN_ENTRY_PORT_WORKSTATION)
	@$(call MBC_CHECK_IN_RANGE,$(RBN_PORT_ENABLED),$(RBN_ENTRY_PORT_WORKSTATION),1,65535)
	@$(call MBC_CHECK_EXPORTED,$(RBN_PORT_ENABLED),RBN_ENTRY_PORT_ENCLAVE)
	@$(call MBC_CHECK_IN_RANGE,$(RBN_PORT_ENABLED),$(RBN_ENTRY_PORT_ENCLAVE),1,65535)

zrbn_validate_port_separation:
	@test "$(RBN_PORT_ENABLED)" = "0" || \
	  test "$(RBN_UPLINK_PORT_MIN)" -gt "$(RBN_ENTRY_PORT_WORKSTATION)" || \
	  (echo "Error: RBN_UPLINK_PORT_MIN must be greater than RBN_ENTRY_PORT_WORKSTATION" && exit 1)
	@test "$(RBN_PORT_ENABLED)" = "0" || \
	  test "$(RBN_UPLINK_PORT_MIN)" -gt "$(RBN_ENTRY_PORT_ENCLAVE)" || \
	  (echo "Error: RBN_UPLINK_PORT_MIN must be greater than RBN_ENTRY_PORT_ENCLAVE" && exit 1)


zrbn_validate_uplink_ports:
	@$(call MBC_CHECK_EXPORTED,1,RBN_UPLINK_PORT_MIN)
	@$(call MBC_CHECK_IN_RANGE,1,$(RBN_UPLINK_PORT_MIN),1024,65535)

#
# Network Address Validation
#
rbn_validate_network_address: \
	zrbn_validate_network_base \
	zrbn_validate_network_mask \
	zrbn_validate_network_ips

zrbn_validate_network_base:
	@$(call MBC_CHECK_EXPORTED,1,RBN_ENCLAVE_BASE_IP)
	@$(call MBC_CHECK__IS_IPV4,1,$(RBN_ENCLAVE_BASE_IP))

zrbn_validate_network_mask:
	@$(call MBC_CHECK_EXPORTED,1,RBN_ENCLAVE_NETMASK)
	@$(call MBC_CHECK_IN_RANGE,1,$(RBN_ENCLAVE_NETMASK),8,30)

zrbn_validate_network_ips:
	@$(call MBC_CHECK_EXPORTED,1,RBN_ENCLAVE_BOTTLE_IP)
	@$(call MBC_CHECK__IS_IPV4,1,$(RBN_ENCLAVE_BOTTLE_IP))
	@$(call MBC_CHECK_EXPORTED,1,RBN_ENCLAVE_SENTRY_IP)
	@$(call MBC_CHECK__IS_IPV4,1,$(RBN_ENCLAVE_SENTRY_IP))

#
# Network Uplink Validation
#
rbn_validate_uplink: \
	zrbn_validate_uplink_flags \
	zrbn_validate_uplink_cidrs \
	zrbn_validate_uplink_domains

zrbn_validate_uplink_flags:
	@$(call MBC_CHECK_EXPORTED,1,RBN_UPLINK_DNS_ENABLED)
	@$(call MBC_CHECK__BOOLEAN,1,$(RBN_UPLINK_DNS_ENABLED))
	@$(call MBC_CHECK_EXPORTED,1,RBN_UPLINK_ACCESS_ENABLED)
	@$(call MBC_CHECK__BOOLEAN,1,$(RBN_UPLINK_ACCESS_ENABLED))
	@$(call MBC_CHECK_EXPORTED,1,RBN_UPLINK_DNS_GLOBAL)
	@$(call MBC_CHECK__BOOLEAN,1,$(RBN_UPLINK_DNS_GLOBAL))
	@$(call MBC_CHECK_EXPORTED,1,RBN_UPLINK_ACCESS_GLOBAL)
	@$(call MBC_CHECK__BOOLEAN,1,$(RBN_UPLINK_ACCESS_GLOBAL))

zrbn_validate_uplink_cidrs:
	@test "$(RBN_UPLINK_ACCESS_ENABLED)" = "0" || \
	 test "$(RBN_UPLINK_ACCESS_GLOBAL)" = "1" || \
	 $(call MBC_CHECK_EXPORTED,1,RBN_UPLINK_ALLOWED_CIDRS)
	@test "$(RBN_UPLINK_ACCESS_ENABLED)" = "0" || \
	 test "$(RBN_UPLINK_ACCESS_GLOBAL)" = "1" || \
	 for cidr in $(RBN_UPLINK_ALLOWED_CIDRS); do \
	   $(call MBC_CHECK__IS_CIDR,1,$$cidr); \
	 done

zrbn_validate_uplink_domains:
	@test "$(RBN_UPLINK_DNS_ENABLED)" = "0" || \
	 test "$(RBN_UPLINK_DNS_GLOBAL)" = "1" || \
	 $(call MBC_CHECK_EXPORTED,1,RBN_UPLINK_ALLOWED_DOMAINS)
	@test "$(RBN_UPLINK_DNS_ENABLED)" = "0" || \
	 test "$(RBN_UPLINK_DNS_GLOBAL)" = "1" || \
	 for domain in $(RBN_UPLINK_ALLOWED_DOMAINS); do \
	   $(call MBC_CHECK_ISDOMAIN,1,$$domain); \
	 done

#
# Volume Mount Validation
#
rbn_validate_volumes: zrbn_validate_volume_mounts

zrbn_validate_volume_mounts:
	@test "$(RBN_VOLUME_MOUNTS)" = "" || $(call MBC_CHECK_EXPORTED,1,RBN_VOLUME_MOUNTS)

#
# Core Required Targets
#
rbn_validate: \
	rbn_validate_core \
	rbn_validate_images \
	rbn_validate_ports \
	rbn_validate_network_address \
	rbn_validate_uplink \
	rbn_validate_volumes

rbn_define:
	@echo "Recipe Bottle Nameplate Configuration"
	@echo
	@echo "== Core Identity =="
	@echo "RBN_MONIKER              # Unique service instance identifier"
	@echo "RBN_DESCRIPTION          # Human-readable service description"
	@echo
	@echo "== Container Images =="
	@echo "RBN_SENTRY_REPO_PATH     # Full repository path for sentry image"
	@echo "RBN_BOTTLE_REPO_PATH     # Full repository path for bottle image"
	@echo "RBN_SENTRY_IMAGE_TAG     # Version tag for sentry image"
	@echo "RBN_BOTTLE_IMAGE_TAG     # Version tag for bottle image"
	@echo
	@echo "== Port Service =="
	@echo "RBN_PORT_ENABLED              # Enable port service functionality (0 or 1)"
	@echo "When RBN_PORT_ENABLED=1, requires:"
	@echo "  RBN_ENTRY_PORT_WORKSTATION  # External port on transit network (1-65535)"
	@echo "  RBN_ENTRY_PORT_ENCLAVE      # port between containers (1-65535)"
	@echo
	@echo "== Network Address =="
	@echo "RBN_ENCLAVE_BASE_IP     # Base IPv4 address for enclave network"
	@echo "RBN_ENCLAVE_NETMASK     # Network mask width (8-30)"
	@echo "RBN_ENCLAVE_BOTTLE_IP   # Gateway IP for container startup"
	@echo "RBN_ENCLAVE_SENTRY_IP   # IP address for Sentry Container"
	@echo
	@echo "== Uplink Port Range =="
	@echo "RBN_UPLINK_PORT_MIN        # Minimum port number for uplink connections (must be > service ports)"
	@echo
	@echo "== Network Uplink =="
	@echo "RBN_UPLINK_DNS_ENABLED   # Enable protected DNS resolution (0 or 1)"
	@echo "RBN_UPLINK_ACCESS_ENABLED # Enable protected IP access (0 or 1)"
	@echo "RBN_UPLINK_DNS_GLOBAL    # Enable unrestricted DNS resolution (0 or 1)"
	@echo "RBN_UPLINK_ACCESS_GLOBAL # Enable unrestricted IP access (0 or 1)"
	@echo "When UPLINK_ACCESS_ENABLED=1 and UPLINK_ACCESS_GLOBAL=0, requires:"
	@echo "  RBN_UPLINK_ALLOWED_CIDRS   # Space-separated list of allowed CIDR ranges"
	@echo "When UPLINK_DNS_ENABLED=1 and UPLINK_DNS_GLOBAL=0, requires:"
	@echo "  RBN_UPLINK_ALLOWED_DOMAINS # Space-separated list of allowed domains"
	@echo
	@echo "== Volume Mounts =="
	@echo "RBN_VOLUME_MOUNTS        # Optional volume mount arguments"

rbn_render:
	$(MBC_STEP) "Recipe Bottle Nameplate Configuration:"
	@echo "Core Identity:"
	@echo "  Moniker: $(RBN_MONIKER)"
	@echo "  Description: $(RBN_DESCRIPTION)"
	@echo "Container Images:"
	@echo "  Sentry: $(RBN_SENTRY_REPO_PATH):$(RBN_SENTRY_IMAGE_TAG)"
	@echo "  Bottle: $(RBN_BOTTLE_REPO_PATH):$(RBN_BOTTLE_IMAGE_TAG)"
	@echo "Port Service: $(if $(filter 1,$(RBN_PORT_ENABLED)),ENABLED,DISABLED)"
	@test "$(RBN_PORT_ENABLED)" != "1" || echo "  Workstation Port: $(RBN_ENTRY_PORT_WORKSTATION)"
	@test "$(RBN_PORT_ENABLED)" != "1" || echo "  Enclave Port: $(RBN_ENTRY_PORT_ENCLAVE)"
	@echo "Network Address:"
	@echo "  Network Base: $(RBN_ENCLAVE_BASE_IP)"
	@echo "  Network Mask: $(RBN_ENCLAVE_NETMASK)"
	@echo "  Bottle IP: $(RBN_ENCLAVE_BOTTLE_IP)"
	@echo "  Sentry IP: $(RBN_ENCLAVE_SENTRY_IP)"
	@echo "Network Uplink:"
	@echo "  DNS Resolution: $(if $(filter 1,$(RBN_UPLINK_DNS_ENABLED)),ENABLED,DISABLED)"
	@echo "  IP Access: $(if $(filter 1,$(RBN_UPLINK_ACCESS_ENABLED)),ENABLED,DISABLED)"
	@echo "  Global DNS: $(if $(filter 1,$(RBN_UPLINK_DNS_GLOBAL)),ENABLED,DISABLED)"
	@echo "  Global Access: $(if $(filter 1,$(RBN_UPLINK_ACCESS_GLOBAL)),ENABLED,DISABLED)"
	@test "$(RBN_UPLINK_ACCESS_ENABLED)" != "1" || \
	 test "$(RBN_UPLINK_ACCESS_GLOBAL)" = "1" || \
	 echo "  Allowed CIDRs: $(RBN_UPLINK_ALLOWED_CIDRS)"
	@test "$(RBN_UPLINK_DNS_ENABLED)" != "1" || \
	 test "$(RBN_UPLINK_DNS_GLOBAL)" = "1" || \
	 echo "  Allowed Domains: $(RBN_UPLINK_ALLOWED_DOMAINS)"
	@test "$(RBN_VOLUME_MOUNTS)" = "" || echo "Volume Mounts: $(RBN_VOLUME_MOUNTS)"

# Container environment arguments for Assignment Variables
RBN__ROLLUP_ENVIRONMENT_VAR := \
  RBN_MONIKER='$(RBN_MONIKER)' \
  RBN_DESCRIPTION='$(RBN_DESCRIPTION)' \
  RBN_SENTRY_REPO_PATH='$(RBN_SENTRY_REPO_PATH)' \
  RBN_BOTTLE_REPO_PATH='$(RBN_BOTTLE_REPO_PATH)' \
  RBN_SENTRY_IMAGE_TAG='$(RBN_SENTRY_IMAGE_TAG)' \
  RBN_BOTTLE_IMAGE_TAG='$(RBN_BOTTLE_IMAGE_TAG)' \
  RBN_PORT_ENABLED='$(RBN_PORT_ENABLED)' \
  RBN_ENTRY_PORT_WORKSTATION='$(RBN_ENTRY_PORT_WORKSTATION)' \
  RBN_ENTRY_PORT_ENCLAVE='$(RBN_ENTRY_PORT_ENCLAVE)' \
  RBN_ENCLAVE_BASE_IP='$(RBN_ENCLAVE_BASE_IP)' \
  RBN_ENCLAVE_NETMASK='$(RBN_ENCLAVE_NETMASK)' \
  RBN_ENCLAVE_BOTTLE_IP='$(RBN_ENCLAVE_BOTTLE_IP)' \
  RBN_ENCLAVE_SENTRY_IP='$(RBN_ENCLAVE_SENTRY_IP)' \
  RBN_UPLINK_PORT_MIN='$(RBN_UPLINK_PORT_MIN)' \
  RBN_UPLINK_DNS_ENABLED='$(RBN_UPLINK_DNS_ENABLED)' \
  RBN_UPLINK_ACCESS_ENABLED='$(RBN_UPLINK_ACCESS_ENABLED)' \
  RBN_UPLINK_DNS_GLOBAL='$(RBN_UPLINK_DNS_GLOBAL)' \
  RBN_UPLINK_ACCESS_GLOBAL='$(RBN_UPLINK_ACCESS_GLOBAL)' \
  RBN_UPLINK_ALLOWED_CIDRS='$(RBN_UPLINK_ALLOWED_CIDRS)' \
  RBN_UPLINK_ALLOWED_DOMAINS='$(RBN_UPLINK_ALLOWED_DOMAINS)' \
  RBN_VOLUME_MOUNTS='$(RBN_VOLUME_MOUNTS)'

