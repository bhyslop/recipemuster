// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: Apache-2.0
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
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTDRU — cupel: command-dependency static analysis over bash.
//
// the allowlists this module classifies against live in rbtchc_cupel.
//
// Algorithm — two-pass, function-aware, with asymmetric scope. Pass 1 collects
// every locally-defined function name across the WHOLE corpus (all kits, minus
// dead ABANDONED code) so that cross-kit and sourced names resolve; pass 2 lints
// only the release kit roots, flagging each command-position token not in {bash
// builtins, local functions, POSIX floor, declared deps} and failing-with-
// replacement on the eviction table. Soundness rests on the corpus already being
// shellcheck-clean (enforced at release qualification and marshal-zero), so a
// command-position lexer suffices — no full shell parser.
//
// Two execution-environment domains, partitioned by path:
//   - Kit-bash   — strict. Eviction table enforced; unknown commands fail.
//   - GCB-bash   — Google Cloud Build job scripts under any Tools/rbk/rbgj*
//                  directory. Looser: they run in the cloud-sdk image where the
//                  evicted commands and gcloud are present, so evictions are not
//                  enforced and the GCB-extra allowlist is added.
//
// Known lexer limits (accepted per the fixture's design — the corpus is
// shellcheck-clean and the discovery run surfaces residue for triage):
//   - Command substitutions nested inside double-quoted strings are not scanned.
//   - The `;&` / `;;&` case fall-through operators are treated as a plain `;`.

use std::collections::BTreeSet;
use std::path::{
    Path,
    PathBuf,
};

use crate::rbtchc_cupel::{
    zrbtchc_is_gcb,
    zrbtchc_walk_ext,
    zrbtchc_Domain,
    zrbtchc_Finding,
    zrbtchc_ScanResult,
    ZRBTCHC_BUILTINS,
    ZRBTCHC_DECLARED_DEPS,
    ZRBTCHC_EVICTIONS,
    ZRBTCHC_GCB_ALLOWED,
    ZRBTCHC_KIT_ROOTS,
    ZRBTCHC_LINT_EXCLUDED_DIR_PREFIXES,
    ZRBTCHC_POSIX_FLOOR,
    ZRBTCHC_SH_EXT,
    ZRBTCHC_UNIVERSE_EXCLUDED_DIR_PREFIXES,
};

// ── Lexer ───────────────────────────────────────────────────

/// Read one shell word starting at `*i`, advancing `*i` past it and `*line`
/// over any embedded newlines. Quoted segments and `${...}` parameter
/// expansions are consumed as part of the word; an embedded `$(` command
/// substitution STOPS the word so the caller's lexer scans the substituted
/// command at its own command position.
pub(crate) fn zrbtchb_read_word(chars: &[char], i: &mut usize, line: &mut usize) -> String {
    let n = chars.len();
    let mut word = String::new();
    while *i < n {
        let c = chars[*i];
        match c {
            ' ' | '\t' | '\r' | '\n' => break,
            ';' | '|' | '&' | '<' | '>' | '(' | ')' | '`' | '#' => break,
            '\'' => {
                word.push(c);
                *i += 1;
                while *i < n && chars[*i] != '\'' {
                    if chars[*i] == '\n' {
                        *line += 1;
                    }
                    word.push(chars[*i]);
                    *i += 1;
                }
                if *i < n {
                    word.push(chars[*i]);
                    *i += 1;
                }
            }
            '"' => {
                word.push(c);
                *i += 1;
                while *i < n && chars[*i] != '"' {
                    if chars[*i] == '\\' && *i + 1 < n {
                        word.push(chars[*i]);
                        word.push(chars[*i + 1]);
                        *i += 2;
                        continue;
                    }
                    if chars[*i] == '\n' {
                        *line += 1;
                    }
                    word.push(chars[*i]);
                    *i += 1;
                }
                if *i < n {
                    word.push(chars[*i]);
                    *i += 1;
                }
            }
            '$' if *i + 1 < n && chars[*i + 1] == '(' => break,
            '$' if *i + 1 < n && chars[*i + 1] == '{' => {
                word.push('$');
                *i += 1;
                let mut depth = 0usize;
                while *i < n {
                    let d = chars[*i];
                    if d == '\n' {
                        *line += 1;
                    }
                    word.push(d);
                    *i += 1;
                    if d == '{' {
                        depth += 1;
                    } else if d == '}' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                }
            }
            _ => {
                word.push(c);
                *i += 1;
            }
        }
    }
    word
}

