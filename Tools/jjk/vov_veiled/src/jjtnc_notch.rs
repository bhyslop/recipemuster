// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use std::collections::HashSet;

use super::jjrnc_notch::{jjrnc_empty_notch_monitum, jjrnc_outside_list_warnings};

fn jjtnc_files_set<'a>(files: &'a [&'a str]) -> HashSet<&'a str> {
    files.iter().copied().collect()
}

// A staged rename whose BOTH endpoints are in the file list is covered — no warning.
#[test]
fn jjtnc_rename_both_sides_listed_no_warning() {
    let status = "R  old/path.rs -> new/path.rs\n";
    let files = jjtnc_files_set(&["old/path.rs", "new/path.rs"]);
    assert!(jjrnc_outside_list_warnings(status, &files).is_empty());
}

// A rename also modified in the worktree ("RM") still resolves by its index status.
#[test]
fn jjtnc_rename_modified_both_sides_listed_no_warning() {
    let status = "RM old/path.rs -> new/path.rs\n";
    let files = jjtnc_files_set(&["old/path.rs", "new/path.rs"]);
    assert!(jjrnc_outside_list_warnings(status, &files).is_empty());
}

// A copy line uses the same "old -> new" shape and the same both-sides rule.
#[test]
fn jjtnc_copy_both_sides_listed_no_warning() {
    let status = "C  src/orig.rs -> src/dup.rs\n";
    let files = jjtnc_files_set(&["src/orig.rs", "src/dup.rs"]);
    assert!(jjrnc_outside_list_warnings(status, &files).is_empty());
}

// A rename genuinely absent from the file list still warns.
#[test]
fn jjtnc_rename_neither_side_listed_warns() {
    let status = "R  old/path.rs -> new/path.rs\n";
    let files = jjtnc_files_set(&["unrelated.rs"]);
    let warnings = jjrnc_outside_list_warnings(status, &files);
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("old/path.rs -> new/path.rs"));
}

// The stricter rule: a rename with only one endpoint listed still warns.
#[test]
fn jjtnc_rename_only_new_side_listed_warns() {
    let status = "R  old/path.rs -> new/path.rs\n";
    let files = jjtnc_files_set(&["new/path.rs"]);
    assert_eq!(jjrnc_outside_list_warnings(status, &files).len(), 1);
}

#[test]
fn jjtnc_rename_only_old_side_listed_warns() {
    let status = "R  old/path.rs -> new/path.rs\n";
    let files = jjtnc_files_set(&["old/path.rs"]);
    assert_eq!(jjrnc_outside_list_warnings(status, &files).len(), 1);
}

// Non-rename entries keep their plain single-path coverage.
#[test]
fn jjtnc_modified_listed_no_warning() {
    let status = "M  tracked.rs\n";
    let files = jjtnc_files_set(&["tracked.rs"]);
    assert!(jjrnc_outside_list_warnings(status, &files).is_empty());
}

#[test]
fn jjtnc_modified_outside_list_warns() {
    let status = "M  stray.rs\n";
    let files = jjtnc_files_set(&["tracked.rs"]);
    assert_eq!(jjrnc_outside_list_warnings(status, &files).len(), 1);
}

// A clean rename pair must not be masked by an unrelated stray file in the same status.
#[test]
fn jjtnc_rename_covered_stray_still_warns() {
    let status = "R  old/path.rs -> new/path.rs\nM  stray.rs\n";
    let files = jjtnc_files_set(&["old/path.rs", "new/path.rs"]);
    let warnings = jjrnc_outside_list_warnings(status, &files);
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("stray.rs"));
}

// The empty-notch monitum is an advisory, never a refusal: it reports the landed commit.
#[test]
fn jjtnc_empty_notch_monitum_reports_the_landed_commit() {
    let monitum = jjrnc_empty_notch_monitum(&[]);
    assert!(monitum.starts_with("warning: "), "a monitum never gates — it warns: {}", monitum);
    assert!(monitum.contains("empty notch"));
    assert!(!monitum.contains("INTERDICTUM"), "an interdictum bars the act; this one landed");
}

// With no files listed, the monitum names the deliberate act — work that changed nothing on disk.
#[test]
fn jjtnc_empty_notch_monitum_no_files_names_the_deliberate_act() {
    let monitum = jjrnc_empty_notch_monitum(&[]);
    assert!(monitum.contains("No files were listed"));
    assert!(monitum.contains("changed nothing on disk"));
}

// With files listed and nothing staged, the monitum names the gap the caller did not expect.
#[test]
fn jjtnc_empty_notch_monitum_listed_files_name_the_gap() {
    let files = vec!["a.rs".to_string(), "b.rs".to_string()];
    let monitum = jjrnc_empty_notch_monitum(&files);
    assert!(monitum.contains("2 file(s) were listed"));
    assert!(monitum.contains("none held changes"));
    assert!(!monitum.contains("No files were listed"));
}
