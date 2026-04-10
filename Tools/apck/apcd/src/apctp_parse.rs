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

//! Tests for HTML clipboard parsing.

use super::apcrp_parse::*;

const PROGRESS_NOTE: &str = include_str!("../../test_fixtures/epic_progress_note.html");
const GERIATRIC_CONSULT: &str = include_str!("../../test_fixtures/epic_geriatric_consult.html");

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn labeled_fields(doc: &apcrp_Document) -> Vec<(&str, &str)> {
    doc.spans
        .iter()
        .filter(|s| s.kind == apcrp_SpanKind::LabeledField)
        .map(|s| (s.label.as_deref().unwrap(), s.text.as_str()))
        .collect()
}

fn section_headers(doc: &apcrp_Document) -> Vec<&str> {
    doc.spans
        .iter()
        .filter(|s| s.kind == apcrp_SpanKind::SectionHeader)
        .map(|s| s.label.as_deref().unwrap())
        .collect()
}

fn narrative_texts(doc: &apcrp_Document) -> Vec<&str> {
    doc.spans
        .iter()
        .filter(|s| s.kind == apcrp_SpanKind::Narrative)
        .map(|s| s.text.as_str())
        .collect()
}

// ---------------------------------------------------------------------------
// Progress note — labeled fields
// ---------------------------------------------------------------------------

#[test]
fn apctp_progress_note_patient_label() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    let fields = labeled_fields(&doc);
    let patient = fields.iter().find(|(l, _)| *l == "Patient");
    assert!(patient.is_some(), "Patient labeled field not found");
    assert_eq!(patient.unwrap().1, "Margaret J. Thornton");
}

#[test]
fn apctp_progress_note_dob_label() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    let fields = labeled_fields(&doc);
    let dob = fields.iter().find(|(l, _)| *l == "DOB");
    assert!(dob.is_some(), "DOB labeled field not found");
    assert_eq!(dob.unwrap().1, "03/15/1952");
}

#[test]
fn apctp_progress_note_mrn_label() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    let fields = labeled_fields(&doc);
    let mrn = fields.iter().find(|(l, _)| *l == "MRN");
    assert!(mrn.is_some(), "MRN labeled field not found");
    assert_eq!(mrn.unwrap().1, "00847293");
}

#[test]
fn apctp_progress_note_attending_label() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    let fields = labeled_fields(&doc);
    let attending = fields.iter().find(|(l, _)| *l == "Attending");
    assert!(attending.is_some(), "Attending labeled field not found");
    assert_eq!(attending.unwrap().1, "James R. Whitfield, MD");
}

#[test]
fn apctp_progress_note_facility_label() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    let fields = labeled_fields(&doc);
    let facility = fields.iter().find(|(l, _)| *l == "Facility");
    assert!(facility.is_some(), "Facility labeled field not found");
    assert_eq!(facility.unwrap().1, "Maine Medical Center");
}

#[test]
fn apctp_progress_note_esigned_label() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    let fields = labeled_fields(&doc);
    let esigned = fields.iter().find(|(l, _)| *l == "Electronically signed by");
    assert!(esigned.is_some(), "Electronically signed by not found");
    assert!(
        esigned.unwrap().1.contains("James R. Whitfield"),
        "Signature value should contain provider name"
    );
}

#[test]
fn apctp_progress_note_phi_fields_present() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    let fields = labeled_fields(&doc);
    let labels: Vec<&str> = fields.iter().map(|(l, _)| *l).collect();

    for expected in &[
        "Patient", "DOB", "MRN", "Account", "SSN", "Address", "Phone",
        "Fax", "Email", "Emergency Contact", "Facility", "Attending",
        "Referring", "Provider", "Electronically signed by",
    ] {
        assert!(
            labels.contains(expected),
            "Missing labeled field: {expected}"
        );
    }
}

// ---------------------------------------------------------------------------
// Progress note — section headers
// ---------------------------------------------------------------------------

