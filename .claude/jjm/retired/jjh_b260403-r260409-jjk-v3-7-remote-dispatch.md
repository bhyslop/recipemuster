# Heat Trophy: jjk-v3-7-remote-dispatch

**Firemark:** ₣A4
**Created:** 260403
**Retired:** 260409
**Status:** retired

## Paddock

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
- `BURX_TRANSCRIPT` — absolute path to the transcript file
- `BURX_LOG_HIST` — absolute path to the historical log file
- `BURX_LABEL` — optional correlation label from curia (passed via BURE_LABEL; empty for local dispatch). xname format, max 120 chars.

**BURX fields written at dispatch completion only:**
- `BURX_EXIT_STATUS` — unix exit code (absence of this field = still running)
- `BURX_ENDED_AT` — nanosecond-precision timestamp

**Key properties:**
- Absence of `BURX_EXIT_STATUS` means the process is still running — no explicit "running" value
- `burx.env` is a fact-file, written via the dual-write fact-file primitive
- BUD always writes it, whether anyone reads it remotely or not
- BURX is a projection of BURD state across the project boundary
- `BURX_LABEL` is JJK-decoupled — BUD writes whatever `BURE_LABEL` contains. JJK happens to put pensum tokens in it, but BURX doesn't know or care.

### Dual-Write Fact-File Module

**Problem**: Fact-files currently write only to `BURD_OUTPUT_DIR`, which is cleared on next dispatch. Remote consumers need durable copies addressable by temp dir path.

**Decision**: New BUK module providing a fact-file write primitive (e.g., `buf_write_fact`) that writes every fact-file to both `BURD_OUTPUT_DIR` and `BURD_TEMP_DIR`. All fact-file producers must use this primitive. No symlinks, no directory restructuring.

- `BURD_OUTPUT_DIR` — latest-command convenience for single-threaded local use
- `BURD_TEMP_DIR` — durable copy, addressable by anyone who captured the path
- Fact-files are never large; dual-write cost is trivial
- Requires BUS0 quoin treatment for the module and primitive

### Two-Tier Identity: Legatio and Pensum

**Legatio** (session): A standing connection to a remote project. Created by `jjx_bind`, identified by a legatio token. Contains host, user, reldir. Multiple legationes can coexist within an officium (different hosts, or same host different projects). Persists until explicit unbind or officium absolution.

**Pensum** (job): A single dispatched async operation within a legatio. Created by `jjx_relay`, identified by a pensum token. The pensum stores which legatio it was dispatched through. Multiple pensa can coexist. Each pensum on the curia maps to a remote temp dir path on the fundus.

**Pensum flows curia → fundus via BURE**: `jjx_relay` mints the pensum on the curia, sets `BURE_LABEL=<token>` in the SSH environment. BUD writes it into `burx.env` as `BURX_LABEL`. Both sides can correlate.

### Curia and Fundus — Role Vocabulary

Two sides of every remote operation need distinct quoin-level names:

- **Curia** — the invoking side (master). Where the officium lives, where JJK runs, where the LLM works. The administrative court from which legationes are sent.
- **Fundus** — the executing side (slave). Where BUD runs, where BURX is written, where the tabtarget physically executes. Latin: estate, landed property — the ground where labor happens. C/F first-letter-distinct.

Evolution: sedes/situs rejected (shared first letter 'S'), locus rejected (too common, practically English).

### Command Surface

| Command | Takes | Returns | Mode |
|---------|-------|---------|------|
| `jjx_bind` | `{host, user, reldir}` | legatio token | Creates session, SSH probe + RELDIR safety check at bind time |
| `jjx_send` | `{legatio, command, timeout}` | exit code + output | Synchronous, no pensum |
| `jjx_relay` | `{legatio, tabtarget, timeout}` | pensum token | Async via nohup, mints pensum |
| `jjx_check` | `{pensum, timeout}` | BURX fields + file list (on terminal) | Probe or poll; timeout=0 instant, timeout>0 polls until done or timeout |
| `jjx_fetch` | `{legatio, path}` | file contents | Single file read; path relative to RELDIR or absolute |
| `jjx_plant` | `{legatio, commit}` | success/failure | Synchronous, resets fundus workspace to exact commit |

**`jjx_send`** is purely synchronous — uses legatio for SSH target, blocks until done, returns inline. No pensum, no BURX consumption. BUD still writes BURX on the fundus (it always does), but nobody reads it remotely.

**`jjx_relay`** fires via nohup on the fundus. The timeout is self-enforced remotely (the nohup wrapper kills the process after N seconds). SSH disconnects immediately after launch. Returns pensum token with remote temp dir path captured.

**`jjx_check`** probes or awaits a pensum. Both params required. `timeout` is in seconds: 0 means instant probe (SSH in, read `burx.env`, probe PID, return immediately), >0 means poll until the pensum reaches a terminal state or the timeout expires. Same return shape either way — BURX fields + liveness report:

| BURX_EXIT_STATUS present? | PID alive? | Report |
|---------------------------|------------|--------|
| no | yes | **running** |
| no | no | **orphaned** (crashed without writing terminal status) |
| yes | n/a | **stopped** (exit code = BURX_EXIT_STATUS) |
| file missing | n/a | **lost** |

When timeout>0 and the pensum is still running at expiry, the report is **running** (or **orphaned**) — the same as an instant probe would return. The caller decides what to do next.

On terminal status (stopped), `jjx_check` additionally returns the file list from `BURX_TEMP_DIR`. This lets the curia see what fact-files were produced and construct absolute paths for selective `jjx_fetch` calls without a heavy bundle transfer.

**`jjx_fetch`** reads a single file from the fundus. Path is either relative to RELDIR or absolute. Returns file contents. Typical workflow: `jjx_check` returns BURX fields (including `BURX_TEMP_DIR`) and file list on terminal status; the LLM constructs absolute paths and fetches individual files of interest. Also useful outside pensum workflows — reading any file on the fundus through the legatio.

**`jjx_plant`** resets the fundus workspace to an exact commit. Synchronous over SSH, fail-fast on any step. No clone fallback — the repo must already exist at RELDIR (fundus precondition). Fundus-side sequence:

```bash
cd ~/RELDIR               || exit
git fetch origin           || exit
git reset --hard <commit>  || exit
git clean -dxf             || exit
```

**Design rationale**: `git reset --hard` + `git clean -dxf` is functionally equivalent to rm-and-reclone but preserves `.git` (fast on subsequent plants). RELDIR safety is not re-checked — `jjx_bind` already validated it. The directory itself is never removed, only its working tree contents are reset. The commit parameter is the exact ref (SHA, branch, tag) the curia wants on the fundus — typically the curia's own HEAD so both sides run identical code.

### Async Dispatch Mechanism

nohup + sentinel pattern. POSIX, zero dependencies beyond standard shell.

The nohup wrapper on the fundus:
1. Sets `BURE_LABEL` in environment (pensum token from curia)
2. Invokes the tabtarget (which goes through BUD dispatch)
3. BUD writes initial `burx.env` (no EXIT_STATUS field = running)
4. Tabtarget runs
5. On completion: BUD exit path writes `BURX_EXIT_STATUS` and `BURX_ENDED_AT`
6. On crash: `burx.env` exists with PID but no EXIT_STATUS — `jjx_check` detects via `kill -0`

### Fundus Configuration

**Curia-side constants** (RCG-compliant Rust `const` in the remote dispatch module):

`const FUNDUS_DEFAULT_USER: &str = "rbtest";`
`const FUNDUS_DEFAULT_RELDIR: &str = "projects/rbm_alpha_recipemuster";`

`jjx_bind` takes all three params `{host, user, reldir}` as required. The LLM reads the constants and passes them explicitly — no hidden defaulting. The constants are the single source of truth for "what user/directory do we use on the fundus."

**RELDIR is relative to the fundus user's home directory.** SSH sessions land in `~` by default, so `cd $RELDIR && ...` works on macOS (`/Users/`), Linux (`/home/`), and any other home directory layout without the curia knowing the absolute home path. BURX fields like `BURX_TEMP_DIR` are still absolute on the fundus — BUD resolves them at dispatch time from the actual filesystem.

**Hairpin testing**: Even when the fundus is `localhost`, a distinct user account is required. Podman/Docker container namespaces and image caches are per-user — same user means shared state and test interference. The legatio `(host, user, reldir)` triple guarantees isolation: `(localhost, rbtest, projects/rbm)` is fully orthogonal to the curia's own environment.

**Fundus preconditions** (manual setup before `jjx_bind` can succeed):
1. User account exists with SSH key access from curia
2. Project directory exists at `~user/RELDIR` with BUK installed (`.buk/burc.env` present)
3. BUD is functional — tabtargets can dispatch (regime chain resolves)
4. Container runtime configured for that user (if crucible tests are in scope)
5. Git repo cloned at `~user/RELDIR` with origin configured and GitHub SSH access (required for `jjx_plant`)

### RELDIR Safety — Defense in Depth

Remote dispatch involves destructive operations on the fundus (workspace cleanup). A malformed RELDIR could resolve to `/` or `~`, destroying the filesystem or the user's home. Three layers defend against this:

**Layer 1 — Rust constant validation (compile-time obvious):**
- Must not be empty
- Must not start with `/` (absolute path)
- Must not start with `.` (relative traversal)
- Must contain at least one `/` (forces subdirectory depth — `projects/foo`, never just `foo`)

**Layer 2 — Fundus-side validation before any destructive operation:**
The remote command that `jjx_relay` sends validates the resolved path on the fundus before doing anything:

```bash
z_resolved="$(cd "$HOME/$RELDIR" 2>/dev/null && pwd)" || exit 99
## Must be strictly under $HOME
case "$z_resolved" in
  "$HOME"/*)  ;;
  *)          echo "FATAL: resolved dir not under HOME"; exit 99 ;;
esac
## Must have minimum depth (at least 2 components under HOME)
z_depth="${z_resolved#"$HOME"/}"
case "$z_depth" in
  */*) ;;
  *)   echo "FATAL: resolved dir too shallow under HOME"; exit 99 ;;
esac
```

**Layer 3 — `jjx_bind` probe at session creation:**
The SSH probe that `jjx_bind` runs at bind time performs the same resolved-path check. Fail fast before any relay is ever attempted.

Layer 1 catches developer mistakes. Layer 2 catches runtime resolution surprises (symlinks, `..` in path components). Layer 3 catches it early. No single layer is sufficient alone.

### Provenance

Design emerged through conversation in officium ☉260403-1021. Key evolution:
- `buf_` tinder constants → BURX regime (controlled interface, less drift)
- Single bind → legatio/pensum two-tier identity (concurrent connections and jobs)
- Synchronous-only → async polled dispatch (nohup + PID probe)
- `BURX_STATUS=running` → absence-of-field = running (cleaner)
- sedes/situs → curia/locus → curia/fundus (first-letter uniqueness, "locus" too common)
- `jjx_bind {host, user, directory}` → `{host, user, reldir}` all required, relative path, Rust constants as source of truth
- RELDIR defense-in-depth: Rust validation + fundus-side resolved-path check + bind-time probe
- `jjx_plant`: rm-and-reclone → `git reset --hard` + `git clean -dxf` (preserves `.git`, fail-fast, no clone fallback). Designed in officium ☉260404-1000.
- `jjx_check` + `jjx_await` merged: separate await command rejected in favor of required `timeout` param on `jjx_check` (0=instant probe, >0=poll). Same return shape, caller always states intent explicitly. Designed in officium ☉260404-1000.
- `BURX_STATUS` → `BURX_EXIT_STATUS`: explicit naming. Designed in officium ☉260404-1000.
- `BURX_PENSUM`/`BURE_PENSUM` → `BURX_LABEL`/`BURE_LABEL`: JJK-decoupled correlation field. xname format, 120 char max, optional. BUD writes whatever BURE_LABEL contains; JJK puts pensum tokens in it but BURX doesn't know that. Designed in officium ☉260404-1000.
- `BURX_TRANSCRIPT`: corrected from "directory" to "file" — BUD creates `transcript.txt` as a single file in temp dir. Absolute path retained.
- `jjx_fetch`: bundle-fetch → single-file read with `{legatio, path}`. Path relative to RELDIR or absolute. Pairs with `jjx_check` returning file list on terminal status. General-purpose — works beyond pensum workflows. Designed in officium ☉260404-1000.
- `jjx_check` returns temp dir file list on terminal status: cheap `ls` over existing SSH connection, lets curia decide what to fetch selectively.

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
`echo "${value}" > "${BURD_OUTPUT_DIR}/${RBF_FACT_HALLMARK}"`
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
- ₣A2 — original paces ₢A2AAG (buf-dispatch-fact-files) and ₢A2AAF (remote-execution-bind-send) dropped; superseded by this heat's design
- ₣Ah paddock — officium lifecycle patterns

## Paces

### fundus-scenario-revision (₢A4AAM) [complete]

**[260405-0845] complete**

## Character
Mechanical refactoring with some design judgment on test structure. Touching spec, Rust test code, workbench routing, and tabtargets. All changes follow decisions already made in the groom conversation.

## Docket
Revise JJSTF and integration test infrastructure to be deployment-agnostic.

**Changes:**
1. JJSTF-test-fundus.adoc: Remove machine-identity profiles (hairpin/cerebro). Define 4 account-shape profiles using `jjfu_*` usernames (`jjfu_full`, `jjfu_nokey`, `jjfu_norepo`, `jjfu_nogit`). Remove setup procedures. Remove all literal hostnames. Rename "integration tests" to "fundus scenarios."
2. JJS0 mapping section: Remove `:jjdxf_hairpin:` and `:jjdxf_cerebro:` attributes.
3. remote_dispatch.rs: Remove `FUNDUS_DEFAULT_USER`/`FUNDUS_DEFAULT_RELDIR` imports. Per-profile modules use `jjfu_*` usernames directly. Host parameterized. Rename modules from machine-identity to account-profile. Merge hairpin/cerebro into single `full` module.
4. jjrlg_legatio.rs: Remove `JJRLG_FUNDUS_DEFAULT_USER` and `JJRLG_FUNDUS_DEFAULT_RELDIR` constants.
5. Tabtargets: Delete old `jjw-tR.TestRemote.*.sh`. Create `jjw-tfs.TestFundusScenario.<host>.sh` (suite) and `jjw-tfS.TestFundusSingle.<host>.sh` (single test by function name).
6. jjw_workbench.sh: Replace `jjw-tR` routing with `jjw-tfs`/`jjw-tfS` routing using `jjfu_*` accounts.
7. Context files: Update references.

**[260405-0825] rough**

## Character
Mechanical refactoring with some design judgment on test structure. Touching spec, Rust test code, workbench routing, and tabtargets. All changes follow decisions already made in the groom conversation.

## Docket
Revise JJSTF and integration test infrastructure to be deployment-agnostic.

**Changes:**
1. JJSTF-test-fundus.adoc: Remove machine-identity profiles (hairpin/cerebro). Define 4 account-shape profiles using `jjfu_*` usernames (`jjfu_full`, `jjfu_nokey`, `jjfu_norepo`, `jjfu_nogit`). Remove setup procedures. Remove all literal hostnames. Rename "integration tests" to "fundus scenarios."
2. JJS0 mapping section: Remove `:jjdxf_hairpin:` and `:jjdxf_cerebro:` attributes.
3. remote_dispatch.rs: Remove `FUNDUS_DEFAULT_USER`/`FUNDUS_DEFAULT_RELDIR` imports. Per-profile modules use `jjfu_*` usernames directly. Host parameterized. Rename modules from machine-identity to account-profile. Merge hairpin/cerebro into single `full` module.
4. jjrlg_legatio.rs: Remove `JJRLG_FUNDUS_DEFAULT_USER` and `JJRLG_FUNDUS_DEFAULT_RELDIR` constants.
5. Tabtargets: Delete old `jjw-tR.TestRemote.*.sh`. Create `jjw-tfs.TestFundusScenario.<host>.sh` (suite) and `jjw-tfS.TestFundusSingle.<host>.sh` (single test by function name).
6. jjw_workbench.sh: Replace `jjw-tR` routing with `jjw-tfs`/`jjw-tfS` routing using `jjfu_*` accounts.
7. Context files: Update references.

### fundus-account-provision-and-smoke (₢A4AAN) [complete]

**[260405-1154] complete**

## Character
Mix of system administration scripting and end-to-end verification. The account provisioning scripts require careful sudo/dscl/sysadminctl work (platform-specific). The smoke run is mechanical — invoke the tabtarget and fix what breaks.

## Progress
Infrastructure is built but untested:

