// GADIU: GAD Inspector User Interface
// Extracted from monolithic gadi_inspector.html for modular architecture
// Contains: UI management, event handling, rail selection, URL state, commit resolution

class gadiu_inspector {
    constructor() {
        this.manifest = null;
        this.currentChanges = [];
        this.currentChangeIndex = 0;

        console.log('[DEBUG] gadiu_inspector constructor called');
        this.initializeElements();
        this.setupEventListeners();
        this.loadManifest();
    }

    initializeElements() {
        console.log('[DEBUG] Initializing DOM elements');
        this.elements = {
            comparisonHeader: document.getElementById('comparisonHeader'),
            statusSection: document.getElementById('statusSection'),
            fromRail: document.getElementById('fromRail'),
            fromRailContent: document.getElementById('fromRailContent'),
            toRail: document.getElementById('toRail'),
            toRailContent: document.getElementById('toRailContent'),
            swapButton: document.getElementById('swapButton'),
            renderedPane: document.getElementById('renderedPane'),
            commitPopover: document.getElementById('commitPopover'),
            commitPopoverContent: document.getElementById('commitPopoverContent'),
        };

        // Initialize selections
        this.selectedFrom = null;
        this.selectedTo = null;

        // Check if all elements were found
        Object.entries(this.elements).forEach(([key, element]) => {
            if (!element) {
                console.log(`[ERROR] Failed to find DOM element: ${key}`);
            }
        });
        console.log('[DEBUG] DOM elements initialized');
    }

    setupEventListeners() {
        this.elements.swapButton.addEventListener('click', () => this.onSwapClick());

        // Rail hover for popover display
        let hoverTimeout;
        let currentHoveredRow = null;

        [this.elements.fromRail, this.elements.toRail].forEach(rail => {
            rail.addEventListener('mouseover', (e) => {
                const railRow = e.target.closest('.rail-row');
                if (!railRow) return;

                // Clear any existing timeout
                if (hoverTimeout) {
                    clearTimeout(hoverTimeout);
                }

                currentHoveredRow = railRow;

                // Show popover after 200ms delay (GADS compliant)
                hoverTimeout = setTimeout(() => {
                    this.showCommitPopover(e, railRow);
                }, 200);
            });

            rail.addEventListener('mouseout', (e) => {
                const railRow = e.target.closest('.rail-row');
                if (!railRow) return;

                // Clear timeout if mouse leaves before delay
                if (hoverTimeout) {
                    clearTimeout(hoverTimeout);
                    hoverTimeout = null;
                }

                currentHoveredRow = null;

                // Hide popover immediately (GADS compliant)
                this.hideCommitPopover();
            });
        });

        // Initialize URL state tracking
        this.suppressUrlUpdate = false;

        // Parse URL state on browser navigation (back/forward)
        window.addEventListener('popstate', () => this.parseUrlState());
    }

    async loadManifest() {
        gadib_logger_d('Starting manifest load');
        gadib_logger_d('Loading start');

        try {
            gadib_logger_d('Fetching manifest.json');
            gadib_logger_d(`Fetch URL: ${window.location.origin}/manifest.json`);
            const response = await fetch('manifest.json');
            gadib_logger_d(`Fetch response status: ${response.status}`);
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            gadib_logger_d('Parsing manifest JSON');
            this.manifest = await response.json();
            gadib_logger_d(`Manifest loaded successfully with ${this.manifest.commits.length} commits`);
            gadib_logger_d(`Successful load with ${this.manifest.commits.length} commits`);

            // Now that Factory is confirmed available, attempt WebSocket connection
            gadib_connect_after_manifest();

            gadib_logger_d('Starting populateRails');
            this.populateRails();
            gadib_logger_d('populateRails completed');

            gadib_logger_d('Starting parseUrlState');
            this.parseUrlState();
            gadib_logger_d('parseUrlState completed');

        } catch (error) {
            gadib_logger_d(`Manifest load failed: ${error.message}`);
            gadib_logger_d(`Load failure: ${error.message}`);
            this.elements.renderedPane.innerHTML = `
                <div class="error">
                    <h3>Failed to load manifest.json</h3>
                    <p>${error.message}</p>
                </div>
            `;
        }
    }

