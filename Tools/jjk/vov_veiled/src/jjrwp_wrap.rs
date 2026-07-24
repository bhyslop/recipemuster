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
//!
//! Behind the work commit stands the converge: the wrap bequeaths the billet's
//! whole tree to the sire's trunk as one commit parented on trunk's tip, and the
//! studbook journal entry records the position it produced. Delivery is the
//! wrap's own act — no hand merge follows it, and no working tree is checked out
//! to perform it.
//!
//! Both stand on ONE ground resolution (`jjrwp_billet_ground`), so the converge
//! can only fire where the gate judged: a wrap that the gate waved through for
//! want of a billet delivers nothing either.

use std::path::{Path, PathBuf};
use std::io::Write;

use vvc::{vvco_out, vvco_err, vvco_Output};

const JJRWP_CMD_NAME_WRAP: &str = "jjx_wrap";

/// How an absent trailer value is spelled. Every W marker carries every trailer,
/// so a census greps one word rather than reasoning about which lines are missing.
pub const JJRWP_TRAILER_NONE: &str = "none";

use crate::jjrf_favor::{jjrf_Coronet};
use crate::jjrfr_farrier::{jjrfr_BequeathOutcome, jjrfr_FarrierBillet};
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

/// The wrap-entry staleness gate's verdict. The two names the refusal speaks —
/// the billet's own branch and the pedigree's trunk — are the ground the gate was
/// asked about, so the verdict carries no payload of its own and the text home
/// (`jjri_staleness_interdictum`) is fed from that one resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrwp_GateVerdict {
    /// Wrap may proceed: the billet already holds trunk's remote counterpart.
    Clear,
    /// Trunk has advanced past what this billet has enfolded. Wrap refuses.
    Outstripped,
}

/// The billet facts a wrap stands on: which tree it is wrapping, what that tree's
/// line of work is called, and which trunk it answers to. One resolution serving
/// both the staleness gate and the converge, so neither can fire on ground the
/// other declined to judge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct jjrwp_BilletGround {
    pub billet_root: PathBuf,
    pub branch: String,
    pub trunk: String,
}

/// Resolve the wrap's ground, or decline. Composed exactly as the refit door
/// resolves the same tree (`jjrrd_refit.rs`): identify at `cwd`, then take the
/// sire's pedigree trunk.
///
/// Ground is a billet alone (`jjrfr_Seat::Partition` on a branch): the gate's
/// contract is about a billet trailing its sire's trunk, and the converge's is
/// about a billet's estate reaching that trunk — a wrap run from the hippodrome
/// is the ground guards' territory, not either one's. Every unresolvable step —
/// foreign ground, a detached checkout, no upstream key, an unrecorded sire, a
/// farrier rejection — declines. Both consumers then act only on what was
/// observed, and neither cries on ignorance.
pub fn jjrwp_billet_ground<F>(farrier: &F, cwd: &Path) -> Option<jjrwp_BilletGround>
where
    F: crate::jjrfr_farrier::jjrfr_FarrierCore,
{
    use crate::jjrfr_farrier::{jjrfr_LineOfWork, jjrfr_Seat};

    let identity = farrier.jjrfr_identify(cwd).ok()?;
    let hippodrome_root = match &identity.seat {
        jjrfr_Seat::Partition { primary_root } => primary_root.clone(),
        jjrfr_Seat::Primary => return None,
    };
    let branch = match &identity.line_of_work {
        jjrfr_LineOfWork::Branch(name) => name.clone(),
        jjrfr_LineOfWork::Detached(_) => return None,
    };
    let infield_root = hippodrome_root.parent()?.to_path_buf();
    let derived_key = identity.upstream_key.clone()?;

    let studbook = crate::jjrvb_blotter::jjdb_studbook_config(&infield_root);
    let pedigree = crate::jjrds_spine::jjrds_pedigree_lookup(
        &studbook,
        &derived_key,
        crate::jjrds_spine::JJRDS_KIND_PLAIN_GIT,
    )
    .ok()?;

    Some(jjrwp_BilletGround { billet_root: identity.root, branch, trunk: pedigree.trunk })
}

