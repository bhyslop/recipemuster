# Configuration: where to find the Makefile Bash Console declarations
include $(RBT_MBC_MAKEFILE)

RBT_PLANTUML_URL = http://localhost:$(RBN_ENTRY_PORT_WORKSTATION)
RBT_TEST_DIAGRAM_PATH = $(RBT_TEMP_DIR)/test_diagram.txt

rbt_test_bottle_service_rule:
	$(MBC_SHOW_WHITE) "COLLECT INFORMATION HELPFUL IN DEBUGGING..."
	$(MBC_SHOW_WHITE) "   fact: RBM_SENTRY_CONTAINER       is $(RBM_SENTRY_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBM_BOTTLE_CONTAINER       is $(RBM_BOTTLE_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBN_ENTRY_PORT_WORKSTATION is $(RBN_ENTRY_PORT_WORKSTATION)"
	$(MBC_SHOW_WHITE) "   fact: RBN_ENCLAVE_SENTRY_IP      is $(RBN_ENCLAVE_SENTRY_IP)"
	$(MBC_SHOW_WHITE) "   fact: RBN_ENCLAVE_BOTTLE_IP      is $(RBN_ENCLAVE_BOTTLE_IP)"
	$(MBC_SHOW_WHITE) "   fact: RBT_TEMP_DIR               is $(RBT_TEMP_DIR)"

	$(MBC_SHOW_WHITE) "Create test diagram content"
	@echo "@startuml\nBob -> Alice: hello there\nAlice --> Bob: boo\n@enduml" > $(RBT_TEST_DIAGRAM_PATH)

	$(MBC_SHOW_WHITE) "Watch network traffic during request attempt"
	podman exec $(RBM_SENTRY_CONTAINER) tcpdump -n -i eth0 port $(RBN_ENTRY_PORT_WORKSTATION) & sleep 1
	curl -v --connect-timeout 5 --max-time 10 $(RBT_PLANTUML_URL)/txt/SyfFKj2rKt3CoKnELR1Io4ZDoSbNACb8BKhbWeZf0cMTyfEi59Boym40 || true
	sleep 2
	podman exec $(RBM_SENTRY_CONTAINER) pkill tcpdump

	$(MBC_SHOW_WHITE) "Test PlantUML text rendering endpoint"
	echo "Testing server response contains expected elements..."
	curl -s $(RBT_PLANTUML_URL)/txt/SyfFKj2rKt3CoKnELR1Io4ZDoSbNACb8BKhbWeZf0cMTyfEi59Boym40 > $(RBT_TEMP_DIR)/response.txt
	grep -q "Bob"         $(RBT_TEMP_DIR)/response.txt
	grep -q "Alice"       $(RBT_TEMP_DIR)/response.txt
	grep -q "hello there" $(RBT_TEMP_DIR)/response.txt
	grep -q "boo"         $(RBT_TEMP_DIR)/response.txt

	$(MBC_SHOW_WHITE) "Test PlantUML server with local diagram"
	cat $(RBT_TEST_DIAGRAM_PATH) | curl -s --data-binary @- $(RBT_PLANTUML_URL)/txt/uml > $(RBT_TEMP_DIR)/local_response.txt
	grep -q "Bob"         $(RBT_TEMP_DIR)/local_response.txt
	grep -q "Alice"       $(RBT_TEMP_DIR)/local_response.txt
	grep -q "hello there" $(RBT_TEMP_DIR)/local_response.txt
	grep -q "boo"         $(RBT_TEMP_DIR)/local_response.txt

	$(MBC_SHOW_WHITE) "Verify server handles basic HTTP headers"
	curl -v -H "User-Agent: Mozilla/5.0" -H "Accept: text/plain" \
	  --connect-timeout 5 --max-time 10 $(RBT_PLANTUML_URL)/txt/SyfFKj2rKt3CoKnELR1Io4ZDoSbNACb8BKhbWeZf0cMTyfEi59Boym40

	$(MBC_SHOW_WHITE) "Test server response with invalid diagram hash"
	! curl -s $(RBT_PLANTUML_URL)/txt/invalid_hash | grep "Bob"

	$(MBC_SHOW_WHITE) "Test server response with malformed diagram"
	echo "invalid uml content" | ! curl -s --data-binary @- $(RBT_PLANTUML_URL)/txt/uml | grep "Bob"

	$(MBC_PASS) "All PlantUML service tests passed successfully."


# eof
