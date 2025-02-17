# Copyright 2024 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# Build Github Containers Makefile

include bgc-config.mk
include $(BGCV_TOOLS_DIR)/mbc.MakefileBashConsole.mk
include $(BGCV_TOOLS_DIR)/bgc_flow_helper.mk

# Acquire the PAT needed to do GHCR image access/ control
include $(BGCV_GITHUB_PAT_ENV)
BGC_SECRET_GITHUB_PAT = $(BGCSV_PAT)

zBGC_GITAPI_URL := https://api.github.com

zBGC_TEMP_DIR = $(BGCV_TEMP_DIR)

zBGC_CURRENT_WORKFLOW_RUN_CACHE    = $(zBGC_TEMP_DIR)/CURR_WORKFLOW_RUN__$(MBC_NOW).txt
zBGC_CURRENT_WORKFLOW_RUN_CONTENTS = $$(cat $(zBGC_CURRENT_WORKFLOW_RUN_CACHE))

zBGC_DELETE_VERSION_ID_CACHE    = $(zBGC_TEMP_DIR)/BGC_VERSION_ID__$(MBC_NOW).txt
zBGC_DELETE_VERSION_ID_CONTENTS = $$(cat $(zBGC_DELETE_VERSION_ID_CACHE))

zBGC_DELETE_RESULT_CACHE    = $(zBGC_TEMP_DIR)/BGC_DELETE__$(MBC_NOW).txt
zBGC_DELETE_RESULT_CONTENTS = $$(cat $(zBGC_DELETE_RESULT_CACHE))


BGC_ARG_RECIPE ?=

BGC_ARG_FQIN_OUTPUT ?=

zBGC_RECIPE_BASENAME  = $(shell basename $(BGC_ARG_RECIPE))

zBGC_VERIFY_BUILD_DIR     = $(shell ls -td $(BGCV_HISTORY_DIR)/$(basename $(zBGC_RECIPE_BASENAME))* 2>/dev/null | head -n1)
zBGC_VERIFY_FQIN_FILE     = $(zBGC_VERIFY_BUILD_DIR)/docker_inspect_RepoTags_0.txt
zBGC_VERIFY_FQIN_CONTENTS = $$(cat zBGC_VERIFY_FQIN_FILE)


zBGC_CURL_HEADERS := -H 'Authorization: token $(BGC_SECRET_GITHUB_PAT)' \
                     -H 'Accept: application/vnd.github.v3+json'

zBGC_CMD_TRIGGER_BUILD = curl -X POST $(zBGC_CURL_HEADERS) \
    '$(zBGC_GITAPI_URL)/repos/$(BGCV_REGISTRY_OWNER)/$(BGCV_REGISTRY_NAME)/dispatches' \
    -d '{"event_type": "build_containers", "client_payload": {"dockerfile": "$(BGC_ARG_RECIPE)"}}'

zBGC_CMD_GET_WORKFLOW_RUN = curl -s $(zBGC_CURL_HEADERS) \
    '$(zBGC_GITAPI_URL)/repos/$(BGCV_REGISTRY_OWNER)/$(BGCV_REGISTRY_NAME)/actions/runs?event=repository_dispatch&branch=main&per_page=1'

zBGC_CMD_GET_SPECIFIC_RUN = curl -s  $(zBGC_CURL_HEADERS) \
    '$(zBGC_GITAPI_URL)/repos/$(BGCV_REGISTRY_OWNER)/$(BGCV_REGISTRY_NAME)/actions/runs/'$(zBGC_CURRENT_WORKFLOW_RUN_CONTENTS)

zBGC_CMD_LIST_IMAGES = curl -s $(zBGC_CURL_HEADERS) \
    '$(zBGC_GITAPI_URL)/user/packages?package_type=container'

zBGC_CMD_DELETE_IMAGE := curl -X DELETE $(zBGC_CURL_HEADERS) \
    '$(zBGC_GITAPI_URL)/packages/container/$(BGCV_REGISTRY_NAME)/$(BGC_ARG_TAG)'

zBGC_CMD_LIST_PACKAGE_VERSIONS = curl -s $(zBGC_CURL_HEADERS) \
    '$(zBGC_GITAPI_URL)/user/packages/container/$(BGCV_REGISTRY_NAME)/versions'

zBGC_CMD_GET_LOGS = $(zBGC_CMD_GET_SPECIFIC_RUN)/logs

zBGC_CMD_QUERY_LAST_INNER = $(zBGC_CMD_GET_SPECIFIC_RUN)            |\
                             jq -r '.status, .conclusion'           |\
                              (read status && read conclusion &&\
                               echo "  Status: $$status    Conclusion: $$conclusion" &&\
                               test "$$status" == "completed")

