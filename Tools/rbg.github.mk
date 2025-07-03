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

# Path to the Dockerfile recipe to be built. Must be a file containing valid Dockerfile
# instructions. The basename must not contain uppercase letters.
# Valid for: Build operations only
RBG_ARG_RECIPE                   ?=

# Optional output file path where the Fully Qualified Image Name (FQIN) of the built 
# container will be written. If not specified, the FQIN is only displayed in the console.
# Valid for: Build operations only
RBG_ARG_FQIN_OUTPUT              ?=

# Fully Qualified Image Name (FQIN) of the container image to operate on, typically 
# in the format ghcr.io/owner/name:tag. Required for image deletion operations.
# Valid for: Delete operations only
RBG_ARG_FQIN                     ?=

# Set to "SKIP" to bypass the interactive confirmation prompt when deleting container
# images. If not set, user must type "YES" to confirm deletion.
# Valid for: Delete operations only
RBG_ARG_SKIP_DELETE_CONFIRMATION ?=

zRBG_GIT_REGISTRY := ghcr.io

zRBG_GITAPI_URL := https://api.github.com

zRBG_CURRENT_WORKFLOW_RUN_CACHE    = $(MBD_TEMP_DIR)/CURR_WORKFLOW_RUN__$(MBD_NOW_STAMP).txt
zRBG_CURRENT_WORKFLOW_RUN_CONTENTS = $$(cat $(zRBG_CURRENT_WORKFLOW_RUN_CACHE))

zRBG_DELETE_VERSION_ID_CACHE    = $(MBD_TEMP_DIR)/RBG_VERSION_ID__$(MBD_NOW_STAMP).txt
zRBG_DELETE_VERSION_ID_CONTENTS = $$(cat $(zRBG_DELETE_VERSION_ID_CACHE))

zRBG_DELETE_RESULT_CACHE    = $(MBD_TEMP_DIR)/RBG_DELETE__$(MBD_NOW_STAMP).txt
zRBG_DELETE_RESULT_CONTENTS = $$(cat $(zRBG_DELETE_RESULT_CACHE))

zRBG_RECIPE_BASENAME  = $(shell basename $(RBG_ARG_RECIPE))

define get_latest_build_dir
$(shell find $(RBRR_HISTORY_DIR) -name "$(basename $(notdir $(RBG_ARG_RECIPE)))*" -type d -print | sort -r | head -n1)
endef

zRBG_REPO_PREFIX = $(zRBG_GITAPI_URL)/repos/$(RBRR_REGISTRY_OWNER)/$(RBRR_REGISTRY_NAME)

zRBG_CURL_HEADERS = -H "Authorization: token $$RBV_PAT" \
                    -H 'Accept: application/vnd.github.v3+json'

zRBG_CMD_TRIGGER_BUILD = source $(RBRR_GITHUB_PAT_ENV) && curl -X POST $(zRBG_CURL_HEADERS) \
    '$(zRBG_REPO_PREFIX)/dispatches' \
    -d '{"event_type": "build_containers", "client_payload": {"dockerfile": "$(RBG_ARG_RECIPE)"}}'

zRBG_CMD_GET_WORKFLOW_RUN = source $(RBRR_GITHUB_PAT_ENV) && curl -s $(zRBG_CURL_HEADERS) \
    '$(zRBG_REPO_PREFIX)/actions/runs?event=repository_dispatch&branch=main&per_page=1'

zRBG_CMD_GET_SPECIFIC_RUN = source $(RBRR_GITHUB_PAT_ENV) && curl -s  $(zRBG_CURL_HEADERS) \
    '$(zRBG_REPO_PREFIX)/actions/runs/'$(zRBG_CURRENT_WORKFLOW_RUN_CONTENTS)

zRBG_CMD_LIST_IMAGES = source $(RBRR_GITHUB_PAT_ENV) && curl -s $(zRBG_CURL_HEADERS) \
    '$(zRBG_GITAPI_URL)/user/packages?package_type=container'

zRBG_CMD_LIST_PACKAGE_VERSIONS = source $(RBRR_GITHUB_PAT_ENV) && curl -s $(zRBG_CURL_HEADERS) \
    '$(zRBG_GITAPI_URL)/user/packages/container/$(RBRR_REGISTRY_NAME)/versions?per_page=100'

zRBG_CMD_GET_LOGS = $(zRBG_CMD_GET_SPECIFIC_RUN)/logs

zRBG_CMD_DELETE_VERSION = source $(RBRR_GITHUB_PAT_ENV) && curl -X DELETE -s $(zRBG_CURL_HEADERS) \
    '$(zRBG_GITAPI_URL)/user/packages/container/$(RBRR_REGISTRY_NAME)/versions/'$(zRBG_DELETE_VERSION_ID_CONTENTS)

