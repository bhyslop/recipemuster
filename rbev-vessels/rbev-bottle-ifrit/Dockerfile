# Debian bookworm-slim for ifrit bottle (adversarial AI + socat echo service)
ARG RBF_IMAGE_1
FROM ${RBF_IMAGE_1}

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Install ifrit operational packages (--no-install-recommends eliminates ~100 scapy
# recommends: g++, Boost, matplotlib, sympy, tk — and nodejs recommends)
# socat:            Echo service on port 8888 (load-bearing for entry port security tests)
# python3-scapy:    Arbitrary packet construction for adversarial probing
# strace:           Syscall boundary mapping from inside the container
# nodejs + npm:     Claude Code runtime (Node.js application)
# bash:             Claude Code operational dependency
# curl:             Claude Code operational dependency
# git:              Claude Code operational dependency
# dnsutils:         dig/nslookup for DNS security tests
# netcat-openbsd:   nc for TCP connectivity tests
# traceroute:       ICMP path verification tests
RUN apt-get update && apt-get install -y --no-install-recommends \
    socat           \
    python3-scapy   \
    strace          \
    nodejs          \
    npm             \
    bash            \
    curl            \
    git             \
    dnsutils        \
    netcat-openbsd  \
    traceroute      \
    && rm -rf /var/lib/apt/lists/*

# Install Claude Code globally (npm, then clean cache)
RUN npm install -g @anthropic-ai/claude-code --omit=dev \
    && npm cache clean --force

# Disable Claude Code telemetry and error reporting (defense-in-depth:
# sentry DNS/CIDR allowlist would block these anyway, but explicit disable
# avoids noisy connection failures and keeps the ifrit's network clean)
ENV DISABLE_TELEMETRY=1
ENV DISABLE_ERROR_REPORTING=1
ENV CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC=1

# Keep the container alive with a simple echo service on port 8888
CMD ["socat", "TCP-LISTEN:8888,fork,reuseaddr", "EXEC:cat"]
