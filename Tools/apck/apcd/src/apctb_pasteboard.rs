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

//! Tests for `apcrb_pasteboard` — UTI-to-extension mapping and sanitizer.
//! The NSPasteboard FFI itself is not unit-testable (requires a live
//! pasteboard and main-thread AppKit context); verification of the FFI
//! path is by deployed run on Ann's machine per the pace exit criteria.

use super::apcrb_pasteboard::{apcrb_extension_for_uti, zapcrb_sanitize_uti_for_test};

#[test]
fn apctb_extension_known_public_utis() {
    assert_eq!(apcrb_extension_for_uti("public.rtf"),              "rtf");
    assert_eq!(apcrb_extension_for_uti("public.utf8-plain-text"),  "utf8.txt");
    assert_eq!(apcrb_extension_for_uti("public.utf16-plain-text"), "utf16.txt");
    assert_eq!(apcrb_extension_for_uti("public.plain-text"),       "txt");
    assert_eq!(apcrb_extension_for_uti("public.html"),             "html");
}

#[test]
fn apctb_extension_known_image_utis() {
    assert_eq!(apcrb_extension_for_uti("public.tiff"), "tiff");
    assert_eq!(apcrb_extension_for_uti("public.png"),  "png");
    assert_eq!(apcrb_extension_for_uti("public.jpeg"), "jpeg");
}

#[test]
fn apctb_extension_known_url_utis() {
    assert_eq!(apcrb_extension_for_uti("public.url"),      "url");
    assert_eq!(apcrb_extension_for_uti("public.file-url"), "fileurl");
}

#[test]
fn apctb_extension_unknown_uti_sanitizes_with_bin() {
    // Unknown but well-formed reverse-dns UTI — dots collapse to hyphens.
    assert_eq!(
        apcrb_extension_for_uti("com.example.proprietary.format"),
        "com-example-proprietary-format.bin",
    );
}

#[test]
fn apctb_extension_legacy_nspasteboard_names_go_through_sanitizer() {
    // Docket explicitly notes legacy names (`NSStringPboardType`,
    // `Unicode text`) take the sanitized-uti path rather than bespoke
    // mappings — they are not canonical flavors, just historical aliases.
    assert_eq!(
        apcrb_extension_for_uti("NSStringPboardType"),
        "NSStringPboardType.bin",
    );
    assert_eq!(
        apcrb_extension_for_uti("Unicode text"),
        "Unicode-text.bin",
    );
    assert_eq!(
        apcrb_extension_for_uti("string"),
        "string.bin",
    );
}

#[test]
fn apctb_sanitize_collapses_run_of_nonkeep_to_single_hyphen() {
    // "a...b" should not produce "a---b" — run of forbidden chars
    // collapses to one hyphen.
    assert_eq!(zapcrb_sanitize_uti_for_test("a...b"),   "a-b");
    assert_eq!(zapcrb_sanitize_uti_for_test("x / y"),   "x-y");
    assert_eq!(zapcrb_sanitize_uti_for_test("a..b..c"), "a-b-c");
}

#[test]
fn apctb_sanitize_trims_leading_and_trailing_hyphens() {
    assert_eq!(zapcrb_sanitize_uti_for_test(".leading"),      "leading");
    assert_eq!(zapcrb_sanitize_uti_for_test("trailing."),     "trailing");
    assert_eq!(zapcrb_sanitize_uti_for_test(".both.sides."),  "both-sides");
}

#[test]
fn apctb_sanitize_scrubs_forbidden_filename_chars() {
    // Path separators, shell metacharacters, and whitespace must never
    // appear in the emitted filename.
    assert_eq!(zapcrb_sanitize_uti_for_test("a/b\\c:d*e?f"), "a-b-c-d-e-f");
    assert_eq!(zapcrb_sanitize_uti_for_test("a b c"),        "a-b-c");
    assert_eq!(zapcrb_sanitize_uti_for_test("a\tb\nc"),      "a-b-c");
}

#[test]
fn apctb_sanitize_preserves_existing_hyphens_and_alphanumerics() {
    assert_eq!(
        zapcrb_sanitize_uti_for_test("com-example-foo-bar-42"),
        "com-example-foo-bar-42",
    );
}

#[test]
fn apctb_sanitize_empty_yields_placeholder() {
    // A pathological empty UTI must still produce a non-empty filename
    // token so `{index}-in.{tag}..bin` — a double-dot — never happens.
    assert_eq!(zapcrb_sanitize_uti_for_test(""),       "flavor");
    assert_eq!(zapcrb_sanitize_uti_for_test("..."),    "flavor");
    assert_eq!(zapcrb_sanitize_uti_for_test("   "),    "flavor");
}

#[test]
fn apctb_extension_unknown_uti_is_filename_safe() {
    // Property: for any input, the resulting extension contains no
    // path separators, no whitespace, no control characters.
    let samples = [
        "com.example/evil",
        "weird name with spaces",
        "tab\there",
        "dot.in.middle",
        "",
        "///",
        "x",
    ];
    for s in &samples {
        let ext = apcrb_extension_for_uti(s);
        for c in ext.chars() {
            assert!(
                c == '-' || c == '.' || c.is_ascii_alphanumeric(),
                "extension {:?} from input {:?} contains forbidden char {:?}",
                ext, s, c
            );
        }
    }
}
