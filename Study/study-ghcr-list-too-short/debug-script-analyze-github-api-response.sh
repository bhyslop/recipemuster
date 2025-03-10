#!/bin/bash
# Debug script to analyze GitHub API response structure

# Load your credentials
source ../secrets/github-ghcr-play.env

# Create a temp directory if it doesn't exist
mkdir -p ./debug_output

# Fetch the raw API response
echo "Fetching API response..."
curl -s -H "Authorization: token $RBV_PAT" \
     -H 'Accept: application/vnd.github.v3+json' \
     'https://api.github.com/user/packages/container/recipemuster/versions?per_page=100' \
     > ./debug_output/raw_response.json

# Check if we got a valid JSON response
if ! jq empty ./debug_output/raw_response.json 2>/dev/null; then
    echo "Error: Invalid JSON response received"
    cat ./debug_output/raw_response.json
    exit 1
fi

# Extract basic info about the response
echo -e "\nBasic response info:"
echo "Total items: $(jq '. | length' ./debug_output/raw_response.json)"

# Check the structure of the first item to understand the schema
echo -e "\nStructure of first item:"
jq '.[0] | keys' ./debug_output/raw_response.json

# Check if metadata.container.tags exists and its structure
echo -e "\nChecking for tags structure:"
jq '.[0].metadata.container | keys' ./debug_output/raw_response.json

# Count items with tags
echo -e "\nItems with tags: $(jq '[.[] | select(.metadata.container.tags != null)] | length' ./debug_output/raw_response.json)"

# Show the first few items with their IDs and tags
echo -e "\nSample of items with their IDs and tags:"
jq -r '.[:5] | .[] | "ID: \(.id), Tags: \(.metadata.container.tags)"' ./debug_output/raw_response.json

# List all version IDs and tags in a cleaner format
echo -e "\nAll versions with their tags:"
jq -r '.[] | select(.metadata.container.tags != null) | "\(.id) \(.metadata.container.tags)"' ./debug_output/raw_response.json > ./debug_output/all_versions.txt
cat ./debug_output/all_versions.txt

# Count total number of tags
echo -e "\nTotal tag entries: $(jq '[.[] | select(.metadata.container.tags != null) | .metadata.container.tags | length] | add' ./debug_output/raw_response.json)"

# Check if there's pagination info in the response headers
echo -e "\nFetching headers to check for pagination..."
curl -s -I -H "Authorization: token $RBV_PAT" \
     -H 'Accept: application/vnd.github.v3+json' \
     'https://api.github.com/user/packages/container/recipemuster/versions?per_page=100' \
     > ./debug_output/response_headers.txt

grep -i "link:" ./debug_output/response_headers.txt
echo "If no 'Link:' header is shown above, there's likely no pagination in the response"

echo -e "\nDebug complete. Check ./debug_output/ directory for full results."
