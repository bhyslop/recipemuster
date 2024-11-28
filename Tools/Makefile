

MBC_CHECK__IS_CIDR = \
  test "$(1)" != "1" || (echo $(2) | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$$' || \
  ($(MBC_SEE_RED) "Value '$(2)' must be in valid CIDR notation" && exit 1))



GOOD_CIDR = 192.168.1.23/23

BAD_CIDR = 192.168.1.23


basic_negative_retry:
	echo "First test..."
	($(call MBC_CHECK__IS_CIDR,1,$(GOOD_CIDR)))
	echo "Second test..."
	! ($(call MBC_CHECK__IS_CIDR,1,$(BAD_CIDR)))
	echo "test passed"



