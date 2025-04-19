#!/bin/bash

# Script to list files matching provided patterns
# Usage: ./list_files.sh <file_pattern1> <file_pattern2> ...

if [ $# -eq 0 ]; then
  echo "Usage: $0 <file_pattern1> <file_pattern2> ..." >&2
  exit 1
fi

for pattern in "$@"; do
  find . -type f -path "$pattern" | sort | while read -r file; do
    echo "vvvvvvvvvvvvvvvvvvvv File: ${file#./} BEGIN vvvvvvvvvvvvvvvvvvvvvvv"
    cat "$file"
    echo -e "\n"
    echo "^^^^^^^^^^^^^^^^^^^^ File: ${file#./} END   ^^^^^^^^^^^^^^^^^^^^^^^"
  done
done


