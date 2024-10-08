#!/bin/sh

set -e
set -x

echo "Checking and displaying environment variables..."
: ${RBEV_SENTRY_GUARDED_INTERFACE:?} && echo "RBEV_SENTRY_GUARDED_INTERFACE = $RBEV_SENTRY_GUARDED_INTERFACE"
: ${RBEV_SENTRY_GUARDED_IP:?}        && echo "RBEV_SENTRY_GUARDED_IP        = $RBEV_SENTRY_GUARDED_IP"

MAX_ATTEMPTS=4
DELAY=2

echo "Waiting for guarded network interface to be ready..."
for i in $(seq 1 $MAX_ATTEMPTS); do
    if ip addr show $RBEV_SENTRY_GUARDED_INTERFACE | grep -q "$RBEV_SENTRY_GUARDED_IP"; then
        echo "Guarded network interface is ready."
        exit 0
    fi
    echo "Attempt $i: Guarded network interface not ready. Waiting $DELAY second(s)..."
    sleep $DELAY
done

echo "Error: Guarded network interface did not become ready within $MAX_ATTEMPTS attempts."
exit 1
