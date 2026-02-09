// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Marker code registry - single source of truth for all JJ commit action codes
//!
//! This module defines all single-character codes used in JJ commit prefixes
//! and provides compile-time validation to prevent collisions.

/// Chalk Marker: Approach - proposed approach before work begins
pub const JJRNM_APPROACH: char = 'A';

/// Chalk Marker: Wrap - pace completion summary
pub const JJRNM_WRAP: char = 'W';

/// Chalk Marker: Fly - autonomous execution began (bridled pace)
pub const JJRNM_FLY: char = 'F';

/// Chalk Marker: Bridle - pace transitioned to bridled state
pub const JJRNM_BRIDLE: char = 'B';

/// Chalk Marker: Discussion - significant decision (lowercase, can be heat-level too)
pub const JJRNM_DISCUSSION: char = 'd';

/// Heat Action: Nominate - create new heat
pub const JJRNM_NOMINATE: char = 'N';

/// Heat Action: Slate - add new pace
pub const JJRNM_SLATE: char = 'S';

/// Heat Action: Rail - reorder paces (lowercase)
pub const JJRNM_RAIL: char = 'r';

/// Heat Action: Tally - add tack to pace
pub const JJRNM_TALLY: char = 'T';

/// Heat Action: Draft - move pace between heats (uppercase, rare)
pub const JJRNM_DRAFT: char = 'D';

/// Heat Action: Retire - archive heat (uppercase)
pub const JJRNM_RETIRE: char = 'R';

/// Heat Action: Furlough - change heat status or rename (lowercase)
pub const JJRNM_FURLOUGH: char = 'f';

/// Heat Action: Garland - celebrate completed heat, create continuation (uppercase)
pub const JJRNM_GARLAND: char = 'G';

/// Implicit Action: Notch - standard commit with heat/pace context
pub const JJRNM_NOTCH: char = 'n';

/// Implicit Action: Landing - agent completion
pub const JJRNM_LANDING: char = 'L';

/// Registry of all marker codes with their names
///
/// This array provides a central registry for validation and documentation.
/// Each entry is (code, name) where name describes the marker's purpose.
pub const fn jjrnm_all_codes() -> &'static [(char, &'static str)] {
    &[
        (JJRNM_APPROACH, "Approach"),
        (JJRNM_WRAP, "Wrap"),
        (JJRNM_FLY, "Fly"),
        (JJRNM_BRIDLE, "Bridle"),
        (JJRNM_DISCUSSION, "Discussion"),
        (JJRNM_NOMINATE, "Nominate"),
        (JJRNM_SLATE, "Slate"),
        (JJRNM_RAIL, "Rail"),
        (JJRNM_TALLY, "Tally"),
        (JJRNM_DRAFT, "Draft"),
        (JJRNM_RETIRE, "Retire"),
        (JJRNM_FURLOUGH, "Furlough"),
        (JJRNM_GARLAND, "Garland"),
        (JJRNM_NOTCH, "Notch"),
        (JJRNM_LANDING, "Landing"),
    ]
}