#[test]
fn apctp_progress_note_sections() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    let headers = section_headers(&doc);

    for expected in &[
        "Chief Complaint",
        "History of Present Illness",
        "Past Medical History",
        "Allergies",
        "Imaging",
        "Assessment/Plan",
    ] {
        assert!(
            headers.contains(expected),
            "Missing section header: {expected}"
        );
    }
}

#[test]
fn apctp_progress_note_medications_section() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    let headers = section_headers(&doc);
    assert!(
        headers.iter().any(|h| h.starts_with("Medications")),
        "Medications section not found"
    );
}

#[test]
fn apctp_progress_note_vitals_section() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    let headers = section_headers(&doc);
    assert!(
        headers.iter().any(|h| h.starts_with("Vitals")),
        "Vitals section not found"
    );
}

#[test]
fn apctp_progress_note_lab_section() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    let headers = section_headers(&doc);
    assert!(
        headers.iter().any(|h| h.starts_with("Laboratory Results")),
        "Laboratory Results section not found"
    );
}

// ---------------------------------------------------------------------------
// Progress note — narrative
// ---------------------------------------------------------------------------

#[test]
fn apctp_progress_note_hpi_narrative() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    let narratives = narrative_texts(&doc);
    let hpi = narratives.iter().find(|t| t.contains("Ms. Thornton"));
    assert!(hpi.is_some(), "HPI narrative not found");
    let hpi = hpi.unwrap();
    assert!(hpi.contains("substernal chest pain"), "HPI should contain clinical detail");
    assert!(hpi.contains("Robert Thornton"), "HPI should contain embedded names");
    assert!(hpi.contains("Dorothy Kowalski"), "HPI should contain family history names");
}

#[test]
fn apctp_progress_note_ap_narrative() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    let narratives = narrative_texts(&doc);
    let ap = narratives.iter().find(|t| t.contains("hemodynamically stable"));
    assert!(ap.is_some(), "A/P narrative not found");
    let ap = ap.unwrap();
    assert!(ap.contains("Dr. Michael Torres"), "A/P should contain consulting provider name");
}

// ---------------------------------------------------------------------------
// Geriatric consult — labeled fields
// ---------------------------------------------------------------------------

#[test]
fn apctp_geriatric_patient_label() {
    let doc = apcrp_parse(GERIATRIC_CONSULT);
    let fields = labeled_fields(&doc);
    let patient = fields.iter().find(|(l, _)| *l == "Patient");
    assert!(patient.is_some(), "Patient labeled field not found");
    assert_eq!(patient.unwrap().1, "Harold W. Eriksen");
}

#[test]
fn apctp_geriatric_consulting_label() {
    let doc = apcrp_parse(GERIATRIC_CONSULT);
    let fields = labeled_fields(&doc);
    let consulting = fields.iter().find(|(l, _)| *l == "Consulting");
    assert!(consulting.is_some(), "Consulting labeled field not found");
    assert!(consulting.unwrap().1.contains("Naomi Okwu"));
}

// ---------------------------------------------------------------------------
// Geriatric consult — sections
// ---------------------------------------------------------------------------

#[test]
fn apctp_geriatric_sections() {
    let doc = apcrp_parse(GERIATRIC_CONSULT);
    let headers = section_headers(&doc);

    for expected in &[
        "Reason for Consult",
        "History of Present Illness",
        "Past Medical History",
        "Cognitive Assessment",
        "Assessment/Plan",
    ] {
        assert!(
            headers.contains(expected),
            "Missing section header: {expected}"
        );
    }
}

// ---------------------------------------------------------------------------
// Geriatric consult — narrative
// ---------------------------------------------------------------------------

