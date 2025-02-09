
# Configuration: where to find the Makefile Bash Console declarations
include $(RBT_MBC_MAKEFILE)

RBT_JUPYTER_URL = http://localhost:8000/lab

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

	$(MBC_SHOW_WHITE) "Attempt to create a Jupyter session"
	XSRF_TOKEN=$$(curl -I http://localhost:8000/lab | grep -i "set-cookie" | grep "_xsrf" | cut -d= -f2 | cut -d\; -f1) && \
	  SESSION_RESPONSE=$$(curl -X POST http://localhost:8000/api/sessions \
	    -H "Content-Type: application/json" \
	    -H "X-XSRFToken: $$XSRF_TOKEN" \
	    -H "Cookie: _xsrf=$$XSRF_TOKEN" \
	    -d '{"kernel":{"name":"python3"},"name":"test.ipynb","path":"test.ipynb","type":"notebook"}') && \
	  echo "Session creation response: $$SESSION_RESPONSE"  && \
	  echo "List active kernels"  && \
	  curl -X GET "http://localhost:8000/api/kernels" \
	    -H "X-XSRFToken: $$XSRF_TOKEN" \
	    -H "Cookie: _xsrf=$$XSRF_TOKEN"; echo "SAW EXIT STATUS $$?"
	$(MBC_SHOW_WHITE) "PASS"

# eof
