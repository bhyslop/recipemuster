# Use a specific version of Ubuntu for better reproducibility
FROM ubuntu:22.04

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Update and install essential packages
RUN apt-get update && apt-get install -y                                       \
    curl                `# For making HTTP requests and basic diagnostics`     \
    dnsmasq             `# For DNS forwarding and DHCP server`                 \
    iproute2            `# For network configuration`                          \
    iptables            `# For configuring firewall rules`                     \
    iputils-ping        `# For basic network connectivity tests`               \
    socat               `# For multipurpose relay (essential for proxying)`    \
    tcpdump             `# For packet capture and analysis`                    \
    && rm -rf /var/lib/apt/lists/*

# Keep the container alive
CMD ["tail", "-f", "/dev/null"]
