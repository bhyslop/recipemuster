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
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBTDRU — cupel: BCG command-dependency static-analysis fixture.
//
// A cupel is the assay vessel in which base metals are driven off and the
// noble metal remains. This fixture drives off command-position tokens that
// violate the Bash Console Guide's Command Dependency Discipline, leaving a
// corpus whose external-command surface is exactly the declared dependency
// floor.
//
// BCG (section "Command Dependency Discipline") is the single source of truth:
//   - POSIX floor   — irreducible external commands, no builtin replacement
//   - Declared deps — the RBS0 Dependency Inventory (bash/git/curl/jq/docker/
//                     openssl/ssh/scp/ssh-keygen + developer/specialized tools)
//   - Eviction table — commands with builtin/declared replacements; enforced,
//                     failing with BCG's prescribed replacement
//
// Algorithm — two-pass, function-aware, with asymmetric scope. Pass 1 collects
// every locally-defined function name across the WHOLE corpus (all kits, minus
// dead ABANDONED code) so that cross-kit and sourced names resolve; pass 2 lints
// only the release kit roots, flagging each command-position token not in {bash
// builtins, local functions, POSIX floor, declared deps} and failing-with-
// replacement on the eviction table. Soundness rests on the corpus already being
// shellcheck-clean (rbq_qualify_fast), so a command-position lexer suffices — no
// full shell parser.
//
// Corpus scope — only the release-relevant kit roots (Tools/buk, Tools/rbk) are
// linted. These are the kits that ship in the recipe-bottle consumer release and
// are authored under BCG; other kits under Tools/ are separate products never
// written to the discipline, and holding them to it would surface noise, not
// defects. A kit adopts the discipline by being added to ZRBTDRU_KIT_ROOTS —
// opt-in, never by default. Within the lint target, ABANDONED* and FUTURE*
// directories are excluded (dead / not-yet-live); the pass-1 function universe
// excludes only ABANDONED* (FUTURE* code is present and sourceable, so its
// definitions stay visible).
//
// Two execution-environment domains, partitioned by path:
//   - Kit-bash   — strict BCG. Eviction table enforced; unknown commands fail.
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

use crate::case;
use crate::rbtdre_engine::{
    rbtdre_Case,
    rbtdre_Disposition,
    rbtdre_Fixture,
    rbtdre_Verdict,
};
use crate::rbtdrm_manifest::RBTDRM_FIXTURE_CUPEL;

// ── Corpus location ─────────────────────────────────────────

/// Repo-relative directory holding all kit trees, walked per release kit root.
pub(crate) const ZRBTDRU_TOOLS_SUBDIR: &str = "Tools";

/// Release-relevant kit roots under Tools/, each walked recursively. Only these
/// kits ship in the recipe-bottle consumer release and are authored under BCG,
/// so only these are held to the discipline. Names are directory basenames under
/// Tools/; adding one opts that kit into the lint deliberately.
pub(crate) const ZRBTDRU_KIT_ROOTS: &[&str] = &["buk", "rbk"];

/// Extension (no dot) selecting bash files from the corpus walk.
pub(crate) const ZRBTDRU_SH_EXT: &str = "sh";

/// Directory-name prefix marking the Google Cloud Build job family. Any bash
/// under a `Tools/rbk/rbgj*` directory is GCB-bash. Partitioning by this prefix
/// — rather than a hardcoded directory list — keeps the partition drift-proof
/// as new rbgj* job groups are added.
pub(crate) const ZRBTDRU_GCB_DIR_PREFIX: &str = "rbgj";

/// Dead-code directory prefixes excluded from the function-visibility universe.
/// `ABANDONED*` is retained for reference but unbuilt and unsourceable, so its
/// definitions must NOT clear a live command-position token — a live reference
/// to a dead function is a defect the lint should still surface.
pub(crate) const ZRBTDRU_UNIVERSE_EXCLUDED_DIR_PREFIXES: &[&str] = &["ABANDONED"];

