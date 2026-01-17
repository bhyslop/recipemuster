// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrn_notch::*;
use super::jjrf_favor::{jjrf_Coronet as Coronet, jjrf_Firemark as Firemark};

// ChalkMarker tests

#[test]
fn jjtn_chalk_marker_parse_single_letter() {
    assert_eq!(jjrn_ChalkMarker::jjrn_parse("A").unwrap(), jjrn_ChalkMarker::Approach);
    assert_eq!(jjrn_ChalkMarker::jjrn_parse("W").unwrap(), jjrn_ChalkMarker::Wrap);
    assert_eq!(jjrn_ChalkMarker::jjrn_parse("F").unwrap(), jjrn_ChalkMarker::Fly);
    assert_eq!(jjrn_ChalkMarker::jjrn_parse("d").unwrap(), jjrn_ChalkMarker::Discussion);
}

#[test]
fn jjtn_chalk_marker_parse_full_word() {
    assert_eq!(jjrn_ChalkMarker::jjrn_parse("APPROACH").unwrap(), jjrn_ChalkMarker::Approach);
    assert_eq!(jjrn_ChalkMarker::jjrn_parse("approach").unwrap(), jjrn_ChalkMarker::Approach);
    assert_eq!(jjrn_ChalkMarker::jjrn_parse("Wrap").unwrap(), jjrn_ChalkMarker::Wrap);
    assert_eq!(jjrn_ChalkMarker::jjrn_parse("FLY").unwrap(), jjrn_ChalkMarker::Fly);
    assert_eq!(jjrn_ChalkMarker::jjrn_parse("discussion").unwrap(), jjrn_ChalkMarker::Discussion);
    assert!(jjrn_ChalkMarker::jjrn_parse("invalid").is_err());
}

#[test]
fn jjtn_chalk_marker_code() {
    assert_eq!(jjrn_ChalkMarker::Approach.jjrn_code(), 'A');
    assert_eq!(jjrn_ChalkMarker::Wrap.jjrn_code(), 'W');
    assert_eq!(jjrn_ChalkMarker::Fly.jjrn_code(), 'F');
    assert_eq!(jjrn_ChalkMarker::Discussion.jjrn_code(), 'd');
}

#[test]
fn jjtn_chalk_marker_as_str() {
    assert_eq!(jjrn_ChalkMarker::Approach.jjrn_as_str(), "APPROACH");
    assert_eq!(jjrn_ChalkMarker::Wrap.jjrn_as_str(), "WRAP");
    assert_eq!(jjrn_ChalkMarker::Fly.jjrn_as_str(), "FLY");
    assert_eq!(jjrn_ChalkMarker::Discussion.jjrn_as_str(), "discussion");
}

#[test]
fn jjtn_chalk_marker_requires_pace() {
    assert!(jjrn_ChalkMarker::Approach.jjrn_requires_pace());
    assert!(jjrn_ChalkMarker::Wrap.jjrn_requires_pace());
    assert!(jjrn_ChalkMarker::Fly.jjrn_requires_pace());
    assert!(!jjrn_ChalkMarker::Discussion.jjrn_requires_pace());
}

// HeatAction tests

#[test]
fn jjtn_heat_action_code() {
    assert_eq!(jjrn_HeatAction::Nominate.jjrn_code(), 'N');
    assert_eq!(jjrn_HeatAction::Slate.jjrn_code(), 'S');
    assert_eq!(jjrn_HeatAction::Rail.jjrn_code(), 'r');
    assert_eq!(jjrn_HeatAction::Tally.jjrn_code(), 'T');
    assert_eq!(jjrn_HeatAction::Draft.jjrn_code(), 'D');
    assert_eq!(jjrn_HeatAction::Retire.jjrn_code(), 'R');
}

#[test]
fn jjtn_heat_action_as_str() {
    assert_eq!(jjrn_HeatAction::Nominate.jjrn_as_str(), "nominate");
    assert_eq!(jjrn_HeatAction::Slate.jjrn_as_str(), "slate");
    assert_eq!(jjrn_HeatAction::Rail.jjrn_as_str(), "rail");
    assert_eq!(jjrn_HeatAction::Tally.jjrn_as_str(), "tally");
    assert_eq!(jjrn_HeatAction::Draft.jjrn_as_str(), "draft");
    assert_eq!(jjrn_HeatAction::Retire.jjrn_as_str(), "retire");
}

