#!/bin/bash
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# count_words.sh — Count unique words in a text file
# Usage: ./count_words.sh <filename>

if [ -z "$1" ]; then
    echo "Usage: $0 <filename>"
    exit 1
fi

if [ ! -f "$1" ]; then
    echo "Error: File '$1' not found"
    exit 1
fi

echo "=== Word Frequency Analysis ==="
echo "File: $1"
echo

# Count unique words and sort by frequency (descending)
cat "$1" \
  | tr -s '[:space:]' '\n' \
  | sort \
  | uniq -c \
  | sort -rn \
  | head -20

echo
total_unique=$(cat "$1" | tr -s '[:space:]' '\n' | sort | uniq | wc -l)
total_words=$(cat "$1" | wc -w)

echo "Total unique words: ${total_unique}"
echo "Total words:        ${total_words}"
