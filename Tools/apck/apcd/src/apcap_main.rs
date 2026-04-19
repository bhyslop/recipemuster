// Copyright 2026 Scale Invariant, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![deny(warnings)]
#![allow(non_camel_case_types)]
#![allow(private_interfaces)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// RCG output discipline: all emission via apcrl_*! — no direct println!/eprintln!

use std::sync::{Mutex, OnceLock};
use apcd::apcrm_match::{apcrm_Finding, apcrm_Severity, apcrm_PhiCategory};

// ---------------------------------------------------------------------------
// Statics — dictionary cache, clipboard tracking, toggle state
// ---------------------------------------------------------------------------

static ZAPCAP_DICTS: OnceLock<apcd::apcrd_dictionaries::apcrd_Dictionaries> = OnceLock::new();
static ZAPCAP_LAST_CLIPBOARD: Mutex<Option<String>> = Mutex::new(None);
static ZAPCAP_LAST_CLINICAL: Mutex<Option<zapcap_ClinicalCache>> = Mutex::new(None);
static ZAPCAP_TOGGLE_STATES: Mutex<Vec<String>> = Mutex::new(Vec::new());

struct zapcap_ClinicalCache {
    findings:   Vec<apcrm_Finding>,
    plain_text: String,
}

// ---------------------------------------------------------------------------
// Focus handler — read clipboard, run detection, push rendered HTML
// ---------------------------------------------------------------------------

fn zapcap_handle_focus(window: &tauri::WebviewWindow) -> Result<(), String> {
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| format!("clipboard error: {}", e))?;

    let content = clipboard.get_text()
        .map_err(|e| format!("clipboard error: {}", e))?;

    // Change detection — same content as last invocation → preserve UI state
    {
        let last = ZAPCAP_LAST_CLIPBOARD.lock()
            .map_err(|e| format!("lock error: {}", e))?;
        if last.as_deref() == Some(content.as_str()) {
            return Ok(());
        }
    }

    let dicts = ZAPCAP_DICTS.get_or_init(apcd::apcrd_dictionaries::apcrd_Dictionaries::apcrd_load);

    match apcd::apcre_engine::apcre_analyze(&content, dicts) {
        apcd::apcre_engine::apcre_Result::Clinical { findings, plain_text } => {
            // Harvest every arboard-accessible flavor before the clipboard
            // zero-out. Success and failure both log — the journal log is
            // a confirmation mechanism, not just an error record.
            match apcd::apcrj_journal::apcrj_journal_path() {
                Some(journal_dir) => {
                    match apcd::apcrh_harvest::apcrh_capture_all_flavors(&journal_dir) {
                        Ok(n)  => apcd::apcrl_info_now!("harvest captured as {}", n),
                        Err(e) => apcd::apcrl_error_now!("harvest capture failed: {}", e),
                    }
                }
                None => {
                    apcd::apcrl_error_now!("harvest skipped: HOME not set");
                }
            }

            // Clear system clipboard after consuming clinical content
            let _ = clipboard.set_text(String::new());

            {
                let mut last = ZAPCAP_LAST_CLIPBOARD.lock()
                    .map_err(|e| format!("lock error: {}", e))?;
                *last = Some(String::new());
            }

            {
                let mut toggle_states = ZAPCAP_TOGGLE_STATES.lock()
                    .map_err(|e| format!("lock error: {}", e))?;
                *toggle_states = vec!["elide".to_string(); findings.len()];
            }

            {
                let mut cache = ZAPCAP_LAST_CLINICAL.lock()
                    .map_err(|e| format!("lock error: {}", e))?;
                *cache = Some(zapcap_ClinicalCache { findings, plain_text });
            }

            zapcap_push_triage(window)
        }
        apcd::apcre_engine::apcre_Result::NonClinical { content_length, content_type, preview } => {
            {
                let mut last = ZAPCAP_LAST_CLIPBOARD.lock()
                    .map_err(|e| format!("lock error: {}", e))?;
                *last = Some(content);
            }

            {
                let mut cache = ZAPCAP_LAST_CLINICAL.lock()
                    .map_err(|e| format!("lock error: {}", e))?;
                *cache = None;
            }

            {
                let mut toggle_states = ZAPCAP_TOGGLE_STATES.lock()
                    .map_err(|e| format!("lock error: {}", e))?;
                *toggle_states = Vec::new();
            }

            let diagnostic = format!(
                "Not clinical ({} bytes, {}): {}",
                content_length, content_type, preview
            );
            zapcap_push_instruction(window, &diagnostic)
        }
    }
}