/// Classify a word as transparent or value-introducing when it sits in command
/// position. `Some(true)` — a keyword whose successor is itself a command
/// (`if`, `then`, `while`, …). `Some(false)` — a keyword whose successor is a
/// value, not a command (`for`, `case`, `in`, …). `None` — not a keyword.
pub(crate) fn zrbtchb_keyword_kind(word: &str) -> Option<bool> {
    match word {
        "if" | "elif" | "while" | "until" | "then" | "else" | "do" | "!"
        | "time" | "fi" | "done" | "esac" => Some(true),
        "for" | "select" | "case" | "in" | "function" => Some(false),
        _ => None,
    }
}

/// True when `word` is a `NAME=`, `NAME+=`, or `NAME[idx]=` assignment prefix —
/// a command may still follow on the same line (`FOO=bar cmd`), so command
/// position is preserved across it.
pub(crate) fn zrbtchb_is_assignment(word: &str) -> bool {
    let bytes = word.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    let first = bytes[0] as char;
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    let mut k = 1;
    while k < bytes.len() {
        let ch = bytes[k] as char;
        if ch == '=' {
            return true;
        }
        if ch == '+' && k + 1 < bytes.len() && bytes[k + 1] == b'=' {
            return true;
        }
        if ch == '[' {
            return word.contains("]=");
        }
        if !(ch.is_ascii_alphanumeric() || ch == '_') {
            return false;
        }
        k += 1;
    }
    false
}

/// Advance `*i` past a balanced run of parentheses, tracking newlines in
/// `*line`. `depth` is the count of opening parens already consumed by the
/// caller; scanning ends when it returns to zero. Quoted segments are skipped
/// so parens inside strings do not affect the balance — covering array literals
/// `NAME=( … )` and nested arithmetic `$(( ( … ) ))`.
pub(crate) fn zrbtchb_skip_balanced_parens(chars: &[char], i: &mut usize, line: &mut usize, mut depth: usize) {
    let n = chars.len();
    while *i < n && depth > 0 {
        match chars[*i] {
            '(' => depth += 1,
            ')' => depth -= 1,
            '\n' => *line += 1,
            '\'' => {
                *i += 1;
                while *i < n && chars[*i] != '\'' {
                    if chars[*i] == '\n' {
                        *line += 1;
                    }
                    *i += 1;
                }
            }
            '"' => {
                *i += 1;
                while *i < n && chars[*i] != '"' {
                    if chars[*i] == '\\' && *i + 1 < n {
                        *i += 2;
                        continue;
                    }
                    if chars[*i] == '\n' {
                        *line += 1;
                    }
                    *i += 1;
                }
            }
            _ => {}
        }
        *i += 1;
    }
}

