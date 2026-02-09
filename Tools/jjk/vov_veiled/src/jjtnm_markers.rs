// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for jjrnm_markers - validate marker code registry

use crate::jjrnm_markers::*;
use std::collections::HashSet;

/// Test that all marker codes are unique (no collisions)
#[test]
fn jjtnm_test_no_collisions() {
    let all_codes = jjrnm_all_codes();
    let mut seen: HashSet<char> = HashSet::new();
    let mut collisions: Vec<(char, Vec<&str>)> = Vec::new();

    // Build map of codes to names
    for &(code, _name) in all_codes {
        if !seen.insert(code) {
            // Collision detected - find all names using this code
            let names: Vec<&str> = all_codes
                .iter()
                .filter_map(|&(c, n)| if c == code { Some(n) } else { None })
                .collect();
            collisions.push((code, names));
        }
    }

    if !collisions.is_empty() {
        let mut msg = String::from("Marker code collisions detected:\n");
        for (code, names) in collisions {
            msg.push_str(&format!("  '{}' used by: {}\n", code, names.join(", ")));
        }
        panic!("{}", msg);
    }
}

/// Test that all marker codes are printable ASCII
#[test]
fn jjtnm_test_printable_ascii() {
    let all_codes = jjrnm_all_codes();
    let mut invalid: Vec<(char, &str)> = Vec::new();

    for &(code, name) in all_codes {
        if !code.is_ascii() || !code.is_ascii_graphic() {
            invalid.push((code, name));
        }
    }

    if !invalid.is_empty() {
        let mut msg = String::from("Non-printable ASCII codes detected:\n");
        for (code, name) in invalid {
            msg.push_str(&format!("  {} ('{}'): U+{:04X}\n", name, code, code as u32));
        }
        panic!("{}", msg);
    }
}

/// Test that case-distinct codes exist for different purposes
/// (This is a documentation test, not a hard requirement)
#[test]
fn jjtnm_test_case_awareness() {
    let all_codes = jjrnm_all_codes();

    // Group codes by their uppercase form
    let mut uppercase_groups: std::collections::HashMap<char, Vec<(char, &str)>> =
        std::collections::HashMap::new();

    for &(code, name) in all_codes {
        let upper = code.to_ascii_uppercase();
        uppercase_groups.entry(upper).or_insert_with(Vec::new).push((code, name));
    }

    // Find groups with both uppercase and lowercase variants
    let mut case_pairs: Vec<(char, Vec<(char, &str)>)> = uppercase_groups
        .into_iter()
        .filter(|(_, variants)| variants.len() > 1)
        .collect();

    case_pairs.sort_by_key(|(upper, _)| *upper);

    // This is informational - document case pairs but don't fail
    if !case_pairs.is_empty() {
        println!("\nCase-distinct marker pairs detected:");
        for (upper, variants) in case_pairs {
            println!("  '{}' family:", upper);
            for (code, name) in variants {
                println!("    '{}' = {}", code, name);
            }
        }
    }
}