// ---------------------------------------------------------------------------
// Tauri commands — toggle finding, copy anonymized
// ---------------------------------------------------------------------------

#[tauri::command]
fn toggle_finding(index: usize, window: tauri::WebviewWindow) -> Result<(), String> {
    {
        let mut states = ZAPCAP_TOGGLE_STATES.lock()
            .map_err(|e| format!("lock error: {}", e))?;
        if index < states.len() {
            states[index] = if states[index] == "elide" {
                "pass".to_string()
            } else {
                "elide".to_string()
            };
        }
    }
    zapcap_push_triage(&window)
}

#[tauri::command]
fn copy_anonymized(window: tauri::WebviewWindow) -> Result<(), String> {
    let toggle_states = ZAPCAP_TOGGLE_STATES.lock()
        .map_err(|e| format!("lock error: {}", e))?
        .clone();

    let anonymized = {
        let cache = ZAPCAP_LAST_CLINICAL.lock()
            .map_err(|e| format!("lock error: {}", e))?;
        let clinical = cache.as_ref()
            .ok_or_else(|| "no clinical data to anonymize".to_string())?;
        apcd::apcre_engine::apcre_anonymize(
            &clinical.plain_text,
            &clinical.findings,
            &toggle_states,
        )
    };

    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| format!("clipboard error: {}", e))?;
    clipboard.set_text(anonymized)
        .map_err(|e| format!("clipboard error: {}", e))?;

    zapcap_push_triage_with_feedback(&window, "Copied!")?;

    let win = window.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(2));
        let _ = zapcap_push_triage(&win);
    });

    Ok(())
}

// ---------------------------------------------------------------------------
// HTML push — render state and inject into webview
// ---------------------------------------------------------------------------

fn zapcap_push_instruction(window: &tauri::WebviewWindow, diagnostic: &str) -> Result<(), String> {
    let html = zapcap_render_instruction(diagnostic);
    zapcap_eval_html(window, &html)
}

fn zapcap_push_triage(window: &tauri::WebviewWindow) -> Result<(), String> {
    zapcap_push_triage_with_feedback(window, "Copy Anonymized to Clipboard")
}

fn zapcap_push_triage_with_feedback(
    window: &tauri::WebviewWindow,
    btn_text: &str,
) -> Result<(), String> {
    // Clone data out of locks to avoid holding multiple locks simultaneously
    let (findings, plain_text) = {
        let cache = ZAPCAP_LAST_CLINICAL.lock()
            .map_err(|e| format!("lock error: {}", e))?;
        let clinical = cache.as_ref()
            .ok_or_else(|| "no clinical data for triage".to_string())?;
        (clinical.findings.clone(), clinical.plain_text.clone())
    };
    let toggle_states = ZAPCAP_TOGGLE_STATES.lock()
        .map_err(|e| format!("lock error: {}", e))?
        .clone();

    let html = zapcap_render_triage(&findings, &plain_text, &toggle_states, btn_text);
    zapcap_eval_html(window, &html)
}

fn zapcap_eval_html(window: &tauri::WebviewWindow, html: &str) -> Result<(), String> {
    let escaped = zapcap_js_string_literal(html);
    window.eval(&format!("document.getElementById('app').innerHTML={}", escaped))
        .map_err(|e| format!("eval error: {}", e))
}

