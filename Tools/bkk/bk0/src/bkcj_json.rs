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

//! A reader for cargo's own machine-readable answers.
//!
//! WHY THE KENNEL CARRIES ONE AT ALL. `cargo metadata` is the currency oracle
//! this estate elected — nothing here reimplements what cargo knows
//! (BKSNC-Kennelcraft.adoc "The Rust Backend") — and it answers in JSON. The
//! kennel declares zero external crates and its collar says so as data, so the
//! serialization crate a richer consumer reaches for is not available here and
//! adding one would put a dependency closure under the door whose whole job is
//! to police dependency closures.
//!
//! IT IS A READER AND NEVER A WRITER. Nothing in the kennel emits JSON: the
//! proclamation's transport is env-shaped text, and a JSON transport arrives as
//! a verb of its own when a consumer stands for one (BKSCL-Collar.adoc "The
//! Common Proclamation"). A writer here would be built for nobody.
//!
//! THE FOREIGN SCHEMA KEEPS ITS FOREIGN NAMES. The type below spells JSON's own
//! six kinds, quoted rather than minted, for the same reason the leash spells
//! `--locked`: these are names this estate did not choose.
//!
//! It is STRICT — trailing commas, unquoted keys, comments and single quotes all
//! refuse — because the input is a machine's output and anything else in it means
//! the reading has gone wrong rather than that the writer was being generous.

/// How deep a nesting the reader will follow before refusing.
///
/// Cargo's own answers nest a handful deep. The cap is here so that hostile or
/// corrupt input meets a refusal rather than a blown stack, which is a crash the
/// caller cannot report on.
const ZBKCJ_DEPTH_LIMIT: usize = 128;

/// A JSON value, in JSON's own six kinds.
#[derive(Debug, Clone, PartialEq)]
pub enum bkcj_Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<bkcj_Value>),
    Object(Vec<(String, bkcj_Value)>),
}

impl bkcj_Value {
    /// One field of an object, or `None` where this is no object or holds no
    /// such field. A duplicate key answers with the LAST, which is what every
    /// mainstream reader does and therefore what a writer's own round trip
    /// assumes.
    pub fn bkcj_field(&self, key: &str) -> Option<&bkcj_Value> {
        match self {
            bkcj_Value::Object(fields) => fields
                .iter()
                .rev()
                .find(|(seated, _)| seated == key)
                .map(|(_, value)| value),
            _ => None,
        }
    }

    /// This value as text, or `None` where it is not a string.
    pub fn bkcj_string(&self) -> Option<&str> {
        match self {
            bkcj_Value::String(text) => Some(text.as_str()),
            _ => None,
        }
    }

    /// This value as a list, or `None` where it is not an array.
    pub fn bkcj_array(&self) -> Option<&[bkcj_Value]> {
        match self {
            bkcj_Value::Array(held) => Some(held.as_slice()),
            _ => None,
        }
    }

    /// This value as a number, or `None` where it is not one.
    pub fn bkcj_number(&self) -> Option<f64> {
        match self {
            bkcj_Value::Number(held) => Some(*held),
            _ => None,
        }
    }

    /// Whether this value is JSON's null. DISTINCT from an absent field, which
    /// is the distinction the local-dependency reading turns on: cargo states a
    /// registry package's origin and states `null` for a local one, and a reader
    /// that could not tell null from missing would read a malformed answer as a
    /// tree full of local crates.
    pub fn bkcj_null(&self) -> bool {
        matches!(self, bkcj_Value::Null)
    }
}

/// Read one JSON value, which must be the whole of the text.
///
/// Trailing text refuses rather than being ignored: a reader that stopped at the
/// first complete value would accept two concatenated answers as one and report
/// on whichever came first.
pub fn bkcj_read(source: &str) -> Result<bkcj_Value, String> {
    let mut reader = zbkcj_Reader {
        bytes: source.as_bytes(),
        at: 0,
        depth: 0,
    };

    reader.zbkcj_blank();
    let value = reader.zbkcj_value()?;
    reader.zbkcj_blank();

    if reader.at != reader.bytes.len() {
        return Err(reader.zbkcj_grievance("text stands after the value the answer ended with"));
    }

    Ok(value)
}

/// The cursor, and the whole of the reader's state.
struct zbkcj_Reader<'a> {
    bytes: &'a [u8],
    at: usize,
    depth: usize,
}

