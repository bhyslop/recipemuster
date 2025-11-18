// GADIE: GAD Inspector Diff Engine
// Extracted from monolithic gadi_inspector.html for modular architecture
// Simplified to 2-step process: diff-dom operations + CSS styling

// Logger wrapper that sends to Factory if available, otherwise browser console
function gadie_log(msg) {
    if (typeof gadib_logger_d === 'function') {
        gadib_logger_d(msg);
    } else {
        console.log(msg);
    }
}

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
// Returns structured object with multiple view outputs for tab-based UI:
// {
//   prototypeHTML: string - Current styled output with operations coda appended
//   dualHTML: string - Placeholder for future side-by-side comparison view  
//   operations: array - Raw diff-dom operations for potential future use
//   fromDOM: DOM - Source DOM structure for potential future use
//   toDOM: DOM - Target DOM structure for potential future use
// }
async function gadie_diff(fromHtml, toHtml, opts = {}) {
    const { fromCommit, toCommit, sourceFiles } = opts;
    
    if (gadie_should_log('debug', opts)) {
        gadie_log('Starting simplified 2-step diff processing');
        gadie_log(`Input sizes: fromHtml=${fromHtml.length} chars, toHtml=${toHtml.length} chars`);
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
        const opsVisualization = gadie_render_operations_html(operations, manifest, fromDOM);
        
        // Step 4: Create prototype view with operations coda appended
        const prototypeDOM = gadie_create_dom_from_html(toHtml);
        gadie_apply_css_styling(prototypeDOM, operations);
        const codaDiv = document.createElement('div');
        codaDiv.innerHTML = opsVisualization;
        prototypeDOM.appendChild(codaDiv);
        const prototypeHTML = prototypeDOM.innerHTML;
        
        // Step 5: Create dual view with change correlation
        const reverseOps = diffDOM.diff(toDOM, fromDOM); // Reverse direction for deletions
        const changeList = gadie_correlate_changes(operations, reverseOps);
        const leftRendered = gadie_render_dual_left(fromDOM, changeList);
        const rightRendered = gadie_render_dual_right(toDOM, changeList);
        const changePaneHTML = gadie_render_change_entries(changeList, operations);
        const dualHTML = gadie_build_dual_view_html(leftRendered, rightRendered, changePaneHTML);
        
        // Step 6: Create structured return object
        const result = {
            prototypeHTML: prototypeHTML,
            dualHTML: dualHTML,
            operations: operations,
            fromDOM: fromDOM,
            toDOM: toDOM
        };
        
        // Debug output if requested
        if (opts.debugArtifacts && sourceFiles) {
            // Ship raw DOM structures for analysis
            gadib_factory_ship('from-dom-structure', fromDOM.outerHTML, fromCommit, toCommit, sourceFiles);
            gadib_factory_ship('to-dom-structure', toDOM.outerHTML, fromCommit, toCommit, sourceFiles);
            
            // Ship route mapping tables for both DOMs
            const fromRouteMap = gadie_generate_route_map(fromDOM);
            const toRouteMap = gadie_generate_route_map(toDOM);
            gadib_factory_ship('from-route-mapping', JSON.stringify(fromRouteMap, null, 2), fromCommit, toCommit, sourceFiles);
            gadib_factory_ship('to-route-mapping', JSON.stringify(toRouteMap, null, 2), fromCommit, toCommit, sourceFiles);
            
            // Ship diff operations with enhanced analysis
            const enhancedOps = operations.map(op => ({
                ...op,
                routeTarget: op.route ? gadie_find_element_by_route(fromDOM, op.route) : null,
                routeDescription: op.route ? fromRouteMap[op.route.join(',')] : null
            }));
            gadib_factory_ship('diff-operations', JSON.stringify(enhancedOps, null, 2), fromCommit, toCommit, sourceFiles);
            
            // Ship all views for inspection
            gadib_factory_ship('prototype-output', prototypeHTML, fromCommit, toCommit, sourceFiles);
            gadib_factory_ship('dual-output', dualHTML, fromCommit, toCommit, sourceFiles);
            gadib_factory_ship('styled-output', prototypeHTML, fromCommit, toCommit, sourceFiles); // backward compatibility

            // Ship dual view debug artifacts
            gadib_factory_ship('dual-left-rendered', leftRendered, fromCommit, toCommit, sourceFiles);
            gadib_factory_ship('dual-right-rendered', rightRendered, fromCommit, toCommit, sourceFiles);
            gadib_factory_ship('dual-changes', JSON.stringify(changeList, null, 2), fromCommit, toCommit, sourceFiles);
            gadib_factory_ship('dual-reverse-ops', JSON.stringify(reverseOps, null, 2), fromCommit, toCommit, sourceFiles);
        }
        
        return result;

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

// Color palette mapping for dual view - 16 colors total
const GADIE_COLOR_PALETTE = [
    // Reciprocal changes (0-7) - warmer palette
    'rgba(255, 107, 107, 0.2)', // 0 - red
    'rgba(255, 193, 7, 0.2)',   // 1 - amber
    'rgba(76, 175, 80, 0.2)',   // 2 - green
    'rgba(33, 150, 243, 0.2)',  // 3 - blue
    'rgba(156, 39, 176, 0.2)',  // 4 - purple
    'rgba(255, 152, 0, 0.2)',   // 5 - orange
    'rgba(0, 150, 136, 0.2)',   // 6 - teal
    'rgba(233, 30, 99, 0.2)',   // 7 - pink
    // Monocular changes (8-15) - greyed/muted palette
    'rgba(117, 117, 117, 0.15)', // 8 - grey
    'rgba(158, 158, 158, 0.15)', // 9 - grey
    'rgba(189, 189, 189, 0.15)', // 10 - grey
    'rgba(224, 224, 224, 0.15)', // 11 - grey
    'rgba(207, 216, 220, 0.15)', // 12 - blue grey
    'rgba(244, 208, 63, 0.15)',  // 13 - yellow
    'rgba(129, 199, 132, 0.15)', // 14 - green
    'rgba(144, 202, 249, 0.15)'  // 15 - blue
];

// Extract text content from an operation for matching
function gadie_extract_operation_text(op) {
    if (!op) return '';

    // Handle different operation types
    if (op.action === 'addTextElement' || op.action === 'removeTextElement') {
        return (op.value || '').trim();
    }

    if (op.action === 'modifyTextElement') {
        return (op.newValue || op.oldValue || '').trim();
    }

    if (op.action === 'addElement' || op.action === 'removeElement') {
        if (op.element && op.element.textContent) {
            return op.element.textContent.trim();
        }
    }

    return '';
}

// Function 1: Correlate changes between forward and reverse operations
function gadie_correlate_changes(forwardList, reverseList) {
    const changeList = [];
    const pairedForward = new Set();
    const pairedReverse = new Set();
    let reciprocalCount = 0;

    // Step 1: Find reciprocal pairs by matching text content
    for (let i = 0; i < forwardList.length; i++) {
        const forwardOp = forwardList[i];
        const forwardText = gadie_extract_operation_text(forwardOp);

        if (!forwardText) continue;

        // Search for matching reverse operation
        for (let j = 0; j < reverseList.length; j++) {
            if (pairedReverse.has(j)) continue;

            const reverseOp = reverseList[j];
            const reverseText = gadie_extract_operation_text(reverseOp);

            if (forwardText === reverseText && forwardText.length > 0) {
                // Found a match - create reciprocal change
                const changeId = changeList.length;
                const colorId = reciprocalCount % 8; // 0-7 for reciprocal

                changeList.push({
                    changeId: changeId,
                    changeType: 'reciprocal',
                    forwardOps: [forwardOp],
                    reverseOps: [reverseOp],
                    colorId: colorId,
                    colorHex: GADIE_COLOR_PALETTE[colorId]
                });

                pairedForward.add(i);
                pairedReverse.add(j);
                reciprocalCount++;
                break;
            }
        }
    }

    // Step 2: Create monocular changes for unpaired reverse operations
    let monocularCount = 0;
    for (let j = 0; j < reverseList.length; j++) {
        if (pairedReverse.has(j)) continue;

        const changeId = changeList.length;
        const colorId = 8 + (monocularCount % 8); // 8-15 for monocular

        changeList.push({
            changeId: changeId,
            changeType: 'monocular',
            forwardOps: [],
            reverseOps: [reverseList[j]],
            colorId: colorId,
            colorHex: GADIE_COLOR_PALETTE[colorId]
        });

        monocularCount++;
    }

    // Step 3: Create monocular changes for unpaired forward operations
    for (let i = 0; i < forwardList.length; i++) {
        if (pairedForward.has(i)) continue;

        const changeId = changeList.length;
        const colorId = 8 + (monocularCount % 8); // 8-15 for monocular

        changeList.push({
            changeId: changeId,
            changeType: 'monocular',
            forwardOps: [forwardList[i]],
            reverseOps: [],
            colorId: colorId,
            colorHex: GADIE_COLOR_PALETTE[colorId]
        });

        monocularCount++;
    }

    return changeList;
}

// Helper: Wrap a text substring in a span with marking classes
function gadie_wrap_text_substring(parentNode, searchText, changeId, colorHex, isDeletion) {
    if (!parentNode || !searchText) return false;

    // If parent is a text node, wrap its parent element instead
    if (parentNode.nodeType === Node.TEXT_NODE) {
        parentNode = parentNode.parentNode;
    }

    if (!parentNode || parentNode.nodeType !== Node.ELEMENT_NODE) return false;

    // Get the text content of the parent
    const parentText = parentNode.textContent;
    if (!parentText.includes(searchText)) {
        return false; // Text not found in this element
    }

    // If the entire element is just the search text, mark the whole element
    if (parentText.trim() === searchText.trim()) {
        parentNode.classList.add(isDeletion ? 'gads-dual-deleted' : 'gads-dual-added');
        parentNode.style.backgroundColor = colorHex;
        parentNode.setAttribute('data-change-id', changeId);
        parentNode.classList.add(`gad-change-${changeId}`);
        return true;
    }

    // Partial match - need to wrap just the substring
    // This is complex because we need to handle mixed text and element nodes
    // Strategy: Find all text nodes in parent, locate the search text, and wrap it
    const walker = document.createTreeWalker(
        parentNode,
        NodeFilter.SHOW_TEXT,
        null,
        false
    );

    let textNode;
    let foundStart = false;
    let accumulatedText = '';

    while ((textNode = walker.nextNode())) {
        const nodeText = textNode.textContent;
        accumulatedText += nodeText;

        // Check if the search text is now complete in our accumulated text
        if (accumulatedText.includes(searchText) && !foundStart) {
            // Found it! Now we need to split and wrap
            const startIndex = accumulatedText.indexOf(searchText);
            const beforeText = accumulatedText.substring(0, startIndex);
            const afterText = accumulatedText.substring(startIndex + searchText.length);

            // Create wrapper span
            const wrapper = document.createElement('span');
            wrapper.classList.add(isDeletion ? 'gads-dual-deleted' : 'gads-dual-added');
            wrapper.style.backgroundColor = colorHex;
            wrapper.setAttribute('data-change-id', changeId);
            wrapper.classList.add(`gad-change-${changeId}`);
            wrapper.textContent = searchText;

            // This is simplified - just mark it without complex splitting
            // A full implementation would need to carefully reconstruct the DOM
            if (textNode.nodeType === Node.TEXT_NODE && nodeText.includes(searchText)) {
                // Simple case: text is all in one node
                const idx = nodeText.indexOf(searchText);
                if (idx !== -1) {
                    const before = document.createTextNode(nodeText.substring(0, idx));
                    const after = document.createTextNode(nodeText.substring(idx + searchText.length));

                    wrapper.textContent = searchText;

                    textNode.parentNode.insertBefore(before, textNode);
                    textNode.parentNode.insertBefore(wrapper, textNode);
                    textNode.parentNode.insertBefore(after, textNode);
                    textNode.parentNode.removeChild(textNode);
                    return true;
                }
            }

            foundStart = true;
            return true;
        }
    }

    return false;
}

// Function 2: Render left pane (from document with deletions marked)
function gadie_render_dual_left(fromDOM, changeList) {
    // Clone fromDOM to avoid modifying original
    const clonedDOM = gadie_create_dom_from_html(fromDOM.innerHTML);

    // Log that we're rendering left pane with marking
    if (typeof gadib_logger_d === 'function') gadib_logger_d(`[GADIE-RENDER] Starting LEFT pane marking for ${changeList.length} changes`);

    // Process each change that has reverse operations (deletions in left pane)
    for (const change of changeList) {
        if (change.reverseOps.length === 0) continue;

        for (const op of change.reverseOps) {
            if (!op.route) continue;

            const element = gadie_find_element_by_route(clonedDOM, op.route);
            if (!element) {
                // Route not found - log but don't fail silently
                gadie_log(`[GADIE-MARK] Left pane: Route [${op.route.join(',')}] not found for changeId ${change.changeId}`);
                continue;
            }

            let marked = false;
            const opText = op.oldValue || op.value;
            gadie_log(`[GADIE-MARK] Left pane changeId=${change.changeId}: element nodeType=${element.nodeType}, opText="${gadie_truncate(opText || '', 60)}"`);

            // Handle text nodes by wrapping them
            if (element.nodeType === Node.TEXT_NODE) {
                // For text nodes, try substring matching first if we have operation text
                if (opText && element.parentNode) {
                    gadie_log(`[GADIE-MARK] Left pane changeId=${change.changeId}: trying substring match in text node`);
                    marked = gadie_wrap_text_substring(
                        element.parentNode,
                        opText,
                        change.changeId,
                        change.colorHex,
                        true
                    );
                    if (marked) {
                        gadie_log(`[GADIE-MARK] Left pane changeId=${change.changeId}: substring match succeeded`);
                    } else {
                        gadie_log(`[GADIE-MARK] Left pane changeId=${change.changeId}: substring match failed, falling back`);
                    }
                }

                // Fallback: wrap the entire text node
                if (!marked) {
                    gadie_log(`[GADIE-MARK] Left pane changeId=${change.changeId}: wrapping entire text node`);
                    const wrapper = document.createElement('span');
                    wrapper.classList.add('gads-dual-deleted');
                    wrapper.style.backgroundColor = change.colorHex;
                    wrapper.setAttribute('data-change-id', change.changeId);
                    wrapper.classList.add(`gad-change-${change.changeId}`);
                    element.parentNode.insertBefore(wrapper, element);
                    wrapper.appendChild(element);
                    marked = true;
                }
            } else if (element.nodeType === Node.ELEMENT_NODE && element.classList) {
                // For element nodes, try substring matching in case it's a container
                if (opText) {
                    gadie_log(`[GADIE-MARK] Left pane changeId=${change.changeId}: trying substring match in element`);
                    marked = gadie_wrap_text_substring(
                        element,
                        opText,
                        change.changeId,
                        change.colorHex,
                        true
                    );
                    if (marked) {
                        gadie_log(`[GADIE-MARK] Left pane changeId=${change.changeId}: substring match succeeded`);
                    } else {
                        gadie_log(`[GADIE-MARK] Left pane changeId=${change.changeId}: substring match failed, falling back`);
                    }
                }

                // Fallback: mark the entire element
                if (!marked) {
                    gadie_log(`[GADIE-MARK] Left pane changeId=${change.changeId}: marking entire element`);
                    element.classList.add('gads-dual-deleted');
                    element.style.backgroundColor = change.colorHex;
                    element.setAttribute('data-change-id', change.changeId);
                    element.classList.add(`gad-change-${change.changeId}`);
                    marked = true;
                }
            } else {
                // Can't mark this element type
                gadie_log(`[GADIE-MARK] Left pane changeId=${change.changeId}: Cannot mark node type ${element.nodeType}`);
            }

            if (marked) {
                gadie_log(`[GADIE-MARK] Left pane changeId=${change.changeId}: Successfully marked`);
            }
        }
    }

    return clonedDOM.innerHTML;
}

// Function 3: Render right pane (to document with additions marked)
function gadie_render_dual_right(toDOM, changeList) {
    // Clone toDOM to avoid modifying original
    const clonedDOM = gadie_create_dom_from_html(toDOM.innerHTML);

    // Process each change that has forward operations (additions in right pane)
    for (const change of changeList) {
        if (change.forwardOps.length === 0) continue;

        for (const op of change.forwardOps) {
            if (!op.route) continue;

            const element = gadie_find_element_by_route(clonedDOM, op.route);
            if (!element) {
                // Route not found - log but don't fail silently
                gadie_log(`[GADIE-MARK] Right pane: Route [${op.route.join(',')}] not found for changeId ${change.changeId}`);
                continue;
            }

            let marked = false;
            const opText = op.newValue || op.value;
            gadie_log(`[GADIE-MARK] Right pane changeId=${change.changeId}: element nodeType=${element.nodeType}, opText="${gadie_truncate(opText || '', 60)}"`);

            // Handle text nodes by wrapping them
            if (element.nodeType === Node.TEXT_NODE) {
                // For text nodes, try substring matching first if we have operation text
                if (opText && element.parentNode) {
                    gadie_log(`[GADIE-MARK] Right pane changeId=${change.changeId}: trying substring match in text node`);
                    marked = gadie_wrap_text_substring(
                        element.parentNode,
                        opText,
                        change.changeId,
                        change.colorHex,
                        false
                    );
                    if (marked) {
                        gadie_log(`[GADIE-MARK] Right pane changeId=${change.changeId}: substring match succeeded`);
                    } else {
                        gadie_log(`[GADIE-MARK] Right pane changeId=${change.changeId}: substring match failed, falling back`);
                    }
                }

                // Fallback: wrap the entire text node
                if (!marked) {
                    gadie_log(`[GADIE-MARK] Right pane changeId=${change.changeId}: wrapping entire text node`);
                    const wrapper = document.createElement('span');
                    wrapper.classList.add('gads-dual-added');
                    wrapper.style.backgroundColor = change.colorHex;
                    wrapper.setAttribute('data-change-id', change.changeId);
                    wrapper.classList.add(`gad-change-${change.changeId}`);
                    element.parentNode.insertBefore(wrapper, element);
                    wrapper.appendChild(element);
                    marked = true;
                }
            } else if (element.nodeType === Node.ELEMENT_NODE && element.classList) {
                // For element nodes, try substring matching in case it's a container
                if (opText) {
                    gadie_log(`[GADIE-MARK] Right pane changeId=${change.changeId}: trying substring match in element`);
                    marked = gadie_wrap_text_substring(
                        element,
                        opText,
                        change.changeId,
                        change.colorHex,
                        false
                    );
                    if (marked) {
                        gadie_log(`[GADIE-MARK] Right pane changeId=${change.changeId}: substring match succeeded`);
                    } else {
                        gadie_log(`[GADIE-MARK] Right pane changeId=${change.changeId}: substring match failed, falling back`);
                    }
                }

                // Fallback: mark the entire element
                if (!marked) {
                    gadie_log(`[GADIE-MARK] Right pane changeId=${change.changeId}: marking entire element`);
                    element.classList.add('gads-dual-added');
                    element.style.backgroundColor = change.colorHex;
                    element.setAttribute('data-change-id', change.changeId);
                    element.classList.add(`gad-change-${change.changeId}`);
                    marked = true;
                }
            } else {
                // Can't mark this element type
                gadie_log(`[GADIE-MARK] Right pane changeId=${change.changeId}: Cannot mark node type ${element.nodeType}`);
            }

            if (marked) {
                gadie_log(`[GADIE-MARK] Right pane changeId=${change.changeId}: Successfully marked`);
            }
        }
    }

    return clonedDOM.innerHTML;
}

// Helper function to format operation summary with action and text content
function gadie_format_operation_summary(op) {
    const action = op.action || 'unknown';
    const text = gadie_extract_operation_text(op);

    if (text) {
        return `<span class="gad-op-action">${gadie_escape_html(action)}</span>: <span class="gad-op-text">"${gadie_escape_html(gadie_truncate(text, 80))}"</span>`;
    }
    return `<span class="gad-op-action">${gadie_escape_html(action)}</span>`;
}

// Function 4: Render change entries list
function gadie_render_change_entries(changeList, operations) {
    if (changeList.length === 0) {
        return '<div class="gad-change-empty">No changes detected</div>';
    }

    // Build HTML for each change entry
    const entries = changeList.map(change => {
        const buttons = [];
        const reverseOpDetails = [];
        const forwardOpDetails = [];

        // Process reverse operations (deletions in left pane)
        for (let i = 0; i < change.reverseOps.length; i++) {
            const op = change.reverseOps[i];
            const routeStr = op.route ? JSON.stringify(op.route) : '[]';
            const summary = gadie_format_operation_summary(op);

            buttons.push(`<button class="gad-operation-button gad-op-left"
                data-change-id="${change.changeId}"
                data-operation-index="${i}"
                data-pane="left"
                data-route='${routeStr}'>
                ← Left
            </button>`);

            reverseOpDetails.push(`<div class="gad-change-operation-detail gad-change-reverse">
                <div class="gad-op-summary">${summary}</div>
            </div>`);
        }

        // Process forward operations (additions in right pane)
        for (let i = 0; i < change.forwardOps.length; i++) {
            const op = change.forwardOps[i];
            const routeStr = op.route ? JSON.stringify(op.route) : '[]';
            const summary = gadie_format_operation_summary(op);

            buttons.push(`<button class="gad-operation-button gad-op-right"
                data-change-id="${change.changeId}"
                data-operation-index="${i}"
                data-pane="right"
                data-route='${routeStr}'>
                → Right
            </button>`);

            forwardOpDetails.push(`<div class="gad-change-operation-detail gad-change-forward">
                <div class="gad-op-summary">${summary}</div>
            </div>`);
        }

        const operationsHTML = `
            ${reverseOpDetails.length > 0 ? `<div class="gad-change-ops-section"><span class="gad-ops-label">Deleted (Left):</span>${reverseOpDetails.join('')}</div>` : ''}
            ${forwardOpDetails.length > 0 ? `<div class="gad-change-ops-section"><span class="gad-ops-label">Added (Right):</span>${forwardOpDetails.join('')}</div>` : ''}
        `;

        return `<div class="gad-change-entry" data-change-id="${change.changeId}" style="background-color: ${change.colorHex}">
            <div class="gad-change-label">Change #${change.changeId + 1} (${change.changeType})</div>
            <div class="gad-change-operations">
                ${operationsHTML}
            </div>
            <div class="gad-change-buttons">
                ${buttons.join('\n                ')}
            </div>
        </div>`;
    }).join('\n        ');

    return entries;
}

// Function 5: Build complete dual view HTML structure
function gadie_build_dual_view_html(leftRendered, rightRendered, changePaneHTML) {
    return `<div class="gad-dual-view">
        <div class="gad-dual-panes">
            <div class="gad-left-pane" id="dualLeftPane">
                ${leftRendered}
            </div>
            <div class="gad-right-pane" id="dualRightPane">
                ${rightRendered}
            </div>
        </div>
        <div class="gad-resize-divider" id="dualResizeDivider"></div>
        <div class="gad-changes-pane" id="dualChangesPane">
            ${changePaneHTML}
        </div>
    </div>`;
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

// Generate route mapping table for debug analysis
function gadie_generate_route_map(dom) {
    const routeMap = {};
    
    function traverseNode(node, route) {
        const routeKey = route.join(',');
        
        // Build node description
        let description = '';
        if (node.nodeType === Node.ELEMENT_NODE) {
            description = `<${node.tagName.toLowerCase()}`;
            if (node.id) description += ` id="${node.id}"`;
            if (node.className) description += ` class="${node.className}"`;
            description += `>`;
        } else if (node.nodeType === Node.TEXT_NODE) {
            const text = node.textContent.trim();
            description = `TEXT: "${text.length > 50 ? text.substring(0, 47) + '...' : text}"`;
        } else {
            description = `${node.nodeType}`;
        }
        
        routeMap[routeKey] = {
            route: [...route],
            nodeType: node.nodeType,
            tagName: node.nodeType === Node.ELEMENT_NODE ? node.tagName : null,
            textContent: node.nodeType === Node.TEXT_NODE ? node.textContent : null,
            description: description,
            childCount: node.childNodes ? node.childNodes.length : 0
        };
        
        // Recurse through children
        if (node.childNodes) {
            for (let i = 0; i < node.childNodes.length; i++) {
                traverseNode(node.childNodes[i], [...route, i]);
            }
        }
    }
    
    traverseNode(dom, []);
    return routeMap;
}

// Enhanced CSS styling function with navigation markers for diff operations
function gadie_apply_css_styling(dom, operations) {
    operations.forEach((op, index) => {
        const operationId = `gadisdp-op-${index}`;
        
        if (op.action === 'addElement' || op.action === 'addTextElement') {
            const element = gadie_find_element_by_route(dom, op.route);
            if (element && element.nodeType === Node.ELEMENT_NODE && element.classList) {
                const isBlock = gadie_is_block(element.tagName);
                element.classList.add(isBlock ? 'gads-addition-block' : 'gads-addition-inline');
                
                // Insert navigation markers around the element
                gadie_insert_navigation_markers(element, operationId, op.action);
            }
        } else if (op.action === 'modifyTextElement' || op.action === 'modifyAttribute') {
            const element = gadie_find_element_by_route(dom, op.route);
            if (element && element.nodeType === Node.ELEMENT_NODE && element.classList) {
                const isBlock = gadie_is_block(element.tagName);
                element.classList.add(isBlock ? 'gads-modification-structural' : 'gads-modification-inline');
                
                // Insert navigation markers around the element
                gadie_insert_navigation_markers(element, operationId, op.action);
            }
        }
    });
}

// Insert route start and end markers around an element
function gadie_insert_navigation_markers(element, operationId, action) {
    if (!element || !element.parentNode) return;
    
    // Create start marker with backlink to annotation
    const startMarker = document.createElement('span');
    startMarker.id = `${operationId}-start`;
    startMarker.className = 'gadisdp-route-start-marker';
    startMarker.setAttribute('data-operation-id', operationId);
    startMarker.setAttribute('data-operation-action', action);
    startMarker.setAttribute('title', `Start of ${action} operation - Click to see details`);
    startMarker.innerHTML = `<a href="#annotation-${operationId}" class="gad-backlink gad-backlink--start" title="View operation details">◀</a>`;
    
    // Create end marker with backlink to annotation
    const endMarker = document.createElement('span');
    endMarker.id = `${operationId}-end`;
    endMarker.className = 'gadisdp-route-end-marker';
    endMarker.setAttribute('data-operation-id', operationId);
    endMarker.setAttribute('data-operation-action', action);
    endMarker.setAttribute('title', `End of ${action} operation - Click to see details`);
    endMarker.innerHTML = `<a href="#annotation-${operationId}" class="gad-backlink gad-backlink--end" title="View operation details">▶</a>`;
    
    // Insert markers before and after the element
    element.parentNode.insertBefore(startMarker, element);
    element.parentNode.insertBefore(endMarker, element.nextSibling);
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
    if (action.includes('move') || action.includes('relocate')) return 'move';
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

// Helper function to safely extract content from a relocateGroup operation
function gadie_extract_relocate_content(op, sourceDOM) {
    if (!sourceDOM || !op.route || !Array.isArray(op.route)) {
        return null;
    }
    
    try {
        // Find the container element using the route
        const container = gadie_find_element_by_route(sourceDOM, op.route);
        if (!container || !container.childNodes) {
            return null;
        }
        
        const groupLength = op.groupLength || 1;
        const fromPos = op.from || 0;
        
        // Extract the content that would be moved - VERBATIM, NO ABBREVIATION
        const extractedElements = [];
        const renderedElements = []; // For proper HTML rendering
        
        for (let i = 0; i < groupLength && (fromPos + i) < container.childNodes.length; i++) {
            const element = container.childNodes[fromPos + i];
            if (element) {
                if (element.nodeType === Node.TEXT_NODE) {
                    const text = element.textContent || '';
                    if (text) {
                        extractedElements.push({ type: 'text', content: text });
                        renderedElements.push({ type: 'text', content: text });
                    }
                } else if (element.nodeType === Node.ELEMENT_NODE) {
                    const tagName = element.tagName?.toLowerCase() || 'unknown';
                    const textContent = element.textContent || '';
                    const outerHTML = element.outerHTML || `<${tagName}>...</${tagName}>`;
                    
                    // Store both the full content and the rendered HTML
                    extractedElements.push({ 
                        type: 'element', 
                        tagName, 
                        textContent, 
                        outerHTML 
                    });
                    renderedElements.push({ 
                        type: 'element', 
                        tagName, 
                        textContent, 
                        outerHTML 
                    });
                }
            }
        }
        
        return extractedElements.length > 0 ? { raw: extractedElements, rendered: renderedElements } : null;
    } catch (error) {
        // Safely handle any DOM access errors
        return null;
    }
}

// Render individual operation payload
function gadie_render_op_payload(op, sourceDOM = null) {
    const action = op.action;
    
    switch (action) {
        case 'modifyTextElement':
            const oldText = gadie_escape_html(op.oldValue || '');
            const newText = gadie_escape_html(op.newValue || '');
            return `<div class="gad-op__text-change">
                <div class="gad-op__before-after">
                    <span class="gad-change__label">Before:</span> 
                    <span class="gad-change__old">"${oldText}"</span>
                </div>
                <div class="gad-op__before-after">
                    <span class="gad-change__label">After:</span> 
                    <span class="gad-change__new">"${newText}"</span>
                </div>
                <div class="gad-op__token-level">
                    <details>
                        <summary>Token-level diff</summary>
                        ${gadie_render_token_diff(op.oldValue, op.newValue)}
                    </details>
                </div>
            </div>`;
            
        case 'removeTextElement':
        case 'addTextElement':
            const value = gadie_escape_html(op.value || '(missing)');
            const truncated = gadie_truncate(value, 200);
            const className = action === 'removeTextElement' ? 'gad-token--del' : 'gad-token--ins';
            return `<div class="gad-token ${className}">${truncated}</div>`;
            
        case 'replaceElement':
            // Extract both old and new elements with full details
            const oldElement = op.oldValue;
            const newElement = op.newValue;
            
            // Helper function to extract element details
            const extractElementDetails = (element) => {
                if (!element) return null;
                
                const details = {
                    nodeName: element.nodeName || '(unknown)',
                    textContent: '',
                    outerHTML: '',
                    isTextNode: false
                };
                
                if (element.nodeName === '#text') {
                    details.isTextNode = true;
                    details.textContent = element.data || '';
                    details.outerHTML = element.data || '';
                } else {
                    details.textContent = element.textContent || '';
                    // Try to reconstruct outerHTML from element structure
                    if (element.attributes && element.childNodes) {
                        const attrs = Object.keys(element.attributes || {})
                            .map(key => `${key}="${element.attributes[key]}"`)
                            .join(' ');
                        const attrStr = attrs ? ` ${attrs}` : '';
                        const childContent = element.childNodes ? 
                            element.childNodes.map(child => 
                                child.nodeName === '#text' ? child.data : `<${child.nodeName}>`
                            ).join('') : details.textContent;
                        details.outerHTML = `<${details.nodeName}${attrStr}>${childContent}</${details.nodeName}>`;
                    } else {
                        details.outerHTML = `<${details.nodeName}>${details.textContent}</${details.nodeName}>`;
                    }
                }
                
                return details;
            };
            
            const oldDetails = extractElementDetails(oldElement);
            const newDetails = extractElementDetails(newElement);
            
            return `<div class="gad-op__replace">
                <div class="gad-replace__summary">
                    <span class="gad-replace__operation">Element replacement</span>
                </div>
                
                <div class="gad-replace__verbatim-content">
                    <div class="gad-replace__content-header">
                        <span class="gad-replace__label">Verbatim content replacement:</span>
                    </div>
                    <div class="gad-replace__old-content">
                        <div class="gad-replace__section-label">Removed:</div>
                        <div class="gad-replace__content-item gad-replace__content-old">
                            ${oldDetails ? `
                                ${oldDetails.isTextNode ? 
                                    `<div class="gad-replace__element-text">${gadie_escape_html(oldDetails.textContent)}</div>` :
                                    `<div class="gad-replace__element-tag">&lt;${gadie_escape_html(oldDetails.nodeName)}&gt;</div>
                                     <div class="gad-replace__element-text">${gadie_escape_html(oldDetails.textContent)}</div>
                                     <div class="gad-replace__element-html">${gadie_escape_html(oldDetails.outerHTML)}</div>`
                                }
                            ` : '<div class="gad-replace__missing">(missing element)</div>'}
                        </div>
                    </div>
                    <div class="gad-replace__new-content">
                        <div class="gad-replace__section-label">Added:</div>
                        <div class="gad-replace__content-item gad-replace__content-new">
                            ${newDetails ? `
                                ${newDetails.isTextNode ? 
                                    `<div class="gad-replace__element-text">${gadie_escape_html(newDetails.textContent)}</div>` :
                                    `<div class="gad-replace__element-tag">&lt;${gadie_escape_html(newDetails.nodeName)}&gt;</div>
                                     <div class="gad-replace__element-text">${gadie_escape_html(newDetails.textContent)}</div>
                                     <div class="gad-replace__element-html">${gadie_escape_html(newDetails.outerHTML)}</div>`
                                }
                            ` : '<div class="gad-replace__missing">(missing element)</div>'}
                        </div>
                    </div>
                </div>
                
                <div class="gad-replace__rendered-content">
                    <div class="gad-replace__content-header">
                        <span class="gad-replace__label">Rendered appearance change:</span>
                    </div>
                    <div class="gad-replace__before-after">
                        <div class="gad-replace__before">
                            <div class="gad-replace__section-label">Before:</div>
                            <div class="gad-replace__rendered-preview gad-replace__rendered-old">
                                ${oldDetails ? oldDetails.outerHTML : '(missing)'}
                            </div>
                        </div>
                        <div class="gad-replace__after">
                            <div class="gad-replace__section-label">After:</div>
                            <div class="gad-replace__rendered-preview gad-replace__rendered-new">
                                ${newDetails ? newDetails.outerHTML : '(missing)'}
                            </div>
                        </div>
                    </div>
                </div>
            </div>`;
            
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
            
        case 'relocateGroup':
            const groupLength = op.groupLength || 1;
            const fromPos = op.from !== undefined ? op.from : '?';
            const toPos = op.to !== undefined ? op.to : '?';
            const routeStr = op.route ? `[${op.route.join(', ')}]` : '[]';
            const groupDesc = groupLength === 1 ? '1 element' : `${groupLength} elements`;
            
            // Try to infer what's being moved based on the operation context
            let contentHint = '';
            let routeHint = '';
            
            // Analyze the route to give context about where the move is happening
            if (op.route && op.route.length > 0) {
                const routeDepth = op.route.length;
                if (routeDepth <= 2) {
                    routeHint = 'Top-level content';
                } else if (routeDepth <= 4) {
                    routeHint = 'Section-level content';
                } else {
                    routeHint = 'Deeply nested content';
                }
            }
            
            // Provide better content description
            if (op.groupLength === 1) {
                contentHint = routeHint ? `${routeHint} - single element` : 'Single element or text block';
            } else if (op.groupLength <= 3) {
                contentHint = routeHint ? `${routeHint} - small group (${op.groupLength} items)` : `Small group of ${op.groupLength} related elements`;
            } else if (op.groupLength <= 10) {
                contentHint = routeHint ? `${routeHint} - medium section (${op.groupLength} items)` : `Medium section with ${op.groupLength} elements`;
            } else {
                contentHint = routeHint ? `${routeHint} - large section (${op.groupLength} items)` : `Large section with ${op.groupLength} elements`;
            }
            
            // Add movement description based on positions
            let movementDesc = '';
            if (fromPos !== '?' && toPos !== '?') {
                if (toPos > fromPos) {
                    movementDesc = `(moved ${toPos - fromPos} positions forward)`;
                } else if (fromPos > toPos) {
                    movementDesc = `(moved ${fromPos - toPos} positions backward)`;
                } else {
                    movementDesc = '(position unchanged)';
                }
            }
            
            // Try to extract actual content being moved
            const extractedData = gadie_extract_relocate_content(op, sourceDOM);
            let contentPreview = '';
            let renderedPreview = '';
            
            if (extractedData && extractedData.raw && extractedData.raw.length > 0) {
                // Raw content display (verbatim, unabbreviated)
                const rawContentItems = extractedData.raw.map(item => {
                    if (item.type === 'text') {
                        return `<div class="gad-relocate__content-item gad-relocate__content-text">${gadie_escape_html(item.content)}</div>`;
                    } else if (item.type === 'element') {
                        return `<div class="gad-relocate__content-item gad-relocate__content-element">
                            <div class="gad-relocate__element-tag">&lt;${gadie_escape_html(item.tagName)}&gt;</div>
                            <div class="gad-relocate__element-text">${gadie_escape_html(item.textContent)}</div>
                            <div class="gad-relocate__element-html">${gadie_escape_html(item.outerHTML)}</div>
                        </div>`;
                    }
                    return '';
                }).join('');
                
                contentPreview = `
                    <div class="gad-relocate__actual-content">
                        <div class="gad-relocate__content-header">
                            <span class="gad-relocate__label">Verbatim content being moved:</span>
                        </div>
                        <div class="gad-relocate__content-list">
                            ${rawContentItems}
                        </div>
                    </div>`;
                
                // Rendered content display (how it appears visually)
                const renderedContentItems = extractedData.rendered.map(item => {
                    if (item.type === 'text') {
                        return item.content;
                    } else if (item.type === 'element') {
                        return item.outerHTML;
                    }
                    return '';
                }).join('');
                
                renderedPreview = `
                    <div class="gad-relocate__rendered-content">
                        <div class="gad-relocate__content-header">
                            <span class="gad-relocate__label">Rendered appearance:</span>
                        </div>
                        <div class="gad-relocate__rendered-preview">
                            ${renderedContentItems}
                        </div>
                    </div>`;
            }
            
            return `<div class="gad-op__relocate">
                <div class="gad-relocate__summary">
                    <span class="gad-relocate__group">${groupDesc}</span> moved within container
                </div>
                <div class="gad-relocate__content">
                    <span class="gad-relocate__label">Content type:</span>
                    <span class="gad-relocate__hint">${contentHint}</span>
                    ${movementDesc ? `<span class="gad-relocate__movement">${movementDesc}</span>` : ''}
                </div>
                ${contentPreview}
                ${renderedPreview}
                <div class="gad-relocate__details">
                    <div class="gad-relocate__positions">
                        <span class="gad-relocate__label">From position:</span>
                        <span class="gad-relocate__pos gad-relocate__from">${fromPos}</span>
                        <span class="gad-relocate__arrow">→</span>
                        <span class="gad-relocate__label">To position:</span>
                        <span class="gad-relocate__pos gad-relocate__to">${toPos}</span>
                    </div>
                    <div class="gad-relocate__container">
                        <span class="gad-relocate__label">Container route:</span>
                        <span class="gad-relocate__route">${gadie_escape_html(routeStr)}</span>
                    </div>
                </div>
            </div>`;
            
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
function gadie_render_operations_html(operations, manifest = null, sourceDOM = null) {
    if (!operations || operations.length === 0) {
        return '<section class="gad-ops"><div class="gad-ops__prov">No operations to display</div></section>';
    }
    
    // Build provenance line
    let provenance = 'Diff Dom Annotations';
    
    // Build operation list
    const opItems = operations.map((op, index) => {
        const actionFamily = gadie_get_action_family(op.action);
        const routeStr = op.route ? JSON.stringify(op.route) : '[]';
        
        // Raw diff-dom operation display - format route array on single line
        const rawOpData = JSON.stringify(op, (key, value) => {
            // Keep route arrays on single line
            if (key === 'route' && Array.isArray(value)) {
                return value;
            }
            return value;
        }, 2).replace(/"route":\s*\[\s*([^\]]+)\s*\]/g, (match, content) => {
            // Compress route array to single line
            const compactRoute = content.replace(/\s+/g, '').replace(/,/g, ', ');
            return `"route": [${compactRoute}]`;
        });
        
        const operationId = `gadisdp-op-${index}`;
        const navigationLinks = `
            <span class="gad-op__navigation">
                <a href="#${operationId}-start" class="gad-nav-link gad-nav-link--start" title="Jump to start of change">▲ Start</a>
                <a href="#${operationId}-end" class="gad-nav-link gad-nav-link--end" title="Jump to end of change">▼ End</a>
            </span>
        `;
        
        return `<li class="gad-op" id="annotation-${operationId}">
            <div class="gad-op__head">
                <span class="gad-op__idx">${index}</span>
                <span class="gad-badge gad-badge--${actionFamily}">${gadie_escape_html(op.action)}</span>
                <span class="gad-op__route">${gadie_escape_html(routeStr)}</span>
                ${navigationLinks}
            </div>
            <div class="gad-op__payload">
                <div class="gad-op__formatted">
                    ${gadie_render_op_payload(op, sourceDOM)}
                </div>
                <div class="gad-op__raw">
                    <pre class="gad-raw-data">${gadie_escape_html(rawOpData)}</pre>
                </div>
            </div>
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