zbgc_argcheck_rule: rbrr_validate
	@test -f "$(RBRR_GITHUB_PAT_ENV)" || ($(MBC_SEE_RED) "Error: GitHub PAT env file not found at $(RBRR_GITHUB_PAT_ENV)" && false)
	@(source $(RBRR_GITHUB_PAT_ENV) && test -n "$$RBV_PAT" || ($(MBC_SEE_RED) "Error: RBV_PAT missing" && false))
	@test -n "$(zRBG_GITAPI_URL)" || ($(MBC_SEE_RED) "Error: zRBG_GITAPI_URL unset" && false)


zbgc_recipe_argument_check:
	$(MBC_START) "Checking recipe argument"
	@test -n "$(RBG_ARG_RECIPE)" || ($(MBC_SEE_RED) "Error: RBG_ARG_RECIPE unset" && exit 1)
	@test -f "$(RBG_ARG_RECIPE)" || ($(MBC_SEE_RED) "Error: '$(RBG_ARG_RECIPE)' is not a file" && exit 1)
	@! basename "$(RBG_ARG_RECIPE)" | grep -q '[A-Z]' || \
	  ($(MBC_SEE_RED) "Error: Basename of '$(RBG_ARG_RECIPE)' contains uppercase letters" && exit 1)
	@$(MBC_STEP) "$(RBG_ARG_RECIPE) is well formed, moving on..."


zRBG_COLLECT_TEMP_PAGE   = $(MBD_TEMP_DIR)/RBG_PAGE__$(MBD_NOW_STAMP).json
zRBG_COLLECT_FULL_JSON   = $(MBD_TEMP_DIR)/RBG_COMBINED__$(MBD_NOW_STAMP).json

zRBG_CMD_COLLECT_PAGED = source $(RBRR_GITHUB_PAT_ENV) && curl -s $(zRBG_CURL_HEADERS) \
    '$(zRBG_GITAPI_URL)/user/packages/container/$(RBRR_REGISTRY_NAME)/versions?per_page=100&page='

zbgc_collect_rule: zbgc_argcheck_rule
	$(MBC_START) "Fetching all registry images with pagination to" $(zRBG_COLLECT_FULL_JSON)
	@echo "[]" > $(zRBG_COLLECT_FULL_JSON)
	$(MBC_STEP) "Retrieving paged results..."
	@page=1                                                                  &&\
	while true; do                                                             \
	  echo "  Fetching page $$page..."                                       &&\
	  $(zRBG_CMD_COLLECT_PAGED)$$page > $(zRBG_COLLECT_TEMP_PAGE)            &&\
	  echo "  Counting items on page $$page..."                              &&\
	  items=$$(jq '. | length' $(zRBG_COLLECT_TEMP_PAGE))                    &&\
	  echo "  Saw $$items items on page $$page..." || exit 10;                 \
	  test $$items -ne 0 || break;                                             \
	  echo "  Appending page $$page to combined JSON..."                     &&\
	  jq -s '.[0] + .[1]' $(zRBG_COLLECT_FULL_JSON) $(zRBG_COLLECT_TEMP_PAGE) >\
	                      $(zRBG_COLLECT_FULL_JSON).tmp                      &&\
	  mv $(zRBG_COLLECT_FULL_JSON).tmp $(zRBG_COLLECT_FULL_JSON)             &&\
	  echo "  Updating page counter..."                                      &&\
	  page=$$((page + 1));                                                     \
	done
	$(MBC_STEP) "Concluding..."
	@echo "  Retrieved" $$(jq '. | length' $(zRBG_COLLECT_FULL_JSON)) "total items"
	$(MBC_PASS) "Pagination complete."


