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

//! HTML clipboard parsing — extracts typed spans with labels and positions.

use scraper::{ElementRef, Html, Node};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum apcrp_SpanKind {
    LabeledField,
    SectionHeader,
    Narrative,
}

/// A typed span extracted from Epic HTML.
///
/// `offset` is the start position of this span's contribution in the
/// plain-text output.  For position mapping: a character at position P in
/// `plain_text` belongs to the span whose `offset` is the largest value ≤ P.
#[derive(Debug, Clone)]
pub struct apcrp_Span {
    pub kind:   apcrp_SpanKind,
    pub label:  Option<String>,
    pub text:   String,
    pub offset: usize,
}

/// Parsed Epic HTML document — typed spans plus derived plain text.
#[derive(Debug)]
pub struct apcrp_Document {
    pub spans:      Vec<apcrp_Span>,
    pub plain_text: String,
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Known Epic section header prefixes.  Matching uses `starts_with` so that
/// suffixes like "(Home)" or "(04/08/2026 06:00)" pass through.
pub(crate) const ZAPCRP_SECTION_PREFIXES: &[&str] = &[
    "Assessment/Plan",
    "Allergies",
    "Chief Complaint",
    "Cognitive Assessment",
    "Family History",
    "History of Present Illness",
    "Imaging",
    "Laboratory Results",
    "Medications",
    "Past Medical History",
    "Physical Exam",
    "Reason for Consult",
    "Review of Systems",
    "Social History",
    "Vitals",
];

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

pub fn apcrp_parse(html: &str) -> apcrp_Document {
    let tree = Html::parse_document(html);
    let root = tree.root_element();
    let mut tokens = Vec::new();
    zapcrp_tokenize_element(root, &mut tokens);
    zapcrp_build_document(&tokens)
}

// ---------------------------------------------------------------------------
// Internal — tokenizer
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub(crate) enum zapcrp_Token {
    Bold(String),
    Text(String),
    LineBreak,
    BlockStart,
    BlockEnd,
}

fn zapcrp_tokenize_element(elem: ElementRef, tokens: &mut Vec<zapcrp_Token>) {
    let tag = elem.value().name.local.as_ref();
    match tag {
        "b" | "strong" => {
            let text: String = elem.text().collect();
            if !text.is_empty() {
                tokens.push(zapcrp_Token::Bold(text));
            }
        }
        "br" => {
            tokens.push(zapcrp_Token::LineBreak);
        }
        "div" | "table" => {
            tokens.push(zapcrp_Token::BlockStart);
            zapcrp_visit_children(elem, tokens);
            tokens.push(zapcrp_Token::BlockEnd);
        }
        "tr" => {
            zapcrp_visit_children(elem, tokens);
            tokens.push(zapcrp_Token::LineBreak);
        }
        _ => {
            zapcrp_visit_children(elem, tokens);
        }
    }
}

fn zapcrp_visit_children(elem: ElementRef, tokens: &mut Vec<zapcrp_Token>) {
    for child in elem.children() {
        match child.value() {
            Node::Text(t) => {
                let s: &str = t;
                if !s.is_empty() {
                    tokens.push(zapcrp_Token::Text(s.to_string()));
                }
            }
            Node::Element(_) => {
                if let Some(child_elem) = ElementRef::wrap(child) {
                    zapcrp_tokenize_element(child_elem, tokens);
                }
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------
// Internal — span builder
// ---------------------------------------------------------------------------

pub(crate) fn zapcrp_is_section_header(label: &str) -> bool {
    ZAPCRP_SECTION_PREFIXES
        .iter()
        .any(|&prefix| label.starts_with(prefix))
}

fn zapcrp_build_document(tokens: &[zapcrp_Token]) -> apcrp_Document {
    let mut spans = Vec::new();
    let mut plain_text = String::new();
    let mut narrative_buf = String::new();
    let mut narrative_offset: usize = 0;
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            zapcrp_Token::Bold(raw) if zapcrp_is_bold_label(raw) => {
                zapcrp_flush_narrative(
                    &mut narrative_buf,
                    narrative_offset,
                    &mut spans,
                    &mut plain_text,
                );

                let label = raw.trim_end().trim_end_matches(':').trim().to_string();

                if zapcrp_is_section_header(&label) {
                    let offset = plain_text.len();
                    plain_text.push_str(&label);
                    plain_text.push_str(":\n");
                    spans.push(apcrp_Span {
                        kind:   apcrp_SpanKind::SectionHeader,
                        label:  Some(label),
                        text:   String::new(),
                        offset,
                    });
                    i += 1;
                    if i < tokens.len() && matches!(&tokens[i], zapcrp_Token::LineBreak) {
                        i += 1;
                    }
                    narrative_offset = plain_text.len();
                    continue;
                }

                // Labeled field — collect value until next boundary.
                let offset = plain_text.len();
                let mut value_parts: Vec<String> = Vec::new();
                i += 1;
                while i < tokens.len() {
                    match &tokens[i] {
                        zapcrp_Token::Bold(t) if zapcrp_is_bold_label(t) => break,
                        zapcrp_Token::LineBreak
                        | zapcrp_Token::BlockStart
                        | zapcrp_Token::BlockEnd => {
                            i += 1;
                            break;
                        }
                        zapcrp_Token::Text(t) => value_parts.push(t.clone()),
                        zapcrp_Token::Bold(t) => value_parts.push(t.clone()),
                    }
                    i += 1;
                }
                let value = zapcrp_join_inline(&value_parts);
                plain_text.push_str(&label);
                plain_text.push_str(": ");
                plain_text.push_str(&value);
                plain_text.push('\n');
                spans.push(apcrp_Span {
                    kind:   apcrp_SpanKind::LabeledField,
                    label:  Some(label),
                    text:   value,
                    offset,
                });
                narrative_offset = plain_text.len();
                continue;
            }

            zapcrp_Token::Bold(text) => {
                zapcrp_append_narrative(&mut narrative_buf, &mut narrative_offset, &plain_text, text);
            }
            zapcrp_Token::Text(text) => {
                zapcrp_append_narrative(&mut narrative_buf, &mut narrative_offset, &plain_text, text);
            }
            zapcrp_Token::LineBreak => {
                if !narrative_buf.is_empty() {
                    narrative_buf.push('\n');
                }
            }
            zapcrp_Token::BlockStart | zapcrp_Token::BlockEnd => {
                zapcrp_flush_narrative(
                    &mut narrative_buf,
                    narrative_offset,
                    &mut spans,
                    &mut plain_text,
                );
                narrative_offset = plain_text.len();
            }
        }
        i += 1;
    }

    zapcrp_flush_narrative(&mut narrative_buf, narrative_offset, &mut spans, &mut plain_text);

    apcrp_Document { spans, plain_text }
}

/// True when a Bold token looks like a label: trimmed text ends with ':'.
fn zapcrp_is_bold_label(raw: &str) -> bool {
    raw.trim_end().ends_with(':')
}

/// Append raw text to the narrative buffer, recording the start offset on
/// first non-empty append.
fn zapcrp_append_narrative(
    buf: &mut String,
    offset: &mut usize,
    plain_text: &str,
    raw: &str,
) {
    if buf.is_empty() {
        *offset = plain_text.len();
    }
    buf.push_str(raw);
}

/// Flush the narrative buffer into a Narrative span.
fn zapcrp_flush_narrative(
    buf: &mut String,
    offset: usize,
    spans: &mut Vec<apcrp_Span>,
    plain_text: &mut String,
) {
    if buf.is_empty() {
        return;
    }
    let normalized = zapcrp_normalize_whitespace(buf);
    if !normalized.is_empty() {
        plain_text.push_str(&normalized);
        plain_text.push('\n');
        spans.push(apcrp_Span {
            kind:   apcrp_SpanKind::Narrative,
            label:  None,
            text:   normalized,
            offset,
        });
    }
    buf.clear();
}

/// Join inline text fragments: concatenate then collapse whitespace.
fn zapcrp_join_inline(parts: &[String]) -> String {
    let raw: String = parts.concat();
    zapcrp_normalize_whitespace(&raw)
}

/// Normalize whitespace: preserve paragraph breaks (`\n\n`), collapse single
/// newlines and runs of spaces to a single space, trim.
fn zapcrp_normalize_whitespace(s: &str) -> String {
    let paragraphs: Vec<&str> = s.split("\n\n").collect();
    let normalized: Vec<String> = paragraphs
        .iter()
        .map(|p| {
            p.split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|p| !p.is_empty())
        .collect();
    normalized.join("\n\n")
}