    populateRails() {
        gadib_logger_d('Clearing rail content');
        // Clear existing content
        this.elements.fromRailContent.innerHTML = '';
        this.elements.toRailContent.innerHTML = '';

        gadib_logger_d('Rails will be populated with commits only (no magic values)');

        gadib_logger_d(`Processing ${this.manifest.commits.length} commits for rails`);
        // Reverse commits first, then apply simple labeling like GADMRC
        const reversedCommits = [...this.manifest.commits].reverse();
        reversedCommits.forEach((commit, index) => {
            gadib_logger_d(`Processing commit ${index + 1}: ${commit.hash.substring(0, 8)}`);
            const isUnchanged = index > 0 &&
                commit.html_sha256 === reversedCommits[index - 1].html_sha256;

            // GADMRC-compliant labeling: H for index 0, -1, -2, etc. for rest
            const position = index === 0 ? 'H' : `-${index}`;

            // Use position as the value, store commit data in dataset (no hash display)
            this.addRailRow(this.elements.fromRailContent, position, position, 'from', null, isUnchanged, commit);
            this.addRailRow(this.elements.toRailContent, position, position, 'to', null, isUnchanged, commit);
        });
        gadib_logger_d('Rail population completed');
        
        // Set default selections per GADS spec: From = -1, To = H
        this.setDefaultSelections();
    }
    
    setDefaultSelections() {
        // Only set defaults if no current selections AND no URL params present
        // Fix: Use only hash fragment, no search params
        const hashParams = this.parseHashParams();
        const hasHashParams = hashParams.get('from') || hashParams.get('to');
        
        if (!this.selectedFrom && !this.selectedTo && !hasHashParams) {
            gadib_logger_d('No selections and no hash params - setting default selections: From = -1, To = H');
            this.setRailSelection('from', '-1', true);  // Skip diff on first
            this.setRailSelection('to', 'H', false);    // Perform diff on second
        } else {
            gadib_logger_d(`Skipping defaults - selections: from=${this.selectedFrom}, to=${this.selectedTo}, hashParams=${hasHashParams}`);
        }
    }

    addRailRow(railContent, value, label, railType, hashText = null, isUnchanged = false, commit = null) {
        const row = document.createElement('div');
        row.className = 'rail-row' + (isUnchanged ? ' unchanged-rail-row' : '');
        row.dataset.value = value;
        row.dataset.railType = railType;
        
        // Store commit data for position resolution
        if (commit) {
            row.dataset.commitHash = commit.hash;
            row.dataset.commitData = JSON.stringify(commit);
        }

        const labelElement = document.createElement('div');
        labelElement.className = 'rail-label';
        labelElement.textContent = label;

        const commitInfo = document.createElement('div');
        commitInfo.className = 'rail-commit-info';
        
        // GADMRC-compliant: show only position labels, no hash text in rails
        if (hashText) {
            const hashElement = document.createElement('div');
            hashElement.className = 'rail-hash';
            hashElement.textContent = hashText;
            commitInfo.appendChild(hashElement);
        } else {
            // Empty info div - position label is sufficient
            commitInfo.style.display = 'none';
        }

        const radio = document.createElement('input');
        radio.type = 'radio';
        radio.name = railType + 'Selection';
        radio.className = 'rail-radio';
        radio.value = value;

        // Add click handler for selection
        row.addEventListener('click', () => {
            this.selectRailRow(row, railType);
        });

        radio.addEventListener('change', () => {
            if (radio.checked) {
                this.selectRailRow(row, railType);
            }
        });

        row.appendChild(labelElement);
        row.appendChild(commitInfo);
        row.appendChild(radio);

        railContent.appendChild(row);
    }

    selectRailRow(row, railType, skipDiff = false) {
        const value = row.dataset.value;
        
        // Clear previous selection in this rail
        const railContent = railType === 'from' ? this.elements.fromRailContent : this.elements.toRailContent;
        railContent.querySelectorAll('.rail-row').forEach(r => r.classList.remove('selected'));
        
        // Set new selection
        row.classList.add('selected');
        const radio = row.querySelector('.rail-radio');
        if (radio) radio.checked = true;

        // Store selection
        const previousValue = railType === 'from' ? this.selectedFrom : this.selectedTo;
        if (railType === 'from') {
            this.selectedFrom = value;
        } else {
            this.selectedTo = value;
        }

        gadib_logger_d(`${railType} rail selected: ${value}`);
        
        // Only update URL and perform diff if selection changed, not skipped, and not suppressed
        if (previousValue !== value && !skipDiff && !this.suppressUrlUpdate) {
            this.writeUrlState();
            this.performDiff();
        }
    }

