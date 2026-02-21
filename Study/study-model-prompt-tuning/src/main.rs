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
//!   smpt plan            Show trial matrix with full prompt text
//!   smpt trial N [R]     Run single trial (1-based), R repeats (default 1)
//!   smpt run [R]         Run all trials, R repeats each (default 1)
//!   smpt smoke           Run 1 trial (haiku+minimal) as connectivity check

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
// Statistics
// =========================================================================

fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = sorted.len();
    if n == 0 {
        return 0.0;
    }
    if n % 2 == 1 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    }
}

fn median_u64(values: &[u64]) -> u64 {
    let mut sorted = values.to_vec();
    sorted.sort();
    let n = sorted.len();
    if n == 0 {
        return 0;
    }
    if n % 2 == 1 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2
    }
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
        "--output-format".to_string(),
        "json".to_string(),
    ];

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
// Core invocation — no printing, returns structured result
// =========================================================================

/// Result of a single claude invocation.
struct InvokeResult {
    passed: bool,
    elapsed: f64,
    detail: String,
    response: String,
    input_tokens: u64,
    output_tokens: u64,
    cache_read_tokens: u64,
    cost_usd: f64,
    duration_api_ms: u64,
    raw_json: Option<serde_json::Value>,
}

/// Parsed token/cost data from JSON response.
struct JsonParsed {
    result_text: String,
    input_tokens: u64,
    output_tokens: u64,
    cache_read_tokens: u64,
    cost_usd: f64,
    duration_api_ms: u64,
    json: serde_json::Value,
}