rbg-b.%: zbgc_argcheck_rule zbgc_recipe_argument_check
	$(MBC_START) "Trigger Build of $(RBG_ARG_RECIPE)"
	$(MBC_STEP) "Make sure your local repo is up to date with github variant..."
	@git fetch                                               &&\
	  git status -uno | grep -q 'Your branch is up to date'  &&\
	  git diff-index --quiet HEAD --                         &&\
	  true || ($(MBC_SEE_RED) "ERROR: Your repo is not cleanly aligned with github variant." &&\
	           $(MBC_SEE_RED) "       Commit or otherwise match to proceed (prevents merge"  &&\
		   $(MBC_SEE_RED) "       conflicts with image history tracking)." && false)
	@$(zRBG_CMD_TRIGGER_BUILD)
	$(MBC_STEP) "Pausing for GitHub to process the dispatch event..."
	@sleep 5
	$(MBC_STEP) "Retrieve workflow run ID..."
	@$(zRBG_CMD_GET_WORKFLOW_RUN) | jq -r '.workflow_runs[0].id' > $(zRBG_CURRENT_WORKFLOW_RUN_CACHE)
	@test -s                                                       $(zRBG_CURRENT_WORKFLOW_RUN_CACHE)
	$(MBC_STEP) "Workflow online at:"
	$(MBC_SHOW_YELLOW) "   https://github.com/$(RBRR_REGISTRY_OWNER)/$(RBRR_REGISTRY_NAME)/actions/runs/"$(zRBG_CURRENT_WORKFLOW_RUN_CONTENTS)
	$(MBC_STEP) "Polling to completion..."
	@status="" && conclusion="" &&  \
	while true; do                                            \
	  response=$$($(zRBG_CMD_GET_SPECIFIC_RUN));              \
	  echo "  TRACE: $$response" | grep -i -e "status" -e "conclusion" -e "TRACE"; \
	  status=$$(echo     "$$response" | jq -r '.status');     \
	  conclusion=$$(echo "$$response" | jq -r '.conclusion'); \
	  echo "  Status: $$status    Conclusion: $$conclusion";  \
	  test           "$$status" != "completed" || break;      \
	  sleep 3;                                                \
	done; test "$$conclusion" == "success" || ($(MBC_SEE_RED) "Error: Workflow fail: $$conclusion" && false)
	$(MBC_STEP) "Git Pull for artifacts with retry..."
	@for i in 9 8 7 6 5 4 3 2 1 0; do \
	  echo "  Attempt $$i: Checking for remote changes..."; \
	  git fetch --quiet; \
	  if [ $$(git rev-list --count HEAD..origin/main 2>/dev/null) -gt 0 ]; then \
	    echo "  Found new commits, pulling..."; \
	    git pull; \
	    echo "  Pull successful, breaking loop"; \
	    break; \
	  fi; \
	  echo "  No new commits yet, waiting 5 seconds (attempt $$i)"; \
	  [ $$i -eq 0 ] && ($(MBC_SEE_RED) "Error: No new commits after many attempts" && false); \
	  sleep 5; \
	done
	$(MBC_STEP) "Verifying build output..."
	@echo "MissingBuidDirDebug: RBRR_HISTORY_DIR = $(RBRR_HISTORY_DIR)"
	@echo "MissingBuidDirDebug: Recipe basename = $(basename $(zRBG_RECIPE_BASENAME))"
	@echo "MissingBuidDirDebug: Search pattern = $(RBRR_HISTORY_DIR)/$(basename $(zRBG_RECIPE_BASENAME))*"
	@echo "MissingBuidDirDebug: Found directories:"
	@ls -td $(RBRR_HISTORY_DIR)/$(basename $(zRBG_RECIPE_BASENAME))* 2>/dev/null || echo "  None found with pattern"
	@echo "DEBUG: About to call get_latest_build_dir function"
	@echo "DEBUG: Function would execute: find $(RBRR_HISTORY_DIR) -name \"$(basename $(notdir $(RBG_ARG_RECIPE)))*\" -type d -print | sort -r | head -n1"
	@echo "DEBUG: Current working directory: $$(pwd)"
	@echo "DEBUG: Testing find command directly:"
	@find $(RBRR_HISTORY_DIR) -name "$(basename $(notdir $(RBG_ARG_RECIPE)))*" -type d -print | sort -r | head -n1
	@echo "DEBUG: Storing result in temp file..."
	@find $(RBRR_HISTORY_DIR) -name "$(basename $(notdir $(RBG_ARG_RECIPE)))*" -type d -print | sort -r | head -n1 > $(MBD_TEMP_DIR)/build_dir.txt
	@echo "DEBUG: Temp file contents:"
	@cat $(MBD_TEMP_DIR)/build_dir.txt
	@echo "MissingBuidDirDebug: Selected build directory = $$(cat $(MBD_TEMP_DIR)/build_dir.txt | xargs)"
	@echo "DEBUG: Checking for FQIN file: $$(cat $(MBD_TEMP_DIR)/build_dir.txt | xargs)/docker_inspect_RepoTags_0.txt"
	@test -s "$(MBD_TEMP_DIR)/build_dir.txt" || ($(MBC_SEE_RED) "Error: Missing build directory - No directory found matching pattern '$(RBRR_HISTORY_DIR)/$(basename $(zRBG_RECIPE_BASENAME))*'" && false)
	@test -d "$$(cat $(MBD_TEMP_DIR)/build_dir.txt | xargs)" || ($(MBC_SEE_RED) "Error: Build directory '$$(cat $(MBD_TEMP_DIR)/build_dir.txt | xargs)' is not a valid directory" && false)
	@echo "MissingBuidDirDebug: Comparing recipes:"
	@echo "  Source recipe: $(RBG_ARG_RECIPE)"
	@echo "  Build recipe: $$(cat $(MBD_TEMP_DIR)/build_dir.txt | xargs)/recipe.txt"
	@test -f "$$(cat $(MBD_TEMP_DIR)/build_dir.txt | xargs)/recipe.txt" || ($(MBC_SEE_RED) "Error: recipe.txt not found in $$(cat $(MBD_TEMP_DIR)/build_dir.txt | xargs)" && false)
	@echo "MissingBuidDirDebug: end"
	@cmp "$(RBG_ARG_RECIPE)" "$$(cat $(MBD_TEMP_DIR)/build_dir.txt | xargs)/recipe.txt" || ($(MBC_SEE_RED) "Error: recipe mismatch" && false)
	$(MBC_STEP) "Extracting FQIN..."
	@test -f "$$(cat $(MBD_TEMP_DIR)/build_dir.txt | xargs)/docker_inspect_RepoTags_0.txt" || ($(MBC_SEE_RED) "Error: Could not find FQIN in build output" && false)
	@fqin_contents=$$(cat "$$(cat $(MBD_TEMP_DIR)/build_dir.txt | xargs)/docker_inspect_RepoTags_0.txt")
	@$(MBC_SEE_YELLOW) "Built container FQIN: $$fqin_contents"
	@test -z "$(RBG_ARG_FQIN_OUTPUT)" || cp "$$(cat $(MBD_TEMP_DIR)/build_dir.txt | xargs)/docker_inspect_RepoTags_0.txt" "$(RBG_ARG_FQIN_OUTPUT)"
	@test -z "$(RBG_ARG_FQIN_OUTPUT)" || $(MBC_SEE_YELLOW) "Wrote FQIN to $(RBG_ARG_FQIN_OUTPUT)"

	$(MBC_STEP) "Verifying image availability in registry..."
	@tag=$$(echo "$$(cat "$$(cat $(MBD_TEMP_DIR)/build_dir.txt | xargs)/docker_inspect_RepoTags_0.txt")" | cut -d: -f2)
	@echo "  Waiting for tag: $$tag to become available..."
	@for i in 1 2 3 4 5; do \
	  $(zRBG_CMD_LIST_PACKAGE_VERSIONS) | jq -e '.[] | select(.metadata.container.tags[] | contains("'$$tag'"))' > /dev/null && break; \
	  echo "  Image not yet available, attempt $$i of 5"; \
	  [ $$i -eq 5 ] && ($(MBC_SEE_RED) "Error: Image '$$tag' not available in registry after 5 attempts" && false); \
	  sleep 5; \
	done
	$(MBC_STEP) "Pull logs..."
	@$(zRBG_CMD_GET_LOGS) > $(MBD_TEMP_DIR)/workflow_logs__$(MBD_NOW_STAMP).txt
	$(MBC_STEP) "Everything went right, delete the run cache..."
	rm $(zRBG_CURRENT_WORKFLOW_RUN_CACHE)
	$(MBC_PASS) "No errors."


