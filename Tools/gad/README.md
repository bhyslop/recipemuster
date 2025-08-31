# GAD: Git AsciiDoc Diff System

GAD is a toolchain for tracking semantic changes in AsciiDoc documents across Git commits by generating normalized HTML comparisons that eliminate formatting noise while preserving meaningful content differences.

## Overview

The GAD system consists of three main components:

- **Factory**: Bash orchestrator that monitors Git branches, processes commits through AsciiDoc rendering, and maintains a manifest of normalized HTML artifacts
- **Python Distill**: Normalizes HTML output by removing volatile elements (timestamps, auto-generated content) and standardizing whitespace while calculating SHA-256 checksums
- **Inspector**: Web-based diff viewer with interactive navigation that displays semantic changes between commits using color-coded visualization

## Key Features

- **Semantic Preservation**: Only shows changes that affect rendered meaning, filtering out formatting-only modifications
- **Persistent Trace Buffer**: Debugging output survives page reloads using browser sessionStorage
- **Magic Values**: Automatic selection of meaningful comparison baselines (`latest`, `before-last`, `before-substantive`)
- **Skip Unchanged**: Visual indicators for commits with identical rendered content
- **Self-Contained Output**: Complete portable results in single directory

## Architecture

GAD operates as a single-threaded pipeline per working directory:

1. **Git Archive**: Extract commit files to clean workspace
2. **AsciiDoctor**: Render AsciiDoc to HTML with reproducible mode
3. **Distill**: Normalize HTML and calculate checksums
4. **Manifest**: Update JSON metadata with commit information
5. **Inspector**: Interactive web interface for diff visualization

The system maintains backward compatibility through persistent state files and supports incremental processing of new commits since the last run.

## Dependencies

- **Core**: asciidoctor 2.0+, git, bash 4.0+, python 3.6+
- **Python**: beautifulsoup4 for HTML processing
- **Web**: Modern browser with JavaScript ES6+ and CSS3 flexbox support
- **JavaScript**: wikEd diff library for word-level difference detection

## File Organization

```
working-directory/
├── output/                    # Self-contained deliverables
│   ├── branch-timestamp-hash.html  # Normalized artifacts
│   ├── manifest.json         # Commit metadata with checksums
│   └── gadt_inspector.html   # Static web application
├── .factory-extract/         # Temporary git archive workspace
├── .factory-distill/         # Temporary asciidoctor output
└── .factory-state            # Last processed commit tracking
```

This system enables precise tracking of documentation evolution while eliminating noise from formatting changes, build timestamps, and other volatile content.