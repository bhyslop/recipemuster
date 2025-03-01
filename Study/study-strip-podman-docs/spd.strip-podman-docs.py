#!/usr/bin/env python3
"""
Podman Documentation Consolidator

This script processes HTML documentation files from multiple Podman distributions
and consolidates them into one Markdown file per distribution optimized for AI comprehension.
"""

import os
import sys
import re
import pypandoc
import logging
import glob
from bs4 import BeautifulSoup
from collections import defaultdict

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

def extract_title_and_section(html_content):
    """Extract the title and section information from the HTML content."""
    soup = BeautifulSoup(html_content, 'lxml')
    title = None
    
    # Try to find title in the header
    header_title = soup.find('title')
    if header_title:
        title = header_title.text.strip()
    
    # Find the first h2 with id="name" - that's typically the command name
    name_header = soup.find('h2', id='name')
    section_name = None
    if name_header and name_header.find_next('p'):
        section_name = name_header.find_next('p').text.strip()
    
    return title, section_name

def sanitize_content(md_content):
    """Clean up the markdown content for better readability."""
    # Remove consecutive blank lines
    md_content = re.sub(r'\n{3,}', '\n\n', md_content)
    
    # Ensure proper header formatting
    md_content = re.sub(r'^##([^#])', r'## \1', md_content, flags=re.MULTILINE)
    
    return md_content

def group_documents_by_category(html_files):
    """Group HTML files by their command category."""
    categories = defaultdict(list)
    
    for html_file in html_files:
        # Skip non-HTML files
        if not html_file.endswith('.html'):
            continue
            
        # Extract category from filename (e.g., podman-container-* belongs to 'container' category)
        match = re.match(r'podman-([a-zA-Z0-9-]+)-', os.path.basename(html_file))
        if match:
            category = match.group(1)
            categories[category].append(html_file)
        else:
            # If no hyphen, it's likely the main podman command
            if os.path.basename(html_file) == 'podman.html':
                categories['main'].append(html_file)
            else:
                categories['other'].append(html_file)
    
    return categories

def process_distribution(dist_path, output_dir):
    """Process a single Podman distribution."""
    dist_name = os.path.basename(dist_path)
    logger.info(f"Processing distribution: {dist_name}")
    
    # Check if dist_path itself is a docs directory or if it contains a docs subdirectory
    if os.path.basename(dist_path) == "docs":
        docs_dir = dist_path
        # Extract distribution name from parent directory
        dist_name = os.path.basename(os.path.dirname(dist_path))
    else:
        docs_dir = os.path.join(dist_path, "docs")
        if not os.path.exists(docs_dir):
            logger.warning(f"No docs directory found in {dist_path}")
            return
    
    html_files = glob.glob(os.path.join(docs_dir, "*.html"))
    if not html_files:
        logger.warning(f"No HTML files found in {docs_dir}")
        return
    
    logger.info(f"Found {len(html_files)} HTML files to process")
    
    # Create one markdown file per distribution
    output_file = os.path.join(output_dir, f"{dist_name}-docs.md")
    logger.info(f"Creating consolidated markdown file: {output_file}")
    
    with open(output_file, 'w', encoding='utf-8') as outfile:
        # Add header to the markdown file
        outfile.write(f"# {dist_name} Documentation\n\n")
        outfile.write("*This document is a consolidated version of the Podman documentation, optimized for AI comprehension.*\n\n")
        outfile.write("## Table of Contents\n\n")
        
        # Group files by category for better organization
        categories = group_documents_by_category(html_files)
        logger.info(f"Grouped documents into {len(categories)} categories: {', '.join(categories.keys())}")
        
        # Generate table of contents by category
        for category in sorted(categories.keys()):
            outfile.write(f"### {category.title()}\n\n")
            
            for html_file in sorted(categories[category]):
                try:
                    with open(html_file, 'r', encoding='utf-8') as infile:
                        html_content = infile.read()
                    
                    _, section_name = extract_title_and_section(html_content)
                    basename = os.path.basename(html_file).replace('.html', '')
                    if section_name:
                        outfile.write(f"- [{section_name}](#{basename})\n")
                    else:
                        outfile.write(f"- [{basename}](#{basename})\n")
                except Exception as e:
                    logger.error(f"Error processing TOC for {html_file}: {e}")
            
            outfile.write("\n")
        
        # Process each category
        for category in sorted(categories.keys()):
            outfile.write(f"# {category.title()} Commands\n\n")
            
            # Process each file in the category
            for html_file in sorted(categories[category]):
                try:
                    logger.info(f"Converting {html_file}")
                    with open(html_file, 'r', encoding='utf-8') as infile:
                        html_content = infile.read()
                    
                    title, section_name = extract_title_and_section(html_content)
                    basename = os.path.basename(html_file).replace('.html', '')
                    
                    # Convert HTML to Markdown using pypandoc
                    md_content = pypandoc.convert_text(html_content, 'markdown', format='html')
                    md_content = sanitize_content(md_content)
                    
                    # Add a separator and file identifier
                    outfile.write(f"<a id='{basename}'></a>\n\n")
                    outfile.write(f"## {section_name or basename}\n\n")
                    outfile.write(md_content)
                    outfile.write("\n\n---\n\n")
                    
                except Exception as e:
                    logger.error(f"Error processing {html_file}: {e}")
        
        logger.info(f"Successfully created {output_file}")

def main():
    if len(sys.argv) < 2:
        print("Usage: python script.py <podman_root_directory> [output_directory]")
        sys.exit(1)
    
    podman_root_dir = sys.argv[1]
    output_dir = sys.argv[2] if len(sys.argv) > 2 else os.path.join(os.path.dirname(os.path.abspath(__file__)), "output")
    
    logger.info(f"Podman root directory: {podman_root_dir}")
    logger.info(f"Output directory: {output_dir}")
    
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    
    try:
        # Check if the provided path is directly to a docs directory
        if os.path.basename(podman_root_dir) == "docs":
            logger.info("Processing single docs directory")
            process_distribution(podman_root_dir, output_dir)
        else:
            # Find all podman distribution directories
            podman_dirs = glob.glob(os.path.join(podman_root_dir, "podman-*"))
            
            if not podman_dirs:
                logger.error(f"No podman distribution directories found in {podman_root_dir}")
                sys.exit(1)
            
            logger.info(f"Found {len(podman_dirs)} podman distributions: {', '.join(os.path.basename(d) for d in podman_dirs)}")
            
            # Process each distribution
            for dist_dir in podman_dirs:
                process_distribution(dist_dir, output_dir)
        
        logger.info("Documentation processing complete")
    except Exception as e:
        logger.error(f"An error occurred: {e}")
        sys.exit(1)

if __name__ == "__main__":
    print("BEGINNING PODMAN DOCUMENTATION CONSOLIDATION")
    main()
    