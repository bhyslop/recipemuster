#!/bin/sh
echo "RBNI: Beginning network info script"

set -e

# Validate required environment variables
: ${RBM_MONIKER:?}              && echo "RBNI: RBM_MONIKER              = ${RBM_MONIKER}"
: ${RBM_ENCLAVE_NETWORK:?}      && echo "RBNI: RBM_ENCLAVE_NETWORK      = ${RBM_ENCLAVE_NETWORK}"
: ${RBM_SENTRY_CONTAINER:?}     && echo "RBNI: RBM_SENTRY_CONTAINER     = ${RBM_SENTRY_CONTAINER}"
: ${RBM_BOTTLE_CONTAINER:?}     && echo "RBNI: RBM_BOTTLE_CONTAINER     = ${RBM_BOTTLE_CONTAINER}"

echo "RBNI: Network Configuration:"
echo "================================"

echo "RBNI: Enclave Network:"
podman network inspect ${RBM_ENCLAVE_NETWORK} --format 'Gateway: {{.Subnets.0.Gateway}}, Subnet: {{.Subnets.0.Subnet}}, Bridge: {{.NetworkInterface}}'
echo ""

echo "RBNI: SENTRY Container Networks:"
podman inspect ${RBM_SENTRY_CONTAINER} --format '{{range $net, $conf := .NetworkSettings.Networks}}{{$net}}: {{$conf.IPAddress}}{{println}}{{end}}'

echo "RBNI: BOTTLE Container Networks:"
podman inspect ${RBM_BOTTLE_CONTAINER} --format '{{range $net, $conf := .NetworkSettings.Networks}}{{$net}}: {{$conf.IPAddress}}{{println}}{{end}}' 2>/dev/null || echo "(not yet created)"
echo ""

echo "RBNI: Container veth interfaces:"
ip link show | grep -E "veth[0-9a-f]+" | grep -v "@" || echo "No veth interfaces found"
echo ""

echo "RBNI: TC filters on container interfaces:"
for veth in $(ip link show | grep -oE "veth[0-9a-f]+" | sort -u); do
    if tc qdisc show dev $veth 2>/dev/null | grep -q clsact; then
        echo "  $veth:"
        tc filter show dev $veth ingress 2>/dev/null | grep -E "bpf|direct-action" | sed 's/^/    /'
        tc filter show dev $veth egress  2>/dev/null | grep -E "bpf|direct-action" | sed 's/^/    /'
    fi
done

echo ""
echo "RBNI: Network info complete"
