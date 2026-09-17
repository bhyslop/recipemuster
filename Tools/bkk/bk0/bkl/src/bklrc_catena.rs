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

//! The admitted subset, and the reading of it.
//!
//! CONTINUATIONS JOIN BEFORE COMMENTS ARE READ, and that ordering is the one the
//! law fixes rather than a convenience of this implementation. A `#` inside a
//! continued value is content — bash is inside a quoted string there and no
//! comment is possible — so a reader that stripped `#` lines before joining
//! would delete an element and report success. The shape below makes the
//! ordering structural rather than remembered: a comment is recognized only
//! while no value stands open, because the loop that consumes a continuation
//! never asks the question at all.

use std::path::Path;

/// The character that opens a comment, on a line where none stands open.
const ZBKLRC_COMMENT: char = '#';

/// The one quote the subset admits. A single quote refuses wherever it stands:
/// bash gives it a different meaning from this one, and a reader that treated
/// the two alike would be guessing at a shell.
const ZBKLRC_QUOTE: char = '"';

/// The mark that continues a quoted value onto the next line.
const ZBKLRC_CONTINUE: char = '\\';

/// A file's assignments, in the order they were authored.
///
/// LAST ASSIGNMENT WINS, exactly as bash resolves one, and the reading is
/// faithful rather than opinionated: a repeated key is two admitted lines, so
/// refusing it here would make this reader narrower than the law in a direction
/// the law does not authorize. A family that wants a repeated field to be an
/// error says so in its own validator, where the meaning of the field lives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bklrc_Regime {
    entries: Vec<(String, String)>,
}

impl bklrc_Regime {
    /// One field's value as authored, joined across any continuation.
    pub fn bklrc_scalar(&self, key: &str) -> Option<&str> {
        self.entries
            .iter()
            .rev()
            .find(|(seated, _)| seated == key)
            .map(|(_, value)| value.as_str())
    }

    /// One field's value read as a catena: split on RUNS of whitespace, empty
    /// elements discarded, so leading and trailing whitespace carry no element
    /// and the indentation of a continued line delimits rather than appearing.
    ///
    /// `None` where the field is absent, which is a different answer from an
    /// empty catena: a door reading a whole-list meaning out of an ABSENCE
    /// cannot tell a declaration from an omission, and that is the one
    /// distinction a validator exists to draw.
    pub fn bklrc_catena(&self, key: &str) -> Option<Vec<&str>> {
        self.bklrc_scalar(key)
            .map(|value| value.split_whitespace().collect())
    }

    /// Whether the file assigns this field at all.
    pub fn bklrc_holds(&self, key: &str) -> bool {
        self.bklrc_scalar(key).is_some()
    }

    /// Every field the file assigns, in authored order, a repeated key standing
    /// once at its first seat.
    pub fn bklrc_keys(&self) -> Vec<&str> {
        let mut said: Vec<&str> = Vec::new();
        for (key, _) in &self.entries {
            if !said.iter().any(|seen| *seen == key.as_str()) {
                said.push(key.as_str());
            }
        }
        said
    }
}

/// Read a regime file from disk under the catena law.
///
/// The path is carried into every refusal, so a finding names the file and the
/// line together and a reader needs no second search to reach the offending
/// text.
pub fn bklrc_read(path: &Path) -> Result<bklrc_Regime, String> {
    let source = std::fs::read_to_string(path)
        .map_err(|err| format!("could not read the regime file at {}: {}", path.display(), err))?;

    bklrc_admit(&source, &path.display().to_string())
}

