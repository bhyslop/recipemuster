#!/bin/sh
set -e

: ${RBRR_BOTTLE_UID:?}             && echo "RBSp0: RBRR_BOTTLE_UID             = ${RBRR_BOTTLE_UID}"
: ${RBRR_DNS_SERVER:?}             && echo "RBSp0: RBRR_DNS_SERVER             = ${RBRR_DNS_SERVER}"
: ${RBRN_ENTRY_ENABLED:?}          && echo "RBSp0: RBRN_ENTRY_ENABLED          = ${RBRN_ENTRY_ENABLED}"
: ${RBRN_ENTRY_PORT_WORKSTATION:?} && echo "RBSp0: RBRN_ENTRY_PORT_WORKSTATION = ${RBRN_ENTRY_PORT_WORKSTATION}"
: ${RBRN_ENTRY_PORT_ENCLAVE:?}     && echo "RBSp0: RBRN_ENTRY_PORT_ENCLAVE     = ${RBRN_ENTRY_PORT_ENCLAVE}"
: ${RBRN_UPLINK_DNS_ENABLED:?}     && echo "RBSp0: RBRN_UPLINK_DNS_ENABLED     = ${RBRN_UPLINK_DNS_ENABLED}"
: ${RBRN_UPLINK_ACCESS_ENABLED:?}  && echo "RBSp0: RBRN_UPLINK_ACCESS_ENABLED  = ${RBRN_UPLINK_ACCESS_ENABLED}"
: ${RBRN_UPLINK_DNS_GLOBAL:?}      && echo "RBSp0: RBRN_UPLINK_DNS_GLOBAL      = ${RBRN_UPLINK_DNS_GLOBAL}"
: ${RBRN_UPLINK_ACCESS_GLOBAL:?}   && echo "RBSp0: RBRN_UPLINK_ACCESS_GLOBAL   = ${RBRN_UPLINK_ACCESS_GLOBAL}"
: ${RBRN_UPLINK_ALLOWED_CIDRS:?}   && echo "RBSp0: RBRN_UPLINK_ALLOWED_CIDRS   = ${RBRN_UPLINK_ALLOWED_CIDRS}"
: ${RBRN_UPLINK_ALLOWED_DOMAINS:?} && echo "RBSp0: RBRN_UPLINK_ALLOWED_DOMAINS = ${RBRN_UPLINK_ALLOWED_DOMAINS}"

echo "RBSp1: Phase 1 - IPTables Initialization"

echo "RBSp1: Flushing existing rules"
iptables -F        || exit 10
iptables -t nat -F || exit 10

echo "RBSp1: Setting default policies to DROP"
iptables -P INPUT DROP   || exit 10
iptables -P FORWARD DROP || exit 10
iptables -P OUTPUT DROP  || exit 10

echo "RBSp1: Accepting loopback traffic"
iptables -I INPUT -i lo -j ACCEPT || exit 10

echo "RBSp1: Setting up connection tracking"
iptables -A INPUT   -m state --state RELATED,ESTABLISHED -j ACCEPT || exit 10
iptables -A FORWARD -m state --state RELATED,ESTABLISHED -j ACCEPT || exit 10
iptables -A OUTPUT  -m state --state RELATED,ESTABLISHED -j ACCEPT || exit 10

echo "RBSp1: Creating RBM chains"
iptables -N RBM-INGRESS || exit 10
iptables -N RBM-EGRESS  || exit 10
iptables -N RBM-FORWARD || exit 10

echo "RBSp1: Setting up chain jumps"
iptables -A INPUT   -j RBM-INGRESS || exit 10
iptables -A OUTPUT  -j RBM-EGRESS  || exit 10
iptables -A FORWARD -j RBM-FORWARD || exit 10

echo "RBSp2: Phase 2 - Port Setup"
if [ "${RBRN_ENTRY_ENABLED}" = "true" ]; then
    echo "RBSp2: Configuring service port access"
    iptables -A RBM-INGRESS -p tcp --dport ${RBRN_ENTRY_PORT_WORKSTATION} -j ACCEPT || exit 20
    iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -p tcp --sport ${RBRN_ENTRY_PORT_ENCLAVE} -j ACCEPT || exit 20
else
    echo "RBSp2: Port access disabled"
fi

echo "RBSp3: Phase 3 - Access Setup"
if [ "${RBRN_UPLINK_ACCESS_ENABLED}" = "false" ]; then
    echo "RBSp3: Blocking all bottle network access"
    iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -j DROP || exit 30
