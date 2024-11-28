
my_cidr_check = \
  test "$(1)" != "1" || (echo $(2) | grep -E '^[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}/[0-9]{1,2}$$' || \
  (echo "Value '$(2)' must be in valid CIDR notation" && exit 1))

BAD_CIDR = 192.168.1.23/


basic_negative_check:
	echo I want this to succeed..
	! false
	echo And I want this to fail
	! true
	echo "test passed"



test-invalid-prefix:
	echo "Testing CIDR with invalid prefix $(BAD_CIDR)"
	$(call my_cidr_check,1,$(BAD_CIDR)); echo retcode be $$?
	echo next
	@($(call my_cidr_check,1,$(BAD_CIDR)) && exit 13) || true
	echo "Class C CIDR format test passed"


