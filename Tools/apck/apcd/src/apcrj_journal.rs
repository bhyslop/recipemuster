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

//! APCK journal directory — one place on disk that accumulates runtime
//! artifacts: verbatim clipboard harvests on the Clinical branch, and a
//! running log of every `apcrl_*` emission teed from stdout. Resolved under
//! `$HOME/apcjd/`. This module does not emit — it resolves paths only.

use std::path::PathBuf;

pub const APCRJ_DIR_NAME:      &str = "apcjd";
pub const APCRJ_LOG_FILE_NAME: &str = "apcap.log";

/// Resolve the journal directory under `$HOME`. Returns `None` when `HOME`
/// is unset. The directory is not created here — creation is lazy at first
/// write by the consumer (harvest or log tee).
pub fn apcrj_journal_path() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join(APCRJ_DIR_NAME))
}

/// Resolve the log file path under the journal directory. Returns `None`
/// when `HOME` is unset.
pub fn apcrj_log_path() -> Option<PathBuf> {
    apcrj_journal_path().map(|d| d.join(APCRJ_LOG_FILE_NAME))
}
