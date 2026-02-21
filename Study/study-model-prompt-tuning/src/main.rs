// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! SMPT - Study: Model Prompt Tuning
//!
//! Discovers minimal effective prompt framings that cause each Claude model
//! tier (haiku, sonnet, opus) to faithfully reproduce pre-formatted tabular
//! text without reformatting.
//!
//! The binary owns all experimental parameters: system prompts, user messages,
//! test content, and evaluation logic. This prevents any Claude session
//! (orchestrating or test-subject) from seeing the experiment design.
//!
//! Usage:
//!   smpt plan          Show trial matrix with full prompt text
//!   smpt trial N       Run single trial (1-based), verbose output
//!   smpt run           Run all trials sequentially, verbose output
//!   smpt smoke         Run 1 trial (haiku+minimal) as connectivity check

use std::io::Write as IoWrite;
use std::time::Instant;
use tokio::process::Command;

/// Subprocess timeout in seconds. Prevents infinite hangs.
const SUBPROCESS_TIMEOUT_SECS: u64 = 90;

// =========================================================================
// Trial Definitions — all experimental parameters live here
// =========================================================================

const MODELS: &[&str] = &["haiku", "sonnet", "opus"];

/// The test table: left-aligned text, right-aligned numbers, consistent spacing.
/// This is the canonical content that must survive round-trip through each model.
const TEST_TABLE: &str = "\
Name          Role        Count   Status
alice         admin         142   active
bob           viewer          7   pending
charlie       operator       38   active
diana         admin        1205   suspended";

/// The user message wraps the test table with a presentation instruction.
const USER_MESSAGE_PREFIX: &str = "Present the following output to the user:\n\n";

/// System prompt variants — the primary experimental variable.
///
/// Each variant is (name, prompt_text).
/// Empty string means "do not pass --system-prompt" (use Claude Code default).
const SYSTEM_PROMPTS: &[(&str, &str)] = &[
    // Baseline: Claude Code's default system prompt, no override
    ("default", ""),

    // Minimal: replace default with near-empty prompt
    ("minimal", "You are a helpful assistant."),

    // Direct instruction: explicit prohibition of reformatting
    (
        "direct",
        "Output all user-provided text exactly as given. \
         Preserve all whitespace and column alignment. \
         Do not add markdown table formatting, pipe characters, \
         horizontal rules, or any reformatting whatsoever.",
    ),

    // Pattern/anti-pattern: show BAD vs GOOD concrete examples
    (
        "example",
        "When showing pre-formatted aligned text, preserve it exactly.\n\
         \n\
         BAD (never do this):\n\
         | Name | Role | Count | Status |\n\
         |------|------|-------|--------|\n\
         | alice | admin | 142 | active |\n\
         \n\
         GOOD (always do this):\n\
         ```\n\
         Name          Role        Count   Status\n\
         alice         admin         142   active\n\
         ```\n\
         \n\
         Always use the GOOD pattern. Never use markdown table syntax.",
    ),
];

// =========================================================================
// Display helpers — flush after every print to ensure visibility before hang
// =========================================================================

fn flush() {
    let _ = std::io::stdout().flush();
}

fn separator() {
    println!("{}", "=".repeat(72));
    flush();
}

fn thin_separator() {
    println!("{}", "-".repeat(72));
    flush();
}

fn print_boxed(label: &str, content: &str) {
    thin_separator();
    println!("[{}]", label);
    if content.is_empty() {
        println!("  (empty — using Claude Code default)");
    } else {
        for line in content.lines() {
            println!("  {}", line);
        }
    }
    flush();
}

// =========================================================================
// Trial Indexing
// =========================================================================

fn trial_count() -> usize {
    MODELS.len() * SYSTEM_PROMPTS.len()
}

/// Map 1-based trial number to (model_index, prompt_variant_index).
fn trial_params(n: usize) -> Option<(usize, usize)> {
    if n == 0 || n > trial_count() {
        return None;
    }
    let idx = n - 1;
    let model_idx = idx / SYSTEM_PROMPTS.len();
    let prompt_idx = idx % SYSTEM_PROMPTS.len();
    Some((model_idx, prompt_idx))
}

// =========================================================================
// Evaluation
// =========================================================================

