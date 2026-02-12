FROM plantuml/plantuml-server:jetty

# Create a second layer with a build timestamp
# This creates a small layer that will be unique for each build
RUN echo "Build timestamp: $(date -u +'%Y-%m-%d %H:%M:%S UTC')" > /tmp/build-info.txt

EXPOSE 8080
