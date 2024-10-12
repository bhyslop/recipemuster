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
	echo "Gathering shell information..."
	echo "SHELL variable: $$SHELL"
	echo "Current shell (ps):"
	ps -p $$$$
	echo "Shell from /proc/self/exe:"
	readlink /proc/self/exe || echo "readlink not available"
	echo "Available shells:"
	cat /etc/shells || echo "Unable to read /etc/shells"
	echo "Bash version (if available):"
	bash --version || echo "Bash not found or not executable"
	echo "Sh version (if available):"
	sh --version || echo "Sh not found or not executable"
	@if [ -z "$$BASH_VERSION" ]; then \
		echo "Error: This makefile requires bash to run"; \
		exit 1; \
	fi
	echo "Displaying and checking BGCV variables..."
	for var in $(BGCV_VARS); do \
		value=$$(eval echo \$${$$var}); \
		echo "$$var=$$value"; \
		if [ -z "$$value" ]; then \
			echo "Error: $$var is not set or empty"; \
			exit 1; \
		fi; \
	done
	echo "All BGCV variables are set and non-empty."


bgcfh_display_rule:
	$(foreach var,$(BGCV_VARS), echo "$(var)=$($(var))";)