/// Read regime text already in hand, `whence` naming where it came from for the
/// sake of the refusals.
///
/// Split from the disk reading so a consumer holding bytes — and a hurdle
/// composing text — reaches the same parse rather than a second one.
pub fn bklrc_admit(source: &str, whence: &str) -> Result<bklrc_Regime, String> {
    let mut entries: Vec<(String, String)> = Vec::new();
    let mut lines = source.lines().enumerate();

    while let Some((index, raw)) = lines.next() {
        let numbered = index + 1;
        let trimmed = raw.trim();

        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with(ZBKLRC_COMMENT) {
            continue;
        }

        let (key, rest) = zbklrc_assignment(raw, whence, numbered)?;

        let value = match rest.strip_prefix(ZBKLRC_QUOTE) {
            Some(opened) => zbklrc_quoted(opened, &mut lines, whence, numbered)?,
            None => zbklrc_bare(rest, whence, numbered)?,
        };

        entries.push((key, value));
    }

    Ok(bklrc_Regime { entries })
}

/// Part a line into its key and everything after the first `=`.
///
/// THE KEY RUNS TO THE FIRST `=` and must be a shell name, which is what refuses
/// the forms the law names without needing a rule apiece: a leading `export`
/// leaves a space inside the key, a quoted key leaves a quote inside it, and a
/// line that is no assignment at all carries no `=` to split on.
///
/// Leading whitespace before the key is admitted and dropped. Bash reads an
/// indented assignment exactly as an unindented one, so refusing it would make
/// this reader narrower than the law without buying a single divergence.
fn zbklrc_assignment<'a>(
    raw: &'a str,
    whence: &str,
    line: usize,
) -> Result<(String, &'a str), String> {
    let split = raw.find('=').ok_or_else(|| {
        zbklrc_refusal(
            whence,
            line,
            raw,
            "it is neither a blank line, a comment, nor an assignment — the subset admits no other line",
        )
    })?;

    let key = raw[..split].trim_start();

    if key.is_empty() {
        return Err(zbklrc_refusal(whence, line, raw, "the key before the '=' is empty"));
    }

    if !zbklrc_named(key) {
        return Err(zbklrc_refusal(
            whence,
            line,
            raw,
            &format!(
                "'{}' is no shell name: a key runs to the first '=' and carries letters, digits \
                 and underscores alone, so an 'export' prefix, a quoted key, or whitespace \
                 standing against the '=' refuses here",
                key
            ),
        ));
    }

    Ok((key.to_string(), &raw[split + 1..]))
}

/// Whether a key is a shell name — a letter or underscore, then letters, digits
/// and underscores.
fn zbklrc_named(key: &str) -> bool {
    let mut chars = key.chars();

    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() || first == '_' => {}
        _ => return false,
    }

    chars.all(|held| held.is_ascii_alphanumeric() || held == '_')
}

/// An unquoted value: admitted only where it carries no whitespace and nothing
/// bash would expand.
///
/// TRAILING WHITESPACE IS DROPPED RATHER THAN REFUSED, because bash drops it
/// too — an assignment's value ends where the word ends — so refusing it would
/// be narrower than the law with no divergence bought. Whitespace INSIDE the
/// value is the refusal the law names, and it survives the trim.
///
/// AN EMPTY VALUE IS ADMITTED. `KEY=` assigns the empty string in bash and says
/// so unambiguously; the quoted `KEY=""` says the same thing more loudly, and a
/// family that wants the loud form says so in its own validator.
fn zbklrc_bare(rest: &str, whence: &str, line: usize) -> Result<String, String> {
    let value = rest.trim_end();

    if value.is_empty() {
        return Ok(String::new());
    }

    if value.chars().any(char::is_whitespace) {
        return Err(zbklrc_refusal(
            whence,
            line,
            rest,
            "an unquoted value carries whitespace — quote it, and continue it across lines with a \
             trailing backslash if it is long",
        ));
    }

    if value.contains('\'') {
        return Err(zbklrc_refusal(
            whence,
            line,
            rest,
            "a single-quoted value is outside the subset — bash gives single and double quotes \
             different meanings, and this reader admits the double quote alone",
        ));
    }

    if value.contains(ZBKLRC_QUOTE) {
        return Err(zbklrc_refusal(
            whence,
            line,
            rest,
            "the value is quoted at one end alone — a double-quoted value opens at the '=' and \
             closes on this line or on a continued one",
        ));
    }

    zbklrc_unexpanded(value, whence, line, rest)?;

    Ok(value.to_string())
}

