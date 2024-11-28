include mbc.MakefileBashConsole.mk

# Test valid CIDR formats
test-cidr-valid: test-basic-cidr test-zero-cidr test-max-cidr test-class-a test-class-b test-class-c

test-basic-cidr:
	$(MBC_START) "Testing basic CIDR format 192.168.1.0/24"
	@$(call MBC_CHECK__IS_CIDR,1,192.168.1.0/24)
	$(MBC_PASS) "Basic CIDR format test passed"

test-zero-cidr:
	$(MBC_START) "Testing zero CIDR format 0.0.0.0/0"
	@$(call MBC_CHECK__IS_CIDR,1,0.0.0.0/0)
	$(MBC_PASS) "Zero CIDR format test passed"

test-max-cidr:
	$(MBC_START) "Testing max CIDR format 255.255.255.255/32"
	@$(call MBC_CHECK__IS_CIDR,1,255.255.255.255/32)
	$(MBC_PASS) "Max CIDR format test passed"

test-class-a:
	$(MBC_START) "Testing Class A CIDR 10.0.0.0/8"
	@$(call MBC_CHECK__IS_CIDR,1,10.0.0.0/8)
	$(MBC_PASS) "Class A CIDR format test passed"

test-class-b:
	$(MBC_START) "Testing Class B CIDR 172.16.0.0/16"
	@$(call MBC_CHECK__IS_CIDR,1,172.16.0.0/16)
	$(MBC_PASS) "Class B CIDR format test passed"

test-class-c:
	$(MBC_START) "Testing Class C CIDR 192.168.0.0/24"
	@$(call MBC_CHECK__IS_CIDR,1,192.168.0.0/24)
	$(MBC_PASS) "Class C CIDR format test passed"

# Test invalid CIDR formats
test-cidr-invalid: test-no-prefix test-invalid-ip test-invalid-prefix test-missing-parts

test-no-prefix:
	$(MBC_START) "Testing CIDR without prefix 192.168.1.0"
	@$(call MBC_CHECK__IS_CIDR,1,192.168.1.0) || $(MBC_PASS) "Invalid CIDR without prefix correctly rejected"

test-invalid-ip:
	$(MBC_START) "Testing CIDR with invalid IP 256.168.1.0/24"
	@$(call MBC_CHECK__IS_CIDR,1,256.168.1.0/24) || $(MBC_PASS) "Invalid IP in CIDR correctly rejected"


GOOD_CIDR = 192.168.1.23/23

BAD_CIDR = 192.168.1.23


test-invalid-prefix:
	echo "First test..."
	(export THE_CIDR=192.168.1.23/23 && $(call MBC_CHECK__IS_CIDR,1,$$THE_CIDR))
	echo "Second test..."
	(! (export THE_CIDR=192.168.1.23    && $(call MBC_CHECK__IS_CIDR,1,$$THE_CIDR))) 
	echo "test passed"


test-missing-parts:
	$(MBC_START) "Testing incomplete CIDR 192.168/24"
	@$(call MBC_CHECK__IS_CIDR,1,192.168/24) || $(MBC_PASS) "Incomplete CIDR correctly rejected"

# Run all tests
test-all: test-cidr-valid test-cidr-invalid
	$(MBC_PASS) "All CIDR format tests completed"

.PHONY: test-all test-cidr-valid test-cidr-invalid test-basic-cidr test-zero-cidr test-max-cidr test-class-a test-class-b test-class-c test-no-prefix test-invalid-ip test-invalid-prefix test-missing-parts
