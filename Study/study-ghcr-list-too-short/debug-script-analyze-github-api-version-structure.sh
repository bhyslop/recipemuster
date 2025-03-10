#!/bin/bash
# Debug script to examine the full structure of version objects

# Load your credentials
source ../secrets/github-ghcr-play.env

# Create a temp directory if it doesn't exist
mkdir -p ./debug_output

# Fetch the raw API response for a couple of specific version IDs
# Let's look at one with a tag and one without
echo "Fetching specific version details..."

# Version with a tag (from your output)
curl -s -H "Authorization: token $RBV_PAT" \
     -H 'Accept: application/vnd.github.v3+json' \
     "https://api.github.com/user/packages/container/recipemuster/versions/370577712" \
     > ./debug_output/version_with_tag.json

# Version without a tag (from your output)
curl -s -H "Authorization: token $RBV_PAT" \
     -H 'Accept: application/vnd.github.v3+json' \
     "https://api.github.com/user/packages/container/recipemuster/versions/370577627" \
     > ./debug_output/version_without_tag.json

# Examine the name fields and other properties
echo -e "\nVersion with tag:"
jq '{id, name, metadata}' ./debug_output/version_with_tag.json

echo -e "\nVersion without tag:"
jq '{id, name, metadata}' ./debug_output/version_without_tag.json

# Let's also check the full list with more fields
echo -e "\nFetching full version list with name field..."
curl -s -H "Authorization: token $RBV_PAT" \
     -H 'Accept: application/vnd.github.v3+json' \
     'https://api.github.com/user/packages/container/recipemuster/versions?per_page=10' \
     > ./debug_output/versions_with_name.json

echo -e "\nShowing name and tags for first 5 versions:"
jq -r '.[:5] | .[] | "ID: \(.id), Name: \(.name), Tags: \(.metadata.container.tags)"' ./debug_output/versions_with_name.json

echo -e "\nDebug complete. Check ./debug_output/ directory for full results."