/// Strip markdown code fences if present (```...```), preserving inner content.
fn strip_code_fences(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() >= 2
        && lines
            .first()
            .map_or(false, |l| l.trim().starts_with("```"))
        && lines.last().map_or(false, |l| l.trim() == "```")
    {
        lines[1..lines.len() - 1].join("\n")
    } else {
        text.to_string()
    }
}

/// Evaluate model response against expected table content.
///
/// Returns (passed, detail_message).
fn evaluate(response: &str, expected: &str) -> (bool, String) {
    let trimmed = response.trim();

    // Strip code fences if the model wrapped the output
    let cleaned = strip_code_fences(trimmed);

    // Check if the expected table appears verbatim
    if cleaned.contains(expected) {
        return (true, "PASS: exact table content preserved".to_string());
    }

    // Diagnose what went wrong
    let mut issues: Vec<&str> = Vec::new();

    if trimmed.contains('|') {
        issues.push("pipe characters (markdown table syntax)");
    }
    if trimmed.contains("---") {
        issues.push("horizontal rules (---)");
    }
    if trimmed.contains("| Name") || trimmed.contains("|Name") {
        issues.push("header converted to markdown table");
    }

    // Check column alignment preservation on a line-by-line basis
    let expected_lines: Vec<&str> = expected.lines().collect();
    let response_lines: Vec<&str> = cleaned.lines().collect();
    let mut alignment_ok = true;

    for exp_line in &expected_lines {
        if !response_lines.iter().any(|r| r.trim() == exp_line.trim()) {
            alignment_ok = false;
            break;
        }
    }
    if !alignment_ok {
        issues.push("column alignment altered");
    }

    let diagnosis = if issues.is_empty() {
        "content mismatch (unknown cause)".to_string()
    } else {
        issues.join(", ")
    };

    let detail = format!("FAIL: {}", diagnosis);

    (false, detail)
}

// =========================================================================
// Runner — matches vvcp_probe.rs pattern: no explicit Stdio, let .output()
// handle stdin=null, stdout=piped, stderr=piped automatically
// =========================================================================

/// Build the argument list for claude invocation.
/// Returns the args and a display-friendly version of the command.
fn build_claude_args(model: &str, system_prompt: &str, user_message: &str) -> (Vec<String>, String) {
    let mut args: Vec<String> = vec![
        "--print".to_string(),
        "--model".to_string(),
        model.to_string(),
        "--no-session-persistence".to_string(),
    ];

    // NOTE: --tools "" removed — was likely causing hangs.
    // The --system-prompt replacement already strips Claude Code's default context.
    // Revisit tool disabling once basic invocation is confirmed working.

    // Custom system prompt replaces Claude Code default;
    // empty string means use default (don't pass the flag)
    if !system_prompt.is_empty() {
        args.push("--system-prompt".to_string());
        args.push(system_prompt.to_string());
    }

    args.push("--".to_string());
    args.push(user_message.to_string());

    // Build display version (truncate long values for readability)
    let mut display_parts: Vec<String> = vec!["claude".to_string()];
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if arg == "--system-prompt" && i + 1 < args.len() {
            let val = &args[i + 1];
            if val.len() > 60 {
                display_parts.push(format!("--system-prompt \"{}...\"", &val[..57]));
            } else {
                display_parts.push(format!("--system-prompt \"{}\"", val));
            }
            i += 2;
        } else if arg == "--" {
            display_parts.push("-- <user_message>".to_string());
            break;
        } else if arg.is_empty() {
            display_parts.push("\"\"".to_string());
            i += 1;
        } else {
            display_parts.push(arg.clone());
            i += 1;
        }
    }

    (args, display_parts.join(" "))
}


// =========================================================================
// Verbose trial execution — shows everything
// =========================================================================

