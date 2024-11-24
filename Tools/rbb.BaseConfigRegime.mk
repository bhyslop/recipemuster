# Regime Prefix: rbs_
# Assignment Prefix: RBS_

# Top-level targets
rbs_define: zrbs_define_registry

rbs_validate: zrbs_validate_registry

rbs_render: zrbs_render_registry

# Helper functions
zrbs_check_exported = @test "$(1)" != "1" || \
    (env | grep -q ^'$(2)'= || (echo "Error: Variable '$(2)' must be exported" && exit 1))

zrbs_check_nonempty = @test "$(1)" != "1" || \
    (test -n '$(2)' || (echo "Error: Variable '$(2)' must not be empty" && exit 1))

# Registry Authentication Feature Group
zrbs_define_registry:
	@echo "== Registry Authentication =="
	@echo "RBS_REGISTRY_CREDENTIALS   # Authentication token for container registry access"

zrbs_validate_registry:
	# Validate optional registry credentials if provided
	@test -z "$(RBS_REGISTRY_CREDENTIALS)" || $(call zrbs_check_exported,1,RBS_REGISTRY_CREDENTIALS)
	@test -z "$(RBS_REGISTRY_CREDENTIALS)" || $(call zrbs_check_nonempty,1,$(RBS_REGISTRY_CREDENTIALS))

zrbs_render_registry:
	@echo "Registry Authentication:"
	@test -z "$(RBS_REGISTRY_CREDENTIALS)" || echo "  Registry Credentials: [REDACTED]"
	@test -n "$(RBS_REGISTRY_CREDENTIALS)" || echo "  Registry Credentials: Not configured (anonymous access)"