**Complete:**
- `jjz_zipper.sh` — JJK zipper registry with fundus colophon group (jjw-tfP, jjw-tfs, jjw-tfS)
- `jjf_cli.sh` — BCG CLI entry point with furnish, delegates to jjf_fundus.sh
- `jjf_fundus.sh` — BCG module with kindle/sentinel, three commands: `jjf_provision` (account provisioning with platform-specific macOS/Linux paths, SSH key auth, repo cloning, BUK install), `jjf_scenario` (suite runner), `jjf_single` (single test runner)
- `jjw_workbench.sh` — converted from inline case routing to `buz_exec_lookup` zipper dispatch
- Tabtargets: `jjw-tfP.ProvisionFundusAccounts.localhost.sh` and `.cerebro.sh`
- Arcanum removed from zipper (slated ₢A4AAP to evaluate full removal)

**Remaining:**
1. Smoke test the zipper dispatch — run a tabtarget and verify the routing chain works (workbench → zipper → CLI → module)
2. Smoke test `jjf_provision` on localhost — creates the 4 jjfu_* accounts with correct precondition shapes
3. Smoke test `jjf_scenario` — run fundus scenario suite against provisioned accounts
4. Fix whatever breaks in steps 1-3
5. Verify `jjf_single` routes correctly
6. Update CLAUDE.md context file references if needed (jjz_zipper, jjf_cli, jjf_fundus)

## Docket
Create tabtargets that provision the 4 `jjfu_*` accounts on a target machine, then run the fundus scenario suite to verify.

**Provisioning tabtargets:**
- `tt/jjw-tfP.ProvisionFundusAccounts.localhost.sh` — provision on localhost
- `tt/jjw-tfP.ProvisionFundusAccounts.cerebro.sh` — provision on cerebro (remote via SSH)

Each provisioning script:
1. Deletes any existing `jjfu_*` accounts (clean slate)
2. Creates the 4 accounts with correct precondition states per JJSTF:
   - `jjfu_full`: SSH key access from curia user, project cloned at `~/projects/rbm_alpha_recipemuster`, BUK installed, git origin configured
   - `jjfu_nokey`: OS account exists, NO SSH key access from curia user
   - `jjfu_norepo`: SSH key access, NO project directory at reldir
   - `jjfu_nogit`: SSH key access, project directory at reldir, BUK installed, git origin removed
3. Idempotent — safe to re-run (delete + recreate pattern)

**Platform consideration:** macOS uses `sysadminctl`/`dscl` for user management, Linux uses `useradd`/`userdel`. Detect platform and use appropriate commands. localhost script runs locally with sudo; cerebro script runs remotely over SSH.

**Smoke verification:**
After provisioning, run `tt/jjw-tfs.TestFundusScenario.localhost.sh` (or cerebro equivalent) and confirm all 4 profiles pass their expected assertions.

**Workbench routing:** Converted to zipper dispatch via `jjz_zipper.sh` and `buz_exec_lookup`. The `jjw-tfP` colophon routes through the zipper to `jjf_cli.sh jjf_provision`.

**[260405-0909] rough**

## Character
Mix of system administration scripting and end-to-end verification. The account provisioning scripts require careful sudo/dscl/sysadminctl work (platform-specific). The smoke run is mechanical — invoke the tabtarget and fix what breaks.

## Progress
Infrastructure is built but untested:

**Complete:**
- `jjz_zipper.sh` — JJK zipper registry with fundus colophon group (jjw-tfP, jjw-tfs, jjw-tfS)
- `jjf_cli.sh` — BCG CLI entry point with furnish, delegates to jjf_fundus.sh
- `jjf_fundus.sh` — BCG module with kindle/sentinel, three commands: `jjf_provision` (account provisioning with platform-specific macOS/Linux paths, SSH key auth, repo cloning, BUK install), `jjf_scenario` (suite runner), `jjf_single` (single test runner)
- `jjw_workbench.sh` — converted from inline case routing to `buz_exec_lookup` zipper dispatch
- Tabtargets: `jjw-tfP.ProvisionFundusAccounts.localhost.sh` and `.cerebro.sh`
- Arcanum removed from zipper (slated ₢A4AAP to evaluate full removal)

**Remaining:**
1. Smoke test the zipper dispatch — run a tabtarget and verify the routing chain works (workbench → zipper → CLI → module)
2. Smoke test `jjf_provision` on localhost — creates the 4 jjfu_* accounts with correct precondition shapes
3. Smoke test `jjf_scenario` — run fundus scenario suite against provisioned accounts
4. Fix whatever breaks in steps 1-3
5. Verify `jjf_single` routes correctly
6. Update CLAUDE.md context file references if needed (jjz_zipper, jjf_cli, jjf_fundus)

## Docket
Create tabtargets that provision the 4 `jjfu_*` accounts on a target machine, then run the fundus scenario suite to verify.

**Provisioning tabtargets:**
- `tt/jjw-tfP.ProvisionFundusAccounts.localhost.sh` — provision on localhost
- `tt/jjw-tfP.ProvisionFundusAccounts.cerebro.sh` — provision on cerebro (remote via SSH)

Each provisioning script:
1. Deletes any existing `jjfu_*` accounts (clean slate)
2. Creates the 4 accounts with correct precondition states per JJSTF:
   - `jjfu_full`: SSH key access from curia user, project cloned at `~/projects/rbm_alpha_recipemuster`, BUK installed, git origin configured
   - `jjfu_nokey`: OS account exists, NO SSH key access from curia user
   - `jjfu_norepo`: SSH key access, NO project directory at reldir
   - `jjfu_nogit`: SSH key access, project directory at reldir, BUK installed, git origin removed
3. Idempotent — safe to re-run (delete + recreate pattern)

**Platform consideration:** macOS uses `sysadminctl`/`dscl` for user management, Linux uses `useradd`/`userdel`. Detect platform and use appropriate commands. localhost script runs locally with sudo; cerebro script runs remotely over SSH.

**Smoke verification:**
After provisioning, run `tt/jjw-tfs.TestFundusScenario.localhost.sh` (or cerebro equivalent) and confirm all 4 profiles pass their expected assertions.

**Workbench routing:** Converted to zipper dispatch via `jjz_zipper.sh` and `buz_exec_lookup`. The `jjw-tfP` colophon routes through the zipper to `jjf_cli.sh jjf_provision`.

**[260405-0835] rough**

## Character
Mix of system administration scripting and end-to-end verification. The account provisioning scripts require careful sudo/dscl/sysadminctl work (platform-specific). The smoke run is mechanical — invoke the tabtarget and fix what breaks.

## Docket
Create tabtargets that provision the 4 `jjfu_*` accounts on a target machine, then run the fundus scenario suite to verify.

**Provisioning tabtargets:**
- `tt/jjw-tfP.ProvisionFundusAccounts.localhost.sh` — provision on localhost
- `tt/jjw-tfP.ProvisionFundusAccounts.cerebro.sh` — provision on cerebro (remote via SSH)

Each provisioning script:
1. Deletes any existing `jjfu_*` accounts (clean slate)
2. Creates the 4 accounts with correct precondition states per JJSTF:
   - `jjfu_full`: SSH key access from curia user, project cloned at `~/projects/rbm_alpha_recipemuster`, BUK installed, git origin configured
   - `jjfu_nokey`: OS account exists, NO SSH key access from curia user
   - `jjfu_norepo`: SSH key access, NO project directory at reldir
   - `jjfu_nogit`: SSH key access, project directory at reldir, BUK installed, git origin removed
3. Idempotent — safe to re-run (delete + recreate pattern)

**Platform consideration:** macOS uses `sysadminctl`/`dscl` for user management, Linux uses `useradd`/`userdel`. Detect platform and use appropriate commands. localhost script runs locally with sudo; cerebro script runs remotely over SSH.

**Smoke verification:**
After provisioning, run `tt/jjw-tfs.TestFundusScenario.localhost.sh` (or cerebro equivalent) and confirm all 4 profiles pass their expected assertions.

**Workbench routing:** Add `jjw-tfP` colophon to `jjw_workbench.sh`.

### concurrent-dispatch-scenario (₢A4AAO) [complete]

**[260405-1249] complete**

## Character
Two parts: a trivial BUK tabtarget (mechanical) and a fundus scenario test function that uses it to prove temporal overlap doesn't cause cross-contamination (moderate — timing assertions need care).

## Docket
Add a BUK delay tabtarget and a fundus scenario test exercising genuinely concurrent dispatch.

**BUK delay tabtarget:**
- Colophon: `buw-dly` (BUK workbench delay). Simple sleep through BUD dispatch.
- `tt/buw-dly.Delay.sh` — dispatches, sleeps 20 seconds, exits 0.
- Workbench routing in `buw_workbench.sh` for the `buw-dly` colophon.
- Purpose: general-purpose timing fixture for any project that installs BUK. Lives in BUK because it tests dispatch infrastructure, not JJK-specific behavior. Usable against foreign BUK instances.

**Fundus scenario test:**
- New test function in `fundus_scenario.rs` under the `full` module: `relay_concurrent_overlap`
- Dispatches 3 relay jobs using the delay tabtarget against `jjfu_full`
- Instant-probes all 3 — asserts all report **running** (temporal overlap guaranteed by 20s sleep)
- Polls one to completion — confirms **stopped** with correct BURX_LABEL
- Verifies all 3 BURX_LABELs are distinct (no cross-contamination)
- Verifies all 3 BURX_TEMP_DIRs are distinct (isolation)

**Replaces:** The existing `relay_parallel` test in `full` module, which uses a fast tabtarget that doesn't guarantee temporal overlap. Or keeps both if the fast version has independent value (testing rapid sequential dispatch).

**[260405-0838] rough**

## Character
Two parts: a trivial BUK tabtarget (mechanical) and a fundus scenario test function that uses it to prove temporal overlap doesn't cause cross-contamination (moderate — timing assertions need care).

## Docket
Add a BUK delay tabtarget and a fundus scenario test exercising genuinely concurrent dispatch.

**BUK delay tabtarget:**
- Colophon: `buw-dly` (BUK workbench delay). Simple sleep through BUD dispatch.
- `tt/buw-dly.Delay.sh` — dispatches, sleeps 20 seconds, exits 0.
- Workbench routing in `buw_workbench.sh` for the `buw-dly` colophon.
- Purpose: general-purpose timing fixture for any project that installs BUK. Lives in BUK because it tests dispatch infrastructure, not JJK-specific behavior. Usable against foreign BUK instances.

**Fundus scenario test:**
- New test function in `fundus_scenario.rs` under the `full` module: `relay_concurrent_overlap`
- Dispatches 3 relay jobs using the delay tabtarget against `jjfu_full`
- Instant-probes all 3 — asserts all report **running** (temporal overlap guaranteed by 20s sleep)
- Polls one to completion — confirms **stopped** with correct BURX_LABEL
- Verifies all 3 BURX_LABELs are distinct (no cross-contamination)
- Verifies all 3 BURX_TEMP_DIRs are distinct (isolation)

**Replaces:** The existing `relay_parallel` test in `full` module, which uses a fast tabtarget that doesn't guarantee temporal overlap. Or keeps both if the fast version has independent value (testing rapid sequential dispatch).

### fundus-account-prefix-cleanup (₢A4AAQ) [complete]

**[260405-1311] complete**

## Character
Mechanical — straightforward constant extraction and platform-specific account enumeration. No design judgment needed.

## Docket
Extract `jjfu_` prefix as a named constant and use prefix-based enumeration for Phase 1 account deletion.

**Constant:**
- `JJFP_account_prefix="jjfu_"` — single source of truth for the fundus test account prefix

**Delete phase (Phase 1):**
- Replace iteration over `ZJJFP_ACCOUNTS` with platform-specific prefix enumeration:
  - macOS: `dscl . -list /Users | grep "^${JJFP_account_prefix}"`
  - Linux: `getent passwd | grep "^${JJFP_account_prefix}" | cut -d: -f1`
- Catches stale accounts from previous configurations

**Magic string cleanup:**
- Audit `jjfp_fundus.sh` for bare `jjfu_` literals — replace with constant or derive from it
- The Rust test constants (`JJFU_FULL`, etc.) are fine as-is — they're in a different namespace and individually named

**Does not change:** Account creation logic (each account has distinct setup steps), `ZJJFP_ACCOUNTS` list (still needed for creation), or test account names.

**[260405-1253] rough**

## Character
Mechanical — straightforward constant extraction and platform-specific account enumeration. No design judgment needed.

## Docket
Extract `jjfu_` prefix as a named constant and use prefix-based enumeration for Phase 1 account deletion.

**Constant:**
- `JJFP_account_prefix="jjfu_"` — single source of truth for the fundus test account prefix

**Delete phase (Phase 1):**
- Replace iteration over `ZJJFP_ACCOUNTS` with platform-specific prefix enumeration:
  - macOS: `dscl . -list /Users | grep "^${JJFP_account_prefix}"`
  - Linux: `getent passwd | grep "^${JJFP_account_prefix}" | cut -d: -f1`
- Catches stale accounts from previous configurations

**Magic string cleanup:**
- Audit `jjfp_fundus.sh` for bare `jjfu_` literals — replace with constant or derive from it
- The Rust test constants (`JJFU_FULL`, etc.) are fine as-is — they're in a different namespace and individually named

**Does not change:** Account creation logic (each account has distinct setup steps), `ZJJFP_ACCOUNTS` list (still needed for creation), or test account names.

### buk-burx-regime-and-dual-write (₢A4AAA) [complete]

**[260404-0949] complete**

## Character
Spec-and-implement in one pass. Mechanical BUK plumbing — the design is fully resolved in the paddock. Requires careful BUS0 quoin treatment but no design judgment.

## Docket
Implement BURX regime and dual-write fact-file primitive across BUS0 spec and BUK source.

**BUS0 spec work:**
- Define BURX regime: 9 fields (PID, BEGAN_AT, TABTARGET, TEMP_DIR, TRANSCRIPT, LOG_HIST, LABEL, EXIT_STATUS, ENDED_AT)
- Define `buf_write_fact` primitive extending `busff_fact_file` pattern for dual-write
- Add BURE_LABEL to BURE kindle definition (xname, max 120 chars, optional in env — BUD writes empty string when absent)
- Quoin treatment for all new terms with appropriate AXLA voicings

**BUK source work:**
- New `buf_fact.sh` module (or similar) with `buf_write_fact` primitive — writes to both BURD_OUTPUT_DIR and BURD_TEMP_DIR
- BUD modifications: write `burx.env` at dispatch start (7 fields) and completion (EXIT_STATUS, ENDED_AT)
- BUC modifications: BURE_LABEL kindle constant
- BURX_LABEL is always present in `burx.env` — set to empty string when BURE_LABEL env var is absent

**Verification:** Run `tt/buw-st.BukSelfTest.sh` after modifications to confirm BUK framework still passes.

**Key constraints:**
- BURX_EXIT_STATUS absence = still running (no explicit "running" value)
- BURX_LABEL is JJK-decoupled — BUD writes whatever BURE_LABEL contains (or empty string)
- All BURX paths must be absolute
- `burx.env` is written to both BURD_OUTPUT_DIR (latest-command, ephemeral) and BURD_TEMP_DIR (durable, addressable by remote consumers). Remote readers (`jjx_check`) read from BURD_TEMP_DIR.
- See paddock "BURX Regime" and "Dual-Write Fact-File Module" sections for full design

**Does NOT include:** RBK producer migration (~15 sites) — deferred to separate heat.

**[260404-0917] rough**

## Character
Spec-and-implement in one pass. Mechanical BUK plumbing — the design is fully resolved in the paddock. Requires careful BUS0 quoin treatment but no design judgment.

## Docket
Implement BURX regime and dual-write fact-file primitive across BUS0 spec and BUK source.

**BUS0 spec work:**
- Define BURX regime: 9 fields (PID, BEGAN_AT, TABTARGET, TEMP_DIR, TRANSCRIPT, LOG_HIST, LABEL, EXIT_STATUS, ENDED_AT)
- Define `buf_write_fact` primitive extending `busff_fact_file` pattern for dual-write
- Add BURE_LABEL to BURE kindle definition (xname, max 120 chars, optional in env — BUD writes empty string when absent)
- Quoin treatment for all new terms with appropriate AXLA voicings

**BUK source work:**
- New `buf_fact.sh` module (or similar) with `buf_write_fact` primitive — writes to both BURD_OUTPUT_DIR and BURD_TEMP_DIR
- BUD modifications: write `burx.env` at dispatch start (7 fields) and completion (EXIT_STATUS, ENDED_AT)
- BUC modifications: BURE_LABEL kindle constant
- BURX_LABEL is always present in `burx.env` — set to empty string when BURE_LABEL env var is absent

**Verification:** Run `tt/buw-st.BukSelfTest.sh` after modifications to confirm BUK framework still passes.

