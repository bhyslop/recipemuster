// GADIE: GAD Inspector Diff Engine
// Extracted from monolithic gadi_inspector.html for modular architecture
// Contains: All 9-phase diff methods, DOM helpers, DFK operations, coalescing logic

// Main diff processing function - exposed as the primary interface
async function gadie_diff(fromHtml, toHtml, opts = {}) {
    const { fromCommit, toCommit, sourceFiles } = opts;
    
    const startTime = performance.now();
    gadib_logger_d('Starting GADS-compliant 9-phase diff processing');
    gadib_logger_d(`Input sizes: fromHtml=${fromHtml.length} chars, toHtml=${toHtml.length} chars`);
    gadib_logger_d(`diff-dom type at diff time: ${typeof window.diffDom}`);

    if (typeof window.diffDom === 'undefined' || typeof window.diffDom.DiffDOM === 'undefined') {
        throw new Error('diff-dom library not available - required for diff processing');
    }

    try {
        // Phase 1: Immutable Input - Establish immutable source DOM trees
        gadib_logger_p(1, 'Immutable Input: Creating immutable source DOM trees');
        const immutableFromDOM = gadie_create_dom_from_html(fromHtml);
        const immutableToDOM = gadie_create_dom_from_html(toHtml);
        gadib_logger_p(1, 'Immutable Input: Immutable DOM trees established and preserved throughout processing');

        // Phase 2: Detached Working - Create Detached Working DOM with Semantic Anchors
        gadib_logger_p(2, 'Detached Working: Creating Detached Working DOM with Semantic Anchors');
        const detachedWorkingDOM = gadie_create_detached_working_dom(immutableToDOM);
        const semanticAnchors = gadie_establish_semantic_anchors(detachedWorkingDOM);
        gadib_logger_p(2, `Detached Working: Created independent processing environment with ${Object.keys(semanticAnchors).length} semantic anchors`);

        // Phase 3: Deletion Fact Capture - Generate Deletion Fact Keys (DFK)
        gadib_logger_p(3, 'Deletion Fact Capture: Generating Deletion Fact Keys (DFK)');
        const deletionFactTable = await gadie_create_deletion_fact_table(immutableFromDOM);
        const phase3Debug = { deletionFactCount: Object.keys(deletionFactTable).length, sampleKeys: Object.keys(deletionFactTable).slice(0, 5) };
        gadib_factory_ship('phase3_dft', JSON.stringify(phase3Debug, null, 2), fromCommit, toCommit, sourceFiles);
        gadib_logger_p(3, `Deletion Fact Capture: Created DFK mapping for ${Object.keys(deletionFactTable).length} potential deletions`);

        // Phase 4: Diff Operations - Generate structured diff operations
        gadib_logger_p(4, 'Diff Operations: Initializing diff-dom and generating operations');
        const diffDOM = new window.diffDom.DiffDOM({
            debug: true,
            diffcap: 500  // Allow more granular detection
        });
        const diffOperations = diffDOM.diff(immutableFromDOM, immutableToDOM);
        gadie_enhance_operations_with_dfk(diffOperations, deletionFactTable);
        gadib_logger_p(4, `Diff Operations: Generated ${diffOperations.length} enhanced diff operations`);

        // Phase 5: Semantic Classification - Classify operations into semantic change types
        gadib_logger_p(5, 'Semantic Classification: Analyzing operations for precise granular styling');
        const classifiedOperations = gadie_classify_semantic_changes(diffOperations, immutableFromDOM, immutableToDOM);
        
        // Phase 5 Debug Output - Show enriched operations with semantic metadata
        const phase45Debug = {
            totalOperations: classifiedOperations.length,
            semanticBreakdown: gadie_generate_semantic_breakdown(classifiedOperations),
            sampleOperations: classifiedOperations.slice(0, 5).map(op => ({
                action: op.action,
                route: op.route,
                semanticType: op.semanticType,
                visualTreatment: op.visualTreatment
            }))
        };
        gadib_factory_ship('phase5_classified', JSON.stringify(phase45Debug, null, 2), fromCommit, toCommit, sourceFiles);
        gadib_logger_p(5, `Semantic Classification: Classified ${classifiedOperations.length} operations with semantic metadata`);

        // Phase 6: Annotated Assembly - Construct semantically annotated output DOM
        gadib_logger_p(6, 'Annotated Assembly: Processing classified operations in Detached Working environment');
        const assemblyResult = gadie_assemble_annotated_dom(detachedWorkingDOM, classifiedOperations, semanticAnchors, fromCommit, toCommit);
        const assembledDOM = assemblyResult.outputDOM;
        const assembledHTML = assembledDOM.innerHTML;
        gadib_factory_ship('phase6_annotated', assembledHTML, fromCommit, toCommit, sourceFiles);
        gadib_logger_p(6, 'Annotated Assembly: Semantic annotation construction completed');

        // Phase 7: Deletion Placement - Position deletion blocks using DFK and Semantic Anchors
        gadib_logger_p(7, 'Deletion Placement: Positioning deletion blocks with DFK mappings');
        const deletionPlacedDOM = await gadie_place_deletion_blocks(assembledDOM, assemblyResult.appliedOperations, deletionFactTable, semanticAnchors);
        const deletionPlacedHTML = deletionPlacedDOM.innerHTML;
        gadib_factory_ship('phase7_deletions', deletionPlacedHTML, fromCommit, toCommit, sourceFiles);
        gadib_logger_p(7, 'Deletion Placement: Structured deletion blocks positioned accurately');

        // Phase 8: Uniform Classing - Merge consecutive same-nature elements
        gadib_logger_p(8, 'Uniform Classing: Merging consecutive same-nature elements into consolidated runs');
        const coalescedDOM = gadie_merge_adjacent_same_nature(deletionPlacedDOM);
        const coalescedHTML = coalescedDOM.innerHTML;
        gadib_factory_ship('phase8_coalesced', coalescedHTML, fromCommit, toCommit, sourceFiles);
        gadib_logger_p(8, 'Uniform Classing: Single-element consolidated runs created');

        // Phase 9: Serialize - Generate final rendered output
        gadib_logger_p(9, 'Serialize: Generating final styled diff HTML');
        const finalHTML = gadie_serialize_final_output(coalescedDOM);
        gadib_factory_ship('phase9_final', finalHTML, fromCommit, toCommit, sourceFiles);
        gadib_logger_p(9, 'Serialize: Final rendered HTML ready for display');

        // Performance logging for Factory debugging
        const endTime = performance.now();
        const totalTime = Math.round(endTime - startTime);
        gadib_logger_d(`GADS 9-phase diff processing completed in ${totalTime}ms`);
        gadib_logger_d('Complete processing isolation maintained - input DOMs never modified');

        return finalHTML;

    } catch (error) {
        gadib_logger_e(`GADS 9-phase diff processing failed: ${error.message}`);
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
    const workingDOM = document.createElement('div');
    gadie_deep_clone_element(immutableToDOM, workingDOM);
    gadib_logger_d('Created detached working DOM environment for semantic processing');
    return workingDOM;
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
                anchors[routeStr] = {
                    element: element,
                    route: route.slice(),
                    tag: tagName,
                    semanticType: gadie_get_semantic_type_from_tag(tagName)
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

// Get semantic type from HTML tag
function gadie_get_semantic_type_from_tag(tagName) {
    const headingTags = ['h1', 'h2', 'h3', 'h4', 'h5', 'h6'];
    if (headingTags.includes(tagName)) return 'heading';
    if (tagName === 'p') return 'paragraph';
    if (['div', 'section', 'article'].includes(tagName)) return 'container';
    return 'generic';
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
            
            // Fix: Remove "best-effort" fallback - if route index is OOB, abort operation
            gadib_logger_e(`Route traversal failed at index ${index} for route [${route.join(',')}]`);
            gadib_logger_e(`Parent has ${availableCount} children, requested index ${index}`);
            gadib_logger_e(`Failed at route position ${i}, partial route was [${route.slice(0, i).join(',')}]`);
            return null; // Fix: Fail fast instead of retargeting
        }
    }
    return element;
}

function gadie_deep_clone_element(sourceElement, targetParent) {
    if (!sourceElement || !sourceElement.childNodes) return;

    // Clone all child nodes recursively
    for (let i = 0; i < sourceElement.childNodes.length; i++) {
        const sourceChild = sourceElement.childNodes[i];
        let clonedChild;

        if (sourceChild.nodeType === Node.ELEMENT_NODE) {
            // Create element with all attributes
            clonedChild = document.createElement(sourceChild.tagName.toLowerCase());
            for (let j = 0; j < sourceChild.attributes.length; j++) {
                const attr = sourceChild.attributes[j];
                clonedChild.setAttribute(attr.name, attr.value);
            }
            // Recursively clone children
            gadie_deep_clone_element(sourceChild, clonedChild);
        } else if (sourceChild.nodeType === Node.TEXT_NODE) {
            clonedChild = document.createTextNode(sourceChild.textContent);
        } else {
            // Other node types (comments, etc.)
            clonedChild = sourceChild.cloneNode(false);
        }

        if (clonedChild) {
            targetParent.appendChild(clonedChild);
        }
    }
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
            gadib_logger_d(`Dropped duplicate fact: ${canonicalKey}`);
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
            gadib_logger_d(`Hash collision detected, using version ${version}`);
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
    gadib_logger_d(`DFK FACT CAPTURE COMPLETED: ${factsRecorded} canonical facts recorded (${beforeDedupeCount} nodes processed, ${dedupeDropCount} duplicates deduped, ${collisionCount} hash collisions resolved)`);
    console.log(`[DFK-CAPTURE-TELEMETRY] facts_captured=${factsRecorded}, nodes_processed=${beforeDedupeCount}, duplicates_dropped=${dedupeDropCount}, collisions_resolved=${collisionCount}, deduplication_rate=${((dedupeDropCount / beforeDedupeCount) * 100).toFixed(1)}%`);
    
    // Sample DFK keys for validation
    const sampleKeys = Object.keys(deletionFactTable).slice(0, 3);
    if (sampleKeys.length > 0) {
        gadib_logger_d(`DFK FORMAT VALIDATION - Sample keys:`);
        sampleKeys.forEach(key => {
            gadib_logger_d(`  ${key}`);
        });
    }
    
    return deletionFactTable;
}

// Note: Using gadib_normalize_payload() and gadib_hash() from Base layer

// DFK Enhancement Method
function gadie_enhance_operations_with_dfk(diffOperations, deletionFactTable) {
    gadib_logger_d(`DFK ENHANCEMENT: Processing ${diffOperations.length} operations with GADS DFK mappings`);
    
    // Enhancement telemetry counters
    let deletionOpsFound = 0;
    let dfkEnhancements = 0;
    let unresolvedOps = 0;
    
    for (const op of diffOperations) {
        if (op.action === 'removeElement' || op.action === 'removeTextElement') {
            deletionOpsFound++;
            
            // GADS Primary Match: op.route → DFK.route exact string equality
            const opRouteStr = op.route.length === 0 ? "root" : op.route.join(',');
            
            // Find DFK entry with matching route
            let matchedDfkKey = null;
            let matchedEntry = null;
            
            for (const [dfkKey, dfkEntry] of Object.entries(deletionFactTable)) {
                if (dfkEntry.routeStr === opRouteStr) {
                    matchedDfkKey = dfkKey;
                    matchedEntry = dfkEntry;
                    break;
                }
            }
            
            if (matchedEntry) {
                // Attach DFK ID to operation for Phase 6 placement
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
                gadib_logger_d(`DFK-ENHANCED: ${matchedDfkKey.substring(0, 60)}... attached to ${op.action} operation`);
            } else {
                unresolvedOps++;
                gadib_logger_d(`DFK-UNRESOLVED: No DFK match for route [${opRouteStr}] - operation flagged as unresolved`);
            }
        }
    }
    
    // Report enhancement metrics
    gadib_logger_d(`DFK ENHANCEMENT COMPLETED: deletion_ops=${deletionOpsFound}, enhanced=${dfkEnhancements}, unresolved=${unresolvedOps}, enhancement_rate=${((dfkEnhancements / deletionOpsFound) * 100).toFixed(1)}%`);
    console.log(`[DFK-ENHANCEMENT-TELEMETRY] deletion_operations=${deletionOpsFound}, dfk_enhanced=${dfkEnhancements}, unresolved=${unresolvedOps}, enhancement_success_rate=${((dfkEnhancements / deletionOpsFound) * 100).toFixed(1)}%`);
}

// Drop unresolved operations before apply phase
function gadie_drop_unresolved_operations(workingDOM, operations) {
    gadib_logger_d('Testing route resolution for all operations');
    const readOnlyDOM = workingDOM.cloneNode(true);
    const resolvedOps = [];
    let droppedCount = 0;
    
    for (const op of operations) {
        const targetElement = gadie_find_element_by_route(readOnlyDOM, op.route);
        if (targetElement) {
            resolvedOps.push(op);
        } else {
            droppedCount++;
            gadib_logger_d(`Dropped unresolved operation: ${op.action} at route [${op.route.join(',')}]`);
        }
    }
    
    gadib_logger_d(`Pre-apply filtering complete: ${droppedCount} unresolved ops dropped`);
    return resolvedOps;
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
        
        gadib_logger_d(`Operation at route [${op.route?.join(',') || 'unknown'}]: ${op.action} → ${classifiedOp.semanticType}`);
        
        return classifiedOp;
    });
    
    // Group related operations by proximity and timing
    gadie_group_related_operations(classifiedOperations);
    
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
            // GADS Text Truncation Detection: Analyze for substring relationships
            return gadie_analyze_text_modification(operation, fromDOM, toDOM);
        case 'modifyValue':
        case 'modifyAttribute':
            return 'INLINE_MODIFICATION';
        
        default:
            gadib_logger_d(`Unknown operation action: ${action}`);
            return 'INLINE_MODIFICATION';
    }
}

