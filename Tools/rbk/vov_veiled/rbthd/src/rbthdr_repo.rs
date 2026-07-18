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
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTHDR — repository anchoring, the fixed conventional sibling locations, and
// the dispose-by-rename discipline (RBSHE step 3, "retire-aside").
//
// Every path the lap touches is derived from the maintainer repository root,
// never hand-typed, so the artifacts always land beside the tree the lap was
// launched from.

use std::path::{Path, PathBuf};

use crate::rbthdr_run;

/// The fixed candidate parent directory, a sibling of the maintainer repo. Not
/// dated (unlike RELEASE.md's operator-hand `rbm_candidate_{date}_{try}`): essai
/// runs the lap repeatedly at one conventional location and retires any prior
/// aside, so the location is memorable and the disposal is safe. Expede builds
/// the clone one level down, at {parent}/{RBTHDR_CANDIDATE_SUBDIR}.
pub const RBTHDR_CANDIDATE_DIRNAME: &str = "rbm_candidate";

/// The clone subdir the cut creates beneath its target dir (rbthdr_expede
/// builds there; the lap and the rig stage find what it built through this
/// one name).
pub const RBTHDR_CANDIDATE_SUBDIR: &str = "candidate";

/// The identity-free station tree the candidate gets, a sibling of the clone
/// (RELEASE.md step 5 / rbk-expede): {parent}/station-files/{burs.env,secrets/}.
pub const RBTHDR_STATION_SUBDIR: &str = "station-files";
pub const RBTHDR_STATION_FILE: &str = "burs.env";
pub const RBTHDR_SECRETS_SUBDIR: &str = "secrets";
pub const RBTHDR_LOGS_SUBDIR: &str = "logs-buk";

/// The fixed parent directory for the freshness matcher's scratch re-cut
/// (RBSHD step 2 / RBSHO step 2, RBSHC single-matcher rule), a sibling of the
/// maintainer repo distinct from RBTHDR_CANDIDATE_DIRNAME so the freshness
/// check can never collide with, or dispose of, the standing candidate it is
/// comparing against.
pub const RBTHDR_FRESHNESS_DIRNAME: &str = "rbm_candidate_freshness";

/// The maintainer repository root, from git. Fatal if not in a repo, empty, or
/// non-absolute — every derived path anchors on it, and the retire-aside guards
/// compare against it.
pub fn toplevel() -> PathBuf {
    let cwd = std::env::current_dir()
        .unwrap_or_else(|e| crate::rbthdr_fatal!("cannot read working directory: {}", e));
    let got = rbthdr_run::capture("git", &["rev-parse", "--show-toplevel"], &cwd);
    if got.code != 0 {
        crate::rbthdr_fatal!(
            "not inside a git repository — essai must run from the maintainer tree: {}",
            got.stderr.trim()
        );
    }
    let top = got.stdout.trim();
    if top.is_empty() {
        crate::rbthdr_fatal!("git returned an empty repository root");
    }
    let top = PathBuf::from(top);
    if !top.is_absolute() {
        crate::rbthdr_fatal!("repository root is not an absolute path: {}", top.display());
    }
    top
}

/// A path as an owned &str, fatal on non-UTF-8. Every path the lap handles is
/// derived from git output or a repo-relative join, so non-UTF-8 is a real fault
/// worth stopping on, not something to lossily paper over into a subprocess arg.
pub fn as_str(p: &Path) -> String {
    p.to_str()
        .unwrap_or_else(|| crate::rbthdr_fatal!("path is not valid UTF-8: {}", p.display()))
        .to_string()
}

/// The parent directory the sibling artifacts land in. Fatal if the root has no
/// parent.
pub fn parent(top: &Path) -> PathBuf {
    top.parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| crate::rbthdr_fatal!("repository root has no parent: {}", top.display()))
}

