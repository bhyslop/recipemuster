

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

	$(MBC_SHOW_WHITE) "Verify Jupyter port is listening"
	podman exec $(RBM_BOTTLE_CONTAINER) netstat -tuln | grep 8000

	$(MBC_SHOW_WHITE) "Test local connection to Jupyter in bottle"
	podman exec $(RBM_BOTTLE_CONTAINER) nc -zv localhost 8000

	$(MBC_SHOW_WHITE) "Verify Jupyter HTTP response"
	podman exec $(RBM_BOTTLE_CONTAINER) echo -e "GET /lab HTTP/1.0\r\n\r\n" | podman exec -i $(RBM_BOTTLE_CONTAINER) /usr/bin/nc localhost 8000 | grep "HTTP/1.1 200 OK"

	$(MBC_SHOW_WHITE) "Show port forwarding rules in sentry" 
	podman exec $(RBM_SENTRY_CONTAINER) iptables -t nat -L PREROUTING -n -v | grep 8000

	$(MBC_SHOW_WHITE) "Let's first verify IP connectivity between sentry and bottle:"
	podman exec srjcl-sentry ping -c 2 10.242.0.3

	$(MBC_SHOW_WHITE) "Let's look at the routing on both sides:"
	podman exec srjcl-sentry ip route show
	podman exec srjcl-bottle ip route show

	$(MBC_SHOW_WHITE) "Let's check the actual interfaces in sentry:"
	podman exec srjcl-sentry ip addr show eth1

	$(MBC_SHOW_WHITE) "Let's examine all forwarding rules in sentry:"
	podman exec srjcl-sentry iptables -L RBM-FORWARD -n -v

	$(MBC_SHOW_WHITE) "Test connectivity from sentry to bottle"
	podman exec srjcl-sentry curl -v --connect-timeout 5 --max-time 10 http://10.242.0.3:8000/lab

	$(MBC_SHOW_WHITE) "PASS"