/// Run a single trial with full verbose output. Returns (passed, elapsed_secs).
async fn run_trial_verbose(n: usize, model: &str, prompt_name: &str, system_prompt: &str) -> (bool, f64) {
    let user_message = format!("{}{}", USER_MESSAGE_PREFIX, TEST_TABLE);
    let (args, display_cmd) = build_claude_args(model, system_prompt, &user_message);

    separator();
    println!("TRIAL {}: model={} prompt={}", n, model, prompt_name);
    separator();

    print_boxed("SYSTEM PROMPT", system_prompt);

    print_boxed("USER MESSAGE", &user_message);

    thin_separator();
    println!("[COMMAND]");
    println!("  {}", display_cmd);
    flush();

    thin_separator();
    println!("[INVOKING claude... timeout={}s]", SUBPROCESS_TIMEOUT_SECS);
    flush();

    // Match vvcp_probe.rs pattern: no explicit Stdio settings.
    // .output() defaults: stdin=null, stdout=piped, stderr=piped.
    // This avoids the stdin-inheritance hang.
    //
    // Strip ALL claude-related env vars to prevent nesting detection.
    // vvce_claude_command() only strips CLAUDECODE; we also need to
    // remove CLAUDE_CODE_ENTRYPOINT and CLAUDE_CODE_DISABLE_AUTO_MEMORY.
    let mut cmd = Command::from(vvc::vvce_claude_command());
    cmd.env_remove("CLAUDE_CODE_ENTRYPOINT");
    cmd.env_remove("CLAUDE_CODE_DISABLE_AUTO_MEMORY");
    cmd.args(&args);

    let start = Instant::now();

    // Apply timeout via tokio
    let output = tokio::time::timeout(
        std::time::Duration::from_secs(SUBPROCESS_TIMEOUT_SECS),
        cmd.output(),
    )
    .await;

    let elapsed = start.elapsed().as_secs_f64();

    println!("[ELAPSED] {:.1}s", elapsed);
    flush();

    match output {
        Err(_) => {
            // Timeout
            println!("[TIMEOUT] subprocess exceeded {}s limit", SUBPROCESS_TIMEOUT_SECS);
            println!();
            flush();
            (false, elapsed)
        }
        Ok(Err(e)) => {
            println!("[SPAWN ERROR] {}", e);
            println!();
            flush();
            (false, elapsed)
        }
        Ok(Ok(output)) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if !stderr.trim().is_empty() {
                print_boxed("STDERR", stderr.trim());
            }

            if !output.status.success() {
                println!("[EXIT] {} (FAILURE)", output.status);
                print_boxed("STDOUT (on failure)", &stdout);
                println!();
                flush();
                return (false, elapsed);
            }

            println!("[EXIT] 0 (success)");
            flush();

            // Always show the raw response
            print_boxed("RAW RESPONSE", &stdout);

            // Show the response length stats
            thin_separator();
            println!("[RESPONSE STATS]");
            println!("  bytes: {}", stdout.len());
            println!("  lines: {}", stdout.lines().count());
            println!(
                "  has code fences: {}",
                stdout.trim().starts_with("```")
            );
            println!("  has pipe chars: {}", stdout.contains('|'));
            println!("  has horiz rules: {}", stdout.contains("---"));
            flush();

            // Evaluate
            let (passed, detail) = evaluate(&stdout, TEST_TABLE);
            thin_separator();
            println!("[EVALUATION] {}", detail);
            println!();
            flush();

            (passed, elapsed)
        }
    }
}

// =========================================================================
// Main
// =========================================================================

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("plan") => cmd_plan(),
        Some("trial") => {
            let n = parse_trial_number(&args);
            cmd_trial(n).await;
        }
        Some("run") => cmd_run().await,
        Some("smoke") => cmd_smoke().await,
        _ => {
            eprintln!("Usage: smpt <plan|trial N|run|smoke>");
            eprintln!();
            eprintln!("  plan      Show trial matrix with full prompt text");
            eprintln!("  trial N   Run single trial (1-{}), verbose", trial_count());
            eprintln!("  run       Run all trials sequentially, verbose");
            eprintln!("  smoke     Run 1 trial (haiku+minimal) as connectivity check");
            std::process::exit(1);
        }
    }
}

