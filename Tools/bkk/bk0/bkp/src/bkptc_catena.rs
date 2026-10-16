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

//! The reader's own hurdles.
//!
//! THEY DRIVE OVER FILES, not over strings, and the file stands under the
//! dispatch's temp root. The disk door is the one every consumer reaches, so a
//! suite that exercised the text door alone would leave the read, the path in
//! the refusal, and the missing-file arm unproven. Absent `BURD_TEMP_DIR` the
//! hurdle REFUSES and reaches for no ambient temp directory, so these are always
//! driven through the substrate.
//!
//! The kennel's lure surface is the richer seat — a composed git repository —
//! and it is deliberately not reached here: it stands in the kennel crate, and a
//! reader that depended on its own first consumer to prove itself could not be
//! the crate the other five readers converge onto.

use super::bkprc_catena::{bkprc_admit, bkprc_read};
use std::path::{Path, PathBuf};

/// The dispatch's temp root. ABSENT IS A REFUSAL, never a fallback.
const ZBKPTC_TEMP_ROOT_VAR: &str = "BURD_TEMP_DIR";

/// A composed seat holding one regime file, removed when the hurdle drops it.
struct zbkptc_Composed {
    root: PathBuf,
    file: PathBuf,
}

impl zbkptc_Composed {
    /// Compose a file under the temp root and hand back the seat.
    ///
    /// The name is the hurdle's, so a directory left behind by a panicking test
    /// says which hurdle made it.
    fn zbkptc_compose(name: &str, text: &str) -> zbkptc_Composed {
        let declared = std::env::var(ZBKPTC_TEMP_ROOT_VAR).unwrap_or_else(|_| {
            panic!(
                "{} is unset. The reader's hurdles stand under the dispatch's temp root and reach \
                 for no ambient temp directory; drive the suite through its own door",
                ZBKPTC_TEMP_ROOT_VAR
            )
        });

        let root = Path::new(&declared).join(format!("bkptc-{}", name));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root)
            .unwrap_or_else(|err| panic!("could not compose a seat at {}: {}", root.display(), err));

        let file = root.join("bkrr.env");
        std::fs::write(&file, text)
            .unwrap_or_else(|err| panic!("could not write {}: {}", file.display(), err));

        zbkptc_Composed { root, file }
    }

    /// Where the composed file stands.
    fn zbkptc_file(&self) -> &Path {
        &self.file
    }
}

