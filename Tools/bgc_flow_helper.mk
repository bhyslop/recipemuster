# OUCH experiment to try and figure gh actions shelling
SHELL = /bin/bash

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
	@for var in $(REQUIRED_BGCV_VARS); do \
	  test -z "$${!var}" && echo "Error: $$var is not set or empty" && exit 1 || true; \
	done


bgcfh_display_rule:
	$(foreach var,$(BGCV_VARS), echo "$(var)=$($(var))";)

