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

//! Tests for PHI detection engine.

#[cfg(test)]
mod tests {
    use crate::apcrd_dictionaries::apcrd_Dictionaries;
    use crate::apcre_engine::*;
    use crate::apcrm_match::*;
    use chrono::NaiveDate;

    const FIXTURE_PROGRESS: &str = include_str!("../../test_fixtures/epic_progress_note.html");

    fn today() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 4, 10).unwrap()
    }

    // -----------------------------------------------------------------------
    // Clinical content heuristic
    // -----------------------------------------------------------------------

    #[test]
    fn test_clinical_heuristic_accepts_fixture() {
        let dicts = apcrd_Dictionaries::apcrd_load();
        let result = zapcre_analyze_with_date(FIXTURE_PROGRESS, &dicts, today());
        assert!(matches!(result, apcre_Result::Clinical { .. }));
    }

    #[test]
    fn test_clinical_heuristic_rejects_non_clinical() {
        let dicts = apcrd_Dictionaries::apcrd_load();
        let html = "<html><body>Hello world, this is a shopping list.</body></html>";
        let result = zapcre_analyze_with_date(html, &dicts, today());
        assert!(matches!(result, apcre_Result::NonClinical { .. }));
    }

    // -----------------------------------------------------------------------
    // DOB → age transform
    // -----------------------------------------------------------------------

    #[test]
    fn test_dob_age_normal() {
        // DOB 03/15/1952, today 04/10/2026 → age 74 (birthday already passed)
        let dob = NaiveDate::from_ymd_opt(1952, 3, 15).unwrap();
        let age = zapcre_compute_age(dob, today());
        assert_eq!(age, 74);
    }

    #[test]
    fn test_dob_age_birthday_not_yet() {
        // DOB 12/25/1952, today 04/10/2026 → age 73 (birthday hasn't passed)
        let dob = NaiveDate::from_ymd_opt(1952, 12, 25).unwrap();
        let age = zapcre_compute_age(dob, today());
        assert_eq!(age, 73);
    }

    #[test]
    fn test_dob_age_over_89() {
        let dicts = apcrd_Dictionaries::apcrd_load();
        let html = r#"<html><body>
            <b>Patient:</b> Ancient Elder<br>
            <b>DOB:</b> 01/01/1930<br>
            <b>MRN:</b> 99999999<br>
        </body></html>"#;
        let result = zapcre_analyze_with_date(html, &dicts, today());
        if let apcre_Result::Clinical { findings, .. } = result {
            let dob = findings.iter().find(|f| f.category == apcrm_PhiCategory::Dob);
            assert!(dob.is_some(), "DOB finding expected");
            assert_eq!(dob.unwrap().replacement, "Age: 90+");
        } else {
            panic!("Expected clinical result");
        }
    }

    #[test]
    fn test_dob_age_transform_in_fixture() {
        let dicts = apcrd_Dictionaries::apcrd_load();
        let result = zapcre_analyze_with_date(FIXTURE_PROGRESS, &dicts, today());
        if let apcre_Result::Clinical { findings, .. } = result {
            let dob = findings.iter().find(|f| f.category == apcrm_PhiCategory::Dob);
            assert!(dob.is_some(), "DOB finding expected in fixture");
            assert_eq!(dob.unwrap().replacement, "Age: 74");
        } else {
            panic!("Expected clinical result");
        }
    }

    // -----------------------------------------------------------------------
    // Merge semantics
    // -----------------------------------------------------------------------

    #[test]
    fn test_merge_red_wins_over_yellow() {
        let dicts = apcrd_Dictionaries::apcrd_load();
        // "Patient: Smith" — "Smith" is both anchored (RED) and in dictionary (YELLOW)
        let html = r#"<html><body>
            <b>Patient:</b> Smith<br>
            <b>DOB:</b> 01/01/1980<br>
            <b>MRN:</b> 12345678<br>
        </body></html>"#;
        let result = zapcre_analyze_with_date(html, &dicts, today());
        if let apcre_Result::Clinical { findings, .. } = result {
            let smith_findings: Vec<&apcrm_Finding> = findings.iter()
                .filter(|f| f.text.eq_ignore_ascii_case("Smith"))
                .collect();
            // At least one RED finding for "Smith"
            assert!(
                smith_findings.iter().any(|f| f.severity == apcrm_Severity::Red),
                "Smith should have RED finding (anchored)"
            );
        } else {
            panic!("Expected clinical result");
        }
    }

    // -----------------------------------------------------------------------
    // Full fixture integration
    // -----------------------------------------------------------------------

    #[test]
    fn test_fixture_catches_all_phi_categories() {
        let dicts = apcrd_Dictionaries::apcrd_load();
        let result = zapcre_analyze_with_date(FIXTURE_PROGRESS, &dicts, today());
        if let apcre_Result::Clinical { findings, .. } = result {
            let categories: std::collections::HashSet<apcrm_PhiCategory> =
                findings.iter().map(|f| f.category).collect();

            // Core PHI categories that must be detected in the fixture
            assert!(categories.contains(&apcrm_PhiCategory::Name),     "NAME missing");
            assert!(categories.contains(&apcrm_PhiCategory::Provider), "PROVIDER missing");
            assert!(categories.contains(&apcrm_PhiCategory::Facility), "FACILITY missing");
            assert!(categories.contains(&apcrm_PhiCategory::Dob),      "DOB missing");
            assert!(categories.contains(&apcrm_PhiCategory::Date),     "DATE missing");
            assert!(categories.contains(&apcrm_PhiCategory::Phone),    "PHONE missing");
            assert!(categories.contains(&apcrm_PhiCategory::Email),    "EMAIL missing");
            assert!(categories.contains(&apcrm_PhiCategory::Ssn),      "SSN missing");
            assert!(categories.contains(&apcrm_PhiCategory::Mrn),      "MRN missing");
            assert!(categories.contains(&apcrm_PhiCategory::Account),  "ACCOUNT missing");
            assert!(categories.contains(&apcrm_PhiCategory::Address),  "ADDRESS missing");
            assert!(categories.contains(&apcrm_PhiCategory::Url),      "URL missing");
        } else {
            panic!("Expected clinical result");
        }
    }

    #[test]
    fn test_fixture_catches_specific_names() {
        let dicts = apcrd_Dictionaries::apcrd_load();
        let result = zapcre_analyze_with_date(FIXTURE_PROGRESS, &dicts, today());
        if let apcre_Result::Clinical { findings, .. } = result {
            // Anchored names from labeled fields
            let texts: Vec<&str> = findings.iter().map(|f| f.text.as_str()).collect();
            assert!(texts.contains(&"Margaret"), "Margaret not found");
            assert!(texts.contains(&"Thornton"), "Thornton not found");
            assert!(texts.contains(&"Whitfield"), "Whitfield not found");
        } else {
            panic!("Expected clinical result");
        }
    }

    #[test]
    fn test_fixture_catches_ssn() {
        let dicts = apcrd_Dictionaries::apcrd_load();
        let result = zapcre_analyze_with_date(FIXTURE_PROGRESS, &dicts, today());
        if let apcre_Result::Clinical { findings, .. } = result {
            let ssn = findings.iter().find(|f| f.category == apcrm_PhiCategory::Ssn);
            assert!(ssn.is_some(), "SSN not found");
            assert_eq!(ssn.unwrap().text, "471-83-2956");
        } else {
            panic!("Expected clinical result");
        }
    }

    // -----------------------------------------------------------------------
    // Anonymize
    // -----------------------------------------------------------------------

    #[test]
    fn test_anonymize_elide_replaces_with_placeholder() {
        let plain = "Patient: John Smith, MRN: 12345678";
        let findings = vec![
            apcrm_Finding {
                text: "John".to_string(),
                replacement: "[NAME]".to_string(),
                severity: apcrm_Severity::Red,
                mechanism: apcrm_Mechanism::Anchored,
                category: apcrm_PhiCategory::Name,
                offset: 9,
                length: 4,
            },
            apcrm_Finding {
                text: "Smith".to_string(),
                replacement: "[NAME]".to_string(),
                severity: apcrm_Severity::Red,
                mechanism: apcrm_Mechanism::Anchored,
                category: apcrm_PhiCategory::Name,
                offset: 14,
                length: 5,
            },
        ];
        let toggles = vec!["elide".to_string(), "elide".to_string()];
        let result = apcre_anonymize(plain, &findings, &toggles);
        assert_eq!(result, "Patient: [NAME] [NAME], MRN: 12345678");
    }

    #[test]
    fn test_anonymize_pass_preserves_original() {
        let plain = "Patient: John Smith";
        let findings = vec![
            apcrm_Finding {
                text: "John".to_string(),
                replacement: "[NAME]".to_string(),
                severity: apcrm_Severity::Red,
                mechanism: apcrm_Mechanism::Anchored,
                category: apcrm_PhiCategory::Name,
                offset: 9,
                length: 4,
            },
            apcrm_Finding {
                text: "Smith".to_string(),
                replacement: "[NAME]".to_string(),
                severity: apcrm_Severity::Red,
                mechanism: apcrm_Mechanism::Anchored,
                category: apcrm_PhiCategory::Name,
                offset: 14,
                length: 5,
            },
        ];
        let toggles = vec!["elide".to_string(), "pass".to_string()];
        let result = apcre_anonymize(plain, &findings, &toggles);
        assert_eq!(result, "Patient: [NAME] Smith");
    }

    #[test]
    fn test_anonymize_dob_uses_age_replacement() {
        let plain = "DOB: 03/15/1952";
        let findings = vec![
            apcrm_Finding {
                text: "03/15/1952".to_string(),
                replacement: "Age: 74".to_string(),
                severity: apcrm_Severity::Red,
                mechanism: apcrm_Mechanism::Regex,
                category: apcrm_PhiCategory::Dob,
                offset: 5,
                length: 10,
            },
        ];
        let toggles = vec!["elide".to_string()];
        let result = apcre_anonymize(plain, &findings, &toggles);
        assert_eq!(result, "DOB: Age: 74");
    }

    #[test]
    fn test_anonymize_preserves_line_breaks() {
        let plain = "Line one\nPatient: John\nLine three";
        let findings = vec![
            apcrm_Finding {
                text: "John".to_string(),
                replacement: "[NAME]".to_string(),
                severity: apcrm_Severity::Red,
                mechanism: apcrm_Mechanism::Anchored,
                category: apcrm_PhiCategory::Name,
                offset: 18,
                length: 4,
            },
        ];
        let toggles = vec!["elide".to_string()];
        let result = apcre_anonymize(plain, &findings, &toggles);
        assert_eq!(result, "Line one\nPatient: [NAME]\nLine three");
    }

    #[test]
    fn test_anonymize_no_findings_returns_original() {
        let plain = "No PHI here at all.";
        let result = apcre_anonymize(plain, &[], &[]);
        assert_eq!(result, plain);
    }

    #[test]
    fn test_fixture_no_medical_terms_flagged_red() {
        let dicts = apcrd_Dictionaries::apcrd_load();
        let result = zapcre_analyze_with_date(FIXTURE_PROGRESS, &dicts, today());
        if let apcre_Result::Clinical { findings, .. } = result {
            // Medical terms should not appear as RED findings
            let medical_terms = [
                "hypertension", "diabetes", "metformin", "lisinopril",
                "atorvastatin", "troponin", "aspirin", "heparin",
            ];
            for term in &medical_terms {
                let red_match = findings.iter().find(|f| {
                    f.text.eq_ignore_ascii_case(term) && f.severity == apcrm_Severity::Red
                });
                assert!(
                    red_match.is_none(),
                    "Medical term '{}' should not be flagged RED",
                    term
                );
            }
        } else {
            panic!("Expected clinical result");
        }
    }
}
