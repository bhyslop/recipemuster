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
#![allow(private_interfaces)]

//! Batch assay — runs parse + detection pipeline on HTML files, writes
//! guillemet-delimited assay output showing PHI classifications inline.
//! Usage: cargo run --bin apcab <input-directory>

// RCG output discipline: all emission via apcrl_*! — no direct println!/eprintln!

use apcd::apcrd_dictionaries::apcrd_Dictionaries;
use apcd::apcre_engine::{apcre_analyze, apcre_Result};
use apcd::apcrm_match::{apcrm_Finding, apcrm_PhiCategory, apcrm_Severity};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        apcd::apcrl_fatal_now!("usage: apcab <input-directory>");
    }
    let input_dir = &args[1];

    let output_dir = std::env::var("BURD_OUTPUT_DIR")
        .unwrap_or_else(|_| apcd::apcrl_fatal_now!("BURD_OUTPUT_DIR not set"));

    let entries: Vec<_> = std::fs::read_dir(input_dir)
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("failed to read directory {}: {}", input_dir, e))
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

    apcd::apcrl_info_now!("loading dictionaries");
    let dicts = apcrd_Dictionaries::apcrd_load();

    let mut processed = 0u32;
    for path in &entries {
        let stem = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        apcd::apcrl_info_now!("processing: {}", path.display());

        let html = std::fs::read_to_string(path)
            .unwrap_or_else(|e| apcd::apcrl_fatal_now!("failed to read {}: {}", path.display(), e));

        let result = apcre_analyze(&html, &dicts);

        let assay_text = match result {
            apcre_Result::Clinical { findings, plain_text } => {
                zapcab_render_assay(&plain_text, &findings)
            }
            apcre_Result::NonClinical { content_type, preview, .. } => {
                format!("[non-clinical content: {} — {}]\n", content_type, preview)
            }
        };

        let output_path = std::path::Path::new(&output_dir).join(format!("{}.assay.txt", stem));
        std::fs::write(&output_path, &assay_text)
            .unwrap_or_else(|e| apcd::apcrl_fatal_now!(
                "failed to write {}: {}", output_path.display(), e
            ));

        apcd::apcrl_info_now!("wrote {} ({} bytes)", output_path.display(), assay_text.len());
        processed += 1;
    }

    apcd::apcrl_info_now!("batch assay complete — {} files processed", processed);
}

fn zapcab_render_assay(plain_text: &str, findings: &[apcrm_Finding]) -> String {
    let mut sorted: Vec<&apcrm_Finding> = findings.iter().collect();
    sorted.sort_by_key(|f| (f.offset, std::cmp::Reverse(f.length)));

    let mut result = String::with_capacity(plain_text.len() * 2);
    let mut pos = 0;

    for finding in &sorted {
        if finding.offset < pos {
            continue;
        }

        if finding.offset > pos {
            result.push_str(&plain_text[pos..finding.offset]);
        }

        let severity_tag = zapcab_severity_tag(finding.severity);
        let category_tag = zapcab_category_tag(finding.category);
        let matched_text = &plain_text[finding.offset..finding.offset + finding.length];

        result.push('\u{2039}'); // ‹
        result.push_str(severity_tag);
        result.push(':');
        result.push_str(category_tag);
        result.push(' ');
        result.push_str(matched_text);
        result.push('\u{203A}'); // ›

        pos = finding.offset + finding.length;
    }

    if pos < plain_text.len() {
        result.push_str(&plain_text[pos..]);
    }

    result
}

fn zapcab_severity_tag(severity: apcrm_Severity) -> &'static str {
    match severity {
        apcrm_Severity::Red    => "RED",
        apcrm_Severity::Yellow => "YELLOW",
        apcrm_Severity::Pass   => "PASS",
    }
}

fn zapcab_category_tag(category: apcrm_PhiCategory) -> &'static str {
    match category {
        apcrm_PhiCategory::Name         => "NAME",
        apcrm_PhiCategory::Provider     => "PROVIDER",
        apcrm_PhiCategory::Facility     => "FACILITY",
        apcrm_PhiCategory::Date         => "DATE",
        apcrm_PhiCategory::Dob          => "DOB",
        apcrm_PhiCategory::Phone        => "PHONE",
        apcrm_PhiCategory::Email        => "EMAIL",
        apcrm_PhiCategory::Ssn          => "SSN",
        apcrm_PhiCategory::Mrn          => "MRN",
        apcrm_PhiCategory::Account      => "ACCOUNT",
        apcrm_PhiCategory::EncounterId  => "ENCOUNTER_ID",
        apcrm_PhiCategory::Address      => "ADDRESS",
        apcrm_PhiCategory::HealthPlanId => "HEALTH_PLAN_ID",
        apcrm_PhiCategory::DeviceId     => "DEVICE_ID",
        apcrm_PhiCategory::Npi          => "NPI",
        apcrm_PhiCategory::Dea          => "DEA",
        apcrm_PhiCategory::Url          => "URL",
        apcrm_PhiCategory::IpAddress    => "IP_ADDRESS",
    }
}
