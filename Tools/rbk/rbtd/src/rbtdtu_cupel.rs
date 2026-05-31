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
// RBTDTU — unit tests for the cupel command-position lexer and classifier.

use std::collections::BTreeSet;

use super::rbtdru_cupel::{
    zrbtdru_classify,
    zrbtdru_collect_functions,
    zrbtdru_command_words,
    zrbtdru_is_assignment,
    zrbtdru_keyword_kind,
    zrbtdru_Domain,
    ZRBTDRU_DECLARED_DEPS,
    ZRBTDRU_EVICTIONS,
    ZRBTDRU_GCB_ALLOWED,
    ZRBTDRU_POSIX_FLOOR,
};

/// Command-position tokens of `src`, as bare strings (line numbers dropped).
fn cmds(src: &str) -> Vec<String> {
    zrbtdru_command_words(src).into_iter().map(|(_, w)| w).collect()
}

#[test]
fn rbtdtu_simple_command() {
    assert_eq!(cmds("grep foo"), vec!["grep"]);
}

#[test]
fn rbtdtu_pipe_opens_command_position() {
    assert_eq!(cmds("cat x | grep y"), vec!["cat", "grep"]);
}

#[test]
fn rbtdtu_or_die_pattern() {
    assert_eq!(cmds("mything arg || buc_die"), vec!["mything", "buc_die"]);
}

#[test]
fn rbtdtu_command_substitution_assignment() {
    // The assignment prefix preserves command position; the substituted
    // command is scanned at its own position.
    assert_eq!(cmds("z_x=$(grep foo)"), vec!["grep"]);
}

#[test]
fn rbtdtu_comment_ignored() {
    assert!(cmds("# grep foo").is_empty());
}

#[test]
fn rbtdtu_trailing_comment_ignored() {
    assert_eq!(cmds("grep foo # not a command"), vec!["grep"]);
}

#[test]
fn rbtdtu_quoted_argument_not_a_command() {
    assert_eq!(cmds("echo 'grep'"), vec!["echo"]);
}

#[test]
fn rbtdtu_double_bracket_contents_skipped() {
    assert_eq!(cmds("[[ -n $x ]] && grep y"), vec!["grep"]);
}

#[test]
fn rbtdtu_bare_assignment_is_not_a_command() {
    assert!(cmds("FOO=bar").is_empty());
}

#[test]
fn rbtdtu_assignment_prefix_then_command() {
    assert_eq!(cmds("FOO=bar grep x"), vec!["grep"]);
}

#[test]
fn rbtdtu_if_then_fi_keywords() {
    assert_eq!(cmds("if grep x; then mything; fi"), vec!["grep", "mything"]);
}

#[test]
fn rbtdtu_heredoc_body_skipped() {
    let src = "cat <<EOF\ngrep evil\nEOF\nreal_cmd";
    assert_eq!(cmds(src), vec!["cat", "real_cmd"]);
}

#[test]
fn rbtdtu_here_string_is_not_heredoc() {
    // `<<<` is a single-line here-string redirection, not a here-doc; the
    // following line is normal source.
    assert_eq!(cmds("grep x <<< \"$y\"\nmything"), vec!["grep", "mything"]);
}

#[test]
fn rbtdtu_arithmetic_not_scanned() {
    // `(( ... ))` and `$(( ... ))` are arithmetic, not command lists.
    assert_eq!(cmds("(( i++ ))\nmything"), vec!["mything"]);
    assert_eq!(cmds("z=$(( a + b ))\nmything"), vec!["mything"]);
}

#[test]
fn rbtdtu_nested_arithmetic_parens_balanced() {
    // Inner parens inside `$(( ... ))` must not terminate the arithmetic early.
    let src = "z=$(( (0xFFFFFFFF << (32 - m)) & 0xFFFFFFFF ))\nmything";
    assert_eq!(cmds(src), vec!["mything"]);
}

