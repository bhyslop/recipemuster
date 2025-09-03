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
import threading
import time
import logging
import argparse
from pathlib import Path
from http.server import HTTPServer, SimpleHTTPRequestHandler
from urllib.parse import urlparse, parse_qs
from bs4 import BeautifulSoup, Comment
import base64
import struct
import socket
import socketserver

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(message)s')
logger = logging.getLogger(__name__)

def gadfl_step(message):
    """GADS-compliant step reporting."""
    logger.info(message)

def gadfl_warn(message):
    """GADS-compliant warning reporting."""
    logger.warning(f"\033[33m{message}\033[0m")

def gadfl_fail(message):
    """GADS-compliant fatal error reporting with cleanup preservation."""
    logger.error(f"\033[31m{message}\033[0m")
    sys.exit(1)

class SimpleWebSocketHandler(socketserver.BaseRequestHandler):
    """Simple WebSocket handler without external dependencies."""
    
    def handle(self):
        """Handle WebSocket connection."""
        try:
            # Perform WebSocket handshake
            if not self.perform_handshake():
                return
            
            gadfl_step(f"WebSocket client connected from {self.client_address}")
            self.server.websocket_handler.add_client(self)
            
            # Keep connection alive and handle messages
            while True:
                try:
                    message = self.receive_message()
                    if message:
                        self.handle_message(message)
                except (ConnectionResetError, BrokenPipeError):
                    break
                except Exception as e:
                    gadfl_warn(f"WebSocket error: {e}")
                    break
                    
        except Exception as e:
            gadfl_warn(f"WebSocket connection error: {e}")
        finally:
            self.server.websocket_handler.remove_client(self)
            gadfl_step("WebSocket client disconnected")
    
    def perform_handshake(self):
        """Perform WebSocket handshake."""
        try:
            request = self.request.recv(1024).decode('utf-8')
            if 'Upgrade: websocket' not in request:
                return False
            
            # Extract WebSocket key
            for line in request.split('\r\n'):
                if line.startswith('Sec-WebSocket-Key:'):
                    key = line.split(': ')[1].strip()
                    break
            else:
                return False
            
            # Generate accept key
            import hashlib
            accept = base64.b64encode(
                hashlib.sha1((key + '258EAFA5-E914-47DA-95CA-C5AB0DC85B11').encode()).digest()
            ).decode()
            
            # Send handshake response
            response = (
                'HTTP/1.1 101 Switching Protocols\r\n'
                'Upgrade: websocket\r\n'
                'Connection: Upgrade\r\n'
                f'Sec-WebSocket-Accept: {accept}\r\n'
                '\r\n'
            )
            self.request.send(response.encode())
            return True
            
        except Exception:
            return False
    
    def receive_message(self):
        """Receive WebSocket message."""
        try:
            # Read first 2 bytes for frame info
            frame = self.request.recv(2)
            if len(frame) < 2:
                return None
                
            # Parse WebSocket frame (simplified)
            byte1, byte2 = frame[0], frame[1]
            masked = byte2 & 0x80
            payload_length = byte2 & 0x7f
            
            if payload_length == 126:
                payload_length = struct.unpack('>H', self.request.recv(2))[0]
            elif payload_length == 127:
                payload_length = struct.unpack('>Q', self.request.recv(8))[0]
            
            # Read mask if present
            if masked:
                mask = self.request.recv(4)
            
            # Read payload
            payload = self.request.recv(payload_length)
            
            # Unmask if necessary
            if masked:
                payload = bytes([payload[i] ^ mask[i % 4] for i in range(len(payload))])
            
            return payload.decode('utf-8')
            
        except Exception:
            return None
    
    def send_message(self, message):
        """Send WebSocket message."""
        try:
            payload = message.encode('utf-8')
            frame = bytearray()
            frame.append(0x81)  # Text frame
            
            if len(payload) < 126:
                frame.append(len(payload))
            elif len(payload) < 65536:
                frame.append(126)
                frame.extend(struct.pack('>H', len(payload)))
            else:
                frame.append(127)
                frame.extend(struct.pack('>Q', len(payload)))
            
            frame.extend(payload)
            self.request.send(frame)
            
        except Exception as e:
            gadfl_warn(f"Failed to send WebSocket message: {e}")
    
    def handle_message(self, message):
        """Handle received WebSocket message."""
        try:
            data = json.loads(message)
            if data.get('type') == 'trace':
                gadfl_step(f"[INSPECTOR-TRACE] {data.get('message', '')}")
        except json.JSONDecodeError:
            gadfl_warn(f"Invalid WebSocket message: {message}")

