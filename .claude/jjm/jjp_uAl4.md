## Character
Cross-project infrastructure with new BUK regime and JJK command surface. Design is substantially resolved through conversation; implementation is a mix of BUK plumbing (mechanical) and JJK command wiring (moderate judgment on identity lifecycle).

## Context

Spun off from ₣A2 (jjk-v3-6-minor-issues). The original paces ₢A2AAG (buf-dispatch-fact-files) and ₢A2AAF (remote-execution-bind-send) grew beyond "minor issues" during design iteration. The `buf_` tinder-constant approach was replaced by a proper BUK regime (BURX), and the remote execution model gained session/job identity tiers and asynchronous polled dispatch.

## Design Decision Record

### BURX Regime — Cross-Project Exchange

**Problem**: Two BUK-equipped projects need to share dispatch status and artifact locations across SSH. The existing regime family (BURC, BURS, BURD, BURE) is all single-project. Ad-hoc fact-file constants (`buf_` prefix) would invent a new contract mechanism when one already exists.

**Decision**: New BUK regime `BURX` (Bash Utility eXchange Regime). Always written by BUD during dispatch. The only regime designed to cross project boundaries.

**BURX fields written at dispatch start:**
- `BURX_PID` — process ID of the dispatch
- `BURX_BEGAN_AT` — nanosecond-precision timestamp (e.g., `20260403-102137.482951000`)
- `BURX_TABTARGET` — the tabtarget that was dispatched
- `BURX_TEMP_DIR` — absolute path to the dispatch temp directory
- `BURX_TRANSCRIPT` — absolute path to the transcript directory
- `BURX_LOG_HIST` — absolute path to the historical log file
- `BURX_PENSUM` — pensum token from curia (passed via BURE_PENSUM; empty for local dispatch)

**BURX fields written at dispatch completion only:**
- `BURX_STATUS` — unix exit code (absence of this field = still running)
- `BURX_ENDED_AT` — nanosecond-precision timestamp

**Key properties:**
- Absence of `BURX_STATUS` means the process is still running — no explicit "running" value
- `burx.env` is a fact-file, written via the dual-write fact-file primitive
- BUD always writes it, whether anyone reads it remotely or not
- BURX is a projection of BURD state across the project boundary

### Dual-Write Fact-File Module

**Problem**: Fact-files currently write only to `BURD_OUTPUT_DIR`, which is cleared on next dispatch. Remote consumers need durable copies addressable by temp dir path.

**Decision**: New BUK module providing a fact-file write primitive (e.g., `buf_write_fact`) that writes every fact-file to both `BURD_OUTPUT_DIR` and `BURD_TEMP_DIR`. All fact-file producers must use this primitive. No symlinks, no directory restructuring.

- `BURD_OUTPUT_DIR` — latest-command convenience for single-threaded local use
- `BURD_TEMP_DIR` — durable copy, addressable by anyone who captured the path
- Fact-files are never large; dual-write cost is trivial
- Requires BUS0 quoin treatment for the module and primitive

### Two-Tier Identity: Legatio and Pensum

**Legatio** (session): A standing connection to a remote project. Created by `jjx_bind`, identified by a legatio token. Contains host, user, directory. Multiple legationes can coexist within an officium (different hosts, or same host different projects). Persists until explicit unbind or officium absolution.

**Pensum** (job): A single dispatched async operation within a legatio. Created by `jjx_relay`, identified by a pensum token. The pensum stores which legatio it was dispatched through. Multiple pensa can coexist. Each pensum on the curia maps to a remote temp dir path on the locus.

**Pensum flows curia → locus via BURE**: `jjx_relay` mints the pensum on the curia, sets `BURE_PENSUM=<token>` in the SSH environment. BUD writes it into `burx.env`. Both sides can correlate.

### Curia and Locus — Role Vocabulary

Two sides of every remote operation need distinct quoin-level names:

- **Curia** — the invoking side. Where the officium lives, where JJK runs, where the LLM works. The administrative court from which legationes are sent.
- **Locus** — the executing side. Where BUD runs, where BURX is written, where the tabtarget physically executes. The place of work.

### Command Surface

| Command | Takes | Returns | Mode |
|---------|-------|---------|------|
| `jjx_bind` | `{host, user, directory}` | legatio token | Creates session, SSH probe at bind time |
| `jjx_send` | `{legatio, command, timeout}` | exit code + output | Synchronous, no pensum |
| `jjx_relay` | `{legatio, tabtarget, timeout}` | pensum token | Async via nohup, mints pensum |
| `jjx_check` | `{pensum}` | BURX fields + liveness | Lightweight, pollable |
| `jjx_fetch` | `{pensum}` | All artifacts (transcript, log, facts) | Heavy, bundle-fetch, no ref param |

**`jjx_send`** is purely synchronous — uses legatio for SSH target, blocks until done, returns inline. No pensum, no BURX consumption. BUD still writes BURX on the locus (it always does), but nobody reads it remotely.

**`jjx_relay`** fires via nohup on the locus. The timeout is self-enforced remotely (the nohup wrapper kills the process after N seconds). SSH disconnects immediately after launch. Returns pensum token with remote temp dir path captured.

**`jjx_check`** is the polling primitive: SSH in, read `burx.env` from the stored temp dir path, probe `kill -0 $PID`. Reports:

