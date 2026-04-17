## Ann's PHI Clipbuddy Kit (APCK)

APCK is a Tauri desktop app that intercepts clipboard content from Epic EHR, detects PHI via a three-tier engine (regex, label-anchored, dictionary), presents a red/yellow/grey triage view, and writes anonymized plain text to clipboard for pasting into Open Evidence.

### File Acronym Mappings

| Acronym | File | Description |
|---------|------|-------------|
| **APCS** | `apck/APCS-Specification.md` | Product spec — UX, workflow, deployment |
| **APCPS** | `apck/APCPS-PrototypeSpecification.md` | Prototype spec — detection pipeline, tech stack |
| **APCW** | `apck/apcw_workbench.sh` | Workbench |
| **APCZ** | `apck/apcz_zipper.sh` | Zipper enrollment |
| **APCC** | `apck/apcc_cli.sh` | CLI command implementations |
| **APCAP** | `apck/apcd/src/apcap_main.rs` | Tauri app entry point |
| **APCAL** | `apck/apcd/src/apcal_main.rs` | Fixture loader (clipboard writer) |
| **APCAD** | `apck/apcd/src/apcad_main.rs` | Dictionary refresh (downloads public sources, regenerates dictionaries) |
| **APCAB** | `apck/apcd/src/apcab_main.rs` | Batch assay (run detection pipeline on HTML directory, write assay output) |

**Source modules:**

| Source | Prefix | Test | Prefix | Purpose |
|--------|--------|------|--------|---------|
| `apcrl_log.rs` | `apcrl` | — | — | Logging macros (`apcrl_info!`, `apcrl_error!`, `apcrl_fatal!`) |
| `apcre_engine.rs` | `apcre` | `apcte_engine.rs` | `apcte` | PHI detection orchestrator |
| `apcrp_parse.rs` | `apcrp` | `apctp_parse.rs` | `apctp` | HTML clipboard parsing |
| `apcrm_match.rs` | `apcrm` | `apctm_match.rs` | `apctm` | Dictionary/regex matching |
| `apcrd_dictionaries.rs` | `apcrd` | `apctd_dictionaries.rs` | `apctd` | Dictionary loading |
| `apcru_update.rs` | `apcru` | — | — | Self-update watcher (no unit tests — I/O + process) |

**Other key paths:**
- `Tools/apck/apcd/ui/` — Frontend (HTML/CSS only — no JavaScript)
- `Tools/apck/apcd/dictionaries/` — Blacklist/whitelist data files
- `Tools/apck/test_fixtures/` — Synthetic Epic clipboard data

**No JavaScript rule:** All rendering logic lives in Rust. The Tauri webview is a passive display surface — Rust pushes complete HTML on every state change via `eval()`. Inline `onclick` attributes use the Tauri bridge primitive (`window.__TAURI__.core.invoke`) for command dispatch. No `.js` files, no JS application logic. This eliminates the Rust-JS language boundary as a bug class.

**Rust conventions:** Follow RCG (`Tools/vok/vov_veiled/RCG-RustCodingGuide.md`). Source prefix: `apcr{classifier}_{name}.rs`. Test prefix: `apct{classifier}_{name}.rs`. Classifier matches between source and test.

### Tabtargets

| Tabtarget | Colophon | Purpose |
|-----------|----------|---------|
| `tt/apcw-b.Build.sh` | `apcw-b` | `cargo tauri build` (release) |
| `tt/apcw-r.Run.sh` | `apcw-r` | `cargo run --bin apcap` (local development) |
| `tt/apcw-d.Deploy.sh` | `apcw-d` | Build + scp to `anns-macbook-air:/Users/Shared/apcua/` |
| `tt/apcw-fl.FixtureLoad.sh` | `apcw-fl` | Run `apcal` to load fixture HTML onto clipboard |
| `tt/apcw-t.Test.sh` | `apcw-t` | `cargo test` in `apcd/` |
| `tt/apcw-dr.DictionaryRefresh.sh` | `apcw-dr` | `cargo run --bin apcad` (refresh dictionaries from public sources) |
| `tt/apcw-ba.BatchAssay.sh` | `apcw-ba` | `cargo run --bin apcab` (batch assay on HTML directory) |

### Prefix Tree

```
apc  (non-terminal)
├── apca   (non-terminal)
│   ├── apcab  — App Batch binary (assay — detection pipeline on HTML files)
│   ├── apcad  — App Dictionary binary (refresh from public sources)
│   ├── apcal  — App Loader binary (fixture clipboard tool)
│   └── apcap  — App Prototype binary (Tauri main)
├── apcc   — CLI command implementations
├── apcd   — Rust/Tauri source directory
│   └── apcrl  — Logging macros (info, error, fatal with file/line)
├── apck   — kit directory
├── apcps  — prototype specification document
├── apcs   — product specification document
├── apcu   (non-terminal)
│   └── apcua — update staging directory (/Users/Shared/apcua/)
├── apcw   — workbench
└── apcz   — zipper
```

### Detection Architecture

Three tiers, merged by highest severity:
- **Tier 1 — Regex (RED):** SSN, phone, email, dates, addresses, zip, labeled identifiers
- **Tier 2 — Label-anchored (RED):** Words following Epic labels (Patient:, Attending:, Facility:, etc.)
- **Tier 3 — Dictionary (YELLOW/PASS):** aho-corasick blacklist scan, whitelist filter, collision → YELLOW

### Deploy Workflow

1. `tt/apcw-b.Build.sh` — produces `.app` bundle
2. `tt/apcw-d.Deploy.sh` — scp to Ann's machine at `/Users/Shared/apcua/`
3. App self-updates: watches staging directory, copies new bundle over self, relaunches
