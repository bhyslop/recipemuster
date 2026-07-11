// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Plain git — the MVP farrier kind: a stateless driver over the `git` binary,
//! implementing `jjrfr_FarrierCore` (JJSVF-farrier.adoc). Worktrees ride within
//! this kind on the partition axis; the lock and billet facets are later paces.
//!
//! Classification policy: a git failure that the farrier sheaf names as a known
//! rejection kind (foreign ground, dirty tree, diverged) translates to
//! `jjrfr_Rejection` and is returned. A git failure this driver cannot classify —
//! the farrier sheaf's taxonomy has no bucket variant by design
//! (`jjdk_no_catch_all`) — is a plumbing fault, not a domain rejection, and panics
//! with the raw detail attached rather than being silently mislabeled.

use crate::jjrfr_farrier::{
    jjrfr_CombReport, jjrfr_ConsignLease, jjrfr_Counterfoil, jjrfr_FarrierCore, jjrfr_GleanOutcome,
    jjrfr_Identity, jjrfr_LineOfWork, jjrfr_Rejection, jjrfr_RejectionKind, jjrfr_Seat, jjrfr_SyncState,
};
use std::path::{Path, PathBuf};

/// The plain-git farrier kind. Zero-sized and stateless — every op takes its repo
/// root explicitly (the no-cwd rule); `self` exists only to select this kind at a
/// trait boundary.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct jjrfg_PlainGit;

/// The sole remote this kind addresses. Spec names no multi-remote requirement;
/// widening past "origin" is a future kind concern, not this one's.
const ZJJRFG_REMOTE: &str = "origin";

struct zjjrfg_GitOutput {
    ok: bool,
    stdout: String,
    stderr: String,
}

/// Run `git -C root <args>`, capturing output. A spawn failure (binary missing) or
/// non-UTF-8 output is an environment precondition violation, not a farrier
/// rejection — it panics here rather than posing as a classified outcome. A
/// non-zero exit status is NOT a panic: it is the normal shape of "git said no,"
/// which the caller classifies.
fn zjjrfg_run_git(root: &Path, args: &[&str]) -> zjjrfg_GitOutput {
    let output = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("git spawn failed for -C {} {:?}: {}", root.display(), args, e));
    zjjrfg_GitOutput {
        ok: output.status.success(),
        stdout: String::from_utf8(output.stdout).expect("git stdout must be UTF-8"),
        stderr: String::from_utf8(output.stderr).expect("git stderr must be UTF-8"),
    }
}

/// The panic path for a git failure this driver cannot classify into one of the
/// farrier sheaf's known rejection kinds.
fn zjjrfg_unexpected(op: &str, root: &Path, detail: &str) -> ! {
    panic!("plain-git {} hit an unclassified git failure at {}: {}", op, root.display(), detail.trim())
}

fn zjjrfg_resolve_relative(base: &Path, maybe_relative: &str) -> PathBuf {
    let p = Path::new(maybe_relative);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        base.join(p)
    }
}

/// Minimal canonicalization: strip a trailing `.git` suffix and surrounding
/// whitespace. Scheme/host unification across ssh and https forms of the same
/// upstream is a future kind concern the spec does not detail; this is not that.
fn zjjrfg_canonicalize_upstream(raw: &str) -> String {
    raw.trim().strip_suffix(".git").unwrap_or(raw.trim()).to_string()
}

fn zjjrfg_line_of_work(root: &Path) -> jjrfr_LineOfWork {
    let branch = zjjrfg_run_git(root, &["rev-parse", "--abbrev-ref", "HEAD"]);
    if !branch.ok {
        zjjrfg_unexpected("line_of_work", root, &branch.stderr);
    }
    let name = branch.stdout.trim();
    if name == "HEAD" {
        let sha = zjjrfg_run_git(root, &["rev-parse", "HEAD"]);
        if !sha.ok {
            zjjrfg_unexpected("line_of_work", root, &sha.stderr);
        }
        jjrfr_LineOfWork::Detached(sha.stdout.trim().to_string())
    } else {
        jjrfr_LineOfWork::Branch(name.to_string())
    }
}

/// Git's own stable rejection vocabulary for a non-fast-forward push, plain or
/// lease-guarded — not a guess, the literal tokens git's transport layer emits.
fn zjjrfg_push_rejected(stderr: &str) -> bool {
    stderr.contains("[rejected]") || stderr.contains("stale info") || stderr.contains("non-fast-forward")
}

impl jjrfr_FarrierCore for jjrfg_PlainGit {
    fn jjrfr_identify(&self, probe_path: &Path) -> Result<jjrfr_Identity, jjrfr_Rejection> {
        let top = zjjrfg_run_git(probe_path, &["rev-parse", "--show-toplevel"]);
        if !top.ok {
            return Err(jjrfr_Rejection::jjrfr_new(
                jjrfr_RejectionKind::ForeignGround,
                "identify",
                probe_path,
                top.stderr,
            ));
        }
        let root = PathBuf::from(top.stdout.trim());

        let git_dir = zjjrfg_run_git(&root, &["rev-parse", "--git-dir"]);
        let common_dir = zjjrfg_run_git(&root, &["rev-parse", "--git-common-dir"]);
        if !git_dir.ok || !common_dir.ok {
            zjjrfg_unexpected("identify", &root, &format!("{}{}", git_dir.stderr, common_dir.stderr));
        }
        let seat = if git_dir.stdout.trim() == common_dir.stdout.trim() {
            jjrfr_Seat::Primary
        } else {
            let common_path = zjjrfg_resolve_relative(&root, common_dir.stdout.trim());
            let primary_root = common_path.parent().map(Path::to_path_buf).unwrap_or(common_path);
            jjrfr_Seat::Partition { primary_root }
        };

        let line_of_work = zjjrfg_line_of_work(&root);

        let remote = zjjrfg_run_git(&root, &["remote", "get-url", ZJJRFG_REMOTE]);
        let upstream_key = if remote.ok {
            zjjrfg_canonicalize_upstream(&remote.stdout)
        } else {
            String::new()
        };

        Ok(jjrfr_Identity { root, upstream_key, seat, line_of_work })
    }

