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

RBT_JUPYTER_URL = http://localhost:$(RBRN_ENTRY_PORT_WORKSTATION)/lab
RBT_JUPYTER_API = http://localhost:$(RBRN_ENTRY_PORT_WORKSTATION)/api
RBT_XSRF_TOKEN_FILE = $(MBD_TEMP_DIR)/jupyter_xsrf_token

rbt_test_bottle_service_rule:
	$(MBC_SHOW_WHITE) "COLLECT INFORMATION HELPFUL IN DEBUGGING..."
	$(MBC_SHOW_WHITE) "   fact: RBM_SENTRY_CONTAINER        is $(RBM_SENTRY_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBM_BOTTLE_CONTAINER        is $(RBM_BOTTLE_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBRN_ENTRY_PORT_WORKSTATION is $(RBRN_ENTRY_PORT_WORKSTATION)"
	$(MBC_SHOW_WHITE) "   fact: RBRN_ENTRY_PORT_ENCLAVE     is $(RBRN_ENTRY_PORT_ENCLAVE)"
	$(MBC_SHOW_WHITE) "   fact: MBD_TEMP_DIR                is $(MBD_TEMP_DIR)"
	$(MBC_SHOW_WHITE) "   fact: RBT_TESTS_DIR               is $(RBT_TESTS_DIR)"

	$(MBC_SHOW_WHITE) "Verify Jupyter process is running in bottle"
	$(MBT_PODMAN_EXEC_BOTTLE) ps aux | grep jupyter

	$(MBC_SHOW_WHITE) "Watch network traffic during curl attempt"
	$(MBT_PODMAN_EXEC_SENTRY) tcpdump -n -i eth0 port $(RBRN_ENTRY_PORT_WORKSTATION) & sleep 1
	curl -v --connect-timeout 5 --max-time 10 $(RBT_JUPYTER_URL) || true
	sleep 2
	$(MBT_PODMAN_EXEC_SENTRY) pkill tcpdump

	$(MBC_SHOW_WHITE) "Try curl with browser-like headers"
	curl -v -H "User-Agent: Mozilla/5.0" -H "Accept: text/html,application/xhtml+xml" \
	  --connect-timeout 5 --max-time 10 $(RBT_JUPYTER_URL) | grep "JupyterLab"
	@echo curl done

	$(MBC_SHOW_WHITE) "Get and cache XSRF token"
	curl -I $(RBT_JUPYTER_URL) | grep -i "set-cookie" | grep "_xsrf" | cut -d= -f2 | cut -d\; -f1 > $(RBT_XSRF_TOKEN_FILE)
	@echo curl done

	$(MBC_SHOW_WHITE) "Attempt to create a Jupyter session"
	curl -X POST $(RBT_JUPYTER_API)/sessions               \
	  -H "Content-Type: application/json"                  \
	  -H "X-XSRFToken: $$(cat $(RBT_XSRF_TOKEN_FILE))"     \
	  -H "Cookie: _xsrf=$$(cat $(RBT_XSRF_TOKEN_FILE))"    \
	  -d '{"kernel":{"name":"python3"},"name":"test.ipynb","path":"test.ipynb","type":"notebook"}'
	@echo curl done

	$(MBC_SHOW_WHITE) "List active kernels"
	curl -X GET "$(RBT_JUPYTER_API)/kernels"                   \
		-H "X-XSRFToken: $$(cat $(RBT_XSRF_TOKEN_FILE))"   \
		-H "Cookie: _xsrf=$$(cat $(RBT_XSRF_TOKEN_FILE))"
	@echo curl done

	$(MBC_SHOW_WHITE) "Cleaning up any existing sessions and kernels"
	curl -X DELETE "$(RBT_JUPYTER_API)/sessions"                        \
	    -H "X-XSRFToken: $$(cat $(RBT_XSRF_TOKEN_FILE))"                \
	    -H "Cookie: _xsrf=$$(cat $(RBT_XSRF_TOKEN_FILE))"
	@echo curl done

	$(MBC_SHOW_WHITE) "Confirm cleanup of sessions and kernels"
	curl -X GET "$(RBT_JUPYTER_API)/kernels"                            \
	    -H "X-XSRFToken: $$(cat $(RBT_XSRF_TOKEN_FILE))"                \
	    -H "Cookie: _xsrf=$$(cat $(RBT_XSRF_TOKEN_FILE))"               \
	    | jq -r '.[].id'                                                \
	    | xargs -I{} curl -X DELETE "$(RBT_JUPYTER_API)/kernels/{}"     \
	    -H "X-XSRFToken: $$(cat $(RBT_XSRF_TOKEN_FILE))"                \
	    -H "Cookie: _xsrf=$$(cat $(RBT_XSRF_TOKEN_FILE))"
	@echo curl done

	$(MBC_SHOW_WHITE) "Running Python Jupyter test using test container"
	$(MBT_PODMAN_BASE) run --rm -i                                           \
	  --network host                                                         \
	  -e RBRN_ENTRY_PORT_WORKSTATION=$(RBRN_ENTRY_PORT_WORKSTATION)          \
	  ghcr.io/bhyslop/recipemuster:rbtest_python_networking.20250215__171409 \
	  python3 - < $(RBT_TESTS_DIR)/rbt.test.srjcl.py

	$(MBC_PASS) "No errors detected."


# eof
