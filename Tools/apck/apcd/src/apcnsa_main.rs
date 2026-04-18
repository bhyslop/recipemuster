// Copyright 2026 Scale Invariant, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![deny(warnings)]
#![allow(non_camel_case_types)]

//! Neural Stanford Assay — parallel spike to apcab. Runs the
//! StanfordAIMI/stanford-deidentifier-base model via ONNX Runtime on
//! HTML fixtures and emits guillemet-delimited findings. No engine
//! integration; comparative evidence only.
//!
//! Byte-offset discipline: tokenizer is fed UTF-8 bytes of each line,
//! and returned offsets are byte offsets in that line's string. The
//! authoritative unit everywhere is UTF-8 bytes — matching apcab.
//! Tokenizer offsets are added to each line's byte position in the
//! full plain_text to produce document-wide byte offsets.
//!
//! Usage: apcnsa <input-directory>
//! Env:
//!   APCNSA_MODEL_DIR   — directory holding model.onnx, tokenizer.json, config.json
//!   BURD_OUTPUT_DIR    — where to write {stem}.stanford.assay.txt

use apcd::apcrp_parse::apcrp_parse;

use ort::session::{builder::GraphOptimizationLevel, Session};
use ort::value::Value;
use serde_json::Value as JsonValue;
use tokenizers::tokenizer::Tokenizer;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Finding type — document-wide byte offsets into plain_text.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct apcnsa_Finding {
    offset: usize, // byte offset in plain_text
    length: usize, // byte length in plain_text
    label:  String, // stanford raw label (e.g., "NAME", "DATE"), "-" prefix stripped
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        apcd::apcrl_fatal_now!("usage: apcnsa <input-directory>");
    }
    let input_dir = &args[1];

    let model_dir = std::env::var("APCNSA_MODEL_DIR")
        .unwrap_or_else(|_| apcd::apcrl_fatal_now!("APCNSA_MODEL_DIR not set"));
    let output_dir = std::env::var("BURD_OUTPUT_DIR")
        .unwrap_or_else(|_| apcd::apcrl_fatal_now!("BURD_OUTPUT_DIR not set"));

    let model_dir = PathBuf::from(&model_dir);
    let model_path     = model_dir.join("model.onnx");
    let tokenizer_path = model_dir.join("tokenizer.json");
    let config_path    = model_dir.join("config.json");

    for p in [&model_path, &tokenizer_path, &config_path] {
        if !p.exists() {
            apcd::apcrl_fatal_now!(
                "missing artifact: {} (run apcw-nsx to populate {})",
                p.display(), model_dir.display()
            );
        }
    }

    let entries: Vec<_> = std::fs::read_dir(input_dir)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!(
            "failed to read directory {}: {}", input_dir, e
        ))
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("html") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    if entries.is_empty() {
        apcd::apcrl_fatal_now!("no .html files found in {}", input_dir);
    }

    apcd::apcrl_info_now!("loading tokenizer: {}", tokenizer_path.display());
    let tokenizer = Tokenizer::from_file(&tokenizer_path)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("tokenizer load: {}", e));

    apcd::apcrl_info_now!("loading id2label: {}", config_path.display());
    let id2label = zapcnsa_load_id2label(&config_path);

    apcd::apcrl_info_now!("loading ONNX model: {}", model_path.display());
    let mut session = Session::builder()
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("session builder: {}", e))
        .with_optimization_level(GraphOptimizationLevel::Level3)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("optimization level: {}", e))
        .commit_from_file(&model_path)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("model load: {}", e));

    let input_names: Vec<String> = session.inputs().iter().map(|i| i.name().to_string()).collect();
    apcd::apcrl_info_now!("model inputs: {:?}", input_names);
    let needs_token_type_ids = input_names.iter().any(|n| n == "token_type_ids");

    let mut processed = 0u32;
    for path in &entries {
        let stem = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        apcd::apcrl_info_now!("processing: {}", path.display());

        let html = std::fs::read_to_string(path)
            .unwrap_or_else(|e| apcd::apcrl_fatal_now!(
                "failed to read {}: {}", path.display(), e
            ));

        let doc = apcrp_parse(&html);
        let t_start = std::time::Instant::now();
        let findings = zapcnsa_analyze(
            &doc.plain_text, &tokenizer, &mut session, &id2label, needs_token_type_ids,
        );
        let elapsed_ms = t_start.elapsed().as_millis();

        let assay_text = zapcnsa_render_assay(&doc.plain_text, &findings);

        let output_path = Path::new(&output_dir)
            .join(format!("{}.stanford.assay.txt", stem));
        std::fs::write(&output_path, &assay_text)
            .unwrap_or_else(|e| apcd::apcrl_fatal_now!(
                "failed to write {}: {}", output_path.display(), e
            ));

        apcd::apcrl_info_now!(
            "wrote {} ({} bytes, {} findings, {}ms)",
            output_path.display(), assay_text.len(), findings.len(), elapsed_ms
        );
        processed += 1;
    }

    apcd::apcrl_info_now!(
        "neural stanford assay complete — {} files processed", processed
    );
}

