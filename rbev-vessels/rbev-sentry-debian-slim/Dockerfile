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

# Bake entrypoint scripts (sentry setup + censer routing)
# Source of truth: Tools/rbk/rbj_sentry.sh, Tools/rbk/rboc_censer.sh
# Changes require deliberate build→vouch cycle
COPY rbj_sentry.sh  /opt/rbk/rbj_sentry.sh
COPY rboc_censer.sh /opt/rbk/rboc_censer.sh
RUN chmod +x /opt/rbk/rbj_sentry.sh /opt/rbk/rboc_censer.sh

# Default: sentry entrypoint (censer overrides via compose command)
CMD ["/opt/rbk/rbj_sentry.sh"]
