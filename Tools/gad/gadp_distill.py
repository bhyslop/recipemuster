#!/usr/bin/env python
"""
Copyright 2025 Scale Invariant, Inc.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

Author: GAD Implementation <generated@scaleinvariant.org>
"""

import sys
import os
import json
import hashlib
import re
from pathlib import Path
from bs4 import BeautifulSoup, Comment

def gadp_fail(message):
    """GADS-compliant fatal error reporting."""
    print(f"\033[31m{message}\033[0m", file=sys.stderr)
    sys.exit(1)

def gadp_step(message):
    """GADS-compliant step reporting."""
    print(message)

def discover_html_file(source_dir):
    """Discover HTML files in source directory per GADS specification."""
    html_files = list(Path(source_dir).glob("*.html"))
    
    if len(html_files) == 0:
        return None  # Will create error HTML
    elif len(html_files) == 1:
        return html_files[0]
    else:
        return None  # Will create error HTML with multiple files message

def normalize_whitespace_in_text(text):
    """Normalize whitespace while preserving block boundaries."""
    # Replace multiple spaces with single space
    text = re.sub(r'[ \t]+', ' ', text)
    # Replace multiple newlines with single newline (preserves block boundaries)
    text = re.sub(r'\n{2,}', '\n', text)
    # Within prose, collapse newlines to spaces
    text = re.sub(r'(?<=[^\n])\n(?=[^\n])', ' ', text)
    return text.strip()

def should_preserve_whitespace(element):
    """Check if element should preserve exact whitespace."""
    if element.name in ['pre', 'code', 'script', 'style']:
        return True
    if element.find_parent(['pre', 'code', 'verse']):
        return True
    return False

def normalize_text_content(element_copy, original_element):
    """Safely normalize text content in copied element."""
    if should_preserve_whitespace(original_element):
        return
    
    # Collect all text nodes to normalize (avoid iterator invalidation)
    text_nodes = []
    for content in element_copy.descendants:
        if isinstance(content, str):
            parent_name = content.parent.name if content.parent else ''
            if parent_name not in ['pre', 'code', 'script', 'style']:
                text_nodes.append(content)
    
    # Now safely modify collected text nodes
    for content in text_nodes:
        normalized = normalize_whitespace_in_text(content)
        if normalized and normalized != content:
            content.replace_with(normalized)

def normalize_list_numbers(soup):
    """Set all ordered list items to value='1' to eliminate renumbering noise."""
    for ol in soup.find_all('ol'):
        for li in ol.find_all('li', recursive=False):
            li['value'] = '1'
    return soup

def normalize_html(html_content):
    """Normalize HTML content per GADS whitespace and metadata requirements using safe DOM processing."""
    original_soup = BeautifulSoup(html_content, 'html.parser')
    
    # Create new soup with same structure for safe processing
    new_soup = BeautifulSoup(str(original_soup), 'html.parser')
    
    # Remove generator meta tags
    for meta in new_soup.find_all('meta', attrs={'name': 'generator'}):
        meta.decompose()
    
    # Remove build timestamp comments
    comments = new_soup.find_all(string=lambda text: isinstance(text, Comment))
    for comment in comments:
        # Remove comments containing timestamps or build info
        if any(keyword in comment.lower() for keyword in ['generated', 'timestamp', 'build', 'date']):
            comment.extract()
    
    # Remove host-specific paths in any remaining attributes
    for element in new_soup.find_all(True):
        for attr in ['href', 'src', 'data-uri']:
            if element.has_attr(attr):
                value = element[attr]
                # Remove absolute paths, keep relative
                if value.startswith('/home/') or value.startswith('/Users/') or value.startswith('C:\\'):
                    element[attr] = os.path.basename(value)
    
    # Remove figure and table auto-numbering if present
    for element in new_soup.find_all(['figcaption', 'caption']):
        text = element.get_text()
        # Remove patterns like "Figure 1.", "Table 2:", etc.
        normalized = re.sub(r'^(Figure|Table|Listing)\s+\d+[\.:]\s*', '', text)
        if normalized != text:
            element.string = normalized
    
    # Normalize ordered list numbers to eliminate renumbering noise
    new_soup = normalize_list_numbers(new_soup)
    
    # Normalize whitespace in prose contexts using safe processing
    prose_elements = new_soup.find_all(['p', 'li', 'td', 'th', 'div'])
    for element in prose_elements:
        original_element = original_soup.find(element.name, attrs=element.attrs)
        if original_element:
            normalize_text_content(element, original_element)
    
    # Remove empty paragraphs and divs that may result from normalization
    for element in new_soup.find_all(['p', 'div']):
        if not element.get_text(strip=True) and not element.find():
            element.decompose()
    
    return str(new_soup)

