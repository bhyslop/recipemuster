include bgc-config.mk


# Dynamic list of all BGCV_ variables
BGCV_VARS := $(filter BGCV_%,$(.VARIABLES))

# Specific list of required BGCV_ variables
REQUIRED_BGCV_VARS := BGCV_REGISTRY_OWNER BGCV_REGISTRY_NAME BGCV_BUILD_ARCHITECTURES \
                      BGCV_HISTORY_DIR BGCV_RECIPES_DIR BGCV_RECIPE_PATTERN \
                      BGCV_TIMEOUT_MINUTES BGCV_CONCURRENCY BGCV_MAX_PARALLEL \
                      BGCV_CONTINUE_ON_ERROR BGCV_FAIL_FAST


bgcv_check_rule:
	@echo "Checking required variables..."
	@for var in $(REQUIRED_BGCV_VARS); do \
		if [ -z "$($$var)" ]; then \
			echo "Error: Undefined required variable $$var"; \
			exit 1; \
		fi; \
	done
	@echo "All required variables are defined."


bgcv_display_rule:
	@$(foreach var,$(BGCV_VARS), echo "$(var)=$($(var))";)


