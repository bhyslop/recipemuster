#!/usr/bin/env python3
"""
Podman Documentation Consolidator

This script processes HTML documentation files from Podman and consolidates them
into a single Markdown file optimized for AI comprehension.
"""

import os
import sys
import re
import pypandoc
import logging
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

def process_html_files(html_dir, output_dir):
    """Process all HTML files in the specified directory and generate consolidated Markdown."""
    html_files = [os.path.join(html_dir, f) for f in os.listdir(html_dir) if f.endswith('.html')]
    logger.info(f"Found {len(html_files)} HTML files to process")
    
    # Group files by category
    categories = group_documents_by_category(html_files)
    logger.info(f"Grouped documents into {len(categories)} categories: {', '.join(categories.keys())}")
    
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)
    
    # Process each category separately
    for category, files in categories.items():
        output_file = os.path.join(output_dir, f"podman-{category}-docs.md")
        logger.info(f"Processing category: {category} with {len(files)} files -> {output_file}")
        
        with open(output_file, 'w', encoding='utf-8') as outfile:
            # Add header to the markdown file
            outfile.write(f"# Podman {category.title()} Documentation\n\n")
            outfile.write("*This document is a consolidated version of the Podman documentation, optimized for AI comprehension.*\n\n")
            outfile.write("## Table of Contents\n\n")
            
            # Generate table of contents
            for html_file in sorted(files):
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
            
            # Process each file and append to the output
            for html_file in sorted(files):
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
                    outfile.write(f"# {section_name or basename}\n\n")
                    outfile.write(md_content)
                    outfile.write("\n\n---\n\n")
                    
                except Exception as e:
                    logger.error(f"Error processing {html_file}: {e}")
        
        logger.info(f"Successfully created {output_file}")

def main():
    if len(sys.argv) < 2:
        print("Usage: python script.py <html_directory> [output_directory]")
        sys.exit(1)
    
    html_dir = sys.argv[1]
    output_dir = sys.argv[2] if len(sys.argv) > 2 else os.path.join(os.path.dirname(os.path.abspath(__file__)), "output")
    
    logger.info(f"HTML directory: {html_dir}")
    logger.info(f"Output directory: {output_dir}")
    
    try:
        process_html_files(html_dir, output_dir)
        logger.info("Documentation processing complete")
    except Exception as e:
        logger.error(f"An error occurred: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()

