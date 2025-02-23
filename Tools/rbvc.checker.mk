# OUCH experiment to try and figure gh actions shelling
SHELL = /bin/bash

include rbv.variables.mk

# Dynamic list of all RBV_ variables
RBV_VARS := $(filter RBV_%,$(.VARIABLES))

# Specific list of required RBV_ variables
REQUIRED_RBV_VARS :=      \
  RBV_BUILD_ARCHITECTURES \
  RBV_HISTORY_DIR         \
  RBV_REGISTRY_NAME       \
  RBV_REGISTRY_OWNER      \


rbvc_check_rule:
	@for var in $(REQUIRED_RBV_VARS); do \
	  test -z "$${!var}" && echo "Error: $$var is not set or empty" && exit 1 || true; \
	done


rbvc_display_rule:
	$(foreach var,$(RBV_VARS), echo "$(var)=$($(var))";)

