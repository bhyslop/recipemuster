#########################
# Container Build Makefile Console
#

zCBC_MBC_MAKEFILE = $(zCBC_TOOLS_DIR)/mbc.MakefileBashConsole.mk

# Specify line prefix used in MBC display commands
MBC_ARG__CONTEXT_STRING = cbc.ContainerBuildConsole.mk

include $(zCBC_MBC_MAKEFILE)

# GitHub Personal Access Token
zCBC_GITHUB_PAT ?= $(shell echo $$RBM_GITHUB_PAT)

# GitHub repository information
zCBC_GITHUB_REPO = $(shell git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')

# GitHub API URL
zCBC_GITHUB_API_URL = https://api.github.com

## External Targets

bc-TR.sh: zcbc_argcheck_rule
	$(MBC_START) "Trigger remote build action"
	$(MBC_STEP) "Triggering GitHub Action..."
	curl -X POST $(zCBC_GITHUB_API_URL)/repos/$(zCBC_GITHUB_REPO)/dispatches     \
	  -H "Authorization: token $(zCBC_GITHUB_PAT)"                               \
	  -H "Accept: application/vnd.github.v3+json"                                \
	  -d '{"event_type": "build_containers"}'                                    \
	  --fail --silent --show-error
	$(MBC_STEP) "Waiting for action to complete..."
	$(MBC_PASS) "GitHub Action triggered successfully. Check the Actions tab for progress."

bc-LI.sh: zcbc_argcheck_rule
	$(MBC_START) "List images in container registry"
	$(MBC_STEP) "Fetching image list..."
	curl -X GET $(zCBC_GITHUB_API_URL)/user/packages?package_type=container      \
	  -H "Authorization: token $(zCBC_GITHUB_PAT)"                               \
	  -H "Accept: application/vnd.github.v3+json"                                \
	  --fail --silent --show-error                                               \
	  | jq -r '.[] | select(.repository.full_name == "$(zCBC_GITHUB_REPO)") | .name'
	$(MBC_PASS) "Image list fetched successfully."

bc-DI.sh: zcbc_argcheck_rule
	$(MBC_START) "Delete image from container registry"
	$(MBC_STEP) "Prompting for confirmation..."
	@read -p "Enter the name of the image to delete: " image_name               &&\
	 read -p "Type YES to confirm deletion of $$image_name: " confirm           &&\
	 test "$$confirm" = "YES"                                                   &&\
	 $(MBC_STEP) "Deleting image $$image_name..."                               &&\
	 curl -X DELETE "$(zCBC_GITHUB_API_URL)/user/packages/container/$(zCBC_GITHUB_REPO)/$$image_name" \
	   -H "Authorization: token $(zCBC_GITHUB_PAT)"                             \
	   -H "Accept: application/vnd.github.v3+json"                              \
	   --fail --silent --show-error
	$(MBC_PASS) "Image deletion process completed."

## Internal Targets

zcbc_argcheck_rule:
	@test -n "$(zCBC_GITHUB_PAT)" || { $(MBC_FAIL) "RBM_GITHUB_PAT environment variable is not set"; exit 1; }