impl zbkcj_Reader<'_> {
    /// A refusal naming where the reading stopped. The offset rather than a line
    /// and column, because cargo answers on one line and a column number would
    /// be the same number said twice.
    fn zbkcj_grievance(&self, why: &str) -> String {
        format!("the answer is no JSON at byte {}: {}", self.at, why)
    }

    /// What stands under the cursor, or `None` at the end.
    fn zbkcj_peek(&self) -> Option<u8> {
        self.bytes.get(self.at).copied()
    }

    /// Step past JSON's four whitespace characters, and no others.
    fn zbkcj_blank(&mut self) {
        while let Some(held) = self.zbkcj_peek() {
            match held {
                b' ' | b'\t' | b'\n' | b'\r' => self.at += 1,
                _ => break,
            }
        }
    }

    /// Step past one expected byte, or refuse naming what stood there instead.
    fn zbkcj_expect(&mut self, wanted: u8) -> Result<(), String> {
        match self.zbkcj_peek() {
            Some(held) if held == wanted => {
                self.at += 1;
                Ok(())
            }
            Some(held) => Err(self.zbkcj_grievance(&format!(
                "'{}' was expected and '{}' stands there",
                wanted as char, held as char
            ))),
            None => Err(self.zbkcj_grievance(&format!(
                "'{}' was expected and the answer ended",
                wanted as char
            ))),
        }
    }

    /// One value of any kind.
    fn zbkcj_value(&mut self) -> Result<bkcj_Value, String> {
        match self.zbkcj_peek() {
            Some(b'{') => self.zbkcj_object(),
            Some(b'[') => self.zbkcj_array(),
            Some(b'"') => Ok(bkcj_Value::String(self.zbkcj_string()?)),
            Some(b't') => self.zbkcj_word("true").map(|_| bkcj_Value::Bool(true)),
            Some(b'f') => self.zbkcj_word("false").map(|_| bkcj_Value::Bool(false)),
            Some(b'n') => self.zbkcj_word("null").map(|_| bkcj_Value::Null),
            Some(held) if held == b'-' || held.is_ascii_digit() => self.zbkcj_number(),
            Some(held) => Err(self
                .zbkcj_grievance(&format!("'{}' opens no JSON value", held as char))),
            None => Err(self.zbkcj_grievance("a value was expected and the answer ended")),
        }
    }

    /// A bare literal — `true`, `false` or `null` — matched whole.
    fn zbkcj_word(&mut self, word: &str) -> Result<(), String> {
        if self.bytes[self.at..].starts_with(word.as_bytes()) {
            self.at += word.len();
            return Ok(());
        }
        Err(self.zbkcj_grievance(&format!("'{}' was expected here", word)))
    }

    /// An object, its fields in the order they were written.
    fn zbkcj_object(&mut self) -> Result<bkcj_Value, String> {
        self.zbkcj_deeper()?;
        self.zbkcj_expect(b'{')?;
        let mut fields: Vec<(String, bkcj_Value)> = Vec::new();

        self.zbkcj_blank();
        if self.zbkcj_peek() == Some(b'}') {
            self.at += 1;
            self.depth -= 1;
            return Ok(bkcj_Value::Object(fields));
        }

        loop {
            self.zbkcj_blank();
            let key = self.zbkcj_string()?;
            self.zbkcj_blank();
            self.zbkcj_expect(b':')?;
            self.zbkcj_blank();
            let value = self.zbkcj_value()?;
            fields.push((key, value));

            self.zbkcj_blank();
            match self.zbkcj_peek() {
                Some(b',') => self.at += 1,
                Some(b'}') => {
                    self.at += 1;
                    self.depth -= 1;
                    return Ok(bkcj_Value::Object(fields));
                }
                _ => return Err(self.zbkcj_grievance("',' or '}' was expected after a field")),
            }
        }
    }

    /// An array, in order.
    fn zbkcj_array(&mut self) -> Result<bkcj_Value, String> {
        self.zbkcj_deeper()?;
        self.zbkcj_expect(b'[')?;
        let mut held: Vec<bkcj_Value> = Vec::new();

        self.zbkcj_blank();
        if self.zbkcj_peek() == Some(b']') {
            self.at += 1;
            self.depth -= 1;
            return Ok(bkcj_Value::Array(held));
        }

        loop {
            self.zbkcj_blank();
            held.push(self.zbkcj_value()?);

            self.zbkcj_blank();
            match self.zbkcj_peek() {
                Some(b',') => self.at += 1,
                Some(b']') => {
                    self.at += 1;
                    self.depth -= 1;
                    return Ok(bkcj_Value::Array(held));
                }
                _ => return Err(self.zbkcj_grievance("',' or ']' was expected after an element")),
            }
        }
    }

    /// Descend one level, refusing past the cap.
    fn zbkcj_deeper(&mut self) -> Result<(), String> {
        self.depth += 1;
        if self.depth > ZBKCJ_DEPTH_LIMIT {
            return Err(self.zbkcj_grievance(&format!(
                "the answer nests deeper than {} levels, which no cargo answer does — reading on \
                 would risk the stack rather than the reading",
                ZBKCJ_DEPTH_LIMIT
            )));
        }
        Ok(())
    }

    /// A quoted string, escapes resolved.
    fn zbkcj_string(&mut self) -> Result<String, String> {
        self.zbkcj_expect(b'"')?;
        let mut said = String::new();

        loop {
            let held = self
                .zbkcj_peek()
                .ok_or_else(|| self.zbkcj_grievance("a string is never closed"))?;

            match held {
                b'"' => {
                    self.at += 1;
                    return Ok(said);
                }
                b'\\' => {
                    self.at += 1;
                    said.push(self.zbkcj_escaped()?);
                }
                0x00..=0x1f => {
                    return Err(self.zbkcj_grievance(
                        "a raw control character stands inside a string, which JSON forbids",
                    ))
                }
                _ => {
                    // Step by whole UTF-8 sequences, so a multi-byte character
                    // is copied rather than split. The source was a `&str`, so
                    // every sequence in it is already well formed.
                    let width = zbkcj_width(held);
                    let upto = (self.at + width).min(self.bytes.len());
                    match std::str::from_utf8(&self.bytes[self.at..upto]) {
                        Ok(text) => said.push_str(text),
                        Err(_) => {
                            return Err(self.zbkcj_grievance("a string holds no readable text"))
                        }
                    }
                    self.at = upto;
                }
            }
        }
    }

    /// One escape, the backslash already stepped past.
    fn zbkcj_escaped(&mut self) -> Result<char, String> {
        let held = self
            .zbkcj_peek()
            .ok_or_else(|| self.zbkcj_grievance("an escape opens and the answer ends"))?;
        self.at += 1;

        let resolved = match held {
            b'"' => '"',
            b'\\' => '\\',
            b'/' => '/',
            b'b' => '\u{8}',
            b'f' => '\u{c}',
            b'n' => '\n',
            b'r' => '\r',
            b't' => '\t',
            b'u' => return self.zbkcj_coded(),
            _ => {
                return Err(self.zbkcj_grievance(&format!(
                    "'\\{}' is no JSON escape",
                    held as char
                )))
            }
        };

        Ok(resolved)
    }

    /// A `\u` escape, joining a surrogate pair where one stands.
    ///
    /// The pair is handled rather than refused because a path carrying a
    /// character outside the basic plane is a real path, and refusing it would
    /// make the reading depend on what a station's directories are called.
    fn zbkcj_coded(&mut self) -> Result<char, String> {
        let lead = self.zbkcj_hex()?;

        if (0xd800..=0xdbff).contains(&lead) {
            self.zbkcj_expect(b'\\')?;
            self.zbkcj_expect(b'u')?;
            let trail = self.zbkcj_hex()?;

            if !(0xdc00..=0xdfff).contains(&trail) {
                return Err(self.zbkcj_grievance(
                    "a leading surrogate is not followed by a trailing one",
                ));
            }

            let joined = 0x10000 + ((lead - 0xd800) << 10) + (trail - 0xdc00);
            return char::from_u32(joined)
                .ok_or_else(|| self.zbkcj_grievance("a surrogate pair names no character"));
        }

        char::from_u32(lead).ok_or_else(|| self.zbkcj_grievance("an escape names no character"))
    }

    /// Four hex digits as one number.
    fn zbkcj_hex(&mut self) -> Result<u32, String> {
        if self.at + 4 > self.bytes.len() {
            return Err(self.zbkcj_grievance("a '\\u' escape wants four hex digits"));
        }

        let text = std::str::from_utf8(&self.bytes[self.at..self.at + 4])
            .map_err(|_| self.zbkcj_grievance("a '\\u' escape wants four hex digits"))?;

        let held = u32::from_str_radix(text, 16)
            .map_err(|_| self.zbkcj_grievance("a '\\u' escape wants four hex digits"))?;

        self.at += 4;
        Ok(held)
    }

    /// A number, taken as JSON spells it and handed to the platform's own parse.
    ///
    /// The span is bounded by JSON's own grammar rather than by a hand-rolled
    /// state machine, and what it yields is then parsed by the standard library:
    /// re-deriving float parsing here would be the second implementation this
    /// module exists to avoid.
    fn zbkcj_number(&mut self) -> Result<bkcj_Value, String> {
        let opened = self.at;

        if self.zbkcj_peek() == Some(b'-') {
            self.at += 1;
        }

        while let Some(held) = self.zbkcj_peek() {
            match held {
                b'0'..=b'9' | b'.' | b'e' | b'E' | b'+' | b'-' => self.at += 1,
                _ => break,
            }
        }

        let text = std::str::from_utf8(&self.bytes[opened..self.at])
            .map_err(|_| self.zbkcj_grievance("a number holds no readable text"))?;

        text.parse::<f64>()
            .map(bkcj_Value::Number)
            .map_err(|_| {
                let at = self.at;
                self.at = opened;
                let said = format!("'{}' is no JSON number", text);
                self.at = at;
                self.zbkcj_grievance(&said)
            })
    }
}

/// How many bytes a UTF-8 sequence opening on this byte occupies.
fn zbkcj_width(lead: u8) -> usize {
    match lead {
        0x00..=0x7f => 1,
        0xc0..=0xdf => 2,
        0xe0..=0xef => 3,
        _ => 4,
    }
}

// eof
