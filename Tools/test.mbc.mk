include mbc.MakefileBashConsole.mk

tmbc_test_all:              \
  tmbc_test_basic_cidr      \
  tmbc_test_zero_cidr       \
  tmbc_test_max_cidr        \
  tmbc_test_class_a         \
  tmbc_test_class_b         \
  tmbc_test_class_c         \
  tmbc_test_max_octet       \
  tmbc_test_edge_masks      \
  tmbc_test_no_prefix       \
  tmbc_test_missing_parts   \
  tmbc_test_too_many_octets \
  # end-of-list
	$(MBC_PASS) "All CIDR format tests completed"


tmbc_test_basic_cidr:
	@(export THE_CIDR=192.168.1.0/24 && \
	  echo "Testing standard CIDR format $$THE_CIDR" && \
	  $(call MBC_CHECK__IS_CIDR,1,$$THE_CIDR))
	$(MBC_PASS) "Basic CIDR test passed"

tmbc_test_zero_cidr:
	@(export THE_CIDR=0.0.0.0/0 && \
	  echo "Testing minimum/catch-all CIDR $$THE_CIDR" && \
	  $(call MBC_CHECK__IS_CIDR,1,$$THE_CIDR))
	$(MBC_PASS) "Zero CIDR test passed"

tmbc_test_max_cidr:
	@(export THE_CIDR=255.255.255.255/32 && \
	  echo "Testing maximum/single-host CIDR $$THE_CIDR" && \
	  $(call MBC_CHECK__IS_CIDR,1,$$THE_CIDR))
	$(MBC_PASS) "Max CIDR test passed"

tmbc_test_class_a:
	@(export THE_CIDR=10.0.0.0/8 && \
	  echo "Testing Class A private network CIDR $$THE_CIDR" && \
	  $(call MBC_CHECK__IS_CIDR,1,$$THE_CIDR))
	$(MBC_PASS) "Class A test passed"

tmbc_test_class_b:
	@(export THE_CIDR=172.16.0.0/16 && \
	  echo "Testing Class B private network CIDR $$THE_CIDR" && \
	  $(call MBC_CHECK__IS_CIDR,1,$$THE_CIDR))
	$(MBC_PASS) "Class B test passed"

tmbc_test_class_c:
	@(export THE_CIDR=192.168.0.0/24 && \
	  echo "Testing Class C private network CIDR $$THE_CIDR" && \
	  $(call MBC_CHECK__IS_CIDR,1,$$THE_CIDR))
	$(MBC_PASS) "Class C test passed"

tmbc_test_no_prefix:
	@(! (export THE_CIDR=192.168.1.0 && \
	  echo "Testing CIDR missing prefix: $$THE_CIDR" && \
	  $(call MBC_CHECK__IS_CIDR,1,$$THE_CIDR)))
	$(MBC_PASS) "No prefix test passed"

tmbc_test_missing_parts:
	@(! (export THE_CIDR=192.168/24 && \
	  echo "Testing incomplete IP address in CIDR: $$THE_CIDR" && \
	  $(call MBC_CHECK__IS_CIDR,1,$$THE_CIDR)))
	$(MBC_PASS) "Missing parts test passed"

tmbc_test_max_octet:
	@(export THE_CIDR=250.251.252.253/24 && \
	  echo "Testing valid high octets in CIDR: $$THE_CIDR" && \
	  $(call MBC_CHECK__IS_CIDR,1,$$THE_CIDR))
	$(MBC_PASS) "Max octet test passed"

tmbc_test_edge_masks:
	@(export THE_CIDR=192.168.1.0/31 && \
	  echo "Testing edge case prefix length: $$THE_CIDR" && \
	  $(call MBC_CHECK__IS_CIDR,1,$$THE_CIDR))
	$(MBC_PASS) "Edge mask test passed"

tmbc_test_too_many_octets:
	@(! (export THE_CIDR=192.168.1.1.1/24 && \
	  echo "Testing CIDR with excess octets: $$THE_CIDR" && \
	  $(call MBC_CHECK__IS_CIDR,1,$$THE_CIDR)))
	$(MBC_PASS) "Too many octets test passed"
