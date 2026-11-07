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

//! The folders the tool acts on, read from the regime files and never from the
//! environment.
//!
//! THE ROAD IS FILES ALONE. The tabtarget trampoline names the project's config
//! directory on its `z_moorings_dir="` line; that directory holds `burc.env`,
//! whose `BURC_STATION_FILE` names the per-machine file relative to the project
//! root. Each file is read through the floor's regime reader, so this tool
//! admits exactly the subset bash and every other reader admit. A redirected
//! environment — a dispatch pointing its folders somewhere of its own — never
//! steers it: the folders acted on are always the ones the files name.
//!
//! EVERY PATH IS RELATIVE TO THE PROJECT ROOT, and an absolute value refuses.
//!
//! THE FOLDERS ARE PROVEN APART BEFORE ANYTHING IS WRITTEN: the archive folder
//! may stand inside neither swept folder, neither swept folder inside the other,
//! and neither swept folder may be or hold the project root.

use bkp::bkprc_catena::{
    bkprc_Regime,
    bkprc_read, //
};
use std::path::{
    Component,
    Path,
    PathBuf, //
};

/// The trampoline every tabtarget execs, relative to the project root.
const ZBKHLRG_TRAMPOLINE: &str = "tt/z-launcher.sh";

/// The trampoline line naming the project's config directory, up to the value.
const ZBKHLRG_CONFIG_DIR_OPEN: &str = "z_moorings_dir=\"";

/// The project config file the config directory holds.
const ZBKHLRG_PROJECT_FILE: &str = "burc.env";

/// The project config field naming the per-machine file.
pub const BKHLRG_STATION_FILE_KEY: &str = "BURC_STATION_FILE";

/// The project config field naming the folder of dispatch run folders.
pub const BKHLRG_TEMP_KEY: &str = "BURC_TEMP_ROOT_DIR";

/// The project config field naming the dispatch output folder.
pub const BKHLRG_OUTPUT_KEY: &str = "BURC_OUTPUT_ROOT_DIR";

/// The per-machine field naming the folder of tabtarget records.
pub const BKHLRG_LOG_KEY: &str = "BURS_LOG_DIR";

/// The per-machine field naming the archive folder.
pub const BKHLRG_HAYLOFT_KEY: &str = "BURS_HAYLOFT";

/// The per-machine field naming the machine's short tag.
pub const BKHLRG_TINCTURE_KEY: &str = "BURS_TINCTURE";

/// The archive folder's suggested value, as the line to add spells it.
pub const BKHLRG_HAYLOFT_EXAMPLE: &str = "../station-hayloft-buk";

/// The archive folder, as the per-machine file spells it and as it resolves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkhlrg_Hayloft {
    /// The value as written, relative to the project root.
    pub spelled: String,
    /// The folder it resolves to.
    pub folder: PathBuf,
}

/// Every folder the tool reads or writes, resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkhlrg_Ground {
    /// The project root every value is relative to.
    pub root: PathBuf,
    /// The folder of tabtarget records, swept by age.
    pub log: PathBuf,
    /// The folder of dispatch run folders, swept by age.
    pub temp: PathBuf,
    /// The dispatch output folder, never touched.
    pub output: PathBuf,
    /// The archive folder, where the per-machine file names one.
    pub hayloft: Option<bkhlrg_Hayloft>,
    /// The machine's short tag, the archive names' head.
    pub tincture: String,
}

impl bkhlrg_Ground {
    /// The archive folder, refusing in this tool's voice where none is named.
    pub fn bkhlrg_hayloft(&self) -> Result<&bkhlrg_Hayloft, String> {
        self.hayloft.as_ref().ok_or_else(|| {
            format!(
                "the per-machine file names no {} - add a line naming the archive folder relative \
                 to the project root, as {}={}",
                BKHLRG_HAYLOFT_KEY,
                BKHLRG_HAYLOFT_KEY,
                BKHLRG_HAYLOFT_EXAMPLE, //
            )
        })
    }
}

/// Read the folders the regime files name for the project at `root`, proving
/// them apart.
pub fn bkhlrg_read(root: &Path) -> Result<bkhlrg_Ground, String> {
    let root = std::fs::canonicalize(root).map_err(|err| {
        format!(
            "the project root {} does not resolve: {}",
            root.display(),
            err, //
        )
    })?;

    let config_dir = zbkhlrg_config_dir(&root)?;
    let project = bkprc_read(&root.join(&config_dir).join(ZBKHLRG_PROJECT_FILE))?;
    let machine_file = zbkhlrg_required(&project, BKHLRG_STATION_FILE_KEY)?;
    let machine = bkprc_read(&root.join(&machine_file))?;

    let temp = bkhlrg_resolve(&root.join(zbkhlrg_required(&project, BKHLRG_TEMP_KEY)?));
    let output = bkhlrg_resolve(&root.join(zbkhlrg_required(&project, BKHLRG_OUTPUT_KEY)?));
    let log = bkhlrg_resolve(&root.join(zbkhlrg_required(&machine, BKHLRG_LOG_KEY)?));

    let tincture = machine
        .bkprc_scalar(BKHLRG_TINCTURE_KEY)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            format!(
                "{} names no {}",
                machine_file,
                BKHLRG_TINCTURE_KEY, //
            )
        })?
        .to_string();

    let hayloft = match machine
        .bkprc_scalar(BKHLRG_HAYLOFT_KEY)
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        Some(spelled) => {
            zbkhlrg_relative(BKHLRG_HAYLOFT_KEY, spelled)?;
            Some(bkhlrg_Hayloft {
                spelled: spelled.to_string(),
                folder: bkhlrg_resolve(&root.join(spelled)),
            })
        }
        None => None,
    };

    let ground = bkhlrg_Ground {
        root,
        log,
        temp,
        output,
        hayloft,
        tincture,
    };

    zbkhlrg_apart(&ground)?;

    Ok(ground)
}

