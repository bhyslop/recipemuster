

rbm-t.TestRBM.nsproto.mk:
	@echo "COLLECT INFORMATION HELPFUL IN DEBUGGING..."
	@echo "facts: $(RBM_SENTRY_CONTAINER) and $(RBM_BOTTLE_CONTAINER)"
	@echo "Check if dnsmasq is running on sentry"
	podman exec $(RBM_SENTRY_CONTAINER) ps aux | grep dnsmasq
	@echo "Verify network connectivity"
	podman exec $(RBM_BOTTLE_CONTAINER) ping $(RBN_ENCLAVE_SENTRY_IP) -c 2
	@echo "Check iptables on sentry"
	podman exec $(RBM_SENTRY_CONTAINER) iptables -L RBM-INGRESS

	@echo "Show that we can access internet from sentry"
	podman exec $(RBM_SENTRY_CONTAINER) nslookup example.com
	@echo "Show that we can access anthropic from bottle"
	podman exec $(RBM_BOTTLE_CONTAINER) nslookup anthropic.com
	@echo "Show that we cannot access google from bottle"
	! podman exec $(RBM_BOTTLE_CONTAINER) nslookup google.com

    
	@echo "Get anthropic.com IP from sentry then try from bottle:"
	@ANTHROPIC_IP=$$(podman exec $(RBM_SENTRY_CONTAINER) dig +short anthropic.com | head -1)  &&\
	  echo "Testing allowed IP $$ANTHROPIC_IP"                                                &&\
	  podman exec $(RBM_BOTTLE_CONTAINER) ping -c 2 -w 2 $$ANTHROPIC_IP

	@echo "Get google.com IP from sentry then try from bottle:"
	@GOOGLE_IP=$$(podman exec $(RBM_SENTRY_CONTAINER) dig +short google.com | head -1)  &&\
	  echo "Testing blocked IP $$GOOGLE_IP"                                             &&\
	  ! podman exec $(RBM_BOTTLE_CONTAINER) ping -c 2 -w 2 $$GOOGLE_IP

	@echo "PASS"

