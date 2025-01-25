

rbm-t.TestRBM.nsproto.mk:
	$(MBC_SHOW_WHITE) "COLLECT INFORMATION HELPFUL IN DEBUGGING..."
	$(MBC_SHOW_WHITE) "facts: $(RBM_SENTRY_CONTAINER) and $(RBM_BOTTLE_CONTAINER)"
	$(MBC_SHOW_WHITE) "Check if dnsmasq is running on sentry"
	podman exec $(RBM_SENTRY_CONTAINER) ps aux | grep dnsmasq
	$(MBC_SHOW_WHITE) "Verify network connectivity"
	podman exec $(RBM_BOTTLE_CONTAINER) ping $(RBN_ENCLAVE_SENTRY_IP) -c 2
	$(MBC_SHOW_WHITE) "Check iptables on sentry"
	podman exec $(RBM_SENTRY_CONTAINER) iptables -L RBM-INGRESS

	$(MBC_SHOW_WHITE) "Show that we can access internet from sentry"
	podman exec $(RBM_SENTRY_CONTAINER) nslookup example.com
	$(MBC_SHOW_WHITE) "Show that we can access anthropic from bottle"
	podman exec $(RBM_BOTTLE_CONTAINER) nslookup anthropic.com
	$(MBC_SHOW_WHITE) "Show that we cannot access google from bottle"
	! podman exec $(RBM_BOTTLE_CONTAINER) nslookup google.com

    
	$(MBC_SHOW_WHITE) "Get anthropic.com IP from sentry then try from bottle:"
	@ANTHROPIC_IP=$$(podman exec $(RBM_SENTRY_CONTAINER) dig +short anthropic.com | head -1)  &&\
	  echo "Testing allowed IP $$ANTHROPIC_IP"                                                &&\
	  podman exec $(RBM_BOTTLE_CONTAINER) ping -c 2 -w 2 $$ANTHROPIC_IP

	$(MBC_SHOW_WHITE) "Get google.com IP from sentry then try from bottle:"
	@GOOGLE_IP=$$(podman exec $(RBM_SENTRY_CONTAINER) dig +short google.com | head -1)  &&\
	  echo "Testing blocked IP $$GOOGLE_IP"                                             &&\
	  ! podman exec $(RBM_BOTTLE_CONTAINER) ping -c 2 -w 2 $$GOOGLE_IP

	$(MBC_SHOW_WHITE) "Testing non-existent domain"
	podman exec -i $(RBM_BOTTLE_CONTAINER) nslookup nonexistentdomain123.test | grep NXDOMAIN

	$(MBC_SHOW_WHITE) "Testing TCP connections"
	@ANTHROPIC_IP=$$(podman exec -i $(RBM_SENTRY_CONTAINER) dig +short anthropic.com | head -1)  &&\
	  podman exec -i $(RBM_BOTTLE_CONTAINER) nc -w 2 -zv $$ANTHROPIC_IP 443                      &&\
	! podman exec -i $(RBM_BOTTLE_CONTAINER) nc -w 2 -zv 8.8.8.8 443

	$(MBC_SHOW_WHITE) "Testing DNS protocols"
	podman exec -i $(RBM_BOTTLE_CONTAINER) dig +tcp anthropic.com
	podman exec -i $(RBM_BOTTLE_CONTAINER) dig +notcp anthropic.com

	$(MBC_SHOW_WHITE) "Testing direct DNS server access"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 anthropic.com
	! podman exec -i $(RBM_BOTTLE_CONTAINER) nc -w 2 -zv 8.8.8.8 53

	$(MBC_SHOW_WHITE) "PASS"