// ---------------------------------------------------------------------------
// HTML rendering — instruction state
// ---------------------------------------------------------------------------

fn zapcap_render_instruction(diagnostic: &str) -> String {
    format!(
        "<h1>Ann&rsquo;s PHI Clipbuddy</h1>\
         <p class=\"instruction\">Copy a clinical note from Epic using \
         &ldquo;Copy All&rdquo;, then switch to this window.</p>\
         <p class=\"diagnostic\">{}</p>",
        zapcap_escape_html(diagnostic)
    )
}

// ---------------------------------------------------------------------------
// HTML rendering — triage state
// ---------------------------------------------------------------------------

fn zapcap_render_triage(
    findings: &[apcrm_Finding],
    plain_text: &str,
    toggle_states: &[String],
    btn_text: &str,
) -> String {
    let preview = zapcap_render_preview(findings, plain_text, toggle_states);
    let (yellow_html, red_html) = zapcap_render_findings(findings, toggle_states);

    format!(
        "<h1>Ann&rsquo;s PHI Clipbuddy</h1>\
         <div id=\"document-preview\">{}</div>\
         <div id=\"findings-panel\">\
         <div class=\"findings-section\" id=\"findings-yellow\">\
         <h3>Questionable Findings</h3>\
         <div class=\"findings-list\" id=\"yellow-list\">{}</div></div>\
         <div class=\"findings-section\" id=\"findings-red\">\
         <h3>Definite PHI</h3>\
         <div class=\"findings-list\" id=\"red-list\">{}</div></div></div>\
         <button id=\"copy-btn\" \
         onclick=\"window.__TAURI__.core.invoke('copy_anonymized')\">{}</button>",
        preview, yellow_html, red_html, zapcap_escape_html(btn_text)
    )
}

// ---------------------------------------------------------------------------
// HTML rendering — document preview with inline PHI highlights
// ---------------------------------------------------------------------------

fn zapcap_render_preview(
    findings: &[apcrm_Finding],
    plain_text: &str,
    toggle_states: &[String],
) -> String {
    // Build index-annotated list sorted by offset (byte offsets used directly)
    let mut sorted: Vec<(usize, &apcrm_Finding)> = findings.iter().enumerate().collect();
    sorted.sort_by(|a, b| a.1.offset.cmp(&b.1.offset).then(b.1.length.cmp(&a.1.length)));

    let mut parts = Vec::new();
    let mut pos: usize = 0;

    for (idx, f) in &sorted {
        if f.offset < pos { continue; } // skip overlap

        // Text before this finding
        if f.offset > pos {
            parts.push(zapcap_escape_html(&plain_text[pos..f.offset]));
        }

        // Finding span — byte offsets consumed directly, no conversion needed
        let end = f.offset + f.length;
        let cls = if toggle_states.get(*idx).map(|s| s.as_str()) == Some("pass") {
            "phi-pass"
        } else if f.severity == apcrm_Severity::Red {
            "phi-red"
        } else {
            "phi-yellow"
        };

        parts.push(format!(
            "<span class=\"{}\" data-index=\"{}\">{}</span>",
            cls, idx, zapcap_escape_html(&plain_text[f.offset..end])
        ));
        pos = end;
    }

    // Remaining text
    if pos < plain_text.len() {
        parts.push(zapcap_escape_html(&plain_text[pos..]));
    }

    parts.join("")
}

// ---------------------------------------------------------------------------
// HTML rendering — findings panel (yellow + red sections)
// ---------------------------------------------------------------------------

