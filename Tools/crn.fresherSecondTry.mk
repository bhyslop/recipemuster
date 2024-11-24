# Regime Prefix: rbn_
# Assignment Prefix: RBN_

.PHONY: rbn_define rbn_validate rbn_render
.PHONY: zrbn_validate_core zrbn_validate_images zrbn_validate_port zrbn_validate_uplink zrbn_validate_volumes
.PHONY: zrbn_render_core zrbn_render_images zrbn_render_port zrbn_render_uplink zrbn_render_volumes

# Core validation helpers
zrbn_check_exported = @test "$(1)" = "1" || test "$(2)" != "1" || \
    (env | grep -q ^$(3)= || (echo "Error: $(3) must be exported" && exit 1))

zrbn_check_bool = @test "$(1)" = "1" || test "$(2)" != "1" || \
    (test "$(3)" = "0" -o "$(3)" = "1" || (echo "Error: $(3) must be 0 or 1" && exit 1))

zrbn_check_range = @test "$(1)" = "1" || test "$(2)" != "1" || \
    (test "$(3)" -ge "$(4)" -a "$(3)" -le "$(5)" || (echo "Error: $(3) must be between $(4) and $(5)" && exit 1))

zrbn_check_nonempty = @test "$(1)" = "1" || test "$(2)" != "1" || \
    (test -n "$(3)" || (echo "Error: $(3) must not be empty" && exit 1))

zrbn_check_matches = @test "$(1)" = "1" || test "$(2)" != "1" || \
    (echo '$(3)' | grep -E '$(4)' || (echo "Error: $(5)" && exit 1))

# Main interface targets
rbn_define:
    @echo "RBN_MONIKER"
    @echo "RBN_DESCRIPTION"
    @echo "RBN_SENTRY_REPO_FULL_NAME"
    @echo "RBN_BOTTLE_REPO_FULL_NAME"
    @echo "RBN_SENTRY_IMAGE_TAG"
    @echo "RBN_BOTTLE_IMAGE_TAG"
    @echo "RBN_PORT_ENABLED"
    @echo "RBN_PORT_UPLINK"
    @echo "RBN_PORT_ENCLAVE"
    @echo "RBN_PORT_SERVICE"
    @echo "RBN_UPLINK_DNS_ENABLED"
    @echo "RBN_UPLINK_ACCESS_ENABLED"
    @echo "RBN_UPLINK_DNS_GLOBAL"
    @echo "RBN_UPLINK_ACCESS_GLOBAL"
    @echo "RBN_UPLINK_ALLOWED_CIDRS"
    @echo "RBN_UPLINK_ALLOWED_DOMAINS"
    @echo "RBN_VOLUME_MOUNTS"

rbn_validate: zrbn_validate_core zrbn_validate_images zrbn_validate_port zrbn_validate_uplink zrbn_validate_volumes

rbn_render: zrbn_render_core zrbn_render_images zrbn_render_port zrbn_render_uplink zrbn_render_volumes

# Core Service Identity validation and rendering
zrbn_validate_core:
    @$(call zrbn_check_exported,1,1,RBN_MONIKER)
    @$(call zrbn_check_nonempty,1,1,$(RBN_MONIKER))
    @$(call zrbn_check_exported,1,1,RBN_DESCRIPTION)
    @$(call zrbn_check_nonempty,1,1,$(RBN_DESCRIPTION))

zrbn_render_core:
    @echo "Core Service Identity:"
    @echo "  Moniker: $(RBN_MONIKER)"
    @echo "  Description: $(RBN_DESCRIPTION)"

# Container Image validation and rendering
zrbn_validate_images:
    @$(call zrbn_check_exported,1,1,RBN_SENTRY_REPO_FULL_NAME)
    @$(call zrbn_check_nonempty,1,1,$(RBN_SENTRY_REPO_FULL_NAME))
    @$(call zrbn_check_exported,1,1,RBN_BOTTLE_REPO_FULL_NAME)
    @$(call zrbn_check_nonempty,1,1,$(RBN_BOTTLE_REPO_FULL_NAME))
    @$(call zrbn_check_exported,1,1,RBN_SENTRY_IMAGE_TAG)
    @$(call zrbn_check_nonempty,1,1,$(RBN_SENTRY_IMAGE_TAG))
    @$(call zrbn_check_exported,1,1,RBN_BOTTLE_IMAGE_TAG)
    @$(call zrbn_check_nonempty,1,1,$(RBN_BOTTLE_IMAGE_TAG))

zrbn_render_images:
    @echo "Container Images:"
    @echo "  Sentry Image: $(RBN_SENTRY_REPO_FULL_NAME):$(RBN_SENTRY_IMAGE_TAG)"
    @echo "  Bottle Image: $(RBN_BOTTLE_REPO_FULL_NAME):$(RBN_BOTTLE_IMAGE_TAG)"

