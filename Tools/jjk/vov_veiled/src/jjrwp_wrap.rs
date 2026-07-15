// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Wrap command - mark a pace complete with automated commit
//!
//! Stages all changes, generates a commit message with Claude,
//! and transitions the pace state to complete with chalk marker.

use std::path::PathBuf;
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
pub fn zjjrx_run_wrap(args: jjrx_WrapArgs, summary: Option<String>, spook: Option<String>) -> (i32, String) {
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
    let mut gallops = match jjrg_Gallops::jjrg_load(&gallops_path) {
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

    if let Err(e) = gallops.jjrg_tally(tally_args) {
        vvco_err!(output, "{}: error: {}", cn, e);
        return (1, output.vvco_finish());
    }

    // Save gallops
    if let Err(e) = gallops.jjrg_save(&gallops_path) {
        vvco_err!(output, "{}: error saving Gallops: {}", cn, e);
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

    let chalk_commit_args = vvc::vvcm_CommitArgs {
        files: vec![".claude/jjm/jjg_gallops.json".to_string()],
        message: chalk_message,
        size_limit: vvc::VVCG_SIZE_LIMIT,
        warn_limit: vvc::VVCG_WARN_LIMIT,
    };

    match vvc::machine_commit(&_lock, &chalk_commit_args, &mut output) {
        Ok(chalk_hash) => {
            vvco_out!(output, "{}", chalk_hash);
            let fm = match gallops.jjrg_heat_key_of_coronet(&coronet.jjrf_display())
                .and_then(|k| jjrf_Firemark::jjrf_parse(&k).ok())
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
            match next_pace_info {
                Some((next_coronet, next_silks, tier, effort)) => {
                    let designation = zjjrx_designation_suffix(tier, effort);
                    vvco_out!(output, "AGENT_RESPONSE: {} wrapped. Next: {} ({}{}) \u{2014} `/clear` then `mount {}`",
                        coronet.jjrf_display(), next_silks, next_coronet, designation, fm_str);
                }
                None => {
                    vvco_out!(output, "AGENT_RESPONSE: {} wrapped. All paces complete \u{2014} `/clear` then `retire {}`",
                        coronet.jjrf_display(), fm_str);
                }
            }
            (0, output.vvco_finish())
        }
        Err(e) => {
            vvco_err!(output, "{}", crate::jjri_io::jjri_commit_refusal(cn, &e));
            vvco_err!(output, "{}: warning: chalk uncommitted — gallops state updated but not committed", cn);
            (1, output.vvco_finish())
        }
    }
    // lock released here
}
