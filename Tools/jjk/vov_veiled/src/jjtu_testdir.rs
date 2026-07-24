// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! RAII test directory guard for JJK tests, plus the shared seam-ON read ground.
//!
//! Provides automatic cleanup of temporary directories, even on panic.

use std::path::{Path, PathBuf};

use crate::jjrg_gallops::jjrg_Gallops;
use crate::jjrvb_blotter::{jjdb_BlotterConfig, jjdb_studbook_config, JJDB_GALLOPS_REL_PATH};

/// The one crate-wide cwd serial. The process cwd is process-global state, so
/// every test ground that repoints it — whatever module it lives in — must
/// serialize through THIS mutex and no other: two modules each holding their
/// own serial still race each other in the parallel runner, and a cwd yanked
/// mid-test sends one fixture's git effects into another fixture's repo (the
/// observed shape: one module's commit pushed to another module's remote).
pub static JJTU_CWD_SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub struct JjkTestDir(PathBuf);

impl JjkTestDir {
    pub fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(name);
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for JjkTestDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Run a git subcommand in `dir`, panicking loud on failure (a ground that fails
/// to build is a broken test, never a silent skip).
fn zjjtu_git(dir: &Path, args: &[&str]) {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("jjtu ground: git spawn -C {} {:?}: {}", dir.display(), args, e));
    assert!(
        out.status.success(),
        "jjtu ground: git -C {} {:?} failed: {}",
        dir.display(),
        args,
        String::from_utf8_lossy(&out.stderr).trim()
    );
}

/// A studbook config pointed at nothing — for seam-OFF tests, where a load must
/// read its `path` and never reach for the studbook. The dead paths make the
/// inertness self-proving: an off branch that touched this config would error,
/// so a green seam-off test also witnesses that off never dials the studbook.
pub fn jjtu_poison_config() -> jjdb_BlotterConfig {
    jjdb_BlotterConfig {
        local_root: PathBuf::from("/nonexistent/jjtu-poison"),
        remote_url: String::new(),
        trunk: String::new(),
        ordinal_sigil: crate::jjrvb_blotter::JJDB_CATCHWORD_SIGIL,
        ordinal_founding: crate::jjrvb_blotter::JJDB_CATCHWORD_FOUNDING,
    }
}

/// A seam-ON read ground: a studbook clone laid at `jjdb_studbook_config`'s
/// derived local_root (`<root>/jjqs_studbook`), seeded with `seed` and pushed to
/// `origin/trunk` so a read through `zjjrm_load_gallops_over(true, ..)` resolves
/// the pinned tip. The read analogue of `jjtvb_blotter`'s `ZjjtvbGround`, laid
/// exactly where the cwd-derived funnel would look — the config is
/// `jjdb_studbook_config`'s own output, verbatim, so a test drives the studbook
/// seam end-to-end at the real derived path. The pinned load is a local ref-read
/// only (`refs/remotes/origin/trunk`, no network), so the config's real
/// `remote_url` is never dialed. Returns (root_guard, config); dropping the guard
/// reaps the tree.
pub fn jjtu_seam_on_ground(name: &str, seed: jjrg_Gallops) -> (JjkTestDir, jjdb_BlotterConfig) {
    let root = JjkTestDir::new(name);
    let config = jjdb_studbook_config(root.path());
    let trunk = config.trunk.clone();

    // Bare origin (scratch — the config's real remote_url is never contacted).
    let bare = root.path().join("origin.git");
    std::fs::create_dir_all(&bare).unwrap();
    zjjtu_git(&bare, &["init", "-q", "--bare", "-b", &trunk]);

    // Local clone laid at the derived studbook path, seeded and pushed so the
    // remote-tracking ref the pin resolves (`refs/remotes/origin/trunk`) exists.
    let local = &config.local_root;
    std::fs::create_dir_all(local).unwrap();
    zjjtu_git(local, &["init", "-q", "-b", &trunk]);
    zjjtu_git(local, &["config", "user.email", "jjtu@test"]);
    zjjtu_git(local, &["config", "user.name", "jjtu"]);
    seed.jjrg_save(&local.join(JJDB_GALLOPS_REL_PATH))
        .expect("seed gallops saves to the ground clone");
    zjjtu_git(local, &["add", JJDB_GALLOPS_REL_PATH]);
    zjjtu_git(local, &["commit", "-q", "-m", "seed"]);
    zjjtu_git(local, &["remote", "add", "origin", &bare.to_string_lossy()]);
    zjjtu_git(local, &["push", "-q", "-u", "origin", &trunk]);

    (root, config)
}