// Helper: Analyze text modification to detect truncation vs genuine modification
function gadie_analyze_text_modification(operation, fromDOM, toDOM) {
    // Extract old and new values from the operation
    const oldValue = operation.oldValue || '';
    const newValue = operation.newValue || '';
    
    // Skip analysis for empty values or identical content
    if (!oldValue || !newValue || oldValue === newValue) {
        return 'INLINE_MODIFICATION';
    }
    
    // GADS Truncation Classification Rules
    
    // Rule 1: Prefix Truncation Detection
    // Check if newValue is a prefix of oldValue (truncated from end)
    if (oldValue.startsWith(newValue) && oldValue.length > newValue.length) {
        const deletedSuffix = oldValue.substring(newValue.length);
        
        // Apply minimum threshold to avoid false positives on minor changes
        if (deletedSuffix.length >= 5) {  // GADS threshold for significant truncation
            gadib_logger_d(`TRUNCATION-DETECTED: Prefix truncation - deleted suffix: "${deletedSuffix}"`);
            return 'INLINE_REMOVAL';
        }
    }
    
    // Rule 2: Suffix Truncation Detection  
    // Check if newValue is a suffix of oldValue (truncated from beginning)
    if (oldValue.endsWith(newValue) && oldValue.length > newValue.length) {
        const deletedPrefix = oldValue.substring(0, oldValue.length - newValue.length);
        
        // Apply minimum threshold to avoid false positives
        if (deletedPrefix.length >= 5) {  // GADS threshold for significant truncation
            gadib_logger_d(`TRUNCATION-DETECTED: Suffix truncation - deleted prefix: "${deletedPrefix}"`);
            return 'INLINE_REMOVAL';
        }
    }
    
    // Rule 3: Middle Truncation Detection
    // Check if newValue appears as a substring within oldValue (content removed from middle)
    const newIndex = oldValue.indexOf(newValue);
    if (newIndex !== -1) {
        const beforeNew = oldValue.substring(0, newIndex);
        const afterNew = oldValue.substring(newIndex + newValue.length);
        const totalDeleted = beforeNew.length + afterNew.length;
        
        if (totalDeleted >= 5) {  // GADS threshold for significant truncation
            gadib_logger_d(`TRUNCATION-DETECTED: Middle truncation - deleted content: "${beforeNew}" + "${afterNew}"`);
            return 'INLINE_REMOVAL';
        }
    }
    
    // Rule 4: True Modification (no substring relationship)
    // If no truncation patterns detected, this is a genuine text modification
    gadib_logger_d(`TEXT-MODIFICATION: No truncation pattern detected - treating as genuine modification`);
    return 'INLINE_MODIFICATION';
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

// Helper: Group related operations by proximity
function gadie_group_related_operations(classifiedOperations) {
    // Simple proximity grouping - could be enhanced with more sophisticated logic
    for (let i = 0; i < classifiedOperations.length - 1; i++) {
        const current = classifiedOperations[i];
        const next = classifiedOperations[i + 1];
        
        // Check if operations are adjacent in the DOM
        if (gadie_are_routes_adjacent(current.route, next.route)) {
            current.groupedWith = current.groupedWith || [];
            current.groupedWith.push(i + 1);
            next.groupedWith = next.groupedWith || [];
            next.groupedWith.push(i);
        }
    }
}

// Helper: Check if two routes are adjacent
function gadie_are_routes_adjacent(route1, route2) {
    if (!route1 || !route2) return false;
    if (route1.length !== route2.length) return false;
    
    // Check if routes differ by only 1 in the last position
    for (let i = 0; i < route1.length - 1; i++) {
        if (route1[i] !== route2[i]) return false;
    }
    
    return Math.abs(route1[route1.length - 1] - route2[route2.length - 1]) === 1;
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
function gadie_assemble_annotated_dom(detachedWorkingDOM, diffOperations, semanticAnchors, fromCommit, toCommit) {
    gadib_logger_d('Processing diff operations using true two-phase approach');
    
    // PRIORITY FIX #1: Drop unresolved ops before apply
    gadib_logger_d('Filtering out unresolved operations before apply phase');
    const preFilteredOps = gadie_drop_unresolved_operations(detachedWorkingDOM, diffOperations);
    gadib_logger_d(`Dropped ${diffOperations.length - preFilteredOps.length} unresolved operations`);
    
    // PRIORITY FIX #2: Canonicalize & deduplicate ops pre-apply
    const canonicalOps = gadie_canonicalize_and_deduplicate_operations(preFilteredOps);
    gadib_logger_d(`Deduplicated ${preFilteredOps.length - canonicalOps.length} operations`);
    
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
        
        // Fix: Record pre-resolution state
        preResolutionSnapshot.push({
            action: op.action,
            route: [...op.route],
            routeString: op.route.join(','),
            resolved: !!targetElement,
            targetNodeType: targetElement ? targetElement.nodeType : null,
            targetTag: targetElement && targetElement.tagName ? targetElement.tagName : null
        });
        
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
    const applyResult = gadie_apply_annotations_from_resolved(detachedWorkingDOM, resolvedOperations);
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
            gadib_logger_d(`Dropped duplicate: ${key}`);
        }
    }
    
    gadib_logger_d(`Deduplication complete: ${duplicateCount} duplicates removed`);
    return canonicalOps;
}

