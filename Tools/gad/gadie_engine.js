// GADIE: GAD Inspector Diff Engine
// Extracted from monolithic gadi_inspector.html for modular architecture
// Simplified to 2-step process: diff-dom operations + CSS styling

// GADIE Constants - Centralized static tables
const GADIE_CONSTANTS = {
    BLOCK_TAGS: new Set(['P', 'DIV', 'H1', 'H2', 'H3', 'H4', 'H5', 'H6', 
                         'LI', 'TD', 'TH', 'BLOCKQUOTE', 'PRE', 'SECTION', 
                         'ARTICLE', 'HEADER', 'FOOTER', 'NAV', 'ASIDE', 'MAIN',
                         'DL', 'DT', 'DD', 'UL', 'OL']),
    
    INLINE_PARENTS: new Set(['SPAN', 'A', 'EM', 'STRONG', 'B', 'I', 'CODE']),
    
    INTERACTIVE_TAGS: new Set(['BUTTON', 'INPUT', 'SELECT', 'TEXTAREA']),
    
    DIFF_CLASSES: {
        'BLOCK_ADDITION': { cssClass: 'gads-addition-block', wrapType: 'block' },
        'INLINE_ADDITION': { cssClass: 'gads-addition-inline', wrapType: 'inline' },
        'BLOCK_REMOVAL': { cssClass: 'gads-deletion-block', wrapType: 'block' },
        'INLINE_REMOVAL': { cssClass: 'gads-deletion-inline', wrapType: 'inline' },
        'STRUCTURAL_CHANGE': { cssClass: 'gads-modification-structural', wrapType: 'block' },
        'INLINE_MODIFICATION': { cssClass: 'gads-modification-inline', wrapType: 'inline' }
    },
    
    COALESCING_CLASSES: ['gads-addition-inline', 'gads-modification-structural', 
                        'gads-deletion-inline', 'gads-modification-inline']
};

// Helper: Check if element tag is block-level
function gadie_is_block(tag) {
    return GADIE_CONSTANTS.BLOCK_TAGS.has(tag);
}

// Verbosity gating helper - default to error-only console output
function gadie_should_log(level, opts) {
    // Force debug logging when debug artifacts are enabled
    if (opts.debugArtifacts) {
        return true;
    }
    
    const verbosity = opts.verbosity || 'error';
    if (verbosity === 'error') return level === 'error';
    if (verbosity === 'warn') return ['error', 'warn'].includes(level);
    if (verbosity === 'debug') return true;
    return false;
}

// Main diff processing function - simplified 2-step process
async function gadie_diff(fromHtml, toHtml, opts = {}) {
    const { fromCommit, toCommit, sourceFiles } = opts;
    
    if (gadie_should_log('debug', opts)) {
        gadib_logger_d('Starting simplified 2-step diff processing');
        gadib_logger_d(`Input sizes: fromHtml=${fromHtml.length} chars, toHtml=${toHtml.length} chars`);
    }

    if (typeof window.diffDom === 'undefined' || typeof window.diffDom.DiffDOM === 'undefined') {
        throw new Error('diff-dom library not available - required for diff processing');
    }

    try {
        // Step 1: Create immutable DOMs and run diff-dom
        const fromDOM = gadie_create_dom_from_html(fromHtml);
        const toDOM = gadie_create_dom_from_html(toHtml);
        
        const diffDOM = new window.diffDom.DiffDOM({
            debug: false,
            diffcap: 500
        });
        const operations = diffDOM.diff(fromDOM, toDOM);
        
        // Step 2: Apply CSS classes to create styled output
        const styledDOM = gadie_create_dom_from_html(toHtml);
        gadie_apply_css_styling(styledDOM, operations);
        
        // Step 3: Generate operations visualization
        const manifest = { fromCommit, toCommit };
        const opsVisualization = gadie_render_operations_html(operations, manifest);
        
        // Step 4: Append operations coda to the styled content
        const codaDiv = document.createElement('div');
        codaDiv.innerHTML = opsVisualization;
        styledDOM.appendChild(codaDiv);
        
        const styledHTML = styledDOM.innerHTML;
        
        // Debug output if requested
        if (opts.debugArtifacts && sourceFiles) {
            gadib_factory_ship('diff-operations', JSON.stringify(operations, null, 2), fromCommit, toCommit, sourceFiles);
            gadib_factory_ship('styled-output', styledHTML, fromCommit, toCommit, sourceFiles);
        }
        
        return styledHTML;

    } catch (error) {
        if (gadie_should_log('error', opts)) {
            gadib_logger_e(`Simplified diff processing failed: ${error.message}`);
        }
        throw error;
    }
}

