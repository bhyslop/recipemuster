
my_cidr_check = \
  test "$(1)" != "1" || (echo $(2) | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$$' || \
  (echo "Value '$(2)' must be in valid CIDR notation" && exit 1))




GOOD_CIDR = 192.168.1.23/23

BAD_CIDR = xyvvv


basic_negative_cidr:
	echo "First test..."
	$(call my_cidr_check,1,$(GOOD_CIDR))
	echo "Second test..."
	! $(call my_cidr_check,1,$(BAD_CIDR))
	echo "test passed"


basic_negative_check:
	echo "First test..."
	! false || { echo "ERROR: Command failed as expected - continuing"; false; }
	echo "Second test..."
	! true || { echo "ERROR: Command passed when it should fail!"; exit 1; }
	echo "test passed"



