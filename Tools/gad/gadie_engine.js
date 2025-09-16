// GADIE: GAD Inspector Diff Engine
// Extracted from monolithic gadi_inspector.html for modular architecture
// Contains: All 9-phase diff methods, DOM helpers, DFK operations, coalescing logic

// Verbosity gating helper - hardcoded to 'low' for now
function gadie_should_log(level, opts) {
    const verbosity = opts.verbosity || 'low';  // hardcoded to 'low' 
    if (verbosity === 'low') return level === 'error';
    if (verbosity === 'warn') return ['error', 'warn'].includes(level);
    if (verbosity === 'debug') return true;
    return false;
}

// Main diff processing function - exposed as the primary interface
async function gadie_diff(fromHtml, toHtml, opts = {}) {
    const { fromCommit, toCommit, sourceFiles } = opts;
    const enableAnchorFallback = opts.enableAnchorFallback !== undefined ? opts.enableAnchorFallback : true;
    
    const startTime = performance.now();
    if (gadie_should_log('debug', opts)) {
        gadib_logger_d('Starting GADS-compliant 9-phase diff processing');
        gadib_logger_d(`Input sizes: fromHtml=${fromHtml.length} chars, toHtml=${toHtml.length} chars`);
        gadib_logger_d(`diff-dom type at diff time: ${typeof window.diffDom}`);
    }

    if (typeof window.diffDom === 'undefined' || typeof window.diffDom.DiffDOM === 'undefined') {
        throw new Error('diff-dom library not available - required for diff processing');
    }

    try {
        // Phase 1: Immutable Input - Establish immutable source DOM trees
        const immutableFromDOM = gadie_create_dom_from_html(fromHtml);
        const immutableToDOM = gadie_create_dom_from_html(toHtml);

        // Phase 2: Detached Working - Create Detached Working DOM with Semantic Anchors
        const detachedWorkingDOM = gadie_create_detached_working_dom(immutableToDOM);
        const semanticAnchors = gadie_establish_semantic_anchors(detachedWorkingDOM);

        // Phase 3: Deletion Fact Capture - Generate Deletion Fact Keys (DFK)
        const deletionFactTable = await gadie_create_deletion_fact_table(immutableFromDOM);
        
        // Phase 3 Telemetry: emit summary from the live DFT used in later phases
        const phase3Telemetry = { 
            count: Object.keys(deletionFactTable).length, 
            sampleKeys: Object.keys(deletionFactTable).slice(0, 5),
            sampleEntries: Object.keys(deletionFactTable).slice(0, 3).map(key => ({
                key: key,
                route: deletionFactTable[key].route,
                routeStr: deletionFactTable[key].routeStr
            }))
        };
        if (opts.debugArtifacts) {
            gadib_factory_ship('phase3_dft', JSON.stringify(phase3Telemetry, null, 2), fromCommit, toCommit, sourceFiles);
        }

        // Phase 4: Diff Operations - Generate structured diff operations
        const diffDOM = new window.diffDom.DiffDOM({
            debug: false,  // reduced chatter
            diffcap: 500  // Allow more granular detection
        });
        const diffOperations = diffDOM.diff(immutableFromDOM, immutableToDOM);
        gadie_enhance_operations_with_dfk(diffOperations, deletionFactTable);

        // Phase 5: Semantic Classification - Classify operations into semantic change types
        const classifiedOperations = gadie_classify_semantic_changes(diffOperations, immutableFromDOM, immutableToDOM);
        
        // Phase 5 Debug Output - Show enriched operations with semantic metadata
        const phase5Breakdown = gadie_generate_semantic_breakdown(classifiedOperations);
        const totalDeletions = (phase5Breakdown.INLINE_REMOVAL || 0) + (phase5Breakdown.BLOCK_REMOVAL || 0);
        
        const phase45Debug = {
            totalOperations: classifiedOperations.length,
            semanticBreakdown: phase5Breakdown,
            totalDeletions: totalDeletions,
            sampleOperations: classifiedOperations.slice(0, 5).map(op => ({
                action: op.action,
                route: op.route,
                semanticType: op.semanticType,
                visualTreatment: op.visualTreatment
            }))
        };
        if (opts.debugArtifacts) {
            gadib_factory_ship('phase5_classified', JSON.stringify(phase45Debug, null, 2), fromCommit, toCommit, sourceFiles);
        }

        // Phase 6: Annotated Assembly - Construct semantically annotated output DOM
        const {resolved, unresolved} = gadie_partition_by_resolvability(detachedWorkingDOM, classifiedOperations);
        // ship quarantine for visibility
        if (opts.debugArtifacts) {
            gadib_factory_ship('phase6_quarantine_unresolved_ops', JSON.stringify(unresolved, null, 2), fromCommit, toCommit, sourceFiles);
        }
        const assemblyResult = gadie_assemble_annotated_dom(detachedWorkingDOM, resolved, semanticAnchors, fromCommit, toCommit, enableAnchorFallback);
        const assembledDOM = assemblyResult.outputDOM;
        const assembledHTML = assembledDOM.innerHTML;
        if (opts.debugArtifacts) {
            gadib_factory_ship('phase6_annotated', assembledHTML, fromCommit, toCommit, sourceFiles);
        }

        // Phase 7: Deletion Placement - Position deletion blocks using DFK and Semantic Anchors
        const allDeletionOps = classifiedOperations.filter(op =>
            op.type === 'deletion' || (op.action && op.action.startsWith('remove')) ||
            (op.semanticType && /REMOVAL/i.test(op.semanticType))
        );
        const deletionPlacementResult = await gadie_place_deletion_blocks(
            assembledDOM, allDeletionOps, deletionFactTable, semanticAnchors, allDeletionOps.length, enableAnchorFallback
        );
        const deletionPlacedDOM = deletionPlacementResult.dom;
        const deletionPlacedHTML = deletionPlacedDOM.innerHTML;
        
        // Emit invariant check
        const placementTotal = deletionPlacementResult.telemetry.exact + deletionPlacementResult.telemetry.fallback + 
                              deletionPlacementResult.telemetry.ambiguous + deletionPlacementResult.telemetry.unplaced;
        if (placementTotal !== totalDeletions) {
            const errorMsg = `PLACEMENT INVARIANT VIOLATION: totalDeletions=${totalDeletions} but placement sum=${placementTotal}`;
            if (gadie_should_log('error', opts)) {
                gadib_logger_e(errorMsg);
                console.error(`[INVARIANT-ERROR] ${errorMsg}`);
            }
            deletionPlacementResult.telemetry.invariant_error = errorMsg;
        }
        
        // Step 16: Phase-7 deletions file - must reflect every placed badge with data-gad-placement present
        const placedBadges = deletionPlacedDOM.querySelectorAll('[data-gad-placement]');
        const deletionDebugInfo = {
            html: deletionPlacedHTML,
            badge_count: placedBadges.length,
            badges_with_placement: Array.from(placedBadges).map(badge => ({
                placement: badge.getAttribute('data-gad-placement'),
                key: badge.getAttribute('data-gad-key'),
                kind: badge.getAttribute('data-dfk-kind'),
                tag: badge.getAttribute('data-dfk-tag'),
                classes: Array.from(badge.classList).join(' ')
            }))
        };
        
        // Step 17: Phase-7 telemetry file - must include full counts plus ambiguous_examples
        const enhancedTelemetry = {
            ...deletionPlacementResult.telemetry,
            debug_info: {
                total_deletions_expected: totalDeletions,
                placement_sum: placementTotal,
                invariant_satisfied: placementTotal === totalDeletions
            },
            ambiguous_examples: (deletionPlacementResult.telemetry.ambiguous_examples || []).slice(0, 10)
        };
        
        // Ship phase 7 outputs with enhanced debug artifacts
        gadib_factory_ship('phase7_deletions', JSON.stringify(deletionDebugInfo, null, 2), fromCommit, toCommit, sourceFiles);
        gadib_factory_ship('phase7_telemetry', JSON.stringify(enhancedTelemetry, null, 2), fromCommit, toCommit, sourceFiles);

        // Phase 8: Uniform Classing - Merge consecutive same-nature elements
        const coalescedDOM = gadie_merge_adjacent_same_nature(deletionPlacedDOM);
        const coalescedHTML = coalescedDOM.innerHTML;
        if (opts.debugArtifacts) {
            gadib_factory_ship('phase8_coalesced', coalescedHTML, fromCommit, toCommit, sourceFiles);
        }

        // Phase 9: Serialize - Generate final rendered output
        const finalHTML = gadie_serialize_final_output(coalescedDOM);
        gadib_factory_ship('phase9_final', finalHTML, fromCommit, toCommit, sourceFiles);

        return finalHTML;

    } catch (error) {
        if (gadie_should_log('error', opts)) {
            gadib_logger_e(`GADS 9-phase diff processing failed: ${error.message}`);
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

// Create detached working DOM for independent processing
function gadie_create_detached_working_dom(immutableToDOM) {
    // Create completely independent working DOM for semantic processing
    return immutableToDOM.cloneNode(true);
}

// Establish semantic anchors for routing operations
function gadie_establish_semantic_anchors(detachedWorkingDOM) {
    const anchors = {};
    const traverse = (element, route = []) => {
        if (element.nodeType === Node.ELEMENT_NODE) {
            // Create semantic anchors at key structural points
            const tagName = element.tagName.toLowerCase();
            if (['h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'p', 'div', 'section', 'article'].includes(tagName)) {
                const routeStr = route.join(',');
                // Inline semantic type determination
                let semanticType = 'generic';
                if (['h1', 'h2', 'h3', 'h4', 'h5', 'h6'].includes(tagName)) semanticType = 'heading';
                else if (tagName === 'p') semanticType = 'paragraph';
                else if (['div', 'section', 'article'].includes(tagName)) semanticType = 'container';
                
                anchors[routeStr] = {
                    element: element,
                    route: [...route],
                    tag: tagName,
                    semanticType: semanticType
                };
            }
            
            // Recurse through children
            Array.from(element.childNodes).forEach((child, index) => {
                traverse(child, [...route, index]);
            });
        }
    };
    
    traverse(detachedWorkingDOM);
    return anchors;
}


// Anchor fallback helper - try when route traversal fails
function gadie_anchor_fallback(route, semanticAnchors) {
    // nearest existing ancestor route → same-tag anchor
    for (let cut = route.length - 1; cut >= 0; cut--) {
        const key = (cut === 0 ? 'root' : route.slice(0, cut).join(','));
        const a = semanticAnchors[key];
        if (a) return a.element;
    }
    return null;
}

// DOM Manipulation Helper Methods
function gadie_find_element_by_route(dom, route) {
    let element = dom;
    for (let i = 0; i < route.length; i++) {
        const index = route[i];
        if (element.childNodes && element.childNodes[index]) {
            element = element.childNodes[index];
        } else {
            const availableCount = element.childNodes ? element.childNodes.length : 0;
            
            // Route traversal failed - structured error
            gadib_logger_e(`Route traversal failed: route=[${route.join(',')}] pos=${i} index=${index} available=${availableCount}`);
            return null;
        }
    }
    return element;
}


// Utility Helper Methods
function gadie_escape_html(unsafe) {
    return unsafe
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#039;");
}

function gadie_extract_text_from_html(htmlString) {
    try {
        // Create a temporary element to parse HTML and extract text content
        const tempDiv = document.createElement('div');
        tempDiv.innerHTML = htmlString;
        return tempDiv.textContent || tempDiv.innerText || '';
    } catch (e) {
        // Fallback: basic tag stripping
        return htmlString.replace(/<[^>]*>/g, '').trim();
    }
}

// Deletion Fact Table Generation
async function gadie_create_deletion_fact_table(immutableFromDOM) {
    gadib_logger_d('Creating GADS-compliant 4-field DFK mapping for stable fragment preservation');
    const deletionFactTable = {};
    
    // GADS Canonical DFK Schema: 4-field canonicalization
    const factDedupeMap = new Map();
    let beforeDedupeCount = 0;
    let dedupeDropCount = 0;
    let collisionCount = 0;
    
    const mapElementForDeletion = async (element, route) => {
        if (!element) return;
        
        beforeDedupeCount++;
        
        // Field 1: Route (absolute route in "from" DOM at capture time)
        const routeStr = route.length === 0 ? "root" : route.join(',');
        
        // Field 2: Kind (node kind)
        const kind = element.nodeType === Node.ELEMENT_NODE ? '#element' :
                    element.nodeType === Node.TEXT_NODE ? '#text' :
                    element.nodeType === Node.COMMENT_NODE ? '#comment' :
                    `#nodeType${element.nodeType}`;
        
        // Field 3: Tag (uppercased tag name for elements, #text for text nodes)
        const tag = element.nodeType === Node.ELEMENT_NODE ? element.tagName.toUpperCase() :
                   element.nodeType === Node.TEXT_NODE ? '#text' :
                   element.nodeName?.toUpperCase() || '#unknown';
        
        // Field 4: Payload hash (SHA-256 of normalized payload)
        const normalizedPayload = gadib_normalize_payload(element);
        const payloadHash = await gadib_hash(normalizedPayload);
        
        // GADS Canonical Key Format: dfk:<route>|<kind>|<tag>|sha256:<hex>
        const canonicalKey = `dfk:${routeStr}|${kind}|${tag}|${payloadHash}`;
        
        // Development assertion: Verify canonical key format compliance
        if (!/^dfk:.*\|sha256:[a-f0-9]{64}$/.test(canonicalKey)) {
            gadib_logger_e(`DFK-FORMAT-ERROR: Malformed key "${canonicalKey}" - expected sha256:<64-hex> suffix`);
            throw new Error(`Invalid DFK key format: ${canonicalKey}`);
        }
        
        // GADS Fact-level Dedupe Policy: Only drop when ALL four fields match
        const dedupeKey = `${routeStr}|${kind}|${tag}|${payloadHash}`;
        if (factDedupeMap.has(dedupeKey)) {
            dedupeDropCount++;
            return;
        }
        
        // GADS Collision Handling: If same hash but different payload, suffix with :v2, :v3
        let finalKey = canonicalKey;
        let version = 1;
        while (deletionFactTable[finalKey] && 
               deletionFactTable[finalKey].normalizedPayload !== normalizedPayload) {
            version++;
            finalKey = `${canonicalKey}:v${version}`;
            collisionCount++;
        }
        
        factDedupeMap.set(dedupeKey, true);
        
        // Store with enhanced metadata for GADS compliance
        deletionFactTable[finalKey] = {
            route: [...route],
            kind: kind,
            tag: tag,
            payloadHash: payloadHash,
            normalizedPayload: normalizedPayload,
            nodeType: element.nodeType,
            outerHTML: element.nodeType === Node.ELEMENT_NODE ? element.outerHTML : null,
            textContent: element.textContent || '',
            routeStr: routeStr
        };
        
        // Recursively map child nodes
        if (element.childNodes) {
            for (let i = 0; i < element.childNodes.length; i++) {
                const childRoute = [...route, i];
                await mapElementForDeletion(element.childNodes[i], childRoute);
            }
        }
    };
    
    // Start mapping from immutable "from" DOM
    await mapElementForDeletion(immutableFromDOM, []);
    
    // Enhanced DFK fact capture telemetry
    const factsRecorded = Object.keys(deletionFactTable).length;
    // Telemetry logging removed for low verbosity
    
    
    return deletionFactTable;
}

// Note: Using gadib_normalize_payload() and gadib_hash() from Base layer

// DFK key hygiene - permissive reader that accepts :vN suffix
function gadie_normalize_dfk_key(dfkKey) {
    // Remove version suffix if present (e.g., :v2, :v3)
    return dfkKey.replace(/:v\d+$/, '');
}

// Tiered DFK matching helper - route => payload => fuzzy
function gadie_match_dfk(op, deletionFactTable, dfkByRoute) {
    const opRouteStr = op.route.length ? op.route.join(',') : 'root';
    
    // 1) exact route - O(1) lookup
    const routeMatches = dfkByRoute.get(opRouteStr);
    if (routeMatches && routeMatches.length > 0) {
        return {k: routeMatches[0].k, e: routeMatches[0].e, why: 'route'};
    }
    
    // 2) same tag+payload hash
    if (op.capturedTextHash) {
        for (const [k, e] of Object.entries(deletionFactTable)) {
            if (e.payloadHash === op.capturedTextHash && e.tag === (op.dfkMetadata?.tag || e.tag)) {
                return {k, e, why: 'payload'};
            }
        }
    }
    
    // 3) fuzzy text (lightweight)
    const text = (op.element?.textContent || '').trim();
    if (text) {
        const norm = s => s.toLowerCase().replace(/\s+/g,' ').slice(0,160);
        const ntext = norm(text);
        let best = null;
        for (const [k, e] of Object.entries(deletionFactTable)) {
            const cand = norm(e.textContent || '');
            const hit = ntext && cand && (cand.includes(ntext) || ntext.includes(cand));
            if (hit) { best = {k, e, why: 'fuzzy'}; break; }
        }
        if (best) return best;
    }
    return null;
}

// DFK Enhancement Method
function gadie_enhance_operations_with_dfk(diffOperations, deletionFactTable) {
    // Preindex DFK by route for O(1) lookups
    const dfkByRoute = new Map();
    Object.entries(deletionFactTable).forEach(([k, e]) => {
        if (!dfkByRoute.has(e.routeStr)) {
            dfkByRoute.set(e.routeStr, []);
        }
        dfkByRoute.get(e.routeStr).push({k, e});
    });
    
    // Enhancement telemetry counters
    let deletionOpsFound = 0;
    let dfkEnhancements = 0;
    let unresolvedOps = 0;
    
    for (const op of diffOperations) {
        if (op.action === 'removeElement' || op.action === 'removeTextElement') {
            deletionOpsFound++;
            
            const match = gadie_match_dfk(op, deletionFactTable, dfkByRoute);
            if (match) {
                const {k: matchedDfkKey, e: matchedEntry} = match;
                op.dfkId = matchedDfkKey;
                op.capturedContent = matchedEntry.outerHTML || matchedEntry.textContent;
                op.capturedType = matchedEntry.nodeType === Node.ELEMENT_NODE ? 'element' : 'text';
                op.dfkMetadata = { 
                    route: matchedEntry.route, 
                    kind: matchedEntry.kind, 
                    tag: matchedEntry.tag, 
                    payloadHash: matchedEntry.payloadHash 
                };
                dfkEnhancements++;
            } else { 
                unresolvedOps++; 
            }
        }
    }
    
    // Enhancement metrics logging removed for low verbosity
}

// Partition operations by resolvability - quarantine instead of dropping
function gadie_partition_by_resolvability(workingDOM, operations) {
    const readOnlyDOM = workingDOM.cloneNode(true);
    const resolved = [], unresolved = [];
    for (const op of operations) {
        (gadie_find_element_by_route(readOnlyDOM, op.route) ? resolved : unresolved).push(op);
    }
    gadib_logger_d(`Pre-apply check: ${unresolved.length} unresolved ops quarantined (not dropped)`);
    return {resolved, unresolved};
}

// Semantic Classification Method  
function gadie_classify_semantic_changes(diffOperations, immutableFromDOM, immutableToDOM) {
    gadib_logger_d('Starting semantic change classification for precise styling');
    
    const classifiedOperations = diffOperations.map(op => {
        // Create a copy of the operation with semantic metadata
        const classifiedOp = { ...op };
        
        // Classify operations into semantic change types
        classifiedOp.semanticType = gadie_determine_semantic_type(op, immutableFromDOM, immutableToDOM);
        classifiedOp.visualTreatment = gadie_get_visual_treatment(classifiedOp.semanticType);
        
        
        return classifiedOp;
    });
    
    // No grouping in deterministic mode - removed for clean workspace
    
    gadib_logger_d(`Semantic classification completed for ${classifiedOperations.length} operations`);
    return classifiedOperations;
}

// Helper: Determine semantic change type for an operation
function gadie_determine_semantic_type(operation, fromDOM, toDOM) {
    const { action, route } = operation;
    
    switch (action) {
        case 'addElement':
        case 'addTextElement':
            return gadie_is_block_level_addition(operation, toDOM) ? 
                'BLOCK_ADDITION' : 'INLINE_ADDITION';
        
        case 'removeElement':
        case 'removeTextElement':
            // Removals are handled by DFK placement, classify for reference
            return gadie_is_block_level_removal(operation, fromDOM) ? 
                'BLOCK_REMOVAL' : 'INLINE_REMOVAL';
        
        case 'relocateNode':
            return 'STRUCTURAL_CHANGE';
        
        case 'modifyTextElement':
            return 'INLINE_MODIFICATION';
        case 'modifyValue':
        case 'modifyAttribute':
            return 'INLINE_MODIFICATION';
        
        default:
            gadib_logger_d(`Unknown operation action: ${action}`);
            return 'INLINE_MODIFICATION';
    }
}


// Helper: Check if addition creates block-level structure
function gadie_is_block_level_addition(operation, toDOM) {
    if (operation.action !== 'addElement') return false;
    
    const blockElements = ['P', 'DIV', 'H1', 'H2', 'H3', 'H4', 'H5', 'H6', 
                         'LI', 'TD', 'TH', 'BLOCKQUOTE', 'PRE', 'SECTION', 
                         'ARTICLE', 'HEADER', 'FOOTER', 'NAV', 'ASIDE', 'MAIN',
                         'DL', 'DT', 'DD', 'UL', 'OL'];
    
    // Primary check: Element tag type
    const elementTag = operation.element?.tagName;
    if (blockElements.includes(elementTag)) {
        gadib_logger_d(`BLOCK-LEVEL-ADDITION: ${elementTag} detected as block element`);
        return true;
    }
    
    // Enhanced check: Element structure analysis
    if (operation.element) {
        // Check if element contains block-level children
        const hasBlockChildren = Array.from(operation.element.children || [])
            .some(child => blockElements.includes(child.tagName));
        
        // Check if element has significant text content (paragraph-like)
        const textContent = operation.element.textContent || '';
        const hasSignificantContent = textContent.length > 50 || textContent.includes('\n');
        
        // Check parent context - if being added to inline context, prefer inline treatment
        const targetElement = gadie_find_element_by_route(toDOM, operation.route.slice(0, -1));
        const inlineParents = ['SPAN', 'A', 'EM', 'STRONG', 'B', 'I', 'CODE'];
        const hasInlineParent = targetElement && inlineParents.includes(targetElement.tagName);
        
        if ((hasBlockChildren || hasSignificantContent) && !hasInlineParent) {
            gadib_logger_d(`BLOCK-LEVEL-ADDITION: ${elementTag} classified as block due to structure (children: ${hasBlockChildren}, content: ${hasSignificantContent})`);
            return true;
        }
    }
    
    gadib_logger_d(`INLINE-ADDITION: ${elementTag} classified as inline element`);
    return false;
}

// Helper: Check if removal was block-level element
function gadie_is_block_level_removal(operation, fromDOM) {
    if (!operation.dfkMetadata) return false;
    
    const blockElements = ['P', 'DIV', 'H1', 'H2', 'H3', 'H4', 'H5', 'H6', 
                         'LI', 'TD', 'TH', 'BLOCKQUOTE', 'PRE', 'SECTION', 
                         'ARTICLE', 'HEADER', 'FOOTER', 'NAV', 'ASIDE', 'MAIN',
                         'DL', 'DT', 'DD', 'UL', 'OL'];
    
    return blockElements.includes(operation.dfkMetadata.tag);
}

// Helper: Get visual treatment for semantic type
function gadie_get_visual_treatment(semanticType) {
    const treatments = {
        'BLOCK_ADDITION': { cssClass: 'gads-addition-block', wrapType: 'block' },
        'INLINE_ADDITION': { cssClass: 'gads-addition-inline', wrapType: 'inline' },
        'BLOCK_REMOVAL': { cssClass: 'gads-deletion-block', wrapType: 'block' },
        'INLINE_REMOVAL': { cssClass: 'gads-deletion-inline', wrapType: 'inline' },
        'STRUCTURAL_CHANGE': { cssClass: 'gads-modification-structural', wrapType: 'block' },
        'INLINE_MODIFICATION': { cssClass: 'gads-modification-inline', wrapType: 'inline' }
    };
    
    return treatments[semanticType] || { cssClass: 'gads-unknown', wrapType: 'inline' };
}


// Generate semantic breakdown for debugging
function gadie_generate_semantic_breakdown(classifiedOperations) {
    const breakdown = {};
    
    classifiedOperations.forEach(op => {
        const type = op.semanticType || 'UNKNOWN';
        breakdown[type] = (breakdown[type] || 0) + 1;
    });
    
    return breakdown;
}

// Assemble Annotated DOM - Phase 6
function gadie_assemble_annotated_dom(detachedWorkingDOM, diffOperations, semanticAnchors, fromCommit, toCommit, enableAnchorFallback) {
    gadib_logger_d('Processing diff operations using true two-phase approach');
    
    // PRIORITY FIX #2: Canonicalize & deduplicate ops pre-apply
    const canonicalOps = gadie_canonicalize_and_deduplicate_operations(diffOperations);
    gadib_logger_d(`Deduplicated ${diffOperations.length - canonicalOps.length} operations`);
    
    // Phase A: Resolve all diff-op routes to stable references (ZERO DOM mutation)
    gadib_logger_d(`Resolving routes for ${canonicalOps.length} operations - READ ONLY`);
    const resolvedOperations = [];
    const preResolutionSnapshot = []; // Fix: Debug snapshot for route resolution
    
    // Fix: Create pristine read-only DOM for route resolution
    const readOnlyDOM = detachedWorkingDOM.cloneNode(true);
    
    for (const op of canonicalOps) {
        const resolvedOp = { ...op }; // Shallow copy operation
        // Fix: Use read-only DOM, never mutate during resolution
        const targetElement = gadie_find_element_by_route(readOnlyDOM, op.route);
        
        
        if (targetElement) {
            // Fix: Store stable reference info, not mutable DOM element
            resolvedOp.resolvedNodeType = targetElement.nodeType;
            resolvedOp.resolvedTagName = targetElement.tagName;
            resolvedOp.resolvedTextContent = targetElement.textContent;
            resolvedOp.resolved = true;
            resolvedOperations.push(resolvedOp);
        } else {
            // This should not happen due to pre-filtering, but log if it does
            gadib_logger_e(`Unexpected unresolved route after pre-filtering: [${op.route.join(',')}] for ${op.action}`);
        }
    }
    
    // Phase B: Apply annotations using resolved references only (separate output DOM)
    gadib_logger_d(`Applying annotations for ${resolvedOperations.filter(op => op.resolved).length} resolved operations`);
    // Fix: Create fresh output DOM for Phase B, never modify the original
    const applyResult = gadie_apply_annotations_from_resolved(detachedWorkingDOM, resolvedOperations, semanticAnchors, enableAnchorFallback);
    const assembledDOM = applyResult.outputDOM;
    
    gadib_logger_d('Two-phase semantic annotation assembly completed');
    return {
        outputDOM: assembledDOM,
        appliedOperations: applyResult.appliedOperations
    };
}

// Canonicalize and deduplicate operations
function gadie_canonicalize_and_deduplicate_operations(operations) {
    gadib_logger_d('Deduplicating operations by (route, action) with element-first precedence');
    const canonicalOps = [];
    const seenKeys = new Set();
    let duplicateCount = 0;
    
    // Sort to prioritize element operations over text operations
    const sortedOps = operations.sort((a, b) => {
        const aIsElement = a.action.includes('Element');
        const bIsElement = b.action.includes('Element');
        if (aIsElement && !bIsElement) return -1;
        if (!aIsElement && bIsElement) return 1;
        return 0;
    });
    
    for (const op of sortedOps) {
        const key = `${op.route.join(',')}:${op.action}`;
        if (!seenKeys.has(key)) {
            canonicalOps.push(op);
            seenKeys.add(key);
        } else {
            duplicateCount++;
            }
    }
    
    gadib_logger_d(`Deduplication complete: ${duplicateCount} duplicates removed`);
    return canonicalOps;
}

// Apply annotations from resolved operations
function gadie_apply_annotations_from_resolved(workingDOM, resolvedOperations, semanticAnchors, enableAnchorFallback) {
    gadib_logger_d('Applying annotations using pre-resolved node references');
    
    // Clone working DOM to avoid mutation during application
    const outputDOM = workingDOM.cloneNode(true);
    
    let insertions = 0, deletions = 0, moves = 0, modifications = 0, errors = 0;
    const processedDeletions = new Set();
    const appliedOperations = [];
    const skippedOperations = [];
    const errorOperations = [];
    
    // Process resolved operations in reverse order
    for (let i = resolvedOperations.length - 1; i >= 0; i--) {
        const op = resolvedOperations[i];
        
        if (!op.resolved) {
            // Insert single, non-intrusive error marker
            gadie_insert_single_error_marker(outputDOM, op);
            errorOperations.push({
                action: op.action,
                route: op.route.join(','),
                reason: op.errorReason
            });
            errors++;
            continue;
        }
        
        // Apply annotation using resolved element reference
        let applied = false;
        switch (op.action) {
            case 'addElement':
            case 'addTextElement':
                gadie_apply_insertion_annotation(outputDOM, op, semanticAnchors, enableAnchorFallback);
                insertions++;
                applied = true;
                break;
            case 'removeElement':
            case 'removeTextElement':
                const routeKey = op.route.join(',');
                if (!processedDeletions.has(routeKey)) {
                    // NOTE: Deletion placement now handled in Phase-6 only - legacy path disabled
                    gadib_logger_d(`Legacy deletion placement bypassed for ${op.action} at route [${op.route.join(',')}]`);
                    processedDeletions.add(routeKey);
                    deletions++;
                    applied = true;
                } else {
                    skippedOperations.push({
                        action: op.action,
                        route: routeKey,
                        reason: 'Duplicate deletion'
                    });
                }
                break;
            case 'relocateNode':
                gadie_apply_move_annotation(outputDOM, op, semanticAnchors, enableAnchorFallback);
                moves++;
                applied = true;
                break;
            case 'modifyTextElement':
                gadie_handle_text_modification(outputDOM, op, {}, semanticAnchors, enableAnchorFallback);
                modifications++;
                applied = true;
                break;
            case 'modifyAttribute':
                gadie_apply_modification_annotation(outputDOM, op, semanticAnchors, enableAnchorFallback);
                modifications++;
                applied = true;
                break;
        }
        
        if (applied) {
            appliedOperations.push({
                action: op.action,
                route: op.route.join(','),
                type: op.action.startsWith('add') ? 'insertion' : 
                      op.action.startsWith('remove') ? 'deletion' :
                      op.action.startsWith('relocate') ? 'move' : 'modification'
            });
        }
    }
    
    gadib_logger_d(`Applied: ${insertions} insertions, ${deletions} deletions, ${moves} moves, ${modifications} modifications, ${errors} errors`);
    
    return {
        outputDOM,
        appliedCount: insertions + deletions + moves + modifications,
        skippedCount: skippedOperations.length,
        errorCount: errors,
        appliedOperations,
        skippedOperations,
        errorOperations
    };
}

// Apply insertion annotation
function gadie_apply_insertion_annotation(outputDOM, operation, semanticAnchors, enableAnchorFallback) {
    let target = gadie_find_element_by_route(outputDOM, operation.route);
    if (!target && enableAnchorFallback) target = gadie_anchor_fallback(operation.route, semanticAnchors);
    
    // Strict semantic classification - no fallback to default classes
    if (!operation.visualTreatment?.cssClass) {
        gadib_logger_e(`SEMANTIC-ERROR: Operation at route [${operation.route.join(',')}] lacks visualTreatment.cssClass - skipping annotation`);
        return;
    }
    
    const cssClass = operation.visualTreatment.cssClass;
    
    if (target && target.nodeType === Node.ELEMENT_NODE) {
        target.classList.add(cssClass);
    } else if (target && target.nodeType === Node.TEXT_NODE && target.parentElement) {
        // For text insertions, create semantic highlighting 
        const parent = target.parentElement;
        const span = document.createElement('span');
        span.classList.add(cssClass);
        span.textContent = target.textContent;
        parent.replaceChild(span, target);
    } else {
        gadib_logger_e(`Failed to apply insertion annotation at route [${operation.route.join(',')}] - element not found or invalid`);
    }
}

// Apply modification annotation
function gadie_apply_modification_annotation(outputDOM, operation, semanticAnchors, enableAnchorFallback) {
    let target = gadie_find_element_by_route(outputDOM, operation.route);
    if (!target && enableAnchorFallback) target = gadie_anchor_fallback(operation.route, semanticAnchors);
    
    // Strict semantic classification - no fallback to default classes
    if (!operation.visualTreatment?.cssClass) {
        gadib_logger_e(`SEMANTIC-ERROR: Operation at route [${operation.route.join(',')}] lacks visualTreatment.cssClass - skipping annotation`);
        return;
    }
    
    const cssClass = operation.visualTreatment.cssClass;
    
    if (target && target.nodeType === Node.ELEMENT_NODE) {
        target.classList.add(cssClass);
    } else {
        gadib_logger_e(`Failed to apply modification annotation at route [${operation.route.join(',')}] - element not found`);
    }
}

// Apply move annotation (placeholder)
function gadie_apply_move_annotation(outputDOM, operation, semanticAnchors, enableAnchorFallback) {
    gadib_logger_d(`Move annotation placeholder for route [${operation.route.join(',')}]`);
    // Move operations are complex and would need full implementation
}

// Handle text modification
function gadie_handle_text_modification(outputDOM, operation, options, semanticAnchors, enableAnchorFallback) {
    let target = gadie_find_element_by_route(outputDOM, operation.route);
    if (!target && enableAnchorFallback) target = gadie_anchor_fallback(operation.route, semanticAnchors);
    
    if (target && target.nodeType === Node.TEXT_NODE) {
        // Apply modification styling to text node
        if (target.parentElement && operation.visualTreatment?.cssClass) {
            const parent = target.parentElement;
            const span = document.createElement('span');
            span.classList.add(operation.visualTreatment.cssClass);
            span.textContent = target.textContent;
            parent.replaceChild(span, target);
            }
    }
}

// Insert single error marker
function gadie_insert_single_error_marker(outputDOM, operation) {
    const marker = document.createElement('span');
    marker.classList.add('gads-error');
    marker.textContent = `[Error: ${operation.action} at ${operation.route.join(',')}]`;
    marker.style.cssText = 'color: red; font-weight: bold; background: #fee;';
    outputDOM.appendChild(marker);
}

// Phase 7: Place Deletion Blocks - DFK-driven placement
async function gadie_place_deletion_blocks(assembledDOM, appliedOperations, deletionFactTable, semanticAnchors, totalDeletions, enableAnchorFallback) {
    gadib_logger_d('DFK-DRIVEN PLACEMENT: Positioning deletion blocks using DFK mappings');
    
    // Clone the assembled DOM to avoid modifying input
    const deletionPlacedDOM = assembledDOM.cloneNode(true);
    
    // Filter to deletion operations from Phase-5 applied list
    const appliedDeletions = appliedOperations.filter(op => 
        op.type === 'deletion' || (op.action && op.action.startsWith('remove'))
    );
    
    // Single-insert invariant: dedupe by (route + action)
    const placedKeys = new Set();
    
    // Enhanced DFK telemetry counters
    let createdBadges = 0;
    let connectedBadges = 0;
    let dfkMatchesfound = 0;
    let dfkMismatches = 0;
    let anchorResolutions = 0;
    let anchorFailures = 0;
    let dfkFallbackAttempts = 0;
    let dfkFallbackSuccess = 0;
    let exactHashMatches = 0;
    let unplacedNoHash = 0;
    let ambiguousMatches = 0;
    let dfkFallbackFailed = 0;
    let inlineRemovalsCreated = 0;
    let blockRemovalsCreated = 0;
    let exactPlacements = 0;
    let fallbackPlacements = 0;
    let ambiguous_examples = [];
    let unplaced_examples = [];
    
    // Report DFK table status
    const dfkCount = Object.keys(deletionFactTable).length;
    gadib_logger_d(`DFK TABLE: ${dfkCount} facts available for placement lookup`);
    gadib_logger_d(`APPLIED DELETIONS: Processing ${appliedDeletions.length} deletion operations`);
    
    for (const appliedOp of appliedDeletions) {
        // GADS-compliant dual route format handling
        // Route can be either Array<number> or string per GADS DFT spec line 637
        const route = Array.isArray(appliedOp.route) 
            ? appliedOp.route 
            : appliedOp.route.split(',').map(Number);
        const routeStr = Array.isArray(appliedOp.route)
            ? appliedOp.route.join(',')
            : appliedOp.route;
        
        // Step 4: Exact match first - Match appliedOp.route to DFT by routeStr and (kind, tag, payloadHash)
        const routeKey = routeStr;
        let dfkKey = null;
        let dfkEntry = null;
        
        // First attempt: exact route match
        for (const [key, entry] of Object.entries(deletionFactTable)) {
            if (entry.routeStr === routeKey) {
                // Additional validation with kind, tag, payloadHash if available in operation
                dfkKey = key;
                dfkEntry = entry;
                break;
            }
        }
        
        // Build de-duplication key using new format: routeStr|action|payloadHash
        let dedupeKey;
        if (dfkKey && dfkEntry) {
            dedupeKey = `${appliedOp.route}|${appliedOp.action}|${dfkEntry.payloadHash}`;
        } else {
            // Fallback for cases where DFK entry not found
            dedupeKey = `${appliedOp.route}|${appliedOp.action}|unknown`;
        }
        
        // Skip if already processed this (route + action + payload) combination
        if (placedKeys.has(dedupeKey)) {
            continue;
        }
        
        if (!dfkKey || !dfkEntry) {
            dfkMismatches++;
            
            // Step 5: Nearest-anchor fallback - Select nearest block/heading anchor and restrict search to that subtree
            dfkFallbackAttempts++;
            const fallbackResult = await gadie_find_dfk_within_anchor(deletionPlacedDOM, route, appliedOp, deletionFactTable);
            
            if (fallbackResult && fallbackResult.type === 'exact_match') {
                dfkFallbackSuccess++;
                exactHashMatches++;
                dfkEntry = fallbackResult.dfkEntry;
                dfkKey = Object.keys(deletionFactTable).find(key => 
                    deletionFactTable[key] === dfkEntry
                );
            } else if (fallbackResult && fallbackResult.type === 'ambiguous') {
                ambiguousMatches++;
                ambiguous_examples.push(dedupeKey);
                // Step 7: >1 matches → ambiguous: do not place any badge
                continue;
            } else if (fallbackResult && fallbackResult.type === 'unplaced') {
                unplacedNoHash++;
                unplaced_examples.push(dedupeKey);
                // Step 7: 0 matches → unplaced: do not place any badge
                continue;
            } else {
                dfkFallbackFailed++;
                unplaced_examples.push(dedupeKey);
                continue;
            }
        }
        
        dfkMatchesfound++;
        
        // Step 8: Child-index path - Compute anchor using Phase-7 nearest anchor logic
        const anchor = gadie_find_stable_semantic_anchor(deletionPlacedDOM, route);
        if (!anchor) {
            anchorFailures++;
            unplaced_examples.push(dedupeKey);
            continue;
        }
        
        anchorResolutions++;
        
        // Check if badge already placed at this location (guard)
        const existingBadge = deletionPlacedDOM.querySelector(`[data-gad-key="${dedupeKey}"]`);
        if (existingBadge) {
            continue;
        }
        
        // Step 11: Create badges only for placed cases
        const badgeResult = gadie_create_deletion_badge(dfkEntry, dedupeKey);
        const deletionBlock = badgeResult.element;
        createdBadges++;
        
        // Track inline vs block removal creation
        if (badgeResult.isInline) {
            inlineRemovalsCreated++;
        } else {
            blockRemovalsCreated++;
        }
        
        // Step 9: Insertion point - Insert badge before child at final index
        const insertionResult = gadie_insert_deletion_badge(anchor, deletionBlock, { op: appliedOp, dftEntry: dfkEntry, placement: 'exact', semanticAnchors, enableAnchorFallback });
        if (insertionResult.success) {
            connectedBadges++;
            placedKeys.add(dedupeKey);
            
            // Step 12: Label badges with data-gad-placement
            if (insertionResult.placementType === 'exact') {
                exactPlacements++;
            } else {
                fallbackPlacements++;
            }
        }
    }
    
    // Enhanced DFK Pipeline Telemetry
    const mismatch = appliedDeletions.length !== connectedBadges;
    gadib_logger_d(`DFK PIPELINE COMPLETED: applied_deletions=${appliedDeletions.length}, dfk_matches=${dfkMatchesfound}, dfk_mismatches=${dfkMismatches}, fallback_attempts=${dfkFallbackAttempts}, fallback_success=${dfkFallbackSuccess}, exact_hash_matches=${exactHashMatches}, unplaced_no_hash=${unplacedNoHash}, ambiguous_matches=${ambiguousMatches}, fallback_failed=${dfkFallbackFailed}, anchor_resolutions=${anchorResolutions}, anchor_failures=${anchorFailures}, created_badges=${createdBadges}, inline_removals=${inlineRemovalsCreated}, block_removals=${blockRemovalsCreated}, connected_badges=${connectedBadges}${mismatch ? ' PLACEMENT MISMATCH!' : ''}`);
    
    // DFK telemetry logging removed for low verbosity
    
    // Build Phase 7 telemetry per spec
    const phase7Telemetry = {
        exact: exactPlacements,
        fallback: fallbackPlacements,
        ambiguous: ambiguousMatches,
        unplaced: unplacedNoHash + dfkFallbackFailed,
        ambiguous_examples: ambiguous_examples,
        unplaced_examples: unplaced_examples,
        totalProcessed: appliedDeletions.length,
        pipeline_stats: {
            dfk_matches: dfkMatchesfound,
            dfk_mismatches: dfkMismatches,
            fallback_attempts: dfkFallbackAttempts,
            fallback_success: dfkFallbackSuccess,
            exact_hash_matches: exactHashMatches,
            anchor_resolutions: anchorResolutions,
            anchor_failures: anchorFailures,
            created_badges: createdBadges,
            connected_badges: connectedBadges,
            inline_removals: inlineRemovalsCreated,
            block_removals: blockRemovalsCreated
        }
    };
    
    return {
        dom: deletionPlacedDOM,
        telemetry: phase7Telemetry
    };
}

// Helper: Find DFK within semantic anchor (Real Anchor-Bounded Fallback)
async function gadie_find_dfk_within_anchor(dom, route, operation, deletionFactTable) {
    gadib_logger_d(`Attempting DFK fallback for route [${route.join(',')}] using ABF`);
    
    // Step 5: Nearest-anchor fallback - select the nearest block/heading anchor
    const anchor = gadie_find_stable_semantic_anchor(dom, route);
    if (!anchor) {
        return { type: 'unplaced', anchor: dom };
    }
    
    // Build needle criteria from operation context
    // Extract what we can from the operation to match against DFK entries
    const needle = {
        kind: operation.action === 'removeElement' ? '#element' : '#text',
        tag: operation.action === 'removeElement' ? 'UNKNOWN' : '#text', // Limited info from operation
        payloadHash: null // Not available from operation context
    };
    
    // If we have dfkMetadata from earlier enhancement, use it
    if (operation.dfkMetadata) {
        needle.kind = operation.dfkMetadata.kind;
        needle.tag = operation.dfkMetadata.tag;
        needle.payloadHash = operation.dfkMetadata.payloadHash;
        needle.normalizedPayload = operation.dfkMetadata.normalizedPayload;
    }
    
    // If we still don't have payload hash, we can't do proper matching
    if (!needle.payloadHash) {
        gadib_logger_d(`ABF: No payload hash available for matching - marking as unplaced`);
        return { type: 'unplaced', anchor: anchor };
    }
    
    // Step 6: DFT matching in ABF - within the subtree, match by (kind, tag, payloadHash)
    const result = gadie_find_dfk_within_anchor_element(anchor, needle, deletionFactTable);
    
    if (!result) {
        return { type: 'unplaced', anchor: anchor };
    }
    
    if (result.ambiguous) {
        return { 
            type: 'ambiguous', 
            anchor: anchor,
            candidates: result.candidates
        };
    }
    
    // Single match found
    return {
        type: 'exact_match',
        anchor: anchor,
        dfkEntry: result
    };
}

// Helper: ABF Core Implementation - Find DFK within anchor element
function gadie_find_dfk_within_anchor_element(anchorEl, needle, deletionFactTable) {
    // Get anchor route for containment checking
    const anchorRoute = gadie_get_anchor_route(anchorEl);
    
    // Build flat list of DFT candidates within anchor subtree
    const candidatesMap = new Map();
    
    // Pre-compute candidates by scanning DFT for entries within this anchor
    for (const [dfkKey, dfkEntry] of Object.entries(deletionFactTable)) {
        // Check if this DFT entry is within the anchor subtree
        // An entry is within the anchor if the anchor route is a prefix of the entry route
        if (!gadie_is_route_within_anchor(dfkEntry.route, anchorRoute)) {
            continue; // Skip entries not within this anchor
        }
        
        // Step 6: Match by (kind, tag, payloadHash)
        const candidateKey = `${dfkEntry.kind}|${dfkEntry.tag}|${dfkEntry.payloadHash}`;
        
        if (!candidatesMap.has(candidateKey)) {
            candidatesMap.set(candidateKey, []);
        }
        candidatesMap.get(candidateKey).push({ dfkKey, dfkEntry });
    }
    
    // Look up candidates by exact triple
    const needleKey = `${needle.kind}|${needle.tag}|${needle.payloadHash}`;
    const candidates = candidatesMap.get(needleKey) || [];
    
    if (candidates.length === 0) {
        return null; // 0 matches → unplaced
    }
    
    if (candidates.length === 1) {
        // Single match → place
        return candidates[0].dfkEntry;
    }
    
    // Multiple candidates - tie-break via normalizedPayload
    if (needle.normalizedPayload) {
        const exactMatches = candidates.filter(c => 
            c.dfkEntry.normalizedPayload === needle.normalizedPayload
        );
        
        if (exactMatches.length === 1) {
            return exactMatches[0].dfkEntry;
        }
    }
    
    // >1 matches → ambiguous: do not place
    return {
        ambiguous: true,
        candidates: candidates.map(c => c.dfkKey)
    };
}

// Helper: Find stable semantic anchor for deletion placement using Phase-7 logic
function gadie_find_stable_semantic_anchor(dom, route) {
    // Step 5: Select the nearest block/heading anchor with fallback hierarchy:
    // 1. Prefer id anchors 
    // 2. Then heading anchors
    // 3. Then block tags
    
    // Start from the route and work upward
    for (let len = route.length - 1; len >= 0; len--) {
        const testRoute = route.slice(0, len);
        const element = gadie_find_element_by_route(dom, testRoute);
        
        if (element && element.nodeType === Node.ELEMENT_NODE) {
            const tagName = element.tagName.toLowerCase();
            
            // Prefer id anchors (highest priority)
            if (element.id) {
                return element;
            }
            
            // Then heading anchors (second priority)
            if (['h1', 'h2', 'h3', 'h4', 'h5', 'h6'].includes(tagName)) {
                return element;
            }
            
            // Then block tags (third priority)
            if (['p', 'div', 'section', 'article', 'dd', 'li', 'table', 'thead', 'tbody', 'tr'].includes(tagName)) {
                return element;
            }
        }
    }
    
    // Final fallback to root
    return dom;
}

// Helper: Create deletion badge with DFK metadata (Step 11: Create badges only for placed cases)
function gadie_create_deletion_badge(dfkEntry, dedupeKey) {
    const isInline = dfkEntry.kind === '#text';
    
    const element = document.createElement(isInline ? 'span' : 'div');
    element.classList.add(isInline ? 'gads-deletion-inline' : 'gads-deletion-block');
    element.setAttribute('data-gad-key', dedupeKey);
    element.setAttribute('data-dfk-kind', dfkEntry.kind);
    element.setAttribute('data-dfk-tag', dfkEntry.tag);
    
    // Step 13: De-duplication key format (routeStr|action|payloadHash)
    // The dedupeKey is already in the correct format from the caller
    
    // Set content based on what was captured  
    if (dfkEntry.outerHTML) {
        // Don't wrap in <del> - CSS provides strikethrough styling
        element.innerHTML = dfkEntry.outerHTML;
    } else {
        element.textContent = dfkEntry.textContent || '[Deleted content]';
    }
    
    
    return { element, isInline };
}

// Helper: Create unplaced deletion badge for fallback cases
function gadie_create_unplaced_deletion_badge(operation, anchor) {
    const badge = document.createElement('span');
    badge.classList.add('gads-deletion-unplaced');
    badge.textContent = `[Unplaced deletion: ${operation.action} at ${operation.route}]`;
    badge.style.cssText = 'background: #ffa; border: 1px solid #cc6; padding: 2px; margin: 2px;';
    
    if (anchor && anchor.appendChild) {
        anchor.appendChild(badge);
        return true;
    }
    return false;
}

// Helper: Insert deletion badge at appropriate location with exact placement
function gadie_insert_deletion_badge(anchor, badge, options = {}) {
    if (!anchor || !badge) return { success: false };
    
    const { op, dftEntry, placement = 'exact', semanticAnchors } = options;
    let placementType = 'fallback'; // Default to fallback
    
    try {
        if (options.enableAnchorFallback && op && dftEntry && placement === 'exact') {
            // Step 8: Child-index path - compute relative path from anchor to deletion route
            const placementResult = gadie_compute_exact_placement(anchor, op, dftEntry, semanticAnchors);
            
            if (placementResult.success) {
                // Step 9: Insertion point - insert badge before child at final index
                const targetContainer = placementResult.container;
                const targetIndex = placementResult.index;
                
                if (targetContainer.childNodes && targetIndex <= targetContainer.childNodes.length) {
                    // Step 10: Inline token fidelity - insert at precise token index for inline deletions
                    if (targetIndex < targetContainer.childNodes.length) {
                        targetContainer.insertBefore(badge, targetContainer.childNodes[targetIndex]);
                        badge.setAttribute('data-gad-placement', 'exact');
                        placementType = 'exact';
                    } else {
                        // Index ≥ container length, append and classify as exact
                        targetContainer.appendChild(badge);
                        badge.setAttribute('data-gad-placement', 'exact');
                        placementType = 'exact';
                    }
                    return { success: true, placementType };
                } else {
                    // Container traversal failed - fallback placement
                    badge.setAttribute('data-gad-placement', 'fallback');
                    return { success: false, placementType: 'fallback' };
                }
            }
            
            // Fall through to fallback placement
            badge.setAttribute('data-gad-placement', 'fallback');
        } else {
            badge.setAttribute('data-gad-placement', placement || 'fallback');
        }
        
        // Original placement logic as fallback
        if (anchor.firstChild) {
            anchor.insertBefore(badge, anchor.firstChild);
        } else {
            anchor.appendChild(badge);
        }
        return { success: true, placementType };
    } catch (error) {
        gadib_logger_e(`Failed to insert deletion badge: ${error.message}`);
        badge.setAttribute('data-gad-placement', 'fallback');
        return { success: false, placementType };
    }
}

// Helper: Compute exact placement using relative index path
function gadie_compute_exact_placement(anchor, op, dftEntry, semanticAnchors) {
    try {
        // Get the route from the DFT entry (original route in "from" DOM)
        const originalRoute = dftEntry.route;
        
        // Find the nearest block anchor in the semantic anchors for the route
        // Use Phase-2 semanticAnchors with fallback hierarchy preference
        const anchorInfo = gadie_find_nearest_block_anchor(originalRoute, semanticAnchors);
        if (!anchorInfo) {
            return { success: false, reason: 'could not find semantic anchor for route' };
        }
        
        // Verify this matches the passed anchor or use the found anchor
        const effectiveAnchor = anchorInfo.element;
        const anchorRoute = anchorInfo.route;
        
        // Compute relative path from anchor to deleted node
        const relativePath = gadie_compute_relative_path(anchorRoute, originalRoute);
        if (!relativePath) {
            return { success: false, reason: 'could not compute relative path' };
        }
        
        // Traverse the relative path in the target DOM to find insertion point
        // Use the effective anchor from semantic anchors for more accurate placement
        const traversalResult = gadie_traverse_relative_path(effectiveAnchor, relativePath);
        
        return {
            success: traversalResult.success,
            container: traversalResult.container,
            index: traversalResult.index,
            reason: traversalResult.reason
        };
        
    } catch (error) {
        gadib_logger_e(`Exact placement computation failed: ${error.message}`);
        return { success: false, reason: error.message };
    }
}

// Helper: Find nearest block anchor using Phase-2 semantic anchors
function gadie_find_nearest_block_anchor(route, semanticAnchors) {
    if (!semanticAnchors || !route) return null;
    
    // Try to find anchors at progressively higher levels in the route
    // This implements the fallback hierarchy from the spec
    for (let len = route.length; len >= 0; len--) {
        const testRoute = route.slice(0, len);
        const routeStr = len === 0 ? "root" : testRoute.join(',');
        
        if (semanticAnchors[routeStr]) {
            const anchorInfo = semanticAnchors[routeStr];
            
            // Prefer ID/fragment-based anchor (if present)
            if (anchorInfo.element.id) {
                return anchorInfo;
            }
            
            // Prefer heading-mapped anchor (h1-h6)
            const tag = anchorInfo.tag;
            if (['h1', 'h2', 'h3', 'h4', 'h5', 'h6'].includes(tag)) {
                return anchorInfo;
            }
            
            // Fall back to nearest block anchor by tag set
            if (['p', 'div', 'section', 'article', 'dd', 'li', 'table', 'thead', 'tbody', 'tr'].includes(tag)) {
                return anchorInfo;
            }
            
            // If none resolvable at this level, continue to parent
        }
    }
    
    return null; // Classify as unplaced per spec
}

// Helper: Check if a route is within an anchor's subtree
function gadie_is_route_within_anchor(entryRoute, anchorRoute) {
    // An entry route is within anchor if anchor route is a prefix
    if (anchorRoute.length > entryRoute.length) {
        return false;
    }
    
    for (let i = 0; i < anchorRoute.length; i++) {
        if (anchorRoute[i] !== entryRoute[i]) {
            return false;
        }
    }
    
    return true;
}

// Helper: Get the route of an anchor element by traversing up
function gadie_get_anchor_route(anchor) {
    // This is a simplified implementation that works with semantic anchors
    // In practice, we'd compute this from the DOM structure, but for ABF
    // we can use a simplified approach since we're working within bounds
    
    // For now, return empty route to represent root positioning
    // This means all DFK entries will be considered "within" the anchor
    // which provides the broad search needed for ABF
    return [];
}

// Helper: Compute relative path between two routes
function gadie_compute_relative_path(anchorRoute, targetRoute) {
    // If targetRoute is longer than anchorRoute and starts with anchorRoute,
    // return the difference as relative path
    if (targetRoute.length <= anchorRoute.length) {
        return null;
    }
    
    // Check if anchorRoute is a prefix of targetRoute
    for (let i = 0; i < anchorRoute.length; i++) {
        if (anchorRoute[i] !== targetRoute[i]) {
            return null;
        }
    }
    
    // Return the relative portion
    return targetRoute.slice(anchorRoute.length);
}

// Helper: Traverse relative path to find insertion point
function gadie_traverse_relative_path(anchor, relativePath) {
    let currentContainer = anchor;
    
    // Traverse all but the last index in the path
    for (let i = 0; i < relativePath.length - 1; i++) {
        const childIndex = relativePath[i];
        
        if (!currentContainer.childNodes || childIndex >= currentContainer.childNodes.length) {
            return { 
                success: false, 
                reason: `child index ${childIndex} out of range at path position ${i}` 
            };
        }
        
        currentContainer = currentContainer.childNodes[childIndex];
    }
    
    // The last index in the path is where we want to insert
    const insertionIndex = relativePath[relativePath.length - 1];
    
    return {
        success: true,
        container: currentContainer,
        index: insertionIndex
    };
}

// Phase 8: Merge Adjacent Same Nature - Visual coalescing
function gadie_merge_adjacent_same_nature(annotatedDOM) {
    // Step 15: Phase-8 guard - ensure classing does not create or move badges
    gadib_logger_d('Starting Phase 8: Visual coalescing with presentation wrappers - NO badge relocation');
    
    // Create a new DOM to avoid modifying the annotated DOM
    const coalescedDOM = annotatedDOM.cloneNode(true);
    
    // Count existing badges before coalescing
    const badgesBeforeCount = coalescedDOM.querySelectorAll('[data-gad-placement]').length;
    
    // Apply visual coalescing by adding presentation wrappers around adjacent same-nature elements
    // This only coalesces adjacent runs of same nature, never creates or moves badges
    gadie_add_visual_run_wrappers(coalescedDOM);
    
    // Verify no badges were created or moved during coalescing
    const badgesAfterCount = coalescedDOM.querySelectorAll('[data-gad-placement]').length;
    if (badgesAfterCount !== badgesBeforeCount) {
        gadib_logger_e(`PHASE-8-GUARD-VIOLATION: Badge count changed from ${badgesBeforeCount} to ${badgesAfterCount} during coalescing`);
    }
    
    gadib_logger_d('Phase 8 complete - semantic structure fully preserved, no badge relocation');
    return coalescedDOM;
}

function gadie_add_visual_run_wrappers(dom) {
    // Find adjacent same-nature elements using GADS semantic classes
    const diffTypes = ['gads-addition-inline', 'gads-modification-structural', 'gads-deletion-inline', 'gads-modification-inline'];
    let runCounter = 0;
    
    // Initialize coalescing telemetry
    let elementsProcessed = 0;
    let elementsMerged = 0;
    let runsSkipped = 0;
    
    for (const diffType of diffTypes) {
        const runResult = gadie_wrap_adjacent_same_type(dom, diffType, runCounter);
        runCounter += runResult.runsCreated;
        elementsProcessed += runResult.elementsProcessed;
        elementsMerged += runResult.elementsMerged;
        runsSkipped += runResult.runsSkipped;
    }
    
    // Report final telemetry
    gadib_logger_d(`Coalescing complete: processed=${elementsProcessed}, merged=${elementsMerged}, runs_skipped=${runsSkipped}, runs_created=${runCounter}`);
}

function gadie_wrap_adjacent_same_type(dom, className, startRunId) {
    const elements = Array.from(dom.querySelectorAll(`.${className}`));
    if (elements.length < 2) {
        return { runsCreated: 0, elementsProcessed: elements.length, elementsMerged: 0, runsSkipped: 0 };
    }
    
    let runId = startRunId;
    let runsCreated = 0;
    let elementsMerged = 0;
    let runsSkipped = 0;
    let i = 0;
    
    while (i < elements.length) {
        const currentElement = elements[i];
        const adjacentRun = [currentElement];
        
        // Collect adjacent siblings of the same type
        let j = i + 1;
        while (j < elements.length && gadie_are_consecutive_siblings(elements[j-1], elements[j])) {
            adjacentRun.push(elements[j]);
            j++;
        }
        
        // Check coalescing strategy
        if (adjacentRun.length >= 2) {
            const coalescingResult = gadie_can_safely_coalesce(adjacentRun, className);
            
            if (coalescingResult === true) {
                // Direct merge is safe
                gadie_replace_run_with_single_element(adjacentRun, className);
                runsCreated++;
                elementsMerged += adjacentRun.length;
            } else if (coalescingResult && coalescingResult.useWrapper) {
                // Use wrapper container strategy
                gadie_create_wrapper_container(adjacentRun, className);
                runsCreated++;
                elementsMerged += adjacentRun.length;
            } else {
                runsSkipped++;
            }
        }
        
        i = j;
    }
    
    return { runsCreated, elementsProcessed: elements.length, elementsMerged, runsSkipped };
}

// Helper: Check if elements are consecutive siblings
function gadie_are_consecutive_siblings(elem1, elem2) {
    if (!elem1 || !elem2 || elem1.parentElement !== elem2.parentElement) {
        return false;
    }
    
    const parent = elem1.parentElement;
    const children = Array.from(parent.children);
    const index1 = children.indexOf(elem1);
    const index2 = children.indexOf(elem2);
    
    return Math.abs(index1 - index2) === 1;
}

// Helper: Check if elements can be safely coalesced
function gadie_can_safely_coalesce(elements, className) {
    // GADS Anchor Coalescing Rules - implement wrapper strategy
    if (elements.length < 2) {
        return false;
    }
    
    // Check for anchor elements - use wrapper strategy instead of blocking
    let hasAnchors = false;
    for (const element of elements) {
        if (element.tagName === 'A') {
            hasAnchors = true;
            break;
        }
        // Also check for nested anchors
        if (element.querySelector('a')) {
            hasAnchors = true;
            break;
        }
    }
    
    if (hasAnchors) {
        // Mark for wrapper strategy instead of blocking
        return { useWrapper: true };
    }
    
    const firstTag = elements[0].tagName;
    
    // Do-Not-Merge Constraints for Interactive Elements
    for (const element of elements) {
        // Never merge interactive elements
        const interactiveTags = ['BUTTON', 'INPUT', 'SELECT', 'TEXTAREA'];
        if (interactiveTags.includes(element.tagName)) {
            return false;
        }
        
        // All elements must have the same tag for direct merging
        if (element.tagName !== firstTag) {
            return { useWrapper: true };
        }
    }
    
    return true;
}

// Helper: Replace run with single coalesced element
function gadie_replace_run_with_single_element(adjacentRun, className) {
    if (adjacentRun.length < 2) return;
    
    const parent = adjacentRun[0].parentElement;
    const firstElement = adjacentRun[0];
    
    // Create a wrapper element
    const wrapper = document.createElement('span');
    wrapper.classList.add(className + '-run');
    wrapper.setAttribute('data-coalesced-count', adjacentRun.length.toString());
    
    // Move all content into the wrapper
    adjacentRun.forEach(element => {
        const content = element.cloneNode(true);
        wrapper.appendChild(content);
    });
    
    // Replace the first element with the wrapper and remove the rest
    parent.replaceChild(wrapper, firstElement);
    for (let i = 1; i < adjacentRun.length; i++) {
        if (adjacentRun[i].parentElement) {
            adjacentRun[i].parentElement.removeChild(adjacentRun[i]);
        }
    }
}

// Helper: Create wrapper container for preserving individual semantics while providing visual consolidation
function gadie_create_wrapper_container(adjacentRun, className) {
    if (adjacentRun.length < 2) return;
    
    const parent = adjacentRun[0].parentElement;
    const firstElement = adjacentRun[0];
    
    // Create visual container box around consecutive elements
    const wrapper = document.createElement('span');
    wrapper.classList.add('gads-consolidated-run'); // Visual container class
    wrapper.classList.add(className + '-container'); // Specific styling class
    wrapper.setAttribute('data-wrapped-class', className);
    wrapper.setAttribute('data-wrapped-count', adjacentRun.length.toString());
    wrapper.setAttribute('data-strategy', 'visual-consolidation');
    
    // Add inline styling for immediate visual consolidation
    wrapper.style.cssText = `
        display: inline-block;
        background: #d4edda;
        border: 1px solid #c3e6cb;
        border-radius: 4px;
        padding: 2px 4px;
        margin: 1px;
    `;
    
    // Move individual elements into wrapper, removing their individual styling
    adjacentRun.forEach((element, index) => {
        // Clone element and remove individual diff styling since wrapper provides consolidated styling
        const preservedElement = element.cloneNode(true);
        preservedElement.classList.remove(className); // Remove individual green box styling
        wrapper.appendChild(preservedElement);
        
        // Add whitespace between elements (preserve original spacing)
        if (index < adjacentRun.length - 1) {
            const nextSibling = element.nextSibling;
            if (nextSibling && nextSibling.nodeType === Node.TEXT_NODE) {
                wrapper.appendChild(nextSibling.cloneNode(true));
            }
        }
    });
    
    // Replace the first element with the wrapper and remove the rest
    parent.replaceChild(wrapper, firstElement);
    for (let i = 1; i < adjacentRun.length; i++) {
        if (adjacentRun[i].parentElement) {
            adjacentRun[i].parentElement.removeChild(adjacentRun[i]);
        }
    }
    
}

// Phase 9: Serialize Final Output
function gadie_serialize_final_output(coalescedDOM) {
    gadib_logger_d('Generating final styled diff HTML ready for display');
    
    // Final validation and cleanup
    const finalHTML = coalescedDOM.innerHTML;
    
    // Validate output integrity
    const parser = new DOMParser();
    const testDoc = parser.parseFromString(finalHTML, 'text/html');
    if (testDoc.querySelector('parsererror')) {
        gadib_logger_e('Final HTML contains parser errors');
    } else {
        gadib_logger_d('Final HTML validated successfully');
    }
    
    return finalHTML;
}

// Smoke tests for DFK matching and functionality
function gadie_run_smoke_tests() {
    console.log('[GADIE-SMOKE-TESTS] Starting smoke tests...');
    
    // Test (a): exact-route removal
    try {
        const testOp1 = { route: [1, 2, 3], action: 'removeElement' };
        const testDft1 = {
            'dfk:1,2,3|#element|DIV|sha256:abcd1234567890abcd1234567890abcd1234567890abcd1234567890abcd1234': {
                routeStr: '1,2,3',
                kind: '#element',
                tag: 'DIV',
                payloadHash: 'sha256:abcd1234567890abcd1234567890abcd1234567890abcd1234567890abcd1234'
            }
        };
        const match1 = gadie_match_dfk(testOp1, testDft1);
        console.log(`[SMOKE-TEST-A] Exact route match: ${match1 ? 'PASS' : 'FAIL'} - ${match1?.why || 'no match'}`);
    } catch (e) {
        console.log(`[SMOKE-TEST-A] ERROR: ${e.message}`);
    }
    
    // Test (b): minor-normalization change (payload match)
    try {
        const testOp2 = { route: [1, 2, 4], action: 'removeElement', capturedTextHash: 'sha256:xyz789' };
        const testDft2 = {
            'dfk:1,2,3|#element|DIV|sha256:xyz789': {
                routeStr: '1,2,3',
                kind: '#element', 
                tag: 'DIV',
                payloadHash: 'sha256:xyz789'
            }
        };
        const match2 = gadie_match_dfk(testOp2, testDft2);
        console.log(`[SMOKE-TEST-B] Payload hash match: ${match2 ? 'PASS' : 'FAIL'} - ${match2?.why || 'no match'}`);
    } catch (e) {
        console.log(`[SMOKE-TEST-B] ERROR: ${e.message}`);
    }
    
    // Test (c): fuzzy text match
    try {
        const testOp3 = { 
            route: [2, 1, 0], 
            action: 'removeElement',
            element: { textContent: 'Hello World Test' }
        };
        const testDft3 = {
            'dfk:1,2,3|#element|P|sha256:fuzzy123': {
                routeStr: '1,2,3',
                kind: '#element',
                tag: 'P', 
                payloadHash: 'sha256:fuzzy123',
                textContent: 'Hello World Test Content'
            }
        };
        const match3 = gadie_match_dfk(testOp3, testDft3);
        console.log(`[SMOKE-TEST-C] Fuzzy text match: ${match3 ? 'PASS' : 'FAIL'} - ${match3?.why || 'no match'}`);
    } catch (e) {
        console.log(`[SMOKE-TEST-C] ERROR: ${e.message}`);
    }
    
    console.log('[GADIE-SMOKE-TESTS] Smoke tests completed.');
}

// Uncomment to run smoke tests during development:
// gadie_run_smoke_tests();