// Helper method for creating DOM from HTML strings
function gadie_create_dom_from_html(html) {
    const parser = new DOMParser();
    const doc = parser.parseFromString(html, 'text/html');
    return doc.body;
}

// Simple route traversal helper
function gadie_find_element_by_route(dom, route) {
    let current = dom;
    for (const index of route) {
        if (!current || !current.childNodes || index >= current.childNodes.length) {
            return null;
        }
        current = current.childNodes[index];
    }
    return current;
}

// Simple CSS styling function for diff operations
function gadie_apply_css_styling(dom, operations) {
    operations.forEach(op => {
        if (op.action === 'addElement' || op.action === 'addTextElement') {
            const element = gadie_find_element_by_route(dom, op.route);
            if (element && element.nodeType === Node.ELEMENT_NODE && element.classList) {
                const isBlock = gadie_is_block(element.tagName);
                element.classList.add(isBlock ? 'gads-addition-block' : 'gads-addition-inline');
            }
        } else if (op.action === 'modifyTextElement' || op.action === 'modifyAttribute') {
            const element = gadie_find_element_by_route(dom, op.route);
            if (element && element.nodeType === Node.ELEMENT_NODE && element.classList) {
                const isBlock = gadie_is_block(element.tagName);
                element.classList.add(isBlock ? 'gads-modification-structural' : 'gads-modification-inline');
            }
        }
    });
}

