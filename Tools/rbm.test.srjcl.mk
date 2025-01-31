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

	$(MBC_SHOW_WHITE) "Initial wait for Jupyter startup"
	sleep 15

	$(MBC_SHOW_WHITE) "Verify Jupyter port is listening"
	podman exec $(RBM_BOTTLE_CONTAINER) netstat -tuln | grep 8000

	$(MBC_SHOW_WHITE) "Test local connection to Jupyter in bottle"
	podman exec $(RBM_BOTTLE_CONTAINER) nc -zv localhost 8000

	$(MBC_SHOW_WHITE) "Verify Jupyter HTTP response in bottle"
	podman exec $(RBM_BOTTLE_CONTAINER) curl -s http://localhost:8000/lab | grep "JupyterLab"

	$(MBC_SHOW_WHITE) "Show port forwarding rules in sentry" 
	podman exec $(RBM_SENTRY_CONTAINER) iptables -t nat -L PREROUTING -n -v | grep 8000

	$(MBC_SHOW_WHITE) "Verify basic connectivity"
	podman exec $(RBM_SENTRY_CONTAINER) ping -c 2 $(RBN_ENCLAVE_BOTTLE_IP)

	$(MBC_SHOW_WHITE) "Test sentry to bottle connection"
	podman exec $(RBM_SENTRY_CONTAINER) curl -s http://$(RBN_ENCLAVE_BOTTLE_IP):8000/lab | grep "JupyterLab"

	$(MBC_SHOW_WHITE) "Extended wait for full web application initialization"
	sleep 20

	$(MBC_SHOW_WHITE) "Verify port binding on host"
	podman machine ssh "netstat -tuln | grep :8000"

	$(MBC_SHOW_WHITE) "Verify host networking path"
	podman machine ssh "curl -v --connect-timeout 5 --max-time 10 http://127.0.0.1:8000/lab | grep JupyterLab"

	$(MBC_SHOW_WHITE) "Test final host access"
	curl -s --connect-timeout 10 --max-time 20 --retry 3 --retry-delay 5 http://localhost:8000/lab | grep "JupyterLab"

	$(MBC_SHOW_WHITE) "PASS"


# eof