else
    echo "RBSp3: Configuring network access"
    
    echo "RBSp3: Allowing DNS server access for bottle"
    if [ -n "${RBRR_DNS_SERVER}" ]; then
        iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -p udp -d ${RBRR_DNS_SERVER} --dport 53 -j ACCEPT || exit 30
        iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -p tcp -d ${RBRR_DNS_SERVER} --dport 53 -j ACCEPT || exit 30
    fi

    if [ "${RBRN_UPLINK_ACCESS_GLOBAL}" = "true" ]; then
        echo "RBSp3: Enabling global access for bottle"
        iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -j ACCEPT || exit 31
    else
        echo "RBSp3: Configuring CIDR-based access"
        for cidr in ${RBRN_UPLINK_ALLOWED_CIDRS}; do
            echo "RBSp3: Allowing access to ${cidr}"
            iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -d ${cidr} -j ACCEPT || exit 32
        done
        echo "RBSp3: Dropping all other bottle traffic"
        iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -j DROP || exit 30
    fi
fi

echo "RBSp4: Phase 4 - DNS Configuration"
if [ "${RBRN_UPLINK_DNS_ENABLED}" = "false" ]; then
    echo "RBSp4: Blocking all DNS traffic from bottle"
    iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -p udp --dport 53 -j DROP || exit 40
    iptables -A RBM-EGRESS -m owner --uid-owner ${RBRR_BOTTLE_UID} -p tcp --dport 53 -j DROP || exit 40