class WebSocketHandler:
    """Manages WebSocket connections using simple built-in server."""
    
    def __init__(self):
        self.clients = set()
        self.server = None
        self.server_thread = None
    
    def add_client(self, client):
        """Add WebSocket client."""
        self.clients.add(client)
    
    def remove_client(self, client):
        """Remove WebSocket client."""
        self.clients.discard(client)
    
    def start_server(self, port):
        """Start WebSocket server."""
        try:
            gadfl_step(f"Starting WebSocket server on port {port}")
            gadfl_step("Creating ThreadingTCPServer...")
            self.server = socketserver.ThreadingTCPServer(('0.0.0.0', port), SimpleWebSocketHandler)
            gadfl_step("Setting websocket_handler reference...")
            self.server.websocket_handler = self
            gadfl_step("Creating server thread...")
            self.server_thread = threading.Thread(target=self.server.serve_forever)
            self.server_thread.daemon = True
            gadfl_step("Starting server thread...")
            self.server_thread.start()
            gadfl_step(f"WebSocket server started successfully on port {port}")
        except OSError as e:
            if "Address already in use" in str(e) or "bind" in str(e).lower():
                gadfl_warn(f"WebSocket port {port} in use, trying {port+1}")
                try:
                    self.server = socketserver.ThreadingTCPServer(('0.0.0.0', port+1), SimpleWebSocketHandler)
                    self.server.websocket_handler = self
                    self.server_thread = threading.Thread(target=self.server.serve_forever)
                    self.server_thread.daemon = True
                    self.server_thread.start()
                    gadfl_step(f"WebSocket server started on alternate port {port+1}")
                    # Update the port for Inspector reference
                    self.websocket_port = port + 1
                except Exception as e2:
                    gadfl_warn(f"WebSocket server failed on both ports: {e2}")
            else:
                gadfl_warn(f"WebSocket server failed to start: {e}")
        except Exception as e:
            gadfl_warn(f"WebSocket server failed to start: {e}")
    
    def broadcast_refresh(self):
        """Send refresh message to all connected clients."""
        if not self.clients:
            return
        
        message = json.dumps({"type": "refresh", "data": "new_commit"})
        disconnected = set()
        
        for client in list(self.clients):
            try:
                client.send_message(message)
            except Exception:
                disconnected.add(client)
        
        # Clean up disconnected clients
        for client in disconnected:
            self.clients.discard(client)
    
    def stop_server(self):
        """Stop WebSocket server."""
        if self.server:
            self.server.shutdown()
            gadfl_step("WebSocket server stopped")

