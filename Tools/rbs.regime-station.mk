# Regime Prefix: rbs_
# Assignment Prefix: RBS_

# Core validation helpers using MBC library 
# Note: Assumes MBC_* functions are available from parent makefile

# Core targets
rbs_define:
	@echo "Station Configuration Regime"
	@echo ""
	@echo "== Registry Authentication =="
	@echo "RBS_REGISTRY_CREDENTIALS   # Optional: Authentication token for container registry access"

# Validation rules for registry credentials
zrbs_validate_registry_auth:
	@test "$(RBS_REGISTRY_CREDENTIALS)" = "" || $(call MBC_CHECK_EXPORTED,1,RBS_REGISTRY_CREDENTIALS)
	@test "$(RBS_REGISTRY_CREDENTIALS)" = "" || $(call MBC_CHECK_NONEMPTY,1,$(RBS_REGISTRY_CREDENTIALS))

# Main validation target aggregates all feature validations
rbs_validate: zrbs_validate_registry_auth

# Render configuration status with consistent formatting
rbs_render:
	$(MBC_START) "Station Configuration Status:"
	@echo ""
	@echo "Registry Authentication"
	@test "$(RBS_REGISTRY_CREDENTIALS)" = "" || echo "  Registry Credentials: [CREDENTIALS ARE HIDDEN]"
	@test "$(RBS_REGISTRY_CREDENTIALS)" = "" && echo "  Registry Credentials: Not configured (anonymous access)"

# Environment variable rollup for container usage
RBS__ROLLUP_ENVIRONMENT_VAR := \
  RBS_REGISTRY_CREDENTIALS='$(RBS_REGISTRY_CREDENTIALS)'

