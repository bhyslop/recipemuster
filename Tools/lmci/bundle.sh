#!/bin/bash
# Script to list files matching provided patterns and recursively include directory contents
# Usage: ./bundle.sh <file_pattern1> <directory1> <file_pattern2> ...

# Force UTF-8 locale for proper unicode handling
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8

if [ $# -eq 0 ]; then
  echo "Usage: $0 <file_pattern1> <directory1> <file_pattern2> ..." >&2
  exit 1
fi

# Initialize empty array for concrete files
files=()

# First pass: collect all concrete filenames from patterns and directories
for pattern in "$@"; do
  # Fail if pattern contains double slash
  if [[ "$pattern" == *"//"* ]]; then
    echo "Error: Pattern contains double slash which is not supported: $pattern" >&2
    exit 1
  fi

  # Check if it's a directory
  if [ -d "$pattern" ]; then
    # Find all files recursively in the directory and add to list
    while IFS= read -r dir_file; do
      files+=("$dir_file")
    done < <(find "$pattern" -type f -not -path "*/\.*" | sort)

    # Inform user we processed a directory
    echo "Added files from directory: $pattern" >&2
  elif [[ "$pattern" == *\** || "$pattern" == *\?* || "$pattern" == *\[* ]]; then
    # if there's a slash plus wildcard, do a recursive find
    if [[ "$pattern" == */*[\*\?\[]* ]]; then
      dir=${pattern%/*}
      pat=${pattern##*/}
      while IFS= read -r f; do
        files+=("$f")
      done < <(find "$dir" -type f -name "$pat" -not -path "*/\.*" | sort)
      echo "Added recursive glob files for pattern: $pattern" >&2
      continue
    fi

    # Perform glob expansion
    glob_files=($pattern)

    # Check if expansion returned any files
    if [ ${#glob_files[@]} -eq 0 ] || [ "${glob_files[0]}" = "$pattern" ]; then
      echo "No files matching pattern: $pattern" >&2
      continue
    fi
    # Add expanded files to our list
    files+=("${glob_files[@]}")
    echo "Added files matching pattern: $pattern" >&2
  else
    # Add concrete file to the list
    files+=("$pattern")
    echo "Added individual file: $pattern" >&2
  fi
done

# Echo the total count of files to process
echo "Processing ${#files[@]} files:" >&2

# Function to check if file is likely text (not binary)
is_text_file() {
  local file="$1"
  # Use file command to check if it's text
  if command -v file >/dev/null 2>&1; then
    file -b --mime-encoding "$file" 2>/dev/null | grep -qE '^(us-ascii|utf-8|utf-16|iso-8859|text)'
    return $?
  else
    # Fallback: check for null bytes in first 512 bytes
    head -c 512 "$file" 2>/dev/null | grep -q $'\x00'
    return $((1 - $?))  # Invert result (no null = text)
  fi
}

# Function to safely output file content
output_file_content() {
  local file="$1"

  # Check if file is text
  if is_text_file "$file"; then
    # Try to detect encoding and convert if needed
    if command -v file >/dev/null 2>&1; then
      local encoding
      encoding=$(file -b --mime-encoding "$file" 2>/dev/null)

      case "$encoding" in
        utf-8|us-ascii)
          # Already good, just cat it
          cat "$file"
          ;;
        utf-16*)
          # Convert UTF-16 to UTF-8
          if command -v iconv >/dev/null 2>&1; then
            iconv -f UTF-16 -t UTF-8 "$file" 2>/dev/null || cat "$file"
          else
            cat "$file"
          fi
          ;;
        iso-8859*)
          # Convert ISO-8859 to UTF-8
          if command -v iconv >/dev/null 2>&1; then
            iconv -f ISO-8859-1 -t UTF-8//IGNORE "$file" 2>/dev/null || cat "$file"
          else
            cat "$file"
          fi
          ;;
        *)
          # Try to output as-is, with fallback to hex dump for safety
          cat "$file" 2>/dev/null || {
            echo "[Binary or non-UTF-8 content detected - skipping raw output]" >&2
            echo "[File encoding: $encoding]" >&2
          }
          ;;
      esac
    else
      # No file command available, just try cat with UTF-8 assumption
      cat "$file"
    fi
  else
    echo "[Skipping binary file: $file]" >&2
    echo "[Binary content not shown]"
  fi
}

# Second pass: process the list of files
for file in "${files[@]}"; do
  if [ -f "$file" ]; then
    # Echo the file being processed to stderr
    echo "Including file: $file" >&2

    # Output the file contents to stdout with headers/footers
    echo "vvvvvvvvvvvvvvvvvvvv File: $file BEGIN vvvvvvvvvvvvvvvvvvvvvvv"

    # Safely output file content with encoding handling
    output_file_content "$file"

    # Use printf instead of echo -e for better portability
    printf "\n"
    echo "^^^^^^^^^^^^^^^^^^^^ File: $file END   ^^^^^^^^^^^^^^^^^^^^^^^"
  else
    echo "File not found: $file" >&2
  fi
done