class GADRequestHandler(SimpleHTTPRequestHandler):
    """Custom HTTP handler for GAD Factory."""
    
    def __init__(self, *args, factory_instance=None, **kwargs):
        self.factory = factory_instance
        super().__init__(*args, **kwargs)
    
    def do_GET(self):
        # Handle WebSocket upgrade requests
        if self.headers.get('Upgrade', '').lower() == 'websocket':
            self.handle_websocket_upgrade()
            return
            
        if self.path == '/':
            # Serve Inspector from source location
            self.serve_inspector()
        elif self.path == '/manifest.json':
            # Serve manifest file
            self.serve_manifest()
        elif self.path == '/ws' or self.path == '/events':
            # WebSocket endpoint - should be handled by upgrade above
            self.send_error(400, "WebSocket upgrade required")
        elif self.path.startswith('/output/'):
            # Serve output files
            self.serve_output_file()
        else:
            # Default handling
            super().do_GET()
    
    def serve_inspector(self):
        """Serve Inspector HTML from source location."""
        try:
            inspector_path = self.factory.inspector_source_path
            with open(inspector_path, 'rb') as f:
                content = f.read()
            
            self.send_response(200)
            self.send_header('Content-Type', 'text/html')
            self.send_header('Content-Length', len(content))
            self.send_header('Cache-Control', 'no-cache, no-store, must-revalidate')
            self.send_header('Pragma', 'no-cache')
            self.send_header('Expires', '0')
            self.end_headers()
            self.wfile.write(content)
        except FileNotFoundError:
            self.send_error(404, "Inspector not found")
    
    def serve_manifest(self):
        """Serve manifest.json file."""
        try:
            manifest_path = self.factory.manifest_file
            with open(manifest_path, 'rb') as f:
                content = f.read()
            
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Content-Length', len(content))
            self.end_headers()
            self.wfile.write(content)
        except FileNotFoundError:
            self.send_error(404, "Manifest not found")
    
    def serve_websocket_redirect(self):
        """Inform clients about WebSocket endpoint."""
        self.send_response(426)  # Upgrade Required
        self.send_header('Content-Type', 'application/json')
        self.send_header('Connection', 'close')
        self.end_headers()
        
        response = {
            "error": "WebSocket connection required",
            "websocket_url": f"ws://localhost:{self.factory.websocket_port}/"
        }
        self.wfile.write(json.dumps(response).encode())
    
    def serve_output_file(self):
        """Serve files from output directory."""
        # Remove '/output/' prefix and serve from factory output directory
        file_path = self.path[8:]  # Remove '/output/'
        full_path = self.factory.output_dir / file_path
        
        try:
            with open(full_path, 'rb') as f:
                content = f.read()
            
            # Determine content type
            if file_path.endswith('.json'):
                content_type = 'application/json'
            elif file_path.endswith('.html'):
                content_type = 'text/html'
            else:
                content_type = 'application/octet-stream'
            
            self.send_response(200)
            self.send_header('Content-Type', content_type)
            self.send_header('Content-Length', len(content))
            self.end_headers()
            self.wfile.write(content)
        except FileNotFoundError:
            self.send_error(404, f"File not found: {file_path}")

    def handle_websocket_upgrade(self):
        """Handle WebSocket upgrade request - delegate to WebSocket handler."""
        try:
            # Extract WebSocket key for handshake
            websocket_key = self.headers.get('Sec-WebSocket-Key')
            if not websocket_key:
                self.send_error(400, "Missing Sec-WebSocket-Key")
                return
            
            # Generate accept key
            import hashlib
            accept = base64.b64encode(
                hashlib.sha1((websocket_key + '258EAFA5-E914-47DA-95CA-C5AB0DC85B11').encode()).digest()
            ).decode()
            
            # Send handshake response
            self.send_response(101, 'Switching Protocols')
            self.send_header('Upgrade', 'websocket')
            self.send_header('Connection', 'Upgrade')
            self.send_header('Sec-WebSocket-Accept', accept)
            self.end_headers()
            
            # Hand off to WebSocket handler
            from socketserver import BaseRequestHandler
            
            # Create a simple WebSocket handler that uses the same connection
            class EmbeddedWebSocketHandler:
                def __init__(self, request_handler):
                    self.request = request_handler.request
                    self.client_address = request_handler.client_address
                    self.server = request_handler.factory
                
                def handle_websocket_messages(self):
                    """Handle WebSocket messages on this connection."""
                    try:
                        gadfl_step(f"WebSocket client connected from {self.client_address}")
                        self.server.websocket_handler.add_client(self)
                        
                        # Keep connection alive and handle messages
                        while True:
                            try:
                                message = self.receive_message()
                                if message:
                                    self.handle_message(message)
                            except (ConnectionResetError, BrokenPipeError):
                                break
                            except Exception as e:
                                gadfl_warn(f"WebSocket error: {e}")
                                break
                                
                    except Exception as e:
                        gadfl_warn(f"WebSocket connection error: {e}")
                    finally:
                        self.server.websocket_handler.remove_client(self)
                        gadfl_step("WebSocket client disconnected")
                
                def receive_message(self):
                    """Receive WebSocket message (simplified version)."""
                    try:
                        # Read first 2 bytes for frame info
                        frame = self.request.recv(2)
                        if len(frame) < 2:
                            return None
                            
                        # Parse WebSocket frame (simplified)
                        byte1, byte2 = frame[0], frame[1]
                        masked = byte2 & 0x80
                        payload_length = byte2 & 0x7f
                        
                        if payload_length == 126:
                            payload_length = struct.unpack('>H', self.request.recv(2))[0]
                        elif payload_length == 127:
                            payload_length = struct.unpack('>Q', self.request.recv(8))[0]
                        
                        # Read mask if present
                        if masked:
                            mask = self.request.recv(4)
                        
                        # Read payload
                        payload = self.request.recv(payload_length)
                        
                        # Unmask if necessary
                        if masked:
                            payload = bytes([payload[i] ^ mask[i % 4] for i in range(len(payload))])
                        
                        return payload.decode('utf-8')
                        
                    except Exception:
                        return None
                
                def send_message(self, message):
                    """Send WebSocket message (simplified version)."""
                    try:
                        payload = message.encode('utf-8')
                        frame = bytearray()
                        frame.append(0x81)  # Text frame
                        
                        if len(payload) < 126:
                            frame.append(len(payload))
                        elif len(payload) < 65536:
                            frame.append(126)
                            frame.extend(struct.pack('>H', len(payload)))
                        else:
                            frame.append(127)
                            frame.extend(struct.pack('>Q', len(payload)))
                        
                        frame.extend(payload)
                        self.request.send(frame)
                        
                    except Exception as e:
                        gadfl_warn(f"Failed to send WebSocket message: {e}")
                
                def handle_message(self, message):
                    """Handle received WebSocket message."""
                    try:
                        data = json.loads(message)
                        if data.get('type') == 'trace':
                            gadfl_step(f"[INSPECTOR-TRACE] {data.get('message', '')}")
                    except json.JSONDecodeError:
                        gadfl_warn(f"Invalid WebSocket message: {message}")
            
            # Handle WebSocket communication in this thread
            ws_handler = EmbeddedWebSocketHandler(self)
            ws_handler.handle_websocket_messages()
            
        except Exception as e:
            gadfl_warn(f"WebSocket upgrade failed: {e}")
            self.send_error(500, f"WebSocket upgrade error: {e}")