# Port Service validation and rendering
zrbn_validate_port:
    @$(call zrbn_check_exported,1,1,RBN_PORT_ENABLED)
    @$(call zrbn_check_bool,1,1,$(RBN_PORT_ENABLED))
    @$(call zrbn_check_exported,$(RBN_PORT_ENABLED),1,RBN_PORT_UPLINK)
    @$(call zrbn_check_range,$(RBN_PORT_ENABLED),1,$(RBN_PORT_UPLINK),1,65535)
    @$(call zrbn_check_exported,$(RBN_PORT_ENABLED),1,RBN_PORT_ENCLAVE)
    @$(call zrbn_check_range,$(RBN_PORT_ENABLED),1,$(RBN_PORT_ENCLAVE),1,65535)
    @$(call zrbn_check_exported,$(RBN_PORT_ENABLED),1,RBN_PORT_SERVICE)
    @$(call zrbn_check_range,$(RBN_PORT_ENABLED),1,$(RBN_PORT_SERVICE),1,65535)

zrbn_render_port:
    @echo "Port Service: $(if $(filter 1,$(RBN_PORT_ENABLED)),ENABLED,DISABLED)"
    @test "$(RBN_PORT_ENABLED)" != "1" || echo "  Uplink Port: $(RBN_PORT_UPLINK)"
    @test "$(RBN_PORT_ENABLED)" != "1" || echo "  Enclave Port: $(RBN_PORT_ENCLAVE)"
    @test "$(RBN_PORT_ENABLED)" != "1" || echo "  Service Port: $(RBN_PORT_SERVICE)"

# Network Uplink validation and rendering
zrbn_validate_uplink:
    @$(call zrbn_check_exported,1,1,RBN_UPLINK_DNS_ENABLED)
    @$(call zrbn_check_bool,1,1,$(RBN_UPLINK_DNS_ENABLED))
    @$(call zrbn_check_exported,1,1,RBN_UPLINK_ACCESS_ENABLED)
    @$(call zrbn_check_bool,1,1,$(RBN_UPLINK_ACCESS_ENABLED))
    @$(call zrbn_check_exported,1,1,RBN_UPLINK_DNS_GLOBAL)
    @$(call zrbn_check_bool,1,1,$(RBN_UPLINK_DNS_GLOBAL))
    @$(call zrbn_check_exported,1,1,RBN_UPLINK_ACCESS_GLOBAL)
    @$(call zrbn_check_bool,1,1,$(RBN_UPLINK_ACCESS_GLOBAL))
    @test "$(RBN_UPLINK_DNS_ENABLED)" = "0" -o "$(RBN_UPLINK_DNS_GLOBAL)" = "1" || \
        $(call zrbn_check_exported,1,1,RBN_UPLINK_ALLOWED_DOMAINS)
    @test "$(RBN_UPLINK_ACCESS_ENABLED)" = "0" -o "$(RBN_UPLINK_ACCESS_GLOBAL)" = "1" || \
        $(call zrbn_check_exported,1,1,RBN_UPLINK_ALLOWED_CIDRS)

zrbn_render_uplink:
    @echo "Network Uplink Configuration:"
    @echo "  DNS Resolution: $(if $(filter 1,$(RBN_UPLINK_DNS_ENABLED)),ENABLED,DISABLED)"
    @test "$(RBN_UPLINK_DNS_ENABLED)" != "1" || \
        echo "  DNS Mode: $(if $(filter 1,$(RBN_UPLINK_DNS_GLOBAL)),GLOBAL,RESTRICTED)"
    @test "$(RBN_UPLINK_DNS_ENABLED)" != "1" -o "$(RBN_UPLINK_DNS_GLOBAL)" = "1" || \
        echo "  Allowed Domains: $(RBN_UPLINK_ALLOWED_DOMAINS)"
    @echo "  IP Access: $(if $(filter 1,$(RBN_UPLINK_ACCESS_ENABLED)),ENABLED,DISABLED)"
    @test "$(RBN_UPLINK_ACCESS_ENABLED)" != "1" || \
        echo "  Access Mode: $(if $(filter 1,$(RBN_UPLINK_ACCESS_GLOBAL)),GLOBAL,RESTRICTED)"
    @test "$(RBN_UPLINK_ACCESS_ENABLED)" != "1" -o "$(RBN_UPLINK_ACCESS_GLOBAL)" = "1" || \
        echo "  Allowed CIDRs: $(RBN_UPLINK_ALLOWED_CIDRS)"

# Volume Mount validation and rendering
zrbn_validate_volumes:
    @true

zrbn_render_volumes:
    @echo "Volume Mounts:"
    @test -z "$(RBN_VOLUME_MOUNTS)" || echo "  $(RBN_VOLUME_MOUNTS)"