#[test]
fn rbtdtu_array_literal_elements_not_commands() {
    // `NAME=( ... )` is an array literal; its elements (including bare flags)
    // are data, not commands.
    let src = "OPTS=(-U -l -nn -vvv)\nmything";
    assert_eq!(cmds(src), vec!["mything"]);
}

#[test]
fn rbtdtu_multiline_array_literal() {
    let src = "ARGS=(\n  -o IdentitiesOnly=yes\n  -o StrictHostKeyChecking=accept-new\n)\nmything";
    assert_eq!(cmds(src), vec!["mything"]);
}

#[test]
fn rbtdtu_line_numbers_tracked() {
    let got = zrbtdru_command_words("one\ntwo\nthree");
    assert_eq!(got, vec![
        (1, "one".to_string()),
        (2, "two".to_string()),
        (3, "three".to_string()),
    ]);
}

#[test]
fn rbtdtu_subshell_command_scanned() {
    assert_eq!(cmds("( grep x )"), vec!["grep"]);
}

#[test]
fn rbtdtu_case_patterns_suppressed_bodies_scanned() {
    // Patterns (200, *) are not commands; branch bodies (echo, buc_die) are.
    let src = "case $x in\n  200) echo hi ;;\n  *) buc_die ;;\nesac";
    assert_eq!(cmds(src), vec!["echo", "buc_die"]);
}

#[test]
fn rbtdtu_case_alternation_pattern_not_a_command() {
    // `a|b)` is pattern alternation — `b` must not be recorded as a command.
    assert_eq!(cmds("case $x in a|b) grep y ;; esac"), vec!["grep"]);
}

#[test]
fn rbtdtu_case_flag_pattern_suppressed() {
    // getopts-style flag patterns must not be flagged as commands.
    assert_eq!(cmds("case $opt in -o) mything ;; esac"), vec!["mything"]);
}

#[test]
fn rbtdtu_nested_case_scoped() {
    let src = "case $a in\n  x) case $b in y) grep z ;; esac ;;\nesac";
    assert_eq!(cmds(src), vec!["grep"]);
}

#[test]
fn rbtdtu_command_after_esac() {
    // Both the branch body and the command following the case are scanned.
    assert_eq!(cmds("case $a in x) mybody ;; esac\nmything"), vec!["mybody", "mything"]);
}

#[test]
fn rbtdtu_is_assignment_recognizes_forms() {
    assert!(zrbtdru_is_assignment("FOO=bar"));
    assert!(zrbtdru_is_assignment("FOO+=bar"));
    assert!(zrbtdru_is_assignment("arr[0]=bar"));
    assert!(!zrbtdru_is_assignment("grep"));
    assert!(!zrbtdru_is_assignment("=bad"));
    assert!(!zrbtdru_is_assignment("1abc=x"));
}

#[test]
fn rbtdtu_keyword_kind_classes() {
    assert_eq!(zrbtdru_keyword_kind("then"), Some(true));
    assert_eq!(zrbtdru_keyword_kind("while"), Some(true));
    assert_eq!(zrbtdru_keyword_kind("for"), Some(false));
    assert_eq!(zrbtdru_keyword_kind("case"), Some(false));
    assert_eq!(zrbtdru_keyword_kind("grep"), None);
}

#[test]
fn rbtdtu_collect_functions_both_forms() {
    let mut set = BTreeSet::new();
    zrbtdru_collect_functions("zrbq_kindle() {\n  :\n}\nfunction bar_fn\n", &mut set);
    assert!(set.contains("zrbq_kindle"));
    assert!(set.contains("bar_fn"));
}

// The classify-membership tests derive their inputs from the source-of-truth
// allowlist arrays rather than hardcoded command names: every entry must obey
// its array's contract, and renaming a dep cannot silently drift the test.

#[test]
fn rbtdtu_classify_evictions_flagged_in_kit() {
    let locals = BTreeSet::new();
    for ev in ZRBTDRU_EVICTIONS {
        let verdict = zrbtdru_classify(ev.command, &locals, zrbtdru_Domain::Kit);
        assert!(
            verdict.as_deref().is_some_and(|d| d.contains("evicted")),
            "eviction-table command must be flagged as evicted in kit: {}",
            ev.command
        );
    }
}

