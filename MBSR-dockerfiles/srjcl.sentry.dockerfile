# Use a minimal base image
FROM alpine:latest

# Install necessary packages
RUN apk update && apk add --no-cache \
    iputils \
    iproute2 \
    net-tools \
    bind-tools \
    curl \
    traceroute \
    tcpdump \
    nmap \
    iptables \
    dnsmasq \
    socat \
  && echo done

# Critical arg/env for distinguishing this instance from others like it (no shared image accidents)
ARG NAMEPLATE_MONIKER
ENV NAMEPLATE_MONIKER=${NAMEPLATE_MONIKER}
# write a file so that following env vars are distinc
RUN echo $NAMEPLATE_MONIKER > /nameplate.txt

# Define build arguments and set environment variables
ARG DNS_SERVER
ENV DNS_SERVER=${DNS_SERVER}

ARG NETWORK_MASK
ENV NETWORK_MASK=${NETWORK_MASK}

ARG HOST_INTERFACE
ENV HOST_INTERFACE=${HOST_INTERFACE}

ARG GUARDED_INTERFACE
ENV GUARDED_INTERFACE=${GUARDED_INTERFACE}

ARG SENTRY_GUARDED_IP
ENV SENTRY_GUARDED_IP=${SENTRY_GUARDED_IP}

ARG ROGUE_IP
ENV ROGUE_IP=${ROGUE_IP}

ARG ROGUE_JUPYTER_PORT
ENV ROGUE_JUPYTER_PORT=${ROGUE_JUPYTER_PORT}

ARG SENTRY_JUPYTER_PORT
ENV SENTRY_JUPYTER_PORT=${SENTRY_JUPYTER_PORT}

ARG GUARDED_NETWORK_SUBNET
ENV GUARDED_NETWORK_SUBNET=${GUARDED_NETWORK_SUBNET}

# Keep the container alive
CMD ["tail", "-f", "/dev/null"]
