# Configuration: where to find the Makefile Bash Console declarations
include $(RBT_MBC_MAKEFILE)

RBT_JUPYTER_URL = http://localhost:$(RBN_ENTRY_PORT_WORKSTATION)/lab
RBT_JUPYTER_API = http://localhost:$(RBN_ENTRY_PORT_WORKSTATION)/api
RBT_XSRF_TOKEN_FILE = $(RBT_TEMP_DIR)/jupyter_xsrf_token

rbt_test_bottle_service_rule:
	$(MBC_SHOW_WHITE) "COLLECT INFORMATION HELPFUL IN DEBUGGING..."
	$(MBC_SHOW_WHITE) "   fact: RBM_SENTRY_CONTAINER       is $(RBM_SENTRY_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBM_BOTTLE_CONTAINER       is $(RBM_BOTTLE_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBN_ENCLAVE_SENTRY_IP      is $(RBN_ENCLAVE_SENTRY_IP)"
	$(MBC_SHOW_WHITE) "   fact: RBN_ENTRY_PORT_WORKSTATION is $(RBN_ENTRY_PORT_WORKSTATION)"
	$(MBC_SHOW_WHITE) "   fact: RBT_TEMP_DIR               is $(RBT_TEMP_DIR)"

	$(MBC_SHOW_WHITE) "Verify Jupyter process is running in bottle"
	podman exec $(RBM_BOTTLE_CONTAINER) ps aux | grep jupyter

	$(MBC_SHOW_WHITE) "Watch network traffic during curl attempt"
	podman exec $(RBM_SENTRY_CONTAINER) tcpdump -n -i eth0 port $(RBN_ENTRY_PORT_WORKSTATION) & sleep 1
	curl -v --connect-timeout 5 --max-time 10 $(RBT_JUPYTER_URL) || true
	sleep 2
	podman exec $(RBM_SENTRY_CONTAINER) pkill tcpdump

	$(MBC_SHOW_WHITE) "Try curl with browser-like headers"
	curl -v -H "User-Agent: Mozilla/5.0" -H "Accept: text/html,application/xhtml+xml" \
	  --connect-timeout 5 --max-time 10 $(RBT_JUPYTER_URL) | grep "JupyterLab"

	$(MBC_SHOW_WHITE) "Get and cache XSRF token"
	curl -I $(RBT_JUPYTER_URL) | grep -i "set-cookie" | grep "_xsrf" | cut -d= -f2 | cut -d\; -f1 > $(RBT_XSRF_TOKEN_FILE)

	$(MBC_SHOW_WHITE) "Attempt to create a Jupyter session"
	curl -X POST $(RBT_JUPYTER_API)/sessions               \
	  -H "Content-Type: application/json"                  \
	  -H "X-XSRFToken: $$(cat $(RBT_XSRF_TOKEN_FILE))"     \
	  -H "Cookie: _xsrf=$$(cat $(RBT_XSRF_TOKEN_FILE))"    \
	  -d '{"kernel":{"name":"python3"},"name":"test.ipynb","path":"test.ipynb","type":"notebook"}'

	$(MBC_SHOW_WHITE) "List active kernels"
	curl -X GET "$(RBT_JUPYTER_API)/kernels" \
		-H "X-XSRFToken: $$(cat $(RBT_XSRF_TOKEN_FILE))" \
		-H "Cookie: _xsrf=$$(cat $(RBT_XSRF_TOKEN_FILE))"

	$(MBC_SHOW_WHITE) "Create and test kernel using WebSocket"

	$(MBC_SHOW_WHITE) "Request kernel ID in temp file"
	podman exec $(RBM_SENTRY_CONTAINER) curl -s -X POST \
		"http://$(RBN_ENCLAVE_SENTRY_IP):$(RBN_ENTRY_PORT_WORKSTATION)/api/kernels" \
		-H "Content-Type: application/json" \
		-d '{"name":"python3"}' \
		> $(RBT_TEMP_DIR)/kernel_response.json

	$(MBC_SHOW_WHITE) "Store kernel ID in temp file"
	podman exec $(RBM_SENTRY_CONTAINER) jq -r .id \
		< $(RBT_TEMP_DIR)/kernel_response.json \
		> $(RBT_TEMP_DIR)/kernel_id

	$(MBC_SHOW_WHITE) "Send WebSocket message to list Python packages"
	podman exec $(RBM_SENTRY_CONTAINER) bash -c 'echo "{ \
		\"header\": { \
			\"msg_id\": \"$$(cat /proc/sys/kernel/random/uuid)\", \
			\"msg_type\": \"execute_request\", \
			\"username\": \"test\", \
			\"session\": \"$$(cat /proc/sys/kernel/random/uuid)\", \
			\"date\": \"$$(date -u +"%Y-%m-%dT%H:%M:%S.%3NZ")\" \
		}, \
		\"parent_header\": {}, \
		\"metadata\": {}, \
		\"content\": { \
			\"code\": \"import pkg_resources; list(pkg_resources.working_set)\", \
			\"silent\": false, \
			\"store_history\": false, \
			\"user_expressions\": {}, \
			\"allow_stdin\": false \
		}, \
		\"channel\": \"shell\" \
	}" | websocat "ws://$(RBN_ENCLAVE_SENTRY_IP):$(RBN_ENTRY_PORT_WORKSTATION)/api/kernels/$$(cat $(RBT_TEMP_DIR)/kernel_id)/channels" \
		| jq "select(.msg_type == \"execute_result\") | .content.data.\"text/plain\""'

	# Cleanup kernel
	podman exec $(RBM_SENTRY_CONTAINER) curl -s -X DELETE \
		"http://$(RBN_ENCLAVE_SENTRY_IP):$(RBN_ENTRY_PORT_WORKSTATION)/api/kernels/$$(cat $(RBT_TEMP_DIR)/kernel_id)"

	$(MBC_PASS) "No errors detected."


# eof
