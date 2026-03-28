# Debian bookworm-slim for minimal trust boundary sentry
ARG RBF_IMAGE_1
FROM ${RBF_IMAGE_1}

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Install sentry operational packages + censer diagnostic packages
# (censer container shares this image for trusted network observation)
RUN apt-get update && apt-get install -y \
    bash            `# Shell for sentry setup script`       \
    iptables        `# Firewall rule configuration`         \
    dnsmasq         `# DNS forwarding for enclave`          \
    iproute2        `# Network configuration (ip command)`  \
    iputils-ping    `# ICMP tests (censer role)`            \
    dnsutils        `# dig/nslookup (censer role)`          \
    && rm -rf /var/lib/apt/lists/*

# Pin iptables-legacy backend (nftables not available in slim)
RUN update-alternatives --set iptables /usr/sbin/iptables-legacy

# Keep the container alive
CMD ["tail", "-f", "/dev/null"]
