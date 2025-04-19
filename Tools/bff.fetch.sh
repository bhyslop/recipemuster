#!/bin/bash

# Script to list files matching provided patterns
# Usage: ./bff.fetch.sh <file_pattern1> <file_pattern2> ...

if [ $# -eq 0 ]; then
  echo "Usage: $0 <file_pattern1> <file_pattern2> ..." >&2
  exit 1
fi

# Initialize empty array for concrete files
files=()

# First pass: collect all concrete filenames from patterns
for pattern in "$@"; do
  if [[ "$pattern" == *\** || "$pattern" == *\?* || "$pattern" == *\[* ]]; then
    # Expand glob pattern
    glob_files=($pattern)
    
    # Check if expansion returned any files
    if [ ${#glob_files[@]} -eq 0 ] || [ "${glob_files[0]}" = "$pattern" ]; then
      echo "No files matching pattern: $pattern" || exit 1
      continue
    fi

    # Add expanded files to our list
    files+=("${glob_files[@]}")
  else
    # Add concrete file to the list
    files+=("$pattern")
  fi
done

# Second pass: process the list of files
for file in "${files[@]}"; do
  if [ -f "$file" ]; then
    echo "vvvvvvvvvvvvvvvvvvvv File: $file BEGIN vvvvvvvvvvvvvvvvvvvvvvv"
    cat "$file"
    echo -e "\n"
    echo "^^^^^^^^^^^^^^^^^^^^ File: $file END   ^^^^^^^^^^^^^^^^^^^^^^^"
  else
    echo "File not found: $file" >&2 || exit 1
  fi
done

