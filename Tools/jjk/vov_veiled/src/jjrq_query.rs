// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Query operations for Gallops JSON
//!
//! Implements read operations: muster, saddle, parade, retire.
//! All operations read from Gallops JSON and optionally paddock files.

use crate::jjrf_favor::jjrf_Firemark as Firemark;
use crate::jjrf_favor::jjrf_Coronet as Coronet;
use crate::jjrg_gallops::{jjrg_Gallops as Gallops, jjrg_HeatStatus as HeatStatus, jjrg_PaceState as PaceState, jjrg_Pace as Pace};
use crate::jjrs_steeplechase::{jjrs_SteeplechaseEntry, jjrs_get_entries, jjrs_ReinArgs};
use serde::Serialize;
use std::fs;

// ============================================================================
// Muster - List all Heats with summary information
// ============================================================================

/// Arguments for muster command
#[derive(Debug)]
pub struct jjrq_MusterArgs {
    pub file: std::path::PathBuf,
}

/// Run the muster command - list Heats as TSV
pub fn jjrq_run_muster(args: jjrq_MusterArgs) -> i32 {
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_muster: error: {}", e);
            return 1;
        }
    };

    for (key, heat) in &gallops.heats {
        let pace_count = heat.paces.len();
        let status_str = match heat.status {
            HeatStatus::Racing => "racing",
            HeatStatus::Stabled => "stabled",
            HeatStatus::Retired => "retired",
        };

        println!("{}\t{}\t{}\t{}", key, heat.silks, status_str, pace_count);
    }

    0
}

// ============================================================================
// Saddle - Return context needed to saddle up on a Heat
// ============================================================================

/// Arguments for saddle command
#[derive(Debug)]
pub struct jjrq_SaddleArgs {
    pub file: std::path::PathBuf,
    pub firemark: Firemark,
}

/// Output structure for saddle command
#[derive(Serialize)]
pub(crate) struct zjjrq_SaddleOutput {
    pub(crate) heat_silks: String,
    pub(crate) paddock_file: String,
    pub(crate) paddock_content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pace_coronet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pace_silks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) pace_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) spec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) direction: Option<String>,
    pub(crate) recent_work: Vec<jjrs_SteeplechaseEntry>,
}

/// Run the saddle command - return Heat context
pub fn jjrq_run_saddle(args: jjrq_SaddleArgs) -> i32 {
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_saddle: error: {}", e);
            return 1;
        }
    };

    let heat_key = args.firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            eprintln!("jjx_saddle: error: Heat '{}' not found", heat_key);
            return 1;
        }
    };

    // Check if heat is stabled (cannot saddle stabled heat)
    if heat.status == HeatStatus::Stabled {
        eprintln!("jjx_saddle: error: Cannot saddle stabled heat '{}'", heat_key);
        return 1;
    }

    // Read paddock file content
    let paddock_content = match fs::read_to_string(&heat.paddock_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("jjx_saddle: error reading paddock file '{}': {}", heat.paddock_file, e);
            return 1;
        }
    };

    // Get recent steeplechase entries
    let recent_work = jjrs_get_entries(&jjrs_ReinArgs {
        firemark: args.firemark.jjrf_as_str().to_string(),
        limit: 10,
    }).unwrap_or_default();

    // Find first actionable pace (rough or bridled)
    let mut output = zjjrq_SaddleOutput {
        heat_silks: heat.silks.clone(),
        paddock_file: heat.paddock_file.clone(),
        paddock_content,
        pace_coronet: None,
        pace_silks: None,
        pace_state: None,
        spec: None,
        direction: None,
        recent_work,
    };

    for coronet_key in &heat.order {
        if let Some(pace) = heat.paces.get(coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                match tack.state {
                    PaceState::Rough | PaceState::Bridled => {
                        output.pace_coronet = Some(coronet_key.clone());
                        output.pace_silks = Some(tack.silks.clone());
                        output.pace_state = Some(match tack.state {
                            PaceState::Rough => "rough".to_string(),
                            PaceState::Bridled => "bridled".to_string(),
                            _ => unreachable!(),
                        });
                        output.spec = Some(tack.text.clone());
                        if tack.state == PaceState::Bridled {
                            output.direction = tack.direction.clone();
                        }
                        break;
                    }
                    _ => continue,
                }
            }
        }
    }

    // Output JSON
    match serde_json::to_string_pretty(&output) {
        Ok(json) => {
            println!("{}", json);
            0
        }
        Err(e) => {
            eprintln!("jjx_saddle: error serializing output: {}", e);
            1
        }
    }
}

// ============================================================================
// Parade - Display comprehensive Heat status for project review
// ============================================================================

