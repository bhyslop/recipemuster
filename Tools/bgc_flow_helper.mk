include bgc-config.mk

# Dynamic list of all BGCV_ variables
BGCV_VARS := $(filter BGCV_%,$(.VARIABLES))

# Specific list of required BGCV_ variables
REQUIRED_BGCV_VARS :=      \
  BGCV_BUILD_ARCHITECTURES \
  BGCV_HISTORY_DIR         \
  BGCV_RECIPES_DIR         \
  BGCV_REGISTRY_NAME       \
  BGCV_REGISTRY_OWNER      \


bgcfh_check_rule:
	@echo "Checking required variables..."
	@echo "Current shell: $$SHELL"
	@echo "Current shell version:"
	@$$SHELL --version
	@for var in $(REQUIRED_BGCV_VARS); do \
	  test -n "$${!var}" || (echo "Error: Undefined required variable $$var" && false); \
	done
	@echo "All required variables are defined."


bgcfh_display_rule:
	@$(foreach var,$(BGCV_VARS), echo "$(var)=$($(var))";)
