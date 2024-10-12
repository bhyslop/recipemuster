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
	echo "Checking required variables..."
	echo "SHELL variable: $$SHELL"
	echo "Current shell (ps):"
	ps -p $$$$
	echo "Shell from /proc/self/exe:"
	readlink /proc/self/exe
	echo "Available shells:"
	cat /etc/shells
	echo "Bash version (if available):"
	bash --version || echo "Bash not found or not executable"
	echo "Sh version (if available):"
	sh --version || echo "Sh not found or not executable"
	echo "Environment variables:"
	env
	for var in $(REQUIRED_BGCV_VARS); do \
	  eval value=\$$$$var; \
	  test -n "$$value" || (echo "Error: Undefined required variable $$var" && false); \
	done
	echo "All required variables are defined."


bgcfh_display_rule:
	$(foreach var,$(BGCV_VARS), echo "$(var)=$($(var))";)