    setRailSelection(railType, value, skipDiff = false) {
        const railContent = railType === 'from' ? this.elements.fromRailContent : this.elements.toRailContent;
        const targetRow = railContent.querySelector(`[data-value="${value}"]`);
        
        if (targetRow) {
            this.selectRailRow(targetRow, railType, skipDiff);
        } else {
            gadib_logger_d(`Rail row not found for value: ${value} in ${railType} rail`);
        }
    }

    parseHashParams() {
        // Fix: Parse hash fragment as URL parameters per GADS spec
        const hash = window.location.hash;
        if (!hash || hash === '#') {
            return new Map();
        }
        
        const paramString = hash.substring(1); // Remove leading #
        const params = new Map();
        
        if (paramString) {
            const pairs = paramString.split('&');
            for (const pair of pairs) {
                const [key, value] = pair.split('=');
                if (key && value) {
                    params.set(decodeURIComponent(key), decodeURIComponent(value));
                }
            }
        }
        
        return params;
    }
    
    parseUrlState() {
        gadib_logger_d(`parseUrlState called with hash: "${window.location.hash}"`);
        
        // Fix: Parse only from hash fragment, ignore search params completely
        const hashParams = this.parseHashParams();
        const fromHash = hashParams.get('from');
        const toHash = hashParams.get('to');

        gadib_logger_d(`Parsed hash params - from: "${fromHash}", to: "${toHash}"`);

        // Only proceed if we have hash parameters to process
        if (!fromHash && !toHash) {
            gadib_logger_d('No hash params present, not overriding current selections');
            return;
        }

        // Convert commit hashes to position values for rail selection
        let fromSet = false;
        let toSet = false;

        if (fromHash) {
            const fromPosition = this.hashToPosition(fromHash);
            if (fromPosition) {
                gadib_logger_d(`Setting from rail to position: ${fromPosition} (from hash: ${fromHash})`);
                this.setRailSelection('from', fromPosition, true); // Skip diff on first selection
                fromSet = true;
            } else {
                gadib_logger_d(`Failed to resolve from hash: ${fromHash} - ignoring`);
            }
        }

        if (toHash) {
            const toPosition = this.hashToPosition(toHash);
            if (toPosition) {
                gadib_logger_d(`Setting to rail to position: ${toPosition} (from hash: ${toHash})`);
                this.setRailSelection('to', toPosition, false); // Perform diff on second selection
                toSet = true;
            } else {
                gadib_logger_d(`Failed to resolve to hash: ${toHash} - ignoring`);
            }
        }

        // If only from was set and resolved, trigger diff
        if (fromSet && !toSet) {
            gadib_logger_d('Only from position set from URL, calling performDiff');
            this.performDiff();
        }
    }

    hashToPosition(commitHash) {
        if (!commitHash || !this.manifest || !this.manifest.commits) {
            return null;
        }

        // Find the commit by hash
        const commitIndex = this.manifest.commits.findIndex(c => c.hash === commitHash);
        if (commitIndex === -1) {
            return null;
        }

        // Convert index to position value (H for head, -1, -2, etc.)
        if (commitIndex === this.manifest.commits.length - 1) {
            return 'H';
        } else {
            const offset = this.manifest.commits.length - 1 - commitIndex;
            return `-${offset}`;
        }
    }

    onSwapClick() {
        const fromValue = this.selectedFrom;
        const toValue = this.selectedTo;

        gadib_logger_d(`Transposing rail positions: ${fromValue} <-> ${toValue}`);

        // Guard against URL updates during transpose
        this.suppressUrlUpdate = true;

        // Transpose the column positions by exchanging selections
        // Skip diff on both selections during transpose
        this.setRailSelection('from', toValue, true);  // Skip diff
        this.setRailSelection('to', fromValue, true);  // Skip diff - we'll do single diff at end

        // Clear the guard flag
        this.suppressUrlUpdate = false;

        // Update URL with resolved commit hashes and perform single diff
        this.writeUrlState();
        this.performDiff();
    }

