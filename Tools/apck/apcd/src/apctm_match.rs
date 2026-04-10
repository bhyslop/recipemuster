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

//! Tests for dictionary and regex matching.

#[cfg(test)]
mod tests {
    use crate::apcrd_dictionaries::apcrd_Dictionaries;
    use crate::apcrm_match::*;
    use crate::apcrp_parse::{apcrp_Span, apcrp_SpanKind};

    // -----------------------------------------------------------------------
    // Tier 1 — Regex
    // -----------------------------------------------------------------------

    #[test]
    fn test_regex_ssn() {
        let findings = apcrm_scan_regex("SSN is 471-83-2956 on file");
        let ssn = findings.iter().find(|f| f.category == apcrm_PhiCategory::Ssn);
        assert!(ssn.is_some(), "SSN not detected");
        assert_eq!(ssn.unwrap().text, "471-83-2956");
    }

    #[test]
    fn test_regex_phone_parens() {
        let findings = apcrm_scan_regex("call (207) 555-0143 now");
        let phone = findings.iter().find(|f| f.category == apcrm_PhiCategory::Phone);
        assert!(phone.is_some(), "Phone not detected");
        assert_eq!(phone.unwrap().text, "(207) 555-0143");
    }

    #[test]
    fn test_regex_phone_dashes() {
        let findings = apcrm_scan_regex("fax 207-555-0199 today");
        let phone = findings.iter().find(|f| f.category == apcrm_PhiCategory::Phone);
        assert!(phone.is_some(), "Phone not detected");
        assert_eq!(phone.unwrap().text, "207-555-0199");
    }

    #[test]
    fn test_regex_email() {
        let findings = apcrm_scan_regex("email m.thornton47@gmail.com for info");
        let email = findings.iter().find(|f| f.category == apcrm_PhiCategory::Email);
        assert!(email.is_some(), "Email not detected");
        assert_eq!(email.unwrap().text, "m.thornton47@gmail.com");
    }

    #[test]
    fn test_regex_date() {
        let findings = apcrm_scan_regex("DOB: 03/15/1952 next");
        let date = findings.iter().find(|f| f.category == apcrm_PhiCategory::Date);
        assert!(date.is_some(), "Date not detected");
        assert_eq!(date.unwrap().text, "03/15/1952");
    }

    #[test]
    fn test_regex_address() {
        let findings = apcrm_scan_regex("lives at 1847 Cranberry Lane nearby");
        let addr = findings.iter().find(|f| f.category == apcrm_PhiCategory::Address
            && f.text.contains("Cranberry"));
        assert!(addr.is_some(), "Address not detected");
    }

    #[test]
    fn test_regex_mrn_labeled() {
        let findings = apcrm_scan_regex("MRN: 00847293\nnext field");
        let mrn = findings.iter().find(|f| f.category == apcrm_PhiCategory::Mrn);
        assert!(mrn.is_some(), "MRN not detected");
        assert_eq!(mrn.unwrap().text, "00847293");
    }

    #[test]
    fn test_regex_account_labeled() {
        let findings = apcrm_scan_regex("Account: 3391057842\nnext");
        let acct = findings.iter().find(|f| f.category == apcrm_PhiCategory::Account);
        assert!(acct.is_some(), "Account not detected");
        assert_eq!(acct.unwrap().text, "3391057842");
    }

    #[test]
    fn test_regex_encounter_id() {
        let findings = apcrm_scan_regex("Encounter ID: ENC-2026-0048817\nnext");
        let enc = findings.iter().find(|f| f.category == apcrm_PhiCategory::EncounterId);
        assert!(enc.is_some(), "Encounter ID not detected");
        assert_eq!(enc.unwrap().text, "ENC-2026-0048817");
    }

    #[test]
    fn test_regex_url() {
        let findings = apcrm_scan_regex("portal https://mychart.mainehealth.org/patient/thornton-m end");
        let url = findings.iter().find(|f| f.category == apcrm_PhiCategory::Url);
        assert!(url.is_some(), "URL not detected");
    }

    #[test]
    fn test_regex_ip_address() {
        let findings = apcrm_scan_regex("IP: 10.142.38.91 logged");
        let ip = findings.iter().find(|f| f.category == apcrm_PhiCategory::IpAddress);
        assert!(ip.is_some(), "IP address not detected");
        assert_eq!(ip.unwrap().text, "10.142.38.91");
    }

    #[test]
    fn test_regex_npi() {
        let findings = apcrm_scan_regex("NPI: 1234567890\nnext");
        let npi = findings.iter().find(|f| f.category == apcrm_PhiCategory::Npi);
        assert!(npi.is_some(), "NPI not detected");
        assert_eq!(npi.unwrap().text, "1234567890");
    }

    #[test]
    fn test_regex_dea() {
        let findings = apcrm_scan_regex("DEA: AW1234567\nnext");
        let dea = findings.iter().find(|f| f.category == apcrm_PhiCategory::Dea);
        assert!(dea.is_some(), "DEA not detected");
        assert_eq!(dea.unwrap().text, "AW1234567");
    }

