// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Wrap command - mark a pace complete with automated commit
//!
//! Stages all changes, generates a commit message with Claude,
//! and transitions the pace state to complete with chalk marker.
//!
//! Ahead of all of that stands the staleness gate (`jjrwp_staleness_gate`): a
//! billet whose position trails its sire's trunk never wraps. The refusal is an
//! interdictum naming refit, and it is the whole act — no retry, no automatic
//! remedy. Everything downstream of the gate may therefore assume the billet
//! carries trunk's tip.

use std::path::{Path, PathBuf};
use std::io::Write;

use vvc::{vvco_out, vvco_err, vvco_Output};

const JJRWP_CMD_NAME_WRAP: &str = "jjx_wrap";

use crate::jjrf_favor::{jjrf_Coronet};
use crate::jjrg_gallops::{jjrg_Gallops, jjrg_TallyArgs, jjrg_PaceState, jjrg_Tier, jjrg_Effort, JJRG_STATE_BRIDLED};
use crate::jjrn_notch::{jjrn_ChalkMarker, jjrn_format_notch_prefix, jjrn_format_chalk_message};

/// Arguments for jjx_wrap command
#[derive(clap::Args, Debug)]
pub struct jjrx_WrapArgs {
    /// Pace identity (Coronet)
    pub coronet: String,

    /// Size limit in bytes (overrides default 50KB guard)
    #[arg(long)]
    pub size_limit: Option<u64>,
}

/// The wrap-entry staleness gate's verdict. `Outstripped` carries the two names
/// the refusal speaks — the billet's own branch and the pedigree's trunk — so the
/// text home (`jjri_staleness_interdictum`) is fed by the gate, not by a second
/// resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum jjrwp_GateVerdict {
    /// Wrap may proceed: the billet already holds trunk's remote counterpart, or
    /// this ground is not one the gate judges.
    Clear,
    /// Trunk has advanced past what this billet has enfolded. Wrap refuses.
    Outstripped { branch: String, trunk: String },
}

/// The wrap-entry staleness gate: does trunk carry work this billet has never
/// enfolded? Read-only apart from the fetch — it decides, and never remedies.
///
/// Composed exactly as the refit door resolves the same tree (`jjrrd_refit.rs`):
/// identify at `cwd`, take the sire's pedigree trunk, glean, then ask the
/// farrier's own staleness probe (`jjrfr_outstripped`). Glean leads because
/// staleness is fetch-revealed; an unreachable remote is not a refusal — the
/// probe then speaks from the last-known counterpart, which is the strongest
/// verdict an offline station can honestly give.
///
/// Judged ground is a billet alone (`jjrfr_Seat::Partition` on a branch): the
/// gate's contract is about a billet trailing its sire's trunk, and a wrap run
/// from the hippodrome is the ground guards' territory, not this gate's. Every
/// unresolvable step — foreign ground, a detached checkout, no upstream key, an
/// unrecorded sire, a farrier rejection — returns `Clear`. This gate refuses only
/// on a fact it observed; it never cries on ignorance, the same posture
/// `jjrfr_outstripped` itself takes when no counterpart is known.
///
/// The verdict is total by operator ruling: a wrap that meets `Outstripped` stops
/// there. The gate runs no refit of its own, so the wrap that follows an
/// operator-directed refit finds the billet holding trunk's tip — which is what
/// lets the converge beat treat its own input as trivial by construction.
pub fn jjrwp_staleness_gate<F>(farrier: &F, cwd: &Path) -> jjrwp_GateVerdict
where
    F: crate::jjrfr_farrier::jjrfr_FarrierCore + crate::jjrfr_farrier::jjrfr_FarrierBillet,
{
    use crate::jjrfr_farrier::{jjrfr_LineOfWork, jjrfr_Seat};

    let identity = match farrier.jjrfr_identify(cwd) {
        Ok(id) => id,
        Err(_) => return jjrwp_GateVerdict::Clear,
    };
    let hippodrome_root = match &identity.seat {
        jjrfr_Seat::Partition { primary_root } => primary_root.clone(),
        jjrfr_Seat::Primary => return jjrwp_GateVerdict::Clear,
    };
    let branch = match &identity.line_of_work {
        jjrfr_LineOfWork::Branch(name) => name.clone(),
        jjrfr_LineOfWork::Detached(_) => return jjrwp_GateVerdict::Clear,
    };
    let infield_root = match hippodrome_root.parent() {
        Some(p) => p.to_path_buf(),
        None => return jjrwp_GateVerdict::Clear,
    };
    let derived_key = match &identity.upstream_key {
        Some(k) => k.clone(),
        None => return jjrwp_GateVerdict::Clear,
    };

    let studbook = crate::jjrvb_blotter::jjdb_studbook_config(&infield_root);
    let pedigree = match crate::jjrds_spine::jjrds_pedigree_lookup(
        &studbook,
        &derived_key,
        crate::jjrds_spine::JJRDS_KIND_PLAIN_GIT,
    ) {
        Ok(p) => p,
        Err(_) => return jjrwp_GateVerdict::Clear,
    };

    let _ = farrier.jjrfr_glean(&identity.root);
    match farrier.jjrfr_outstripped(&identity.root, &pedigree.trunk) {
        Ok(true) => jjrwp_GateVerdict::Outstripped { branch, trunk: pedigree.trunk },
        Ok(false) | Err(_) => jjrwp_GateVerdict::Clear,
    }
}