// ---------------------------------------------------------------------------
// Label map
// ---------------------------------------------------------------------------

fn zapcnsa_load_id2label(config_path: &Path) -> Vec<String> {
    let text = std::fs::read_to_string(config_path)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("config read: {}", e));
    let json: JsonValue = serde_json::from_str(&text)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("config parse: {}", e));
    let map = json.get("id2label")
        .unwrap_or_else(|| apcd::apcrl_fatal_now!("config.json missing id2label"))
        .as_object()
        .unwrap_or_else(|| apcd::apcrl_fatal_now!("id2label not an object"));

    let mut pairs: Vec<(usize, String)> = Vec::with_capacity(map.len());
    for (k, v) in map {
        let id: usize = k.parse()
            .unwrap_or_else(|e| apcd::apcrl_fatal_now!("id2label key '{}': {}", k, e));
        let label = v.as_str()
            .unwrap_or_else(|| apcd::apcrl_fatal_now!("id2label[{}] not a string", k))
            .to_string();
        pairs.push((id, label));
    }
    pairs.sort_by_key(|(id, _)| *id);
    let labels: Vec<String> = pairs.into_iter().map(|(_, l)| l).collect();
    apcd::apcrl_info_now!("id2label: {} labels", labels.len());
    labels
}

// ---------------------------------------------------------------------------
// Analysis — line-by-line inference, byte offsets in plain_text.
// ---------------------------------------------------------------------------

fn zapcnsa_analyze(
    plain_text: &str,
    tokenizer: &Tokenizer,
    session: &mut Session,
    id2label: &[String],
    needs_token_type_ids: bool,
) -> Vec<apcnsa_Finding> {
    let mut findings = Vec::new();

    // Walk lines, tracking byte offset of each line's start in plain_text.
    let bytes = plain_text.as_bytes();
    let mut line_start: usize = 0;
    while line_start < bytes.len() {
        let rel_nl = bytes[line_start..].iter().position(|&b| b == b'\n');
        let line_end = match rel_nl {
            Some(n) => line_start + n,
            None    => bytes.len(),
        };
        let line = &plain_text[line_start..line_end];

        if !line.trim().is_empty() {
            let line_findings = zapcnsa_process_line(
                line, tokenizer, session, id2label, needs_token_type_ids,
            );
            for mut f in line_findings {
                f.offset += line_start;
                findings.push(f);
            }
        }

        line_start = line_end + 1;
    }

    findings
}

fn zapcnsa_process_line(
    line: &str,
    tokenizer: &Tokenizer,
    session: &mut Session,
    id2label: &[String],
    needs_token_type_ids: bool,
) -> Vec<apcnsa_Finding> {
    let encoding = tokenizer.encode(line, true)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("tokenize line: {}", e));

    let ids     = encoding.get_ids();
    let mask    = encoding.get_attention_mask();
    let offsets = encoding.get_offsets();

    let seq_len = ids.len();
    if seq_len == 0 {
        return Vec::new();
    }

    let ids_i64:  Vec<i64> = ids.iter().map(|&x| x as i64).collect();
    let mask_i64: Vec<i64> = mask.iter().map(|&x| x as i64).collect();

    let shape: [i64; 2] = [1, seq_len as i64];

    let mut inputs: HashMap<&str, Value> = HashMap::new();
    inputs.insert("input_ids",
        Value::from_array((shape, ids_i64))
            .unwrap_or_else(|e| apcd::apcrl_fatal_now!("input_ids value: {}", e))
            .into_dyn());
    inputs.insert("attention_mask",
        Value::from_array((shape, mask_i64))
            .unwrap_or_else(|e| apcd::apcrl_fatal_now!("attention_mask value: {}", e))
            .into_dyn());
    if needs_token_type_ids {
        let tt: Vec<i64> = vec![0i64; seq_len];
        inputs.insert("token_type_ids",
            Value::from_array((shape, tt))
                .unwrap_or_else(|e| apcd::apcrl_fatal_now!("token_type_ids value: {}", e))
                .into_dyn());
    }

    let outputs = session.run(inputs)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("session run: {}", e));

    // First (and only) output is logits: [1, seq_len, num_labels].
    let (out_shape, logits_flat) = outputs[0]
        .try_extract_tensor::<f32>()
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("logits extract: {}", e));

    let dims: Vec<i64> = out_shape.iter().copied().collect();
    if dims.len() != 3 || dims[0] != 1 || dims[1] != seq_len as i64 {
        apcd::apcrl_fatal_now!(
            "unexpected logits shape: {:?} (expected [1, {}, num_labels])",
            dims, seq_len
        );
    }
    let num_labels = dims[2] as usize;
    if num_labels != id2label.len() {
        apcd::apcrl_fatal_now!(
            "logits label dim {} != id2label.len() {}", num_labels, id2label.len()
        );
    }

    // argmax per token — logits_flat is row-major [seq_len × num_labels].
    let mut pred_labels: Vec<&str> = Vec::with_capacity(seq_len);
    for t in 0..seq_len {
        let row = &logits_flat[t * num_labels .. (t + 1) * num_labels];
        let mut best_idx = 0usize;
        let mut best_val = f32::NEG_INFINITY;
        for (i, &v) in row.iter().enumerate() {
            if v > best_val { best_val = v; best_idx = i; }
        }
        pred_labels.push(&id2label[best_idx]);
    }

    zapcnsa_decode_bio(&pred_labels, offsets)
}