/// The wrap-entry staleness gate: does trunk carry work this billet has never
/// enfolded? Read-only apart from the fetch — it decides, and never remedies.
///
/// Glean leads because staleness is fetch-revealed; an unreachable remote is not
/// a refusal — the probe (`jjrfr_outstripped`) then speaks from the last-known
/// counterpart, which is the strongest verdict an offline station can honestly
/// give. A rejected probe is `Clear` for the same reason: this gate refuses only
/// on a fact it observed, and never cries on ignorance.
///
/// The verdict is total by operator ruling: a wrap that meets `Outstripped` stops
/// there. The gate runs no refit of its own, so the wrap that follows an
/// operator-directed refit finds the billet holding trunk's tip — which is what
/// lets the converge beat treat its own input as trivial by construction.
pub fn jjrwp_staleness_gate<F>(farrier: &F, ground: &jjrwp_BilletGround) -> jjrwp_GateVerdict
where
    F: crate::jjrfr_farrier::jjrfr_FarrierCore + crate::jjrfr_farrier::jjrfr_FarrierBillet,
{
    let _ = farrier.jjrfr_glean(&ground.billet_root);
    match farrier.jjrfr_outstripped(&ground.billet_root, &ground.trunk) {
        Ok(true) => jjrwp_GateVerdict::Outstripped,
        Ok(false) | Err(_) => jjrwp_GateVerdict::Clear,
    }
}

