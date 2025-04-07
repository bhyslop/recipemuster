#!/bin/bash
# vm_manifest_extractor.sh

# Function to display help
show_help() {
  echo "vm_manifest_extractor.sh - Extract manifest information for Podman VM images"
  echo ""
  echo "Purpose: Extract manifest information specifically for Podman VM images"
  echo "This script is tailored for the unique structure of Podman VM images and may"
  echo "not work correctly with other container image types."
  echo ""
  echo "Usage: $(basename "$0") <image_base> <image_ref> <crane_executable> <temp_dir> <output_file>"
  echo ""
  echo "Parameters:"
  echo "  image_base        - Registry and repository (e.g., quay.io/podman/machine-os-wsl)"
  echo "  image_ref         - Tag or digest (e.g., v4.6.1 or sha256:abc123...)"
  echo "  crane_executable  - Path to crane executable"
  echo "  temp_dir          - Existing empty directory for temporary files"
  echo "  output_file       - Path to output JSON file"
  echo ""
  echo "Output JSON fields:"
  echo "  image_base        - The registry and repository part"
  echo "  image_ref         - The reference specifier provided (tag or digest)"
  echo "  full_reference    - Complete reference combining base and ref"
  echo "  index_digest      - Top-level manifest hash (with algorithm prefix)"
  echo "  digest_reference  - Immutable reference using the index digest"
  echo "  blob_filter_pattern - Concatenated blob digests with \| separators for filtering"
  echo ""
  echo "Example:"
  echo "  $(basename "$0") quay.io/podman/machine-os-wsl v4.6.1 /usr/local/bin/crane /tmp/workdir output.json"
  echo ""
  echo "Note: This script is specifically designed for Podman VM images and may produce"
  echo "errors if used with container images that have incompatible manifest structures."
  exit 1
}

# Check arguments
if [ "$#" -ne 5 ]; then
  show_help
fi

# Parse arguments
IMAGE_BASE="$1"
IMAGE_REF="$2"
CRANE="$3"
TEMP_DIR="$4"
OUTPUT_FILE="$5"

# Validate crane executable
if ! command -v "$CRANE" &> /dev/null; then
  echo "Error: Crane executable '$CRANE' not found or not executable."
  exit 1
fi

# Construct full reference
if [[ "$IMAGE_REF" == sha256:* ]]; then
  FULL_REFERENCE="${IMAGE_BASE}@${IMAGE_REF}"
else
  FULL_REFERENCE="${IMAGE_BASE}:${IMAGE_REF}"
fi

echo "Extracting manifest information for: $FULL_REFERENCE"

# Get top-level manifest digest
INDEX_DIGEST_FILE="${TEMP_DIR}/index_digest.txt"
echo "Getting top-level manifest digest..."
if ! "$CRANE" digest "$FULL_REFERENCE" > "$INDEX_DIGEST_FILE" 2>/dev/null; then
  echo "Error: Failed to retrieve index digest for $FULL_REFERENCE"
  exit 1
fi
INDEX_DIGEST=$(cat "$INDEX_DIGEST_FILE")
echo "Index digest: $INDEX_DIGEST"

# Construct digest reference
DIGEST_REFERENCE="${IMAGE_BASE}@${INDEX_DIGEST}"
echo "Digest reference: $DIGEST_REFERENCE"

# Retrieve the index manifest
INDEX_FILE="${TEMP_DIR}/index.json"
echo "Retrieving index manifest..."
if ! "$CRANE" manifest "$FULL_REFERENCE" > "$INDEX_FILE" 2>/dev/null; then
  echo "Error: Failed to retrieve manifest for $FULL_REFERENCE"
  exit 1
fi

# Extract all platform manifests
ALL_PLATFORM_DIGESTS="${TEMP_DIR}/platform_digests.txt"
echo "Extracting platform digests..."
if ! jq -r '.manifests[].digest' < "$INDEX_FILE" > "$ALL_PLATFORM_DIGESTS" 2>/dev/null; then
  echo "Error: Failed to extract platform digests from manifest"
  echo "This may indicate an incompatible manifest structure."
  exit 1
fi

# Check if we have platform digests
if [ ! -s "$ALL_PLATFORM_DIGESTS" ]; then
  echo "Error: No platform manifests found in the index"
  echo "This may indicate an incompatible manifest structure."
  exit 1
fi

# Create blob filter pattern
BLOB_FILTER="${TEMP_DIR}/blob_filter.txt"
echo "Creating blob filter pattern from platform manifests..."

for digest in $(cat "$ALL_PLATFORM_DIGESTS"); do
  echo "Processing platform manifest: $digest"
  if ! "$CRANE" manifest "${FULL_REFERENCE}@${digest}" | jq -r '.layers[].digest' 2>/dev/null >> "${TEMP_DIR}/all_layers.txt"; then
    echo "Error: Could not extract layers from manifest ${digest}"
    exit 1
  fi
done

sed 's/sha256://g' "${TEMP_DIR}/all_layers.txt" | sort | uniq | tr '\n' '|' | sed 's/|$//' | sed 's/|/\\|/g' > "$BLOB_FILTER"

BLOB_FILTER_PATTERN=$(cat "$BLOB_FILTER")

# If blob filter pattern is empty, that's an error
if [ -z "$BLOB_FILTER_PATTERN" ]; then
  echo "Error: Failed to extract blob digests from platform manifests"
  echo "This may indicate an incompatible manifest structure."
  exit 1
fi
echo "Blob filter pattern created successfully"

# Create JSON output
echo "Creating JSON output..."
jq -n \
  --arg image_base "$IMAGE_BASE" \
  --arg image_ref "$IMAGE_REF" \
  --arg full_reference "$FULL_REFERENCE" \
  --arg index_digest "$INDEX_DIGEST" \
  --arg digest_reference "$DIGEST_REFERENCE" \
  --arg blob_filter_pattern "$BLOB_FILTER_PATTERN" \
  '{
    image_base: $image_base,
    image_ref: $image_ref,
    full_reference: $full_reference,
    index_digest: $index_digest,
    digest_reference: $digest_reference,
    blob_filter_pattern: $blob_filter_pattern
  }' > "$OUTPUT_FILE"

if [ $? -ne 0 ]; then
  echo "Error: Failed to write JSON output to $OUTPUT_FILE"
  exit 1
fi

echo "Successfully extracted manifest information to $OUTPUT_FILE"
exit 0
