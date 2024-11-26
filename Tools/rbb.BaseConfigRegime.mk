# Regime Prefix: rbb_
# Assignment Prefix: RBB_

# Core validation helpers
zrbb_check_exported = test "$(1)" != "1" || \
    (env | grep -q ^'$(2)'= || (echo "Error: Variable '$(2)' must be exported" && exit 1))

zrbb_check__boolean = test "$(1)" != "1" || \
    (test '$(2)' = "0" -o '$(2)' = "1" || (echo "Error: Value '$(2)' must be 0 or 1" && exit 1))

zrbb_check_in_range = \
  test "$(1)" != "1" || (test '$(2)' -ge '$(3)' -a '$(2)' -le '$(4)' || \
  (echo "Error: Value '$(2)' must be between '$(3)' and '$(4)'" && exit 1))

zrbb_check_nonempty = \
  test "$(1)" != "1" || (test -n '$(2)' || \
  (echo "Error: Variable '$(2)' must not be empty" && exit 1))

zrbb_check__matches = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '$(3)' || \
  (echo "Error: $(4)" && exit 1))

zrbb_check_startwth = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '^$(3)' || \
  (echo "Error: $(4)" && exit 1))

zrbb_check_endswith = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '$(3)$$' || \
  (echo "Error: $(4)" && exit 1))

zrbb_check__is_cidr = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$$' || \
  (echo "Error: Value '$(2)' must be in valid CIDR notation" && exit 1))

zrbb_check_isdomain = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]$$' || \
  (echo "Error: Value '$(2)' must be a valid domain name" && exit 1))

zrbb_check__is_ipv4 = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}$$' || \
  (echo "Error: Value '$(2)' must be a valid IPv4 address" && exit 1))

# Core targets
rbb_define:
	@echo "Base Configuration Regime"
	@echo ""
	@echo "== Network Configuration =="
	@echo "RBB_ENCLAVE_SUBNET      # CIDR subnet range for Enclave Networks"
	@echo "RBB_ENCLAVE_GATEWAY     # Gateway IP address for Enclave Networks"
	@echo "RBB_DNS_SERVER          # Optional: Upstream DNS resolver for Bottle Services"
	@echo ""
	@echo "== System Configuration =="
	@echo "RBB_NAMEPLATE_PATH      # Optional: Directory containing Nameplate definitions"
	@echo "RBB_REGISTRY_SERVER     # Optional: Container registry server for Published Images"

# Validation rules for network configuration
zrbb_validate_network: zrbb_validate_subnet zrbb_validate_gateway zrbb_validate_dns

zrbb_validate_subnet:
	@$(call zrbb_check_exported,1,RBB_ENCLAVE_SUBNET)
	@$(call zrbb_check_nonempty,1,$(RBB_ENCLAVE_SUBNET))
	@$(call zrbb_check__is_cidr,1,$(RBB_ENCLAVE_SUBNET))

zrbb_validate_gateway:
	@$(call zrbb_check_exported,1,RBB_ENCLAVE_GATEWAY)
	@$(call zrbb_check_nonempty,1,$(RBB_ENCLAVE_GATEWAY))
	@$(call zrbb_check__is_ipv4,1,$(RBB_ENCLAVE_GATEWAY))

zrbb_validate_dns:
	@test "$(RBB_DNS_SERVER)" = "" || $(call zrbb_check_exported,1,RBB_DNS_SERVER)
	@test "$(RBB_DNS_SERVER)" = "" || $(call zrbb_check__is_ipv4,1,$(RBB_DNS_SERVER))

# Validation rules for system configuration
zrbb_validate_system: zrbb_validate_nameplate_path zrbb_validate_registry

zrbb_validate_nameplate_path:
	@test "$(RBB_NAMEPLATE_PATH)" = "" || $(call zrbb_check_exported,1,RBB_NAMEPLATE_PATH)
	@test "$(RBB_NAMEPLATE_PATH)" = "" || $(call zrbb_check_nonempty,1,$(RBB_NAMEPLATE_PATH))

zrbb_validate_registry:
	@test "$(RBB_REGISTRY_SERVER)" = "" || $(call zrbb_check_exported,1,RBB_REGISTRY_SERVER)
	@test "$(RBB_REGISTRY_SERVER)" = "" || $(call zrbb_check_isdomain,1,$(RBB_REGISTRY_SERVER))

# Main validation target
rbb_validate: zrbb_validate_network zrbb_validate_system

# Render configuration
rbb_render:
	@echo "Base Configuration Status:"
	@echo ""
	@echo "Network Configuration:"
	@echo "  Enclave Subnet: $(RBB_ENCLAVE_SUBNET)"
	@echo "  Enclave Gateway: $(RBB_ENCLAVE_GATEWAY)"
	@test "$(RBB_DNS_SERVER)" = "" || echo "  DNS Server: $(RBB_DNS_SERVER)"
	@echo ""
	@echo "System Configuration:"
	@test "$(RBB_NAMEPLATE_PATH)" = "" || echo "  Nameplate Path: $(RBB_NAMEPLATE_PATH)"
	@test "$(RBB_REGISTRY_SERVER)" = "" || echo "  Registry Server: $(RBB_REGISTRY_SERVER)"

# Environment variable rollup
RBB__ROLLUP_ENVIRONMENT_VAR := \
  RBB_ENCLAVE_SUBNET='$(RBB_ENCLAVE_SUBNET)' \
  RBB_ENCLAVE_GATEWAY='$(RBB_ENCLAVE_GATEWAY)' \
  RBB_DNS_SERVER='$(RBB_DNS_SERVER)' \
  RBB_NAMEPLATE_PATH='$(RBB_NAMEPLATE_PATH)' \
  RBB_REGISTRY_SERVER='$(RBB_REGISTRY_SERVER)'