    writeUrlState() {
        // Fix: Use hash fragment instead of query params per GADS spec
        const fromCommit = this.resolveCommit(this.selectedFrom);
        const toCommit = this.resolveCommit(this.selectedTo);

        if (fromCommit && toCommit) {
            const newHash = `#from=${encodeURIComponent(fromCommit.hash)}&to=${encodeURIComponent(toCommit.hash)}`;
            const newUrl = window.location.protocol + '//' + window.location.host + window.location.pathname + newHash;
            window.history.replaceState(null, '', newUrl);
            gadib_logger_d(`Updated hash state: from=${fromCommit.hash.substring(0,8)} to=${toCommit.hash.substring(0,8)}`);
        }
    }

    async performDiff() {
        const fromValue = this.selectedFrom;
        const toValue = this.selectedTo;

        if (!fromValue || !toValue) return;

        try {
            gadib_logger_d('Magic value resolution start');

            const fromCommit = this.resolveCommit(fromValue);
            const toCommit = this.resolveCommit(toValue);

            if (!fromCommit || !toCommit) {
                gadib_logger_e('Invalid magic values or missing commits in manifest');
                throw new Error('Failed to resolve commits');
            }

            gadib_logger_d(`Resolved from: ${fromCommit.hash.substring(0, 8)}, to: ${toCommit.hash.substring(0, 8)}`);

            // Update comparison header
            this.elements.comparisonHeader.textContent =
                `${fromCommit.hash.substring(0, 8)} → ${toCommit.hash.substring(0, 8)}`;

            // Check for identical SHA256
            if (fromCommit.html_sha256 === toCommit.html_sha256) {
                gadib_logger_d('Detection of identical html_sha256 values between selected commits');
                this.showStatusMessage('These commits have identical rendered content');
            } else {
                gadib_logger_d('Detection of different html_sha256 values between selected commits');
                this.hideStatusMessage();
            }

            await this.fetchAndDiff(fromCommit, toCommit);

        } catch (error) {
            gadib_logger_e(`Diff error: ${error.message}`);
            this.elements.renderedPane.innerHTML = `
                <div class="error">
                    <h3>Diff Error</h3>
                    <p>${error.message}</p>
                </div>
            `;
        }
    }

    resolveCommit(positionValue) {
        // First check: direct hash lookup
        const commitByHash = this.manifest.commits.find(c => c.hash === positionValue);
        if (commitByHash) {
            return commitByHash;
        }

        // Second check: position value handling
        if (positionValue === 'H') {
            return this.manifest.commits[this.manifest.commits.length - 1];
        } else if (positionValue.startsWith('-')) {
            const offset = parseInt(positionValue.substring(1));
            const index = this.manifest.commits.length - 1 - offset;
            return index >= 0 ? this.manifest.commits[index] : null;
        }

        return null;
    }

    async fetchAndDiff(fromCommit, toCommit) {
        gadib_logger_d(`HTML fetch start for ${fromCommit.html_file} and ${toCommit.html_file}`);

        try {
            const [fromResponse, toResponse] = await Promise.all([
                fetch(`/output/${fromCommit.html_file}`),
                fetch(`/output/${toCommit.html_file}`)
            ]);

            if (!fromResponse.ok || !toResponse.ok) {
                throw new Error('Failed to fetch HTML files');
            }

            const [fromHtml, toHtml] = await Promise.all([
                fromResponse.text(),
                toResponse.text()
            ]);

            gadib_logger_d('Successful fetch completion');
            gadib_logger_d('Diff computation start');

            // Prepare source file information for gadfd_rendered_capture
            const sourceFiles = [
                `From: /output/${fromCommit.html_file}`,
                `To: /output/${toCommit.html_file}`
            ];

            // Use gadie_diff function from engine
            const styledDiff = await gadie_diff(fromHtml, toHtml, {
                fromCommit: fromCommit,
                toCommit: toCommit,
                sourceFiles: sourceFiles
            });

            this.elements.renderedPane.innerHTML = styledDiff;

            // Send rendered content to Factory via WebSocket for raw diff file creation
            gadib_logger_d(`Creating raw diff file for ${fromCommit.hash.substring(0, 8)} → ${toCommit.hash.substring(0, 8)}`);
            gadib_factory_ship('rendered', styledDiff, fromCommit, toCommit, sourceFiles);

            // Enhanced logging for Factory debugging
            const changeTypeBreakdown = {
                insertions: this.elements.renderedPane.querySelectorAll('.gads-addition-inline, .gads-addition-block').length,
                moves: this.elements.renderedPane.querySelectorAll('.gads-modification-structural').length,
                deletions: this.elements.renderedPane.querySelectorAll('.gads-deletion-inline, .gads-deletion-block').length,
                modifications: this.elements.renderedPane.querySelectorAll('.gads-modification-inline').length
            };

            const totalChanges = changeTypeBreakdown.insertions + changeTypeBreakdown.moves + 
                               changeTypeBreakdown.deletions + changeTypeBreakdown.modifications;

            // Debug trace for change counting
            gadib_logger_d(`Found ${totalChanges} total changes in rendered output`);
            gadib_logger_d(`Breakdown: ${changeTypeBreakdown.insertions} insertions, ${changeTypeBreakdown.moves} moves, ${changeTypeBreakdown.deletions} deletions, ${changeTypeBreakdown.modifications} modifications`);

            gadib_logger_d(`Completion with ${totalChanges} total changes`);
            gadib_logger_d(`Change breakdown: ${JSON.stringify(changeTypeBreakdown)}`);

        } catch (error) {
            // Enhanced error reporting for Factory debugging
            if (error.message.includes('diff-dom')) {
                gadib_logger_e(`diff-dom library failure: ${error.message}`);
            } else if (error.message.includes('fetch') || error.message.includes('HTTP')) {
                gadib_logger_d(`Fetch failure: ${error.message}`);
            } else {
                gadib_logger_e(`GADS processing failure: ${error.message}`);
            }

            // Additional debugging context
            gadib_logger_d(`Error occurred during diff processing for commits: ${fromCommit?.hash?.substring(0,8)} → ${toCommit?.hash?.substring(0,8)}`);
            throw error;
        }
    }

