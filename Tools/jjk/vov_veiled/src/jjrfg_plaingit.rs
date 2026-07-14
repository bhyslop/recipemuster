// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Plain git — the MVP farrier kind: a stateless driver over the `git` binary,
//! implementing `jjrfr_FarrierCore`, `jjrfr_FarrierLock`, and `jjrfr_FarrierBillet`
//! (JJSVF-farrier.adoc). Worktrees ride within this kind on the partition axis.
//!
//! Classification policy: a git failure that the farrier sheaf names as a known
//! rejection kind (foreign ground, dirty tree, diverged, lock-held, lock-broken)
//! translates to `jjrfr_Rejection` and is returned. A git failure this driver
//! cannot classify is a plumbing fault, not a domain rejection — the taxonomy is
//! closed by the sheaf, with no catch-all variant to hide in — so it panics with
//! the raw detail attached rather than being silently mislabeled under a familiar
//! kind. A merge conflict at `jjrfr_enfold` is one such case: it is not a named
//! rejection kind, so it panics and leaves the conflict markers standing for the
//! attended session, per the billet sheaf's "resolution belonging to the attended
//! session."

use crate::jjrfr_farrier::{
    jjrfr_BilletBirth,
    jjrfr_CombReport,
    jjrfr_ConsignLease,
    jjrfr_Counterfoil,
    jjrfr_FarrierBillet,
    jjrfr_FarrierCore,
    jjrfr_FarrierLock,
    jjrfr_GleanOutcome,
    jjrfr_Identity,
    jjrfr_LineOfWork,
    jjrfr_Rejection,
    jjrfr_RejectionKind,
    jjrfr_Seat,
    jjrfr_SyncState,
};
use std::io::Write;
use std::path::{
    Path,
    PathBuf,
};
use std::process::Stdio;

/// The plain-git farrier kind. Zero-sized and stateless — every op takes its repo
/// root explicitly (the no-cwd rule); `self` exists only to select this kind at a
/// trait boundary.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct jjrfg_PlainGit;

/// The sole remote this kind addresses. Spec names no multi-remote requirement;
/// widening past "origin" is a future kind concern, not this one's.
const ZJJRFG_REMOTE: &str = "origin";

/// The blotter's one well-known lock ref (`jjdb_blotter`, blotter sheaf):
/// `refs/jjv/*` is reserved to JJ entire, and the guidon is its sole resident.
const ZJJRFG_GUIDON_REF: &str = "refs/jjv/guidon";

/// The op tags carried in rejection and panic context — one const per op, so
/// every failure site of an op names it identically.
const ZJJRFG_OP_IDENTIFY: &str = "identify";
const ZJJRFG_OP_COMB: &str = "comb";
const ZJJRFG_OP_SYNC_STATE: &str = "sync_state";
const ZJJRFG_OP_COUNTERFOIL: &str = "counterfoil";
const ZJJRFG_OP_LODGE: &str = "lodge";
const ZJJRFG_OP_ADVANCE: &str = "advance";
const ZJJRFG_OP_CONSIGN: &str = "consign";
const ZJJRFG_OP_LINE_OF_WORK: &str = "line_of_work";
const ZJJRFG_OP_STAKE: &str = "stake";
const ZJJRFG_OP_PLUCK: &str = "pluck";
const ZJJRFG_OP_SIGHT: &str = "sight";
const ZJJRFG_OP_BILLET_CREATE: &str = "billet_create";
const ZJJRFG_OP_BILLET_SEAT: &str = "billet_seat";
const ZJJRFG_OP_BILLET_DETACH: &str = "billet_detach";
const ZJJRFG_OP_BILLET_REMOVE: &str = "billet_remove";
const ZJJRFG_OP_LINE_EXISTS: &str = "line_exists";
const ZJJRFG_OP_OUTSTRIPPED: &str = "outstripped";
const ZJJRFG_OP_ENFOLD: &str = "enfold";
const ZJJRFG_OP_PRIMARY_ROOT: &str = "primary_root";

struct zjjrfg_GitOutput {
    ok: bool,
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

impl zjjrfg_GitOutput {
    /// Everything git said, for the panic path. Git does not keep its refusals on
    /// one stream: `commit` with nothing staged exits non-zero and explains itself
    /// on STDOUT, so a detail rendered from stderr alone reports an empty reason
    /// for it — which is exactly what an unclassified failure cannot afford, being
    /// the one path where the driver has nothing else left to say. It says all of
    /// it, and lets the reader judge.
    fn zjjrfg_detail(&self) -> String {
        format!(
            "exit {} | stderr: {} | stdout: {}",
            self.code.map(|c| c.to_string()).unwrap_or_else(|| "killed by signal".to_string()),
            self.stderr.trim(),
            self.stdout.trim()
        )
    }
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
        code: output.status.code(),
        stdout: String::from_utf8(output.stdout).expect("git stdout must be UTF-8"),
        stderr: String::from_utf8(output.stderr).expect("git stderr must be UTF-8"),
    }
}