// ---------------------------------------------------------------------------
// Entity decoder — handles both BIO-encoded labels (B-X / I-X / O) and flat
// labels (kind-per-token, no B-/I- prefix). Stanford deidentifier-base uses
// the flat form; other models use BIO.
//
// Flat form: consecutive tokens with the same kind are merged into one span;
// kind change (including to/from "O") is a boundary.
//
// BIO form: B-X always starts a new span; I-X extends if kind matches, else
// starts a new span; O always closes.
//
// Special tokens ([CLS], [SEP], [PAD]) have offsets (0, 0) and are skipped.
// ---------------------------------------------------------------------------

enum zapcnsa_TagRole {
    Begin(String),    // start a new span (or extend if same kind, for flat form)
    Inside(String),   // extend current span of this kind
    Outside,          // close any current span
}

fn zapcnsa_classify_label(label: &str) -> zapcnsa_TagRole {
    if label == "O" {
        return zapcnsa_TagRole::Outside;
    }
    match label.split_once('-') {
        Some(("B", kind)) => zapcnsa_TagRole::Begin(kind.to_string()),
        Some(("I", kind)) => zapcnsa_TagRole::Inside(kind.to_string()),
        Some((_, kind))   => zapcnsa_TagRole::Begin(kind.to_string()),
        None              => zapcnsa_TagRole::Inside(label.to_string()), // flat: extend if same kind
    }
}

fn zapcnsa_decode_bio(
    labels: &[&str],
    offsets: &[(usize, usize)],
) -> Vec<apcnsa_Finding> {
    let mut findings = Vec::new();
    let mut cur_kind:  Option<String> = None;
    let mut cur_start: usize = 0;
    let mut cur_end:   usize = 0;

    let flush = |cur_kind: &Option<String>, start: usize, end: usize, out: &mut Vec<apcnsa_Finding>| {
        if let Some(k) = cur_kind {
            if end > start {
                out.push(apcnsa_Finding {
                    offset: start,
                    length: end - start,
                    label:  k.clone(),
                });
            }
        }
    };

    for (i, &label) in labels.iter().enumerate() {
        let (tok_start, tok_end) = offsets[i];
        if tok_start == 0 && tok_end == 0 {
            continue;
        }

        match zapcnsa_classify_label(label) {
            zapcnsa_TagRole::Outside => {
                flush(&cur_kind, cur_start, cur_end, &mut findings);
                cur_kind = None;
            }
            zapcnsa_TagRole::Begin(kind) => {
                flush(&cur_kind, cur_start, cur_end, &mut findings);
                cur_kind  = Some(kind);
                cur_start = tok_start;
                cur_end   = tok_end;
            }
            zapcnsa_TagRole::Inside(kind) => {
                if cur_kind.as_deref() == Some(kind.as_str()) {
                    cur_end = tok_end.max(cur_end);
                } else {
                    flush(&cur_kind, cur_start, cur_end, &mut findings);
                    cur_kind  = Some(kind);
                    cur_start = tok_start;
                    cur_end   = tok_end;
                }
            }
        }
    }

    flush(&cur_kind, cur_start, cur_end, &mut findings);
    findings
}

// ---------------------------------------------------------------------------
// Rendering — mirrors apcab's guillemet format but only emits raw stanford labels.
// ---------------------------------------------------------------------------

fn zapcnsa_render_assay(plain_text: &str, findings: &[apcnsa_Finding]) -> String {
    let mut sorted: Vec<&apcnsa_Finding> = findings.iter().collect();
    sorted.sort_by_key(|f| (f.offset, std::cmp::Reverse(f.length)));

    let mut result = String::with_capacity(plain_text.len() * 2);
    let mut pos = 0usize;

    for finding in &sorted {
        if finding.offset < pos {
            continue;
        }
        if finding.offset > pos {
            result.push_str(&plain_text[pos..finding.offset]);
        }
        let end = finding.offset + finding.length;
        if end > plain_text.len() {
            apcd::apcrl_error_now!(
                "finding out of bounds: offset={} length={} plain_text_len={}",
                finding.offset, finding.length, plain_text.len()
            );
            continue;
        }
        let matched = &plain_text[finding.offset..end];

        result.push('\u{2039}'); // ‹
        result.push_str(&finding.label);
        result.push(' ');
        result.push_str(matched);
        result.push('\u{203A}'); // ›

        pos = end;
    }

    if pos < plain_text.len() {
        result.push_str(&plain_text[pos..]);
    }

    result
}
