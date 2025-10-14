# GADP: Git AsciiDoc Diff Planner

## Active ideas

* STRIP DOWN
* VIZ RAW
* BUILD UP
* debug!
* gads prefixes in css: appropriate?
* Make the localhost URL into a button
* Make button out of...
  Inspector available at http://localhost:8080
* Confirm attempt repair GADF numbering on preprocess, GADS should be right.
* Clean up the deleted text mess
* have HEAD update if new file appears
* Dark mode presentation
* Consider CLAUDE.md bidir map of acronyms to files, and also linking specs to followers enabling WCC_WATERFALL or WCC_REVERSE ops.

## DEFERRED CONCEPTS

### Experiment with Windows Terminal for OSC-52 and Claude Code Robustness

### Multi-HTML File Rendering ASCIIDOC

### Python Factory with Integrated Webserver

Convert the entire GAD Factory from Bash to Python, making Distill a function rather than a standalone script. This unified Python architecture would enable:

**Factory-as-Service Architecture:**
- Single Python process managing git operations, asciidoctor execution, and HTML normalization
- Built-in web server exposing the Inspector interface via configurable port
- Real-time WebSocket updates to browser clients during incremental processing
- Container-friendly deployment with external port visibility

**Simplified State Management:**
- Direct manifest manipulation without jq dependency
- In-memory caching of manifest state for performance
- Atomic transaction-like processing for render sequences
- Better error handling and recovery with Python exception handling

**Enhanced Development Experience:**
- Unified logging and debugging across all GAD components  
- Hot-reload capability for Inspector interface during development
- Built-in health checks and metrics endpoints for monitoring
- Configurable processing modes (batch, watch, on-demand via API)

**Container Integration Benefits:**
- Single Python container with all dependencies
- Web interface accessible via docker port mapping
- API endpoints for external automation and CI/CD integration
- Simplified deployment compared to current multi-script architecture

This would eliminate the complexity of Bash-to-Python interop while enabling GAD to function as a proper containerized service with web-based interaction.