/// A double-quoted value, joined across as many continued lines as it takes.
///
/// The line the value OPENED at is what a refusal names when the value is never
/// closed, because that is the line whose author has to fix it — the last line
/// of the file says nothing about where the mistake was made.
fn zbklrc_quoted<'a>(
    opened: &'a str,
    lines: &mut std::iter::Enumerate<std::str::Lines<'a>>,
    whence: &str,
    opening: usize,
) -> Result<String, String> {
    let mut value = String::new();
    let mut segment = opened;
    let mut line = opening;

    loop {
        match segment.find(ZBKLRC_QUOTE) {
            Some(at) => {
                let body = &segment[..at];
                zbklrc_unexpanded(body, whence, line, segment)?;
                value.push_str(body);

                let tail = segment[at + 1..].trim();
                if !tail.is_empty() {
                    return Err(zbklrc_refusal(
                        whence,
                        line,
                        segment,
                        "text stands after the closing quote — a comment is a line whose FIRST \
                         non-whitespace character is '#', so nothing trails a value on its own line",
                    ));
                }

                return Ok(value);
            }
            None => {
                let body = segment.strip_suffix(ZBKLRC_CONTINUE).ok_or_else(|| {
                    zbklrc_refusal(
                        whence,
                        line,
                        segment,
                        &format!(
                            "the value opened at line {} is quoted at one end alone: this line \
                             neither closes the quote nor ends in a backslash",
                            opening
                        ),
                    )
                })?;

                zbklrc_unexpanded(body, whence, line, segment)?;

                // THE BACKSLASH ALONE IS REMOVED, never the space before it.
                // Bash removes the backslash-newline pair and nothing else, so a
                // line ending `buk \` yields `buk ` and the next element stands
                // apart, while `buk\` would join two authored elements into one.
                value.push_str(body);

                let (index, raw) = lines.next().ok_or_else(|| {
                    zbklrc_refusal(
                        whence,
                        line,
                        segment,
                        &format!(
                            "the value opened at line {} is never closed — the file ends inside it",
                            opening
                        ),
                    )
                })?;

                line = index + 1;
                segment = raw;
            }
        }
    }
}

/// Refuse anything bash would expand, and any backslash the continuation rule
/// has not already accounted for.
///
/// The three are one check because they are one hazard: each would make the file
/// mean something to bash that it cannot mean to a reader holding no shell, and
/// this crate's promise is that the two answer alike. An escape is refused for
/// the same reason rather than interpreted — implementing bash's escape rules
/// here is implementing bash.
fn zbklrc_unexpanded(body: &str, whence: &str, line: usize, raw: &str) -> Result<(), String> {
    if body.contains('$') {
        return Err(zbklrc_refusal(
            whence,
            line,
            raw,
            "a variable reference stands in the value — a regime file states its values literally, \
             so that a reader holding no shell reads what bash reads",
        ));
    }

    if body.contains('`') {
        return Err(zbklrc_refusal(
            whence,
            line,
            raw,
            "command substitution stands in the value — a regime file states its values literally, \
             so that a reader holding no shell reads what bash reads",
        ));
    }

    if body.contains(ZBKLRC_CONTINUE) {
        return Err(zbklrc_refusal(
            whence,
            line,
            raw,
            "a backslash stands inside the value — the subset admits one only as the last \
             character of a continued line, and interpreting bash's escapes here would make this \
             reader a second implementation of a shell",
        ));
    }

    Ok(())
}

/// One refusal, spelled the one way: where, which line, what stands there, and
/// why it is outside the subset.
///
/// The offending text rides the message because a finding a reader has to go and
/// look up is a finding that costs a second search.
fn zbklrc_refusal(whence: &str, line: usize, raw: &str, why: &str) -> String {
    format!("{}:{}: {} — the line reads: {}", whence, line, why, raw.trim())
}

// eof