/// Output format modes for parade command
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum jjrq_ParadeFormat {
    /// One line per pace: [state] silks (₢coronet)
    Overview,
    /// Numbered list: N. [state] silks (₢coronet)
    Order,
    /// Full tack text for one pace (requires --pace)
    Detail,
    /// Paddock + all paces with tack text (default)
    #[default]
    Full,
}

/// Arguments for parade command
#[derive(Debug)]
pub struct jjrq_ParadeArgs {
    pub file: std::path::PathBuf,
    pub firemark: Firemark,
    pub format: jjrq_ParadeFormat,
    pub pace: Option<String>,
    pub remaining: bool,
}

/// Run the parade command - display comprehensive Heat status
pub fn jjrq_run_parade(args: jjrq_ParadeArgs) -> i32 {
    // Validate: detail format requires --pace
    if args.format == jjrq_ParadeFormat::Detail && args.pace.is_none() {
        eprintln!("jjx_parade: error: --format detail requires --pace <coronet>");
        return 1;
    }

    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_parade: error: {}", e);
            return 1;
        }
    };

    let heat_key = args.firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            eprintln!("jjx_parade: error: Heat '{}' not found", heat_key);
            return 1;
        }
    };

    match args.format {
        jjrq_ParadeFormat::Overview => {
            for coronet_key in &heat.order {
                if let Some(pace) = heat.paces.get(coronet_key) {
                    if let Some(tack) = pace.tacks.first() {
                        // Skip complete/abandoned if --remaining
                        if args.remaining && (tack.state == PaceState::Complete || tack.state == PaceState::Abandoned) {
                            continue;
                        }
                        let state_str = zjjrq_pace_state_str(&tack.state);
                        println!("[{}] {} ({})", state_str, tack.silks, coronet_key);
                    }
                }
            }
        }
        jjrq_ParadeFormat::Order => {
            let mut num = 0;
            for coronet_key in &heat.order {
                if let Some(pace) = heat.paces.get(coronet_key) {
                    if let Some(tack) = pace.tacks.first() {
                        // Skip complete/abandoned if --remaining
                        if args.remaining && (tack.state == PaceState::Complete || tack.state == PaceState::Abandoned) {
                            continue;
                        }
                        num += 1;
                        let state_str = zjjrq_pace_state_str(&tack.state);
                        println!("{}. [{}] {} ({})", num, state_str, tack.silks, coronet_key);
                    }
                }
            }
        }
        jjrq_ParadeFormat::Detail => {
            let pace_arg = args.pace.as_ref().unwrap();
            let (coronet_key, pace) = match zjjrq_resolve_pace(heat, pace_arg) {
                Some(result) => result,
                None => {
                    eprintln!("jjx_parade: error: Pace '{}' not found in Heat '{}' (tried coronet and silks)", pace_arg, heat_key);
                    return 1;
                }
            };
            if let Some(tack) = pace.tacks.first() {
                let state_str = zjjrq_pace_state_str(&tack.state);
                println!("Pace: {} ({})", tack.silks, coronet_key);
                println!("State: {}", state_str);
                println!("Heat: {}", heat_key);
                println!();
                println!("{}", tack.text);
                if let Some(ref direction) = tack.direction {
                    println!();
                    println!("Direction: {}", direction);
                }
            }
        }
        jjrq_ParadeFormat::Full => {
            // Read paddock file content
            let paddock_content = match fs::read_to_string(&heat.paddock_file) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("jjx_parade: error reading paddock file '{}': {}", heat.paddock_file, e);
                    return 1;
                }
            };

            let status_str = match heat.status {
                HeatStatus::Racing => "racing",
                HeatStatus::Stabled => "stabled",
                HeatStatus::Retired => "retired",
            };

            println!("Heat: {} ({})", heat.silks, heat_key);
            println!("Status: {}", status_str);
            println!("Created: {}", heat.creation_time);
            println!();
            println!("## Paddock");
            println!();
            println!("{}", paddock_content);
            println!();
            println!("## Paces");
            println!();

            for coronet_key in &heat.order {
                if let Some(pace) = heat.paces.get(coronet_key) {
                    if let Some(tack) = pace.tacks.first() {
                        // Skip complete/abandoned if --remaining
                        if args.remaining && (tack.state == PaceState::Complete || tack.state == PaceState::Abandoned) {
                            continue;
                        }
                        let state_str = zjjrq_pace_state_str(&tack.state);
                        println!("### {} ({}) [{}]", tack.silks, coronet_key, state_str);
                        println!();
                        println!("{}", tack.text);
                        if let Some(ref direction) = tack.direction {
                            println!();
                            println!("**Direction:** {}", direction);
                        }
                        println!();
                    }
                }
            }
        }
    }

    0
}