**Key constraints:**
- BURX_EXIT_STATUS absence = still running (no explicit "running" value)
- BURX_LABEL is JJK-decoupled — BUD writes whatever BURE_LABEL contains (or empty string)
- All BURX paths must be absolute
- `burx.env` is written to both BURD_OUTPUT_DIR (latest-command, ephemeral) and BURD_TEMP_DIR (durable, addressable by remote consumers). Remote readers (`jjx_check`) read from BURD_TEMP_DIR.
- See paddock "BURX Regime" and "Dual-Write Fact-File Module" sections for full design

**Does NOT include:** RBK producer migration (~15 sites) — deferred to separate heat.

**[260404-0853] rough**

## Character
Spec-and-implement in one pass. Mechanical BUK plumbing — the design is fully resolved in the paddock. Requires careful BUS0 quoin treatment but no design judgment.

## Docket
Implement BURX regime and dual-write fact-file primitive across BUS0 spec and BUK source.

**BUS0 spec work:**
- Define BURX regime: 9 fields (PID, BEGAN_AT, TABTARGET, TEMP_DIR, TRANSCRIPT, LOG_HIST, LABEL, EXIT_STATUS, ENDED_AT)
- Define `buf_write_fact` primitive extending `busff_fact_file` pattern for dual-write
- Add BURE_LABEL to BURE kindle definition (xname, max 120 chars, optional)
- Quoin treatment for all new terms with appropriate AXLA voicings

**BUK source work:**
- New `buf_fact.sh` module (or similar) with `buf_write_fact` primitive — writes to both BURD_OUTPUT_DIR and BURD_TEMP_DIR
- BUD modifications: write `burx.env` at dispatch start (7 fields) and completion (EXIT_STATUS, ENDED_AT)
- BUC modifications: BURE_LABEL kindle constant
- BURX_LABEL populated from BURE_LABEL env var; empty string when absent

**Key constraints:**
- BURX_EXIT_STATUS absence = still running (no explicit "running" value)
- BURX_LABEL is JJK-decoupled — BUD writes whatever BURE_LABEL contains
- All BURX paths must be absolute
- See paddock "BURX Regime" and "Dual-Write Fact-File Module" sections for full design

**Does NOT include:** RBK producer migration (~15 sites) — deferred to separate heat.

### jjk-ssh-foundation-sync-commands (₢A4AAC) [complete]

**[260404-1317] complete**

## Character
Standard Rust development. SSH infrastructure is the foundation — get this right and the async pace builds on it cleanly. Moderate judgment on error handling and fundus regime discovery.

## Docket
Implement SSH execution infrastructure and all synchronous remote commands in JJK Rust source.

**SSH implementation:** Use platform `ssh` binary via `std::process::Command`. No Rust SSH crate — platform openssh is already security-audited, inherits user's `~/.ssh/config`, known_hosts, and agent forwarding.

**SSH infrastructure:**
- SSH connection management using legatio (host, user, reldir)
- Command execution over SSH (blocking, capture stdout+stderr as raw bytes)
- Error propagation from remote exit codes

**Commands to implement:**
- `jjx_bind` — SSH probe, RELDIR safety validation (Layer 1 Rust + Layer 3 remote probe), legatio token minting, persist in officium state. Additionally: read `.buk/burc.env` on fundus and cache `BURC_OUTPUT_ROOT_DIR` (needed by `jjx_relay` to locate `burx.env` after dispatch).
- `jjx_send` — synchronous exec via legatio, return exit code + output. Command param is a single string interpreted by `bash -c` on the fundus after `cd RELDIR`. Caller (LLM) is responsible for shell quoting within the string. For tabtargets: `./tt/foo.sh arg1 arg2` — no quoting issues in practice.
- `jjx_plant` — `git fetch origin && git reset --hard <commit> && git clean -dxf` over SSH, fail-fast
- `jjx_fetch` — SSH cat of single file, path relative to RELDIR or absolute. Transfers raw bytes — works for both text and binary files.

**Legatio state:**
- Persisted in officium directory (host, user, reldir, cached output dir path per legatio)
- Multiple legationes per officium
- Token format TBD (follow existing JJK identity patterns)

**Key constraints:**
- RELDIR validation: Rust constant checks (Layer 1) + bind-time remote probe (Layer 3)
- Fundus-side constants: `FUNDUS_DEFAULT_USER`, `FUNDUS_DEFAULT_RELDIR` as Rust `const`
- All commands fail hard on SSH errors — no retry logic

**Depends on:** ₢A4AAA (BURX regime exists in BUS0/BUK — ecosystem coherence, and bind caches regime paths that relay needs)

**[260404-0917] rough**

## Character
Standard Rust development. SSH infrastructure is the foundation — get this right and the async pace builds on it cleanly. Moderate judgment on error handling and fundus regime discovery.

## Docket
Implement SSH execution infrastructure and all synchronous remote commands in JJK Rust source.

**SSH implementation:** Use platform `ssh` binary via `std::process::Command`. No Rust SSH crate — platform openssh is already security-audited, inherits user's `~/.ssh/config`, known_hosts, and agent forwarding.

**SSH infrastructure:**
- SSH connection management using legatio (host, user, reldir)
- Command execution over SSH (blocking, capture stdout+stderr as raw bytes)
- Error propagation from remote exit codes

**Commands to implement:**
- `jjx_bind` — SSH probe, RELDIR safety validation (Layer 1 Rust + Layer 3 remote probe), legatio token minting, persist in officium state. Additionally: read `.buk/burc.env` on fundus and cache `BURC_OUTPUT_ROOT_DIR` (needed by `jjx_relay` to locate `burx.env` after dispatch).
- `jjx_send` — synchronous exec via legatio, return exit code + output. Command param is a single string interpreted by `bash -c` on the fundus after `cd RELDIR`. Caller (LLM) is responsible for shell quoting within the string. For tabtargets: `./tt/foo.sh arg1 arg2` — no quoting issues in practice.
- `jjx_plant` — `git fetch origin && git reset --hard <commit> && git clean -dxf` over SSH, fail-fast
- `jjx_fetch` — SSH cat of single file, path relative to RELDIR or absolute. Transfers raw bytes — works for both text and binary files.

**Legatio state:**
- Persisted in officium directory (host, user, reldir, cached output dir path per legatio)
- Multiple legationes per officium
- Token format TBD (follow existing JJK identity patterns)

**Key constraints:**
- RELDIR validation: Rust constant checks (Layer 1) + bind-time remote probe (Layer 3)
- Fundus-side constants: `FUNDUS_DEFAULT_USER`, `FUNDUS_DEFAULT_RELDIR` as Rust `const`
- All commands fail hard on SSH errors — no retry logic

**Depends on:** ₢A4AAA (BURX regime exists in BUS0/BUK — ecosystem coherence, and bind caches regime paths that relay needs)

**[260404-0854] rough**

## Character
Standard Rust development. SSH infrastructure is the foundation — get this right and the async pace builds on it cleanly. Moderate judgment on SSH session management and error handling.

## Docket
Implement SSH execution infrastructure and all synchronous remote commands in JJK Rust source.

**SSH infrastructure:**
- SSH connection management using legatio (host, user, reldir)
- Command execution over SSH (blocking, capture stdout/stderr)
- Error propagation from remote exit codes

**Commands to implement:**
- `jjx_bind` — SSH probe, RELDIR safety validation (Layer 1 Rust + Layer 3 remote probe), legatio token minting, persist in officium state
- `jjx_send` — synchronous exec via legatio, return exit code + output
- `jjx_plant` — `git fetch origin && git reset --hard <commit> && git clean -dxf` over SSH, fail-fast
- `jjx_fetch` — SSH cat of single file, path relative to RELDIR or absolute

**Legatio state:**
- Persisted in officium directory (host, user, reldir per legatio)
- Multiple legationes per officium
- Token format TBD (follow existing JJK identity patterns)

**Key constraints:**
- RELDIR validation: Rust constant checks (Layer 1) + bind-time remote probe (Layer 3)
- Fundus-side constants: `FUNDUS_DEFAULT_USER`, `FUNDUS_DEFAULT_RELDIR` as Rust `const`
- All commands fail hard on SSH errors — no retry logic

