

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

	$(MBC_SHOW_WHITE) "Testing non-standard DNS ports"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 -p 5353 example.com
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 -p 443 example.com

	$(MBC_SHOW_WHITE) "Testing alternate DNS providers"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @1.1.1.1 example.com
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @9.9.9.9 example.com

	$(MBC_SHOW_WHITE) "Testing DNS zone transfers"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 example.com AXFR

	$(MBC_SHOW_WHITE) "Testing IPv6 DNS"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @2001:4860:4860::8888 example.com

	$(MBC_SHOW_WHITE) "Testing multicast DNS"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @224.0.0.251%eth0 -p 5353 example.local

	$(MBC_SHOW_WHITE) "Testing source IP spoofing"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 +nsid example.com -b 192.168.1.2

	$(MBC_SHOW_WHITE) "Testing DNS tunneling"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) nc -u 8.8.8.8 53 < /dev/urandom

	$(MBC_SHOW_WHITE) "Verify package management isolation"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) timeout 2 apt-get -qq update 2>&1 | grep -q "Could not resolve"

	$(MBC_SHOW_WHITE) "Test ICMP tunneling attempts"  
	! podman exec -i $(RBM_BOTTLE_CONTAINER) traceroute -I 8.8.8.8

	$(MBC_SHOW_WHITE) "PASS"

