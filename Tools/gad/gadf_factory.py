#!/usr/bin/env python
"""
GAD Factory - Python implementation per GADMCR specification

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
import subprocess
import time
import logging
import argparse
from pathlib import Path
from urllib.parse import urlparse, parse_qs
from bs4 import BeautifulSoup, Comment
import asyncio

import tornado.ioloop
import tornado.web
import tornado.websocket
from tornado.web import StaticFileHandler

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(message)s')
logger = logging.getLogger(__name__)

def gadfl_step(message):
    """GADS-compliant step reporting."""
    logger.info(message)
    sys.stdout.flush()

def gadfl_warn(message):
    """GADS-compliant warning reporting."""
    logger.warning(f"\033[33m{message}\033[0m")
    sys.stdout.flush()

def gadfl_fail(message):
    """GADS-compliant fatal error reporting with cleanup preservation."""
    logger.error(f"\033[31m{message}\033[0m")
    sys.stdout.flush()
    sys.exit(1)

class WebSocketHandler(tornado.websocket.WebSocketHandler):
    """Tornado WebSocket handler for GAD Factory."""

    clients = set()

    def open(self):
        """Handle new WebSocket connection."""
        WebSocketHandler.clients.add(self)
        gadfl_step(f"WebSocket client connected from {self.request.remote_ip}")

    def on_close(self):
        """Handle WebSocket disconnection."""
        WebSocketHandler.clients.discard(self)
        gadfl_step("WebSocket client disconnected")

    def on_message(self, message):
        """Handle received WebSocket message."""
        try:
            data = json.loads(message)
            if data.get('type') == 'trace':
                gadfl_step(f"[INSPECTOR-TRACE] {data.get('message', '')}")
            elif data.get('type') == 'debug_output':
                self.handle_debug_output(data)
            elif data.get('type') == 'rendered_content':
                # Legacy compatibility - redirect to debug_output
                self.handle_legacy_rendered_content(data)
            elif data.get('type') == 'annotated_dom':
                # Legacy compatibility - redirect to debug_output  
                self.handle_legacy_annotated_dom(data)
        except json.JSONDecodeError:
            gadfl_warn(f"Invalid WebSocket message: {message}")

    def handle_debug_output(self, data):
        """Handle consolidated debug output message from Inspector for all 8-phase artifacts."""
        try:
            factory = self.application.factory
            debug_type = data.get('debug_type', 'unknown')
            content = data.get('content', '')
            
            # Use GADS-compliant filename pattern provided by Inspector
            filename = data.get('filename_pattern')
            if not filename:
                # Fallback pattern
                timestamp = time.strftime('%Y%m%d%H%M%S')
                extension = 'json' if debug_type.endswith('dft') else 'html'
                filename = f"debug-{debug_type}-{timestamp}.{extension}"
            
            filepath = factory.output_dir / filename
            source_files = data.get('source_files', [])

            # Write content based on file type
            if filename.endswith('.json'):
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(content)
            else:
                with open(filepath, 'w', encoding='utf-8') as f:
                    # Write comment lines with source file information per GADS spec
                    if source_files:
                        debug_type_display = self.get_debug_type_display_name(debug_type)
                        f.write(f"<!-- GAD {debug_type_display} source files:\n")
                        for source_file in source_files:
                            f.write(f"     {source_file}\n")
                        f.write("-->\n")
                    
                    f.write(content)

            phase_display = self.get_debug_type_display_name(debug_type)
            gadfl_step(f"{phase_display} debug file created: {filename}")
            gadfl_step(f"Full path: {filepath}")

        except Exception as e:
            gadfl_warn(f"Failed to save {debug_type} debug output: {e}")

    def get_debug_type_display_name(self, debug_type):
        """Get display name for debug type."""
        debug_type_names = {
            # New modular engine phase labels
            'phase3_dft': 'Phase 3 Deletion Fact Table',
            'phase6_annotated': 'Phase 6 Annotated Assembly', 
            'phase7_deletions': 'Phase 7 Deletion Placement',
            'phase8_coalesced': 'Phase 8 Uniform Classing',
            'phase9_final': 'Phase 9 Final Serialize',
            'rendered': 'Rendered Content',
            # Legacy phase labels for backwards compatibility
            'phase5_annotated': 'Phase 5 Annotated Assembly (Legacy)',
            'phase6_deletions': 'Phase 6 Deletion Placement (Legacy)',
            'phase7_coalesced': 'Phase 7 Uniform Classing (Legacy)',
            'phase8_final': 'Phase 8 Final Serialize (Legacy)',
            'annotated': 'Annotated DOM'
        }
        return debug_type_names.get(debug_type, f"Debug Output ({debug_type})")

    def handle_legacy_rendered_content(self, data):
        """Handle legacy rendered content message for backwards compatibility."""
        # Convert to new debug_output format
        debug_data = {
            'type': 'debug_output',
            'debug_type': 'rendered',
            'content': data.get('content', ''),
            'filename_pattern': data.get('filename_pattern'),
            'source_files': data.get('source_files', [])
        }
        self.handle_debug_output(debug_data)

    def handle_legacy_annotated_dom(self, data):
        """Handle legacy annotated DOM message for backwards compatibility."""
        # Convert to new debug_output format
        debug_data = {
            'type': 'debug_output',
            'debug_type': 'annotated',
            'content': data.get('content', ''),
            'filename_pattern': data.get('filename_pattern'),
            'source_files': data.get('source_files', [])
        }
        self.handle_debug_output(debug_data)

    @classmethod
    def broadcast_refresh(cls):
        """Send refresh message to all connected clients."""
        if not cls.clients:
            return

        message = json.dumps({"type": "refresh", "data": "new_commit"})
        disconnected = set()

        for client in list(cls.clients):
            try:
                client.write_message(message)
            except Exception:
                disconnected.add(client)

        # Clean up disconnected clients
        for client in disconnected:
            cls.clients.discard(client)


class InspectorHandler(tornado.web.RequestHandler):
    """Serve Inspector HTML from source location."""

    def get(self):
        try:
            factory = self.application.factory
            with open(factory.inspector_source_path, 'rb') as f:
                content = f.read()

            self.set_header('Content-Type', 'text/html')
            self.set_header('Cache-Control', 'no-cache, no-store, must-revalidate')
            self.set_header('Pragma', 'no-cache')
            self.set_header('Expires', '0')
            self.write(content)
        except FileNotFoundError:
            self.send_error(404, reason="Inspector not found")

class ManifestHandler(tornado.web.RequestHandler):
    """Serve manifest.json file."""

    def get(self):
        try:
            factory = self.application.factory
            with open(factory.manifest_file, 'rb') as f:
                content = f.read()

            self.set_header('Content-Type', 'application/json')
            self.write(content)
        except FileNotFoundError:
            self.send_error(404, reason="Manifest not found")

class HTMLNormalizer:
    """HTML normalization functionality (absorbed from gadp_distill)."""

    @staticmethod
    def normalize_whitespace_in_text(text):
        """Normalize whitespace while preserving block boundaries and inline spacing."""
        # Preserve leading/trailing space markers
        has_leading_space = text.startswith((' ', '\t', '\n'))
        has_trailing_space = text.endswith((' ', '\t', '\n'))

        # Normalize internal whitespace
        text = re.sub(r'[ \t]+', ' ', text)
        text = re.sub(r'\n{2,}', '\n', text)
        text = re.sub(r'(?<=[^\n])\n(?=[^\n])', ' ', text)

        # Strip internal normalization
        normalized = text.strip()

        # Restore significant leading/trailing spaces for inline context
        if normalized and has_leading_space:
            normalized = ' ' + normalized
        if normalized and has_trailing_space:
            normalized = normalized + ' '

        return normalized

    @staticmethod
    def should_preserve_whitespace(element):
        """Check if element should preserve exact whitespace."""
        if element.name in ['pre', 'code', 'script', 'style']:
            return True
        if element.find_parent(['pre', 'code', 'verse']):
            return True
        return False

    @staticmethod
    def normalize_text_content(element_copy, original_element):
        """Safely normalize text content in copied element."""
        if HTMLNormalizer.should_preserve_whitespace(original_element):
            return

        text_nodes = []
        for content in element_copy.descendants:
            if isinstance(content, str):
                parent_name = content.parent.name if content.parent else ''
                if parent_name not in ['pre', 'code', 'script', 'style']:
                    text_nodes.append(content)

        for content in text_nodes:
            normalized = HTMLNormalizer.normalize_whitespace_in_text(content)
            if normalized and normalized != content:
                content.replace_with(normalized)

    @staticmethod
    def normalize_list_numbers(soup):
        """Set all ordered list items to value='1'."""
        for ol in soup.find_all('ol'):
            for li in ol.find_all('li', recursive=False):
                li['value'] = '1'
        return soup

    @staticmethod
    def normalize_html(html_content):
        """Normalize HTML content per GADS specification."""
        # Safe DOM processing: create clean output from original
        original_soup = BeautifulSoup(html_content, 'html.parser')
        # Create completely new DOM structure to avoid in-place modification
        new_soup = BeautifulSoup('', 'html.parser')

        # Clone the document structure
        if original_soup.doctype:
            new_soup.append(original_soup.doctype)

        # Process HTML element
        if original_soup.html:
            new_html = new_soup.new_tag('html')
            for attr, value in original_soup.html.attrs.items():
                new_html[attr] = value
            new_soup.append(new_html)

            # Copy structure safely
            HTMLNormalizer._copy_element_safely(original_soup.html, new_html, new_soup)
        else:
            # Handle fragments
            for element in original_soup.children:
                HTMLNormalizer._copy_element_safely(element, new_soup, new_soup)

        # Remove generator meta tags
        for meta in new_soup.find_all('meta', attrs={'name': 'generator'}):
            meta.decompose()

        # Remove build timestamp comments
        comments = new_soup.find_all(string=lambda text: isinstance(text, Comment))
        for comment in comments:
            if any(keyword in comment.lower() for keyword in ['generated', 'timestamp', 'build', 'date']):
                comment.extract()

        # Remove host-specific paths (Linux environment only)
        for element in new_soup.find_all(True):
            for attr in ['href', 'src', 'data-uri']:
                if element.has_attr(attr):
                    value = element[attr]
                    if value.startswith('/home/') or value.startswith('/Users/'):
                        element[attr] = os.path.basename(value)

        # Remove auto-numbering
        for element in new_soup.find_all(['figcaption', 'caption']):
            text = element.get_text()
            normalized = re.sub(r'^(Figure|Table|Listing)\s+\d+[\.:]\s*', '', text)
            if normalized != text:
                element.string = normalized

        # Normalize list numbers
        new_soup = HTMLNormalizer.normalize_list_numbers(new_soup)

        # Normalize whitespace in prose contexts
        prose_elements = new_soup.find_all(['p', 'li', 'td', 'th', 'div'])
        for element in prose_elements:
            original_element = original_soup.find(element.name, attrs=element.attrs)
            if original_element:
                HTMLNormalizer.normalize_text_content(element, original_element)

        # Remove empty elements
        for element in new_soup.find_all(['p', 'div']):
            if not element.get_text(strip=True) and not element.find():
                element.decompose()

        return str(new_soup)

    @staticmethod
    def _copy_element_safely(source_element, target_parent, soup):
        """Safely copy element from source to target without in-place modification."""
        from bs4 import NavigableString, Comment, Tag

        if isinstance(source_element, NavigableString):
            if isinstance(source_element, Comment):
                # Skip comments containing volatile content
                comment_text = str(source_element).lower()
                if any(keyword in comment_text for keyword in ['generated', 'timestamp', 'build', 'date']):
                    return
                new_comment = Comment(str(source_element))
                target_parent.append(new_comment)
            else:
                # Handle text nodes with whitespace normalization
                text_content = str(source_element)
                if HTMLNormalizer.should_preserve_whitespace_context(source_element):
                    target_parent.append(NavigableString(text_content))
                else:
                    normalized_text = HTMLNormalizer.normalize_whitespace_in_text(text_content)
                    if normalized_text:
                        target_parent.append(NavigableString(normalized_text))
        elif isinstance(source_element, Tag):
            # Create new tag
            new_tag = soup.new_tag(source_element.name)

            # Copy and clean attributes
            for attr, value in source_element.attrs.items():
                if attr in ['href', 'src', 'data-uri']:
                    if isinstance(value, str) and (value.startswith('/home/') or value.startswith('/Users/')):
                        new_tag[attr] = os.path.basename(value)
                    else:
                        new_tag[attr] = value
                else:
                    new_tag[attr] = value

            # Handle special elements
            if source_element.name == 'meta' and source_element.get('name') == 'generator':
                return  # Skip generator meta tags

            # Set ordered list normalization
            if source_element.name == 'li' and source_element.find_parent('ol'):
                new_tag['value'] = '1'

            # Handle auto-numbering removal
            if source_element.name in ['figcaption', 'caption']:
                text = source_element.get_text()
                normalized = re.sub(r'^(Figure|Table|Listing)\s+\d+[\.:]\s*', '', text)
                if normalized != text:
                    new_tag.string = normalized
                    target_parent.append(new_tag)
                    return

            target_parent.append(new_tag)

            # Recursively copy children
            for child in source_element.children:
                HTMLNormalizer._copy_element_safely(child, new_tag, soup)

    @staticmethod
    def should_preserve_whitespace_context(element):
        """Check if element context should preserve whitespace."""
        if hasattr(element, 'parent'):
            parent = element.parent
            while parent:
                if hasattr(parent, 'name') and parent.name in ['pre', 'code', 'script', 'style']:
                    return True
                parent = getattr(parent, 'parent', None)
        return False

class GADFactory:
    """Main GAD Factory class implementing Python Factory architecture."""

    def __init__(self, adoc_filename, directory, branch='main', max_distinct_renders=5,
                 once=False, port=8080):
        self.adoc_filename = Path(adoc_filename).resolve()
        self.directory = Path(directory).resolve()
        self.branch = branch
        self.max_distinct_renders = max_distinct_renders
        self.once = once
        self.port = port

        # Directory structure
        self.extract_dir = self.directory / '.factory-extract'
        self.distill_dir = self.directory / '.factory-distill'
        self.output_dir = self.directory / 'output'
        self.manifest_file = self.output_dir / 'manifest.json'

        # Find repository root
        self.repo_dir = self.find_git_repo(self.adoc_filename)
        if not self.repo_dir:
            gadfl_fail(f"No git repository found for AsciiDoc file '{self.adoc_filename}'")

        # Inspector source path
        self.inspector_source_path = Path(__file__).parent / 'gadiw_webpage.html'

        # Relative path from repo root
        try:
            self.adoc_relpath = self.adoc_filename.relative_to(self.repo_dir)
        except ValueError:
            gadfl_fail(f"AsciiDoc file '{self.adoc_filename}' is not within repository '{self.repo_dir}'")

        # Initialize components
        self.manifest = {
            'branch': self.branch,
            'asciidoc': str(self.adoc_relpath),
            'last_processed_hash': '',
            'commits': []
        }

        # Tornado application and server
        self.app = None
        self.server = None

    def find_git_repo(self, start_path):
        """Find git repository root."""
        current = start_path.parent
        while current != current.parent:
            if (current / '.git').exists():
                return current
            current = current.parent
        return None

    def clean_directory_contents(self, directory):
        """Recursively delete all contents of directory per GADS specification."""
        if not directory.exists():
            return

        import shutil
        for item in directory.iterdir():
            if item.is_file() or item.is_symlink():
                item.unlink()
            elif item.is_dir():
                shutil.rmtree(item)

    def sort_commits_by_parent_chain(self):
        """Sort manifest commits by parent chain traversal order."""
        if len(self.manifest['commits']) <= 1:
            return

        # Get commit hashes in parent chain traversal order from git
        os.chdir(self.repo_dir)
        try:
            # Get all commit hashes from our manifest
            manifest_hashes = [c['hash'] for c in self.manifest['commits']]
            hash_list = '\n'.join(manifest_hashes)

            # Get them in parent chain traversal order (oldest first)
            result = subprocess.run([
                'git', 'log', '--reverse', '--format=%H', '--no-walk', '--stdin'
            ], input=hash_list, text=True, capture_output=True, check=True)

            parent_chain_hashes = [line.strip() for line in result.stdout.strip().split('\n') if line.strip()]

            # Create hash-to-commit mapping
            commit_map = {c['hash']: c for c in self.manifest['commits']}

            # Reorder commits by parent chain traversal order
            self.manifest['commits'] = [commit_map[hash_val] for hash_val in parent_chain_hashes if hash_val in commit_map]

        except subprocess.CalledProcessError:
            gadfl_warn("Failed to sort commits by parent chain traversal, keeping processing order")

    def setup_directories(self):
        """Create directory structure per GADS specification."""
        gadfl_step("Creating GAD directory structure")
        self.extract_dir.mkdir(parents=True, exist_ok=True)
        self.distill_dir.mkdir(parents=True, exist_ok=True)
        self.output_dir.mkdir(parents=True, exist_ok=True)

        # Clean workspace per single-threaded premise
        gadfl_step("Cleaning workspace")
        self.clean_directory_contents(self.extract_dir)
        self.clean_directory_contents(self.distill_dir)
        self.clean_directory_contents(self.output_dir)

        # Delete existing manifest for fresh start
        if self.manifest_file.exists():
            gadfl_step("Deleting existing manifest for fresh start")
            self.manifest_file.unlink()

    def start_tornado_server(self):
        """Start Tornado server for HTTP and WebSocket on single port."""
        try:
            # Create Tornado application
            self.app = tornado.web.Application([
                (r"/", InspectorHandler),
                (r"/ws", WebSocketHandler),
                (r"/manifest.json", ManifestHandler),
                (r"/(gadic_cascade\.css)", StaticFileHandler, {
                    "path": str(Path(__file__).parent)
                }),
                (r"/(gadib_base\.js)", StaticFileHandler, {
                    "path": str(Path(__file__).parent)
                }),
                (r"/(gadie_engine\.js)", StaticFileHandler, {
                    "path": str(Path(__file__).parent)
                }),
                (r"/(gadiu_user\.js)", StaticFileHandler, {
                    "path": str(Path(__file__).parent)
                }),
                (r"/output/(.*)", StaticFileHandler, {
                    "path": str(self.output_dir),
                    "default_filename": None
                })
            ])

            # Store factory reference in application
            self.app.factory = self

            # Start server
            self.app.listen(self.port, address='0.0.0.0')
            gadfl_step(f"Tornado server started on port {self.port}")
            gadfl_step(f"HTTP and WebSocket integrated on single port {self.port}")

        except OSError as e:
            gadfl_fail(f"Failed to start Tornado server on port {self.port}: {e}")


    def render_commit(self, commit_hash):
        """Execute render sequence for a single commit."""
        gadfl_step(f"Executing render sequence for commit {commit_hash[:8]}")

        # Clean temporary directories
        self.clean_directory_contents(self.extract_dir)
        self.clean_directory_contents(self.distill_dir)

        # Change to repository directory for git operations
        os.chdir(self.repo_dir)

        # Extract commit files via git archive
        try:
            result = subprocess.run(['git', 'archive', commit_hash],
                                  capture_output=True, check=True)
            subprocess.run(['tar', '-xf', '-', '-C', str(self.extract_dir)],
                         input=result.stdout, check=True)
        except subprocess.CalledProcessError:
            gadfl_fail(f"Failed to extract commit {commit_hash}")

        # Check if AsciiDoc file exists in this commit
        extracted_adoc = self.extract_dir / self.adoc_relpath
        if not extracted_adoc.exists():
            gadfl_warn(f"AsciiDoc file '{self.adoc_relpath}' not found in commit {commit_hash}, skipping")
            return False

        # Run asciidoctor
        gadfl_step("Running asciidoctor")
        try:
            subprocess.run([
                'asciidoctor', '-a', 'reproducible', '-a', 'sectnum!',
                str(extracted_adoc), '-D', str(self.distill_dir)
            ], check=True)
        except subprocess.CalledProcessError:
            gadfl_fail(f"Asciidoctor failed for commit {commit_hash}")

        # Get commit metadata
        try:
            commit_timestamp = subprocess.run([
                'git', 'log', '-1', '--format=%cd', '--date=format:%Y%m%d%H%M%S', commit_hash
            ], capture_output=True, text=True, check=True).stdout.strip()

            commit_message = subprocess.run([
                'git', 'log', '-1', '--format=%s', commit_hash
            ], capture_output=True, text=True, check=True).stdout.strip()
        except subprocess.CalledProcessError:
            gadfl_fail(f"Failed to get commit metadata for {commit_hash}")

        # Process HTML (integrated distill functionality)
        self.process_html(commit_hash, commit_timestamp, commit_message)
        return True

    def process_html(self, commit_hash, commit_timestamp, commit_message):
        """Process HTML files using integrated normalization."""
        gadfl_step(f"Processing HTML for commit {commit_hash[:8]}")

        # Discover HTML file
        html_files = list(self.distill_dir.glob("*.html"))

        if len(html_files) == 0:
            html_content = self.create_error_html("No HTML files found", self.distill_dir)
        elif len(html_files) > 1:
            files_list = ", ".join([f.name for f in html_files])
            html_content = self.create_error_html(f"Multiple HTML files found: {files_list}", self.distill_dir)
        else:
            # Read and normalize HTML
            try:
                with open(html_files[0], 'r', encoding='utf-8') as f:
                    raw_html = f.read()
                html_content = HTMLNormalizer.normalize_html(raw_html)
            except IOError as e:
                gadfl_fail(f"Failed to read HTML file {html_files[0]}: {e}")

        # Generate output filename using content-based naming per GADS specification
        html_sha256 = hashlib.sha256(html_content.encode('utf-8')).hexdigest()
        output_filename = self.generate_filename(html_content)
        output_path = self.output_dir / output_filename

        # Write normalized HTML
        try:
            with open(output_path, 'w', encoding='utf-8') as f:
                f.write(html_content)
            gadfl_step(f"Created normalized HTML: {output_filename}")
        except IOError as e:
            gadfl_fail(f"Failed to write output file {output_path}: {e}")

        # Update manifest
        commit_data = {
            'hash': commit_hash,
            'timestamp': commit_timestamp,
            'date': commit_timestamp,
            'message': commit_message,
            'html_file': output_filename,
            'html_sha256': html_sha256
        }

        self.update_manifest(commit_data)

    def create_error_html(self, error_message, source_dir):
        """Create error HTML file per GADS specification."""
        return f"""<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>GAD Factory Processing Error</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, sans-serif; margin: 40px; background: #ffeef0; }}
        .error-container {{ background: white; padding: 20px; border-radius: 8px; border: 1px solid #f97583; }}
        .error-header {{ color: #d73a49; font-size: 24px; margin-bottom: 15px; }}
        .error-details {{ color: #586069; margin-bottom: 10px; }}
        .source-path {{ font-family: monospace; background: #f6f8fa; padding: 5px; border-radius: 3px; }}
    </style>
</head>
<body>
    <div class="error-container">
        <h1 class="error-header">GAD Factory Processing Error</h1>
        <p class="error-details">
            <strong>Error:</strong> {error_message}
        </p>
        <p class="error-details">
            <strong>Source Directory:</strong> <code class="source-path">{source_dir}</code>
        </p>
        <p class="error-details">
            This error occurred during AsciiDoc to HTML conversion. Please check:
        </p>
        <ul>
            <li>AsciiDoc file syntax and structure</li>
            <li>Asciidoctor installation and version</li>
            <li>File permissions in source directory</li>
        </ul>
    </div>
</body>
</html>"""

    def generate_filename(self, html_content):
        """Generate filename per GADS pattern using content-based SHA256."""
        html_sha256 = hashlib.sha256(html_content.encode('utf-8')).hexdigest()
        return f"{self.branch}-{html_sha256}.html"

    def calculate_distinct_count(self):
        """Calculate count of unique SHA256 values."""
        sha256_values = set()
        for commit in self.manifest['commits']:
            if 'html_sha256' in commit:
                sha256_values.add(commit['html_sha256'])
        return len(sha256_values)

    def update_manifest(self, commit_data):
        """Update manifest with new commit entry."""
        # Add new commit entry
        self.manifest['commits'].append(commit_data)

        # Sort commits by parent chain traversal order per GADS specification
        self.sort_commits_by_parent_chain()

        # Update state per GADS specification
        self.manifest['last_processed_hash'] = commit_data['hash']

        # Write updated manifest atomically
        try:
            temp_file = self.manifest_file.with_suffix('.tmp')
            with open(temp_file, 'w') as f:
                json.dump(self.manifest, f, indent=2)
            temp_file.rename(self.manifest_file)
            gadfl_step(f"Updated manifest with commit {commit_data['hash'][:8]}")
        except IOError as e:
            gadfl_fail(f"Failed to update manifest: {e}")

        # Send WebSocket refresh message to connected clients
        WebSocketHandler.broadcast_refresh()

    def get_commits_to_process(self):
        """Get list of commits for initial population (HEAD-first)."""
        os.chdir(self.repo_dir)

        try:
            # Get HEAD commit first
            head_commit = subprocess.run([
                'git', 'rev-parse', self.branch
            ], capture_output=True, text=True, check=True).stdout.strip()

            # Get commit list (we'll process HEAD first, then others by parent chain traversal)
            result = subprocess.run([
                'git', 'log', '--format=%H', self.branch, '-100'
            ], capture_output=True, text=True, check=True)

            commits = [line.strip() for line in result.stdout.strip().split('\n') if line.strip()]
            return commits
        except subprocess.CalledProcessError:
            gadfl_fail(f"Failed to get commit list for branch {self.branch}")

    def initial_population(self):
        """Process commits using HEAD-first with intelligent interleaving."""
        gadfl_step("Starting initial population with HEAD-first processing")

        commits = self.get_commits_to_process()
        if not commits:
            gadfl_fail("No commits found")

        # Process HEAD first
        head_commit = commits[0]
        if self.commit_has_adoc(head_commit):
            if self.render_commit(head_commit):
                gadfl_step(f"HEAD commit processed: {head_commit[:8]}")

        # Continue with remaining commits until we reach max distinct count
        for commit_hash in commits[1:]:
            current_distinct_count = self.calculate_distinct_count()
            if current_distinct_count >= self.max_distinct_renders:
                gadfl_step(f"Reached maximum distinct renders ({self.max_distinct_renders})")
                break

            if self.commit_has_adoc(commit_hash):
                if self.render_commit(commit_hash):
                    current_count = self.calculate_distinct_count()
                    gadfl_step(f"Current distinct count: {current_count}")

    def commit_has_adoc(self, commit_hash):
        """Check if commit contains the AsciiDoc file."""
        os.chdir(self.repo_dir)
        try:
            subprocess.run([
                'git', 'show', f'{commit_hash}:{self.adoc_relpath}'
            ], capture_output=True, check=True)
            return True
        except subprocess.CalledProcessError:
            return False

    async def incremental_watch_mode(self):
        """Enter incremental watch mode using async polling."""
        if self.once:
            gadfl_step("Once mode: exiting after initial population")
            return

        gadfl_step("Entering incremental watch mode (polling every 3 seconds)")

        while True:
            try:
                await asyncio.sleep(3)

                os.chdir(self.repo_dir)

                # Get current HEAD
                current_head = subprocess.run([
                    'git', 'rev-parse', self.branch
                ], capture_output=True, text=True, check=True).stdout.strip()

                last_processed = self.manifest.get('last_processed_hash', '')

                if last_processed and current_head != last_processed:
                    gadfl_step(f"Detected new commits beyond {last_processed[:8]}")

                    # Get new commits
                    result = subprocess.run([
                        'git', 'log', '--reverse', '--format=%H',
                        f'{last_processed}..{self.branch}'
                    ], capture_output=True, text=True, check=True)

                    new_commits = [line.strip() for line in result.stdout.strip().split('\n')
                                 if line.strip()]

                    for commit_hash in new_commits:
                        if self.commit_has_adoc(commit_hash):
                            self.render_commit(commit_hash)

            except asyncio.CancelledError:
                gadfl_step("Watch mode cancelled, shutting down")
                break
            except subprocess.CalledProcessError as e:
                gadfl_warn(f"Git operation failed: {e}")
            except Exception as e:
                gadfl_warn(f"Watch mode error: {e}")

    async def run_async(self):
        """Main async execution method."""
        # Validate files
        if not self.adoc_filename.exists():
            gadfl_fail(f"AsciiDoc file '{self.adoc_filename}' not found")

        if not self.inspector_source_path.exists():
            gadfl_fail(f"Inspector file not found at {self.inspector_source_path}")

        # Setup
        self.setup_directories()
        self.start_tornado_server()

        # Execute processing
        self.initial_population()

        if not self.once:
            try:
                await self.incremental_watch_mode()
            except asyncio.CancelledError:
                gadfl_step("Factory operation cancelled")
        else:
            # Keep server running for once mode
            gadfl_step(f"GAD factory processing complete. Output in {self.output_dir}")
            gadfl_step(f"Inspector available at http://localhost:{self.port}/")
            gadfl_step("Press Ctrl+C to stop server and exit")

            try:
                # Wait indefinitely for Ctrl+C
                while True:
                    await asyncio.sleep(1)
            except asyncio.CancelledError:
                pass
            except KeyboardInterrupt:
                pass

            gadfl_step("Tornado server stopped")

    def run(self):
        """Main execution method - wrapper for async run."""
        try:
            asyncio.run(self.run_async())
        except KeyboardInterrupt:
            gadfl_step("Interrupted by user")

def main():
    """Main entry point with argument parsing."""
    parser = argparse.ArgumentParser(description='GAD Factory - Tornado implementation')
    parser.add_argument('--file', required=True, help='AsciiDoc filename to process')
    parser.add_argument('--directory', required=True, help='Working directory')
    parser.add_argument('--branch', default='main', help='Git branch to track')
    parser.add_argument('--max-distinct-renders', type=int, default=5,
                       help='Maximum distinct renders to maintain')
    parser.add_argument('--once', action='store_true',
                       help='Disable watch mode after initial population')
    parser.add_argument('--port', type=int, default=8080,
                       help='HTTP and WebSocket server port')

    args = parser.parse_args()

    try:
        factory = GADFactory(
            adoc_filename=args.file,
            directory=args.directory,
            branch=args.branch,
            max_distinct_renders=args.max_distinct_renders,
            once=args.once,
            port=args.port
        )
        factory.run()
    except KeyboardInterrupt:
        gadfl_step("Interrupted by user")
        sys.exit(0)
    except Exception as e:
        gadfl_fail(f"Unexpected error: {e}")

if __name__ == "__main__":
    main()
