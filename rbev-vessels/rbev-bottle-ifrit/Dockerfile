# Debian bookworm-slim for ifrit bottle (adversarial AI + socat echo service)
ARG RBF_IMAGE_1
FROM ${RBF_IMAGE_1}

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Install ifrit operational packages (--no-install-recommends eliminates ~100 scapy
# recommends: g++, Boost, matplotlib, sympy, tk)
# socat:         Echo service on port 8888 (load-bearing for entry port security tests)
# python3-scapy: Arbitrary packet construction for adversarial probing
# strace:        Syscall boundary mapping from inside the container
# bash:          Claude Code operational dependency
# curl:          Claude Code operational dependency + native installer fetch
# git:           Claude Code operational dependency
RUN apt-get update && apt-get install -y --no-install-recommends \
    socat           \
    python3-scapy   \
    strace          \
    bash            \
    curl            \
    git             \
    && rm -rf /var/lib/apt/lists/*

# Install Claude Code via native installer (bundles own runtime — no system Node.js)
RUN curl -fsSL https://claude.ai/install.sh | sh
ENV PATH="/root/.claude/bin:${PATH}"

# Keep the container alive with a simple echo service on port 8888
CMD ["socat", "TCP-LISTEN:8888,fork,reuseaddr", "EXEC:cat"]
