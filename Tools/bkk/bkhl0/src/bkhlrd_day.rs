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

//! Calendar days in UTC.
//!
//! THE DAY IS UTC'S, NEVER THE LOCAL ZONE'S. The standard library carries no
//! time zone, and reading one would take a dependency this tool does not hold,
//! so a file's day is the UTC calendar day its modification time falls in, and
//! the usage text says so.
//!
//! A day is counted from the Unix epoch, so ordering and stepping back are
//! integer arithmetic; its eight-digit spelling is the one form the command line
//! takes and the archive names wear.

use std::time::{
    SystemTime,
    UNIX_EPOCH, //
};

/// Seconds in one UTC day.
const ZBKHLRD_DAY_SECONDS: i64 = 86_400;

/// The count of digits a day is spelled in.
const ZBKHLRD_DIGITS: usize = 8;

/// One UTC calendar day, counted from 1970-01-01.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct bkhlrd_Day {
    days: i64,
}

impl bkhlrd_Day {
    /// The day so many days after the epoch.
    pub fn bkhlrd_from_count(days: i64) -> bkhlrd_Day {
        bkhlrd_Day { days }
    }

    /// The count of days since the epoch.
    pub fn bkhlrd_count(&self) -> i64 {
        self.days
    }

    /// The UTC day a moment falls in, moments before the epoch included.
    pub fn bkhlrd_from_time(moment: SystemTime) -> bkhlrd_Day {
        bkhlrd_Day {
            days: bkhlrd_seconds(moment).div_euclid(ZBKHLRD_DAY_SECONDS),
        }
    }

    /// The day so many days before this one.
    pub fn bkhlrd_back(&self, days: i64) -> bkhlrd_Day {
        bkhlrd_Day {
            days: self.days - days,
        }
    }

    /// Read a day spelled as eight digits, year then month then day, refusing
    /// any other spelling and any date the calendar does not hold.
    pub fn bkhlrd_parse(spelled: &str) -> Result<bkhlrd_Day, String> {
        if spelled.len() != ZBKHLRD_DIGITS || !spelled.bytes().all(|b| b.is_ascii_digit()) {
            return Err(format!(
                "'{}' is not a day: a day is eight digits, year then month then day, as 20260131",
                spelled, //
            ));
        }

        let year: i64 = spelled[0..4].parse().map_err(|_| spelled.to_string())?;
        let month: i64 = spelled[4..6].parse().map_err(|_| spelled.to_string())?;
        let day: i64 = spelled[6..8].parse().map_err(|_| spelled.to_string())?;

        let counted = zbkhlrd_days_from_civil(year, month, day);
        if !(1..=12).contains(&month) || zbkhlrd_civil_from_days(counted) != (year, month, day) {
            return Err(format!(
                "'{}' names no calendar date",
                spelled, //
            ));
        }

        Ok(bkhlrd_Day { days: counted })
    }

    /// The day's eight-digit spelling.
    pub fn bkhlrd_digits(&self) -> String {
        let (year, month, day) = zbkhlrd_civil_from_days(self.days);
        format!(
            "{:04}{:02}{:02}",
            year,
            month,
            day, //
        )
    }
}

/// Whole seconds from the epoch to a moment, negative before it.
pub fn bkhlrd_seconds(moment: SystemTime) -> i64 {
    match moment.duration_since(UNIX_EPOCH) {
        Ok(after) => after.as_secs() as i64,
        Err(before) => {
            let back = before.duration();
            let whole = back.as_secs() as i64;
            if back.subsec_nanos() > 0 {
                -whole - 1
            } else {
                -whole
            }
        }
    }
}

/// The count of days from the epoch to a proleptic Gregorian date.
fn zbkhlrd_days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let shifted = if month <= 2 { year - 1 } else { year };
    let era = shifted.div_euclid(400);
    let year_of_era = shifted - era * 400;
    let month_from_march = if month > 2 { month - 3 } else { month + 9 };
    let day_of_year = (153 * month_from_march + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// The proleptic Gregorian date so many days from the epoch.
fn zbkhlrd_civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_from_march = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_from_march + 2) / 5 + 1;
    let month = if month_from_march < 10 {
        month_from_march + 3
    } else {
        month_from_march - 9
    };
    let year = year_of_era + era * 400 + if month <= 2 { 1 } else { 0 };
    (year, month, day)
}

// eof
