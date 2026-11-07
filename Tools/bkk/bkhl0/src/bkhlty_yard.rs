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

//! The yard every hurdle stands in: a whole composed project with its own
//! trampoline, config file and per-machine file, and its own swept, output and
//! archive folders, every one beneath the dispatch's temp directory.
//!
//! THE TEMP DIRECTORY IS READ FROM THE SUITE'S OWN DISPATCH FILE, through the
//! floor's reader, and an absent pointer or field is a panic: a yard never falls
//! back to an ambient temp directory, so no hurdle can land anywhere a dispatch
//! did not hand it.
//!
//! EVERY ROOT IS CONFINED BEFORE ANY ACT: `bkhlty_confine` panics unless every
//! folder the library resolved stands strictly inside the yard, so a hurdle that
//! could reach a real machine's folders fails before it acts.
//!
//! REACHED TWICE, from the library's own hurdles and from the integration hurdle
//! that runs the built binary, which reaches this one file by path rather than
//! holding a copy. Both crate roots seat the library's ground and day modules
//! at the same path, which is all this file names.

use crate::bkhlrd_day::bkhlrd_Day;
use crate::bkhlrg_ground::{
    bkhlrg_Ground,
    BKHLRG_HAYLOFT_KEY,
    BKHLRG_LOG_KEY,
    BKHLRG_OUTPUT_KEY,
    BKHLRG_STATION_FILE_KEY,
    BKHLRG_TEMP_KEY,
    BKHLRG_TINCTURE_KEY, //
};
use std::path::{
    Path,
    PathBuf, //
};
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH, //
};

/// The pointer naming the dispatch's file.
const ZBKHLTY_POINTER_VAR: &str = "BURD_BURD_FILE";

/// The dispatch's temp directory, read from the file the pointer names.
const ZBKHLTY_TEMP_DIR_VAR: &str = "BURD_TEMP_DIR";

/// The made-up machine tag every yard carries.
pub(crate) const BKHLTY_TINCTURE: &str = "zq";

/// The project's folder inside the yard.
pub(crate) const BKHLTY_PROJECT: &str = "proj";

/// The swept folder of records, relative to the project.
pub(crate) const BKHLTY_RECORDS: &str = "../quillery";

/// The swept folder of run folders, relative to the project.
pub(crate) const BKHLTY_RUNS: &str = "../murkwell";

/// The archive folder, relative to the project.
pub(crate) const BKHLTY_HAYLOFT: &str = "../strawloft";

/// The output folder standing beside the swept ones.
pub(crate) const BKHLTY_OUTPUT_APART: &str = "../outfold";

/// The output folder standing inside the swept folder of run folders.
pub(crate) const BKHLTY_OUTPUT_WITHIN: &str = "../murkwell/outfold";

/// Seconds past midnight a planted time stands at: noon UTC.
const ZBKHLTY_NOON: u64 = 12 * 3600;

/// A composed project, removed when the hurdle drops it.
pub(crate) struct bkhlty_Yard {
    pub(crate) root: PathBuf,
}

impl bkhlty_Yard {
    /// Compose a yard named for its hurdle, with an archive folder named or
    /// not and the output folder at the place given.
    pub(crate) fn bkhlty_compose(name: &str, hayloft: bool, output: &str) -> bkhlty_Yard {
        let root = PathBuf::from(zbkhlty_declared()).join(format!(
            "bkhlty-{}",
            name, //
        ));
        let _ = std::fs::remove_dir_all(&root);
        let yard = bkhlty_Yard { root };

        yard.bkhlty_write(
            &format!(
                "{}/tt/z-launcher.sh",
                BKHLTY_PROJECT, //
            ),
            "#!/bin/bash\nz_moorings_dir=\"moor\"\n",
        );
        yard.bkhlty_write(
            &format!(
                "{}/moor/burc.env",
                BKHLTY_PROJECT, //
            ),
            &format!(
                "# a composed project file\n{}=../machine/burs.env\n{}={}\n{}={}\n",
                BKHLRG_STATION_FILE_KEY,
                BKHLRG_TEMP_KEY,
                BKHLTY_RUNS,
                BKHLRG_OUTPUT_KEY,
                output, //
            ),
        );
        let mut machine = format!(
            "{}={}\n{}={}\n",
            BKHLRG_TINCTURE_KEY,
            BKHLTY_TINCTURE,
            BKHLRG_LOG_KEY,
            BKHLTY_RECORDS, //
        );
        if hayloft {
            machine.push_str(&format!(
                "{}={}\n",
                BKHLRG_HAYLOFT_KEY,
                BKHLTY_HAYLOFT, //
            ));
        }
        yard.bkhlty_write("machine/burs.env", &machine);

        for folder in [BKHLTY_RECORDS, BKHLTY_RUNS, output] {
            std::fs::create_dir_all(yard.bkhlty_project().join(folder)).unwrap_or_else(|err| {
                panic!(
                    "could not compose the folder {}: {}",
                    folder,
                    err, //
                )
            });
        }

        yard
    }