/// The panic path for a git failure this driver cannot classify into one of the
/// farrier sheaf's known rejection kinds.
fn zjjrfg_unexpected(op: &str, root: &Path, detail: &str) -> ! {
    panic!("plain-git {} hit an unclassified git failure at {}: {}", op, root.display(), detail.trim())
}

/// Run `git -C root <args>`, feeding `stdin_data` to the child's stdin. Spawn or
/// pipe-write failure panics, matching `zjjrfg_run_git`'s precondition posture.
fn zjjrfg_run_git_with_stdin(root: &Path, args: &[&str], stdin_data: &str) -> zjjrfg_GitOutput {
    let mut child = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("git spawn failed for -C {} {:?}: {}", root.display(), args, e));
    child
        .stdin
        .take()
        .expect("piped stdin must be present on a freshly spawned child")
        .write_all(stdin_data.as_bytes())
        .unwrap_or_else(|e| panic!("git stdin write failed for -C {} {:?}: {}", root.display(), args, e));
    let output = child
        .wait_with_output()
        .unwrap_or_else(|e| panic!("git wait failed for -C {} {:?}: {}", root.display(), args, e));
    zjjrfg_GitOutput {
        ok: output.status.success(),
        code: output.status.code(),
        stdout: String::from_utf8(output.stdout).expect("git stdout must be UTF-8"),
        stderr: String::from_utf8(output.stderr).expect("git stderr must be UTF-8"),
    }
}

/// The one composer of a trunk branch's remote-counterpart ref: fully qualified
/// so a perverse local branch literally named `origin/<trunk>` cannot shadow it
/// (the enfold contract, farrier sheaf).
fn zjjrfg_counterpart(trunk: &str) -> String {
    format!("refs/remotes/{}/{}", ZJJRFG_REMOTE, trunk)
}

/// Compute a blob's object id for `content` under `root`'s object database.
/// `write` persists the blob (the stake path, which then pushes it); omitted, this
/// is a pure hash computation (the pluck path, comparing an observed guidon's
/// content against the lease it expects to find on the remote).
fn zjjrfg_hash_object(root: &Path, content: &str, write: bool, op: &'static str) -> String {
    let mut args: Vec<&str> = vec!["hash-object"];
    if write {
        args.push("-w");
    }
    args.push("--stdin");
    let out = zjjrfg_run_git_with_stdin(root, &args, content);
    if !out.ok {
        zjjrfg_unexpected(op, root, &out.zjjrfg_detail());
    }
    out.stdout.trim().to_string()
}

/// The one composer of git's push-lease flag: `target_ref:expected`, where an
/// empty expected value is git's own spelling of "must not exist" (the stake
/// create form).
fn zjjrfg_lease_flag(target_ref: &str, expected: &str) -> String {
    format!("--force-with-lease={}:{}", target_ref, expected)
}

/// Classify a tree's seat from its git metadata: git-dir equal to git-common-dir
/// means primary; differing means a partition, whose primary root derives from
/// the common dir's parent.
fn zjjrfg_seat(root: &Path, op: &'static str) -> jjrfr_Seat {
    let git_dir = zjjrfg_run_git(root, &["rev-parse", "--git-dir"]);
    let common_dir = zjjrfg_run_git(root, &["rev-parse", "--git-common-dir"]);
    if !git_dir.ok || !common_dir.ok {
        zjjrfg_unexpected(op, root, &format!("{} | {}", git_dir.zjjrfg_detail(), common_dir.zjjrfg_detail()));
    }
    if git_dir.stdout.trim() == common_dir.stdout.trim() {
        jjrfr_Seat::Primary
    } else {
        let common_path = zjjrfg_resolve_relative(root, common_dir.stdout.trim());
        let primary_root = common_path.parent().map(Path::to_path_buf).unwrap_or(common_path);
        jjrfr_Seat::Partition { primary_root }
    }
}

