# Build Github Containers Makefile

include bgc-config.mk
include ../BGC_STATION.mk
include ../secrets/github-ghcr-play.env
include $(BGCV_TOOLS_DIR)/bgc_flow_helper.mk

zBGC_GITAPI_URL := https://api.github.com

# OUCH fix this
BGC_SECRET_GITHUB_PAT = $(GITHUB_GHCR_PLAY_PAT)

zBGC_LAST_RUN_CACHE = ../LAST_GET_WORKFLOW_RUN.txt
zBGC_LAST_RUN_CONTENTS := $(shell cat $(zBGC_LAST_RUN_CACHE))

BGC_ARG_RECIPE ?=

zBGC_CURL_HEADERS := -H 'Authorization: token $(BGC_SECRET_GITHUB_PAT)' \
                     -H 'Accept: application/vnd.github.v3+json'

zBGC_CMD_TRIGGER_BUILD := curl -X POST $(zBGC_CURL_HEADERS) \
    '$(zBGC_GITAPI_URL)/repos/$(BGCV_REGISTRY_OWNER)/$(BGCV_REGISTRY_NAME)/dispatches' \
    -d '{"event_type": "build_containers", "client_payload": {"dockerfile": "$(BGC_ARG_RECIPE)"}}'

zBGC_CMD_GET_WORKFLOW_RUN := curl -s $(zBGC_CURL_HEADERS) \
    '$(zBGC_GITAPI_URL)/repos/$(BGCV_REGISTRY_OWNER)/$(BGCV_REGISTRY_NAME)/actions/runs?event=repository_dispatch&branch=main&per_page=1'

zBGC_CMD_GET_SPECIFIC_RUN := curl -s  $(zBGC_CURL_HEADERS) \
    '$(zBGC_GITAPI_URL)/repos/$(BGCV_REGISTRY_OWNER)/$(BGCV_REGISTRY_NAME)/actions/runs/'

zBGC_CMD_LIST_IMAGES := curl -s $(zBGC_CURL_HEADERS) \
    '$(zBGC_GITAPI_URL)/user/packages?package_type=container'

zBGC_CMD_DELETE_IMAGE := curl -X DELETE $(zBGC_CURL_HEADERS) \
    '$(zBGC_GITAPI_URL)/user/packages/container/$(zBGC_IMAGE_NAME)/versions/$(zBGC_IMAGE_VERSION)'

zBGC_CMD_LIST_PACKAGE_VERSIONS := curl -s $(zBGC_CURL_HEADERS) \
    '$(zBGC_GITAPI_URL)/user/packages/container/$(BGCV_REGISTRY_NAME)/versions'

zBGC_CMD_GET_JOBS := curl -s $(zBGC_CURL_HEADERS) \
     '$(zBGC_GITAPI_URL)/repos/$(BGCV_REGISTRY_OWNER)/$(BGCV_REGISTRY_NAME)/actions/runs/$(zBGC_LAST_RUN_CONTENTS)/jobs'

zBGC_CMD_GET_LOGS := curl -sL $(zBGC_CURL_HEADERS) \
     '$(zBGC_GITAPI_URL)/repos/$(BGCV_REGISTRY_OWNER)/$(BGCV_REGISTRY_NAME)/actions/runs/$(zBGC_LAST_RUN_CONTENTS)/logs'

zbgc_argcheck_rule: bgcfh_check_rule
	$(MBC_START) "Checking needed variables..."
	@test -n "$(BGC_SECRET_GITHUB_PAT)"    || ($(MBC_SEE_RED) "Error: BGC_SECRET_GITHUB_PAT unset" && false)
	@test -n "$(zBGC_GITAPI_URL)"          || ($(MBC_SEE_RED) "Error: zBGC_GITAPI_URL unset"       && false)
	$(MBC_PASS)

bc-trigger-build.sh: zbgc_argcheck_rule
	$(MBC_START) "Triggering container build on specified recipe or dockerfile"
	@test "$(BGC_ARG_RECIPE)" != "" || ($(MBC_SEE_RED) "Error: BGC_ARG_RECIPE unset" && false)
	@$(zBGC_CMD_TRIGGER_BUILD)
	$(MBC_STEP) "Pausing for GitHub to process the dispatch event"
	@sleep 5
	@$(zBGC_CMD_GET_WORKFLOW_RUN) | jq -r '.workflow_runs[0].id' > $(zBGC_LAST_RUN_CACHE)
	@test -s $(zBGC_LAST_RUN_CACHE) || ($(MBC_SEE_RED) "Failed to obtain workflow run ID" && false)
	$(MBC_STEP) "See progress at:"
	$(MBC_SHOW_YELLOW) "   https://github.com/bhyslop/recipemuster/actions/runs/"$$(cat $(zBGC_LAST_RUN_CACHE))
	$(MBC_PASS)

bc-query-build.sh: zbgc_argcheck_rule
	$(MBC_START) "Querying build status"
	@$(zBGC_CMD_GET_SPECIFIC_RUN)$$(cat $(zBGC_LAST_RUN_CACHE)) | \
	  jq -r '.status' | ( read status && test "$$status" == "completed" || \
	     (echo "Build ongoing. Current status: $$status" && false))
	@echo "Build succeeded"
	$(MBC_PASS)

bc-list-images.sh: zbgc_argcheck_rule
	$(MBC_START) "Listing container registry images and versions"
	@$(zBGC_CMD_LIST_IMAGES) | jq -r '.[] | select(.package_type=="container") | .name' | while read -r package_name; do \
		echo "Package: $$package_name"; \
		echo "Versions:"; \
		$(zBGC_CMD_LIST_PACKAGE_VERSIONS) | jq -r '.[] | "\(.metadata.container.tags[]) \(.created_at) \(.name)"' | \
		sort -r | \
		awk '{split($$3, digest, ":"); printf "%-40s %-25s %.12s\n", $$1, $$2, digest[2]}' | \
		awk 'BEGIN {printf "%-40s %-25s %-12s\n", "Tag", "Created At", "Short Digest"}1'; \
		echo; \
	done
	$(MBC_PASS)

bc-delete-image.sh: zbgc_argcheck_rule
	$(MBC_START) "Deleting container registry image"
	@read -p "Enter image name to delete: " zBGC_IMAGE_NAME && \
	read -p "Enter image version to delete: " zBGC_IMAGE_VERSION && \
	echo "Deleting image: $$zBGC_IMAGE_NAME:$$zBGC_IMAGE_VERSION" && \
	read -p "Are you sure? (y/n) " confirm && \
	[ "$$confirm" = "y" ] && \
	$(zBGC_CMD_DELETE_IMAGE) && \
	echo "Image deleted successfully" || \
	(echo "Deletion cancelled or failed" && exit 1)
	$(MBC_PASS)

bc-display-config:
	$(MBC_START) "Displaying configuration variables"
	@$(MAKE) -f bgcv.Variables.mk bgcv_display_rule
	$(MBC_PASS)

bc-get-jobs.sh: zbgc_argcheck_rule
	$(MBC_START) "Get job info"
	$(zBGC_CMD_GET_JOBS) | jq '.jobs[] | select(.name == "build") | .steps[] | select(.name == "Build and push")'
	$(MBC_PASS)

bc-get-logs.sh: zbgc_argcheck_rule
	$(MBC_START) "Downloading and processing logs"
	@$(zBGC_CMD_GET_LOGS) > ../workflow_logs.zip
	$(MBC_PASS)