fn zapcap_render_findings(
    findings: &[apcrm_Finding],
    toggle_states: &[String],
) -> (String, String) {
    let mut yellow = Vec::new();
    let mut red = Vec::new();

    for (i, f) in findings.iter().enumerate() {
        let state = toggle_states.get(i).map(|s| s.as_str()).unwrap_or("elide");
        let label = zapcap_format_category(f.category);
        let btn_text = if state == "elide" { "ELIDE \u{25bc}" } else { "PASS \u{25bc}" };

        let entry = format!(
            "<div class=\"finding-entry\">\
             <span class=\"finding-text\">{}</span>\
             <span class=\"finding-category\">{}</span>\
             <button class=\"finding-toggle {}\" \
             onclick=\"window.__TAURI__.core.invoke('toggle_finding',{{index:{}}})\"\
             >{}</button></div>",
            zapcap_escape_html(&f.text),
            label,
            state,
            i,
            btn_text
        );

        if f.severity == apcrm_Severity::Yellow {
            yellow.push(entry);
        } else {
            red.push(entry);
        }
    }

    let yellow_html = if yellow.is_empty() {
        "<div class=\"empty-section\">None detected</div>".to_string()
    } else {
        yellow.join("")
    };

    let red_html = if red.is_empty() {
        "<div class=\"empty-section\">None detected</div>".to_string()
    } else {
        red.join("")
    };

    (yellow_html, red_html)
}

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

fn zapcap_format_category(cat: apcrm_PhiCategory) -> &'static str {
    match cat {
        apcrm_PhiCategory::Name         => "NAME",
        apcrm_PhiCategory::Provider     => "PROVIDER",
        apcrm_PhiCategory::Facility     => "FACILITY",
        apcrm_PhiCategory::Date         => "DATE",
        apcrm_PhiCategory::Dob          => "DOB",
        apcrm_PhiCategory::Phone        => "PHONE",
        apcrm_PhiCategory::Email        => "EMAIL",
        apcrm_PhiCategory::Ssn          => "SSN",
        apcrm_PhiCategory::Mrn          => "MRN",
        apcrm_PhiCategory::Account      => "ACCOUNT",
        apcrm_PhiCategory::EncounterId  => "ENCOUNTER ID",
        apcrm_PhiCategory::Address      => "ADDRESS",
        apcrm_PhiCategory::HealthPlanId => "HEALTH PLAN ID",
        apcrm_PhiCategory::DeviceId     => "DEVICE ID",
        apcrm_PhiCategory::Npi          => "NPI",
        apcrm_PhiCategory::Dea          => "DEA",
        apcrm_PhiCategory::Url          => "URL",
        apcrm_PhiCategory::IpAddress    => "IP ADDRESS",
    }
}

fn zapcap_escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn zapcap_js_string_literal(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '\\'     => out.push_str("\\\\"),
            '"'      => out.push_str("\\\""),
            '\n'     => out.push_str("\\n"),
            '\r'     => out.push_str("\\r"),
            '\t'     => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn zapcap_init_log_tee() {
    let log_path = match apcd::apcrj_journal::apcrj_log_path() {
        Some(p) => p,
        None    => {
            apcd::apcrl_error_now!("log tee skipped: HOME not set");
            return;
        }
    };
    if let Some(parent) = log_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            apcd::apcrl_error_now!(
                "create journal dir {}: {}", parent.display(), e
            );
            return;
        }
    }
    if let Err(e) = apcd::apcrl_log::apcrl_tee_init(&log_path) {
        apcd::apcrl_error_now!("log tee init failed: {}", e);
    }
}

fn main() {
    zapcap_init_log_tee();
    apcd::apcrl_info_now!("starting Ann's PHI Clipbuddy");
    // Self-update watcher disabled for prototype — manual quit + relaunch
    // via /Users/Shared/apcua/Apcap.app is the current deploy workflow.
    // apcd::apcru_update::apcru_start_watcher();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![toggle_finding, copy_anonymized])
        .setup(|app| {
            use tauri::Manager;
            let window = app.get_webview_window("main")
                .expect("main window must exist");
            let win = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Focused(true) = event {
                    if let Err(e) = zapcap_handle_focus(&win) {
                        apcd::apcrl_error_now!("focus handler error: {}", e);
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("tauri application error: {}", e));
}
