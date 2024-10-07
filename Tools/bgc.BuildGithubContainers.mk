# Build Github Containers Makefile

include mbc.MakefileBashConsole.mk
include ../BGC_STATION.mk
include ../secrets/BGC_SECRETS.mk

zBGC_GITHUB_API_URL := https://api.github.com
zBGC_REPO_OWNER := $(BGC_STATION_GITHUB_USERNAME)
zBGC_REPO_NAME := $(BGC_STATION_GITHUB_REPO)

zBGC_CMD_TRIGGER_BUILD = curl -X POST -H "Authorization: token $(BGC_SECRET_GITHUB_PAT)" \
    -H "Accept: application/vnd.github.v3+json" \
    $(zBGC_GITHUB_API_URL)/repos/$(zBGC_REPO_OWNER)/$(zBGC_REPO_NAME)/dispatches \
    -d '{"event_type": "build_containers"}'

zBGC_CMD_GET_WORKFLOW_RUN = curl -s -H "Authorization: token $(BGC_SECRET_GITHUB_PAT)" \
    -H "Accept: application/vnd.github.v3+json" \
    $(zBGC_GITHUB_API_URL)/repos/$(zBGC_REPO_OWNER)/$(zBGC_REPO_NAME)/actions/runs

zBGC_CMD_LIST_IMAGES = curl -s -H "Authorization: token $(BGC_SECRET_GITHUB_PAT)" \
    -H "Accept: application/vnd.github.v3+json" \
    $(zBGC_GITHUB_API_URL)/user/packages?package_type=container

zBGC_CMD_DELETE_IMAGE = curl -X DELETE -H "Authorization: token $(BGC_SECRET_GITHUB_PAT)" \
    -H "Accept: application/vnd.github.v3+json" \
    $(zBGC_GITHUB_API_URL)/user/packages/container/$(zBGC_IMAGE_NAME)/versions/$(zBGC_IMAGE_VERSION)

zBGC_check_vars:
	$(MBC_START) "Checking critical variables"
	test -n "$(BGC_STATION_GITHUB_USERNAME)" || (echo "BGC_STATION_GITHUB_USERNAME is not set" && false)
	test -n "$(BGC_STATION_GITHUB_REPO)"     || (echo "BGC_STATION_GITHUB_REPO is not set" && false)
	test -n "$(BGC_SECRET_GITHUB_PAT)"       || (echo "BGC_SECRET_GITHUB_PAT is not set" && false)
	test -n "$(zBGC_GITHUB_API_URL)"         || (echo "zBGC_GITHUB_API_URL is not set" && false)
	$(MBC_PASS)

bc-trigger-build.sh: zBGC_check_vars
	$(MBC_START) "Triggering container build"
	$(zBGC_CMD_TRIGGER_BUILD)
	jq -r '.workflow_runs[0].url' <<< "$$($(zBGC_CMD_GET_WORKFLOW_RUN))" > ../LAST_GET_WORKFLOW_RUN.txt
	$(MBC_PASS)

bc-query-build.sh: zBGC_check_vars
	$(MBC_START) "Querying build status"
	status=$$(curl -s -H "Authorization: token $(BGC_SECRET_GITHUB_PAT)" -H "Accept: application/vnd.github.v3+json" $$(cat ../LAST_GET_WORKFLOW_RUN.txt) | jq -r '.status')
	if [ "$$status" = "completed" ]; then
		echo "Build finished"
		exit 0
	else
		echo "Build ongoing"
		exit 1
	fi

bc-list-images.sh: zBGC_check_vars
	$(MBC_START) "Listing container registry images"
	$(zBGC_CMD_LIST_IMAGES) | jq -r '.[] | select(.package_type=="container") | "\(.name)\t\(.version_count)\t\(.html_url)"' | \
		awk 'BEGIN {printf "%-30s %-10s %-50s\n", "Image Name", "Versions", "URL"} {printf "%-30s %-10s %-50s\n", $$1, $$2, $$3}'
	$(MBC_PASS)

bc-delete-image.sh: zBGC_check_vars
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