// HTML escaper for safe rendering
function gadie_escape_html(str) {
    if (str == null) return '';
    return String(str)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;')
        .replace(/'/g, '&#39;');
}

// Truncate text with ellipsis
function gadie_truncate(str, maxLen) {
    if (!str || str.length <= maxLen) return str;
    return str.substring(0, maxLen - 1) + '…';
}

// Get action family for badge styling
function gadie_get_action_family(action) {
    if (action.includes('modify') || action.includes('replace')) return 'modify';
    if (action.includes('add')) return 'add';
    if (action.includes('remove')) return 'remove';
    if (action.includes('move')) return 'move';
    if (action.includes('Attribute')) return 'attr';
    return 'modify'; // fallback
}

// Render token diff for text modifications
function gadie_render_token_diff(oldValue, newValue) {
    const oldTokens = (oldValue || '').split(/(\s+)/);
    const newTokens = (newValue || '').split(/(\s+)/);
    
    // Simple token-level diff
    const result = [];
    const maxLen = Math.max(oldTokens.length, newTokens.length);
    
    for (let i = 0; i < maxLen; i++) {
        const oldToken = oldTokens[i];
        const newToken = newTokens[i];
        
        if (oldToken && newToken && oldToken === newToken) {
            result.push(`<span class="gad-token gad-token--ctx">${gadie_escape_html(oldToken)}</span>`);
        } else {
            if (oldToken) {
                result.push(`<span class="gad-token gad-token--del">${gadie_escape_html(oldToken)}</span>`);
            }
            if (newToken) {
                result.push(`<span class="gad-token gad-token--ins">${gadie_escape_html(newToken)}</span>`);
            }
        }
    }
    
    return result.join('');
}

// Extract key attributes from element
function gadie_extract_key_attrs(element) {
    const keyAttrs = ['id', 'class', 'href', 'src', 'name', 'value'];
    const attrs = [];
    
    if (element && element.attributes) {
        keyAttrs.forEach(attr => {
            const val = element.attributes[attr];
            if (val) attrs.push(`${attr}="${gadie_escape_html(val)}"`);
        });
    }
    
    return attrs.length > 0 ? ` ${attrs.join(' ')}` : '';
}

// Extract text content snippet
function gadie_extract_text_snippet(element) {
    if (!element) return '';
    
    // Simple text extraction - get first text node content
    if (element.textContent) {
        return gadie_truncate(element.textContent.trim(), 80);
    }
    
    return '';
}

// Render individual operation payload
function gadie_render_op_payload(op) {
    const action = op.action;
    
    switch (action) {
        case 'modifyTextElement':
            return `<div class="gad-op__text-diff">${gadie_render_token_diff(op.oldValue, op.newValue)}</div>`;
            
        case 'removeTextElement':
        case 'addTextElement':
            const value = gadie_escape_html(op.value || '(missing)');
            const truncated = gadie_truncate(value, 200);
            const className = action === 'removeTextElement' ? 'gad-token--del' : 'gad-token--ins';
            return `<div class="gad-token ${className}">${truncated}</div>`;
            
        case 'removeElement':
        case 'addElement':
            const element = op.element;
            const nodeName = element ? element.nodeName || '(unknown)' : '(missing)';
            const attrs = gadie_extract_key_attrs(element);
            const textSnippet = gadie_extract_text_snippet(element);
            
            return `<div class="gad-op__element">
                <span class="gad-snap__tag">${gadie_escape_html(nodeName)}</span>
                ${attrs ? `<span class="gad-snap__attrs">${attrs}</span>` : ''}
                ${textSnippet ? `<span class="gad-snap__text">"${gadie_escape_html(textSnippet)}"</span>` : ''}
            </div>`;
            
        case 'modifyAttribute':
        case 'replaceAttribute':
            const attrName = op.name || '(missing)';
            const oldVal = op.oldValue || '';
            const newVal = op.newValue || '';
            return `<div class="gad-op__attr">
                <span class="gad-snap__tag">${gadie_escape_html(attrName)}</span>: 
                <span class="gad-token gad-token--del">"${gadie_escape_html(oldVal)}"</span> → 
                <span class="gad-token gad-token--ins">"${gadie_escape_html(newVal)}"</span>
            </div>`;
            
        case 'moveElement':
            const from = op.from ? JSON.stringify(op.from) : '(missing)';
            const to = op.to ? JSON.stringify(op.to) : '(missing)';
            return `<div class="gad-op__move">From: ${gadie_escape_html(from)} → To: ${gadie_escape_html(to)}</div>`;
            
        default:
            return `<div class="gad-op__unknown">Unknown operation: ${gadie_escape_html(action)}</div>`;
    }
}

// Render micro-preview line
function gadie_render_micro_preview(op) {
    const action = op.action;
    let preview = '';
    
    if (action.includes('Element')) {
        const element = op.element;
        if (element && element.nodeName) {
            const nodeName = element.nodeName;
            const attrs = gadie_extract_key_attrs(element);
            const textSnippet = gadie_extract_text_snippet(element);
            preview = `${nodeName}${attrs}${textSnippet ? ` "${textSnippet}"` : ''}`;
        }
    } else if (action.includes('TextElement')) {
        const value = op.value || op.newValue || op.oldValue || '';
        preview = `"${gadie_truncate(value, 60)}"`;
    }
    
    return preview ? `<div class="gad-op__preview">${gadie_escape_html(gadie_truncate(preview, 80))}</div>` : '';
}

// Main function to render diff operations as HTML
function gadie_render_operations_html(operations, manifest = null) {
    if (!operations || operations.length === 0) {
        return '<section class="gad-ops"><div class="gad-ops__prov">No operations to display</div></section>';
    }
    
    // Build provenance line
    let provenance = 'Diff Operations';
    if (manifest && manifest.fromCommit && manifest.toCommit) {
        const fromShort = manifest.fromCommit.substring(0, 8);
        const toShort = manifest.toCommit.substring(0, 8);
        provenance = `from: ${fromShort} → to: ${toShort}`;
    }
    
    // Build operation list
    const opItems = operations.map((op, index) => {
        const actionFamily = gadie_get_action_family(op.action);
        const routeStr = op.route ? JSON.stringify(op.route) : '[]';
        
        return `<li class="gad-op">
            <div class="gad-op__head">
                <span class="gad-op__idx">${index}</span>
                <span class="gad-badge gad-badge--${actionFamily}">${gadie_escape_html(op.action)}</span>
                <span class="gad-op__route">${gadie_escape_html(routeStr)}</span>
            </div>
            <details class="gad-op__payload">
                <summary>payload</summary>
                ${gadie_render_op_payload(op)}
            </details>
            ${gadie_render_micro_preview(op)}
        </li>`;
    }).join('');
    
    // Footer with op count
    const footer = `<div class="gad-ops__prov">${provenance} • ${operations.length} ops</div>`;
    
    return `<section class="gad-ops">
        <div class="gad-ops__prov">${provenance}</div>
        <ol class="gad-ops__list">${opItems}</ol>
        ${footer}
    </section>`;
}