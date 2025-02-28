# Use a specific version of Ubuntu for better reproducibility
FROM ubuntu:24.10

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Update and install essential packages
RUN apt-get update && apt-get install -y                                                 \
    apparmor-utils      `# For AppArmor utilities including aa-status`                   \
    curl                `# For making HTTP requests and basic diagnostics`               \
    dnsmasq             `# For DNS forwarding and DHCP server`                           \
    dnsutils            `# For dig and nslookup commands`                                \
    iproute2            `# For network configuration`                                    \
    iptables            `# For configuring firewall rules`                               \
    iptraf-ng           `# For real-time IP LAN monitoring`                              \
    iputils-ping        `# For basic network connectivity tests`                         \
    lsof                `# For listing open files and the processes that opened them`    \
    net-tools           `# For netstat and other networking tools`                       \
    netcat-openbsd      `# For nc command, useful for testing network connections`       \
    procps              `# For ps command to check running processes`                    \
    socat               `# For multipurpose relay (essential for proxying)`              \
    tcpdump             `# For packet capture and analysis`                              \
    traceroute          `# For tracing network routes`                                   \
    vim                 `# For text editing in the container`                            \
    ethtool             `# For querying, controlling network driver/hardware settings`   \
    iputils-arping      `# For ARP probing and network debugging`                        \
    tcptraceroute       `# For TCP-based traceroute utility`                             \
    mtr                 `# For network diagnostic combining ping and traceroute`         \
    tshark              `# For command-line Wireshark packet capture and analysis`       \
    conntrack           `# For connection tracking`                                      \
    iftop               `# For bandwidth monitoring`                                     \
    psmisc              `# For fuser, killall, etc.`                                     \
    ipcalc              `# For IP calculations and subnet info`                          \
    jq                  `# For JSON parsing and manipulation`                            \
    && rm -rf /var/lib/apt/lists/*

# Keep the container alive
CMD ["tail", "-f", "/dev/null"]
