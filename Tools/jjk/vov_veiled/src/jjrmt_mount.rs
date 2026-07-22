// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Mount command - Return context needed to mount a Heat

use std::fs;
use std::path::PathBuf;

use vvc::{vvco_out, vvco_err, vvco_Output};

use crate::jjrf_favor::jjrf_Firemark;
use crate::jjrf_favor::jjrf_Coronet;
use crate::jjrf_favor::{jjrf_bare, JJRF_FIREMARK_LEN, JJRF_CORONET_LEN};
use crate::jjrg_gallops::{
    jjrg_HeatStatus as HeatStatus,
    jjrg_lines_to_text,
    jjrg_PaceState as PaceState,
};
use crate::jjrp_print::{jjrp_Table, jjrp_Column, jjrp_Align};
use crate::jjrs_steeplechase::{jjrs_get_entries, jjrs_ReinArgs};
use crate::jjrpd_parade::jjrpd_write_pace_digest;
use crate::jjrz_gazette::{jjrz_Gazette, jjrz_Slug};

const JJRMT_CMD_NAME_ORIENT: &str = "jjx_orient";

/// Arguments for mount command
#[derive(clap::Args, Debug)]
pub struct jjrmt_MountArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Target Heat identity (Firemark) or Pace identity (Coronet).
    pub firemark: String,
}

