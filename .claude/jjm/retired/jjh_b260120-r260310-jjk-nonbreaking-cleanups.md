# Heat Trophy: jjk-nonbreaking-cleanups

**Firemark:** ₣AH
**Created:** 260120
**Retired:** 260310
**Status:** retired

## Paddock

# Paddock: jjk-commission-haiku-pilot

## Context

Architect JJK for tiered model execution: Haiku as the fast, cheap "jockey" for routine operations, with Opus as the "steward" brought in for judgment calls.

**Origin:** Trial run with Haiku piloting JJK went surprisingly well, but exposed gaps. The path forward: tune JJX output to be Haiku-legible, and explicitly elect Opus for operations requiring deep judgment.

**Vision:**
- Haiku handles 80% of operations — mechanical, well-structured, fast
- Opus handles 20% — bridleability assessment, complex groom decisions, ambiguous specs
- The system knows when small-think suffices and when big-think is needed

## Key Questions

1. **Escalation triggers** — Hard-coded (bridle → always opus) vs. dynamic (haiku recognizes "I need help")?
2. **Model switching mechanics** — How does Claude Code switch models mid-flow? Task tool with `model: "opus"`?
3. **Which operations need Opus?** — Bridle yes. Also: groom? reslate? quarter?
4. **Output redesign** — What makes JJX output "Haiku-friendly"? More explicit state, decision trees, less inference required.

## Architectural Considerations

- JJX commands should emit structured, unambiguous output that smaller models can parse reliably
- Slash command prompts may need model-specific variants or clearer decision criteria
- Bridling is fundamentally a judgment call about scope, ambiguity, and risk — Opus territory
- Cost/speed tradeoff is significant: Haiku for iteration, Opus for commitment

## Spook Log

A "spook" is a failure of slash command guidance or CLAUDE.md context to produce a correct `jjx_*` invocation, causing the agent to stumble and recover via `--help` or trial-and-error. Each spook indicates a gap in the upper API layer. The fix for a spook is always a deft adjustment to CLAUDE.md, a slash command, or jjx output — whichever layer failed to guide the agent correctly. The goal is prevention: make the same mistake impossible next time.

### jjx_alter --status (260209)

CLAUDE.md CLI reference shows `jjx_alter FIREMARK [--status racing|stabled]` but the actual CLI uses `--racing` / `--stabled` flags (no `--status` option). Agent used `--status racing`, got rejected, had to `--help` to discover the correct flag.

**Root cause:** CLAUDE.md CLI reference diverged from implemented CLI.

## References

- Tools/jjk/vov_veiled/JJS0-GallopsData.adoc — JJK data model
- .claude/commands/ — Slash command definitions
- Task tool `model` parameter — mechanism for model selection in subagents

## Paces

### orient-bitmap-displays (₢AHAAb) [complete]

**[260210-0223] complete**

Add file-touch bitmap and commit swim lanes to jjx_orient output.

## Context

Mount workflow calls jjx_orient, which returns Recent-work table but no file-level history. The bitmap displays already exist in jjx_show (parade) heat view. Adding them to orient helps the agent understand repo state without reaching past JJ to raw git.

## Scope

Wire existing bitmap/swim-lane rendering from parade into orient's output path. No new display features.

1. Update JJSCSD-saddle.adoc — document bitmap sections in orient output
2. In Rust orient (saddle) code, call the same bitmap rendering functions used by parade after the Recent-work section
3. Build and test

## Verification

- `jjx_orient AP` shows file-touch bitmap and commit swim lanes after Recent-work
- Output format matches jjx_show exactly
- tt/vow-b.Build.sh passes
- tt/vow-t.Test.sh passes

**[260210-0202] bridled**

Add file-touch bitmap and commit swim lanes to jjx_orient output.

## Context

Mount workflow calls jjx_orient, which returns Recent-work table but no file-level history. The bitmap displays already exist in jjx_show (parade) heat view. Adding them to orient helps the agent understand repo state without reaching past JJ to raw git.

## Scope

Wire existing bitmap/swim-lane rendering from parade into orient's output path. No new display features.

1. Update JJSCSD-saddle.adoc — document bitmap sections in orient output
2. In Rust orient (saddle) code, call the same bitmap rendering functions used by parade after the Recent-work section
3. Build and test

## Verification

- `jjx_orient AP` shows file-touch bitmap and commit swim lanes after Recent-work
- Output format matches jjx_show exactly
- tt/vow-b.Build.sh passes
- tt/vow-t.Test.sh passes

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: jjrpd_parade.rs, jjrsd_saddle.rs, JJSCSD-saddle.adoc (3 files) | Steps: 1. In jjrpd_parade.rs rename zjjrpd_print_file_bitmap to jjrpd_print_file_bitmap and zjjrpd_print_commit_swimlanes to jjrpd_print_commit_swimlanes, change both from fn to pub crate fn, update all call sites within parade.rs 2. In jjrsd_saddle.rs add use of jjrpd_print_file_bitmap and jjrpd_print_commit_swimlanes, add import for jjrg_Heat type, call both functions after Recent-work section passing firemark ref and heat ref 3. In JJSCSD-saddle.adoc add File-touch bitmap and Commit swim lanes sections to the output format spec after Recent-work section, matching the format documented in JJSCPD-parade.adoc 4. Build with tt/vow-b.Build.sh 5. Test with tt/vow-t.Test.sh | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260209-1153] rough**

Add file-touch bitmap and commit swim lanes to jjx_orient output.

## Context

Mount workflow calls jjx_orient, which returns Recent-work table but no file-level history. The bitmap displays already exist in jjx_show (parade) heat view. Adding them to orient helps the agent understand repo state without reaching past JJ to raw git.

## Scope

Wire existing bitmap/swim-lane rendering from parade into orient's output path. No new display features.

1. Update JJSCSD-saddle.adoc — document bitmap sections in orient output
2. In Rust orient (saddle) code, call the same bitmap rendering functions used by parade after the Recent-work section
3. Build and test

## Verification

- `jjx_orient AP` shows file-touch bitmap and commit swim lanes after Recent-work
- Output format matches jjx_show exactly
- tt/vow-b.Build.sh passes
- tt/vow-t.Test.sh passes

### spook-alter-status-flag (₢AHAAc) [complete]

**[260210-0235] complete**

Fix jjx_alter CLI reference in CLAUDE.md to match actual implementation.

## Spook

CLAUDE.md CLI reference shows `jjx_alter FIREMARK [--status racing|stabled]` but the actual CLI uses `--racing` / `--stabled` boolean flags. Agent invoked `--status racing`, got rejected, recovered via `--help`.

## Fix

Update CLAUDE.md JJK section: change `[--status racing|stabled]` to `[--racing|--stabled]` in the CLI Command Reference table.

## Verification

- CLAUDE.md CLI reference matches `jjx_alter --help` output

**[260209-1155] rough**

Fix jjx_alter CLI reference in CLAUDE.md to match actual implementation.

## Spook

CLAUDE.md CLI reference shows `jjx_alter FIREMARK [--status racing|stabled]` but the actual CLI uses `--racing` / `--stabled` boolean flags. Agent invoked `--status racing`, got rejected, recovered via `--help`.

## Fix

Update CLAUDE.md JJK section: change `[--status racing|stabled]` to `[--racing|--stabled]` in the CLI Command Reference table.

## Verification

- CLAUDE.md CLI reference matches `jjx_alter --help` output

### vos-add-routines-section (₢AHAAB) [complete]

**[260125-1412] complete**

Add Routines section to VOS spec, formalizing existing VVC infrastructure and adding vosr_probe.

## Context

VVC implementation exists (Tools/vvc/src/) but lacks formal VOS spec entries. JJK documents its routines properly with `// ⟦axl_voices axo_routine⟧` annotations; VOS should follow the same pattern.

## Deliverables

1. **Add vosr_ prefix mappings to VOS mapping section:**
   - `:vosr_lock:` — lock routine
   - `:vosr_commit:` — commit routine  
   - `:vosr_guard:` — guard routine
   - `:vosr_probe:` — probe routine (NEW)

2. **Add Routines section to VOS spec** (after Operations):
   ```
   == Routines
   
   Routines are internal reusable procedures invoked by operations.
   ```

3. **Document vosr_lock** — wraps vvcc_CommitLock:
   - Inputs: none
   - Behavior: acquire git ref lock via `git update-ref refs/vvg/locks/vvx`
   - Outputs: RAII guard (lock released on drop)
   - Create VOSRL-lock.adoc, include in VOS

4. **Document vosr_commit** — wraps vvcc_run workflow:
   - Inputs: prefix, message, allow_empty, no_stage, size limits
   - Behavior: stage → guard → generate message (optional claude) → commit
   - Outputs: commit hash
   - Create VOSRC-commit.adoc, include in VOS

5. **Document vosr_guard** — wraps vvcg_run:
   - Inputs: limit, warn thresholds
   - Behavior: check staged content size against limits
   - Outputs: exit code (0=ok, 1=blocked, 2=warn)
   - Create VOSRG-guard.adoc, include in VOS

6. **Document vosr_probe** (NEW):
   - Inputs: none
   - Behavior: 
     a. Spawn `claude -p --model haiku --system-prompt "Report only your exact model ID string." --tools "" "go"` (parallel via tokio)
     b. Same for sonnet, opus
     c. Collect hostname, platform via std library
   - Outputs: 5-line string (haiku/sonnet/opus model IDs, host, platform)
   - Create VOSRP-probe.adoc, include in VOS

## Files

- VOS0-VoxObscuraSpec.adoc (add mappings + Routines section)
- VOSRL-lock.adoc (new)
- VOSRC-commit.adoc (new)
- VOSRG-guard.adoc (new)
- VOSRP-probe.adoc (new)

## Pattern

Follow JJSA routine pattern:
- Anchor with `// ⟦axl_voices axo_routine⟧`
- Include file for each routine
- Standard sections: Inputs, Behavior, Outputs

Acceptance: VOS has Routines section with all four routines documented per AXLA voicing patterns.

**[260125-1401] rough**

Add Routines section to VOS spec, formalizing existing VVC infrastructure and adding vosr_probe.

## Context

VVC implementation exists (Tools/vvc/src/) but lacks formal VOS spec entries. JJK documents its routines properly with `// ⟦axl_voices axo_routine⟧` annotations; VOS should follow the same pattern.

## Deliverables

1. **Add vosr_ prefix mappings to VOS mapping section:**
   - `:vosr_lock:` — lock routine
   - `:vosr_commit:` — commit routine  
   - `:vosr_guard:` — guard routine
   - `:vosr_probe:` — probe routine (NEW)

2. **Add Routines section to VOS spec** (after Operations):
   ```
   == Routines
   
   Routines are internal reusable procedures invoked by operations.
   ```

3. **Document vosr_lock** — wraps vvcc_CommitLock:
   - Inputs: none
   - Behavior: acquire git ref lock via `git update-ref refs/vvg/locks/vvx`
   - Outputs: RAII guard (lock released on drop)
   - Create VOSRL-lock.adoc, include in VOS

4. **Document vosr_commit** — wraps vvcc_run workflow:
   - Inputs: prefix, message, allow_empty, no_stage, size limits
   - Behavior: stage → guard → generate message (optional claude) → commit
   - Outputs: commit hash
   - Create VOSRC-commit.adoc, include in VOS

5. **Document vosr_guard** — wraps vvcg_run:
   - Inputs: limit, warn thresholds
   - Behavior: check staged content size against limits
   - Outputs: exit code (0=ok, 1=blocked, 2=warn)
   - Create VOSRG-guard.adoc, include in VOS

6. **Document vosr_probe** (NEW):
   - Inputs: none
   - Behavior: 
     a. Spawn `claude -p --model haiku --system-prompt "Report only your exact model ID string." --tools "" "go"` (parallel via tokio)
     b. Same for sonnet, opus
     c. Collect hostname, platform via std library
   - Outputs: 5-line string (haiku/sonnet/opus model IDs, host, platform)
   - Create VOSRP-probe.adoc, include in VOS

## Files

- VOS0-VoxObscuraSpec.adoc (add mappings + Routines section)
- VOSRL-lock.adoc (new)
- VOSRC-commit.adoc (new)
- VOSRG-guard.adoc (new)
- VOSRP-probe.adoc (new)

## Pattern

Follow JJSA routine pattern:
- Anchor with `// ⟦axl_voices axo_routine⟧`
- Include file for each routine
- Standard sections: Inputs, Behavior, Outputs

Acceptance: VOS has Routines section with all four routines documented per AXLA voicing patterns.

**[260125-0841] rough**

Define session probe as a Rust-autonomous routine that invokes `claude --print` directly.

## Critical Architecture Decision

The session probe MUST be fully autonomous in Rust - no slash command interpretation. Follow the pattern established in `Tools/vvc/src/vvcc_commit.rs:196-249` where Rust spawns `claude --print --model haiku` as a subprocess.

## Definition to Establish First

Before writing any code, document this routine definition in JJSA:

**jjdr_session_probe** (Rust-autonomous routine):
- Inputs: firemark
- Behavior:
  1. Query steeplechase for most recent `s` marker for this heat
  2. If none found OR timestamp > 1 hour ago: probe needed
  3. If probe needed:
     a. Spawn `claude --print --model haiku` → capture model ID
     b. Spawn `claude --print --model sonnet` → capture model ID  
     c. Spawn `claude --print --model opus` → capture model ID
     d. Collect hostname via `hostname` command
     e. Collect platform via `uname -s` + `uname -m`
     f. Create `s` marker via internal chalk creation (same transaction)
  4. Return: probe was performed (true/false)
- Key: ALL execution happens in Rust. Slash commands just read the output flag.

## Files

1. JJS0-GallopsData.adoc - Add jjdr_session_probe mapping and inline definition
2. JJSCSD-saddle.adoc - Add Needs-session-probe output (Rust computes and sets this)
3. JJSCPD-parade.adoc - Add Needs-session-probe output for heat mode

## NOT in scope

- Slash command modifications - they consume the flag, don't execute the probe
- Separate JJSRSP file - routine is simple enough to define inline in JJSA

Acceptance: jjdr_session_probe is clearly defined as Rust-autonomous following vvcc pattern. Specs show Rust does the work.

**[260125-0828] bridled**

Update JJK specs to define session probe behavior correctly:

**New routine definition in JJSA:**

1. Add `jjdr_session_probe` to mapping section:
   `:jjdr_session_probe: <<jjdr_session_probe,session probe routine>>`

