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

//! The JSON reader's hurdles.
//!
//! They drive over composed text rather than over cargo, deliberately: what
//! cargo says is proven where the resolver is proven, and a reader's own laws —
//! null against absent, escapes, strictness — are provable without spending a
//! cargo invocation apiece.

use super::bkcj_json::{bkcj_read, bkcj_Value};

/// The shape the closure reading actually walks, read back field by field.
#[test]
fn bktj_a_cargo_shaped_answer_reads_back() {
    let answer = bkcj_read(
        r#"{"version":1,"packages":[
             {"name":"bkk","source":null,"manifest_path":"/x/Tools/bkk/bk0/Cargo.toml",
              "dependencies":[{"name":"bkl","kind":null}]},
             {"name":"serde","source":"registry+https://example.invalid",
              "manifest_path":"/reg/serde/Cargo.toml","dependencies":[]}]}"#,
    )
    .expect("cargo's own shape is readable");

    assert_eq!(answer.bkcj_field("version").unwrap().bkcj_number(), Some(1.0));

    let packages = answer.bkcj_field("packages").unwrap().bkcj_array().unwrap();
    assert_eq!(packages.len(), 2);

    let local = &packages[0];
    assert!(
        local.bkcj_field("source").unwrap().bkcj_null(),
        "a package cargo reports with no source is one standing in a tree of ours"
    );
    assert_eq!(
        local.bkcj_field("manifest_path").unwrap().bkcj_string(),
        Some("/x/Tools/bkk/bk0/Cargo.toml")
    );
    assert_eq!(
        local.bkcj_field("dependencies").unwrap().bkcj_array().unwrap()[0]
            .bkcj_field("name")
            .unwrap()
            .bkcj_string(),
        Some("bkl")
    );

    assert!(
        !packages[1].bkcj_field("source").unwrap().bkcj_null(),
        "a registry package states its origin"
    );
}

/// NULL AND ABSENT ARE DIFFERENT ANSWERS, which is the distinction the closure
/// reading turns on: a reader that could not tell them apart would read a
/// malformed answer as a tree full of local crates.
#[test]
fn bktj_null_and_absent_are_different_answers() {
    let answer = bkcj_read(r#"{"stated":null}"#).expect("readable");

    assert!(answer.bkcj_field("stated").unwrap().bkcj_null());
    assert!(answer.bkcj_field("unstated").is_none());
}

/// Escapes resolve, surrogate pairs among them, so a path is read as the path it
/// is rather than as the text cargo had to write it as.
#[test]
fn bktj_escapes_resolve() {
    let answer = bkcj_read(r#"{"k":"a\"b\\c\/d\ne\tfA🐕"}"#).expect("readable");

    assert_eq!(
        answer.bkcj_field("k").unwrap().bkcj_string(),
        Some("a\"b\\c/d\ne\tfA\u{1f415}")
    );
}

/// A value carrying multi-byte text is copied whole rather than split, which is
/// what a station whose directories are not named in ASCII depends on.
#[test]
fn bktj_raw_multibyte_text_survives() {
    let answer = bkcj_read("{\"k\":\"caño/日本\"}").expect("readable");
    assert_eq!(answer.bkcj_field("k").unwrap().bkcj_string(), Some("caño/日本"));
}

/// Nesting and emptiness, both directions.
#[test]
fn bktj_containers_nest_and_stand_empty() {
    let answer = bkcj_read(r#"{"a":[],"b":{},"c":[[1,2],{"d":true},false]}"#).expect("readable");

    assert_eq!(answer.bkcj_field("a").unwrap().bkcj_array().unwrap().len(), 0);
    assert_eq!(answer.bkcj_field("b").unwrap(), &bkcj_Value::Object(Vec::new()));

    let held = answer.bkcj_field("c").unwrap().bkcj_array().unwrap();
    assert_eq!(held[0].bkcj_array().unwrap()[1].bkcj_number(), Some(2.0));
    assert_eq!(held[1].bkcj_field("d").unwrap(), &bkcj_Value::Bool(true));
    assert_eq!(held[2], bkcj_Value::Bool(false));
}

/// Numbers in every shape cargo can write one.
#[test]
fn bktj_numbers_read_in_every_shape() {
    let answer = bkcj_read(r#"{"a":0,"b":-17,"c":1.5,"d":2e3,"e":-1.25E-2}"#).expect("readable");

    assert_eq!(answer.bkcj_field("a").unwrap().bkcj_number(), Some(0.0));
    assert_eq!(answer.bkcj_field("b").unwrap().bkcj_number(), Some(-17.0));
    assert_eq!(answer.bkcj_field("c").unwrap().bkcj_number(), Some(1.5));
    assert_eq!(answer.bkcj_field("d").unwrap().bkcj_number(), Some(2000.0));
    assert_eq!(answer.bkcj_field("e").unwrap().bkcj_number(), Some(-0.0125));
}

/// THE READER IS STRICT, and every one of these is a shape a generous reader
/// would have accepted. The input is a machine's output, so anything outside
/// JSON means the reading has gone wrong rather than that the writer was being
/// generous — and a reader that healed it would report on text nobody wrote.
#[test]
fn bktj_the_generous_shapes_refuse() {
    let refused: &[(&str, &str)] = &[
        ("a trailing comma in an object", r#"{"a":1,}"#),
        ("a trailing comma in an array", r#"[1,2,]"#),
        ("an unquoted key", r#"{a:1}"#),
        ("a single-quoted string", r#"{"a":'b'}"#),
        ("a comment", "{\"a\":1} // why"),
        ("a second value after the first", r#"{"a":1}{"b":2}"#),
        ("an unclosed object", r#"{"a":1"#),
        ("an unclosed string", r#"{"a":"b}"#),
        ("a bare word", "nope"),
        ("nothing at all", ""),
        ("a raw newline inside a string", "{\"a\":\"b\nc\"}"),
        ("an escape JSON does not have", r#"{"a":"\q"}"#),
        ("a lone leading surrogate", r#"{"a":"\uD83D"}"#),
        ("a short hex escape", r#"{"a":"\u00"}"#),
    ];

    for (what, text) in refused {
        let refusal = bkcj_read(text).expect_err(&format!("{} must refuse", what));
        assert!(
            refusal.contains("byte"),
            "the refusal for {} must name where the reading stopped, and it reads: {}",
            what,
            refusal
        );
    }
}

/// A duplicate key answers with the LAST, which is what every mainstream reader
/// does and therefore what a writer's own round trip assumes.
#[test]
fn bktj_a_duplicate_key_answers_with_the_last() {
    let answer = bkcj_read(r#"{"a":1,"a":2}"#).expect("readable");
    assert_eq!(answer.bkcj_field("a").unwrap().bkcj_number(), Some(2.0));
}

/// Nesting past the cap refuses rather than blowing the stack, so corrupt input
/// meets a report instead of a crash nobody can report on.
#[test]
fn bktj_runaway_nesting_refuses() {
    let deep = format!("{}{}", "[".repeat(500), "]".repeat(500));
    let refusal = bkcj_read(&deep).expect_err("nesting past the cap must refuse");
    assert!(refusal.contains("nests deeper"), "it reads: {}", refusal);
}

// eof