#[test]
fn apctp_geriatric_hpi_narrative() {
    let doc = apcrp_parse(GERIATRIC_CONSULT);
    let narratives = narrative_texts(&doc);
    let hpi = narratives.iter().find(|t| t.contains("Mr. Eriksen"));
    assert!(hpi.is_some(), "HPI narrative not found");
    let hpi = hpi.unwrap();
    assert!(hpi.contains("Karen Eriksen-Moody"), "HPI should contain caregiver name");
    assert!(hpi.contains("Thomas Eriksen"), "HPI should contain family member name");
    assert!(hpi.contains("Patricia Nowak"), "HPI should contain aide name");
}

// ---------------------------------------------------------------------------
// Plain text — structure
// ---------------------------------------------------------------------------

#[test]
fn apctp_plain_text_contains_labels() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    assert!(doc.plain_text.contains("Patient: Margaret J. Thornton"));
    assert!(doc.plain_text.contains("DOB: 03/15/1952"));
    assert!(doc.plain_text.contains("MRN: 00847293"));
}

#[test]
fn apctp_plain_text_contains_sections() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    assert!(doc.plain_text.contains("Chief Complaint:\n"));
    assert!(doc.plain_text.contains("History of Present Illness:\n"));
    assert!(doc.plain_text.contains("Assessment/Plan:\n"));
}

#[test]
fn apctp_plain_text_contains_narrative() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    assert!(doc.plain_text.contains("substernal chest pain"));
    assert!(doc.plain_text.contains("hemodynamically stable"));
}

// ---------------------------------------------------------------------------
// Offsets — consistency
// ---------------------------------------------------------------------------

#[test]
fn apctp_offsets_monotonically_increasing() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    for window in doc.spans.windows(2) {
        assert!(
            window[1].offset >= window[0].offset,
            "Offsets must be non-decreasing: {} >= {} failed for {:?} vs {:?}",
            window[1].offset,
            window[0].offset,
            window[0].label,
            window[1].label,
        );
    }
}

#[test]
fn apctp_offsets_within_plain_text() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    for span in &doc.spans {
        assert!(
            span.offset <= doc.plain_text.len(),
            "Offset {} exceeds plain_text length {} for {:?}",
            span.offset,
            doc.plain_text.len(),
            span.label,
        );
    }
}

#[test]
fn apctp_labeled_field_offset_points_to_label() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    for span in &doc.spans {
        if span.kind == apcrp_SpanKind::LabeledField {
            let label = span.label.as_ref().unwrap();
            let at_offset = &doc.plain_text[span.offset..];
            assert!(
                at_offset.starts_with(label),
                "Offset for '{label}' should point to label text in plain_text, got: {:?}",
                &at_offset[..at_offset.len().min(40)],
            );
        }
    }
}

#[test]
fn apctp_section_header_offset_points_to_header() {
    let doc = apcrp_parse(PROGRESS_NOTE);
    for span in &doc.spans {
        if span.kind == apcrp_SpanKind::SectionHeader {
            let label = span.label.as_ref().unwrap();
            let at_offset = &doc.plain_text[span.offset..];
            assert!(
                at_offset.starts_with(label),
                "Offset for section '{label}' should point to header in plain_text",
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Section header classification
// ---------------------------------------------------------------------------

#[test]
fn apctp_section_header_classification() {
    assert!(zapcrp_is_section_header("Chief Complaint"));
    assert!(zapcrp_is_section_header("Medications (Home)"));
    assert!(zapcrp_is_section_header("Vitals (04/08/2026 06:00)"));
    assert!(zapcrp_is_section_header("Laboratory Results (04/08/2026)"));
    assert!(zapcrp_is_section_header("Assessment/Plan"));
}

#[test]
fn apctp_non_section_labels() {
    assert!(!zapcrp_is_section_header("Patient"));
    assert!(!zapcrp_is_section_header("DOB"));
    assert!(!zapcrp_is_section_header("Attending"));
    assert!(!zapcrp_is_section_header("Electronically signed by"));
    assert!(!zapcrp_is_section_header("Facility"));
}