/// Lint-target directory prefixes — additionally exclude `FUTURE*`. Not-yet-live
/// code is present on disk (so its functions stay visible for pass-1 collection,
/// resolving live code that sources it) but is not itself held to the discipline.
pub(crate) const ZRBTDRU_LINT_EXCLUDED_DIR_PREFIXES: &[&str] = &["ABANDONED", "FUTURE"];

/// Filename affixes for the per-domain findings trace written into the case
/// directory: `cupel-<label>-findings.txt`.
pub(crate) const ZRBTDRU_FINDINGS_PREFIX: &str = "cupel-";
pub(crate) const ZRBTDRU_FINDINGS_SUFFIX: &str = "-findings.txt";

/// Domain labels — name the findings trace and the verdict message per domain.
pub(crate) const ZRBTDRU_LABEL_KIT: &str = "kit";
pub(crate) const ZRBTDRU_LABEL_GCB: &str = "gcb";

// ── BCG allowlists (source of truth: BCG + RBS0 Dependency Inventory) ──

/// POSIX Utility Allowlist — the irreducible external-command floor. No bash
/// 3.2 builtin replacement; mandated by POSIX wherever bash runs.
pub(crate) const ZRBTDRU_POSIX_FLOOR: &[&str] = &[
    "chmod", "cp", "date", "find", "mkdir", "mktemp", "mv", "rm", "sed",
    "sleep", "sort", "stty",
];

/// Declared dependencies — the RBS0 Dependency Inventory (consumer + developer
/// + specialized). A cost accepted by every consumer; each appears in RBS0 with
/// its justification.
pub(crate) const ZRBTDRU_DECLARED_DEPS: &[&str] = &[
    "bash", "curl", "docker", "git", "jq", "openssl", "podman", "scp",
    "shellcheck", "ssh", "ssh-keygen", "tcpdump", "timeout",
];

/// GCB-extra allowlist — commands present in the Cloud Build cloud-sdk image
/// and used only by GCB-bash. Empty until the discovery run enumerates actual
/// GCB command usage; encoded thereafter.
pub(crate) const ZRBTDRU_GCB_EXTRA: &[&str] = &[
    // Populated from the cupel discovery run over Tools/rbk/rbgj*.
];

/// Bash builtins and command-position keywords that name no external command —
/// never flagged. Keywords with control-flow semantics ([[, ]], the loop/branch
/// words) are handled structurally in the lexer; the rest live here so that a
/// builtin reaching classification (echo, printf, read, local, …) is cleared.
pub(crate) const ZRBTDRU_BUILTINS: &[&str] = &[
    ":", ".", "[", "[[", "]", "]]", "alias", "bg", "bind", "break", "builtin",
    "caller", "cd", "command", "compgen", "complete", "compopt", "continue",
    "coproc", "declare", "dirs", "disown", "echo", "enable", "eval", "exec",
    "exit", "export", "false", "fc", "fg", "getopts", "hash", "help", "history",
    "jobs", "kill", "let", "local", "logout", "mapfile", "popd", "printf",
    "pushd", "pwd", "read", "readarray", "readonly", "return", "set", "shift",
    "shopt", "source", "suspend", "test", "times", "trap", "true", "type",
    "typeset", "ulimit", "umask", "unalias", "unset", "wait",
];

/// One evicted command and the BCG-prescribed replacement reported in its
/// stead. Verbatim from BCG's "Evicted Utilities" table.
pub(crate) struct zrbtdru_Eviction {
    pub(crate) command: &'static str,
    pub(crate) replacement: &'static str,
}

