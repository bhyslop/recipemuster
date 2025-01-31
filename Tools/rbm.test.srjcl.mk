

rbm-t.TestRBM.srjcl.mk:
	$(MBC_SHOW_WHITE) "COLLECT INFORMATION HELPFUL IN DEBUGGING..."
	$(MBC_SHOW_WHITE) "   fact: RBM_SENTRY_CONTAINER  is $(RBM_SENTRY_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBM_BOTTLE_CONTAINER  is $(RBM_BOTTLE_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBN_ENCLAVE_SENTRY_IP is $(RBN_ENCLAVE_SENTRY_IP)"

	$(MBC_SHOW_WHITE) "Verify Jupyter process is running in bottle"
	podman exec $(RBM_BOTTLE_CONTAINER) ps aux | grep jupyter

	$(MBC_SHOW_WHITE) "Verify network interfaces in bottle"
	podman exec $(RBM_BOTTLE_CONTAINER) ip link show lo
	podman exec $(RBM_BOTTLE_CONTAINER) ip link show eth0

	$(MBC_SHOW_WHITE) "Waiting for Jupyter server full initialization"
	sleep 5

	$(MBC_SHOW_WHITE) "Verify Jupyter port is listening and attempt connection until success"
	for i in 1 2 3 4 5; do \
		$(MBC_SHOW_WHITE) "Attempt $$i: Checking if Jupyter is responding..." && \
		podman exec $(RBM_BOTTLE_CONTAINER) curl -s -o /dev/null -w "%{http_code}" http://localhost:8000/lab && break || \
		$(MBC_SHOW_WHITE) "Waiting for Jupyter to initialize..." && \
		sleep 5; \
	done

	$(MBC_SHOW_WHITE) "Test local connection to Jupyter in bottle"
	podman exec $(RBM_BOTTLE_CONTAINER) nc -zv localhost 8000

	$(MBC_SHOW_WHITE) "Verify Jupyter HTTP response from bottle"
	podman exec $(RBM_BOTTLE_CONTAINER) curl -s http://localhost:8000/lab | grep "JupyterLab"

	$(MBC_SHOW_WHITE) "Show port forwarding rules in sentry" 
	podman exec $(RBM_SENTRY_CONTAINER) iptables -t nat -L PREROUTING -n -v | grep 8000

	$(MBC_SHOW_WHITE) "Verify IP connectivity between sentry and bottle"
	podman exec $(RBM_SENTRY_CONTAINER) ping -c 2 $(RBN_ENCLAVE_BOTTLE_IP)

	$(MBC_SHOW_WHITE) "Test connectivity from sentry to bottle with retries"
	for i in 1 2 3; do \
		$(MBC_SHOW_WHITE) "Attempt $$i: Testing sentry to bottle connection..." && \
		podman exec $(RBM_SENTRY_CONTAINER) curl -s http://$(RBN_ENCLAVE_BOTTLE_IP):8000/lab | grep "JupyterLab" && break || \
		$(MBC_SHOW_WHITE) "Waiting for connection to establish..." && \
		sleep 3; \
	done

	$(MBC_SHOW_WHITE) "Wait for host port binding to stabilize"
	sleep 3

	$(MBC_SHOW_WHITE) "Verify host connectivity"
	podman machine ssh "netstat -tlpn | grep :8000"

	$(MBC_SHOW_WHITE) "Test host access to Jupyter with retries"
	for i in 1 2 3; do \
		$(MBC_SHOW_WHITE) "Attempt $$i: Testing host access..." && \
		curl -s --connect-timeout 5 --max-time 10 http://localhost:8000/lab | grep "JupyterLab" && break || \
		$(MBC_SHOW_WHITE) "Waiting for host connection to establish..." && \
		sleep 3; \
	done

	$(MBC_SHOW_WHITE) "PASS"


#eof