| BURX_STATUS present? | PID alive? | Report |
|----------------------|------------|--------|
| no | yes | **running** |
| no | no | **orphaned** (crashed without writing terminal status) |
| yes | n/a | **stopped** (exit code = BURX_STATUS) |
| file missing | n/a | **lost** |

**`jjx_fetch`** pulls everything in one bundle: transcript dir contents, historical log, all fact-files from temp dir. No selective ref parameter — officium storage is ephemeral anyway.

### Async Dispatch Mechanism

nohup + sentinel pattern. POSIX, zero dependencies beyond standard shell.

The nohup wrapper on the locus:
1. Sets `BURE_PENSUM` in environment
2. Invokes the tabtarget (which goes through BUD dispatch)
3. BUD writes initial `burx.env` (no STATUS field = running)
4. Tabtarget runs
5. On completion: BUD exit path writes `BURX_STATUS` and `BURX_ENDED_AT`
6. On crash: `burx.env` exists with PID but no STATUS — `jjx_check` detects via `kill -0`

### Provenance

Design emerged through conversation in officium ☉260403-1021. Key evolution:
- `buf_` tinder constants → BURX regime (controlled interface, less drift)
- Single bind → legatio/pensum two-tier identity (concurrent connections and jobs)
- Synchronous-only → async polled dispatch (nohup + PID probe)
- `BURX_STATUS=running` → absence-of-field = running (cleaner)
- sedes/situs → curia/locus (first-letter uniqueness)

### Existing Fact-File Infrastructure (Codebase Survey)

The fact-file pattern and AXLA voicing machinery already exist. BURX fields and the dual-write module build on established patterns, not new ones.

**AXLA voicings (in AXLA-Lexicon.adoc):**
- `axpof_fact` — voices a term as a fact-file constant: named output file carrying a single atomic value. Used at definition sites as `//axpof_fact`.
- `axpot_tally` — voices a term as a fact-map infix: dynamically-named set of files sharing a common infix. Used at definition sites as `//axpot_tally`.
- `axd_fact` / `axd_tally` — dimension markers on `axhoot_typed_output` at operation detail sites.

**BUS0 pattern (`busff_fact_file`):**
- Producer writes to `${BURD_OUTPUT_DIR}/${CONSTANT}`
- Consumer reads from `${ZBUTO_BURV_OUTPUT}/current/${CONSTANT}`
- The constant name IS the contract between producer and consumer
- Currently single-destination only — dual-write extends this

**Existing RBK fact-files (in RBS0, constants in `rbgc_Constants.sh`):**

| Constant | Voicing | Value |
|----------|---------|-------|
| `RBF_FACT_HALLMARK` | `axpof_fact` | Hallmark string |
| `RBF_FACT_BUILD_ID` | `axpof_fact` | Cloud Build job ID |
| `RBF_FACT_GAR_ROOT` | `axpof_fact` | GAR registry root path |
| `RBF_FACT_ARK_STEM` | `axpof_fact` | Vessel:hallmark |
| `RBF_FACT_ARK_YIELD` | `axpof_fact` | Per-platform image ref |
| `RBCC_FACT_CONSEC_INFIX` | `axpot_tally` | Per-vessel-hallmark presence marker |

**Producer pattern today** (~15 instances in `rbfd_FoundryDirectorBuild.sh`, plus `rbfl_FoundryLedger.sh` for tally):
```
echo "${value}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_HALLMARK}"
```
All raw `echo > path` to `BURD_OUTPUT_DIR` only. No dual-write. The `buf_write_fact` primitive replaces every one of these.

**Implications for ₣A4 paces:**
1. Each BURX field gets a quoin in BUS0 voiced `//axpof_fact` — the voicing pattern is established.
2. The `busff_fact_file` pattern description in BUS0 needs extending for dual-write, but the consumer contract (read from known path) is unchanged.
3. Migrating ~15 raw `echo` writes in RBK to `buf_write_fact` is mechanical but touches `rbfd_FoundryDirectorBuild.sh` heavily. This is a separate pace from the BURX/JJK work — BUK infrastructure first, then RBK producer migration, then JJK command surface.

## References
- BUS0 (`Tools/buk/vov_veiled/BUS0-BashUtilitiesSpec.adoc`) — regime definitions, `busff_fact_file` pattern, existing BUR* regime family
- AXLA (`Tools/cmk/vov_veiled/AXLA-Lexicon.adoc`) — `axpof_fact`, `axpot_tally`, `axd_fact`, `axhoot_typed_output`
- RBS0 (`Tools/rbk/vov_veiled/RBS0-SpecTop.adoc`) — `rbf_fact_*` quoins, foundry fact-file section
- `Tools/rbk/rbgc_Constants.sh` — `RBF_FACT_*` constant definitions
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh` — primary fact-file producer (~15 write sites)
- `Tools/rbk/rbfl_FoundryLedger.sh` — tally fact-file producer
- `Tools/buk/bud_dispatch.sh` — dispatch runtime, BURD state assembly
- `Tools/buk/buc_command.sh` — BURE kindle
- ₣A2 paddock — original paces ₢A2AAG and ₢A2AAF
- ₣Ah paddock — officium lifecycle patterns