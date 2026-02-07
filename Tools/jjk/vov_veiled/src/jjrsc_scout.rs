// Copyright 2025 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Scout command - Search across heats and paces with regex

use clap::Args;
use std::path::PathBuf;
use regex::Regex;
use std::fs;

use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_PaceState as PaceState};

/// Arguments for jjx_scout command
#[derive(Args, Debug)]
pub struct jjrsc_ScoutArgs {
    /// Path to the Gallops JSON file
    #[arg(long, short = 'f', default_value = ".claude/jjm/jjg_gallops.json")]
    pub file: PathBuf,

    /// Regex pattern to search for
    pub pattern: String,

    /// Limit to actionable paces only (rough/bridled)
    #[arg(long)]
    pub actionable: bool,
}

/// Run the scout command - regex search across heats/paces
pub fn jjrsc_run_scout(args: jjrsc_ScoutArgs) -> i32 {
    use regex::RegexBuilder;

    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_scout: error: {}", e);
            return 1;
        }
    };

    // Build case-insensitive regex
    let re = match RegexBuilder::new(&args.pattern)
        .case_insensitive(true)
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("jjx_scout: error: invalid regex pattern: {}", e);
            return 1;
        }
    };

    // Track which heats have matches (for group headers)
    let mut current_heat_key: Option<String> = None;

    // Iterate all heats (racing, stabled, retired)
    for (heat_key, heat) in &gallops.heats {
        // Search each pace in order
        for coronet_key in &heat.order {
            if let Some(pace) = heat.paces.get(coronet_key) {
                if let Some(tack) = pace.tacks.first() {
                    // Apply actionable filter if requested
                    if args.actionable {
                        match tack.state {
                            PaceState::Rough | PaceState::Bridled => {},
                            _ => continue,
                        }
                    }

                    // Search: silks, spec (tack text), direction, paddock content
                    // Use owned strings to avoid lifetime issues
                    let mut match_result: Option<(String, String)> = None;

                    if re.is_match(&tack.silks) {
                        match_result = Some(("silks".to_string(), tack.silks.clone()));
                    } else if re.is_match(&tack.text) {
                        match_result = Some(("spec".to_string(), tack.text.clone()));
                    } else if let Some(ref direction) = tack.direction {
                        if re.is_match(direction) {
                            match_result = Some(("direction".to_string(), direction.clone()));
                        }
                    } else {
                        // Search paddock content
                        let paddock_path = std::path::Path::new(&heat.paddock_file);
                        if let Ok(content) = fs::read_to_string(paddock_path) {
                            if re.is_match(&content) {
                                match_result = Some(("paddock".to_string(), content));
                            }
                        }
                    }

                    // If we found a match, output it
                    if let Some((field_name, field_content)) = match_result {
                        // Print heat header if this is a new heat
                        if current_heat_key.as_ref() != Some(heat_key) {
                            println!("{} {}", heat_key, heat.silks);
                            current_heat_key = Some(heat_key.clone());
                        }

                        // Print pace line
                        let state_str = zjrsc_pace_state_str(&tack.state);
                        println!("  {} [{}] {}", coronet_key, state_str, tack.silks);

                        // Print match line with context (extract ~60 chars around match)
                        let match_excerpt = zjrsc_extract_match_context(&field_content, &re);
                        println!("    {}: {}", field_name, match_excerpt);
                    }
                }
            }
        }
    }

    0
}

/// Helper to convert PaceState to display string
pub(crate) fn zjrsc_pace_state_str(state: &PaceState) -> &'static str {
    match state {
        PaceState::Rough => "rough",
        PaceState::Bridled => "bridled",
        PaceState::Complete => "complete",
        PaceState::Abandoned => "abandoned",
    }
}

/// Extract context around the first regex match in content
pub(crate) fn zjrsc_extract_match_context(content: &str, re: &Regex) -> String {
    if let Some(mat) = re.find(content) {
        let start = mat.start();
        let end = mat.end();

        // Find context window (30 chars before, 30 chars after)
        let context_before = 30;
        let context_after = 30;

        // Snap window boundaries to valid UTF-8 char boundaries
        let mut window_start = start.saturating_sub(context_before);
        while !content.is_char_boundary(window_start) {
            window_start -= 1;
        }

        let mut window_end = (end + context_after).min(content.len());
        while !content.is_char_boundary(window_end) {
            window_end += 1;
        }
        let window_end = window_end.min(content.len());

        let mut excerpt = String::new();
        if window_start > 0 {
            excerpt.push_str("...");
        }
        excerpt.push_str(&content[window_start..window_end]);
        if window_end < content.len() {
            excerpt.push_str("...");
        }

        // Replace newlines with spaces for single-line display
        excerpt.replace('\n', " ").replace('\r', "")
    } else {
        // Shouldn't happen, but fallback to truncated content
        let truncated = content.chars().take(60).collect::<String>();
        if content.len() > 60 {
            format!("{}...", truncated)
        } else {
            truncated
        }
    }
}
