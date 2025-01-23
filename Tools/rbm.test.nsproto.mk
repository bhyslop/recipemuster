

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

	@echo "Testing non-existent domain"
	podman exec -i $(TRBM_BOTTLE) nslookup nonexistentdomain123.test | grep NXDOMAIN

	@echo "Testing TCP connections"
	@ANTHROPIC_IP=$$(podman exec -i $(TRBM_SENTRY) dig +short anthropic.com | head -1)  &&\
	  podman exec -i $(TRBM_BOTTLE) nc -w 2 -zv $$ANTHROPIC_IP 443                      &&\
	! podman exec -i $(TRBM_BOTTLE) nc -w 2 -zv 8.8.8.8 443

	@echo "Testing DNS protocols"
	podman exec -i $(TRBM_BOTTLE) dig +tcp anthropic.com
	podman exec -i $(TRBM_BOTTLE) dig +notcp anthropic.com

	@echo "Testing direct DNS server access"
	! podman exec -i $(TRBM_BOTTLE) dig @8.8.8.8 anthropic.com
	! podman exec -i $(TRBM_BOTTLE) nc -w 2 -zv 8.8.8.8 53

	@echo "PASS"

