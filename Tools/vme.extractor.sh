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
  vme_log "Usage: $(basename "$0") <registry> <repository> <tag> <crane_executable>"
  vme_log ""
  vme_log "Parameters:"
  vme_log "  registry          - Container registry (e.g., ghcr.io)"
  vme_log "  repository        - Container repository (e.g., bhyslop/recipemuster)"
  vme_log "  tag               - Image tag (e.g., stash-quay.io-podman-machine-os-wsl-5.3-6898117ca935)"
  vme_log "  crane_executable  - Path to crane executable"
  vme_log ""
  vme_log "Output: JSON object with manifest information is printed to stdout"
  vme_log ""
  vme_log "Output JSON fields:"
  vme_log "  registry          - The registry part"
  vme_log "  repository        - The repository part"
  vme_log "  tag               - The tag part"
  vme_log "  full_reference    - Complete reference combining registry, repository, and tag"
  vme_log "  index_digest      - Top-level manifest hash (with algorithm prefix)"
  vme_log "  digest_reference  - Immutable reference using the index digest"
  vme_log "  blob_filter_pattern - Concatenated blob digests with \| separators for filtering"
  vme_log "  original_image    - The original image reference extracted from the stash tag (if applicable)"
  vme_log ""
  vme_log "Example:"
  vme_log "  $(basename "$0") ghcr.io bhyslop/recipemuster stash-quay.io-podman-machine-os-wsl-5.3-6898117ca935 /usr/local/bin/crane > output.json"
  exit 1
}

# Function to handle errors
error_exit() {
  vme_log "ERROR: $1"
  exit 1
}

# Check arguments
if [ "$#" -ne 4 ]; then
  show_help
fi

# Parse arguments
REGISTRY="$1"
REPOSITORY="$2"
TAG="$3"
CRANE="$4"

command -v jq       &> /dev/null || error_exit "jq executable not found. Please install jq to process JSON manifests."
command -v "$CRANE" &> /dev/null || error_exit "Crane executable '$CRANE' not found or not executable."

# Create a temporary directory with datestamp and random component
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
RANDOM_SUFFIX=$(tr -dc 'a-zA-Z0-9' < /dev/urandom | fold -w 8 | head -n 1)
TEMP_DIR="/tmp/vme_${TIMESTAMP}_${RANDOM_SUFFIX}"
mkdir -p "$TEMP_DIR" || error_exit "Failed to create temporary directory $TEMP_DIR"
vme_log "Created temporary directory: $TEMP_DIR"

# Construct full reference
FULL_REFERENCE="${REGISTRY}/${REPOSITORY}:${TAG}"

vme_log "Extracting manifest information for: $FULL_REFERENCE"

# Check for stash tag and extract original image information
ORIGINAL_IMAGE=""
if [[ "$TAG" == stash-* ]]; then
  # Extract original image information from the stash tag
  # Format: stash-registry-repository-tag-digest
  # Example: stash-quay.io-podman-machine-os-wsl-5.3-6898117ca935
  
  # Remove the "stash-" prefix
  STASH_INFO=${TAG#stash-}
  
  # Extract components - this is a simplified approach and may need refinement
  # based on how complex your tag format is
  if [[ "$STASH_INFO" =~ (.*)-([^-]+)-([^-]+)$ ]]; then
    ORIG_PATH="${BASH_REMATCH[1]}"
    ORIG_TAG="${BASH_REMATCH[2]}"
    ORIG_DIGEST="${BASH_REMATCH[3]}"
    
    # Replace hyphens with slashes in the path except for registry domain hyphens
    # This is a simplified approach and may need refinement
    ORIG_PATH_FIXED=$(echo "$ORIG_PATH" | sed 's/-/\//g')
    
    ORIGINAL_IMAGE="${ORIG_PATH_FIXED}:${ORIG_TAG}"
    vme_log "Extracted original image reference: $ORIGINAL_IMAGE"
  else
    vme_log "Warning: Could not extract original image information from tag"
  fi
fi

# Get top-level manifest digest
INDEX_DIGEST_FILE="${TEMP_DIR}/index_digest.txt"
vme_log "Getting top-level manifest digest..."
if ! "$CRANE" digest "$FULL_REFERENCE" > "$INDEX_DIGEST_FILE" 2>/dev/null; then
  error_exit "Failed to retrieve index digest for $FULL_REFERENCE"
fi
INDEX_DIGEST=$(cat "$INDEX_DIGEST_FILE")
vme_log "Index digest: $INDEX_DIGEST"

# Construct digest reference
DIGEST_REFERENCE="${REGISTRY}/${REPOSITORY}@${INDEX_DIGEST}"
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
  --arg registry "$REGISTRY" \
  --arg repository "$REPOSITORY" \
  --arg tag "$TAG" \
  --arg full_reference "$FULL_REFERENCE" \
  --arg index_digest "$INDEX_DIGEST" \
  --arg digest_reference "$DIGEST_REFERENCE" \
  --arg blob_filter_pattern "$BLOB_FILTER_PATTERN" \
  --arg vm_temp_dir "$TEMP_DIR" \
  --arg short_digest "${INDEX_DIGEST:7:12}" \
  --arg original_image "$ORIGINAL_IMAGE" \
  --arg canonical_tag "stash-$(echo $REGISTRY | tr '/' '-')-$(echo $REPOSITORY | tr '/' '-')-$TAG-${INDEX_DIGEST:7:12}" \
  '{
    registry: $registry,
    repository: $repository,
    tag: $tag,
    full_reference: $full_reference,
    index_digest: $index_digest,
    digest_reference: $digest_reference,
    blob_filter_pattern: $blob_filter_pattern,
    vm_temp_dir: $vm_temp_dir,
    short_digest: $short_digest,
    original_image: $original_image,
    canonical_tag: $canonical_tag
  }'

if [ $? -ne 0 ]; then
  error_exit "Failed to generate JSON output"
fi

vme_log "Successfully extracted manifest information"
vme_log "Temporary directory location (for debugging): $TEMP_DIR"
exit 0

