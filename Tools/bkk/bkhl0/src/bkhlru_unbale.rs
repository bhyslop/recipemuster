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

//! Unbale: list the archive folder, or unpack one archive into a fresh folder.
//!
//! THE LISTING IS PASTE-READY: one archive per line, each spelled as the archive
//! folder's own value joined with the archive's name, so it reads relative to
//! the project root exactly as the command line takes it. A temporary left by a
//! failed bale wears a hidden name and is never listed.
//!
//! THE TARGET IS EMPTY OR ABSENT, and never inside either swept folder: an
//! unpack lands beside the live folders, never among them. The tar crate's own
//! unpack keeps every member inside the target, keeps modification times, and
//! makes a link as a link.

use crate::bkhlrb_bale::BKHLRB_SUFFIX;
use crate::bkhlrg_ground::{
    bkhlrg_Ground,
    bkhlrg_resolve,
    bkhlrg_within,
    BKHLRG_LOG_KEY,
    BKHLRG_TEMP_KEY, //
};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::BufReader;
use std::path::{
    Path,
    PathBuf, //
};
use tar::Archive;

/// The archives standing in the archive folder, each a path relative to the
/// project root, in name order; none where the folder does not stand yet.
pub fn bkhlru_listing(ground: &bkhlrg_Ground) -> Result<Vec<String>, String> {
    let hayloft = ground.bkhlrg_hayloft()?;
    if !hayloft.folder.exists() {
        return Ok(Vec::new());
    }

    let mut names: Vec<String> = Vec::new();
    let entries = std::fs::read_dir(&hayloft.folder).map_err(|err| {
        format!(
            "could not read {}: {}",
            hayloft.folder.display(),
            err, //
        )
    })?;
    for entry in entries {
        let entry = entry.map_err(|err| err.to_string())?;
        let Some(name) = entry.file_name().to_str().map(str::to_string) else {
            continue;
        };
        let regular = entry
            .file_type()
            .map(|kind| kind.is_file())
            .unwrap_or(false);
        if regular && name.ends_with(BKHLRB_SUFFIX) && !name.starts_with('.') {
            names.push(name);
        }
    }
    names.sort();

    let folder = hayloft.spelled.trim_end_matches('/');
    Ok(names
        .into_iter()
        .map(|name| {
            format!(
                "{}/{}",
                folder,
                name, //
            )
        })
        .collect())
}

/// Unpack `archive` into `target`, each read relative to the project root
/// where it is not absolute, answering the folder unpacked into.
pub fn bkhlru_unbale(
    ground: &bkhlrg_Ground,
    archive: &Path,
    target: &Path,
) -> Result<PathBuf, String> {
    let archive = bkhlrg_resolve(&ground.root.join(archive));
    let target = bkhlrg_resolve(&ground.root.join(target));

    if !archive.is_file() {
        return Err(format!(
            "no archive stands at {}",
            archive.display(), //
        ));
    }

    for (key, folder) in [
        (BKHLRG_LOG_KEY, &ground.log),
        (BKHLRG_TEMP_KEY, &ground.temp),
    ] {
        if bkhlrg_within(&target, folder) {
            return Err(format!(
                "the target {} stands inside the folder {} names, and an unpack never writes \
                 among the live folders",
                target.display(),
                key, //
            ));
        }
    }

    match std::fs::symlink_metadata(&target) {
        Ok(stat) if stat.is_dir() => {
            let mut held = std::fs::read_dir(&target).map_err(|err| err.to_string())?;
            if held.next().is_some() {
                return Err(format!(
                    "the target {} is not empty: unbale writes only into an empty or absent folder",
                    target.display(), //
                ));
            }
        }
        Ok(_) => {
            return Err(format!(
                "the target {} stands and is no folder",
                target.display(), //
            ));
        }
        Err(_) => {
            std::fs::create_dir_all(&target).map_err(|err| {
                format!(
                    "could not create the target {}: {}",
                    target.display(),
                    err, //
                )
            })?;
        }
    }

    let opened = File::open(&archive).map_err(|err| {
        format!(
            "could not open {}: {}",
            archive.display(),
            err, //
        )
    })?;
    let mut reader = Archive::new(GzDecoder::new(BufReader::new(opened)));
    reader.set_preserve_mtime(true);
    reader.set_preserve_permissions(true);
    reader.set_overwrite(false);
    reader.unpack(&target).map_err(|err| {
        format!(
            "could not unpack {} into {}: {}",
            archive.display(),
            target.display(),
            err, //
        )
    })?;

    Ok(target)
}

// eof