// Apply annotations from resolved operations
function gadie_apply_annotations_from_resolved(workingDOM, resolvedOperations) {
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
                gadie_apply_insertion_annotation(outputDOM, op);
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
                gadie_apply_move_annotation(outputDOM, op);
                moves++;
                applied = true;
                break;
            case 'modifyTextElement':
                gadie_handle_text_modification(outputDOM, op, {});
                modifications++;
                applied = true;
                break;
            case 'modifyAttribute':
                gadie_apply_modification_annotation(outputDOM, op);
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
function gadie_apply_insertion_annotation(outputDOM, operation) {
    const element = gadie_find_element_by_route(outputDOM, operation.route);
    
    // Strict semantic classification - no fallback to default classes
    if (!operation.visualTreatment?.cssClass) {
        gadib_logger_e(`SEMANTIC-ERROR: Operation at route [${operation.route.join(',')}] lacks visualTreatment.cssClass - skipping annotation`);
        return;
    }
    
    const cssClass = operation.visualTreatment.cssClass;
    
    if (element && element.nodeType === Node.ELEMENT_NODE) {
        element.classList.add(cssClass);
        gadib_logger_d(`Applied semantic insertion annotation (${cssClass}) at route [${operation.route.join(',')}]`);
    } else if (element && element.nodeType === Node.TEXT_NODE && element.parentElement) {
        // For text insertions, create semantic highlighting 
        const parent = element.parentElement;
        const span = document.createElement('span');
        span.classList.add(cssClass);
        span.textContent = element.textContent;
        parent.replaceChild(span, element);
        gadib_logger_d(`Applied semantic text insertion annotation (${cssClass}) at route [${operation.route.join(',')}]`);
    } else {
        gadib_logger_e(`Failed to apply insertion annotation at route [${operation.route.join(',')}] - element not found or invalid`);
    }
}

// Apply modification annotation
function gadie_apply_modification_annotation(outputDOM, operation) {
    const element = gadie_find_element_by_route(outputDOM, operation.route);
    
    // Strict semantic classification - no fallback to default classes
    if (!operation.visualTreatment?.cssClass) {
        gadib_logger_e(`SEMANTIC-ERROR: Operation at route [${operation.route.join(',')}] lacks visualTreatment.cssClass - skipping annotation`);
        return;
    }
    
    const cssClass = operation.visualTreatment.cssClass;
    
    if (element && element.nodeType === Node.ELEMENT_NODE) {
        element.classList.add(cssClass);
        gadib_logger_d(`Applied semantic modification annotation (${cssClass}) at route [${operation.route.join(',')}]`);
    } else {
        gadib_logger_e(`Failed to apply modification annotation at route [${operation.route.join(',')}] - element not found`);
    }
}

// Apply move annotation (placeholder)
function gadie_apply_move_annotation(outputDOM, operation) {
    gadib_logger_d(`Move annotation placeholder for route [${operation.route.join(',')}]`);
    // Move operations are complex and would need full implementation
}

// Handle text modification
function gadie_handle_text_modification(outputDOM, operation, options) {
    const element = gadie_find_element_by_route(outputDOM, operation.route);
    if (element && element.nodeType === Node.TEXT_NODE) {
        // Apply modification styling to text node
        if (element.parentElement && operation.visualTreatment?.cssClass) {
            const parent = element.parentElement;
            const span = document.createElement('span');
            span.classList.add(operation.visualTreatment.cssClass);
            span.textContent = element.textContent;
            parent.replaceChild(span, element);
            gadib_logger_d(`Applied text modification annotation at route [${operation.route.join(',')}]`);
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
    gadib_logger_d(`Inserted error marker for ${operation.action} at route [${operation.route.join(',')}]`);
}

// Phase 7: Place Deletion Blocks - DFK-driven placement
async function gadie_place_deletion_blocks(assembledDOM, appliedOperations, deletionFactTable, semanticAnchors) {
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
    
    // Report DFK table status
    const dfkCount = Object.keys(deletionFactTable).length;
    gadib_logger_d(`DFK TABLE: ${dfkCount} facts available for placement lookup`);
    gadib_logger_d(`APPLIED DELETIONS: Processing ${appliedDeletions.length} deletion operations`);
    
    for (const appliedOp of appliedDeletions) {
        // Parse route from string format back to array
        const route = appliedOp.route.split(',').map(Number);
        const dedupeKey = appliedOp.route + '_' + appliedOp.action;
        
        // Skip if already processed this (route + action) combination
        if (placedKeys.has(dedupeKey)) {
            gadib_logger_d(`Skipped duplicate: ${dedupeKey}`);
            continue;
        }
        
        // Find DFK entry for content using correct property name
        const routeKey = appliedOp.route;
        const dfkKey = Object.keys(deletionFactTable).find(key => 
            deletionFactTable[key].routeStr === routeKey
        );
        
        if (!dfkKey || !deletionFactTable[dfkKey]) {
            dfkMismatches++;
            gadib_logger_d(`DFK-MISMATCH: No DFK entry for route [${routeKey}] - attempting semantic-anchor fallback`);
            
            // Semantic-anchor bounded fallback for shifted routes with strict payload identity matching
            dfkFallbackAttempts++;
            const fallbackResult = await gadie_find_dfk_within_anchor(deletionPlacedDOM, route, appliedOp, deletionFactTable);
            
            if (fallbackResult && fallbackResult.type === 'exact_match') {
                dfkFallbackSuccess++;
                exactHashMatches++;
                gadib_logger_d(`DFK-FALLBACK-SUCCESS: Exact hash match ${fallbackResult.dfkKey.substring(0, 60)}... for route [${routeKey}]`);
                // Continue with this matched dfkKey
            } else if (fallbackResult && fallbackResult.type === 'ambiguous') {
                ambiguousMatches++;
                gadib_logger_d(`DFK-FALLBACK-AMBIGUOUS: Found ${fallbackResult.count} matching candidates for route [${routeKey}] - creating error marker`);
                
                // Create unplaced badge for ambiguous matches
                if (gadie_create_unplaced_deletion_badge(appliedOp, fallbackResult.anchor)) {
                    createdBadges++;
                    gadib_logger_d(`Created unplaced deletion badge for ambiguous route [${routeKey}]`);
                }
                continue;
            } else if (fallbackResult && fallbackResult.type === 'unplaced') {
                unplacedNoHash++;
                gadib_logger_d(`DFK-FALLBACK-UNPLACED: No payload hash match for route [${routeKey}] - creating unplaced marker`);
                
                // Create unplaced deletion badge at anchor root
                if (gadie_create_unplaced_deletion_badge(appliedOp, fallbackResult.anchor)) {
                    createdBadges++;
                    gadib_logger_d(`Created unplaced deletion badge for route [${routeKey}]`);
                }
                continue;
            } else {
                dfkFallbackFailed++;
                gadib_logger_d(`DFK-FALLBACK-FAILED: No semantic anchor found for route [${routeKey}] - operation cannot be placed`);
                continue;
            }
        }
        
        dfkMatchesfound++;
        const dfkEntry = deletionFactTable[dfkKey];
        gadib_logger_d(`DFK-MATCH: Found ${dfkKey.substring(0, 60)}... for route [${routeKey}]`);
        
        // Compute anchor using enhanced logic
        const anchor = gadie_find_stable_semantic_anchor(deletionPlacedDOM, route);
        if (!anchor) {
            anchorFailures++;
            gadib_logger_d(`ANCHOR-FAIL: No semantic anchor found for route [${routeKey}]`);
            continue;
        }
        
        anchorResolutions++;
        gadib_logger_d(`ANCHOR-SUCCESS: Resolved semantic anchor for route [${routeKey}]`);
        
        // Check if badge already placed at this location (guard)
        const existingBadge = deletionPlacedDOM.querySelector(`[data-gad-key="${dedupeKey}"]`);
        if (existingBadge) {
            gadib_logger_d(`Badge already exists for key: ${dedupeKey}`);
            continue;
        }
        
        // Create deletion badge
        const badgeResult = gadie_create_deletion_badge(dfkEntry, dedupeKey);
        const deletionBlock = badgeResult.element;
        createdBadges++;
        
        // Track inline vs block removal creation
        if (badgeResult.isInline) {
            inlineRemovalsCreated++;
        } else {
            blockRemovalsCreated++;
        }
        
        // Smart insertion based on anchor type
        if (gadie_insert_deletion_badge(anchor, deletionBlock)) {
            connectedBadges++;
            placedKeys.add(dedupeKey);
            gadib_logger_d(`Placed badge for ${dedupeKey}`);
        }
    }
    
    // Enhanced DFK Pipeline Telemetry
    const mismatch = appliedDeletions.length !== connectedBadges;
    gadib_logger_d(`DFK PIPELINE COMPLETED: applied_deletions=${appliedDeletions.length}, dfk_matches=${dfkMatchesfound}, dfk_mismatches=${dfkMismatches}, fallback_attempts=${dfkFallbackAttempts}, fallback_success=${dfkFallbackSuccess}, exact_hash_matches=${exactHashMatches}, unplaced_no_hash=${unplacedNoHash}, ambiguous_matches=${ambiguousMatches}, fallback_failed=${dfkFallbackFailed}, anchor_resolutions=${anchorResolutions}, anchor_failures=${anchorFailures}, created_badges=${createdBadges}, inline_removals=${inlineRemovalsCreated}, block_removals=${blockRemovalsCreated}, connected_badges=${connectedBadges}${mismatch ? ' PLACEMENT MISMATCH!' : ''}`);
    
    // Console telemetry for DFK pipeline effectiveness with strict payload identity tracking
    console.log(`[DFK-TELEMETRY] facts_available=${dfkCount}, operations_processed=${appliedDeletions.length}, dfk_matches=${dfkMatchesfound}, dfk_mismatches=${dfkMismatches}, fallback_attempts=${dfkFallbackAttempts}, fallback_success=${dfkFallbackSuccess}, exact_hash_matches=${exactHashMatches}, unplaced_no_hash=${unplacedNoHash}, ambiguous_matches=${ambiguousMatches}, fallback_failed=${dfkFallbackFailed}, anchor_resolutions=${anchorResolutions}, anchor_failures=${anchorFailures}, badges_placed=${connectedBadges}, inline_removals=${inlineRemovalsCreated}, block_removals=${blockRemovalsCreated}, placement_success_rate=${((connectedBadges / appliedDeletions.length) * 100).toFixed(1)}%`);
    
    return deletionPlacedDOM;
}

// Helper: Find DFK within semantic anchor (fallback for shifted routes)
async function gadie_find_dfk_within_anchor(dom, route, operation, deletionFactTable) {
    // Simplified fallback - would need full implementation for production
    gadib_logger_d(`Attempting DFK fallback for route [${route.join(',')}]`);
    return { type: 'unplaced', anchor: dom }; // Placeholder implementation
}

// Helper: Find stable semantic anchor for deletion placement
function gadie_find_stable_semantic_anchor(dom, route) {
    // Find the nearest parent element that can serve as a semantic anchor
    let element = gadie_find_element_by_route(dom, route.slice(0, -1)); // Parent route
    while (element && element !== dom) {
        if (element.nodeType === Node.ELEMENT_NODE) {
            const tagName = element.tagName.toLowerCase();
            if (['p', 'div', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'section', 'article'].includes(tagName)) {
                return element;
            }
        }
        element = element.parentElement;
    }
    return dom; // Fallback to root
}

// Helper: Create deletion badge with DFK metadata
function gadie_create_deletion_badge(dfkEntry, dedupeKey) {
    const isInline = dfkEntry.kind === '#text' || 
                    ['SPAN', 'A', 'EM', 'STRONG', 'B', 'I', 'CODE'].includes(dfkEntry.tag);
    
    const element = document.createElement(isInline ? 'span' : 'div');
    element.classList.add(isInline ? 'gads-deletion-inline' : 'gads-deletion-block');
    element.setAttribute('data-gad-key', dedupeKey);
    element.setAttribute('data-dfk-kind', dfkEntry.kind);
    element.setAttribute('data-dfk-tag', dfkEntry.tag);
    
    // Set content based on what was captured  
    if (dfkEntry.outerHTML) {
        // Don't wrap in <del> - CSS provides strikethrough styling
        element.innerHTML = gadie_escape_html(dfkEntry.outerHTML);
    } else {
        element.textContent = dfkEntry.textContent || '[Deleted content]';
    }
    
    gadib_logger_d(`Created ${isInline ? 'inline' : 'block'} deletion badge for ${dfkEntry.kind}:${dfkEntry.tag}`);
    
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

// Helper: Insert deletion badge at appropriate location
function gadie_insert_deletion_badge(anchor, badge) {
    if (!anchor || !badge) return false;
    
    try {
        // Insert at the beginning of the anchor element
        if (anchor.firstChild) {
            anchor.insertBefore(badge, anchor.firstChild);
        } else {
            anchor.appendChild(badge);
        }
        return true;
    } catch (error) {
        gadib_logger_e(`Failed to insert deletion badge: ${error.message}`);
        return false;
    }
}

// Phase 8: Merge Adjacent Same Nature - Visual coalescing
function gadie_merge_adjacent_same_nature(annotatedDOM) {
    // Phase 8: Visual coalescing - preserve semantic structure with presentation wrappers only
    gadib_logger_d('Starting Phase 8: Visual coalescing with presentation wrappers');
    
    // Create a new DOM to avoid modifying the annotated DOM
    const coalescedDOM = annotatedDOM.cloneNode(true);
    
    // Apply visual coalescing by adding presentation wrappers around adjacent same-nature elements
    gadie_add_visual_run_wrappers(coalescedDOM);
    
    gadib_logger_d('Phase 8 complete - semantic structure fully preserved');
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
                gadib_logger_d(`COALESCE-SUCCESS: Merged ${adjacentRun.length} adjacent ${className} elements`);
            } else if (coalescingResult && coalescingResult.useWrapper) {
                // Use wrapper container strategy
                gadie_create_wrapper_container(adjacentRun, className);
                runsCreated++;
                elementsMerged += adjacentRun.length;
                gadib_logger_d(`COALESCE-WRAPPER: Created wrapper for ${adjacentRun.length} ${className} elements with preserved semantics`);
            } else {
                runsSkipped++;
                gadib_logger_d(`COALESCE-SKIP: ${adjacentRun.length} ${className} elements - reason: UNSAFE_TO_MERGE`);
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
        gadib_logger_d(`COALESCE-BLOCK: Insufficient elements (${elements.length})`);
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
        gadib_logger_d(`COALESCE-WRAPPER: Anchor elements detected - will use wrapper container strategy`);
        return { useWrapper: true };
    }
    
    const firstTag = elements[0].tagName;
    
    // Do-Not-Merge Constraints for Interactive Elements
    for (const element of elements) {
        // Never merge interactive elements
        const interactiveTags = ['BUTTON', 'INPUT', 'SELECT', 'TEXTAREA'];
        if (interactiveTags.includes(element.tagName)) {
            gadib_logger_d(`COALESCE-BLOCK: Interactive ${element.tagName} element`);
            return false;
        }
        
        // All elements must have the same tag for direct merging
        if (element.tagName !== firstTag) {
            gadib_logger_d(`COALESCE-BLOCK: Mixed tags (${firstTag} vs ${element.tagName}) - considering wrapper strategy`);
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
    
    gadib_logger_d(`WRAPPER-STRATEGY: Created neutral container preserving ${adjacentRun.length} individual elements`);
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