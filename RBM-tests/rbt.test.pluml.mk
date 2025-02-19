# Copyright 2024 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>

RBT_PLANTUML_URL = http://localhost:$(RBN_ENTRY_PORT_WORKSTATION)
RBT_TEST_DIAGRAM_PATH = $(MBD_TEMP_DIR)/test_diagram.txt

rbt_test_bottle_service_rule:
	$(MBC_SHOW_WHITE) "COLLECT INFORMATION HELPFUL IN DEBUGGING..."
	$(MBC_SHOW_WHITE) "   fact: RBM_SENTRY_CONTAINER       is $(RBM_SENTRY_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBM_BOTTLE_CONTAINER       is $(RBM_BOTTLE_CONTAINER)"
	$(MBC_SHOW_WHITE) "   fact: RBN_ENTRY_PORT_WORKSTATION is $(RBN_ENTRY_PORT_WORKSTATION)"
	$(MBC_SHOW_WHITE) "   fact: RBN_ENCLAVE_SENTRY_IP      is $(RBN_ENCLAVE_SENTRY_IP)"
	$(MBC_SHOW_WHITE) "   fact: RBN_ENCLAVE_BOTTLE_IP      is $(RBN_ENCLAVE_BOTTLE_IP)"
	$(MBC_SHOW_WHITE) "   fact: MBD_TEMP_DIR               is $(MBD_TEMP_DIR)"

	$(MBC_SHOW_WHITE) "Create test diagram content"
	@echo "@startuml\nBob -> Alice: hello there\nAlice --> Bob: boo\n@enduml" > $(RBT_TEST_DIAGRAM_PATH)

	$(MBC_SHOW_WHITE) "Watch network traffic during request attempt"
	podman exec $(RBM_SENTRY_CONTAINER) tcpdump -n -i eth0 port $(RBN_ENTRY_PORT_WORKSTATION) & sleep 1
	curl -v --connect-timeout 5 --max-time 10 $(RBT_PLANTUML_URL)/txt/SyfFKj2rKt3CoKnELR1Io4ZDoSbNACb8BKhbWeZf0cMTyfEi59Boym40 || true
	sleep 2
	podman exec $(RBM_SENTRY_CONTAINER) pkill tcpdump

	$(MBC_SHOW_WHITE) "Test PlantUML text rendering endpoint"
	echo "Testing server response contains expected elements..."
	curl -s $(RBT_PLANTUML_URL)/txt/SyfFKj2rKt3CoKnELR1Io4ZDoSbNACb8BKhbWeZf0cMTyfEi59Boym40 > $(MBD_TEMP_DIR)/response.txt
	grep -q "Bob"         $(MBD_TEMP_DIR)/response.txt
	grep -q "Alice"       $(MBD_TEMP_DIR)/response.txt
	grep -q "hello there" $(MBD_TEMP_DIR)/response.txt
	grep -q "boo"         $(MBD_TEMP_DIR)/response.txt

	$(MBC_SHOW_WHITE) "Test PlantUML server with local diagram"
	cat $(RBT_TEST_DIAGRAM_PATH) | curl -s --data-binary @- $(RBT_PLANTUML_URL)/txt/uml > $(MBD_TEMP_DIR)/local_response.txt
	grep -q "Bob"         $(MBD_TEMP_DIR)/local_response.txt
	grep -q "Alice"       $(MBD_TEMP_DIR)/local_response.txt
	grep -q "hello there" $(MBD_TEMP_DIR)/local_response.txt
	grep -q "boo"         $(MBD_TEMP_DIR)/local_response.txt

	$(MBC_SHOW_WHITE) "Verify server handles basic HTTP headers"
	curl -v -H "User-Agent: Mozilla/5.0" -H "Accept: text/plain" \
	  --connect-timeout 5 --max-time 10 $(RBT_PLANTUML_URL)/txt/SyfFKj2rKt3CoKnELR1Io4ZDoSbNACb8BKhbWeZf0cMTyfEi59Boym40

	$(MBC_SHOW_WHITE) "Test server response with invalid diagram hash"
	@test 0 -eq $$(curl -s $(RBT_PLANTUML_URL)/txt/invalid_hash | grep -c "Bob")

	$(MBC_SHOW_WHITE) "Test server response with malformed diagram"
	@test 0 -eq $$(echo "invalid uml content" | curl -s --data-binary @- $(RBT_PLANTUML_URL)/txt/uml | grep -c "Bob")

	$(MBC_PASS) "All PlantUML service tests passed successfully."


# eof
