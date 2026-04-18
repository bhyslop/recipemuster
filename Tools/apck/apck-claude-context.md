## Ann's PHI Clipbuddy Kit (APCK)

APCK is a Tauri desktop app that intercepts clipboard content from Epic EHR, detects PHI via a three-tier engine (regex, label-anchored, dictionary), presents a red/yellow/grey triage view, and writes anonymized plain text to clipboard for pasting into Open Evidence.

### File Acronym Mappings

| Acronym | File | Description |
|---------|------|-------------|
| **APCAS** | `apck/APCAS-Specification.md` | Application spec — UX, workflow, deployment |
| **APCS0** | `apck/APCS0-SpecTop.adoc` | Detection pipeline spec — formal vocabulary (MCM concept model) |
| **APCPS** | `apck/APCPS-PrototypeSpecification.md` | Prototype spec — tech stack, data sources, project structure |
| **APCNS0** | `apck/APCNS0-NeuralStanford.md` | Neural Stanford spike doc — model card, export procedure, observations |
| **APCW** | `apck/apcw_workbench.sh` | Workbench |
| **APCZ** | `apck/apcz_zipper.sh` | Zipper enrollment |
| **APCC** | `apck/apcc_cli.sh` | CLI command implementations |
| **APCAP** | `apck/apcd/src/apcap_main.rs` | Tauri app entry point |
| **APCAL** | `apck/apcd/src/apcal_main.rs` | Fixture loader (clipboard writer) |
| **APCAD** | `apck/apcd/src/apcad_main.rs` | Dictionary refresh (downloads public sources, regenerates dictionaries) |
| **APCAB** | `apck/apcd/src/apcab_main.rs` | Batch assay (run detection pipeline on HTML directory, write assay output) |
| **APCNSA** | `apck/apcd/src/apcnsa_main.rs` | Neural Stanford assay — spike binary running StanfordAIMI/stanford-deidentifier-base via ONNX Runtime |

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
| `tt/apcw-nsi.NeuralStanfordInstall.sh` | `apcw-nsi` | Neural Stanford spike — convergent install: create venv, install optimum + optimum-onnx + onnxruntime if needed, clear any prior artifacts, re-export ONNX. Always reaches a working state. |
| `tt/apcw-nsa.NeuralStanfordAssay.sh` | `apcw-nsa` | Neural Stanford spike — `cargo run --bin apcnsa` on HTML directory |

### Prefix Tree

```
apc  (non-terminal)
├── apca   (non-terminal)
│   ├── apcab  — App Batch binary (assay — detection pipeline on HTML files)
│   ├── apcad  — App Dictionary binary (refresh from public sources)
│   ├── apcal  — App Loader binary (fixture clipboard tool)
│   ├── apcap  — App Prototype binary (Tauri main)
│   └── apcas  — application specification document (UX, workflow)
├── apcc   — CLI command implementations
├── apcd   — Rust/Tauri source directory
│   └── apcrl  — Logging macros (info, error, fatal with file/line)
├── apck   — kit directory
├── apcn   (non-terminal — neural)
│   └── apcns  (non-terminal — neural stanford)
│       ├── apcns0  — Neural Stanford spike documentation (APCNS0-NeuralStanford.md)
│       └── apcnsa  — App Neural Stanford Assay binary (ONNX Runtime + stanford-deidentifier-base)
├── apcps  — prototype specification document
├── apcs   (non-terminal)
│   └── apcs0  — detection pipeline specification (MCM concept model)
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