/// Compose the W marker message the wrap records: the marker line, then two
/// one-line trailers.
///
/// `Converge:` is the second counterfoil the squash makes necessary — a bequest
/// leaves no ancestry an ancestry check could follow, so the position trunk
/// accepted is recorded here or nowhere. `Spook:` is the wrap-time friction
/// report. Both are always present and both spell an absent value `none`, so each
/// is a reliable grep surface across the whole corpus rather than one that only
/// appears when it has something to say. A spook's internal newlines collapse to
/// spaces so the trailer stays a single line.
pub fn jjrwp_chalk_message(
    coronet: &jjrf_Coronet,
    description: &str,
    converge: &str,
    spook: Option<&str>,
) -> String {
    let spook_line = spook
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(JJRWP_TRAILER_NONE)
        .replace('\n', " ");
    format!(
        "{}\n\nConverge: {}\nSpook: {}",
        jjrn_format_chalk_message(coronet, jjrn_ChalkMarker::Wrap, description),
        converge,
        spook_line
    )
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

    // The one ground resolution this wrap stands on: the staleness gate below and
    // the converge behind the work commit both read it, so a wrap that resolves no
    // billet neither refuses nor delivers.
    let ground = std::env::current_dir()
        .ok()
        .and_then(|cwd| jjrwp_billet_ground(&crate::jjrfg_plaingit::jjrfg_PlainGit, &cwd));

    // Staleness gate, at wrap entry and ahead of every mutation — the lock, the
    // staging, the commits. A billet trailing its sire's trunk refuses here and
    // stops: no retry, and no refit run from inside the refusal. Passing the gate
    // is what leaves the converge beat a trivial input.
    if let Some(g) = &ground {
        if jjrwp_staleness_gate(&crate::jjrfg_plaingit::jjrfg_PlainGit, g) == jjrwp_GateVerdict::Outstripped {
            vvco_err!(output, "{}", crate::jjri_io::jjri_staleness_interdictum(cn, &g.branch, &g.trunk));
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

    // The converge: bequeath the billet's whole tree to the sire's trunk as one
    // commit parented on trunk's tip. It runs behind the work commit — that
    // commit is what the estate carries, and under the studbook seam it is the
    // last thing to touch the work repo, since the gallops rides the journal now
    // — and ahead of the journal, so the journal entry can record the position
    // produced. Trunk's own record of the pace is a W marker like the journal's,
    // since the two are one event seen from the work repo and from the store; the
    // `Billet:` trailer names the branch whose interior history did not travel.
    //
    // A refusal stops the wrap with the pace still open. Nothing was delivered,
    // so nothing may be recorded as delivered — and by the bequest's
    // compose-then-push construction the refusal leaves no residue: trunk is
    // untouched, no local line of work moved, and the work commit standing on the
    // billet is that branch's own history, which a re-attempt after refit carries
    // forward unchanged.
    //
    // Delivering ahead of the record also decides which way a journal failure
    // falls: work on trunk with the pace still open, never a pace closed on work
    // that never arrived. That re-attempt is safe by the same construction — a
    // billet whose estate already reached trunk is outstripped by it, so the gate
    // sends it through refit, and the bequest that follows finds trunk holding
    // the very tree it would pass and delivers `Unchanged`. Nothing is ever
    // delivered twice.
    let converged: Option<String> = match &ground {
        Some(g) => {
            let trunk_message = format!(
                "{}\n\nBillet: {}",
                jjrn_format_chalk_message(&coronet, jjrn_ChalkMarker::Wrap, &chalk_description),
                g.branch
            );
            match crate::jjrfg_plaingit::jjrfg_PlainGit.jjrfr_bequeath(&g.billet_root, &g.trunk, &trunk_message) {
                Ok(jjrfr_BequeathOutcome::Landed(sha)) => {
                    vvco_out!(output, "{}: converged to trunk '{}' as {}", cn, g.trunk, sha);
                    Some(sha)
                }
                Ok(jjrfr_BequeathOutcome::Unchanged) => {
                    vvco_out!(output, "{}: nothing to converge — trunk '{}' already holds this billet's tree", cn, g.trunk);
                    None
                }
                Err(rejection) => {
                    vvco_err!(output, "{}", crate::jjri_io::jjri_converge_refusal(cn, &g.trunk, &rejection));
                    return (1, output.vvco_finish());
                }
            }
        }
        None => None,
    };

    let chalk_message = jjrwp_chalk_message(
        &coronet,
        &chalk_description,
        converged.as_deref().unwrap_or(JJRWP_TRAILER_NONE),
        spook.as_deref(),
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
    // Converge-era guidance. Delivery already happened above, so the billet arm
    // no longer asks the operator for anything — it reports where the work landed
    // and directs the session out, because the next pace mounts on its own billet
    // and this one's tree is spent. The hippodrome arm keeps the plain form: no
    // billet resolved, so nothing was delivered and there is nowhere to name.
    let landed = ground.as_ref().map(|g| match &converged {
        Some(sha) => format!("delivered to trunk '{}' as {}", g.trunk, sha),
        None => format!("trunk '{}' already held this billet's tree", g.trunk),
    });
    match next_pace_info {
        Some((next_coronet, next_silks, tier, effort)) => {
            let designation = zjjrx_designation_suffix(tier, effort);
            let next_display = gallops.jjrg_qualify_coronet(&next_coronet);
            match &landed {
                Some(where_it_went) => {
                    vvco_out!(output, "AGENT_RESPONSE: {} wrapped and {} \u{2014} this session exits. Next: {} ({}{}) mounts from a fresh session \u{2014} `mount {}`",
                        wrapped_display, where_it_went, next_silks, next_display, designation, fm_str);
                }
                None => {
                    vvco_out!(output, "AGENT_RESPONSE: {} wrapped. Next: {} ({}{}) \u{2014} `/clear` then `mount {}`",
                        wrapped_display, next_silks, next_display, designation, fm_str);
                }
            }
        }
        None => {
            match &landed {
                Some(where_it_went) => {
                    vvco_out!(output, "AGENT_RESPONSE: {} wrapped and {}. All paces complete \u{2014} this session exits; then `retire {}` from a fresh session",
                        wrapped_display, where_it_went, fm_str);
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
