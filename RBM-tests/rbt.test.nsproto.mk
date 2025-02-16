
# Configuration: where to find the Makefile Bash Console declarations
include $(RBT_MBC_MAKEFILE)

rbt_test_bottle_service_rule:
	$(MBC_STEP) "COLLECT INFORMATION HELPFUL IN DEBUGGING..."
	$(MBC_STEP) "   fact: RBM_SENTRY_CONTAINER  is $(RBM_SENTRY_CONTAINER)"
	$(MBC_STEP) "   fact: RBM_BOTTLE_CONTAINER  is $(RBM_BOTTLE_CONTAINER)"
	$(MBC_STEP) "   fact: RBN_ENCLAVE_SENTRY_IP is $(RBN_ENCLAVE_SENTRY_IP)"
	$(MBC_STEP) "   fact: RBT_TEMP_DIR          is $(RBT_TEMP_DIR)"

	$(MBC_STEP) "Check if dnsmasq is running on sentry"
	podman exec $(RBM_SENTRY_CONTAINER) ps aux | grep dnsmasq
	$(MBC_STEP) "Verify network connectivity"
	podman exec $(RBM_BOTTLE_CONTAINER) ping $(RBN_ENCLAVE_SENTRY_IP) -c 2
	$(MBC_STEP) "Check iptables on sentry"
	podman exec $(RBM_SENTRY_CONTAINER) iptables -L RBM-INGRESS

	$(MBC_STEP) "Show that we can access internet from sentry"
	podman exec $(RBM_SENTRY_CONTAINER) nslookup example.com
	$(MBC_STEP) "Show that we can access anthropic from bottle"
	podman exec $(RBM_BOTTLE_CONTAINER) nslookup anthropic.com
	$(MBC_STEP) "Show that we cannot access google from bottle"
	! podman exec $(RBM_BOTTLE_CONTAINER) nslookup google.com
    
	$(MBC_STEP) "Get anthropic.com IP from sentry then try TCP connection from bottle:"
	@ANTHROPIC_IP=$$(podman exec $(RBM_SENTRY_CONTAINER) dig +short anthropic.com | head -1)  &&\
	  echo "Testing allowed IP $$ANTHROPIC_IP"                                                &&\
	  podman exec $(RBM_BOTTLE_CONTAINER) nc -w 2 -zv $$ANTHROPIC_IP 443

	$(MBC_STEP) "Get google.com IP from sentry then verify TCP blocking from bottle:"
	@GOOGLE_IP=$$(podman exec $(RBM_SENTRY_CONTAINER) dig +short google.com | head -1)  &&\
	  echo "Testing blocked IP $$GOOGLE_IP"                                             &&\
	  ! podman exec $(RBM_BOTTLE_CONTAINER) nc -w 2 -zv $$GOOGLE_IP 443

	$(MBC_STEP) "Testing non-existent domain"
	podman exec -i $(RBM_BOTTLE_CONTAINER) nslookup nonexistentdomain123.test | grep NXDOMAIN

	$(MBC_STEP) "Testing TCP connections"
	@ANTHROPIC_IP=$$(podman exec -i $(RBM_SENTRY_CONTAINER) dig +short anthropic.com | head -1)  &&\
	  podman exec -i $(RBM_BOTTLE_CONTAINER) nc -w 2 -zv $$ANTHROPIC_IP 443                      &&\
	! podman exec -i $(RBM_BOTTLE_CONTAINER) nc -w 2 -zv 8.8.8.8 443

	$(MBC_STEP) "Testing DNS protocols"
	podman exec -i $(RBM_BOTTLE_CONTAINER) dig +tcp anthropic.com
	podman exec -i $(RBM_BOTTLE_CONTAINER) dig +notcp anthropic.com

	$(MBC_STEP) "Testing direct DNS server access"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 anthropic.com
	! podman exec -i $(RBM_BOTTLE_CONTAINER) nc -w 2 -zv 8.8.8.8 53

	$(MBC_STEP) "Testing non-standard DNS ports"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 -p 5353 example.com
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 -p 443 example.com

	$(MBC_STEP) "Testing alternate DNS providers"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @1.1.1.1 example.com
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @9.9.9.9 example.com

	$(MBC_STEP) "Testing DNS zone transfers"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 example.com AXFR

	$(MBC_STEP) "Testing IPv6 DNS"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @2001:4860:4860::8888 example.com

	$(MBC_STEP) "Testing multicast DNS"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @224.0.0.251%eth0 -p 5353 example.local

	$(MBC_STEP) "Testing source IP spoofing"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 +nsid example.com -b 192.168.1.2

	$(MBC_STEP) "Testing DNS tunneling"
	! podman exec $(RBM_BOTTLE_CONTAINER) nc -z -w 1 8.8.8.8 53
	
	$(MBC_STEP) "Verify package management isolation"
	! podman exec -i $(RBM_BOTTLE_CONTAINER) timeout 2 apt-get -qq update 2>&1 | grep -q "Could not resolve"

	$(MBC_STEP) "Test ICMP forwarding behavior"
	podman exec -i $(RBM_BOTTLE_CONTAINER) traceroute -I -m 1 8.8.8.8 2>&1 | grep -q "$(RBN_ENCLAVE_SENTRY_IP)"

	$(MBC_STEP) "Verify ICMP blocking beyond SENTRY" 
	podman exec -i $(RBM_BOTTLE_CONTAINER) traceroute -I -m 2 8.8.8.8 2>&1 | grep -q "^[[:space:]]*2[[:space:]]*\* \* \*"

	$(MBC_STEP) "PASS"


# eof