    showStatusMessage(message) {
        this.elements.statusSection.textContent = message;
        this.elements.statusSection.style.display = 'block';
    }

    hideStatusMessage() {
        this.elements.statusSection.style.display = 'none';
    }

    showCommitPopover(event, railRow) {
        const value = railRow.dataset.value;
        const commit = this.resolveCommit(value);
        if (!commit) return;

        // Position popover in upper-left of render pane (GADS compliant)
        this.positionPopoverInRenderPane();

        // Update popover content
        this.updateCommitPopoverContent(value, commit);

        // Show the popover
        this.elements.commitPopover.style.display = 'block';
    }

    updateCommitPopoverContent(value, commit) {
        // Determine status for GADMRC memo compliance
        const isHead = value === 'H';
        const isUnchanged = commit && this.manifest.commits.length > 1 &&
            this.manifest.commits.some((c, i) => 
                i > 0 && c.hash === commit.hash && 
                c.html_sha256 === this.manifest.commits[i-1].html_sha256
            );
        
        const status = isHead ? "HEAD" : (isUnchanged ? "No substantive change" : "Changed");

        this.elements.commitPopoverContent.innerHTML = `
            <div class="popover-field">
                <div class="popover-label">Position:</div>
                <div class="popover-value">${value}</div>
            </div>
            <div class="popover-field">
                <div class="popover-label">Hash:</div>
                <div class="popover-value">${commit.hash}</div>
            </div>
            <div class="popover-field">
                <div class="popover-label">Date:</div>
                <div class="popover-value">${new Date(commit.timestamp ? 
                    commit.timestamp.replace(/(\d{4})(\d{2})(\d{2})(\d{2})(\d{2})(\d{2})/, '$1-$2-$3T$4:$5:$6Z')
                    : commit.date || 'Unknown').toLocaleString()}</div>
            </div>
            <div class="popover-field">
                <div class="popover-label">Message:</div>
                <div class="popover-value">${commit.message}</div>
            </div>
            <div class="popover-field">
                <div class="popover-label">html_sha256:</div>
                <div class="popover-value">${commit.html_sha256}</div>
            </div>
            <div class="popover-field">
                <div class="popover-label">Status:</div>
                <div class="popover-value">${status}</div>
            </div>
        `;
    }

    hideCommitPopover() {
        this.elements.commitPopover.style.display = 'none';
    }

    positionPopoverInRenderPane() {
        // Position in upper-left of render pane as per GADS specification
        const renderPaneRect = this.elements.renderedPane.getBoundingClientRect();
        const popover = this.elements.commitPopover;

        popover.style.left = (renderPaneRect.left + 20) + 'px';
        popover.style.top = (renderPaneRect.top + 20) + 'px';
    }
}

// Export class globally
window.gadiu_inspector = gadiu_inspector;