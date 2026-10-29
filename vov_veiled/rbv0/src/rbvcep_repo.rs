// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVCEP — repository anchoring, the fixed conventional sibling locations, and
// the dispose-by-rename discipline (RBSHE step 3, seposition).
//
// Every path the lap touches is derived from the maintainer repository root,
// never hand-typed, so the artifacts always land beside the tree the lap was
// launched from.

use std::path::{Path, PathBuf};

use crate::rbvcer_run;

// The fixed regulus parent and freshness-scratch directories are the elected
// world's (rbvcew_world bundle: regulus_dirname, freshness_dirname), conserved
// spellings declared in the cosmology's cadaster; none is spelled here.

/// The clone subdir the cut creates beneath its target dir (rbvcex_expede
/// builds there; the lap and the temenos stage find what it built through this
/// one name).
pub const RBVCEP_REGULUS_SUBDIR: &str = "regulus";

/// The identity-free station tree the regulus gets, a sibling of the clone
/// (RELEASE.md step 5 / rbk-expede): {parent}/station-files/{burs.env,secrets/}.
pub const RBVCEP_STATION_SUBDIR: &str = "station-files";
pub const RBVCEP_STATION_FILE: &str = "burs.env";
pub const RBVCEP_SECRETS_SUBDIR: &str = "secrets";
pub const RBVCEP_LOGS_SUBDIR: &str = "logs-buk";
pub const RBVCEP_TACKROOM_SUBDIR: &str = "tackroom-buk";

/// The maintainer repository root, from git. Fatal if not in a repo, empty, or
/// non-absolute — every derived path anchors on it, and the seposition guards
/// compare against it.
pub fn rbvcep_toplevel() -> PathBuf {
    let cwd = std::env::current_dir()
        .unwrap_or_else(|e| crate::rbvcel_fatal!("cannot read working directory: {}", e));
    let got = rbvcer_run::rbvcer_capture("git", &["rev-parse", "--show-toplevel"], &cwd);
    if got.code != 0 {
        crate::rbvcel_fatal!(
            "not inside a git repository — essai must run from the maintainer tree: {}",
            got.stderr.trim()
        );
    }
    let top = got.stdout.trim();
    if top.is_empty() {
        crate::rbvcel_fatal!("git returned an empty repository root");
    }
    let top = PathBuf::from(top);
    if !top.is_absolute() {
        crate::rbvcel_fatal!("repository root is not an absolute path: {}", top.display());
    }
    top
}

/// A path as an owned &str, fatal on non-UTF-8. Every path the lap handles is
/// derived from git output or a repo-relative join, so non-UTF-8 is a real fault
/// worth stopping on, not something to lossily paper over into a subprocess arg.
pub fn rbvcep_as_str(p: &Path) -> String {
    p.to_str()
        .unwrap_or_else(|| crate::rbvcel_fatal!("path is not valid UTF-8: {}", p.display()))
        .to_string()
}

/// The parent directory the sibling artifacts land in. Fatal if the root has no
/// parent.
pub fn rbvcep_parent(top: &Path) -> PathBuf {
    top.parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| crate::rbvcel_fatal!("repository root has no parent: {}", top.display()))
}

/// Sepose a directory: dispose of it by RENAME to a timestamped sibling — never
/// a delete (RBSHE step 3, the dispose-by-rename discipline the temenos proves).
/// Absent target is a no-op. The caller asserts the target's identity (fixed
/// basename, not the repo root) before calling; this only refuses to clobber an
/// existing seposed sibling. Returns whether anything was seposed.
pub fn rbvcep_sepose(dir: &Path, cwd: &Path) -> bool {
    if !dir.exists() {
        return false;
    }
    let base = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| crate::rbvcel_fatal!("cannot read basename of {}", dir.display()));
    let stamp = rbvcer_run::rbvcer_timestamp(cwd);
    let seposed = dir.with_file_name(format!("{}.seposed-{}", base, stamp));
    if seposed.exists() {
        crate::rbvcel_fatal!("seposition target already exists: {}", seposed.display());
    }
    std::fs::rename(dir, &seposed).unwrap_or_else(|e| {
        crate::rbvcel_fatal!(
            "failed to sepose {} -> {}: {}",
            dir.display(),
            seposed.display(),
            e
        )
    });
    crate::rbvcel_log::rbvcel_line(&format!("seposed prior: {}", seposed.display()));
    true
}

