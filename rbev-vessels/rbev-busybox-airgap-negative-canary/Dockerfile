# Airgap negative canary — this build is designed to FAIL on the airgap pool.
# Without an anchor, FROM resolves to Docker Hub, which is blocked by NO_PUBLIC_EGRESS.
ARG RBF_IMAGE_1
FROM ${RBF_IMAGE_1}

WORKDIR /app
RUN adduser -D appuser
USER appuser
CMD ["echo", "This should never run — airgap canary"]