def generate_filename(branch, timestamp, commit_hash):
    """Generate filename per GADS pattern: branch-timestamp-shorthash.html"""
    short_hash = commit_hash[:7]
    return f"{branch}-{timestamp}-{short_hash}.html"

def calculate_sha256(content):
    """Calculate SHA-256 hash of content."""
    return hashlib.sha256(content.encode('utf-8')).hexdigest()

def create_error_html(error_message, source_dir):
    """Create error HTML file per GADS specification."""
    html_files = list(Path(source_dir).glob("*.html"))
    if len(html_files) == 0:
        message = f"Error: No HTML files found in {source_dir}"
    else:
        files_list = ", ".join([f.name for f in html_files])
        message = f"Error: Multiple HTML files found in {source_dir}: {files_list}"
    
    return f"""<!DOCTYPE html>
<html>
<head>
    <title>GAD Distill Error</title>
</head>
<body>
    <h1>GAD Distill Error</h1>
    <p>{message}</p>
</body>
</html>"""

def calculate_distinct_count(commits):
    """Calculate count of unique SHA256 values in commits."""
    sha256_values = set()
    for commit in commits:
        if "html_sha256" in commit:
            sha256_values.add(commit["html_sha256"])
    return len(sha256_values)

def load_manifest(manifest_path):
    """Load existing manifest or create new structure."""
    if manifest_path.exists():
        try:
            with open(manifest_path, 'r') as f:
                return json.load(f)
        except (json.JSONDecodeError, IOError):
            gadp_fail(f"Failed to parse existing manifest: {manifest_path}")
    
    return {
        "branch": "", 
        "asciidoc": "", 
        "distinct_sha256_count": 0,
        "last_processed_hash": "",
        "commits": []
    }

def update_manifest(manifest_path, branch, asciidoc_filename, commit_data):
    """Update manifest with new commit entry and state tracking fields."""
    manifest = load_manifest(manifest_path)
    
    # Update top-level fields
    manifest["branch"] = branch
    manifest["asciidoc"] = asciidoc_filename
    
    # Add new commit entry
    manifest["commits"].append(commit_data)
    
    # Update distinct SHA256 count
    manifest["distinct_sha256_count"] = calculate_distinct_count(manifest["commits"])
    
    # Set last_processed_hash to current commit (automatic via gadp_distill)
    manifest["last_processed_hash"] = commit_data["hash"]
    
    # Write updated manifest
    with open(manifest_path, 'w') as f:
        json.dump(manifest, f, indent=2)

def main():
    """Main distill processing per GADS specification."""
    if len(sys.argv) != 8:
        gadp_fail("Usage: gadp_distill.py <source_dir> <output_dir> <branch> <commit_hash> <commit_date> <commit_message> <asciidoc_filename>")
    
    source_dir = sys.argv[1]
    output_dir = sys.argv[2]
    branch = sys.argv[3]
    commit_hash = sys.argv[4]
    commit_date = sys.argv[5]
    commit_message = sys.argv[6]
    asciidoc_filename = sys.argv[7]
    
    # Validate directories
    if not Path(source_dir).exists():
        gadp_fail(f"Source directory does not exist: {source_dir}")
    
    Path(output_dir).mkdir(parents=True, exist_ok=True)
    
    gadp_step(f"Distilling HTML from {source_dir}")
    
    # Discover HTML file
    html_file = discover_html_file(source_dir)
    
    if html_file is None:
        # Create error HTML
        gadp_step("Creating error HTML due to file discovery failure")
        html_content = create_error_html("Discovery error", source_dir)
    else:
        # Read and normalize HTML
        try:
            with open(html_file, 'r', encoding='utf-8') as f:
                raw_html = f.read()
            html_content = normalize_html(raw_html)
        except IOError as e:
            gadp_fail(f"Failed to read HTML file {html_file}: {e}")
    
    # Generate output filename
    output_filename = generate_filename(branch, commit_date, commit_hash)
    output_path = Path(output_dir) / output_filename
    
    # Calculate SHA-256
    html_sha256 = calculate_sha256(html_content)
    
    # Write normalized HTML
    try:
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write(html_content)
        gadp_step(f"Created normalized HTML: {output_filename}")
    except IOError as e:
        gadp_fail(f"Failed to write output file {output_path}: {e}")
    
    # Update manifest
    manifest_path = Path(output_dir) / "manifest.json"
    commit_data = {
        "hash": commit_hash,
        "timestamp": commit_date,
        "date": commit_date,
        "message": commit_message,
        "html_file": output_filename,
        "html_sha256": html_sha256
    }
    
    try:
        update_manifest(manifest_path, branch, asciidoc_filename, commit_data)
        gadp_step(f"Updated manifest with commit {commit_hash[:8]}")
    except IOError as e:
        gadp_fail(f"Failed to update manifest: {e}")

if __name__ == "__main__":
    main()

# eof