/// Parse JSON response from `claude --output-format json`.
/// Falls back to raw stdout if JSON parsing fails.
fn parse_json_response(stdout: &str) -> Result<JsonParsed, String> {
    let json: serde_json::Value = serde_json::from_str(stdout)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    let result_text = json.get("result")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let usage = json.get("usage");

    let input_tokens = usage
        .and_then(|u| u.get("input_tokens"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let output_tokens = usage
        .and_then(|u| u.get("output_tokens"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let cache_read_tokens = usage
        .and_then(|u| u.get("cache_read_input_tokens"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let duration_api_ms = json.get("duration_api_ms")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let cost_usd = json.get("total_cost_usd")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    Ok(JsonParsed {
        result_text,
        input_tokens,
        output_tokens,
        cache_read_tokens,
        cost_usd,
        duration_api_ms,
        json,
    })
}

/// Invoke claude subprocess and evaluate response. No output printed.
async fn invoke_trial(model: &str, system_prompt: &str) -> InvokeResult {
    let user_message = format!("{}{}", USER_MESSAGE_PREFIX, TEST_TABLE);
    let (args, _display_cmd) = build_claude_args(model, system_prompt, &user_message);

    let mut cmd = Command::from(vvc::vvce_claude_command());
    cmd.env_remove("CLAUDE_CODE_ENTRYPOINT");
    cmd.env_remove("CLAUDE_CODE_DISABLE_AUTO_MEMORY");
    cmd.args(&args);

    let start = Instant::now();

    let output = tokio::time::timeout(
        std::time::Duration::from_secs(SUBPROCESS_TIMEOUT_SECS),
        cmd.output(),
    )
    .await;

    let elapsed = start.elapsed().as_secs_f64();

    let empty_result = |detail: String| InvokeResult {
        passed: false,
        elapsed,
        detail,
        response: String::new(),
        input_tokens: 0,
        output_tokens: 0,
        cache_read_tokens: 0,
        cost_usd: 0.0,
        duration_api_ms: 0,
        raw_json: None,
    };

    match output {
        Err(_) => empty_result(format!("TIMEOUT after {}s", SUBPROCESS_TIMEOUT_SECS)),
        Ok(Err(e)) => empty_result(format!("SPAWN ERROR: {}", e)),
        Ok(Ok(output)) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();

            if !output.status.success() {
                return empty_result(format!("EXIT {}", output.status));
            }

            match parse_json_response(&stdout) {
                Ok(parsed) => {
                    let (passed, detail) = evaluate(&parsed.result_text, TEST_TABLE);
                    InvokeResult {
                        passed,
                        elapsed,
                        detail,
                        response: parsed.result_text,
                        input_tokens: parsed.input_tokens,
                        output_tokens: parsed.output_tokens,
                        cache_read_tokens: parsed.cache_read_tokens,
                        cost_usd: parsed.cost_usd,
                        duration_api_ms: parsed.duration_api_ms,
                        raw_json: Some(parsed.json),
                    }
                }
                Err(e) => {
                    // JSON parse failed — evaluate raw stdout
                    let (passed, detail) = evaluate(&stdout, TEST_TABLE);
                    InvokeResult {
                        passed,
                        elapsed,
                        detail: format!("{} ({})", detail, e),
                        response: stdout,
                        input_tokens: 0,
                        output_tokens: 0,
                        cache_read_tokens: 0,
                        cost_usd: 0.0,
                        duration_api_ms: 0,
                        raw_json: None,
                    }
                }
            }
        }
    }
}

// =========================================================================
// Verbose trial execution — shows everything (single invocation)
// =========================================================================

/// Run a single trial with full verbose output. Returns (passed, elapsed_secs).
async fn run_trial_verbose(n: usize, model: &str, prompt_name: &str, system_prompt: &str) -> (bool, f64) {
    let user_message = format!("{}{}", USER_MESSAGE_PREFIX, TEST_TABLE);
    let (_args, display_cmd) = build_claude_args(model, system_prompt, &user_message);

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

    let result = invoke_trial(model, system_prompt).await;

    println!("[ELAPSED] {:.1}s", result.elapsed);
    flush();

    if result.response.is_empty() {
        println!("[{}]", result.detail);
        println!();
        flush();
    } else {
        println!("[EXIT] 0 (success)");
        flush();

        print_boxed("RAW RESPONSE", &result.response);

        thin_separator();
        println!("[RESPONSE STATS]");
        println!("  bytes: {}", result.response.len());
        println!("  lines: {}", result.response.lines().count());
        println!(
            "  has code fences: {}",
            result.response.trim().starts_with("```")
        );
        println!("  has pipe chars: {}", result.response.contains('|'));
        println!("  has horiz rules: {}", result.response.contains("---"));
        println!("  input_tokens: {}", result.input_tokens);
        println!("  cache_read_tokens: {}", result.cache_read_tokens);
        println!("  output_tokens: {}", result.output_tokens);
        println!("  cost_usd: ${:.6}", result.cost_usd);
        println!("  duration_api_ms: {}", result.duration_api_ms);
        flush();

        // Show full JSON envelope if available
        if let Some(ref json) = result.raw_json {
            print_boxed("JSON ENVELOPE", &serde_json::to_string_pretty(json).unwrap_or_default());
        }

        thin_separator();
        println!("[EVALUATION] {}", result.detail);
        println!();
        flush();
    }

    (result.passed, result.elapsed)
}

// =========================================================================
// Repeat trial execution — condensed per-repeat output
// =========================================================================

/// Per-trial aggregate across repeats.
struct TrialAggregate {
    trial_n: usize,
    #[allow(dead_code)]
    model: String,
    #[allow(dead_code)]
    prompt_name: String,
    pass_count: usize,
    repeat_count: usize,
    median_secs: f64,
    min_secs: f64,
    max_secs: f64,
    times: Vec<f64>,
    median_input_tokens: u64,
    median_output_tokens: u64,
    median_cache_read_tokens: u64,
    total_cost_usd: f64,
    median_api_ms: u64,
}

/// Run a trial R times with condensed output per repeat.
async fn run_trial_repeats(
    n: usize,
    model: &str,
    prompt_name: &str,
    system_prompt: &str,
    repeats: usize,
) -> TrialAggregate {
    let user_message = format!("{}{}", USER_MESSAGE_PREFIX, TEST_TABLE);
    let (_args, display_cmd) = build_claude_args(model, system_prompt, &user_message);

    separator();
    println!(
        "TRIAL {}: model={} prompt={} repeats={}",
        n, model, prompt_name, repeats
    );
    separator();

    print_boxed("SYSTEM PROMPT", system_prompt);

    thin_separator();
    println!("[COMMAND]");
    println!("  {}", display_cmd);
    flush();

    println!();

    let mut pass_count: usize = 0;
    let mut times: Vec<f64> = Vec::new();
    let mut input_tokens_vec: Vec<u64> = Vec::new();
    let mut output_tokens_vec: Vec<u64> = Vec::new();
    let mut cache_read_vec: Vec<u64> = Vec::new();
    let mut api_ms_vec: Vec<u64> = Vec::new();
    let mut total_cost: f64 = 0.0;

    for r in 1..=repeats {
        print!("  [rep {}/{}] ", r, repeats);
        flush();

        let result = invoke_trial(model, system_prompt).await;
        times.push(result.elapsed);
        input_tokens_vec.push(result.input_tokens);
        output_tokens_vec.push(result.output_tokens);
        cache_read_vec.push(result.cache_read_tokens);
        api_ms_vec.push(result.duration_api_ms);
        total_cost += result.cost_usd;

        if result.passed {
            pass_count += 1;
            println!(
                "PASS  {:.2}s  in:{} cache:{} out:{} ${:.4}",
                result.elapsed, result.input_tokens, result.cache_read_tokens,
                result.output_tokens, result.cost_usd
            );
        } else {
            let reason = result
                .detail
                .strip_prefix("FAIL: ")
                .unwrap_or(&result.detail);
            println!(
                "FAIL  {:.2}s  in:{} cache:{} out:{} ${:.4}  ({})",
                result.elapsed, result.input_tokens, result.cache_read_tokens,
                result.output_tokens, result.cost_usd, reason
            );
        }
        flush();
    }

    let median_secs = median(&times);
    let min_secs = times.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_secs = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let median_input = median_u64(&input_tokens_vec);
    let median_output = median_u64(&output_tokens_vec);
    let median_cache = median_u64(&cache_read_vec);
    let median_api = median_u64(&api_ms_vec);

    println!();
    println!(
        "  RESULT: {}/{} pass | median {:.2}s (api {}ms) | range {:.2}s - {:.2}s",
        pass_count, repeats, median_secs, median_api, min_secs, max_secs
    );
    println!(
        "  TOKENS: in:{} cache:{} out:{} | total cost ${:.4}",
        median_input, median_cache, median_output, total_cost
    );
    println!();
    flush();

    TrialAggregate {
        trial_n: n,
        model: model.to_string(),
        prompt_name: prompt_name.to_string(),
        pass_count,
        repeat_count: repeats,
        median_secs,
        min_secs,
        max_secs,
        times,
        median_input_tokens: median_input,
        median_output_tokens: median_output,
        median_cache_read_tokens: median_cache,
        total_cost_usd: total_cost,
        median_api_ms: median_api,
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
            let repeats = parse_repeat_count(&args, 3);
            cmd_trial(n, repeats).await;
        }
        Some("run") => {
            let repeats = parse_repeat_count(&args, 2);
            cmd_run(repeats).await;
        }
        Some("smoke") => cmd_smoke().await,
        _ => {
            eprintln!("Usage: smpt <plan|trial N [R]|run [R]|smoke>");
            eprintln!();
            eprintln!("  plan        Show trial matrix with full prompt text");
            eprintln!(
                "  trial N [R] Run single trial (1-{}), R repeats (default 1)",
                trial_count()
            );
            eprintln!("  run [R]     All trials, R repeats each (default 1)");
            eprintln!("  smoke       Run 1 trial (haiku+minimal) as connectivity check");
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
            eprintln!("Usage: smpt trial <N> [R]  (N: 1-{}, R: repeats)", trial_count());
            std::process::exit(1);
        })
}

/// Parse optional repeat count from args at given position.
/// For `trial N R`: repeat is at index 3.
/// For `run R`: repeat is at index 2.
fn parse_repeat_count(args: &[String], pos: usize) -> usize {
    args.get(pos)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
        .max(1)
}

async fn cmd_trial(n: usize, repeats: usize) {
    let (mi, pi) = trial_params(n).unwrap_or_else(|| {
        eprintln!("Trial {} out of range (1-{})", n, trial_count());
        std::process::exit(1);
    });

    let model = MODELS[mi];
    let (prompt_name, system_prompt) = SYSTEM_PROMPTS[pi];

    if repeats == 1 {
        // Single run: full verbose output (backward compatible)
        let (passed, _elapsed) = run_trial_verbose(n, model, prompt_name, system_prompt).await;
        if !passed {
            std::process::exit(1);
        }
    } else {
        let agg = run_trial_repeats(n, model, prompt_name, system_prompt, repeats).await;
        if agg.pass_count == 0 {
            std::process::exit(1);
        }
    }
}

/// Smoke test: run just 1 trial (haiku + minimal) to verify connectivity.
async fn cmd_smoke() {
    println!("SMOKE TEST: single trial to verify claude -p connectivity");
    println!();
    flush();

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

async fn cmd_run(repeats: usize) {
    separator();
    println!("STUDY: Model Prompt Tuning — Table Rendering");
    if repeats > 1 {
        println!(
            "Running {} trials x {} repeats = {} invocations...",
            trial_count(),
            repeats,
            trial_count() * repeats
        );
    } else {
        println!("Running {} trials sequentially...", trial_count());
    }
    separator();
    println!();

    let run_start = Instant::now();

    if repeats == 1 {
        // Single repeat: use verbose output (backward compatible)
        let mut results: Vec<(usize, String, String, bool, f64)> = Vec::new();

        for n in 1..=trial_count() {
            let (mi, pi) = trial_params(n).unwrap();
            let model = MODELS[mi];
            let (prompt_name, system_prompt) = SYSTEM_PROMPTS[pi];
            let (passed, elapsed) =
                run_trial_verbose(n, model, prompt_name, system_prompt).await;
            results.push((n, model.to_string(), prompt_name.to_string(), passed, elapsed));
        }

        let total_elapsed = run_start.elapsed().as_secs_f64();
        print_summary_single(&results, total_elapsed);
    } else {
        // Multiple repeats: condensed output + statistical summary
        let mut aggregates: Vec<TrialAggregate> = Vec::new();

        for n in 1..=trial_count() {
            let (mi, pi) = trial_params(n).unwrap();
            let model = MODELS[mi];
            let (prompt_name, system_prompt) = SYSTEM_PROMPTS[pi];
            let agg =
                run_trial_repeats(n, model, prompt_name, system_prompt, repeats).await;
            aggregates.push(agg);
        }

        let total_elapsed = run_start.elapsed().as_secs_f64();
        print_summary_repeats(&aggregates, repeats, total_elapsed);
    }
}

// =========================================================================
// Summary display
// =========================================================================

fn print_summary_single(results: &[(usize, String, String, bool, f64)], total_elapsed: f64) {
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
        results.len(),
        total_elapsed
    );
    println!();
}

fn print_summary_repeats(aggregates: &[TrialAggregate], repeats: usize, total_elapsed: f64) {
    separator();
    println!("SUMMARY ({} repeats per trial, times are median)", repeats);
    separator();
    println!();

    // Header row — wider columns for "N/N Xs" format
    print!("{:<10}", "");
    for (name, _) in SYSTEM_PROMPTS {
        print!("{:<14}", name);
    }
    println!();

    // Data rows
    for (mi, model) in MODELS.iter().enumerate() {
        print!("{:<10}", model);
        for pi in 0..SYSTEM_PROMPTS.len() {
            let n = mi * SYSTEM_PROMPTS.len() + pi + 1;
            if let Some(agg) = aggregates.iter().find(|a| a.trial_n == n) {
                let cell = format!(
                    "{}/{} {:.1}s",
                    agg.pass_count, agg.repeat_count, agg.median_secs
                );
                print!("{:<14}", cell);
            } else {
                print!("{:<14}", "---");
            }
        }
        println!();
    }

    // Timing detail table
    println!();
    println!("Timing detail (median / min / max):");
    println!();
    print!("{:<10}", "");
    for (name, _) in SYSTEM_PROMPTS {
        print!("{:<22}", name);
    }
    println!();

    for (mi, model) in MODELS.iter().enumerate() {
        print!("{:<10}", model);
        for pi in 0..SYSTEM_PROMPTS.len() {
            let n = mi * SYSTEM_PROMPTS.len() + pi + 1;
            if let Some(agg) = aggregates.iter().find(|a| a.trial_n == n) {
                let cell = format!(
                    "{:.2} / {:.2} / {:.2}",
                    agg.median_secs, agg.min_secs, agg.max_secs
                );
                print!("{:<22}", cell);
            } else {
                print!("{:<22}", "---");
            }
        }
        println!();
    }

    // Token usage table
    println!();
    println!("Token usage (median: input + cache_read / output):");
    println!();
    print!("{:<10}", "");
    for (name, _) in SYSTEM_PROMPTS {
        print!("{:<22}", name);
    }
    println!();

    for (mi, model) in MODELS.iter().enumerate() {
        print!("{:<10}", model);
        for pi in 0..SYSTEM_PROMPTS.len() {
            let n = mi * SYSTEM_PROMPTS.len() + pi + 1;
            if let Some(agg) = aggregates.iter().find(|a| a.trial_n == n) {
                let cell = format!(
                    "{}+{} / {}",
                    agg.median_input_tokens, agg.median_cache_read_tokens,
                    agg.median_output_tokens
                );
                print!("{:<22}", cell);
            } else {
                print!("{:<22}", "---");
            }
        }
        println!();
    }

    // Cost table
    println!();
    println!("Cost per trial (total across {} repeats):", repeats);
    println!();
    print!("{:<10}", "");
    for (name, _) in SYSTEM_PROMPTS {
        print!("{:<14}", name);
    }
    println!();

    for (mi, model) in MODELS.iter().enumerate() {
        print!("{:<10}", model);
        for pi in 0..SYSTEM_PROMPTS.len() {
            let n = mi * SYSTEM_PROMPTS.len() + pi + 1;
            if let Some(agg) = aggregates.iter().find(|a| a.trial_n == n) {
                let cell = format!("${:.4}", agg.total_cost_usd);
                print!("{:<14}", cell);
            } else {
                print!("{:<14}", "---");
            }
        }
        println!();
    }

    // API latency table
    println!();
    println!("API latency (median ms, excludes CLI overhead):");
    println!();
    print!("{:<10}", "");
    for (name, _) in SYSTEM_PROMPTS {
        print!("{:<14}", name);
    }
    println!();

    for (mi, model) in MODELS.iter().enumerate() {
        print!("{:<10}", model);
        for pi in 0..SYSTEM_PROMPTS.len() {
            let n = mi * SYSTEM_PROMPTS.len() + pi + 1;
            if let Some(agg) = aggregates.iter().find(|a| a.trial_n == n) {
                let cell = format!("{}ms", agg.median_api_ms);
                print!("{:<14}", cell);
            } else {
                print!("{:<14}", "---");
            }
        }
        println!();
    }

    // Totals
    let total_invocations: usize = aggregates.iter().map(|a| a.repeat_count).sum();
    let total_passes: usize = aggregates.iter().map(|a| a.pass_count).sum();
    let total_fails = total_invocations - total_passes;

    // All individual times for grand median
    let all_times: Vec<f64> = aggregates.iter().flat_map(|a| a.times.clone()).collect();
    let grand_median = median(&all_times);

    let total_cost: f64 = aggregates.iter().map(|a| a.total_cost_usd).sum();

    println!();
    println!(
        "Totals: {} pass, {} fail across {} invocations in {:.1}s",
        total_passes, total_fails, total_invocations, total_elapsed
    );
    println!("Grand median per invocation: {:.2}s", grand_median);
    println!("Total cost: ${:.4}", total_cost);
    println!();
}