// Format function tests

#[test]
fn jjtn_format_notch_prefix() {
    let coronet = Coronet::jjrf_parse("ABAAA").unwrap();
    let prefix = jjrn_format_notch_prefix(&coronet);
    assert_eq!(prefix, "jjb:RBM:₢ABAAA:n: ");
}

#[test]
fn jjtn_format_chalk_message_approach() {
    let coronet = Coronet::jjrf_parse("ABAAA").unwrap();
    let msg = jjrn_format_chalk_message(&coronet, jjrn_ChalkMarker::Approach, "Starting work on feature");
    assert_eq!(msg, "jjb:RBM:₢ABAAA:A: Starting work on feature");
}

#[test]
fn jjtn_format_chalk_message_wrap() {
    let coronet = Coronet::jjrf_parse("__AAA").unwrap();
    let msg = jjrn_format_chalk_message(&coronet, jjrn_ChalkMarker::Wrap, "Completed the task");
    assert_eq!(msg, "jjb:RBM:₢__AAA:W: Completed the task");
}

#[test]
fn jjtn_format_chalk_message_fly() {
    let coronet = Coronet::jjrf_parse("ABCDE").unwrap();
    let msg = jjrn_format_chalk_message(&coronet, jjrn_ChalkMarker::Fly, "Autonomous execution");
    assert_eq!(msg, "jjb:RBM:₢ABCDE:F: Autonomous execution");
}

#[test]
fn jjtn_format_chalk_message_discussion() {
    let coronet = Coronet::jjrf_parse("ABAAA").unwrap();
    let msg = jjrn_format_chalk_message(&coronet, jjrn_ChalkMarker::Discussion, "Design discussion");
    assert_eq!(msg, "jjb:RBM:₢ABAAA:d: Design discussion");
}

#[test]
fn jjtn_format_heat_discussion() {
    let fm = Firemark::jjrf_parse("AB").unwrap();
    let msg = jjrn_format_heat_discussion(&fm, "Design discussion without pace");
    assert_eq!(msg, "jjb:RBM:₣AB:d: Design discussion without pace");
}

#[test]
fn jjtn_format_heat_message_nominate() {
    let fm = Firemark::jjrf_parse("AB").unwrap();
    let msg = jjrn_format_heat_message(&fm, jjrn_HeatAction::Nominate, "my-new-heat");
    assert_eq!(msg, "jjb:RBM:₣AB:N: my-new-heat");
}

#[test]
fn jjtn_format_heat_message_slate() {
    let fm = Firemark::jjrf_parse("AB").unwrap();
    let msg = jjrn_format_heat_message(&fm, jjrn_HeatAction::Slate, "new-pace-silks");
    assert_eq!(msg, "jjb:RBM:₣AB:S: new-pace-silks");
}

#[test]
fn jjtn_format_heat_message_rail() {
    let fm = Firemark::jjrf_parse("AB").unwrap();
    let msg = jjrn_format_heat_message(&fm, jjrn_HeatAction::Rail, "reordered");
    assert_eq!(msg, "jjb:RBM:₣AB:r: reordered");
}

#[test]
fn jjtn_format_heat_message_tally() {
    let fm = Firemark::jjrf_parse("AB").unwrap();
    let msg = jjrn_format_heat_message(&fm, jjrn_HeatAction::Tally, "pace-name");
    assert_eq!(msg, "jjb:RBM:₣AB:T: pace-name");
}

#[test]
fn jjtn_format_heat_message_draft() {
    let fm = Firemark::jjrf_parse("CD").unwrap();
    let msg = jjrn_format_heat_message(&fm, jjrn_HeatAction::Draft, "₢ABAAA → ₣CD");
    assert_eq!(msg, "jjb:RBM:₣CD:D: ₢ABAAA → ₣CD");
}

#[test]
fn jjtn_format_heat_message_retire() {
    let fm = Firemark::jjrf_parse("AB").unwrap();
    let msg = jjrn_format_heat_message(&fm, jjrn_HeatAction::Retire, "my-heat-silks");
    assert_eq!(msg, "jjb:RBM:₣AB:R: my-heat-silks");
}