rbg-l.%: zbgc_argcheck_rule zbgc_collect_rule
	$(MBC_START) "List Current Registry Images"
	$(MBC_STEP) "Processing collected JSON data..."
	@echo "Package: $(RBRR_REGISTRY_NAME)"
	$(MBC_SHOW_YELLOW) "    https://github.com/$(RBRR_REGISTRY_OWNER)/$(RBRR_REGISTRY_NAME)/pkgs/container/$(RBRR_REGISTRY_NAME)"
	@echo "Versions:"
	@printf "%-13s %-70s\n" "Version ID" "Fully Qualified Image Name"
	@jq -r '.[] | select(.metadata.container.tags | length > 0) | .id as $$id | .metadata.container.tags[] as $$tag | [$$id, "$(zRBG_GIT_REGISTRY)/$(RBRR_REGISTRY_OWNER)/$(RBRR_REGISTRY_NAME):" + $$tag] | @tsv' \
	  $(zRBG_COLLECT_FULL_JSON) | sort -k2 -r | while read id tag; do printf "%-13s %s\n" "$$id" "$$tag"; done
	@echo "$(MBC_RESET)"
	$(MBC_STEP) "Total image versions: $$(jq '[.[] | select(.metadata.container.tags | length > 0) | .metadata.container.tags | length] | add // 0' $(zRBG_COLLECT_FULL_JSON))"
	$(MBC_PASS) "No errors."