zbgc_argcheck_rule: bgcfh_check_rule
	@test -n "$(BGC_SECRET_GITHUB_PAT)"    || ($(MBC_SEE_RED) "Error: BGC_SECRET_GITHUB_PAT unset" && false)
	@test -n "$(zBGC_GITAPI_URL)"          || ($(MBC_SEE_RED) "Error: zBGC_GITAPI_URL unset"       && false)
	@mkdir -p $(zBGC_TEMP_DIR)


zbgc_recipe_argument_check:
	$(MBC_START) "Checking recipe argument"
	@test -n "$(BGC_ARG_RECIPE)" || ($(MBC_SEE_RED) "Error: BGC_ARG_RECIPE unset" && exit 1)
	@test -f "$(BGC_ARG_RECIPE)" || ($(MBC_SEE_RED) "Error: '$(BGC_ARG_RECIPE)' is not a file" && exit 1)
	@! basename "$(BGC_ARG_RECIPE)" | grep -q '[A-Z]' || \
	  ($(MBC_SEE_RED) "Error: Basename of '$(BGC_ARG_RECIPE)' contains uppercase letters" && exit 1)
	@$(MBC_STEP) "$(BGC_ARG_RECIPE) is well formed, moving on..."


bgc-b%: zbgc_argcheck_rule zbgc_recipe_argument_check
	$(MBC_START) "Trigger Build of $(BGC_ARG_RECIPE)"
	$(MBC_STEP) "Make sure your local repo is up to date with github variant..."
	@git fetch                                               &&\
	  git status -uno | grep -q 'Your branch is up to date'  &&\
	  git diff-index --quiet HEAD --                         &&\
	  true || ($(MBC_SEE_RED) "ERROR: Your repo is not cleanly aligned with github variant." &&\
	           $(MBC_SEE_RED) "       Commit or otherwise match to proceed (prevents merge"  &&\
		   $(MBC_SEE_RED) "       conflicts with image history tracking)." && false)
	@$(zBGC_CMD_TRIGGER_BUILD)
	$(MBC_STEP) "Pausing for GitHub to process the dispatch event..."
	@sleep 5
	$(MBC_STEP) "Retrieve workflow run ID..."
	@$(zBGC_CMD_GET_WORKFLOW_RUN) | jq -r '.workflow_runs[0].id' > $(zBGC_CURRENT_WORKFLOW_RUN_CACHE)
	@test -s                                                       $(zBGC_CURRENT_WORKFLOW_RUN_CACHE)
	$(MBC_STEP) "Workflow online at:"
	$(MBC_SHOW_YELLOW) "   https://github.com/$(BGCV_REGISTRY_OWNER)/$(BGCV_REGISTRY_NAME)/actions/runs/"$(zBGC_CURRENT_WORKFLOW_RUN_CONTENTS)
	$(MBC_STEP) "Polling to completion..."
	@until $(zBGC_CMD_QUERY_LAST_INNER); do sleep 3; done
	$(MBC_STEP) "Git Pull for artifacts..."
	@git pull
	$(MBC_STEP) "Verifying build output..."
	@test -n "$(zBGC_VERIFY_BUILD_DIR)" || ($(MBC_SEE_RED) "Error: Missing build directory" && false)
	@cmp "$(BGC_ARG_RECIPE)" "$(zBGC_VERIFY_BUILD_DIR)/recipe.txt" || ($(MBC_SEE_RED) "Error: recipe mismatch" && false)
	$(MBC_STEP) "Extracting FQIN..."
	@test -f "$(zBGC_VERIFY_FQIN_FILE)" || ($(MBC_SEE_RED) "Error: Could not find FQIN in build output" && false)
	@$(MBC_SEE_YELLOW) "Built container FQIN: $(zBGC_VERIFY_FQIN_CONTENTS)"
	@test -z "$(BGC_ARG_FQIN_OUTPUT)" || cp "$(zBGC_VERIFY_FQIN_FILE)"   "$(BGC_ARG_FQIN_OUTPUT)"
	@test -z "$(BGC_ARG_FQIN_OUTPUT)" || $(MBC_SEE_YELLOW) "Wrote FQIN to $(BGC_ARG_FQIN_OUTPUT)"
	$(MBC_STEP) "Pull logs..."
	@$(zBGC_CMD_GET_LOGS) > $(zBGC_TEMP_DIR)/workflow_logs__$(MBC_NOW).txt
	$(MBC_STEP) "Everything went right, delete the run cache..."
	rm $(zBGC_CURRENT_WORKFLOW_RUN_CACHE)
	$(MBC_PASS) "No errors."


