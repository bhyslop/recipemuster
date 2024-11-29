include mbc.MakefileBashConsole.mk

tmbc_check_ipv4_test_all:                \
  ztmbc_check_ipv4_test_basic            \
  ztmbc_check_ipv4_test_zeros            \
  ztmbc_check_ipv4_test_max_octets       \
  ztmbc_check_ipv4_test_leading_zeros    \
  ztmbc_check_ipv4_test_missing_octet    \
  ztmbc_check_ipv4_test_too_many_octets  \
  ztmbc_check_ipv4_test_double_dots      \
  ztmbc_check_ipv4_test_end_dot          \
  # end-of-list
	$(MBC_PASS) "All IPv4 format tests completed"

ztmbc_check_ipv4_test_basic:
	@(export THE_IPV4=192.168.1.1 && \
	  echo "Testing basic IPv4 format: $$THE_IPV4" && \
	  $(call MBC_CHECK__IS_IPV4,1,$$THE_IPV4))
	$(MBC_PASS) "Basic IPv4 test passed"

ztmbc_check_ipv4_test_zeros:
	@(export THE_IPV4=0.0.0.0 && \
	  echo "Testing all zeros: $$THE_IPV4" && \
	  $(call MBC_CHECK__IS_IPV4,1,$$THE_IPV4))
	$(MBC_PASS) "Zero IPv4 test passed"

ztmbc_check_ipv4_test_max_octets:
	@(export THE_IPV4=255.255.255.255 && \
	  echo "Testing maximum values: $$THE_IPV4" && \
	  $(call MBC_CHECK__IS_IPV4,1,$$THE_IPV4))
	$(MBC_PASS) "Max octets test passed"

ztmbc_check_ipv4_test_leading_zeros:
	@(export THE_IPV4=192.168.001.001 && \
	  echo "Testing leading zeros: $$THE_IPV4" && \
	  $(call MBC_CHECK__IS_IPV4,1,$$THE_IPV4))
	$(MBC_PASS) "Leading zeros test passed"

ztmbc_check_ipv4_test_missing_octet:
	@(! (export THE_IPV4=192.168.1 && \
	  echo "Testing missing octet: $$THE_IPV4" && \
	  $(call MBC_CHECK__IS_IPV4,1,$$THE_IPV4)))
	$(MBC_PASS) "Missing octet test passed"

ztmbc_check_ipv4_test_too_many_octets:
	@(! (export THE_IPV4=192.168.1.1.1 && \
	  echo "Testing too many octets: $$THE_IPV4" && \
	  $(call MBC_CHECK__IS_IPV4,1,$$THE_IPV4)))
	$(MBC_PASS) "Too many octets test passed"

ztmbc_check_ipv4_test_double_dots:
	@(! (export THE_IPV4=192..168.1.1 && \
	  echo "Testing double dots: $$THE_IPV4" && \
	  $(call MBC_CHECK__IS_IPV4,1,$$THE_IPV4)))
	$(MBC_PASS) "Double dots test passed"

ztmbc_check_ipv4_test_end_dot:
	@(! (export THE_IPV4=192.168.1.1. && \
	  echo "Testing trailing dot: $$THE_IPV4" && \
	  $(call MBC_CHECK__IS_IPV4,1,$$THE_IPV4)))
	$(MBC_PASS) "End dot test passed"

