//! Query operations for Gallops JSON
//!
//! Implements read operations: muster, saddle, parade, retire.
//! All operations read from Gallops JSON and optionally paddock files.

use crate::jjrf_favor::Firemark;
use crate::jjrg_gallops::{Gallops, HeatStatus, PaceState};
use serde::Serialize;
use std::fs;

// ============================================================================
// Muster - List all Heats with summary information
// ============================================================================

/// Arguments for muster command
#[derive(Debug)]
pub struct MusterArgs {
    pub file: std::path::PathBuf,
    pub status: Option<HeatStatus>,
}

/// Run the muster command - list Heats as TSV
pub fn run_muster(args: MusterArgs) -> i32 {
    let gallops = match Gallops::load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_muster: error: {}", e);
            return 1;
        }
    };

    for (key, heat) in &gallops.heats {
        // Apply status filter if provided
        if let Some(ref filter_status) = args.status {
            if &heat.status != filter_status {
                continue;
            }
        }

        let pace_count = heat.paces.len();
        let status_str = match heat.status {
            HeatStatus::Current => "current",
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
pub struct SaddleArgs {
    pub file: std::path::PathBuf,
    pub firemark: Firemark,
}

/// Output structure for saddle command
#[derive(Serialize)]
struct SaddleOutput {
    heat_silks: String,
    paddock_file: String,
    paddock_content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pace_coronet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pace_silks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pace_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tack_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tack_direction: Option<String>,
}

/// Run the saddle command - return Heat context
pub fn run_saddle(args: SaddleArgs) -> i32 {
    let gallops = match Gallops::load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_saddle: error: {}", e);
            return 1;
        }
    };

    let heat_key = args.firemark.display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            eprintln!("jjx_saddle: error: Heat '{}' not found", heat_key);
            return 1;
        }
    };

    // Read paddock file content
    let paddock_content = match fs::read_to_string(&heat.paddock_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("jjx_saddle: error reading paddock file '{}': {}", heat.paddock_file, e);
            return 1;
        }
    };

    // Find first actionable pace (rough or primed)
    let mut output = SaddleOutput {
        heat_silks: heat.silks.clone(),
        paddock_file: heat.paddock_file.clone(),
        paddock_content,
        pace_coronet: None,
        pace_silks: None,
        pace_state: None,
        tack_text: None,
        tack_direction: None,
    };

    for coronet_key in &heat.order {
        if let Some(pace) = heat.paces.get(coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                match tack.state {
                    PaceState::Rough | PaceState::Primed => {
                        output.pace_coronet = Some(coronet_key.clone());
                        output.pace_silks = Some(pace.silks.clone());
                        output.pace_state = Some(match tack.state {
                            PaceState::Rough => "rough".to_string(),
                            PaceState::Primed => "primed".to_string(),
                            _ => unreachable!(),
                        });
                        output.tack_text = Some(tack.text.clone());
                        if tack.state == PaceState::Primed {
                            output.tack_direction = tack.direction.clone();
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

/// Arguments for parade command
#[derive(Debug)]
pub struct ParadeArgs {
    pub file: std::path::PathBuf,
    pub firemark: Firemark,
    pub full: bool,
}

/// Output structure for parade command
#[derive(Serialize)]
struct ParadeOutput {
    heat_silks: String,
    heat_created: String,
    heat_status: String,
    paddock_file: String,
    paddock_content: String,
    paces: Vec<ParadePace>,
}

/// Pace structure for parade output
#[derive(Serialize)]
struct ParadePace {
    coronet: String,
    silks: String,
    state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tack_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tack_direction: Option<String>,
}

/// Run the parade command - display comprehensive Heat status
pub fn run_parade(args: ParadeArgs) -> i32 {
    let gallops = match Gallops::load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_parade: error: {}", e);
            return 1;
        }
    };

    let heat_key = args.firemark.display();
    let heat = match gallops.heats.get(&heat_key) {
        Some(h) => h,
        None => {
            eprintln!("jjx_parade: error: Heat '{}' not found", heat_key);
            return 1;
        }
    };

    // Read paddock file content
    let paddock_content = match fs::read_to_string(&heat.paddock_file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("jjx_parade: error reading paddock file '{}': {}", heat.paddock_file, e);
            return 1;
        }
    };

    // Build paces array
    let mut paces = Vec::new();
    for coronet_key in &heat.order {
        if let Some(pace) = heat.paces.get(coronet_key) {
            if let Some(tack) = pace.tacks.first() {
                let state_str = match tack.state {
                    PaceState::Rough => "rough",
                    PaceState::Primed => "primed",
                    PaceState::Complete => "complete",
                    PaceState::Abandoned => "abandoned",
                };

                let mut parade_pace = ParadePace {
                    coronet: coronet_key.clone(),
                    silks: pace.silks.clone(),
                    state: state_str.to_string(),
                    tack_text: None,
                    tack_direction: None,
                };

                // Include tack_text for rough/primed, or for complete/abandoned with --full
                match tack.state {
                    PaceState::Rough | PaceState::Primed => {
                        parade_pace.tack_text = Some(tack.text.clone());
                        if tack.state == PaceState::Primed {
                            parade_pace.tack_direction = tack.direction.clone();
                        }
                    }
                    PaceState::Complete | PaceState::Abandoned => {
                        if args.full {
                            parade_pace.tack_text = Some(tack.text.clone());
                        }
                    }
                }

                paces.push(parade_pace);
            }
        }
    }

    let output = ParadeOutput {
        heat_silks: heat.silks.clone(),
        heat_created: heat.creation_time.clone(),
        heat_status: match heat.status {
            HeatStatus::Current => "current".to_string(),
            HeatStatus::Retired => "retired".to_string(),
        },
        paddock_file: heat.paddock_file.clone(),
        paddock_content,
        paces,
    };

    // Output JSON
    match serde_json::to_string_pretty(&output) {
        Ok(json) => {
            println!("{}", json);
            0
        }
        Err(e) => {
            eprintln!("jjx_parade: error serializing output: {}", e);
            1
        }
    }
}

// ============================================================================
// Retire - Extract complete Heat data for archival trophy
// ============================================================================

/// Arguments for retire command
#[derive(Debug)]
pub struct RetireArgs {
    pub file: std::path::PathBuf,
    pub firemark: Firemark,
}

/// Output structure for retire command
#[derive(Serialize)]
struct RetireOutput {
    firemark: String,
    silks: String,
    created: String,
    status: String,
    paddock_file: String,
    paddock_content: String,
    paces: Vec<RetirePace>,
    // TODO: steeplechase array from jjx_rein
    // steeplechase: Vec<SteeplechaseEntry>,
}

/// Pace structure for retire output (full tack history)
#[derive(Serialize)]
struct RetirePace {
    coronet: String,
    silks: String,
    tacks: Vec<RetireTack>,
}

/// Tack structure for retire output
#[derive(Serialize)]
struct RetireTack {
    ts: String,
    state: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<String>,
}

/// Run the retire command - extract complete Heat data for archival
pub fn run_retire(args: RetireArgs) -> i32 {
    let gallops = match Gallops::load(&args.file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("jjx_retire: error: {}", e);
            return 1;
        }
    };

    let heat_key = args.firemark.display();
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
            let tacks: Vec<RetireTack> = pace.tacks.iter().map(|tack| {
                RetireTack {
                    ts: tack.ts.clone(),
                    state: match tack.state {
                        PaceState::Rough => "rough".to_string(),
                        PaceState::Primed => "primed".to_string(),
                        PaceState::Complete => "complete".to_string(),
                        PaceState::Abandoned => "abandoned".to_string(),
                    },
                    text: tack.text.clone(),
                    direction: tack.direction.clone(),
                }
            }).collect();

            paces.push(RetirePace {
                coronet: coronet_key.clone(),
                silks: pace.silks.clone(),
                tacks,
            });
        }
    }

    let output = RetireOutput {
        firemark: heat_key.clone(),
        silks: heat.silks.clone(),
        created: heat.creation_time.clone(),
        status: match heat.status {
            HeatStatus::Current => "current".to_string(),
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

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jjrg_gallops::{Heat, Pace, Tack};
    use std::collections::HashMap;

    fn create_test_gallops() -> Gallops {
        let mut paces = HashMap::new();
        paces.insert(
            "₢ABAAA".to_string(),
            Pace {
                silks: "test-pace-one".to_string(),
                tacks: vec![Tack {
                    ts: "260101-1200".to_string(),
                    state: PaceState::Rough,
                    text: "First pace rough plan".to_string(),
                    direction: None,
                }],
            },
        );
        paces.insert(
            "₢ABAAB".to_string(),
            Pace {
                silks: "test-pace-two".to_string(),
                tacks: vec![Tack {
                    ts: "260101-1300".to_string(),
                    state: PaceState::Complete,
                    text: "Completed pace".to_string(),
                    direction: None,
                }],
            },
        );

        let mut heats = HashMap::new();
        heats.insert(
            "₣AB".to_string(),
            Heat {
                silks: "test-heat".to_string(),
                creation_time: "260101".to_string(),
                status: HeatStatus::Current,
                order: vec!["₢ABAAA".to_string(), "₢ABAAB".to_string()],
                next_pace_seed: "AAC".to_string(),
                paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
                paces,
            },
        );

        Gallops {
            next_heat_seed: "AC".to_string(),
            heats,
        }
    }

    #[test]
    fn test_muster_output_format() {
        // This test validates the TSV format conceptually
        // Full integration test would need file I/O
        let gallops = create_test_gallops();
        let heat = gallops.heats.get("₣AB").unwrap();
        let expected_format = format!(
            "₣AB\t{}\tcurrent\t{}",
            heat.silks,
            heat.paces.len()
        );
        assert!(expected_format.contains("test-heat"));
        assert!(expected_format.contains("2")); // pace count
    }

    #[test]
    fn test_saddle_output_structure() {
        // Test the SaddleOutput serialization
        let output = SaddleOutput {
            heat_silks: "my-heat".to_string(),
            paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
            paddock_content: "# Test content".to_string(),
            pace_coronet: Some("₢ABAAA".to_string()),
            pace_silks: Some("my-pace".to_string()),
            pace_state: Some("rough".to_string()),
            tack_text: Some("Do the thing".to_string()),
            tack_direction: None,
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("heat_silks"));
        assert!(json.contains("pace_coronet"));
        assert!(!json.contains("tack_direction")); // None should be skipped
    }

    #[test]
    fn test_saddle_output_with_primed_direction() {
        let output = SaddleOutput {
            heat_silks: "my-heat".to_string(),
            paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
            paddock_content: "# Test".to_string(),
            pace_coronet: Some("₢ABAAA".to_string()),
            pace_silks: Some("my-pace".to_string()),
            pace_state: Some("primed".to_string()),
            tack_text: Some("Ready to execute".to_string()),
            tack_direction: Some("Execute autonomously".to_string()),
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("tack_direction"));
        assert!(json.contains("Execute autonomously"));
    }

    #[test]
    fn test_saddle_output_no_actionable_pace() {
        let output = SaddleOutput {
            heat_silks: "my-heat".to_string(),
            paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
            paddock_content: "# All done".to_string(),
            pace_coronet: None,
            pace_silks: None,
            pace_state: None,
            tack_text: None,
            tack_direction: None,
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("heat_silks"));
        assert!(!json.contains("pace_coronet"));
        assert!(!json.contains("pace_silks"));
    }

    #[test]
    fn test_parade_output_structure() {
        let output = ParadeOutput {
            heat_silks: "my-heat".to_string(),
            heat_created: "260101".to_string(),
            heat_status: "current".to_string(),
            paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
            paddock_content: "# Heat context".to_string(),
            paces: vec![
                ParadePace {
                    coronet: "₢ABAAA".to_string(),
                    silks: "first-pace".to_string(),
                    state: "rough".to_string(),
                    tack_text: Some("Plan here".to_string()),
                    tack_direction: None,
                },
                ParadePace {
                    coronet: "₢ABAAB".to_string(),
                    silks: "done-pace".to_string(),
                    state: "complete".to_string(),
                    tack_text: None,
                    tack_direction: None,
                },
            ],
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("heat_created"));
        assert!(json.contains("heat_status"));
        assert!(json.contains("paces"));
        assert!(json.contains("first-pace"));
    }

    #[test]
    fn test_retire_output_structure() {
        let output = RetireOutput {
            firemark: "₣AB".to_string(),
            silks: "my-heat".to_string(),
            created: "260101".to_string(),
            status: "current".to_string(),
            paddock_file: ".claude/jjm/jjp_AB.md".to_string(),
            paddock_content: "# Archive this".to_string(),
            paces: vec![RetirePace {
                coronet: "₢ABAAA".to_string(),
                silks: "test-pace".to_string(),
                tacks: vec![
                    RetireTack {
                        ts: "260101-1400".to_string(),
                        state: "complete".to_string(),
                        text: "Final plan".to_string(),
                        direction: None,
                    },
                    RetireTack {
                        ts: "260101-1200".to_string(),
                        state: "rough".to_string(),
                        text: "Initial plan".to_string(),
                        direction: None,
                    },
                ],
            }],
        };
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("firemark"));
        assert!(json.contains("₣AB"));
        assert!(json.contains("tacks"));
        // Verify tack history is included
        assert!(json.contains("260101-1400"));
        assert!(json.contains("260101-1200"));
    }

    #[test]
    fn test_retire_tack_with_direction() {
        let tack = RetireTack {
            ts: "260101-1200".to_string(),
            state: "primed".to_string(),
            text: "Ready to fly".to_string(),
            direction: Some("Execute with agent X".to_string()),
        };
        let json = serde_json::to_string(&tack).unwrap();
        assert!(json.contains("direction"));
        assert!(json.contains("Execute with agent X"));
    }

    #[test]
    fn test_heat_status_filter_current() {
        let gallops = create_test_gallops();
        let heat = gallops.heats.get("₣AB").unwrap();
        assert_eq!(heat.status, HeatStatus::Current);

        // Simulate filter
        let filter = Some(HeatStatus::Current);
        let matches = filter.as_ref().map_or(true, |f| &heat.status == f);
        assert!(matches);
    }

    #[test]
    fn test_heat_status_filter_retired() {
        let gallops = create_test_gallops();
        let heat = gallops.heats.get("₣AB").unwrap();

        // Simulate filter for retired (should not match)
        let filter = Some(HeatStatus::Retired);
        let matches = filter.as_ref().map_or(true, |f| &heat.status == f);
        assert!(!matches);
    }

    #[test]
    fn test_pace_state_to_string() {
        assert_eq!(
            match PaceState::Rough {
                PaceState::Rough => "rough",
                PaceState::Primed => "primed",
                PaceState::Complete => "complete",
                PaceState::Abandoned => "abandoned",
            },
            "rough"
        );
    }

    #[test]
    fn test_find_first_actionable_pace() {
        let gallops = create_test_gallops();
        let heat = gallops.heats.get("₣AB").unwrap();

        // Find first actionable pace in order
        let mut found_coronet: Option<String> = None;
        for coronet_key in &heat.order {
            if let Some(pace) = heat.paces.get(coronet_key) {
                if let Some(tack) = pace.tacks.first() {
                    match tack.state {
                        PaceState::Rough | PaceState::Primed => {
                            found_coronet = Some(coronet_key.clone());
                            break;
                        }
                        _ => continue,
                    }
                }
            }
        }

        assert_eq!(found_coronet, Some("₢ABAAA".to_string()));
    }
}