/// Helper to resolve pace by coronet or silks
///
/// First tries to parse pace_arg as a Coronet and lookup by coronet key.
/// If that fails, iterates heat.paces to find a pace where tacks[0].silks matches pace_arg.
/// Returns tuple of (coronet_key, pace_ref) if found.
fn zjjrq_resolve_pace<'a>(
    heat: &'a crate::jjrg_gallops::jjrg_Heat,
    pace_arg: &str,
) -> Option<(&'a String, &'a Pace)> {
    // Try parsing as Coronet first
    if let Ok(coronet) = Coronet::jjrf_parse(pace_arg) {
        let coronet_key = coronet.jjrf_display();
        if let Some((key, pace)) = heat.paces.get_key_value(&coronet_key) {
            return Some((key, pace));
        }
    }

    // Fallback: search by silks
    for (coronet_key, pace) in &heat.paces {
        if let Some(tack) = pace.tacks.first() {
            if tack.silks == pace_arg {
                return Some((coronet_key, pace));
            }
        }
    }

    None
}

/// Helper to convert PaceState to display string
fn zjjrq_pace_state_str(state: &PaceState) -> &'static str {
    match state {
        PaceState::Rough => "rough",
        PaceState::Bridled => "bridled",
        PaceState::Complete => "complete",
        PaceState::Abandoned => "abandoned",
    }
}

// ============================================================================
// Retire - Extract complete Heat data for archival trophy
// ============================================================================

/// Arguments for retire command
#[derive(Debug)]
pub struct jjrq_RetireArgs {
    pub file: std::path::PathBuf,
    pub firemark: Firemark,
}

/// Output structure for retire command
#[derive(Serialize)]
pub(crate) struct zjjrq_RetireOutput {
    pub(crate) firemark: String,
    pub(crate) silks: String,
    pub(crate) created: String,
    pub(crate) status: String,
    pub(crate) paddock_file: String,
    pub(crate) paddock_content: String,
    pub(crate) paces: Vec<zjjrq_RetirePace>,
    // TODO: steeplechase array from jjx_rein
    // steeplechase: Vec<SteeplechaseEntry>,
}

/// Pace structure for retire output (full tack history)
#[derive(Serialize)]
pub(crate) struct zjjrq_RetirePace {
    pub(crate) coronet: String,
    pub(crate) silks: String,
    pub(crate) tacks: Vec<zjjrq_RetireTack>,
}

/// Tack structure for retire output
#[derive(Serialize)]
pub(crate) struct zjjrq_RetireTack {
    pub(crate) ts: String,
    pub(crate) state: String,
    pub(crate) text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) direction: Option<String>,
}

/// Run the retire command - extract complete Heat data for archival
pub fn jjrq_run_retire(args: jjrq_RetireArgs) -> i32 {
    let gallops = match Gallops::jjrg_load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_retire: error: {}", e);
            return 1;
        }
    };

    let heat_key = args.firemark.jjrf_display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            eprintln!("jjx_retire: error: Heat '{}' not found", heat_key);
            return 1;
        }
    };

    // Read paddock file content
    let paddock_content = match fs::read_to_string(&heat.paddock_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("jjx_retire: error reading paddock file '{}': {}", heat.paddock_file, e);
            return 1;
        }
    };

    // Build paces array with full tack history, ordered per Heat's order array
    let mut paces = Vec::new();
    for coronet_key in &heat.order {
        if let Some(pace) = heat.paces.get(coronet_key) {
            let tacks: Vec<zjjrq_RetireTack> = pace.tacks.iter().map(|tack| {
                zjjrq_RetireTack {
                    ts: tack.ts.clone(),
                    state: match tack.state {
                        PaceState::Rough => "rough".to_string(),
                        PaceState::Bridled => "bridled".to_string(),
                        PaceState::Complete => "complete".to_string(),
                        PaceState::Abandoned => "abandoned".to_string(),
                    },
                    text: tack.text.clone(),
                    direction: tack.direction.clone(),
                }
            }).collect();

            paces.push(zjjrq_RetirePace {
                coronet: coronet_key.clone(),
                silks: pace.tacks.first().map(|t| t.silks.clone()).unwrap_or_default(),
                tacks,
            });
        }
    }

    let output = zjjrq_RetireOutput {
        firemark: heat_key.clone(),
        silks: heat.silks.clone(),
        created: heat.creation_time.clone(),
        status: match heat.status {
            HeatStatus::Racing => "racing".to_string(),
            HeatStatus::Stabled => "stabled".to_string(),
            HeatStatus::Retired => "retired".to_string(),
        },
        paddock_file: heat.paddock_file.clone(),
        paddock_content,
        paces,
        // TODO: Call jjx_rein internally to get steeplechase entries
    };

    // Output JSON
    match serde_json::to_string_pretty(&output) {
        Ok(json) => {
            println!("{}", json);
            0
        }
        Err(e) => {
            eprintln!("jjx_retire: error serializing output: {}", e);
            1
        }
    }
}

