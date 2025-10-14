# GADI Architecture Memo v2
## Pragmatic Refactor for LLM-Assisted Development

### Primary Rationale
Current GADI (~3500 lines) exceeds the effective working memory of LLM assistants, causing:
- Inconsistent suggestions across conversations
- Lost context about earlier design decisions
- Contradictory modifications
- "LLM slippage" where models lose traction on the codebase

**Goal:** Split GADI into focused modules of ~500-800 lines each, enabling coherent single-topic conversations with LLM assistants.

## Proposed 3-Module Architecture

### **GADIB – Base Utilities** (~400 lines)
**Purpose:** Shared infrastructure and transport layer

**Responsibilities:**
- WebSocket connection management
- Logger with phase/debug/telemetry methods
- Factory artifact shipping
- SHA-256 hashing utility
- Coalescing telemetry helpers

**Key Design:**
- Single source of truth for all I/O operations
- Fail-fast philosophy: errors throw immediately with clear context

**API Examples:**
```javascript
GADIB.logger.p(phase, message)  // Phase logging
GADIB.logger.d(message)          // Debug
GADIB.factory.ship(type, content, metadata)
GADIB.hash(payload) -> sha256:...
```

---

### **GADIE – Diff Engine** (~1200 lines)
**Purpose:** Complete 9-phase diff pipeline

**Responsibilities:**
- All 9 phases of diff processing (kept together as conceptual whole)
- DOM manipulation and route resolution (not separated)
- DFK creation and management
- Semantic classification
- Coalescing logic
- Deletion badge creation and placement

**Key Design:**
- Phases 6-9 remain integrated with 1-5 (avoid artificial boundaries)
- When diff breaks, entire algorithm is in one file
- Returns rendered HTML directly (no intermediate representation)

**API:**
```javascript
GADIE.diff(fromHtml, toHtml) -> {
  html: string,              // Final rendered diff
  metrics: DiffMetrics,      // Statistics
  debugArtifacts: Map<...>   // Phase outputs for Factory
}
```

---

### **GADIU – User Interface Controller** (~600 lines)
**Purpose:** User interaction and orchestration

**Responsibilities:**
- Manifest fetching and caching
- Rail population and selection logic
- URL/hash state management
- Swap button handling
- Commit resolution (position to hash mapping)
- Popover display
- Status ribbon updates
- Renders GADIE output into #renderedPane

**Key Design:**
- Owns all UI event handlers
- Manages all application state
- Single point of control for user interactions

---

## Rejected Alternatives

### Why Not a Separate Renderer Module?
Initially considered splitting DOM manipulation into GADIR, but rejected because:
1. **Artificial boundary:** Phases 6-9 are tightly coupled to phases 3-5
2. **Debugging complexity:** Would require tracing failures across module boundaries
3. **No real benefit:** Since graceful degradation isn't a goal (prefer fail-fast)
4. **Conceptual integrity:** The 9-phase pipeline is a single algorithm

### Why Not More Granular Modules?
Considered 5-6 modules but rejected because:
1. **Human context management:** Hard to remember which files to provide to LLM
2. **Architectural overhead:** More boundaries = more potential failure points
3. **LLM confusion:** Too many small files causes models to lose overall architecture

## Design Principles

### 1. Fail Fast and Loud
```javascript
// Good: Immediate failure with context
if (!element) {
  throw new Error(`Route [${route.join(',')}] unresolvable at index ${i}`);
}

// Bad: Silent recovery attempts
if (!element) {
  return this.findApproximateElement(route) || this.createErrorMarker();
}
```

### 2. Optimize for Single-File Debugging
When something breaks, you should know immediately which ONE file to open:
- Diff algorithm bug → GADIE
- UI interaction bug → GADIU
- Transport/logging bug → GADIB

### 3. LLM Conversation Structure
Each module enables focused conversations:
- "Fix the coalescing algorithm" → only need GADIE
- "Change rail selection behavior" → only need GADIU
- "Add new Factory artifact type" → only need GADIB

## Migration Path

### Phase 1: Extract GADIB (Day 1)
- Lowest risk, immediate benefit
- Reduces noise in all algorithm discussions
- ~2 hours effort

### Phase 2: Separate GADIU from GADIE (Day 2)
- Clear boundary at manifest.json processing
- UI events vs algorithmic processing
- ~4 hours effort

### Phase 3: Cleanup and Validation (Day 3)
- Ensure no cross-dependencies
- Verify each module is self-contained
- Add module loading to GADIH.html

## Success Metrics

1. **Each module readable in single LLM context** (under 1500 lines)
2. **Single-topic conversations possible** without providing other modules
3. **Debugging requires opening ONE file** for most issues
4. **No increase in total complexity** (same LOC, just organized)

## Anti-Patterns to Avoid

1. **Over-abstraction:** Don't create interfaces for single implementations
2. **Premature optimization:** This is for LLM collaboration, not performance
3. **Graceful degradation:** Fail immediately with clear errors
4. **Module proliferation:** Stop at 3 modules (4 with HTML)

## File Structure
```
gadi_inspector.html (GADIH) - 100 lines
gadb_base.js (GADIB) - 400 lines  
gadeL_engine.js (GADIE) - 1200 lines
gadu_user.js (GADIU) - 600 lines
gadc_cascade.css (GADIC) - unchanged
```

## Notes

- This refactor is specifically optimized for solo development with LLM assistance
- Architecture serves the development process, not abstract principles
- When in doubt, keep related code together rather than split it
- The goal is focused conversations, not architectural purity