/// Render a bridled next-pace's designation as `" [bridled tier effort]"`,
/// mirroring the bracket shape `jjx_coronets` prints; a rough (undesignated)
/// pace has no tier, so it renders as an empty suffix.
fn zjjrx_designation_suffix(tier: Option<jjrg_Tier>, effort: Option<jjrg_Effort>) -> String {
    match tier {
        Some(t) => {
            let designation = match effort {
                Some(e) => format!("{} {}", t.jjrg_as_str(), e.jjrg_as_str()),
                None => t.jjrg_as_str().to_string(),
            };
            format!(" [{} {}]", JJRG_STATE_BRIDLED, designation)
        }
        None => String::new(),
    }
}

/// Helper to get pace silks or return default message
fn get_pace_silks_or_default(gallops: &jjrg_Gallops, firemark_key: &str, coronet_key: &str) -> String {
    gallops.heats
        .get(firemark_key)
        .and_then(|heat| heat.paces.get(coronet_key))
        .and_then(|pace| pace.tacks.first())
        .map(|tack| format!("pace {} complete", tack.silks))
        .unwrap_or_else(|| "pace complete".to_string())
}

/// Execute wrap command - mark pace complete with commit
///
/// Stages all changes, checks size, generates commit message using Claude,
/// creates the work commit, then transitions pace to complete state with
/// chalk marker commit.
///
/// The `spook` argument carries the agent's wrap-time friction report (in-chat
/// stumbles: re-reads, a docket pointing at a renamed file, a confusing paddock).
/// It rides the W chalk commit as a single-line `Spook:` trailer, building a
/// grep-extractable corpus for later affordance tuning. Absent or empty becomes
/// `Spook: none`, so every wrap carries the line as a reliable grep surface.
///
/// Returns exit code (0 for success, non-zero for failure).
pub fn zjjrx_run_wrap(args: jjrx_WrapArgs, summary: Option<String>, spook: Option<String>, officium: &str) -> (i32, String) {
    let cn = JJRWP_CMD_NAME_WRAP;
    let mut output = vvco_Output::buffer();

    // Parse coronet
    let coronet = match jjrf_Coronet::jjrf_parse(&args.coronet) {
        Ok(c) => c,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Staleness gate, at wrap entry and ahead of every mutation — the lock, the
    // staging, the commits. A billet trailing its sire's trunk refuses here and
    // stops: no retry, and no refit run from inside the refusal. Passing the gate
    // is what leaves the converge beat a trivial input.
    let gate_cwd = std::env::current_dir().ok();
    if let Some(cwd) = gate_cwd.as_deref() {
        if let jjrwp_GateVerdict::Outstripped { branch, trunk } =
            jjrwp_staleness_gate(&crate::jjrfg_plaingit::jjrfg_PlainGit, cwd)
        {
            vvco_err!(output, "{}", crate::jjri_io::jjri_staleness_interdictum(cn, &branch, &trunk));
            return (2, output.vvco_finish());
        }
    }

    let stdin_summary = summary;

    // Acquire commit lock
    let _lock = match vvc::vvcc_CommitLock::vvcc_acquire() {
        Ok(l) => l,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Stage all changes
    let add_output = match vvc::vvce_git_command(&["add", "-A"])
        .output()
    {
        Ok(o) => o,
        Err(e) => {
            vvco_err!(output, "{}: error: failed to run git add: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    if !add_output.status.success() {
        vvco_err!(output, "{}: error: git add failed", cn);
        return (1, output.vvco_finish());
    }

    // Size gate. Wrap commits its content with git directly, so it weighs the staged
    // tree itself — through the same cost model every other commanding path is judged
    // by, against the same ceiling. (It formerly summed added+deleted *lines* from
    // numstat and compared that count to a byte limit, which let any bulk through so
    // long as it arrived on few enough lines.)
    let size_limit = args.size_limit.unwrap_or(vvc::VVCG_SIZE_LIMIT);
    let cost = match vvc::vvcg_cost(None) {
        Ok(c) => c,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    if cost.total > size_limit {
        vvco_err!(output, "{}", crate::jjri_io::jjri_size_interdictum(cn, &cost, size_limit));
        // Lock released automatically by Drop
        return (2, output.vvco_finish());
    }

    // Check if there are staged changes to commit
    let has_staged_changes = !cost.files.is_empty();

    // Only generate commit message and commit if there are staged changes
    if has_staged_changes {
        // Generate commit message using Claude CLI
        let diff_content = match vvc::vvce_git_command(&["diff", "--cached"])
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                vvco_err!(output, "{}: error: failed to run git diff --cached: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };

        if !diff_content.status.success() {
            vvco_err!(output, "{}: error: git diff --cached failed", cn);
            return (1, output.vvco_finish());
        }

        let mut claude_cmd = match vvc::vvce_claude_command()
            .args([
                "--print",
                &format!("Generate a concise commit message for this diff. The commit wraps pace {}. Output only the message, no quotes.", args.coronet)
            ])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                vvco_err!(output, "{}: error: failed to spawn claude command: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };

        // Write diff to stdin
        if let Some(mut stdin) = claude_cmd.stdin.take() {
            if let Err(e) = stdin.write_all(&diff_content.stdout) {
                vvco_err!(output, "{}: error: failed to write to claude stdin: {}", cn, e);
                return (1, output.vvco_finish());
            }
        }

        let claude_output = match claude_cmd.wait_with_output() {
            Ok(o) => o,
            Err(e) => {
                vvco_err!(output, "{}: error: failed to wait for claude: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };

        if !claude_output.status.success() {
            vvco_err!(output, "{}: error: claude command failed", cn);
            vvco_err!(output, "{}", String::from_utf8_lossy(&claude_output.stderr));
            return (1, output.vvco_finish());
        }

        let generated_message = String::from_utf8_lossy(&claude_output.stdout).trim().to_string();

        // Commit with git directly (already staged via git add -A)
        let prefix = jjrn_format_notch_prefix(&coronet);
        let full_message = format!("{}{}\n\nCo-Authored-By: Claude <noreply@anthropic.com>", prefix, generated_message);

        let commit_output = match vvc::vvce_git_command(&["commit", "-m", &full_message])
            .output()
        {
            Ok(o) => o,
            Err(e) => {
                vvco_err!(output, "{}: error: failed to run git commit: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };

        if !commit_output.status.success() {
            vvco_err!(output, "{}: error: git commit failed", cn);
            vvco_err!(output, "{}", String::from_utf8_lossy(&commit_output.stderr));
            return (1, output.vvco_finish());
        }

    } else {
        // No staged changes - this is valid for verification-only paces
        vvco_out!(output, "{}: no staged changes, proceeding with state transition only", cn);
    }

    // Transition pace state to complete
    let gallops_path = PathBuf::from(".claude/jjm/jjg_gallops.json");
    let mut gallops = match crate::jjrm_mcp::zjjrm_load_gallops(&gallops_path) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error loading Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    let tally_args = jjrg_TallyArgs {
        coronet: args.coronet.clone(),
        state: Some(jjrg_PaceState::Complete),
        text: None,
        silks: None,
    };

    // Apply the transition to the session gallops — it backs the chalk message and
    // the post-wrap lookahead below in both seam states. Seam-off it is also what
    // persists; seam-on it is display-only (the journal re-applies the same state
    // transition to the locked tip, and a state change mints no identity, so the
    // double application is safe — the trap the machine_commit family's ruling
    // reserved for the mint-bearing draft/restring, not for wrap).
    if let Err(e) = gallops.jjrg_tally(tally_args.clone()) {
        vvco_err!(output, "{}: error: {}", cn, e);
        return (1, output.vvco_finish());
    }

    // Build chalk description: use stdin if provided, else "pace {silks} complete".
    // Resolve the pace's heat by paces-scan (JJS0 jjdt_coronet Resolution).
    let firemark_key = match gallops.jjrg_heat_key_of_coronet(&coronet.jjrf_display()) {
        Some(k) => k,
        None => {
            vvco_err!(output, "{}: error: Pace '{}' not found", cn, coronet.jjrf_display());
            return (1, output.vvco_finish());
        }
    };
    let coronet_key = coronet.jjrf_display();

    let chalk_description = if let Some(ref text) = stdin_summary {
        let trimmed = text.trim();
        if !trimmed.is_empty() {
            trimmed.to_string()
        } else {
            // Empty stdin - use default with silks
            get_pace_silks_or_default(&gallops, &firemark_key, &coronet_key)
        }
    } else {
        // No stdin - use default with silks
        get_pace_silks_or_default(&gallops, &firemark_key, &coronet_key)
    };

    // Create W chalk marker commit with gallops state change.
    // Append the wrap-time friction report as a single-line `Spook:` trailer:
    // absent/empty becomes "none", and internal newlines collapse to spaces so
    // the trailer stays one grep-extractable line.
    let spook_line = spook
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("none")
        .replace('\n', " ");
    let chalk_message = format!(
        "{}\n\nSpook: {}",
        jjrn_format_chalk_message(&coronet, jjrn_ChalkMarker::Wrap, &chalk_description),
        spook_line
    );

    // Commit the W chalk. Off: save the gallops and machine_commit it to the
    // consumer repo (the pre-seam path). On: journal the same state transition to
    // the studbook against the locked tip — the gallops leaves the consumer repo
    // (the work commit above already carried the user's code), and the chalk
    // message rides the journal commit. Either way `chalk_hash` is the marker SHA.
    let chalk_hash = if !crate::jjrvb_blotter::JJDB_GALLOPS_OVER_STUDBOOK_ENABLED {
        if let Err(e) = gallops.jjrg_save(&gallops_path) {
            vvco_err!(output, "{}: error saving Gallops: {}", cn, e);
            return (1, output.vvco_finish());
        }
        let chalk_commit_args = vvc::vvcm_CommitArgs {
            files: vec![".claude/jjm/jjg_gallops.json".to_string()],
            message: chalk_message,
            size_limit: vvc::VVCG_SIZE_LIMIT,
            warn_limit: vvc::VVCG_WARN_LIMIT,
        };
        match vvc::machine_commit(&_lock, &chalk_commit_args, &mut output) {
            Ok(chalk_hash) => chalk_hash,
            Err(e) => {
                vvco_err!(output, "{}", crate::jjri_io::jjri_commit_refusal(cn, &e));
                vvco_err!(output, "{}: warning: chalk uncommitted — gallops state updated but not committed", cn);
                return (1, output.vvco_finish());
            }
        }
    } else {
        // Seam-on: derive the studbook + guidon, then journal the same state
        // transition through the extracted ON path (jjrwp_wrap_over) — the
        // explicit-config form a test drives against a fixture studbook while the
        // const stays false.
        let (studbook, guidon) = match crate::jjrm_mcp::zjjrm_studbook_and_guidon(officium, cn) {
            Ok(sg) => sg,
            Err(e) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };
        match jjrwp_wrap_over(
            &crate::jjrfg_plaingit::jjrfg_PlainGit,
            &studbook,
            &guidon,
            tally_args,
            chalk_message,
            &mut output,
            cn,
        ) {
            Ok(sha) => sha,
            Err(code) => return (code, output.vvco_finish()),
        }
    };

    vvco_out!(output, "{}", chalk_hash);
    let fm = match gallops.jjrg_heat_key_of_coronet(&coronet.jjrf_display())
        .and_then(|k| crate::jjrf_favor::jjrf_Firemark::jjrf_parse(&k).ok())
    {
        Some(f) => f,
        None => {
            vvco_err!(output, "{}: error: wrapped pace '{}' not found in any heat", cn, coronet.jjrf_display());
            return (1, output.vvco_finish());
        }
    };
    let fm_key = fm.jjrf_display();
    let fm_str = fm.jjrf_as_str();

    // Lookahead: find next actionable pace in this heat (both open
    // states — a bridled pace is next-actionable, at its tier).
    let next_pace_info = gallops.heats.get(&fm_key).and_then(|heat| {
        heat.order.iter().find_map(|c| {
            heat.paces.get(c.as_str()).and_then(|pace| {
                pace.tacks.first().and_then(|tack| {
                    match tack.state {
                        jjrg_PaceState::Rough | jjrg_PaceState::Bridled => {
                            Some((c.clone(), tack.silks.clone(), tack.tier, tack.effort))
                        }
                        _ => None,
                    }
                })
            })
        })
    });

    vvco_out!(output, "");
    // Heat-qualified coronets for the operator-facing relay line.
    let wrapped_display = gallops.jjrg_qualify_coronet(&coronet.jjrf_display());
    // Hand-merge era interim: until the wrap converge machinery lands, billet
    // work reaches the trunk only by an operator-directed plain merge in the
    // hippodrome — merge, never rebase. A billet is recognized by reading its
    // branch's livery badge and matching the carried body against the wrapped
    // pace (the hippodrome sits on the trunk, which wears no badge, so it never
    // matches); reading rather than composing keeps this blind to a sire's
    // recorded livery prefix. The probe is advisory, so a git failure falls back
    // to the standard guidance. Dies when the converge machinery replaces it.
    let billet_branch = vvc::vvce_git_command(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|b| {
            crate::jjrf_favor::jjrf_livery_parse(b).is_some_and(|(kind, body)| {
                kind == crate::jjrf_favor::jjrf_LiveryKind::Pace && body == coronet.jjrf_as_str()
            })
        });
    match next_pace_info {
        Some((next_coronet, next_silks, tier, effort)) => {
            let designation = zjjrx_designation_suffix(tier, effort);
            let next_display = gallops.jjrg_qualify_coronet(&next_coronet);
            match &billet_branch {
                Some(b) => {
                    vvco_out!(output, "AGENT_RESPONSE: {} wrapped on billet branch '{}'. Hand-merge era: offer `git push origin {}` from here; the operator then merges in the hippodrome \u{2014} `git merge {}` on the trunk, then push; merge, never rebase \u{2014} and this session exits. Next: {} ({}{}) mounts from a fresh session \u{2014} `mount {}`",
                        wrapped_display, b, b, b, next_silks, next_display, designation, fm_str);
                }
                None => {
                    vvco_out!(output, "AGENT_RESPONSE: {} wrapped. Next: {} ({}{}) \u{2014} `/clear` then `mount {}`",
                        wrapped_display, next_silks, next_display, designation, fm_str);
                }
            }
        }
        None => {
            match &billet_branch {
                Some(b) => {
                    vvco_out!(output, "AGENT_RESPONSE: {} wrapped on billet branch '{}'. All paces complete. Hand-merge era: offer `git push origin {}` from here; the operator then merges in the hippodrome \u{2014} `git merge {}` on the trunk, then push; merge, never rebase \u{2014} and this session exits. Then `retire {}` from a fresh session",
                        wrapped_display, b, b, b, fm_str);
                }
                None => {
                    vvco_out!(output, "AGENT_RESPONSE: {} wrapped. All paces complete \u{2014} `/clear` then `retire {}`",
                        wrapped_display, fm_str);
                }
            }
        }
    }
    (0, output.vvco_finish())
    // lock released here
}

/// The seam-ON wrap path, extracted from the const gate so a test drives it
/// against a fixture studbook while `JJDB_GALLOPS_OVER_STUDBOOK_ENABLED` stays
/// false (the `_over` idiom). wrap is the machine_commit family's easiest member:
/// `jjrg_tally` mints no identity, so the on path journals the same Complete-state
/// transition to the studbook against the locked tip — the gallops leaves the
/// consumer repo (the work commit already carried the user's code) and the chalk
/// message rides the journal commit. `studbook`/`guidon` arrive resolved. On
/// success returns the marker SHA for the shared lookahead tail; on refusal writes
/// to `output` (the Blotter arm keeps wrap's chalk-uncommitted warning) and returns
/// the exit code.
#[allow(clippy::too_many_arguments)]
pub(crate) fn jjrwp_wrap_over<F>(
    farrier: &F,
    studbook: &crate::jjrvb_blotter::jjdb_BlotterConfig,
    guidon: &str,
    tally_args: jjrg_TallyArgs,
    chalk_message: String,
    output: &mut vvco_Output,
    cn: &str,
) -> Result<String, i32>
where
    F: crate::jjrfr_farrier::jjrfr_FarrierCore + crate::jjrfr_farrier::jjrfr_FarrierLock,
{
    match crate::jjrm_mcp::zjjrm_journal_run(farrier, studbook, guidon, |g| {
        g.jjrg_tally(tally_args).map(|u| (u, chalk_message))
    }) {
        Ok((_unit, sha)) => Ok(sha),
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Handler(e)) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            Err(1)
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Commit(e)) => {
            vvco_err!(output, "{}", crate::jjri_io::jjri_commit_refusal(cn, &e));
            Err(1)
        }
        Err(crate::jjrm_mcp::zjjrm_WriteRefusal::Blotter(r)) => {
            vvco_err!(output, "{}: studbook journal refused: {}", cn, r);
            vvco_err!(output, "{}: warning: chalk uncommitted — gallops state updated but not committed", cn);
            Err(1)
        }
    }
}