else
    echo "RBSp4: Skipping DNS connectivity test (iptables rules already applied)"
    if [ -n "${RBRR_DNS_SERVER}" ]; then
        echo "RBSp4: DNS server configured: ${RBRR_DNS_SERVER}"
    fi

    echo "RBSp4: Configuring dnsmasq"
    echo "interface=*"                                     > /etc/dnsmasq.conf || exit 41
    echo "listen-address=127.0.0.1"                       >> /etc/dnsmasq.conf || exit 41
    echo "bind-interfaces"                                >> /etc/dnsmasq.conf || exit 41
    echo "port=53"                                        >> /etc/dnsmasq.conf || exit 41
    echo "no-dhcp-interface=*"                            >> /etc/dnsmasq.conf || exit 41
    echo "cache-size=1000"                                >> /etc/dnsmasq.conf || exit 41
    echo "min-cache-ttl=600"                              >> /etc/dnsmasq.conf || exit 41
    echo "max-cache-ttl=3600"                             >> /etc/dnsmasq.conf || exit 41
    echo "no-resolv"                                      >> /etc/dnsmasq.conf || exit 41
    echo "strict-order"                                   >> /etc/dnsmasq.conf || exit 41
    echo "log-queries=extra"                              >> /etc/dnsmasq.conf || exit 41
    echo "log-facility=/var/log/dnsmasq.log"              >> /etc/dnsmasq.conf || exit 41
    echo "log-dhcp"                                       >> /etc/dnsmasq.conf || exit 41
    echo "log-debug"                                      >> /etc/dnsmasq.conf || exit 41
    echo "log-async=20"                                   >> /etc/dnsmasq.conf || exit 41

    if [ "${RBRN_UPLINK_DNS_GLOBAL}" = "true" ]; then
        echo "RBSp4: Enabling global DNS resolution"
        echo "server=${RBRR_DNS_SERVER}" >> /etc/dnsmasq.conf || exit 41
    else
        echo "RBSp4: Configuring domain-based DNS filtering"
        for domain in ${RBRN_UPLINK_ALLOWED_DOMAINS}; do
            echo "server=/${domain}/${RBRR_DNS_SERVER}" >> /etc/dnsmasq.conf || exit 41
        done
        echo "address=/#/" >> /etc/dnsmasq.conf || exit 41
    fi

    echo "RBSp4: DNSMASQ DEBUG - Checking for existing dnsmasq processes before start"
    echo "RBSp4: DNSMASQ DEBUG - Current process list:"
    ps aux | grep dnsmasq || echo "RBSp4: DNSMASQ DEBUG - No dnsmasq processes found"
    
    echo "RBSp4: DNSMASQ DEBUG - Checking for zombie dnsmasq processes:"
    ps aux | grep dnsmasq | grep -E "(Z|defunct)" || echo "RBSp4: DNSMASQ DEBUG - No zombie dnsmasq processes found"
    
    echo "RBSp4: DNSMASQ DEBUG - Checking current socket bindings:"
    netstat -tlnp | grep :53 || echo "RBSp4: DNSMASQ DEBUG - No processes listening on port 53"
    
    echo "RBSp4: DNSMASQ DEBUG - Killing any existing dnsmasq processes"
    pkill dnsmasq || echo "RBSp4: DNSMASQ DEBUG - No dnsmasq processes to kill"
    sleep 2
    
    echo "RBSp4: DNSMASQ DEBUG - Process list after kill attempt:"
    ps aux | grep dnsmasq || echo "RBSp4: DNSMASQ DEBUG - No dnsmasq processes remaining"
    
    echo "RBSp4: DNSMASQ DEBUG - Starting dnsmasq with debug output"
    echo "RBSp4: DNSMASQ DEBUG - dnsmasq configuration:"
    cat /etc/dnsmasq.conf
    
    echo "RBSp4: Starting dnsmasq"
    echo "RBSp4: DNSMASQ DEBUG - Shell PID: $$"
    echo "RBSp4: DNSMASQ DEBUG - Process tree before dnsmasq start:"
    ps -ef | grep -E "($$|dnsmasq)" || echo "RBSp4: DNSMASQ DEBUG - No relevant processes found"
    
    dnsmasq --no-daemon &
    DNSMASQ_PID=$!
    echo "RBSp4: DNSMASQ DEBUG - Started dnsmasq with PID: $DNSMASQ_PID"
    echo "RBSp4: DNSMASQ DEBUG - Background job status:"
    jobs || echo "RBSp4: DNSMASQ DEBUG - No background jobs"
    
    sleep 2
    echo "RBSp4: DNSMASQ DEBUG - Checking if dnsmasq process is still running:"
    if kill -0 $DNSMASQ_PID 2>/dev/null; then
        echo "RBSp4: DNSMASQ DEBUG - dnsmasq PID $DNSMASQ_PID is still running"
    else
        echo "RBSp4: DNSMASQ DEBUG - dnsmasq PID $DNSMASQ_PID is NOT running"
    fi
    
    echo "RBSp4: DNSMASQ DEBUG - Process tree after dnsmasq start:"
    ps -ef | grep -E "($$|dnsmasq)" || echo "RBSp4: DNSMASQ DEBUG - No relevant processes found"
    
    echo "RBSp4: DNSMASQ DEBUG - dnsmasq started, checking process state"
    echo "RBSp4: DNSMASQ DEBUG - New dnsmasq process info:"
    ps aux | grep dnsmasq | grep -v grep || echo "RBSp4: DNSMASQ DEBUG - No dnsmasq process found after start"
    
    echo "RBSp4: DNSMASQ DEBUG - Checking process states in detail:"
    ps -o pid,ppid,state,comm,args | grep -E "(PID|dnsmasq)" || echo "RBSp4: DNSMASQ DEBUG - No detailed process info"
    
    echo "RBSp4: DNSMASQ DEBUG - Checking socket bindings after start:"
    netstat -tlnp | grep :53 || echo "RBSp4: DNSMASQ DEBUG - No socket bindings found"
    
    echo "RBSp4: DNSMASQ DEBUG - Testing dnsmasq connectivity as root:"
    echo "RBSp4: DNSMASQ DEBUG - Current user: $(whoami), UID: $(id -u)"
    echo "RBSp4: DNSMASQ DEBUG - Testing basic connectivity with nc:"
    timeout 2 nc -zv 127.0.0.1 53 > /tmp/nc_test.log 2>&1
    NC_EXIT_CODE=$?
    echo "RBSp4: DNSMASQ DEBUG - nc test exit code: $NC_EXIT_CODE"
    echo "RBSp4: DNSMASQ DEBUG - nc test output:"
    cat /tmp/nc_test.log || echo "RBSp4: DNSMASQ DEBUG - No nc test output"
    
    echo "RBSp4: DNSMASQ DEBUG - Testing with dig:"
    timeout 5 dig @127.0.0.1 anthropic.com > /tmp/dnsmasq_test.log 2>&1
    DIG_EXIT_CODE=$?
    echo "RBSp4: DNSMASQ DEBUG - dig test exit code: $DIG_EXIT_CODE"
    echo "RBSp4: DNSMASQ DEBUG - dnsmasq test output:"
    cat /tmp/dnsmasq_test.log || echo "RBSp4: DNSMASQ DEBUG - No test output available"
    
    if [ $NC_EXIT_CODE -eq 0 ]; then
        echo "RBSp4: DNSMASQ DEBUG - SUCCESS: Port 53 is reachable"
    else
        echo "RBSp4: DNSMASQ DEBUG - FAILURE: Port 53 is NOT reachable"
        echo "RBSp4: DNSMASQ DEBUG - Checking if dnsmasq is still running after connectivity test:"
        if kill -0 $DNSMASQ_PID 2>/dev/null; then
            echo "RBSp4: DNSMASQ DEBUG - dnsmasq PID $DNSMASQ_PID is still running after test"
        else
            echo "RBSp4: DNSMASQ DEBUG - dnsmasq PID $DNSMASQ_PID is NOT running after test"
        fi
    fi
    
    echo "RBSp4: DNSMASQ DEBUG - Recent dnsmasq log entries:"
    tail -n 10 /var/log/dnsmasq.log || echo "RBSp4: DNSMASQ DEBUG - No dnsmasq log entries"

    echo "RBSp4: Setting up DNS interception"
    iptables -A RBM-EGRESS    -m owner --uid-owner ${RBRR_BOTTLE_UID} -p udp --dport 53 -d 127.0.0.1 -j ACCEPT    || exit 43
    iptables -A RBM-EGRESS    -m owner --uid-owner ${RBRR_BOTTLE_UID} -p tcp --dport 53 -d 127.0.0.1 -j ACCEPT    || exit 43
    iptables -t nat -A OUTPUT -m owner --uid-owner ${RBRR_BOTTLE_UID} -p udp --dport 53 -j REDIRECT --to-ports 53 || exit 43
    iptables -t nat -A OUTPUT -m owner --uid-owner ${RBRR_BOTTLE_UID} -p tcp --dport 53 -j REDIRECT --to-ports 53 || exit 43
    
    echo "RBSp4: DNSMASQ DEBUG - Final iptables rules for DNS:"
    echo "RBSp4: DNSMASQ DEBUG - RBM-EGRESS chain:"
    iptables -L RBM-EGRESS -n || echo "RBSp4: DNSMASQ DEBUG - Failed to list RBM-EGRESS rules"
    echo "RBSp4: DNSMASQ DEBUG - NAT OUTPUT chain:"
    iptables -t nat -L OUTPUT -n || echo "RBSp4: DNSMASQ DEBUG - Failed to list NAT OUTPUT rules"
