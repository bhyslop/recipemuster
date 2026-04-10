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

// ---------------------------------------------------------------------------
// Statics — dictionary cache and clipboard change tracking
// ---------------------------------------------------------------------------

static ZAPCAP_DICTS: OnceLock<apcd::apcrd_dictionaries::apcrd_Dictionaries> = OnceLock::new();
static ZAPCAP_LAST_CLIPBOARD: Mutex<Option<String>> = Mutex::new(None);

// ---------------------------------------------------------------------------
// IPC result type — tagged enum for JS consumption
// ---------------------------------------------------------------------------

#[derive(serde::Serialize)]
#[serde(tag = "type")]
enum zapcap_ConsumeResult {
    #[serde(rename = "unchanged")]
    Unchanged,
    #[serde(rename = "clinical")]
    Clinical {
        findings:   Vec<apcd::apcrm_match::apcrm_Finding>,
        plain_text: String,
    },
    #[serde(rename = "non_clinical")]
    NonClinical {
        content_length: usize,
        content_type:   String,
        preview:        String,
    },
}

// ---------------------------------------------------------------------------
// Tauri command — consume clipboard, run detection, return classified stream
// ---------------------------------------------------------------------------

#[tauri::command]
fn consume_clipboard() -> Result<zapcap_ConsumeResult, String> {
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|e| format!("clipboard error: {}", e))?;

    // Read clipboard text — arboard 3.x lacks get_html(), so the fixture loader
    // sets alt_text=html to make get_text() return the HTML content.
    // Real Epic integration will need platform-specific HTML clipboard reading.
    let content = clipboard.get_text()
        .map_err(|e| format!("clipboard error: {}", e))?;

    // Change detection — same content as last invocation → preserve UI state
    {
        let last = ZAPCAP_LAST_CLIPBOARD.lock()
            .map_err(|e| format!("lock error: {}", e))?;
        if last.as_deref() == Some(content.as_str()) {
            return Ok(zapcap_ConsumeResult::Unchanged);
        }
    }

    let dicts = ZAPCAP_DICTS.get_or_init(apcd::apcrd_dictionaries::apcrd_Dictionaries::apcrd_load);

    match apcd::apcre_engine::apcre_analyze(&content, dicts) {
        apcd::apcre_engine::apcre_Result::Clinical { findings, plain_text } => {
            // Clear system clipboard after consuming clinical content
            let _ = clipboard.set_text(String::new());
            // Track the cleared state so next refocus sees "unchanged"
            let mut last = ZAPCAP_LAST_CLIPBOARD.lock()
                .map_err(|e| format!("lock error: {}", e))?;
            *last = Some(String::new());
            Ok(zapcap_ConsumeResult::Clinical { findings, plain_text })
        }
        apcd::apcre_engine::apcre_Result::NonClinical { content_length, content_type, preview } => {
            // Track so repeated refocus with same non-clinical content → unchanged
            let mut last = ZAPCAP_LAST_CLIPBOARD.lock()
                .map_err(|e| format!("lock error: {}", e))?;
            *last = Some(content);
            Ok(zapcap_ConsumeResult::NonClinical { content_length, content_type, preview })
        }
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    apcd::apcrl_info_now!("starting Ann's PHI Clipbuddy");
    apcd::apcru_update::apcru_start_watcher();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![consume_clipboard])
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| apcd::apcrl_fatal_now!("tauri application error: {}", e));
}
