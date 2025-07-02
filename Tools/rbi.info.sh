#!/bin/bash
# RBI SPLIT IMPLEMENTATION PLAN
# =============================
# This script should be split into two execution contexts to avoid namespace/command issues
#
# PART 1: Host Execution (using podman connection)
# -------------------------------------------------
# Execute from makefile with: podman $(RBM_CONNECTION) exec -i $(RBM_SENTRY_CONTAINER) /path/to/rbi-host.sh
# Or inline the commands directly in the makefile
#
# Commands that work from host:
#   - podman $(RBM_CONNECTION) network inspect ${RBM_ENCLAVE_NETWORK}
#   - podman $(RBM_CONNECTION) inspect ${RBM_SENTRY_CONTAINER}
#   - podman $(RBM_CONNECTION) inspect ${RBM_BOTTLE_CONTAINER}
#   - Any podman commands that query container/network state
#
# PART 2: VM Execution with Namespace Access
# ------------------------------------------
# Execute from makefile with: $(zRBM_PODMAN_SSH_CMD) < rbi-vm.sh
# Must find aardvark-dns PID and use nsenter for network visibility
#
# Commands that require VM + namespace:
#   - AARDVARK_PID=$(pgrep -f aardvark-dns)
#   - sudo nsenter -t $AARDVARK_PID -n ip link show | grep veth
#   - sudo nsenter -t $AARDVARK_PID -n tc qdisc show dev <veth>
#   - sudo nsenter -t $AARDVARK_PID -n tc filter show dev <veth> ingress/egress
#
# IMPLEMENTATION NOTES:
# - The aardvark-dns process manages the podman network namespace
# - veth interfaces only exist within this namespace
# - TC/eBPF filters are attached within this namespace
# - Regular VM commands won't see these network objects
# - Consider whether this diagnostic info is worth the complexity vs. removing entirely

set -e

echo "NEEDS REPAIRS.  Concept above."  &&  false

# Validate required environment variables
: ${RBM_MONIKER:?}              && echo "RBI: RBM_MONIKER              = ${RBM_MONIKER}"
: ${RBM_ENCLAVE_NETWORK:?}      && echo "RBI: RBM_ENCLAVE_NETWORK      = ${RBM_ENCLAVE_NETWORK}"
: ${RBM_SENTRY_CONTAINER:?}     && echo "RBI: RBM_SENTRY_CONTAINER     = ${RBM_SENTRY_CONTAINER}"
: ${RBM_BOTTLE_CONTAINER:?}     && echo "RBI: RBM_BOTTLE_CONTAINER     = ${RBM_BOTTLE_CONTAINER}"

echo "RBI: Network Configuration:"
echo "================================"

echo "RBI: Enclave Network:"
podman network inspect ${RBM_ENCLAVE_NETWORK} --format 'Gateway: {{(index .Subnets 0).Gateway}}, Subnet: {{(index .Subnets 0).Subnet}}, Bridge: {{.NetworkInterface}}'
echo ""

echo "RBI: SENTRY Container Networks:"
podman inspect ${RBM_SENTRY_CONTAINER} --format '{{range $net, $conf := .NetworkSettings.Networks}}{{$net}}: {{$conf.IPAddress}}{{println}}{{end}}'

echo "RBI: BOTTLE Container Networks:"
podman inspect ${RBM_BOTTLE_CONTAINER} --format '{{range $net, $conf := .NetworkSettings.Networks}}{{$net}}: {{$conf.IPAddress}}{{println}}{{end}}' 2>/dev/null || echo "(not yet created)"
echo ""

echo "RBI: Container veth interfaces:"
ip link show | grep -E "veth[0-9a-f]+" | grep -v "@" || echo "No veth interfaces found"
echo ""

echo "RBI: TC filters on container interfaces:"
for veth in $(ip link show | grep -oE "veth[0-9a-f]+" | sort -u); do
    if tc qdisc show dev $veth 2>/dev/null | grep -q clsact; then
        echo "  $veth:"
        tc filter show dev $veth ingress 2>/dev/null | grep -E "bpf|direct-action" | sed 's/^/    /'
        tc filter show dev $veth egress  2>/dev/null | grep -E "bpf|direct-action" | sed 's/^/    /'
    fi
done

echo ""
echo "RBI: Network info complete"