impl Drop for zbkptc_Composed {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

/// A single-quoted value refuses, and the refusal names the line.
///
/// The line NUMBER is asserted rather than merely the refusal, because a reader
/// that refused the file without saying where has handed its author a search:
/// the whole point of refusing rather than skipping is that the author can go to
/// the line and fix it.
#[test]
fn zbkptc_single_quote_refuses_naming_the_line() {
    let seat = zbkptc_Composed::zbkptc_compose(
        "single-quote",
        "# a collar\nBKRR_COLLAR=\"suite-example\"\nBKRR_KIND='bknre_suite'\n",
    );

    let refusal = bkprc_read(seat.zbkptc_file())
        .expect_err("a single-quoted value stands outside the subset and must refuse");

    assert!(
        refusal.contains(":3:"),
        "the refusal must name the offending line, and it reads: {}",
        refusal
    );
    assert!(
        refusal.contains("single-quoted"),
        "the refusal must say what is outside the subset, and it reads: {}",
        refusal
    );
    assert!(
        refusal.contains(&seat.zbkptc_file().display().to_string()),
        "the refusal must name the file, and it reads: {}",
        refusal
    );
}

/// A three-line continuation whose middle line opens with `#` yields FOUR
/// elements, not two.
///
/// This is the ordering the law fixes, and it is the one defect that reports
/// success at every layer: a reader that stripped `#` lines before joining would
/// drop the middle line, hand two elements to everything downstream, and raise
/// no error at either layer.
#[test]
fn zbkptc_continuation_joins_before_comments_are_read() {
    let seat = zbkptc_Composed::zbkptc_compose(
        "hash-inside-continuation",
        "BKRR_ROOTS=\"one \\\n  #two three \\\n  four\"\n",
    );

    let regime = bkprc_read(seat.zbkptc_file()).expect("the file stands inside the subset");

    let elements = regime
        .bkprc_catena("BKRR_ROOTS")
        .expect("the field is assigned");

    assert_eq!(
        elements,
        vec!["one", "#two", "three", "four"],
        "the '#' line is CONTENT inside a quoted value: joining it before comments are read is \
         what yields four elements, and a reader that stripped it first would answer two"
    );
}

/// The authored layout of the law's own example reads back as its own elements,
/// and the space before each backslash keeps them apart.
#[test]
fn zbkptc_authored_catena_reads_as_its_elements() {
    let seat = zbkptc_Composed::zbkptc_compose(
        "authored-layout",
        "BURC_MANAGED_KITS=\"buk \\\n  vok \\\n  vvk\"\n",
    );

    let regime = bkprc_read(seat.zbkptc_file()).expect("the file stands inside the subset");

    assert_eq!(
        regime.bkprc_catena("BURC_MANAGED_KITS").unwrap(),
        vec!["buk", "vok", "vvk"]
    );
}

/// A backslash carrying no space before it JOINS two authored elements, exactly
/// as bash joins them — the one-character difference the law calls load-bearing.
///
/// THE PAIR IS DRIVEN AGAINST AN UNINDENTED CONTINUATION, and that is the
/// condition rather than a tidier spelling of one. Bash removes the
/// backslash-newline pair and NOTHING ELSE, so a continued line's own
/// indentation survives into the value and delimits on its own: `buk\` above an
/// indented `vok` still yields two elements, and the join the law warns about
/// bites only where the continued line opens at the margin. Asserted both ways
/// in one hurdle, because a reader is only useful if it is wrong in the same
/// places bash is wrong, and only the pair shows which character did the work.
#[test]
fn zbkptc_the_space_before_the_backslash_is_load_bearing() {
    let apart = bkprc_admit("K=\"buk \\\nvok\"\n", "composed").expect("inside the subset");
    assert_eq!(
        apart.bkprc_catena("K").unwrap(),
        vec!["buk", "vok"],
        "the space before the backslash keeps the elements apart"
    );

    let joined = bkprc_admit("K=\"buk\\\nvok\"\n", "composed").expect("inside the subset");
    assert_eq!(
        joined.bkprc_catena("K").unwrap(),
        vec!["bukvok"],
        "the same layout one character apart joins them, as bash joins them"
    );

    let indented = bkprc_admit("K=\"buk\\\n  vok\"\n", "composed").expect("inside the subset");
    assert_eq!(
        indented.bkprc_catena("K").unwrap(),
        vec!["buk", "vok"],
        "a continued line's own indentation survives the join and delimits on its own"
    );
}

/// Comments and blank lines carry no assignment, and a comment is recognized by
/// its first NON-whitespace character.
#[test]
fn zbkptc_comments_and_blanks_carry_nothing() {
    let regime = bkprc_admit(
        "# a heading\n\n   # an indented comment\nK=\"v\"\n\n",
        "composed",
    )
    .expect("inside the subset");

    assert_eq!(regime.bkprc_keys(), vec!["K"]);
    assert_eq!(regime.bkprc_scalar("K"), Some("v"));
}

/// An absent field and an empty one are DIFFERENT answers, which is the
/// distinction every validator downstream is built on.
#[test]
fn zbkptc_absent_and_empty_are_different_answers() {
    let regime = bkprc_admit("K=\"\"\nB=\n", "composed").expect("inside the subset");

    assert!(regime.bkprc_holds("K"), "a quoted empty value is a declaration");
    assert_eq!(regime.bkprc_scalar("K"), Some(""));
    assert_eq!(regime.bkprc_catena("K"), Some(Vec::new()));

    assert!(regime.bkprc_holds("B"), "a bare empty value is a declaration too");
    assert_eq!(regime.bkprc_scalar("B"), Some(""));

    assert!(!regime.bkprc_holds("ABSENT"));
    assert_eq!(regime.bkprc_scalar("ABSENT"), None);
    assert_eq!(regime.bkprc_catena("ABSENT"), None);
}

/// An unquoted value with no whitespace is admitted, and its trailing
/// whitespace is dropped rather than refused — bash drops it too.
#[test]
fn zbkptc_a_bare_word_is_admitted() {
    let regime = bkprc_admit("K=value   \n", "composed").expect("inside the subset");
    assert_eq!(regime.bkprc_scalar("K"), Some("value"));
}

/// Last assignment wins, as bash resolves one, and the key stands once in the
/// listing at its first seat.
#[test]
fn zbkptc_the_last_assignment_wins() {
    let regime = bkprc_admit("K=\"first\"\nJ=\"other\"\nK=\"second\"\n", "composed")
        .expect("inside the subset");

    assert_eq!(regime.bkprc_scalar("K"), Some("second"));
    assert_eq!(regime.bkprc_keys(), vec!["K", "J"]);
}

/// Every form the law names as outside the subset refuses, and each refusal
/// names its line.
///
/// One hurdle over a table rather than one apiece: the law states these as a
/// list, and a table keeps the reader's roster legible against the law's own.
#[test]
fn zbkptc_the_refused_forms_refuse() {
    let refused: &[(&str, &str, &str)] = &[
        ("an unquoted value carrying whitespace", "K=two words\n", ":1:"),
        ("a leading export", "export K=\"v\"\n", ":1:"),
        ("a variable reference", "K=\"$HOME\"\n", ":1:"),
        ("command substitution", "K=\"`date`\"\n", ":1:"),
        ("a value quoted at one end alone", "K=\"open\n", ":1:"),
        ("a value closed but never opened", "K=open\"\n", ":1:"),
        ("a line that is no assignment", "K=\"v\"\nnot an assignment\n", ":2:"),
        ("an escape inside a value", "K=\"a\\\"b\"\n", ":1:"),
        ("text after the closing quote", "K=\"v\" trailing\n", ":1:"),
        ("a trailing comment after a value", "K=\"v\" # why\n", ":1:"),
        ("a value never closed", "K=\"open \\\n  still open \\\n", ":2:"),
        ("an empty key", "=v\n", ":1:"),
    ];

    for (what, text, at) in refused {
        let refusal =
            bkprc_admit(text, "composed").expect_err(&format!("{} must refuse", what));
        assert!(
            refusal.contains(at),
            "the refusal for {} must name line {}, and it reads: {}",
            what,
            at,
            refusal
        );
    }
}

/// A file the reader cannot open refuses naming the path, rather than reading as
/// an empty regime.
#[test]
fn zbkptc_an_absent_file_refuses() {
    let seat = zbkptc_Composed::zbkptc_compose("absent-file", "K=\"v\"\n");
    let missing = seat.zbkptc_file().with_file_name("bkrp.env");

    let refusal = bkprc_read(&missing).expect_err("a file that is not there must refuse");

    assert!(
        refusal.contains("bkrp.env"),
        "the refusal must name the path it could not read, and it reads: {}",
        refusal
    );
}

/// The kennel's own collar shape reads back whole — the shape this crate was
/// founded for, driven end to end rather than field by field.
#[test]
fn zbkptc_a_collar_reads_back_whole() {
    let seat = zbkptc_Composed::zbkptc_compose(
        "collar-shape",
        "# The kennel's own suite collar.\n\
         \n\
         BKRR_COLLAR=\"bki_suite\"\n\
         BKRR_KIND=\"bknre_suite\"\n\
         BKRR_MANIFEST=\"Tools/bkk/bk0/Cargo.toml\"\n\
         BKRR_TARGET=\"bknre_manifest\"\n\
         BKRR_ROOTS=\"Tools/bkk/bk0/src \\\n  Tools/bkk/bk0/tests \\\n  Tools/bkk/bk0/Cargo.toml\"\n\
         BKRR_FEATURES=\"\"\n\
         BKRR_PROFILE=\"test\"\n\
         BKRR_SPEND=\"bknre_reader\"\n\
         \n\
         # EMPTY IS THE DECLARATION, never an omission.\n\
         BKRR_TAMED_CRATES=\"\"\n\
         BKRR_MUZZLE=\".\"\n\
         BKRR_RUNNER=\"bknre_cargo\"\n\
         BKRR_TONGUE=\"bknre_harness\"\n",
    );

    let regime = bkprc_read(seat.zbkptc_file()).expect("a collar stands inside the subset");

    assert_eq!(regime.bkprc_scalar("BKRR_COLLAR"), Some("bki_suite"));
    assert_eq!(regime.bkprc_scalar("BKRR_MUZZLE"), Some("."));
    assert_eq!(
        regime.bkprc_catena("BKRR_ROOTS").unwrap(),
        vec![
            "Tools/bkk/bk0/src",
            "Tools/bkk/bk0/tests",
            "Tools/bkk/bk0/Cargo.toml"
        ]
    );
    assert_eq!(regime.bkprc_catena("BKRR_TAMED_CRATES"), Some(Vec::new()));
    assert!(!regime.bkprc_holds("BKRR_RESIDENCE"));
}

// eof
