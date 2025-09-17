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
            if (element) {
                const isBlock = gadie_is_block(element.tagName);
                element.classList.add(isBlock ? 'gads-addition-block' : 'gads-addition-inline');
            }
        } else if (op.action === 'modifyTextElement' || op.action === 'modifyAttribute') {
            const element = gadie_find_element_by_route(dom, op.route);
            if (element) {
                const isBlock = gadie_is_block(element.tagName);
                element.classList.add(isBlock ? 'gads-modification-structural' : 'gads-modification-inline');
            }
        }
    });
}