fi

echo "RBSp5: Security configuration complete"
echo "RBSp5: DNSMASQ DEBUG - Final process state:"
ps aux | grep dnsmasq || echo "RBSp5: DNSMASQ DEBUG - No dnsmasq processes in final state"

echo "RBSp5: DNSMASQ DEBUG - Final detailed process analysis:"
ps -o pid,ppid,state,comm,args | grep -E "(PID|dnsmasq)" || echo "RBSp5: DNSMASQ DEBUG - No detailed process info in final state"

echo "RBSp5: DNSMASQ DEBUG - Final socket bindings:"
netstat -tlnp | grep :53 || echo "RBSp5: DNSMASQ DEBUG - No socket bindings on port 53 in final state"

echo "RBSp5: DNSMASQ DEBUG - Final background job status:"
jobs || echo "RBSp5: DNSMASQ DEBUG - No background jobs in final state"

if [ -n "$DNSMASQ_PID" ]; then
    echo "RBSp5: DNSMASQ DEBUG - Final check of dnsmasq PID $DNSMASQ_PID:"
    if kill -0 $DNSMASQ_PID 2>/dev/null; then
        echo "RBSp5: DNSMASQ DEBUG - dnsmasq PID $DNSMASQ_PID is RUNNING in final state"
    else
        echo "RBSp5: DNSMASQ DEBUG - dnsmasq PID $DNSMASQ_PID is NOT RUNNING in final state"
    fi
else
    echo "RBSp5: DNSMASQ DEBUG - DNSMASQ_PID variable is not set"
fi

echo "RBSp5: DNSMASQ DEBUG - Final dnsmasq log check:"
tail -n 5 /var/log/dnsmasq.log || echo "RBSp5: DNSMASQ DEBUG - No final dnsmasq log entries"