**Depends on:** ₢A4AAA (BURX regime must exist for the ecosystem, though sync commands don't consume BURX directly)

### jjk-pensum-identity-type (₢A4AAJ) [complete]

**[260404-1416] complete**

## Character
Infrastructure work requiring precision — new identity type with sentinel disambiguation, gallops schema evolution, and seed lifecycle. Moderate judgment on parse/display API design; the structural decisions are already made.

## Docket
Implement the Pensum identity type (₱) in JJK Rust source. Pensum is a globally-unique identity for a single remote dispatch operation, used in `BURX_LABEL` for fossil-record correlation.

**Authoritative spec:** See `jjdt_pensum` in JJS0 for the type definition, `jjdhm_pensum_seed` for the heat member spec.

**Structure:** 5 characters — 2 base64url (heat firemark) + literal `%` sentinel + 2 base64url (index within heat). Example: `₱Ah%BE` belongs to heat `₣Ah`.

**Unicode prefix:** `₱` (U+20B1, peso sign).

**Sentinel disambiguation:** Pensum shares character count 5 with coronet. The `%` at position 3 cannot appear in base64url — parse checks position 3 to distinguish types mechanically.

**Charset confirmation:** The existing `JJRF_CHARSET` in `jjrf_favor.rs` is the full 64-character base64url alphabet (A-Z, a-z, 0-9, `-`, `_`). Pensum uses the same charset for its heat and index digits.

**Identity type implementation (`jjrf_favor.rs`):**
- `jjrf_Pensum` struct (newtype wrapping String, like Firemark/Coronet)
- `JJRF_PENSUM_PREFIX: char = '₱'`
- `jjrf_encode(heat: &jjrf_Firemark, index: u32) -> Self` — format: `{firemark}%{base64url}{base64url}`
- `jjrf_decode(&self) -> Result<(jjrf_Firemark, u32), String>` — split on `%`, validate both parts
- `jjrf_parse(input: &str) -> Result<Self, String>` — strip prefix if present, validate 5 chars with `%` at position 3
- `jjrf_display(&self) -> String` — prefix + payload
- `jjrf_as_str(&self) -> &str` — payload only
- `jjrf_parent_firemark(&self) -> jjrf_Firemark` — extract chars 0-1

**Gallops schema change (`jjrt_types.rs`):**
- Add `next_pensum_seed: String` field to `jjrg_Heat` (2-char base64url seed, like `next_pace_seed`)
- `#[serde(default = "default_pensum_seed")]` for backwards compatibility with existing gallops files
- Default: `"AA"`

**Seed minting (`jjro_ops.rs`):**
- `jjrg_mint_pensum(heat, firemark) -> jjrf_Pensum` — read `next_pensum_seed`, encode, increment seed
- Same increment logic as pace seed minting

**Capacity:** 4,096 pensa per heat (64^2).

**Tests:** Encode/decode round-trip, parse with/without prefix, sentinel disambiguation vs coronet, parent firemark extraction, seed increment, gallops backwards compatibility with missing field.

**Does NOT include:** `jjx_relay`, `jjx_check`, or any command that consumes the pensum type. This pace is purely the identity infrastructure.

**[260404-1312] rough**

## Character
Infrastructure work requiring precision — new identity type with sentinel disambiguation, gallops schema evolution, and seed lifecycle. Moderate judgment on parse/display API design; the structural decisions are already made.

## Docket
Implement the Pensum identity type (₱) in JJK Rust source. Pensum is a globally-unique identity for a single remote dispatch operation, used in `BURX_LABEL` for fossil-record correlation.

**Authoritative spec:** See `jjdt_pensum` in JJS0 for the type definition, `jjdhm_pensum_seed` for the heat member spec.

**Structure:** 5 characters — 2 base64url (heat firemark) + literal `%` sentinel + 2 base64url (index within heat). Example: `₱Ah%BE` belongs to heat `₣Ah`.

**Unicode prefix:** `₱` (U+20B1, peso sign).

**Sentinel disambiguation:** Pensum shares character count 5 with coronet. The `%` at position 3 cannot appear in base64url — parse checks position 3 to distinguish types mechanically.

**Charset confirmation:** The existing `JJRF_CHARSET` in `jjrf_favor.rs` is the full 64-character base64url alphabet (A-Z, a-z, 0-9, `-`, `_`). Pensum uses the same charset for its heat and index digits.

**Identity type implementation (`jjrf_favor.rs`):**
- `jjrf_Pensum` struct (newtype wrapping String, like Firemark/Coronet)
- `JJRF_PENSUM_PREFIX: char = '₱'`
- `jjrf_encode(heat: &jjrf_Firemark, index: u32) -> Self` — format: `{firemark}%{base64url}{base64url}`
- `jjrf_decode(&self) -> Result<(jjrf_Firemark, u32), String>` — split on `%`, validate both parts
- `jjrf_parse(input: &str) -> Result<Self, String>` — strip prefix if present, validate 5 chars with `%` at position 3
- `jjrf_display(&self) -> String` — prefix + payload
- `jjrf_as_str(&self) -> &str` — payload only
- `jjrf_parent_firemark(&self) -> jjrf_Firemark` — extract chars 0-1

**Gallops schema change (`jjrt_types.rs`):**
- Add `next_pensum_seed: String` field to `jjrg_Heat` (2-char base64url seed, like `next_pace_seed`)
- `#[serde(default = "default_pensum_seed")]` for backwards compatibility with existing gallops files
- Default: `"AA"`

**Seed minting (`jjro_ops.rs`):**
- `jjrg_mint_pensum(heat, firemark) -> jjrf_Pensum` — read `next_pensum_seed`, encode, increment seed
- Same increment logic as pace seed minting

**Capacity:** 4,096 pensa per heat (64^2).

**Tests:** Encode/decode round-trip, parse with/without prefix, sentinel disambiguation vs coronet, parent firemark extraction, seed increment, gallops backwards compatibility with missing field.

**Does NOT include:** `jjx_relay`, `jjx_check`, or any command that consumes the pensum type. This pace is purely the identity infrastructure.

**[260404-1251] rough**

## Character
Infrastructure work requiring precision — new identity type with sentinel disambiguation, gallops schema evolution, and seed lifecycle. Moderate judgment on parse/display API design; the structural decisions are already made.

## Docket
Implement the Pensum identity type (₱) in JJK Rust source. Pensum is a globally-unique identity for a single remote dispatch operation, used in `BURX_LABEL` for fossil-record correlation.

**Structure:** 5 characters — 2 base64url (heat firemark) + literal `%` sentinel + 2 base64url (index within heat). Example: `₱Ah%BE` belongs to heat `₣Ah`.

**Unicode prefix:** `₱` (U+20B1, peso sign).

**Sentinel disambiguation:** Pensum shares character count 5 with coronet. The `%` at position 3 cannot appear in base64url — parse checks position 3 to distinguish types mechanically.

**Identity type implementation (`jjrf_favor.rs`):**
- `jjrf_Pensum` struct (newtype wrapping String, like Firemark/Coronet)
- `JJRF_PENSUM_PREFIX: char = '₱'`
- `jjrf_encode(heat: &jjrf_Firemark, index: u32) -> Self` — format: `{firemark}%{base64}{base64}`
- `jjrf_decode(&self) -> Result<(jjrf_Firemark, u32), String>` — split on `%`, validate both parts
- `jjrf_parse(input: &str) -> Result<Self, String>` — strip prefix if present, validate 5 chars with `%` at position 3
- `jjrf_display(&self) -> String` — prefix + payload
- `jjrf_as_str(&self) -> &str` — payload only
- `jjrf_parent_firemark(&self) -> jjrf_Firemark` — extract chars 0-1

**Gallops schema change (`jjrt_types.rs`):**
- Add `next_pensum_seed: String` field to `jjrg_Heat` (2-char base64 seed, like `next_pace_seed`)
- `#[serde(default = "default_pensum_seed")]` for backwards compatibility with existing gallops files
- Default: `"AA"`

**Seed minting (`jjro_ops.rs`):**
- `jjrg_mint_pensum(heat, firemark) -> jjrf_Pensum` — read `next_pensum_seed`, encode, increment seed
- Same increment logic as pace seed minting

**Capacity:** 3,844 pensa per heat (62^2). Index digits follow same base64url charset as all other identity types.

**Tests:** Encode/decode round-trip, parse with/without prefix, sentinel disambiguation vs coronet, parent firemark extraction, seed increment, gallops backwards compatibility with missing field.

**Does NOT include:** `jjx_relay`, `jjx_check`, or any command that consumes the pensum type. This pace is purely the identity infrastructure.

### jjk-async-dispatch-commands (₢A4AAD) [complete]

**[260404-1446] complete**

## Character
Architecturally distinct from sync commands — nohup wrapper generation, BURX parsing, PID probing, polling loop. Moderate complexity, builds on SSH infrastructure from ₢A4AAC.

## Docket
Implement async remote dispatch commands in JJK Rust source.

**Commands to implement:**
- `jjx_relay` — mint pensum token, construct nohup wrapper script, SSH #1 launches dispatch with `BURE_LABEL=<pensum>` and disconnects immediately. SSH #2 reads `burx.env` from the output dir (path cached in legatio during bind) to capture BURX_TEMP_DIR and all start fields. Store temp dir path in pensum state. Timeout is fundus-side self-enforced (nohup wrapper kills after N seconds).
- `jjx_check` — SSH into fundus, read `burx.env` from stored BURX_TEMP_DIR path (the durable copy), probe `kill -0 $PID`. Both params required (pensum, timeout). timeout=0 instant probe, timeout>0 poll until terminal or expiry. On terminal status, additionally `ls` the temp dir and return file list. Return BURX fields + liveness report (running/orphaned/stopped/lost). When timeout>0 and pensum is still running at expiry, returns **running** (or **orphaned**) — caller distinguishes timeout from completion by the report state.

**Pensum state:**
- Persisted in officium directory (legatio ref, remote temp dir path per pensum)
- Multiple pensa per officium
- Token format TBD (follow existing JJK identity patterns)

**BURX parsing:**
- Read `burx.env` as key=value fact-file
- Detect terminal state: BURX_EXIT_STATUS present → stopped
- Detect anomalies: no EXIT_STATUS + PID dead → orphaned; file missing → lost

**Polling implementation (timeout > 0):**
- Sleep interval is a system-level implementation detail, not specified here
- Each poll iteration: SSH in, read burx.env, probe PID
- Return same shape whether timed out or terminal reached

**Depends on:** ₢A4AAA (BURX regime), ₢A4AAC (SSH infrastructure + legatio state with cached output dir path)

**[260404-0917] rough**

## Character
Architecturally distinct from sync commands — nohup wrapper generation, BURX parsing, PID probing, polling loop. Moderate complexity, builds on SSH infrastructure from ₢A4AAC.

## Docket
Implement async remote dispatch commands in JJK Rust source.

**Commands to implement:**
- `jjx_relay` — mint pensum token, construct nohup wrapper script, SSH #1 launches dispatch with `BURE_LABEL=<pensum>` and disconnects immediately. SSH #2 reads `burx.env` from the output dir (path cached in legatio during bind) to capture BURX_TEMP_DIR and all start fields. Store temp dir path in pensum state. Timeout is fundus-side self-enforced (nohup wrapper kills after N seconds).
- `jjx_check` — SSH into fundus, read `burx.env` from stored BURX_TEMP_DIR path (the durable copy), probe `kill -0 $PID`. Both params required (pensum, timeout). timeout=0 instant probe, timeout>0 poll until terminal or expiry. On terminal status, additionally `ls` the temp dir and return file list. Return BURX fields + liveness report (running/orphaned/stopped/lost). When timeout>0 and pensum is still running at expiry, returns **running** (or **orphaned**) — caller distinguishes timeout from completion by the report state.

**Pensum state:**
- Persisted in officium directory (legatio ref, remote temp dir path per pensum)
- Multiple pensa per officium
- Token format TBD (follow existing JJK identity patterns)

**BURX parsing:**
- Read `burx.env` as key=value fact-file
- Detect terminal state: BURX_EXIT_STATUS present → stopped
- Detect anomalies: no EXIT_STATUS + PID dead → orphaned; file missing → lost

**Polling implementation (timeout > 0):**
- Sleep interval is a system-level implementation detail, not specified here
- Each poll iteration: SSH in, read burx.env, probe PID
- Return same shape whether timed out or terminal reached

**Depends on:** ₢A4AAA (BURX regime), ₢A4AAC (SSH infrastructure + legatio state with cached output dir path)

**[260404-0854] rough**

## Character
Architecturally distinct from sync commands — nohup wrapper generation, BURX parsing, PID probing, polling loop. Moderate complexity, builds on SSH infrastructure from ₢A4AAC.

## Docket
Implement async remote dispatch commands in JJK Rust source.

**Commands to implement:**
- `jjx_relay` — mint pensum token, construct nohup wrapper script, SSH launch with `BURE_LABEL=<pensum>`, capture remote BURX_TEMP_DIR, disconnect immediately. Timeout is fundus-side self-enforced (nohup wrapper kills after N seconds).
- `jjx_check` — SSH into fundus, read `burx.env` from stored temp dir path, probe `kill -0 $PID`. Both params required (pensum, timeout). timeout=0 instant probe, timeout>0 poll until terminal or expiry. On terminal status, additionally `ls` the temp dir and return file list. Return BURX fields + liveness report (running/orphaned/stopped/lost).

**Pensum state:**
- Persisted in officium directory (legatio ref, remote temp dir path per pensum)
- Multiple pensa per officium
- Token format TBD (follow existing JJK identity patterns)

**BURX parsing:**
- Read `burx.env` as key=value fact-file
- Detect terminal state: BURX_EXIT_STATUS present → stopped
- Detect anomalies: no EXIT_STATUS + PID dead → orphaned; file missing → lost

**Polling implementation (timeout > 0):**
- Sleep interval TBD (1-5 seconds reasonable)
- Each poll iteration: SSH in, read burx.env, probe PID
- Return same shape whether timed out or terminal reached

**Depends on:** ₢A4AAA (BURX regime), ₢A4AAC (SSH infrastructure + legatio state)

### remote-dispatch-test-fundus-spec (₢A4AAE) [complete]

**[260404-1446] complete**

## Character
Spec writing and infrastructure design — opus-tier. Defines the contract that test code will execute against. Requires judgment about what failure modes are worth investing in, what invariants each account must satisfy, and how the spec integrates with JJS0's existing structure. Manual account setup follows as a human procedure.

## Docket
Write JJS0 subdocument specifying the test fundus infrastructure for remote dispatch integration testing.

**Deliverable: JJS0 subdocument** (e.g., `JJSCTF-test-fundus.adoc`, acronym TBD — mint at write time):
- Included in JJS0's test infrastructure section
- Uses quoins for fundus profiles, account contracts, and test invariants
- Cross-references remote dispatch commands (`jjx_bind`, `jjx_send`, `jjx_relay`, `jjx_check`, `jjx_fetch`, `jjx_plant`)

**Fundus profile specifications** — each profile is a named configuration:

Happy-path profiles:
- `hairpin` — localhost, dedicated test user, real repo clone at RELDIR. Full happy-path suite: bind, send, relay (parallel dispatch of N concurrent BUK invocations), check, fetch, plant.
- `cerebro` — remote machine, same expectations as hairpin. Validates that nothing is accidentally localhost-dependent.

Failure-mode profiles (localhost only, one broken property each):
- No SSH key → bind auth failure
- Good SSH, no repo at RELDIR → bind Layer 3 probe failure
- Good SSH, repo exists, no git → plant failure, send still works
- Additional profiles as warranted by design conversation

**For each profile, specify:**
- Host, user, RELDIR expectations
- What is deliberately configured vs deliberately missing
- What test assertions the profile supports
- Setup procedure (one-time manual steps to create the account/environment)

**Tabtarget surface design:**
- `jjw-tR` colophon with imprint channel (one tabtarget per fundus profile)
- Tabtarget wraps `cargo test` with appropriate filter/feature flags
- Define how imprint maps to fundus profile selection at runtime

**Preflight contract:**
- Each profile has a preflight probe (SSH attempt, RELDIR check) that skips the suite cleanly if preconditions aren't met
- Preflight failures are informational, not errors — a machine may not have all profiles configured

**Does NOT include:** Writing the Rust test code. This pace produces the specification; ₢A4AAG implements against it.

**Depends on:** ₢A4AAB (JJS0 remote dispatch vocabulary must exist for quoin cross-references)

**[260404-1418] rough**

## Character
Spec writing and infrastructure design — opus-tier. Defines the contract that test code will execute against. Requires judgment about what failure modes are worth investing in, what invariants each account must satisfy, and how the spec integrates with JJS0's existing structure. Manual account setup follows as a human procedure.

## Docket
Write JJS0 subdocument specifying the test fundus infrastructure for remote dispatch integration testing.

**Deliverable: JJS0 subdocument** (e.g., `JJSCTF-test-fundus.adoc`, acronym TBD — mint at write time):
- Included in JJS0's test infrastructure section
- Uses quoins for fundus profiles, account contracts, and test invariants
- Cross-references remote dispatch commands (`jjx_bind`, `jjx_send`, `jjx_relay`, `jjx_check`, `jjx_fetch`, `jjx_plant`)

**Fundus profile specifications** — each profile is a named configuration:

Happy-path profiles:
- `hairpin` — localhost, dedicated test user, real repo clone at RELDIR. Full happy-path suite: bind, send, relay (parallel dispatch of N concurrent BUK invocations), check, fetch, plant.
- `cerebro` — remote machine, same expectations as hairpin. Validates that nothing is accidentally localhost-dependent.

Failure-mode profiles (localhost only, one broken property each):
- No SSH key → bind auth failure
- Good SSH, no repo at RELDIR → bind Layer 3 probe failure
- Good SSH, repo exists, no git → plant failure, send still works
- Additional profiles as warranted by design conversation

**For each profile, specify:**
- Host, user, RELDIR expectations
- What is deliberately configured vs deliberately missing
- What test assertions the profile supports
- Setup procedure (one-time manual steps to create the account/environment)

**Tabtarget surface design:**
- `jjw-tR` colophon with imprint channel (one tabtarget per fundus profile)
- Tabtarget wraps `cargo test` with appropriate filter/feature flags
- Define how imprint maps to fundus profile selection at runtime

**Preflight contract:**
- Each profile has a preflight probe (SSH attempt, RELDIR check) that skips the suite cleanly if preconditions aren't met
- Preflight failures are informational, not errors — a machine may not have all profiles configured

**Does NOT include:** Writing the Rust test code. This pace produces the specification; ₢A4AAG implements against it.

**Depends on:** ₢A4AAB (JJS0 remote dispatch vocabulary must exist for quoin cross-references)

**[260404-1418] rough**

## Character
Spec writing and infrastructure design — opus-tier. Defines the contract that test code will execute against. Requires judgment about what failure modes are worth investing in, what invariants each account must satisfy, and how the spec integrates with JJS0's existing structure. Manual account setup follows as a human procedure.

## Docket
Write JJS0 subdocument specifying the test fundus infrastructure for remote dispatch integration testing.

**Deliverable: JJS0 subdocument** (e.g., `JJSCTF-test-fundus.adoc`, acronym TBD — mint at write time):
- Included in JJS0's test infrastructure section
- Uses quoins for fundus profiles, account contracts, and test invariants
- Cross-references remote dispatch commands (`jjx_bind`, `jjx_send`, `jjx_relay`, `jjx_check`, `jjx_fetch`, `jjx_plant`)

**Fundus profile specifications** — each profile is a named configuration:

Happy-path profiles:
- `hairpin` — localhost, dedicated test user, real repo clone at RELDIR. Full happy-path suite: bind, send, relay (parallel dispatch of N concurrent BUK invocations), check, fetch, plant.
- `cerebro` — remote machine, same expectations as hairpin. Validates that nothing is accidentally localhost-dependent.

Failure-mode profiles (localhost only, one broken property each):
- No SSH key → bind auth failure
- Good SSH, no repo at RELDIR → bind Layer 3 probe failure
- Good SSH, repo exists, no git → plant failure, send still works
- Additional profiles as warranted by design conversation

**For each profile, specify:**
- Host, user, RELDIR expectations
- What is deliberately configured vs deliberately missing
- What test assertions the profile supports
- Setup procedure (one-time manual steps to create the account/environment)

**Tabtarget surface design:**
- `jjw-tR` colophon with imprint channel (one tabtarget per fundus profile)
- Tabtarget wraps `cargo test` with appropriate filter/feature flags
- Define how imprint maps to fundus profile selection at runtime

**Preflight contract:**
- Each profile has a preflight probe (SSH attempt, RELDIR check) that skips the suite cleanly if preconditions aren't met
- Preflight failures are informational, not errors — a machine may not have all profiles configured

**Does NOT include:** Writing the Rust test code. This pace produces the specification; ₢A4AAG implements against it.

**Depends on:** ₢A4AAB (JJS0 remote dispatch vocabulary must exist for quoin cross-references)

**[260404-0917] rough**

## Character
Integration testing across SSH boundary. Requires a configured fundus (or hairpin localhost with separate user). Covers the basic happy-path and core error scenarios. Advanced failure/edge cases planned separately in a dedicated pace.

## Docket
End-to-end integration tests for the remote dispatch command surface — basic scenarios.

**Test scenarios:**
- `jjx_bind` + `jjx_send`: bind to fundus, run simple command, verify output
- `jjx_bind` failure: bad host, bad RELDIR, SSH not configured
- `jjx_plant`: reset fundus to known commit, verify working tree state
- `jjx_relay` + `jjx_check` (timeout=0): launch async job, instant probe shows running
- `jjx_relay` + `jjx_check` (timeout>0): launch short job, await completion, verify stopped + exit code
- `jjx_check` file list: verify temp dir contents returned on terminal status
- `jjx_fetch`: read specific file from fundus using absolute path from BURX_TEMP_DIR
- BURX_LABEL correlation: verify pensum token round-trips through BURE_LABEL → BURX_LABEL

**Infrastructure considerations:**
- Hairpin testing (localhost, separate user) is simplest for CI
- Tests must be sequential (shared fundus state)
- Fundus preconditions must be met before test suite runs — fail fast if not

**Deferred to ₢A4AAG:** Advanced failure/edge cases (orphaned, lost, plant failures, multiple legationes, fetch relative vs absolute paths). Planned interactively.

**Depends on:** All prior paces (₢A4AAA through ₢A4AAD)

**[260404-0855] rough**

## Character
Integration testing across SSH boundary. Requires a configured fundus (or hairpin localhost with separate user). May need test infrastructure scaffolding. Last pace — validates the full stack.

## Docket
End-to-end integration tests for the remote dispatch command surface.

**Test scenarios:**
- `jjx_bind` + `jjx_send`: bind to fundus, run simple command, verify output
- `jjx_bind` failure: bad host, bad RELDIR, SSH not configured
- `jjx_plant`: reset fundus to known commit, verify working tree state
- `jjx_relay` + `jjx_check` (timeout=0): launch async job, instant probe shows running
- `jjx_relay` + `jjx_check` (timeout>0): launch short job, await completion, verify stopped + exit code
- `jjx_check` file list: verify temp dir contents returned on terminal status
- `jjx_fetch`: read specific file from fundus using absolute path from BURX_TEMP_DIR
- `jjx_check` orphaned: simulate crash (kill process before BURX_EXIT_STATUS written)
- `jjx_check` lost: probe nonexistent temp dir
- BURX_LABEL correlation: verify pensum token round-trips through BURE_LABEL → BURX_LABEL

**Infrastructure considerations:**
- Hairpin testing (localhost, separate user) is simplest for CI
- Tests must be sequential (shared fundus state)
- Fundus preconditions must be met before test suite runs — fail fast if not

**Depends on:** All prior paces (₢A4AAA through ₢A4AAD)

### remote-dispatch-integration-tests (₢A4AAG) [complete]

**[260404-1559] complete**

## Character
Standard Rust development — sonnet-tier. The spec from ₢A4AAE defines what to build; this pace implements it. Moderate judgment on test harness ergonomics, but the scenarios and assertions are specified.

## Docket
Implement Rust integration tests for remote dispatch, executing against the fundus profiles specified in ₢A4AAE's JJS0 subdocument.

**Test infrastructure:**
- Integration test module in JJK crate (e.g., `tests/remote.rs` or `jjt_remote.rs` in-crate)
- Fundus profile configuration: read from imprint, resolve to (host, user, reldir, expected_behavior)
- Preflight guard per profile: probe SSH, skip suite if preconditions not met

**Happy-path test scenarios (hairpin + cerebro profiles):**
- `bind` + `send`: bind to fundus, run simple command, verify stdout capture
- `relay` + `check` (instant): launch async job, immediate probe shows running
- `relay` + `check` (poll): launch short job, poll to completion, verify stopped + exit code
- `relay` parallel: dispatch 3-5 concurrent BUK invocations, check each independently, verify all complete with correct BURX state and no cross-contamination
- `plant`: reset fundus to known commit, verify working tree state via send
- `fetch`: read specific file, verify content (both RELDIR-relative and absolute paths)
- BURX_LABEL correlation: verify pensum round-trips through BURE_LABEL -> BURX_LABEL

**Failure-mode test scenarios (broken profiles):**
- Each misconfigured profile: bind or command fails with expected error class
- Verify errors are clean (no panics, no partial state left behind)

**Tabtarget enrollment:**
- `jjw-tR` colophon, imprint channel, one tabtarget per configured profile
- Workbench routing in `jjw_workbench.sh` (or new workbench if jjw doesn't exist yet)
- Zipper registration

**Depends on:** ₢A4AAE (spec), ₢A4AAC (SSH infrastructure), ₢A4AAD (async dispatch), ₢A4AAJ (pensum identity type)

**[260404-1418] rough**

## Character
Standard Rust development — sonnet-tier. The spec from ₢A4AAE defines what to build; this pace implements it. Moderate judgment on test harness ergonomics, but the scenarios and assertions are specified.

## Docket
Implement Rust integration tests for remote dispatch, executing against the fundus profiles specified in ₢A4AAE's JJS0 subdocument.

**Test infrastructure:**
- Integration test module in JJK crate (e.g., `tests/remote.rs` or `jjt_remote.rs` in-crate)
- Fundus profile configuration: read from imprint, resolve to (host, user, reldir, expected_behavior)
- Preflight guard per profile: probe SSH, skip suite if preconditions not met

**Happy-path test scenarios (hairpin + cerebro profiles):**
- `bind` + `send`: bind to fundus, run simple command, verify stdout capture
- `relay` + `check` (instant): launch async job, immediate probe shows running
- `relay` + `check` (poll): launch short job, poll to completion, verify stopped + exit code
- `relay` parallel: dispatch 3-5 concurrent BUK invocations, check each independently, verify all complete with correct BURX state and no cross-contamination
- `plant`: reset fundus to known commit, verify working tree state via send
- `fetch`: read specific file, verify content (both RELDIR-relative and absolute paths)
- BURX_LABEL correlation: verify pensum round-trips through BURE_LABEL -> BURX_LABEL

**Failure-mode test scenarios (broken profiles):**
- Each misconfigured profile: bind or command fails with expected error class
- Verify errors are clean (no panics, no partial state left behind)

**Tabtarget enrollment:**
- `jjw-tR` colophon, imprint channel, one tabtarget per configured profile
- Workbench routing in `jjw_workbench.sh` (or new workbench if jjw doesn't exist yet)
- Zipper registration

**Depends on:** ₢A4AAE (spec), ₢A4AAC (SSH infrastructure), ₢A4AAD (async dispatch), ₢A4AAJ (pensum identity type)

**[260404-1418] rough**

## Character
Standard Rust development — sonnet-tier. The spec from ₢A4AAE defines what to build; this pace implements it. Moderate judgment on test harness ergonomics, but the scenarios and assertions are specified.

## Docket
Implement Rust integration tests for remote dispatch, executing against the fundus profiles specified in ₢A4AAE's JJS0 subdocument.

**Test infrastructure:**
- Integration test module in JJK crate (e.g., `tests/remote.rs` or `jjt_remote.rs` in-crate)
- Fundus profile configuration: read from imprint, resolve to (host, user, reldir, expected_behavior)
- Preflight guard per profile: probe SSH, skip suite if preconditions not met

**Happy-path test scenarios (hairpin + cerebro profiles):**
- `bind` + `send`: bind to fundus, run simple command, verify stdout capture
- `relay` + `check` (instant): launch async job, immediate probe shows running
- `relay` + `check` (poll): launch short job, poll to completion, verify stopped + exit code
- `relay` parallel: dispatch 3-5 concurrent BUK invocations, check each independently, verify all complete with correct BURX state and no cross-contamination
- `plant`: reset fundus to known commit, verify working tree state via send
- `fetch`: read specific file, verify content (both RELDIR-relative and absolute paths)
- BURX_LABEL correlation: verify pensum round-trips through BURE_LABEL -> BURX_LABEL

**Failure-mode test scenarios (broken profiles):**
- Each misconfigured profile: bind or command fails with expected error class
- Verify errors are clean (no panics, no partial state left behind)

**Tabtarget enrollment:**
- `jjw-tR` colophon, imprint channel, one tabtarget per configured profile
- Workbench routing in `jjw_workbench.sh` (or new workbench if jjw doesn't exist yet)
- Zipper registration

**Depends on:** ₢A4AAE (spec), ₢A4AAC (SSH infrastructure), ₢A4AAD (async dispatch), ₢A4AAJ (pensum identity type)

**[260404-0918] rough**

## Character
Interactive planning conversation. Sophisticated failure modes and edge cases need careful thought about how to simulate them reliably in a test harness. Not mechanical — requires judgment about what's testable, what's worth testing, and what infrastructure is needed.

## Docket
Plan and implement advanced test cases for remote dispatch — failure modes, edge cases, and multi-resource scenarios deferred from ₢A4AAE.

**Cases to plan:**
- `jjx_check` orphaned: simulate crash (kill process before BURX_EXIT_STATUS written) — how to reliably trigger and detect?
- `jjx_check` lost: probe nonexistent temp dir — straightforward but needs clean setup
- `jjx_plant` failures: nonexistent commit, no `.git` at RELDIR, `git fetch` network failure — which are worth simulating?
- Multiple legationes: bind to two different targets (or same host, different reldirs), operate on each independently
- `jjx_fetch` relative vs absolute paths: both modes specified, both should be tested
- `jjx_relay` timeout expiry: dispatch a long-running job, check with short timeout, verify running report
- `jjx_send` with shell-quoting edge cases: commands with spaces, quotes, special characters
- `jjx_bind` with stale legatio: what happens when fundus reboots between bind and send?

**Outcome:** Concrete test case specifications added to the test fixture, then implemented.

**Depends on:** ₢A4AAE (basic tests working first)

### jjs0-remote-dispatch-spec (₢A4AAB) [complete]

**[260404-1317] complete**

## Character
Design conversation requiring judgment. The paddock captures the design decisions, but formalizing them into JJS0's existing document structure requires careful quoin treatment, vocabulary precision, and attention to how the new commands interact with existing JJK concepts (officium, gazette, identity tiers). Expect iterative refinement with the user.

## Docket
Formalize the remote dispatch command surface in JJS0 spec.

**New command definitions:**
- `jjx_bind` — create legatio session with SSH probe + RELDIR safety
- `jjx_send` — synchronous remote exec (single string, bash -c on fundus)
- `jjx_relay` — async dispatch via nohup, mint pensum, two-SSH mechanism
- `jjx_check` — probe/poll pensum status (required timeout: 0=instant, >0=poll), file list on terminal
- `jjx_fetch` — single file read from fundus (relative to RELDIR or absolute, raw bytes)
- `jjx_plant` — reset fundus workspace to exact commit

**New vocabulary requiring quoin treatment:**
- Curia / Fundus role vocabulary
- Legatio (session identity) / Pensum (job identity) two-tier model
- BURX regime cross-reference (defined in BUS0, consumed here)

**Additional deliverable:** Update `jjk-claude-context.md` — add new commands to MCP command reference table, verb table, and param schemas so LLMs in future sessions can invoke them.

**Structural considerations:**
- Where do these commands land in JJS0's existing organization?
- How do legatio/pensum relate to officium lifecycle (absolution cleans up legationes)?
- BURE_LABEL as the bridge between JJK pensum tokens and BUK's BURX_LABEL

**See paddock** for full design decision record, command surface table, and provenance.

**[260404-0917] rough**

## Character
Design conversation requiring judgment. The paddock captures the design decisions, but formalizing them into JJS0's existing document structure requires careful quoin treatment, vocabulary precision, and attention to how the new commands interact with existing JJK concepts (officium, gazette, identity tiers). Expect iterative refinement with the user.

## Docket
Formalize the remote dispatch command surface in JJS0 spec.

**New command definitions:**
- `jjx_bind` — create legatio session with SSH probe + RELDIR safety
- `jjx_send` — synchronous remote exec (single string, bash -c on fundus)
- `jjx_relay` — async dispatch via nohup, mint pensum, two-SSH mechanism
- `jjx_check` — probe/poll pensum status (required timeout: 0=instant, >0=poll), file list on terminal
- `jjx_fetch` — single file read from fundus (relative to RELDIR or absolute, raw bytes)
- `jjx_plant` — reset fundus workspace to exact commit

**New vocabulary requiring quoin treatment:**
- Curia / Fundus role vocabulary
- Legatio (session identity) / Pensum (job identity) two-tier model
- BURX regime cross-reference (defined in BUS0, consumed here)

**Additional deliverable:** Update `jjk-claude-context.md` — add new commands to MCP command reference table, verb table, and param schemas so LLMs in future sessions can invoke them.

**Structural considerations:**
- Where do these commands land in JJS0's existing organization?
- How do legatio/pensum relate to officium lifecycle (absolution cleans up legationes)?
- BURE_LABEL as the bridge between JJK pensum tokens and BUK's BURX_LABEL

**See paddock** for full design decision record, command surface table, and provenance.

**[260404-0854] rough**

## Character
Design conversation requiring judgment. The paddock captures the design decisions, but formalizing them into JJS0's existing document structure requires careful quoin treatment, vocabulary precision, and attention to how the new commands interact with existing JJK concepts (officium, gazette, identity tiers). Expect iterative refinement with the user.

## Docket
Formalize the remote dispatch command surface in JJS0 spec.

**New command definitions:**
- `jjx_bind` — create legatio session with SSH probe + RELDIR safety
- `jjx_send` — synchronous remote exec
- `jjx_relay` — async dispatch via nohup, mint pensum
- `jjx_check` — probe/poll pensum status (required timeout: 0=instant, >0=poll), file list on terminal
- `jjx_fetch` — single file read from fundus (relative to RELDIR or absolute)
- `jjx_plant` — reset fundus workspace to exact commit

**New vocabulary requiring quoin treatment:**
- Curia / Fundus role vocabulary
- Legatio (session identity) / Pensum (job identity) two-tier model
- BURX regime cross-reference (defined in BUS0, consumed here)

**Structural considerations:**
- Where do these commands land in JJS0's existing organization?
- How do legatio/pensum relate to officium lifecycle (absolution cleans up legationes)?
- BURE_LABEL as the bridge between JJK pensum tokens and BUK's BURX_LABEL

**See paddock** for full design decision record, command surface table, and provenance.

### remote-dispatch-procedure-specs (₢A4AAK) [complete]

**[260405-0833] complete**

## Character
Mechanical spec writing against a resolved design. Each of the 6 remote dispatch commands needs a procedure specification following the established JJS0 operation pattern (Arguments, Behavior, Exit Status, Stdout). Template-filling, not design — all vocabulary, types, and structural decisions are resolved in ₢A4AAB.

## Docket
Write per-procedure specifications for the 6 remote dispatch commands in JJS0.

**Commands to specify:**
- `jjx_bind` — create legatio handle (SSH probe + RELDIR safety)
- `jjx_send` — synchronous remote exec
- `jjx_relay` — async dispatch via nohup, mint pensum
- `jjx_check` — probe/poll pensum status
- `jjx_fetch` — single file read from fundus
- `jjx_plant` — reset fundus workspace to exact commit

**Each command gets:**
- `[[jjdo_*]]` anchor with `//axvo_procedure axd_transient` voicing
- Arguments section with shared argument references (`{jjda_legatio}`, `{jjda_timeout}`, etc.)
- Behavior section (numbered steps)
- Exit Status section
- Stdout section

**Pattern to follow:** Existing operations like `jjdo_enroll` (slate.adoc), `jjdo_record` (notch.adoc). Each command may warrant its own include file or may be inline — decide during writing.

**Depends on:** ₢A4AAB (vocabulary, types, structural placement all resolved first)

**[260404-1255] rough**

## Character
Mechanical spec writing against a resolved design. Each of the 6 remote dispatch commands needs a procedure specification following the established JJS0 operation pattern (Arguments, Behavior, Exit Status, Stdout). Template-filling, not design — all vocabulary, types, and structural decisions are resolved in ₢A4AAB.

## Docket
Write per-procedure specifications for the 6 remote dispatch commands in JJS0.

**Commands to specify:**
- `jjx_bind` — create legatio handle (SSH probe + RELDIR safety)
- `jjx_send` — synchronous remote exec
- `jjx_relay` — async dispatch via nohup, mint pensum
- `jjx_check` — probe/poll pensum status
- `jjx_fetch` — single file read from fundus
- `jjx_plant` — reset fundus workspace to exact commit

**Each command gets:**
- `[[jjdo_*]]` anchor with `//axvo_procedure axd_transient` voicing
- Arguments section with shared argument references (`{jjda_legatio}`, `{jjda_timeout}`, etc.)
- Behavior section (numbered steps)
- Exit Status section
- Stdout section

**Pattern to follow:** Existing operations like `jjdo_enroll` (slate.adoc), `jjdo_record` (notch.adoc). Each command may warrant its own include file or may be inline — decide during writing.

**Depends on:** ₢A4AAB (vocabulary, types, structural placement all resolved first)

### jjk-mcp-command-name-const-registry (₢A4AAH) [complete]

**[260404-1559] complete**

## Character
Mechanical refactoring — no judgment, no new behavior. Every change is a grep-verifiable string-to-const substitution.

## Docket
Migrate all MCP command name magic strings across JJK Rust source to RCG-compliant const definitions. Two scopes:

**Scope 1 — Dispatcher (`jjrm_mcp.rs`):**
- Define `JJRM_CMD_NAME_*` consts for all ~26 pre-existing commands (open, list, show, orient, record, log, validate, create, enroll, close, archive, reorder, redocket, relabel, drop, relocate, alter, search, brief, coronets, paddock, continue, transfer, landing, chapter, absolve)
- Replace all match arm string literals with the corresponding const
- Build `JJRM_ALL_COMMANDS` array from all consts (replacing `JJRM_LEGATIO_COMMANDS`)
- Replace the unknown-command error listing with `JJRM_ALL_COMMANDS.join(", ")`
- Update the schemars description if a compile-time approach is feasible; otherwise document the proc-macro limitation

**Scope 2 — Handler files (~168 occurrences across 24 files):**
- Each handler file defines its own `{PREFIX}_CMD_NAME_{COMMAND}` const (e.g., `JJRGS_CMD_NAME_GET_SPEC`)
- All `vvco_err!` / `vvco_out!` calls referencing the command name use the const via `let cn = ...;` pattern (as established in `jjrlg_legatio.rs`)
- Pattern to follow: `jjrlg_legatio.rs` which already uses `JJRLG_CMD_NAME_BIND` etc.

**Verification:** `grep '"jjx_'` across `Tools/jjk/vov_veiled/src/` finds ONLY const definitions and the schemars attribute. Zero bare string literals in match arms, error messages, or output formatting.

**Size estimate:** ~225 occurrences across 25 files. Mechanical but wide — consider parallel agents per file group.

**[260404-1031] rough**

## Character
Mechanical refactoring — no judgment, no new behavior. Every change is a grep-verifiable string-to-const substitution.

## Docket
Migrate all MCP command name magic strings across JJK Rust source to RCG-compliant const definitions. Two scopes:

**Scope 1 — Dispatcher (`jjrm_mcp.rs`):**
- Define `JJRM_CMD_NAME_*` consts for all ~26 pre-existing commands (open, list, show, orient, record, log, validate, create, enroll, close, archive, reorder, redocket, relabel, drop, relocate, alter, search, brief, coronets, paddock, continue, transfer, landing, chapter, absolve)
- Replace all match arm string literals with the corresponding const
- Build `JJRM_ALL_COMMANDS` array from all consts (replacing `JJRM_LEGATIO_COMMANDS`)
- Replace the unknown-command error listing with `JJRM_ALL_COMMANDS.join(", ")`
- Update the schemars description if a compile-time approach is feasible; otherwise document the proc-macro limitation

**Scope 2 — Handler files (~168 occurrences across 24 files):**
- Each handler file defines its own `{PREFIX}_CMD_NAME_{COMMAND}` const (e.g., `JJRGS_CMD_NAME_GET_SPEC`)
- All `vvco_err!` / `vvco_out!` calls referencing the command name use the const via `let cn = ...;` pattern (as established in `jjrlg_legatio.rs`)
- Pattern to follow: `jjrlg_legatio.rs` which already uses `JJRLG_CMD_NAME_BIND` etc.

**Verification:** `grep '"jjx_'` across `Tools/jjk/vov_veiled/src/` finds ONLY const definitions and the schemars attribute. Zero bare string literals in match arms, error messages, or output formatting.

**Size estimate:** ~225 occurrences across 25 files. Mechanical but wide — consider parallel agents per file group.

**[260404-1009] rough**

## Character
Mechanical refactoring — no judgment, no new behavior. Every change is a grep-verifiable string-to-const substitution.

## Docket
Migrate all pre-existing MCP command name strings in `jjrm_mcp.rs` to RCG-compliant const definitions.

**Problem:** ~26 command names appear as bare string literals in match arms and in the unknown-command error listing. RCG String Boundary Discipline requires a single const definition per serialized string. The legatio commands (bind/send/plant/fetch) were added with consts and a `JJRM_LEGATIO_COMMANDS` array as the pattern to follow.

**Work:**
- Define `JJRM_CMD_*` consts for all remaining commands (open, list, show, orient, record, log, validate, create, enroll, close, archive, reorder, redocket, relabel, drop, relocate, alter, search, brief, coronets, paddock, continue, transfer, landing, chapter, absolve)
- Replace all match arm string literals with the corresponding const
- Build `JJRM_ALL_COMMANDS` array from all consts (replacing `JJRM_LEGATIO_COMMANDS`)
- Replace the unknown-command error listing with `JJRM_ALL_COMMANDS.join(", ")`
- Update the schemars description if a compile-time approach is feasible; otherwise document the proc-macro limitation
- Verify: `grep '"jjx_'` in jjrm_mcp.rs finds only the const definitions and the schemars attribute

### rbk-fact-file-producer-migration (₢A4AAL) [complete]

**[260405-1022] complete**

## Character
Mechanical migration — no judgment, no new behavior. Every change is a grep-verifiable substitution of raw `echo > path` writes with the `buf_write_fact` primitive.

## Docket
Migrate all existing RBK fact-file producers from raw `echo > BURD_OUTPUT_DIR` writes to the `buf_write_fact` dual-write primitive introduced in ₢A4AAA.

**Context:** ₢A4AAA introduced the `buf_write_fact` module providing dual-write (to both `BURD_OUTPUT_DIR` and `BURD_TEMP_DIR`). But existing RBK producers still use raw `echo` to `BURD_OUTPUT_DIR` only. Until migrated, those fact-files are not available via the durable temp-dir path that remote consumers (BURX) need.

**Scope:**
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh` — ~15 raw `echo "${value}" > "${BURD_OUTPUT_DIR}/${CONSTANT}"` sites for `RBF_FACT_HALLMARK`, `RBF_FACT_BUILD_ID`, `RBF_FACT_GAR_ROOT`, `RBF_FACT_ARK_STEM`, `RBF_FACT_ARK_YIELD`
- `Tools/rbk/rbfl_FoundryLedger.sh` — tally fact-file producer (`RBCC_FACT_CONSEC_INFIX`)
- Any other RBK files with raw fact-file writes discovered during grep

**Method:** `grep -rn 'BURD_OUTPUT_DIR' Tools/rbk/` to find all sites. Replace each with `buf_write_fact` call. Verify `buf_write_fact` is sourced where needed.

**Verification:** `grep 'BURD_OUTPUT_DIR' Tools/rbk/` finds zero raw writes — only `buf_write_fact` references remain.

**Depends on:** ₢A4AAA (buf_write_fact primitive must exist)

**[260404-1601] rough**

## Character
Mechanical migration — no judgment, no new behavior. Every change is a grep-verifiable substitution of raw `echo > path` writes with the `buf_write_fact` primitive.

## Docket
Migrate all existing RBK fact-file producers from raw `echo > BURD_OUTPUT_DIR` writes to the `buf_write_fact` dual-write primitive introduced in ₢A4AAA.

**Context:** ₢A4AAA introduced the `buf_write_fact` module providing dual-write (to both `BURD_OUTPUT_DIR` and `BURD_TEMP_DIR`). But existing RBK producers still use raw `echo` to `BURD_OUTPUT_DIR` only. Until migrated, those fact-files are not available via the durable temp-dir path that remote consumers (BURX) need.

**Scope:**
- `Tools/rbk/rbfd_FoundryDirectorBuild.sh` — ~15 raw `echo "${value}" > "${BURD_OUTPUT_DIR}/${CONSTANT}"` sites for `RBF_FACT_HALLMARK`, `RBF_FACT_BUILD_ID`, `RBF_FACT_GAR_ROOT`, `RBF_FACT_ARK_STEM`, `RBF_FACT_ARK_YIELD`
- `Tools/rbk/rbfl_FoundryLedger.sh` — tally fact-file producer (`RBCC_FACT_CONSEC_INFIX`)
- Any other RBK files with raw fact-file writes discovered during grep

**Method:** `grep -rn 'BURD_OUTPUT_DIR' Tools/rbk/` to find all sites. Replace each with `buf_write_fact` call. Verify `buf_write_fact` is sourced where needed.

**Verification:** `grep 'BURD_OUTPUT_DIR' Tools/rbk/` finds zero raw writes — only `buf_write_fact` references remain.

**Depends on:** ₢A4AAA (buf_write_fact primitive must exist)

### spec-source-alignment-audit (₢A4AAF) [complete]

**[260407-2051] complete**

## Character
Painstaking verification. Requires reading every spec definition against every source implementation, line by line. No creativity — pure discipline. The kind of work where missing one field name mismatch means a runtime bug that's hard to trace.

## Docket
Systematic audit that spec documents and source code are perfectly aligned after all implementation paces complete.

**BUS0 ↔ BUK source:**
- Every BURX field name in BUS0 matches constants in source
- BURE_LABEL kindle definition matches BUC implementation
- `buf_write_fact` spec description matches module implementation
- Dual-write destinations match (BURD_OUTPUT_DIR + BURD_TEMP_DIR)
- burx.env write points in BUD match spec (start fields vs completion fields)
- BURX_LABEL always present (empty string when BURE_LABEL absent)

**JJS0 ↔ JJK source:**
- Every command param in JJS0 matches Rust handler signature
- Legatio/pensum identity formats match between spec and token minting code
- jjx_check return shape matches spec (BURX fields + liveness report + file list on terminal)
- jjx_fetch path resolution (relative vs absolute) matches spec
- jjx_fetch handles binary files (raw bytes) per spec
- jjx_plant fundus-side sequence matches spec
- jjx_relay two-SSH mechanism matches spec (dispatch + read burx.env from output dir)
- jjx_send command interpretation matches spec (single string, bash -c on fundus)
- Curia/fundus vocabulary used consistently (no master/slave leaking through)

**jjk-claude-context.md ↔ JJK source:**
- New commands present in MCP command reference with correct param schemas
- Verb table entries match actual command names
- No stale references to pre-implementation design

**Cross-spec:**
- JJS0 references to BURX fields match BUS0 definitions
- BURE_LABEL flow: JJS0 pensum → BURE_LABEL → BUS0 BURX_LABEL chain is consistent

**Paddock ↔ specs:**
- Design decisions in paddock are faithfully represented in both specs
- No paddock decisions lost or contradicted

**Method:** Read spec section, read corresponding source, note any divergence. Fix or flag.

**[260404-0917] rough**

## Character
Painstaking verification. Requires reading every spec definition against every source implementation, line by line. No creativity — pure discipline. The kind of work where missing one field name mismatch means a runtime bug that's hard to trace.

## Docket
Systematic audit that spec documents and source code are perfectly aligned after all implementation paces complete.

**BUS0 ↔ BUK source:**
- Every BURX field name in BUS0 matches constants in source
- BURE_LABEL kindle definition matches BUC implementation
- `buf_write_fact` spec description matches module implementation
- Dual-write destinations match (BURD_OUTPUT_DIR + BURD_TEMP_DIR)
- burx.env write points in BUD match spec (start fields vs completion fields)
- BURX_LABEL always present (empty string when BURE_LABEL absent)

**JJS0 ↔ JJK source:**
- Every command param in JJS0 matches Rust handler signature
- Legatio/pensum identity formats match between spec and token minting code
- jjx_check return shape matches spec (BURX fields + liveness report + file list on terminal)
- jjx_fetch path resolution (relative vs absolute) matches spec
- jjx_fetch handles binary files (raw bytes) per spec
- jjx_plant fundus-side sequence matches spec
- jjx_relay two-SSH mechanism matches spec (dispatch + read burx.env from output dir)
- jjx_send command interpretation matches spec (single string, bash -c on fundus)
- Curia/fundus vocabulary used consistently (no master/slave leaking through)

**jjk-claude-context.md ↔ JJK source:**
- New commands present in MCP command reference with correct param schemas
- Verb table entries match actual command names
- No stale references to pre-implementation design

**Cross-spec:**
- JJS0 references to BURX fields match BUS0 definitions
- BURE_LABEL flow: JJS0 pensum → BURE_LABEL → BUS0 BURX_LABEL chain is consistent

**Paddock ↔ specs:**
- Design decisions in paddock are faithfully represented in both specs
- No paddock decisions lost or contradicted

**Method:** Read spec section, read corresponding source, note any divergence. Fix or flag.

**[260404-0857] rough**

## Character
Painstaking verification. Requires reading every spec definition against every source implementation, line by line. No creativity — pure discipline. The kind of work where missing one field name mismatch means a runtime bug that's hard to trace.

## Docket
Systematic audit that spec documents and source code are perfectly aligned after all implementation paces complete.

**BUS0 ↔ BUK source:**
- Every BURX field name in BUS0 matches constants in source
- BURE_LABEL kindle definition matches BUC implementation
- `buf_write_fact` spec description matches module implementation
- Dual-write destinations match (BURD_OUTPUT_DIR + BURD_TEMP_DIR)
- burx.env write points in BUD match spec (start fields vs completion fields)

**JJS0 ↔ JJK source:**
- Every command param in JJS0 matches Rust handler signature
- Legatio/pensum identity formats match between spec and token minting code
- jjx_check return shape matches spec (BURX fields + liveness report + file list on terminal)
- jjx_fetch path resolution (relative vs absolute) matches spec
- jjx_plant fundus-side sequence matches spec
- jjx_relay nohup wrapper sets BURE_LABEL per spec
- Curia/fundus vocabulary used consistently (no master/slave leaking through)

**Cross-spec:**
- JJS0 references to BURX fields match BUS0 definitions
- BURE_LABEL flow: JJS0 pensum → BURE_LABEL → BUS0 BURX_LABEL chain is consistent

**Paddock ↔ specs:**
- Design decisions in paddock are faithfully represented in both specs
- No paddock decisions lost or contradicted

**Method:** Read spec section, read corresponding source, note any divergence. Fix or flag.

### equestrian-verb-reconciliation (₢A4AAI) [abandoned]

**[260405-1234] abandoned**

## Character
Design conversation. The remote dispatch commands introduced non-equestrian vocabulary (bind, send, relay, check, fetch, plant). Need to decide: do these get jjsuv_* verb mappings, or does JJS0 formally acknowledge a second vocabulary domain?

## Docket
Reconcile remote dispatch command names with the equestrian verb layer (jjsuv_*).

**Question**: Remote dispatch commands use their own domain vocabulary rather than equestrian metaphors. Three options:
1. Mint equestrian verbs that map to the remote operations (forced metaphor)
2. Formally define a second verb domain in JJS0 for remote operations
3. Declare remote operations exempt from the verb layer (they're infrastructure, not gallops operations)

**Depends on:** ₢A4AAB (spec work establishes the operations first)

**[260404-1205] rough**

## Character
Design conversation. The remote dispatch commands introduced non-equestrian vocabulary (bind, send, relay, check, fetch, plant). Need to decide: do these get jjsuv_* verb mappings, or does JJS0 formally acknowledge a second vocabulary domain?

## Docket
Reconcile remote dispatch command names with the equestrian verb layer (jjsuv_*).

**Question**: Remote dispatch commands use their own domain vocabulary rather than equestrian metaphors. Three options:
1. Mint equestrian verbs that map to the remote operations (forced metaphor)
2. Formally define a second verb domain in JJS0 for remote operations
3. Declare remote operations exempt from the verb layer (they're infrastructure, not gallops operations)

**Depends on:** ₢A4AAB (spec work establishes the operations first)

### evaluate-arcanum-removal (₢A4AAP) [complete]

**[260405-1037] complete**

## Character
Investigative — survey what arcanum still provides vs what MCP has superseded. Low-risk decision with clear success criteria.

## Docket
Evaluate whether `jja_arcanum.sh` and its supporting files (`jjl_ledger.json`, emitted slash commands) can be fully removed now that JJK operates via the `mcp__vvx__jjx` MCP tool.

**Survey:**
1. Identify all artifacts arcanum creates: `.claude/commands/jjc-*.md`, `.claude/jjm/` directories, `settings.local.json` permissions, ledger entries
2. Determine which are still consumed by the MCP architecture vs dead weight
3. Check if the `.claude/jjm/` directory creation and settings permissions are handled elsewhere (e.g., by VVK install)

**Decision:** Remove arcanum entirely, or strip it to only the parts still needed.

**Cleanup if removing:**
- Delete `jja_arcanum.sh`, `jjl_ledger.json`
- Remove arcanum routing from workbench (already removed from zipper)
- Update CLAUDE.md file acronym mappings

**[260405-0907] rough**

## Character
Investigative — survey what arcanum still provides vs what MCP has superseded. Low-risk decision with clear success criteria.

## Docket
Evaluate whether `jja_arcanum.sh` and its supporting files (`jjl_ledger.json`, emitted slash commands) can be fully removed now that JJK operates via the `mcp__vvx__jjx` MCP tool.

**Survey:**
1. Identify all artifacts arcanum creates: `.claude/commands/jjc-*.md`, `.claude/jjm/` directories, `settings.local.json` permissions, ledger entries
2. Determine which are still consumed by the MCP architecture vs dead weight
3. Check if the `.claude/jjm/` directory creation and settings permissions are handled elsewhere (e.g., by VVK install)

**Decision:** Remove arcanum entirely, or strip it to only the parts still needed.

**Cleanup if removing:**
- Delete `jja_arcanum.sh`, `jjl_ledger.json`
- Remove arcanum routing from workbench (already removed from zipper)
- Update CLAUDE.md file acronym mappings

### move-pensum-seed-to-officium (₢A4AAR) [complete]

**[260407-2127] complete**

## Character
Surgical plumbing change. Mechanical once the design is clear — move a field from one storage location to another, update all read/write sites, update spec. The tricky part is ensuring token uniqueness without git-tracked state.

## Docket
Move pensum seed from gallops (git-tracked, commits on every mint) to officium-local storage (ephemeral, no commit).

**Problem**: `jjx_relay` mints a pensum by incrementing `pensum_seed` on the heat in gallops under commit lock. Each mint creates a git commit, advancing HEAD past origin. The curia readiness guard (also on relay) then refuses subsequent relays because HEAD is unpushed. Pensum tokens are session-scoped ephemeral correlation tokens — they don't belong in permanent project history.

**Fix — Rust (`jjrlg_legatio.rs` / `jjrm_mcp.rs`)**:
- Move pensum seed from `jjrg_Heat.pensum_seed` to a file in the officium directory (e.g., `pensum_seed.json` keyed by firemark)
- `zjjrlg_mint_pensum_locked` no longer acquires gallops lock or commits — reads/writes officium-local file
- Remove gallops lock acquisition from the relay dispatch path entirely
- Pensum tokens remain unique within an officium; cross-officium collision is acceptable (tokens are ephemeral correlation labels, not permanent identifiers)

**Fix — JJS0 (`JJS0_JobJockeySpec.adoc`)**:
- Update `jjdo_relay` behavior: pensum minting no longer touches gallops or creates a commit
- Update `jjdhm_pensum_seed` definition: lives in officium exchange directory, not heat metadata
- Remove pensum seed from heat field inventory if listed
- Update any cross-references to pensum minting under gallops lock

**Fix — Test (`jjtlg_fundus_scenario.rs`)**:
- Localhost relay tests should now pass without advancing HEAD
- Verify: run full localhost suite after fix, all 13 non-ignored tests pass

**Confidence gate**: If pensum token format or identity encoding needs to change, stop and flag — that cascades to BURX_LABEL correlation and check/fetch workflows.

**[260407-1032] rough**

## Character
Surgical plumbing change. Mechanical once the design is clear — move a field from one storage location to another, update all read/write sites, update spec. The tricky part is ensuring token uniqueness without git-tracked state.

## Docket
Move pensum seed from gallops (git-tracked, commits on every mint) to officium-local storage (ephemeral, no commit).

**Problem**: `jjx_relay` mints a pensum by incrementing `pensum_seed` on the heat in gallops under commit lock. Each mint creates a git commit, advancing HEAD past origin. The curia readiness guard (also on relay) then refuses subsequent relays because HEAD is unpushed. Pensum tokens are session-scoped ephemeral correlation tokens — they don't belong in permanent project history.

**Fix — Rust (`jjrlg_legatio.rs` / `jjrm_mcp.rs`)**:
- Move pensum seed from `jjrg_Heat.pensum_seed` to a file in the officium directory (e.g., `pensum_seed.json` keyed by firemark)
- `zjjrlg_mint_pensum_locked` no longer acquires gallops lock or commits — reads/writes officium-local file
- Remove gallops lock acquisition from the relay dispatch path entirely
- Pensum tokens remain unique within an officium; cross-officium collision is acceptable (tokens are ephemeral correlation labels, not permanent identifiers)

**Fix — JJS0 (`JJS0_JobJockeySpec.adoc`)**:
- Update `jjdo_relay` behavior: pensum minting no longer touches gallops or creates a commit
- Update `jjdhm_pensum_seed` definition: lives in officium exchange directory, not heat metadata
- Remove pensum seed from heat field inventory if listed
- Update any cross-references to pensum minting under gallops lock

**Fix — Test (`jjtlg_fundus_scenario.rs`)**:
- Localhost relay tests should now pass without advancing HEAD
- Verify: run full localhost suite after fix, all 13 non-ignored tests pass

**Confidence gate**: If pensum token format or identity encoding needs to change, stop and flag — that cascades to BURX_LABEL correlation and check/fetch workflows.

### rcg-internal-prefix-bcg-alignment (₢A4AAS) [complete]

**[260407-2143] complete**

## Character
Careful reading and editing. Requires comparing BCG z-prefix conventions against RCG z-prefix conventions to find gaps, then updating RCG to match. Not creative — alignment work.

## Docket
Audit RCG internal prefix (`z`) conventions against BCG and align. BCG uses `z` prefix for private/internal functions consistently (e.g., `zbuc_color`, `zbud_die`). RCG already has the `z` prefix pattern but may have gaps or inconsistencies relative to BCG's more mature treatment.

Review BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) z-prefix sections, then review RCG (`Tools/vok/vov_veiled/RCG-RustCodingGuide.md`) internal declarations section. Ensure RCG covers:
- Naming pattern alignment with BCG convention
- When to use z-prefix vs true private
- Visibility mapping (`pub(crate)` as Rust equivalent of BCG's file-scoped private)
- Any BCG z-prefix rules that RCG is missing or contradicts

Also while in RCG: upgrade all four `lib.rs` files from `deny(unused_variables)` to `deny(warnings)` per the new RCG boilerplate rule. Verify no new warnings surface.

**[260407-1037] rough**

## Character
Careful reading and editing. Requires comparing BCG z-prefix conventions against RCG z-prefix conventions to find gaps, then updating RCG to match. Not creative — alignment work.

## Docket
Audit RCG internal prefix (`z`) conventions against BCG and align. BCG uses `z` prefix for private/internal functions consistently (e.g., `zbuc_color`, `zbud_die`). RCG already has the `z` prefix pattern but may have gaps or inconsistencies relative to BCG's more mature treatment.

Review BCG (`Tools/buk/vov_veiled/BCG-BashConsoleGuide.md`) z-prefix sections, then review RCG (`Tools/vok/vov_veiled/RCG-RustCodingGuide.md`) internal declarations section. Ensure RCG covers:
- Naming pattern alignment with BCG convention
- When to use z-prefix vs true private
- Visibility mapping (`pub(crate)` as Rust equivalent of BCG's file-scoped private)
- Any BCG z-prefix rules that RCG is missing or contradicts

Also while in RCG: upgrade all four `lib.rs` files from `deny(unused_variables)` to `deny(warnings)` per the new RCG boilerplate rule. Verify no new warnings surface.

## Commit Activity

```
File-touch bitmap (x = pace commit touched file):

  1 M fundus-scenario-revision
  2 N fundus-account-provision-and-smoke
  3 O concurrent-dispatch-scenario
  4 Q fundus-account-prefix-cleanup
  5 A buk-burx-regime-and-dual-write
  6 C jjk-ssh-foundation-sync-commands
  7 J jjk-pensum-identity-type
  8 D jjk-async-dispatch-commands
  9 E remote-dispatch-test-fundus-spec
  10 G remote-dispatch-integration-tests
  11 B jjs0-remote-dispatch-spec
  12 K remote-dispatch-procedure-specs
  13 H jjk-mcp-command-name-const-registry
  14 L rbk-fact-file-producer-migration
  15 F spec-source-alignment-audit
  16 P evaluate-arcanum-removal
  17 R move-pensum-seed-to-officium
  18 S rcg-internal-prefix-bcg-alignment

MNOQACJDEGBKHLFPRS
x·······x·xx··x·x· JJS0_JobJockeySpec.adoc
x····x·x······x·x· jjrlg_legatio.rs
·····x·x····x·x··· jjrm_mcp.rs
·xxx··········x··· jjfp_fundus.sh
xxx···········x··· fundus_scenario.rs
······x·····x···x· jjro_ops.rs
·····x········x··x RCG-RustCodingGuide.md
·····xx··········x lib.rs
x·······x······x·· CLAUDE.md
xx··········x····· jjw_workbench.sh
xx······x········· JJSTF-test-fundus.adoc
·······x········x· jjri_io.rs
······x·········x· jjrg_gallops.rs, jjrt_types.rs, jjrv_validate.rs, jjtfu_furlough.rs, jjtg_gallops.rs, jjtgl_garland.rs, jjtpd_parade.rs, jjtq_query.rs, jjtrl_rail.rs, jjtrs_restring.rs
x···········x····· jjw-tR.TestRemote.cerebro.sh, jjw-tR.TestRemote.hairpin.sh, jjw-tR.TestRemote.nogit.sh, jjw-tR.TestRemote.nokey.sh, jjw-tR.TestRemote.norepo.sh, remote_dispatch.rs
·················x rbida_sorties.rs
···············x·· README.md, jja_arcanum.sh, jjl_ledger.json
··············x··· JJSCCU-curry.adoc, JJSCSL-slate.adoc, JJSCTL-tally.adoc, jjk-claude-context.md, jjtlg_fundus_scenario.rs
·············x···· rbfd_FoundryDirectorBuild.sh, rbfd_cli.sh, rbfl_FoundryLedger.sh, rbfl_cli.sh
············x····· jjrch_chalk.rs, jjrcu_curry.rs, jjrdr_draft.rs, jjrfu_furlough.rs, jjrgc_get_coronets.rs, jjrgl_garland.rs, jjrgs_get_spec.rs, jjrld_landing.rs, jjrmu_muster.rs, jjrnc_notch.rs, jjrno_nominate.rs, jjrpd_parade.rs, jjrrl_rail.rs, jjrrs_restring.rs, jjrrt_retire.rs, jjrs_steeplechase.rs, jjrsc_scout.rs, jjrsd_saddle.rs, jjrsl_slate.rs, jjrtl_tally.rs, jjrvl_validate.rs, jjrwp_wrap.rs
··········x······· AXLA-Lexicon.adoc
······x··········· jjrf_favor.rs, jjtf_favor.rs
····x············· BUS0-BashUtilitiesSpec.adoc, bud_dispatch.sh, buf_fact.sh, bure_regime.sh, butcbe_BureEnvironment.sh, butcbx_BurxExchange.sh, butt_testbench.sh
··x··············· buw-SI.StationInit.sh, buw-xd.Delay.sh, buwz_zipper.sh, bux_cli.sh
·x················ jjf_cli.sh, jjf_fundus.sh, jjfp_cli.sh, jjw-tfP.ProvisionFundusAccounts.cerebro.sh, jjw-tfP.ProvisionFundusAccounts.localhost.sh, jjw-tfP1.ProvisionPhase1.sh, jjw-tfP2.ProvisionPhase2.cerebro.sh, jjw-tfP2.ProvisionPhase2.localhost.sh, jjw-tfs.TestFundusScenario.cerebro.sh, jjz_zipper.sh
x················· jjw-tfS.TestFundusSingle.localhost.sh, jjw-tfs.TestFundusScenario.localhost.sh

Commit swim lanes (x = commit affiliated with pace):

(showing last 35 of 110 commits)

  1 N fundus-account-provision-and-smoke
  2 O concurrent-dispatch-scenario
  3 Q fundus-account-prefix-cleanup
  4 F spec-source-alignment-audit
  5 R move-pensum-seed-to-officium
  6 S rcg-internal-prefix-bcg-alignment

123456789abcdefghijklmnopqrstuvwxyz
xxxxx······························  N  5c
·····xxx·xxx·······················  O  6c
···············xx··················  Q  2c
·················xxxxxxxx·x·x······  F  10c
·····························xxxx··  R  4c
·································xx  S  2c
```

## Steeplechase

### 2026-04-07 21:43 - ₢A4AAS - W

Aligned RCG internal prefix conventions with BCG: extended naming table to show z/Z-prefixed internal patterns for all declaration types (struct, enum, function, constant, trait, type alias); rewrote Internal Declarations section stating universality and BCG local-variable divergence; updated checklist with per-type internal naming rules. Upgraded rbtd and ifrit lib.rs from deny(unused_variables) to deny(warnings); fixed ifrit platform-gated unused params.

### 2026-04-07 21:43 - ₢A4AAS - n

Promote deny(warnings) in RBTD/RBID crates, expand RCG internal naming conventions to cover all declaration types with z/Z prefix patterns

### 2026-04-07 21:27 - ₢A4AAR - W

Moved pensum seed from gallops (git-tracked, commit lock) to officium-local pensum_seeds.json (BTreeMap, no commit). Removed next_pensum_seed from Heat struct, gallops mint function, validation, and test fixtures. Updated JJS0 spec (definition, relay procedure steps, curia-side state description). All 21 fundus scenario tests pass (localhost + cerebro).

### 2026-04-07 21:15 - ₢A4AAR - n

Review repairs: BTreeMap for deterministic pensum_seeds.json; fix stale relay procedure steps in JJS0

### 2026-04-07 21:11 - ₢A4AAR - n

Update JJS0 spec: pensum seed now officium-local, relay bypasses gallops entirely, updated type and member definitions

### 2026-04-07 21:07 - ₢A4AAR - n

Move pensum seed from gallops (git-tracked) to officium-local storage; remove next_pensum_seed from Heat struct, gallops mint function, validation, and test fixtures

### 2026-04-07 20:51 - ₢A4AAF - W

Spec-source alignment audit plus five repairs: removed gazette JSON fallback params (enroll/redocket/paddock gazette-only); folded jjx_chapter/jjx_absolve into jjx_open with exsanguination summary; added curia readiness guard on jjx_relay (dirty tree + unpushed HEAD); documented 6 remote dispatch commands and foray protocol in claude-context.md; added dirty-tree integration test; RCG fixes (SHA derivation from JJRG_UNKNOWN_BASIS, jjtlg_ prefix rename, deny(warnings) boilerplate rule). Discovered pensum-seed-commits-block-relay flaw — slated as A4AAR.

### 2026-04-07 10:37 - Heat - S

rcg-internal-prefix-bcg-alignment

### 2026-04-07 10:36 - ₢A4AAF - n

RCG: require deny(warnings) in all crate roots, not just lib.rs; add boilerplate to integration test file

### 2026-04-07 10:32 - Heat - S

move-pensum-seed-to-officium

### 2026-04-07 10:26 - ₢A4AAF - n

Drop #[ignore] from localhost tests; split into localhost (always-run) and cerebro (ignored) modules; remove JJTEST_HOST env var dependency

### 2026-04-07 10:13 - ₢A4AAF - n

RCG rename: fundus_scenario.rs to jjtlg_fundus_scenario.rs with full jjtlg_ prefix discipline on all declarations

### 2026-04-07 10:04 - ₢A4AAF - n

RCG fix: derive SHA abbreviation length from JJRG_UNKNOWN_BASIS, prefix local variables with z_

### 2026-04-07 10:01 - ₢A4AAF - n

Add relay_refuses_dirty_tree integration test verifying curia readiness guard fires on dirty working tree

### 2026-04-07 09:52 - ₢A4AAF - n

Add remote dispatch commands, foray protocol, and foray verb to claude-context; remove stale gazette fallback language

### 2026-04-07 09:50 - ₢A4AAF - n

Add curia readiness gate to jjx_relay: refuse dispatch if working tree is dirty or HEAD is unpushed

### 2026-04-07 09:17 - ₢A4AAF - n

Fold jjx_chapter and jjx_absolve into jjx_open; exsanguination now reports summary counts in open output

### 2026-04-07 09:08 - ₢A4AAF - n

Remove JSON fallback params from enroll/redocket/paddock; gazette is now the sole input path for docket content

### 2026-04-05 13:11 - ₢A4AAQ - W

Extracted JJFP_account_prefix constant with four derived account constants; replaced Phase 1 deletion with platform-specific prefix enumeration (dscl/getent) catching stale accounts; eliminated all bare jjfu_ magic strings. 12/12 localhost, 12/12 cerebro.

### 2026-04-05 13:05 - ₢A4AAQ - n

Extract JJFP_account_prefix constant with four derived account constants; replace Phase 1 deletion with platform-specific prefix enumeration catching stale accounts; eliminate all bare jjfu_ magic strings

### 2026-04-05 12:54 - Heat - r

moved A4AAQ after A4AAO

### 2026-04-05 12:53 - Heat - r

moved A4AAQ to first

### 2026-04-05 12:53 - Heat - S

fundus-account-prefix-cleanup

### 2026-04-05 12:49 - ₢A4AAO - W

BUK delay tabtarget (buw-xd, 20s sleep) and relay_concurrent_overlap test proving temporal overlap with distinct BURX_LABELs and BURX_TEMP_DIRs. Fixed pre-existing silent-skip bug: ensure_project_root for cargo test CWD, replaced find_racing_firemark with constant firemark, added dispatch smoke preflight catching missing station regime, zjjfp_require_pushed guard for unpushed commits, standalone buw-SI.StationInit for Phase 2 provisioning. 12/12 localhost, 12/12 cerebro.

### 2026-04-05 12:46 - ₢A4AAO - n

Standalone buw-SI.StationInit creates default station regime from burc.env; Phase 2 provisioning calls it on fundus after clone

### 2026-04-05 12:36 - ₢A4AAO - n

Add zjjfp_require_pushed guard — fundus scenario tabtargets fail fast on unpushed commits before test execution

### 2026-04-05 12:34 - Heat - T

equestrian-verb-reconciliation

### 2026-04-05 12:27 - ₢A4AAO - n

Preflight dispatch smoke check catches missing station regime, ensure_project_root fixes CWD for cargo test, remove plant from concurrent overlap test, preflight_happy returns diagnostic Result

### 2026-04-05 12:19 - ₢A4AAO - n

Fix fundus scenario tests: ensure_project_root for CWD, replace silent-skip gallops lookup with constant firemark, remove find_racing_firemark and GALLOPS_PATH from test code

### 2026-04-05 12:04 - ₢A4AAO - n

BUK delay tabtarget (buw-xd, 20s sleep) and relay_concurrent_overlap fundus scenario test proving temporal overlap with distinct labels and temp dirs

### 2026-04-05 11:54 - ₢A4AAN - W

Two-phase fundus provisioning: P1 (sudo) creates OS accounts, installs keypairs, enables macOS SSH access; P2 (operator) injects curia pubkey via double-hop for remote hosts, clones repos from GitHub (remote) or local (localhost), BUK tracked in repo. JJK workbench converted to zipper dispatch. Tests fail honestly on missing accounts. Verified 11/11 on both localhost and cerebro.

### 2026-04-05 11:53 - ₢A4AAN - n

Add cerebro tabtarget for fundus scenario suite — 11/11 tests pass against remote cerebro

### 2026-04-05 11:47 - ₢A4AAN - n

Phase 2 remote: double-hop SSH injects curia pubkey into test accounts, then direct SSH for GitHub host keys and clone. Localhost unchanged. Distinct machine identities preserved.

### 2026-04-05 11:09 - ₢A4AAN - n

Phase 2 remote-capable: localhost clones from local repo, remote clones from GitHub URL. Eliminated zjjfp_ssh_install_buk (.buk/ is tracked). GitHub host keys added before remote clone. Cerebro tabtarget.

### 2026-04-05 11:04 - ₢A4AAN - n

Add cerebro tabtarget for phase 2 — remote repo provisioning from curia via SSH

### 2026-04-05 11:01 - ₢A4AAN - n

Phase 2 nuclear idempotent: rm -rf project dir before clone, revert origin-remove tolerance, add GitHub host key for jjfu_full, fix safe.directory .git path, macOS home dir mkdir+chown

### 2026-04-05 10:54 - ₢A4AAN - n

macOS sysadminctl may not create home dir: mkdir -p before chown

### 2026-04-05 10:53 - ₢A4AAN - n

Fix macOS home dir ownership: sysadminctl creates home as root, chown to user after account creation

### 2026-04-05 10:52 - ₢A4AAN - n

BCG compliance: eliminate $() on non-capture functions (use temp file resolvers), fix local -r in loop bug, capture stderr instead of 2>/dev/null, replace test $? with direct || buc_die

### 2026-04-05 10:48 - ₢A4AAN - n

Phase 1 enables macOS SSH access (com.apple.access_ssh) for jjfu_full/norepo/nogit, adds StrictHostKeyChecking=accept-new to phase 2 SSH commands

### 2026-04-05 10:44 - ₢A4AAN - n

Phase 1 restores ownership of dispatch temp/output dirs to SUDO_USER before exit, preventing root-owned debris from poisoning operator's next dispatch

### 2026-04-05 10:40 - ₢A4AAN - n

Split provisioning into two phases: P1 (root) creates accounts + installs keypairs, P2 (operator) SSHes to accounts to clone repos + install BUK — validates SSH access as side effect

### 2026-04-05 10:37 - ₢A4AAP - W

Removed jja_arcanum.sh and jjl_ledger.json — all functionality (slash commands, brand/ledger versioning, directory creation, settings permissions) superseded by MCP tool and Rust layer. Updated CLAUDE.md acronym map and README installation/command sections.

### 2026-04-05 10:32 - ₢A4AAN - n

Provision takes private key path, installs full keypair (private for GitHub access, public for authorized_keys) to each test account

### 2026-04-05 10:22 - ₢A4AAL - W

Migrated all RBK fact-file producers from raw echo > BURD_OUTPUT_DIR to buf_write_fact dual-write primitive. 15 write sites in rbfd_FoundryDirectorBuild.sh (conjure, kludge, mirror, graft) and 1 in rbfl_FoundryLedger.sh (tally). Both CLI entry points source buf_fact.sh. Four-mode integration test passes all modes.

### 2026-04-05 10:16 - ₢A4AAN - n

JJSTF preflight contract: tests fail on missing preconditions instead of silently skipping — a passing test means the assertion was actually exercised

### 2026-04-05 10:15 - ₢A4AAN - n

Fundus scenario tests fail honestly when accounts missing: replace silent skip-and-return with assert! panics showing provision instructions

### 2026-04-05 10:07 - ₢A4AAN - n

Provision assumes root: remove all sudo calls, add root gate, use su for target-user ops, accept pubkey as param, delete cerebro tabtarget

### 2026-04-05 10:01 - Heat - n

Expand theurge colophon manifest to include theurge-own colophons alongside RBK zipper colophons

### 2026-04-05 10:01 - ₢A4AAP - n

Remove arcanum and ledger — all functionality superseded by MCP tool. Update CLAUDE.md acronym map and README installation/command sections to reflect MCP-only architecture.

### 2026-04-05 09:53 - ₢A4AAL - n

Migrate all RBK fact-file producers from raw echo > BURD_OUTPUT_DIR writes to buf_write_fact dual-write primitive

### 2026-04-05 09:49 - ₢A4AAN - n

Remove old jjf_ files superseded by jjfp_ rename

### 2026-04-05 09:49 - ₢A4AAN - n

Fix minting violation: rename jjf_ prefix to jjfp_ (jjf is non-terminal due to jjfu_/jjfg_ children)

### 2026-04-05 09:08 - ₢A4AAN - n

JJK zipper registry (jjz_zipper.sh), fundus BCG module (jjf_fundus.sh + jjf_cli.sh) with provision/scenario/single commands, workbench converted to buz_exec_lookup dispatch, provisioning tabtargets for localhost and cerebro

### 2026-04-05 09:07 - Heat - S

evaluate-arcanum-removal

### 2026-04-05 08:45 - ₢A4AAM - W

All 7 docket items verified complete: JJSTF revised with jjfu_* account-shape profiles replacing machine-identity, JJS0 mapping cleaned, remote_dispatch.rs and jjrlg_legatio.rs purged of FUNDUS_DEFAULT constants, old jjw-tR tabtargets replaced by jjw-tfs/jjw-tfS, workbench routing updated, context files refreshed.

### 2026-04-05 08:38 - Heat - S

concurrent-dispatch-scenario

### 2026-04-05 08:35 - Heat - S

fundus-account-provision-and-smoke

### 2026-04-05 08:33 - ₢A4AAK - W

Wrote 6 remote dispatch procedure specs inline in JJS0 (bind, send, relay, check, fetch, plant) with full Arguments/Behavior/Exit Status/Stdout sections and AXLA hierarchy operation markers (axhob, axhopt, axhos, axhoot, axhoc). Also wired orphaned JJSCTL-tally.adoc (via tagged regions for redocket/relabel/drop) and JJSCCH-chalk.adoc into JJS0, added jjdo_redocket and jjdo_chalk mapping entries.

### 2026-04-05 08:33 - ₢A4AAM - n

Remove old machine-identity test file and tabtargets replaced by fundus scenario infrastructure

### 2026-04-05 08:32 - ₢A4AAM - n

Deployment-agnostic fundus scenario profiles: jjfu_* account names replace machine-identity profiles (hairpin/cerebro), host parameterized via JJTEST_HOST, removed FUNDUS_DEFAULT constants, new jjw-tfs/jjw-tfS tabtargets

### 2026-04-05 08:25 - Heat - S

fundus-scenario-revision

### 2026-04-05 08:23 - ₢A4AAK - n

Add AXLA hierarchy operation markers to 6 remote dispatch procedure specs: axhob_operation (6), axhopt_typed_parameter (15), axhos_step (34), axhoot_typed_output (2 — bind/relay only, typed domain outputs), axhoc_completion (6)

### 2026-04-05 08:09 - ₢A4AAK - n

Remote dispatch procedure specs: replace 6 stub entries (bind, send, relay, check, fetch, plant) with full Arguments/Behavior/Exit Status/Stdout specifications inline in JJS0 Remote Dispatch Operations section, faithful to Rust implementation

### 2026-04-05 08:04 - Heat - n

Wire JJSCTL-tally.adoc and JJSCCH-chalk.adoc into JJS0: add jjdo_redocket and jjdo_chalk mapping entries, restructure tally.adoc with tagged regions replacing superseded header, replace inline relabel/drop with tag-based includes from tally, add chalk include in Steeplechase Operations

### 2026-04-04 16:01 - Heat - S

rbk-fact-file-producer-migration

### 2026-04-04 15:59 - ₢A4AAH - W

MCP command name const registry: 32 JJRM_CMD_NAME_* consts in dispatcher with JJRM_ALL_COMMANDS array replacing hand-maintained command list, per-handler CMD_NAME consts with let cn pattern across 24 handler files, zero bare jjx_ string literals in production code (schemars attributes excepted — proc-macro limitation)

### 2026-04-04 15:59 - ₢A4AAG - W

Implemented 17 Rust integration tests across 5 fundus profiles (hairpin, cerebro, nokey, norepo, nogit) with RAII test officia, preflight guards, shared happy-path implementations covering bind/send/plant/fetch/relay/check/parallel dispatch and BURX_LABEL correlation, plus jjw-tR workbench routing with per-profile preflight probes and 5 imprint-channel tabtargets

### 2026-04-04 15:56 - ₢A4AAH - n

MCP command name const registry: 32 JJRM_CMD_NAME_* consts in dispatcher with JJRM_ALL_COMMANDS array, per-handler CMD_NAME consts with let cn pattern across 24 handler files, zero bare jjx_ string literals in production code

### 2026-04-04 14:57 - Heat - r

moved A4AAH before A4AAF

### 2026-04-04 14:46 - ₢A4AAE - W

JJSTF-test-fundus.adoc: JJS0 subdocument specifying remote dispatch integration test infrastructure. 5 fundus profiles (hairpin, cerebro, nokey, norepo, nogit) with setup procedures, preflight contracts (skip-not-fail), jjw-tR tabtarget surface with imprint channel, per-profile test assertions covering all 6 remote dispatch commands. New jjdxf_ quoin category (10 quoins) in JJS0 mapping section, JJSTF acronym in CLAUDE.md.

### 2026-04-04 14:46 - ₢A4AAD - W

Implemented jjx_relay (async nohup dispatch with pensum minting, BURX label-matching capture, timeout watchdog) and jjx_check (probe/poll with running/orphaned/stopped/lost reports, file listing on terminal status), pensum state persistence in officium, MCP wiring with firemark param addition, plus jjri_io migration fix for next_pensum_seed round-trip check

### 2026-04-04 14:43 - ₢A4AAD - n

jjx_relay and jjx_check async dispatch commands: nohup wrapper with timeout watchdog, BURX capture with label-matching poll, pensum state persistence, probe/poll with running/orphaned/stopped/lost reports, MCP wiring, plus jjri_io migration fix for next_pensum_seed round-trip check (latent defect from pensum identity type pace)

### 2026-04-04 14:34 - ₢A4AAE - n

JJSTF test fundus spec: 5 fundus profiles (hairpin, cerebro, nokey, norepo, nogit) with preflight contracts, jjw-tR tabtarget surface, per-profile test assertions, jjdxf_ quoin category in JJS0 mapping section

### 2026-04-04 14:18 - Heat - T

remote-dispatch-integration-tests

### 2026-04-04 14:18 - Heat - T

remote-dispatch-test-fundus-spec

### 2026-04-04 14:16 - ₢A4AAJ - W

Pensum identity type (₱) with jjrf_Pensum struct, encode/decode/parse/display, JJRF_PENSUM_SENTINEL for coronet disambiguation, next_pensum_seed on jjrg_Heat with serde backwards compat, jjrg_mint_pensum using direct string concatenation, validation, 20 tests. RCG review yielded JJRF_RADIX const retiring pre-existing magic-number debt across all identity types.

### 2026-04-04 14:13 - ₢A4AAJ - n

RCG compliance fixes: JJRF_RADIX const derived from JJRF_CHARSET.len() replacing all magic 64/4096 literals across Firemark/Coronet/Pensum encode/decode, MAX consts derived from RADIX, module doc updated for Pensum, jjrg_mint_pensum simplified from decode-encode roundtrip to direct string concatenation matching jjrg_slate pattern

### 2026-04-04 14:05 - ₢A4AAJ - n

Pensum identity type (₱): jjrf_Pensum struct with encode/decode/parse/display, JJRF_PENSUM_SENTINEL for coronet disambiguation, next_pensum_seed on jjrg_Heat with serde default for backwards compat, jjrg_mint_pensum seed lifecycle, validation in jjrv, 20 tests, all Heat construction sites updated

### 2026-04-04 13:17 - ₢A4AAB - W

JJS0 remote dispatch vocabulary and type system: insignia abstract type (axt_insignia + axd_algebraic/temporal in AXLA), curia/fundus roles, legatio structural type, pensum identity type with % sentinel and heat-embedded firemark, RELDIR safety premise, 6 operation stubs for remote dispatch, pensum report states (running/orphaned/stopped/lost), retrofit firemark/coronet to voice axt_insignia, all charset references use jjdz_charset quoin. Slated ₢A4AAI (verb reconciliation) and ₢A4AAK (per-procedure specs). jjk-claude-context.md update deferred to alignment audit.

### 2026-04-04 13:17 - ₢A4AAC - W

SSH legatio module (jjrlg_legatio.rs) with 4 sync commands: bind (Layer 1+3 RELDIR validation, BURC_OUTPUT_ROOT_DIR caching), send (bash -c via legatio), plant (git reset to commit), fetch (single file read). RCG String Boundary Discipline applied: JJRLG_CMD_NAME_* and JJRM_CMD_NAME_* consts replace all magic strings, Identity Rule added to RCG Constant Discipline. Legatio confirmed session-local (L0/L1 tokens in officium dir). Pensum identity design resolved in conversation: globally-allocated ₱ type, 5 chars with % sentinel, seed on heat record in gallops. Slated ₢A4AAH (const registry cleanup) and ₢A4AAJ (pensum identity type).

### 2026-04-04 13:16 - ₢A4AAB - n

Replace all bare 'base64url' with {jjdz_charset} linked term — 9 occurrences across identity encoding, insignia definition, firemark, coronet, pensum, and seed member descriptions

### 2026-04-04 13:13 - ₢A4AAB - n

Fix 4 remaining base62→base64url occurrences in Identity Encoding overview and pensum type definition

### 2026-04-04 13:09 - ₢A4AAB - n

Fix base62→base64url throughout JJS0 and AXLA, correct capacity numbers (3,844→4,096 heats/pensa, ~238K→~262K coronets), remove concrete alphabet example from AXLA axd_algebraic definition

### 2026-04-04 13:04 - ₢A4AAB - n

JJS0 remote dispatch vocabulary: insignia type system (axt_insignia + axd_algebraic/temporal in AXLA), curia/fundus roles, legatio structural type, pensum identity type with % sentinel, RELDIR safety premise, 6 operation stubs, pensum report states, retrofit firemark/coronet to voice axt_insignia

### 2026-04-04 12:55 - Heat - S

remote-dispatch-procedure-specs

### 2026-04-04 12:51 - Heat - S

jjk-pensum-identity-type

### 2026-04-04 12:05 - Heat - S

equestrian-verb-reconciliation

### 2026-04-04 10:35 - ₢A4AAC - n

RCG String Boundary Discipline compliance: command name consts (JJRLG_CMD_NAME_BIND etc.) replace all magic strings in legatio handlers, JJRM_CMD_NAME_* rename in dispatcher. Added Identity Rule to RCG Constant Discipline with checklist item — const names must identify the specific value, not just the category.

### 2026-04-04 10:15 - ₢A4AAC - n

Implement SSH legatio module with jjx_bind, jjx_send, jjx_plant, jjx_fetch commands. RCG-compliant const registry for new command names with JJRM_LEGATIO_COMMANDS array. Layer 1 RELDIR validation, platform ssh transport, legatio state persistence in officium directory. 8 unit tests (RELDIR, git ref, shell quoting, token minting).

### 2026-04-04 10:09 - Heat - S

jjk-mcp-command-name-const-registry

### 2026-04-04 09:49 - ₢A4AAA - W

Implemented BURX exchange regime and dual-write fact-file primitive across BUS0 spec and BUK source. New buf_fact.sh module with buf_write_fact (preexistence guard, BUF_burx_env tinder constant). BUD dispatch writes burx.env at start (7 fields) and appends completion (EXIT_STATUS, ENDED_AT) with nanosecond timestamps via temp-file discipline. BURE_LABEL added to ambient regime kindle (xname, max 120 chars). BUS0 quoins for all 13 new terms with AXLA voicings. BUK self-test expanded from 9 to 15 cases: burx-exchange fixture (dual-write, field completeness, preexistence rejection, timestamp format) and BURE_LABEL validation (valid, too-long). All BCG-compliant.

### 2026-04-04 09:49 - ₢A4AAA - n

Implement BURX regime and dual-write fact-file primitive: buf_fact.sh module with preexistence guard, BURE_LABEL kindle, BUD dispatch writes burx.env at start/completion, BUS0 quoins for all new terms, 15-case self-test covering dual-write, field completeness, preexistence rejection, timestamp format, and BURE_LABEL validation

### 2026-04-04 09:18 - Heat - S

advanced-test-case-planning

### 2026-04-04 08:57 - Heat - S

spec-source-alignment-audit

### 2026-04-04 08:56 - Heat - r

moved A4AAB after A4AAE

### 2026-04-04 08:55 - Heat - S

jjk-remote-dispatch-integration-tests

### 2026-04-04 08:54 - Heat - S

jjk-async-dispatch-commands

### 2026-04-04 08:54 - Heat - S

jjk-ssh-foundation-sync-commands

### 2026-04-04 08:54 - Heat - S

jjs0-remote-dispatch-spec

### 2026-04-04 08:53 - Heat - S

buk-burx-regime-and-dual-write

### 2026-04-04 08:34 - Heat - d

paddock curried: BURX field renames (EXIT_STATUS, LABEL), fetch→single-file, check returns file list on terminal

### 2026-04-04 08:19 - Heat - d

paddock curried: merge jjx_await into jjx_check with required timeout param (0=instant, >0=poll)

### 2026-04-04 08:13 - Heat - d

paddock curried: add jjx_plant design decision (git reset+clean, fail-fast, fundus precondition)

### 2026-04-03 18:55 - Heat - d

paddock curried: restored bash code examples (gazette fence fix live), updated ₣A2 supersession note

### 2026-04-03 18:43 - Heat - d

paddock curried: fundus terminology, reldir safety, hairpin testing, all bind params required

### 2026-04-03 18:08 - Heat - d

paddock curried: added fact-file infrastructure survey from codebase

### 2026-04-03 17:59 - Heat - f

racing

### 2026-04-03 17:59 - Heat - d

paddock curried: design decisions from officium conversation

### 2026-04-03 17:54 - Heat - N

jjk-v3-7-remote-dispatch

