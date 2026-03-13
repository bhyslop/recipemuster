# Start with the latest busybox image
FROM busybox:latest

# Set working directory
WORKDIR /app

# Create a non-root user for security
RUN adduser -D appuser

# Switch to non-root user
USER appuser

# Set the entrypoint to show it's running
CMD ["echo", "BusyBox native-verify container is running!"]
