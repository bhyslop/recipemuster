// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Guidon — the lock-holder mark flown on a blotter's lock ref (`jjdb_guidon`,
//! JJSVB-blotter.adoc): its composition, and the tolerant read the cashier door
//! renders it with.
//!
//! The composed form is `key=value` fields, space-separated, in the fixed order
//! officium, station, acquire time (RFC 3339, UTC), operation. This module is
//! that format's single home — no other site spells it.
//!
//! The lock primitives never parse a guidon: `stake`, `pluck`, and `consign`'s
//! lease compare by blob-hash equality alone. So the parse below is *for display
//! alone*, and it is tolerant by contract: a guidon this engine did not compose —
//! an older engine's, a hand-staked probe's, a truncated one — is still a lock,
//! and must stay sightable and breakable. An unparseable field renders as
//! itself; nothing here can refuse.

use chrono::{
    DateTime,
    Utc,
};

/// The field keys, in composed order. The composer writes them; the tolerant
/// read below recognizes them and is unbothered by anything else.
const ZJJRVG_KEY_OFFICIUM: &str = "officium";
const ZJJRVG_KEY_STATION: &str = "station";
const ZJJRVG_KEY_ACQUIRED: &str = "acquired";
const ZJJRVG_KEY_OPERATION: &str = "operation";

/// Compose a guidon (`jjdb_guidon`): the four fields, in order, as one line.
///
/// `acquired` is passed rather than read from the clock, so a caller's mark is a
/// pure function of what it declares — the ceremony's own acquire moment, not
/// this function's. Values carry no spaces (the line reads back field-wise), so
/// any interior whitespace in a caller's value is squeezed to a single `_`
/// rather than silently breaking the composed shape: a mark is a mark, and no
/// composition of it may fail.
pub fn jjdb_guidon_compose(officium: &str, station: &str, acquired: DateTime<Utc>, operation: &str) -> String {
    format!(
        "{}={} {}={} {}={} {}={}",
        ZJJRVG_KEY_OFFICIUM,
        zjjrvg_scrub(officium),
        ZJJRVG_KEY_STATION,
        zjjrvg_scrub(station),
        ZJJRVG_KEY_ACQUIRED,
        acquired.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        ZJJRVG_KEY_OPERATION,
        zjjrvg_scrub(operation),
    )
}

/// Squeeze whitespace out of one field value. A value with no spaces is what
/// makes the composed line readable field-wise; an empty value becomes `-` so a
/// field never vanishes from the line entirely.
fn zjjrvg_scrub(value: &str) -> String {
    let scrubbed: String = value
        .chars()
        .map(|c| if c.is_whitespace() { '_' } else { c })
        .collect();
    if scrubbed.is_empty() {
        "-".to_string()
    } else {
        scrubbed
    }
}

/// This station's name — the one guidon field the ceremony cannot supply from
/// its own context. A station is a machine, and the machine's hostname is what
/// the operator will recognize when the door names the victim.
///
/// Total by construction: a station that cannot name itself still gets a mark
/// (`unknown`). Refusing here would make an unnamed station unable to take a
/// lock at all — a far worse failure than a vague guidon, and one the operator
/// meets as a broken ceremony rather than a legible one.
pub fn jjdb_station_name() -> String {
    std::process::Command::new("hostname")
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

/// A guidon read for display (`jjdb_guidon` — "parsed tolerantly for display
/// alone, never for mechanics"). Every field is optional: what this engine
/// composed reads back whole, and what it did not still yields whatever it can.
/// `verbatim` is the mark exactly as sighted, and it — never these fields — is
/// what a break plucks against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjdb_GuidonRead {
    pub verbatim: String,
    pub officium: Option<String>,
    pub station: Option<String>,
    pub acquired: Option<DateTime<Utc>>,
    pub operation: Option<String>,
}

impl jjdb_GuidonRead {
    /// Whether the mark parsed as this engine composes them. A `false` here is
    /// never a reason to refuse a break — it is a reason to render the mark
    /// verbatim and say so.
    pub fn jjdb_is_well_formed(&self) -> bool {
        self.officium.is_some() && self.station.is_some() && self.acquired.is_some() && self.operation.is_some()
    }
}

/// Read a sighted guidon for display. Never fails: an unrecognized shape yields
/// a read whose fields are all `None` and whose `verbatim` is the whole mark.
pub fn jjdb_guidon_read(verbatim: &str) -> jjdb_GuidonRead {
    let mut read = jjdb_GuidonRead {
        verbatim: verbatim.to_string(),
        officium: None,
        station: None,
        acquired: None,
        operation: None,
    };

    for field in verbatim.split_whitespace() {
        let Some((key, value)) = field.split_once('=') else { continue };
        match key {
            ZJJRVG_KEY_OFFICIUM => read.officium = Some(value.to_string()),
            ZJJRVG_KEY_STATION => read.station = Some(value.to_string()),
            ZJJRVG_KEY_OPERATION => read.operation = Some(value.to_string()),
            ZJJRVG_KEY_ACQUIRED => {
                read.acquired = DateTime::parse_from_rfc3339(value).ok().map(|when| when.with_timezone(&Utc));
            }
            _ => {}
        }
    }

    read
}

/// Render an acquire time as an AGE against `now` — the form the cashier door's
/// gate must show, because staleness is what the operator actually judges and a
/// bare timestamp makes them subtract in their head (JJSVD `jjdd_cashier`).
/// A mark from the future (a station with a skewed clock) reads as `just now`
/// rather than a negative age: the skew is not the operator's problem at the
/// moment they are deciding whether to break a lock.
pub fn jjdb_render_age(acquired: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let seconds = now.signed_duration_since(acquired).num_seconds();
    if seconds < 5 {
        return "just now".to_string();
    }
    if seconds < 60 {
        return format!("{}s ago", seconds);
    }
    let minutes = seconds / 60;
    if minutes < 60 {
        return format!("{}m ago", minutes);
    }
    let hours = minutes / 60;
    if hours < 24 {
        return format!("{}h {}m ago", hours, minutes % 60);
    }
    format!("{}d {}h ago", hours / 24, hours % 24)
}

/// The age below which a lock is probably a LIVE writer, not a corpse: a
/// ceremony runs in seconds (JJSVD `jjdd_cashier`, the liveness warning). The
/// door WARNS on a younger lock and never blocks — a human may know the station
/// is unplugged, and a door that refused would be un-runnable in the case it
/// exists for.
pub const JJDB_LIVENESS_WARN_SECONDS: i64 = 60;

/// Whether a sighted mark is young enough to warrant the liveness warning. A
/// mark with no readable acquire time cannot be judged young — the warning is
/// advice, and advice invented from nothing is noise.
pub fn jjdb_is_probably_live(read: &jjdb_GuidonRead, now: DateTime<Utc>) -> bool {
    match read.acquired {
        Some(acquired) => now.signed_duration_since(acquired).num_seconds() < JJDB_LIVENESS_WARN_SECONDS,
        None => false,
    }
}
