# Build Github Containers Makefile

include bgc-config.mk
include ../BGC_STATION.mk
include ../secrets/github-ghcr-play.env
include $(BGCV_TOOLS_DIR)/bgc_flow_helper.mk

zBGC_GITAPI_URL := https://api.github.com

# OUCH fix this
BGC_SECRET_GITHUB_PAT = $(GITHUB_GHCR_PLAY_PAT)

zBGC_CMD_TRIGGER_BUILD = curl -X POST \
    -H "Authorization: token $(BGC_SECRET_GITHUB_PAT)" \
    -H "Accept: application/vnd.github.v3+json" \
    $(zBGC_GITAPI_URL)/repos/$(BGCV_REGISTRY_OWNER)/$(BGCV_REGISTRY_NAME)/dispatches \
    -d '{"event_type": "build_containers", "client_payload": {"dockerfile": "$(DOCKERFILE)"}}'

zBGC_CMD_GET_WORKFLOW_RUN = curl -s \
    -H "Authorization: token $(BGC_SECRET_GITHUB_PAT)" \
    -H "Accept: application/vnd.github.v3+json" \
    $(zBGC_GITAPI_URL)/repos/$(BGCV_REGISTRY_OWNER)/$(BGCV_REGISTRY_NAME)/actions/runs

zBGC_CMD_LIST_IMAGES = curl -s \
    -H "Authorization: token $(BGC_SECRET_GITHUB_PAT)" \
    -H "Accept: application/vnd.github.v3+json" \
    $(zBGC_GITAPI_URL)/user/packages?package_type=container

zBGC_CMD_DELETE_IMAGE = curl -X DELETE \
    -H "Authorization: token $(BGC_SECRET_GITHUB_PAT)" \
    -H "Accept: application/vnd.github.v3+json" \
    $(zBGC_GITAPI_URL)/user/packages/container/$(zBGC_IMAGE_NAME)/versions/$(zBGC_IMAGE_VERSION)

zbgc_argcheck_rule: bgcfh_check_rule
	$(MBC_START) "Checking needed variables..."
	test -n "$(BGC_SECRET_GITHUB_PAT)"    || (echo "BGC_SECRET_GITHUB_PAT is not set"     && false)
	test -n "$(zBGC_GITAPI_URL)"          || (echo "zBGC_GITAPI_URL is not set"           && false)
	$(MBC_PASS)

bc-trigger-build.sh: zbgc_argcheck_rule
	$(MBC_START) "Triggering container build"
	@if [ -z "$(DOCKERFILE)" ]; then \
		echo "Error: DOCKERFILE is not set. Usage: make bc-trigger-build.sh DOCKERFILE=path/to/Dockerfile"; \
		exit 1; \
	fi
	$(zBGC_CMD_TRIGGER_BUILD)
	jq -r '.workflow_runs[0].url' <<< "$$($(zBGC_CMD_GET_WORKFLOW_RUN))" > ../LAST_GET_WORKFLOW_RUN.txt
	$(MBC_STEP) "Query api determined to be::"
	$(MBC_SHOW_YELLOW) "   " $(shell cat ../LAST_GET_WORKFLOW_RUN.txt)
	$(MBC_PASS)

bc-query-build.sh: zbgc_argcheck_rule
	$(MBC_START) "Querying build status"
	status=$$(curl -s -H "Authorization: token $(BGC_SECRET_GITHUB_PAT)" -H "Accept: application/vnd.github.v3+json" $$(cat ../LAST_GET_WORKFLOW_RUN.txt) | jq -r '.status')
	if [ "$$status" = "completed" ]; then \
		echo "Build finished"; \
		exit 0; \
	else \
		echo "Build ongoing"; \
		exit 1; \
	fi

bc-list-images.sh: zbgc_argcheck_rule
	$(MBC_START) "Listing container registry images"
	$(zBGC_CMD_LIST_IMAGES) | jq -r '.[] | select(.package_type=="container") | "\(.name)\t\(.version_count)\t\(.html_url)"' | \
		awk 'BEGIN {printf "%-30s %-10s %-50s\n", "Image Name", "Versions", "URL"} {printf "%-30s %-10s %-50s\n", $$1, $$2, $$3}'
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

