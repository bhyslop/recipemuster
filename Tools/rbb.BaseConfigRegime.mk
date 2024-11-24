# Regime Prefix: rbb_
# Assignment Prefix: RBB_

# Top-level targets
rbb_define: zrbb_define_registry

rbb_validate: zrbb_validate_registry

rbb_render: zrbb_render_registry

# Helper functions
zrbb_check_exported = @test "$(1)" != "1" || \
    (env | grep -q ^'$(2)'= || (echo "Error: Variable '$(2)' must be exported" && exit 1))

zrbb_check_nonempty = @test "$(1)" != "1" || \
    (test -n '$(2)' || (echo "Error: Variable '$(2)' must not be empty" && exit 1))

# Registry Authentication Feature Group
zrbb_define_registry:
	@echo "== Registry Authentication =="
	@echo "RBB_REGISTRY_CREDENTIALS   # Authentication token for container registry access"

zrbb_validate_registry:
	@test -z "$(RBB_REGISTRY_CREDENTIALS)" || $(call zrbb_check_exported,1,RBB_REGISTRY_CREDENTIALS)
	@test -z "$(RBB_REGISTRY_CREDENTIALS)" || $(call zrbb_check_nonempty,1,$(RBB_REGISTRY_CREDENTIALS))

zrbb_render_registry:
	@echo "Registry Authentication:"
	@test -z "$(RBB_REGISTRY_CREDENTIALS)" || echo "  Registry Credentials: [REDACTED]"
	@test -n "$(RBB_REGISTRY_CREDENTIALS)" || echo "  Registry Credentials: Not configured (anonymous access)"