rbg_container_registry_login_rule: zbgc_argcheck_rule
	$(MBC_START) "Log in to container registry"
	source $(RBRR_GITHUB_PAT_ENV)  && \
	  podman $(RBM_CONNECTION) login $(zRBG_GIT_REGISTRY) -u $$RBV_USERNAME -p $$RBV_PAT
	$(MBC_PASS) "No errors."


rbg-r.%: rbg_container_registry_login_rule
	$(MBC_START) "Retrieve Container Registry Image"
	@test "$(RBG_ARG_TAG)" != "" || ($(MBC_SEE_RED) "Error: Which container FQIN?" && false)
	$(MBC_STEP) "Fetch image..."
	podman $(RBM_CONNECTION) pull $(RBG_ARG_TAG)
	$(MBC_PASS) "No errors."


zRBG_TAG_CACHE    = $(MBD_TEMP_DIR)/RBG_TAG__$(MBD_NOW_STAMP).txt
zRBG_TAG_CONTENTS = $$(cat $(zRBG_TAG_CACHE))

rbg-d.%: zbgc_argcheck_rule zbgc_collect_rule
	$(MBC_START) "Delete Container Registry Image"
	@test "$(RBG_ARG_FQIN)" != "" || \
	  ($(MBC_SEE_RED) "Error: Must provide FQIN of image to delete (RBG_ARG_FQIN)" && false)
	@echo "Deleting image: $(RBG_ARG_FQIN)"
	@echo "Extracting tag from FQIN..."
	@echo "$(RBG_ARG_FQIN)" | sed 's/.*://' > $(zRBG_TAG_CACHE)
	@echo "Using tag: '$(zRBG_TAG_CONTENTS)'"
	@test -n "$(zRBG_TAG_CONTENTS)" || ($(MBC_SEE_RED) "Error: Could not extract a valid tag from FQIN $(RBG_ARG_FQIN)" && false)
	
	@echo "DEBUG: Collecting tag and ID mapping..."
	@jq -r '.[] | select(.metadata.container.tags != null) | .id as $$id | .metadata.container.tags[] as $$tag | [$$id, $$tag] | @tsv' $(zRBG_COLLECT_FULL_JSON) > $(MBD_TEMP_DIR)/all_tags_with_ids.txt
	
	@echo "DEBUG: Searching for tag '$(zRBG_TAG_CONTENTS)' in extracted mapping..."
	@grep "$(zRBG_TAG_CONTENTS)$$" $(MBD_TEMP_DIR)/all_tags_with_ids.txt | cut -f1 > $(zRBG_DELETE_VERSION_ID_CACHE) || true
	
	@match_count=$$(wc -l < $(zRBG_DELETE_VERSION_ID_CACHE) | tr -d ' '); \
	echo "DEBUG: Found $$match_count exact matching version(s)"; \
	echo "DEBUG: Matching version IDs:"; \
	cat $(zRBG_DELETE_VERSION_ID_CACHE); \
	test "$$match_count" -eq 1 || ($(MBC_SEE_RED) "Error: Expected exactly 1 matching version, found $$match_count" && rm $(zRBG_DELETE_VERSION_ID_CACHE) && false)
	
	@echo "Found version ID: $(zRBG_DELETE_VERSION_ID_CONTENTS)"
	@test "$(RBG_ARG_SKIP_DELETE_CONFIRMATION)" = "SKIP" || \
	  ($(MBC_SEE_YELLOW) "Confirm delete image?" && \
	  read -p "Type YES: " confirm && \
	  (test "$$confirm" = "YES" || \
	  ($(MBC_SEE_RED) "WONT DELETE" && false)))
	$(MBC_STEP) "Deleting image version..."
	@$(zRBG_CMD_DELETE_VERSION) -s -w "\nHTTP_STATUS:%{http_code}\n" > $(zRBG_DELETE_RESULT_CACHE) 2>&1
	@grep -q "HTTP_STATUS:204" $(zRBG_DELETE_RESULT_CACHE) || \
	  ($(MBC_SEE_RED) "Failed to delete image version. HTTP Status: $$(grep 'HTTP_STATUS' $(zRBG_DELETE_RESULT_CACHE) || echo 'unknown')" && \
	   rm $(zRBG_DELETE_VERSION_ID_CACHE) $(zRBG_DELETE_RESULT_CACHE) && false)
	@echo "Successfully deleted image version."
	$(MBC_PASS) "No errors."


# eof