/// A path made absolute and free of `.` and `..`, its longest standing prefix
/// resolved through the filesystem so a link in it is followed once.
///
/// A folder that does not stand yet — an archive folder bale will create, a
/// target unbale will create — still resolves, so every nesting question is
/// asked of the place a write would land.
pub fn bkhlrg_resolve(path: &Path) -> PathBuf {
    let mut lexical = PathBuf::new();
    for part in path.components() {
        match part {
            Component::CurDir => {}
            Component::ParentDir => {
                lexical.pop();
            }
            other => lexical.push(other.as_os_str()),
        }
    }

    let mut standing = lexical.clone();
    let mut rest: Vec<std::ffi::OsString> = Vec::new();
    loop {
        if let Ok(real) = std::fs::canonicalize(&standing) {
            let mut resolved = real;
            for part in rest.iter().rev() {
                resolved.push(part);
            }
            return resolved;
        }
        match (
            standing.file_name().map(|n| n.to_os_string()),
            standing.parent(),
        ) {
            (Some(name), Some(parent)) => {
                rest.push(name);
                standing = parent.to_path_buf();
            }
            _ => return lexical,
        }
    }
}

/// Whether `inner` is `outer` or stands beneath it.
pub fn bkhlrg_within(inner: &Path, outer: &Path) -> bool {
    inner.starts_with(outer)
}

/// The config directory the trampoline names.
fn zbkhlrg_config_dir(root: &Path) -> Result<String, String> {
    let trampoline = root.join(ZBKHLRG_TRAMPOLINE);
    let text = std::fs::read_to_string(&trampoline).map_err(|err| {
        format!(
            "no trampoline reads at {}: {}",
            trampoline.display(),
            err, //
        )
    })?;

    text.lines()
        .find_map(|line| {
            line.strip_prefix(ZBKHLRG_CONFIG_DIR_OPEN)
                .and_then(|rest| rest.split('"').next())
        })
        .filter(|named| !named.is_empty())
        .map(str::to_string)
        .ok_or_else(|| {
            format!(
                "the trampoline {} carries no {} line",
                trampoline.display(),
                ZBKHLRG_CONFIG_DIR_OPEN, //
            )
        })
}

/// One field's value, refusing where it is absent, empty or absolute.
fn zbkhlrg_required(regime: &bkprc_Regime, key: &str) -> Result<String, String> {
    let value = regime
        .bkprc_scalar(key)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            format!(
                "no value stands for {}",
                key, //
            )
        })?;
    zbkhlrg_relative(key, value)?;
    Ok(value.to_string())
}

/// Refuse an absolute value: every path is written relative to the project root.
fn zbkhlrg_relative(key: &str, value: &str) -> Result<(), String> {
    if Path::new(value).is_absolute() {
        return Err(format!(
            "{} is absolute ({}): it is written relative to the project root",
            key,
            value, //
        ));
    }
    Ok(())
}

/// Prove the folders apart.
fn zbkhlrg_apart(ground: &bkhlrg_Ground) -> Result<(), String> {
    let swept = [
        (BKHLRG_LOG_KEY, &ground.log),
        (BKHLRG_TEMP_KEY, &ground.temp),
    ];

    for (key, folder) in swept {
        if bkhlrg_within(&ground.root, folder) {
            return Err(format!(
                "{} resolves to {}, which is or holds the project root: emptying it would take \
                 the project",
                key,
                folder.display(), //
            ));
        }
    }

    if bkhlrg_within(&ground.log, &ground.temp) || bkhlrg_within(&ground.temp, &ground.log) {
        return Err(format!(
            "{} ({}) and {} ({}) resolve one inside the other: the two folders are swept apart \
             and must stand apart",
            BKHLRG_LOG_KEY,
            ground.log.display(),
            BKHLRG_TEMP_KEY,
            ground.temp.display(), //
        ));
    }

    if let Some(hayloft) = &ground.hayloft {
        for (key, folder) in swept {
            if bkhlrg_within(&hayloft.folder, folder) {
                return Err(format!(
                    "{} resolves to {}, inside the folder {} names: an archive written there \
                     would stand among what is swept",
                    BKHLRG_HAYLOFT_KEY,
                    hayloft.folder.display(),
                    key, //
                ));
            }
        }
    }

    Ok(())
}

// eof
