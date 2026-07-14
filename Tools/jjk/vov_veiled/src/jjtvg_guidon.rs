// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for the guidon composer and its display-only read (`jjrvg_guidon`,
//! JJSVB `jjdb_guidon`).
//!
//! The contract under test is mostly about what the read must NOT do: it must
//! never refuse, because a guidon the mechanics accept as a lock is a lock, and
//! the cashier door must be able to break it whatever it says.

use crate::jjrvg_guidon::{
    jjdb_guidon_compose,
    jjdb_guidon_read,
    jjdb_is_probably_live,
    jjdb_render_age,
    JJDB_LIVENESS_WARN_SECONDS,
};
use chrono::{
    Duration,
    TimeZone,
    Utc,
};

fn zjjtvg_when() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 7, 14, 9, 30, 0).unwrap()
}

#[test]
fn jjtvg_compose_writes_the_four_fields_in_order() {
    let guidon = jjdb_guidon_compose("☉260714-1006", "beast", zjjtvg_when(), "journal");
    assert_eq!(
        guidon,
        "officium=☉260714-1006 station=beast acquired=2026-07-14T09:30:00Z operation=journal"
    );
}

#[test]
fn jjtvg_a_composed_guidon_reads_back_whole() {
    let when = zjjtvg_when();
    let read = jjdb_guidon_read(&jjdb_guidon_compose("☉260714-1006", "beast", when, "journal"));
    assert_eq!(read.officium.as_deref(), Some("☉260714-1006"));
    assert_eq!(read.station.as_deref(), Some("beast"));
    assert_eq!(read.acquired, Some(when));
    assert_eq!(read.operation.as_deref(), Some("journal"));
    assert!(read.jjdb_is_well_formed());
}

/// A value with interior whitespace would break the field-wise line, so the
/// composer squeezes it rather than fail: no composition of a mark may refuse,
/// or a station could be unable to take a lock at all.
#[test]
fn jjtvg_compose_squeezes_whitespace_and_never_refuses() {
    let read = jjdb_guidon_read(&jjdb_guidon_compose("of ficium", "", zjjtvg_when(), "two words"));
    assert_eq!(read.officium.as_deref(), Some("of_ficium"));
    assert_eq!(read.station.as_deref(), Some("-"), "an empty value becomes a placeholder, never nothing");
    assert_eq!(read.operation.as_deref(), Some("two_words"));
    assert!(read.jjdb_is_well_formed(), "a squeezed mark is still a well-formed one");
}

/// The load-bearing tolerance: a mark this engine did not compose is STILL a
/// lock (the mechanics compare blob hashes, never text), so the read must yield
/// something renderable rather than refuse — and the verbatim mark, which is
/// what a break plucks against, must survive untouched.
#[test]
fn jjtvg_a_foreign_mark_reads_tolerantly_and_keeps_its_verbatim_text() {
    let foreign = "who knows what this is";
    let read = jjdb_guidon_read(foreign);
    assert_eq!(read.verbatim, foreign, "the break plucks against this, so it must survive verbatim");
    assert!(!read.jjdb_is_well_formed());
    assert_eq!(read.officium, None);
    assert_eq!(read.acquired, None);
}

/// A partial mark — an older engine's, or a truncated one — yields what it can
/// and nothing more.
#[test]
fn jjtvg_a_partial_mark_yields_the_fields_it_has() {
    let read = jjdb_guidon_read("station=beast operation=journal acquired=not-a-timestamp");
    assert_eq!(read.station.as_deref(), Some("beast"));
    assert_eq!(read.operation.as_deref(), Some("journal"));
    assert_eq!(read.acquired, None, "an unparseable time is absent, never a guessed one");
    assert!(!read.jjdb_is_well_formed());
}

#[test]
fn jjtvg_age_renders_in_the_grain_the_operator_judges_on() {
    let now = zjjtvg_when();
    assert_eq!(jjdb_render_age(now, now), "just now");
    assert_eq!(jjdb_render_age(now - Duration::seconds(30), now), "30s ago");
    assert_eq!(jjdb_render_age(now - Duration::minutes(5), now), "5m ago");
    assert_eq!(jjdb_render_age(now - Duration::minutes(90), now), "1h 30m ago");
    assert_eq!(jjdb_render_age(now - Duration::hours(50), now), "2d 2h ago");
}

/// A skewed station's clock must not render a negative age at the moment an
/// operator is deciding whether to break a lock.
#[test]
fn jjtvg_a_mark_from_the_future_reads_as_just_now() {
    let now = zjjtvg_when();
    assert_eq!(jjdb_render_age(now + Duration::hours(3), now), "just now");
}

/// The liveness warning: a lock younger than the window is probably a LIVE
/// writer. The door warns and never blocks on it.
#[test]
fn jjtvg_a_young_lock_is_probably_live_and_an_old_one_is_not() {
    let now = zjjtvg_when();
    let young = jjdb_guidon_read(&jjdb_guidon_compose("o", "s", now - Duration::seconds(5), "journal"));
    let old = jjdb_guidon_read(&jjdb_guidon_compose(
        "o",
        "s",
        now - Duration::seconds(JJDB_LIVENESS_WARN_SECONDS + 1),
        "journal",
    ));
    assert!(jjdb_is_probably_live(&young, now));
    assert!(!jjdb_is_probably_live(&old, now));
}

/// A mark with no readable acquire time cannot be judged young: the warning is
/// advice, and advice invented from nothing is noise.
#[test]
fn jjtvg_an_unreadable_mark_earns_no_liveness_warning() {
    let read = jjdb_guidon_read("something else entirely");
    assert!(!jjdb_is_probably_live(&read, zjjtvg_when()));
}