/// Run the mount command - return Heat context. `studbook_root` locates the
/// paddock tenant file (paddock tenancy, operator ruling 260722) — threaded
/// from the dispatch door, never derived from cwd here.
pub async fn jjrmt_run_mount(args: jjrmt_MountArgs, gazette: &mut jjrz_Gazette, studbook_root: &std::path::Path) -> (i32, String) {
    let cn = JJRMT_CMD_NAME_ORIENT;
    let mut output = vvco_Output::buffer();

    // Disk space guard — report survey or block if critical
    match crate::jjrdk_diskcheck::jjrdk_check_disk_space() {
        Ok(survey) => vvco_out!(output, "{}", survey),
        Err(msg) => {
            vvco_err!(output, "{}", msg);
            return (1, output.vvco_finish());
        }
    }

    let gallops = match crate::jjrm_mcp::zjjrm_load_gallops(&args.file) {
        Ok(g) => g,
        Err(e) => {
            vvco_err!(output, "{}: error: {}", cn, e);
            return (1, output.vvco_finish());
        }
    };

    // Collect racing heats in heat_order priority sequence for display table
    {
        let racing_heats: Vec<(&String, &crate::jjrg_gallops::jjrg_Heat)> = gallops.heat_order.iter()
            .filter_map(|fm| gallops.heats.get(fm).map(|h| (fm, h)))
            .filter(|(_, heat)| heat.status == HeatStatus::Racing)
            .collect();

        if !racing_heats.is_empty() {
            let mut table = jjrp_Table::jjrp_new(vec![
                jjrp_Column::new("₣Fire", jjrp_Align::Left),
                jjrp_Column::new("Silks", jjrp_Align::Left),
                jjrp_Column::new("Status", jjrp_Align::Left),
                jjrp_Column::new("Done", jjrp_Align::Right),
                jjrp_Column::new("Total", jjrp_Align::Right),
            ]);

            // Measure all rows to compute column widths
            for (key, heat) in &racing_heats {
                let defined_count = heat.paces.values().filter(|pace| {
                    if let Some(tack) = pace.tacks.first() {
                        tack.state != PaceState::Abandoned
                    } else {
                        true
                    }
                }).count();

                let completed_count = heat.paces.values().filter(|pace| {
                    if let Some(tack) = pace.tacks.first() {
                        tack.state == PaceState::Complete
                    } else {
                        false
                    }
                }).count();

                table.jjrp_measure(&[
                    key,
                    &heat.silks,
                    "racing",
                    &completed_count.to_string(),
                    &defined_count.to_string(),
                ]);
            }

            // Print header and separator
            vvco_out!(output, "Racing-heats:");
            table.jjrp_write_header(&mut output);
            table.jjrp_write_separator(&mut output);

            // Print data rows
            for (key, heat) in &racing_heats {
                let defined_count = heat.paces.values().filter(|pace| {
                    if let Some(tack) = pace.tacks.first() {
                        tack.state != PaceState::Abandoned
                    } else {
                        true
                    }
                }).count();

                let completed_count = heat.paces.values().filter(|pace| {
                    if let Some(tack) = pace.tacks.first() {
                        tack.state == PaceState::Complete
                    } else {
                        false
                    }
                }).count();

                table.jjrp_write_row(&mut output, &[
                    key,
                    &heat.silks,
                    "racing",
                    &completed_count.to_string(),
                    &defined_count.to_string(),
                ]);
            }

            vvco_out!(output, "");
        }
    }

    let firemark_str = args.firemark;

    // Detect if input is a coronet (5 chars) or firemark (2 chars). jjrf_bare
    // strips the ₣/₢ glyph and any `·` heat-qualifier to the bare body, so a
    // pasted qualified coronet (₢Bc·CAAAB) types by its 5-char tail.
    let stripped_input = jjrf_bare(&firemark_str);

    let target_coronet = if stripped_input.len() == JJRF_CORONET_LEN {
        // It's a coronet - parse to get parent firemark
        let coronet = match jjrf_Coronet::jjrf_parse(&firemark_str) {
            Ok(c) => c,
            Err(e) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                return (1, output.vvco_finish());
            }
        };
        Some(coronet)
    } else if stripped_input.len() == JJRF_FIREMARK_LEN {
        // It's a firemark - existing behavior
        None
    } else {
        vvco_err!(output, "{}: error: Invalid argument '{}' (must be 2-char firemark or 5-char coronet)", cn, firemark_str);
        return (1, output.vvco_finish());
    };

    // Extract firemark: for a coronet, resolve the harbouring heat by paces-scan
    // (JJS0 jjdt_coronet Resolution); for a firemark, use it directly.
    let firemark = if let Some(ref coronet) = target_coronet {
        match gallops.jjrg_heat_key_of_coronet(&coronet.jjrf_display())
            .and_then(|k| jjrf_Firemark::jjrf_parse(&k).ok())
        {
            Some(fm) => fm,
            None => {
                vvco_err!(output, "{}: error: Pace '{}' not found", cn, coronet.jjrf_display());
                return (1, output.vvco_finish());
            }
        }
    } else {
        match jjrf_Firemark::jjrf_parse(&firemark_str) {
            Ok(fm) => fm,
            Err(e) => {
                vvco_err!(output, "{}: error: {}", cn, e);
                return (1, output.vvco_finish());
            }
        }
    };

    let heat_key = firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            vvco_err!(output, "{}: error: Heat '{}' not found", cn, heat_key);
            return (1, output.vvco_finish());
        }
    };

    // Check if heat is stabled (cannot mount stabled heat)
    if heat.status == HeatStatus::Stabled {
        vvco_err!(output, "{}: error: Cannot mount stabled heat '{}'", cn, heat_key);
        return (1, output.vvco_finish());
    }

    // Read paddock tenant file from the studbook
    let paddock_path = crate::jjri_io::jjri_paddock_file(studbook_root, firemark.jjrf_as_str());
    let paddock_content = match fs::read_to_string(&paddock_path) {
        Ok(content) => content,
        Err(e) => {
            vvco_err!(output, "{}: error reading paddock file '{}': {}", cn, paddock_path.display(), e);
            return (1, output.vvco_finish());
        }
    };

    // Get recent steeplechase entries
    let recent_work = jjrs_get_entries(&jjrs_ReinArgs {
        firemark: firemark.jjrf_as_str().to_string(),
        limit: 10,
    }).unwrap_or_default();

    // Find pace to display
    let mut pace_coronet: Option<String> = None;
    let mut pace_silks: Option<String> = None;
    let mut pace_state: Option<String> = None;
    let mut spec: Option<String> = None;

    if let Some(ref coronet) = target_coronet {
        // Specific coronet requested - look it up directly
        let coronet_key = coronet.jjrf_display();

        match heat.paces.get(&coronet_key) {
            Some(pace) => {
                if let Some(tack) = pace.tacks.first() {
                    match tack.state {
                        // Both open states are mountable; the orient designation guard
                        // (MCP layer) judges the resolved pace against the
                        // caller's tier after this resolution.
                        PaceState::Rough | PaceState::Bridled => {
                            pace_coronet = Some(coronet_key.clone());
                            pace_silks = Some(tack.silks.clone());
                            pace_state = Some(tack.jjrg_state_label());
                            spec = Some(jjrg_lines_to_text(&tack.text));
                        }
                        PaceState::Complete => {
                            vvco_err!(output, "{}: error: Pace '{}' is already complete", cn, coronet_key);
                            return (1, output.vvco_finish());
                        }
                        PaceState::Abandoned => {
                            vvco_err!(output, "{}: error: Pace '{}' is abandoned", cn, coronet_key);
                            return (1, output.vvco_finish());
                        }
                    }
                } else {
                    vvco_err!(output, "{}: error: Pace '{}' has no tacks", cn, coronet_key);
                    return (1, output.vvco_finish());
                }
            }
            None => {
                vvco_err!(output, "{}: error: Pace '{}' not found in heat '{}'", cn, coronet_key, heat_key);
                return (1, output.vvco_finish());
            }
        }
    } else {
        // Find first actionable pace. Bridled is a distinct open state that
        // next-actionable resolution lands on — resolution never skips a
        // tier-mismatched pace (pace order is the dependency tree); the orient
        // guard refuses after resolution instead.
        for coronet_key in &heat.order {
            if let Some(pace) = heat.paces.get(coronet_key) {
                if let Some(tack) = pace.tacks.first() {
                    match tack.state {
                        PaceState::Rough | PaceState::Bridled => {
                            pace_coronet = Some(coronet_key.clone());
                            pace_silks = Some(tack.silks.clone());
                            pace_state = Some(tack.jjrg_state_label());
                            spec = Some(jjrg_lines_to_text(&tack.text));
                            break;
                        }
                        _ => continue,
                    }
                }
            }
        }
    }

    // Determine heat status string
    let status_str = match heat.status {
        HeatStatus::Racing => "racing",
        HeatStatus::Stabled => "stabled",
        HeatStatus::Retired => "retired",
    };

    // Output plain text format
    vvco_out!(output, "Heat: {} ({}) [{}]", heat.silks, heat_key, status_str);
    vvco_out!(output, "Paddock: {}", paddock_path.display());
    vvco_out!(output, "");

    // Save gazette data before output section consumes variables
    let gazette_pace_coronet = pace_coronet.clone();
    let gazette_spec = spec.clone();

    // Pace-level original-intent capture — read once here, covering both
    // resolution paths (specific coronet and first actionable).
    let intent_fields = gazette_pace_coronet.as_ref()
        .and_then(|c| heat.paces.get(c))
        .map(|p| (p.dictation.clone(), p.precis.clone(), p.slated.clone(), p.redocket_count));

    if let Some(coronet) = pace_coronet {
        if let Some(silks) = pace_silks {
            if let Some(state) = pace_state {
                let next_display = gallops.jjrg_qualify_coronet(&coronet);
                vvco_out!(output, "Next: {} ({}) [{}]", silks, next_display, state);
                vvco_out!(output, "");
                // Original-intent block, above the docket — the slate-frozen
                // capture is read before any later-reconstructed docket
                // rationale. The caveat is STANDING (a frozen field over a
                // mutable docket is always possibly-stale); the redocket count
                // is the honest drift signal; the date annotates, never
                // adjudicates. A pre-capture pace has no fields and no block.
                if let Some((ref dictation, ref precis, ref slated, redockets)) = intent_fields {
                    if dictation.is_some() || precis.is_some() {
                        vvco_out!(
                            output,
                            "Original-intent (frozen at slate {}; redockets since: {} — the docket below is the living authority and may have moved on):",
                            slated.as_deref().unwrap_or("<unrecorded>"),
                            redockets
                        );
                        if let Some(d) = dictation {
                            vvco_out!(output, "  Dictation (operator verbatim):");
                            for line in d.lines() {
                                vvco_out!(output, "    {}", line);
                            }
                        }
                        if let Some(p) = precis {
                            vvco_out!(output, "  Precis (LLM-authored from editor context at slate):");
                            for line in p.lines() {
                                vvco_out!(output, "    {}", line);
                            }
                        }
                        vvco_out!(output, "");
                    }
                }
                if let Some(spec_text) = spec {
                    vvco_out!(output, "Docket:");
                    for line in spec_text.lines() {
                        vvco_out!(output, "  {}", line);
                    }
                    vvco_out!(output, "");
                }
                vvco_out!(output, "");
                vvco_out!(output, "Recommended: mount {}", firemark.jjrf_as_str());
                vvco_out!(output, "");
            }
        }
    }

    // The digest is scoped to the pace being mounted — a mount asks what this
    // pace touches and who else is in those files, never what the whole heat
    // has moved. It is silent for a pace with no commits, which is the ordinary
    // case at mount.
    if let Some(coronet) = &gazette_pace_coronet {
        jjrpd_write_pace_digest(&mut output, &firemark, heat, coronet);
    }

    // Output recent work table
    // Filter entries to only show action codes 'n' (notch), 'A' (approach), 'd' (discussion)
    let filtered_work: Vec<_> = recent_work.iter().filter(|entry| {
        if let Some(ref action) = entry.action {
            action == "n" || action == "A" || action == "d"
        } else {
            false
        }
    }).collect();

    if !filtered_work.is_empty() {
        vvco_out!(output, "Recent-work:");

        let mut table = jjrp_Table::jjrp_new(vec![
            jjrp_Column::new("Commit", jjrp_Align::Left),
            jjrp_Column::new("Identity", jjrp_Align::Left),
            jjrp_Column::new("Subject", jjrp_Align::Left),
        ]);

        // Measure all rows to compute column widths
        for entry in &filtered_work {
            let identity_str = if let Some(ref coronet) = entry.coronet {
                coronet.clone()
            } else {
                heat_key.clone()
            };
            table.jjrp_measure(&[
                &entry.commit,
                &identity_str,
                &entry.subject,
            ]);
        }

        // Print header and separator
        table.jjrp_write_header(&mut output);
        table.jjrp_write_separator(&mut output);

        // Print data rows
        for entry in &filtered_work {
            let identity_str = if let Some(ref coronet) = entry.coronet {
                coronet.clone()
            } else {
                heat_key.clone()
            };
            table.jjrp_write_row(&mut output, &[
                &entry.commit,
                &identity_str,
                &entry.subject,
            ]);
        }
    }

    // Add gazette notices for downstream consumption
    gazette.jjrz_add(jjrz_Slug::Paddock, &heat_key, &paddock_content).ok();
    if let (Some(c), Some(d)) = (&gazette_pace_coronet, &gazette_spec) {
        gazette.jjrz_add(jjrz_Slug::Pace, c, d).ok();
    }

    (0, output.vvco_finish())
}
