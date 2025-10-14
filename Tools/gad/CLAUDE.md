# Claude Code Project Memory - GAD (Google AsciiDoc Differ)

## GAD Project Overview
The GAD project implements a web-based tool for visualizing and analyzing differences between AsciiDoc documents, with special emphasis on structured diff computation and interactive inspection.

## Working in This Directory
When working exclusively in Tools/gad/, this file provides all necessary context for understanding GAD architecture, acronyms, and patterns.

## AsciiDoc Linked Terms (Core GAD Pattern)
When working with .adoc files using MCM patterns:
- **Linked Term**: Concept with three parts:
  - Attribute reference: `:prefix_snake_case:` (mapping section)
  - Replacement text: `<<anchor,Display Text>>` (what readers see)
  - Definition: `[[anchor]] {attribute}:: Definition text` (meaning)
- Definitions may be grouped in lists or dispersed through document
- GAD uses prefix: `gad_` for all terms
- Use snake_case for anchors, match attribute to anchor

## GAD Acronym Prefix Guide
GAD file acronyms follow a systematic pattern:
- **GADF** - GAD Factory (Python backend)
- **GADI** - GAD Inspector (JavaScript frontend)
  - **GADIB** - Inspector Base (infrastructure layer)
  - **GADIC** - Inspector Cascade (CSS styling)
  - **GADIE** - Inspector Engine (diff computation)
  - **GADIU** - Inspector User (UI interaction layer)
  - **GADIW** - Inspector Webpage (HTML container)
- **GADS** - GAD Specification (primary requirements doc)
- **GADP** - GAD Planner (design planning)
- **GADM** - GAD Memos (research, decisions, implementation notes)

## File Acronym Mappings

### Implementation Files
- **GADF**  → `gadf_factory.py` (Python backend - serves diffs, handles WebSocket)
- **GADIB** → `gadib_base.js` (Infrastructure - WebSocket, state, utilities)
- **GADIE** → `gadie_engine.js` (Diff computation engine)
- **GADIU** → `gadiu_user.js` (UI layer - event handlers, interactions)
- **GADIW** → `gadiw_webpage.html` (HTML page structure)
- **GADIC** → `gadic_cascade.css` (Styling and layout)

### Documentation Files
- **GADS**  → `GADS-GoogleAsciidocDifferSpecification.adoc` (Primary specification)
- **GADP**  → `GADP-GoogleAsciidocDifferPlanner.md` (Design planning document)

### Memo Files (Research & Implementation Notes)
- **GADMCR**  → `GADMCR-MemoCorsResolution.md` (CORS configuration)
- **GADMDD**  → `GADMDD-MemoDualDiffs.md` (Dual diff view design)
- **GADMDR**  → `GADMDR-MemoDeleteRefactor.md` (Delete operation refactoring)
- **GADMDUG** → `GADMDUG-MemoDifDomUsersGpt5.pdf` (GPT-5 research on diff/DOM)
- **GADMDUO** → `GADMDUO-MemoDifDomUsersOpus4p1.md` (Opus research on diff/DOM)
- **GADMPA1** → `GADMPA1-MemoPruneAlgorithm1.md` (Prune algorithm design)
- **GADMRC**  → `GADMRC-MemoRailCommit.html` (Rails commit visualization)
- **GADMRHA5** → `GADMRHA5-chatgpt5Research.pdf` (GPT-5 HTML algorithm research)
- **GADMRHAI** → `GADMRHAI-HtmlAlgoIssue.md` (HTML algorithm issues)
- **GADMRHAO** → `GADMRHAO-opus4p1Research.md` (Opus HTML algorithm research)
- **GADMRHAP** → `GADMRHAP-HtmlAlgoPrompt.md` (HTML algorithm prompt design)
- **GADMRW**  → `GADMRW-MemoRenderWickedFix.md` (Rendering bug fix)
- **GADMSI1** → `GADMSI1-MemoSplitInspector1.md` (Inspector split phase 1)
- **GADMSI2** → `GADMSI2-MemoSplitInspector2.md` (Inspector split phase 2)
- **GADMSI3** → `GADMSI3-MemoSplitInspector3.md` (Inspector split phase 3)
- **GADMWP**  → `GADMWP-MemoWebsocketPaths.md` (WebSocket path configuration)

## GAD Architecture Overview

### Three-Layer Inspector Architecture
1. **Base Layer (GADIB)**: WebSocket communication, state management, utilities
2. **Engine Layer (GADIE)**: Diff computation algorithms, data processing
3. **User Layer (GADIU)**: UI event handling, user interactions, view updates

### Key Concepts from GADS
- **Diff Sequence**: Ordered comparison of document versions
- **Prototype View**: Single-version inspection mode
- **Dual View**: Side-by-side comparison mode
- **Tab Navigation**: UI pattern for switching between views
- **Prune Algorithm**: Optimization for large diff computations

## Working Preferences
- Maintain separation between GADIB/GADIE/GADIU layers
- Follow linked term pattern for all GADS documentation
- Use snake_case with `gad_` prefix for all specification terms
- Keep memos for research and implementation decisions
- Test WebSocket communication when modifying GADF or GADIB

## Common Workflows
1. **Specification Updates**: Edit GADS, verify linked term consistency
2. **Backend Changes**: Modify GADF, test WebSocket/HTTP endpoints
3. **Frontend Changes**: Edit GADIE/GADIU/GADIB, maintain layer separation
4. **Styling Updates**: Modify GADIC, test across view modes
5. **Research**: Create new GADM memo to document findings

## Development Notes
- Python backend (GADF) serves static files and handles diff computation requests
- JavaScript frontend uses WebSocket for real-time diff updates
- CSS uses cascading patterns to support multiple view modes
- GADS specification uses MCM-style linked terms for precise concept definition
