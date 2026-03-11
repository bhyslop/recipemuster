// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Marker code registry - single source of truth for all JJ commit action codes
//!
//! This module defines all single-character codes used in JJ commit prefixes
//! and provides compile-time validation to prevent collisions.

/// Chalk Marker: Wrap - pace completion summary
pub const JJRNM_WRAP: char = 'W';

/// Chalk Marker: Discussion - significant decision (lowercase, can be heat-level too)
pub const JJRNM_DISCUSSION: char = 'd';

/// Heat Action: Nominate - create new heat
pub const JJRNM_NOMINATE: char = 'N';

/// Heat Action: Slate - add new pace
pub const JJRNM_SLATE: char = 'S';

/// Heat Action: Rail - reorder paces (lowercase)
pub const JJRNM_RAIL: char = 'r';

/// Heat Action: Tally - modify pace fields (docket, silks, state)
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
        (JJRNM_WRAP, "Wrap"),
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
