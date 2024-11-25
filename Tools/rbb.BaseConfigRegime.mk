# Regime Prefix: rbb_
# Assignment Prefix: RBB_

.PHONY: rbb_define rbb_validate rbb_render
.PHONY: zrbb_validate_network zrbb_validate_system zrbb_render_network zrbb_render_system

# Core validation helper functions
zrbb_check_exported = test "$(1)" != "1" || \
    (env | grep -q ^'$(2)'= || (echo "Error: Variable '$(2)' must be exported" && exit 1))

zrbb_check_nonempty = test "$(1)" != "1" || \
    (test -n '$(2)' || (echo "Error: Variable '$(2)' must not be empty" && exit 1))

zrbb_check_ipv4 = test "$(1)" != "1" || \
    (echo '$(2)' | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}$$' || \
        (echo "Error: '$(2)' must be a valid IPv4 address" && exit 1))

zrbb_check_cidr = test "$(1)" != "1" || \
    (echo '$(2)' | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}/[0-9]{1,2}$$' || \
        (echo "Error: '$(2)' must be in CIDR notation (e.g., 172.16.0.0/24)" && exit 1))

zrbb_check_fqdn = test "$(1)" != "1" || \
    (echo '$(2)' | grep -E '^[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]*[a-zA-Z0-9])?)*?(:[0-9]+)?$$' || \
        (echo "Error: '$(2)' must be a valid FQDN with optional port" && exit 1))

# Main targets
rbb_define:
	@echo "Base Configuration Regime"
	@echo ""
	@echo "== Network Configuration =="
	@echo "RBB_ENCLAVE_SUBNET        # CIDR subnet range for Enclave Networks"
	@echo "RBB_ENCLAVE_GATEWAY       # Gateway IP for Enclave Networks"
	@echo "RBB_DNS_SERVER           # Optional: DNS resolver for Bottle Services"
	@echo ""
	@echo "== System Configuration =="
	@echo "RBB_NAMEPLATE_PATH       # Optional: Directory containing Nameplate definitions"
	@echo "RBB_REGISTRY_SERVER      # Optional: Container registry server FQDN"

rbb_validate: zrbb_validate_network zrbb_validate_system

rbb_render: zrbb_render_network zrbb_render_system

# Network validation
zrbb_validate_network:
	@$(call zrbb_check_exported,1,RBB_ENCLAVE_SUBNET)
	@$(call zrbb_check_nonempty,1,$(RBB_ENCLAVE_SUBNET))
	@$(call zrbb_check_cidr,1,$(RBB_ENCLAVE_SUBNET))
	@$(call zrbb_check_exported,1,RBB_ENCLAVE_GATEWAY)
	@$(call zrbb_check_nonempty,1,$(RBB_ENCLAVE_GATEWAY))
	@$(call zrbb_check_ipv4,1,$(RBB_ENCLAVE_GATEWAY))
	@test "$(RBB_DNS_SERVER)" = "" || $(call zrbb_check_exported,1,RBB_DNS_SERVER)
	@test "$(RBB_DNS_SERVER)" = "" || $(call zrbb_check_ipv4,1,$(RBB_DNS_SERVER))

# System validation
zrbb_validate_system:
	@test "$(RBB_NAMEPLATE_PATH)" = "" || $(call zrbb_check_exported,1,RBB_NAMEPLATE_PATH)
	@test "$(RBB_REGISTRY_SERVER)" = "" || $(call zrbb_check_exported,1,RBB_REGISTRY_SERVER)
	@test "$(RBB_REGISTRY_SERVER)" = "" || $(call zrbb_check_fqdn,1,$(RBB_REGISTRY_SERVER))

# Network rendering
zrbb_render_network:
	@echo "Network Configuration:"
	@echo "  Enclave Subnet:     $(RBB_ENCLAVE_SUBNET)"
	@echo "  Enclave Gateway:    $(RBB_ENCLAVE_GATEWAY)"
	@test "$(RBB_DNS_SERVER)" = "" || echo "  DNS Server:        $(RBB_DNS_SERVER)"

# System rendering
zrbb_render_system:
	@echo "System Configuration:"
	@test "$(RBB_NAMEPLATE_PATH)" = "" || echo "  Nameplate Path:    $(RBB_NAMEPLATE_PATH)"
	@test "$(RBB_REGISTRY_SERVER)" = "" || echo "  Registry Server:   $(RBB_REGISTRY_SERVER)"

# Environment rollup for container usage
rbb_ENV_ARGS := \
  -e RBB_ENCLAVE_SUBNET='$(RBB_ENCLAVE_SUBNET)' \
  -e RBB_ENCLAVE_GATEWAY='$(RBB_ENCLAVE_GATEWAY)' \
  -e RBB_DNS_SERVER='$(RBB_DNS_SERVER)' \
  -e RBB_NAMEPLATE_PATH='$(RBB_NAMEPLATE_PATH)' \
  -e RBB_REGISTRY_SERVER='$(RBB_REGISTRY_SERVER)'