/// Assert a target directory is safe to move aside wholesale: its basename is
/// exactly the fixed name, and it is not the maintainer repo root. Serves the
/// regulus and the freshness scratch beside the maintainer tree — the disposal
/// is a rename, so even a wrong guard destroys nothing.
pub fn rbvcep_guard_disposable(dir: &Path, expected_basename: &str, top: &Path) {
    let base = dir.file_name().and_then(|n| n.to_str());
    if base != Some(expected_basename) {
        crate::rbvcel_fatal!(
            "refusing to dispose {} — its basename is not '{}'",
            dir.display(),
            expected_basename
        );
    }
    if dir == top {
        crate::rbvcel_fatal!(
            "refusing to dispose {} — it is the maintainer repository root",
            dir.display()
        );
    }
}

/// The tree hash of `dir`'s HEAD — the byte-identity a cachet is keyed on and
/// the freshness matcher compares (RBS0 rbshc_cachet, RBSHD/RBSHO). Fatal on
/// any git failure or an empty result: a cachet or a freshness verdict keyed
/// on an empty string would silently compare equal to nothing.
pub fn rbvcep_tree_hash(dir: &Path, cwd: &Path) -> String {
    let dir_str = rbvcep_as_str(dir);
    let got = rbvcer_run::rbvcer_capture("git", &["-C", &dir_str, "rev-parse", "HEAD^{tree}"], cwd);
    if got.code != 0 {
        crate::rbvcel_fatal!("failed to read the tree hash of {}:\n{}", dir.display(), got.stderr.trim());
    }
    let hash = got.stdout.trim().to_string();
    if hash.is_empty() {
        crate::rbvcel_fatal!("git rev-parse HEAD^{{tree}} in {} returned empty", dir.display());
    }
    hash
}

/// The commit SHA of `dir`'s HEAD (the tip). Fatal on any git failure or an
/// empty result — the same load-bearing reasoning as `tree_hash`.
pub fn rbvcep_commit_sha(dir: &Path, cwd: &Path) -> String {
    let dir_str = rbvcep_as_str(dir);
    let got = rbvcer_run::rbvcer_capture("git", &["-C", &dir_str, "rev-parse", "HEAD"], cwd);
    if got.code != 0 {
        crate::rbvcel_fatal!("failed to read the commit SHA of {}:\n{}", dir.display(), got.stderr.trim());
    }
    let sha = got.stdout.trim().to_string();
    if sha.is_empty() {
        crate::rbvcel_fatal!("git rev-parse HEAD in {} returned empty", dir.display());
    }
    sha
}

/// List `url`'s refs as (sha, refname) pairs — the authenticated read the
/// narthex and kerygma gates use (RBSHD "Gate the narthex" /
/// "Preview"; RBSHO "Re-assert the ground" / "The kerygma" / "Promotion").
/// Fatal on a non-zero exit: the narthex is expected to already exist
/// (RELEASE.md: "create it empty and private... before the cut"), so an
/// unreachable remote is a real deficit, not the fresh-and-empty case — an
/// existing empty repository exits 0 with empty stdout, which returns an
/// empty Vec here, not a fatal.
///
/// The bare symbolic `HEAD` pseudo-ref every non-empty repository also emits
/// is dropped: it never carries information a `refs/...` entry doesn't
/// already carry at the same SHA, and every caller here counts or matches
/// real branch refs — counting it in would make "exactly one real branch"
/// unreachable.
pub fn rbvcep_ls_remote(url: &str, cwd: &Path) -> Vec<(String, String)> {
    let got = rbvcer_run::rbvcer_capture("git", &["ls-remote", url], cwd);
    if got.code != 0 {
        crate::rbvcel_fatal!("git ls-remote {} failed:\n{}", url, got.stderr.trim());
    }
    got.stdout
        .lines()
        .filter_map(|line| line.split_once('\t'))
        .map(|(sha, refname)| (sha.trim().to_string(), refname.trim().to_string()))
        .filter(|(_, refname)| refname != "HEAD")
        .collect()
}