    fn jjrfr_comb(&self, root: &Path) -> Result<jjrfr_CombReport, jjrfr_Rejection> {
        let out = zjjrfg_run_git(root, &["status", "--porcelain"]);
        if !out.ok {
            zjjrfg_unexpected("comb", root, &out.stderr);
        }
        let dirty_paths = out
            .stdout
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| PathBuf::from(line.get(3..).unwrap_or("").trim()))
            .collect();
        Ok(jjrfr_CombReport { dirty_paths })
    }

    fn jjrfr_sync_state(&self, root: &Path) -> Result<jjrfr_SyncState, jjrfr_Rejection> {
        let out = zjjrfg_run_git(root, &["rev-list", "--left-right", "--count", "HEAD...@{upstream}"]);
        if !out.ok {
            return Ok(jjrfr_SyncState::Untracked);
        }
        let mut counts = out.stdout.trim().split_whitespace();
        let ahead: u32 = counts
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| zjjrfg_unexpected("sync_state", root, &out.stdout));
        let behind: u32 = counts
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| zjjrfg_unexpected("sync_state", root, &out.stdout));
        Ok(jjrfr_SyncState::Tracking { ahead, behind })
    }

    fn jjrfr_counterfoil(&self, root: &Path) -> Result<jjrfr_Counterfoil, jjrfr_Rejection> {
        let sha = zjjrfg_run_git(root, &["rev-parse", "HEAD"]);
        if !sha.ok {
            zjjrfg_unexpected("counterfoil", root, &sha.stderr);
        }
        let comb = self.jjrfr_comb(root)?;
        let line_of_work = zjjrfg_line_of_work(root);

        let mut members = std::collections::BTreeMap::new();
        // Single-repo is the constellation axis's one-element case; the sole
        // member keys on "." until a multi-member kind needs a real member name.
        members.insert(".".to_string(), sha.stdout.trim().to_string());

        Ok(jjrfr_Counterfoil { members, line_of_work, dirty: !comb.jjrfr_is_clean() })
    }

    fn jjrfr_lodge(&self, root: &Path, files: &[PathBuf], message: &str) -> Result<(), jjrfr_Rejection> {
        let file_strs: Vec<String> = files.iter().map(|p| p.to_string_lossy().into_owned()).collect();

        // A new file is unknown to git until staged — `commit -- <path>` alone
        // rejects it with "pathspec did not match". Staging first, then limiting
        // the commit to the same explicit list, keeps both halves additive: only
        // these paths are staged, only these paths are committed.
        let mut add_args: Vec<&str> = vec!["add", "--"];
        add_args.extend(file_strs.iter().map(String::as_str));
        let add_out = zjjrfg_run_git(root, &add_args);
        if !add_out.ok {
            zjjrfg_unexpected("lodge", root, &add_out.stderr);
        }

        let mut commit_args: Vec<&str> = vec!["commit", "-m", message, "--"];
        commit_args.extend(file_strs.iter().map(String::as_str));
        let out = zjjrfg_run_git(root, &commit_args);
        if !out.ok {
            zjjrfg_unexpected("lodge", root, &out.stderr);
        }
        Ok(())
    }

    fn jjrfr_glean(&self, root: &Path) -> jjrfr_GleanOutcome {
        let out = zjjrfg_run_git(root, &["fetch", ZJJRFG_REMOTE]);
        if out.ok {
            jjrfr_GleanOutcome::Updated
        } else {
            jjrfr_GleanOutcome::Unreachable
        }
    }

    fn jjrfr_advance(&self, root: &Path, remote_ref: &str) -> Result<(), jjrfr_Rejection> {
        let comb = self.jjrfr_comb(root)?;
        if !comb.jjrfr_is_clean() {
            return Err(jjrfr_Rejection::jjrfr_new(
                jjrfr_RejectionKind::DirtyTree,
                "advance",
                root,
                "uncommitted changes block a fast-forward move",
            ));
        }
        let out = zjjrfg_run_git(root, &["merge", "--ff-only", remote_ref]);
        if !out.ok {
            return Err(jjrfr_Rejection::jjrfr_new(jjrfr_RejectionKind::Diverged, "advance", root, out.stderr));
        }
        Ok(())
    }

    fn jjrfr_consign(&self, root: &Path, branch: &str, lease: Option<&jjrfr_ConsignLease>) -> Result<(), jjrfr_Rejection> {
        let refspec = format!("{}:{}", branch, branch);
        let lease_flag;
        let mut args: Vec<&str> = vec!["push"];
        if let Some(jjrfr_ConsignLease(expected_sha)) = lease {
            lease_flag = format!("--force-with-lease={}:{}", branch, expected_sha);
            args.push(&lease_flag);
        }
        args.push(ZJJRFG_REMOTE);
        args.push(&refspec);

        let out = zjjrfg_run_git(root, &args);
        if out.ok {
            return Ok(());
        }
        if zjjrfg_push_rejected(&out.stderr) {
            return Err(jjrfr_Rejection::jjrfr_new(jjrfr_RejectionKind::Diverged, "consign", root, out.stderr));
        }
        zjjrfg_unexpected("consign", root, &out.stderr)
    }
}
