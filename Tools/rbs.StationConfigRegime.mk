# Regime Prefix: rbs_
# Assignment Prefix: RBS_

# Core validation helpers
zrbs_check_exported = test "$(1)" != "1" || \
    (env | grep -q ^'$(2)'= || (echo "Error: Variable '$(2)' must be exported" && exit 1))

zrbs_check__boolean = test "$(1)" != "1" || \
    (test '$(2)' = "0" -o '$(2)' = "1" || (echo "Error: Value '$(2)' must be 0 or 1" && exit 1))

zrbs_check_in_range = \
  test "$(1)" != "1" || (test '$(2)' -ge '$(3)' -a '$(2)' -le '$(4)' || \
  (echo "Error: Value '$(2)' must be between '$(3)' and '$(4)'" && exit 1))

zrbs_check_nonempty = \
  test "$(1)" != "1" || (test -n '$(2)' || \
  (echo "Error: Variable '$(2)' must not be empty" && exit 1))

zrbs_check__matches = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '$(3)' || \
  (echo "Error: $(4)" && exit 1))

zrbs_check_startwth = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '^$(3)' || \
  (echo "Error: $(4)" && exit 1))

zrbs_check_endswith = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '$(3)$$' || \
  (echo "Error: $(4)" && exit 1))

zrbs_check__is_cidr = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$$' || \
  (echo "Error: Value '$(2)' must be in valid CIDR notation" && exit 1))

zrbs_check_isdomain = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '^[a-zA-Z0-9][a-zA-Z0-9\.-]*[a-zA-Z0-9]$$' || \
  (echo "Error: Value '$(2)' must be a valid domain name" && exit 1))

zrbs_check__is_ipv4 = \
  test "$(1)" != "1" || (echo '$(2)' | grep -E '^([0-9]{1,3}\.){3}[0-9]{1,3}$$' || \
  (echo "Error: Value '$(2)' must be a valid IPv4 address" && exit 1))

# Core targets
rbs_define:
	@echo "Station Configuration Regime"
	@echo ""
	@echo "== Registry Authentication =="
	@echo "RBS_REGISTRY_CREDENTIALS  # Optional: Authentication token for container registry access"

# Validation rules for registry credentials
zrbs_validate_registry_auth:
	@test "$(RBS_REGISTRY_CREDENTIALS)" = "" || $(call zrbs_check_exported,1,RBS_REGISTRY_CREDENTIALS)
	@test "$(RBS_REGISTRY_CREDENTIALS)" = "" || $(call zrbs_check_nonempty,1,$(RBS_REGISTRY_CREDENTIALS))

# Main validation target
rbs_validate: zrbs_validate_registry_auth

# Render configuration
rbs_render:
	@echo "Station Configuration Status:"
	@echo ""
	@echo "Registry Authentication:"
	@test "$(RBS_REGISTRY_CREDENTIALS)" = "" || echo "  Registry Credentials: [CREDENTIALS HIDDEN]"
	@test "$(RBS_REGISTRY_CREDENTIALS)" = "" && echo "  Registry Credentials: Not configured (anonymous access)"

# Environment variable rollup
RBS__ROLLUP_ENVIRONMENT_VAR := \
  RBS_REGISTRY_CREDENTIALS='$(RBS_REGISTRY_CREDENTIALS)'
