#!/usr/bin/env python3
"""
Podman Documentation Consolidator

This script processes HTML documentation files from multiple Podman distributions
and consolidates them into multiple Markdown files per distribution organized by command category.
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
    
    # Main command groups based on the log file pattern
    main_groups = [
        "container", "farm", "generate", "healthcheck", "image", "kube", "machine", 
        "manifest", "network", "pod", "secret", "system", "volume"
    ]
    
    for html_file in html_files:
        # Skip non-HTML files
        if not html_file.endswith('.html'):
            continue
            
        file_basename = os.path.basename(html_file)
        
        # Extract category from filename (e.g., podman-container-* belongs to 'container' category)
        match = re.match(r'podman-([a-zA-Z0-9-]+)-', file_basename)
        if match:
            category = match.group(1)
            # Make sure the category is one of the main groups
            if category in main_groups:
                categories[category].append(html_file)
            else:
                categories["misc"].append(html_file)
        else:
            # Handle special cases
            if file_basename == 'podman.html':
                categories['main'].append(html_file)
            elif file_basename.startswith('podman-') and sum(1 for c in file_basename if c == '-') == 1:
                # Commands like podman-build.html, podman-run.html go to the "core" category
                categories['core'].append(html_file)
            else:
                categories['misc'].append(html_file)
    
    return categories

def create_markdown_file(category, html_files, dist_name, output_dir):
    """Create a markdown file for a specific category of commands."""
    category_name = category.title()
    output_file = os.path.join(output_dir, f"{dist_name}-{category}-docs.md")
    logger.info(f"Creating markdown file for {category_name} commands: {output_file}")
    
    with open(output_file, 'w', encoding='utf-8') as outfile:
        # Add header to the markdown file
        outfile.write(f"# {dist_name} {category_name} Commands\n\n")
        outfile.write(f"*This document contains {category_name} commands from the Podman documentation.*\n\n")
        outfile.write("## Table of Contents\n\n")
        
        # Generate table of contents
        for html_file in sorted(html_files):
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
        
        # Process each file in the category
        for html_file in sorted(html_files):
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
    return output_file

def create_index_file(dist_name, category_files, output_dir):
    """Create an index markdown file pointing to all category files."""
    index_file = os.path.join(output_dir, f"{dist_name}-index.md")
    logger.info(f"Creating index file: {index_file}")
    
    with open(index_file, 'w', encoding='utf-8') as outfile:
        outfile.write(f"# {dist_name} Documentation Index\n\n")
        outfile.write("*This document provides links to all command categories in the Podman documentation.*\n\n")
        
        for category, file_path in sorted(category_files.items()):
            relative_path = os.path.basename(file_path)
            outfile.write(f"- [{category.title()} Commands]({relative_path})\n")
    
    logger.info(f"Successfully created index file: {index_file}")

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
    
    # Create a distribution-specific output directory
    dist_output_dir = os.path.join(output_dir, dist_name)
    if not os.path.exists(dist_output_dir):
        os.makedirs(dist_output_dir)
    
    # Group files by category for better organization
    categories = group_documents_by_category(html_files)
    logger.info(f"Grouped documents into {len(categories)} categories: {', '.join(categories.keys())}")
    
    # Create a file for each category
    category_files = {}
    for category, files in categories.items():
        if files:  # Only create files for non-empty categories
            md_file = create_markdown_file(category, files, dist_name, dist_output_dir)
            category_files[category] = md_file
    
    # Create an index file
    create_index_file(dist_name, category_files, dist_output_dir)
    
    logger.info(f"Distribution {dist_name} processing complete")

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
    