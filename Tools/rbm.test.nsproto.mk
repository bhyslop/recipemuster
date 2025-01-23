
TRBM_SENTRY = nsproto-sentry
TRBM_BOTTLE = nsproto-bottle

rbm-t.TestRBM.nsproto.mk:
	@echo "COLLECT INFORMATION HELPFUL IN DEBUGGING..."
	@echo "facts: $(RBM_SENTRY_CONTAINER) and $(RBM_BOTTLE_CONTAINER)"
	@echo "Check if dnsmasq is running on sentry"
	podman exec $(RBM_SENTRY_CONTAINER) ps aux | grep dnsmasq
	@echo "Start dnsmasq if not running"
	podman exec $(RBM_SENTRY_CONTAINER) dnsmasq --keep-in-foreground
	@echo "Verify network connectivity"
	podman exec $(RBM_BOTTLE_CONTAINER) ping $(RBN_ENCLAVE_SENTRY_IP)
	@echo "Check iptables on sentry"
	podman exec $(RBM_SENTRY_CONTAINER) iptables -L RBM-INGRESS

	@echo "Show that we can access internet from sentry"
	podman exec -it $(TRBM_SENTRY) nslookup example.com
	@echo "Show that we can access anthropic from bottle"
	podman exec -it $(TRBM_BOTTLE) nslookup anthropic.com
	@echo "Show that we cannot access google from bottle"
	! podman exec -it $(TRBM_BOTTLE) nslookup google.com
	@echo "PASS"

