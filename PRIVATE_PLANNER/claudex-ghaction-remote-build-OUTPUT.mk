## GitHub Container Registry Build Makefile
## Created: 2024-10-07

# Master Makefile Prefix
GHCR_PREFIX := GHCR

# External variables
GITHUB_REPO ?=
RBM_GITHUB_PAT ?=
CONTAINER_TOOL ?= docker

# Internal variables
zGHCR_API_URL := https://api.github.com
zGHCR_WORKFLOW_FILE := .github/workflows/build-containers.yml
zGHCR_CONFIG_FILE := rbm-config.yml
zGHCR_LAST_WORKFLOW_RUN_FILE := ../LAST_GET_WORKFLOW_RUN.txt

# Error handling and messaging
define MBC_START
	@echo "Starting: $(1)"
endef

define MBC_STEP
	@echo "Step: $(1)"
endef

define MBC_FAIL
	@echo "Failed: $(1)" && exit 1
endef

define MBC_PASS
	@echo "Completed: $(1)"
endef

# External Targets

bc-trigger-build.sh:
	$(MBC_START) "Triggering container build"
	test -n "$(GITHUB_REPO)" || (echo "GITHUB_REPO is not set" && false)
	test -n "$(RBM_GITHUB_PAT)" || (echo "RBM_GITHUB_PAT is not set" && false)
	$(MBC_STEP) "Initiating build via repository_dispatch event"
	curl -X POST -H "Authorization: token $(RBM_GITHUB_PAT)" \
		-H "Accept: application/vnd.github.v3+json" \
		$(zGHCR_API_URL)/repos/$(GITHUB_REPO)/dispatches \
		-d '{"event_type": "build_containers"}'
	$(MBC_STEP) "Monitoring build progress"
	$(MAKE) zghcr_monitor_build_rule
	$(MBC_PASS) "Build triggered and monitored"

bc-query-build.sh:
	$(MBC_START) "Querying build status"
	test -f $(zGHCR_LAST_WORKFLOW_RUN_FILE) || (echo "No recent build found" && false)
	$(MAKE) zghcr_query_build_rule
	$(MBC_PASS) "Build status queried"

bc-list-images.sh:
	$(MBC_START) "Listing container registry images"
	test -n "$(GITHUB_REPO)" || (echo "GITHUB_REPO is not set" && false)
	test -n "$(RBM_GITHUB_PAT)" || (echo "RBM_GITHUB_PAT is not set" && false)
	$(MAKE) zghcr_list_images_rule
	$(MBC_PASS) "Images listed"

bc-delete-image.sh:
	$(MBC_START) "Deleting specified image"
	test -n "$(GITHUB_REPO)" || (echo "GITHUB_REPO is not set" && false)
	test -n "$(RBM_GITHUB_PAT)" || (echo "RBM_GITHUB_PAT is not set" && false)
	test -n "$(IMAGE_NAME)" || (echo "IMAGE_NAME is not set" && false)
	$(MAKE) zghcr_delete_image_rule
	$(MBC_PASS) "Image deletion process completed"

# Internal Targets

zghcr_monitor_build_rule:
	$(MBC_STEP) "Polling GitHub API for build status"
	while true; do \
		workflow_run_url=$$(curl -s -H "Authorization: token $(RBM_GITHUB_PAT)" \
			-H "Accept: application/vnd.github.v3+json" \
			"$(zGHCR_API_URL)/repos/$(GITHUB_REPO)/actions/runs?event=repository_dispatch" | \
			jq -r '.workflow_runs[0].url'); \
		echo "$$workflow_run_url" > $(zGHCR_LAST_WORKFLOW_RUN_FILE); \
		status=$$(curl -s -H "Authorization: token $(RBM_GITHUB_PAT)" \
			-H "Accept: application/vnd.github.v3+json" "$$workflow_run_url" | \
			jq -r '.status'); \
		if [ "$$status" = "completed" ]; then \
			echo "Build finished"; \
			break; \
		fi; \
		echo "Build status: $$status"; \
		sleep 30; \
	done

zghcr_query_build_rule:
	$(MBC_STEP) "Querying build status"
	workflow_run_url=$$(cat $(zGHCR_LAST_WORKFLOW_RUN_FILE)); \
	status=$$(curl -s -H "Authorization: token $(RBM_GITHUB_PAT)" \
		-H "Accept: application/vnd.github.v3+json" "$$workflow_run_url" | \
		jq -r '.status'); \
	conclusion=$$(curl -s -H "Authorization: token $(RBM_GITHUB_PAT)" \
		-H "Accept: application/vnd.github.v3+json" "$$workflow_run_url" | \
		jq -r '.conclusion'); \
	echo "Build status: $$status"; \
	echo "Build conclusion: $$conclusion"; \
	test "$$status" = "completed" || exit 1

zghcr_list_images_rule:
	$(MBC_STEP) "Fetching and displaying images"
	curl -s -H "Authorization: token $(RBM_GITHUB_PAT)" \
		-H "Accept: application/vnd.github.v3+json" \
		"$(zGHCR_API_URL)/user/packages?package_type=container" | \
		jq -r '.[] | select(.repository.full_name == "$(GITHUB_REPO)") | \
			"Name: \(.name)\tTag: \(.metadata.container.tags[0])\tSize: \(.size)\tCreated: \(.created_at)"' | \
		column -t -s $$'\t'

zghcr_delete_image_rule:
	$(MBC_STEP) "Fetching image details"
	image_details=$$(curl -s -H "Authorization: token $(RBM_GITHUB_PAT)" \
		-H "Accept: application/vnd.github.v3+json" \
		"$(zGHCR_API_URL)/user/packages?package_type=container" | \
		jq -r '.[] | select(.repository.full_name == "$(GITHUB_REPO)" and .name == "$(IMAGE_NAME)")'); \
	echo "Image details:"; \
	echo "$$image_details" | jq .; \
	read -p "Are you sure you want to delete this image? (y/N) " confirm && \
	test "$$confirm" = "y" && \
	package_id=$$(echo "$$image_details" | jq -r '.id') && \
	curl -X DELETE -H "Authorization: token $(RBM_GITHUB_PAT)" \
		-H "Accept: application/vnd.github.v3+json" \
		"$(zGHCR_API_URL)/user/packages/container/$(IMAGE_NAME)/versions/$$package_id" && \
	echo "Image deleted successfully" || \
	echo "Image deletion cancelled or failed"
