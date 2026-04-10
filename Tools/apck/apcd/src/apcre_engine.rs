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

//! PHI detection orchestrator — merges three tiers by highest severity.

use crate::apcrd_dictionaries::apcrd_Dictionaries;
use crate::apcrm_match::*;
use crate::apcrp_parse;
use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

pub enum apcre_Result {
    Clinical {
        findings:   Vec<apcrm_Finding>,
        plain_text: String,
    },
    NonClinical {
        content_length: usize,
        content_type:   String,
        preview:        String,
    },
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const ZAPCRE_CLINICAL_LABELS: &[&str] = &[
    "patient:",
    "dob:",
    "mrn:",
    "attending:",
    "facility:",
    "chief complaint:",
    "history of present illness:",
    "assessment/plan:",
    "vitals:",
    "laboratory results:",
    "medications:",
    "allergies:",
    "physical exam:",
    "review of systems:",
];

const ZAPCRE_CLINICAL_THRESHOLD: usize = 2;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

pub fn apcre_analyze(html: &str, dicts: &apcrd_Dictionaries) -> apcre_Result {
    let today = chrono::Local::now().date_naive();
    zapcre_analyze_with_date(html, dicts, today)
}

pub(crate) fn zapcre_analyze_with_date(
    html: &str,
    dicts: &apcrd_Dictionaries,
    today: NaiveDate,
) -> apcre_Result {
    let doc = apcrp_parse::apcrp_parse(html);

    if !zapcre_is_clinical(&doc.plain_text) {
        return apcre_Result::NonClinical {
            content_length: html.len(),
            content_type:   if html.contains('<') { "HTML".to_string() } else { "plain text".to_string() },
            preview:        html.chars().take(100).collect(),
        };
    }

    let mut findings = Vec::new();
    findings.extend(apcrm_scan_regex(&doc.plain_text));
    findings.extend(apcrm_scan_anchored(&doc.spans));
    findings.extend(apcrm_scan_dictionary(&doc.plain_text, dicts));

    zapcre_transform_dob(&mut findings, &doc, today);

    findings = zapcre_merge_findings(findings);
    findings.sort_by_key(|f| f.offset);

    apcre_Result::Clinical {
        findings,
        plain_text: doc.plain_text,
    }
}

// ---------------------------------------------------------------------------
// Internal — clinical content heuristic
// ---------------------------------------------------------------------------

fn zapcre_is_clinical(plain_text: &str) -> bool {
    let lower = plain_text.to_lowercase();
    let count = ZAPCRE_CLINICAL_LABELS
        .iter()
        .filter(|label| lower.contains(**label))
        .count();
    count >= ZAPCRE_CLINICAL_THRESHOLD
}

// ---------------------------------------------------------------------------
// Internal — DOB → age transform
// ---------------------------------------------------------------------------

fn zapcre_transform_dob(
    findings: &mut [apcrm_Finding],
    doc: &apcrp_parse::apcrp_Document,
    today: NaiveDate,
) {
    let dob_span = doc.spans.iter().find(|s| {
        s.kind == apcrp_parse::apcrp_SpanKind::LabeledField
            && s.label.as_deref() == Some("DOB")
    });

    let dob_span = match dob_span {
        Some(s) => s,
        None    => return,
    };

    let dob_date = match zapcre_parse_date(&dob_span.text) {
        Some(d) => d,
        None    => return,
    };

    let age = zapcre_compute_age(dob_date, today);
    let age_str = if age > 89 {
        "Age: 90+".to_string()
    } else {
        format!("Age: {}", age)
    };

    let value_offset = dob_span.offset + "DOB".len() + 2;
    let value_end    = value_offset + dob_span.text.len();

    for finding in findings.iter_mut() {
        if finding.category == apcrm_PhiCategory::Date
            && finding.offset >= value_offset
            && finding.offset + finding.length <= value_end
        {
            finding.category    = apcrm_PhiCategory::Dob;
            finding.replacement = age_str.clone();
        }
    }
}

fn zapcre_parse_date(text: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(text.trim(), "%m/%d/%Y")
        .or_else(|_| NaiveDate::parse_from_str(text.trim(), "%m-%d-%Y"))
        .or_else(|_| NaiveDate::parse_from_str(text.trim(), "%m/%d/%y"))
        .ok()
}

pub(crate) fn zapcre_compute_age(dob: NaiveDate, today: NaiveDate) -> u32 {
    let mut age = today.year() - dob.year();
    if today.month() < dob.month()
        || (today.month() == dob.month() && today.day() < dob.day())
    {
        age -= 1;
    }
    age.max(0) as u32
}

// ---------------------------------------------------------------------------
// Internal — merge findings by highest severity at each position
// ---------------------------------------------------------------------------

fn zapcre_merge_findings(findings: Vec<apcrm_Finding>) -> Vec<apcrm_Finding> {
    let mut by_position: HashMap<(usize, usize), apcrm_Finding> = HashMap::new();

    for finding in findings {
        let key = (finding.offset, finding.length);
        match by_position.get(&key) {
            Some(existing)
                if apcrm_severity_rank(existing.severity)
                    >= apcrm_severity_rank(finding.severity) =>
            {
                // existing has equal or higher severity — keep it
            }
            _ => {
                by_position.insert(key, finding);
            }
        }
    }

    by_position.into_values().collect()
}
