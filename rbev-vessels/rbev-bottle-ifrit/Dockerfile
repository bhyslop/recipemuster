# Debian bookworm-slim for ifrit bottle (adversarial AI + socat echo service)
ARG RBF_IMAGE_1
FROM ${RBF_IMAGE_1}

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Install ifrit operational packages
# socat:         Echo service on port 8888 (load-bearing for entry port security tests)
# nodejs + npm:  Claude Code runtime (Claude Code is a Node.js application)
# python3-scapy: Arbitrary packet construction for adversarial probing
# strace:        Syscall boundary mapping from inside the container
# bash:          Claude Code operational dependency
# curl:          Claude Code operational dependency
# git:           Claude Code operational dependency
RUN apt-get update && apt-get install -y \
    socat           \
    nodejs          \
    npm             \
    python3-scapy   \
    strace          \
    bash            \
    curl            \
    git             \
    && rm -rf /var/lib/apt/lists/*

# Keep the container alive with a simple echo service on port 8888
CMD ["socat", "TCP-LISTEN:8888,fork,reuseaddr", "EXEC:cat"]
