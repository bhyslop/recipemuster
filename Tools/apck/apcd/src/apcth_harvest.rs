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

//! Tests for clipboard harvest scan/write helpers. Full-capture tests are
//! out of scope here — the clipboard is a global resource and arboard cannot
//! be exercised reliably under parallel `cargo test`.

use super::apcrh_harvest::*;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static ZAPCTH_NONCE: AtomicU64 = AtomicU64::new(0);

struct zapcth_TempDir {
    path: PathBuf,
}

impl zapcth_TempDir {
    fn zapcth_new(tag: &str) -> Self {
        let nonce = ZAPCTH_NONCE.fetch_add(1, Ordering::SeqCst);
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let mut path = std::env::temp_dir();
        path.push(format!("apcth_{}_{}_{}_{}", tag, std::process::id(), nanos, nonce));
        fs::create_dir_all(&path).unwrap();
        zapcth_TempDir { path }
    }
}

impl Drop for zapcth_TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn apcth_scan_empty_dir_returns_seed() {
    let td = zapcth_TempDir::zapcth_new("empty");
    let next = zapcrh_scan_next_index(&td.path).unwrap();
    assert_eq!(next, APCRH_HARVEST_SEED_INDEX);
}

#[test]
fn apcth_scan_same_index_group_counts_once() {
    let td = zapcth_TempDir::zapcth_new("group");
    fs::write(td.path.join("10000.html"), "<html/>").unwrap();
    fs::write(td.path.join("10000.txt"),  "text").unwrap();
    let next = zapcrh_scan_next_index(&td.path).unwrap();
    assert_eq!(next, 10001);
}

#[test]
fn apcth_scan_ignores_non_numeric_stems() {
    let td = zapcth_TempDir::zapcth_new("alpha");
    fs::write(td.path.join("README.md"),     "docs").unwrap();
    fs::write(td.path.join("notes.txt"),     "notes").unwrap();
    fs::write(td.path.join("10005.txt"),     "five").unwrap();
    fs::write(td.path.join("garbage.html"),  "<p/>").unwrap();
    let next = zapcrh_scan_next_index(&td.path).unwrap();
    assert_eq!(next, 10006);
}

#[test]
fn apcth_scan_gaps_do_not_fill() {
    let td = zapcth_TempDir::zapcth_new("gaps");
    fs::write(td.path.join("10000.txt"), "a").unwrap();
    fs::write(td.path.join("10003.txt"), "b").unwrap();
    fs::write(td.path.join("10007.txt"), "c").unwrap();
    let next = zapcrh_scan_next_index(&td.path).unwrap();
    assert_eq!(next, 10008);
}
