// GADIB: GAD Inspector Base Infrastructure
// Extracted from monolithic gadi_inspector.html for modular architecture
// Contains: WebSocket handling, logging, hashing, payload normalization, factory communication

// Global WebSocket trace handler instance
let gadib_ws_instance = null;

// Callback for handling refresh messages from factory
let gadib_refresh_callback = null;

// Initialize WebSocket handler - called once during module setup
function gadib_init_websocket() {
    gadib_ws_instance = {
        ws: null,
        
        connectAfterManifest() {
            // Only attempt connection after successful manifest fetch (GADS compliant)
            try {
                const wsPort = window.location.port || 8080; // Use same port as HTTP
                const wsUrl = `ws://${window.location.hostname}:${wsPort}/ws`;
                console.log(`[DEBUG] Factory available, attempting WebSocket connection to: ${wsUrl}`);
                this.ws = new WebSocket(wsUrl);

                this.ws.onopen = () => {
                    console.log('[DEBUG] WebSocket connected after manifest success');
                };

                this.ws.onerror = (error) => {
                    console.log('[DEBUG] WebSocket error (non-fatal):', error);
                };

                this.ws.onclose = () => {
                    console.log('[DEBUG] WebSocket disconnected');
                };

                this.ws.onmessage = (event) => {
                    try {
                        const message = JSON.parse(event.data);
                        if (message.type === 'refresh') {
                            gadib_logger_d('Received refresh notification from factory');
                            if (gadib_refresh_callback) {
                                gadib_refresh_callback();
                            }
                        }
                    } catch (error) {
                        console.log('[DEBUG] Failed to parse WebSocket message:', error);
                    }
                };
            } catch (error) {
                console.log('[DEBUG] WebSocket connection failed (non-fatal):', error);
            }
        },

        sendTrace(prefix, message) {
            // Always log to console, send to WebSocket if available
            console.log(`${prefix} ${message}`);

            // Send to WebSocket if connected
            if (this.ws && this.ws.readyState === WebSocket.OPEN) {
                try {
                    this.ws.send(JSON.stringify({
                        type: 'trace',
                        message: `${prefix} ${message}`
                    }));
                } catch (error) {
                    // WebSocket error is not critical
                    console.log(`[WS-ERROR] Failed to send trace: ${error}`);
                }
            }
        },

        sendDebugOutput(debugType, content, fromCommit, toCommit, sourceFiles) {
            // Simplified debug output sender for 2-step diff process
            if (this.ws && this.ws.readyState === WebSocket.OPEN) {
                try {
                    const timestamp = new Date().toISOString().replace(/[-:]/g, '').replace(/\..+/, '');
                    const fromHash = fromCommit ? fromCommit.hash.substring(0, 12) : 'unknown';
                    const toHash = toCommit ? toCommit.hash.substring(0, 12) : 'unknown';
                    
                    // Simple file extension logic: JSON for operations, HTML for output
                    const fileExtension = debugType === 'diff-operations' ? 'json' : 'html';
                    const filename = `debug-${debugType}-${fromHash}-${toHash}-${timestamp}.${fileExtension}`;
                    
                    this.ws.send(JSON.stringify({
                        type: 'debug_output',
                        debug_type: debugType,
                        content: content,
                        filename_pattern: filename,
                        source_files: sourceFiles || []
                    }));
                    console.log(`[DEBUG-OUTPUT] Sent ${debugType} debug data to Factory for ${filename} creation`);
                } catch (error) {
                    console.log(`[WS-ERROR] Failed to send ${debugType} debug output: ${error}`);
                }
            }
            // Silent if WebSocket not available - this is optional functionality
        }
    };
}

// Flat logging functions for GADS compliance
function gadib_logger_d(msg) {
    if (gadib_ws_instance) {
        gadib_ws_instance.sendTrace('D', msg);
    } else {
        console.log(`D ${msg}`);
    }
}

function gadib_logger_e(msg) {
    if (gadib_ws_instance) {
        gadib_ws_instance.sendTrace('E', msg);
    } else {
        console.log(`E ${msg}`);
    }
}


// SHA-256 hash function (Web Crypto API) for DFK compliance
async function gadib_hash(str) {
    // Use Web Crypto API for proper SHA-256 hashing as required by GADS specification
    // This provides cryptographically strong, deterministic DFK payload hashing
    const encoder = new TextEncoder();
    const data = encoder.encode(str);
    const hashBuffer = await crypto.subtle.digest('SHA-256', data);
    const hashArray = new Uint8Array(hashBuffer);
    const hexHash = Array.from(hashArray)
        .map(b => b.toString(16).padStart(2, '0'))
        .join('');
    
    return `sha256:${hexHash}`;
}

// Payload normalization for consistent DFK generation
function gadib_normalize_payload(element) {
    if (element.nodeType === Node.ELEMENT_NODE) {
        // For elements: serialized outerHTML with normalized attribute order
        const attributes = Array.from(element.attributes || [])
            .sort((a, b) => a.name.localeCompare(b.name))
            .map(attr => `${attr.name}="${attr.value}"`)
            .join(' ');
        
        const tagName = element.tagName.toLowerCase();
        const innerHTML = element.innerHTML.trim()
            .replace(/\s+/g, ' '); // Normalize insignificant whitespace
        
        return attributes.length > 0 
            ? `<${tagName} ${attributes}>${innerHTML}</${tagName}>`
            : `<${tagName}>${innerHTML}</${tagName}>`;
    } else if (element.nodeType === Node.TEXT_NODE) {
        // For text nodes: normalized text content
        return element.textContent.trim().replace(/\s+/g, ' ');
    } else {
        // For other node types: fallback to string representation
        return element.toString().trim();
    }
}

// Factory communication for debug artifacts
function gadib_factory_ship(debugType, content, fromCommit, toCommit, sourceFiles) {
    if (gadib_ws_instance) {
        gadib_ws_instance.sendDebugOutput(debugType, content, fromCommit, toCommit, sourceFiles);
    }
    // Silent if WebSocket not available - this is optional functionality
}

// Connect to WebSocket after manifest success
function gadib_connect_after_manifest() {
    if (gadib_ws_instance) {
        gadib_ws_instance.connectAfterManifest();
    }
}

// Register callback for factory refresh messages
function gadib_register_refresh_callback(callback) {
    gadib_refresh_callback = callback;
}

// Initialize WebSocket handler on module load
gadib_init_websocket();