2. Add routine definition in Routines section:
   - Voiced as `axo_routine`
   - Inputs: firemark (heat to check)
   - Behavior:
     a. Query steeplechase for most recent \`s\` marker for that heat
     b. If none found OR timestamp > 1 hour ago: probe needed
     c. If probe needed: spawn haiku/sonnet/opus agents to report model IDs, collect hostname + platform
     d. Create \`s\` marker via jjx_chalk with session body
   - Outputs: session marker committed (or skipped if recent)

**Operation updates:**

3. JJSCSD-saddle.adoc:
   - Add \`Needs-session-probe: true|false\` to output
   - Logic: invoke jjdr_session_probe check (steps a-b only, report result)
   - Caller (slash command) handles actual probe execution

4. JJSCPD-parade.adoc:
   - Add \`Needs-session-probe\` output when firemark provided
   - Same logic as saddle

5. Slash commands (jjc-heat-groom.md, jjc-heat-mount.md):
   - Document session probe handling via jjdr_session_probe
   - groom needs probe check added to its flow
   - mount Step 2.5 already handles, verify alignment

**Key design point:** The Rust CLI (jjx_*) only CHECKS if probe is needed and reports via output flag. The slash command (running in Claude Code) performs the actual probe (spawning agents) and creates the chalk marker.

Acceptance: jjdr_session_probe is voiced as axo_routine in JJSA. Specs are consistent and clearly define the corrected session probe behavior.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: JJS0-GallopsData.adoc, JJSRSP-session-probe.adoc new, JJSCSD-saddle.adoc, JJSCPD-parade.adoc, jjc-heat-groom.md, jjc-heat-mount.md 6 files | Steps: 1. Add jjdr_session_probe mapping to JJSA 2. Create JJSRSP-session-probe.adoc with routine spec following JJSRLD pattern 3. Add include in JJSA Routines section 4. Add Needs-session-probe output to saddle spec 5. Add Needs-session-probe output to parade spec for heat mode 6. Add session probe step to groom slash command 7. Verify mount Step 2.5 alignment | Verify: Review consistency across all files

**[260125-0826] rough**

Update JJK specs to define session probe behavior correctly:

**New routine definition in JJSA:**

1. Add `jjdr_session_probe` to mapping section:
   `:jjdr_session_probe: <<jjdr_session_probe,session probe routine>>`

2. Add routine definition in Routines section:
   - Voiced as `axo_routine`
   - Inputs: firemark (heat to check)
   - Behavior:
     a. Query steeplechase for most recent \`s\` marker for that heat
     b. If none found OR timestamp > 1 hour ago: probe needed
     c. If probe needed: spawn haiku/sonnet/opus agents to report model IDs, collect hostname + platform
     d. Create \`s\` marker via jjx_chalk with session body
   - Outputs: session marker committed (or skipped if recent)

**Operation updates:**

3. JJSCSD-saddle.adoc:
   - Add \`Needs-session-probe: true|false\` to output
   - Logic: invoke jjdr_session_probe check (steps a-b only, report result)
   - Caller (slash command) handles actual probe execution

4. JJSCPD-parade.adoc:
   - Add \`Needs-session-probe\` output when firemark provided
   - Same logic as saddle

5. Slash commands (jjc-heat-groom.md, jjc-heat-mount.md):
   - Document session probe handling via jjdr_session_probe
   - groom needs probe check added to its flow
   - mount Step 2.5 already handles, verify alignment

**Key design point:** The Rust CLI (jjx_*) only CHECKS if probe is needed and reports via output flag. The slash command (running in Claude Code) performs the actual probe (spawning agents) and creates the chalk marker.

Acceptance: jjdr_session_probe is voiced as axo_routine in JJSA. Specs are consistent and clearly define the corrected session probe behavior.

**[260125-0823] rough**

Update JJK specs to define session probe behavior correctly:

1. JJS0-GallopsData.adoc - Session marker section:
   - Threshold check: look for most recent \`s\` marker for that heat (not any steeplechase commit)
   - 1-hour threshold relative to most recent \`s\` marker
   - List trigger operations: saddle, parade (with firemark), groom (slash command)
   - Clarify muster does NOT trigger (global, no heat context)

2. JJSCSD-saddle.adoc:
   - Update \`Needs-session-probe\` output logic to check \`s\` markers only
   - If no \`s\` marker OR most recent \`s\` > 1 hour: emit \`Needs-session-probe: true\`

3. JJSCPD-parade.adoc:
   - Add \`Needs-session-probe\` output when firemark provided
   - Same logic: check \`s\` markers for that heat

4. Slash commands (jjc-heat-groom.md, jjc-heat-mount.md):
   - Document session probe handling
   - groom needs probe check added to its flow
   - mount already has Step 2.5, verify alignment

Acceptance: Specs are consistent and clearly define the corrected session probe behavior.

### jjsa-session-probe-update (₢AHAAD) [complete]

**[260125-1417] complete**

Update JJSA to consume vosr_probe from VOS layer.

## Context

After VOS defines vosr_probe, JJSA needs to reference it for session marker creation. The session probe check (1-hour threshold) stays in JJK; the actual probing (claude --print calls) delegates to VOS.

## Deliverables

1. **Update session marker section in JJSA:**
   - Reference vosr_probe for model/platform collection
   - Clarify: JJK checks threshold, calls vosr_probe, formats result into session marker

2. **Update jjx_saddle spec (JJSCSD-saddle.adoc):**
   - Add `Needs-session-probe: true|false` to output
   - Logic: check steeplechase for most recent `s` marker for heat
   - If none or > 1 hour: emit true

3. **Update slash commands if needed:**
   - jjc-heat-mount.md — verify Step 2.5 alignment
   - jjc-heat-groom.md — add session probe check if missing

## NOT in scope

- vosr_probe definition (that's the VOS pace)
- Implementation (that's the coding pace)

## Dependency

Blocked by: vos-add-routines-section (₢AHAAB)

Acceptance: JJSA session marker section clearly references vosr_probe. Saddle spec includes Needs-session-probe output.

**[260125-1415] bridled**

Update JJSA to consume vosr_probe from VOS layer.

## Context

After VOS defines vosr_probe, JJSA needs to reference it for session marker creation. The session probe check (1-hour threshold) stays in JJK; the actual probing (claude --print calls) delegates to VOS.

## Deliverables

1. **Update session marker section in JJSA:**
   - Reference vosr_probe for model/platform collection
   - Clarify: JJK checks threshold, calls vosr_probe, formats result into session marker

2. **Update jjx_saddle spec (JJSCSD-saddle.adoc):**
   - Add `Needs-session-probe: true|false` to output
   - Logic: check steeplechase for most recent `s` marker for heat
   - If none or > 1 hour: emit true

3. **Update slash commands if needed:**
   - jjc-heat-mount.md — verify Step 2.5 alignment
   - jjc-heat-groom.md — add session probe check if missing

## NOT in scope

- vosr_probe definition (that's the VOS pace)
- Implementation (that's the coding pace)

## Dependency

Blocked by: vos-add-routines-section (₢AHAAB)

Acceptance: JJSA session marker section clearly references vosr_probe. Saddle spec includes Needs-session-probe output.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: JJS0-GallopsData.adoc, JJSCSD-saddle.adoc, jjc-heat-mount.md (3 files) | Steps: 1. Update JJSA session marker section lines 1123-1147 to reference vosr_probe instead of ephemeral subagents pattern 2. Add Needs-session-probe output line to JJSCSD-saddle.adoc stdout format and add behavior step to check steeplechase for most recent s marker 3. Update jjc-heat-mount.md Step 2.5 to clarify delegation to VOS vosr_probe routine | Verify: None - documentation only

**[260125-1403] rough**

Update JJSA to consume vosr_probe from VOS layer.

## Context

After VOS defines vosr_probe, JJSA needs to reference it for session marker creation. The session probe check (1-hour threshold) stays in JJK; the actual probing (claude --print calls) delegates to VOS.

## Deliverables

1. **Update session marker section in JJSA:**
   - Reference vosr_probe for model/platform collection
   - Clarify: JJK checks threshold, calls vosr_probe, formats result into session marker

2. **Update jjx_saddle spec (JJSCSD-saddle.adoc):**
   - Add `Needs-session-probe: true|false` to output
   - Logic: check steeplechase for most recent `s` marker for heat
   - If none or > 1 hour: emit true

3. **Update slash commands if needed:**
   - jjc-heat-mount.md — verify Step 2.5 alignment
   - jjc-heat-groom.md — add session probe check if missing

## NOT in scope

- vosr_probe definition (that's the VOS pace)
- Implementation (that's the coding pace)

## Dependency

Blocked by: vos-add-routines-section (₢AHAAB)

Acceptance: JJSA session marker section clearly references vosr_probe. Saddle spec includes Needs-session-probe output.

### impl-vosr-probe (₢AHAAE) [complete]

**[260127-1052] complete**

Implement vosr_probe routine in Rust.

## Context

VOS spec defines vosr_probe; this pace implements it following the vvcc_commit.rs pattern.

## Deliverables

1. **Create vosrp_probe.rs** in Tools/vok/src/ (or appropriate location):
   - Public function `vosr_probe() -> Result<String, String>`
   - Spawn 3 parallel claude invocations via tokio:
     ```
     claude -p --model haiku --system-prompt "Report only your exact model ID string." --tools "" "go"
     ```
   - Collect hostname via std::env or hostname command
   - Collect platform via uname equivalent
   - Return 5-line string format:
     ```
     haiku: {model_id}
     sonnet: {model_id}
     opus: {model_id}
     host: {hostname}
     platform: {os}-{arch}
     ```
   - On probe failure for a tier: return `{tier}: unavailable`

2. **Add tokio dependency** if not already present

3. **Wire into jjx_saddle** (or leave for separate pace if complex)

4. **Tests:**
   - Unit test for output format
   - Integration test (may need mocking for claude calls)

## Pattern

Follow vvcc_commit.rs:196-249 for spawning claude subprocess.

## Dependency

Blocked by: vos-add-routines-section (₢AHAAB), jjsa-session-probe-update (₢AHAAD)

Acceptance: `cargo build` succeeds, `cargo test` passes, vosr_probe returns expected format.

**[260127-1046] bridled**

Implement vosr_probe routine in Rust.

## Context

VOS spec defines vosr_probe; this pace implements it following the vvcc_commit.rs pattern.

## Deliverables

1. **Create vosrp_probe.rs** in Tools/vok/src/ (or appropriate location):
   - Public function `vosr_probe() -> Result<String, String>`
   - Spawn 3 parallel claude invocations via tokio:
     ```
     claude -p --model haiku --system-prompt "Report only your exact model ID string." --tools "" "go"
     ```
   - Collect hostname via std::env or hostname command
   - Collect platform via uname equivalent
   - Return 5-line string format:
     ```
     haiku: {model_id}
     sonnet: {model_id}
     opus: {model_id}
     host: {hostname}
     platform: {os}-{arch}
     ```
   - On probe failure for a tier: return `{tier}: unavailable`

2. **Add tokio dependency** if not already present

3. **Wire into jjx_saddle** (or leave for separate pace if complex)

4. **Tests:**
   - Unit test for output format
   - Integration test (may need mocking for claude calls)

## Pattern

Follow vvcc_commit.rs:196-249 for spawning claude subprocess.

## Dependency

Blocked by: vos-add-routines-section (₢AHAAB), jjsa-session-probe-update (₢AHAAD)

Acceptance: `cargo build` succeeds, `cargo test` passes, vosr_probe returns expected format.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/vvc/Cargo.toml, Tools/vvc/src/vvcp_probe.rs, Tools/vvc/src/lib.rs, Tools/vok/Cargo.toml, Tools/vok/src/vorm_main.rs, Tools/vok/vov_veiled/VOSRP-probe.adoc (6 files) | Steps: 1. Add tokio with rt-multi-thread and process features to vvc Cargo.toml 2. Add tokio with rt-multi-thread and macros features to vok Cargo.toml 3. Create vvcp_probe.rs with pub async fn vvcp_probe spawning 3 parallel claude invocations via tokio join, collecting hostname and platform 4. Add vvcp_probe module and re-export to vvc lib.rs 5. Add tokio main attribute to vorm_main.rs making main async 6. Update VOSRP-probe.adoc line 52 to point to Tools/vvc/src/vvcp_probe.rs | Verify: tt/vow-b.Build.sh

**[260125-1403] rough**

Implement vosr_probe routine in Rust.

## Context

VOS spec defines vosr_probe; this pace implements it following the vvcc_commit.rs pattern.

## Deliverables

1. **Create vosrp_probe.rs** in Tools/vok/src/ (or appropriate location):
   - Public function `vosr_probe() -> Result<String, String>`
   - Spawn 3 parallel claude invocations via tokio:
     ```
     claude -p --model haiku --system-prompt "Report only your exact model ID string." --tools "" "go"
     ```
   - Collect hostname via std::env or hostname command
   - Collect platform via uname equivalent
   - Return 5-line string format:
     ```
     haiku: {model_id}
     sonnet: {model_id}
     opus: {model_id}
     host: {hostname}
     platform: {os}-{arch}
     ```
   - On probe failure for a tier: return `{tier}: unavailable`

2. **Add tokio dependency** if not already present

3. **Wire into jjx_saddle** (or leave for separate pace if complex)

4. **Tests:**
   - Unit test for output format
   - Integration test (may need mocking for claude calls)

## Pattern

Follow vvcc_commit.rs:196-249 for spawning claude subprocess.

## Dependency

Blocked by: vos-add-routines-section (₢AHAAB), jjsa-session-probe-update (₢AHAAD)

Acceptance: `cargo build` succeeds, `cargo test` passes, vosr_probe returns expected format.

### impl-session-probe-triggers (₢AHAAC) [abandoned]

**[260125-1405] abandoned**

Superseded by decomposition into:
- vos-add-routines-section (₢AHAAB) — VOS spec
- jjsa-session-probe-update (₢AHAAD) — JJSA spec  
- impl-vosr-probe (₢AHAAE) — implementation

Original scope absorbed into the new three-pace structure.

**[260125-0823] rough**

Implement session probe trigger behavior in Rust and slash commands:

**Rust changes (jjx_*):**

1. jjx_saddle:
   - Query steeplechase for most recent \`s\` marker commit for the heat
   - If none found OR timestamp > 1 hour ago: emit \`Needs-session-probe: true\`
   - Existing behavior: checks any commit (wrong) → change to \`s\` markers only

2. jjx_parade:
   - When given a firemark (heat-level parade), add same \`s\` marker check
   - Emit \`Needs-session-probe: true\` using same logic as saddle
   - Pace-level parade (coronet): no session probe needed

**Slash command changes:**

3. jjc-heat-groom.md:
   - Add session probe check at start of groom flow
   - Either call jjx_saddle to get probe status, or add jjx_parade check
   - If probe needed, run the 3-agent model probe and create \`s\` marker via jjx_chalk

4. jjc-heat-mount.md:
   - Verify Step 2.5 works with corrected saddle output
   - No changes expected if saddle emits correct flag

**Verification:**
- Fresh heat with no \`s\` markers triggers probe on saddle/parade/groom
- Heat with recent \`s\` marker (< 1 hour) does not trigger probe
- Administrative commits (N, S, f, r, T) do not suppress probe

Acceptance: Session probes fire correctly based on \`s\` marker history, not general commit activity.

### wire-saddle-vosr-probe (₢AHAAF) [complete]

**[260127-1058] complete**

Wire vosr_probe into jjx_saddle output.

## Context

With vosr_probe implemented in VVC, jjx_saddle needs to call it when session probe is needed.

## Deliverables

1. Add VVC dependency to JJK Cargo.toml if not present
2. In jjrsd_saddle.rs, when needs_session_probe is true:
   - Call vvc::vosr_probe()
   - Include probe result in saddle output (new field or section)
3. Update saddle output format documentation

## Dependency

Blocked by: impl-vosr-probe (₢AHAAE)

Acceptance: jjx_saddle includes probe data when needs_session_probe is true.

**[260127-1050] bridled**

Wire vosr_probe into jjx_saddle output.

## Context

With vosr_probe implemented in VVC, jjx_saddle needs to call it when session probe is needed.

## Deliverables

1. Add VVC dependency to JJK Cargo.toml if not present
2. In jjrsd_saddle.rs, when needs_session_probe is true:
   - Call vvc::vosr_probe()
   - Include probe result in saddle output (new field or section)
3. Update saddle output format documentation

## Dependency

Blocked by: impl-vosr-probe (₢AHAAE)

Acceptance: jjx_saddle includes probe data when needs_session_probe is true.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/jjk/vov_veiled/src/jjrx_cli.rs, Tools/jjk/vov_veiled/src/jjrsd_saddle.rs, Tools/vok/src/vorm_main.rs (3 files) | Steps: 1. Make jjrx_dispatch async and add .await to the Saddle match arm only 2. Make jjrsd_run_saddle async 3. In saddle when needs_session_probe is true call vvc::vvcp_probe().await and print result as Session-probe section with 5 indented lines 4. In vorm_main.rs make dispatch_external async and add .await to jjrx_dispatch call | Verify: tt/vow-b.Build.sh

**[260125-1425] rough**

Wire vosr_probe into jjx_saddle output.

## Context

With vosr_probe implemented in VVC, jjx_saddle needs to call it when session probe is needed.

## Deliverables

1. Add VVC dependency to JJK Cargo.toml if not present
2. In jjrsd_saddle.rs, when needs_session_probe is true:
   - Call vvc::vosr_probe()
   - Include probe result in saddle output (new field or section)
3. Update saddle output format documentation

## Dependency

Blocked by: impl-vosr-probe (₢AHAAE)

Acceptance: jjx_saddle includes probe data when needs_session_probe is true.

### saddle-creates-session-chalk (₢AHAAI) [complete]

**[260127-1142] complete**

Make saddle create session chalk commit automatically.

## Context

After AHAAF, saddle calls vvcp_probe() and outputs Session-probe section. But Claude still needs to parse that and create the chalk commit. Per JJSA: "Session marker (created automatically by {jjdo_saddle})". Saddle should do the full operation in Rust.

## Deliverables

1. **In jjrsd_saddle.rs**, after calling vvcp_probe():
   - Use existing `jjrc_timestamp_full()` for YYMMDD-HHMM format (already in jjrc_core.rs)
   - Create chalk commit via jjx_chalk logic (or call jjrch_chalk directly):
     - Firemark (heat-level)
     - Marker: s
     - Description: "{YYMMDD-HHMM} session"
     - Body: the 5-line probe output
   - Output informational line: "Session-marker: {commit_hash} ({YYMMDD-HHMM})"

2. **Remove Session-probe section from output** — the probe data goes into the chalk commit body, not stdout

3. **Update saddle output format** — just the informational Session-marker line when probe occurred

## Dependency

Blocked by: wire-saddle-vosr-probe (AHAAF)

## Key Reference

Timestamp utility already exists at `jjrc_core::jjrc_timestamp_full()` (line 27-30). Pattern for importing: `use crate::jjrc_core::jjrc_timestamp_full;`

Acceptance: Running jjx_saddle on a heat with >1hr gap creates chalk commit automatically. No Claude participation needed.

**[260127-1138] bridled**

Make saddle create session chalk commit automatically.

## Context

After AHAAF, saddle calls vvcp_probe() and outputs Session-probe section. But Claude still needs to parse that and create the chalk commit. Per JJSA: "Session marker (created automatically by {jjdo_saddle})". Saddle should do the full operation in Rust.

## Deliverables

1. **In jjrsd_saddle.rs**, after calling vvcp_probe():
   - Use existing `jjrc_timestamp_full()` for YYMMDD-HHMM format (already in jjrc_core.rs)
   - Create chalk commit via jjx_chalk logic (or call jjrch_chalk directly):
     - Firemark (heat-level)
     - Marker: s
     - Description: "{YYMMDD-HHMM} session"
     - Body: the 5-line probe output
   - Output informational line: "Session-marker: {commit_hash} ({YYMMDD-HHMM})"

2. **Remove Session-probe section from output** — the probe data goes into the chalk commit body, not stdout

3. **Update saddle output format** — just the informational Session-marker line when probe occurred

## Dependency

Blocked by: wire-saddle-vosr-probe (AHAAF)

## Key Reference

Timestamp utility already exists at `jjrc_core::jjrc_timestamp_full()` (line 27-30). Pattern for importing: `use crate::jjrc_core::jjrc_timestamp_full;`

Acceptance: Running jjx_saddle on a heat with >1hr gap creates chalk commit automatically. No Claude participation needed.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/jjk/vov_veiled/src/jjrsd_saddle.rs (1 file) | Steps: 1. Add import for jjrc_timestamp_full from jjrc_core 2. After vvcp_probe succeeds get timestamp via jjrc_timestamp_full 3. Build commit message using jjrn_format_session_message with firemark and timestamp as description 4. Append probe_result as body after double newline 5. Call vvc::commit with allow_empty true and no_stage true 6. Replace Session-probe println block with single Session-marker line showing commit hash and timestamp | Verify: tt/vow-b.Build.sh and tt/vow-t.Test.sh

**[260127-1137] rough**

Make saddle create session chalk commit automatically.

## Context

After AHAAF, saddle calls vvcp_probe() and outputs Session-probe section. But Claude still needs to parse that and create the chalk commit. Per JJSA: "Session marker (created automatically by {jjdo_saddle})". Saddle should do the full operation in Rust.

## Deliverables

1. **In jjrsd_saddle.rs**, after calling vvcp_probe():
   - Use existing `jjrc_timestamp_full()` for YYMMDD-HHMM format (already in jjrc_core.rs)
   - Create chalk commit via jjx_chalk logic (or call jjrch_chalk directly):
     - Firemark (heat-level)
     - Marker: s
     - Description: "{YYMMDD-HHMM} session"
     - Body: the 5-line probe output
   - Output informational line: "Session-marker: {commit_hash} ({YYMMDD-HHMM})"

2. **Remove Session-probe section from output** — the probe data goes into the chalk commit body, not stdout

3. **Update saddle output format** — just the informational Session-marker line when probe occurred

## Dependency

Blocked by: wire-saddle-vosr-probe (AHAAF)

## Key Reference

Timestamp utility already exists at `jjrc_core::jjrc_timestamp_full()` (line 27-30). Pattern for importing: `use crate::jjrc_core::jjrc_timestamp_full;`

Acceptance: Running jjx_saddle on a heat with >1hr gap creates chalk commit automatically. No Claude participation needed.

**[260127-1101] bridled**

Make saddle create session chalk commit automatically.

## Context

After AHAAF, saddle calls vvcp_probe() and outputs Session-probe section. But Claude still needs to parse that and create the chalk commit. Per JJSA: "Session marker (created automatically by {jjdo_saddle})". Saddle should do the full operation in Rust.

## Deliverables

1. **In jjrsd_saddle.rs**, after calling vvcp_probe():
   - Generate timestamp in YYMMDD-HHMM format
   - Create chalk commit via jjx_chalk logic (or call jjrch_chalk directly):
     - Firemark (heat-level)
     - Marker: s
     - Description: "{YYMMDD-HHMM} session"
     - Body: the 5-line probe output
   - Output informational line: "Session-marker: {commit_hash} ({YYMMDD-HHMM})"

2. **Remove Session-probe section from output** — the probe data goes into the chalk commit body, not stdout

3. **Update saddle output format** — just the informational Session-marker line when probe occurred

## Dependency

Blocked by: wire-saddle-vosr-probe (AHAAF)

Acceptance: Running jjx_saddle on a heat with >1hr gap creates chalk commit automatically. No Claude participation needed.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/jjk/vov_veiled/src/jjrsd_saddle.rs (1 file) | Steps: 1. After vvcp_probe call succeeds generate timestamp via chrono in YYMMDD-HHMM format 2. Build chalk args with firemark and marker s and description as timestamp plus session and body as probe output 3. Call jjrch_run_chalk or equivalent to create the commit 4. Replace Session-probe output section with single line Session-marker showing commit hash and timestamp 5. Remove Needs-session-probe output since probe now happens automatically | Verify: tt/vow-b.Build.sh

**[260127-1057] rough**

Make saddle create session chalk commit automatically.

## Context

After AHAAF, saddle calls vvcp_probe() and outputs Session-probe section. But Claude still needs to parse that and create the chalk commit. Per JJSA: "Session marker (created automatically by {jjdo_saddle})". Saddle should do the full operation in Rust.

## Deliverables

1. **In jjrsd_saddle.rs**, after calling vvcp_probe():
   - Generate timestamp in YYMMDD-HHMM format
   - Create chalk commit via jjx_chalk logic (or call jjrch_chalk directly):
     - Firemark (heat-level)
     - Marker: s
     - Description: "{YYMMDD-HHMM} session"
     - Body: the 5-line probe output
   - Output informational line: "Session-marker: {commit_hash} ({YYMMDD-HHMM})"

2. **Remove Session-probe section from output** — the probe data goes into the chalk commit body, not stdout

3. **Update saddle output format** — just the informational Session-marker line when probe occurred

## Dependency

Blocked by: wire-saddle-vosr-probe (AHAAF)

Acceptance: Running jjx_saddle on a heat with >1hr gap creates chalk commit automatically. No Claude participation needed.

### remove-slash-cmd-session-probe (₢AHAAG) [complete]

**[260127-1146] complete**

Remove session probe logic from slash commands.

## Context

Session probe is moving from slash command (mount) to Rust (saddle calling vosr_probe).
The slash commands should no longer contain session probe implementation.

## Deliverables

1. Audit all slash commands in .claude/commands/ for session probe references
2. Remove Step 2.5 (Session probe) from jjc-heat-mount.md
3. Remove any probe-related instructions from other commands
4. Update any documentation that references the old probe location

## Dependency

Blocked by: wire-saddle-vosr-probe

Acceptance: No slash commands reference session probe implementation; probe is handled entirely by saddle.

**[260127-1119] bridled**

Remove session probe logic from slash commands.

## Context

Session probe is moving from slash command (mount) to Rust (saddle calling vosr_probe).
The slash commands should no longer contain session probe implementation.

## Deliverables

1. Audit all slash commands in .claude/commands/ for session probe references
2. Remove Step 2.5 (Session probe) from jjc-heat-mount.md
3. Remove any probe-related instructions from other commands
4. Update any documentation that references the old probe location

## Dependency

Blocked by: wire-saddle-vosr-probe

Acceptance: No slash commands reference session probe implementation; probe is handled entirely by saddle.

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: .claude/commands/jjc-heat-mount.md (1 file) | Steps: 1. Remove line 52 that parses Needs-session-probe from saddle output 2. Remove entire Step 2.5 Session probe section from line 56 through line 96 3. Renumber Step 3 to Step 2.5 or leave numbering as-is with gap 4. Verify no other references to session probe or Needs-session-probe remain in file | Verify: grep -v session-probe .claude/commands/jjc-heat-mount.md

**[260125-1425] rough**

Remove session probe logic from slash commands.

## Context

Session probe is moving from slash command (mount) to Rust (saddle calling vosr_probe).
The slash commands should no longer contain session probe implementation.

## Deliverables

1. Audit all slash commands in .claude/commands/ for session probe references
2. Remove Step 2.5 (Session probe) from jjc-heat-mount.md
3. Remove any probe-related instructions from other commands
4. Update any documentation that references the old probe location

## Dependency

Blocked by: wire-saddle-vosr-probe

Acceptance: No slash commands reference session probe implementation; probe is handled entirely by saddle.

### mount-auto-saddle-first-racing (₢AHAAR) [complete]

**[260206-0916] complete**

When /jjc-heat-mount is invoked, always start by showing racing heats (same table format as muster — factor/reuse the table-rendering code). Then:

- **No firemark argument**: Auto-select the first racing heat and saddle it.
- **With firemark argument**: Validate it's a racing heat, then saddle it.

Changes required:

1. **JJSA spec update** (jjk/vov_veiled/JJS0-GallopsData.adoc): Document that saddle accepts an optional firemark. When omitted, it selects the first racing heat. When provided, it validates against racing heats.

2. **Rust code update** (jjx_saddle handler):
   - Factor the muster table-rendering code so both jjx_muster and jjx_saddle can share it
   - On entry, print the racing heats table (reusing factored code, filtered to racing only)
   - No firemark: auto-select first racing heat, proceed with saddle
   - With firemark: validate it's racing, proceed with saddle
   - Error if no racing heats exist or if provided firemark isn't racing

3. **Slash command update** (jjc-heat-mount.md): Firemark is now optional. Document both paths.

Keep it simple — no extra flags. This is the most common operation.

**[260203-1845] rough**

When /jjc-heat-mount is invoked, always start by showing racing heats (same table format as muster — factor/reuse the table-rendering code). Then:

- **No firemark argument**: Auto-select the first racing heat and saddle it.
- **With firemark argument**: Validate it's a racing heat, then saddle it.

Changes required:

1. **JJSA spec update** (jjk/vov_veiled/JJS0-GallopsData.adoc): Document that saddle accepts an optional firemark. When omitted, it selects the first racing heat. When provided, it validates against racing heats.

2. **Rust code update** (jjx_saddle handler):
   - Factor the muster table-rendering code so both jjx_muster and jjx_saddle can share it
   - On entry, print the racing heats table (reusing factored code, filtered to racing only)
   - No firemark: auto-select first racing heat, proceed with saddle
   - With firemark: validate it's racing, proceed with saddle
   - Error if no racing heats exist or if provided firemark isn't racing

3. **Slash command update** (jjc-heat-mount.md): Firemark is now optional. Document both paths.

Keep it simple — no extra flags. This is the most common operation.

**[260203-1840] rough**

When /jjc-heat-mount is invoked without a firemark argument, automatically saddle the first racing heat instead of requiring the user to specify one.

Changes required:

1. **JJSA spec update** (jjk/vov_veiled/JJS0-GallopsData.adoc): Document the saddle command behavior when no firemark is provided — it should select the first racing heat by default.

2. **Rust code update** (jjx_saddle or jjx_mount handler): When no firemark argument is supplied:
   - Run the equivalent of muster to get racing heats
   - Print the racing heats list so the user sees context
   - Auto-select the first racing heat
   - Proceed with normal saddle behavior for that heat

3. **Slash command update** (jjc-heat-mount.md): Update the mount slash command to reflect that firemark is now optional, and document the auto-selection behavior.

This is a convenience improvement — currently the user must always specify a firemark, but in most sessions only one or two heats are actively racing.

### heat-file-touch-visualization (₢AHAAT) [complete]

**[260206-0948] complete**

Drafted from ₢AGAAJ in ₣AG.

Add file-touch visualization to parade output using a hybrid bitmap format optimized for token efficiency and model readability.

## Problem

No consolidated view of which files a heat has touched across paces. Makes it hard to assess blast radius, identify hotspots, or brief a new session.

## Design: Hybrid Bitmap

Combine a fixed-width bitmap (one char per pace) with file grouping — files sharing identical touch patterns collapse to one row. Bare filenames only (no paths), since the prefix naming discipline guarantees uniqueness.

### Format

```
Paces: A B C D E F
───────────────────
·x··x· jjrg_gallops.rs, jjrt_types.rs
··x··x JJS0-GallopsData.adoc
····x· jjro_ops.rs
```

**Pace legend:** One header line mapping single chars (coronet terminal letter) to pace silks. Fixed overhead regardless of file count.

**Bitmap rows:** One row per unique touch-pattern. `x` = touched, `·` = untouched. Files sharing a pattern are comma-separated on the same row.

**File names:** Bare filename only (e.g. `jjrg_gallops.rs` not `Tools/jjk/vov_veiled/src/jjrg_gallops.rs`). Prefix encodes provenance. Sort files within a group by prefix for visual clustering of co-located files.

### Properties

- Worst case (every file unique pattern): degrades to one file per line — still compact
- Best case (spec + impl + test move together): 3 files collapse to 1 row
- Fixed-width bitmap column is scannable and diffable
- Reads left-to-right: pattern → files

## Data Source

Steeplechase tack records contain basis SHAs. `git diff --name-only <sha1> <sha2>` between consecutive tack entries yields files per pace. Only completed paces with commits contribute rows.

## Integration

- Surface via `--files` flag on parade
- Compute on demand from tack SHAs (no caching needed for typical heat sizes)
- Paces with no commits yet are columns in the legend but will have no `x` marks

## Acceptance

- `jjx_parade <firemark> --files` outputs hybrid bitmap visualization
- Bare filenames, grouped by touch pattern, sorted by prefix within groups
- Pace legend uses coronet terminal letters

**[260206-0903] rough**

Drafted from ₢AGAAJ in ₣AG.

Add file-touch visualization to parade output using a hybrid bitmap format optimized for token efficiency and model readability.

## Problem

No consolidated view of which files a heat has touched across paces. Makes it hard to assess blast radius, identify hotspots, or brief a new session.

## Design: Hybrid Bitmap

Combine a fixed-width bitmap (one char per pace) with file grouping — files sharing identical touch patterns collapse to one row. Bare filenames only (no paths), since the prefix naming discipline guarantees uniqueness.

### Format

```
Paces: A B C D E F
───────────────────
·x··x· jjrg_gallops.rs, jjrt_types.rs
··x··x JJS0-GallopsData.adoc
····x· jjro_ops.rs
```

**Pace legend:** One header line mapping single chars (coronet terminal letter) to pace silks. Fixed overhead regardless of file count.

**Bitmap rows:** One row per unique touch-pattern. `x` = touched, `·` = untouched. Files sharing a pattern are comma-separated on the same row.

**File names:** Bare filename only (e.g. `jjrg_gallops.rs` not `Tools/jjk/vov_veiled/src/jjrg_gallops.rs`). Prefix encodes provenance. Sort files within a group by prefix for visual clustering of co-located files.

### Properties

- Worst case (every file unique pattern): degrades to one file per line — still compact
- Best case (spec + impl + test move together): 3 files collapse to 1 row
- Fixed-width bitmap column is scannable and diffable
- Reads left-to-right: pattern → files

## Data Source

Steeplechase tack records contain basis SHAs. `git diff --name-only <sha1> <sha2>` between consecutive tack entries yields files per pace. Only completed paces with commits contribute rows.

## Integration

- Surface via `--files` flag on parade
- Compute on demand from tack SHAs (no caching needed for typical heat sizes)
- Paces with no commits yet are columns in the legend but will have no `x` marks

## Acceptance

- `jjx_parade <firemark> --files` outputs hybrid bitmap visualization
- Bare filenames, grouped by touch pattern, sorted by prefix within groups
- Pace legend uses coronet terminal letters

**[260206-0902] rough**

Add file-touch visualization to parade output using a hybrid bitmap format optimized for token efficiency and model readability.

## Problem

No consolidated view of which files a heat has touched across paces. Makes it hard to assess blast radius, identify hotspots, or brief a new session.

## Design: Hybrid Bitmap

Combine a fixed-width bitmap (one char per pace) with file grouping — files sharing identical touch patterns collapse to one row. Bare filenames only (no paths), since the prefix naming discipline guarantees uniqueness.

### Format

```
Paces: A B C D E F
───────────────────
·x··x· jjrg_gallops.rs, jjrt_types.rs
··x··x JJS0-GallopsData.adoc
····x· jjro_ops.rs
```

**Pace legend:** One header line mapping single chars (coronet terminal letter) to pace silks. Fixed overhead regardless of file count.

**Bitmap rows:** One row per unique touch-pattern. `x` = touched, `·` = untouched. Files sharing a pattern are comma-separated on the same row.

**File names:** Bare filename only (e.g. `jjrg_gallops.rs` not `Tools/jjk/vov_veiled/src/jjrg_gallops.rs`). Prefix encodes provenance. Sort files within a group by prefix for visual clustering of co-located files.

### Properties

- Worst case (every file unique pattern): degrades to one file per line — still compact
- Best case (spec + impl + test move together): 3 files collapse to 1 row
- Fixed-width bitmap column is scannable and diffable
- Reads left-to-right: pattern → files

## Data Source

Steeplechase tack records contain basis SHAs. `git diff --name-only <sha1> <sha2>` between consecutive tack entries yields files per pace. Only completed paces with commits contribute rows.

## Integration

- Surface via `--files` flag on parade
- Compute on demand from tack SHAs (no caching needed for typical heat sizes)
- Paces with no commits yet are columns in the legend but will have no `x` marks

## Acceptance

- `jjx_parade <firemark> --files` outputs hybrid bitmap visualization
- Bare filenames, grouped by touch pattern, sorted by prefix within groups
- Pace legend uses coronet terminal letters

**[260206-0854] rough**

Add file-touch visualization to saddle and/or groom commands, showing which files a heat has modified and which paces touched them.

## Problem

When working on a heat, there is no consolidated view of which files have been affected across all paces. This makes it hard to:
- Assess blast radius of a heat's changes
- Identify hotspot files touched by many paces (potential conflict zones)
- Review whether a heat's scope has drifted from its paddock intent
- Brief a new session on what areas of the codebase a heat has been working in

## Proposed Feature

Surface a file-touch summary in saddle and/or groom output. Data source: steeplechase commit history (already tracked via rein/notch).

### Visualization Options

**Per-file summary:**
```
Files touched by ₣AG (5 paces, 12 commits):
  Tools/jjk/vov_veiled/src/jjrg_gallops.rs  [₢AGAAA ₢AGAAF]  3 commits
  Tools/jjk/vov_veiled/src/jjro_ops.rs       [₢AGAAE]          1 commit
  Tools/jjk/vov_veiled/JJS0-GallopsData.adoc [₢AGAAC ₢AGAAF]  4 commits