/// Extract every command-position token from a bash source string, paired with
/// its 1-based line number. A token is in command position at the start of the
/// script and after any command separator (`;`, `|`, `&`, `&&`, `||`, newline,
/// `(`, `$(`, `` ` ``, an open brace group) and after a transparent keyword.
/// Assignments, redirections, `[[ ]]` test contents, comments, here-doc bodies,
/// and arithmetic `(( ))` / `$(( ))` are excluded.
pub(crate) fn zrbtchb_command_words(src: &str) -> Vec<(usize, String)> {
    let chars: Vec<char> = src.chars().collect();
    let n = chars.len();
    let mut out: Vec<(usize, String)> = Vec::new();
    let mut i = 0usize;
    let mut line = 1usize;
    let mut cmd_pos = true;
    let mut paren_depth = 0usize;
    let mut in_dbracket = false;
    // True from a declaration word (`local`, `readonly`, …) until command
    // position reopens; its flags and assignment words follow.
    let mut after_decl = false;
    let mut pending_heredoc: Option<String> = None;
    // case…esac nesting. Each frame tracks the position within a `case`:
    // 0 = subject (between `case` and `in`), 1 = pattern (suppress recording;
    // `|` is alternation, not a pipe), 2 = branch body (record commands).
    let mut case_stack: Vec<u8> = Vec::new();

    while i < n {
        let c = chars[i];

        if c == '\n' {
            i += 1;
            line += 1;
            cmd_pos = true;
            if let Some(delim) = pending_heredoc.take() {
                loop {
                    let start = i;
                    while i < n && chars[i] != '\n' {
                        i += 1;
                    }
                    let body: String = chars[start..i].iter().collect();
                    let had_nl = i < n;
                    if had_nl {
                        i += 1;
                        line += 1;
                    }
                    if body.trim() == delim {
                        break;
                    }
                    if !had_nl {
                        break;
                    }
                }
            }
            continue;
        }
        if c == ' ' || c == '\t' || c == '\r' {
            i += 1;
            continue;
        }
        if c == '\\' && i + 1 < n && chars[i + 1] == '\n' {
            i += 2;
            line += 1;
            continue;
        }
        if c == '#' {
            while i < n && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c == ';' {
            i += 1;
            if i < n && chars[i] == ';' {
                i += 1;
                // `;;` ends a case branch — the next token opens a new pattern.
                if let Some(top) = case_stack.last_mut() {
                    *top = 1;
                }
                cmd_pos = false;
                continue;
            }
            if !in_dbracket {
                cmd_pos = true;
            }
            continue;
        }
        if c == '|' {
            i += 1;
            if i < n && chars[i] == '|' {
                i += 1;
            }
            // Within a case pattern `|` is alternation, not a pipe.
            if case_stack.last() == Some(&1) {
                continue;
            }
            if !in_dbracket {
                cmd_pos = true;
            }
            continue;
        }
        if c == '&' {
            if i + 1 < n && chars[i + 1] == '>' {
                i += 2;
                if i < n && chars[i] == '>' {
                    i += 1;
                }
                cmd_pos = false;
                continue;
            }
            i += 1;
            if i < n && chars[i] == '&' {
                i += 1;
            }
            if !in_dbracket {
                cmd_pos = true;
            }
            continue;
        }
        if c == '<' {
            if i + 1 < n && chars[i + 1] == '<' {
                if i + 2 < n && chars[i + 2] == '<' {
                    i += 3;
                    cmd_pos = false;
                    continue;
                }
                i += 2;
                if i < n && chars[i] == '-' {
                    i += 1;
                }
                while i < n && (chars[i] == ' ' || chars[i] == '\t') {
                    i += 1;
                }
                let mut delim = String::new();
                let quote = if i < n && (chars[i] == '\'' || chars[i] == '"') {
                    let q = chars[i];
                    i += 1;
                    Some(q)
                } else {
                    None
                };
                while i < n {
                    let d = chars[i];
                    match quote {
                        Some(qc) => {
                            if d == qc {
                                i += 1;
                                break;
                            }
                            delim.push(d);
                            i += 1;
                        }
                        None => {
                            if d == ' ' || d == '\t' || d == '\n' || d == ';'
                                || d == '&' || d == '|' || d == '<' || d == '>'
                                || d == '(' || d == ')'
                            {
                                break;
                            }
                            if d == '\\' {
                                i += 1;
                                continue;
                            }
                            delim.push(d);
                            i += 1;
                        }
                    }
                }
                if !delim.is_empty() {
                    pending_heredoc = Some(delim);
                }
                cmd_pos = false;
                continue;
            }
            i += 1;
            if i < n && chars[i] == '&' {
                i += 1;
            }
            cmd_pos = false;
            continue;
        }
        if c == '>' {
            i += 1;
            if i < n && (chars[i] == '>' || chars[i] == '&') {
                i += 1;
            }
            cmd_pos = false;
            continue;
        }
        if c == '(' {
            if i + 1 < n && chars[i + 1] == '(' {
                // Arithmetic `(( … ))` — not a command list.
                i += 2;
                zrbtchb_skip_balanced_parens(&chars, &mut i, &mut line, 2);
                cmd_pos = false;
                continue;
            }
            i += 1;
            paren_depth += 1;
            cmd_pos = true;
            continue;
        }
        if c == ')' {
            i += 1;
            if paren_depth > 0 {
                paren_depth -= 1;
                cmd_pos = false;
            } else if case_stack.last() == Some(&1) {
                // Pattern terminator — the branch body's command list follows.
                if let Some(top) = case_stack.last_mut() {
                    *top = 2;
                }
                cmd_pos = true;
            } else {
                cmd_pos = true;
            }
            continue;
        }
        if c == '`' {
            i += 1;
            cmd_pos = true;
            continue;
        }
        if c == '$' {
            if i + 1 < n && chars[i + 1] == '(' {
                if i + 2 < n && chars[i + 2] == '(' {
                    // Arithmetic substitution `$(( … ))` — not a command.
                    i += 3;
                    zrbtchb_skip_balanced_parens(&chars, &mut i, &mut line, 2);
                    cmd_pos = false;
                    continue;
                }
                i += 2;
                paren_depth += 1;
                cmd_pos = true;
                continue;
            }
            let _ = zrbtchb_read_word(&chars, &mut i, &mut line);
            cmd_pos = false;
            continue;
        }
        if c == '{' {
            if i + 1 < n && (chars[i + 1] == ' ' || chars[i + 1] == '\t' || chars[i + 1] == '\n') {
                i += 1;
                cmd_pos = true;
                continue;
            }
            let _ = zrbtchb_read_word(&chars, &mut i, &mut line);
            cmd_pos = false;
            continue;
        }
        if c == '}' {
            i += 1;
            continue;
        }

        let word_line = line;
        let word = zrbtchb_read_word(&chars, &mut i, &mut line);
        if word.is_empty() {
            i += 1;
            continue;
        }
        if word == "[[" {
            if cmd_pos {
                in_dbracket = true;
            }
            cmd_pos = false;
            continue;
        }
        if word == "]]" {
            in_dbracket = false;
            cmd_pos = false;
            continue;
        }
        if in_dbracket {
            continue;
        }
        // case…esac structure — tracked regardless of command position so that
        // branch patterns (which sit at command position) are not mistaken for
        // commands.
        if word == "case" {
            case_stack.push(0);
            cmd_pos = false;
            continue;
        }
        if word == "esac" {
            case_stack.pop();
            cmd_pos = false;
            continue;
        }
        if word == "in" && case_stack.last() == Some(&0) {
            if let Some(top) = case_stack.last_mut() {
                *top = 1;
            }
            cmd_pos = false;
            continue;
        }
        if case_stack.last() == Some(&1) {
            // Matching a case pattern — suppress; the pattern is not a command.
            cmd_pos = false;
            continue;
        }
        if cmd_pos {
            after_decl = false;
        }
        if !cmd_pos && !(after_decl && zrbtchb_is_assignment(&word)) {
            continue;
        }
        if zrbtchb_is_assignment(&word) {
            // `NAME=( … )` array literal — the elements are data, not commands.
            let mut j = i;
            while j < n && (chars[j] == ' ' || chars[j] == '\t') {
                j += 1;
            }
            if j < n && chars[j] == '(' {
                i = j + 1;
                zrbtchb_skip_balanced_parens(&chars, &mut i, &mut line, 1);
            }
            continue;
        }
        match zrbtchb_keyword_kind(&word) {
            Some(true) => continue,
            Some(false) => {
                cmd_pos = false;
                continue;
            }
            None => {}
        }
        after_decl = matches!(word.as_str(), "local" | "readonly" | "declare" | "typeset");
        out.push((word_line, word));
        cmd_pos = false;
    }
    out
}

// ── Function collection and classification ──────────────────

/// Harvest locally-defined function names from one source into `out`. Matches
/// both `name() {` and `function name` forms.
pub(crate) fn zrbtchb_collect_functions(src: &str, out: &mut BTreeSet<String>) {
    for raw in src.lines() {
        let trimmed = raw.trim_start();
        let rest = if let Some(after) = trimmed.strip_prefix("function ") {
            after.trim_start()
        } else {
            trimmed
        };
        let mut name = String::new();
        for ch in rest.chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                name.push(ch);
            } else {
                break;
            }
        }
        if name.is_empty() {
            continue;
        }
        let after_name = rest[name.len()..].trim_start();
        if after_name.starts_with("()") || trimmed.starts_with("function ") {
            out.insert(name);
        }
    }
}