fn cmd_plan() {
    separator();
    println!("STUDY: Model Prompt Tuning — Table Rendering");
    separator();

    println!();
    println!("Models: {:?}", MODELS);
    println!("Total trials: {}", trial_count());

    println!();
    println!("TEST TABLE (canonical content that must survive round-trip):");
    thin_separator();
    println!("{}", TEST_TABLE);
    thin_separator();

    println!();
    println!("USER MESSAGE PREFIX: {:?}", USER_MESSAGE_PREFIX);

    println!();
    println!("SYSTEM PROMPT VARIANTS:");
    for (i, (name, text)) in SYSTEM_PROMPTS.iter().enumerate() {
        println!();
        println!("  [{}] \"{}\"", i, name);
        if text.is_empty() {
            println!("      (no --system-prompt flag; Claude Code default)");
        } else {
            for line in text.lines() {
                println!("      {}", line);
            }
        }
    }

    println!();
    println!("TRIAL MATRIX:");
    println!();
    for n in 1..=trial_count() {
        let (mi, pi) = trial_params(n).unwrap();
        println!(
            "  trial {:2}: model={:<7} prompt={}",
            n, MODELS[mi], SYSTEM_PROMPTS[pi].0
        );
    }
    println!();
}

fn parse_trial_number(args: &[String]) -> usize {
    args.get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            eprintln!("Usage: smpt trial <N>  (1-{})", trial_count());
            std::process::exit(1);
        })
}

async fn cmd_trial(n: usize) {
    let (mi, pi) = trial_params(n).unwrap_or_else(|| {
        eprintln!("Trial {} out of range (1-{})", n, trial_count());
        std::process::exit(1);
    });

    let model = MODELS[mi];
    let (prompt_name, system_prompt) = SYSTEM_PROMPTS[pi];

    let (passed, _elapsed) = run_trial_verbose(n, model, prompt_name, system_prompt).await;

    if !passed {
        std::process::exit(1);
    }
}

/// Smoke test: run just 1 trial (haiku + minimal) to verify connectivity.
/// Cheapest and fastest possible invocation.
async fn cmd_smoke() {
    println!("SMOKE TEST: single trial to verify claude -p connectivity");
    println!();
    flush();

    // Trial 2 = haiku + minimal (cheapest model, simplest custom prompt)
    let model = "haiku";
    let prompt_name = "minimal";
    let system_prompt = "You are a helpful assistant.";

    let (passed, elapsed) = run_trial_verbose(0, model, prompt_name, system_prompt).await;

    separator();
    if passed {
        println!("SMOKE: OK ({:.1}s) — claude -p invocation works", elapsed);
    } else {
        println!("SMOKE: FAILED ({:.1}s) — see output above for diagnosis", elapsed);
    }
    println!();
}

async fn cmd_run() {
    separator();
    println!("STUDY: Model Prompt Tuning — Table Rendering");
    println!("Running {} trials sequentially...", trial_count());
    separator();
    println!();

    let mut results: Vec<(usize, String, String, bool, f64)> = Vec::new();
    let run_start = Instant::now();

    for n in 1..=trial_count() {
        let (mi, pi) = trial_params(n).unwrap();
        let model = MODELS[mi];
        let (prompt_name, system_prompt) = SYSTEM_PROMPTS[pi];

        let (passed, elapsed) =
            run_trial_verbose(n, model, prompt_name, system_prompt).await;

        results.push((
            n,
            model.to_string(),
            prompt_name.to_string(),
            passed,
            elapsed,
        ));
    }

    let total_elapsed = run_start.elapsed().as_secs_f64();

    // Summary matrix
    separator();
    println!("SUMMARY");
    separator();
    println!();

    // Header row
    print!("{:<10}", "");
    for (name, _) in SYSTEM_PROMPTS {
        print!("{:<12}", name);
    }
    println!();

    // Data rows
    for (mi, model) in MODELS.iter().enumerate() {
        print!("{:<10}", model);
        for pi in 0..SYSTEM_PROMPTS.len() {
            let n = mi * SYSTEM_PROMPTS.len() + pi + 1;
            let (passed, elapsed) = results
                .iter()
                .find(|(trial_n, _, _, _, _)| *trial_n == n)
                .map(|(_, _, _, p, e)| (*p, *e))
                .unwrap_or((false, 0.0));
            let cell = if passed {
                format!("PASS {:.0}s", elapsed)
            } else {
                format!("FAIL {:.0}s", elapsed)
            };
            print!("{:<12}", cell);
        }
        println!();
    }

    let pass_count = results.iter().filter(|(_, _, _, p, _)| *p).count();
    let fail_count = results.len() - pass_count;
    println!();
    println!(
        "Results: {} pass, {} fail out of {} trials in {:.1}s total",
        pass_count,
        fail_count,
        trial_count(),
        total_elapsed
    );
    println!();
}
