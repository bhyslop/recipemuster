FROM python:3.10-slim

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

RUN pip install --no-cache-dir jupyter jupyterlab
RUN pip install --no-cache-dir anthropic

# Critical arg/env for distinguishing this instance from others like it (no shared image accidents)
ARG NAMEPLATE_MONIKER
ENV NAMEPLATE_MONIKER=${NAMEPLATE_MONIKER}
# write a file so that following env vars are distinct
RUN echo $NAMEPLATE_MONIKER > /nameplate.txt

# Environment variables are now expected to be set at runtime
# Disable IPv6
RUN \
    echo "net.ipv6.conf.all.disable_ipv6 = 1"              >> /etc/sysctl.conf  &&\
    echo "net.ipv6.conf.default.disable_ipv6 = 1"          >> /etc/sysctl.conf  &&\
    echo done

# Set the working directory to the mount point
WORKDIR ${RBEV_ROGUE_WORKDIR}

# Generate Jupyter config and set dark theme using settings override
RUN jupyter lab --generate-config                                                                                                     &&\
    mkdir -p                              /root/.jupyter/lab/user-settings/@jupyterlab/apputils-extension                             &&\
    echo '{"theme": "JupyterLab Dark"}' > /root/.jupyter/lab/user-settings/@jupyterlab/apputils-extension/themes.jupyterlab-settings  &&\
    echo "c.ServerApp.allow_origin = '*'" >> /root/.jupyter/jupyter_server_config.py

# Create a startup script
RUN echo '#!/bin/sh'                                                                     >  /startup.sh  &&\
    echo 'sysctl -p'                                                                    >>  /startup.sh  &&\
    echo 'ip route add default via ${RBEV_SENTRY_GUARDED_IP}'                           >>  /startup.sh  &&\
    echo 'echo "nameserver ${RBEV_SENTRY_GUARDED_IP}" > /etc/resolv.conf'               >>  /startup.sh  &&\
    echo 'echo "Disable nag: Would you like to receive official Jupyter news?"'         >>  /startup.sh  &&\
    echo 'jupyter labextension disable "@jupyterlab/apputils-extension:announcements"'  >>  /startup.sh  &&\
    echo 'jupyter lab'                                        \
         '--ip=0.0.0.0'                                       \
         '--port=${RBEV_ROGUE_JUPYTER_PORT}'                  \
         '--no-browser'                                       \
         '--allow-root'                                       \
         '--ServerApp.token=""'                               \
         '--ServerApp.password=""'                            \
         '--ServerApp.authenticate_prometheus=False'          \
         '--ServerApp.base_url=${JUPYTER_BASE_URL}'           \
         '--notebook-dir=${ROGUE_WORKDIR}'                    \
                                                                                        >>  /startup.sh  &&\
    echo 'tail -f /dev/null' >>                                                             /startup.sh  &&\
    chmod +x                                                                                /startup.sh

# Expose the Jupyter port
EXPOSE ${JUPYTER_PORT}

# Use the startup script as the entry point
CMD ["/startup.sh"]
