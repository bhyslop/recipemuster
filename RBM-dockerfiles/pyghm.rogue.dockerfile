FROM python:3.12.6-slim-bookworm

# Install system dependencies and diagnostic tools
RUN apt-get update && apt-get install -y  \
    iputils-ping                          \
    iproute2                              \
    net-tools                             \
    dnsutils                              \
    curl                                  \
    traceroute                            \
    tcpdump                               \
    nmap                                  \
    && rm -rf /var/lib/apt/lists/*

RUN pip install --no-cache-dir GitPython

# Critical arg/env for distinguishing this instance from others like it (no shared image accidents)
ARG NAMEPLATE_MONIKER
ENV NAMEPLATE_MONIKER=${NAMEPLATE_MONIKER}
# write a file so that following env vars are distinc
RUN echo $NAMEPLATE_MONIKER > /nameplate.txt

# Define build arguments and set environment variables
ARG JUPYTER_PORT
ENV JUPYTER_PORT=${JUPYTER_PORT}

ARG ROGUE_IP
ENV ROGUE_IP=${ROGUE_IP}

ARG ROGUE_WORKDIR
ENV ROGUE_WORKDIR=${ROGUE_WORKDIR}

ARG GUARDED_INTERFACE
ENV GUARDED_INTERFACE=${GUARDED_INTERFACE}

ARG SENTRY_GUARDED_IP
ENV SENTRY_GUARDED_IP=${SENTRY_GUARDED_IP}

ARG GUARDED_NETWORK_SUBNET
ENV GUARDED_NETWORK_SUBNET=${GUARDED_NETWORK_SUBNET}

# Disable IPv6
RUN \
    echo "net.ipv6.conf.all.disable_ipv6 = 1"              >> /etc/sysctl.conf  &&\
    echo "net.ipv6.conf.default.disable_ipv6 = 1"          >> /etc/sysctl.conf  &&\
    echo done

# Set the working directory to the mount point
WORKDIR ${ROGUE_WORKDIR}

# Set the default command to run when the container starts
CMD ["/bin/bash"]

