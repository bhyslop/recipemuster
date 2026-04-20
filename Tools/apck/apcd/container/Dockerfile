# APCK container — Python 3.11-slim base + transformers/PyTorch CPU + scispaCy + Stanza.
#
# Models are pre-fetched during build so the runtime container can run with
# --network=none. First build is slow (~5–10 min) because of the model pulls;
# the layer cache makes incremental builds quick.

FROM python:3.11-slim AS base

# Locale-stable, no .pyc litter, unbuffered stdio.
ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    PIP_NO_CACHE_DIR=1 \
    PIP_DISABLE_PIP_VERSION_CHECK=1 \
    HF_HOME=/opt/models/hf \
    TRANSFORMERS_CACHE=/opt/models/hf \
    STANZA_RESOURCES_DIR=/opt/models/stanza

# curl + ca-certificates for the en_core_sci_md wheel pull; build-essential
# because scispaCy 0.5.5 depends on nmslib-metabrainz, which has no prebuilt
# arm64 wheel and must compile from source. Build-essential stays in the
# image — kept simple for prototype; multistage trim is a Future-Brad task.
RUN apt-get update && apt-get install -y --no-install-recommends \
        curl \
        ca-certificates \
        build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt/apck

# Install Python deps in two phases so the heavy torch wheel is its own layer.
RUN pip install --upgrade pip
# Pinned to 2.5.1 — last release on the PyTorch CPU index that uses the
# bare version (no '+cpu' suffix). 2.6.0+ switched to '+cpu'-suffixed wheels;
# either family works, but 2.5.1 has the broadest transformers/scispaCy
# compatibility window.
RUN pip install --index-url https://download.pytorch.org/whl/cpu torch==2.5.1

COPY requirements.txt /opt/apck/requirements.txt
RUN pip install -r /opt/apck/requirements.txt

# scispaCy model (en_core_sci_md @ 0.5.4 — the version published against
# scispaCy 0.5.5 / spaCy 3.7).
RUN pip install \
    https://s3-us-west-2.amazonaws.com/ai2-s2-scispacy/releases/v0.5.4/en_core_sci_md-0.5.4.tar.gz

# Pre-fetch HuggingFace + Stanza models into image layers so runtime needs no network.
RUN python -c "from transformers import AutoTokenizer, AutoModelForTokenClassification; \
m='StanfordAIMI/stanford-deidentifier-base'; \
AutoTokenizer.from_pretrained(m); AutoModelForTokenClassification.from_pretrained(m)"

# Download dir is governed by STANZA_RESOURCES_DIR env var (set above).
# stanza.download()'s explicit param is `model_dir`; stanza.Pipeline()'s is
# `dir` — different names for the same concept. Env var sidesteps both.
RUN python -c "import stanza; stanza.download('en', processors='tokenize,pos,lemma,depparse,ner')"

# Application sources last — change-frequency low layers above, high below.
COPY entrypoint.py /opt/apck/entrypoint.py
COPY discerners /opt/apck/discerners

# Non-root runtime user. Pre-existing 'nobody' UID 65534 / GID 65534 keeps
# bind-mount file ownership stable. Models live under /opt/models — owned by
# root but world-readable, fine for inference.
RUN chmod -R a+rX /opt/apck /opt/models

USER nobody:nogroup

ENTRYPOINT ["python", "/opt/apck/entrypoint.py"]
