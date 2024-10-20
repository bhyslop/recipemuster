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
	@test -n "$(BGC_ARG_RECIPE)" || ($(MBC_SEE_RED) "Error: BGC_ARG_RECIPE unset" && false)
	@test -f "$(BGC_ARG_RECIPE)" || ($(MBC_SEE_RED) "Error: '$(BGC_ARG_RECIPE)' is not a file" && false)
	@basename "$(BGC_ARG_RECIPE)" | grep -q '[A-Z]' && \
	  ($(MBC_SEE_RED) "Error: Recipe '$(BGC_ARG_RECIPE)' cannot have uppercase" && false) || true


bgc-tb%: zbgc_argcheck_rule zbgc_recipe_argument_check
	$(MBC_START) "Trigger Build"
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
	until $(zBGC_CMD_QUERY_LAST_INNER); do sleep 3; done
	$(MBC_STEP) "Git Pull for artifacts..."
	git pull
	$(MBC_STEP) "Everything went right, delete the run cache..."
	rm $(zBGC_CURRENT_WORKFLOW_RUN_CACHE)
	$(MBC_PASS) "No errors."


bgc-lcri%: zbgc_argcheck_rule
	$(MBC_START) "List Current Registry Images"
	@$(zBGC_CMD_LIST_IMAGES)                                   |\
	  jq -r '.[] | select(.package_type=="container") | .name' |\
	  while read -r package_name; do                \
	    echo "Package: $$package_name";             \
	    $(MBC_SEE_YELLOW) "    https://github.com/$(BGCV_REGISTRY_OWNER)/$$package_name/pkgs/container/$$package_name"; \
	    echo "Versions:";                           \
	    $(zBGC_CMD_LIST_PACKAGE_VERSIONS)                                            |\
	      jq -r '.[] | "\(.metadata.container.tags[]) \(.id)"'                       |\
	      sort -r                                                                    |\
	      awk       '{printf "%-40s %-20s ghcr.io/$(BGCV_REGISTRY_OWNER)/$(BGCV_REGISTRY_NAME):%s\n", $$1, $$2, $$1}' |\
	      awk 'BEGIN {printf "%-40s %-20s %-70s\n", "Image Tag", "Version ID", "FQIN (Fully Qualified Image Name)"}1'; \
	    echo; \
	  done
	$(MBC_PASS) "No errors."


bgc-di%: zbgc_argcheck_rule
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


bgc-flbl%: zbgc_argcheck_rule
	$(MBC_START) "Fetch Last Build Logs"
	@$(zBGC_CMD_GET_LOGS) > $(zBGC_TEMP_DIR)/workflow_logs.zip
	$(MBC_PASS) "No errors."


# eof
