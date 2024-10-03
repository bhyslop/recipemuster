#########################
# Container Management Console
#

zCMC_MBC_MAKEFILE = $(zCMC_TOOLS_DIR)/mbc.MakefileBashConsole.mk

# Specify line prefix used in MBC display commands
MBC_ARG__CONTEXT_STRING = cmc.ContainerManagementConsole.mk

include $(zCMC_MBC_MAKEFILE)

# GitHub-related variables
zCMC_GITHUB_PAT ?= $(GITHUB_PAT)
zCMC_GITHUB_REPO ?= $(shell git config --get remote.origin.url | sed 's/.*://;s/.git$//')
zCMC_GITHUB_API_URL = https://api.github.com
zCMC_GITHUB_CONTAINER_REGISTRY = ghcr.io

# Action Trigger
bc-TR.%: zcmc_argcheck_rule
	$(MBC_START) "Triggering GitHub Action for container building"
	$(MBC_STEP) "Dispatching repository event..."
	curl -X POST $(zCMC_GITHUB_API_URL)/repos/$(zCMC_GITHUB_REPO)/dispatches \
	  -H "Authorization: token $(zCMC_GITHUB_PAT)" \
	  -H "Accept: application/vnd.github.v3+json" \
	  -d '{"event_type": "build_containers"}'
	$(MBC_STEP) "Waiting for action to complete..."
	while true; do \
	  status=$$(curl -s -H "Authorization: token $(zCMC_GITHUB_PAT)" \
	    $(zCMC_GITHUB_API_URL)/repos/$(zCMC_GITHUB_REPO)/actions/runs | \
	    jq -r '.workflow_runs[0].status'); \
	  if [ "$$status" = "completed" ]; then \
	    break; \
	  fi; \
	  sleep 10; \
	done
	$(MBC_PASS) "GitHub Action completed"

# List Images
bc-LI.%: zcmc_argcheck_rule
	$(MBC_START) "Listing images in container registry"
	$(MBC_STEP) "Fetching image list..."
	curl -s -H "Authorization: token $(zCMC_GITHUB_PAT)" \
	  $(zCMC_GITHUB_API_URL)/user/packages?package_type=container | \
	  jq -r '.[] | select(.name | startswith("$(zCMC_GITHUB_REPO)")) | .name'
	$(MBC_PASS) "Image list retrieved"

# Delete Image
bc-DI.%: zcmc_argcheck_rule
	$(MBC_START) "Deleting image from container registry"
	$(MBC_STEP) "Prompting for confirmation..."
	@read -p "Enter the name of the image to delete: " image_name && \
	read -p "Are you sure you want to delete $$image_name? (y/N): " confirm && \
	[[ $$confirm == [yY] || $$confirm == [yY][eE][sS] ]] || (echo "Deletion cancelled."; exit 1)
	$(MBC_STEP) "Deleting image..."
	@image_name="$$image_name" && \
	curl -X DELETE -H "Authorization: token $(zCMC_GITHUB_PAT)" \
	  $(zCMC_GITHUB_API_URL)/user/packages/container/$(zCMC_GITHUB_REPO)/versions/$$image_name
	$(MBC_PASS) "Image deleted successfully"

# Internal rule for argument checking
zcmc_argcheck_rule:
	@:
