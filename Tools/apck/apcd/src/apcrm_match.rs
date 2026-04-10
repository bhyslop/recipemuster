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

//! Dictionary and regex matching — Tier 1 structural patterns, Tier 2 label-anchored, Tier 3 dictionary.

use crate::apcrd_dictionaries::apcrd_Dictionaries;
use crate::apcrp_parse::{apcrp_Span, apcrp_SpanKind};
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum apcrm_Severity {
    Red,
    Yellow,
    Pass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum apcrm_Mechanism {
    Regex,
    Anchored,
    Matched,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum apcrm_PhiCategory {
    Name,
    Provider,
    Facility,
    Date,
    Dob,
    Phone,
    Email,
    Ssn,
    Mrn,
    Account,
    EncounterId,
    Address,
    HealthPlanId,
    DeviceId,
    Npi,
    Dea,
    Url,
    IpAddress,
}

impl apcrm_PhiCategory {
    pub fn apcrm_placeholder(&self) -> &'static str {
        match self {
            Self::Name         => "[NAME]",
            Self::Provider     => "[PROVIDER]",
            Self::Facility     => "[FACILITY]",
            Self::Date         => "[DATE]",
            Self::Dob          => "[DOB]",
            Self::Phone        => "[PHONE]",
            Self::Email        => "[EMAIL]",
            Self::Ssn          => "[SSN]",
            Self::Mrn          => "[MRN]",
            Self::Account      => "[ACCOUNT]",
            Self::EncounterId  => "[ENCOUNTER_ID]",
            Self::Address      => "[ADDRESS]",
            Self::HealthPlanId => "[HEALTH_PLAN_ID]",
            Self::DeviceId     => "[DEVICE_ID]",
            Self::Npi          => "[NPI]",
            Self::Dea          => "[DEA]",
            Self::Url          => "[URL]",
            Self::IpAddress    => "[IP_ADDRESS]",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct apcrm_Finding {
    pub text:        String,
    pub replacement: String,
    pub severity:    apcrm_Severity,
    pub mechanism:   apcrm_Mechanism,
    pub category:    apcrm_PhiCategory,
    pub offset:      usize,
    pub length:      usize,
}

// ---------------------------------------------------------------------------
// Tier 1 — Regex patterns (→ RED)
// ---------------------------------------------------------------------------

struct zapcrm_RegexDef {
    pattern:  &'static str,
    category: apcrm_PhiCategory,
    group:    usize,
}

const ZAPCRM_REGEX_DEFS: &[zapcrm_RegexDef] = &[
    // SSN
    zapcrm_RegexDef { pattern: r"\b\d{3}-\d{2}-\d{4}\b",                                                                              category: apcrm_PhiCategory::Ssn,          group: 0 },
    // Phone — (XXX) XXX-XXXX
    zapcrm_RegexDef { pattern: r"\(\d{3}\)\s?\d{3}-\d{4}",                                                                             category: apcrm_PhiCategory::Phone,        group: 0 },
    // Phone — XXX-XXX-XXXX or XXX.XXX.XXXX
    zapcrm_RegexDef { pattern: r"\b\d{3}[-\.]\d{3}[-\.]\d{4}\b",                                                                       category: apcrm_PhiCategory::Phone,        group: 0 },
    // Email
    zapcrm_RegexDef { pattern: r"[\w.+-]+@[\w-]+\.[\w.-]+",                                                                            category: apcrm_PhiCategory::Email,        group: 0 },
    // Dates — MM/DD/YYYY or MM/DD/YY
    zapcrm_RegexDef { pattern: r"\b\d{1,2}/\d{1,2}/\d{2,4}\b",                                                                        category: apcrm_PhiCategory::Date,         group: 0 },
    // URL
    zapcrm_RegexDef { pattern: r"https?://\S+",                                                                                         category: apcrm_PhiCategory::Url,          group: 0 },
    // IP address
    zapcrm_RegexDef { pattern: r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b",                                                             category: apcrm_PhiCategory::IpAddress,    group: 0 },
    // Street address — number + 1-3 words + suffix
    zapcrm_RegexDef { pattern: r"(?i)\b\d+\s+(?:\w+\s+){1,3}(?:St|Street|Dr|Drive|Ave|Avenue|Blvd|Boulevard|Rd|Road|Ln|Lane|Way|Ct|Court|Pl|Place|Cir|Circle)\b", category: apcrm_PhiCategory::Address, group: 0 },
    // Zip code — standalone 5-digit (known ambiguity: matches other 5-digit numbers)
    zapcrm_RegexDef { pattern: r"\b\d{5}(?:-\d{4})?\b",                                                                                category: apcrm_PhiCategory::Address,      group: 0 },
];

/// Labeled identifier patterns — value captured in group 1.
const ZAPCRM_LABELED_DEFS: &[zapcrm_RegexDef] = &[
    zapcrm_RegexDef { pattern: r"MRN:\s*([\w.-]+)",                          category: apcrm_PhiCategory::Mrn,          group: 1 },
    zapcrm_RegexDef { pattern: r"(?i)(?:Account|Acct):\s*([\w.-]+)",         category: apcrm_PhiCategory::Account,      group: 1 },
    zapcrm_RegexDef { pattern: r"Encounter\s+ID:\s*([\w.-]+)",               category: apcrm_PhiCategory::EncounterId,  group: 1 },
    zapcrm_RegexDef { pattern: r"Health\s+Plan\s+ID:\s*([\w.-]+)",           category: apcrm_PhiCategory::HealthPlanId, group: 1 },
    zapcrm_RegexDef { pattern: r"NPI:\s*(\d{10})\b",                         category: apcrm_PhiCategory::Npi,          group: 1 },
    zapcrm_RegexDef { pattern: r"DEA:\s*([A-Z]{1,2}\d{7})\b",               category: apcrm_PhiCategory::Dea,          group: 1 },
    zapcrm_RegexDef { pattern: r"(?i)(?:implant\s+)?serial:\s*([\w.-]+)",    category: apcrm_PhiCategory::DeviceId,     group: 1 },
];

pub fn apcrm_scan_regex(plain_text: &str) -> Vec<apcrm_Finding> {
    let mut findings = Vec::new();

    for def in ZAPCRM_REGEX_DEFS.iter().chain(ZAPCRM_LABELED_DEFS.iter()) {
        let re = Regex::new(def.pattern).expect("bad regex pattern");
        for cap in re.captures_iter(plain_text) {
            let m = match cap.get(def.group) {
                Some(m) => m,
                None    => continue,
            };
            findings.push(apcrm_Finding {
                text:        m.as_str().to_string(),
                replacement: def.category.apcrm_placeholder().to_string(),
                severity:    apcrm_Severity::Red,
                mechanism:   apcrm_Mechanism::Regex,
                category:    def.category,
                offset:      m.start(),
                length:      m.len(),
            });
        }
    }

    findings
}

// ---------------------------------------------------------------------------
// Tier 2 — Label-anchored extraction (→ RED)
// ---------------------------------------------------------------------------

struct zapcrm_AnchorDef {
    label:    &'static str,
    category: apcrm_PhiCategory,
}

const ZAPCRM_ANCHOR_LABELS: &[zapcrm_AnchorDef] = &[
    zapcrm_AnchorDef { label: "Patient",                  category: apcrm_PhiCategory::Name },
    zapcrm_AnchorDef { label: "Attending",                category: apcrm_PhiCategory::Provider },
    zapcrm_AnchorDef { label: "Provider",                 category: apcrm_PhiCategory::Provider },
    zapcrm_AnchorDef { label: "Referring",                category: apcrm_PhiCategory::Provider },
    zapcrm_AnchorDef { label: "Facility",                 category: apcrm_PhiCategory::Facility },
    zapcrm_AnchorDef { label: "Electronically signed by", category: apcrm_PhiCategory::Provider },
    zapcrm_AnchorDef { label: "Co-signed by",             category: apcrm_PhiCategory::Provider },
    zapcrm_AnchorDef { label: "Emergency Contact",        category: apcrm_PhiCategory::Name },
    zapcrm_AnchorDef { label: "Primary Care Physician",   category: apcrm_PhiCategory::Provider },
];

pub fn apcrm_scan_anchored(spans: &[apcrp_Span]) -> Vec<apcrm_Finding> {
    let mut findings = Vec::new();

    for span in spans {
        if span.kind != apcrp_SpanKind::LabeledField {
            continue;
        }
        let label = match &span.label {
            Some(l) => l,
            None    => continue,
        };
        let category = match zapcrm_match_anchor_label(label) {
            Some(c) => c,
            None    => continue,
        };

        let value_offset = span.offset + label.len() + 2; // "Label: " → label + ": "

        for (byte_idx, word) in span.text.unicode_word_indices() {
            findings.push(apcrm_Finding {
                text:        word.to_string(),
                replacement: category.apcrm_placeholder().to_string(),
                severity:    apcrm_Severity::Red,
                mechanism:   apcrm_Mechanism::Anchored,
                category,
                offset:      value_offset + byte_idx,
                length:      word.len(),
            });
        }
    }

    findings
}

fn zapcrm_match_anchor_label(label: &str) -> Option<apcrm_PhiCategory> {
    let lower = label.to_lowercase();
    for def in ZAPCRM_ANCHOR_LABELS {
        if lower == def.label.to_lowercase() {
            return Some(def.category);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Tier 3 — Dictionary blacklist/whitelist scan (→ YELLOW)
// ---------------------------------------------------------------------------

pub fn apcrm_scan_dictionary(
    plain_text: &str,
    dicts: &apcrd_Dictionaries,
) -> Vec<apcrm_Finding> {
    let lower = plain_text.to_lowercase();
    let mut findings = Vec::new();

    for mat in dicts.apcrd_automaton().find_iter(&lower) {
        let start = mat.start();
        let end   = mat.end();

        if !zapcrm_is_word_boundary(&lower, start, end) {
            continue;
        }

        let original_text = &plain_text[start..end];
        let matched_lower = &lower[start..end];

        // Whitelist check: collision (black+white) is still YELLOW per spec
        let _is_collision = dicts.apcrd_is_whitelisted(matched_lower);

        findings.push(apcrm_Finding {
            text:        original_text.to_string(),
            replacement: apcrm_PhiCategory::Name.apcrm_placeholder().to_string(),
            severity:    apcrm_Severity::Yellow,
            mechanism:   apcrm_Mechanism::Matched,
            category:    apcrm_PhiCategory::Name,
            offset:      start,
            length:      end - start,
        });
    }

    findings
}

fn zapcrm_is_word_boundary(text: &str, start: usize, end: usize) -> bool {
    let bytes = text.as_bytes();
    let before_ok = start == 0 || !bytes[start - 1].is_ascii_alphanumeric();
    let after_ok  = end == bytes.len() || !bytes[end].is_ascii_alphanumeric();
    before_ok && after_ok
}

// ---------------------------------------------------------------------------
// Merge utilities (used by engine)
// ---------------------------------------------------------------------------

pub fn apcrm_severity_rank(s: apcrm_Severity) -> u8 {
    match s {
        apcrm_Severity::Red    => 2,
        apcrm_Severity::Yellow => 1,
        apcrm_Severity::Pass   => 0,
    }
}
