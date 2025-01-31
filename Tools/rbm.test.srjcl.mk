rbm-t.TestRBM.srjcl.mk:
	$(MBC_SHOW_WHITE) "COLLECT INFORMATION HELPFUL IN DEBUGGING..."
	$(MBC_SHOW_WHITE) "   fact: RBM_SENTRY_CONTAINER  is $(RBM_SENTRY_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBM_BOTTLE_CONTAINER  is $(RBM_BOTTLE_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBN_ENCLAVE_SENTRY_IP is $(RBN_ENCLAVE_SENTRY_IP)"

	$(MBC_SHOW_WHITE) "Verify Jupyter process is running in bottle"
	podman exec $(RBM_BOTTLE_CONTAINER) ps aux | grep jupyter

	$(MBC_SHOW_WHITE) "Examine initial connection tracking state"
	podman exec $(RBM_SENTRY_CONTAINER) conntrack -L | grep 8000 || true

	$(MBC_SHOW_WHITE) "Make initial connection attempt to establish NAT state"
	curl -s --connect-timeout 5 --max-time 10 http://localhost:8000/lab || true

	$(MBC_SHOW_WHITE) "Examine connection tracking state after first attempt"
	podman exec $(RBM_SENTRY_CONTAINER) conntrack -L | grep 8000 || true

	$(MBC_SHOW_WHITE) "Make second connection attempt now that NAT is primed"
	curl -s --connect-timeout 5 --max-time 10 http://localhost:8000/lab | grep "JupyterLab"

	$(MBC_SHOW_WHITE) "PASS"


# eof