class HTMLNormalizer:
    """HTML normalization functionality (absorbed from gadp_distill)."""
    
    @staticmethod
    def normalize_whitespace_in_text(text):
        """Normalize whitespace while preserving block boundaries."""
        text = re.sub(r'[ \t]+', ' ', text)
        text = re.sub(r'\n{2,}', '\n', text)
        text = re.sub(r'(?<=[^\n])\n(?=[^\n])', ' ', text)
        return text.strip()
    
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
        self.inspector_source_path = Path(__file__).parent / 'gadi_inspector.html'
        
        # Relative path from repo root
        try:
            self.adoc_relpath = self.adoc_filename.relative_to(self.repo_dir)
        except ValueError:
            gadfl_fail(f"AsciiDoc file '{self.adoc_filename}' is not within repository '{self.repo_dir}'")
        
        # Initialize components
        self.websocket_handler = WebSocketHandler()
        self.websocket_port = port + 1  # Use next port for WebSocket
        self.manifest = {
            'branch': self.branch,
            'asciidoc': str(self.adoc_relpath),
            'last_processed_hash': '',
            'commits': []
        }
        
        # HTTP server
        self.server = None
        self.server_thread = None
    
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
            hash_list = ' '.join(manifest_hashes)
            
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
    
    def start_http_server(self):
        """Start HTTP server for Inspector and artifact serving."""
        def make_handler(*args, **kwargs):
            return GADRequestHandler(*args, factory_instance=self, **kwargs)
        
        try:
            self.server = HTTPServer(('0.0.0.0', self.port), make_handler)
            self.server_thread = threading.Thread(target=self.server.serve_forever)
            self.server_thread.daemon = True
            self.server_thread.start()
            gadfl_step(f"HTTP server started on port {self.port}")
            gadfl_step(f"WebSocket support integrated on port {self.port} (same as HTTP)")
            
        except OSError as e:
            gadfl_fail(f"Failed to start HTTP server on port {self.port}: {e}")
    
    
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
                'asciidoctor', '-a', 'reproducible', 
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
        
        # Send WebSocket refresh message if server is running
        if self.server:
            self.websocket_handler.broadcast_refresh()
    
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
    
    def incremental_watch_mode(self):
        """Enter incremental watch mode while maintaining HTTP server."""
        if self.once:
            gadfl_step("Once mode: exiting after initial population")
            return
        
        gadfl_step("Entering incremental watch mode (polling every 3 seconds)")
        
        while True:
            try:
                time.sleep(3)
                
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
            
            except KeyboardInterrupt:
                gadfl_step("Received interrupt, shutting down")
                break
            except subprocess.CalledProcessError as e:
                gadfl_warn(f"Git operation failed: {e}")
            except Exception as e:
                gadfl_warn(f"Watch mode error: {e}")
    
    def run(self):
        """Main execution method."""
        # Validate files
        if not self.adoc_filename.exists():
            gadfl_fail(f"AsciiDoc file '{self.adoc_filename}' not found")
        
        if not self.inspector_source_path.exists():
            gadfl_fail(f"Inspector file not found at {self.inspector_source_path}")
        
        # Setup
        self.setup_directories()
        self.start_http_server()
        
        # Execute processing
        self.initial_population()
        
        if not self.once:
            try:
                self.incremental_watch_mode()
            finally:
                if self.server:
                    self.server.shutdown()
                    gadfl_step("HTTP server stopped")
                self.websocket_handler.stop_server()
        else:
            # Keep server running briefly for once mode
            gadfl_step(f"GAD factory processing complete. Output in {self.output_dir}")
            gadfl_step(f"Inspector available at http://localhost:{self.port}/")
            
            if self.server:
                try:
                    gadfl_step("Press Ctrl+C to stop server and exit")
                    while True:
                        time.sleep(1)
                except KeyboardInterrupt:
                    self.server.shutdown()
                    self.websocket_handler.stop_server()
                    gadfl_step("HTTP and WebSocket servers stopped")

def main():
    """Main entry point with argument parsing."""
    parser = argparse.ArgumentParser(description='GAD Factory - Python implementation')
    parser.add_argument('--file', required=True, help='AsciiDoc filename to process')
    parser.add_argument('--directory', required=True, help='Working directory')
    parser.add_argument('--branch', default='main', help='Git branch to track')
    parser.add_argument('--max-distinct-renders', type=int, default=5, 
                       help='Maximum distinct renders to maintain')
    parser.add_argument('--once', action='store_true', 
                       help='Disable watch mode after initial population')
    parser.add_argument('--port', type=int, default=8080, 
                       help='HTTP server port')
    
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
        sys.exit(1)
    except Exception as e:
        gadfl_fail(f"Unexpected error: {e}")

if __name__ == "__main__":
    main()