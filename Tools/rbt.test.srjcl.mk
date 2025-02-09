

rbt_test_bottle_service_rule:
	$(MBC_SHOW_WHITE) "COLLECT INFORMATION HELPFUL IN DEBUGGING..."
	$(MBC_SHOW_WHITE) "   fact: RBM_SENTRY_CONTAINER  is $(RBM_SENTRY_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBM_BOTTLE_CONTAINER  is $(RBM_BOTTLE_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBN_ENCLAVE_SENTRY_IP is $(RBN_ENCLAVE_SENTRY_IP)"

	$(MBC_SHOW_WHITE) "Verify Jupyter process is running in bottle"
	podman exec $(RBM_BOTTLE_CONTAINER) ps aux | grep jupyter

	$(MBC_SHOW_WHITE) "Watch network traffic during curl attempt"
	podman exec $(RBM_SENTRY_CONTAINER) tcpdump -n -i eth0 port 8000 & sleep 1
	curl -v --connect-timeout 5 --max-time 10 http://localhost:8000/lab || true
	sleep 2
	podman exec $(RBM_SENTRY_CONTAINER) pkill tcpdump

	$(MBC_SHOW_WHITE) "Try curl with browser-like headers"
	curl -v -H "User-Agent: Mozilla/5.0" -H "Accept: text/html,application/xhtml+xml" --connect-timeout 5 --max-time 10 http://localhost:8000/lab | grep "JupyterLab"

	$(MBC_SHOW_WHITE) "PASS"


# eof
