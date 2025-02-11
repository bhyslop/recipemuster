#!/bin/sh
echo "RBTJ: Beginning Jupyter test script"

set -e
set -x

# Validate required environment variables
: ${RBN_ENCLAVE_BOTTLE_IP:?}    && echo "RBTJ0: RBN_ENCLAVE_BOTTLE_IP    = ${RBN_ENCLAVE_BOTTLE_IP}"
: ${RBN_ENTRY_PORT_ENCLAVE:?}   && echo "RBTJ0: RBN_ENTRY_PORT_ENCLAVE   = ${RBN_ENTRY_PORT_ENCLAVE}"

# Define API endpoint
JUPYTER_BASE="http://${RBN_ENCLAVE_BOTTLE_IP}:${RBN_ENTRY_PORT_ENCLAVE}"

echo "RBTJ1: Testing base Jupyter connectivity"
curl -s "${JUPYTER_BASE}/api/status" | jq . || exit 1

echo "RBTJ2: Obtaining XSRF token"
XSRF_TOKEN=$(curl -s -I "${JUPYTER_BASE}/lab" | grep -i "set-cookie" | grep "_xsrf" | cut -d= -f2 | cut -d\; -f1) || exit 1
echo "RBTJ2: XSRF token obtained: ${XSRF_TOKEN}" || exit 1
test -n "${XSRF_TOKEN}" || exit 1

echo "RBTJ3: Creating new kernel"
KERNEL_RESPONSE=$(curl -s -X POST "${JUPYTER_BASE}/api/kernels" \
    -H "X-XSRFToken: ${XSRF_TOKEN}" \
    -H "Cookie: _xsrf=${XSRF_TOKEN}" \
    -H "Content-Type: application/json" \
    -d '{"name":"python3"}') || exit 1
echo "RBTJ3: Kernel response: ${KERNEL_RESPONSE}" || exit 1

echo "RBTJ4: Extracting kernel ID"
KERNEL_ID=$(echo "${KERNEL_RESPONSE}" | jq -r .id) || exit 1
echo "RBTJ4: Kernel ID: ${KERNEL_ID}" || exit 1
test -n "${KERNEL_ID}" || exit 1

echo "RBTJ5: Testing WebSocket connection"
MSG_ID=$(cat /proc/sys/kernel/random/uuid) || exit 1
SESSION_ID=$(cat /proc/sys/kernel/random/uuid) || exit 1
CURRENT_DATE=$(date -u +"%Y-%m-%dT%H:%M:%S.%3NZ") || exit 1

echo "RBTJ6: Sending test computation via WebSocket"
echo '{
    "header": {
        "msg_id": "'"${MSG_ID}"'",
        "msg_type": "execute_request",
        "username": "test",
        "session": "'"${SESSION_ID}"'",
        "date": "'"${CURRENT_DATE}"'"
    },
    "parent_header": {},
    "metadata": {},
    "content": {
        "code": "import sys\nprint(f\"Python {sys.version.split()[0]}\")",
        "silent": false,
        "store_history": false,
        "user_expressions": {},
        "allow_stdin": false
    },
    "channel": "shell"
}' | websocat "ws://${RBN_ENCLAVE_BOTTLE_IP}:${RBN_ENTRY_PORT_ENCLAVE}/api/kernels/${KERNEL_ID}/channels" \
    | tee /dev/stderr \
    | jq 'select(.msg_type == "stream") | .content.text' || exit 1

echo "RBTJ7: Cleaning up test kernel"
curl -s -X DELETE "${JUPYTER_BASE}/api/kernels/${KERNEL_ID}" \
    -H "X-XSRFToken: ${XSRF_TOKEN}" \
    -H "Cookie: _xsrf=${XSRF_TOKEN}" || exit 1

echo "RBTJ8: Verifying kernel deletion"
REMAINING_KERNELS=$(curl -s "${JUPYTER_BASE}/api/kernels" \
    -H "X-XSRFToken: ${XSRF_TOKEN}" \
    -H "Cookie: _xsrf=${XSRF_TOKEN}" | jq '. | length') || exit 1
echo "RBTJ8: Remaining kernels: ${REMAINING_KERNELS}" || exit 1
test "${REMAINING_KERNELS}" = "0" || exit 1

echo "RBTJ: Jupyter test script completed successfully"