    /// The composed project's root.
    pub(crate) fn bkhlty_project(&self) -> PathBuf {
        self.root.join(BKHLTY_PROJECT)
    }

    /// A path inside the composed project, relative to its root.
    pub(crate) fn bkhlty_at(&self, relative: &str) -> PathBuf {
        self.bkhlty_project().join(relative)
    }

    /// Write a file relative to the yard, its folders made as needed.
    pub(crate) fn bkhlty_write(&self, relative: &str, text: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|err| {
                panic!(
                    "could not compose {}: {}",
                    parent.display(),
                    err, //
                )
            });
        }
        std::fs::write(&path, text).unwrap_or_else(|err| {
            panic!(
                "could not write {}: {}",
                path.display(),
                err, //
            )
        });
    }

    /// Plant a file relative to the project, dated noon UTC of `day`.
    pub(crate) fn bkhlty_plant(&self, relative: &str, bytes: &[u8], day: bkhlrd_Day) -> PathBuf {
        let path = self.bkhlty_at(relative);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|err| {
                panic!(
                    "could not compose {}: {}",
                    parent.display(),
                    err, //
                )
            });
        }
        std::fs::write(&path, bytes).unwrap_or_else(|err| {
            panic!(
                "could not plant {}: {}",
                path.display(),
                err, //
            )
        });
        bkhlty_date(&path, day);
        path
    }
}

impl Drop for bkhlty_Yard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

/// Set a file's or a folder's modification time to noon UTC of `day`.
pub(crate) fn bkhlty_date(path: &Path, day: bkhlrd_Day) {
    let moment =
        UNIX_EPOCH + Duration::from_secs(day.bkhlrd_count() as u64 * 86_400 + ZBKHLTY_NOON);
    std::fs::File::open(path)
        .and_then(|held| held.set_modified(moment))
        .unwrap_or_else(|err| {
            panic!(
                "could not date {}: {}",
                path.display(),
                err, //
            )
        });
}

/// The modification time of a path, never its link's target's.
pub(crate) fn bkhlty_mtime(path: &Path) -> SystemTime {
    std::fs::symlink_metadata(path)
        .and_then(|stat| stat.modified())
        .unwrap_or_else(|err| {
            panic!(
                "could not read the time of {}: {}",
                path.display(),
                err, //
            )
        })
}

/// Panic unless every folder the library resolved stands strictly inside the
/// yard: called before any act.
pub(crate) fn bkhlty_confine(yard: &bkhlty_Yard, ground: &bkhlrg_Ground) {
    let fence = std::fs::canonicalize(&yard.root).unwrap_or_else(|err| {
        panic!(
            "the yard {} does not resolve: {}",
            yard.root.display(),
            err, //
        )
    });
    let mut roots = vec![
        &ground.root,
        &ground.log,
        &ground.temp,
        &ground.output, //
    ];
    if let Some(hayloft) = &ground.hayloft {
        roots.push(&hayloft.folder);
    }
    for root in roots {
        assert!(
            root.starts_with(&fence) && root != &fence,
            "{} stands outside the yard {}, and no hurdle acts there",
            root.display(),
            fence.display(), //
        );
    }
}

/// The dispatch's temp directory, refusing where the pointer or the field is
/// absent.
fn zbkhlty_declared() -> String {
    let pointer = std::env::var_os(ZBKHLTY_POINTER_VAR).unwrap_or_else(|| {
        panic!(
            "{} is unset. The hurdles stand under the dispatch's temp directory and reach for no \
             ambient one; drive the suite through its own door",
            ZBKHLTY_POINTER_VAR, //
        )
    });
    bkp::bkprc_catena::bkprc_read(Path::new(&pointer))
        .unwrap_or_else(|err| {
            panic!(
                "{} names no readable dispatch file: {}",
                ZBKHLTY_POINTER_VAR,
                err, //
            )
        })
        .bkprc_scalar(ZBKHLTY_TEMP_DIR_VAR)
        .map(str::trim)
        .filter(|dir| !dir.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| {
            panic!(
                "the suite's dispatch file names no {}",
                ZBKHLTY_TEMP_DIR_VAR, //
            )
        })
}

// eof
