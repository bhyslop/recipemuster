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

//! Self-update watcher — monitors /Users/Shared/apcua/ for new .app bundles.

// RCG output discipline: all emission via apcrl_*! — no direct println!/eprintln!

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};

pub const APCRU_STAGING_DIR: &str = "/Users/Shared/apcua";

/// Start the self-update watcher on a background thread.
pub fn apcru_start_watcher() {
    std::thread::spawn(zapcru_watch_loop);
}

fn zapcru_watch_loop() {
    let staging = Path::new(APCRU_STAGING_DIR);
    if !staging.exists() {
        if let Err(e) = std::fs::create_dir_all(staging) {
            crate::apcrl_error_now!(
                "failed to create staging dir {}: {}", APCRU_STAGING_DIR, e
            );
            return;
        }
        crate::apcrl_info_now!("created staging directory {}", APCRU_STAGING_DIR);
    }

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = match RecommendedWatcher::new(
        move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default(),
    ) {
        Ok(w) => w,
        Err(e) => {
            crate::apcrl_error_now!("failed to create file watcher: {}", e);
            return;
        }
    };

    if let Err(e) = watcher.watch(staging, RecursiveMode::NonRecursive) {
        crate::apcrl_error_now!("failed to watch {}: {}", APCRU_STAGING_DIR, e);
        return;
    }

    crate::apcrl_info_now!("watching {} for updates", APCRU_STAGING_DIR);

    for event in rx {
        if !matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
            continue;
        }
        // Debounce: let scp finish copying the bundle contents
        std::thread::sleep(std::time::Duration::from_secs(2));
        if let Some(staged_app) = zapcru_find_app_bundle(staging) {
            zapcru_apply_update(&staged_app);
        }
    }
}

/// Find the first .app bundle in the staging directory.
fn zapcru_find_app_bundle(dir: &Path) -> Option<PathBuf> {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            crate::apcrl_error_now!("failed to read staging dir: {}", e);
            return None;
        }
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "app") && path.is_dir() {
            return Some(path);
        }
    }
    None
}

/// Walk up from the running executable to find the enclosing .app bundle.
fn zapcru_current_bundle_path() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    // macOS .app structure: Something.app/Contents/MacOS/binary
    let mut path = exe.as_path();
    loop {
        if path.extension().map_or(false, |ext| ext == "app") && path.is_dir() {
            return Some(path.to_path_buf());
        }
        path = path.parent()?;
    }
}

/// Copy the staged bundle over the current bundle, relaunch, and exit.
fn zapcru_apply_update(staged_app: &Path) {
    crate::apcrl_info_now!("update detected: {}", staged_app.display());

    let current_bundle = match zapcru_current_bundle_path() {
        Some(p) => p,
        None => {
            // Running via cargo run — no .app bundle to replace
            crate::apcrl_info_now!(
                "not running from .app bundle, launching staged app directly"
            );
            zapcru_launch_and_exit(staged_app);
        }
    };

    if staged_app == current_bundle {
        crate::apcrl_info_now!("staged app is current bundle, skipping");
        return;
    }

    crate::apcrl_info_now!(
        "replacing {} with {}", current_bundle.display(), staged_app.display()
    );

    if let Err(e) = zapcru_replace_bundle(&current_bundle, staged_app) {
        crate::apcrl_error_now!("failed to replace bundle: {}", e);
        return;
    }

    zapcru_launch_and_exit(&current_bundle);
}

/// Remove the current bundle and copy the staged bundle to its location.
fn zapcru_replace_bundle(current: &Path, staged: &Path) -> Result<(), String> {
    std::fs::remove_dir_all(current)
        .map_err(|e| format!("remove current bundle: {}", e))?;
    // ditto preserves macOS extended attributes and resource forks
    let status = std::process::Command::new("ditto")
        .arg(staged)
        .arg(current)
        .status()
        .map_err(|e| format!("ditto command: {}", e))?;
    if !status.success() {
        return Err(format!("ditto exited with status {}", status));
    }
    Ok(())
}

/// Launch the app bundle via `open -n` and exit the current process.
fn zapcru_launch_and_exit(app_path: &Path) -> ! {
    crate::apcrl_info_now!("launching {} and exiting", app_path.display());
    let _ = std::process::Command::new("open")
        .arg("-n")
        .arg(app_path)
        .spawn();
    std::process::exit(0);
}