```

**Per-pace file list:**
```
₢AGAAA express-pace-state:
  jjrg_gallops.rs, jjrt_types.rs
₢AGAAF json-schema-commit-to-basis:
  jjrt_types.rs, jjrg_gallops.rs, JJS0-GallopsData.adoc
```

### Data Source

Steeplechase tack records already contain `basis` (commit SHA). Between consecutive tack entries, `git diff --name-only <sha1> <sha2>` yields the files changed per pace. Alternatively, parse commit messages matching `jjb:HALLMARK:CORONET:` pattern from git log.

## Design Questions

- Which command(s) should surface this? saddle, groom, parade, or a new subcommand?
- Should this be a flag (`--files`) or always shown?
- How to handle paces with no commits yet (rough/bridled but unstarted)?
- Performance: git log parsing for large heats — cache or compute on demand?
- Depends on base-commit (₢AGAAI)? Could use heat base_commit as diff anchor, or work without it by using per-pace tack SHAs.

## Acceptance

- Design decision on where visualization lives and what format
- If proceeding: implementation plan covering data extraction, formatting, and command integration

### pace-commit-timeline (₢AHAAU) [complete]

**[260206-2048] complete**

Add commit-history visualizations as standard output in parade, complementing the file-touch bitmap with temporal views.

## Two Views, One Infrastructure

Both views use steeplechase entries via `jjrs_get_entries()` and `git diff-tree` per commit. Both filter administrative files (see filtering section). Both are standard output, not opt-in flags. Both include a column-header line of terminal characters above bitmap rows (eliminates positional counting for model comprehension).

## Administrative File Filtering

Exclude files matching `.claude/jjm/*` (gallops JSON, paddock files, trophy files) from ALL bitmap outputs — file-touch bitmap, swim lanes, and pace commits. Best implemented as a filter in or around `zjjrpd_files_for_commit()` so it applies uniformly. This is a new filter that also retroactively applies to the existing file-touch bitmap from prior work.

If a commit touches ONLY administrative files, it should still appear as a column in swim lanes (it represents work) but will have no marks in the file-touch bitmap.

## Heat View: Commit Swim Lanes

Appended after the file-touch bitmap in heat parade. Shows paces × commits — when work happened and how it interleaved.

```
Commit swim lanes (x = commit affiliated with pace):

  1 B vos-add-routines-section
  2 D jjsa-session-probe-update
  3 * heat-level

123456789ab
··xx·xx····  D  4c
xx··x······  B  3c
··········x  *  1c
```

- Column-header line (123456789ab) above bitmap rows for positional decoding
- NO row labels on the left — bitmap starts at column 0, same convention as file-touch bitmap
- Row label (terminal char) and commit count suffix AFTER the bitmap, right-justified
- Column = commit (chronological, dense, 1-9 then a-z then A-Z for 61 max)
- Row = pace (only paces with commits, ordered by first appearance)
- `*` row for heat-level commits
- Vertical legend matches file-touch bitmap convention
- Contiguous marks = focused work; interleaved = context switching

**Important:** `jjrs_get_entries()` returns newest-first. Reverse with `.rev()` for chronological column order.

**Truncation:** Default to last 35 commits. If truncated, prefix output with `(showing last 35 of N commits)`. This matches the 1-9,a-z encoding range. Extend to A-Z (61 total) if needed later.

## Pace View: Files Per Commit

Appended after tack history in pace parade. Shows files × commits — what each commit in this pace touched.

```
Pace commits (x = commit touched file):

  1 abc1234  Fix the main bug
  2 def5678  Update tests
  3 ghi9abc  Refactor helper

123
xx· jjrpd_parade.rs
x·x jjro_ops.rs
·xx jjtpd_parade.rs
```

- Column-header line (123) above bitmap rows for positional decoding
- Column = commit within this pace (chronological, dense)
- Row = file (bare filename, grouped by touch pattern)
- Legend shows commit SHA + subject line
- Filter: exclude administrative files

**Sourcing:** `jjrs_get_entries()` returns all heat entries. Filter by coronet: `entries.iter().filter(|e| e.coronet.as_deref() == Some(coronet_display))`. Then reverse for chronological order.

## Edge Cases

- If a pace or heat has no non-administrative file commits, print: `(no project file commits)` instead of an empty bitmap
- If steeplechase query fails, print error to stderr and continue (don't fail the whole parade)

## Data Source

Steeplechase entries from `git log` matching `jjb:HALLMARK:IDENTITY:ACTION:` pattern. Same source as file-touch bitmap. NOT tack basis SHAs.

## Implementation

- Heat view: new function `zjjrpd_print_commit_swimlanes()`, called after `zjjrpd_print_file_bitmap()` in heat branch
- Pace view: new function `zjjrpd_print_pace_commits()`, called after tack history in pace branch
- Administrative filter: modify `zjjrpd_files_for_commit()` or add wrapper to exclude `.claude/jjm/*` paths — applies to ALL bitmap outputs including existing file-touch
- Both reuse `zjjrpd_files_for_commit()` and `zjjrpd_bare_filename()` from prior work
- Column-header line in all bitmap outputs
- Update JJSCPD-parade.adoc with examples and behavior description

## Acceptance

- Heat parade appends commit swim lanes after file-touch bitmap (standard, no flag)
- Pace parade appends files-per-commit bitmap after tack history (standard, no flag)
- Administrative files (`.claude/jjm/*`) filtered from all bitmap outputs
- Column-header line above all bitmap rows
- Vertical legend in both views
- Steeplechase entries reversed to chronological order
- Truncation at 35 commits with indicator message
- Empty-bitmap edge case handled gracefully
- JJSCPD spec updated with examples

**[260206-2033] rough**

Add commit-history visualizations as standard output in parade, complementing the file-touch bitmap with temporal views.

## Two Views, One Infrastructure

Both views use steeplechase entries via `jjrs_get_entries()` and `git diff-tree` per commit. Both filter administrative files (see filtering section). Both are standard output, not opt-in flags. Both include a column-header line of terminal characters above bitmap rows (eliminates positional counting for model comprehension).

## Administrative File Filtering

Exclude files matching `.claude/jjm/*` (gallops JSON, paddock files, trophy files) from ALL bitmap outputs — file-touch bitmap, swim lanes, and pace commits. Best implemented as a filter in or around `zjjrpd_files_for_commit()` so it applies uniformly. This is a new filter that also retroactively applies to the existing file-touch bitmap from prior work.

If a commit touches ONLY administrative files, it should still appear as a column in swim lanes (it represents work) but will have no marks in the file-touch bitmap.

## Heat View: Commit Swim Lanes

Appended after the file-touch bitmap in heat parade. Shows paces × commits — when work happened and how it interleaved.

```
Commit swim lanes (x = commit affiliated with pace):

  1 B vos-add-routines-section
  2 D jjsa-session-probe-update
  3 * heat-level

123456789ab
··xx·xx····  D  4c
xx··x······  B  3c
··········x  *  1c
```

- Column-header line (123456789ab) above bitmap rows for positional decoding
- NO row labels on the left — bitmap starts at column 0, same convention as file-touch bitmap
- Row label (terminal char) and commit count suffix AFTER the bitmap, right-justified
- Column = commit (chronological, dense, 1-9 then a-z then A-Z for 61 max)
- Row = pace (only paces with commits, ordered by first appearance)
- `*` row for heat-level commits
- Vertical legend matches file-touch bitmap convention
- Contiguous marks = focused work; interleaved = context switching

**Important:** `jjrs_get_entries()` returns newest-first. Reverse with `.rev()` for chronological column order.

**Truncation:** Default to last 35 commits. If truncated, prefix output with `(showing last 35 of N commits)`. This matches the 1-9,a-z encoding range. Extend to A-Z (61 total) if needed later.

## Pace View: Files Per Commit

Appended after tack history in pace parade. Shows files × commits — what each commit in this pace touched.

```
Pace commits (x = commit touched file):

  1 abc1234  Fix the main bug
  2 def5678  Update tests
  3 ghi9abc  Refactor helper

123
xx· jjrpd_parade.rs
x·x jjro_ops.rs
·xx jjtpd_parade.rs
```

- Column-header line (123) above bitmap rows for positional decoding
- Column = commit within this pace (chronological, dense)
- Row = file (bare filename, grouped by touch pattern)
- Legend shows commit SHA + subject line
- Filter: exclude administrative files

**Sourcing:** `jjrs_get_entries()` returns all heat entries. Filter by coronet: `entries.iter().filter(|e| e.coronet.as_deref() == Some(coronet_display))`. Then reverse for chronological order.

## Edge Cases

- If a pace or heat has no non-administrative file commits, print: `(no project file commits)` instead of an empty bitmap
- If steeplechase query fails, print error to stderr and continue (don't fail the whole parade)

## Data Source

Steeplechase entries from `git log` matching `jjb:HALLMARK:IDENTITY:ACTION:` pattern. Same source as file-touch bitmap. NOT tack basis SHAs.

## Implementation

- Heat view: new function `zjjrpd_print_commit_swimlanes()`, called after `zjjrpd_print_file_bitmap()` in heat branch
- Pace view: new function `zjjrpd_print_pace_commits()`, called after tack history in pace branch
- Administrative filter: modify `zjjrpd_files_for_commit()` or add wrapper to exclude `.claude/jjm/*` paths — applies to ALL bitmap outputs including existing file-touch
- Both reuse `zjjrpd_files_for_commit()` and `zjjrpd_bare_filename()` from prior work
- Column-header line in all bitmap outputs
- Update JJSCPD-parade.adoc with examples and behavior description

## Acceptance

- Heat parade appends commit swim lanes after file-touch bitmap (standard, no flag)
- Pace parade appends files-per-commit bitmap after tack history (standard, no flag)
- Administrative files (`.claude/jjm/*`) filtered from all bitmap outputs
- Column-header line above all bitmap rows
- Vertical legend in both views
- Steeplechase entries reversed to chronological order
- Truncation at 35 commits with indicator message
- Empty-bitmap edge case handled gracefully
- JJSCPD spec updated with examples

**[260206-2027] rough**

Add commit-history visualizations as standard output in parade, complementing the file-touch bitmap with temporal views.

## Two Views, One Infrastructure

Both views use steeplechase entries via `jjrs_get_entries()` and `git diff-tree` per commit. Both filter administrative files (`.claude/jjm/*` — gallops, paddocks, trophies). Both are standard output, not opt-in flags. Both include a column-header line of terminal characters above bitmap rows (eliminates positional counting for model comprehension).

### Heat View: Commit Swim Lanes

Appended after the file-touch bitmap in heat parade. Shows paces × commits — when work happened and how it interleaved.

```
Commit swim lanes (x = commit affiliated with pace):

  1 B vos-add-routines-section
  2 D jjsa-session-probe-update
  3 * heat-level

    123456789ab
B   xx··x······  3c
D   ··xx·xx····  4c
*   ··········x  1c
```

- Column-header line (123456789ab) above bitmap rows for positional decoding
- Column = commit (chronological, dense, 1-9 then a-z)
- Row = pace (terminal char label, only paces with commits)
- `*` row for heat-level commits
- Suffix = commit count per pace
- Vertical legend matches file-touch bitmap convention
- Contiguous marks = focused work; interleaved = context switching
- Filter: exclude commits touching only administrative files

### Pace View: Files Per Commit

Appended after tack history in pace parade. Shows files × commits — what each commit in this pace touched.

```
Pace commits (x = commit touched file):

  1 abc1234  Fix the main bug
  2 def5678  Update tests
  3 ghi9abc  Refactor helper

123
xx· jjrpd_parade.rs
x·x jjro_ops.rs
·xx jjtpd_parade.rs
```

- Column-header line (123) above bitmap rows for positional decoding
- Column = commit within this pace (chronological, dense)
- Row = file (bare filename, grouped by touch pattern)
- Legend shows commit SHA + subject line
- Filter: exclude administrative files

### Large Heat Handling

For heats with many commits, truncate swim lanes to last N commits with a prefix marker — the recent tail is most decision-relevant.

### Data Source

Steeplechase entries from `git log` matching `jjb:HALLMARK:IDENTITY:ACTION:` pattern. Same source as file-touch bitmap. NOT tack basis SHAs.

## Implementation

- Heat view: new function `zjjrpd_print_commit_swimlanes()`, called after `zjjrpd_print_file_bitmap()` in heat branch
- Pace view: new function `zjjrpd_print_pace_commits()`, called after tack history in pace branch
- Both reuse `zjjrpd_files_for_commit()` and `zjjrpd_bare_filename()` from prior work
- Column-header line in all bitmap outputs (file-touch, swim lanes, pace commits)
- Update JJSCPD-parade.adoc with examples and behavior description

## Acceptance

- Heat parade appends commit swim lanes after file-touch bitmap (standard, no flag)
- Pace parade appends files-per-commit bitmap after tack history (standard, no flag)
- Administrative files filtered from both views
- Column-header line above all bitmap rows
- Vertical legend in both views
- JJSCPD spec updated with examples

**[260206-2022] rough**

Add commit-history visualizations as standard output in parade, complementing the file-touch bitmap with temporal views.

## Two Views, One Infrastructure

Both views use steeplechase entries via `jjrs_get_entries()` and `git diff-tree` per commit. Both filter administrative files (`.claude/jjm/*` — gallops, paddocks, trophies). Both are standard output, not opt-in flags.

### Heat View: Commit Swim Lanes

Appended after the file-touch bitmap in heat parade. Shows paces × commits — when work happened and how it interleaved.

```
Commit swim lanes (x = commit affiliated with pace):

  1 B vos-add-routines-section
  2 D jjsa-session-probe-update
  3 * heat-level

Commits: 1 2 3 4 5 6 7 8 9 a b
B  x x · · x · · · · · ·  3c
D  · · x x · x x · · · ·  4c
*  · · · · · · · · · · x  1c
```

- Column = commit (chronological, dense, 1-9 then a-z)
- Row = pace (terminal char, only paces with commits)
- `*` row for heat-level commits
- Suffix = commit count per pace
- Vertical legend matches file-touch bitmap convention
- Contiguous marks = focused work; interleaved = context switching
- Filter: exclude commits touching only administrative files

### Pace View: Files Per Commit

Appended after tack history in pace parade. Shows files × commits — what each commit in this pace touched.

```
Pace commits (x = commit touched file):

  1 abc1234  Fix the main bug
  2 def5678  Update tests
  3 ghi9abc  Refactor helper

xx· jjrpd_parade.rs
x·x jjro_ops.rs
·xx jjtpd_parade.rs
```

- Column = commit within this pace (chronological, dense)
- Row = file (bare filename, grouped by touch pattern)
- Legend shows commit SHA + subject line
- Filter: exclude administrative files

### Large Heat Handling

For heats with many commits, truncate swim lanes to last N commits with `…` prefix — the recent tail is most decision-relevant.

### Data Source

Steeplechase entries from `git log` matching `jjb:HALLMARK:IDENTITY:ACTION:` pattern. Same source as file-touch bitmap. NOT tack basis SHAs.

## Implementation

- Heat view: new function `zjjrpd_print_commit_swimlanes()`, called after `zjjrpd_print_file_bitmap()` in heat branch
- Pace view: new function `zjjrpd_print_pace_commits()`, called after tack history in pace branch
- Both reuse `zjjrpd_files_for_commit()` and `zjjrpd_bare_filename()` from ₢AHAAT
- Update JJSCPD-parade.adoc with examples and behavior description

## Acceptance

- Heat parade appends commit swim lanes after file-touch bitmap (standard, no flag)
- Pace parade appends files-per-commit bitmap after tack history (standard, no flag)
- Administrative files filtered from both views
- Vertical legend in both views
- JJSCPD spec updated with examples

**[260206-0948] rough**

Add commit-timeline visualization to parade output using a commit-indexed bitmap, complementing the spatial file-touch bitmap (₢AHAAT) with a temporal view.

## Design: Commit-Indexed Swim Lanes

Each row is a pace, each column is a commit (ordered chronologically). Marks show which pace each commit belongs to.

### Format

```
File-touch timeline (x = commit affiliated with pace):

  1 B vos-add-routines-section
  2 D jjsa-session-probe-update
  3 * heat-level

Commits: 1 2 3 4 5 6 7 8 9 a b
───────────────────────────────────
B  x x · · x · · · · · ·  3c
D  · · x x · x x · · · ·  4c
*  · · · · · · · · · · x  1c
```

**Vertical legend:** Numbered list mapping coronet terminal char to pace silks, matching file-touch bitmap (₢AHAAT) convention.

**Column index:** Sequential commit identifier (1-9, a-z for >9). Every column has at least one mark — dense, no wasted width.

**Row per pace:** Only paces with commits appear. Terminal char as row label. Suffix shows total commit count.

**Heat-level row:** `*` row for commits affiliated with heat but not a specific pace, matching file-touch bitmap convention.

### Properties

- Width = actual commit count (dense, no calendar gaps)
- Contiguous marks = focused work on one pace
- Interleaved marks across rows = parallel work / context switching
- Rightmost columns = most recent activity = what's "hot"
- A pace with marks only on the left = stale, may need re-engagement

### Data Source

Steeplechase entries via `jjrs_get_entries()` — pure git commits from `git log` matching `jjb:HALLMARK:IDENTITY:ACTION:` pattern. Each commit maps to a pace via coronet identity. Same data source as file-touch bitmap. NOT tack basis SHAs.

### Large Heat Handling

For heats with many commits, truncate to last N commits with a `...` prefix — the recent tail is most decision-relevant.

### Relationship to File-Touch Bitmap

Orthogonal views of the same underlying data:
- File-touch (₢AHAAT): files × paces — **where** paces overlap (spatial)
- Commit-timeline (this): paces × commits — **when** paces overlap (temporal)

Together: spatial-temporal map for orientation when investigating malformed paces.

## Integration

- Surface via `--timeline` flag on parade (note: ₢AHAAV may recommend relocating this)
- Compute on demand from steeplechase commit history

## Acceptance

- `jjx_parade <firemark> --timeline` outputs commit-indexed swim lane bitmap
- Commit axis is dense (no gaps), ordered chronologically
- Vertical legend uses coronet terminals matching file-touch convention
- `*` row for heat-level commits
- Commit count suffix per row

**[260206-0929] rough**

Add commit-timeline visualization to parade output using a commit-indexed bitmap, complementing the spatial file-touch bitmap (₢AHAAT) with a temporal view.

## Design: Commit-Indexed Swim Lanes

Each row is a pace, each column is a commit (ordered chronologically). Marks show which pace each commit belongs to.

### Format

```
Commits: 1 2 3 4 5 6 7 8 9 a b
───────────────────────────────────
₢AAA     x x · · x · · · · · ·  3c
₢AAB     · · x x · x x · · · ·  4c
₢AAC     · · · · · · · x x x ·  3c
₢AAD     · · · · · · · · · · x  1c
```

**Column index:** Sequential commit identifier (1-9, a-z for >9). Every column has at least one mark — no sparse matrix, no wasted width.

**Row per pace:** Only paces with commits appear. Suffix shows total commit count.

**Pace legend:** Map coronet terminals to silks (same style as file-touch bitmap).

### Properties

- Width = actual commit count (dense, no calendar gaps)
- Contiguous marks = focused work on one pace
- Interleaved marks across rows = parallel work / context switching
- Rightmost columns = most recent activity = what's "hot"
- A pace with marks only on the left = stale, may need re-engagement

### Data Source

Steeplechase tack records and git log with `jjb:HALLMARK:` pattern matching. Each commit maps to a pace via coronet in the commit message. Heat-only commits (firemark but no coronet) get a separate row or are omitted.

### Large Heat Handling

For heats with many commits, truncate to last N commits with a `...` prefix — the recent tail is most decision-relevant.

### Relationship to File-Touch Bitmap

These are orthogonal views of the same underlying data:
- File-touch (₢AHAAT): files × paces — **where** paces overlap (spatial)
- Commit-timeline (this pace): paces × commits — **when** paces overlap (temporal)

Together they form a spatial-temporal map: file-touch shows which files are conflict zones, commit-timeline shows when the conflicts occurred.

## Integration

- Surface via `--timeline` flag on parade
- Compute on demand from steeplechase commit history

## Acceptance

- `jjx_parade <firemark> --timeline` outputs commit-indexed swim lane bitmap
- Commit axis is dense (no gaps), ordered chronologically
- Pace legend uses coronet terminals matching file-touch convention
- Commit count suffix per row

### explore-parade-flag-reduction (₢AHAAV) [complete]

**[260206-2203] complete**

Explore whether parade has too many flags (--full, --remaining, --files) and whether some should become separate commands.

Consider:
- parade's core job vs. accumulated concerns
- Whether --files bitmap warrants its own command (e.g., jjx_bitmap)
- Whether --full and --remaining add value as flags vs. being defaults or separate commands
- CLI discoverability and option-count discipline

Produce a recommendation memo, not implementation.

**[260206-0943] rough**

Explore whether parade has too many flags (--full, --remaining, --files) and whether some should become separate commands.

Consider:
- parade's core job vs. accumulated concerns
- Whether --files bitmap warrants its own command (e.g., jjx_bitmap)
- Whether --full and --remaining add value as flags vs. being defaults or separate commands
- CLI discoverability and option-count discipline

Produce a recommendation memo, not implementation.

### fix-notch-deleted-files (₢AHAAQ) [complete]

**[260208-1417] complete**

Fix jjx_notch to accept deleted files in the file list.

## Problem
`jjx_notch` validates that all file arguments exist on disk (`path.exists()`). This fails for deleted files because they no longer exist, even though git tracks them.

## Solution
Change validation to accept files that either:
1. Exist on disk (additions/modifications), OR
2. Are tracked by git (deletions) — check via `git ls-files --error-unmatch <file>`

## Files to modify

### Implementation
- `Tools/jjk/vov_veiled/src/jjrnc_notch.rs` lines 48-54
  - Replace `path.exists()` with: exists OR git-tracked
  - Use `git ls-files --error-unmatch <file>` to check if git knows the file

### Spec  
- `Tools/jjk/vov_veiled/JJSCNC-notch.adoc` line 28
  - Change: "If any file does not exist: exit 1 with..."
  - To: "If any file does not exist AND is not tracked by git: exit 1 with..."

### Tests
- `Tools/jjk/vov_veiled/src/jjrnc_notch.rs` or new test file
  - Add test case for deleted file acceptance

## Verification
After implementation:
1. `tt/vow-b.Build.sh` — build
2. `tt/vow-t.Test.sh` — run tests
3. Manual test: stage a deletion, run notch with deleted file path

**[260128-0801] bridled**

Fix jjx_notch to accept deleted files in the file list.

## Problem
`jjx_notch` validates that all file arguments exist on disk (`path.exists()`). This fails for deleted files because they no longer exist, even though git tracks them.

## Solution
Change validation to accept files that either:
1. Exist on disk (additions/modifications), OR
2. Are tracked by git (deletions) — check via `git ls-files --error-unmatch <file>`

## Files to modify

### Implementation
- `Tools/jjk/vov_veiled/src/jjrnc_notch.rs` lines 48-54
  - Replace `path.exists()` with: exists OR git-tracked
  - Use `git ls-files --error-unmatch <file>` to check if git knows the file

### Spec  
- `Tools/jjk/vov_veiled/JJSCNC-notch.adoc` line 28
  - Change: "If any file does not exist: exit 1 with..."
  - To: "If any file does not exist AND is not tracked by git: exit 1 with..."

### Tests
- `Tools/jjk/vov_veiled/src/jjrnc_notch.rs` or new test file
  - Add test case for deleted file acceptance

## Verification
After implementation:
1. `tt/vow-b.Build.sh` — build
2. `tt/vow-t.Test.sh` — run tests
3. Manual test: stage a deletion, run notch with deleted file path

*Direction:* Agent: haiku | Cardinality: 1 sequential | Files: jjrnc_notch.rs, JJSCNC-notch.adoc (2 files) | Steps: 1. In jjrnc_notch.rs replace path.exists check with exists OR git-tracked via git ls-files --error-unmatch 2. In JJSCNC-notch.adoc update line 28 to allow tracked deletions 3. Build and test | Verify: tt/vow-b.Build.sh && tt/vow-t.Test.sh

**[260128-0801] rough**

Fix jjx_notch to accept deleted files in the file list.

## Problem
`jjx_notch` validates that all file arguments exist on disk (`path.exists()`). This fails for deleted files because they no longer exist, even though git tracks them.

## Solution
Change validation to accept files that either:
1. Exist on disk (additions/modifications), OR
2. Are tracked by git (deletions) — check via `git ls-files --error-unmatch <file>`

## Files to modify

### Implementation
- `Tools/jjk/vov_veiled/src/jjrnc_notch.rs` lines 48-54
  - Replace `path.exists()` with: exists OR git-tracked
  - Use `git ls-files --error-unmatch <file>` to check if git knows the file

### Spec  
- `Tools/jjk/vov_veiled/JJSCNC-notch.adoc` line 28
  - Change: "If any file does not exist: exit 1 with..."
  - To: "If any file does not exist AND is not tracked by git: exit 1 with..."

### Tests
- `Tools/jjk/vov_veiled/src/jjrnc_notch.rs` or new test file
  - Add test case for deleted file acceptance

## Verification
After implementation:
1. `tt/vow-b.Build.sh` — build
2. `tt/vow-t.Test.sh` — run tests
3. Manual test: stage a deletion, run notch with deleted file path

### design-model-tiering-strategy (₢AHAAA) [abandoned]

**[260208-1421] abandoned**

Design the model tiering strategy for JJK. Analyze which operations genuinely require Opus-level judgment vs. which can be reliably executed by Haiku. Map each jjx_* command and slash command to a model tier. Consider: What makes output 'Haiku-friendly'? What are the escalation triggers? How does the Task tool's model parameter integrate? Produce a tiering matrix and architectural recommendations.

**[260120-1818] rough**

Design the model tiering strategy for JJK. Analyze which operations genuinely require Opus-level judgment vs. which can be reliably executed by Haiku. Map each jjx_* command and slash command to a model tier. Consider: What makes output 'Haiku-friendly'? What are the escalation triggers? How does the Task tool's model parameter integrate? Produce a tiering matrix and architectural recommendations.

### doc-update-tokio-probe (₢AHAAH) [complete]

**[260208-1424] complete**

Update documentation to reflect tokio-based session probe.

## Context

With vosr_probe implemented in VVC (vvcp_probe.rs) and wired into saddle, documentation needs updating to reflect the new architecture.

## Deliverables

1. **JJSCSD-saddle.adoc** — Update output format:
   - Remove or update any Needs-session-probe reference
   - Add Session-probe section to output format example:
     ```
     Session-probe:
       haiku: claude-3-5-haiku-20241022
       sonnet: claude-sonnet-4-20250514
       opus: claude-opus-4-5-20251101
       host: macbook-pro.local
       platform: darwin-arm64
     ```
   - Add behavior step: "If session probe needed (gap > 1 hour), call vosr_probe and output Session-probe section"

2. **JJS0-GallopsData.adoc** — Update line ~1147:
   - OLD: "Model IDs are collected using ephemeral subagents created via --agents JSON flag..."
   - NEW: "Model IDs are collected via {vosr_probe} routine, which spawns parallel claude invocations via tokio async subprocess spawning."
   - Ensure {vosr_probe} attribute is defined if not present

## Dependency

Blocked by: wire-saddle-vosr-probe (AHAAF)

Acceptance: Both docs accurately describe the new tokio-based probe flow.

**[260127-1053] bridled**

Update documentation to reflect tokio-based session probe.

## Context

With vosr_probe implemented in VVC (vvcp_probe.rs) and wired into saddle, documentation needs updating to reflect the new architecture.

## Deliverables

1. **JJSCSD-saddle.adoc** — Update output format:
   - Remove or update any Needs-session-probe reference
   - Add Session-probe section to output format example:
     ```
     Session-probe:
       haiku: claude-3-5-haiku-20241022
       sonnet: claude-sonnet-4-20250514
       opus: claude-opus-4-5-20251101
       host: macbook-pro.local
       platform: darwin-arm64
     ```
   - Add behavior step: "If session probe needed (gap > 1 hour), call vosr_probe and output Session-probe section"

2. **JJS0-GallopsData.adoc** — Update line ~1147:
   - OLD: "Model IDs are collected using ephemeral subagents created via --agents JSON flag..."
   - NEW: "Model IDs are collected via {vosr_probe} routine, which spawns parallel claude invocations via tokio async subprocess spawning."
   - Ensure {vosr_probe} attribute is defined if not present

## Dependency

Blocked by: wire-saddle-vosr-probe (AHAAF)

Acceptance: Both docs accurately describe the new tokio-based probe flow.

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/jjk/vov_veiled/JJSCSD-saddle.adoc, Tools/jjk/vov_veiled/JJS0-GallopsData.adoc (2 files) | Steps: 1. In JJSCSD-saddle.adoc add Session-probe section to output format showing 5 indented lines for haiku sonnet opus host platform 2. Add behavior step after recent-work: if session probe needed call vosr_probe and output Session-probe section 3. In JJS0-GallopsData.adoc update line 1147 to say Model IDs are collected via vosr_probe routine which spawns parallel claude invocations via tokio | Verify: grep -q vosr_probe Tools/jjk/vov_veiled/JJS0-GallopsData.adoc

**[260127-1052] rough**

Update documentation to reflect tokio-based session probe.

## Context

With vosr_probe implemented in VVC (vvcp_probe.rs) and wired into saddle, documentation needs updating to reflect the new architecture.

## Deliverables

1. **JJSCSD-saddle.adoc** — Update output format:
   - Remove or update any Needs-session-probe reference
   - Add Session-probe section to output format example:
     ```
     Session-probe:
       haiku: claude-3-5-haiku-20241022
       sonnet: claude-sonnet-4-20250514
       opus: claude-opus-4-5-20251101
       host: macbook-pro.local
       platform: darwin-arm64
     ```
   - Add behavior step: "If session probe needed (gap > 1 hour), call vosr_probe and output Session-probe section"

2. **JJS0-GallopsData.adoc** — Update line ~1147:
   - OLD: "Model IDs are collected using ephemeral subagents created via --agents JSON flag..."
   - NEW: "Model IDs are collected via {vosr_probe} routine, which spawns parallel claude invocations via tokio async subprocess spawning."
   - Ensure {vosr_probe} attribute is defined if not present

## Dependency

Blocked by: wire-saddle-vosr-probe (AHAAF)

Acceptance: Both docs accurately describe the new tokio-based probe flow.

### cleanup-jjrq-dead-code (₢AHAAJ) [complete]

**[260127-1130] complete**

Clean up dead code in jjrq_query.rs after CLI modularization.

## Context

The Jan 24 CLI refactor (d72e0e2b, 34e85785) created per-command modules but left dead code in jjrq_query.rs.

Investigation confirmed these are DEAD (never called):
- jjrq_MusterArgs, jjrq_run_muster (replaced by jjrmu_muster)
- jjrq_ParadeArgs, jjrq_run_parade (replaced by jjrpd_parade)
- jjrq_ScoutArgs, jjrq_run_scout (replaced by jjrsc_scout)
- jjrq_GetSpecArgs, jjrq_run_get_spec (replaced by jjrgs_get_spec)
- jjrq_GetCoronetsArgs, jjrq_run_get_coronets (replaced by jjrgc_get_coronets)
- jjrq_RetireArgs, jjrq_run_retire (replaced by jjrrt_retire)
- jjrq_SaddleArgs, jjrq_run_saddle (ALREADY REMOVED - replaced by jjrsd_saddle)

These MUST STAY (actively used):
- jjrq_resolve_default_heat — used by jjrsd_saddle.rs
- jjrq_parse_silks_sequence — used by jjro_ops.rs, jjtgl_garland.rs
- jjrq_build_garlanded_silks — used by jjro_ops.rs, jjtgl_garland.rs
- jjrq_build_continuation_silks — used by jjro_ops.rs, jjtgl_garland.rs
- zjjrq_Retire* types — used by test file jjtq_query.rs

## Deliverables

1. Remove all dead jjrq_run_* functions and their Args structs
2. Keep the 4 utility functions + retire types for tests
3. Verify build: cargo build succeeds
4. Verify tests: cargo test succeeds

## Acceptance

jjrq_query.rs contains only actively-used code; build and tests pass.

**[260127-1119] rough**

Clean up dead code in jjrq_query.rs after CLI modularization.

## Context

The Jan 24 CLI refactor (d72e0e2b, 34e85785) created per-command modules but left dead code in jjrq_query.rs.

Investigation confirmed these are DEAD (never called):
- jjrq_MusterArgs, jjrq_run_muster (replaced by jjrmu_muster)
- jjrq_ParadeArgs, jjrq_run_parade (replaced by jjrpd_parade)
- jjrq_ScoutArgs, jjrq_run_scout (replaced by jjrsc_scout)
- jjrq_GetSpecArgs, jjrq_run_get_spec (replaced by jjrgs_get_spec)
- jjrq_GetCoronetsArgs, jjrq_run_get_coronets (replaced by jjrgc_get_coronets)
- jjrq_RetireArgs, jjrq_run_retire (replaced by jjrrt_retire)
- jjrq_SaddleArgs, jjrq_run_saddle (ALREADY REMOVED - replaced by jjrsd_saddle)

These MUST STAY (actively used):
- jjrq_resolve_default_heat — used by jjrsd_saddle.rs
- jjrq_parse_silks_sequence — used by jjro_ops.rs, jjtgl_garland.rs
- jjrq_build_garlanded_silks — used by jjro_ops.rs, jjtgl_garland.rs
- jjrq_build_continuation_silks — used by jjro_ops.rs, jjtgl_garland.rs
- zjjrq_Retire* types — used by test file jjtq_query.rs

## Deliverables

1. Remove all dead jjrq_run_* functions and their Args structs
2. Keep the 4 utility functions + retire types for tests
3. Verify build: cargo build succeeds
4. Verify tests: cargo test succeeds

## Acceptance

jjrq_query.rs contains only actively-used code; build and tests pass.

**[260127-1101] rough**

Investigate duplicate saddle implementation in jjrq_query.rs.

## Context

Found two saddle implementations:
- jjrsd_saddle.rs (jjrsd_run_saddle) — wired in jjrx_cli.rs, actively used
- jjrq_query.rs (jjrq_run_saddle) — appears to be dead code

Both contain session probe check logic. Need to verify jjrq version is truly unused before deleting.

## Deliverables

1. **Git history check**: When was jjrq_run_saddle added? Was it the original, later replaced by jjrsd?
2. **Usage check**: Grep for any callers of jjrq_run_saddle or jjrq_SaddleArgs
3. **Decision**: If confirmed dead code, delete jjrq_run_saddle and jjrq_SaddleArgs from jjrq_query.rs
4. **Verify build**: Ensure cargo build succeeds after removal

## Acceptance

Either: dead code removed and build passes, OR: documented reason why code must stay.

### rename-tack-commit-to-basis (₢AHAAK) [complete]

**[260127-1123] complete**

Rename tack's `commit` field to `basis` for semantic clarity.

## Context

The tack `commit` field records the commit SHA that was HEAD when the tack was created — NOT the commit that contains the tack (which is impossible due to hash circularity). The name "commit" was confusing.

## Changes

1. Rename Rust field from `commit` to `basis`
2. Keep JSON field as `"commit"` via `#[serde(rename = "commit")]` — schema unchanged
3. Rename constant `JJRG_UNKNOWN_COMMIT` to `JJRG_UNKNOWN_BASIS`
4. Update display output to show `(basis: ...)` instead of just the hash
5. Update all code references and tests

## Acceptance

- Build passes
- All 269 tests pass
- Parade output shows `(basis: <hash>)` format
- JSON files unchanged (still use `"commit"` key)

**[260127-1119] rough**

Rename tack's `commit` field to `basis` for semantic clarity.

## Context

The tack `commit` field records the commit SHA that was HEAD when the tack was created — NOT the commit that contains the tack (which is impossible due to hash circularity). The name "commit" was confusing.

## Changes

1. Rename Rust field from `commit` to `basis`
2. Keep JSON field as `"commit"` via `#[serde(rename = "commit")]` — schema unchanged
3. Rename constant `JJRG_UNKNOWN_COMMIT` to `JJRG_UNKNOWN_BASIS`
4. Update display output to show `(basis: ...)` instead of just the hash
5. Update all code references and tests

## Acceptance

- Build passes
- All 269 tests pass
- Parade output shows `(basis: <hash>)` format
- JSON files unchanged (still use `"commit"` key)

### add-vow-clean-command (₢AHAAL) [complete]

**[260209-0526] complete**

Add vow-c.Clean.sh tabtarget for complete build artifact cleanup.

## Context

Parallel Claude sessions can corrupt cargo build caches, causing "could not write output" errors. A dedicated clean command provides a manual recovery path.

## Deliverables

1. Create `tt/vow-c.Clean.sh` tabtarget
2. Clean all three crates: vok, vvc, jjk
3. Remove target directories completely (not just cargo clean)
4. Report what was cleaned

## Implementation

Pattern after existing vow-b.Build.sh colophon routing. The clean should:
- `rm -rf Tools/vok/target`
- `rm -rf Tools/vvc/target`  
- `rm -rf Tools/jjk/vov_veiled/target`
- Report total space reclaimed

## Acceptance

Running `tt/vow-c.Clean.sh` removes all build artifacts; subsequent `tt/vow-b.Build.sh` does full rebuild.

**[260127-1131] rough**

Add vow-c.Clean.sh tabtarget for complete build artifact cleanup.

## Context

Parallel Claude sessions can corrupt cargo build caches, causing "could not write output" errors. A dedicated clean command provides a manual recovery path.

## Deliverables

1. Create `tt/vow-c.Clean.sh` tabtarget
2. Clean all three crates: vok, vvc, jjk
3. Remove target directories completely (not just cargo clean)
4. Report what was cleaned

## Implementation

Pattern after existing vow-b.Build.sh colophon routing. The clean should:
- `rm -rf Tools/vok/target`
- `rm -rf Tools/vvc/target`  
- `rm -rf Tools/jjk/vov_veiled/target`
- Report total space reclaimed

## Acceptance

Running `tt/vow-c.Clean.sh` removes all build artifacts; subsequent `tt/vow-b.Build.sh` does full rebuild.

### test-session-probe (₢AHAAN) [complete]

**[260127-1209] complete**

Test session probe by temporarily forcing it to always fire.

## Context

Session probe only fires when gap > 1 hour. To verify the new saddle-based implementation works correctly, we need to test it without waiting an hour.

## Steps

1. Temporarily modify `jjrc_needs_session_probe` to always return true
2. Run `/jjc-heat-mount` and verify session probe commit is created with correct format:
   - Subject: `jjb:XXXX:₣XX:s: YYMMDD-HHMM session` (lowercase s, single "session")
   - Body: haiku/sonnet/opus model IDs, host, platform
3. Restore original logic (gap > 1 hour check)
4. Verify build + tests pass

## Acceptance

- Session probe commit created with correct format (no "session session" duplication)
- Body contains model probe results
- Original time-gap logic restored

**[260127-1153] rough**

Test session probe by temporarily forcing it to always fire.

## Context

Session probe only fires when gap > 1 hour. To verify the new saddle-based implementation works correctly, we need to test it without waiting an hour.

## Steps

1. Temporarily modify `jjrc_needs_session_probe` to always return true
2. Run `/jjc-heat-mount` and verify session probe commit is created with correct format:
   - Subject: `jjb:XXXX:₣XX:s: YYMMDD-HHMM session` (lowercase s, single "session")
   - Body: haiku/sonnet/opus model IDs, host, platform
3. Restore original logic (gap > 1 hour check)
4. Verify build + tests pass

## Acceptance

- Session probe commit created with correct format (no "session session" duplication)
- Body contains model probe results
- Original time-gap logic restored

### marker-code-registry (₢AHAAO) [complete]

**[260209-0547] complete**

Add a marker code registry that validates uniqueness at initialization.

## Context

Currently `jjrn_ChalkMarker` and `jjrn_HeatAction` each have inline char codes in their `jjrn_code()` methods. The S/s distinction (Slate vs Session) is easy to miss. We want a registry pattern that catches collisions.

## Design

Create a `jjrnm_MarkerRegistry` (marker module) with:

1. **Code constants** — all marker chars defined in one place:
   ```rust
   pub const JJRNM_APPROACH: char = 'A';
   pub const JJRNM_WRAP: char = 'W';
   pub const JJRNM_SLATE: char = 'S';
   pub const JJRNM_SESSION: char = 's';
   // etc.
   ```

2. **Registration struct** — mini object that registers on construction:
   ```rust
   pub struct jjrnm_MarkerCode {
       code: char,
       name: &'static str,
   }
   
   impl jjrnm_MarkerCode {
       pub const fn new(code: char, name: &'static str) -> Self {
           Self { code, name }
       }
   }
   ```

3. **Static registry** — collects all codes, validates uniqueness:
   ```rust
   // Using inventory crate or lazy_static with a validation function
   // Panics at init if any two codes collide
   ```

4. **Validation** — either:
   - Runtime: `lazy_static` init that panics on collision
   - Test: `#[test]` that iterates all markers and checks uniqueness
   - Compile-time: const fn if feasible (preferred)

## Deliverables

1. New file `jjrnm_markers.rs` with constants and registry
2. Update `jjrn_notch.rs` to use constants from registry
3. Test that validates no collisions exist
4. Bonus: test that all codes are printable ASCII and case-distinct per category

## Acceptance

- All marker codes defined in one file
- Collision between any two codes causes test failure or init panic
- Existing functionality unchanged

**[260127-1155] rough**

Add a marker code registry that validates uniqueness at initialization.

## Context

Currently `jjrn_ChalkMarker` and `jjrn_HeatAction` each have inline char codes in their `jjrn_code()` methods. The S/s distinction (Slate vs Session) is easy to miss. We want a registry pattern that catches collisions.

## Design

Create a `jjrnm_MarkerRegistry` (marker module) with:

1. **Code constants** — all marker chars defined in one place:
   ```rust
   pub const JJRNM_APPROACH: char = 'A';
   pub const JJRNM_WRAP: char = 'W';
   pub const JJRNM_SLATE: char = 'S';
   pub const JJRNM_SESSION: char = 's';
   // etc.
   ```

2. **Registration struct** — mini object that registers on construction:
   ```rust
   pub struct jjrnm_MarkerCode {
       code: char,
       name: &'static str,
   }
   
   impl jjrnm_MarkerCode {
       pub const fn new(code: char, name: &'static str) -> Self {
           Self { code, name }
       }
   }
   ```

3. **Static registry** — collects all codes, validates uniqueness:
   ```rust
   // Using inventory crate or lazy_static with a validation function
   // Panics at init if any two codes collide
   ```

4. **Validation** — either:
   - Runtime: `lazy_static` init that panics on collision
   - Test: `#[test]` that iterates all markers and checks uniqueness
   - Compile-time: const fn if feasible (preferred)

## Deliverables

1. New file `jjrnm_markers.rs` with constants and registry
2. Update `jjrn_notch.rs` to use constants from registry
3. Test that validates no collisions exist
4. Bonus: test that all codes are printable ASCII and case-distinct per category

## Acceptance

- All marker codes defined in one file
- Collision between any two codes causes test failure or init panic
- Existing functionality unchanged

### fix-jjk-release-directory (₢AHAAP) [complete]

**[260209-0615] complete**

Fix .jjk release directory location and gitignore setup:

**Problem:** The .jjk directory is currently at `../.jjk` (parent of repo) instead of in the repo root.

**Required changes:**
1. Configure JJK to use `.jjk/` in the repo root (not `../.jjk/`)
2. Create `.jjk/.gitignore` that ignores `parcels/` contents but keeps the directory structure tracked
3. Ensure `.jjk/` directory itself is tracked in git

**Acceptance criteria:**
- `.jjk/` exists in repo root
- `.jjk/.gitignore` contains appropriate rules to ignore parcel contents
- `git ls-files .jjk/` shows the .gitignore (and any other config files)
- `parcels/` directory contents are not tracked

**[260127-1213] rough**

Fix .jjk release directory location and gitignore setup:

**Problem:** The .jjk directory is currently at `../.jjk` (parent of repo) instead of in the repo root.

**Required changes:**
1. Configure JJK to use `.jjk/` in the repo root (not `../.jjk/`)
2. Create `.jjk/.gitignore` that ignores `parcels/` contents but keeps the directory structure tracked
3. Ensure `.jjk/` directory itself is tracked in git

**Acceptance criteria:**
- `.jjk/` exists in repo root
- `.jjk/.gitignore` contains appropriate rules to ignore parcel contents
- `git ls-files .jjk/` shows the .gitignore (and any other config files)
- `parcels/` directory contents are not tracked

### wrap-coda-next-pace-hint (₢AHAAW) [complete]

**[260208-1406] complete**

Enhance the wrap coda output (jjrwp_wrap.rs) to include the next actionable pace's silks and one-line description in the recommendation message.

Current output:
  Recommended: /clear then /jjc-heat-mount AR

Desired output:
  Recommended: /clear then /jjc-heat-mount AR for pace the-next-pace-silks (first line of pace spec)

If no next actionable pace exists (heat complete), output something like:
  Heat AR complete — no remaining paces.

Implementation:
- After the chalk commit succeeds (line 284), use the already-modified `gallops` (in scope from line 247) to find the parent heat via `coronet.jjrf_parent_firemark()`
- Iterate `heat.order` looking for first pace with state Rough or Bridled (same pattern as saddle lines 172-201)
- Extract that pace's silks and first line of its tack text
- Format the enhanced recommendation message

**[260206-2126] rough**

Enhance the wrap coda output (jjrwp_wrap.rs) to include the next actionable pace's silks and one-line description in the recommendation message.

Current output:
  Recommended: /clear then /jjc-heat-mount AR

Desired output:
  Recommended: /clear then /jjc-heat-mount AR for pace the-next-pace-silks (first line of pace spec)

If no next actionable pace exists (heat complete), output something like:
  Heat AR complete — no remaining paces.

Implementation:
- After the chalk commit succeeds (line 284), use the already-modified `gallops` (in scope from line 247) to find the parent heat via `coronet.jjrf_parent_firemark()`
- Iterate `heat.order` looking for first pace with state Rough or Bridled (same pattern as saddle lines 172-201)
- Extract that pace's silks and first line of its tack text
- Format the enhanced recommendation message

### add-vosr-init-env-gate (₢AHAAY) [complete]

**[260208-1433] complete**

Add vosr_init() to vvx main entry point — a panic-based environment gate that runs unconditionally before any argument parsing or library use.

## What

Add a `vosr_init()` function to `Tools/vok/src/vorm_main.rs` that checks `CLAUDE_CODE_DISABLE_AUTO_MEMORY == "1"` and panics if not set. Call it as the first line of `main()`, before `Cli::parse()`.

## Implementation

**vorm_main.rs** (2 changes):
1. Add function:
```rust
fn vosr_init() {
    if std::env::var("CLAUDE_CODE_DISABLE_AUTO_MEMORY").as_deref() != Ok("1") {
        panic!("CLAUDE_CODE_DISABLE_AUTO_MEMORY must be 1");
    }
}
```
2. Call at top of main():
```rust
async fn main() -> ExitCode {
    vosr_init();
    let cli = Cli::parse();
```

**VOS0-VoxObscuraSpec.adoc** (2 additions):
1. Add to mapping section: `:vosr_init: <<vosr_init,init routine>>`
2. Add definition in Routines section with `include::VOSRI-init.adoc[]`

**VOSRI-init.adoc** (new file):
Routine spec: unconditional call from main(), checks env var, panics on failure.

## Key decisions
- panic! not Result — cannot be silently ignored by future edits
- Before Cli::parse() — no command can bypass the gate
- Single env var check — only CLAUDE_CODE_DISABLE_AUTO_MEMORY for now

## Files touched
- Tools/vok/src/vorm_main.rs
- Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc
- Tools/vok/vov_veiled/VOSRI-init.adoc (new)

## Verify
- tt/vow-b.Build.sh succeeds
- tt/vvw-r.RunVVX.sh jjx_list panics without env var set
- tt/vvw-r.RunVVX.sh jjx_list works with CLAUDE_CODE_DISABLE_AUTO_MEMORY=1

**[260208-1427] bridled**

Add vosr_init() to vvx main entry point — a panic-based environment gate that runs unconditionally before any argument parsing or library use.

## What

Add a `vosr_init()` function to `Tools/vok/src/vorm_main.rs` that checks `CLAUDE_CODE_DISABLE_AUTO_MEMORY == "1"` and panics if not set. Call it as the first line of `main()`, before `Cli::parse()`.

## Implementation

**vorm_main.rs** (2 changes):
1. Add function:
```rust
fn vosr_init() {
    if std::env::var("CLAUDE_CODE_DISABLE_AUTO_MEMORY").as_deref() != Ok("1") {
        panic!("CLAUDE_CODE_DISABLE_AUTO_MEMORY must be 1");
    }
}
```
2. Call at top of main():
```rust
async fn main() -> ExitCode {
    vosr_init();
    let cli = Cli::parse();
```

**VOS0-VoxObscuraSpec.adoc** (2 additions):
1. Add to mapping section: `:vosr_init: <<vosr_init,init routine>>`
2. Add definition in Routines section with `include::VOSRI-init.adoc[]`

**VOSRI-init.adoc** (new file):
Routine spec: unconditional call from main(), checks env var, panics on failure.

## Key decisions
- panic! not Result — cannot be silently ignored by future edits
- Before Cli::parse() — no command can bypass the gate
- Single env var check — only CLAUDE_CODE_DISABLE_AUTO_MEMORY for now

## Files touched
- Tools/vok/src/vorm_main.rs
- Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc
- Tools/vok/vov_veiled/VOSRI-init.adoc (new)

## Verify
- tt/vow-b.Build.sh succeeds
- tt/vvw-r.RunVVX.sh jjx_list panics without env var set
- tt/vvw-r.RunVVX.sh jjx_list works with CLAUDE_CODE_DISABLE_AUTO_MEMORY=1

*Direction:* Agent: sonnet | Cardinality: 1 sequential | Files: Tools/vok/src/vorm_main.rs, Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc, Tools/vok/vov_veiled/VOSRI-init.adoc new (3 files) | Steps: 1. Add vosr_init fn to vorm_main.rs that panics if CLAUDE_CODE_DISABLE_AUTO_MEMORY != 1 2. Call vosr_init as first line of main before Cli::parse 3. Add vosr_init mapping and routine definition to VOS spec following vosr_probe pattern 4. Create VOSRI-init.adoc routine spec following VOSRP-probe.adoc structure 5. Build with tt/vow-b.Build.sh | Verify: tt/vow-b.Build.sh

**[260208-1425] rough**

Add vosr_init() to vvx main entry point — a panic-based environment gate that runs unconditionally before any argument parsing or library use.

## What

Add a `vosr_init()` function to `Tools/vok/src/vorm_main.rs` that checks `CLAUDE_CODE_DISABLE_AUTO_MEMORY == "1"` and panics if not set. Call it as the first line of `main()`, before `Cli::parse()`.

## Implementation

**vorm_main.rs** (2 changes):
1. Add function:
```rust
fn vosr_init() {
    if std::env::var("CLAUDE_CODE_DISABLE_AUTO_MEMORY").as_deref() != Ok("1") {
        panic!("CLAUDE_CODE_DISABLE_AUTO_MEMORY must be 1");
    }
}
```
2. Call at top of main():
```rust
async fn main() -> ExitCode {
    vosr_init();
    let cli = Cli::parse();
```

**VOS0-VoxObscuraSpec.adoc** (2 additions):
1. Add to mapping section: `:vosr_init: <<vosr_init,init routine>>`
2. Add definition in Routines section with `include::VOSRI-init.adoc[]`

**VOSRI-init.adoc** (new file):
Routine spec: unconditional call from main(), checks env var, panics on failure.

## Key decisions
- panic! not Result — cannot be silently ignored by future edits
- Before Cli::parse() — no command can bypass the gate
- Single env var check — only CLAUDE_CODE_DISABLE_AUTO_MEMORY for now

## Files touched
- Tools/vok/src/vorm_main.rs
- Tools/vok/vov_veiled/VOS0-VoxObscuraSpec.adoc
- Tools/vok/vov_veiled/VOSRI-init.adoc (new)

## Verify
- tt/vow-b.Build.sh succeeds
- tt/vvw-r.RunVVX.sh jjx_list panics without env var set
- tt/vvw-r.RunVVX.sh jjx_list works with CLAUDE_CODE_DISABLE_AUTO_MEMORY=1

### jjk-spook-paddocks-curry (₢AHAAZ) [complete]

**[260210-0245] complete**

Remove the dysfunctional verb flags (--refine, --level, --muck) from jjx_paddock/jjx_curry.

These three mutually-exclusive flags have zero behavioral effect — the code path is identical regardless of which flag is passed. They exist only to annotate the chalk commit message with a subcategory string like "paddock curried (refine)". This is metadata on metadata for a heat-level commit.

The flags are an agent trap: the CLI reference in CLAUDE.md doesn't document them, so agents guess wrong and fail. The --help text has no descriptions for --level and --muck. The taxonomy (manual vs cross-heat vs reduction) is subjective and agents will always get it wrong.

Changes:
1. Remove --refine, --level, --muck from jjrcu_CurryArgs and jjrcu_CurryVerb enum
2. Simplify jjrcu_run_curry to not require a verb in setter mode
3. Update chalk message to just "paddock curried" (or use --note if caller wants annotation)
4. Update jjtcu_curry.rs tests to match
5. Update CLAUDE.md JJK managed insert: fix jjx_paddock entry in CLI reference to show setter requires stdin only (no verb flags)
6. Build and test

Key files:
- Tools/jjk/vov_veiled/src/jjrcu_curry.rs
- Tools/jjk/vov_veiled/src/jjtcu_curry.rs
- Tools/jjk/vov_veiled/src/jjro_ops.rs (jjrg_curry fn signature — remove verb param)
- CLAUDE.md (JJK managed insert, CLI reference)

**[260209-0610] rough**

Remove the dysfunctional verb flags (--refine, --level, --muck) from jjx_paddock/jjx_curry.

These three mutually-exclusive flags have zero behavioral effect — the code path is identical regardless of which flag is passed. They exist only to annotate the chalk commit message with a subcategory string like "paddock curried (refine)". This is metadata on metadata for a heat-level commit.

The flags are an agent trap: the CLI reference in CLAUDE.md doesn't document them, so agents guess wrong and fail. The --help text has no descriptions for --level and --muck. The taxonomy (manual vs cross-heat vs reduction) is subjective and agents will always get it wrong.

Changes:
1. Remove --refine, --level, --muck from jjrcu_CurryArgs and jjrcu_CurryVerb enum
2. Simplify jjrcu_run_curry to not require a verb in setter mode
3. Update chalk message to just "paddock curried" (or use --note if caller wants annotation)
4. Update jjtcu_curry.rs tests to match
5. Update CLAUDE.md JJK managed insert: fix jjx_paddock entry in CLI reference to show setter requires stdin only (no verb flags)
6. Build and test

Key files:
- Tools/jjk/vov_veiled/src/jjrcu_curry.rs
- Tools/jjk/vov_veiled/src/jjtcu_curry.rs
- Tools/jjk/vov_veiled/src/jjro_ops.rs (jjrg_curry fn signature — remove verb param)
- CLAUDE.md (JJK managed insert, CLI reference)

### design-richer-wrap-commit-messages (₢AHAAa) [complete]

**[260210-0301] complete**

Wrap commit messages currently say only "pace complete" — no summary of what was accomplished.

## Problem
Every `:W:` commit has the identical message "pace complete", which provides no useful information when reviewing git history. Example from 2026-02-08: 30 wraps all saying "pace complete".

## Inspiration
Could we generate a whole-pace summary at wrap time? Challenges:
- The session performing the wrap may not have full context of all work done during the pace
- However, pace commit history IS available (jjx_show --detail shows all pace commits)
- The docket text describes what was intended
- Combined, these could produce a meaningful summary

## Design Questions
1. Should the wrap message be auto-generated from pace commit history + docket?
2. Should jjx_close accept an optional summary via stdin or flag?
3. What's the right balance between automation and manual summary?
4. How to handle multi-session paces where the wrapping session lacks full context?

## Scope
Design decision only — determine approach, then slate implementation paces.

**[260209-0755] rough**

Wrap commit messages currently say only "pace complete" — no summary of what was accomplished.

## Problem
Every `:W:` commit has the identical message "pace complete", which provides no useful information when reviewing git history. Example from 2026-02-08: 30 wraps all saying "pace complete".

## Inspiration
Could we generate a whole-pace summary at wrap time? Challenges:
- The session performing the wrap may not have full context of all work done during the pace
- However, pace commit history IS available (jjx_show --detail shows all pace commits)
- The docket text describes what was intended
- Combined, these could produce a meaningful summary

## Design Questions
1. Should the wrap message be auto-generated from pace commit history + docket?
2. Should jjx_close accept an optional summary via stdin or flag?
3. What's the right balance between automation and manual summary?
4. How to handle multi-session paces where the wrapping session lacks full context?

## Scope
Design decision only — determine approach, then slate implementation paces.

### permit-uppercase-silks (₢AHAAd) [complete]

**[260210-1312] complete**

Drafted from ₢AGAAN in ₣AG.

Relax silks validation to permit uppercase letters (A-Z) in addition to lowercase.

Continue rejecting spaces and underscores — only allow [a-zA-Z0-9] and hyphens.

Current pattern: `[a-z0-9]+(-[a-z0-9]+)*`
Target pattern: `[a-zA-Z0-9]+(-[a-zA-Z0-9]+)*`

Locate silks validation in jjk/vvx and update the regex plus any related error messages or tests.

**[260210-1124] rough**

Drafted from ₢AGAAN in ₣AG.

Relax silks validation to permit uppercase letters (A-Z) in addition to lowercase.

Continue rejecting spaces and underscores — only allow [a-zA-Z0-9] and hyphens.

Current pattern: `[a-z0-9]+(-[a-z0-9]+)*`
Target pattern: `[a-zA-Z0-9]+(-[a-zA-Z0-9]+)*`

Locate silks validation in jjk/vvx and update the regex plus any related error messages or tests.

**[260208-1545] rough**

Relax silks validation to permit uppercase letters (A-Z) in addition to lowercase.

Continue rejecting spaces and underscores — only allow [a-zA-Z0-9] and hyphens.

Current pattern: `[a-z0-9]+(-[a-z0-9]+)*`
Target pattern: `[a-zA-Z0-9]+(-[a-zA-Z0-9]+)*`

Locate silks validation in jjk/vvx and update the regex plus any related error messages or tests.

### quiet-slate-output (₢AHAAe) [complete]

**[260210-1455] complete**

Remove three noisy eprintln! lines from slate/enroll output:

1. `Tools/vvc/src/vvcm_machine.rs:98` — `eprintln!("vvcm_commit: staged {} file(s)", files.len());`
2. `Tools/vvc/src/vvcm_machine.rs:134` — `eprintln!("vvcm_commit: committed {}", &hash[..8.min(hash.len())]);`
3. `Tools/jjk/vov_veiled/src/jjrsl_slate.rs:88` — `eprintln!("jjx_slate: committed {}", &hash[..8]);`

Delete the three eprintln! lines. The coronet line (stdout) is the only useful output from enroll/slate.

Keep the WARNING line at vvcm_machine.rs:76 — that one is useful.

Build after: `tt/vow-b.Build.sh`

**[260210-1237] rough**

Remove three noisy eprintln! lines from slate/enroll output:

1. `Tools/vvc/src/vvcm_machine.rs:98` — `eprintln!("vvcm_commit: staged {} file(s)", files.len());`
2. `Tools/vvc/src/vvcm_machine.rs:134` — `eprintln!("vvcm_commit: committed {}", &hash[..8.min(hash.len())]);`
3. `Tools/jjk/vov_veiled/src/jjrsl_slate.rs:88` — `eprintln!("jjx_slate: committed {}", &hash[..8]);`

Delete the three eprintln! lines. The coronet line (stdout) is the only useful output from enroll/slate.

Keep the WARNING line at vvcm_machine.rs:76 — that one is useful.

Build after: `tt/vow-b.Build.sh`

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 b orient-bitmap-displays
  2 c spook-alter-status-flag
  3 B vos-add-routines-section
  4 D jjsa-session-probe-update
  5 E impl-vosr-probe
  6 F wire-saddle-vosr-probe
  7 I saddle-creates-session-chalk
  8 G remove-slash-cmd-session-probe
  9 R mount-auto-saddle-first-racing
  10 T heat-file-touch-visualization
  11 U pace-commit-timeline
  12 V explore-parade-flag-reduction
  13 Q fix-notch-deleted-files
  14 H doc-update-tokio-probe
  15 J cleanup-jjrq-dead-code
  16 K rename-tack-commit-to-basis
  17 L add-vow-clean-command
  18 N test-session-probe
  19 O marker-code-registry
  20 P fix-jjk-release-directory
  21 W wrap-coda-next-pace-hint
  22 Y add-vosr-init-env-gate
  23 Z jjk-spook-paddocks-curry
  24 a design-richer-wrap-commit-messages
  25 d permit-uppercase-silks
  26 e quiet-slate-output

bcBDEFIGRTUVQHJKLNOPWYZade
·········xx····x·········· jjrpd_parade.rs
········xx···x············ JJSCSD-saddle.adoc
·····xx·x················· jjrsd_saddle.rs
····xx···············x···· vorm_main.rs
······················x·x· jjro_ops.rs
···············x········x· jjrv_validate.rs, jjtfu_furlough.rs, jjtg_gallops.rs
·············x··········x· JJSA-GallopsData.adoc
·········x·····x·········· jjrq_query.rs
·········xx··············· JJSCPD-parade.adoc
·······xx················· jjc-heat-mount.md
····x·············x······· lib.rs
····x············x········ vvcp_probe.rs
··x··················x···· VOS-VoxObscuraSpec.adoc
··x·x····················· VOSRP-probe.adoc
·x·····················x·· CLAUDE.md
·························x jjrsl_slate.rs, vvcm_machine.rs
·······················x·· JJSRWP-wrap.adoc, jjrwp_wrap.rs
······················x··· jjrcu_curry.rs, jjtcu_curry.rs
·····················x···· VOSRI-init.adoc
··················x······· jjrn_notch.rs, jjrnm_markers.rs, jjtnm_markers.rs
·················x········ jjrc_core.rs
················x········· vob_build.sh, vow-c.Clean.sh, vow_workbench.sh
···············x·········· jjrt_types.rs, jjru_util.rs, jjtgl_garland.rs, jjtpd_parade.rs, jjtq_query.rs, jjtrs_restring.rs
············x············· JJSCNC-notch.adoc, jjrnc_notch.rs
·····x···················· jjrx_cli.rs
····x····················· Cargo.lock, Cargo.toml
··x······················· VOSRC-commit.adoc, VOSRG-guard.adoc, VOSRL-lock.adoc

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 174 commits)

  1 b orient-bitmap-displays
  2 c spook-alter-status-flag
  3 Z jjk-spook-paddocks-curry
  4 a design-richer-wrap-commit-messages
  5 d permit-uppercase-silks
  6 e quiet-slate-output

123456789abcdefghijklmnopqrstuvwxyz
···x·xxxx··························  b  5c
·········xxxxx·····················  c  5c
···············xxxxx···············  Z  5c
····················xxxxx··········  a  5c
····························xxx····  d  3c
·······························xxx·  e  3c
```

## Steeplechase

### 2026-02-11 07:37 - Heat - f

stabled

### 2026-02-10 14:55 - ₢AHAAe - W

Removed 3 noisy eprintln lines from vvcm_machine.rs and jjrsl_slate.rs

### 2026-02-10 14:55 - ₢AHAAe - n

Remove 3 noisy eprintln lines from vvcm_machine.rs and jjrsl_slate.rs, keeping WARNING line

### 2026-02-10 14:54 - ₢AHAAe - A

Delete 3 noisy eprintln lines in vvcm_machine.rs and jjrsl_slate.rs, build to verify

### 2026-02-10 13:12 - ₢AHAAd - W

Widened silks validation to accept uppercase [a-zA-Z0-9], updated error messages in ops/validate, tests in gallops/furlough, and JJSA spec pattern

### 2026-02-10 13:12 - ₢AHAAd - n

Widen silks validation to accept uppercase letters [a-zA-Z0-9], update error messages, tests, and JJSA spec

### 2026-02-10 13:06 - ₢AHAAd - A

Widen zjjrg_is_kebab_case to accept uppercase, update error msgs and tests in jjrv_validate/jjtg_gallops/jjtfu_furlough

### 2026-02-10 13:02 - Heat - r

moved AHAAX after AHAAd

### 2026-02-10 12:37 - Heat - S

quiet-slate-output

### 2026-02-10 11:24 - Heat - D

restring 1 paces from ₣AG

### 2026-02-10 03:01 - ₢AHAAa - W

Richer wrap commit messages: jjx_close now accepts optional stdin summary for chalk description, falls back to 'pace {silks} complete' instead of generic 'pace complete'

### 2026-02-10 03:01 - ₢AHAAa - n

Read optional stdin summary in wrap, use as chalk description; fallback to pace silks complete

### 2026-02-10 02:52 - ₢AHAAa - L

sonnet landed

### 2026-02-10 02:49 - ₢AHAAa - F

Executing via sonnet agent

### 2026-02-10 02:49 - ₢AHAAa - A

Read optional stdin summary in wrap, use as chalk description; fallback to pace silks complete

### 2026-02-10 02:45 - ₢AHAAZ - W

pace complete

### 2026-02-10 02:45 - ₢AHAAZ - n

Remove CurryVerb enum and verb flags from curry command, simplify chalk message to paddock curried with optional note

### 2026-02-10 02:45 - ₢AHAAZ - L

sonnet landed

### 2026-02-10 02:42 - ₢AHAAZ - F

Executing via sonnet agent

### 2026-02-10 02:42 - ₢AHAAZ - A

Remove CurryVerb enum and verb flags from curry command, simplify chalk message to paddock curried with optional note

### 2026-02-10 02:40 - Heat - r

moved AHAAX to last

### 2026-02-10 02:35 - ₢AHAAc - W

pace complete

### 2026-02-10 02:35 - ₢AHAAc - n

Fix CLAUDE.md jjx_alter CLI reference: change --status racing|stabled to --racing|--stabled boolean flags

### 2026-02-10 02:33 - ₢AHAAc - L

haiku landed

### 2026-02-10 02:33 - ₢AHAAc - F

Executing via haiku agent directly

### 2026-02-10 02:23 - ₢AHAAc - A

Fix CLAUDE.md jjx_alter CLI reference: change --status racing|stabled to --racing|--stabled boolean flags

### 2026-02-10 02:23 - ₢AHAAb - W

pace complete

### 2026-02-10 02:06 - ₢AHAAb - L

sonnet landed

### 2026-02-10 02:02 - ₢AHAAb - F

Executing bridled pace via sonnet agent

### 2026-02-10 02:02 - ₢AHAAb - B

arm | orient-bitmap-displays

### 2026-02-10 02:02 - Heat - T

orient-bitmap-displays

### 2026-02-10 01:58 - ₢AHAAb - A

Wire existing bitmap/swimlane rendering from parade into saddle: make zjjrpd functions pub(crate), call from saddle after Recent-work, update spec

### 2026-02-09 11:57 - Heat - n

heat plan status update

### 2026-02-09 11:55 - Heat - S

spook-alter-status-flag

### 2026-02-09 11:53 - Heat - f

racing

### 2026-02-09 11:53 - Heat - S

orient-bitmap-displays

### 2026-02-09 07:55 - Heat - S

design-richer-wrap-commit-messages

### 2026-02-09 06:25 - Heat - d

furlough AH to jjk-nonbreaking-cleanups (stabled)

### 2026-02-09 06:15 - ₢AHAAP - W

pace complete

### 2026-02-09 06:12 - ₢AHAAP - A

Fix parcels_dir in vob_build.sh to use repo root, create .jjk/.gitignore, verify git tracking

### 2026-02-09 06:10 - Heat - S

jjk-spook-paddocks-curry

### 2026-02-09 05:47 - ₢AHAAO - W

pace complete

### 2026-02-09 05:46 - ₢AHAAO - n

Add jjrnm_markers.rs marker code registry with constants, update jjrn_notch.rs to use registry constants, add collision uniqueness test

### 2026-02-09 05:42 - ₢AHAAO - F

Executing bridled pace via sonnet agent

### 2026-02-09 05:41 - ₢AHAAO - A

Create jjrnm_markers.rs with code constants, collision validator; update jjrn_notch.rs to use constants; add uniqueness test

### 2026-02-09 05:26 - ₢AHAAL - W

pace complete

### 2026-02-09 05:26 - ₢AHAAL - n

Add vow-c.Clean.sh tabtarget: create launcher, add route in vow_workbench.sh, add vob_clean function in vob_build.sh removing all four kit target dirs with space reporting

### 2026-02-09 05:24 - ₢AHAAL - A

Create vow-c.Clean.sh tabtarget, add route in workbench, add vob_clean function in vob_build.sh

### 2026-02-08 14:33 - ₢AHAAY - W

pace complete

### 2026-02-08 14:33 - ₢AHAAY - n

Add vosr_init panic-based environment gate requiring CLAUDE_CODE_DISABLE_AUTO_MEMORY=1 before any vvx operation, with VOS spec and routine documentation

### 2026-02-08 14:27 - ₢AHAAL - A

Add vow-c tabtarget: create tt/vow-c.Clean.sh, add route in vow_workbench.sh, add vob_clean function in vob_build.sh using rm -rf on three target dirs with space reporting

### 2026-02-08 14:27 - ₢AHAAY - B

arm | add-vosr-init-env-gate

### 2026-02-08 14:27 - Heat - T

add-vosr-init-env-gate

### 2026-02-08 14:25 - Heat - S

add-vosr-init-env-gate

### 2026-02-08 14:24 - ₢AHAAH - W

pace complete

### 2026-02-08 14:24 - ₢AHAAH - n

Update saddle spec and gallops data model for session probe integration via vosr_probe routine

### 2026-02-08 14:21 - ₢AHAAH - F

Executing bridled pace via sonnet agent

### 2026-02-08 14:21 - Heat - T

design-model-tiering-strategy

### 2026-02-08 14:17 - ₢AHAAQ - W

pace complete

### 2026-02-08 14:17 - ₢AHAAQ - n

Allow notch to commit deleted files by accepting git-tracked paths that no longer exist on disk

### 2026-02-08 14:15 - ₢AHAAQ - F

Executing bridled pace via haiku agent

### 2026-02-08 14:07 - Heat - n

Fix wrap lookahead using jjrf_display() for heat key lookup instead of jjrf_as_str()

### 2026-02-08 14:06 - ₢AHAAW - W

pace complete

### 2026-02-08 13:21 - Heat - n

Add AGENT_RESPONSE lookahead to jjx_close: emit next pace silks after wrap

### 2026-02-07 07:36 - Heat - S

attempt-bash-permission-grant

### 2026-02-06 22:03 - ₢AHAAV - W

pace complete

### 2026-02-06 21:26 - Heat - S

wrap-coda-next-pace-hint

### 2026-02-06 20:50 - ₢AHAAV - A

Analyze parade flags and bitmap concerns; produce recommendation memo

### 2026-02-06 20:48 - ₢AHAAU - W

pace complete

### 2026-02-06 20:48 - ₢AHAAU - n

Two new bitmap functions in jjrpd_parade.rs (swimlanes + pace-commits), update JJSCPD spec

### 2026-02-06 20:41 - ₢AHAAU - A

Two new bitmap functions in jjrpd_parade.rs (swimlanes + pace-commits), update JJSCPD spec

### 2026-02-06 20:33 - Heat - T

pace-commit-timeline

### 2026-02-06 20:29 - ₢AHAAT - n

Add column-header line above bitmap rows for positional decoding without counting

### 2026-02-06 20:27 - Heat - T

pace-commit-timeline

### 2026-02-06 20:22 - Heat - T

pace-commit-timeline

### 2026-02-06 20:17 - ₢AHAAT - n

Remove --files flag; make file-touch bitmap standard in all heat parade views; update JJSCPD spec

### 2026-02-06 20:10 - ₢AHAAT - n

Add shared jjrq_files_for_pace query with infra filter, refactor bitmap to use it, add Work files to coronet view, update JJSCPD and JJSCSD specs

### 2026-02-06 19:54 - Heat - r

moved AHAAU to first

### 2026-02-06 09:49 - ₢AHAAV - A

Inventory parade flags, assess CLI design, write recommendation memo

### 2026-02-06 09:48 - Heat - T

pace-commit-timeline

### 2026-02-06 09:48 - ₢AHAAT - W

pace complete

### 2026-02-06 09:48 - ₢AHAAT - n

Add --files flag to parade: bitmap visualization of file touches per pace via git diff between tack basis SHAs

### 2026-02-06 09:43 - Heat - S

explore-parade-flag-reduction

### 2026-02-06 09:34 - ₢AHAAT - A

Add --files flag to parade: bitmap visualization of file touches per pace via git diff between tack basis SHAs

### 2026-02-06 09:29 - Heat - S

pace-commit-timeline

### 2026-02-06 09:16 - ₢AHAAR - W

pace complete

### 2026-02-06 09:16 - ₢AHAAR - n

Add racing-heats summary table to saddle output and simplify mount heat selection by delegating to saddle

### 2026-02-06 09:05 - Heat - r

moved AHAAR to first

### 2026-02-06 09:03 - Heat - r

moved AHAAT to first

### 2026-02-06 09:03 - Heat - D

restring 1 paces from ₣AG

### 2026-02-03 18:53 - Heat - S

simplify-thin-wrapper-commands

### 2026-02-03 18:45 - Heat - T

mount-auto-saddle-first-racing

### 2026-02-03 18:40 - Heat - S

mount-auto-saddle-first-racing

### 2026-01-28 08:02 - Heat - r

moved AHAAQ to first

### 2026-01-28 08:01 - ₢AHAAQ - B

tally | fix-notch-deleted-files

### 2026-01-28 08:01 - Heat - T

fix-notch-deleted-files

### 2026-01-28 08:01 - Heat - S

fix-notch-deleted-files

### 2026-01-27 12:13 - Heat - S

fix-jjk-release-directory

### 2026-01-27 12:09 - ₢AHAAN - W

pace complete

### 2026-01-27 12:09 - ₢AHAAN - n

Refactor session gap check to use seconds for clarity and fix vvcp_probe CLI args

### 2026-01-27 12:06 - Heat - s

260127-1206 session

### 2026-01-27 12:05 - Heat - s

260127-1205 session

### 2026-01-27 12:05 - Heat - s

260127-1205 session

### 2026-01-27 12:04 - Heat - s

260127-1204 session

### 2026-01-27 12:01 - Heat - s

260127-1201 session

### 2026-01-27 11:55 - Heat - S

marker-code-registry

### 2026-01-27 11:55 - Heat - s

260127-1155 session

### 2026-01-27 11:53 - Heat - S

test-session-probe

### 2026-01-27 11:46 - ₢AHAAG - W

pace complete

### 2026-01-27 11:46 - ₢AHAAG - n

jjc-heat-mount: remove session probe step, renumber subsequent steps

### 2026-01-27 11:46 - ₢AHAAG - L

haiku landed

### 2026-01-27 11:45 - Heat - S

jjx-advice-output

### 2026-01-27 11:44 - ₢AHAAG - F

Executing bridled pace via haiku agent

### 2026-01-27 11:42 - ₢AHAAI - W

pace complete

### 2026-01-27 11:42 - ₢AHAAI - n

saddle creates session chalk commit with probe result as body

### 2026-01-27 11:41 - ₢AHAAI - L

sonnet landed

### 2026-01-27 11:39 - ₢AHAAI - F

Executing bridled pace via sonnet agent

### 2026-01-27 11:38 - ₢AHAAI - B

tally | saddle-creates-session-chalk

### 2026-01-27 11:38 - Heat - T

saddle-creates-session-chalk

### 2026-01-27 11:37 - Heat - T

saddle-creates-session-chalk

### 2026-01-27 11:31 - Heat - S

add-vow-clean-command

### 2026-01-27 11:30 - ₢AHAAJ - W

pace complete

### 2026-01-27 11:23 - ₢AHAAK - W

pace complete

### 2026-01-27 11:23 - ₢AHAAK - n

Rename tack commit field to basis for semantic clarity

### 2026-01-27 11:20 - ₢AHAAK - n

Rename tack commit field to basis for semantic clarity

### 2026-01-27 11:19 - ₢AHAAG - B

tally | remove-slash-cmd-session-probe

### 2026-01-27 11:19 - Heat - T

remove-slash-cmd-session-probe

### 2026-01-27 11:19 - Heat - S

rename-tack-commit-to-basis

### 2026-01-27 11:19 - Heat - T

investigate-jjrq-saddle-duplicate

### 2026-01-27 11:01 - ₢AHAAI - B

tally | saddle-creates-session-chalk

### 2026-01-27 11:01 - Heat - T

saddle-creates-session-chalk

### 2026-01-27 11:01 - Heat - S

investigate-jjrq-saddle-duplicate

### 2026-01-27 10:58 - ₢AHAAF - W

pace complete

### 2026-01-27 10:58 - ₢AHAAF - n

Integrate async tokio probe call into saddle session detection

### 2026-01-27 10:57 - Heat - r

moved AHAAI before AHAAG

### 2026-01-27 10:57 - Heat - S

saddle-creates-session-chalk

### 2026-01-27 10:53 - ₢AHAAF - F

Executing bridled pace via sonnet agent

### 2026-01-27 10:53 - ₢AHAAH - B

tally | doc-update-tokio-probe

### 2026-01-27 10:53 - Heat - T

doc-update-tokio-probe

### 2026-01-27 10:52 - Heat - S

doc-update-tokio-probe

### 2026-01-27 10:52 - ₢AHAAE - W

pace complete

### 2026-01-27 10:52 - ₢AHAAE - n

Add vvcp_probe module for Claude Code environment discovery

### 2026-01-27 10:50 - ₢AHAAF - B

tally | wire-saddle-vosr-probe

### 2026-01-27 10:50 - Heat - T

wire-saddle-vosr-probe

### 2026-01-27 10:48 - ₢AHAAE - F

Executing bridled pace via sonnet agent

### 2026-01-27 10:46 - ₢AHAAE - B

tally | impl-vosr-probe

### 2026-01-27 10:46 - Heat - T

impl-vosr-probe

### 2026-01-25 14:25 - Heat - r

moved AHAAG before AHAAA

### 2026-01-25 14:25 - Heat - r

moved AHAAF before AHAAA

### 2026-01-25 14:25 - Heat - S

remove-slash-cmd-session-probe

### 2026-01-25 14:25 - Heat - S

wire-saddle-vosr-probe

### 2026-01-25 14:21 - ₢AHAAE - A

vosr_probe implementation in VVC with tokio async

### 2026-01-25 14:17 - ₢AHAAD - W

pace complete

### 2026-01-25 14:15 - ₢AHAAD - B

tally | jjsa-session-probe-update

### 2026-01-25 14:15 - Heat - T

jjsa-session-probe-update

### 2026-01-25 14:14 - ₢AHAAD - A

Update JJSA+saddle+mount to reference vosr_probe

### 2026-01-25 14:12 - ₢AHAAB - W

pace complete

### 2026-01-25 14:12 - ₢AHAAB - n

Add Routines section to VOS with vosr_lock, vosr_commit, vosr_guard, vosr_probe

### 2026-01-25 14:05 - Heat - T

impl-session-probe-triggers

### 2026-01-25 14:04 - Heat - r

moved AHAAE after AHAAD

### 2026-01-25 14:04 - Heat - r

moved AHAAD after AHAAB

### 2026-01-25 14:03 - Heat - S

impl-vosr-probe

### 2026-01-25 14:03 - Heat - S

jjsa-session-probe-update

### 2026-01-25 14:01 - Heat - T

define-session-probe-routine

### 2026-01-25 08:41 - Heat - T

spec-session-probe-triggers

### 2026-01-25 08:33 - ₢AHAAB - L

sonnet landed

### 2026-01-25 08:29 - ₢AHAAB - F

Executing bridled pace via sonnet agent

### 2026-01-25 08:28 - ₢AHAAB - B

tally | spec-session-probe-triggers

### 2026-01-25 08:28 - Heat - T

spec-session-probe-triggers

### 2026-01-25 08:26 - Heat - T

spec-session-probe-triggers

### 2026-01-25 08:25 - Heat - r

moved AHAAA to last

### 2026-01-25 08:23 - Heat - S

impl-session-probe-triggers

### 2026-01-25 08:23 - Heat - S

spec-session-probe-triggers

### 2026-01-25 08:17 - Heat - f

silks=jjk-cleanups-and-haiku-pilot

