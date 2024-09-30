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

# Environment variables are now expected to be set at runtime

# Keep the container alive
CMD ["tail", "-f", "/dev/null"]