# TOKEN AUTH FAIL INSCRUTABILITY:
#
# $ tt/bgc-l.ListCurrentRegistryImages.sh
# rbc-console.mk: List Current Registry Images
# rbc-console.mk: DEBUG: raw listing...
# curl -s -H 'Authorization: token ghp_xxxx' -H 'Accept: application/vnd.github.v3+json' 'https://api.github.com/user/packages?package_type=container'
# {
#   "message": "Bad credentials",
#   "documentation_url": "https://docs.github.com/rest",
#   "status": "401"
# }
# rbc-console.mk: JQ execution...
# assertion "cb == jq_util_input_next_input_cb" failed: file "/cygdrive/d/a/scallywag/jq/jq-1.7.1-1.x86_64/src/jq-1.7.1/src/util.c", line 360, function: jq_util_input_get_position


bgc-l%: zbgc_argcheck_rule
	$(MBC_START) "List Current Registry Images"
	$(MBC_STEP) "JQ execution..."
	@$(zBGC_CMD_LIST_IMAGES)                                   |\
	  jq -r '.[] | select(.package_type=="container") | .name' |\
	  while read -r package_name; do                \
	    echo "Package: $$package_name";             \
	    $(MBC_SEE_YELLOW) "    https://github.com/$(BGCV_REGISTRY_OWNER)/$$package_name/pkgs/container/$$package_name"; \
	    echo "Versions:";                           \
	    $(zBGC_CMD_LIST_PACKAGE_VERSIONS)                                            |\
	      jq -r '.[] | "\(.metadata.container.tags[]) \(.id)"'                       |\
	      sort -r                                                                    |\
	      awk       '{printf "%-50s %-13s ghcr.io/$(BGCV_REGISTRY_OWNER)/$(BGCV_REGISTRY_NAME):%s\n", $$1, $$2, $$1}' |\
	      awk 'BEGIN {printf "%-50s %-13s %-70s\n", "Image Tag", "Version ID", "FQIN (Fully Qualified Image Name)"}1'; \
	    echo; \
	  done
	$(MBC_PASS) "No errors."


bgc-r%: zbgc_argcheck_rule
	$(MBC_START) "Retrieve Container Registry Image"
	@test "$(BGC_ARG_TAG)" != ""  ||\
	  ($(MBC_SEE_RED) "Error: Must say which image tag to retrieve" && false)
	$(MBC_STEP) "Log in to container registry..."
	@podman login ghcr.io -u $(BGCSV_USERNAME) -p $(BGCSV_PAT)
	$(MBC_STEP) "Fetch image..."
	podman pull $(BGC_ARG_TAG)
	$(MBC_PASS) "No errors."


bgc-d%: zbgc_argcheck_rule
	$(MBC_START) "Delete Container Registry Image"
	@test "$(BGC_ARG_TAG)" != ""  ||\
	  ($(MBC_SEE_RED) "Error: Must say which image tag to delete" && false)
	@echo "Deleting image with tag: $(BGC_ARG_TAG)"
	@echo "Fetching package version information..."
	@$(zBGC_CMD_LIST_PACKAGE_VERSIONS) |\
	  jq -r '.[] | select(.metadata.container.tags[] | contains("$(BGC_ARG_TAG)")) | .id' \
	       > $(zBGC_DELETE_VERSION_ID_CACHE)
	@test -s $(zBGC_DELETE_VERSION_ID_CACHE)  ||\
	  ($(MBC_SEE_RED) "Error: No version found for tag $(BGC_ARG_TAG)" && rm false)
	@echo "Found version ID:" $(zBGC_DELETE_VERSION_ID_CONTENTS) "for tag $(BGC_ARG_TAG)"
	@$(MBC_SEE_YELLOW) "Confirm delete image?" && read -p "Type YES: " confirm && test "$$confirm" = "YES"  ||\
	  ($(MBC_SEE_RED) "WONT DELETE" && false)
	$(MBC_STEP) "Sending delete request..."
	@curl -X DELETE $(zBGC_CURL_HEADERS) \
	  '$(zBGC_GITAPI_URL)/user/packages/container/$(BGCV_REGISTRY_NAME)/versions/'$(zBGC_DELETE_VERSION_ID_CONTENTS) \
	  -s -w "HTTP_STATUS:%{http_code}\n" > $(zBGC_DELETE_RESULT_CACHE)
	@echo "Delete response:" $(zBGC_DELETE_RESULT_CONTENTS)
	@grep -q "HTTP_STATUS:204" $(zBGC_DELETE_RESULT_CACHE) ||\
	  ($(MBC_SEE_RED) "Failed to delete image version. HTTP Status:" $(zBGC_DELETE_RESULT_CONTENTS)  &&  false)
	@echo "Successfully deleted image version."
	@rm $(zBGC_DELETE_VERSION_ID_CACHE) $(zBGC_DELETE_RESULT_CACHE)
	$(MBC_PASS) "No errors."



# eof
