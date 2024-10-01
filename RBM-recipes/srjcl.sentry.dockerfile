# Use a minimal base image
FROM alpine:latest

# Define ARG and ENV for RBEV variables with defaults
# (None are currently used in this file, but we'll add them for consistency)
ARG RBEV_SENTRY_JUPYTER_PORT=8000
ENV RBEV_SENTRY_JUPYTER_PORT=${RBEV_SENTRY_JUPYTER_PORT}

ARG RBEV_SENTRY_GUARDED_IP=172.17.0.2
ENV RBEV_SENTRY_GUARDED_IP=${RBEV_SENTRY_GUARDED_IP}

# Hardcode NAMEPLATE_MONIKER
ENV NAMEPLATE_MONIKER=srjcl

# Install necessary packages
RUN apk update && apk add --no-cache \
    iputils                          \
    iproute2                         \
    net-tools                        \
    bind-tools                       \
    curl                             \
    traceroute                       \
    tcpdump                          \
    nmap                             \
    iptables                         \
    dnsmasq                          \
    socat                            \
  && echo done

# write a file with the hardcoded nameplate
RUN echo $NAMEPLATE_MONIKER > /nameplate.txt

# Keep the container alive
CMD ["tail", "-f", "/dev/null"]
