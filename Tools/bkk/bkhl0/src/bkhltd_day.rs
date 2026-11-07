// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: Apache-2.0
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

//! Hurdles for the UTC calendar day: its spelling, its refusals, and the day a
//! moment falls in.

use crate::bkhlrd_day::bkhlrd_Day;
use std::time::{
    Duration,
    UNIX_EPOCH, //
};

#[test]
fn bkhltd_the_epoch_is_day_zero_and_spells_itself() {
    let epoch = bkhlrd_Day::bkhlrd_parse("19700101").expect("the epoch parses");
    assert_eq!(epoch.bkhlrd_count(), 0);
    assert_eq!(epoch.bkhlrd_digits(), "19700101");
}

#[test]
fn bkhltd_every_day_across_four_centuries_round_trips() {
    let start = bkhlrd_Day::bkhlrd_parse("18991231").expect("a day parses");
    for step in 0..(400 * 366) {
        let day = bkhlrd_Day::bkhlrd_from_count(start.bkhlrd_count() + step);
        let again = bkhlrd_Day::bkhlrd_parse(&day.bkhlrd_digits()).expect("its spelling parses");
        assert_eq!(again, day);
    }
}

#[test]
fn bkhltd_leap_days_stand_where_the_calendar_holds_them() {
    assert!(bkhlrd_Day::bkhlrd_parse("20240229").is_ok());
    assert!(bkhlrd_Day::bkhlrd_parse("20000229").is_ok());
    assert!(bkhlrd_Day::bkhlrd_parse("21000229").is_err());
    assert!(bkhlrd_Day::bkhlrd_parse("20260229").is_err());
}

#[test]
fn bkhltd_any_other_spelling_is_refused() {
    for spelled in [
        "2026131",
        "202601311",
        "2026-1-31",
        "abcdefgh",
        "20261301",
        "20260100",
        "20260431",
        "",
    ] {
        assert!(
            bkhlrd_Day::bkhlrd_parse(spelled).is_err(),
            "'{}' should be refused",
            spelled, //
        );
    }
}

#[test]
fn bkhltd_a_moment_falls_in_its_utc_day() {
    let day = bkhlrd_Day::bkhlrd_parse("20261105").expect("a day parses");
    let midnight = UNIX_EPOCH + Duration::from_secs(day.bkhlrd_count() as u64 * 86_400);
    assert_eq!(bkhlrd_Day::bkhlrd_from_time(midnight), day);
    assert_eq!(
        bkhlrd_Day::bkhlrd_from_time(midnight - Duration::from_secs(1)),
        day.bkhlrd_back(1), //
    );
    assert_eq!(
        bkhlrd_Day::bkhlrd_from_time(midnight + Duration::from_secs(86_399)),
        day, //
    );
    assert_eq!(
        bkhlrd_Day::bkhlrd_from_time(UNIX_EPOCH - Duration::from_millis(1)).bkhlrd_digits(),
        "19691231", //
    );
}

// eof