/// Dispose of a directory by RENAME to a timestamped sibling — never a delete
/// (RBSHE step 3, the dispose-by-rename discipline the rig proves). Absent
/// target is a no-op. The caller asserts the target's identity (fixed basename,
/// not the repo root) before calling; this only refuses to clobber an existing
/// retirement sibling. Returns whether anything was retired.
pub fn retire_aside(dir: &Path, cwd: &Path) -> bool {
    if !dir.exists() {
        return false;
    }
    let base = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| crate::rbthdr_fatal!("cannot read basename of {}", dir.display()));
    let stamp = rbthdr_run::timestamp(cwd);
    let retired = dir.with_file_name(format!("{}.retired-{}", base, stamp));
    if retired.exists() {
        crate::rbthdr_fatal!("retirement target already exists: {}", retired.display());
    }
    std::fs::rename(dir, &retired).unwrap_or_else(|e| {
        crate::rbthdr_fatal!(
            "failed to retire {} -> {}: {}",
            dir.display(),
            retired.display(),
            e
        )
    });
    crate::rbthdr_log::line(&format!("retired prior aside: {}", retired.display()));
    true
}

/// Assert a target directory is safe to move aside wholesale: its basename is
/// exactly the fixed name, and it is not the maintainer repo root. Mirrors the
/// rig-move guard in rblm_harbinger.sh — the disposal is a rename, so even a
/// wrong guard destroys nothing, but the guard makes a wrong move impossible.
pub fn guard_disposable(dir: &Path, expected_basename: &str, top: &Path) {
    let base = dir.file_name().and_then(|n| n.to_str());
    if base != Some(expected_basename) {
        crate::rbthdr_fatal!(
            "refusing to dispose {} — its basename is not '{}'",
            dir.display(),
            expected_basename
        );
    }
    if dir == top {
        crate::rbthdr_fatal!(
            "refusing to dispose {} — it is the maintainer repository root",
            dir.display()
        );
    }
}

/// The tree hash of `dir`'s HEAD — the byte-identity a cachet is keyed on and
/// the freshness matcher compares (RBS0 rbth_cachet, RBSHD/RBSHO). Fatal on
/// any git failure or an empty result: a cachet or a freshness verdict keyed
/// on an empty string would silently compare equal to nothing.
pub fn tree_hash(dir: &Path, cwd: &Path) -> String {
    let dir_str = as_str(dir);
    let got = rbthdr_run::capture("git", &["-C", &dir_str, "rev-parse", "HEAD^{tree}"], cwd);
    if got.code != 0 {
        crate::rbthdr_fatal!("failed to read the tree hash of {}:\n{}", dir.display(), got.stderr.trim());
    }
    let hash = got.stdout.trim().to_string();
    if hash.is_empty() {
        crate::rbthdr_fatal!("git rev-parse HEAD^{{tree}} in {} returned empty", dir.display());
    }
    hash
}

/// The commit SHA of `dir`'s HEAD (the tip). Fatal on any git failure or an
/// empty result — the same load-bearing reasoning as `tree_hash`.
pub fn commit_sha(dir: &Path, cwd: &Path) -> String {
    let dir_str = as_str(dir);
    let got = rbthdr_run::capture("git", &["-C", &dir_str, "rev-parse", "HEAD"], cwd);
    if got.code != 0 {
        crate::rbthdr_fatal!("failed to read the commit SHA of {}:\n{}", dir.display(), got.stderr.trim());
    }
    let sha = got.stdout.trim().to_string();
    if sha.is_empty() {
        crate::rbthdr_fatal!("git rev-parse HEAD in {} returned empty", dir.display());
    }
    sha
}

/// List `url`'s refs as (sha, refname) pairs — the authenticated read the
/// quarantine and disclosure gates use (RBSHD "Gate the quarantine" /
/// "Preview"; RBSHO "Re-assert the ground" / "The disclosure" / "Promotion").
/// Fatal on a non-zero exit: the quarantine is expected to already exist
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
pub fn ls_remote(url: &str, cwd: &Path) -> Vec<(String, String)> {
    let got = rbthdr_run::capture("git", &["ls-remote", url], cwd);
    if got.code != 0 {
        crate::rbthdr_fatal!("git ls-remote {} failed:\n{}", url, got.stderr.trim());
    }
    got.stdout
        .lines()
        .filter_map(|line| line.split_once('\t'))
        .map(|(sha, refname)| (sha.trim().to_string(), refname.trim().to_string()))
        .filter(|(_, refname)| refname != "HEAD")
        .collect()
}
