# Regime Prefix: rbb_
# Assignment Prefix: RBB_

# ======= Network Configuration Validation =======

zrbb_validate_network: \
  zrbb_validate_dns

zrbb_validate_dns:
	@test "$(RBB_DNS_SERVER)" = "" || $(call MBC_CHECK_EXPORTED,1,RBB_DNS_SERVER)
	@test "$(RBB_DNS_SERVER)" = "" || $(call MBC_CHECK__IS_IPV4,1,$(RBB_DNS_SERVER))

# ======= System Configuration Validation =======

zrbb_validate_system: \
  zrbb_validate_nameplate \
  zrbb_validate_registry

zrbb_validate_nameplate:
	@test "$(RBB_NAMEPLATE_PATH)" = "" || $(call MBC_CHECK_EXPORTED,1,RBB_NAMEPLATE_PATH)
	@test "$(RBB_NAMEPLATE_PATH)" = "" || $(call MBC_CHECK_NONEMPTY,1,$(RBB_NAMEPLATE_PATH))

zrbb_validate_registry:
	@test "$(RBB_REGISTRY_SERVER)" = "" || $(call MBC_CHECK_EXPORTED,1,RBB_REGISTRY_SERVER)
	@test "$(RBB_REGISTRY_SERVER)" = "" || $(call MBC_CHECK_ISDOMAIN,1,$(RBB_REGISTRY_SERVER))

# ======= Primary Interface Targets =======

rbb_validate: zrbb_validate_network zrbb_validate_system

rbb_define:
	@echo "Base Configuration Regime"
	@echo ""
	@echo "== Network Configuration =="
	@echo "RBB_DNS_SERVER             # Optional: DNS resolver for Bottle Services"
	@echo ""
	@echo "== System Configuration =="
	@echo "RBB_NAMEPLATE_PATH         # Optional: Directory for Nameplate definitions"
	@echo "RBB_REGISTRY_SERVER        # Optional: Container registry server address"

rbb_render:
	$(MBC_STEP) "Network Configuration:"
	@test "$(RBB_DNS_SERVER)" = "" || echo "DNS Server: $(RBB_DNS_SERVER)"
	$(MBC_STEP) "System Configuration:"
	@test "$(RBB_NAMEPLATE_PATH)"  = "" || echo "Nameplate Path: $(RBB_NAMEPLATE_PATH)"
	@test "$(RBB_REGISTRY_SERVER)" = "" || echo "Registry Server: $(RBB_REGISTRY_SERVER)"

# Environment rollup for container usage
RBB__ROLLUP_ENVIRONMENT_VAR := \
  RBB_DNS_SERVER='$(RBB_DNS_SERVER)' \
  RBB_NAMEPLATE_PATH='$(RBB_NAMEPLATE_PATH)' \
  RBB_REGISTRY_SERVER='$(RBB_REGISTRY_SERVER)'