    // -----------------------------------------------------------------------
    // Tier 2 — Anchored
    // -----------------------------------------------------------------------

    #[test]
    fn test_anchored_patient_name() {
        let spans = vec![apcrp_Span {
            kind:   apcrp_SpanKind::LabeledField,
            label:  Some("Patient".to_string()),
            text:   "Margaret J. Thornton".to_string(),
            offset: 0,
        }];
        let findings = apcrm_scan_anchored(&spans);
        let names: Vec<&str> = findings.iter().map(|f| f.text.as_str()).collect();
        assert!(names.contains(&"Margaret"), "Margaret not anchored");
        assert!(names.contains(&"J"), "J not anchored");
        assert!(names.contains(&"Thornton"), "Thornton not anchored");
        assert!(findings.iter().all(|f| f.severity == apcrm_Severity::Red));
        assert!(findings.iter().all(|f| f.mechanism == apcrm_Mechanism::Anchored));
        assert!(findings.iter().all(|f| f.category == apcrm_PhiCategory::Name));
    }

    #[test]
    fn test_anchored_provider() {
        let spans = vec![apcrp_Span {
            kind:   apcrp_SpanKind::LabeledField,
            label:  Some("Attending".to_string()),
            text:   "James R. Whitfield, MD".to_string(),
            offset: 0,
        }];
        let findings = apcrm_scan_anchored(&spans);
        let names: Vec<&str> = findings.iter().map(|f| f.text.as_str()).collect();
        assert!(names.contains(&"James"), "James not anchored");
        assert!(names.contains(&"Whitfield"), "Whitfield not anchored");
        assert!(names.contains(&"MD"), "MD not anchored");
        assert!(findings.iter().all(|f| f.category == apcrm_PhiCategory::Provider));
    }

    #[test]
    fn test_anchored_facility() {
        let spans = vec![apcrp_Span {
            kind:   apcrp_SpanKind::LabeledField,
            label:  Some("Facility".to_string()),
            text:   "Maine Medical Center".to_string(),
            offset: 0,
        }];
        let findings = apcrm_scan_anchored(&spans);
        assert!(findings.iter().all(|f| f.category == apcrm_PhiCategory::Facility));
        assert_eq!(findings.len(), 3); // Maine, Medical, Center
    }

    #[test]
    fn test_anchored_skips_non_anchor_labels() {
        let spans = vec![apcrp_Span {
            kind:   apcrp_SpanKind::LabeledField,
            label:  Some("MRN".to_string()),
            text:   "00847293".to_string(),
            offset: 0,
        }];
        let findings = apcrm_scan_anchored(&spans);
        assert!(findings.is_empty(), "MRN should not be anchored");
    }

    #[test]
    fn test_anchored_case_insensitive_label() {
        let spans = vec![apcrp_Span {
            kind:   apcrp_SpanKind::LabeledField,
            label:  Some("PATIENT".to_string()),
            text:   "Jane Doe".to_string(),
            offset: 0,
        }];
        let findings = apcrm_scan_anchored(&spans);
        assert_eq!(findings.len(), 2); // Jane, Doe
    }

    // -----------------------------------------------------------------------
    // Tier 3 — Dictionary
    // -----------------------------------------------------------------------

    #[test]
    fn test_dictionary_finds_surname_in_narrative() {
        let dicts = apcrd_Dictionaries::apcrd_load();
        let text = "The patient's husband Robert Thornton called EMS.";
        let findings = apcrm_scan_dictionary(text, &dicts);
        // "thornton" should be in the surnames dictionary
        let has_thornton = findings.iter().any(|f| f.text.eq_ignore_ascii_case("Thornton"));
        assert!(has_thornton, "Thornton not found in dictionary scan");
        assert!(findings.iter().all(|f| f.severity == apcrm_Severity::Yellow));
        assert!(findings.iter().all(|f| f.mechanism == apcrm_Mechanism::Matched));
    }

    #[test]
    fn test_dictionary_respects_word_boundaries() {
        let dicts = apcrd_Dictionaries::apcrd_load();
        // "park" might be in firstnames; "parking" should NOT match "park"
        let text = "parking lot nearby";
        let findings = apcrm_scan_dictionary(text, &dicts);
        let has_park = findings.iter().any(|f| f.text == "park");
        assert!(!has_park, "Substring 'park' should not match within 'parking'");
    }

    #[test]
    fn test_dictionary_preserves_original_case() {
        let dicts = apcrd_Dictionaries::apcrd_load();
        let text = "consulted with JOHNSON about treatment";
        let findings = apcrm_scan_dictionary(text, &dicts);
        let johnson = findings.iter().find(|f| f.text.eq_ignore_ascii_case("johnson"));
        assert!(johnson.is_some(), "JOHNSON not found");
        assert_eq!(johnson.unwrap().text, "JOHNSON"); // original case preserved
    }
}