/// Resolve a partition's primary root from its own git metadata alone — the
/// billet facet's ops take only the billet's own root, never the primary's, so
/// `enfold` and `billet_remove` derive it themselves rather than requiring the
/// caller to carry it. Panics (unclassified) if `root` is not itself a
/// partition, since operating billet ops on a primary root is a caller-contract
/// violation, not a named rejection kind.
fn zjjrfg_primary_root(root: &Path) -> PathBuf {
    match zjjrfg_seat(root, ZJJRFG_OP_PRIMARY_ROOT) {
        jjrfr_Seat::Partition { primary_root } => primary_root,
        jjrfr_Seat::Primary => zjjrfg_unexpected(ZJJRFG_OP_PRIMARY_ROOT, root, "root is a primary, not a partition"),
    }
}

pub(crate) fn zjjrfg_resolve_relative(base: &Path, maybe_relative: &str) -> PathBuf {
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
pub(crate) fn zjjrfg_canonicalize_upstream(raw: &str) -> String {
    raw.trim().strip_suffix(".git").unwrap_or(raw.trim()).to_string()
}

fn zjjrfg_line_of_work(root: &Path) -> jjrfr_LineOfWork {
    let branch = zjjrfg_run_git(root, &["rev-parse", "--abbrev-ref", "HEAD"]);
    if !branch.ok {
        zjjrfg_unexpected(ZJJRFG_OP_LINE_OF_WORK, root, &branch.zjjrfg_detail());
    }
    let name = branch.stdout.trim();
    if name == "HEAD" {
        let sha = zjjrfg_run_git(root, &["rev-parse", "HEAD"]);
        if !sha.ok {
            zjjrfg_unexpected(ZJJRFG_OP_LINE_OF_WORK, root, &sha.zjjrfg_detail());
        }
        jjrfr_LineOfWork::Detached(sha.stdout.trim().to_string())
    } else {
        jjrfr_LineOfWork::Branch(name.to_string())
    }
}

/// Git's own stable rejection vocabulary for a non-fast-forward push, plain or
/// lease-guarded — not a guess, the literal tokens git's transport layer emits.
pub(crate) fn zjjrfg_push_rejected(stderr: &str) -> bool {
    stderr.contains("[rejected]") || stderr.contains("stale info") || stderr.contains("non-fast-forward")
}

impl jjrfr_FarrierCore for jjrfg_PlainGit {
    fn jjrfr_identify(&self, probe_path: &Path) -> Result<jjrfr_Identity, jjrfr_Rejection> {
        let top = zjjrfg_run_git(probe_path, &["rev-parse", "--show-toplevel"]);
        if !top.ok {
            return Err(jjrfr_Rejection::jjrfr_new(
                jjrfr_RejectionKind::ForeignGround,
                ZJJRFG_OP_IDENTIFY,
                probe_path,
                top.stderr,
            ));
        }
        let root = PathBuf::from(top.stdout.trim());

        let seat = zjjrfg_seat(&root, ZJJRFG_OP_IDENTIFY);
        let line_of_work = zjjrfg_line_of_work(&root);

        let remote = zjjrfg_run_git(&root, &["remote", "get-url", ZJJRFG_REMOTE]);
        let upstream_key = if remote.ok {
            Some(zjjrfg_canonicalize_upstream(&remote.stdout))
        } else {
            None
        };

        Ok(jjrfr_Identity { root, upstream_key, seat, line_of_work })
    }

    fn jjrfr_comb(&self, root: &Path) -> Result<jjrfr_CombReport, jjrfr_Rejection> {
        let out = zjjrfg_run_git(root, &["status", "--porcelain"]);
        if !out.ok {
            zjjrfg_unexpected(ZJJRFG_OP_COMB, root, &out.zjjrfg_detail());
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
            .unwrap_or_else(|| zjjrfg_unexpected(ZJJRFG_OP_SYNC_STATE, root, &out.stdout));
        let behind: u32 = counts
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or_else(|| zjjrfg_unexpected(ZJJRFG_OP_SYNC_STATE, root, &out.stdout));
        Ok(jjrfr_SyncState::Tracking { ahead, behind })
    }

    fn jjrfr_counterfoil(&self, root: &Path) -> Result<jjrfr_Counterfoil, jjrfr_Rejection> {
        let sha = zjjrfg_run_git(root, &["rev-parse", "HEAD"]);
        if !sha.ok {
            zjjrfg_unexpected(ZJJRFG_OP_COUNTERFOIL, root, &sha.zjjrfg_detail());
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
            zjjrfg_unexpected(ZJJRFG_OP_LODGE, root, &add_out.zjjrfg_detail());
        }

        let mut commit_args: Vec<&str> = vec!["commit", "-m", message, "--"];
        commit_args.extend(file_strs.iter().map(String::as_str));
        let out = zjjrfg_run_git(root, &commit_args);
        if !out.ok {
            zjjrfg_unexpected(ZJJRFG_OP_LODGE, root, &out.zjjrfg_detail());
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

    fn jjrfr_advance(&self, root: &Path) -> Result<(), jjrfr_Rejection> {
        // JJr_b52
        //
        // Advancing a line with no remote counterpart is a composition fault —
        // sync_state's Untracked answer is the guard callers consult first.
        let upstream = zjjrfg_run_git(root, &["rev-parse", "--abbrev-ref", "@{upstream}"]);
        if !upstream.ok {
            zjjrfg_unexpected(ZJJRFG_OP_ADVANCE, root, &upstream.zjjrfg_detail());
        }
        let out = zjjrfg_run_git(root, &["reset", "--hard", upstream.stdout.trim()]);
        if !out.ok {
            zjjrfg_unexpected(ZJJRFG_OP_ADVANCE, root, &out.zjjrfg_detail());
        }
        Ok(())
    }

    fn jjrfr_consign(&self, root: &Path, branch: &str, lease: Option<&jjrfr_ConsignLease>) -> Result<(), jjrfr_Rejection> {
        let refspec = format!("{}:{}", branch, branch);
        let out = match lease {
            Some(jjrfr_ConsignLease(guidon)) => {
                // JJr_d81
                // Atomic two-ref push: the content branch (plain, still
                // fast-forward-protected) plus a same-value update of the
                // guidon ref under a lease on the held guidon's blob. While
                // the lock is ours the guidon update is an up-to-date no-op;
                // a lock broken under us fails its lease, and --atomic pulls
                // the content update down with it.
                let blob_sha = zjjrfg_hash_object(root, guidon, true, ZJJRFG_OP_CONSIGN);
                let lease_flag = zjjrfg_lease_flag(ZJJRFG_GUIDON_REF, &blob_sha);
                let guidon_refspec = format!("{}:{}", blob_sha, ZJJRFG_GUIDON_REF);
                zjjrfg_run_git(root, &["push", "--atomic", &lease_flag, ZJJRFG_REMOTE, &refspec, &guidon_refspec])
            }
            None => zjjrfg_run_git(root, &["push", ZJJRFG_REMOTE, &refspec]),
        };
        if out.ok {
            return Ok(());
        }
        if zjjrfg_push_rejected(&out.stderr) {
            // A rejection naming the guidon ref is the lock broken under the
            // holder; one naming only the branch is a plain content race.
            let kind = if out.stderr.contains(ZJJRFG_GUIDON_REF) {
                jjrfr_RejectionKind::LockBroken
            } else {
                jjrfr_RejectionKind::Diverged
            };
            return Err(jjrfr_Rejection::jjrfr_new(kind, ZJJRFG_OP_CONSIGN, root, out.stderr));
        }
        zjjrfg_unexpected(ZJJRFG_OP_CONSIGN, root, &out.zjjrfg_detail())
    }
}

impl jjrfr_FarrierLock for jjrfg_PlainGit {
    fn jjrfr_stake(&self, root: &Path, guidon: &str) -> Result<(), jjrfr_Rejection> {
        let blob_sha = zjjrfg_hash_object(root, guidon, true, ZJJRFG_OP_STAKE);
        let refspec = format!("{}:{}", blob_sha, ZJJRFG_GUIDON_REF);
        let lease = zjjrfg_lease_flag(ZJJRFG_GUIDON_REF, "");
        let out = zjjrfg_run_git(root, &["push", ZJJRFG_REMOTE, &lease, &refspec]);
        if out.ok {
            return Ok(());
        }
        if zjjrfg_push_rejected(&out.stderr) {
            return Err(jjrfr_Rejection::jjrfr_new(jjrfr_RejectionKind::LockHeld, ZJJRFG_OP_STAKE, root, out.stderr));
        }
        zjjrfg_unexpected(ZJJRFG_OP_STAKE, root, &out.zjjrfg_detail())
    }

    fn jjrfr_pluck(&self, root: &Path, observed_guidon: &str) -> Result<(), jjrfr_Rejection> {
        let expected_sha = zjjrfg_hash_object(root, observed_guidon, false, ZJJRFG_OP_PLUCK);
        let lease = zjjrfg_lease_flag(ZJJRFG_GUIDON_REF, &expected_sha);
        let refspec = format!(":{}", ZJJRFG_GUIDON_REF);
        let out = zjjrfg_run_git(root, &["push", ZJJRFG_REMOTE, &lease, &refspec]);
        if out.ok {
            return Ok(());
        }
        if zjjrfg_push_rejected(&out.stderr) {
            return Err(jjrfr_Rejection::jjrfr_new(jjrfr_RejectionKind::LockBroken, ZJJRFG_OP_PLUCK, root, out.stderr));
        }
        zjjrfg_unexpected(ZJJRFG_OP_PLUCK, root, &out.zjjrfg_detail())
    }

    fn jjrfr_sight(&self, root: &Path) -> Result<Option<String>, jjrfr_Rejection> {
        let ls = zjjrfg_run_git(root, &["ls-remote", ZJJRFG_REMOTE, ZJJRFG_GUIDON_REF]);
        if !ls.ok {
            zjjrfg_unexpected(ZJJRFG_OP_SIGHT, root, &ls.zjjrfg_detail());
        }
        let sha = match ls.stdout.lines().next().and_then(|line| line.split_whitespace().next()) {
            Some(sha) => sha.to_string(),
            None => return Ok(None),
        };

        // ls-remote reports the guidon ref's SHA, not its content — fetch the
        // blob into the local object database (no local ref created) so
        // cat-file can read what it actually says.
        let fetch = zjjrfg_run_git(root, &["fetch", ZJJRFG_REMOTE, ZJJRFG_GUIDON_REF]);
        if !fetch.ok {
            zjjrfg_unexpected(ZJJRFG_OP_SIGHT, root, &fetch.zjjrfg_detail());
        }
        let content = zjjrfg_run_git(root, &["cat-file", "-p", &sha]);
        if !content.ok {
            zjjrfg_unexpected(ZJJRFG_OP_SIGHT, root, &content.zjjrfg_detail());
        }
        Ok(Some(content.stdout))
    }
}

impl jjrfr_FarrierBillet for jjrfg_PlainGit {
    fn jjrfr_billet_create(&self, root: &Path, birth: &jjrfr_BilletBirth, billet_root: &Path, trunk: &str) -> Result<(), jjrfr_Rejection> {
        let billet_str = billet_root.to_string_lossy().into_owned();
        // Both birth forms anchor at trunk's remote counterpart, never the
        // primary's own checkout (jjrfr_BilletBirth's no-exfiltration posture).
        // One canonical form per birth kind, never branching on whether the
        // name happens to already exist: a branch-name collision is a
        // caller-contract violation, not a case this op tolerates silently —
        // it surfaces as git's own unclassified failure. A missing counterpart
        // (never gleaned, or no such trunk on the remote) likewise fails loud.
        let counterpart = zjjrfg_counterpart(trunk);
        let out = match birth {
            jjrfr_BilletBirth::Branch(name) => {
                zjjrfg_run_git(root, &["worktree", "add", "-q", &billet_str, "-b", name, &counterpart])
            }
            jjrfr_BilletBirth::Detached => {
                zjjrfg_run_git(root, &["worktree", "add", "-q", "--detach", &billet_str, &counterpart])
            }
        };
        if !out.ok {
            zjjrfg_unexpected(ZJJRFG_OP_BILLET_CREATE, root, &out.zjjrfg_detail());
        }
        Ok(())
    }

    fn jjrfr_billet_seat(&self, root: &Path, branch: &str, billet_root: &Path) -> Result<(), jjrfr_Rejection> {
        let billet_str = billet_root.to_string_lossy().into_owned();
        // Seat the existing durable branch as-is: no anchoring, no reset — the
        // branch carries its own WIP history across chats (dispatch sheaf:
        // "billets are chat-ephemeral; branches are durable"). Git itself
        // rejects a branch already checked out in another worktree; that and a
        // missing branch both fail loud as caller-contract violations (the
        // spine consults jjrfr_line_exists first).
        let out = zjjrfg_run_git(root, &["worktree", "add", "-q", &billet_str, branch]);
        if !out.ok {
            zjjrfg_unexpected(ZJJRFG_OP_BILLET_SEAT, root, &out.zjjrfg_detail());
        }
        Ok(())
    }

    fn jjrfr_billet_detach(&self, billet_root: &Path, trunk: &str) -> Result<(), jjrfr_Rejection> {
        let comb = self.jjrfr_comb(billet_root)?;
        if !comb.jjrfr_is_clean() {
            return Err(jjrfr_Rejection::jjrfr_new(
                jjrfr_RejectionKind::DirtyTree,
                ZJJRFG_OP_BILLET_DETACH,
                billet_root,
                "uncommitted changes block re-detaching the billet",
            ));
        }
        let counterpart = zjjrfg_counterpart(trunk);
        let out = zjjrfg_run_git(billet_root, &["checkout", "-q", "--detach", &counterpart]);
        if !out.ok {
            zjjrfg_unexpected(ZJJRFG_OP_BILLET_DETACH, billet_root, &out.zjjrfg_detail());
        }
        Ok(())
    }

    fn jjrfr_line_exists(&self, root: &Path, branch: &str) -> Result<bool, jjrfr_Rejection> {
        let full_ref = format!("refs/heads/{}", branch);
        let out = zjjrfg_run_git(root, &["show-ref", "--verify", "--quiet", &full_ref]);
        match out.code {
            Some(0) => Ok(true),
            Some(1) => Ok(false),
            _ => zjjrfg_unexpected(ZJJRFG_OP_LINE_EXISTS, root, &out.zjjrfg_detail()),
        }
    }

    fn jjrfr_outstripped(&self, billet_root: &Path, trunk: &str) -> Result<bool, jjrfr_Rejection> {
        let counterpart = zjjrfg_counterpart(trunk);
        // No counterpart known locally → not outstripped: nothing observed can
        // be ahead, and the staleness warning this feeds must not cry on
        // ignorance (trait contract).
        let seen = zjjrfg_run_git(billet_root, &["rev-parse", "--verify", "--quiet", &counterpart]);
        if !seen.ok {
            return Ok(false);
        }
        // Ancestry, not ahead/behind counts: exit 0 means the counterpart is
        // already contained in the billet's position, 1 means trunk holds work
        // the billet lacks. Any other status is an unclassified failure.
        let out = zjjrfg_run_git(billet_root, &["merge-base", "--is-ancestor", &counterpart, "HEAD"]);
        match out.code {
            Some(0) => Ok(false),
            Some(1) => Ok(true),
            _ => zjjrfg_unexpected(ZJJRFG_OP_OUTSTRIPPED, billet_root, &out.zjjrfg_detail()),
        }
    }

    fn jjrfr_billet_remove(&self, billet_root: &Path) -> Result<(), jjrfr_Rejection> {
        let comb = self.jjrfr_comb(billet_root)?;
        if !comb.jjrfr_is_clean() {
            return Err(jjrfr_Rejection::jjrfr_new(
                jjrfr_RejectionKind::DirtyTree,
                ZJJRFG_OP_BILLET_REMOVE,
                billet_root,
                "uncommitted changes block reaping the billet",
            ));
        }
        let primary_root = zjjrfg_primary_root(billet_root);
        let billet_str = billet_root.to_string_lossy().into_owned();
        let out = zjjrfg_run_git(&primary_root, &["worktree", "remove", &billet_str]);
        if !out.ok {
            zjjrfg_unexpected(ZJJRFG_OP_BILLET_REMOVE, billet_root, &out.zjjrfg_detail());
        }
        Ok(())
    }

    fn jjrfr_enfold(&self, billet_root: &Path, trunk: &str) -> Result<(), jjrfr_Rejection> {
        let comb = self.jjrfr_comb(billet_root)?;
        if !comb.jjrfr_is_clean() {
            return Err(jjrfr_Rejection::jjrfr_new(
                jjrfr_RejectionKind::DirtyTree,
                ZJJRFG_OP_ENFOLD,
                billet_root,
                "uncommitted changes block merging trunk in",
            ));
        }
        // The counterpart of the caller-named trunk, never the local ref of that
        // name: merging the local ref would push the operator's unpushed trunk
        // work out as billet ancestry at the next consign (the enfold contract,
        // farrier sheaf).
        let counterpart = zjjrfg_counterpart(trunk);
        // Never rebase: a plain merge, fast-forwarding when possible and
        // otherwise recording a real merge commit. A conflict is not one of the
        // taxonomy's rejection kinds — it is not this driver's to resolve
        // (billet sheaf: "resolution belonging to the attended session") — so it
        // falls through to the unclassified panic, leaving the conflict markers
        // standing exactly as git left them.
        let out = zjjrfg_run_git(billet_root, &["merge", "-q", &counterpart, "-m", "enfold trunk"]);
        if !out.ok {
            zjjrfg_unexpected(ZJJRFG_OP_ENFOLD, billet_root, &out.zjjrfg_detail());
        }
        Ok(())
    }
}
