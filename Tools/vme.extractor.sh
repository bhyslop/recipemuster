#!/bin/bash
# vm_manifest_extractor.sh - Extracts manifest information for Podman VM images

# Function for logging messages to stderr
vme_log() {
  echo "$@" >&2
}

# Function to display help
show_help() {
  vme_log "vm_manifest_extractor.sh - Extract manifest information for Podman VM images"
  vme_log ""
  vme_log "Purpose: Extract manifest information specifically for Podman VM images"
  vme_log "This script is tailored for the unique structure of Podman VM images and may"
  vme_log "not work correctly with other container image types."
  vme_log ""
  vme_log "Usage: $(basename "$0") <image_base> <image_ref> <crane_executable>"
  vme_log ""
  vme_log "Parameters:"
  vme_log "  image_base        - Registry and repository (e.g., quay.io/podman/machine-os-wsl)"
  vme_log "  image_ref         - Tag or digest (e.g., v4.6.1 or sha256:abc123...)"
  vme_log "  crane_executable  - Path to crane executable"
  vme_log ""
  vme_log "Output: JSON object with manifest information is printed to stdout"
  vme_log ""
  vme_log "Output JSON fields:"
  vme_log "  image_base        - The registry and repository part"
  vme_log "  image_ref         - The reference specifier provided (tag or digest)"
  vme_log "  full_reference    - Complete reference combining base and ref"
  vme_log "  index_digest      - Top-level manifest hash (with algorithm prefix)"
  vme_log "  digest_reference  - Immutable reference using the index digest"
  vme_log "  blob_filter_pattern - Concatenated blob digests with \| separators for filtering"
  vme_log ""
  vme_log "Example:"
  vme_log "  $(basename "$0") quay.io/podman/machine-os-wsl v4.6.1 /usr/local/bin/crane > output.json"
  vme_log ""
  vme_log "Note: This script is specifically designed for Podman VM images and may produce"
  vme_log "errors if used with container images that have incompatible manifest structures."
  exit 1
}

# Function to handle errors
error_exit() {
  vme_log "ERROR: $1"
  exit 1
}

# Check arguments
if [ "$#" -ne 3 ]; then
  show_help
fi

# Parse arguments
IMAGE_BASE="$1"
IMAGE_REF="$2"
CRANE="$3"

command -v jq       &> /dev/null || error_exit "jq executable not found. Please install jq to process JSON manifests."
command -v "$CRANE" &> /dev/null || error_exit "Crane executable '$CRANE' not found or not executable."


# Create a temporary directory with datestamp and random component
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
RANDOM_SUFFIX=$(tr -dc 'a-zA-Z0-9' < /dev/urandom | fold -w 8 | head -n 1)
TEMP_DIR="/tmp/vme_${TIMESTAMP}_${RANDOM_SUFFIX}"
mkdir -p "$TEMP_DIR" || error_exit "Failed to create temporary directory $TEMP_DIR"
vme_log "Created temporary directory: $TEMP_DIR"

# Construct full reference
if [[ "$IMAGE_REF" == sha256:* ]]; then
  FULL_REFERENCE="${IMAGE_BASE}@${IMAGE_REF}"
else
  FULL_REFERENCE="${IMAGE_BASE}:${IMAGE_REF}"
fi

vme_log "Extracting manifest information for: $FULL_REFERENCE"

# Get top-level manifest digest
INDEX_DIGEST_FILE="${TEMP_DIR}/index_digest.txt"
vme_log "Getting top-level manifest digest..."
if ! "$CRANE" digest "$FULL_REFERENCE" > "$INDEX_DIGEST_FILE" 2>/dev/null; then
  error_exit "Failed to retrieve index digest for $FULL_REFERENCE"
fi
INDEX_DIGEST=$(cat "$INDEX_DIGEST_FILE")
vme_log "Index digest: $INDEX_DIGEST"

# Construct digest reference
DIGEST_REFERENCE="${IMAGE_BASE}@${INDEX_DIGEST}"
vme_log "Digest reference: $DIGEST_REFERENCE"

# Retrieve the index manifest
INDEX_FILE="${TEMP_DIR}/index.json"
vme_log "Retrieving index manifest..."
if ! "$CRANE" manifest "$FULL_REFERENCE" > "$INDEX_FILE" 2>/dev/null; then
  error_exit "Failed to retrieve manifest for $FULL_REFERENCE"
fi

# Debug - Print the index manifest content
vme_log "Index manifest content:"
vme_log "$(cat "$INDEX_FILE")"

# Extract all platform manifests
ALL_PLATFORM_DIGESTS="${TEMP_DIR}/platform_digests.txt"
vme_log "Extracting platform digests..."
if ! jq -r '.manifests[].digest' < "$INDEX_FILE" > "$ALL_PLATFORM_DIGESTS" 2>/dev/null; then
  error_exit "Failed to extract platform digests from manifest. This may indicate an incompatible manifest structure."
fi

# Check if we have platform digests
if [ ! -s "$ALL_PLATFORM_DIGESTS" ]; then
  error_exit "No platform manifests found in the index. This may indicate an incompatible manifest structure."
fi

# Create blob filter pattern
BLOB_FILTER="${TEMP_DIR}/blob_filter.txt"
vme_log "Creating blob filter pattern from platform manifests..."

for digest in $(cat "$ALL_PLATFORM_DIGESTS"); do
  vme_log "Processing platform manifest: $digest"
  if ! "$CRANE" manifest "${FULL_REFERENCE}@${digest}" | jq -r '.layers[].digest' 2>/dev/null >> "${TEMP_DIR}/all_layers.txt"; then
    error_exit "Could not extract layers from manifest ${digest}"
  fi
done

sed 's/sha256://g' "${TEMP_DIR}/all_layers.txt" | sort | uniq | tr '\n' '|' | sed 's/|$//' | sed 's/|/\\|/g' > "$BLOB_FILTER"

BLOB_FILTER_PATTERN=$(cat "$BLOB_FILTER")

# If blob filter pattern is empty, that's an error
if [ -z "$BLOB_FILTER_PATTERN" ]; then
  error_exit "Failed to extract blob digests from platform manifests. This may indicate an incompatible manifest structure."
fi
vme_log "Blob filter pattern created successfully"

# Create and output JSON
vme_log "Creating JSON output..."
jq -n \
  --arg image_base "$IMAGE_BASE" \
  --arg image_ref "$IMAGE_REF" \
  --arg full_reference "$FULL_REFERENCE" \
  --arg index_digest "$INDEX_DIGEST" \
  --arg digest_reference "$DIGEST_REFERENCE" \
  --arg blob_filter_pattern "$BLOB_FILTER_PATTERN" \
  --arg temp_dir "$TEMP_DIR" \
  --arg short_digest "${INDEX_DIGEST:7:12}" \
  --arg canonical_tag "stash-$(echo $IMAGE_BASE | tr '/' '-')-$IMAGE_REF-${INDEX_DIGEST:7:12}" \
  '{
    image_base: $image_base,
    image_ref: $image_ref,
    full_reference: $full_reference,
    index_digest: $index_digest,
    digest_reference: $digest_reference,
    blob_filter_pattern: $blob_filter_pattern,
    temp_dir: $temp_dir,
    short_digest: $short_digest,
    canonical_tag: $canonical_tag
  }'

if [ $? -ne 0 ]; then
  error_exit "Failed to generate JSON output"
fi

vme_log "Successfully extracted manifest information"
vme_log "Temporary directory location (for debugging): $TEMP_DIR"
exit 0
