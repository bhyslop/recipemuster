# Ifrit — in-bottle attack binary for Recipe Bottle crucible testing
#
# Single-stage build: Rust toolchain stays in image (test vessel, size
# irrelevant, iteration speed matters). Layer-cached dependency compilation
# means source-only changes rebuild just the ifrit crate.
#
# Build context: Tools/rbk/rbid/ (the ifrit crate directory)

ARG RBF_IMAGE_1
FROM ${RBF_IMAGE_1}

# Avoid prompts from apt
ENV DEBIAN_FRONTEND=noninteractive

# Install network attack tools (load-bearing for security boundary probes)
# socat:          Echo service on port 8888 (entry port security tests)
# dnsutils:       dig/nslookup for DNS security tests
# netcat-openbsd: nc for TCP connectivity tests
# traceroute:     ICMP path verification tests
# strace:         Syscall boundary mapping from inside the container
RUN apt-get update && apt-get install -y --no-install-recommends \
    socat           \
    dnsutils        \
    netcat-openbsd  \
    traceroute      \
    strace          \
    && rm -rf /var/lib/apt/lists/*

# Layer-cached dependency build: copy manifests first, dummy compile
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs && \
    cargo build --release && \
    rm -rf src target/release/deps/rbid-* target/release/.fingerprint/rbid-*

# Real source build
COPY src/ src/
RUN cargo build --release

# Install binary to standard location
RUN cp target/release/rbid /usr/local/bin/rbid

# Create unprivileged ifrit user
RUN useradd -m -s /bin/bash ifrit

# Keep the container alive with echo service on port 8888
CMD ["socat", "TCP-LISTEN:8888,fork,reuseaddr", "EXEC:cat"]
