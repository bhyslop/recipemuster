#########################
# Simple Makefile Console
#

zSMC_MBC_MAKEFILE = $(zSMC_TOOLS_DIR)/mbc.MakefileBashConsole.mk

# Specify line prefix used in MBC display commands
MBC_ARG__CONTEXT_STRING = smc.ContainerManagement.mk

include $(zSMC_MBC_MAKEFILE)

# GitHub-related variables
zSMC_GITHUB_TOKEN ?= $(GITHUB_PAT)
zSMC_GITHUB_REPO  ?= $(shell git config --get remote.origin.url | sed 's/.*://;s/.git$//')
zSMC_REGISTRY     := ghcr.io

# External Targets

bc-trigger-build.sh: zsmc_argcheck_rule
	$(MBC_START) "Triggering GitHub Action to build containers"
	@curl -X POST -H "Authorization: token $(zSMC_GITHUB_TOKEN)" \
		-H "Accept: application/vnd.github.v3+json" \
		https://api.github.com/repos/$(zSMC_GITHUB_REPO)/dispatches \
		-d '{"event_type": "build-containers"}'
	@echo "Waiting for GitHub Action to complete..."
	@while true; do \
		status=$$(curl -s -H "Authorization: token $(zSMC_GITHUB_TOKEN)" \
			-H "Accept: application/vnd.github.v3+json" \
			https://api.github.com/repos/$(zSMC_GITHUB_REPO)/actions/runs | \
			jq -r '.workflow_runs[0].status'); \
		if [ "$$status" = "completed" ]; then \
			break; \
		fi; \
		sleep 10; \
	done
	$(MBC_PASS) "GitHub Action completed"

bc-list-images.sh: zsmc_argcheck_rule
	$(MBC_START) "Listing images in the container registry"
	@curl -s -H "Authorization: token $(zSMC_GITHUB_TOKEN)" \
		-H "Accept: application/vnd.github.v3+json" \
		https://api.github.com/orgs/$(shell echo $(zSMC_GITHUB_REPO) | cut -d'/' -f1)/packages/container/$(shell echo $(zSMC_GITHUB_REPO) | cut -d'/' -f2)/versions | \
		jq -r '.[] | "\(.metadata.container.tags[0]) - \(.name)"'
	$(MBC_PASS) "Image list retrieved"

bc-delete-image.sh: zsmc_argcheck_rule
	$(MBC_START) "Deleting image from the container registry"
	@read -p "Enter the image tag to delete: " image_tag; \
	read -p "Are you sure you want to delete $$image_tag? (y/N): " confirm; \
	if [ "$$confirm" = "y" ] || [ "$$confirm" = "Y" ]; then \
		version_id=$$(curl -s -H "Authorization: token $(zSMC_GITHUB_TOKEN)" \
			-H "Accept: application/vnd.github.v3+json" \
			https://api.github.com/orgs/$(shell echo $(zSMC_GITHUB_REPO) | cut -d'/' -f1)/packages/container/$(shell echo $(zSMC_GITHUB_REPO) | cut -d'/' -f2)/versions | \
			jq -r '.[] | select(.metadata.container.tags[0] == "'$$image_tag'") | .id'); \
		if [ -n "$$version_id" ]; then \
			curl -X DELETE -H "Authorization: token $(zSMC_GITHUB_TOKEN)" \
				-H "Accept: application/vnd.github.v3+json" \
				https://api.github.com/orgs/$(shell echo $(zSMC_GITHUB_REPO) | cut -d'/' -f1)/packages/container/$(shell echo $(zSMC_GITHUB_REPO) | cut -d'/' -f2)/versions/$$version_id; \
			$(MBC_PASS) "Image $$image_tag deleted"; \
		else \
			$(MBC_FAIL) "Image $$image_tag not found"; \
		fi; \
	else \
		$(MBC_PASS) "Deletion cancelled"; \
	fi

# Internal targets

zsmc_argcheck_rule:
	@if [ -z "$(zSMC_GITHUB_TOKEN)" ]; then \
		$(MBC_FAIL) "GITHUB_PAT environment variable is not set"; \
		exit 1; \
	fi

