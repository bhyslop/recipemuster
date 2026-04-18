# APCNS0 — Neural Stanford Spike

A parallel investigation to compare the [StanfordAIMI/stanford-deidentifier-base](https://huggingface.co/StanfordAIMI/stanford-deidentifier-base) token-classification model against the existing 3-tier APCK detection engine on the HTML fixtures. Emits `{stem}.stanford.assay.txt` alongside `apcab`'s `{stem}.assay.txt` so the two can be diffed side-by-side.

**Status**: spike. No integration into `apcre_engine.rs`. Not a production code path.

## Model Card (summary)

- Identifier: `StanfordAIMI/stanford-deidentifier-base`
- Architecture: BERT-base, token classification head, IOB2/BIO tagging
- License: check the HuggingFace model card before any redistribution
- Training data: Stanford de-identification corpus (radiology-adjacent clinical text)
- Purpose: clinical-text PHI de-identification
- Label set: read from `config.json:id2label` at runtime — no hard-coded mapping

## Artifact Layout

Hardcoded model directory: `/Users/bhyslop/models/stanford-deidentifier/`

Contents after export:

```
/Users/bhyslop/models/stanford-deidentifier/
├── .venv/                 # Python venv holding optimum-cli + transformers
│   └── bin/optimum-cli
├── model.onnx             # ONNX graph
├── tokenizer.json         # HuggingFace tokenizer, fast variant
├── config.json            # Contains id2label, num_labels, etc.
├── special_tokens_map.json
├── tokenizer_config.json
└── vocab.txt
```

Rationale for colocating the venv with the model: all of the spike's external artifacts live under one path, and the install command can heal the model dir without touching the project tree.

## Setup on a Fresh Machine

1. Ensure `python3` is on PATH (any 3.10+).
2. Run `tt/apcw-nsi.NeuralStanfordInstall.sh`. This is the **single convergent install command** — it always reaches a working state:
   - Creates `/Users/bhyslop/models/stanford-deidentifier/` if absent.
   - Creates a venv at `{model_dir}/.venv/` if absent.
   - Installs `optimum + optimum-onnx + onnxruntime` into the venv if the `onnx` subcommand isn't registered.
   - Clears any prior model artifacts (preserving the venv) and re-exports from scratch.
3. Run `tt/apcw-nsa.NeuralStanfordAssay.sh Tools/apck/test_fixtures` to produce `{stem}.stanford.assay.txt` files in `BURD_OUTPUT_DIR`.

Re-running `apcw-nsi` heals any partial or corrupt state. There is no separate "zap" command — the install command itself is the reset.

## Install Procedure (what `apcw-nsi` runs)

```
python3 -m venv /Users/bhyslop/models/stanford-deidentifier/.venv
/Users/bhyslop/models/stanford-deidentifier/.venv/bin/python -m pip install --upgrade pip
/Users/bhyslop/models/stanford-deidentifier/.venv/bin/python -m pip install optimum optimum-onnx onnxruntime
find /Users/bhyslop/models/stanford-deidentifier -mindepth 1 -maxdepth 1 ! -name '.venv' -exec rm -rf {} +
/Users/bhyslop/models/stanford-deidentifier/.venv/bin/optimum-cli export onnx \
    --model StanfordAIMI/stanford-deidentifier-base \
    --task token-classification \
    /Users/bhyslop/models/stanford-deidentifier/
```

First run: `pip install` pulls ~2 GB of dependencies (PyTorch + transformers + optimum + onnxruntime); the export downloads the ~440 MB base model from HuggingFace. Subsequent runs skip the pip install, and the export step re-uses the HuggingFace cache at `~/.cache/huggingface/` so only the ONNX trace is re-computed (seconds).

## Runtime Architecture

### Binary

`Tools/apck/apcd/src/apcnsa_main.rs` — `cargo run --bin apcnsa <input-dir>`.

Reads `APCNSA_MODEL_DIR` and `BURD_OUTPUT_DIR` from the environment. The workbench sets both.

### Pipeline

1. Glob `*.html` under the input directory.
2. For each file:
   1. Parse with `apcrp_parse` → `plain_text`.
   2. Walk lines; tokenize each non-empty line; run ONNX inference; argmax per token; BIO-decode into entity spans.
   3. Add the line's byte offset in `plain_text` to each span's local offsets — offsets are byte-authoritative in `plain_text`.
   4. Render with guillemet markers and write `{stem}.stanford.assay.txt`.

### Byte-Offset Discipline

The anticipated hazard for this pace was token-to-byte offset alignment — the same bug class that drove `₢A9AAP`.

- The tokenizer is fed UTF-8 bytes from `&plain_text[line_start..line_end]` (`&str`).
- HuggingFace `tokenizers` returns `(start, end)` byte offsets into the string that was tokenized — for this pipeline, that string is the line, so offsets are line-local bytes.
- We add `line_start` (byte offset of the line in `plain_text`) to produce document-wide byte offsets.
- The final slice `&plain_text[offset..offset+length]` is a Rust `str` slice (byte indices that must land on UTF-8 char boundaries; WordPiece does not split mid-char for well-formed input).

If the output renders corrupted multibyte characters inside guillemets, check for a char-boundary violation — that would indicate the tokenizer's offsets are not aligned to UTF-8 boundaries as assumed.

### Long-Sequence Handling

Lines longer than the model's context window (typically 512 tokens) are not currently split. For clinical fixtures where lines correspond to labeled fields or short narrative sentences, this is expected to be safe. If a fixture produces an inference error, the remedy is to chunk within-line.

### Dynamic Input Set

The session is queried for its declared inputs. `token_type_ids` is supplied (zero-filled) if and only if the model declares it — so the same binary works whether the exported model is BERT-family (uses them) or DistilBERT (does not).

## Output Format

```
Patient: ‹NAME Margaret Thornton›
DOB: ‹DATE 03/15/1952›
Attending: ‹DOCTOR J. Feng, MD›
```

Labels are the raw BIO kind from `config.json:id2label` (e.g., `NAME`, `DATE`, `DOCTOR`, `HOSPITAL`, `ID`, `LOCATION`, `PHONE`, `PATIENT`). No mapping to APCK's `apcrm_PhiCategory` — that's a followup if the spike earns integration.

## Observations

### Label Set (discovered at runtime from `config.json`)

Only 8 labels, flat (not BIO-encoded):

```
O, VENDOR, DATE, HCW, HOSPITAL, ID, PATIENT, PHONE
```

This is much coarser than APCK's ~18 HIPAA categories. HCW = healthcare worker; ID conflates MRN, SSN, NPI, DEA, Account, Encounter. No dedicated category for Address, Email, URL, Age, SSN, or Device ID — these either collapse into adjacent labels or are missed entirely.

The tokenizer returns flat labels (no `B-`/`I-` prefix), so runs of identical labels form a single entity. The decoder in `apcnsa_main.rs` handles both BIO and flat forms.

### Accuracy on fixtures

**Stanford wins on multi-token named entities:**
- Patient names correctly merged as one span: `Margaret J. Thornton` → one `HCW` span. apcab splits into three separate `NAME` tokens.
- Facility names merged: `Maine Medical Center` → one `HOSPITAL` span. apcab produces three `FACILITY` tokens.
- Catches narrative mentions apcab only YELLOW-flagged: `Ms. Thornton` as `PATIENT`, family members `Dorothy Kowalski`, `Edward Kowalski`, `Linda Washington` as `PATIENT`.
- Provider roles correctly flagged: `Dr. Susan Chen`, `James R. Whitfield, MD`, `Priya Anand, MD`, `Elena Vasquez, PA-C` as `HCW`.

**Stanford fails badly on structured identifiers:**
- SSN `471-83-2956` fragmented and mislabeled: `SS`→HOSPITAL, `N: 471-83-`→ID, `2956`→PHONE. Completely missed as SSN.
- Email `m.thornton47@gmail.com` split: `m.thornton47`→HCW, `@gmail`→VENDOR, `.com`→untagged.
- URL `https://mychart.mainehealth.org/patient/thornton-m` almost entirely missed (only `mainehealth`→VENDOR).
- Device serial `STR-KN-2023-44819` fragmented into four chunks across ID/PHONE labels.
- Address `1847 Cranberry Lane, Westbrook, ME 04092` fragmented: `1847`→DATE, `Cranberry`→HCW, `Lane, Westbrook, ME`→HOSPITAL, `04092`→PHONE.
- Phone fax number correctly caught; some phones mis-split when adjacent to other numeric tokens.

**False positives (unique to stanford):**
- `Vitals` (section header) tagged HCW.
- `Cranberry Lane` in narrative tagged VENDOR.
- Diagnosis years (2008, 2012) tagged DATE — debatable whether PHI.

**apcab wins on:**
- SSN, Email, URL, Device ID — all caught cleanly via regex.
- Dedicated category taxonomy (NPI vs DEA vs MRN vs Account distinguishable downstream).

### Performance

- Export time (fresh, cold model download + export + verification): ~90s (network-bound on ~440 MB download).
- Export time (idempotent skip): <1s.
- Model load + ORT session init: ~1–2s (one-time per binary run).
- Per-fixture inference latency (CPU EP, dev build): 575ms for `epic_geriatric_consult.html`, 709ms for `epic_progress_note.html` on M-series Mac. Release build is expected to be modestly faster (not measured — spike is dev-only).
- Finding count: 64 (geriatric), 73 (progress note) — entity-level, sensible.

### Footprint

- Model artifacts: 436 MB (`model.onnx` 435 MB + tokenizer 706 KB + config/vocab ~230 KB).
- venv with PyTorch + transformers + optimum + onnxruntime: ~2 GB at `/Users/bhyslop/models/stanford-deidentifier/.venv/`.
- Rust binary dependency delta: `ort` + `tokenizers` + `serde_json` + ONNX Runtime dylib (~30 MB from `download-binaries` feature).

### Byte-Offset Discipline (validated)

No corruption observed inside guillemet markers. Line-by-line tokenization with `line_start` offset addition produces document-wide byte offsets that land on UTF-8 boundaries. The anticipated hazard (token-to-byte alignment, same bug class as `₢A9AAP`) did not materialize.

## Productization Recommendation

**Augment, don't replace.** The Stanford model's gift is multi-token named-entity merging; its weakness is structured identifiers. apcab's strength is the inverse. They are complementary, not competing.

If pursued as a Tier 4:

- **Add** the neural model for person names (PATIENT/HCW) and facility names (HOSPITAL, VENDOR). Use it to elevate YELLOW dictionary hits to RED when a recognized span overlaps.
- **Do not** use it for SSN, phone, email, URL, address, MRN, NPI, DEA, device ID — regex (Tier 1) is more reliable and produces dedicated category tags.
- The coarse 8-label set means any integration requires a lossy mapping (stanford `ID` → which APCK category? depends on context). Either resolve ambiguity via co-located regex, or accept ID as a catch-all bucket with reduced specificity.

**Red flags against integration:**

- **Footprint**: +440 MB in the distributed app bundle. APCK currently targets a lightweight macOS Tauri build; shipping a half-gig neural model is a policy decision, not a technical one.
- **Latency**: ~600ms per clipboard paste on CPU. Acceptable for batch assay, borderline for interactive clipboard triage. CoreML EP may bring this down but adds complexity.
- **License / provenance**: need to confirm redistribution terms for StanfordAIMI/stanford-deidentifier-base before shipping. Model card check pending.
- **Label taxonomy drift**: the model's categories are not APCK's. Any mapping layer will carry edge cases.

**If the recommendation is to proceed**, the next pace would be a Tier 4 integration spike that measures: (1) RED precision/recall lift from neural-assisted disambiguation; (2) end-to-end p99 latency on the interactive path; (3) bundle size impact after quantization to INT8. Until those three numbers are known, the current 3-tier engine remains the right production path.

**If the recommendation is to abandon**, the artifacts stage at a hardcoded path outside the repo — `rm -rf /Users/bhyslop/models/stanford-deidentifier/` clears them, then delete this binary, the two tabtargets, the Cargo deps, and this document.

## Out of Scope

- Label-set mapping (stanford → APCK categories).
- Engine integration.
- CoreML execution provider (future work if CPU latency is unacceptable).
- Model artifact distribution policy (the production app currently cannot ship a 440MB model without an explicit plan).
- Windows support.
- Unit tests for `apcnsa_main.rs` — this is spike code.