/// Classify a command-position token. Returns `Some(detail)` when it violates
/// the domain's discipline, `None` when it is permitted. Dynamic tokens
/// (containing an expansion or quote) cannot be statically named and are
/// skipped.
pub(crate) fn zrbtchb_classify(
    command: &str,
    locals: &BTreeSet<String>,
    domain: zrbtchc_Domain,
) -> Option<String> {
    if command.is_empty() {
        return None;
    }
    if command.contains('$')
        || command.contains('`')
        || command.contains('"')
        || command.contains('\'')
    {
        return None;
    }
    let base = command.rsplit('/').next().unwrap_or(command);
    if ZRBTCHC_BUILTINS.contains(&base) {
        return None;
    }
    if locals.contains(command) || locals.contains(base) {
        return None;
    }
    // POSIX floor is universal — cleared in both domains.
    if ZRBTCHC_POSIX_FLOOR.contains(&base) {
        return None;
    }
    match domain {
        zrbtchc_Domain::Kit => {
            if ZRBTCHC_DECLARED_DEPS.contains(&base) {
                return None;
            }
            for ev in ZRBTCHC_EVICTIONS {
                if ev.command == base {
                    return Some(format!("evicted command — use {}", ev.replacement));
                }
            }
            Some("unknown command — not in POSIX floor or declared dependencies".to_string())
        }
        zrbtchc_Domain::Gcb => {
            // No declared-dep inheritance and no eviction free-pass: GCB-bash is
            // held to floor ∪ the curated container-tool list (supply-chain).
            if ZRBTCHC_GCB_ALLOWED.contains(&base) {
                return None;
            }
            Some("unknown command — not in the curated GCB container-tool allowlist".to_string())
        }
    }
}

