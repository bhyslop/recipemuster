FROM python:3.10-slim

# Install system dependencies and diagnostic tools
RUN apt-get update && apt-get install -y                      \
    iputils-ping         `#For network connectivity tests`    \
    iproute2             `#For network configuration`         \
    net-tools                                                 \
    dnsutils                                                  \
    curl                 `#For making HTTP requests`          \
    traceroute                                                \
    tcpdump              `#For packet analysis`               \
    nmap                 `#For network discovery`             \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js
RUN apt-get update && apt-get install -y curl                  &&\
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash -  &&\
    apt-get install -y nodejs                                  &&\
    npm install -g npm@latest

# Install Python dependencies
RUN pip install --no-cache-dir           \
    jupyter                              \
    jupyterlab                           \
    notebook                             \
    ipython                              \
    matplotlib                           \
    seaborn                              \
    scipy                                \
    numpy                                \
    ipywidgets                           \
    ipympl                               \
    jupyterlab-widgets                   \
    scikit-image                         \
    pyttsx3                              \
    folium                               \
    networkx                             \
    plotly                               \
    wordcloud                            \
    opencv-python                        \
    && echo "Done installing Jupyter environment and additional packages"

# The rest of the Dockerfile remains unchanged
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

# Generate Jupyter config and set dark theme using settings override
RUN jupyter lab --generate-config                                                                                                     &&\
    mkdir -p                              /root/.jupyter/lab/user-settings/@jupyterlab/apputils-extension                             &&\
    echo '{"theme": "JupyterLab Dark"}' > /root/.jupyter/lab/user-settings/@jupyterlab/apputils-extension/themes.jupyterlab-settings  &&\
    echo "c.ServerApp.allow_origin = '*'" >> /root/.jupyter/jupyter_server_config.py

# Create a startup script
RUN echo '#!/bin/sh'                                                                     >  /startup.sh  &&\
    echo 'sysctl -p'                                                                    >>  /startup.sh  &&\
    echo 'ip route add default via ${SENTRY_GUARDED_IP}'                                >>  /startup.sh  &&\
    echo 'echo "nameserver ${SENTRY_GUARDED_IP}" > /etc/resolv.conf'                    >>  /startup.sh  &&\
    echo 'echo "Disable nag: Would you like to receive official Jupyter news?"'         >>  /startup.sh  &&\
    echo 'jupyter labextension disable "@jupyterlab/apputils-extension:announcements"'  >>  /startup.sh  &&\
    echo 'jupyter lab'                                        \
         '--ip=0.0.0.0'                                       \
         '--port=${JUPYTER_PORT}'                             \
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