/// The enforced eviction table. In kit-bash these fail with the replacement; in
/// GCB-bash they are tolerated (the cloud-sdk image carries them and the
/// portability concern is moot).
pub(crate) const ZRBTDRU_EVICTIONS: &[zrbtdru_Eviction] = &[
    zrbtdru_Eviction { command: "awk", replacement: "read with IFS + parameter expansion" },
    zrbtdru_Eviction { command: "base64", replacement: "openssl enc -base64" },
    zrbtdru_Eviction { command: "cut", replacement: "read with IFS + parameter expansion" },
    zrbtdru_Eviction { command: "grep", replacement: "case / test / [[ =~ ]]" },
    zrbtdru_Eviction { command: "head", replacement: "read -r" },
    zrbtdru_Eviction { command: "ls", replacement: "glob expansion (for f in dir/*)" },
    zrbtdru_Eviction { command: "sha256sum", replacement: "openssl dgst -sha256 -r" },
    zrbtdru_Eviction { command: "shasum", replacement: "openssl dgst -sha256 -r" },
    zrbtdru_Eviction { command: "tr", replacement: "${var//old/new} parameter expansion" },
    zrbtdru_Eviction { command: "wc", replacement: "${#var} / ${#arr[@]}" },
];

// ── Domain and findings ─────────────────────────────────────

/// Execution-environment partition controlling allowlist strictness.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum zrbtdru_Domain {
    Kit,
    Gcb,
}

/// A single command-discipline violation: where it is and why it failed.
#[derive(Clone, Debug)]
pub(crate) struct zrbtdru_Finding {
    pub(crate) file: String,
    pub(crate) line: usize,
    pub(crate) command: String,
    pub(crate) detail: String,
}

// ── Lexer ───────────────────────────────────────────────────

/// Read one shell word starting at `*i`, advancing `*i` past it and `*line`
/// over any embedded newlines. Quoted segments and `${...}` parameter
/// expansions are consumed as part of the word; an embedded `$(` command
/// substitution STOPS the word so the caller's lexer scans the substituted
/// command at its own command position.
pub(crate) fn zrbtdru_read_word(chars: &[char], i: &mut usize, line: &mut usize) -> String {
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
pub(crate) fn zrbtdru_keyword_kind(word: &str) -> Option<bool> {
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
pub(crate) fn zrbtdru_is_assignment(word: &str) -> bool {
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
pub(crate) fn zrbtdru_skip_balanced_parens(chars: &[char], i: &mut usize, line: &mut usize, mut depth: usize) {
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
pub(crate) fn zrbtdru_command_words(src: &str) -> Vec<(usize, String)> {
    let chars: Vec<char> = src.chars().collect();
    let n = chars.len();
    let mut out: Vec<(usize, String)> = Vec::new();
    let mut i = 0usize;
    let mut line = 1usize;
    let mut cmd_pos = true;
    let mut paren_depth = 0usize;
    let mut in_dbracket = false;
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
                zrbtdru_skip_balanced_parens(&chars, &mut i, &mut line, 2);
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
                    zrbtdru_skip_balanced_parens(&chars, &mut i, &mut line, 2);
                    cmd_pos = false;
                    continue;
                }
                i += 2;
                paren_depth += 1;
                cmd_pos = true;
                continue;
            }
            let _ = zrbtdru_read_word(&chars, &mut i, &mut line);
            cmd_pos = false;
            continue;
        }
        if c == '{' {
            if i + 1 < n && (chars[i + 1] == ' ' || chars[i + 1] == '\t' || chars[i + 1] == '\n') {
                i += 1;
                cmd_pos = true;
                continue;
            }
            let _ = zrbtdru_read_word(&chars, &mut i, &mut line);
            cmd_pos = false;
            continue;
        }
        if c == '}' {
            i += 1;
            continue;
        }

        let word_line = line;
        let word = zrbtdru_read_word(&chars, &mut i, &mut line);
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
        if !cmd_pos {
            continue;
        }
        if zrbtdru_is_assignment(&word) {
            // `NAME=( … )` array literal — the elements are data, not commands.
            let mut j = i;
            while j < n && (chars[j] == ' ' || chars[j] == '\t') {
                j += 1;
            }
            if j < n && chars[j] == '(' {
                i = j + 1;
                zrbtdru_skip_balanced_parens(&chars, &mut i, &mut line, 1);
            }
            continue;
        }
        match zrbtdru_keyword_kind(&word) {
            Some(true) => continue,
            Some(false) => {
                cmd_pos = false;
                continue;
            }
            None => {}
        }
        out.push((word_line, word));
        cmd_pos = false;
    }
    out
}