/// True when `command` names an external command in the corpus's surface — a
/// static token that is neither a bash builtin nor a locally-defined function.
/// This is the classification-independent inventory predicate: it admits the
/// union of allowed externals (floor / declared / gcb-extra / tolerated
/// evictions) AND violations. Dynamic tokens (expansion or quote) cannot be
/// statically named and are excluded.
fn zrbtchb_is_external(command: &str, locals: &BTreeSet<String>) -> bool {
    if command.is_empty()
        || command.contains('$')
        || command.contains('`')
        || command.contains('"')
        || command.contains('\'')
    {
        return false;
    }
    let base = command.rsplit('/').next().unwrap_or(command);
    !ZRBTCHC_BUILTINS.contains(&base)
        && !locals.contains(command)
        && !locals.contains(base)
}

// ── Corpus walk and scan ────────────────────────────────────

/// Walk the corpus, collect functions across all of it, then scan the files
/// belonging to `domain`, returning every finding (sorted by file and line)
/// plus the domain's external-command inventory.
pub(crate) fn zrbtchb_scan_domain(tools: &Path, domain: zrbtchc_Domain) -> Result<zrbtchc_ScanResult, String> {
    // Pass 1 — function-visibility universe. Walk every kit so cross-kit and
    // sourced function names resolve; only dead ABANDONED code stays invisible.
    let mut universe_files: Vec<PathBuf> = Vec::new();
    zrbtchc_walk_ext(tools, ZRBTCHC_SH_EXT, ZRBTCHC_UNIVERSE_EXCLUDED_DIR_PREFIXES, &mut universe_files);
    universe_files.sort();

    let mut locals: BTreeSet<String> = BTreeSet::new();
    for f in &universe_files {
        let src = std::fs::read_to_string(f)
            .map_err(|e| format!("read {} failed: {}", f.display(), e))?;
        zrbtchb_collect_functions(&src, &mut locals);
    }

    // Pass 2 — lint target. Only the release kit roots, minus dead/not-yet-live.
    let mut lint_files: Vec<PathBuf> = Vec::new();
    for kit in ZRBTCHC_KIT_ROOTS {
        zrbtchc_walk_ext(&tools.join(kit), ZRBTCHC_SH_EXT, ZRBTCHC_LINT_EXCLUDED_DIR_PREFIXES, &mut lint_files);
    }
    lint_files.sort();

    let root = tools.parent().unwrap_or(tools);
    let mut findings: Vec<zrbtchc_Finding> = Vec::new();
    let mut inventory: BTreeSet<String> = BTreeSet::new();
    for path in &lint_files {
        let is_gcb = zrbtchc_is_gcb(path);
        let in_domain = match domain {
            zrbtchc_Domain::Gcb => is_gcb,
            zrbtchc_Domain::Kit => !is_gcb,
        };
        if !in_domain {
            continue;
        }
        let classify_domain = if is_gcb {
            zrbtchc_Domain::Gcb
        } else {
            zrbtchc_Domain::Kit
        };
        let src = std::fs::read_to_string(path)
            .map_err(|e| format!("read {} failed: {}", path.display(), e))?;
        let rel = crate::rbtcep_platform::rbtcep_repo_rel(root, path);
        for (line, command) in zrbtchb_command_words(&src) {
            if zrbtchb_is_external(&command, &locals) {
                let base = command.rsplit('/').next().unwrap_or(&command);
                inventory.insert(base.to_string());
            }
            if let Some(detail) = zrbtchb_classify(&command, &locals, classify_domain) {
                findings.push(zrbtchc_Finding {
                    file: rel.clone(),
                    line,
                    command,
                    detail,
                });
            }
        }
    }
    Ok(zrbtchc_ScanResult { findings, inventory })
}