#[test]
fn rbtdtu_classify_floor_clears_in_both_domains() {
    let locals = BTreeSet::new();
    for cmd in ZRBTDRU_POSIX_FLOOR {
        assert!(zrbtdru_classify(cmd, &locals, zrbtdru_Domain::Kit).is_none(),
            "POSIX floor must clear in kit: {cmd}");
        assert!(zrbtdru_classify(cmd, &locals, zrbtdru_Domain::Gcb).is_none(),
            "POSIX floor is universal — must clear in gcb: {cmd}");
    }
}

#[test]
fn rbtdtu_classify_declared_deps_clear_in_kit() {
    let locals = BTreeSet::new();
    for cmd in ZRBTDRU_DECLARED_DEPS {
        assert!(zrbtdru_classify(cmd, &locals, zrbtdru_Domain::Kit).is_none(),
            "declared dependency must clear in kit: {cmd}");
    }
}

#[test]
fn rbtdtu_classify_gcb_curated_list_clears_in_gcb() {
    let locals = BTreeSet::new();
    for cmd in ZRBTDRU_GCB_ALLOWED {
        assert!(zrbtdru_classify(cmd, &locals, zrbtdru_Domain::Gcb).is_none(),
            "curated GCB container tool must clear in gcb: {cmd}");
    }
}

#[test]
fn rbtdtu_classify_gcb_does_not_inherit_kit_only_deps() {
    // GCB inherits the floor but NOT the kit declared deps: any declared dep
    // absent from both the floor and the curated GCB list must be flagged —
    // the supply-chain conformance contract (jq/openssl/etc. aren't in the
    // reliquary containers).
    let locals = BTreeSet::new();
    for cmd in ZRBTDRU_DECLARED_DEPS {
        if ZRBTDRU_POSIX_FLOOR.contains(cmd) || ZRBTDRU_GCB_ALLOWED.contains(cmd) {
            continue;
        }
        assert!(zrbtdru_classify(cmd, &locals, zrbtdru_Domain::Gcb).is_some(),
            "kit-only declared dep must be flagged in gcb: {cmd}");
    }
}

#[test]
fn rbtdtu_classify_gcb_has_no_eviction_free_pass() {
    // Evictions are not blanket-tolerated in GCB: an eviction-table command
    // absent from the curated list must be flagged.
    let locals = BTreeSet::new();
    for ev in ZRBTDRU_EVICTIONS {
        if ZRBTDRU_GCB_ALLOWED.contains(&ev.command) {
            continue;
        }
        assert!(zrbtdru_classify(ev.command, &locals, zrbtdru_Domain::Gcb).is_some(),
            "eviction absent from the curated GCB list must be flagged in gcb: {}",
            ev.command);
    }
}

#[test]
fn rbtdtu_classify_local_function_clear() {
    let mut locals = BTreeSet::new();
    locals.insert("buc_die".to_string());
    assert!(zrbtdru_classify("buc_die", &locals, zrbtdru_Domain::Kit).is_none());
}

#[test]
fn rbtdtu_classify_unknown_command_flagged() {
    let locals = BTreeSet::new();
    let verdict = zrbtdru_classify("frobnicate", &locals, zrbtdru_Domain::Kit);
    assert!(verdict.is_some());
    assert!(verdict.unwrap().contains("unknown"));
}

#[test]
fn rbtdtu_classify_dynamic_token_skipped() {
    let locals = BTreeSet::new();
    assert!(zrbtdru_classify("${RBGC_CMD}", &locals, zrbtdru_Domain::Kit).is_none());
}

#[test]
fn rbtdtu_classify_path_command_uses_basename() {
    let locals = BTreeSet::new();
    // A path-qualified evicted command is still evicted by its basename.
    let verdict = zrbtdru_classify("/usr/bin/grep", &locals, zrbtdru_Domain::Kit);
    assert!(verdict.is_some());
    assert!(verdict.unwrap().contains("evicted"));
}