// ── Function collection and classification ──────────────────

/// Harvest locally-defined function names from one source into `out`. Matches
/// both `name() {` and `function name` forms.
pub(crate) fn zrbtdru_collect_functions(src: &str, out: &mut BTreeSet<String>) {
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
pub(crate) fn zrbtdru_classify(
    command: &str,
    locals: &BTreeSet<String>,
    domain: zrbtdru_Domain,
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
    if ZRBTDRU_BUILTINS.contains(&base) {
        return None;
    }
    if locals.contains(command) || locals.contains(base) {
        return None;
    }
    if ZRBTDRU_POSIX_FLOOR.contains(&base) {
        return None;
    }
    if ZRBTDRU_DECLARED_DEPS.contains(&base) {
        return None;
    }
    match domain {
        zrbtdru_Domain::Kit => {
            for ev in ZRBTDRU_EVICTIONS {
                if ev.command == base {
                    return Some(format!("evicted command — use {}", ev.replacement));
                }
            }
            Some("unknown command — not in POSIX floor or RBS0 declared dependencies".to_string())
        }
        zrbtdru_Domain::Gcb => {
            if ZRBTDRU_GCB_EXTRA.contains(&base) {
                return None;
            }
            for ev in ZRBTDRU_EVICTIONS {
                if ev.command == base {
                    return None;
                }
            }
            Some("unknown command — not in the GCB allowlist".to_string())
        }
    }
}

// ── Corpus walk and scan ────────────────────────────────────

/// Recursively collect every `*.sh` file under `dir` into `out`, skipping any
/// subdirectory whose basename begins with one of `excluded_prefixes`.
fn zrbtdru_walk_sh(dir: &Path, excluded_prefixes: &[&str], out: &mut Vec<PathBuf>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let excluded = path
                .file_name()
                .and_then(|s| s.to_str())
                .map(|name| {
                    excluded_prefixes
                        .iter()
                        .any(|prefix| name.starts_with(prefix))
                })
                .unwrap_or(false);
            if excluded {
                continue;
            }
            zrbtdru_walk_sh(&path, excluded_prefixes, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some(ZRBTDRU_SH_EXT) {
            out.push(path);
        }
    }
}

/// True when `path` lies under a Google Cloud Build job directory (any path
/// component beginning with the rbgj prefix).
fn zrbtdru_is_gcb(path: &Path) -> bool {
    path.components().any(|comp| {
        comp.as_os_str()
            .to_str()
            .map(|s| s.starts_with(ZRBTDRU_GCB_DIR_PREFIX))
            .unwrap_or(false)
    })
}

/// Walk the corpus, collect functions across all of it, then scan the files
/// belonging to `domain`, returning every finding sorted by file and line.
fn zrbtdru_scan_domain(tools: &Path, domain: zrbtdru_Domain) -> Result<Vec<zrbtdru_Finding>, String> {
    // Pass 1 — function-visibility universe. Walk every kit so cross-kit and
    // sourced function names resolve (e.g. rbk's Windows handbook sources jjk's
    // zipper); only dead ABANDONED code stays invisible.
    let mut universe_files: Vec<PathBuf> = Vec::new();
    zrbtdru_walk_sh(tools, ZRBTDRU_UNIVERSE_EXCLUDED_DIR_PREFIXES, &mut universe_files);
    universe_files.sort();

    let mut locals: BTreeSet<String> = BTreeSet::new();
    for f in &universe_files {
        let src = std::fs::read_to_string(f)
            .map_err(|e| format!("read {} failed: {}", f.display(), e))?;
        zrbtdru_collect_functions(&src, &mut locals);
    }

    // Pass 2 — lint target. Only the release kit roots, minus dead/not-yet-live.
    let mut lint_files: Vec<PathBuf> = Vec::new();
    for kit in ZRBTDRU_KIT_ROOTS {
        zrbtdru_walk_sh(&tools.join(kit), ZRBTDRU_LINT_EXCLUDED_DIR_PREFIXES, &mut lint_files);
    }
    lint_files.sort();

    let root = tools.parent().unwrap_or(tools);
    let mut findings: Vec<zrbtdru_Finding> = Vec::new();
    for path in &lint_files {
        let is_gcb = zrbtdru_is_gcb(path);
        let in_domain = match domain {
            zrbtdru_Domain::Kit => !is_gcb,
            zrbtdru_Domain::Gcb => is_gcb,
        };
        if !in_domain {
            continue;
        }
        let src = std::fs::read_to_string(path)
            .map_err(|e| format!("read {} failed: {}", path.display(), e))?;
        let rel = path.strip_prefix(root).unwrap_or(path).display().to_string();
        for (line, command) in zrbtdru_command_words(&src) {
            if let Some(detail) = zrbtdru_classify(&command, &locals, domain) {
                findings.push(zrbtdru_Finding {
                    file: rel.clone(),
                    line,
                    command,
                    detail,
                });
            }
        }
    }
    Ok(findings)
}

/// Render findings as a stable one-per-line report.
fn zrbtdru_render(findings: &[zrbtdru_Finding]) -> String {
    let mut report = String::new();
    for f in findings {
        report.push_str(&format!("{}:{}: {} — {}\n", f.file, f.line, f.command, f.detail));
    }
    report
}

/// Drive one domain's scan, persist a findings trace into the case dir, and
/// fail the verdict when any violation remains.
fn zrbtdru_run_domain(dir: &Path, domain: zrbtdru_Domain, label: &str) -> rbtdre_Verdict {
    let root = match std::env::current_dir() {
        Ok(r) => r,
        Err(e) => return rbtdre_Verdict::Fail(format!("cannot get cwd: {}", e)),
    };
    let tools = root.join(ZRBTDRU_TOOLS_SUBDIR);
    let findings = match zrbtdru_scan_domain(&tools, domain) {
        Ok(f) => f,
        Err(e) => return rbtdre_Verdict::Fail(e),
    };
    let report = zrbtdru_render(&findings);
    let trace_name = format!("{}{}{}", ZRBTDRU_FINDINGS_PREFIX, label, ZRBTDRU_FINDINGS_SUFFIX);
    let _ = std::fs::write(dir.join(trace_name), &report);

    if findings.is_empty() {
        rbtdre_Verdict::Pass
    } else {
        rbtdre_Verdict::Fail(format!(
            "{} BCG command-discipline violation(s) in {}-bash:\n{}",
            findings.len(),
            label,
            report
        ))
    }
}

// ── Cases and fixture ───────────────────────────────────────

fn rbtdru_kit_bash(dir: &Path) -> rbtdre_Verdict {
    zrbtdru_run_domain(dir, zrbtdru_Domain::Kit, ZRBTDRU_LABEL_KIT)
}

fn rbtdru_gcb_bash(dir: &Path) -> rbtdre_Verdict {
    zrbtdru_run_domain(dir, zrbtdru_Domain::Gcb, ZRBTDRU_LABEL_GCB)
}

pub static RBTDRU_CASES_CUPEL: &[rbtdre_Case] = &[
    case!(rbtdru_kit_bash),
    case!(rbtdru_gcb_bash),
];

pub static RBTDRU_FIXTURE_CUPEL: rbtdre_Fixture = rbtdre_Fixture {
    name: RBTDRM_FIXTURE_CUPEL,
    disposition: rbtdre_Disposition::Independent,
    setup: None,
    teardown: None,
    cases: RBTDRU_CASES_CUPEL,
};
